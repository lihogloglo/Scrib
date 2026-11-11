# bake history

Dated or survey blocks moved verbatim from [bake.md](../bake.md) (doc cleanup S12c, 2026-10-26).

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


## M2 steps (moved 2026-11-11, bake-policy; superseded by bake.md "Inventory")

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

## De-lighting as built (moved 2026-11-11, bake-policy)

Product (D-2b built, `mw_bake::delight`): `texture.roles` per model (NIF material slots: albedo, glow,
detail/dark, emissive = base glowing through emissive colour/vertex colours/unlit, effect = particles, additive
and `meshes/e/`, translucent = blend that isn't a cutout candidate; clamp flags; alpha-test cutoff). The adapter
groups uses by winning file (the runtime's `resolve_preferring` rule) and skips, counted per reason: any non-albedo
use, a `_n`/`_nh`/`_spec` companion, sky/UI names, a winner already KTX2, more than one variant of the stem
(`a.tga` + `a.dds`), missing; at decode time sides < 64 px, non-4-aligned, undecodable (a constant skip marker
object). `texture.delight` per winner: strength, wrap, cutoff in the params, `ALGORITHM_VERSION` as the step version,
BC7 basic with coverage-preserving cutout mips. `texture.delight.index` plans every source and product and reads
digests only (new `RunCx::file_fingerprint` / `product_digest`): `(source, its fingerprint, ktx2 object)`.
Cache layer: a VFS source mounted *above* every mod (`Vfs::with_top_layer`) that serves `<stem>.ktx2` only while the
source is still the sole variant with the baked fingerprint; a mod's newer file changes that, so it wins and the stale
entry is never served (re-bake picks it up). Not covered: terrain layers, far impostors (bake from the originals),
hot-reloaded textures until restart. `--off delight` skips the mount. Vanilla + Tribunal + Bloodmoon (2026-11-05):
3521 de-lit, skipped small 356, effect 121, translucent 49, emissive 30, sky/ui 13, glow 9, detail 7, missing 3;
cold 34 s wall (395 s CPU summed over threads), warm 0.5 s.

Encode (D-2a built, `sf_texture::ktx2_out`): mips are an area-weighted box filter (odd axes blend partial texels)
in linear light, colour alpha-weighted so cutout holes don't bleed, kept in f32 between levels. Cutouts keep their
alpha-test coverage (D-2c, `preserve_coverage` / `bc7_ktx2_cutout`, cutoff from the content): each mip's alpha is
scaled (bisection) so the fraction passing the cutoff matches level 0 (Castaño 2010, as NVTT, Unity, UE). BC7 via `intel_tex_2` (Intel ISPC Texture Compressor kernels,
prebuilt in the crate, MIT/Apache), "basic" profile by default, its alpha variant only for a level with alpha < 255.
Rejected: `block_compression` (pure-Rust port of the same kernels, MIT, same PSNR, 4.5x slower on CPU; its decoder
is our test oracle); `basis-universal` (UASTC/ETC1S transcoding, heavier, not needed for desktop BC7). KTX2: own
layout, header and DFD serialised by the `ktx2` crate (Bevy's reader); vkFormat 146/145, no supercompression, level
0 must be 4-aligned for wgpu (`is_block_aligned`). 512x512 mips + BC7, release, one core: fast 128 ms, basic 318 ms
(alpha: 221 / 442 ms); ~5000 textures of ~256 px on 8 cores is about a minute. Bevy's loader takes sRGB from its
load settings, not the vkFormat.

## Dev-loop gates 2026-10-12 (moved 2026-11-11, bake-policy)

- Dev-loop gates (2026-10-12, Morrowind: 26k products, 16 threads): cold
  bake ≤ 2 min (measured 47.7 s), no-op (all hits) bake ≤ 2.5 s (measured
  1.75 s, of which trace validation is ~0.4 s; the rest is the adapter's
  content load, world source and report). Per-step numbers:
  `bevyrrowind/docs/specs/bake-adapter.md`. Known slack: step CPU covers
  only about 1.4x the wall time on 16 threads in the model and far-field
  phases, so time outside `Step::run` (input fingerprints, store writes,
  per-level barriers) dominates a cold bake.

## Decisions made 2026-10-11 (moved 2026-11-11, bake-policy)

- Cook LOD0 for models without animated parts (above). Animated models
  follow once `sf_model` covers controllers and particles.
- One file per trace, written by atomic rename. Switch to an embedded
  database only if file counts measurably slow down validation.
- LZ4 (`lz4_flex`) for stored objects.
- `meshopt` 0.6.2 now. Move to 0.7 when it reaches crates.io and Bevy
  moves with it; the API we use carries over.

## Concurrency rule, full text (moved 2026-11-11, bake-policy)

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

## De-lighting algorithm (moved 2026-11-11, bake-policy)

Albedo textures painted for renderers without bounce light carry baked shading that GI lights a second time.
The bake divides it out (owner decision 2026-11-05: on by default, strength 0.5; research
`docs/research/delighting-2026-11-05.md`). Algorithm: port of openmw-rtx `ShadingMap` (commit `f2f8b2d`),
in `sf_texture::delight` (Bevy-free, no deps). Per texel, linear Rec. 709 luminance weighted by alpha, summed
into a 32x32 grid by the texel's cell (empty cells take the mean of sampled ones; none sampled = neutral);
3 passes of 3x3 box blur, wrapping on a repeating axis and clamping on a clamped one; normalised to mean 1,
clamped to [0.5, 2.0]. Apply: `linear / mix(1, bilinear factor, strength)`, then the colour scaled so its
brightest channel is at most 1 (hue kept), re-encoded sRGB, alpha untouched. Differences from upstream: input is
decoded RGBA8 (per-texel cells, like their device pass); the grid is f32, never stored; the lookup clamps on a
clamped axis (theirs always wraps). Limits (Retinex): slow true-albedo gradients are flattened, few-texel
occlusion and rims stay. Cost: 512x512 grid 0.6 ms + apply 5.4 ms (release, one core).

## sf_model and ModelImporter (moved 2026-11-11, bake-policy)

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

## Reuse check and AssetProcessor deviation (moved 2026-11-11, bake-policy)

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
through an asset source. Bevy 0.20's `CompressedImageSaver` writes BCn with
full mips, but our texture cook needs alpha-coverage mips and a fingerprint-gated layer (De-lighting), so we don't use it.

## Unit test list (moved 2026-11-11, bake-policy)

- Unit tests (`sf_bake`): a fresh session hits
  everything; an invalidation matrix (file, record, parameter, version)
  re-runs exactly the dependents; unchanged product bytes stop propagation;
  dynamic reads invalidate when they change; cycles are errors; authored
  overrides win and un-override cleanly; failures don't poison other
  products; gc keeps referenced objects, and collected ones are rebuilt;
  stat memo reads only new stats. Writes are temp-file + rename, so an
  interrupted write never leaves a partial object.
