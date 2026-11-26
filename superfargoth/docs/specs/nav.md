# Navigation: navmesh tiles, links, the worldspace graph (`sf_nav`)

Status: **draft (2026-11-08); NV0 done, NV1 + NV2 built (2026-11-11)**: `sf_nav` core with the `GridBackend` double;
`sf_nav_recast` (`navmesh-rs` backend). Scope label M7 (RPG core: AI). Slices NV0-NV12 below. The former open
questions are decided (2026-11-08, owner-delegated): reasoning in `../research/ai-nav-decisions-2026-11-08.md` (a-c).
Crates: `sf_nav` (Bevy-free core: types, `NavBackend` trait; planned: worldspace graph, `portal_query`, query service
logic), `sf_nav_recast` (Bevy-free backend over vendored `navmesh-rs`), planned `sf_nav_bevy` (thin: tile residency,
worker tasks, debug draw, console). Game side: `mw_ai` + `mw_bake`
([ai-adapter](../../../bevyrrowind/docs/specs/ai-adapter.md) "Navigation"). As-built history:
[archive/nav-history.md](archive/nav-history.md).
Sources: `../research/npc-ai-2026-11-08.md` §4, §5.2-5.3, §7.4 (approved); `../research/seamless-interiors-2026-11-08.md`
§4.2, §4.8; [ai.md](ai.md) (the main consumer); [streaming.md](streaming.md) "Rings"; [bake.md](bake.md) "Data model";
[locomotion.md](locomotion.md) "Local avoidance"; [net.md](net.md) "Net-ready rules".

<!-- agent-summary -->
> **Agent summary (2026-11-26).** Draft; NV1-NV4c, NV6-NV8, NV11a-d1, NV12a built ("Build plan"). Recast/Detour
> navmesh per worldspace, ground class and nav region, baked once by `sf_bake` (`nav.tiles`); two ground classes
> (humanoid, large). Fliers use a clearance height field, swimmers a water-body volume ("Volumes", NV13). Doorways are
> baked door polygons switched by area/flags; a tile whose walkable ground changed, or a missing one, is rebuilt by the
> bake's own step on a worker, off the frame, cached ("Runtime rebuild", NV10). Tiles stream with a fixed nav ring
> around gameplay anchors. Routing: worldspace graph (door links) → per-space route graph (NV12) → Detour over resident
> tiles → corridor → our ORCA (+ wall lines), on the fixed tick (NV7a), watched by the stuck monitor (NV14).
> Read next: "Data model", "Routing", "Volumes", "Build plan" (order = `docs/roadmaps/nav.md`).
<!-- /agent-summary -->

## Goal

Every non-player mover (NPCs, creatures, followers, escorts) gets paths that respect the same collision the mover
collides with, inside one worldspace and across door links, whether the door is seamless or fade-only. The same
graphs serve quest markers and the compass. Host-only for gameplay; clients route locally for UI only ("Co-op").
Fliers and swimmers are in scope ("Volumes", roadmap decision 2). Non-goals: flow fields (RTS tool), a lane graph for
city crowds (UE ZoneGraph; later if measured), navmesh authoring tools (data comes from the bake; overrides are mods).

## Owner decisions this spec implements (2026-11-08)

| Decision | Here |
|---|---|
| Layering and the split `sf_nav` + `sf_ai` (+ planned `sf_interaction`) | this crate set; ai.md |
| Runtime tile patches for doors and props are fine; they are not "baking during play" | "Runtime rebuild" |
| Spike `navmesh-rs` first; C++ recastnavigation FFI is the accepted fallback | NV0, "Backend" |
| Seamless interiors optional per door / per pack: AI works with seamless and fade-only doors | "Doors" |
| No multiplayer ceiling; gameplay never depends on graphics settings | "Streaming" (fixed nav ring), "Co-op" |
| Steam Deck is the floor | "Budgets" |
| Delegated 2026-11-08: two agent classes; doors by poly flags, no prop carving by default; client UI routes | "Data model", "Runtime rebuild", "Co-op" |
| Owner 2026-11-11: NPCs climb, mantle and vault with the player's own moves (guards can follow onto roofs); later, after NV9: bake-time auto links from ledges in the mover's mantle/vault/drop envelopes, per class (UE NavLink generation, Unity auto off-mesh links) | "Links" (NV9b) |
| Owner 2026-11-26 (nav roadmap §7): fliers and swimmers in scope; tiles built in play only when missing or changed, off the frame, cached; NPCs swim after you, non-swimmers wait | "Volumes", "Runtime rebuild" |

## Canonical technique

**Recast voxel navmesh generation + Detour tiled navmesh queries** (Mononen; used by UE, Unity, Godot, OpenMW),
tiled and baked offline, a tile rebuilt on a worker when its inputs change (UE dynamic regeneration), **off-mesh connections** for links the
mesh can't express, a **road/route graph** above the navmesh ("Route graph"; HPA*, Botea et al. 2004, only on a measured need), and **path corridor +
reciprocal velocity obstacles** for following and local avoidance (Detour `dtPathCorridor`; ORCA, van den Berg 2011).
Deviation: we keep our own ORCA (`sf_controller::avoid`, deterministic, tested) instead of DetourCrowd's sampled
avoidance, and take only `PathCorridor` and `LocalBoundary` from Detour. Fliers: a clearance height field searched in
2D with a vertical profile (Horizon Zero Dawn); swimmers: water-body volumes ("Volumes").

## Reuse (Bevy, then crates, then our own)

- **Bevy 0.20** ships no navigation.
- **`navmesh-rs` 0.2.2** (MIT OR Apache-2.0 + zlib, pure Rust Recast + Detour + DetourCrowd + TileCache, bit-exact
  with upstream `9f4ce644`; one author): **adopted (NV0, 2026-11-11)**, pinned `=0.2.2`, features `libm`
  (cross-platform determinism) + `polyref64`, vendored with one own patch (`superfargoth/vendor/README.md`), behind
  `NavBackend` in `sf_nav_recast`. [Spike report](../research/navmesh-spike-nv0-2026-11-11.md).
- Rejected (research §5.2): `rerecast` (no tiles; re-check at "update all"), `landmass` (glam 0.30 + `dodgy_2d`, no
  dynamic obstacles; its islands idea becomes nav regions), `vleue_navigator`/`polyanya` (2D/layered),
  `oxidized_navigation` (archived), `bevy_northstar`/flow-field crates (grids). `petgraph` covers the graphs.
- **Route graph (NV12, checked 2026-11-26):** Bevy 0.20 has no graph search (no A*/Dijkstra in any `bevy_*` 0.20 crate;
  `bevy_math` curves are Bevy-side, `sf_nav` is Bevy-free). `petgraph` 0.8.3 (MIT OR Apache-2.0, maintained, in the
  workspace) and `pathfinding` (MIT OR Apache-2.0; `Ord` costs, so f64 needs a wrapper) were rejected for this query:
  both fix their own tie-break, neither seeds a search mid-edge or ends at a virtual goal without mutating the graph.
  Own ~100-line A* in the NV11b pattern (`world_graph.rs` `Open`: `total_cmp` + index tie-break), CSR-style arrays.
