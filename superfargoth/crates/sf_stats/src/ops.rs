//! Operations, events and the pure step (stats.md "Operations", "Events"): every change to a
//! sheet is a [`StatOp`] run by [`apply`] or produced by a rules model inside [`apply`] / [`tick`].

use serde::{Deserialize, Serialize};
use sf_ids::{ActorKey, PlayerKey};
use sf_records::RecordId;
use smallvec::SmallVec;

use crate::rules::{IncreaseSource, Inputs, Rules};
use crate::sheet::{Membership, Modifier, Sheet, SourceId};
use crate::table::{Role, Slot, StatIdx, StatKind, StatTable};

/// One change to a sheet.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum StatOp {
    /// A value's or pool's base, or a counter's value.
    SetBase { stat: StatIdx, value: f32 },
    ModBase { stat: StatIdx, delta: f32 },
    /// Raises a value's damage; `cap_at_modified` caps the amount at the value left before `Mul`
    /// modifiers (`OpenMW` `spelleffects.cpp` `damageAttribute`/`damageSkill`), so the modified
    /// value bottoms out at 0 and damage already past it (an uncapped drain) is kept.
    Damage { stat: StatIdx, amount: f32, cap_at_modified: bool },
    /// Lowers a value's damage, never below 0.
    Restore { stat: StatIdx, amount: f32 },
    AddModifier(Modifier),
    /// Removes every modifier of a source; an unknown source is a no-op.
    RemoveSource(SourceId),
    /// current += delta with `OpenMW`'s `DynamicStat::setCurrent` flags: `below_zero` lets it go
    /// under 0, `above_max` over the max.
    PoolAdjust { pool: StatIdx, delta: f32, below_zero: bool, above_max: bool },
    /// Sets a pool's current as given (no clamping).
    PoolSetCurrent { pool: StatIdx, value: f32 },
    /// Base and current together, no [`crate::Rebase`] (`OpenMW` `modDynamicStat`). A pool whose
    /// base the derived model computes rewrites it at the next recompute: a lasting change to a
    /// derived pool is a [`StatOp::AddModifierShift`] instead.
    PoolModMax { pool: StatIdx, delta: f32 },
    CounterSet { stat: StatIdx, value: f32 },
    CounterAdd { stat: StatIdx, delta: f32 },
    /// Joins at rank 0 (no-op for a member).
    Join { group: RecordId },
    /// Sets the rank, joining if needed.
    SetRank { group: RecordId, rank: i32 },
    Expel { group: RecordId },
    ClearExpelled { group: RecordId },
    /// Group reputation: added, or set when `set`.
    GroupRep { group: RecordId, delta: i32, set: bool },
    /// One use of a skill, through [`crate::SkillUseModel::use_skill`].
    UseSkill { skill: StatIdx, use_kind: u8, scale: f32 },
    /// One skill level from `source`, through [`crate::SkillUseModel::raise`].
    RaiseSkill { skill: StatIdx, source: IncreaseSource },
    /// A level-up with the player's picks, through [`crate::LevelingModel::level_up`].
    LevelUp { picks: SmallVec<[StatIdx; 3]> },
    /// Rest or wait, through [`crate::DerivedModel::rest`].
    Rest { hours: f32, sleep: bool },
    /// Leaves the group: no rank and not expelled; the group reputation stays (`OpenMW`
    /// `NpcStats::setFactionRank` below 0 erases the rank and the expulsion, not the reputation).
    /// No-op for a non-member. Serialized op lists keep their indices: new variants go last.
    Leave { group: RecordId },
    /// [`StatOp::AddModifier`] whose change to a pool's max moves the current by the same
    /// amount, never by the pool's [`crate::Rebase`]: max and current together, source-tracked
    /// (`OpenMW` `modDynamicStat`, `spelleffects.cpp`; a pool fortify). The shifted current is
    /// clamped as `OpenMW`'s `setCurrent(current + delta)` (default flags: not below 0). While
    /// the sheet is building it is a plain `AddModifier` (no rebase then either).
    AddModifierShift(Modifier),
    /// [`StatOp::RemoveSource`] whose pools' currents shift by their max change, as
    /// [`StatOp::AddModifierShift`]: the inverse of a shifting add.
    RemoveSourceShift(SourceId),
}

/// Whose sheet (net rule 6: never a singleton, never an `Entity`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum StatOwner {
    Player(PlayerKey),
    Actor(ActorKey),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CauseKind {
    Combat,
    Effect,
    Script,
    Dialogue,
    Rules,
    Regen,
    Rest,
    Load,
    Debug,
}

