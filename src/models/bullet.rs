use drawing::color;
use super::Vector;
use traits::{Advance, Collide, Position};

use graphics::{Context, Ellipse};
use opengl_graphics::GlGraphics;

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
        Ellipse::new(color::BLUE).resolution(8).draw(
            [self.x() - self.radius(), self.y() - self.radius(), self.diameter(), self.diameter()],
            &c.draw_state, c.transform, gl);
    }

    /// Update the bullet's position
    pub fn update(&mut self, units: f64) {
        self.advance(units);
    }
}

impl Collide for Bullet {
    fn radius(&self) -> f64 { 3.0 }
}
