# Animation plan: slices (anim-refactor lane, 2026-10-22)

Work order and status log only. The design (R11b "Decided shape", layers 1-4) lives in [animation-layers.md](animation-layers.md); the Next-gen roadmap shape there was replaced by R11b (2026-10-19), so read this file for slice order, acceptance and owner-pass issues (AN-*), not for design.

Execution order for ROADMAP Phase 3 items 3.1, 3.2 and 3.4, ordered by "characters look best soonest".
Design lives in [animation-layers.md](animation-layers.md) ("Decided shape (R11b)", layers 1-4), [animation.md](animation.md), [action.md](action.md)
(phases A-F), [locomotion.md](locomotion.md) and `../research/animation/12-delirium-physics.md` (R0-R9).
This file only orders the work and states the acceptance tests. Each slice is one fresh agent: 1-2
steps, about 60 tool calls, headless tests, a `level-anim-*` scene (test level `bevyrrowind/levels/anim-lab.ron`)
and a visual-checks entry for the owner ("awaiting owner"). Mark slices `[x]` when their tests passed, and
`[~]` while they're started.

Rules for every slice: research first; reuse Bevy, then crates, then our own; logic in Bevy-free crates;
no GPL-derived code in superfargoth; every cargo command through `bevyrrowind/tools/cargo_turn.sh ... -j 4`;
tests and pedantic clippy clean for the crates touched; out-of-scope bugs are logged, not chased.

| # | Slice | Items | Acceptance |
|---|---|---|---|
| 1 | **Test level** `anim-lab`: walk loop, ramps, stairs, ledges + wall, props, drops, idles/reactions dummies, named cameras, `level-anim-*` scenes | lab | Level loads in the level parse test; scenes open; visual checks `AL-*` |
| 2 | **R0 ragdoll write-back**: the skinned mesh follows the capsules (`sf_ragdoll` `local_pose`, `sf_physics_bevy` `read_back`, `sf_anim_bevy` `write_bones`) | 3.2, PH-2 | Headless test: cooked humanoid ragdoll dropped on a slope with the entity frame moved, yawed, race-scaled and cell-rebased mid-fall; every body's world pose matches its bone's world transform after write-back (≤ 1 cm, ≤ 2°). Failing first. PH-2 re-entered on `level-anim-drop` |
| 3 | **Idle and crowd variety**: per-character phase offset and rate jitter on idles/locomotion, idle fidget selection without repeats (animation.md open question 8) | CH-1, CH-3 | Unit tests: deterministic per seed, no two neighbours in phase, fidget never repeats twice in a row; `level-anim-idles` |
| 4 | **Foot-slide and penetration metric** (Bevy-free, from contact curves + ground probes, as animation.md "Verification" promises), then fix the clip contact bugs it finds (strafe contacts, "one leg constant contact" on flat ground) | SB-5, WK-3 | Metric unit-tested on synthetic clips; headless run on the lab loop reports slide (cm/step) per gait; contact fix lowers it (numbers in locomotion.md) |
| 5 | **Feet on stairs and slopes**: per-girdle FootPlacement (animation.md audit row 2), pelvis drop from the lower foot, step-aware probes | WK-4, SB-5 | Headless: feet within 2 cm of the step/slope surface at contact frames on the lab stairs and 10/20/30° ramps; `level-anim-stairs` |
| 6 | **R2 reaction controller core** (`sf_ragdoll::react`, Bevy-free): hit → strength/region-weight timeline, Tracking → Stagger → Falling → Down → GettingUp | 3.2 | Unit tests per transition, deterministic per seed; data in RON |
| 7 | **R3 powered falls on authored clips**: `KnockDown`/`Hit*` clips as drive targets at reduced strength, fall collides with stairs/walls; get-up from pose (`ROLL_TO_BACK` scaffold removed) | PH-9 | Headless: knockdown on the lab slope tracks the clip (pelvis error gate) and settles; `level-anim-drop` |
| 8 | **A2 fixed tick + interpolation** (`sf_action_bevy`): commands on the fixed tick, transform interpolation, render-body correction spring | 3.1 | action.md A2 acceptance: walk/jump at 30/60/144 fps, no jitter |
| 9 | **B1 Progress root mode + chain warp** (`sf_anim_graph`) | 3.1 | action.md B1: 2-clip chain ends within 2 cm / 2°; refusal above ratio |
| 10 | **B2 mantle/vault/step-up + minimal `clip.annotate`** | 3.1 | action.md B2: lab ledges 0.5-1.6 m, feet within 2 cm; `level-anim-ledges` |
| 11 | **Doc diet**: animation.md (108 KB), locomotion.md, character.md split into design + archive (ROADMAP 0.4) | 0.4 | Each spec ≤ ~25 KB, history in `archive/` |

