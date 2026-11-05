//! The first `SkillUseModel` and `LevelingModel`: `OpenMW`'s skill progression and level-up.
//!
//! Ported from `OpenMW` `ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f`:
//! `files/data-mw/scripts/omw/playerskillhandlers.lua` (`getSkillProgressRequirement`,
//! `getSkillLevelUpOptions`, `skillLevelUpHandler`, `skillUsedHandler`),
//! `files/data/scripts/omw/skillhandlers.lua` (`skillUsed`: gain = use value x scale, use types
//! 0..=3), `apps/openmw/mwmechanics/npcstats.cpp` (`levelUp`, `getLevelupAttributeMultiplier`),
//! `apps/openmw/mwgui/levelupdialog.cpp` (`open`: picks = min(3, attributes below 100);
//! `onOkButtonClicked`: + multiplier, cap 100, then `levelUp`), `apps/openmw/mwgui/
//! waitdialog.cpp` (a level-up is due at `iLevelUpTotal`).
//!
//! The progress arithmetic runs in `f64` and stores `f32`, as the Lua handler (doubles) writing
//! the C++ skill progress (float). The class comes from the sheet's `class.<skill>` and
//! `class.specialization` counters ([`crate::class_ops`]): the trait methods see only the sheet.
//!
//! Waiting for the effect runtime: no skill gain in werewolf form (`skillUsedHandler`).

use sf_stats::{IncreaseSource, Inputs, LevelOffer, LevelingModel, Refused, SheetRead, SkillUseModel, StatIdx, StatOp};
use smallvec::SmallVec;

use crate::gmst::MwGmst;
use crate::table::{MwIdx, MwTable, SkillInfo, attr, class_skill};

/// ESM indices of the two skills jail raises instead of lowering (`getSkillLevelUpOptions`).
const SECURITY: usize = 18;
const SNEAK: usize = 19;

/// Attribute picks per level-up (`sMaxCoins`).
const MAX_PICKS: usize = 3;

/// `getSkillProgressRequirement`: (base + 1) x `fMajorSkillBonus` / `fMinorSkillBonus` /
/// `fMiscSkillBonus`, x `fSpecialSkillBonus` when the skill's specialization is the class's.
#[must_use]
pub fn skill_progress_requirement(g: &MwGmst, base: f32, class_kind: f32, same_specialization: bool) -> f64 {
    let mut factor = f64::from(if class_kind == class_skill::MAJOR {
        g.major_skill_bonus
    } else if class_kind == class_skill::MINOR {
        g.minor_skill_bonus
    } else {
        g.misc_skill_bonus
    });
    if same_specialization {
        factor *= f64::from(g.special_skill_bonus);
    }
    (f64::from(base) + 1.0) * factor
}

/// `getLevelupAttributeMultiplier`: `iLevelUpNNMult` with NN = clamp(tally, 0, 10), 1 for a 0
/// tally. A GMST missing from the content took its vanilla value in [`MwGmst::from_db`] (`OpenMW`
/// would use 1).
#[must_use]
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn levelup_attribute_multiplier(g: &MwGmst, tally: f32) -> f32 {
    let num = tally.clamp(0.0, 10.0) as usize;
    if num == 0 { 1.0 } else { g.level_up_mult[num - 1] }
}

/// `OpenMW`'s skill progression (`playerskillhandlers.lua`).
#[derive(Clone, Debug)]
pub struct OpenMwSkillUse {
    pub gmst: MwGmst,
    pub idx: MwIdx,
    pub skills: [SkillInfo; 27],
}

impl OpenMwSkillUse {
    #[must_use]
    pub fn new(t: &MwTable, gmst: &MwGmst) -> Self {
        Self { gmst: gmst.clone(), idx: t.idx.clone(), skills: t.skills }
    }

    fn skill_number(&self, skill: StatIdx) -> Option<usize> {
        self.idx.skills.iter().position(|&s| s == skill)
    }

    fn class_kind(&self, s: &SheetRead<'_>, n: usize) -> f32 {
        s.base(self.idx.class_skills[n])
    }

    /// The `class.specialization` counter holds specialization + 1 (0 = none).
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn same_specialization(&self, s: &SheetRead<'_>, n: usize) -> bool {
        let stored = s.base(self.idx.class_specialization);
        stored >= 1.0 && stored as usize - 1 == self.skills[n].specialization
    }

