use ggez::graphics::{self, Color, DrawMode, Rect};
use ggez::nalgebra::{Point2, Vector2};
use ggez::{conf, Context, ContextBuilder, GameResult};
use std;

use game_state::Message;
use geometry::{Advance, Collide, Position, Size};
use models::{Player, PowerupKind, World, PLAYER_POLYGON};
use view::drawing::color;
use view::Resources;
use ApplicationState;

const SPRITE_SIZE: f32 = 32.0;
const GUN_HEAT_STATUS_WIDTH: f32 = 100.0;
const GUN_HEAT_STATUS_HEIGHT: f32 = 20.0;

pub fn init_ctx_builder(game_size: Size) -> GameResult<ContextBuilder> {
    Ok(ContextBuilder::new("rocket", "ggez")
        .modules(conf::ModuleConf {
            audio: true,
            gamepad: false,
        })
        .window_setup(conf::WindowSetup::default().title("Rocket!"))
        .window_mode(conf::WindowMode::default().dimensions(game_size.width, game_size.height)))
}

/// Renders the game to the screen
pub fn render_game(app: &mut ApplicationState, ctx: &mut Context) -> GameResult<()> {
    // Clear everything
    graphics::clear(ctx, color::BLACK);

    // Render the world
    render_world(ctx, &app.game_state.world, &mut app.resources)?;

    // Render a message if there is one set
    render_message(ctx, app)?;

    // Render the score
    let text = graphics::Text::new(graphics::TextFragment {
        text: format!("Score: {}", app.game_state.score),
        scale: Some(graphics::Scale::uniform(28.0)),
        ..Default::default()
    });
    let pt = Point2::new(8.0, 4.0);
    graphics::draw(ctx, &text, (pt, color::SCORE))?;

    // Render the gun's heat status in the bottom right of the screen
    let gun = &app.game_state.world.player.gun;
    let gun_color = if !gun.is_available() {
        color::RED
    } else {
        Color {
            r: 1.0 * gun.temperature,
            g: 0.5 - gun.temperature / 2.0,
            b: 1.0 - gun.temperature,
            a: 1.0,
        }
    };

    let Size { width, height } = app.game_state.world.size;
    let x = width - GUN_HEAT_STATUS_WIDTH - 20.0;
    let y = height - 40.0;
    let heat_level_rect = Rect {
        x: x,
        y: y,
        w: GUN_HEAT_STATUS_WIDTH * gun.temperature,
        h: GUN_HEAT_STATUS_HEIGHT,
    };
    let heat_level_mesh = graphics::MeshBuilder::new()
        .rectangle(DrawMode::Fill, heat_level_rect, gun_color)
        .build(ctx)?;
    graphics::draw(ctx, &heat_level_mesh, (Point2::new(0.0, 0.0),))?;

    let heat_border_rect = Rect {
        x: x,
        y: y,
        w: GUN_HEAT_STATUS_WIDTH,
        h: GUN_HEAT_STATUS_HEIGHT,
    };
    let heat_border_mesh = graphics::MeshBuilder::new()
        .rectangle(DrawMode::Line(2.0), heat_border_rect, gun_color)
        .build(ctx)?;
    graphics::draw(ctx, &heat_border_mesh, (Point2::new(0.0, 0.0),))?;

    // NOTE: for limiting FPS rate, see https://github.com/ggez/ggez/issues/171
    // If you want to log the current FPS, uncomment the next line
    // println!("{}", ggez::timer::get_fps(ctx));

    graphics::present(ctx)?;
    Ok(())
}

/// Renders the Message struct contained in the game's state to the middle of the screen
fn render_message(ctx: &mut Context, app: &mut ApplicationState) -> GameResult<()> {
    if let Some(ref message) = app.game_state.message {
        let Message { title, subtitle } = *message;
        let Size { width, height } = app.game_state.world.size;
        let center_y = height / 2.0;

        let mut draw_text =
            |text: &str, color: graphics::Color, is_title: bool| -> GameResult<()> {
                let mut text = graphics::Text::new(graphics::TextFragment {
                    text: text.to_owned(),
                    scale: Some(graphics::Scale::uniform(28.0)),
                    ..Default::default()
                });

                let x = 196.0;
                let y = if is_title {
                    center_y - text.height(&ctx) as f32
                } else {
                    center_y
                };

                text.set_bounds(Point2::new(width, height), graphics::Align::Center);
                graphics::draw(ctx, &text, (Point2::new(x, y), color))
            };

        draw_text(title, color::WHITE, true)?;
        draw_text(subtitle, color::GREY, false)?;
    }

    Ok(())
}

