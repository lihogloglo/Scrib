//! The static scene's occupancy gauges (`scene/*`, scene-runtime-plan.md §9):
//! [`SceneSpace::stats`](sf_scene::SceneSpace::stats) summed over every space,
//! set every frame in `Last`, zeros included (a gauge that stops being set
//! reads stale).
//!
//! Set: `scene/cells/resident`, `scene/chains/{live,pending,high_water}`,
//! `scene/slots/{scene_live,scene_bump}` (the scene buffer's live slots and
//! high water), `scene/slots/{batch_live,batch_bump,batch_capacity}`,
//! `scene/slots/waste` (`batch_bump − batch_live`: what first fit costs over
//! perfect packing), `scene/batches/count`, `scene/promoted`,
//! `scene/overflow/instances`, and `scene/slots/batch_capacity_bytes`
//! (`batch_capacity ×` [`SLOT_BYTES`], bytes: the VRAM gauge, per view).
//!
//! The event counters (`scene/cells/{missing,failed}`, `scene/chains/failed`,
//! `scene/slots/overflow`, `scene/overflow/cells`, `scene/shift/*`) are
//! counted where they happen (`residency`, `shift`).
//!
//! Not built (no source yet): `scene/chains/loading`, `scene/cells/covered`,
//! `scene/batches/{ranges,instances}`, `scene/upload/bytes`, `scene/plan/ms`.

use bevy_ecs::prelude::*;
use sf_diag_bevy::{DiagCounters, sf_diag::unit};

use crate::residency::SceneSpaces;
use crate::upload::SLOT_BYTES;

/// `Last`, behind `resource_exists::<DiagCounters>`: publishes the `scene/*`
/// occupancy gauges (module docs).
pub fn publish_scene_gauges(spaces: Res<SceneSpaces>, counters: Res<DiagCounters>) {
    let mut sum = Totals::default();
    for state in spaces.0.values() {
        let s = state.space.stats();
        sum.cells += u64::from(s.cells);
        sum.chains_live += u64::from(s.chains.live);
        sum.chains_pending += u64::from(s.chains.pending);
        sum.chains_high_water += u64::from(s.chains.high_water);
        sum.scene_live += u64::from(s.scene.live_slots);
        sum.scene_bump += u64::from(s.scene.high_water);
        sum.batch_live += u64::from(s.batch_live_slots);
        sum.batch_bump += u64::from(s.batch_high_water);
        sum.batch_capacity += u64::from(s.batch_capacity);
        sum.batches += u64::from(s.batches);
        sum.promoted += u64::from(s.promoted);
        sum.overflow_instances += u64::from(s.overflow_instances);
    }
    #[expect(clippy::cast_precision_loss, reason = "gauges; exact below 2^53")]
    let set = |name: &str, unit: &str, value: u64| counters.set_named(name, unit, value as f64);
    set("scene/cells/resident", unit::COUNT, sum.cells);
    set("scene/chains/live", unit::COUNT, sum.chains_live);
    set("scene/chains/pending", unit::COUNT, sum.chains_pending);
    set("scene/chains/high_water", unit::COUNT, sum.chains_high_water);
    set("scene/slots/scene_live", unit::COUNT, sum.scene_live);
    set("scene/slots/scene_bump", unit::COUNT, sum.scene_bump);
    set("scene/slots/batch_live", unit::COUNT, sum.batch_live);
    set("scene/slots/batch_bump", unit::COUNT, sum.batch_bump);
    set("scene/slots/batch_capacity", unit::COUNT, sum.batch_capacity);
    set("scene/slots/batch_capacity_bytes", unit::BYTES, sum.batch_capacity * SLOT_BYTES);
    set("scene/slots/waste", unit::COUNT, sum.batch_bump.saturating_sub(sum.batch_live));
    set("scene/batches/count", unit::COUNT, sum.batches);
    set("scene/promoted", unit::COUNT, sum.promoted);
    set("scene/overflow/instances", unit::COUNT, sum.overflow_instances);
}

#[derive(Default)]
struct Totals {
    cells: u64,
    chains_live: u64,
    chains_pending: u64,
    chains_high_water: u64,
    scene_live: u64,
    scene_bump: u64,
    batch_live: u64,
    batch_bump: u64,
    batch_capacity: u64,
    batches: u64,
    promoted: u64,
    overflow_instances: u64,
}
