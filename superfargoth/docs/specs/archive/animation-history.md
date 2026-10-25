# Animation: history and as-built notes

Moved verbatim from [animation.md](../animation.md) (doc cleanup S7a, 2026-10-25): as-built notes, audit, optional clips, industry comparison.

### As built (animbevy, 2026-10-12)

**Industry check (Bevy 0.20 source, read 2026-10-12).**
- `bevy_animation::animate_targets` is a `par_iter_mut` over *bone*
  entities (`AnimationTargetId` + `AnimatedBy(player)`): each bone looks up
  its player and graph and samples its own curves, then writes `Transform`
  through `AnimationEntityMut`, every frame. We keep the parallel per-bone
  write-back shape (it is the only parallel, `unsafe`-free way to write many
  entities' `Transform`s from ECS), but the pose is computed once per
  skeleton in `Evaluate`, so a bone only copies a `Trs`. We also compare
  before writing: unchanged and frozen (A3) bones don't trip change
  detection.
- `bevy_gltf` names node entities `Name::new(node name)` and builds
  `AnimationTargetId::from_names(path)` from the root; it binds skins by
  node index. Our rig spawn names bones by skeleton joint and skins bind
  by name (`SkeletonBinding::find`: exact, then ASCII case-insensitive).
- Transform propagation (0.20): `StaticTransformOptimizations` (on by
  default) marks dirty subtrees from `Changed<Transform>` up to the root
  (`mark_dirty_trees`), and `propagate_parent_transforms` skips clean
  subtrees, in parallel. Animated skeletons are dirty every frame by
  nature; the write-if-changed rule keeps A3 and idle-but-unchanged bones
  out of it. The "write `GlobalTransform` directly" fallback above stays
  unneeded until a profile says otherwise.
- UE (docs: "Animation Optimization", Animation Budget Allocator plugin):
  URO skips evaluation by screen size and interpolates the skipped frames;
  the Budget Allocator takes a fixed ms budget, a per-component
  significance, and throttles the least significant first (tick rate, then
  interpolation off, then no evaluation), learning cost per component. Our
  tiers are URO's screen-size rule; the allocator is the same "least
  significant first, measured cost" loop with per-tier EMA cost.

**Crate.** `sf_anim_bevy` (deps: bevy_app/ecs/transform/math/time/camera,
`sf_diag_bevy` for counters; `debug` adds bevy_gizmos/bevy_color).
- `AnimAsset` (skeleton + compiled graph + clips, validated by
  `Binding::new`, shared `Arc`), `Animator { asset, instance, output,
  drive: AnimDrive, time_scale }` with `set_float/bool/vec2/fire` by name,
  `AnimLod`, `SkeletonBinding`, `AnimJoint { character, joint }` on bones,
  `AnimRootMotion { delta, drive_weight, apply }` (store for the mover, or
  apply to the root `Transform` when there is no mover).
  `AnimDrive::moved` is composed by the host each frame (`RootDelta::then`)
  and reset by Evaluate per update, so A2 updates get every frame's motion
  (R3-1). `AnimEvent`/`AnimNotify` carry the clip event's `payload`.
- `spawn_rig(commands, character, &Skeleton)`: one entity per joint with
  `Name`, rest `Transform`, `AnimJoint`, parented by the skeleton. It
  replaces `sf_model_bevy::spawn_rest_skeleton` for animated characters
  (the scaffold stays for the bind-pose `skinned` example).
- `PostUpdate` sets chained before `TransformSystems::Propagate`:
  `AnimSystems::{Lod, Probe, Evaluate, Apply}`. `Probe` is empty here: the
  host fills `Animator::drive` (ground samples, IK/warp targets).
- Events: `AnimEvent` message (`MessageReader`) and `AnimNotify` entity
  event (`On<AnimNotify>` observers) per graph event, names shared as
  `Arc<str>`.
- LOD: tier from projected height normalised to the 1440 px reference
  (resolution-independent), × significance, with 15 % promotion
  hysteresis; outside the view cone (padded by the character's height) →
  A3. A2 updates every 2nd frame, staggered by entity, with the skipped
  time handed to the next update. Budget (`AnimLodSettings::budget_us`,
  off by default): `lod::allocate` demotes least significant first, one
  tier per round, cost per tier = EMA of measured µs. Forced tiers bypass it.
- Counters (when `DiagCounters` exists): `anim/evaluate/ms` (wall),
  `anim/evaluate_cpu/ms` (summed), `anim/apply/ms`, `anim/characters/count`,
  `anim/evaluated/count`, `anim/a0..a3/count`, `anim/events/count`,
  `anim/bones_written/count`; `AnimStats` holds the same for overlays.
- Gizmos (`debug`, `AnimGizmos { enabled }`): a line per bone, tinted by tier.
- Left: A2 interpolation between updates (now: the pose holds), shadow
  casters off-screen as A2 (host can force), morph curves →
  `ModelMorphWeights`, LOD joint masks on write-back (the runtime keeps
  joints outside the mask at their last value; the compare skips them),
  the Probe stage's batched ground queries (`sf_controller_bevy`), timings
  on the crowd bench.

### Foot placement guards (as built, animfix 2026-10-12)

**Root cause of the walkers' blow-ups** (legs far behind the body, pelvis on the ground, sliding
like that): a planted foot's lock is held in the world while the contact curve is above 0.5. When
the character travels faster than its cycle (playback clamped by `SpeedMatch`, avoidance pushes,
a walker sliding at < 0.15 m/s while its idle — contact 1 on both feet — never releases), the
lock drifts behind the hip. Once it is beyond the leg's reach, `rise_cap` (target height − hip
height, with no horizontal term left) pulled the pelvis down to the lock's height: ~a leg length.
Reproduced headless (`foot_placement_guards_stop_lock_drift_teleports_and_bad_samples`, guards
disabled: drop > 0.5 leg at 6 m/s on a 1.2 m/s cycle). No probe faults were needed for it; in
Balmora the guard counters show none (0 rejected samples, 0 teleports, 0 non-finite in 150 s).

