//! A 2D toy game written in Rust, using the ggez library.
#![deny(missing_docs)]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate ggez;
extern crate itertools_num;
extern crate rand;

// Note: we need to load `geometry` first so the macro is available for
// the modules that come afterwards
#[macro_use]
mod geometry;
mod controllers;
mod resources;
mod view;
mod drawing;
mod game_state;
mod models;
mod util;

use controllers::{CollisionsController, InputController, TimeController};
use resources::Resources;
use game_state::GameState;
use geometry::Size;
use drawing::color;

use ggez::conf;
use ggez::graphics;
use ggez::event::{self, Keycode, Mod};
use ggez::{Context, ContextBuilder, GameResult};

/// This struct contains the application's state
pub struct ApplicationState {
    // Keep track of window focus to play/pause the game
    has_focus: bool,
    // Resources holds our loaded font, images and sounds
    resources: Resources,
    // Our game logic is controlled within the game_state
    game_state: GameState,
    // We control the game state with the passage of time
    time_controller: TimeController,
    // We handle input events with the input_controller
    input_controller: InputController,
}

impl ApplicationState {
    /// Simply creates a new application state
    fn new(ctx: &mut Context, game_state: GameState) -> GameResult<ApplicationState> {
        let app_state = ApplicationState {
            has_focus: true,
            resources: Resources::new(ctx),
            game_state: game_state,
            time_controller: TimeController::new(),
            input_controller: InputController::new(),
        };
        Ok(app_state)
    }

    /// This will be called when the game needs to be reset
    fn reset(&mut self) {
        // Reset time controller
        self.time_controller.reset();

        // Reset game state
        self.game_state.reset();

        // Play game start sound
        let _ = self.resources.game_start_sound.play();
    }
}

// We implement `ggez::event::EventHandler` trait on our application state - this is where we can
// listen for input and other events
impl event::EventHandler for ApplicationState {
    // This is called each time the game loop updates so we can update the game state
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Update game state, and check for collisions
        if self.has_focus {
            let duration = ggez::timer::get_delta(ctx);
            self.time_controller.update_seconds(
                duration,
                self.input_controller.actions(),
                &mut self.game_state,
                &self.resources,
            );
            CollisionsController::handle_collisions(self);
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
        if let Some(_) = self.game_state.message {
            self.reset();
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
        .window_mode(
            conf::WindowMode::default().dimensions(game_size.width as u32, game_size.height as u32),
        );

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
