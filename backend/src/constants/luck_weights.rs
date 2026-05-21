use std::collections::HashMap;
use std::sync::LazyLock;

// uh, this is not used - but maybe sometime later
pub static BASE_CRIT_CHANCE_PER_GEN: LazyLock<HashMap<u8, f64>> = LazyLock::new(|| {
    HashMap::from([
        (1, f64::MAX), // workaround for gen1 - dependent on speed
        (2, 17.0/256.0),
        (3, 1.0/16.0),
        (4, 1.0/16.0),
        (5, 1.0/16.0),
        (6, 1.0/16.0),
        (7, 1.0/24.0),
        (8, 1.0/24.0),
        (9, 1.0/24.0),
    ])
});

pub const CRIT_WEIGHT: f64 = 1.0;
pub const MISS_WEIGHT: f64 = 1.5;
pub const SECONDARY_EFFECT_WEIGHT: f64 = 0.5;
pub const STATUS_WEIGHT: f64 = 1.0;

