//! `flora`: learning on a fixture mosaic and the scatter v2 pass on its 10× guide (WG-E2i.3).

#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
// Grid maths reads best in the usual letters (m mosaic, g grid, c col, r row, x, y).
#![allow(clippy::many_single_char_names, clippy::similar_names)]

use glam::Vec3;
use sf_world::{PlacementClass, PlacementFlags, RecordRef};
use sf_worldgen::filter::FilterParams;
use sf_worldgen::guide::Guide;
use sf_worldgen::scatter::{CanopyTargets, Params, WaterMask};
use sf_worldgen::scatter_calib::Calibration;
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
    assert_eq!(l.learned, [50, 5]);
    let names: Vec<&str> = l.file.species.iter().map(|s| s.name.as_str()).collect();
    assert_eq!(names, ["tree", "flora"], "flora_a (3-3.9 m) is a tree, flora_b (0.5 m) ground flora, no shrub");
    assert_eq!(l.slots.iter().map(Vec::len).collect::<Vec<_>>(), [1, 1]);
    assert_eq!(l.reps.len(), 3, "kelp never reached a row; the rock model's reference comes after the rows'");
    assert_eq!(l.rocks.len(), 1, "one rock base record, one learned rock model (WG-E3f.1)");
    assert_eq!(l.reps[l.rocks[0].rep].key, "terrain_rock_b");
    assert_eq!((l.rocks[0].count(), l.rocks[0].scale, l.rocks[0].extent_m), (5, [2.0, 2.0], 6.0));
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
    assert!((l.rock_density[1] - 5.0 / land).abs() < 1e-12 && l.rock_density[0].abs() < f64::EPSILON);
}

/// Library classes built today (`sf_rockgen` `classes.rs` `CLASSES`; `mw_worldgen` does not depend on `sf_rockgen`).
const BUILT_CLASSES: [&str; 9] = ["block", "platy", "cobble", "chipped", "talus", "jagged", "columnar", "scoria", "slab"];

fn geology_table() -> sf_worldgen::geology::GeologyTable {
    crate::geology::AdapterGeology::from_toml(include_str!("../../../worldgen/geology.toml")).unwrap().table
}

/// WG-E3f.1: the shipped rock units are geology.toml's units, name only built library classes, and have an outcrop
/// context exactly when their geology unit has outcrops.
#[test]
fn shipped_rock_units_are_the_geology_units_with_built_classes() {
    use sf_worldgen::scatter_species::{RockContext, RockEntry};
    let f = shipped();
    let geo = geology_table();
    let units: Vec<&str> = f.rock_units.keys().map(String::as_str).collect();
    let mut names: Vec<&str> = geo.units.iter().map(|u| u.name.as_str()).collect();
    names.sort_unstable();
    assert_eq!(units, names);
    assert_eq!(f.rocks.len(), 3, "talus, outcrop and field rows");
    for u in &geo.units {
        let ru = &f.rock_units[&u.name];
        assert_eq!(!ru.entries(RockContext::Outcrop).is_empty(), u.outcrop, "{}: outcrop context", u.name);
        assert!(!ru.entries(RockContext::Talus).is_empty() && !ru.entries(RockContext::Field).is_empty(), "{}", u.name);
        for e in ru.entries.values().flatten() {
            let RockEntry::Class { class, .. } = e else { panic!("{}: the shipped file names no model", u.name) };
            assert!(BUILT_CLASSES.contains(&class.as_str()), "{}: class {class:?} is not built", u.name);
        }
    }
}

/// WG-E3f.1: every learned rock model becomes a pack-model entry with its bounds axis and learned scale range in
/// every context its units list, weighted by its count x the region's geology unit share; the file stays valid.
#[test]
fn learned_rock_models_become_entries_weighted_by_unit_share() {
    use sf_worldgen::scatter_species::{RockContext, RockEntry};
    let m = mosaic();
    let geo = geology_table();
    let models = |l: &Learned, unit: &str| -> Vec<(RockContext, RockEntry)> {
        l.file.rock_units[unit]
            .entries
            .iter()
            .flat_map(|(c, v)| v.iter().filter(|e| matches!(e, RockEntry::Model { .. })).map(|e| (*c, e.clone())))
            .collect()
    };
    let entry = |weight: f64| RockEntry::Model { model: "terrain_rock_b".into(), axis_m: 6.0, scale_min: 2.0, scale_max: 2.0, weight };
    // A named region: Bitter Coast = alluvium 0.85, coastal_basalt 0.15; 5 references.
    let mut named = regions();
    named.legend[1].name = "Bitter Coast Region".into();
    let mut l = learned(&m);
    let before = l.file.rock_units.clone();
    assert_eq!(flora::rocks::rock_entries(&mut l, &named, &geo), 2 + 3);
    let (t, o, fl) = (RockContext::Talus, RockContext::Outcrop, RockContext::Field);
    let near = |got: Vec<(RockContext, RockEntry)>, want: Vec<(RockContext, f64)>| {
        assert_eq!(got.len(), want.len(), "{got:?}");
        for ((gc, ge), (wc, w)) in got.iter().zip(&want) {
            let RockEntry::Model { weight, .. } = ge else { unreachable!() };
            assert_eq!(*gc, *wc);
            assert!((weight - w).abs() < 1e-12, "{weight} vs {w}");
            assert_eq!(*ge, entry(*weight));
        }
    };
    near(models(&l, "alluvium"), vec![(t, 5.0 * 0.85), (fl, 5.0 * 0.85)]);
    near(models(&l, "coastal_basalt"), vec![(t, 5.0 * 0.15), (o, 5.0 * 0.15), (fl, 5.0 * 0.15)]);
    for u in ["basalt_flow", "tuff", "tephra", "capped_sediment"] {
        assert_eq!(l.file.rock_units[u], before[u], "{u}: no share in the region, no entry");
    }
    let back = SpeciesFile::parse_toml(&l.file.to_toml().unwrap()).unwrap();
    assert_eq!(back.rock_units, l.file.rock_units, "the entries pass the file's checks");
    // No region row: the default shares (tuff 0.5, alluvium 0.5).
    let mut l = learned(&m);
    assert_eq!(flora::rocks::rock_entries(&mut l, &regions(), &geo), 3 + 2);
    near(models(&l, "tuff"), vec![(t, 2.5), (o, 2.5), (fl, 2.5)]);
    near(models(&l, "alluvium"), vec![(t, 2.5), (fl, 2.5)]);
}