    /// The requirement for one more level of skill `n` on this sheet.
    #[must_use]
    pub fn requirement(&self, s: &SheetRead<'_>, n: usize) -> f64 {
        let base = s.base(self.idx.skills[n]);
        skill_progress_requirement(&self.gmst, base, self.class_kind(s, n), self.same_specialization(s, n))
    }
}

impl SkillUseModel for OpenMwSkillUse {
    /// `skillUsed` + `skillUsedHandler`: gain = SKIL use value x scale; progress += gain /
    /// requirement; a `RaiseSkill { Usage }` at progress ≥ 1. Returns 0 (no ops) for an unknown
    /// use kind or a refused use (base 100 gaining, base 0 losing).
    #[allow(clippy::cast_possible_truncation)]
    fn use_skill(
        &self,
        s: &SheetRead<'_>,
        skill: StatIdx,
        use_kind: u8,
        scale: f32,
        _inputs: &Inputs,
        out: &mut SmallVec<[StatOp; 2]>,
    ) -> f32 {
        let Some(n) = self.skill_number(skill) else { return 0.0 };
        let Some(&use_value) = self.skills[n].use_values.get(usize::from(use_kind)) else { return 0.0 };
        let gain = f64::from(use_value) * f64::from(scale);
        let base = s.base(skill);
        if (base >= 100.0 && gain > 0.0) || (base <= 0.0 && gain < 0.0) {
            return 0.0;
        }
        let progress_idx = self.idx.skill_progress[n];
        let progress = (f64::from(s.base(progress_idx)) + gain / self.requirement(s, n)) as f32;
        out.push(StatOp::CounterSet { stat: progress_idx, value: progress });
        if progress >= 1.0 {
            out.push(StatOp::RaiseSkill { skill, source: IncreaseSource::Usage });
        }
        gain as f32
    }

    /// `getSkillLevelUpOptions` + `skillLevelUpHandler`: base + 1 (refused at 100); level
    /// progress + `iLevelUpMajorMult` / `iLevelUpMinorMult` (misc 0); the governing attribute's
    /// tally + `iLevelUpMajorMultAttribute` / `iLevelUpMinorMultAttribute` /
    /// `iLevelupMiscMultAttriubte`; the specialization tally + `iLevelupSpecialization`; usage
    /// resets the skill's progress (training, books keep it). `Penalty` is `OpenMW`'s jail
    /// source: -1 without tallies (refused at 0), except Security and Sneak, which jail raises
    /// like training. `Script` and `Debug` count as training.
    fn raise(&self, s: &SheetRead<'_>, skill: StatIdx, source: IncreaseSource, out: &mut SmallVec<[StatOp; 6]>) {
        let Some(n) = self.skill_number(skill) else { return };
        let jail_loss = source == IncreaseSource::Penalty && n != SECURITY && n != SNEAK;
        let increase = if jail_loss { -1.0 } else { 1.0 };
        let base = s.base(skill);
        if (base >= 100.0 && increase > 0.0) || (base <= 0.0 && increase < 0.0) {
            return;
        }
        out.push(StatOp::ModBase { stat: skill, delta: increase });
        if !jail_loss {
            let g = &self.gmst;
            let kind = self.class_kind(s, n);
            let (progress, attribute) = if kind == class_skill::MINOR {
                (g.level_up_minor_mult, g.level_up_minor_mult_attribute)
            } else if kind == class_skill::MAJOR {
                (g.level_up_major_mult, g.level_up_major_mult_attribute)
            } else {
                (0.0, g.level_up_misc_mult_attribute)
            };
            let info = self.skills[n];
            for (stat, delta) in [
                (self.idx.level_progress, progress),
                (self.idx.levelup_attr[info.governing], attribute),
                (self.idx.levelup_spec[info.specialization], g.level_up_specialization),
            ] {
                if delta != 0.0 {
                    out.push(StatOp::CounterAdd { stat, delta });
                }
            }
        }
        if source == IncreaseSource::Usage {
            out.push(StatOp::CounterSet { stat: self.idx.skill_progress[n], value: 0.0 });
        }
    }
}

/// `OpenMW`'s level-up (`levelupdialog.cpp`, `NpcStats::levelUp`).
#[derive(Clone, Debug)]
pub struct OpenMwLeveling {
    pub gmst: MwGmst,
    pub idx: MwIdx,
}

