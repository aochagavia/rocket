use rand::Rng;

use super::Size;

/// A `Point` represents a position in space
#[derive(Clone, Default)]
pub struct Point {
    pub x: f64,
    pub y: f64
}

impl Point {
    /// Returns a new `Point` with the given coordinates
    pub fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y}
    }

    /// Returns a random `Point` within the given bounds (exclusive)
    pub fn random<R: Rng>(rng: &mut R, bounds: Size) -> Point {
        Point {
            x: rng.gen_range(0.0, bounds.width),
            y: rng.gen_range(0.0, bounds.height)
        }
    }

    /// Returns the squared distance from this point to the given one
    pub fn squared_distance_to(&self, target: &Point) -> f64 {
        (self.x - target.x) * (self.x - target.x)
        + (self.y - target.y) * (self.y - target.y)
    }

    /// Rotates the point through the origin in the given angle (radians)
    pub fn rotate(mut self, radians: f64) -> Point {
        let radius = (self.x * self.x + self.y * self.y).sqrt();
        let point_angle = (self.y / self.x).atan();
        let final_angle = point_angle + radians;
        self.x = final_angle.cos() * radius;
        self.y = final_angle.sin() * radius;
        self
    }

    /// Translates the point by another point
    pub fn translate(mut self, other: &Point) -> Point {
        self.x += other.x;
        self.y += other.y;
        self
    }
}
