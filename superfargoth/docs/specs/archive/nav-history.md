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
| **NV0** spike (**done 2026-11-11**) | `navmesh-rs` verdict on our cooked collision (parity, ms per tile, MiB, query µs, code read, tile format, setters) | [report](../../research/navmesh-spike-nv0-2026-11-11.md): adopt |
| **NV0b** (not needed) | FFI fallback spike (recastnavigation bindings) | same |
| **NV1** core (**done 2026-11-11**) | `sf_nav`: types, `NavBackend`, `GridBackend` double, `Goal`, filters | exact paths on synthetic grids (17 tests) |
| **NV2** backend (**done 2026-11-11**) | `sf_nav_recast`: `build_tile`, tiles, `nearest`, sliced `find_path`, `raycast` | paths on synthetic boxes, ramps, two floors; no NaN (18 tests + measurement) |
| **NV3** debug draw (**done 2026-11-11**) | `sf_nav_bevy` gizmos + `nav.draw` / `nav.stats` + BRP over a test bake | headless plugin smoke; stats line counts tiles |
| **NV4a** bake steps (**done 2026-11-11**) | `nav.tiles`, `nav.links`, `nav.coarse` steps + `NavGeometrySource`, door polygons (`sf_nav_bake`) | synthetic: cache hit on rerun; moving one box re-bakes only its tiles; each doorway has its own polys; empty tiles empty; ms and MiB printed per class |

## Moved from nav.md 2026-11-26 (NV0b)

NV0b brought nav.md to the nav lane roadmap (`../../../../docs/roadmaps/nav.md`). Below, verbatim: the build-plan "As built"
paragraphs (NV1-NV11d1), then the lines NV0b replaced (old wording kept for history).

### As built, NV1-NV11d1

**As built** (full notes: [nav-history.md](nav-history.md)). **NV1**: the trait as in "Interfaces",
search state in the job (one backend serves many workers). **NV2**: one Detour mesh per (worldspace, class, region),
region-local f32; per-job `DetachedQuery` (vendored patch); synthetic town at the NV0 calibration: 4.6 ms per tile,
`nearest` 1.2 µs, a 115 m path 48 µs. **NV3**: `sf_nav_bevy` (`NavMeshes`, `nav.stats` / `nav.draw`, BRP
`sf_nav/stats`, gizmos behind `debug`); not wired into the game yet. **NV4a**: `sf_nav_bake` steps as in "Bake";
door polygons by area + centroid in the door volume; `nav.coarse` is an HPA* stub (straight-line costs until NV12).
**NV4c**: `sf_nav::ring::NavRing` (tile set per class and region around one anchor, load/unload radii, nearest-first
reads, re-plans only on a tile or worldspace change) + `sf_nav_bevy::residency::NavRingPlugin` (store reads by `nav.tiles`
subject, cap per frame); tunables `ai.nav.ring.*`; game side `mw_walkers_bevy::nav` (humanoid only, anchor = first
`Gameplay` streaming anchor). Scaffold `NavRingScaffold`, removed by NV5.
**NV6**: `sf_nav::service::QueryService` (per-tick node allowances fixed before jobs run, so serial, reordered and parallel
runs give the same replies; a job finished in tick t is ready at t+1; ring-edge goals become `Partial` via `NavCoverage`,
which `NavRing` implements) + `sf_nav_bevy::queries::NavQueryPlugin` (`FixedFirst` publish, `FixedUpdate` run, replies in a
map keyed by `RequestId`, not a Message: staggered AI polls its id). Scaffold `NavQueryOnMain`: slices run inside the tick.
**NV7a**: NPC drive + ORCA + mover in `FixedUpdate` (`ControllerSystems::Drive` → `Step`, after the player's move, before
`PhysicsSystems::Step`); `Mover` keeps the tick's start/end poses, `ControllerSystems::Interpolate` draws them by overstep
(the player's `TickPose`, now in `sf_controller::frame`). **NV8**: `sf_nav::corridor::PathCorridor` (`move_along_surface`,
local repair by bounded partial search when pushed past `repair_drift_m` or on an invalid start polygon, lookahead
trimming, periodic topology optimization, visibility shortcutting, string-pulled corners) + `sf_nav_recast::corridor`
and `GridBackend` implementations; `sf_controller::PathFollower::set_corridor_corners` preserves spring state across
ticks; `sf_nav_bevy::corridor::NavCorridorPlugin` (`NavCorridor`, `NavSystems::Corridor` in `FixedUpdate` inside
`ControllerSystems::Drive`, `.before(NavSystems::Queries)` (`NavSystems::Publish` runs earlier, in `FixedFirst`, so the `.after(Publish)` in code is a no-op), tunables `ai.nav.corridor.*`).

