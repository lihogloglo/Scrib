# NV0 spike: `navmesh-rs` on our geometry (2026-11-11)

Slice NV0 of [nav.md](../specs/nav.md) "Build plan" (ROADMAP 3.9). Research only: nothing in the workspace depends
on `navmesh-rs`. Machine: AMD Ryzen 7 7435HS (8 cores, 16 threads), Windows 11, MSVC toolchain, release builds.

## Verdict: **adopt** (pin `=0.2.2`, behind `NavBackend`; fork kept as a standby, not needed now)

- It bakes our real cooked collision (Census Office, Balmora, three wilderness blocks, all 1,328 interiors) with
  zero failures, serial and parallel bakes give identical bytes, and queries are in the low microseconds.
- Its own determinism suite passes on Windows: the port's record digests equal the upstream-parity golden files
  on all 8 public cases, with both the `std` and `libm` builds (3,116 records + 27 corrected records, 0 diffs).
- The code is clean (`#![forbid(unsafe_code)]`, no_std, no `panic!`, fuzz targets), MIT OR Apache-2.0 plus the
  zlib notice for the translated sources (both allowed by `deny.toml`). The risk is the project's age (one author,
  13 days old, 10 releases), handled by an exact pin, the backend trait and the fact that a fork is cheap.

The FFI fallback (NV0b) is **not needed**. If it ever is: recastnavigation at `9f4ce644` (zlib) built with `cc`
(the 2024 `recastnavigation-sys` 1.0.3 is stale), a C ABI shim for tile build and the query set below, and this
harness rerun against it.

## Method

- **Geometry** (in-workspace scratch example, no new deps): `bevyrrowind/crates/mw_bake/examples/nav_geometry_dump.rs`
  reads the last bake's `collision.cook` products (`sf_controller::CollisionMesh` from the `sf_bake` store via the
  profile manifest), places them with each placement's `WorldTransform` (static, dynamic; doors left out of the soup
  and written as boxes, nav.md "Door polygons"), adds LAND heights at the source 128-unit spacing for exterior
  blocks (`HeightSampler`), and writes a raw triangle soup. Blocks: Balmora 3×3 centred on cell (-3,-2) (the mean of
  the cells named "Balmora") and wilderness 3×3 blocks centred on (-6,-3), (2,6), (10,-2); every interior
  (`NV_INTERIORS=1`), `NV_BLOCKS="x,y;..."` for more blocks.
- **Harness** (outside the workspace, in the session scratch dir: no `Cargo.lock` or hakari change): a 300-line
  binary on crates.io `navmesh-rs = "=0.2.2"` + `parallel`. Per file: `bake::tiles` (serial) and
  `bake::tiles_parallel` (rayon pool), byte-compare, `into_navmesh`, 1,000 random point pairs
  (`find_random_point`), timing `find_nearest_poly`, `find_path` (4,096-node pool), `find_straight_path`,
  `raycast`; then a runtime flag patch. Door boxes become `AreaVolume`s (area 5). Batch mode bakes a directory.
- **Agent classes** (nav.md "Data model" calibration): humanoid r 0.3 m, h 1.8 m, climb 0.4 m, cs 0.15, ch 0.1,
  128-voxel tiles (19.2 m); large r 1.0 m, h 3.0 m, climb 0.6 m, cs 0.5, ch 0.2, 64-voxel tiles (32 m). Slope 45°,
  watershed, demo filters, max edge 12 m, detail sample 6·cs.
- **Parity**: the crate's `reference/determinism.sh` (no C++ oracle needed) run by hand on Windows against
  `reference/corpus/fetch.sh --public-only`, plus the crate's own `cargo test --release`.

## Measurements

Times: serial = one thread, CPU; parallel = wall. Bytes = Detour DNAV tile bytes (uncompressed); resident =
decoded tile structures + the kept byte buffer. Query times: median (p99, max) over 1,000 random pairs inside the
block (paths up to ~350 m, i.e. longer than typical NPC legs).

