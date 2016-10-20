use graphics::{self, triangulation, Context, Graphics, Transformed};
use opengl_graphics::GlGraphics;

use drawing::{color, Camera, Size};
use models::{Bullet, Enemy, Particle, Player};
use traits::{Collide, Position};

/// A model that contains the other models and renders them
pub struct World {
    pub player: Player,
    pub particles: Vec<Particle>,
    pub bullets: Vec<Bullet>,
    pub enemies: Vec<Enemy>,
    pub size: Size,
}

impl World {
    /// Returns a new world of the given size
    pub fn new(size: Size) -> World {
        World {
            player: Player::new(),
            particles: Vec::with_capacity(1000),
            bullets: Vec::with_capacity(100),
            enemies: Vec::with_capacity(1000),
            size: size,
        }
    }

    /// Renders the world and everything in it
    pub fn render(&self, c: graphics::context::Context, g: &mut GlGraphics, cam: &Camera, triangle_buffer: &mut Vec<f32>) {
        let c = c.trans(-cam.pos.x, -cam.pos.y);
        let particles = self.particles.iter().map(|p| {
            let radius = 5.0 * p.ttl;
            [p.x() - radius, p.y() - radius, radius * 2.0, radius * 2.0]
        });
        draw_ellipses(&c, g, triangle_buffer, &color::VIOLET, 8, particles);

        let bullets = self.bullets.iter().map(|b| [b.x() - b.radius(), b.y() - b.radius(), b.diameter(), b.diameter()]);
        draw_ellipses(&c, g, triangle_buffer, &color::BLUE, 8, bullets);

        let enemies = self.enemies.iter().map(|e| [e.x() - 10.0, e.y() - 10.0, 20.0, 20.0]);
        draw_ellipses(&c, g, triangle_buffer, &color::YELLOW, 16, enemies);

        self.player.draw(&c, g);
    }
}

fn draw_ellipses<I>(c: &Context, g: &mut GlGraphics, buffer: &mut Vec<f32>, color: &[f32; 4], resolution: u32, rects: I)
where I: Iterator<Item=[f64; 4]> {
    buffer.clear();
    for rect in rects {
        triangulation::with_ellipse_tri_list(resolution, c.transform, rect, |vertices| {
            for &v in vertices {
                buffer.push(v);
            }
        });
    }
    g.tri_list(&c.draw_state, &color, |f| f(buffer));
}