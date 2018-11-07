use game_state::GameState;

#[derive(Copy, Clone)]
pub enum Timeout {
    RemovePowerup,
    ShowGameOverScreen,
}

impl Timeout {
    pub fn handle(self, state: &mut GameState) {
        match self {
            Timeout::RemovePowerup => {
                state.world.player.powerup = None;
            }
            Timeout::ShowGameOverScreen => {
                state.game_over();
            }
        }
    }
}
