use crate::schema::lines::main_lines::MainLine;
use crate::schema::state::GameState;

pub fn handle_detailschange(state: &mut GameState, line: &MainLine) {
    let Some(player_state) = state.get_player_state_mut(&line.player) else {
        panic!("Invalid player: {}", line.player);
    };

    player_state.active_pokemon = Some(line.pokemon_nickname.clone());

    if let Some(pokemon_state) = player_state.team.get_mut(&line.pokemon_nickname) {
        if let Some(ref species) = line.species {
            pokemon_state.species = species.clone();
        }
    }
}
