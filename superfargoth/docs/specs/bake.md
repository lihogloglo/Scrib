# Bake pipeline

Status: agreed (2026-10-11)
Crates: `sf_bake` (new, Bevy-free: step graph, cache, CLI helpers),
`sf_model` (new, Bevy-free: neutral model and the `.sfmesh` container),
`sf_model_bevy` (new: `.sfmesh` asset loader and the `baked://` asset source)

## Goal

One pipeline for every expensive derived asset, with content-hash caching
per mod set. It is incremental (adding a mod rebuilds only what the mod
touches), deterministic, runnable from a CLI or at game start, and never
blocks streaming. This settles WISHLIST early decision 6.

**Budget (owner's rule: bake once, like a "compiling shaders" screen).**
The first bake of a full profile, *every* product through M7 included,
targets **≤ 2 min, with a 5 min ceiling**, on the reference machine
(8-core CPU, RTX 3070-class). After that, a start bakes nothing. A mod
change re-bakes only what it touches (seconds), and a game update re-bakes
only the steps whose version changed. A step that would break the budget
moves to the GPU or gets cheaper; the budget doesn't move. The CLI's
per-step timings are an `sf_diag` gate, and a breach is a bug.

Baking exists to make the runtime faster and prettier. Anything that costs
frame time at runtime and can be precomputed within the budget is
precomputed.

M2 products: model info, mesh LODs, the world index, and terrain tiles.
HLOD clusters, far textures and impostors (far-field.md, moved into M2
2026-10-11). Later: PBR maps and texture compression (M7),
erosion (M7), the shoreline field (M6), navmesh (M10).

Non-goals: shared or distributed caches (determinism keeps that possible),
live re-bake in an editor.

## Canonical technique

- A **build system with constructive traces and dynamic dependencies**
  (Mokhov, Mitchell, Peyton Jones, *Build Systems à la Carte*, ICFP 2018:
  the Shake/Bazel model), over a **content-addressed object store** (as in
  git and Bazel's CAS). Steps declare their known inputs up front and may
  read more while running. The trace records both.
- **Mesh LODs: quadric-error simplification** (Garland & Heckbert 1997)
  via [meshoptimizer](https://github.com/zeux/meshoptimizer), with
  attribute-aware simplification plus vertex-cache, overdraw, and
  vertex-fetch optimization. LODs are selected by projected screen-space
  error.

**Reuse check (2026-10-11).** No Rust crate provides persistent,
trace-based incremental builds as a library. `salsa` and `comemo` memoize
in memory only. `cacache` is a content-addressed disk cache, but it is
SHA-based, async-first, and last released in 2024. We reuse the pieces:
`blake3` (already in the tree through `bevy_asset`), `lz4_flex`,
`postcard`, and `rayon`. The step graph itself is about 500 lines of our
own code.

**Deviation: Bevy's `AssetProcessor` is not the bake engine.** It maps one
source file to one processed file, keyed by that file's hash. Our products
have many inputs (records, models, neighboring tiles) and must be keyed by
all of them. The core must also run without Bevy (CLI, tests, compile time),
and we want one cache, not two. Bevy stays the **loader** of our products,
through an asset source. Revisit it for 1:1 texture conversion in M7: Bevy
0.20's `CompressedImageSaver` now writes BCn with full mips.

## Data model

**Steps.**

```rust
pub trait BakeStep: Send + Sync {
    const ID: &'static str;          // "mesh.cook"
    const VERSION: u32;              // bump to invalidate every product of this step
    type Subject;                    // e.g. a model path, (worldspace, tile)
    fn plan(&self, cx: &PlanCx, s: &Self::Subject) -> Inputs;               // cheap, static deps
    fn run(&self, cx: &mut RunCx, s: &Self::Subject) -> Result<Output, BakeError>; // reads via cx = dynamic deps
}
```

**Inputs and fingerprints:**

| Input | Fingerprint |
|---|---|
| VFS file (the winning file for a path) | blake3 of the bytes, memoized by a stat key (source, path, size, mtime; archive entries by the archive's stat plus entry offset). It is re-hashed only when the stat changes, as in git's index and ninja's restat. |
| Record (kind, id) | A per-kind fingerprint function registered with the kind. Native kinds use serde bytes. Adapters may hash their raw source bytes of the winning version plus its patches. A kind without one can't be a bake input. |
| Product | its object hash |
| Params | quality tier and settings that change the output |

**Key** = blake3(step id, version, subject, params, sorted input
fingerprints). A **trace** maps (step, subject) to the static key, the
dynamic inputs read during `run` with their fingerprints, and the output
object hash. It is a hit when the static key matches and every dynamic
input still has its recorded fingerprint.

**Store**, under a game-chosen cache root (never inside a mod folder):
- `objects/ab/cdef…`: content-addressed by blake3 (already in the tree
  through `bevy_asset`), compressed with LZ4 (`lz4_flex`, pure Rust, so no
  C build) unless the format is already compressed. LZ4 is chosen for
  decode speed (GB/s per core) over ratio, because streaming reads these
  objects while the game runs;
- `traces/<step>/<subject hash>`: one small file per trace, written by
  atomic rename;
- `profiles/<profile>.manifest`: a (step, subject) → object table per mod
  set, written at the end of a bake and loaded once at startup.

Objects are shared across profiles, so switching back to a profile is
instant. `gc` keeps everything reachable from any manifest, then evicts by
LRU down to a size cap (default 10 GiB).

**Authored beats derived.** A step may declare an override path (for
example `lod/<model path>/<n>.sfmesh`). If the VFS has a file there, that
file *is* the product, fingerprinted like any input. Mods can ship
hand-made LODs.

**`sf_model`** (Bevy-free). It is the neutral form that importers produce
and that steps and the runtime consume:
`Model { nodes (name, parent, transform, mesh, LodPolicy, animated),
meshes (streams: positions, normals, UVs, colors, joints/weights; indices;
material), materials (descriptors) }`.
`.sfmesh`: a little-endian container holding a header (magic, format
version), a LOD table (absolute error, index and vertex ranges, submeshes),
a vertex layout, and meshopt-encoded vertex and index streams (they decode
at GB/s). `CookedModel::to_obj` writes any `.sfmesh` (all LODs side by side) as OBJ for
inspection.

```rust
/// Adapters import legacy formats; the framework ships glTF (§6.1: modern formats win).
pub trait ModelImporter: Send + Sync {
    fn extensions(&self) -> &[&str];
    fn import(&self, path: &VfsPath, bytes: &[u8], cx: &mut ImportCx) -> Result<Model, ImportError>;
}
```

## M2 steps

| Step | Subject | Inputs | Output |
|---|---|---|---|
| `model.info` | model path | model file, importer version | bounds, triangle/vertex counts, animated/skinned flags |
| `mesh.cook` | model path | model file | `.sfmesh`: LOD0 plus the LOD chain, for models without animated parts |
| `world.index` | worldspace, 1 km page | region fingerprints (`WorldSource`), `model.info` of referenced models, ring config | index page (streaming.md) |
| `terrain.tile` | worldspace, level-0 tile | overlapping source tiles plus neighbors (dynamic), footprints from `world.index` | tile (terrain.md) |
| `terrain.pyramid` | worldspace, level-k tile | four child tiles | tile |
| `terrain.macro` | worldspace, level-k tile | tile, layer average colors | macro albedo |
| `terrain.layers` | profile | all `TerrainLayer` records and their textures, quality tier | KTX2 texture arrays |
| `terrain.rt_proxy` | worldspace, tile | tile | static mesh for Solari |

## Cooked meshes and LODs (`mesh.cook`)

**Cooking LOD0 too.** Models without animated parts (most statics) are
cooked whole, so streaming never parses a legacy format at runtime: it
decodes `.sfmesh` at GB/s on the task pool. Models with controllers or
particles are skipped (`Cooked::Skipped`) and keep the runtime importer
path; models with skins or morph targets cook to `Cooked::Skinned` (M3, see
"Skinned and morphed models" below). Flattening
and merging per material also cut draw calls, so LOD0 gets cheaper than the
source.

1. **Flatten.** Static nodes are transformed to model space and merged
   per material (mirroring transforms get their winding restored). The
   importer sets each node's `LodPolicy` (`Simplify`, `Keep`, `Drop`). Any
   animated node makes a model without skins or morphs skip cooking (see
   above). Importers leave collision geometry out of render meshes.
2. **Simplify.** Weld (`generate_vertex_remap`), then each level is
   simplified *from LOD0* with `simplify_with_attributes_and_locks`
   (normal, UV, and color weights; `ErrorAbsolute`), targeting half the
   previous level's triangles. The error cap is 1% of the bounding radius
   for LOD1, doubling per level. `Keep` nodes are vertex-locked; `Drop`
   nodes leave after LOD0; open borders are *not* locked by default (the
   error is sub-pixel at each switch, so seams between modular pieces are
   too, and locking froze legacy content made of open pieces: measured on the
   first real bake, most chains stalled at one level); `Prune` removes small disconnected bits from LOD2 on. If a level
   stalls, `simplify_sloppy_with_locks` makes one final level.
3. **Stop** at < 25% reduction, when the previous level is < 128
   triangles, or at 5 levels. Each level stores its absolute error in
   meters, never decreasing along the chain.
4. **Optimize** each level: `optimize_vertex_cache`, `optimize_overdraw`
   (threshold 1.05), `optimize_vertex_fetch`. Then encode.
5. **Runtime distance**: `d = e · H / (2 · tan(fov/2) · τ)` with screen
   height `H` and pixel threshold `τ` (1 px at the preset's reference 1440p
   and 70°), times a global LOD bias. The spawner writes it to
   `VisibilityRange` (crossfade margin 10% of d), clamped to the streaming
   distance table, and recomputes it when settings change.

**Binding**: [`meshopt`](https://crates.io/crates/meshopt) **0.6.2**
(MIT/Apache, compiles the C++ library with `cc`). Bevy 0.20's meshlet
feature depends on the same version, so there is one copy in the tree. It
vendors a pre-1.0 meshoptimizer: attribute-aware simplification with locks,
sloppy simplification, cluster building and partitioning. The v1.x
additions (clamped-error simplification, `simplifyWithUpdate`, the
`clusterlod` hierarchy) arrive with `meshopt` 0.7, which is on the repo's
main branch but not on crates.io (2026-10-11). Pure-Rust ports such as
`meshopt-rs` stopped in 2022.

**Format choice (reuse check).** Cooking to `.glb` would reuse Bevy's glTF
loader, but `bevy_gltf` supports neither `EXT_meshopt_compression` nor
`KHR_mesh_quantization`, so GLB would lose the codec's size and decode
speed. Bevy 0.20's `SerializedMesh` is documented as "for short-term
transfer", not a stable on-disk format. Hence `.sfmesh`. Its Bevy loader
builds plain `Mesh`es and can apply Bevy's own `Mesh::compressed_mesh`
vertex compression for GPU memory.

**Measured (2026-10-11):** about 4 ms to cook a 2,000-triangle prop with its
full chain on one thread. That projects to seconds for a full content pack
across 8 cores, a small slice of the 2-minute budget.

### Skinned and morphed models (M3, as built 2026-10-12)

**Source model.** `MeshData::skin: Option<MeshSkin>`: a palette of
`SkinJoint { node, inverse_bind }` (column-major 4×4, glTF
`inverseBindMatrices`) and per-vertex `(palette index, weight)` lists of any
length. `MeshData::morph_targets`: dense position (and optional normal)
deltas, a name and a default weight. Skinned positions are in bind space and
the mesh node's transform is ignored (the glTF rule). `skinned: true` with
no `skin` still means "skinned, not readable yet": the model is skipped as
before, so an importer can flag skins before it reads them.

**Cook** (`sf_model::skin`). A model with any skin or morph target cooks to
`Cooked::Skinned(CookedSkinned)`:

1. **No flatten, one group per source mesh** (a *part*, with its node path
   root first), so a runtime can apply `BodyPart::node_filter` before it
   merges parts per material (character.md, assembly step 3). Skinned
   meshes stay in bind space. **Rigid meshes become single-joint skins**:
   flattened to model space at rest and bound with weight 1 to the nearest
   node at or above them that moves (a skin joint or an animated node, else
   the root), inverse bind = inverse of that node's rest transform; mirrored
   transforms get their winding restored as in the static path.
2. **Palette.** One model-wide palette of `CookedJoint { name, inverse_bind }`,
   deduplicated by (node, inverse bind). A joint bound in two poses gets two
   entries (exact, where re-binding to one pose would change deformation).
3. **Influences**: repeated joints summed, non-positive weights dropped,
   the four largest kept (largest first) and renormalized to sum 1; unused
   slots are joint 0, weight 0. A vertex with no influence binds fully to
   its mesh's first joint.
4. **Weld** on every stream (vertex, skin, all morph deltas), then the same
   LOD chain as static models, simplified *from LOD0* in bind space. Extra
   simplifier attributes: the vertex's **joint center** (its weights' blend
   of the bind-space joint origins, weight 0.5 per meter) so collapses across
   hard joint boundaries cost about half the joint distance while smooth
   blends still simplify, and its **largest morph displacement** (weight
   1.0) so morphing regions keep vertices longer. Every level indexes a
   subset of LOD0's vertices, so weights stay normalized and joint indices
   valid by construction; per-level vertex fetch order is applied to every
   stream.
5. **Streams** per submesh (`CookedSkinStreams`, parallel to
   `CookedModel::lods[l].submeshes[i]`): `SkinVertex { joints: [u16; 4],
   weights: [f32; 4] }` and target-major `MorphDelta { position, normal }`
   blocks for the submesh's own target list only ("Morph layout" below),
   both meshopt-encoded. `bounds` are the rest pose.

**Rigid body parts** (`PartKind::Rigid { joint, offset }`):
`CookedSkinned::rigid(&cooked, joint, offset)` turns any cooked part
(static or deforming) into a single-joint skin with inverse bind = `offset`
and flips the winding when `det(offset) < 0`. The mirror is owned here, in
data, not by the renderer: Bevy 0.20 culls back faces with a fixed front
face (`MeshFlags::SIGN_DETERMINANT_MODEL_3X3` only fixes tangent frames), so
a negative-scale `Transform` would show the back faces.

**Format v2 → v3.** v2 added `Cooked::Skinned` as a new trailing enum
variant, so every v1 body is a valid v2 body. **v3 (M3-a, 2026-10-12)**
changed `Skinned`'s morph layout (per-submesh target lists), so v2 is
*retired*: `RETIRED_FORMAT_VERSIONS = [2]`, readers accept 1 and 3, a v2
file is a `FormatError::Version(2)` and re-cooks. Files carry the lowest
version that holds them (static and skipped cooks stay v1, byte-identical),
and a skinned body under a v1 header is rejected. `KEY_FORMAT` went 1 → 2
because the same skinned input now cooks to different bytes: the next bake
re-runs every `mesh.cook` / `mesh.rest` product once (≈ 38 s full model
re-cook). Importer versions are unchanged (the NIF importer's `Model` output
is identical: its targets take the default `MorphKind::Expression`, now set
explicitly). The glTF importer moved to version 2 with skins; an adapter
that starts emitting skins must bump its importer version.

#### Morph layout (M3-a, as built 2026-10-12)

Gap G1 (industry-audit.md): Bevy 0.20 stores morph targets densely, one
48-byte `MorphAttributes` (position, normal, tangent deltas, each padded to
16 bytes) per vertex of the whole mesh per target, and its vertex shader
loops over every target for every vertex in every pass. So:

- **`MorphTarget::kind: MorphKind`**, set by the importer (adapter):
  `Expression` (default; animated at runtime: blink, mouth, visemes) or
  `Shape` (per-instance slider that never animates). The NIF adapter marks
  every geometry morph `Expression` (all keyframed); glTF has no flag and
  keeps the default. `MorphTargetInfo` carries the kind.
- **Per-submesh target lists.** `CookedSkinStreams::targets: Vec<u16>`
  (ascending indices into `CookedSkinned::morph_targets`) names the targets a
  submesh carries; `morphs` holds exactly one block per entry, target-major,
  and is `None` exactly when `targets` is empty. No zero blocks.
- **All-zero stripping.** A target is carried by a submesh only if some
  position or normal delta component of its vertices exceeds
  `MORPH_EPSILON = 1e-6` (catches importer noise only). Targets no submesh
  carries are removed from the model list (and the lists renumbered).
- **LOD rule.** `Expression` targets are kept on LODs `0..RUNTIME_MORPH_LODS`
  (= 0 and 1) only; `Shape` targets stay on every level, because assembly
  bakes them (a far LOD would otherwise lose the character's face shape).
- **Shared helpers.** `CookedSkinned::decode_submesh` → `DecodedSubmesh`
  (vertices, indices, skin, targets, deltas); `DecodedSubmesh::bake_shapes`
  adds `weight × delta` of every `Shape` target into the vertices and removes
  it (position and normal; normals stay unnormalized as in Bevy's
  `morph_vertex`, skinning normalizes); `apply_block` is the runtime formula.

**Bevy mapping** (`sf_model_bevy`, built 2026-10-12: `skinned.rs` maps,
`spawn.rs` spawns; per submesh = one `Mesh`):

| Cooked | Bevy 0.20 |
|---|---|
| `Vertex` position, normal, uv, color | `ATTRIBUTE_POSITION` `Float32x3`, `ATTRIBUTE_NORMAL` `Float32x3`, `ATTRIBUTE_UV_0` `Float32x2`, `ATTRIBUTE_COLOR` `Float32x4` (u8 / 255, only if `has_colors`) |
| `SkinVertex::joints` / `weights` | `ATTRIBUTE_JOINT_INDEX` `Uint16x4` / `ATTRIBUTE_JOINT_WEIGHT` `Float32x4` |
| `joints[*].inverse_bind` | one `SkinnedMeshInverseBindposes` (`Mat4::from_cols_array`) per model, shared by its submeshes; `joints[*].name` → skeleton joint entities in `SkinnedMesh::joints` (same order) |
| `DecodedSubmesh` after `bake_shapes(default weights)` | the base vertices (a plain model bakes `Shape` targets at their default weight; a character's are baked by `sf_character::assemble` at the recipe's weights) |
| remaining `targets` (all `Expression`), `morphs` | `Mesh::set_morph_targets(Vec<MorphAttributes>)`: for block `k`, vertex `v`: `morphs[k·n + v]` → `MorphAttributes { position, normal, tangent: ZERO }` (target-major, the order `bevy_gltf` builds); `set_morph_target_names` from `morph_targets[targets[k]].name` |
| weights | one model-wide weight vector (by target name, driven by `sf_anim` curves); each morphing mesh entity gets `MeshMorphWeights::Value` with its own targets' weights gathered through `targets` (`MeshMorphWeights::Reference` only works when a mesh carries the full list in order, as glTF primitives do) |

No morph data on a submesh with empty `targets` (never call
`set_morph_targets`). Limits checked at load: 256 joints per `SkinnedMesh`
(`MAX_JOINTS`), 256 targets per mesh (`MAX_MORPH_WEIGHTS`), and
`vertices × 9 ≤ 2048²` per target layer when Bevy falls back to morph
textures (`MorphTargetImage::new`); above any of them the load fails with
`SfMeshLoadError::Limit` (no silent truncation; splitting is future work).

**As built (skinloader, 2026-10-12).** The loader returns the same
`CookedModelAsset` with `skin: Some(CookedSkin)` (palette names, one
labeled `inverse_bindposes` asset, morph target infos, per-mesh target
lists); static consumers (`sf_world_bevy` streaming, `mw_assets` cooked
spawn) treat a skinned asset as uncooked. `spawn_skinned(root, asset, lod,
joints, material)` copies `bevy_gltf` 0.20's spawn (`loader/mod.rs`): mesh
entities with `Mesh3d`, `SkinnedMesh { inverse_bindposes, joints }` and
`DynamicSkinnedMeshBounds`, joints resolved by name (`resolve_joints`
reports every missing one). Findings from Bevy 0.20's source:
- **Bounds.** `bevy_gltf` defaults to `GltfSkinnedMeshBoundsPolicy::Dynamic`:
  `Mesh::generate_skinned_mesh_bounds` stores per-joint bind-space boxes
  and `DynamicSkinnedMeshBounds` refits the entity `Aabb` from the joint
  transforms every frame, so a posed character is never culled from its
  bind-pose box. We do the same. The boxes are generated **before** Bevy's
  vertex compression (they read `Float32x3` positions) and survive
  `take_gpu_data`, so GPU-only meshes keep them. They ignore morph
  displacement (heads move millimeters; fine).
- **Batching.** On storage-buffer platforms (desktop) skinned and morphed
  meshes batch and go through multi-draw indirect like static ones
  (`no_automatic_skin_batching` / `no_automatic_morph_batching` only add
  `NoAutomaticBatching` on uniform-buffer platforms, WebGL 2). So the
  per-material merge in `sf_character::assemble` pays off; nothing in the
  mapping needs to change for MDI.
- **Compression.** `MeshCompressionArgs::regular()` also quantizes joint
  weights to unorm16; joint indices stay `Uint16x4`. Same position-compression
  scaffold as static meshes (`compression_args`).
- **Morph weights.** `MeshMorphWeights::Value` is read directly by
  `extract_morphs`; the model-wide vector lives in `ModelMorphWeights` on the
  model root and `sync_morph_weights` (PostUpdate) gathers it per mesh. That
  component is the seam `sf_anim_bevy` curves will write into.
- **Skeleton.** `spawn_rest_skeleton` (named scaffold) spawns an
  `sf_model::Model`'s node tree as named joint entities at rest; it stands in
  for `sf_anim_bevy`'s rig spawn. The app's `--skinned` view instead binds to
  the NIF runtime scene's named nodes.
- **Ray tracing.** No Solari variants for skinned models (Solari 0.20 traces
  static BLASes only).

**Reuse check.** Bevy: its skinning and morph runtime is used as is (the
mapping above); `bevy_gltf` does import skins, but only at runtime and
into Bevy types, which the Bevy-free cook can't use. Crates: the `gltf`
crate's reader (all `JOINTS_n`/`WEIGHTS_n` sets, inverse binds, morph
targets with sparse accessors expanded; `extras` feature for
`targetNames`, as `bevy_gltf` does) and meshopt 0.6.2's attribute-weighted
simplifier and codecs. Our own: the palette merge, the rigid → single-joint
conversion, the multi-stream weld and fetch remap (meshopt's
`optimize_vertex_fetch` reorders one stream; 0.6.2's `optimize_vertex_fetch_remap`
returns a remap truncated to the unique count, unusable as a full remap).

**Industry comparison.** glTF 2.0 is the data model (4 influences per set,
inverse bind matrices, dense or sparse morph targets). UE's skeletal mesh
reduction also simplifies from a base LOD, remaps morph targets onto
reduced LODs, offers "Enforce Bone Boundaries" (penalize collapses between
vertices with different major bones; our joint-center attribute is a
continuous version) and per-LOD "Max Bone Influences" and bone removal. We
skip influence reduction: Bevy's skinning always reads four weights, so
fewer would save nothing on the GPU, and joint LOD belongs to the
animation tiers (animation.md A0–A3). Not done: tangent morph deltas (the
cook keeps no tangents), per-target simplifier attributes (only the largest
displacement is weighted). **Gap (J9, 2026-10-12), closed by M3-a the
same day**: the zero blocks "where a part lacks a target" would have become
real GPU memory once parts are merged, since Bevy 0.20 stores morphs densely
(48 bytes per vertex per target) and loops over every target per vertex in
every pass; UE's merge drops morphs and Mutable bakes them. Now: per-submesh
target lists, all-zero stripping, expression targets only on LOD 0–1, shape
targets baked at assembly ("Morph layout" above; character.md
"Morph-bearing parts").

## Execution

- **Planner.** `bake`/`product` first plan every unfinished product
  reachable through planned product inputs (each planned once), then run
  them on rayon with a dependency-counting scheduler: a product starts when
  its last planned dependency finishes (cargo's job queue; Bazel and Buck2
  likewise never park a worker on an unfinished dependency). Dynamic reads
  (`RunCx::product`) and dynamic inputs in a trace resolve inline. Cycles
  are errors (found inline, by the chain of products being built), and a
  failed product doesn't stop unrelated ones. Per-step memory caps are not
  implemented yet; add them when a step's peak memory calls for it.
- **Concurrency (rule, 2026-10-12).** Nothing in a bake blocks on work that
  runs on the rayon pool. A rayon worker waiting in a `join`/`par_iter`
  steals unrelated jobs onto its own stack, so if it holds a lock or a
  `OnceLock` initializer, a stolen job that waits on that same cell (or on
  a thread waiting on it) can never finish: the deadlock `animcook` hit in
  2 of ~6 real anim bakes, reproduced by `sf_bake/tests/concurrency.rs`
  (old code: a worker with three nested `get_or_init` initializers between
  `join_context` frames, under a stolen top-level job). Rayon's own
  guidance is the same: don't hold locks across parallel calls. So:
  - A product's slot is never waited on. A thread that needs a product
    another thread has claimed but not finished builds it itself; steps
    are deterministic, so the first published result is the result.
    `StepStats::duplicates` counts these; nonzero means a dependency was
    read dynamically while still in flight, and the host should bake it in
    an earlier `bake` call (as `mw_bake` does: clips, then graphs).
  - Inside a build, inputs are fingerprinted sequentially (scheduled
    products are already finished), so sf_bake opens no steal window while
    a product is in progress. Steps may still use rayon inside `run`
    (terrain layer arrays do): what they steal never blocks.
  - Step-side memos (`AnimBake` imports, terrain plans, world regions) may
    block only if their initializer is plain sequential code: no rayon, no
    baker calls. A memo whose initializer needs parallelism must compute
    outside the lock and publish (`OnceLock::get`, compute, `get_or_init`
    with the value), never wait.
- **Determinism.** Same inputs give bit-identical outputs: no hash-map
  order in outputs, fixed-order reductions, no dependence on thread count,
  no timestamps. `verify --sample N` re-runs N cached products and compares
  them.
- **CLI.** The game binary provides it, because adapters must be wired:
  `<game> bake [--profile P] [--step S] [--force] [--verify N] [gc]`, with
  progress and per-step timings. `sf_bake` supplies the subcommand.
- **Game start.** On first start (or after a mod change) every product
  bakes behind one progress screen, within the budget above. Nothing bakes
  during play: background work would cost frame time. A product missing
  for any reason (bake cancelled, step failed) falls back to the runtime
  importer and full LOD0, and is reported, never silently baked mid-game.
  Streaming never waits on the bake.
- **Pipeline warm-up** belongs on the same screen: render pipelines that
  streaming will need (terrain, cooked-mesh material variants) are compiled
  there, not on first sight, so new content never hitches mid-flight.
  Whether Bevy 0.20 can persist its pipeline cache to disk needs checking
  during implementation. If it can, the warm-up is also a one-time cost.
- **Cold start** (WISHLIST §0: < 10 s after the first bake): load the
  manifest and validate traces through stat keys. Nothing is content-hashed
  unless a stat changed.
- **Loading.** `sf_model_bevy` registers a `baked://` Bevy asset source
  that resolves (step, subject) through the profile manifest. It is not an
  `sf_vfs` layer: products are content-addressed objects named by
  (step, subject), not files in the mod namespace, and a re-bake swaps the
  manifest without rebuilding the VFS index.

## Verification

- Unit tests (`sf_bake`, 13 passing as of 2026-10-11): a fresh session hits
  everything; an invalidation matrix (file, record, parameter, version)
  re-runs exactly the dependents; unchanged product bytes stop propagation;
  dynamic reads invalidate when they change; cycles are errors; authored
  overrides win and un-override cleanly; failures don't poison other
  products; gc keeps referenced objects, and collected ones are rebuilt;
  stat memo reads only new stats. Writes are temp-file + rename, so an
  interrupted write never leaves a partial object.
- Determinism: baking with 1 and N threads produces identical manifests.
- Incremental: a test mod that touches one model re-runs only that model's
  `model.info`/`mesh.cook` and the index pages that reference it. The CLI
  prints the counts.
- Budget gate: the full first bake on the reference machine finishes in
  ≤ 2 min (ceiling 5 min), with per-step timings recorded in `sf_diag`.
- LOD quality: the CLI reports error and reduction per level. An
  LOD-colored debug view and crossfade screenshots in the viewer;
  `to_obj` exports for side-by-side checks in Blender.
- Timings recorded as a baseline: full first bake of the first content
  pack, and warm-start validation (target < 1 s).
- Dev-loop gates (2026-10-12, Morrowind: 26k products, 16 threads): cold
  bake ≤ 2 min (measured 47.7 s), no-op (all hits) bake ≤ 2.5 s (measured
  1.75 s, of which trace validation is ~0.4 s; the rest is the adapter's
  content load, world source and report). Per-step numbers:
  `bevyrrowind/docs/specs/bake-adapter.md`. Known slack: step CPU covers
  only about 1.4x the wall time on 16 threads in the model and far-field
  phases, so time outside `Step::run` (input fingerprints, store writes,
  per-level barriers) dominates a cold bake.

## Decisions made (2026-10-11, owner's priorities: speed, visuals, bake once)

- Cook LOD0 for models without animated parts (above). Animated models
  follow once `sf_model` covers controllers and particles.
- One file per trace, written by atomic rename. Switch to an embedded
  database only if file counts measurably slow down validation.
- LZ4 (`lz4_flex`) for stored objects.
- `meshopt` 0.6.2 now. Move to 0.7 when it reaches crates.io and Bevy
  moves with it; the API we use carries over.

## Industry comparison

J9 (industry-audit.md), 2026-10-12, docs research only (`.sessions/j9rest.md`).
The skinned/morph part has its own comparison above ("Skinned and morphed
models"); this covers the pipeline as a whole. "Unverified" marks claims
from memory not re-read this session.

| Technique | Industry | Ours | Verdict |
|---|---|---|---|
| Derived-data cache keyed by all inputs + tool version | UE Derived Data Cache (keys from source hash + format/version GUIDs; local, shared and Zen-server backends); Unity Asset Database v2 (import artifacts keyed by dependency hashes, Unity Accelerator as a shared cache); Bazel/Buck2 CAS | trace-based steps over a content-addressed store, step versions in keys | **match** |
| Who bakes | UE/Unity: developers cook, players download cooked data; Bethesda: mod authors run offline tools (Creation Kit precombines/previs, LOD generation, FaceGen; xLODGen/DynDOLOD for LOD); OpenMW: no bake (runtime loading), except an optional navmesh disk cache (`openmw-navmeshtool`) | the player's machine bakes once per mod set, ≤ a few minutes | **differ, by design**: a mod list changes the world, so a per-load-order bake replaces the Bethesda "every LOD/precombine mod ships its own generated files" conflict problem. Shared/prebuilt caches (UE shared DDC) are unnecessary for one player |
| Mesh LOD generation | QEM simplification: UE's built-in reduction (and Simplygon in many AAA pipelines); Unity 6 automatic Mesh LOD (meshoptimizer-based, unverified); meshoptimizer is the common open choice | meshopt, attribute-aware, absolute error per level | **match** |
| Vertex/index optimization + compression | meshoptimizer vertex cache/overdraw/fetch + `EXT_meshopt_compression` / `KHR_mesh_quantization` (glTF); UE Nanite/static mesh compression | same, in `.sfmesh` | **match** |
| Texture cooking (BCn + mips, per-platform) | universal (UE texture compression settings, Unity import platforms, BC7 for color, BC5 for normals) | terrain arrays and far textures cook BC7/BC5; object textures load as shipped (M7: `CompressedImageSaver` route) | **partial**, already planned (M7); no new job |
| Shader/pipeline precompile | UE PSO precaching + bundled caches; Unity variant collections / PSO tracing | warm-up screen planned, not built | known gap (audit row 22, J7) |
| Packing many small products | UE pak/IoStore containers, Zen store; Unity `Library/Artifacts` with an index; git pack files | one file per object (open question 2) | **differ**; fine until file counts hurt (question 2) |
| Deterministic, parallel builds | Bazel hermeticity; UE/Unity parallel importers (Unity parallel import, unverified scope) | rayon steps, determinism tests, no blocking on in-flight work ("Concurrency") | **match** |

No new gap: the differences are planned (M7 texture cooking), known (J7) or
deliberate (per-player bake).

**Sources:** UE Derived Data Cache docs
(https://dev.epicgames.com/documentation/en-us/unreal-engine/derived-data-cache-in-unreal-engine,
not re-read); Unity Asset Database and Accelerator manual pages (not re-read);
OpenMW `openmw-navmeshtool` docs (not re-read); meshoptimizer README
(cited above). All four from memory, unverified on detail.

## Open questions

1. The pipeline-cache persistence check above (implementation detail, not
   a design choice).
2. Small-file I/O: the first real bake writes ~26k objects and traces (temp
   file + rename each), and on Windows with Defender the cold bake is
   I/O-bound (6 s of step CPU in a 42 s single-thread wall). Fine for the
   budget today. If terrain and HLOD push the count up, pack objects into
   append-only pack files (git-style) and batch trace writes.
