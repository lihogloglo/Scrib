//! The cached point-light shadows' view of the scene (perf audit 2026-11-11, fix 5).
//!
//! `sf_shadow_cache_bevy` bakes static casters into each shadowed light once and
//! recaptures a light when a static in its range changes. It sees a batch only
//! as one `GpuBatchedMesh3d` entity (a static caster), not the instances behind
//! its slots, so this module tells it where they changed: a cell committed
//! (its slots written), a cell streamed out (its ranges released) and a
//! placement promoted out of the scene (its slots written inactive) each send
//! one [`ShadowCastersChanged`] around the cell's table bounds, in the render
//! frame of the floating origin's cell (the frame the batch slots are placed in).
//!
//! The diff runs over the resident tables (`SpaceState::tables`) rather than
//! inside `SceneHook::upload`/`release`, so the residency code stays as it is
//! and every path that commits or drops a cell is covered by one comparison.
//! Origin shifts send nothing: the shadow cache moves its lights with
//! `ShadowOriginShift` and the shift rewrite places the same statics again.
//! Without `ShadowCachePlugin` (no `Messages<ShadowCastersChanged>`) this is a no-op.

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use bevy_asset::Assets;
use bevy_ecs::message::Messages;
use bevy_ecs::prelude::*;
use bevy_math::{IVec3, Vec3};
use sf_coords::WorldPos;
use sf_scene::Aabb;
use sf_shadow_cache_bevy::ShadowCastersChanged;
use sf_world::WorldspaceId;
use sf_world_bevy::WorldOrigin;

use crate::SceneTableAsset;
use crate::residency::{PromotePlacement, SceneSpaces};

/// A resident cell's table bounds: its origin cell and cell-relative box.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CellBounds {
    pub origin_cell: IVec3,
    pub aabb: Aabb,
}

/// The cells reported so far ([`report_shadow_casters`]'s memory).
#[derive(Resource, Default)]
pub struct ReportedCells(pub HashMap<(WorldspaceId, IVec3), CellBounds>);

/// Brings `known` up to `resident` and returns every cell that came or went,
/// with its bounds. `resident` yields each resident cell with its bounds,
/// `None` while its table can't be read yet (it is reported once it can).
fn diff_cells<K: Clone + Eq + Hash>(
    known: &mut HashMap<K, CellBounds>,
    resident: impl IntoIterator<Item = (K, Option<CellBounds>)>,
) -> Vec<(K, CellBounds)> {
    let mut changed = Vec::new();
    let mut seen = HashSet::with_capacity(known.len());
    for (key, bounds) in resident {
        if !known.contains_key(&key)
            && let Some(b) = bounds
        {
            known.insert(key.clone(), b);
            changed.push((key.clone(), b));
        }
        seen.insert(key);
    }
    known.retain(|key, b| {
        let keep = seen.contains(key);
        if !keep {
            changed.push((key.clone(), *b));
        }
        keep
    });
    changed
}

/// The message around `bounds`, placed in the frame of `frame` (the origin's cell).
pub fn casters_changed(bounds: CellBounds, frame: IVec3) -> ShadowCastersChanged {
    let offset = sf_scene::cell_offset(bounds.origin_cell, WorldPos::new(frame, Vec3::ZERO));
    let b = bounds.aabb.translated(offset);
    ShadowCastersChanged::from_aabb(b.min, b.max)
}

/// `PostUpdate`, after `WorldOriginSystems` and before `ShadowCacheSystems`:
/// sends a [`ShadowCastersChanged`] for each cell of the rendered worldspace
/// (the origin's; every space while there is no origin) that committed or
/// streamed out since the last run, and for each cell a placement was promoted out of.
pub fn report_shadow_casters(
    spaces: Res<SceneSpaces>,
    tables: Res<Assets<SceneTableAsset>>,
    origin: Option<Res<WorldOrigin>>,
    mut promotions: MessageReader<PromotePlacement>,
    mut reported: ResMut<ReportedCells>,
    out: Option<ResMut<Messages<ShadowCastersChanged>>>,
) {
    let Some(mut out) = out else {
        promotions.clear();
        return;
    };
    let frame = origin.as_ref().map_or(IVec3::ZERO, |o| o.cell);
    let shown = |ws: &WorldspaceId| origin.as_ref().and_then(|o| o.worldspace.as_ref()).is_none_or(|o| o == ws);
    let resident = spaces.0.iter().flat_map(|(ws, state)| {
        state.tables.iter().map(|(cell, handle)| {
            let bounds = tables.get(handle).and_then(|a| a.view().ok()).map(|v| CellBounds { origin_cell: v.origin_cell(), aabb: v.aabb() });
            ((ws.clone(), *cell), bounds)
        })
    });
    for ((ws, _), b) in diff_cells(&mut reported.0, resident) {
        if shown(&ws) {
            out.write(casters_changed(b, frame));
        }
    }
    for p in promotions.read() {
        if shown(&p.worldspace)
            && let Some(b) = reported.0.get(&(p.worldspace.clone(), p.cell))
        {
            out.write(casters_changed(*b, frame));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bounds(x: f32) -> CellBounds {
        CellBounds { origin_cell: IVec3::ZERO, aabb: Aabb { min: Vec3::splat(x), max: Vec3::splat(x + 1.0) } }
    }

    #[test]
    fn diff_reports_cells_that_came_and_went_once() {
        let mut known = HashMap::new();
        // A table not readable yet waits; a new one is reported once.
        assert_eq!(diff_cells(&mut known, [(1, Some(bounds(1.0))), (2, None)]), vec![(1, bounds(1.0))]);
        assert_eq!(diff_cells(&mut known, [(1, Some(bounds(1.0))), (2, Some(bounds(2.0)))]), vec![(2, bounds(2.0))]);
        assert!(diff_cells(&mut known, [(1, Some(bounds(1.0))), (2, Some(bounds(2.0)))]).is_empty());
        // Streamed out: reported with the bounds it was reported with.
        assert_eq!(diff_cells(&mut known, [(2, Some(bounds(2.0)))]), vec![(1, bounds(1.0))]);
        assert_eq!(known.len(), 1);
    }

    #[test]
    fn message_is_placed_in_the_origin_frame() {
        let b = CellBounds { origin_cell: IVec3::new(1, 0, 0), aabb: Aabb { min: Vec3::new(-1.0, 0.0, -1.0), max: Vec3::new(1.0, 2.0, 1.0) } };
        let here = casters_changed(b, IVec3::new(1, 0, 0));
        assert!(here.center.abs_diff_eq(Vec3::new(0.0, 1.0, 0.0), 1e-4));
        assert!((here.radius - Vec3::new(2.0, 2.0, 2.0).length() * 0.5).abs() < 1e-4);
        // Seen from the neighbouring cell's frame the same box sits one cell over.
        let there = casters_changed(b, IVec3::ZERO);
        assert!((there.center - here.center).length() > 1.0);
        assert!((there.radius - here.radius).abs() < 1e-4);
    }
}
