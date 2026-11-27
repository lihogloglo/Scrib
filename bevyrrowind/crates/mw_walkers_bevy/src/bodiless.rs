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
//! it walked into, not at once (`AI10c3c2b`, `Streaming::spawn_into`). The kept rotation is
//! written unchanged (the facing it walked with is not known at tier 0). No proof line yet (it
//! comes with `AI10c3c2b`).

use std::collections::HashMap;
use std::sync::PoisonError;

use bevy_ecs::prelude::*;
use bevy_math::Quat;
use sf_ai_bevy::{AiActor, AiLogical};
use sf_ids::ActorKey;
use sf_world::{Location, Partition, Placement, PlacementId, WorldspaceId};
use sf_world_bevy::{Streaming, WorldOverlay};

use crate::logical::{Tier0World, WorldGraphEnv};
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

/// The fixed-tick system around [`follow`] (module docs): the tier-0 env as `step_tier0` builds
/// it (no walk speed, placement or body lookups: a routed state reads only its route), the
/// streaming directory's partitions, one overlay lock per write.
#[allow(clippy::needless_pass_by_value)]
pub(crate) fn follow_bodiless(
    world: Tier0World,
    store: Res<AiLogical>,
    resident: Option<ResMut<ResidentActors>>,
    overlay: Option<Res<WorldOverlay>>,
    streaming: Option<Res<Streaming>>,
    bodies: Query<&AiActor>,
) {
    let (Some(mut resident), Some(overlay), Some(streaming)) = (resident, overlay, streaming) else { return };
    if !resident.has_away() {
        return;
    }
    let mut keys: Vec<ActorKey> = bodies.iter().map(|a| a.0).collect();
    keys.sort_unstable();
    let door = |id| world.door(id);
    let no_speed = |_: ActorKey| None;
    let nowhere = |_: PlacementId| None;
    let no_body = |_: ActorKey| None;
    let env = WorldGraphEnv::new(world.graph(), &door, world.traverse(), &no_speed, &nowhere, &no_body);
    let (now, scale) = (world.now(), world.time_scale());
    follow(
        resident.away_mut(),
        &store,
        |k| keys.binary_search(&k).is_ok(),
        |k| store.position_at(k, now, &env, scale),
        |ws| streaming.scheduler().directory(ws).map(|d| d.partition),
        |p, to, rotation| overlay.0.write().unwrap_or_else(PoisonError::into_inner).move_to(p, to, rotation),
    );
}

#[cfg(test)]
#[path = "bodiless_tests.rs"]
mod tests;
