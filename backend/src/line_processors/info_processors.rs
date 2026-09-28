use crate::models::lines::InfoLine;
use crate::models::GameState;

pub mod process_poke;
pub use process_poke::process_poke;

pub mod process_player;
pub use process_player::process_player;

pub mod process_upkeep;
pub use process_upkeep::process_upkeep;

pub fn process_info_line(state: &mut GameState, line: &InfoLine) {
    match line {
        InfoLine::Poke {
            player, species, gender,
        } => process_poke(state, player, species, gender),
        InfoLine::Player {
            player,
            name,
            avatar,
        } => process_player(state, player.as_deref(), name.as_deref(), avatar.as_deref()),
        InfoLine::Upkeep { } => { process_upkeep(state) }
        InfoLine::Turn { turn } => { state.turn = *turn; }
    }
}
