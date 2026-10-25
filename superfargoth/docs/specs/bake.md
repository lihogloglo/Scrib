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

Moved to [bake-mesh-cook.md](bake-mesh-cook.md).

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

Moved to [history](archive/bake-history.md#industry-comparison).

## Open questions

1. The pipeline-cache persistence check above (implementation detail, not
   a design choice).
2. Small-file I/O: the first real bake writes ~26k objects and traces (temp
   file + rename each), and on Windows with Defender the cold bake is
   I/O-bound (6 s of step CPU in a 42 s single-thread wall). Fine for the
   budget today. If terrain and HLOD push the count up, pack objects into
   append-only pack files (git-style) and batch trace writes.