- **Volumes, rebuild, stuck (NV0b, 2026-11-26):** Bevy 0.20 ships no navigation; `navmesh-rs` stays (its tilecache and
  crowd modules are references); no crate does 3D flight volumes (`rerecast` 1.0, `landmass` 0.9, `vleue_navigator`
  0.17, `bevy_northstar` 0.8; `oxidized_navigation` stale): field, swim band, rebuild queue, stuck monitor are ours.
- **Links (NV0c):** `navmesh-rs` has Detour's off-mesh connections (radius, flags, area, direction, u32 user id;
  `PathCorridor::move_over_offmesh_connection`); `sf_nav_recast` passes none yet; link generation is ours.
- Our own: the thin glue (types, trait, graphs, query service, streaming, debug draw); no Recast/Detour rewrite.

## Data model

Built in NV1 (`sf_nav::types`, the code is the reference): `AgentClass { id, radius, height, max_climb, max_slope_deg,
can_swim }` (adapter record `sf.nav.agent`), `AreaDef { id, name, cost: [f32; MAX_CLASSES], flags }` in the
adapter's `AreaTable` (`sf.nav.area`), `PolyFlags` (WALK, SWIM, DOOR, JUMP, AVOID, DISABLED + adapter bits), `TileKey
{ ws, class, region, x, z }`, `OffMeshLink` ("Links"), `NavPoint { loc, poly }`. Serde except the backend-opaque `PolyRef` /
`TileRef`, which never leave memory.

- **Agent classes**: content data, one navmesh per (worldspace, class); Morrowind bakes **two**, `humanoid` (small
  creatures too) and `large` (adapter). Voxel cost scales with `1 / cs²` and `cs ≈ radius / 2`, so a large class costs
  ~10 % of the humanoid bake and a small one ~4×: no `small` class (decisions (a)). A class's tiles are resident only
  around anchors whose simulated set holds an actor of that class (humanoid always). NV4 prints time and MiB per class.
- **Area types**: per-polygon ids from the bake (ground, water, door, path, hazard) or set at runtime (closed door),
  a cost per agent class, flags for filters (swimmers, avoid-hazard). The adapter's table names them, not the framework.
- **Nav regions**: meshes split into regions of at most `region_size` (16 km: f32 precision, poly-ref bits), each a
  Detour mesh in local coordinates with a `WorldPos` origin (landmass's islands); every Morrowind worldspace is one
  region (NV0 verifies). Region borders are coarse nodes; corridors hand over there. NV2: one Detour mesh per
  (worldspace, class, region), origin set with `RecastBackend::set_region_origin`.
- **Tiles** (bake product): one blob per `TileKey`, backend format, LZ4. NV0 calibrates: humanoid cs 0.15 m, ch 0.1 m,
  128 voxels per side (19.2 m); `large` cs ≈ 0.5 m (`cs ≈ radius / 2`).
- **Off-mesh links**: kinds, sources, traversal and auto generation in "Links".
- **Route graph** (bake, per worldspace, class mask per edge; replaces the HPA* "coarse graph", NV12): junctions,
  anchors and door approach nodes, polyline edges with walk-metre costs. ≤ 1.5 MiB Vvardenfell; always resident.
  "Route graph".
- **Worldspace graph** (`WorldGraph`, built at load from `DoorLink`s + the coarse graphs): nodes are worldspaces,
  edges are link ends with an approach point on each side, walking cost through, and flags (`Seamless`, `FadeOnly`,
  `OneWay`); dynamic door state (open, locked) comes from the host each tick.
- **Door polygons** (bake product): the bake drops door leaves from the geometry and marks each doorway footprint with
  a convex area volume (`door`), so every doorway has its own polygons, listed per tile as `DoorPolys { door, polys }`.
- **Runtime changes** (not saved): dirty and missing tiles rebuilt by the bake step ("Runtime rebuild", NV10b-c); a
  `NavObstacle` carve only if measured needed. **Height field and water bodies** (fliers, swimmers): "Volumes".

## Interfaces

`NavBackend` (`sf_nav::backend`, built NV1; the code is the reference): `build_tile(input, cfg, areas)` (bake only),
`add_tile` / `remove_tile` / `tile`, `nearest`, sliced `find_path(&self, &mut PathJob, &mut SliceBudget)`, `raycast`,
`poly_state` / `set_poly_state` (doors: `setPolyArea`/`setPolyFlags`, no rebuild), and read-only debug accessors
(NV3) `tile_summaries` (key, bytes, polygons per resident tile) and `tile_geometry` (a tile's polygons with wall
edges, region-local, built on request), and corridor moves (`is_valid_poly`, `closest_point_on_poly`,
`move_along_surface`, `corridor_corners`, `repair_corridor`, NV8). Still to add: local boundary (`dtLocalBoundary`, NV7b), tile
swap for rebuilds (`add_tile` over a resident key, NV10b; no TileCache API), and bake-side `read_tile_blob` (NV4a: a blob's polygons with edge links:
wall, in-tile polygon, or portal to a tile side). The bake's adapter seam (`sf_nav_bake`, built NV4a):

Methods `regions`, `tile_input` (`TileInput` + `DoorVolume`s), `links`, + `water_bodies` (NV13c); the code is the reference.
NV9 adds the class envelope hook and the link events ("Links").

A **test double** (`GridBackend`: a walkable-cell grid with exact A*) lives in `sf_nav` so the core, `sf_ai` and the
adapter test without Recast. Queries the AI calls (all take the actor's worldspace; there is no global
`ActiveWorldspace`, seamless SI3):

| Query | Returns | Where it runs |
|---|---|---|
| `request_path(PathQuery) -> RequestId` | `PathResult { status, waypoints: Vec<NavWaypoint>, links: Vec<NavLinkId \| LinkId>, cost }` at a later tick | worker, sliced |
| `nearest(Location, class)` | `NavPoint` | inline (µs) |
| `raycast(NavPoint, target)` | hit / clear (shortcuts, flee checks) | inline |
| `journey(Location, Location, rules) -> Journey` | door hops + route-graph walks + travel time; no tiles ("Route graph") | inline (graph) |
| `portal_query::map / path_distance / line_of_sight` | below | inline |

`Goal` covers `Point(Location)`, `Near { at, radius }` (follow, escort), `Away { from, min_dist }` (flee: search for
the cheapest polygon past the distance, Detour-style), and `AnyOf(Vec<Location>)` (nearest of several slots).

### The shared through-door query (`sf_nav::portal_query`)

One Bevy-free API for everything that crosses spaces: AI perception, audio, crime witnesses, net interest, quest
markers (seamless §4.2; it replaces OpenMW's scattered `positionFor` calls).

- `map(loc, link) -> Location`: through the link's rigid transform (`DoorLink::cross`).
- `path_distance(a, b, max_hops) -> Option<(f32, SmallVec<LinkId>)>`: same space = straight distance; else the
  cheapest `WorldGraph` route of straight legs through each opening's centre (hearing, OpenMW `heard`); `Cost::Walk`
  uses coarse-graph costs (AI timing).
- `line_of_sight(a, b, los) -> Los`: same space = one ray; across a link only if **seamless and open** (`openness >
  0`): ray A → opening, then mapped point → B. Fade-only or shut links block sight; hearing through them takes the
  adapter's `door_attenuation`. Rays go through a `LosProvider` (batched in `sf_physics`): no Rapier in `sf_nav`.

## Bake (`sf_bake`, one-time, mod-aware)

- Step **`nav.tiles`**, subject `(worldspace, class, tile)`: input = the adapter's `NavGeometrySource` for the tile's
  bounds plus a border (Recast tile border), whose reads (cooked collision products, terrain heights, water level)
  are recorded as dynamic dependencies. A mod that moves one wall changes only the tiles whose inputs it touched.
- Step **`nav.links`** per worldspace (off-mesh links, door approach points) and **`nav.routes`** per worldspace (the
  route graph from the adapter's `RouteSource`; replaces the `nav.coarse` HPA* stub, "Route graph").
- Parallel per tile (`sf_bake` execution); empty tiles (open sea, void) produce no object of their own (zero bytes:
  every empty tile shares the store's one empty object).
- **Where** (NV4a): `sf_nav_bake`, a Bevy-free crate on `sf_bake` + `sf_nav`, backend-agnostic (the host passes a
  `NavBackend`). Not in `sf_nav` (the runtime core must not link the bake) nor `sf_nav_recast` (steps would name a
  backend); product types (`BakedTile`, `NavLinks`, `CoarseGraph`) live in `sf_nav::baked` so the runtime reads them
  without the bake. Re-bake granularity is the adapter's read granularity: a moved wall re-bakes the tiles whose
  reads (e.g. a cell's placement list) changed. Finer early cutoff (a stored per-tile input product) was rejected: it
  would store the triangle soup, ~3× the navmesh on disk.
- **Gate**: a cold bake of every navmesh for the first pack, both classes, stays within the "few minutes" rule on the
  High reference; measured in NV4; over it, coarsen exterior humanoid tiles (`large` is cheap).

## Runtime rebuild (not a bake; NV10, roadmap decision 7)

The baked tiles are the base layer; the host changes them at runtime, cheapest mechanism first, **always off the frame**
(owner 2026-11-26; CONTRIBUTING "Design constraints": never a stall while playing):
- **Doors: polygon state, no rebuild** (NV10a; Recast demo's door flags, OpenMW's door area). A shut door sets its
  `DoorPolys` to area `door_closed` (cost, still pathable: the NPC opens it); a locked door adds flag `LOCKED`, which
  every filter excludes unless the query's per-agent predicate admits it (keys, scripted rights). Filters are per
  query, so the mesh is shared. Applied at a tick boundary.
- **Changed walkable inputs: rebuild the tile** (NV10b; UE dynamic regeneration). A cause event `NavDirty { ws, aabb }`
  (NV15: `Destroyed` on walkable geometry; any adapter emitter) marks every ground and field tile whose bounds + border
  it touches. A worker re-runs the same pure `nav.tiles` step over `NavGeometrySource` on the live world (bake inputs +
  save overlay): the result equals an offline bake of that state bit for bit, one code path, no layer format.
  **Coalesced**: one job per `TileKey`; a mark during a job bumps the generation and requeues once; ≥
  `ai.nav.rebuild.min_interval_ms` apart per tile (OpenMW). **Budget**: `ai.nav.rebuild.workers`, nearest an anchor
  first; a tile ≈ 3.75 ms single-thread (215.8 s / 57,516 humanoid tiles, cold-bake mean; NV-R1 measures), so a broken
  wall's ~4 tiles take ~15 ms on one worker. The old tile serves until the **swap**: current-generation results apply
  at the next fixed-tick boundary, ≤ `ai.nav.rebuild.max_swaps` per tick, in `TileKey` order (decision 15); then the
  door overlay re-applies, corridors in the tile repair from the agent's position (as NV8 `repair_start`; else a new
  request, the failed-replan gate re-arms on a tile change), and sliced jobs that visited it restart.
- **Missing tiles** (NV10c; owner Q1; modded or generated area, a bake older than a mod): the same worker path, at a
  loading screen for the arrival ring, else requested `ai.nav.rebuild.prefetch_m` past the ring's load radius; still
  missing when needed = `Partial` at its edge, never a main-thread wait. **Disk cache**: key = blake3 over (step
  version, backend format, class params, the tile's input hashes, overlay entries in its bounds), in the bake store's
  runtime namespace: a second load hits, any input change misses (LRU cap `ai.nav.rebuild.cache_mib`). A save's
  changed tiles re-derive the same way on load.
- **Carving** (TileCache, `carves_nav`): only if NVM measures props blocking what ORCA and the stuck monitor can't
  absorb (old design: [archive](archive/nav-history.md#moved-from-navmd-2026-11-26-nv0b)).
- **Net, saves**: host-only, derived: only the cause event replicates; clients keep baked tiles for UI routes; nothing
  saved. **Tunables** (Guess): `ai.nav.rebuild.workers` 1 (1-4), `ai.nav.rebuild.max_swaps` 4 per tick (1-32),
  `ai.nav.rebuild.min_interval_ms` 250 ms (0-5000, OpenMW), `ai.nav.rebuild.prefetch_m` 64 m (0-512),
  `ai.nav.rebuild.cache_mib` 256 MiB (16-4096). **Tests**: NV10a door cases, zero rebuilds; NV10b removed wall box =
  offline bake bytes, three marks = one rebuild, swaps in key order, a corridor through it repairs; NV10c generated =
  baked, second load a cache hit, main thread does only the swap.

## Streaming

- **Nav ring**: tiles are resident around every `Gameplay` anchor (players, and on a host every remote player) and
  around the mapped anchor of every open portal (the simulated set, seamless §4.2). Default load at R0 load + 64 m
  (256 m), unload at R0 unload + 64 m (320 m). **Fixed per game, never scaled by the graphics preset** (vision
  §4.2.4): what an NPC can path must not depend on a client's settings, and a headless server uses the same ring.
  R0 itself is fixed too (only R1-R3 scale; streaming.md fix in the decisions note). Per class: see "Agent classes".
- Requests ride the streaming scheduler's per-frame budgets (streaming.md "Scheduler and budgets"); a tile is a bake
  store read and `add_tile`; a missing one is built off the frame ("Runtime rebuild", NV10c).
- Route graphs and the `WorldGraph` are resident for every worldspace (small), so tier-1/0 actors and long routes
  never need tiles outside the ring.
- Memory: ≤ 64 MiB of tiles on the Floor (NVM calibrates; OpenMW's 256 MiB cache is a ceiling, not a target).

## Routing

Three levels, each only as detailed as the actor's tier needs (ai.md "Tiers"):
1. **Worldspace graph**: which door links to take (A* on `WorldGraph`, cost = walking time, filtered by door state
   and the agent's keys or rules).
2. **Route graph** inside each space ("Route graph"; one journey A* with level 1, NV12e): roads, paths and door
   approach nodes; tier 0 stops here and advances along it by time.
3. **Detour** over resident tiles from the agent to the next route point or the goal; sliced on the worker.

Then **corridor** (`dtPathCorridor`: string pulling, `movePosition`, optimisation, repair when pushed off) → steer
target → **ORCA** with navmesh wall segments → `LocomotionIntent` → mover.

**Query service** (core logic, worker execution in `sf_nav_bevy`): requests `(RequestId = (ActorKey, seq), class,
from, goal, filter, priority, tick)` in a priority queue (combat and player-visible first) feed sliced jobs (node
budget per slice, time budget per tick); results apply at the next tick boundary in `RequestId` order. Tiles not
resident: `Partial` to the ring edge plus the route-graph journey. Tests use a synchronous inline mode.

### Stuck monitor (NV14; roadmap decision 10)

On the fixed tick after the corridor (logic `sf_nav::stuck`, system in `sf_nav_bevy`), per actor that wants to move
(intent speed > 0; not in a link traversal, a door queue or a deliberate wait): progress = drop in remaining corridor
distance over `ai.nav.stuck.window_s`; under `ai.nav.stuck.min_progress_m` it escalates one step per window, in order:
**repath** (new request from where it stands, past the failed-replan gate once) → **corridor repair** (NV8
`repair_start`) → **snap** to the nearest poly (≤ `ai.nav.stuck.snap_m`, ahead on the corridor) only when no viewer
sees it (the fade-only door's watch test, AI12; seen = it waits up to `ai.nav.stuck.seen_wait_s`) → **fail** to the
planner: `NavEvent::Stuck { actor, reason: NoProgress | RepathFailed(PathStatus) | NoPoly | Seen }`. Progress resets
the ladder. Fliers and swimmers take the same steps on 3D progress (snap = nearest field or band cell). Counters
`nav.stuck/min` and per step in `nav.stats` and BRP feed NVM; windows count ticks (deterministic, host-only).
Industry: UE `UPathFollowingComponent` block detection (`BlockDetectionDistance` over `BlockDetectionSampleCount`
samples → `Blocked`), universal. **Tunables** (Guess): `ai.nav.stuck.window_s` 1.5 s (0.25-10),
`ai.nav.stuck.min_progress_m` 0.5 m (0.05-5), `ai.nav.stuck.snap_m` 2 m (0.25-10), `ai.nav.stuck.seen_wait_s` 10 s
(1-120). **Tests (NV14, headless)**: an agent pinned by a collider the mesh lacks emits repath, repair, snap, fail on
exact ticks; a seen agent never snaps and fails `Seen`; a walking agent and one in a door queue never trip.

## Route graph (`sf_nav::route_graph`, NV12; spec NV0b1 2026-11-26)

The coarse level for long walks and every tier-0 actor: a road/route graph per worldspace (nav roadmap decision 3:
RDR2/GTA path nodes, UE ZoneGraph lanes; no shipped open world plans whole-world on navmesh). Tier 0 (ai.md AI10)
stores a route and progress, never a position; promotion and `fast_forward` ask `position_at`. Class-agnostic geometry
with a class mask; costs in **walk metres** (`cost_m`), time = `cost_m / speed_mps` of the traveller (Detour's cost
model: length × area cost; per-class or per-actor speed applies at query time, nothing re-bakes).

**Model.** `RouteGraph { worldspace, nodes, edges, points: Vec<Vec3>, cum_m: Vec<f32> }` (+ a load-built bucket index,
`#[serde(skip)]`). `RouteNode { id: RouteNodeId, kind, pos: WorldPos }`, kind `Junction | End | Anchor(String)` (adapter
settlement/landmark tag) `| Door { link: LinkId, side: LinkSide }`. `RouteEdge { id: RouteEdgeId, a: u32, b: u32, kind:
EdgeKind (Road | Path | Street | Connector), two_way, classes: ClassMask, length_m: f32, cost_m: f32, points: Range<u32> }`;
the polyline runs a → b as offsets from node `a` (f32; edges longer than `MAX_EDGE_M` = 4096 m are split, precision
bound ≤ 0.5 mm), `cum_m` holds cumulative lengths per point for O(log k) lookup. Ids: blake3 64-bit like NV11a,
`RouteNodeId` over tag `sf.route.node/1`, worldspace and key (`Door`: `LinkId` + side, so a retuned approach keeps its id;
else position quantised to 1 cm, equal keys merge), `RouteEdgeId` over `sf.route.edge/1`, both node ids, kind and the
1 cm-quantised points. Nodes and edges are sorted by id; dense indices follow that order.

