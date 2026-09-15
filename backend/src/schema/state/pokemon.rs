use std::collections::HashSet;

use serde::Serialize;

#[derive(Debug, Clone, Default, Serialize)]
pub struct PokemonState {
    pub identity: Identity,
    pub condition: Condition,
    pub battle: BattleState,
    pub set: PokemonSet,
    pub pending: Pending,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct Identity {
    pub nickname: String,
    pub species: String,
    pub gender: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct Condition {
    pub current_hp: u8,
    pub max_hp: u8,
    pub status: Option<Status>,
    pub status_turns: u32,
    pub is_fainted: bool,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct BattleState {
    pub stat_boosts: StatBoosts,
    pub tera_type: Option<String>,
    pub is_mega: bool,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PokemonSet {
    pub evs: Option<EvSpread>,
    pub ivs: Option<IvSpread>,
    pub moves: HashSet<String>,
    pub item: Option<String>,
    pub ability: Option<String>,
    pub nature: Option<String>,
    pub has_pokepaste: bool,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct Pending {
    pub flinch_chance: Option<(u64, String)>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct Stats<T> {
    pub hp: T,
    pub atk: T,
    pub def: T,
    pub spa: T,
    pub spd: T,
    pub spe: T,
    pub acc: T,
    pub eva: T,
}

pub type StatBoosts = Stats<i8>;
pub type EvSpread = Stats<u16>;
pub type IvSpread = Stats<u8>;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Burn,
    Freeze,
    Paralysis,
    Poison,
    Toxic,
    Sleep,
}

impl Status {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "brn" => Some(Status::Burn),
            "frz" => Some(Status::Freeze),
            "par" => Some(Status::Paralysis),
            "psn" => Some(Status::Poison),
            "tox" => Some(Status::Toxic),
            "slp" => Some(Status::Sleep),
            _ => None,
        }
    }
}

impl PokemonState {
    pub fn new(nickname: String, species: String, current_hp: u8, max_hp: u8) -> Self {
        PokemonState {
            identity: Identity {
                nickname,
                species,
                gender: None,
            },
            condition: Condition {
                current_hp,
                max_hp,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn increment_status_turns(&mut self) {
        self.condition.status_turns += 1;
    }
}

impl Stats<i8> {
    pub fn apply_boost(&mut self, stat: &str, amount: i8) {
        let target = match stat {
            "atk" => &mut self.atk,
            "def" => &mut self.def,
            "spa" => &mut self.spa,
            "spd" => &mut self.spd,
            "spe" => &mut self.spe,
            "accuracy" => &mut self.acc,
            "evasion" => &mut self.eva,
            _ => return,
        };
        *target = (*target + amount).clamp(-6, 6);
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
