#![allow(clippy::float_cmp)] // exact comparisons are the point of these tests
#![allow(clippy::cast_precision_loss)] // counts of test fixtures
//! WG-E2o.2: the class census (land only, exact areas, the weight mean of a brute loop, the same on any thread
//! count) and the calibration (a target under the crown ceiling is met within `calib_tol`, one above it is flagged
//! within `calib_iters`, the same inputs give the same `g`, the file round-trips). WG-E2o.f: the hash-ordered sample
//! reads the class over its whole land where the purest tiles do not; a joint check reopens a class its neighbour
//! moved, at most `calib_iters` checks.

use super::*;
use crate::scatter::WaterMask;
use crate::scatter_fields::{FieldSample, Suitability};
use crate::scatter_species::SpeciesFile;

/// A canopy tree and a ground fern on one temperature hat, no mosaic clearing, two classes; `{dense}` is the dense
/// class's target. The calibration runs on 100 ha per class.
const FILE_TEXT: &str = r#"
[params]
"scatter.stand_clearing_threshold" = { v = 0.0, unit = "ratio", min = 0, max = 1, source = "Guess" }
"scatter.calib_area_ha" = { v = {area}, unit = "ha", min = 10, max = 2000, source = "Guess" }

[columns]
hat_temperature_c = { unit = "C", min = -40, max = 40, source = "Guess" }

[classes."sparse"]
stems_per_ha = { v = 100, unit = "1/ha", min = 0, max = 2000, source = "Guess" }
[classes."dense"]
stems_per_ha = { v = {dense}, unit = "1/ha", min = 0, max = 2000, source = "Guess" }

[[species]]
name = "tree"
layer = "canopy"
size_law = "reverse_j"
hat_temperature_c = [0, 10, 30, 40]

[[species]]
name = "fern"
layer = "ground"
size_law = "reverse_j"
hat_temperature_c = [0, 10, 30, 40]
"#;

/// Class 0 west of x = 2000 m, class 1 east of it. With `gradient`, the east warms from 30 C at x = 2000 to 36 C at
/// x = 4000 (the hat's shoulder: weight 1 to 0.4), so tiles differ in weight inside the dense class.
struct TwoClasses {
    names: Vec<String>,
    gradient: bool,
}

impl Fields for TwoClasses {
    fn sample(&self, x: f64, _: f64) -> FieldSample {
        FieldSample {
            temperature_c: if self.gradient && x >= 2000.0 { 30.0 + 6.0 * (x - 2000.0) / 2000.0 } else { 10.0 },
            precipitation_mm: 800.0,
            moisture: 0.5,
            class: Some(u8::from(x >= 2000.0)),
            slope_deg: 0.0,
            aspect_deg: 0.0,
            altitude_m: 100.0,
            curvature: 0.0,
            water_distance_m: 1000.0,
        }
    }

    fn class_names(&self) -> &[String] {
        &self.names
    }
}

/// Two 4 km2 classes side by side.
const BOUNDS: [f64; 4] = [0.0, 0.0, 4000.0, 2000.0];

/// The forced clearing: a 96 m strip on the west edge (six census columns of class 0).
fn excluded(x: f64, _: f64) -> bool {
    x < 96.0
}

fn one(_: usize, _: f64, _: f64) -> f64 {
    1.0
}

fn flat(_: f64, _: f64) -> f64 {
    0.0
}

struct Fixture {
    file: SpeciesFile,
    fields: TwoClasses,
    suit: Vec<Suitability>,
    canopy: Vec<Suitability>,
    water: WaterMask,
    targets: CanopyTargets,
    tile_m: f64,
}

impl Fixture {
    fn new(dense: f64) -> Self {
        Self::build(dense, 100.0, false)
    }

    /// `dense`'s target, `scatter.calib_area_ha`, the east's temperature gradient.
    fn build(dense: f64, area_ha: f64, gradient: bool) -> Self {
        let text = FILE_TEXT.replace("{dense}", &dense.to_string()).replace("{area}", &format!("{area_ha:.1}"));
        let file = SpeciesFile::parse_toml(&text).expect("fixture parses");
        let fields = TwoClasses { names: vec!["sparse".to_owned(), "dense".to_owned()], gradient };
        let suit: Vec<Suitability> = file.species.iter().map(|s| Suitability::new(s, fields.class_names())).collect();
        let canopy = vec![suit[0].clone()];
        let targets = CanopyTargets::uncalibrated(&file, fields.class_names());
        Self { file, fields, suit, canopy, water: WaterMask::default(), targets, tile_m: 256.0 }
    }

