use std::collections::HashMap;
use std::sync::LazyLock;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BaseStatSpread {
    pub atk: u8,
    pub def: u8,
    pub hp: u8,
    pub spa: u8,
    pub spd: u8,
    pub spe: u8,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Species {
    pub abilities: HashMap<String, String>,
    pub base_species: Option<String>,
    pub base_stats: BaseStatSpread,
    pub forme: Option<String>,
    pub is_nonstandard: Option<String>,
    pub name: String,
    pub num: i16,
    pub types: Vec<String>,
    pub weightkg: f32,
}

pub static SPECIES: LazyLock<HashMap<String, Species>> = LazyLock::new(|| {
    serde_json::from_str(include_str!("data/species.json"))
        .expect("Failed to parse bundled species.json")
});

pub fn species() -> &'static HashMap<String, Species> {
    &SPECIES
}

pub fn to_id(input: &str) -> String {
    input
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect()
}

pub fn get_species(name: &str) -> Option<&'static Species> {
    SPECIES.get(&to_id(name))
}


#[derive(Debug, Clone, Deserialize)]
pub struct Nature {
    pub name: String,
    pub minus: String,
    pub plus: String,
}

pub static NATURES: LazyLock<HashMap<String, Species>> = LazyLock::new(|| {
    serde_json::from_str(include_str!("data/natures.json"))
        .expect("Failed to parse bundled natures.json")
});

pub fn natures() -> &'static HashMap<String, Species> {
    &NATURES
}