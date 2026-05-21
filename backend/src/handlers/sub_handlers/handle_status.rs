use crate::schema::lines::sub_lines::SubLine;
use crate::schema::state::GameState;

pub fn handle_status(state: &mut GameState, line: &SubLine) {
    let Some(player) = line.player.as_deref() else { return };
    let Some(nickname) = line.pokemon_nickname.as_deref() else { return };
    let Some(player_state) = state.get_player_state_mut(player) else { return };
    let Some(pokemon) = player_state.team.get_mut(nickname) else { return };

    pokemon.status = line.status.clone();
}
