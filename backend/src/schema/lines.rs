use crate::schema::state::Status;

pub mod info_lines;
pub mod line_types;
pub mod main_lines;
pub mod sub_lines;

pub struct MainLine {
    pub kind: MainLineKind,
    pub sublines: Vec<SubLine>,
}

pub enum MainLineKind {
    Switch { source_pokemon: PokemonRef, species: String, hp: Hp },
    Move   { source_pokemon: PokemonRef, move_name: String, target: PokemonRef },
    Faint  { source_pokemon: PokemonRef },
    Cant   { source_pokemon: PokemonRef, reason: String, source: Option<PokemonRef> },
    DetailsChange { source_pokemon: PokemonRef, new_form: String },
    CureStatus { source_pokemon: PokemonRef, cured_status: Status, meta: String},
}

pub enum SubLine {
    Damage { target: PokemonRef, hp: Hp, source: Option<String>},
    Heal   { target: PokemonRef, hp: Hp },
    // update `stat` here to Stat enum - sometime in the future
    StatChange  { target: PokemonRef, stat: String, amount: i8 },
    Status { target: PokemonRef, status: Status, from: Option<String> },
    Miss   { source: PokemonRef, target: PokemonRef },
    Crit   { target: PokemonRef },
    MoveEffectiveness { target: PokemonRef },
    EndItem { target: PokemonRef, item: String, from: Option<String>, of: Option<String> },
    // some more stuff here - activate (ability), tera, mega
}

pub enum PlayerId {
    p1,
    p2,
}

pub struct PokemonRef {
    player: PlayerId,
    pokemon_nickname: String,
}

pub struct Hp {
    current: u8,
    max: u8,
}