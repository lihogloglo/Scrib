# Lane roadmap: Navigation (ground navmesh, links, routes, fliers and swimmers, streaming)

Specs: `superfargoth/docs/specs/{nav,orca}.md`, `bevyrrowind/docs/specs/ai-adapter.md` "Navigation". Research:
`docs/research/gameplay-2026-11-25/nav.md` (cited §n). Ledger `docs/ledger/nav.md`; wiki `docs/wiki/engine/nav-and-ai.md`;
checks NV- in `bevyrrowind/docs/visual-checks/ai.md`. Kept IDs: NV0-NV12, NVM (spec build plan), NV13 (named by ai.md);
new slices take NV14 on. Nav is a service lane: its players are the AI, travel, interaction and combat lanes.

## 1. Player-facing goal

Modern preset, lane done: NPCs and creatures go where they mean to, by the same collision you walk on, and are never
seen stuck in a doorway [1]. A guard chases you through a door into the interior, over a wall and off a ledge with your own
moves, and swims after you [2]. Cliff racers dive around towers and rocks instead of through them; slaughterfish stay in
the water [3]. Travellers walk the roads between towns, there when you arrive whether or not you watched them [4].
A collapsed wall or a smashed barricade changes where NPCs walk [5]. The generated 10x world is as walkable as
Vvardenfell [5]. **Classic** uses the same navigation (OpenMW navigates by navmesh too); only behaviour differs (AI lane).

## 2. Where we are (2026-11-26)

- **Owner-verified:** NV-1 baked tiles stream around the player and follow the terrain (Balmora, 2026-11-18); NV-2 NPCs
  walk smoothly on the fixed tick at any fps (NV7a); MA3 wander on the navmesh (AI-2).
- **Built, headless:** `sf_nav` core + `navmesh-rs` 0.2.2 backend (vendored, one patch); bake `nav.tiles` (humanoid
  57,516 tiles 215.8 s, `large` 69 s, 304.5 MiB; warm re-bake 2.2 s); NV4c ring (one anchor, humanoid only, scaffold
  `NavRingScaffold`; 517 tiles resident at Seyda Neen); NV6 sliced query service; NV8 corridor (Detour parity);
  **NV11a-d1** (2026-11-26): stable `LinkId`, `WorldGraph` door routes, `portal_query` (distance, LOS through seamless
  links), Bevy resource (not yet fed by the game: MA21, ai lane).
- **Stubs:** `nav.coarse` is straight-line costs (NV12); fliers and swimmers walk PGRD polylines (`mw_walkers_bevy`).
- **Not started:** NV5 multi-anchor residency, NV7b ORCA wall lines, NV9 links and areas, NV10 runtime patches, NV12,
  NVM; query and ORCA ms never measured in game.

## 3. Decisions

1. **Keep the ground stack** (§0; Recast tiles + Detour corridor + sliced queries = UE, Unity, Godot, OpenMW). Reuse
   check: Bevy 0.20 ships no navigation; `navmesh-rs` stays the backend (its tilecache and crowd modules are references);
   `rerecast` 1.0 (generation only), `landmass` 0.9, `vleue_navigator` 0.17 (2D), `bevy_northstar` 0.8 (grid HPA*)
   remain references, `oxidized_navigation` is stale (Bevy 0.15). No crate does 3D flight volumes or road routing.
2. **Fliers on a clearance height field, swimmers in a water-body volume** (Horizon Zero Dawn, GDC 2017 "Putting the AI
   back into air", rejected octrees and layered meshes; ai.md decision 11, owner). Per tile: `min_safe_height` (terrain +
   prop tops + clearance) with a mip chain, plus a `max_height` layer for ceilings; 2D A* on the field with a vertical
   profile and ray-clear shortcuts. Swimmers: per water body, surface and bed height; path over cells deeper than the
   body, steer in `[bed + r, surface - r]`. Same bake pass, KBs per tile. No Recast swim meshes for deep water (giant
   flat polys lose depth). Replaces the spec non-goal "fliers steer with probes". Levitation = class swap to the field.
