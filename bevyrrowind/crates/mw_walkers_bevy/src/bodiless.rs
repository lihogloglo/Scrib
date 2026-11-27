//! A tier-0 actor without a body keeps its overlay entry in the cell its logical position stands
//! in (`AI10c3c2a`; `ai.md` "Tiers and budget", `docs/roadmaps/ai.md`), so the next fetch of that
//! cell spawns its body there and its old cell's fetch does not.
//!
//! **What.** When an actor's cell leaves Active, `crate::poses::ActorPoseHook::deactivate` keeps its
//! placement as the overlay now holds it ([`ResidentActors`]' away map). Each fixed tick, after the
//! tier-0 step (`crate::logical::add`, last of the `AiLogicalSystems::Step` chain),
//! [`follow_bodiless`] runs [`follow`] over those kept placements:
//!
//! - a key with a body (an `AiActor` entity) is skipped: `AI10c3a` and `AI10c3b2` own bodies;
//! - a key with neither a body nor a stored state (`sf_ai_bevy::AiLogical`) is pruned, so the map
//!   does not grow with every NPC the player walked past;
//! - a stored state on a route (not following) gives its position now
//!   (`AiLogical::position_at`, analytic); when its `(worldspace, partition cell)` differs from
//!   the kept placement's, the overlay gets `PlacementOverlay::move_to(kept, position, kept
//!   rotation)` and the kept copy moves with it: one write per crossing, none while it stays in a
//!   cell. A state without a route does not move and is skipped.
//!
//! Same result as `OpenMW` moving an actor between inactive `CellStore`s at once
//! (`apps/openmw/mwworld/worldimp.cpp` `moveObject`, commit b8285cc9), here through the overlay
//! bucket. No tunable: no distance, budget or cadence (one analytic position per routed kept
//! actor and tick). Without `Streaming` or `WorldOverlay` nothing runs.
//!
//! **Gaps (named):** a tier-0 actor that never had a loaded body since the start has no kept
//! placement and is skipped. A follower (`LogicalState::follow`) is skipped: its leader's position
//! is not read here. A loaded cell is not re-read, so a body appears at the next fetch of the cell
//! it walked into, not at once, unless that cell is Active (below). The kept rotation is
//! written unchanged (the facing it walked with is not known at tier 0). No proof line yet (it
//! comes with `AI10c3c2b2`).
//!
//! **Into an Active cell (`AI10c3c2b1`).** Right after the follow, [`pick`] takes each kept
//! placement of a bodiless stored actor (not a follower) whose `(worldspace, partition cell)` is
//! Active, in placement-id order, at most `ai.tier.promote_per_tick` tries a tick, and resolves
//! where its body appears with the promotion rules (`sf_ai::resolve_promotion` against the same
//! `ViewsEnv` the promotion uses: its spot at this tick, else back along its route, snapped to
//! the navmesh, unseen by every player). A spot in an Active cell becomes a [`Pending`] spawn; a
//! wait (every candidate seen or off the mesh, or the picked spot's cell not Active) is not tried
//! again for `ai.tier.promote_retry_s` simulated seconds (game ms over the clock's scale, so a
//! trace reproduces). Then [`spawn_bodiless`] (exclusive, last of the chain) hands each to
//! `Streaming::spawn_into` (`AI10c3c1`) at the picked spot, kept rotation; on success the
//! overlay gets the placement there (`PlacementOverlay::move_to`), the cell's Active list and
//! `ActorPoseHook::activate` take it (which drops the kept entry: the cell's later fetch spawns
//! no twin), and `enter_tier0` re-marks the new body (`AiLogicalBody`, state untouched), so it is
//! queued and promoted like any stored body: the store keeps the state until it is placed. A
//! refusal (a stage in flight) is tried again next tick. The canonical technique is the
//! promotion's (UE5 `MassRepresentation` spawn budget, GTA/RDR on-path ambient spawns out of
//! view); `OpenMW` instead inserts an actor moved into an active cell at once, wherever it stands
//! (`apps/openmw/mwworld/worldimp.cpp` `moveObject`, commit b8285cc9). No new tunable.

