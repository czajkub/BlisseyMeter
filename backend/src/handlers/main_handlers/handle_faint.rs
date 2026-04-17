use crate::schema::lines::main_lines::MainLine;
use crate::schema::state::GameState;

pub fn handle_faint(state: &mut GameState, line: &MainLine) {
    if let Some(player_state) = state.get_player_state_mut(&line.player) {
        player_state.active_pokemon = None;
        if let Some(pokemon_state) = player_state.team.get_mut(&line.pokemon_nickname) {
            pokemon_state.is_fainted = true;
        }
    } else {
        panic!("Invalid player: {}", line.player);
    }
}
