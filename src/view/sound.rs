use ggez::audio::SoundSource;
use ggez::{Context, GameResult};

use crate::{
    controllers::Event,
    view::Resources,
};

pub fn play_sounds(ctx: &Context, events: &mut Vec<Event>, resources: &mut Resources) -> GameResult<()> {
    use self::Event::*;
    for event in events.drain(..) {
        match event {
            EnemyDestroyed => resources.enemy_destroyed_sound.play(ctx)?,
            PlayerDestroyed => resources.player_destroyed_sound.play(ctx)?,
            PowerupGained => resources.powerup_sound.play(ctx)?,
            ShotFired => resources.shot_sound.play(ctx)?,
            EnemySpawned => resources.enemy_spawn_sound.play(ctx)?,
            GameStart => resources.game_start_sound.play(ctx)?
        }
    }

    Ok(())
}
