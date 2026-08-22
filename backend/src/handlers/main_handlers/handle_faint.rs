use crate::schema::lines::{MainLine, MainLineKind};
use crate::schema::state::GameState;

pub fn handle_faint(state: &mut GameState, line: &MainLine) {
    let MainLineKind::Faint { source_pokemon } = &line.kind else {
        return;
    };
    let Some(player_state) = state.get_player_state_mut(source_pokemon.player.as_str()) else {
        return;
    };

    player_state.active_pokemon = None;

    if let Some(pokemon_state) = player_state.team.get_mut(&source_pokemon.pokemon_nickname) {
        pokemon_state.is_fainted = true;
    }
}
