//! The floating-origin shift rewrite (O1, scene-runtime-plan.md §5.4): batch
//! output slots hold render-space transforms, placed in the frame of the
//! origin's cell at upload, so when `sf_world_bevy::WorldOrigin` moves every
//! resident cell's slots are rewritten in the new frame, **in the shift frame**.
//!
//! Not spread over frames: statics placed in two frames at once would tear.
//! One frame costs 64 B per resident slot (~3.5 MB at 55k instances), one
//! upload. A per-batch origin delta in `materialize.wesl` (no rewrite at all)
//! is a later vendor-patch slice.
//!
//! Ordering against writes already queued this frame needs nothing: the
//! rewrite is appended to [`SceneUploads::writes`] after them,
//! `extract_scene_uploads` appends new writes after the ones it kept for a
//! missing reservation, and `slot_runs` sorts **stably**, so for any slot the
//! write appended last (this one, in the new frame) wins.

use bevy_asset::Assets;
use bevy_ecs::prelude::*;
use bevy_math::IVec3;
use sf_diag_bevy::DiagCounters;
use sf_world::WorldspaceId;
use sf_world_bevy::WorldOrigin;

use crate::SceneTableAsset;
use crate::residency::{SceneSpaces, SceneUploads, count, queue_slot_writes};
use crate::upload::SLOT_BYTES;

/// `PostUpdate`, after `sf_world_bevy::WorldOriginSystems`: when the origin's
/// `(worldspace, cell)` changes, queues every slot of every resident cell of
/// the origin's worldspace (the space being rendered; a worldspace switch is
/// the same event) through `queue_slot_writes` in the new frame
/// (`SceneSpace::cell_writes`: promoted and overflow-split placements stay
/// inactive). The first frame only records the origin. Counts
/// `scene/shift/rewrites` (cells) and `scene/shift/rewrite_bytes`.
pub fn rewrite_on_origin_shift(
    origin: Option<Res<WorldOrigin>>,
    mut last: Local<Option<(Option<WorldspaceId>, IVec3)>>,
    spaces: Res<SceneSpaces>,
    tables: Res<Assets<SceneTableAsset>>,
    mut uploads: ResMut<SceneUploads>,
    counters: Option<Res<DiagCounters>>,
) {
    let Some(origin) = origin else { return };
    let now = (origin.worldspace.clone(), origin.cell);
    if last.as_ref() == Some(&now) {
        return;
    }
    let previous = last.replace(now);
    let Some(previous) = previous else { return };
    let Some(state) = origin.worldspace.as_ref().and_then(|ws| spaces.0.get(ws)) else { return };
    let mut cells = 0u32;
    let mut slots = 0u64;
    for (cell, _) in state.space.cells() {
        let Some(view) = state.tables.get(&cell).and_then(|t| tables.get(t)).and_then(|a| a.view().ok()) else {
            log::error!("sf_scene_bevy: cell {cell}'s table went away while it was resident; not rewritten on the origin shift");
            continue;
        };
        let writes = match state.space.cell_writes(cell, &view) {
            Ok(writes) => writes,
            Err(e) => {
                log::error!("sf_scene_bevy: cell {cell} cannot be rewritten on the origin shift: {e}");
                continue;
            }
        };
        let resolved = queue_slot_writes(&tables, state, cell, &writes, origin.cell);
        slots += resolved.len() as u64;
        cells += 1;
        uploads.writes.extend(resolved);
    }
    let bytes = slots * SLOT_BYTES;
    log::debug!(
        "sf_scene_bevy: origin shift {:?} -> {:?}: {cells} cells, {slots} slots ({bytes} B) rewritten",
        previous,
        (&origin.worldspace, origin.cell)
    );
    let counters = counters.as_deref().map(|c| &*c.0);
    count(counters, "scene/shift/rewrites", f64::from(cells));
    #[allow(clippy::cast_precision_loss)] // bytes, far below 2^52
    count(counters, "scene/shift/rewrite_bytes", bytes as f64);
}
