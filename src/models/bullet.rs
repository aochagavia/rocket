use super::Vector;
use traits::{Advance, Collide, Position};

use graphics::{Context, Ellipse};
use opengl_graphics::GlGraphics;
use drawing::Size;

/// Bullets are spawned when the player shoots
///
/// When an enemy is reached by a bullet, it will explode
pub struct Bullet {
    vector: Vector,
    pub ttl: f64,
    pub start_ttl: f64
}

derive_position_direction!(Bullet);

impl Bullet {
    /// Create a bullet with the given vector
    pub fn new(vector: Vector) -> Bullet {
        Bullet { vector: vector, ttl: 1.5, start_ttl: 1.5 }
    }
    
    pub fn new_dur(vector: Vector, duration: f64) -> Bullet {
        Bullet { vector: vector, ttl: duration, start_ttl: duration }
    }

    /// Draw the bullet
    pub fn draw(&self, c: &Context, gl: &mut GlGraphics) {
        let dia = self.diameter() * (self.ttl/self.start_ttl) + 0.1;
        let ttl32 = self.ttl as f32;
        let start_ttl32 = self.start_ttl as f32;
        Ellipse::new([f32::min(1f32, 1.2*start_ttl32 - ttl32), 0f32, ttl32/start_ttl32, 1f32]).resolution(8).draw(
            [self.x() - self.radius(), self.y() - self.radius(), dia, dia],
            &c.draw_state, c.transform, gl);
    }

    /// Update the bullet's position
    pub fn update(&mut self, units: f64, bounds: Size, elapsed_time: f64) {
        self.ttl -= elapsed_time;
        let ttl = self.ttl;
        self.advance_wrapping(units * ttl, bounds);
    }
}

impl Collide for Bullet {
    fn radius(&self) -> f64 { 3.0 }
}
