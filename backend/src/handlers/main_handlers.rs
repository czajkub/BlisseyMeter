use crate::schema::lines::line_types::MainLineType;
use crate::schema::lines::main_lines::MainLine;
use crate::schema::state::GameState;

pub mod handle_switch;
pub use handle_switch::handle_switch;

pub fn handle_main_line(state: &mut GameState, line: &MainLine) {
    match line.line_type {
        MainLineType::Switch => handle_switch(state, line),
        // MainLineType::Move => handle_move(state, line),
        // MainLineType::Faint => handle_faint(state, line),
        // MainLineType::DetailsChange => handle_detailschange(state, line),
        _ => panic!("Invalid or not implemented line type: {:?}", line.line_type),
    }
}
