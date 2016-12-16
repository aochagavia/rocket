use super::Vector;
use traits::{Advance, Collide};

/// Bullets are spawned when the player shoots
///
/// When an enemy is reached by a bullet, it will explode
pub struct Bullet {
    vector: Vector,
    pub ttl: f64
}

derive_position_direction!(Bullet);

impl Bullet {
    /// Create a bullet with the given vector
    pub fn new(vector: Vector) -> Bullet {
        Bullet { vector: vector, ttl: 4.0 }
    }

    /// Create a bullet with the given vector and ttl
    pub fn new_ttl(vector: Vector, ttl: f64) -> Bullet {
        Bullet { vector: vector, ttl: ttl }
    }

    /// Update the bullet's position
    pub fn update(&mut self, dt: f64) {
        self.ttl -= dt;
        self.advance(dt * 500.0);
    }
}

impl Collide for Bullet {
    fn radius(&self) -> f64 { 3.0 }
}
