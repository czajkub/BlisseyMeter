use crate::constants::luck_weights::STATUS_WEIGHT;
use crate::schema::lines::main_lines::MainLine;
use crate::schema::state::{GameState, LuckCategory, LuckEvent};


pub fn handle_curestatus(state: &mut GameState, line: &MainLine) {
    let current_turn = state.turn;
    let status_str = line.reason.as_deref().unwrap_or_default();

    let Some(player_state) = state.get_player_state_mut(&line.player) else { return };
    let pokemon_display = player_state.pokemon_display_name(&line.pokemon_nickname);
    let Some(pokemon) = player_state.team.get_mut(&line.pokemon_nickname) else { return };

    if status_str == "slp" {
        let slept_turns = pokemon.status_turns;

        if slept_turns == 0 {
            // should never fire - just in case
            return;
        }

        // equal probability to wake up after 1,2,3 turns.
        // turn 2 is neutral, turn 1 wake-up favours sleeping mon,
        // turn 3 is disadvantageous
        let wake_up_luck = 1.0 / 3.0 * (2.0 - slept_turns as f64);

        player_state.luck_events.push(LuckEvent {
            turn: current_turn,
            pokemon: pokemon_display,
            category: LuckCategory::StatusTurn,
            score: STATUS_WEIGHT * wake_up_luck,
            description: format!("Woke up after {slept_turns} sleep turn(s)"),
            source_move: None,
            is_beneficial: wake_up_luck > 0.,
        });
    }

    pokemon.status = None;
    pokemon.status_turns = 0;
}