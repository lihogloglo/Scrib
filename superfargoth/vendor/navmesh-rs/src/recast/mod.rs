//! Navmesh generation: a port of upstream Recast, stage by stage.
//!
//! The pipeline, in upstream order: [`mark_walkable_triangles`] and
//! [`Heightfield::rasterize_triangles`], the three heightfield filters,
//! [`CompactHeightfield::build`], [`erode_walkable_area`] and the area marking
//! functions, a partition ([`build_distance_field`] + [`build_regions`],
//! [`build_regions_monotone`] or [`build_layer_regions`]), [`ContourSet::build`],
//! [`PolyMesh::build`] and [`PolyMeshDetail::build`]; [`HeightfieldLayerSet::build`]
//! produces the layers the tile cache stores. Each function matches its upstream
//! counterpart bit for bit on valid input; where upstream asserts, overflows a
//! bit field or reads out of bounds, the port returns an [`Error`].

// The port keeps upstream's index loops, fixed-stride vertex arrays, clamps and
// argument lists so each function reads line by line against the C++ source.
#![allow(
    clippy::needless_range_loop,
    clippy::chunks_exact_to_as_chunks,
    clippy::manual_clamp,
    clippy::too_many_arguments
)]

mod area;
mod compact;
mod contour;
mod detail;
mod heightfield;
mod layers;
mod mesh;
mod region;

pub use area::{
    erode_walkable_area, mark_box_area, mark_convex_poly_area, mark_cylinder_area,
    median_filter_walkable_area, offset_poly,
};
pub use compact::{CompactCell, CompactHeightfield, CompactSpan};
pub use contour::{CONTOUR_TESS_AREA_EDGES, CONTOUR_TESS_WALL_EDGES, Contour, ContourSet};
pub use detail::PolyMeshDetail;
pub use heightfield::{Heightfield, Span};
pub use layers::{HeightfieldLayer, HeightfieldLayerSet};
pub use mesh::{PolyMesh, TriangulationReport};
pub use region::{
    build_distance_field, build_layer_regions, build_regions, build_regions_monotone,
};

use crate::common::{cosf, v3, vcross, vmax, vmin, vnormalize, vsub};
use crate::{Error, Workspace};

/// Area id of walkable triangles and spans (`RC_WALKABLE_AREA`).
pub const WALKABLE_AREA: u8 = 63;
/// Area id of unwalkable spans (`RC_NULL_AREA`).
pub const NULL_AREA: u8 = 0;
/// Largest span height (`RC_SPAN_MAX_HEIGHT`, 13 bits).
pub const SPAN_MAX_HEIGHT: i32 = (1 << 13) - 1;
/// Neighbour slot value for "no connection" (`RC_NOT_CONNECTED`).
pub const NOT_CONNECTED: i32 = 0x3f;
/// Region id flag for border regions (`RC_BORDER_REG`).
pub const BORDER_REG: u16 = 0x8000;
/// Polygon mesh region flag for multiple regions (`RC_MULTIPLE_REGS`).
pub const MULTIPLE_REGS: u16 = 0;
/// Contour vertex flag for border vertices (`RC_BORDER_VERTEX`).
pub const BORDER_VERTEX: i32 = 0x10000;
/// Contour vertex flag for area borders (`RC_AREA_BORDER`).
pub const AREA_BORDER: i32 = 0x20000;
/// Mask of the region id in contour vertices (`RC_CONTOUR_REG_MASK`).
pub const CONTOUR_REG_MASK: i32 = 0xffff;
/// Index value for "no vertex / no neighbour" in polygon meshes (`RC_MESH_NULL_IDX`).
pub const MESH_NULL_IDX: u16 = 0xffff;