impl OpenMwLeveling {
    #[must_use]
    pub fn new(t: &MwTable, gmst: &MwGmst) -> Self {
        Self { gmst: gmst.clone(), idx: t.idx.clone() }
    }

    /// The attributes a level-up can raise (base below 100) and what each would gain.
    fn pickable(&self, s: &SheetRead<'_>) -> Vec<(StatIdx, f32)> {
        (0..8)
            .filter_map(|a| {
                let stat = self.idx.attributes[a];
                let base = s.base(stat);
                (base < 100.0).then(|| {
                    let mult = levelup_attribute_multiplier(&self.gmst, s.base(self.idx.levelup_attr[a]));
                    (stat, mult.min(100.0 - base))
                })
            })
            .collect()
    }
}

impl LevelingModel for OpenMwLeveling {
    /// `level.progress` ≥ `iLevelUpTotal` (the wait dialog offers the level-up on sleep).
    fn ready(&self, s: &SheetRead<'_>) -> bool {
        s.base(self.idx.level_progress) >= self.gmst.level_up_total
    }

    /// min(3, attributes below 100) picks; each pickable attribute's gain = its multiplier, up
    /// to 100.
    #[allow(clippy::cast_possible_truncation)]
    fn offer(&self, s: &SheetRead<'_>) -> LevelOffer {
        let gains = self.pickable(s);
        LevelOffer { picks: gains.len().min(MAX_PICKS) as u8, gains }
    }

    /// Distinct pickable attributes, exactly the offered count: each + its multiplier (cap 100);
    /// then level progress - `iLevelUpTotal` (≥ 0), attribute tallies cleared (specialization
    /// tallies stay, as `OpenMW`), health base and current + End (after the picks) x
    /// `fLevelUpHealthEndMult` (current ≥ 1), level + 1.
    #[allow(clippy::cast_possible_truncation)]
    fn level_up(&self, s: &SheetRead<'_>, picks: &[StatIdx], out: &mut SmallVec<[StatOp; 8]>) -> Result<(), Refused> {
        if !self.ready(s) {
            return Err(Refused::NotReady);
        }
        let pickable = self.pickable(s);
        let expected = pickable.len().min(MAX_PICKS);
        if picks.len() != expected {
            return Err(Refused::PickCount { expected: expected as u8, got: picks.len().min(255) as u8 });
        }
        for (k, pick) in picks.iter().enumerate() {
            if picks[..k].contains(pick) || !pickable.iter().any(|(stat, _)| stat == pick) {
                return Err(Refused::BadPick(*pick));
            }
        }
        let i = &self.idx;
        let g = &self.gmst;
        let mut endurance = s.base(i.attributes[attr::ENDURANCE]);
        for &pick in picks {
            let a = i.attributes.iter().position(|&x| x == pick).expect("checked pickable");
            let mult = levelup_attribute_multiplier(g, s.base(i.levelup_attr[a]));
            let value = (s.base(pick) + mult).min(100.0);
            if a == attr::ENDURANCE {
                endurance = value;
            }
            out.push(StatOp::SetBase { stat: pick, value });
        }
        out.push(StatOp::CounterSet {
            stat: i.level_progress,
            value: (s.base(i.level_progress) - g.level_up_total).max(0.0),
        });
        for &tally in &i.levelup_attr {
            if s.base(tally) != 0.0 {
                out.push(StatOp::CounterSet { stat: tally, value: 0.0 });
            }
        }
        let gain = endurance * g.level_up_health_end_mult;
        out.push(StatOp::PoolModMax { pool: i.health, delta: gain });
        if s.current(i.health) + gain < 1.0 {
            out.push(StatOp::PoolSetCurrent { pool: i.health, value: 1.0 });
        }
        out.push(StatOp::CounterAdd { stat: i.level, delta: 1.0 });
        Ok(())
    }
}

/// `OpenMW`'s rules: [`crate::MwDerived`], [`OpenMwSkillUse`], [`OpenMwLeveling`] on one GMST
/// snapshot (the first rule set; a modern profile swaps models behind the same traits).
#[must_use]
pub fn openmw_rules(t: &MwTable, gmst: &MwGmst) -> sf_stats::Rules {
    sf_stats::Rules {
        derived: Box::new(crate::MwDerived { gmst: gmst.clone(), idx: t.idx.clone() }),
        skill_use: Box::new(OpenMwSkillUse::new(t, gmst)),
        leveling: Box::new(OpenMwLeveling::new(t, gmst)),
    }
}
