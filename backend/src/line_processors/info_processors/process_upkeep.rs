use crate::models::{GameState};

pub fn process_upkeep(state: &mut GameState) {
    // when weather is implemented - update weather turns here
    // turn count is incremented on |turn line
    for active_state in [&mut state.p1, &mut state.p2] {
        let active_pokemon = active_state.get_active_pokemon_state_mut();
        match active_pokemon {
            Some(active_pokemon) => {
                active_pokemon.increment_status_turns();
                active_pokemon.clear_pending_flinch();
            }
            // active pokemon probably fainted at this point, switch happens at start of next turn
            None => { }
        }
    }
}
