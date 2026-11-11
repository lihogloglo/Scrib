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
