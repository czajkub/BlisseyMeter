use crate::schema::lines::line_types::SubLineType;
use crate::schema::lines::sub_lines::SubLine;
use crate::schema::state::GameState;

pub fn handle_boost(state: &mut GameState, line: &SubLine) {
    let Some(player) = line.player.as_deref() else { return };
    let Some(nickname) = line.pokemon_nickname.as_deref() else { return };
    let Some(player_state) = state.get_player_state_mut(player) else { return };
    let Some(pokemon) = player_state.team.get_mut(nickname) else { return };

    let Some(stat) = line.stat.as_deref() else { return; };
    let Some(amount) = line.amount else { return; };
    let is_positive = match line.line_type {
        SubLineType::Boost => 1,
        SubLineType::Unboost => -1,
        _ => 0,
    };

    pokemon.stat_boosts.apply_boost(stat, amount * is_positive);
}
