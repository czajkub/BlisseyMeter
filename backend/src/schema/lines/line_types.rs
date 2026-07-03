#[derive(Debug, Clone, PartialEq)]
pub enum MainLineType {
    Switch,
    Move,
    Faint,
    DetailsChange,
    Cant,
    CureStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SubLineType {
    // HP changes
    Damage,
    Heal,

    // Move effectiveness
    Resisted,
    SuperEffective,
    Immune,

    // Luck factors
    Crit,
    Miss,
    Status,

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

#[derive(Debug, Clone, PartialEq)]
pub enum InfoLineType {
    Gen,
    GameType,
    Tier,
    Rule,

    Start,
    Upkeep,
    Time,
    Turn,
    Cant,

    Win,
    Join,
    Chat,

    Player,
    TeamSize,
    Poke,
}
