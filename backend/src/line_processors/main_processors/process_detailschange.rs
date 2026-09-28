use crate::schema::lines::PokemonRef;
use crate::schema::state::GameState;

pub fn process_detailschange(
    state: &mut GameState,
    source_pokemon: &PokemonRef,
    new_form: &str
) -> Result<(), String> {
    let Some(player_state) = state.get_player_state_mut(source_pokemon.player.as_str()) else {
        return Err(format!("Couldn't get state of pokemon {}", source_pokemon.player.as_str()));
    };

    player_state.set_active_pokemon(&source_pokemon.pokemon_nickname);

    if let Some(pokemon_state) = player_state.get_pokemon_mut(&source_pokemon.pokemon_nickname) {
        pokemon_state.identity.species = new_form.to_string();
    }
    
    Ok(())
}
