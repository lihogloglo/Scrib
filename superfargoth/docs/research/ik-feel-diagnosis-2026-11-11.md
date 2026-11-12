# IK and movement feel: diagnosis (2026-11-11)

Owner 2026-11-11: "The animation of the character and the feel of moving around is still not great. I feel that the IK
doesn't work as it should (maybe that's because we have issues with Delirium? or unrelated?)". Related: ROADMAP 1.18
(IK runs while paused), 3.4 (faint jolty walk on slopes), 2.1 (inertia), 3.1 (hand IK tuning), 3.2/3.2b (Delirium).
Diagnosis only: no fix was made. Measurements: two `#[ignore = "measurement: ..."]` tests added at the end of
`bevyrrowind/crates/mw_graphs/tests/slope_walk.rs` (real install, shipped humanoid graph, NPC mover):

```sh
bash bevyrrowind/tools/cargo_turn.sh test --release -p mw_graphs --test slope_walk -- --ignored --nocapture player_without_foot_probes paused_updates_move_the_pose
```

## Verdict in one paragraph

The IK is not broken; **the player never gets it**. `mw_player_bevy::avatar::feed` clears the graph's ground samples
every frame (`avatar.rs:271`, `a.drive.ground.fill(None)`; the module doc says "Foot probes are not fed yet: foot
placement holds the animated pose"), and nothing else writes them for the player. With no samples the humanoid
graph's `FootPlacement` weight goes to 0 (`sf_anim_graph/src/procedural.rs:626-635`: `goal = 0` unless `sampled`), so
the player has **no foot placement, no foot lock (anti-skate), no pelvis drop**, and the filtered ground grade stays 0
(`instance.rs:879-883`), so **no slope trunk lean (`Lean`) and no uphill/downhill stride change (`StrideWarp` grade)**
either. This has been so since the player body was added (commit 2e73695). Every foot/slope check the owner confirmed
(AL-3, AL-9, AN-1, AN-9, AN-21) was on NPC walkers in `anim-lab` scenes, which go through `sf_controller_bevy`'s feed
and do get probes. Delirium is unrelated (section 3).

## 1. IK features, per character

| Feature (op in `mw_humanoid.animgraph.ron`) | NPC walkers | Player | Schedule |
|---|---|---|---|
| `SpeedMatch` (playback rate from measured speed) | on | on | `PostUpdate`, `AnimSystems::Evaluate` |
| `StrideWarp` (stride length; grade term) | on, grade from probes | on, **grade always 0** | same |
| `FootPlacement` (foot height/tilt, foot lock, pelvis drop) | on (K toggles) | **off: no samples** | same |
| `Lean` (slope trunk lean, AN-21) | on | **never leans: grade 0** | same |
| `HandIk` x2 (lip grip, wall brace) | weight 0 | on during mantles / brace (`grip.rs`) | same |
| `HitReact`, `SpringBones`, cloth | on | on | same |
| Look-at, orientation warp, full-body IK | not in the graph (ops exist in `sf_anim_graph`) | not in the graph | - |

NPC ground probes: `sf_controller_bevy/src/lib.rs:678-717` (batched, at the last output's `probe_origins`, in the
mover's render frame, scaled by race). Player: none (`avatar.rs:271`). The graph builder even expects the player's
feed (`mw_graphs/src/build.rs:945`: "Foot IK fades out in the air: the player feeds a debounced `on_ground`"): the
gate was wired, the samples never were.

Measured (`player_without_foot_probes`, "npc" = walkers' probes, "player" = none):

Walk 1.6 m/s (the player's walk), planted feet only (contact > 0.5), on the ramp part of each walk:

| Walk | sole sink / float cm, npc | player | planted slide cm/s, npc | player | mean grade, npc | player |
|---|---|---|---|---|---|---|
| flat | 1.4 / 0.4 | 1.4 / 0.4 | 1.5 | 7.0 | 0 | 0 |
| up 10° | 1.3 / 0.5 | 7.9 / 4.8 | 1.8 | 7.3 | 0.17 | 0 |
| up 20° | 0.9 / 0.9 | 15.2 / 10.6 | 2.6 | 6.8 | 0.36 | 0 |
| down 20° | 0.9 / 0.9 | 12.7 / 12.6 | 0.9 | 6.8 | -0.36 | 0 |
| up 30° | 0.2 / 1.6 | 23.5 / 17.2 | 1.8 | 7.2 | 0.56 | 0 |
| down 30° | 0.2 / 1.6 | 19.6 / 20.6 | 1.5 | 7.2 | -0.56 | 0 |
| stairs 0.15/0.30 up | 1.4 / 0.4 | 34.0 / 6.1 | 2.4 | 7.6 | 0.34 | 0 |

Run 4.5 m/s: planted slide 19-26 cm/s (npc) vs 54-61 cm/s (player) on every surface; for the player sole sink up to
18 cm and float up to 36 cm at 30°. (The run's contact curves are loose, so both read ~10 cm of "float" on the flat:
compare the rows, not the absolute value.) The foot lock cuts planted slide about 4x at walk and 2.5x at run; the
player gets none of it, on flat ground too.

What the owner sees on the player because of this: feet floating or sinking on every slope and stair (the capsule
root sits on the ground under its centre, the animated feet assume flat ground at root height), planted feet
skating on starts, stops and the fast turns (720-1080 deg/s, `feel.rs`) with nothing to pin them, the trunk upright
on ramps, the same step length up and down. That matches "the IK doesn't work as it should".

Hand IK (3.1) works as built but is a tuning job already logged (hands inside the mesh, waist mirrored right hand
~0.2 m short, needs shoulder/arm length baked into contacts: `.sessions/hands-on-lip.md` follow-ups). Not a cause of
the general feel.

## 2. Ordering, tick, inertia

Checked, no defect found:
- Player: fixed-tick mover (`sf_action_bevy`, `FixedUpdate` at 60 Hz) with render interpolation
  (`interpolate`, `RunFixedMainLoop` after the fixed loop, `lib.rs:547`) plus the correction spring; the step-up
  height is smoothed in the mover (`height_offset`). The avatar feed measures motion from the **drawn** root
  (`avatar.rs:225-242`), so the graph sees smooth motion between ticks. Camera rig in `PostUpdate` before transform
  propagation, after the interpolated `Transform` is written: same frame as the pose. This is Gaffer's "fix your
  timestep" + UE's mesh smoothing; correct.
- NPCs: variable-step mover in `Update` (`ControllerSystems::Step`), feed and graph in `PostUpdate` of the same frame;
  probes in the render frame (`Locomotion::frame` = `render_position`), consistent with the drawn root.
- Root motion vs capsule: locomotion is capsule-driven with speed matching and stride warping (UE Lyra/GASP model);
  root motion only for traversal/combat clips (`RootMotionMode::Progress` on the montage slot). No conflict.
- Both `sf_action_bevy` and `sf_physics_bevy` insert `Time<Fixed>`, both at 60 Hz: harmless today, but the last
  plugin wins; worth one owner of the tick rate (note, not a slice).
- Inertia (ROADMAP 2.1 note "too much inertia"): stale. Fixed by the PL-4 feel pass (velocity half-lives 0.07 s up /
  0.05 s braking, `mw_player_bevy/src/feel.rs`; PL-4 confirmed 2026-10-27 "about right"). Remaining "weight" in the
  feel is animation-side: idle/walk/run are cycles only (no start/stop/pivot clips, distance matching is gap G7 in
  `industry-audit.md`), the gait inertialization is 0.15 s, and without the foot lock the starts and stops slide.
- Sprint: the 7 m/s sprint plays the run cycle at the 2.2 rate cap (4.75 steps/s, `build.rs:926-930`): a frantic
  cadence, already logged (3.4 "sprint cycle").

Faint jolty walk on slopes (3.4, walkers): **lead found on the walkers.** The pelvis' vertical jerk
(RMS second difference per frame, printed by `player_without_foot_probes`) grows with the grade when foot placement
is on and stays flat without it: walk 2.1 mm flat, 2.6/2.7/3.2 mm up 10/20/30°, **2.7/4.5/6.1 mm down 10/20/30°**
(no IK: 2.1 at every grade); run 5.6 flat, 13.1 up 30°, **18.3 down 30°** (no IK: 5.2). On a ramp the mover's own
height is linear (no jerk), so the extra is the `FootPlacement` pelvis offset following whichever foot is lower,
step by step (the need re-taken each step through `held.max(goal)`, `procedural.rs:438`, then a 0.08 s spring,
`asset.rs:909`): a per-step bob, worse downhill, which reads as "jolty, syncopated". Pin the term before retuning
(slice P-IK3).

## 3. Delirium: unrelated

- The player is never taken over: `mw_delirium_bevy::start` refuses a root without a `Mover`
  (`lib.rs:463`, "an action character"; L4 in the feel-coord handoff, needs ROADMAP 3.2b hold/hand-back).
- A character not taken over has no `PhysicsPose`; `sf_anim_bevy::evaluate` (`lib.rs:497-504`) then clears or skips
  the physics stage, so no physics blend at weight > 0 outside a takeover. The balancer steps only for `Takeovers::live`
  (`step`, `FixedUpdate`). At hand-back `PhysicsPose` is removed by commands in `Update`, applied before `PostUpdate`.
- Under a takeover the foot ops scale their goal by `1 - legs' physics share` (`procedural.rs:192-203, 627`): IK
  yields to physics, no fight. Minor: `evaluate` allocates a `Vec<f32>` of joint weights per physics character per
  frame (`lib.rs:499`); trivial.

## 4. ROADMAP 1.18: IK runs while paused

Cause: nothing gates the animation stage on pause. The pause menu pauses `Time<Virtual>` (`mw_exterior_bevy/src/
pause_menu.rs:244-255`), which stops the fixed loop (mover, Delirium, physics), but `sf_anim_bevy`'s `PostUpdate`
chain (`select_tiers`, `evaluate`, `write_bones`, `lib.rs:70-87`) still runs every frame with `dt = 0`, and the hosts'
`Probe` feeds keep running (NPC ground probes `sf_controller_bevy::feed_animators`, the player's `avatar::feed` and
`grip::feed`, whose wall brace re-probes the physics world). Every spring in the ops is dt-scaled, so most of the pose
freezes, but the graph is re-evaluated in full each paused frame (CPU wasted) and parts that aren't spring-filtered
react at once: the probe lead drops to 0 at `dt = 0` (`instance.rs:1068-1074`), so the next samples come from other
points under the feet (on stairs, another tread). Measured
(`paused_updates_move_the_pose`, walker, probes refreshed each paused update):

| Walker stopped by the pause | first paused update | later paused updates |
|---|---|---|
| 20° ramp, mid-stride | 0 mm | 0 mm |
| flat | 0 mm | 0 mm |
| 0.15 m stairs, mid-stride | **one foot drops 136 mm** (y 0.536 -> 0.400) | back to 0.536 on the next, then still |

So the pose doesn't drift while paused, but the paused updates are not a no-op: on stairs the first one re-probes
at lead-less origins and a foot pops 13.6 cm for a frame (any hitch frame with a near-zero dt can do the same). The
player has no foot IK today (section 1), so visible paused motion on the player can only come from `grip::feed` (the
wall brace probes the physics world every frame; weights and smoothing are dt-scaled, its "never behind the face"
push is not, `sf_action/src/brace.rs:212-216`) or from the camera. Ask the owner what moved (NPC feet, the player's
hands, the camera?). Either way the standard fix is the same: don't tick animation while the game is paused.

## 5. Ranked causes and fix slices

1. **Player gets no foot probes** (impact: high; every slope, stair, start, stop and turn).
   Slice P-IK1: feed the player's foot probes in `avatar::feed` (or a `Probe`-set system after it) through the same
   `Physics` ground resource and the same batched query as `sf_controller_bevy::feed_animators` (factor that code
   into a public helper taking a `CharacterFrame`, the animator and the body scale), in the **drawn** root frame
   (`drawn` at `avatar.rs:225`), skipped while a Warp/Clip move owns the body (the `on_ground` gate already fades the
   op in the air). Verify: a headless `mw_player_bevy` test on `level-flat-slopes` (the player walking up the ramp:
   ground samples `Some`, `output.feet.weight_max > 0`, `output.grade > 0` on the ramp; sanity only) and the
   `player_without_foot_probes` table re-run. Owner: `scene level-anim-stairs`/`level-flat-slopes` as the player
   (F11 off, Tab third person), K to A/B: feet on ramps and treads, planted feet on quick turns and stops, the trunk
   leaning uphill. Re-run AL-3/AN-21 looks for the player.
2. **No pause gate on the animation stage** (1.18; impact: low on look, a CPU cost in menus). Slice P-IK2: one
   run condition on the `AnimSystems` chain (and the host `Probe` feeds) "virtual time not paused", framework-side
   (`sf_anim_bevy`: skip `Probe`/`Evaluate`/`Apply` when `Time<Virtual>::is_paused()`, i.e. UE's
   `bTickEvenWhenPaused = false` default; an opt-in component for things that must animate in menus, e.g. a character
   preview). Verify: headless `sf_anim_bevy` test: pause `Time<Virtual>`, assert the graph instance is not updated
   (`Animator::updated == false`) and bone transforms unchanged; un-pause resumes with no pop. Owner: open the pause
   menu while walking on a slope and next to a wall: nothing moves.
3. **Pelvis bob on slopes with foot placement on** (3.4 "jolty, syncopated"; impact: medium, NPCs now, the player
   too once P-IK1 lands). Slice P-IK3 (diagnosis): print per frame the pelvis need, held value and spring output on
   the 20/30° ramps (extend `drive` in `slope_walk.rs`) to pin the term; then a build slice to smooth the pelvis
   goal across the stride (UE Foot Placement interpolates the pelvis between planted feet; or take the plane of
   both locks rather than the lower foot's need), verified by the same jerk column. Owner: `scene
   level-anim-stairs`, camera `slopes`, walkers going down 20/30°: the hips bob once per step.
4. **Sprint and run-strafe cycles** (impact: medium, already 3.4 "left"): a sprint cycle clip (free pack) so the
   7 m/s sprint doesn't play the run at 2.2x. Clip sourcing slice, then a bake.
5. **Starts/stops/pivots are cycles only** (impact: medium, G7): distance matching with start/stop clips from the
   free packs. Its own spec slice (animation.md open question 5); after P-IK1, since the foot lock fixes most start
   and stop slide.
6. **No look-at** (impact: low-medium on "alive" feel): the `LookAt` op exists; a slice to add it to the humanoid
   graph and feed the camera aim for the player (head/neck/spine weights, ±80°), NPCs the dialogue partner. Industry
   standard for third-person (Lyra/ALS, TLOU2, RDR2).
7. Docs: ROADMAP 2.1 "too much inertia" is stale (PL-4 confirmed); 3.4's "IK seems not to work" is P-IK1.

## 6. Industry check

- **UE5 (Lyra, Game Animation Sample)**: the same op stack we built: Foot Placement node (plant/lock, pelvis modes),
  Leg IK, Stride Warping, Orientation Warping, Distance Matching for starts/stops/pivots, Inertialization; the
  capsule's floor traces feed foot IK for **every** character, the locally controlled one first. Motion Warping for
  traversal (we have it: `WarpTarget`). Animation ticks after `CharacterMovementComponent` (tick prerequisite) and
  does not tick while the world is paused unless `bTickEvenWhenPaused`.
- **Unity**: Animator IK pass (`OnAnimatorIK`, "Foot IK" on humanoids), Final IK Grounder (pelvis offset + foot
  placement on the player), Animation Rigging two-bone IK; `timeScale = 0` stops a Normal-mode Animator.
- **Shipped third-person games** (Elden Ring, TLOU2, RDR2, Horizon): player foot IK on slopes and stairs is a baseline;
  head look-at toward the camera/target is common; starts/stops come from dedicated clips or motion matching.
- **Gaps vs ours**: (1) player foot IK not fed (ours is built, just not wired: P-IK1); (2) pause gate (P-IK2);
  (3) distance matching (G7, logged); (4) look-at not used; (5) orientation warping not used (fine today: facing
  follows travel except when locked on, where 8-way strafe clips play).

## 7. Player IK after P-IK1 (P-IK1b, owner PL-7 failed)

Owner 2026-11-11: legs "spreading a lot", the IK "overriding the animation"; slope lean "outrageous", "head super
tilted", while the same ops looked right on walkers. Measured by `player_ik_vs_walkers` (`mw_graphs/tests/slope_walk.rs`,
ignored): the shipped humanoid graph with probes, the player's mover (feel.rs springs, 720/1080/480 deg/s turns,
measured half-life 0.025 s) next to the walkers' (default mover, 0.05 s), each against a **twin** graph fed the same
motion without ground (= the animated pose). Values are IK minus twin: "fore-aft" is phase-contaminated (the grade term
changes the rate) and is not used below.

| Surface, gait | grade npc / player | lean mean (peak) | head pitch | pelvis cm | foot lateral max cm | lock drift max cm |
|---|---|---|---|---|---|---|
| flat walk 1.6 | 0 / 0 | 0 | 0 | 0 | 2.1 | 5.5 / 3.2 |
| up 20° walk | 0.36 / 0.36 | +11.2° (+14.7) | +15.4° | -11.9 | 6.8 | 3.5 |
| up 30° walk | 0.57 / 0.57 | +17.3° (+21.5) | **+23.4°** | -16.2 | 6.7 | 4.7 |
| up 40° walk (flat-slopes only) | 0.83 / 0.83 | +17.7° (+21.0, cap) | **+23.2°** | -21.4 | 6.7 | 6.5 |
| down 40° walk | -0.83 | -2.3° | -4.5° | -20.4 | 6.6 (knees +8.6) | 3.2 |
| up 30° run 4.5 | 0.51 / 0.51 | +12.9° (+21.3) | +15.2° | -10.9 | 6.7 | 4.6 |
| stairs 0.15/0.30 down, run | -0.27 / -0.30 | -0.5° | -0.7° | **-28 / -31** | 4.0 | 5.6 |
| stairs down, sprint 7 | -0.15 / -0.17 | 0 | 0 | **-31 / -24** | 4.5 | 7.2 |
| 90° turn on 20°, walk (0.6 s) | 0.34 / 0.26 | +10.9 / +10.4° | -9.8 / +20.2° | -10 / -6 | 6.4 / **15.2** | 20.1 / **33.7** (1 unplant) |
| 180° turn on 20°, run | 0.15 / -0.03 | +5° | +10 / +12° | -8 / -6 | **32.5 / 21.3** | **31.6 / 21.7** |

Probe sample error vs the ground under it: 0.0-0.6 cm on ramps (stairs 15-25 cm = the sphere catching the nosing, same
for both). Straight-line rows match npc vs player to within 0.03 grade, 0.5° and 1 cm on every surface and gait.

**Hypotheses.**
1. *Probe frame mismatch*: **ruled out.** The real player path test (`mw_player_bevy/src/tests_avatar.rs:525`) asserts
   samples within 1 mm of the ramp and exact normals in the drawn frame; `FootProbes::queue`/`write`
   (`sf_controller_bevy/src/lib.rs` ~650-730) are shared; the scale is the body child's (`sf_character_bevy/src/lib.rs:987`),
   the walkers' `body_scale`; yaw by the same `Mover::yaw_of`.
2. *Grade too large/noisy*: **ruled out as a player bug, confirmed as a surface effect.** Grade = npc's; the capsule is
   never probed (`GROUND_LAYERS` = STATIC|TERRAIN|PROP, `sf_physics_bevy/src/lib.rs:131`). But PL-7's `level-flat-slopes`
   has a **40° ramp and 0.25/0.22 stairs** (`levels/flat-slopes.ron:23,31`) that walkers were never judged on
   (anim-lab stops at 30°): grade 0.83-1.0 pins `Lean` at its cap 0.45 rad (25.8°).
3. **Head pitch (root cause of "head super tilted")**: `Lean` (`procedural.rs:506-518`) adds chest 0.45 + upperChest 0.4
   + neck 0.15 = 1.0 of the pitch cumulatively, so **the head takes the whole pitch** (+23° at 30°+, more than the trunk's
   17°) and nothing levels it. Same on walkers, but there it is seen from the side at 2 m/s on ≤ 30°; the player is seen
   from behind/above, so a 23-26° head drop reads as the head hanging. Humans keep the head near level on slopes (gaze
   stabilisation); UE/ALS spread lean over the spine and keep the head on an aim/look layer.
4. *Speed*: **confirmed for the pelvis.** At run/sprint the same `FootPlacement` drops the pelvis 28-31 cm going down
   stairs (walk 18 cm) and 18-21 cm on 40° (a crouch: knees bend and splay, "legs spreading"). Nothing scales foot
   placement or pelvis drop with speed (`guards.max_speed` 8 m/s, `asset.rs:524`; `max_drop` 0.4 of reach).
5. **Foot lock on fast turns: confirmed (player-specific kinematics).** The player's 720 deg/s walk turn holds a planted
   foot up to 33.7 cm from the animated one (walker 20 cm), lateral 15 cm, until `unplant` (0.3 × reach, `asset.rs:524`)
   releases it; 180° run turns 21-32 cm for both. Locked feet held in world while the body spins = spread/crossed legs.
6. *`StrideWarp` grade term*: **ruled out** as the spread: x0.63-1.21, identical for both, drift on straight ramps ≤ 7.8 cm.
7. *Player-only extras*: none touch legs or spine pitch in plain locomotion (stance slot only with a weapon drawn,
   `combat.rs:378`; HandIk weight 0 off mantles/brace; no look-at). **Not settled**: render-rate ≠ 60 Hz, the correction
   spring and the `Physics` trimesh probes are outside this proxy (the real-path test uses a toy graph without `Lean`).

**Fix slices (ranked; one deliverable each).**
1. *P-IK4 head stabilisation in `Lean`*: give the op a head joint with a negative share (or a "keep head level" term) so
   the head pitch stays within ~±5° of the animation; UE Control Rig / ALS head-look layer, Unity Animation Rigging
   Multi-Aim on the head. Verify: this test's head column ≤ ~5° on every ramp, `slope_lean_table` trunk unchanged. Owner:
   `level-flat-slopes` camera `ramps`, player third person on 30° and 40°, head looks ahead.
2. *P-IK5 turn-aware foot lock*: release a lock when the root's yaw since plant exceeds a limit (UE Foot Placement
   "Plant Settings": rotation/distance unplant limits; GASP unplants on turn-in-place), not only on 0.3 × reach drift.
   Verify: turn rows' lock drift ≤ ~10 cm, lateral ≤ ~8 cm, no NaN. Owner: quick 90/180° turns on the 20° ramp, K A/B.
3. *P-IK6 speed-scaled pelvis drop / IK weight*: cap the pelvis offset and ease foot placement toward the animation
   with speed (UE Foot Placement pelvis "Max Offset" + speed-based alpha; GASP lowers IK at run/sprint). Verify: pelvis
   column at run/sprint on stairs down ≤ ~12 cm. Owner: running down `stairs-comfy` and the 40° ramp.
4. *P-IK7 lean cap by grade*: saturate the grade input near 30° (beyond it, a climb, not a lean). Owner taste: ask with 1.
5. *P-IK1c real-graph player test*: the baked humanoid graph under `avatar::feed` at 60 and 144 fps (closes 7).

Command: `bash bevyrrowind/tools/cargo_turn.sh test --release -p mw_graphs --test slope_walk player_ik_vs_walkers --
--ignored --nocapture` (needs a Morrowind install; ~2.5 min).
