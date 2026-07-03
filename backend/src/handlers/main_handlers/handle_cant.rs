use crate::constants::luck_weights::{SECONDARY_EFFECT_WEIGHT, STATUS_WEIGHT};
use crate::schema::lines::main_lines::MainLine;
use crate::schema::state::{GameState, LuckCategory, LuckEvent};

fn handle_flinch(state: &mut GameState, line: &MainLine, current_turn: u32) {
     let target_display = state.get_player_state(&line.player)
        .map(|p| p.pokemon_display_name(&line.pokemon_nickname))
        .unwrap_or_else(|| line.pokemon_nickname.clone());

    let attacker_display = match &line.target_pokemon_nickname {
        Some(nick) => state.get_opponent_state(&line.player)
            .map(|p| p.pokemon_display_name(nick))
            .unwrap_or_else(|| nick.clone()),
        None => state.get_opponent_state(&line.player)
            .map(|p| p.active_pokemon_display_name())
            .unwrap_or_default(),
    };

    if let Some(affected_state) = state.get_player_state_mut(&line.player) {
        affected_state.luck_events.push(LuckEvent {
            turn: current_turn,
            pokemon: target_display.clone(),
            category: LuckCategory::Flinch,
            score: SECONDARY_EFFECT_WEIGHT * 0.30,
            description: "Flinched by opponent".to_string(),
            source_move: None,
            is_beneficial: false,
        });

        if let Some(target_pokemon) = affected_state.team.get_mut(&line.pokemon_nickname) {
            target_pokemon.pending_flinch_chance = None;
        }
    }

    if let Some(opponent_state) = state.get_opponent_state_mut(&line.player) {
        opponent_state.luck_events.push(LuckEvent {
            turn: current_turn,
            pokemon: attacker_display,
            category: LuckCategory::SecondaryEffect,
            score: SECONDARY_EFFECT_WEIGHT * 0.70,
            description: format!("Flinched target: {target_display}"),
            source_move: None,
            is_beneficial: true,
        });
    }
}



fn handle_paralysis(state: &mut GameState, line: &MainLine, current_turn: u32) {
    let pokemon_display = state
        .get_player_state(&line.player)
        .map(|p| p.pokemon_display_name(&line.pokemon_nickname))
        .unwrap_or_else(|| line.pokemon_nickname.clone());

    if let Some(player_state) = state.get_player_state_mut(&line.player) {
        player_state.luck_events.push(LuckEvent {
            turn: current_turn,
            pokemon: pokemon_display,
            category: LuckCategory::StatusTurn,
            score: STATUS_WEIGHT * 0.75,
            description: "Fully paralyzed".to_string(),
            source_move: None,
            is_beneficial: false,
        });
    }
}

fn handle_sleep(state: &mut GameState, line: &MainLine) {
    if let Some(player_state) = state.get_player_state_mut(&line.player) {
        if let Some(active_mon_state) = player_state.get_active_pokemon_state_mut() {
            active_mon_state.increment_status_turns();
        }
    }

}

pub fn handle_cant(state: &mut GameState, line: &MainLine) {
    let current_turn = state.turn;
    let reason = line.reason.as_deref().unwrap_or_default();

    match reason {
        "flinch" => handle_flinch(state, line, current_turn),
        "par" => handle_paralysis(state, line, current_turn),
        "slp" => handle_sleep(state, line),
        _ => {}
    }
}
