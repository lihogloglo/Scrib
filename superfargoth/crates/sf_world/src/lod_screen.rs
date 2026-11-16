//! The streaming screen's quality tunables (tunables slice T5a, spec `docs/specs/tunables.md`): the LOD
//! bias and the size-based draw distance (industry-audit.md J1) a host copies into its streaming
//! `LodScreen`. Each streamed placement's LOD switch distances and its draw end are computed from them when
//! the placement spawns and kept in its components, so a change applies to cells streamed after it
//! (`Lifetime::Respawn`); cells already resident keep theirs until they stream out and back.
//! Defaults are `sf_model::LodScreen::default()`'s (the High preset).

use sf_tunables::{Def, Group, Lifetime, Source, Unit, def};

/// `lod.bias`: `LodScreen::bias`, the global multiplier on LOD switch distances and the size-culling
/// distance (UE `r.ViewDistanceScale`, Bethesda `fLODFadeOutMultObjects`).
pub const LOD_BIAS: Def<f32> = def!(float "lod.bias" = 1.0)
    .range(0.25, 4.0)
    .unit(Unit::Ratio)
    .help("View-distance multiplier on LOD switch distances and size culling (streamed cells after the change)")
    .source(Source::Industry("UE r.ViewDistanceScale (scalability sg.ViewDistanceQuality)"))
    .group(Group::ViewDistance)
    .lifetime(Lifetime::Respawn);

/// `lod.size_cull.min_px`: `LodScreen::min_pixels`, the size-based draw distance (an object stops drawing
/// once its bounding sphere's diameter projects under this many reference pixels; 0: no size culling).
pub const SIZE_CULL_MIN_PX: Def<f32> = def!(float "lod.size_cull.min_px" = 2.0)
    .range(0.0, 64.0)
    .unit(Unit::Pixels)
    .help("Size culling: objects smaller than this on the reference screen stop drawing (0: off)")
    .source(Source::Reference("OpenMW small-feature culling, 2 px (industry-audit.md J1)"))
    .group(Group::ViewDistance)
    .lifetime(Lifetime::Respawn);

/// `lod.size_cull.min_distance_m`: `LodScreen::min_draw_distance`, no object is size-culled closer.
pub const SIZE_CULL_MIN_DISTANCE: Def<f32> = def!(float "lod.size_cull.min_distance_m" = 24.0)
    .range(0.0, 1000.0)
    .unit(Unit::Meters)
    .help("No object is size-culled closer than this")
    .source(Source::Guess)
    .group(Group::ViewDistance)
    .lifetime(Lifetime::Respawn);
