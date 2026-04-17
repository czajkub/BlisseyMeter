use std::collections::HashMap;
use std::fs::File;

#[derive(Debug, serde::Deserialize)]
pub struct Move {
    pub name: String,
    #[serde(rename = "type")]
    pub move_type: String,
    pub category: String,
    pub power: Option<u64>,
    pub accuracy: Option<u64>,
    pub pp: Option<u64>,
    pub effect: String,
    #[serde(rename = "secondaryEffect")]
    pub secondary_effect: Option<u64>,
}

pub fn moves() -> HashMap<String, Move> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(File::open("moves.csv").unwrap());
    let mut moves = HashMap::new();
    for result in rdr.deserialize() {
        let move_record: Move = result.unwrap();
        moves.insert(move_record.name.clone(), move_record);
    }
    moves
}
