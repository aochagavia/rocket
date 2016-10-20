//! This module contains the game logic
use std::f64;
use std::env::current_exe;

use graphics::{self, Transformed};
use itertools;
use opengl_graphics::GlGraphics;
use opengl_graphics::glyph_cache::GlyphCache;
use piston::input::*;
use rand::{self, ThreadRng};

use drawing::{color, Camera, Point, Size};
use models::{Bullet, Enemy, Particle, Vector, World};
use traits::{Advance, Collide, Position};

const UPS: u16 = 120;
const BULLET_RATE: f64 = 0.01;

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
    resources: Resources,
    /// The camera that is used to render the game
    ///
    /// FIXME: should this be used directly from main?
    cam: Camera,
    triangle_buffer: Vec<f32>
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
    last_spawned_enemy: f64,
}

/// Additional resources needed for the game
struct Resources {
    font: GlyphCache<'static>
}

impl Game {
    /// Returns a new `Game` containing a `World` of the given `Size`
    pub fn new(size: Size) -> Game {
        let rng = rand::thread_rng();
        let exe_directory = current_exe().unwrap().parent().unwrap().to_owned();
        let mut game = Game {
            world: World::new(size.clone()),
            score: 0,
            actions: Actions::default(),
            timers: Timers::default(),
            rng: rng,
            resources: Resources { font: GlyphCache::new(&exe_directory.join("resources/FiraMono-Bold.ttf")).unwrap() },
            cam: Camera { size: Size::new(1024., 600.), pos: Point::new(0.0, 0.0) },
            triangle_buffer: Vec::with_capacity(15000)
        };

        game.reset();
        game
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

        // TODO: Actually use stick position value to affect rotation amount
        //       Actually use trigger position value affect boost amount
    }

    /// Renders the game to the screen
    pub fn render(&mut self, c: graphics::context::Context, g: &mut GlGraphics) {
        // Clear everything
        graphics::clear(color::BLACK, g);

        // Render the world
        self.world.render(c, g, &self.cam, &mut self.triangle_buffer);

        // Render the score
        let mut text = graphics::Text::new(22);
        text.color = color::ORANGE;
        text.draw(&format!("Score: {}", self.score),
                  &mut self.resources.font,
                  &c.draw_state,
                  c.trans(10.0, 20.0).transform,
                  g);
    }

    /// Updates the game
    ///
    /// `dt` is the amount of seconds that have passed since the last update
    pub fn update(&mut self, dt: f64) {
        self.timers.current_time += dt;

        self.cam.pos = self.world.player.vector.position.clone().translate(&Point::new(-512., -300.));

        // Update rocket rotation
        if self.actions.rotate_left {
            *self.world.player.direction_mut() += (-0.06 * UPS as f64) * dt;
        }
        if self.actions.rotate_right {
            *self.world.player.direction_mut() += (0.06 * UPS as f64) * dt;
        };

        // Set speed and advance the player
        let speed = if self.actions.boost { 400.0  } else { 200.0 };
        self.world.player.advance(dt * speed);

        // Update particles
        for particle in &mut self.world.particles {
            particle.update(dt);
        }

        // Remove old particles
        self.world.particles.retain(|p| p.ttl > 0.0);

        // Add new particles at the player's position, to leave a trail
        if self.timers.current_time - self.timers.last_tail_particle > 0.05 {
            self.timers.last_tail_particle = self.timers.current_time;
            self.world.particles.push(Particle::new(self.world.player.vector.clone().invert(), 0.5));
        }

        // Add bullets
        if self.actions.shoot && self.timers.current_time - self.timers.last_shoot > BULLET_RATE {
            self.timers.last_shoot = self.timers.current_time;
            self.world.bullets.push(Bullet::new(Vector::new(self.world.player.nose(), self.world.player.direction())));
        }

        // Update bullets
        for bullet in &mut self.world.bullets {
            bullet.update(dt);
        }

        // Remove dead bullets
        self.world.bullets.retain(|b| b.ttl > 0.0);

        // Spawn enemies at random locations
        if self.timers.current_time - self.timers.last_spawned_enemy > 0.2 {
            self.timers.last_spawned_enemy = self.timers.current_time;
            let mut new_enemy: Enemy;
            loop {
                let pos = Point::random(&mut self.rng, &self.world.player.position(), self.cam.size.width);
                new_enemy = Enemy::new(Vector::new(pos, 0.0));
                if !self.world.player.collides_with(&new_enemy) {
                    break;
                }
            }
            self.world.enemies.push(new_enemy);
        }

        // Move enemies in the player's direction
        for enemy in &mut self.world.enemies {
            enemy.update(dt * 100.0, self.world.player.position());
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

        { // We introduce a scope to shorten the lifetime of the borrows below
        // The references are to avoid using self in the closure
        // (the borrow checker doesn't like that)
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
        *self.world.player.x_mut() = 0.0;
        *self.world.player.y_mut() = 0.0;

        // Let the camera follow the new position of the player
        self.cam.follow(self.world.player.position());

        // Reset score
        self.score = 0;

        // Remove all enemies and bullets
        self.world.bullets.clear();
        self.world.enemies.clear();

        // Create new enemies
        for _ in 0..5000 {
            let pos = Point::random(&mut self.rng, &Point::new(0.0, 0.0), 5000.0);
            self.world.enemies.push(Enemy::new(Vector::new(pos, 0.0)));
        }
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

    /// Generates a new explosion of the given intensity at the given position. This works best with values between 5 and 25
    fn make_explosion(particles: &mut Vec<Particle>, position: Point, intensity: u8) {
        for rotation in itertools::linspace(0.0, 2.0 * f64::consts::PI, 30) {
            for ttl in (1..intensity).map(|x| (x as f64) / 10.0) {
                particles.push(Particle::new(Vector::new(position.clone(), rotation), ttl));
            }
        }
    }
}