/// WG-E3f.1: the field-stone prior on a hand table: D = (0, 2, 6), land a = (0, 3, 1) -> M = 12 / 4 = 3.
#[test]
fn rock_prior_is_the_density_over_its_land_mean() {
    let land = [0, 3, 1];
    let p = flora::rocks::rock_prior(&[0.0, 2.0, 6.0], &land);
    assert_eq!(p, vec![0.0, 2.0 / 3.0, 2.0]);
    let mean = p.iter().zip(land).map(|(v, a)| v * a as f64).sum::<f64>() / 4.0;
    assert!((mean - 1.0).abs() < 1e-12, "{mean}");
    assert_eq!(flora::rocks::rock_prior(&[0.0, 0.0], &[0, 5]), vec![0.0, 0.0], "no rocks anywhere: prior 0");
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

/// The fixture's region prior (one region of woodland land, prior 1 where the row lives) and sea.
fn prior_water(l: &Learned, prm: &Params) -> (Vec<Vec<Vec<f64>>>, WaterMask) {
    (flora::region_prior(&l.density, &[vec![0u64; 3], vec![1, 0, 0]]), WaterMask::sea(0.0, prm.water_margin_m))
}

/// The canopy calibration of the fixture pass on `world`.
fn cal_on(l: &Learned, world: &World<'_>, f: &GridFields, prm: &Params) -> Calibration {
    let (prior, water) = prior_water(l, prm);
    flora::calibration(l, world, f, &water, &prior, None, prm)
}

/// The pass over `tiles` (all when `None`) at `targets`.
fn run_at(l: &Learned, world: &World<'_>, f: &GridFields, prm: &Params, targets: &CanopyTargets, tiles: Option<&[(i32, i32)]>) -> (Vec<flora::Plant>, [usize; 4]) {
    let (prior, water) = prior_water(l, prm);
    let (plants, _, full) = flora::scatter_tiles(l, world, f, &water, &prior, None, prm, targets, tiles);
    (plants, full)
}

/// The pass over every tile at `world`'s own calibration.
fn run_on(l: &Learned, world: &World<'_>, f: &GridFields, prm: &Params) -> (Vec<flora::Plant>, [usize; 4]) {
    run_at(l, world, f, prm, &cal_on(l, world, f, prm).targets(), None)
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
    let targets = cal_on(&l, &world, &f, &prm).targets();
    let (plants, full) = run_at(&l, &world, &f, &prm, &targets, None);
    assert_eq!(full[0], 0, "no rock layer in this pass");
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
        alone.extend(run_at(&l, &world, &f, &prm, &targets, Some(&[tile])).0);
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
        rocks: Vec::new(),
        region_names: vec![String::new(), "Test Region".into()],
        scale: 10.0,
        stamped: false,
        towns: flora::towns::TownCensus::default(),
        clearings: flora::ClearingCensus::default(),
        product: flora::ProductUse::Off,
        calibration: None,
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
    let before = run_on(&l, &plain, &f, &prm).0;
    let with = World::new(&m, &regions, &t, Some(&c), &mask);
    let after = run_on(&l, &with, &f, &prm).0;
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
    let run = |prm: &Params| {
        let (plants, _, full, used, _) = flora::scatter_cached(d, &l, &world, &f, &water, &prior, None, prm).unwrap();
        (plants, full, used)
    };
    let cal = flora::calibration(&l, &world, &f, &water, &prior, None, &prm);
    let (cp, _, cf) = flora::scatter_tiles(&l, &world, &f, &water, &prior, None, &prm, &cal.targets(), None);
    let computed = (cp, cf);
    let (plants, full, used) = run(&prm);
    assert_eq!(used, ProductUse::Wrote, "no product yet");
    assert_eq!((plants.clone(), full), computed, "written: the passes' plants");
    let (again, full2, used) = run(&prm);
    assert_eq!(used, ProductUse::Read, "same inputs: read");
    assert_eq!((again, full2), computed, "read == computed");
    let inputs = flora::product_inputs(d, &world, &f, &l, &prior, None, &prm).unwrap();
    let (stored, won) = scatter_product::read(d).unwrap();
    assert_eq!(stored, inputs);
    assert_eq!(won, flora::winners(&l, &world, &f, &water, &prior, None, &prm, &cal.targets(), None), "the product is the kept set");
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
    assert_ne!(flora::product_inputs(d, &world, &f, &l, &prior, None, &prm).unwrap(), inputs, "a guide file");
    std::fs::remove_file(d.join("roads.json")).unwrap();
    assert_eq!(flora::product_inputs(d, &world, &f, &l, &prior, None, &prm).unwrap(), inputs);
    // A damaged product of these inputs is rebuilt.
    let bin = d.join(scatter_product::DIR).join("plants.bin");
    let mut bytes = std::fs::read(&bin).unwrap();
    bytes[9] ^= 0x40;
    std::fs::write(&bin, bytes).unwrap();
    assert!(matches!(run(&prm).2, ProductUse::Rebuilt(_)), "damaged: rebuilt");
    assert_eq!(run(&prm), (computed.0, computed.1, ProductUse::Read));
}

/// WG-E2p: the full radius at load. On the fixture's kept set, with a focus at the land's centre and a small budget:
/// the plants placed within the radius are the unbudgeted pass's there, the rest stays under the budget's remainder,
/// the radius is not in the product hash (a change reads it), and no focus places what the whole-world cut places.
#[test]
fn the_full_radius_places_everything_round_the_focus_at_load() {
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
    let run = |prm: &Params| {
        let (plants, _, _, used, _) = flora::scatter_cached(d, &l, &world, &f, &water, &prior, None, prm).unwrap();
        (plants, used)
    };
    let unbudgeted = Params { budget: f64::from(u32::MAX), ..prm };
    let (all, used) = run(&unbudgeted);
    assert_eq!(used, ProductUse::Wrote);
    let r = world.rect();
    let at = [f64::midpoint(r[0], r[2]), f64::midpoint(r[1], r[3])];
    let radius = (r[2] - r[0]).min(r[3] - r[1]) / 4.0;
    let near = |p: &flora::Plant| (p.pos[0] - at[0]).powi(2) + (p.pos[1] - at[1]).powi(2) <= radius * radius;
    let inside: Vec<_> = all.iter().filter(|p| near(p)).copied().collect();
    assert!(!inside.is_empty() && inside.len() < all.len() / 2, "the radius holds a part: {} of {}", inside.len(), all.len());
    let budget = (inside.len() / 2) as f64;
    let focused = Params { budget, full_radius_m: radius, focus: Some(at), ..prm };
    let (cut, used) = run(&focused);
    assert_eq!(used, ProductUse::Read, "the focus and radius are load values: the product is read");
    let cut_inside: Vec<_> = cut.iter().filter(|p| near(p)).copied().collect();
    assert_eq!(cut_inside, inside, "inside the radius == the unbudgeted pass");
    assert_eq!(cut.len(), inside.len(), "a budget under the inside set leaves the outside nothing");
    // A budget over the inside set: the total is the budget, the outside nests in the unbudgeted pass.
    let wide = Params { budget: (inside.len() + (all.len() - inside.len()) / 2) as f64, ..focused };
    let (more, _) = run(&wide);
    assert!((more.len() as f64 - wide.budget).abs() <= 4.0, "total {} at budget {}", more.len(), wide.budget);
    let keys = |v: &[flora::Plant]| v.iter().map(|p| p.key).collect::<std::collections::BTreeSet<_>>();
    assert!(keys(&more).is_subset(&keys(&all)) && keys(&cut).is_subset(&keys(&more)), "budgets nest");
    // No focus, or radius 0: the whole-world cut.
    let plain = Params { budget, ..prm };
    let (whole, _) = run(&plain);
    assert_eq!(run(&Params { focus: Some(at), full_radius_m: 0.0, ..plain }).0, whole);
    assert!(whole.iter().filter(|p| near(p)).count() < inside.len(), "the whole-world cut thins the focus too");
}

/// WG-E2o.3: the adapter's calibration is the class census of the guide's land rect (one row per class id in the
/// fields' order; area exact and mean equal to a brute pass over the same cells), it is written beside the product
/// and read back with it, and a class target change makes the product stale.
#[test]
fn the_calibration_is_the_guide_census_and_sits_beside_the_product() {
    use flora::ProductUse;
    use sf_worldgen::scatter::CandidateRules;
    use sf_worldgen::scatter_calib;
    use sf_worldgen::scatter_fields::{Fields, Suitability};
    use sf_worldgen::scatter_stand::{StandClass, StandRules};
    let m = mosaic();
    let regions = regions();
    let l = learned(&m);
    let t = tiles(&m);
    let towns = TownMask::default();
    let world = World::new(&m, &regions, &t, None, &towns);
    let f = fields(&world);
    let prm = Params::default();
    let (prior, water) = prior_water(&l, &prm);
    let cal = flora::calibration(&l, &world, &f, &water, &prior, None, &prm);
    // Brute census: the cell centres of `census_step_m` over the land rect, land only, a mosaic clearing w = 0.
    let file = &l.file;
    let names = f.class_names();
    let suit: Vec<Suitability> = file.species.iter().map(|s| Suitability::new(s, names)).collect();
    let canopy: Vec<Suitability> =
        file.species.iter().filter(|s| s.layer == Layer::Canopy).map(|s| Suitability::new(s, names)).collect();
    let n = names.len();
    let prior_at = |s: usize, x: f64, y: f64| {
        let Some(r) = world.region(x, y) else { return 0.0 };
        let c = f.sample(x, y).class.map_or(n, |k| usize::from(k).min(n));
        prior.get(s).and_then(|p| p.get(usize::from(r))).and_then(|p| p.get(c)).copied().unwrap_or(0.0)
    };
    let height = |x: f64, y: f64| world.height(x, y);
    let targets = cal.targets();
    let rules = CandidateRules {
        file,
        suitability: &suit,
        seed: prm.seed,
        tile_m: prm.tile_m,
        prior: &prior_at,
        water: &water,
        height: &height,
        rocks: None,
        canopy: &targets,
    };
    let stand = StandRules { values: &file.values, seed: prm.seed, canopy: &canopy };
    let step = file.values.census_step_m;
    let r = world.rect();
    let (nx, ny) = (((r[2] - r[0]) / step).floor() as usize, ((r[3] - r[1]) / step).floor() as usize);
    let (mut cells, mut sum, mut ws) = (vec![0u64; n], vec![0.0f64; n], vec![0.0; file.species.len()]);
    for j in 0..ny {
        for i in 0..nx {
            let (x, y) = (r[0] + (i as f64 + 0.5) * step, r[1] + (j as f64 + 0.5) * step);
            let Some(c) = f.sample(x, y).class.map(usize::from) else { continue };
            if world.excluded(x, y) || water.wet(x, y, world.height(x, y)) {
                continue;
            }
            cells[c] += 1;
            let d = stand.density(&f, x, y);
            if stand.class_of(d) != StandClass::Clearing {
                sum[c] += rules.canopy_weight(&f, x, y, d, &mut ws);
            }
        }
    }
    assert!(cells[0] > 0 && cells[1] > 0 && sum[0] > 0.0, "woodland and desert land, woodland canopy: {cells:?} {sum:?}");
    assert_eq!(cal.classes.len(), n, "one row per class id");
    for (c, row) in cal.classes.iter().enumerate() {
        assert_eq!(row.name, names[c], "class ids in the fields' order");
        assert_eq!(row.stems_per_ha.to_bits(), file.classes.get(&row.name).map_or(0.0, |e| e.v).to_bits(), "{}: T", row.name);
        assert_eq!(row.area_ha.to_bits(), (cells[c] as f64 * step * step / 10_000.0).to_bits(), "{}: A exact", row.name);
        let mean = if cells[c] == 0 { 0.0 } else { sum[c] / cells[c] as f64 };
        assert!((row.weight_mean - mean).abs() <= 1e-12 * mean.max(1.0), "{}: m {} vs brute {mean}", row.name, row.weight_mean);
    }
    // Written beside the product, read with it.
    let dir = tempfile::tempdir().unwrap();
    let d = dir.path();
    let (_, _, _, used, wrote) = flora::scatter_cached(d, &l, &world, &f, &water, &prior, None, &prm).unwrap();
    assert_eq!(used, ProductUse::Wrote, "no product yet");
    assert_eq!(wrote.map(|c| c.classes), Some(cal.classes.clone()), "the passes' calibration");
    assert!(d.join(scatter_product::DIR).join(scatter_calib::FILE).is_file(), "calibration.toml beside the product");
    let (_, _, _, used, read) = flora::scatter_cached(d, &l, &world, &f, &water, &prior, None, &prm).unwrap();
    assert_eq!(used, ProductUse::Read, "same inputs: read");
    assert_eq!(read.map(|c| c.classes), Some(cal.classes.clone()), "a read product has its calibration file");
    // A class target change: stale, recalibrated, rewritten.
    let mut moved = l.clone();
    moved.file.classes.get_mut("woodland").expect("a woodland target").v *= 2.0;
    let inputs = flora::product_inputs(d, &world, &f, &l, &prior, None, &prm).unwrap();
    assert_ne!(flora::product_inputs(d, &world, &f, &moved, &prior, None, &prm).unwrap(), inputs, "the target is hashed");
    let (_, _, _, used, again) = flora::scatter_cached(d, &moved, &world, &f, &water, &prior, None, &prm).unwrap();
    assert_eq!(used, ProductUse::Wrote, "a target change: stale");
    let again = again.expect("the passes' calibration");
    assert_eq!(again.classes[0].stems_per_ha.to_bits(), (2.0 * cal.classes[0].stems_per_ha).to_bits(), "the new target");
    assert_eq!(Calibration::read(d).unwrap().classes, again.classes, "the file follows the product");
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

/// WG-E2m: scatter v2 on the real 10× guide (`target/worldgen/x10s`, or `SCRIB_X10S`), copied to a temp folder with
/// its climate product made there when it has none: per biome class the land, stems/ha per layer (target: the class's
/// declared canopy stems/ha) and canopy crown cover (sum of crown discs over the class land), before the budget; then
/// the seconds of the first load (passes + product written) against the second (product read). No towns (as the
/// `worldgen-flora` census).
#[test]
#[ignore = "measurement: scatter v2 counts and seconds"]
fn scatter_v2_counts_and_seconds_on_x10s() {
    use std::collections::HashMap;
    use std::path::{Path, PathBuf};
    use std::time::Instant;

    let Ok((install, _)) = mw_install::MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return;
    };
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let src = std::env::var_os("SCRIB_X10S").map_or_else(|| root.join("target/worldgen/x10s"), PathBuf::from);
    if !src.join("heights.f32").exists() {
        eprintln!("no guide at {}; skipping", src.display());
        return;
    }
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path();
    let (climate, climate_s) = x10s_copy(&src, dir);

    let t = Instant::now();
    let vfs = mw_vfs::mount_install(&install, sf_vfs::Vfs::builder()).unwrap().vfs;
    let content = mw_records::load(&vfs, &install.game_files()).unwrap();
    let load_s = t.elapsed().as_secs_f64();
    let cache = std::cell::RefCell::new(HashMap::<String, Option<([f32; 3], [f32; 3])>>::new());
    let bounds = |model: &str| {
        if let Some(b) = cache.borrow().get(model) {
            return *b;
        }
        let b = sf_vfs::VfsPath::new(model)
            .ok()
            .and_then(|path| vfs.read(&path).ok())
            .and_then(|bytes| mw_nif::load(&bytes).ok())
            .and_then(|m| mw_nif::visible_bounds(&m));
        cache.borrow_mut().insert(model.to_owned(), b);
        b
    };
    let tiles = GuideTiles::open(dir, mw_world::ids::exterior()).unwrap();
    let source = mw_world::MwWorldSource::build(&content.db, &mw_world::SourceOptions::default());
    let prm = Params::default();
    let (file, f, water, clearings) = flora::guide_inputs(dir, &tiles, &prm).unwrap();
    let rock_source = flora::rocks::RockSource::read(dir).unwrap();
    let towns = TownMask::default();
    let pass = |prm: Params| {
        let t = Instant::now();
        let run = flora::scatter_world(&content.db, &source, &bounds, &tiles, &f, &water, &file, clearings.as_ref(), &towns, prm, Some(dir), Some(&rock_source))
            .unwrap();
        (run, t.elapsed().as_secs_f64())
    };
    let (first, first_s) = pass(prm);
    let (second, second_s) = pass(prm);
    // The densities before the budget: the product holds the kept set before the cut.
    let (full, full_s) = pass(Params { budget: f64::from(u32::MAX), ..prm });
    assert_eq!(full.product, flora::ProductUse::Read, "a budget change reads the product");
    assert_eq!(first.product, flora::ProductUse::Wrote, "a fresh copy has no product");
    assert_eq!(second.product, flora::ProductUse::Read, "same inputs: read");
    assert_eq!(first.plants, second.plants, "read == passes");
    assert!(first.plants.iter().all(|p| p.pos.iter().all(|v| v.is_finite()) && p.scale.is_finite() && p.scale > 0.0));

    println!("WG-E2m scatter v2 on {} (climate {climate} in {climate_s:.1} s, content {load_s:.1} s)", src.display());
    println!("first:  {}", flora::summary_line(&first));
    println!("second: {}", flora::summary_line(&second));
    println!("full:   {}", flora::summary_line(&full));
    println!("load seconds: first {first_s:.1} (passes + write), second {second_s:.1} (product read), no budget {full_s:.1}");
    let product_mb = std::fs::read_dir(dir.join("scatter"))
        .map_or(0, |d| d.filter_map(Result::ok).filter_map(|e| e.metadata().ok()).map(|m| m.len()).sum::<u64>())
        as f64
        / 1e6;
    println!("scatter/ product on disk: {product_mb:.0} MB");
    // WG-E2o.m: the calibration the first pass made (target T, reached W, scale g, ceiling flag per class).
    let cal = first.calibration.as_ref().expect("the passes' calibration");
    println!("| class | target /ha | land ha | weight mean | scale g | reached /ha | steps | ceiling | low weight |");
    println!("|---|---:|---:|---:|---:|---:|---:|---|---|");
    for c in cal.classes.iter().filter(|c| c.area_ha > 0.0) {
        println!(
            "| {} | {:.0} | {:.0} | {:.3} | {:.2} | {:.1} | {} | {} | {} |",
            c.name, c.stems_per_ha, c.area_ha, c.weight_mean, c.scale, c.reached_per_ha, c.steps, c.ceiling, c.low_weight
        );
    }
    for w in &cal.warnings {
        println!("calibration warning: {w}");
    }
    class_table(&full, &f);
}

/// Copies the guide files of `src` into `dir`, with its climate product when it has one, else makes it there;
/// returns how and the seconds it took.
pub(crate) fn x10s_copy(src: &std::path::Path, dir: &std::path::Path) -> (&'static str, f64) {
    use sf_worldgen::climate_map::{ClimateRequest, run_climate};
    use std::time::Instant;
    for e in std::fs::read_dir(src).unwrap() {
        let e = e.unwrap();
        if e.file_type().unwrap().is_file() {
            std::fs::copy(e.path(), dir.join(e.file_name())).unwrap();
        }
    }
    let t = Instant::now();
    let how = if src.join("climate").is_dir() {
        for e in std::fs::read_dir(src.join("climate")).unwrap() {
            let e = e.unwrap();
            std::fs::create_dir_all(dir.join("climate")).unwrap();
            std::fs::copy(e.path(), dir.join("climate").join(e.file_name())).unwrap();
        }
        "copied"
    } else {
        let req = ClimateRequest { base: dir.into(), params: None, classes: None, sets: Vec::new(), out: dir.into() };
        run_climate(&req).unwrap();
        "made"
    };
    let climate_s = t.elapsed().as_secs_f64();
    (how, climate_s)
}

/// Prints the per-class table of a run: land, stems/ha per layer against the class's canopy target, crown cover.
fn class_table(first: &FloraRun, f: &GridFields) {
    use sf_worldgen::scatter_fields::Fields;
    let names = f.class_names().to_vec();
    let n = names.len();
    let mut land = vec![0u64; n + 1];
    for a in &first.area {
        for (c, v) in a.iter().enumerate() {
            land[c] += v;
        }
    }
    let mut stems = vec![[0usize; 3]; n + 1];
    let mut crown_m2 = vec![0.0f64; n + 1];
    for p in &first.plants {
        let c = f.sample(p.pos[0], p.pos[1]).class.map_or(n, |k| usize::from(k).min(n));
        stems[c][p.key.layer as usize] += 1;
        if p.key.layer == Layer::Canopy {
            let s = &first.learned.file.species[p.key.species as usize];
            let r = s.crown_a_m + s.crown_b * p.size_m;
            crown_m2[c] += std::f64::consts::PI * r * r;
        }
    }
    println!("| class | land km2 | target canopy /ha | canopy /ha | understorey /ha | ground /ha | canopy crown cover |");
    println!("|---|---:|---:|---:|---:|---:|---:|");
    for c in 0..=n {
        if land[c] == 0 {
            continue;
        }
        let ha = land[c] as f64 * first.sample_m2 / 1e4;
        let name = names.get(c).map_or("none", String::as_str);
        let target = first.learned.file.classes.get(name).map_or(f64::NAN, |e| e.v);
        let per = |l: usize| stems[c][l] as f64 / ha;
        println!(
            "| {name} | {:.1} | {target:.0} | {:.1} | {:.1} | {:.1} | {:.1} % |",
            ha / 100.0,
            per(0),
            per(1),
            per(2),
            100.0 * crown_m2[c] / (ha * 1e4)
        );
    }
}

/// WG-E2n diagnosis: the canopy intensity of every class on x10s, stage by stage, against the spec's formula
/// (`worldgen-scatter.md` "Design" 3-6). On every 7th guide tile (16 m point grid on dry land): the parent thinning's
/// terms at the point (canopy suitability `su`, stand `D` (0 in a clearing), region prior), the mean-field candidate
/// intensity `parent_per_ha x offspring_mean x E[su^2 D prior]` (thinning `su D prior` at the parent, offspring mean
/// `x su`), then the canopy candidates the code emits in the tile and the winners of its competition (grown by the
/// canopy reach, kept inside the tile), each per hectare of the class's dry land, beside the declared
/// `class.<name>.stems_per_ha`. Last, the canopy count the class targets ask on the whole guide.
#[test]
#[ignore = "measurement: canopy density trace on x10s"]
#[allow(clippy::too_many_lines, clippy::cast_precision_loss)]
fn canopy_density_trace_on_x10s() {
    use std::collections::HashMap;
    use std::path::{Path, PathBuf};

    use sf_worldgen::scatter::{CandidateRules, Cover, compete};
    use sf_worldgen::scatter_fields::{Fields, Suitability};
    use sf_worldgen::scatter_stand::{StandClass, StandMap, StandRules};

    let Ok((install, _)) = mw_install::MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return;
    };
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let src = std::env::var_os("SCRIB_X10S").map_or_else(|| root.join("target/worldgen/x10s"), PathBuf::from);
    if !src.join("heights.f32").exists() {
        eprintln!("no guide at {}; skipping", src.display());
        return;
    }
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path();
    x10s_copy(&src, dir);
    let vfs = mw_vfs::mount_install(&install, sf_vfs::Vfs::builder()).unwrap().vfs;
    let content = mw_records::load(&vfs, &install.game_files()).unwrap();
    let cache = std::cell::RefCell::new(HashMap::<String, Option<([f32; 3], [f32; 3])>>::new());
    let bounds = |model: &str| {
        if let Some(b) = cache.borrow().get(model) {
            return *b;
        }
        let b = sf_vfs::VfsPath::new(model)
            .ok()
            .and_then(|path| vfs.read(&path).ok())
            .and_then(|bytes| mw_nif::load(&bytes).ok())
            .and_then(|m| mw_nif::visible_bounds(&m));
        cache.borrow_mut().insert(model.to_owned(), b);
        b
    };
    let tiles = GuideTiles::open(dir, mw_world::ids::exterior()).unwrap();
    let source = mw_world::MwWorldSource::build(&content.db, &mw_world::SourceOptions::default());
    let prm = Params::default();
    let (file, f, water, clearings) = flora::guide_inputs(dir, &tiles, &prm).unwrap();
    let towns = TownMask::default();
    // The learned rows, region prior and calibration of the run (the passes as the game runs them), with no budget:
    // its plants are the whole kept set (WG-E2o.d).
    let unbudgeted = Params { budget: f64::from(u32::MAX), ..prm };
    let run = flora::scatter_world(&content.db, &source, &bounds, &tiles, &f, &water, &file, clearings.as_ref(), &towns, unbudgeted, None, None)
        .unwrap();
    let m = crate::assemble(&content.db).unwrap();
    let regions = crate::region_map(&content.db, m.cells_min, m.cells_max).unwrap();
    let world = World::new(&m, &regions, &tiles, clearings.as_ref(), &towns);
    let lf = &run.learned.file;
    let names = f.class_names().to_vec();
    let n = names.len();
    let suit: Vec<Suitability> = lf.species.iter().map(|s| Suitability::new(s, &names)).collect();
    let canopy: Vec<Suitability> =
        lf.species.iter().filter(|s| s.layer == Layer::Canopy).map(|s| Suitability::new(s, &names)).collect();
    let cs = lf.species.iter().position(|s| s.layer == Layer::Canopy).expect("a canopy row");
    assert_eq!(canopy.len(), 1, "the trace follows the one canopy row");
    let sp = &lf.species[cs];
    let pmax = lf.values.region_prior_max;
    let class_of = |x: f64, y: f64| f.sample(x, y).class.map_or(n, |k| usize::from(k).min(n));
    let prior_at = |s: usize, x: f64, y: f64| {
        let Some(r) = world.region(x, y) else { return 0.0 };
        let c = class_of(x, y);
        run.prior.get(s).and_then(|p| p.get(usize::from(r))).and_then(|p| p.get(c)).copied().unwrap_or(0.0)
    };
    let height = |x: f64, y: f64| world.height(x, y);
    let targets = run.calibration.as_ref().expect("the passes' calibration").targets();
    let rules = CandidateRules {
        file: lf,
        suitability: &suit,
        seed: run.prm.seed,
        tile_m: run.prm.tile_m,
        prior: &prior_at,
        water: &water,
        height: &height,
        rocks: None,
        canopy: &targets,
    };
    let excluded = |x: f64, y: f64| world.excluded(x, y);
    let grow = rules.layer_grows().into_iter().find(|(l, _)| *l == Layer::Canopy).map(|(_, g)| g).unwrap();
    let sea = run.prm.sea_level_m;
    let t = run.prm.tile_m;
    // Per class: [points, su, D, prior, su^2 D prior, prior clamped, clearing, D/su where su > 0, su > 0 points,
    // candidates, winners, winner crown m2].
    let mut acc = vec![[0.0f64; 12]; n + 1];
    let step = 16.0;
    let per_side = (t / step) as usize;
    let all = rules.tiles(world.rect());
    let stride = 7;
    for &(tx, ty) in all.iter().step_by(stride) {
        let map = StandMap::build(
            StandRules { values: &lf.values, seed: run.prm.seed, canopy: &canopy },
            &f,
            &excluded,
            rules.tile_rect((tx, ty)),
        );
        let sq = [f64::from(tx) * t, f64::from(ty) * t, f64::from(tx + 1) * t, f64::from(ty + 1) * t];
        for j in 0..per_side {
            for i in 0..per_side {
                let (x, y) = (sq[0] + (i as f64 + 0.5) * step, sq[1] + (j as f64 + 0.5) * step);
                let h = world.height(x, y);
                if h <= sea || world.region(x, y).is_none() || water.wet(x, y, h) || world.excluded(x, y) {
                    continue;
                }
                let a = &mut acc[class_of(x, y)];
                let su = suit[cs].at(&f.sample(x, y));
                let p = map.at(x, y);
                let d = if p.class == StandClass::Clearing { 0.0 } else { p.d };
                let pr = prior_at(cs, x, y);
                a[0] += 1.0;
                a[1] += su;
                a[2] += d;
                a[3] += pr.clamp(0.0, pmax);
                a[4] += su * su * d * pr.clamp(0.0, pmax);
                a[5] += f64::from(u8::from(pr > pmax));
                a[6] += f64::from(u8::from(p.class == StandClass::Clearing));
                if su > 0.0 {
                    a[7] += p.d / su;
                    a[8] += 1.0;
                }
            }
        }
        let g = [sq[0] - grow, sq[1] - grow, sq[2] + grow, sq[3] + grow];
        let cands = rules.layer_candidates(&f, &map, g, Layer::Canopy, &Cover::new(lf.values.shade_per_crown));
        let won = compete(lf, &cands, lf.values.competition_rounds);
        for (c, w) in cands.iter().zip(won) {
            let [x, y] = c.pos;
            if x < sq[0] || x >= sq[2] || y < sq[1] || y >= sq[3] {
                continue;
            }
            let a = &mut acc[class_of(x, y)];
            a[9] += 1.0;
            if w {
                a[10] += 1.0;
                a[11] += std::f64::consts::PI * c.crown_m.powi(2);
            }
        }
    }
    let ceiling = sp.parent_per_ha * sp.offspring_mean;
    println!(
        "WG-E2n canopy trace on {} ({} of {} tiles, {step} m points on dry land; row {:?}: parent_per_ha {}, \
         offspring_mean {}, background {}, cluster_sigma {} m, crown {} + {} h, crown_k {}, size {}-{} m, \
         region_prior_max {pmax}; ceiling parent_per_ha x offspring_mean = {ceiling}/ha)",
        src.display(),
        all.len().div_ceil(stride),
        all.len(),
        sp.name,
        sp.parent_per_ha,
        sp.offspring_mean,
        sp.background_fraction,
        sp.cluster_sigma_m,
        sp.crown_a_m,
        sp.crown_b,
        sp.crown_k,
        sp.size_min_m,
        sp.size_max_m,
    );
    println!(
        "| class | dry km2 sampled | target /ha | E[su] | E[D] | E[noise] = E[D/su] | clearing % | E[prior] | prior > max % \
         | expected cands /ha | candidates /ha | winners /ha | crown cover |"
    );
    println!("|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|");
    for (c, a) in acc.iter().enumerate() {
        if a[0] == 0.0 {
            continue;
        }
        let ha = a[0] * step * step / 1e4;
        let name = names.get(c).map_or("none", String::as_str);
        let target = lf.classes.get(name).map_or(f64::NAN, |e| e.v);
        let mean = |k: usize| a[k] / a[0];
        println!(
            "| {name} | {:.1} | {target:.0} | {:.2} | {:.2} | {:.2} | {:.0} % | {:.2} | {:.0} % | {:.1} | {:.1} | {:.1} | {:.1} % |",
            ha / 100.0,
            mean(1),
            mean(2),
            if a[8] > 0.0 { a[7] / a[8] } else { f64::NAN },
            100.0 * mean(6),
            mean(3),
            100.0 * mean(5),
            ceiling * mean(4),
            a[9] / ha,
            a[10] / ha,
            100.0 * a[11] / (ha * 1e4),
        );
    }
    // The canopy the class targets ask on the whole guide (land samples of the run's census).
    let mut land = vec![0u64; n + 1];
    for a in &run.area {
        for (c, v) in a.iter().enumerate() {
            land[c] += v;
        }
    }
    let mut target_total = 0.0;
    for (c, &l) in land.iter().enumerate() {
        let name = names.get(c).map_or("none", String::as_str);
        target_total += lf.classes.get(name).map_or(0.0, |e| e.v) * l as f64 * run.sample_m2 / 1e4;
    }
    let land_ha = land.iter().sum::<u64>() as f64 * run.sample_m2 / 1e4;
    println!(
        "whole guide: land {:.0} km2; canopy at the class targets {:.2} M ({:.0}/ha); kept before the budget: {}; \
         scatter.budget {} over the world",
        land_ha / 100.0,
        target_total / 1e6,
        target_total / land_ha,
        (0..4).map(|g| format!("{} {}", flora::BUDGET_GROUPS[g], run.full[g])).collect::<Vec<_>>().join(", "),
        prm.budget
    );
    calibration_gap(&run, &world, &f, &water, &rules, &canopy, &excluded, &land);
}

