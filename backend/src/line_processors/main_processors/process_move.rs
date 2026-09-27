use crate::constants::flinch_chances::FLINCH_MOVES;
use crate::constants::luck_weights::*;
use crate::constants::moves::moves;
use crate::line_processors::sub_processors::{process_boost, process_status};
use crate::schema::lines::{MoveTag, PokemonRef, SubLine};
use crate::schema::state::{GameState, LuckCategory, LuckEvent, Status};

fn check_preconditions(
    state: &mut GameState,
    source_player: &str,
    source_nickname: &str,
    current_turn: u32,
) -> Result<(), String> {
    let check_passed_flinch = state
        .get_player_state_mut(source_player)
        .and_then(|player| player.take_pending_flinch());

    if let Some((flinch_chance, source_move, _active_nick)) = check_passed_flinch
        && flinch_chance > 0
        && flinch_chance < 100
        && let Some(opponent_state) = state.get_opponent_state_mut(source_player)
    {
        let attacker_display = opponent_state.active_pokemon_display_name();
        opponent_state.add_luck_event(LuckEvent {
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
        return Err(format!("Couldn't get state of player {}", source_player));
    };
    let Some(pokemon) = player_state.team.get(source_nickname) else {
        return  Err(format!("Couldn't get state of pokemon {}", source_nickname));
    };
    let pokemon_status = pokemon.condition.status.clone();
    let pokemon_display = player_state.pokemon_display_name(source_nickname);

    if pokemon_status == Some(Status::Paralysis) {
        player_state.add_luck_event(LuckEvent {
            turn: current_turn,
            pokemon: pokemon_display,
            category: LuckCategory::StatusTurn,
            score: STATUS_WEIGHT * 0.25,
            description: "Moved despite paralysis".to_string(),
            source_move: None,
            is_beneficial: true,
        });
    }
    Ok(())
}

#[derive(Default)]
struct MovePeculiarities { // this is genuinely an awful name
    has_secondary_sublines: bool,
    has_missed: bool,
    has_failed: bool,
    is_still: bool,
    no_target: bool,
    locked_move: bool,
}

impl MovePeculiarities {
    pub fn has_succeeded(&self) -> bool {
        !self.has_missed && !self.has_failed && !self.is_still && !self.no_target
    }
}

fn apply_move_peculiarities(state: &mut GameState, tag: &Option<MoveTag>, sublines: &[SubLine]) -> MovePeculiarities {
    let mut peculiarities = MovePeculiarities::default();

    for subline in sublines {
        match subline {
            SubLine::Fail { .. } => peculiarities.has_failed = true,
            SubLine::Boost {
                target,
                stat,
                amount,
            } => {
                peculiarities.has_secondary_sublines = true;
                process_boost(state, target, stat, *amount);
            }
            SubLine::Unboost {
                target,
                stat,
                amount,
            } => {
                peculiarities.has_secondary_sublines = true;
                process_boost(state, target, stat, -*amount);
            }
            SubLine::Status { target, status, .. } => {
                peculiarities.has_secondary_sublines = true;
                process_status(state, target, status.as_ref());
            }
            _ => {}
        }
    }

    match tag {
        Some(tag) => {
            match tag {
                MoveTag::Miss => { peculiarities.has_missed = true; }
                MoveTag::Still => { peculiarities.is_still = true; }
                MoveTag::NoTarget => { peculiarities.no_target = true; }
                MoveTag::LockedMove => { peculiarities.locked_move = true; }
            }
        }
        None => { }
    }

    peculiarities
}

fn record_move_luck_events(
    state: &mut GameState,
    source_pokemon: &PokemonRef,
    move_name: &str,
    target_pokemon: Option<&PokemonRef>,
    sublines: &[SubLine],
    current_turn: u32,
    move_result: &MovePeculiarities,
) -> Result<(), String> {
    let source_player = source_pokemon.player.as_str();
    let source_nickname = &source_pokemon.pokemon_nickname;
    
    let move_data = moves().get(move_name);
    
    let move_accuracy = move_data.map_or(
        100.0,
        |data| match target_pokemon {
            Some(target_pokemon) => data.get_accuracy_with_modifiers(
                &state,
                source_pokemon,
                target_pokemon,
            ),
            None => data.get_accuracy(),
        }
    );

    let secondary_effect_chance = move_data
        .and_then(|data| data.secondary_effect)
        .unwrap_or(0);

    
    let Some(player_state) = state.get_player_state_mut(source_player) else {
        return Err(format!("Couldn't get state of player {}", source_player));
    };
    
    let pokemon_display = player_state.pokemon_display_name(source_nickname);
    let mut luck_events = Vec::new();
    
    if move_result.has_succeeded()
        && secondary_effect_chance > 0
        && secondary_effect_chance < 100
        && !move_result.has_secondary_sublines
    {
        luck_events.push(LuckEvent {
            turn: current_turn,
            pokemon: pokemon_display.clone(),
            category: LuckCategory::SecondaryEffect,
            score: SECONDARY_EFFECT_WEIGHT * (secondary_effect_chance as f64 / 100.0),
            description: format!("Didn't activate secondary effect of {move_name}"),
            source_move: Some(move_name.to_string()),
            is_beneficial: false,
        });
    }

    for subline in sublines {
        match subline {
            SubLine::Crit { .. } => luck_events.push(LuckEvent {
                turn: current_turn,
                pokemon: pokemon_display.clone(),
                category: LuckCategory::CriticalHit,
                score: CRIT_WEIGHT,
                description: "Critical hit!".to_string(),
                source_move: Some(move_name.to_string()),
                is_beneficial: true,
            }),
            SubLine::Miss { .. } => luck_events.push(LuckEvent {
                turn: current_turn,
                pokemon: pokemon_display.clone(),
                category: LuckCategory::AccuracyMiss,
                score: MISS_WEIGHT * (move_accuracy as f64 / 100.0),
                description: format!("Missed move with accuracy {move_accuracy}"),
                source_move: Some(move_name.to_string()),
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
                    source_move: Some(move_name.to_string()),
                    is_beneficial: true,
                });
            }
            _ => {}
        }
    }

    player_state.add_luck_events(luck_events);
    Ok(())
}

fn set_pending_flinch(state: &mut GameState, source_player: &str, move_name: &str, has_succeeded: bool) {
    if !has_succeeded {
        return;
    }
    if let Some(&(flinch_move, flinch_chance)) =
        FLINCH_MOVES.iter().find(|(name, _)| *name == move_name)
        && let Some(opponent_state) = state.get_opponent_state_mut(source_player)
    {
        opponent_state.set_active_pending_flinch(flinch_chance, flinch_move.to_string());
    }
}

pub fn process_move(
    state: &mut GameState,
    source_pokemon: &PokemonRef,
    move_name: &str,
    target_pokemon: Option<&PokemonRef>,
    tag: &Option<MoveTag>,
    sublines: &[SubLine],
) -> Result<(), String> {
    let source_player = source_pokemon.player.as_str();
    let source_nickname = &source_pokemon.pokemon_nickname;
    let current_turn = state.turn;

    match check_preconditions(state, source_player, source_nickname, current_turn) {
        Ok(()) => {},
        Err(err) => { return Err(err) }
    };
    
    let move_peculiarities = apply_move_peculiarities(state, tag, sublines);

    match record_move_luck_events(
        state,
        source_pokemon,
        move_name,
        target_pokemon,
        sublines,
        current_turn,
        &move_peculiarities,
    ) {
        Ok(()) => {},
        Err(err) => { return Err(err) }
    };
    
    set_pending_flinch(state, source_player, move_name, move_peculiarities.has_succeeded());
    Ok(())
}
