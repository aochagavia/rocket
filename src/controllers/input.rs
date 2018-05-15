#[derive(Default)]
pub struct InputController {
    actions: Actions,
}

pub enum Action {
    RotateLeft,
    RotateRight,
    Boost,
    Shoot
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

    /// Enable a player action
    pub fn start_action(&mut self, action: Action) {
        self.handle_action(action, true);
    }

    /// Disable a player action
    pub fn stop_action(&mut self, action: Action) {
        self.handle_action(action, false);
    }

    /// Enable or disable a player action
    fn handle_action(&mut self, action: Action, enabled: bool) {
        match action {
            Action::RotateLeft => self.actions.rotate_left = enabled,
            Action::RotateRight => self.actions.rotate_right = enabled,
            Action::Boost => self.actions.boost = enabled,
            Action::Shoot => self.actions.shoot = enabled,
        }
    }
}