/// Build settings, field for field as upstream's `rcConfig`.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Config {
    /// Grid width in cells (along x).
    pub width: i32,
    /// Grid height in cells (along z).
    pub height: i32,
    /// Tile size in cells (tiled builds; 0 for solo).
    pub tile_size: i32,
    /// Non-navigable border around the tile, in cells.
    pub border_size: i32,
    /// Cell size on the xz plane, in world units.
    pub cs: f32,
    /// Cell height along y, in world units.
    pub ch: f32,
    /// Minimum bounds of the build.
    pub bmin: [f32; 3],
    /// Maximum bounds of the build.
    pub bmax: [f32; 3],
    /// Steepest walkable slope, in degrees.
    pub walkable_slope_angle: f32,
    /// Minimum floor-to-ceiling height, in cells.
    pub walkable_height: i32,
    /// Largest walkable ledge, in cells.
    pub walkable_climb: i32,
    /// Erosion radius, in cells.
    pub walkable_radius: i32,
    /// Longest contour edge, in cells.
    pub max_edge_len: i32,
    /// Largest distance a simplified contour may deviate from the raw one, in cells.
    pub max_simplification_error: f32,
    /// Smallest isolated region kept, in cells.
    pub min_region_area: i32,
    /// Regions smaller than this merge with larger ones when possible, in cells.
    pub merge_region_area: i32,
    /// Largest number of vertices per polygon (3..=6 for Detour).
    pub max_verts_per_poly: i32,
    /// Detail mesh sampling distance, in world units.
    pub detail_sample_dist: f32,
    /// Largest detail mesh height error, in world units.
    pub detail_sample_max_error: f32,
}

/// `rcGetDirOffsetX`.
#[inline(always)]
pub(crate) fn dir_offset_x(dir: i32) -> i32 {
    const OFFSET: [i32; 4] = [-1, 0, 1, 0];
    OFFSET[(dir & 3) as usize]
}

/// `rcGetDirOffsetY`.
#[inline(always)]
pub(crate) fn dir_offset_y(dir: i32) -> i32 {
    const OFFSET: [i32; 4] = [0, 1, 0, -1];
    OFFSET[(dir & 3) as usize]
}

/// `rcGetDirForOffset`.
#[inline(always)]
pub(crate) fn dir_for_offset(x: i32, y: i32) -> i32 {
    const DIRS: [i32; 5] = [3, 0, -1, 2, 1];
    DIRS.get((((y + 1) << 1) + x) as usize)
        .copied()
        .unwrap_or(-1)
}

/// Bounds of a vertex list (`rcCalcBounds`). Returns zeros for an empty list.
pub fn calc_bounds(verts: &[f32]) -> ([f32; 3], [f32; 3]) {
    if verts.len() < 3 {
        return ([0.0; 3], [0.0; 3]);
    }
    let mut bmin = v3(verts);
    let mut bmax = v3(verts);
    for v in verts.chunks_exact(3).skip(1) {
        vmin(&mut bmin, v);
        vmax(&mut bmax, v);
    }
    (bmin, bmax)
}

/// Grid size in cells for the given bounds (`rcCalcGridSize`).
pub fn calc_grid_size(bmin: &[f32; 3], bmax: &[f32; 3], cs: f32) -> (i32, i32) {
    (
        ((bmax[0] - bmin[0]) / cs + 0.5) as i32,
        ((bmax[2] - bmin[2]) / cs + 0.5) as i32,
    )
}