**Builder** (`RouteGraphBuilder`, pure, the only way in): `add_node(pos, kind) -> RouteNodeId`, `add_edge(a, b, kind,
two_way, interior: &[WorldPos], cost_m: Option<f32>)` (`None` = polyline length), `add_door(link, side, pos)`,
`build(&RouteBuildRules) -> (RouteGraph, RouteBuildStats)`. Input order never matters (shuffled input = identical postcard
bytes). It merges equal keys, collapses degree-2 chains of one kind into one polyline (anchors and doors are kept; costs
add), simplifies polylines (Douglas-Peucker, `simplify_m`), clamps `cost_m` to at least `length_m` (the traveller walks
the polyline; keeps every partial edge ≥ its straight distance, so the heuristic stays consistent; counted), and joins each door to the nearest edge within
`door_connect_m` by splitting that edge at the projection plus a `Connector` edge (beyond: isolated, counted).

**Queries** (inline, no tiles):

```rust
pub struct TravelRules { pub speed_mps: f64, pub class: AgentClassId, pub kinds: EdgeKinds,
    pub join_max_m: f64, pub offroad_cost_mul: f64, pub max_expansions: u32 }
impl RouteGraph {
    pub fn route(&self, from: &Location, to: &Location, r: &TravelRules) -> Result<RoutePath, RouteError>;
    pub fn position_at(&self, path: &RoutePath, t_s: f64) -> RoutePose;
    pub fn nearest_point(&self, at: &Location, r: &TravelRules, max_m: f64) -> Option<RoutePoint>;
    pub fn anchor(&self, tag: &str) -> Option<RouteNodeId>;
}
pub struct RoutePath { pub worldspace: WorldspaceId, pub speed_mps: f64, pub kind: RouteKind /* Graph | Direct */,
    pub legs: Vec<RouteLeg>, pub cost_m: f64 }               // duration_s() = cost_m / speed_mps
pub enum RouteLeg { Straight { a: WorldPos, b: WorldPos, cost_m: f64 },
    Edge { edge: RouteEdgeId, from_m: f32, to_m: f32, a: WorldPos, cost_m: f64 } } // to_m < from_m = reverse
pub struct RoutePose { pub loc: Location, pub heading: Vec2 /* unit XZ */, pub leg: u16, pub done: bool }
pub struct RoutePoint { pub edge: RouteEdgeId, pub along_m: f32, pub pos: WorldPos, pub dist_m: f64 }
pub enum RouteError { WrongSpace, NoJoin(RouteEnd), Unreachable, Budget }
```

