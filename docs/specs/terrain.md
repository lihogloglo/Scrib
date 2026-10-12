# Terrain

Status: agreed (2026-10-11)
Crates: `sf_terrain` (new, Bevy-free: tiles, height stack, CPU queries,
bake steps, CDLOD selection), `sf_terrain_bevy` (new: clipmap textures,
material, colliders)

## Goal

Render heightfield terrain from any source spacing as one continuous
surface out to the horizon, with ~0.5 m detail near the camera, and serve
the *same* heights to rendering, physics, navigation, foliage, and water.
This settles WISHLIST early decision 3: **terrain is a layered stack that
every consumer reads.**

Non-goals for M2 (hooks only): erosion (M7), persistent height edits (M10;
the data model is fixed now), transient deformation (M6), virtual texturing
of the splat result (M7), overhangs and caves (heightfield only; a hole mask
covers cave entrances).

## Canonical technique

- **Geometry: CDLOD** (Strugar, *Continuous Distance-Dependent Level of
  Detail for Rendering Heightmaps*, JGT 2009). A quadtree is selected by
  3D distance ranges, one shared grid patch is drawn instanced per node, and
  the vertex shader morphs between levels so there are no cracks or popping.
  It adapts to camera height, which matters for the M2 fly-over.
- **Height residency: geometry-clipmap texture stack** (Losasso & Hoppe,
  SIGGRAPH 2004; Asirvatham & Hoppe, *GPU Gems 2* ch. 2). The GPU holds L
  toroidally updated levels of N² heights centered on the camera, each level
  at double the spacing of the one before. A node at LOD k samples level k,
  or a coarser level if k isn't resident. Far Cry 5 (GDC 2018) uses the same
  split: quadtree selection over streamed height data.
- **Splatting: height-based blending** (Mishkinis, *Advanced Terrain
  Texture Splatting*, 2013). Each texel stores its top-4 layer indices and
  weights, sampled from texture arrays. Triplanar only where the slope
  needs it. Macro variation and distance-dependent UV scale fight tiling.
  Beyond the splat range, a baked macro-albedo pyramid makes far terrain a
  single fetch.
