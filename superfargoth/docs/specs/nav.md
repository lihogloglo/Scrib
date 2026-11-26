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
> **Agent summary (2026-11-11).** Draft; NV1 (`sf_nav` core) and NV2 (`sf_nav_recast`) built ("Build plan"). Recast/Detour
> navmesh per worldspace, agent class and nav region, baked once by `sf_bake` (`nav.tiles`); two classes (humanoid, large).
> Doorways are baked door polygons switched at runtime by area/flags; prop carving (TileCache) is opt-in later (owner:
> patches are not "baking during play"). Tiles stream with a fixed nav ring around gameplay anchors. Routing:
> worldspace graph (door links) → per-space route graph (NV12) → Detour over resident tiles → corridor → our ORCA (+ navmesh
> wall lines), on the fixed tick (NV7a). Backend behind `NavBackend`: `navmesh-rs` (NV0 verdict).
> Read next: "Data model", "Routing", "Streaming", "Build plan".
<!-- /agent-summary -->

## Goal

Every non-player mover (NPCs, creatures, followers, escorts) gets paths that respect the same collision the mover
collides with, inside one worldspace and across door links, whether the door is seamless or fade-only. The same
graphs serve quest markers and the compass. Host-only for gameplay; clients route locally for UI only ("Co-op").
Non-goals: flying 3D navigation (fliers steer with probes), flow fields (RTS tool), a lane graph for city crowds (UE
ZoneGraph; later if measured), navmesh authoring tools (data comes from the bake; overrides are mods).

## Owner decisions this spec implements (2026-11-08)

| Decision | Here |
|---|---|
| Layering and the split `sf_nav` + `sf_ai` (+ planned `sf_interaction`) | this crate set; ai.md |
| Runtime tile patches for doors and props are fine; they are not "baking during play" | "Runtime patches" |
| Spike `navmesh-rs` first; C++ recastnavigation FFI is the accepted fallback | NV0, "Backend" |
| Seamless interiors optional per door / per pack: AI works with seamless and fade-only doors | "Doors" |
| No multiplayer ceiling; gameplay never depends on graphics settings | "Streaming" (fixed nav ring), "Co-op" |
| Steam Deck is the floor | "Budgets" |
| Delegated 2026-11-08: two agent classes; doors by poly flags, no prop carving by default; client UI routes | "Data model", "Runtime patches", "Co-op" |
| Owner 2026-11-11: NPCs climb, mantle and vault with the player's own moves (guards can follow onto roofs); later, after NV9: bake-time auto links from ledges in the mover's mantle/vault/drop envelopes, per class (UE NavLink generation, Unity auto off-mesh links) | NV9b (planned) |

## Canonical technique

**Recast voxel navmesh generation + Detour tiled navmesh queries** (Mononen; used by UE, Unity, Godot, OpenMW),
tiled and baked offline, with **TileCache** layers for runtime obstacles, **off-mesh connections** for links the
mesh can't express, a **road/route graph** above the navmesh ("Route graph"; HPA*, Botea et al. 2004, only on a measured need), and **path corridor +
reciprocal velocity obstacles** for following and local avoidance (Detour `dtPathCorridor`; ORCA, van den Berg 2011).
Deviation: we keep our own ORCA (`sf_controller::avoid`, deterministic, tested) instead of DetourCrowd's sampled
avoidance, and take only `PathCorridor` and `LocalBoundary` from Detour.

## Reuse (Bevy, then crates, then our own)

- **Bevy 0.20** ships no navigation.
- **`navmesh-rs` 0.2.2** (MIT OR Apache-2.0 + zlib, pure Rust Recast + Detour + DetourCrowd + TileCache, bit-exact
  with upstream `9f4ce644`; one author): **adopted (NV0, 2026-11-11)**, pinned `=0.2.2`, features `libm`
  (cross-platform determinism) + `polyref64`, vendored with one own patch (`superfargoth/vendor/README.md`), behind
  `NavBackend` in `sf_nav_recast`. NV0 baked all our cooked collision with no failures; full bake ≈ 55 s at 16
  threads, over the 45 s slot (NV4 tunes). [Spike report](../research/navmesh-spike-nv0-2026-11-11.md). Fallback
  (not needed): recastnavigation through a vendored `cc` build.
