use crate::schema::lines::{MainLine, MainLineKind};
use crate::schema::state::GameState;

pub fn handle_detailschange(state: &mut GameState, line: &MainLine) {
    let MainLineKind::DetailsChange {
        source_pokemon,
        new_form,
    } = &line.kind
    else {
        return;
    };
    let Some(player_state) = state.get_player_state_mut(source_pokemon.player.as_str()) else {
        return;
    };

    player_state.active_pokemon = Some(source_pokemon.pokemon_nickname.clone());

    if let Some(pokemon_state) = player_state.team.get_mut(&source_pokemon.pokemon_nickname) {
        pokemon_state.species = new_form.clone();
    }
}
