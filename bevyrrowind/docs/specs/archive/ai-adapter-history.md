# ai-adapter history

Text moved verbatim from [ai-adapter.md](../ai-adapter.md) "Navigation" when NV0d rewrote it for the nav lane roadmap
(2026-11-26): the PGRD runtime fallback is gone, routes, water bodies and flier/swimmer classes were added.

## Navigation before NV0d (2026-11-08 to 2026-11-26), with the NV4b as-built paragraph

- **Agent classes** (`sf.nav.agent` records from `mw_ai::nav`): `humanoid` (radius and height from the human capsule
  at race scale 1.0; small creatures use it too) and `large` (creature collision bounds above a size threshold). **Two
  classes, no `small`** (decisions (a)): no vanilla passage admits only small creatures, and a small class would cost
  ~4× the humanoid bake. MA1 prints the creature-radius histogram per class (a revisit trigger). OpenMW bakes one mesh
  per agent bounds (`components/detournavigator/navigator.hpp`).
- **Areas** (`sf.nav.area`), from OpenMW's `areatype.hpp` (ground, water, door, pathgrid) plus ours: `ground` 1,
  `water` (swimmers only; others high cost), `door` 2 (OpenMW's door cost; door leaves excluded from the bake geometry,
  the doorway footprint marked), `door_closed` (set at runtime on those polygons), `path` (pathgrid corridors, slightly
  cheaper so NPCs keep to streets), `hazard` (lava: avoid filter). No vanilla record sets `carves_nav`.
- **Pathgrid** (`mw_ai::pathgrid`, PGRD → `petgraph`, exists): edges become **off-mesh links** only where the navmesh
  has no connection between the endpoints (OpenMW `offmeshconnectionsmanager.hpp`); nodes stay as **wander hints** and
  **door approach points** (nearest node in front of each door, seamless research §4.11). Fallback when a worldspace
  has no navmesh: the pathgrid route, as OpenMW `pathfinding.cpp` 150-210 does.
- **Geometry**: exactly the cooked collision the mover uses (`collision.cook`), terrain from LAND, water level per
  worldspace; NIF collision quirks are the bake's (actors-adapter "Static collision").
- **As built (NV4b, 2026-11-11, `mw_bake::nav`)**: `MwNavSource` reads, per tile, the `mw.nav.cell` products (64 m
  squares listing every static/item/door whose world bounds touch them, plus the level-0 `terrain.tile`s under them;
  bytes = plan param, so a moved wall re-bakes only its cells' tiles), then `collision.cook` per model and the baked
  terrain heights (what the physics heightfield uses, not raw LAND). Actors, lights, markers, editor-only and
  start-disabled references are not geometry. Water: one quad at OpenMW's swim level (`getSwimLevel`: water −
  (`fSwimHeightScale` + 1) × half height, GMST read from the records), area `water`, only over tiles with terrain or
  objects; seabed kept. Doors: leaf excluded, `DoorVolume` = leaf footprint widened to 0.75 m each side, id = the
  reference's `PlacementId` folded to 64 bits; approach = nearest pathgrid node ≤ 6 m (`mw.nav.links`). Areas
  ground 1 / door 2 / water 3 (water cost 2); humanoid swims, `large` doesn't. Pathgrid off-mesh links: NV9.

