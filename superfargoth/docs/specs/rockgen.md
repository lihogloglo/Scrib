# Rock generation (`sf_rockgen`: boulders, cliff bands, cave mouths, hero formations)

Status: **spec agreed for slice RG-1** (owner approved the system 2026-11-22: procedural rocks, cliffs, caves and rock
formations as meshes on top of the heightfield, without Houdini). Built: RG-1 (Bevy-free boulder generator), RG-3 (viewer + material; ahead of RG-2 at the owner's request, 2026-11-22). Specced, not built: RG-9 rock typology per geology unit and weathering regime (2026-11-27, "(e) Typology").
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
  Talus fans (density below a steep face along the fall line) are a scatter rule fed by the cliff mask
  (specced: `worldgen-scatter.md` Rocks, WG-E3b).
- **Cliff mask** (`sf_worldgen::cliffs`, `cliff_mask.f32`): read by the band finder alongside slope; the same mask
  drives `P_CLIFF` in the terrain shader, so mesh bands and shaded rock agree.
- **Heights**: the CPU twin of the runtime height function (`sf_worldgen::tiles`), never a different height source.
- **Physics / nav**: `parry3d` shapes handed to `sf_physics` statics; navmesh through colliders.

### (e) Typology: classes per geology unit and weathering regime (RG-9, specced 2026-11-27)

Owner ask 2026-11-26: "add more typology of rocks if needed, depending on the biome / geology; our mesh vocab is
very limited". Research: `docs/research/rock-typology-2026-11-27.md` (cited below as T§n). **Lithology picks the
shape, climate modifies it and covers it** (T§1): the geology unit selects classes (fracture pattern), the weathering
regime selects variants (frost shatter, rounding, cavities) and a surface cover (moss, lichen, varnish, dust), which is
a material layer, not geometry. Of 31 natural forms, 18 are data over today's parameters, 5 need a parameter, 3 a
recipe, 2 are material and 3 are not rock meshes (landforms, caves, scatter density: T§5).

**Reuse check.** Bevy 0.20: no rock vocabulary; `StandardMaterial` has no layer blend, so the cover is a branch of
our own `rock.wesl` (already a custom triplanar material, `sf_rockgen_bevy/src/material.rs`). Crates: none ships rock
presets or a geology-to-asset map (T§4 found none; RG-1 reuse check above). Our own: `classes.rs` presets are Rust
consts with fn overrides (V, `classes.rs:23-42`), so an adapter cannot add one as data; RG-9a moves them to data
(the `scatter_species.rs` TOML pattern). Pits already subtract spheres from the solid (`sdf.rs:112-124`, smooth max),
uniform over the surface (`sdf.rs:230-285`); `Piece` = planes plus an optional ellipsoid (`sdf.rs:73-77`), so blob
unions need no new SDF primitive.

**Industry comparison.** Per biome a rock kit in size tiers, a shared base material with biome variants, and
placement rules per ecotope picking from the kit: Horizon Zero Dawn ecotopes (U), Ghost of Tsushima base rock
material + biome variants (U), Megascans packs sold per biome (U), Morrowind one `terrain_rock_<region>` set per
region (V: `classify.rs:118-131` keys on the `terrain_rock` prefix; the model names carry the region,
`terrain_rock_ma/ac/ai/rm/gl/bc/wg`, read from the source's master file 2026-11-27) (T§6). Cover by world up-normal is UE's World Aligned Blend material function (U,
general knowledge). Here: kit = classes + pack models per `[rock_units]` context (`worldgen-scatter.md` Rocks), tiers
= the scatter's size law, ecotope = unit x regime, biome variant = one material variant per (rock layer, regime).
Gap: no scanned rock quality (as above).

**Regimes** (Peltier 1950, U; T§3): a regime is a name the adapter maps climate classes onto; the framework ships the
default for its own `climate_classes.toml`: **frost** (tundra, taiga), **arid** (desert, cold desert), **semi-arid**
(shrubland, grassland, savanna), **temperate** (woodland, temperate forest, seasonal forest), **wet** (temperate
rainforest, rainforest). Regime effects, the adapter's choice of entries and covers:

| Regime | Shape (class swap) | Placement (`[rock_units]` data) | Cover (material) |
|---|---|---|---|
| frost | `talus` -> `frost_block`; little rounding | blockfield: `field_stone_per_ha` up on gentle slopes | crustose lichen |
| arid | `ventifact` among field stones; tafoni on tuff | pavement: many small field stones | varnish on tops |
| semi-arid | spheroidal `cobble`, `tor` formations | scattered boulders | sparse lichen |
| temperate | rounded variants (`block`, `cobble`) | as the unit | moss on tops and in shade, lichen in the open |
| wet | rounded corestones, few fresh blocks | `field_stone_per_ha` down (rock buried) | heavy moss, darker |

**Cover assets** (checked 2026-11-27 in Bevyrrowind's three mounted packs and the source game's master file): moss has
a tiling layer (`tx_bc_moss*`, a landscape texture record there); the lichen files are inventory icons, not tiling
layers, and no pack has varnish or dust. So a cover is a mask (RG-9i) with a colour and an **optional** texture layer:
moss textured; lichen, varnish and dust tint-only until a pack brings layers. No row waits on an asset.

**Classes per unit** (generic class names; unit names are the adapter's, here Bevyrrowind's six units of
`bevyrrowind/worldgen/geology.toml`, T§2; `outcrop = false` units have no outcrop context, V):

| Unit | talus | outcrop | field | Hero (RG-8) |
|---|---|---|---|---|
| basalt_flow | `talus`, `jagged` | `columnar` parent, `block` | `jagged` (clinker), `crust_plate` | `volcanic_neck` |
| tuff | `soft_block`, `block` | `soft_block` (+ tafoni, RG-9c) | `soft_block`, `cobble` | `hoodoo` |
| tephra | `scoria` | (none) | `scoria`, `bomb`, `spatter` (RG-9f) | (none) |
| capped_sediment | `slab`, `platy` | `slab` parent, `bedded` (RG-9d) | `platy` | `hoodoo`, `arch` |
| coastal_basalt | `column_segment`, `talus` | `columnar`, `entablature` (RG-9e) | `beach_column`, `cobble` | `sea_stack`, `arch` |
| alluvium | `river_cobble` | (none) | `river_cobble`, `cobble` | (none) |

**New classes as data (RG-9b)**, starting numbers `Guess` over today's presets (the owner's look loop tunes them):
`frost_block` (= `talus`, chips 14, `joint_jitter_deg` 25), `bomb` (= `cobble`, aspect [0.45, 0.4], light pits),
`column_segment` (= `columnar`, `columns` 2: the hex lattice gives 1-3 in the footprint, long aspect), `beach_column` (= `column_segment`, rounding and
`weather_amp` up), `soft_block` (= `block`, rounding 0.12, 2 chips, light pits), `ventifact` (= `block`, 3 deep chips,
low weathering; approximate: chips are not biased to one side), `river_cobble` (= `cobble`, aspect [0.75, 0.4];
imbrication is a pose rule, later), `crust_plate` (= `slab`, `weather_amp` up, tint 0.8). Parameter classes once their
row lands: `tafoni` (RG-9c), `bedded` (RG-9d), `entablature` (RG-9e), `spatter` (RG-9f); formation `tor` (RG-9g).

**Not here:** tumuli, pressure ridges, cones (landforms, `landforms.md`); lava tubes (RG-7); blockfields, pavements
(scatter density per regime, RG-9j); vent-distance fining, river imbrication (later scatter fields); breadcrust crack
grooves, exfoliation shells (Later: low value per T§5); ropy pahoehoe (a texture, pack data).

### (f) Placement in the game (RG-P, specced 2026-11-27)

Reviewed 2026-11-27 (adversarial, every cited path read): fixed the physics scaled-copy growth (point 8), the shared
solid list with the navmesh (8), string interning (4), the index's `ModelInfo` type (5), the end-band rule and the P3
range test (6), and named the unused `Library::pick` (3).

The scatter's **library rocks** (`mw_worldgen` `flora_rocks.rs:377` `RockPlacement`: class name, variant hash,
transform whose uniform scale is the size mark in m, `sink` share of the short axis, regime) reach the generated world
as ordinary streamed placements of one content-pack class, drawn by a rock spawner. Research:
`docs/research/rock-placement-game-2026-11-27.md` (option B). Scale (WG-E3m, x10s): ~1.66 M library rocks, ~3-8 k in a
1 km ring, each up to `lod_levels` LOD entities.

