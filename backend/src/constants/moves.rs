use std::collections::HashMap;
use std::fs::File;

#[derive(Debug, serde::Deserialize)]
struct Move {
    name: String,
    #[serde(rename = "type")]
    move_type: String,
    category: String,
    power: Option<u64>,
    accuracy: Option<u64>,
    pp: Option<u64>,
    effect: String,
    #[serde(rename = "secondaryEffect")]
    secondary_effect: Option<u64>,
}

pub fn moves() -> HashMap<String, Move> {
    let mut rdr = csv::Reader::from_reader(File::open("moves.csv").unwrap());
    let mut moves = HashMap::new();
    for result in rdr.deserialize() {
        let move_record: Move = result.unwrap();
        moves.insert(move_record.name.clone(), move_record);
    }
    moves
}
