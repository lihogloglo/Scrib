# Physics: rigid bodies, items, carry/throw, ragdolls, cloth (M4)

Status: draft; decisions owner-confirmed 2026-10-12 (ARCHITECTURE §10). P1 (`sf_physics` core + spike) in progress.
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

**Bevy 0.20** has no physics engine, scene queries or interpolation. So it comes down to crates.

Checked 2026-10-12 (crates.io API, GitHub):

| Option | State | For | Against |
|---|---|---|---|
| **`rapier3d` 0.36.1 directly** (chosen) | 2026-10-10, Apache-2.0, no Bevy | On any Bevy version, now and later. **Same `parry3d` 0.31.1 as our probes**, so `collision.cook` shapes and `StaticShape` trimeshes go in unchanged and the tree keeps one parry. Joints: generic 6-DOF joints with **per-axis motors and limits** (spherical included), reduced-coordinate **multibodies** (Featherstone), implicit motors with an `AccelerationBased` model (mass-independent gains), recent joint-limit fixes (0.35). **Soft bodies** since 0.36 (particles, distance and bending edges, pinned particles with kinematic targets, coupled with rigid bodies). SIMD + optional rayon parallelism, CCD, contact-force events, `enhanced-determinism`, serde snapshots. A Bevy-free core matches §8b | We write the ECS sync, interpolation and debug draw (~1 k lines, thin). Pulls `nalgebra` + `simba` (compiled once, cached; isolated in `sf_physics`) |
| `bevy_rapier3d` 0.37.0 | 2026-10-10, on Bevy 0.20 | Ready today; same engine | Syncs bodies and colliders from Bevy transforms on `Changed<GlobalTransform>` (`bindings/bevy_rapier/src/plugin/systems/rigid_body.rs`, `collider.rs`). Under big_space `GlobalTransform` is origin-relative and every one is rewritten when the origin changes cell (world-coordinates.md "Implementation notes"), which this sync reads as a teleport of every body: wakes all sleepers and moves the solver frame each shift. Mirrors all of rapier's state into ECS. Couples the physics core to Bevy (tests need an `App`) |
| `avian3d` 0.7.0 / 0.8.0-dev | 0.7 on Bevy 0.19; main still on 0.19. A port exists: PR #1070 (andriyDev, open, "Bump Bevy to 0.20.0-rc.1, and Parry to 0.31"): about 100 changed lines in `src/` (mostly `On<Insert, B>` → `On<Insert<B>>`), plus git forks of `bevy_heavy`, `bevy_transform_interpolation` and `glam_matrix_extras` whose own 0.20 PRs are open | ECS-native, MIT/Apache, built-in interpolation, `move_and_slide` helpers, `enhanced-determinism`, f64 only for world positions (#1027, Jolt-style) | **No spherical or generic joint motor** on main (motors only on revolute and prismatic; `src/dynamics/joints/`), so powered-ragdoll shoulders and hips would need explicit PD torques, the unstable kind at high gains. No articulations, no cloth or soft bodies. Release on parry 0.27, so two parry versions until 0.8 ships. Components live in the Bevy world, so no Bevy-free core. Same `GlobalTransform`-based sync question under big_space (avian issue #439 on large worlds is still open; #1027 answers it with f64, which we don't need). Each Bevy release blocks on four repos |
| Wait for avian 0.8 | For 0.19 the main crates took about two weeks (ARCHITECTURE §4 lag tracker); 0.20 shipped 2026-10-08 | Zero porting | Doesn't change the joint-motor and Bevy-coupling points above; blocks M4 on someone else's schedule |
| Our `[patch]` fork of avian on 0.20 | PR #1070 + 3 dependency forks; rc.1 → 0.20.0 likely trivial; about a day | | Same technical points; a fork to maintain until 0.8 |
| Jolt (C++) through `joltc-sys`/`rolt` | 0.3.1, Jolt 5.0.0, last release 2024-05 | The industry reference (Decima: Horizon Forbidden West, Death Stranding 2): swing-twist motors, ragdoll pose drives, skinned soft bodies, double-precision big worlds | Bindings unmaintained for 2.5 years and behind Jolt; C++ toolchain at build; not worth owning bindings when Rapier covers M4 |
| `bevy_tnua` 0.32 (controller) | Bevy 0.19; `bevy-tnua-rapier3d` 0.18 is on `bevy_rapier3d` 0.36 / Bevy 0.19 | | See "Controller": we keep our kinematic mover |

**Why rapier directly, in one line each:** it is on our Bevy today and never blocks a Bevy upgrade again;
it shares our parry, so the M3 static world moves in unchanged; it has the per-axis motors and
articulations powered ragdolls need; the floating-origin contract is ours to implement exactly
instead of fighting a transform sync; and it keeps physics logic Bevy-free (§8b, rule 7).
**Performance** is not settled by reading: task P1's spike measures it against the gates below, and
avian 0.8 stays the fallback if Rapier misses them (the `sf_physics` API hides the engine, so a swap
touches one crate).

