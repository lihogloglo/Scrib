# Physics: rigid bodies, items, carry/throw, ragdolls, cloth (M4)

Status: draft; decisions owner-confirmed 2026-10-12 (ARCHITECTURE §10). Built: P1-P10a, R3 (as-built logs,
measurements, the backend survey and superseded sketches: [archive/physics-history.md](archive/physics-history.md)).
Crates: `sf_physics` (new, Bevy-free: world, bodies, queries, `body.cook`, `items.check`),
`sf_physics_bevy` (new, thin plugin), `sf_ragdoll` (new, Bevy-free: `ragdoll.cook`, drives, balance,
get-up), cloth lattice in `sf_anim` (procedural op) + `cloth.cook` in `sf_character`; touches
`sf_controller` (`PhysicsProbe`, pushes), `sf_anim_graph` (`PoseInput` blend), `sf_world_bevy` (hooks).
Adapter spec (first content pack): `../../../bevyrrowind/docs/specs/physics-adapter.md`.

## Goal

ARCHITECTURE §9 M4: "Knock a table's contents over; a dead guard ragdolls; robes move."

- **Rigid bodies.** Loose items placed by content are real rigid bodies. They start asleep, so authored
  scenes stay exactly as authored until something disturbs them, then fall, roll, stack and slide.
- **Controller on physics.** The M3 kinematic mover keeps moving characters; it now sees dynamic
  bodies, pushes light ones, is blocked by heavy ones, and reports impacts.
- **Carry and throw.** Grab an item in reach, carry it without jitter at any frame rate, rotate it,
  drop or throw it, within limits from the holder's strength; items too heavy to carry, and dead bodies (by a
  limb), are dragged.
- **Ragdolls.** Generated per skeleton from the rig profile; limp on death, powered (following the
  animation through joint drives) for hit reactions, stumbles and knockdowns, with a blended get-up.
- **Cloth.** Robes, skirts and capes swing and collide with the legs.
- **Large worlds.** Exact under the floating origin (world-coordinates.md "Physics contract").

