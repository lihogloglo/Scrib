# Locomotion, action, camera

## What it does
Every character (player, NPC) moves by a kinematic capsule mover on a 60 Hz fixed tick. One chooser picks what a
character does next from one `Command` per tick and data move rules. The drawn body is the interpolation of the last
two ticks (`ControllerSystems::Interpolate`, the same `TickPose` mechanism for player and NPCs, `sf_controller::frame`);
the camera follows the drawn body at frame rate and never feeds gameplay.

## How it works
- **Tick**: `TICK_HZ = 60` (`superfargoth/crates/sf_action/src/ids.rs:6`); Bevy `Time<Fixed>`, `physics_in_fixed_timestep`
  pattern (`sf_action_bevy/src/lib.rs:1-30`). Order per frame: `PreUpdate` input (`bevy_enhanced_input`), `Gather`
  (device edges into `InputSampler`), `FixedUpdate` `Sample` then `Command` (AI, scripts, net write theirs) then
  `Step` then `Move`, then `Interpolate` (blend by `overstep_fraction`, plus the render-correction spring), then cameras.
- **Chooser**: `sf_action/src/step.rs` (`step`: pure, one per character per tick; Sekiro-style two-stage buffer,
  jump buffer and coyote time in ticks, forced landings by impact band; a move whose open `Accept` latches a press it also `Cancel`s into restarts itself, so mashed rolls chain; `ActionOutput::buffering` says what the move latches). Tap/hold buttons (`command.rs` `TapHold`, threshold tunable `move.input.tap_hold_s` 0.30 s) fire the tap on release from neutral but on press while the current move buffers it (dodge during a roll or an attack recovery). Rules: `rule.rs` (`MoveRule` RON compiled
  to a priority-ordered `RuleSet`; `Owner` at `:64`: `Sim`, `Warp`, `Clip`), `BASE_RULES`, `TRAVERSAL_RULES`
  (mantle waist/high, vault), holds (guard), look limits, grip (`grip.rs`) and brace (`brace.rs`). State is plain
  serde with integer clocks and no singleton player (`state.rs`), so it is net-ready.
  The resting intent (stick at rest, or a move that does not steer) has zero velocity and keeps the stance's gait:
  `Gait::Sneak` in `Mode::Sneak` (crouch is held by its button, not by speed: the graph's `sneaking` and `sneak_idle`
  follow it), `Walk` in every other mode (`step.rs` `rest`, ROADMAP 2.1f).
- **Moving bases** (IN11a): on ground with a body, `Locomotion::ride_base` (`mover.rs:406`, first in `begin`) moves
  the feet with the base (rotation about its instantaneous centre from `GroundHit` point velocity and yaw rate) and
  turns yaw with it; `moved` excludes it, so an idle rider plays standing. Leaving keeps the base's horizontal
  velocity in `inherited` (`:322`, runtime only), cleared on landing; a jump adds the base's vertical speed (`launch`). A medium's
  drift is `Locomotion::current` (`:336`, m/s world, runtime only, written by the host every tick; FL-5b): `begin` adds its
  horizontal part x dt to the displacement in every mode before the wall sweep, so banks stop it and the intent stays
  relative to the water (nothing writes it until FL-5c's swim system).
  Stage 0 `KinematicMover::carry` (IN11c1, `mover.rs:507`) sweeps the ride (`carry_slide` :717: blocks only on
  closing relative velocity, ignores the base) and applies pushes from `GroundProbe::contacts` (`probe.rs:137`, Jolt
  `CharacterVirtual` style; default none: the physics probe implements it in IN11c2a).
- **Mover**: `sf_controller/src/mover.rs:48` (`MoverParams`), `:214` (`MoverMode`), `:281` (`Locomotion`);
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

- **The approach before a sit** (IN2c4): an offered seat whose enter warp would refuse (correction over half the
  clip's travel) starts `interact.sit.approach` (`sf_action/src/approach.rs`, `Owner::Approach`): the action layer
  walks the body to the enter clip's start pose (slot frame minus the clip's whole root travel), turns it in place,
  then `ExitOn::Arrived` chains into `interact.sit.enter`. Stick, Jump, 3 s or a target past 2.5 m end it with one
  `SeatRefused` (`ApproachCancelled | ApproachTimeout | ApproachTooFar`); numbers are rule data in `seat_rules.ron`.
- **Slot aliases and seats** (AN26c): a rule names fixed slots (`interact.sit.enter|loop|exit`, `SEAT_RULES`); a character's
  `ActionAliases` (`sf_action_bevy/src/lib.rs:136`, `sf_action::SlotAliases` in `sf_action/src/alias.rs`) maps a rule slot to the clip
  slot to time and warp with (`Aliased` source in `step_actions`); `ActionOutput::slot` stays the rule's. The seat binder
  (`sf_interaction_bevy/src/seat_clips.rs`: `SeatClipSets::from_names` indexes `<family>.b<cm>.{enter.<side>, loop, exit.<side>}`,
  nearest band, fallback family, side else front; `bind_seat_clips` after the seat offer, before the step, latched while the slot
  plays) writes the three aliases from the seat's `SeatClips`. Game: `mw_exterior_bevy` `player_clips` loads the player's own slots
  plus every graph slot clip named `seat.*` (`mw_player_bevy::is_seat_clip`; log `player clips: N of M slot clips baked`);
  `mw_player_bevy` `rules()` gives each seat clip a `SlotInfo` and root path under its own name, `add()` inserts `SeatClipSets`
  (fallback `seat.chair`), and the avatar (`avatar.rs` `traversal`) plays the clip the rule slot resolves to, restarting when the
  alias changes. NPCs: walkers are not action characters yet, so only the player sits.

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
- Moving bases: yaw only (no pitch/roll), vertical ride unswept, no crush handling, kinematic bodies push (IN11c2a), dynamic
  ones don't; a mid-air save drops inherited momentum.
- NPC walkers (ORCA) do not block the player; separation has no wall sweep ([combat ledger](../../ledger/combat-feel.md)).

## Specs
[locomotion.md](../../../superfargoth/docs/specs/locomotion.md), [action.md](../../../superfargoth/docs/specs/action.md),
[camera.md](../../../superfargoth/docs/specs/camera.md), [combat.md](../../../superfargoth/docs/specs/combat.md)
