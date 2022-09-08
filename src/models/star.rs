use std;
use rand::Rng;
use geometry::{Advance, Position, Point, Size, Vector};
use geometry_derive::{Advance, Position};

/// Stars glide from right to left across the screen in the background
#[derive(Advance, Position)]
pub struct Star {
    vector: Vector,
    pub speed: f32,
    pub size: f32,
}

impl Star {
    // Create a new star at a random point, with a random speed and size
    pub fn new(bounds: Size, rng: &mut impl Rng) -> Star {
        let point = Point {
            x: rng.gen_range(0.0 .. bounds.width),
            y: rng.gen_range(0.0 .. bounds.height),
        };

        Star {
            vector: Vector::new(point, std::f32::consts::PI),
            speed: rng.gen_range(1.0 .. 3.0),
            size: rng.gen_range(2.0 .. 5.0),
        }
    }
}
