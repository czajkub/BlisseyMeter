use crate::schema::state::{Status, Weather};

#[derive(Debug, Clone)]
pub enum Line {
    Main(MainLine),
    Info(InfoLine),
    Sub(SubLine),
    Unknown,
}

#[derive(Debug, Clone)]
pub struct MainLine {
    pub kind: MainLineKind,
    pub sublines: Vec<SubLine>,
}

#[derive(Debug, Clone)]
pub enum MainLineKind {
    Switch {
        source_pokemon: PokemonRef,
        species: String,
        hp: Hp,
    },
    Move {
        source_pokemon: PokemonRef,
        move_name: String,
        target: PokemonRef,
    },
    Faint {
        source_pokemon: PokemonRef,
},
    Cant {
        source_pokemon: PokemonRef,
        reason: String,
        source: Option<PokemonRef>,
    },
    DetailsChange {
        source_pokemon: PokemonRef,
        new_form: String,
    },
    CureStatus {
        source_pokemon: PokemonRef,
        cured_status: Option<Status>,
        meta: String,
    },
    WeatherChange {
        new_weather: Weather,
    },
}

#[derive(Debug, Clone)]
pub enum SubLine {
    Damage { target: PokemonRef, hp: Hp, source: Option<String> },
    Heal { target: PokemonRef, hp: Hp },
    Boost { target: PokemonRef, stat: String, amount: i8 },
    Unboost { target: PokemonRef, stat: String, amount: i8 },
    Status { target: PokemonRef, status: Option<Status>, from: Option<String> },
    Miss { source: PokemonRef, target: PokemonRef },
    Crit { target: PokemonRef },
    Resisted { target: PokemonRef },
    SuperEffective { target: PokemonRef },
    Immune { target: PokemonRef },
    EndItem { target: PokemonRef, item: String, from: Option<String>, of: Option<String> },
    Activate { target: PokemonRef, ability: String },
    Terastallize { target: PokemonRef, tera_type: String },
    Mega { target: PokemonRef, species: String, mega_stone: String },
}

#[derive(Debug, Clone)]
pub enum InfoLine {
    Turn {
        turn: u32,
    },
    Player {
        player: Option<String>,
        name: Option<String>,
        avatar: Option<String>,
    },
    Poke {
        player: String,
        species: String,
        gender: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerId {
    P1,
    P2,
}

impl PlayerId {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "p1" | "p1a" | "p1b" => Some(Self::P1),
            "p2" | "p2a" | "p2b" => Some(Self::P2),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::P1 => "p1",
            Self::P2 => "p2",
        }
    }
}

#[derive(Debug, Clone)]
pub struct PokemonRef {
    pub player: PlayerId,
    pub pokemon_nickname: String,
}

#[derive(Debug, Clone, Copy)]
pub struct Hp {
    pub current: u8,
    pub max: u8,
}
