//! `flora`: learning on a fixture mosaic and the scatter v2 pass on its 10× guide (WG-E2i.3).

#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
// Grid maths reads best in the usual letters (m mosaic, g grid, c col, r row, x, y).
#![allow(clippy::many_single_char_names, clippy::similar_names)]

use glam::Vec3;
use sf_world::{PlacementClass, PlacementFlags, RecordRef};
use sf_worldgen::filter::FilterParams;
use sf_worldgen::guide::Guide;
use sf_worldgen::scatter::{Params, WaterMask};
use sf_worldgen::scatter_fields::{FieldSample, GridFields};
use sf_worldgen::scatter_product;
use sf_worldgen::scatter_species::{Layer, SpeciesFile};
use sf_worldgen::tiles::{GuideTiles, TileSettings};

use crate::flora::towns::TownMask;
use crate::flora::{self, FloraRun, GROUP_FLORA, GROUP_ROCKS, Learned, SourceRef, World};
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
        extent_m: Some(if group == GROUP_ROCKS { 6.0 } else if key == "flora_b" { 0.5 } else { 3.0 }),
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
    for k in 0..20 {
        refs.push(sref("flora_b", GROUP_FLORA, m, 8.0 + f64::from(k % 10), 40.0 + f64::from(k), [0.0, 0.0, 1.0], 1.0));
    }
    refs.push(sref("flora_kelp", GROUP_FLORA, m, 1.0, 20.0, [0.0, 0.0, 1.0], 1.0));
    refs.push(sref("flora_kelp", GROUP_FLORA, m, 2.0, 30.0, [0.0, 0.0, 1.0], 1.0));
    refs.push(sref("flora_a", GROUP_FLORA, m, 80.0, 30.0, [0.0, 0.0, 1.0], 1.0));
    refs
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


fn shipped() -> SpeciesFile {
    SpeciesFile::parse_toml(flora::SHIPPED_SPECIES).unwrap()
}

fn learned(m: &Mosaic) -> Learned {
    flora::learn(m, &regions(), &fixture_refs(m), &shipped(), &Params::default(), 0.0)
}

/// WG-E2i.3: base records join the row nearest their size, the learned columns replace the authored ones, the
/// density is per m² of dry land, and a row with no model is dropped.
#[test]
fn learning_fills_the_rows_models_sizes_pose_and_slope() {
    let m = mosaic();
    let prm = Params::default();
    let l = learned(&m);
    assert_eq!((l.skipped_wet, l.skipped_outside, l.skipped_no_region), ([2, 0], 1, 0));
    assert_eq!(l.learned, [50, 5], "rocks are counted, not slotted");
    let names: Vec<&str> = l.file.species.iter().map(|s| s.name.as_str()).collect();
    assert_eq!(names, ["tree", "flora"], "flora_a (3-3.9 m) is a tree, flora_b (0.5 m) ground flora, no shrub");
    assert_eq!(l.slots.iter().map(Vec::len).collect::<Vec<_>>(), [1, 1]);
    assert_eq!(l.reps.len(), 2, "kelp and rocks never reached a row");
    let (tree, fl) = (&l.file.species[0], &l.file.species[1]);
    assert!((tree.size_min_m - 3.0).abs() < 1e-5 && (tree.size_max_m - 3.0 * 1.29).abs() < 1e-5, "{tree:?}");
    // flora_b's 0.5 m stays inside the row's column range [0.2, 40]; min = max is a fixed size.
    assert!((fl.size_min_m - 0.5).abs() < 1e-6 && (fl.size_max_m - 0.5).abs() < 1e-6);
    let a = &l.slots[0][0];
    assert_eq!(a.count(), 30);
    assert!(a.align.abs() < 1e-6 && (a.offset_m + 0.3).abs() < 1e-4 && a.max_slope_deg < 1.0, "{a:?}");
    // The slope hat's e_max is the steepest reference + the margin; the authored inner edge is cut to it.
    let h = tree.hats.slope_deg.unwrap();
    assert!((h.e_max - f64::from(a.max_slope_deg + prm.slope_margin_deg)).abs() < 1e-5 && h.i_max <= h.e_max, "{h:?}");
    // Dry land: columns 4..=64 of 65 rows.
    let sp = flora::source_grid(&m).spacing;
    let land = f64::from(61 * 65) * sp * sp;
    assert!((l.density[0][1] - 30.0 / land).abs() < 1e-12 && (l.density[1][1] - 20.0 / land).abs() < 1e-12);
    assert!(l.density[0][0].abs() < f64::EPSILON, "region 0 is none");
}

