# Navigation: navmesh tiles, links, the worldspace graph (`sf_nav`)

Status: **draft, docs only (2026-11-08)**, written on a machine that cannot compile. Nothing built. Scope label M7
(RPG core: AI). Slices NV0-NV12 below; NV0 (the `navmesh-rs` spike) is research and comes first.
Crates (planned): `sf_nav` (Bevy-free core: types, `NavBackend` trait, worldspace graph, `portal_query`, query
service logic), `sf_nav_recast` (backend over `navmesh-rs`, chosen in NV0; `sf_nav_recast_sys` is the C++ FFI
fallback), `sf_nav_bevy` (thin: tile residency, worker tasks, debug draw, console). Game side: `mw_ai` + `mw_bake`
([ai-adapter](../../../bevyrrowind/docs/specs/ai-adapter.md) "Navigation").
Sources: `../research/npc-ai-2026-11-08.md` §4, §5.2-5.3, §7.4 (approved); `../research/seamless-interiors-2026-11-08.md`
§4.2, §4.8; [ai.md](ai.md) (the main consumer); [streaming.md](streaming.md) "Rings"; [bake.md](bake.md) "Data model";
[locomotion.md](locomotion.md) "Local avoidance"; [net.md](net.md) "Net-ready rules".

<!-- agent-summary -->
> **Agent summary (2026-11-08).** Draft, nothing built. Recast/Detour navmesh per worldspace and agent class, baked
> once by `sf_bake` (`nav.tiles`), patched at runtime by TileCache obstacles (doors, big props; owner: not "baking
> during play"). Tiles stream with a fixed nav ring around gameplay anchors. Three-level routing: worldspace graph
> (door links) → per-space coarse graph → Detour over resident tiles → corridor → our ORCA (+ navmesh wall lines).
> Backend behind `NavBackend`: `navmesh-rs` after the NV0 spike, recastnavigation FFI as fallback (owner).
> Read next: "Data model", "Routing", "Streaming", "Build plan".
<!-- /agent-summary -->

## Goal

Every non-player mover (NPCs, creatures, followers, escorts) gets paths that respect the same collision the mover
collides with, inside one worldspace and across door links, whether the door is seamless or fade-only. The same
routes serve quest markers and the compass. Host-only for gameplay; clients may run local route queries for UI.

Non-goals: flying 3D navigation (fliers steer with probes, ai.md "Executors"), flow fields (RTS tool; research §6),
a lane graph for dense city crowds (UE ZoneGraph; later if measured need), navmesh authoring tools (data comes from
the bake; authored overrides are mods).

## Owner decisions this spec implements (2026-11-08)

| Decision | Here |
|---|---|
| Layering and the split `sf_nav` + `sf_ai` (+ planned `sf_interaction`) | this crate set; ai.md |
| Runtime tile patches for doors and props are fine; they are not "baking during play" | "Runtime patches" |
| Spike `navmesh-rs` first; C++ recastnavigation FFI is the accepted fallback | NV0, "Backend" |
| Seamless interiors optional per door / per pack: AI works with seamless and fade-only doors | "Doors" |
| No multiplayer ceiling; gameplay never depends on graphics settings | "Streaming" (fixed nav ring), "Co-op" |
| Steam Deck is the floor | "Budgets" |

## Canonical technique

**Recast voxel navmesh generation + Detour tiled navmesh queries** (Mononen; used by UE, Unity, Godot, OpenMW),
tiled and baked offline, with **TileCache** layers for runtime obstacles, **off-mesh connections** for links the
mesh can't express, **hierarchical routing** (HPA*, Botea et al. 2004) above the tile level, and **path corridor +
reciprocal velocity obstacles** for following and local avoidance (Detour `dtPathCorridor`; ORCA, van den Berg 2011).
Deviation: we keep our own ORCA (`sf_controller::avoid`, deterministic, tested) instead of DetourCrowd's sampled
avoidance, and take only `PathCorridor` and `LocalBoundary` from Detour.

## Reuse (Bevy, then crates, then our own)

- **Bevy 0.20** ships no navigation.
- **`navmesh-rs` 0.2.2** (MIT/Apache, pure Rust Recast + Detour + DetourCrowd + TileCache, claims bit-exact parity
  with upstream `9f4ce644`; 2 days old, one author): adopt **after NV0** re-runs its parity suite, bakes our
  geometry and reads the code. **Fallback**: upstream recastnavigation (zlib) through refreshed `-sys` bindings or a
  vendored `cc` build (`sf_nav_recast_sys`). If navmesh-rs really is bit-exact, its tile blobs may match Detour's
  native format, so one bake could serve either backend (NV0 checks).
- Rejected (research §5.2): `rerecast` (no tiles yet; re-check at the next "update all"), `landmass` (glam 0.30 and
  `dodgy_2d`, both rejected; no dynamic obstacles; its islands-with-transforms idea is copied as nav regions),
  `vleue_navigator`/`polyanya` (2D/layered, no voxel 3D for multi-floor interiors), `oxidized_navigation` (archived),
  `bevy_northstar`/`bevy_flowfield_tiles_plugin` (grids). `petgraph` (in the tree) covers the graphs.
- Our own: the thin glue (types, trait, graphs, query service, streaming, debug draw); no Recast/Detour rewrite.

## Data model

All types derive `Clone, Debug, PartialEq, Serialize, Deserialize` unless noted. Backend refs never leave memory.

```rust
// sf_nav (Bevy-free; deps sf_ids, sf_coords, sf_world, glam, petgraph, serde). Illustrative, not compiled.
pub struct AgentClassId(pub u8);
pub struct AgentClass { pub id: AgentClassId, pub radius: f32, pub height: f32, pub max_climb: f32,
                        pub max_slope_deg: f32, pub can_swim: bool }        // adapter data (record `sf.nav.agent`)
pub struct AreaId(pub u8);                                                // adapter table `sf.nav.area`
pub struct AreaDef { pub id: AreaId, pub name: Symbol, pub cost: [f32; MAX_CLASSES], pub flags: AreaFlags } // WALK, SWIM, DOOR, AVOID
pub struct TileKey { pub ws: WorldspaceId, pub class: AgentClassId, pub region: u16, pub x: i32, pub z: i32 }
pub struct NavLinkId(pub u64);                                            // stable: hash of (kind, ends)
pub enum TraverseKind { Walk, Jump, Drop, Climb, Ladder, Swim, Door(LinkId) }
pub struct OffMeshLink { pub id: NavLinkId, pub a: Location, pub b: Location, pub radius: f32, pub two_way: bool,
                         pub kind: TraverseKind, pub area: AreaId, pub classes: ClassMask }
pub struct NavPoint { pub loc: Location, pub poly: PolyRef }              // PolyRef: backend-opaque, never saved/sent
```

- **Agent classes**: content data, one navmesh per (worldspace, class). Morrowind's starting set is in the adapter.
  Each class costs bake time and memory, so the count is a measured choice (NV4).
- **Area types**: per-polygon ids from the bake (ground, water, door, authored path, hazard) or from runtime
  obstacles (closed door), with a cost per agent class and flags for query filters (swimmers, avoid-hazard). The
  framework names no areas; the adapter's table does.
- **Nav regions**: a worldspace's mesh is split into regions of at most `region_size` (default 16 km, f32 precision
  and poly-ref bits), each a Detour mesh in local coordinates with a `WorldPos` origin (landmass's islands idea).
  Small worldspaces (every Morrowind one, verify the exterior extent in NV0) are one region. Region borders are coarse
  graph nodes; a corridor hands over at a border.
- **Tiles** (bake product): one blob per `TileKey`, backend format, LZ4 in the bake store. Defaults to calibrate in
  NV0: cell size 0.15 m, cell height 0.1 m, 128 voxels per tile side (19.2 m), Recast's rule `cs ≈ radius / 2`.
- **Off-mesh links** (bake product, per worldspace): pathgrid edges the mesh misses (adapter), authored jumps, drops,
  ladders and ledges (from `sf_interaction` edges once it exists, or adapter data), and door approach links.
- **Coarse graph** (bake product, per worldspace and class): HPA*-style nodes at tile-border crossings, region
  borders, door approach points and named anchors; edges with walking cost from the bake. KB to low MB per worldspace;
  always resident.
- **Worldspace graph** (`WorldGraph`, built at load from `DoorLink`s + the coarse graphs): nodes are worldspaces,
  edges are link ends with an approach point on each side, walking cost through, and flags (`Seamless`, `FadeOnly`,
  `OneWay`); dynamic door state (open, locked) comes from the host each tick.
- **Runtime obstacles** (not saved): `NavObstacle { id, ws, shape: Cylinder | Box, area: Option<AreaId> }` with
  `None` = carve, `Some` = re-mark the area (closed door = `door_closed` cost).

`LinkId` does not exist yet (`DoorLinks` is a `Vec<DoorLink>`): `sf_world` gains a stable `LinkId` (hash of both
ends' worldspace and frame) in NV11, since the AI saves routes by link.

## Interfaces

```rust
/// What a backend provides; the core, the bake step and the query service use only this.
pub trait NavBackend: Send + Sync {
    fn build_tile(&self, input: &TileInput, cfg: &BuildConfig) -> Result<Vec<u8>, NavError>;  // bake only
    fn add_tile(&mut self, key: TileKey, blob: &[u8]) -> Result<(), NavError>;
    fn remove_tile(&mut self, key: TileKey);
    fn nearest(&self, at: &Location, class: AgentClassId, extent: Vec3, f: &Filter) -> Option<NavPoint>;
    fn find_path(&self, job: &mut PathJob, budget: &mut SliceBudget) -> JobState;         // sliced A*
    fn raycast(&self, from: &NavPoint, to: Vec3, f: &Filter) -> NavRay;                   // walkable straight line?
    fn corridor_move(&self, c: &mut Corridor, agent: Vec3, target: Vec3, f: &Filter);     // dtPathCorridor
    fn boundary(&self, at: &NavPoint, range: f32, f: &Filter, out: &mut Vec<Segment>);    // dtLocalBoundary
    fn set_obstacle(&mut self, o: &NavObstacle) -> DirtyTiles;                            // TileCache
    fn clear_obstacle(&mut self, id: ObstacleId) -> DirtyTiles;
    fn rebuild_dirty(&mut self, budget: &mut SliceBudget) -> Vec<TileKey>;
}

/// The adapter's geometry for the bake: the same cooked collision the mover uses.
pub trait NavGeometrySource: Send + Sync {
    fn tile_input(&self, cx: &mut RunCx, ws: WorldspaceId, bounds: Aabb) -> TileInput;   // triangles + area marks + water
    fn links(&self, cx: &mut RunCx, ws: WorldspaceId) -> Vec<OffMeshLink>;
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
| `route(Location, Location, class) -> Route` | worldspace hops + coarse legs + travel time; no tiles needed | inline (graph) |
| `portal_query::map / path_distance / line_of_sight` | below | inline |

`Goal` covers `Point(Location)`, `Near { at, radius }` (follow, escort), `Away { from, min_dist }` (flee: search for
the cheapest polygon past the distance, Detour-style), and `AnyOf(Vec<Location>)` (nearest of several slots).

### The shared through-door query (`sf_nav::portal_query`)

One Bevy-free API for everything that crosses spaces: AI perception, audio, crime witnesses, net interest, quest
markers (seamless §4.2; it replaces OpenMW's scattered `positionFor` calls).

- `map(loc, link) -> Location`: through the link's rigid transform (`DoorLink::cross`).
- `path_distance(a, b, max_hops) -> Option<(f32, SmallVec<LinkId>)>`: same space = straight distance; else the
  cheapest `WorldGraph` route summing straight legs through each opening's centre. `Cost::Walk` uses coarse-graph
  costs instead (for AI timing); hearing uses the cheap form (OpenMW `heard` semantics).
- `line_of_sight(a, b, los) -> Los`: same space = one ray; across a link only when the link is **seamless and open**
  (`PortalState.openness > 0`): ray A → a point in the opening, then ray from the mapped point → B. Fade-only or shut
  links block sight; hearing through them takes the adapter's `door_attenuation`. Rays go through a `LosProvider`
  (batched per worldspace in `sf_physics`), so `sf_nav` doesn't depend on Rapier.

## Bake (`sf_bake`, one-time, mod-aware)

- Step **`nav.tiles`**, subject `(worldspace, class, tile)`: input = the adapter's `NavGeometrySource` for the tile's
  bounds plus a border (Recast tile border), whose reads (cooked collision products, terrain heights, water level)
  are recorded as dynamic dependencies. A mod that moves one wall changes only the tiles whose inputs it touched.
- Step **`nav.links`** per worldspace (off-mesh links, door approach points) and **`nav.coarse`** per (worldspace,
  class) (border nodes and costs; depends on that worldspace's tile products).
- Parallel per tile (`sf_bake` execution); empty tiles (open sea, void) produce no object.
- **Gate**: a cold bake of every navmesh for the first pack stays within the "few minutes" baking rule on the High
  reference machine, all classes included; measured in NV4, and a reason to drop a class or coarsen exterior tiles.

## Runtime patches (TileCache; not a bake)

The baked tiles are the base layer; the host applies **obstacles** at runtime (owner decision 2):
- **Doors**: a shut door marks its doorway `door_closed` (cost, still pathable: the NPC opens it); a locked door the
  agent can't open marks `blocked` for that agent's filter (filters are per query, so the mesh is shared).
- **Large movable props** (crates, carts, a dropped body if large) once at rest: carve a box. Small items never
  carve; ORCA and the mover handle them.
- Rebuilds run on the worker under a per-tick budget, coalesced per tile, minimum 250 ms between rebuilds of one
  tile (OpenMW's `min update interval`), applied at a tick boundary in tile-key order. Obstacles are derived state
  (door state, prop transforms): not saved, recomputed on load.

## Streaming

- **Nav ring**: tiles are resident around every `Gameplay` anchor (players, and on a host every remote player) and
  around the mapped anchor of every open portal (the simulated set, seamless §4.2). Default load at R0 load + 64 m
  (256 m), unload at R0 unload + 64 m (320 m). **Fixed per game, never scaled by the graphics preset** (vision
  §4.2.4): what an NPC can path must not depend on a client's settings, and a headless server uses the same ring.
- Requests ride the streaming scheduler's per-frame budgets (streaming.md "Scheduler and budgets"); a tile is a bake
  store read and `add_tile`, no generation.
- Coarse graphs and the `WorldGraph` are resident for every worldspace (small), so tier-1/0 actors and long routes
  never need tiles outside the ring.
- Memory: target ≤ 64 MiB of tiles on the Floor tier (to calibrate in NVM; OpenMW's 256 MiB cache is a ceiling, not a
  target).

## Routing

Three levels, each only as detailed as the actor's tier needs (ai.md "Tiers"):
1. **Worldspace graph**: which door links to take (A* on `WorldGraph`, cost = walking time, filtered by door state
   and the agent's keys or rules).
2. **Coarse graph** inside each space: HPA* legs between border nodes; tier 0 stops here and uses its travel time.
3. **Detour** over resident tiles from the agent to the next coarse node or the goal; sliced on the worker.

Then **corridor** (`dtPathCorridor`: string pulling, `movePosition`, visibility and topology optimisation, repair when
the agent is pushed off) → steer target → **ORCA** with navmesh wall segments → `LocomotionIntent` → mover.

**Query service** (core logic, worker execution in `sf_nav_bevy`): requests carry `(RequestId = (ActorKey, seq),
class, from, goal, filter, priority, tick)`; a priority queue (combat and player-visible first) feeds sliced jobs
with a node budget per slice and a time budget per tick; results are applied at the next tick boundary in
`RequestId` order. A request whose tiles aren't resident returns `Partial` up to the ring edge plus the coarse
route beyond. Tests use a synchronous mode that solves inline.

## Crowds and avoidance (`sf_controller::avoid`)

- ORCA stays ours (DECISIONS 2026-10-12 orca). Added: **static obstacle lines** from `LocalBoundary` (navmesh edges
  within ~2 m of the agent, RVO2's obstacle half-planes), the item DECISIONS left "waiting for navmesh paths".
  The batch must still equal the single-agent scan bit for bit.
- Avoidance runs among tier 1-2 movers in the same worldspace; the player stays `NonReactive`.
- Doorways and corridors: the existing keep-right and priority rules; door traversal is queued by the AI operator
  (one agent through a door at a time, a claim on the link's approach point).
- **Fixed tick**: today's walkers step ORCA and the mover once per frame (DECISIONS walkers); net rule 1 needs the
  fixed tick (physics.md "Controller", net.md N0). NV7 moves the AI-driven path into `FixedUpdate`.

## Doors

- Each link end has an **approach point** (adapter: nearest walkable point in front of the door, from pathgrid nodes
  in Morrowind) inside its worldspace's mesh; the `WorldGraph` edge joins the two.
- **Seamless link** (portal-ready, seamless on): the traverse operator opens the door through activation rules, waits
  for `passable`, walks through the `Gate`; the actor's `InWorldspace` changes at the crossing.
- **Fade-only link** (seamless off, trapdoors, one-way): the actor takes it only when no player can see either end
  (no viewer within the adapter's `door_watch_radius` with LOS to the end); otherwise it waits nearby or re-routes.
  This is Bethesda's out-of-view door teleport and works with seamless disabled for the whole pack.

## Debug draw (`sf_nav_bevy`, feature `debug`)

Bevy gizmos, off by default, console `nav.draw <what>` with `tiles` (borders, coloured by area), `links` (off-mesh,
door edges), `obstacles`, `paths <actor>` (waypoints, corridor), `boundary <actor>` (ORCA wall lines), `coarse`
(graph), `graph` (worldspace hops as text). `nav.stats` prints tiles resident per class, MiB, queue depth, ms per
tick, rebuilds. BRP exposes the same as JSON so agents can inspect without screenshots. Log lines only for automated
checks (no visual judgement by agents).

## Co-op and saves (net-ready rules)

1: queries and obstacle updates apply on the fixed tick. 2: nothing reads input. 3: nav state is static data or
derived from replicated state. 4: a query's result depends only on (tiles, obstacles, query); worker timing changes
the tick it lands at most, never its content; host-only, so no replay is needed. 5: host-owned for gameplay; clients
may build local copies (identical bake, content fingerprint) for quest-marker and compass routes from replicated door
state. 6: anchors per player, no singleton. 7: `Location`, `LinkId`, `NavLinkId`, never `PolyRef` or `Entity` on the
wire or in saves. 8: n/a.
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

"Unverified" marks claims from memory not re-read on 2026-11-08.

| Technique | Industry | Ours | Verdict |
|---|---|---|---|
| Navmesh generation | Recast voxelisation: UE `RecastNavMesh`, Unity AI Navigation (`NavMeshSurface` per agent type), Godot `NavigationServer3D`, OpenMW `detournavigator` | Recast via backend trait | **match** |
| When it's built | UE static or dynamic tiles, nav invokers around agents; OpenMW generates async around the player with a disk cache and `navmeshtool` pregen; Bethesda authors navmesh per cell in the Creation Kit; Decima generates around the player at runtime (unverified) | baked per tile, mod-aware, patched at runtime | **match** (UE static + dynamic modifiers); differ from OpenMW's runtime generation (our "nothing bakes during play") |
| Agent sizes | UE supported agents, Unity agent types, OpenMW one mesh per agent bounds | one mesh per class, classes as data | **match** |
| Area costs | UE `NavArea` classes + query filters, Unity areas + cost, OpenMW ground/water/door/pathgrid (door cost 2) | area table per class + per-query filters | **match** |
| Runtime obstacles | Detour TileCache, Unity `NavMeshObstacle` carving, UE dynamic obstacles / modifiers | TileCache obstacles for doors, big props | **match** |
| Off-mesh links | UE `NavLinkProxy` (smart links), Unity `NavMeshLink`, OpenMW pathgrid edges as off-mesh connections, Bethesda door triangles | jump/drop/ladder/pathgrid/door links | **match** |
| Across load doors | Bethesda: door markers teleport out of view; OpenMW: none (followers teleport); UE World Partition streams nav data with cells | `WorldGraph` + traverse; fade links only unseen | **match** Bethesda; **ahead** of OpenMW |
| Hierarchical routing | HPA* (Botea 2004); UE hierarchical pathfinding (deprecated, unverified); Killzone/Decima graph abstraction (unverified) | worldspace graph + coarse graph + Detour | **match** |
| Path following | Detour `dtPathCorridor`, UE `UPathFollowingComponent`, Unity agent | Detour corridor + our `PathFollower` braking | **match** |
| Local avoidance | DetourCrowd (sampled RVO), UE RVO / Detour crowd, Unity agents (RVO), Godot RVO2 | our ORCA + navmesh wall lines | **match**; deterministic, unlike sampled crowd |
| Debug | UE nav display + Visual Logger, Recast demo, OpenMW navmesh/agent path render | gizmos + `nav.*` + BRP | **match**; recording is ai.md's trace |
| Scaling cost | tiles grow with nav ring area × classes × anchors; queries with path length (bounded by the coarse graph); rebuilds with moving props; avoidance with tier 1-2 agents | caps per CPU tier | checked |

**Sources**: Mononen, recastnavigation (https://github.com/recastnavigation/recastnavigation); Botea, Müller, Schaeffer,
"Near Optimal Hierarchical Path-Finding" (JoGD 2004); van den Berg et al., ORCA (2011); Unity AI Navigation docs;
UE navigation docs; OpenMW `components/detournavigator/` at `b8285cc9` and the navigator settings page (research
"Sources").

## Build plan

One deliverable plus its test per slice; research and building are separate.

| Slice | Deliverable | Test |
|---|---|---|
| **NV0** spike (research) | `navmesh-rs` verdict: re-run its parity suite, bake the Census Office and a 3×3 Balmora block from cooked collision in a throwaway harness, measure ms per tile, MiB, query µs, extrapolate a full bake; read the code (unsafe, panics, licence); check tile format vs Detour | report in `../research/` + `#[ignore = "measurement: ..."]` numbers |
| **NV0b** (only if NV0 fails) | FFI fallback spike: refreshed recastnavigation bindings, same measurements | same |
| **NV1** core | `sf_nav`: types, `NavBackend`, `GridBackend` double, `Goal`, filters | exact paths on synthetic grids |
| **NV2** backend | `sf_nav_recast`: `build_tile`, tiles, `nearest`, sliced `find_path`, `raycast` | paths on synthetic boxes, ramps, two floors; no NaN |
| **NV3** debug draw | `sf_nav_bevy` gizmos + `nav.draw` / `nav.stats` + BRP over a test bake | headless plugin smoke; stats line counts tiles |
| **NV4** bake step | `nav.tiles`, `nav.links`, `nav.coarse` steps + `NavGeometrySource` | cache hit on rerun; moving one box re-bakes only its tiles; cold bake time printed |
| **NV5** streaming | nav ring residency per gameplay anchor and open portal | residency set exact for scripted anchor paths; unload hysteresis |
| **NV6** query service | worker queue, slicing, tick-boundary results, partial results at the ring edge | ordering and budget exact; sync mode equals async results |
| **NV7** ORCA walls | obstacle lines in `sf_controller::avoid`; AI path on the fixed tick | agent never crosses a wall segment; batch = single bit for bit |
| **NV8** corridor | `PathCorridor` feeding `PathFollower`; repair after a push | headless walk around a pillar and through a door frame, nothing stuck |
| **NV9** links + areas | off-mesh links (jump, drop, ladder, pathgrid) and area costs per class | path takes the cheaper area; link traversal events in order |
| **NV10** runtime patches | TileCache obstacles for doors and big props, rebuild budget | shut door raises cost, locked blocks for one filter only; rebuild count per tick ≤ budget |
| **NV11** worldspace graph | `LinkId`, `WorldGraph`, `portal_query` (supersedes seamless SI13) | exact routes on a synthetic 3-space graph; LOS through an open seamless opening, none through fade links |
| **NV12** coarse graph | HPA* legs, travel time for tier 0 | coarse cost within the measured band of Detour path length on test levels |
| **NVM** measurement | Floor and High sweep: bake time, MiB, query and ORCA ms (Balmora, Vivec canton, Census Office) | owner runs; numbers into "Budgets" |

Order: NV0 → NV1 → NV2 → NV3 → NV4 → NV5 → NV6 → NV7 → NV8 → NV9 → NV10 → NV11 → NV12; NVM after NV8.

## Open questions

1. Agent classes for the first pack: two (humanoid, large creature) or three (add small)? NV4's bake time decides.
2. Are big props (crates, carts) carving obstacles from day one, or only doors until a need shows up?
3. Clients computing routes locally for the compass (identical bake): acceptable, or always ask the host?
