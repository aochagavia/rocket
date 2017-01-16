//! This module contains the game logic

use std::f64;
use std::env::current_exe;

use itertools_num;
use opengl_graphics::GlGraphics;
use opengl_graphics::glyph_cache::GlyphCache;
use piston_window::{clear, ControllerButton, Context, ControllerAxisArgs, Key, text, Transformed};
use rand::{self, ThreadRng};

use drawing::{color, Point, Size};
use models::{Bullet, Enemy, Particle, Vector, World};
use traits::{Advance, Collide, Position};

// Constants related to time
const BULLETS_PER_SECOND: f64 = 100.0;
const BULLET_RATE: f64 = 1.0 / BULLETS_PER_SECOND;

const ENEMY_SPAWNS_PER_SECOND: f64 = 1.0;
const ENEMY_SPAWN_RATE: f64 = 1.0 / ENEMY_SPAWNS_PER_SECOND;

const TRAIL_PARTICLES_PER_SECOND: f64 = 20.0;
const TRAIL_PARTICLE_RATE: f64 = 1.0 / TRAIL_PARTICLES_PER_SECOND;

// Constants related to movement
// Speed is measured in pixels per second
// Rotation speed is measured in radians per second
const ADVANCE_SPEED: f64 = 200.0;
const BULLET_SPEED: f64 = 500.0;
const ENEMY_SPEED: f64 = 100.0;
const ROTATE_SPEED: f64 = 2.0 * f64::consts::PI;


/// The data structure that drives the game
pub struct Game {
    /// The world contains everything that needs to be drawn
    world: World,
    /// The current score of the player
    score: u32,
    /// The active actions
    actions: Actions,
    /// Timers needed by the game
    timers: Timers,
    /// A random number generator
    rng: ThreadRng,
    /// Resources needed for drawing
    resources: Resources
}

/// Active actions (toggled by user input)
#[derive(Default)]
struct Actions {
    rotate_left: bool,
    rotate_right: bool,
    boost: bool,
    shoot: bool
}

/// Timers to handle creation of bullets, enemies and particles
#[derive(Default)]
struct Timers {
    current_time: f64,
    last_tail_particle: f64,
    last_shoot: f64,
    last_spawned_enemy: f64
}

/// Additional resources needed for the game
struct Resources {
    font: GlyphCache<'static>
}

impl Game {
    /// Returns a new `Game` containing a `World` of the given `Size`
    pub fn new(size: Size) -> Game {
        let mut rng = rand::thread_rng();
        let exe_directory = current_exe().unwrap().parent().unwrap().to_owned();
        Game {
            world: World::new(&mut rng, size),
            score: 0,
            actions: Actions::default(),
            timers: Timers::default(),
            rng: rng,
            resources: Resources {
                font: GlyphCache::new(&exe_directory.join("resources/FiraMono-Bold.ttf")).unwrap()
            }
        }
    }

    /// Processes a key press
    pub fn key_press(&mut self, key: Key) {
        self.handle_key(key, true);
    }

    /// Processes a key release
    pub fn key_release(&mut self, key: Key) {
        self.handle_key(key, false);
    }

    /// Handles a key press or release
    fn handle_key(&mut self, key: Key, pressed: bool) {
        match key {
            Key::Left => self.actions.rotate_left = pressed,
            Key::Right => self.actions.rotate_right = pressed,
            Key::Up => self.actions.boost = pressed,
            Key::Space => self.actions.shoot = pressed,
            _ => ()
        }
    }

    /// Processes a button press
    pub fn button_press(&mut self, controller: ControllerButton) {
        self.handle_button(controller, true);
    }

    /// Processes a button release
    pub fn button_release(&mut self, controller: ControllerButton) {
        self.handle_button(controller, false);
    }

    /// Handles a button press or release
    fn handle_button(&mut self, controller: ControllerButton, pressed: bool) {
        // Button 10 is A button on XInput
        match controller.button {
            10 => self.actions.shoot = pressed,
            _ => ()
        }
    }

    /// Handles a controller axis input
    pub fn handle_axis(&mut self, controller: ControllerAxisArgs) {
        // Axis 0 is left stick (XInput). -1.0 left to 1.0 right
        if controller.axis == 0 {
            match controller.position {
                -1.0 ... -0.2 => {
                    self.actions.rotate_left = true;
                    self.actions.rotate_right = false;
                },
                0.2 ... 1.0 => {
                    self.actions.rotate_left = false;
                    self.actions.rotate_right = true;
                },
                -0.199 ... 0.199 => {
                    self.actions.rotate_left = false;
                    self.actions.rotate_right = false;
                },
                _ => {}
            }
        }

        // Axis 5 is right trigger (XInput). -1.0 is not pressed, 1.0 is fully pressed
        if controller.axis == 5 {
            match controller.position {
                -0.8 ... 1.0 => {
                    self.actions.boost = true;
                },
                -1.0 ... -0.799 => {
                    self.actions.boost = false;
                },
                _ => {}
            }
        }
    }