    fn rules(&self) -> CandidateRules<'_> {
        CandidateRules {
            file: &self.file,
            suitability: &self.suit,
            seed: 55,
            tile_m: self.tile_m,
            prior: &one,
            water: &self.water,
            height: &flat,
            rocks: None,
            canopy: &self.targets,
        }
    }

    fn stand(&self) -> StandRules<'_> {
        StandRules { values: &self.file.values, seed: 55, canopy: &self.canopy }
    }

    fn census(&self) -> Census {
        census(&self.rules(), &self.stand(), &self.fields, &excluded, BOUNDS)
    }

    fn calibrate(&self, census: &Census) -> Calibration {
        calibrate(&self.rules(), &self.stand(), &self.fields, &excluded, census)
    }
}

#[test]
fn class_census_is_land_only_exact_and_thread_independent() {
    let fx = Fixture::new(400.0);
    let got = fx.census();
    let step = fx.file.values.census_step_m;
    assert_eq!(step, 16.0, "the declared default");
    // 125 x 125 cells per class; the strip removes 6 columns of class 0.
    assert_eq!(got.classes[0].cells, 119 * 125);
    assert_eq!(got.classes[1].cells, 125 * 125);
    assert_eq!(got.area_ha(0), 119.0 * 125.0 * 256.0 / 10_000.0);
    assert_eq!(got.area_ha(1), 400.0);
    // A brute loop over the same cells: w = D (one canopy species, prior 1, no clearing).
    let stand = fx.stand();
    let mut brute = [(0.0, 0u64); 2];
    for j in 0..125 {
        for i in 0..250 {
            let (x, y) = ((f64::from(i) + 0.5) * step, (f64::from(j) + 0.5) * step);
            if excluded(x, y) {
                continue;
            }
            let c = usize::from(x >= 2000.0);
            brute[c].0 += stand.density(&fx.fields, x, y);
            brute[c].1 += 1;
        }
    }
    for (c, &(sum, n)) in brute.iter().enumerate() {
        let want = sum / n as f64;
        let m = got.classes[c].mean();
        println!("class {c}: m {m:.6} (brute {want:.6}), A {} ha", got.area_ha(c));
        assert!((m - want).abs() <= 1e-12 * want, "class {c}: m {m} vs brute {want}");
        assert!(m > 0.1 && m < 1.0, "class {c}: m {m}");
        assert_eq!(got.species_weight[c][1], 0.0, "the fern is not canopy");
    }
    let tile_cells: u64 = got.tiles.values().map(|(n, _)| n).sum();
    assert_eq!(tile_cells, 250 * 125, "every cell in one tile");
    let one_thread = rayon::ThreadPoolBuilder::new().num_threads(1).build().expect("pool").install(|| fx.census());
    assert_eq!(one_thread, got, "1 thread and the default pool");
}

#[test]
fn square_canopy_is_the_canopy_of_square_layers() {
    let fx = Fixture::new(400.0);
    let rules = fx.rules();
    let tile = (9, 3);
    let map = StandMap::build(fx.stand(), &fx.fields, &excluded, rules.tile_rect(tile));
    let square = rules.tile_square(tile);
    let all = rules.square_layers(&fx.fields, &map, square).plants;
    let canopy: Vec<_> = all.iter().filter(|c| c.key.layer == Layer::Canopy).copied().collect();
    assert!(!canopy.is_empty() && canopy.len() < all.len(), "{} canopy of {}", canopy.len(), all.len());
    assert_eq!(rules.square_canopy(&fx.fields, &map, square), canopy);
}

