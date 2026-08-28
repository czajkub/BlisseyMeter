use crate::schema::lines::PokemonRef;
use crate::schema::state::GameState;

pub fn handle_boost(state: &mut GameState, target: &PokemonRef, stat: &str, amount: i8) {
    let Some(pokemon) = state.get_pokemon_mut(target) else {
        return;
    };
    pokemon.stat_boosts.apply_boost(stat, amount);
}
