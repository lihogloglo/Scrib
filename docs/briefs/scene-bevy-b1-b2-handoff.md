# Handoff: scene runtime B1 and B2 (`sf_scene_bevy` the crate, `CellScene` the seam)

Written 2026-11-10 by the implementation agent on `static-scene/b1-b2`, branched off `main` = `6fc7722`.
Brief: `docs/briefs/scene-bevy-b1-b2.md`. Owner ruling folded in: `upload` is one invocation that must
complete, and `run_stage`'s cost accounting is untouched.

Two commits: B1 (`0244ba5`), B2 (`d88e360`).

**Nothing here has been exercised in a running game.** There is no Morrowind install and no baked data on
this machine, and the owner's standing rule forbids judging rendering from screenshots. Everything below
marked "verified" means: it compiles, clippy is clean with `-D warnings`, and a headless test asserts it.
`ScenePlugin` is in no `add_plugins` call and no app inserts `SceneCells`, so the tree ships identically
until C2.

---

## 1. What exists now

### B1 — `superfargoth/crates/sf_scene_bevy`

```
superfargoth/crates/sf_scene_bevy/Cargo.toml
superfargoth/crates/sf_scene_bevy/src/lib.rs      # ScenePlugin, SceneConfig, baked_extensions
superfargoth/crates/sf_scene_bevy/src/assets.rs   # SceneTableAsset, SceneTableLoader, SceneLoadError
superfargoth/crates/sf_scene_bevy/src/tests.rs    # 2 tests
```
Plus one line in the root `Cargo.toml` `[workspace.dependencies]` (alphabetical, between `sf_scene` and
`sf_script`) and the `sf_scene_bevy` `[[package]]` entry in `Cargo.lock` — exactly one package, no new
external crate.

Every API, verbatim:

```rust
// assets.rs
#[derive(Debug, thiserror::Error)]
pub enum SceneLoadError { Io(#[from] std::io::Error), Table(#[from] sf_scene::TableError) }

#[derive(Asset, TypePath, Debug)]
pub struct SceneTableAsset(pub Vec<u8>);
impl SceneTableAsset {
    pub fn view(&self) -> Result<StaticCellTableView<'_>, TableError>;
    pub fn bytes(&self) -> u64;                       // self.0.len()
}

#[derive(Default, TypePath)]
pub struct SceneTableLoader;                          // Asset = SceneTableAsset, Settings = (),
                                                      // extensions() = &[sf_scene::CELLS_EXT]

// lib.rs
#[derive(Resource, Clone, Debug, PartialEq, Eq)]
pub struct SceneConfig {
    pub enabled: bool,                    // false
    pub space: sf_scene::SpaceConfig,     // SpaceConfig::default()
    pub worldspaces: Option<Vec<WorldspaceId>>,  // None: every worldspace with a scene.cells product
    pub shift_rewrite_bytes: u64,         // 1 << 20
}
impl SceneConfig { pub fn admits(&self, worldspace: &WorldspaceId) -> bool; }  // enabled && in the list

#[derive(Default)]
pub struct ScenePlugin { pub config: SceneConfig }
// build(): insert_resource(config.clone()), init_asset::<SceneTableAsset>(), register_asset_loader(SceneTableLoader)

pub fn baked_extensions() -> impl Iterator<Item = (String, String)>;
// [("scene.cells", "sfscene"), ("scene.chains", "sfmesh")]
```

Decisions taken, with the brief's §8 defaults they follow:

- **The loader validates.** `load()` does `read_to_end`, then `StaticCellTableView::parse(&bytes)?`, then
  returns `SceneTableAsset(bytes)` — the view is discarded. A corrupt table is therefore an ordinary
  `LoadState::Failed` decided on Bevy's loader task, off the main thread, and §5.3's rows 1 and 2 collapse
  into one code path for B3 (`ready` true, `covered` `None`).
- **Narrow dependency set**, only what B1 uses: `bevy_app`, `bevy_asset`, `bevy_ecs`, `bevy_reflect`,
  `sf_scene`, `sf_world`, `thiserror`, `workspace-hack`. B1 needed neither `bevy_math` nor `log` (the brief
  listed both); the manifest says in a comment which crates B3/B4 add and that doing so is free.
  Dev-dependencies: `bevy`, `glam`, `sf_bake`, `sf_model_bevy`, `tempfile`.
- **`SceneConfig` holds a `SpaceConfig`** instead of re-declaring `granularity` / `frames_in_flight` /
  `batch_capacity`. `SpaceConfig::space: SpaceId` is per space, so B3 overrides it per worldspace.
- **No `upload_bytes_per_frame`.** There is no resumable upload to configure (§3 below).
- **No `sf_scene::product::EXTENSIONS`** added: the pairs are built from the four consts, as the brief ruled.

