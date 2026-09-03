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
        MainLineKind::Switch {
            source_pokemon,
            species,
            hp,
        } => handle_switch(state, source_pokemon, species, hp),
        MainLineKind::Move {
            source_pokemon,
            move_name,
            target,
        } => handle_move(state, source_pokemon, move_name, target, &line.sublines),
        MainLineKind::Faint { source_pokemon } => handle_faint(state, source_pokemon),
        MainLineKind::DetailsChange {
            source_pokemon,
            new_form,
        } => handle_detailschange(state, source_pokemon, new_form),
        MainLineKind::Cant {
            source_pokemon,
            reason,
            source,
        } => handle_cant(state, source_pokemon, reason, source.as_ref()),
        MainLineKind::CureStatus {
            source_pokemon,
            cured_status,
            ..
        } => handle_curestatus(state, source_pokemon, cured_status.as_ref()),
    }
}
