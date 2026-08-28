use crate::schema::lines::PokemonRef;
use crate::schema::state::GameState;

pub fn handle_faint(state: &mut GameState, source_pokemon: &PokemonRef) {
    let Some(player_state) = state.get_player_state_mut(source_pokemon.player.as_str()) else {
        return;
    };

    if player_state.active_pokemon.as_deref() == Some(&source_pokemon.pokemon_nickname) {
        player_state.active_pokemon = None;
    }

    if let Some(pokemon_state) = player_state.team.get_mut(&source_pokemon.pokemon_nickname) {
        pokemon_state.is_fainted = true;
    }
}