### B2 — the seam in `sf_world_bevy`

`superfargoth/crates/sf_world_bevy/src/scene.rs` (new, ~110 lines), `mod scene;` beside `mod precombine;`
and `pub use scene::{CellScene, SceneCellCx, SceneCells};` beside the precombine's line.

```rust
pub struct SceneCellCx<'a> {
    pub worldspace: &'a WorldspaceId,
    pub cell: IVec3,
    pub cell_origin: WorldPos,   // info(ws).partition.cell_origin(cell)
    pub level: Level,            // always Level::Near while only the near ring has a scene
}

pub trait CellScene: Send + Sync + 'static {
    fn acquire(&self, world: &World, worldspace: &WorldspaceId, cell: IVec3) -> Option<UntypedHandle>;
    fn ready(&self, world: &World, worldspace: &WorldspaceId, cell: IVec3) -> bool;
    fn covered(&self, world: &World, worldspace: &WorldspaceId, cell: IVec3)
        -> Option<Arc<HashSet<PlacementId>>>;          // sf_world::PlacementId, Work::members' own type
    fn upload(&self, world: &mut World, cx: &SceneCellCx<'_>) -> u64;   // bytes queued; MUST complete
    fn release(&self, world: &mut World, worldspace: &WorldspaceId, cell: IVec3);  // MUST be idempotent
    fn bytes(&self, _worldspace: &WorldspaceId, _cell: IVec3) -> u64 { 0 }
}

#[derive(Resource, Clone, Default)]
pub struct SceneCells(pub Option<Arc<dyn CellScene>>);
impl SceneCells { pub fn is_installed(&self) -> bool; }
```

Order for one cell: `acquire` → the fetch waits for the handle it returned → `covered` (once, on the
resolved table) → `ready` until true → `upload` (once) → `release`, also after a cancelled load.

`stream.rs` fields added: `Runtime::scene: SceneCells` (refreshed in `drive` beside `self.rt.far`),
`Work::scene: bool`, `Work::scene_table: Option<UntypedHandle>`,
`StreamStats::{scene_covered: u64, scene_upload_bytes: u64}`.

The seven call sites, as built:

| # | Where | What |
|---|---|---|
| 1 | `start_fetch` | `scene = installed && kind == Load && to == Near`; `scene_table = acquire(..)`. Which cells and which worldspaces have a table is the hook's business. |
| 2 | `poll_fetches`' `settled` | a `pending(state)` helper, then two `is_none_or` conjuncts: `near` and `scene_table`. This is what makes `covered` a decision on a resolved table. |
| 3 | `poll_fetches` | `covered` before the model gather; a `drawn(id)` closure = precombine members ∪ covered feeds the gather; then a three-way `match (members, covered)` for `work.members` — precombine only `Arc::new(m)`, scene only the hook's `Arc` **moved, not rebuilt**, both a union. `work.near = None` on a precombine fallback now fires **independently** of `covered`. `stats.scene_covered += covered.len()`. |
| 4 | `drive`'s `ready` closure | one more conjunct, the shape of the `w.proxy` arm. |
| 5 | `upload_estimate` | `scene` bytes `saturating_add`ed to **both** `Level::Near` arms (the scene is orthogonal to whether a far field is installed, so the match is not widened to 2×2). |
| 6 | `despawn_item` | `if (request.from == Level::Near \|\| request.to == Level::Unloaded)` → `release`. The second disjunct is the `to == Unloaded` sweep that `std::mem::take`s every remaining level's group with no per-level despawn request. |
| 7 | `Runtime::cancel` | `if work.scene` → `release`, beside the `work.proxy` far release it copies exactly. |
| 8 | `reveal` | `upload` runs **before** the reveal, so the cell's instances are in the scene buffer by the time its group becomes visible. `self.stats.scene_upload_bytes += scene.upload(world, &cx)`. The `Work` removal and the rest of the function are unchanged. |

---

## 2. Where the brief needed correcting

1. **The `sf_world_bevy` test baseline is 29, not 25.** §7 run 8 says "the `e3112be` baseline is 25 (11 in
   `tests.rs`, 14 in `gpu_batch_tests.rs`) + 3 new" and to expect 28. The binary actually runs **29** tests
   at `main` (`tests.rs`, `gpu_batch_tests.rs` **and** `matclass::tests`, which §7 forgot). With the three
   new ones: **32 passed, 0 failed**. Never fewer than 32 from here.
