# Lane roadmap: Animation for gameplay (locomotion, styles, interaction clips, combat clips, physics tint)

Specs: `superfargoth/docs/specs/{animation,animation-plan,locomotion,action}.md`, `bevyrrowind/docs/specs/{animation-sources,
motion-generation}.md`. Research: `docs/research/gameplay-2026-11-25/{anim,anim-diagnosis,anim-style}.md` (anim-style's
"Orchestrator corrections" win), `gta6.md` §6, `combat.md` §1/§4; R19 = `superfargoth/docs/research/animation/19-kimodo-prompting-2026-11-12.md`.
Ledgers `docs/ledger/animation.md`, `combat-feel.md`; ROADMAP 3.1, 3.4, 3.4c, 3.4d, 3.8. Kept IDs: A*/B*/C* (2026-11-12
plan), MG*, AG*, P-IK*; new slices `AN<n>`, owner checks `ANIM-<n>`.

## 1. Player-facing goal

Modern preset, lane done: player and NPCs walk, jog and sprint on clean generated cycles with real starts, stops and
turns, arms swinging naturally, upright on slopes [1]. Each NPC moves like who they are: the old shuffle, the heavy plod,
the wounded limp, the drunk sway, the tired slump [3]. People sit, pull out chairs, drink at tables, open doors and carry
things with clips that fit their body [4]. Swings are Morrowind's own attacks, retimed for an Elden Ring feel, beside a
fast spammable roll [5]. Rare strong states (stagger, stumble, collapse) react physically through Delirium [6].
**Classic keeps** Morrowind walk/run (`Preset::Classic`), attacks over walking legs, no style classes, no tint.

## 2. Where we are (2026-11-25)

- **Confirmed today (uncommitted):** slope lean fix (small, only moving, head and upper arms counter-rotated; PL-10;
  P-IK4's level head folded in); spammable roll (buffered, press-to-roll in moves, 0.58 s cadence, UAL `Roll` rate 1.6;
  CB-6 "roll is good").
- **Built:** A3 gait set on Kimodo loco1 cycles (loop cutter, rate clamp 0.8-1.25, sync groups; WALK-1, LOOP-1
  confirmed), A0 A/B ladder (WALK-0 awaiting), A4 switch-aware plant, matched trunk retarget + shoulder `pose_offsets`,
  style seam `loco.<style>.<gait>.*` with base fallback. Factory: Kimodo worker, census, `clip-preview --browse` (Y/N/M),
  AG1 `motiongen grade` (recall 33-38 %), loco1 228 clips (LOCO-2 picks pending), tier1 268 activity clips. `LookAt`,
  `OrientationWarp` ops exist, **unwired**. Delirium default: knockdowns/deaths via `RagdollTakeover` (PH-14/15).
- **Owner verdicts open:** Kimodo walk/run still bad (fix generation/retarget, no pack); combat clips (UAL2 sword chains)
  very bad; most Quaternius clips bad; review tool weak; NPCs need many style variants.
- **Known, unfixed:** elbows 6-7 cm from the chest on `walk.cycle.1`, run.3 hands cross the midline (body-capsule arm IK
  not built); Kimodo arm swing hangs back (-36..+15 deg vs Morrowind -40..+31); path-forced run/sprint skate 83/179 cm/s
  (R19 §3: sparse 1 Hz keys, speed out of distribution); no start/stop/pivot clips; Morrowind attack groups registered as
  `upper` clips (`WEAPON_SECTIONS`) but played by no sheet.
- **Not started:** style model, MotionBricks (MG8; v0.2.0 SOMA model out), grip table MG7b, AG2-AG4, MG9.

## 3. Decisions

1. **Fix the generated base, never swap in a pack** (owner 2026-11-25: 100STYLE rejected, no mocap or paid packs, no
   Bones SEED). Locomotion and interaction = Kimodo (MotionBricks for links and style presets); combat = Morrowind.
   Every "100STYLE base" and "Kimodo = seeds only" line in the research is superseded.
2. **Generate free, retime at bake** (R19 §4-5): no forced speed on cycles, dense per-frame root keys where a path is
   needed, retime ≤ 1.15x walk / 1.3x run; R19 prompt form. Sparse 1 Hz paths dropped.