- **`route`**: each end joins at its `nearest_point` (≤ `join_max_m`, edge allowed for `class` and `kinds`) by a
  straight leg costing distance × `offroad_cost_mul` (≥ 1). A* seeds both ends of the start edge (one-way: forward only)
  and finishes through a virtual goal on the goal edge; same edge adds the along-edge candidate. Heuristic = straight
  metres to the goal point (admissible and consistent given the length clamp and mul ≥ 1); a partial edge costs
  `cost_m × piece / length_m`. Costs sum in f64 (edge costs
  stored f32), no transcendental maths (`sqrt` is IEEE exact; heading is a vector, not an angle). Ties: open set by
  `(f, node index)`, a parent changes only on a strictly lower `g`, adjacency sorted by `(neighbour, edge id)`. Direct
  (one straight leg × `offroad_cost_mul`, `kind: Direct`) is a candidate when the ends are ≤ `join_max_m` apart: it
  wins at cost ≤ the graph route or when an end has no join. Otherwise an end without a join is `NoJoin(From | To)`;
  `Unreachable`: different components; `Budget`: past `max_expansions`. Missing graph for a space: the caller walks
  straight (NV11b behaviour).
- **`position_at`**: legs by cumulative cost (binary search), uniform pacing inside a leg (measured `cost_m` ≠
  `length_m` slows the whole edge), point by `cum_m`; `t_s` clamps to `[0, duration]`, `done` at the end; heading = the
  current segment's XZ direction (previous segment's on a zero-length one).
