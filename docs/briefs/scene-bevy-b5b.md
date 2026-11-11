# Brief: static scene B5b, the scene skips what the world overlay moved or removed

Coordinator `scene-coord`, 2026-11-11. One deliverable: a placement the `WorldOverlay` removed (a mod or a save
deleted it) or placed (moved it) is not drawn by the static scene. It leaves `covered`, so streaming handles it
like any non-scene placement (a moved one spawns as an entity from the overlay-applied list, a removed one spawns
nothing), and its already-planned slots are written inactive at upload. Previous slices: B5a `019b13a`, B5a-2
`cc7f4ec`. Handoff: `docs/briefs/s3b-session-end-2026-11-11b.md`, slice 1.

**Out of scope:** an overlay change on a cell that is already resident (later; log it as a gap in your report),
the support gate (B5c), counters (B6).

## Where it stands

- `superfargoth/crates/sf_world/src/overlay.rs:39` `remove`, `:48` `is_removed`, `:55` `place`, `:86` `placed`.
  `sf_world_bevy::WorldOverlay(pub Arc<RwLock<PlacementOverlay>>)` (`sf_world_bevy/src/stream.rs:156`), always
  initialised by the streaming plugin (`lib.rs:98`). The precombine's test of the same thing: `stream.rs`
  `valid_members` (~:1760-1770, `o.is_removed(id) || o.placed(id).is_some()`).
- `superfargoth/crates/sf_scene_bevy/src/residency.rs:758-790` `SceneHook::covered` maps `plan.covered` locals to
  global ids. `:793-835` `upload` commits, stores `SpaceState::covered` (`:808`) and queues the plan's writes.
- `SceneSpace::promote(cell, local, &view)` (`sf_scene/src/space.rs:573-611`) returns the inactive `SlotWrite`s of
  one placement and records it in `Promotions`; `queue_slot_writes` (`residency.rs` ~:634-667) resolves them. A
  later write to the same slot wins (`upload.rs` `slot_runs`, stable sort).

## Read (line ranges only)

The ranges above, plus `sf_scene_bevy/src/residency_tests.rs:124-165` (`Fixture`, `f.indexed`), `:245-330`
(`fixture_with`), `:721-745` (`planned`, `commit`, `stream_cell_in`), and B5a's three tests at the end of the file
(their `promote` helper is the shape to copy).

## Do

1. One function `overlay_touched(world: &World, id: PlacementId) -> bool` in `residency.rs` (removed, or placed
   anywhere). No `WorldOverlay` resource: nothing is touched.
2. `covered` leaves touched ids out of its set.
3. `upload`, after `commit_in` and before queuing: for every `plan.covered` local whose id is touched, call
   `SceneSpace::promote` and append its resolved writes **after** the plan's own (so they win), and leave the id out
   of `SpaceState::covered`. Re-reading the overlay at upload (not only at `covered`) also catches a removal that
   landed between the two; say so in a comment.

## Tests (exact, headless, in `residency_tests.rs`)

1. `an_overlay_removed_placement_is_not_drawn`: remove placement 1 in the fixture's `WorldOverlay` before the cell
   streams in: `covered`'s answer lacks it, every slot of placement 1 ends with an inactive write as its last entry
   in `SceneUploads.writes`, and `SpaceState::covered[cell]` lacks it.
2. `an_overlay_moved_placement_leaves_the_scene`: `place` placement 2 with a new transform: same three facts.
3. `an_untouched_cell_is_drawn_as_planned`: the overlay touches only a placement of another cell (and stores a state
   blob for one of this cell's, if the API makes that one line): `covered` equals the full plan set and the writes
   equal `plan.writes` exactly.

## Rules

- **Hand off at ~80k tokens of context.** This slice is three small edits in one file plus tests: if you are past
  60k, stop on compiling code and report.
- Every cargo command goes through `bash bevyrrowind/tools/cargo_turn.sh <args>` from the repo root. It always exits
  0, so judge by output. Check `tasklist | grep -iE "cargo|rustc"` first (another session may be building; wait).
  Never `--workspace`, never `cargo fmt`. Filter cargo output to errors, warnings and `test result`.
- Touch only `sf_scene_bevy/src/residency.rs` and `residency_tests.rs`. Other sessions have uncommitted files
  elsewhere. No commit, no game launch.
- Floor: `test -p sf_scene_bevy --all-targets` 23 passed (yours adds 3); `clippy -p sf_scene_bevy --all-targets --
  -D warnings` clean. `sf_world_bevy` is untouched: don't run it.

## Report (≤ 30 lines)

Files with line ranges, the tests and counts, what was wrong in this brief, the resident-cell gap, and the context
you had used when you stopped.
