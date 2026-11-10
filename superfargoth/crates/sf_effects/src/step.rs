//! The pure step (effects.md "Lifecycle"): [`apply_book`] (and its book-less form [`apply`])
//! runs ops, [`tick`] advances one fixed tick. Both read the active set and the stat table and
//! write no sheet: stat changes leave as ops with one `Cause { kind: Effect, source }` per
//! effect; casts read the caster's sheet through the cast model only.

use sf_records::RecordId;
use sf_stats::{Cause, CauseKind, Layer, ModOp, Modifier, Role, SheetRead, SourceId, StatIdx, StatOp, StatTable};
use smallvec::SmallVec;

use crate::active::{
    ActiveEffect, ActiveSet, ActiveSource, ActorRef, EffectIdAlloc, InstanceFlags, InstanceId, MAX_EFFECTS_PER_SOURCE,
    Phase, SourceClass, SourceRef, StackKey,
};
use crate::book::{BookRejection, GameMinutes, SourceDef, SourceTable, SpellBook};
use crate::def::{ArgKind, ArgRef, Archetype, CureFilter, EffectDef, EffectTable, PoolMode, StackPolicy, StatTarget};
use crate::op::{
    Delivery, EffectCtx, EffectEntry, EffectEvent, EffectList, EffectOp, EffectOut, EffectRequest, EndReason, Range,
    Rejection, RequestPhase, RollSeed,
};
use crate::rules::{CastFailReason, CastModel, CastPlan, CastQuery, EffectRules, LandModel, ResistQuery, RollQuery, roll};

/// What the step reads besides the active set and the book: content tables, the rules, the
/// target's own sheet (the caster of `Cast`) and the world clock (power recharges).
#[derive(Clone, Copy)]
pub struct EffectEnv<'a> {
    pub defs: &'a EffectTable,
    pub stats: &'a StatTable,
    pub sources: &'a SourceTable,
    pub land: &'a dyn LandModel,
    pub cast: &'a dyn CastModel,
    /// The target's sheet; `None` refuses casts (`NotCastable`).
    pub sheet: Option<SheetRead<'a>>,
    /// The world clock now.
    pub time: GameMinutes,
}

impl<'a> EffectEnv<'a> {
    /// The env of one game's [`EffectRules`].
    pub fn new(
        defs: &'a EffectTable,
        stats: &'a StatTable,
        sources: &'a SourceTable,
        rules: &'a EffectRules,
        sheet: Option<SheetRead<'a>>,
        time: GameMinutes,
    ) -> Self {
        Self { defs, stats, sources, land: &*rules.land, cast: &*rules.cast, sheet, time }
    }
}

impl std::fmt::Debug for EffectEnv<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EffectEnv").field("sheet", &self.sheet.is_some()).field("time", &self.time).finish_non_exhaustive()
    }
}

/// The cast model of the book-less [`apply`]: never reached (the empty book refuses first).
struct NoCast;

impl CastModel for NoCast {
    fn plan(&self, _q: &CastQuery<'_>) -> CastPlan {
        CastPlan::Refused(CastFailReason::NotCastable)
    }
}

/// Runs `ops` in order on `set` (the context's target) with no book and no source table, as
/// E1: `Learn` reports `BookRejected { UnknownSource }`, `Cast` fails `UnknownSource`, tag cures
/// match nothing. Hosts with books use [`apply_book`].
pub fn apply(
    defs: &EffectTable,
    stats: &StatTable,
    land: &dyn LandModel,
    set: &mut ActiveSet,
    ops: &[EffectOp],
    ctx: &mut EffectCtx<'_>,
    out: &mut EffectOut,
) {
    let sources = SourceTable::default();
    let env = EffectEnv { defs, stats, sources: &sources, land, cast: &NoCast, sheet: None, time: GameMinutes::default() };
    apply_book(&env, set, &mut SpellBook::new(), ops, ctx, out);
}