/// The region prior on a hand table: two regions, two classes plus none. Row densities D = (0, 2, 6) per region;
/// class 0 land A = (0, 1, 1) -> M = 4, class 1 A = (0, 3, 1) -> M = 3, no land of class "none" -> prior 0.
#[test]
fn region_prior_is_the_density_over_its_class_mean() {
    let density = vec![vec![0.0, 2.0, 6.0], vec![0.0, 0.0, 0.0]];
    let area = vec![vec![0, 0, 0], vec![1, 3, 0], vec![1, 1, 0]];
    let p = flora::region_prior(&density, &area);
    assert_eq!(p[0][1], vec![0.5, 2.0 / 3.0, 0.0]);
    assert_eq!(p[0][2], vec![1.5, 2.0, 0.0]);
    assert_eq!(p[0][0], vec![0.0, 0.0, 0.0]);
    assert!(p[1].iter().flatten().all(|&v| v == 0.0), "a row absent everywhere has prior 0");
    // The area-weighted mean prior of a class is 1: the class keeps its authored intensity.
    for c in 0..2 {
        let (num, den) = (1..3).fold((0.0, 0.0), |(n, d), r| (n + p[0][r][c] * area[r][c] as f64, d + area[r][c] as f64));
        assert!((num / den - 1.0).abs() < 1e-12, "class {c}: {}", num / den);
    }
}

/// Guide fields for the fixture: woodland west of guide column 48, desert east of it; the slope from the guide.
fn fields(world: &World<'_>) -> GridFields {
    let g = &world.guide;
    let fill = FieldSample {
        temperature_c: 15.0,
        precipitation_mm: 1000.0,
        moisture: 0.5,
        class: Some(0),
        slope_deg: 0.0,
        aspect_deg: 0.0,
        altitude_m: 0.0,
        curvature: 0.0,
        water_distance_m: 1e6,
    };
    let mut f = GridFields::filled(g.w, g.h, g.spacing, g.origin, &fill, vec!["woodland".into(), "desert".into()]);
    for r in 0..g.h {
        for c in 0..g.w {
            f.classes[r * g.w + c] = u8::from(c >= 48);
            f.slope_deg[r * g.w + c] = g.slope_normal(c as f64, r as f64).0;
        }
    }
    f
}

/// The pass over `tiles` (all when `None`) with one region of woodland land (prior 1 where the row lives).
fn run_on(l: &Learned, world: &World<'_>, f: &GridFields, prm: &Params, tiles: Option<&[(i32, i32)]>) -> (Vec<flora::Plant>, [usize; 4]) {
    let area = vec![vec![0u64; 3], vec![1, 0, 0]];
    let prior = flora::region_prior(&l.density, &area);
    let water = WaterMask::sea(0.0, prm.water_margin_m);
    flora::scatter_tiles(l, world, f, &water, &prior, prm, tiles)
}

