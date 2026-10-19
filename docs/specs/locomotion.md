# Locomotion: intent, trajectory, ground probes, kinematic mover

Status: `sf_controller` built (2026-10-11; ORCA local avoidance 2026-10-12); `sf_controller_bevy` not started
Crates: `sf_controller` (Bevy-free: intent, spring motion, trajectory,
`GroundProbe` with terrain and static probes, the kinematic mover's logic, ORCA local avoidance,
path follower, `collision.cook`), `sf_controller_bevy` (to build: schedule,
batching, gizmos, counters in M3; physics controller in M4)

## Goal

Move characters, the player and NPCs, so animation can follow them
believably: smooth accelerations and turns that the animation graph can
read ahead of time, ground contact on terrain, stairs and ramps, and
ground probes for foot placement. M3 needs this for NPCs walking the
streets before physics exists (avian3d 0.7 is on Bevy 0.19 as of
2026-10-11). This spec is the first slice of `sf_controller`
(ARCHITECTURE §5.3). M4 replaces only the mover, not the model.

Non-goals: navigation and AI (M10; M3 drives NPCs with an adapter
scaffold), climbing (hooks only), physics pushing. Flying, hovering and
swimming creatures, context probes and traversal are designed in
"Context probes, traversal and creature modes" below.

## Canonical technique

- **Capsule-driven locomotion with an animation-aware trajectory.** Input
  becomes a desired velocity and facing. Actual velocity and facing follow
  through critically damped springs (Holden, *Spring-It-On*, 2021). The
  same springs **predict the trajectory** 0.2/0.4/0.6/0.8/1.0 s ahead,
  which the graph uses (start, stop and turn states, motion matching
  later). This is the controller half of UE's Character Movement plus
  motion-matching trajectories, and of Clavet (GDC 2016).
- **Animation follows the capsule** for locomotion (animation.md, root
  motion `Match`). It **drives** the capsule for actions (`Drive`). Root
  motion deltas are applied by the mover, so one place moves the character.
- **Ground following, kinematic (M3)**: snap to the highest walkable ground
  within a step-up and step-down range, slopes limited by angle, with the
  vertical motion spring-smoothed. That smoothing hides stair steps, and
  the foot IK puts the feet on the real steps.
- **Ground probes** are ray and short sphere casts against terrain heights
  plus static collision. They are batched once per frame for every
  consumer (mover, foot placement).

## Data model

```rust
pub struct LocomotionIntent {          // written by player input, AI, scripts
    pub desired_velocity: Vec3,        // world, horizontal
    pub desired_facing: Option<Dir3>,  // None = face velocity
    pub gait: Gait,                    // Walk | Run | Sprint | Sneak; adapter-defined speeds
}
pub struct Locomotion {                // controller state, saved
    pub velocity: Vec3, pub facing: Dir3, pub angular_velocity: f32,
    pub grounded: Option<GroundHit>,
    pub trajectory: [TrajectoryPoint; 5], // predicted position and facing, +0.2 … +1.0 s
}
pub struct MoverParams { pub halflife_vel: f32, pub halflife_rot: f32, pub step_up: f32, pub step_down: f32, pub max_slope_deg: f32, pub radius: f32, pub height: f32 }
```

The graph parameters fed each frame are speed, direction relative to
facing, turn rate, grounded, and the trajectory's future heading change
(used for turn-in-place and start/stop states).

## Ground probes

```rust
pub trait GroundProbe: Send + Sync {
    /// Batched: one call per frame for all queries.
    fn probe(&self, queries: &[ProbeQuery], out: &mut [Option<GroundHit>]);
}
pub struct ProbeQuery { pub origin: WorldPos, pub dir: Vec3, pub max: f32, pub radius: f32 } // radius 0 = ray
pub struct GroundHit { pub point: WorldPos, pub normal: Vec3, pub surface: SurfaceId, pub walkable: bool }
```

Implementations, composed by taking the nearest hit:
- `TerrainProbe` over `sf_terrain::TerrainHeights` (terrain.md: `raycast`,
  `height`, `normal`).
- `StaticProbe` over a **static query world**: a BVH of the R0 ring's
  static collision shapes, built on the task pool as cells activate
  (streaming.md stage 4). It uses **parry3d** (0.31.1, 2026-09, Apache-2.0,
  Bevy-free), the query library under both avian and rapier, so in M4 the
  shapes and code move into the physics world unchanged. Collision shapes
  come from the model's collision geometry when the importer provides it,
  else from render LOD1 (a `collision.cook` bake step that M4 reuses).
- M4: `PhysicsProbe` (physics engine scene queries) replaces both.

Surface ids give foot-placement and footstep sounds their material
(stone, wood, sand).

## Movers

- **`KinematicMover` (M3).** Per frame, in `Update` after intents are
  written and before animation (M4's physics controller moves to the fixed
  step with transform interpolation): spring the velocity and facing
  toward the intent, add last frame's root-motion delta for `Drive` states, move
  horizontally, then probe down from step-up height. The character lands
  on the highest walkable hit within step-down, with height smoothed
  (half-life 0.08 s) when stepping up. Unwalkable hits block, sliding along
  the contact. There is no actor-to-actor collision, but a cheap separation
  (radius push) between nearby characters, after ORCA avoidance has steered
  the intents ("Local avoidance (ORCA)"). Named as a stage of the
  controller, not a scaffold: its state and inputs are the M4 controller's.
- **Physics controller (M4)**: the same `LocomotionIntent`/`Locomotion` on
  a physics capsule; `bevy_tnua` or our own (ARCHITECTURE §11). **Decided 2026-10-12 (physics.md
  "Controller"):** the kinematic mover stays, in `Update` at frame rate (not moved to the fixed step), with a
  kinematic capsule body, capped pushes on dynamic bodies and `PhysicsProbe` replacing `StaticProbe`.
  **Required change (R11b + R12 co-op rule 1, 2026-10-19):** movers step on the fixed tick; see
  "Action-layer changes (R11b)" below.

**NPC drivers.** Until `sf_ai` (M10), NPC intents come from an adapter
system that follows authored paths. This is the named scaffold
`PathWalkScaffold`, removed when `sf_ai` takes over the intent. The
framework provides the follower: path polyline, arrival radius, wait at
nodes with an idle-variant parameter for the graph. Adapters provide the
paths.

## Verification

- Unit tests: spring convergence and no overshoot; trajectory prediction
  matches simulated motion within 1 cm at +1 s for constant intent; step-up
  and step-down; slope limit; batched probe results identical to single
  queries. Done: `cargo test -p sf_controller --all-features` (18 unit
  tests, plus `tests/graph_drive.rs` driving a real `sf_anim_graph` graph:
  SpeedMatch from the controller's velocity, FootPlacement on probed
  ground with the character turned 1 rad, `Drive` root motion fed back
  into the mover).
- Viewer: a walking NPC on stairs, a ramp, and terrain, with trajectory
  gizmos. Screenshot sequence plus the foot metrics in animation.md.
- `sf_diag`: probes per frame and probe time (budget ≤ 0.2 ms for the
  150-NPC crowd scenario, about 3 probes per A0/A1 character).

