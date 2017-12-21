use rand::Rng;

use geometry::Size;
use models::{Bullet, Enemy, Particle, Player, Star};

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
        World {
            player: Player::random(rng, size),
            particles: Vec::with_capacity(1000),
            bullets: vec![],
            enemies: vec![],
            stars: vec![],
            size: size
        }
    }
}
