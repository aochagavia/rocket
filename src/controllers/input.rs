use ggez::event::{KeyCode, KeyMods};

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
    pub fn key_press(&mut self, keycode: KeyCode, _keymod: KeyMods) {
        self.handle_key(keycode, true);
    }

    /// Processes a key release
    pub fn key_release(&mut self, keycode: KeyCode, _keymod: KeyMods) {
        self.handle_key(keycode, false);
    }

    /// Handles a key press or release
    fn handle_key(&mut self, keycode: KeyCode, pressed: bool) {
        match keycode {
            KeyCode::Left => self.actions.rotate_left = pressed,
            KeyCode::Right => self.actions.rotate_right = pressed,
            KeyCode::Up => self.actions.boost = pressed,
            KeyCode::Space => self.actions.shoot = pressed,
            _ => (),
        }
    }
}