- Rejected (research §5.2): `rerecast` (no tiles; re-check at "update all"), `landmass` (glam 0.30 + `dodgy_2d`, no
  dynamic obstacles; its islands idea becomes nav regions), `vleue_navigator`/`polyanya` (2D/layered),
  `oxidized_navigation` (archived), `bevy_northstar`/flow-field crates (grids). `petgraph` covers the graphs.
- **Route graph (NV12, checked 2026-11-26):** Bevy 0.20 has no graph search (no A*/Dijkstra in any `bevy_*` 0.20 crate;
  `bevy_math` curves are Bevy-side, `sf_nav` is Bevy-free). `petgraph` 0.8.3 (MIT OR Apache-2.0, maintained, in the
  workspace) and `pathfinding` (MIT OR Apache-2.0; `Ord` costs, so f64 needs a wrapper) were rejected for this query:
  both fix their own tie-break, neither seeds a search mid-edge or ends at a virtual goal without mutating the graph.
  Own ~100-line A* in the NV11b pattern (`world_graph.rs` `Open`: `total_cmp` + index tie-break), CSR-style arrays.
- Our own: the thin glue (types, trait, graphs, query service, streaming, debug draw); no Recast/Detour rewrite.

## Data model

Built in NV1 (`sf_nav::types`, the code is the reference): `AgentClass { id, radius, height, max_climb, max_slope_deg,
can_swim }` (adapter record `sf.nav.agent`), `AreaDef { id, name, cost: [f32; MAX_CLASSES], flags }` in the
adapter's `AreaTable` (`sf.nav.area`), `PolyFlags` (WALK, SWIM, DOOR, JUMP, AVOID, DISABLED + adapter bits), `TileKey
{ ws, class, region, x, z }`, `OffMeshLink { id: NavLinkId, a, b, radius, two_way, kind: TraverseKind, area, classes
}` (`TraverseKind::Door(LinkId)` with NV11), `NavPoint { loc, poly }`. Serde except the backend-opaque `PolyRef` /
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
- **Off-mesh links** (bake, per worldspace): pathgrid edges the mesh misses, authored jumps, drops, ladders, ledges
  (`sf_interaction` or adapter data), door approach links.
- **Route graph** (bake, per worldspace, class mask per edge; replaces the HPA* "coarse graph", NV12): junctions,
  anchors and door approach nodes, polyline edges with walk-metre costs. ≤ 1.5 MiB Vvardenfell; always resident.
  "Route graph".
- **Worldspace graph** (`WorldGraph`, built at load from `DoorLink`s + the coarse graphs): nodes are worldspaces,
  edges are link ends with an approach point on each side, walking cost through, and flags (`Seamless`, `FadeOnly`,
  `OneWay`); dynamic door state (open, locked) comes from the host each tick.
- **Door polygons** (bake product): the bake drops door leaves from the geometry and marks each doorway footprint with
  a convex area volume (`door`), so every doorway has its own polygons, listed per tile as `DoorPolys { door, polys }`.
- **Runtime obstacles** (not saved, NV10, opt-in): `NavObstacle { id, ws, shape: Cylinder | Box }`, a TileCache carve.

`LinkId` does not exist yet (`DoorLinks` is a `Vec<DoorLink>`): `sf_world` gains a stable `LinkId` (hash of both
ends' worldspace and frame) in NV11, since the AI saves routes by link.

## Interfaces

`NavBackend` (`sf_nav::backend`, built NV1; the code is the reference): `build_tile(input, cfg, areas)` (bake only),
`add_tile` / `remove_tile` / `tile`, `nearest`, sliced `find_path(&self, &mut PathJob, &mut SliceBudget)`, `raycast`,
`poly_state` / `set_poly_state` (doors: `setPolyArea`/`setPolyFlags`, no rebuild), and read-only debug accessors
(NV3) `tile_summaries` (key, bytes, polygons per resident tile) and `tile_geometry` (a tile's polygons with wall
edges, region-local, built on request), and corridor moves (`is_valid_poly`, `closest_point_on_poly`,
`move_along_surface`, `corridor_corners`, `repair_corridor`, NV8). Still to add: local boundary (`dtLocalBoundary`, NV7b), `set_obstacle` / `clear_obstacle` /
`rebuild_dirty` (TileCache, NV10, opt-in), and bake-side `read_tile_blob` (NV4a: a blob's polygons with edge links:
wall, in-tile polygon, or portal to a tile side). The bake's adapter seam (`sf_nav_bake`, built NV4a):

```rust
/// The adapter's geometry for the bake: the same cooked collision the mover uses. Reads go through `cx`.
pub trait NavGeometrySource: Send + Sync {
    fn regions(&self, ws: &WorldspaceId) -> Vec<NavRegion>;            // plan time: origin + XZ extent per region
    fn tile_input(&self, cx: &mut RunCx, ws: &WorldspaceId, class: &AgentClass, bounds: &NavTileBounds)
        -> Result<NavTileSource, BakeError>;                            // TileInput + DoorVolume { door, volume }
    fn links(&self, cx: &mut RunCx, ws: &WorldspaceId) -> Result<NavLinks, BakeError>; // links + door approaches
}
```

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

## Streaming

- **Nav ring**: tiles are resident around every `Gameplay` anchor (players, and on a host every remote player) and
  around the mapped anchor of every open portal (the simulated set, seamless §4.2). Default load at R0 load + 64 m
  (256 m), unload at R0 unload + 64 m (320 m). **Fixed per game, never scaled by the graphics preset** (vision
  §4.2.4): what an NPC can path must not depend on a client's settings, and a headless server uses the same ring.
  R0 itself is fixed too (only R1-R3 scale; streaming.md fix in the decisions note). Per class: see "Agent classes".
- Requests ride the streaming scheduler's per-frame budgets (streaming.md "Scheduler and budgets"); a tile is a bake
  store read and `add_tile`, no generation.
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

**Budgets** (to measure, NV12d/NVM): Vvardenfell ≤ 5k nodes, 6k edges, 60k points (≤ 1.5 MiB); 10x world ≤ 50k nodes,
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

## Doors

- Each link end has an **approach point** (adapter: nearest walkable point in front of the door, from pathgrid nodes
  in Morrowind) inside its worldspace's mesh; the `WorldGraph` edge joins the two.
- Seamless vs classic is a **host world rule** (`world.doors`, saved with the world; pack default, per-door data
  overrides), never a graphics option, because it changes sight, witnesses and crossings (seamless research §7 (i)).
- **Seamless link** (portal-ready, seamless on): the traverse operator opens the door through activation rules, waits
  for `passable`, walks through the `Gate`, and closes a door it found shut; `InWorldspace` changes at the crossing.
- **Fade-only link** (seamless off, trapdoors, one-way): the actor takes it only when no player can see either end
  (no viewer within the adapter's `door_watch_radius` with LOS to the end); otherwise it waits nearby or re-routes.
  This is Bethesda's out-of-view door teleport and works with seamless disabled for the whole pack.

## Debug draw (`sf_nav_bevy`, feature `debug`)

Bevy gizmos, off by default: `nav.draw tiles|links|doors|obstacles|paths <actor>|boundary <actor>|coarse|graph`.
`nav.stats` prints tiles resident per class, MiB, queue depth, ms per tick, rebuilds; BRP serves the same as JSON.
Automated checks use log lines only (no visual judgement by agents). Built in NV3: `tiles`, `paths [actor]`, `off`,
`nav.stats`, BRP `sf_nav/stats`; the other layers print the slice that builds their data, counters not yet kept
print `n/a`.

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
| Tile rebuilds (worker) | ≤ 0.10 ms | ≤ 0.20 ms |
| Tile residency | ≤ 64 MiB | ≤ 128 MiB |

These scale with the host's CPU tier (ai.md "Tiers and budget"), never with the graphics preset.

## Industry comparison

**Match** UE, Unity, Godot and OpenMW on Recast generation, agent classes as data, area costs and filters, runtime
obstacles (door poly state, carving opt-in), off-mesh links, a coarse graph above the navmesh, Detour corridor
following, RVO-family avoidance (ours ORCA, deterministic) and debug display. **Differ** from OpenMW on when it is
built (baked per tile, mod-aware; nothing bakes during play); **ahead** of OpenMW across load doors (`WorldGraph`,
fade links only unseen; Bethesda's out-of-view teleport). Full table and sources:
[archive](archive/nav-history.md#industry-comparison-table-moved-from-navmd-at-nv4a).

**Route graph (NV12):** coarse routing on a road/path graph with lane-location progress, as UE ZoneGraph + Mass LOD and
GTA V/RDR2 path nodes, not tile-border HPA*; tier 0 advances by time (Bethesda unloaded Travel packages, KCD2), ahead
of OpenMW, where actors outside the processing range freeze ("Route graph" Industry line).

## Build plan

One deliverable plus its test per slice; research and building are separate.

| Slice | Deliverable | Test |
|---|---|---|
| **NV0-NV4a** (**done 2026-11-11**) | spike, `sf_nav` core, `sf_nav_recast`, `sf_nav_bevy`, `sf_nav_bake` steps | rows with their tests: [archive](archive/nav-history.md#done-build-plan-rows-moved-from-navmd-at-nv4a) |
| **NV4b** Morrowind bake (**done 2026-11-11**) | adapter `NavGeometrySource` (cooked collision, terrain, water, doors), `bevyrrowind bake` wiring | cold bake time and MiB per class on the first pack (the "Bake" gate) |
| **NV4c** game wiring (**built 2026-11-17**, player-ring scaffold until NV5) | `sf_nav_bevy` in the game: `NavPlugin`, baked tiles into `NavMeshes`, draw frame from the floating origin, BRP `sf_nav/stats` | owner check NV-1 (tiles drawn over Seyda Neen) |
| **NV5** streaming | nav ring residency per gameplay anchor and open portal, per class on demand (needs seamless SI3) | residency set exact for scripted anchor paths; `large` tiles absent with no large actor; unload hysteresis |
| **NV6** query service (**built 2026-11-17**) | worker queue, slicing, tick-boundary results, partial results at the ring edge | ordering and budget exact; sync mode equals async results |
| **NV7a** fixed tick (**built 2026-11-17**) | NPC drive (path follower, ORCA, mover) in `FixedUpdate` + render interpolation | headless: walker positions identical at 30 and 144 fps for the same ticks |
| **NV7b** ORCA walls | obstacle lines in `sf_controller::avoid` | agent never crosses a wall segment; batch = single bit for bit |
| **NV8** corridor (**built 2026-11-18**) | `PathCorridor` feeding `PathFollower`; repair after a push | headless walk around a pillar and through a door frame, nothing stuck |
| **NV9** links + areas | off-mesh links (jump, drop, ladder, pathgrid) and area costs per class | path takes the cheaper area; link traversal events in order |
| **NV10** runtime patches | door poly state (area/flags); opt-in TileCache carving (`carves_nav`, at rest), rebuild budget | shut door raises cost with zero rebuilds, locked blocks except for an admitted agent; carve rebuilds per tick ≤ budget |
| **NV11** worldspace graph (split a-d 2026-11-26, owner unparked: built headless before portals) | `LinkId` (a, done), `WorldGraph` (b), `portal_query` (c), Bevy resource built from `DoorLinks` (d1), hearing propagation through it (d2 = AI8c3) (supersedes seamless SI13) | exact routes on a synthetic 3-space graph; LOS through an open seamless opening, none through fade links |
| **NV12** route graph (split a-f by NV0b1 2026-11-26; "Route graph") | a: `RouteGraph` core + builder (`sf_nav`, + `blake3` dep); b: `roads.json` source; c: Morrowind source (`mw_bake`); d: Detour-measured `cost_m`, `nav.coarse` deleted; e: `journey` over route graphs + `WorldGraph`; f: `sf_nav_bevy` resource `NavRouteGraphs` + `nav.routes` console | a: "Route graph" Tests; b: exact graph from a fixture; c: Balmora-Caldera route exists; d: cost within the Detour band; e: exact 3-space journey; f: headless load |
| **NVM** measurement | Floor and High sweep: bake time and MiB per class, query and ORCA ms at 60 Hz (Balmora, Vivec canton, Census Office); stuck counts near props | owner runs; numbers into "Budgets"; class or carving revisits |

Order: NV0 → NV1 → NV2 → NV3 → NV4a → NV4b → NV4c → NV6 → NV7a → NV8 (→ AI6) → NV5 → NV7b (DECISIONS 2026-11-17: NV5 and seamless SI3 no longer gate AI6) → NV9 → NV10 → NV11 → NV12; NVM
after NV8. NV11 replaces seamless SI13 (M9b); NV7a lands before AI6.

**As built** (full notes: [archive/nav-history.md](archive/nav-history.md)). **NV1**: the trait as in "Interfaces",
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

## Open questions

None open. Decided 2026-11-08 (`../research/ai-nav-decisions-2026-11-08.md`): (a) two classes; (b) doors by poly
state, prop carving opt-in later; (c) client UI routes as presentation. Revisit triggers are listed there.