/// WG-E2o.d diagnosis, WG-E2o.f check: the calibration's `reached` against the world's canopy winners at the same
/// `g`. Per class with a target: its calibration sample (WG-E2o.h: `scatter_calib::first_tiles` of the calibration's
/// sized `n` on the same census and world order; the pilot's tiles, `N(c)`, `CV_r` and `rse` beside it), its tiles,
/// land, share and weight mean `m_t` beside `m`; `reached` (the last joint check's `W`); the
/// passes' class winners on the same tiles (built alone, [`flora::winners`]) and the world run's (after pose), per ha
/// of that land; the world's class winners per ha of the census land (every tile; E2o.f: `reached` == this within
/// `calib_tol` for a class not flagged) and of the guide land (`run.area`, the E2m table's denominator, which keeps
/// rivers, lakes and the excluded land). Last, the calibration's seconds and joint checks.
#[allow(clippy::too_many_arguments, clippy::cast_precision_loss)]
fn calibration_gap(
    run: &FloraRun,
    world: &World<'_>,
    f: &GridFields,
    water: &WaterMask,
    rules: &sf_worldgen::scatter::CandidateRules<'_>,
    canopy: &[sf_worldgen::scatter_fields::Suitability],
    excluded: &(dyn Fn(f64, f64) -> bool + Sync),
    guide_land: &[u64],
) {
    use std::collections::{BTreeMap, BTreeSet};

    use sf_worldgen::scatter_calib;
    use sf_worldgen::scatter_fields::Fields;
    use sf_worldgen::scatter_stand::StandRules;

    let lf = &run.learned.file;
    let cal = run.calibration.as_ref().expect("the passes' calibration");
    let targets = cal.targets();
    let stand = StandRules { values: &lf.values, seed: run.prm.seed, canopy };
    let census = scatter_calib::census(rules, &stand, f, excluded, world.rect());
    let order = scatter_calib::tile_order(&census, rules.seed);
    let n = f.class_names().len();
    let t = run.prm.tile_m;
    let tile_of = |x: f64, y: f64| ((x / t).floor() as i32, (y / t).floor() as i32);
    let class_of = |x: f64, y: f64| f.sample(x, y).class.map_or(n, |k| usize::from(k).min(n));
    // The world run's canopy winners (after pose) per tile and class.
    let mut world_won: BTreeMap<(i32, i32), Vec<u64>> = BTreeMap::new();
    let mut world_class = vec![0u64; n + 1];
    for p in run.plants.iter().filter(|p| p.key.layer == Layer::Canopy) {
        let c = class_of(p.pos[0], p.pos[1]);
        world_class[c] += 1;
        world_won.entry(tile_of(p.pos[0], p.pos[1])).or_insert_with(|| vec![0; n + 1])[c] += 1;
    }
    println!(
        "WG-E2o.h calibration vs world (scatter.calib_area_ha {}, calib_tiles_min {}, calib_tiles_max {}, calib_z {}, \
         census step {} m; W per ha; reached = the last joint check; alone = the passes on the sample built alone; \
         world = the full run after pose)",
        lf.values.calib_area_ha, lf.values.calib_tiles_min, lf.values.calib_tiles_max, lf.values.calib_z, census.step_m
    );
    println!(
        "| class | T | g | pilot | tiles | N | CV_r pilot | CV_r | rse | capped | calib land ha | class share | m | m_t | \
         reached | alone | world on tiles | world, census land | reached / world | world, guide land | steps | reopened | \
         ceiling |"
    );
    println!("|---|---:|---:|---:|---:|---:|---:|---:|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|");
    for (c, row) in cal.classes.iter().enumerate() {
        if row.stems_per_ha <= 0.0 || row.weight_mean <= 0.0 || row.area_ha <= 0.0 {
            continue;
        }
        let s = scatter_calib::first_tiles(&census, &order, c, row.calib_tiles as usize);
        assert!((census.area_ha(c) - row.area_ha).abs() < 1e-6, "{}: the replayed census is the calibration's", row.name);
        assert!((s.land_ha - row.calib_area_ha).abs() < 1e-6, "{}: the replayed sample is the calibration's", row.name);
        assert_eq!(s.tiles.len(), row.calib_tiles as usize, "{}: the replayed sample's tiles", row.name);
        let all: u64 = s.tiles.iter().map(|k| census.tiles[k].0).sum();
        let share = if all == 0 { 0.0 } else { s.land_ha / (all as f64 * census.step_m * census.step_m / 1e4) };
        let set: BTreeSet<(i32, i32)> = s.tiles.iter().copied().collect();
        let alone = flora::winners(&run.learned, world, f, water, &run.prior, None, &run.prm, &targets, Some(&s.tiles));
        let alone_n = alone
            .plants
            .iter()
            .filter(|p| p.key.layer == Layer::Canopy && class_of(p.pos[0], p.pos[1]) == c)
            .filter(|p| set.contains(&tile_of(p.pos[0], p.pos[1])))
            .count();
        let on_tiles: u64 = s.tiles.iter().filter_map(|k| world_won.get(k)).map(|v| v[c]).sum();
        let guide_ha = guide_land[c] as f64 * run.sample_m2 / 1e4;
        let per = |k: f64| if s.land_ha > 0.0 { k / s.land_ha } else { f64::NAN };
        let world_census = world_class[c] as f64 / row.area_ha;
        println!(
            "| {} | {:.0} | {:.2} | {} | {} | {} | {:.3} | {:.3} | {:.4} | {} | {:.0} | {:.0} % | {:.3} | {:.3} | {:.1} | {:.1} | \
             {:.1} | {:.1} | {:.2} | {:.1} | {} | {} | {} |",
            row.name,
            row.stems_per_ha,
            row.scale,
            row.pilot_tiles,
            s.tiles.len(),
            row.class_tiles,
            row.pilot_cv_r,
            row.cv_r,
            row.rse,
            row.capped,
            s.land_ha,
            100.0 * share,
            row.weight_mean,
            s.weight_mean,
            row.reached_per_ha,
            per(alone_n as f64),
            per(on_tiles as f64),
            world_census,
            row.reached_per_ha / world_census,
            world_class[c] as f64 / guide_ha,
            row.steps,
            row.reopened,
            row.ceiling,
        );
    }
    let checks = cal.classes.iter().map(|c| c.joint_checks).max().unwrap_or(0);
    println!("calibration: {:.1} s, {checks} joint checks", cal.seconds);
}

