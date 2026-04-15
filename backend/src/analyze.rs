use crate::schema::lines::line_types::SubLineType;
use crate::schema::lines::main_lines::MainLine;
use crate::schema::lines::sub_lines::SubLine;
use crate::schema::state::GameState;

#[derive(Debug, Clone)]
pub enum Line {
    Main(MainLine),
    Sub(SubLine),
    // Info(InfoLine), // TODO: implement later
    Unknown,
}

fn parse_line(line: &str) -> Line {
    // Lines start with | so first split element is empty
    let line_type = line.split('|').nth(1).unwrap_or_default().trim();

    match line_type {
        // Main lines
        "switch" => Line::Main(MainLine::from_switch(line)),
        "move" => Line::Main(MainLine::from_move(line)),
        "faint" => Line::Main(MainLine::from_faint(line)),
        "detailschange" | "-formechange" => Line::Main(MainLine::from_detailschange(line)),

        // Sub lines - HP changes
        "-damage" => Line::Sub(SubLine::from_damage(line)),
        "-heal" => Line::Sub(SubLine::from_heal(line)),

        // Sub lines - Move effectiveness
        "-crit" => Line::Sub(SubLine::from_move_effectiveness(line, SubLineType::Crit)),
        "-resisted" => Line::Sub(SubLine::from_move_effectiveness(line, SubLineType::Resisted)),
        "-supereffective" => {
            Line::Sub(SubLine::from_move_effectiveness(line, SubLineType::SuperEffective))
        }
        "-immune" => Line::Sub(SubLine::from_move_effectiveness(line, SubLineType::Immune)),

        // Sub lines - Stat changes
        "-boost" => Line::Sub(SubLine::from_stat_change(line, true)),
        "-unboost" => Line::Sub(SubLine::from_stat_change(line, false)),

        // Sub lines - Items
        "-enditem" => Line::Sub(SubLine::from_enditem(line)),

        // Sub lines - Ability
        "-activate" => Line::Sub(SubLine::from_activate(line)),

        // Sub lines - Forme changes
        "-terastallize" => Line::Sub(SubLine::from_terastallize(line)),
        "-mega" => Line::Sub(SubLine::from_mega(line)),

        // Unknown/unhandled line types
        _ => Line::Unknown,
    }
}

fn parse_game_lines(lines: Vec<String>) -> Vec<Line> {
    let mut parsed_lines = Vec::new();
    let mut last_main_line: Option<MainLine> = None;
    for line in lines {
        let parsed = parse_line(&line);
        match parsed {
            Line::Main(main_line) => {
                last_main_line = Some(main_line.clone());
                parsed_lines.push(Line::Main(main_line));
            }
            Line::Sub(sub_line) => {
                if let Some(last_main_line) = last_main_line.as_mut() {
                    last_main_line.sublines.push(sub_line);
                }
            }
            Line::Unknown => {
                // Skip unknown lines or log them
            }
        }
    }
    parsed_lines
}

pub async fn analyze(lines: Vec<String>) {
    let mut game_state = GameState::default();

    for line in &lines {
        let parsed = parse_line(line);

        match parsed {
            Line::Main(main_line) => {
                // Handle main line
                // TODO: process main line
            }
            Line::Sub(sub_line) => {
                // Handle sub line
                // TODO: process sub line
            }
            Line::Unknown => {
                // Skip unknown lines or log them
            }
        }
    }
}
