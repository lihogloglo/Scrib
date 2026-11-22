//! Mesh renderer tunables (`groundcover.render.*`, spec `docs/specs/tunables.md`; groundcover.md "Tunables", all
//! `Guess` until GC-2b measures them). Presentation only (`Net::Presentation`). The ranges, the slot budget and
//! the job budget rebuild every tile when they change (`Lifetime::Respawn`); the wind values apply live to every
//! groundcover material. The scatter's own tunables (`groundcover.tile_size_m`, ...) are the core's
//! (`sf_groundcover::tunables`), registered by the plugin beside these.

use sf_tunables::{AnyDef, Def, Lifetime, Source, Unit, def};

pub const MESH_RANGE: Def<f32> = def!(float "groundcover.render.mesh_range_m" = 60.0)
    .range(4.0, 400.0)
    .unit(Unit::Meters)
    .help("Mesh groundcover draw distance: tiles enter the ring when their nearest point is this close, instances fade out by it")
    .source(Source::Guess)
    .lifetime(Lifetime::Respawn);

pub const HYSTERESIS: Def<f32> = def!(float "groundcover.render.hysteresis_m" = 8.0)
    .range(0.0, 64.0)
    .unit(Unit::Meters)
    .help("Extra distance before a resident tile (or a shadow-casting one) leaves its ring, so a viewer on a boundary does not thrash")
    .source(Source::Guess)
    .lifetime(Lifetime::Respawn);

pub const SHADOW_RANGE: Def<f32> = def!(float "groundcover.render.shadow_range_m" = 30.0)
    .range(0.0, 200.0)
    .unit(Unit::Meters)
    .help("Species with near shadows cast them in tiles wholly within this distance (clamped so casters never reach the fade band); 0 = no groundcover shadows")
    .source(Source::Guess)
    .lifetime(Lifetime::Respawn);

pub const FADE_WIDTH: Def<f32> = def!(float "groundcover.render.fade_width_m" = 10.0)
    .range(0.0, 100.0)
    .unit(Unit::Meters)
    .help("Width of the dithered distance fade before the mesh range: each instance vanishes at its own seeded distance in this band")
    .source(Source::Guess)
    .lifetime(Lifetime::Respawn);

pub const FADE_SHRINK: Def<f32> = def!(float "groundcover.render.fade_shrink_m" = 2.0)
    .range(0.0, 20.0)
    .unit(Unit::Meters)
    .help("Distance over which a fading instance shrinks to nothing before its seeded cut (0 = it pops)")
    .source(Source::Guess)
    .lifetime(Lifetime::Respawn);

pub const MAX_BLOCKS_PER_BATCH: Def<u32> = def!(count "groundcover.render.max_blocks_per_batch" = 256)
    .range(1, 4096)
    .unit(Unit::Count)
    .help("Upper bound on tile blocks (one per tile and species) in one GPU batch; a batch is sized to its ring's worst case under it. GPU cost scales with capacity x views")
    .source(Source::Guess)
    .lifetime(Lifetime::Respawn);

pub const JOBS_PER_FRAME: Def<u32> = def!(count "groundcover.render.jobs_per_frame" = 4)
    .range(1, 64)
    .unit(Unit::Count)
    .help("Tile scatter tasks started per frame (nearest first)")
    .source(Source::Guess)
    .lifetime(Lifetime::Respawn);

pub const MAX_JOBS_IN_FLIGHT: Def<u32> = def!(count "groundcover.render.max_jobs_in_flight" = 8)
    .range(1, 256)
    .unit(Unit::Count)
    .help("Tile scatter tasks running at once on the async compute pool")
    .source(Source::Guess)
    .lifetime(Lifetime::Respawn);

pub const GROUND_CACHE_SPACING: Def<f32> = def!(float "groundcover.render.ground_cache_m" = 0.5)
    .range(0.0, 4.0)
    .unit(Unit::Meters)
    .help("Lattice spacing of the per-tile ground cache shared by every species of a tile (bilinear); used only when it takes fewer ground samples than direct sampling. 0 = always sample directly")
    .source(Source::Guess)
    .lifetime(Lifetime::Respawn);

pub const WIND_SCALE: Def<f32> = def!(float "groundcover.render.wind_scale" = 1.0)
    .range(0.0, 4.0)
    .unit(Unit::Ratio)
    .help("Global multiplier on the wind bend of every groundcover species")
    .source(Source::Guess)
    .lifetime(Lifetime::Live);

pub const BEND_PER_MPS: Def<f32> = def!(float "groundcover.render.bend_per_mps" = 0.05)
    .range(0.0, 0.5)
    .unit(Unit::Ratio)
    .help("Tip displacement per m/s of wind, as a fraction of the plant's height (capped at most of the height)")
    .source(Source::Guess)
    .lifetime(Lifetime::Live);

pub const PHASE_OFFSET: Def<f32> = def!(float "groundcover.render.phase_offset_m" = 3.0)
    .range(0.0, 50.0)
    .unit(Unit::Meters)
    .help("How far upwind or downwind each instance samples the gust field (from its wind phase), so neighbours do not sway in lockstep")
    .source(Source::Guess)
    .lifetime(Lifetime::Live);

pub const TUNABLES: &[&dyn AnyDef] = &[
    &MESH_RANGE,
    &HYSTERESIS,
    &SHADOW_RANGE,
    &FADE_WIDTH,
    &FADE_SHRINK,
    &MAX_BLOCKS_PER_BATCH,
    &JOBS_PER_FRAME,
    &MAX_JOBS_IN_FLIGHT,
    &GROUND_CACHE_SPACING,
    &WIND_SCALE,
    &BEND_PER_MPS,
    &PHASE_OFFSET,
];
