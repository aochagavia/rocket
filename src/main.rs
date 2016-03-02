extern crate piston_window;
extern crate graphics;
extern crate itertools;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;
extern crate sdl2_window;

mod drawing;
mod game;
mod models;
mod traits;

use piston_window::*;
use opengl_graphics::GlGraphics;
use sdl2_window::Sdl2Window;


use game::Game;

// Use this typedef to make type of window prettier.
// Need to use Sdl2Window as backend in order to get joystick events currently.
pub type SDL2GameWindow = PistonWindow<(), Sdl2Window>;

// Returns a result containing a GlutinWindow or an error if the window
// settings are not supported
fn main() {

    let opengl = OpenGL::V3_2;

    let mut game = Game::new(drawing::Size::new(1024.0, 600.0));

    let mut window: SDL2GameWindow = WindowSettings::new("Rocket!", [1024, 600])
        .opengl(opengl).samples(8).exit_on_esc(true).build().unwrap();

    window.set_ups(60);
    window.set_max_fps(60);

    let mut gl = GlGraphics::new(opengl);

    let mut events = window.events();

    while let Some(e) = events.next(&mut window) {
    // Event handling
        match e {
            Event::Input(Input::Press(Button::Keyboard(key))) => {
                game.key_press(key);
            }

            Event::Input(Input::Release(Button::Keyboard(key))) => {
                game.key_release(key);
            }

            Event::Input(Input::Press(Button::Joystick(button))) => {
                game.button_press(button);
            }

            Event::Input(Input::Release(Button::Joystick(button))) => {
                game.button_release(button);
            }

            // Joystick Axis are Move Input types
            Event::Input(Input::Move(Motion::JoystickAxis(axis))) => {
                game.handle_axis(axis);
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
