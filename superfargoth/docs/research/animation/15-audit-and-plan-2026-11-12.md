# 15. Animation audit and plan (2026-11-12)

Owner: the player's walk is "super wonky" after a lot of work; goals are an Elden Ring-like third person and a
complete, Dark Messiah-like first-person melee. Research: `14-rust-rewrites-and-fp-melee.md`. Inputs read:
`../ik-feel-diagnosis-2026-11-11.md`, `../locomotion-rethink-2026-11-11.md` (their L0-L8 slices are **not
started**: no ROADMAP line, last animation commit `ff6060e` P-IK5), specs first-person/camera/combat, ROADMAP
3.1/3.4/3.4b/3.5/1.18, visual-check verdicts, and the code cited below. No code was changed and nothing was
run. Marks: **[V]** verified in code this session, **[M]** measured by an earlier slice (doc cited),
**[I]** inference to confirm by a measurement slice before building.

## 1. Verdict

The architecture is the industry's (Lyra shape): capsule-driven locomotion on a 60 Hz fixed tick, input as
commands, render interpolation, a pose graph with sync, inertialization and a procedural op stack, root motion
only for traversal and combat clips, presentation never feeding gameplay. Keep it. **The walk is wonky because
of what the stack is fed, not because of the stack:** clips whose speed does not fit the player's gaits, no
transition clips, a body bolted to a very fast capsule, and correction ops (stride warp, foot lock, pelvis,
lean) asked to hide all of that. Each op was tuned on NPC walkers at 2 m/s; the player was first seen with them
on 2026-11-11 and failed (PL-7 partial). Two months of work went into the top of the stack while the base
(clips, gait set, transitions) stayed at "Morrowind cycles only".

## 2. Why the walk looks wonky, ranked

1. **Gait clips do not match gait speeds; the rate range hides it.** [V] Free movement plays the Morrowind
   `Walk*`/`Run*` sets (`bevyrrowind/crates/mw_graphs/src/build.rs:720-724`, header `:22-24`), authored at about
   2.2 m/s and 2.67-3.2 m/s (`build.rs:918-919`, `:932`). The player moves at 1.6 / 4.5 / 7 m/s
   (`ik-feel-diagnosis` §1, §2). `SpeedMatch` allows 0.5-2.2x with half the ratio to cadence
   (`build.rs:925-936`; `sf_anim_graph/src/instance.rs:1527-1533`), `StrideWarp` up to 1.2x (`build.rs:944`).
   So the walk plays at ~0.85 rate with steps shortened ~15%, the run at ~1.2-1.4 rate with steps stretched to
   the 1.2 cap, the sprint at 2.2 (4.75 steps/s, already logged). Shipped practice keeps the rate near 1 (about
   0.8-1.2) and picks **another clip** outside it (note 14 §3). Stretched 2002 keyframe cycles with a warped
   stride are the first thing to suspect in "wonky", on flat ground, before any IK. The bundled UAL
   `Walk_Loop`, `Jog_Fwd_Loop`, `Sprint_Loop` are unused for free movement (only UAL *strafe* loops were tried
   and rejected, 2026-10-27).
2. **No start, stop or pivot; the foot lock turns a stop into a pop.** [M] `locomotion-rethink` §1a, §6:
   14 of 16 measured stops unplant a foot and slide it 40-100 cm at 3-10 m/s; hips rise 2-9.5 cm from the pose
   change alone. [V] every gait switch is one 0.15 s inertialization (`build.rs:354`, `:867-868`); braking
   half-life 0.05 s (`mw_player_bevy/src/feel.rs:21`).
3. **Walk and run are two states switched by a bool, not a speed blend.** [V] `RUNNING` comes from the gait
   intent, not the speed (`sf_controller_bevy/src/lib.rs:571-581`, `run_speed` infinite `:303`), and
   `WALK`/`RUN` are separate states (`build.rs:388-389`). A run started from rest plays the run cycle at the
   0.5 floor while the capsule accelerates; a half-tilted stick runs the run clip slowed and shortened; a gait
   toggle is a 0.15 s morph between unsynchronised cycles. [I] whether the entered state is phase-matched to
   the left one was not verified (measure in A1). Standard: one 1D blend (walk, jog, sprint) on speed with
   foot-down sync, entered from start clips.
