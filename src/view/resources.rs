use ggez::{
    audio,
    Context,
    graphics::{
        Font,
        Image,
        spritebatch::SpriteBatch,
    }
};
use ggez::audio::SoundSource;

/// Additional resources needed for the game
pub struct Resources {
    pub(in crate::view) font: Font,

    // Images
    pub(in crate::view) powerup_shield: Image,
    pub(in crate::view) powerup_time_slow: Image,
    pub(in crate::view) powerup_triple_shot: Image,
    pub(in crate::view) circle_image: Image,
    pub(in crate::view) star_sprite: SpriteBatch,
    pub(in crate::view) circle_sprite: SpriteBatch,

    // Sounds
    pub(in crate::view) shot_sound: audio::Source,
    pub(in crate::view) powerup_sound: audio::Source,
    pub(in crate::view) game_start_sound: audio::Source,
    pub(in crate::view) enemy_spawn_sound: audio::Source,
    pub(in crate::view) enemy_destroyed_sound: audio::Source,
    pub(in crate::view) player_destroyed_sound: audio::Source,
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
            font: Font::new(ctx, "/FiraMono-Bold.ttf").unwrap(),

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
