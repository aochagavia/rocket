use std::f64;
use rand::{self, ThreadRng};

use super::Actions;
use game_state::GameState;
use geometry::{Advance, Position, Point};
use models::{Bullet, Enemy, Particle, Vector};
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

        let player_alive = !state.world.player.is_dead;

        // Update rocket rotation
        if actions.rotate_left {
            *state.world.player.direction_mut() += -ROTATE_SPEED * dt;
        }
        if actions.rotate_right {
            *state.world.player.direction_mut() += ROTATE_SPEED * dt;
        };

        // Set speed and advance the player with wrap around
        let speed = if actions.boost { 2.0 * ADVANCE_SPEED } else { ADVANCE_SPEED };
        state.world.player.advance_wrapping(dt * speed, state.world.size);

        // Update particles
        for particle in &mut state.world.particles {
            particle.update(dt);
        }

        // Remove old particles
        util::fast_retain(&mut state.world.particles, |p| p.ttl > 0.0);

        // Add new particles at the player's position, to leave a trail
        if player_alive && self.current_time - self.last_tail_particle > TRAIL_PARTICLE_RATE {
            self.last_tail_particle = self.current_time;
            state.world.particles.push(Particle::new(state.world.player.vector.clone().invert(), 0.5));
        }

        // Add bullets
        if player_alive && actions.shoot && self.current_time - self.last_shoot > BULLET_RATE {
            self.last_shoot = self.current_time;
            state.world.bullets.push(Bullet::new(Vector::new(state.world.player.front(),
                                                            state.world.player.direction())));
        }

        // Advance bullets
        for bullet in &mut state.world.bullets {
            bullet.update(dt * BULLET_SPEED);
        }

        // Remove bullets outside the viewport
        {
            // Shorten the lifetime of size
            let size = &state.world.size;
            util::fast_retain(&mut state.world.bullets, |b| size.contains(b.position()));
        }

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
            if player_alive {
                enemy.update(dt * ENEMY_SPEED + state.difficulty, state.world.player.position());
            } else {
                enemy.advance(dt * ENEMY_SPEED);
            }
        }
    }
}
