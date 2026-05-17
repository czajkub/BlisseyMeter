use crate::schema::lines::info_lines::InfoLine;
use crate::schema::state::{GameState, PokemonState};

pub fn handle_poke(state: &mut GameState, line: &InfoLine) {
    let Some(ref player_name) = line.player else { return; };
    let Some(ref poke_name) = line.poke else { return; };

    let Some(player_state) = state.get_player_state_mut(player_name) else {
        panic!("Invalid player: {player_name}");
    };

    player_state.team.insert(
        poke_name.clone(),
        PokemonState::new(
            poke_name.clone(),
            line.gender.clone().unwrap_or_default(),
            0,
            100,
        ),
    );
}
