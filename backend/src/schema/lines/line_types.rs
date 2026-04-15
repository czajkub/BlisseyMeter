#[derive(Debug, Clone)]
pub enum MainLineType {
    Switch,
    Move,
    Faint,
    DetailsChange,
}

#[derive(Debug, Clone)]
pub enum SubLineType {
    // HP changes
    Damage,
    Heal,

    // Move effectiveness
    Crit,
    Resisted,
    SuperEffective,
    Immune,

    // Stat changes
    Boost,
    Unboost,

    // Item
    EndItem,

    // Ability
    Activate,

    // Forme changes
    Terastallize,
    Mega,
}

#[derive(Debug, Clone)]
pub enum InfoLineType {
    Gen,
    GameType,
    Tier,
    Rule,

    Start,
    Upkeep,
    Time,
    Turn,

    Win,
    Join,
    Chat,

    Player,
    TeamSize,
    Poke,
}
