# Animation layers: warping, IK, probes, physical animation

Moved verbatim from [animation.md](animation.md) "Next-gen roadmap" (doc cleanup S7a, 2026-10-25). Current design for layers 1-4.
Work order, slice status and owner-pass issues (AN-*) live in [animation-plan.md](animation-plan.md) (kept separate: live status log, not design; reconciled S7b).

## Next-gen roadmap (anim-plan, 2026-10-11; shape replaced by R11b, 2026-10-19)

The target, in the owner's words: next-gen like Assassin's Creed or Elden
Ring / Dark Souls 3; IK is mandatory; characters react to their
environment; the system works for creatures. Each layer below names its
canonical technique, its interface against the crates as built, its
milestone, and measurable criteria.

### Decided shape (R11b, 2026-10-19)

The generic "authored backbone" shape written here from READMEs (2026-10-11) is replaced by the
animation study (`../research/animation/`, notes R1-R10) and the owner's decisions
(`11-tradeoffs.md` "Decisions"). **Read [action.md](action.md) and [camera.md](camera.md) first.**
In one line: a Rust **action layer** (`sf_action`) decides every move from data rules on the fixed
tick; the graph only plays and blends; typed clip windows gate when a choice is allowed (the
FromSoftware split, R5 §1). Gameplay reads baked clip data on the action clock, never the evaluated
pose, so the graph, IK, first-person bodies and camera are presentation (net-ready for co-op).
Motion matching stays an optional locomotion node ("Later" below).

What changes in this spec (each is built in the action.md phase named):

1. **`Progress` root mode** (phase B1; R2 adopt 1). A fourth mode beside `Drive`, `Match`,
   `Ignore`: the player's playhead is set each update from a parameter (seconds or normalised,
   either direction), as UE's sequence evaluator; its root output is not applied to the character.
   Every move the action layer starts plays this way, from `ActionOutput`'s playhead, so the graph
   never drifts from the gameplay clock. `Drive` stays for hosts without an action layer (viewers,
   scripted one-offs).
2. **Chain warping** (phase B1; R3 §5, §11). Layer 1 below is single-clip. Added: a warp across a
   chain of clips (the correction carried by the clip with the most root motion), correction
   profiles `Linear` and `Late` (cubic: rise first, over the lip late), a **refusal ratio** (correction
   / root distance) the action layer checks before starting, variant mixing before warping, an
   `arc` mode (ballistic flight with a clip-given arrival time, R5 adopt 8), and `mode=scale` with
   min/max clamps and a stand-off for attack lunges (exempt from the 2 cm check; R6 adapt). The warp
   function stays Bevy-free in `sf_anim_graph` and is called by both the graph (presentation) and
   `sf_action` (gameplay root). **Built (B1, 2026-10-26)** as `sf_anim::WarpPlan` / `RootChain`
   (moved to `sf_anim` so `sf_action` needs no `sf_anim_graph` edge) with `Linear`/`Late` and the
   refusal ratio; items 1 and 2 as built: action.md "As built (B1)". Not yet: variant mixing,
   `arc`, `mode=scale` lunges.
3. **Typed windows and window-gated interruption** (phases B2, E1; R5 adopt 1, R6 adopt 2).
   `animgraph.compile` parses clip `Event` windows into the typed vocabulary of action.md
   "Windows" (accept, cancel per action class, tail, turn, lunge, contact, i-frames, hit, camera...).
   Slot/montage requests carry an action class; an interrupt is allowed only inside a `cancel`
   window for that class or on an unrestricted clip (today: priority only). Priority stays the
   tie-break.
4. **Gap-sized inertial blends and pose-matched entry** (phase B2; R4 adopt 1-2). Transitions get
   `duration: Auto`: the inertialization length from the largest hand/foot gap between the current
   pose and the new source's entry frame, clamped ~0.05-0.3 s; combat cancels default to ~0.1 s.
   A `Player` start mode picks the entry frame (cycle phase and leading foot) by leg-joint match.
   Root impulses due on a clip's first frame apply on the start tick (R8 §1.1).
