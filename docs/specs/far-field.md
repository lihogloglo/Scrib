# Far field: HLOD hierarchy, impostors, distant lights

Status: implemented and verified on Morrowind (2026-10-11): sf_lod + sf_lod_bevy +
streaming hook built and tested; bake and runtime measured on Morrowind (below,
"Measured on Morrowind"); two Bevy 0.20 pitfalls found and worked around
("Bevy 0.20 pitfalls"). Moves HLOD and
impostors from M7 into M2 (owner, 2026-10-11: "terrain, assets on
terrain, LOD/HLOD/impostors, fast streaming, very distant rendering" is
the current goal).
Crates: `sf_lod` (new, Bevy-free: node tree, cluster build, impostor
capture, light batches, bake steps behind feature `bake`), `sf_lod_bevy`
(new: plugin, node loader, impostor and light-sprite materials, residency
fallback, counters)

## Goal

Everything static in a worldspace is visible out to the horizon, at a
frame cost that doesn't grow with view distance, and with no streaming
spikes as the camera moves. Per-object LOD chains (bake.md) handle the
first few hundred meters; beyond that the world is drawn by a small,
bounded number of baked cluster meshes, instanced impostors, and light
sprites.

Non-goals (M7+): texture streaming, far foliage folded into the terrain's
macro albedo, a second HLOD for interiors, Solari RT scenes from HLODs
(the ray tracer keeps using cooked object LODs within its own range).

## Research summary (2026-10-11)

Reuse order (CLAUDE.md rule 8): Bevy, then crates, then our own.

- **UE5 World Partition HLOD.** HLOD actors stand in for *unloaded*
  cells and stream on their own runtime grid. Layers: *Instancing*
  (lowest-LOD instanced meshes, "ideal for impostor meshes such as trees
  and foliage"), *Merged*, *Simplified* (merged then simplified to a
  screen size), *Approximated* (remeshed with baked materials), *Custom*.
  Layers chain through a parent layer, so each level switches at its own
  distance; the top layer is usually "always loaded". UE's HLOD swap is a
  hard switch unless the material opts into dithered LOD transitions.
  This is the shape we adopt.
