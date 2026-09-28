use crate::models::lines::PokemonRef;
use crate::models::GameState;
use crate::models::Status;

pub fn process_status(state: &mut GameState, target: &PokemonRef, status: Option<&Status>) {
    let Some(pokemon) = state.get_pokemon_mut(target) else {
        return;
    };

    let Some(status) = status else { return };
    pokemon.condition.status = Some(status.clone());
    pokemon.condition.status_turns = 0;
}
