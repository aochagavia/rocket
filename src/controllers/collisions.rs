use std::time::Duration;

use controllers::time::{TimeController, Timeout};
use controllers::{Event, PLAYER_GRACE_AREA};
use game_state::GameState;
use geometry::{Collide, Point, Position};
use models::{Enemy, Particle, PowerupKind};
use util;

const SCORE_PER_ENEMY: u32 = 10;
const POWERUP_DURATION: u64 = 10;

pub struct CollisionsController;

impl CollisionsController {
    pub fn handle_collisions(
        state: &mut GameState,
        time_controller: &mut TimeController,
        events: &mut Vec<Event>,
    ) {
        let old_enemy_count = state.world.enemies.len();

        CollisionsController::handle_bullet_collisions(state, events);

        let got_powerup = CollisionsController::handle_powerup_collisions(state, events);
        if got_powerup {
            // Powerups run out after `POWERUP_DURATION` seconds
            let offset = Duration::from_secs(POWERUP_DURATION);
            time_controller.schedule_timeout(offset, Timeout::RemovePowerup);
        }

        // If the player died then we set a timeout after which a game over message
        // will appear, and the user will be able to restart.
        let player_died = CollisionsController::handle_player_collisions(state, events);
        if player_died {
            let offset = Duration::from_secs(2);
            time_controller.schedule_timeout(offset, Timeout::ShowGameOverScreen);
        }

        let killed_enemies = (old_enemy_count - state.world.enemies.len()) as u32;
        state.score += SCORE_PER_ENEMY * killed_enemies;
    }

    /// Handles collisions between the bullets and the enemies
    ///
    /// When an enemy is reached by a bullet, both the enemy and the bullet will be removed.
    /// Additionally, the score of the player will be increased
    fn handle_bullet_collisions(state: &mut GameState, events: &mut Vec<Event>) {
        // We introduce a scope to shorten the lifetime of the borrows below
        {
            let bullets = &mut state.world.bullets;
            let enemies = &mut state.world.enemies;
            let particles = &mut state.world.particles;

            // Note: this is O(n * m) where n = amount of bullets and m = amount of enemies
            // This is pretty bad, but we don't care because n and m are small
            util::fast_retain(bullets, |bullet| {
                // Remove the first enemy that collides with a bullet (if any)
                // Add an explosion on its place
                if let Some((index, position)) = enemies
                    .iter()
                    .enumerate()
                    .find(|&(_, enemy)| enemy.collides_with(bullet))
                    .map(|(index, enemy)| (index, enemy.position()))
                {
                    enemies.remove(index);
                    events.push(Event::EnemyDestroyed);
                    util::make_explosion(particles, &position, 10);

                    // Play enemy_destroyed_sound sound
                    // TODO: these sounds (like all the others) are queued rather than played
                    // atop of one another - this is a current limitation of ggez
                    // See https://github.com/ggez/ggez/issues/208
                    // let _ = resources.enemy_destroyed_sound.play();
                    events.push(Event::EnemyDestroyed);
                    false
                } else {
                    true
                }
            });
        }
    }

    /// Handles collisions between the player and powerups
    fn handle_powerup_collisions(state: &mut GameState, events: &mut Vec<Event>) -> bool {
        let mut gained_powerup = false;
        let player = &mut state.world.player;
        let powerups = &mut state.world.powerups;

        if !player.is_dead {
            if let Some((index, kind)) = powerups
                .iter()
                .enumerate()
                .find(|&(_, powerup)| powerup.collides_with(player))
                .map(|(index, powerup)| (index, powerup.kind))
            {
                gained_powerup = true;

                // Set player's powerup kind to the powerup we just picked up
                player.powerup = Some(kind);
                powerups.remove(index);

                events.push(Event::PowerupGained);
            }
        }

        return gained_powerup;
    }

    /// Handles collisions between the player and the enemies
    /// This function will return true if the player died
    fn handle_player_collisions(state: &mut GameState, events: &mut Vec<Event>) -> bool {
        let mut player_died = false;
        let player = &mut state.world.player;

        if !player.is_dead
            && state
                .world
                .enemies
                .iter()
                .any(|enemy| player.collides_with(enemy))
        {
            // Remove shield powerup from player, also killing any enemies within close range
            if let Some(PowerupKind::Shield) = player.powerup {
                player.powerup = None;

                let enemies = &mut state.world.enemies;
                let particles = &mut state.world.particles;
                CollisionsController::remove_surrounding_enemies(
                    enemies,
                    particles,
                    player.position(),
                );
                events.push(Event::EnemyDestroyed);
            } else {
                // Make an explosion where the player was
                let ppos = player.position();
                util::make_explosion(&mut state.world.particles, &ppos, 16);
                // Mark the player as dead (to stop drawing it on screen)
                player_died = true;
                player.is_dead = true;
                events.push(Event::PlayerDestroyed);
            }
        }

        return player_died;
    }

    fn remove_surrounding_enemies(
        enemies: &mut Vec<Enemy>,
        particles: &mut Vec<Particle>,
        point: Point,
    ) {
        util::fast_retain(enemies, |enemy| {
            let enemy_pos = enemy.position();
            if enemy_pos.intersect_circle(&point, PLAYER_GRACE_AREA) {
                util::make_explosion(particles, &enemy_pos, 10);
                false
            } else {
                true
            }
        });
    }
}
