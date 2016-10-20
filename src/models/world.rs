use graphics::{self, Transformed};
use opengl_graphics::GlGraphics;
use rand::Rng;

use drawing::{Camera, Size, Point};
use models::{Bullet, Enemy, Particle, Player, Vector};

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
    pub fn new(size: Size) -> World {
        World {
            player: Player::new(),
            particles: Vec::with_capacity(1000),
            bullets: Vec::with_capacity(100),
            enemies: Vec::with_capacity(1000),
            size: size
        }
    }

    /// Renders the world and everything in it
    pub fn render(&self, c: graphics::context::Context, g: &mut GlGraphics, cam: &Camera) {
        let new_context = c.trans(-cam.pos.x, -cam.pos.y);
        for particle in &self.particles {
            particle.draw(&new_context, g);
        }

        for bullet in &self.bullets {
            bullet.draw(&new_context, g);
        }

        for enemy in &self.enemies {
            enemy.draw(&new_context, g);
        }

        self.player.draw(&new_context, g);
    }
}