`FootPlacement(guards: FootGuards)` (UE Foot Placement / Leg IK practice; lengths are fractions
of each leg's reach, so they hold for any rig and race scale):

| guard | default | does |
|---|---|---|
| `max_step` | 0.5 | a ground sample further above/below the animated foot is rejected (roof, ledge, stale hit) |
| `max_drop` / `max_rise` | 0.4 / 0.5 | girdle offset limits (the pelvis never drops further; `Fit` raises the uphill girdle) |
| `unplant` | 0.3 | a lock drifting this far (horizontally) from the animated foot releases over `release`, then re-plants (UE's unplant radius) |
| `max_speed` | 8 m/s | above it the op blends out |
| `blend_halflife` | 0.1 s | the op's weight is a critical spring: 0 while no leg has a ground sample (probes off, dormant, airborne, in-place lineup) or the character is fast; blends in after a LOD return or wake |
| `foot_halflife` | 0.04 s | per-foot height offset smoothing (a lock plants on the raw ground height, not the smoothed one) |

Always on: a `moved` longer than a leg's reach or non-finite resets the locks (teleports, door
crossings); every target is clamped to its leg's reach from the hip after the body fit; any
non-finite offset or target skips the op and resets its state. `GraphOutput::feet: FootStats`
counts what fired per update; `AnimStats::{feet, feet_total}` sums them, and the Bevyrrowind
walkers report logs the totals (Balmora 150 s, ~40 walking: 65 unplants, 0 pelvis clamps;
stairs: 91 rejected samples, 166 pelvis clamps, all bounded). Graph format 5.

The humanoid template now runs `SpeedMatch(0.5–1.5)`, `StrideWarp`, `FootPlacement` (see
locomotion.md "Measured feed").

### Morph curves (as built, actorfx 2026-10-12)

Clip curves named like a character's morph targets drive them
(`sf_character_bevy::morph`, character.md "Morph playback"): `Evaluate` already
blends curves; `drive_character_morphs` maps `GraphOutput::curves` to
`MeshMorphWeights` by name after `Apply`, plus an externally timed
`MorphCurvePlayer` for faces. `AnimImporter::companion`/`import_with` let an
importer read curves keyed in a sibling file (legacy models key morphs in the
model, bones in the animation file). Facial LOD: morph targets only on outfit
LOD 0-1; skipped while the animator is frozen or culled (j9m3's "morphs only
near the camera").

### Optional clips (M3, for bring-your-own sources)

Modern clip sets come from libraries the user supplies (the content pack's
data plan), so a graph must compile when some are missing. A clip
reference may be `optional` with a `fallback` (another clip reference, or
`drop`): a missing blend-space sample is dropped and the space
re-triangulated, a missing state clip falls back, a transition into a
state whose clips all dropped is removed. `animgraph.compile` reports
which slots resolved to which source (provenance kept in the compiled
graph for the license check of the content pack's export). Clip
references already route through `GraphRules`, which is where an adapter
picks the best available source per slot.

**Optional clips as built (optclips, 2026-10-12).** General engines meet
a missing animation at load or play time (a null motion that plays the
reference pose or nothing). We resolve at bake instead, so the runtime
never sees a hole and the substitution is recorded.

```ron
clips: {
    "walk_kf": "xbase_anim.kf#WalkForward",                     // required
    "walk": (source: "byo/ual2.glb#Walk_Loop", optional: true, fallback: Clip("walk_kf")),
    "run":  (source: "byo/100style.bvh#Run", optional: true),   // fallback defaults to Drop
    "getup_back": (source: "byo/lafan1.bvh#getup", optional: true, fallback: Drop),
},
```

- **Syntax** (`asset::ClipRef`): a plain string is required. A struct
  needs `source`; `fallback` is `Clip(<clip name or raw reference>)` or
  `Drop` (the default) and requires `optional: true`; unknown fields are
  errors. A fallback name is resolved like any clip name, so chains work
  (`walk → walk_mocap → walk_kf`); cycles are errors; a raw reference or
  an undeclared name at the end of a chain is required.
- **Compile** (`compile::Compiler::clip_slot`): a required clip loads
  directly, so a missing one fails the compile with the source's error
  (`(required fallback of "<name>")` appended inside a chain). An optional
  clip asks `CompileSource::available` first; when it is missing, its
  fallback is tried. A drop is structural: a blend-space sample leaves the
  space (re-normalized and re-triangulated over the rest; with none left
  the space drops); `Clip`, `RateScale`, `Mirror`, `Subgraph` and a
  `LayeredPerBone` base drop with their input; a `Blend` with one side left
  and an `Additive` with its additive side gone become a pass-through to
  the remaining input; a dropped state leaves its machine with every
  transition from or to it (a transition to an *unknown* state is still an
  error); a dropped layer leaves its list. Errors: the initial state
  dropped, or the base layer dropped ("nothing to play"). Dropped
  subtrees are truncated from the depth-first node table, so ranges stay
  valid.
- **Bake step**: `GraphRules::clip_files(graph, skeleton, reference)`
  (default: the `<file>` of `<file>#<clip>`) lists the source files; the
  step checks each with `RunCx::file_exists`, which records the
  fingerprint even when it is missing, so adding (or removing) a source
  recompiles exactly the graphs that asked for it. An adapter
  `clip_product` returning `None` also counts as missing. Step version 3,
  `SFAG` format 3.
- **Provenance**: every `ClipSlot` carries `Provenance { source, license,
  redistributable }` from `GraphRules::provenance` (default: own content,
  shareable; the framework names no libraries or licenses).
  `CompiledGraph::resolutions` lists, per clip name in first-use order
  (`<subgraph>:<name>` inside subgraphs), the references found missing
  and the slot it resolved to (`None` = dropped): that is the "which slot
  came from where" report. `CompiledGraph::non_redistributable()` is what
  an export guard refuses.
- **Tests** (`sf_anim_graph/src/tests/optional.rs`): missing optional
  clips fall back or drop (sample, state + 2 transitions, layer), the
  node layout stays valid and the graph plays; all sources present keeps
  everything; provenance flags the private clip and survives the
  round-trip; diagnostics (missing required, required fallback, cycle,
  fallback without `optional`, unknown field, nothing to play, initial
  state dropped, unknown state still an error, blend pass-through); the
  bake step falls back, stays warm, recompiles onto a source when its file
  appears, and falls back again when it is removed.

### Slots and montages (M3-d, as built 2026-10-12)

Gameplay, AI and scripts play one-shot clips without an authored state per
clip: UE's Animation Montages through Slot nodes (gap G4, industry-audit.md).
Code: `sf_anim_graph::slot`, `GraphInstance::play_slot` / `stop_slot` /
`montage`; tests in `src/tests/slot.rs`.

- **Graph side.** `Slot(slot: "upper", input: "locomotion", group: "default",
  mask: "upper")` passes its input through until a montage plays in the
  slot, then shows the montage, over the whole body or through the mask
  (then the input still shows elsewhere and keeps playing; a full-body
  montage hides the input, whose clocks hold, UE's default without "always
  update source pose"). Several nodes may name one slot (they show the same
  montage). Slot groups as UE's: starting a montage in one slot of a group
  ends the montages of its other slots. The mask lives on the node so an
  upper-body slot doesn't need a cached pose (we have no cached-pose node;
  node references are instances).
- **Which clips.** By id from the binding: the graph's `slot_clips: [...]`
  list puts named clips into the compiled graph's clip table without a
  node playing them (`CompiledGraph::slot_clip(name)`; optional clips that
  are missing are not offered), and any other clip slot of the binding
  plays too. Late-bound: `LateClip::new(graph, skeleton, Arc<Clip>)` maps
  any cooked clip on the skeleton onto the graph at runtime (events and
  curves by name, `warp` windows parsed by the same `parse_warp_window` the
  compiler uses, against the graph's warp targets); built once per (graph,
  clip) and shared. Late events report `clip == LATE_CLIP`.
- **Request.** `SlotRequest { clip, start: Interrupt | Queue, rate, from,
  loops, blend_in, blend_out, end_early, root_motion, priority }`.
  `from` starts mid-clip (an adapter's "start at the loop start");
  `loops` repeats the clip's loop range that many times before playing out
  to the end (`LOOP_FOREVER`); `end_early` is UE's blend-out trigger time
  (the montage ends that many seconds before the clip end; 0 plays it out).
  Defaults: interrupt, rate 1, 0.2 s blends, `Drive`.
- **Interrupt and queue.** `Interrupt` ends the playing montage
  (interrupted) and drops the slot's queue, unless the playing one has a
  higher `priority` (then `SlotError::Busy`). `Queue` waits behind the
  playing and queued montages and starts at once in a free slot; the next
  queued montage starts on the update after the previous one showed its end
  frame, with no frame of the input between them. `stop_slot` ends the
  montage and the queue.
- **Blending: inertialization only.** Into a montage, between montages and
  back to the input, the slot asks the graph's post-process inertializer
  for one blend (`blend_in` of the new montage on start or interrupt, as UE
  uses the interrupter's blend-in; `blend_out` when it ends or is stopped),
  only when the slot is evaluated this update. One pose is evaluated per
  slot, never two. UE's montages crossfade by default and offer
  inertialization per montage (`BlendModeIn/Out`); we only inertialize,
  like our state machines' default.
- **Clock, events, root motion.** The montage clock runs every update, even
  when no evaluated branch reaches its slot (UE ticks montages on the
  instance), so script timing never depends on the graph state. Its events
  all fire, once per crossing (`Clip::advance` per pass), outside the
  per-layer highest-weight selection, tagged `GraphEvent::montage`; an
  interrupted or stopped montage closes the windows it is inside with
  `Exit` events at the next update (UE ends notify states on
  interruption). Root motion follows the request's mode: a `Drive` montage
  outputs exactly a `Drive` player's root delta (tested per update), through
  the mask's root weight on a masked slot; its `warp` windows warp like a
  player's where the slot is evaluated.
- **Notices.** `GraphOutput::slot_notices`: `Started` and
  `Ended { interrupted }` per montage id (UE's OnMontageStarted / OnBlendOut /
  OnMontageEnded collapse to these two, since an inertialized blend-out has
  no overlap to report).
- **Not done.** Montage sections and branching points (UE sections with
  next-section links): no legacy need yet, a clip plus `from`/`loops`/
  `end_early` covers the group ranges; additive montages (rejected); a
  per-slot "always update source pose" option; per-request blend profiles.

**Adapter hook for scripted "play this group" requests.** The framework
knows only `SlotRequest`. An adapter maps its script verbs onto it, e.g.
the Bevyrrowind side for PlayGroup / LoopGroup (UESP docs: flag 0 lets the
current animation finish its cycle, then the group starts from its first
frame; 1 starts it at once from its first frame; 2 at once from its loop
start; LoopGroup's count is how many times it plays, and the original plays
exactly that many, which OpenMW issue 7641 reports OpenMW overshooting by
one): `Queue` for flag 0, `Interrupt` for 1 and 2, `from` = the loop start
for 2, `loops` = count − 1. Which slot a script group plays in, and
whether flag 0 waits for the base locomotion cycle (the framework's
`Queue` starts at once in a free slot), are adapter rules. OpenMW's Lua
`playQueued` / `playBlended` (loops, speed, start/stop keys, priorities,
bone-group masks) cover the same ground; its "Scripted" priority pausing
all other animations is what a full-body slot with a high `priority` does.
(Built 2026-10-12 on the Bevyrrowind side, `mw_graphs::script`, with its graph templates: scripted
groups in a `full` slot at priority 100, attacks in a masked `upper` slot.)

### Creatures (M3)

Skeleton-agnostic core plus a rig profile per skeleton
([rig-profile.md](../rig-profile.md)): N support limbs, girdles, spine,
neck/head, tails, wings. Every op above reads limbs from the profile; the
humanoid rig is an optional layer for retargeting. The audit of the built
crates against this is below.

### Audit of the built crates: biped and humanoid assumptions (2026-10-11)

| # | Where | Assumption | Fix |
|---|---|---|---|
| 1 | `sf_anim_graph/src/compiled.rs:164`, `asset.rs:353` | `Leg` is exactly hip, knee, foot | `Limb` from the rig profile: N joints, solver kind, pole hint, girdle |
| 2 | `sf_anim_graph/src/procedural.rs:160-209` (`FootPlacement`) | one `pelvis` drops by the largest need (`:198`, `:203`) | one offset per girdle; body pitch and roll fitted from the girdle offsets and spread over the spine |
| 3 | `procedural.rs:86-93` (`solve_leg`) | two-bone only; straight legs fall back to a +Z pole (`:90`), wrong for backward-bending hind legs | solver by limb (`TwoBoneAnkle`, FABRIK); `PoleHint::Axis` from the profile |
| 4 | `procedural.rs:111-143` (`StrideWarp`) | scales about one pelvis; drop along −Y of the single pelvis (`:138`) | per girdle center |
| 5 | `procedural.rs:144-159` (`OrientationWarp`) | biped lower-body/torso split | biped template only; add `SpineBend` for quadrupeds |
| 6 | `procedural.rs:230-242`, `compiled.rs:181` (`HandIk`) | three joints, position only (`instance.rs:141` targets are `Vec3`) | `LimbIk` with optional rotation; `IkTarget { position, rotation }` |
| 7 | `sf_anim_graph` ops | FABRIK/CCD exist in `sf_anim::ik` but no op exposes them | `ChainIk` op |
| 8 | `sf_anim/src/cook.rs:854` | contacts only on joints tagged `foot` | support-limb effectors from the profile |
| 9 | `sf_anim/src/cook.rs:114` | contact height 5 cm and speed 0.3 m/s absolute | fractions of leg length from `Measures` |
| 10 | `sf_anim/src/skeleton.rs:124-139, 144-185` | tags and name guesses are humanoid only | add `tail`, `wing`, `claw`/`hoof` tags; limbs come from `rig.profile`, not names |
| 11 | `sf_anim/src/retarget.rs:43-44` | retarget IK fix-up needs exactly three joints | solver by limb kind; per-limb scale for IK goals |
| 12 | `sf_anim/src/clip.rs:89-93` (`RootDelta`) | root motion is translation + yaw only | flyers and swimmers: optional pitch/roll (`RootMotionRules::full_rotation`), ground creatures unchanged |
| 13 | `sf_controller/src/intent.rs:19`, `mover.rs:251` | horizontal intent; vertical root motion dropped while grounded | `Fly`/`Swim` mover modes with 3D intent; `Traversal` flag for warped vertical motion |
| 14 | `sf_controller/src/mover.rs:69-79`, `probe.rs:17-18` | upright capsule only | M3: half-body-length capsule for long creatures; M4: horizontal capsule shape |
| 15 | `sf_anim_graph/src/procedural.rs:102` | `probe_origins` are per leg (already N) | none: works for N legs; keep |
| 16 | `sf_character/src/records.rs:128` | `CharacterRecipe::rig` is a required `RigMapRef` (humanoid) | `RigRef = Humanoid(..) \| Profile(..)`; `RigMap` also produces a `RigProfile` (rig-profile.md, proposed character.md changes) |
| 17 | `sf_character/src/rig.rs:189-194` (`retarget_rig`) | leg chains are exactly upper leg, lower leg, foot, skipping any joint between (digitigrade metatarsal) | chain = the joint path from upper leg to foot, solver `TwoBoneAnkle` when it has four joints |

**Status of the audit (anim-rig, 2026-10-12).** Fixed: 1, 2, 3, 4, 6,
7, 8, 9, 10, 11, 12, 16, 17 (and 5 partly: `SpineBend` and `Lean` added,
`OrientationWarp` unchanged). 13 and 14 fixed by anim-creatures
(2026-10-12): `RootMotion::traversal`, `MoverMode { Ground, Fly { hover },
Swim { surface } }` with 3D intent, `Locomotion::attitude` from root tilt,
`MoverParams::from_profile` (locomotion.md "As implemented"). Details in
rig-profile.md "As implemented".

**Full-body IK as built (`sf_anim::ik::FullBody`, `FullBodyIk` op, 2026-10-12).**
Research (rule 8/9): UE5 FBIK (PBIK: particles, per-bone stiffness, `PrePull` root,
20 iterations), XPBD (Macklin et al. 2016), FABRIK's multi-end-effector
extension (Aristidou & Lasenby 2011; sub-bases, no stiffness or limit model),
Final IK's `FullBodyBipedIK` (body solved as a few weighted nodes, then each
limb by analytic two-bone IK; biped only) and `VRIK` (three-point humanoid
heuristic). Crates: `lux-ik` (Jacobian DLS, proof of concept, unmaintained
since 2023), `skelly` (FABRIK, 2021), `k` (robot-arm Jacobian IK on
nalgebra), Rapier's multibody IK (Jacobian, physics-engine bodies),
`bevy_mod_inverse_kinematics` (single chain, Bevy-bound): none solves a
rig-profile body with stiffness, limits and determinism, so we wrote ours on
our pose types. Shape: one particle per body joint (girdles, spine, neck,
head, limb joints, closed to their common ancestor = root). Per iteration:
soft root anchor; joint stiffness (each bone keeps its animated direction in its parent's
frame turned by the parent bone's swing; compliance-weighted PBD steps
under-relaxed by 0.5, no Lagrange multipliers: measured, XPBD's multipliers
at these compliances need ~150 iterations to reach their equilibrium and a
full step makes the stiff torso and the hard constraints trade the same
error every iteration, a period-2 oscillation of 10-17 cm on the biped);
torso limits in the solve (a capped joint hands the bend to its neighbours);
**limb ends constrain their limb root to a sphere of comfortable reach**
(`settings.reach`, 0.95 of full reach) around the goal, reaching limbs first,
planted (`pull` 0) limbs last; hard bone lengths swept down then up the tree.
Then rotations are recovered top-down (swing to the solved child, best-fit
rotation over several children by Müller et al. 2016, the animated twist
kept), each joint's local change clamped to its limit; joints on the path to a pinned non-limb
effector (a head) turn to that child exactly; the body translates by the
mean residual of those effectors (the recovery's loss is mostly radial,
which rotations cannot remove); planted limbs move the root back within
reach; one CCD sweep up each chain to the root's child brings a head to its
goal and a reaching limb's root within comfortable reach (stopping at 1 mm); finally each limb end is solved exactly by its limb's
own solver (two-bone, two-bone-ankle, FABRIK), the knee toward its animated
side turned with the limb (the Final IK step), limits re-clamped. Defaults by
role (`BodyRole`; tune per joint with `FullBody::joint_mut`): limits are
deviations from the animated pose (the profile has no authored ranges):
root 45°, spine 30°, neck 50°, clavicle/scapula 30°, hip/shoulder 120°,
knee/elbow 150°; knees and elbows never pass straight (two-bone hinge).
Deviation from the plan above: no particle hinge or cone-twist constraints;
the limb pass makes them redundant and its pole is flip-free. Op:
`FullBodyIk(effectors: [(limb | joint, target, weight, pull, rotation)],
pin_support: true, pin: [..], iterations: 10, max_lod: 0)`; support limbs
are pinned where the ops before left them (after `FootPlacement`: the placed
feet, with their tilt). Tests (`sf_anim/src/fbik_tests.rs`, `creature_tests`
`full_body_ik_reaches_with_the_body_and_keeps_placed_feet`): a biped hand
0.84 m from the shoulder (arm 0.53 m) lands within 2 cm with feet ≤ 5 mm
from their pins, chest +10 cm forward, spine > 5°; reachable goals ≤ 1 cm at
10 iterations; unreachable goals clamp (arm toward the goal, no knee flip);
limits hold on every body joint; a moving goal moves no joint more than 3×
the goal's step per frame and repeats bit for bit; 1 and 8 threads give
identical poses; a quadruped head reach beyond the neck lands ≤ 1 cm at 10 and 20 iterations
(measured < 0.001 mm) with four feet planted, the chest lowered, no knee
or hock flip, the tail following the pelvis rigidly. Release timing
(`cargo test --release -p sf_anim --all-features -- --ignored --nocapture
full_body_timing`, reference laptop, 2026-10-12, fbik2): biped 17 joints,
2 feet + 1 hand, 5.0 µs (was 7.3); quadruped 31 joints, 4 feet + head,
7.5 µs (was 13.3); Quaternius humanoid 66 joints, 2 feet + 1 hand, 8.7 µs
(was 11.3; gate 8 µs still missed by 0.7 µs). Profile (humanoid): particle
loop 3.4 µs (latency-bound sqrt chains: one length sweep per iteration, a
square-root-free cone test, per-solve link table), recovery + full refresh
1.7, limb passes 2.0, reach polish 1.0. Next: refresh each subtree once
(recovery, polish and the limb passes each refresh the arm and finger
joints again) and read animated model rotations once. A worst-case A0
character (5.0 µs graph + 9 µs FBIK + 0.1 µs warp) stays inside the 35 µs
gate.

**Motion warping as built (`sf_anim_graph`, 2026-10-12).** Windows are
clip events named `warp` with payload `<target>;<skew|scale>[;key=value]`,
keys `rot=default|facing|none`, `y=ignore|warp`, `rot_time`, `max_dist`
(m), `max_turn` (deg/s), `ease` (fraction of the window, default 0.25),
`point=root|<joint>`; `animgraph.compile` parses them per clip slot
(`CompiledGraph::warp_windows`, `warp_targets`; graph format 2) and
precomputes a joint warp point in the root frame at the window end.
Runtime: `UpdateInputs::warp_targets: &[Option<WarpTarget { position,
yaw, velocity }>]` (character space; `velocity` extrapolates a moving
target to the end of the update, removing that update's latency);
after root blending, the highest-weighted `Drive` lane whose playhead
interval overlaps an open window with a set target has the in-window part
of its delta warped: translation by the rank-one skew `δ' = δ + (D −
R)(R·δ)/|R|²` (R = remaining authored path, D = vector to the target,
`y=ignore` keeps R's height; scale mode `δ·|D|/|R|`; linear fallback when
R ≈ 0), eased in by smoothstep over `ease`; rotation by the remaining turn
spread until `rot_time`, clamped by `max_turn`. Recomputed every update, so
the last update lands exactly. `GraphOutput::warp: Option<ActiveWarp {
target, remaining, error, yaw_error, vertical }>`; `vertical` is what the
controller glue passes to the mover as `RootMotion::traversal`. Measured: static
target end error 0.0 mm / 0.000°, root-speed jump at the window start
13% (target 30% farther: ≤ 20%), target moving 3 m/s: ≤ 5 cm (with
`velocity`), vault onto a 1.1 m ledge 0.0 mm, joint warp point within
2 cm. Cost: `cargo test --release -p sf_anim_graph --all-features --
--ignored --nocapture warp_and_creature_timing`: 0.10–0.12 µs per
warping update (gate 0.5 µs; release, reference laptop, 2026-10-12,
another session active). Through the controller glue, a warped mantle
onto a 1.2 m box (`sf_controller` `tests/graph_drive.rs`) ends within
1 mm of the target, grounded, the capsule never inside the box.

Generic already: the pose runtime, graph, sync, inertialization, `Mirror`
(any axis), `LookAt` (chain + aim axis), joint LOD masks, retargeting by
chain name, ground probes.

## Industry comparison

J9 (industry-audit.md), 2026-10-12, docs research only (`.sessions/j9m3.md`).
The Bevy 0.20 rows were read from `bevy_pbr-0.20.0` and `bevy_mesh-0.20.0`
source in the local registry (`render/skin.rs`, `skinning.wesl`,
`morph.wesl`, `mesh.wesl`, `prepass/prepass.wesl`, `morph.rs`).

| Technique | What shipped engines do | Us | Verdict |
|---|---|---|---|
| Graph shape | UE5 Animation Blueprints: state machines, blend spaces, layered blend per bone with blend profiles, sync groups and markers, cached poses, slots. Unity Mecanim: state machines, layers with avatar masks, 1D/2D and freeform blend trees. Creation Engine: Havok Behavior graphs (`.hkx`), state machines driven by named events, clip annotations as events. OpenMW: text-key driven animation groups, no graph; since 0.49 optional per-file `blending_rules` (from/to, easing, duration) behind "smooth animation transitions" | layered graph, weighted masks, sync groups and markers, blend spaces, state machines (Canonical technique) | **match** UE's model; Havok's event-driven state machines map onto our trigger params and event tracks |
| Transitions | UE inertialization node (Bollo, Gears of War); Unity and OpenMW crossfade or eased blends | inertialization, crossfade kept for layers | **match** |
| Runtime and threading | UE evaluates anim graphs on worker threads; ozz's job model (sample into buffers, blend, local-to-model) | ozz model, `par_iter` per character | **match** |
| Clip compression | ACL (UE's default codec) | ACL's uniform-sampling subset | **match** |
| Clip sample rate (animcook 2026-10-12) | ACL compresses raw tracks the caller has already sampled uniformly (`sample_rate` given) and measures its error against those raw samples; no rate search. UE resamples at FBX import (`Use Default Sample Rate` 30 fps, or `Custom Sample Rate`, 0 = derived from the file) and its codecs (ACL plugin included) compress against that raw data; per-bone virtual-vertex distances (3 cm default, larger for socketed bones). Unity keeps curves and reduces keys against angle/position/scale tolerances. None measures between-sample fidelity against the authored curves | rate searched per clip (30/60/120 Hz) against the continuous source, frames and midpoints, 0.1 mm at 1 m | **stricter than industry** (and ~2x more cook work); cheap fix done (resampling-error skip, early-out, reuse). **Proposal (P3)**: ACL-style split: pick the rate once from the source's key density (native rate, else the min rate; 60 Hz for dense keys), measure quantization against those frames only, and report the resampling error as a separate per-clip meta (`resample_error`) for review instead of searching rates. A Rust port of ACL doesn't exist (C++ only, MIT); an FFI binding is possible but our encoder already matches its uniform-sampling, per-track-bit-rate design |
| Root motion | UE: root motion from montages for actions, in-place + capsule for locomotion (Lyra); Unity `applyRootMotion` per animator | `Drive` / `Match` / `Ignore` per clip or state | **match** |
| Motion warping | UE Motion Warping: notify-state windows, skew warp (`SimpleWarp` deprecated for `SkewWarp`), warp point from a transform or a bone, "ignore Z" | Layer 1, same windows, modes and warp points | **match** |
| Stride / orientation warping | UE Animation Warping (Paragon, Fortnite, Lyra) | `StrideWarp`, `OrientationWarp` ops | **match**; OpenMW's "turn to movement direction" (lower body turns, head follows the view) is the same idea |
| **Distance matching** | UE Animation Locomotion Library (Lyra): starts, stops and pivots play by *distance* (a baked distance curve, `DistanceMatchToTarget` against the predicted stop or pivot point), not time, so stops land without sliding. The docs note it ignores the previous cycle's phase | none: starts and stops are time-driven states; `SpeedMatch` covers cycles only | **GAP (P2)**: legacy sets have no start/stop clips, so M3 loses nothing; library clips (animation-sources) do. Needs a distance curve at `anim.cook` and a stop/pivot prediction from the trajectory springs (locomotion.md). Open question 5 |
| Foot IK conventions | UE Foot Placement node: per-foot plant from root-motion speed or a per-foot curve, lock types, heel lift before pelvis drop (`heel lift ratio`), pelvis height modes `AllLegs`, `AllPlantedFeet`, `FrontPlantedFeetUphill_FrontFeetDownhill`, pelvis max offset. Unity: Mecanim humanoid foot IK, Animation Rigging Two Bone IK. OpenMW: no foot IK (observation) | contact-curve lock and release, pelvis drops by the largest need, two-bone with the animated knee as pole, per-girdle offsets planned (rig-profile.md) | **match** in technique; **differ**: our pelvis rule is UE's `AllLegs`; planted-feet-only and heel-lift are cheap refinements (open question 6) |
| Runtime rigging | UE Control Rig + Full Body IK (PBIK); Unity Animation Rigging (constraints on the Animation Jobs API: Two Bone IK, Multi-Aim, Twist Chain, Damped Transform) | procedural op stack + XPBD full-body IK (Layer 2) | **match** in capability; **differ**: ops are a closed enum, not a user rig graph, so graphs stay patchable data |
| **Gameplay-driven one-shot clips** | UE montages played into named **slots** of the anim graph (attack, interaction, emote: any clip, blend in/out, root motion, notifies, without a state per clip); Unity Playables / `Animator.CrossFade` by state name; Havok Behavior events; OpenMW plays any animation group on request (scripts) | `Slot` nodes with slot groups; `play_slot` takes a binding clip (`slot_clips`) or a late-bound `LateClip`, interrupt or queue with priority, loops, start time, early end, inertialized blend in/out, root-motion mode, events and warp windows ("Slots and montages") | **match** (G4 resolved 2026-10-12, M3-d). **Differ**: inertialization only (UE crossfades by default, inertialization per montage); no montage sections or branching points yet; a queue (OpenMW `playQueued`, Morrowind PlayGroup flag 0) that UE leaves to gameplay code |
| Motion matching | UE Pose Search (production-ready 5.4; GASP hybrid with Chooser); Ubisoft (Büttner, nucl.ai 2015; Clavet, *For Honor*, 2016); Naughty Dog (TLOU2); Unity Kinematica (preview only, last release 0.8.0-preview, 2020-07). Decima: no public source found that Guerrilla uses MM (Oud's GDC 2022 Horizon talk abstract doesn't say) | later node, data plan in place | **differ, justified**: legacy content is a few hand-keyed cycles |
| Physical animation | RAGE + Euphoria (GTA IV onward, evolved for RDR2; NaturalMotion stopped licensing it in 2017); Havok powered ragdolls; UE Physics Control | M4 Layer 4 | **match** (planned) |
| Update-rate LOD | UE URO: per-LOD frame skip (`LODToFrameSkipMap`), a non-rendered update rate, interpolation over skipped frames. Unity `cullingMode`: `AlwaysAnimate`, `CullUpdateTransforms` (no retarget/IK/write while invisible), `CullCompletely`. Creation Engine process levels High / Middle-high / Middle-low / Low (high = every frame; lower levels much less often; documented for Oblivion and Fallout 3). OpenMW: "actors processing range" (default 7168 units, min 3584): AI, animation and physics stop beyond it, with a fade at the edge | A0–A3 by projected size × significance, A2 at 1/2–1/4 rate interpolated, A3 clocks + events only | **match** |
| Budget allocator | UE Animation Budget Allocator: `a.Budget.BudgetMs` 1.0 ms game thread by default (scaled per quality level up to 2.5), tick rate up to `MaxTickRate` 10, interpolation capped (`MaxInterpolatedComponents` 16), `MaxTickedOffscreen` 4, `AlwaysTick` with a falloff, significance per component (`bNeverSkip`, `bTickEvenIfNotRendered`, `bForceInterpolate`) | allocator demotes by tier until an EMA cost fits, hysteresis | **match**; worth borrowing UE's caps (max interpolated, max off-screen ticked) as allocator parameters |
| **Crowd pose sharing** | UE Animation Sharing plugin: crowd actors are grouped into animation-state buckets; one leader evaluates each bucket and every member copies its pose (leader pose), with per-member blends and randomized start offsets for variety. UE's Leader Pose Component (one evaluation drives several meshes) is the per-character version | every character evaluates its own graph; tiers only lower the rate | **GAP (P2)**: for A2 ambient NPCs playing the same idle or walk cycle, bucket by (skeleton, clip, quantized phase) and copy. Open question 8 |
| **Distant crowd tier** | Vertex or bone animation textures, instanced: UE AnimToTexture (VAT, or bone mode with bone position/rotation textures; secondhand: used for City Sample's medium- and far-distance pedestrians), Unity's Animation Instancing (2018, root motion, attachments and events, no transitions or layers), GPU Gems 3 ch. 2 (bone matrices in a texture). AC Unity: 40 real AIs, 120 high-resolution models, ~10,000 crowd NPCs, swapping low- and high-res NPCs unnoticed | "later tier" in LOD and budget | **GAP (P2)**, already named; now a job (industry-audit.md, M3 follow-ups) so R1 NPCs can move instead of freezing |
| Skinning path | UE: vertex-factory GPU skinning by default, plus the compute **Skin Cache** (skin once, reuse in every pass; mandatory for hardware ray tracing; per-world memory cap; per-LOD opt-in). Unity: GPU (compute) skinning option | Bevy 0.20 as is: skinning in the vertex shader of **every pass** (depth prepass, main, each shadow cascade; with motion vectors the prepass also skins with the previous joints). Joint palettes are copied **per `SkinnedMesh` entity** (`SkinUniforms` keyed by entity), allocated only while visible, and only joints whose `GlobalTransform` changed are rewritten | **differ** (open question 3); **GAP (P1)** in measurement: the "GPU skinning ≤ 0.5 ms all passes" gate has no per-pass breakdown, and nothing keeps characters out of far cascades. UE has a per-component `bCastFarShadow` (render into far directional cascades only when set; we found no source for its default). Job M3-c |
| Morph targets | UE: GPU morph targets ("Use GPU for computing morph targets"), per-LOD "Remap Morph Targets" and a morph position error tolerance for compression; UE's skeletal mesh **merge drops morph targets** (Epic's modular character chart) | Bevy 0.20: dense `MorphAttributes` (48 bytes per vertex per target, storage buffer), a loop over every target per vertex in every pass that skips zero weights, `MAX_MORPH_WEIGHTS` 256 | **GAP (P1)**: see character.md "Industry comparison" (morphed parts must not be merged) |
| Events | UE anim notifies and notify states; Havok clip annotations; OpenMW text keys | point and window events, highest-weighted clip fires | **match** |

Sources (checked 2026-10-12):
- UE Animation Budget Allocator: https://dev.epicgames.com/documentation/en-us/unreal-engine/animation-budget-allocator-in-unreal-engine
- UE `USkinnedMeshComponent` (URO: `bEnableUpdateRateOptimizations`, `AnimUpdateRateParams`): https://dev.epicgames.com/documentation/unreal-engine/API/Runtime/Engine/USkinnedMeshComponent ; `LODToFrameSkipMap` usage: https://forums.unrealengine.com/t/animation-frameskip-skeletalmesh/432638
- UE Animation Sharing plugin: https://dev.epicgames.com/documentation/unreal-engine/animation-sharing-plugin-in-unreal-engine (read via the ja/zh pages) ; API: https://dev.epicgames.com/documentation/unreal-engine/API/Plugins/AnimationSharing/UAnimationSharingManager
- UE Distance Matching: https://dev.epicgames.com/documentation/en-us/unreal-engine/distance-matching-in-unreal-engine ; `DistanceMatchToTarget`: https://dev.epicgames.com/documentation/unreal-engine/BlueprintAPI/DistanceMatching/DistanceMatchtoTarget
- UE Motion Warping: https://dev.epicgames.com/documentation/en-us/unreal-engine/motion-warping-in-unreal-engine
- UE Foot Placement node: https://dev.epicgames.com/documentation/unreal-engine/API/Plugins/AnimationWarpingRuntime/FAnimNode_FootPlacement ; pelvis modes: https://dev.epicgames.com/documentation/unreal-engine/API/Plugins/AnimationWarpingRuntime/EPelvisHeightMode
- UE skeletal mesh rendering paths (Skin Cache): https://dev.epicgames.com/documentation/en-us/unreal-engine/skeletal-mesh-rendering-paths-in-unreal-engine ; `cast_far_shadow`: https://dev.epicgames.com/documentation/en-us/unreal-engine/python-api/class/SkinnedMeshComponent
- UE modular characters (leader pose / copy pose / merge chart): https://dev.epicgames.com/documentation/en-us/unreal-engine/working-with-modular-characters-in-unreal-engine
- UE AnimToTexture: https://dev.epicgames.com/documentation/unreal-engine/API/Plugins/AnimToTextureEditor/UAnimToTextureBPLibrary
- Unity `AnimatorCullingMode`: https://docs.unity3d.com/ScriptReference/AnimatorCullingMode.html ; Animation Rigging constraints: https://docs.unity3d.com/Packages/com.unity.animation.rigging@1.2/manual/ConstraintComponents.html ; Kinematica changelog: https://docs.unity3d.com/Packages/com.unity.kinematica@0.8/changelog/CHANGELOG.html ; Animation Instancing: https://blog.unity.com/engine-platform/animation-instancing-for-skinnedmeshrenderer
- GPU Gems 3 ch. 2, *Animated Crowd Rendering*: https://developer.nvidia.com/gpugems/GPUGems3/gpugems3_ch02.html
- Cournoyer, *Massive Crowd on Assassin's Creed Unity: AI Recycling*, GDC 2015: https://gdcvault.com/play/1022141/Massive-Crowd-on-Assassin-s
- Clavet / Büttner motion matching history: https://gamedeveloper.com/programming/most-inspiring-game-animation-tech-talks-of-2016
- Oud, *Evolving the Horizon Series*, GDC 2022 (abstract): https://gdcvault.com/play/1027894/Evolving-the-Horizon-Series-Animating
- Euphoria: https://en.wikipedia.org/wiki/Euphoria_(software) ; RDR2 interview: https://wccftech.com/rockstar-euphoria-evolved-rdr2/amp/
- Creation Engine process levels: https://cs.uesp.net/wiki/GetProcessLevel ; https://geck.uesp.net/wiki/GetActorProcessingLevel ; Skyrim behavior files (community tooling): https://github.com/Monitor221hz/Pandora-Behaviour-Engine-Plus
- UE montages (sections, slots, blend in/out, blend-out trigger time, interruption uses the new montage's blend-in): https://dev.epicgames.com/documentation/en-us/unreal-engine/animation-montage-editor-in-unreal-engine ; `UAnimMontage` (`blend_mode_in/out`, group from the first slot): https://dev.epicgames.com/documentation/en-us/unreal-engine/python-api/class/AnimMontage
- Morrowind PlayGroup / LoopGroup flags: https://en.uesp.net/wiki/Morrowind_Mod:PlayGroup ; https://en.uesp.net/wiki/Morrowind_Mod:LoopGroup ; OpenMW LoopGroup count issue: https://gitlab.com/OpenMW/openmw/-/issues/7641 ; OpenMW Lua `playQueued` / `playBlended`: https://openmw-trav.readthedocs.io/en/latest/reference/lua-scripting/openmw_animation.html
- Animancer layer `Play` (fade in the state, fade out the layer's others): https://kybernetik.com.au/animancer/api/Animancer/AnimancerLayer/A27E17D4
- OpenMW game settings: https://openmw.readthedocs.io/en/latest/reference/modding/settings/game.html ; animation blending: https://openmw-trav.readthedocs.io/en/latest/reference/modding/animation-blending.html ; 0.49 release: https://openmw.org/2025/openmw-0-49-0-released/

**Unverified** (from memory or secondhand, check before quoting details):
City Sample's use of AnimToTexture for mid/far pedestrians (a secondhand blog,
https://teletype.in/@hardway/anim2texture); UE5 Pose Search production-ready
in 5.4 and GASP 5.8's Chooser (carried from "Next-gen roadmap"); Skyrim foot
IK inside Havok Behavior; `bCastFarShadow`'s default value.

