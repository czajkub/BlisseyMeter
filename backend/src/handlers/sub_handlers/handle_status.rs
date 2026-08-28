use crate::schema::lines::PokemonRef;
use crate::schema::state::GameState;
use crate::schema::state::Status;

pub fn handle_status(state: &mut GameState, target: &PokemonRef, status: Option<&Status>) {
    let Some(pokemon) = state.get_pokemon_mut(target) else {
        return;
    };

    let Some(status) = status else { return };
    pokemon.status = Some(status.clone());
    pokemon.status_turns = 0;
}
