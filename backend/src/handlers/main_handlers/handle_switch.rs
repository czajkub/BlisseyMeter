use crate::schema::lines::main_lines::MainLine;
use crate::schema::state::{GameState, PokemonState};

pub fn handle_switch(state: &mut GameState, line: &MainLine) {
    let Some(player_state) = state.get_player_state_mut(&line.player) else {
        panic!("Invalid player: {}", line.player);
    };

    player_state.active_pokemon = Some(line.pokemon_nickname.clone());

    if !player_state.team.contains_key(&line.pokemon_nickname) {
        player_state.team.insert(
            line.pokemon_nickname.clone(),
            PokemonState::new(
                line.pokemon_nickname.clone(),
                line.species.clone().unwrap_or_default(),
                line.pokemon_current_hp.unwrap_or(0),
                line.pokemon_max_hp.unwrap_or(0),
            ),
        );
    }
}
