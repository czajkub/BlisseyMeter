use crate::handlers::main_handlers::handle_main_line;
use crate::schema::lines::line_types::SubLineType;
use crate::schema::lines::main_lines::MainLine;
use crate::schema::lines::sub_lines::SubLine;
use crate::schema::state::GameState;
use crate::handlers::main_handlers::handle_move::handle_move;

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

        // Sub lines - Possible luck factors
        "-status" => Line::Sub(SubLine::from_status(line)),
        "-miss" => Line::Sub(SubLine::from_miss(line)),
        "-crit" => Line::Sub(SubLine::from_move_effectiveness(line, SubLineType::Crit)),

        // Sub lines - Move effectiveness
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
    let mut last_main_line_idx: Option<usize> = None;

    for line in lines {
        let parsed = parse_line(&line);
        match parsed {
            Line::Main(main_line) => {
                last_main_line_idx = Some(parsed_lines.len());
                parsed_lines.push(Line::Main(main_line));
            }
            Line::Sub(sub_line) => {
                if let Some(idx) = last_main_line_idx {
                    if let Line::Main(last_main_line) = &mut parsed_lines[idx] {
                        last_main_line.sublines.push(sub_line);
                    }
                } else {
                    // If we encounter a subline before any mainline, just push it (or ignore)
                    parsed_lines.push(Line::Sub(sub_line));
                }
            }
            Line::Unknown => {
                // Skip unknown lines or log them
            }
        }
    }
    parsed_lines
}

pub async fn analyze(lines: Vec<String>) -> GameState {
    let mut game_state = GameState::default();
    let game_lines = parse_game_lines(lines);

    for line in game_lines {
        match line {
            Line::Main(main_line) => {
                handle_main_line(&mut game_state, &main_line);
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

    game_state
}
