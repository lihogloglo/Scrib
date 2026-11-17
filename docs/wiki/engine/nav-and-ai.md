# Navmesh and AI

## What it does
The framework has a navmesh core with a Recast backend, a bake and a debug overlay, plus an AI core (HTN planner,
directive stacks, drives, decision traces). In the game, baked navmesh tiles are resident around the player (a
scaffold ring), but nothing paths on them yet and the AI core is not wired in: NPCs still wander on Morrowind's PGRD
pathgrids through a named scaffold.

## How it works
- **Navmesh core** (`sf_nav`, Bevy-free): agent classes, `AreaTable`, `QueryFilter`, sliced `PathJob`, and the
  `NavBackend` trait (`sf_nav/src/backend.rs:253`: bake tile, residency, nearest, sliced `find_path`, raycast).
- **Backend** (`sf_nav_recast`): `RecastBackend` over the pure-Rust `navmesh-rs` (Recast + Detour). One Detour mesh per
  (worldspace, agent class, region); tiles are DNAV v7; region-local f32 meters; stale refs fail by salt.
- **Bake** (`sf_nav_bake`): `nav.tiles` (per tile, reads recorded as dynamic inputs so a moved wall re-bakes only its
  tiles; door volumes get own polygons) and `nav.links` (off-mesh links). Backend-agnostic.
- **Morrowind adapter** (`mw_bake/src/nav.rs`): `MwNavSource` feeds the cooked collision and baked terrain heights;
  `bake_nav` (`:956`) runs inside `bevyrrowind bake` (`mw_boot_bevy/src/bake.rs:70`). Water is a flat swim-level quad,
  as in OpenMW. Scaffold: `BEVYRROWIND_BAKE_SKIP_NAV=1` skips it (`bake.rs:65`).
- **Nav runtime shell** (`sf_nav_bevy`): `NavMeshes` resource, `nav.stats`, `nav.draw tiles|paths` debug layers, and
  `NavRingPlugin` (`residency.rs:158`): `sf_nav::ring::NavRing` (`ring.rs:209`) keeps the tiles within
  `ai.nav.ring.load_m` (256 m) of `NavAnchor` resident, drops them past `unload_m` (320 m), reads at most
  `tiles_per_frame` (4) store blobs per frame, nearest first. Scaffold `NavRingScaffold` (one anchor, synchronous
  reads) until NV5. Query service `NavQueryPlugin` (`sf_nav_bevy/src/queries.rs`, core `sf_nav/src/service.rs`): requests
  by `RequestId`, node-budgeted slices on `FixedUpdate`, replies at the next tick; not added by the game until AI6.
- **Game side** (`mw_walkers_bevy/src/nav.rs:41`, added at `exterior.rs:244`): Recast backend, humanoid class only,
  anchor = the first `Gameplay` streaming anchor, draw frame from the floating origin. A bake without `nav.tiles`
  logs one line and adds nothing. Debug draw is compiled into every build for now (the crate enables `debug`).
- **AI core** (`sf_ai`, Bevy-free): ids, decision `trace`, typed `WorldState`, HTN `Library` and planner (`htn`,
  MTR compare, bounded backtracking), per-actor `DirectiveStack` with `ModernPolicy` (`directive`), `AiSave` shape for
  the `sf/ai` section (`save`), and drives (`drive`: `sf.ai.drive`/`sf.ai.consideration` records, IAUS curves, pure
  `arbitrate` at `drive.rs:614` with momentum, minimum commit and the directive floor). No executors yet.
- **AI shell** (`sf_ai_bevy`): `AiPlugin`, trace store, `AiLibrary` loaded from records, `ai.select`, optional gizmos.
  Not added by the game; the `sf/ai` save section exists in `sf_save_bevy/src/sections/mod.rs:282` but nothing fills it.
- **What NPCs actually do** (`mw_ai`, Bevy-free; `mw_walkers_bevy/src/walkers.rs`, added at `exterior.rs:229`):
  PGRD pathgrids as per-cell graphs with A* (`pathgrid`), the `AI_W` wander package and OpenMW's idle roll (`wander`),
  and `PathWalkScaffold` (`scaffold.rs:57`) turning wander steps into `sf_controller` `PathFollower` walks. It is
  named a scaffold "until `sf_ai` exists". No combat AI, schedules, reactions, perception or following.

## Bevy features used
Resources and console commands only; gizmos behind feature `debug`. No Bevy navmesh crate (surveyed in the spec).

## Switches and counters
`nav.stats` (tiles per class, polys, MiB, and a `ring` part: pending, empty, unbaked, failed, loaded, dropped, last ms),
`nav.draw tiles|paths|off` (outlines along the detail mesh, `TileGeometry::for_each_edge`), tunables `ai.nav.ring.*` (live). Log: `nav ring: N tiles resident in <worldspace> (...)` once
per worldspace. `ai.select` only in the `ai_overlay` example (AI plugin not in the game). Walkers' physics follows
`--off physics` (`exterior.rs:241`). No nav or AI `--off` part yet.

## Known limits
One anchor (the player), humanoid class only, synchronous store reads: NPC paths, portals and the large class wait for
NV5 ([ledger](../../ledger/nav.md)). Warm nav bake not re-measured since NV4f, so `BEVYRROWIND_BAKE_SKIP_NAV` stays.
Walkers ignore the navmesh, the HTN and the drives. See `docs/ROADMAP.md` 3.9/3.10.

## Specs
`superfargoth/docs/specs/nav.md`, `ai.md`, `ai-as-built.md`, `orca.md`; `bevyrrowind/docs/specs/ai-adapter.md`,
`actors-adapter.md`.
