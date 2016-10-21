use drawing::Point;
use super::Vector;
use traits::{Advance, Collide};

/// Enemies follow the player in order to cause a collision and let him explode
pub struct Enemy {
    vector: Vector,
    size: f64
}

derive_position_direction!(Enemy);

impl Enemy {
    /// Create a enemy with the given vector
    pub fn new(vector: Vector) -> Enemy {
        Enemy { vector: vector, size: 1.0 }
    }

    /// Update the enemy
    pub fn update(&mut self, speed: f64, player_position: Point) {
        // Point to the player
        self.point_to(player_position);
        self.advance(speed);
    }

    pub fn melt(&mut self, other: &Self) {
        self.size += other.size;
    }
}

impl Collide for Enemy {
    fn radius(&self) -> f64 { self.size * 10.0 }
}
