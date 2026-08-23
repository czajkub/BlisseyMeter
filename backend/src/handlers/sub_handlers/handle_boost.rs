use crate::schema::lines::SubLine;
use crate::schema::state::GameState;

pub fn handle_boost(state: &mut GameState, line: &SubLine) {
    let (target, stat, amount) = match line {
        SubLine::Boost {
            target,
            stat,
            amount,
        } => (target, stat, *amount),
        SubLine::Unboost {
            target,
            stat,
            amount,
        } => (target, stat, -*amount),
        _ => return,
    };
    let Some(player_state) = state.get_player_state_mut(target.player.as_str()) else {
        return;
    };
    let Some(pokemon) = player_state.team.get_mut(&target.pokemon_nickname) else {
        return;
    };
    pokemon.stat_boosts.apply_boost(stat, amount);
}
