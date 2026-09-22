use crate::schema::lines::{Hp, PokemonRef};
use crate::schema::state::{GameState};

pub fn process_switch(state: &mut GameState, source_pokemon: &PokemonRef, species: &str, hp: &Hp) -> Result<(), String> {
    let Some(player_state) = state.get_player_state_mut(source_pokemon.player.as_str()) else {
        return Err(format!("Couldn't reach player {} state", source_pokemon.player.as_str()));
    };

    let nickname = &source_pokemon.pokemon_nickname;

    let key = if player_state.team.contains_key(species) {
        species.to_string()
    } else if player_state.team.contains_key(nickname) {
        nickname.to_string()
    } else {
        return Err("Unknown pokemon switched in".to_string());
    };

    let mut pokemon = player_state
        .team
        .remove(&key)
        .expect("key checked with contains_key just above");

    pokemon.identity.nickname = nickname.clone();
    if !species.is_empty() {
        pokemon.identity.species = species.to_string();
    }
    pokemon.condition.current_hp = hp.current;
    pokemon.condition.max_hp = hp.max;

    player_state.team.insert(nickname.clone(), pokemon);
    player_state.active_pokemon = Some(nickname.clone());

    Ok(())
}