#[test]
fn calibration_meets_reachable_targets_and_flags_the_ceiling() {
    let v = Fixture::new(400.0).file.values;
    let fx = Fixture::new(400.0);
    let census = fx.census();
    let cal = fx.calibrate(&census);
    for w in &cal.warnings {
        println!("warning: {w}");
    }
    for c in &cal.classes {
        println!(
            "{}: T {} m {:.3} g {:.3} W {:.1} steps {} ceiling {} calib {:.1} ha",
            c.name, c.stems_per_ha, c.weight_mean, c.scale, c.reached_per_ha, c.steps, c.ceiling, c.calib_area_ha
        );
        assert!(!c.ceiling && !c.low_weight, "{}: flagged", c.name);
        assert!((c.reached_per_ha - c.stems_per_ha).abs() <= v.calib_tol * c.stems_per_ha, "{}: W off", c.name);
        assert!(c.steps <= v.calib_iters);
        assert!(c.calib_area_ha >= 100.0 && c.calib_area_ha < 110.0, "{}: {} ha", c.name, c.calib_area_ha);
        assert!(c.scale >= 1.0 && c.scale <= v.intensity_scale_max * c.weight_mean);
        assert_eq!(c.weight_mean, census.classes[usize::from(c.name == "dense")].mean());
    }
    assert!(cal.warnings.is_empty(), "{:?}", cal.warnings);
    assert_eq!(fx.calibrate(&census).classes, cal.classes, "same inputs, same g");

    // 2000 stems per ha: crowns of ~2 m conflict distance pack below it; flagged, the largest W kept.
    let packed = Fixture::new(2000.0);
    let pc = packed.calibrate(&packed.census());
    let dense = &pc.classes[1];
    println!("packed dense: g {:.3} W {:.1} steps {}", dense.scale, dense.reached_per_ha, dense.steps);
    assert!(dense.ceiling && dense.steps <= v.calib_iters, "{dense:?}");
    assert!(dense.reached_per_ha < (1.0 - v.calib_tol) * 2000.0 && dense.reached_per_ha > 0.0);
    assert!(pc.warnings.iter().any(|w| w.contains("dense") && w.contains("ceiling")), "{:?}", pc.warnings);
    assert!(!pc.classes[0].ceiling, "the sparse class is unchanged");

    // The file round-trips (no warnings written).
    let dir = std::env::temp_dir().join(format!("sf_worldgen_calib_{}", std::process::id()));
    cal.write(&dir).expect("write");
    let back = Calibration::read(&dir).expect("read");
    let _ = std::fs::remove_dir_all(&dir);
    assert_eq!(back.classes, cal.classes);
    assert!(back.warnings.is_empty());
    assert_eq!(back.targets(), cal.targets());
}

/// Class `c`'s canopy winners per ha of its census land over every census tile, at `cal`'s kept `g` (exact winners).
fn world_per_ha(fx: &Fixture, census: &Census, cal: &Calibration, c: usize) -> f64 {
    let targets = cal.targets();
    let rules = CandidateRules { canopy: &targets, ..fx.rules() };
    let tiles: Vec<(i32, i32)> = census.tiles.keys().copied().collect();
    let won = winners_per_tile(&rules, &fx.stand(), &fx.fields, &excluded, &tiles, 2);
    won.values().map(|n| n[c]).sum::<u64>() as f64 / census.area_ha(c)
}

/// The v1 estimate at `cal`'s kept `g`: class `c`'s purest tiles (census share, ties by key) until
/// `calib_area_ha` of its land, `W` there rescaled by `m / m_t`.
fn purest_estimate(fx: &Fixture, census: &Census, cal: &Calibration, c: usize) -> f64 {
    let cell_ha = census.step_m * census.step_m / M2_PER_HA;
    let mut ranked: Vec<(f64, (i32, i32))> = census
        .tiles
        .iter()
        .filter(|(_, (_, per))| per[c].cells > 0)
        .map(|(&k, (all, per))| (per[c].cells as f64 / *all as f64, k))
        .collect();
    ranked.sort_by(|a, b| b.0.total_cmp(&a.0).then(a.1.cmp(&b.1)));
    let (mut tiles, mut land) = (Vec::new(), ClassCount::default());
    for (_, k) in ranked {
        if land.cells as f64 * cell_ha >= fx.file.values.calib_area_ha {
            break;
        }
        land.merge(&census.tiles[&k].1[c]);
        tiles.push(k);
    }
    let targets = cal.targets();
    let rules = CandidateRules { canopy: &targets, ..fx.rules() };
    let won = winners_per_tile(&rules, &fx.stand(), &fx.fields, &excluded, &tiles, 2);
    let n: u64 = tiles.iter().map(|k| won[k][c]).sum();
    n as f64 / (land.cells as f64 * cell_ha) * census.classes[c].mean() / land.mean()
}

