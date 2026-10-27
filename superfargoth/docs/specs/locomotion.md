# Locomotion: intent, trajectory, ground probes, kinematic mover

Status: `sf_controller` built (2026-10-11; ORCA local avoidance 2026-10-12); `sf_controller_bevy` built (player and walker movers in game, ROADMAP 2.1)
Crates: `sf_controller` (Bevy-free: intent, spring motion, trajectory,
`GroundProbe` with terrain and static probes, the kinematic mover's logic, ORCA local avoidance,
path follower, `collision.cook`), `sf_controller_bevy` (to build: schedule,
batching, gizmos, counters in M3; physics controller in M4)

<!-- agent-summary -->
> **Agent summary (2026-10-26).** Status: `sf_controller` built (2026-10-11; ORCA 2026-10-12; traversal, creature modes and context probes 2026-10-12). Player controller works in game (ROADMAP 2.1; PL-1 awaiting owner); 3.1 traversal per R11 open.
> Crates / key types: `sf_controller` (Bevy-free: `GroundProbe`, kinematic mover, ORCA avoidance, path follower, `collision.cook`), `sf_controller_bevy` (`Mover`, `MoveIntent`, `Avoider`).
> Decisions: spring-damped velocity/facing predicts trajectory ("Canonical technique"); animation follows the capsule except `Drive` actions; kinematic ground snap with foot IK on real steps; ORCA lives in `sf_controller` ("Open questions" 4); action layer decides moves ("Action-layer changes").
> Read next: "Ground probes"; "Movers"; "Context probes, traversal and creature modes"; `action.md`.
<!-- /agent-summary -->

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

**Player feel (2026-10-27, owner: the 0.27 s springs "drive like a car").** Design: a player wants
a fast speed ramp, a faster stop, and rate-capped turning where the motion follows the facing
(ELDEN RING Combat Rewrite, MIT: run speed in ~0.15 s, 1080 deg/s running, 480 sprinting; a
reversal turns at speed instead of braking through zero). As built, two opt-in `SpringParams`
fields (defaults keep Holden's springs, so NPCs are unchanged): `halflife_brake` (the velocity
half-life while the goal speed is below the current one) and `turn: Option<TurnRates>` (rad/s per
`Gait`). With `turn`, `MotionState::advance` turns the facing toward its goal at the gait's rate
and holds the horizontal velocity as a speed (1D spring, accel or brake half-life) along a
heading that turns toward the goal direction at the same rate; from rest the heading starts on
the facing when the facing goal is the travel direction (no `desired_facing`). Prediction then
sub-steps at 60 Hz (no closed form). A 0 half-life and an infinite rate snap (a future classic
preset). `sf_action` asks for the gait's forward speed whatever the current facing (the body
turns to face the stick). Player numbers: `mw_player_bevy::feel` (visual check PL-4).

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

Moved to [history](archive/locomotion-history.md#as-implemented-in-sf_controller-2026-10-11).

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

Moved verbatim to [orca.md](orca.md#local-avoidance-orca-orca-2026-10-12) (current design, own file).

Moved to [history](archive/locomotion-history.md#bevy-integration-sf_controller_bevy-as-built-walkers-2026-10-12).

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

**Launch** (as built 2026-10-21). `Locomotion::launch(speed)` sets the vertical speed and clears
`grounded`; `sf_action`'s `ActionOutput::launch` maps onto it one to one. While an airborne body
rises (`vertical_speed > 0`) the mover never snaps it to the ground (`rise_ballistic` in
`mover.rs`): gravity integrates exactly, a capsule sweep up stops the head and zeroes the upward
speed, and ground the step's own cast finds above the feet only keeps them out of it (a ledge met on
the way up). It lands through the normal airborne cast once descending. This is the common
controller rule: Rapier snaps to ground only when not moving up, UE `LaunchCharacter` enters
falling mode, Unity KCC `ForceUnground`. No time window is needed because snapping only runs while
`grounded` is set. Tests in `src/tests/launch.rs` cover the arc, apex and air time, a running jump,
a head bump, and a jump onto a 0.8 m box. `Locomotion` (with `GroundHit`, and `RootDelta` through a
remote derive) is serde data and round-trips through postcard.

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
| Stop and pivot prediction | UE's locomotion library predicts the ground-movement stop location from the movement component's braking (feeds distance matching; unverified function name) | trajectory points only | **GAP (P2)**: the springs give the stop point in closed form (a critically damped spring's remaining displacement; `PathFollower` arrival uses it since R3-2); the animation side needs it only with distance matching (animation.md open question 5). Open question 4 |
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
