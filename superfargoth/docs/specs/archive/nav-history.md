# nav.md history

As-built notes moved out of `nav.md` ("Build plan") to keep the spec under its budget. Newest last.

## NV1 as built (2026-11-11; moved from nav.md at NV2)

`sf_nav` deps: `sf_world`, `sf_coords`, glam, serde, bitflags, thiserror, postcard
(grid blobs); no Bevy, no navmesh-rs. Departures from the sketch in nav.md "Interfaces": `NavBackend` has `build_tile(input, cfg,
areas)` (flags from our `AreaTable`), `add_tile -> TileRef`, `tile`, `nearest`, sliced `find_path(&self, &mut
PathJob, &mut SliceBudget) -> JobState` (the job carries the backend's search state, `Box<dyn Any + Send>`, so one
backend serves many workers; a removed tile fails it with `MeshChanged`, added tiles don't), `raycast(from, to:
&Location)`, `poly_state` / `set_poly_state` (doors as `PolyState { area, flags }` on poly lists; `DoorPolys` wraps it
in NV10); corridor, boundary and obstacle methods come with NV7b, NV8, NV10. `QueryFilter` = Detour's
include/exclude flags + 64 area costs (∞ = UE excluded area); `AreaTable` costs ≥ 1 (Detour's heuristic). `PathResult { status, polys,
waypoints, cost }`, links with NV9; `TraverseKind::Door` with `LinkId` (NV11). `GridBackend`: 2.5D, one surface per
column, 8-neighbour exact A* without corner cutting, climb limit, Detour-style partial results.

## NV2 as built (2026-11-11)

`sf_nav_recast` (Bevy-free; deps `sf_nav`, `sf_world`, `sf_coords`, glam, `navmesh-rs =0.2.2` with `libm`,
`recast`, `detour`, `polyref64`, vendored with one own patch). `RecastBackend::new(RecastConfig { classes:
Vec<BuildConfig> (tile size per class), max_tiles (≤ 2^13 per mesh), max_nodes (4,096), max_path (2,048) })`.
- **Meshes**: one Detour `NavMesh` per (worldspace, class, region), created with its first tile, dropped with its
  last; ≤ 256 resident. Each region has a `WorldPos` origin (`set_region_origin`, default the worldspace's, fixed
  while tiles are resident); tile grids start at it, so meshes work in region-local f32.
- **Refs**: our `PolyRef` = the Detour 64-bit ref (salt 16 | tile 28 | poly 20) with the mesh slot (8 bits) and
  its generation (7 bits) in tile bits 33..48, unused below 2^13 tiles. Stale refs fail by Detour salt or our
  generation; a job also checks a per-mesh serial, so a dropped and re-created mesh fails it.
- **Bake** (`build_tile`): the Recast demo's per-stage tile pipeline (`Sample_TileMesh::buildTileMesh`): slope
  marking keeps the adapter's triangle areas, tile + border (radius + 3 cells) rasterised, the three span filters,
  erosion, `AreaVolume`s as convex area marks, watershed regions, contours, polygons, detail mesh, DNAV with
  flags from `AreaTable::flags(area)` (not navmesh-rs's `bake` default `1 | 1 << area`). Demo settings for edge
  length (12 m), contour error, region sizes, detail sampling. An empty `Vec` = no polygons (the bake stores none;
  `add_tile` rejects it). Blobs are bare DNAV v7; LZ4 is the bake store's (NV4).
- **Sliced A*** (the NV1 handoff's open choice): each job owns a navmesh-rs `DetachedQuery` (node pool, open
  list, sliced state) and attaches it to its mesh only for one `updateSlicedFindPath` step, so
  `find_path(&self, &mut PathJob, &mut SliceBudget)` serves any number of workers, no `unsafe`, no query bound to
  a mesh's lifetime. `DetachedQuery` was `pub(crate)` and `crowd`-only (DetourCrowd's `PathQueue` uses it): the
  vendored patch makes it public (`superfargoth/vendor/README.md`, to offer upstream). Rejected: a per-worker
  `NavMeshQuery<'mesh>` pool (self-referential inside the backend, one sliced search per query, so a worker could
  not interleave jobs); re-running A* from scratch per slice. Finished jobs return their query to a pool of 16.
- **Mesh changes**: Detour's epoch and node-pool tile check map onto the contract: added tiles tolerated, a
  removed tile the search touched fails the job with `MeshChanged` (an untouched one does not: finer than
  `GridBackend`, which fails on any removal).