**Determinism.** Same binary, same inputs, same insertion order → same result with the serial solver
(insertion follows index order, which is source-order independent: streaming.md). That is enough for
replay tests and the `items.check` bake. Cross-platform `enhanced-determinism` (no SIMD) stays off.
Rayon `parallel` stays off until a measured step exceeds its budget (Bevy's task pools already use the
cores; oversubscription is a known cost).

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
pub enum BodyKind { Fixed, Dynamic { asleep: bool }, Kinematic }
pub struct BodyDesc {
    pub kind: BodyKind, pub pose: WorldTransform, pub shape: ShapeRef, pub layer: Layer,
    pub mass: Option<f32>,          // kg (simulated mass, clamped); None = shape volume × density
    pub ccd: bool, pub damping: (f32, f32), pub user: u64,   // user: PlacementId low bits / entity
}
pub enum ShapeRef { Static(Arc<StaticShape>), Body(Arc<BodyShape>), Capsule { r: f32, half: f32 },
                    Box(Vec3), Sphere(f32), Heightfield(Arc<HeightChunk>) }
bitflags! pub struct Layer: u32 { STATIC, TERRAIN, DEBRIS, PROP, CHARACTER, RAGDOLL, HELD, SENSOR, PROJECTILE }
```

- **Frame.** All engine positions are `anchor.delta_f32(pos)`; every read returns `WorldPos`.
- **As built (P1, 2026-10-12).** `ShapeRef::Mesh(Arc<MeshShape>)` is built from the `collision.cook` `CollisionMesh`
  (one parry `TriMesh` per model, cached scaled copies per distinct scale), not from `sf_controller::StaticShape`
  (whose trimesh is private; sharing one copy between `StaticWorld` and physics needs an accessor there, P2).
  `ShapeRef::Body` lands with `body.cook` (P3). `BodyDesc` gains `extra_iterations` (ragdolls, held items).
  **Settle on insert:** rapier wakes both sides of every new contact and the partners of every moved collider, so
  a body inserted asleep is locked on every axis for one step with a zero sleep delay; the engine wakes it, it can't
  move, and the engine's own sleep pass re-sleeps it in that step; then it is unlocked. Rebases re-settle every
  sleeper the same way. Result: 0 wakes and bit-exact poses. Queries see inserts after a step or
  `PhysicsWorld::refresh_queries`.
- **Layers.** `DEBRIS` = light, small dynamic items (below `debris_mass` 15 kg and `debris_extent`
  0.6 m by default): characters push them and never stand on or are blocked by them. `PROP` = everything
  else dynamic: blocks movers, is pushed with a capped force. `HELD` = the carried item (no contact with
  its holder's capsule). `RAGDOLL` bodies skip contacts with jointed neighbors.
- **Simulated mass** is clamped to `[0.2, 60]` kg by default (a cup under a cuirass is 300:1 raw); the
  gameplay weight stays in records. Thin shapes get a 4 mm contact skin.
- **Events** (drained per step): `Woke`, `Slept`, `ContactForce { a, b, impulse, point }` above a
  per-layer threshold (impact sounds, damage later, knockback), `LeftCoverage(BodyId)`, `FellOut(BodyId)`.
- **Interpolation state**: previous and current pose per awake body; `pose_at(alpha)` lerp + slerp.

**Bake products** (feature `bake`, sf_bake steps):

| Step | Subject | Inputs | Output |
|---|---|---|---|
| `body.cook` | model | `mesh.cook` LOD0/LOD1, adapter `BodyRules::authored` (authored collision), settings | `BodyShape { parts: Vec<ShapePart>, volume, center_of_mass, unit_inertia, aabb, source }`; `ShapePart = Hull(points) \| Box \| Capsule \| Cylinder \| Sphere`, each with a local isometry |
| `items.check` | index page | page placements of class `Dynamic`, their `body.cook` products, statics' `collision.cook` | per placement: `support` (gap to the surface below), `penetration`, flag `HOLD_UNTIL_GRABBED` when nothing supports it within 5 cm (hanging lanterns, floating props); a report of overlaps over 1 cm |
| `ragdoll.cook` | skeleton | skeleton, `RigProfile`, skin extents, adapter overrides | `RagdollDef` (below) |
| `cloth.cook` | (part model, skeleton) | cooked skinned part, `RigProfile`, adapter `ClothRules` | `ClothLattice` (below) |

`body.cook` shape choice, first match: authored collision (its convex parts; trimeshes are hulled);
a primitive (box, capsule, cylinder, sphere) when its volume is within 15% of the mesh hull's and it
covers the hull (bottles roll, swords lie flat); one convex hull when the hull's volume is within 25%
of the closed mesh's (or the mesh isn't closed); else a convex decomposition (parry's V-HACD, at most
8 hulls of at most 32 vertices). CoACD (Wei et al., SIGGRAPH 2022) is the quality upgrade if V-HACD
misfits; it has no maintained Rust crate today. Minimum extent 1 cm per axis.

**As built (P3, 2026-10-13).** `sf_physics::cook` (`BodyShape`, `ShapePart`, `fit_render`, `fit_authored`; `SFBD`
products) and `sf_physics::steps` (feature `bake`: `BodyCookStep` + `BodyRules`, `ItemsCheckStep` + `ItemsRules`).
Shape choice as above, with these details: the primitive fit runs in the hull's covariance frame (parry `utils::obb`)
or the model axes, whichever box is smaller, and tries box, sphere, three cylinders and three capsules, keeping the
smallest that covers every hull vertex; "closed" means watertight after welding seams on a 0.1 mm grid; hulls keep at
most 32 vertices (farthest-point subset of the hull's vertices, re-hulled; at most a few mm inside the true hull), the
cap applies to single hulls too; a shape thinner than 1 cm becomes a box padded to 1 cm (`BodySource::Thin`); models
`mesh.cook` skips use their `mesh.rest` cook; mods override with `bodies/<model>.sfbody`. Mass properties are
unit-density (volume, center of mass, principal inertia per kg and its frame); the runtime scales by the adapter's
mass. `ShapeRef::Body(Arc<BodyShape>)` builds the engine shape (one part, or a compound).
**`items.check` is a settle test, not a ray/shape-cast support probe**: per page, the items go into a `PhysicsWorld`
as awake `PROP` bodies over the static collision (`collision.cook`) whose bounds come within 1 m of an item, and,
where the adapter says the worldspace has terrain, a 4 m heightfield patch (0.5 m, from the level-0 `terrain.tile`
products) under each item; the world runs 0.5 s; an item that dropped more than 5 cm is `Hold::Unsupported`
(inserted `Fixed` until grabbed); rises and slides are reported as overlaps (it started inside something). Why: a
probe answers "is there a surface below" but not "does it stay": items leaning on a wall, half off a shelf edge, on
a steep slope or stacked on other items all need the contact solve, and the settle test is the editor practice
for clutter (UE "Simulate" + "Keep Simulation Changes", the Creation Kit's Havok Sim), here run as a check. Product
`ItemChecks` (`SFIC`): per item `id`, `hold` (`No`, `Unsupported`, `Rules` = the adapter's `always_hold`,
`NoShape`), `drop`, `rise`, `shift`. Limitation: a page sees only its own statics (exterior pages are large; an
item within 1 m of a page edge may miss a neighbor's static and be held: the safe side).
**Revised in P4 (2026-10-13): sequential settle.** Settling a page's items all at once flagged 38 % of a vanilla
install unsupported and 51 % overlapping: authored clutter interpenetrates its neighbors by a centimeter or more, and
the solver shoved bottles off shelves and plates off tables. Now each item settles **alone, lowest first** (by the
bottom of its placed bounds), over the statics, the terrain and the items already tested, which stay in the world
`Fixed` at their placed pose (as at runtime, where every item starts there, asleep or held); its test shape is shrunk
uniformly about its bounds' center so its largest extent loses `overlap_allowance` (1 cm) per side (at most 20 %; a
negative contact offset, like PhysX's rest offset); it runs at most `settle_time`, stopping early once it has dropped
past the tolerance or has been still for 5 steps. Displacements are measured from the shrunk start. Census: ROADMAP
M4 P4.

**Revised in P4 (physics-p4b): support meshes and "fell".** A legacy collision proxy is often one box around a whole
bookshelf or cabinet, so the clutter on its shelves starts *inside* static collision (one-sided meshes let it fall
through, two-sided ones shove it out): a third of a vanilla install's items. Engines keep two representations, simple
collision for characters and complex (per-triangle) collision for physics props and traces (UE "simple vs complex",
Bethesda's per-shelf Havok shapes). So: a static whose collision **encloses** an item center (`MeshShape::encloses`:
the nearest triangle straight below the point is a back face) is tested with its `support.cook` mesh (the `mesh.cook`
LOD0 as a `CollisionMesh`, requested by `items.check` on demand) and listed in `ItemChecks::supports`; the runtime
gives those placements the same mesh (`ItemContent::is_support`, `PhysicsBodySource::supports`); characters keep the
proxy. An item is unsupported only when its center **and** the lowest corner of its bounds both dropped past the
tolerance: a book tipping over in its row, or a bowl rocking in place, did not fall. Runtime: items of a worldspace
go in only once it has no static left to insert (a woken item above a not-yet-inserted floor falls out of the world),
and every frame inserts at least one static whatever the refresh estimate says (one slow refresh must not stall
streaming).

### `sf_ragdoll` (Bevy-free; sf_anim, sf_physics)

```rust
pub struct RagdollDef {
    pub bodies: Vec<RagBody>,      // joint: u16, shape: Capsule|Box|Sphere, local: Isometry, mass, region: u8
    pub joints: Vec<RagJoint>,     // parent, child, frames, kind: Ball { swing: [f32; 2], twist: [f32; 2] } | Hinge { axis, range } | Fixed
    pub regions: Vec<String>,      // "core", "head", "arm_l", ..., from the rig profile's limb names
    pub total_mass: f32,
}
pub enum Drive { Kinematic, Powered { strength: f32 }, Limp }        // per region
pub struct StrengthProfile { pub region: Vec<(u8, Drive)>, pub pelvis_world: f32 }
pub struct HitImpulse { pub body: u16, pub point: Vec3, pub impulse: Vec3, pub dip: f32, pub recover: f32 }
```

`ragdoll.cook`: one body per girdle, spine segment (merged to at most 3), neck+head, and limb joint
(hands and feet kept, fingers and toes merged into them); tails and `extra` chains get bodies up to
`max_chain_bodies` (4). Capsules run along the bone to its child, radius = the 80th percentile of
vertex distances to the bone axis among vertices with weight ≥ 0.5 on that joint (rest pose), else
`length / 4`. Mass = volume × 1000 kg/m³, then scaled to the adapter's total mass. Limits by limb kind:
knees and elbows hinge (0..150°), shoulders and hips ball (swing 80/60°, twist ±40°), spine ±30°,
neck ±45°, digitigrade and unguligrade legs get the extra hinge; adapter or mod override files patch
any value. Adjacent bodies never collide.

### Cloth lattice (`sf_character` cook, `sf_anim` op)

```rust
pub struct ClothLattice {
    pub rows: u8, pub cols: u8, pub ring: bool,            // a skirt is a ring; a cape is open
    pub particles: Vec<ClothParticle>,  // rest pos (character space), parent joint, max_distance, inv_mass
    pub edges: Vec<(u16, u16, Kind)>,   // Structural | Shear | Bend | Tether (one-sided, to the pinned row)
    pub colliders: Vec<(u16, f32, f32)>, // body joint, radius, half length: from RagdollDef capsules (legs, pelvis, spine)
    pub weights: SkinRebind,             // part vertices → lattice joints (+ residual original weights)
}
```

`cloth.cook`: for parts the adapter marks as cloth (skirt and robe hems, capes), place `cols` columns
around the body's vertical axis (or across, for a cape) and `rows` rows from the attachment line to the
hem on the rest-pose surface (default 10 × 5 for a full robe); pin the top row to its girdle joint;
re-bind the part's vertices below the attachment line to the lattice joints with the harmonic
inpainting `sf_character::smooth_body` already has (smooth-bodies.md), so the hem moves and the waist
stays. Cloth joints are outfit joints appended after the skeleton's joints in the merged palette.

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
- **Write-back.** Each frame, every awake body's interpolated pose → `WorldPos` → `CellCoord + Transform`
  (`sf_world_bevy::to_grid`). Items begin as low-precision children of their stream cell, like statics;
  on first wake they become their own grid entities (streaming.md: "Dynamic objects get their own
  `CellCoord` when activated") and are `Stationary` while asleep.
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
- **Fell out.** A body below `TerrainHeights` − 2 m (or the worldspace's kill height) returns to its
  last resting pose (the pose at its last sleep) and sleeps.
- **Hold until grabbed.** `items.check`'s `HOLD_UNTIL_GRABBED` placements are inserted `Fixed` and become
  dynamic only when grabbed or explicitly impulsed (script, spell): a lantern on a hook never drops
  because an NPC brushed it.
- **Authored overlaps** wake gently: depenetration speed capped (`max_corrective_velocity` 0.5 m/s).
- **As built (P2, 2026-10-13).** `sf_physics_bevy`: `PhysicsHook` wraps the placement spawners the host picks
  (activation queues, deactivation removes at once); each model's `MeshShape` is built on the task pool from the
  host's `PhysicsShapeSource` and shares the controller `StaticShape`'s `Arc<TriMesh>` (`StaticShape::trimesh()`);
  scaled copies are built on the task pool too (a scaled trimesh rebuilds its BVH). Inserts run while the frame's
  0.5 ms budget lasts, with `refresh_queries` every 32 bodies so broad-phase work lands in the inserting frame, not
  in the next step. Terrain chunks cover the active worldspace's R0 cells, sampled from `TerrainHeightsMap` on the
  task pool, nearest first, 16 per task; chunks with non-resident heights retry when the heights change (edit
  rebuilds are still open). Debug draw is our own per-triangle cull within 40 m (`PhysicsWorld::debug_lines`):
  rapier's `DebugRenderPipeline` draws whole colliders, too many lines for an R0. `PhysicsProbe` for the walkers
  waits for walkers.rs to move over (it keeps its `StaticWorld` meanwhile).

- **As built (P4, 2026-10-13).** Items: the adapter's `ItemContent` (`is_item`, `item_body` → mass and hold; the
  spec's `PhysicsContent::item_body`) on `PhysicsBodySource`; `hook_physics` (statics, or items for items) and
  `hook_items` (items only: carryable lights among the lights). Item visuals spawn under an `ItemPivot` (translation
  and rotation of the placement, relative to the stream cell's origin; scale stays on the visuals for their LOD
  ranges), which follows the body's interpolated pose; a transform is written only when it changed. This replaces
  "own grid entities on first wake" for now: items stay children of their stream cell (R0 is at most a few cells
  wide, f32 is exact enough there), so level unloads take them along. Items go in as `BodyDesc::item` (asleep, CCD on)
  or `Fixed` when held, `DEBRIS` under `debris_mass` 15 kg and `debris_extent` 0.6 m, else `PROP`. Inserts run in two
  phases per frame: statics (refresh every `insert_batch` = 16 bodies, stopping while the per-body refresh estimate
  still fits the budget), one refresh, then up to `item_batch` = 32 items with no refresh (rapier puts new dynamic
  bodies in its islands only in a step; re-queuing them as modified after a collision-only pass leaves their contacts
  unlinked from the persistent islands, which rapier's debug validation catches), so their broad-phase work lands in
  the next step. Deactivation writes items that moved (> 1 mm or 1 mrad) to the overlay (`move_to` + `physics` blob
  v1: asleep flag, linear and angular velocity); activation reads the overlay for cells whose visuals stayed loaded.
  Not yet: reinserting awake bodies with their saved velocity (all reinsert asleep), grab releasing held items (P6).
  **P4 leftovers (P5 session, 2026-10-13):** *coverage freeze* (`stream::coverage`, 4 Hz: an awake item whose
  surrounding 1 m isn't `Active` per `Scheduler::covers` writes its pose to the overlay, leaves the world, and its
  visual stays put; worldspaces the scheduler doesn't know are always covered); *kill height*: with
  `PhysicsParams::kill_height` unset, `PhysicsWorld::kill_height` is `kill_margin` (10 m) below the lowest fixed
  body inserted so far (statics and terrain chunks; it never rises), so every world gets fell-out recovery without
  the host knowing its depth (`TerrainHeights − 2 m` per body would need a height lookup per awake body: not worth
  it while the floor rule works); *held items* are `BodyKind::Held` (`BodyDesc::held`: fixed, but with mass,
  damping and CCD ready), and `PhysicsWorld::release` or any `apply_impulse` turns one into an awake dynamic body;
  pushes and contacts never do; *insert time-slicing*: statics and items each keep a decaying-maximum per-insert
  cost, and the next insert goes in only while it fits the 0.5 ms budget (one per frame always, when nothing else
  went in).
  **Rolling resistance**: rapier has none, so a knocked bottle rolled for many seconds; awake `DEBRIS`/`PROP` bodies
  in contact lose `rolling_resistance` (20 rad/s², about μr 0.05 on a 6 cm radius) of angular speed per second, and
  items get `ITEM_DAMPING` (0.1 linear, 1.5 angular). Havok exposes rolling friction per material; this is the
  stand-in.

### Queries: `PhysicsProbe`

`sf_controller::GroundProbe` over the engine's query pipeline with a layer filter, batched like the M3
probes. It replaces `StaticProbe`; `TerrainProbe` stays (it reads the exact `TerrainHeights` the
renderer uses), composed through `Nearest` as today. Context probes, activation raycasts, the
crosshair and M9 hit tests all use it. `StaticWorld` stays as the physics-off path (`--off physics`)
and the equality oracle in tests.

**Revised (P6 session, 2026-10-13): terrain stays analytic for the movers.** A sphere or capsule cast against a
heightfield chunk costs ~13 µs (`sf_physics` `probe_bench`: 150 movers × 4 casts, `StaticWorld` + `TerrainProbe` 2.14
ms, physics over `STATIC | TERRAIN | PROP` 6.36 ms, `Nearest(TerrainProbe, physics STATIC | PROP)` 2.58 ms). So
`impl GroundProbe for Physics` composes `TerrainProbe` over the active worldspace's `TerrainHeights` (`Physics::terrain`)
with the bodies' `STATIC | PROP`; the chunks stay for bodies. wk-balmora mover stage median, physics vs `--off
physics` (shared machine): 424 vs 272 µs (was 584 vs 319). The rest of the gap is rapier's query BVH (every body,
filtered per leaf) and the one-sided internal-edge static meshes: a statics-only BVH view is the next step if needed.

### Controller

`KinematicMover` stays (locomotion.md); `bevy_tnua` is not adopted. Reasons: the mover is already
batched (150 characters, 0.096 ms), drives root motion, ORCA and the animation feed at frame rate, and
the industry's character controllers are kinematic (UE CMC, Unity CCT, Jolt `CharacterVirtual`).
A floating dynamic capsule (tnua) would put 150 bodies in the solver and fight root motion.
Changes in M4:

- The mover keeps running in `Update` at frame rate (locomotion.md had planned a move to the fixed
  step; UE's CMC also ticks per frame). Each character also owns a **kinematic capsule** body
  (`CHARACTER`) whose target before each fixed step is the mover's latest position, so falling and
  thrown bodies hit characters.
- Sweeps include `STATIC | PROP` (and `TERRAIN` via `TerrainProbe`); `DEBRIS` never blocks.
- **Pushes**: for each `PROP` hit in a sweep, an impulse `min(m_char · Δv_n, push_force · dt)` on the
  body (default push force 400 N, per character); `DEBRIS` in the capsule's path get the same capped
  impulse. Pushed props slow the mover by their resistance (UE "push force factor").
- **Impacts**: `ContactForce` on a `CHARACTER` capsule above `impact_threshold` becomes an
  `ImpactOnCharacter { entity, impulse, point }` event; gameplay picks a hit reaction or a knockdown.
- **Sliding** down unwalkable slopes (locomotion.md open question 3): the mover adds gravity along the
  slope when the ground is unwalkable; no physics body needed.
- Ragdolled characters carry `Ragdolled` (like `Dormant`: not stepped, capsule disabled); the root
  follows the pelvis projected to the ground.
- **As built (P5, 2026-10-13).** `sf_physics::character`: `BodyDesc::character` (kinematic `CHARACTER` capsule,
  3 cm thinner than the mover's so a prop the mover rests against isn't shoved by an infinitely heavy body),
  `PhysicsWorld::move_character`, `PhysicsWorld::push`. **Correction to the push formula above:** the impulse is
  `min(m_body · Δv_n, push_force · dt)`, the body's mass, not the character's (`m_char · Δv` would launch a 0.3 kg
  cup at the cap every frame; with the body's mass a cup takes the character's speed and a crate gets the capped
  force). `Δv_n` uses the **intended** velocity (the mover removes the part into whatever blocked it, which would
  leave nothing to push with); the normal is horizontal, from the capsule axis to the body's center of mass, and
  bodies within `reach` 4 cm of the mover's capsule count (more than its 1 cm wall skin). One push pass per mover
  step (frame rate), DEBRIS and PROP alike, held items stay held. `sf_physics_bevy` feature `controller`
  (`character.rs`): capsules for every non-`Dormant` `Mover` in the active world, targeted after
  `ControllerSystems::Step`, pushes from `Mover::applied`, `ImpactOnCharacter` messages from `ContactForce` events
  on capsules over 5 N·s (`PhysicsEvents` keeps a frame's events), `CharacterStats`. Ground: `impl GroundProbe for
  Physics` over `GROUND_LAYERS` = `STATIC | TERRAIN | PROP` of the active world; the host picks
  `SfControllerPlugin::<Physics>` (the Bevyrrowind walkers do unless `--off physics`). Slope sliding is in the mover
  (`Locomotion::slide`, gravity's horizontal part along the unwalkable ground normal, decays with a 0.1 s time
  constant on walkable ground; unwalkable ground is probed every step even standing still). Tests:
  `sf_physics` `character_tests` (a 60 kg crate blocks then yields with every push ≤ the cap; a cup never blocks;
  a 25 m/s ball hits a standing capsule and bounces; no tunnelling through a 4 cm plank at 7 m/s),
  `sf_controller` `unwalkable_slopes_slide`.

### Carry and throw

- **Pick target**: `PhysicsProbe` ray from the view within `reach` (2.5 m), layers `DEBRIS | PROP`, plus
  `RAGDOLL` bodies of **limp** ragdolls (corpses, knocked-out characters).
- **Holder limits** come from the adapter per holder (`PhysicsContent::holder_strength`), tied to the
  character's strength stat (owner request, 2026-10-12): `HolderLimits { carry_force, carry_mass,
  drag_mass, throw_impulse }`. The framework never names a stat; the adapter maps its own.
- **Mode by mass** (simulated mass of the item, or the whole ragdoll's mass for a limb):
  - **Carry** (`mass ≤ carry_mass`): the item lifts to the hold point, as below.
  - **Drag** (`carry_mass < mass ≤ drag_mass`): too heavy to lift, light enough to move. The joint is a
    **ball joint at the grab point** (position motors only, rotation free), so the object hangs from
    the hand, rests its weight on the floor and slides, tips or trails behind (Amnesia, Skyrim's grab on
    heavy objects, Half-Life 2 crates). The hand target sits at waist height ahead of the holder, the
    force cap still applies, and the holder's mover speed is scaled by `1 − mass / (2 · drag_mass)`
    (gameplay may override). **Dead bodies are dragged by a limb**: the pick ray hits one ragdoll body
    (a wrist, an ankle, the head) and the ball joint attaches to that body at the hit point; the rest
    of the ragdoll follows through its joints, so a corpse is pulled by the arm or leg that was grabbed.
    The ragdoll stays `Limp` and awake while dragged; its pelvis-follow root (Controller, last bullet)
    keeps the character's entity with the body.
  - **Too heavy** (`mass > drag_mass`): no grab; the attempt returns `GrabResult::TooHeavy` so the game
    can show feedback.
- **Grab (carry)**: a kinematic **hand** body at `view · hold_offset`, a 6-DOF joint from the hand to the item's
  grab point with motors on all axes (stiffness tuned for a 0.15 s settle, critically damped) and a
  **force cap** from the holder's strength (adapter). Over the cap the item lags and drags on the
  floor (Amnesia), and over `break_distance` (1 m) for 0.3 s it drops. Held items are `HELD` (no contact
  with the holder), never ground for the mover, and wake whatever they touch.
- **Rotate** (modifier + look input) turns `hold_offset`; **drop** releases; **throw** releases with the
  hand's velocity plus an impulse along the view (`throw_impulse` from the adapter, speed capped at
  15 m/s; carry mode only, a dragged object is released, not thrown), CCD on.
- **No carry vibration** (Godotwind's bug, ARCHITECTURE §7.3). The view moves at frame rate and the item
  at 60 Hz, so the item is rendered **in holder space**: `render = view_now · (view_at_step⁻¹ · item_at_step)`
  interpolated, which removes the step-rate beat between the two clocks. Test below.
- Pick-into-inventory, ownership and crime are gameplay (M5/M10) on top of the same target query.
- **As built (P6, 2026-10-13).** `sf_physics::carry`: `Carrier` (one holder's grab state), `HolderLimits`
  (`mode_for(mass)`), `CarryParams` (reach 2.5 m, hold distance 0.6 m + the item's bounding radius, settle 0.15 s,
  break 1 m / 0.3 s, throw cap 15 m/s, torque arm 0.3 m, 4 extra iterations), `GrabResult`, `CarryEnd`. The hand is a
  kinematic body with no collision layer; the joint is a rapier generic joint with every axis free and
  acceleration-based motors (ω = 5.8 / settle time, ζ = 1): carry on all six axes (linear cap `carry_force` per axis,
  angular `carry_force · torque_arm`), drag on the three linear axes at the grab point. The pick ray sees
  `pick_layers | STATIC | TERRAIN` and accepts the first hit only if it is a grabbable dynamic or `Held` body (a
  `Held` item is released by the grab). The carried body moves to `HELD` (restored on release), and its holder's
  capsule is excluded **per pair** (`PhysicsWorld::exclude_pair`, a rapier contact-pair filter hook: PhysX's filter
  shader, Jolt's group filter), not by layer, so it still hits other characters. Deviations from the bullets above:
  - **Drag force cap**: `max(carry_force, drag_friction · g · drag_mass)` with `drag_friction` 0.6, not `carry_force`:
    with the adapter's numbers (S 50: 240 N, drag mass 120 kg) the grip cannot even slide a 40 kg crate pulled at its
    top (measured: it doesn't move), so "light enough to move" would not move. The drag hand keeps the grab point's
    horizontal distance and its height below the eye from the moment of the grab, turning with the view's yaw (not a
    fixed waist height: a crate grabbed by its top edge is not lifted by it).
  - **No carry vibration** takes two parts: each step's hand target is the view **at the step's time** (interpolated
    or extrapolated from the last two recorded views, at most 0.1 s ahead), so two steps in one 30 fps frame don't move
    the hand in one jump; and the carried item renders at `hand(view_now) · lerp(rel_prev, rel_curr, alpha)` with
    `rel` the item relative to the hand target after each step. A dragged object renders at its own interpolated
    pose (its weight is on the floor). Test: view-space spread 0.008 / 0.010 / 0.044 mm at 30 / 60 / 144 fps.
  - Throw speed is `|hand velocity + view · throw_impulse / mass|` capped at 15 m/s.
  `sf_physics_bevy::carry`: `Holder` component (its entity's `CellCoord + Transform` is the eye, −Z forward),
  `CarryInput` / `CarryFeedback` messages, `HeldPoses` (read by the write-back and item pivots), views recorded in
  `RunFixedMainLoop` (`PhysicsSystems::Views`, after camera controllers that run there) and hands driven around the
  step in `FixedUpdate`; changing worldspace drops. The adapter maps Strength (`mw_physics::holder_limits`); the host
  fills `Holder::carrier.limits` (the `PhysicsContent::holder_strength` hook becomes this host call until characters
  carry). Not yet: limp-ragdoll limbs (P7), the holder's mover slowdown (the host reads `Holder::speed_scale`).

### Ragdolls and the animation blend

Per character with a ragdoll, `sf_ragdoll` owns a `RagdollInstance` (bodies, joints, current
`StrengthProfile`). Flow per frame:

1. The graph evaluates the animated model pose (animation.md).
2. `sf_ragdoll::drive_targets` turns it into joint motor targets (local rotations), kinematic targets
   for `Kinematic` regions, and the pelvis world target. Motors are rapier's implicit, acceleration-
   based joint motors (stiffness = strength × the region's gain, damping for ζ = 1), i.e. stable PD with mass-independent
   gains; avian could only add explicit torques here. A world-space pelvis drive (UE's world-space
   physical animation) holds powered characters up; it is off for `Limp`.
3. Physics steps (fixed). Bodies read back interpolated → joint local rotations → the graph's
   `PoseInput` node, blended per joint by the region's physics weight (0 = animation, 1 = physics).
   Procedural ops after it (foot placement, look-at) run only where the physics weight is 0.

Behaviours:

- **Death**: regions switch to `Powered` following the death clip, then strength fades to `Limp` over
  0.3–0.6 s (a crumple, not a drop); initial body velocities come from the last two animated poses.
  After the ragdoll sleeps for 2 s its bodies stay asleep; on deactivation the pose is quantized into a
  `ragdoll` state blob and restored frozen.
- **Hit reaction** (A0 characters): `HitImpulse` on a body + a strength dip on its region recovering
  over `recover` s. Below A0, or past the powered budget, the kinematic `HitReact` op (animation.md)
  answers instead.
- **Balance**: the center of mass against the support polygon of `support` limbs' contacts; outside by
  more than `stumble_margin` → a stumble step (a mover intent nudge + stumble clip); beyond recovery →
  knockdown (`Limp` for `down_time`).
- **Get-up**: when the pelvis is slower than 0.2 m/s for 0.5 s, face up or down from the pelvis axis
  picks the get-up clip; the character root is placed under the pelvis with the clip's first-frame yaw;
  `PoseInput` inertializes from the ragdoll pose into the clip (the transition test in animation.md).
- **Budget**: at most 8 powered ragdolls at once (others answer kinematically), limp ones unlimited
  but each asleep within seconds.

### Cloth

- `ClothLattice` op in the procedural op stack, after foot placement (it needs the final legs), at
  A0–A1; at A2 and below the lattice joints take their animated (skinned) rest offsets, which is how the
  robe looked before M4.
- Solver: XPBD, fixed internal substep 1/240 s accumulated over the frame (Small Steps), in **character
  space**: the character's world acceleration enters scaled by `inertia_scale` (0.5 linear, 0.3
  angular by default), so teleports, root snaps and fast turns don't whip the hem. Constraints:
  structural, shear and bend edges with compliances; tethers (one-sided, to the pinned row: no
  stretch, the long-range attachment); max distance to the animated position (0 at the pins, growing
  with row); capsule collisions with the rig's leg and pelvis capsules (from `RagdollDef`) with a
  thickness. Wind (M6 `sf_wind`) and drag come in through one external acceleration.
- Budget: 0.03 ms per robe at A0 single-threaded; cloths run in the animation tasks, so they spread
  across cores with the characters.

**Not in M4 (later, recorded):** mesh cloth (a per-vertex proxy mesh driving the render mesh, for
capes and banners), evaluated then against rapier's soft bodies (0.36, CPU, coupled with rigid
bodies) and a GPU XPBD path if counts demand it (WISHLIST §7). Crates checked 2026-10-12 and not
taken: `bevy_silk` 0.10 (Bevy 0.17), `bevy_softbody` 0.1 (GPU XPBD, Bevy 0.19, one release),
`rapier-cloth-core` 0.3 (MIT, XPBD mesh cloth, four weeks old, no character constraints): references
for the mesh-cloth step.

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

1. Impulse joints or a reduced-coordinate multibody for ragdolls: the P7 spike tries both (multibodies
   don't stretch and take high gains, impulse joints sleep and island more simply). P1 spike data (2026-10-12): 2 ragdolls
   + a 50-item pile step in 0.44 ms with impulse joints vs 3.1–4.3 ms with multibodies (plus a 450 ms hitch); a
   0.5 m drop stretches impulse joints 2–2.6 cm (1.1 cm at 12 extra iterations) vs 0.6 cm for multibodies. Leaning
   impulse joints with more iterations and bounded mass ratios; P7 decides.
2. Rapier `parallel` (rayon) once a measured step exceeds budget, and whether to run the step as an
   async task overlapping render extraction (UE Chaos async physics).
3. Mesh cloth for capes and banners: rapier soft bodies vs our own XPBD on the GPU (after M4).
4. Character-vs-character physical contact (shoulder barging) beyond ORCA + separation: not planned.