- **Upsampling: clamped Catmull-Rom** (interpolating bicubic, overshoot
  clamped to the 4×4 neighborhood's min/max, as in TAA resolves). It passes
  through the source samples and adds no ridges. Refinement steps (detail,
  erosion) stack on top, and a constraint step keeps placed objects
  grounded.

**Reuse check (2026-10-11).** The CPU core is our own code, about 2,000
lines. Nothing on crates.io fits:
- `fast_image_resize` 6.1 (MIT/Apache) has Catmull-Rom convolution and
  float crops. It has no neighborhood clamp, it resamples one contiguous
  image (our windows stitch many source tiles), and its per-CPU SIMD paths
  put bit-exact bake determinism across machines at risk. The separable
  pass we need is about 100 lines.
- `oxigeo-algorithms` 0.2 (Apache-2.0) has unclamped bicubic geospatial
  resampling.
- `martini` 0.2 builds RTIN meshes, which CDLOD doesn't use.
- `bevy_terrain` (UDLOD over chunked clipmaps) is Bevy-bound, and its last
  push was in April 2025. It's a reference for `sf_terrain_bevy`, not for
  this core.

## Data model

**Canonical tiles** (bake products, bake.md), aligned to the
world-coordinates grid:

| Level k | Tile edge | Spacing | Samples |
|---|---|---|---|
| 0 | 256 m | 0.5 m | 513² (shared borders) |
| k | 256·2^k m | 0.5·2^k m | 513² |

Level k is level k−1 sampled at even indices, so coarse lattices nest
exactly inside fine ones (needed for crack-free morphing). The pyramid goes
up until at most 2×2 tiles cover the worldspace. Not one: the grid corner
at −128 m is a tile border at every level (tiles −1 and 0 never share a
parent), so a world around the origin keeps up to four quadtree roots.

Per tile:
- **heights**: u16 with a per-tile base and step (step ≤ 2 cm). A tile whose
  range doesn't fit (more than 1.3 km) falls back to f32. The border ring
  (2,048 samples, 8 KiB) is stored as f32 in both forms, so neighbors decode
  bit-identical shared edges whatever their steps.
- **min/max** per 16×16 block, for CDLOD node bounds and culling.
- **layers** at 1 m (257²): 4 slot indices (u8, into a per-tile palette of
  global `TerrainLayer` ids) plus 3 weights, packed in two RGBA8 textures.
  Weights sum to 255, heaviest first. Coarser levels keep the dominant
  layers: each sample accumulates the 3×3 child samples around it with
  (1, 2, 1) tent weights (in integers) and keeps the top 4.
- **tint** (optional, RGB8 at 2 m): macro color the adapter supplies, for
  example source vertex colors.
- **holes** (optional): 1 bit per 0.5 m quad.

**`TerrainLayer`** (`sf_records` kind; adapters map their texture records
to it, mods patch it):
`{ albedo_height: VfsPath, normal: Option<VfsPath>, arm: Option<VfsPath>,
tiling_m: f32, triplanar: bool, displacement_m: f32, surface: RecordRef }`.
`surface` points at a surface type: footstep sounds, the deformation
material (snow, mud, ash, sand, none), and the grass rule (M6).
The bake resamples every layer into fixed-size **texture arrays**:
1024² by default, 2048² on high. BC7 for albedo + height, BC5 for normal,
BC7 for ARM, full mips. Missing maps get conservative defaults until PBR
generation (M7).

**The height stack:**

| Layer | Lifetime | Storage | Read by |
|---|---|---|---|
| 1. Base | per mod set | baked tiles (immutable) | everyone |
| 2. Persistent edits (M10) | per save | sparse 32 m bricks of f32 deltas at 0.5 m, in the save overlay | everyone |
| 3. Transient deformation (M6) | seconds | GPU ring-buffer render target around the camera | rendering, grass |
| 4. Detail displacement (M2) | static | none: computed in the vertex shader from the layers' height channels | rendering, grass |

**Detail displacement** is how terrain gets finer than the 0.5 m bake for
free. Within about 64 m of the camera, CDLOD adds two levels below LOD 0
(0.25 m and 0.125 m vertex spacing). wgpu has no tessellation shaders, so
the denser patches are the standard substitute. Their heights are a bicubic
filter of the base, plus the splat layers' height maps blended by the same
height blend the material uses. Rocks, cobbles, and ruts appear in
geometry, not only in normals. Amplitude is per layer (`TerrainLayer`), at
most 8 cm by default. That is under foot-IK and collision tolerance, so
CPU consumers ignore it, as they ignore transient deformation.

CPU consumers read **base + edits**. Rendering reads all four. When edits
change, `TerrainChanged { worldspace, rect, revision }` fires, and physics
chunks, navmesh tiles, grass masks, the shoreline field, and GPU clipmap
regions rebuild that rect. Per-tile revision counters let caches check
staleness.

**Refinement chain** (bake): `upsample → [detail] → [erosion, M7] →
constrain`. Each step is a versioned function over a height grid plus
context (source surface, footprint mask, layers). Adding erosion is a new
step version, not a format change. Every refinement step counts against
the bake budget (bake.md: the whole first bake in ≤ 2 min), so erosion runs
as GPU compute. **constrain**: under the footprints of
`GROUNDED` placements (from `world.index`), the height blends back to the
source's own reference surface over 1.5 m, so objects authored on the old
surface stay grounded.

## Interfaces

```rust
/// Adapter side (Bevy-free). sf_terrain depends on sf_world (for
/// `WorldspaceId`, `Fingerprint`), not the reverse: adapters hand their
/// `TerrainSource` to the terrain bake directly, not through `WorldSource`.
pub trait TerrainSource: Send + Sync {
    fn tiles(&self, ws: &WorldspaceId, out: &mut dyn FnMut(SourceTileInfo));
    fn read(&self, ws: &WorldspaceId, tile: SourceTileId) -> Option<SourceTile>;
}
/// Bounds let the bake planner map canonical tiles to source tiles without reading them.
pub struct SourceTileInfo { pub id: SourceTileId, pub fingerprint: Fingerprint, pub min: WorldPos, pub max: WorldPos }
pub struct SourceTile {
    pub origin: WorldPos,             // sample (0, 0)
    pub spacing: f32,                 // meters
    pub size: UVec2,
    pub heights: Vec<f32>,            // meters; row-major, x fastest, rows step +Z
    pub surface: Triangulation,       // Bilinear | Fixed(Diagonal) | Alternating(Diagonal)
    pub layers: Option<SourceLayers>, // texels naming TerrainLayer records, on their own grid
    pub tint: Option<SourceTint>,
}

/// Runtime CPU queries (`sf_terrain`): physics, AI, foliage, water. One
/// instance per worldspace (the Bevy side keeps a map by `WorldspaceId`),
/// so hot queries skip a lookup. It owns that worldspace's `HeightEdits`.
impl TerrainHeights {
    pub fn height(&self, p: WorldPos) -> Option<f32>; // None = not resident
    pub fn normal(&self, p: WorldPos) -> Option<Vec3>;
    pub fn raycast(&self, ray: WorldRay, max: f32) -> Option<TerrainHit>; // two-sided
    pub fn sample_rect(&self, rect: WorldRect, spacing: f32, out: &mut [f32]) -> usize; // NaN + count when not resident
}
```

**Core functions** (`sf_terrain`, pure, one tile at a time; bake steps
wrap them): `SourceMosaic::new` (stitch source tiles onto one lattice),
`upsample`, `FootprintMask` + `constrain`, `downsample`, `BlockBounds`,
`EncodedHeights`, `resample_layers` / `downsample_layers`,
`resample_tint` / `downsample_tint`. Canonical tiles split quads along the
(i, j)→(i+1, j+1) diagonal (`CANONICAL_DIAGONAL`); CPU queries use it, and
colliders and the render patch must too.

Implementation rules that keep seams and caches exact:
- Every result is a function of **global sample indices** (level-0 sample
  `g` sits at `g · 0.5 − 128` m) and the inputs, computed with one formula,
  never of which tile computes it. Shared edges are bitwise equal, and
  results don't depend on thread count.
- A worldspace's source tiles share one lattice. Its phase is canonicalized
  (rounded to 0.1 mm; `WorldPos` f32 offsets carry µm noise), so a mosaic
  of any subset of the tiles (what one bake step reads) gives the same
  bits as the whole set. Where tiles overlap (shared borders), a fixed
  order decides.
- Sources finer than 0.5 m are downsampled with the same kernel stretched
  by the ratio and normalized (the usual resize prefilter), not point-sampled.
- Edit bricks own 64² samples each (no shared border), so every sample has
  exactly one delta; the delta is added before interpolation.

CPU residency: level-0 tiles within R1 (streaming.md) plus every coarse
level (small). Requests go through the streaming scheduler's budgets.

**Rendering (`sf_terrain_bevy`):**
- **Height clipmap**: 10 levels × 512², R32Float (10 MiB), from 0.5 m to
  256 m spacing. Strips are updated toroidally when the camera moves, under
  the upload budget. Normals come from central differences in the shader.
  A matching **layer clipmap** covers the splat range (~2 km). Beyond it,
  the macro-albedo pyramid takes over.
- **CDLOD**: a 32×32-quad patch (16 m at LOD 0), ranges double per LOD, and
  morphing covers the last 30% of each range. Selection runs on the CPU over
  the min/max pyramid (a few hundred nodes, ≤ 0.2 ms) into an instance
  buffer, with one draw for the main view. Shadow views select separately.
  GPU-driven selection comes later, if profiles ask for it.
- **Precision**: node origins are relative to the floating origin, and
  texturing uses the `WorldOrigin` phase (world-coordinates.md).
- **Material**: a custom `Material` with prepass support. It builds Bevy's
  `PbrInput` and calls Bevy's PBR lighting, so shadows, clustered lights,
  fog, and atmosphere match objects. It includes the surface-state WGSL hook
  from day one (a no-op until M6; §10 decision). Terrain draws in the depth
  prepass, where it's the main occluder for `OcclusionCulling`.
- **Far shadows** (weather.md "Distant shadows", as built 2026-10-12): past the sun's cascades
  (504 m in the app, derived from the far-field handoff) the material sets Bevy's
  `PbrInput::directional_shadow_factor` to the terrain's own shadow: a horizon march over the height
  clipmap toward the shadowed light out to 700 m view distance (`shaders/far_shadow.wesl`), then the
  CPU shadow-height map `FarShadowMap` (`far_map.rs`, `sf_terrain::shadow`; 384² × 24 m, also read by
  far-field proxies and impostors). The cascade end is a dithered transition (`P_FAR`, `P_DETAIL.w` =
  highest resident height). Off: `TerrainDebug::no_far_shadows`.
- **Solari**: CDLOD's shader-morphed geometry can't enter the ray-tracing
  acceleration structure. The bake produces static **RT proxy meshes** per
  tile (2 m spacing near, coarser far) as `RaytracingMesh3d`, resident by
  ring.
- **Colliders**: heightfield per 64 m chunk (129² at 0.5 m) inside R0,
  built from base + edits on the task pool and rebuilt on `TerrainChanged`.

**Water (the parts of `bevy-aqua` that affect terrain).** Terrain exports
three things:
1. CPU `TerrainHeights`, for depth under a point (wading, buoyancy).
2. The height clipmap as bindable GPU textures, plus the origin phase.
3. Heights as a bake input for the shoreline distance field, which belongs
   to `sf_water`; `TerrainChanged` invalidates it.

`bevy-aqua` 0.1.3 (Bevy 0.19) takes one global `BedHeightMap` image (normalized
height plus origin, size, and range). That suits a small static world, not
a streamed one. If we adopt it, `sf_water` copies a camera-centered,
non-toroidal window out of the clipmap whenever the camera moves a step (a
cheap GPU copy), or we patch aqua to sample the clipmap. Its queries are GPU
readback (about one frame of latency, 256 probes per frame). That doesn't
affect terrain, but it is not the CPU water query we decided on (§10).

## Object-terrain blending (as built, 2026-10-12, terrainblend)

Placed rocks, cliffs, tree roots and building bases blend into the ground
they stand on, instead of meeting the splat with a hard seam.

**Technique.** The UE runtime-virtual-texture mesh blend (the landscape
writes base color and world height into RVTs; mesh materials sample them and
lerp albedo and normal by height above the landscape over a fade band) and
CryEngine 5's "terrain layer blending into objects" (the object shader
blends the terrain's layers, per-object flag). Our clipmaps already are what
the RVT provides there: camera-centered height, splat layers and macro
albedo, so object shaders sample them directly and no virtual texture is
needed. Reuse order: Bevy has no terrain blend; no crate blends into a
custom terrain; so this is our own, on Bevy's `ExtendedMaterial`.

**How it works.**
- `GroundBlendMaterial = ExtendedMaterial<StandardMaterial, GroundBlendExt>`
  (`sf_terrain_bevy::ground_blend`). The extension carries the terrain
  material's own handles (params buffer, height, layer, macro, normal
  clipmaps, layer arrays, far map) at the terrain's bindings, renumbered to
  100..112 so they sit beside `StandardMaterial`'s 0..30, plus the band
  (`#[uniform(113)]`: band meters, edge break-up).
