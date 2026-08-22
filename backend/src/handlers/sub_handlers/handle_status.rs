use crate::schema::lines::SubLine;
use crate::schema::state::GameState;

pub fn handle_status(state: &mut GameState, line: &SubLine) {
    let SubLine::Status { target, status, .. } = line else {
        return;
    };
    let Some(player_state) = state.get_player_state_mut(target.player.as_str()) else {
        return;
    };
    let Some(pokemon) = player_state.team.get_mut(&target.pokemon_nickname) else {
        return;
    };

    pokemon.status = status.clone();
    pokemon.status_turns = 0;
}
