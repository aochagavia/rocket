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
use rocket::controllers::{Actions, Event, TimeController, CollisionsController};

thread_local! {
    static DATA: RefCell<GameData> = RefCell::new(new_game_data(1024.0, 600.0));
}

struct GameData {
    state: GameState,
    actions: Actions,
    time_controller: TimeController,
    events: Vec<Event>,
    rng: Pcg32Basic
}

fn new_game_data(width: f32, height: f32) -> GameData {
    let mut rng = Pcg32Basic::from_seed([44, 44]);
    GameData {
        state: GameState::new(Size::new(width, height), &mut rng),
        actions: Actions::default(),
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
pub extern "C" fn update(time: c_float) {
    DATA.with(|data| {
        let data: &mut GameData = &mut data.borrow_mut();
        data.time_controller.update(
            Duration::from_millis(time as u64),
            &data.actions,
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
    DATA.with(|data| {
        let data: &mut GameData = &mut data.borrow_mut();
        if let Some(_) = data.state.message {
            data.state.reset(&mut data.rng);
        }
    });
}

#[no_mangle]
pub extern "C" fn toggle_shoot(b: c_int) {
    DATA.with(|data| data.borrow_mut().actions.shoot = int_to_bool(b));
}

#[no_mangle]
pub extern "C" fn toggle_boost(b: c_int) {
    DATA.with(|data| data.borrow_mut().actions.boost = int_to_bool(b));
}

#[no_mangle]
pub extern "C" fn toggle_turn_left(b: c_int) {
    DATA.with(|data| data.borrow_mut().actions.rotate_left = int_to_bool(b));
}

#[no_mangle]
pub extern "C" fn toggle_turn_right(b: c_int) {
    DATA.with(|data| data.borrow_mut().actions.rotate_right = int_to_bool(b));
}
