# Locomotion: as-built history

Moved out of [locomotion.md](../locomotion.md) on 2026-10-26 (doc budget, S11). Append-only; grep it.

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
  (waits, ends) are approached on a constant-deceleration profile
  (as built R3-2, AN-2): commanded speed `min(speed, sqrt(2·a·(d − c)))`,
  `a` per gait from `MoverParams::arrival` (`ArrivalParams`, walk 6, run
  3.5 m/s²; `PathFollower::for_mover`), `c` the distance the mover's
  velocity spring still coasts if released now (`2v/y + a_s/y²`, from a
  copy of that spring driven by the command), so the walker comes to rest
  on the node. Below `cutoff_speed` (0.1 m/s) the command drops to zero
  and the stop arrives when the spring has settled: no creep. Walk
  1.5 m/s: 0.59 m, 0.87 s to < 0.1 m/s; run 3.5 m/s: 2.4 m, 1.5 s. The
  feed's `ControllerSettings::stop_speed` hands the graph to idle earlier
  while the intent asks for no motion. Stop clips / distance matching
  (Lyra) are a later step: vanilla has no stop clips. Waiting outputs the
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

**Foot metrics (anim slices 4 and 4b, 2026-10-22).** `sf_anim::foot_metrics` (`FootMeter`, Bevy-free) measures slide
while planted, sole penetration/float, stances. `gaits_plant_their_feet_on_flat_ground` (male, flat, walk 2.1 / run
3.7 m/s) gates every gait at <= 3 cm/stance and prints the stance lock drift (`FootStats::drift_*`: lock vs animated
foot). Worst-leg slide before: walk right 30, run back 55, run left 32, run right 36 cm/stance (others 0); after: 0.
Cause: sidestep/back clips' root outruns their planted ankles (WalkRight root 1.72, feet 0.92 m/s). Fix: the cook
stores `ClipMeta::stance_velocity` (root minus the slower of heel/toe's slide in stance); `stride_velocity()` uses it
where it differs from the root by > 20 % (RunForward 3.18 vs 2.59 stays on the root) and SpeedMatch plays by it; the
humanoid's SpeedMatch max is 2.2 (was 1.5: run strafes need ~2x playback; owner may trade it for slide). Stance drift (mean/max cm,
before -> after): walk left 10/26 -> 6/16, walk right 17/41 -> 6/22, run back 20/53 -> 8/14, run left 18/45 -> 11/24, run right 16/45 -> 12/25; forward gaits 12-14/23 unchanged.

**Feet on slopes and stairs (anim slice 5, 2026-10-22).** `feet_meet_the_ground_on_ramps` gates the sole
(heel and toe as a rigid foot: deepest point, lowest-point gap) within 2 cm on planted frames. `FootPlacement`
now keeps the sole's clearance along a tilted ground's normal and probes each foot's tip too (`Leg::toe`,
slots after the legs in `probe_origins`): a tip over a higher step lifts the foot until it clears. Worst leg,
cm, before → after: ramps 10/20/30° sole pen 1.6/2.0/2.6 → 1.5/1.3/1.0 (flat 1.5: the clip's own offset);
stairs 0.15/0.30 up toe 16.5 → 5.3 (the lock keeps the heel-strike height while the toe rolls onto the
tread), down 1.5; steep 0.25/0.22 up/down 37/68 unchanged: a 1.1 m walk step spans five treads (1.25 m of
rise), beyond the leg. Stairs stay `#[ignore]` (`feet_meet_the_ground_on_stairs`) until grade-aware stride.
