//! One tile's bake, Recast's per-stage pipeline as the Recast demo's tiled build
//! (`Sample_TileMesh::buildTileMesh`, recastnavigation `9f4ce644`): slope marking, rasterising
//! the tile plus its border, the three span filters, erosion by the agent radius, area volumes,
//! watershed regions, contours, polygons, the detail mesh, then Detour tile bytes with polygon
//! flags from the adapter's [`AreaTable`] (not navmesh-rs's `bake` default `1 | 1 << area`).
//!
//! Settings not in [`BuildConfig`] are the demo's: edges ≤ 12 m, contour error 1.3 cells, regions
//! of ≥ 8² cells kept and < 20² merged, 6 vertices per polygon, detail samples every 6 cells
//! within one cell height. Where two triangles' spans merge, the higher area id wins (Recast).

#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    reason = "cell counts are small (tile_cells is u16) and agent sizes are meters"
)]

use navmesh_rs::detour::{self, NavMeshCreateParams};
use navmesh_rs::recast::{
    self, CompactHeightfield, ContourSet, Heightfield, PolyMesh, PolyMeshDetail,
};
use navmesh_rs::{Error, Workspace};
use sf_nav::{AreaId, AreaTable, BuildConfig, MAX_AREAS, NavError, TileInput};

const MAX_EDGE_LEN_M: f32 = 12.0;
const MAX_SIMPLIFICATION_ERROR: f32 = 1.3;
const MIN_REGION_AREA: i32 = 8 * 8;
const MERGE_REGION_AREA: i32 = 20 * 20;
const VERTS_PER_POLY: i32 = 6;
const DETAIL_SAMPLE_CELLS: f32 = 6.0;
const DETAIL_SAMPLE_MAX_ERROR_CELLS: f32 = 1.0;

fn stage(what: &'static str) -> impl Fn(Error) -> NavError {
    move |e| NavError::Backend(format!("tile bake, {what}: {e}"))
}

fn check_input(input: &TileInput, cfg: &BuildConfig) -> Result<(), NavError> {
    let bad = |r: String| Err(NavError::InvalidConfig(r));
    let c = &cfg.class;
    let ok = c.radius >= 0.0
        && c.height > 0.0
        && c.max_climb >= 0.0
        && (0.0..90.0).contains(&c.max_slope_deg);
    if !ok {
        return bad(format!(
            "agent class {:?}: bad radius, height, climb or slope",
            c.id
        ));
    }
    if input.tri_areas.len() != input.tris.len() {
        return bad(format!(
            "{} triangles but {} triangle areas",
            input.tris.len(),
            input.tri_areas.len()
        ));
    }
    if !input.verts.iter().all(|v| v.is_finite()) {
        return bad("a vertex is not finite".into());
    }
    let n = input.verts.len();
    if input.tris.iter().flatten().any(|&i| i as usize >= n) {
        return bad("a triangle index is out of range".into());
    }
    let areas = input
        .tri_areas
        .iter()
        .chain(input.volumes.iter().map(|v| &v.area));
    if let Some(a) = areas.into_iter().find(|a| a.index() >= MAX_AREAS) {
        return bad(format!("area id {} outside 0..{MAX_AREAS}", a.0));
    }
    Ok(())
}