**Reuse check (V = source read 2026-11-27).** Bevy 0.20: entities sharing one `Mesh` and one material handle batch
into one instanced draw with GPU preprocessing and multi-draw indirect; `MeshTag` rides per instance and does not split
the batch (it is a field of the per-instance `MeshUniform`, `vendor/bevy_pbr/src/render/mesh.rs:605-606`, filled at
`:1261-1275`, V); LOD is one entity per level with `VisibilityRange`
(dithered crossfade, V); there is no instance-list component outside our vendored `gpu_instance_batch` (V), so shared
handles are the native path. Framework: `PlacementClass::Custom(u16)` is "Content-pack defined"
(`sf_world/src/placement.rs:77-78`, V) and `PlacementSpawners` maps a class to a `PlacementSpawner` with
`needs_model` (`sf_world_bevy/src/spawn.rs:105-123`, V): the streamed index, pages, rings, R0 activation and the physics
hook (`sf_physics_bevy/src/stream.rs:500-517`, V) all come for free. Cooking variants into `mesh.cook` (research
option A) would lose the approved UV-less triplanar `RockMaterial` (`ModelTemplate` parts are `StandardMaterial`,
`spawn.rs:38-54`, V); a second rock streamer (option C) would duplicate `sf_world_bevy`. Crates: none needed beyond
Bevy's batching (not surveyed further, U).

**Industry comparison (U unless marked).** UE PCG's Static Mesh Spawner emits (mesh, transform) instances per
partition cell into ISM/HISM, at edit time or around generation sources, per-instance LOD (Epic docs, U); Far Cry 5
generates placements offline in Houdini and streams them as instanced entities (Carrier GDC 2018, U); Horizon Zero
Dawn places on the GPU at runtime from maps (van Muijden GDC 2017, U); OpenMW: rocks are `STAT` references, one each
(V, our pack-model rocks). Ours = the FC5 / UE edit-time pattern: offline product (`rocks.bin`), instances in the
world's own streamed partition, one shared mesh and one shared collision shape per variant. Gap: no HLOD / far-field
proxy for rocks (the generated world has the far field off, `mw_exterior_bevy/src/worldgen.rs:10-13`); rocks end at
their ring with the dithered end band.

**Design.**

1. **Libraries, loaded once.** `sf_rockgen::placed::Libraries` (Bevy-free): class name -> decoded `Library`
   (`bake::decode` of each `rockgen.library` product the boot bake made, `mw_boot_bevy/src/bake.rs:287-297`), read
   once by the app and shared as `Arc<Libraries>` by the content builder, the spawner and the physics shape source.
2. **Id scheme.** A library variant's model id is the `VfsPath` `rockgen/<class>/<index>` (`model_id` /
   `parse_model_id`). It is never mounted: the spawner's `needs_model` is false (no `mesh.cook` load) and the shape
   source matches the `rockgen/` prefix before the bake lookup. Class names are lowercase ASCII without separators
   (`sf_rockgen/src/classes.rs` names), so `VfsPath::new` (lowercases, `/` joins, `sf_vfs/src/path.rs`) keeps them.
