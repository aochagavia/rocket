use std;
use rand::{self, Rng};
use geometry::{Point, Size, Vector};

/// Stars glide from right to left across the screen in the background
pub struct Star {
    vector: Vector,
    pub speed: f32,
    pub size: f32,
}

derive_position_direction!(Star);

impl Star {
    // Create a new star at a random point, with a random speed and size
    pub fn new(bounds: Size) -> Star {
        let mut rng = rand::thread_rng();

        let point = Point {
            x: rng.gen_range(0.0, bounds.width),
            y: rng.gen_range(0.0, bounds.height),
        };

        Star {
            vector: Vector::new(point, std::f32::consts::PI),
            speed: rng.gen_range(1.0, 3.0),
            size: rng.gen_range(2.0, 5.0),
        }
    }
}
