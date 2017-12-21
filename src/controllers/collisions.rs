use std::time::Duration;
use ggez::{self, Context};

use ApplicationState;
use game_state::GameState;
use geometry::{Collide, Position};
use util;

const SCORE_PER_ENEMY: u32 = 10;

pub struct CollisionsController;

impl CollisionsController {
    pub fn handle_collisions(app: &mut ApplicationState, ctx: &mut Context) {
        // Don't handle collisions if the player is dead
        if app.game_state.world.player.is_dead { return; }

        CollisionsController::handle_bullet_collisions(&mut app.game_state);
        CollisionsController::handle_player_collisions(&mut app.game_state);
        
        // If the player died then we set a timeout (3 seconds) after which a game over message
        // will appear, and the user will be able to restart.
        if app.game_state.world.player.is_dead {
            // TODO: use closures here ?
            fn handler(app: &mut ApplicationState) { app.game_state.game_over(); };
            let when = ggez::timer::get_time_since_start(ctx) + Duration::from_secs(3);
            app.scheduled_events.push(when, handler);
        }
    }

    /// Handles collisions between the bullets and the enemies
    ///
    /// When an enemy is reached by a bullet, both the enemy and the bullet will be removed. 
    /// Additionally, the score of the player will be increased
    fn handle_bullet_collisions(state: &mut GameState) {
        let old_enemy_count = state.world.enemies.len();

        // We introduce a scope to shorten the lifetime of the borrows below
        {
            let bullets = &mut state.world.bullets;
            let enemies = &mut state.world.enemies;
            let particles = &mut state.world.particles;

            // Note: this is O(n * m) where n = amount of bullets and n = amount of enemies
            // This is pretty bad, but we don't care because n and m are small
            util::fast_retain(bullets, |bullet| {
                // Remove the first enemy that collides with a bullet (if any)
                // Add an explosion on its place
                if let Some((index, position)) = enemies.iter().enumerate()
                    .find(|&(_, enemy)| enemy.collides_with(bullet))
                    .map(|(index, enemy)| (index, enemy.position()))
                    {
                        util::make_explosion(particles, &position, 10);
                        enemies.remove(index);
                        false
                    } else {
                    true
                }
            });
        }

        let killed_enemies = (old_enemy_count - state.world.enemies.len()) as u32;
        state.score += SCORE_PER_ENEMY * killed_enemies;
    }

    /// Handles collisions between the player and the enemies
    fn handle_player_collisions(state: &mut GameState) {
        if state.world.enemies.iter().any(|enemy| state.world.player.collides_with(enemy)) {
            // Make an explosion where the player was
            let ppos = state.world.player.position();
            util::make_explosion(&mut state.world.particles, &ppos, 8);
            // Mark the player as dead (to stop drawing it on screen)
            state.world.player.is_dead = true;
        }
    }
}
