//! The floating-origin shift rewrite (O1, scene-runtime-plan.md §5.4): batch
//! output slots hold render-space transforms, placed in their space's own frame
//! (`SpaceState::frame`, a floating-origin cell), so when `sf_world_bevy::WorldOrigin`
//! moves in a worldspace whose space sits in another frame, every resident slot of
//! that space is rewritten in the new frame, **in the shift frame**. A space whose
//! worldspace is not the active one keeps its frame (its slots stay consistent for
//! whatever still views it) until its worldspace is active again.
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

use std::collections::HashMap;

use bevy_asset::Assets;
use bevy_ecs::prelude::*;
use bevy_math::IVec3;
use sf_diag_bevy::DiagCounters;
use sf_world::WorldspaceId;
use sf_world_bevy::WorldOrigin;

use crate::SceneTableAsset;
use crate::residency::{SceneSpaces, SceneUploads, count, queue_slot_writes};
use crate::upload::SLOT_BYTES;

/// Each worldspace's last known floating-origin cell (`WorldOrigin.cell` the last
/// [`rewrite_on_origin_shift`] saw while that worldspace was the origin's), recorded even
/// while it has no scene: the frame a space created later starts in.
#[derive(Resource, Default, Debug)]
pub struct OriginFrames(HashMap<WorldspaceId, IVec3>);

impl OriginFrames {
    /// The frame a new space of `worldspace` starts in: the origin's cell when `worldspace`
    /// is the origin's, else the worldspace's last known origin, else `IVec3::ZERO` (a
    /// worldspace never drawn; its first activation rewrites it).
    ///
    /// Why the last known origin for an inactive worldspace: it is the frame every resident
    /// slot of that worldspace already sits in (the last rewrite moved them there), and the one
    /// `WorldOrigin` returns to on the way back (`update_world_origin` reads that worldspace's
    /// grid floating origin, unchanged while its camera did not move), so the return rewrites
    /// nothing or only what moved. The active origin's cell is another worldspace's frame:
    /// statics written in it are misplaced for any view of their own worldspace (RO-18).
    pub fn frame_for(&self, worldspace: &WorldspaceId, origin: Option<&WorldOrigin>) -> IVec3 {
        match origin {
            Some(o) if o.worldspace.as_ref() == Some(worldspace) => o.cell,
            _ => self.0.get(worldspace).copied().unwrap_or(IVec3::ZERO),
        }
    }
}

/// `PostUpdate`, after `sf_world_bevy::WorldOriginSystems`: records the origin's cell as its
/// worldspace's last known origin ([`OriginFrames`]) and, when the space of the origin's
/// worldspace is in another frame (`SpaceState::frame`: the origin shifted, or the worldspace
/// became active again somewhere else), moves the space to the origin's cell and queues every
/// slot of every resident cell through `queue_slot_writes` in it (`SceneSpace::cell_writes`:
/// promoted and overflow-split placements stay inactive). Counts `scene/shift/rewrites`
/// (cells) and `scene/shift/rewrite_bytes`.
pub fn rewrite_on_origin_shift(
    origin: Option<Res<WorldOrigin>>,
    mut frames: ResMut<OriginFrames>,
    mut spaces: ResMut<SceneSpaces>,
    tables: Res<Assets<SceneTableAsset>>,
    mut uploads: ResMut<SceneUploads>,
    counters: Option<Res<DiagCounters>>,
) {
    let Some(origin) = origin else { return };
    let Some(worldspace) = origin.worldspace.as_ref() else { return };
    if frames.0.get(worldspace) != Some(&origin.cell) {
        frames.0.insert(worldspace.clone(), origin.cell);
    }
    let Some(state) = spaces.0.get_mut(worldspace) else { return };
    let previous = state.frame();
    if previous == origin.cell {
        return;
    }
    state.set_frame(origin.cell);
    let state = &*state;
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
        let resolved = queue_slot_writes(&tables, state, cell, &writes);
        slots += resolved.len() as u64;
        cells += 1;
        uploads.writes.extend(resolved);
    }
    let bytes = slots * SLOT_BYTES;
    log::debug!(
        "sf_scene_bevy: origin shift in {worldspace}: frame {previous} -> {}: {cells} cells, {slots} slots ({bytes} B) rewritten",
        origin.cell
    );
    let counters = counters.as_deref().map(|c| &*c.0);
    count(counters, "scene/shift/rewrites", f64::from(cells));
    #[allow(clippy::cast_precision_loss)] // bytes, far below 2^52
    count(counters, "scene/shift/rewrite_bytes", bytes as f64);
}
