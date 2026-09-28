use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum LuckCategory {
    CriticalHit,
    AccuracyMiss,
    SecondaryEffect,
    Flinch,
    StatusTurn,
    DamageRoll,
    AbilityProc,
    MultiHit,
    Other(String),
}

#[derive(Debug, Clone, Serialize)]
pub struct LuckEvent {
    pub turn: u32,
    pub pokemon: String,
    pub category: LuckCategory,
    pub score: f64,
    pub description: String,
    pub source_move: Option<String>,
    pub is_beneficial: bool,
}
