use crate::schema::lines::{MainLine, MainLineKind};
use crate::schema::state::GameState;

pub mod handle_switch;
pub use handle_switch::handle_switch;

pub mod handle_faint;
pub use handle_faint::handle_faint;

pub mod handle_detailschange;
pub use handle_detailschange::handle_detailschange;

pub mod handle_cant;
pub mod handle_curestatus;
pub mod handle_move;
pub use handle_cant::handle_cant;
pub use handle_curestatus::handle_curestatus;
pub use handle_move::handle_move;

pub fn handle_main_line(state: &mut GameState, line: &MainLine) {
    match &line.kind {
        MainLineKind::Switch { .. } => handle_switch(state, line),
        MainLineKind::Move { .. } => handle_move(state, line),
        MainLineKind::Faint { .. } => handle_faint(state, line),
        MainLineKind::DetailsChange { .. } => handle_detailschange(state, line),
        MainLineKind::Cant { .. } => handle_cant(state, line),
        MainLineKind::CureStatus { .. } => handle_curestatus(state, line),
    }
}
