use crate::models::lines::PokemonRef;
use crate::models::{GameState, Status};

pub fn process_curestatus(
    state: &mut GameState,
    source_pokemon: &PokemonRef,
    status: Option<&Status>,
) -> Result<(), String> {
    let current_turn = state.turn;
    let player = source_pokemon.player.as_str();
    let nickname = &source_pokemon.pokemon_nickname;

    let Some(status) = status else {
        return Err(format!("Couldn't recognize cured status"));
    };

    let Some(player_state) = state.get_player_state_mut(player) else {
        return Err(format!("Couldn't get state of player {}", player));
    };

    if status == &Status::Sleep {
        player_state.resolve_sleep_luck(nickname, current_turn, "Woke up");
    } else if let Some(pokemon) = player_state.get_pokemon_mut(nickname) {
        pokemon.condition.status = None;
        pokemon.condition.status_turns = 0;
    }

    Ok(())
}
