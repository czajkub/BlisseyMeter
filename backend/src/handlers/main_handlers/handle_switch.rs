use crate::schema::lines::main_lines::MainLine;
use crate::schema::state::GameState;

pub fn handle_switch(state: &mut GameState, line: &MainLine) {
    match line.player.as_str() {
        "p1" => {
            state.p1.active_pokemon = Some(line.pokemon_nickname.clone());
        }
        "p2" => {
            state.p2.active_pokemon = Some(line.pokemon_nickname.clone());
        }
        _ => {
            panic!("Invalid player: {}", line.player);
        }
    }
}
