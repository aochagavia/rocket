#![deny(missing_docs)]

//! A 2D toy game written in Rust, using the Piston library.

extern crate piston_window;
extern crate itertools_num;
extern crate opengl_graphics;
extern crate rand;

mod drawing;
mod game;
mod models;
mod traits;
mod util;

use piston_window::{Button, EventLoop, Input, Motion, OpenGL, PistonWindow, WindowSettings};
use opengl_graphics::GlGraphics;

use game::Game;

fn main() {
    let opengl = OpenGL::V3_2;

    let game_size: drawing::Size = drawing::Size::new(1024.0, 600.0);
    let mut game = Game::new(game_size);

    let mut window: PistonWindow = WindowSettings::new(
        "Rocket!", [game_size.width as u32, game_size.height as u32])
        .opengl(opengl).samples(8).exit_on_esc(true).build().unwrap();

    window.set_ups(60);
    window.set_max_fps(60);

    let mut gl = GlGraphics::new(opengl);

    while let Some(e) = window.next() {
        // Event handling
        match e {
            Input::Press(Button::Keyboard(key)) => {
                game.key_press(key);
            }

            Input::Release(Button::Keyboard(key)) => {
                game.key_release(key);
            }

            Input::Press(Button::Controller(button)) => {
                game.button_press(button);
            }

            Input::Release(Button::Controller(button)) => {
                game.button_release(button);
            }

            // Controller Axis are Move Input types
            Input::Move(Motion::ControllerAxis(axis)) => {
                game.handle_axis(axis);
            }

            Input::Render(args) => {
                gl.draw(args.viewport(), |c, g| game.render(c, g));
            }

            Input::Update(args) => {
                game.update(args.dt);
            }

            _ => {}
        }
    }
}
