use crate::schema::lines::main_lines::MainLine;
use crate::schema::state::GameState;
use crate::schema::state::PokemonState;

pub fn handle_switch(state: &mut GameState, line: &MainLine) {
    match line.player.as_str() {
        "p1" => {
            state.p1.active_pokemon = Some(line.pokemon_nickname.clone());
            if !state.p1.team.contains_key(&line.pokemon_nickname) {
                state.p1.team.insert(
                    line.pokemon_nickname.clone(),
                    PokemonState::new(
                        line.pokemon_nickname.clone(),
                        line.species.clone().unwrap(),
                        line.pokemon_current_hp.unwrap(),
                        line.pokemon_max_hp.unwrap()
                    )
                );
            }
        }
        "p2" => {
            state.p2.active_pokemon = Some(line.pokemon_nickname.clone());
            if !state.p2.team.contains_key(&line.pokemon_nickname) {
                state.p2.team.insert(
                    line.pokemon_nickname.clone(),
                    PokemonState::new(
                        line.pokemon_nickname.clone(),
                        line.species.clone().unwrap(),
                        line.pokemon_current_hp.unwrap(),
                        line.pokemon_max_hp.unwrap()
                    )
                );
            }
        }
        _ => {
            panic!("Invalid player: {}", line.player);
        }
    }
}
