use crate::schema::lines::{
    Hp, InfoLine, Line, MainLine, MainLineKind, PlayerId, PokemonRef, SubLine,
};
use crate::schema::state::{GameState, Weather};

fn field<'a>(fields: &mut impl Iterator<Item = &'a str>) -> &'a str {
    fields.next().unwrap_or_default().trim()
}

fn parse_pokemon(value: &str) -> Option<PokemonRef> {
    let (player, nickname) = value.split_once(':')?;
    Some(PokemonRef {
        player: PlayerId::parse(player.trim())?,
        pokemon_nickname: nickname.trim().to_string(),
    })
}

fn parse_species(value: &str) -> String {
    value
        .split(',')
        .next()
        .unwrap_or_default()
        .trim()
        .to_string()
}

fn parse_hp(value: &str) -> Hp {
    let mut fields = value.split('/');
    Hp {
        current: fields.next().unwrap_or("0").parse().unwrap_or(0),
        max: fields.next().unwrap_or("100").parse().unwrap_or(100),
    }
}

fn parse_line(line: &str) -> Line {
    let mut fields = line.split('|');
    fields.next();
    let line_type = field(&mut fields);

    match line_type {
        "switch" => {
            let pokemon = parse_pokemon(field(&mut fields));
            match pokemon {
                Some(source_pokemon) => Line::Main(MainLine {
                    kind: MainLineKind::Switch {
                        source_pokemon,
                        species: parse_species(field(&mut fields)),
                        hp: parse_hp(field(&mut fields)),
                    },
                    sublines: Vec::new(),
                }),
                None => Line::Unknown,
            }
        }
        "move" => {
            let source_pokemon = parse_pokemon(field(&mut fields));
            let move_name = field(&mut fields).to_string();
            let target = parse_pokemon(field(&mut fields));
            match (source_pokemon, target) {
                (Some(source_pokemon), Some(target)) => Line::Main(MainLine {
                    kind: MainLineKind::Move {
                        source_pokemon,
                        move_name,
                        target,
                    },
                    sublines: Vec::new(),
                }),
                _ => Line::Unknown,
            }
        }
        "faint" => parse_pokemon(field(&mut fields))
            .map(|source_pokemon| {
                Line::Main(MainLine {
                    kind: MainLineKind::Faint { source_pokemon },
                    sublines: Vec::new(),
                })
            })
            .unwrap_or(Line::Unknown),
        "detailschange" | "-formechange" => parse_pokemon(field(&mut fields))
            .map(|source_pokemon| {
                Line::Main(MainLine {
                    kind: MainLineKind::DetailsChange {
                        source_pokemon,
                        new_form: parse_species(field(&mut fields)),
                    },
                    sublines: Vec::new(),
                })
            })
            .unwrap_or(Line::Unknown),
        "cant" | "-cant" => {
            let source_pokemon = parse_pokemon(field(&mut fields));
            let reason = field(&mut fields).to_string();
            let source = if reason == "flinch" {
                field(&mut fields)
                    .strip_prefix("[of]")
                    .and_then(parse_pokemon)
            } else {
                None
            };
            source_pokemon
                .map(|source_pokemon| {
                    Line::Main(MainLine {
                        kind: MainLineKind::Cant {
                            source_pokemon,
                            reason,
                            source,
                        },
                        sublines: Vec::new(),
                    })
                })
                .unwrap_or(Line::Unknown)
        }
        "-curestatus" => parse_pokemon(field(&mut fields))
            .map(|source_pokemon| {
                Line::Main(MainLine {
                    kind: MainLineKind::CureStatus {
                        source_pokemon,
                        cured_status: crate::schema::state::Status::from_str(field(&mut fields)),
                        meta: field(&mut fields).to_string(),
                    },
                    sublines: Vec::new(),
                })
            })
            .unwrap_or(Line::Unknown),
        "-weather" => Line::Main(MainLine {
            kind: MainLineKind::WeatherChange { 
                new_weather: Weather::from_log(field(&mut fields))
            },
            sublines: Vec::new(),
        }), 
        "turn" => Line::Info(InfoLine::Turn {
            turn: field(&mut fields).parse().unwrap_or(0),
        }),
        "poke" => {
            let player = field(&mut fields).to_string();
            let mut poke = field(&mut fields).split(',');
            Line::Info(InfoLine::Poke {
                player,
                species: poke.next().unwrap_or_default().trim().to_string(),
                gender: poke.next().unwrap_or_default().trim().to_string(),
            })
        }
        "player" => Line::Info(InfoLine::Player {
            player: non_empty(field(&mut fields)),
            name: non_empty(field(&mut fields)),
            avatar: non_empty(field(&mut fields)),
        }),
        "-damage" | "-heal" => {
            let target = parse_pokemon(field(&mut fields));
            target
                .map(|target| {
                    let hp = parse_hp(field(&mut fields));
                    let subline = if line_type == "-damage" {
                        SubLine::Damage {
                            target,
                            hp,
                            source: None,
                        }
                    } else {
                        SubLine::Heal { target, hp }
                    };
                    Line::Sub(subline)
                })
                .unwrap_or(Line::Unknown)
        }
        "-status" => parse_pokemon(field(&mut fields))
            .map(|target| {
                Line::Sub(SubLine::Status {
                    target,
                    status: crate::schema::state::Status::from_str(field(&mut fields)),
                    from: fields.next().map(|value| {
                        value
                            .trim()
                            .strip_prefix("[from]")
                            .unwrap_or(value.trim())
                            .trim()
                            .to_string()
                    }),
                })
            })
            .unwrap_or(Line::Unknown),
        "-miss" => {
            let source = parse_pokemon(field(&mut fields));
            let target = parse_pokemon(field(&mut fields));
            match (source, target) {
                (Some(source), Some(target)) => Line::Sub(SubLine::Miss { source, target }),
                _ => Line::Unknown,
            }
        }
        "-crit" | "-resisted" | "-supereffective" | "-immune" => parse_pokemon(field(&mut fields))
            .map(|target| {
                Line::Sub(match line_type {
                    "-crit" => SubLine::Crit { target },
                    "-resisted" => SubLine::Resisted { target },
                    "-supereffective" => SubLine::SuperEffective { target },
                    _ => SubLine::Immune { target },
                })
            })
            .unwrap_or(Line::Unknown),
        "-boost" | "-unboost" => parse_pokemon(field(&mut fields))
            .map(|target| {
                let stat = field(&mut fields).to_string();
                let amount: i8 = field(&mut fields).parse().unwrap_or(0);
                let subline = if line_type == "-boost" {
                    SubLine::Boost {
                        target,
                        stat,
                        amount,
                    }
                } else {
                    SubLine::Unboost {
                        target,
                        stat,
                        amount,
                    }
                };
                Line::Sub(subline)
            })
            .unwrap_or(Line::Unknown),
        "-enditem" => parse_pokemon(field(&mut fields))
            .map(|target| {
                Line::Sub(SubLine::EndItem {
                    target,
                    item: field(&mut fields).to_string(),
                    from: fields.next().map(|value| value.trim().to_string()),
                    of: fields.next().map(|value| value.trim().to_string()),
                })
            })
            .unwrap_or(Line::Unknown),
        "-activate" => parse_pokemon(field(&mut fields))
            .map(|target| {
                Line::Sub(SubLine::Activate {
                    target,
                    ability: field(&mut fields).to_string(),
                })
            })
            .unwrap_or(Line::Unknown),
        "-terastallize" => parse_pokemon(field(&mut fields))
            .map(|target| {
                Line::Sub(SubLine::Terastallize {
                    target,
                    tera_type: field(&mut fields).to_string(),
                })
            })
            .unwrap_or(Line::Unknown),
        "-mega" => parse_pokemon(field(&mut fields))
            .map(|target| {
                Line::Sub(SubLine::Mega {
                    target,
                    species: parse_species(field(&mut fields)),
                    mega_stone: field(&mut fields).to_string(),
                })
            })
            .unwrap_or(Line::Unknown),
        _ => Line::Unknown,
    }
}

fn non_empty(value: &str) -> Option<String> {
    (!value.is_empty()).then(|| value.to_string())
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
            Line::Info(info_line) => {
                parsed_lines.push(Line::Info(info_line));
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
                crate::handlers::main_handlers::handle_main_line(&mut game_state, &main_line);
            }
            Line::Sub(_sub_line) => {
                // Handle sub line
                // TODO: process sub line
            }
            Line::Info(InfoLine::Turn { turn }) => game_state.turn = turn,
            Line::Info(info_line) => {
                crate::handlers::info_handlers::handle_info_line(&mut game_state, &info_line)
            }
            Line::Unknown => {
                // Skip unknown lines or log them
            }
        }
    }

    game_state
}