/// Renders the world and everything in it
pub fn render_world(ctx: &mut Context, world: &World, resources: &mut Resources) -> GameResult<()> {
    render_stars(ctx, world, resources)?;
    render_particles(ctx, world, resources)?;
    render_bullets(ctx, world, resources)?;
    render_enemy(ctx, world, resources)?;

    if !world.player.is_dead {
        render_player(ctx, &world.player, resources)?;
    }

    // Draw powerups
    for powerup in &world.powerups {
        let image = match powerup.kind {
            PowerupKind::Shield => &resources.powerup_shield,
            PowerupKind::TimeSlow => &resources.powerup_time_slow,
            PowerupKind::TripleShot => &resources.powerup_triple_shot,
        };
        let scale = powerup.radius() / SPRITE_SIZE;
        let params = graphics::DrawParam::new()
            .dest(Point2::new(powerup.x(), powerup.y()))
            .scale(Vector2::new(scale, scale))
            .color(color::POWERUP);

        graphics::draw(ctx, image, params)?;
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
        let p = graphics::DrawParam::new()
            .dest(Point2::new(star.x(), star.y()))
            .rotation((i as f32 / 100.0) * 2.0 * std::f32::consts::PI)
            .scale(Vector2::new(scale, scale))
            .color(color::STAR);
        resources.star_sprite.add(p);
    }
    graphics::draw(ctx, &resources.star_sprite, graphics::DrawParam::new())
}

/// Renders all the particles
pub fn render_particles(
    ctx: &mut Context,
    world: &World,
    resources: &mut Resources,
) -> GameResult<()> {
    resources.circle_sprite.clear();
    for particle in &world.particles {
        let scale = 0.4 * particle.ttl;
        let p = graphics::DrawParam::new()
            .dest(Point2::new(particle.x(), particle.y()))
            .offset(Point2::new(0.5, 0.5))
            .scale(Vector2::new(scale, scale))
            .color(color::PARTICLE);
        resources.circle_sprite.add(p);
    }
    graphics::draw(ctx, &resources.circle_sprite, graphics::DrawParam::new())
}

/// Renders a bullet
pub fn render_bullets(
    ctx: &mut Context,
    world: &World,
    resources: &mut Resources,
) -> GameResult<()> {
    resources.circle_sprite.clear();
    for bullet in &world.bullets {
        let scale = bullet.radius() / SPRITE_SIZE;
        let p = graphics::DrawParam::new()
            .dest(Point2::new(bullet.x(), bullet.y()))
            .offset(Point2::new(0.5, 0.5))
            .scale(Vector2::new(scale, scale))
            .color(color::BULLET);
        resources.circle_sprite.add(p);
    }
    graphics::draw(ctx, &resources.circle_sprite, graphics::DrawParam::new())
}

/// Renders an enemy
pub fn render_enemy(ctx: &mut Context, world: &World, resources: &mut Resources) -> GameResult<()> {
    resources.circle_sprite.clear();
    for enemy in &world.enemies {
        let scale = enemy.radius() * 2.0 / SPRITE_SIZE;
        let p = graphics::DrawParam::new()
            .dest(Point2::new(enemy.x(), enemy.y()))
            .offset(Point2::new(0.5, 0.5))
            .scale(Vector2::new(scale, scale))
            .color(color::ENEMY);
        resources.circle_sprite.add(p);
    }
    graphics::draw(ctx, &resources.circle_sprite, graphics::DrawParam::new())
}

/// Renders the player
pub fn render_player(ctx: &mut Context, player: &Player, resources: &Resources) -> GameResult<()> {
    // Render shield if one is active
    let pt = Point2::new(player.x(), player.y());
    if let Some(powerup) = player.powerup {
        if powerup == PowerupKind::Shield {
            let offset = SPRITE_SIZE / 2.0;
            let params = graphics::DrawParam::new()
                .dest(Point2::new(player.x() - offset, player.y() - offset))
                .color(color::SHIELD);
            graphics::draw(ctx, &resources.circle_image, params)?;
        }
    }

    // Render the player
    let p1 = Point2::new(PLAYER_POLYGON[0][0], PLAYER_POLYGON[0][1]);
    let p2 = Point2::new(PLAYER_POLYGON[1][0], PLAYER_POLYGON[1][1]);
    let p3 = Point2::new(PLAYER_POLYGON[2][0], PLAYER_POLYGON[2][1]);
    let mesh = graphics::Mesh::new_polygon(ctx, DrawMode::Fill, &[p1, p2, p3], color::PLAYER)?;
    let params = graphics::DrawParam::new()
        .dest(pt)
        .rotation(player.direction());
    graphics::draw(ctx, &mesh, params)
}
