use std;
use ggez::graphics::{self, DrawMode, Point2};
use ggez::{Context, GameResult};

use ApplicationState;
use Resources;
use drawing::color;
use geometry::{Advance, Collide, Position, Size};
use models::{Player, World, PowerupKind, PLAYER_POLYGON};
use game_state::Message;

const SPRITE_SIZE: f32 = 32.0;

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
    graphics::set_color(ctx, color::SCORE)?;
    graphics::draw(ctx, &text, pt, 0.0)?;

    // Render the resource
    let text = graphics::Text::new(ctx, &format!("Heat: {} / {}", app.game_state.world.player.resource.status(), app.game_state.world.player.resource.capacity()), &app.resources.font)?;
    let pt = Point2::new(8.0, 50.0);
    graphics::set_color(ctx, color::SCORE)?;
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

        let w = width / 2.0;
        let h = height / 2.0;

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
    graphics::set_color(ctx, color::STAR)?;
    render_stars(ctx, world, resources)?;

    // Draws particles in violet
    graphics::set_color(ctx, color::PARTICLE)?;
    render_particles(ctx, world, resources)?;
    
    // Draw any bullets as blue
    graphics::set_color(ctx, color::BULLET)?;
    render_bullets(ctx, world, resources)?;

    // Now we draw the enemies as yellow
    graphics::set_color(ctx, color::ENEMY)?;
    render_enemy(ctx, world, resources)?;

    // Finally draw the player as red
    if !world.player.is_dead {
        render_player(ctx, &world.player, resources)?;
    }

    // Draw powerups
    graphics::set_color(ctx, color::POWERUP)?;
    for powerup in &world.powerups {
        let image = match powerup.kind {
            PowerupKind::Shield => &resources.powerup_shield,
            PowerupKind::TimeSlow => &resources.powerup_time_slow,
            PowerupKind::TripleShot => &resources.powerup_triple_shot
        };
        let scale = powerup.radius() / SPRITE_SIZE;
        let params = graphics::DrawParam {
            dest: Point2::new(powerup.x(), powerup.y()),
            scale: Point2::new(scale, scale),
            ..Default::default()
        };

        graphics::draw_ex(ctx, image, params)?;
    }

    Ok(())
}

/// Renders all the stars in the background
fn render_stars(ctx: &mut Context, world: &World, resources: &mut Resources) -> GameResult<()> {
    resources.star_sprite.clear();
    // Iterate through the stars list and draw them with a rotation based on their index in the
    // list - this isn't a truly random rotation, but it works visually
    for (i, star) in world.stars.iter().enumerate() {
        let scale = star.size / SPRITE_SIZE;
        resources.star_sprite.add(graphics::DrawParam {
            dest: Point2::new(star.x(), star.y()),
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
        let scale = 0.4 * particle.ttl;
        resources.circle_sprite.add(graphics::DrawParam {
            dest: Point2::new(particle.x(), particle.y()),
            offset: Point2::new(0.5, 0.5),
            scale: Point2::new(scale, scale),
            ..Default::default()
        });
    }
    graphics::draw_ex(ctx, &resources.circle_sprite, graphics::DrawParam { ..Default::default() })
}

/// Renders a bullet
pub fn render_bullets(ctx: &mut Context, world: &World, resources: &mut Resources) -> GameResult<()> {
    resources.circle_sprite.clear();
    for bullet in &world.bullets {
        let scale = bullet.radius() / SPRITE_SIZE;
        resources.circle_sprite.add(graphics::DrawParam {
            dest: Point2::new(bullet.x(), bullet.y()),
            offset: Point2::new(0.5, 0.5),
            scale: Point2::new(scale, scale),
            ..Default::default()
        });
    }
    graphics::draw_ex(ctx, &resources.circle_sprite, graphics::DrawParam { ..Default::default() })
}

/// Renders an enemy
pub fn render_enemy(ctx: &mut Context, world: &World, resources: &mut Resources) -> GameResult<()> {
    resources.circle_sprite.clear();
    for enemy in &world.enemies {
        let scale = enemy.radius() * 2.0 / SPRITE_SIZE;
        resources.circle_sprite.add(graphics::DrawParam {
            dest: Point2::new(enemy.x(), enemy.y()),
            offset: Point2::new(0.5, 0.5),
            scale: Point2::new(scale, scale),
            ..Default::default()
        });
    }
    graphics::draw_ex(ctx, &resources.circle_sprite, graphics::DrawParam { ..Default::default() })
}

/// Renders the player
pub fn render_player(ctx: &mut Context, player: &Player, resources: &Resources) -> GameResult<()> {
    // Render shield if one is active
    let pt = Point2::new(player.x(), player.y());
    if let Some(powerup) = player.powerup {
        if powerup == PowerupKind::Shield {
            let scale = (player.radius() + 30.0) / SPRITE_SIZE;
            let params = graphics::DrawParam {
                dest: pt,
                offset: Point2::new(0.5, 0.5),
                scale: Point2::new(scale, scale),
                ..Default::default()
            };
            graphics::set_color(ctx, color::SHEILD)?;
            graphics::draw_ex(ctx, &resources.circle_image, params)?;
        }
    }

    // Render the player
    graphics::set_color(ctx, color::PLAYER)?;
    let p1 = Point2::new(PLAYER_POLYGON[0][0], PLAYER_POLYGON[0][1]);
    let p2 = Point2::new(PLAYER_POLYGON[1][0], PLAYER_POLYGON[1][1]);
    let p3 = Point2::new(PLAYER_POLYGON[2][0], PLAYER_POLYGON[2][1]);
    let mesh = graphics::Mesh::new_polygon(ctx, DrawMode::Fill, &[p1, p2, p3])?;
    let dir = player.direction();
    graphics::draw(ctx, &mesh, pt, dir)
}
