use super::{FieldState, PlayerState};

#[derive(Debug, Clone, Default)]
pub struct GameState {
    pub turn: u32,
    pub p1: PlayerState,
    pub p2: PlayerState,
    pub field: FieldState,
}

impl GameState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_player_state(&self, player: &str) -> Option<&PlayerState> {
        if player.starts_with("p1") {
            Some(&self.p1)
        } else if player.starts_with("p2") {
            Some(&self.p2)
        } else {
            None
        }
    }

    pub fn get_player_state_mut(&mut self, player: &str) -> Option<&mut PlayerState> {
        if player.starts_with("p1") {
            Some(&mut self.p1)
        } else if player.starts_with("p2") {
            Some(&mut self.p2)
        } else {
            None
        }
    }
}