- Fragment `shaders/ground_blend.wesl`: Bevy's
  `pbr_input_from_standard_material`, then height above terrain =
  fragment height − `height_bicubic(xz)` (the same Catmull-Rom height the
  near terrain is drawn with). Inside the band it runs `terrain_surface`
  (`shaders/surface.wesl`: the terrain fragment's surface code, moved into a
  shared module so terrain and objects can't drift) and lerps base color,
  roughness, metallic (to 0), emissive (to 0), diffuse occlusion and the
  shading normal toward the ground's (layers projected by the object's
  normal, see below), weight
  `smoothstep(0.35, 0.75, t + 0.5 · (h − 0.5))` with `t` = 1 at the ground
  and 0 at the band top and `h` the splat layers' blended height map, so
  pebbles and grains creep higher than flat soil (the height-blend rule, as
  the terrain's own layers). Outside the band the cost is the one height
  lookup. Forward only (`opaque_render_method = Forward`); prepass and shadow
  passes are `StandardMaterial`'s, so depth, motion vectors (DLSS/TAA) and
  shadows are unchanged.
- **Opt-in per model, by the adapter.** `sf_world_bevy::GroundBlendModels`
  (a rule: model path → band in meters) is evaluated once per model when its
  template is built (`ModelTemplate::ground_blend`); `spawn_template` adds
  `GroundBlend { band, worldspace }` to every part. `GroundBlendPlugin::<M>`
  (`sf_terrain_bevy`, generic over the host's marker so neither crate depends
  on the other) swaps marked parts in the terrain's worldspace to a cached
  `GroundBlendMaterial` per (source material, band), drops the marker in
  other worldspaces (interiors share coordinates; they must never sample the
  terrain) and on transparent materials, and rebinds every cached material
  in place when the terrain is rebuilt. Bevyrrowind's rule
  (`mw_assets::ground_blend_band`, from a census of the vanilla and
  expansion meshes): rocks and cliffs 0.6 m, building bases 0.4 m (Hlaalu,
  Redoran, Imperial, Telvanni, Velothi, Dwemer, Nord, stronghold sets,
  minus bridges, docks, awnings, windows, doors and other parts that hang
  above the ground), tree trunks and roots 0.3 m.
- **A/B.** `--off terrainblend` leaves out the rule and the plugin (no
  extended material exists). The live key F11 flips
  `TerrainDebug::no_ground_blend` (params flag `DEBUG_NO_GROUND_BLEND`), so
  the band test is skipped on the GPU without re-spawning.

**Not blended (decisions).**
- Far-field proxies: no runtime blend (their merged, vertex-colored cell
  meshes mix blending and non-blending models; the seam is sub-pixel at
  proxy distance). Follow-up: bake the blend into proxy vertex colors (macro
  albedo by vertex height above terrain).
- `--rt` (Solari): off. Solari's RT scene reads `MeshMaterial3d<StandardMaterial>`
  only, so an extended material would drop the mesh from the RT scene.
- Pixel depth offset + dithered temporal AA (the UE trick for soft
  intersections): rejected for now. PDO writes depth from the fragment
  shader (no early-Z on those meshes, prepass and shadow interplay) and the
  dither is noisy under DLSS.

**Change from the RVT blend (2026-10-12).** The first version projected the
layers top-down, as UE's RVT blend does: on Balmora's wall bases that read
as vertical streaks and a bright band (the ground's up-facing normal on a
wall). `terrain_surface` now takes the object's normal and
`force_triplanar`: flat parts still continue the ground top-down, steep
faces get the same layers, tint and variation side-on, and the shading
normal stays around the object's, so the lighting keeps its shape. The
terrain passes `false` (its own triplanar stays per-layer flagged).

**Known limits.** Triplanar side projections don't line up with the
ground's top-down texture at the contact line (colour matches; pattern
turns 90°). The height
reference ignores detail displacement (±8 cm, inside the band). Blended
parts are not bindless (the extension isn't), so they draw in their own
batches. GPU cost (indicative, not measured: owner on the machine): one
bicubic height lookup (16 loads) per fragment of a blended model, plus the
full splat shading inside the band.

## Verification

- Unit tests (`sf_terrain`):
  - upsampling passes through source samples (≤ 1 mm) and never leaves the
    4×4 neighborhood's range;
  - neighboring tiles' shared edges are bitwise equal at every level;
  - coarse samples equal fine samples at even indices;
  - under `GROUNDED` footprints the height matches the source reference
    surface within 1 cm;
  - stack composition order.
- Bake determinism: same inputs give the same tile hashes (bake.md).
- Visual: an LOD-colored view and a wireframe fly-over checked for cracks
  and popping. Screenshots from fixed cameras (summit, coast, valley) in
  raster and Solari.
- Perf (`sf_diag`, 1440p, RTX 3070-class): terrain GPU ≤ 1.5 ms including
  shadows and detail displacement; CPU selection ≤ 0.2 ms; clipmap updates ≤ 2 MiB/frame at 60 m/s.

First real content (2026-10-11, the first content pack's exterior, 1,540
source tiles at 1.83 m; RTX 4060 Laptop, 720p): cold bake 7.8 s for 376
level-0 and 179 pyramid tiles plus macro and RT proxies. At pivots of
placements under `constrain`, the baked surface matches the source's own
triangulated surface to 0.4 mm median, 3.2 cm p99; elsewhere the upsample
deviates 2.7 mm median, 9.4 cm p99. Runtime in a town: 511 patches,
selection 0.24 ms median (over the 0.2 ms target), whole update 0.27 ms;
at 60 m/s uploads stay ≤ 1.2 MB/frame, but one frame's update took 16.8 ms
(a burst of clipmap level moves right after a floating-origin shift; not
investigated yet). LOD colors and wireframe show no cracks. The splat
boundary sits where neighboring texels' weights cross, because the
fragment's height blend only mixes layers within `blend_depth`; an
adapter's `blend_radius` sets how wide the weight ramp is (the first
content pack uses ¾ texel).

