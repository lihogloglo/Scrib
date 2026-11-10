//! Thin Bevy shell for `sf_effects` (`docs/specs/effects.md` "Architecture", "Per-tick
//! pipeline"). The rules live in `sf_effects` (Bevy-free) and the content's tables; this crate
//! schedules them and routes their stat ops into `sf_stats_bevy`.
//!
//! A character with effects carries an [`Effects`] component: its [`ActorRef`] (net rule 7:
//! saves and messages name the owner, never the `Entity`), its [`ActiveSet`] and its
//! [`SpellBook`]. It sits beside the character's `Stats` (the content attaches both); the
//! effect step reads that sheet for casts and writes it only through `StatCommand`s. The content
//! inserts [`EffectTableRes`]; without it (or without `StatTableRes`) the systems do nothing.
//! [`SourceTableRes`] (empty) and [`EffectRulesRes`] (the modern defaults) start with defaults.
//!
//! Per fixed tick ([`EffectSystems`] in `FixedUpdate`, chained, all before
//! `StatSystems::Apply`, so the stat ops land on the sheets in the same tick):
//! 1. [`EffectSystems::Commands`]: every [`EffectCommand`] written since the last tick runs in
//!    write order through `sf_effects::apply_book` on its target (casts, item uses, script and
//!    book ops). A command for an owner without [`Effects`] is dropped with a warning.
//! 2. [`EffectSystems::Tick`]: `sf_effects::tick` on every set with timed effects (rates,
//!    expiries), in owner order so the output never depends on query iteration; then the
//!    commands' and the tick's requests go out as [`EffectRequestMessage`]s.
//! 3. [`EffectSystems::Requests`]: the game's request handlers (empty here); their answers come
//!    back as [`EffectCommand`]s, run on the next tick.
//! 4. [`EffectSystems::Out`]: the step's stat ops into the same tick's `StatCommand` queue (one
//!    command per run of ops with one owner and one cause), and its events as
//!    [`EffectMessage`]s: the commands' events in command order, then the tick's by owner.
//!
//! **Clocks** (effects.md "Per-tick pipeline", Clocks): durations count fixed ticks (`Time<Fixed>`, which advances
//! only while the simulation runs; `StatTick` numbers them); power recharges read
//! [`EffectClock`], world minutes the content keeps current.
//!
//! Presentation (VFX, sounds, HUD icons) reads [`Effects`] and [`EffectMessage`]s in `Update`.

use std::collections::HashMap;
use std::sync::Arc;

use bevy_app::{App, FixedUpdate, Plugin};
use bevy_ecs::prelude::*;
use bevy_log::warn;
use bevy_time::Time;
pub use sf_effects;
use sf_effects::{
    ActiveSet, ActorRef, EffectCtx, EffectEnv, EffectEvent, EffectIdAlloc, EffectOp, EffectOut,
    EffectRequest, EffectRules, EffectTable, GameMinutes, SourceTable, SpellBook,
};
use sf_stats_bevy::sf_stats::{Cause, StatOp, StatOwner};
use sf_stats_bevy::{StatCommand, StatSystems, StatTableRes, StatTick, Stats};

#[cfg(test)]
mod tests;

/// The effect definitions (content data), shared.
#[derive(Resource, Clone, Debug)]
pub struct EffectTableRes(pub Arc<EffectTable>);

/// The effect sources (spells, powers, abilities, potions) the books and casts name.
#[derive(Resource, Clone, Debug, Default)]
pub struct SourceTableRes(pub Arc<SourceTable>);

/// The effect rule set in use (chosen per new game, effects.md "The rules seam").
#[derive(Resource, Clone, Debug, Default)]
pub struct EffectRulesRes(pub Arc<EffectRules>);

/// The host's instance counter (effects.md "Active state per target"): world state, saved.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct EffectIds(pub EffectIdAlloc);

/// The world clock the effect step reads (power recharges), in game minutes. The content keeps
/// it current from its calendar; ROADMAP 1.17's single host clock will feed it instead.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct EffectClock(pub GameMinutes);

/// One character's effects: what is active on it and what it knows.
#[derive(Component, Clone, Debug, PartialEq)]
pub struct Effects {
    pub owner: ActorRef,
    pub active: ActiveSet,
    pub book: SpellBook,
}

impl Effects {
    /// Nothing active, an empty book.
    pub fn new(owner: ActorRef) -> Self {
        Self {
            owner,
            active: ActiveSet::new(),
            book: SpellBook::new(),
        }
    }
}

