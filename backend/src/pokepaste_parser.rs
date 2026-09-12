use regex::Regex;

use crate::constants::moves::Move;

pub struct ParsedPokemon {
    nickname: Option<String>,
    species: String,
    item: Option<String>,
    ability: String,
    evs: EVs,
    nature: String,
    ivs: Option<String>,
    moves: Vec<Move>,
}

pub struct EVs {
    hp: u8,
    atk: u8,
    def: u8,
    spa: u8,
    spd: u8,
    spe: u8,
}

pub fn parse_pokepastes(
    p1_pokepaste: Option<String>,
    p2_pokepaste: Option<String>
) -> (ParsedPokemon, ParsedPokemon) {
    
}