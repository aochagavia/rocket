use ggez::graphics::{self, DrawMode, Point2};
use ggez::{Context, GameResult};

use ApplicationState;
use drawing::color;
use geometry::{Advance, Collide, Position};
use models::{Bullet, Enemy, Particle, Player, World, PLAYER_POLYGON};

/// Renders the game to the screen
pub fn render_game(app: &mut ApplicationState, ctx: &mut Context) -> GameResult<()> {
    // Clear everything
    graphics::clear(ctx);

    // Render the world
    render_world(&app.game_state.world, ctx)?;

    // Render the score
    let text = graphics::Text::new(ctx, &format!("Score: {}", app.game_state.score), &app.resources.font)?;
    let pt = Point2::new(8.0, 4.0);
    graphics::set_color(ctx, color::ORANGE)?;
    graphics::draw(ctx, &text, pt, 0.0)?;

    // NOTE: for limiting FPS rate, see https://github.com/ggez/ggez/issues/171
    // If you want to log the current FPS, uncomment the next line
    // println!("{}", ggez::timer::get_fps(ctx));

    graphics::present(ctx);
    Ok(())
}

/// Renders the world and everything in it
pub fn render_world(world: &World, ctx: &mut Context) -> GameResult<()> {
    // Draws particles in violet
    graphics::set_color(ctx, color::VIOLET)?;
    for particle in &world.particles {
        render_particle(particle, ctx)?;
    }
    
    // Draw any bullets as blue
    graphics::set_color(ctx, color::BLUE)?;
    for bullet in &world.bullets {
        render_bullet(bullet, ctx)?;
    }

    // Now we draw the enemies as yellow
    graphics::set_color(ctx, color::YELLOW)?;
    for enemy in &world.enemies {
        render_enemy(enemy, ctx)?;
    }

    // Finally draw the player as red
    graphics::set_color(ctx, color::RED)?;
    render_player(&world.player, ctx)?;

    Ok(())
}

/// Renders a particle
pub fn render_particle(particle: &Particle, ctx: &mut Context) -> GameResult<()> {
    let radius = 5.0 * particle.ttl as f32;
    let pt = Point2::new(particle.x() as f32, particle.y() as f32);
    graphics::circle(ctx, DrawMode::Fill, pt, radius, 2.0)
}

/// Renders a bullet
pub fn render_bullet(bullet: &Bullet, ctx: &mut Context) -> GameResult<()> {
    let pt = Point2::new(bullet.x() as f32, bullet.y() as f32);
    graphics::circle(ctx, DrawMode::Fill, pt, bullet.radius() as f32, 2.0)
}

/// Renders an enemy
pub fn render_enemy(enemy: &Enemy, ctx: &mut Context) -> GameResult<()> {
    let pt = Point2::new(enemy.x() as f32, enemy.y() as f32);
    graphics::circle(ctx, DrawMode::Fill, pt, enemy.radius() as f32, 0.5)
}

/// Render the player
pub fn render_player(player: &Player, ctx: &mut Context) -> GameResult<()> {
    let p1 = Point2::new(PLAYER_POLYGON[0][0] as f32, PLAYER_POLYGON[0][1] as f32);
    let p2 = Point2::new(PLAYER_POLYGON[1][0] as f32, PLAYER_POLYGON[1][1] as f32);
    let p3 = Point2::new(PLAYER_POLYGON[2][0] as f32, PLAYER_POLYGON[2][1] as f32);

    let mesh = graphics::Mesh::new_polygon(ctx, DrawMode::Fill, &[p1, p2, p3])?;
    let pt = Point2::new(player.x() as f32, player.y() as f32);
    let dir = player.direction() as f32;
    graphics::draw(ctx, &mesh, pt, dir)
}
