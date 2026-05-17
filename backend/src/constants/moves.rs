use std::collections::HashMap;
use std::sync::LazyLock;

#[derive(Debug, serde::Deserialize, Clone)]
pub struct Move {
    pub name: String,
    #[serde(rename = "type")]
    pub move_type: String,
    pub category: String,
    pub power: Option<String>,
    pub accuracy: Option<String>,
    pub pp: Option<u64>,
    pub effect: String,
    #[serde(rename = "secondaryEffect")]
    pub secondary_effect: Option<u64>,
}

impl Move {
    pub fn get_accuracy(&self) -> u64 {
        match &self.accuracy {
            Some(acc) if acc == "∞" || acc == "999" => 100,
            Some(acc) => acc.parse::<u64>().unwrap_or(100),
            None => 100,
        }
    }
}

/// Statics and constants/move_details.csv are baked into the binary via include_str!
/// for reliability in serverless (Lambda) and cross-platform environments.
pub static MOVES: LazyLock<HashMap<String, Move>> = LazyLock::new(|| {
    let csv_data = include_str!("move_details.csv");
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .quoting(true)
        .from_reader(csv_data.as_bytes());
    
    let mut moves = HashMap::new();
    for result in rdr.deserialize() {
        let move_record: Move = result.expect("Failed to parse move_details.csv");
        moves.insert(move_record.name.clone(), move_record);
    }
    moves
});

pub fn moves() -> &'static HashMap<String, Move> {
    &MOVES
}
