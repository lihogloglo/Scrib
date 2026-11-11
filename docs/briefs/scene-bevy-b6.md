# Brief B6: `scene/*` occupancy gauges, and the perf summary shows them

Coordinator: scene-coord, 2026-11-11. Spec: `superfargoth/docs/specs/scene-runtime-plan.md` `## 9. Counters` (read
only that section, ~12 lines). One deliverable: **every frame, the static scene publishes its occupancy as `scene/*`
gauges, and `perf suite` summaries show the key ones**, plus its test. Headless only; never launch the game; don't commit.

## What exists (don't rebuild)

- Event counters already counted with `count(...)` (`superfargoth/crates/sf_scene_bevy/src/residency.rs:367-371`,
  uses `Counters::add_named`): `scene/cells/{missing,failed}`, `scene/chains/failed`, `scene/slots/overflow`,
  `scene/overflow/cells` (`residency.rs:523-626`), `scene/shift/{rewrites,rewrite_bytes}` (`shift.rs:77-79`). Keep them.
- `SceneSpace::stats() -> SpaceStats` (`superfargoth/crates/sf_scene/src/space.rs:244-264`, `stats` at ~:663): cells,
  `scene: SlotStats` (`sf_scene/src/alloc.rs:48-58`: high_water, live_slots, live_ranges, free_slots, pending_slots…),
  `chains: ChainStats` (`sf_scene/src/chains.rs:136-142`: live, pending, high_water), batches, batch_live_slots,
  batch_high_water, batch_capacity, overflow_cells, overflow_instances, promoted.
- Gauges are set with `Counters::set_named(name, unit, value)` (`superfargoth/crates/sf_diag/src/counters.rs:186`),
  units `unit::COUNT` / `unit::BYTES` (`counters.rs:15-17`). Template for a `Last` gauge system behind
  `resource_exists::<DiagCounters>`: `sf_world_bevy::matclass::track_static_materials` and where it is registered
  (grep `track_static_materials` in `superfargoth/crates/sf_world_bevy/src/lib.rs`). `DiagCounters` access as in
  `sf_scene_bevy/src/shift.rs` (`counters.as_deref().map(|c| &*c.0)`).

## What to build

1. `sf_scene_bevy`: `publish_scene_gauges` in a new small module (`gauges.rs`), `Last`,
   `.run_if(resource_exists::<DiagCounters>)`, registered in `ScenePlugin::build` (`lib.rs:113-138`). Sum `stats()`
   over every `SpaceState` in `SceneSpaces` and set (names from plan §9, mapped to the fields that exist):
   `scene/cells/resident`, `scene/chains/{live,pending,high_water}`, `scene/slots/{scene_live,scene_bump}`
   (= `scene.live_slots`, `scene.high_water`), `scene/slots/{batch_live,batch_bump,batch_capacity}`,
   `scene/slots/waste` (= `batch_high_water - batch_live_slots`), `scene/batches/count`, `scene/promoted`,
   `scene/overflow/instances`, and `scene/slots/batch_capacity_bytes` (= `batch_capacity × SLOT_BYTES`,
   `upload.rs`, unit BYTES: the VRAM gauge). Set them every frame, zeros included (a gauge that stops being set
   reads stale). Skip plan §9 names with no source (`chains/loading`, `cells/covered`, `batches/ranges|instances`,
   `upload/bytes`, `plan/ms`): list them in the module doc as not built.
2. `bevyrrowind/apps/bevyrrowind/src/perf/cli.rs`: add `GaugeColumn`s next to the existing ones (~`:373-395`, the
   list with "entities", "batch sets"): `scene cells` → `scene/cells/resident`, `scene batches` →
   `scene/batches/count`, `scene live slots` → `scene/slots/batch_live`, `scene slot MB`? (only if the summary has
   a way to scale; otherwise the bytes gauge as is). Check how a gauge absent from a run is shown (the `--scene`-off
   run will have none): it must not fail the summary.
3. Plan §9: 2-3 lines "as built" (what is set, what is not).

## Test

`residency_tests.rs` (fixture `fixture` ~`:245`, `stream_cell_in` ~`:735`): with a `DiagCounters` resource in the
fixture app, stream one cell in, run a frame, read the counters back (find how other tests read a named counter:
grep `DiagCounters` in `superfargoth/crates/*/src/*tests*.rs`) and assert `scene/cells/resident == 1`,
`scene/batches/count == plan's batch count`, `scene/slots/batch_live == plan.writes.len()`; after streaming it out
(the fixture's release helper), `scene/cells/resident == 0`. Exact assertions: these are counts, not taste.

## Verify (one build at a time: `bash bevyrrowind/tools/cargo_turn.sh <args>`; it now returns cargo's real exit code)

- `test -p sf_scene_bevy` (27 before), `test -p bevyrrowind --bins` (7 before).
- `clippy -p sf_scene_bevy -p bevyrrowind --all-targets -- -D warnings`.
- If the app crate fails to build because of another session's in-progress file (tex-t4 was editing
  `mw_assets/src/cooked.rs`), report it; don't fix it.

## Hand-off

Stop by ~80k tokens. Report files, functions, the gauge list as built, test counts, anything that differed.
Register in `.sessions/scene-b6.md`; delete it when done. Touch only the files above.
