//! Parked worldspaces (CW-9, ROADMAP 1.25; frame-waste audit 2026-11-16 §2 item 10; streaming.md
//! "Worldspaces at runtime"). A worldspace the player left (the exterior while indoors) stays resident,
//! so the door back out stays fast (the residency cache), but every entity of it leaves every query and
//! render: Bevy 0.20's [`Disabled`] with its default query filter, on the worldspace root's whole
//! subtree. The industry form: Unreal's streaming levels loaded but not visible stop ticking; `OpenMW`
//! and Bethesda keep preloaded cells outside the scene graph. Opt-in: `stream.interior.disable_exterior`
//! ([`sf_world::park`]), off by default (owner rule: a change stays opt-in until judged).
//!
//! **What parks.** A loaded worldspace that the player was in and left ([`ActiveWorldspace`] moved
//! away from it), that streaming leaves alone (no request for it) and that stayed so for
//! `stream.interior.park_settle_s`. A worldspace never entered (a door prefetch target) is not parked:
//! its resident meshes feed the pipeline warm-up before the first visit.
//!
//! **How.** Frame 1, the root's `Visibility` goes `Hidden`: Bevy's visibility marks the subtree
//! invisible and extraction drops its mesh instances and lights from the render world. Bevy 0.20's
//! render-world sync has no `Disabled` handling (`bevy_extract` `sync_world.rs`): an entity disabled
//! while extracted would keep its render-world data. Frame 2, [`Disabled`] goes on the root and every
//! descendant (`Children`, direct world access, one batch insert). Relationships stay intact, direct
//! `World` access still works (streaming's stages, spawner hooks), queries skip them.
//!
//! **Unpark** (remove `Disabled`, restore the root's visibility, same command flush), when any of:
//! the worldspace becomes active (a door cut: before the fade's hold reads it), streaming has a
//! request for it (a door prefetch reload, the residency cache expiring and unloading cells), the
//! switch goes off, or an entity is parented under the parked subtree (`Insert<ChildOf>` observer: a
//! late model or a reparented traveler; the new entity must never run under a disabled parent). The
//! settle timer restarts each time, so a worldspace that keeps receiving spawns stays live.
//!
//! Queries that must see parked entities name them (`Allow<Disabled>`): the subtree layer/range
//! hooks, live LOD re-ranging, a game's save capture and teardown.

use std::collections::{BTreeMap, BTreeSet};

use bevy_camera::visibility::Visibility;
use bevy_ecs::entity_disabling::Disabled;
use bevy_ecs::prelude::*;
use bevy_platform::time::Instant;
use sf_diag_bevy::DiagCounters;
use sf_diag_bevy::sf_diag::unit;
use sf_world::WorldspaceId;
use sf_world::park::{PARK_LEFT, PARK_SETTLE_S};

use crate::stream::{ActiveWorldspace, Streaming};
use crate::worldspace::{WorldspaceRegistry, WorldspaceRoot};

/// Where a parked worldspace is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Stage {
    /// Root hidden; the subtree is disabled the next frame (until that command applies, a spawn
    /// under the root still wakes it, and the command checks it is still parked).
    Hiding,
    /// Root and subtree disabled.
    Disabled,
}

#[derive(Clone, Copy, Debug)]
struct Parked {
    root: Entity,
    /// The root's own visibility before parking, restored on unpark.
    visibility: Visibility,
    stage: Stage,
    /// The disable command is queued (stage still `Hiding` until it applies).
    queued: bool,
    /// Entities disabled (root included), for counters and tests.
    entities: usize,
}

/// Parking: the switch (`stream.interior.disable_exterior`), the settle time
/// (`stream.interior.park_settle_s`) and the state. Hosts bind the two fields live.
#[derive(Resource, Debug)]
pub struct WorldspaceParking {
    pub enabled: bool,
    /// Seconds a left worldspace stays quiet before it parks.
    pub settle_s: f32,
    /// Worldspaces the player was in and left (cleared when they leave the registry).
    left: BTreeSet<WorldspaceId>,
    last_active: Option<WorldspaceId>,
    /// When each worldspace last stood in the way of parking (active, a request, a spawn under it).
    busy_at: BTreeMap<WorldspaceId, f64>,
    parked: BTreeMap<WorldspaceId, Parked>,
    /// The streaming clock this frame, for the observer.
    now: f64,
}