4. **The body is bolted to a capsule that turns at 720-1080°/s.** [V] `feel.rs:23-29` (facing half-life 0.1 s,
   velocity 0.07 s). The free-movement states are 4-way blend spaces on the measured *local* move direction
   (`build.rs:499-510`, smoothing 0.05 s `:357`). [I] During every sharp turn the travel direction leaves the
   body's forward for a few frames, so the blend swings toward the strafe and back lanes, then back: legs
   cross and the cadence hiccups. There is no offset root, orientation warp or turn lean in the humanoid graph
   (`ik-feel-diagnosis` §1 table: "not in the graph (ops exist)"), and turn-in-place states only run while
   standing (`build.rs:390-391`). P-IK5 (PL-9 "seems ok") fixed the foot lock's share, not this.
5. **The op stack over-corrects on slopes and at speed.** [M] rethink §1c-1e, diagnosis §7: head pitch +23°,
   arms carried back by the trunk lean, downhill steps lengthened by the grade term (wrong sign versus
   biomechanics), pelvis drop 28-31 cm running down stairs with no speed cap, per-step pelvis bob growing with
   grade (the "jolty, syncopated" walk). Built slices so far: P-IK1, P-IK2, P-IK5 only.
6. **Animation speed is a finite difference of the drawn root.** [V] `mw_player_bevy/src/avatar.rs:230-252`
   measures velocity and turn rate per render frame from the interpolated transform, low-passed 0.025 s
   (`feel.rs:33`). Sound in principle, but [I] the correction spring and frame rates other than 60 Hz were never
   tested on the real path (diagnosis §7 item 7 "not settled", P-IK1c not built); any noise goes straight into
   the play rate and the stride scale. Standard: feed the mover's own velocity (UE reads the movement
   component), keep the drawn root only for the root delta.
7. **Nothing was ever signed off on the player.** AL-2 (walk loop, turns) and AL-3 are still "awaiting owner";
   every confirmed foot check (AL-9, AN-1, AN-21) was on walkers. The measurement tests compare IK against a
   twin without IK, so they cannot see causes 1, 3, 4 at all.

Ruled out earlier and still out: update ordering, fixed tick versus render, Delirium, probe frames
(`ik-feel-diagnosis` §2, §3, §7). Pause (1.18) is built, awaiting PL-8.

## 3. Architecture against shipped games

| Question | Shipped practice | Ours | Verdict |
|---|---|---|---|
| Who moves the character | capsule for locomotion (UE, Unity, Bethesda); clip root for attacks, traversal (all) | same (`RootMotionMode::Progress` on the montage slot) | match |
| Tick split | fixed sim, interpolated presentation, input commands | `sf_action_bevy` FixedUpdate 60 Hz, `interpolate`, `Command` | match; two crates insert `Time<Fixed>` (diagnosis §2): give it one owner |
| Who owns the pose | one graph per character, IK last, physics blend after | `sf_anim_graph` instance, ops, `PhysicsPose` | match |
| Gait set | clip per gait at its speed, rate clamp ~0.8-1.2, stride warp for the rest | one clip per bool state, rate 0.5-2.2 | **gap** (cause 1, 3) |
| Starts, stops, pivots | distance-matched clips or motion matching | none | **gap** (G7, rethink L3-L5) |
| Fast turns | offset root or twist unwind, orientation warp, turn-in-place | rigid body, 4-way blend | **gap** (cause 4) |
| Foot IK control | per-state gain (Havok control track, UE curve alpha) | one `on_ground` bool gate (`build.rs:953`) | **gap**, small |
| Head | look-at / aim layer | none; `Lean` gives the neck a share | **gap** (rethink L6) |
| Motion matching | GASP, TLOU2 | deferred | right call with our clip supply |

**Over-built:** slope ops tuned to 40° ramps and stair tables before the flat walk was good; the 0.5-2.2 rate
range and its special cases (`build.rs:920-935`); `StrideWarp`'s grade term; the many `slope_walk`
measurement tables that only compare IK to no-IK. **Missing:** the gait set, transitions, offset root or
orientation warp, look-at, an IK gain curve, a sprint cycle. **Delete or retire:** `Lean`'s neck share and
P-IK4 (superseded by the gaze layer); the downhill-lengthening grade term; `SpeedMatch` beyond the clamp once
the gait set lands; Morrowind `TurnLeft/Right` for the modern humanoid once UAL `Turn90/180` are slotted (keep
for the Classic preset); the fixed first-person eye (`sf_camera_bevy/src/lib.rs:131`, `:517`) once F2 lands.

