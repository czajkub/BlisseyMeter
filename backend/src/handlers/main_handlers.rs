use crate::schema::lines::line_types::MainLineType;
use crate::schema::lines::main_lines::MainLine;
use crate::schema::state::GameState;

pub mod handle_switch;
pub use handle_switch::handle_switch;

pub mod handle_faint;
pub use handle_faint::handle_faint;

pub mod handle_detailschange;
pub use handle_detailschange::handle_detailschange;

pub mod handle_move;
pub mod handle_cant;
pub use handle_move::handle_move;
pub use handle_cant::handle_cant;

pub fn handle_main_line(state: &mut GameState, line: &MainLine) {
    match line.line_type {
        MainLineType::Switch => handle_switch(state, line),
        MainLineType::Move => handle_move(state, line),
        MainLineType::Faint => handle_faint(state, line),
        MainLineType::DetailsChange => handle_detailschange(state, line),
        MainLineType::Cant => handle_cant(state, line),
    }
}
