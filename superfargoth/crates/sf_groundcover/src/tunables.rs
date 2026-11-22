//! Scatter tunables (`groundcover.*`, spec `docs/specs/tunables.md`; groundcover.md "Tunables", all `Guess` until
//! measured). Presentation only (`Net::Presentation`): groundcover never feeds gameplay. A change applies to tiles
//! scattered after it (`Lifetime::Respawn`). The renderer's distances (blade, near, mid, shadow ranges) are the
//! renderer's tunables, not these.

use sf_tunables::{AnyDef, Def, Lifetime, Source, Unit, def};

pub const TILE_SIZE: Def<f32> = def!(float "groundcover.tile_size_m" = 16.0)
    .range(4.0, 128.0)
    .unit(Unit::Meters)
    .help("Edge of a world-anchored scatter tile (one task, one block of GPU instances per species); changing it moves every instance")
    .source(Source::Guess)
    .lifetime(Lifetime::Respawn);

pub const MAX_INSTANCES_PER_TILE: Def<u32> = def!(count "groundcover.max_instances_per_tile" = 16384)
    .range(0, 262_144)
    .unit(Unit::Count)
    .help("Most instances one species may place in one tile (the GPU slot block size); the scatter stops there and reports the tile as capped")
    .source(Source::Guess)
    .lifetime(Lifetime::Respawn);

pub const DENSITY_SCALE: Def<f32> = def!(float "groundcover.density_scale" = 1.0)
    .range(0.0, 4.0)
    .unit(Unit::Ratio)
    .help("Global multiplier on every species' density (per preset; saturates at one instance per footprint cell)")
    .source(Source::Guess)
    .lifetime(Lifetime::Respawn);

pub const TUNABLES: &[&dyn AnyDef] = &[&TILE_SIZE, &MAX_INSTANCES_PER_TILE, &DENSITY_SCALE];
