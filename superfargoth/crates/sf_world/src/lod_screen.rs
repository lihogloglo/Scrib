//! The streaming screen's quality tunables (tunables slices T5a, T5c; spec `docs/specs/tunables.md`): the
//! LOD bias and the size-based draw distance (industry-audit.md J1) a host copies into its streaming
//! `LodScreen`, and the budget of the live re-range. Each streamed placement's LOD switch distances and
//! draw end are computed from the screen when it spawns; its LOD parts keep the inputs, so a change
//! re-ranges the resident ones in place over the next frames (`sf_world_bevy` `lod_live`, the UE
//! `r.ViewDistanceScale` / Unity `lodBias` behaviour): `Lifetime::Live`.
//! Defaults are `sf_model::LodScreen::default()`'s (the High preset).

use sf_tunables::{Def, Group, Lifetime, Source, Unit, def};

/// `lod.bias`: `LodScreen::bias`, the global multiplier on LOD switch distances and the size-culling
/// distance (UE `r.ViewDistanceScale`, Bethesda `fLODFadeOutMultObjects`).
pub const LOD_BIAS: Def<f32> = def!(float "lod.bias" = 1.0)
    .range(0.25, 4.0)
    .unit(Unit::Ratio)
    .help("View-distance multiplier on LOD switch distances and size culling (resident cells re-ranged over a few frames)")
    .source(Source::Industry("UE r.ViewDistanceScale (scalability sg.ViewDistanceQuality)"))
    .group(Group::ViewDistance)
    .lifetime(Lifetime::Live);

/// `lod.size_cull.min_px`: `LodScreen::min_pixels`, the size-based draw distance (an object stops drawing
/// once its bounding sphere's diameter projects under this many reference pixels; 0: no size culling).
pub const SIZE_CULL_MIN_PX: Def<f32> = def!(float "lod.size_cull.min_px" = 2.0)
    .range(0.0, 64.0)
    .unit(Unit::Pixels)
    .help("Size culling: objects smaller than this on the reference screen stop drawing (0: off)")
    .source(Source::Reference("OpenMW small-feature culling, 2 px (industry-audit.md J1)"))
    .group(Group::ViewDistance)
    .lifetime(Lifetime::Live);

/// `lod.size_cull.min_distance_m`: `LodScreen::min_draw_distance`, no object is size-culled closer.
pub const SIZE_CULL_MIN_DISTANCE: Def<f32> = def!(float "lod.size_cull.min_distance_m" = 24.0)
    .range(0.0, 1000.0)
    .unit(Unit::Meters)
    .help("No object is size-culled closer than this")
    .source(Source::Guess)
    .group(Group::ViewDistance)
    .lifetime(Lifetime::Live);

/// `lod.live.chains_per_frame`: after a screen change, the resident LOD chains (one per placement or
/// precombine bucket) re-ranged per frame. Each changed range re-extracts its mesh instances, so a
/// preset switch is spread over frames instead of re-extracting every resident mesh at once (~10
/// frames for a 40 k-placement exterior at the default).
pub const LOD_LIVE_BUDGET: Def<u32> = def!(count "lod.live.chains_per_frame" = 4096)
    .range(1, 1_000_000)
    .unit(Unit::Count)
    .help("LOD chains re-ranged per frame after a view-distance change")
    .source(Source::Guess)
    .group(Group::ViewDistance);
