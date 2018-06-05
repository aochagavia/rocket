extern crate rand;
extern crate rocket;
extern crate pcg_rand;

use std::cell::RefCell;
use std::os::raw::{c_float, c_int};
use std::time::Duration;

use pcg_rand::Pcg32Basic;
use rand::SeedableRng;

use rocket::game_state::GameState;
use rocket::geometry::{Advance, Position, Size};
use rocket::controllers::{Event, InputController, TimeController, CollisionsController};

thread_local! {
    static DATA: RefCell<GameData> = RefCell::new(new_game_data(1024.0, 600.0));
}

struct GameData {
    /// The game state contains all information needed to run the game
    state: GameState,
    /// The input controller keeps track of active player actions
    input_controller: InputController,
    /// The time controller modifies the game state as time passes
    time_controller: TimeController,
    /// We keep track of events that require playing sounds
    events: Vec<Event>,
    /// A source of randomness
    rng: Pcg32Basic
}

fn new_game_data(width: f32, height: f32) -> GameData {
    let mut rng = Pcg32Basic::from_seed([44, 44]);
    GameData {
        state: GameState::new(Size::new(width, height), &mut rng),
        input_controller: InputController::default(),
        time_controller: TimeController::new(),
        events: Vec::new(),
        rng
    }
}

// These functions are provided by the runtime
extern "C" {
    fn clear_screen();
    fn draw_player(_: c_float, _: c_float, _: c_float);
    fn draw_enemy(_: c_float, _: c_float);
    fn draw_bullet(_: c_float, _: c_float);
    fn draw_particle(_: c_float, _: c_float, _: c_float);
    fn draw_score(_: c_float);

    // TODO: implement on the JS side
    fn draw_message(); // FIXME: use wasm-bindgen here to pass a &str?
    fn draw_gun_heat(); // FIXME: get the right parameters
    fn draw_shield_powerup(_: c_float, _: c_float, _: c_float);
    fn draw_time_slow_powerup(_: c_float, _: c_float, _: c_float);
    fn draw_triple_shot_powerup(_: c_float, _: c_float, _: c_float);
    fn draw_star(); // FIXME: will this work out with the amount of stars we have?
    fn play_enemy_destroyed_sound();
    fn play_player_destroyed_sound();
    fn play_powerup_sound();
    fn play_shot_sound();
    fn play_enemy_spawn_sound();
    fn play_game_start_sound();
}

#[no_mangle]
pub extern "C" fn resize(width: c_float, height: c_float) {
    DATA.with(|data| *data.borrow_mut() = new_game_data(width, height));
}

#[no_mangle]
pub unsafe extern "C" fn draw() {
    DATA.with(|data| {
        let data = data.borrow_mut();
        let world = &data.state.world;

        clear_screen();
        for particle in &world.particles {
            draw_particle(particle.x(), particle.y(), 5.0 * particle.ttl);
        }

        for bullet in &world.bullets {
            draw_bullet(bullet.x(), bullet.y());
        }

        for enemy in &world.enemies {
            draw_enemy(enemy.x(), enemy.y());
        }

        if !world.player.is_dead {
            draw_player(world.player.x(), world.player.y(), world.player.direction());
        }
        draw_score(data.state.score as f32);
    });
}

#[no_mangle]
pub extern "C" fn play_sounds() {
    DATA.with(|data| {
        let mut data = data.borrow_mut();
        for event in data.events.drain(..) {
            use rocket::controllers::Event::*;
            match event {
                EnemyDestroyed => unsafe { play_enemy_destroyed_sound() },
                PlayerDestroyed => unsafe { play_player_destroyed_sound() },
                PowerupGained => unsafe { play_powerup_sound() },
                ShotFired => unsafe { play_shot_sound() },
                EnemySpawned => unsafe { play_enemy_spawn_sound() },
                GameStart => unsafe { play_game_start_sound() }
            }
        }
    });
}

#[no_mangle]
pub extern "C" fn update(time: c_float) {
    DATA.with(|data| {
        let data: &mut GameData = &mut data.borrow_mut();
        data.time_controller.update(
            Duration::from_millis(time as u64),
            &data.input_controller,
            &mut data.state,
            &mut data.events,
            &mut data.rng);
        CollisionsController::handle_collisions(&mut data.state, &mut data.time_controller, &mut data.events);
    });
}

fn int_to_bool(i: c_int) -> bool {
    i != 0
}

#[no_mangle]
pub extern "C" fn key_pressed() {
    // This one...
    DATA.with(|data| {
        let data: &mut GameData = &mut data.borrow_mut();
        if let Some(_) = data.state.message {
            data.state.reset(&mut data.rng);
        }
    });
}

#[no_mangle]
pub extern "C" fn toggle_shoot(b: c_int) {
    DATA.with(|data| data.borrow_mut().input_controller.shoot = int_to_bool(b));
}

#[no_mangle]
pub extern "C" fn toggle_boost(b: c_int) {
    DATA.with(|data| data.borrow_mut().input_controller.boost = int_to_bool(b));
}

#[no_mangle]
pub extern "C" fn toggle_turn_left(b: c_int) {
    DATA.with(|data| data.borrow_mut().input_controller.rotate_left = int_to_bool(b));
}

#[no_mangle]
pub extern "C" fn toggle_turn_right(b: c_int) {
    DATA.with(|data| data.borrow_mut().input_controller.rotate_right = int_to_bool(b));
}
