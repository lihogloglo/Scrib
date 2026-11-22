# Rock generation (`sf_rockgen`: boulders, cliff bands, cave mouths, hero formations)

Status: **spec agreed for slice RG-1** (owner approved the system 2026-11-22: procedural rocks, cliffs, caves and rock
formations as meshes on top of the heightfield, without Houdini). Built: RG-1 (Bevy-free boulder generator), RG-3 (viewer + material; ahead of RG-2 at the owner's request, 2026-11-22).
Crates: `sf_rockgen` (Bevy-free core, this spec), `sf_rockgen_bevy` (RG-3: meshes, LODs, material; stream-in
tasks at RG-6). Game side (rock types, texture choices, where caves open): `mw_worldgen` adapter, later. Ledger:
`docs/ledger/worldgen.md` (rockgen lines start `RG-`). ROADMAP: Phase 4 World generation lane, line "Rock generation".

<!-- agent-summary -->
> **Agent summary.** Everything a heightfield cannot show (overhangs, vertical faces, blocky boulders, arches, cave
> mouths) is a mesh made from a **signed distance field (SDF)** and meshed with **surface nets**, all from a seed and
> a declared parameter set, so any peer regenerates the same rock. Four products: (a) **boulders** (a convex block
> from joint-set slabs and random chip planes, smooth-max rounded, noise weathered; a few variant libraries per
> rock type, instanced by the `sf_worldgen` scatter); (b) **cliff bands** (a 3D volume per ~32 m chunk around steep
> heightfield bands: the terrain surface plus strata of varying hardness, joint planes and talus; meshed only within
> ~1.5 km); (c) **cave mouths** (capsule tunnels carved into the band, terrain hole + collar mesh); (d) **hero
> formations** (arches, sea stacks, basalt columns, hoodoos, dikes). LOD by `meshopt`, collision by `parry3d`,
> materials are world-space triplanar with the terrain's hex tiling (no UVs). Boulder libraries generate at bake
> (seconds); cliff chunks generate on stream-in worker threads (deterministic, no disk). Read "Data model", your
> slice in "Build plan", and the handoff in `.sessions/rockgen.md` while a slice runs.
<!-- /agent-summary -->

## Goal

The 10x generated worlds (`worldgen.md`) have steep land shown only as heightfield ramps; the owner's look loop says
"rocks all one size", "blobby relief", and Morrowind's own `terrain_rock_*` cliff meshes are dropped at 10x because
they become giant smooth blobs (ledger 2026-11-19, research `../research/terrain-sharp-features-2026-11-19.md` §4).
This system supplies the mesh layer of that research's step 4: rock that breaks the way rock breaks (along joint sets
and bedding), in sizes that follow a power law, with the vertical and overhanging faces no heightfield can carry.
Not in scope: the heightfield's own sharp passes (`sf_worldgen::cliffs`, the detail function), interiors (a game's
caves behind doors stay its interior cells).

## Constraints (CONTRIBUTING applied)

- **Framework/game split.** `sf_rockgen` knows SDFs, rock classes as numbers, masks and seeds; it never names a
  game, a texture or a region. Rock types ("basalt", "ash tuff"), their parameters, texture picks and cave-mouth
  sites are adapter data (`mw_worldgen`, RG-A1).
- **Net-ready.** Every product is a pure function of (seed, parameters, inputs): counter-hash noise (splitmix64), no
  global RNG, no thread-count dependence, no float reductions in thread order. Collision geometry is derived from
  the same mesh, so its shape parameters are `Net::Gameplay` tunables; LOD and quality values are presentation.
- **No hardcoded tunables.** Every size, ratio, count and resolution is a `sf_tunables` `Def` with unit, range and
  source (`tunables.md`); slice RG-1's are in `sf_rockgen::boulder::tunables` (`rockgen.boulder.*`).
- **Compile time.** The core is Bevy-free (hakari `final-excludes`), depends only on `sf_tunables`, `meshopt`
  (already in the tree) and `fast-surface-nets`; `parry3d` joins only behind a `collision` feature (RG-2).
- **Speed first.** Frame cost is draws + triangles (instancing, LOD, culling); generation runs off the main thread
  and never inside a frame.

## Reuse check (2026-11-22; source read unless marked)