/// WG-E2i.3: the v2 pass on the fixture's 10× guide. Every tile alone (reversed order) gives the whole run's plants;
/// nothing on the sea, no tree in the desert or past its slope hat; the placements copy their model's reference.
#[test]
fn scatter_v2_on_the_ten_times_guide_tile_alone_and_placements() {
    let m = mosaic();
    let regions = regions();
    let l = learned(&m);
    let t = tiles(&m);
    assert!((flora::guide_scale(t.guide(), &m).unwrap() - 10.0).abs() < 1e-9);
    let towns = TownMask::default();
    let world = World::new(&m, &regions, &t, None, &towns);
    let f = fields(&world);
    let prm = Params::default();
    let (plants, full) = run_on(&l, &world, &f, &prm, None);
    assert_eq!(full[0], 0, "no rock rows yet (WG-E3f)");
    assert!(full[1] > 50 && full[3] > 50, "trees and flora: {full:?}");
    assert_eq!(full[2], 0, "no shrub row learned");
    assert_eq!(plants.len(), full.iter().sum::<usize>(), "the default budget keeps everything");
    // Tile alone == whole, in any order.
    let rect = world.rect();
    let tm = prm.tile_m;
    let (x0, y0) = ((rect[0] / tm).floor() as i32, (rect[1] / tm).floor() as i32);
    let (x1, y1) = ((rect[2] / tm).ceil() as i32, (rect[3] / tm).ceil() as i32);
    let tiles: Vec<(i32, i32)> = (y0..y1).flat_map(|y| (x0..x1).map(move |x| (x, y))).collect();
    let mut alone: Vec<flora::Plant> = Vec::new();
    for &tile in tiles.iter().rev() {
        alone.extend(run_on(&l, &world, &f, &prm, Some(&[tile])).0);
    }
    let key = |p: &flora::Plant| (p.key, p.pos[0].to_bits(), p.pos[1].to_bits());
    let mut w = plants.clone();
    alone.sort_by_key(key);
    w.sort_by_key(key);
    assert_eq!(alone, w, "every tile alone == the whole run");
    let (mut trees, mut on_ramp) = (0, 0);
    for p in &plants {
        assert!(world.height(p.pos[0], p.pos[1]) > 0.0, "{p:?} on the sea");
        let (c, _) = world.guide.colrow(p.pos[0], p.pos[1]);
        if p.key.layer == Layer::Canopy {
            trees += 1;
            assert!(c < 48.0, "a tree in the desert: col {c}");
            on_ramp += usize::from(c > 33.0);
            assert!(p.scale >= 1.0 - 1e-6 && p.scale <= 1.29 + 1e-6, "scale inside the learned range: {}", p.scale);
            assert!((p.pos[2] - world.height(p.pos[0], p.pos[1]) + 0.3 * f64::from(p.scale)).abs() < 1e-3, "offset");
        }
    }
    assert!(trees > 50, "{trees} trees");
    let e_max = l.file.species[0].hats.slope_deg.unwrap().e_max;
    assert_eq!(on_ramp, 0, "no tree on the 45 deg ramp (slope hat e_max {e_max})");
    let run = FloraRun {
        prm,
        learned: l.clone(),
        prior: flora::region_prior(&l.density, &[vec![0; 3], vec![1, 0, 0]]),
        area: vec![vec![0; 3], vec![1, 0, 0]],
        sample_m2: 1.0,
        full,
        plants: plants.clone(),
        region_names: vec![String::new(), "Test Region".into()],
        scale: 10.0,
        stamped: false,
        towns: flora::towns::TownCensus::default(),
        clearings: flora::ClearingCensus::default(),
        product: flora::ProductUse::Off,
        seconds: [0.0; 4],
    };
    let placements = flora::placements(&run);
    assert_eq!(placements.len(), run.plants.len());
    let mut ids: Vec<_> = placements.iter().map(|p| p.id).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), placements.len(), "ids are unique");
    for (p, i) in placements.iter().zip(&run.plants) {
        let e = p.transform.pos.to_f64();
        assert!((e.x - i.pos[0]).abs() < 1e-3 && (e.y - i.pos[2]).abs() < 1e-3 && (e.z + i.pos[1]).abs() < 1e-3);
        let up = p.transform.rotation * Vec3::Y;
        assert!((up - Vec3::new(i.up[0], i.up[2], -i.up[1])).length() < 1e-4);
        let slot = &l.slots[i.key.species as usize][i.slot as usize];
        assert_eq!(p.base, l.reps[slot.rep].base);
    }
    let md = flora::census_md(&run, std::path::Path::new("fixture"), 4);
    assert!(md.contains("| tree |") && md.contains("## Test Region") && md.contains("flora_a"), "{md}");
    println!("flora v2 fixture: {full:?} before budget, {} placed", plants.len());
}

