# Navigation for NPCs and creatures in a streamed open world (research, 2026-11-25)

Scope: research only. Sources are cited inline; items I could not verify from a primary source are marked (unverified).
Our state: wiki `engine/nav-and-ai.md`, `docs/ledger/nav.md`, `superfargoth/docs/specs/nav.md` (NV0-NV12).

## 0. Summary (what to do)

1. The ground stack (Recast tiles + Detour corridor + sliced queries) is the industry standard and is built. Keep it.
2. Biggest player-facing gaps, in order: (a) **no 3D navigation for fliers** (cliff racers are a headline Morrowind
   creature; our non-goal "fliers steer with probes" is below what Horizon shipped in 2017); (b) **no runtime
   generation path** for the generated 10x world (our tiles are baked once from a content pack; the generated world
   needs bake-on-demand or invoker-style generation); (c) long-range routing for off-screen travellers is only a stub
   (NV12 `nav.coarse` = straight-line costs); (d) NV9 traversal links have no animation/ownership contract; (e) no
   NPC-vs-NPC crowd stack beyond ORCA (acceptable).
3. Do not adopt any Bevy navmesh crate as the core. `navmesh-rs` (already vendored) has tilecache and crowd modules;
   use them. Take `rerecast`/`landmass` only as references (see 6).
4. Change: replace the NV12 "HPA* over tile borders" idea by a **road/route graph + tile-border graph** generated
   from the same data that Magna Urbis already emits (roads), and add a **height-layer flight volume** for fliers.
5. Departing from Morrowind pathgrids: keep pathgrids only as bake input (links, wander hints), never as runtime nav.

## 1. Navmesh generation and streaming

