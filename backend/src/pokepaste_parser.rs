use std::collections::HashSet;

use regex::Regex;

use crate::constants::moves;
use crate::schema::state::{EvSpread, IvSpread, PokemonSet};

#[derive(Debug, Clone)]
pub struct ParsedPokemon {
    pub nickname: Option<String>,
    pub species: String,
    pub set: PokemonSet,
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
            parse_ivs(&line["IVs: ".len()..])
        } else {
            None
        }
    } else {
        None
    };

    let mut move_set = HashSet::new();
    for name in next_line
        .into_iter()
        .chain(lines)
        .filter_map(|line| line.strip_prefix("- "))
        .map(str::trim)
    {
        if !moves::moves().contains_key(name) {
            return None;
        }
        move_set.insert(name.to_owned());
    }

    Some(ParsedPokemon {
        nickname,
        species,
        set: PokemonSet {
            evs: Some(evs),
            ivs,
            moves: move_set,
            item,
            ability: Some(ability),
            nature: Some(nature),
            has_pokepaste: true,
        },
    })
}

fn parse_evs(value: &str) -> Option<EvSpread> {
    let mut evs = EvSpread::default();

    for entry in value.split(" / ") {
        let mut parts = entry.split_whitespace();
        let amount = parts.next()?.parse::<u16>().ok()?;
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

fn parse_ivs(value: &str) -> Option<IvSpread> {
    let mut ivs = IvSpread {
        hp: 31,
        atk: 31,
        def: 31,
        spa: 31,
        spd: 31,
        spe: 31,
        acc: 0,
        eva: 0,
    };

    for entry in value.split(" / ") {
        let mut parts = entry.split_whitespace();
        let amount = parts.next()?.parse::<u8>().ok()?;
        let stat = parts.next()?;

        match stat {
            "HP" => ivs.hp = amount,
            "Atk" => ivs.atk = amount,
            "Def" => ivs.def = amount,
            "SpA" => ivs.spa = amount,
            "SpD" => ivs.spd = amount,
            "Spe" => ivs.spe = amount,
            _ => return None,
        }
    }

    Some(ivs)
}
