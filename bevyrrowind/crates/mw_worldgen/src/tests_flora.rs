//! `flora`: learning on a fixture mosaic and the scatter on its 10× guide.

#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
// Grid maths reads best in the usual letters (m mosaic, g grid, c col, r row, x, y).
#![allow(clippy::many_single_char_names, clippy::similar_names)]

use glam::Vec3;
use sf_world::{PlacementClass, PlacementFlags, RecordRef};
use sf_worldgen::filter::FilterParams;
use sf_worldgen::guide::Guide;
use sf_worldgen::scatter::{Params, Scatter, Terrain};
use sf_worldgen::tiles::{GuideTiles, TileSettings};

use crate::flora::{self, FloraRun, GROUP_FLORA, GROUP_ROCKS, Grid, SourceRef, World};
use crate::{Mosaic, RegionEntry, RegionMap};

/// One cell, 65 × 65 samples: wet for col ≤ 3, flat at 5 m to col 31, then a 45° ramp rising east.
fn mosaic() -> Mosaic {
    let sp = 128.0 * mw_world::convert::UNIT_METERS_F64;
    let h = |c: usize| {
        if c <= 3 {
            -1.0
        } else if c <= 32 {
            5.0
        } else {
            5.0 + (c - 32) as f32 * sp as f32
        }
    };
    let heights = (0..65 * 65).map(|k| h(k % 65)).collect();
    Mosaic {
        cells_min: (0, 0),
        cells_max: (0, 0),
        width: 65,
        height: 65,
        heights,
        mask: vec![1; 65 * 65],
        land_cells: 1,
        without_heights: 0,
        edge_mismatches: 0,
        max_edge_mismatch_m: 0.0,
    }
}

fn regions() -> RegionMap {
    let e = |index: u8, id: &str| RegionEntry { index, id: id.into(), name: String::new(), color: [index; 3], missing: false };
    RegionMap { cells_min: (0, 0), cells_max: (0, 0), width: 1, height: 1, index: vec![1], legend: vec![e(0, ""), e(1, "Test Region")] }
}

fn sref(key: &str, group: u8, m: &Mosaic, c: f64, r: f64, up: [f32; 3], scale: f32) -> SourceRef {
    let g = flora::source_grid(m);
    let (x, y) = (g.origin[0] + c * g.spacing, g.origin[1] - r * g.spacing);
    SourceRef {
        key: key.into(),
        base: RecordRef::new("STAT", key),
        model: sf_vfs::VfsPath::new(&format!("meshes/f/{key}.nif")).ok(),
        class: PlacementClass::Static,
        flags: PlacementFlags::empty(),
        group,
        pos: [x, y, g.at(c, r) - 0.3 * f64::from(scale)],
        up,
        scale,
        extent_m: Some(if group == GROUP_ROCKS { 6.0 } else { 3.0 }),
    }
}

fn fixture_refs(m: &Mosaic) -> Vec<SourceRef> {
    let mut refs = Vec::new();
    for k in 0..30 {
        refs.push(sref("flora_a", GROUP_FLORA, m, 6.0 + f64::from(k % 20), 5.0 + f64::from(k), [0.0, 0.0, 1.0], 1.0 + 0.01 * k as f32));
    }
    let n = std::f32::consts::FRAC_1_SQRT_2;
    for k in 0..5 {
        refs.push(sref("terrain_rock_b", GROUP_ROCKS, m, 40.0 + f64::from(k), 10.0 + f64::from(k), [-n, 0.0, n], 2.0));
    }
    refs.push(sref("flora_kelp", GROUP_FLORA, m, 1.0, 20.0, [0.0, 0.0, 1.0], 1.0));
    refs.push(sref("flora_kelp", GROUP_FLORA, m, 2.0, 30.0, [0.0, 0.0, 1.0], 1.0));
    refs.push(sref("flora_a", GROUP_FLORA, m, 80.0, 30.0, [0.0, 0.0, 1.0], 1.0));
    refs
}

#[test]
fn learning_counts_densities_and_occupancy() {
    let m = mosaic();
    let prm = Params::default();
    let l = flora::learn(&m, &regions(), &fixture_refs(&m), &prm, 0.0);
    assert_eq!((l.skipped_wet, l.skipped_outside, l.skipped_no_region), ([2, 0], 1, 0));
    assert_eq!(l.learned, [30, 5]);
    assert_eq!(l.reps.len(), 2, "kelp never reached the table");
    let entries = &l.table.regions[1];
    assert_eq!(entries.len(), 2);
    let (a, rock) = (&entries[0], &entries[1]);
    assert_eq!((a.count, rock.count), (30, 5));
    // Dry land: columns 4..=64 of 65 rows; flat (slope class 0, 5 m band): columns 5..=31.
    let sp = flora::source_grid(&m).spacing;
    let land = f64::from(61 * 65);
    assert!((a.density_m2 - 30.0 / (land * sp * sp)).abs() < 1e-12, "{}", a.density_m2);
    let share = f64::from(27 * 65) / land;
    let flat = a.occupancy[prm.class(0.0, 5.0)];
    let want = ((30.0 + prm.occupancy_prior * share) / (30.0 + prm.occupancy_prior)) / share;
    assert!((f64::from(flat) - want).abs() < 1e-5, "flat occupancy {flat}, want {want}");
    assert!(a.max_slope_deg < 1.0 && rock.max_slope_deg > 40.0);
    // Every class steeper than the flora's flat ground is empty.
    assert!(a.occupancy.iter().enumerate().all(|(c, &o)| c / prm.bands == 0 || o == 0.0));
    assert!((a.scale[0] - 1.0).abs() < 1e-6 && (a.scale[1] - 1.29).abs() < 1e-5);
    assert!((a.offset_m + 0.3).abs() < 1e-4, "offset {}", a.offset_m);
    assert!((rock.align - 1.0).abs() < 1e-3, "rocks lie along the slope: align {}", rock.align);
    assert!(a.align.abs() < 1e-6);
    assert!(l.table.species[1].size_m > l.table.species[0].size_m);
}

