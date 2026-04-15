use crate::schema::lines::main_lines::MainLine;
use crate::schema::state::GameState;

pub fn handle_faint(state: &mut GameState, line: &MainLine) {
    match line.player.as_str() {
        "p1" => {
            state.p1.active_pokemon = None;
        }
        "p2" => {
            state.p2.active_pokemon = None;
        }
        _ => {
            panic!("Invalid player: {}", line.player);
        }
    }
}