5. **`clip.annotate` bake step** (phase B2 minimal, E1 full; R9 §6). On the retargeted clip it
   writes: reconstructed root motion for in-place one-shots, foot **and hand** contacts, markers,
   leading foot, phase, turn windows, flight phases, end pose and effector spacing, a
   contact-bounded warp window, the weapon/hand socket track (root space), the **virtual camera
   track** (camera.md), IK weight hints, reach and first-contact proposals. Category defaults come
   from the clip's slot; a moveset RON sidecar per moveset holds the design numbers (about one line
   per attack slot). It replaces `ClipRules::events` as the authoring point. Lints: unconfirmed hit
   windows, warp residual over the refusal ratio, missing hand contacts on traversal slots,
   reconstructed roots, seam/jolt checks, tempo outliers.
6. **First-person render bodies** (phase D2; R7 §2.1, R8 §3). One gameplay body; a first-person
   render body (arms, legs when looking down, own FOV and scale, head hidden) plus the world body
   for shadows. Per action: arms layer (a content pack's own first-person clips allowed as
   presentation on the shared clock) or full body with the baked camera track. Hits always come
   from the world body's baked socket tracks. character.md's "view-model arms plus shadow body"
   section is superseded by this model and is rewritten in D2.

Also from the study, smaller: IK policy as data (ramps, gating, latching, proximity weights;
R3 adopt 5, R4 adopt 4), `ArmClearance` and reach assist (R4 adopt 5, 8), hit-stop as a
per-character clock scale shrinking with crowd size (R6 adopt 4, R8 adopt 5), large-turn
inertialization (R4 adopt 3). Each lands with the phase that first needs it.

