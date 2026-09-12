use regex::Regex;

use crate::constants::moves::{self, Move};

#[derive(Debug, Clone)]
pub struct ParsedPokemon {
    pub nickname: Option<String>,
    pub species: String,
    pub item: Option<String>,
    pub ability: String,
    pub evs: EVs,
    pub nature: String,
    pub ivs: Option<String>,
    pub moves: Vec<Move>,
}

#[derive(Debug, Clone, Default)]
pub struct EVs {
    pub hp: u8,
    pub atk: u8,
    pub def: u8,
    pub spa: u8,
    pub spd: u8,
    pub spe: u8,
}

pub fn parse_pokepastes(
    p1_pokepaste: Option<String>,
    p2_pokepaste: Option<String>,
) -> (Vec<ParsedPokemon>, Vec<ParsedPokemon>) {
    (parse_pokepaste(p1_pokepaste), parse_pokepaste(p2_pokepaste))
}

fn parse_pokepaste(pokepaste: Option<String>) -> Vec<ParsedPokemon> {
    pokepaste
        .as_deref()
        .map(|paste| paste.split("\n\n").filter_map(parse_pokemon).collect())
        .unwrap_or_default()
}

fn parse_pokemon(block: &str) -> Option<ParsedPokemon> {
    let mut lines = block.lines().map(str::trim_end);
    let header = lines.next()?;
    let header_re = Regex::new(
        r"^(?<nickname>.*) (?:\((?<species>(?![MF]\))[^()\r\n]+)\))?(?:\s+\([MF]\))? @ (?<item>.*)$",
    )
    .ok()?;
    let captures = header_re.captures(header)?;

    let raw_nickname = captures.name("nickname")?.as_str().trim();
    let species = captures
        .name("species")
        .map(|value| value.as_str().trim().to_owned())
        .unwrap_or_else(|| raw_nickname.to_owned());
    let nickname = captures
        .name("species")
        .filter(|_| !raw_nickname.is_empty())
        .map(|_| raw_nickname.to_owned());
    let item = captures
        .name("item")
        .map(|value| value.as_str().trim())
        .filter(|value| !value.is_empty())
        .map(str::to_owned);

    let ability = lines.next()?.strip_prefix("Ability: ")?.to_owned();
    let evs = parse_evs(lines.next()?.strip_prefix("EVs: ")?)?;
    let nature = lines.next()?.strip_suffix(" Nature")?.to_owned();

    let mut next_line = lines.next();
    let ivs = if let Some(line) = next_line {
        if line.starts_with("IVs: ") {
            next_line = lines.next();
            Some(line["IVs: ".len()..].to_owned())
        } else {
            None
        }
    } else {
        None
    };

    let moves = next_line
        .into_iter()
        .chain(lines)
        .filter_map(|line| line.strip_prefix("- "))
        .map(str::trim)
        .map(|name| moves::moves().get(name).cloned())
        .collect::<Option<Vec<_>>>()?;

    Some(ParsedPokemon {
        nickname,
        species,
        item,
        ability,
        evs,
        nature,
        ivs,
        moves,
    })
}

fn parse_evs(value: &str) -> Option<EVs> {
    let mut evs = EVs::default();

    for entry in value.split(" / ") {
        let mut parts = entry.split_whitespace();
        let amount = parts.next()?.parse::<u8>().ok()?;
        let stat = parts.next()?;

        match stat {
            "HP" => evs.hp = amount,
            "Atk" => evs.atk = amount,
            "Def" => evs.def = amount,
            "SpA" => evs.spa = amount,
            "SpD" => evs.spd = amount,
            "Spe" => evs.spe = amount,
            _ => return None,
        }
    }

    Some(evs)
}
