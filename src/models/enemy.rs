use crate::geometry::{Point, Size, Advance, Collide, Vector};

/// Enemies follow the player in order to cause a collision and let him explode
pub struct Enemy {
    vector: Vector,
}

derive_position_direction!(Enemy);

impl Enemy {
    /// Create a enemy with the given vector
    pub fn new(vector: Vector) -> Enemy {
        Enemy { vector: vector }
    }

    /// Update the enemy
    pub fn update(&mut self, speed: f32, player_position: Point, size: Size) {
        let vector_position = self.vector.position;
        // Point to the player
        self.point_to(nearest_virtual_position(
            vector_position,
            player_position,
            size
        ));
        self.advance_wrapping(speed, size);
    }

}

fn nearest_virtual_position(origin: Point, destination: Point, size: Size) -> Point {
    let mut nearest = destination;
    for i in -1..2 {
        for j in -1..2 {
            // A point where the enemy "sees" one of the player copies.
            let virtual_position = destination + Point{
                x: size.width * i as f32,
                y: size.height * j as f32,
            };
            if origin.squared_distance_to(virtual_position)
                < origin.squared_distance_to(nearest)
            {
                nearest = virtual_position;
            }
        }
    }
    nearest
}

impl Collide for Enemy {
    fn radius(&self) -> f32 {
        10.0
    }
}