## As implemented in `sf_controller` (2026-10-11)

The sketches above stay as the design record; these points refine or
change them. Conventions: Y up, meters, yaw about +Y with yaw 0 facing +Z
(`sf_anim`'s character space), positions as `WorldPos`.

- **Mover logic is Bevy-free.** `KinematicMover::step_batch` (springs,
  root-motion blend, collide-and-slide, ground following, trajectory) lives
  in `sf_controller`, so its tests run without Bevy (ARCHITECTURE §8b).
  `sf_controller_bevy` only gathers characters, calls it, and writes
  transforms. Probes per step: one batched capsule sweep for the moving
  characters, one batched ground cast, plus single casts only for
  characters sliding along walls or blocked by a slope. A grounded
  character standing still casts nothing.
- **Types.** Facing is a yaw `f32`, not `Dir3` (a Bevy type).
  `LocomotionIntent { desired_velocity, desired_facing: Option<yaw>, gait }`;
  `GaitSpeeds { forward, side, back }` turns a move input into a desired
  velocity (Holden 2022: slower sideways and backward). `Locomotion` holds
  `position` (feet, exact), `MotionState` (velocity, acceleration, yaw,
  yaw rate, held facing goal), vertical speed, `grounded`, the trajectory,
  a drawn-height offset, and `moved` (the graph's `UpdateInputs::moved`).
  `MoverParams` adds `skin`, `gravity`, `max_fall_speed`, `ground_radius`
  (0 = ray) and nests the spring half-lives (`SpringParams`, Holden's
  0.27 s defaults).
- **Springs and prediction** are Holden's exact closed forms: the velocity
  spring integrated into position ("spring character") and a yaw spring
  the short way round (`sf_anim::spring`). The trajectory evaluates the
  same functions at +0.2 … +1.0 s, so for a constant intent prediction and
  simulation agree to float precision at any frame rate (tested with
  4–33 ms frame jitter). Points carry offset, yaw and velocity. No history
  samples yet: motion matching (UE's trajectory has history plus
  prediction) adds them with its node.
- **Graph inputs.** `LocomotionParams` (speed, local velocity, direction,
  turn rate, grounded, future heading change, future speed).
  `CharacterFrame` converts world ↔ character space: `foot_queries` turns
  the graph's `probe_origins` into ray probes, `ground_sample` turns hits
  into `GroundSample` points and normals, `delta_to` gives `moved`. The
  controller doesn't depend on `sf_anim_graph`; the integration test
  (`tests/graph_drive.rs`) is the per-frame glue the Bevy crates will run.
- **Root motion**: displacement, velocity and yaw are blended with the
  spring's by `drive_weight` (the graph's output), one frame late. Root
  motion's vertical part is ignored while grounded.
- **Ground hits** carry `distance`; `walkable` is the mover's decision
  (`GroundHit::walkable(min_normal_y)`), not a probe field, since the slope
  limit is per character. `ProbeQuery` gained `half_height` (upright
  capsule) for the wall sweep.
- **Wall sweep**: an upright capsule from `step_up` to `height`, so
  anything lower is a step for the ground cast. Its rounded bottom rides
  ledges just above `step_up` with a slightly smaller radius (about
  0.26 m of 0.3 at 0.15 m above `step_up`); nothing climbs them.
  Collide-and-slide follows Fauerby (2003), as Unity's KCC and Rapier's
  controller do: move to the contact minus `skin`, keep the tangential
  remainder, up to four sweeps, and remove velocity into the wall so the
  spring stops pushing.
- **Slope limit**: unwalkable ground blocks motion that *climbs* it
  (horizontal normal · displacement < 0), keeping the motion along the
  contour. A rise threshold was tried first and failed: the re-accelerating
  spring crept up a 60° ramp in sub-centimeter steps. Descending steep
  ground is allowed within `step_down`; sliding down steep slopes waits for
  M4.
- **Height smoothing** only smooths steps: the part of the height change
  a planar continuation of the previous ground doesn't explain. It is a
  drawn offset (`render_position`), so probes and the trajectory use the
  exact feet. Smoothing every height change would lag about 0.25 m below
  the surface when running up a ramp.
- **`TerrainProbe`** is generic over a `HeightSurface` trait (height,
  normal, raycast, surface), implemented for `sf_terrain::TerrainHeights`
  behind the `terrain` feature, so `sf_terrain` didn't change. Downward
  casts read height and normal directly; a downward sphere rests on the
  tangent plane (center `radius / normal.y` above it). Terrain surfaces
  default to `SurfaceId::DEFAULT`: adapters wanting per-layer footsteps
  wrap `TerrainHeights` in their own `HeightSurface`.
- **`StaticWorld`** (the static query world, a `GroundProbe`): parry3d
  `TriMesh` per model (`StaticShape`, shared by `Arc`), placements in a
  parry `Bvh` (`insert`, `remove`, `rebuild` after a cell's batch,
  `rebase` to move the f32 anchor). Uniform scale reuses the shared mesh;
  non-uniform scale bakes a scaled copy. Rays use `cast_local_ray`, sweeps
  `cast_shapes` with `stop_at_penetration: false` (a character touching a
  wall can still move away); the hit triangle gives the surface.
- **`collision.cook`** (feature `bake`, `sf_controller::steps`): subject =
  model path. Planned input: the `mesh.cook` product, plus whatever the
  adapter's `CollisionRules::plan` names. Output `CollisionMesh` (`SFCL`,
  postcard): welded positions (1 mm grid), triangles, a `SurfaceId` per
  triangle from `CollisionRules::surface(material)`. Geometry is
  `CollisionRules::authored` (e.g. a legacy format's collision nodes) when
  the adapter has it, else render LOD1 (`CollisionSettings::lod`).
  Degenerate and duplicate triangles are dropped. Models `mesh.cook`
  skipped get an empty mesh (no static collision for animated models in
  M3). Override: `collision/<model path>.sfcol`.
- **Path follower** (`PathFollower`): `Once`, `Loop`, `PingPong`;
  walk-through nodes are cut within `arrival_radius` (0.5 m), stop nodes
  (waits, ends) are approached at `min(speed, max(distance · braking,
  min_speed))` with braking `ln 2 / (4 · halflife)`, which keeps the closed
  loop with the velocity spring from overshooting. Waiting outputs the
  node's idle variant and facing.
- **Separation**: `separate` pushes overlapping characters apart (half
  each, sort-and-sweep on X, ignores characters 1.5 m apart vertically).
- **Measured** (`crowd_step_timing`, release, one thread, battery power):
  one mover step for 150 characters in a town of 800 static boxes on
  terrain, all probes included, is 0.096 ms median (0.105 ms p95), inside
  the 0.2 ms probe budget. Foot probes come on top (two rays per A0/A1
  character); re-measure in the viewer's crowd scenario.

**Traversal, flying, swimming, shapes (anim-creatures, 2026-10-12;
animation.md audit 13–14).** The canonical shape is UE's
`CharacterMovementComponent` movement modes (Walking, Flying, Swimming
with an immersion depth; a mantle montage runs in Flying mode) on our
kinematic mover:
- **`RootMotion::traversal`**: the glue sets it from the graph's
  `ActiveWarp::vertical` (a `y=warp` window). That step moves freely: the
  full root delta, vertical included, no ground snap or gravity, walls
  still slide, a ceiling sweep clamps rising and the floor cast clamps
  descending. The climb is not kept as momentum. `Locomotion::traversing`
  remembers it, so the first step after the window re-grounds with the
  step-down probe instead of the airborne one.
- **`Locomotion::mode: MoverMode { Ground, Fly { hover }, Swim { surface } }`**,
  set by gameplay. `Fly` and `Swim` spring the velocity in 3D (the
  intent's Y is used; `MotionState::velocity` keeps Y only in these modes)
  and apply vertical root motion. `Fly { hover: Some(h) }` keeps the feet
  `h` above the ground cast by a critical spring on the height error in
  the ground's moving frame: the slope under the motion (from the hit
  normal) is fed forward, so ramps are followed without lag (a plain
  height spring lagged 0.40 m on a 10° ramp at 6 m/s); with no ground
  within `h + step_down` it holds altitude. `Swim { surface }`: the
  adapter gives the water surface height above the feet each step (water
  queries are M6); the feet stay `MoverParams::swim_depth` (default
  1.35 m, from the profile 0.75 × height) below it, the bottom stops them.
  Switching a flyer to `Ground` in the air turns its vertical velocity
  into the fall speed. Free movement costs one extra up-sweep per rising
  character and a ground cast every step (hovering looks `h + step_down`
  down).
- **Attitude**: `Locomotion::attitude` (pitch, roll in the heading frame)
  accumulates `Drive` root motion's `RootDelta::tilt` (clips that extract
  attitude); undriven it holds while flying or swimming and levels out on
  the ground with the facing half-life. `render_rotation()` = facing ×
  attitude; the graph's character space stays the heading frame.