## 4. First person as a base for Dark Messiah-style melee

Sound and worth keeping: one skeleton and one clip set for both views, roles (`sf_character::ViewRole`), head
hidden with its shadow kept (FP-1, FP-1b), a single-pass view model with its own FOV (FP-2, the UE 5.5
shape), one gameplay body deciding hits from baked socket tracks, soft aim and attack look limits in the look
accumulator so the host sees the same view (camera.md "First person"). This is technique C of note 14 §2 and
is what full-body first-person games converge on. Shared gameplay with third person is real, not planned.

Gaps, in order of how much they block a Dark Messiah feel:
1. **The eye is a fixed point, not on the body.** [V] `root + 1.62 m up + 0.12 m forward`
   (`sf_camera_bevy/src/lib.rs:131`, `:517`); the baked camera track "arrives with D2"
   (`sf_camera/src/first_person.rs:4`), not built. The body animates under a floating camera: hit reactions,
   rolls, mantles and a future kick are not felt, and the collar or chest can cross the near plane.
2. **No aim pitch.** [V] no code turns view pitch into spine or arm pitch (grep of `sf_anim_graph`,
   `mw_graphs`, `mw_player_bevy`, `sf_camera_bevy`). Looking down or up, a swing stays at the clip's height and
   leaves the view; the socket sweep ignores pitch too, so a low or high target cannot be hit by aiming.
3. **No view kick** on hits (decided in camera.md, "not built"), no **world-hit recoil**, no **kick** move.
4. **Third-person clips are unchecked from the inside**: no per-move framing pass; FP-6 (override clips) is
   last in the build plan.
5. **Seven first-person and combat checks are unseen**: FP-1, FP-1b, FP-2, CB-3a, CB-3b, CB-5, CB-5b all
   "awaiting owner". Building F-slices on top repeats the walk's mistake; the owner batch comes first.

## 5. Plan

One deliverable plus its test per slice. Logic gets exact tests; look and feel get sanity tests, printed
measurements and an owner scene (agents never judge pictures). Build slices hand off at ~80-100k. Test and
scene names are proposals. "rethink Lx" = `locomotion-rethink-2026-11-11.md` §4, kept as written.

### Phase A: the visible walk (diagnose, then the smallest fixes)

| # | Slice | Files | Verify | Needs |
|---|---|---|---|---|
| A0 | **Owner A/B ladder** (build, small): console flags to switch off, one by one, `SpeedMatch` (rate 1), `StrideWarp`, `FootPlacement` (exists: K), `Lean`, and "forward lane only"; register scene WALK-0 | `mw_graphs/src/build.rs` (op enable params), game console glue, `visual-checks/player-ui.md` | exact: each flag zeroes its op's output in a headless graph; owner WALK-0 on flat `level-flat-slopes`: which switch makes the walk look right | - |
| A1 | **Gait measurement** (test only) | `mw_graphs/tests/slope_walk.rs` new `player_gaits` (ignored) | prints per gait and at half stick: clip natural speed, ratio, rate, stride scale; side/back lane weight peak during 90/180° turns at walk and run; phase jump at walk<->run; rate jitter with the real `avatar::feed` at 60 and 144 fps (this is P-IK1c). Sanity: no NaN | - |
| A2 | **Gait clip dance card** (research) | new section in `bevyrrowind/docs/specs/animation-sources.md` | table: authored speed and step length of Morrowind walk/run, UAL `Walk_Loop`, `Jog_Fwd_Loop`, `Sprint_Loop`, 8-way loops, Kimodo candidates versus 1.6/4.5/7 m/s; which clip per gait keeps the rate in 0.8-1.2; reuse check, licences | A1; owner Q1 |
| A3 | **Speed gait set** (build): humanoid free movement becomes one synced 1D blend walk/jog/sprint on speed; `SpeedMatch` clamp 0.8-1.25 for it; sprint gets its cycle; Classic keeps the Morrowind states | `mw_graphs/src/build.rs`, `data/animation/sources.ron`, graph RON | exact: lane picked per speed band, slots resolve or fall back; A1 table reprinted with rates inside the clamp; `bevyrrowind bake`; owner WALK-1 (walk, jog, sprint on flat, analog speeds) | A2 |
| A4 | **Switch-aware plant** = rethink L2 (stop pop without clips) | `sf_anim_graph/src/procedural.rs` | exact unit test "contact jump on a state switch does not lock a moving foot"; `player_stop` table: unplants 0; owner LOCO-1 | - |
| A5 | **Turns: body follows, legs do not strafe** (build): offset-root yaw spring on the render body (capsule snaps, body eases, FromSoft/UE offset root) and the existing orientation-warp op for the residual angle, free movement only | `sf_anim_graph` (op wiring), `mw_graphs/src/build.rs`, `mw_player_bevy/src/avatar.rs` (presentation yaw only) | exact: gameplay yaw unchanged, render yaw converges; A1 turn rows: side-lane weight ~0; owner WALK-2 (sharp 90/180° at walk and run) | A1 confirms cause 4 |
| A6 | **Feed the mover's velocity** (build, only if A1 shows jitter) | `mw_player_bevy/src/avatar.rs:230-252` | exact: fed speed equals the mover's at 60/144 fps; printed rate jitter | A1 |