/// WG-E2q: the stand cap at load. On the fixture's kept set, a cap at half the densest tile's canopy leaves no tile
/// above `round(cap x tile area)`, keeps the lower layers and a subset of the canopy, and is not in the product hash
/// (a change reads it); a cap above every tile (2000/ha) places what no cap places.
#[test]
fn the_stand_cap_thins_the_densest_tiles_at_load() {
    use flora::ProductUse;
    use std::collections::{BTreeMap, BTreeSet};
    let m = mosaic();
    let regions = regions();
    let l = learned(&m);
    let t = tiles(&m);
    let towns = TownMask::default();
    let world = World::new(&m, &regions, &t, None, &towns);
    let f = fields(&world);
    let prm = Params { budget: f64::from(u32::MAX), ..Params::default() };
    let prior = flora::region_prior(&l.density, &[vec![0; 3], vec![1, 0, 0]]);
    let water = WaterMask::sea(0.0, prm.water_margin_m);
    let dir = tempfile::tempdir().unwrap();
    let d = dir.path();
    let run = |prm: &Params| {
        let (plants, _, _, used, _) = flora::scatter_cached(d, &l, &world, &f, &water, &prior, None, prm).unwrap();
        (plants, used)
    };
    let (open, used) = run(&Params { canopy_stems_per_ha_max: f64::MAX, ..prm });
    assert_eq!(used, ProductUse::Wrote);
    let per_tile = |v: &[flora::Plant]| {
        let mut n: BTreeMap<(i64, i64), usize> = BTreeMap::new();
        for p in v.iter().filter(|p| p.key.layer == Layer::Canopy) {
            *n.entry(((p.pos[0] / prm.tile_m).floor() as i64, (p.pos[1] / prm.tile_m).floor() as i64)).or_default() += 1;
        }
        n
    };
    let densest = per_tile(&open).into_values().max().unwrap_or(0);
    assert!(densest >= 4, "the fixture has a canopy stand: {densest} in its densest tile");
    let area_ha = prm.tile_m * prm.tile_m / 10_000.0;
    let limit = densest / 2;
    let capped = Params { canopy_stems_per_ha_max: limit as f64 / area_ha, ..prm };
    let (cut, used) = run(&capped);
    assert_eq!(used, ProductUse::Read, "the cap is a load value: the product is read");
    assert!(per_tile(&cut).into_values().all(|n| n <= limit), "no tile above {limit}");
    let keys = |v: &[flora::Plant], canopy: bool| {
        v.iter().filter(|p| (p.key.layer == Layer::Canopy) == canopy).map(|p| p.key).collect::<BTreeSet<_>>()
    };
    assert_eq!(keys(&cut, false), keys(&open, false), "lower layers never capped");
    assert!(keys(&cut, true).len() < keys(&open, true).len() && keys(&cut, true).is_subset(&keys(&open, true)));
    assert_eq!(run(&Params { canopy_stems_per_ha_max: 2000.0, ..prm }).0, open, "a cap above every tile is no cap");
}

