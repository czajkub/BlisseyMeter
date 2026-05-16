use crate::constants::luck_weights::*;
use crate::constants::moves::moves;
use crate::schema::lines::line_types::SubLineType;
use crate::schema::lines::main_lines::MainLine;
use crate::schema::state::{GameState, LuckCategory, LuckEvent};

pub fn handle_move(state: &mut GameState, line: &MainLine) {
    let current_move = line.move_name.as_ref();
    let moves_map = moves();
    let move_data = moves_map.get(current_move.unwrap_or(&String::new()));

    let mut luck_events: Vec<LuckEvent> = Vec::new();

    let move_accuracy = move_data
        .map_or(100, |m| m.accuracy.unwrap_or(100));

    let secondary_effect_chance = move_data
        .map_or(0, |m| m.secondary_effect.unwrap_or(0));

    let has_unboost_subline = line.sublines.iter().any(|subline| subline.line_type == SubLineType::Unboost);


    if  secondary_effect_chance > 0 && secondary_effect_chance < 100 {
        println!("Secondary effect chance: {secondary_effect_chance}, unboost subline: {has_unboost_subline}");
        if !has_unboost_subline {
            luck_events.push(LuckEvent {
                turn: state.turn,
                pokemon: line.pokemon_nickname.clone(),
                category: LuckCategory::SecondaryEffect,
                score: SECONDARY_EFFECT_WEIGHT * (secondary_effect_chance as f64 / 100.0),
                description: format!("Didn't activate secondary effect of {}", current_move.unwrap_or(&String::new())),
                source_move: current_move.cloned(),
                is_beneficial: false,
            });
        }
    }

    for subline in &line.sublines {
        match subline.line_type {
            SubLineType::Crit => {
                println!(
                    "Crit found! Move: {:?} for player {:?}",
                    line.move_name, line.player
                );
                let pokemon_with_nick = format!(
                    "{} ({})",
                    line.pokemon_nickname,
                    line.species.clone().unwrap_or_default(),
                );
                luck_events.push(LuckEvent {
                    turn: state.turn,
                    pokemon: pokemon_with_nick,
                    category: LuckCategory::CriticalHit,
                    score: CRIT_WEIGHT,
                    description: String::new(),
                    source_move: current_move.cloned(),
                    is_beneficial: true,
                });
            }
            SubLineType::Miss => {
                println!(
                    "Miss found! Move: {:?} for player {:?}",
                    line.move_name, line.player
                );
                let pokemon_with_nick = format!(
                    "{} ({})",
                    line.pokemon_nickname,
                    line.species.clone().unwrap_or_default(),
                );
                let miss_chance = 100.0 - (move_accuracy as f64);
                luck_events.push(LuckEvent {
                    turn: state.turn,
                    pokemon: pokemon_with_nick,
                    category: LuckCategory::AccuracyMiss,
                    score: MISS_WEIGHT * (1.0 - (miss_chance / 100.0)),
                    description: format!("Move accuracy: {move_accuracy}"),
                    source_move: current_move.cloned(),
                    is_beneficial: false,
                });
            }
            SubLineType::Unboost => {
                println!(
                    "Unboost found! Move: {:?} for player {:?}",
                    line.move_name, line.player
                );
                let pokemon_with_nick = format!(
                    "{} ({})",
                    line.pokemon_nickname,
                    line.species.clone().unwrap_or_default(),
                );
                luck_events.push(LuckEvent {
                    turn: state.turn,
                    pokemon: pokemon_with_nick,
                    category: LuckCategory::SecondaryEffect,
                    score: SECONDARY_EFFECT_WEIGHT * ((100.0 - secondary_effect_chance as f64) / 100.0),
                    description: format!("Move secondary effect chance: {secondary_effect_chance}"),
                    source_move: current_move.cloned(),
                    is_beneficial: true,
                });
            }
            _ => {}
        }
    }

    if let Some(player_state) = state.get_player_state_mut(&line.player) {
        player_state.luck_events.extend(luck_events);
    } else {
        panic!("Invalid player: {}", line.player);
    }
}