### Phase B: locomotion base (rethink L0-L8, order kept)

| # | Slice | Verify | Needs |
|---|---|---|---|
| B0 | rethink L0 docs: animation.md "Locomotion transitions" plus "Gait set" from A2/A3; industry-audit G7 | doc review | A3 |
| B1 | rethink L3 distance curve at cook (`sf_anim/src/cook.rs`) | exact, synthetic clip | - |
| B2 | rethink L4 seek by distance + stop predictor (`sf_anim_graph`, `sf_controller`) | exact: predicted stop within 1 cm of a simulated mover | B1 |
| B3 | rethink L5a start/stop/pivot dance card (research), then L5b Kimodo generation + bake (data) | owner LOCO-2 in `clip-preview --browse` | A3 (style must match the chosen cycles) |
| B4 | rethink L5c states, stop clip by planted foot (AC1 and Lyra rule, note 14 §1.3); UAL `Turn90/180` turn-in-place | exact: chosen clip matches the planted foot; owner LOCO-3 | B2, B3 |
| B5 | **IK gain curve** (build): optional per-clip `ik.feet` curve multiplying `FootPlacement` weight (Havok control track, note 14 §1.1) | exact: curve 0 gives the animated feet | - |
| B6 | rethink L6 gaze layer; L7 slope posture; L8 downhill stride; P-IK6 rescoped (pelvis max offset by speed, asymmetric spring); P-IK3 after L8 | printed tables as in the rethink; owner LOCO-4..6 | A3 (retune on the new cycles, not before) |

### Phase C: third-person combat feel (combat.md build plan, unchanged order)

| # | Slice | Verify | Needs |
|---|---|---|---|
| C0 | Owner batch: CB-3a, CB-3b (orchestrator launches `level-sparring`); log verdicts | verdicts in visual-checks | - |
| C1 | combat.md C2c backstep and knockdown; then C3 heavy charge/release, openers, poise | combat.md tests; CB-3, CB-4 | C0 |
| C2 | Locked-on strafe on the new gait set (C1c-2b): 8-way loops from A2's card, orientation warp between lanes | exact: lanes sync; owner CB-2 re-check | A3, A5 |
| C3 | combat.md C4 remainder: combat shake and view kick (`CameraShake` fed by `HitLanded`), target switching | exact: shake never reaches `Command.view`; CB-5 | - |
| C4 | Attack clip pass: new sword clips (CB-3 follow-up), `ik.feet` curves on attacks and rolls | bake; owner CB-1 re-check | B5 |

### Phase D: first-person combat (Dark Messiah direction)

