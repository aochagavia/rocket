use drawing::color;
use super::Vector;
use traits::{Advance, Position};

use opengl_graphics::GlGraphics;
use piston_window::{Context, ellipse};

/// A model representing a particle
///
/// Particles are visible objects that have a time to live and move around
/// in a given direction until their time is up. They are spawned when the
/// player or an enemy is killed
pub struct Particle {
    pub vector: Vector,
    pub ttl: f64
}

derive_position_direction!(Particle);

impl Particle {
    /// Create a particle with the given vector and time to live in seconds
    pub fn new(vector: Vector, ttl: f64) -> Particle {
        Particle { vector: vector, ttl: ttl }
    }

    /// Draw the particle
    pub fn draw(&self, c: &Context, gl: &mut GlGraphics) {
        let radius = 5.0 * self.ttl;
        ellipse(color::VIOLET,
                [self.x() - radius, self.y() - radius, radius * 2.0, radius * 2.0],
                c.transform, gl);
    }

    /// Update the particle
    pub fn update(&mut self, elapsed_time: f64) {
        self.ttl -= elapsed_time;
        let speed = 500.0 * self.ttl * self.ttl;
        self.advance(elapsed_time * speed);
    }
}
