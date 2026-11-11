# Bake pipeline

Status: agreed (2026-10-11); built (M2). "Inventory" and "What bakes when" drafted 2026-11-11 (bake-policy), owner
questions open at the end.
Crates: `sf_bake` (new, Bevy-free: step graph, cache, CLI helpers),
`sf_model` (new, Bevy-free: neutral model and the `.sfmesh` container),
`sf_model_bevy` (new: `.sfmesh` asset loader and the `baked://` asset source)

<!-- agent-summary -->
> **Agent summary (2026-11-11).** Built: step graph, content-addressed cache with traces, planner on rayon, CLI helpers, `.sfmesh`, `baked://` source. Policy ("What bakes when"): four tiers, runtime / setup bake / incremental at start / never lazy; every derived artifact is in "Inventory" with its cost, class (gameplay or presentation) and what happens when it's missing. Budget: first bake <= 3 min on the owner's laptop (ceiling 5), then nothing at start; a mod change rebakes only what it touches, behind the start screen, never in play.
> Crates / key types: `sf_bake` (Bevy-free), `sf_model`, `sf_model_bevy`; `BakeStep` (`ID`, `VERSION`, `plan`, `run`). Decisions: constructive-trace build system over a blake3 object store, LZ4 objects; Bevy `AssetProcessor` is not the engine; one file per trace; meshopt for LODs. Co-op compares a gameplay digest, never files.
> Read next: "Inventory"; "What bakes when"; "Data model" for keys and traces; "Execution" for scheduling and the concurrency rule; `bake-mesh-cook.md`; game side `bevyrrowind/docs/specs/bake-adapter.md` "What bakes when (Morrowind)"; "Gaps and owner questions".
<!-- /agent-summary -->

## Goal

One pipeline for every expensive derived asset, with content-hash caching
per mod set. It is incremental (adding a mod rebuilds only what the mod
touches), deterministic, runnable from a CLI or at game start, and never
blocks streaming. This settles WISHLIST early decision 6.

**Budget (owner: "a one-time step under a few minutes", like a "compiling shaders" screen).** The first bake of a
full profile (vanilla + expansions, everything in "Inventory") targets **≤ 3 min, ceiling 5 min**, on the owner's
laptop (Ryzen 7 7435HS 8C/16T, RTX 4060 Laptop, NVMe). It was 2 min on an "RTX 3070-class" machine (2026-10-11), but
measured products already take ~1.5 min with navmesh, static scene and warm-up to come (owner question 1). Then a
start bakes nothing; a mod change or game update re-bakes only what it touches. A step that would break the budget
moves to the GPU or gets cheaper; per-step timings are an `sf_diag` gate, and a breach is a bug.

Baking exists to make the runtime faster and prettier. Anything that costs
frame time at runtime and can be precomputed within the budget is
precomputed.

