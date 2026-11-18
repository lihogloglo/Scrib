//! Headless tests of `worldgen-view`: mesh building on a small fixture, LOD choice, arm list parsing, and the
//! WG-V2 scale references (marker mapping, deterministic trees, contour formula, tint).

#![allow(
    clippy::float_cmp,
    clippy::many_single_char_names,
    reason = "exact on purpose: shared edge samples are copies, not recomputed; x, z, t, p are the geometry's names"
)]

use std::path::PathBuf;

use super::arms;
use super::mesh::{self, ChunkRect, LOD_STRIDES, Terrain, Tint};
use super::refs::{self, Contours, TreeParams};
use super::pick_lod;

/// A 65 × 49 fixture: a ridge plus waves, a corner under the sea, 20 m spacing.
fn fixture() -> Terrain {
    let (w, h) = (65, 49);
    let heights = (0..h)
        .flat_map(|r| {
            (0..w).map(move |c| {
                let (x, z) = (f32::from(u16::try_from(c).unwrap()), f32::from(u16::try_from(r).unwrap()));
                300.0 - (x - 30.0).abs() * 8.0 + (z * 0.4).sin() * 25.0 + (x * 0.7).cos() * 10.0 - 120.0
            })
        })
        .collect();
    Terrain::new(w, h, 20.0, 0.0, heights)
}

#[test]
fn chunks_cover_the_grid_and_share_edges() {
    let t = fixture();
    let rects = mesh::chunks(t.w, t.h, 16);
    // 64 quads / 16 = 4 columns; 48 / 16 = 3 rows.
    assert_eq!(rects.len(), 12);
    assert_eq!(rects[0], ChunkRect { c0: 0, r0: 0, c1: 16, r1: 16 });
    assert_eq!(rects[1].c0, rects[0].c1, "neighbours share the edge column");
    assert_eq!(rects[4].r0, rects[0].r1, "neighbours share the edge row");
    assert_eq!(rects.last().unwrap().c1, t.w - 1);
    assert_eq!(rects.last().unwrap().r1, t.h - 1);
    // A partial last chunk when the size doesn't divide the grid.
    let odd = mesh::chunks(t.w, t.h, 48);
    assert_eq!(odd.len(), 2);
    assert_eq!(odd[1], ChunkRect { c0: 48, r0: 0, c1: 64, r1: 48 });
}

#[test]
fn axis_always_ends_on_the_last_sample() {
    assert_eq!(mesh::axis(0, 16, 4), vec![0, 4, 8, 12, 16]);
    assert_eq!(mesh::axis(48, 64, 16), vec![48, 64]);
    assert_eq!(mesh::axis(0, 10, 4), vec![0, 4, 8, 10]);
}

#[test]
fn chunk_mesh_counts_normals_and_winding() {
    let t = fixture();
    let rect = ChunkRect { c0: 16, r0: 16, c1: 32, r1: 32 };
    for &stride in &LOD_STRIDES {
        let n = 16 / stride + 1;
        let m = mesh::build_chunk(&t, rect, stride, 0.0);
        assert_eq!(m.positions.len(), n * n, "stride {stride}");
        assert_eq!(m.grid_vertices, n * n);
        assert_eq!(m.indices.len(), (n - 1) * (n - 1) * 6);
        assert_eq!(m.normals.len(), m.positions.len());
        assert_eq!(m.colors.len(), m.positions.len());
        for nrm in &m.normals {
            let len = (nrm[0] * nrm[0] + nrm[1] * nrm[1] + nrm[2] * nrm[2]).sqrt();
            assert!(nrm.iter().all(|v| v.is_finite()) && (len - 1.0).abs() < 1e-4, "normal {nrm:?}");
            assert!(nrm[1] > 0.0, "normals point up");
        }
        for c in &m.colors {
            assert!(c.iter().all(|v| v.is_finite() && (0.0..=1.0).contains(v)));
        }
        for tri in m.indices.chunks(3) {
            assert!(m.face_normal([tri[0], tri[1], tri[2]])[1] > 0.0, "surface triangles face up (counter-clockwise)");
        }
    }
    // Local frame: the first vertex is the chunk's origin sample.
    let m = mesh::build_chunk(&t, rect, 1, 0.0);
    assert_eq!(m.positions[0], [0.0, t.heights[16 * t.w + 16], 0.0]);
    assert_eq!(m.positions[16], [320.0, t.heights[16 * t.w + 32], 0.0]);
}

