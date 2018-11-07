use ggez::graphics::spritebatch::SpriteBatch;
use ggez::graphics::Image;
use ggez::{audio, Context};

/// Additional resources needed for the game
pub struct Resources {
    // Images
    pub(in view) powerup_shield: Image,
    pub(in view) powerup_time_slow: Image,
    pub(in view) powerup_triple_shot: Image,
    pub(in view) circle_image: Image,
    pub(in view) star_sprite: SpriteBatch,
    pub(in view) circle_sprite: SpriteBatch,

    // Sounds
    pub(in view) shot_sound: audio::Source,
    pub(in view) powerup_sound: audio::Source,
    pub(in view) game_start_sound: audio::Source,
    pub(in view) enemy_spawn_sound: audio::Source,
    pub(in view) enemy_destroyed_sound: audio::Source,
    pub(in view) player_destroyed_sound: audio::Source,
}

impl Resources {
    /// Initialize and return the `Resources`
    pub fn new(ctx: &mut Context) -> Resources {
        let new_with_volume = |ctx: &mut Context, path: &str, volume: f32| {
            let mut sound = audio::Source::new(ctx, path).unwrap();
            sound.set_volume(volume);
            sound
        };

        let circle_image = Image::new(ctx, "/images/circle.png").unwrap();
        Resources {
            powerup_shield: Image::new(ctx, "/images/powerup_shield.png").unwrap(),
            powerup_time_slow: Image::new(ctx, "/images/powerup_time_slow.png").unwrap(),
            powerup_triple_shot: Image::new(ctx, "/images/powerup_triple_shot.png").unwrap(),

            circle_image: circle_image.clone(),
            star_sprite: SpriteBatch::new(Image::new(ctx, "/images/star.png").unwrap()),
            circle_sprite: SpriteBatch::new(circle_image),

            shot_sound: new_with_volume(ctx, "/audio/shot.ogg", 0.2),
            powerup_sound: new_with_volume(ctx, "/audio/powerup.ogg", 1.0),
            game_start_sound: new_with_volume(ctx, "/audio/game_start.ogg", 1.0),
            enemy_spawn_sound: new_with_volume(ctx, "/audio/enemy_spawn.ogg", 0.4),
            enemy_destroyed_sound: new_with_volume(ctx, "/audio/enemy_destroyed.ogg", 1.0),
            player_destroyed_sound: new_with_volume(ctx, "/audio/player_destroyed.ogg", 1.0),
        }
    }
}