/// Why a change happened (net rule 8: events carry tick, cause, owner).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Cause {
    pub kind: CauseKind,
    pub source: Option<SourceId>,
    /// The fixed tick it happened on.
    pub tick: u32,
}

/// The context of one [`apply`] call.
#[derive(Clone, Copy, Debug)]
pub struct ApplyCtx<'a> {
    pub owner: StatOwner,
    pub cause: Cause,
    pub inputs: &'a Inputs,
}

/// The context of one [`tick`].
#[derive(Clone, Copy, Debug)]
pub struct TickCtx<'a> {
    pub owner: StatOwner,
    pub tick: u32,
    /// Tick length in seconds.
    pub dt: f32,
    pub inputs: &'a Inputs,
}

/// What a sheet change tells the world (plain data, net rule 8).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum StatEvent {
    /// A value's modified value, a pool's current or a counter changed (one per stat per call).
    Changed { owner: StatOwner, stat: StatIdx, old: f32, new: f32, cause: Cause },
    /// A pool's current crossed below its `depleted_below`.
    Depleted { owner: StatOwner, pool: StatIdx, cause: Cause },
    SkillUsed { owner: StatOwner, skill: StatIdx, use_kind: u8, gain: f32, cause: Cause },
    SkillRaised { owner: StatOwner, skill: StatIdx, base: f32, source: IncreaseSource, cause: Cause },
    /// A level-up became due.
    LevelReady { owner: StatOwner, cause: Cause },
    /// The [`Role::Level`] counter changed.
    LevelChanged { owner: StatOwner, level: f32, cause: Cause },
    /// Rank in a group changed (`None` = not a member).
    RankChanged { owner: StatOwner, group: RecordId, old: Option<i32>, new: Option<i32>, cause: Cause },
    Expelled { owner: StatOwner, group: RecordId, cause: Cause },
}

impl StatEvent {
    /// Whose sheet the event is about.
    #[must_use]
    pub fn owner(&self) -> StatOwner {
        match self {
            Self::Changed { owner, .. }
            | Self::Depleted { owner, .. }
            | Self::SkillUsed { owner, .. }
            | Self::SkillRaised { owner, .. }
            | Self::LevelReady { owner, .. }
            | Self::LevelChanged { owner, .. }
            | Self::RankChanged { owner, .. }
            | Self::Expelled { owner, .. } => *owner,
        }
    }
}

/// Nested model ops deeper than this are rejected (a model loop can't hang the step).
const MAX_DEPTH: u8 = 4;

/// Runs `ops` in order on `sheet`, recomputes the caches and appends the events. Model calls
/// see fresh caches. Returns how many ops were rejected (wrong stat kind, unknown index, refused
/// level-up, model recursion too deep); a rejected op changes nothing.
pub fn apply(
    table: &StatTable,
    rules: &Rules,
    sheet: &mut Sheet,
    ops: &[StatOp],
    ctx: &ApplyCtx<'_>,
    out: &mut Vec<StatEvent>,
) -> usize {
    let before = Observed::take(sheet, true);
    sheet.recompute(table, rules, ctx.inputs);
    let ready_before = rules.leveling.ready(&sheet.view(table));
    let mut run = Run { table, rules, inputs: ctx.inputs, owner: ctx.owner, cause: ctx.cause, out, rejected: 0 };
    for op in ops {
        run.op(sheet, op, 0);
    }
    sheet.recompute(table, rules, ctx.inputs);
    sheet.refresh_derived(table, rules, ctx.inputs);
    let rejected = run.rejected;
    before.diff(table, sheet, ctx.owner, ctx.cause, out);
    if !ready_before && rules.leveling.ready(&sheet.view(table)) {
        out.push(StatEvent::LevelReady { owner: ctx.owner, cause: ctx.cause });
    }
    rejected
}

