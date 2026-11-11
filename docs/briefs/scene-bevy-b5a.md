# Brief: static scene B5a, promotion (the scene stops drawing a placement)

Coordinator `scene-coord`, 2026-11-11. One deliverable: gameplay sends `PromotePlacement { worldspace, cell,
placement }` with the **global** `sf_world::PlacementId`, and the scene queues one `BatchSlotWrite` with
`is_active = 0` for every slot of that placement, so the GPU stops drawing it from the next publish. Previous slices:
B3a `b992292`, B3b-1 `1804108`, B3b-2 `bc28b59`, B4a `8101d76`.

**The scene never spawns** (scene-runtime-plan.md §7: "the scene side never spawns, it only stops drawing"). Spawning
the replacement entity is slice **B5a-2** (a scene-covered placement's model was never gathered, so it is an
on-demand load in `sf_world_bevy`; that slice sends `PromotePlacement` once its entity is revealed, so no pop).
**Out of scope:** the entity spawn, `suppress_scene_placements` / `SceneSupported` / the overlay gap (B5b), counters
(B6), a dedicated 4-byte `is_active` write (B4a's 64 B slot write is reused; the cost is the same order).

## Where it stands

- `sf_scene/src/space.rs:573-611` `SceneSpace::promote(cell, placement: sf_scene::PlacementId, &view)` exists and is
  tested (`:1003-1027`): one-way, idempotent, `NotResident` for a cell not committed. **Its `PlacementId` is the
  cell-local one** (`sf_scene::PlacementId`, the table's index into the cell's *indexed* placements), not
  `sf_world::PlacementId`.
- The local → global map exists only inside `SceneHook::covered` (`sf_scene_bevy/src/residency.rs` ~:700-725, from
  its `indexed` argument) and is thrown away. Streaming builds `indexed` in the fetch task
  (`sf_world_bevy/src/stream.rs:723-733`, the `ids` Vec), hands it to `covered` (`:786-812`), then drops it.
- `SceneCellCx` (`sf_world_bevy/src/scene.rs:34-43`) is what `upload` gets; it is built at `stream.rs:1357-1362`.
  `Work` is at `stream.rs:260+`.
- `SpaceState::tables` (`residency.rs:91-93`) keeps each resident cell's table for promotion; `release`
  (`:770-811`) drops it on stream-out.
- `queue_slot_writes(world, state, cell, plan_writes, frame)` (`residency.rs:629-665`) resolves `SlotWrite`s to
  `BatchSlotWrite`s. It reads only `Assets<SceneTableAsset>` from `world`.
- `slot_runs` (`upload.rs:88-104`) sorts stably, so a promotion queued after a same-frame commit's `active: true`
  write of the same slot wins. Pending writes of a batch with no reservation are kept and published first.
- `SceneSystems::Promote` is configured, after `Residency`, with nothing in it (`lib.rs:99-106`, `:118-137`).

## Read (line ranges only)

- `superfargoth/docs/specs/scene-runtime-plan.md:653-664` (§7, first paragraph only).
- `superfargoth/crates/sf_scene/src/space.rs:573-611`, `sf_scene/src/eligibility.rs:124-160`.
- `superfargoth/crates/sf_scene_bevy/src/residency.rs:75-110`, `:629-811`; `lib.rs:30-60`, `:99-137`.
- `superfargoth/crates/sf_world_bevy/src/scene.rs:30-100`; `stream.rs:250-275`, `:715-740`, `:780-850`,
  `:1345-1368`.
- Tests to copy: `sf_scene_bevy/src/residency_tests.rs:76-124` (placements, `overflow_split`), `:245-260`,
  `:721-745` (`planned`, `commit`, `stream_cell_in`), `:993-1022` (B4a test 3, the shape of your test 1). Every
  `SceneCellCx { .. }` literal in the tree (`grep -rn "SceneCellCx {" superfargoth/crates`) needs the new field.

## Do

1. **Seam:** `SceneCellCx` gains `indexed: &'a [PlacementId]` (the cell's ids in index order, as `covered` gets
   them; empty when the load has no scene). `Work` keeps the fetch's `ids` (an `Arc<[PlacementId]>`, only when
   `w.scene`) until `reveal` passes it. Update the trait docs of `covered`/`upload` in one sentence each.
2. **Map:** `upload` stores, next to `state.tables`, the committed cell's covered placements as global → local
   (`plan.covered` locals through `cx.indexed`); `release` drops it with the table.
3. **Promote:** `#[derive(Message)] pub struct PromotePlacement { worldspace, cell, placement }` (registered by
   `ScenePlugin`), and `apply_promotions` in `SceneSystems::Promote`: global → local, `SceneSpace::promote`,
   `queue_slot_writes` (change its first argument to `&Assets<SceneTableAsset>` so this is an ordinary system),
   append to `SceneUploads::writes`. Unknown worldspace, cell not resident, or a placement the cell doesn't cover
   (overflowed, failed chain, not a static): `log::debug!` and nothing queued, never a panic.

## Tests (exact, headless, in `residency_tests.rs`)

1. `promotion_queues_one_inactive_write_per_slot`: with `chain_a_and_b`, promote placement 1 (two parts) by its
   global id: `SceneUploads.writes` gains exactly 2 entries, each `!active()`, with the batch entity, batch and slot
   of that placement's `plan.writes` entries, and the instance bytes of `gpu_instance(row, offset, false)`.
2. `promotion_is_one_way_and_skips_what_the_scene_does_not_draw`: promoting the same placement again queues nothing;
   with `overflow_split` and capacity 4, an overflowed placement (`plan.overflowed`) queues nothing; `stats().promoted`
   counts 1.
3. `promotion_of_a_cell_that_is_not_resident_is_dropped`: a cell still loading (planned, not committed), and the
   same cell after `release`: no write, no panic, and after re-streaming the cell its placements are written
   `active: true` again (promotions die with the cell).
Also: `sf_world_bevy`'s floor stays green (the seam field), and one assertion in an existing `sf_world_bevy`
scene test that `upload` receives the cell's `indexed` ids in index order (`tests.rs` ~:620-700, the test
`CellScene` there logs calls).

## Rules

- **Hand off at ~80k tokens of context**: stop on code that compiles, run what tests you have, and report what's
  left.
- Every cargo command goes through `bash bevyrrowind/tools/cargo_turn.sh <args>` from the repo root. It always exits
  0, so judge by output. Check `tasklist | grep -iE "cargo|rustc"` first (another session may be building; wait).
  Never `--workspace`, never `cargo fmt`. Filter cargo output to errors, warnings and `test result`.
- Other sessions have uncommitted files (menus: `mw_exterior_bevy`, root `Cargo.toml`, workspace-hack; clock:
  `sf_ids`, `sf_rpg`, `sf_ai`, `sf_effects`). Don't touch them. No commit, no game launch.
- Floor: `test -p sf_scene_bevy --all-targets` 19 passed (yours adds 3); `test -p sf_world_bevy --all-targets` 33;
  `test -p sf_scene --features bake` 107 passed / 2 ignored (untouched, run once); `clippy -p sf_scene_bevy -p
  sf_world_bevy --all-targets -- -D warnings` clean.

## Report (≤ 40 lines)

Files with line ranges, the tests and counts, what was wrong in this brief, what B5a-2 and B5b inherit (the
message's exact shape, where the map lives), and the context you had used when you stopped.
