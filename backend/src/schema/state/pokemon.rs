use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct PokemonState {
    pub nickname: String,
    pub species: String,
    pub current_hp: u8,
    pub max_hp: u8,
    pub stat_boosts: StatBoosts,
    pub status: Option<Status>,
    pub is_fainted: bool,
    pub tera_type: Option<String>,
    pub is_mega: bool,
    pub pending_flinch_chance: Option<(u64, String)>, // (flinch_chance, move_name)
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct StatBoosts {
    pub atk: i8,
    pub def: i8,
    pub spa: i8,
    pub spd: i8,
    pub spe: i8,
    pub acc: i8,
    pub eva: i8,
}

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
            nickname,
            species,
            current_hp,
            max_hp,
            stat_boosts: StatBoosts::default(),
            status: None,
            is_fainted: false,
            tera_type: None,
            is_mega: false,
            pending_flinch_chance: None,
        }
    }
}

impl StatBoosts {
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
