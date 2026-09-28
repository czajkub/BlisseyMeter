use crate::constants::luck_weights::{SECONDARY_EFFECT_WEIGHT, STATUS_WEIGHT};
use crate::schema::lines::PokemonRef;
use crate::schema::state::{GameState, LuckCategory, LuckEvent};

fn process_flinch(
    state: &mut GameState,
    target_player: &str,
    target_nickname: &str,
    source: Option<&crate::schema::lines::PokemonRef>,
    current_turn: u32,
) -> Result<(), String> {
    let target_display = state
        .get_player_state(target_player)
        .map(|player| player.pokemon_display_name(target_nickname))
        .unwrap_or_else(|| target_nickname.to_string());
    let attacker_display = source
        .and_then(|pokemon| {
            state
                .get_opponent_state(target_player)
                .map(|player| player.pokemon_display_name(&pokemon.pokemon_nickname))
        })
        .or_else(|| {
            state
                .get_opponent_state(target_player)
                .map(|player| player.active_pokemon_display_name())
        })
        .unwrap_or_default();

    if let Some(affected_state) = state.get_player_state_mut(target_player) {
        affected_state.add_luck_event(LuckEvent {
            turn: current_turn,
            pokemon: target_display.clone(),
            category: LuckCategory::Flinch,
            score: SECONDARY_EFFECT_WEIGHT * 0.30,
            description: "Flinched by opponent".to_string(),
            source_move: None,
            is_beneficial: false,
        });

        affected_state.clear_pending_flinch(target_nickname);
    }

    if let Some(opponent_state) = state.get_opponent_state_mut(target_player) {
        opponent_state.add_luck_event(LuckEvent {
            turn: current_turn,
            pokemon: attacker_display,
            category: LuckCategory::SecondaryEffect,
            score: SECONDARY_EFFECT_WEIGHT * 0.70,
            description: format!("Flinched target: {target_display}"),
            source_move: None,
            is_beneficial: true,
        });
    }

    Ok(())
}

fn process_paralysis(
    state: &mut GameState,
    player: &str,
    nickname: &str,
    current_turn: u32
) -> Result<(), String> {
    let pokemon_display = state
        .get_player_state(player)
        .map(|state| state.pokemon_display_name(nickname))
        .unwrap_or_else(|| nickname.to_string());

    if let Some(player_state) = state.get_player_state_mut(player) {
        player_state.add_luck_event(LuckEvent {
            turn: current_turn,
            pokemon: pokemon_display,
            category: LuckCategory::StatusTurn,
            score: STATUS_WEIGHT * 0.75,
            description: "Fully paralyzed".to_string(),
            source_move: None,
            is_beneficial: false,
        });
    }

    Ok(())
}

fn process_sleep(state: &mut GameState, player: &str) -> Result<(), String> {
    if let Some(player_state) = state.get_player_state_mut(player)
        && let Some(active_mon_state) = player_state.get_active_pokemon_state_mut()
    {
        active_mon_state.increment_status_turns();
    }
    
    Ok(())
}

pub fn process_cant(
    state: &mut GameState,
    source_pokemon: &PokemonRef,
    reason: &str,
    source: Option<&PokemonRef>,
) -> Result<(), String> {
    let player = source_pokemon.player.as_str();
    let nickname = &source_pokemon.pokemon_nickname;

    match reason {
        "flinch" => process_flinch(state, player, nickname, source, state.turn),
        "par" => process_paralysis(state, player, nickname, state.turn),
        "slp" => process_sleep(state, player),
        _ => Ok(())
    }
}