/// One fixed tick: recomputes when the sheet or its inputs changed, then regeneration for a
/// sheet with a pool below its max. Regeneration only moves pool currents (`PoolAdjust`,
/// `PoolSetCurrent`; other regen ops are ignored), so a clean sheet observes its pools only.
/// Events carry [`CauseKind::Regen`].
pub fn tick(table: &StatTable, rules: &Rules, sheet: &mut Sheet, ctx: &TickCtx<'_>, out: &mut Vec<StatEvent>) {
    let inputs_changed = sheet.last_inputs.as_ref().is_none_or(|l| !l.same_derivation(ctx.inputs));
    let below_max = sheet.pools.iter().any(|p| p.current < p.max);
    if !sheet.dirty && !inputs_changed && !below_max {
        return;
    }
    let cause = Cause { kind: CauseKind::Regen, source: None, tick: ctx.tick };
    let before = Observed::take(sheet, sheet.dirty || inputs_changed);
    sheet.recompute(table, rules, ctx.inputs);
    if sheet.pools.iter().any(|p| p.current < p.max) {
        let mut ops = SmallVec::<[StatOp; 3]>::new();
        rules.derived.regen(&sheet.view(table), ctx.dt, ctx.inputs, &mut ops);
        let mut run = Run { table, rules, inputs: ctx.inputs, owner: ctx.owner, cause, out, rejected: 0 };
        for op in ops.iter().filter(|op| matches!(op, StatOp::PoolAdjust { .. } | StatOp::PoolSetCurrent { .. })) {
            run.op(sheet, op, 1);
        }
        sheet.refresh_derived(table, rules, ctx.inputs);
    }
    before.diff(table, sheet, ctx.owner, cause, out);
}

/// The observed numbers before a step (modified values, pool currents, counters), from the
/// caches: what consumers last saw. Values and counters only when `full`. Inline sizes cover a
/// large game's table without a heap allocation per step.
struct Observed {
    values: SmallVec<[f32; 96]>,
    pools: SmallVec<[f32; 8]>,
    counters: SmallVec<[f32; 32]>,
    full: bool,
}

impl Observed {
    fn take(sheet: &Sheet, full: bool) -> Self {
        Self {
            values: if full { sheet.values.iter().map(|v| v.modified).collect() } else { SmallVec::new() },
            pools: sheet.pools.iter().map(|p| p.current).collect(),
            counters: if full { sheet.counters.iter().copied().collect() } else { SmallVec::new() },
            full,
        }
    }

    fn diff(&self, table: &StatTable, sheet: &Sheet, owner: StatOwner, cause: Cause, out: &mut Vec<StatEvent>) {
        let changed = |stat: StatIdx, old: f32, new: f32, out: &mut Vec<StatEvent>| {
            if old.to_bits() != new.to_bits() {
                out.push(StatEvent::Changed { owner, stat, old, new, cause });
            }
        };
        let (values, counters) = if self.full { (table.of_kind(0), table.of_kind(2)) } else { (&[][..], &[][..]) };
        for (slot, &idx) in values.iter().enumerate() {
            changed(idx, self.values[slot], sheet.values[slot].modified, out);
        }
        for (slot, &idx) in table.of_kind(1).iter().enumerate() {
            let (old, new) = (self.pools[slot], sheet.pools[slot].current);
            changed(idx, old, new, out);
            if let StatKind::Pool { depleted_below, .. } = table.def(idx).kind
                && old >= depleted_below
                && new < depleted_below
            {
                out.push(StatEvent::Depleted { owner, pool: idx, cause });
            }
        }
        let level = table.role(Role::Level);
        for (slot, &idx) in counters.iter().enumerate() {
            let (old, new) = (self.counters[slot], sheet.counters[slot]);
            changed(idx, old, new, out);
            if Some(idx) == level && old.to_bits() != new.to_bits() {
                out.push(StatEvent::LevelChanged { owner, level: new, cause });
            }
        }
    }
}

struct Run<'a, 'o> {
    table: &'a StatTable,
    rules: &'a Rules,
    inputs: &'a Inputs,
    owner: StatOwner,
    cause: Cause,
    out: &'o mut Vec<StatEvent>,
    rejected: usize,
}

impl Run<'_, '_> {
    fn reject(&mut self) {
        self.rejected += 1;
    }

    /// Recomputes with shifting pools; a moved current is saved state.
    fn shift(&self, sheet: &mut Sheet) {
        if sheet.recompute_shifting(self.table, self.rules, self.inputs) {
            sheet.changed_since_init = true;
        }
    }