/// The fixture at 10×: the guide is the mosaic's grid, heights × 10.
fn tiles(m: &Mosaic) -> GuideTiles {
    let g = flora::source_grid(m);
    let guide = Guide {
        w: m.width,
        h: m.height,
        spacing_m: g.spacing * 10.0,
        sea_level_m: 0.0,
        fill_m: None,
        origin_m: [g.origin[0] * 10.0, g.origin[1] * 10.0],
        heights: m.heights.iter().map(|h| h * 10.0).collect(),
        regions: None,
    };
    GuideTiles::new(guide, mw_world::ids::exterior(), FilterParams { octaves: 0, ..FilterParams::default() }, TileSettings::default())
}

#[test]
fn scatter_on_the_ten_times_guide_and_its_placements() {
    let m = mosaic();
    let regions = regions();
    let prm = Params { tile_m: 128.0, census_step_m: 8.0, ..Params::default() };
    let learned = flora::learn(&m, &regions, &fixture_refs(&m), &prm, 0.0);
    let t = tiles(&m);
    let g = t.guide();
    assert!((flora::guide_scale(g, &m).unwrap() - 10.0).abs() < 1e-9);
    let world = World {
        guide: Grid { w: g.w, h: g.h, origin: g.origin_m, spacing: g.spacing_m, data: &g.heights },
        source: &m,
        regions: &regions,
        tiles: &t,
    };
    let rect = [g.origin_m[0], g.origin_m[1] - 64.0 * g.spacing_m, g.origin_m[0] + 64.0 * g.spacing_m, g.origin_m[1]];
    let area = sf_worldgen::scatter::area_census(&world, &prm, rect, regions.legend.len());
    let expected = sf_worldgen::scatter::expected(&learned.table, &prm, &area);
    let keep = sf_worldgen::scatter::budget_keep(&learned.table, &prm, &expected);
    let items = Scatter::new(&learned.table, keep.clone(), prm).run(&world, rect);
    let full: f64 = expected.iter().flatten().sum();
    // Same density per m² on 100× the land: ~100× the source's 35 references.
    assert!(full > 1500.0 && full < 5000.0, "full {full}");
    assert!((items.len() as f64 / full - 1.0).abs() < 0.1, "{} items, {full:.0} expected", items.len());
    let (mut rocks, mut on_ramp) = (0, 0);
    for i in &items {
        assert!(world.height(i.pos[0], i.pos[1]) > 0.0, "{i:?} on water");
        let (c, _) = world.guide.colrow(i.pos[0], i.pos[1]);
        if i.species == 0 {
            assert!((4.0..32.5).contains(&c), "flora on the flat: col {c}");
        } else {
            rocks += 1;
            on_ramp += usize::from(c > 31.0);
        }
    }
    // Rocks prefer the ramp; the smoothing prior (2 pseudo-placements against 5 seen) leaves ~10 % on the shore step.
    assert!(rocks > 100 && on_ramp * 10 >= rocks * 8, "{on_ramp} of {rocks} rocks on the ramp");
    let run = FloraRun {
        prm,
        learned,
        expected,
        keep,
        items,
        region_names: vec![String::new(), "Test Region".into()],
        scale: 10.0,
        stamped: false,
        seconds: [0.0; 4],
    };
    let placements = flora::placements(&run);
    assert_eq!(placements.len(), run.items.len());
    let mut ids: Vec<_> = placements.iter().map(|p| p.id).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), placements.len(), "ids are unique");
    for (p, i) in placements.iter().zip(&run.items) {
        let e = p.transform.pos.to_f64();
        assert!((e.x - i.pos[0]).abs() < 1e-3 && (e.y - i.pos[2]).abs() < 1e-3 && (e.z + i.pos[1]).abs() < 1e-3);
        let up = p.transform.rotation * Vec3::Y;
        assert!((up - Vec3::new(i.up[0], i.up[2], -i.up[1])).length() < 1e-4);
        assert_eq!(p.base, run.learned.reps[i.species as usize].base);
    }
    let md = flora::census_md(&run, std::path::Path::new("fixture"), 4);
    assert!(md.contains("## Test Region") && md.contains("flora_a") && md.contains("terrain_rock_b"), "{md}");
}