Non-goals for M4: buoyancy (M6, needs water heights), destruction and fracture, vehicles, world
cloth (banners, flags: later), mesh-deforming cloth on the GPU (later, below), learned controllers,
networking and cross-platform lockstep determinism, combat hitboxes (M9 uses this world's queries).

## Canonical technique

- **A third-party rigid-body engine behind our own API**, stepped at a **fixed rate with render
  interpolation** (Fiedler, *Fix Your Timestep!*, 2004; Unity `Rigidbody.interpolation`, UE async
  physics). The engine is **Rapier** (`rapier3d` 0.36.1, Apache-2.0, no Bevy dependency), wrapped by
  the Bevy-free `sf_physics`. Rapier types never leave that crate.
- **Floating origin for physics**: the simulation runs in a local f32 frame around an anchor cell and is
  **rebased by whole grid cells** (PhysX `PxScene::shiftOrigin`, used by UE4's world origin rebasing;
  Havok's world shift). Only R0 simulates (streaming.md), so f32 is ample.
- **Sleeping from the start**: placed items are inserted asleep and wake on contact (Havok/Bethesda
  practice), with soft depenetration so authored overlaps never explode.
- **Kinematic character controller** with explicit push impulses (UE Character Movement Component,
  Jolt `CharacterVirtual`, PhysX CCT): characters are not solver bodies.
- **Powered ragdolls** with implicit (stable) PD joint drives (Tan, Liu, Turk, *Stable
  Proportional-Derivative Controllers*, IEEE CG&A 2011; animation.md "Layer 4"), per-region strength,
  and an inertialized get-up.
- **Character cloth as a bone lattice solved with XPBD** (Macklin, Müller, Chentanez, *XPBD*, MIG 2016;
  Macklin et al., *Small Steps in Physics Simulation*, SCA 2019) plus the standard character-cloth
  constraints: long-range attachments (Kim, Chentanez, Müller, SCA 2012), max distance to the animated
  pose, capsule colliders, simulation in character space with inertia scaling (UE Chaos Cloth, NvCloth,
  Havok Cloth, Jolt skinned soft bodies). The robe mesh is skinned to the lattice's joints, as Magica
  Cloth's BoneCloth and UE's AnimDynamics/Kawaii Physics do, so it reuses our skinning path.

### Backend decision (rule 9: Bevy, then crates, then our own)

**Bevy 0.20** has no physics engine, scene queries or interpolation, so it came down to crates
(survey of 2026-10-12 in the [archive](archive/physics-history.md#backend-decision-rule-9-bevy-then-crates-then-our-own):
`bevy_rapier3d`, `avian3d` 0.7/0.8, an avian fork, Jolt bindings, `bevy_tnua`).
**Chosen: `rapier3d` directly**, because it is on our Bevy today and never blocks a Bevy upgrade;
it shares our `parry3d` 0.31.1, so the M3 static world moves in unchanged; it has the per-axis
motors and articulations powered ragdolls need (avian has no spherical or generic joint motor); the
floating-origin contract is ours to implement exactly instead of fighting a `GlobalTransform` sync
under big_space; and physics logic stays Bevy-free (§8b, rule 7). avian stays the fallback if
Rapier misses the gates (the `sf_physics` API hides the engine, so a swap touches one crate).

**Determinism.** Same binary, same inputs, same insertion order → same result with the serial solver
(insertion follows index order, which is source-order independent: streaming.md). That is enough for
replay tests and the `items.check` bake. Cross-platform `enhanced-determinism` (no SIMD) stays off.
Rayon `parallel` stays off until a measured step exceeds its budget (Bevy's task pools already use the
cores; oversubscription is a known cost; P8b measured it slower).

## Industry comparison

| Technique | What shipped engines do | Us | Verdict |
|---|---|---|---|
| Engine | UE5: Chaos (own). Unity: PhysX (plus Unity Physics / Havok for DOTS). Creation Engine (Skyrim, Fallout 4, Starfield): Havok. RAGE: own physics + Euphoria. Decima: Jolt. Frostbite: Havok. OpenMW: Bullet, for actor collision only (no rigid-body items) | Rapier behind `sf_physics` | **match** (third-party engine behind our own API, as Creation and Decima) |
| Large worlds | PhysX `PxScene::shiftOrigin` (UE4 world origin rebasing); Jolt double-precision mode (`RVec3`, f32 internals relative to a base offset; [Jolt "Big worlds"](https://jrouwe.github.io/JoltPhysics/index.html#big-worlds)); Chaos uses doubles with UE5 Large World Coordinates; Havok world shift | f32 frame around an anchor cell, rebased by whole 256 m cells, exact | **match** (PhysX approach) |
| Simulation region | Creation Engine: Havok runs in the loaded grid (uGridsToLoad), unloaded cells freeze; OpenMW: actors processing range; UE World Partition streams physics with the actors | R0 only; bodies freeze into the placement overlay at R0's edge | **match** |
| Loose items | Skyrim/Fallout 4: Havok bodies placed at rest, wake on contact (interpenetrating placements fling: "Havok fling"); Oblivion first did this; OpenMW: items never move | Asleep until disturbed, bake-time support check, capped depenetration speed | **match**, better than Creation on authored overlaps |
| Item shapes | Creation: authored `bhk` shapes; UE: auto convex decomposition (V-HACD) or primitives in the Physics Asset; Havok: hulls and primitives | Authored collision if present, else primitive fit, single hull or decomposition, baked | **match** |
| Mass ratios | PhysX and Havok guidance: keep stacked mass ratios near 10:1; engines clamp or use extra iterations | Simulated mass clamped to a range; gameplay weight unchanged | **match** |
| Fixed step + interpolation | Unity `FixedUpdate` (50 Hz) + interpolation; UE async physics; Creation's Havok step was frame-tied (physics broke at high fps until capped) | 60 Hz fixed, interpolated, held items interpolated in holder space | **match** |
| Character controller | UE CMC (kinematic sweeps, push force factor on physics objects); Unity `CharacterController` (PhysX CCT); Jolt `CharacterVirtual` (kinematic, pushes with a max strength); Creation: Havok character proxy | Keep `KinematicMover`; capsule as a kinematic body for others to hit; capped push impulses | **match** |
| Carry/throw | Skyrim grab (spring to a point ahead of the camera); Half-Life 2 `+use` and gravity gun (motion controller toward a "shadow" pose); Amnesia (drag with mass) | Soft 6-DOF joint from a kinematic hand, force cap, drag when too heavy, throw impulse | **match** |
| Ragdoll setup | UE Physics Asset (auto capsules per bone, swing/twist limits); Unity Ragdoll Wizard; Havok `hkaRagdollInstance` | `ragdoll.cook` from `RigProfile` and skin extents, creature-aware | **match** |
| Powered ragdoll | Euphoria (GTA IV/V, Red Dead): behaviours on a powered ragdoll; UE Physical Animation Component (local and world-space drives) and Physics Control Component; Havok powered constraints; Frostbite (Sachania, GDC 2018); Unity `ArticulationBody` drives | Implicit joint motors from the animated pose, per-region strength, world pelvis drive, balance → stumble/fall | **match** for drives; **gap** (later): Euphoria-style behaviours (brace, grab, protect head) |
| Get-up | UE and most action games: face up/down from the pelvis, pick a get-up clip, blend from the ragdoll pose | Same, through `PoseInput` + inertialization | **match** |
| Character cloth | UE Chaos Cloth (max distance, backstop, long-range attachments, component-space sim with fictitious inertia); NvCloth; Havok Cloth; Jolt skinned soft bodies (max distance + backstop); bone-driven skirts (UE AnimDynamics, Kawaii Physics, Magica Cloth BoneCloth) | Bone lattice with XPBD, tethers, max distance, capsules, character-space inertia | **match** for robes and skirts; **gap** (later, P2): mesh cloth (per-vertex proxy deformer) for capes and banners |
| Collision filtering | UE object channels + responses; Unity layer matrix; Havok collision filter | Layer bits + per-pair masks | **match** |

## Data model

### `sf_physics` (Bevy-free; rapier3d, parry3d, sf_coords, sf_controller for the probe trait and `CollisionMesh`)

```rust
pub struct PhysicsWorld { /* rapier sets, pipeline, anchor: WorldPos, interpolation state */ }
pub struct PhysicsWorlds(HashMap<WorldspaceId, PhysicsWorld>);  // only the active one steps

#[derive(Clone, Copy)] pub struct BodyId(u32, u32);     // index + generation; never a rapier handle
pub enum BodyKind { Fixed, Dynamic { asleep: bool }, Kinematic, Held }   // Held: fixed until grabbed or impulsed
pub struct BodyDesc {
    pub kind: BodyKind, pub pose: WorldTransform, pub shape: ShapeRef, pub layer: Layer,
    pub mass: Option<f32>,          // kg (simulated mass, clamped); None = shape volume × density
    pub ccd: bool, pub damping: (f32, f32), pub user: u64,   // user: PlacementId low bits / entity
    pub extra_iterations: u8,       // ragdolls, held items
}
pub enum ShapeRef { Mesh(Arc<MeshShape>), Body(Arc<BodyShape>), Capsule { r: f32, half: f32 },
                    Box(Vec3), Sphere(f32), Heightfield(Arc<HeightChunk>) }
bitflags! pub struct Layer: u32 { STATIC, TERRAIN, DEBRIS, PROP, CHARACTER, RAGDOLL, POWERED, HELD, SENSOR, PROJECTILE }
```

- **Frame.** All engine positions are `anchor.delta_f32(pos)`; every read returns `WorldPos`.
- **Shapes.** `ShapeRef::Mesh` is one parry `TriMesh` per model from the `collision.cook`
  `CollisionMesh`, shared with the controller's `StaticShape` (one `Arc`), with cached scaled copies;
  `ShapeRef::Body` builds a hull or compound from `body.cook`.
- **Settle on insert.** A body inserted asleep is locked on every axis for one step, so the contacts
  rapier wakes can't move it and its own sleep pass re-sleeps it; rebases re-settle every sleeper the
  same way: 0 wakes, bit-exact poses (P1).
- **Layers.** `DEBRIS` = light, small dynamic items (below `debris_mass` 15 kg and `debris_extent`
  0.6 m by default): characters push them and never stand on or are blocked by them. `PROP` = everything
  else dynamic: blocks movers, is pushed with a capped force. `HELD` = the carried item (no contact with
  its holder's capsule). `RAGDOLL` bodies skip contacts with jointed neighbors.
- **Simulated mass** is clamped to `[0.2, 60]` kg by default (a cup under a cuirass is 300:1 raw); the
  gameplay weight stays in records. Thin shapes get a 4 mm contact skin.
- **Events** (drained per step): `Woke`, `Slept`, `Hit { target, by, impulse, point, direction }` (first contact on a `hit_layers` body, below), `ContactForce { a, b, impulse, point }` above a
  per-layer threshold (damage later, knockback), `LeftCoverage(BodyId)`, `FellOut(BodyId)`.
  `Contact { a, b, impulse, speed, slide, point, normal, surfaces }` (2026-10-22, audio.md "Impacts"):
  after each step, every touching pair of an awake dynamic body on `contact_report_layers` (default
  `MOVING`) whose normal speed change `impulse·(1/m_a + 1/m_b)` or contact slip reaches
  `contact_report_speed` (0.3 m/s; resting support is `g·dt` = 0.16). `surfaces` are
  `sf_controller::SurfaceId`s: the touched triangle's on a mesh, `BodyDesc::surface` otherwise;
  `sf_physics_bevy` replaces a terrain chunk's with the terrain's own (`HeightSurface::surface`).
- **Interpolation state**: previous and current pose per awake body; `pose_at(alpha)` lerp + slerp.

**Bake products** (feature `bake`, sf_bake steps):

| Step | Subject | Inputs | Output |
|---|---|---|---|
| `body.cook` | model | `mesh.cook` LOD0/LOD1, adapter `BodyRules::authored` (authored collision), settings | `BodyShape { parts: Vec<ShapePart>, volume, center_of_mass, unit_inertia, aabb, source }`; `ShapePart = Hull(points) \| Box \| Capsule \| Cylinder \| Sphere`, each with a local isometry |
| `items.check` | index page | page placements of class `Dynamic`, their `body.cook` products, statics' `collision.cook` | `ItemChecks` (`SFIC`): per item `hold` (`No`, `Unsupported` = held until grabbed: hanging lanterns, floating props; `Rules`; `NoShape`), `drop`, `rise`, `shift`; the `supports` statics (below) |
| `ragdoll.cook` | skeleton | skeleton, `RigProfile`, skin extents, adapter overrides | `RagdollDef` (below) |
| `cloth.cook` | (part model, skeleton) | cooked skinned part, `RigProfile`, adapter `ClothRules` | `ClothLattice` (below) |

`body.cook` shape choice, first match: authored collision (its convex parts; trimeshes are hulled);
a primitive (box, capsule, cylinder, sphere) when its volume is within 15% of the mesh hull's and it
covers the hull (bottles roll, swords lie flat); one convex hull when the hull's volume is within 25%
of the closed mesh's (or the mesh isn't closed); else a convex decomposition (parry's V-HACD, at most
8 hulls of at most 32 vertices). CoACD (Wei et al., SIGGRAPH 2022) is the quality upgrade if V-HACD
misfits; it has no maintained Rust crate today. Minimum extent 1 cm per axis.

`items.check` is a **settle test**, not a support probe (editor practice: UE "Simulate", the
Creation Kit's Havok Sim): each item settles **alone, lowest first**, over the statics within 1 m, a
terrain patch and the items already tested (`Fixed` at their placed pose), shrunk by
`overlap_allowance` (1 cm per side), for at most 0.5 s. A static whose simple collision **encloses**
the item's center is tested with its per-triangle `support.cook` mesh (UE "simple vs complex"), which
the runtime gives that placement too. `Hold::Unsupported` only when the center **and** the lowest
corner dropped more than 5 cm; rises and slides are overlaps. `body.cook` products are unit-density;
mods override with `bodies/<model>.sfbody`.

### `sf_ragdoll` (Bevy-free; sf_anim, sf_physics)

```rust
pub struct RagdollDef {
    pub skeleton, pub policy,       // Ragdoll | DeathClip | None
    pub bodies: Vec<RagBody>,       // joint, Capsule { radius, half } along its Y (sphere when half = 0), local, mass, region
    pub joints: Vec<RagJoint>,      // parent, child, frames, kind: Ball { swing, twist } | Hinge { axis, range } | Fixed
    pub regions: Vec<String>,       // "core", "head", "arm_l", ..., from the rig profile's limb names
    pub total_mass: f32, pub no_collide: Vec<(u16, u16)>, pub owner: Vec<u16>,  // owner: skeleton joint → body
}
pub enum Drive { Kinematic, Powered { strength: f32 }, Limp }        // per region
pub struct StrengthProfile { pub region: Vec<(u8, Drive)>, pub pelvis_world: f32 }
pub struct HitImpulse { pub body: u16, pub point: Vec3, pub impulse: Vec3, pub dip: f32, pub recover: f32 }
```

`ragdoll.cook` from the `RigProfile`: one capsule per girdle, at most 3 spine bodies to the neck base,
neck + head as one, one per limb joint (fingers and toes merged into hands and feet), tails and
`extra` chains up to `max_chain_bodies` (4). Radii are fractions of the segment (limbs 0.2, hands and
feet 0.3, wings and fins 0.08), girdle bodies as wide as the girdle; skin extents for creatures later.
Limits by limb kind: shoulders and wing roots ball 95°/±60° (arms rest in a T pose), hips 70°/±40°,
effectors 50°/±25°, spine 30°/±30°, neck 45°/±45°, tails 35°/±20°; knees, elbows and hocks hinge
toward the rest pose's bend. No body lighter than its parent ÷ `max_mass_ratio` (4), then scaled to the
adapter's total; bodies overlapping at rest don't collide. Adapter or mod override files patch any
value. Ragdolls use impulse joints with 8 extra solver iterations (Open question 1).

### Cloth lattice (`sf_character` cook, `sf_anim` op)

Opt-in only since 2026-10-19 (Interfaces "Cloth"). `sf_anim::cloth::ClothLattice`: rows × columns of
particles (row 0 pinned to the attachment joint, a ring for skirts), structural, shear and bend edges,
one tether per free particle to its column's pin (geodesic rest length), max distance to the animated
target, and leg/pelvis capsule colliders from `RagdollDef::cloth_colliders`. `sf_character::cook_cloth`
(adapter `ClothRules`) places particles on the garment's **outermost** surface, keeps the authored
skin under each as its animated target, and rebinds the vertices below the attachment line to the
lattice joints (outfit joints after the skeleton's), blending the band at the waist harmonically with
`smooth_body`'s inpainting. Capes (`ring: false`) are not supported yet. Sketch, cook rules, tests and
the robe census: [archive](archive/physics-history.md#cloth-lattice).

## Interfaces

### Stepping and the floating origin (`sf_physics_bevy`)

- `PhysicsPlugin` steps the **active worldspace's** world in `FixedUpdate` at **60 Hz** (`Time<Fixed>`),
  at most 3 steps per frame (beyond that, time dilates instead of spiralling). Rapier's substepped
  solver (4 iterations default; ragdolls and held items get extra iterations per island).
- **Anchor and rebase.** A world's anchor is a grid cell. When the gameplay anchor (the player) is more
  than `rebase_distance` (512 m) from it, the world rebases at the next step boundary by the whole-cell
  delta `d`: every body and collider translation minus `d · 256 m` (exact for positions whose
  distance to the anchor shrinks; the rest round once, by at most half an ulp at their new distance:
  ≤ 31 µm inside R0), velocities and sleep states untouched, interpolation history shifted alike, then one broad-phase refit. Rebases are
  independent of big_space's render origin (camera cell, 32 m hysteresis): rendering reads `WorldPos`,
  never the engine frame. Worst case inside R0: 768 m from the anchor, ulp 61 µm.
- **Write-back.** Each frame, every awake body's interpolated pose → `WorldPos` → its entity. Items
  spawn under an `ItemPivot` (placement translation and rotation relative to its stream cell; scale
  stays on the visuals) that follows the body, and stay children of their stream cell, so level
  unloads take them along (R0 is a few cells wide; f32 is exact there).
- **Other worldspaces** (door prefetch, M8 portal views) hold their statics and items but don't step.

### Streaming

- **Activation (R0 enter).** The host's activation hook (the one that fills `StaticWorld` today) inserts
  statics as `Fixed` colliders sharing one `Arc` shape per model, terrain as heightfield chunks
  (terrain.md: 64 m chunks, 129² at 0.5 m, from `TerrainHeights`, rebuilt on `TerrainChanged`), and
  `Dynamic` placements as sleeping bodies at their placement or overlay transform. Shapes build on the
  task pool; insertion is batched under a per-frame budget (0.5 ms).
- **Coverage.** At 4 Hz each awake body checks `Scheduler::covers(ws, pos, margin, Active)`. Leaving
  coverage freezes it: removed from the world, pose written to `PlacementOverlay::move_to`, entity kept
  visual-only (R1 semantics). Nothing simulates without ground around it.
- **Deactivation (R0 → R1).** Bodies that moved since activation write `move_to` + a `physics` state
  blob (`set_state(id, "physics", ..)`: v1 = asleep flag, linear and angular velocity); then removal.
  Saves serialize the overlay (`sf_save`, M10): persistence needs no new interface.
- **Fell out.** A body below the world's kill height returns to its last resting pose and sleeps. The
  kill height is `PhysicsParams::kill_height`, else `kill_margin` (10 m) below the lowest fixed body
  inserted so far (never rises), so every world gets recovery without the host knowing its depth.
- **Collision-ready gate** (characters, fallfix 2026-10-18; G1 2026-10-26): while a content cell the
  square around a mover touches is not activated yet (activation is time-sliced) or a queued static's
  bounds reach it (`PhysicsStreaming::collision_loading`, systems: `CollisionReady`), the host keeps it
  `Dormant`, as engines hold actors until streamed collision is in. Items wait the same way, each over
  its own bounds. Safety net: a `Ground` mover more than 2 m under
  resident terrain (else the kill height) returns to its last walkable ground (UE `KillZ`).
- **Hold until grabbed**: `Hold::Unsupported` placements go in `Held` and become dynamic only when
  grabbed or impulsed (a lantern on a hook never drops because an NPC brushed it). **Authored
  overlaps** wake gently: `max_corrective_velocity` 0.5 m/s.
- **Inserts** run while a decaying-maximum per-insert cost fits the 0.5 ms budget (at least one static
  per frame); a worldspace's items go in only once its statics are in. Deactivation writes moved items
  (> 1 mm or 1 mrad) as `move_to` + `physics` blob v1; all reinsert asleep for now.
- **Rolling resistance** (rapier has none): awake `DEBRIS`/`PROP` in contact lose 20 rad/s² of angular
  speed; items get `ITEM_DAMPING` (0.1 linear, 1.5 angular).

### Queries: `PhysicsProbe`

`sf_controller::GroundProbe` over the engine's query pipeline with a layer filter, batched like the M3
probes. It replaces `StaticProbe`; `TerrainProbe` stays (it reads the exact `TerrainHeights` the
renderer uses), composed through `Nearest` as today. Context probes, activation raycasts, the
crosshair and M9 hit tests all use it. `StaticWorld` stays as the physics-off path (`--off physics`)
and the equality oracle in tests.

**Terrain stays analytic for the movers**: a cast against a heightfield chunk costs ~13 µs, so
`impl GroundProbe for Physics` composes `TerrainProbe` with the bodies' `STATIC | PROP` (`GROUND_LAYERS`
adds `TERRAIN` through it); the chunks stay for bodies. Measurements in the archive (P6 session).

**Query refresh**: `refresh_queries` updates only the broad-phase BVH that queries read (body positions
to colliders, then `BroadPhaseBvh::set_aabb` for colliders inserted or moved since the last step).
Contacts, islands and the solver graph are left to the step. **Never call `detect_collisions` on a
stepped world**: it broke rapier's island invariant under streaming (physcrash 2026-10-19; regression
`sf_physics` `churn_tests`).

### Controller

`KinematicMover` stays (locomotion.md); `bevy_tnua` is not adopted. Reasons: the mover is already
batched (150 characters, 0.096 ms), drives root motion, ORCA and the animation feed in one stage, and
the industry's character controllers are kinematic (UE CMC, Unity CCT, Jolt `CharacterVirtual`).
A floating dynamic capsule (tnua) would put 150 bodies in the solver and fight root motion.
Changes in M4:
- **The mover steps on the fixed tick** (N0, 2026-10-19): `FixedUpdate` at 60 Hz, after the action
  layer's chooser and before the physics step, rendering interpolated; far tiers may step every second
  or fourth tick. The chooser's timers count ticks, action root motion comes from the same tick, and
  co-op prediction replays ticks (net.md rule 1), as in Overwatch and UE's Mover. Budget: within 10 % of
  today's per-frame cost. Built in action.md A2; until then capsule targets are extrapolated per step.
- Each character owns a **kinematic capsule** body (`CHARACTER`, 3 cm thinner than the mover's) whose
  target for each physics step is the mover's pose of the same tick, so falling and thrown bodies hit
  characters.
- **Co-op ownership** (net.md): the host steps every mover. A client steps only its own player's
  mover (predicted, replayed on correction) against its local static world, with props and other
  characters at their interpolated poses; it never pushes props (pushes and impacts are host-only,
  their results arrive as replicated body state).
- Sweeps include `STATIC | PROP` (and `TERRAIN` via `TerrainProbe`); `DEBRIS` never blocks.
- **Pushes**: for each `DEBRIS` or `PROP` body within `reach` (4 cm) of the mover's capsule, a
  horizontal impulse `min(m_body · Δv_n, push_force · dt)` along the capsule-to-center-of-mass normal
  (default push force 400 N per character; `dt` the tick; `Δv_n` from the **intended** velocity). The
  body's mass, not the character's: a cup takes the character's speed, a crate gets the capped force.
  Pushed props slow the mover by their resistance (UE "push force factor"). Held items stay held.
- **Impacts**: a `Hit` on a `CHARACTER` capsule of at least `impact_threshold` (1 N·s) becomes
  `ImpactOnCharacter { entity, impulse, point, direction }`; gameplay picks a hit reaction or a
  knockdown. The impulse is the incoming body's pre-solve momentum along the normal at first contact,
  `m · min(v_by·n, (v_by − v_capsule)·n)` (PhysX `eNOTIFY_TOUCH_FOUND` + `ePRE_SOLVER_VELOCITY`), never
  the solver's contact force (unbounded against a kinematic capsule), once per contact start.
- **Sliding** down unwalkable slopes (locomotion.md open question 3): the mover adds gravity along the
  slope when the ground is unwalkable; no physics body needed.
- Ragdolled characters carry `Ragdolled` (like `Dormant`: not stepped, capsule disabled); the root
  follows the pelvis projected to the ground.
- Host side: `sf_physics_bevy` feature `controller`; the host picks `SfControllerPlugin::<Physics>`
  (`--off physics` keeps `StaticWorld`).

### Carry and throw

- **Pick**: a ray from the view within `reach` (2.5 m) over `DEBRIS | PROP`, `RAGDOLL` bodies of
  **limp** ragdolls, and the statics and terrain that block it; a `Held` item is released by the grab.
- **Holder limits** from the adapter per holder, tied to the character's strength stat (owner,
  2026-10-12): `HolderLimits { carry_force, carry_mass, drag_mass, throw_impulse }`. The framework
  never names a stat.
- **Mode by mass** (simulated mass, or the whole ragdoll's for a limb): **carry** up to `carry_mass`;
  **drag** up to `drag_mass` (too heavy to lift: a ball joint at the grab point, so it rests on the
  floor and slides, tips or trails, as in Amnesia, Skyrim and Half-Life 2; the holder slows by
  `1 − mass / (2 · drag_mass)`; **dead bodies are dragged by the limb hit**, the ragdoll staying `Limp`
  and awake); above, `GrabResult::TooHeavy`.
- **Grab**: a kinematic **hand** body (no collision layer) at `view · hold_offset` (0.6 m + the item's
  bounding radius) and a generic joint with acceleration-based motors (ω = 5.8 / 0.15 s settle, ζ = 1):
  six axes for carry, capped by `carry_force` (angular cap × `torque_arm` 0.3 m); three linear axes for
  drag, capped by `max(carry_force, drag_friction · g · drag_mass)` (0.6, so a draggable crate moves),
  the hand keeping the grab point's distance and height below the eye, turning with the view's yaw.
  Over the cap the item lags; over `break_distance` (1 m) for 0.3 s it drops. The carried body is `HELD`
  (never ground for the mover), and only its holder's capsule is excluded, **per pair** (a contact-pair
  filter, like PhysX filter shaders and Jolt group filters).
- **Rotate** (modifier + look input) turns `hold_offset`; **drop** releases; **throw** (carry only)
  releases at `|hand velocity + view · throw_impulse / mass|`, capped at 15 m/s, CCD on.
- **No carry vibration** (Godotwind's bug, ARCHITECTURE §7.3): each step's hand target is the view **at
  the step's time** (at most 0.1 s extrapolated), and the carried item renders in holder space,
  `hand(view_now) · lerp(rel_prev, rel_curr, alpha)`; a dragged one at its own interpolated pose.
- Pick-into-inventory, ownership and crime are gameplay (M5/M10) on the same target query. Host side:
  `sf_physics_bevy::carry` (`Holder`, `CarryInput` / `CarryFeedback`, `HeldPoses`).

### Ragdolls and the animation blend

Per character with a ragdoll, `sf_ragdoll` owns a `RagdollInstance` (bodies, joints, current
`StrengthProfile`). Flow per frame:

1. The graph evaluates the animated model pose (animation.md).
2. `sf_ragdoll::powered` turns it into joint motor targets (each joint's relative rotation, as per-axis
   errors `2·asin(q.v_i)` with velocity feed-forward), kinematic targets and the pelvis world target.
   Motors are rapier's implicit acceleration-based ones: stable PD with **composite-inertia gains**
   (stiffness `strength · region gain · ω² · I_subtree / I_child`, ω 160 rad/s, damping 2·√stiffness;
   Zordan & Hodgins 2002), since a motor sees only its two bodies. A world-space pelvis drive (kinematic
   anchor, six-axis motors, ω 12, both gains scaled by the composite load, plus gravity compensation)
   holds powered characters up; it is off for `Limp`. Driven bodies sit on `Layer::POWERED`.
3. Physics steps (fixed). Bodies read back interpolated → joint local rotations → the graph's
   **physics stage** (`GraphInstance::set_physics`, per-joint weights from `def.owner`, after
   inertialization and before history and ops; 0 = animation, 1 = physics). Procedural ops run only
   where the physics weight is 0.

Behaviours (`sf_physics_bevy` feature `ragdoll`, `RagdollMode`; strengths in `sf_ragdoll::react`'s RON
`ReactionSet`, default `data/reactions.ron`, driven by a `ReactionController` per ragdoll):

- **Death** (`Death { crumple }`): full strength following the death clip, fading to `Limp` over 0.45 s
  (a crumple, not a drop); initial velocities from the last two animated poses; the entity frame follows
  the pelvis. Later: after 2 s asleep the bodies stay asleep, and deactivation stores the pose in a
  quantized `ragdoll` blob, restored frozen.
- **Knockdown** (`Knockdown { point, impulse, get_up, .. }`): the hit on the nearest body, brace 0.25
  fading over 0.8 s, get-up when settled.
- **Hit reaction** (A0 characters): `HitImpulse` on a body + a strength dip on its region recovering
  over `recover` s. Below the knockdown threshold, below A0 or past the powered budget, the kinematic
  `HitReact` op answers (animation.md "Secondary motion"; capsule impacts of 1-15 N·s, strength =
  impulse / 15); `SpringBones` swings tails and other chains.
- **Balance**: the center of mass against the support polygon of `support` limbs' contacts; outside by
  more than `stumble_margin` → a stumble step (a mover intent nudge + stumble clip); beyond recovery →
  knockdown (`Limp` for `down_time`).
- **Get-up**: when the pelvis is slower than 0.2 m/s for 0.5 s (any speed after `settle_timeout`, 6 s),
  face up or down picks the clip; the frame goes on the ground under the pelvis (WORLD ray) with the
  clip's first-frame root body (`clip_root_body`); the hand-back pose is pelvis-anchored and its 0.3 s
  inertialized blend (`GraphInstance::inertialize_from`) wins over the graph's state switch. The host
  uses an in-place get-up clip that ends over its origin, or applies its root motion, never both. Every
  request ends in `RagdollGotUp` or `RagdollFailed`.
- **Budget**: at most 8 powered ragdolls at once (others answer kinematically), limp ones unlimited
  but each asleep within seconds.

### Cloth

**Decision (2026-10-19, research `docs/research/cloth/long-robes.md`):** long garments that hide the legs (robes,
long skirts) stay **skinned as authored** by default: a content adapter's `ClothRules` answers `None` for them, so
nothing is cooked or rebound. The lattice below replaced the authored leg weights and could not keep the hidden legs
covered at a stride. The XPBD lattice stays for garments that hide nothing (capes, banners, tents) and as an
adapter opt-in for long garments.

The opt-in path: a `ClothLattice` op after foot placement at A0-A1 (A2 and below take the animated
rest offsets); XPBD in **character space** at fixed 1/240 s substeps (Small Steps), inertia entering as
**fictitious forces** scaled by `inertia_linear` / `inertia_angular` (UE Chaos, NvCloth), so a steady
run hangs like standing; capsule push-out last; teleports keep the shape and drop the velocity; wind
through `ClothInputs::acceleration`. Budget 0.03 ms per robe at A0 (measured 0.014 ms). A robe narrower
than the stride can't cover the legs without stretching, which is why long garments stay skinned.
Solver details and measurements: [archive](archive/physics-history.md#cloth).

**Not in M4 (later):** mesh cloth (a per-vertex proxy mesh) for capes and banners, against rapier's
soft bodies and a GPU XPBD path (WISHLIST §7); crates `bevy_silk`, `bevy_softbody`,
`rapier-cloth-core` checked 2026-10-12 and not taken.

### Adapter hooks (game-agnostic)

```rust
pub trait PhysicsContent: Send + Sync {
    /// Body for a `Dynamic` placement: shape product key, gameplay weight → simulated mass, layer.
    fn item_body(&self, placement: &Placement) -> Option<ItemBody>;
    fn holder_strength(&self, holder: Entity) -> HolderLimits;     // carry force, carry and drag mass, throw impulse
}
pub trait BodyRules: Send + Sync { /* authored collision for body.cook (point sets, one hull each), like CollisionRules */ }
pub trait ItemsRules: Send + Sync { /* items.check: is_item, always_hold, is_static, has_terrain */ }
pub trait RagdollRules: Send + Sync { /* per skeleton: policy (Ragdoll | DeathClip | None), total mass, overrides */ }
pub trait ClothRules: Send + Sync { /* which parts are cloth, attachment joint, lattice size, stiffness */ }
```

## Verification

Gates (numbers are the M4 budgets; the spike in task P1 confirms or revises them):

- **Rebase**: a 30-body stack and a powered ragdoll stepped with and without a mid-run rebase: the step across the
  rebase within 0.1 mm for every body, the stack within 0.1 mm at the end (P1: bit-identical; a ragdoll whose drives
  don't settle amplifies any perturbation, so its end state is compared once P8's drives settle); no body wakes from a rebase; rebase ≤ 2 ms with the content pack's densest town R0.
- **Idle cost**: R0 in a dense town with every item asleep: step ≤ 0.3 ms. **Active**: a 50-item pile
  collapsing plus 2 ragdolls: ≤ 2 ms. Activation insertion ≤ 0.5 ms per frame.
- **Probes**: `PhysicsProbe` hits equal `StaticProbe`'s on the walkers scenes; the 150-walker probe
  budget (0.2 ms) holds.
- **Items**: placed items don't move when the cell activates (0 wakes in 10 s with nobody near);
  knocked items rest within 3 s; leave and re-enter a cell: every moved item is where it rested.
- **Carry**: at 30, 60 and 144 fps, the held item's view-space position varies < 1 mm while walking
  and turning (the vibration test); throw speed within 1% of the configured cap; a stronger holder carries
  what a weaker one drags; a 75 kg ragdoll dragged by a wrist follows without joint stretch > 1 cm.
- **Ragdolls**: powered at full strength, standing and walking: tracking ≤ 5 cm RMS at effectors and
  ≤ 5° per joint (animation.md); get-up angular velocity continuous at shrinking dt; ≤ 0.05 ms per
  powered character; no stretch at joints > 1 cm in a fall down stairs.
- **Cloth**: walking, running, turning 180°: hem stretch ≤ 5%, no particle inside a leg capsule at
  run speed, ≤ 0.03 ms per robe; teleport: no explosion.
- **Visual** (owner): table knock-over, dead guard ragdoll on stairs, robes walking (scenes in the
  adapter spec and `docs/visual-checks.md`). Synthetic framework examples: `physics_table`,
  `ragdoll_stairs`, `cloth_robe`.

## Open questions

1. **Resolved (P7): impulse joints** with 8 extra iterations and a 4:1 mass-ratio cap, over
   reduced-coordinate multibodies (P1 spike: 0.44 ms vs 3.1-4.3 ms for 2 ragdolls + a 50-item pile).
   Spike numbers and history in the archive.
2. Rapier `parallel` (rayon) once a measured step exceeds budget, and whether to run the step as an
   async task overlapping render extraction (UE Chaos async physics).
3. Mesh cloth for capes and banners: rapier soft bodies vs our own XPBD on the GPU (after M4).
4. Character-vs-character physical contact (shoulder barging) beyond ORCA + separation: not planned.
