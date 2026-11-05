//! Thin Bevy shell for `sf_stats` (`docs/specs/stats.md` "Architecture", "Per-tick pipeline").
//! The rules live in `sf_stats` (Bevy-free) and the content's models; this crate schedules them.
//!
//! A character with stats carries a [`Stats`] component: its [`StatOwner`] (net rule 7: saves
//! and messages name the owner, never the `Entity`), its [`Sheet`] and the [`Inputs`] the
//! content's models read (the game keeps them current). The content inserts [`StatTableRes`]
//! and [`StatRules`]; without both, the systems do nothing.
//!
//! Per fixed tick ([`StatSystems`] in `FixedUpdate`, chained):
//! 1. `FixedFirst`: [`StatTick`] advances (the tick events carry).
//! 2. [`StatSystems::Apply`]: every [`StatCommand`] written since the last tick is applied in
//!    write order, one `sf_stats::apply` call per command (atomic per command, stats.md
//!    "Operations"). A command for an owner without a sheet is dropped with a warning.
//! 3. [`StatSystems::Combat`]: the content's combat glue (vitals differences back, stats.md
//!    "Consumers"); empty here. The content orders it after its combat step and orders
//!    `Apply` before its vitals refresh.
//! 4. [`StatSystems::Tick`]: `sf_stats::tick` on every sheet (regen, recompute; a clean sheet
//!    with full pools returns at once).
//! 5. [`StatSystems::Events`]: the tick's events go out as [`StatMessage`]s: the commands'
//!    events in command order, then the tick's events by owner (stable), so the order never
//!    depends on query iteration.
//!
//! Presentation (HUD, UI) reads sheets and messages in `Update`. [`Stats::apply`] is the
//! synchronous path for a host step that must read its own write in the same frame (a script,
//! stats.md "Consumers"); its events go through [`PendingStatEvents`].

use std::collections::HashMap;
use std::sync::Arc;

use bevy_app::{App, FixedFirst, FixedUpdate, Plugin};
use bevy_ecs::prelude::*;
use bevy_log::warn;
use bevy_time::Time;
pub use sf_stats;
use sf_stats::{ApplyCtx, Cause, CauseKind, Inputs, Rules, Sheet, SheetRead, SourceId, StatEvent, StatOp, StatOwner, StatTable, TickCtx};

#[cfg(test)]
mod tests;

/// The stat definitions (content data), shared.
#[derive(Resource, Clone, Debug)]
pub struct StatTableRes(pub Arc<StatTable>);

/// The rule set in use (chosen per new game).
#[derive(Resource, Clone)]
pub struct StatRules(pub Arc<Rules>);

/// Fixed ticks since the app started: the `tick` of the [`Cause`]s this crate builds.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StatTick(pub u32);

/// One character's stats.
#[derive(Component, Clone, Debug)]
pub struct Stats {
    pub owner: StatOwner,
    pub sheet: Sheet,
    /// External numbers the content's models read (carried weight, combatant, is-player); the
    /// content keeps them current.
    pub inputs: Inputs,
}

impl Stats {
    pub fn new(owner: StatOwner, sheet: Sheet, inputs: Inputs) -> Self {
        Self { owner, sheet, inputs }
    }

    /// Reads through the table.
    pub fn view<'a>(&'a self, table: &'a StatTable) -> SheetRead<'a> {
        self.sheet.view(table)
    }

    /// Applies `ops` now (one atomic `sf_stats::apply`) and appends the events to `out`.
    /// Returns how many ops were rejected.
    pub fn apply(&mut self, table: &StatTable, rules: &Rules, ops: &[StatOp], cause: Cause, out: &mut Vec<StatEvent>) -> usize {
        let ctx = ApplyCtx { owner: self.owner, cause, inputs: &self.inputs };
        sf_stats::apply(table, rules, &mut self.sheet, ops, &ctx, out)
    }
}

/// A request to change one sheet (net rule 2: the host turns validated requests into these).
/// Applied on the next fixed tick in write order.
#[derive(Message, Clone, Debug, PartialEq)]
pub struct StatCommand {
    pub owner: StatOwner,
    pub cause: CauseKind,
    pub source: Option<SourceId>,
    pub ops: Vec<StatOp>,
}

impl StatCommand {
    pub fn new(owner: StatOwner, cause: CauseKind, ops: Vec<StatOp>) -> Self {
        Self { owner, cause, source: None, ops }
    }
}

/// A sheet change, published in [`StatSystems::Events`].
#[derive(Message, Clone, Debug, PartialEq)]
pub struct StatMessage(pub StatEvent);

/// Events waiting for [`StatSystems::Events`], in publication order. Synchronous host steps
/// ([`Stats::apply`]) append theirs here.
#[derive(Resource, Clone, Debug, Default)]
pub struct PendingStatEvents(pub Vec<StatEvent>);