| # | Slice | Files | Verify | Needs |
|---|---|---|---|---|
| F0 | Owner batch: FP-1, FP-1b, FP-2 (FP-2c), CB-5, CB-5b; decide open question 4 (FOV seam) | - | verdicts logged | - |
| F1 | **Aim pitch** (build): `Command.view` pitch drives a spine/chest/clavicle pitch layer (share per joint, clamp); the socket sweep applies the same pitch on the fixed tick | `sf_anim_graph` (op), `sf_combat` sweeps, `mw_graphs/src/build.rs` | exact: sweep at pitch p equals the baked track rotated by p about the pitch pivot; host and client agree; sanity: no NaN at ±85°; owner FPM-1 (hit a low and a high dummy in both views) | F0 |
| F2 | **Head-joint eye** (camera.md D2): eye position from the head joint, low-passed, rotation from the view; per-move camera-motion amount and a comfort setting; replaces the fixed eye | `sf_camera/src/first_person.rs`, `sf_camera_bevy/src/lib.rs:514-518` | exact: eye equals the filtered head at rest; never feeds `Command`; sanity: bounded offset through a roll and a hit; owner FPM-2 | F0 |
| F3 | **Kick** (build): moveset entry in both views, foot socket sweep, stagger tier, impulse on the target and on props through `Physics`; stamina cost and recovery from the sheet | combat sheet RON, `sf_combat`, clip slot `attack.kick` (UAL or Kimodo) | exact: window, contact order, impulse sent once; owner FPM-3 (kick the dummy and a crate, both views) | C1, F1 |
| F4 | **World-hit recoil** (build): a weapon sweep that meets static geometry ends the swing in a recoil move (rule seam: on or off per profile) | `sf_combat`, sheet | exact: wall contact before a target cancels the hit; owner FPM-4 | - |
| F5 | FP-3 held items in the view model; FP-5 wall pull-back | first-person.md plan | first-person.md tests; FP-3, FP-5 | F0 |
| F6 | **First-person framing pass** (research then data): per attack, from the F2 eye, is the weapon in view through windup and strike; fix with an additive view pose or hand IK target, else an FP-6 override clip | `clip-preview` first-person mode, sheet `look` | owner reviews every move once (FPM-5) | F1, F2 |
| F7 | Later: directional power strikes, ledge kicks and environment kills, paired finishers with a framed camera (combat C6) | - | - | owner Q5 |

**Order:** A0, A1 (parallel with the owner batches C0/F0, which need no build), A2, A3, A4, A5, then B, C, D.
Nothing in B6 is retuned before A3: tuning slope ops on clips we are about to replace is the trap we are in.

## 6. Owner questions (taste and scope only)

1. **Free-movement clips for the modern preset.** Keep Morrowind's 2002 walk/run (familiar, stiff, wrong
   speeds for our gaits) or switch to pack/generated loops, with Morrowind's kept for the Classic preset?
   Recommended: switch; A0 lets you see the difference first.
2. **Stop length, slope slowdown, where the head looks**: `locomotion-rethink` §5 questions 1-3, unchanged.
3. **First-person camera motion.** How much of the body's motion should the view carry (Dying Light: a lot;
   Skyrim: none)? Recommended: position follows the head, rotation stays yours, with a comfort slider.
4. **Kick.** A dedicated button in both views, with physics pushes and environment kills as the goal?
   Recommended: yes; it is the cheapest piece of the Dark Messiah feel.
5. **Attack direction in first person.** Movement keys choose the swing (Dark Messiah power strikes) or mouse
   drag (Mordhau)? Recommended: movement keys; it also works on a pad and in third person.
6. **First-person FOV seam** (first-person.md open question 4), at FP-2c.

**Owner answers 2026-11-12.**
1. Yes: the modern preset drops Morrowind's walk/run. Generate the free-movement set (walk/jog/sprint cycles **and**
   starts, stops, pivots, turns in place) with the clip factory (Kimodo, `bevyrrowind/docs/specs/motion-generation.md`), so cycles and
   transitions share one style; Morrowind's clips stay for Classic. B3 (L5a/L5b generation) moves up next to A2/A3.
2. Not fixed yet: rethink §5 Q1-3 stay open, to be answered with the A0 ladder in front of the owner.
3. Both styles are wanted: Doom/Source (stable view, no body sway) is popular; Mirror's Edge/Dishonored (view on
   the body) is fine too. Ship both as a first-person camera option; F2 builds the body-driven one, the stable one
   is the default eye with sway off. The eye height always follows the character's actual height (race and NPC
   height scale), never a fixed 1.62 m.
4. Yes: dedicated kick in both views, physics pushes.
5. Movement keys choose the swing direction.

Decided from the standard, not asked: rate clamp 0.8-1.25 (Lyra), offset-root spring, foot-IK gain curve, aim
pitch shares, eye low-pass, host-side sweeps with commanded pitch.

## Open

- Causes 3 (phase at walk<->run) and 4 (strafe-lane bleed in turns) are read from code, not measured: A1.
- Authored speeds of the UAL loops were not read; A2 does it.
- `sf_action` step, `sf_combat` and `mw_actors` were audited through their specs, not line by line; the
  untracked working-tree changes in `sf_anim*` and `mw_graphs` (git status) were not diffed.
- Lyra's exact clamp and UE 5.5 first-person fields are from memory (note 14 "Open").
