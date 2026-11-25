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

**Bevy glue** (`sf_physics_bevy/src/lib.rs`): `PhysicsPlugin` steps the active worldspace's world at `PhysicsSettings::hz`
(`:74`), capped by `max_steps_per_frame` (time dilates beyond it). `PhysicsFocus` (the streaming anchor) drives rebases.
`PhysicsBody` entities get interpolated pose written back. Streaming (`stream.rs`): `PhysicsHook` wraps placement
spawners, Active cells queue `Fixed` colliders (one shared mesh per model, built on the task pool) under
`insert_budget_ms`; terrain chunks 64 m at 0.5 m follow the active cells; an item gate waits for the floor.
Characters: `character.rs` (order since IN11b, `:104`: `PhysicsSystems::Step` before `ControllerSystems::Drive`, the
game also before `ActionSystems::Sample`, as UE/Jolt tick the base first; `base_velocities` / `refresh_base_velocity`
re-read the ground body's point velocity after the step; capsules target the feet plus the tick's mover motion;
`TickView` written after the movers; `CharacterFellOut`, safety net). Moving contacts (IN11c2a): `PhysicsProbe::contacts`
(`sf_physics/src/probe.rs:272`) finds awake kinematic bodies near each capsule (broad phase grown by this step's reach),
relative shape cast from `c + v·dt` back along -v for the entry face and separation; dynamic bodies never push (the
solver and `PhysicsEvent::Hit` own them). Proxies: `proxies.rs`. Carry: `carry.rs`.

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