/// A guide town (walled square) and the guide's road clearing: no plant inside either or within the town's apron.
#[test]
fn scatter_v2_keeps_out_of_towns_and_clearings() {
    use flora::towns::{TownParams, footprint};
    use glam::{DVec3, Quat};
    use sf_coords::{WorldPos, WorldTransform};
    use sf_world::{Placement, PlacementId};

    let m = mosaic();
    let regions = regions();
    let l = learned(&m);
    let t = tiles(&m);
    let g = t.guide();
    let at = |c: f64, r: f64| [g.origin_m[0] + c * g.spacing_m, g.origin_m[1] - r * g.spacing_m];
    let [cx, cy] = at(18.0, 32.0);
    let wall = |key: &str, x: f64, y: f64, yaw: f32| Placement {
        id: PlacementId::from_key("test.town", key.as_bytes()),
        base: RecordRef::new("STAT", "wall"),
        model: sf_vfs::VfsPath::new("meshes/x/wall.nif").ok(),
        transform: WorldTransform {
            pos: WorldPos::from_f64(DVec3::new(x, 5.0, -y)),
            rotation: Quat::from_rotation_y(yaw),
            scale: Vec3::ONE,
        },
        class: PlacementClass::Static,
        flags: PlacementFlags::empty(),
    };
    let q = std::f32::consts::FRAC_PI_2;
    let town =
        vec![wall("n", cx, cy + 50.0, 0.0), wall("s", cx, cy - 50.0, 0.0), wall("e", cx + 50.0, cy, q), wall("w", cx - 50.0, cy, -q)];
    let bounds = |model: &str| (model == "meshes/x/wall.nif").then_some(([-50.0, 0.0, -1.0], [50.0, 5.0, 1.0]));
    let tp = TownParams::default();
    let mask = TownMask::new(&[town.as_slice()], &bounds, &tp);
    let fps: Vec<_> = town.iter().map(|p| footprint(p, &bounds).0).collect();

    let dir = tempfile::tempdir().unwrap();
    let (r0, r1, r2) = (at(10.0, 2.0), at(10.0, 32.0), at(10.0, 62.0));
    let roads = serde_json::json!({ "format": 1, "junctions": [], "segments": [{ "id": 1, "class": "local",
        "surface": "dirt", "width_m": 4.0, "a": 0, "b": 1, "points": [r0, r1, r2], "crossings": [],
        "length_m": r0[1] - r2[1], "grade": 0.0 }] });
    std::fs::write(dir.path().join("roads.json"), roads.to_string()).unwrap();
    let c = flora::guide_clearings(dir.path()).unwrap().expect("the road clears");

    let plain_towns = TownMask::default();
    let plain = World::new(&m, &regions, &t, None, &plain_towns);
    let f = fields(&plain);
    let prm = Params::default();
    let before = run_on(&l, &plain, &f, &prm, None).0;
    let with = World::new(&m, &regions, &t, Some(&c), &mask);
    let after = run_on(&l, &with, &f, &prm, None).0;
    let inside = |p: &flora::Plant| c.contains(p.pos[0], p.pos[1]) || mask.contains(p.pos[0], p.pos[1]);
    let held = before.iter().filter(|p| inside(p)).count();
    assert!(held >= 10, "the town and road held {held} plants before");
    for p in &after {
        assert!(!inside(p), "{p:?} in a town or clearing");
        let d = fps.iter().map(|fp| fp.distance(p.pos[0], p.pos[1])).fold(f64::INFINITY, f64::min);
        assert!(d > tp.margin_m, "{p:?} {d:.2} m from the town");
    }
    println!("flora v2 census delta: {} plants without exclusions, {} with ({held} stood in them)", before.len(), after.len());
}

/// A guide folder without the climate product: the pass names the command that makes it; no own species file
/// gives the shipped one.
#[test]
fn a_guide_without_the_climate_product_says_how_to_make_it() {
    let m = mosaic();
    let t = tiles(&m);
    let dir = tempfile::tempdir().unwrap();
    let e = flora::product_fields(dir.path(), t.guide()).unwrap_err().to_string();
    assert!(e.contains("scrib-worldgen climate"), "{e}");
    assert_eq!(flora::species_file(dir.path()).unwrap(), shipped());
}