3. **Coarse routing = a road/route graph, not tile-border HPA*** (RDR2 road network, UE ZoneGraph lanes, §2; no shipped
   open world plans whole-world on navmesh): nodes are junctions, settlements and door approach points; edges carry a
   walk time measured at bake. Sources are adapter data: the generated world's `roads.json` (`sf_worldgen::roads`,
   Magna Urbis via scrib-worldgen), Morrowind's exterior pathgrid chains between towns. Off-road legs: a short Detour
   query once the actor is in the ring. Tile-border HPA* only if a measured need appears (creature migrations).
4. **Off-screen actors never touch tiles** (Skyrim/Starfield packages, Witcher 3 tiers): tier 0 advances by time along
   a route; it materializes on the road centreline out of view on entering the ring and drops to tier 1 on the navmesh
   (ai.md AI10, AI46).
5. **Traversal is an explicit contract** (UE Smart Links, Unity `NavMeshLink` with manual traversal): a link carries its
   `TraverseKind`; on arrival nav hands the mover to a traversal operator (AI33) that drives the player's own mantle,
   vault, drop or ladder states and releases it at the far end, or fails and raises that link's cost for repath. No
   NPC-only climb code (owner 2026-11-11). Drops and jump-downs are generated at bake from each class's envelope
   (`max_climb`, `max_drop`; UE Navigation Link Generation); ladders and ledges come from `sf_interaction` slots (IN43).
6. **Pathgrids are bake input only** (§0.5): extra links the mesh misses, wander hints, road chains; never runtime nav.
7. **Runtime tile rebuild instead of TileCache carving** (UE navigation invokers, §1): a tile whose walkable inputs
   changed (destroyed wall, opened barricade) re-runs the same pure `nav.tiles` step on a worker and swaps atomically at
   a tick boundary; derived state, so only the cause event replicates. One code path with the bake, no layer format.
   Doors stay poly-state flips with zero rebuilds (NV10a). Carving only if measured needed. A missing tile (modded or
   generated world) uses the same path with a disk cache (owner 2026-11-26, Q1: amends "nothing bakes during play").
   **Always off the frame** (owner 2026-11-26; CONTRIBUTING "Design constraints"): missing tiles build at a loading
   screen or on worker threads ahead of streaming; a changed tile rebuilds async and the old tile serves until the
   swap; never a stall while playing.
8. **Swimming costs, not links** (§3): area `water` with a per-class cost (humanoid ×5, swimmers ×1); shore polys touch,
   so no link. Lakes and rivers at their own heights need per-water-body polygons from the world10x water bodies.
9. **Two ground classes stay** (spec decision (a); Unity/UE ship 2-4): `huge` only when a real creature needs it.
10. **Stuck monitor in the corridor** (UE `PathFollowingComponent`, universal): progress over a window, then repath →
    corridor repair → snap to the nearest poly when unseen → fail to the planner with a reason; `nav.stuck/min` counter
    feeds NVM. The most visible nav bug, cheap to catch.
11. **Followers, escorts, formations are AI** (ai.md decision 9): slots in the leader frame over `MoveTo` + ORCA; nav adds
    nothing but the stuck monitor and the teleport-near-when-unseen query.
12. **ORCA stays ours** (DECISIONS 2026-10-12): NV7b adds navmesh wall lines (RVO2 obstacle half-planes); `dodgy_2d`
    is the reference test, not a dependency. Batch = single-agent bit for bit.
13. **Residency is fixed, never by graphics preset** (spec "Streaming"): rings around every gameplay anchor (players,
    remote players, far riders TV25, open portals), per class on demand.
14. **Budgets stay the spec's** (Floor/High per tick) until NVM measures; NVM is one run per scene, no repeats (owner
    2026-11-23). All new numbers (field resolution, clearance, stuck window, rebuild cap) are `ai.nav.*` tunables.
