use geometry::{Advance, Collide, Vector};

/// Bullets are spawned when the player shoots
///
/// When an enemy is reached by a bullet, it will explode
#[derive(Clone)]
pub struct Bullet {
    vector: Vector,
}

derive_position_direction!(Bullet);

impl Bullet {
    /// Create a bullet with the given vector
    pub fn new(vector: Vector) -> Bullet {
        Bullet { vector: vector }
    }

    /// Update the bullet's position
    pub fn update(&mut self, units: f32) {
        self.advance(units);
    }
}

impl Collide for Bullet {
    fn radius(&self) -> f32 {
        3.0
    }
}
