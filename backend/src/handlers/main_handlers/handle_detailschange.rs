use crate::schema::lines::main_lines::MainLine;
use crate::schema::state::GameState;

pub fn handle_detailschange(state: &mut GameState, line: &MainLine) {
    match line.player.as_str() {
        "p1" => {
            state.p1.active_pokemon = Some(line.pokemon_nickname.clone());
            state.p1.get_active_pokemon_mut().unwrap().species = line.species.clone().unwrap();
        }
        "p2" => {
            state.p2.active_pokemon = Some(line.pokemon_nickname.clone());
            state.p2.get_active_pokemon_mut().unwrap().species = line.species.clone().unwrap();
        }
        _ => {
            panic!("Invalid player: {}", line.player);
        }
    }
}