15. **Net-ready:** host-only paths and rebuilds, applied at tick boundaries in key order; clients route only for UI.
16. **Re-pin `navmesh-rs`** (0.2.2 → 0.2.6 upstream) only in the batched update-all pass, after checking whether upstream
    took our detached-query patch (NV-R1).

## 4. Stages

### Stage 0: specs and research (docs only)

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| NV0b | nav.md: Goal (non-goal removed), "Volumes" (field, water bodies, queries), "Runtime rebuild", stuck monitor | docs | - | doccheck; done 2026-11-26 |
| NV0b1 | nav.md "Route graph" section (NV12 spec) (**done 2026-11-26**) | docs | - | doccheck |
| NV0c | nav.md "Links": traversal contract (handoff event, failure cost), auto drop/jump envelopes, interaction slots | docs | NV0b | doccheck; done 2026-11-26 |
| NV0d | ai-adapter.md "Navigation": PGRD road chains, water bodies, fliers/swimmers classes, PGRD fallback removal | docs | NV0b | doccheck; done 2026-11-26 |
| NV-R1 | Research (Sonnet): `navmesh-rs` 0.2.6 changelog vs our patch; HZD field details; Detour tile rebuild cost at our cell size | research | - | **done 2026-11-26** (`docs/research/gameplay-2026-11-25/nav-r1.md`; nav.md "Volumes" fixed) |

### Stage 1: NPCs go everywhere you are, and never stay stuck (AI Stages 1-3 lean on it)

Ends with: NPCs path near every player and through open portals; large creatures path; a crowd at a doorway doesn't squeeze
through walls; a stuck NPC recovers and the counter shows it.

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| NV5a | Multi-anchor ring: every gameplay anchor, per class on demand; `NavRingScaffold` removed | `sf_nav`, `sf_nav_bevy`, `mw_walkers_bevy` | - | residency set exact; `large` absent with no large actor |
| NV5b | Open-portal anchors (mapped anchor of each open portal) | `sf_nav_bevy` | NV5a, seamless SI3 | exact set through a portal |
| NV7b | ORCA navmesh wall lines | `sf_controller` (avoid) | - | never crosses a wall; batch = single |
| NV14 | Stuck monitor + escalation + `nav.stuck/min` | `sf_nav`, `sf_nav_bevy` | - | headless pinned agent escalates in order |
| NVM | Measurement: query, corridor+ORCA ms, bake MiB/class, stuck counts (Balmora, Vivec canton, Census Office) | tests, game | NV5a, NV14 | one run per scene; ledger + "Budgets" |

### Stage 2: over walls, off ledges, into water (with AI Stage 5)

Ends with: a guard follows you over a 1.2 m wall, drops off a ledge, swims a canal.

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| NV9a | Link contract: `TraverseKind` on arrival → handoff event, release, failure cost; an actor reaching a link at tier 1 is pinned to tier 2 for the traversal (ai.md AI0f `TraverseLink`) | `sf_nav`, `sf_nav_bevy` | NV0c | link events in order; failed link avoided |
| NV9b | Bake-time auto drop/jump-down links per class envelope | `sf_nav_bake` | NV9a | exact links on a synthetic ledge level |
| NV9c | Interaction links: ladders, ledges, vault edges from `sf_interaction` slots | `sf_nav_bake`, `mw_bake` | NV9a, IN43 | exact; owner with AI33 (AI-7) |
| NV9d | Pathgrid links the mesh misses; area costs per class (water, hazard, path) | `mw_bake`, `sf_nav` | NV9a | path takes the cheaper area |
| NV9e | Per-water-body swim polygons (lakes and rivers at their own heights) | `sf_nav_bake`, `mw_bake` | NV9d, world10x water bodies | shore join without links |

### Stage 3: fliers and swimmers (with AI34, AI35, MA12)

