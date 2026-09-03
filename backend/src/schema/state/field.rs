#[derive(Debug, Clone, Default)]
pub struct FieldState {
    pub weather: Option<Weather>,
    pub terrain: Option<Terrain>,
    pub p1_side: SideConditions,
    pub p2_side: SideConditions,
}

impl FieldState {
    pub fn set_new_weather(&mut self, new_weather: &Weather) {
        self.weather = Some(*new_weather);
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub enum Weather {
    #[default]
    None,
    Sun,
    HarshSun,
    Rain,
    HeavyRain,
    Sand,
    Snow,
    Hail,
    DeltaStream,
}

impl Weather {
    pub fn from_log(value: &str) -> Self {
        match value {
            "none" => Weather::None,
            "RainDance" => Weather::Rain,
            "PrimordialSea" => Weather::HeavyRain,
            "SunnyDay" => Weather::Sun,
            "DesolateLand" => Weather::HarshSun,
            "Sandstorm" => Weather::Sand,
            "DeltaStream" => Weather::DeltaStream,
            "Snowscape" => Weather::Snow,
            "Hail" => Weather::Hail,
            &_ => Weather::None
        }
    }
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
