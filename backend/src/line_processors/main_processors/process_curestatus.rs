use crate::constants::luck_weights::STATUS_WEIGHT;
use crate::schema::lines::PokemonRef;
use crate::schema::state::{GameState, LuckCategory, LuckEvent, Status};

pub fn process_curestatus(
    state: &mut GameState,
    source_pokemon: &PokemonRef,
    status: Option<&Status>,
) -> Result<(), String> {
    let current_turn = state.turn;

    let Some(status) = status else {
        return Err(format!("Couldn't recognize cured status"));
    };

    let Some(player_state) = state.get_player_state_mut(source_pokemon.player.as_str()) else {
        return Err(format!("Couldn't get state of player {}", source_pokemon.player.as_str()));
    };
    let pokemon_display = player_state.pokemon_display_name(&source_pokemon.pokemon_nickname);
    let Some(status_turns) = player_state
        .get_pokemon(&source_pokemon.pokemon_nickname)
        .map(|pokemon| pokemon.condition.status_turns)
    else {
        return Err(format!("Couldn't get status turn counter of cured pokemon"));
    };

    if status == &Status::Sleep {
        if status_turns == 0 {
            return Err(format!("Status cured with 0 turns passed"));
        }

        // equal probability to wake up after 1,2,3 turns.
        // turn 2 is neutral, turn 1 wake-up favours sleeping mon,
        // turn 3 is disadvantageous
        let wake_up_luck = 1.0 / 3.0 * (2.0 - status_turns as f64);

        player_state.add_luck_event(LuckEvent {
            turn: current_turn,
            pokemon: pokemon_display,
            category: LuckCategory::StatusTurn,
            score: STATUS_WEIGHT * wake_up_luck,
            description: format!("Woke up after {status_turns} sleep turn(s)"),
            source_move: None,
            is_beneficial: wake_up_luck > 0.,
        });
    }

    if let Some(pokemon) = player_state.get_pokemon_mut(&source_pokemon.pokemon_nickname) {
        pokemon.condition.status = None;
        pokemon.condition.status_turns = 0;
    }

    Ok(())
}
