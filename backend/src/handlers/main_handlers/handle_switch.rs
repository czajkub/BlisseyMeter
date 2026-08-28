use crate::schema::lines::{Hp, PokemonRef};
use crate::schema::state::{GameState, PokemonState};

pub fn handle_switch(state: &mut GameState, source_pokemon: &PokemonRef, species: &str, hp: &Hp) {
    let Some(player_state) = state.get_player_state_mut(source_pokemon.player.as_str()) else {
        return;
    };

    let nickname = &source_pokemon.pokemon_nickname;

    player_state.active_pokemon = Some(nickname.clone());

    if nickname != species && !species.is_empty() {
        player_state.team.remove(species);
    }

    if let Some(pokemon) = player_state.team.get_mut(nickname) {
        pokemon.current_hp = hp.current;
        pokemon.max_hp = hp.max;
        if pokemon.species.is_empty() && !species.is_empty() {
            pokemon.species = species.to_string();
        }
    } else {
        player_state.team.insert(
            nickname.clone(),
            PokemonState::new(nickname.clone(), species.to_string(), hp.current, hp.max),
        );
    }
}