/// Bakes the tile at (`input.tile_x`, `input.tile_z`) of a grid whose origin is the region's
/// (region-local meters). An empty `Vec` means the tile has no polygons: the bake stores none.
#[allow(
    clippy::too_many_lines,
    reason = "the Recast pipeline reads best as one sequence of stages"
)]
pub(crate) fn build_tile(
    input: &TileInput,
    cfg: &BuildConfig,
    areas: &AreaTable,
) -> Result<Vec<u8>, NavError> {
    cfg.validate()?;
    check_input(input, cfg)?;
    if input.tris.is_empty() {
        return Ok(Vec::new());
    }
    let (cs, ch, class) = (cfg.cell_size, cfg.cell_height, &cfg.class);
    let walkable_height = (class.height / ch).ceil() as i32;
    let walkable_climb = (class.max_climb / ch).floor() as i32;
    let walkable_radius = (class.radius / cs).ceil() as i32;
    let border = walkable_radius + 3;
    let cells = i32::from(cfg.tile_cells) + 2 * border;
    let size = cfg.tile_size();

    let verts: Vec<f32> = input.verts.iter().flat_map(glam::Vec3::to_array).collect();
    #[allow(
        clippy::cast_possible_wrap,
        reason = "indices were checked against the vertex count"
    )]
    let tris: Vec<i32> = input.tris.iter().flatten().map(|&i| i as i32).collect();
    let (lo, hi) = recast::calc_bounds(&verts);
    let pad = border as f32 * cs;
    let bmin = [
        input.tile_x as f32 * size - pad,
        lo[1],
        input.tile_z as f32 * size - pad,
    ];
    let bmax = [
        (input.tile_x + 1) as f32 * size + pad,
        hi[1],
        (input.tile_z + 1) as f32 * size + pad,
    ];

    let mut ws = Workspace::new();
    let mut tri_areas: Vec<u8> = input.tri_areas.iter().map(|a| a.0).collect();
    recast::clear_unwalkable_triangles(&mut ws, class.max_slope_deg, &verts, &tris, &mut tri_areas)
        .map_err(stage("slope"))?;
    let mut hf = Heightfield::new(&mut ws, cells, cells, bmin, bmax, cs, ch)
        .map_err(stage("heightfield"))?;
    hf.rasterize_triangles(&mut ws, &verts, &tris, &tri_areas, walkable_climb)
        .map_err(stage("rasterize"))?;
    hf.filter_low_hanging_walkable_obstacles(&mut ws, walkable_climb)
        .map_err(stage("low hanging filter"))?;
    hf.filter_ledge_spans(&mut ws, walkable_height, walkable_climb)
        .map_err(stage("ledge filter"))?;
    hf.filter_walkable_low_height_spans(&mut ws, walkable_height)
        .map_err(stage("low height filter"))?;
    let mut chf = CompactHeightfield::build(&mut ws, walkable_height, walkable_climb, &hf)
        .map_err(stage("compact"))?;
    drop(hf);
    recast::erode_walkable_area(&mut ws, walkable_radius, &mut chf).map_err(stage("erode"))?;
    for v in &input.volumes {
        let pv: Vec<f32> = v
            .footprint
            .iter()
            .flat_map(|p| [p.x, v.min_y, p.y])
            .collect();
        recast::mark_convex_poly_area(&mut ws, &pv, v.min_y, v.max_y, v.area.0, &mut chf)
            .map_err(stage("area volume"))?;
    }
    recast::build_distance_field(&mut ws, &mut chf).map_err(stage("distance field"))?;
    recast::build_regions(
        &mut ws,
        &mut chf,
        border,
        MIN_REGION_AREA,
        MERGE_REGION_AREA,
    )
    .map_err(stage("regions"))?;
    let contours = ContourSet::build(
        &mut ws,
        &chf,
        MAX_SIMPLIFICATION_ERROR,
        (MAX_EDGE_LEN_M / cs) as i32,
        recast::CONTOUR_TESS_WALL_EDGES,
    )
    .map_err(stage("contours"))?;
    let mesh = PolyMesh::build(&mut ws, &contours, VERTS_PER_POLY).map_err(stage("polygons"))?;
    if mesh.npolys == 0 {
        return Ok(Vec::new());
    }
    let detail = PolyMeshDetail::build(
        &mut ws,
        &mesh,
        &chf,
        cs * DETAIL_SAMPLE_CELLS,
        ch * DETAIL_SAMPLE_MAX_ERROR_CELLS,
    )
    .map_err(stage("detail mesh"))?;
    let flags: Vec<u16> = mesh
        .areas
        .iter()
        .map(|&a| areas.flags(AreaId(a)).bits())
        .collect();
    let to_i32 = |n: usize| i32::try_from(n).map_err(|_| stage("tile data")(Error::SizeOverflow));
    detour::create_nav_mesh_data(&NavMeshCreateParams {
        verts: &mesh.verts,
        vert_count: mesh.nverts,
        polys: &mesh.polys,
        poly_flags: &flags,
        poly_areas: &mesh.areas,
        poly_count: mesh.npolys,
        nvp: mesh.nvp,
        detail_meshes: &detail.meshes,
        detail_verts: &detail.verts,
        detail_verts_count: to_i32(detail.verts.len() / 3)?,
        detail_tris: &detail.tris,
        detail_tri_count: to_i32(detail.tris.len() / 4)?,
        tile_x: input.tile_x,
        tile_y: input.tile_z,
        tile_layer: 0,
        bmin: mesh.bmin,
        bmax: mesh.bmax,
        walkable_height: class.height,
        walkable_radius: class.radius,
        walkable_climb: class.max_climb,
        cs,
        ch,
        build_bv_tree: true,
        ..NavMeshCreateParams::default()
    })
    .map_err(stage("tile data"))
}
