# Brief O1: static scene statics stay put across a floating-origin shift (option (a))

Coordinator: scene-coord, 2026-11-11. Previous handoff: `docs/briefs/s3b-session-end-2026-11-11b.md` item 5 (read
only that item). One deliverable: **after an origin shift, every resident cell's batch slots are rewritten in the
shift frame**, plus its tests. Headless only; never launch the game; don't commit (the coordinator does).

## The bug

Batch output slots hold render-space transforms: `queue_slot_writes`
(`superfargoth/crates/sf_scene_bevy/src/residency.rs:633-665`) adds `sf_scene::cell_offset(view.origin_cell(),
WorldPos::new(frame, ZERO))` with `frame = WorldOrigin.cell` at upload time. `WorldOrigin` changes every ~256 m of
travel (`sf_world_bevy/src/origin.rs:111-122`, `update_world_origin`, `PostUpdate` after
`TransformSystems::Propagate`; grid `sf_world_bevy/src/coords.rs:26`, `GRID_CELL = 256`; the game does not override
it, checked). Nothing rewrites the slots afterwards, so every resident static draws 256 m off after a shift.
`materialize.wesl` uses `world_from_local` raw, so there is no per-batch offset to lean on (that is option (b),
a later vendor-patch slice: out of scope).

## Ruled out (don't revisit)

- Spreading the rewrite over frames (`SceneConfig::shift_rewrite_bytes`, plan §5.4): it would tear for ~7 frames.
  **Remove the field** (`sf_scene_bevy/src/lib.rs:81-82, 87`; nothing reads it).
- Ordering against writes already queued this frame: no work needed. `slot_runs` sorts **stably**
  (`upload.rs:88-102`), and `extract_scene_uploads` appends new writes after kept ones (`upload.rs:115-128`), so a
  write appended later in the frame wins, including over runs kept for a missing reservation. Say so in a comment,
  don't re-engineer it.

## What to build

1. **`sf_scene` (Bevy-free): `SceneSpace::cell_writes(cell, &StaticCellTableView) -> Result<Vec<SlotWrite>,
   SceneError>`** in `superfargoth/crates/sf_scene/src/space.rs`: the current state of every slot a resident cell
   holds. Same `active` rule as `plan_in` (`space.rs:472-497`: a placement is active only if every part got a slot),
   **and** inactive for placements in `CellResidency::promotions` (B5a/B5b promotions, overlay-touched ones). The
   slot → batch lookup is the one `promote` uses (`space.rs:590-606`: chain + level name the block, the block names
   the batch); factor that lookup into one private helper shared by both, don't copy it. `NotResident` for an
   unknown cell.
   Test (in `space.rs`'s tests): right after `commit_in`, `cell_writes` equals the plan's `writes` (sorted the
   same way); after a `promote`, the promoted placement's writes are `active: false` and the rest unchanged; an
   overflow fixture keeps the sibling parts inactive.
2. **`sf_world_bevy`: a public set for `update_world_origin`** (`origin.rs:98`), e.g. `WorldOriginSystems`, so the
   scene can order after it. One line plus the set type; nothing else in that file.
3. **`sf_scene_bevy`: `rewrite_on_origin_shift`** (new module `shift.rs`, registered in `lib.rs`'s plugin next to
   the others at `:129-139`): `PostUpdate`, after `WorldOriginSystems`. Keep a `Local` of the last
   `(worldspace, cell)` seen. When it changes, for the `SpaceState` of `WorldOrigin.worldspace` (only that one: it
   is the space being rendered; a worldspace switch is the same event), call `cell_writes` for every resident cell
   and push the results through `queue_slot_writes` with the **new** frame into `SceneUploads::writes`. The first
   frame (no previous value) does nothing. Count `scene/shift/rewrites` (cells) and `scene/shift/rewrite_bytes` the
   way the crate counts others (`count(...)`, see `residency.rs:626`), and one `debug!` per shift.
   Test (`residency_tests.rs`, template `a_committed_cell_queues_one_resolved_write_per_plan_write` at
   `:992-1025`, helpers `fixture` `:245`, `stream_cell_in` `:735`): stream a cell in, clear `SceneUploads::writes`,
   set `WorldOrigin` to another cell (same worldspace), run one update; assert one write per resident slot, each
   with `gpu_instance(row, new_offset, active)` bytes; a promoted placement stays inactive; a second update with
   no change queues nothing. If the fixture has no `WorldOrigin`, insert one.
4. Docs, short: `upload.rs:17-20` "Known gap" becomes what is built; `superfargoth/docs/specs/scene-runtime-plan.md`
   §5.4 (`:573-583`) rewritten in 3-5 lines (shift-frame rewrite, why not spread, option (b) later, ~3.5 MB at 55k
   instances in one frame).

## Verify (one build at a time: `bash bevyrrowind/tools/cargo_turn.sh <args>`; filter output to errors/summary)

- `test -p sf_scene` and `test -p sf_scene --features bake` (107 passed / 2 ignored before), `test -p sf_scene_bevy`
  (26 before), `test -p sf_world_bevy` (37 before).
- `clippy -p sf_scene -p sf_scene_bevy -p sf_world_bevy --all-targets -- -D warnings`.

## Hand-off

Stop by ~80k tokens of context at a compiling state. Report: files and functions changed, test counts, the
`cell_writes` signature, anything that didn't match this brief (line numbers drift). Register in
`.sessions/scene-o1.md` (scope: the files above) and delete it when done.
