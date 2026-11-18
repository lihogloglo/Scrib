# Navmesh and AI

## What it does
The framework has a navmesh core with a Recast backend, a bake, a query service, a corridor driver and a debug overlay,
plus an AI core (HTN planner, directive stacks, drives, primitive move executors, decision traces). In the game, baked
navmesh tiles are resident around the player and ground `AI_W` wanderers plan through the `sf_ai` wander/idle HTN
method set (`mw_ai::methods`) and walk over `sf_nav` via `NavCorridor` (`PathWalkScaffold` is gone).

## How it works
- **Navmesh core** (`sf_nav`, Bevy-free): agent classes, `AreaTable`, `QueryFilter`, sliced `PathJob`, `PathCorridor`
  (`sf_nav/src/corridor.rs:88`: surface move, local push repair, lookahead trim, topology + visibility optimization,
  string-pulled corners), and the `NavBackend` trait (`sf_nav/src/backend.rs:344`: bake tile, residency, `nearest`,
  sliced `find_path`, `raycast`, `move_along_surface`, `corridor_corners`, `repair_corridor`).
- **Backend** (`sf_nav_recast`): `RecastBackend` over the pure-Rust `navmesh-rs` (Recast + Detour). One Detour mesh per
  (worldspace, agent class, region); tiles are DNAV v7; region-local f32 meters; stale refs fail by salt; corridor
  queries in `sf_nav_recast/src/corridor.rs`.
- **Bake** (`sf_nav_bake`): `nav.tiles` (per tile, reads recorded as dynamic inputs so a moved wall re-bakes only its
  tiles; door volumes get own polygons) and `nav.links` (off-mesh links). Backend-agnostic.
- **Morrowind adapter** (`mw_bake/src/nav.rs`): `MwNavSource` feeds the cooked collision and baked terrain heights;
  `bake_nav` (`:960`) runs inside `bevyrrowind bake` (`mw_boot_bevy/src/bake.rs:73`). Water is a flat swim-level quad,
  as in OpenMW.
- **Nav runtime shell** (`sf_nav_bevy`): `NavMeshes` resource, `nav.stats`, `nav.draw tiles|paths` debug layers, and
  `NavRingPlugin` (`residency.rs:158`): `sf_nav::ring::NavRing` (`ring.rs:209`) keeps the tiles within
  `ai.nav.ring.load_m` (256 m) of `NavAnchor` resident, drops them past `unload_m` (320 m), reads at most
  `tiles_per_frame` (4) store blobs per frame, nearest first. Scaffold `NavRingScaffold` (one anchor, synchronous
  reads) until NV5. Query service `NavQueryPlugin` (`sf_nav_bevy/src/queries.rs`, core `sf_nav/src/service.rs`): requests
  by `RequestId`, node-budgeted slices on `FixedUpdate`, replies at the next tick. Corridor driver `NavCorridorPlugin`
  (`sf_nav_bevy/src/corridor.rs:189`): `NavCorridor` steps `PathCorridor` in `NavSystems::Corridor` (inside
  `ControllerSystems::Drive`, between `Publish` and `Queries`) and feeds corners into `PathFollower`
  (`sf_controller/src/path.rs:158`) -> `MoveIntent`.
- **Game side** (`mw_walkers_bevy/src/nav.rs:41`, added at `exterior.rs:244`): Recast backend, humanoid class only,
  `NavPlugin`, `NavQueryPlugin`, `NavCorridorPlugin` and `NavRingPlugin`, anchor = the first `Gameplay` streaming
  anchor, draw frame from the floating origin. A bake without `nav.tiles` logs one line and adds nothing. Debug draw is
  compiled into every build for now (the crate enables `debug`).
- **AI core** (`sf_ai`, Bevy-free): ids, decision `trace`, typed `WorldState`, HTN `Library` and planner (`htn`,
  MTR compare, bounded backtracking), per-actor `DirectiveStack` with `ModernPolicy` (`directive`), `AiSave` shape for
  the `sf/ai` section (`save`), drives (`drive`: `sf.ai.drive`/`sf.ai.consideration` records, IAUS curves, pure
  `arbitrate` at `drive.rs:614` with momentum, minimum commit and the directive floor), and primitive move executors
  (`exec`: `MoveTo`, `Wait`, `Face`, `OperatorExec`, `PlanRunner` at `exec.rs:321` with dirty-key MTR replan,
  `Library::holds` per-step monitoring and `Pause` resume).
- **AI shell** (`sf_ai_bevy`): `AiPlugin`, trace store, `AiLibrary` loaded from records, `ai.select`, optional gizmos,
  and `AiExecPlugin` (`exec.rs:151`: `AiExecutor`, `AiSystems::Execute` in `FixedUpdate` inside
  `ControllerSystems::Drive` before `NavSystems::Corridor`, tunables `ai.exec.*`).
  Added by `mw_walkers_bevy::walkers::add` (`walkers.rs:768`); the `sf/ai` save section exists in
  `sf_save_bevy/src/sections/mod.rs:282` (filled once MA2 wires directive stacks).
- **What NPCs actually do** (`mw_ai`, Bevy-free; `mw_walkers_bevy/src/walkers.rs`, added at `exterior.rs:229`):
  `WanderVocabulary` + `WanderDriver` (`mw_ai/src/methods.rs:55,156`: HTN tasks `wander` / `do_idle` / `do_walk` over
  `sf_ai::exec::PlanRunner`, `AI_W` idle rolls and PGRD wander destination hints from `wander.rs`) drive ground
  wanderers over the resident navmesh via `NavCorridor` (`walkers.rs:703`), recording decisions into `AiTraces`
  (`AiActor` from `CharacterPlacement`), and falling back to PGRD polyline `PathFollower` walks only for fliers/swimmers
  or unbaked cells (`PathWalkScaffold` removed in MA3). No combat AI, schedules, reactions, perception or following yet.

## Bevy features used
Resources and console commands only; gizmos behind feature `debug`. No Bevy navmesh crate (surveyed in the spec).

## Switches and counters
`nav.stats` (tiles per class, polys, MiB, queue/tick stats, and a `ring` part: pending, empty, unbaked, failed, loaded, dropped, last ms),
`nav.draw tiles|paths|off` (outlines along the detail mesh, `TileGeometry::for_each_edge`), tunables `ai.nav.ring.*`,
`ai.nav.query.*`, `ai.nav.corridor.*`, `ai.exec.*` (live). Log: `nav ring: N tiles resident in <worldspace> (...)` once
per worldspace. `ai.select`, `ai.trace`, `ai.tiers`, `ai.stats` and `ai.gizmos` work in game on `AiActor` entities. Walkers' physics follows
`--off physics` (`exterior.rs:241`). No nav or AI `--off` part yet.

## Known limits
One anchor (the player), humanoid class only, synchronous store reads: portals and the large class wait for
NV5 ([ledger](../../ledger/nav.md)). Only `AI_W` wander runs on `sf_ai` so far; packages (`MA2`), drives (`MA4`) and
perception (`AI8`/`MA5`) are next. See `docs/ROADMAP.md` 3.9/3.10.

## Specs
`superfargoth/docs/specs/nav.md`, `ai.md`, `ai-as-built.md`, `orca.md`; `bevyrrowind/docs/specs/ai-adapter.md`,
`actors-adapter.md`.
