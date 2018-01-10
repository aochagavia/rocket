use rand::Rng;

use geometry::{Point, Size};
use super::PowerupKind;
use geometry::{Advance, Collide, Vector, Position};

/// The `Player` is the rocket controlled by the user
#[derive(Default)]
pub struct Player {
    pub vector: Vector,
    pub is_dead: bool,
    pub powerup: Option<PowerupKind>
}

derive_position_direction!(Player);

/// The player is represented as the polygon below
pub const POLYGON: &'static [[f32; 2]] = &[
    [-10.0, -8.0],
    [10.0, 0.0],
    [-10.0, 8.0]
];

impl Player {
    /// Create a new `Player` with a random position and direction
    pub fn random<R: Rng>(rng: &mut R, bounds: Size) -> Player {
        Player {
            vector: Vector::random(rng, bounds),
            is_dead: true,
            powerup: None
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
    fn radius(&self) -> f32 { 6.0 }
}
