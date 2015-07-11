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
use piston::event::{Event, Events, RenderEvent};
use piston::input::{Button, Input};
use piston::window::WindowSettings;

use drawing::Size;
use game::Game;

fn main() {
    // Initialization stuff
    let opengl = OpenGL::_3_2;

    let window = GlutinWindow::new(
        opengl,
        WindowSettings::new("Rocket", [1024, 600]).exit_on_esc(true)
    );

    let mut gl = GlGraphics::new(opengl);

    // The game object
    let mut game = Game::new(Size::new(1024.0, 600.0));

    // Event handling
    for e in window.events() {
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
