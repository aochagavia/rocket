use rand::Rng;

use crate::{
    geometry::{Position, Size},
    models::World,
};

/// This is a message that will be drawn to the screen. When it's shown on the screen the game
/// will be waiting for user input
pub struct Message {
    pub title: &'static str,
    pub subtitle: &'static str,
}

/// The Message to show when the game starts
const WELCOME_MESSAGE: Message = Message {
    title: "Welcome to Rocket!",
    subtitle: "Press any key to start",
};

/// The Message to show when the game is over
const GAMEOVER_MESSAGE: Message = Message {
    title: "Game Over",
    subtitle: "Press any key to restart",
};

/// The data structure that contains the state of the game
pub struct GameState {
    /// The world contains everything that needs to be drawn
    pub world: World,
    /// The current difficulty - the enemies will speed up over time
    pub difficulty: f32,
    /// Information about the Message to draw on the screen
    pub message: Option<Message>,
    /// The current score of the player
    pub score: u32,
}

impl GameState {
    /// Returns a new `GameState` containing a `World` of the given `Size`
    pub fn new(size: Size, rng: &mut impl Rng) -> GameState {
        GameState {
            world: World::new(rng, size),
            difficulty: 0.0,
            message: Some(WELCOME_MESSAGE),
            score: 0,
        }
    }

    /// Called when the game is over - displays a message onscreen
    pub fn game_over(&mut self) {
        self.message = Some(GAMEOVER_MESSAGE);
    }

    /// Reset our game-state
    pub fn reset(&mut self, rng: &mut impl Rng) {
        // Reset player
        self.world.player.is_dead = false;
        self.world.player.powerup = None;
        *self.world.player.x_mut() = self.world.size.random_x(rng);
        *self.world.player.y_mut() = self.world.size.random_y(rng);
        self.world.player.gun.reset();

        // Reset score
        self.score = 0;

        // Reset difficulty
        self.difficulty = 0.0;

        // Reset message
        self.message = None;

        // Remove all enemies, bullets and powerups
        self.world.bullets.clear();
        self.world.enemies.clear();
        self.world.powerups.clear();
    }
}
