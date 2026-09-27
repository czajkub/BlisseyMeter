use crate::schema::lines::{MainLine, MainLineKind};
use crate::schema::state::GameState;

pub mod process_cant;
pub mod process_curestatus;
pub mod process_detailschange;
pub mod process_faint;
pub mod process_move;
pub mod process_switch;

pub use process_cant::process_cant;
pub use process_curestatus::process_curestatus;
pub use process_detailschange::process_detailschange;
pub use process_faint::process_faint;
pub use process_move::process_move;
pub use process_switch::process_switch;

pub fn process_main_line(state: &mut GameState, line: &MainLine) -> Result<(), String> {
    match &line.kind {
        MainLineKind::Switch {
            source_pokemon,
            species,
            hp,
        } => process_switch(state, source_pokemon, species, hp),
        MainLineKind::Move {
            source_pokemon,
            move_name,
            target,
            tag,
        } => process_move(state, source_pokemon, move_name, target.as_ref(), tag, &line.sublines),
        MainLineKind::Faint { source_pokemon } => process_faint(state, source_pokemon),
        MainLineKind::DetailsChange {
            source_pokemon,
            new_form,
        } => process_detailschange(state, source_pokemon, new_form),
        MainLineKind::Cant {
            source_pokemon,
            reason,
            source,
        } => process_cant(state, source_pokemon, reason, source.as_ref()),
        MainLineKind::CureStatus {
            source_pokemon,
            cured_status,
            ..
        } => process_curestatus(state, source_pokemon, cured_status.as_ref()),
        MainLineKind::WeatherChange { new_weather } => { state.field.set_new_weather(new_weather); Ok(()) }
    }
}