/// Ops for one character's effects (net rule 2: the host turns validated intents into these).
/// Run on the next fixed tick in write order.
#[derive(Message, Clone, Debug, PartialEq)]
pub struct EffectCommand {
    pub target: ActorRef,
    pub ops: Vec<EffectOp>,
}

impl EffectCommand {
    pub fn new(target: ActorRef, ops: Vec<EffectOp>) -> Self {
        Self { target, ops }
    }
}

/// An effect event, published in [`EffectSystems::Out`].
#[derive(Message, Clone, Debug, PartialEq)]
pub struct EffectMessage(pub EffectEvent);

/// Work for the game's handlers (`Request` effects, touch and target deliveries), published at
/// the end of [`EffectSystems::Tick`] for [`EffectSystems::Requests`].
#[derive(Message, Clone, Debug, PartialEq)]
pub struct EffectRequestMessage(pub EffectRequest);

/// The step's output waiting for [`EffectSystems::Out`] (stat ops, events) and the end of
/// [`EffectSystems::Tick`] (requests), in production order.
#[derive(Resource, Clone, Debug, Default)]
pub struct PendingEffectOut(pub EffectOut);

/// Owner → entity of every [`Effects`] (kept by observers). Two entities with one owner: the
/// later insert wins, with a warning.
#[derive(Resource, Clone, Debug, Default)]
pub struct EffectIndex(HashMap<ActorRef, Entity>);

impl EffectIndex {
    pub fn get(&self, owner: ActorRef) -> Option<Entity> {
        self.0.get(&owner).copied()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// The effect sets in `FixedUpdate` (see the crate docs).
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EffectSystems {
    /// Run the [`EffectCommand`]s.
    Commands,
    /// Rates and expiries; publish requests.
    Tick,
    /// The game's request handlers.
    Requests,
    /// Stat ops out, [`EffectMessage`]s out.
    Out,
}

/// The effects layer, on top of `SfStatsPlugin` (added if missing). The content inserts
/// [`EffectTableRes`] and orders [`EffectSystems::Commands`] after its cast release and strikes.
pub struct SfEffectsPlugin;

impl Plugin for SfEffectsPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<sf_stats_bevy::SfStatsPlugin>() {
            app.add_plugins(sf_stats_bevy::SfStatsPlugin);
        }
        app.init_resource::<SourceTableRes>()
            .init_resource::<EffectRulesRes>()
            .init_resource::<EffectIds>()
            .init_resource::<EffectClock>()
            .init_resource::<EffectIndex>()
            .init_resource::<PendingEffectOut>()
            .add_message::<EffectCommand>()
            .add_message::<EffectMessage>()
            .add_message::<EffectRequestMessage>()
            .add_observer(index_insert)
            .add_observer(index_discard)
            .configure_sets(
                FixedUpdate,
                (
                    EffectSystems::Commands,
                    EffectSystems::Tick,
                    EffectSystems::Requests,
                    EffectSystems::Out,
                )
                    .chain()
                    .before(StatSystems::Apply),
            )
            .add_systems(FixedUpdate, run_commands.in_set(EffectSystems::Commands))
            .add_systems(
                FixedUpdate,
                (tick_sets, publish_requests)
                    .chain()
                    .in_set(EffectSystems::Tick),
            )
            .add_systems(FixedUpdate, publish.in_set(EffectSystems::Out));
    }
}

fn index_insert(
    insert: On<Insert<Effects>>,
    effects: Query<&Effects>,
    mut index: ResMut<EffectIndex>,
) {
    let Ok(fx) = effects.get(insert.entity) else {
        return;
    };
    if let Some(old) = index.0.insert(fx.owner, insert.entity)
        && old != insert.entity
    {
        warn!(
            "effects: {:?} is on {old} and {}; the later wins",
            fx.owner, insert.entity
        );
    }
}

fn index_discard(
    replace: On<Discard<Effects>>,
    effects: Query<&Effects>,
    mut index: ResMut<EffectIndex>,
) {
    let Ok(fx) = effects.get(replace.entity) else {
        return;
    };
    if index.0.get(&fx.owner) == Some(&replace.entity) {
        index.0.remove(&fx.owner);
    }
}