// WG-E3f.3: the game's rock ground. The fixture's geology product: all six geology.toml units, `tuff` west of col 32,
// `basalt_flow` from it; one mapped cliff column at col 34 (on the 45 deg ramp, below the scarp slope, so only the product makes it a
// scarp; its talus runs out west onto the flat).

const CLIFF_COL: usize = 34;

fn rock_scratch(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("mw_worldgen_rock_ground_{}_{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// Writes the fixture's geology product for `world` into `dir`.
fn write_geology(world: &World<'_>, dir: &std::path::Path) {
    use sf_worldgen::geology::{GeologyMap, GeologyParams, declared_defaults, f16_bits};
    let table = geology_table();
    let idx = |n: &str| u8::try_from(table.units.iter().position(|u| u.name == n).unwrap()).unwrap();
    let g = &world.guide;
    let n = g.w * g.h;
    let map = GeologyMap {
        w: g.w,
        h: g.h,
        spacing_m: g.spacing,
        origin_m: g.origin,
        params: GeologyParams::from_declared(&declared_defaults()).unwrap(),
        units: table.units.clone(),
        unit: (0..n).map(|k| if k % g.w < 32 { idx("tuff") } else { idx("basalt_flow") }).collect(),
        hardness: vec![0; n],
        datum: vec![0.0; n],
        cliff: (0..n).map(|k| u8::from(k % g.w == CLIFF_COL)).collect(),
        talus: vec![0; n],
        human: vec![0; n],
        lip: vec![f16_bits(0.0); n],
        feature: vec![0; n],
    };
    map.write(dir, &declared_defaults()).unwrap();
}

/// The product's unit and cliff at every sample (and half a sample off it), nothing outside the guide; a tampered or
/// misframed product is refused, a folder without one reads `None`; slope and aspect from the guide heights.
#[test]
fn rock_ground_reads_the_geology_product_unit_and_cliff() {
    use flora::rocks::{Ground, guide_geology};
    use sf_worldgen::scatter_rocks::RockGround;
    let (m, regions) = (mosaic(), regions());
    let t = tiles(&m);
    let towns = TownMask::default();
    let world = World::new(&m, &regions, &t, None, &towns);
    let dir = rock_scratch("product");
    assert!(guide_geology(&dir).unwrap().is_none());
    write_geology(&world, &dir);
    let geo = guide_geology(&dir).unwrap().unwrap();
    let ground = Ground::new(&world, Some(&geo), &geology_table()).unwrap();
    let g = &world.guide;
    for r in 0..g.h {
        for c in 0..g.w {
            let (x, y) = (g.origin[0] + c as f64 * g.spacing, g.origin[1] - r as f64 * g.spacing);
            let want = if c < 32 { "tuff" } else { "basalt_flow" };
            for (dx, dy) in [(0.0, 0.0), (0.45, -0.45), (-0.45, 0.45)] {
                let (px, py) = (x + dx * g.spacing, y + dy * g.spacing);
                assert_eq!(ground.unit(px, py), Some(want), "({c}, {r})");
                assert_eq!(ground.cliff(px, py), c == CLIFF_COL, "({c}, {r})");
            }
        }
    }
    let [x0, y0, x1, y1] = world.rect();
    assert_eq!(ground.unit(x0 - g.spacing, y0), None);
    assert_eq!(ground.unit(x1, y1 + g.spacing), None);
    assert!(!ground.cliff(x1 + g.spacing, y1));
    // Flat west of col 32, the 45 deg ramp rising east beyond: downslope west.
    let at = |c: f64| ground.slope_aspect_deg(g.origin[0] + c * g.spacing, g.origin[1] - 30.0 * g.spacing);
    assert_eq!(at(20.0), (0.0, 0.0));
    let (s, a) = at(45.0);
    assert!((s - 45.0).abs() < 1e-3 && (a - 270.0).abs() < 1e-9, "{s} {a}");
    // Misframed: one sample wider.
    let mut wide = geo.clone();
    wide.w += 1;
    assert!(Ground::new(&world, Some(&wide), &geology_table()).is_err());
    // Tampered unit layer.
    let path = dir.join("geology").join("unit.u8");
    let mut bytes = std::fs::read(&path).unwrap();
    bytes[0] ^= 1;
    std::fs::write(&path, bytes).unwrap();
    assert!(guide_geology(&dir).is_err());
    let _ = std::fs::remove_dir_all(&dir);
}

/// Without a product: no cliff anywhere, and the region's largest share as its unit (the table's default shares for a
/// region without a row; ties to the first name); nothing outside the guide.
#[test]
fn rock_ground_without_a_product_takes_the_region_default_unit() {
    use flora::rocks::Ground;
    use sf_worldgen::geology::{GeologyTable, RegionShares};
    use sf_worldgen::scatter_rocks::RockGround;
    let m = mosaic();
    let t = tiles(&m);
    let towns = TownMask::default();
    let check = |rm: &RegionMap, table: &GeologyTable, want: Option<&str>| {
        let world = World::new(&m, rm, &t, None, &towns);
        let ground = Ground::new(&world, None, table).unwrap();
        let g = &world.guide;
        for (c, r) in [(10.0, 10.0), (40.0, 50.0), (64.0, 0.0)] {
            let (x, y) = (g.origin[0] + c * g.spacing, g.origin[1] - r * g.spacing);
            assert_eq!(ground.unit(x, y), want, "({c}, {r})");
            assert!(!ground.cliff(x, y));
        }
        assert_eq!(ground.unit(g.origin[0] - 2.0 * g.spacing, g.origin[1]), None);
    };
    let mut table = geology_table();
    let mut rm = regions();
    // The fixture region has no row: the default shares' largest (first name on a tie).
    let mut best: Option<(&String, f64)> = None;
    for (u, &v) in &table.shares {
        if best.is_none_or(|(_, b)| v > b) {
            best = Some((u, v));
        }
    }
    let best = best.map(|(u, _)| u.clone());
    check(&rm, &table, best.as_deref());
    // Its own row.
    rm.legend[1].name = "Test Region".into();
    let row = |a: f64, b: f64| RegionShares {
        shares: [("tephra".to_string(), a), ("alluvium".to_string(), b)].into_iter().collect(),
    };
    table.region.insert("Test Region".into(), row(0.4, 0.6));
    check(&rm, &table, Some("alluvium"));
    table.region.insert("Test Region".into(), row(0.5, 0.5));
    check(&rm, &table, Some("alluvium"));
    table.region.insert("Test Region".into(), row(0.0, 0.0));
    check(&rm, &table, None);
}

/// The rock candidates of each context on the game's ground, one rect alone, are the full run's inside it.
#[test]
fn rock_ground_candidates_tile_alone() {
    use flora::rocks::{Ground, guide_geology};
    use sf_worldgen::scatter_rocks::{RockCandidate, RockRules, RockValues};
    let (m, regions) = (mosaic(), regions());
    let t = tiles(&m);
    let towns = TownMask::default();
    let world = World::new(&m, &regions, &t, None, &towns);
    let dir = rock_scratch("alone");
    write_geology(&world, &dir);
    let geo = guide_geology(&dir).unwrap().unwrap();
    let ground = Ground::new(&world, Some(&geo), &geology_table()).unwrap();
    let file = shipped();
    let f = fields(&world);
    let rules = RockRules { file: &file, values: RockValues::default(), seed: 5, tile_m: 256.0, prior: None };
    let full = world.rect();
    let g = &world.guide;
    let col = |c: f64| g.origin[0] + c * g.spacing;
    let row = |r: f64| g.origin[1] - r * g.spacing;
    let rects = [[col(12.0), full[1], col(30.5), full[3]], [col(33.0), row(40.0), col(60.0), row(10.25)]];
    let run = |name: &str, rect: [f64; 4]| match name {
        "talus" => rules.talus_candidates(&ground, &f, rect),
        "outcrop" => rules.outcrop_candidates(&ground, &f, rect),
        _ => rules.field_candidates(&ground, &f, rect),
    };
    for name in ["talus", "outcrop", "field"] {
        let all = run(name, full);
        assert!(!all.is_empty(), "{name}");
        for rect in rects {
            let inside: Vec<RockCandidate> = all
                .iter()
                .filter(|c| c.pos[0] >= rect[0] && c.pos[0] < rect[2] && c.pos[1] >= rect[1] && c.pos[1] < rect[3])
                .copied()
                .collect();
            assert_eq!(run(name, rect), inside, "{name} {rect:?}");
        }
    }
    let _ = std::fs::remove_dir_all(&dir);
}

/// WG-E3f.4: the rock layer in the passes on the fixture's geology product (tuff west of col 32, `basalt_flow` from it,
/// a mapped cliff column), with the learned rock model widened to fit field stones (axis 6 m at 0.05-0.4: 0.3-2.4 m).
/// Every tile alone gives the whole run's kept rocks and the same placed rocks; the rocks are budget group 0 and
/// counted there; a pack-model share of 0 places no pack model (the default share places some); a model rock copies
/// its learned reference and sits at its learned offset; a library rock stands on the ground.
#[test]
fn rock_layer_in_the_passes_tile_alone_share_zero_and_groups() {
    use flora::rocks::{Ground, RockMesh, Rocks, guide_geology, pass_prior, rock_placements, values_of};
    use sf_worldgen::scatter::LayerWinners;
    use sf_worldgen::scatter_species::RockContext;
    let (m, regions) = (mosaic(), regions());
    let mut l = learned(&m);
    l.rocks[0].scale = [0.05, 0.4];
    let table = geology_table();
    assert!(flora::rocks::rock_entries(&mut l, &regions, &table) > 0);
    let t = tiles(&m);
    let towns = TownMask::default();
    let world = World::new(&m, &regions, &t, None, &towns);
    let dir = rock_scratch("wired");
    write_geology(&world, &dir);
    let geo = guide_geology(&dir).unwrap().unwrap();
    let ground = Ground::new(&world, Some(&geo), &table).unwrap();
    let area = vec![vec![0u64; 3], vec![1, 0, 0]];
    let rock_prior = pass_prior(&l, &area);
    assert_eq!(rock_prior.as_deref(), Some(&[0.0, 1.0][..]), "one region: prior 1 where the rocks were learned");
    let rocks = Rocks { ground: &ground, values: values_of(&l.file).unwrap(), prior: rock_prior.as_deref(), table: &table };
    let f = fields(&world);
    let prm = Params::default();
    let (prior, water) = prior_water(&l, &prm);
    let targets = flora::calibration(&l, &world, &f, &water, &prior, Some(&rocks), &prm).targets();
    let won = flora::winners(&l, &world, &f, &water, &prior, Some(&rocks), &prm, &targets, None);
    let (plants, placed, full) = flora::place(&l, &world, Some(&rocks), &prm, &won);
    let ctx = |c: RockContext| won.rocks.iter().filter(|r| r.key.context == c).count();
    let (talus, field) = (ctx(RockContext::Talus), ctx(RockContext::Field));
    println!("kept rocks {} (talus {talus}, outcrop {}, field {field}), placed {}", won.rocks.len(), ctx(RockContext::Outcrop), placed.len());
    assert!(talus > 0 && field > 0, "talus below the mapped cliff and field stones on the flat");
    assert_eq!(full[0], won.rocks.len(), "the kept rocks are budget group 0");
    assert_eq!(placed.len(), full[0], "the default budget keeps every rock, and every unit has an entry per context");
    // Tile alone == whole, in any order: the kept rocks and their placed mesh and pose.
    let rect = world.rect();
    let tm = prm.tile_m;
    let (x0, y0) = ((rect[0] / tm).floor() as i32, (rect[1] / tm).floor() as i32);
    let (x1, y1) = ((rect[2] / tm).ceil() as i32, (rect[3] / tm).ceil() as i32);
    let mut alone = Vec::new();
    for y in (y0..y1).rev() {
        for x in (x0..x1).rev() {
            alone.extend(flora::winners(&l, &world, &f, &water, &prior, Some(&rocks), &prm, &targets, Some(&[(x, y)])).rocks);
        }
    }
    // The run concatenates its tiles (a talus block may land outside its parent's tile): compare by key.
    let mut whole = won.rocks.clone();
    whole.sort_unstable_by_key(|c| c.key);
    alone.sort_unstable_by_key(|c| c.key);
    assert_eq!(alone, whole, "every tile alone == the whole run's rocks");
    let only = |rocks: Vec<_>| LayerWinners { rocks, plants: Vec::new() };
    let mut by_key = placed.clone();
    by_key.sort_unstable_by_key(|r| r.key);
    assert_eq!(flora::place(&l, &world, Some(&rocks), &prm, &only(alone)).1, by_key, "the same placed rocks");
    // Pack-model share: some models at the default, none at 0.
    let models = |p: &[flora::rocks::Rock]| p.iter().filter(|r| matches!(r.mesh, RockMesh::Model(_))).count();
    assert!(models(&placed) > 0 && models(&placed) < placed.len(), "both kinds at share {}", prm.rock_pack_model_share);
    let none = Params { rock_pack_model_share: 0.0, ..prm };
    let library = flora::place(&l, &world, Some(&rocks), &none, &won).1;
    assert_eq!(library.len(), placed.len());
    assert_eq!(models(&library), 0, "share 0: library only");
    for r in &placed {
        let h = world.height(r.pos[0], r.pos[1]);
        match r.mesh {
            RockMesh::Model(i) => {
                assert_eq!(i, 0);
                assert!(r.scale >= 0.05 - 1e-6 && r.scale <= 0.4 + 1e-6, "{r:?}");
                let want = h + f64::from(l.rocks[0].offset_m) * f64::from(r.scale);
                assert!((r.pos[2] - want).abs() < 1e-6, "a model at its learned offset: {r:?}");
            }
            RockMesh::Class { .. } => assert!((r.pos[2] - h).abs() < 1e-9 && r.sink >= 0.0, "{r:?}"),
        }
    }
    // Counted per group: the run's summary and placements.
    let run = FloraRun {
        prm,
        learned: l.clone(),
        prior: prior.clone(),
        area,
        sample_m2: 1.0,
        full,
        plants: plants.clone(),
        rocks: placed.clone(),
        region_names: vec![String::new(), "Test Region".into()],
        scale: 10.0,
        stamped: false,
        towns: flora::towns::TownCensus::default(),
        clearings: flora::ClearingCensus::default(),
        product: flora::ProductUse::Off,
        calibration: None,
        seconds: [0.0; 4],
    };
    let line = flora::summary_line(&run);
    let n = models(&placed);
    assert!(line.starts_with(&format!("{} placements (rocks {} of {}, ", plants.len() + placed.len(), placed.len(), full[0])), "{line}");
    assert!(line.contains(&format!("rocks {n} pack models / {} library", placed.len() - n)), "{line}");
    let p = rock_placements(&run);
    assert_eq!(p.len(), n, "one placement per pack-model rock");
    let rep = &l.reps[l.rocks[0].rep];
    assert!(p.iter().all(|q| q.base == rep.base && q.model == rep.model && q.class == rep.class), "copies of the reference");
    let mut ids: Vec<_> = p.iter().map(|q| q.id).chain(flora::rocks::library_rocks(&run).iter().map(|q| q.id)).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), placed.len(), "rock ids are unique");
    library_placements_on_the_run(&run);
    let _ = std::fs::remove_dir_all(&dir);
}

