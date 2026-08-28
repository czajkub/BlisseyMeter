use crate::schema::lines::InfoLine;
use crate::schema::state::GameState;

pub mod handle_poke;
pub use handle_poke::handle_poke;

pub mod handle_player;
pub use handle_player::handle_player;

pub fn handle_info_line(state: &mut GameState, line: &InfoLine) {
    match line {
        InfoLine::Poke {
            player, species, ..
        } => handle_poke(state, player, species),
        InfoLine::Player {
            player,
            name,
            avatar,
        } => handle_player(state, player.as_deref(), name.as_deref(), avatar.as_deref()),
        InfoLine::Turn { .. } => {}
    }
}
