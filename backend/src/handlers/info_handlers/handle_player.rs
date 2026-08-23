use crate::schema::lines::InfoLine;
use crate::schema::state::GameState;

pub fn handle_player(state: &mut GameState, line: &InfoLine) {
    let InfoLine::Player {
        player,
        name,
        avatar,
    } = line
    else {
        return;
    };
    let Some(player) = player.as_deref() else {
        return;
    };
    let Some(player_state) = state.get_player_state_mut(player) else {
        return;
    };

    if let Some(name) = name {
        player_state.name = name.clone();
    }
    if let Some(avatar) = avatar {
        player_state.avatar = avatar.clone();
    }
}
