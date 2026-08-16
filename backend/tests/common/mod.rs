use backend::analyze::analyze;
use backend::schema::state::{GameState, LuckCategory, LuckEvent};

pub const OPENING_LINES: Vec<String> = &[
    "|player|p1|Adam|",
    "|player|p2|Bob|",
    "|poke|p1|Mew|",
    "|poke|p1|Primarina|",
    "|poke|p2|Mew|",
    "|poke|p2|Mudkip|",
    "|turn|1",
    "|switch|p1a: Cinderace|Cinderace|100/100",
    "|switch|p2a: Blissey|Blissey|100/100",
];


pub fn run_scenario(scenario: Vec<String>) -> GameState {
    
}