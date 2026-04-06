#[derive(Debug, Clone, Default)]
pub struct GameState {
    pub turn: u32,
    pub p1: PlayerState,
    pub p2: PlayerState,
    pub field: FieldState,
}

#[derive(Debug, Clone, Default)]
pub struct PlayerState {
    pub name: String,
    pub active_pokemon: Option<String>, // nickname of active pokemon
    pub team: Vec<PokemonState>,
}

#[derive(Debug, Clone)]
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
}

#[derive(Debug, Clone, Default)]
pub struct StatBoosts {
    pub atk: i8, // -6 to +6
    pub def: i8,
    pub spa: i8,
    pub spd: i8,
    pub spe: i8,
    pub acc: i8,
    pub eva: i8,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Burn,
    Freeze,
    Paralysis,
    Poison,
    Toxic,
    Sleep,
}

#[derive(Debug, Clone, Default)]
pub struct FieldState {
    pub weather: Option<Weather>,
    pub terrain: Option<Terrain>,

    // Per-side hazards and screens
    pub p1_side: SideConditions,
    pub p2_side: SideConditions,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Weather {
    Sun,
    HarshSun,
    Rain,
    HeavyRain,
    Sand,
    Snow,
    Hail,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Terrain {
    Electric,
    Grassy,
    Psychic,
    Misty,
}

#[derive(Debug, Clone, Default)]
pub struct SideConditions {
    // Entry hazards
    pub stealth_rock: bool,
    pub spikes: u8,       // 0-3 layers
    pub toxic_spikes: u8, // 0-2 layers
    pub sticky_web: bool,

    // Screens
    pub reflect: bool,
    pub light_screen: bool,
    pub aurora_veil: bool,

    // Other
    pub tailwind: bool,
}

impl GameState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_player_state(&self, player: &str) -> Option<&PlayerState> {
        if player.starts_with("p1") {
            Some(&self.p1)
        } else if player.starts_with("p2") {
            Some(&self.p2)
        } else {
            None
        }
    }

    pub fn get_player_state_mut(&mut self, player: &str) -> Option<&mut PlayerState> {
        if player.starts_with("p1") {
            Some(&mut self.p1)
        } else if player.starts_with("p2") {
            Some(&mut self.p2)
        } else {
            None
        }
    }
}

impl PlayerState {
    pub fn get_pokemon(&self, nickname: &str) -> Option<&PokemonState> {
        self.team.iter().find(|p| p.nickname == nickname)
    }

    pub fn get_pokemon_mut(&mut self, nickname: &str) -> Option<&mut PokemonState> {
        self.team.iter_mut().find(|p| p.nickname == nickname)
    }

    pub fn get_active_pokemon(&self) -> Option<&PokemonState> {
        self.active_pokemon
            .as_ref()
            .and_then(|nick| self.get_pokemon(nick))
    }

    pub fn get_active_pokemon_mut(&mut self) -> Option<&mut PokemonState> {
        let nick = self.active_pokemon.clone()?;
        self.get_pokemon_mut(&nick)
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