    /// Renders the game to the screen
    pub fn render(&mut self, c: Context, g: &mut GlGraphics) {
        // Clear everything
        clear(color::BLACK, g);

        // Render the world
        self.world.render(c, g);

        // Render the score
        text(color::ORANGE,
             22,
             &format!("Score: {}", self.score),
             &mut self.resources.font,
             c.trans(10.0, 20.0).transform,
             g);
    }

    /// Updates the game
    ///
    /// `dt` is the amount of seconds that have passed since the last update
    pub fn update(&mut self, dt: f64) {
        self.timers.current_time += dt;

        // Update rocket rotation
        if self.actions.rotate_left {
            *self.world.player.direction_mut() += -ROTATE_SPEED * dt;
        }
        if self.actions.rotate_right {
            *self.world.player.direction_mut() += ROTATE_SPEED * dt;
        };

        // Set speed and advance the player with wrap around
        let speed = if self.actions.boost { 2.0 * ADVANCE_SPEED } else { ADVANCE_SPEED };
        self.world.player.advance_wrapping(dt * speed, self.world.size);

        // Update particles
        for particle in &mut self.world.particles {
            particle.update(dt);
        }

        // Remove old particles
        self.world.particles.retain(|p| p.ttl > 0.0);

        // Add new particles at the player's position, to leave a trail
        if self.timers.current_time - self.timers.last_tail_particle > TRAIL_PARTICLE_RATE {
            self.timers.last_tail_particle = self.timers.current_time;
            self.world.particles.push(Particle::new(self.world.player.vector.clone().invert(),
                                                    0.5));
        }

        // Add bullets
        if self.actions.shoot && self.timers.current_time - self.timers.last_shoot > BULLET_RATE {
            self.timers.last_shoot = self.timers.current_time;
            self.world.bullets.push(Bullet::new(Vector::new(self.world.player.nose(),
                                                            self.world.player.direction())));
        }

        // Advance bullets
        for bullet in &mut self.world.bullets {
            bullet.update(dt * BULLET_SPEED);
        }

        // Remove bullets outside the viewport
        {
            // Shorten the lifetime of size
            let size = &self.world.size;
            self.world.bullets.retain(|b| size.contains(b.position()));
        }

        // Spawn enemies at random locations
        if self.timers.current_time - self.timers.last_spawned_enemy > ENEMY_SPAWN_RATE {
            self.timers.last_spawned_enemy = self.timers.current_time;
            let mut new_enemy;
            loop {
                new_enemy = Enemy::new(Vector::random(&mut self.rng, self.world.size));
                if !self.world.player.collides_with(&new_enemy) {
                    break;
                }
            }
            self.world.enemies.push(new_enemy);
        }

        // Move enemies in the player's direction
        for enemy in &mut self.world.enemies {
            enemy.update(dt * ENEMY_SPEED, self.world.player.position());
        }

        self.handle_player_collisions();
        self.handle_bullet_collisions();
    }

    /// Handles collisions between the bullets and the enemies
    ///
    /// When an enemy is reached by a bullet, both the enemy and the bullet
    /// will be removed. Additionally, the score will be increased by 10
    fn handle_bullet_collisions(&mut self) {
        let old_enemy_count = self.world.enemies.len();

        // We introduce a scope to shorten the lifetime of the borrows below
        {
            let bullets = &mut self.world.bullets;
            let enemies = &mut self.world.enemies;
            let particles = &mut self.world.particles;

            bullets.retain(|bullet| {
                // Remove the first enemy that collides with a bullet (if any)
                // Add an explosion on its place
                if let Some((index, position)) = enemies.iter().enumerate()
                    .find(|&(_, enemy)| enemy.collides_with(bullet))
                    .map(|(index, enemy)| (index, enemy.position()))
                    {
                        Game::make_explosion(particles, position, 10);
                        enemies.remove(index);
                        false
                    } else {
                    true
                }
            });
        }

        let killed_enemies = (old_enemy_count - self.world.enemies.len()) as u32;
        self.score += 10 * killed_enemies;
    }

    /// reset our game-state
    fn reset(&mut self) {
        // Reset player position
        *self.world.player.x_mut() = self.world.size.random_x(&mut self.rng);
        *self.world.player.y_mut() = self.world.size.random_y(&mut self.rng);

        // Reset score
        self.score = 0;

        // Remove all enemies and bullets
        self.world.bullets.clear();
        self.world.enemies.clear();
    }

    /// Handles collisions between the player and the enemies
    fn handle_player_collisions(&mut self) {
        if self.world.enemies.iter().any(|enemy| self.world.player.collides_with(enemy)) {
            // Make an explosion where the player was
            let ppos = self.world.player.position();
            Game::make_explosion(&mut self.world.particles, ppos, 8);

            self.reset();
        }
    }

    /// Generates a new explosion of the given intensity at the given position.
    /// This works best with values between 5 and 25
    fn make_explosion(particles: &mut Vec<Particle>, position: Point, intensity: u8) {
        for rotation in itertools_num::linspace(0.0, 2.0 * f64::consts::PI, 30) {
            for ttl in (1..intensity).map(|x| (x as f64) / 10.0) {
                particles.push(Particle::new(Vector::new(position.clone(), rotation), ttl));
            }
        }
    }
}
