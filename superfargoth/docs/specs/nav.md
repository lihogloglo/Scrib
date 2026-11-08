# Navigation: navmesh tiles, links, the worldspace graph (`sf_nav`)

Status: **draft, docs only (2026-11-08)**, written on a machine that cannot compile. Nothing built. Scope label M7
(RPG core: AI). Slices NV0-NV12 below; NV0 (the `navmesh-rs` spike) is research and comes first. The former open
questions are decided (2026-11-08, owner-delegated): reasoning in `../research/ai-nav-decisions-2026-11-08.md` (a-c).
Crates (planned): `sf_nav` (Bevy-free core: types, `NavBackend` trait, worldspace graph, `portal_query`, query
service logic), `sf_nav_recast` (backend over `navmesh-rs`, chosen in NV0; `sf_nav_recast_sys` is the C++ FFI
fallback), `sf_nav_bevy` (thin: tile residency, worker tasks, debug draw, console). Game side: `mw_ai` + `mw_bake`
([ai-adapter](../../../bevyrrowind/docs/specs/ai-adapter.md) "Navigation").
Sources: `../research/npc-ai-2026-11-08.md` §4, §5.2-5.3, §7.4 (approved); `../research/seamless-interiors-2026-11-08.md`
§4.2, §4.8; [ai.md](ai.md) (the main consumer); [streaming.md](streaming.md) "Rings"; [bake.md](bake.md) "Data model";
[locomotion.md](locomotion.md) "Local avoidance"; [net.md](net.md) "Net-ready rules".

<!-- agent-summary -->
> **Agent summary (2026-11-08).** Draft, nothing built. Recast/Detour navmesh per worldspace and agent class, baked
> once by `sf_bake` (`nav.tiles`); two classes (humanoid, large). Doorways are baked door polygons switched at runtime
> by area/flags; prop carving (TileCache) is opt-in later (owner: patches are not "baking during play"). Tiles stream
> with a fixed nav ring around gameplay anchors. Routing: worldspace graph (door links) → per-space coarse graph →
> Detour over resident tiles → corridor → our ORCA (+ navmesh wall lines), on the fixed tick (NV7a).
> Backend behind `NavBackend`: `navmesh-rs` after the NV0 spike, recastnavigation FFI as fallback (owner).
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
  vendored `cc` build (`sf_nav_recast_sys`). If bit-exact, its tiles may match Detour's format (NV0 checks).
- Rejected (research §5.2): `rerecast` (no tiles; re-check at "update all"), `landmass` (glam 0.30 + `dodgy_2d`, no
  dynamic obstacles; its islands idea becomes nav regions), `vleue_navigator`/`polyanya` (2D/layered),
  `oxidized_navigation` (archived), `bevy_northstar`/flow-field crates (grids). `petgraph` covers the graphs.
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

- **Agent classes**: content data, one navmesh per (worldspace, class); Morrowind bakes **two**, `humanoid` (small
  creatures too) and `large` (adapter). Voxel cost scales with `1 / cs²` and `cs ≈ radius / 2`, so a large class costs
  ~10 % of the humanoid bake and a small one ~4×: no `small` class (decisions (a)). A class's tiles are resident only
  around anchors whose simulated set holds an actor of that class (humanoid always). NV4 prints time and MiB per class.
- **Area types**: per-polygon ids from the bake (ground, water, door, path, hazard) or set at runtime (closed door),
  a cost per agent class, flags for filters (swimmers, avoid-hazard). The adapter's table names them, not the framework.
