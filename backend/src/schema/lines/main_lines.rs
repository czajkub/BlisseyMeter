use crate::schema::lines::line_types::MainLineType;
use crate::schema::state::PokeSpecies;

pub struct MainLine {
    pub line_type: MainLineType,

    // Common fields (present in all line types)
    pub player: String,
    pub pokemon_nickname: String,

    // Switch-specific fields
    pub species: Option<PokeSpecies>,
    pub pokemon_current_hp: Option<u8>,
    pub pokemon_max_hp: Option<u8>,

    // Move-specific fields
    pub move_name: Option<String>,
    pub target_player: Option<String>,
    pub target_pokemon_nickname: Option<String>,
    // DetailsChange uses `species` for changed_form (reused)
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
    let gender = split.next().unwrap_or("").to_string().chars().next();

    PokeSpecies { species, gender }
}

fn extract_hp(hp_split: &str) -> (u8, u8) {
    let mut split = hp_split.split("/");
    let current_hp = split.next().unwrap().parse::<u8>().unwrap();
    let max_hp = split.next().unwrap().parse::<u8>().unwrap();
    (current_hp, max_hp)
}

impl MainLine {
    pub fn from_line(line: &str) -> Self {
        let mut split = line.split("|");
        let line_type = split.next().unwrap_or_default();
        match line_type {
            "s" => Self::from_switch(line),
            "m" => Self::from_move(line),
            "f" => Self::from_faint(line),
            "d" => Self::from_detailschange(line),
            _ => panic!("Invalid line type"),
        }
    }

    pub fn from_switch(line: &str) -> Self {
        let mut split = line.split("|");
        split.next();
        let (player, pokemon_nickname) = extract_pokemon(split.next().unwrap_or_default());
        let species = extract_pokemon_species(split.next().unwrap_or_default());
        let (pokemon_current_hp, pokemon_max_hp) = extract_hp(split.next().unwrap_or_default());

        MainLine {
            line_type: MainLineType::Switch,
            player,
            pokemon_nickname,
            species: Some(species),
            pokemon_current_hp: Some(pokemon_current_hp),
            pokemon_max_hp: Some(pokemon_max_hp),
            move_name: None,
            target_player: None,
            target_pokemon_nickname: None,
        }
    }

    pub fn from_move(line: &str) -> Self {
        let mut split = line.split("|");
        split.next();
        let (player, pokemon_nickname) = extract_pokemon(split.next().unwrap_or_default());
        let move_name = split.next().unwrap_or_default().to_string();
        let (target_player, target_pokemon_nickname) =
            extract_pokemon(split.next().unwrap_or_default());

        MainLine {
            line_type: MainLineType::Move,
            player,
            pokemon_nickname,
            species: None,
            pokemon_current_hp: None,
            pokemon_max_hp: None,
            move_name: Some(move_name),
            target_player: Some(target_player),
            target_pokemon_nickname: Some(target_pokemon_nickname),
        }
    }

    pub fn from_faint(line: &str) -> Self {
        let mut split = line.split("|");
        split.next();
        let (player, pokemon_nickname) = extract_pokemon(split.next().unwrap_or_default());

        MainLine {
            line_type: MainLineType::Faint,
            player,
            pokemon_nickname,
            species: None,
            pokemon_current_hp: None,
            pokemon_max_hp: None,
            move_name: None,
            target_player: None,
            target_pokemon_nickname: None,
        }
    }

    pub fn from_detailschange(line: &str) -> Self {
        let mut split = line.split("|");
        split.next();
        let (player, pokemon_nickname) = extract_pokemon(split.next().unwrap_or_default());
        let species = extract_pokemon_species(split.next().unwrap_or_default());

        MainLine {
            line_type: MainLineType::DetailsChange,
            player,
            pokemon_nickname,
            species: Some(species),
            pokemon_current_hp: None,
            pokemon_max_hp: None,
            move_name: None,
            target_player: None,
            target_pokemon_nickname: None,
        }
    }
}