use std::collections::HashMap;
use std::sync::PoisonError;

use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use bevy_math::{IVec3, Quat};
use sf_ai::{LogicalState, PromoteConfig, Resolve, resolve_promotion};
use sf_ai_bevy::{AiActor, AiLogical, AiPromoteSettings};
use sf_coords::WorldPos;
use sf_ids::{ActorKey, GameInstant};
use sf_nav::QueryFilter;
use sf_nav_bevy::NavMeshes;
use sf_world::{IndexedPlacement, Level, Location, Partition, Placement, PlacementId, WorldspaceId};
use sf_world_bevy::{ActiveWorldspace, PlacementSpawners, Streaming, WorldOverlay};

use crate::in_view::{AiViewer, InViewSettings, ViewRules};
use crate::logical::{Tier0World, ViewsEnv, WorldGraphEnv, default_radius_m, view_snapshot};
use crate::poses::ResidentActors;

/// Follows and prunes the kept placements of `away` (module docs): `has_body` and `position` by
/// key (the position at this tick), `partition_of` each worldspace's streaming partition, `write`
/// one overlay move `(kept placement, to, rotation)`. Returns the number of writes.
pub(crate) fn follow(
    away: &mut HashMap<PlacementId, (WorldspaceId, Placement)>,
    store: &AiLogical,
    has_body: impl Fn(ActorKey) -> bool,
    position: impl Fn(ActorKey) -> Option<Location>,
    partition_of: impl Fn(&WorldspaceId) -> Option<Partition>,
    mut write: impl FnMut(&Placement, &Location, Quat),
) -> usize {
    let mut writes = 0;
    away.retain(|id, (ws, kept)| {
        let key = ActorKey(id.0);
        if has_body(key) {
            return true;
        }
        let Some(state) = store.get(key) else { return false };
        if state.route.is_none() || state.follow.is_some() {
            return true;
        }
        let Some(to) = position(key) else { return true };
        if crossed(ws, kept, &to, &partition_of) {
            write(kept, &to, kept.transform.rotation);
            kept.transform.pos = to.pos;
            *ws = to.worldspace;
            writes += 1;
        }
        true
    });
    writes
}

/// Whether `to` stands in another worldspace or another partition cell than `kept` (of `ws`). A
/// worldspace without a known partition never crosses inside itself.
fn crossed(ws: &WorldspaceId, kept: &Placement, to: &Location, partition_of: &impl Fn(&WorldspaceId) -> Option<Partition>) -> bool {
    to.worldspace != *ws || partition_of(ws).is_some_and(|p| p.cell_of(kept.transform.pos) != p.cell_of(to.pos))
}

/// One body [`pick`] wants spawned into a loaded cell (`AI10c3c2b1`).
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Pending {
    /// The kept placement (as the overlay holds it).
    pub kept: Placement,
    /// The picked spot (snapped, unseen).
    pub at: Location,
    /// The Active cell of `at`.
    pub cell: IVec3,
}

impl Pending {
    /// The placement spawned and written: the kept one at the picked spot, kept rotation.
    fn placement(&self) -> Placement {
        let mut p = self.kept.clone();
        p.transform.pos = self.at.pos;
        p
    }
}

/// The spawns of bodiless actors into loaded cells (`AI10c3c2b1`, module docs): the waits' retry
/// instants, this tick's picks, the count since start.
#[derive(Resource, Debug, Default)]
pub struct BodilessSpawns {
    /// Game instant before which a waiting kept placement is not tried again.
    retry_at: HashMap<PlacementId, GameInstant>,
    pending: Vec<Pending>,
    /// Bodies spawned into loaded cells since start.
    pub spawned: u32,
}