- **`LocomotionParams`** gains `vertical_speed`, `pitch` (flight-path
  angle) and `bank` (coordinated turn, `atan(speed · turn rate / g)`,
  ±60°): flyers choose or blend banked clips by them (authored first,
  rig-profile.md open question 2).
- **`MoverParams::from_profile(&RigProfile)`**: bipeds radius height / 6;
  other templates the M3 stand-in for a horizontal body, an upright
  capsule half the body length across (radius `body_length / 4`, at least
  half the support width); height from the measures (at least 2 × radius);
  step up/down half a support leg (the human 0.45 m at a 0.9 m leg). The
  M4 physics controller adds the horizontal capsule.
- **Measured** (`cargo test -p sf_controller --all-features`): traversal
  onto a 1 m box lands within 2 cm and grounds, without the flag the box
  blocks; flyer stops 0 mm under a ceiling and on the floor; hover error
  0 mm on flat ground and ≤ 1.9 cm on a 10° ramp at 6 m/s (gate 5 cm);
  swimmer floats at exactly surface − `swim_depth`; end to end
  (`tests/graph_drive.rs`): a warped mantle onto a 1.2 m box ends within
  1 mm, the capsule never inside the box.

**Crates checked (2026-10-11).** Bevy 0.20 has no character controller or
scene queries without a physics plugin. Rapier's
`KinematicCharacterController` (`rapier3d` 0.36) does collide-and-slide,
autostep and snap-to-ground, but needs rapier's `QueryPipeline` (the
physics engine's collider sets and broad phase) and can't see our terrain
heights; pulling a physics engine for queries in M3 would pre-empt the M4
engine decision (ARCHITECTURE §11). `bevy_tnua` is a Bevy plugin on a
physics backend (an M4 candidate). Used: **parry3d 0.31.1** (Apache-2.0,
2026-09, no Bevy; its math is glam 0.33 through `glamx`, the same glam as
ours, so no conversions), with default features off
(`required-features`, `std`, `alloc`). Its `Bvh`, `TriMesh`, ray casts and
`cast_shapes` are the whole static query world.

## Context probes, traversal and creature modes (anim-plan, 2026-10-11; traversal and modes built 2026-10-12, see above; context probes built 2026-10-12, see "Context probes as built")

Design for animation.md "Next-gen roadmap" layers 1 and 3 and for
rig-profile.md's creatures.

**Context probes** (M3). Batched like ground probes, on the same
`GroundProbe` implementations (terrain + `StaticWorld`, later
`PhysicsProbe`), so they move into M4's physics world unchanged:

```rust
pub enum ContextQuery {
    /// Capsule sweep along the trajectory up to `range`; on a hit, a column of
    /// down-casts over the obstacle gives its top and depth.
    ObstacleAhead { range: f32 },
    /// Down-cast `reach` ahead of the character from `max_height`, then an up
    /// clearance sweep: a ledge the hands can grab or the feet can step on.
    Ledge { reach: f32, max_height: f32 },
    DropAhead { range: f32, min_drop: f32 },
    WallBeside { side: Side, range: f32 },      // hand-on-wall, cover later
    Ceiling { height: f32 },                    // crouch
}
pub struct ContextHit { pub kind: Affordance, pub point: WorldPos, pub normal: Vec3, pub height: f32, pub depth: f32, pub clearance: f32 }
pub enum Affordance { Step, Vault, Mantle, Climb, Blocked, Ledge, Drop, Wall, LowCeiling } // classified by height × the character's measures
```

They run only at A0, only when the trajectory is blocked or the graph
asks (a state flag), at most 4 per character per update. Results feed
graph parameters, warp targets (`WarpTarget`, animation.md) and IK targets.
Thresholds scale with the rig profile's `measures` (a step for a guar is a
vault for a scrib), following GASP's traversal checks (traces, then height
and depth classified, then a chooser picks the clip and motion warping
aligns it).

**Context probes as built** (probes, 2026-10-12; `sf_controller/src/context.rs`). As designed above,
with these specifics:

- `ContextProbes::run(&[ContextRequest { feet, dir, params, query }], probe, out)`: each request
  is a phase machine; all requests' casts go in one `GroundProbe::probe` call per stage, at most
  four stages. Results are `Option<ContextHit { kind, point, normal, height, depth, clearance,
  landing }>`; `point` is the lip (obstacles, ledges), the edge walked off (drops) or the contact.
