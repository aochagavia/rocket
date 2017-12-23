use ggez::{audio, Context};
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::graphics::{Font, Image};

/// Additional resources needed for the game
pub struct Resources {
    pub font: Font,

    // Images
    pub star_sprite: SpriteBatch,
    pub circle_sprite: SpriteBatch,

    // Sounds
    pub shot_sound: audio::Source,
    pub boost_sound: audio::Source,
    pub game_start_sound: audio::Source,
    pub enemy_spawn_sound: audio::Source,
    pub enemy_destroyed_sound: audio::Source,
    pub player_destroyed_sound: audio::Source,
}

impl Resources {
    /// Initialize and return the `Resources`
    pub fn new(ctx: &mut Context) -> Resources {
        let new_with_volume = |ctx: &mut Context, path: &str, volume: f32| {
            let mut sound = audio::Source::new(ctx, path).unwrap();
            sound.set_volume(volume);
            sound
        };
        
        Resources {
            font: Font::new(ctx, "/FiraMono-Bold.ttf", 14).unwrap(),

            star_sprite: SpriteBatch::new(Image::new(ctx, "/images/star.png").unwrap()),
            circle_sprite: SpriteBatch::new(Image::new(ctx, "/images/circle.png").unwrap()),

            shot_sound:             new_with_volume(ctx, "/audio/shot.ogg", 0.2),
            boost_sound:            new_with_volume(ctx, "/audio/boost.ogg", 0.2),
            game_start_sound:       new_with_volume(ctx, "/audio/game_start.ogg", 1.0),
            enemy_spawn_sound:      new_with_volume(ctx, "/audio/enemy_spawn.ogg", 0.4),
            enemy_destroyed_sound:  new_with_volume(ctx, "/audio/enemy_destroyed.ogg", 1.0),
            player_destroyed_sound: new_with_volume(ctx, "/audio/player_destroyed.ogg", 1.0),
        }
    }
}
