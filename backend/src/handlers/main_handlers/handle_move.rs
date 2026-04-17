use crate::constants::luck_weights::*;
use crate::schema::lines::line_types::SubLineType;
use crate::schema::lines::main_lines::MainLine;
use crate::schema::state::{GameState, LuckCategory, LuckEvent};

pub fn handle_move(state: &mut GameState, line: &MainLine) {
    let current_move = line.move_name.as_ref();

    let mut luck_events: Vec<LuckEvent> = Vec::new();

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
                    description: "".to_string(),
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
