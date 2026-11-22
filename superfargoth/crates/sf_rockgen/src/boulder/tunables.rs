//! Boulder tunables (`rockgen.boulder.*`, spec `docs/specs/tunables.md`; rockgen.md "Tunables"). Every one is
//! part of the boulder library's bake key (`Lifetime::Bake`). Shape values shape collision too, so they are
//! host-authoritative (`Net::Gameplay`); mesh resolution and LOD values are presentation.

use sf_tunables::{AnyDef, Def, Lifetime, Net, Source, Unit, def};

pub const SIZE: Def<f32> = def!(float "rockgen.boulder.size_m" = 2.0)
    .range(0.2, 40.0)
    .unit(Unit::Meters)
    .help("Longest dimension of a generated boulder (the mesh is scaled so its bounds' longest side is this)")
    .source(Source::Guess)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Bake);

pub const ASPECT: Def<[f32; 2]> = def!(float2 "rockgen.boulder.aspect" = [0.75, 0.55])
    .range([0.05, 0.05], [1.0, 1.0])
    .unit(Unit::Ratio)
    .help("Middle and short axis over the long axis of the joint block (blocky ~[0.8, 0.7], platy ~[0.8, 0.25])")
    .source(Source::Guess)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Bake);

pub const JOINT_SETS: Def<u32> = def!(count "rockgen.boulder.joint_sets" = 3)
    .range(0, 3)
    .unit(Unit::Count)
    .help("Joint families bounding the block by flat faces, short axis first (0 = a weathered cobble, 3 = a block)")
    .source(Source::Industry("geology: 2-3 joint families per outcrop (research terrain-sharp-features §4)"))
    .net(Net::Gameplay)
    .lifetime(Lifetime::Bake);

pub const JOINT_JITTER: Def<f32> = def!(float "rockgen.boulder.joint_jitter_deg" = 12.0)
    .range(0.0, 45.0)
    .unit(Unit::Degrees)
    .help("Largest tilt of a joint family away from orthogonal (real joint sets are near, not exactly, square)")
    .source(Source::Guess)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Bake);

pub const CHIPS: Def<u32> = def!(count "rockgen.boulder.chips" = 6)
    .range(0, 64)
    .unit(Unit::Count)
    .help("Planes that clip corners and edges off the block (spalling, impact)")
    .source(Source::Guess)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Bake);

pub const CHIP_DEPTH: Def<f32> = def!(float "rockgen.boulder.chip_depth" = 0.15)
    .range(0.0, 0.5)
    .unit(Unit::Ratio)
    .help("Deepest chip cut below the block's corner, as a fraction of the rock's size")
    .source(Source::Guess)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Bake);

pub const CHIP_SPREAD: Def<f32> = def!(float "rockgen.boulder.chip_spread" = 0.35)
    .range(0.0, 2.0)
    .unit(Unit::Ratio)
    .help("Random spread of a chip plane's direction around its corner (0 = cut exactly across the corner)")
    .source(Source::Guess)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Bake);

pub const ROUNDING: Def<f32> = def!(float "rockgen.boulder.rounding" = 0.06)
    .range(0.0, 0.5)
    .unit(Unit::Ratio)
    .help("Edge rounding radius (smooth-max blend) as a fraction of the size; without edge snap at least 1.5 cells are used, with it a radius under one cell is a hard edge")
    .source(Source::Guess)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Bake);

pub const WEATHER_AMP: Def<f32> = def!(float "rockgen.boulder.weather_amp" = 0.02)
    .range(0.0, 0.2)
    .unit(Unit::Ratio)
    .help("Weathering noise amplitude as a fraction of the size (keep it below the rounding so the rock stays one piece)")
    .source(Source::Guess)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Bake);

pub const WEATHER_WAVELENGTH: Def<f32> = def!(float "rockgen.boulder.weather_wavelength" = 0.3)
    .range(0.05, 2.0)
    .unit(Unit::Ratio)
    .help("Weathering noise base wavelength as a fraction of the size")
    .source(Source::Guess)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Bake);

