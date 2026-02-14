enum MainLineType {
    SWITCH,
    MOVE,
    FAINT,
    DETAILSCHANGE,
}

enum SubLineType {
    HPCHANGE,
    MOVEINFO,
    STATCHANGE,
    FORMECHANGE,
    ABILITY_ACTIVATION,
    ITEM_ACTIVATION,
}

enum InfoLineType {
    GEN,
    GAMETYPE,
    TIER,
    RULE,

    START,
    UPKEEP,
    TIME,
    TURN,

    WIN,
    JOIN,
    CHAT,

    PLAYER,
    TEAMSIZE,
    POKE,
}

enum LineType {
    MainLineType,
    SubLineType,
    InfoLineType,
}
