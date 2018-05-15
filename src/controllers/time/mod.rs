mod timeout;
mod timeout_queue;
mod timer;

use std::{mem, f32};
use std::time::Duration;
use rand::Rng;

use controllers::input::Actions;
use controllers::Event;
use game_state::GameState;
use geometry::{Advance, Point, Position, Vector};
use models::{Bullet, Enemy, Particle, Powerup, PowerupKind};
use util;

use self::timer::Timer;
pub use self::timeout::Timeout;
use self::timeout_queue::TimeoutQueue;

// Constants related to time
const BULLETS_PER_SECOND: f32 = 30.0;
const BULLET_RATE: f32 = 1.0 / BULLETS_PER_SECOND;

const ENEMY_SPAWNS_PER_SECOND: f32 = 1.0;
const ENEMY_SPAWN_RATE: f32 = 1.0 / ENEMY_SPAWNS_PER_SECOND;

const TRAIL_PARTICLES_PER_SECOND: f32 = 20.0;
const TRAIL_PARTICLE_RATE: f32 = 1.0 / TRAIL_PARTICLES_PER_SECOND;

const POWERUP_SPAWNS_PER_SECOND: f32 = 1.0 / 10.0; // every ~10 seconds
const POWERUP_SPAWN_RATE: f32 = 1.0 / POWERUP_SPAWNS_PER_SECOND;

// Constants related to movement
// Speed is measured in pixels per second
// Rotation speed is measured in radians per second
const ADVANCE_SPEED: f32 = 200.0;
const BULLET_SPEED: f32 = 500.0;
const ENEMY_SPEED: f32 = 100.0;
const ROTATE_SPEED: f32 = 2.0 * f32::consts::PI;
const STAR_BASE_SPEED: f32 = 50.0;

pub const PLAYER_GRACE_AREA: f32 = 200.0;

pub struct TimeController {
    /// The duration of the current game, since the last restart
    current_time: Duration,
    /// A timer to trigger creation of trail particles
    trail_timer: Timer,
    /// A timer to trigger creation of bullets
    shoot_timer: Timer,
    /// A timer to spawn enemies
    enemy_timer: Timer,
    /// A timer to spawn powerups
    powerup_timer: Timer,
    /// Scheduled events that should happen in the future
    scheduled_timeouts: TimeoutQueue,
}

impl TimeController {
    pub fn new() -> TimeController {
        TimeController {
            current_time: Duration::from_secs(0),
            trail_timer: Timer::from_seconds(TRAIL_PARTICLE_RATE),
            shoot_timer: Timer::from_seconds(BULLET_RATE),
            enemy_timer: Timer::from_seconds(ENEMY_SPAWN_RATE),
            powerup_timer: Timer::from_seconds(POWERUP_SPAWN_RATE),
            scheduled_timeouts: TimeoutQueue::new(),
        }
    }

    // Called when the game is reset
    pub fn reset(&mut self) {
        mem::replace(self, TimeController::new());
    }

    pub fn schedule_timeout(&mut self, offset: Duration, timeout: Timeout) {
        self.scheduled_timeouts.push(self.current_time + offset, timeout);
    }

    /// Updates the game
    ///
    /// `dt` is the amount of seconds that have passed since the last update
    pub fn update_seconds<R: Rng>(
        &mut self,
        dt: Duration,
        actions: &Actions,
        state: &mut GameState,
        events: &mut Vec<Event>,
        rng: &mut R
    ) {
        self.current_time += dt;

        let dt = util::duration_to_seconds(dt);
        state.difficulty += dt / 100.0;

        // Check if we have any events that are scheduled to run, and if so, run them now
        if let Some(when) = self.scheduled_timeouts.peek() {
            if when <= self.current_time {
                self.scheduled_timeouts.pop().unwrap().handle(state);
            }
        }

        // Check if we have the "TimeSlow" powerup
        let time_slow = state.world.player.powerup == Some(PowerupKind::TimeSlow);

        // Only modify player/powerups if player is alive
        if !state.world.player.is_dead {
            self.update_player(dt, actions, state);
            self.update_powerups(dt, state, rng);
        }

        self.update_bullets(dt, actions, state, events);
        self.update_particles(dt, state);
        self.update_enemies(dt, state, events, time_slow, rng);
        self.update_stars(dt, state, time_slow);
    }

    // Updates the position and rotation of the player
    fn update_player(&mut self, dt: f32, actions: &Actions, state: &mut GameState) {
        if !state.world.player.is_dead {
            if actions.rotate_left {
                *state.world.player.direction_mut() += -ROTATE_SPEED * dt;
            } else if actions.rotate_right {
                *state.world.player.direction_mut() += ROTATE_SPEED * dt;
            }

            // Set speed and advance the player with wrap around
            let speed = if actions.boost {
                2.0 * ADVANCE_SPEED
            } else {
                ADVANCE_SPEED
            };
            state
                .world
                .player
                .advance_wrapping(dt * speed, state.world.size);

            // Cool down the player's gun
            state.world.player.gun.cool_down(dt);
        }
    }