3. **Retarget fixes live in the retarget** (UE5 IK Rig collision, retarget-pose offsets, DECISIONS 2026-11-12): arm IK
   against a body capsule and per-source offsets hold on every take; graph ops are retuned only after (ledger rule).
4. **Style = gait classes + continuous modifiers** (GTA V/RDR2 movement clipsets, TLOU injured sets, UE Chooser;
   anim-style §1, §4). A per-individual `MotionProfile` (data, like the perception profile, DECISIONS 2026-11-25):
   `style_id` (stable, saved) + smoothed `age`, `injury` (+ side), `fatigue`, `load`, `intoxication`, `mood`.
   `(class, state) -> clipset` is a data table; modifiers (hunch, limp, breathing, slump, load offset) are tunables.
   **Classes are data:** a recipe plus slot rows, no code; batches keep coming (owner: "a LOT").
5. **LOD carries style** (animation.md "LOD and budget"): A0 all layers, A1 class + bone modifiers, A2 class + speed
   match, A3 none. A0 ≤ 25 µs per character within the 8.3 ms frame.
6. **Family consistency by filters** (anim-style §2.1): one style recipe string, the family's posture key pose at both
   ends, one speed envelope, numeric bands (speed, stride, spine pitch, asymmetry) after the physical gates.
7. **Interaction clips: one canonical body, IK fits the rest** (interaction.md "Clips fit any body", gta6.md §6): variants
   by side, entry, height band and hand, never body size; style = modifiers plus a second variant for sit down/stand up,
   more as later data batches. Each clip is a fragment so chains recombine (RDR2).
8. **Review is the bottleneck, so the tool is a stage** (owner; ~2 h per 200 clips): scores in the browser, side-by-side
   elimination (the owner rates by elimination), playback on the real graph and bodies, picks written to slots.
9. **Combat on Morrowind clips** (owner 2026-11-25; OpenMW ReAnimation: text keys drive timing): text keys become sheet
   windows, per-phase rate (strike 1.3-1.6x), additive anticipation/overshoot, step-in by the `Warp` owner. Missing
   moves are key-pose `PoseSeq` clips (Overgrowth, GDC 2014). The roll stays UAL `Roll` (CB-6) until beaten.
10. **Physics tint on Delirium, rare and capped** (DECISIONS 2026-11-19; anim-style §3 cost verdict): stagger, drunk or
    wounded stumble, exhausted collapse, medium and large hits (combat.md §4); A0/A1 only, concurrent cap (tunable), additive
    fallback over the cap and in Classic. Baseline limp is never physics. Framework seam `PhysicsTint`; Delirium fills it.
11. **Motion matching stays parked** (anim.md §2): no shippable dataset.

## 4. Stages

Stage 2 runs beside Stage 1 and lands before Stage 3's first review; Stages 4-5 start after Stage 1. Stage 0 is done:
P-IK4 + lean fix (PL-10) and the roll chain (CB-6), both 2026-11-25.

