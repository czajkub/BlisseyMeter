use crate::constants::luck_weights::*;
use crate::constants::moves::moves;
use crate::schema::lines::line_types::SubLineType;
use crate::schema::lines::main_lines::MainLine;
use crate::schema::state::{GameState, LuckCategory, LuckEvent};

pub fn handle_move(state: &mut GameState, line: &MainLine) {
    let current_turn = state.turn;
    
    let Some(player_state) = state.get_player_state_mut(&line.player) else {
        panic!("Invalid player: {}", line.player);
    };

    let moves_map = moves();
    let move_name = line.move_name.as_deref().unwrap_or_default();
    let move_data = moves_map.get(move_name);

    let move_accuracy = move_data.map_or(100, |m| m.get_accuracy());
    let secondary_effect_chance = move_data.and_then(|m| m.secondary_effect).unwrap_or(0);

    let has_unboost_subline = line
        .sublines
        .iter()
        .any(|s| s.line_type == SubLineType::Unboost);

    let mut luck_events = Vec::new();
    let pokemon_with_nick = format!(
        "{} ({})",
        line.pokemon_nickname,
        line.species.as_deref().unwrap_or_default()
    );

    if secondary_effect_chance > 0 && secondary_effect_chance < 100 {
        println!("Secondary effect chance: {secondary_effect_chance}, unboost subline: {has_unboost_subline}");
        if !has_unboost_subline {
            luck_events.push(LuckEvent {
                turn: current_turn,
                pokemon: pokemon_with_nick.clone(),
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
                println!("Crit found! Move: {:?} for player {:?}", line.move_name, line.player);
                luck_events.push(LuckEvent {
                    turn: current_turn,
                    pokemon: pokemon_with_nick.clone(),
                    category: LuckCategory::CriticalHit,
                    score: CRIT_WEIGHT,
                    description: String::new(),
                    source_move: line.move_name.clone(),
                    is_beneficial: true,
                });
            }
            SubLineType::Miss => {
                println!("Miss found! Move: {:?} for player {:?}", line.move_name, line.player);
                let miss_chance = 100.0 - (move_accuracy as f64);
                luck_events.push(LuckEvent {
                    turn: current_turn,
                    pokemon: pokemon_with_nick.clone(),
                    category: LuckCategory::AccuracyMiss,
                    score: MISS_WEIGHT * (1.0 - (miss_chance / 100.0)),
                    description: format!("Move accuracy: {move_accuracy}"),
                    source_move: line.move_name.clone(),
                    is_beneficial: false,
                });
            }
            SubLineType::Unboost => {
                if secondary_effect_chance == 100 || secondary_effect_chance == 0 {
                    continue;
                }
                println!("Unboost found! Move: {:?} for player {:?}", line.move_name, line.player);
                luck_events.push(LuckEvent {
                    turn: current_turn,
                    pokemon: pokemon_with_nick.clone(),
                    category: LuckCategory::SecondaryEffect,
                    score: SECONDARY_EFFECT_WEIGHT * ((100.0 - secondary_effect_chance as f64) / 100.0),
                    description: format!("Move secondary effect chance: {secondary_effect_chance}"),
                    source_move: line.move_name.clone(),
                    is_beneficial: true,
                });
            }
            _ => {}
        }
    }

    player_state.luck_events.extend(luck_events);
}
