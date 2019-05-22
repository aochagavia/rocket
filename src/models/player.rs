use rand::Rng;

use super::{Gun, PowerupKind};
use geometry::{Advance, Collide, Point, Position, Size, Vector};
use geometry_derive::{Advance, Position};

/// The `Player` is the rocket controlled by the user
#[derive(Default, Advance, Position)]
pub struct Player {
    pub vector: Vector,
    pub is_dead: bool,
    pub powerup: Option<PowerupKind>,
    pub gun: Gun,
}

/// The player is represented as the polygon below
pub const POLYGON: &'static [[f32; 2]] = &[[-10.0, -8.0], [10.0, 0.0], [-10.0, 8.0]];

impl Player {
    /// Create a new `Player` with a random position and direction
    pub fn random<R: Rng>(rng: &mut R, bounds: Size) -> Player {
        Player {
            vector: Vector::random(rng, bounds),
            is_dead: true,
            powerup: None,
            gun: Gun::new(),
        }
    }

    /// Returns the front of the rocket
    pub fn front(&self) -> Point {
        Point::new(POLYGON[1][0], POLYGON[1][1])
            .rotate(self.direction())
            .translate(&self.position())
    }
}

impl Collide for Player {
    fn radius(&self) -> f32 {
        6.0
    }
}
