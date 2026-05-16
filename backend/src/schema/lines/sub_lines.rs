use crate::schema::lines::line_types::SubLineType;
use crate::schema::state::Status;

#[derive(Debug, Clone)]
pub struct SubLine {
    pub line_type: SubLineType,

    // Common fields (present in most subline types)
    pub player: Option<String>,
    pub pokemon_nickname: Option<String>,

    // HP change fields (-damage, -heal)
    pub new_hp: Option<u8>,
    pub max_hp: Option<u8>,

    // Stat change fields (-boost, -unboost)
    pub stat: Option<String>,
    pub amount: Option<u8>,

    // Item fields (-enditem)
    pub item: Option<String>,
    pub from: Option<String>,
    pub of: Option<String>,

    // Ability fields (-activate)
    pub ability: Option<String>,

    // Terastallize fields
    pub tera_type: Option<String>,

    // Mega fields
    pub species: Option<String>,
    pub mega_stone: Option<String>,

    // Status effect fields
    pub status: Option<Status>,
    
    // Fields related to moves
    pub target_player: Option<String>,
    pub target_pokemon_nickname: Option<String>,
}

// Helper to extract player + nickname
fn extract_pokemon(player_and_nick: &str) -> (String, String) {
    let mut split = player_and_nick.split(':');
    let player = split.next().unwrap_or("").to_string();
    let pokemon_nickname = split.next().unwrap_or("").trim().to_string();
    (player, pokemon_nickname)
}