- **`nearest_point`**: uniform XZ buckets (`grid_cell_m`), each segment listed in the buckets its AABB overlaps; ring
  search outward, stopping once the ring's inner distance exceeds the best; O((max_m / cell)² + segments seen). Ties:
  distance, then edge id, then `along_m` (result independent of bucket iteration order).

**Journey across spaces** (`sf_nav::journey`, NV12e): one A* over the union of route graphs, each `Door` node joined to
its link's far side by the `WorldGraph` traverse (NV11b times, door state and unlock rules); a space with no route graph
(interiors) is a straight clique of its exits plus start/goal. Heuristic: straight metres / speed in the goal's space, 0
elsewhere (NV11b rule). `Journey { legs: Vec<JourneyLeg> }`, `JourneyLeg::{ Walk(RoutePath), Door { link, side,
traverse_s } }`, cost in seconds (walk metres / `speed_mps` + traverse), owned by `sf_nav`; it is `journey(...)` of
"Interfaces" and supersedes `WorldRoute` for the AI.
Detour joins only inside the ring (Routing level 3).

**Bake.** Step **`nav.routes`** per worldspace replaces `nav.coarse`: input = the adapter's `RouteSource::route_input(cx,
ws) -> RouteInput` (generic nodes, edges, kinds, door ends; reads recorded as dynamic dependencies, so a mod that edits a
source re-bakes that space), output = postcard `RouteGraph` with `format: u16` (`ROUTE_GRAPH_FORMAT`, bump = rebuild).
Sources: generated worlds' `roads.json` (NV12b, `sf_worldgen` reader; field layout confirmed there), Morrowind exterior
pathgrid chains + door approaches (NV12c, `mw_bake`). NV12d measures each edge's Detour walk (reference class, area
costs) into `cost_m`, clears the bit of a class whose walk fails, and deletes the HPA* stub (`CoarseGraph`, `nav.coarse`).

**Tunables** (Guess unless noted; core `Default`s mirror them; runtime ones declared in `sf_nav_bevy`, bake ones in
`sf_nav_bake`): `ai.nav.route.join_max_m` 250 m (0-2000), `ai.nav.route.offroad_cost_mul` 1.5 ratio (1-5; off-trail
walking is slower, Naismith/Tobler), `ai.nav.route.max_expansions` 200000 count (1e3-1e6), `ai.nav.route.grid_cell_m`
64 m (8-1024), `ai.nav.route.simplify_m` 0.25 m (0-5, bake), `ai.nav.route.door_connect_m` 60 m (0-500, bake).
`MAX_EDGE_M` and the 1 cm quantum are formats (const).

**Budgets** (to measure, NV12d/NVM): Vvardenfell ≤ 10k nodes, 20k edges, 60k points (≤ 1.5 MiB; vanilla PGRDs measured 8.8k / 17.7k / 38k / 1.31 MiB, NV12c2; was a 5k / 6k Guess); 10x world ≤ 50k nodes,
60k edges, 600k points (≤ 12 MiB resident, index included). `route` ≤ 0.2 ms cross-island on Vvardenfell and ≤ 2 ms
worst on 10x (planned once per directive, worker-friendly); `nearest_point` ≤ 5 µs at 250 m; `position_at` ≤ 1 µs.

**Net and saves.** Host-only for gameplay; clients may run it for compass routes (identical bake, presentation only).
Pure functions of (graph, query), applied on the fixed tick. Tier 0 saves `RoutePath` (ids + leg start points) and
elapsed travel seconds, never a live position; on load an edge id missing from the graph (mod changed a source)
re-plans from the start of the leg reached.

**Tests (NV12a, all exact, synthetic graphs):** shortest route on a square-plus-diagonal (node sequence, `cost_m` to 1e-9);
equal-cost tie takes the lower node id, same answer from shuffled input; `Unreachable` across components; off-graph
start/goal joins (Straight + partial Edge legs, off-road mul applied; past `join_max_m` → `NoJoin`); `Direct` when
cheaper; same-edge forward and reverse, one-way refuses reverse; `position_at` at t ≤ 0, t = duration (`done`), mid
polyline (1e-5 m, unit heading) and uniform pacing when `cost_m` ≠ `length_m`; `nearest_point` projection, equidistant
tie → lower edge id, `None` past `max_m`, kind filter, equal to a linear scan on 1,000 seeded points; builder collapse
(anchors and doors kept), merge, length clamp count, door split + `Connector`; golden `RouteNodeId`, shuffled input =
identical postcard bytes; postcard + JSON round trip of `RouteGraph` and `RoutePath`; `Budget` at a tiny
`max_expansions`; `#[ignore = "measurement: ..."]` 50k-node grid route µs.

