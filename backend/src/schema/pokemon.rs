pub struct Pokemon {
    name: String,
    forme: String,
    ability: String,
    item: String,
    moveset: Vec<String>,

    hp: u8,
    maxhp: u8,
    status: String,
    
}

pub struct PokeSpecies {
    pub species: String,
    pub gender: Option<char>,
}