/// Owner → entity of every [`Stats`] (kept by observers). Two entities with one owner: the
/// later insert wins, with a warning.
#[derive(Resource, Clone, Debug, Default)]
pub struct StatIndex(HashMap<StatOwner, Entity>);

impl StatIndex {
    pub fn get(&self, owner: StatOwner) -> Option<Entity> {
        self.0.get(&owner).copied()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// The stat sets in `FixedUpdate` (see the crate docs).
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StatSystems {
    /// Drain the [`StatCommand`]s.
    Apply,
    /// The content's combat glue (vitals differences back).
    Combat,
    /// Regeneration and recompute.
    Tick,
    /// Publish [`StatMessage`]s.
    Events,
}

/// The stats layer. The content inserts [`StatTableRes`] and [`StatRules`].
pub struct SfStatsPlugin;

impl Plugin for SfStatsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StatTick>()
            .init_resource::<StatIndex>()
            .init_resource::<PendingStatEvents>()
            .add_message::<StatCommand>()
            .add_message::<StatMessage>()
            .add_observer(index_insert)
            .add_observer(index_discard)
            .configure_sets(FixedUpdate, (StatSystems::Apply, StatSystems::Combat, StatSystems::Tick, StatSystems::Events).chain())
            .add_systems(FixedFirst, advance_tick)
            .add_systems(FixedUpdate, apply_commands.in_set(StatSystems::Apply))
            .add_systems(FixedUpdate, tick_sheets.in_set(StatSystems::Tick))
            .add_systems(FixedUpdate, publish.in_set(StatSystems::Events));
    }
}

fn index_insert(insert: On<Insert<Stats>>, stats: Query<&Stats>, mut index: ResMut<StatIndex>) {
    let Ok(s) = stats.get(insert.entity) else { return };
    if let Some(old) = index.0.insert(s.owner, insert.entity)
        && old != insert.entity
    {
        warn!("stats: {:?} is on {old} and {}; the later wins", s.owner, insert.entity);
    }
}

fn index_discard(replace: On<Discard<Stats>>, stats: Query<&Stats>, mut index: ResMut<StatIndex>) {
    let Ok(s) = stats.get(replace.entity) else { return };
    if index.0.get(&s.owner) == Some(&replace.entity) {
        index.0.remove(&s.owner);
    }
}

fn advance_tick(mut tick: ResMut<StatTick>) {
    tick.0 = tick.0.wrapping_add(1);
}

#[allow(clippy::needless_pass_by_value)] // system parameters
fn apply_commands(
    table: Option<Res<StatTableRes>>,
    rules: Option<Res<StatRules>>,
    tick: Res<StatTick>,
    index: Res<StatIndex>,
    mut commands: MessageReader<StatCommand>,
    mut stats: Query<&mut Stats>,
    mut pending: ResMut<PendingStatEvents>,
) {
    let (Some(table), Some(rules)) = (table, rules) else {
        commands.clear();
        return;
    };
    for c in commands.read() {
        let Some(mut s) = index.get(c.owner).and_then(|e| stats.get_mut(e).ok()) else {
            warn!("stats: command for {:?}, which has no sheet; dropped", c.owner);
            continue;
        };
        let cause = Cause { kind: c.cause, source: c.source, tick: tick.0 };
        let rejected = s.apply(&table.0, &rules.0, &c.ops, cause, &mut pending.0);
        if rejected > 0 {
            warn!("stats: {rejected} of {} ops for {:?} rejected", c.ops.len(), c.owner);
        }
    }
}

#[allow(clippy::needless_pass_by_value)] // system parameters
fn tick_sheets(
    table: Option<Res<StatTableRes>>,
    rules: Option<Res<StatRules>>,
    tick: Res<StatTick>,
    time: Res<Time>,
    mut stats: Query<&mut Stats>,
    mut pending: ResMut<PendingStatEvents>,
    mut out: Local<Vec<StatEvent>>,
) {
    let (Some(table), Some(rules)) = (table, rules) else { return };
    let dt = time.delta_secs();
    out.clear();
    for mut s in &mut stats {
        let s = &mut *s;
        let ctx = TickCtx { owner: s.owner, tick: tick.0, dt, inputs: &s.inputs };
        sf_stats::tick(&table.0, &rules.0, &mut s.sheet, &ctx, &mut out);
    }
    // Query order is not an owner order: sort (stable, so one owner's events keep theirs).
    out.sort_by_key(StatEvent::owner);
    pending.0.append(&mut out);
}

fn publish(mut pending: ResMut<PendingStatEvents>, mut out: MessageWriter<StatMessage>) {
    out.write_batch(pending.0.drain(..).map(StatMessage));
}
