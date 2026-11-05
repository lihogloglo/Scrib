//! Morrowind's character stats on `sf_stats` (`docs/specs/stats-adapter.md`): the stat table
//! from the format's attributes and the SKIL records ([`MwTable`]), the GMST snapshot
//! ([`MwGmst`]), sheet init for NPCs (record or autocalc), creatures and chargen ([`npc_ops`],
//! [`creature_ops`], [`chargen_ops`], [`new_sheet`]) and the derivation half of the first
//! `DerivedModel` ([`MwDerived`]). Bevy-free.
//!
//! Content mechanics ported from `OpenMW` (GPL-3.0) at commit
//! `ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f` (github.com/OpenMW/openmw); each module cites its
//! files. Skill use, levelling, regen and rest (the rules seams) are stats S3.

mod derived;
mod gmst;
mod init;
mod table;

pub use derived::{MwDerived, capacity, fatigue_max, fatigue_term, magicka_max};
pub use gmst::MwGmst;
pub use init::{
    Chargen, ClassInfo, InitError, autocalc_attributes, autocalc_health, autocalc_skills, chargen_ops, creature_ops,
    is_autocalc, new_sheet, npc_ops, ops_for_missing, race_attributes, race_skill_bonus, round_half_even,
};
pub use table::{ATTRIBUTES, MwIdx, MwTable, MwTableError, SKILLS, SPECIALIZATIONS, SkillInfo, attr, defs, roles};

#[cfg(test)]
mod tests;
