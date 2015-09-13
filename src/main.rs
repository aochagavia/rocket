extern crate glutin_window;
extern crate graphics;
extern crate itertools;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

mod drawing;
mod game;
mod models;
mod traits;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{Events, EventLoop};
use piston::input::{Button, Event, Input, RenderEvent};
use piston::window::WindowSettings;

use drawing::Size;
use game::Game;

const OPEN_GL: OpenGL = OpenGL::V2_1;

// Returns a result containing a GlutinWindow or an error if the window
// settings are not supported
fn try_create_window(samples: u8) -> Result<GlutinWindow, String> {
    WindowSettings::new("Rocket", [1024, 600])
        .exit_on_esc(true)
        .opengl(OPEN_GL)
        .samples(samples)
        .build()
}

fn main() {
    // Create a window with a sampling of 8 or fall back to 0
    let window = try_create_window(8).or_else(|_| try_create_window(0)).unwrap();

    let mut gl = GlGraphics::new(OPEN_GL);

    // The game object
    let mut game = Game::new(Size::new(1024.0, 600.0));

    // Event handling
    for e in window.events().ups(60).max_fps(60) {
        match e {
            Event::Input(Input::Press(Button::Keyboard(key))) => {
                game.key_press(key);
            }

            Event::Input(Input::Release(Button::Keyboard(key))) => {
                game.key_release(key);
            }

            Event::Render(args) => {
                gl.draw(args.viewport(), |c, g| game.render(c, g));
            }

            Event::Update(args) => {
                game.update(args.dt);
            }

            _ => {}
        }
    }
}
