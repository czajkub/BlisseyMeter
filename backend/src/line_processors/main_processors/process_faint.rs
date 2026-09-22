use crate::schema::lines::PokemonRef;
use crate::schema::state::GameState;

pub fn process_faint(state: &mut GameState, source_pokemon: &PokemonRef) -> Result<(), String> {
    let Some(player_state) = state.get_player_state_mut(source_pokemon.player.as_str()) else {
        return Err(format!("Couldn't get state of player {}", source_pokemon.player.as_str()));
    };

    if player_state.active_pokemon.as_deref() == Some(&source_pokemon.pokemon_nickname) {
        player_state.active_pokemon = None;
    }

    if let Some(pokemon_state) = player_state.team.get_mut(&source_pokemon.pokemon_nickname) {
        pokemon_state.condition.is_fainted = true;
    }
    
    Ok(())
}
