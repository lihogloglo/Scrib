//! Saves (effects.md "Saves"): one character's [`EffectsSave`] (active set and book, with the
//! tick it was taken on), its restore with drift reported, and [`reregister`], the modifiers a
//! loaded sheet gets back from the restored set (stats.md "Saves", load order).

use serde::{Deserialize, Serialize};
use sf_records::RecordId;
use sf_stats::{Cause, StatOp, StatOwner, StatTable};

use crate::active::{ActiveSet, EffectIdAlloc, Phase};
use crate::book::{SourceTable, SpellBook};
use crate::def::{EffectDef, EffectKey, EffectTable};

/// One character's effects in a save: what was active, what it knew. Fixed-tick fields (a
/// source's `started`, the book's cooldowns) are as of `tick` and are rebased on restore, so the
/// save does not depend on the host's tick counter; durations count `elapsed` ticks and need no
/// rebase. Power recharges are world minutes (`GameMinutes`, the world clock's own save).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EffectsSave {
    /// The fixed tick the save was taken on.
    pub tick: u32,
    pub active: ActiveSet,
    pub book: SpellBook,
}

/// What a restore could not place (effects.md "Migrations / drift"): effects whose key the table
/// no longer has (their mod was removed; the effect is dropped, a source left with nothing live
/// too) and book entries the source table no longer has (dropped).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct EffectsLoadReport {
    pub dropped_effects: Vec<EffectKey>,
    pub dropped_sources: Vec<RecordId>,
}

impl EffectsLoadReport {
    pub fn is_empty(&self) -> bool {
        self.dropped_effects.is_empty() && self.dropped_sources.is_empty()
    }
}

impl EffectsSave {
    /// A save of `active` and `book` at fixed tick `tick`.
    pub fn capture(active: &ActiveSet, book: &SpellBook, tick: u32) -> Self {
        Self {
            tick,
            active: active.clone(),
            book: book.clone(),
        }
    }

    /// Nothing active and an empty book (nothing worth a blob).
    pub fn is_empty(&self) -> bool {
        self.active.is_empty() && self.book == SpellBook::default()
    }

    /// The active set and book back at fixed tick `now`: `started` and cooldowns rebased from
    /// [`Self::tick`] (a cooldown already over is dropped), unknown effects and book entries
    /// dropped and reported, and `ids` moved past every restored instance (a save whose world
    /// counter was lost never reissues a live id).
    pub fn restore(
        self,
        defs: &EffectTable,
        sources: &SourceTable,
        now: u32,
        ids: &mut EffectIdAlloc,
    ) -> (ActiveSet, SpellBook, EffectsLoadReport) {
        let Self {
            tick,
            mut active,
            mut book,
        } = self;
        let mut report = EffectsLoadReport::default();
        for src in &mut active.sources {
            src.started = now.saturating_sub(tick.saturating_sub(src.started));
            src.effects.retain(|e| {
                let known = defs.get(&e.effect).is_some();
                if !known && e.phase.is_live() {
                    report.dropped_effects.push(e.effect.clone());
                }
                known
            });
            ids.next = ids.next.max(src.id.0.saturating_add(1));
        }
        active.sources.retain(crate::ActiveSource::is_live);
        active.timed = timed(defs, &active);
        book.known.retain(|id| {
            let known = sources.get(id).is_some();
            if !known {
                report.dropped_sources.push(id.clone());
            }
            known
        });
        if book
            .selected
            .as_ref()
            .is_some_and(|s| !book.known.contains(s))
        {
            book.selected = None;
        }
        book.cooldowns = book
            .cooldowns
            .into_iter()
            .filter_map(|(id, ready)| {
                let left = ready.saturating_sub(tick);
                (left > 0).then(|| (id, now.saturating_add(left)))
            })
            .collect();
        (active, book, report)
    }
}

/// Some running effect has a duration or a rate (the [`ActiveSet::is_timed`] flag).
fn timed(defs: &EffectTable, set: &ActiveSet) -> bool {
    set.sources.iter().flat_map(|s| &s.effects).any(|e| {
        e.phase == Phase::Running
            && (e.duration.is_some() || defs.get(&e.effect).is_some_and(EffectDef::is_rate))
    })
}

/// The modifiers `set`'s running effects hold, for a sheet loaded from a save (stats.md "Saves":
/// sheets load in building mode, sources re-add their modifiers, then one settle). Only
/// `AddModifier` ops: damage, restores and pool currents are already in the saved sheet, so a
/// shifting pool modifier ([`StatOp::AddModifierShift`], a pool fortify) comes out as a plain
/// `AddModifier` (its shift is in the saved current). No events, no requests (a `Request`
/// effect's handler state is the game's own save). An absorb's caster side comes out under the caster's owner, for the
/// host to route to that sheet if it is loaded. Same causes as the landing (`tick` the load's).
pub fn reregister(
    defs: &EffectTable,
    stats: &StatTable,
    set: &ActiveSet,
    target: StatOwner,
    tick: u32,
    out: &mut Vec<(StatOwner, StatOp, Cause)>,
) {
    crate::step::start_ops(defs, stats, set, target, tick, &mut |owner, op, cause| {
        match op {
            StatOp::AddModifier(m) | StatOp::AddModifierShift(m) => out.push((owner, StatOp::AddModifier(m), cause)),
            _ => {}
        }
    });
}