**Industry**: **Match** UE ZoneGraph (polyline lanes, lane location = lane + distance, nearest-lane query, tag filters
≈ `EdgeKinds`, Mass LOD moving off-screen agents along lanes; [ZoneGraph API](https://dev.epicgames.com/documentation/unreal-engine/API/Plugins/ZoneGraph/FZoneGraphLaneLocation)),
GTA V/RDR2 path-node networks for ambient travellers and GPS, Bethesda persistent actors advancing Travel packages
unloaded and appearing along the path ([CK Persistence](https://ck.uesp.net/wiki/Persistence_(Creation_Kit))), KCD2
LOD-unaware schedules (ai roadmap 13). **Differ** from HPA* (Botea 2004): authored/generated graph, not tile borders
(kept only on a measured need). **Ahead** of OpenMW/Morrowind, where actors outside the processing range freeze.

**As built (NV12a, 2026-11-26):** `sf_nav::route_graph` (`mod.rs` types/ids/index, `build.rs`, `query.rs`; 14 exact tests + ignored timing). Readings: `Junction`/`End` come from the
final degree (`add_node`'s kind is a hint); edge ids also hash `two_way`, two-way edges run from the lower node id; `add_edge` allows every class, unknown ends are dropped (counted); only
edge-less doors auto-join (the Connector takes the split edge's classes); `position_at` finds the leg by a linear prefix scan (`RoutePath` stores no prefix), heading falls back to -Z; A* stops
once `f ≥` the Direct cost; the index builds lazily (`set_grid_cell` rebinds it); `encode`/`decode` prefix `ROUTE_GRAPH_FORMAT`; the leg type is `route_graph::RouteLeg`. 224² grid (50k nodes):
route 3.8 ms avg, 15 ms corner to corner (grid ties under a straight-line heuristic expand ~every node; over the 2 ms budget, recheck on real roads, NVM), `nearest_point` 3.3 µs, `position_at` 0.33 µs.

**As built (NV12c1, 2026-11-26):** `sf_nav_bake::routes`: `RouteBake::new(source, rules)` (own bake, no backend) registers `nav.routes`; `RouteSource { plan(ws, PlanCx) (default none), route_input(RunCx, ws)
-> Result<Option<RouteInput>, BakeError> }`; `RouteInput { nodes: (pos, RouteNodeKind), edges: (a, b node indices, kind, two_way, interior, cost_m), doors: (LinkId, side, pos) }` (in the bake
crate, the builder stays the only way in); plan params = rules + format + the source's own; `None` = zero bytes, `decode_routes` reads it as `None` and errors on another format. Tunables
`ai.nav.route.simplify_m` / `door_connect_m` declared there (`Lifetime::Bake`, `TUNABLES`, `route_build_rules(&Tunables)`); new dep `sf_tunables` (already in the closure).

**As built (NV12c2, 2026-11-26):** Morrowind source `mw_bake::nav::routes` (ai-adapter.md "Route source"). Vvardenfell + Solstheim: 8794 nodes, 17712 edges, 37988 points, 1.31 MiB (nodes and edges
over the 5k / 6k Guess, now the "Budgets" line: town pathgrids are dense meshes, not trees), 220 components; vanilla has exterior PGRDs only in settlements (148 of 1559 cells), so towns don't connect (Balmora → Caldera
`Unreachable`) and inter-town edges come from NV12c3 (Detour walks on the navmesh between neighbouring components). Connected routes > 300 m: 0.13 ms avg, 0.67 ms worst (release).

## Volumes: fliers and swimmers (NV13; roadmap decision 2)

Fliers and swimmers never use the navmesh (ground classes swim on its `water` polys, decision 8). Both read one
class-free **height field** per worldspace, rasterised in the `nav.tiles` pass from the same `NavGeometrySource`
triangles (Horizon Zero Dawn, Game AI North 2017 "Putting the AI back into Air": a max-collision height field in 64 × 64
tiles with mips, chosen over volumetric representations for memory and query cost; NV-R1). HZD bakes one dilated field
per agent size; ours is class-free (`floor` + the class's clearance, dilation by mip, added at query): one field, all
classes, our own choice for memory.

- **Classes** (`sf.nav.agent`): `AgentClass` gains `medium: Ground | Air | Water` (default `Ground`; only ground classes
  bake Recast tiles) and, for `Air`, `clearance_m`, `max_climb_deg`, `max_dive_deg`, `max_altitude_m` (content). The
  adapter maps actors to classes and cost profiles (ai-adapter "Navigation": `Flies` → flier, `Swims` without `Walks` →
  swimmer, amphibious = ground with `water` ×1). **Levitation** = class swap to the adapter's flier class (new flight
  query); at its end, ground class at `nearest` within `ai.nav.levitate.land_search_m` (none: it falls, retries on landing).
- **`FieldTile`** (subject `Field { ws, x, z }`, LZ4, 64 × 64 cells of `ai.nav.field.cell_m`): `base_m: f32`; per cell
  in decimetres over it `floor: u16` (top of the solid under the flight gap: terrain + prop tops), `ceil: u16`
  (`max_height`, underside above the gap; `MAX` = open sky), `bed: u16` (solid top under the covering water surface;
  `MAX` = no data); mips (floor and bed = max of 4, ceil = min). Flight gap: above the topmost span under open sky
  (adapter flag: exteriors), else the tallest open gap. No actors. ≤ 32 KiB raw per 128 m tile, KBs in LZ4 (NV13d).
- **`WaterBody`** (`NavGeometrySource::water_bodies(cx, ws) -> Vec<WaterBodyInput>`, a recorded dependency): `{ id: u32,
  ws, surface_m: f32, extent: Aabb2 }`; the bed sampler is the field's `bed`; a cell under two bodies takes the higher
  surface. Morrowind: one body per space (ai-adapter); generated worlds: one per lake or river (NV9e). **No bed data**
  (open sea past the last cell, an empty tile) = deep, `bed = surface - ai.nav.water.default_depth_m`: shallow would
  strand swimmers at every map edge, nothing solid is there, and a finite depth bounds the band and dive steering.
- **Flight query** (NV13b; a job kind of the sliced query service, same `RequestId` order, `PathResult` with 3D
  waypoints): 8-connected A* at mip `k = ceil(log2(radius / cell_m))`, a cell's floor = max of its 3 × 3 (dilation ≥
  radius); cost = metres + `ai.nav.flight.climb_cost` × metres climbed; a gap under `height + 2 clearance` is a wall;
  coarse to fine like HZD: plan at mip `k + ai.nav.flight.coarse_mips`, then refine at mip `k` only inside that
  corridor (dilated one coarse cell), each search capped at `ai.nav.flight.max_iterations` (HZD: 1024; over it =
  `Partial` toward the best node, grid A* path symmetry makes uncapped searches blow up). **Landing**: the query also
  returns landing candidates (cells with `ceil - floor ≥ height` near the goal on walkable ground) for levitation end and
  a downed flier; a flier killed in the air falls on a guided arc to the nearest candidate instead of through props. **Vertical profile**: `floor + clearance` per
  waypoint, under `ceil - clearance` and `floor + max_altitude_m`, climb and dive angles limited by a forward and a
  backward pass; over water the floor is at least the surface. **Ray-clear shortcuts**: string-pull to the farthest
  waypoint whose segment (cell DDA) stays above floor + clearance.
- **Swim query** (NV13c): cells in the extent with `surface - bed ≥ height + ai.nav.water.margin_m` (deeper than the
  body), same A* and shortcuts; waypoints clamped into `[bed + r, surface - r]`; a goal out of the water = `Partial` at
  the nearest band cell. Field tiles stream on demand like `large`, within `ai.nav.field.ring_m`; `nav.draw field|water`.
- **Tunables** (Guess): `ai.nav.field.cell_m` 2 m (0.5-8, bake), `ai.nav.field.ring_m` 512 m (128-2048),
  `ai.nav.flight.climb_cost` 2 ratio (1-10), `ai.nav.flight.coarse_mips` 3 count (1-5; HZD plans at mip 3), `ai.nav.flight.max_iterations` 1024 count (128-8192; HZD), `ai.nav.water.default_depth_m`
  30 m (2-500), `ai.nav.water.margin_m` 0.5 m (0-5), `ai.nav.levitate.land_search_m` 4 m (0.5-20).
- **Tests (exact, synthetic)**: NV13a two box towers: floor/ceil per cell, mip maxima, bed kept under a bridge deck,
  KB per tile printed; NV13b never under floor + clearance, the gap over the summit when `climb_cost` says so, one
  segment on open ground, under a ceiling; NV13c exact basin cell set, a fish stays in the band, shore goal `Partial`,
  no bed = `default_depth_m`; NV13d (measurement) field bake s and MiB for Vvardenfell, query µs. Budget: path queries.

## Crowds and avoidance (`sf_controller::avoid`)

- ORCA stays ours (DECISIONS 2026-10-12 orca). Added: **static obstacle lines** from `LocalBoundary` (navmesh edges
  within ~2 m of the agent, RVO2's obstacle half-planes), the item DECISIONS left "waiting for navmesh paths".
  The batch must still equal the single-agent scan bit for bit.
- Avoidance runs among tier 1-2 movers in the same worldspace; the player stays `NonReactive`.
- Doorways and corridors: the existing keep-right and priority rules; door traversal is queued by the AI operator
  (one agent through a door at a time, a claim on the link's approach point).
- **Fixed tick**: today `mw_walkers_bevy::walkers::drive` steps path follower, ORCA and mover in `Update` (DECISIONS
  walkers); net rule 1 needs the fixed tick (physics.md "Controller", net.md N0). **NV7a** moves NPC drive into
  `FixedUpdate` beside the player's move, with render interpolation like the player's tick view; before AI6. On the
  Deck that is two ticks per 30 fps frame: tier 1 avoidance may run every other tick, staggered by `ActorKey`.

## Links (off-mesh links and traversal, NV9a-c; spec NV0c 2026-11-26)

Roadmap decisions 5, 6, 8, 15. `OffMeshLink { id: NavLinkId, a, b, radius, two_way, kind: TraverseKind, area, classes }`
(a Detour off-mesh connection) lives in the tile holding `a` as an input of its build, so a rebuild re-includes it;
state flips its polygon (`set_poly_state`, `DISABLED`) at a tick boundary. Ends stay under a tile apart. Kinds:
`Walk` (pathgrid edges the mesh misses, NV9d), `Drop` (one-way), `Jump`, `Vault`, `Mantle`, `Climb` (auto per class,
NV9b), `Ladder` (IN43 slot, NV9c), `Door(LinkId)` (`WorldGraph` hops only, never in a tile); `Swim` goes (decision 8).

**Traversal (NV9a).** Within `radius` of `a` the corridor steps over the link (`move_over_offmesh_connection`),
pauses, and nav emits `NavEvent::LinkReached { actor, link, kind, from, to }`. The operator (AI33; ladders
`UseSlot(ladder)`) owns the mover through `sf_action`, the player's own states; nav never moves the body. It answers
`NavCmd::LinkDone` (corridor resumes, NV8 `repair_start`) or `LinkFailed { reason: Timeout | Blocked | Envelope |
Disabled | Interrupted }`, timeout `ai.nav.link.timeout_s` + `ai.nav.link.timeout_per_m_s` × length. The first three
add `ai.nav.link.fail_cost_m` to that link for that actor only (a weak mover never bars a strong one), decaying over
`ai.nav.link.fail_decay_s`, via the per-query filter; `ai.nav.link.max_fails` in the decay excludes it; then repath.
A link disabled mid-traversal sends `NavEvent::LinkDisabled` (abort before the warp starts, else finish); a tile swap
waits for the release. **Stuck monitor**: paused from `LinkReached` to the answer, restarted empty. **Net**: host-only,
at tick boundaries in (`ActorKey`, `NavLinkId`) order; penalties derived, never saved or sent.

**Auto links (NV9b; UE Navigation Link Generation, Unity drop height / jump distance).** Per class `TraverseEnvelope {
max_vault, max_mantle, max_climb_up, max_drop, max_jump_gap }`, derived at bake from the class's mover profile by the
player's own code (`ContextParams::from_mover`: 0.7 / 1.5 / 2.5 h; `max_drop` = the highest fall landed without damage;
gap from the launch arc), no nav-only numbers; they enter the tile input hash. In `nav.tiles`, border edges in
(polygon, edge) order are sampled every `ai.nav.link.spacing_m`; each sample seeks clear walkable ground outward
(input gathered `max_jump_gap` past the border): lower by `max_climb`..`max_drop` = `Drop`, plus the reverse `Mantle` /
`Climb` within reach; a gap = `Jump`; a wall ≤ `max_vault` = `Vault`. Same kind and landing polygon within
`ai.nav.link.dedupe_m` = dropped; `ai.nav.link.max_per_tile` keeps the earliest. **Tests** (synthetic ledges): exact
links for each limit ± 1 cm per class, a blocked landing, a straight ledge, permuted input, a rebuilt tile.

**Interaction links (NV9c).** A ladder def (IN43, Bottom and Top entrances) becomes a two-way `Ladder` link between
the approach points, from the cooked defs the adapter places (`links()`); its `WorldChange` flips it like a door.

**Tunables** (Guess): `ai.nav.link.timeout_s` 4 s (0.5-30), `ai.nav.link.timeout_per_m_s` 0.5 s/m (0-5),
`ai.nav.link.fail_cost_m` 50 m (0-1000), `ai.nav.link.fail_decay_s` 60 s (5-600), `ai.nav.link.max_fails` 2 (1-10),
`ai.nav.link.spacing_m` 1 m (0.25-5), `ai.nav.link.dedupe_m` 0.5 m (0.1-3), `ai.nav.link.max_per_tile` 64 (0-1024).

## Doors

- Each link end has an **approach point** (adapter: nearest walkable point in front of the door, from pathgrid nodes
  in Morrowind) inside its worldspace's mesh; the `WorldGraph` edge joins the two.
- Seamless vs classic is a **host world rule** (`world.doors`, saved with the world; pack default, per-door data
  overrides), never a graphics option, because it changes sight, witnesses and crossings (seamless research §7 (i)).
- **Seamless link** (portal-ready, seamless on): kind `Door(LinkId)` ("Links"), operator AI12 `Traverse` (ai.md
  "Executors"; closes a door it found shut); `InWorldspace` changes at the crossing.
- **Fade-only link** (seamless off, trapdoors, one-way): the actor takes it only when no player can see either end
  (no viewer within the adapter's `door_watch_radius` with LOS to the end); otherwise it waits nearby or re-routes.
  This is Bethesda's out-of-view door teleport and works with seamless disabled for the whole pack.

## Debug draw (`sf_nav_bevy`, feature `debug`)

Bevy gizmos, off by default: `nav.draw tiles|links|doors|obstacles|paths <actor>|boundary <actor>|coarse|graph`.
`nav.stats` prints tiles resident per class, MiB, queue depth, ms per tick, rebuilds; BRP serves the same as JSON.
Automated checks use log lines only (no visual judgement by agents). Layers whose slice is not built print it.

## Co-op and saves (net-ready rules)

1: queries and obstacle updates apply on the fixed tick. 2: nothing reads input. 3: nav state is static data or
derived from replicated state. 4: a query's result depends only on (tiles, obstacles, query); worker timing changes
the tick it lands at most, never its content; host-only, so no replay is needed. 5: host-owned for gameplay. **Clients
compute compass and quest-marker routes locally as presentation** (decisions (c)): `WorldGraph` + route graphs from
the identical bake (content fingerprint), door state as replicated in their interest set, the baked default beyond;
a near-field breadcrumb may use tiles in the client's own ring. The host sends marker targets (`Location`), never
routes; a dedicated server computes none; results never feed gameplay. 6: anchors per player, no singleton. 7:
`Location`, `LinkId`, `NavLinkId`, never `PolyRef` or `Entity` on the wire or in saves. 8: n/a.
**Saves**: nothing. Tiles are bake products, obstacles re-derive from the overlay and door state, in-flight requests
are re-issued by the AI after load.

## Budgets (per 60 Hz tick; Floor = Steam Deck, High = RTX 4060 laptop; to calibrate)

| Item | Floor | High |
|---|---|---|
| Path queries (worker) | ≤ 0.30 ms | ≤ 0.50 ms |
| Corridor + ORCA, tier 1-2 (main) | ≤ 0.20 ms | ≤ 0.25 ms |
| Tile rebuild swaps (main; the rebuild itself runs on `ai.nav.rebuild.workers`) | ≤ 0.05 ms | ≤ 0.05 ms |
| Tile residency (height-field tiles included) | ≤ 64 MiB | ≤ 128 MiB |

These scale with the host's CPU tier (ai.md "Tiers and budget"), never with the graphics preset.

## Industry comparison

**Match** UE, Unity, Godot and OpenMW on Recast generation, agent classes as data, area costs and filters, runtime
changes (door poly state; dirty tiles rebuilt, as UE dynamic regeneration), off-mesh links, a coarse graph above the navmesh, Detour corridor
following, RVO-family avoidance (ours ORCA, deterministic) and debug display. **Differ** from OpenMW on when it is
built (baked per tile, mod-aware; in play only missing or changed tiles, off the frame, cached); **ahead** of OpenMW across load doors (`WorldGraph`,
fade links only unseen; Bethesda's out-of-view teleport). Full table and sources:
[archive](archive/nav-history.md#industry-comparison-table-moved-from-navmd-at-nv4a).

| Part (NV0b) | Industry | Ours | Verdict |
|---|---|---|---|
| Fliers | HZD height field + 2D A*; UE, Unity, OpenMW: no flight planner (UE plugins: octrees) | the HZD field | **match** HZD |
| Swimmers | OpenMW swim-flagged polys; UE/Unity surface navmesh | swim polys + water-body band | **match**, **ahead** on depth |
| Rebuild | UE dynamic generation + invokers; Unity async `UpdateNavMesh`; Detour TileCache; OpenMW async + disk cache | bake step per tile on a worker, swap, cache | **match** UE, OpenMW |
| Stuck | UE `UPathFollowingComponent` blocked detection; Bethesda unseen teleport | window → repath → repair → snap → fail | **match** |
| Links | UE Smart Links + link generation; Unity `NavMeshLink`, `autoTraverseOffMeshLink = false`; OpenMW (`navigatorimpl.cpp` `4e718b5`): door and pathgrid links only | handoff to the player's moves, auto links per envelope | **match**; **ahead** of OpenMW |

**Route graph (NV12):** coarse routing on a road/path graph with lane-location progress, as UE ZoneGraph + Mass LOD and
GTA V/RDR2 path nodes, not tile-border HPA*; tier 0 advances by time (Bethesda unloaded Travel packages, KCD2), ahead
of OpenMW, where actors outside the processing range freeze ("Route graph" Industry line).

## Build plan

One deliverable plus its test per slice; research and building are separate.

| Slice | Deliverable | Test |
|---|---|---|
| **NV0-NV4a** (**done 2026-11-11**) | spike, `sf_nav` core, `sf_nav_recast`, `sf_nav_bevy`, `sf_nav_bake` steps | rows with their tests: [archive](archive/nav-history.md#done-build-plan-rows-moved-from-navmd-at-nv4a) |
| **NV4b-NV8** (**built 2026-11-11..18**) | Morrowind bake, game wiring, query service, fixed tick, corridor | rows: [archive](archive/nav-history.md#moved-from-navmd-2026-11-26-nv0b) |
| **NV5a-b** streaming | a: ring per gameplay anchor, per class on demand, `NavRingScaffold` removed; b: open-portal anchors (SI3) | roadmap Stage 1 |
| **NV7b** ORCA walls | obstacle lines in `sf_controller::avoid` | agent never crosses a wall segment; batch = single bit for bit |
| **NV9a-e** links + areas ("Links", NV0c) | a: traversal contract; b: auto drop/jump links per class envelope; c: interaction slots (IN43); d: pathgrid links, area costs per class; e: per-water-body swim polys | roadmap Stage 2 |
| **NV10a-c** runtime rebuild ("Runtime rebuild") | a: door poly state; b: dirty-tile rebuild + swap; c: missing tiles + disk cache | its Tests; roadmap Stage 5 |
| **NV11** worldspace graph (a-d1 **built 2026-11-26**) | `LinkId`, `WorldGraph`, `portal_query`, Bevy resource (d1); d2 hearing through it = AI8c3 | exact 3-space routes; LOS only through open seamless links |
| **NV12** route graph (split a-f by NV0b1 2026-11-26; "Route graph") | a: `RouteGraph` core + builder (`sf_nav`, + `blake3` dep); b: `roads.json` source; c: Morrowind source (`mw_bake`); d: Detour-measured `cost_m`, `nav.coarse` deleted; e: `journey` over route graphs + `WorldGraph`; f: `sf_nav_bevy` resource `NavRouteGraphs` + `nav.routes` console | a: "Route graph" Tests; b: exact graph from a fixture; c: Balmora-Caldera route exists; d: cost within the Detour band; e: exact 3-space journey; f: headless load |
| **NV13a-d** volumes ("Volumes") | a: field bake; b: flight query; c: water bodies + swim query; d: measurement | its Tests; roadmap Stage 3 |
| **NV14** stuck monitor ("Stuck monitor") | escalation ladder, `NavEvent::Stuck`, `nav.stuck/min` | its Tests; roadmap Stage 1 |
| **NV15** destruction feed | `Destroyed` on walkable geometry → `NavDirty` (`sf_interaction_bevy` → `sf_nav_bevy`) | roadmap Stage 5 |
| **NVM** measurement | Floor and High sweep: bake time and MiB per class, query and ORCA ms at 60 Hz (Balmora, Vivec canton, Census Office); stuck counts near props | owner runs; numbers into "Budgets"; class or carving revisits |

Order = `docs/roadmaps/nav.md` stages ([roadmap](../../../docs/roadmaps/nav.md) §4; depends and verify live there; NV12a
and NV12c run ahead of Stages 1-3). NV11 replaces seamless SI13 (M9b). Owner checks: NV-3 fliers and swimmers (NV13),
NV-4 changing world (NV10b, NV15), NV-5 road traveller (NV12, with AI-11).

**As built** NV1-NV11d1: [archive](archive/nav-history.md#moved-from-navmd-2026-11-26-nv0b) (moved 2026-11-26); NV12a: "Route graph".

## Open questions

None open. Decided 2026-11-08 (`../research/ai-nav-decisions-2026-11-08.md`): (a) two classes; (b) doors by poly
state, prop carving opt-in later; (c) client UI routes as presentation. Revisit triggers are listed there.
