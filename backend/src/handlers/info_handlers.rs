use crate::schema::lines::InfoLine;
use crate::schema::state::GameState;

pub mod handle_poke;
pub use handle_poke::handle_poke;

pub mod handle_player;
pub use handle_player::handle_player;

pub fn handle_info_line(state: &mut GameState, line: &InfoLine) {
    match line {
        InfoLine::Poke { .. } => handle_poke(state, line),
        InfoLine::Player { .. } => handle_player(state, line),
        InfoLine::Turn { .. } => {}
    }
}