impl Default for WorldspaceParking {
    fn default() -> Self {
        Self {
            enabled: PARK_LEFT.default_value(),
            settle_s: PARK_SETTLE_S.default_value(),
            left: BTreeSet::new(),
            last_active: None,
            busy_at: BTreeMap::new(),
            parked: BTreeMap::new(),
            // tunables: const (state: the streaming clock, set every frame)
            now: 0.0,
        }
    }
}

impl WorldspaceParking {
    /// Whether `ws` is parked (hidden or disabled).
    pub fn is_parked(&self, ws: &WorldspaceId) -> bool {
        self.parked.contains_key(ws)
    }

    /// Whether `ws` is fully parked (its subtree disabled).
    pub fn is_disabled(&self, ws: &WorldspaceId) -> bool {
        self.parked.get(ws).is_some_and(|p| p.stage == Stage::Disabled)
    }

    /// Entities disabled in every parked worldspace.
    pub fn parked_entities(&self) -> usize {
        self.parked.values().map(|p| p.entities).sum()
    }

    /// Parked worldspaces.
    pub fn parked_count(&self) -> usize {
        self.parked.len()
    }

    /// Takes `ws` out of parking (no-op when it isn't parked): restores the root's visibility now,
    /// removes `Disabled` from its subtree at the next command flush.
    fn unpark(&mut self, ws: &WorldspaceId, commands: &mut Commands, roots: &mut Query<&mut Visibility, (With<WorldspaceRoot>, Allow<Disabled>)>) {
        self.busy_at.insert(ws.clone(), self.now);
        let Some(p) = self.parked.remove(ws) else { return };
        if let Ok(mut v) = roots.get_mut(p.root) {
            *v = p.visibility;
        }
        if p.stage == Stage::Disabled {
            let name = ws.as_str().to_owned();
            commands.queue(move |world: &mut World| enable_subtree(world, p.root, &name));
        }
    }
}

/// Every entity of `root`'s subtree, root first (direct access: sees disabled entities).
fn subtree(world: &World, root: Entity) -> Vec<Entity> {
    let mut out = Vec::new();
    let mut stack = vec![root];
    while let Some(e) = stack.pop() {
        if world.get_entity(e).is_err() {
            continue;
        }
        out.push(e);
        if let Some(children) = world.get::<Children>(e) {
            stack.extend(children.iter());
        }
    }
    out
}

fn disable_subtree(world: &mut World, ws: &WorldspaceId, root: Entity) {
    // Woken between the queue and now (a spawn applied earlier in this flush): stay live.
    let still = world
        .get_resource::<WorldspaceParking>()
        .and_then(|p| p.parked.get(ws))
        .is_some_and(|p| p.root == root && p.stage == Stage::Hiding);
    if !still {
        return;
    }
    let started = Instant::now();
    let all = subtree(world, root);
    let n = all.len();
    if let Err(e) = world.try_insert_batch(all.into_iter().map(|e| (e, Disabled))) {
        log::warn!("sf_world_bevy: park {}: {e}", ws.as_str());
    }
    if let Some(mut parking) = world.get_resource_mut::<WorldspaceParking>()
        && let Some(p) = parking.parked.get_mut(ws)
    {
        p.stage = Stage::Disabled;
        p.entities = n;
    }
    report_ms(world, started);
    log::info!("sf_world_bevy: parked {}: {n} entities disabled in {:.2} ms", ws.as_str(), started.elapsed().as_secs_f64() * 1000.0);
}

fn enable_subtree(world: &mut World, root: Entity, ws: &str) {
    let started = Instant::now();
    let all = subtree(world, root);
    for &e in &all {
        if let Ok(mut entity) = world.get_entity_mut(e) {
            entity.remove::<Disabled>();
        }
    }
    report_ms(world, started);
    log::info!("sf_world_bevy: unparked {ws}: {} entities enabled in {:.2} ms", all.len(), started.elapsed().as_secs_f64() * 1000.0);
}

fn report_ms(world: &World, started: Instant) {
    if let Some(c) = world.get_resource::<DiagCounters>() {
        c.add_duration_named("stream/park/ms", started.elapsed());
    }
}