Not here: smooth-body seams SB-1/2/3 (`sf_character` smooth, a character slice of its own), AP-2 particle
curves, HI-3 torch orientation, SN-2/3 (model/morph checks): re-verified in one owner pass after slices 2-5.
Combat (3.5), R4-R8 and the B3+ traversal phases come after this plan.

## Status

- [x] 1 test level (2026-10-22)
- [x] 2 R0 write-back (headless; PH-2 awaiting owner)
- [x] 3 idle and crowd variety
- [x] 4 foot-slide metric (fix not landed: 4b)
- [x] 4b strafe/back slide: lock-drift metric, toe-aware stance velocity, per-gait stride speed
- [x] 5 feet on stairs and slopes (stairs left for 5b): ramps gated (sole within 2 cm), toe probes + slope clearance; stairs ignored (comfy-up toe 5 cm: lock vs heel-strike; steep flight needs grade-aware stride, 5b)
- [ ] 5b stairs (owner 2026-10-22: look fine in the lab; low priority, measure only): foot lock follows toe roll in stance; steep-stair stride (owner call); map MW toe bones
- [x] 6 R2 reaction controller core (`sf_ragdoll::react` + thin wiring in `sf_physics_bevy`; stagger/weight outputs consumed in R3)

## Owner pass 2026-10-22 (anim-lab, after slices 1-6): open issues

Fix the cause, not the symptom: for each, find which stage (clip data, cook, graph, root motion, controller,
physics, ragdoll hand-off, write-back) produces it, and say why in the slice handoff before changing code.