/// RG-P2 on a fixture run: no libraries drop every library rock; one variant per used class places each, ids in order.
fn library_placements_on_the_run(run: &FloraRun) {
    let lib_rocks = flora::rocks::library_rocks(run);
    assert!(!lib_rocks.is_empty(), "the fixture run has library rocks");
    let none = flora::rocks::library_placements(run, &sf_rockgen::Libraries::default());
    assert_eq!((none.placements.len(), none.dropped), (0, lib_rocks.len()), "no library: all dropped");
    let classes: std::collections::BTreeSet<_> = lib_rocks.iter().map(|r| r.class.clone()).collect();
    let libs = sf_rockgen::Libraries::new(classes.into_iter().map(|c| (c, rock_library(1.0, &[[1.0, 0.8, 0.5]]))));
    let all = flora::rocks::library_placements(run, &libs);
    assert_eq!(all.dropped, 0);
    assert_eq!(all.placements.iter().map(|p| p.id).collect::<Vec<_>>(), lib_rocks.iter().map(|r| r.id).collect::<Vec<_>>());
}

/// A rock library of size mark `size_m` whose variants have LOD0 boxes from the origin to each `max` shifted by -0.25
/// on y (off-centre, so the axis conversion is visible in the bounds).
fn rock_library(size_m: f32, maxes: &[[f32; 3]]) -> sf_rockgen::Library {
    let variants = maxes
        .iter()
        .zip(1u64..)
        .map(|(m, seed)| sf_rockgen::Variant {
            seed,
            lods: sf_rockgen::RockLods {
                lods: Vec::new(),
                bounds: sf_rockgen::Aabb { min: [-m[0] * 0.5, -0.25, -m[2] * 0.5], max: [m[0] * 0.5, m[1] - 0.25, m[2] * 0.5] },
                hash: seed,
            },
            hull: None,
        })
        .collect();
    sf_rockgen::Library { key: 7, size_m, variants }
}