    #[allow(clippy::too_many_lines)]
    fn op(&mut self, sheet: &mut Sheet, op: &StatOp, depth: u8) {
        if depth > MAX_DEPTH {
            self.reject();
            return;
        }
        let (table, owner, cause) = (self.table, self.owner, self.cause);
        let slot = |stat: StatIdx| table.try_slot(stat);
        match op {
            StatOp::SetBase { stat, value } | StatOp::ModBase { stat, delta: value } => {
                let set = matches!(op, StatOp::SetBase { .. });
                let new = |old: f32| if set { *value } else { old + value };
                match slot(*stat) {
                    Some(Slot::Value(s)) => sheet.values[usize::from(s)].base = new(sheet.values[usize::from(s)].base),
                    Some(Slot::Pool(s)) => sheet.pools[usize::from(s)].base = new(sheet.pools[usize::from(s)].base),
                    Some(Slot::Counter(s)) => set_counter(table, sheet, *stat, s, new(sheet.counters[usize::from(s)])),
                    _ => return self.reject(),
                }
                sheet.dirty = true;
                sheet.changed_since_init = true;
            }
            StatOp::Damage { stat, amount, cap_at_modified } => {
                let Some(Slot::Value(s)) = slot(*stat) else { return self.reject() };
                let sums = sheet.sums(*stat);
                let v = &mut sheet.values[usize::from(s)];
                let mut amount = amount.max(0.0);
                if *cap_at_modified {
                    // `OpenMW` `damageAttribute`/`damageSkill`: the amount is capped at the value
                    // left (base - damage + modifiers, ≥ 0); damage already past it is kept.
                    amount = amount.min((v.base + sums.permanent - v.damage + sums.temp).max(0.0));
                }
                v.damage += amount;
                sheet.dirty = true;
                sheet.changed_since_init = true;
            }
            StatOp::Restore { stat, amount } => {
                let Some(Slot::Value(s)) = slot(*stat) else { return self.reject() };
                let v = &mut sheet.values[usize::from(s)];
                if v.damage > 0.0 {
                    v.damage -= v.damage.min(amount.max(0.0));
                }
                sheet.dirty = true;
                sheet.changed_since_init = true;
            }
            StatOp::AddModifier(m) | StatOp::AddModifierShift(m) => {
                if !matches!(slot(m.stat), Some(Slot::Value(_) | Slot::Pool(_))) {
                    return self.reject();
                }
                let shift = matches!(op, StatOp::AddModifierShift(_));
                if shift {
                    // Earlier ops in this call rebase as usual; only this modifier shifts.
                    sheet.recompute(table, self.rules, self.inputs);
                }
                sheet.mods.push(*m);
                sheet.dirty = true;
                if shift {
                    self.shift(sheet);
                }
            }
            StatOp::RemoveSource(src) | StatOp::RemoveSourceShift(src) => {
                let shift = matches!(op, StatOp::RemoveSourceShift(_));
                if shift {
                    sheet.recompute(table, self.rules, self.inputs);
                }
                let n = sheet.mods.len();
                sheet.mods.retain(|m| m.source != *src);
                if sheet.mods.len() != n {
                    sheet.dirty = true;
                    if shift {
                        self.shift(sheet);
                    }
                }
            }
            StatOp::PoolAdjust { pool, delta, below_zero, above_max } => {
                let Some(Slot::Pool(s)) = slot(*pool) else { return self.reject() };
                // A max moved by earlier ops in this call must be current before the flags compare.
                sheet.recompute(table, self.rules, self.inputs);
                let p = &mut sheet.pools[usize::from(s)];
                p.current = set_current(p.current, p.max, p.current + delta, *below_zero, *above_max);
                sheet.changed_since_init = true;
            }
            StatOp::PoolSetCurrent { pool, value } => {
                let Some(Slot::Pool(s)) = slot(*pool) else { return self.reject() };
                sheet.pools[usize::from(s)].current = *value;
                sheet.changed_since_init = true;
            }
            StatOp::PoolModMax { pool, delta } => {
                let Some(Slot::Pool(s)) = slot(*pool) else { return self.reject() };
                sheet.recompute(table, self.rules, self.inputs);
                let p = &mut sheet.pools[usize::from(s)];
                p.base += delta;
                p.current += delta;
                // The max moves with the base here, so the recompute sees no change to rebase.
                p.max += delta;
                sheet.dirty = true;
                sheet.changed_since_init = true;
            }
            StatOp::CounterSet { stat, value } | StatOp::CounterAdd { stat, delta: value } => {
                let Some(Slot::Counter(s)) = slot(*stat) else { return self.reject() };
                let v = if matches!(op, StatOp::CounterSet { .. }) { *value } else { sheet.counters[usize::from(s)] + value };
                set_counter(table, sheet, *stat, s, v);
                sheet.changed_since_init = true;
            }
            StatOp::Join { group } => {
                let m = sheet.groups.entry(group.clone()).or_default();
                if m.rank.is_none() {
                    m.rank = Some(0);
                    self.out.push(StatEvent::RankChanged { owner, group: group.clone(), old: None, new: Some(0), cause });
                }
                sheet.changed_since_init = true;
            }
            StatOp::SetRank { group, rank } => {
                let m = sheet.groups.entry(group.clone()).or_default();
                let old = m.rank.replace(*rank);
                if old != Some(*rank) {
                    self.out.push(StatEvent::RankChanged { owner, group: group.clone(), old, new: Some(*rank), cause });
                }
                sheet.changed_since_init = true;
            }
            StatOp::Expel { group } => {
                let m = sheet.groups.entry(group.clone()).or_default();
                if !m.expelled {
                    m.expelled = true;
                    self.out.push(StatEvent::Expelled { owner, group: group.clone(), cause });
                }
                sheet.changed_since_init = true;
            }
            StatOp::ClearExpelled { group } => {
                if let Some(m) = sheet.groups.get_mut(group) {
                    m.expelled = false;
                }
                sheet.changed_since_init = true;
            }
            StatOp::GroupRep { group, delta, set } => {
                let m: &mut Membership = sheet.groups.entry(group.clone()).or_default();
                m.reputation = if *set { *delta } else { m.reputation.saturating_add(*delta) };
                sheet.changed_since_init = true;
            }
            StatOp::Leave { group } => {
                if let Some(m) = sheet.groups.get_mut(group) {
                    let old = m.rank.take();
                    m.expelled = false;
                    if old.is_some() {
                        self.out.push(StatEvent::RankChanged { owner, group: group.clone(), old, new: None, cause });
                    }
                    sheet.changed_since_init = true;
                }
            }
            StatOp::UseSkill { skill, use_kind, scale } => {
                if slot(*skill).is_none() {
                    return self.reject();
                }
                sheet.recompute(table, self.rules, self.inputs);
                let mut ops = SmallVec::<[StatOp; 2]>::new();
                let gain = self.rules.skill_use.use_skill(&sheet.view(table), *skill, *use_kind, *scale, self.inputs, &mut ops);
                self.out.push(StatEvent::SkillUsed { owner, skill: *skill, use_kind: *use_kind, gain, cause });
                for op in &ops {
                    self.op(sheet, op, depth + 1);
                }
            }
            StatOp::RaiseSkill { skill, source } => {
                if slot(*skill).is_none() {
                    return self.reject();
                }
                sheet.recompute(table, self.rules, self.inputs);
                let mut ops = SmallVec::<[StatOp; 6]>::new();
                self.rules.skill_use.raise(&sheet.view(table), *skill, *source, &mut ops);
                if ops.is_empty() {
                    return;
                }
                for op in &ops {
                    self.op(sheet, op, depth + 1);
                }
                let base = sheet.view(table).base(*skill);
                self.out.push(StatEvent::SkillRaised { owner, skill: *skill, base, source: *source, cause });
            }
            StatOp::LevelUp { picks } => {
                sheet.recompute(table, self.rules, self.inputs);
                let mut ops = SmallVec::<[StatOp; 8]>::new();
                if self.rules.leveling.level_up(&sheet.view(table), picks, &mut ops).is_err() {
                    return self.reject();
                }
                for op in &ops {
                    self.op(sheet, op, depth + 1);
                }
            }
            StatOp::Rest { hours, sleep } => {
                sheet.recompute(table, self.rules, self.inputs);
                let mut ops = SmallVec::<[StatOp; 4]>::new();
                self.rules.derived.rest(&sheet.view(table), *hours, *sleep, self.inputs, &mut ops);
                for op in &ops {
                    self.op(sheet, op, depth + 1);
                }
            }
        }
    }
}

fn set_counter(table: &StatTable, sheet: &mut Sheet, stat: StatIdx, slot: u16, value: f32) {
    let v = match table.def(stat).kind {
        StatKind::Counter { min, max } => value.clamp(min, max),
        _ => value,
    };
    sheet.counters[usize::from(slot)] = v;
}

/// `OpenMW` `DynamicStat<T>::setCurrent(value, allowDecreaseBelowZero, allowIncreaseAboveModified)`
/// (`apps/openmw/mwmechanics/stat.cpp` at `ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f`, checked
/// branch by branch in stats S2): an increase stops at the max unless allowed (and never
/// lowers a current already above it); a decrease stops at 0 unless allowed (and never raises a
/// current already below it).
pub fn set_current(current: f32, max: f32, value: f32, below_zero: bool, above_max: bool) -> f32 {
    if value > current {
        if value <= max || above_max {
            value
        } else if current > max {
            current
        } else {
            max
        }
    } else if value > 0.0 || below_zero {
        value
    } else if current > 0.0 {
        0.0
    } else {
        current
    }
}