#[test]
fn the_sample_reads_the_whole_class_where_the_purest_tiles_do_not() {
    // 250 m tiles: every tile inside the bounds, the class border on a tile border; the dense class's weight falls
    // from 1 (west) to 0.4 (east), so its purest tiles by key (the west columns) are its heaviest.
    let mut fx = Fixture::build(400.0, 200.0, true);
    fx.tile_m = 250.0;
    let v = fx.file.values;
    let census = fx.census();
    let cal = fx.calibrate(&census);
    let order = tile_order(&census, 55);
    assert_eq!(order.len(), census.tiles.len());
    for (c, row) in cal.classes.iter().enumerate() {
        let world = world_per_ha(&fx, &census, &cal, c);
        let old = purest_estimate(&fx, &census, &cal, c);
        println!(
            "{}: T {} m {:.3} m_t {:.3} g {:.3} reached {:.1} world {world:.1} purest x m/m_t {old:.1} tiles {} land {:.0} ha \
             steps {} checks {} reopened {}",
            row.name,
            row.stems_per_ha,
            row.weight_mean,
            row.calib_weight_mean,
            row.scale,
            row.reached_per_ha,
            row.calib_tiles,
            row.calib_area_ha,
            row.steps,
            row.joint_checks,
            row.reopened
        );
        assert!(!row.ceiling && !row.low_weight, "{}: flagged", row.name);
        assert!((row.reached_per_ha - world).abs() <= v.calib_tol * world, "{}: reached vs the whole class", row.name);
        assert!(row.joint_checks >= 1 && row.joint_checks <= v.calib_iters);
        // The sample: the classes' tiles of the one world order, until 200 ha of class land.
        let s = calib_tiles(&census, &order, c, 200.0);
        assert_eq!(s.tiles.len(), row.calib_tiles as usize);
        assert_eq!((s.land_ha, s.weight_mean), (row.calib_area_ha, row.calib_weight_mean));
        assert!(s.land_ha >= 200.0 && s.land_ha < 200.0 + 6.25 + 1e-9, "{}: {} ha", row.name, s.land_ha);
        let last = order.iter().position(|k| k == s.tiles.last().expect("tiles")).expect("in the order");
        let holding: Vec<(i32, i32)> = order[..=last].iter().copied().filter(|k| census.tiles[k].1[c].cells > 0).collect();
        assert_eq!(holding, s.tiles, "{}: a prefix of the world order", row.name);
    }
    let dense = &cal.classes[1];
    let world = world_per_ha(&fx, &census, &cal, 1);
    let old = purest_estimate(&fx, &census, &cal, 1);
    assert!((old - world).abs() > v.calib_tol * world, "the fixture tells the purest tiles ({old:.1}) from the class ({world:.1})");
    assert!((dense.calib_weight_mean - dense.weight_mean).abs() < 0.1, "the sample's m_t {} near m {}", dense.calib_weight_mean, dense.weight_mean);
    let one_thread = rayon::ThreadPoolBuilder::new().num_threads(1).build().expect("pool").install(|| fx.calibrate(&census));
    assert_eq!(one_thread.classes, cal.classes, "1 thread and the default pool");
    assert_eq!(fx.calibrate(&census).classes, cal.classes, "same inputs, same g");
}

#[test]
fn a_joint_check_reopens_a_class_its_neighbour_moved() {
    // Class a's land takes class b's offspring: `W_a = 50 g_a + 2 g_b`, T 100; `W_b = 100 g_b^0.7`, T 400.
    let w = |gs: &[f64], i: usize| if i == 0 { 50.0 * gs[0] + 2.0 * gs[1] } else { 100.0 * gs[1].powf(0.7) };
    let run = |iters: u32| {
        let mut its = vec![Iterate::new(100.0, 1.0, 16.0, 2.0), Iterate::new(400.0, 1.0, 16.0, 1.0)];
        let mut asked = Vec::new();
        let checks = drive(&mut its, 0.05, 0.25, iters, &mut |gs, ask| {
            asked.push(ask.to_vec());
            ask.iter().map(|&i| w(gs, i)).collect()
        });
        (its, checks, asked)
    };
    // a meets 100 at once (g_b 1, W 102); b in 2 steps (g 4, then the secant: 7.25, W 400); the check at
    // b's g reads a at 114.5: reopened, one step (g 1.75, W 101.8); the second check reopens nothing.
    let (its, checks, asked) = run(4);
    for (i, it) in its.iter().enumerate() {
        println!("class {i}: joint {:?} steps {} reopened {} seen {:?}", it.joint, it.steps, it.reopened, it.seen);
    }
    assert_eq!(checks, 2);
    assert_eq!((its[0].reopened, its[1].reopened), (1, 0));
    assert_eq!(asked, vec![vec![0, 1], vec![1], vec![1], vec![0, 1], vec![0], vec![0, 1]]);
    let gs = [its[0].joint.expect("checked").0, its[1].joint.expect("checked").0];
    for (i, it) in its.iter().enumerate() {
        let (_, wj) = it.joint.expect("checked");
        assert!(it.converged && (wj - it.target).abs() <= 0.05 * it.target, "class {i}: {wj}");
        assert_eq!(wj, w(&gs, i), "class {i}: the reported W is the last check's, at the reported g");
    }
    assert!(its[0].seen.iter().any(|&(_, w)| (w - 100.0).abs() > 5.0), "a was re-measured out of tol");
    // At most `calib_iters` checks: with 1, b runs out of steps (kept g 4, W 264) and a stays off at the check.
    let (its, checks, _) = run(1);
    assert_eq!(checks, 1);
    assert_eq!(its[0].reopened, 0);
    assert_eq!(its[0].joint, Some((2.0, 108.0)));
    assert!(!its[1].converged && !its[1].ceiling);
}
