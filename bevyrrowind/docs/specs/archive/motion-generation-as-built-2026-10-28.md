# Motion generation: as built (MG1, MG3, MG4, MG5, MG6, MG7a, MG10a, 2026-10-28)

Archived 2026-11-26 from [motion-generation.md](../motion-generation.md) (AN25 size cleanup); sections unchanged.

## As built (MG1, 2026-10-28)

- `bevyrrowind/tools/motiongen/` (now [scrib-motiongen](https://github.com/scrib-tools/scrib-motiongen), run from its root; README has setup/env): from `bevyrrowind/tools`, `C:/mbtest/kimodo/.venv/Scripts/python.exe -m motiongen selftest | plan | run | bench | joints`. Selftest all ok; `run` resumable, writes BVH + `.prov.json` + `jobs.jsonl`.
- Speed: model load 1.6 s; prompt embeddings cached on disk, encoder (CPU) loaded only on a miss (31 s load + ~7 s/prompt), then freed. 6 s clips, fp32: steps 100 = 3.7 s/clip, 50 = 2.1, 25 = 1.1; batch 1/2/4/8 at 50 steps = 2.3/2.0/2.1/2.1 s/clip (VRAM 1.2-1.5 GB); fp16 1.76 (jerk 41 -> 53 m/s^3), bf16 2.3 (jerk 284: avoid); post-process costs ~0.01 s/clip (skate 0.045 -> 0.038 m/s with it). Target (<= 10 s) met ~4x.
- Defaults: steps 50, batch 8, fp32, post-process on. Steps 25 loses the pelvis constraint (err 0.12 m vs 0.006 m at 50).
- Constraints: `pelvis_height` = Kimodo `root_y_pos` only (err ~6 mm), `root fixed|fixed_xz|path`, hand wrist position (custom class; err ~0.3 m, i.e. weakly followed: next lever is Kimodo's own end-effector set with a full reference pose).
- BVH (for MG3): Kimodo exporter, cm, +Y up, facing +Z, default rest = standard T-pose (`--bvh-rest seed` for BONES-SEED rest); wrapper `Root` joint + `Hips` with absolute translation channels (Hips OFFSET is a dummy 0,100,0); joints in `tools/motiongen/soma77_joints.txt`.
- Gaps: no seamless loops (MG8), `start_pose` rejected, hand constraint weak, encoder on GPU infeasible (16 GB bf16 > 8 GB; 4-bit needs bitsandbytes, not installed, not worth it with the cache). The LLM2Vec load prints LoRA MISSING/UNEXPECTED key warnings (inside Kimodo's loader, unchanged).
- Bench output: `animation packs/generated/mg1-test/_bench/bench.json`; next: `run motiongen/recipes/seed.toml --out "../../animation packs/generated"`.

## As built (MG4, 2026-10-28)

- `bevyrrowind furniture-census [--json <path>]` (default `animation packs/generated/bands.json`, `placeholder: false`, the `default_bands.json` keys plus `shelf`; `recipe.load_bands` reads it). Logic: `mw_bake::census` (Bevy-free); test `mw_bake/tests/census_real_install.rs` (skips without install; `--nocapture` prints the tables).
- Records: `STAT`/`ACTI`/`CONT` whose `furn_*`/`active_*` id (or `furn_*` model) has a word bed/bedroll/hammock/bunk, counter/bar, table/desk, chair/stool/bench/throne, shelf/bookshelf (`door` excluded: bar flaps). Vanilla+expansions: 32 seats, 28 tables, 12 counters, 46 beds, 28 shelves; ~10.3k placements, 97 % at `XSCL` 1.00.
- Height: horizontal (|n.y| ≥ 0.9) surfaces of `collision_of` above the model's lowest point, 2 cm bins merged; highest with both sides ≥ 0.25 m, fill ≥ 30 % of its rectangle, between 0.1 m and the kind cap (seat 0.8, table/bed 1.2, counter 1.5). No such surface (chairs collided as one tall box, canopy beds) → render geometry.
- Bands (one sample per placement, 5 cm bins, merge < 10 cm apart within a 15 cm span, < 3 % share folds, ≤ 4 per kind): seat 0.14 (prayer stool, 780 refs) / 0.48 (1504) / 0.54 (2133) / 0.64 (benches, stools; 770); table 0.84 (1632) / 0.98 (320); counter 0.94 (605); bed 0.22 (bedrolls, 474) / 0.52 (45) / 0.64 (173) / 0.74 (694); shelf 0.33 / 1.50 / 1.89 / 2.35.
- Races (`RADT` height factors m/f): High Elf 1.10, Nord 1.06, Orc 1.05, Argonian 1.03/1.00, Redguard 1.02/1.00, Breton and Khajiit 1.00/0.95, Wood Elf 0.90/1.00, others 1.00. In the `note` only (the planner rejects unknown top-level keys); `reference_height_m` stays null.
- Scale: Morrowind furniture is ~15-30 % above real-world heights (tables 0.84 vs ~0.74 m, chair seats 0.48-0.54 vs ~0.45, beds 0.74 vs ~0.55), consistent with its tall actors; retarget/plan against these, not real-world numbers.

## As built (MG3, 2026-10-28)

- Source `generated` in `mw_graphs/data/animation/sources.ron`: cm (`unit_scale: 0.01`), +Y up, `root_from_pelvis`
  (Kimodo's `Root` is a static wrapper), `private_only`, `License::GeneratedNvidiaOpenModel`
  (`LicenseRef-generated-nvidia-open-model`), `files: []` (no slots yet).
- Export with `--bvh_standard_tpose`: the zero pose is a T-pose, auto-align needs no facing turn. Kimodo's default
  (BONES-SEED rest, every bone along +X) puts the retargeted ankles ~1.17 m up: unusable. Re-export from a run's `.npz`
  with `kimodo.exports.bvh.save_motion_bvh(..., standard_tpose=True)` (same motion, bit-for-bit hierarchy).
- `sf_character::auto_map`: SOMA-77 maps cleanly (`LeftLeg` thigh by hierarchy, `LeftShin` lower leg, Spine1/Spine2/Chest,
  Neck1); a non-thumb finger with four joints drops its base (metacarpal) unless the last is a childless end (Mixamo).
- In process (MG5): `mw_bake::anim::HumanoidTarget::new(skeleton)` and `cook_library_clip(source, path, clip, rules,
  target) -> Result<Clip, String>`, equal to the baked `anim.retarget` bytes (`generated_real_install.rs`).
- Measured (`measure_generated_feet`, 2.97-5.97 s clips): standing ankle error 3-7 mm (stand still), max 1-4 cm
  (sweep, lean); `base_animkna` ankle 0.16 m below its rest height: digitigrade legs, no beast-leg fix (as LAFAN1/UAL).

## As built (MG5, 2026-10-28)

- Command (dev builds): `bevyrrowind clip-preview --clip <file|name in sources folder> [--name <clip>] [--source <id>]
  [--batch <file: path[|name[|source]] per line>] [--npc <NPC_>] [--camera front|side|orbit] [--out shots/clip-preview]
  [--fps 30] [--size 960x540] [--ffmpeg ffmpeg] [--keep-frames]`. Default NPC `draren thiralas` (Dunmer commoner).
- Code: `mw_viewers_bevy::clip_preview` (feature `clip_preview`, on in the app's `dev`); Bevy-free `clip_preview::cook`
  calls MG3's `mw_bake::anim::{HumanoidTarget, cook_library_clip}` on the dressed actor's skeleton (no bake round trip).
- Scene: the `character` viewer's actor path (mw_actors_bevy content: real body and graph, no AI, no physics), not the
  test-level machinery (that pulls streaming, player and physics); 1 m checker floor, fixed noon sun. Clip = `LateClip`
  in slot `full`, root motion applied, anim LOD forced A0, camera follows the actor.
- Capture: `TimeUpdateStrategy::ManualDuration(1/fps)`; frame 0 on a zero time scale (pose t=0), then one step per
  frame; `Screenshot` in `Last` -> `<out>/<id>/frames/%05d.png`; N = round(duration*fps)+1; run fails unless PNGs == N;
  ffmpeg libx264 yuv420p -> `<out>/<id>.mp4` (`<out>` is relative to `bevyrrowind/`, the exe's working folder).
- Measured (dev build, 960x540, 30 fps, 4060 laptop): NPC load ~1.1 s (250 frames) once per run; per clip cook + retarget
  75-90 ms, capture ~0.35 s per s of clip (89 frames in 1.0 s), encode 0.1-0.2 s; a 3 s clip, one-clip run: ~4 s wall.
- Tests: `cook.rs` unit tests (frame count, batch lines, bundled UAL1 resolve). MG6/MG7 hook into `stage` / the body entity.

## As built (MG6, 2026-10-28)

- Args: `clip-preview ... --furniture <record id> [--at x,y,z[,yaw]]` (meters, degrees about +Y, scene frame: NPC at the
  origin facing +Z; default `0,0,0,0`). Any kind `mw_world::resolve_base` places except actors (STAT/ACTI/CONT/DOOR/...);
  unknown id: error with the 8 closest ids (contains-first, then edit distance; `cook::near_matches`).
- Spawn path: the `view` viewer's uncooked-reference path (`WorldAssetRoot(assets.load(model))`, NIF via `MwAssetsPlugin`),
  spawned in `Startup` next to `stage`; `Loading` also waits for the model's recursive load. No collision draw, no auto-align.
- Verified: `--clip UAL1_RM.glb --name Sitting_Idle_Loop --furniture furn_de_p_chair_01 --at 0,0,-0.15,0 --camera side`
  -> `shots/clip-preview/UAL1_RM.Sitting_Idle_Loop.mp4` (mesh `f/furn_de_chair_01.nif`; seat under the pelvis, owner to judge).
- Tests: `cook.rs` `at_parses_meters_and_yaw`, `near_matches_rank_containing_ids_first`.

## As built (MG7a, 2026-10-28)

- Args: `clip-preview ... --prop <record id>[@<bone>] [--prop-offset x,y,z[,rx,ry,rz]]` (bone frame: meters, then degrees
  about X, Y, Z, `EulerRot::XYZ`; default `0,0,0`). Any record `mw_world::resolve_base` places (MISC, LIGH, ...); unknown id: closest ids.
- Attach: the game's held-weapon convention, `mw_actors::held::bone::WEAPON` (`Weapon Bone`), else `Bip01 R Hand` (as
  `mw_combat::WEAPON_BONES`), found via the actor's `SkeletonBinding`; `WorldAssetRoot` child of that joint, joints up to the body
  get `Visibility` (as `sf_character_bevy` held props). Attached during `Loading` once the rig is bound; `Loading` waits for it.
- Tankard: `--prop-offset 0,0,0,90,0,0` (Weapon Bone's +Z is the blade axis, the mug's up; +90 about X stands it in the fist;
  owner to judge). Verified: `--clip UAL1_RM.glb --name Drink --prop misc_com_tankard_01 --prop-offset 0,0,0,90,0,0 --camera side`
  -> `shots/clip-preview/UAL1_RM.Drink.mp4`. Tests: `cook.rs` `prop_parses_id_and_bone`, `prop_offset_parses_meters_and_degrees`.

## As built (MG10a, 2026-10-28)

- Command (dev builds): `bevyrrowind clip-preview --browse <dir> [--source generated] [--npc ...] [--size WxH]` (exclusive with
  `--clip`/`--batch`; `--furniture`/`--prop` stay global). Real time (no `TimeUpdateStrategy`), vsync, resizable window, no capture.
- Keys: Left/Right step clips within the activity, Up/Down step activities (first folder under `<dir>`; `_`/`.` folders skipped),
  both wrap; any mouse button drag orbits, the wheel zooms. Overlay + window title: activity, clip path, `.prov.json` band and prompt.
- Switch: the clip is cooked on demand by the capture's own path (`cook::resolve` + `cook::cook`, 15-20 ms per tier1 clip), then
  `play_full` (shared with capture) puts the actor back on the origin facing +Z, root motion on, and plays it in slot `full`; the
  loop replays it the same way at its end. A cook error shows in the overlay and stops the slot (graph idle).
- Code: Bevy-free list `clip_preview/browse.rs` (`BrowseList`, `Prov`; 5 tests), systems `clip_preview/live.rs`.
