# Animation: clips, pose runtime, graph, IK, LOD

Status: draft (2026-10-11); `sf_anim` and `sf_anim_graph` built; next-gen
roadmap (warping, full-body IK, context probes, physical animation, pose
search, creatures) drafted, not built; R11b (2026-10-19): the decider moved to
[action.md](action.md), see "Next-gen roadmap: Decided shape (R11b)" for what changes here
Slice plan and execution order (2026-10-22): [animation-plan.md](animation-plan.md).
Crates: `sf_anim` (new, Bevy-free: skeletons, cooked clips, sampling, pose
ops, root motion, events and curves, IK solvers, retarget math, bake steps),
`sf_anim_graph` (new, Bevy-free: graph assets, layers, state machines, blend
spaces, sync, inertialization, procedural op stack), `sf_anim_bevy` (new:
plugin, scheduling, LOD and budget, bone write-back, events, debug draw)

## Goal

Characters and creatures animate with modern quality and at crowd scale:
layered graphs with masks and additive layers, blend spaces, root motion,
events and curves that gameplay can key off, and a procedural pass (foot
placement, look-at, hand IK, stride and orientation warping) on top. The
cost is bounded per frame by animation LOD and a budget. Everything that
can be precomputed is a bake product, so the runtime never parses a legacy
format and never resamples a source curve.

Skeleton conventions (bone names, axes, which bone carries root motion)
come from adapters through data. Nothing here assumes a particular rig;
humanoid-specific features go through the standard rig (character.md).

Non-goals for M3's first slice, though the design leaves room for them:
motion matching (below), physics poses (powered ragdolls, hit reactions in
M4), combat windows (M9 uses the event tracks defined here), and
procedural cloth and hair (M4). Where each next-gen layer lands (motion
warping, full-body IK, context probes, physical animation, pose search,
creatures) is in "Next-gen roadmap" below; limb chains for any skeleton
are in [rig-profile.md](rig-profile.md).

## Canonical technique

- **Runtime shape: ozz-animation's job model** (Guillaume Blanc, MIT,
  guillaumeblanc.github.io/ozz-animation). Clips are sampled into
  contiguous **local-space pose buffers**. Blending, masking and additive
  ops work on whole buffers, then a local-to-model pass runs. IK runs on
  the model-space buffer, and the result is written to the engine once.
  The graph decides *what* to sample and how to mix; the pose code is a
  handful of tight loops.
- **Graph: layered blend trees and state machines** (UE5 Animation
  Blueprints, Unity Mecanim, Esoterica by Bobby Anguelov, MIT). Per-bone
  **blend masks** with weights (UE "blend profiles"), not on/off bits.
  **Sync groups** by normalized phase, refined by **sync markers** (UE).
- **Transitions: inertialization** (David Bollo, *Inertialization:
  High-Performance Animation Transitions in Gears of War*, GDC 2018). At a
  switch, the offset between the last output pose and the new source
  decays over the blend time with a per-bone quintic. Only the new source
  is evaluated, so a transition costs about one pose, not two. A plain
  crossfade stays available for layer weights and blend nodes.
