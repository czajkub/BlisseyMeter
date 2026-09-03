use std::collections::HashMap;
use std::sync::LazyLock;
use std::cmp::max;

use crate::schema::lines::PokemonRef;
use crate::schema::state::{GameState, Weather};

#[derive(Debug, serde::Deserialize, Clone)]
pub struct Move {
    pub name: String,
    #[serde(rename = "type")]
    pub move_type: String,
    pub category: String,
    pub power: Option<String>,
    pub accuracy: Option<String>,
    pub pp: Option<u64>,
    pub effect: String,
    #[serde(rename = "secondaryEffect")]
    pub secondary_effect: Option<u64>,
}

fn resolve_boosts(
    initial_accuracy: &f64,
    state: &GameState,
    source: &PokemonRef,
    target: &PokemonRef, 
) -> f64 {
    let Some(source_pokemon) = state.get_pokemon(source) else { return *initial_accuracy; };
    let Some(target_pokemon) = state.get_pokemon(target) else { return *initial_accuracy; };

    let source_accuracy = source_pokemon.stat_boosts.acc;
    let target_evasion = target_pokemon.stat_boosts.eva;
    
    let stage_multiplier = (source_accuracy - target_evasion).clamp(-6, 6);

    let accuracy_mult = (3.0 + max(0, stage_multiplier) as f64) / (3.0 + max(0, -stage_multiplier) as f64);

    *initial_accuracy * accuracy_mult
    
}

impl Move {
    fn get_accuracy(&self) -> f64 {
        match &self.accuracy {
            Some(acc) if acc == "∞" || acc == "999" => 999.0,
            Some(acc) => acc.parse::<f64>().unwrap_or(100.0),
            None => 100.0,
        }
    }

    // this is gen 5 onwards calculations
    pub fn get_accuracy_with_modifiers(
        &self,
        state: &GameState,
        source: &PokemonRef,
        target: &PokemonRef,
    ) -> f64 {
        let initial_accuracy = self.get_accuracy();
        
        let current_weather = state.field.weather.clone().unwrap_or_default();
        let after_weather_accuracy = self.apply_weather_override(&initial_accuracy, &current_weather);
        
        let accuracy_after_boosts = resolve_boosts(
            &after_weather_accuracy,
            &state,
            source,
            target
        );

        accuracy_after_boosts
        
    }

    fn apply_weather_override(
        &self,
        current_accuracy: &f64,
        current_weather: &Weather,
    ) -> f64 {
        if ! self.is_weather_reliant() {
            return *current_accuracy
        }
        
        match self.name.as_str() {
            "Bleakwind Storm" | "Windbolt Storm" | "Sandsear Storm" => {
                return if *current_weather == Weather::Rain { 999.0 } else { *current_accuracy }
            }
            "Blizzard" => {
                return if [Weather::Hail, Weather::Snow].contains(current_weather) { 999.0 } else { *current_accuracy }
            }
            "Thunder" | "Hurricane" => {
                match current_weather {
                    Weather::Rain => { return 999.0 }
                    Weather::Sun =>  { return 50.0 }
                    _ => { return *current_accuracy }
                }
            }
            // this should never be reached but compiler stuff
            _ => { return *current_accuracy }
        }
        
    }
    
    fn is_weather_reliant(&self) -> bool {
        if ["Blizzard", "Thunder", "Hurricane", "Bleakwind Storm", "Windbolt Storm", "Sandsear Storm"].contains(&self.name.as_str()) {
            return true
        } else {
            return false
        }
    }
}

/// Statics and constants/move_details.csv are baked into the binary via include_str!
/// for reliability in serverless (Lambda) and cross-platform environments.
pub static MOVES: LazyLock<HashMap<String, Move>> = LazyLock::new(|| {
    let csv_data = include_str!("move_details.csv");
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .quoting(true)
        .from_reader(csv_data.as_bytes());
    
    let mut moves = HashMap::new();
    for result in rdr.deserialize() {
        let move_record: Move = result.expect("Failed to parse move_details.csv");
        moves.insert(move_record.name.clone(), move_record);
    }
    moves
});

pub fn moves() -> &'static HashMap<String, Move> {
    &MOVES
}