### Stage 1: clean walk and run (player and NPCs share the graph)

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| AN1 | Diagnosis of the shipped cycles, flat and at speed: arm swing, elbow/hand clearance, spine pitch, bob, skate; ranks generation vs retarget vs graph | tests `mw_bake`, `mw_graphs` | - | **done 2026-11-26** (ledger: swing, skate, cadence = generation; 15.7 cm pelvis dip at 2.2 m/s = graph) |
| AN1b | Diagnosis: the graph pelvis dip at the walk/jog blend (2.2 m/s: 15.7 cm vs 4 cm clips; foot placement lock hold vs reach clamp vs SpeedMatch on a 2-lane blend) | tests `mw_bake` | AN1 | **done 2026-11-26** (heel-roll toe pin held on the blended roll curve, straight leg, pelvis lowered by −`rise_cap`; lock drift into release 2nd) |
| AN1c | FootPlacement: unplant a locked/pinned foot when the leg over-extends (extension ratio, UE5 Foot Placement / ALS) instead of lowering the pelvis to keep it | `sf_anim_graph` | AN1b | exact: 2.2 m/s dip ≤ clip bob + 1 cm; flat 1.6 unchanged | **done 2026-11-26 (toe pins)** (`FootGuards.extension` 0.99, UE `MaxExtensionRatio` form; dip 13.2→2.5 cm at 2.2, 18.0→3.4 at 2.4; stance-lock release dip 4.6-6.9 cm at 2.6-3.0 left for AN1d: unplanting stance locks slid run strafes 20-25 cm; GRAPH_FORMAT 15)
| AN1d | FootPlacement: plant and toe pin decided from the final blended pose (foot/toe speed + height), not thresholded blended curves | `sf_anim_graph` | AN1c | exact: no pin while the blended toe moves; owner ANIM-1 | **done 2026-11-26** (dip 0.5-1.1 cm at 2.2-3.0 m/s, was 2.5-6.9; strafe/back slide +0.9-2.0 cm -> AN1e; GRAPH_FORMAT 16) |
| AN1e | FootPlacement: heel rise measured as foot pitch (heel minus toe rise), so a whole-foot lift mid-contact (strafe/back clips) keeps the ankle lock; also log the 13 cm pose pop of the slow steep contour walk | `sf_anim_graph` | AN1d | exact: strafe/back slide within 0.5 cm of AN1d off; dip unchanged; owner ANIM-1 |
| AN2 | Retarget arm IK against a body capsule (eased like the leg IK) | `sf_anim`, `sf_character` | AN1 | exact: clearance on every take; no contact = byte-identical |
| AN3 | Swing-bias offset (upper-arm pitch, `pose_offsets`) if AN1 ranks it (AN1: range too small, not biased; fix in AN4 prompts first, this only if batch 2 still swings < ~50 deg) | `mw_graphs` data | AN1, AN5 | swing in Morrowind's range |
| AN4a | Worker: dense root paths from a speed profile (hold/ramp/steady/decel/rest, one key per frame) + `cfg_weight` per activity | scrib-motiongen | - | **done 2026-11-26** (`root = "profile:hold=,ramp=,speed=,decel=,rest=,turn="`, `cfg_weight = [text, constraint]`, Kimodo default [2,2] when absent; batch 1 plans byte-identical) |
| AN4b | Batch 2 recipe (R19 §6): free cycles + dense-path sweep, `cfg_weight`, starts/stops/turns/pivots, side/back cycles (C2); arm-swing and jog-cadence prompt arms from AN1 | scrib-motiongen | AN1, AN4a | **done 2026-11-26** (`recipes/loco-batch2.toml`: 41 activities, 246 clips, ids `loco2.*`; steps A/B left out: run-wide `--steps` only) |
| AN5 | Batch 2 generation + grade (≥ 6 samples per prompt) | generation | AN4b | **done 2026-11-26** (246 clips, 676 s; 26/246 gated, all skate: dense jog 3.5/4.5, jog stride prompts, sprint 5.0) |
| AN5b | Measurement: per-take arm swing, cadence, speed, bob, skate of the loco2 cycles (AN1 harness), ranked | tests `mw_bake` | AN5, AN1b | printed table; ledger | **done 2026-11-26** (`measure_an5b_loco2_takes`; short list in the ledger; no usable sprint) |
| AN6 | Picks into slots, bake retime limits | `mw_graphs` data | AN5, AN10 | `player_gaits` in clamp; owner **ANIM-1** (replaces LOCO-2) |
| A5 | Turns: body follows (offset-root spring), `OrientationWarp` for the rest | `sf_anim_graph`, `mw_graphs` | AN6 | exact; owner ANIM-2 |
| B1 | Distance curve at cook | `sf_anim` | - | **done 2026-11-26** (`distance.rs`, `ClipRules::distance`, 10 exact tests) |
| B1b | `anim.retarget` re-derives the `distance` curve on the target root (retargeted clips carry the source curve or none) | `sf_anim`/`sf_character` | B1 | **done 2026-11-26** (`apply_distance` after both retarget paths; `"distance"` retarget key param only when set) |
| B2a | Seek by distance: `Clip` node `distance: (value, rate_clamp)` | `sf_anim_graph` | B1b | **done 2026-11-26** (playhead = curve inverse, never back, clamp 0.75-1.25 (UE5); one-shot, unsynced only; GRAPH_FORMAT 14) |
| B2b | Stop/pivot predictor: where the mover comes to rest from its springs (closed form) | `sf_controller` | - | **done 2026-11-26** (`stop.rs`: `stop_offset`, `stop_time`, `pivot`; 8 tests vs the mover within 1 cm) | stop within 1 cm |
| B4 | Start/stop/pivot states, stop clip by planted foot | `mw_graphs` | B2, AN6 | exact; owner ANIM-3 |
| AN7 | Acceleration and turn lean in `Lean` | `sf_anim_graph` | AN6 | exact: no accel, no lean |
| AN8 | `LookAt` wired (head then eyes, limits, spring, target param) | `sf_anim_graph`, `mw_graphs` | - | **done 2026-11-26** (humanoid chest/upperChest/neck/head 8/10/30/60°, total 80°, A0; release 120°/rejoin 105° fixes the behind-the-head flip; no eye bones; GRAPH_FORMAT 13) |
| AN8b | Look target feed: NPC head tracking at the player (port OpenMW `Actors::updateHeadTracking`, cite file + commit), player none yet | `mw_walkers_bevy` | AN8 | **done 2026-11-26** (`head_track.rs`, OpenMW b8285cc9 `actors.cpp:270-363`; GMST `fMaxHeadTrackDistance` 400 u = 5.7 m, interior x0.5; 12 exact tests); owner **ANIM-4** OK 2026-11-26 |
| B6 | Gaze, slope posture, downhill stride, P-IK6, P-IK3 retuned on the new cycles | `sf_anim_graph`, `mw_graphs` | AN6 | printed; owner ANIM-5 |