- **Blend spaces**: 1D, and 2D over a Delaunay triangulation of the sample
  points (barycentric weights, at most 3 clips active; UE's approach).
  Gradient-band interpolation (Johansen, *Automated Semi-Procedural
  Animation for Character Locomotion*, 2009, used by Unity's freeform blend
  trees) is smoother but blends N clips. We take the cheaper one and add
  gradient bands only if seams show.
- **Root motion**: the root bone's motion is extracted at bake time into a
  separate track (UE, Unity). Each clip chooses how it moves the character
  (see Root motion below).
- **Locomotion fidelity without a huge clip set**: playback-rate matching
  plus **stride warping** and **orientation warping** (UE Animation
  Warping, from Paragon and Fortnite), **foot placement** with pelvis
  adjustment and foot locking (Bereznyak, *IK Rig: Procedural Pose
  Animation*, GDC 2016; Johansen 2009), and contact curves detected at bake.
- **IK solvers**: analytic two-bone with a pole vector (law of cosines;
  Holden, *Simple Two Joint IK*, 2017), aim/look-at chains with per-joint
  weights and limits, FABRIK for longer chains (Aristidou & Lasenby,
  *FABRIK*, Graphical Models 73(5), 2011), CCD as a fallback (Wang & Chen,
  1991). Targets are smoothed with critically damped springs (Holden,
  *Spring-It-On*, 2021).
- **Crowd cost: significance-based LOD and a budget** (UE Animation Budget
  Allocator and Update Rate Optimization). Throttled characters interpolate
  between updates; off-screen characters advance clocks only.
- **Clip storage: uniform sampling with quantized tracks** (the ACL design,
  Nicholas Frechette, MIT, Unreal's default codec). It needs no key search,
  so sampling is two frame reads and a lerp. Error is measured on virtual
  vertices a fixed distance from each bone, as ACL does.

### Why not Bevy's animation system (deviation from ARCHITECTURE §7.6)

Bevy 0.20's `bevy_animation` (checked 2026-10-11) has `AnimationGraph` with
clip, blend and add nodes, masks as on/off bits over mask groups,
`AnimationTransitions` crossfades, and `AnimationEvent` observers. The
0.17–0.20 release notes add nothing to animation itself; 0.19 improved
skinned-mesh culling (bounds from joints) and morph-target batching. It
lacks what M3 needs:
1. **No root motion** extraction or modes.
2. **No hook between blending and write-back.** IK must run on the blended
   pose in model space. With Bevy, the graph writes `Transform`s, so IK would
   have to run after transform propagation and then re-propagate.
3. **Per-target evaluation.** `animate_targets` walks each bone entity's
   subgraph (`ThreadedAnimationSubgraph`) and samples dynamically
   dispatched curves per bone. A pose-buffer runtime samples a whole
   skeleton in one loop.
4. No weighted masks, sync groups or markers, inertialization, blend
   spaces, curve outputs, or animation LOD.

So characters use our runtime. Bevy still renders: GPU skinning, morph
targets, culling, and bone entities stay Bevy's. `AnimationClip` remains
fine for UI or glTF props that need nothing above.

**Crates checked (2026-10-11):**

| Crate | Version / Bevy | License | Verdict |
|---|---|---|---|
| `bevy_animation_graph` | 0.11.0 (2026-07-11), Bevy 0.19 | MIT/Apache | Closest fit: graph assets, state machines, 2D blend space, two-bone IK, partial ragdolls, visual editor. But it lags a Bevy version, says "young, expect bugs, API churn", ties nodes to Bevy reflection, and doesn't mention root motion. We take ideas and revisit its editor as an authoring tool. |
| `bevy_mod_inverse_kinematics` | 0.12.0 (2026-09-01), Bevy 0.19 | MIT/Apache | Maintained, so ARCHITECTURE §4's "IK crates are stale" is out of date. It solves on entity `Transform`s after propagation, which is the pattern we avoid. |
| `bevy_fabrik` 0.2.1 (2026-04), `bevy_skeleton_modifiers` 0.1.1 (2026-10-09, MIT, spring bones + IK) | — | MIT/Apache | Entity-based and new. Spring bones are a candidate for hair and tails before cloth (M4). |
| `ozz-animation-rs` | 0.11.0 (2025-10) | **MPL-2.0**, nightly-only SIMD | Not usable: license outside our list, and it needs nightly. The design reference is the MIT C++ original. |
| `bevy_motion_matching` (voxell-tech) | app, not a library | MIT/Apache | Reference only. |
| ACL (C++) | — | MIT | No Rust binding. We implement the uniform-sampling subset. |

The solvers are textbook and each is under 200 lines on our pose buffers,
so writing them ourselves is cheaper than adapting entity-based crates.

## Data model

**Skeleton** (bake product `skeleton.cook`, per skeleton source):
`Skeleton { joints: [name, parent], rest: [Trs], root_motion_joint,
lod: [JointLodMask; 4], tags }`. Joints are depth-first ordered, so a
parent always precedes its children and local-to-model is one forward
loop. `tags` are adapter-provided (foot, toe, hand, head, eye, twist…) and
humanoid-mapped through the rig (character.md). Joint LOD masks list the
joints evaluated at each LOD. They are derived at bake from skin-weight
coverage and chain depth (fingers, face, twist joints drop first), with
adapter overrides.

**Clip** (bake product `anim.cook`, per (source clip, target skeleton)):

```rust
pub struct Clip {
    pub skeleton: SkeletonId,     // fingerprint of the skeleton it is bound to
    pub duration: f32, pub rate: f32,              // uniform sample rate (≥ 30 Hz)
    pub tracks: TrackTable,       // per joint: rot/trans/scale = Rest | Constant | Animated(range, offset)
    pub frames: Bytes,            // frame-major, quantized: rotations 48-bit smallest-three,
                                  // translations 16-bit per axis over per-track ranges
    pub root_motion: Option<RootMotionTrack>,      // extracted, see below
    pub curves: Vec<CurveTrack>,  // named f32 curves (contact.l/r, user curves), and morph weights
    pub events: Vec<Event>,       // sorted by time
    pub markers: Vec<SyncMarker>, // e.g. foot-down l/r, at bake from contacts or authored
    pub loop_range: Option<(f32, f32)>,
    pub meta: ClipMeta,           // cycle distance, average speed and heading, additive base
}
pub struct Event { pub time: f32, pub end: Option<f32>, pub name: EventId, pub payload: SmallStr }
```

The rate and quantization are chosen per clip at bake, as the coarsest
that keeps virtual-vertex error under 0.1 mm at 1 m. Additive clips are
stored as deltas from their declared base pose (first frame, a pose, or a
reference clip), computed at bake.

**As implemented in `sf_anim` (2026-10-11).** These points differ from the
sketch above on purpose:
- Bit widths are per track, not fixed. Each animated rotation, translation
  or scale track gets the fewest bits (1–24 per component) that meet its
  share of the error budget, or raw f32. Rotations drop whichever of the
  four components stays farthest from zero (ACL always drops W), so tracks
  swinging through 180° still quantize. Frames are bit-packed records,
  frame-major. Channels that never move live in a per-clip constant pose.
- The rotation-error metric is `2·atan2(|v|, |w|)` of the relative
  rotation. The usual `2·acos(|a·b|)` reads every angle below about 3e-4
  rad as zero in f32, so a quantizer that uses it picks too few bits, and
  the measured error then stays near 1 mm at any rate.
- Additive bases are `FirstFrame`, `Rest`, or the clip's own pose at a time.
  A reference-clip base is left to `animgraph.compile`.
- Joint LOD masks: three masks (LOD0–2; A3 evaluates none), derived from
  joint tags (fingers, face, eyes, jaw, twist and end joints drop at LOD1,
  toes at LOD2) plus an adapter `keep` list. Skin-weight coverage isn't
  used yet: `sf_model` has no skin data until its M3 item lands.
- `LocalPose` is SoA four joints wide from the start (open question 2): a
  pose op is a few 4-wide multiply-adds per joint group.

**Pose** (runtime, per character): `LocalPose { trs: Vec<Trs> }` and
`ModelPose { mats: Vec<Affine3A> }` from a per-thread pool, plus curve
values. The character keeps two past output poses for inertialization.
That's about 5 KB for a 60-joint skeleton.

**Graph asset** (`*.animgraph.ron`, compiled at bake per skeleton by
`animgraph.compile`):
- **Parameters**: typed blackboard (`f32`, `bool`, `Vec2`, trigger, event
  flags), written by gameplay, AI and scripts.
- **Layers**, evaluated bottom-up: `{ graph, mode: Override | Additive,
  mask: BlendMask, weight: Param | const }`. `BlendMask` is per-joint
  weights over the skeleton, authored as rig-relative rules ("spine and
  above", "left arm") and resolved per skeleton at compile.
- **Nodes**: `Clip` (rate, loop, start phase), `BlendSpace1D`,
  `BlendSpace2D` (triangulated at compile), `Blend` (by param or bool),
  `Additive`, `LayeredPerBone`, `StateMachine` (states, prioritized
  transitions with conditions on params, events, clip phase or time left,
  `Inertialize(t)` or `Crossfade(t)`), `Sync` (group id, leader by weight
  or forced, markers or phase), `Mirror` (by rig left/right map),
  `RateScale`, `PoseInput` (pose supplied from outside, e.g. a physics
  ragdoll in M4), `Subgraph`. Nodes are a closed Rust enum, so graphs are
  data, patchable by mods like records. New node kinds are framework code.
- **Procedural op stack** after the graph (per graph asset, ordered, each
  with a LOD cut-off): `SpeedMatch`, `StrideWarp`, `OrientationWarp`,
  `FootPlacement`, `LookAt`, `HandIk`, `AimOffset`, `SpringBones` (later).

**As implemented in `sf_anim_graph` (2026-10-11).** Authoring types live in
`asset` (RON, parsed with `IMPLICIT_SOME`, so optional fields need no
`Some(..)`; numbers or parameter names are interchangeable wherever a value
is read). `compile` (feature `bake`) produces a `CompiledGraph` (`SFAG`,
postcard). The runtime is `Binding` (graph + skeleton + clips in slot
order) plus one `GraphInstance` per character. These points refine or
change the sketch above:
- **Node references are instances.** A node named from two places gets two
  instances (own clock, own state). Nodes are laid out depth-first, so a
  subtree is an index range: resetting a state on entry, or finding its
  highest-weighted player for a phase condition, is a slice walk.
  `Subgraph` inlines another asset at compile; parameters merge by name.
- **Sync is a property of players, not a `Sync` node** (UE's model). A clip
  or blend space declares `sync: (group, role)` with roles `CanLead`,
  `AlwaysLead`, `NeverLead`. The highest-weighted active member leads, and
  followers are placed, not advanced: by markers when the leader's current
  marker pair exists in the follower (nearest occurrence to its playhead),
  else by normalized phase. Blend-space samples are always synced the same
  way to the space's highest-weighted sample, and the space plays at the
  weighted length of its samples (UE). Zero-weight branches aren't
  ticked, so their clocks hold until sync realigns them.
  A looping clip whose loop range sits inside it (an intro before, an
  outro after) syncs over its loop only: marker pairs wrap at the loop's
  end, markers outside the loop are skipped, and a follower always lands
  inside its loop. (Placed past the loop's end, a lane played out to the
  clip's end once it led and froze there: walkers5, 2026-10-13.)
- **Inertialization is a post-process on the graph output**, as Bollo and
  UE place it: a transition requests it and the shortest pending duration
  wins. Two corrections to the talk's per-bone scalar quintic (both needed
  for the continuity test to pass at shrinking dt):
  the offset's initial velocity is source minus *target* velocity (Holden,
  *Spring-It-On*, 2021; the talk and UE ignore the target's velocity), the
  target's velocity coming from one extra evaluation a frame ahead on the
  switch frame only; and each channel's offset is a 3-vector (translation,
  scaled-axis rotation, scale) with a quintic per component, because a
  fixed axis drops the velocity across it. Bollo's guards stay along the
  offset direction (blend shortened to `-5·x0/v0`, initial acceleration
  never toward zero). The switch frame outputs the previous output
  extrapolated by its velocity, so there is no one-frame stall, and an
  interruption restarts from the last two outputs. Dead blending (Holden
  2023, in UE 5.3+) is the alternative if large offsets look floaty.
- **State machines**: one transition per update, first match in list
  order, `"*"` for any state. `Crossfade(t)` uses smoothstep weights and
  freezes the current weights when interrupted; re-entering the current
  state inertializes instead. Triggers are consumed by the transition that
  reads them and cleared at the end of every update. `Event` conditions
  see the source state's events from the previous update; `PhaseAtLeast`,
  `TimeLeft` and `Finished` read its highest-weighted player.
- **Root motion output** is the weighted delta of `Drive` players only, plus
  `drive_weight` (their share of the pose): `Match` and `Ignore` players
  contribute zero, so the mover blends its capsule velocity with the root
  delta by `drive_weight`. A state's mode is inherited at compile by the
  players under it that set none. Masked blends scale root motion by the
  mask weight at the root-motion joint.
- **Events**: per top-level layer, only the highest-weighted lane fires
  point events (UE's "highest weighted" mode, scoped per layer so an
  upper-body attack still fires). Window edges come from lanes at weight ≥
  0.5. Events carry the clip slot, event index, graph-wide id and weight.
- **Curves** are blended along the tree like poses (absent where no clip
  below has them), so `contact.*` drives foot locks correctly in blends.
- **Procedural ops**: `SpeedMatch` acts on playback rates before the
  advance (it can't be an after-the-fact op) and leaves the residual ratio
  to `StrideWarp`. `AimOffset` is not an op: as in UE it is a 2D blend space
  of additive clips under an `Additive` node or additive layer. Ops take
  ground samples per leg (character space) and the character's motion
  since the last update, so foot locks stay planted in the world; the
  output's `probe_origins` are next update's probe points.
  `HitReact` and `SpringBones` (M4 P9): see "Next-gen roadmap" layer 4.
- **`Mirror`** works in model space: joint `j` takes its counterpart's
  model transform reflected, times a correction computed from the rest pose
  at compile, so any joint-axis convention mirrors correctly (pairs from
  the graph or the adapter's rig).
- **LOD**: levels 0–2 sample only the skeleton's LOD joint mask; 3 and up
  run state machines, clocks, events, root motion and curves but evaluate
  no pose, and drop the pose history (the next switch cuts).
- Not done yet: reference-clip additive bases (needs a derived clip
  product), per-joint inertialization blend profiles, and a graph-level
  event mode switch (all events vs highest weighted).
- **Measured** (`a0_update_timing`, release, battery power): one A0 update of
  a 2-layer + additive graph with a synced blend space under a state
  machine and all ops is 4.4 µs on the 17-joint test rig (5.0 µs on
  2026-10-12 after the creature ops, with another session active;
  the 31-joint quadruped with `Fit` foot placement 5.4 µs). Re-measure on a
  real ~60-joint skeleton against the 25 µs gate in `sf_anim_bevy`.

**Crates checked for the graph (2026-10-11).** `bevy_animation_graph` (above)
ties nodes to Bevy reflection. `animgraph` 0.1.0 (MIT/Apache, hierarchical
state machines) has had no release since 2023-05. Nothing on crates.io
implements inertialization or blend-space triangulation as a library. Used:
`spade` 2.15 (MIT/Apache, maintained) for the Delaunay triangulation at
bake, and `ron` 0.12, the version Bevy 0.20 already pulls in. Both are behind
the `bake` feature, so the runtime depends on neither.

## Evaluation (per character, per update)

1. **Advance**: the state machines tick, clocks advance, and sync groups
   align followers to the leader's phase or marker interval.
2. **Sample and blend**: the tree is evaluated depth-first into pooled local
   poses. Masked layers lerp/slerp per joint by mask weight, and additive
   layers apply `q = slerp(I, Δq, w) · q`. Inertialization offsets are
   added. Only joints in the current LOD mask are touched.
3. **Root motion**: the weighted blend of the active clips' root deltas.
4. **Model space**, then the **procedural ops** (below), then a final
   local pose.
5. **Outputs**: final local pose, root delta, fired events (with source
   weight), curve values, and bounds (for culling and probes).

The whole step is a pure function of (graph instance, params, dt, probe
results), so it runs in parallel per character and is deterministic.

**Events.** A point event fires when the playhead crosses its time in the
play direction, including across a loop wrap, at most once per crossing. A
window event (`end` set) reports enter and exit and is "active" while its
clip's weight is at least 0.5. In a blend or sync group, only the
highest-weighted clip fires point events by default (no double footsteps
from a 50/50 walk blend), as UE's "highest weighted animation" mode does.
Clips fading out under inertialization fire nothing.

**Root motion modes**, per clip or state:
- `Drive`: the delta moves the character (attacks, rolls, climbs,
  interactions). The mover (locomotion.md) consumes it on its next update:
  the kinematic mover in M3, the physics controller's fixed step from M4.
  That is one frame of latency, the industry norm.
- `Match` (locomotion default): the character is capsule-driven. The clip's
  baked speed sets playback rate = actual speed / clip speed, clamped to
  [0.75, 1.3]. Stride warping covers the rest, so feet don't slide.
- `Ignore`: in-place.

## Procedural pass and IK

Inputs come from a batched **ground probe** stage that runs before
evaluation (the `GroundProbe` trait, locomotion.md). It probes down from
last frame's foot positions moved by this frame's root delta.

- **Stride warping**: scales foot targets along the velocity about the
  pelvis's ground projection, and lowers the pelvis so legs stay
  reachable. **Orientation warping**: turns the lower body toward the
  velocity, up to about 60°, and counter-rotates the spine so the torso
  keeps facing the aim direction. Four-way locomotion sets then cover all
  directions.
- **Foot placement**: per foot, the target is the animated foot height
  above its ground hit, with the foot rotated to the hit normal (clamped to
  30°). While the baked contact curve is above 0.5 the world position is
  locked, and it is released over 0.1 s. The pelvis drops by the largest
  amount any foot needs, spring-smoothed. Legs solve with two-bone IK,
  using the animated knee direction as pole. Toes bend to the ground if
  tagged.
- **Look-at**: the target is distributed over spine, neck, head and eyes by
  weights, each joint clamped to limits and the total to ±80° yaw, with a
  smoothed target and weight.
- **Hand IK**: two-bone to an attachment or interaction target (door
  handle, lever, off-hand on a two-handed grip), weighted by a curve or
  parameter.

## LOD and budget

Tiers are chosen by projected height in pixels (as `mesh.cook` LODs are,
bake.md), times a significance factor (player target, in dialogue, in
combat ×2).

| Tier | Projected height (1440p, 70°) | Update | Graph | Procedural | Joints |
|---|---|---|---|---|---|
| A0 | ≥ 120 px (≈ ≤ 15 m) | every frame | full | all ops | all |
| A1 | ≥ 50 px (≈ ≤ 37 m) | every frame | full | speed match, stride, feet | LOD1 mask |
| A2 | ≥ 20 px (≈ ≤ 90 m) | 1/2 to 1/4 rate, interpolated | full | speed match | LOD2 mask |
| A3 | < 20 px, or off-screen | clocks, events, root motion only | state machines only | none | none (pose frozen) |

- Off-screen characters that cast shadows into a view count as A2.
- Events still fire at A3 within audible range, so footsteps work behind
  the camera.
- The **budget allocator** demotes the least significant characters one
  tier at a time until the predicted cost (an EMA per tier) fits the
  budget, and promotes back with hysteresis.
- Characters in streaming ring R1 (visual-only, streaming.md) never exceed
  A2 and play ambient idles. Crowds beyond that (vertex-animation-texture
  crowds, as in UE's City Sample) are a later tier.

**Targets** (reference machine, raster path; these become `sf_diag` gates):

| Measure | Target |
|---|---|
| Animation stage, wall time, "crowd street" scenario (150 NPCs, 40 at A0) | ≤ 1.0 ms |
| One A0 character (graph with 2 layers + 2D blend space + all IK), single thread | ≤ 25 µs |
| GPU skinning, all passes, same scenario | ≤ 0.5 ms |
| Foot slide during contact | ≤ 2 cm/s mean |
| Foot penetration or float | ≤ 2 cm, 95th percentile |
| `anim.cook` for a whole content pack | ≤ 5 s of the 2 min bake |
| Foot slide and penetration on creatures (every support limb) | same as above |
| Worst-case A0 character with `FullBodyIk` and warping | ≤ 35 µs |
| Warp target error at window end (static target) | ≤ 2 cm, ≤ 2° |

A city needs this many: dense modern cities run tens of fully simulated
NPCs and hundreds of high-detail ones (Assassin's Creed Unity: 40 real AIs,
120 high-resolution models, about 10,000 crowd NPCs; Cournoyer, *Massive
Crowd on Assassin's Creed Unity: AI Recycling*, GDC 2015). The 150-NPC
scenario is our M3 floor, not the ceiling.

## Bevy integration (`sf_anim_bevy`)

- Components: `Animator { graph, instance, params }` on the character root,
  `SkeletonBinding { joints: Vec<Entity> }` (skeleton index → bone entity),
  `AnimLod`. The `AnimEvent { entity, name, payload, weight }` message and
  entity-event observers carry events. `AnimCurves` holds curve values for
  gameplay.
- Schedule (PostUpdate, before transform propagation): `Probe` (batched
  ground queries) → `Evaluate` (`par_iter` over characters due this frame,
  pure) → `Apply` (root delta stored for the mover, bone `Transform`s
  written only for joints in the LOD mask, events sent). The mover runs
  earlier, in `Update` (locomotion.md). Bevy then propagates
  transforms and skins on the GPU.
- Bone entities stay, because Bevy's `SkinnedMesh` reads their
  `GlobalTransform`. If profiling shows propagation or skin extraction
  dominating, the fallback is writing bone `GlobalTransform`s directly
  after propagation (we already have the model-space pose), without
  touching Bevy's renderer.
- Debug: skeleton and IK gizmos, pose and graph state overlay, an LOD tint.
  `sf_diag` counters for characters per tier, µs per tier, and events per
  frame.
- **Mesh side, as built (skinloader, 2026-10-12)** in `sf_model_bevy`
  (bake.md "Bevy mapping"): `spawn_skinned` gives each mesh `SkinnedMesh`
  (joints resolved by name, `resolve_joints`) and `DynamicSkinnedMeshBounds`
  (Bevy refits the `Aabb` from joint transforms each frame, as `bevy_gltf`
  does by default). Morph curves write the model-wide `ModelMorphWeights`
  on the root; `sync_morph_weights` gathers each mesh's own targets into
  `MeshMorphWeights::Value`. So `Apply` writes bone `Transform`s and
  `ModelMorphWeights`, nothing renderer-specific. `spawn_rest_skeleton` is a
  named scaffold until the rig spawn here replaces it; `SkeletonBinding`
  is what `resolve_joints` returns.

Moved to [animation-history.md](animation-history.md#as-built-animbevy-2026-10-12).

Moved to [animation-history.md](animation-history.md#foot-placement-guards-as-built-animfix-2026-10-12).

Moved to [animation-history.md](animation-history.md#morph-curves-as-built-actorfx-2026-10-12).

## Bake steps

| Step | Subject | Inputs | Output |
|---|---|---|---|
| `skeleton.cook` | skeleton source | model file, adapter rig data | `Skeleton` + joint LOD masks |
| `anim.cook` | (clip source, skeleton) | source clip, skeleton, adapter clip rules (split, loop, root joint) | `Clip` with root motion, contacts, markers, meta |
| `anim.retarget` | (clip, target skeleton) | cooked clip, both rig maps (character.md) | `Clip` on the target skeleton |
| `animgraph.compile` | (graph, skeleton) | graph asset, skeleton, clips it names | compiled graph: resolved clips and masks, triangulated blend spaces |
| `rig.profile` (planned, M3) | skeleton | cooked skeleton, adapter `RigHints` | `RigProfile` (rig-profile.md) |
| `ragdoll.cook` (planned, M4) | skeleton | skeleton, rig profile, skin extents | ragdoll bodies, joints, limits, masses |
| `posesearch.cook` (planned, later) | (graph, skeleton) | clips of a pose-search database | normalized feature matrix + acceleration boxes |

**BVH and FBX importers (built 2026-10-12, `sf_anim` features `bvh` and
`fbx`, bake only, both in `AnimImporters::framework()` when enabled).**
Bevy has no BVH or FBX loader (bevy_gltf only), so rule 8 goes to crates:

- **BVH: our own parser** (`bvh_import.rs`, ~250 lines, no
  dependencies). `bvh_anim` 0.4.0 (MIT) is from 2019 on `bstr` 0.1,
  `lexical` 2 and `smallvec` 0.6 (old, advisories); `bvh_anim_parser`
  1.0.1 (MIT, 2024) has `cargo` 0.78, `regex` and `cgmath` as *runtime*
  dependencies. The grammar fits on a page. Joints from `ROOT`/`JOINT`
  (the name is the rest of the line, so `Bip01 Pelvis` works), rest =
  `OFFSET` with identity rotation, each `End Site` a leaf `<parent>_End`
  (lengths for auto-map/auto-align). One clip per file named after the
  file stem, keyed at every frame at `1 / Frame Time` (30/60/120 fps kept
  exactly: `BoundSource::native_rate` sees it). Rotation channels compose
  in listed order (`Zrotation Xrotation Yrotation` = Rz·Rx·Ry), degrees;
  position channels replace the offset on any joint; a truncated take keeps
  its complete frames.
- **FBX: `ufbx` 0.11.5** (MIT OR Unlicense, 2026; Rust bindings to the
  maintained single-file ufbx C library, built by `cc`: the bake needs a C
  compiler, MSVC here, ~20 s once). It resolves the FBX transform chain
  (pre/post rotation, pivots, inherit modes) and converts any file to
  meters, Y up, right-handed with `SpaceConversion::AdjustTransforms`, so
  evaluated local transforms are already in our space (unit scale folded
  into translations: verified, rest scales stay 1). Skeleton = bone nodes
  plus ancestors except ufbx's scene root (an `Armature` node is kept, as in
  glTF); without bones, the null nodes. Clips: one per animation stack,
  sampled with `ufbx::evaluate_transform` at the scene's
  `frames_per_second` (30 if unset) from the stack's start, channels that
  never leave rest dropped. Mixamo's `mixamorig:` prefix is kept (auto-map
  handles namespaces). `fbxcel`/`fbxcel-dom` (MIT/Apache) are stale since
  2022–23 and low-level; `russimp` has a non-standard license and a C++
  dependency.
- **Units and axes: `SourceSpace`** (`import.rs`: `unit_scale`, `up`,
  `forward` as signed `Axis`, `right_handed`). The change of basis `C`
  (rows: the source's left, up, forward) conjugates every local transform:
  translation `s·C·t`, rotation `C·R·Cᵀ` (one quaternion even for a
  left-handed, mirrored source, since `C·R·Cᵀ = P·R·Pᵀ` with `P = −C`), so
  model positions become `s·C·p` exactly. BVH carries no unit metadata:
  `BvhAnimImporter` defaults to centimeters, Y up, facing +Z and takes
  path-prefix overrides (`byo/cmu/` in inches, …); the space's postcard
  bytes join the bake key through the new `AnimImporter::settings_key`
  (empty by default, so glTF keys are unchanged). A forward axis other than
  +Z is also fixed later by auto-align's facing correction; the space is
  what makes root motion point the right way.
- **Root-motion joint detection** (`detect_root_motion_joint`, every
  importer fills `AnimSource::root_motion`): a `root`-named joint (after a
  `ns:`/`a|b` prefix) whose translation moves, else the shallowest moving
  joint (a BVH `ROOT`, Mixamo `Hips`, an armature object carrying the
  travel), else a `root`-named joint. `skeleton.cook` uses it when the
  adapter's `SkeletonRules` name no joint and set `detect_root_motion`
  (default off, so existing bakes don't change); `anim.cook` then projects
  that joint to the ground as before.
- **Tests**: BVH fixtures written in the tests (hierarchy, channel order,
  rate, malformed files, spaces vs hand-computed model positions); a 129 KB
  CC0 FBX made by us with Blender (`sf_anim/tests/data`, generator script
  included: Z up, facing −Y source → hips at 1.0 m Y up, left +X, travel
  +Z, 24 fps); and `sf_character/tests/bvh_retarget.rs`: a mocap-named
  BVH (cm, Y up, 60 fps) → auto-map + auto-align → retarget onto a
  `Bip01`-named Biped skeleton (Z up facing −Y, meters, arbitrary joint
  frames): worst 0.000 cm and 0.034° over 31 frames, 21 joints (gate
  1 cm / 1°). Real 100STYLE and Quaternius FBX files are checked when the
  BYO bake runs (bevyrrowind animation-sources.md).

Contacts come from foot height and speed thresholds on the source clip
(with adapter overrides). Sync markers come from contact onsets.
As in motion-matching data pipelines, a foot is planted while it is low
(height over its lowest point in the clip, `ContactRules::height`, in leg
lengths) *and* still over the ground (`speed`, leg lengths per second),
with hysteresis: it plants under the thresholds and lifts only above
`release` times them (2×), so a rolling heel or a slow lift doesn't
flicker. Speed over the ground is the foot's velocity in clip space plus
the ground velocity: zero for travelling clips (the root carries the
ground), and for in-place clips the velocity the planted feet slide back
at (component median of the low feet, refined by the mean of those moving
with it; it also becomes the clip's `average_velocity`). Keyframed sources skate, so each foot is measured against the per-frame
stance reference (the slowest low foot). A stance ends at heel-off: the
foot's last rise faster than `lift` (leg lengths per second) before it
leaves, trimmed within the stance's second half, so the runtime lock
releases the heel as the authored foot peels off (a heel held into
heel-off overreaches uphill: 25° ramps slid 2.6 cm/s). Before cook v4
in-place clips used height alone and Morrowind's sidesteps planted on
0–6 of 81 frames (walkers3).

Curves have a joint (UE's curve "linked bones",
`CompiledGraph::curve_joints`): a leg's contact curve belongs to its
effector. A masked override layer or masked slot blends a linked curve by
its joint's mask weight, so an upper-body or arm overlay never replaces
the legs' contacts (it used to: the torch arm and the crossbow/spell
stance idles froze the walk's contacts at their idle's values).

**Implementation (`sf_anim::steps`).** The three `sf_anim` steps share one
`AnimBake`: importers by extension (`AnimImporter`; glTF, BVH, FBX by feature), the
adapter's `AnimRules`, and the `CookSettings`. Subjects:
`skeleton.cook` takes the skeleton source path; `anim.cook` takes
`<skeleton>|<clip file>|<clip>`; `anim.retarget` takes
`<target skeleton>|<anim.cook subject>`. `AnimRules` returns the
`SkeletonRules` (root-motion joint, tags, LOD rules), a `ClipSpec` per clip
(the source clip name, so one take can be split into several clips, plus
`ClipRules`), and a `RetargetRig` per skeleton. That data isn't a file, so
its postcard bytes are step parameters, part of the bake key.

The glTF importer (the `gltf` crate, as in `sf_model`) builds the skeleton
from every skin joint and animated node plus their ancestors, so an
`Armature` node's transform is kept. Each animation becomes a source clip
with step, linear and cubic-spline channels as authored. Morph-weight
channels become curves named `<node>.morph<k>`. glTF has no events.

**`animgraph.compile` (`sf_anim_graph::steps`, feature `bake`).** Subject
`<graph file>|<skeleton>` (a `skeleton.cook` subject). Planned inputs: the
graph file and the skeleton product. Clip products and subgraph files are
read during the run, so they are dynamic inputs and re-cooking a clip
recompiles the graphs that play it. Clip references default to
`<file>#<clip>` → `anim.cook` on the graph's skeleton; the adapter's
`GraphRules` can map them elsewhere (e.g. to `anim.retarget`) and supplies
rig bone names and mirror pairs, whose postcard bytes are a parameter.

**Adapters whose clip rules depend on file contents (as built, animbake
2026-10-12).** Bake keys are fixed at plan time, so rules computed from a
source's markers (sub-action ranges, loop tails, which layered file holds a
clip) can't be read during planning. The first adapter bakes in two phases:
a group-table product per source file (an adapter step) plus
`skeleton.cook`, then builds its `AnimRules` and `GraphRules` from those
products and bakes `anim.cook` and `animgraph.compile`. Warm, phase 1 is
all hits. `GraphRules::clip_files` should list every file of a layered set
(and candidates that may appear), so layering changes recompile. `AnimBake`
memoizes imports per session by (path, content digest): legacy files hold
~100 clips each, and every `anim.cook` used to re-import its file;
`AnimBake::clear_imports` frees the memo after a phase.

**Measured (Morrowind, animbake 2026-10-12).** 96 skeletons, 3,987 clips,
96 graphs: cold 13.5 s wall (`anim.cook` 131 s CPU, 33 ms per clip), warm
0.31 s. The 5 s `anim.cook` gate is missed: 1,150 clips never reach the
0.1 mm bound (errors up to 3.6 cm at 120 Hz, so the cause is source
discontinuities or quantization, not the rate) and run all three rate
passes, more than half the cook time.

**Rate search, as built (animcook 2026-10-12, `anim.cook` version 3).**
Each candidate rate first measures its *resampling error*: the unquantized
frames, interpolated as the runtime does (root motion and additive base
re-applied), against the source at every midpoint. Quantization adds at
most half the bound (its budget), so a rate whose resampling error exceeds
1.5x the bound can't pass and is neither quantized nor measured. Otherwise
the cooked clip is measured at every frame and midpoint as before, but the
measurement stops at the first sample over the bound, reuses the sampled
frames (frame times) and the resampling check's source poses (midpoints)
instead of sampling the source again, and computes each virtual vertex from
the column and translation differences of the two model matrices. When no
rate passes, the rate with the lowest resampling error is kept (the coarser
within 10%) and only it gets a full measurement. Bit search starts from the
rounding-bound estimate (`log2(half-diagonal / tolerance + 1)`) and walks,
about two tests per channel instead of six. Survey (`mw_bake` test
`anim_cook_survey`, 3,307 group clips, 16 threads, cook only, no store):
35.9 s CPU / 3.9 s wall before, 24.1 s / 2.3 s after (quantize 15.7 → 5.8 s,
measure 14.6 → 12.3 s incl. the resampling checks). Quality: 2,559 clips
identical, 416 lower error, 332 higher but all still under the bound (bit
choices at the tolerance edge, up to +34% of a sub-bound error); the 788
clips over the bound: 5 better, 5 worse by at most 0.19%, none newly over.
Real-install bake (`anim_real_install`, indicative only: run at below-normal
priority while the owner tested the game): `anim.cook` 131 → 54.6 s summed
run time (3,987 clips; ≈3.4 s at 16 threads, under the 5 s gate), cold
13.5 → 9.0 s, warm 0.22 s, 1 vs N threads byte-identical (693 products).
A dedicated 16-thread timing run on a quiet machine is still owed. The
summed run time includes waits on the shared import (`AnimBake` memo) and
on dependencies, so it overstates CPU. Two hangs seen in ~6 runs (one
`sf_anim` `bake_steps_cook_retarget_and_cache`, one real-install bake),
idle CPU, not reproducible on rerun: blocking `OnceLock` slots (`sf_bake`
products, `AnimBake` imports) initialized inside rayon tasks can deadlock
when a waiting thread steals work that needs the same slot. Pre-existing;
needs an `sf_bake` fix (no blocking inside the pool, or `rayon`-aware
waits).

**Why ~20% of the Morrowind clips can't reach 0.1 mm** (survey
`SURVEY_DIAG=1`, worst channel per clip): (1) *linear quaternion keys at
off-grid times* (NIF quaternion keys, and the Euler tracks `mw_anim`
samples at 60 Hz into linear keys): each key is a kink, so uniform
resampling error only halves per doubling and fast rotations stay above
0.1 mm at 120 Hz (~500 clips); (2) *fast cubic translations* (weapon bones,
creature root bones; error falls 4x per doubling but needs 240–480 Hz;
~150 clips; several segments have Hermite tangents 8–16x their chord,
worth checking against the NIF tangent convention in `mw_anim`); (3) the
rest are not resampling at all (whole-pose resampling error well under the
cooked error): root-motion extraction (yaw as the Y twist of the root,
interpolated separately from the residual, which is ill-conditioned when
the root pitches far, e.g. deaths) and source discontinuities (error that
doesn't fall with the rate). The bound is stricter than the industry's (see
"Industry comparison", clip compression), not wrong; these clips cook at
the best rate and record their error.

## Motion matching: later, not in M3 (WISHLIST §8 kept)

Motion matching (Clavet, *Motion Matching and The Road to Next-Gen
Animation*, GDC 2016; Mach & Zhuravlov, *Motion Matching in The Last of Us
Part II*, GDC 2021; production-ready in UE 5.4) needs a large, consistent
mocap library. Learned motion matching (Holden et al., SIGGRAPH 2020) cuts
its memory. Legacy content is a few hand-keyed cycles, where MM gains
nothing. M3 builds what it needs, so it arrives as one more node type: the
inertialization, the trajectory model (locomotion.md), and bake-time
feature extraction. Candidate data is the 100STYLE dataset (CC BY 4.0, BVH,
60 fps) and Quaternius' Universal Animation Library (CC0, glTF). LAFAN1
(CC BY-NC-ND 4.0) and the Bandai Namco sets (CC BY-NC 4.0) are usable only
as bring-your-own sources baked privately on the user's machine, never
shipped (data plan and license table: `bevyrrowind/docs/specs/animation-sources.md`).

## Next-gen roadmap (anim-plan, 2026-10-11; shape replaced by R11b, 2026-10-19)

Moved to [animation-layers.md](animation-layers.md#next-gen-roadmap-anim-plan-2026-10-11-shape-replaced-by-r11b-2026-10-19).

Moved to [animation-history.md](animation-history.md#optional-clips-m3-for-bring-your-own-sources).

Moved to [animation-history.md](animation-history.md#slots-and-montages-m3-d-as-built-2026-10-12).

Moved to [animation-history.md](animation-history.md#creatures-m3).

Moved to [animation-history.md](animation-history.md#audit-of-the-built-crates-biped-and-humanoid-assumptions-2026-10-11).

## Verification

- Unit tests (`sf_anim`, `sf_anim_graph`): the cook round-trips within the
  error bound, sampling matches the source curves, and additive bases
  cancel. Masks, sync phase alignment, and inertialization are continuous
  in position and velocity at the switch. Events fire exactly once across
  wraps and in reverse. Two-bone IK reaches reachable targets and clamps
  unreachable ones without flipping. Root-motion blending is correct, and
  evaluation is identical across thread counts.
- Viewer: `anim` mode with skeleton gizmos, a graph state overlay, and
  parameter sliders. Screenshot sequences of a walk on stairs and a slope.
- `sf_diag` scenario "crowd street" with the gates above, plus foot-slide
  and penetration metrics computed from contact curves and ground probes.

## Industry comparison

Moved to [animation-history.md](animation-history.md#industry-comparison).

## Open questions

1. Whether the procedural op stack belongs per graph asset or per
   character archetype (shared by several graphs). Per graph asset for
   now (`sf_anim_graph`); subgraphs share node logic but not ops.
2. SoA (4-wide) pose math as in ozz: adopted in `sf_anim` from the start
   (see Data model). Measure against the A0 target once the graph exists.
3. A compute "skin cache" (skin once per frame, reuse in every pass, feed
   ray-tracing BLAS refits) like UE's: needed for Solari, whose
   `RaytracingMesh3d` takes no joint attributes, so skinned characters
   cast no ray-traced shadows or GI today. Measure in M3, decide by M7.
4. Authoring: RON first, an in-viewer graph inspector next. A full editor
   (or `bevy_animation_graph`'s, if it converges) later.
5. Distance matching (gap, Industry comparison): a `distance` curve per
   start/stop/pivot clip at `anim.cook` (root translation along the path,
   negative before the stop), a `Player` mode that seeks by remaining
   distance, and a stop/pivot point predicted from the trajectory springs
   (locomotion.md open question 4). Worth it once library start/stop clips
   are baked; nothing to gain on cycle-only legacy sets.
6. Foot placement pelvis modes (UE's `AllPlantedFeet` and uphill/downhill
   variants) and a heel-lift ratio before the pelvis drops: fold into the
   per-girdle `FootPlacement` rewrite (audit row 2) and pick by visual check.
7. ~~Montage-style `Slot` node~~ **Resolved 2026-10-12 (M3-d)**: both the
   binding's clips (`slot_clips`) and late-bound clips (`LateClip`, mapped at
   runtime); interrupt with priority or queue; see "Slots and montages".
   Still open: montage sections / branching points, and whether a late-bound
   set should be resolved by `GraphRules` at spawn rather than per request.
8. Crowd pose sharing (gap, P2): A2 characters on the same skeleton and
   clip, bucketed by quantized phase, copy one evaluated pose (UE Animation
   Sharing). Still open. **Variety half built 2026-10-22 (animation-plan
   slice 3):** `sf_anim_graph::variety`: `Variety::from_seed(seed, 0.05)` (splitmix64
   hash of a stable id; `AnimSeed::from_id(placement id)` in `sf_character_bevy`, else spawn
   position) gives a start phase in [0,1) and a rate in 1±5 %. `GraphInstance::set_variety`
   applies the phase at every looping player (re)start and the rate to looping players that are
   not root-motion `Drive` (locomotion's speed match divides it back out: no foot slide).
   `FidgetPicker`: weighted, min/max gap, never the same fidget twice in a row (a sole
   candidate may repeat). Off via `AnimVarietySettings.enabled`. Content: AI idle rolls skip the
   last fidget (`mw_ai`), level `@fidget` action (lab crowd, `level-anim-crowd`).
