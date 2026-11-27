//! Live view distance (tunables slice T5c): the `lod.*` tunables (`lod.bias`, `lod.size_cull.*`, bound into
//! `Streaming::set_lod_screen` by `quality_presets::bind_live`) are `Live`: `sf_world_bevy` re-ranges the
//! resident LOD chains in place on a screen change (no restream, no state touched; UE
//! `r.ViewDistanceScale` / Unity `lodBias` behaviour). This binds the re-range's per-frame budget,
//! `lod.live.chains_per_frame`, so a quality switch is spread over a few frames.
//!
//! Not reached live: `far.handoff_scale` (the near rings are sized from it at start, `Restart`), uncooked
//! models (their subtree end is set once), the static scene (`--scene`, no LOD or size cull yet).

use bevy::prelude::*;
use sf_tunables_bevy::TunablesAppExt;
use sf_world::STREAM_PENDING_SPAWNS;
use sf_world::lod_screen::LOD_LIVE_BUDGET;
use sf_world_bevy::Streaming;

/// Binds `lod.live.chains_per_frame` into the streaming runtime (registers it).
pub fn bind(app: &mut App) {
    let _budget = app
        .add_tunable(&LOD_LIVE_BUDGET)
        .bind_resource(|s: &mut Streaming, v| s.set_lod_live_budget(usize::try_from(v).unwrap_or(usize::MAX)));
    // Queued late spawns (a broken crate's spilled items, IN38b3a) placed per frame.
    let _pending = app
        .add_tunable(&STREAM_PENDING_SPAWNS)
        .bind_resource(|s: &mut Streaming, v| s.set_pending_spawns_per_frame(usize::try_from(v).unwrap_or(usize::MAX)));
}
