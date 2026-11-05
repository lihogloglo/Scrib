//! A small configurable rule set for tests (this crate's, the Bevy shell's, adapters' smoke
//! tests). Not a game: linear formulas named by index.

use smallvec::SmallVec;

use crate::ops::StatOp;
use crate::rules::{
    DerivedModel, IncreaseSource, Inputs, LevelOffer, LevelingModel, Refused, Rules, SkillUseModel,
};
use crate::sheet::SheetRead;
use crate::table::{Role, StatIdx};

/// Every list is `(target, ...)` by [`StatIdx`]; empty lists mean "no such rule".
#[derive(Clone, Debug, Default)]
pub struct TestModel {
    /// Pool base = `factor` × modified value of `from`: `(pool, from, factor)`.
    pub pool_from: Vec<(StatIdx, StatIdx, f32)>,
    /// Derived value = `factor` × modified value of `from` (+ carried weight when `from` is the
    /// stat itself): `(stat, from, factor)`.
    pub derived_from: Vec<(StatIdx, StatIdx, f32)>,
    /// Regeneration per second: `(pool, rate)`; the stamina role is skipped for combatants.
    pub regen: Vec<(StatIdx, f32)>,
    /// Restored per rested hour: `(pool, rate)`.
    pub rest: Vec<(StatIdx, f32)>,
    /// Skill progress counters: `(skill, progress)`; a raise is due at `threshold`.
    pub progress: Vec<(StatIdx, StatIdx)>,
    pub threshold: f32,
    /// Skills don't rise past this base.
    pub skill_cap: f32,
    /// Level-up tally counter (+1 per skill raise by use or training) and how many make a level.
    pub tally: Option<StatIdx>,
    pub tally_needed: f32,
    /// Picks per level-up, the pickable stats and each pick's gain.
    pub picks: u8,
    pub pickable: Vec<StatIdx>,
    pub pick_gain: f32,
}

impl TestModel {
    /// The three seams, each a clone of this model.
    pub fn rules(self) -> Rules {
        Rules { derived: Box::new(self.clone()), skill_use: Box::new(self.clone()), leveling: Box::new(self) }
    }
}

impl DerivedModel for TestModel {
    fn pool_base(&self, s: &SheetRead<'_>, pool: StatIdx, _inputs: &Inputs) -> Option<f32> {
        self.pool_from.iter().find(|(p, ..)| *p == pool).map(|&(_, from, f)| s.modified(from) * f)
    }

    fn derived(&self, s: &SheetRead<'_>, stat: StatIdx, inputs: &Inputs) -> f32 {
        self.derived_from.iter().find(|(d, ..)| *d == stat).map_or(0.0, |&(_, from, f)| {
            if from == stat { inputs.carried * f } else { s.modified(from) * f }
        })
    }

    fn regen(&self, s: &SheetRead<'_>, dt: f32, inputs: &Inputs, out: &mut SmallVec<[StatOp; 3]>) {
        let stamina = s.role(Role::Stamina);
        for &(pool, rate) in &self.regen {
            if inputs.combatant && Some(pool) == stamina {
                continue;
            }
            if s.current(pool) < s.max(pool) {
                out.push(StatOp::PoolAdjust { pool, delta: rate * dt, below_zero: false, above_max: false });
            }
        }
    }

    fn rest(&self, _s: &SheetRead<'_>, hours: f32, _sleep: bool, _inputs: &Inputs, out: &mut SmallVec<[StatOp; 4]>) {
        for &(pool, rate) in &self.rest {
            out.push(StatOp::PoolAdjust { pool, delta: rate * hours, below_zero: false, above_max: false });
        }
    }
}

impl SkillUseModel for TestModel {
    fn use_skill(
        &self,
        s: &SheetRead<'_>,
        skill: StatIdx,
        _use_kind: u8,
        scale: f32,
        _inputs: &Inputs,
        out: &mut SmallVec<[StatOp; 2]>,
    ) -> f32 {
        let Some(&(_, progress)) = self.progress.iter().find(|(sk, _)| *sk == skill) else { return 0.0 };
        out.push(StatOp::CounterAdd { stat: progress, delta: scale });
        if s.modified(progress) + scale >= self.threshold {
            out.push(StatOp::RaiseSkill { skill, source: IncreaseSource::Usage });
        }
        scale
    }

    fn raise(&self, s: &SheetRead<'_>, skill: StatIdx, source: IncreaseSource, out: &mut SmallVec<[StatOp; 6]>) {
        if source == IncreaseSource::Penalty {
            out.push(StatOp::ModBase { stat: skill, delta: -1.0 });
            return;
        }
        if s.base(skill) >= self.skill_cap {
            return;
        }
        out.push(StatOp::ModBase { stat: skill, delta: 1.0 });
        if let Some(&(_, progress)) = self.progress.iter().find(|(sk, _)| *sk == skill) {
            out.push(StatOp::CounterSet { stat: progress, value: 0.0 });
        }
        if let Some(tally) = self.tally
            && matches!(source, IncreaseSource::Usage | IncreaseSource::Training)
        {
            out.push(StatOp::CounterAdd { stat: tally, delta: 1.0 });
        }
    }
}

impl LevelingModel for TestModel {
    fn ready(&self, s: &SheetRead<'_>) -> bool {
        self.tally.is_some_and(|t| s.modified(t) >= self.tally_needed)
    }

    fn offer(&self, _s: &SheetRead<'_>) -> LevelOffer {
        LevelOffer { picks: self.picks, gains: self.pickable.iter().map(|&i| (i, self.pick_gain)).collect() }
    }

    fn level_up(&self, s: &SheetRead<'_>, picks: &[StatIdx], out: &mut SmallVec<[StatOp; 8]>) -> Result<(), Refused> {
        if !self.ready(s) {
            return Err(Refused::NotReady);
        }
        if picks.len() != usize::from(self.picks) {
            return Err(Refused::PickCount { expected: self.picks, got: u8::try_from(picks.len()).unwrap_or(u8::MAX) });
        }
        if let Some(&bad) = picks.iter().find(|p| !self.pickable.contains(p)) {
            return Err(Refused::BadPick(bad));
        }
        for &stat in picks {
            out.push(StatOp::ModBase { stat, delta: self.pick_gain });
        }
        if let Some(level) = s.role(Role::Level) {
            out.push(StatOp::CounterAdd { stat: level, delta: 1.0 });
        }
        if let Some(tally) = self.tally {
            out.push(StatOp::CounterSet { stat: tally, value: 0.0 });
        }
        Ok(())
    }
}
