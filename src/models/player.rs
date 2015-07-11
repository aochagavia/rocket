use graphics::{Context, Polygon, Transformed};
use opengl_graphics::GlGraphics;
use rand::Rng;

use drawing::{color, Point, Size};
use super::Vector;
use traits::{Advance, Collide, Position};

/// The `Player` is the rocket controlled by the user
#[derive(Default)]
pub struct Player {
    pub vector: Vector
}

derive_position_direction!(Player);

/// The player is drawn as the triangle below
const POLYGON: &'static [[f64; 2]] = &[
    [0.0, -8.0],
    [20.0, 0.0],
    [0.0, 8.0]
];

impl Player {
    /// Create a new `Player` with a random position and direction
    pub fn random<R: Rng>(rng: &mut R, bounds: Size) -> Player {
        Player { vector: Vector::random(rng, bounds) }
    }

    /// Draw the player
    pub fn draw(&self, c: &Context, gl: &mut GlGraphics) {
        // Set the center of the player as the origin and rotate it
        let transform = c.transform.trans(self.x(), self.y())
                                   .rot_rad(self.direction());

        // Draw a rectangle on the position of the player
        Polygon::new(color::RED).draw(POLYGON, &c.draw_state, transform, gl);
    }

    /// Returns the nose of the rocket
    pub fn nose(&self) -> Point {
        Point::new(POLYGON[1][0], POLYGON[1][1])
            .rotate(self.direction())
            .translate(&self.position())
    }
}

impl Collide for Player {
    fn radius(&self) -> f64 { 6.0 }
}