#[test]
fn neighbouring_chunks_share_edge_heights_at_every_lod() {
    let t = fixture();
    let rects = mesh::chunks(t.w, t.h, 16);
    let (west, east) = (rects[0], rects[1]);
    for &stride in &LOD_STRIDES {
        let a = mesh::build_chunk(&t, west, stride, 0.0);
        let b = mesh::build_chunk(&t, east, stride, 0.0);
        let n = 16 / stride + 1;
        for j in 0..n {
            let (pa, pb) = (a.positions[j * n + n - 1], b.positions[j * n]);
            assert_eq!(pa[1], pb[1], "height on the shared edge, stride {stride}, row {j}");
            assert_eq!(pa[2], pb[2]);
            assert_eq!(a.normals[j * n + n - 1], b.normals[j * n], "normals match across the seam");
            assert!((pa[0] - 320.0).abs() < 1e-3 && pb[0].abs() < 1e-6);
        }
    }
}

#[test]
fn skirts_hang_below_and_face_outward() {
    let t = fixture();
    let rect = ChunkRect { c0: 16, r0: 16, c1: 32, r1: 32 };
    let depths = mesh::skirt_depths(&t, rect, 2.0);
    assert!(depths.iter().all(|d| d.is_finite() && *d >= 2.0));
    assert_eq!(mesh::edge_error(&t, rect, 1), 0.0, "full resolution matches itself");
    assert!(mesh::edge_error(&t, rect, 16) > 0.0, "the fixture's waves don't fit a 16-sample edge");
    let m = mesh::build_chunk(&t, rect, 4, depths[1]);
    let n = 5;
    assert_eq!(m.positions.len(), n * n + 4 * n);
    assert_eq!(m.indices.len(), (n - 1) * (n - 1) * 6 + 4 * (n - 1) * 6);
    let grid_min = m.positions[..m.grid_vertices].iter().map(|p| p[1]).fold(f32::INFINITY, f32::min);
    assert!(m.min[1] <= grid_min - depths[1] + 1e-3, "bounds include the skirts");
    // The north skirt (z = 0) faces -z, the east one (x = 320) +x.
    for tri in m.indices[(n - 1) * (n - 1) * 6..].chunks(3) {
        let fnrm = m.face_normal([tri[0], tri[1], tri[2]]);
        let p = m.positions[tri[0] as usize];
        let centre = [160.0, 160.0];
        let out = [p[0] - centre[0], p[2] - centre[1]];
        assert!(fnrm[0] * out[0] + fnrm[2] * out[1] > 0.0, "skirt triangle faces outward");
    }
}

#[test]
fn heights_peak_and_wet_chunks() {
    let t = fixture();
    let (c, r, h) = t.peak();
    assert_eq!(t.heights[r * t.w + c], h);
    assert_eq!(h, t.max_h);
    assert_eq!(t.height_at(0.0, 0.0), Some(t.heights[0]));
    let mid = t.height_at(10.0, 0.0).unwrap();
    assert!((mid - f32::midpoint(t.heights[0], t.heights[1])).abs() < 1e-3, "bilinear");
    assert_eq!(t.height_at(-1.0, 0.0), None);
    assert_eq!(t.height_at(0.0, 48.0 * 20.0 + 1.0), None);
    let sunk = Terrain::new(5, 5, 10.0, 100.0, vec![50.0; 25]);
    assert!(mesh::chunk_wet(&sunk, ChunkRect { c0: 0, r0: 0, c1: 4, r1: 4 }));
    assert!(!mesh::chunk_wet(&t, ChunkRect { c0: 16, r0: 16, c1: 32, r1: 32 }));
}

