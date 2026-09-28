use crate::constants::poke_set_data::get_species;
use crate::models::{GameState, PokemonState};

pub fn process_poke(state: &mut GameState, player_name: &str, poke_name: &str, gender: &str) {
    let Some(player_state) = state.get_player_state_mut(player_name) else {
        return;
    };

    let Some(species_data) = get_species(poke_name) else {
        return;
    };

    let mut pokemon = PokemonState::new(species_data);

    if !gender.is_empty() {
        pokemon.identity.gender = Some(gender.to_string());
    }

    player_state
        .team
        .insert(species_data.name.clone(), pokemon);
}
