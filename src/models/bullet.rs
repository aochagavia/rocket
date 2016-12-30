use drawing::color;
use super::Vector;
use traits::{Advance, Collide, Position};

use opengl_graphics::GlGraphics;
use piston_window::{Context, ellipse};

/// Bullets are spawned when the player shoots
///
/// When an enemy is reached by a bullet, it will explode
pub struct Bullet {
    vector: Vector
}

derive_position_direction!(Bullet);

impl Bullet {
    /// Create a bullet with the given vector
    pub fn new(vector: Vector) -> Bullet {
        Bullet { vector: vector }
    }

    /// Draw the bullet
    pub fn draw(&self, c: &Context, gl: &mut GlGraphics) {
        ellipse(color::BLUE,
                [self.x() - self.radius(),
                    self.y() - self.radius(),
                    self.diameter(),
                    self.diameter()],
                c.transform,
                gl);
    }

    /// Update the bullet's position
    pub fn update(&mut self, units: f64) {
        self.advance(units);
    }
}

impl Collide for Bullet {
    fn radius(&self) -> f64 { 3.0 }
}
