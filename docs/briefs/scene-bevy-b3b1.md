# Brief: static scene B3b-1, batch entities and their ranges

Coordinator `scene-coord`, 2026-11-11. One deliverable: every batch a committed plan names exists as a
`GpuBatchedMesh3d` entity whose `ranges` match `SceneSpace::batch_ranges`, kept right through stream-in and
stream-out. Previous slice: B3a (`b992292`). Its report is summarised here; don't look for more.

## Where B3a left it (`superfargoth/crates/sf_scene_bevy/src/`)

- `residency.rs:79-81` `SpaceState::batches: Vec<Option<Entity>>`, indexed by `BatchId` (dense, never reused),
  never filled yet.
- `residency.rs:177-190` placeholder `CellUpload` / `SceneUploads` (B4's input; nothing writes them).
- `residency.rs:499-525` `SceneHook::upload`: removes the `Planned` load, `commit_in`, keeps the table. **Your
  hook point for stream-in.**
- `residency.rs:527-564` `SceneHook::release`: `cancel_in` (planned) or `plan_out` (resident). **Your hook
  point for stream-out**; `plan_out` returns `CellRelease { ranges, .. }`, today ignored.
- `chains.rs`: `ChainAssets { product, levels: Vec<Handle<Mesh>>, material: Option<..> }`, material always
  `None` (B3b-2).
- `residency_tests.rs`: a real streaming harness (bake store, real tables and chain products,
  `far_min_radius = f32::MAX`). Reuse its fixture; don't build a new one.

## Read (line ranges only)

- `docs/briefs/scene-bevy-b3-b4.md` lines 183-287 (`GpuBatchedMesh3d` and the range list), 385-390 (item 12:
  `mesh` must be one of the ranges' meshes and is re-pointed when ranges empty and refill), 582-586 (step 5:
  spawn order and the zero-fill), 610-616 (step 8: range mapping), 619-623 (step 10: despawn policy).
- `superfargoth/crates/sf_scene/src/space.rs`: `RangeSpec` :168, `CellPlan::batches` :190 / `ranges` :205,
  `CellRelease::ranges` :222, `batch_ranges` :626.
- The vendored component itself only if the brief's quote isn't enough: `superfargoth/vendor/bevy_pbr/src/
  gpu_instance_batch/mod.rs:124-241`.

## Do

In `upload`, after `commit_in`: for each `(id, key)` in `plan.batches` with no entity yet, spawn it exactly as
step 5 says (`max_capacity = config.batch_capacity`, `ranges` empty for that tick, no `Mesh3d`, no
`NoCpuCulling`, no `NotShadowCaster`). The material is `MeshMaterial3d(Handle::default())` with a `TODO(B3b-2)`.
Push a zero-fill request for its whole capacity into `SceneUploads` as a small new typed entry, because B4
consumes it. Then rebuild `ranges` for every id in `plan.ranges` per step 8, re-pointing `mesh` at the first
range's mesh. In `release` after `plan_out`, rebuild `ranges` for `CellRelease::ranges`. A batch whose ranges
stay empty for more than `frames_in_flight` frames is despawned (count with `SceneFrame`); one that refills
before then keeps its entity.

Out of scope: materials and bind-group ids (B3b-2), writing `scene_bytes` / `writes` (B4), promotion, counters.

## Tests (exact, headless, in `residency_tests.rs`)

1. One committed cell: entity count = `plan.batches.len()`; each entity's ranges equal `batch_ranges(id)`
   mapped per step 8; `mesh` = the first range's mesh; one zero-fill entry per new batch.
2. Two cells sharing a batch, then release one: the shared batch keeps its entity, its ranges equal
   `batch_ranges` again, and `mesh` is still a mesh named by its ranges.
3. Release everything: entities survive `frames_in_flight` frames, then despawn; a cell streaming back in
   before then reuses the entity (same `Entity`, no second zero-fill).

## Rules

- **Hand off at ~80k tokens of context**: stop on code that compiles, run what tests you have, and report
  what's left. A fresh agent continues.
- Every cargo command goes through `bash bevyrrowind/tools/cargo_turn.sh <args>` from the repo root. It always
  exits 0, so judge by output. Check `tasklist | grep -iE "cargo|rustc"` first. Never `--workspace`, never
  `cargo fmt`. Filter cargo output to errors, warnings and results.
- Other sessions have uncommitted files in the tree (menus: `mw_exterior_bevy`, root `Cargo.toml`,
  workspace-hack). Don't touch them. No commit (the coordinator commits), no game launch.
- Floor: `test -p sf_scene_bevy --all-targets` 10 passed (yours adds 3);
  `clippy -p sf_scene_bevy --all-targets -- -D warnings` clean.

## Report (short, ≤ 40 lines)

Files with line ranges, the 3 tests and their counts, anything in this brief that was wrong, what B3b-2 and B4
inherit, and the context you had used when you stopped.