/// WG-E2j.2: the pass through the guide's `scatter/` product. The first load writes it and the next reads it, both
/// with the passes' plants; the product read back is the computed kept set; the budget stays at load (no rebuild);
/// another seed or a changed guide file is stale and rebuilds; a damaged product is rebuilt.
#[test]
fn the_scatter_product_is_written_once_read_at_load_and_rebuilt_when_stale() {
    use flora::ProductUse;
    let m = mosaic();
    let regions = regions();
    let l = learned(&m);
    let t = tiles(&m);
    let towns = TownMask::default();
    let world = World::new(&m, &regions, &t, None, &towns);
    let f = fields(&world);
    let prm = Params::default();
    let prior = flora::region_prior(&l.density, &[vec![0; 3], vec![1, 0, 0]]);
    let water = WaterMask::sea(0.0, prm.water_margin_m);
    let dir = tempfile::tempdir().unwrap();
    let d = dir.path();
    let run = |prm: &Params| flora::scatter_cached(d, &l, &world, &f, &water, &prior, prm).unwrap();
    let computed = flora::scatter_tiles(&l, &world, &f, &water, &prior, &prm, None);
    let (plants, full, used) = run(&prm);
    assert_eq!(used, ProductUse::Wrote, "no product yet");
    assert_eq!((plants.clone(), full), computed, "written: the passes' plants");
    let (again, full2, used) = run(&prm);
    assert_eq!(used, ProductUse::Read, "same inputs: read");
    assert_eq!((again, full2), computed, "read == computed");
    let inputs = flora::product_inputs(d, &world, &f, &l, &prior, &prm).unwrap();
    let (stored, won) = scatter_product::read(d).unwrap();
    assert_eq!(stored, inputs);
    assert_eq!(won, flora::winners(&l, &world, &f, &water, &prior, &prm, None), "the product is the kept set");
    // The budget cut stays at load.
    let half = Params { budget: (plants.len() / 2) as f64, ..prm };
    let (cut, _, used) = run(&half);
    assert_eq!(used, ProductUse::Read, "a budget change reads the product");
    assert!(!cut.is_empty() && cut.len() < plants.len(), "{} of {}", cut.len(), plants.len());
    // Stale: another seed rebuilds, and the product follows it.
    let seeded = Params { seed: prm.seed + 1, ..prm };
    assert_eq!(run(&seeded).2, ProductUse::Wrote, "another seed: stale");
    assert_ne!(scatter_product::read(d).unwrap().0, inputs);
    assert_eq!(run(&prm).2, ProductUse::Wrote, "back to the first seed: stale again");
    // Stale: a guide file the pass reads.
    std::fs::write(d.join("roads.json"), b"{}").unwrap();
    assert_ne!(flora::product_inputs(d, &world, &f, &l, &prior, &prm).unwrap(), inputs, "a guide file");
    std::fs::remove_file(d.join("roads.json")).unwrap();
    assert_eq!(flora::product_inputs(d, &world, &f, &l, &prior, &prm).unwrap(), inputs);
    // A damaged product of these inputs is rebuilt.
    let bin = d.join(scatter_product::DIR).join("plants.bin");
    let mut bytes = std::fs::read(&bin).unwrap();
    bytes[9] ^= 0x40;
    std::fs::write(&bin, bytes).unwrap();
    assert!(matches!(run(&prm).2, ProductUse::Rebuilt(_)), "damaged: rebuilt");
    assert_eq!(run(&prm), (computed.0, computed.1, ProductUse::Read));
}

/// WG-E2i.2: the shipped species file parses, has its one canopy tree plus understorey and ground rows, and names
/// only classes of the shipped class table (the x10 climate run's).
#[test]
fn shipped_species_file_parses_and_names_only_table_classes() {
    use sf_worldgen::climate_classes::ClassTable;
    use sf_worldgen::scatter_species::{Layer, SpeciesFile};
    assert_eq!(flora::SPECIES_FILE, "scatter_species.toml");
    let f = SpeciesFile::parse_toml(include_str!("../../../worldgen/scatter_species.toml")).unwrap();
    let layer = |l: Layer| f.species.iter().filter(|s| s.layer == l).count();
    assert_eq!(layer(Layer::Canopy), 1, "one canopy tree species (owner 2026-11-18)");
    assert!(layer(Layer::Understorey) >= 1 && layer(Layer::Ground) >= 1);
    assert!(f.species.iter().all(|s| s.model.is_none()), "model slots are filled from learned refs at run time");
    let table = ClassTable::whittaker();
    let names = table.names();
    for class in f.classes.keys().chain(f.species.iter().flat_map(|s| &s.classes)) {
        assert!(names.contains(class), "class {class:?} is not in the shipped class table");
    }
    assert_eq!(f.classes.len(), 12, "every table class has its stems per hectare");
}