2. **The gated `IndexReader` of §6.2 must not be used, and I did not ship it.** `IoTaskPool` is a
   *process-wide* singleton shared by every test in the binary. A reader that spins in `page()` holds its
   threads, and it starved three unrelated tests: `reload_resident_respawns_cells_from_the_loaded_overlay`,
   `a_cell_without_a_scene_hook_streams_as_before` and
   `scene_covered_placements_load_no_models_and_spawn_nothing` all failed with "streaming never went idle".
   **What works instead, and is just as deterministic:** gate `CellScene::ready`. A near load then sits in
   its `Stage::Assets` forever, on the main thread, holding no IO; moving the anchor out of every ring
   cancels it, and `Runtime::cancel` is the only path that can release. Measured: the cancelled-load half
   passes, and `releases` is non-empty before the gate opens.
3. **`IVec3` is not `Ord`**, so the test log's per-cell maps are `HashMap`, not `BTreeMap` (§6.2 does not say
   which, but the obvious reading is wrong).
4. **The test fixture's near step is empty**, which §6.2 does not mention and which silently makes the
   covered test prove nothing. The plate model is 8 m across, so `ModelInfo` gives every placement of
   `placements()` a radius of ≈5.66 > the directory's `far_min_radius` of 4.0, and
   `LevelContent::level_of` puts **all 1602 of them at `Level::Far`**. The near step — the only level the
   static scene draws — therefore contains nothing, and `covered` has nothing to cover. Fix: a new
   `streaming_app_radius(dir, far_min_radius)` under the unchanged `streaming_app`; the covered test runs
   with `f32::MAX`, which puts every placement at `Level::Near`. **B3/B4 must know this**: any test of
   scene residency in this harness has to override `far_min_radius` or it will assert on an empty cell.
   (With `f32::MAX` the fly-away leaves 4 `StreamCell` entities resident where the default radius leaves 0,
   so the release test's near-unload half keeps the default radius.)
5. **`clippy::inconsistent_struct_constructor`** rejects `Work { near, scene, scene_table, .. }`: the order
   must follow the declaration, `Work { scene, near, scene_table, .. }`. Trivial, but it is a `-D warnings`
   failure and §7 run 7 would have caught it only after a build.
6. §2.3's claim that the root `Cargo.toml` needs only one `[workspace.dependencies]` line is **confirmed**:
   `members` is a glob, and `cargo hakari generate --diff` and `manage-deps --dry-run` are both clean.

Everything else the brief asserted held, including the two structural findings (§3.1's two gates, §3.2's
non-resumable `reveal`) and the seventh call site.

---

## 3. The upload decision, as shipped

The owner ruled (2026-11-10): **do not touch `run_stage`'s cost accounting**; `upload()` is a single
invocation that returns `done == true`. So the trait returns a plain `u64` — the bytes queued — and has no
"done" at all, which is more honest than a `bool` nobody may return `false` for. `reveal` stays
non-resumable; `Stage::Upload` keeps charging `request.upload_bytes` once per request, which is correct
today because `reveal` returns `true` unconditionally.

The latent bug that remains (logged, not chased): if any stage item ever returns `false` from an
`Stage::Upload` invocation, `run_stage` (`stream.rs` `let cost = if stage == Stage::Upload { request.upload_bytes as f64 }`)
charges the **whole** estimate on **every** invocation. Evidence: `run_stage`'s cost closure and `reveal`'s
unconditional `true`. A ROADMAP line is in the report to the coordinator.

**For B3/B4:** make `CellScene::bytes` report the real table byte count (`SceneTableAsset::bytes()` plus
whatever chain geometry the cell newly pins), because that is what `upload_estimate` turns into
`request.upload_bytes` and therefore the only throttle on how many cells upload per frame. If a real bake
produces a cell table too large for one buffer write, resumable upload becomes its own slice **with a
measurement behind it** — and that slice is the one that fixes the charging bug.

---

## 4. What is verified, and how

| Command | Output |
|---|---|
| `hakari generate --diff` | no output (no diff) |
| `hakari manage-deps --dry-run` | `info: no operations to perform` |
| `check -p sf_scene_bevy --all-targets --locked` | `Finished ... in 33.88s`, no error, no warning |
| `clippy -p sf_scene_bevy --all-targets --no-deps -- -D warnings` | `Finished`, clean |
| `test -p sf_scene_bevy --locked` | `test result: ok. 2 passed; 0 failed; 0 ignored` |
| `check -p sf_world_bevy --all-targets --locked` | `Finished`, clean |
| `clippy -p sf_world_bevy --all-targets --no-deps -- -D warnings` | `Finished`, clean |
| `test -p sf_world_bevy --locked` | `test result: ok. 32 passed; 0 failed; 0 ignored` |
| `check -p sf_lod_bevy -p mw_exterior_bevy --all-targets --locked` | `Finished ... in 1m 10s`, clean |
| `test -p sf_scene --features bake --all-targets` | `test result: ok. 105 passed; 0 failed; 2 ignored` — the `main` baseline exactly |
| `cargo tree -p sf_scene_bevy --edges normal --depth 1` | the eight manifest deps, no `sf_bake` |
| `cargo tree -p sf_scene_bevy --edges features --depth 1` | `sf_scene feature "default"` — **`bake` is not reachable** |

The five tests and what each asserts exactly:

- `the_cells_loader_parses_a_baked_table` — a synthetic v3 table through `baked_extensions()` →
  `BakedReader` → the extension-keyed loader → the owned `Vec` → the borrowing view: bytes identical,
  7 instances in placement order, `origin_cell` as written.
- `a_corrupt_table_fails_the_load` — a truncated table is `LoadState::Failed` and nothing lands in `Assets`.
- `scene_covered_placements_load_no_models_and_spawn_nothing` — `scene_covered == |covered|`,
  `precombined == |covered|` (exactly those placements skipped at spawn and no others), `covered` asked
  once for the cell, `upload` called once for it, `scene_upload_bytes == 4096`, mesh parts strictly fewer
  than the no-hook run, the cell's two dynamic placements' spawner called twice without the hook and
  **zero** times with it, and **activations identical either way** (the §2.4 property: covered placements
  still activate, because the activate stage is a separate request with its own empty `members`).
- `a_cell_without_a_scene_hook_streams_as_before` — an inert hook, proved exercised (`acquires`,
  `covereds`, `readies`, `uploads` all non-empty), leaves the whole `StreamStats`, the mesh-part count and
  the spawner counts **equal** to a run with no `SceneCells` at all. This is the regression gate for all
  eight call sites, and it also proves `covered -> None` is a true no-op.
- `release_runs_on_the_near_unload_and_after_a_cancelled_load` — near-unload half: fly 50 km out, settle,
  and every cell ever acquired has been released at least once, with `releases` non-empty. Cancelled half:
  the readiness gate holds every near load in its assets stage, the anchor leaves every ring, `releases` is
  non-empty **before** the gate opens (so the release came from `Runtime::cancel` and nowhere else), and
  after settling every acquired cell has been released.

**Not verified, and cannot be here:** that a real baked `scene.cells` product loads (no data on this
machine); that `acquire`'s handle actually holds a fetch open for a *real* asset — `TestScene::acquire`
returns `None`, so site 2's `is_none_or` short-circuits and only the code path, not the waiting, is
exercised. **B3 must add that test** once `SceneTableAsset` is really loaded per cell: start a handle in
`acquire` and assert `covered` is asked only after the handle resolved. Also unverified: anything about
rendering, timing, or the Steam Deck floor.

---

## 5. What B3/B4 must know

1. **`sf_world_bevy` must never depend on `sf_scene_bevy`.** The hook exists so the dependency goes one
   way; `sf_scene_bevy` implements `CellScene` and inserts `SceneCells`.
2. **The precombine interlock is C2's line, and B2's `covered` union is only safe because of it.**
   `spawn_precombine` builds its buckets' LOD ends from `Work::members`, which the scene's `covered` set now
   joins, so an app that installs a `CellScene` **must** also clear `StreamingConfig::precombine`
   (`streaming.precombine &= !scene`, before `add_plugins`). This is written into `scene.rs`'s module docs
   so the plugin cannot land without it. `--off matclass` keeps working either way.
3. **Origin shifts need no new `sf_world_bevy` API.** `Streaming::shift_soon` and `guarded` are private with
   no accessor and must stay that way; `sf_scene_bevy` watches the public `WorldOrigin` resource's `cell`,
   which changes exactly on a shift. Do not add a `shift_guarded()` accessor speculatively.
4. **`covered` runs before `ready`, always** — it feeds the Fetch stage's model gather. What only the
   resolved table can name (the chain `CookedModelAsset`s) belongs in `ready`, through
   `AssetServer::get_recursive_dependency_load_state`, as `sf_lod_bevy`'s `FarHook::ready` does.
5. **`release` is called from three places and must be idempotent** — the near unload, the `to == Unloaded`
   sweep, and `Runtime::cancel`. A cell can legitimately see it twice.
6. **Chain geometry needs no loader.** `CHAINS_EXT == "sfmesh"` and `sf_model_bevy`'s `SfMeshLoader` already
   claims that extension; `baked_extensions()` carries the `scene.chains` entry anyway because
   `BakedReader` strips the extension through that map before the manifest lookup.
7. The `sf_scene` API to build on is `docs/briefs/scene-core-a5-a6.md` §2; `GpuBatchedMesh3d`'s contract is
   in `bevyrrowind/docs/test-pass-handoffs/s3a-gpu-batch.md` ("For S3b").
8. `SceneConfig::admits` exists but nothing calls it yet; B3 is where the worldspace filter is consulted
   (inside the hook, never in `stream.rs`).