- **OGC 3D Tiles** is the open standard for exactly this problem: a tree
  of baked proxies, each with a `geometricError` in meters, REPLACE
  refinement (children replace the parent), and screen-space-error
  traversal. **We adopt its semantics, not its format**: every node stores
  its geometric error, refinement is REPLACE, and the switch distance is
  the bake.md screen-error formula (= 3D Tiles' SSE test). The format
  (`tileset.json` + glTF/b3dm content) buys nothing here: our products
  live in the `sf_bake` store and are read through `baked://`.
- **`bevy_3d_tiles` 0.5.1** (MIT/Apache, active): a 3D Tiles renderer for
  Bevy. **Rejected**: it targets Bevy 0.19; it loads glTF/b3dm tile
  content (we have `.sfmesh` and need instanced impostors, which it
  doesn't support: no `i3dm`); it despawns tiles outside the render cut
  (entity churn, the cost we're minimizing); its traversal is per-frame
  SSE over a tileset, while our residency is the streaming scheduler's.
  Useful data point from its docs: its maintainers built a dithered LOD
  crossfade, measured it, and removed it for a plain swap. Its
  "keep the parent until children arrive" rule is our no-hole invariant.
- **Bevy 0.20, engine features used first:**
  - `VisibilityRange` does LOD switching with a *complementary* ordered
    dither: an entity leaving through its `end_margin` and one entering
    through an equal `start_margin` draw disjoint pixels (verified in
    `mesh_functions.wesl` / `pbr_functions.wesl`: the outgoing entity
    keeps threshold < 16 − k, the incoming keeps the rest). The dither
    distance is measured on the GPU from the mesh's *origin*
    (`world_from_local[3]`); `use_aabb` only changes the CPU culling
    check, so it is left off (on, CPU and GPU would disagree).
  - Async loading through the `AssetServer` (`baked://`), upload
    throttling by `RenderAssetBytesPerFrame`, GPU frustum and two-phase
    occlusion culling, multi-draw indirect, bindless `StandardMaterial`
    with vertex colors. Proxies are plain `Mesh3d` + `StandardMaterial`
    entities, so they batch with everything else.
  - Missing from Bevy: world partition, HLOD baking, impostors, light
    sprites. Those are ours.
- **Bevy virtual geometry (`MeshletPlugin`): not used.** Vulkan/Metal
  only, needs 64-bit texture atomics, position/normal/UV only (content
  relies on vertex colors; alpha-tested parts are common), no MSAA, no
  mixing forward and deferred (our raster path is forward + prepass;
  Solari is deferred). Its per-cluster LOD also doesn't remove
  per-instance entities, which is our real cost. Revisit for hero assets
  when it drops those limits.
- **Godotwind (the owner's earlier Godot attempt): an anecdote, not a
  reference.** Its costs came from Godot specifics, so none of its tiers
  or constants carry over. The general point, which UE's offline HLOD
  build already makes: far representations are baked and simplified
  offline, never merged at runtime. Distances here are starting values,
  calibrated by our own Bevy measurements.
- **Crates.** No Bevy or Rust crate does octahedral impostors or HLOD
  baking (crates.io searched 2026-10-11 for impostor, hlod, lod,
  streaming; GitHub: Godot and three.js impostor ports only). Reused:
  `meshopt` 0.6.2 (merge, weld, simplify, optimize), the `sf_bake` store,
  `sf_model`'s `.sfmesh` container and `sf_model_bevy`'s mesh decoding.
  - Impostor capture: a CPU rasterizer. `euc` 0.5.3 (Apache-2.0 AND MIT)
    was last released in 2021 and is a generic programmable pipeline; an
    orthographic capture with a depth buffer and barycentric attribute
    interpolation is ~150 lines, deterministic, and needs no GPU in the
    bake. Ours. GPU capture only if CPU misses the bake budget.
  - Texture decode/encode (for textured L1 and impostor atlases of
    textured models): `image` 0.25 (already in the tree through
    `bevy_image`; decodes PNG/TGA/BMP/DXT DDS) for decode; BC7 through
    `ctt` 0.5 (MIT OR Apache-2.0 OR Zlib, the backend of Bevy 0.20's
    `CompressedImageSaver`, C++ encoders built with `cc` like meshopt)
    when far textures land. Phase 1 ships vertex-color proxies and RGBA8
    impostor atlases; compression follows the textured L1 (below).

## Canonical technique

A **hierarchy of baked cluster proxies over the streaming grid**
(3D Tiles REPLACE refinement, UE HLOD layers), with **object LOD chains
at the leaves**, **octahedral impostors** for foliage (Ryan Brucks,
*Octahedral Impostors*, 2018; Amplify Impostors; UE's Instancing layer),
and **distant light sprites** for night skylines. Representation swaps
are Bevy `VisibilityRange` crossfades arranged so a coarser level is
always fully drawn before a finer one starts to fade out ("lead bands",
below): no holes by construction, at the price of a band where both are
drawn.

| Level | Node | Drawn (starting values) | Representation |
|---|---|---|---|
| L0 objects | streaming cell (128 m) | 0 → `d1` (~400 m) | per-object entities with their cooked LOD chain (bake.md) |
| L1 | streaming cell | `d1` → `d2` (~2 km) | merged + simplified mesh, vertex-color albedo; foliage as impostors; light sprites |
| L2 | 8×8 cells (1 km, = an index page) | `d2` → `d3` (~8 km) | L1 meshes merged and simplified again, small objects dropped; impostors for large foliage; light sprites |
| L3 | 32×32 cells (4 km) | `d3` → horizon | landmarks only, vertex-color albedo; light sprites |

Distances are measured from a node's **center** (its entity origin, see
lead bands). Each level's **error budget** is one pixel at the nearest
distance the level is fully drawn (`e = d · 2·tan(fov/2) / H`, bake.md,
1440p, 70°). Objects whose bounds radius projects under 2 px there leave
the level. Each node stores its actual geometric error (≤ budget), as 3D
Tiles does. Levels exist only as far as the world needs them: a 6 km
world never selects L3.

**Albedo in vertex colors (phase 1, every level).** A per-vertex albedo
(source vertex color × the source texture's average over the vertex's
footprint) is indistinguishable from a texture once a pixel is larger
than the texture's features: at 2 km a pixel is ~2 m. At L1's near end
(~300 m, ~0.3 m per pixel) large texture features are lost, so L1 gets a
far texture array (128² per source texture, BC7, per-vertex layer) in
phase 2, through `ExtendedMaterial<StandardMaterial, FarTextures>` (the
array index per vertex is the stated reason a custom material is needed).
Synthetic content has no textures, so phase 1 is exact there.

## Selection, residency, and crossfade

**Lead bands.** A node *N* with children *C* (objects for L1, L1 nodes for
L2, L2 for L3) switches at handoff distance `d` (its level's lower bound).
Every child ends its `VisibilityRange` at `d` (fade band `[d, d + w]`,
`w` = 10% of `d`) measured from its own origin. *N* starts its range at
`[d − o − w, d − o]` measured from its center, where `o` is the largest
distance from *N*'s center to a child origin. Whenever any child is
fading out (`d_child ≥ d`), `d_N ≥ d − o`, so *N* is already fully
drawn: there is never a hole, in either direction. The cost is a band
of width ~`o` where both levels draw; since *N*'s error is ≤ 1 px there,
the overlap is sub-pixel. If screenshots or fly-throughs show double
draws, z-fighting, or popping at handoffs, the custom fallback below
replaces it; the evidence goes here.

Evidence (Morrowind, 2026-10-11): **lead bands kept.** The double draw
costs 0.26 ms of main-pass GPU at 1440p in Balmora (6.26 vs 6.00 ms with
the leads forced to zero, RTX 4060 Laptop, ~4%). The visible artifacts first
seen at handoffs (dark screen-door patches on rocks) were not the double draw
but Bevy's depth prepass ignoring the dither (see "Bevy 0.20 pitfalls");
without the prepass the handoffs read clean (`shots/farfield/balmora_a*`,
`handoff_forest*`). Where both levels draw, a proxy is pushed behind the
geometry it replaces by a **depth offset** (`FarFieldSettings::
proxy_depth_offset_px`, 2 px of depth for L1, 1.5×/2× for L2/L3, through
`StandardMaterial::depth_bias`): with a float reverse-Z depth buffer one unit
of constant bias is ~2⁻²³ of the depth, a fixed fraction of the distance like
a pixel, so the offset is expressed in pixels and stays consistent with the
1 px error budgets. The finer level wins wherever it exists; a distant wall
sinks into the terrain by well under a pixel at grazing angles.

Why not a shared origin: Bevy's dither is complementary only between
entities measured from the same point, and a child must switch at its
own distance (CDLOD/3D Tiles: near children refine before far ones), so
no single origin serves both of a node's swaps.

**Residency (changes to streaming.md):**
- R1 (individual objects) stays a per-cell ring; its radius becomes
  `d1` + band + prefetch margin (objects past `d1` are hidden by their
  `VisibilityRange` anyway; residency past it only costs entities).
- R2 loads a cell's **L1 node** (proxy mesh, impostor batch, light batch)
  instead of "large placements at coarsest LOD". R2 is measured to the
  cell's *parent L2 node* bounds (its index page), so all children of an
  L2 node arrive and leave together.
- L2 and L3 nodes are **always resident per active worldspace** when the
  directory says their bytes fit a budget (default 64 MiB); otherwise L3
  stays always resident and L2 gets its own ring at `d3` + margin.

**L0 ends at its size-based draw distance (J1, built 2026-10-12, `.sessions/sizecull.md`).**
Each L0 chain ends at `min(d_size(r), d1)` with `d_size = K·2r / s_min · bias`
(`LodScreen::draw_distance`) and a dithered fade over its last 10 %. The rule
that keeps the lead bands hole-free: **a placement its L1 proxy represents keeps
its end at `d1` at every preset**, because culling it earlier opens a gap until
the proxy draws. Corrected from the audit: the L1 bake keeps objects whose
*diameter* projects to ≥ `min_pixels` (2) px where the node is first fully
drawn, `max(d1 − lead, d1/4)` (`FarLevels::min_radius`), so r ≥ 0.39 m at
400 m with no lead and ~0.3 m at a typical cell lead, not r ≥ 0.78 m. At High
(2 px) such an object has `d_size = K·r`, i.e. `d1 − lead` for the smallest
kept ones: without the clamp they would fade out up to `lead` meters before
the proxy fades in. So the clamp is a real rule, not a guard: the far field
reports each cell's kept radius from the loaded L1 node's lead
(`CellProxies::kept_radius`, the bake's own rule on the bake's screen, bias 1),
and streaming exempts eligible placements at or above it. Upper bound of what
it keeps (histogram, eligible placements size-culled before `d1` with
r ≥ d1/4K): Low 1937, Medium 1254, High 528, Ultra 126 of 147,897. Precombined near cells (an "L0.5" level, Fallout 4 precombines /
OpenMW active-grid paging) are evaluated as J6.

**Never a hole while loading.** The lead bands assume children are
resident. A per-frame CPU pass (`sf_lod::Residency` rules, Bevy-free)
checks each L2/L3 node whose lead band the camera is in: if any child it
covers isn't resident, the node's `VisibilityRange` start drops to 0
(fully drawn) until they are, then returns. That is the 3D Tiles / UE
"keep the parent until children arrive" rule. A child is evicted only by
the streaming rings, which are wider than the handoffs, so its parent is
already fully drawn when it leaves. Only the nodes whose state changed
are touched, so the pass costs O(nodes near a handoff).

**Custom fallback (only on evidence).** A time-based complementary
screen-door dither over 0.4 s, fade values in a per-node slot buffer
named by each entity's `MeshTag`, with statics becoming
`ExtendedMaterial<StandardMaterial, ClusterFade>` (one change in the
streaming `MaterialMapper`). Not built: the lead bands passed (above).

## Bevy 0.20 pitfalls (found on Morrowind, 2026-10-11)

Both made objects vanish at random or punched holes into crossfades, and both
were invisible on the synthetic world (too few, too small uploads).

1. **Deferred uploads leave stale instances.** With GPU preprocessing, a
   mesh instance's `MeshInputUniform` caches its mesh's vertex/index offsets
   and metadata slot when the instance is collected, and is rebuilt only when
   one of its components changes. A mesh whose upload
   `RenderAssetBytesPerFrame` defers has only a *pending* slab allocation
   (`MeshAllocator::mesh_vertex_slice` is `None`), so an instance collected in
   that window bakes `index_count = 0` (invisible) or a wrong metadata slot
   (garbled geometry) and keeps it; only a missing *material* schedules a
   re-extraction. The far field's large L2/L3 node meshes and atlases filled
   the 8 MiB/frame queue, so near objects spawned in that window disappeared.
   Re-marking `Mesh3d` changed once a mesh became resident fixed most cases
   but not all (probably the same caching for material slots), while no
   limiter fixed every run. Decision: `sf_world_bevy` leaves Bevy's limiter
   off (streaming.md, "Upload").
   *Correction (2026-10-12, spikes2/spikes4, from the 0.20 source):* the
   limiter does not defer the GPU copy. `allocate_and_free_meshes` allocates
   and writes every mesh in `ExtractedAssets<RenderMesh>::extracted` in its
   extraction frame; `RenderAssetBytesPerFrame` only defers `prepare_assets`
   (the `RenderAssets` insertion). So a burst of new meshes costs its upload
   in one render frame whatever the limiter says, and the limiter only opens
   the stale-instance window above. Upload pacing therefore has to happen at
   the source: `sf_world_bevy` caps the meshes of models loading at once
   (`StreamingConfig::max_loading_meshes`, streaming.md "Spikes").
2. **The depth prepass ignores `VisibilityRange` dithering.** A depth-only
   prepass gives opaque materials no fragment stage (`fragment_required` in
   `bevy_pbr::prepass` doesn't consider `VISIBILITY_RANGE_DITHER`), so the
   prepass writes full depth while the main pass discards the dither pixels,
   and whatever should fill them fails the depth test: dark screen-door
   holes at every LOD and far-field crossfade. The exterior camera runs
   without `DepthPrepass` (it also cost 0.4 ms there). Occlusion culling
   needs the prepass; adopting it needs this fixed first (or a
   `MAY_DISCARD`-style key for dithered meshes).

Both are upstream candidates (owner: file issues); the removal paths are in
the code comments.

## Data model

**Node keys.** `NodeKey { level: 1..=3, x, y, z }` over the streaming
partition: L1 = streaming cell (shift −1, 128 m), L2 = cell >> 3 (1 km,
the index page), L3 = cell >> 5 (4 km). Parents are integer shifts
(world-coordinates.md: every partition is a power of two of the grid).

**Node blob** (`.sfnode`, one per node, bake product): a small header
(magic, version, key, center as `WorldPos`, bounds relative to the
center, geometric error, triangle/instance/light counts, max child
offset `o`, list of impostor models) followed by:
- **mesh**: a `.sfmesh` body (`sf_model::Cooked::Static`) with one LOD
  and up to two submeshes (opaque, alpha-mask), positions relative to the
  node center (a `WorldPos`, so precision holds anywhere);
- **impostor batch**: instances (position relative to the center, yaw,
  scale, model index into the header list);
- **light batch**: points (position, linear color, intensity, radius).

**Impostor atlas** (per foliage model, bake product): hemi-octahedral
8×8 views (full octahedral when the profile allows flight), 32 px frames
(256² atlas, enough for a 10 m tree at `d1`), two RGBA8 images: albedo +
alpha and normal + depth. Plus the model's bounds, pivot and the frame
radius. The runtime builds one texture array from the atlases a
worldspace references.

**Far policy** (per model, decided at bake): `Merge` (default for
statics), `Impostor`, `Drop`. Framework default: a model whose area is
mostly alpha-masked and whose height exceeds its width becomes
`Impostor`; adapters override per model through `FarSource`, because
only the adapter knows its trees. Placements flagged `NO_LOD`,
`NEAR_ONLY`, `EDITOR_ONLY` or `DISABLED`, or of class
`Actor`/`Dynamic`/`Marker`, never enter proxies. `Light` placements feed
light batches (color, intensity, radius from `FarSource::light`; a warm
default otherwise) and merge their model like statics. `Door` statics
merge like statics.

```rust
/// Adapter hook (Bevy-free): far policy and light parameters.
pub trait FarSource: Send + Sync {
    /// Changes when any answer changes (part of every lod.* key).
    fn fingerprint(&self) -> [u8; 32];
    fn policy(&self, model: &str, info: &ModelSummary) -> Option<FarPolicy> { None }
    fn light(&self, placement: &Placement) -> Option<FarLight> { None }
}
```

## Bake steps (`sf_lod::bake`, feature `bake`)

| Step | Subject | Inputs | Output |
|---|---|---|---|
| `lod.policy` | model path | `mesh.cook` (if `Skipped`, *dynamic*: `mesh.rest`), `FarSource` | `FarPolicy` + per-material average albedo |
| `lod.impostor` | model path | `mesh.cook` (else `mesh.rest`) of an `Impostor` model | impostor atlas |
| `lod.l1` | `<ws>/<x>,<y>,<z>` (cell) | `world.page` of its page; *dynamic*: `mesh.cook` (else `mesh.rest`) + `lod.policy` of each model | L1 node blob |
| `lod.l2` | `<ws>/<x>,<y>,<z>` (node) | `world.dir`; *dynamic*: its L1 blobs | L2 node blob |
| `lod.l3` | `<ws>/<x>,<y>,<z>` (node) | `world.dir`; *dynamic*: its L2 blobs | L3 node blob |
| `lod.dir` | worldspace | `world.dir`; *dynamic*: every node's header | node directory (keys, centers, bounds, errors, bytes, impostor models) |

**Animated models in the far field (weather.md bug B2, 2026-10-12).** A
model `mesh.cook` skips because something animates (material/UV
controllers, auto-play keyframes, particles, billboards, switches) enters
the far field through its rest pose, `mesh.rest` (`sf_model::cook_rest`:
the model at its imported rest transforms; skinned models stay out). The
far steps request it only after `mesh.cook` came back `Skipped`, so only
far-eligible exterior models pay for it (21 in Morrowind: lamps, fires,
lava sparks). Its near copy stays on the content pack's runtime path, with
the placement's handoff range applied by `sf_world_bevy::SubtreeRange`
(streaming.md), so the two cross-fade like a cooked static. Blended parts
still leave L1, so a model made only of blended parts (the Ghostfence
field) gets no proxy until open question 7 lands.

L1 build (as implemented): each eligible placement's cooked **LOD0**
(measured on Morrowind: starting from the coarsest cooked level within
half the budget left 6.2M L1 triangles, LOD0 4.1M), transformed into node
space with albedo = vertex color × base color × texture average (adapter
`TextureDecoder`; alpha-masked and blended parts leave L1 in phase 1),
**welded by position and color only** (hard-edged sources duplicate
vertices per face, which would leave every face its own component),
simplified with `simplify_with_attributes_and_locks` (`ErrorAbsolute` +
`Prune`; color weight scaled to the budget), then normals are rebuilt with
a 40° crease angle and the mesh optimized. A vertex-clustering pass
(`simplify_sloppy`) was tried and never paid off: the result is error-bound.
L2/L3 merge their children's proxies the same way (hierarchical). Node
error = source error + simplifier error (an upper bound, ≤ budget). Lights
come from every `Light` placement in play, with or without a model.

**Bake budget, measured (Morrowind, 16 threads, shared machine,
2026-10-11).** Whole first bake (models, exterior index and terrain, far
field) 21.9 s; far field 20.6 s wall (L1 ~250 s CPU summed). 1,621 L1 /
25 L2 / 4 L3 nodes; L1 4.16M triangles (150 MiB), L2 1.53M (63 MiB), L3
0.37M (17 MiB); 121 impostor models (80 MiB of RGBA8 atlases); 3,031
lights. Runtime: "Measured on Morrowind" below.

## Runtime (`sf_lod_bevy`)

- `FarFieldPlugin`: registers the `.sfnode` loader, the impostor and
  light-sprite materials, the residency pass, and a streaming hook so R2
  spawns L1 nodes. Per worldspace, the node directory (`lod.dir`) loads
  with the cell directory; L2/L3 nodes load at activation.
- Each resident node is 1–4 entities (opaque, alpha-mask, impostors,
  lights) under its stream cell (L1) or a `Stationary` node entity (L2,
  L3) in the worldspace's `BigSpace`, with explicit `Aabb`s.
- Proxy materials: `ProxyMaterial = ExtendedMaterial<StandardMaterial, FarShadowExt>` (2026-10-12):
  Bevy's material with vertex colors, extended only by the terrain far shadow map (`proxy.wesl`,
  forward; weather.md "Distant shadows"), one per level. Impostors bind the same map. Shadows: L1
  casts into the cascades (to 504 m, fading out by the dithered cascade end); L2+ don't.
  Each level's material carries the proxy depth offset (`depth_bias`,
  "Lead bands" evidence).
- `ImpostorMaterial` (custom: Bevy has no impostors): one entity per
  batch; its mesh holds four vertices per instance (center, corner, yaw,
  scale, atlas layer), and the vertex shader builds the camera-facing
  quad and picks the three nearest octahedral frames (the grid triangle
  containing the view direction, barycentric weights); the fragment
  blends them, alpha-tests, and lights through Bevy's `PbrInput` /
  `apply_pbr_lighting` (as `sf_terrain_bevy` does). Prepass variant for
  depth and shadows.
- `LightSpriteMaterial` (custom: camera-facing additive points with a
  minimum screen size, faded by a night factor from the sky): one entity
  per batch.
- Counters (`sf_diag`): `far/nodes/{l1,l2,l3}`, `far/entities`,
  `far/triangles`, `far/impostors`, `far/lights`, `far/forced`,
  `far/select/ms`, `far/resident/bytes`.

## Verification

- Unit tests (`sf_lod`): node keys and parents; lead bands: on scripted
  anchor paths no point is uncovered (some level fully drawn) and the
  residency fallback never leaves a hole while children load; proxy
  error ≤ budget; node bytes deterministic across thread counts;
  octahedral encode/decode round-trips; impostor capture covers the
  model's silhouette.
- Visual (synthetic world, then bevyrrowind): level-tint debug mode
  (L0 white, L1 green, L2 blue, L3 purple) on and off, a horizon view, a
  handoff mid-band, impostor trees at the handoff distance next to the
  meshes; Morrowind: Red Mountain summit, Vivec at night from Ebonheart
  (light sprites).
- Perf (`sf_diag`, WISHLIST §0 gates): far-field CPU ≤ 0.3 ms, GPU ≤
  1.5 ms at 1440p on a 3070-class GPU; far-field entities ≤ 5,000;
  fly-through at 60 m/s with worst frame < 1.5× median.

## Measured on Morrowind (2026-10-11)

Release build, RTX 4060 Laptop + Ryzen 7 7435HS, on AC, idle machine (no
cargo/rustc), `benchmarks/exterior_*.json` (run/flight with
`--bench-ground`). "Off" = `--off farfield` (streaming's old far ring).

| scenario | far field | off |
|---|---|---|
| Balmora standing, 720p | 7.84 ms median, p99 9.6 | 20.86 ms |
| Balmora standing, 1440p | 11.87 ms (GPU-bound: 8.6 ms timed passes) | 22.2 ms |
| run 7 m/s, 720p / 1440p | 10.0 / 11.3 ms, worst 17.8 / 18.4 | 23.4 ms, worst 53 |
| flight 60 m/s, 720p / 1440p | 9.6 / 10.9 ms, worst 19.4 / 20.9 | 27.6 ms, worst 66 |
| entities, Balmora | 48.4k (far 2,317) | 353k |

- Gates: far-field CPU (`far/select/ms`) 0.14–0.17 ms mean, 0.5 ms max
  (≤ 0.3 ✓); far-field entities 2.3–3.5k (≤ 5,000 ✓); far-field GPU
  ≈ 1.2 ms at 1440p (main pass with near objects off minus terrain only,
  on a GPU below the 3070 reference: ✓); no frame over 33 ms ✓; worst
  frame vs median 1.6× (run) and 1.9× (flight) at 1440p ✗: the remaining
  spikes are streaming frames (spawn/despawn 1–3 ms, big_space 2–3 ms,
  terrain updates), not the far field. Origin shifts now cost 2–3 ms
  (big_space rewrites ~50–75k transforms instead of ~350k).
- Lead-band double draw: 0.26 ms GPU (above). Depth prepass: +0.4 ms and
  broken crossfades (pitfall 2), removed.
- **`d1` calibration** (runtime `screen.bias`, Balmora 1440p): d1 300 m →
  11.55 ms, 30.4k entities, main pass 6.54 ms (proxies shown at ~1.3 px);
  400 m → 11.87 ms, 48.4k, 6.26 ms; 600 m → 15.05 ms, 97.4k, 7.88 ms.
  **400 m stays the default** ("high"); 300 m is the "low" candidate. The
  GPU frame is led by terrain (2.9 ms of main pass in that view), then
  near objects (~2.2 ms), then the far field (~1.2 ms).
- Screenshots: `D:/Gamedev/Bevyrrowind/shots/farfield/` (`balmora_a*`,
  `redmtn_view*` (the mountain's see-through gaps with the far field off),
  `summit_vivec*` incl. night skyline lights, `ebonheart_night*`,
  `handoff_forest*` (impostor handoff), `evidence_prepass_dither_holes.png`).

## Industry comparison

J9 (industry-audit.md), 2026-10-12, docs research only (`.sessions/j9rest.md`).
"Research summary" above covers UE HLOD, 3D Tiles and Bevy; this table adds
the other engines and verdicts. "Unverified" marks claims from memory not
re-read this session.

| Technique | Industry | Ours | Verdict |
|---|---|---|---|
| Hierarchical baked proxies over a grid | UE World Partition HLOD layers (Instancing, Merged, Simplified, Approximated); Bethesda object LOD per 4/8/16/32-cell block (generated offline by the Creation Kit, extended by the community's DynDOLOD, unverified detail); RAGE SLOD1–4 + per-entity `lodDist` (GTA V); OpenMW object paging (objects merged per chunk by size); 3D Tiles REPLACE | L1–L3 baked clusters, 3D Tiles semantics | **match** |
| Precombined near statics | Fallout 4/76 precombines; OpenMW active-grid paging | none | known gap (audit row 10, J6) |
| Foliage far representation | octahedral/hemi-octahedral impostors (Brucks 2018, Amplify, UE Instancing HLOD layer); SpeedTree billboards; Bethesda flat 2-view tree LOD billboards (Skyrim; DynDOLOD adds 3D tree LOD, unverified); **UE 5.7 Nanite Foliage (experimental): assemblies of instanced parts, bone-skinned wind, distant trees as Nanite voxels** | hemi-octahedral 8×8, albedo + normal/depth, 3-frame blend, lit by Bevy PBR | **match** for raster impostors. Nanite voxel foliage is n/a (Bevy meshlets, below); it's the direction AAA foliage is moving, worth re-checking when Bevy's virtual geometry grows |
| Distant light points | GTA V "LOD lights" (city light sprites to the horizon); DynDOLOD glow windows (unverified); OpenMW none | light sprites, night factor | **match** |
| LOD transition | UE dithered LOD transition (opt-in per material), Unity LODGroup crossfade (dither), RAGE fades; 3D Tiles renderers often hard swap | `VisibilityRange` complementary dither + lead bands | **match** |
| Virtualized geometry | UE5 Nanite (Karis 2021): cluster DAG, per-cluster LOD and culling, software raster for small triangles, Nanite landscape and foliage | Bevy 0.20 `MeshletPlugin` re-checked in source 2026-10-12: still Vulkan/Metal only, needs `TEXTURE_INT64_ATOMIC`, no MSAA, no forward/deferred mixing, position/normal/UV materials; has instance BVH + cluster culling | **n/a** (unchanged reasons above; audit row 32, P2 revisit) |
| Shadows of far proxies | UE far shadow cascades + distance field shadows; Bethesda LOD casts no dynamic shadow (unverified) | L1 in cascades, terrain far-shadow map beyond | **match** |
| Size culling of small objects in proxies | UE HLOD "minimum screen size" per layer; OpenMW min size | 2 px at the level's near end | **match** |

No new gap: the open differences are audit rows 10, 12, 16 and 32.

**Sources** (checked 2026-10-12, beyond "Research summary"):
- UE, Nanite Foliage (5.7, experimental): https://dev.epicgames.com/documentation/en-us/unreal-engine/nanite-foliage
- UE 5.7 feature coverage (assemblies, skinning, voxels): https://80.lv/articles/check-out-new-features-arriving-with-unreal-engine-5-7 and https://tomlooman.com/unreal-engine-5-7-performance-highlights/
- Bevy 0.20 `bevy_pbr/src/meshlet/mod.rs` (`MeshletPlugin` docs), local registry.
- OpenMW object paging: settings docs, cited in industry-audit.md.

## Open questions

1. Handoff distances `d1`–`d3`: d1 calibrated (400 m high, 300 m low
   candidate, above); d2/d3 only matter once views reach 2 km+ in
   bigger worlds (Vvardenfell is ~5 km across: L3 never selects).
2. Streaming spikes (1.6–1.9× median): Bevy re-extracts every
   `VisibilityRange` whenever one changes (`extract_visibility_ranges`
   walks all of them), and spawn/despawn run in the same frames. Levers:
   fewer ranges per object (share one range per LOD chain shape), spawn
   batching, or an upstream incremental extraction.
   Measured 2026-10-12 (streaming.md, "Spikes"): the re-walk costs ~1.1 ms of
   extraction per spawn/despawn frame with ~40k ranges; the larger spikes were
   origin shifts (Bevy re-collects every instance), particles re-uploading
   their meshes every frame, mesh-add bursts and terrain respawns. Particles,
   terrain and an origin-shift guard are fixed (flight: frames over 1.5×
   median 32 → 9, worst 1.67–1.86×); the re-walk needs Bevy:
   `RenderVisibilityRanges` keeps its fields private, so the extraction can't
   be replaced from outside (third upstream candidate, with "Bevy 0.20
   pitfalls").
3. Tree impostors inside L0 (a last level of the object's own chain) if
   trees dominate L0 cost. Measure first.
4. Approximated (remeshed + texture-baked) L1 proxies if merged +
   simplified meshes are too heavy in dense towns. Measure first.
5. ~~L1 bake threshold vs size presets~~ (closed by J1, 2026-10-12): the bake
   keeps diameters ≥ 2 px at `max(d1 − lead, d1/4)`. Objects it drops are
   under ~2 px at the handoff; at Ultra (1 px) they now fade out at their own
   `d_size ≤ d1` instead of popping at `d1`. No rebake for a preset.