## Decisions made (2026-10-11, owner's priorities: speed, visuals, bake once)

- **Base spacing stays 0.5 m.** Baking at 0.25 m would quadruple disk use
  and bake time and add no information: an upsample is only smooth
  interpolation. Detail displacement (above) puts the real near-camera
  detail in geometry at zero bake cost. Erosion (M7) may justify a finer
  base later, if it fits the bake budget.
- **Clipmap storage: R32Float.** 10 MiB of VRAM buys no quantization and
  simpler shaders.
- **Solari RT proxies: yes.** They are sized by a VRAM budget per quality
  preset; the near ring gets 2 m spacing, coarser rings less.
- **Layer map: 1 m.** Height-blended splatting and detail displacement
  carry the fine detail. Mods that need finer masks get a source path when
  one asks for it.

## Industry comparison

J9 (industry-audit.md), 2026-10-12, docs research only (`.sessions/j9rest.md`).
"Unverified" marks claims from memory of talks or docs that weren't re-read this session.

| Technique | Industry | Ours | Verdict |
|---|---|---|---|
| Geometry LOD | UE4/5 Landscape: per-component quadtree LOD with screen-size ranges, morphing between LODs (unverified detail); UE 5.2+ optional **Nanite landscape** (baked Nanite mesh, mainly for virtual shadow maps; must be rebuilt after sculpting). Unity: quadtree patches by "pixel error". Far Cry 5: GPU compute LOD selection, culling and stitching. OpenMW: quadtree chunks with LOD factor. Bethesda: near cells full-res, distant terrain as pre-generated LOD meshes per 4/8/16/32-cell block (unverified) | CDLOD over a geometry-clipmap height stack, CPU selection | **match** (CDLOD/clipmap is the standard family). Differ: CPU selection instead of FC5-style GPU; deferred until selection > budget (now 0.24 ms vs 0.2 target). Nanite landscape is n/a (Bevy meshlets rejected, far-field.md) |
| Displacement below the base spacing | UE4 hardware tessellation, removed in UE5 and replaced by Nanite tessellation/displacement (5.3+, experimental-to-beta, unverified status); FC5 "displacement geometry"; Witcher 3 tessellated terrain (unverified) | denser CDLOD patches within 64 m + layer height maps (wgpu has no tessellation) | **match** in effect. Parallax occlusion past 64 m: not used (normal maps only); acceptable at that distance |
| Distant texturing | Unity terrain "basemap" (precomposited low-res albedo past a distance); OpenMW "composite map" (distant chunks' layers pre-rendered); Far Cry 4 adaptive virtual texture (Chen 2015); UE runtime virtual texture (RVT) caching the layer blend | splat to ~2 km, baked macro-albedo pyramid beyond | **match** basemap/composite map; RVT caching is audit row 25 (P2, M7). Note: distant normals come from central differences of the coarse height level; UE stores landscape normals with the heightmap (unverified), so lighting detail drops with level. Watch in vistas, not a gap yet |
| Splat blending | height-based blending (Mishkinis 2013) in UE (`Height Blend` layers), Unity HDRP/URP terrain (height-based blend option), most AAA; triplanar on steep slopes | height blend, top-4 layers per texel, triplanar per layer | **match** |
| Anti-tiling | macro variation and distance UV scale (universal); **stochastic / hex tiling** (Heitz & Neyret 2018 by-example noise; Mikkelsen 2022 "Practical Real-Time Hex-Tiling", Unity Labs; UE material-node ports) | macro variation + distance UV scale + tint | **GAP (P2)**: hex tiling is cheap (3 taps per layer, or only on the nearest layers) and kills the grid pattern on large uniform layers. Open question 1 |
| **Object-terrain blending** | UE RVT "blend with meshes": rocks and props sample the landscape's virtual texture and blend into it near the ground (Epic RVT docs list it as a primary use); Far Cry 5, Horizon, Star Citizen do the same family (unverified per title) | object shaders sample the terrain clipmaps (our RVT) and lerp albedo, roughness, AO and normal toward the ground over a per-model band (adapter opt-in); CryEngine 5 "terrain layer blending into objects" is the same family | **match** (2026-10-12, "Object-terrain blending (as built)"). Not on far-field proxies (vertex-colour bake later) or `--rt`. UE pixel depth offset + dither: rejected for now |
| Holes | UE landscape visibility mask, Unity terrain holes, Bethesda/OpenMW none (caves are meshes) | 1 bit per 0.5 m quad | **match** |
| Self-shadowing far away | UE distance field shadows / far shadows (unverified for landscape), Bethesda LOD shadows | horizon march + CPU shadow-height map (weather.md) | **match** |
| Horizon/terrain occlusion of objects | common in terrain engines | none | gap already listed (audit row 16, P2) |
| Height editing at runtime | UE Landscape edit layers (editor), Ghost of Tsushima/Horizon deformation as transient render targets | height stack with edits (M10) + transient ring buffer (M6) | **match** (design) |

**Sources** (checked 2026-10-12):
- UE, Using Nanite with Landscapes: https://dev.epicgames.com/documentation/en-us/unreal-engine/using-nanite-with-landscapes-in-unreal-engine
- UE, Runtime Virtual Texturing quick start (landscape + mesh blending): https://dev.epicgames.com/documentation/en-us/unreal-engine/runtimevirtual-texturing-quick-start-in-unreal-engine
- UE, Virtual texturing: https://dev.epicgames.com/documentation/unreal-engine/virtual-texturing-in-unreal-engine
- CryEngine 5, Terrain Layer Blending into Objects (object shader blends terrain layers; per-object flag; the fetch returned only navigation, details unverified): https://www.cryengine.com/docs/static/engines/cryengine-5/categories/23756816/pages/36340007
- UE RVT mesh blend in practice (World Height RVT, lerp by height above the landscape): MythicLemon "MF_RVTBlend" article; unrealcode.net "Nanite Landscape Materials" (forum-level sources)
- Moore, "Terrain Rendering in Far Cry 5", GDC 2018 (abstract: GPU LOD selection, culling, stitching; cliffs and displacement geometry): https://gdcvault.com/play/1025480/Terrain-Rendering-in-Far-Cry
- Mikkelsen, "Practical Real-Time Hex-Tiling", JCGT 11(3), 2022: https://jcgt.org/published/0011/03/05/paper.pdf
- OpenMW terrain settings (distant terrain, composite map): https://openmw.readthedocs.io/en/openmw-0.46.0/reference/modding/settings/terrain.html
- Already cited above: Strugar 2009, Losasso & Hoppe 2004, Mishkinis 2013; Chen 2015 in industry-audit.md.

## Open questions

None at the design level. Tuning numbers (CDLOD ranges, displacement
amplitude, RT proxy budget) are calibrated against the perf gates.

From J9 (2026-10-12, industry comparison above):
1. **Hex tiling (P2).** Mikkelsen 2022 hex tiling on the 1–2 nearest
   layers of each texel (or on layers flagged `anti_tile` in `TerrainLayer`),
   with the usual mip-safe gradients. Cost: 3 fetches per affected layer.
   Decide with a screenshot pair on a large uniform layer and a GPU A/B.
2. **Object-terrain blending: closed 2026-10-12** (see "Object-terrain
   blending (as built)"; per-model adapter opt-in instead of a bake flag,
   since the band test is per fragment). The original note: static meshes flagged by the
   bake as touching terrain (`GROUNDED` footprint, or vertex within ~0.5 m
   of the surface) blend their albedo/normal toward the terrain's shaded
   layers by height above terrain (fade over ~0.3–1 m), sampling the layer
   clipmap (near) or macro albedo (far) through a WGSL import exported by
   `sf_terrain_bevy`. Same family as UE RVT mesh blending without a virtual
   texture. Needs: the clipmaps bindable from object materials (bind group
   cost), a per-mesh opt-in, and a check against the Solari path.
