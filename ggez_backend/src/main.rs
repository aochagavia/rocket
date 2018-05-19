extern crate ggez;
extern crate rand;
extern crate rocket;

mod view;

use ggez::{Context, GameResult};
use ggez::event::{self, Keycode, Mod};
use rand::ThreadRng;
use rocket::controllers::{CollisionsController, Event, InputController, TimeController};
use rocket::game_state::GameState;
use rocket::geometry::Size;

use view::Resources;

pub fn main() {
    // Create the rendering context and set the background color to black
    let game_size = Size::new(1024.0, 600.0);
    let ctx = &mut view::init_rendering_ctx(game_size).unwrap();

    // Load the application state and start the event loop
    let state = &mut ApplicationState::new(ctx, game_size).unwrap();
    if let Err(err) = event::run(ctx, state) {
        println!("Error encountered: {}", err);
    } else {
        println!("Exited cleanly, thanks for playing Rocket!");
    }
}

/// This struct contains the application's state
pub struct ApplicationState {
    // Keep track of window focus to play/pause the game
    has_focus: bool,
    // Resources holds our loaded font, images and sounds
    resources: Resources,
    // The game state contains all information needed to run the game
    game_state: GameState,
    // The time controller modifies the game state as time passes
    time_controller: TimeController,
    // The input controller keeps track of the actions that are triggered by the player
    input_controller: InputController,
    // The event buffer keeps track of events that trigger sounds, so we can separate
    // sound playing from the game logic
    event_buffer: Vec<Event>,
    // A source of randomness
    rng: ThreadRng,
}

impl ApplicationState {
    /// Simply creates a new application state
    fn new(ctx: &mut Context, game_size: Size) -> GameResult<ApplicationState> {
        let mut rng = rand::thread_rng();
        let app_state = ApplicationState {
            has_focus: true,
            resources: Resources::new(ctx),
            game_state: GameState::new(game_size, &mut rng),
            time_controller: TimeController::new(),
            input_controller: InputController::new(),
            event_buffer: Vec::new(),
            rng,
        };
        Ok(app_state)
    }

    /// This will be called when the game needs to be reset
    fn reset(&mut self) {
        // Reset time controller
        self.time_controller.reset();

        // Reset game state
        self.game_state.reset(&mut self.rng);

        self.event_buffer.push(Event::GameStart);
    }
}

// We implement `ggez::event::EventHandler` trait on our application state - this is where we can
// listen for input and other events
impl event::EventHandler for ApplicationState {
    // This is called each time the game loop updates so we can update the game state
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Pause the game if the window has no focus
        if !self.has_focus {
            return Ok(())
        }

        // Update game state, and check for collisions
        let duration = ggez::timer::get_delta(ctx);
        self.time_controller.update(
            duration,
            self.input_controller.actions(),
            &mut self.game_state,
            &mut self.event_buffer,
            &mut self.rng
        );

        CollisionsController::handle_collisions(&mut self.game_state, &mut self.time_controller, &mut self.event_buffer);

        Ok(())
    }

    // This is called when ggez wants us to draw our game
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        view::play_sounds(&mut self.event_buffer, &mut self.resources)?;
        view::render_game(&mut self.game_state, &mut self.resources, ctx)
    }

    // Listen for keyboard events
    fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, _: Mod, _: bool) {
        // If we're displaying a message (waiting for user input) then hide it and reset the game
        if let Some(_) = self.game_state.message {
            self.reset();
        }
        handle_key(&mut self.input_controller, keycode, true);
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _: Mod, _: bool) {
        handle_key(&mut self.input_controller, keycode, false);
    }

    // Listen for window focus to pause the game's execution
    fn focus_event(&mut self, _ctx: &mut Context, has_focus: bool) {
        self.has_focus = has_focus;
    }
}

/// Handle key press or release
fn handle_key(input: &mut InputController, keycode: Keycode, enabled: bool) {
    match keycode {
        Keycode::Left => input.actions.rotate_left = enabled,
        Keycode::Right => input.actions.rotate_right = enabled,
        Keycode::Up => input.actions.boost = enabled,
        Keycode::Space => input.actions.shoot = enabled,
        _ => ()
    }
}
