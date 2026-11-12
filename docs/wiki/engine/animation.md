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
  Op stack order: `SpeedMatch` (0.5-2.2, `rate_share` 0.5) `:925`, `StrideWarp` (max 1.2) `:947`, `FootPlacement`
  (unplant 20 deg, `gate on_ground`) `:956`, `Lean` (chest .45, upperChest .4, neck .15) `:974`, `HandIk` x2 `:981`,
  then `HitReact` and `SpringBones` (not in first person).
- **Foot IK** (`FootPlacement`, `procedural.rs`): needs ground samples; without them its weight is 0, so no lock, no
  pelvis drop and no grade. NPCs: `sf_controller_bevy::FootProbes` via `feed_animators`; player:
  `bevyrrowind/crates/mw_player_bevy/src/avatar.rs:213-289` (drawn root frame, dropped under Warp/Clip moves and with
  `ControllerSettings::foot_probes` off, key K).
- **Clip sources and bake**: `bevyrrowind bake` (`bevyrrowind/crates/mw_bake/src/anim.rs:5,155,169`) runs
  `skeleton.cook`, `rig.profile`, `anim.cook` (Morrowind `.kf` groups, `Group@head/@tail` sections) and
  `anim.retarget` (library clips onto the humanoid rig) and `animgraph.compile`. The manifest
  `bevyrrowind/crates/mw_graphs/data/animation/sources.ron` maps slots to `source:clip` chains; only the CC0
  Quaternius UAL1/UAL2 packs ship (LFS). The game reads the last bake: re-run it after changing slots.
- **Generated clips** (Kimodo, MotionBricks) come in through the clip factory (`specs/motion-generation.md`).

## Bevy features used
Skinning and transform propagation only; graph, IK and LOD are ours (Bevy's `AnimationGraph` has no
inertialization, ops or LOD budget). See [characters](characters.md).

## Switches and counters
`anim/evaluate/ms`, `anim/evaluate_cpu/ms`, `anim/characters|evaluated|a0|a1|a2|a3/count` (`lib.rs:628-649`);
`AnimStats.feet` guard counts. Key K: foot probes off. Perf `--off walkers` / `actors`.

## Known limits
- Free-movement clips are Morrowind's 2002 cycles at the wrong speeds, with no start/stop/pivot clips; the
  modern preset is moving to generated cycles ([animation ledger](../../ledger/animation.md)).
- Slope ops over-correct on steep ramps and at speed (head pitch, pelvis drop); P-IK3/P-IK4 not built.
- Nothing is signed off on the player's walk: AL-2, AL-3 awaiting owner, PL-7 partial, PL-8 awaiting.

## Specs
[animation.md](../../../superfargoth/docs/specs/animation.md), [animation-layers.md](../../../superfargoth/docs/specs/animation-layers.md),
[animation-plan.md](../../../superfargoth/docs/specs/animation-plan.md),
[animation-sources.md](../../../bevyrrowind/docs/specs/animation-sources.md)