### Stage 2: the clip review and pick tool

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| AN9 | Diagnosis: what slows decisions in `--browse` (owner notes, a timed session) vs dailies review; gap list in the spec | docs | - | **done 2026-11-26** (`motion-generation.md` "Browse review: diagnosis", 8 ranked gaps; no timed session, owner away) |
| AG2 | Browser reads `grades.json`: overlay, sort, candidates only (absorbs ROADMAP's MG10b) | `mw_viewers_bevy` | AN9 | list order; owner ANIM-6 | **done 2026-11-26** (C candidates only, V view all/unrated/maybe/keep, gated last per activity, grade line in the overlay)
| AN10 | Side-by-side elimination: 2-4 takes synced in a row, reject drops one, last ones get keep/maybe | `mw_viewers_bevy` | AG2 | exact list state; owner ANIM-6 **done 2026-11-26** (G group review, one shared `GroupClock`, 4 exact tests; owner ANIM-6b) |
| AN46 | Review controls: Z undo (Backspace today clears the NEXT clip, `browse.rs:174-190`), pause/frame step/speed, number-key reason tags into the note, mirror preview of `mirror_candidate` | `mw_viewers_bevy` | AN10 | exact list state |
| AN11 | In context: a take through the shipped graph at slot speed (treadmill, ramps) on chosen bodies (old/young, races, female rig) | `mw_viewers_bevy` | AN10 | pose = `mw_graphs::sim`; owner ANIM-6 |
| AG3 | Fitted gates + score; `motiongen calibrate` from `verdicts.toml` | scrib-motiongen | AG2 | AUC printed |
| AG4 | Target-body arm/chest check on retargeted clips | `mw_bake` | AN2 | exact on fixtures |
| MG9 | Keep verdicts write `sources.ron` rows with class/family tags | `mw_viewers_bevy`, `mw_graphs` | AN10 | round trip; bake loads |

### Stage 3: NPC style variety (ROADMAP 3.4d)

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| AN12 | Spec: profile, classes, modifiers, selection, LOD, budget | docs | - | doccheck | **done 2026-11-26** (5a66209a, `superfargoth/docs/specs/animation-style.md`: lookup moves to the framework `StyleTable::resolve`, `gait_slot` calls it; a gait class is a clip remap in the compiled graph, Classic digest unchanged, `GRAPH_FORMAT` 16 -> 17) |
| AN13 | MotionBricks v0.2.0 spike: SOMA out, Injured presets, style by key pose, output licence, VRAM | research | - | note; ledger | **done 2026-11-26** (`docs/research/anim-2026-11-26/motionbricks.md`: ends exact by construction (hubcheck 0.0 deg / 0.0 cm), stand->sit in one 1.67 s segment, slide p95 15-19 cm/s vs Kimodo 100-150; style presets = key poses; outputs free (Open Model Agreement), BONES sample poses never ship; cached TensorRT plan 1.5 s load, +384 MiB VRAM, 5-9 ms/segment) |
| AN14a | `style` module: `MotionProfile`, smoothing, `StyleTable` resolve + chooser | `sf_anim_graph` | AN12 | exact lookup, fallback, thresholds |
| AN14b | Compiled class remap + `set_style` (inertialized) | `sf_anim_graph` | AN14a | exact remap; walk-only class; Classic digest |
| AN15a | `Posture` op: hunch, slump, chest up, load, sway, breathing | `sf_anim_graph` | AN14a | exact at 0; sanity at 1 |
| AN15b | `gait_speed` / speed scale | `sf_anim_graph` | AN14a | exact |
| AN16 | Limp: hip drop, short stance, asymmetric timing, one side | `sf_anim_graph` | AN14a | exact side/phase; slide <= +0.5 cm/s |
| AN17 | Style bands in `motiongen grade` | scrib-motiongen | AG3 | exact on fixtures |
| AN18 | First style batch recipe: old, heavy, wounded, drunk, tired, timid, proud, female neutral (~14 loco clips + idles each) | scrib-motiongen | AN6, AN13, AN17 | plan prints |
| AN19 | First style batch generation + grade + bands | generation | AN18 | grade table; ledger |
| AN20 | Picks into `loco.<style>.*` slots | `mw_graphs` data | AN19, MG9 | counts per class; owner ANIM-7 |
| AN21 | Adapter: profile from race, sex, class, health, fatigue, encumbrance, drink effects; age per Q4; Classic none | `mw_walkers_bevy`, `mw_graphs` | AN14 | exact per record |
| AN22a | Glue: components, host-tick choose, weight params by tier | `sf_anim_bevy` | AN14b, AN15a, AN16 | exact tiers; counts |
| AN22b | Walkers + player carry the profile; mover reads `gait_speed` | `mw_walkers_bevy` | AN20, AN21, AN22a | counts; owner ANIM-8 (Balmora) |
| AN23 | Measurement: style layers per tier, High, 4060 laptop | tests | AN22b | `#[ignore = "measurement"]`; perf ledger |
| AN24 | Further class batches (repeatable, data only) | generation, data | AN22 | grade; owner per batch |

### Stage 4: interaction clip families (interaction and AI lanes)

Each family: recipe against slot contact points and MG4 height bands, generation, picks through Stage 2.

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| AN25 | Spec: families, slot naming, contact constraints, variant axes | docs | IN27 | **done 2026-11-26** (`motion-generation.md` "Interaction families": 10 family rows, 16 reactions, AN26 first batch 36 generated + 12 mirrored, hub table) |
| AN43 | Hub-pose rule (owner 2026-11-26: chains are assembled at runtime from pieces, never pre-generated): every interaction piece starts and ends on a listed hub pose (standing idle, seated idle, seated holding a cup...); hub list per family as data; runtime = warp + layers/masks + IK to contact points + inertialization | docs | AN25 | **done 2026-11-26** (`motion-generation.md` "Interaction pieces and hub poses": body hub + hand state, `hubs.toml` from a kept take's frame, `from_hub`/`to_hub` tags; AN25 fills the per-family table) |
| AN44a | Kimodo worker: full-body keyframe constraints at given frames (hub pose at first/last frame; README `start_pose` "not yet (MG8)"); ARDY note `docs/research/anim-2026-11-26/ardy.md` | scrib-motiongen | AN43 | selftest: constrained frames printed vs the pose | **done 2026-11-26** (`start_pose`/`end_pose = "hub:<id>"`, `pose_frames`, Kimodo `FullBodyConstraintSet` on SOMA-30 from the hub BVH frame; `recipes/hubs.toml` seeds `stand.idle`, `sit.chair`; `--hub-root`)
| AN44b | `motiongen hubcheck`: hub error per take (per-joint deg + hand/foot/pelvis cm at first/last frame, AN43's measure) and foot slide; Kimodo arm recipe for the 3 pieces (stand→sit, seated reach cup at `sit.chair`, sit→stand), 8 seeds each | scrib-motiongen | AN44a | selftest; plan prints | **done 2026-11-26** (`motiongen hubcheck`, numpy only; Kimodo arm generated + checked: `generated/hubspike/hubcheck.json`)
| AN44c | ARDY arm: install (≤ 1 day; gated Llama, C++ ext on Windows), npz→BVH, same 3 pieces with hub keyframes | scrib-motiongen | AN44b | hubcheck table |
| AN44d | MotionBricks arm: in-betweening between the hub poses, same 3 pieces (exploratory) | scrib-motiongen | AN44b | hubcheck table |
| AN44 | Spike verdict: the three arms' hubcheck + foot-slide tables side by side; owner ranks takes | docs | AN44b-d | table; owner ranks |
| AN45 | Bake hub snap + hub gate: blend the last frames into the exact hub pose, reject takes whose hub error exceeds a tunable (beside the foot-slide gate) | scrib-motiongen, `mw_graphs` bake | AN44 | exact: snapped ends equal the hub; rejects printed |
| MG7b | Grip table + second-hand `FullBodyIk` | `mw_viewers_bevy`, data | MG7a | exact sockets |
| MG8 | MotionBricks link worker (approach, loop seams, exit) | scrib-motiongen | AN13 | **done 2026-11-26** (seams printed: `motiongen link`, `[[link]]`/`[[loop_seam]]` recipes, one MotionBricks venv with a numpy Kimodo port; hubcheck on 4 takes: ends 0.0 deg/cm, slide p95 13-19 cm/s, raw RightHand miss ~50 deg spread by the correction, 4 ms/segment; ledger animation 2026-11-26) |
| MG8b | Link worker: wrists and fingers slerped hub to hub over the piece (the model's raw wrists miss the hubs by 29-53 deg, MG8 diagnosis), the body keeps the endpoint correction; hand contacts stay runtime IK | scrib-motiongen | MG8 | hubcheck: ends 0/0, no wrist correction spread; per-frame wrist twist printed **done 2026-11-26** (`--hand-mode slerp` default, `model` = A/B; ends 0/0, wrist peak 2-67 deg/s vs MG8 113-217; left forearm correction 16-17 deg remains, ledger) |
| AN26 | Sit/stand (IN2): approach-turn-in, sit, idle loops, stand; stool/chair/bench/floor; sides; bands; old/heavy variant | generation | AN25, MG8 | grade; owner ANIM-9; **AN26a generated 2026-11-26** (sit1: 36 pieces / 136 takes, hubs sit.stool, sit.chair.seat54, sit.bench, sit.floor, sit.chair@old, stand.idle@old; hubcheck ends 0/0); **AN26b baked 2026-11-26** (pelvis_fit + cook mirror + 48 `seat.*.b<cm>` slots, provisional picks); AN26c: c1a slot aliases in the step (8179b016) + c1b seat binder (ecab5d12) done 2026-11-26, c2 player loads the `seat.*` clips + avatar plays the aliased clip + `SeatClipSets` inserted (508a628a) done 2026-11-26: the player sits in game, owner ANIM-11 |
| AN27 | Chair pull-out/push-in, bench scoot, seated prop chain: reach, pick, drink/eat/read, put down (IN9, AI31) | generation | AN26, MG7b | grade; owner ANIM-9 |
| AN28 | Doors (IN33): push/pull, hinge L/R, both sides, walk-through, knock, locked rattle, bash (IN37) | generation | AN25 | grade; owner ANIM-10 |
| AN29 | Carry and place (IN35, AI31): pick up at 3 heights, carry 1H/2H/shoulder, place, give/take, throw | generation | MG7b | grade; owner ANIM-10 |
| AN30 | Beds and lean spots (IN3) | generation | AN25 | grade; owner ANIM-9 |
| AN31 | Social and fear reactions: turn-to-face, placate, cower, flee start, surrender (LW14), talk gestures (DQ-30) | generation | AN25 | grade; owner ANIM-11 |

### Stage 5: combat on Morrowind clips (the combat lane owns the sheets)

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| AN32 | Measurement: weapon groups' text keys, durations, root travel; `Hit1-5`, `KnockDown`, `Shield`; missing moves | tests `mw_bake` | - | printed; ledger | **done 2026-11-26** (a humanoid sets, b creature `Attack1-3`: `mw_bake/tests/attack_clips_survey.rs`; ledger animation: hit at 0.47-0.80 s, one of three follow windows by strength, no attack lunges, no held guard; creatures: 6 of 234 moves travel >= 1.0 m to the hit, none >= 1.5 m)
| AN33 | Text keys -> sheet windows at cook (anticipation, active, recovery) | `mw_combat` | AN32 | exact vs keys |
| AN34 | Per-phase rate + additive anticipation/overshoot op | `sf_anim_graph`, `mw_combat` | AN33 | exact; owner ANIM-12 (level-sparring) |
| C4 | Attack clip pass: sword sheet on Morrowind chop/slash/thrust by direction, step-in `Warp`, `ik.feet` (B5) | `mw_combat`, `mw_player_bevy` | AN34, B5, Q2 | owner CB-1 re-check |
| AN35 | Reaction map: `Hit1-5`/`KnockDown` to tiers, `HitReact` additive for light hits | `mw_combat` data | AN32 | exact rows |
| AN36 | `PoseSeq` asset: key poses, eases, root arc cooked to a clip | `sf_anim` | - | exact at keys |
| AN37 | Missing moves as `PoseSeq` from Morrowind poses: backstep, 2H guard, parry, riposte, kick (Q1) | `mw_combat` data | AN36 | owner ANIM-13 |
| C2 | Locked-on strafe on the new gait set + `OrientationWarp` (C1c-2b) | `mw_graphs` | AN6, A5 | exact sync; CB-2 re-check |

### Stage 6: physics tint on Delirium

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| AN38 | Research: Delirium partial drive (per-region strength, disturbance), cost per body | research, test | - | measurement; ledger | research **done 2026-11-26** (d96d9ec4); measurement: **a done** (A/B/F at N = 1, 4, 8: 0.50 / 0.67 / 0.86 ms per body, linear; pose follow alone collapses; ledger physics), **b done** (C, D, E, G + first 2 s + N = 2: pinned-pelvis pose follow G stands at 0.65 ms per body, 1.28 median / 2.1 p99 ms per tick at N = 2, fails the budget at N = 4: cap 2 confirmed; C 0.63, stumble 0.52, a fallen body 1.1-1.2), c (pose error), d (`step_group`, start/drop) left |
| AN39 | Spec: `PhysicsTint` seam, states, cap, gate, fallback | docs | AN38 | doccheck | **done 2026-11-26** (`superfargoth/docs/specs/physics-tint.md`, fed5bd81; the concurrent cap stays a Guess until AN38's numbers) |
| AN40a | Seam: `sf_anim::tint` types, envelope, `TintBook` (gate, cap, states), `sf_anim_bevy` plugin, `TintTarget`, fallback messages; exposes the pose before the physics blend (`GraphOutput::pose` already includes it, `sf_anim_graph/src/instance.rs:239-241`); checks hurt volumes do not follow the tinted pose | `sf_anim`, `sf_anim_bevy` | AN39 | exact cap, states (fake provider; `TintEnd` has 7 variants, no `Interrupted`: only a takeover ends a tint early; the same kind again refreshes it at the higher of the two intensities, one exact case): the cap test keeps 4 as its fixture value (6 requests -> 4 Active, 2 Refused); the default read from `TintSettings` is 2 (Low 1) |
| AN40b | Delirium tint provider (pinned-pelvis pose follow); step 1 = balancer coexistence test | `mw_delirium`, `mw_delirium_bevy` | AN38, AN40a | no NaN, not through floor, comes to rest; pelvis still at standing height 2 s into an undisturbed tint (the AN38a collapse); `Failed` comes from the provider's own check (NaN, part below the floor, pelvis off target), not `agent.fallen` |
| AN41a | Hit triggers: Medium and Large tiers + impulse band -> Stagger (Large, owner 2026-11-26: the profile's UPPER regions, priority `Hit`, intensity = tunable `mw.tint.large.intensity`, 0-1, default 1.0, Guess; 0 sends no request = the Large clip alone, the A/B arm; the Large clip stays the move; refused -> the clip plus the `HitReact` kick; exact: one `TintRequested` per Large reaction, none at intensity 0. Medium: physics hits of 10 to `band.max_ns` 25 N s, Guess, become a stagger tint instead of today's owner-confirmed push takeover PH-14/15; `band.max_ns` = 10 keeps today's behaviour and is the A/B arm) | `mw_combat`, `mw_walkers_bevy` | AN40b, AN22 | exact |
| AN41b | State triggers: injury, alcohol, fatigue -> Stumble, Sway, Sag; scene `anim-tint` | `mw_walkers_bevy` | AN41a | exact; owner ANIM-14 |
| AN41c | Sway/Sag fallback key-pose clips (data) | `mw_combat` data | AN41b, AN36 | bake loads; ANIM-14 |
| AN42 | Measurement: tint ms at the cap (cap 2, each kind, and mixed with 1 and 2 takeovers) | tests | AN41b | `#[ignore = "measurement"]` |

**Later / parked:** motion matching; quadruped legs IN26 (TV-R2 parked); first-person body (3.4b); player limp (Q3).

## 5. Spec changes (as spec slices, before build)

- AN12: new `animation-style.md` (pointer in `animation.md` "Motion style"); `animation-sources.md` "Gait classes" (fill, `loco.<style>.*`, Classic).
- AN9: `motion-generation.md` "Review renderer" rewrite; AN25 its "Interaction families"; AN39 `animation.md` "Physics tint".
- Before AN33: `combat-adapter.md` "Morrowind attack clips" (windows, per-phase rate, reaction map).
- Cleanup: drop 100STYLE from `animation-sources.md` and ROADMAP 3.4c ("look for a free sprint clip (100STYLE)"). **Done 2026-11-26** (070b6f75).

## 6. Dependencies on other lanes

| Lane | Needs | Provides |
|---|---|---|
| Interaction | IN27 slot frames, IK targets | AN26-AN30, MG7b, MG8; modifiers on slot clips |
| AI | AI14/IN8, AI31, AI37, perception profile | AN8 look-at, AN22 styled walkers, AN31 |
| Combat | sheets, poise (C3), C6 | AN32-AN37, C2, C4, AN41 medium- and large-hit tint |
| Crime, dialogue | LW14, DQ-30 | AN31 surrender/search, talk gestures; AN8. **Needed by DQ-30b** (`superfargoth/docs/specs/dialogue-presentation.md`; rows get IDs and an order when the dialogue lane reaches DQ-30): a per-character `look_target` writer, an additive head/neck rotation input for nods, a one-shot upper-body gesture slot |
| Travel | IN7/IN23 seats | sit/astride clips (AN26 family) |

## 7. Owner questions (each with a recommended answer)

1. **Moves Morrowind has no clip for** (backstep, two-hand guard, parry, riposte, kick): key poses from Morrowind's own
   poses, or Kimodo? Recommended: key poses (Morrowind's look, exact timing); Kimodo only where one reads stiff.
2. **Swinging while moving:** Elden Ring full-body commit (legs stop, short step-in) or Morrowind's swing over walking?
   Recommended: commit in Modern, over-walk swing in Classic.
3. **Style states on the player:** Recommended: fatigue and heavy load yes (slower, slumped), drunk sway yes; a
   low-health limp only as an option, off by default; Classic none.
4. **Who looks old** (Morrowind has no age)? Recommended: named NPCs from an override list filled from head models and
   lore, the rest by a stable per-NPC roll weighted by race and class; mods override.
   **Answered (owner 2026-11-25):** age is a per-NPC value set in code/data; in Morrowind nobody is "very old" except
   chosen obviously old NPCs (a matriarch...) by override; how age shows visually comes later, low priority.

Owner answers 2026-11-25 (DECISIONS): Q1 Morrowind's poses first, Kimodo takes too; Q2 commit with a short step-in;
Q3 as recommended. The Quaternius roll clip is fine for now (maybe Kimodo later).

Also answered 2026-11-25 (DECISIONS): launch gait classes neutral, old, heavy, drunk, wounded/limp (+ fatigue/load,
more in batches); wounded NPCs and companions limp by health band and recover.
