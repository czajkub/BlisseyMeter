use crate::schema::lines::line_types::InfoLineType;

#[derive(Debug, Clone)]
pub struct InfoLine {
    pub line_type: InfoLineType,
    pub turn: Option<u32>,
    pub player: Option<String>,
    pub poke: Option<String>,
    pub gender: Option<String>,
}

impl InfoLine {
    pub fn from_turn(line: &str) -> Self {
        let mut split = line.split('|');
        split.next();
        split.next(); // skip "turn"
        let turn = split.next().unwrap_or_default().parse::<u32>().unwrap_or(0);

        InfoLine {
            line_type: InfoLineType::Turn,
            turn: Some(turn),
            player: None,
            poke: None,
            gender: None,
        }
    }

    pub fn from_poke(line: &str) -> Self {
        let mut split = line.split('|');
        split.next();
        split.next(); // skip "poke"
        let player = split.next().unwrap_or_default().to_string();
        let mut poke_split = split.next().unwrap_or_default().split(',');
        let poke = poke_split.next().unwrap_or_default().to_string();
        let gender = poke_split.next().unwrap_or_default().to_string();

        InfoLine {
            line_type: InfoLineType::Poke,
            turn: None,
            player: Some(player),
            poke: Some(poke),
            gender: Some(gender),
        }
    }
}
