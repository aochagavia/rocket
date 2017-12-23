use rand::Rng;

use geometry::Size;
use models::{Bullet, Enemy, Particle, Player, Star};

const MAX_STARS: usize = 100;

/// A model that contains the other models and renders them
pub struct World {
    pub player: Player,
    pub particles: Vec<Particle>,
    pub bullets: Vec<Bullet>,
    pub enemies: Vec<Enemy>,
    pub stars: Vec<Star>,
    pub size: Size
}

impl World {
    /// Returns a new world of the given size
    pub fn new<R: Rng>(rng: &mut R, size: Size) -> World {

        // Add stars up to MAX_STARS
        let mut stars = vec![];
        while stars.len() < MAX_STARS {
            stars.push(Star::new(size));
        }

        World {
            player: Player::random(rng, size),
            particles: Vec::with_capacity(1000),
            bullets: vec![],
            enemies: vec![],
            stars: stars,
            size: size
        }
    }
}
