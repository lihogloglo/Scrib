//! Morrowind's magic effects on `sf_effects` (`docs/specs/effects-adapter.md`): the 143 MGEF
//! effects as [`sf_effects::EffectDef`]s ([`MwEffects`]: `OpenMW` key names, hardcoded flags,
//! resistances and weaknesses, school, cost, presentation), which effects write the magnitude
//! stats of `mw_stats::MAGNITUDE_STATS` ([`MAGNITUDE_STAT_EFFECTS`]), and SPEL / ALCH records as effect sources
//! ([`map_spell`], [`map_potion`], [`MwSource`]), the SPEL source table ([`source_table`]) and
//! the spells a character starts with ([`start_spells`]), and the casting rules behind the
//! framework's cast seam ([`OpenMwEffectRules`] classic, [`MwModernEffectRules`] modern, A5a).
//! Bevy-free.
//!
//! Slice A1: the archetypes of the first effect set (fortify / drain / damage / restore of
//! attributes, skills and the three pools, Fortify Maximum Magicka, elemental damage and poison,
//! every Resist / Weakness and the three elemental shields as magnitude stats). The other MGEF
//! effects keep their key and metadata ([`MwEffects::info`]) but have no def yet, so a landing
//! of one is `Rejected { UnknownEffect }` (reported, never silent) until its slice lands.
//!
//! Content mechanics ported from `OpenMW` (GPL-3.0) at commit
//! `ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f` (github.com/OpenMW/openmw); each module cites its
//! files.

mod book;
mod cast;
mod mgef;
mod source;
mod stats;

pub use book::{StartSpells, source_table, start_spells};
pub use cast::{
    BaseChance, CastChanceIn, ChanceEffectIn, ModernCast, ModernCastIn, MwModernEffectRules, OpenMwEffectRules,
    SILENCE_STAT, SOUND_STAT, base_success_chance, modern_cast, roll_chance, spell_school, success_chance,
};
pub use mgef::{
    EFFECT_COUNT, MwEffectInfo, MwEffects, Presentation, RangeFlags, SCHOOLS, archetype, effect_flags, effect_index,
    effect_key, effect_label, effect_name, effective_flags, flag, hardcoded_flags, resistance_effect, shield_effect,
    weakness_effect,
};
pub use source::{
    Affliction, FOOT_M, MapNote, MwEffectGmst, MwSource, Recharge, SourceFlags, effect_cost, map_effects, map_potion,
    map_spell, spell_cost,
};
pub use stats::MAGNITUDE_STAT_EFFECTS;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_book;
#[cfg(test)]
mod tests_cast;
