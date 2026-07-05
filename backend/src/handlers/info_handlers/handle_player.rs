use crate::schema::lines::info_lines::InfoLine;
use crate::schema::state::GameState;

pub fn handle_player(state: &mut GameState, line: &InfoLine) {
    let Some(player) = line.player.as_deref() else { return };
    let Some(player_state) = state.get_player_state_mut(player) else { return };

    if let Some(nick) = &line.player_nick {
        player_state.name = nick.clone();
    }
    if let Some(avatar) = &line.avatar {
        player_state.avatar = avatar.clone();
    }
}