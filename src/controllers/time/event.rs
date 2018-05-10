use game_state::GameState;

#[derive(Copy, Clone)]
pub enum Event {
    RemovePowerup,
    ShowGameOverScreen
}

impl Event {
    pub fn handle(self, state: &mut GameState) {
        match self {
            Event::RemovePowerup => {
                state.world.player.powerup = None;
            }
            Event::ShowGameOverScreen => {
                state.game_over();
            }
        }
    }
}
