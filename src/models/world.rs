use graphics;
use opengl_graphics::GlGraphics;
use rand::Rng;

use drawing::Size;
use models::{Bullet, Enemy, Particle, Player};

/// A model that contains the other models and renders them
pub struct World {
    pub player: Player,
    pub particles: Vec<Particle>,
    pub bullets: Vec<Bullet>,
    pub enemies: Vec<Enemy>,
    pub size: Size
}

impl World {
    /// Returns a new world of the given size
    pub fn new<R: Rng>(rng: &mut R, size: Size) -> World {
        World {
            player: Player::random(rng, size.clone()),
            particles: Vec::with_capacity(1000),
            bullets: vec![],
            enemies: vec![],
            size: size
        }
    }

    /// Renders the world and everything in it
    pub fn render(&self, c: graphics::context::Context, g: &mut GlGraphics) {
        for particle in &self.particles {
            particle.draw(&c, g);
        }

        for bullet in &self.bullets {
            bullet.draw(&c, g);
        }

        for enemy in &self.enemies {
            enemy.draw(&c, g);
        }

        self.player.draw(&c, g);
    }
}
