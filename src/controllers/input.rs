use ggez::{input::keyboard::KeyInput, winit::event::VirtualKeyCode};

#[derive(Default)]
pub struct InputController {
    actions: Actions,
}

/// Active actions (toggled by user input)
#[derive(Default)]
pub struct Actions {
    pub rotate_left: bool,
    pub rotate_right: bool,
    pub boost: bool,
    pub shoot: bool,
}

impl InputController {
    /// Create a new `InputController`
    pub fn new() -> InputController {
        InputController::default()
    }

    /// Returns a shared reference to the underlying actions
    pub fn actions(&self) -> &Actions {
        &self.actions
    }

    /// Processes a key press
    pub fn key_press(&mut self, input: KeyInput) {
        self.handle_key(input, true);
    }

    /// Processes a key release
    pub fn key_release(&mut self, input: KeyInput) {
        self.handle_key(input, false);
    }

    /// Handles a key press or release
    fn handle_key(&mut self, input: KeyInput, pressed: bool) {
        match input.keycode.unwrap() {
            VirtualKeyCode::Left => self.actions.rotate_left = pressed,
            VirtualKeyCode::Right => self.actions.rotate_right = pressed,
            VirtualKeyCode::Up => self.actions.boost = pressed,
            VirtualKeyCode::Space => self.actions.shoot = pressed,
            _ => (),
        }
    }
}
