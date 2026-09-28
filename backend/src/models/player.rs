use std::collections::HashMap;

use crate::constants::luck_weights::STATUS_WEIGHT;
use super::{LuckCategory, LuckEvent, PokemonSet, PokemonState};

#[derive(Debug, Clone)]
pub struct PlayerState {
    pub name: String,
    pub active_pokemon: Option<String>,
    pub team: HashMap<String, PokemonState>,
    pub luck_events: Vec<LuckEvent>,
    pub total_luck_score: f64,
    pub avatar: String,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            name: "unknown".into(),
            avatar: "default".into(),
            active_pokemon: None,
            team: HashMap::new(),
            luck_events: Vec::new(),
            total_luck_score: 0.0,
        }
    }
}

impl PlayerState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_luck_event(&mut self, event: LuckEvent) {
        self.total_luck_score += event.score;
        self.luck_events.push(event);
    }

    pub fn add_luck_events(&mut self, events: Vec<LuckEvent>) {
        for event in &events {
            self.total_luck_score += event.score;
        }
        self.luck_events.extend(events);
    }
    

    pub fn recalculate_total(&mut self) {
        self.total_luck_score = self.luck_events.iter().map(|event| event.score).sum();
    }

    pub fn update_pokemon_paste(&mut self, species: &str, paste: &PokemonSet) {
        let Some(pokemon) = self.get_pokemon_mut(species) else {
            return;
        };

        pokemon.set.evs = paste.evs.clone();
        pokemon.set.ivs = paste.ivs.clone();
        pokemon.set.moves.extend(paste.moves.iter().cloned());
        pokemon.set.item = paste.item.clone();
        pokemon.set.ability = paste.ability.clone();
        pokemon.set.nature = paste.nature.clone();
        pokemon.set.has_pokepaste = true;
    }

    pub fn pokemon_display_name(&self, nickname: &str) -> String {
        match self.get_pokemon(nickname) {
            Some(p) if !p.identity.species.is_empty() && p.identity.species != nickname => {
                format!("{nickname} ({})", p.identity.species)
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

    pub fn get_pokemon(&self, nickname: &str) -> Option<&PokemonState> {
        self.team.get(nickname)
    }

    pub fn get_pokemon_mut(&mut self, nickname: &str) -> Option<&mut PokemonState> {
        self.team.get_mut(nickname)
    }

    pub fn get_active_pokemon_state(&self) -> Option<&PokemonState> {
        self.active_pokemon
            .as_deref()
            .and_then(|nickname| self.get_pokemon(nickname))
    }

    pub fn get_active_pokemon_state_mut(&mut self) -> Option<&mut PokemonState> {
        self.active_pokemon
            .as_ref()
            .and_then(|nickname| self.team.get_mut(nickname))
    }

    pub fn set_active_pokemon(&mut self, nickname: &str) {
        self.active_pokemon = Some(nickname.to_string());
    }

    pub fn clear_active_pokemon(&mut self) {
        self.active_pokemon = None;
    }

    pub fn is_active_pokemon(&self, nickname: &str) -> bool {
        self.active_pokemon.as_deref() == Some(nickname)
    }

    pub fn clear_pending_flinch(&mut self, nickname: &str) {
        if let Some(pokemon) = self.get_pokemon_mut(nickname) {
            pokemon.clear_pending_flinch();
        }
    }

    pub fn take_pending_flinch(&mut self) -> Option<(u64, String, String)> {
        let active_nick = self.active_pokemon.as_ref()?.clone();
        let pokemon = self.get_pokemon_mut(&active_nick)?;
        let (flinch_chance, source_move) = pokemon.pending.flinch_chance.take()?;
        Some((flinch_chance, source_move, active_nick))
    }

    pub fn set_active_pending_flinch(&mut self, flinch_chance: u64, source_move: String) {
        if let Some(pokemon) = self.get_active_pokemon_state_mut() {
            pokemon.pending.flinch_chance = Some((flinch_chance, source_move));
        }
    }

    pub fn resolve_sleep_luck(&mut self, nickname: &str, current_turn: u32, cause: &str) {
        let display = self.pokemon_display_name(nickname);
        let Some(turns) = self.get_pokemon_mut(nickname).and_then(|p| p.take_sleep_turns()) else {
            return;
        };
        if turns == 0 {
            return;
        }
    
        let capped = turns.min(3);
        let wake_up_luck = 1.0 / 3.0 * (2.0 - capped as f64);
        self.add_luck_event(LuckEvent {
            turn: current_turn,
            pokemon: display,
            category: LuckCategory::StatusTurn,
            score: STATUS_WEIGHT * wake_up_luck,
            description: format!("{cause} after {capped} sleep turn(s)"),
            source_move: None,
            is_beneficial: wake_up_luck > 0.0,
        });
    }
}
