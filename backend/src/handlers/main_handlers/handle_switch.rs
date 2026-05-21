use crate::schema::lines::main_lines::MainLine;
use crate::schema::state::{GameState, PokemonState};

pub fn handle_switch(state: &mut GameState, line: &MainLine) {
    let Some(player_state) = state.get_player_state_mut(&line.player) else {
        panic!("Invalid player: {}", line.player);
    };

    let nickname = &line.pokemon_nickname;
    let species = line.species.as_deref().unwrap_or("");

    player_state.active_pokemon = Some(nickname.clone());

    if nickname != species && !species.is_empty() {
        player_state.team.remove(species);
    }

    if let Some(pokemon) = player_state.team.get_mut(nickname) {
        pokemon.current_hp = line.pokemon_current_hp.unwrap_or(pokemon.current_hp);
        pokemon.max_hp = line.pokemon_max_hp.unwrap_or(pokemon.max_hp);
        if pokemon.species.is_empty() && !species.is_empty() {
            pokemon.species = species.to_string();
        }
    } else {
        player_state.team.insert(
            nickname.clone(),
            PokemonState::new(
                nickname.clone(),
                species.to_string(),
                line.pokemon_current_hp.unwrap_or(0),
                line.pokemon_max_hp.unwrap_or(0),
            ),
        );
    }
}
