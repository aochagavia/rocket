//! A 2D toy game written in Rust, using the ggez library.
#![deny(missing_docs)]
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate ggez;
extern crate itertools_num;
extern crate rand;

mod controllers;
mod resources;
mod view;
mod drawing;
mod game_state;
mod geometry;
mod models;
mod util;
mod priority_queue;

use controllers::{CollisionsController, InputController, TimeController};
use resources::Resources;
use game_state::GameState;
use geometry::Size;
use drawing::color;
use priority_queue::PriorityQueue;

use ggez::conf;
use ggez::graphics;
use ggez::event::{self, Mod, Keycode};
use ggez::{Context, ContextBuilder, GameResult};

/// This struct contains the application's state
pub struct ApplicationState {
    // Keep track of window focus to play/pause the game
    has_focus: bool,
    // Resources holds our loaded font
    resources: Resources,
    // Our game logic is controlled within the game_state
    game_state: GameState,
    // We control the game state with the passage of time
    time_controller: TimeController,
    // We handle input events with the input_controller
    input_controller: InputController,
    // A place to store scheduled events
    scheduled_events: PriorityQueue

}

impl ApplicationState {
    fn new(ctx: &mut Context, game_state: GameState) -> GameResult<ApplicationState> {
        let app_state = ApplicationState {
            has_focus: true,
            resources: Resources::new(ctx),
            game_state: game_state,
            time_controller: TimeController::new(),
            input_controller: InputController::new(),
            scheduled_events: PriorityQueue::new()
        };
        Ok(app_state)
    }
}

// We implement `ggez::event::EventHandler` trait on our application state - this is where we can
// listen for input and other events
impl event::EventHandler for ApplicationState {
    // This is called each time the game loop updates so we can update the game state
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Check if we have any events that are scheduled to run, and if so, run them now
        if let Some(when) = self.scheduled_events.peek() {
            let now = ggez::timer::get_time_since_start(ctx);
            if when <= now { self.scheduled_events.pop().unwrap().1(self); }
        }

        // Update game state, and check for collisions
        if self.has_focus {
            let duration = ggez::timer::get_delta(ctx);
            let dt = duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9;
            self.time_controller.update_seconds(dt, self.input_controller.actions(), &mut self.game_state);
            CollisionsController::handle_collisions(self, ctx);
        }

        Ok(())
    }

    // This is called when ggez wants us to draw our game
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        view::render_game(self, ctx)
    }

    // Listen for keyboard events
    fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, keymod: Mod, _repeat: bool) {
        // If we're displaying a message (waiting for user input) then hide it and reset the game
        if self.game_state.message.should_show {
            self.game_state.message.should_show = false;
            self.game_state.reset();
        }
        self.input_controller.key_press(keycode, keymod);
    }
    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, keymod: Mod, _repeat: bool) {
        self.input_controller.key_release(keycode, keymod);
    }

    // Listen for window focus to pause the game's execution
    fn focus_event(&mut self, _ctx: &mut Context, has_focus: bool) {
        self.has_focus = has_focus;
    }
}

fn main() {
    // Setup Rocket's Game State
    let game_size = Size::new(1024.0, 600.0);
    let game_state = GameState::new(game_size);

    // Create configuration for ggez using its ContextBuilder
    let cb = ContextBuilder::new("rocket", "ggez")
        .window_setup(conf::WindowSetup::default().title("Rocket!"))
        .window_mode(conf::WindowMode::default().dimensions(game_size.width as u32, game_size.height as u32));

    // Create the rendering context and set the background color to black
    let ctx = &mut cb.build().unwrap();
    graphics::set_background_color(ctx, color::BLACK);

    // Load the application state and start the event loop
    let state = &mut ApplicationState::new(ctx, game_state).unwrap();
    if let Err(err) = event::run(ctx, state) {
        println!("Error encountered: {}", err);
    } else {
        println!("Exited cleanly, thanks for playing Rocket!");
    }
}
