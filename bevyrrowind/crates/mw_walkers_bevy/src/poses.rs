//! Actors keep their pose across their cell's unload and reload (`AI10c3a`; `ai.md` "Tiers and
//! budget", `docs/roadmaps/ai.md`).
//!
//! [`ActorPoseHook`] wraps the actor class spawner (installed with `PlacedObjectsHook` by
//! `crate::directives::hook_placed_objects`). When an actor placement leaves R0 (its cell drops out
//! of Active; the body and its `Mover` are still alive, and from then on `Dormant`, so the pose
//! written is the pose it has when its Near group despawns), a body that moved from its
//! placement is written to the placement overlay: `PlacementOverlay::move_to` its worldspace
//! (its `ActorSpace`), feet (`mover.loco.position`) and facing (the mover's yaw, as the placement
//! rotation `CharacterSpawner` turns back to it, [`placement_rotation`]). The overlay then loads
//! the placement with the cell it now stands in, at its next fetch, and `CharacterSpawner` +
//! `attach` respawn the body there, facing that way. Not on `ActivateCx::discard` (a reload after
//! a save load: the loaded overlay wins), as `sf_physics_bevy`'s `persist_item`.
//!
//! - **Moved** means more than 1e-3 m on the ground plane or 1e-3 rad of yaw (maths tolerance, as
//!   `sf_physics_bevy`'s `moved_item`), or another worldspace than the placement's. A body that
//!   only settled onto the ground at spawn is not moved: the overlay is saved and must not grow
//!   with every NPC the player walked past. A dead or knocked-down body is written too.
//! - **Tier 0** (`AI10c2`): a stored actor's body is written like any other; its logical state
//!   stays in `sf_ai_bevy::AiLogical`, and the respawned body is re-marked by `enter_tier0`
//!   (`crate::logical`).
//! - **Saves** (`MA22`, not wired yet): [`persist_actor_poses`] writes every resident moved actor
//!   into a copy of the overlay as its deactivation would, without removing anything.
//! - **Proof line**: `actors: N poses kept at unload`, at most once a second.
//! - **Kept placements** (`AI10c3c2a`): deactivation (not a discard) also keeps the placement as
//!   it now stands in the overlay (the pose just written, an earlier entry, else the listed one)
//!   in [`ResidentActors`]' away map; activation drops it. [`crate::bodiless`] moves a bodiless
//!   tier-0 actor's kept placement along its logical position.
//!
//! **Crossings (`AI10c3b2`):** a body that walks into a neighbouring loaded cell becomes that
//! cell's ([`crate::crossing`]), with a pose write here ([`write_crossing`]), so its home cell's
//! unload no longer despawns it and no cell lists it twice.

use std::collections::HashMap;
use std::f32::consts::TAU;
use std::sync::{Arc, PoisonError};

use bevy_asset::{AssetServer, UntypedHandle};
use bevy_ecs::prelude::*;
use bevy_log::prelude::*;
use bevy_math::Quat;
use bevy_time::prelude::*;
use sf_character_bevy::{CharacterPlacement, Characters};
use sf_controller_bevy::Mover;
use sf_world::{IndexedPlacement, Location, Placement, PlacementId, PlacementOverlay, WorldspaceId};
use sf_world_bevy::{ActivateCx, LoadedModel, PlacementSpawner, SpawnCx, WorldOverlay};

use crate::walkers::ActorSpace;

/// Position tolerance on the ground plane, metres (maths tolerance, as `moved_item`).
// tunables: const (comparison epsilon, not a gameplay or quality value)
const MOVED_M: f32 = 1e-3;
/// Yaw tolerance, radians (maths tolerance, as `moved_item`).
// tunables: const (comparison epsilon, not a gameplay or quality value)
const TURNED_RAD: f32 = 1e-3;

/// The actor placements of the Active cells, with their worldspace (inserted at activation,
/// removed at deactivation): what [`persist_actor_poses`] writes from. Also counts the poses
/// kept since the last proof line.
#[derive(Resource, Debug, Default)]
pub struct ResidentActors {
    placements: HashMap<PlacementId, (WorldspaceId, Placement)>,
    /// `AI10c3c2a`: the placements of actors whose cell left Active, as the overlay now holds
    /// them (kept at deactivation, dropped at activation, pruned by [`crate::bodiless`]).
    away: HashMap<PlacementId, (WorldspaceId, Placement)>,
    kept: u32,
}

impl ResidentActors {
    /// Resident actor placements.
    pub fn len(&self) -> usize {
        self.placements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.placements.is_empty()
    }

