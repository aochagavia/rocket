extern crate piston_window;
extern crate graphics;
extern crate itertools;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

mod drawing;
mod game;
mod models;
mod traits;

use piston_window::*;
use opengl_graphics::GlGraphics;


use game::Game;

// Returns a result containing a GlutinWindow or an error if the window
// settings are not supported
fn main() {

    let opengl = OpenGL::V3_2;

    let mut game = Game::new(drawing::Size::new(1024.0, 600.0));

    let mut window: PistonWindow = WindowSettings::new("Rocket!", [1024, 600])
        .opengl(opengl).exit_on_esc(true).build().unwrap();

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
