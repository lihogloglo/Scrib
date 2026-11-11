# Brief: static scene B4a, the batch-slot writer (what makes statics draw)

Coordinator `scene-coord`, 2026-11-11. One deliverable: the main world queues each cell's batch-slot writes,
and a render-world `publish_scene` writes them into the batches' output buffers, so a committed cell's
instances are live on the GPU. Previous slices: B3a `b992292`, B3b-1 `1804108`, B3b-2 `bc28b59`.

**Out of scope (B4b, later):** the per-space scene buffer (write 1 of the old brief's §5.3, `CellPlan::scene_bytes`).
Nothing reads it under M1; it is the S3a3 cull's input. Also out: promotion, counters, origin-shift rewrites.

## Where it stands (`superfargoth/crates/sf_scene_bevy/src/residency.rs`)

- `:202-227`: `CellUpload`, `SceneUploads { cells, writes, zero_fills }`. `upload` (around :680-700) fills
  `zero_fills` only; `TODO(B4)` at :692 asks for `plan.writes`.
- `SceneBatch { space, batch }` component on each batch entity maps the entity back to its batch.
- `sf_scene/src/space.rs:151-159`: `SlotWrite { batch, slot, index, active }`. `index` is the instance's row in
  the cell's table; the transform comes from that row plus the cell origin (`sf_scene/src/lib.rs:52-55`
  `cell_offset`).

## Read (line ranges only)

- `docs/briefs/scene-bevy-b3-b4.md` lines 288-308 (§2.3 `GpuMeshInstance`, 64 B, the affine-transpose layout and the
  `is_active` offset) and 701-738 (§5.3: writes 2 and 3, the `Publish` set, skip a batch with no reservation yet,
  `frames_in_flight`, the origin-shift gap). For registration and extraction, lines 646-674 and 676-683 (copy
  `origin.rs`'s shape, hand-registered `extract_resource`-style, **not** `ExtractResourcePlugin`).
- `superfargoth/vendor/bevy_pbr/src/gpu_instance_batch/mod.rs:79-100` (`GpuMeshInstance`, `GpuInstanceBatchSystems`)
  and `:324-357` (`GpuInstanceBatchReservations::get(..).output_buffer()`).

## Do

1. In `upload`, push the plan's `writes` into `SceneUploads` resolved to everything the render world needs: the
   batch entity (`MainEntity`), the slot, a ready `GpuMeshInstance` (render-space transform from the table row +
   cell origin, per §2.3), and `active`.
2. Keep the conversion pure and testable: one function from (table row, cell origin) to `GpuMeshInstance`, and one
   that coalesces consecutive slots of one batch into runs.
3. `extract_scene_uploads` (ExtractSchedule) takes `SceneUploads` out of the main world. `publish_scene`
   (RenderGraph schedule, `GpuInstanceBatchSystems::Publish`) does the zero-fills first, then the coalesced slot
   writes, as `RenderQueue::write_buffer` calls. A batch with no reservation yet keeps its work for the next frame
   (don't drop it).

## Tests (exact, headless)

1. The conversion: a known row and cell origin give the exact `GpuMeshInstance` bytes (transform, `is_active`
   at offset 48).
2. Coalescing: slots `[3,4,5,9]` of one batch and `[4]` of another give three runs.
3. After a cell commits, `SceneUploads.writes` holds exactly one entry per `plan.writes` with the right entity,
   slot and `active`, including an overflowed placement's `active: false` parts.
The render systems are compile-only headless. Say so in the report.

## Rules

- **Hand off at ~80k tokens of context**: stop on code that compiles, run what tests you have, and report what's
  left.
- Every cargo command goes through `bash bevyrrowind/tools/cargo_turn.sh <args>` from the repo root. It always exits
  0, so judge by output. Check `tasklist | grep -iE "cargo|rustc"` first. Never `--workspace`, never `cargo fmt`.
  Filter cargo output.
- Other sessions have uncommitted files (menus: `mw_exterior_bevy`, root `Cargo.toml`, workspace-hack). Don't touch
  them. No commit, no game launch.
- Floor: `test -p sf_scene_bevy --all-targets` 16 passed (yours adds 3);
  `clippy -p sf_scene_bevy --all-targets -- -D warnings` clean.

## Report (≤ 40 lines)

Files with line ranges, the tests and counts, what was wrong in this brief, what B4b and B5 inherit, and the
context you had used when you stopped.
