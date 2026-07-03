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

    pub fn pokemon_display_name(&self, nickname: &str) -> String {
        match self.team.get(nickname) {
            Some(p) if !p.species.is_empty() && p.species != nickname => {
                format!("{nickname} ({})", p.species)
            }
            _ => nickname.to_string(),
        }
    }

    pub fn active_pokemon_display_name(&self) -> String {
        match &self.active_pokemon {
            Some(nick) => self.pokemon_display_name(nick),
            None => String::new(),
        }
    }

    pub fn get_active_pokemon_state(&self) -> Option<&PokemonState> {
        self.team.get(&self.active_pokemon_display_name())
    }

    pub fn get_active_pokemon_state_mut(&mut self) -> Option<&mut PokemonState> {
        self.team.get_mut(&self.active_pokemon_display_name())
    }

    /// Takes (and clears) the pending flinch chance from the active Pokémon,
    /// returning (flinch_chance, source_move, active_nickname) if one was set.
    pub fn take_pending_flinch(&mut self) -> Option<(u64, String, String)> {
        let active_nick = self.active_pokemon.as_ref()?.clone();
        let pokemon = self.team.get_mut(&active_nick)?;
        let (flinch_chance, source_move) = pokemon.pending_flinch_chance.take()?;
        Some((flinch_chance, source_move, active_nick))
    }

    /// Sets a pending flinch chance on the active Pokémon.
    pub fn set_active_pending_flinch(&mut self, flinch_chance: u64, source_move: String) {
        let Some(active_nick) = self.active_pokemon.clone() else { return };
        let Some(pokemon) = self.team.get_mut(&active_nick) else { return };
        pokemon.pending_flinch_chance = Some((flinch_chance, source_move));
    }
}
