use crate::schema::state::GameState;

pub fn handle_player(
    state: &mut GameState,
    player: Option<&str>,
    name: Option<&str>,
    avatar: Option<&str>,
) {
    let Some(player) = player else {
        return;
    };
    let Some(player_state) = state.get_player_state_mut(player) else {
        return;
    };

    if let Some(name) = name {
        player_state.name = name.to_string();
    }
    if let Some(avatar) = avatar {
        player_state.avatar = avatar.to_string();
    }
}
