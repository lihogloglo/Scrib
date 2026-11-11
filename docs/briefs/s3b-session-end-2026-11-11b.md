# Static scene: state at the end of 2026-11-11 (second session), and the next slices

Written by session `scene-coord` on the owner's desktop. Previous handoff: `s3b-session-end-2026-11-11.md` (its
"Known gaps" list and brief template still apply; this file only adds to them). Everything below is committed on
`main`. Nothing has been pushed.

## What landed (headless-tested, nothing run in game, nothing baked)

| Commit | Slice | What |
|---|---|---|
| `019b13a` | B5a | `sf_scene_bevy::PromotePlacement { worldspace, cell, placement }` (global id) → `apply_promotions` (`SceneSystems::Promote`): global → local via `SpaceState::covered`, `SceneSpace::promote`, inactive 64 B slot writes through B4a's writer. Seam: `SceneCellCx::indexed` (kept in `Work::scene_ids` from the fetch) |
| `cc7f4ec` | B5a-2 | `sf_world_bevy::PromoteToEntity` (gameplay's entry point): the near `Group` keeps its load's placements/members/covered; the model is acquired for the group, spawned after the settle frame through the shared `spawn_placement`, then `CellScene::promoted` → `SceneHook` writes `PromotePlacement`. Entity first, so no missing frame |

Floor at `cc7f4ec`: `sf_scene_bevy` 23, `sf_world_bevy` 36, `sf_scene --features bake` 107 / 2 ignored; clippy
`-D warnings --all-targets` clean on both Bevy crates.

## New known gaps (logged here, not fixed)

7. **Precombine members can't be promoted** (`accept_promotion` drops them with a `debug!`): one bucket can't hide one
   member. Moot once the scene replaces the precombine (C2 interlock turns precombine off with `--scene`).
8. **First-seen material pipeline:** `promoted` fires in the spawn frame; if the entity's pipeline still compiles,
   the placement can be missing for a frame or so. The pipeline-warmup spec (`specs/pipeline-warmup.md`) is the fix.
9. Promotions spawn outside the origin-shift guard (one placement; acceptable, noted).
10. A promotion dropped after acceptance (uncooked, `prepare` spawner) keeps its id and model ref until unload.

## Coordinator rulings this session

- The plan's §7 holds: the scene never spawns. "Promotion + entity spawn" was split into B5a / B5a-2.
- **Drop `suppress_scene_placements`** (plan §8, the "belt to the braces"): `covered` already stops both spawns and
  model loads (`sf_world_bevy` test `scene_covered_placements_load_no_models_and_spawn_nothing`), and a wrapper on
  every spawner would now also block B5a-2's promoted spawn. C2's game wiring skips that line. Record it in
  DECISIONS.md with C2 (not done yet) and strike it from plan §8 then.

## Next slices, one deliverable each, in order

1. **B5b, overlay-touched placements** (gap 2 of the previous handoff): a placement the `WorldOverlay` removed or
   moved (`sf_world/src/overlay.rs:48` `is_removed`, `:86` `placed`; the precombine's test of the same thing is
   `stream.rs` `valid_members`, ~:1760) must not be drawn by the scene. Design: `SceneHook::covered` (has `&World`)
   leaves touched ids out of its set (so a moved one spawns as an entity from the overlay-applied placements, and a
   removed one spawns nothing), and `upload` writes their slots inactive. That is promotion without the entity:
   reuse `SceneSpace::promote` (it records them in `Promotions`). Per placement, not per cell (unlike the precombine).
   A runtime overlay change on a resident cell is the same path later (out of scope: log it).
2. **B5c, the support gate:** `GpuPreprocessingSupport::is_culling_supported()` lives in the render world
   (`vendor/bevy_render/src/batching/gpu_preprocessing.rs:152-154`); publish it back as a main-world
   `SceneSupported(bool)` (shape of `extract_chain_slabs`) and make the hook behave as off when false, plus one
   startup `warn!`. Includes the rewritten `promotion_is_safe_without_the_shader_fix` (plan §7 last paragraph).
   Check the timing: the first cell's `acquire` may run before the first extract.
3. **B6** counters (`scene/…`, plan §9; names from `SpaceStats`, B3/B4 brief §3 item 19), plus `scene/promoted`.
4. **C1** `bake_scene` + `bake_chains` in `bevyrrowind bake`.
5. **C2** `--scene`, `ScenePlugin` in the game, `Part::Scene`, precombine interlock (no `suppress_scene_placements`),
   visual-check entry, the owner run. The owner's first look at anything is here.
6. Lower: B4b, S1c-b, the origin-shift rewrite.

Agents this session ran 115-160k tokens of total context on these slices, more than the ~80k target: keep the
B5b/B5c briefs narrower (one function each, line ranges in the brief itself).

Open owner questions carried forward unchanged (slot budget past ~16 batches; interiors; `NO_LOD`; S3a3).
