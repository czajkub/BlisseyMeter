use std::collections::HashMap;

use super::{LuckEvent, PokemonState};

#[derive(Debug, Clone, Default)]
pub struct PlayerState {
    pub name: String,
    pub active_pokemon: Option<String>,
    pub team: HashMap<String, PokemonState>,
    pub luck_events: Vec<LuckEvent>,
    pub total_luck_score: f64,
}

impl PlayerState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_luck_event(&mut self, event: LuckEvent) {
        self.total_luck_score += event.score;
        self.luck_events.push(event);
    }

    pub fn recalculate_total(&mut self) {
        self.total_luck_score = self.luck_events.iter().map(|event| event.score).sum();
    }
}
