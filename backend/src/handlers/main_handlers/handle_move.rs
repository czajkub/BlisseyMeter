use crate::constants::flinch_chances::FLINCH_MOVES;
use crate::constants::luck_weights::*;
use crate::constants::moves::moves;
use crate::schema::lines::line_types::SubLineType;
use crate::schema::lines::main_lines::MainLine;
use crate::schema::state::{GameState, LuckCategory, LuckEvent, Status};

fn check_preconditions(state: &mut GameState, line: &MainLine, current_turn: u32) {
    // 1. Check Flinch: If the pokemon had a pending flinch chance and successfully moved.
    let check_passed_flinch = state
        .get_player_state_mut(&line.player)
        .and_then(|ps| ps.take_pending_flinch());

    if let Some((flinch_chance, source_move, _active_nick)) = check_passed_flinch {
        if let (true, Some(opponent_state)) = (
            flinch_chance > 0 && flinch_chance < 100,
            state.get_opponent_state_mut(&line.player),
        ) {
            let attacker_display = opponent_state.active_pokemon_display_name();
            opponent_state.luck_events.push(LuckEvent {
                turn: current_turn,
                pokemon: attacker_display,
                category: LuckCategory::SecondaryEffect,
                score: SECONDARY_EFFECT_WEIGHT * (flinch_chance as f64 / 100.0),
                description: format!("Didn't activate flinch of {source_move}"),
                source_move: Some(source_move),
                is_beneficial: false,
            });
        }
    }

    // 2. Check Paralysis: If the pokemon is paralyzed and successfully moved (25% chance to fail).
    let Some(player_state) = state.get_player_state_mut(&line.player) else { return };
    let Some(pokemon) = player_state.team.get(&line.pokemon_nickname) else { return };

    if pokemon.status == Some(Status::Paralysis) {
        let pokemon_display = player_state.pokemon_display_name(&line.pokemon_nickname);
        player_state.luck_events.push(LuckEvent {
            turn: current_turn,
            pokemon: pokemon_display,
            category: LuckCategory::StatusTurn,
            score: STATUS_WEIGHT * 0.25,
            description: "Moved despite paralysis".to_string(),
            source_move: None,
            is_beneficial: true,
        });
    }
}

pub fn handle_move(state: &mut GameState, line: &MainLine) {
    let current_turn = state.turn;

    check_preconditions(state, line, current_turn);

    let Some(player_state) = state.get_player_state_mut(&line.player) else {
        panic!("Invalid player: {}", line.player);
    };

    let moves_map = moves();
    let move_name = line.move_name.as_deref().unwrap_or_default();
    let move_data = moves_map.get(move_name);

    let move_accuracy = move_data.map_or(100, |m| m.get_accuracy());
    let secondary_effect_chance = move_data.and_then(|m| m.secondary_effect).unwrap_or(0);

    let mut has_miss_subline = false;
    let mut has_unboost_subline = false;
    let mut has_status_subline = false;

    for subline in &line.sublines {
        match subline.line_type {
            SubLineType::Miss => has_miss_subline = true,
            SubLineType::Unboost => has_unboost_subline = true,
            SubLineType::Status => has_status_subline = true,
            _ => {},
        }
    }

    let secondary_effect_happened = has_unboost_subline || has_status_subline;

    let pokemon_display = player_state.pokemon_display_name(&line.pokemon_nickname);

    let mut luck_events = Vec::new();

    if !has_miss_subline && secondary_effect_chance > 0 && secondary_effect_chance < 100 {
        if !secondary_effect_happened {
            luck_events.push(LuckEvent {
                turn: current_turn,
                pokemon: pokemon_display.clone(),
                category: LuckCategory::SecondaryEffect,
                score: SECONDARY_EFFECT_WEIGHT * (secondary_effect_chance as f64 / 100.0),
                description: format!("Didn't activate secondary effect of {move_name}"),
                source_move: line.move_name.clone(),
                is_beneficial: false,
            });
        }
    }

    for subline in &line.sublines {
        match subline.line_type {
            SubLineType::Crit => {
                luck_events.push(LuckEvent {
                    turn: current_turn,
                    pokemon: pokemon_display.clone(),
                    category: LuckCategory::CriticalHit,
                    score: CRIT_WEIGHT,
                    description: "Critical hit!".to_string(),
                    source_move: line.move_name.clone(),
                    is_beneficial: true,
                });
            }
            SubLineType::Miss => {
                luck_events.push(LuckEvent {
                    turn: current_turn,
                    pokemon: pokemon_display.clone(),
                    category: LuckCategory::AccuracyMiss,
                    score: MISS_WEIGHT * (move_accuracy as f64 / 100.0),
                    description: format!("Missed move with accuracy {move_accuracy}"),
                    source_move: line.move_name.clone(),
                    is_beneficial: false,
                });
            }
            SubLineType::Unboost | SubLineType::Status => {
                if secondary_effect_chance == 100 || secondary_effect_chance == 0 {
                    continue;
                }
                luck_events.push(LuckEvent {
                    turn: current_turn,
                    pokemon: pokemon_display.clone(),
                    category: LuckCategory::SecondaryEffect,
                    score: SECONDARY_EFFECT_WEIGHT * ((100.0 - secondary_effect_chance as f64) / 100.0),
                    description: format!("Secondary effect activated ({secondary_effect_chance}% chance)"),
                    source_move: line.move_name.clone(),
                    is_beneficial: true,
                });
            }
            _ => {}
        }
    }

    player_state.luck_events.extend(luck_events);

    if !has_miss_subline {
        if let Some(&(move_name_static, flinch_chance)) = FLINCH_MOVES.iter().find(|(m, _)| *m == move_name) {
            if let Some(opponent_state) = state.get_opponent_state_mut(&line.player) {
                opponent_state.set_active_pending_flinch(flinch_chance, move_name_static.to_string());
            }
        }
    }
}