- `ContextParams::from_mover` scales thresholds with the capsule (GASP mannequin ratios): vault
  ≤ 0.7 h and ≤ 0.33 h deep with a lower far floor, mantle ≤ 1.5 h with ≥ 0.6 h room on top,
  climb ≤ 2.5 h (the column's start), crouch 0.6 h, hang 1.15 h below the lip, one radius off the face.
- `ObstacleAhead`: capsule sweep from a quarter step up to the head (lower is a curb), then three
  down-casts from the climb limit at 5 cm, 15 cm (flat top, not a ramp: a rising top behind a
  walkable contact normal is the mover's) and 1.5 vault depths past the face, then a clearance
  ray on top, a mid-face ray (only when the sweep touched above it; a miss is an overhang: an
  underside ray gives `LowCeiling` if a crouch fits under, else the slab is mantled) and a back
  ray that measures the depth exactly. A top below the sweep's contact means the column started
  inside a taller wall: `Blocked`.
- `Ledge`: down-cast `reach` ahead plus a ray at `max_height` (a wall there: no ledge), then a face
  ray just under the top (lip, normal) and a clearance ray. `ContextHit::hang_point` gives the
  hanging root and yaw; `warp_target(inset)` the root on top (mantle, step) or the landing (vault).
- `DropAhead`: four down-casts over `range` plus a wall ray; a drop is a fall between samples
  steeper than `step_down` plus the slope limit's rise (stairs and walkable slopes are not drops),
  continued while the samples keep falling (a cliff slope spans several); a back ray finds the edge.
- `WallBeside`: a chest-height ray; walls are |normal.y| < 0.3; `wall_tangent(dir)` is the run
  direction. `Ceiling`: an upward sphere cast from a step up. Cover is not built.
- Verified: classification table (0.3–5 m boxes, several depths), no room on top, ramp ignored,
  ledges at 1.5/2.0/2.4 m (lip ≤ 1 cm, hang point), stairs (step up, no drop down), overhangs
  (duck, mantle, ignore), walls, drops off boxes and a 70° terrain cliff (20° walked), batched =
  single; `graph_drive.rs`: a probed mantle target drives the `y=warp` window and the feet land
  ≤ 2 cm from it. Timing (release, town of `crowd_step_timing`): 150 blocked obstacle probes
  0.79 ms single-thread, 150 × 4 queries 1.1 ms; over the 0.2 ms budget unless gated (A0, blocked
  only: 40 × 5.3 µs ≈ 0.21 ms). Next: chunk over the compute pool in the plugin, cache per target.

**Traversal flag** (M3 with warping, used by M4 actions; built as
`RootMotion::traversal` rather than on `MoveInput`, since it qualifies the
root motion it travels with). `MoveInput` gains `traversal: bool`, set while a `y=warp` warp window is open: ground
snap and gravity are suspended and the full root delta, vertical included,
is applied (today `mover.rs:251` zeroes it), then the character
re-grounds with the normal probe. Collision stays on (the capsule may be
shrunk by the clip's curve, UE-style, if mantles clip through edges).

**Mover modes** (M3 for creatures). `MoverMode { Ground, Fly { hover:
Option<f32> }, Swim }` per character. `Fly` takes a 3D desired velocity
(intent's Y is used), no ground snap, an optional hover height kept by a
spring above the ground probe (floating creatures), and reports bank and
pitch for the graph. `Swim` is the same inside water volumes, with a
surface clamp (water queries come with M6; until then an adapter-given
water height). Long-bodied creatures use an upright capsule of half their
body length in M3; M4's physics controller adds a horizontal capsule
(rig-profile.md "Mover by template").

**Interaction points** (M4, `sf_interaction`): authored or generated slots
`{ transform, tag, approach: WarpTarget, claim }` on objects (benches,
doors, levers, counters), as UE Smart Objects. The path follower walks to
the approach point, the graph plays the tag's clip, warping aligns it.

Verification: classification table tests (boxes 0.3–2.5 m, ledges with
and without clearance, drops) per measures; batched = single; a warped
mantle onto a 1.2 m box in `tests/graph_drive.rs` style (final feet within
2 cm of the top, no penetration of the box by the capsule); a hovering
creature holds its height within 5 cm over terrain at 6 m/s.

## Local avoidance (ORCA; orca, 2026-10-12)

Industry-audit gap G2, job M3-b. `sf_controller::avoid` (`Avoidance`, `AvoidAgent`, `AvoidParams`,
`AvoidRole`), Bevy-free, about 500 lines with docs.

**Canonical technique.** Optimal reciprocal collision avoidance (ORCA: van den Berg, Guy, Lin,
Manocha, "Reciprocal n-Body Collision Avoidance", ISRR 2009, Springer STAR 70, 2011), the
velocity-level method behind RVO2, UE's Character Movement RVO and, in sampled form, Detour
Crowd. Each neighbour within `neighbour_dist` (5 m, closest 8) contributes a half-plane of
velocities that stay collision-free for `time_horizon` (2 s) if both sides take their share; a
2D incremental linear program picks the velocity closest to the preferred one inside the
speed disc, and when the half-planes leave no room a 3D program minimizes the largest
violation (paper section 5). Written from the paper; no OpenMW code.

**Where it runs.** Per frame, after intents are written (player input, `PathFollower`,
scripts) and before `KinematicMover::step_batch`: `Avoidance::solve(&agents, &params, dt, &mut
desired)` takes each character's feet, current velocity (`MotionState::velocity`, what the
others see), preferred velocity (the intent's `desired_velocity`), radius, max speed, role,
priority and direction of travel, and writes back the desired velocity (vertical part kept for
flyers and swimmers). Avoidance steers the intent and the springs smooth it; walls stay the
mover's collide-and-slide, and `separate` remains the last-resort push for overlaps ORCA
can't prevent (it works on velocities only, so a crowd pinned against a door frame can overlap
by centimeters). UE does the same: RVO runs inside the movement component on the requested velocity.

**Rules on top of the paper** (each tested):
- *Player non-reactive* (`AvoidRole::NonReactive`): its velocity passes through unchanged and
  every other agent takes the full avoidance against it (RVO2's and UE's avoidance weights).
- *Give-way* for idle characters (preferred speed under `idle_speed`, 0.1 m/s): against a
  moving agent the idle side takes `yield_weight` (9) to the walker's 1, so standing NPCs step
  aside and walkers keep their line, as OpenMW's "NPCs give way" behaves (from observation).
- *Slack split.* The paper and RVO2 apply one responsibility share to the whole ORCA line, so a
  pair that is *not* on a collision course also splits its slack by that share: a walker with a
  10 % share against a standing NPC was capped at 0.36 m/s from 5 m away. Here the share applies
  to a required change and the slack is split the other way round; each pair's two parts still
  sum to the full `u`, so the reciprocal guarantee holds.
- *Doorway and corridor deadlocks* (ORCA's known pitfall: symmetric opposing streams meet in a
  choke point and stall). Of two agents walking against each other (directions of travel more
  than 90° apart), one yields (`yield_weight`): the lower `priority`, or for equal priorities a
  fixed heading rule (the one travelling toward +X when the pair runs mostly along X, else toward
  +Z), a traffic-rule convention, so a whole stream yields together. The rule needs the route's
  direction (`AvoidAgent::travel`, the path segment): the steering direction toward a doorway
  waypoint cuts across the flow and flips the rule mid-door (measured: deadlock). Priorities must
  be coarse ranks (escort, guard, citizen); per-character values (an id hash) mix yielders into
  both streams and jam (measured). Plus a *keep-right bias* (0.15 rad turn of the preferred
  velocity with an oncoming neighbour), the RVO2 examples' random perturbation made
  deterministic, so open-space streams form lanes.
- *Determinism.* No randomness (dodgy picks a random direction for exactly coincident agents; we
  split by position, else +X). Neighbours are chosen by (distance, index), so `solve_one` (brute
  force) equals the batched result bit for bit.
- *Low-speed jitter* is left to the velocity springs (half-life 0.27 s). An idle dead zone was
  tried and removed: an idle agent only gets a nonzero result when standing still would collide,
  so a dead zone only suppressed needed sidesteps.
- *Known limit*: ORCA assumes agents reach the chosen velocity at once; our springs lag (UE RVO
  has the same issue with acceleration). The tests run through the real springs and pass; the
  acceleration-aware variant (AVO, van den Berg et al. 2011) is not needed so far.

**Industry check.** RVO2 (the paper's reference library) and `dodgy_2d`: one responsibility
share per line, random tie-breaks, obstacle lines for static polygons. UE: RVO in the Character
Movement Component (avoidance weight, groups to ignore) or the Detour Crowd manager (Recast's
crowd: adaptive velocity sampling around the preferred velocity plus path-corridor
optimization and wall segments from the navmesh), one or the other. Unity: `NavMeshAgent`
avoidance at a chosen quality level, with `avoidancePriority` (0–99, an avoiding agent ignores
lower-priority agents): the same idea as our ranks, used the same way (coarse classes, not ids).
Known ORCA pitfalls and how they are handled here: deadlocks in corridors and doorways
(yielding by priority or heading rule, keep-right bias; measured above), jitter at low speed
(springs), reciprocity broken by acceleration limits (tests run through the springs), and
agents pushed into walls the solver can't see (mover slides; navmesh paths in M10).

**Neighbours.** A per-frame spatial hash (Teschner et al. 2003) of the agents by counting sort,
cells of half `neighbour_dist`, searched ring by ring with an exact early-out once the 8th
closest is nearer than the next ring: dense crowds look at 3 × 3 cells, sparse ones at 5 × 5.
The static query world isn't used (it holds no dynamic agents).

**Measured** (`cargo test -p sf_controller --all-features avoid`, real mover on flat ground,
60 Hz, radius 0.3 m, walk 1.5 m/s):
- Head-on, two agents 10 m apart: pass with a 4.3 cm minimum gap, arrive in 7.15 s; without
  avoidance they overlap (gap < −0.5 m). Offsets and crossing angles (0–1.2 rad) pass too.
- Player head-on: the player's line drifts 0 mm, the NPC goes round (gap 4.2 cm).
- Give-way: a walker passing a standing NPC swerves 7.5 cm, the NPC steps aside 0.74 m, gap 4.9 cm.
- Doorway, 1.5 m wide in a wall, 4 + 4 agents swapping sides: all arrive in 13.2 s (equal
  priorities, heading rule) and 13.1 s (one side ranked higher); 3.4 cm worst overlap at the
  door frame. Without yielding, or with interleaved per-character priorities: no finish in 20 s.
  Without the keep-right bias: 12.0 s (yielding, not the bias, clears doorways).
- Batched = single on 150 scattered agents (sparse, dense, a 3 m cluster).
- Time (`avoidance_timing_150_agents`, ignored; `cargo test -p sf_controller --release
  avoidance_timing -- --ignored --nocapture`; one thread, battery, three quiet runs): 150 agents over 40 × 40 m
  0.054–0.056 ms median (≤ 0.068 p95); 150 in a 12 × 12 m square 0.087–0.089 ms median (≤ 0.112
  p95). Budget 0.1 ms; the test asserts the median, so it fails while another build loads the machine.

**Reuse order** (superfargoth CLAUDE.md rule 8). Bevy 0.20 has no avoidance. Crates: `dodgy_2d`
0.5.5 (MIT/Apache, a tested ORCA port of RVO2 with obstacles) was evaluated and not taken: it
pins glam 0.29 and rand 0.9 (we are on glam 0.33 and rand 0.10: two extra crate families in both
workspaces, against the compile-time rule §8b), calls `rand::random()` for coincident agents
(non-deterministic: breaks batched = single and replays), allocates its line lists per call,
takes neighbours as a `Cow<Agent>` slice per agent, and applies responsibility to the slack as
RVO2 does (the give-way problem above). Patching it means forking most of its agent code, and
its static-obstacle half isn't needed while walls are the mover's. `landmass` (navigation with
avoidance through dodgy) stays an M10 candidate. So: our own, from the paper, about the size of
dodgy's agent part. No new dependency.

**Plugging in.**
- `sf_controller_bevy` (to build): one system between "intents written" and the mover, gathering
  `AvoidAgent`s from the active ring's characters (`Locomotion`, `MoverParams`, intent); the
  player's controller marks itself `NonReactive`. One `Avoidance` per thread if it is ever
  chunked (0.05–0.09 ms single-threaded for 150, so not yet). Counters: agents, neighbours per
  agent, 3D-program fallbacks. Gizmos: the chosen velocity next to the intent's.
- `PathWalkScaffold` (`mw_ai`): `travel` = the current path edge's direction (node to node, not
  toward the steering point), `priority` 0 for ordinary NPCs, `max_speed` the gait's speed.
  Waiting at nodes makes an NPC idle, so it gives way; it doesn't walk back to its spot
  afterwards (the follower's next stop does). The scaffold's stuck timeout stays the last
  resort. When `sf_ai` (M10) owns intents, it fills the same fields.
- M4: the physics controller keeps this stage unchanged. ORCA obstacle lines for walls (the
  paper's static obstacles) are not planned: navmesh paths (M10) keep agents off walls.

## Bevy integration: `sf_controller_bevy` as built (walkers, 2026-10-12)

New thin crate `sf_controller_bevy` (bevy_app/ecs/math/time/transform; gizmos behind `debug`),
generic over the ground: `SfControllerPlugin::<G>` where `G: GroundProbe + Resource` is kept current
by the host (Bevyrrowind: resident terrain heights + a `StaticWorld` of the R0 statics'
`collision.cook` shapes, through `Nearest`).

- **Components.** `Mover { loco, params, frame_origin, root, animator, applied }` (the saved
  `Locomotion`; `frame_origin` is the parent frame's `WorldPos`, so the root `Transform` is
  `render_position - frame_origin` and the crate needs neither `big_space` nor streaming),
  `MoveIntent` (what the driver wants), `Avoider { priority, travel, max_speed }` (reactive ORCA
  agent), `AvoidObstacle { position, velocity, radius }` (non-reactive participant the host
  updates: the camera now, the player later). Movers without `Avoider` are non-reactive agents.
- **Order per frame** (industry order, below): `Update`: `ControllerSystems::Drive` (host
  intents, obstacles) → `Step` (one system: gather → `Avoidance::solve` over all movers +
  obstacles → `KinematicMover::step_batch` with one probe call per stage → write root
  transforms only when changed). `PostUpdate`: `Feed` inside `AnimSystems::Probe` (before
  Evaluate): `Animator::drive` velocity/acceleration/turn rate in character space, `moved`, the
  adapter-named parameters (`AnimParamNames`: moving, direction `x` right `y` forward, running,
  turn, speed), and per-leg ground samples from one batched ray probe at the graph's
  `probe_origins` (`CharacterFrame::foot_queries` → `GroundSample`). `foot_probes: false` feeds
  none (A/B).
- **Capsule-driven.** Morrowind clips are played in place; the graph's `SpeedMatch` scales
  playback to the mover's speed. `Mover::root` carries `Drive` root motion when a graph uses it.
- **Counters** (with `SfDiagPlugin`): `controller/avoid/ms`, `controller/move/ms`,
  `controller/write/ms`, `controller/feed/ms`, `controller/movers/count`; the app adds
  `controller/colliders/ms`, `controller/statics/count`. Animation has its own `anim/...`.
  `ControllerStats` holds the last frame's µs. No benchmark yet (owner on the machine).
- **Tests** (headless `App`): head-on walkers pass without overlap and transforms follow; a
  walker avoids a standing non-reactive obstacle (camera) that never moves; feed values in the
  character frame.
- **Adapter (Bevyrrowind `walkers.rs`).** Dressed characters get a `Mover` (+ `animator` =
  `CharacterBody::body`); `AI_W` wanderers get `PathWalkScaffold` (`Walker`) + `Avoider`
  (`travel` = current path segment, priority 0); NPC walk speed from the Speed attribute and
  `fMin/MaxWalkSpeed`; game hours at timescale 30; Idle2–9 → `idle_variant` + `fidget`; flying
  creatures `MoverMode::Fly { hover: 1.5 m }`; creature capsules scaled by the record scale.
  `ColliderHook` wraps the static/dynamic/door spawners: R0 activation inserts the placement's
  shape, deactivation removes it, one `rebuild` per frame with changes. `mw_humanoid` now has
  `profile_limbs: Support` + `FootPlacement(pelvis: Rig("hips"))`.
- **First run** (debug build log, not a benchmark): Balmora, 472–478 movers in the near rings,
  65–75 wanderers, 30–45 moving, all but 5 grounded, 2,056–2,430 R0 statics.

Industry check (2026-10-12):
- UE: `CharacterMovementComponent` has built-in RVO (`UAvoidanceManager::UpdateRVO` adjusts the
  requested velocity before the movement step, no navmesh needed); `DetourCrowdAIController`
  steers with dtCrowd (`update`: steering → `updateStepAvoidance` → integrate); Epic says use one
  or the other, not both. Our order (driver → ORCA → mover) is the same pipeline with one
  avoidance system. Mover 2.0 (`UNavMoverComponent`) keeps a basic RVO interface for Detour.
- Unity: `NavMeshAgent` computes `desiredVelocity`, avoids, then moves; with root motion either
  the agent owns the transform (animation follows `desiredVelocity`) or animation does
  (`updatePosition = false`, sync in `OnAnimatorMove` after `Update`); mixing both loses avoidance.
  We keep the mover as the single owner and feed animation from it (the first option), with
  `Drive` root motion entering the next mover step.
- OpenMW `AiWander`: idles between walks, picks pathgrid nodes within the wander distance of the
  spawn point (FIXME: of the current position), stuck detection removes unreachable nodes;
  non-NPC actors without a pathgrid wander near spawn on a small chance. `mw_ai` already follows
  the idle roll and has a stuck timeout (`WALK_SLACK`).

### Processing range, interiors, race scale, stuck handling (walkers2, 2026-10-12)

**Rule: no actor moves without ground and statics around it.** A mover simulates (driver,
avoidance, mover step, foot probes) only in the active worldspace and only while every
streaming cell within a margin of it (Bevyrrowind: 4 m, a walk step plus the capsule) is at
R0 (`Level::Active`, the level whose activation builds the static colliders; streaming.md
"Processing range"). Everywhere else it carries `Dormant`: it is parked where it stands (velocity
zeroed when the marker arrives), skipped by the step and by avoidance, fed zero motion and no
ground samples (so its graph settles into idle and foot placement holds the animated pose), its
driver paused (the scaffold's timers don't run), and its animation significance is lowered
(0.25) so the budget allocator keeps it at a low tier. Removing `Dormant` resumes it from the
same spot and the same walk. The activation ring is thus the collision ring by construction:
both are R0, and the margin keeps a walker from stepping into a neighbouring cell whose
colliders are not built. The test is `Scheduler::covers(ws, pos, margin, level)` (sf_world,
Bevy-free); the marker and its semantics are framework (`sf_controller_bevy::Dormant`); the
range policy (margin, significance, which worldspace) is the host's.

Industry check (2026-10-12): OpenMW's "actors processing range" (game units, 3584–7168, default
7168 ≈ 102 m) is the distance within which actors get AI, animation and physics updates; beyond
it they are not processed (AiTravel is the documented exception). Bethesda's engines (Oblivion,
Fallout 3/NV, community-documented) give each actor an AI process level by proximity: high
(every frame, loaded cells near the player), middle-high/middle-low (seconds apart), low (about
every two minutes, unloaded cells: schedules advance off-screen without physics). UE: the
Significance Manager feeds per-object significance, the Animation Budget Allocator turns it into
skeletal tick rates under a ms budget; AI and movement LOD are game-side on top of it. RAGE
(GTA, community-documented) demotes distant peds and vehicles to cheap "dummy" LOD. Common to all:
full simulation only where collision is loaded, a cheap or frozen tier beyond. We take OpenMW's
freeze (simplest correct rule) with Bethesda/UE-style animation demotion; an off-screen
schedule tier (Bethesda low process) waits for `sf_ai` schedules (M10).

**Ground per worldspace.** The host's ground must not mix worldspaces: interior coordinates
overlap the exterior's, and door prefetch activates other worldspaces' R0 cells. Bevyrrowind's
`MwGround` keeps one `StaticWorld` per worldspace and probes the active one (plus terrain only
when the exterior is active).

**Interiors.** `mw_ai` already keys interior pathgrids by cell name with absolute points; the
walker keys its grid by its worldspace (`CellKey::interior(ws)`; the interior worldspace id is
the cell name) and by its spawn cell outdoors.

**Race scale.** `Mover::body_scale` is the animated body's scale under the mover frame (race
height/weight, set by the host from the body `Transform`). The graph works in unscaled skeleton
space, so the feed scales the probe origins and probe lengths up into the mover frame and maps
samples back (points by 1/s, normals by s, normalized), and divides the fed velocity,
acceleration and `moved` by s (so `SpeedMatch` plays a short Bosmer's walk faster).

**Stuck handling** (`PathWalkScaffold`, Bevyrrowind, modelled on OpenMW's `AiWander` as observed
and documented: an actor that makes no headway towards its destination gives it up, stands, and
picks another). Each frame of a walk measures the flat distance to the current path node; when
it has not improved by 0.25 m for 3 s (ORCA yields are shorter), the walk is abandoned, the
actor idles 1–2 s and replans, re-rolling once a goal within 0.5 m of the abandoned one. The
old time backstop (2 × nominal + 5 s) stays. OpenMW also tries short evasive moves before
giving up; ORCA covers the dynamic case here, so we skip them. Counter: walks given up, in the
walkers report.

Known issues / next: `StaticWorld` anchor is the first activated cell (no rebase yet); creature
capsule from scale only (bounds next); no `separate` pass (ORCA keeps gaps; add for spawn
overlaps); doorway crowd is sparse in vanilla; dormant actors are not avoidance obstacles
(they stand outside the active cells active walkers may enter).

### Measured feed, gait hysteresis, turn in place (animfix, 2026-10-12)

Owner report: feet slide, "walking on the spot", and now and then a character whose legs stretch
far behind while the pelvis sits on the ground. Industry practice for capsule-driven locomotion
(OpenMW scales the movement animation by actor speed over the clip's own velocity; UE Lyra feeds
the *displacement* speed to speed warping / stride warping and distance matching; Unity's speed
sync) is to drive the graph from the distance actually travelled, not from the intent.

- **Measured velocity.** `Mover::measured` (character space, horizontal) is the step's `moved`
  rotated into the new frame over dt, after avoidance, collision slides and step handling,
  smoothed with a 0.05 s halflife (`MEASURE_HALFLIFE`); `Mover::measured_turn` likewise from the
  yaw delta. The feed sends these as `velocity`, `speed`, `move` and `turn`. A walker pushing
  into a wall (intent 1.5 m/s, no progress) is now told it stands still.
- **Hysteresis.** `ControllerSettings::{moving_speed, idle_speed}` (defaults 0.3 / 0.15 m/s) and
  `{run_speed, run_exit}` (default infinite = by gait): walk vs run is picked by measured speed
  when the adapter sets them. Bevyrrowind: moving 0.5 → idle 0.25, run 2.9 → walk 2.6 m/s.
- **Turn in place.** The walkers feed now names the graph's `turn` parameter (it was `None`), so
  a standing NPC that turns plays TurnLeft/TurnRight above 0.5 rad/s; foot locks follow the yaw
  in `moved` and unplant (animation.md, foot placement guards) when the legs twist too far.
- **Measured table** (male `base_anim`, cooked root motion, `mw_bake` real-install test):
  WalkForward 2.20 m/s (cycle 2.20 m / 1.00 s), WalkBack 2.21, WalkLeft/Right 1.73, RunForward
  3.18, RunBack 3.23, SneakForward 0.48, Idle/Turn 0. NPC walkers: `fMinWalkSpeed +
  Speed/100 × (fMaxWalkSpeed − fMinWalkSpeed)` = 1.86–2.43 m/s for Speed 30–70 (ORCA may ask
  up to 1.5×, and slows them in crowds). So the clamp `SpeedMatch 0.75–1.3` was not the main
  fault at cruise; at the slow end (avoidance, decelerating to a stop) it was: walk played at
  ≥ 1.65 m/s while the body moved 0.3–1 m/s. Now `SpeedMatch 0.5–1.5` + `StrideWarp`
  (0.6–1.6) cover 0.66–5.3 m/s of walk/run without sliding.
- **Root-motion-driven locomotion?** Rejected for walkers: the AI, ORCA and the mover need to own
  speed and path (stops at nodes, yields, collisions); Morrowind itself moves actors by stats and
  scales the clip (as observed and as OpenMW documents). Root motion `Drive` stays for hits,
  knockdowns and montages, unchanged.

### Orient to movement around obstacles (slopefix, 2026-10-12)

Owner report (WK-4): on some slopes one leg stops, the other takes short steps, the body slides.
Reproduced headless (`mw_graphs/tests/slope_walk.rs`: real humanoid graph, kinematic mover, measured
feed and foot probes as `sf_controller_bevy` runs them): plain ramps (15–30°, up and down) and stairs
were clean; a path meeting ground too steep to climb (or a wall) at a shallow angle was not. The
mover keeps only the motion along the contour (2.1 m/s at 75° leaves 0.54 m/s, the speed the in-game
trace showed), but the walker kept facing its goal, into the slope, so the graph was fed a slow
sideways walk: the WalkLeft/Right blend, whose cooked contact curves plant one foot almost never.

Fix (UE `bOrientRotationToMovement`, Detour and Unity agents face their movement): a surface that
turned the last step aside sets `Locomotion::deflect` (its horizontal normal; walls in `slide`,
too-steep ground in `finish`). While the intent still pushes into it and names no facing, the yaw
goal is the motion that surface leaves; nearly head-on (rest below `DEFLECT_FACING` = 0.1 of the
speed, under the moving threshold) the walker keeps facing its goal and stands. It clears when the
intent stops pushing into it or a step moves past where it was. Explicit facings (strafing) are
kept. Tests: `deflected_walkers_face_where_they_go` (sf_controller),
`walkers_turned_aside_by_steep_ground_keep_walking` + `walkers_keep_their_stride_on_ramps_and_stairs`
(mw_graphs, real install). Bevyrrowind logs a stride monitor (stances, skated stances, legs without
a plant for 2 s while walking; `BEVYRROWIND_STRIDE_TRACE=1` details).

Open: strafe clips' contacts (sf_anim cook: in-place clips use height only; MW WalkLeft/Right feet
are within 6% of a leg of their lowest point on 0–6 of 81 frames); some walkers on flat ground walk
at full speed with constant contacts (one leg 0, one 1) in state `walk`, cause not yet found.

## Action-layer changes (R11b, 2026-10-19)

Draft, awaiting owner review. The action layer ([action.md](action.md)) now decides every move and
owns jump, air and traversal timing. Only what the owner's decisions change is here; the rest of
this spec stands.

**Fixed tick.** Movers step on the fixed tick (60 Hz, the physics rate) with transform
interpolation for rendering, not in `Update` at frame rate. Reason: the chooser runs on the fixed
tick, and co-op prediction replays gameplay ticks from commands (action.md "Co-op readiness"). For
action moves the root delta comes from `sf_action` (baked root track at the action clock) on the
same tick, which removes today's one-frame root-motion handoff. Far tiers (A2/A3) may step every
second or fourth tick. This is required by the co-op net-ready rules (`../research/coop/README.md` §4 rule 1)
and supersedes physics.md "Controller" on this point; physics.md itself is a follow-up (not edited here).

**Jump and air** (R1 spec gap). Jump is set by height: launch speed from the wanted apex (mode
profile × stat band, action.md). Air control steers and brakes but never exceeds the launch's
horizontal speed (R5 adopt 9). Landing classes by fall speed (normal, roll, hard, lethal) are action
rules. Coyote time and jump buffer are action-layer timers in ticks.

**Momentum mover mode** (R1 adopt 9; the player in the FreeRun profile). A ground mode beside the
springs: speed builds up toward sprint along a curve instead of a spring, momentum carries through
landings and vaults (each rule's exit velocity), the turn rate falls with speed (view-turn damping),
and back-to-back jumps lose some speed (hop tax). Holden springs stay for NPCs and for the Normal
and Sneak profiles. Tuned on `flat-slopes` and `ledges`.

**Correction springs** (R8 adopt 2, Holden's "Code vs Data Driven Displacement"). The gameplay
capsule is the truth; the render body's root may sit off it while the clip catches up. The offset
decays toward zero with a halflife (position ~0.1 s, rotation slower), is **velocity-limited** (at
most ~0.5× the character's own speed, so feet don't skate at plants, starts and stops) and
**clamped** (a maximum distance and angle). Separate position and rotation dials. The same spring
hides co-op reconciliation corrections. Bevy-free math in `sf_controller`; applied to the render
transform by the Bevy shell, never fed back.

**Docked mover mode** (R5 adopt 5; ledges and ladders). `MoverMode::Docked { edge, s, side, offset }`:
- the character is on an `sf_interaction` edge chain (a ledge, or a ladder's vertical chain) at
  parameter `s`;
- gravity, ground snap and pushes are off; collision stays on;
- input inside a side cone moves `s` at the docked speed (from the clip's root distance, scaled by
  the stat band); edge ends and corners are probed ahead and stop or turn the move;
- the hang offset (hand joints vs root) comes from the clip at bake, not a constant;
- leaving is an action rule: climb up (Warp), let go (Air), drop to the ground, ladder exit.

**Context probes** (needed by action rules, built in the phases that use them): speed, vertical
speed and time-to-contact gating (R1 adopt 4, 8); a `WallAhead` query; back-ledge and landing
targets for vaults (R7 §5); state-gated probes when a rule asks; the player polled every airborne
tick. NPC gating (A0, blocked only) stays.

## Industry comparison

J9 (industry-audit.md), 2026-10-12, docs research only (`.sessions/j9m3.md`).

| Technique | What shipped engines do | Us | Verdict |
|---|---|---|---|
| Capsule mover | UE Character Movement Component: floor finding, step-up, walkable slope, root motion from montages applied by the component; Unity `CharacterController` (slope limit, step offset); Creation Engine: Havok character proxy (unverified detail); OpenMW: its own movement solver over Bullet shapes (collision shape type: axis-aligned box by default, rotating box or cylinder) | `KinematicMover`: probe-based ground snap, step up/down, slope limit, slide | **match**; M4 swaps in a physics capsule as planned |
| Animation-aware trajectory | Clavet (*For Honor*), UE GASP and Pose Search trajectories, Holden's spring-damper prediction | five predicted points from the same springs | **match** |
| Root motion handoff | UE: the movement component consumes the montage's root motion next tick; Unity `OnAnimatorMove` | the mover consumes last frame's `Drive` delta | **match** (one frame latency, the norm) |
| Turn toward movement | OpenMW "turn to movement direction" (lower body turns on side and diagonal moves, head follows the view) and "smooth movement"; UE orientation warping | `desired_facing` + `OrientationWarp` (animation.md) | **match** |
| **Local avoidance between characters** | UE: **RVO** avoidance inside the Character Movement Component (agents only, no navmesh) or the **Detour Crowd** manager (adaptive RVO sampling plus path-corridor optimization, navmesh-aware, slower); use one, not both. Unity: `NavMeshAgent` avoidance with quality levels (`obstacleAvoidanceType`) and `avoidancePriority` 0–99 (default 50; an avoiding agent ignores lower-priority agents). OpenMW: "NPCs avoid collisions" (evasion maneuvers, default off) and "NPCs give way" (standing NPCs step aside for moving ones, default on) | ORCA on the intent's desired velocity before the springs (`sf_controller::avoid`, from the paper), player non-reactive, idle NPCs give way, opposing pairs yield by priority or a heading rule, keep-right bias, per-frame spatial hash; the radius push stays as the overlap fallback | **match (G2 resolved 2026-10-12, M3-b)**: head-on passes with a 4.3 cm gap; 4 + 4 agents swap through a 1.5 m doorway in 13.2 s; 150 agents 0.054 ms (street) to 0.087 ms (dense square) single-thread. `dodgy_2d` 0.5.5 evaluated, not taken (glam 0.29 + rand 0.9 duplicates, random tie-break, per-call allocation, slack split by responsibility): "Local avoidance (ORCA)". Not built: Detour-Crowd-style path-corridor optimization and wall segments in the solver (navmesh, M10). `landmass` stays an M10 candidate |
| Movement LOD | UE: distant AI can walk on the navmesh instead of sweeping collision (`NavWalking`, unverified detail); Creation Engine process levels; OpenMW stops AI, animation and physics past "actors processing range" | open question 1 (A3 walkers on terrain height only) | **match** in plan |
| Stop and pivot prediction | UE's locomotion library predicts the ground-movement stop location from the movement component's braking (feeds distance matching; unverified function name) | trajectory points only | **GAP (P2)**: the springs give the stop point in closed form (a critically damped spring's remaining displacement); needed only with distance matching (animation.md open question 5). Open question 4 |
| Batched ground probes | UE sweeps per character on the game thread; Mass batches by processor | one batched probe stage per frame for every consumer | **match**, ahead of per-character sweeps |

Sources (checked 2026-10-12):
- UE avoidance (RVO vs Detour Crowd): https://dev.epicgames.com/documentation/en-us/unreal-engine/using-avoidance-with-the-navigation-system-in-unreal-engine ; https://dev.epicgames.com/documentation/unreal-engine/API/Runtime/AIModule/FCrowdAvoidanceConfig
- `dodgy_2d`: https://lib.rs/crates/dodgy_2d ; https://docs.rs/dodgy_2d ; RVO2: https://gamma.cs.unc.edu/RVO2/documentation/2.0/whatsnew.html ; `landmass`: https://docs.rs/crate/landmass/0.9.2
- (orca, 2026-10-12) `dodgy_2d` 0.5.5 manifest and `src/lib.rs` (glam 0.29.1, rand 0.9.1, `rand::random()` for zero-length relative positions, `Vec<Line>` per call): https://docs.rs/crate/dodgy_2d/0.5.5/source/ ; Unity `avoidancePriority`: https://docs.unity3d.com/ScriptReference/AI.NavMeshAgent-avoidancePriority.html ; `obstacleAvoidanceType`: https://docs.unity3d.com/ScriptReference/AI.NavMeshAgent-obstacleAvoidanceType.html ; ORCA paper: van den Berg, Guy, Lin, Manocha, "Reciprocal n-Body Collision Avoidance", Robotics Research (ISRR 2009), Springer STAR 70, 2011, pp. 3-19
- OpenMW game settings (processing range, turn to movement direction, NPCs avoid collisions, give way, collision shape type): https://openmw.readthedocs.io/en/latest/reference/modding/settings/game.html
- UE Motion Warping and Distance Matching: see animation.md "Industry comparison"

**Unverified**: UE `NavWalking` behavior and the stop-location prediction
function name (from memory); Unity `obstacleAvoidanceType` levels (only "quality vs cost" documented);
Havok character proxy in Creation Engine games.

## Open questions

1. Whether R1 visual-only NPCs (frozen per streaming.md) should keep
   walking short loops for life at distance (cheap: A3 tier, no probes,
   terrain height only), or stand.
2. Collision shapes for legacy models without collision geometry: LOD1
   triangles or convex decomposition (M4 decides; M3 uses triangles).
3. Sliding down steep slopes and pushing between characters: M4's physics
   controller, or the kinematic mover if M3's crowd shows the need.
4. Local avoidance: **resolved** 2026-10-12 (M3-b, "Local avoidance (ORCA)" above): it lives
   in `sf_controller` (UE puts RVO in the movement component), on the intent's desired
   velocity, with a per-frame spatial hash of the characters. Still open: the closed-form stop
   and pivot point from the springs for distance matching (P2, animation.md open question 5).
