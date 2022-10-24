use geometry::Point;
use ggez::event::EventLoop;
use ggez::graphics::{
    self, Canvas, Color, DrawMode, DrawParam, Drawable, FillOptions, Mesh, Rect, StrokeOptions,
    TextFragment,
};
use ggez::mint::Point2;
use ggez::{conf, Context, ContextBuilder, GameResult};
use std;

use crate::{
    game_state::Message,
    geometry::{Advance, Collide, Position, Size},
    models::{Player, PowerupKind, World, PLAYER_POLYGON},
    view::drawing::color,
    view::Resources,
    ApplicationState,
};

const SPRITE_SIZE: f32 = 32.0;
const GUN_HEAT_STATUS_WIDTH: f32 = 100.0;
const GUN_HEAT_STATUS_HEIGHT: f32 = 20.0;

pub fn init_rendering_ctx(game_size: Size) -> GameResult<(Context, EventLoop<()>)> {
    let cb = ContextBuilder::new("rocket", "ggez")
        .window_setup(conf::WindowSetup::default().title("Rocket!"))
        .window_mode(conf::WindowMode::default().dimensions(game_size.width, game_size.height));

    let ctx = cb.build()?;
    Ok(ctx)
}

/// Renders the game to the screen
pub fn render_game(app: &mut ApplicationState, ctx: &mut Context) -> GameResult<()> {
    // Clear everything
    let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

    // Render the world
    render_world(ctx, &mut canvas, &app.game_state.world, &mut app.resources);

    // Render a message if there is one set
    render_message(ctx, &mut canvas, app)?;

    // Render the score
    let fragment =
        TextFragment::new(format!("Score: {}", app.game_state.score)).font(&app.resources.font);
    let text = graphics::Text::new(fragment);
    let pt = point2(Point::new(8.0, 4.0));
    canvas.draw(&text, DrawParam::new().dest(pt).color(color::SCORE));

    // Render the gun's heat status in the bottom right of the screen
    let gun = &app.game_state.world.player.gun;
    let color = if !gun.is_available() {
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
    let rect = Rect {
        x,
        y,
        w: GUN_HEAT_STATUS_WIDTH * gun.temperature,
        h: GUN_HEAT_STATUS_HEIGHT,
    };
    let r1 = Mesh::new_rectangle(ctx, DrawMode::Fill(FillOptions::default()), rect, color)?;
    let r2 = Mesh::new_rectangle(
        ctx,
        DrawMode::Stroke(StrokeOptions::default().with_line_width(1.0)),
        Rect {
            x,
            y,
            w: GUN_HEAT_STATUS_WIDTH,
            h: GUN_HEAT_STATUS_HEIGHT,
        },
        color,
    )?;

    canvas.draw(&r1, DrawParam::new());
    canvas.draw(&r2, DrawParam::new());

    // NOTE: for limiting FPS rate, see https://github.com/ggez/ggez/issues/171
    // If you want to log the current FPS, uncomment the next line
    // println!("{}", ggez::timer::get_fps(ctx));

    canvas.finish(ctx)?;
    Ok(())
}

/// Renders the Message struct contained in the game's state to the middle of the screen
fn render_message(
    ctx: &mut Context,
    canvas: &mut Canvas,
    app: &mut ApplicationState,
) -> GameResult<()> {
    if let Some(ref message) = app.game_state.message {
        let Message { title, subtitle } = *message;
        let Size { width, height } = app.game_state.world.size;

        let w = width / 2.0;
        let h = height / 2.0;

        let mut draw_text = |text: &str, color: Color, is_title: bool| {
            let fragment = TextFragment::new(text).font(&app.resources.font);
            let drawable = graphics::Text::new(fragment);
            let width = w - (drawable.dimensions(ctx).unwrap().w as f32 / 2.0);
            let height = if is_title {
                h - drawable.dimensions(ctx).unwrap().w as f32
            } else {
                h
            };
            let point = point2(Point::new(width, height));
            canvas.draw(&drawable, DrawParam::new().dest(point).color(color));
        };

        draw_text(title, color::WHITE, true);
        draw_text(subtitle, color::GREY, false);
    }

    Ok(())
}

/// Renders the world and everything in it
pub fn render_world(
    ctx: &mut Context,
    canvas: &mut Canvas,
    world: &World,
    resources: &mut Resources,
) {
    render_stars(canvas, world, resources);
    render_particles(canvas, world, resources);
    render_bullets(canvas, world, resources);
    render_enemy(canvas, world, resources);

    // Finally draw the player as red
    if !world.player.is_dead {
        render_player(ctx, canvas, &world.player, resources);
    }

    // Draw powerups
    for powerup in &world.powerups {
        let image = match powerup.kind {
            PowerupKind::Shield => &resources.powerup_shield,
            PowerupKind::TimeSlow => &resources.powerup_time_slow,
            PowerupKind::TripleShot => &resources.powerup_triple_shot,
        };
        let scale = powerup.radius() / SPRITE_SIZE;
        let params = DrawParam::new()
            .dest(point2(powerup.position()))
            .scale(point2(Point::new(scale, scale)))
            .color(color::POWERUP);

        canvas.draw(image, params);
    }
}

/// Renders all the stars in the background
fn render_stars(canvas: &mut Canvas, world: &World, resources: &mut Resources) {
    resources.star_sprite.clear();
    // Iterate through the stars list and draw them with a rotation based on their index in the
    // list - this isn't a truly random rotation, but it works visually
    for (i, star) in world.stars.iter().enumerate() {
        let scale = star.size / SPRITE_SIZE;
        resources.star_sprite.push(
            DrawParam::new()
                .dest(point2(Point::new(star.x(), star.y())))
                .rotation((i as f32 / 100.0) * 2.0 * std::f32::consts::PI)
                .scale(point2(Point::new(scale, scale)))
                .color(color::STAR),
        );
    }
    canvas.draw(&resources.star_sprite, DrawParam::new())
}

/// Renders all the particles
pub fn render_particles(canvas: &mut Canvas, world: &World, resources: &mut Resources) {
    resources.circle_sprite.clear();
    for particle in &world.particles {
        let scale = 0.4 * particle.ttl;
        resources.circle_sprite.push(
            DrawParam::new()
                .dest(point2(Point::new(particle.x(), particle.y())))
                .offset(point2(Point::new(0.5, 0.5)))
                .scale(point2(Point::new(scale, scale)))
                .color(color::PARTICLE),
        );
    }
    canvas.draw(&resources.circle_sprite, DrawParam::new())
}

/// Renders a bullet
pub fn render_bullets(canvas: &mut Canvas, world: &World, resources: &mut Resources) {
    resources.circle_sprite.clear();
    for bullet in &world.bullets {
        let scale = bullet.radius() / SPRITE_SIZE;
        resources.circle_sprite.push(
            DrawParam::new()
                .dest(point2(bullet.position()))
                .offset(point2(Point::new(0.5, 0.5)))
                .scale(point2(Point::new(scale, scale)))
                .color(color::BULLET),
        );
    }
    canvas.draw(&resources.circle_sprite, DrawParam::new())
}

/// Renders an enemy
pub fn render_enemy(canvas: &mut Canvas, world: &World, resources: &mut Resources) {
    resources.circle_sprite.clear();
    for enemy in &world.enemies {
        let scale = enemy.radius() * 2.0 / SPRITE_SIZE;
        resources.circle_sprite.push(
            DrawParam::new()
                .dest(point2(enemy.position()))
                .offset(point2(Point::new(0.5, 0.5)))
                .scale(point2(Point::new(scale, scale)))
                .color(color::ENEMY),
        );
    }
    canvas.draw(
        &resources.circle_sprite,
        DrawParam {
            ..Default::default()
        },
    )
}

/// Renders the player
pub fn render_player(
    ctx: &mut Context,
    canvas: &mut Canvas,
    player: &Player,
    resources: &Resources,
) {
    // Render shield if one is active
    let pt = Point::new(player.x(), player.y());
    if let Some(powerup) = player.powerup {
        if powerup == PowerupKind::Shield {
            let scale = (player.radius() + 30.0) / SPRITE_SIZE;
            let params = DrawParam::new()
                .dest(point2(pt))
                .offset(point2(Point::new(0.5, 0.5)))
                .scale(point2(Point::new(scale, scale)))
                .color(color::SHIELD);
            canvas.draw(&resources.circle_image, params);
        }
    }

    // Render the player
    let p1 = point2(Point::new(PLAYER_POLYGON[0][0], PLAYER_POLYGON[0][1]));
    let p2 = point2(Point::new(PLAYER_POLYGON[1][0], PLAYER_POLYGON[1][1]));
    let p3 = point2(Point::new(PLAYER_POLYGON[2][0], PLAYER_POLYGON[2][1]));
    let mesh = Mesh::new_polygon(
        ctx,
        DrawMode::Fill(FillOptions::default()),
        &[p1, p2, p3],
        color::PLAYER,
    )
    .unwrap();
    let dir = player.direction();
    canvas.draw(&mesh, DrawParam::new().dest(point2(pt)).rotation(dir))
}

fn point2(p: Point) -> Point2<f32> {
    Point2 { x: p.x, y: p.y }
}
