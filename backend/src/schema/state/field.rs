#[derive(Debug, Clone, Default)]
pub struct FieldState {
    pub weather: Option<Weather>,
    pub terrain: Option<Terrain>,
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
    pub stealth_rock: bool,
    pub spikes: u8,
    pub toxic_spikes: u8,
    pub sticky_web: bool,
    pub reflect: bool,
    pub light_screen: bool,
    pub aurora_veil: bool,
    pub tailwind: bool,
}