3. **Resolution at content build, not at spawn.** `Libraries::resolve(class, variant_hash, size_mark_m, sink)` ->
   `Resolved { index, scale, lift_m }`: `index = variant_hash mod variants.len()` (the scatter's documented contract,
   `sf_worldgen/src/scatter_rocks.rs:506`), `scale = Library::scale_for(size_mark_m)` (mark / reference size, both the
   longest LOD0 side), `lift_m = (0.5 - sink) x bounds.extent().z x scale` (sf_rockgen's +Z is the short axis and the
   mesh is centred, `boulder.rs:6-7, 305`; the rock lab seats rocks the same way, `level/rock_lab.rs:381-385`). The
   placement's position moves by `lift_m` along its rotated up (+Y), its scale becomes `splat(scale)`. Unknown class
   or an empty library: the rock is dropped and counted in the summary line. `Library::pick(seed)` (`library.rs:125`,
   a mix64 + multiply-shift over a placement seed) is not used: the scatter's hash is already uniform 64-bit (modulo
   bias <= 32 / 2^64) and modulo is its documented contract; one rule, stated on the scatter side.
4. **The placement.** Class `ROCK_LIBRARY = PlacementClass::Custom(3)` (adapter constant beside `CONTAINER` 1 and
   `ACTIVATOR` 2, `mw_world/src/source.rs:25-27`), `base = RecordRef::new("rockgen", <class>)`, `model = Some(id)`,
   flags default, id = `library_rocks`' (`ROCK_ID_NAMESPACE`). They join `WorldgenContent.extra` as `"rocks library"`
   (`worldgen.rs:101-108`). Regime is dropped until the cover (RG-9i) needs it (then it enters the id or the material
   key, decided there). Memory: ~130 B per placement, ~215 MB on x10s beside the 20.8 M flora placements (+8 %). That
   inline size holds only if the strings are shared: `RecordRef::new` and `VfsPath::new` each allocate an `Arc<str>`
   (`sf_world/src/records.rs:101-107`, `sf_vfs/src/path.rs:11`), so the builder makes one `RecordRef` per class and one
   `VfsPath` per (class, index) and clones them (a refcount), never one per rock.
5. **Bounds for the index.** `WorldgenContent` gains `models: HashMap<VfsPath, sf_world::ModelInfo>` (`min`, `max`,
   `bytes`, `lod_distance`: `sf_world/src/index.rs:27-34`, not `sf_model::ModelInfo`): per used id the variant's
   LOD0 bounds in the engine frame (`[x, y, z] -> [x, z, -y]`, as `rock_mesh`, `sf_rockgen_bevy/src/lib.rs:140-141`),
   `bytes 0` (meshes are shared and resident for the session; the per-placement overhead covers the entity),
   `lod_distance 0` (no limit known: the ring end and size culling decide). `worldgen.rs` `add` (`:204-215`) looks up
   these before the bake's `model_infos`, so library rocks index with real extents, not as points.
6. **Spawner** (`sf_rockgen_bevy::RockSpawner`, framework): holds per (class, index) the LOD mesh handles (built once
   from the libraries with `rock_mesh`), per class one `RockMaterial` handle per LOD (plus a default set), and
   `RockLodSettings`. `spawn`: parse the id; a root under `cx.parent` with `cx.local`; children `lod_bundles` with
   ranges `lod_ranges(reference size x scale, levels, settings)` (`sf_rockgen_bevy/src/lib.rs:189`; its last level is
   open-ended) whose last level is ended by `chain_ranges`' rule (`spawn.rs:351-365`): with `cx.end_band > 0` its end
   margin is `[max_distance, max_distance + end_band]`, else it ends at `cx.max_distance` (already size-culled:
   `SpawnCx::end`, `spawn.rs:76-86`). Every range goes through `shared_range` (`spawn.rs:407-427`: Bevy's 65535 range
   slots; per-instance ranges must be snapped); `NoCpuCulling` on the parts when `StaticGpuCulling::on` (the rule the
   `StaticSpawner` applies to `Static` only, `spawn.rs:284-313`; library rocks never move, so they take it too);
   `MeshTag` = the id's low 32 bits (texture offset). Returns the part count. The parts carry no `lod_live::LodPart`, so
   a live `LodScreen` change does not re-range them until their cell re-streams: accepted, rock switches follow their
   own `rockgen.lod.*` law, not the screen error.
   Same shadow behaviour as the statics (casters by default). The LOD switch law stays `size x lod1_distance x
   growth^k` (owner-approved in the rock lab; distance proportional to size = a constant projected size, the screen-size
   rule UE uses).
7. **Materials (adapter).** `bevyrrowind/worldgen/rock_textures.toml`: `default` texture and per-class rows (initial
   default `textures/tx_rock_wg_01.dds`, the rock lab's first, `level/rock_lab.rs:41-45`), built per class and LOD with
   the rock lab's path (`cooked_material_with` + `RockTextures::from_standard`, tiling = the terrain's texel repeat),
   factored out of `rock_lab.rs` so both use one builder. Per geology unit textures (basalt vs tuff) need the unit on
   the placement: later, with RG-9k.
