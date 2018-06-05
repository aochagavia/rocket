//! This module contains the game logic
//!
//! There are three main controllers: collisions, input and time

mod collisions;
pub mod input;
mod time;

pub use self::collisions::CollisionsController;
pub use self::input::InputController;
pub use self::time::{TimeController, PLAYER_GRACE_AREA};

pub enum Event {
    PlayerDestroyed,
    EnemyDestroyed,
    PowerupGained,
    ShotFired,
    EnemySpawned,
    GameStart
}
