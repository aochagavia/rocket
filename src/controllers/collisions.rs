use std::time::Duration;
use ggez::{self, Context};

use ApplicationState;
use Resources;
use game_state::GameState;
use geometry::{Collide, Position};
use util;

const SCORE_PER_ENEMY: u32 = 10;

pub struct CollisionsController;

impl CollisionsController {
    pub fn handle_collisions(app: &mut ApplicationState, ctx: &mut Context) {
        CollisionsController::handle_bullet_collisions(&mut app.game_state, &app.resources);
        
        // If the player died then we set a timeout (3 seconds) after which a game over message
        // will appear, and the user will be able to restart.
        let player_died = CollisionsController::handle_player_collisions(&mut app.game_state, &app.resources);
        if player_died {
            let when = ggez::timer::get_time_since_start(ctx) + Duration::from_secs(3);
            app.scheduled_events.push(when, |app| app.game_state.game_over());
        }
    }

    /// Handles collisions between the bullets and the enemies
    ///
    /// When an enemy is reached by a bullet, both the enemy and the bullet will be removed. 
    /// Additionally, the score of the player will be increased
    fn handle_bullet_collisions(state: &mut GameState, resources: &Resources) {
        let old_enemy_count = state.world.enemies.len();

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
                if let Some((index, position)) = enemies.iter().enumerate()
                    .find(|&(_, enemy)| enemy.collides_with(bullet))
                    .map(|(index, enemy)| (index, enemy.position()))
                    {
                        util::make_explosion(particles, &position, 10);
                        enemies.remove(index);
                        
                        // Play enemy_destroyed_sound sound
                        // TODO: these sounds (like all the others) are queued rather than played
                        // atop of one another - this is a current limitation of ggez
                        // See https://github.com/ggez/ggez/issues/208
                        let _ = resources.enemy_destroyed_sound.play();
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
    fn handle_player_collisions(state: &mut GameState, resources: &Resources) -> bool {
        let player_alive = !state.world.player.is_dead;
        if player_alive && state.world.enemies.iter().any(|enemy| state.world.player.collides_with(enemy)) {
            // Make an explosion where the player was
            let ppos = state.world.player.position();
            util::make_explosion(&mut state.world.particles, &ppos, 16);
            // Mark the player as dead (to stop drawing it on screen)
            state.world.player.is_dead = true;
            // Play player_destroyed sound
            let _ = resources.player_destroyed_sound.play();
            return true;
        }
        
        false
    }
}