/// RG-P2: library rocks become `ROCK_LIBRARY` placements with their own ids, the variant `hash mod n` as model id,
/// scale mark / size and the lift along the rotated up, exactly; one model info per used variant with its LOD0 bounds
/// in the engine frame; rocks of an unknown class or an empty library are dropped and counted; one base per class.
#[test]
#[allow(clippy::float_cmp)] // exact: the spec's formula, evaluated the same way
fn library_rocks_become_placements_with_model_infos() {
    use flora::rocks::{LIBRARY_BASE_KIND, RockPlacement, engine_bounds, resolve_library_rocks};
    use mw_world::source::ROCK_LIBRARY;
    use sf_coords::{WorldPos, WorldTransform};
    use sf_rockgen::placed::parse_model_id;
    use sf_world::PlacementId;
    let maxes = [[2.0, 1.5, 0.8], [2.0, 1.2, 1.0], [1.6, 2.0, 0.5]];
    let libs = sf_rockgen::Libraries::new([
        ("talus".to_owned(), rock_library(2.0, &maxes)),
        ("slab".to_owned(), sf_rockgen::Library { key: 8, size_m: 1.0, variants: Vec::new() }),
    ]);
    let rotation = glam::Quat::from_rotation_x(0.3) * glam::Quat::from_rotation_y(1.1);
    let rock = |n: u8, class: &str, variant: u64, mark: f32, sink: f32| RockPlacement {
        id: PlacementId::from_key("test.rocks", &[n]),
        class: class.to_owned(),
        variant,
        transform: WorldTransform {
            pos: WorldPos::from_f64(glam::DVec3::new(100.0 + f64::from(n), 5.0, -40.0)),
            rotation,
            scale: Vec3::splat(mark),
        },
        sink,
        regime: None,
    };
    let rocks = [
        rock(0, "talus", 0, 1.0, 0.2),
        rock(1, "talus", 4, 3.0, 0.5),
        rock(2, "slab", 1, 1.0, 0.1),
        rock(3, "talus", 5, 0.5, 0.7),
        rock(4, "nosuch", 2, 1.0, 0.1),
        rock(5, "talus", 9, 2.0, 0.0),
    ];
    let out = resolve_library_rocks(&rocks, &libs);
    assert_eq!(out.dropped, 2, "the empty library and the unknown class");
    let kept: Vec<_> = rocks.iter().filter(|r| r.class == "talus").collect();
    assert_eq!(out.placements.len(), kept.len());
    for (p, r) in out.placements.iter().zip(&kept) {
        let index = u32::try_from(r.variant % 3).unwrap();
        assert_eq!(p.id, r.id);
        assert_eq!(p.class, ROCK_LIBRARY);
        assert_eq!(p.flags, PlacementFlags::default());
        assert_eq!(p.base, RecordRef::new(LIBRARY_BASE_KIND, "talus"));
        assert_eq!(parse_model_id(p.model.as_ref().unwrap().as_str()), Some(("talus", index)));
        let scale = r.transform.scale.x / 2.0;
        assert_eq!(p.transform.scale, Vec3::splat(scale));
        assert_eq!(p.transform.rotation, r.transform.rotation);
        let lift = (0.5 - r.sink) * maxes[index as usize][2] * scale;
        assert_eq!(p.transform.pos, r.transform.pos.offset(rotation * Vec3::new(0.0, lift, 0.0)), "{r:?}");
    }
    assert!(std::sync::Arc::ptr_eq(&out.placements[0].base.kind, &out.placements[3].base.kind), "one base per class, cloned");
    assert_eq!(out.models.len(), 3, "variants 0, 1, 2 used");
    for (id, info) in &out.models {
        let (_, index) = parse_model_id(id.as_str()).unwrap();
        let b = libs.variant("talus", index).unwrap().lods.bounds;
        assert_eq!((info.min, info.max), engine_bounds(b.min, b.max));
        let m = maxes[index as usize];
        assert_eq!(info.min, Vec3::new(-m[0] * 0.5, -m[2] * 0.5, -(m[1] - 0.25)), "z = -y: the far side is the max y");
        assert_eq!(info.max, Vec3::new(m[0] * 0.5, m[2] * 0.5, 0.25));
        assert_eq!((info.bytes, info.lod_distance), (0, 0.0));
    }
}
