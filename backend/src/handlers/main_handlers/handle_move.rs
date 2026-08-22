use crate::constants::flinch_chances::FLINCH_MOVES;
use crate::constants::luck_weights::*;
use crate::constants::moves::moves;
use crate::handlers::sub_handlers::{handle_boost, handle_status};
use crate::schema::lines::{MainLine, MainLineKind, SubLine};
use crate::schema::state::{GameState, LuckCategory, LuckEvent, Status};

fn check_preconditions(
    state: &mut GameState,
    source_player: &str,
    source_nickname: &str,
    current_turn: u32,
) {
    let check_passed_flinch = state
        .get_player_state_mut(source_player)
        .and_then(|player| player.take_pending_flinch());

    if let Some((flinch_chance, source_move, _active_nick)) = check_passed_flinch
        && flinch_chance > 0
        && flinch_chance < 100
        && let Some(opponent_state) = state.get_opponent_state_mut(source_player)
    {
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

    let Some(player_state) = state.get_player_state_mut(source_player) else {
        return;
    };
    let Some(pokemon) = player_state.team.get(source_nickname) else {
        return;
    };
    let pokemon_status = pokemon.status.clone();
    let pokemon_display = player_state.pokemon_display_name(source_nickname);

    if pokemon_status == Some(Status::Paralysis) {
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
    let MainLineKind::Move {
        source_pokemon,
        move_name,
        ..
    } = &line.kind
    else {
        return;
    };
    let source_player = source_pokemon.player.as_str();
    let source_nickname = &source_pokemon.pokemon_nickname;
    let current_turn = state.turn;

    check_preconditions(state, source_player, source_nickname, current_turn);

    let mut has_miss_subline = false;
    let mut has_secondary_subline = false;

    for subline in &line.sublines {
        match subline {
            SubLine::Miss { .. } => has_miss_subline = true,
            SubLine::Boost { .. } | SubLine::Unboost { .. } => {
                has_secondary_subline = true;
                handle_boost(state, subline);
            }
            SubLine::Status { .. } => {
                has_secondary_subline = true;
                handle_status(state, subline);
            }
            _ => {}
        }
    }

    let Some(player_state) = state.get_player_state_mut(source_player) else {
        return;
    };
    let move_data = moves().get(move_name);
    let move_accuracy = move_data.map_or(100, |data| data.get_accuracy());
    let secondary_effect_chance = move_data
        .and_then(|data| data.secondary_effect)
        .unwrap_or(0);
    let pokemon_display = player_state.pokemon_display_name(source_nickname);
    let mut luck_events = Vec::new();

    if !has_miss_subline
        && secondary_effect_chance > 0
        && secondary_effect_chance < 100
        && !has_secondary_subline
    {
        luck_events.push(LuckEvent {
            turn: current_turn,
            pokemon: pokemon_display.clone(),
            category: LuckCategory::SecondaryEffect,
            score: SECONDARY_EFFECT_WEIGHT * (secondary_effect_chance as f64 / 100.0),
            description: format!("Didn't activate secondary effect of {move_name}"),
            source_move: Some(move_name.clone()),
            is_beneficial: false,
        });
    }

    for subline in &line.sublines {
        match subline {
            SubLine::Crit { .. } => luck_events.push(LuckEvent {
                turn: current_turn,
                pokemon: pokemon_display.clone(),
                category: LuckCategory::CriticalHit,
                score: CRIT_WEIGHT,
                description: "Critical hit!".to_string(),
                source_move: Some(move_name.clone()),
                is_beneficial: true,
            }),
            SubLine::Miss { .. } => luck_events.push(LuckEvent {
                turn: current_turn,
                pokemon: pokemon_display.clone(),
                category: LuckCategory::AccuracyMiss,
                score: MISS_WEIGHT * (move_accuracy as f64 / 100.0),
                description: format!("Missed move with accuracy {move_accuracy}"),
                source_move: Some(move_name.clone()),
                is_beneficial: false,
            }),
            SubLine::Boost { .. } | SubLine::Unboost { .. } | SubLine::Status { .. }
                if secondary_effect_chance > 0 && secondary_effect_chance < 100 =>
            {
                luck_events.push(LuckEvent {
                    turn: current_turn,
                    pokemon: pokemon_display.clone(),
                    category: LuckCategory::SecondaryEffect,
                    score: SECONDARY_EFFECT_WEIGHT
                        * ((100.0 - secondary_effect_chance as f64) / 100.0),
                    description: format!(
                        "Secondary effect activated - {secondary_effect_chance}% chance"
                    ),
                    source_move: Some(move_name.clone()),
                    is_beneficial: true,
                });
            }
            _ => {}
        }
    }

    player_state.luck_events.extend(luck_events);

    if !has_miss_subline
        && let Some(&(move_name, flinch_chance)) =
            FLINCH_MOVES.iter().find(|(name, _)| *name == move_name)
        && let Some(opponent_state) = state.get_opponent_state_mut(source_player)
    {
        opponent_state.set_active_pending_flinch(flinch_chance, move_name.to_string());
    }
}