/// `Update`, after the fade: parks and unparks (module docs).
#[allow(clippy::needless_pass_by_value)]
pub(crate) fn park_worldspaces(
    mut commands: Commands,
    mut parking: ResMut<WorldspaceParking>,
    active: Res<ActiveWorldspace>,
    registry: Res<WorldspaceRegistry>,
    streaming: Option<Res<Streaming>>,
    mut roots: Query<&mut Visibility, (With<WorldspaceRoot>, Allow<Disabled>)>,
    counters: Option<Res<DiagCounters>>,
) {
    let Some(streaming) = streaming else { return };
    let now = streaming.now();
    let parking = &mut *parking;
    parking.now = now;
    if parking.last_active != active.0 {
        if let Some(prev) = parking.last_active.take() {
            parking.left.insert(prev);
        }
        parking.last_active.clone_from(&active.0);
    }
    // A released worldspace (root despawned) is forgotten; a re-prefetch starts fresh.
    parking.parked.retain(|ws, p| registry.get(ws).is_some_and(|e| e.root == p.root));
    parking.left.retain(|ws| registry.get(ws).is_some());
    parking.busy_at.retain(|ws, _| registry.get(ws).is_some());

    let busy: BTreeSet<&WorldspaceId> = streaming.scheduler().requests().map(|r| &r.worldspace).collect();
    for (ws, entry) in registry.iter() {
        let wanted = parking.enabled && parking.left.contains(ws) && active.0.as_ref() != Some(ws) && !busy.contains(ws);
        if !wanted {
            parking.unpark(ws, &mut commands, &mut roots);
            continue;
        }
        match parking.parked.get_mut(ws) {
            None => {
                let since = *parking.busy_at.entry(ws.clone()).or_insert(now);
                if now - since < f64::from(parking.settle_s) {
                    continue;
                }
                let Ok(mut v) = roots.get_mut(entry.root) else { continue };
                let visibility = *v;
                *v = Visibility::Hidden;
                let parked = Parked { root: entry.root, visibility, stage: Stage::Hiding, queued: false, entities: 0 };
                parking.parked.insert(ws.clone(), parked);
            }
            Some(p) if p.stage == Stage::Hiding && !p.queued => {
                p.queued = true;
                let (ws, root) = (ws.clone(), p.root);
                commands.queue(move |world: &mut World| disable_subtree(world, &ws, root));
            }
            Some(_) => {}
        }
    }
    if let Some(c) = counters {
        #[allow(clippy::cast_precision_loss)]
        {
            c.set_named("stream/parked/worldspaces", unit::COUNT, parking.parked_count() as f64);
            c.set_named("stream/parked/entities", unit::COUNT, parking.parked_entities() as f64);
        }
    }
}

/// An entity parented under a parked worldspace (a late spawn, a traveler arriving) unparks it at
/// once, so nothing ever runs under a disabled parent (module docs).
#[allow(clippy::needless_pass_by_value)]
pub(crate) fn wake_on_parent(
    insert: On<Insert<ChildOf>>,
    mut parking: ResMut<WorldspaceParking>,
    parents: Query<&ChildOf, Allow<Disabled>>,
    disabled: Query<(), With<Disabled>>,
    names: Query<&WorldspaceRoot, Allow<Disabled>>,
    mut roots: Query<&mut Visibility, (With<WorldspaceRoot>, Allow<Disabled>)>,
    mut commands: Commands,
) {
    if parking.parked.is_empty() {
        return;
    }
    let Ok(parent) = parents.get(insert.entity).map(ChildOf::parent) else { return };
    // Only a disabled parent or a worldspace still hiding (one frame) can be parked.
    let hiding = parking.parked.values().any(|p| p.stage == Stage::Hiding);
    if !hiding && !disabled.contains(parent) {
        return;
    }
    let Some(ws) = std::iter::once(parent).chain(parents.iter_ancestors(parent)).find_map(|a| names.get(a).ok().map(|r| r.id.clone())) else {
        return;
    };
    if parking.is_parked(&ws) {
        log::debug!("sf_world_bevy: {} spawned under parked {}: unparked", insert.entity, ws.as_str());
        parking.unpark(&ws, &mut commands, &mut roots);
    }
}