/// Runs `ops` in order on `set` and `book` (both the context's target's), appending stat ops,
/// events and requests to `out`.
pub fn apply_book(
    env: &EffectEnv<'_>,
    set: &mut ActiveSet,
    book: &mut SpellBook,
    ops: &[EffectOp],
    ctx: &mut EffectCtx<'_>,
    out: &mut EffectOut,
) {
    let step =
        Step { defs: env.defs, stats: env.stats, src_defs: env.sources, target: ctx.target, tick: ctx.tick, dt: ctx.dt };
    for op in ops {
        match op {
            EffectOp::Inflict { source, class, caster, entries, flags, rolls } => {
                let landing =
                    Landing { source, class: *class, caster: *caster, entries, flags: *flags, rolls: *rolls };
                let cured = step.inflict(env.land, set, &landing, ctx.ids, out);
                step.unlearn(book, &cured, out);
            }
            EffectOp::End { instance, reason } => step.end_instance(set, *instance, *reason, out),
            EffectOp::EndBySource { record } => {
                let ids: SmallVec<[InstanceId; 4]> =
                    set.sources.iter().filter(|s| s.source.record == *record).map(|s| s.id).collect();
                for id in ids {
                    step.end_instance(set, id, EndReason::SourceRemoved, out);
                }
            }
            EffectOp::Skip { seconds } => step.advance(set, ticks_for(*seconds, ctx.dt), out),
            EffectOp::Cast { source, target, rolls } => {
                let cast = CastOp { source, target: *target, rolls: *rolls };
                step.cast(env, set, book, &cast, ctx.ids, out);
            }
            EffectOp::Learn { source } => step.learn(env.land, set, book, source, ctx.ids, out),
            EffectOp::Forget { source } => step.forget(set, book, source, out),
            EffectOp::Select { source } => step.select(book, source.as_ref(), out),
            EffectOp::Cure { filter, rolls } => {
                let cured = step.cure(set, filter, 100.0, *rolls, out);
                step.unlearn(book, &cured, out);
            }
            EffectOp::Invalidate { instance, index } => step.invalidate(set, *instance, *index, out),
        }
    }
    set.timed = step.timed(set);
}

/// One fixed tick: rate effects deliver `magnitude · dt`, timed effects age and expire. Sets
/// without timed effects return at once.
pub fn tick(defs: &EffectTable, stats: &StatTable, set: &mut ActiveSet, ctx: &EffectCtx<'_>, out: &mut EffectOut) {
    if !set.timed {
        return;
    }
    let src_defs = SourceTable::default();
    let step = Step { defs, stats, src_defs: &src_defs, target: ctx.target, tick: ctx.tick, dt: ctx.dt };
    step.advance(set, 1, out);
    set.timed = step.timed(set);
}

/// Every running effect's start-stage ops in `set`, as its landing emitted them (target side and
/// an absorb's caster side), for [`crate::reregister`] to filter.
pub(crate) fn start_ops(
    defs: &EffectTable,
    stats: &StatTable,
    set: &ActiveSet,
    target: ActorRef,
    tick: u32,
    f: &mut impl FnMut(ActorRef, StatOp, Cause),
) {
    let src_defs = SourceTable::default();
    let step = Step { defs, stats, src_defs: &src_defs, target, tick, dt: 0.0 };
    let mut out = EffectOut::default();
    for src in &set.sources {
        let who = Owner::from(src);
        for e in src.effects.iter().filter(|e| e.phase == Phase::Running) {
            if let Some(def) = defs.get(&e.effect) {
                step.emit(who, def, e, Stage::Start(e.magnitude), &mut out);
            }
        }
    }
    for (owner, op, cause) in out.stat_ops {
        f(owner, op, cause);
    }
}

/// `round(seconds / dt)`; float-to-int `as` saturates (negative and NaN give 0).
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, reason = "saturating float-to-int cast")]
fn ticks_for(seconds: f32, dt: f32) -> u32 {
    if dt > 0.0 { (seconds / dt).round() as u32 } else { 0 }
}

#[allow(clippy::cast_precision_loss, reason = "tick counts stay far below 2^24")]
fn ticks_f32(ticks: u32) -> f32 {
    ticks as f32
}

struct Step<'a> {
    defs: &'a EffectTable,
    stats: &'a StatTable,
    /// Source defs: book ops, casts, tag cures.
    src_defs: &'a SourceTable,
    target: ActorRef,
    tick: u32,
    dt: f32,
}

/// One `Cast` op's fields.
struct CastOp<'a> {
    source: &'a RecordId,
    target: Option<ActorRef>,
    rolls: RollSeed,
}

/// A cast's entries by range: self entries land now on the caster, the rest leave as requests
/// (touch only with a contact; dropped without one).
fn split_delivery(def: &SourceDef, c: &CastOp<'_>, caster: ActorRef) -> (EffectList, SmallVec<[EffectRequest; 2]>) {
    let (mut own, mut touch, mut far) = (EffectList::new(), EffectList::new(), EffectList::new());
    for e in &def.effects {
        match e.range {
            Range::OnSelf => own.push(e.clone()),
            Range::Touch => touch.push(e.clone()),
            Range::Target => far.push(e.clone()),
        }
    }
    let delivery = |entries| Delivery { source: def.source_ref(), class: def.class, caster, entries, rolls: c.rolls };
    let mut requests = SmallVec::new();
    if let Some(target) = c.target
        && !touch.is_empty()
    {
        requests.push(EffectRequest::Touch { target, delivery: delivery(touch) });
    }
    if !far.is_empty() {
        requests.push(EffectRequest::Launch(delivery(far)));
    }
    (own, requests)
}