fn extract_species(pokemon_string: &str) -> String {
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

impl SubLine {
    fn new_empty(line_type: SubLineType) -> Self {
        SubLine {
            line_type,
            player: None,
            pokemon_nickname: None,
            new_hp: None,
            max_hp: None,
            stat: None,
            amount: None,
            item: None,
            from: None,
            of: None,
            ability: None,
            tera_type: None,
            species: None,
            mega_stone: None,
            status: None,
            target_player: None,
            target_pokemon_nickname: None,
        }
    }

    // -damage: hit pokemon (p+n) | new hp
    pub fn from_damage(line: &str) -> Self {
        let mut split = line.split('|');
        split.next();
        split.next(); // skip "-damage"
        let (player, pokemon_nickname) = extract_pokemon(split.next().unwrap_or_default());
        let (new_hp, max_hp) = extract_hp(split.next().unwrap_or_default());

        let mut sub = Self::new_empty(SubLineType::Damage);
        sub.player = Some(player);
        sub.pokemon_nickname = Some(pokemon_nickname);
        sub.new_hp = Some(new_hp);
        sub.max_hp = Some(max_hp);
        sub
    }

    // -heal: targeted pokemon (p+n) | new hp
    pub fn from_heal(line: &str) -> Self {
        let mut split = line.split('|');
        split.next();
        split.next(); // skip "-heal"
        let (player, pokemon_nickname) = extract_pokemon(split.next().unwrap_or_default());
        let (new_hp, max_hp) = extract_hp(split.next().unwrap_or_default());

        let mut sub = Self::new_empty(SubLineType::Heal);
        sub.player = Some(player);
        sub.pokemon_nickname = Some(pokemon_nickname);
        sub.new_hp = Some(new_hp);
        sub.max_hp = Some(max_hp);
        sub
    }

    // -crit, -resisted, -supereffective, -immune: target of move (p+n)
    pub fn from_move_effectiveness(line: &str, line_type: SubLineType) -> Self {
        let mut split = line.split('|');
        split.next();
        split.next(); // skip line type marker
        let (player, pokemon_nickname) = extract_pokemon(split.next().unwrap_or_default());

        let mut sub = Self::new_empty(line_type);
        sub.player = Some(player);
        sub.pokemon_nickname = Some(pokemon_nickname);
        sub
    }

    // -boost / -unboost: target | stat | amount(1-6)
    pub fn from_stat_change(line: &str, is_boost: bool) -> Self {
        let mut split = line.split('|');
        split.next();
        split.next(); // skip "-boost" or "-unboost"
        let (player, pokemon_nickname) = extract_pokemon(split.next().unwrap_or_default());
        let stat = split.next().unwrap_or_default().trim().to_string();
        let amount = split
            .next()
            .unwrap_or("0")
            .trim()
            .parse::<u8>()
            .unwrap_or(0);

        let line_type = if is_boost {
            SubLineType::Boost
        } else {
            SubLineType::Unboost
        };

        let mut sub = Self::new_empty(line_type);
        sub.player = Some(player);
        sub.pokemon_nickname = Some(pokemon_nickname);
        sub.stat = Some(stat);
        sub.amount = Some(amount);
        sub
    }

    // -enditem: p+n | item | [from] | [of]
    pub fn from_enditem(line: &str) -> Self {
        let mut split = line.split('|');
        split.next();
        split.next(); // skip "-enditem"
        let (player, pokemon_nickname) = extract_pokemon(split.next().unwrap_or_default());
        let item = split.next().unwrap_or_default().trim().to_string();
        let from = split.next().map(|s| s.trim().to_string());
        let of = split.next().map(|s| s.trim().to_string());

        let mut sub = Self::new_empty(SubLineType::EndItem);
        sub.player = Some(player);
        sub.pokemon_nickname = Some(pokemon_nickname);
        sub.item = Some(item);
        sub.from = from;
        sub.of = of;
        sub
    }

    // -activate: p+n | ability
    pub fn from_activate(line: &str) -> Self {
        let mut split = line.split('|');
        split.next();
        split.next(); // skip "-activate"
        let (player, pokemon_nickname) = extract_pokemon(split.next().unwrap_or_default());
        let ability = split.next().unwrap_or_default().trim().to_string();

        let mut sub = Self::new_empty(SubLineType::Activate);
        sub.player = Some(player);
        sub.pokemon_nickname = Some(pokemon_nickname);
        sub.ability = Some(ability);
        sub
    }

    // -terastallize: p+n | type
    pub fn from_terastallize(line: &str) -> Self {
        let mut split = line.split('|');
        split.next();
        split.next(); // skip "-terastallize"
        let (player, pokemon_nickname) = extract_pokemon(split.next().unwrap_or_default());
        let tera_type = split.next().unwrap_or_default().trim().to_string();

        let mut sub = Self::new_empty(SubLineType::Terastallize);
        sub.player = Some(player);
        sub.pokemon_nickname = Some(pokemon_nickname);
        sub.tera_type = Some(tera_type);
        sub
    }

    // -mega: p+n | pokemon | item
    pub fn from_mega(line: &str) -> Self {
        let mut split = line.split('|');
        split.next();
        split.next(); // skip "-mega"
        let (player, pokemon_nickname) = extract_pokemon(split.next().unwrap_or_default());
        let species = extract_species(split.next().unwrap_or_default());
        let mega_stone = split.next().unwrap_or_default().trim().to_string();

        let mut sub = Self::new_empty(SubLineType::Mega);
        sub.player = Some(player);
        sub.pokemon_nickname = Some(pokemon_nickname);
        sub.species = Some(species);
        sub.mega_stone = Some(mega_stone);
        sub
    }

    pub fn from_miss(line: &str) -> Self {
        let mut split = line.split('|');
        split.next();
        split.next(); // skip "-miss"
        let (player, pokemon_nickname) = extract_pokemon(split.next().unwrap_or_default());
        let (target_player, target_pokemon_nickname) = extract_pokemon(split.next().unwrap_or_default());

        let mut sub = Self::new_empty(SubLineType::Miss);
        sub.player = Some(player);
        sub.pokemon_nickname = Some(pokemon_nickname);
        sub.target_player = Some(target_player);
        sub.target_pokemon_nickname = Some(target_pokemon_nickname);
        sub
    }

    pub fn from_status(line: &str) -> Self {
        let mut split = line.split('|');
        split.next();
        split.next(); // skip "-status"
        let (player, affected_pokemon) = extract_pokemon(split.next().unwrap_or_default());
        let status = Status::from_str(split.next().unwrap_or_default().trim());

        let mut sub = Self::new_empty(SubLineType::Status);
        sub.player = Some(player);
        sub.pokemon_nickname = Some(affected_pokemon);
        sub.status = status;
        sub
    }
}
