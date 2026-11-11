# Static scene: state at the end of 2026-11-11, and the next slices

Written by session `scene-coord` on the owner's desktop. Previous handoff: `s3b-session-end-2026-11-10.md`
(its "next slices" 1, 2, 3 and 6 are now done). Everything below is committed on `main`. Nothing has been pushed.

## What landed today (all headless-tested, nothing run in game, nothing baked)

| Commit | Slice | What |
|---|---|---|
| `28a109d` | S1c-a | format v3 instance contract closed: `ALPHA_MASKED`/`WIND` dropped (bits 1 and 4 reserved, rejected on read); `material_override` = future 1-based per-cell override index |
| `28a109d` | ruling | **plan before `covered`** (scene-runtime-plan.md §5.3): the fetch waits for table, chains, slabs and `plan_in`; overflow and failed chains spawn as entities |
| `b992292` | B3a | `sf_scene_bevy` residency: `poll_cell_loads` (Table → Chains → Slabs → Planned/Fallback), `SceneHook: CellScene`, `extract_chain_slabs`; `covered` takes the cell's indexed ids; `ChainSlabs::slabs_keyed` |
| `1804108` | B3b-1 | batch entities (`GpuBatchedMesh3d` + `SceneBatch`), ranges rebuilt on stream-in/out, zero-fill queued, empty batches despawn after `frames_in_flight` |
| `bc28b59` | B3b-2 | chain materials (`ChainMaterials`, per `material_key`, `matclass::fold` per `Streaming` config); bind group from `RenderMaterialBindings` in the batch key |
| `8101d76` | B4a | slot writer: `BatchSlotWrite` queued in `upload`, `extract_scene_uploads`, `publish_scene` (zero-fills, then coalesced runs) |

Regression floor at `8101d76`: `sf_scene --features bake` 107 passed / 2 ignored; `sf_world_bevy` 33;
`sf_scene_bevy` 19; clippy `-D warnings --all-targets` clean on all three.

**Compile-only, first exercised in game:** `extract_chain_slabs` (slab and bind-group values), `extract_scene_uploads`,
`publish_scene`. Headless apps have no render world.

## Known gaps (logged, not fixed)

1. **Origin shift:** batch outputs are placed against `WorldOrigin.cell` at upload (last frame's value) and go stale
   on a shift; the cell's worldspace isn't checked against the origin's (plan §5.4 defers the rewrite).
2. **Overlay-removed placements** (a mod deleting a reference) are still drawn by the scene: belongs to B5's suppression.
3. Without a render app, nothing drains `SceneUploads` (`zero_fills`, `writes` grow): matters once `ScenePlugin` runs
   in a headless game app (C2).
4. Zero-fills upload `capacity × 64 B` of zeros; an encoder `clear_buffer` would be cheaper.
5. A chain material re-prepared after planning (texture hot reload) may move bind group; the port logs it, nothing re-keys.
6. `CellScene::bytes` has no `&World`, so the scheduler's estimate is 0; the true bytes come back from `upload`.

## Next slices, one deliverable each, in order

Brief format that worked today (65-75k context per slice): a file in `docs/briefs/scene-bevy-<slice>.md` with "where
it stands" (file:line), a read list of **line ranges only**, one "Do", 3 exact tests, rules, ≤ 40-line report; the
agent prompt is three sentences pointing at the file. Templates: `scene-bevy-b3b1.md`, `-b3b2.md`, `-b4a.md`.

1. **B5a** promotion: `SceneSpace::promote` → a one-slot `BatchSlotWrite` `active: false` + the entity spawn (old brief
   `scene-bevy-b3-b4.md` and plan §7).
2. **B5b** `suppress_scene_placements` + the overlay gap (2 above).
3. **B6** counters (`scene/…`, plan §9), incl. `scene/batches/count` and `scene/slots/batch_capacity`.
4. **C1** `bake_scene` + `bake_chains` in `bevyrrowind bake` (closes the format window).
5. **C2** `--scene` switch, `ScenePlugin` in the game, `Part::Scene`, precombine interlock, visual-check entry, and
   the owner run. Then the S3b numbers (expect a CPU win, GPU flat or slightly worse until S3a3).
6. Lower: **B4b** the per-space scene buffer (S3a3 input), **S1c-b** decode LOD0 only for eligible models, the origin
   shift rewrite.

Open owner questions carried forward: total slot budget if batches exceed ~16; interiors (default out); `NO_LOD`;
whether S3a3 is worth building once S3b's numbers exist.

## Docs debt

CHANGELOG, DECISIONS and ROADMAP 1.16 lines for today were committed with this file (only this session's hunks; the
menus/clock/bake sessions had uncommitted edits in the same files).
