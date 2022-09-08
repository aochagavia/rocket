use rand::Rng;

use super::Size;

use std::ops::{Add, Div, Mul, Sub};

/// A `Point` represents a position in space
#[derive(Clone, Default, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    /// Returns a new `Point` with the given coordinates
    pub fn new(x: f32, y: f32) -> Point {
        Point { x: x, y: y }
    }

    /// Returns a random `Point` within the given bounds (exclusive)
    pub fn random<R: Rng>(rng: &mut R, bounds: Size) -> Point {
        Point {
            x: rng.gen_range(0.0 .. bounds.width),
            y: rng.gen_range(0.0 .. bounds.height),
        }
    }

    /// Returns the squared distance from this point to the given one
    pub fn squared_distance_to(self, target: Point) -> f32 {
        (self.x - target.x) * (self.x - target.x) + (self.y - target.y) * (self.y - target.y)
    }

    /// Rotates the point through the origin in the given angle (radians)
    pub fn rotate(mut self, radians: f32) -> Point {
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

    /// Checks if this point is contained in a circle
    pub fn intersect_circle(self, center: &Point, radius: f32) -> bool {
        (self.x - center.x).powi(2) + (self.y - center.y).powi(2) < radius.powi(2)
    }
}

/// Implements '==' for Point, as well as its inverse '!='
impl PartialEq for Point {
    fn eq(&self, _rhs: &Self) -> bool {
        (self.x == _rhs.x) && (self.y == _rhs.y)
    }
}

/// Implements the '+' operator for Point + Point
impl Add for Point {
    type Output = Point;

    fn add(self, _rhs: Point) -> Point {
        Point {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

/// Implements the '+' operator for Point + f32
impl Add<f32> for Point {
    type Output = Point;

    fn add(self, _rhs: f32) -> Point {
        Point {
            x: self.x + _rhs,
            y: self.y + _rhs,
        }
    }
}

/// Implements the '-' operator for Point - Point
impl Sub for Point {
    type Output = Point;

    fn sub(self, _rhs: Point) -> Point {
        Point {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
        }
    }
}

/// Implements the '-' operator for Point - f32
impl Sub<f32> for Point {
    type Output = Point;

    fn sub(self, _rhs: f32) -> Point {
        Point {
            x: self.x - _rhs,
            y: self.y - _rhs,
        }
    }
}

/// Implements the '*' operator for Point * Point
impl Mul for Point {
    type Output = Point;

    fn mul(self, _rhs: Point) -> Point {
        Point {
            x: self.x * _rhs.x,
            y: self.y * _rhs.y,
        }
    }
}

/// Implements the '*' operator for Point * f32
impl Mul<f32> for Point {
    type Output = Point;

    fn mul(self, _rhs: f32) -> Point {
        Point {
            x: self.x * _rhs,
            y: self.x * _rhs,
        }
    }
}

/// Implements the '/' operator for Point / Point
impl Div for Point {
    type Output = Point;

    fn div(self, _rhs: Point) -> Point {
        assert!(_rhs.x != 0f32);
        assert!(_rhs.y != 0f32);
        Point {
            x: self.x / _rhs.x,
            y: self.y / _rhs.y,
        }
    }
}

/// Implements the '/' operator for Point / f32:
impl Div<f32> for Point {
    type Output = Point;

    fn div(self, _rhs: f32) -> Point {
        assert!(_rhs != 0f32);
        Point {
            x: self.x / _rhs,
            y: self.y / _rhs,
        }
    }
}
