//! Mesh renderer tunables (`groundcover.render.*`, spec `docs/specs/tunables.md`; groundcover.md "Tunables", all
//! `Guess` until GC-2b measures them). Presentation only (`Net::Presentation`). The ranges, the slot budget and
//! the job budget rebuild every tile when they change (`Lifetime::Respawn`); the wind values apply live to every
//! groundcover material, and [`ENABLED`] switches the whole renderer off and on live ([`GROUNDCOVER`]). The
//! scatter's own tunables (`groundcover.tile_size_m`, ...) are the core's
//! (`sf_groundcover::tunables`), registered by the plugin beside these.

use sf_tunables::{AnyDef, Category, Cost, Def, Effect, FeatureSwitch, Group, Lifetime, Scope, Source, Unit, def};

/// `groundcover.enabled`: the renderer runs (feature-switches.md "Live in Bevy", slice FS-L1). Off despawns every
/// batch (GPU buffers freed, out of every view, shadow and GPU preprocessing), drops the scatter jobs and stops the
/// per-frame systems; on rebuilds the rings around the viewer as after a teleport.
pub const ENABLED: Def<bool> = def!(bool "groundcover.enabled" = true)
    .help("Groundcover (grass, ferns, flowers) is drawn; off skips its scatter, uploads and draws")
    .source(Source::Owner("2026-11-23"))
    .group(Group::Foliage)
    .scope(Scope::Player)
    .lifetime(Lifetime::Live);

/// The feature switch on [`ENABLED`] (registered by [`GroundcoverPlugin`](crate::GroundcoverPlugin) beside its
/// tunables, slice FS-2b). Its batches draw in the main passes (no pass of their own), so the CPU spans are its cost
/// tags.
pub const GROUNDCOVER: FeatureSwitch = FeatureSwitch::new(&ENABLED)
    .label("Groundcover")
    .category(Category::Foliage)
    .effect(Effect::Skips)
    .cost(&[Cost::Cpu("sf_groundcover_bevy::drive"), Cost::Cpu("sf_groundcover_bevy::terrain_ground::refresh_terrain_ground")])
    .at(file!(), line!());

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
    .help("Upper bound on tile blocks (one per tile and species) in one GPU batch: a batch grows on demand up to its ring's worst case under it. GPU cost scales with capacity x views")
    .source(Source::Guess)
    .lifetime(Lifetime::Respawn);

pub const INITIAL_BLOCKS: Def<u32> = def!(count "groundcover.render.initial_blocks" = 8)
    .range(1, 4096)
    .unit(Unit::Count)
    .help("Tile blocks a GPU batch opens with when a resident tile first needs its model; it doubles when full, up to max_blocks_per_batch")
    .source(Source::Guess)
    .lifetime(Lifetime::Respawn);

pub const BLOCK_SIGMA: Def<f32> = def!(float "groundcover.render.block_sigma" = 5.0)
    .range(0.0, 20.0)
    .unit(Unit::Ratio)
    .help("Headroom of a tile block over a species' expected instances per tile, in standard deviations (sqrt of the expected count); a tile past it drops its extra instances (tiles/capped)")
    .source(Source::Guess)
    .lifetime(Lifetime::Respawn);

pub const RESERVE_WORST_CASE: Def<bool> = def!(bool "groundcover.render.reserve_worst_case" = false)
    .help("A/B arm: every resolved model's batch opens at once at its ring's worst-case capacity with one block per grid cell (the pre-GC-M1 sizing) instead of on demand, sized to the expected count")
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

pub const BEND_PER_MPS: Def<f32> = def!(float "groundcover.render.bend_per_mps" = 0.15)
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

pub const GUST_CONTRAST: Def<f32> = def!(float "groundcover.render.gust_contrast" = 0.7)
    .range(0.0, 1.0)
    .unit(Unit::Ratio)
    .help("Minimum gustiness of the wind field as groundcover reads it: lulls slow and gust fronts speed the bend by up to this fraction, even when the weather's own gustiness is lower")
    .source(Source::Guess)
    .lifetime(Lifetime::Live);

pub const SWAY_AMOUNT: Def<f32> = def!(float "groundcover.render.sway_amount" = 0.35)
    .range(0.0, 2.0)
    .unit(Unit::Ratio)
    .help("Per-instance sway on top of the wind lean, as a fraction of the wind speed (along the wind; half of it across); 0 = lean only")
    .source(Source::Guess)
    .lifetime(Lifetime::Live);

// `sf_tunables::Unit` has no frequency unit: the Hz tunables are `Unit::None` with the unit in the help.
pub const SWAY_HZ: Def<f32> = def!(float "groundcover.render.sway_hz" = 0.9)
    .range(0.0, 5.0)
    .unit(Unit::None)
    .help("Sway frequency in Hz (the cross-wind sway runs at 0.63 of it); rounded to whole cycles per hour")
    .source(Source::Guess)
    .lifetime(Lifetime::Live);

