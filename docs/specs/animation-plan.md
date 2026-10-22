# Animation plan: slices (anim-refactor lane, 2026-10-22)

Execution order for ROADMAP Phase 3 items 3.1, 3.2 and 3.4, ordered by "characters look best soonest".
Design lives in [animation.md](animation.md) ("Decided shape (R11b)", layers 1-4), [action.md](action.md)
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
- [ ] 5b stairs: foot lock follows toe roll in stance; steep-stair stride (owner call); map MW toe bones
- [x] 6 R2 reaction controller core (`sf_ragdoll::react` + thin wiring in `sf_physics_bevy`; stagger/weight outputs consumed in R3)

## Owner pass 2026-10-22 (anim-lab, after slices 1-6): open issues

Fix the cause, not the symptom: for each, find which stage (clip data, cook, graph, root motion, controller,
physics, ragdoll hand-off, write-back) produces it, and say why in the slice handoff before changing code.

- **AN-1 Foot slide at one moment of the walk cycle:** the planted foot moves with the body during one specific
  phase of the walk (seen on the walker next to the orange cube). Slices 4/4b gated strafe/back, not this.
- **AN-2 Stops are far too slow:** walkers reaching their destination decelerate over a long glide. People stop
  within one or two steps at normal speed. Look at how shipped games do it (stop clips/foot-aware stops, decel).
- **AN-3 Footstep sounds out of sync** with the feet (animation text keys vs playback rate/SpeedMatch?).
- **AN-4 Pushed by the ball, characters sometimes freeze** (animation stops) and **lose their capsule**.
- **AN-5 Get-up pops:** a teleport from the lying ragdoll pose to the first frame of the stand-up clip.
- **AN-6 After get-up, the character slides** to its capsule at the end of the stand-up clip.
- **AN-7 Capsule vibrates while walking** (G view). Is it expected? Probably not.
- **AN-8 Mudcrab capsule too small** while standing.
