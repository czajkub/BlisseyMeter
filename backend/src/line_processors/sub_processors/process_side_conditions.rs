use crate::models::lines::PlayerRef;
use crate::models::{GameState, SideCondition};

pub fn process_side_conditions(
    state: &mut GameState,
    player_ref: &PlayerRef,
    side_condition: &SideCondition,
    start: bool
) {
    state.field.update_side_condition(player_ref.player, side_condition, start);
}
