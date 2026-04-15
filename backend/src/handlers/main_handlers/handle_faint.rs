use crate::schema::lines::main_lines::MainLine;
use crate::schema::state::GameState;

pub fn handle_faint(state: &mut GameState, line: &MainLine) {
    match line.player.as_str() {
        "p1" => {
            state.p1.active_pokemon = None;
            let pokemon_state = state.p1.team.get_mut(&line.pokemon_nickname).unwrap();
            pokemon_state.is_fainted = true;
        }
        "p2" => {
            state.p2.active_pokemon = None;
            let pokemon_state = state.p2.team.get_mut(&line.pokemon_nickname).unwrap();
            pokemon_state.is_fainted = true;
        }
        _ => {
            panic!("Invalid player: {}", line.player);
        }
    }
}