    // Adds, removes and updates the positions of bullets on screen
    fn update_bullets(
        &mut self,
        dt: f32,
        actions: &Actions,
        state: &mut GameState,
        events: &mut Vec<Event>,
    ) {
        // Add bullets - usually when the player shoots the gun heats up, if it has overheated the
        // player can no longer shoot - unless they have the tripleshot powerup, which will work
        // regardless of the gun's state
        if !state.world.player.is_dead && actions.shoot {
            self.shoot_timer.update(self.current_time, || {
                match state.world.player.powerup {
                    // If the player has the TripleShot powerup, apply that here
                    Some(PowerupKind::TripleShot) => {
                        let pos = state.world.player.front();
                        let dir = state.world.player.direction();
                        state.world.bullets.extend_from_slice(&[
                            Bullet::new(Vector::new(pos, dir - f32::consts::PI / 6.0)),
                            Bullet::new(Vector::new(pos, dir)),
                            Bullet::new(Vector::new(pos, dir + f32::consts::PI / 6.0)),
                        ]);
                        events.push(Event::ShotFired);
                    }
                    // If there was no powerup, shoot normally
                    _ => {
                        if state.world.player.gun.is_available() {
                            let pos = state.world.player.front();
                            let dir = state.world.player.direction();
                            state.world.bullets.push(Bullet::new(Vector::new(pos, dir)));
                            state.world.player.gun.heat_up();

                            events.push(Event::ShotFired);
                        }
                    }
                }
            });
        }

        // Advance bullets
        for bullet in &mut state.world.bullets {
            bullet.update(dt * BULLET_SPEED);
        }

        // Remove bullets outside the viewport
        let size = &state.world.size;
        util::fast_retain(&mut state.world.bullets, |b| size.contains(b.position()));
    }

    fn update_powerups<R: Rng>(&mut self, dt: f32, state: &mut GameState, rng: &mut R) {
        for powerup in &mut state.world.powerups {
            powerup.update(dt);
        }

        // Remove any expired powerups
        util::fast_retain(&mut state.world.powerups, |p| p.ttl > 0.0);

        // Add new powerups
        self.powerup_timer.update(self.current_time, || {
            state
                .world
                .powerups
                .push(Powerup::random(rng, state.world.size));
        });
    }

    // Updates or removes particles on screen, adds particles behind player
    fn update_particles(&mut self, dt: f32, state: &mut GameState) {
        for particle in &mut state.world.particles {
            particle.update(dt);
        }

        // Remove old particles
        util::fast_retain(&mut state.world.particles, |p| p.ttl > 0.0);

        // Add new particles at the player's position, to leave a trail
        if !state.world.player.is_dead {
            self.trail_timer.update(self.current_time, || {
                state.world.particles.push(Particle::new(
                    state.world.player.vector.clone().invert(),
                    0.5,
                ));
            });
        }
    }

    // Updates positions of enemies, and spawns new ones when necessary
    fn update_enemies<R: Rng>(
        &mut self,
        dt: f32,
        state: &mut GameState,
        events: &mut Vec<Event>,
        time_slow: bool,
        rng: &mut R
    ) {
        // Spawn enemies at random locations
        self.enemy_timer.update(self.current_time, || {
            let player_pos: &Vector = &state.world.player.vector;
            let mut enemy_pos;
            // We loop here, just in case the new enemy random position is exactly equal
            // to the players current position, this would break our calculations below
            loop {
                enemy_pos = Vector::random(rng, state.world.size);
                if enemy_pos.position != player_pos.position {
                    break;
                }
            }

            // Check if the newly spawned enemy is inside the player's grace area,
            // if so, we push its spawn point to the edge of the area
            if enemy_pos
                .position
                .intersect_circle(&player_pos.position, PLAYER_GRACE_AREA)
            {
                // Treat the player as the centre of a circle with radius PLAYER_GRACE_AREA
                let Point { x: cx, y: cy } = player_pos.position;
                let dp: Point = enemy_pos.position - player_pos.position;
                // Calculate the angle between the player's position and the enemy's
                let angle = (dp.y).atan2(dp.x);
                // Use that to place the enemy on the edge of the circle surrounding the player
                enemy_pos.position = Point {
                    x: cx + PLAYER_GRACE_AREA * angle.cos(),
                    y: cy + PLAYER_GRACE_AREA * angle.sin(),
                };
            }

            let new_enemy = Enemy::new(enemy_pos);
            state.world.enemies.push(new_enemy);
            events.push(Event::EnemySpawned);
        });

        // Move enemies in the player's direction if player is alive, otherwise let them drift in
        // the direction they're facing
        for enemy in &mut state.world.enemies {
            if !state.world.player.is_dead {
                let base_speed = if time_slow {
                    ENEMY_SPEED - 75.0
                } else {
                    ENEMY_SPEED
                };
                enemy.update(
                    dt * base_speed + state.difficulty,
                    state.world.player.position(),
                );
            } else {
                enemy.advance(dt * ENEMY_SPEED);
            }
        }
    }

    // Advance stars, wrapping them around the view
    fn update_stars(&mut self, dt: f32, state: &mut GameState, time_slow: bool) {
        for star in &mut state.world.stars {
            let base_speed = if time_slow { 20.0 } else { STAR_BASE_SPEED };
            star.advance_wrapping(dt * base_speed * star.speed, state.world.size);
        }
    }
}