| Input | Class | Tris | Tiles (non-empty) | Serial ms/tile | Wall x8 / x16 | DNAV MiB | Resident MiB | Polys | nearest µs | find_path µs | raycast µs |
|---|---|---|---|---|---|---|---|---|---|---|---|
| Census Office | humanoid | 19.5k | 3 | 4.5 | 6 ms / – | 0.01 | 0.02 | 54 | 1.2 (2.0) | 1.5 (3.2, 4.9) | 0.6 (1.5) |
| Census Office | large | 19.5k | 1 | 6.4 | 5 ms / – | <0.01 | <0.01 | 3 | 0.4 | 0.1 | 0.3 |
| Balmora 3×3 | humanoid | 140k | 400 | 6.5-9.3 | 493 / 258 ms | 2.64 | 5.10 | 11,933 | 4.8 (14.4) | 250 (1,447, 6,104) | 2.0 (12.4) |
| Balmora 3×3 | large | 140k | 142 | 2.8 | 63 ms / – | 0.59 | 1.13 | 2,946 | 1.7 (4.3) | 9.7 (193, 828) | 0.5 (3.1) |
| Wild (-6,-3) | humanoid | 112k | 365 | 7.4-7.6 | 441 / 348 ms | 2.11 | 4.09 | 9,445 | 1.7 (4.7) | 55 (668, 773) | 0.7 (4.7) |
| Wild (2,6) | humanoid | 119k | 360 | 4.3 | 290 ms / – | 2.02 | 3.90 | 9,171 | 1.9 (5.1) | 16 (574, 778) | 0.4 (3.2) |
| Wild (10,-2) | humanoid | 125k | 361 | 6.5 | 400 ms / – | 1.90 | 3.68 | 8,217 | 1.9 (4.8) | 75 (625, 787) | 0.6 (3.1) |
| Wild blocks | large | 112-125k | 134-144 | 1.5-1.6 | 34-43 ms / – | 0.35-0.56 | 0.67-1.08 | 1.7-2.7k | 0.7-0.9 | 1-34 (≤ 388 max) | 0.2-0.4 |
| All 1,328 interiors | humanoid | 11.5M | 8,443 | 5.8-6.3 | 6.4 / 3.5 s | 43.3 | ~85 (est.) | – | – | – | – |
| All 1,328 interiors | large | 11.5M | 2,909 | 3.0 | 1.2 s / – | 5.0 | ~10 (est.) | – | – | – | – |

Interiors: per interior median 18 ms, p99 240-280 ms, max 0.7-0.8 s (humanoid). `find_straight_path` 0.4-11 µs
median. Navmesh load (`add_tile` of a whole block) 1-8 ms. Serial runs vary ±30 % with other builds on the machine.

**Full bake extrapolation** (1,540 LAND cells, 1,328 interiors): exterior humanoid ≈ 270 ms CPU per cell (mean of
the four blocks; Balmora is the densest at 290-415) → ~420 CPU-s, ~46 s wall at 16 threads (measured speedup
8-10×); exterior large ≈ 29 ms per cell → ~45 CPU-s, ~5 s; interiors 3.5 s + ~0.7 s. **Total ≈ 520 CPU-s,
≈ 55 s wall at 16 threads (≈ 75 s at 8)**, over the 45 s navmesh slot (bake.md "Budget") by ~20 %. Disk:
humanoid ≈ 0.24 MiB DNAV per exterior cell → ~370 MiB + 43 MiB interiors + ~95 MiB large ≈ 510 MiB before LZ4.

