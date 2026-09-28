use crate::models::lines::PokemonRef;
use crate::models::GameState;

pub fn process_faint(state: &mut GameState, source_pokemon: &PokemonRef) -> Result<(), String> {
    let current_turn = state.turn;
    let Some(player_state) = state.get_player_state_mut(source_pokemon.player.as_str()) else {
        return Err(format!("Couldn't get state of player {}", source_pokemon.player.as_str()));
    };
    let nickname = &source_pokemon.pokemon_nickname;
    
    if player_state.is_active_pokemon(nickname) {
        player_state.clear_active_pokemon();
    }

    if let Some(pokemon_state) = player_state.get_pokemon_mut(nickname) {
        pokemon_state.condition.is_fainted = true;
    }
    
    player_state.resolve_sleep_luck(nickname, current_turn, "Fainted while asleep");

    Ok(())
}