- **Nav regions**: meshes split into regions of at most `region_size` (16 km: f32 precision, poly-ref bits), each a
  Detour mesh in local coordinates with a `WorldPos` origin (landmass's islands); every Morrowind worldspace is one
  region (NV0 verifies). Region borders are coarse nodes; corridors hand over there.
- **Tiles** (bake product): one blob per `TileKey`, backend format, LZ4. NV0 calibrates: humanoid cs 0.15 m, ch 0.1 m,
  128 voxels per side (19.2 m); `large` cs ≈ 0.5 m (`cs ≈ radius / 2`).
- **Off-mesh links** (bake, per worldspace): pathgrid edges the mesh misses, authored jumps, drops, ladders, ledges
  (`sf_interaction` or adapter data), door approach links.
- **Coarse graph** (bake, per worldspace and class): HPA* nodes at tile borders, region borders, door approach points,
  named anchors; walking-cost edges. KB to low MB; always resident.
- **Worldspace graph** (`WorldGraph`, built at load from `DoorLink`s + the coarse graphs): nodes are worldspaces,
  edges are link ends with an approach point on each side, walking cost through, and flags (`Seamless`, `FadeOnly`,
  `OneWay`); dynamic door state (open, locked) comes from the host each tick.
- **Door polygons** (bake product): the bake drops door leaves from the geometry and marks each doorway footprint with
  a convex area volume (`door`), so every doorway has its own polygons, listed per tile as `DoorPolys { door, polys }`.
- **Runtime obstacles** (not saved, NV10, opt-in): `NavObstacle { id, ws, shape: Cylinder | Box }`, a TileCache carve.

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
    fn set_door(&mut self, d: &DoorPolys, area: AreaId, flags: PolyFlags);               // setPolyArea/Flags, no rebuild
    fn set_obstacle(&mut self, o: &NavObstacle) -> DirtyTiles;                            // TileCache (NV10, opt-in)
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
  cheapest `WorldGraph` route of straight legs through each opening's centre (hearing, OpenMW `heard`); `Cost::Walk`
  uses coarse-graph costs (AI timing).
- `line_of_sight(a, b, los) -> Los`: same space = one ray; across a link only if **seamless and open** (`openness >
  0`): ray A → opening, then mapped point → B. Fade-only or shut links block sight; hearing through them takes the
  adapter's `door_attenuation`. Rays go through a `LosProvider` (batched in `sf_physics`): no Rapier in `sf_nav`.

## Bake (`sf_bake`, one-time, mod-aware)

- Step **`nav.tiles`**, subject `(worldspace, class, tile)`: input = the adapter's `NavGeometrySource` for the tile's
  bounds plus a border (Recast tile border), whose reads (cooked collision products, terrain heights, water level)
  are recorded as dynamic dependencies. A mod that moves one wall changes only the tiles whose inputs it touched.
- Step **`nav.links`** per worldspace (off-mesh links, door approach points) and **`nav.coarse`** per (worldspace,
  class) (border nodes and costs; depends on that worldspace's tile products).
- Parallel per tile (`sf_bake` execution); empty tiles (open sea, void) produce no object.
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
- Coarse graphs and the `WorldGraph` are resident for every worldspace (small), so tier-1/0 actors and long routes
  never need tiles outside the ring.
- Memory: ≤ 64 MiB of tiles on the Floor (NVM calibrates; OpenMW's 256 MiB cache is a ceiling, not a target).

## Routing

Three levels, each only as detailed as the actor's tier needs (ai.md "Tiers"):
1. **Worldspace graph**: which door links to take (A* on `WorldGraph`, cost = walking time, filtered by door state
   and the agent's keys or rules).
2. **Coarse graph** inside each space: HPA* legs between border nodes; tier 0 stops here and uses its travel time.
3. **Detour** over resident tiles from the agent to the next coarse node or the goal; sliced on the worker.

Then **corridor** (`dtPathCorridor`: string pulling, `movePosition`, optimisation, repair when pushed off) → steer
target → **ORCA** with navmesh wall segments → `LocomotionIntent` → mover.

**Query service** (core logic, worker execution in `sf_nav_bevy`): requests `(RequestId = (ActorKey, seq), class,
from, goal, filter, priority, tick)` in a priority queue (combat and player-visible first) feed sliced jobs (node
budget per slice, time budget per tick); results apply at the next tick boundary in `RequestId` order. Tiles not
resident: `Partial` to the ring edge plus the coarse route. Tests use a synchronous inline mode.

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
Automated checks use log lines only (no visual judgement by agents).

## Co-op and saves (net-ready rules)

1: queries and obstacle updates apply on the fixed tick. 2: nothing reads input. 3: nav state is static data or
derived from replicated state. 4: a query's result depends only on (tiles, obstacles, query); worker timing changes
the tick it lands at most, never its content; host-only, so no replay is needed. 5: host-owned for gameplay. **Clients
compute compass and quest-marker routes locally as presentation** (decisions (c)): `WorldGraph` + coarse graphs from
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

## Build plan

One deliverable plus its test per slice; research and building are separate.

| Slice | Deliverable | Test |
|---|---|---|
| **NV0** spike (research) | `navmesh-rs` verdict: re-run its parity suite, bake the Census Office and a 3×3 Balmora block from cooked collision in a throwaway harness, measure ms per tile, MiB, query µs, extrapolate a full bake; read the code (unsafe, panics, licence); check tile format vs Detour, poly area/flag setters, convex area volumes | report in `../research/` + `#[ignore = "measurement: ..."]` numbers |
| **NV0b** (only if NV0 fails) | FFI fallback spike: refreshed recastnavigation bindings, same measurements | same |
| **NV1** core | `sf_nav`: types, `NavBackend`, `GridBackend` double, `Goal`, filters | exact paths on synthetic grids |
| **NV2** backend | `sf_nav_recast`: `build_tile`, tiles, `nearest`, sliced `find_path`, `raycast` | paths on synthetic boxes, ramps, two floors; no NaN |
| **NV3** debug draw | `sf_nav_bevy` gizmos + `nav.draw` / `nav.stats` + BRP over a test bake | headless plugin smoke; stats line counts tiles |
| **NV4** bake step | `nav.tiles`, `nav.links`, `nav.coarse` steps + `NavGeometrySource`, door polygons | cache hit on rerun; moving one box re-bakes only its tiles; each doorway has its own polys; cold bake time and MiB printed per class |
| **NV5** streaming | nav ring residency per gameplay anchor and open portal, per class on demand (needs seamless SI3) | residency set exact for scripted anchor paths; `large` tiles absent with no large actor; unload hysteresis |
| **NV6** query service | worker queue, slicing, tick-boundary results, partial results at the ring edge | ordering and budget exact; sync mode equals async results |
| **NV7a** fixed tick | NPC drive (path follower, ORCA, mover) in `FixedUpdate` + render interpolation | headless: walker positions identical at 30 and 144 fps for the same ticks |
| **NV7b** ORCA walls | obstacle lines in `sf_controller::avoid` | agent never crosses a wall segment; batch = single bit for bit |
| **NV8** corridor | `PathCorridor` feeding `PathFollower`; repair after a push | headless walk around a pillar and through a door frame, nothing stuck |
| **NV9** links + areas | off-mesh links (jump, drop, ladder, pathgrid) and area costs per class | path takes the cheaper area; link traversal events in order |
| **NV10** runtime patches | door poly state (area/flags); opt-in TileCache carving (`carves_nav`, at rest), rebuild budget | shut door raises cost with zero rebuilds, locked blocks except for an admitted agent; carve rebuilds per tick ≤ budget |
| **NV11** worldspace graph | `LinkId`, `WorldGraph`, `portal_query` (supersedes seamless SI13) | exact routes on a synthetic 3-space graph; LOS through an open seamless opening, none through fade links |
| **NV12** coarse graph | HPA* legs, travel time for tier 0 | coarse cost within the measured band of Detour path length on test levels |
| **NVM** measurement | Floor and High sweep: bake time and MiB per class, query and ORCA ms at 60 Hz (Balmora, Vivec canton, Census Office); stuck counts near props | owner runs; numbers into "Budgets"; class or carving revisits |

Order: NV0 → NV1 → NV2 → NV3 → NV4 → (seamless SI3) → NV5 → NV6 → NV7a → NV7b → NV8 → NV9 → NV10 → NV11 → NV12; NVM
after NV8. NV11 replaces seamless SI13 (M9b); NV7a lands before AI6.

## Open questions

None open. Decided 2026-11-08 (`../research/ai-nav-decisions-2026-11-08.md`): (a) two classes; (b) doors by poly
state, prop carving opt-in later; (c) client UI routes as presentation. Revisit triggers are listed there.
