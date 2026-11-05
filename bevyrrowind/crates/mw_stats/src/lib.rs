//! Morrowind's character stats on `sf_stats` (`docs/specs/stats-adapter.md`): the stat table
//! from the format's attributes and the SKIL records ([`MwTable`]), the GMST snapshot
//! ([`MwGmst`]), sheet init for NPCs (record or autocalc), creatures and chargen ([`npc_ops`],
//! [`creature_ops`], [`chargen_ops`], [`new_sheet`]), the first rule set ([`openmw_rules`]:
//! [`MwDerived`], [`OpenMwSkillUse`], [`OpenMwLeveling`]) and faction rank requirements
//! ([`FactionRanks`]). Bevy-free.
//!
//! Content mechanics ported from `OpenMW` (GPL-3.0) at commit
//! `ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f` (github.com/OpenMW/openmw); each module cites its
//! files.

mod derived;
mod faction;
mod gmst;
mod init;
mod progression;
mod table;

pub use derived::{
    MwDerived, capacity, fatigue_max, fatigue_regen_per_second, fatigue_term, magicka_max, normalized_encumbrance,
    rest_fatigue_per_hour, sleep_restoration_per_hour,
};
pub use faction::{FactionRanks, RANKS, RankRequirement};
pub use gmst::MwGmst;
pub use init::{
    Chargen, ClassInfo, InitError, autocalc_attributes, autocalc_health, autocalc_skills, chargen_ops, class_ops,
    creature_ops, is_autocalc, new_sheet, npc_ops, ops_for_missing, player_start_ops, race_attributes, race_skill_bonus,
    round_half_even,
};
pub use progression::{
    OpenMwLeveling, OpenMwSkillUse, levelup_attribute_multiplier, openmw_rules, skill_progress_requirement,
};
pub use table::{
    ATTRIBUTES, MwIdx, MwTable, MwTableError, SKILLS, SPECIALIZATIONS, SkillInfo, attr, class_skill, defs, roles,
};

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_s3;