pub const WEATHER_OCTAVES: Def<u32> = def!(count "rockgen.boulder.weather_octaves" = 3)
    .range(1, 6)
    .unit(Unit::Count)
    .help("Weathering noise octaves; octaves finer than 4 mesh cells are dropped (they would alias)")
    .source(Source::Guess)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Bake);

pub const RESOLUTION: Def<u32> = def!(count "rockgen.boulder.resolution" = 40)
    .range(8, 128)
    .unit(Unit::Count)
    .help("Meshing cells along the long axis of a rock up to resolution_ref_m long (bigger rocks get more, see resolution_size_exp)")
    .source(Source::Guess)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Bake);

pub const LOD0_ERROR: Def<f32> = def!(float "rockgen.boulder.lod0_error_voxels" = 0.2)
    .range(0.0, 2.0)
    .unit(Unit::Ratio)
    .help("LOD0 simplification error from the raw surface-nets mesh, in mesh cells (flat joint faces collapse)")
    .source(Source::Guess)
    .lifetime(Lifetime::Bake);

pub const LOD_LEVELS: Def<u32> = def!(count "rockgen.boulder.lod_levels" = 4)
    .range(1, 8)
    .unit(Unit::Count)
    .help("Levels in the LOD chain including LOD0 (the chain stops early once simplification no longer reduces)")
    .source(Source::Guess)
    .lifetime(Lifetime::Bake);

pub const LOD_RATIO: Def<f32> = def!(float "rockgen.boulder.lod_ratio" = 0.5)
    .range(0.05, 0.9)
    .unit(Unit::Ratio)
    .help("Triangles each LOD keeps from the previous one")
    .source(Source::Industry("UE static mesh LOD groups: each auto LOD keeps 50 % of the previous triangles"))
    .lifetime(Lifetime::Bake);

// --- RG-4: sharp families, feature-preserving meshing, size-scaled budgets ---

pub const EDGE_SNAP: Def<bool> = def!(bool "rockgen.boulder.edge_snap" = true)
    .help("Snap surface-nets vertices onto the analytic joint / chip / column planes (QEF on the active planes, dual contouring's vertex rule): flat faces and crisp edges; only where the rounding is under one cell")
    .source(Source::Industry("Ju et al. 2002 dual contouring QEF vertex placement; Schaefer 2007 manifold DC"))
    .net(Net::Gameplay)
    .lifetime(Lifetime::Bake);

pub const CREASE: Def<f32> = def!(float "rockgen.boulder.crease_deg" = 35.0)
    .range(0.0, 180.0)
    .unit(Unit::Degrees)
    .help("Faces meeting at more than this angle get split vertex normals (a hard edge); 180 = smooth everywhere (SDF-gradient normals)")
    .source(Source::Industry("auto smoothing by angle: Blender 30 deg, Unity model import 60 deg; rocks: between"))
    .lifetime(Lifetime::Bake);

pub const PIT_DENSITY: Def<f32> = def!(float "rockgen.boulder.pit_density" = 0.0)
    .range(0.0, 1.0)
    .unit(Unit::Ratio)
    .help("Vesicle pits: chance that a pit lattice cell holds one (scoria / vesicular lava; 0 = no pits)")
    .source(Source::Guess)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Bake);

pub const PIT_SIZE: Def<f32> = def!(float "rockgen.boulder.pit_size" = 0.04)
    .range(0.005, 0.2)
    .unit(Unit::Ratio)
    .help("Largest vesicle pit radius as a fraction of the size (pits under 2 mesh cells are dropped)")
    .source(Source::Guess)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Bake);

pub const COLUMNS: Def<u32> = def!(count "rockgen.boulder.columns" = 0)
    .range(0, 40)
    .unit(Unit::Count)
    .help("Columnar jointing: prism columns in the cluster (Voronoi cells of a jittered hex lattice; 0 = a joint-set block)")
    .source(Source::Guess)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Bake);