    /// The indexed placement an Active cell listed for `id` (`AI10c3b2`: what a crossing writes).
    pub(crate) fn placement(&self, id: PlacementId) -> Option<&Placement> {
        self.placements.get(&id).map(|(_, p)| p)
    }

    /// Drops `id` when no Active list holds it any more (`AI10c3b2`: a re-home into a Near-only
    /// cell, whose activation lists it again from the overlay).
    pub(crate) fn forget(&mut self, id: PlacementId) {
        self.placements.remove(&id);
    }

    /// The placement kept for `id` when its cell left Active (`AI10c3c2a`), with its worldspace.
    #[cfg(test)]
    pub(crate) fn kept_placement(&self, id: PlacementId) -> Option<&(WorldspaceId, Placement)> {
        self.away.get(&id)
    }

    /// Whether any placement is kept (`AI10c3c2a`).
    pub(crate) fn has_away(&self) -> bool {
        !self.away.is_empty()
    }

    /// The kept placements, to follow and prune (`AI10c3c2a`, [`crate::bodiless`]).
    pub(crate) fn away_mut(&mut self) -> &mut HashMap<PlacementId, (WorldspaceId, Placement)> {
        &mut self.away
    }
}

/// Wraps the actor class spawner: deactivation (not a discard) writes a body that moved to the
/// placement overlay (module docs).
pub struct ActorPoseHook(pub Arc<dyn PlacementSpawner>);

impl PlacementSpawner for ActorPoseHook {
    fn spawn(&self, world: &mut World, cx: &SpawnCx<'_>, placement: &IndexedPlacement) -> u32 {
        self.0.spawn(world, cx, placement)
    }
    fn needs_model(&self, placement: &IndexedPlacement) -> bool {
        self.0.needs_model(placement)
    }
    fn prepare(&self, assets: &AssetServer, placement: &IndexedPlacement, model: Option<&LoadedModel>) -> Option<UntypedHandle> {
        self.0.prepare(assets, placement, model)
    }
    fn activate(&self, world: &mut World, cx: &ActivateCx<'_>, placement: &IndexedPlacement) {
        self.0.activate(world, cx, placement);
        if let Some(mut resident) = world.get_resource_mut::<ResidentActors>() {
            let p = &placement.placement;
            resident.away.remove(&p.id);
            resident.placements.insert(p.id, (cx.worldspace.clone(), p.clone()));
        }
    }
    fn deactivate(&self, world: &mut World, cx: &ActivateCx<'_>, placement: &IndexedPlacement) {
        self.0.deactivate(world, cx, placement);
        let p = &placement.placement;
        // A reload after a save load discards live poses: the overlay is the loaded one.
        if !cx.discard
            && let Some((to, rotation)) = body_of(world, p.id).and_then(|body| moved(cx.worldspace, p, &body, turn(world)))
            && let Some(overlay) = world.get_resource::<WorldOverlay>()
        {
            overlay.0.write().unwrap_or_else(PoisonError::into_inner).move_to(p, &to, rotation);
            if let Some(mut resident) = world.get_resource_mut::<ResidentActors>() {
                resident.kept += 1;
            }
        }
        let away = (!cx.discard).then(|| standing(world, cx.worldspace, p));
        if let Some(mut resident) = world.get_resource_mut::<ResidentActors>() {
            resident.placements.remove(&p.id);
            match away {
                Some(kept) => resident.away.insert(p.id, kept),
                None => resident.away.remove(&p.id),
            };
        }
    }
}

/// Placement `p` of worldspace `ws` as the overlay now holds it (its moved or spawned entry),
/// else as listed (`AI10c3c2a`).
fn standing(world: &World, ws: &WorldspaceId, p: &Placement) -> (WorldspaceId, Placement) {
    world
        .get_resource::<WorldOverlay>()
        .and_then(|o| o.0.read().unwrap_or_else(PoisonError::into_inner).placed(p.id).map(|(w, p)| (w.clone(), p.clone())))
        .unwrap_or_else(|| (ws.clone(), p.clone()))
}

/// Saving (`MA22`, save.md "One path for unload and save"): writes every resident actor that
/// moved into `overlay` (a copy of the live one, for the snapshot), as its deactivation would,
/// without removing anything.
pub fn persist_actor_poses(world: &World, overlay: &mut PlacementOverlay) {
    let Some(resident) = world.get_resource::<ResidentActors>() else { return };
    let Some(mut bodies) = world.try_query::<(&CharacterPlacement, &Mover, &ActorSpace)>() else { return };
    let bodies: HashMap<PlacementId, (Location, f32)> = bodies.iter(world).map(|(p, m, s)| (p.0, pose(m, s))).collect();
    let turn = turn(world);
    for (id, (ws, p)) in &resident.placements {
        if let Some((to, rotation)) = bodies.get(id).and_then(|body| moved(ws, p, body, turn)) {
            overlay.move_to(p, &to, rotation);
        }
    }
}

