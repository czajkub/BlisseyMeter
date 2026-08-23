use crate::schema::lines::InfoLine;
use crate::schema::state::{GameState, PokemonState};

pub fn handle_poke(state: &mut GameState, line: &InfoLine) {
    let InfoLine::Poke {
        player: player_name,
        species: poke_name,
        ..
    } = line
    else {
        return;
    };
    let Some(player_state) = state.get_player_state_mut(player_name) else {
        return;
    };

    player_state.team.insert(
        poke_name.clone(),
        PokemonState::new(
            poke_name.clone(), // at this point nickname is unknown, use species
            poke_name.clone(), // species
            0,
            100,
        ),
    );
}