/// The game ms of `config.retry_s` simulated seconds at `time_scale` (game s per simulated s).
#[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss, reason = "clamped non-negative, game ms")]
pub(crate) fn retry_ms(config: &PromoteConfig, time_scale: f64) -> u64 {
    let ms = f64::from(config.retry_s) * time_scale * 1000.0;
    if ms.is_finite() { ms.max(0.0).round() as u64 } else { 0 }
}

/// Picks the bodies to spawn into Active cells (module docs, "Into an Active cell"): `active`
/// gives the Active cell a point stands in (`None`: not Active), `resolve` one promotion try.
/// Pushes into `spawns`' pending list; returns the number of tries.
#[allow(clippy::too_many_arguments)]
pub(crate) fn pick(
    away: &HashMap<PlacementId, (WorldspaceId, Placement)>,
    store: &AiLogical,
    has_body: impl Fn(ActorKey) -> bool,
    active: impl Fn(&WorldspaceId, WorldPos) -> Option<IVec3>,
    mut resolve: impl FnMut(ActorKey, &LogicalState) -> Resolve,
    now: GameInstant,
    retry_ms: u64,
    per_tick: u32,
    spawns: &mut BodilessSpawns,
) -> u32 {
    spawns.retry_at.retain(|id, at| away.contains_key(id) && *at > now);
    let mut ids: Vec<PlacementId> = away.keys().copied().collect();
    ids.sort_unstable();
    let mut tries = 0;
    for id in ids {
        if tries >= per_tick {
            break;
        }
        let key = ActorKey(id.0);
        let (ws, kept) = &away[&id];
        if has_body(key) || spawns.retry_at.contains_key(&id) || active(ws, kept.transform.pos).is_none() {
            continue;
        }
        let Some(state) = store.get(key).filter(|s| s.follow.is_none()) else { continue };
        tries += 1;
        let picked = match resolve(key, state) {
            Resolve::Place(p) => active(&p.at.worldspace, p.at.pos).map(|cell| (p.at, cell)),
            Resolve::Wait => None,
        };
        match picked {
            Some((at, cell)) => spawns.pending.push(Pending { kept: kept.clone(), at, cell }),
            None => {
                spawns.retry_at.insert(id, GameInstant(now.0.saturating_add(retry_ms)));
            }
        }
    }
    tries
}

/// What [`follow_bodiless`] reads to pick spawns (the views and navmesh promotion places against)
/// and the picks it writes.
#[derive(SystemParam)]
pub(crate) struct SpawnViews<'w, 's> {
    spawns: ResMut<'w, BodilessSpawns>,
    viewers: Query<'w, 's, &'static AiViewer>,
    settings: Option<Res<'w, InViewSettings>>,
    active: Option<Res<'w, ActiveWorldspace>>,
    meshes: Option<Res<'w, NavMeshes>>,
    promote: Option<Res<'w, AiPromoteSettings>>,
    filter: Local<'s, Option<QueryFilter>>,
}