/// The placement rotation of a body facing `yaw`: `CharacterSpawner` adds the content's `turn`
/// (`CharacterContent::placement_rotation`) to a placement's rotation, so the stored one has it
/// taken off again.
pub fn placement_rotation(yaw: f32, turn: Quat) -> Quat {
    Quat::from_rotation_y(yaw) * turn.inverse()
}

/// The yaw a body spawned from a placement rotation faces (`attach`: `Mover::yaw_of` of the
/// spawned transform).
pub fn placement_yaw(rotation: Quat, turn: Quat) -> f32 {
    Mover::yaw_of(rotation * turn)
}

/// `proof line`: `actors: N poses kept at unload`, when some were, at most once a second.
#[allow(clippy::needless_pass_by_value)]
pub(crate) fn report_kept(time: Res<Time>, mut resident: ResMut<ResidentActors>, mut last: Local<Option<f64>>) {
    let now_s = time.elapsed_secs_f64();
    if resident.kept == 0 || last.is_some_and(|t| now_s - t < crate::logical::LOG_EVERY_S) {
        return;
    }
    info!("actors: {} poses kept at unload", resident.kept);
    resident.kept = 0;
    *last = Some(now_s);
}

/// The placement a crossing body is written from (`AI10c3b2`): its Active cell's listed one
/// ([`ResidentActors`]), else the overlay's (a body spawned from a moved placement by a cell that
/// was never Active). `None`: nothing holds it, so a crossing must not re-home it (no overlay
/// write, and the home cell's reload would spawn a twin).
pub(crate) fn held_placement(resident: &ResidentActors, overlay: &WorldOverlay, id: PlacementId) -> Option<Placement> {
    resident.placement(id).cloned().or_else(|| {
        let overlay = overlay.0.read().unwrap_or_else(PoisonError::into_inner);
        overlay.placed(id).map(|(_, p)| p.clone())
    })
}

/// A crossing's overlay write (`AI10c3b2`, on a successful re-home only): the body's pose, as
/// [`ActorPoseHook::deactivate`] writes it but unconditionally, so the next fetch of the cell it
/// now stands in lists it there and its home cell's no longer does.
pub(crate) fn write_crossing(overlay: &WorldOverlay, placement: &Placement, mover: &Mover, space: &ActorSpace, turn: Quat) {
    let (to, yaw) = pose(mover, space);
    overlay.0.write().unwrap_or_else(PoisonError::into_inner).move_to(placement, &to, placement_rotation(yaw, turn));
}

/// The content's placement turn (identity without a character plugin).
fn turn(world: &World) -> Quat {
    content_turn(world.get_resource::<Characters>())
}

/// The placement turn of `characters`' content (identity without one).
pub(crate) fn content_turn(characters: Option<&Characters>) -> Quat {
    characters.map_or(Quat::IDENTITY, |c| c.content().placement_rotation())
}

/// Where a body stands and the yaw it faces.
fn pose(mover: &Mover, space: &ActorSpace) -> (Location, f32) {
    (Location::new(space.0.clone(), mover.loco.position), mover.loco.motion.yaw)
}

/// The pose of placement `id`'s body, if it has one.
fn body_of(world: &World, id: PlacementId) -> Option<(Location, f32)> {
    let mut bodies = world.try_query::<(&CharacterPlacement, &Mover, &ActorSpace)>()?;
    bodies.iter(world).find(|(p, ..)| p.0 == id).map(|(_, m, s)| pose(m, s))
}

/// Where and how to store a body standing at `body` for placement `p` of worldspace `ws`: `None`
/// when it has not moved from the placement's pose (module docs).
fn moved(ws: &WorldspaceId, p: &Placement, (at, yaw): &(Location, f32), turn: Quat) -> Option<(Location, Quat)> {
    let shifted = p.transform.pos.delta_f32(at.pos).with_y(0.0).length() > MOVED_M;
    let d = (yaw - placement_yaw(p.transform.rotation, turn)).rem_euclid(TAU);
    let turned = d.min(TAU - d) > TURNED_RAD;
    (shifted || turned || at.worldspace != *ws).then(|| (at.clone(), placement_rotation(*yaw, turn)))
}

#[cfg(test)]
#[path = "poses_tests.rs"]
mod tests;