/// What the systems read: the tables and rules (if the content set them up), the tick, its
/// length and the world clock.
#[derive(bevy_ecs::system::SystemParam)]
struct Tables<'w> {
    defs: Option<Res<'w, EffectTableRes>>,
    stats: Option<Res<'w, StatTableRes>>,
    sources: Res<'w, SourceTableRes>,
    rules: Res<'w, EffectRulesRes>,
    tick: Res<'w, StatTick>,
    time: Res<'w, Time>,
    clock: Res<'w, EffectClock>,
}

#[allow(clippy::needless_pass_by_value)] // system parameters
fn run_commands(
    tables: Tables,
    index: Res<EffectIndex>,
    mut ids: ResMut<EffectIds>,
    mut commands: MessageReader<EffectCommand>,
    mut effects: Query<(&mut Effects, Option<&Stats>)>,
    mut pending: ResMut<PendingEffectOut>,
) {
    let (Some(defs), Some(stats)) = (&tables.defs, &tables.stats) else {
        commands.clear();
        return;
    };
    let (tick, dt) = (tables.tick.0, tables.time.delta_secs());
    for c in commands.read() {
        let Some((mut fx, sheet)) = index.get(c.target).and_then(|e| effects.get_mut(e).ok())
        else {
            warn!(
                "effects: command for {:?}, which has no effects; dropped",
                c.target
            );
            continue;
        };
        let sheet = sheet.map(|s| s.view(&stats.0));
        let env = EffectEnv::new(
            &defs.0,
            &stats.0,
            &tables.sources.0,
            &tables.rules.0,
            sheet,
            tables.clock.0,
        );
        let mut ctx = EffectCtx {
            target: c.target,
            tick,
            dt,
            ids: &mut ids.0,
        };
        let fx = &mut *fx;
        sf_effects::apply_book(
            &env,
            &mut fx.active,
            &mut fx.book,
            &c.ops,
            &mut ctx,
            &mut pending.0,
        );
    }
}

/// One fixed tick on every set with timed effects, in owner order. `pub(crate)` for the
/// measurement test.
#[allow(clippy::needless_pass_by_value)] // system parameters
pub(crate) fn tick_sets(
    tables: Tables,
    mut ids: ResMut<EffectIds>,
    mut effects: Query<(Entity, &mut Effects)>,
    mut pending: ResMut<PendingEffectOut>,
    mut timed: Local<Vec<(ActorRef, Entity)>>,
) {
    let (Some(defs), Some(stats)) = (&tables.defs, &tables.stats) else {
        return;
    };
    timed.clear();
    timed.extend(
        effects
            .iter()
            .filter(|(_, fx)| fx.active.is_timed())
            .map(|(e, fx)| (fx.owner, e)),
    );
    // Query order is not an owner order.
    timed.sort_unstable();
    let (tick, dt) = (tables.tick.0, tables.time.delta_secs());
    for &(owner, e) in &*timed {
        let Ok((_, mut fx)) = effects.get_mut(e) else {
            continue;
        };
        let ctx = EffectCtx {
            target: owner,
            tick,
            dt,
            ids: &mut ids.0,
        };
        sf_effects::tick(&defs.0, &stats.0, &mut fx.active, &ctx, &mut pending.0);
    }
}

fn publish_requests(
    mut pending: ResMut<PendingEffectOut>,
    mut out: MessageWriter<EffectRequestMessage>,
) {
    out.write_batch(pending.0.requests.drain(..).map(EffectRequestMessage));
}

fn publish(
    mut pending: ResMut<PendingEffectOut>,
    mut stat_out: MessageWriter<StatCommand>,
    mut out: MessageWriter<EffectMessage>,
) {
    stat_out.write_batch(stat_commands(pending.0.stat_ops.drain(..)));
    out.write_batch(pending.0.events.drain(..).map(EffectMessage));
}

/// The step's stat ops as `StatCommand`s, in order: one command per run of ops with the same
/// owner and cause (one effect's start, tick or end), so each lands atomically (stats.md
/// "Operations") and carries its effect's `SourceId`.
pub fn stat_commands(
    ops: impl IntoIterator<Item = (StatOwner, StatOp, Cause)>,
) -> Vec<StatCommand> {
    let mut out: Vec<StatCommand> = Vec::new();
    for (owner, op, cause) in ops {
        match out.last_mut() {
            Some(last)
                if last.owner == owner
                    && last.cause == cause.kind
                    && last.source == cause.source =>
            {
                last.ops.push(op)
            }
            _ => out.push(StatCommand {
                owner,
                cause: cause.kind,
                source: cause.source,
                ops: vec![op],
            }),
        }
    }
    out
}
