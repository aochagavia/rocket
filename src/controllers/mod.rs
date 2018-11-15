//! This module contains the game logic
//!
//! There are three main controllers: collisions, input and time

mod collisions;
mod input;
mod time;

pub use self::collisions::CollisionsController;
pub use self::input::{Actions, InputController};
pub use self::time::{TimeController, PLAYER_GRACE_AREA};

pub enum Event {
    PlayerDestroyed,
    EnemyDestroyed,
    PowerupGained,
    ShotFired,
    EnemySpawned,
    GameStart,
}