**Industry.**
- UE5 World Partition nav: a Base Navmesh built per partition cell in editor, with modes Static, Dynamic Modifiers
  Only, Dynamic (runtime generation); data layers can gate it, but forum reports say data-layer support is thin.
  [World Partitioned Navigation Mesh](https://dev.epicgames.com/documentation/unreal-engine/world-partitioned-navigation-mesh?lang=en-US),
  [forum](https://forums.unrealengine.com/t/world-parition-nav-mesh-with-dynamic-build-and-data-layers/2686688).
- UE5 **Navigation Invokers**: generate tiles only around agents, drop tiles beyond a removal radius; this is the
  standard for procedurally generated or huge worlds where no editor bake exists.
  [Using Navigation Invokers](https://dev.epicgames.com/documentation/unreal-engine/using-navigation-invokers-in-unreal-engine).
  Epic's roadmap item "Navmesh Generation for Large Worlds" targets the same.
- Unity AI Navigation: `NavMeshSurface` per agent type, tiled bake (default tile 256 voxels), runtime build/update via
  `NavMeshBuilder`; `NavMeshModifier` volumes and `NavMeshObstacle` (carving) for dynamic changes.
  [docs](https://docs.unity3d.com/Packages/com.unity.ai.navigation@1.1/manual/NavMeshBuildingComponents.html).
- Horizon Zero Dawn: several navmeshes per agent type (secondhand; unverified).
- Dynamic obstacles everywhere use Detour's **TileCache** model: compressed tile layers (heightfield + regions, no
  contours), obstacles queued, only touched tiles rebuilt (contours + polys), budgeted per frame.

**Our state.** Bake-time only: `nav.tiles` per tile from cooked collision (humanoid 215.8 s / 57,516 tiles, large 69 s;
warm re-bake 2.2 s for 154k hits, ledger 2026-11-18), residency ring 256/320 m, tile reads 4/frame. Dynamic inputs are
tracked so a moved wall re-bakes its tiles (offline). Runtime: door poly state (no rebuild); TileCache carving
planned opt-in NV10.

**Assessment.**
- Baked-on-install for Morrowind is fine (a one-time step under 5 min fits the owner rule). For the **generated 10x
  world** a ~100x bigger area at the same density is ~hours of bake; the standard answer is invoker generation: build
  tiles at runtime near agents from the world's own collision/heightfield, cache them to disk. Our bake already is a
  pure function (tile inputs -> blob, content-hashed, warm hits 14 us/tile), so runtime generation = "run the same
  bake step on a worker when the ring asks for a missing tile and store the result". 4.6 ms per tile (NV2 synthetic)
  makes this cheap for a handful of tiles per second. Owner rule "nothing bakes during play" applies to **baking**;
  the owner decision needed is whether on-demand tile generation with disk cache counts (design-affecting: it gates
  whether modded/edited worlds work). Recommended: yes, as fallback only (missing tile), never for the base set.
- Dynamic obstacles: our policy (doors by poly state, carving only for stationary big props, coalesced, 250 ms
  apart) matches Unity ("carve only stationary") and OpenMW. Gap: **destroyed/moved geometry that is part of the
  tile input** (a collapsed wall, a placed building) goes through the offline dependency re-bake, not a runtime
  one. Recommendation: allow the same tile bake step to run at runtime for a dirty tile (worker, ~5-30 ms/tile at
  our cell size) instead of TileCache layers. This is simpler than TileCache (no layer format, one code path with the
  bake) and is what invokers do. Keep TileCache carving only for the cheap small-obstacle case if measured needed.
  Each rebuilt tile must swap atomically at a tick boundary (host-authoritative, net-ready: rebuild is derived state,
  the cause event is replicated, not the mesh).
- Memory: 304 MiB humanoid tiles for Vvardenfell on disk; resident ring is 517 tiles around the player. Fine.

## 2. Long-range routing and off-screen travel

**Industry.** No shipped open-world game path-plans across the whole world with navmesh. Standard layers:
- **Road/route graph** for NPCs and vehicles (RDR2's patent describes pathfinding over road networks with density and
  junction behaviour; secondhand via
  [G2A](https://www.g2a.com/news/latest/red-dead-redemption-2s-npc-interaction-system/), unverified in detail).
- **ZoneGraph lanes** in UE5 (Mass crowds/traffic): a lane graph with tags, density per lane, spawn points on lanes
  [MassCrowd API](https://dev.epicgames.com/documentation/en-us/unreal-engine/API/Plugins/MassCrowd),
  [City Sample](https://dev.epicgames.com/documentation/unreal-engine/city-sample-project-unreal-engine-demonstration?lang=en-US).
  Our spec lists this as a non-goal "later if measured"; for ambient city crowds it is the standard (Mass), but our
  NPCs are individual simulated actors (schedules, quests), so a navmesh + ORCA is right and a lane graph is only for
  traffic-like ambient flows (carts, caravans). Keep as a later add-on.
- **Level-of-detail simulation**: off-screen NPCs do not move on a navmesh; they advance along a precomputed route
  by travel time and get "materialized" at a position on the path when the player nears (Skyrim/Starfield packages,
  Witcher 3 and Cyberpunk-style tiers; Radiant-style teleports). Our `ai.md` tiers already say tier 0 uses travel time.
- HPA* / abstract tile graph: the academic standard (Botea et al.; Detour has no hierarchy). Cheaper variants: portal
  graph over tile borders (what Recast-based engines do implicitly via sliced A*), and `landmass`'s "islands + nodes".

**Our state/plan.** NV11 `WorldGraph` (door links between spaces) is good and standard (it equals a portal graph).
NV12 coarse graph = "HPA* nodes at tile borders, region borders, door approach points, named anchors": this is the
weak spot. A tile-border graph for a 10x world is ~10^5 nodes and its edge costs need Detour paths between border
points, i.e. a bake of its own.

**Recommendation (change NV12).**
1. Coarse graph = **road/route graph first** (roads from the generated world's road network and the Morrowind silt
   strider/path data, towns as nodes, door approach points as leaf nodes), edge cost = measured walk time from the
   bake. Off-road legs between a road and a goal use a short Detour query inside the ring once the actor is
   materialized. Off-screen travellers: advance by time along the route graph, no tiles needed (tier 0), materialize
   on the road centreline when entering the ring, then switch to tier 1 on the navmesh.
2. A tile-border HPA* level is only added for off-road long routes (creature migration, hunting parties), generated
   lazily: keep tile borders' connectivity bitset (which border poly groups connect) per tile in the blob; A* over
   those at region scale. Defer until measured.
3. Cross-worldspace routing is NV11's job; keep it. It mirrors portal graphs (UE streaming levels, Bethesda doors).
   For interiors, pre-baked tile data is trivial (one tile set per interior) and there is no streaming issue.

## 3. Off-mesh links and traversal

**Industry.** Detour off-mesh connections (`dtOffMeshConnection`: endpoints, radius, direction, area, flags) are the
base; traversal is the game's job. UE5 Navigation Link Proxy / **Smart Links** fire a delegate/event when an agent
reaches a link, and the game (Behavior Tree/State Tree/GAS) plays a jump/climb animation and moves the capsule; Unity
`NavMeshLink` plus `OffMeshLinkData` and `agent.autoTraverseOffMeshLink = false` do the same
[NavMeshLink](https://docs.unity3d.com/Packages/com.unity.ai.navigation@1.1/manual/NavMeshLink.html). Auto-generated
links (jump-down and jump-across) come from the baker with per-class `max_climb` and drop height; Unity's
`NavMeshBuilder` and Recast demo's "off-mesh link generation" do this; UE5 5.x generates "Navigation Link
Generation" at build.

**Our plan.** NV9 includes jump/drop/ladder/pathgrid links; `TraverseKind` exists; `sf_interaction` supplies ledges.

**Recommendation.**
- Make traversal an explicit contract: the link carries `TraverseKind` and a **traversal operator** (same kind as
  `MoveTo`) that takes ownership of the mover, plays the animation, and releases it on the end point (or fails and
  reports to the planner as a cost penalty/blocked link, so path repair avoids it). Doors already follow this pattern
  (NV11). Reuse the **player's mantle/vault** controller states: the AI asks the same state machine for "mantle to
  this edge" (one implementation, network-consistent, host-authoritative). Do not invent NPC-only climb code.
- Auto-generate drop and jump-down links from the bake (class `max_climb`, `max_drop`), and ladders/climb points
  from `sf_interaction` data; humanoid only at first, creatures get their own per-class link flags (a cat can jump up
  1.5 m, a guar cannot). Per class mask on links: `OffMeshLink.classes` already exists.
- Swimming: area type `water` with cost per class (humanoid pays 5x, swimmers 1x, `can_swim` in `AgentClass`
  exists); transitions at shores need no link if the water polygons touch the shore (we bake a flat swim-level quad,
  OK for sea level; lakes at different heights need per-water-body polygons: ties to the world10x water bodies).
- Levitation (a spell): treat as temporary agent class swap to `flyer` volume (below), not as a link.

## 4. Agent profiles

**Industry.** One navmesh per (agent radius/height/slope) class: Unity "agent types", UE `SupportedAgents`. Cost of
extra classes scales with `1/cs^2` (our spec notes it). Typical shipped counts: 2-4 ground classes (human, large,
huge/vehicle). Fliers: **Horizon Zero Dawn used a height map with hierarchical planning over mipmaps** after
rejecting waypoint graphs, voxel grids, sparse voxel octrees and layered navmeshes
([Putting the AI Back Into Air, GDC 2017/18](https://www.slideshare.net/guerrillagames/putting-the-ai-back-into-air-navigating-the-air-space-of-horizon-zero-dawn-83388716));
queries: minimum safe flying height at (x,y) and a height-map raycast. UE has no built-in 3D nav; community 3D navs
use sparse voxel octrees (A* over octree leaves, as in the "3D Pathfinding Plugin" family; unverified). Unity: no 3D
nav; Aron Granberg's A* Pathfinding Project is the usual add-on.

**Our state.** Two ground classes (`humanoid`, `large`); fliers/swimmers fall back to **PGRD polyline** walks
(`mw_walkers_bevy`); spec non-goal says fliers use probes.

**Recommendation (new, ranked high).**
- **Fliers: a terrain-clearance height layer plus the tile geometry's own height.** For an open-world ARPG whose fliers
  mostly roam over terrain (cliff racers, birds, bats), the HZD result applies: store a coarse `min_safe_height`
  field per tile (max of terrain + prop top + clearance; mip chain for hierarchical queries), sampled on streaming
  tiles; paths are 2D A* on the field with a vertical profile, with raycast-clear shortcuts; interiors and cave
  ceilings need a `max_height` layer too (a second field). That is ~2 bytes/cell at 4-8 m resolution, i.e. KBs per
  tile, built in the same bake pass. Avoid octrees unless there are true 3D caves with flying (not Morrowind).
- **Swimmers (slaughterfish)**: a **water-volume layer** is simpler than a navmesh: per water body store surface
  height and bed height field; swimmers path 2D over cells with depth > body height and steer in 3D within
  `[bed + r, surface - r]`. Use the same field machinery as fliers (clearance in the vertical direction inverted).
  Do not generate Recast "swim" meshes for deep water (voxels over open water give giant flat polys, and depth is
  lost).
- **Mounts/vehicles**: a `mount` class at the large radius with larger turn radius; path steering needs
  curvature-limited following (the existing `PathFollower` springs can do wide turns, no Reeds-Shepp needed). Defer
  vehicles (not Morrowind), keep boats as swimmer-on-surface.
- Keep 2 ground classes; if small creatures (rats, mudcrabs) fit humanoid corridors the third class is not needed
  (spec decision (a) holds). Add `huge` only for a real creature (e.g., netch) when it appears.

## 5. Local movement: crowds, avoidance, formations, stuck

**Industry.**
- Detour Crowd (`dtCrowd`): per-agent corridor + `dtObstacleAvoidanceQuery` (sampled velocity obstacles) + separation
  - topology optimisation + path-queue. Our corridor is a port; ORCA (RVO2) is stronger on reciprocity and is the
  standard in Unity `NavMeshAgent` (RVO-like) and Mass (UE5 Mass Avoidance, ORCA-style, plus nearby-obstacle
  segments). ORCA with wall segments is ours (NV7b); `dodgy_2d/3d` (MIT/Apache) is the Rust implementation to compare
  against `sf_controller::avoid`.
- Mass/lane flows for dense ambient crowds: non-goal for individually simulated NPCs.
- **Formations/following/escort**: standard is a leader-follower slot model (slots in leader frame, each follower
  paths to its slot with ORCA), keep-distance and speed-matching (RDR2 posses, Dragon Age). Escorts: leader speed
  clamp to the slowest follower, wait at corners.
- **Stuck detection/recovery** (universal): progress monitor (distance to next corner over a window), then
  escalate: repath -> corridor repair -> teleport-to-nearest-navmesh-point when unseen -> abort the action and
  report to the planner (HTN replans). UE's `PathFollowingComponent` and Unity agents do this implicitly.

**Our state.** NV7a/NV8 done; NV7b pending; no stuck monitor or formation/follow documented in nav spec.

**Recommendation.**
- NV7b stays (a prerequisite for believable doorways), but run it only for tier 1-2 actors near the player, as planned.
- Add a **stuck monitor** inside `NavCorridor` (progress window, 3 escalation steps, event to the AI), which feeds
  nav counters (stuck/min) for NVM. Cheap, high player impact: stuck NPCs are the most visible nav bug.
- Follow/escort: plan as an AI package type (MA2) on top of `MoveTo`: leader position target with a slot offset;
  no separate nav feature needed. Teleport-near rule when a follower is out of the ring behind the player (Bethesda
  standard, hidden from view).

## 6. Crates (checked on crates.io 2026-11-25)

| Crate | Version | Bevy | Licence | Last release | Notes |
|---|---|---|---|---|---|
| `navmesh-rs` (ours, vendored) | 0.2.6 (vendored 0.2.2) | none (pure Rust) | MIT OR Apache-2.0 | 2026-11-21 | Recast + Detour + **tilecache + crowd** modules; we patch DetachedQuery; upstream is moving (0.2.2 -> 0.2.6): check whether the patch is still needed and re-pin at the next update-all pass |
| `rerecast` / `bevy_rerecast` | 1.0.0 / 0.6.1 | 0.20 path via `bevy_rerecast` | MIT OR Apache-2.0 | 2026-11-13 | Rust port of Recast (generation only, no tilecache/crowd); reference for our generation code and an editor/BRP bake integration; not a replacement |
| `vleue_navigator` / `polyanya` | 0.17.0 / 0.18.0 | `bevy ^0.20` | MIT OR Apache-2.0 | 2026-10-27 | 2D/2.5D navmesh, Polyanya any-angle; runtime obstacle updates; not for 3D terrain with multiple floors |
| `landmass` / `bevy_landmass` / `landmass_rerecast` | 0.9.3 / 0.13.0 | `bevy_* ^0.20` | MIT OR Apache-2.0 | 2026-10-27 | pathfinding + steering + avoidance (via dodgy) on any navmesh; islands/ nodes architecture; good reference for region handling; ours is further along |
| `oxidized_navigation` | 0.12.0 | `bevy ^0.15` | MIT OR Apache-2.0 | 2024-12-25 | stale on Bevy 0.15; drop |
| `dodgy_2d` / `dodgy_3d` | 0.5.5 / 0.5.2 | none | MIT OR Apache-2.0 | 2025-04 / 2024-12 | ORCA; compare with our `orca.md` implementation (batch = single bit-for-bit test is ours); 3D variant could serve flyers |
| `bevy_orca` | 0.2.1 | bevy_math only | MIT OR Apache-2.0 | 2026-10-30 | 2D ORCA LP, new, small |
| `bevy_northstar` | 0.8.0 | `bevy ^0.20` | MIT | 2026-10-29 | grid HPA* (hierarchical) with chunked abstract graph: reference implementation of HPA* on tile chunks, 2D/3D grids; not a navmesh |
| `avian_navmesh` | 0.0.1 | n/a | n/a | 2025-06 | placeholder |
| `recast-navigation` | 0.1.0 | none | n/a | 2022 | abandoned FFI |

No Rust crate gives 3D flying volumes or a road-network routing layer; both are our own (small) code. No licence
issue: all MIT/Apache; no AGPL/GPL-2.0-only found.

## 7. Gaps, ranked by player impact, with recommendations

| # | Gap | Impact | Recommendation | Ref |
|---|---|---|---|---|
| 1 | Fliers and swimmers have no real nav (PGRD fallback; spec non-goal) | cliff racers/fish clip through terrain, can't reach targets | **Adopt** height-layer flight field + water-body volume layer; **change** the non-goal; a new slice "NV13 volumes" with an exact-test on clearance | HZD GDC |
| 2 | NV5 residency not generalized (one anchor, humanoid only) | NPCs near remote players/portals and large creatures don't path | **Keep** NV5 as planned; do it before AI6 packages go multi-actor; add on-demand tile generation fallback for the generated world | UE invokers |
| 3 | Missing tile generation at runtime (generated 10x world) | world can't be walked without a multi-hour bake | **Add** runtime bake worker for missing/dirty tiles with disk cache; replaces most TileCache carving | UE invokers |
| 4 | NV12 coarse graph = tile-border HPA* stub | off-screen travellers/quest routes across the region are straight lines | **Change**: road/route graph + door approach nodes, travel time from bake; tile-border HPA* only if measured | RDR2 patent, UE ZoneGraph |
| 5 | NV9 traversal has no animation/ownership contract | NPCs teleport or fail at ledges and ladders | **Specify** traversal operator reusing player mantle/vault controller state; per-class link masks; auto drop/jump links in the bake | UE smart links, Unity links |
| 6 | No stuck monitor or follow/escort/formation design | NPCs stuck in doors, followers lost | **Add** stuck monitor to `NavCorridor`; follow = AI package on `MoveTo` (MA2) | UE PathFollowing |
| 7 | NV7b ORCA walls | NPCs squeeze through walls in crowds | **Keep** as is | dodgy, RVO2 |
| 8 | NV10 carving | rare (crates) | **Downgrade**: keep opt-in, but prefer tile rebuild | Unity carving |
| 9 | Lane graph for ambient traffic | only caravans/crowds | **Keep** as non-goal; revisit when ambient flows are designed | UE Mass |
| 10 | Re-pin `navmesh-rs` (0.2.2 -> 0.2.6) | maintenance | update-all pass; check whether upstream took the detached query | -- |

**Candidate owner questions** (design/player-facing only):
1. **Do creatures that fly or swim path through the real world (obstacle-aware, 3D), or may they keep today's
   fixed pathgrid flights?** Recommended: real 3D (height field + water volumes). Changes what the player sees
   (cliff racers dive around towers; fish stay in water).
2. **Should NPCs that travel between towns appear on the roads (visible, simulated) when the player is near, with
   off-screen ones advancing by travel time?** Recommended: yes (materialize on the road, tier switch).
3. **May the world change the walkable ground at runtime (a collapsed wall, a built house) with NPCs re-pathing
   around it?** Recommended: yes, via tile rebuild; limited to authored/destructible dynamic geometry, not general
   physics props.
4. **Is on-demand tile generation (cached to disk) acceptable for generated worlds, given "nothing bakes during
   play"?** Recommended: yes as missing-tile fallback only; the base set is baked once at world creation.

## Missing / not verified
- Primary-source detail of RDR2, Cyberpunk, Decima route layers: only a patent news item found (secondhand).
- UE5 smart-link and Mass avoidance specifics (docs not fetched); Unity `NavMeshObstacle` carve docs not fetched.
- Cost numbers for runtime tile rebuild at our cell size (4.6 ms/tile is the NV2 synthetic figure; real towns need
  a measurement in NVM). Not benchmarked here by rule.
- `navmesh-rs` 0.2.6 changelog not read (whether it includes our detached-query patch).
