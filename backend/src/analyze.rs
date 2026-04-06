use crate::schema::lines::line_types::SubLineType;
use crate::schema::lines::main_lines::MainLine;
use crate::schema::lines::sub_lines::SubLine;

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

pub async fn analyze(lines: Vec<String>) {
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