Ways back under the slot, for NV4 to measure (not decided here): exterior humanoid cs 0.2 (voxels ×0.56, ~-20 s,
the nav.md "Bake" gate's own remedy); drop seabed terrain below water level minus agent height for non-swimming
classes (much of the exterior grid is sea); a coarser detail mesh (`detail_sample_dist`) also cuts bytes.

## Checks from the brief

- **Version**: 0.2.2 (2026-11-06) is still the latest; `main` has 7 unreleased commits (crowd world store, shared
  goal fields, ORCA mode) we don't need.
- **Parity on our platform**: upstream bit-exactness is claimed only for x86-64 Linux glibc against a C++ oracle
  built with GCC flags (`-ffp-contract=off`), which we did not rebuild (needs clang-cl/GCC flags; MSVC BuildTools
  has Llvm, so possible later). What we ran: the oracle-free determinism suite, whose golden digests equal the
  Linux oracle records on these cases; Windows `std` and `libm` builds both match it exactly, so on this corpus
  Windows output equals upstream's. Platform variance is confined to `cos`/`sin` (walkable slope, avoidance
  sampling); the `libm` feature makes output identical across platforms: **use `libm`** for host-authoritative
  co-op and reproducible bakes. Not covered: the 13 generated-mesh cases (need the oracle's generator).
- **Crate tests on Windows**: 200 passed, 0 failed ("Crate test suite").
- **Tile format**: DNAV magic `56 41 4E 44` + version 7, upstream `dtMeshHeader` layout, bytes equal to upstream on
  their corpus. Ref width is part of the format: the crate defaults to 64-bit refs (`polyref64`, upstream's
  `DT_POLYREF64`); C++ Detour would need the same define to load our tiles.
- **Areas and flags**: per-triangle areas (`Input::areas`), convex area volumes (`AreaVolume`, applied after
  erosion: 70 door boxes in Balmora gave 71 door-area polys), off-mesh connections (`Input::off_mesh`), runtime
  `NavMesh::set_poly_flags` / `set_poly_area` (checked: an excluded poly drops out of `find_nearest_poly`), plus
  `store_tile_state` / `restore_tile_state`. The `bake` module hard-codes flags as `1 | 1 << area` (area < 16):
  workable, but NV2 should set flags from our area table (patch after `add_tile`, or build tiles through the
  per-stage API + `create_nav_mesh_data`, which takes `poly_flags`).
- **TileCache, streaming**: TileCache (obstacles over cached layers, raw codec only: bring LZ4) and a streaming API
  (`prepare_tile`/commit, salts, `remove_tile`) exist; `bake::BakeJob` + `GeometrySource` + `BakeManifest` overlap
  `sf_bake`: NV4 should use per-tile baking from a `GeometrySource` and let `sf_bake` own caching.
- **Code**: 24.5k lines, `#![no_std]`, `#![forbid(unsafe_code)]`, 0 `panic!`/`unreachable!`, 63 `unwrap()`
  (integer `try_from`s and tests), 24 `assert!`, errors as `Result`, byte/work `Limits` per tile, 10 fuzz targets.
  Deps: `sha2` (soft), optional `rayon`, `libm`, `serde`. MSRV 1.88, edition 2024.
- **Licence**: MIT OR Apache-2.0 for the port, zlib for the translated recastnavigation sources (`NOTICE`); both in
  `deny.toml`. NV2 adds a `THIRD_PARTY_NOTICES.md` entry carrying the zlib notice.
- **Alternatives re-checked** (crates.io, 2026-11-11): `rerecast` 0.5.0 / `bevy_rerecast` 0.6 (Bevy 0.20 now)
  still generation-only, queries via `landmass` (0.9.3) or `vleue_navigator` 0.17 / `polyanya` 0.18 (2D/layered);
  `oxidized_navigation` unchanged since 2024; `recastnavigation-sys` 1.0.3 from 2024. New: `sylphx-keel-nav`
  (2026-10-29, an engine-internal crate with auto-incremented versions; not a fit). Bevy 0.20 has no navmesh.

## Risks

1. **Bus factor and churn** (one author, 0.1 → 0.2 in a week). Mitigation: exact pin, all use behind
   `NavBackend` (`sf_nav_recast` only), upgrades in the batched "update all" pass with this harness and the
   determinism suite rerun; a fork is pure Rust and verifiable against upstream with the crate's own harness.
2. **Bake time over the slot** by ~20 % at the current calibration (above); NV4 measures the remedies.
3. **Long `find_path`**: up to 6 ms for a 350 m path through Balmora with a 4,096-node pool: NV6 must use the sliced
   API with an iteration budget, and NV12's coarse graph bounds leg length.
4. **Memory doubling**: a resident `MeshTile` keeps the decoded arrays and the original bytes (~1.9× DNAV).
   Fine for the budget (a 5×5-cell humanoid ring ≈ 14 MiB vs 64 MiB Floor).
5. **Windows vs upstream** is shown on 8 public cases, not the full corpus; `libm` removes the question for our
   own determinism.

## What NV1/NV2 must know

- NV1 (`sf_nav`, no backend): nothing blocks; keep `PolyRef` opaque 64-bit.
- NV2 (`sf_nav_recast`): dependency `navmesh-rs = { version = "=0.2.2", default-features = false, features =
  ["libm", "recast", "detour", "polyref64"] }` (+ `parallel` in the bake crate; `tilecache` at NV10; skip
  `crowd`: we keep our ORCA and take `PathCorridor`/`LocalBoundary`, check they are reachable without the crowd
  manager); `cargo hakari generate`. Flags from our area table (above). Rebuild the dump example's soup as the
  NV2/NV4 `NavGeometrySource` (same transform: collision products × placement `WorldTransform`, LAND heights).
- Doors: a door's box padded 0.2 m does not always cover its floor (Census: 6 doors → 2 door polys; load doors set
  into walls); NV4 needs a real doorway footprint (frame + floor probe, `mw_world::portal`).
- Terrain: the dump used LAND at source spacing; NV4 must take the heights the player's collider uses.

## Crate test suite

`cargo test --release -p navmesh-rs` at `v0.2.2` on Windows (MSVC): 15 test binaries, 200 passed, 0 failed, 0 ignored.

## Reproduce

`NV_BLOCKS="-6,-3;2,6;10,-2" NV_INTERIORS=1 cargo run --release -p mw_bake --example nav_geometry_dump -- <out>`;
harness source and the clone live in the session scratch dir only (not kept): rebuild from "Method" if needed.
