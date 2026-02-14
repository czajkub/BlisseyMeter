struct HPChangeLine {
    affected_pokemon: String,
    new_hp: u8,
}

struct MoveInfoLine {
    target_of_move: String,
}

struct StatChangeLine {
    affected_pokemon: String,
    stat: String,
    value: i8,
}

struct FormeChangeLine {
    affected_pokemon: String,
    tera_type: String,
    pokemon_nick: String,
    mega_item: String,
}

struct AbilityActivationLine {
    affected_pokemon: String,
    ability: String,
}

struct ItemActivationLine {
    affected_pokemon: String,
    item: String,
    from: String,
    source: String,
}