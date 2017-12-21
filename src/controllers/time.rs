use std::f64;
use rand::{self, ThreadRng};

use super::Actions;
use game_state::GameState;
use geometry::{Advance, Position, Point};
use models::{Bullet, Enemy, Particle, Star, Vector};
use util;

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
const STAR_BASE_SPEED: f64 = 50.0;

const MAX_STARS: usize = 100;
const PLAYER_GRACE_AREA: f64 = 200.0;

/// Timers to handle creation of bullets, enemies and particles
pub struct TimeController {
    /// A random number generator
    rng: ThreadRng,
    current_time: f64,
    last_tail_particle: f64,
    last_shoot: f64,
    last_spawned_enemy: f64
}

impl TimeController {
    pub fn new() -> TimeController {
        TimeController {
            rng: rand::thread_rng(),
            current_time: 0.0,
            last_tail_particle: 0.0,
            last_shoot: 0.0,
            last_spawned_enemy: 0.0
        }
    }

    /// Updates the game
    ///
    /// `dt` is the amount of seconds that have passed since the last update
    pub fn update_seconds(&mut self, dt: f64, actions: &Actions, state: &mut GameState) {
        self.current_time += dt;
        state.difficulty += dt / 100.0;

        self.update_player(dt, actions, state);
        self.update_bullets(dt, actions, state);
        self.update_particles(dt, state);
        self.update_enemies(dt, state);
        self.update_stars(dt, state);
    }

    // Updates the position and rotation of the player
    fn update_player(&mut self, dt: f64, actions: &Actions, state: &mut GameState) {
        if actions.rotate_left {
            *state.world.player.direction_mut() += -ROTATE_SPEED * dt;
        } else if actions.rotate_right {
            *state.world.player.direction_mut() += ROTATE_SPEED * dt;
        }

        // Set speed and advance the player with wrap around
        let speed = if actions.boost { 2.0 * ADVANCE_SPEED } else { ADVANCE_SPEED };
        state.world.player.advance_wrapping(dt * speed, state.world.size);
    }

    // Adds, removes and updates the positions of bullets on screen
    fn update_bullets(&mut self, dt: f64, actions: &Actions, state: &mut GameState) {
        // Add bullets
        if !state.world.player.is_dead && actions.shoot && self.current_time - self.last_shoot > BULLET_RATE {
            self.last_shoot = self.current_time;
            let vector = Vector::new(state.world.player.front(), state.world.player.direction());
            state.world.bullets.push(Bullet::new(vector));
        }

        // Advance bullets
        for bullet in &mut state.world.bullets {
            bullet.update(dt * BULLET_SPEED);
        }

        // Remove bullets outside the viewport
        let size = &state.world.size;
        util::fast_retain(&mut state.world.bullets, |b| size.contains(b.position()));
    }

    // Updates or removes particles on screen, adds particles behind player
    fn update_particles(&mut self, dt: f64, state: &mut GameState) {
        for particle in &mut state.world.particles {
            particle.update(dt);
        }

        // Remove old particles
        util::fast_retain(&mut state.world.particles, |p| p.ttl > 0.0);

        // Add new particles at the player's position, to leave a trail
        if !state.world.player.is_dead && self.current_time - self.last_tail_particle > TRAIL_PARTICLE_RATE {
            self.last_tail_particle = self.current_time;
            state.world.particles.push(Particle::new(state.world.player.vector.clone().invert(), 0.5));
        }
    }

    // Updates positions of enemies, and spawns new ones when necessary
    fn update_enemies(&mut self, dt: f64, state: &mut GameState) {
        // Spawn enemies at random locations
        if self.current_time - self.last_spawned_enemy > ENEMY_SPAWN_RATE {
            self.last_spawned_enemy = self.current_time;

            let player_pos: &Vector = &state.world.player.vector;
            let mut enemy_pos;
            // We loop here, just in case the new enemy random position is exactly equal
            // to the players current position, this would break our calculations below
            loop {
                enemy_pos = Vector::random(&mut self.rng, state.world.size);
                if enemy_pos.position != player_pos.position {
                    break;
                }
            }

            // Check if the newly spawned enemy is inside the player's grace area,
            // if so, we push its spawn point to the edge of the area
            if enemy_pos.position.intersect_circle(&player_pos.position, PLAYER_GRACE_AREA) {
                // Treat the player as the centre of a circle with radius PLAYER_GRACE_AREA
                let Point { x: cx, y: cy } = player_pos.position;
                let dp: Point = enemy_pos.position - player_pos.position;
                // Calculate the angle between the player's position and the enemy's
                let angle = (dp.y).atan2(dp.x);
                // Use that to place the enemy on the edge of the circle surrounding the player
                enemy_pos.position = Point {
                    x: cx + PLAYER_GRACE_AREA * angle.cos(),
                    y: cy + PLAYER_GRACE_AREA * angle.sin()
                };
            }

            let new_enemy = Enemy::new(enemy_pos);
            state.world.enemies.push(new_enemy);
        }

        // Move enemies in the player's direction if player is alive, otherwise let them drift in
        // the direction they're facing
        for enemy in &mut state.world.enemies {
            if !state.world.player.is_dead {
                enemy.update(dt * ENEMY_SPEED + state.difficulty, state.world.player.position());
            } else {
                enemy.advance(dt * ENEMY_SPEED);
            }
        }
    }

    // Slowly moves stars across screen, adding and removing them when necessary
    fn update_stars(&mut self, dt: f64, state: &mut GameState) {
        // Advance stars
        for star in &mut state.world.stars {
            let speed = star.speed;
            star.update(dt * STAR_BASE_SPEED * speed);
        }

        // Remove stars outside the viewport
        let size = &state.world.size;
        util::fast_retain(&mut state.world.stars, |s| size.contains(s.position()));

        // Add stars up to MAX_STARS
        while state.world.stars.len() < MAX_STARS {
            state.world.stars.push(Star::new(state.world.size));
        }
    }
}