#[test]
fn lod_choice_by_distance_with_hysteresis() {
    let lod_m = [1000.0, 4000.0];
    let all = [true; 3];
    assert_eq!(pick_lod(10.0, None, lod_m, all), Some(0));
    assert_eq!(pick_lod(2000.0, None, lod_m, all), Some(1));
    assert_eq!(pick_lod(9000.0, None, lod_m, all), Some(2));
    // Just past a switch distance keeps the shown LOD; well past it switches.
    assert_eq!(pick_lod(1050.0, Some(0), lod_m, all), Some(0));
    assert_eq!(pick_lod(1200.0, Some(0), lod_m, all), Some(1));
    assert_eq!(pick_lod(950.0, Some(1), lod_m, all), Some(1));
    // Not built yet: the nearest built one, coarser first.
    assert_eq!(pick_lod(10.0, None, lod_m, [false, false, true]), Some(2));
    assert_eq!(pick_lod(10.0, None, lod_m, [false, true, true]), Some(1));
    assert_eq!(pick_lod(9000.0, None, lod_m, [true, false, false]), Some(0));
    assert_eq!(pick_lod(10.0, None, lod_m, [false; 3]), None);
}

/// A scratch folder under the system temp dir, removed on drop.
struct Scratch(PathBuf);

impl Scratch {
    fn new(name: &str) -> Self {
        let dir = std::env::temp_dir().join(format!("wgview-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        Self(dir)
    }

    fn guide(&self, name: &str) -> PathBuf {
        let d = self.0.join(name);
        std::fs::create_dir_all(&d).unwrap();
        std::fs::write(d.join("meta.json"), "{}").unwrap();
        d
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[test]
fn arm_list_from_guides_and_parent_folders() {
    let s = Scratch::new("arms");
    let a1 = s.guide("A1");
    let a0 = s.guide("A0");
    std::fs::create_dir_all(s.0.join("notes")).unwrap();
    // Given one by one: in the given order, named by folder.
    let arms = arms::parse(&[a1.clone(), a0.clone()]).unwrap();
    assert_eq!(arms.iter().map(|a| a.name.as_str()).collect::<Vec<_>>(), ["A1", "A0"]);
    assert_eq!(arms[0].dir, a1);
    // A parent folder: its guide subfolders, sorted; folders without meta.json are skipped.
    let arms = arms::parse(std::slice::from_ref(&s.0)).unwrap();
    assert_eq!(arms.iter().map(|a| a.name.as_str()).collect::<Vec<_>>(), ["A0", "A1"]);
    // Errors: nothing, a folder with no guide, too many arms.
    assert!(arms::parse(&[]).is_err());
    assert!(arms::parse(&[s.0.join("notes")]).is_err());
    let ten: Vec<PathBuf> = (0..10).map(|_| a0.clone()).collect();
    assert!(arms::parse(&ten).unwrap_err().contains("at most 9"));
}

/// Build cost of a real guide (`WGVIEW_GUIDE=<guide folder>`): chunks, vertices, triangles and mesh bytes per
/// LOD, and the build time on one thread.
#[test]
#[ignore = "measurement: needs WGVIEW_GUIDE, prints the build cost of a real guide"]
fn measure_real_guide() {
    let Ok(dir) = std::env::var("WGVIEW_GUIDE") else { return };
    let g = sf_worldgen::guide::Guide::read(std::path::Path::new(&dir)).unwrap();
    #[allow(clippy::cast_possible_truncation, reason = "metres")]
    let t = Terrain::new(g.w, g.h, g.spacing_m as f32, g.sea_level_m as f32, g.heights);
    let dry: Vec<ChunkRect> = mesh::chunks(t.w, t.h, 256).into_iter().filter(|r| !mesh::chunk_wet(&t, *r)).collect();
    println!("{}x{}: {} land chunks of {}", t.w, t.h, dry.len(), mesh::chunks(t.w, t.h, 256).len());
    for (lod, &stride) in LOD_STRIDES.iter().enumerate() {
        let started = std::time::Instant::now();
        let (mut verts, mut tris) = (0, 0);
        for r in &dry {
            let m = mesh::build_chunk(&t, *r, stride, mesh::skirt_depths(&t, *r, 2.0)[lod]);
            verts += m.positions.len();
            tris += m.indices.len() / 3;
        }
        println!(
            "LOD {lod} (stride {stride}): {verts} vertices, {tris} triangles, {} MB, built in {:.2} s",
            (verts * 40 + tris * 12) >> 20,
            started.elapsed().as_secs_f32()
        );
    }
}

// ---------------------------------------------------------------------------------------------------------------
// WG-V2 scale references.

/// The S2/S2b macro grid's `meta.json` origin (arm frame = guide frame x 10).
const S2_ORIGIN: [f64; 2] = [-20_209.008, 32_699.712];

/// A flat 300 x 400 grid at 100 m whose sample (0, 0) is at [`S2_ORIGIN`]: 30 x 40 km, ground at 50 m.
fn flat_arm() -> Terrain {
    Terrain::new(300, 400, 100.0, 0.0, vec![50.0; 300 * 400])
}

#[allow(clippy::cast_precision_loss, reason = "a few cells")]
fn marker(name: &str, cells: &[[i32; 2]]) -> mw_worldgen::Marker {
    let n = cells.len() as f64;
    let centres: Vec<[f64; 2]> = cells.iter().map(|&[x, y]| mw_worldgen::markers::cell_centre_m((x, y))).collect();
    mw_worldgen::Marker {
        name: name.into(),
        cells: cells.to_vec(),
        centroid_m: [centres.iter().map(|c| c[0]).sum::<f64>() / n, centres.iter().map(|c| c[1]).sum::<f64>() / n],
        // Not read by the viewer.
        min_m: [0.0; 2],
        max_m: [0.0; 2],
    }
}

#[test]
#[allow(clippy::cast_possible_truncation, reason = "metres")]
fn markers_map_to_scaled_positions_at_true_size() {
    // The S2b README's red-mountain crop: guide (422.5, 1055.2) m is world (4225, 10552) m at S = 10.
    let (x, z) = refs::source_to_local([422.5, 1055.2], 10.0, S2_ORIGIN);
    assert!((f64::from(x) - (4225.0 - S2_ORIGIN[0])).abs() < 1e-2, "{x}");
    assert!((f64::from(z) - (S2_ORIGIN[1] - 10_552.0)).abs() < 1e-2, "{z}");
    // Sample (0, 0) is the origin; north (+y) is -z.
    assert_eq!(refs::source_to_local([S2_ORIGIN[0] / 10.0, S2_ORIGIN[1] / 10.0], 10.0, S2_ORIGIN), (0.0, 0.0));

    let t = flat_arm();
    let cell = mw_worldgen::markers::cell_size_m();
    // Two cells side by side east-west, then one north of the second.
    let m = marker("Balmora", &[[-3, -2], [-2, -2], [-2, -1]]);
    let p = refs::place_marker(&m, &t, 10.0, S2_ORIGIN, cell, 15.0).unwrap();
    assert_eq!((p.name.as_str(), p.cells, p.blocks.len()), ("Balmora", 3, 3));
    let want = refs::source_to_local(m.centroid_m, 10.0, S2_ORIGIN);
    assert_eq!(p.at, want);
    let half = (cell / 2.0) as f32;
    for b in &p.blocks {
        assert!((b.half - half).abs() < 1e-4);
        assert_eq!((b.base, b.top), (49.0, 65.0));
    }
    // 1:1 spacing between neighbouring cells (not x 10), north = -z.
    let (a, b, c) = (p.blocks[0].centre, p.blocks[1].centre, p.blocks[2].centre);
    let cell_f = cell as f32;
    assert!(((b.0 - a.0) - cell_f).abs() < 1e-2 && (b.1 - a.1).abs() < 1e-2, "{a:?} {b:?}");
    assert!(((b.1 - c.1) - cell_f).abs() < 1e-2 && (c.0 - b.0).abs() < 1e-2, "{b:?} {c:?}");
    // The blocks sit around the scaled centroid.
    let mean = ((a.0 + b.0 + c.0) / 3.0, (a.1 + b.1 + c.1) / 3.0);
    assert!((mean.0 - want.0).abs() < 1e-2 && (mean.1 - want.1).abs() < 1e-2);
    assert_eq!(p.top, 65.0);

    // Off the grid (far east) and under the sea: not placed.
    assert!(refs::place_marker(&marker("Far", &[[200, 0]]), &t, 10.0, S2_ORIGIN, cell, 15.0).is_none());
    let wet = Terrain::new(300, 400, 100.0, 0.0, vec![-5.0; 300 * 400]);
    assert!(refs::place_marker(&m, &wet, 10.0, S2_ORIGIN, cell, 15.0).is_none());

    // The summit pole and the nearest marker.
    let top = mw_worldgen::SummitMarker { x_m: 422.5, y_m: 1055.2, height_m: 270.8, cell: [3, 9] };
    let pole = refs::place_summit(&top, &t, 10.0, S2_ORIGIN, 2.0, 60.0).unwrap();
    assert_eq!((pole.at, pole.blocks[0].top), (refs::source_to_local([422.5, 1055.2], 10.0, S2_ORIGIN), 110.0));
    let near = refs::nearest(&[p, pole.clone()], (pole.at.0 + 30.0, pole.at.1)).unwrap();
    assert_eq!(near.0, 1);
    assert!((near.1 - 30.0).abs() < 1e-3);
}

#[test]
fn block_mesh_faces_point_outward() {
    let b = refs::Block { centre: (10.0, -5.0), half: 3.0, base: 1.0, top: 9.0 };
    let m = refs::blocks_mesh(&[b], [0.0, 0.0], [1.0; 4]);
    assert_eq!((m.positions.len(), m.indices.len()), (24, 36));
    let centre = [10.0, 5.0, -5.0];
    for tri in m.indices.chunks(3) {
        let n = m.face_normal([tri[0], tri[1], tri[2]]);
        let p = m.positions[tri[0] as usize];
        let out = [p[0] - centre[0], p[1] - centre[1], p[2] - centre[2]];
        assert!(n[0] * out[0] + n[1] * out[1] + n[2] * out[2] > 0.0, "inward face {tri:?}");
    }
    assert_eq!((m.min, m.max), ([7.0, 1.0, -8.0], [13.0, 9.0, -2.0]));
}

fn tree_params() -> TreeParams {
    TreeParams {
        spacing_m: 40.0,
        density: 0.5,
        height_m: [8.0, 20.0],
        radius_ratio: 0.22,
        max_slope_deg: 25.0,
        max_above_sea_m: 400.0,
        clump_m: 300.0,
    }
}

/// A 129 x 129 grid at 20 m: a gentle ramp (14 degrees) from under the sea (west) up past the treeline (east).
fn ramp_arm() -> Terrain {
    let heights = (0..129).flat_map(|_| (0..129).map(|c| f32::from(u8::try_from(c).unwrap()) * 5.0 - 20.0)).collect();
    Terrain::new(129, 129, 20.0, 0.0, heights)
}

#[test]
fn trees_are_deterministic_and_independent_of_chunking() {
    let t = ramp_arm();
    let p = tree_params();
    let all = ChunkRect { c0: 0, r0: 0, c1: 128, r1: 128 };
    let sorted = |mut v: Vec<refs::Tree>| {
        v.sort_by(|a, b| a.pos[0].total_cmp(&b.pos[0]).then(a.pos[2].total_cmp(&b.pos[2])));
        v
    };
    let whole = sorted(refs::scatter_trees(&t, all, S2_ORIGIN, &p));
    assert!(whole.len() > 50, "{} trees", whole.len());
    let split = sorted(mesh::chunks(t.w, t.h, 32).into_iter().flat_map(|r| refs::scatter_trees(&t, r, S2_ORIGIN, &p)).collect());
    // The grid's far edge (x = 2560 m, z = 2560 m) is outside every half-open chunk: compare without it.
    let inside = |v: &[refs::Tree]| v.iter().filter(|t| t.pos[0] < 2560.0 && t.pos[2] < 2560.0).copied().collect::<Vec<_>>();
    assert_eq!(inside(&whole), inside(&split), "chunking moved, lost or doubled trees");
    assert_eq!(whole, sorted(refs::scatter_trees(&t, all, S2_ORIGIN, &p)));
    for tree in &whole {
        assert!((8.0..=20.0).contains(&tree.height));
        assert!(tree.pos[1] >= 1.0 && tree.pos[1] <= 400.0, "ground {}", tree.pos[1]);
        assert!((tree.radius - tree.height * 0.22).abs() < 1e-4);
    }
    // Another arm origin moves the candidate grid: other trees.
    let shifted = sorted(refs::scatter_trees(&t, all, [S2_ORIGIN[0] + 17.0, S2_ORIGIN[1]], &p));
    assert_ne!(whole, shifted);
    // No trees on steep ground (45 degrees) or at density 0.
    let steep = Terrain::new(65, 65, 20.0, 0.0, (0..65 * 65).map(|k| f32::from(u16::try_from(k % 65).unwrap()) * 20.0 + 5.0).collect());
    assert!(refs::scatter_trees(&steep, ChunkRect { c0: 0, r0: 0, c1: 64, r1: 64 }, S2_ORIGIN, &p).is_empty());
    let none = TreeParams { density: 0.0, ..p };
    assert!(refs::scatter_trees(&t, all, S2_ORIGIN, &none).is_empty());

    // Cones: 2 x sides vertices and sides triangles each, faces outward, bases under the ground.
    let cones = refs::tree_mesh(&whole[..3], [0.0, 0.0], p.max_slope_deg);
    assert_eq!(cones.positions.len(), 3 * 2 * refs::TREE_SIDES);
    assert_eq!(cones.indices.len(), 3 * 3 * refs::TREE_SIDES);
    for (k, tree) in whole[..3].iter().enumerate() {
        let tris = &cones.indices[k * 3 * refs::TREE_SIDES..(k + 1) * 3 * refs::TREE_SIDES];
        for tri in tris.chunks(3) {
            let n = cones.face_normal([tri[0], tri[1], tri[2]]);
            let p0 = cones.positions[tri[0] as usize];
            let p1 = cones.positions[tri[1] as usize];
            let out = [f32::midpoint(p0[0], p1[0]) - tree.pos[0], 0.0, f32::midpoint(p0[2], p1[2]) - tree.pos[2]];
            assert!(n[0] * out[0] + n[2] * out[2] > 0.0, "inward cone face");
        }
        let base = cones.positions[k * 2 * refs::TREE_SIDES][1];
        assert!(base < tree.pos[1]);
    }
}

#[test]
fn contour_ink_draws_constant_width_lines() {
    let c = Contours { interval_m: [100.0, 500.0], width_px: [1.0, 2.5] };
    // 0.5 m per pixel: a major line at 500 m, a minor one at 100 m, nothing between.
    assert_eq!(refs::contour_ink(500.0, 0.5, &c), 1.0);
    assert!((refs::contour_ink(100.0, 0.5, &c) - 0.6).abs() < 1e-6);
    assert_eq!(refs::contour_ink(150.0, 0.5, &c), 0.0);
    assert!((refs::contour_ink(-300.0, 0.5, &c) - 0.6).abs() < 1e-6, "below the sea too");
    // A line stays ~width pixels wide whatever the slope: at 4 m/px, 3 m off a major line is under a pixel away.
    assert!(refs::contour_ink(503.0, 4.0, &c) > 0.9);
    assert_eq!(refs::contour_ink(503.0, 0.5, &c), 0.0);
    // No intervals: no lines.
    let off = Contours { interval_m: [0.0, 0.0], width_px: [1.0, 2.5] };
    assert_eq!(refs::contour_ink(500.0, 0.5, &off), 0.0);
}

#[test]
#[allow(clippy::cast_precision_loss, reason = "a loop index")]
fn compass_and_noise() {
    assert_eq!(refs::compass(0.0, -1.0), "N");
    assert_eq!(refs::compass(1.0, 0.0), "E");
    assert_eq!(refs::compass(0.0, 1.0), "S");
    assert_eq!(refs::compass(-1.0, -1.0), "NW");
    assert_eq!(refs::compass(1.0, 1.0), "SE");
    assert_eq!(refs::hash2(3, -7, 1), refs::hash2(3, -7, 1));
    assert_ne!(refs::hash2(3, -7, 1), refs::hash2(-7, 3, 1));
    let mut lo = f32::INFINITY;
    let mut hi = f32::NEG_INFINITY;
    for k in 0..1000 {
        let v = refs::value_noise(k as f32 * 0.137, k as f32 * -0.071, 9);
        assert!((0.0..1.0).contains(&v));
        lo = lo.min(v);
        hi = hi.max(v);
    }
    assert!(hi - lo > 0.5, "noise varies");
    // Continuous across a lattice line.
    assert!((refs::value_noise(2.0 - 1e-4, 0.3, 9) - refs::value_noise(2.0 + 1e-4, 0.3, 9)).abs() < 1e-2);
}

#[test]
#[allow(clippy::cast_precision_loss, reason = "a loop index")]
fn tint_varies_flat_ground_only_when_asked() {
    let mut t = fixture();
    let along = |t: &Terrain| (0..20).map(|k| mesh::colour(t, (k as f32 * 97.0, 13.0), 150.0, 1.0)).collect::<Vec<_>>();
    assert!(along(&t).windows(2).all(|w| w[0] == w[1]), "no tint: one colour for one height and slope");
    t.tint = Tint { amount: 0.1, wavelength_m: 300.0 };
    let tinted = along(&t);
    assert!(tinted.windows(2).any(|w| w[0] != w[1]), "the tint varies across space");
    assert!(tinted.iter().flatten().all(|v| (0.0..=1.0).contains(v)));
    assert_eq!(mesh::colour(&t, (5.0, 6.0), 150.0, 1.0), mesh::colour(&t, (5.0, 6.0), 150.0, 1.0));
}

/// The contour shader resolves where the material loads it from, straight after embedding (as
/// `MaterialPlugin::build` does): the WG-V2 `Path not found: mw_viewers_bevy/worldgen_view/terrain.wesl` bug.
#[test]
fn terrain_shader_is_embedded_where_the_material_loads_it() {
    use bevy::asset::LoadState;
    use bevy::pbr::MaterialExtension;
    use bevy::prelude::*;
    use bevy::shader::{Shader, ShaderLoader, ShaderRef};

    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default())).init_asset::<Shader>().init_asset_loader::<ShaderLoader>();
    super::embed_terrain_shader(&mut app);
    let ShaderRef::Path(path) = super::ContourExt::fragment_shader() else {
        panic!("the contour extension names its fragment shader by path");
    };
    assert_eq!(path, super::terrain_shader_path());
    assert_eq!(path.to_string().replace('\\', "/"), "embedded://mw_viewers_bevy/worldgen_view/terrain.wesl");
    let handle: Handle<Shader> = app.world().resource::<AssetServer>().load(path);
    for _ in 0..2_000 {
        app.update();
        match app.world().resource::<AssetServer>().load_state(handle.id()) {
            LoadState::Loaded => break,
            LoadState::Failed(e) => panic!("terrain.wesl failed to load: {e}"),
            _ => std::thread::sleep(std::time::Duration::from_millis(1)),
        }
    }
    let shaders = app.world().resource::<Assets<Shader>>();
    let shader = shaders.get(&handle).expect("terrain.wesl loaded within 2 s");
    assert!(shader.imports.iter().all(|i| !matches!(i, bevy::shader::ShaderImport::AssetPath(_))), "no default-source imports");
}
