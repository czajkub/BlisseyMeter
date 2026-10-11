use crate::constants::moves::Move;
use crate::models::lines::PlayerId;

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

    pub fn update_side_condition(&mut self, player: PlayerId, condition: &SideCondition, start: bool) {
        let side = match player {
            PlayerId::P1 => &mut self.p1_side,
            PlayerId::P2 => &mut self.p2_side,
        };
        side.apply(condition, start);
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

#[derive(Debug, Clone)]
pub enum SideCondition {
    StealthRock,
    Spikes,
    ToxicSpikes,
    StickyWeb,
    Reflect,
    LightScreen,
    AuroraVeil,
    Tailwind,
}

impl SideCondition {
    pub fn from_move(_move: Option<&Move>) -> Option<Self> {
        match _move?.name.as_str() {
            "Stealth Rock" => { return Some(SideCondition::StealthRock) }
            "Spikes" => { return Some(SideCondition::Spikes) }
            "Toxic Spikes" => { return Some(SideCondition::ToxicSpikes) }
            "Sticky Web" => { return Some(SideCondition::StickyWeb) }
            "Reflect" => { return Some(SideCondition::Reflect) }
            "Light Screen" => { return Some(SideCondition::LightScreen) }
            "Aurora Veil" => { return Some(SideCondition::AuroraVeil) }
            "Tailwind" => { return Some(SideCondition::Tailwind) }
            _ => { return None }
        }
    }
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

impl SideConditions {
    pub fn apply(&mut self, condition: &SideCondition, start: bool) {
        match condition {
            SideCondition::Spikes => {
                match start {
                    true => self.spikes = (self.spikes+1).clamp(0, 3),
                    false => self.spikes = 0
                }
            }
            SideCondition::ToxicSpikes => {
                match start {
                    true => self.toxic_spikes = (self.toxic_spikes+1).clamp(0, 2),
                    false => self.toxic_spikes = 0
                }
            }
            SideCondition::StealthRock => self.stealth_rock = start,
            SideCondition::StickyWeb => self.sticky_web = start,
            SideCondition::Reflect => self.reflect = start,
            SideCondition::LightScreen => self.light_screen = start,
            SideCondition::AuroraVeil => self.aurora_veil = start,
            SideCondition::Tailwind => self.tailwind = start,
        }
        
    }
}