pub(crate) fn validate_mesh(verts: &[f32], tris: &[i32], areas_len: usize) -> Result<usize, Error> {
    if !verts.len().is_multiple_of(3) {
        return Err(Error::InvalidInput(
            "vertex array length is not a multiple of 3",
        ));
    }
    if !tris.len().is_multiple_of(3) {
        return Err(Error::InvalidInput(
            "index array length is not a multiple of 3",
        ));
    }
    let nt = tris.len() / 3;
    if areas_len != nt {
        return Err(Error::InvalidInput(
            "area array length differs from the triangle count",
        ));
    }
    let nv = verts.len() / 3;
    // Dense meshes reuse most vertices across triangles. A contiguous finite
    // scan plus an index-only scan avoids repeatedly gathering coordinates.
    // For sparse tile subsets keep the referenced-only path below. If unused
    // vertices are non-finite, fall back so their presence remains permitted.
    if tris.len() >= verts.len() && verts.iter().all(|v| v.is_finite()) {
        return if tris.iter().all(|&i| i >= 0 && (i as usize) < nv) {
            Ok(nt)
        } else {
            Err(Error::InvalidInput("triangle index out of range"))
        };
    }
    // Only the referenced vertices are checked, so a tile that uses a few
    // triangles of a large shared vertex array pays for its triangles only.
    for &i in tris {
        if i < 0 || i as usize >= nv {
            return Err(Error::InvalidInput("triangle index out of range"));
        }
        let v = &verts[i as usize * 3..i as usize * 3 + 3];
        if !(v[0].is_finite() && v[1].is_finite() && v[2].is_finite()) {
            return Err(Error::InvalidInput("non-finite vertex coordinate"));
        }
    }
    Ok(nt)
}

fn tri_normal(verts: &[f32], tri: &[i32]) -> [f32; 3] {
    let v0 = &verts[tri[0] as usize * 3..][..3];
    let v1 = &verts[tri[1] as usize * 3..][..3];
    let v2 = &verts[tri[2] as usize * 3..][..3];
    let e0 = vsub(v1, v0);
    let e1 = vsub(v2, v0);
    let mut n = vcross(&e0, &e1);
    vnormalize(&mut n);
    n
}

/// Sets the area of every triangle whose slope is walkable to [`WALKABLE_AREA`]
/// (`rcMarkWalkableTriangles`); other entries are left unchanged.
pub fn mark_walkable_triangles(
    ws: &mut Workspace,
    walkable_slope_angle: f32,
    verts: &[f32],
    tris: &[i32],
    areas: &mut [u8],
) -> Result<(), Error> {
    ws.begin();
    let nt = validate_mesh(verts, tris, areas.len())?;
    ws.work(nt as u64)?;
    let thr = cosf(walkable_slope_angle / 180.0 * core::f32::consts::PI);
    for (tri, area) in tris.chunks_exact(3).zip(areas.iter_mut()) {
        if tri_normal(verts, tri)[1] > thr {
            *area = WALKABLE_AREA;
        }
    }
    Ok(())
}

/// Sets the area of every triangle whose slope is not walkable to [`NULL_AREA`]
/// (`rcClearUnwalkableTriangles`).
pub fn clear_unwalkable_triangles(
    ws: &mut Workspace,
    walkable_slope_angle: f32,
    verts: &[f32],
    tris: &[i32],
    areas: &mut [u8],
) -> Result<(), Error> {
    ws.begin();
    let nt = validate_mesh(verts, tris, areas.len())?;
    ws.work(nt as u64)?;
    let thr = cosf(walkable_slope_angle / 180.0 * core::f32::consts::PI);
    for (tri, area) in tris.chunks_exact(3).zip(areas.iter_mut()) {
        if tri_normal(verts, tri)[1] <= thr {
            *area = NULL_AREA;
        }
    }
    Ok(())
}

#[cfg(test)]
mod mesh_validation_tests {
    use super::*;

    #[test]
    fn dense_validation_retains_referenced_vertex_rules() {
        let mut verts = [0.0; 9];
        let tris = [0, 1, 0, 1, 0, 1, 0, 1, 0];
        assert_eq!(validate_mesh(&verts, &tris, 3), Ok(3));
        verts[8] = f32::NAN; // Unused vertices remain allowed in the dense path.
        assert_eq!(validate_mesh(&verts, &tris, 3), Ok(3));
        verts[0] = f32::INFINITY;
        assert!(validate_mesh(&verts, &tris, 3).is_err());
        assert!(validate_mesh(&[0.0; 9], &[0, 1, 3, 0, 1, 0, 0, 1, 0], 3).is_err());
        assert!(validate_mesh(&[0.0; 9], &[0, 1, -1, 0, 1, 0, 0, 1, 0], 3).is_err());
    }
}
