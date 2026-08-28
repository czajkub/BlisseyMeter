use crate::schema::state::{GameState, PokemonState};

pub fn handle_poke(state: &mut GameState, player_name: &str, poke_name: &str) {
    let Some(player_state) = state.get_player_state_mut(player_name) else {
        return;
    };

    player_state.team.insert(
        poke_name.to_string(),
        PokemonState::new(
            poke_name.to_string(), // at this point nickname is unknown, use species
            poke_name.to_string(), // species
            0,
            100,
        ),
    );
}
