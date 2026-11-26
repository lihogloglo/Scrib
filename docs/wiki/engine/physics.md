# Physics, collision, items, ragdolls

## What it does

One rapier3d rigid-body world per worldspace behind our own API (rapier types never leave `sf_physics`), stepped at 60 Hz
in `FixedUpdate`. Static collision comes from baked NIF collision meshes and terrain heightfield chunks; placed items
are baked body shapes that sleep until touched; characters are kinematic capsules plus optional per-bone proxies. Ragdolls:
a cooked limp/powered Rapier ragdoll for beasts and creatures, and the optional Delirium active ragdoll for human NPCs.

## How it works

**World** (`superfargoth/crates/sf_physics/src/world.rs:151` `PhysicsWorld`): bodies (`BodyDesc` `body.rs`, `insert` `:275`,
`item`/`held` constructors `body.rs:90,96`), `Layer` bitset (`layer.rs`: static, terrain, prop, debris, ragdoll,
character, projectile, `BODY_PROXY`), interpolated poses (`pose_at` `:444`), events, joints with implicit PD drives
(`joint.rs`), floating origin by whole-cell `rebase`. `PhysicsProbe` answers the controller's ground queries. Carrying:
`carry.rs` (hands as kinematic targets).

**Bevy glue** (`sf_physics_bevy/src/lib.rs`): `PhysicsPlugin` steps the active worldspace's world at `PhysicsParams::hz` (the one rate: `Time<Fixed>` and the body `dt()` = 1/hz both come from it, IN11c2f)
(`:74`), capped by `max_steps_per_frame` (time dilates beyond it). `PhysicsFocus` (the streaming anchor) drives rebases.
`PhysicsBody` entities get interpolated pose written back. Streaming (`stream.rs`): `PhysicsHook` wraps placement
spawners, Active cells queue `Fixed` colliders (one shared mesh per model, built on the task pool) under
`insert_budget_ms` (the game hooks statics, doors, items, containers and, since 2026-11-26, activators: `mw_walkers_bevy/src/physics.rs` `hook`); terrain chunks 64 m at 0.5 m follow the active cells; an item gate waits for the floor.
Characters: `character.rs` (order since IN11b, `:104`: `PhysicsSystems::Step` before `ControllerSystems::Drive`, the
game also before `ActionSystems::Sample`, as UE/Jolt tick the base first; `base_velocities` / `refresh_base_velocity`
re-read the ground body's point velocity after the step; capsules target the feet plus the tick's mover motion;
`TickView` written after the movers; `CharacterFellOut`, safety net). Crush (IN11c2c2): a squeeze past `move.crush.depth_m` gives one `CharacterCrushed` per episode (movers automatic; action-body hosts call `sf_physics_bevy::crushed`); the static world never overlaps the character, the deck may. Lifts into ceilings (IN11c2c1): the ride's vertical part and the snap onto a moving base are swept up (`headroom`, `sf_controller/src/mover.rs:755`), stopping at the skin. Riding (IN11c2b): `refresh_base_motion` (`character.rs:126`) turns the base's pose change since the ground cast (`GroundHit::base_pose`) into the tick's twist, so a dropped physics step rides nothing and a doubled one rides both. Moving contacts (IN11c2a): `PhysicsProbe::contacts`
(`sf_physics/src/probe.rs:272`) finds awake kinematic bodies near each capsule (broad phase grown by this step's reach),
relative shape cast from `c + v·dt` back along -v for the entry face and separation; dynamic bodies never push (the
solver and `PhysicsEvent::Hit` own them). Overlap (IN33b1a): `body_overlaps_at(id, pose, layers)`
(`probe.rs:46`) asks whether a body placed at another pose would overlap a body in `layers` (shape intersection per
collider; a mesh tests its triangles, not its volume): the swing door's "is the next pose free" test; a Kinematic +
Mesh body pushes capsules through `contacts` like a box. `KinematicPlacements` (`sf_physics_bevy/src/stream.rs`,
IN33b1b): a streamed static listed there before its body drains in is inserted Kinematic (door leaves, lifts); read at
drain time only, the owner removes its entry. Proxies: `proxies.rs`. Carry: `carry.rs`.

**Collision from NIF** (`mw_bake/src/collision.rs:1-30`, `collision.cook`): OpenMW `bulletnifloader.cpp` rules (commit
4e718b50): skinned models none; `NC` markers; non-empty `RootCollisionNode` subtree; models `mesh.cook` skips and
markers use the rest-pose geometry; everything else all NIF triangles (not the render LOD). `.glb` derives from LOD0.
`StaticProbe` reads the products.

**Items** (`mw_bake/src/body.rs`, `mw_physics/src/lib.rs`): `body.cook` fits per model (`sf_physics/src/cook.rs`: authored
hulls, then a covering primitive, one hull, else V-HACD), `items.check` settles placed items. Which placements are items
and mass (`item_mass` from weight): `mw_physics`. Items insert asleep and stay put until disturbed (`undisturbed` flag,
`world.rs`). Cloth rules: `mw_physics/src/cloth.rs`.

**Ragdolls**: `sf_ragdoll` cooks `RagdollDef` (`cook.rs`, `mw_bake/src/ragdoll.rs`: humanoids 75 kg), `RagdollInstance`
(`instance.rs`), powered stable-PD drives (`powered.rs`), reaction machine (`react.rs`). Runtime `sf_physics_bevy/src/ragdoll.rs`;
walkers route deaths and hits through the `RagdollTakeover` hook (`mw_walkers_bevy/src/physics.rs:331`).
**Delirium** (`mw_delirium`, `mw_delirium_bevy`, GPL-3.0-only): builds a character from `Bip01` (`character_from_skeleton`),
`Takeover` (`takeover.rs:176`) from the animated pose, hit pushes with the balancer on (>= 10 N s; forced fall >= 100 N s),
deaths terminal and frozen, per-part proxies; hand-back to the animator as `PhysicsPose`. Step in `FixedUpdate`
(`mw_delirium_bevy/src/lib.rs:119`).

## Bevy features used

`FixedUpdate`, `Time<Fixed>`, observers (`let_go_on_remove`), messages (`CharacterFellOut`, `RagdollGotUp`). Rapier is the
engine; no avian. See [Locomotion](locomotion-and-action.md) for the mover that reads the probes.

## Switches and counters

`--off physics` (`mw_game_core/src/perf.rs:158`); `physics/...` counters under `SfDiagPlugin`; collider wireframes with the
`debug` feature. Delirium is on by default (`apps/bevyrrowind/Cargo.toml:19`); `--no-default-features` leaves it out.

## Known limits

Delirium's ~700 engine numbers come from the no-game fork's stand-in table (`delirium_data::kv::unlock_own`, opened in
`mw_delirium/src/sim.rs:17`): estimates from Delirium's own source, not GTA IV's, so behaviour differs from upstream. One Delirium
character per process (`nmrs::set`). PH-12 floor hole and PH-13 drop-on-load are open (see [ledger](../../ledger/physics.md)).
The player has no physics capsule (slice 11 waits for the owner).

## Specs

`superfargoth/docs/specs/physics.md` (Data model, Interfaces, Industry comparison); `bevyrrowind/docs/specs/physics-adapter.md`
(Items, Ragdolls, Cloth), `physics-fix-plan.md`, `character-shapes.md`.