pub const COLUMN_JITTER: Def<f32> = def!(float "rockgen.boulder.column_jitter" = 0.22)
    .range(0.0, 0.45)
    .unit(Unit::Ratio)
    .help("Column centre jitter off the hex lattice, as a fraction of the column spacing (0 = all hexagons; more = 4-7 sided)")
    .source(Source::Industry("columnar basalt: 5-6 sided columns dominate, mean ~6 sides (Goehring & Morris 2009)"))
    .net(Net::Gameplay)
    .lifetime(Lifetime::Bake);

pub const COLUMN_GROOVE: Def<f32> = def!(float "rockgen.boulder.column_groove" = 0.08)
    .range(0.0, 0.3)
    .unit(Unit::Ratio)
    .help("Width of the open joint between neighbouring columns, as a fraction of the column spacing (under 2.5 mesh cells: fused)")
    .source(Source::Guess)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Bake);

pub const COLUMN_STEP: Def<f32> = def!(float "rockgen.boulder.column_step" = 0.35)
    .range(0.0, 0.8)
    .unit(Unit::Ratio)
    .help("Spread of the column tops (cross-joint breaks) as a fraction of the cluster height")
    .source(Source::Guess)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Bake);

pub const COLUMN_TILT: Def<f32> = def!(float "rockgen.boulder.column_tilt_deg" = 8.0)
    .range(0.0, 30.0)
    .unit(Unit::Degrees)
    .help("Largest tilt of a column's top (its cross joint) from square to the column axis")
    .source(Source::Guess)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Bake);

pub const COLUMN_FUSE: Def<f32> = def!(float "rockgen.boulder.column_fuse" = 0.3)
    .range(0.0, 0.9)
    .unit(Unit::Ratio)
    .help("Where the joints between columns close: this fraction of the way from the cluster's base up to its lowest top")
    .source(Source::Guess)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Bake);

pub const RESOLUTION_REF: Def<f32> = def!(float "rockgen.boulder.resolution_ref_m" = 2.0)
    .range(0.2, 40.0)
    .unit(Unit::Meters)
    .help("Rocks up to this long mesh at `resolution` cells; longer ones get more (resolution_size_exp)")
    .source(Source::Guess)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Bake);

pub const RESOLUTION_SIZE_EXP: Def<f32> = def!(float "rockgen.boulder.resolution_size_exp" = 0.5)
    .range(0.0, 1.0)
    .unit(Unit::Ratio)
    .help("Cells grow with (size / resolution_ref_m)^this (0 = fixed cells per rock, 1 = fixed cell size in metres)")
    .source(Source::Guess)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Bake);

pub const RESOLUTION_MAX: Def<u32> = def!(count "rockgen.boulder.resolution_max" = 128)
    .range(8, 192)
    .unit(Unit::Count)
    .help("Most meshing cells along any rock's long axis (caps generation time of the biggest rocks)")
    .source(Source::Guess)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Bake);

pub const LOD0_ERROR_MAX: Def<f32> = def!(float "rockgen.boulder.lod0_error_max_m" = 0.015)
    .range(0.001, 0.5)
    .unit(Unit::Meters)
    .help("LOD0 simplification error never exceeds this in metres (a big rock seen up close keeps its detail)")
    .source(Source::Guess)
    .lifetime(Lifetime::Bake);

pub const TUNABLES: &[&dyn AnyDef] = &[
    &SIZE,
    &ASPECT,
    &JOINT_SETS,
    &JOINT_JITTER,
    &CHIPS,
    &CHIP_DEPTH,
    &CHIP_SPREAD,
    &ROUNDING,
    &WEATHER_AMP,
    &WEATHER_WAVELENGTH,
    &WEATHER_OCTAVES,
    &RESOLUTION,
    &LOD0_ERROR,
    &LOD_LEVELS,
    &LOD_RATIO,
    &EDGE_SNAP,
    &CREASE,
    &PIT_DENSITY,
    &PIT_SIZE,
    &COLUMNS,
    &COLUMN_JITTER,
    &COLUMN_GROOVE,
    &COLUMN_STEP,
    &COLUMN_TILT,
    &COLUMN_FUSE,
    &RESOLUTION_REF,
    &RESOLUTION_SIZE_EXP,
    &RESOLUTION_MAX,
    &LOD0_ERROR_MAX,
];
