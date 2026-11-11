//! A safe Rust port of [recastnavigation](https://github.com/recastnavigation/recastnavigation)'s
//! Recast (navmesh generation) and Detour (navmesh runtime and queries).
//!
//! This crate is an altered, translated version of recastnavigation (zlib licence,
//! Copyright (c) 2009 Mikko Mononen and the recastnavigation contributors) at
//! upstream commit `9f4ce64`; see the `NOTICE` file. With default features on
//! x86-64 Linux its output is bit-identical to the C++ library: every generation
//! stage, the navmesh tile bytes and every query result are compared against a
//! reference driver built from the pinned upstream sources (the repository's
//! `docs/VERIFICATION.md` describes how).
//!
//! Modules:
//! - [`recast`] (feature `recast`): navmesh generation, one function per upstream
//!   stage, from triangle rasterization to the detail mesh and heightfield layers.
//! - [`detour`] (feature `detour`): tile data in upstream's `DNAV` layout, the
//!   navmesh (validated tiles, links, poly refs: 64-bit with the default `polyref64`
//!   feature, 32-bit without) and every navmesh query.
//!
//! The `bake` extension (Recast + Detour) builds tiles serially or, with the
//! optional `parallel` dependency, on rayon workers; insertion stays ordered.
//!
//! Every stage takes a [`Workspace`], which carries the caller's [`Limits`] and
//! reports the [`Usage`] of the last call. Stages never panic; invalid input and
//! exhausted limits are reported as [`Error`].
//!
//! # What is supported
//!
//! - Recast: walkable-triangle marking, rasterization, the three span filters,
//!   the compact heightfield, erosion, median filter and area volumes (box,
//!   convex polygon, cylinder), the distance field, the three partitions
//!   (watershed, monotone, layers), contours, the polygon mesh, the detail mesh
//!   and heightfield layers.
//! - Detour: tile data creation ([`detour::create_nav_mesh_data`]), the navmesh
//!   with single or tiled layouts, off-mesh connections, a validating tile reader
//!   (malformed bytes are rejected with [`TileDataError`], never read out of
//!   bounds), and [`detour::NavMeshQuery`] with all of upstream's queries: nearest
//!   polygon, path finding (plain, sliced and partial), straight paths, raycasts,
//!   distance to wall, move along surface, polygon searches, wall segments,
//!   random points and height queries, with [`detour::DefaultFilter`] or a custom
//!   [`detour::QueryFilter`].
//! - Crowd (feature `crowd`): [`crowd::Crowd`], corridors, local walls, proximity
//!   grid, sliced path queue, steering, separation and sampled avoidance.
//! - `no_std` + `alloc`, `wasm32`, no `unsafe` code.
//!
//! - Tile cache (feature `tilecache`): validated layers, pluggable compression,
//!   three obstacle forms and incremental navmesh rebuilding.
//!
//! Not yet ported: merging and copying polygon
//! meshes, and big-endian tile data.
//!
//! # Features
//!
//! - `recast` (default): navmesh generation.
//! - `detour` (default): tile data, navmesh and queries.
//! - `crowd` (default): agent simulation and local avoidance; implies `detour`.
//! - `tilecache` (default): cached layers and dynamic obstacles; implies Recast and Detour.
//! - `std` (default): floating-point functions from the platform libm.
//! - `parallel`: rayon-backed tile baking (opt-in; bake functions remain available without it).
//! - `serde`: serialization of diagnostic reports (opt-in, supports alloc without std).
//! - `libm`: floating-point functions from the `libm` crate; required without `std`.
//! - `polyref64` (default): 64-bit poly refs (upstream's `DT_POLYREF64`); without it
//!   refs are 32-bit. The width is part of the tile byte format.
//!
//! # Example
//!
//! Build a navmesh from a flat square with Recast, load it in Detour and find a
//! path across it.
//!
//! ```
//! # #[cfg(all(feature = "recast", feature = "detour"))]
//! # {
//! use navmesh_rs::detour::{self, DefaultFilter, NavMesh, NavMeshCreateParams, NavMeshQuery};
//! use navmesh_rs::recast::{self, CompactHeightfield, ContourSet, Heightfield, PolyMesh, PolyMeshDetail};
//! use navmesh_rs::{Error, Workspace};
//!
//! fn main() -> Result<(), Error> {
//!     // A 10 x 10 floor made of two triangles.
//!     let verts = [0.0, 0.0, 0.0, 0.0, 0.0, 10.0, 10.0, 0.0, 10.0, 10.0, 0.0, 0.0];
//!     let tris = [0, 1, 2, 0, 2, 3];
//!
//!     // Recast, with upstream's demo settings: 0.3 cell size, 0.2 cell height,
//!     // agent height 2.0 (10 cells), climb 0.9 (4 cells), radius 0.6 (2 cells).
//!     let mut ws = Workspace::new();
//!     let (bmin, bmax) = recast::calc_bounds(&verts);
//!     let (w, h) = recast::calc_grid_size(&bmin, &bmax, 0.3);
//!     let mut hf = Heightfield::new(&mut ws, w, h, bmin, bmax, 0.3, 0.2)?;
//!     let mut areas = [0u8; 2];
//!     recast::mark_walkable_triangles(&mut ws, 45.0, &verts, &tris, &mut areas)?;
//!     hf.rasterize_triangles(&mut ws, &verts, &tris, &areas, 4)?;
//!     let mut chf = CompactHeightfield::build(&mut ws, 10, 4, &hf)?;
//!     recast::erode_walkable_area(&mut ws, 2, &mut chf)?;
//!     recast::build_distance_field(&mut ws, &mut chf)?;
//!     recast::build_regions(&mut ws, &mut chf, 0, 8, 20)?;
//!     let cset = ContourSet::build(&mut ws, &chf, 1.3, 40, recast::CONTOUR_TESS_WALL_EDGES)?;
//!     let pmesh = PolyMesh::build(&mut ws, &cset, 6)?;
//!     let dmesh = PolyMeshDetail::build(&mut ws, &pmesh, &chf, 1.8, 0.2)?;
//!
//!     // Detour tile bytes (upstream's DNAV layout), then a single-tile navmesh.
//!     let flags = vec![1u16; pmesh.npolys as usize];
//!     let tile = detour::create_nav_mesh_data(&NavMeshCreateParams {
//!         verts: &pmesh.verts,
//!         vert_count: pmesh.nverts,
//!         polys: &pmesh.polys,
//!         poly_flags: &flags,
//!         poly_areas: &pmesh.areas,
//!         poly_count: pmesh.npolys,
//!         nvp: pmesh.nvp,
//!         detail_meshes: &dmesh.meshes,
//!         detail_verts: &dmesh.verts,
//!         detail_verts_count: (dmesh.verts.len() / 3) as i32,
//!         detail_tris: &dmesh.tris,
//!         detail_tri_count: (dmesh.tris.len() / 4) as i32,
//!         bmin: pmesh.bmin,
//!         bmax: pmesh.bmax,
//!         walkable_height: 2.0,
//!         walkable_radius: 0.6,
//!         walkable_climb: 0.9,
//!         cs: 0.3,
//!         ch: 0.2,
//!         build_bv_tree: true,
//!         ..Default::default()
//!     })?;
//!     let (nav, _tile_ref) = NavMesh::new_single(tile)?;
//!
//!     // Queries: snap both ends to the mesh, find the corridor, then the corners.
//!     let mut query = NavMeshQuery::new(&nav, 512)?;
//!     let filter = DefaultFilter::default();
//!     let extents = [2.0, 4.0, 2.0];
//!     let (start, start_pos, _) = query.find_nearest_poly(&[2.0, 0.5, 2.0], &extents, &filter)?;
//!     let (end, end_pos, _) = query.find_nearest_poly(&[8.0, 0.5, 8.0], &extents, &filter)?;
//!     let mut path = Vec::new();
//!     query.find_path(start, end, &start_pos, &end_pos, &filter, &mut path, 64)?;
//!     let mut corners = Vec::new();
//!     query.find_straight_path(&start_pos, &end_pos, &path, &mut corners, 16, 0)?;
//!     assert_eq!(corners.len(), 2); // an open floor: a straight segment
//!     Ok(())
//! }
//! # main()?;
//! # }
//! # Ok::<(), navmesh_rs::Error>(())
//! ```

#![no_std]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

#[cfg(not(any(feature = "std", feature = "libm")))]
compile_error!("navmesh-rs needs the `std` or the `libm` feature for floating-point functions");

mod budget;
mod common;
mod fixes;
pub use budget::{Budget, Budgeted, Progress};
pub use common::{CancelToken, Error, Limits, TileDataError, TileStateError, Usage, Workspace};
pub use fixes::UpstreamFixes;

#[cfg(feature = "recast")]
pub mod recast;

#[cfg(feature = "detour")]
pub mod detour;

// Compiles the README's code blocks as doctests.
#[cfg(all(
    doctest,
    feature = "recast",
    feature = "detour",
    feature = "crowd",
    feature = "tilecache"
))]
#[doc = include_str!("../README.md")]
struct ReadmeDoctests;

#[cfg(feature = "crowd")]
pub mod crowd;

#[cfg(feature = "tilecache")]
pub mod tilecache;

#[cfg(all(feature = "recast", feature = "detour"))]
pub mod bake;

#[cfg(feature = "detour")]
pub mod connectivity;
