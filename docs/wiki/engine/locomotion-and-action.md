# Locomotion, action, camera

## What it does
Every character (player, NPC) moves by a kinematic capsule mover on a 60 Hz fixed tick. One chooser picks what a
character does next from one `Command` per tick and data move rules. The drawn body is the interpolation of the last
two ticks; the camera follows the drawn body at frame rate and never feeds gameplay.

## How it works
- **Tick**: `TICK_HZ = 60` (`superfargoth/crates/sf_action/src/ids.rs:6`); Bevy `Time<Fixed>`, `physics_in_fixed_timestep`
  pattern (`sf_action_bevy/src/lib.rs:1-30`). Order per frame: `PreUpdate` input (`bevy_enhanced_input`), `Gather`
  (device edges into `InputSampler`), `FixedUpdate` `Sample` then `Command` (AI, scripts, net write theirs) then
  `Step` then `Move`, then `Interpolate` (blend by `overstep_fraction`, plus the render-correction spring), then cameras.
- **Chooser**: `sf_action/src/step.rs` (`step`: pure, one per character per tick; Sekiro-style two-stage buffer,
  jump buffer and coyote time in ticks, forced landings by impact band). Rules: `rule.rs` (`MoveRule` RON compiled
  to a priority-ordered `RuleSet`; `Owner` at `:64`: `Sim`, `Warp`, `Clip`), `BASE_RULES`, `TRAVERSAL_RULES`
  (mantle waist/high, vault), holds (guard), look limits, grip (`grip.rs`) and brace (`brace.rs`). State is plain
  serde with integer clocks and no singleton player (`state.rs`), so it is net-ready.
- **Mover**: `sf_controller/src/mover.rs:35` (`MoverParams`), `:185` (`MoverMode`), `:214` (`Locomotion`);
  `KinematicMover::step_batch` springs velocity and facing toward the intent (Holden spring, closed form), blends
  last update's root motion, collide-and-slides statics, steps up/down, predicts the `Trajectory`. All probes go
  through one batched `GroundProbe` call per stage (`probe.rs`). ORCA avoidance runs first (`avoid.rs`).
- **Bevy glue**: `sf_controller_bevy/src/lib.rs:1-40` (`Drive`, `Step`, then `Feed` in `AnimSystems::Probe`: velocity,
  moved delta, turn rate into each `Animator`). `ControllerSettings` `:259` (`moving_speed`, `run_speed`,
  `foot_probes`).
- **Player feel**: `bevyrrowind/crates/mw_player_bevy/src/feel.rs:15-33` (accel half-life 0.07 s, brake 0.05 s,
  turn 720/1080/480/540 deg/s walk/run/sprint/sneak, gravity 18). `avatar.rs:213-289` feeds the graph (parameters,
  drive, foot probes) from the **drawn** root; velocity is a finite difference of the drawn root (`:230-252`,
  half-life 0.025 s).
- **Camera**: `sf_camera` is Bevy-free math (orbit rig, sphere-cast arm collision, lock-on scoring and soft aim,
  first-person look limits, comfort shake, view-model projection; `lib.rs:1-30`). `sf_camera_bevy/src/lib.rs:1-30`:
  `Look` before the fixed loop (look goes into the sampler as `Command.view`, the move stick resolves against its
  yaw), `Rig` in `PostUpdate` before transform propagation. Move look limits: `follow_move_look` `:361`,
  `update_look` `:378`.
- **First-person eye**: a fixed eye (`eye_height` 1.62 m, `eye_forward` 0.12 m, pitch +-85 deg;
  `sf_camera_bevy/src/lib.rs:118-131`, used at `:502-518`). It is not yet the head joint and ignores actor height.
- **Combat sits on top**: `sf_combat(_bevy)` runs `Gate`, then action `Step` and mover, then combat `Step`
  (`sf_combat_bevy/src/lib.rs:1-25`); lock-on in `lock.rs`, soft aim `sf_combat/src/aim.rs:61`; Morrowind sheets
  `bevyrrowind/crates/mw_combat/data/movesets/sword_1h.ron`, player glue `mw_player_bevy/src/combat.rs`.

## Bevy features used
`FixedUpdate` and `Time<Fixed>`, `RunFixedMainLoop` sets, `bevy_enhanced_input`, `big_space` grids for `WorldPos`.
Own: mover, chooser, camera rig (Bevy has no character controller).

## Switches and counters
`ControllerSettings::foot_probes` (K), `--off walkers`/`ai`, `physics` (`mw_game_core/src/perf.rs:154-158`).
Headless tests: ticks identical at 30/60/144 fps (`sf_action_bevy/src/tests.rs`).

## Known limits
- Walk and run are separate states switched by a bool; braking is near-instant, there are no start/stop/pivot clips
  ([animation ledger](../../ledger/animation.md) 2026-11-12 audit).
- The capsule turns at 720-1080 deg/s and the free-movement blend follows the local move direction, so sharp turns
  swing the lanes (audit cause 4, unmeasured).
- No look-at, no offset root, no aim pitch layer; first-person eye is fixed.
- NPC walkers (ORCA) do not block the player; separation has no wall sweep ([combat ledger](../../ledger/combat-feel.md)).

## Specs
[locomotion.md](../../../superfargoth/docs/specs/locomotion.md), [action.md](../../../superfargoth/docs/specs/action.md),
[camera.md](../../../superfargoth/docs/specs/camera.md), [combat.md](../../../superfargoth/docs/specs/combat.md)
