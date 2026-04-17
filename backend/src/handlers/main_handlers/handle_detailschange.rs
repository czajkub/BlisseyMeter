use crate::schema::lines::main_lines::MainLine;
use crate::schema::state::GameState;

pub fn handle_detailschange(state: &mut GameState, line: &MainLine) {
    if let Some(player_state) = state.get_player_state_mut(&line.player) {
        player_state.active_pokemon = Some(line.pokemon_nickname.clone());
        if let Some(pokemon_state) = player_state.team.get_mut(&line.pokemon_nickname) {
            pokemon_state.species = line.species.clone().unwrap_or_default();
        }
    } else {
        panic!("Invalid player: {}", line.player);
    }
}
