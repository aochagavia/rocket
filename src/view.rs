use std;
use ggez::graphics::{self, DrawMode, Point2};
use ggez::{Context, GameResult};

use ApplicationState;
use Resources;
use drawing::color;
use geometry::{Advance, Collide, Position, Size};
use models::{Bullet, Player, World, PLAYER_POLYGON};
use game_state::Message;

/// Renders the game to the screen
pub fn render_game(app: &mut ApplicationState, ctx: &mut Context) -> GameResult<()> {
    // Clear everything
    graphics::clear(ctx);

    // Render the world
    render_world(ctx, &app.game_state.world, &mut app.resources)?;

    // Render a message if there is one set
    render_message(ctx, app)?;

    // Render the score
    let text = graphics::Text::new(ctx, &format!("Score: {}", app.game_state.score), &app.resources.font)?;
    let pt = Point2::new(8.0, 4.0);
    graphics::set_color(ctx, color::VIOLET)?;
    graphics::draw(ctx, &text, pt, 0.0)?;

    // NOTE: for limiting FPS rate, see https://github.com/ggez/ggez/issues/171
    // If you want to log the current FPS, uncomment the next line
    // println!("{}", ggez::timer::get_fps(ctx));

    graphics::present(ctx);
    Ok(())
}

/// Renders the Message struct contained in the game's state to the middle of the screen
fn render_message(ctx: &mut Context, app: &mut ApplicationState) -> GameResult<()> {
    if let Some(ref message) = app.game_state.message {
        let Message { title, subtitle } = *message;
        let Size { width, height } = app.game_state.world.size;

        let w = width as f32 / 2.0;
        let h = height as f32 / 2.0;

        let mut draw_text = |text: &str, color: graphics::Color, is_title: bool| {
            let drawable = graphics::Text::new(ctx, text, &app.resources.font).unwrap();
            let width = w - (drawable.width() as f32 / 2.0);
            let height = if is_title { h - drawable.height() as f32 } else { h };
            let point = Point2::new(width, height);
            graphics::set_color(ctx, color).unwrap();
            graphics::draw(ctx, &drawable, point, 0.0).unwrap();
        };

        draw_text(title, color::WHITE, true);
        draw_text(subtitle, color::GREY, false);
    }

    Ok(())
}

/// Renders the world and everything in it
pub fn render_world(ctx: &mut Context, world: &World, resources: &mut Resources) -> GameResult<()> {
    // Render stars in the background
    graphics::set_color(ctx, color::GREY)?;
    render_stars(ctx, world, resources)?;

    // Draws particles in violet
    graphics::set_color(ctx, color::ORANGE)?;
    render_particles(ctx, world, resources)?;
    
    // Draw any bullets as blue
    graphics::set_color(ctx, color::BLUE)?;
    for bullet in &world.bullets {
        render_bullet(ctx, bullet)?;
    }

    // Now we draw the enemies as yellow
    graphics::set_color(ctx, color::YELLOW)?;
    render_enemy(ctx, world, resources)?;

    // Finally draw the player as red
    if !world.player.is_dead {
        graphics::set_color(ctx, color::RED)?;
        render_player(ctx, &world.player)?;
    }

    Ok(())
}

/// Renders all the stars in the background
fn render_stars(ctx: &mut Context, world: &World, resources: &mut Resources) -> GameResult<()> {
    resources.star_sprite.clear();
    // Iterate through the stars list and draw them with a rotation based on their index in the
    // list - this isn't a truly random rotation, but it works visually
    for (i, star) in world.stars.iter().enumerate() {
        let scale = 0.05 * (star.size as f32);
        resources.star_sprite.add(graphics::DrawParam {
            dest: Point2::new(star.x() as f32, star.y() as f32),
            rotation: (i as f32 / 100.0) * 2.0 * std::f32::consts::PI,
            scale: Point2::new(scale, scale),
            .. Default::default()
        });
    }
    graphics::draw_ex(ctx, &resources.star_sprite, graphics::DrawParam { ..Default::default() })
}

/// Renders all the particles
pub fn render_particles(ctx: &mut Context, world: &World, resources: &mut Resources) -> GameResult<()> {
    resources.circle_sprite.clear();
    for particle in &world.particles {
        let scale = (0.4 * particle.ttl) as f32;
        resources.circle_sprite.add(graphics::DrawParam {
            dest: Point2::new(particle.x() as f32, particle.y() as f32),
            offset: Point2::new(0.5, 0.5),
            scale: Point2::new(scale, scale),
            ..Default::default()
        });
    }
    graphics::draw_ex(ctx, &resources.circle_sprite, graphics::DrawParam { ..Default::default() })
}

/// Renders a bullet
pub fn render_bullet(ctx: &mut Context, bullet: &Bullet) -> GameResult<()> {
    let pt = Point2::new(bullet.x() as f32, bullet.y() as f32);
    graphics::circle(ctx, DrawMode::Fill, pt, bullet.radius() as f32, 2.0)
}

/// Renders an enemy
pub fn render_enemy(ctx: &mut Context, world: &World, resources: &mut Resources) -> GameResult<()> {
    resources.circle_sprite.clear();
    for enemy in &world.enemies {
        resources.circle_sprite.add(graphics::DrawParam {
            dest: Point2::new(enemy.x() as f32, enemy.y() as f32),
            offset: Point2::new(0.5, 0.5),
            scale: Point2::new(0.65, 0.65),
            ..Default::default()
        });
    }
    graphics::draw_ex(ctx, &resources.circle_sprite, graphics::DrawParam { ..Default::default() })
}

/// Render the player
pub fn render_player(ctx: &mut Context, player: &Player) -> GameResult<()> {
    let p1 = Point2::new(PLAYER_POLYGON[0][0] as f32, PLAYER_POLYGON[0][1] as f32);
    let p2 = Point2::new(PLAYER_POLYGON[1][0] as f32, PLAYER_POLYGON[1][1] as f32);
    let p3 = Point2::new(PLAYER_POLYGON[2][0] as f32, PLAYER_POLYGON[2][1] as f32);

    let mesh = graphics::Mesh::new_polygon(ctx, DrawMode::Fill, &[p1, p2, p3])?;
    let pt = Point2::new(player.x() as f32, player.y() as f32);
    let dir = player.direction() as f32;
    graphics::draw(ctx, &mesh, pt, dir)
}
