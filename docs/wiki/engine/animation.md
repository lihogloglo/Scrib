# Animation

## What it does
A data-driven pose graph (state machines, blends, slots, procedural ops) is evaluated per character in
`PostUpdate`, written to bone `Transform`s, then skinned on the GPU as for glTF. The Morrowind humanoid, first-person
and creature graphs are built in code, shipped as RON, and compiled at bake time.

## How it works
- **Schedule** (`superfargoth/crates/sf_anim_bevy/src/lib.rs:8-30,61-104`): `PostUpdate`, before transform
  propagation: `Lod` (tier per character) then `Probe` (host feeds: ground samples, IK and grip targets, velocity)
  then `Evaluate` (`par_iter`, `:565`, deterministic) then `Apply` (`write_bones` `:671`, root motion, events).
  `Probe` is gated off while `Time<Virtual>` is paused (`world_paused` `:312`); Evaluate/Apply skip unless an
  `AnimateWhilePaused` character exists (`:298`).
- **write_bones** loops characters (posed, or with a `PhysicsPose`) through their `SkeletonBinding` and writes a
  `Transform` only when it changed (`:671-690`); frozen characters' bones are never visited.
- **LOD tiers** (`sf_anim_bevy/src/lod.rs:12-56`): A0 every frame, full graph and ops; A1 every frame, LOD1 joint
  mask; A2 every `A2_INTERVAL` (2) frames, LOD2 mask; A3 clocks, events and root motion only (pose frozen,
  `posed = lod < 3`, `sf_anim_graph/src/instance.rs:929`). Thresholds in reference pixels at 1440 px:
  120 / 50 / 20, hysteresis 0.15, off-screen to A3 (`AnimLodSettings` `lod.rs:60-85`); an optional per-frame
  `AnimBudget` demotes the least significant first (default `None`: distance only).
- **Graph runtime** (Bevy-free): `sf_anim_graph/src/instance.rs:872` (`GraphInstance::update`), nodes and ops in
  `asset.rs` (nodes `:284-390`, transitions `Inertialize`/`Crossfade` `:407-409`, ops `:624-837`), compile in
  `compile.rs`, inertialization `inertial.rs`. Pose maths, IK, warp, retarget, cook: `superfargoth/crates/sf_anim/src`.
- **Ops** (`procedural.rs`, listed in `OpSpec`): `SpeedMatch` (play rate from measured speed), `StrideWarp`,
  `FootPlacement` (probes, plant/lock, pelvis, turn unplant), `Lean`, `LimbIk`, `ChainIk`, `LookAt`, `FullBodyIk`,
  `HandIk`, `HitReact`, `SpringBones`, `OrientationWarp`, `SpineBend`. `max_lod` per op skips them at lower tiers.
- **Humanoid graph** (`bevyrrowind/crates/mw_graphs/src/build.rs`, shipped as
  `bevyrrowind/crates/mw_graphs/data/animation/graphs/mw_humanoid.animgraph.ron`, ~209 KB): base `locomotion` state
  machine (idle, walk, run, turn left/right, sneak, jump, swim, reactions) picked by parameter conditions
  (`:386-391`); nested stance machines (`hh`, `1h`, `2c`, `2w`); lock-on `strafe_walk`/`strafe_run` 8-way blends only
  if the manifest has `loco.*` slots (it has none, `:14-24`); `upper` and `full` slots; torch arm layer.
  Op stack order: `SpeedMatch` (0.5-2.2, `rate_share` 0.5, `gate speed_match`) `:941`, `StrideWarp` (max 1.2, `gate
  stride_warp`) `:968`, `FootPlacement` (unplant 20 deg, `gate on_ground`) `:977`, `Lean` (chest .45, upperChest .4, neck
  .15, `gate lean`) `:995`, `HandIk` x2 `:1001`,
  then `HitReact` and `SpringBones` (not in first person).
