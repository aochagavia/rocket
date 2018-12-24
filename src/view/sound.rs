use ggez::GameResult;

use crate::{
    controllers::Event,
    view::Resources,
};

pub fn play_sounds(events: &mut Vec<Event>, resources: &Resources) -> GameResult<()> {
    use self::Event::*;
    for event in events.drain(..) {
        match event {
            EnemyDestroyed => resources.enemy_destroyed_sound.play()?,
            PlayerDestroyed => resources.player_destroyed_sound.play()?,
            PowerupGained => resources.powerup_sound.play()?,
            ShotFired => resources.shot_sound.play()?,
            EnemySpawned => resources.enemy_spawn_sound.play()?,
            GameStart => resources.game_start_sound.play()?
        }
    }

    Ok(())
}
