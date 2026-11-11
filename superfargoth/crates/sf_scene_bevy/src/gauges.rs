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
//! Per batch group, so a class's capacity can be tuned from its residency
//! (`SpaceConfig::class_capacity`): `scene/batch/<group>/{count,live,
//! high_water,capacity,overflow}` (batches, live output slots, first fit's
//! high water, reserved slots per view, instances overflowed to entities so
//! far; all counts), where `<group>` is a material class in snake case
//! ([`group_name`]: `opaque`, `masked_two_sided`, `opaque_wind`, ...) or
//! `chain` for every single-chain batch.
//!
//! The event counters (`scene/cells/{missing,failed}`, `scene/chains/failed`,
//! `scene/slots/overflow`, `scene/overflow/cells`, `scene/shift/*`) are
//! counted where they happen (`residency`, `shift`).
//!
//! Not built (no source yet): `scene/chains/loading`, `scene/cells/covered`,
//! `scene/batches/{ranges,instances}`, `scene/upload/bytes`, `scene/plan/ms`.

use std::sync::LazyLock;

use bevy_ecs::prelude::*;
use sf_diag_bevy::{DiagCounters, sf_diag::unit};
use sf_scene::{BatchGroupStats, BatchKey, MaterialClass};

use crate::residency::SceneSpaces;
use crate::upload::SLOT_BYTES;

/// The `<group>` of `scene/batch/<group>/*`, by `MaterialClass as usize`, then `chain`.
const GROUPS: [&str; MaterialClass::ALL.len() + 1] = [
    "opaque",
    "opaque_two_sided",
    "masked",
    "masked_two_sided",
    "opaque_wind",
    "opaque_wind_two_sided",
    "masked_wind",
    "masked_wind_two_sided",
    "chain",
];

/// The gauge group of the batch keyed `key`: its class's name, or `chain` for a
/// single-chain batch (module docs).
#[must_use]
pub fn group_name(key: &BatchKey) -> &'static str {
    if key.chain.is_some() { GROUPS[MaterialClass::ALL.len()] } else { GROUPS[key.class as usize] }
}

/// `scene/batch/<group>/{count,live,high_water,capacity,overflow}`, built once.
static GROUP_GAUGES: LazyLock<Vec<[String; 5]>> = LazyLock::new(|| {
    GROUPS
        .iter()
        .map(|g| ["count", "live", "high_water", "capacity", "overflow"].map(|field| format!("scene/batch/{g}/{field}")))
        .collect()
});

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
        for (group, stats) in sum.groups.iter_mut().zip(s.classes.iter().chain([&s.chain_batches])) {
            group.add(stats);
        }
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
    for (names, group) in GROUP_GAUGES.iter().zip(&sum.groups) {
        let values = [group.batches, group.live, group.high_water, group.capacity, group.overflow];
        for (name, value) in names.iter().zip(values) {
            set(name, unit::COUNT, value);
        }
    }
}

/// One batch group summed over the spaces.
#[derive(Default, Clone, Copy)]
struct GroupTotals {
    batches: u64,
    live: u64,
    high_water: u64,
    capacity: u64,
    overflow: u64,
}

impl GroupTotals {
    fn add(&mut self, s: &BatchGroupStats) {
        self.batches += u64::from(s.batches);
        self.live += u64::from(s.live_slots);
        self.high_water += u64::from(s.high_water);
        self.capacity += u64::from(s.capacity);
        self.overflow += u64::from(s.overflow_instances);
    }
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
    groups: [GroupTotals; MaterialClass::ALL.len() + 1],
}