/// The fixed-tick system around [`follow`] and [`pick`] (module docs): the tier-0 env as
/// `step_tier0` builds it (no walk speed, placement or body lookups: a routed state reads only
/// its route), the streaming directory's partitions, one overlay lock per write; then the picks
/// against a `ViewsEnv` (every viewer, the navmesh, the default body radius: none has a body).
#[allow(clippy::needless_pass_by_value)]
pub(crate) fn follow_bodiless(
    world: Tier0World,
    store: Res<AiLogical>,
    resident: Option<ResMut<ResidentActors>>,
    overlay: Option<Res<WorldOverlay>>,
    streaming: Option<Res<Streaming>>,
    bodies: Query<&AiActor>,
    mut views: SpawnViews,
) {
    let (Some(mut resident), Some(overlay), Some(streaming)) = (resident, overlay, streaming) else { return };
    if !resident.has_away() {
        return;
    }
    let mut keys: Vec<ActorKey> = bodies.iter().map(|a| a.0).collect();
    keys.sort_unstable();
    let has_body = |k: ActorKey| keys.binary_search(&k).is_ok();
    let door = |id| world.door(id);
    let no_speed = |_: ActorKey| None;
    let nowhere = |_: PlacementId| None;
    let no_body = |_: ActorKey| None;
    let env = WorldGraphEnv::new(world.graph(), &door, world.traverse(), &no_speed, &nowhere, &no_body);
    let (now, scale) = (world.now(), world.time_scale());
    let scheduler = streaming.scheduler();
    let partition_of = |ws: &WorldspaceId| scheduler.directory(ws).map(|d| d.partition);
    follow(
        resident.away_mut(),
        &store,
        has_body,
        |k| store.position_at(k, now, &env, scale),
        partition_of,
        |p, to, rotation| overlay.0.write().unwrap_or_else(PoisonError::into_inner).move_to(p, to, rotation),
    );

    let config = views.promote.as_ref().map_or_else(PromoteConfig::default, |s| s.config);
    let filter = &*views.filter.get_or_insert_with(|| QueryFilter::for_class(&mw_bake::nav::areas(), mw_bake::nav::HUMANOID));
    let snapshot = view_snapshot(views.viewers.iter(), views.active.as_ref().and_then(|a| a.0.as_ref()));
    let rules = views.settings.as_ref().map_or_else(ViewRules::default, |s| s.rules);
    let nav = views.meshes.as_deref().map(|m| &*m.0);
    let radius = |_: ActorKey| default_radius_m();
    let promote = ViewsEnv { views: &snapshot, rules, nav, filter, body_radius: &radius };
    let active = |ws: &WorldspaceId, pos: WorldPos| {
        let cell = partition_of(ws)?.cell_of(pos);
        (scheduler.level(ws, cell) == Level::Active).then_some(cell)
    };
    pick(
        resident.away_mut(),
        &store,
        has_body,
        active,
        |k, state| resolve_promotion(k, state, now, scale, &env, &promote, &config),
        now,
        retry_ms(&config, scale),
        config.per_tick,
        &mut views.spawns,
    );
}

/// Spawns this tick's picks (module docs): `Streaming::spawn_into` at the picked spot, then the
/// overlay write; a refusal is picked again next tick. A placement no Active list took leaves the
/// kept map by hand, as an activation would have dropped it.
pub(crate) fn spawn_bodiless(world: &mut World) {
    let pending = match world.get_resource_mut::<BodilessSpawns>() {
        Some(mut s) if !s.pending.is_empty() => std::mem::take(&mut s.pending),
        _ => return,
    };
    let Some(spawners) = world.get_resource::<PlacementSpawners>().cloned() else { return };
    if !world.contains_resource::<Streaming>() {
        return;
    }
    let mut spawned = 0;
    world.resource_scope(|world, mut streaming: Mut<Streaming>| {
        for p in pending {
            let placement = p.placement();
            let id = placement.id;
            let indexed = IndexedPlacement { placement, radius: 0.0 };
            let Some(done) = streaming.spawn_into(world, &spawners, &p.at.worldspace, p.cell, indexed) else { continue };
            if let Some(overlay) = world.get_resource::<WorldOverlay>() {
                overlay.0.write().unwrap_or_else(PoisonError::into_inner).move_to(&p.kept, &p.at, p.kept.transform.rotation);
            }
            if !done.listed
                && let Some(mut resident) = world.get_resource_mut::<ResidentActors>()
            {
                resident.away_mut().remove(&id);
            }
            spawned += 1;
        }
    });
    if let Some(mut s) = world.get_resource_mut::<BodilessSpawns>() {
        s.spawned += spawned;
    }
}

#[cfg(test)]
#[path = "bodiless_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "bodiless_spawn_tests.rs"]
mod spawn_tests;