/// One `Inflict` op's fields.
struct Landing<'a> {
    source: &'a SourceRef,
    class: SourceClass,
    caster: Option<ActorRef>,
    entries: &'a [EffectEntry],
    flags: InstanceFlags,
    rolls: RollSeed,
}

impl Landing<'_> {
    /// `s` is an earlier landing of the same source under the class's [`StackKey`].
    fn same_stack(&self, s: &ActiveSource) -> bool {
        let record = s.source.record == self.source.record;
        match self.class.stack_key() {
            StackKey::BySource => record,
            StackKey::BySourceCaster => record && s.caster == self.caster,
            StackKey::BySourceCasterItem => record && s.caster == self.caster && s.source.item == self.source.item,
        }
    }
}

/// The source an effect belongs to, as the emitters need it.
#[derive(Clone, Copy)]
struct Owner {
    id: InstanceId,
    class: SourceClass,
    caster: Option<ActorRef>,
}

impl From<&ActiveSource> for Owner {
    fn from(s: &ActiveSource) -> Self {
        Self { id: s.id, class: s.class, caster: s.caster }
    }
}

#[derive(Clone, Copy)]
enum Stage {
    /// The landing, with the magnitude.
    Start(f32),
    /// A rate delivery, with the amount.
    Tick(f32),
    /// The end, with the amount applied.
    End(f32),
}

/// What an innermost archetype does once its stat is resolved.
#[derive(Clone, Copy)]
enum Kind {
    /// `Modify` and `MagnitudeStat`.
    Modify,
    Damage { cap: bool },
    Restore,
    Drain,
    Pool(PoolMode),
}

impl Kind {
    fn of(arch: &Archetype) -> Self {
        match arch.innermost() {
            Archetype::Damage { cap_at_modified, .. } => Self::Damage { cap: *cap_at_modified },
            Archetype::Restore { .. } => Self::Restore,
            Archetype::Drain { .. } => Self::Drain,
            Archetype::Pool { mode, .. } => Self::Pool(*mode),
            // `Cure` and `Request` write no stat: `emit` returns before `Kind`.
            Archetype::Modify { .. }
            | Archetype::MagnitudeStat { .. }
            | Archetype::Absorb { .. }
            | Archetype::Cure { .. }
            | Archetype::Request { .. } => Self::Modify,
        }
    }

    /// The caster's side of an absorb and its modifier sign.
    fn inverse(self) -> (Self, f32) {
        match self {
            Self::Modify => (Self::Modify, -1.0),
            Self::Damage { .. } => (Self::Restore, 1.0),
            Self::Restore => (Self::Damage { cap: false }, 1.0),
            Self::Drain => (Self::Modify, 1.0),
            Self::Pool(mode) => (Self::Pool(mode.inverse()), 1.0),
        }
    }
}

/// One side (target or absorbing caster) of one effect.
struct Side {
    kind: Kind,
    sign: f32,
    stat: StatIdx,
    layer: Layer,
    innate: bool,
    source: SourceId,
}

impl Side {
    /// The stat ops of `stage`. `once`: rate kinds deliver their whole magnitude at the start.
    fn ops(&self, stage: Stage, once: bool, push: &mut impl FnMut(StatOp)) {
        let (stat, source) = (self.stat, self.source);
        let modifier = |layer, value| StatOp::AddModifier(Modifier { stat, layer, op: ModOp::Add, value, source });
        let adjust = |delta, below_zero, above_max| StatOp::PoolAdjust { pool: stat, delta, below_zero, above_max };
        let delivered = match stage {
            Stage::Start(a) if once => Some(a),
            Stage::Tick(a) => Some(a),
            _ => None,
        };
        match self.kind {
            Kind::Modify => match stage {
                Stage::Start(m) => push(modifier(self.layer, self.sign * m)),
                Stage::End(_) => push(StatOp::RemoveSource(source)),
                Stage::Tick(_) => {}
            },
            Kind::Damage { cap } => {
                if let Some(amount) = delivered {
                    push(StatOp::Damage { stat, amount, cap_at_modified: cap });
                }
            }
            Kind::Restore => {
                if let Some(amount) = delivered {
                    push(StatOp::Restore { stat, amount });
                }
            }
            Kind::Pool(mode @ (PoolMode::Hurt | PoolMode::Heal)) => {
                if let Some(a) = delivered {
                    push(adjust(if mode == PoolMode::Hurt { -a } else { a }, false, false));
                }
            }
            Kind::Drain => match stage {
                Stage::Start(m) => push(StatOp::Damage { stat, amount: m, cap_at_modified: false }),
                Stage::End(a) => push(StatOp::Restore { stat, amount: a }),
                Stage::Tick(_) => {}
            },
            Kind::Pool(PoolMode::Drain) => match stage {
                Stage::Start(m) => push(adjust(-m, true, false)),
                Stage::End(a) => push(adjust(a, false, false)),
                Stage::Tick(_) => {}
            },
            Kind::Pool(PoolMode::Fortify) => match stage {
                Stage::Start(m) if self.innate => push(StatOp::PoolModMax { pool: stat, delta: m }),
                Stage::End(a) if self.innate => push(StatOp::PoolModMax { pool: stat, delta: -a }),
                Stage::Start(m) => {
                    push(modifier(Layer::Temporary, m));
                    push(adjust(m, false, true));
                }
                // The current first, while the max still holds the modifier.
                Stage::End(a) => {
                    push(adjust(-a, false, true));
                    push(StatOp::RemoveSource(source));
                }
                Stage::Tick(_) => {}
            },
        }
    }
}

