use crate::schema::lines::line_types::MainLineType;
use crate::schema::lines::sub_lines::SubLine;

#[derive(Debug, Clone)]
pub struct MainLine {
    pub line_type: MainLineType,

    // Common fields (present in all line types)
    pub player: String,
    pub pokemon_nickname: String,

    // Switch-specific fields
    pub species: Option<String>,
    pub pokemon_current_hp: Option<u8>,
    pub pokemon_max_hp: Option<u8>,

    // Move-specific fields
    pub move_name: Option<String>,
    pub target_player: Option<String>,
    pub target_pokemon_nickname: Option<String>,
    // DetailsChange uses `species` for changed_form (reused)

    // Cant-specific fields
    pub reason: Option<String>,

    pub sublines: Vec<SubLine>,
}

// player + nick - so like p1a: N95; p2a: Pecharunt
fn extract_pokemon(player_and_nick: &str) -> (String, String) {
    let mut split = player_and_nick.split(':');
    let player = split.next().unwrap_or("").to_string();
    let pokemon_nickname = split.next().unwrap_or("").trim().to_string();
    (player, pokemon_nickname)
}

fn extract_species(pokemon_string: &str) -> String {
    // Species string may have ", M" or ", F" suffix for gender - just take species
    pokemon_string
        .split(',')
        .next()
        .unwrap_or("")
        .trim()
        .to_string()
}

fn extract_hp(hp_split: &str) -> (u8, u8) {
    let mut split = hp_split.split('/');
    let current_hp = split.next().unwrap_or("0").parse::<u8>().unwrap_or(0);
    let max_hp = split.next().unwrap_or("100").parse::<u8>().unwrap_or(100);
    (current_hp, max_hp)
}

impl MainLine {
    pub fn from_switch(line: &str) -> Self {
        let mut split = line.split('|');
        split.next();
        split.next(); // skip "switch"
        let (player, pokemon_nickname) = extract_pokemon(split.next().unwrap_or_default());
        let species = extract_species(split.next().unwrap_or_default());
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
            reason: None,
            sublines: Vec::new(),
        }
    }

    pub fn from_move(line: &str) -> Self {
        let mut split = line.split('|');
        split.next();
        split.next(); // skip "move"
        let (player, pokemon_nickname) = extract_pokemon(split.next().unwrap_or_default());
        let move_name = split.next().unwrap_or_default().trim().to_string();
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
            reason: None,
            sublines: Vec::new(),
        }
    }

    pub fn from_faint(line: &str) -> Self {
        let mut split = line.split('|');
        split.next();
        split.next(); // skip "faint"
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
            reason: None,
            sublines: Vec::new(),
        }
    }

    pub fn from_detailschange(line: &str) -> Self {
        let mut split = line.split('|');
        split.next();
        split.next(); // skip "detailschange"
        let (player, pokemon_nickname) = extract_pokemon(split.next().unwrap_or_default());
        let species = extract_species(split.next().unwrap_or_default());

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
            reason: None,
            sublines: Vec::new(),
        }
    }

    pub fn from_curestatus(line: &str) -> Self {
        let mut split = line.split('|');
        split.next();
        split.next(); // skip "-curestatus"
        let (player, pokemon_nickname) = extract_pokemon(split.next().unwrap_or_default());
        let status = split.next().unwrap_or_default().trim().to_string();

        MainLine {
            line_type: MainLineType::CureStatus,
            player,
            pokemon_nickname,
            species: None,
            pokemon_current_hp: None,
            pokemon_max_hp: None,
            move_name: None,
            target_player: None,
            target_pokemon_nickname: None,
            reason: Some(status),
            sublines: Vec::new(),
        }
    }

    pub fn from_cant(line: &str) -> Self {
        let mut split = line.split('|');
        split.next();
        split.next(); // skip "cant" or "-cant"
        let (player, pokemon_nickname) = extract_pokemon(split.next().unwrap_or_default());
        let reason = split.next().unwrap_or_default().trim().to_string();
        let (target_player, target_pokemon_nickname) = if reason == "flinch" {
            // For flinch cant, we can also extract the opponent or source move if present
            let of_part = split.next().unwrap_or_default().trim();
            if of_part.starts_with("[of]") {
                let of_split = of_part.strip_prefix("[of]").unwrap_or_default().trim();
                let (opp_player, opp_nick) = extract_pokemon(of_split);
                (Some(opp_player), Some(opp_nick))
            } else {
                (None, None)
            }
        } else {
            (None, None)
        };

        MainLine {
            line_type: MainLineType::Cant,
            player,
            pokemon_nickname,
            species: None,
            pokemon_current_hp: None,
            pokemon_max_hp: None,
            move_name: None,
            target_player,
            target_pokemon_nickname,
            reason: Some(reason),
            sublines: Vec::new(),
        }
    }
}
