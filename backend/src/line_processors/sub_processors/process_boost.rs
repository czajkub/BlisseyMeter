use crate::models::lines::PokemonRef;
use crate::models::GameState;

pub fn process_boost(state: &mut GameState, target: &PokemonRef, stat: &str, amount: i8) {
    let Some(pokemon) = state.get_pokemon_mut(target) else {
        return;
    };
    pokemon.battle.stat_boosts.apply_boost(stat, amount);
}