8. **Collision.** `ROCK_LIBRARY` joins `mw_world::source::MOVER_SOLID` (`source.rs:31-32`), the one list the physics
   hook (`hook_physics(.., MOVER_SOLID)`, `mw_walkers_bevy/src/physics.rs:688-693`) and the navmesh filter
   (`mw_bake::nav::is_solid`, `nav.rs:141-144`) share, never a second list. The game's `ShapeLoader` (the
   `PhysicsShapeSource` closure, `physics.rs:102-107`; type `sf_physics_bevy/src/stream.rs:53`) answers `rockgen/` ids
   with the variant `Hull` as a `CollisionMesh` (points axis-converted as the mesh; the conversion is a proper rotation,
   so the outward winding holds; `SurfaceId::DEFAULT` until the adapter names a stone surface) -> `StaticShape` (parry
   `TriMesh`). The generated world has no nav bake yet; when it gets one, its geometry source must answer `rockgen/`
   ids the same way, or `is_solid` rocks add no geometry (the rule for an empty `collision.cook`). **Scaled copies.**
   The hook scales a mesh by a cached copy per exact scale (`stream.rs:650-660`; `MeshShape.scaled`,
   `sf_physics/src/shape.rs:30, 80-95`: a `Vec` searched linearly, keyed by the scale's bits, never evicted). Pack
   statics reuse a few scales; library rocks have a continuous scale, so every rock ever activated would leave one copy
   for the session. RG-P5b therefore prunes the cache: a copy no body holds (its `SharedShape` strong count back to the
   cache's own) is dropped when its body is removed, so the cache is bounded by the live R0 bodies. Industry: PhysX
   `PxMeshScale` and Jolt `ScaledShape` apply a per-instance scale to one shared mesh without a copy (U); parry has no
   scaled-mesh wrapper, so a copy per live scale is our equivalent. A convex-polyhedron shape kind would be cheaper for
   contacts but is a new `ShapeRef` kind for every static: not in RG-P.
9. **Switch.** `--off rocklibrary` (one word, as `--off farfield`) leaves the list out (the A/B arm for the WG-E3 check and `perf ab`).

## Tunables (slice RG-1, `rockgen.boulder.*`, all `Lifetime::Bake`, `Scope::Dev`)

`size_m` 2 (0.2-40 m), `aspect` [0.75, 0.55] (ratio), `joint_sets` 3 (0-3), `joint_jitter_deg` 12 (0-45),
`chips` 6 (0-24), `chip_depth` 0.15 (ratio of size), `chip_spread` 0.35 (ratio), `rounding` 0.06 (ratio), `weather_amp` 0.02 (ratio),
`weather_wavelength` 0.3 (ratio), `weather_octaves` 3 (1-6), `resolution` 40 (8-128 cells), `lod0_error_voxels`
0.2 (ratio of a cell), `lod_levels` 4 (1-8), `lod_ratio` 0.5 (UE auto-LOD default). Shape values are
`Net::Gameplay` (they shape collision), mesh and LOD values `Net::Presentation`. Source `Guess` except where noted.
RG-4 adds 14: `edge_snap` (bool, default off since 2026-11-22: the owner prefers the rounded look; on = the lab's Q arm), `crease_deg` 35, `pit_density`,
`pit_size`, `columns`, `column_jitter`, `column_groove`, `column_step`, `column_tilt_deg`, `column_fuse`,
`resolution_ref_m` 2, `resolution_size_exp` 0.5, `resolution_max` 128, `lod0_error_max_m` 0.015; `chips` range 0-64.
RG-9 adds (all `Guess`, `Net::Gameplay` except the cover): `pit_side_bias` 0 (0-1), `pit_top_cos` 0.5 (0-1, cosine
of the normal to +Z above which a face counts as a top), `pit_size_spread` 0.6 (0.1-1, ratio: smallest pit radius
over the largest, today's const `PIT_MIN_FRACTION`, `sdf.rs:30`), `beds` 0 (0-12), `bed_recess` 0.04 (0-0.15, ratio
of size), `column_fans` 1 (1-6), `column_fan_tilt_deg` 0 (0-40), `blobs` 0 (0 = joint block, 3-8), `blob_flatten` 0.6
(0.2-1, ratio); material (`rockgen.cover.*`, `Net::Presentation`): `amount` 0 (0-1), `up_sharpness` 4 (1-16),
`cavity_weight` 0.5 (-1-1: negative = cover in cavities, moss; positive = on exposed tops, dust), cover colour (linear
RGB, per regime in adapter data). Every new boulder parameter's default
leaves today's meshes bitwise unchanged (the rows' tests).

## Build plan

One deliverable + test per slice. IDs `RG-*`; visual checks in `bevyrrowind/docs/visual-checks/worldgen.md`.
**Order (owner 2026-11-22):** finish the normal rocks and their placement first (RG-4 fixes, RG-2 library + collision,
rock placement: scree fans below faces instead of rocks in lines along them); cliff bands (RG-5, RG-6) come later.

- **RG-1 (built 2026-11-22): boulder generator.** `sf_rockgen::boulder`: SDF (joint-set block, chips, smooth max,
  weathering) → surface nets → meshopt LOD chain → `RockLods`. Tests: same seed same hash, closed meshes, no NaN,
  bounds within the declared size, LOD triangle counts strictly decrease; `#[ignore = "measurement: ..."]` ms per
  rock and triangles per LOD.
- **RG-2 (built 2026-11-22): variant library + convex hulls.** `Library` per class from seeds, `collision` feature
  (`parry3d` hull). Bake product **RG-2b** (built 2026-11-22; `sf_bake` product keyed by `library_key` + code
  version, serialised `Variant`/`Hull`); scatter wiring (class + size law per species, `pick`/`scale_for`, hulls to
  `sf_physics` statics) is the placement slice (scatter side: WG-E3f; game side: RG-P, "(f)"). Test: library deterministic; hull contains every LOD0 vertex within tolerance.
- **RG-3 (built 2026-11-22, before RG-2 at the owner's request): viewer + material.** `sf_rockgen_bevy` mesh
  conversion, `VisibilityRange` LODs, triplanar `RockMaterial` (scaffold copy of the terrain functions), the game's
  `level:rock-lab` greybox. Owner look RG-3: one family, too smooth (river boulders) -> sharp families (RG-4).
- **RG-4 (built 2026-11-22): sharp families.** Edge snap onto hard planes, crease normals, size-scaled resolution,
  five new classes (talus, jagged, columnar, scoria, slab). Tests: snapped cube fixture exact (12 triangles), every
  raw net edge-manifold, closed LODs per class. Owner look RG-4: "amazing"; edge snap off preferred. Left: wrong-way
  LOD0 triangles (snap arm only; 0 with snap off). **RG-4b (2026-11-22):** `edge_snap` default off (owner); columnar
  closed + in budget (see As built RG-4b).
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
  layers from the mounted packs, scatter species mapping (mapping: WG-E3f,
  `worldgen-scatter.md` Rocks), cave-mouth sites from the game's cave doors.

**Placement rows (RG-P, "(f)"; specced 2026-11-27, none built).** Order P1 -> P2 -> P3 -> P4 -> P5a -> P5b -> P6;
P3 and P4 need only P1. Then the WG-E3 owner check (`visual-checks/worldgen.md`), with `--off rocklibrary` as its arm.

- **RG-P1: `sf_rockgen::placed`** (Bevy-free; `Libraries` + `read_libraries(store, manifest, class names)` under the
  existing `bake` feature; `model_id` / `parse_model_id`, `Libraries::resolve` -> `Resolved { index, scale, lift_m }`,
  `variant(class, index)`; claims `sf_rockgen/src/placed.rs`, `lib.rs`, tests). Test: id round trip for every built-in
  class name and `parse_model_id` refuses other prefixes or a non-number index; on a 3-variant fixture library `resolve`
  gives `index = hash % 3`, `scale = mark / size_m`, `lift_m = (0.5 - sink) x extent.z x scale` exactly; unknown class
  and empty library give `None`; `read_libraries` over a store baked by `LibraryStep` gives `content_hash`-equal libraries
  (it reads `manifest.get(LIBRARY_STEP, <class name>)`: the product subject is the class name, `bake.rs:82-84`; a class
  the manifest lacks is absent, not an error).
- **RG-P2: library rocks as placements** (`mw_worldgen` `flora_rocks.rs` `library_placements(run, libs) ->
  (Vec<Placement>, HashMap<VfsPath, ModelInfo>, dropped)`; `ROCK_LIBRARY` in `mw_world/src/source.rs`; `mw_worldgen`
  gains the `sf_rockgen` dependency: manifest, `cargo deny`, hakari). Test (fixture run + fixture `Libraries`): one
  placement per library rock with `library_rocks`' ids, class `ROCK_LIBRARY`, scale and lift exact, model ids parse
  back; each info's bounds are the variant's LOD0 bounds axis-converted; an unknown class is dropped and counted.
- **RG-P3: `RockSpawner`** (`sf_rockgen_bevy` new `spawner.rs`; gains `sf_world_bevy`: manifest, deny, hakari). Test
  (headless `World` with mesh and material assets): a rock spawns `lod_levels` parts under its root, two rocks of one
  variant share mesh handles and two classes' rocks share none, with `end_band > 0` the last part's end margin equals
  `shared_range` of `[max_distance, max_distance + end_band]` and with 0 it ends at `shared_range` of `max_distance`,
  every part's range equals `shared_range` of the unsnapped `lod_ranges` value (compare to the snap of the expected
  input, not by re-snapping the output: the log-grid rounding is not guaranteed idempotent), `NoCpuCulling` present iff
  `StaticGpuCulling(true)`, an unknown id spawns 0, `needs_model` false.
- **RG-P4: rock materials from adapter data** (`bevyrrowind/worldgen/rock_textures.toml`; `mw_exterior_bevy` new
  `rock_materials.rs` builder, `level/rock_lab.rs` uses it). Test: the shipped file parses; every class row names a
  built-in class; a class without a row takes `default`; unknown keys rejected (pure parse + resolve test).
- **RG-P5a: draw wiring** (app `world.rs` loads `Arc<Libraries>` and passes it to `flora::extra`; `WorldgenContent.models`;
  `worldgen.rs` `add` reads them before `model_infos`, registers `RockSpawner` for `ROCK_LIBRARY` when the content has
  library rocks; `--off rocklibrary`). Test: the `world_content.rs` content-index test with a fixture library rock
  indexes it with model info (not a point) and the log line counts `+ rocks library N`.
- **RG-P5b: collision** (`mw_world/src/source.rs`: `ROCK_LIBRARY` in `MOVER_SOLID`; `mw_walkers_bevy/src/physics.rs`:
  `rockgen/` branch in the shape source; `sf_physics/src/shape.rs`: prune scaled copies no body holds). Test: a fixture
  hull id gives a `StaticShape` whose AABB equals the axis-converted hull points'; a non-`rockgen/` path still takes the
  `collision.cook` lookup; an unknown variant gives `None`; `MeshShape` with three scaled copies of which one is still
  held keeps exactly that one after the prune (`sf_physics` unit test); `nav/tests.rs` `is_solid` over `MOVER_SOLID`
  still passes. `sf_physics` and `mw_walkers_bevy` are also edited by the interaction and NPC AI lanes: touch only the
  named functions (merge risk).
- **RG-P6: measurement** (`mw_worldgen` test only, `#[ignore = "measurement: rock placements"]` on `SCRIB_X10S`):
  library placements, dropped, index build seconds and MB with and without them, library rocks per 1 km ring at three
  points. Ledger line. Frame cost (`perf ab` on `--off rocklibrary`) goes with the WG-E3 owner sitting.

**Typology rows (RG-9, "(e) Typology"; specced 2026-11-27, none built).** Order: data first (9a, 9b), then the
scatter key and the adapter (9j, 9k), then parameters and recipes (9c-9g), then the cover (9h, 9i). Each new boulder
parameter joins `library_key` (exhaustive destructuring, `library.rs:169-171`) and `TUNABLES`; its default must leave
every existing class's `RockLods.hash` unchanged (fixture: today's 9 class hashes at seed 1-4, recorded in 9a; only
RG-9h re-records it, see there). Reviewed 2026-11-27 against the source (fixes: 9b, 9c, 9d, 9e, 9f, 9h, 9i, covers).

- **RG-9a: class presets as data** (`sf_rockgen::classes`). `classes.toml` (built in, `include_str!`) holds today's 9
  presets as `[[class]]` rows (`name`, `family`, `analogue`, `source`, `tint`, then any `BoulderParams` field by name);
  `ClassTable::parse_toml` + `with_adapter(file)` (adds rows, overrides by name); the `CLASSES` const slice (fn
  overrides) becomes `ClassTable::builtin()`, and its readers take the table: the bake (`mw_boot_bevy/src/bake.rs:285-290`,
  `LibraryStep::from_classes`), the rock lab and `sf_rockgen/src/tests.rs` `classes()`. Test: the 9 parsed classes give the recorded
  `RockLods` hashes bitwise; unknown field, value outside its tunable's range, repeated name rejected; an adapter row
  adds one class and overrides one.
- **RG-9b: eight new classes** as `classes.toml` rows ("(e)": `frost_block`, `bomb`, `column_segment`,
  `beach_column`, `soft_block`, `ventifact`, `river_cobble`, `crust_plate`). Test: the existing per-class tests (`tests.rs` `classes()`: same seed same mesh, closed
  LODs, bounds within size) cover the 8 new rows over 8 seeds, no NaN; LOD0 triangles per class are printed by
  `measure_boulders` (a measurement, no pass/fail budget exists: RG-4b reported counts). Visual check RG-9b (rock lab column per
  class, awaiting owner).
- **RG-9c: tafoni pits** (`sdf.rs` `with_pits`, `sdf.rs:235-285`). A cell's keep threshold is `density x (1 -
  pit_side_bias)` where the union's gradient at the pit centre (centres already sit within `PIT_OPEN_FRACTION x r` of
  the surface) has `n_z >= pit_top_cos`, `density` elsewhere; radius uniform in [`pit_size_spread`, 1] x `max_radius`;
  the five draws per cell unchanged. Class `tafoni` (= `soft_block`, bias 0.9, `pit_size_spread` 0.25: a wide range
  of cavity sizes; pits on the sides and under overhangs). Test: bias 0
  and spread 0.6 give the recorded hashes; bias 1: no kept pit with `n_z >= pit_top_cos` (exact); closed over 16 seeds.
- **RG-9d: bed bands** on a joint block, the `formations.rs` `Stack` band pattern on a block: `beds` > 0 splits the
  block along its shortest axis (the bedding family) into `beds` band `Piece`s that share their bed planes as `hard`
  planes (never rounded, `sdf.rs:69-71`), each band with a seed hardness in [0, 1] and its side planes moved in by
  `bed_recess x size x (1 - hardness)` (a ledge per hard bed; weathering noise softens it). All faces stay planes, so
  the Lipschitz bound and the snap's Hermite data are unchanged. Chips cut the union once (`with_cap`, as a column
  cluster's). Class `bedded`. Test: `beds` 0 gives the recorded hashes; on the raw field, a band's side plane sits
  `bed_recess x size x (1 - h)` inside the unsplit block's (exact, planes); closed over 16 seeds.
- **RG-9e: column fans** (`columns.rs`): entablature as `column_fans` sub-clusters, each `column_cluster` in its own
  frame tilted by `column_fan_tilt_deg` toward azimuth `i x 360 / column_fans` (plus the shared seed phase), unioned;
  a tilted frame keeps every column face a plane (a per-column curved axis would not: Voronoi bisectors of
  z-dependent seeds are not planes, and `Piece` is planes). Thin columns = a high `columns`. Class `entablature`.
  Test: `column_fans` 1 or tilt 0 gives the recorded hashes; on a 2-fan fixture every side plane of a fan's columns
  is parallel to that fan's axis (|n . axis| < 1e-5) and the two axes are `column_fan_tilt_deg` from +Z within 1e-4
  rad; closed over 16 seeds.
- **RG-9f: clump recipe.** `blobs` 3-8 replace the joint block with a union of flattened ellipsoid `Piece`s (no
  planes, `ellipsoid` only; axes `blob_flatten` on z, overlap as `formations.rs` `OVERLAP`); chips cut the union once
  (`with_cap`, as a column cluster's), pits as usual. Class `spatter`. Test: `blobs`
  0 gives the recorded hashes; every blob centre is inside the solid; bounds within size; closed over 16 seeds.
- **RG-9g: tor formation** (`formations.rs`): `Shape::Tor`, a `Stack` whose bands are rounded joint blocks offset
  and yawed per band (stacked corestones). Preset `tor`. Test: closed over 8 seeds, each band's piece overlaps the
  one below by at least `OVERLAP`, bounds within size.
- **RG-9h: vertex cavity** (the unbuilt "Materials" extra). `RockMesh` gains `cavity: Vec<f32>` from the SDF along
  the normal (5-tap SDF ambient occlusion, Evans SIGGRAPH 2006, U), computed per net vertex before the LOD chain; it
  rides in the mesher's `Vertex` (`mesher.rs:129-155`) through `optimize_vertex_fetch` and `split_creases` (every LOD
  vertex is a net vertex or a copy of one); `sf_rockgen_bevy` writes `ATTRIBUTE_COLOR`. Hash: `content_hash` gains the
  cavity, so this row re-records the 9a fixture, with a test that positions, normals and indices hash as before.
  Test: values in [0, 1]; on a block with one pit, pit-floor vertices > flat-face vertices; every LOD vertex's cavity
  equals its net vertex's.
- **RG-9i: cover layer** (`rock.wesl`, `RockExt` of `RockMaterial = ExtendedMaterial<StandardMaterial, RockExt>`):
  a cover mask `smoothstep` of world up-normal^`up_sharpness` x `amount`, shifted by cavity x `cavity_weight`, mixing
  in the cover colour and, if the regime has one, a second triplanar layer (pack texture id); one material variant
  per (rock layer, regime). Switched by flag bits as today's options (`material.rs` `FLAG_*`, `rock.wesl:39-46`):
  `FLAG_COVER`, `FLAG_COVER_TEXTURE`; `amount` 0 clears `FLAG_COVER`. Test: `shader_tests.rs` links and validates,
  the uniform layout and the new bits match Rust; a CPU test of the material builder: `amount` 0 or no cover gives
  today's `RockParams` exactly. Visual check RG-9i: rock lab, fixed hour and weather, live key toggling cover on/off,
  one line naming the live arm (awaiting owner).
- **RG-9j: regime key in the scatter** (`sf_worldgen::scatter_species`, `scatter_rocks`). `[rock_regimes]` maps
  climate class names to regime names (framework default for its `climate_classes.toml`, "(e)"); `[rock_units.
  "<unit>".regime."<name>"]` overrides per context the entry lists and `field_stone_per_ha`; a candidate reads
  `FieldSample::class` at its parent's point (as built: one clump one regime, where the unit is read); placements
  carry `regime: Option<u8>`. Test: round trip; unknown class or regime
  rejected; on a two-class `GridFields` fixture every rock in the frost half draws only frost entries (exact); no
  `[rock_regimes]` gives today's placements bitwise (tile-alone hash).
- **RG-9k: adapter data** (`bevyrrowind/worldgen/scatter_species.toml`, after WG-E3f creates it): the "(e)" unit table
  per context and regime, cover colour per regime and a texture layer only where a pack has one (moss today, "Cover
  assets"). Test: the shipped file parses; every class
  name resolves in the class table; every unit of `geology.toml` has a row. Needs WG-E1c3 (climate rasters) for
  regimes to vary in the world; until then every point is the adapter's default regime.

## Open questions

None for the owner on method (decided from the industry standard above). The look of each class is the owner's
call at RG-3.

Typology (RG-9), scope and taste only:
1. (Answered, owner 2026-11-27: shape and cover.) Should the weathering regime change rock **shape** in the world (frost-shattered vs rounded classes per climate),
   or only the **cover** (moss, lichen, varnish), keeping one shape set per geology unit? Recommendation: both, since
   the research shows climate decides rounding (T§3); "cover only" drops RG-9j's class overrides, not the row.
2. (Answered, owner 2026-11-27: after the WG-E3 check.) Is the moss / lichen cover layer (RG-9h, RG-9i) wanted now, or after the placed rocks have been seen bare (WG-E3
   check)? Recommendation: after, so the owner judges shapes and covers separately.

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

**RG-4b (2026-11-22, rockgen-col).** `edge_snap` default false (owner: rounded edges; snap = the lab's Q arm), which
exposed columnar's LOD0 not closing. Cause (measured): every meshopt level pinched (2-54 doubly used directed edges) so
LOD0 fell back to the raw net (24-160 k tris), from sealed interior bubbles along shared column faces, sub-cell slits
where per-piece rounding of shared floor/chip planes disagreed, and coincident sheets at concave steps. Fix (all
`sf_rockgen`): `Solid::with_cap` (shared planes smooth-maxed once over the union), `mesher::fill_cavities` (flood fill of
the noise band, only for solids with shared faces), `mesher::unpinch` (non-manifold split: radial face pairing + one
vertex copy per fan, as CGAL PMP `duplicate_non_manifold_vertices`; retry schedule + raw-net fallback kept behind),
`separate_duplicates` on the unsnapped path, two exact speed-ups (ambiguity re-passes only near flips, union early-out).
Columnar 2 m 388 -> 97 ms, LOD0/1/2/3 26803/20340/11123/762 -> 1360/679/338/168; 12 m 3013 -> 727 ms, 162 k -> 4267
LOD0; snap arm closes now (805 at 2 m). 17 of 384 other rocks changed (pinched levels now at the nominal ratio, e.g.
scoria LOD3 306 -> 224); same error bound. Left: columnar interior stays near zero along shared faces (block skip
can't prune; a per-shared-face union term would make the cavity fill unnecessary).

**RG-9j (2026-11-27, ecosystems loop).** `scatter_species`: `[rock_regimes]` (class -> regime, classes checked
against the file's class table, <= 255 regimes, ids = sorted names), `[rock_units."<u>".regime."<r>"]` (`RegimeOverride`:
optional `field_stone_per_ha`, per-context lists that replace the unit's, `[]` = none), `entries_in` / `field_stone_in`;
the framework map ships as `rock_regimes.toml` (`default_rock_regimes`). `scatter_rocks`: candidates take `&dyn Fields`,
read the class at the parent (only when a class maps; else no sample, bitwise today's), field-stone bound over the
overrides, outcrop parents kept by the regime's list; `RockCandidate::regime` rides in `rocks.bin`'s former pad word
(0 = none, id + 1), so `FORMAT` 3 products stay valid. `pick` uses the regime's entries. 5 tests (species round trip,
refusals, shipped default; frost-parent rocks draw only frost entries, no table = today's, tile alone).

**RG-2 (2026-11-22, rockgen-rg2).** `sf_rockgen::library` (`build_library(params, &LibrarySettings, seed) -> Library {
key, size_m, variants }`, `Library::pick(placement_seed)`, `scale_for(size_m)`, `content_hash`; `Variant { seed, lods,
hull }`; `Hull { points, triangles, error_m }` plain arrays, `Hull::to_parry()` -> `ConvexPolyhedron` behind feature
`collision` = optional `parry3d`). Variant seeds don't depend on the count (growing a library keeps earlier variants);
variants built at the class size, instances scaled; `library_key` covers every clamped param + count + hull settings
(a new `BoulderParams` field must join it to compile); parallel by index, equal to serial. **Hull from LOD0 points**
(the spec's "lowest LOD" would miss LOD0 vertices, every LOD is a subset of LOD0), reduced by a greedy quickhull
refinement (parry3d has no hull reducer) until every vertex is within `min(0.01 x size, 0.05 m)` or 255 vertices
(PhysX `PxConvexMeshDesc` vertex limit). The hull is inside the mesh's convex hull by at most `error_m`; over
concavities (pits, grooves) it is solid where the mesh is open, as any single-hull simple collision. Tunables
`rockgen.library.variants` 16 (8-32), `rockgen.collision.hull_tolerance` 0.01, `hull_tolerance_max_m` 0.05,
`hull_max_vertices` 255. Release, 16 variants at 2 m: library 19-185 ms per class (all 9: 0.67 s), hull 48-109
vertices (exact hull 89-249), 0.9-4.9 ms per hull, max error 20 mm.

**RG-2b (2026-11-22, rockgen-rg2b).** `sf_rockgen::bake` (feature `bake` = sf_bake + serde + postcard; crate stays
Bevy-free): step `rockgen.library`, subject = class name, all 9 `CLASSES`; `version()` = `library::LIBRARY_FORMAT`
(doc lists the generator files that must bump it; also inside `library_key`, so one bump re-keys everything). Planned
parameters: `library_key(params, settings, seed)` + whether hulls are built; reads nothing, so one class's param change
re-bakes only that class. Class seeds `class_seed(LIBRARY_SEED, name)`. Output = postcard `Library` (store-compressed,
as `lod.policy` / `nav.*`); `decode(&[u8])`. Registered by the game in `mw_boot_bevy/src/bake.rs`
(`bake_rock_libraries`, features `bake,collision`), after the pipeline catalog, before the manifest. Release: 9 x 16
variants 0.59 s cold (4142 KiB), 8.6 ms warm. **Gap:** the bake passes the tunables' defaults (no tunable store in the
bake yet), so a mod override of `rockgen.*` doesn't reach the baked library ("everything is a mod"): the placement
slice or a bake-tunables slice passes adapter/mod values through `LibraryStep::new`.

**RG-8a (2026-11-23, first try, owner ask "spires, volcanic towers, arches").** `sf_rockgen::formations`: presets
`FORMATIONS` (Guess numbers, as `CLASSES`) over three recipes, all unions of convex pieces in the boulder `Solid`, so
the mesher, LODs, crease normals and snap are shared: `Stack` (polygonal frustum bands between jittered boundaries,
hardness-recessed, optional caprock and wave notch, lean, twist, drift, foot flare; hoodoo, pinnacle, sea_stack),
`Neck` (the columnar cluster standing tall, tapered by shared cap planes; volcanic_neck) and `Arch` (half ring of
convex segments, radial cuts + outer tangent + inner chord, thinning to the crown, on flared legs; the spec's capsule
subtraction needs an SDF difference the `Solid` doesn't have). Pieces overlap 15 % (≥ 3 cells) so the union has no
seam. `generate_formation(f, mesh, size_m, seed)` takes the boulder params for meshing only. Also
`rockgen.boulder.aspect_spread` (octaves, default 1, in the library key): each seed scales both aspect ratios by its
own `2^(±spread)` on a separate stream. Shown in the rock lab's formation row. Open: recipes as adapter data (RG-A1),
placement rules, V-HACD or per-piece hulls for collision, an SDF difference for carved arches and caves (RG-7).

**RG-P1 (2026-11-27).** `sf_rockgen/src/placed.rs` (+ `placed/tests.rs`): `Libraries`, `Resolved`, `model_id` /
`parse_model_id` (prefix `MODEL_PREFIX`; ids with a sign, a leading zero or a separator in the class are refused, so the
round trip is exact), `resolve` and `variant` as "(f)" 2-3; `read_libraries` + `ReadError` under `bake`. Ids are plain
`String`s (the crate takes no `sf_vfs`): the content builder wraps them in a `VfsPath`, unchanged by its lowercasing.

**RG-P2 (2026-11-27).** `mw_worldgen` `flora_rocks.rs`: `library_placements(run, libs)` = `resolve_library_rocks(
library_rocks(run), libs)` -> `LibraryPlacements { placements, models, dropped }` (a struct, not the planned tuple);
base `RecordRef("rockgen", <class>)` (`LIBRARY_BASE_KIND`), `engine_bounds` for the infos; an id `VfsPath::new` refuses
is dropped too. `mw_world::source::ROCK_LIBRARY = Custom(3)`, not yet in `MOVER_SOLID`. `mw_worldgen` takes `sf_rockgen`
without features (the app's reader needs `bake`, `mw_boot_bevy` already has it).

**RG-P3 (2026-11-27).** `sf_rockgen_bevy/src/spawner.rs` (+ `spawner/tests.rs`): `RockSpawner::new(libs, meshes,
&RockMaterialSet, RockLodSettings)` builds every variant's LOD meshes once; `RockMaterialSet { default, by_class }`.
Parts go straight under `cx.parent` with `cx.local` (no root entity per rock, as `StaticSpawner`), each with the
worldspace's `RenderLayers`, the variant's engine-frame LOD0 `Aabb` + `NoAutoAabb`; `part_ranges` drops levels whose
start fade begins past `max_distance` and ends the last kept one at `[max, max + band]`. Gains `sf_world` + `sf_world_bevy`.

**RG-P4 (2026-11-27).** `bevyrrowind/worldgen/rock_textures.toml` (`default` = `tx_rock_wg_01`, no class rows: which
texture suits which class is the owner's WG-E3 call) read by `mw_exterior_bevy::rock_materials`: `RockTextureTable::parse`
(unknown keys, non built-in classes, empty textures rejected; paths normalized as `--rock-textures`), `material_set(..)`
-> `RockMaterialSet` with every built-in class (its texture x its `tint`; equal pairs share handles, one batch) and an
untinted `default`. `rock_textures` + `lod_materials` are the rock lab's path, which now calls them. Gains `serde`, `toml`.

**RG-P5a (2026-11-27).** App `world.rs` `rock_libraries` reads every built-in class's product (none baked or one
unreadable: logged, left out; `--off rocklibrary` = new `Part::RockLibrary`: not read) and passes them to
`mw_worldgen::flora::extra`, which now returns `Extra { lists, models }` (list `"rocks library"` = `LIBRARY_ROCKS`).
`WorldgenContent` gains `models` (looked up before the bake's by `content_models`) and `rock_libraries`; with a
`ROCK_LIBRARY` placement, `worldgen::add` registers the `RockSpawner` in `Startup` after `exterior::setup_spawners`
(before the `PostStartup` hooks, so RG-P5b's physics hook will wrap it), materials at the max variant LOD count.