Non-goals: shared or distributed caches (determinism keeps that possible),
live re-bake in an editor, shipping baked products (they derive from the player's own game data).

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

**Reuse (2026-10-11).** No Rust crate does persistent trace-based incremental builds (`salsa`, `comemo` are
in-memory; `cacache` is SHA/async, stale); we reuse `blake3`, `lz4_flex`, `postcard`, `rayon` around ~500 lines of
our own. Bevy's `AssetProcessor` is not the engine (one source file → one product, Bevy-only); Bevy only loads our
products through an asset source. Full text: [history](archive/bake-history.md#reuse-check-and-assetprocessor-deviation-moved-2026-11-11-bake-policy).

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

**Inputs and fingerprints.** VFS file (the winning file for a path): blake3 of the bytes, memoized by a stat key
(source, path, size, mtime; archive entries by the archive's stat plus offset), re-hashed only when the stat changes
(git's index, ninja's restat). Record (kind, id): a per-kind fingerprint function (native kinds: serde bytes; adapters
may hash the winning version's raw bytes plus its patches); a kind without one can't be a bake input. Product: its
object hash. Params: settings that change the output.

**Key** = blake3(step id, version, subject, params, sorted input fingerprints). A **trace** maps (step, subject) to
the static key, the dynamic inputs read during `run` with their fingerprints, and the output object hash; a hit
when the static key matches and every dynamic input still has its fingerprint.

**Store**, under a game-chosen cache root (never inside a mod folder; Morrowind: `%LOCALAPPDATA%/bevyrrowind/bake`):
`objects/ab/cdef…` (blake3-addressed, LZ4 via `lz4_flex` unless already compressed: decode speed over ratio, since
streaming reads them in play); `traces/<step>/<subject hash>` (one file per trace, atomic rename);
`profiles/<profile>.manifest` ((step, subject) → object, loaded once at startup) and `.fingerprint` beside it;
`filehash.memo` (stat → blake3). Planned: `profiles/<profile>.gameplay` (gameplay digest, "Co-op"),
`stats/<step>.ron` (ms per product for the progress estimate), `gpu/<adapter key>/` (pipeline-warmup.md). Objects
are shared across profiles; `gc` keeps everything reachable from any manifest, then evicts by LRU to a cap (10 GiB).

**Authored beats derived.** A step may declare an override path (for
example `lod/<model path>/<n>.sfmesh`). If the VFS has a file there, that
file *is* the product, fingerprinted like any input. Mods can ship
hand-made LODs.

**`sf_model`** (Bevy-free) is the neutral model importers produce and steps consume; `.sfmesh` is its cooked
container (LOD table, meshopt-encoded streams); `ModelImporter` maps legacy formats into it:
[history](archive/bake-history.md#sf_model-and-modelimporter-moved-2026-11-11-bake-policy).

`mesh.cook` details: [bake-mesh-cook.md](bake-mesh-cook.md). The M2 step table moved to the
[history](archive/bake-history.md#m2-steps-moved-2026-11-11-bake-policy-superseded-by-bakemd-inventory).

## Inventory

Every derived artifact, 2026-11-11. Costs: cold, release, owner's laptop, 16 threads, vanilla + Tribunal + Bloodmoon
(dev-loop 2026-10-12 and later per-system runs; CPU-s = step time summed over threads). Class: **G** gameplay (the
simulation reads it; co-op peers must agree), **P** presentation (may differ per machine). "If missing" is the
runtime's behaviour when the product is absent or stale ("Missing or stale at runtime"). Every built step runs in
`bevyrrowind bake` today; exceptions are in italics.

**Baked products (`sf_bake` steps).**

| Artifact (steps) | Crate | Input → output | Cold cost | Disk | Invalidated by | Class | If missing |
|---|---|---|---|---|---|---|---|
| Model info, cooked meshes + LODs (`model.info`, `mesh.cook`, `mesh.rest`, `mesh.rig`) | `sf_model`, importer `mw_bake` | model file → bounds/flags; `.sfmesh` LOD chain; skinned rest/rig | 22.4 s wall with collision (`mesh.cook` 27.8 CPU-s) | in the 793 MB below | model bytes, importer + step version | P (bounds feed the index) | runtime NIF import, LOD0 (built) |
| Static collision (`collision.cook`) | `sf_controller`, rules `mw_bake::collision` | collision node / LOD1 → collision source | 2.8 CPU-s | small | model bytes, rules version | G | runtime path unverified: treat as required |
| Item bodies, supports, settle check (`body.cook`, `support.cook`, `items.check`) | `sf_physics`, `mw_bake::body` | item models, placements → convex bodies, support data, settle report | unmeasured | unmeasured | models, placements | G | required |
| Ragdolls (`ragdoll.cook`) | `sf_ragdoll` | skeleton + parts → ragdoll | 9 ms median per part | small | skeleton, rules | G | ragdoll off (corpse plays death clip) |
| World index (`world.region/layout/page/dir`) | `sf_world` | placement records + bounds → 1 km pages, door links | 3.0 s (source 0.58 + index 2.4) | small | placements, `model.info` | G | required: exterior can't stream (built: error) |
| Terrain (`terrain.tile/pyramid/macro/layer_table/layers`) | `sf_terrain` | heights + layers + textures → tiles, pyramid, macro albedo, KTX2 arrays | 6.2 s wall (CPU: pyramid 24.4, tile 11.7, macro 10.7) | in 793 MB | land records, layer textures, quality tier | heights G, layers P | required |
| RT terrain proxy (`terrain.rt_proxy`) | `sf_terrain` | tile → static mesh | 8.7 CPU-s | small | tile | P (Ultra) | Solari without terrain |
| Far field (`lod.policy/impostor/node/near/dir`) | `sf_lod` | models + placements → L1-L3 proxies, impostor atlases, near buckets, lights | 15.2 s (CPU: policy 12.5, impostor 5.5) | ~230 MiB meshes + 80 MiB RGBA8 atlases | placements, models, versions (no preset: J1) | P | no distant land |
| Static scene (`scene.chains`, `scene.cells`) *(built, not wired (scene-coord C1))* | `sf_scene`, `mw_bake::scene` | static placements → per-cell instance tables + chains | unmeasured | ~80 B/instance + chains | placements, cooked meshes, material keys | P | statics as entities (today's path) |
| Animation (`skeleton.cook`, `rig.profile`, `anim.cook`, `anim.retarget`, `animgraph.compile`, `mw.anim.groups`, `mw.anim.source_clips`) *(tests bake their own cache)* | `sf_anim`, `sf_anim_graph`, `mw_bake::anim` | skeletons, KF files, graphs, BYO clip libraries → clips, compiled graphs | 9.0 s wall (was 13.5) | unmeasured | KF/skeleton bytes, graph files, BYO library contents | G (root motion, hit windows) | required |
| Legacy scripts (`mwscript.translate`, `mwscript.coverage`) | `mw_bake::script` | MWScript + dialogue results → Luau + bytecode | gate ≤ 2 s, unmeasured | small | script records, translator version | G | required |
| De-lit albedo + alpha classes (`texture.roles`, `texture.delight(.index)`, `texture.alpha_class(.index)`) | `mw_bake::delight`, `sf_texture` | albedo textures → de-lit BC7 KTX2 with coverage mips | 34 s wall (395 CPU-s) | unmeasured (estimate 0.3-0.6 GB) | texture bytes, strength, cutoff, algorithm version | P | originals (built: fingerprint-gated layer) |
| Navmesh (`nav.tiles`, `nav.links`, `nav.coarse`) *(not built)* | `sf_nav` (unbuilt, NV4) | cooked collision + terrain + water + doors → Detour tiles, links, HPA* graph | estimate 30-60 s, both classes (NV4 measures) | unmeasured | collision products, heights, door records, class config | G | AI without navmesh (nav.md must define: pathgrid or direct steer) |
| Reserved: pipeline warm-up *(not built (first-sight specialisation))* | wgpu / Bevy, `pipeline-warmup.md` (parallel session) | material/mesh key catalog → driver pipeline blobs | unmeasured; budget slot 20 s | per GPU | GPU, driver, wgpu version, shader set, material catalog | P, per machine | compile on first sight (hitch) |
| Planned, unbuilt | | shoreline field (water M6), erosion and PBR maps (M7) | | | | P | |

The 2026-10-12 store for one profile (models, world, terrain, far field) was 793 MB in 48k files; animation, de-light
and scripts are not in that number.

**Runtime-only derived data (never cached).**

| Artifact | Where | Cost | Why not baked |
|---|---|---|---|
| Record database, load order merge, VFS index | `sf_mods`, `sf_records`, `sf_vfs` | 0.27 s at load | cheap; the fingerprint that keys every bake comes from here |
| Profile fingerprint | `sf_mods` + `filehash.memo` | stats only when unchanged | it is the cache check |
| Native Luau scripts | `sf_script` | µs per file at load | cheaper than a cache read (mods.md) |
| Audio | `bevy_audio` | decoded while playing | streaming decode is the industry default for music and VO |
| Video (Bink) | `sf_video` | decoded while playing | |
| GI | `gi.md` | per frame | dynamic probes, nothing bakes (owner 2026-11-01) |
| Character outfits (merged bodies) | `character.md` | per spawn, in-memory LRU | combinatorial; pre-baking common outfits is character.md question 3 |
| Nav runtime patches (door flags, later TileCache carves) | `sf_nav` | per event, budgeted | nav.md: patches are not "baking during play" |
| Other textures (normal, glow, non-de-lit albedo) | Bevy loader | at load | **gap**: uncompressed TGA/DDS upload as RGBA8; designed as `texture.cook` in [textures.md](textures.md) §5 (gap 8) |

## What bakes when

**Tiers.**

1. **Runtime** (the table above): cheap, per-instance, per-frame or per-machine-state work. Never cached on disk.
2. **Setup bake**: every product in "Inventory", baked once per profile on the setup screen (`bevyrrowind/docs/specs/setup.md`)
   before the main menu. There is no "lazy" product class and no "bake only if the preset needs it" class: the
   products are preset-independent (far field J1, LOD selection, mip choice are runtime choices), so a preset change
   never bakes. The few bake params (de-light strength, terrain layer size) change only from an Options control that
   says "requires a rebake (~N s)" and runs the incremental bake on the start screen.
3. **Incremental at start**: on every start the host computes the profile fingerprint (stat-based, < 1 s warm); a
   mismatch with the stored one runs the bake behind the start screen before the menu. Traces rebuild only products
   whose inputs changed, so a texture mod costs the de-light of its textures (and terrain layers if a land texture
   changed), a mesh replacer costs its `mesh.cook`, collision, the far-field nodes, scene chains and nav tiles that
   read it. Same for a game update: only steps whose `VERSION` changed rebuild.
4. **Never during play.** No background bake, no cook behind a door or fast-travel load screen: load screens are play.
   The one exception is dev "mod author mode" hot reload (mods.md), which re-runs affected products on a background
   task. The pipeline warm-up's leftovers (keys the catalog missed) compile on first sight as Bevy does today; that is
   pipeline-warmup.md's problem, not a bake.

**Budget per artifact** (first launch, owner's laptop, vanilla + expansions; targets sum to the 3 min target):

| Phase | Measured | Target |
|---|---|---|
| Content load + first input hashing (`filehash.memo` cold) | 0.27 s load; cold hashing unmeasured | 5 s |
| Models + collision | 22.4 s | 20 s |
| World index | 3.0 s | 3 s |
| Terrain (+ RT proxy) | 6.2 s | 8 s |
| Far field | 15.2 s | 15 s |
| Static scene | unmeasured | 10 s |
| Animation | 9.0 s | 10 s |
| Physics (bodies, items, ragdolls) | unmeasured | 5 s |
| Scripts | ≤ 2 s gate | 2 s |
| De-light textures | 34 s | 30 s |
| Navmesh, 2 classes | not built | 45 s |
| Pipeline warm-up (reserved) | not built | 20 s |
| **Total** | ~92 s of measured phases, run one after another | **≤ 3 min (ceiling 5)** |

Levers if the sum breaks the target, in order: one plan across phases (today `mw_boot_bevy::bake` runs ~9 `bake()`
calls in sequence, each a barrier, so the CPU-bound de-light can't overlap the I/O-bound model phase); the low
parallel efficiency outside `Step::run` (history: models 32 CPU-s in 22 s wall on 16 threads); the BC7 "fast" profile
(128 vs 318 ms per 512²) or a GPU BC7 encoder for de-light; pack files for small objects (question below); coarser
exterior humanoid nav tiles (nav.md). Mod lists scale the bake with content; the setup screen shows an estimate.

**Invalidation keys** (all in the key or the profile fingerprint): mod list and load order (VFS and record winners
are inputs), file bytes, game data version (content hashes; no version string), step `VERSION` and importer
versions (object format versions included), bake params. A code change without a `VERSION` bump goes unnoticed
(gap 7; UE's version GUIDs share the failure mode).

**Progress reporting.** The plan pass knows every product before it runs, so `sf_bake` emits
`BakeEvent::{Planned { step, total }, Done { step, built }, Failed { step, subject, error }, Phase(name)}` on a
channel the setup screen polls (Bevy-free core, thin UI): one bar weighted by expected cost (`stats/<step>.ron`,
defaults from the table above), per-phase rows, an ETA, "Cancel". Cancel is safe and resumable (atomic writes: a
rerun hits everything built). The pool is `cores - 1` threads at below-normal priority so the screen stays live.

**Missing or stale at runtime.** The game never bakes in play and never serves a stale product:

- **Start check**: fingerprint mismatch or no manifest → tier 3 bake before the menu. The player can't skip a
  required product (world index, terrain, collision, bodies, animation, scripts): the menu offers "Bake" or "Quit".
- **Degradable products** (far field, static scene, de-light, RT proxy, ragdolls, navmesh while nav is optional) have a
  defined fallback (Inventory, "If missing"); a cancelled or failed bake of one starts the game with that feature off
  and a warning, and the next start retries.
- **Per product**: a product whose step failed falls back the same way and is reported, never silently baked
  mid-game. Products that are served through a VFS layer follow the de-light rule: served only while their source is
  still the winner with the baked fingerprint, so a mod's newer file wins over a stale product.
- Streaming never waits on the bake.

**Co-op (host and client must agree; hashes, not files).** Products derive from each player's own game data and are
never transferred; determinism makes the same inputs and build give the same bytes. Each step declares its class (G
or P). After a bake the store writes the **gameplay digest**: blake3 over the sorted (step, subject, object hash) of
every G product. The join handshake compares build version and gameplay digest; on a mismatch the host sends its
mod list (ids, versions, content hashes, Wabbajack style) and the client shows what differs. P products may differ.
G steps must be deterministic across machines (no runtime CPU-feature dispatch, no thread-count-dependent
reductions; ISPC BC7 is P). Verify: owner's desktop and laptop give the same digest (schedule with net.md).

**Profiles and disk.** One manifest per mod profile (the profile name must identify the profile, not only the base
game files: bake-adapter.md gap). After a successful bake the host runs `gc` when the store exceeds the cap.

## Industry comparison: who bakes what, when

Table and sources: `../research/bake-vs-runtime-2026-11-11.md`. Verdicts: **differ by design** from Bethesda (CK
precombines, previs, navmesh baked by authors and shipped in plugins, so mods conflict over them and users run PRP or
DynDOLOD last): we rebake per load order on the player's machine, and the start check is DynDOLOD's "re-run after
mod changes", automatic and incremental. **Differ** from OpenMW (runtime paging and navmesh, optional
`openmw-navmeshtool` DB): we bake more within one budget. **Match** UE DDC, Unity Asset DB v2, Bazel/ccache on
keying (inputs + tool version), and UE PSO precaching / Unity PSO tracing on the warm-up (slot reserved).
Earlier pipeline comparison (J9): [history](archive/bake-history.md#industry-comparison).

## De-lighting (designed 2026-11-05)

Albedo painted for renderers without bounce light carries baked shading that GI lights twice; the bake divides it
out (owner 2026-11-05: on by default, strength 0.5; research `docs/research/delighting-2026-11-05.md`). Port of
openmw-rtx `ShadingMap` (commit `f2f8b2d`) in `sf_texture::delight`: a blurred 32x32 luminance grid per texture,
divided out at the chosen strength. Algorithm, limits and cost: [history](archive/bake-history.md#de-lighting-algorithm-moved-2026-11-11-bake-policy).

Products (`texture.roles`, `texture.delight`, `texture.delight.index`, BC7 KTX2 with coverage-preserving mips via
`intel_tex_2`) and the fingerprint-gated cache layer (`Vfs::with_top_layer`, `--off delight`): as built in the
[history](archive/bake-history.md#de-lighting-as-built-moved-2026-11-11-bake-policy). Vanilla + expansions: 3521
de-lit, cold 34 s wall (395 CPU-s), warm 0.5 s.

## Execution

- **Planner.** Plan every unfinished product reachable through planned product inputs, then run them on rayon with a
  dependency-counting scheduler (cargo's job queue; Bazel and Buck2 never park a worker on an unfinished
  dependency). Dynamic reads resolve inline; cycles are errors; a failed product doesn't stop unrelated ones.
- **Concurrency (rule, 2026-10-12).** Nothing in a bake blocks on work that runs on the rayon pool (a worker waiting
  in `join` steals jobs that may wait on its lock: the `animcook` deadlock, `sf_bake/tests/concurrency.rs`). A
  product's slot is never waited on (the thread builds it itself; `StepStats::duplicates` counts these); inputs are
  fingerprinted sequentially; step memos block only on sequential initializers. Full rule:
  [history](archive/bake-history.md#concurrency-rule-full-text-moved-2026-11-11-bake-policy).
- **Determinism.** Same inputs give bit-identical outputs (no hash-map order, fixed-order reductions, no thread-count
  dependence, no timestamps); `verify --sample N` re-runs N cached products and compares.
- **CLI.** `<game> bake [--profile P] [--step S] [--force] [--verify N] [gc]` from the game binary (adapters must be
  wired), with progress and per-step timings; the setup screen runs the same entry point on a background thread.
- **Game start**: "What bakes when" (tiers 2-4, "Missing or stale at runtime").
- **Pipeline warm-up**: owned by `pipeline-warmup.md` (research `../research/shader-precompile-2026-11-11.md`); this spec reserves its slot in the
  budget, its progress row on the setup screen and `gpu/<adapter key>/` in the store. Its key is the machine's
  (GPU, driver, wgpu version, shader set, material catalog), not content, so it is not an `sf_bake` step.
- **Cold start** (WISHLIST §0: < 10 s after the first bake): load the manifest, validate traces through stat keys.
- **Loading.** `sf_model_bevy`'s `baked://` asset source resolves (step, subject) through the manifest; not an
  `sf_vfs` layer, so a re-bake swaps the manifest without rebuilding the VFS index.

## Verification

- Unit tests (`sf_bake`): hits, an invalidation matrix (file, record, param, version), early cut-off, dynamic reads,
  cycles, overrides, failure isolation, gc, stat memo, atomic writes:
  [list](archive/bake-history.md#unit-test-list-moved-2026-11-11-bake-policy).
- Determinism: baking with 1 and N threads produces identical manifests. Planned: same gameplay digest on two machines.
- Incremental: a test mod that touches one model re-runs only that model's
  `model.info`/`mesh.cook` and the index pages that reference it. The CLI
  prints the counts.
- Budget gate: the full first bake on the owner's laptop finishes in ≤ 3 min (ceiling 5), per-step timings in
  `sf_diag`; no-op bake ≤ 2.5 s (measured 1.75 s, 2026-10-12). Per-step numbers: `bevyrrowind/docs/specs/bake-adapter.md`;
  the 2026-10-12 gate run is in the [history](archive/bake-history.md#dev-loop-gates-2026-10-12-moved-2026-11-11-bake-policy).
- LOD quality: the CLI reports error and reduction per level. An
  LOD-colored debug view and crossfade screenshots in the viewer (owner's checks);
  `to_obj` exports for side-by-side checks in Blender.

Decisions of 2026-10-11 (LOD0 cook for static models, one file per trace, LZ4, meshopt 0.6.2):
[history](archive/bake-history.md#decisions-made-2026-10-11-moved-2026-11-11-bake-policy).

## Gaps and owner questions

Gaps (2026-11-11, found by this audit; log, don't chase):

1. **No start check in the game.** `bevyrrowind` loads the manifest without comparing fingerprints
   (`apps/bevyrrowind/src/main.rs` view/exterior options); only `mods` reports "stale". Tier 3 needs it.
2. **Profile identity.** `mw_bake::profile_name` hashes only the install's archives and content files, so two
   `sf_mods` profiles share one manifest and overwrite it; "switching back is instant" doesn't hold (bake-adapter.md).
3. **Phases run in sequence** (~9 `bake()` calls): the budget's first lever.
4. **No progress events** in `sf_bake` (stats are printed at the end), and no per-step cost model.
5. **No `bake gc` subcommand** and no auto-gc (the live store grew to 2.5 GB by 2026-10-12).
6. **No step class (G/P)** and no gameplay digest (co-op).
7. **No golden-digest test per step** to catch a changed output without a `VERSION` bump.
8. **Unbaked textures**: normal, glow and non-de-lit albedo textures upload as shipped (uncompressed TGA/DDS as
   RGBA8, mips as shipped). Designed 2026-11-11: `texture.cook` (BC7/BC5/BC4 per role, rewrap of BCn sources,
   one served layer with de-light), [textures.md](textures.md) §5, slices T1-T3; residency (streaming) is §6 there.
9. **Impostor atlases are RGBA8** (80 MiB): BC7 them ([textures.md](textures.md) slice T7).
10. **Unmeasured**: cold input hashing, physics steps, scene cells, script translation, animation and de-light disk
    sizes. A full `bevyrrowind bake --cache <empty>` on the owner's laptop with all steps records them.
11. **Small-file I/O**: the store is one file per object and trace (48k files for 793 MB); with Defender the cold
    bake is I/O-bound (6 s of step CPU in a 42 s single-thread wall). If terrain, nav and scene push counts up, pack
    objects into append-only pack files (git-style) and batch trace writes.
12. **No-navmesh fallback** is undefined in nav.md (needed for "degradable" or else navmesh joins the required set).

Owner questions:

1. Budget: accept **≤ 3 min target / 5 min ceiling** on the laptop for vanilla + expansions (was 2 min on a
   desktop-class reference)? Should large mod lists (Tamriel Rebuilt) get their own number, or "scales with content,
   shown with an ETA"?
2. Degradable products: may the player start with far field, static scene or de-light missing after a cancelled bake
   (feature off, warned), or must every product be baked before the menu?
3. Load screens: confirm they count as play (no lazy cooks behind doors or fast travel), as this spec assumes.
4. Should the bake use the GPU (BC7 encode, impostor renders) on the setup screen, if it buys the budget back?

Owner answers 2026-11-11: (1) yes, ≤ 3 min / 5 min ceiling. (2) yes: "they'll suffer frame spikes but it should
run", and hide loading behind a loading screen when necessary (degradable products fall back to their runtime path,
e.g. statics as entities). (3) decided by the standard (UE/Unity loading screens precache PSOs and
stream, never cook content): load screens may do bounded per-arrival work (pipeline warm-up, uploads,
`pipeline-warmup.md`) but no content bakes. (4) yes, the GPU may be used.
