use crate::schema::lines::info_lines::InfoLine;
use crate::schema::lines::line_types::InfoLineType;
use crate::schema::state::GameState;

pub mod handle_poke;
pub use handle_poke::handle_poke;

pub fn handle_info_line(state: &mut GameState, line: &InfoLine) {
    match line.line_type {
        InfoLineType::Poke => handle_poke(state, line),
        _ => {}
    }
}
