#![allow(clippy::float_cmp)] // exact comparisons are the point of these tests
#![allow(clippy::cast_precision_loss)] // counts of test fixtures
//! WG-E2o.2: the class census (land only, exact areas, the weight mean of a brute loop, the same on any thread
//! count) and the calibration (a target under the crown ceiling is met within `calib_tol`, one above it is flagged
//! within `calib_iters`, the same inputs give the same `g`, the file round-trips).

use super::*;
use crate::scatter::WaterMask;
use crate::scatter_fields::{FieldSample, Suitability};
use crate::scatter_species::SpeciesFile;

/// A canopy tree and a ground fern on one temperature hat, no mosaic clearing, two classes; `{dense}` is the dense
/// class's target. The calibration runs on 100 ha per class.
const FILE_TEXT: &str = r#"
[params]
"scatter.stand_clearing_threshold" = { v = 0.0, unit = "ratio", min = 0, max = 1, source = "Guess" }
"scatter.calib_area_ha" = { v = 100.0, unit = "ha", min = 10, max = 2000, source = "Guess" }

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

/// Class 0 west of x = 2000 m, class 1 east of it.
struct TwoClasses {
    names: Vec<String>,
}

impl Fields for TwoClasses {
    fn sample(&self, x: f64, _: f64) -> FieldSample {
        FieldSample {
            temperature_c: 10.0,
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
}

impl Fixture {
    fn new(dense: f64) -> Self {
        let file = SpeciesFile::parse_toml(&FILE_TEXT.replace("{dense}", &dense.to_string())).expect("fixture parses");
        let fields = TwoClasses { names: vec!["sparse".to_owned(), "dense".to_owned()] };
        let suit: Vec<Suitability> = file.species.iter().map(|s| Suitability::new(s, fields.class_names())).collect();
        let canopy = vec![suit[0].clone()];
        let targets = CanopyTargets::uncalibrated(&file, fields.class_names());
        Self { file, fields, suit, canopy, water: WaterMask::default(), targets }
    }

    fn rules(&self) -> CandidateRules<'_> {
        CandidateRules {
            file: &self.file,
            suitability: &self.suit,
            seed: 55,
            tile_m: 256.0,
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
