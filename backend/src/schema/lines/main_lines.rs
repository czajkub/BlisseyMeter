use crate::schema::pokemon::PokeSpecies;

pub struct SwitchLine {
    player: String,
    pokemon_nickname: String,
    species: PokeSpecies,
    pokemon_current_hp: u8,
    pokemon_max_hp: u8,
}

pub struct MoveLine {
    player: String,
    pokemon_nickname: String,
    move_name: String,
    target_player: String,
    target_pokemon_nickname: String,
}

pub struct FaintLine {
    player: String,
    pokemon_nickname: String,
}

pub struct DetailsChangeLine {
    player: String,
    pokemon_nickname: String,
    changed_form: PokeSpecies,
}


// player + nick - so like p1a: N95; p2a: Pecharunt
fn extract_pokemon(player_and_nick: &str) -> (String, String) {
    let mut split = player_and_nick.split(":");
    let player = split.next().unwrap_or("").to_string();
    let pokemon_nickname = split.next().unwrap_or("").to_string();
    (player, pokemon_nickname)
}

fn extract_pokemon_species(pokemon_string: &str) -> PokeSpecies {
    let mut split = pokemon_string.split(",");
    let species = split.next().unwrap_or("").to_string();
    let gender = split.next()
        .unwrap_or("")
        .to_string()
        .chars().next();
    
    PokeSpecies {
        species,
        gender,
    }
}

fn extract_hp(hp_split: &str) -> (u8, u8) {
    let mut split = hp_split.split("/");
    let current_hp = split.next().unwrap().parse::<u8>().unwrap();
    let max_hp = split.next().unwrap().parse::<u8>().unwrap();
    (current_hp, max_hp)
}

impl SwitchLine {
    fn from_line_string(line: &str) -> Self {
        let mut split = line.split("|");
        split.next();
        let (player, pokemon_nickname) = extract_pokemon(split.next().unwrap_or_default());
        let species = extract_pokemon_species(split.next().unwrap_or_default());
        let (pokemon_current_hp, pokemon_max_hp) = extract_hp(split.next().unwrap_or_default());

        SwitchLine {
            player,
            pokemon_nickname,
            species,
            pokemon_current_hp,
            pokemon_max_hp,
        }
    }
}