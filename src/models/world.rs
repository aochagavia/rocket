use rand::Rng;

use crate::{
    geometry::Size,
    models::{Bullet, Enemy, Particle, Player, Powerup, Star},
};

const MAX_STARS: usize = 100;

/// A model that contains the other models and renders them
pub struct World {
    pub player: Player,
    pub particles: Vec<Particle>,
    pub powerups: Vec<Powerup>,
    pub bullets: Vec<Bullet>,
    pub enemies: Vec<Enemy>,
    pub stars: Vec<Star>,
    pub size: Size,
}

impl World {
    /// Returns a new world of the given size
    pub fn new<R: Rng>(rng: &mut R, size: Size) -> World {
        World {
            player: Player::random(rng, size),
            particles: Vec::with_capacity(1000),
            powerups: vec![],
            bullets: vec![],
            enemies: vec![],
            stars: (0..MAX_STARS).map(|_| Star::new(size, rng)).collect(),
            size: size,
        }
    }
}