pub const FLUTTER_AMOUNT: Def<f32> = def!(float "groundcover.render.flutter_amount" = 0.12)
    .range(0.0, 1.0)
    .unit(Unit::Ratio)
    .help("Fast cross-wind tip flutter, as a fraction of the wind speed, each card of a clump slightly out of step; 0 = none")
    .source(Source::Guess)
    .lifetime(Lifetime::Live);

pub const FLUTTER_HZ: Def<f32> = def!(float "groundcover.render.flutter_hz" = 3.5)
    .range(0.0, 20.0)
    .unit(Unit::None)
    .help("Flutter frequency in Hz; rounded to whole cycles per hour")
    .source(Source::Guess)
    .lifetime(Lifetime::Live);

pub const BLADES_BLOCK: Def<f32> = def!(float "groundcover.blades.block_m" = 64.0)
    .range(16.0, 256.0)
    .unit(Unit::Meters)
    .help("Side of one world-anchored grass blade field (one bevy_grass field per block); keep it a multiple of bevy_grass's 16 m chunk")
    .source(Source::Guess)
    .lifetime(Lifetime::Respawn);

pub const BLADES_RING: Def<u32> = def!(count "groundcover.blades.ring_blocks" = 5)
    .range(1, 15)
    .unit(Unit::Count)
    .help("Blade fields per side of the square around the viewer's block (odd; an even value is rounded up)")
    .source(Source::Guess)
    .lifetime(Lifetime::Respawn);

/// `groundcover.blades.hysteresis_blocks`: a spawned blade field is despawned only once its block is this many blocks
/// outside the ring on some side, so a viewer pacing across a block edge spawns and despawns nothing (slice GC-3b3).
pub const BLADES_HYSTERESIS: Def<u32> = def!(count "groundcover.blades.hysteresis_blocks" = 1)
    .range(0, 4)
    .unit(Unit::Count)
    .help("Blocks a blade field may lag outside the ring around the viewer before it is despawned")
    .source(Source::Guess)
    .lifetime(Lifetime::Respawn);

pub const BLADES_CELL: Def<f32> = def!(float "groundcover.blades.cell_m" = 0.5)
    .range(0.25, 4.0)
    .unit(Unit::Meters)
    .help("Cell of a blade field's ground height and density grids (snapped to divide the block)")
    .source(Source::Guess)
    .lifetime(Lifetime::Respawn);

/// `groundcover.blades.enabled`: the blade fields ([`crate::blades::BladesPlugin`]'s systems) are planned and spawned.
/// Off by default until the owner's first look (slice GC-3a2b); read when the fields are planned.
pub const BLADES_ENABLED: Def<bool> = def!(bool "groundcover.blades.enabled" = false)
    .help("Grass blades are drawn; off until the owner's first look")
    .source(Source::Guess)
    .group(Group::Foliage)
    .lifetime(Lifetime::Respawn);

// Blade wind (slice GC-3c): how the weather's wind (`sf_wind_bevy::WindField`) maps onto `bevy_grass`'s
// `GrassWindFrame` (`crate::blades::blade_wind_frame`). All live: read every frame.

/// `groundcover.blades.wind_full_mps`: the wind speed at which the blades bend at `bevy_grass`'s strength 1.
pub const BLADES_WIND_FULL: Def<f32> = def!(float "groundcover.blades.wind_full_mps" = 12.0)
    .range(2.0, 40.0)
    .unit(Unit::None)
    .help("Wind speed in m/s that bends the blades at full strength (1); default = bevy_grass's STORM preset gust speed, its strength-1 wind")
    .source(Source::Guess)
    .lifetime(Lifetime::Live);

/// `groundcover.blades.wind_max_strength`: cap on the blade wind strength, so a gale does not fold the blades flat.
pub const BLADES_WIND_MAX: Def<f32> = def!(float "groundcover.blades.wind_max_strength" = 1.5)
    .range(0.0, 4.0)
    .unit(Unit::Ratio)
    .help("Most the blade wind strength may reach (1 = bevy_grass's storm); default guessed at half a storm above it")
    .source(Source::Guess)
    .lifetime(Lifetime::Live);

/// `groundcover.blades.gust_scale_m`: the size of a wind front in the blades (not the weather's larger gust cell).
pub const BLADES_GUST_SCALE: Def<f32> = def!(float "groundcover.blades.gust_scale_m" = 20.0)
    .range(4.0, 200.0)
    .unit(Unit::Meters)
    .help("Size of a gust front rolling across the blades; default between bevy_grass's BREEZE (25 m) and GUSTY (15 m) presets, under the blade range")
    .source(Source::Guess)
    .lifetime(Lifetime::Live);