impl Step<'_> {
    fn cause(&self, source: SourceId) -> Cause {
        Cause { kind: CauseKind::Effect, source: Some(source), tick: self.tick }
    }

    /// The stat `def` writes for an entry with `arg`, if the table has it (`None` for `Cure`
    /// and `Request`, which write none).
    fn resolve(&self, def: &EffectDef, arg: Option<&ArgRef>) -> Option<StatIdx> {
        match def.archetype.stat_target()? {
            StatTarget::Fixed(key) => self.stats.idx(key),
            StatTarget::Arg => {
                let Some(ArgRef::Stat(key)) = arg else { return None };
                let idx = self.stats.idx(key)?;
                match &def.arg {
                    ArgKind::Stat(group) if self.stats.def(idx).group != *group => None,
                    _ => Some(idx),
                }
            }
        }
    }

    /// Emits one effect's ops for `stage`: on the target, and the inverse on the caster for an
    /// absorb. Every op carries the effect's own cause.
    fn emit(&self, who: Owner, def: &EffectDef, e: &ActiveEffect, stage: Stage, out: &mut EffectOut) {
        match def.archetype.innermost() {
            Archetype::Request { kind } => {
                let phase = match stage {
                    Stage::Start(_) => RequestPhase::Start,
                    Stage::End(_) => RequestPhase::End,
                    Stage::Tick(_) => return,
                };
                out.requests.push(EffectRequest::Effect {
                    kind: kind.clone(),
                    phase,
                    target: self.target,
                    caster: who.caster,
                    instance: who.id,
                    index: e.index,
                    effect: e.effect.clone(),
                    magnitude: e.magnitude,
                    arg: e.arg.clone(),
                    tick: self.tick,
                });
                return;
            }
            // Cures act when they land (`inflict`); nothing to emit.
            Archetype::Cure { .. } => return,
            _ => {}
        }
        let Some(stat) = self.resolve(def, e.arg.as_ref()) else { return };
        let source = who.id.source_id(e.index);
        let cause = self.cause(source);
        let once = !def.is_rate();
        let kind = Kind::of(&def.archetype);
        let innate = who.class == SourceClass::Innate;
        let target = Side { kind, sign: 1.0, stat, layer: who.class.layer(), innate, source };
        let to = self.target;
        target.ops(stage, once, &mut |op| out.stat_ops.push((to, op, cause)));
        if let (Archetype::Absorb { .. }, Some(caster)) = (&def.archetype, who.caster) {
            let (kind, sign) = kind.inverse();
            let side = Side { kind, sign, ..target };
            side.ops(stage, once, &mut |op| out.stat_ops.push((caster, op, cause)));
        }
    }

    /// Ends one running effect: its end ops, then `Ended`. A def missing from the table still
    /// drops the effect's modifiers.
    fn end_effect(&self, who: Owner, e: &mut ActiveEffect, reason: EndReason, out: &mut EffectOut) {
        if !e.phase.is_live() {
            return;
        }
        if let Some(def) = self.defs.get(&e.effect) {
            self.emit(who, def, e, Stage::End(e.applied), out);
        } else {
            let source = who.id.source_id(e.index);
            out.stat_ops.push((self.target, StatOp::RemoveSource(source), self.cause(source)));
        }
        e.phase = Phase::Ended;
        out.events.push(EffectEvent::Ended {
            target: self.target,
            instance: who.id,
            effect: e.effect.clone(),
            reason,
            tick: self.tick,
        });
    }

    fn end_instance(&self, set: &mut ActiveSet, id: InstanceId, reason: EndReason, out: &mut EffectOut) {
        let Some(pos) = set.sources.iter().position(|s| s.id == id) else { return };
        let mut src = set.sources.remove(pos);
        let who = Owner::from(&src);
        for e in &mut src.effects {
            self.end_effect(who, e, reason, out);
        }
    }

    /// `n` ticks in one step: rates deliver `magnitude · dt · k` (k = ticks left, at most `n`),
    /// expiries fire in instance order.
    fn advance(&self, set: &mut ActiveSet, n: u32, out: &mut EffectOut) {
        if n == 0 {
            return;
        }
        for src in &mut set.sources {
            let who = Owner::from(&*src);
            for e in &mut src.effects {
                if e.phase != Phase::Running {
                    continue;
                }
                let k = e.duration.map_or(n, |d| n.min(d.saturating_sub(e.elapsed)));
                if let Some(def) = self.defs.get(&e.effect)
                    && def.is_rate()
                    && k > 0
                {
                    let amount = e.magnitude * self.dt * ticks_f32(k);
                    self.emit(who, def, e, Stage::Tick(amount), out);
                    e.applied += amount;
                }
                if let Some(d) = e.duration {
                    e.elapsed = e.elapsed.saturating_add(k);
                    if e.elapsed >= d {
                        self.end_effect(who, e, EndReason::Expired, out);
                    }
                }
            }
        }
        set.sources.retain(ActiveSource::is_live);
    }

    /// Stacking for one entry (pipeline step 2): whether the entry lands. `Replace` ends the
    /// matches first.
    fn stack(&self, set: &mut ActiveSet, policy: StackPolicy, l: &Landing<'_>, entry: &EffectEntry, out: &mut EffectOut) -> bool {
        let same = |e: &ActiveEffect| e.phase == Phase::Running && e.effect == entry.effect && e.arg == entry.arg;
        let mut matches = set.sources.iter_mut().filter(|s| l.same_stack(s));
        match policy {
            StackPolicy::Replace => {
                for src in matches {
                    let who = Owner::from(&*src);
                    for e in src.effects.iter_mut().filter(|e| same(e)) {
                        self.end_effect(who, e, EndReason::Replaced, out);
                    }
                }
                set.sources.retain(ActiveSource::is_live);
                true
            }
            StackPolicy::Refresh => {
                let mut found = false;
                for src in matches {
                    for e in src.effects.iter_mut().filter(|e| same(e)) {
                        e.elapsed = 0;
                        found = true;
                    }
                }
                !found
            }
            StackPolicy::Stack { limit } => {
                matches.map(|s| s.effects.iter().filter(|e| same(e)).count()).sum::<usize>() < usize::from(limit)
            }
            StackPolicy::Ignore => !matches.any(|s| s.effects.iter().any(same)),
        }
    }

    /// The application pipeline (effects.md): recast, stacking, magnitude and resist per entry,
    /// then one instance with its start ops. Instant entries end the same step; a landing
    /// `Cure` acts on the set at once. Returns the book sources a cure ended (the caller drops
    /// them from the book).
    #[allow(clippy::too_many_lines, reason = "the application pipeline reads best in one place")]
    fn inflict(
        &self,
        land: &dyn LandModel,
        set: &mut ActiveSet,
        l: &Landing<'_>,
        ids: &mut EffectIdAlloc,
        out: &mut EffectOut,
    ) -> SmallVec<[RecordId; 2]> {
        let mut cured: SmallVec<[RecordId; 2]> = SmallVec::new();
        let (target, tick) = (self.target, self.tick);
        let reject = |out: &mut EffectOut, entry: &EffectEntry, reason| {
            out.events.push(EffectEvent::Rejected { target, effect: entry.effect.clone(), reason, tick });
        };
        let non_recastable = l.entries.iter().any(|en| self.defs.get(&en.effect).is_some_and(|d| d.flags.non_recastable));
        if non_recastable && set.sources.iter().any(|s| l.same_stack(s) && s.is_live()) {
            out.events.push(EffectEvent::CastRefused { target, source: l.source.record.clone(), caster: l.caster, tick });
            return cured;
        }
        let mut kept: SmallVec<[(u8, &EffectEntry, &EffectDef, f32); 4]> = SmallVec::new();
        for (i, entry) in l.entries.iter().enumerate().take(MAX_EFFECTS_PER_SOURCE) {
            let Ok(index) = u8::try_from(i) else { break };
            let Some(def) = self.defs.get(&entry.effect) else {
                reject(out, entry, Rejection::UnknownEffect);
                continue;
            };
            if def.flags.caster_linked && l.caster.is_none() {
                reject(out, entry, Rejection::NoCaster);
                continue;
            }
            if def.archetype.stat_target().is_some() && self.resolve(def, entry.arg.as_ref()).is_none() {
                reject(out, entry, Rejection::UnknownStat);
                continue;
            }
            if !self.stack(set, def.stack_policy(l.class), l, entry, out) {
                continue;
            }
            let rolls = [roll(l.rolls, index, 0), roll(l.rolls, index, 1)];
            let mut magnitude = land.magnitude(&RollQuery { def, entry, rolls });
            if !l.flags.ignore_resist {
                let mult = land.resist(&ResistQuery { def, entry, caster: l.caster, rolls, target: None });
                let resisted = if def.flags.no_magnitude { rolls[1] >= mult } else { mult <= 0.0 };
                if resisted {
                    out.events.push(EffectEvent::Resisted { target, effect: entry.effect.clone(), caster: l.caster, tick });
                    continue;
                }
                if !def.flags.no_magnitude {
                    magnitude *= mult;
                }
            }
            kept.push((index, entry, def, magnitude));
        }
        if kept.is_empty() {
            return cured;
        }
        let id = ids.alloc();
        let who = Owner { id, class: l.class, caster: l.caster };
        let mut src = ActiveSource {
            id,
            source: l.source.clone(),
            class: l.class,
            caster: l.caster,
            started: tick,
            flags: l.flags,
            effects: SmallVec::new(),
        };
        for (index, entry, def, magnitude) in kept {
            let cure = match def.archetype.innermost() {
                Archetype::Cure { filter } => Some(filter),
                _ => None,
            };
            let instant =
                cure.is_some() || (!l.class.lives_with_source() && (def.flags.no_duration || entry.duration <= 0.0));
            let duration = if l.class.lives_with_source() || instant {
                None
            } else {
                Some(ticks_for(entry.duration, self.dt).max(1))
            };
            let mut e = ActiveEffect {
                index,
                effect: entry.effect.clone(),
                arg: entry.arg.clone(),
                magnitude,
                duration,
                elapsed: 0,
                applied: 0.0,
                phase: Phase::Running,
            };
            // An instant rate effect delivers its whole magnitude once.
            let start = if instant && def.is_rate() { Stage::Tick(magnitude) } else { Stage::Start(magnitude) };
            self.emit(who, def, &e, start, out);
            if !def.is_rate() || instant {
                e.applied = magnitude;
            }
            out.events.push(EffectEvent::Applied { target, instance: id, effect: e.effect.clone(), magnitude, caster: l.caster, tick });
            if let Some(filter) = cure {
                let seed = RollSeed(l.rolls.0 ^ (u64::from(index) << 56));
                for record in self.cure(set, filter, magnitude, seed, out) {
                    if !cured.contains(&record) {
                        cured.push(record);
                    }
                }
            }
            if instant {
                self.end_effect(who, &mut e, EndReason::Expired, out);
            }
            src.effects.push(e);
        }
        if src.is_live() {
            set.sources.push(src);
        }
        cured
    }

    /// Drops cured innate and affliction sources from the book (their instances already ended).
    fn unlearn(&self, book: &mut SpellBook, cured: &[RecordId], out: &mut EffectOut) {
        for record in cured {
            if book.remove(record) {
                let (owner, tick) = (self.target, self.tick);
                out.events.push(EffectEvent::BookChanged { owner, source: record.clone(), added: false, tick });
            }
        }
    }

    /// Ends what `filter` matches (effects.md "Removal"); `magnitude` is the cure's (dispel
    /// chance). Returns the innate and affliction records whose instances it ended.
    fn cure(
        &self,
        set: &mut ActiveSet,
        filter: &CureFilter,
        magnitude: f32,
        seed: RollSeed,
        out: &mut EffectOut,
    ) -> SmallVec<[RecordId; 2]> {
        let mut book: SmallVec<[RecordId; 2]> = SmallVec::new();
        match filter {
            CureFilter::Effect(key) => {
                for src in &mut set.sources {
                    if src.class.lives_with_source() {
                        continue;
                    }
                    let who = Owner::from(&*src);
                    for e in src.effects.iter_mut().filter(|e| e.phase == Phase::Running && e.effect == *key) {
                        self.end_effect(who, e, EndReason::Cured, out);
                    }
                }
                set.sources.retain(ActiveSource::is_live);
            }
            CureFilter::SourceTag(tag) => {
                let tagged = |s: &&ActiveSource| self.src_defs.get(&s.source.record).is_some_and(|d| d.tags.contains(tag));
                let hits: SmallVec<[(InstanceId, SourceClass, RecordId); 2]> =
                    set.sources.iter().filter(tagged).map(|s| (s.id, s.class, s.source.record.clone())).collect();
                for (id, class, record) in hits {
                    self.end_instance(set, id, EndReason::Cured, out);
                    if matches!(class, SourceClass::Innate | SourceClass::Affliction) && !book.contains(&record) {
                        book.push(record);
                    }
                }
            }
            CureFilter::Dispel { harmful_only, chance_from_magnitude } => {
                let chance = if *chance_from_magnitude { magnitude / 100.0 } else { 1.0 };
                let harmful = |s: &ActiveSource| {
                    s.effects.iter().any(|e| e.phase.is_live() && self.defs.get(&e.effect).is_some_and(|d| d.flags.harmful))
                };
                let hits: SmallVec<[InstanceId; 4]> = set
                    .sources
                    .iter()
                    .filter(|s| matches!(s.class, SourceClass::Cast | SourceClass::Power))
                    .filter(|s| !*harmful_only || harmful(s))
                    .filter(|s| roll(RollSeed(seed.0 ^ s.id.0), 0, 3) < chance)
                    .map(|s| s.id)
                    .collect();
                for id in hits {
                    self.end_instance(set, id, EndReason::Dispelled, out);
                }
            }
        }
        book
    }

    fn book_reject(&self, source: &RecordId, reason: BookRejection, out: &mut EffectOut) {
        let (owner, tick) = (self.target, self.tick);
        out.events.push(EffectEvent::BookRejected { owner, source: source.clone(), reason, tick });
    }

    /// `Learn`: into the book; an innate or affliction source starts its instance with
    /// resistances ignored and the owner as caster (`OpenMW` `ActiveSpellParams(spell, actor,
    /// true)`), rolled from the instance id it is about to get.
    fn learn(
        &self,
        land: &dyn LandModel,
        set: &mut ActiveSet,
        book: &mut SpellBook,
        source: &RecordId,
        ids: &mut EffectIdAlloc,
        out: &mut EffectOut,
    ) {
        let Some(def) = self.src_defs.get(source) else {
            return self.book_reject(source, BookRejection::UnknownSource, out);
        };
        if !def.is_book_source() {
            return self.book_reject(source, BookRejection::NotABookSource, out);
        }
        if !book.add(source) {
            return;
        }
        let (owner, tick) = (self.target, self.tick);
        out.events.push(EffectEvent::BookChanged { owner, source: source.clone(), added: true, tick });
        if def.lives_in_book() {
            let sref = def.source_ref();
            let flags = InstanceFlags { ignore_resist: true, ..InstanceFlags::default() };
            let rolls = RollSeed(ids.next);
            let landing =
                Landing { source: &sref, class: def.class, caster: Some(owner), entries: &def.effects, flags, rolls };
            let cured = self.inflict(land, set, &landing, ids, out);
            self.unlearn(book, &cured, out);
        }
    }

    /// `Forget`: out of the book; the record's innate and affliction instances end
    /// (`SourceRemoved`). Instances of a forgotten spell already landed run their course.
    fn forget(&self, set: &mut ActiveSet, book: &mut SpellBook, source: &RecordId, out: &mut EffectOut) {
        if !book.remove(source) {
            return;
        }
        let (owner, tick) = (self.target, self.tick);
        out.events.push(EffectEvent::BookChanged { owner, source: source.clone(), added: false, tick });
        let ids: SmallVec<[InstanceId; 2]> = set
            .sources
            .iter()
            .filter(|s| s.source.record == *source && matches!(s.class, SourceClass::Innate | SourceClass::Affliction))
            .map(|s| s.id)
            .collect();
        for id in ids {
            self.end_instance(set, id, EndReason::SourceRemoved, out);
        }
    }

    /// `Select`: a known spell or power, or none.
    fn select(&self, book: &mut SpellBook, source: Option<&RecordId>, out: &mut EffectOut) {
        match source {
            None => book.selected = None,
            Some(id) if !book.knows(id) => self.book_reject(id, BookRejection::NotKnown, out),
            Some(id) if !self.src_defs.get(id).is_some_and(SourceDef::is_castable) => {
                self.book_reject(id, BookRejection::NotABookSource, out);
            }
            Some(id) => book.selected = Some(id.clone()),
        }
    }

    /// `Cast` (effects.md "Casting"): book, recharge, the cast model's plan, the cost on the
    /// caster's `Mana` role, the chance roll (`roll(seed, 0, 2) < chance`, else `Fizzle` with
    /// the cost spent), then cooldown, power use and delivery. Refusals spend nothing.
    fn cast(
        &self,
        env: &EffectEnv<'_>,
        set: &mut ActiveSet,
        book: &mut SpellBook,
        c: &CastOp<'_>,
        ids: &mut EffectIdAlloc,
        out: &mut EffectOut,
    ) {
        let (caster, tick) = (self.target, self.tick);
        let fail = |out: &mut EffectOut, reason| {
            out.events.push(EffectEvent::CastFailed { caster, source: c.source.clone(), reason, tick });
        };
        let Some(def) = self.src_defs.get(c.source) else { return fail(out, CastFailReason::UnknownSource) };
        if !book.knows(c.source) {
            return fail(out, CastFailReason::NotKnown);
        }
        let Some(sheet) = env.sheet.filter(|_| def.is_castable()) else { return fail(out, CastFailReason::NotCastable) };
        if book.recharging(c.source, def.recharge, env.time) {
            return fail(out, CastFailReason::Recharging);
        }
        let query = CastQuery {
            defs: self.defs,
            source: &def.id,
            class: def.class,
            entries: &def.effects,
            cost: def.cost,
            always_succeeds: def.flags.always_succeeds,
            cooldown: def.cooldown,
            ready_at: book.ready_at(c.source),
            tick,
            caster: sheet,
            skill: def.skill.as_ref().and_then(|k| self.stats.idx(k)),
        };
        let (cost, chance, cooldown) = match env.cast.plan(&query) {
            CastPlan::Refused(reason) => return fail(out, reason),
            CastPlan::Ready { cost, chance, cooldown } => (cost, chance, cooldown),
        };
        if cost > 0.0
            && let Some(mana) = self.stats.role(Role::Mana)
        {
            let spend = StatOp::PoolAdjust { pool: mana, delta: -cost, below_zero: false, above_max: false };
            out.stat_ops.push((caster, spend, Cause { kind: CauseKind::Effect, source: None, tick }));
        }
        let chance = if def.flags.always_succeeds { 1.0 } else { chance };
        if roll(c.rolls, 0, 2) >= chance {
            return fail(out, CastFailReason::Fizzle);
        }
        if cooldown > 0 {
            book.cooldowns.insert(def.id.clone(), tick.saturating_add(cooldown));
        }
        if def.recharge.is_some() {
            book.power_used.insert(def.id.clone(), env.time);
            out.events.push(EffectEvent::PowerUsed { caster, source: def.id.clone(), time: env.time, tick });
        }
        out.events.push(EffectEvent::CastSucceeded { caster, source: def.id.clone(), tick });
        let (own, requests) = split_delivery(def, c, caster);
        if !own.is_empty() {
            let sref = def.source_ref();
            let flags = InstanceFlags::default();
            let landing =
                Landing { source: &sref, class: def.class, caster: Some(caster), entries: &own, flags, rolls: c.rolls };
            let cured = self.inflict(env.land, set, &landing, ids, out);
            self.unlearn(book, &cured, out);
        }
        out.requests.extend(requests);
    }

    /// A handler's `Invalidate`: a `Request` effect ends `Invalid` with no `End` request; any
    /// other effect ends normally (its stats revert) and is marked `Invalid`.
    fn invalidate(&self, set: &mut ActiveSet, instance: InstanceId, index: u8, out: &mut EffectOut) {
        let Some(src) = set.sources.iter_mut().find(|s| s.id == instance) else { return };
        let who = Owner::from(&*src);
        let Some(e) = src.effects.iter_mut().find(|e| e.index == index && e.phase.is_live()) else { return };
        let request = self.defs.get(&e.effect).is_some_and(|d| matches!(d.archetype.innermost(), Archetype::Request { .. }));
        if request {
            out.events.push(EffectEvent::Ended {
                target: self.target,
                instance,
                effect: e.effect.clone(),
                reason: EndReason::Invalid,
                tick: self.tick,
            });
        } else {
            self.end_effect(who, e, EndReason::Invalid, out);
        }
        e.phase = Phase::Invalid;
        set.sources.retain(ActiveSource::is_live);
    }

    /// Some live effect still needs ticks: a duration or a rate.
    fn timed(&self, set: &ActiveSet) -> bool {
        set.sources.iter().flat_map(|s| &s.effects).any(|e| {
            e.phase == Phase::Running && (e.duration.is_some() || self.defs.get(&e.effect).is_some_and(EffectDef::is_rate))
        })
    }
}