**Bevy 0.20 native (V, `bevy_mesh-0.20.0`, `bevy_pbr-0.20.0`, `bevy_tasks-0.20.1` in the registry).**
- Meshes: `bevy_mesh::Mesh::new` + `with_inserted_attribute` / `insert_indices`, `compute_smooth_normals`,
  `generate_tangents` (mikktspace; not needed: triplanar). The core emits plain arrays; `sf_rockgen_bevy` wraps
  them in a `Mesh` with `RenderAssetUsages::RENDER_WORLD` only (no CPU copy kept after upload).
- Background work: `bevy_tasks::AsyncComputeTaskPool::get().spawn(..)` + `poll_once`, the pattern
  `sf_terrain_bevy/src/far_map.rs:170` already uses. Cliff-chunk generation uses the same.
- Virtual geometry: `bevy_pbr` feature `meshlet` / `meshlet_processor` (`MeshletMesh::from_mesh`, built on
  meshopt + METIS) exists but requires `TEXTURE_INT64_ATOMIC`, `SHADER_INT64`, `SUBGROUP` and more
  (`meshlet/mod.rs:120`), renders through its own material path (no `ExtendMaterial` triplanar shader of ours) and
  is marked experimental. Not used: classic discrete LODs (the far-field path, `statics-and-lod.md`) work on every
  preset. Revisit when meshlets support custom materials on our presets.
- Bevy has no SDF or isosurface module.

**Crates.**