- **AN-1 Foot slide at one moment of the walk cycle:** fixed R3-1 (A2 updates got only the last frame's motion for the foot lock; the feed now composes `drive.moved` until an update consumes it). the planted foot moves with the body during one specific
  phase of the walk (seen on the walker next to the orange cube). Slices 4/4b gated strafe/back, not this.
- **AN-2 Stops are far too slow:** fixed R3-2 (`PathFollower` exponential arrival braking 0.64/s from 2 m plus a 0.25 m/s creep; now constant-deceleration arrival that budgets the velocity spring's lag, cutoff to zero, idle on the stop request; walk 1.5 m/s stops in 0.59 m/0.87 s, was 1.9 m/3.0 s). walkers reaching their destination decelerate over a long glide. People stop
  within one or two steps at normal speed. Look at how shipped games do it (stop clips/foot-aware stops, decel).
- **AN-3 Footstep sounds out of sync:** fixed R3-1 (sounds came from a distance timer, not the animation; now the graph's `SoundGen` key events, payload = foot). with the feet (animation text keys vs playback rate/SpeedMatch?).
- **AN-4 Pushed by the ball, characters sometimes freeze** (animation stops) and **lose their capsule**: fixed R3-3 (knockdown paths that never returned: failed handoff retried forever, fallback get-up and a skipped `GettingUp` never told the host, no settle timeout; now `RagdollFailed`, every path hands back, 6 s timeout).
- **AN-5 Get-up pops:** fixed R3-3 (the 0.1 s state-switch blend overrode the 0.3 s hand-back; frame not on the ground; pelvis moved by the build-scale restore). A teleport from the lying ragdoll pose to the first frame of the stand-up clip.
- **AN-6 After get-up, the character slides** (fixed R3-3: the get-up tail kept its lying root offset, 1.1 m off the capsule; now cooked in place, anchored on its last frame) to its capsule at the end of the stand-up clip.
- **AN-7 Capsule vibrates while walking:** fixed R3-1 (capsule targeted the last frame's feet, beating against the 60 Hz step; now the feet at each step's time, G draws interpolated poses). (G view). Is it expected? Probably not.
- **AN-8 Mudcrab capsule too small** while standing: fixed R3-4 (every creature got a fixed `0.4*s` x `1.2*s` column, never sized from its model; now `MoverParams::from_bounds` over the outfit's bind-pose bounds, and a body wider than it is high gets an upright cylinder, `BodyDesc::character`).

## Owner pass 2026-10-22, second (after run 3): open issues

AN-4 (freeze, lost capsule) confirmed fixed by the owner. Same rule: find the cause before changing code.

- **AN-9 Residual foot slide on every walker:** fixed AF2-2 (the stance ended at heel-off and the ankle lock lerped back to the animation while the toe was still down, returning all the stance drift at weight transfer; now cooked `roll.<foot>` curves pin the toe from heel-off to toe-off and the release decays in swing). a planted foot holds for most of its stance, then slides a few
  cm forward right when weight transfers and the other foot lands. Owner suspects something fundamental in
  how foot placement/locking works, or an incompatibility with Morrowind's base anims, not a near/far issue.
- **AN-10 Get-up spin:** fixed AF2-3 (`ROLL_TO_BACK` rolled face-down bodies about the frame's up axis, a 180° heading flip; now about the root body's long axis). no horizontal slide anymore, but characters often spin fast between the lying pose
  and the first frame of the stand-up clip (heading mismatch between the ragdoll and the clip's start).
- **AN-11 Ragdoll torso:** the torso rotates freely with the hips and bends unnaturally; check every joint's
  constraint (limits, axes, twist/swing, spine chain). **Fixed (run 2, awaiting owner):** axes were right (AngX = twist about the bone, AngY/Z = swing across it); the spine allowed 30/30 degrees per segment, now 15/15 (`sf_ragdoll` cook, `ragdoll.cook` v2); test `spine_segments_allow_only_a_little_bend_and_twist`.
- **AN-12 Guar box too small** (not tall enough). Creature shapes must fit their models.
- **AN-13 Guar hit by the ball loops forever** fixed AF2-1 (no `Knockout@tail`: `knockout_end` dropped with its transitions, so `knockout` had no exit; it now exits to locomotion when the flag clears) between falling and standing up.
- **AN-14 Dead stays dead:** fixed AF2-1 (a live knockdown ragdoll ignored the host's replaced death request; it now goes to the controller's terminal Dead). an actor killed while knocked down must not stand up.
- **AN-15 Creature collision from several primitives** (industry standard, e.g. per-girdle shapes) so
  quadrupeds stand correctly on slopes; replaces the single oversized capsule/cylinder.

## Owner pass 2026-10-22, third (after AN-9): open issues

- **AN-16 Fargoth takes ridiculously long strides:** fixed af3-gait (`SpeedMatch` put the whole speed ratio into play rate, so steps never shortened: the MW male walk, 1.1 m steps made for 2.1 m/s, played at 0.64x at 1.4 m/s; and its feet skate 10% under the root, inside the old 20% `STRIDE_TOLERANCE`, so the lock drifted 30 cm. Now `rate_share` 0.5 splits the ratio between cadence and stride warp, bounded by the warp's limits; tolerance 5%). Awaiting owner. Left: male/beast walk lock drift still ~12 cm mean (clip skate within the stance), not chased.
- **AN-17 Elone walks stiff, jolting every step:** fixed af3-gait (the female walk's left `contact` dropped for one frame at the loop seam: two plants per step, the lock released mid stance; the cook now closes seam gaps up to `ContactRules::gap` 0.05 s, `anim.cook` v7). Awaiting owner. Lead: the beast walk's left heel-off falls on the seam, so `roll_frames` (wraps the whole clip, not the cycle) gives that foot no toe pin.
- **AN-19 Per-step jolt on the MW male and beast walks:** fixed af3-skate (awaiting owner). Two clip defects, both
  turned into lock drift (12-19 cm per stance, released as a whip): the extracted root carried the hips' yaw swing
  (male ±17°, beast ±13°), so the rootlocal body turned about the capsule and swung the planted foot; and the stance
  feet moved unevenly against the root. Fix in the cook (`contact_root`, `anim.cook` v8), the root half of footskate
  cleanup (Kovar, Schreiner & Gleicher 2002, as motion-matching data prep re-derives the root): a travelling loop's
  heading is its yaw averaged over a cycle, and in stance the planar root moves with the pinned point (heel, toe while
  rolling) plus one constant `u` that closes the loop, so the planted foot moves back at exactly `u`, which becomes the
  stance velocity; double support hands over by trapezoid weights (`ContactRules::root_blend` 0.1 s). Chosen over leg
  IK (Kovar's other half) because the runtime foot lock already does the IK and needed only a consistent root, and
  over a time warp (would re-time events, curves, markers). `stride_velocity` is now the feet's whenever measured
  (5% tolerance dropped). Clip-space drift per stance: male 2.4/0.8, beast 2.0/4.5, female 2.9/4.3 cm (was 13-19,
  16-21, 5); sim lock drift mean 1.2-1.6 cm (was 12), foot peak = authored (no release spike). Left: sim max drift
  9-12 cm (start transition?), recook at the chosen rate may exceed the 0.1 mm error bound slightly, beast left foot
  toe pin on the seam (not addressed), strafes not gated.
- **AN-18 Ragdoll torso spins, limbs too free.** **Fixed AF3 (awaiting owner):** not the drives or the limit axes: the pelvis
- AN-20 (ragdoll looseness, in progress): AN-18's inertia floor on every body made limbs 3-12x harder to swing (knockdown arms 28° vs 44° with shape inertia, head 11° vs 32°; stagger 3° vs 12°) and the drive gains ignored it. Now: floor on core and end bodies only, mid-limbs keep shape inertia plus a long-axis lift, gains count the floored inertia, 16 limp iterations; anatomical asymmetric hip/shoulder/neck ranges (`CookParams::ranges`, ragdoll.cook v4). Gate `hits_leave_limbs_loose`. Open: 4 tests red (see .sessions anim-fixes-3 af3-stiff).
  was the lightest body (2.9 kg, inertia ~0.005 kg·m² under two 9.9 kg thighs), so the joint rows were ill-conditioned and
  limits gave way (spine 25° past its 15° limit at a 120 N·s chest hit, 95° and stuck flipped at 300). Now core bodies weigh
  at least their heaviest child (cook, `ragdoll.cook` v3) and every body gets an inertia floor `mass · reach²` (inertia
  conditioning, `RagdollSettings::inertia_floor`, `sf_physics` `BodyDesc::min_inertia`); limb cones tightened (arms 80/45,
  legs 60/20, hands/feet 35/15, head 40/30). Test `falls_keep_joints_in_limits_and_the_torso_from_spinning`.
  Open: asymmetric swing ranges with anatomical frames (hip flexion vs extension, shoulder), data-overridable per limb kind.
- **AN-21 Slope-aware walking (owner: shorter steps uphill, longer down, a subtle lean).** Done af3-slope (awaiting owner):
  the graph filters the ground grade along the travel from the probes (least squares through the hits' heights plus their
  normals, half-life 0.25 s; `GraphOutput::grade`); `StrideWarp.grade` (up 0.45, down 0.25) scales the stride, SpeedMatch's
  cadence takes the rest (`rate_share` untouched); `Lean.grade` pitches chest/neck (0.12/0.06 rad per unit grade, cap 4°).
  30° male walk: step 1.01 -> 0.75 m up, 1.15 m down; trunk +2.5°/-1.2°; feet within 2 cm. Graph format 6.
  Left: path/AI speed vs grade (locomotion.md owns no grade hook; Tobler-style factor proposed), stairs (5b).
