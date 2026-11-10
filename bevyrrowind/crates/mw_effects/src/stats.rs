//! The magnitude stats of slice A1 (and Silence and Sound, A5a) (effects-adapter.md "Vanilla effects by archetype"): one
//! base-0 `Value` stat per resist, weakness and elemental-shield effect, keyed by the effect's
//! own key (`resistfire`), group `magnitude`. Their defs live in the game's stat table,
//! `mw_stats::MAGNITUDE_STATS` (appended to `mw_stats::defs()`, `MwIdx::magnitudes`), so every
//! `MwTable` has them; this module names which MGEF effects they belong to.

/// MGEF indices whose magnitude is a stat in `mw_stats::MAGNITUDE_STATS`, in that list's order:
/// Fire, Lightning and Frost Shield (4-6), the nine Weakness effects (28-36), the ten Resist
/// effects (90-99), Silence (46) and Sound (48: the cast rules read both, A5a). Fortify Maximum
/// Magicka (84) writes `mw_stats`'s `fortifymaximummagicka`.
pub const MAGNITUDE_STAT_EFFECTS: [usize; 24] =
    [4, 5, 6, 28, 29, 30, 31, 32, 33, 34, 35, 36, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 46, 48];
