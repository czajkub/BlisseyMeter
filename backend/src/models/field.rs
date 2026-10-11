use crate::constants::moves::Move;

#[derive(Debug, Clone, Default)]
pub struct FieldState {
    pub weather: Option<Weather>,
    pub field_modifiers: FieldModifiers,
    pub p1_side: SideConditions,
    pub p2_side: SideConditions,
}

impl FieldState {
    pub fn set_new_weather(&mut self, new_weather: &Weather) {
        self.weather = Some(*new_weather);
    }

    pub fn update_field_modifier(&mut self, field_modifier: &FieldModifier, start: bool) {
        match (field_modifier, start) {
            (FieldModifier::ElectricTerrain, true) => { self.field_modifiers.terrain = Some(Terrain::Electric) }
            (FieldModifier::GrassyTerrain, true) => { self.field_modifiers.terrain = Some(Terrain::Grassy) }
            (FieldModifier::PsychicTerrain, true) => { self.field_modifiers.terrain = Some(Terrain::Psychic) }
            (FieldModifier::MistyTerrain, true) => { self.field_modifiers.terrain = Some(Terrain::Misty) }
            (FieldModifier::TrickRoom, start) => { self.field_modifiers.trick_room = start }
            (FieldModifier::Gravity, start) => { self.field_modifiers.gravity = start }
            (FieldModifier::MagicRoom, start) => { self.field_modifiers.magic_room = start }
            (FieldModifier::WonderRoom, start) => { self.field_modifiers.wonder_room = start }
            (_, false) => { self.field_modifiers.terrain = None }
        }
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

/*
 * When weather is better implemented
 * its turn count also needs to be added
 * and incremented in upkeep processing
 */
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

#[derive(Debug, Clone)]
pub enum FieldModifier {
    ElectricTerrain,
    GrassyTerrain,
    PsychicTerrain,
    MistyTerrain,
    TrickRoom,
    MagicRoom,
    WonderRoom,
    Gravity,
}

impl FieldModifier {
    pub fn from_move(_move: Option<&Move>) -> Option<Self> {
        match _move?.name.as_str() {
            "Electric Terrain" => { return Some(FieldModifier::ElectricTerrain) }
            "Grassy Terrain" => { return Some(FieldModifier::GrassyTerrain) }
            "Psychic Terrain" => { return Some(FieldModifier::PsychicTerrain) }
            "Misty Terrain" => { return Some(FieldModifier::MistyTerrain) }
            "Trick Room" => { return Some(FieldModifier::TrickRoom) }
            "Magic Room" => { return Some(FieldModifier::MagicRoom) }
            "Wonder Room" => { return Some(FieldModifier::WonderRoom) }
            "Gravity" => { return Some(FieldModifier::Gravity) }
            _ => { return None }
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FieldModifiers {
    terrain: Option<Terrain>,
    trick_room: bool,
    magic_room: bool,
    wonder_room: bool,
    gravity: bool,
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
