use geometry::{Advance, Collide, Size, Vector};

const BULLET_DISTANCE: f32 = 384.;

/// Bullets are spawned when the player shoots
///
/// When an enemy is reached by a bullet, it will explode
#[derive(Clone)]
pub struct Bullet {
    vector: Vector,
    distance_left: f32,
}

derive_position_direction!(Bullet);

impl Bullet {
    /// Create a bullet with the given vector
    pub fn new(vector: Vector) -> Bullet {
        Bullet {
            vector,
            distance_left: BULLET_DISTANCE,
        }
    }

    /// Update the bullet's position
    pub fn update(&mut self, units: f32, size: Size) {
        self.advance_wrapping(units, size);
        self.distance_left -= units;
    }

    /// Check if the bullet needs to be destroyed
    pub fn reached_max_distance(&self) -> bool {
        self.distance_left <= 0.
    }
}

impl Collide for Bullet {
    fn radius(&self) -> f32 {
        3.0
    }
}
