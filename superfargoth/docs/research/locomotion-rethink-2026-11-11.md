# Locomotion rethink: starts, stops, turns, slopes (LOCO-R1, 2026-11-11)

Owner 2026-11-11 (player in `level-flat-slopes`): feet are on the ground, but on a stop a leg (often the right)
"pops back into place brutally", the blend looks odd, and sometimes the whole body rises from the hips "as if
standing back up". Wanted: a stop in one or two natural steps (animation matching, or the Sekiro way). On
slopes the arms stay straight and point backwards on steep ones ("ninja run"), the head is too tilted ("people
look where they go"), and going down a steep slope the steps are "outrageously big". Target: Elden Ring-like
feel. Research and design only; no code changed. Sources marked [S] fetched this session, [K] known but not
re-fetched, [I] our inference.

## 1. What causes each symptom today

Today's humanoid locomotion: idle/walk/run (+ strafe, turn-in-place, sneak) **cycles only** in a state machine
(`mw_graphs/src/build.rs:368-408`), every gait switch an inertialization of `GAIT_BLEND = 0.15 s`
(`build.rs:349-358`), then the op stack `SpeedMatch` -> `StrideWarp` -> `FootPlacement` -> `Lean` -> `HandIk`
(`build.rs:922-978`). The player brakes with a velocity half-life of 0.05 s (`mw_player_bevy/src/feel.rs:20`):
stop distance about 0.12 m from walk (1.6 m/s), 0.32 m from run (4.5), 0.5 m from sprint, all within ~0.2 s.
No start, stop or pivot clip exists, so a stop is "cycle pose -> idle pose in 0.15 s".

**1a. Leg pop on stop (cause found in code, not yet measured).** The stop fires when measured speed drops under
`stop_speed` (`sf_controller_bevy/src/lib.rs:554-575`); `WALK -> IDLE` inertializes. Curves are not
inertialized: the idle clip's `contact.<foot>` curves (baked by speed, `sf_anim/src/cook.rs:1245-1372`) read 1 for
both still feet from the switch frame. In `FootPlacement` (`sf_anim_graph/src/procedural.rs:699-752`) the swing
foot, contact 0 -> 1, **plants at once wherever it is** (mid-stride), and the stance foot stays locked where the
walk left it. Over the next 0.15 s the inertializer carries the animated feet to the idle stance (feet
together); the locks don't follow. A walk step puts each foot ~0.3-0.4 m from the pelvis, the unplant limit is
`0.3 x reach` ~ 0.26 m (`asset.rs:511-537`), so the foot with the larger offset trips `unplanting`
(`procedural.rs:718`) and its offset decays in a critical spring of half-life `release / 2 = 0.05 s`
(`procedural.rs:755-765`, `release` default 0.1): the foot **slides ~0.3 m along the ground in ~0.15 s**, a pop.
The other foot, under the limit, stays locked off its idle spot until a later drift releases it the same way.
Which foot pops depends on the walk phase when the brake ends; "often the right" may come from the idle clip's
stance asymmetry or the brake timing (unverified: measure, slice S1). The "weird merging" is the inertializer
itself: it is built for nearby poses, and between a mid-stride pose and a feet-together idle it morphs the legs
through each other with the swing's carried velocity (Holden, *Spring-It-On* [K]: inertialization hides
discontinuities, it doesn't author a step). The 0.15 s floor was set because shorter "pops at the feet"
(`build.rs:350-351`): the same root cause.

**1b. Hips rise on stop (two candidates in code, unmeasured).** (1) `FootPlacement` `BodyFit::Lower` drops the
pelvis by the largest leg need (`procedural.rs:808-812`, spring `pelvis_halflife` 0.08 s, `asset.rs:926`): with
feet held at stride spacing under an idle pelvis (higher than the walk's), the legs can't reach, the pelvis drops;
when the lock releases (1a), the need vanishes and the pelvis springs up in ~0.2 s: the "standing back up". (2) A
stop while the facing is still turning: `IDLE` needs `|turn| < 0.5 rad/s` (`build.rs:372-374`); above it the graph
enters `TURN_LEFT/RIGHT` (Morrowind `TurnLeft/TurnRight`, `build.rs:728-731`) for a moment, two more 0.15 s
inertializations and the turn clip's own pelvis motion. `StrideWarp`'s drop is not the cause on flat ground (its
held need decays with half-life 1 s, `asset.rs:889`, but at the player's walk ratio 0.73 no drop is needed).

**1c. Arms point backwards on slopes (cause certain).** `Lean` pitches chest 0.45 + upperChest 0.4 + neck 0.15 of
the grade pitch (`build.rs:961`; `procedural.rs:522`), +17° at 30° up, the cap 25.8° from ~35°
(`ik-feel-diagnosis-2026-11-11.md` §7). The arms hang off the upper chest and keep their chest-relative pose, so
they stay parallel to the trunk: a trunk leaning forward by 20° carries the hanging arms 20° back from vertical,
the "ninja run". Real arms hang and swing about the shoulder in the gravity frame (pendulum) [K]; nothing in our
stack counter-rotates them. The walk/run cycles' small arm swing ("arms stay straight") is the clip.

**1d. Head too tilted (cause measured, §7 of the IK diagnosis).** The neck share makes the head take the whole
cumulative lean (+23° at 30°+, more than the trunk's 17°) and nothing levels it. Humans keep the head near level
and the gaze 3-5 m ahead on slopes (vestibulo-collic reflex) [K]. There is no look-at in the humanoid graph
(the `LookAt` op exists, `procedural.rs` doc line 24, unused in `mw_graphs`).

**1e. Huge steps downhill (cause certain, three multiplying terms).**
1. `StrideWarp` grade term **lengthens** downhill steps: `scale = 1 - 0.25 x grade` (grade < 0), capped 1.2
   (`asset.rs:554-559`, `build.rs:944`): 1.21 at 40°. Biomechanics says the opposite: downhill steps are ~8%
   *shorter* with ~6% *higher* cadence (ISBS 2019 [S], core.ac.uk/download/235628692.pdf; PeerJ 2024 [S]).
2. `SpeedMatch` divides the rate by the grade scale (`instance.rs:1528`): longer steps at a *slower* cadence.
3. Geometry: the mover keeps horizontal speed on any grade (no slope term in `sf_action`) and the stride warp
   scales steps in the horizontal plane, so a flat-authored step of horizontal length L spans L / cos θ along the
   surface (x1.31 at 40°). With (1) the step is ~1.6x the flat step along the ground, and `FootPlacement`'s pelvis
   drop (−20 cm at 40° down, knees out 8.6 cm, §7 table) lets the legs reach it: a lunge. UE's Stride Warping
   warps in the floor plane (floor-normal and gravity inputs) [K].

## 2. Candidate architectures

What industry ships [S/K]: **Lyra** (UE5): capsule-driven, a locomotion machine idle/start/cycle/stop/pivot,
starts advanced by distance (`Advance Time by Distance Matching`), stops and pivots matched to a predicted stop
point (`Distance Match to Target`, distance from v²/2a of the movement component), Distance curves baked by a
modifier on root-motion clips, stride + orientation warping on the cycles, Foot Placement last (plant by foot
speed curve, unplant radius and angle, pelvis max offset). **ALS v4/Refactored**: cycles + transition anims +
per-clip FootLock curves (the lock follows the curve, released as it decays: no pop), additive lean, aim offset
for the head; stops not distance matched and known to slide. **GASP** (UE 5.4+): motion matching over 500+
mocap animations (loops, starts, stops, pivots per gait and direction) with a Chooser picking the database,
trajectory from the movement predictor, Foot Placement Control Rig on slopes, additive Look-At (5.8). **Motion
matching**: Clavet 2016 (trajectory +0.33/0.66/1 s plus feet), TLOU2 2021, learned MM (Holden 2020: GB -> MB).
**FromSoftware** (from sekiro-rs, below): Havok Behavior + script choose authored start/loop/stop/turn clips,
root motion drives the body, TAE windows set turn speed and cancels, blends are timed crossfades, foot IK is
visual only (pelvis drop by the lower foot, never raised, asymmetric ascend/descend gains), no foot lock.

**sekiro-rs** (AKJama, MIT OR Apache-2.0, Bevy 0.20, ~260 commits Oct-Nov 2026) [S]: a runtime for FromSoft's
own data (HKS VM, HKB interpreter, TAE runtime, HKX root motion read from the user's install); the start/stop/
turn logic lives in Sekiro's `c0000.hks`/`.behbnd`, not in Rust, and it ships no clips. Reusable: the *ideas* in
`crates/game/src/foot_ik.rs` (asymmetric pelvis gains 0.2 up / 1.0 down, dt-correct `1-(1-g)^(dt*60)`), the
yaw-twist unwind (body yaw snaps to the stick, the visual twist unwinds at 720°/s) and the locked-on leg twist
with spine counter-twist, TAE-style windows. Not reusable: everything under `formats`/`hks`/`behavior` (FromSoft
data). `FFwLo/RustSekiro` (MIT) is a combat prototype with traced engine values, no clips. Neither gives us
locomotion content: the FromSoft feel comes from its authored clip set.

**Clip supply under our rules.** UAL1/UAL2 (CC0, bundled): `Walk_Loop`, `Jog_Fwd_Loop`, `Sprint_Loop`,
**`Sprint_Enter`, `Sprint_Exit`**, **`Turn90_L/R`**, **`Turn180_L/R`**, `Jog_Fwd_LeanL/R`, 8-way walk/jog loops,
`StepUp`. No walk or jog start, stop or pivot; Morrowind has none either. Kimodo (code Apache-2.0, SOMA weights
NVIDIA Open Model License, commercial use allowed [S]; shipping its clips already accepted by the owner, ROADMAP
3.8, pipeline MG1-MG6 built, 268 clips generated) is strongest at locomotion and takes 2D root-path and
end-effector constraints, 30 fps, ≤ 10 s, with some foot skate. So starts/stops/pivots are **generated**, with
our contact bake and foot cleanup, 10-40 clips: a Lyra-sized set. A GASP-sized MM database (hundreds of
clips, many speeds and angles, consistent style) is a much larger generation and review job; nobody has
published an MM database built from text-to-motion [S: none found]. LaFAN1 stays optional.

| | (a) cycles + fixed ops | (b) Lyra: distance-matched start/stop/pivot + warping | (c) FromSoft: authored transitions, root motion drives | (d) Motion matching (GASP) | (e) **hybrid: (b) + posture layers + MM-ready data** |
|---|---|---|---|---|---|
| Stop feel | best case a quick settle; feet still slide or step-pop: the stance change has no authored step | a real stop step per planted foot, timed to the brake | authored stop steps; movement follows the clip | best (picks the stop frame matching pose + trajectory) | as (b) |
| Turns | TIP clips + orientation warp | pivot clips (180° run), TIP | turn-and-go clips by angle | best | as (b); UAL Turn90/180 for TIP |
| Slopes, head, arms | needs posture fixes anyway | same fixes needed | not handled (foot IK only) | same fixes needed (GASP: Foot Placement + look-at) | posture layers built once, serve all |
| Clips | none new | ~10-40 generated (Kimodo) + UAL Sprint_Enter/Exit, Turn90/180 | the same set, but each tuned as gameplay (its root motion *is* the move) | ~200-500 consistent clips; Kimodo at that scale unproven | the (b) set, cooked with MM features so (d) can be added |
| Cost per character | lowest | + a curve lookup per distance-matched player (µs) | low | a search per 0.1-0.2 s: est. 20-100 µs brute force on ~20k frames, less with AABB culling [I]; LOD needed for crowds | as (b) |
| Net-ready | yes (capsule) | yes: capsule authority, clip matched to the predicted stop | no by default: animation drives gameplay, host must run the same clip time; fights fixed-tick springs and ORCA | yes (capsule) | yes |
| NPCs | same as player | same graph; NPC stops come from the walker brake | clip-driven NPCs vs ORCA avoidance: conflict | same, LOD'd | same graph |
| Effort | small | medium: cook curve, a seek mode, stop predictor, states, clip generation | large: a second movement model | large: features, search, data | medium, then (d) as one node later |

## 3. Recommendation: (e), Lyra-style transitions plus posture layers, capsule-driven

Keep the capsule as the only gameplay authority (locomotion.md, net-ready rule) and add what our stack lacks, in
the industry's order:
1. **Transitions from clips, not from the inertializer.** States `start`, `stop`, `pivot` and turn-in-place
   around the cycles. A stop chooses the clip for the **planted foot** (`stop_l`/`stop_r` by the cycle's contact
   curves at the request) and seeks it by **distance to the predicted stop point**; the stop point comes in closed
   form from the mover's own braking spring (the trajectory already predicts +0.2-1.0 s,
   `locomotion.md` "Canonical technique"); a start advances by distance travelled; a 180° run pivot matches the
   reversal point. This is exactly animation.md open question 5 / audit G7. The clip carries the stop step in the
   air, so no lock is dragged and the hips never wait on a released need. Inertialization stays, but only
   between close poses (cycle -> stop clip at a matching phase), so 0.15-0.2 s no longer has to hide a stance
   change. Elden Ring's stops are short: our 0.05 s brake stays, and the stop clips are authored short
   ("plant and gather": one gather step, ≤ 0.3 m at walk, ≤ ~0.6 m at run), owner question 1.
2. **The foot lock follows the clip.** Contact curves stay authoritative (ALS's FootLock curves); a state switch
   must not plant a swinging foot on the switch frame: when `contact` rises because of a switch rather than
   inside one clip, plant only once the foot's model-space speed is below the plant threshold (UE plant speed).
   This is the fallback for characters or stances without stop clips (creatures, Morrowind sets): with a stop
   chosen at a foot-down phase, the trailing foot then swings into the idle pose instead of being pinned.
3. **Posture on slopes** as separate layers, not more `Lean`: a **gaze layer** (the existing `LookAt` op on
   neck/head, target a point on the predicted path ~3 m ahead at eye height above the ground there; lock-on
   target when locked; Lyra/ALS aim offset, GASP 5.8 additive look-at), which replaces P-IK4's negative head
   share and also turns the head into turns before the body; **arm hang**: the upper arms counter-rotate by most
   of the trunk's slope pitch (a gravity-relative hang, the standard Control Rig counter-rotation or an additive
   slope pose); and `Lean` without the neck share, total trunk lean kept to about half the slope angle (biomechanics
   and P-IK7's cap).
4. **Downhill stride:** grade shortens steps both ways (downhill ~5-10% at 10-20°, cadence up), and the warp
   measures step length in the floor plane (x cos θ), so the along-ground step stays the clip's. The pelvis drop
   gets a max offset by speed and an asymmetric spring (fast down, slow up: sekiro-rs/FromSoft gains, UE pelvis
   interpolation).
5. **Motion matching later**, as planned in animation.md "Motion matching: later": every clip of (1) is cooked
   with trajectory/foot features, so when the generated set is big enough (the dance card filled), an MM node can
   replace the start/stop/pivot choice inside the same states (GASP's Chooser + Pose Search). Not before: with
   ~20 clips the state machine is equivalent and cheaper, and the data is the hard part.

Why not (c): it would move gameplay onto clip root motion, break fixed-tick springs, ORCA avoidance and the
host-authoritative plan, and still needs the same clips (the FromSoft feel is its content, which we can't use).
Why not (d) now: no shippable database exists under our rules; generating and reviewing hundreds of consistent
clips is a larger job than the whole of (e), which produces its seed data.

**Existing pieces.** `FootPlacement` stays (foot height, tilt, lock, pelvis); it gains the switch-aware plant
(2) and the pelvis cap and asymmetric spring (4). `StrideWarp` stays on cycles (Lyra) with the fixed grade
sign and floor plane; off during distance-matched clips (their timing is the match). `SpeedMatch` stays on
cycles. `Lean` stays for the trunk only. Inertialization stays for every switch. The Morrowind turn states stay
for Morrowind sets; humanoids get UAL `Turn90/180`.

**P-IK5 (turn unplant, in flight in `.sessions/ik-p5.md`): finish it.** Small, nearly built (`unplant_angle` 45°,
`asset.rs:514-537`), the UE Foot Placement standard, and still needed with pivot clips: 720-1080°/s facing turns,
strafing and NPC turns have no clip. **P-IK6 (speed-scaled pelvis): continue, rescoped** to "pelvis max offset by
speed + asymmetric up/down spring" (one slice; verify with the §7 run/stairs pelvis column and a stop
measurement); a pure speed scale alone would *raise* the drop at a stop. **P-IK4 is superseded** by the gaze
layer (slice L6). **P-IK7** folds into L7. **P-IK3** (slope pelvis bob) still stands, measured after L7.

## 4. Slice plan (one deliverable + its test each)

Research and build kept apart; each build slice "hands off at ~80-100k". Order follows the owner's symptoms.

- **L0 docs (no code):** animation.md "Locomotion transitions" (states, distance curve, seek modes, stop-foot
  choice, switch-aware plant, gaze and arm layers; reuse check and industry comparison from this doc);
  locomotion.md stop/pivot point prediction; industry-audit G7 row -> planned. ROADMAP line.
- **L1 measure the stop (test only):** `#[ignore = "measurement: ..."]` in `mw_graphs/tests/slope_walk.rs`: the
  player's mover stops from walk/run at 8 phases on flat ground; print per foot lock drift, `unplanted`
  count, release slide speed, pelvis offset peak and rise, and which state ran (TURN_* hit or not). Pins 1a/1b and
  "often the right". Sanity: no NaN.
- **L2 switch-aware plant (build):** `FootPlacement` plants after a state switch only below the plant speed; the
  stop is requested at the next foot-down marker when no stop clip exists (cycle-only fallback). Test: L1's
  table with `unplanted` on stops -> 0 and slide ≤ a few cm/s (printed); exact unit tests in `procedural.rs` for
  "contact jump on switch does not lock a moving foot". Owner check LOCO-1.
- **L3 distance curve at cook (build, `sf_anim`):** a `distance` curve on root-motion clips tagged start/stop/
  pivot (along-path root distance, negative before the stop, zero at rest). Exact test on a synthetic
  decelerating clip.
- **L4 seek by distance + stop predictor (build, `sf_anim_graph`, `sf_controller`):** a player mode "advance by
  distance" / "match remaining distance", and the closed-form stop distance of the mover's braking spring in the
  feed. Exact tests: predicted stop vs a simulated mover within 1 cm; seek picks the frame whose remaining
  distance equals the input.
- **L5a clip dance card (research):** the list (walk/jog/sprint start 0/±90/180, stop L/R foot, run pivot 180
  L/R; UAL Sprint_Enter/Exit and Turn90/180 slotted), Kimodo prompts and root-path constraints per clip, the
  short "plant and gather" stop lengths, acceptance rules (contact curves clean, foot skate under the cook's
  limit, distance curve monotonic).
- **L5b generate + bake (data):** Kimodo pass (3.8 pipeline), retarget onto `base_anim`, slots in the animation
  sources manifest; `bevyrrowind bake`. Owner reviews clips in `clip-preview --browse` (LOCO-2).
- **L5c states (build, `mw_graphs`):** start/stop/pivot states using L4, stop clip by planted foot, fallbacks to
  L2 when a slot is missing. Test: headless stop/start/pivot from each gait, sanity (no NaN, feet above ground,
  lock drift printed), exact: the chosen stop clip matches the planted foot. Owner LOCO-3.
- **L6 gaze layer (build):** `LookAt` in the humanoid graph, path target 3 m ahead at eye height, lock-on
  target, head/neck weights, ±60° yaw. Test: §7 head-pitch column within ~8° of flat on every ramp (printed), no
  NaN; supersedes P-IK4. Owner LOCO-4.
- **L7 slope posture (build):** arm counter-rotation share in `Lean`, neck share removed, trunk lean ~half the
  slope with P-IK7's cap. Test: printed world pitch of the shoulder-to-hand vector on 0/20/30/40° within a few
  degrees of flat; trunk lean column. Owner LOCO-5.
- **L8 downhill stride (build):** grade shortens both ways, floor-plane step, cadence up. Test: printed
  along-ground step length at −10/−20/−30/−40° within ~±10% of flat, planted slide not worse. Owner LOCO-6.
- **P-IK5** finish (in flight); **P-IK6** rescoped as in §3.

Visual checks to register (`bevyrrowind/docs/visual-checks/player-ui.md` + index, awaiting owner), all as the
player, third person, F11 off, `level-flat-slopes` unless named:
- LOCO-1 (after L2): walk and run, release the stick at random moments on flat ground; no foot slides or pops,
  the hips don't rise after the stop; K toggles foot placement.
- LOCO-2 (after L5b): the generated starts/stops/pivots in `clip-preview --browse`: style matches the cycles,
  no skate, stop lengths read natural.
- LOCO-3 (after L5c): stops from walk/run end in one gather step; starts and 180° run reversals; turns in place.
- LOCO-4 (after L6): camera `ramps`, walk up and down 20-40°: the head stays near level and looks ahead; locked
  on, the head tracks the target.
- LOCO-5 (after L7): same ramps: arms hang and swing vertically, no "ninja run".
- LOCO-6 (after L8): walk and run down 30° and 40°: short, quick steps, no lunge.

## 5. Owner questions (taste or what the player can do)

1. **Stop length.** Keep today's near-instant capsule stop (0.12 m from walk, 0.3 m from run: Elden Ring-snappy,
   the stop clip a short gather step), or let the player coast into a natural one-to-two-step stop (~0.5 m walk,
   ~1.5 m run, Lyra-like, less responsive)? **Recommended:** snappy at walk, a slightly longer run/sprint stop
   (brake half-life 0.05 -> ~0.1 s at run/sprint only, ~0.6 m), which gives the run stop room for one real step.
2. **Speed on steep slopes.** Should the player slow down on steep grades (gameplay change), or keep full speed
   as now? **Recommended:** keep full speed up to ~25°, then up to ~20% slower uphill and ~10% downhill by 40°
   (most action games keep speed on walkable slopes; the slowdown sells steepness and shortens the downhill
   strides naturally).
3. **Where the head looks.** Path ahead (and the lock-on target) only, or also toward where the camera points
   when standing or walking (ALS/RDR2 "alive" look)? **Recommended:** path ahead + lock-on target now; camera
   look only when idle, later, as an option.

## 6. L1 measured (2026-11-11, `player_stop` in `mw_graphs/tests/slope_walk.rs`)

Player stops on flat ground, walk and run, 8 phases each. **1a confirmed:** peak lock drift 28-36 cm vs the ~26 cm
unplant limit, unplants in 14 of 16 stops, release slides 3-10 m/s over 40-100 cm: the swinging foot plants off its
idle spot and springs back. **1b partly:** the IK pelvis drops up to 12 cm and springs back up to 9 cm, but the
animated pose alone (no IK) also rises 2-9.5 cm: much of the hip rise is the cycle-to-idle pose change itself, which
stop clips (L5) fix and L2 does not. **TURN state on a straight stop: ruled out** (stop while turning not tested).
**"Often the right":** not a phase effect here (the popping foot is the swinging one, L and R even). 7 of 16 rows still
move at +2 s (cause not found). Run: `bash bevyrrowind/tools/cargo_turn.sh test --release -p mw_graphs --test
slope_walk player_stop -- --ignored --nocapture`.
