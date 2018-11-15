use rand::Rng;
use std::f32;

use super::{Point, Size};

/// A `Vector`
#[derive(Clone, Default)]
pub struct Vector {
    /// The position of the vector
    pub position: Point,
    /// The direction angle, in radians
    pub direction: f32,
}

impl Vector {
    /// Returns a new `Vector`
    pub fn new(position: Point, direction: f32) -> Vector {
        Vector {
            position: position,
            direction: direction,
        }
    }

    /// Returns a random `Vector` within the given bounds
    pub fn random<R: Rng>(rng: &mut R, bounds: Size) -> Vector {
        Vector::new(
            Point::random(rng, bounds),
            rng.gen::<f32>() * 2.0 * f32::consts::PI,
        )
    }

    /// Consumes the vector and returns a new one with inverted direction
    pub fn invert(mut self) -> Vector {
        self.direction -= f32::consts::PI;
        self
    }
}

/// A macro to implement `Position` and `Direction` for any type that has a field named `vector`
#[macro_export]
macro_rules! derive_position_direction {
    ($t:ty) => {
        impl ::geometry::Position for $t {
            fn x(&self) -> f32 {
                self.vector.position.x
            }
            fn y(&self) -> f32 {
                self.vector.position.y
            }
            fn x_mut(&mut self) -> &mut f32 {
                &mut self.vector.position.x
            }
            fn y_mut(&mut self) -> &mut f32 {
                &mut self.vector.position.y
            }
        }

        impl ::geometry::Advance for $t {
            fn direction(&self) -> f32 {
                self.vector.direction
            }

            fn direction_mut(&mut self) -> &mut f32 {
                &mut self.vector.direction
            }
        }
    };
}