| Crate | Version, licence | Read | Verdict |
|---|---|---|---|
| `fast-surface-nets` | 0.2.1 (2025-01), MIT/Apache | `src/lib.rs` (458 lines): naive surface nets on a padded grid, central-difference normals, chunk seams by skipping max-plane quads, ~20 M tri/s | **Adopted** for boulders and cliff chunks. Bevy-free, deps `glam 0.29` + `ndshape 0.3` (one more glam version in the lock: `deny` warns, a few seconds of cold build; removal path: a ~200-line port onto glam 0.33 with a NOTICES entry, if the duplicate ever matters). |
| `transvoxel` | 2.0.0 (2025-11), MIT/Apache | `Cargo.toml`, crate root: marching cubes with Lengyel transition cells; Bevy only as a dev-dependency | **Candidate for RG-6** if cliff chunks need LOD-to-LOD seams; RG-6 first tries surface nets + `LockBorder` simplification. |
| `saft` | 0.34.1 (2024-02), MIT/Apache (Embark) | `src/lib.rs`, `marching_cubes.rs`: SDF graph compiler + interpreter, marching cubes | Rejected: no release since 2024, `macaw 0.19` (old glam), graph language would have to host our hash noise. Its primitive set is the standard one we write as plain Rust. |
| `fidget` / `fidget-mesh` | 0.5.1 (2026-09), MPL-2.0 (unmodified only) | `fidget-mesh/src/lib.rs`: Manifold Dual Contouring (Schaefer 2007), watertight, keeps sharp edges | Not for RG-1: shapes must be expression trees with interval evaluation (our counter-hash noise doesn't map), heavy deps (`nalgebra`, JIT). **Candidate for hero formations** (RG-8) where sharp crisp edges matter and the field is plain CSG. |
| `isosurface` | 0.0.4 stable / 0.1.0-alpha.0 (2021), Apache-2.0 | crate root: MC, extended MC, dual contouring over octrees | Rejected: unmaintained alpha, old glam. |
| `meshopt` | 0.6.2, MIT/Apache (in `Cargo.lock`) | `src/simplify.rs`: `simplify` (topology-preserving, `LockBorder`, `ErrorAbsolute`, `Prune`), `optimize_vertex_cache`, `optimize_vertex_fetch` | **Adopted** for the LOD chain; same use as `sf_lod/src/cluster.rs:103-150`. |
| `parry3d` | 0.31.1, Apache-2.0 (in `Cargo.lock`) | `shape/shared_shape.rs:500` `convex_decomposition(_with_params)` (V-HACD), `convex_hull3`, `TriMesh` | **Adopted** (RG-2): one convex hull per boulder (it is convex before weathering), trimesh for cliff chunks near the player, V-HACD only for hero formations. |

**Meshing method.** Surface nets (dual, one vertex per cell) over marching cubes: fewer, better-shaped triangles and
no case tables, and over dual contouring: no Hermite data or QEF solve for an SDF that is rounded and weathered
anyway (sharp edges are not wanted below the rounding radius). Surface nets can make a non-manifold edge where a
cell face is a saddle; the boulder field keeps its noise wavelength ≥ 4 voxels and its rounding ≥ 1.5 voxels so this
does not happen, and RG-1's test checks closedness on many seeds.
**RG-4 (sharp families):** still surface nets, plus a dual-contouring vertex rule of our own: every hard face is a
known plane, so each net vertex is snapped onto the planes crossing its cell (QEF, truncated pseudo-inverse), then
Newton-stepped back onto the weathered surface. `fidget-mesh` (Manifold DC) stays rejected: it needs expression trees
with interval evaluation, which our counter-hash noise, pits and Voronoi columns don't map onto, and brings
nalgebra/JIT dependencies. Naive surface nets is non-manifold at alternating-sign cell faces; RG-4 resolves those
faces (`mesher::resolve_ambiguous_faces`) instead of relying on the wavelength/rounding limits alone.

**Whole projects.** No Bevy or Rust project generating geological rock meshes was found (research §4: the
`bevy_generative`, `bevy_procedural_meshes`, `procedural_modelling` crates are generic mesh builders). OpenMW has no
procedural rock. Prior Scrib projects: none.

## Industry comparison (V = source read this session or in the research doc; U = known from talks, unverified)

| Who | What they do | Here |
|---|---|---|
| Far Cry 5 (Carrier, GDC 2018 "Procedural World Generation of Far Cry 5"; HIVE 2018) | Houdini tool set in the world editor generates cliffs, biomes, rivers; results are baked into the world data (V: talk list on SideFX / Game Developer; cliff algorithm U: cliff rock geometry along slope-detected bands, art-kit textures) | Same input (slope bands from the heightfield), no Houdini: SDF band volumes in Rust; generated at stream-in instead of stored (below). |
| Horizon Zero Dawn (van Muijden, GDC 2017) | GPU procedural placement of authored rock/plant assets from density maps × terrain attributes, per tile, deterministic (V, research §4) | Already `sf_worldgen::scatter` (WG-X5); RG adds the assets it places and size classes. |
| No Man's Sky (McKendrick, GDC 2017, U) | Voxel density fields generated per region on worker threads at stream-in, polygonised at runtime (dual contouring family), deterministic from seed | Cliff bands and cave mouths: same runtime-generation model, bounded to steep bands near the player. |
| Astroneer (U), Dreams (Evans, SIGGRAPH 2015, U) | SDF/voxel edits meshed at runtime (MC variant), SDF CSG with smooth blends | Smooth-min / smooth-max CSG for rounding (Quilez polynomial smin). |
| Paris et al. 2019 "Terrain amplification with implicit 3D features" (TOG, hal-02273097; U, abstract level) | Heightfield turned into an implicit field, sparse 3D primitives (arches, overhangs, caves) blended in a construction tree, placed from geological rules | Exactly our cliff-band field (heightfield SDF ⊕ strata ⊕ features), restricted to bands so the heightfield stays the renderer elsewhere. |
| Becher et al. 2019 (feature-based volumetric terrain, U) | Volumetric features (overhangs, caves) authored as curves, voxel field, polygonised | Cave tunnels as capsule graphs (RG-7). |
| Peytavie et al. 2009 "Arches" (EG, U) | Layered material stacks (bedrock, rocks, sand); rocks as implicit blocks; erosion by layer hardness | Per-layer hardness → soft layers recess into overhangs (RG-5). |
| UE5 (Nanite / PCG, U) | Scanned rock kits, Nanite virtual geometry, PCG placement | No Nanite on our presets (meshlets above); discrete LODs + instancing. |
| Geology (research §4, S) | Rock breaks along 2-3 families of near-parallel joints plus bedding; boulder size N(>d) ~ d^-b, b ≈ 1-2.5, truncated | Joint-set slabs define the block; size classes come from a truncated power law (RG-A1 / scatter). |

**Gaps vs the standard, written down:** no scanned-asset quality (procedural rock reads as rock at 2-10 m only with
good triplanar materials; the owner judges, RG-3); no virtual geometry (triangle budgets per LOD instead).

## Data model

All lengths in metres, rock-local frame (+Z up for cliff fields in world frame; boulders are oriented by the
scatter). The core types (Bevy-free):

- `RockMesh { positions: Vec<[f32;3]>, normals: Vec<[f32;3]>, indices: Vec<u32> }`; `RockLods { lods: Vec<RockMesh>,
  bounds: Aabb, hash: u64 }`. Normals are the SDF gradient (central differences), normalised, so every LOD shades
  alike. The hash (FNV-1a over the bit patterns) is the bake-product key and the determinism test.
- `BoulderParams` (one rock class): size (longest axis), aspect (b/a, c/a), joint sets (count, angular jitter),
  chips (count, depth), rounding, weathering (amplitude, wavelength, octaves), mesh resolution, LOD settings.
  Defaults come from the tunables; an adapter provides per-type values as data.
- `Library { class: ClassId, variants: Vec<RockLods> }` per rock class (RG-2): N variants from seeds
  `hash(world_seed, class, i)`; scatter picks `variant = hash(placement) mod N` and scales within the class range.
- `CliffChunk { key: (worldspace, IVec3 at 32 m), lods, collider }` (RG-5/6); `CaveGraph { nodes, edges(radius) }`
  (RG-7); `Formation { kind, seed, transform, params }` (RG-8).

## Design

### (a) Boulders and rocks (RG-1, RG-2)

SDF, inside negative, built in order:

1. **Joint-set block.** Three orthonormal family normals from the seed, each tilted by up to `joint_jitter_deg`
   (real joints are near, not exactly, orthogonal); for a set count < 3 the remaining axes are bounded by the
   ellipsoid-scaled extent instead. Each family gives a slab `|n·p| − h_k` with half-thickness `h_k` = the axis
   extent (size × aspect). `d = max over slabs`: a parallelepiped, which is what joint sets cut (blocky basalt,
   platy flagstone with a small c/a, elongated columns with two small axes).
2. **Chips.** `chips` random half-spaces clip corners and edges (spalling and impact): normal toward a random
   corner (weighted by the extents) plus `chip_spread` of random direction, offset = the block's support in that
   direction minus `chip_depth·√u` (never more than half the support: a deeper cut is another block).
3. **Rounding.** All planes are combined with a polynomial smooth max of radius `rounding × size` (Quilez smin
   mirrored), which rounds edges the way weathering does, in one pass.
4. **Weathering.** `+ amp × size × fbm(p / (wavelength × size))`: gradient-hash value noise, `octaves` octaves,
   lacunarity 2, gain 0.5 (maths constants). Amplitude is kept below the rounding so the surface stays one piece.
5. **Meshing.** A grid of `resolution` cells along the longest axis (cubic cells) over the solid's reach (the
   ellipsoid, or tighter the tilted joint faces' bound), padded by 2 cells, outer layer forced outside, meshed by
   `fast_surface_nets::surface_nets`: closed by construction. Sampling skips 4³ blocks whose centre is farther from
   the surface than the field's Lipschitz bound allows (exact: same net), and noise is only evaluated in a band of
   `amp + (L·√3 + 1)` cells around the base surface.
6. **LOD chain.** LOD0 = `meshopt::simplify` of the raw net to `lod0_error_voxels` × voxel (absolute error):
   flat joint faces collapse, curved edges keep their vertices. LOD k+1 targets `lod_ratio` × LOD k's triangles
   (UE auto-LOD's 50 % per level as default), the chain stops early if simplification no longer reduces. Each LOD is
   vertex-cache and fetch optimised and compacted. `meshopt::simplify` keeps topology but can pinch two sheets into
   one edge (3 of 384 levels in RG-1's first run): a level that is not closed is retried gentler (target moved
   0.25, 0.5, 0.75 of the way back to the source), LOD0 falls back to the raw net.

**Variant libraries and instancing (RG-2).** Per rock class, 8-32 variants (tunable), each a few ms (RG-1
measurement) → a library of 6 classes × 16 variants is well under a second. The scatter places instances; one mesh
per variant LOD, GPU-instanced through the existing static path. Sink depth 20-40 % of the short axis and
orientation along the ground normal blended by the scatter's learned `align` (research §4). Size classes follow a
truncated power law per class (RG-A1 data): 0.3-1 m (below ~1 m prefer the terrain detail function), 1-3 m, 3-10 m,
10-40 m; the big two cast shadows, the small two don't (presets decide).

**Collision (RG-2).** One convex hull (`parry3d` `ConvexPolyhedron` from the lowest LOD's points): the block is
convex before weathering, so a hull is the standard simple collision (UE "auto convex" / simple collision).
`sf_nav_recast` builds the navmesh from colliders, so rocks enter navigation with no extra path.

### (b) Cliff bands (RG-5, RG-6)

Where the heightfield is steep (slope above `rockgen.cliff.slope_deg`, ~45°, or `sf_worldgen::cliffs`'
`cliff_mask.f32` above a threshold), dilated by a margin, the world is divided into **32 m chunks** (tunable) whose
SDF is:

- `d_terrain(p) = (p.z − h(p.xy)) × cos(slope)` (first-order distance to the heightfield, using the CPU twin of
  the runtime height function so the mesh sits on the rendered ground),
- `+ strata`: horizontal (or dipping, per region) layers of thickness 1-8 m with per-layer **hardness** in [0,1];
  soft layers are pushed inward by `(1 − hardness) × recess_m`: hard caprock overhangs soft beds (cuestas, ledges),
- `max(..., joint planes)`: 2 vertical families with spacing 2-10 m carve vertical faces and blocky steps,
- `smin(..., talus)`: a debris apron at the base (angle of repose ~35°) mixing into the scatter's talus fans.

Meshed at 0.5 m cells (tunable) with 1 cell overlap between chunks; seams between chunks are shared samples
(surface nets is chunk-seamless with padding). LODs per chunk with `LockBorder` so neighbours agree. **Range:**
mesh within `rockgen.cliff.mesh_range_m` (~1.5 km); beyond it the heightfield and its slope-driven triplanar rock
(`P_CLIFF`, WG-X7b) carry the look, as Far Cry and RDR2 terrain does at distance (U).

**Seam with the heightfield.** The cliff mesh's base is buried `rockgen.cliff.bury_m` below the heightfield
(standard rock-kit practice) and the material is the same world-space triplanar rock layer the terrain shader uses
on cliffs, so colour is continuous by construction. No per-object terrain-blend material: that design (TB-1..3)
was rejected by the owner on 2026-10-20 and cost 10-13 ms of render thread (`terrain.md` "Object-terrain blending").

### (c) Caves and lava tubes (RG-7)

A cave is a small graph of capsules (node radius, edge radius tapering), carved with `smax(d_rock, −d_tunnel)`,
noise on the radius, a floor flattened by `max(d, z_floor − p.z)`. **Morrowind's caves are interior cells behind
doors**, so the game mostly needs **cave mouths**: the first 10-30 m of tunnel ending at a door/portal anchor. The
mouth cuts a hole in the heightfield (`terrain.md` holes, 1 bit per 0.5 m quad), and a collar mesh (the mouth's
chunk of the cliff field, rock-only) covers the hole's edge. Through-caves and lava tubes in open worlds use the same
graph at full length (generic; a game decides).

### (d) Hero formations (RG-8)

Few, named by the adapter, placed by hand or by rule: **arches** (a cliff-band slab minus a capsule, Peytavie 2009),
**sea stacks** (a cylinder of strata with hardness recess and a wave-notch at sea level), **columnar basalt**
(Voronoi prisms: 2D Voronoi cells of 0.5-2 m, each a hexagonal-ish prism with its own top height and a slight
taper, the standard columnar-jointing look), **hoodoos** (stacked hardness bands on a cone), **dikes** (a vertical
hard slab standing proud of soft surroundings). Generated at bake (a handful, seconds), V-HACD collision. `fidget`'s
Manifold Dual Contouring is the candidate mesher if crisp edges are wanted here.

### Materials

World-space triplanar with the terrain's hex tiling (Mikkelsen 2022), reusing `sample_layer_triplanar` and the
`hex_*` functions of `sf_terrain_bevy/src/shaders/surface.wesl` as a shared WESL module (moved, not copied, in RG-3b; RG-3 ships a named scaffold copy, "As built"),
so rock meshes need **no UV unwrap** and texel density matches the terrain. Per rock class: one albedo + normal/height
+ ORM layer from the mods' texture layers. Inputs are the owner's PBR packs mounted as mods under
`%LOCALAPPDATA%/bevyrrowind/mods`: `mvr-pbr` (MVR PBR v2), `tamrielic-pbr-west-gash`, and `saintj-pbr` ("saintj":
`textures PBR/saintj PBR version`, 334 ground files, loads after MVR; e.g. `tx_ac_rock_01`, `tx_ashland_rock_03`,
`tx_bc_rock_01` with `_nh` / `_spec` / `_diffusespec`), all Wareya conventions (`bevyrrowind/docs/specs/
textures-adapter.md` §7). Which layer per rock type is adapter data; the framework takes layer ids.
Cheap extras from the SDF itself, baked into vertex colour: cavity (SDF ambient-occlusion estimate) and strata
index (for per-layer tint), no extra textures.

### When generation runs

| Product | When | Why |
|---|---|---|
| Boulder libraries | **Bake** (`sf_bake` product, key = params hash + seed), one-time, seconds | Small, fixed count, reused by thousands of instances; collision hulls precomputed with them. |
| Cliff-band chunks, cave mouths | **Stream-in**, deterministic, on `AsyncComputeTaskPool` workers; in-memory LRU, nothing on disk | Storage and bake time scale with world area (10x Vvardenfell: ~50-75 km² of steep land ≈ 60 k chunks ≈ 4-5 min on 8 threads and GBs of meshes, past the few-minutes bake rule); runtime cost is bounded by the ring (~600 chunks resident within 1.5 km, a few new per second at sprint), each ~30-60 ms on a worker. Same model as the terrain's runtime detail fill and No Man's Sky. "Nothing bakes during play" holds: no step blocks play, generation is streaming work with a per-frame upload budget. |
| Hero formations | **Bake** | A handful; V-HACD collision is slow (100s of ms). |

Hitch control: generation never runs on the main thread; mesh uploads and collider inserts per frame are capped by
`rockgen.stream.uploads_per_frame` (tunable); chunks request before they enter mesh range (prefetch ring), so a
late chunk shows the heightfield (which is always there) instead of a hole.

### Interfaces

- **Scatter** (`sf_worldgen::scatter`, `Species { size_m, group }`): a rock species gains `class: ClassId` and a size
  law (b exponent, d_min, d_max); a placement's `(seed, scale)` selects `(variant, scale)` from the class library.
  Talus fans (density below a steep face along the fall line) are a scatter rule fed by the cliff mask.
- **Cliff mask** (`sf_worldgen::cliffs`, `cliff_mask.f32`): read by the band finder alongside slope; the same mask
  drives `P_CLIFF` in the terrain shader, so mesh bands and shaded rock agree.
- **Heights**: the CPU twin of the runtime height function (`sf_worldgen::tiles`), never a different height source.
- **Physics / nav**: `parry3d` shapes handed to `sf_physics` statics; navmesh through colliders.

## Tunables (slice RG-1, `rockgen.boulder.*`, all `Lifetime::Bake`, `Scope::Dev`)

`size_m` 2 (0.2-40 m), `aspect` [0.75, 0.55] (ratio), `joint_sets` 3 (0-3), `joint_jitter_deg` 12 (0-45),
`chips` 6 (0-24), `chip_depth` 0.15 (ratio of size), `chip_spread` 0.35 (ratio), `rounding` 0.06 (ratio), `weather_amp` 0.02 (ratio),
`weather_wavelength` 0.3 (ratio), `weather_octaves` 3 (1-6), `resolution` 40 (8-128 cells), `lod0_error_voxels`
0.2 (ratio of a cell), `lod_levels` 4 (1-8), `lod_ratio` 0.5 (UE auto-LOD default). Shape values are
`Net::Gameplay` (they shape collision), mesh and LOD values `Net::Presentation`. Source `Guess` except where noted.
RG-4 adds 14: `edge_snap` (bool; owner 2026-11-22 prefers off, default flip queued), `crease_deg` 35, `pit_density`,
`pit_size`, `columns`, `column_jitter`, `column_groove`, `column_step`, `column_tilt_deg`, `column_fuse`,
`resolution_ref_m` 2, `resolution_size_exp` 0.5, `resolution_max` 128, `lod0_error_max_m` 0.015; `chips` range 0-64.

## Build plan

One deliverable + test per slice. IDs `RG-*`; visual checks in `bevyrrowind/docs/visual-checks/worldgen.md`.
**Order (owner 2026-11-22):** finish the normal rocks and their placement first (RG-4 fixes, RG-2 library + collision,
rock placement: scree fans below faces instead of rocks in lines along them); cliff bands (RG-5, RG-6) come later.

- **RG-1 (built 2026-11-22): boulder generator.** `sf_rockgen::boulder`: SDF (joint-set block, chips, smooth max,
  weathering) → surface nets → meshopt LOD chain → `RockLods`. Tests: same seed same hash, closed meshes, no NaN,
  bounds within the declared size, LOD triangle counts strictly decrease; `#[ignore = "measurement: ..."]` ms per
  rock and triangles per LOD.
- **RG-2: variant library + convex hulls.** `Library` per class from seeds, `collision` feature (`parry3d` hull),
  bake product registration. Test: library deterministic; hull contains every LOD0 vertex within tolerance.
- **RG-3 (built 2026-11-22, before RG-2 at the owner's request): viewer + material.** `sf_rockgen_bevy` mesh
  conversion, `VisibilityRange` LODs, triplanar `RockMaterial` (scaffold copy of the terrain functions), the game's
  `level:rock-lab` greybox. Owner look RG-3: one family, too smooth (river boulders) -> sharp families (RG-4).
- **RG-4 (built 2026-11-22): sharp families.** Edge snap onto hard planes, crease normals, size-scaled resolution,
  five new classes (talus, jagged, columnar, scoria, slab). Tests: snapped cube fixture exact (12 triangles), every
  raw net edge-manifold, closed LODs per class. Owner look RG-4: "amazing"; edge snap off preferred. Left: columnar
  budget, 1-9 % wrong-way LOD0 triangles on talus/jagged/slab, `edge_snap` default off, census opt-out in `mesher.rs`.
- **RG-3b: shared triplanar module.** After the terrain LOD fix (`wg-lodfix`) lands: move the projection +
  hex-tiling functions out of `surface.wesl` into one WESL module parameterised by texture + sampler, import it in
  the terrain and `rock.wesl`, delete the scaffold copy. Test: both shaders link and validate; terrain pixels unchanged
  is the owner's call (WG-X7a toggles).
- **RG-5: cliff-band field + chunk mesher** (Bevy-free): band finder from slope + cliff mask, strata/hardness, joints,
  talus; chunk SDF from the height twin. Tests: chunk seams match (shared border vertices equal), base buried below
  the heightfield, deterministic.
- **RG-6: cliff streaming** in `sf_rockgen_bevy`: worker tasks, LRU, upload budget, trimesh colliders near the
  player. Perf A/B (`perf suite --arm`, untraced, hitches per minute + p99) before it defaults on.
- **RG-7: cave mouths**: capsule graph carve + terrain hole + collar. **RG-8: hero formations.**
- **RG-A1 (adapter, `mw_worldgen`):** rock types (basalt, ash tuff, red mountain scoria...), size laws, texture
  layers from the mounted packs, scatter species mapping, cave-mouth sites from the game's cave doors.

## Open questions

None for the owner on method (decided from the industry standard above). The look of each class is the owner's
call at RG-3.

## As built

**RG-1 (2026-11-22).** `superfargoth/crates/sf_rockgen` (`lib.rs` mesh types + hash, `noise.rs` counter-hash value
noise, `boulder.rs` SDF + generator, `boulder/tunables.rs`, `tests.rs`). Tunables are declared (`TUNABLES`) but not
yet registered in the game's catalog (RG-3, with the viewer). Measured 2026-11-22 (`test -p sf_rockgen --release
measure_boulders -- --ignored --nocapture --test-threads=1`, owner laptop, 24 seeds, resolution 40), ms per rock
and mean triangles LOD0/1/2/3: default block 18.7 ms, 429/214/107/53; platy 7.3 ms, 352/175/87/43; cobble 8.4 ms,
445/222/110/54; chipped (24 chips, 12 m) 24.8 ms, 415/207/104/51. Phases (default): field sampling ~9 ms (≈ 80 ns per
evaluated sample, noise in the band dominates), surface nets ~1 ms (12.5 k raw triangles), LOD chain ~4 ms, normals
+ compaction the rest. A 6-class × 16-variant library ≈ 2 s on one thread. LOD0 is low (~430 triangles for any size:
error is in cells) by design for triplanar-shaded rock; the owner judges at RG-3 (raise `resolution` or lower
`lod0_error_voxels` if edges read faceted).

**RG-4 (2026-11-22).** `sf_rockgen`: `sdf.rs` (`Solid`: hard planes per piece, `snap`, `surface`, pits), `classes.rs`
(`CLASSES`: 9 presets with family, analogue, source, tint; the 4 RG-3 classes unchanged), `columns.rs` (Voronoi
prisms, shared faces as `Piece::hard` planes), `mesher.rs` (ambiguous-face resolve, `separate_coincident` at 2^-17,
`resplit_quads` along the crease). Sampling keeps rounding >= 2 cells for topology, the snap uses the hard field.
LOD chain on snapped positions, falling back to the smooth net if no closed LOD0 (level valid only above 0.5x the
net's volume). `RockMesh::split_creases` (angle-weighted smoothing groups) on every LOD. Resolution scales as
res x max(1, size/2 m)^0.5, capped 128. Rock lab: 9 columns 22 m apart, keys N crease, Q edge snap, cameras
`eroded`, `sharp`, `sharp_near`. Measured (release, 24 seeds; ms/rock, LOD0 tris, wrong-way LOD0 tris / total):
block 2 m 25.9 ms 428 (0); talus 2 m 29.8 ms 117 (9 %); jagged 2 m 59.6 ms 92, 12 m 542 ms 705 (7 %); scoria 2 m
64.6 ms 1687 (0); slab 2 m 15.6 ms 109 (1 %); columnar 2 m 364 ms 24 141 tris = broken budget (its snapped LOD0 never
closes, falls back to the smooth chain; 3.1 s at 12 m).

**RG-3 (2026-11-22).** `superfargoth/crates/sf_rockgen_bevy` (`lib.rs` plugin, mesh conversion, LOD ranges, viewer
classes, tunable readers; `material.rs`; `shaders/rock.wesl`; `tunables.rs`; `tests.rs`, `shader_tests.rs`).
- **Mesh:** `rock_mesh` maps `sf_rockgen`'s frame (+Z short axis) to Bevy's +Y up, `(x, y, z) → (x, z, -y)` (a
  rotation: winding kept), positions + normals + `u32` indices, no UVs or tangents.
- **LOD:** one entity per LOD under the rock's root, each with Bevy's `VisibilityRange` (Bevy 0.20
  `bevy_camera::visibility::range`: per-entity distance ranges with dithered crossfade, the far field's mechanism):
  LOD k from `size × lod1_distance × growth^(k-1)` (8 sizes, ×2), crossfade ±`fade` (10 %), last LOD open-ended.
  Tunables `rockgen.lod.*` (Respawn).
- **Material:** `RockMaterial = ExtendedMaterial<StandardMaterial, RockExt>`, forward only, non-bindless (a field
  uses texture sets × LODs materials). Fragment: Bevy's `pbr_input_from_standard_material`, then base colour, normal
  (whiteout-blended, XY rebuilt so RGB and BC5 maps both work), roughness/metallic (Bevy ORM channels G/B) and
  occlusion (R, only for a packed ORM) from a world-axis triplanar of the texture set with Mikkelsen hex-tiling per
  projection; prepass and shadows are Bevy's (geometric normal). Projection origin is the rock's translation plus a
  `MeshTag` offset, so the texture is fixed on the rock under the floating origin; the adapter passes the terrain's
  land-texture tiling (texel density matches the ground). Live tunables `rockgen.material.*` (blend exponent 6, hex
  on, rotation 1, contrast 7, scale 1: the terrain's values). **Scaffold:** `surface.wesl`'s triplanar/hex functions
  read the terrain's arrays and parameter slots and could not be imported unedited while `wg-lodfix` edits that file,
  so `rock.wesl` carries a named minimal copy ("rock triplanar scaffold", non-periodic hex grid); RG-3b removes it.
- **Textures** come from the adapter as `RockTextures` (the game builds them with the statics' path,
  `cooked_material_with` over the bake's companions: a texture no static references may have albedo only; RG-A1 routes
  rock layers through the terrain's companion conversion).
- **Tunables live:** `RockgenPlugin` registers `sf_rockgen`'s `rockgen.boulder.*` and its own lists in the store; the
  game's catalog (`mw_devtools_bevy::perf::tunables::CATALOG`) lists both. Viewer classes (`CLASSES`: block, platy,
  cobble, chipped) are the RG-1 test overrides, adapter data at RG-A1.
- **Viewer:** `bevyrrowind` `level:rock-lab` (scene `level-rock-lab`, `test-levels.md` "Rock lab"): 56 rocks generated
  on the async compute pool; keys X texture, Z class, R new seed, L LOD tint, M wireframe, I info.
- Tests: `rock_mesh_conversion_is_sane` (counts, indices unchanged, unit normals, no NaN, rotation, Y short),
  `lod_ranges_chain`, `tunable_defaults_are_boulder_defaults`, `rock_fragment_compiles_for_every_def_set` (wesl link +
  naga validate, 4 def sets incl. the dither and the game's view features), `uniform_and_flags_match_rust`.