/// `groundcover.blades.gust_contrast_per_gustiness`: the blades' gust contrast per unit of the weather's gustiness.
pub const BLADES_GUST_CONTRAST: Def<f32> = def!(float "groundcover.blades.gust_contrast_per_gustiness" = 3.3)
    .range(0.0, 10.0)
    .unit(Unit::Ratio)
    .help("Blade gust contrast per unit of weather gustiness (steady air shows no fronts); default makes the weather's default 0.3 give bevy_grass's default contrast of about 1")
    .source(Source::Guess)
    .lifetime(Lifetime::Live);

/// `groundcover.blades.sway`: each blade's own sway, as a multiple of `bevy_grass`'s default.
pub const BLADES_SWAY: Def<f32> = def!(float "groundcover.blades.sway" = 1.0)
    .range(0.0, 4.0)
    .unit(Unit::Ratio)
    .help("How far each blade sways on its own, as a multiple of bevy_grass's default; default = its BREEZE preset")
    .source(Source::Guess)
    .lifetime(Lifetime::Live);

/// `groundcover.blades.sway_speed`: how fast the blades sway, as a multiple of `bevy_grass`'s default.
pub const BLADES_SWAY_SPEED: Def<f32> = def!(float "groundcover.blades.sway_speed" = 1.0)
    .range(0.0, 4.0)
    .unit(Unit::Ratio)
    .help("How fast blades sway, as a multiple of bevy_grass's default; default = its BREEZE preset")
    .source(Source::Guess)
    .lifetime(Lifetime::Live);

/// `groundcover.blades.flutter`: tip flutter of full-detail blades, as a multiple of `bevy_grass`'s default.
pub const BLADES_FLUTTER: Def<f32> = def!(float "groundcover.blades.flutter" = 1.0)
    .range(0.0, 4.0)
    .unit(Unit::Ratio)
    .help("How much the tips of full-detail blades flutter (grows with the wind strength), as a multiple of bevy_grass's default; default = its BREEZE preset")
    .source(Source::Guess)
    .lifetime(Lifetime::Live);

/// `groundcover.blades.near_density`: the near tier (slice GC-3n, vendored patch "near tier"): blades are this many
/// times as dense at the viewer's feet, so the first metres do not look sparse. 1 turns it off.
pub const BLADES_NEAR_DENSITY: Def<f32> = def!(float "groundcover.blades.near_density" = 2.5)
    .range(1.0, 4.0)
    .unit(Unit::Ratio)
    .help("Blade density multiplier at the viewer's feet (1 = off), full to near_start_m, back to 1 at near_end_m; costs blades only there")
    .source(Source::Guess)
    .lifetime(Lifetime::Live);

/// `groundcover.blades.near_start_m`: where the near tier starts to taper back to the normal density.
pub const BLADES_NEAR_START: Def<f32> = def!(float "groundcover.blades.near_start_m" = 4.0)
    .range(0.0, 16.0)
    .unit(Unit::Meters)
    .help("Distance from the viewer where the near tier's extra density starts to taper")
    .source(Source::Guess)
    .lifetime(Lifetime::Live);

/// `groundcover.blades.near_end_m`: where the near tier is back to the normal density (at least `near_start_m`).
pub const BLADES_NEAR_END: Def<f32> = def!(float "groundcover.blades.near_end_m" = 7.0)
    .range(0.0, 24.0)
    .unit(Unit::Meters)
    .help("Distance from the viewer where the near tier ends; the blade count grows with its square")
    .source(Source::Guess)
    .lifetime(Lifetime::Live);

pub const TUNABLES: &[&dyn AnyDef] = &[
    &ENABLED,
    &MESH_RANGE,
    &HYSTERESIS,
    &SHADOW_RANGE,
    &FADE_WIDTH,
    &FADE_SHRINK,
    &MAX_BLOCKS_PER_BATCH,
    &INITIAL_BLOCKS,
    &BLOCK_SIGMA,
    &RESERVE_WORST_CASE,
    &JOBS_PER_FRAME,
    &MAX_JOBS_IN_FLIGHT,
    &GROUND_CACHE_SPACING,
    &WIND_SCALE,
    &BEND_PER_MPS,
    &PHASE_OFFSET,
    &GUST_CONTRAST,
    &SWAY_AMOUNT,
    &SWAY_HZ,
    &FLUTTER_AMOUNT,
    &FLUTTER_HZ,
    &BLADES_BLOCK,
    &BLADES_RING,
    &BLADES_HYSTERESIS,
    &BLADES_CELL,
    &BLADES_ENABLED,
    &BLADES_WIND_FULL,
    &BLADES_WIND_MAX,
    &BLADES_GUST_SCALE,
    &BLADES_GUST_CONTRAST,
    &BLADES_SWAY,
    &BLADES_SWAY_SPEED,
    &BLADES_FLUTTER,
    &BLADES_NEAR_DENSITY,
    &BLADES_NEAR_START,
    &BLADES_NEAR_END,
];
