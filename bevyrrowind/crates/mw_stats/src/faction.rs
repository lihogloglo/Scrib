//! Faction rank requirements from FACT records (content, one implementation for every rule set).
//!
//! Ported from `OpenMW` `ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f`:
//! `apps/openmw/mwmechanics/npcstats.cpp` (`hasSkillsForRank`) and `apps/openmw/mwdialogue/
//! filter.cpp` (`hasFactionRankSkillRequirements`, `hasFactionRankReputationRequirements`,
//! `Function_RankRequirement`). Skills and attributes are compared by base value, truncated to
//! an integer for skills, as `OpenMW`.

use sf_records::RecordId;
use sf_stats::SheetRead;
use tes3::esp::Faction;

use crate::init::{attribute_index, skill_index};
use crate::table::MwIdx;

/// Ranks per faction (FADT).
pub const RANKS: usize = 10;

/// One rank's requirements (FADT `RankData`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RankRequirement {
    /// Minimum base of the faction's two favoured attributes.
    pub attributes: [i32; 2],
    /// Minimum of the best faction skill.
    pub primary_skill: i32,
    /// Minimum of the second and third best faction skills.
    pub favoured_skill: i32,
    /// Minimum faction reputation.
    pub reputation: i32,
}

/// A faction's rank requirements and the stats they read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FactionRanks {
    /// Favoured attributes (ESM indices); `None` for an unset slot, which then requires nothing
    /// (`OpenMW` would look up an empty attribute id).
    pub attributes: [Option<usize>; 2],
    /// Faction skills (ESM indices), unset slots dropped.
    pub skills: Vec<usize>,
    pub ranks: [RankRequirement; RANKS],
}

impl FactionRanks {
    #[must_use]
    pub fn of(faction: &Faction) -> Self {
        let d = &faction.data;
        Self {
            attributes: d.favored_attributes.map(attribute_index),
            skills: d.favored_skills.iter().copied().filter_map(skill_index).collect(),
            ranks: d.requirements.each_ref().map(|r| RankRequirement {
                attributes: r.attributes,
                primary_skill: r.primary_skill,
                favoured_skill: r.favored_skill,
                reputation: r.reputation,
            }),
        }
    }

    /// `hasSkillsForRank`: the best faction skill ≥ the rank's primary value, the next two ≥
    /// its favoured value (fewer skills check fewer; none passes). `false` for a rank past 9.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn has_skills_for_rank(&self, idx: &MwIdx, s: &SheetRead<'_>, rank: usize) -> bool {
        let Some(r) = self.ranks.get(rank) else { return false };
        let mut skills: Vec<i32> = self.skills.iter().map(|&n| s.base(idx.skills[n]) as i32).collect();
        skills.sort_unstable_by(|a, b| b.cmp(a));
        skills.iter().enumerate().take(3).all(|(k, &v)| v >= if k == 0 { r.primary_skill } else { r.favoured_skill })
    }

    /// `hasFactionRankSkillRequirements`: [`Self::has_skills_for_rank`] and both favoured
    /// attributes' base ≥ the rank's values.
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn meets_skill_requirements(&self, idx: &MwIdx, s: &SheetRead<'_>, rank: usize) -> bool {
        if !self.has_skills_for_rank(idx, s, rank) {
            return false;
        }
        let r = self.ranks[rank];
        self.attributes
            .iter()
            .zip(r.attributes)
            .all(|(a, min)| a.is_none_or(|a| s.base(idx.attributes[a]) >= min as f32))
    }

    /// `hasFactionRankReputationRequirements`: the sheet's reputation in `faction` (0 without a
    /// standing) ≥ the rank's. `false` for a rank past 9.
    #[must_use]
    pub fn meets_reputation(&self, s: &SheetRead<'_>, faction: &RecordId, rank: usize) -> bool {
        let Some(r) = self.ranks.get(rank) else { return false };
        s.membership(faction).map_or(0, |m| m.reputation) >= r.reputation
    }

    /// The `RankRequirement` dialogue function for the next rank in `faction` (the speaker's
    /// primary faction): +1 skills and attributes met, +2 reputation met; 0 at rank 9 or above.
    /// A non-member (rank -1) is checked against rank 0.
    #[must_use]
    pub fn rank_requirement(&self, idx: &MwIdx, s: &SheetRead<'_>, faction: &RecordId) -> i32 {
        let rank = s.membership(faction).and_then(|m| m.rank).unwrap_or(-1);
        if rank >= 9 {
            return 0;
        }
        let Ok(next) = usize::try_from(rank + 1) else { return 0 };
        i32::from(self.meets_skill_requirements(idx, s, next)) + 2 * i32::from(self.meets_reputation(s, faction, next))
    }
}