**NV11a** (2026-11-26): `sf_world::LinkId(u64)` (`records.rs`; serde hex text / u64 binary like `PlacementId`),
`DoorLink::id()`: blake3 (64-bit cut) over tag `sf.door_link.id/1`, `two_way`, each end's length-prefixed worldspace,
position quantised to 1 mm (absolute f64), normalised quaternion to 1e-4 with sign canonical (q == -q); two-way ends
sorted, one-way kept in order; arrival, door and extent excluded (retuning keeps the id). Golden `49e3c99d02b35c8f`.
`DoorLink::find(&[DoorLink], id)` / `DoorLinks::get` are linear (NV11b's graph builds the index).

**NV11b** (2026-11-26): `sf_nav::world_graph`: `WorldGraph::build(links, approach, flags)` (adapter gives approach
points and `LinkFlags` SEAMLESS / FADE_ONLY / ONE_WAY; links without an approach end or duplicate ids are skipped and
counted), `route(from, to, door_state, &RouteRules { speed_mps, traverse, unlock })` -> `WorldRoute { links, cost_s,
legs, final_walk_s }` (serde). Cost = seconds: straight walk to each departure approach point + traverse
(`SEAMLESS_TRAVERSE_S` 3, `FADE_TRAVERSE_S` 1, Guess, overridable) + straight walk to the goal; A* with heuristic 0
outside the goal's space (admissible, not consistent: re-expansion on improvement); ties by `(LinkId, side)`; locked
refused unless `unlock`, closed passable; one-way leaves only from `a`. Same-space route empty. Door state is per link,
not per side; fade-only visibility is the traverse step's (AI12); straight in-space costs until NV12.

**NV11c** (2026-11-26): `sf_nav::portal_query`: `map(loc, link, from: LinkSide)` / `map_direction` (side explicit:
both ends may share a worldspace); `WorldGraph::path_distance(a, b, &PathRules { max_hops, closed_door_m, door_state })`
-> `PortalPath { metres, links, closed_doors }` (straight legs through opening centres, every link carries sound, a
non-open link counts in `closed_doors` and costs `closed_door_m` when choosing, not in `metres`; hop-bounded Dijkstra);
`WorldGraph::line_of_sight(a, b, door_state, &impl LosProvider)` -> `Los::{Clear, Blocked, BlockedByLink}`: across
spaces only through one seamless open link joining them directly, aimed at the point where the line crosses the door
plane (must lie inside the opening), then viewer -> point and mapped point -> target. `DoorState::openness()` (Open 1).
`GraphLink` keeps its `DoorLink`; `WorldGraph::door_link(id)` is the index. Gaps: sight crosses one link; the LOS
provider must ignore the door leaf at the plane; `PortalPath` doesn't name the opening; `Cost::Walk` waits NV12.

**NV11d1** (2026-11-26): `sf_nav_bevy::world_graph`: `NavWorldGraph`, `NavWorldGraphStats`, `NavLinkRules` (boxed
`approach` / `flags`; defaults `arrival_approach` = the end's `arrival.pos`, `classic_flags` = FADE_ONLY, + ONE_WAY when
not two-way), `NavDoorStates` (unreported link = Closed: routes pass, sight blocked), `NavWorldGraphPlugin<R>` reading
any host links resource through an accessor (`NavWorldGraphPlugin::new(|l: &DoorLinks| &l.0)`: no `sf_world_bevy`
edge into the thin shell); rebuild in `Update` set `NavSystems::WorldGraph` only on change, logs `nav: world graph N
links (M skipped)`; console `nav.links`. The game adds the plugin (not part of `NavPlugin`); game approach points
(pathgrid nodes) and seamless flags are a later adapter slice (MA21).

### Replaced lines (pre-NV0b wording)

Agent summary (2026-11-11):

<!-- agent-summary -->
> **Agent summary (2026-11-11).** Draft; NV1 (`sf_nav` core) and NV2 (`sf_nav_recast`) built ("Build plan"). Recast/Detour
> navmesh per worldspace, agent class and nav region, baked once by `sf_bake` (`nav.tiles`); two classes (humanoid, large).
> Doorways are baked door polygons switched at runtime by area/flags; prop carving (TileCache) is opt-in later (owner:
> patches are not "baking during play"). Tiles stream with a fixed nav ring around gameplay anchors. Routing:
> worldspace graph (door links) → per-space route graph (NV12) → Detour over resident tiles → corridor → our ORCA (+ navmesh
> wall lines), on the fixed tick (NV7a). Backend behind `NavBackend`: `navmesh-rs` (NV0 verdict).
> Read next: "Data model", "Routing", "Streaming", "Build plan".

Goal, non-goals (fliers on probes, before roadmap decision 2):

Non-goals: flying 3D navigation (fliers steer with probes), flow fields (RTS tool), a lane graph for city crowds (UE
ZoneGraph; later if measured), navmesh authoring tools (data comes from the bake; overrides are mods).

Owner decisions rows:

| Runtime tile patches for doors and props are fine; they are not "baking during play" | "Runtime patches" |
| Delegated 2026-11-08: two agent classes; doors by poly flags, no prop carving by default; client UI routes | "Data model", "Runtime patches", "Co-op" |

Canonical technique, Data model, Interfaces lines:

tiled and baked offline, with **TileCache** layers for runtime obstacles, **off-mesh connections** for links the
- **Runtime obstacles** (not saved, NV10, opt-in): `NavObstacle { id, ws, shape: Cylinder | Box }`, a TileCache carve.
`move_along_surface`, `corridor_corners`, `repair_corridor`, NV8). Still to add: local boundary (`dtLocalBoundary`, NV7b), `set_obstacle` / `clear_obstacle` /
`rebuild_dirty` (TileCache, NV10, opt-in), and bake-side `read_tile_blob` (NV4a: a blob's polygons with edge links:

Streaming line:

- Requests ride the streaming scheduler's per-frame budgets (streaming.md "Scheduler and budgets"); a tile is a bake
store read and `add_tile`, no generation.

"Runtime patches (not a bake)" section (TileCache carving design, before roadmap decision 7):

## Runtime patches (not a bake)

The baked tiles are the base layer; the host patches them at runtime (owner decision 2), cheapest mechanism first
(decisions (b)):
- **Doors: polygon state, no rebuild** (Recast demo's door flags, OpenMW's door area). A shut door sets its
  `DoorPolys` to area `door_closed` (cost, still pathable: the NPC opens it); a locked door adds flag `LOCKED`, which
  every filter excludes unless the query's per-agent predicate admits it (keys, scripted rights). Filters are per
  query, so the mesh is shared. Applied at a tick boundary.
- **Large movable props: no carving by default.** Vanilla has none; small items and bodies are left to ORCA and the
  mover (Unity: moving obstacles are avoided, not planned around). NV10 adds TileCache carving **opt-in per record**
  (`carves_nav`, adapter, default off) for a body at rest above a size threshold ("carve only stationary"), layers
  baked only for tiles that can host one. Rebuilds: worker, per-tick budget, coalesced per tile, ≥ 250 ms apart per
  tile (OpenMW), applied at a tick boundary in tile-key order.
- Door state and obstacles are derived (door overlay, prop transforms): not saved, recomputed on load.

Budgets row:

| Tile rebuilds (worker) | ≤ 0.10 ms | ≤ 0.20 ms |

Industry comparison paragraph:

**Match** UE, Unity, Godot and OpenMW on Recast generation, agent classes as data, area costs and filters, runtime
obstacles (door poly state, carving opt-in), off-mesh links, a coarse graph above the navmesh, Detour corridor
following, RVO-family avoidance (ours ORCA, deterministic) and debug display. **Differ** from OpenMW on when it is
built (baked per tile, mod-aware; nothing bakes during play); **ahead** of OpenMW across load doors (`WorldGraph`,
fade links only unseen; Bethesda's out-of-view teleport). Full table and sources:

Build plan rows and order line:

| Slice | Deliverable | Test |
|---|---|---|
| **NV5** streaming | nav ring residency per gameplay anchor and open portal, per class on demand (needs seamless SI3) | residency set exact for scripted anchor paths; `large` tiles absent with no large actor; unload hysteresis |
| **NV9** links + areas | off-mesh links (jump, drop, ladder, pathgrid) and area costs per class | path takes the cheaper area; link traversal events in order |
| **NV10** runtime patches | door poly state (area/flags); opt-in TileCache carving (`carves_nav`, at rest), rebuild budget | shut door raises cost with zero rebuilds, locked blocks except for an admitted agent; carve rebuilds per tick ≤ budget |

Order: NV0 → NV1 → NV2 → NV3 → NV4a → NV4b → NV4c → NV6 → NV7a → NV8 (→ AI6) → NV5 → NV7b (DECISIONS 2026-11-17: NV5 and seamless SI3 no longer gate AI6) → NV9 → NV10 → NV11 → NV12; NVM
after NV8. NV11 replaces seamless SI13 (M9b); NV7a lands before AI6.

Built build-plan rows (moved 2026-11-26):

| Slice | Deliverable | Test |
|---|---|---|
| **NV4b** Morrowind bake (**done 2026-11-11**) | adapter `NavGeometrySource` (cooked collision, terrain, water, doors), `bevyrrowind bake` wiring | cold bake time and MiB per class on the first pack (the "Bake" gate) |
| **NV4c** game wiring (**built 2026-11-17**, player-ring scaffold until NV5) | `sf_nav_bevy` in the game: `NavPlugin`, baked tiles into `NavMeshes`, draw frame from the floating origin, BRP `sf_nav/stats` | owner check NV-1 (tiles drawn over Seyda Neen) |
| **NV6** query service (**built 2026-11-17**) | worker queue, slicing, tick-boundary results, partial results at the ring edge | ordering and budget exact; sync mode equals async results |
| **NV7a** fixed tick (**built 2026-11-17**) | NPC drive (path follower, ORCA, mover) in `FixedUpdate` + render interpolation | headless: walker positions identical at 30 and 144 fps for the same ticks |
| **NV8** corridor (**built 2026-11-18**) | `PathCorridor` feeding `PathFollower`; repair after a push | headless walk around a pillar and through a door frame, nothing stuck |

Interfaces, the bake's adapter seam (NV4a):

```rust
/// The adapter's geometry for the bake: the same cooked collision the mover uses. Reads go through `cx`.
pub trait NavGeometrySource: Send + Sync {
    fn regions(&self, ws: &WorldspaceId) -> Vec<NavRegion>;            // plan time: origin + XZ extent per region
    fn tile_input(&self, cx: &mut RunCx, ws: &WorldspaceId, class: &AgentClass, bounds: &NavTileBounds)
        -> Result<NavTileSource, BakeError>;                            // TileInput + DoorVolume { door, volume }
    fn links(&self, cx: &mut RunCx, ws: &WorldspaceId) -> Result<NavLinks, BakeError>; // links + door approaches
}
```

Other as-built or stale lines (Reuse NV0 numbers, `LinkId` plan, Debug draw NV3, NV11 row):

  `NavBackend` in `sf_nav_recast`. NV0 baked all our cooked collision with no failures; full bake ≈ 55 s at 16
  threads, over the 45 s slot (NV4 tunes). [Spike report](../../research/navmesh-spike-nv0-2026-11-11.md). Fallback
  (not needed): recastnavigation through a vendored `cc` build.

`LinkId` does not exist yet (`DoorLinks` is a `Vec<DoorLink>`): `sf_world` gains a stable `LinkId` (hash of both
ends' worldspace and frame) in NV11, since the AI saves routes by link.

Automated checks use log lines only (no visual judgement by agents). Built in NV3: `tiles`, `paths [actor]`, `off`,
`nav.stats`, BRP `sf_nav/stats`; the other layers print the slice that builds their data, counters not yet kept
print `n/a`.

| **NV11** worldspace graph (split a-d 2026-11-26, owner unparked: built headless before portals) | `LinkId` (a, done), `WorldGraph` (b), `portal_query` (c), Bevy resource built from `DoorLinks` (d1), hearing propagation through it (d2 = AI8c3) (supersedes seamless SI13) | exact routes on a synthetic 3-space graph; LOS through an open seamless opening, none through fade links |

Canonical technique, last line; Budgets residency row:

avoidance, and take only `PathCorridor` and `LocalBoundary` from Detour.

| Tile residency | ≤ 64 MiB | ≤ 128 MiB |

## Moved from nav.md 2026-11-26 (NV0c)

Replaced by nav.md "Links" (owner decisions row, Data model, Doors):

| Owner 2026-11-11: NPCs climb, mantle and vault with the player's own moves (guards can follow onto roofs); later, after NV9: bake-time auto links from ledges in the mover's mantle/vault/drop envelopes, per class (UE NavLink generation, Unity auto off-mesh links) | NV9b (planned) |
{ ws, class, region, x, z }`, `OffMeshLink { id: NavLinkId, a, b, radius, two_way, kind: TraverseKind, area, classes
}` (`TraverseKind::Door(LinkId)` with NV11), `NavPoint { loc, poly }`. Serde except the backend-opaque `PolyRef` /
- **Off-mesh links** (bake, per worldspace): pathgrid edges the mesh misses, authored jumps, drops, ladders, ledges
  (`sf_interaction` or adapter data), door approach links.
- **Seamless link** (portal-ready, seamless on): the traverse operator opens the door through activation rules, waits
  for `passable`, walks through the `Gate`, and closes a door it found shut; `InWorldspace` changes at the crossing.