- **Filter**: Detour's `QueryFilter` from ours (`pass_filter` = include/exclude flags + finite area cost; cost =
  distance × the polygon's area cost). Detour's heuristic (0.999 × distance) stays admissible since costs ≥ 1.
- **Results**: corridor from `finalizeSlicedFindPath` (Detour's partial result = visited polygon nearest the goal,
  ending at the closest point on it), waypoints from `findStraightPath` with a vertex at each area change, cost =
  straight-path length per segment × its area cost.
- **Goals**: `Near` = the A* to `at` (snapped with the extent widened to the radius), the string-pulled path cut
  where it first comes within the radius (Complete then; corridor trimmed to the cut's polygon). `AnyOf` = one A*
  per candidate by straight distance, pruning candidates whose straight distance × the cheapest cost already
  exceeds the best complete path (EQS-style per-item test; Detour's A* has one end polygon). `Away` =
  navmesh-rs's self-owned resumable Dijkstra (`begin_polys_around_circle`, one polygon per budget unit) until the
  first polygon whose centroid is `min_dist` from the threat; exhausted: the farthest, `Partial`.
- **Raycast**: Detour `raycast` (2D along the surface), end height from the last polygon's detail mesh.
- **Extras**: `tile_polys(key)` (door polygon lists for NV4/NV10, debug draw NV3), `resident_tiles`,
  `resident_bytes`. `sf_nav` gained `NavError::Backend(String)` and `SliceBudget::charge`.
- **Not yet**: off-mesh connections (NV9), corridor and boundary (NV7b/NV8: `PathCorridor` and `LocalBoundary`
  are public in navmesh-rs's `crowd` module and usable without the `Crowd` manager, so NV8 turns that feature on),
  TileCache (NV10), the bake's `parallel` feature (NV4).
- **Tests** (18 + 1 measurement, 0.2 m voxels, 8 m tiles): open floor across tiles, a wall gap, ramps within and
  over 45°, two floors joined by 15 stairs, door polygons locked / shut / reopened with zero rebuilds (reopened =
  the first path exactly), raycast hit at the eroded wall edge and clear through the gap, sliced (1, 2, 5 per step)
  = one-shot for all four goal kinds, removal of touched vs untouched tiles and of a whole mesh, goal kinds and
  their errors, partial results, area costs and SWIM exclusion, flags = `AreaTable`, nearest within extent and
  class, region origins, residency errors and stale refs, per-class meshes (a 1.2 m gap passes the humanoid, not
  the large class). Every path: finite and each waypoint on the mesh. The NV1 grid scenarios were rewritten for
  geometry rather than shared: exact cell paths don't carry over to polygons.
- **Measured** (`measure_bake_and_queries`, release, owner's laptop, 2026-11-11): a 6 × 6 tile synthetic town at
  the NV0 calibration (0.15 m voxels, 128 per tile = 19.2 m; houses with roof ramps): 4.6 ms per tile mean,
  5.6 ms worst, 4.8 KiB DNAV per tile, `add_tile` 24 µs; `nearest` 1.2 µs, a 115 m diagonal path 48 µs (57
  polygons, 443 expansions, 7 slices of 64), raycast 2.0 µs. Real cells are denser (NV0: Balmora).

## NV3 as built (2026-11-11)

`sf_nav_bevy` (thin Bevy shell: `bevy_app`, `bevy_ecs`, `sf_console_bevy`, `serde_json`; feature `debug` adds
`bevy_gizmos` + `bevy_color`; the backend is a `dyn NavBackend`, so no navmesh code compiles in it).
- **Trait** (`sf_nav`): `tile_summaries() -> Vec<TileSummary { key, bytes, polys }>` and `tile_geometry(key) ->
  Option<TileGeometry { origin, verts, polys: Vec<DebugPoly { verts, walls, state }> }>`, default empty / `None`.
  Recast: DNAV bytes, ground polygons only (off-mesh connections are the links layer), a wall is `neis == 0`
  (Detour's `duDebugDrawNavMeshPolyBoundaries`). `GridBackend`: one square per walkable cell, a side is a wall when
  the cell across isn't walkable or resident.
- **Resources**: `NavMeshes`, `NavCounters` (queue depth, ms per tick, rebuilds: `None` = `n/a` until NV6 / NV10),
  `NavDebugPaths` (actor label -> waypoints, `record(actor, &PathResult)`), `NavDraw` (layers), `NavDrawFrame`
  (render origin `WorldPos` + worldspace shown, the game updates it from its floating origin), `NavDrawn`.
- **Console**: `nav.stats` -> `nav: 5 tiles (class 0: 4, class 1: 1), N polys, X MiB, queue n/a, tick n/a ms,
  rebuilds n/a`; `nav.draw tiles` / `paths [actor]` toggle, `off` clears; links (NV9), doors (NV4), obstacles (NV10),
  boundary (NV7b), coarse (NV12), graph (NV11) warn `not built yet (NVx)`.
- **Gizmos**: rebuilt from the backend every frame in `PostUpdate` (tiles stream and door patches change state, so no
  retained mesh), area colour by golden-angle hue, inner edges alpha 0.35, walls solid, disabled grey, 5 cm lift;
  paths yellow with a 0.3 m tick per waypoint. Runs only with a `GizmoConfigStore` and a layer on.
- **BRP**: `brp_stats_system(In<Option<Value>>, &World) -> Result<Value, String>`, the sf_ai_bevy shape; the app that
  owns `RemotePlugin` maps the error (not registered in the game yet).
- **Tests** (3): smoke over a test bake (floor, box, ramp; 4 humanoid + 1 large tile): exact stats line and BRP
  JSON, gizmos drawn for a few frames (`NavDrawn` = 5 tiles, every polygon, every waypoint), worldspace filter,
  not-built layers, `off`; Recast tile geometry finite and inside its tile; grid double's walls exact.

## NV4a as built (2026-11-11)

- **Crate** `sf_nav_bake` (Bevy-free, hakari final-excludes; deps `sf_bake`, `sf_nav`, `sf_world`, `sf_coords`, glam,
  postcard, serde; dev `sf_nav_recast`, tempfile). `NavBake::new(source, backend, NavBakeConfig { classes, areas,
  backend_tag })`, `register(&mut Baker)`, `tile_keys(ws, class)`, `class_products`, `products(worldspaces)` (the
  host's bake roots: links, then per class every tile over the source's region extents, then the coarse graph).
- **Subjects**: `nav.tiles` `class/region/x/z/worldspace`, `nav.links` `worldspace`, `nav.coarse` `class/worldspace`
  (worldspace last: ids may hold `/`). Params: the class's `BuildConfig`, the `AreaTable`, `backend_tag`
  (`sf_nav_recast::BAKE_TAG`); `nav.coarse` also the sorted regions and plans every tile + the links as product
  inputs (scheduled first, so its reads are hits).
- **`nav.tiles` run**: bounds = tile ± `BuildConfig::border()` (radius in cells + 3, now shared with `build.rs`);
  `source.tile_input(cx, ws, class, bounds)`; door volumes sorted by id and appended last to `TileInput::volumes`;
  `build_tile`; empty blob → zero bytes; else `read_tile_blob` and door polygons = polygons with the door's area whose
  centroid lies in its footprint (y range ± 2 cell heights), first door by id wins; `BakedTile { blob, doors }`.
- **`nav.coarse`** (stub for NV12): per tile, connected polygon sets by union-find over in-tile edge links; per (set,
  side) portal edges merged into runs (1 cm join) → one entrance node at the run's middle on the tile border; door
  approach points placed on the covering polygon nearest in height (≤ class height), else listed in `unplaced`;
  edges = every pair within one (tile, set) plus +x/+z entrances to the neighbour's opposite entrances whose spans
  overlap (height ≤ class height); costs straight-line meters.
- **`sf_nav` additions**: `NavDoorId`, `BuildConfig::border_cells/border`, `TileSide`, `PolyEdge`, `BlobPoly`,
  `BlobTile`, `NavBackend::read_tile_blob` (default `Unsupported`; Recast decodes with `MeshTile::parse`, Detour
  `neis` → wall / poly / portal side), module `baked` (`BakedTile` with `encode`/`decode`, `DoorPolys`,
  `DoorApproach`, `NavLinks`, `CoarseGraph` / `CoarseNode` / `CoarseEdge` / `CoarseNodeKind`).
- **Tests** (6, `sf_nav_bake`): a bucketed synthetic town in in-memory files (10 m buckets; a tile reads the buckets
  under its bounds): subject round trip; rerun = full hit (10 tiles, 2 coarse); empty tiles exactly the void column;
  two doorways each with their own `DOOR` polygons, every door-area polygon listed once, approach points placed,
  cross-tile and door edges present; moving one crate re-bakes exactly the tiles that read its bucket (2 of 10).
  Measurement (dev profile, other builds running): 8×8 houses, 122 m square, humanoid cs 0.15 / 128 cells: 49 tiles,
  wall 183 ms, 14.7 ms CPU per tile, 0.26 MiB (uncompressed); large: 16 tiles, 33 ms, 4.7 ms per tile, 0.08 MiB;
  coarse graph 488 nodes, 1,600 edges.

## Industry comparison table (moved from nav.md at NV4a)

"Unverified" marks claims from memory not re-read on 2026-11-08.

| Technique | Industry | Ours | Verdict |
|---|---|---|---|
| Navmesh generation | Recast voxelisation: UE `RecastNavMesh`, Unity AI Navigation (`NavMeshSurface` per agent type), Godot `NavigationServer3D`, OpenMW `detournavigator` | Recast via backend trait | **match** |
| When it's built | UE static/dynamic tiles, nav invokers; OpenMW async around the player + disk cache + `navmeshtool`; Bethesda authored per cell | baked per tile, mod-aware, patched at runtime | **match** UE static + dynamic modifiers; differ from OpenMW (nothing bakes during play) |
| Agent sizes | UE supported agents, Unity agent types, OpenMW one mesh per agent bounds | 2 classes as data | **match** |
| Area costs | UE `NavArea` + filters, Unity areas, OpenMW ground/water/door/pathgrid (door 2) | area table + per-query filters | **match** |
| Runtime obstacles | Detour TileCache, Unity `NavMeshObstacle` carving, UE dynamic obstacles / modifiers | door poly area/flags (Recast demo, OpenMW door area); TileCache carving opt-in, at rest | **match** |
| Off-mesh links | UE `NavLinkProxy`, Unity `NavMeshLink`, OpenMW pathgrid edges, Bethesda door triangles | jump/drop/ladder/pathgrid/door links | **match** |
| Across load doors | Bethesda: teleport out of view; OpenMW: none (followers teleport); UE WP streams nav with cells | `WorldGraph` + traverse; fade links only unseen | **match** Bethesda; **ahead** of OpenMW |
| Hierarchical routing | HPA* (Botea 2004); Killzone/Decima graph abstraction (unverified) | worldspace + coarse graph + Detour | **match** |
| Path following | Detour `dtPathCorridor`, UE `UPathFollowingComponent` | Detour corridor + our `PathFollower` | **match** |
| Local avoidance | DetourCrowd (sampled RVO), UE RVO, Unity agents, Godot RVO2 | our ORCA + navmesh walls, fixed tick | **match**; deterministic |
| Debug | UE nav display + Visual Logger, Recast demo, OpenMW navmesh render | gizmos + `nav.*` + BRP | **match** |
| Scaling cost | tiles: ring area × classes present × anchors; queries: path length (coarse graph bounds it); rebuilds: carving props; avoidance: tier 1-2 agents | caps per CPU tier | checked |

**Sources**: Mononen, recastnavigation (https://github.com/recastnavigation/recastnavigation); Botea, Müller, Schaeffer,
"Near Optimal Hierarchical Path-Finding" (JoGD 2004); van den Berg et al., ORCA (2011); Unity AI Navigation docs;
UE navigation docs; OpenMW `components/detournavigator/` at `b8285cc9` and the navigator settings page (research
"Sources").

## Done build-plan rows (moved from nav.md at NV4a)

| Slice | Deliverable | Test |
|---|---|---|
| **NV0** spike (**done 2026-11-11**) | `navmesh-rs` verdict on our cooked collision (parity, ms per tile, MiB, query µs, code read, tile format, setters) | [report](../research/navmesh-spike-nv0-2026-11-11.md): adopt |
| **NV0b** (not needed) | FFI fallback spike (recastnavigation bindings) | same |
| **NV1** core (**done 2026-11-11**) | `sf_nav`: types, `NavBackend`, `GridBackend` double, `Goal`, filters | exact paths on synthetic grids (17 tests) |
| **NV2** backend (**done 2026-11-11**) | `sf_nav_recast`: `build_tile`, tiles, `nearest`, sliced `find_path`, `raycast` | paths on synthetic boxes, ramps, two floors; no NaN (18 tests + measurement) |
| **NV3** debug draw (**done 2026-11-11**) | `sf_nav_bevy` gizmos + `nav.draw` / `nav.stats` + BRP over a test bake | headless plugin smoke; stats line counts tiles |
| **NV4a** bake steps (**done 2026-11-11**) | `nav.tiles`, `nav.links`, `nav.coarse` steps + `NavGeometrySource`, door polygons (`sf_nav_bake`) | synthetic: cache hit on rerun; moving one box re-bakes only its tiles; each doorway has its own polys; empty tiles empty; ms and MiB printed per class |
