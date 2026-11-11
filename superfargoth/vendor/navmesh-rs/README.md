# navmesh-rs

A pure-Rust port of [recastnavigation](https://github.com/recastnavigation/recastnavigation)'s
Recast (navmesh generation), Detour (navmesh runtime and queries), DetourCrowd
(agent simulation and local avoidance), and DetourTileCache (dynamic obstacles).

The port follows upstream function by function and is checked against it record by record: on
x86-64 Linux with default features, every Recast stage, the navmesh tile bytes and every Detour
query give the same bits as the C++ library at the pinned commit. Verification covers
76,018 corpus records, 9,200 tile-cache records and 158,400 crowd simulation ticks,
with 0 mismatches. See
[docs/VERIFICATION.md](docs/VERIFICATION.md) for how this is checked and what is covered.

## Install

```toml
[dependencies]
navmesh-rs = "0.1"
```

The library is `navmesh_rs`. Minimum Rust version: 1.88.

## Usage

```rust
use navmesh_rs::detour::{self, DefaultFilter, NavMesh, NavMeshCreateParams, NavMeshQuery};
use navmesh_rs::recast::{self, CompactHeightfield, ContourSet, Heightfield, PolyMesh, PolyMeshDetail};
use navmesh_rs::{Error, Workspace};

fn build_and_query(verts: &[f32], tris: &[i32]) -> Result<(), Error> {
    // Generation: one call per upstream stage (upstream demo settings).
    let mut ws = Workspace::new();
    let (bmin, bmax) = recast::calc_bounds(verts);
    let (w, h) = recast::calc_grid_size(&bmin, &bmax, 0.3);
    let mut hf = Heightfield::new(&mut ws, w, h, bmin, bmax, 0.3, 0.2)?;
    let mut areas = vec![0u8; tris.len() / 3];
    recast::mark_walkable_triangles(&mut ws, 45.0, verts, tris, &mut areas)?;
    hf.rasterize_triangles(&mut ws, verts, tris, &areas, 4)?;
    let mut chf = CompactHeightfield::build(&mut ws, 10, 4, &hf)?;
    recast::erode_walkable_area(&mut ws, 2, &mut chf)?;
    recast::build_distance_field(&mut ws, &mut chf)?;
    recast::build_regions(&mut ws, &mut chf, 0, 8, 20)?;
    let cset = ContourSet::build(&mut ws, &chf, 1.3, 40, recast::CONTOUR_TESS_WALL_EDGES)?;
    let pmesh = PolyMesh::build(&mut ws, &cset, 6)?;
    let dmesh = PolyMeshDetail::build(&mut ws, &pmesh, &chf, 1.8, 0.2)?;

    // Tile bytes in upstream's DNAV layout, loaded into a single-tile navmesh.
    let flags = vec![1u16; pmesh.npolys as usize];
    let tile = detour::create_nav_mesh_data(&NavMeshCreateParams {
        verts: &pmesh.verts,
        vert_count: pmesh.nverts,
        polys: &pmesh.polys,
        poly_flags: &flags,
        poly_areas: &pmesh.areas,
        poly_count: pmesh.npolys,
        nvp: pmesh.nvp,
        detail_meshes: &dmesh.meshes,
        detail_verts: &dmesh.verts,
        detail_verts_count: (dmesh.verts.len() / 3) as i32,
        detail_tris: &dmesh.tris,
        detail_tri_count: (dmesh.tris.len() / 4) as i32,
        bmin: pmesh.bmin,
        bmax: pmesh.bmax,
        walkable_height: 2.0,
        walkable_radius: 0.6,
        walkable_climb: 0.9,
        cs: 0.3,
        ch: 0.2,
        build_bv_tree: true,
        ..Default::default()
    })?;
    let (nav, _) = NavMesh::new_single(tile)?;

    // Queries.
    let mut query = NavMeshQuery::new(&nav, 2048)?;
    let filter = DefaultFilter::default();
    let ext = [2.0, 4.0, 2.0];
    let (a, pa, _) = query.find_nearest_poly(&[1.0, 0.0, 1.0], &ext, &filter)?;
    let (b, pb, _) = query.find_nearest_poly(&[9.0, 0.0, 9.0], &ext, &filter)?;
    let mut path = Vec::new();
    query.find_path(a, b, &pa, &pb, &filter, &mut path, 256)?;
    let mut corners = Vec::new();
    query.find_straight_path(&pa, &pb, &path, &mut corners, 64, 0)?;
    Ok(())
}
```

Tiled navmeshes use `NavMesh::new` with `NavMeshParams` and `add_tile` per tile. The crate
documentation lists every stage and query.

## Tiled baking

The `bake` module is an extension **not in recastnavigation**, available whenever
`recast` and `detour` are enabled. Enable `parallel` to opt into rayon tile workers:

```toml
navmesh-rs = { version = "0.1", features = ["parallel"] }
```

```rust
use navmesh_rs::{bake::{self, BakeConfig, Input}, Limits, Workspace};

let verts = [0., 0., 0., 0., 0., 24., 24., 0., 24., 24., 0., 0.];
let tris = [0, 1, 2, 0, 2, 3];
let input = Input::new(&verts, &tris);
let mut config = BakeConfig::default();
config.recast.tile_size = 32; // cells; other Recast settings are also in cell units
config.tile_limits.max_bytes = 64 << 20;
config.max_total_bytes = 2 << 30; // setup + retained tiles + concurrent workspaces
config.tile_limits.max_work = 100_000_000;
let mut setup = Workspace::new(); // optionally limit validation/bucketing separately
let baked = bake::tiles_parallel(&input, &config, &mut setup)?;
let nav = baked.into_navmesh(Limits::default())?;
# Ok::<(), bake::BakeError>(())
```

`tiles` always builds serially. `tiles_parallel` uses the current rayon pool with
`parallel`, or the serial implementation without it. Custom rayon pools control
thread counts. Both preserve triangle order and return row-major tiles, including
empty tiles; insertion into the navmesh stays serial. Errors identify the first
failing tile in that order. Input supports per-triangle areas, convex area volumes
and off-mesh connections. Limits apply independently and cumulatively per tile;
setup has its own workspace budget. Retained output and concurrent tiles add to
memory use, and rayon's scheduler allocations are outside these budgets.
`parallel` is off by default; leave it off for wasm and `no_std` builds.

### Large worlds: incremental, resumable, cancellable jobs

For a world too large to hold as one triangle soup, implement `bake::GeometrySource` (it
answers "what is in this box", in a fixed order) and drive a `bake::BakeJob`. Each tile
is baked from its own bordered column with the pipeline `tiles` uses, so the bytes are
identical. The caller persists a `BakeManifest` beside the tiles; after an edit,
`dirty_from_boxes(&config, &grid, &boxes)` names the tiles the edited boxes touch, the tiles
a larger grid added and, if the configuration or layout changed, every tile (`full_rebake`),
and `skip_unchanged` drops those whose input hash did not change. `retarget` moves the
manifest to the new grid and `apply` merges the job's `manifest_delta`, refusing a delta baked
under another configuration. Rebaking only the dirty tiles gives exactly the tiles and
manifest of a full bake. `step` works in slices of a `Budget`, `run_parallel` uses the rayon
pool, and a `CancelToken` stops a job without ever returning a partial tile. A navmesh fed
by `TileInstaller` replaces a resident tile with its rebake and keeps the newest queued
version of each tile; after a cancel, every tile is whole, new exactly where
`manifest_delta` lists it. For a large `Input`, `Input::indexed` builds the spatial index
once so each fetch costs what the tile holds, not what the mesh holds. Manifest digests are
not collision resistant: use the manifest as a cache for trusted content.

```rust
use navmesh_rs::bake::{BakeConfig, BakeJob, GridLayout, Input, JobOptions};
use navmesh_rs::{Budget, Progress};

let verts = [0., 0., 0., 0., 0., 24., 24., 0., 24., 24., 0., 0.];
let tris = [0, 1, 2, 0, 2, 3];
let source = Input::new(&verts, &tris); // any `GeometrySource` works here
let config = BakeConfig::default();
let grid = GridLayout::from_input(&source, &config)?; // a world states its own bounds
let mut job = BakeJob::new(&source, &config, &grid, grid.keys(), JobOptions::default())?;
while job.step(&mut Budget::new(50_000))? != Progress::Done {
    while let Some(tile) = job.next_ready() { /* hand `tile.data` to the navmesh */ }
}
while let Some(tile) = job.next_ready() { /* the last tiles */ }
# Ok::<(), navmesh_rs::bake::BakeError>(())
```

## Crowd

Enable the default `crowd` feature to manage agents over an existing navmesh.
Targets use polygon references from `NavMeshQuery::find_nearest_poly`; requests
borrow the mesh to validate refs. Agent speed must be strictly positive. Construct
non-exhaustive parameter structs with `Default::default()` (or an avoidance quality
preset), then assign individual fields.

```rust
use navmesh_rs::{Error, Workspace};
use navmesh_rs::crowd::{AgentParams, Crowd};
use navmesh_rs::detour::{DefaultFilter, NavMesh, NavMeshQuery};

fn simulate(nav: &NavMesh, start: [f32; 3], goal: [f32; 3]) -> Result<(), Error> {
    let mut ws = Workspace::new();
    let mut crowd = Crowd::new(&mut ws, nav, 64, 0.6)?;
    let agent = crowd.add_agent(&mut ws, nav, start, AgentParams::default())?;
    let query = NavMeshQuery::new(nav, 512)?;
    let (poly, target, _) = query.find_nearest_poly(&goal, &[2.0, 4.0, 2.0], &DefaultFilter::default())?;
    crowd.request_move_target(nav, agent, poly, target)?;
    for _ in 0..300 {
        crowd.update(&mut ws, nav, 1.0 / 30.0)?;
        let position = crowd.agent(agent).ok_or(Error::InvalidRef)?.npos;
        // Use position to drive the caller's presentation.
        let _ = position;
    }
    crowd.remove_agent(agent)?;
    Ok(())
}
```

`Crowd` provides target and velocity requests, target resets, sixteen query filters,
eight avoidance parameter slots, and read-only agent state. `PathCorridor`,
`LocalBoundary`, `ObstacleAvoidanceQuery`, `ProximityGrid`, and `PathQueue` can also
be used independently. Agent handles name reusable slots. The mesh is borrowed only
during calls, allowing tile changes between ticks. An update error may leave earlier
phases applied; do not consume that tick as a successful simulation result.

## Dynamic obstacles

The default `tilecache` feature adds `tilecache::TileCache`, validated `DTLR`
layer encoding/decoding, region/contour/polygon builders, cylinder/box/oriented-box
obstacles and incremental navmesh rebuilding. `RawCompressor` stores grids without
compression; applications can implement `LayerCompressor` for other codecs.

```rust
use navmesh_rs::{Limits, Error};
use navmesh_rs::tilecache::{TileCache, TileCacheParams, RawCompressor,
    DefaultMeshProcess, ObstacleShape};
use navmesh_rs::detour::NavMesh;

fn carve(cache: &mut TileCache, nav: &mut NavMesh) -> Result<(), Error> {
    let obstacle = cache.add_obstacle(ObstacleShape::Cylinder {
        pos: [4.0, 0.0, 4.0], radius: 0.8, height: 2.0,
    })?;
    while !cache.update(1.0 / 30.0, nav)? {}
    cache.remove_obstacle(obstacle)?;
    while !cache.update(1.0 / 30.0, nav)? {}
    Ok(())
}
let params = TileCacheParams::new(32, 32, 0.3, 0.2, 128, 128);
let cache = TileCache::new(params, RawCompressor, DefaultMeshProcess, Limits::default())?;
// Add encoded layers, then build_nav_mesh_tiles_at before carving.
# Ok::<(), Error>(())
```

`MeshProcess` maps polygon areas/flags and optionally supplies off-mesh links.
Handles are salted; stale references, full pools, more than eight touched layers,
malformed grids and exhausted work/byte limits return errors. Tile membership is
fixed while obstacle requests/rebuilds are pending. An update rebuilds at most one
layer; errors preserve that pending rebuild for retry. Completed earlier rebuilds
remain applied. Custom codecs and hooks are responsible for their own internal
resource use; they write into the cache's budgeted output slices.

## What it adds over the C++ library

- No C++ toolchain or bindings; builds for `wasm32` and for `no_std` targets (with `alloc`).
- Tile bytes from outside are validated before use: section sizes, every index and every
  reference are checked, and a malformed tile is rejected with a typed error instead of being
  read out of bounds. Upstream's own checks keep their order, so its verdicts are unchanged.
- Per-call limits on memory and work (`Limits`, `Workspace`); a call that exceeds them returns
  an error. Upstream asserts and crash paths become errors as well; the library does not panic
  on bad input and contains no `unsafe` code.
- Work budgets on every query (not in recastnavigation): a `Budget` bounds one call or one
  frame's worth of calls, budgeted forms return a typed `Budgeted::Exhausted` with the partial
  result, and the Dijkstra-style searches and the raycast can be resumed over several frames.
  Calls without a budget behave as upstream.
- With the `libm` feature, every recorded output (Recast stages, tile bytes, query results,
  crowd trajectories, tile-cache layers) is identical on x86-64 Linux, AArch64 Linux and wasm32;
  upstream cannot offer this (platform libm, `qsort` order, FMA contraction).
- Global off-mesh links (opt-in, `NavMesh::set_global_off_mesh_links`): an off-mesh connection
  can resolve its far endpoint in any loaded tile, not only the owner tile, its same-coordinate
  layers and its eight neighbours as upstream does. Tiles can be loaded in any order and
  removed and re-added; the links follow. Off by default, and then identical to upstream.
  Endpoints are snapped with upstream's rule: the candidate polygons are those whose bounds
  overlap a box of half extents (connection radius, walkable climb of the connection's owner
  tile, connection radius) around the authored endpoint; the nearest one wins, and the link is
  made only if the horizontal distance from the authored endpoint to the snapped point is at
  most the connection radius. There is no further vertical test; the polygon's height replaces
  the endpoint's.
- At bake time (`bake::tiles`) a triangle crossing a tile's bounds is given whole to every tile
  whose bounds plus border overlap it, and Recast's rasteriser clips it to that tile and its
  border.
- 64-bit poly refs by default (upstream defaults to 32-bit), for large tiled worlds. Turn off
  the `polyref64` feature for upstream's 32-bit tile format.

## Features

| Feature | Default | Purpose |
|---|---|---|
| `recast` | yes | navmesh generation |
| `detour` | yes | tile data, navmesh, queries |
| `tilecache` | yes | Cached layers, dynamic obstacles and incremental rebuilds; implies `recast` and `detour`. |
| `crowd` | yes | Agent simulation, corridors, avoidance and path queue; implies `detour`. |
| `std` | yes | floating-point functions from the platform libm |
| `libm` | no | floating-point functions from the `libm` crate (identical on every target); required without `std` |
| `parallel` | no | rayon tile workers; serial bake API remains available without it |
| `polyref64` | yes | 64-bit poly refs; off gives upstream's 32-bit refs |

`no_std`: `default-features = false, features = ["libm", "recast", "detour", "crowd", "tilecache", "polyref64"]`.

## Status and limits

- Ported: Recast, Detour, DetourCrowd and DetourTileCache.
- Also ported: merging and copying polygon meshes, tile state (polygon flags and areas) save
  and restore, and byte-order conversion of tile and layer data. The upstream API left out on
  purpose (allocators, logging, debug drawing, raw helpers) is listed in `docs/VERIFICATION.md`.
- Stage functions follow upstream; the `bake` extension provides serial and parallel tiled builds.
- Bit-exactness with the C++ library is checked on x86-64 Linux with glibc, with the default
  `std` feature. On other platforms the default build can differ in the last bit where upstream
  calls `cos`/`sin` (the walkable-slope threshold and avoidance sampling). With `libm` the
  results are identical on x86-64 Linux, AArch64 Linux and wasm32, but not bit-identical to
  the C++ library built on glibc. `docs/VERIFICATION.md` states the contract exactly.
- Speed: the query set and the Recast stages take about 1.1 to 1.35 times the instructions of
  the C++ library on the test corpus; no optimisation work has been done yet.

## Licence

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.

This crate is an altered, translated version of recastnavigation (Copyright (c) 2009 Mikko
Mononen and the recastnavigation contributors, zlib licence). It is not the original software;
the upstream licence is reproduced in [NOTICE](NOTICE).

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.

### Float imports, heuristic control and build diagnostics

These opt-in APIs are extensions not in recastnavigation. Existing entry points
retain upstream defaults.

`detour::create_nav_mesh_data_f32(&params, &world_vertices)` accepts existing
world-space polygon vertices. Set `vert_count`, polygon indices and adjacency,
flags, areas, bounds, and the other tile settings as for the quantized builder;
`params.verts` is ignored. DNAV vertex coordinates preserve the supplied f32 bits.
Bounds must contain every vertex. Only the optional BV tree bounds use a grid:
choose `cs` so the extent on every axis divided by `cs` fits 65,535, or disable
`build_bv_tree` for linear polygon scans. Import does not clip, weld, or infer
adjacency. The polygon/index limits of DNAV still apply.

`NavMeshQuery::find_path_with_heuristic_scale` takes the usual path arguments
followed by a finite, nonnegative scale. Use `0.0` for Dijkstra when a filter has
zero-cost links. For an admissible Euclidean heuristic, choose a scale no greater
than the minimum traversal cost per world-space distance. The ordinary
`find_path` keeps its upstream `0.999` scale. Path and nearest-polygon calls
accept both concrete filters and `dyn QueryFilter`; concrete filters can be
specialized by the compiler. For batches, reuse the query, filter, corridor Vec
and straight-path Vec across calls so their storage is retained.

`PolyMesh::build_with_report(workspace, contours, nvp, strict)` returns the mesh
and a `TriangulationReport` counting incomplete contours and missing triangles.
With `strict = true`, incomplete clipping returns `Error::PartialTriangulation`.
The ordinary builder keeps upstream's partial-output behavior. The tile bake
exposes the report on each `Tile::triangulation`; set
`BakeConfig::strict_triangulation` to reject partial output, with the tile location
carried by `BakeError`. Strict failures describe the first incomplete contour.

For large terrain use `bake::tiles_parallel` inside a rayon pool with 16 threads;
this includes triangle bucketing and ordered tile insertion when assembling the
result. A follow-up query corpus run reduced the mean from 36,998 to 35,829
instructions per route pipeline; C++ used 29,293. The per-stage 1.1x C++ goal
remains unmet. Performance measurements and qualification limits are recorded in
[`docs/VERIFICATION.md`](docs/VERIFICATION.md).

### Optional reciprocal crowd avoidance

Set `AgentParams::avoidance` to `AvoidanceMode::Orca(OrcaParams::default())` to
select reciprocal half-plane velocity planning. The sampled planner remains the
default. See [ORCA parameters, geometry and limits](docs/ORCA.md) for the pure
planar API, budgets, qualification commands and collision-guarantee assumptions.

The current ORCA candidate is [not qualified for integration](docs/ORCA-RESULTS.md);
its oracle, penetration and cost acceptance criteria remain unmet.