Ends with: cliff racers circle Balmora's towers without clipping; slaughterfish chase you in the Odai and never leave it.

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| NV13a | Clearance field bake: `min_safe_height`, `max_height`, mips, in `nav.tiles` | `sf_nav_bake`, `sf_nav` | NV0b | exact on synthetic towers; KB/tile printed |
| NV13b | Flight query: 2D A* on the field, vertical profile, ray-clear shortcuts, in the query service | `sf_nav`, `sf_nav_bevy` | NV13a | never under clearance |
| NV13c | Water-body volume: surface + bed per body, depth-filtered cells, swim band query | `sf_nav`, `mw_bake` | NV0d | stays in band; exact cell set |
| NV13d | Measurement: field bake s and MiB for Vvardenfell, query µs | tests | NV13b, NV13c | ledger |

### Stage 4: roads and long journeys (with AI10, AI46, travel TV19-TV21)

**Pulled forward (order.md):** NV12a and NV12c run right after MA21, ahead of Stages 1-3: AI10 (AI Stage 2, a day in
town) needs them.

Ends with: a pilgrim walks Balmora to Caldera; leave and come back, and she's further along the road, appearing out of view.

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| NV12a | `RouteGraph` core + builder: nodes, polyline edges with walk-metre cost, A*, position-at-time, nearest road point (nav.md "Route graph") (**done 2026-11-26**: 14 exact tests; 50k grid route 3.8 ms avg, 15 ms worst) | `sf_nav` | NV0b1 | the spec's "Tests (NV12a)" list, all exact |
| NV12b | Generated-world source: `roads.json` → `RouteInput` (field layout confirmed here) | `sf_nav_bake`, `sf_worldgen` reader | NV12a | exact node/edge/kind set from a fixture; postcard bytes stable |
| NV12c1 | `nav.routes` step + `RouteSource` seam (`sf_nav_bake`) (**done 2026-11-26**: `RouteBake`, `RouteInput`, bake tunables `ai.nav.route.simplify_m`/`door_connect_m`; 6 exact tests, sf_nav_bake 11 pass) | `sf_nav_bake` | NV12a | baked = builder output, shuffled input same bytes, reads/params re-bake, `None` = zero bytes, format prefix |
| NV12c2 | Morrowind `RouteSource` (`mw_bake`): ai-adapter.md "Route source" rules 1-8; `approach_point` moved to Bevy-free `mw_ai::nav`, MA21 calls it (**done 2026-11-26**: 10 exact tests; real install 8794 nodes / 17712 edges / 1.31 MiB, 220 components; **Balmora → Caldera `Unreachable`**: vanilla has 148 exterior PGRDs, settlements only, so no inter-town roads; open question for NV12d) | `mw_bake`, `mw_ai`, `mw_exterior_bevy` | NV12c1, MA21, NV0d | Balmora → Caldera anchor route is `Graph`; door nodes = `WorldGraph` approaches; component histogram printed |
| NV12c3 | Inter-town edges for the Morrowind route graph: vanilla exterior PGRDs cover settlements only (NV12c2: 220 components), so components are bridged by Detour walks on the baked humanoid navmesh between the nearest anchor/door nodes of neighbouring components (kind `Path`, measured cost; NV12d's walk machinery); road LTEX under a walk may later mark it `Road` | `mw_bake`, `sf_nav_bake` | NV12c2, NV12d | Balmora → Caldera anchor route is `Graph` (measurement assert); bridged-component histogram printed; worst route time on the joined graph |
| NV12d | Bake-measured edge costs (Detour walk per edge into `cost_m`, class bit cleared on failure; edge times include link traversal time, ai.md AI0f), `nav.coarse` + `CoarseGraph` deleted | `sf_nav_bake` | NV12b or NV12c | every `cost_m` ≥ `length_m`; failing class cleared on a synthetic wall; clamp count printed |
| NV12e | `journey`: one A* over route graphs + `WorldGraph` door hops (interiors = straight clique) | `sf_nav` | NV12a | exact 3-space journey; locked door reroutes; ties by id |
| NV12f | `NavRouteGraphs` resource from `nav.routes` products, `nav.routes` console | `sf_nav_bevy` | NV12a | headless load, counts logged |

### Stage 5: a world that changes (with interaction IN1, worldgen)

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| NV10a | Door polys: closed = cost, locked = filtered unless admitted; zero rebuilds | `sf_nav`, `sf_nav_bevy` | MA21 | exact filter cases |
| NV10b | Dirty-tile runtime rebuild: worker, budget, coalesced, atomic swap at tick boundary; first move navmesh-rs 0.2.2 -> 0.2.6 (0.2.3 `replace_tile` / `replace_tile_budgeted` is the swap; rebase our `DetachedQuery` patch, upstream keeps it crate-private; compare tile-blob blake3 before/after, differ = rebake), unless the batched "update all" pass does it first (NV-R1, `docs/research/gameplay-2026-11-25/nav-r1.md`) | `sf_nav_bevy`, `sf_nav_bake` | NV0b | rebuilt tile = offline bake bit for bit; ≤ budget |
| NV10c | Missing-tile generation with disk cache, off the frame (loading screen or workers ahead of streaming; owner Q1, yes) | `sf_nav_bevy` | NV10b | generated = baked; second load reads cache |
| NV15 | Destruction feed: `Destroyed` on walkable geometry marks tiles dirty | `sf_interaction_bevy` → `sf_nav_bevy` | NV10b, IN1 | broken barricade opens a path; owner NV-4 |
| NV16 | Activators and containers are navmesh obstacles (the mover's solid set; IN32d dc64d84b) | `mw_bake` | – | **done 2026-11-26** (`is_solid` = `mw_world::source::MOVER_SOLID`, shared with the physics hook; `RULES_VERSION` 2; 2 tests) |

**Parked:** lane graph for ambient traffic (UE Mass; revisit with ambient flows), tile-border HPA*, `huge` class,
TileCache carving, deck-local nav IN22 (interaction lane builds it, nav reviews).

## 5. Spec changes

NV0b-NV0d above. nav.md also: Goal drops "flying 3D navigation" non-goal; "Coarse graph" in Data model becomes the route
graph; "Runtime patches" gains the rebuild path and demotes carving; build plan gains NV5a/b, NV9a-e, NV10a-c, NV12a-d,
NV13a-d, NV14, NV15; order line rewritten to this file's stages. Owner checks: NV-3 fliers and swimmers, NV-4 changing
world, NV-5 road traveller (with AI-11).

## 6. Dependencies on other lanes

| Lane | Nav needs | Nav provides |
|---|---|---|
| AI | MA21 (game feeds `NavWorldGraph`), AI33 link operator, AI34/AI35 executors, AI10/AI46 tier 0 | NV5, NV7b, NV9, NV11, NV12, NV13, NV14; ring-slot reachability for AI36 |
| Anim | AL-10, AL-12 NPC mantle/vault states (via AI33), creature fly/swim modes | link endpoints and kinds |
| Interaction | IN43 ladder slots, IN1 `Destroyed`, door objects' state | NV9c, NV10a, NV15; IN22 review |
| Travel | TV25 far-rider anchor | NV5a, NV12 for TV19-TV21 |
| Combat | - | LOS and reachability for ranged bands (C8e), AI36 slots |
| Worldgen (world10x) | `roads.json`, water bodies | NV12b, NV9e |
| Streaming / seamless | SI3 portal anchors | NV5b |

## 7. Owner questions (answered 2026-11-26: both recommendations accepted)

1. **Building navmesh during play:** allowed only for a tile that is missing (modded or generated area) or whose walkable
   ground changed (a destroyed wall), cached to disk so it happens once; the base set is still baked once at install or
   world creation. This amends "nothing bakes during play" for nav tiles (DECISIONS 2026-11-26). (NV10b, NV10c)
2. **Swimming pursuers:** guards and bandits swim after you, slower than on land; creatures that can't swim wait at the
   shore. (NV9d area costs, AI lane pursuit)