**Kept: our ozz-model core and `sf_anim_graph`.** Re-checked 2026-10-11:
`ozz-animation-rs` 0.11.0 (2025-10) is MPL-2.0 and nightly-only, and would
duplicate `sf_anim` (unchanged on 2026-10-18; see "Prior art" under Layer 3). `bevy_animation_graph` 0.11 is still on Bevy 0.19.
NVIDIA **MotionBricks** (in `NVlabs/GR00T-WholeBodyControl`, "preview
release"): code is Apache-2.0 and weights are under the NVIDIA Open Model
License, but the released checkpoints drive a Unitree G1 robot from SMPL
references, inside a robotics stack. Watch item only: at most an offline
generator of coverage clips, retargeted G1 → humanoid at bake, if a human
checkpoint appears.

### Layer 1: motion warping (M3)

**Technique.** Witkin & Popović, *Motion Warping*, SIGGRAPH 1995, as UE5
productized it (Motion Warping plugin): a clip carries **warp windows**
(UE notify states) naming a **warp target**; during the window the
remaining authored root motion is transformed so the character arrives
at the target's position and facing at the window's end. Modes: **skew
warp** (translation of the remaining path skewed onto the target, the UE
default), **scale** (uniform scale of root translation), and rotation
`Default` (match the target's facing) or `Facing` (face the target), with
a rotation time multiplier. Uses: attacks that track and snap to an enemy
(DS3-style tracking limited by a turn rate), vaults, mantles, ledge
grabs, sitting, door and lever interactions.

**Interface.**
- Graph asset: `warp_targets: ["enemy", "ledge", "seat"]`, slots like
  `ik_targets`. Windows are clip events named `warp` (an `Event` window,
  `payload` = `"<target>;<mode>[;key=value]"`, keys `rot=default|facing|none`,
  `y=ignore|warp`, `rot_time=0.5`, `max_dist=1.5`, `max_turn=360`
  (deg/s), `point=root|<joint>`), authored in the adapter's
  `ClipRules::events` or the animation-sources manifest.
  `animgraph.compile` parses them into `WarpWindow`s per (clip slot,
  event) and precomputes the window's authored root delta and the warp
  point (root, or a joint's position at the window end: UE's "warp point
  anim provider: bone").
- Runtime: `UpdateInputs` gains `warp_targets: &[Option<WarpTarget>]` with
  `WarpTarget { position: Vec3, yaw: f32 }` in character space, refreshed
  every update (moving targets are tracked). After the root delta is
  blended (`GraphOutput::root`), the highest-weighted `Drive` player with
  an open window and a set target has its delta replaced by
  `warp(delta, clip.root_delta(t, end), target)`; `GraphOutput` gains
  `warp: Option<ActiveWarp { target, remaining, error }>` for gameplay.
- Mover: today vertical root motion is dropped while grounded
  (`sf_controller/src/mover.rs:251`). A window with `y=warp` sets a
  `Traversal` flag on `MoveInput` that suspends ground snap and gravity for
  its duration (UE puts the character in flying mode during a mantle
  montage), then re-grounds with the normal probe.

**Verification.** Static target, unobstructed: position error at the
window end ≤ 2 cm and facing error ≤ 2° (`ActiveWarp::error`); a target
moving at 3 m/s: ≤ 5 cm. No root speed jump > 20% between the frames
around the window's start. Cost ≤ 0.5 µs per warping character.

### Layer 2: IK, from limbs to full body (M3)

**Technique.** We have two-bone (analytic), aim chains, FABRIK and CCD
(`sf_anim::ik`). Missing for "react to the environment": solving several
effectors at once while the pelvis and spine give way (a hand on a ledge
while the feet stay planted; four feet and a head on a quadruped). UE5's
**Full Body IK** is a position-based solver (`PBIK`): one particle per
joint, bone-length and joint-limit constraints, weighted effectors with
per-joint stiffness and a root that pre-pulls toward the effectors, 20
iterations by default, rotations recovered from the solved positions.
Our solver is the same shape with **XPBD compliance** (Macklin, Müller,
Chentanez, *XPBD*, MIG 2016; Müller et al., *Position Based Dynamics*,
2007), so stiffness doesn't depend on the iteration count; survey:
Aristidou et al., *Inverse Kinematics Techniques in Computer Graphics*,
CGF 2018. Bereznyak's *IK Rig* (GDC 2016) is the model for driving it from
a few goals.

**Interface.**
- `sf_anim::ik::FullBody` (Bevy-free): built once per (skeleton, profile)
  from the rig profile: particles for the girdles, spine, neck, head and
  limb joints; distance constraints; hinge limits for knees and elbows,
  cone-twist for hips and shoulders. `solve(&mut LocalPose, &mut ModelPose,
  effectors: &[Effector], iterations)` with `Effector { joint, position,
  rotation: Option<Quat>, weight, pull }`. Twist is preserved by
  reapplying the animated twist about each solved bone.
- New ops: `LimbIk` (replaces `HandIk`: any limb or chain, position plus
  optional rotation target, solver picked by the limb), `ChainIk` (FABRIK
  or CCD over a named chain: tails, necks, tentacles), `FullBodyIk`
  (effectors from IK target slots and from `FootPlacement`'s foot targets,
  when it runs after it). `ik_targets` become `Option<IkTarget { position,
  rotation: Option<Quat> }>` (rotation is needed for hands on ledges and
  handles).
- `Lean` and `SpineBend` ops (procedural, from `LocomotionParams`
  acceleration and turn rate): bipeds lean into turns, quadrupeds bend the
  spine with the head leading (Frey, GDC 2016).
- Grade-aware gait (AN-21, as built): `StrideWarp.grade` and `Lean.grade` read the graph's filtered
  ground grade (`GraphOutput::grade`, rise over run along the travel, from the probes). Biomechanics:
  at a fixed speed people shorten their steps and raise the cadence uphill and lean the trunk forward a
  few degrees (backward slightly downhill); UE/Lyra cover slopes only with foot placement, so the stride
  and lean terms are ours. Lean joints must sit above the leg branch (biped thighs hang off the spine).

**Verification.** Reachable effectors within 1 cm after 10 iterations;
unreachable ones clamp without flips; joint limits never exceeded;
identical results on 1 and 8 threads. `FullBodyIk` ≤ 8 µs on a 30-joint
profile at 10 iterations (A0 only; the budget allocator counts it), so a
worst-case A0 character stays ≤ 35 µs against the 25 µs typical gate.

### Layer 3: context probes and interaction points (M3 probes, M4 actions)

**Technique.** Environment-aware animation runs on a few cheap scene
queries per character, classified into affordances, as GASP's traversal
does (forward traces, obstacle height and depth classified into
vault/hurdle/mantle, a chooser picks the clip, motion warping aligns it)
and as AC III's navigation rework did with procedural adjustment (Ubisoft
Montreal, *AI Postmortems: Assassin's Creed III…*, GDC 2013). Authored
**interaction points** (UE Smart Objects: slots with a transform, an
animation tag and a claim) cover benches, doors, levers and counters.

**Interface** (`sf_controller`, locomotion.md "Context probes"):
batched queries on the same `GroundProbe`/`StaticWorld` path as ground
probes: obstacle ahead along the trajectory (height, depth, top point,
normal), ledge (edge point, normal, clearance above), drop ahead, wall
beside (for a hand on the wall), ceiling (crouch). Results become graph
parameters (`obstacle_height`…) and warp or IK targets. Interaction points
are `sf_interaction` data (M4). Look-at targets come from a cheap
"points of interest" scan (nearest characters and interaction points in a
cone) until `sf_ai` perception (M10) provides them.

**Verification.** Batched results equal single queries; classification
table tests on boxes 0.3–2.5 m high; probes stay inside the 0.2 ms probe
budget for the 150-NPC scenario (context probes only at A0, at most 4 per
character per update, and only when the trajectory is blocked).

**As built** (probes, 2026-10-12): `sf_controller::ContextProbes` answers all five queries in ≤ 4
batched probe stages and returns `ContextHit` data (affordance, lip, face normal, height, depth,
clearance, landing) with `warp_target`/`hang_point` for warp windows; vault/mantle actions stay M4.
Classification and timing: locomotion.md "Context probes as built" (150 all-blocked: 0.79 ms
single-thread, inside the budget only with the gating above).

#### Prior art: open Rust/Bevy game reimplementations (owner, 2026-10-18; superseded 2026-10-19)

**Superseded by the animation study.** These projects were read in detail in
`../research/animation/` (faith-runner R1-R2, AC1-rs R3-R4, sekiro-rs R5, ELDEN-RING-Combat-Rewrite
and sekiro-rs combat R6, Mudcrab R10), which also corrects claims the old summary made (R2: faith-runner
has wall-arm IK; R3: jump targets are nearest-first except hangs, and AC1's chooser is code +
tables; R4: AC1's IK policy is worth adopting; R5: sekiro-rs is the best open description of the
FromSoftware split). The traversal layer is now [action.md](action.md); the clip question is
answered there ("Clip gaps") and in R9. The README-level summary that stood here was removed (git history
before 2026-10-19). Still true from it: ozz-animation (C++, MIT) and `ozz-animation-rs` 0.11 (MPL-2.0,
nightly SIMD) are not adopted; they would duplicate `sf_anim` and have no graphs or traversal.

### Layer 4: physical animation (M4)

**Technique.** A **powered ragdoll** follows the animated pose through
per-body drives, and gameplay lowers the drive strength per body region
to react: hit reactions, stumbles, knockdowns, death, then a get-up blend
back to animation. Industry: NaturalMotion Euphoria (GTA IV, Red Dead),
Havok's "powered ragdoll", Frostbite (Sachania, *Physics Driven Ragdolls
and Animation at EA: From Sports to Star Wars*, GDC 2018), UE5's Physical
Animation Component and its successor the Physics Control Component
(GASP 5.8 examples). Drives are **stable PD controllers** (Tan, Liu, Turk,
*Stable Proportional-Derivative Controllers*, IEEE CG&A 2011), so high
gains don't explode at our fixed step. Learned physics controllers
(DReCon, Bergamin et al., SIGGRAPH Asia 2019; SuperTrack, Fussell et al.,
2021; both Ubisoft La Forge) are the research frontier, not a plan.

**Interface.**
- `ragdoll.cook` (bake): bodies and joints from the rig profile (capsules
  sized from bone length and skin-weight extents, masses by volume,
  limits by limb kind); adapter or mod override files.
- `sf_ragdoll` (Bevy-free core, M4): drive targets from the graph's model
  pose; per-region strength profiles (`Kinematic`, `Powered(strength)`,
  `Limp`); hit impulse + strength dip + recovery curve; balance check
  (center of mass against the support polygon of `support` limbs) that
  triggers stumble steps or a fall; get-up selection (face up/down from the
  pelvis axis) and the hand-back: the ragdoll pose enters the graph through
  the existing `PoseInput` node and inertializes into the get-up clip.
- Drives are applied as torques/forces from the PD law each physics step,
  which works on either engine: avian3d 0.7 has motors only on revolute
  and prismatic joints (no spherical motor), Rapier has per-axis motors on
  generic joints. Decided 2026-10-12: rapier3d, whose implicit joint motors are used directly (physics.md "Ragdolls and the animation blend").
- Kinematic hit reactions before physics (M3/M4): additive directional hit
  clips (the graph's additive layer) plus a `HitReact` op that kicks
  critically damped springs on the hit region's joints (`sf_anim::spring`).
- Secondary motion: `SpringBones` (Verlet chains with spring constraints)
  for tails, ears, antennae and hair; cloth stays WISHLIST §7.

**As built (P9, 2026-10-13; `sf_anim_graph::secondary`).** Reuse check
(rule 8): Bevy 0.20 has no spring bones; `bevy_skeleton_modifiers` 0.1.1
works on entities after Bevy's animation, outside our pose-buffer op stack
(no op order with foot placement, no LOD, no graph state), and nothing else
current fits; the technique is small (the Unity Dynamic Bone / VRM
`springBone` / UE `AnimDynamics` family), so both ops are ours.
- `HitReact { joints, depth 3, falloff 0.5, gain 0.35 rad, max_angle 30°,
  halflife 0.12 s }`: `GraphInstance::hit(Hit { point, direction, strength,
  joint })` (character space) queues a hit; at the next update the hit joint
  (or the reactive joint nearest the point) and up to `depth − 1` reactive
  ancestors get an angular-velocity kick about `lever × direction`, sized so
  the hit joint peaks at `gain · strength` after `halflife / (2 ln 2)`
  (x(t) = v0·t·e^(−yt)). The springs are Holden's closed form, so the curve
  is the same at any frame rate (tested against the analytic curve at 30,
  60 and 144 fps) and the pose returns exactly to the animation. Default
  joints: the rig profile's spine (without the root girdle, which would
  swing the support legs), neck, head and non-support limbs. Runs after
  foot placement (feet stay planted). Additive directional hit clips stay
  possible on the graph's additive layer.
- `SpringBones { chains, explicit, frequency 3 Hz, damping 0.6, gravity 0,
  inertia 0.5, max_angle 60° }`: each chain joint's child is a Verlet
  particle pulled toward its animated offset from the simulated parent (a
  damped spring, ω² pull and 2ζω drag, as frequency and damping ratio, not
  per-step factors), kept at bone length and within `max_angle` of the
  animated direction; joints are then aimed at their particles root first.
  Particles live in character space; the character's motion since the last
  update (`UpdateInputs::moved`) moves them back by `inertia` (1: they stay
  put in the world). Fixed 1/120 s substeps accumulated across updates (at
  most 12 per update) make it frame-rate independent: a strafing guar's
  tail trails, swings and settles the same at 30, 60 and 144 fps (7 fps:
  the same rest), with no drift; gravity sag is equal at every rate.
  `chains` names rig-profile chains (empty: every tail). An LOD cut resets
  both ops (springs to zero, particles re-seeded from the animation). The
  Morrowind graphs (humanoid, creature) carry both ops with defaults.
  `ClothLattice` (P10) extends the particle solver with lattice constraints.

**Verification.** Powered at full strength, standing and walking: tracking
error ≤ 5 cm RMS at the effectors and ≤ 5° per joint. Get-up: joint
angular velocity continuous across the hand-back (inertialization test at
shrinking dt, as for transitions). Cost: ≤ 0.05 ms physics per powered
character, budget of 8 powered characters at once (others go limp or stay
kinematic). Visual: knock a guard down a staircase, he gets up on the
stairs with feet placed.

### Later: pose search node (after M4, once a library is baked)

**Technique.** Motion matching (Clavet, GDC 2016; Zadziuk, *Motion
Matching, The Future of Games Animation… Today*, GDC 2016; Mach &
Zhuravlov, GDC 2021; UE5 Pose Search, production-ready in 5.4; Holden et
al., *Learned Motion Matching*, SIGGRAPH 2020, for memory; Holden's
*Dead Blending*, 2023, for transitions).

**Interface.** `NodeKind::PoseSearch { database: u16, interval, bias }`:
a `posesearch.cook` bake product per (graph, skeleton) with normalized
features (trajectory positions and facings at +0.33/0.67/1.0 s from
`sf_controller`'s trajectory, which gains history samples; foot positions
and velocities; hip velocity), searched with Holden's bounding-box
acceleration every 0.1 s or when the trajectory changes; switching uses
the existing inertialization. It sits in a state machine like any node, so
combat and creatures keep authored states.

**Verification.** ≤ 15 µs per search over 100k frames (≤ 3 µs per frame
amortized), foot slide ≤ 2 cm/s with the database's own contact curves,
trajectory tracking error ≤ 15 cm at +1 s.