- **Gait set** (A3, 2026-11-12; `mw_graphs/src/build.rs:156-190` `Preset`, `gaits`; `:703-725`): humanoid free
  movement in `walk` and `run` (sheathed and spell stances) is one `BlendSpace1D` over `speed` (`params::SPEED`, fed by
  `mw_player_bevy/src/avatar.rs:281` and walkers' `AnimParamNames.speed`, `mw_walkers_bevy/src/walkers.rs:738`) of the
  slots `loco.walk|jog|sprint.cycle` at 1.6 / 4.5 / 7.0 m/s (`sources.ron:120`, Kimodo loco1 takes, `loop_cut` at defaults,
  `root: Match`: the take's heading drift is never applied). Both states' sets are in sync group `gait`;
  `rate_clamp` 0.8-1.25 (`sf_anim_graph/src/asset.rs:320`, `instance.rs:1560`) replaces `SpeedMatch`'s 0.5-2.2 on its
  lanes. A `Blend` on `strafing` fades to the Morrowind directional set while locked on (`walk.none.directional`).
  Without the library the slots drop and the Morrowind sets play; weapon stances, sneak and swim keep them.
  `Preset::Classic` (`asset_for`, `:1133`) builds the pre-A3 graph (digest pinned in `mw_graphs/src/tests.rs`); nothing
  selects it yet. Style seam: `SourcesManifest::gait_slot` (`sources.rs:294`) looks up `loco.<style>.<gait>.<part>`,
  then the base set. Tests: `mw_graphs/tests/gait_set.rs`; A1 tables `slope_walk player_gaits` (`A1_CLASSIC=1`: Morrowind).
- **Sync groups across states** (`instance.rs:1489-1503`, `join` `:1611`): a member entering alone (an inertialized
  switch stops the old state at once) joins the last update's leader by markers or phase (UE sync groups keep their
  position across updates); with no member for an update the group forgets.
- **Foot IK** (`FootPlacement`, `procedural.rs`): needs ground samples; without them its weight is 0, so no lock, no
  pelvis drop and no grade. NPCs: `sf_controller_bevy::FootProbes` via `feed_animators`; player:
  `bevyrrowind/crates/mw_player_bevy/src/avatar.rs:213-289` (drawn root frame, dropped under Warp/Clip moves and with
  `ControllerSettings::foot_probes` off, key K).
- **Clip sources and bake**: `bevyrrowind bake` (`bevyrrowind/crates/mw_bake/src/anim.rs:5,155,169`) runs
  `skeleton.cook`, `rig.profile`, `anim.cook` (Morrowind `.kf` groups, `Group@head/@tail` sections) and
  `anim.retarget` (library clips onto the humanoid rig) and `animgraph.compile`. The manifest
  `bevyrrowind/crates/mw_graphs/data/animation/sources.ron` maps slots to `source:clip` chains; only the CC0
  Quaternius UAL1/UAL2 packs ship (LFS). The game reads the last bake: re-run it after changing slots.
- **Loop cutter** (`superfargoth/crates/sf_anim/src/loop_cut.rs:1-39`, rules `:53` `LoopCutRules`, cut `:199`): opt-in
  `ClipRules::loop_cut` (`cook.rs:242`; a slot's `loop_cut: Some((...))` in `sources.ron`, `mw_graphs/src/sources.rs:187`)
  turns a non-periodic take (Kimodo ramps up, holds, slows) into a seamless loop: Kovar 2002 transition-point search
  between same-foot touchdowns (the unlooped cook's `down.<foot>` markers) inside the steady part (within 10% of the
  median stride-averaged speed), joint position + velocity distance in a stride-smoothed character frame, fewest strides (1-2) on
  ties; trims to `[a, b]` and fades the last 0.12 s (4 frames at 30 Hz) into the frames before `a` moved by one cycle,
  so the last frame is exactly the first moved by the loop's root delta. `cook_clip_report` (`cook.rs:341`) returns
  `LoopCut` (window, strides, seam errors before/after the fade, speed, turn) or a `LoopCutError` (clip left
  unlooped); `mw_bake::anim::cook_library_clip_report` (`anim.rs:157`). Rule absent: byte-identical cook. Used by
  the gait slots (below). `anim.cook` step version 12. Measure: `mw_bake/tests/loop_cut.rs` (loco1 keep/maybe cycles). Preview:
  `clip-preview --browse` cooks `*.cycle.*` takes with the cut (`clip_preview/cook.rs` `cook_report`), loops them in the slot
  (`LOOP_FOREVER`, no re-blend), overlay = `cut_text`, L = raw take A/B (`clip_preview/live.rs`).
- **Retarget pose** (`superfargoth/crates/sf_character/src/align.rs:1-35`, rule at `:213`): auto-align turns limbs to the
  canonical T-pose (arms ±X, legs −Y) but the trunk (hips, spine, neck, head) keeps each rig's own rest curvature
  (UE5's matched retarget pose, 2026-11-12), so a clip carries the source's change from rest onto Morrowind's bind;
  a trunk bone > 45° from up is straightened. The retargeter (`sf_anim/src/retarget.rs:1-40`) copies model-space
  deltas from that pose (`D = qS · Rs⁻¹`, `qT = D · Rt`). `anim.retarget` step version 6.
- **Retarget-pose offsets** (`sf_character/src/align.rs:268` `PoseOffset`, `:293` `offset_retarget_pose`; UE5 "Edit
  Retarget Pose", 2026-11-12): after auto-align a rig may rotate a limb bone and its subtree (canonical frame, left
  side given, right mirrored: `raise_deg` about forward, `forward_deg` about up). Data in `mw_graphs/data/animation/sources.ron`:
  a source's `pose_offsets` (Kimodo `generated`: clavicle −10.5°, upper arm +10.5° to keep the arm at T: its walks hold the
  clavicles that low) and `target_pose_offsets` for the Morrowind humanoid (upper arm +4° abduction, every source:
  bulky arm and chest meshes). Applied in `mw_bake/src/anim.rs:96` `library_rig` and `:109` `humanoid_target_map`.
  Measure: `mw_bake/tests/generated_arms.rs` (`measure_generated_clavicle_offset` sets the source value).
- **Generated clips** (Kimodo, MotionBricks) come in through the clip factory (`specs/motion-generation.md`).

## Bevy features used
Skinning and transform propagation only; graph, IK and LOD are ours (Bevy's `AnimationGraph` has no
inertialization, ops or LOD budget). See [characters](characters.md).

## Switches and counters
`anim/evaluate/ms`, `anim/evaluate_cpu/ms`, `anim/characters|evaluated|a0|a1|a2|a3/count` (`lib.rs:628-649`);
`AnimStats.feet` guard counts. Key K: foot probes off. Perf `--off walkers` / `actors`.
Walk A/B ladder (WALK-0, player only, live): console `off speed_match` (clips at rate 1), `off stride_warp`,
`off foot_placement` (= K, walkers too), `off lean`, `off side_lanes` (free movement feeds `move` = forward,
`mw_graphs::build::lane_direction`); `on <name>` restores. `mw_game_core::player::LocoAb` -> avatar feed -> the humanoid
graph's gate params `speed_match`/`stride_warp`/`lean` (default 1; `SpeedMatch` raises its rate factor to the gate, `Lean`
scales its goals, `StrideWarp` its weight).

## Known limits
- Free movement plays generated walk/jog/sprint loops (A3, WALK-1 awaiting owner); still no start/stop/pivot clips,
  and the sprint take is a "maybe" (retimed free sprint next) ([animation ledger](../../ledger/animation.md)).
- Slope ops over-correct on steep ramps and at speed (head pitch, pelvis drop); P-IK3/P-IK4 not built.
- Nothing is signed off on the player's walk: AL-2, AL-3 awaiting owner, PL-7 partial, PL-8 awaiting.

## Specs
[animation.md](../../../superfargoth/docs/specs/animation.md), [animation-layers.md](../../../superfargoth/docs/specs/animation-layers.md),
[animation-plan.md](../../../superfargoth/docs/specs/animation-plan.md),
[animation-sources.md](../../../bevyrrowind/docs/specs/animation-sources.md)
