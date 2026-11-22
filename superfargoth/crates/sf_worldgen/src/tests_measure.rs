//! WG-M measurement tests: spectral peak of a known sine, zero roughness of a plane, the land mask.

use crate::Grid;
use crate::guide::RegionMap;
use crate::measure::{
    MeasureParams, distance_transform, fft, land_rows, lowpass_local_linear, measure_field, prepare_field,
    region_labels, walk_slope_deg, walkability,
};

const DX: f64 = 20.0;

fn params() -> MeasureParams {
    let mut p = MeasureParams::standard(DX);
    p.patch = 256;
    p.edge_m = 0.0;
    p.coast_m = 200.0;
    p
}

fn field(name: &str, g: Grid, p: &MeasureParams) -> crate::measure::Field {
    prepare_field(name.into(), g, [0.0, 0.0], 0.0, p).unwrap()
}

#[test]
fn fft_matches_a_direct_dft() {
    let n = 16;
    let x: Vec<f64> = (0..n).map(|i| ((i * 7 + 3) % 11) as f64 - 4.0).collect();
    let (mut re, mut im) = (x.clone(), vec![0.0; n]);
    fft(&mut re, &mut im);
    for k in 0..n {
        let (mut r, mut s) = (0.0, 0.0);
        for (j, v) in x.iter().enumerate() {
            let a = -2.0 * std::f64::consts::PI * (k * j) as f64 / n as f64;
            r += v * a.cos();
            s += v * a.sin();
        }
        assert!((re[k] - r).abs() < 1e-9 && (im[k] - s).abs() < 1e-9, "bin {k}");
    }
}

#[test]
fn psd_of_a_sine_peaks_at_its_wavelength() {
    let p = params();
    let n = 512;
    // Wavelength = patch / 8 = 640 m along x, 30 m amplitude.
    let lambda = 256.0 * DX / 8.0;
    let mut g = Grid::filled(n, n, DX, 0.0);
    for y in 0..n {
        for x in 0..n {
            g.data[y * n + x] =
                500.0 + 30.0 * (2.0 * std::f64::consts::PI * x as f64 * DX / lambda).sin();
        }
    }
    let r = measure_field(&field("sine", g, &p), &p, false);
    let z = &r.zones[0];
    assert!(z.spectrum.patches > 0);
    // Raw spectrum: the densest bin holds the sine's wavelength.
    let raw = z
        .spectrum
        .bins
        .iter()
        .max_by(|a, b| a.1.total_cmp(&b.1))
        .unwrap();
    assert!(
        (raw.0 / lambda).log10().abs() < 0.1,
        "raw peak {} m vs {lambda} m",
        raw.0
    );
    let peak = z.spectrum.peak_m.unwrap();
    assert!((peak / lambda).log10().abs() < 0.1, "normalized peak {peak} m vs {lambda} m");
    // The 300 m-1 km band holds the sine's variance (amplitude² / 2), both estimates.
    let b = z.bands.iter().find(|b| (b.lo_m - 300.0).abs() < 1e-9).unwrap();
    let rms = 30.0 / std::f64::consts::SQRT_2;
    assert!((b.psd_rms_m.unwrap() / rms - 1.0).abs() < 0.1, "psd {:?}", b.psd_rms_m);
    // The DoG pair passes 640 m at ~0.86 − 0.19 of its amplitude (Gaussian transfer), not a sharp band.
    assert!((0.5..1.0).contains(&(b.dog_rms_m / rms)), "dog {}", b.dog_rms_m);
}

#[test]
fn a_plane_has_zero_roughness() {
    let p = params();
    let (w, h) = (300, 260);
    let mut g = Grid::filled(w, h, DX, 0.0);
    for y in 0..h {
        for x in 0..w {
            g.data[y * w + x] = 1000.0 + 0.12 * x as f64 * DX - 0.07 * y as f64 * DX;
        }
    }
    let r = measure_field(&field("plane", g, &p), &p, false);
    let z = &r.zones[0];
    assert!(!z.bands.is_empty());
    for b in &z.bands {
        assert!(b.dog_rms_m < 1e-6, "dog {} at {}-{} m", b.dog_rms_m, b.lo_m, b.hi_m);
        assert!(b.psd_rms_m.unwrap_or(0.0) < 1e-6, "psd {:?}", b.psd_rms_m);
    }
    // The local-linear low-pass reproduces the plane at the border too, with a hole in the mask.
    let mask: Vec<bool> = (0..w * h).map(|i| (i % w) < 150 || (i / w) > 100).collect();
    let lp = lowpass_local_linear(&r_heights(w, h), &mask, w, h, 25.0);
    let want = r_heights(w, h);
    for (i, v) in lp.iter().enumerate().filter(|(i, _)| mask[*i]) {
        assert!((v - want[i]).abs() < 1e-6, "sample {i}: {v} vs {}", want[i]);
    }
}

fn r_heights(w: usize, h: usize) -> Vec<f64> {
    (0..w * h)
        .map(|i| 3.0 + 0.5 * (i % w) as f64 - 0.25 * (i / w) as f64)
        .collect()
}

#[test]
fn land_masking_excludes_sea_and_the_coast_margin() {
    let p = params();
    let (w, h) = (300, 200);
    let mut g = Grid::filled(w, h, DX, 0.0);
    let mut seed = 0x9e37_79b9_u32;
    for y in 0..h {
        for x in 0..w {
            g.data[y * w + x] = if x < 100 {
                // Wild sea floor: would dominate every number if it were measured.
                seed ^= seed << 13;
                seed ^= seed >> 17;
                seed ^= seed << 5;
                -1.0 - f64::from(seed % 2000)
            } else {
                50.0 + 0.05 * x as f64 * DX
            };
        }
    }
    let r = measure_field(&field("half-sea", g, &p), &p, false);
    let z = &r.zones[0];
    // Land is x ≥ 100; the coast margin (≥ 200 m = 10 samples from the last sea column, 99) keeps x ≥ 109.
    let expected = (w - 109) * h;
    assert_eq!(z.samples, expected);
    assert!(z.height.min_m >= 50.0, "min {}", z.height.min_m);
    assert!(z.slope.p99_deg < 3.0, "slope p99 {}", z.slope.p99_deg);
    for b in &z.bands {
        assert!(b.dog_rms_m < 1e-6, "dog {} at {} m", b.dog_rms_m, b.lo_m);
    }
    // Distance transform: exact on a single seed.
    let mut s = vec![false; 25];
    s[12] = true;
    let d = distance_transform(&s, 5, 5);
    assert!((d[0] - 8f64.sqrt()).abs() < 1e-12 && (d[2] - 2.0).abs() < 1e-12);
}

#[test]
fn walkability_counts_two_plateaus_split_by_a_wall() {
    // 40 x 20 at 1 m: plateau 10 m for x <= 24, plateau 40 m for x >= 25 (central differences make x = 24, 25 a
    // 86 deg wall). Interior dry land: x 1..=38, y 1..=18 = 38 x 18 = 684; walkable: x 1..=23 (414) and
    // x 26..=38 (234).
    let (w, h) = (40, 20);
    let data: Vec<f64> = (0..w * h).map(|i| if i % w <= 24 { 10.0 } else { 40.0 }).collect();
    let g = Grid { w, h, spacing: 1.0, data };
    let mask = vec![true; w * h];
    let wk = walkability(&walk_slope_deg(&g, 0.0), w, &mask, 46.0);
    assert_eq!(wk.land, 684);
    assert_eq!(wk.components, 2);
    assert!((wk.walkable - 648.0 / 684.0).abs() < 1e-12, "walkable {}", wk.walkable);
    assert!((wk.largest - 414.0 / 684.0).abs() < 1e-12, "largest {}", wk.largest);
    let shares: Vec<f64> = wk.bands.iter().map(|b| b.2).collect();
    assert_eq!(wk.bands.len(), 6);
    assert!((shares[0] - 648.0 / 684.0).abs() < 1e-12 && (shares[5] - 36.0 / 684.0).abs() < 1e-12);
    assert!(shares[1..5].iter().all(|&v| v == 0.0));
    // Sea at 20 m drowns the low plateau: land is x 25..=38 (252), one walkable component x 26..=38 (234).
    let wk = walkability(&walk_slope_deg(&g, 20.0), w, &mask, 46.0);
    assert_eq!((wk.land, wk.components), (252, 1));
    assert!((wk.walkable - 234.0 / 252.0).abs() < 1e-12 && (wk.largest - 234.0 / 252.0).abs() < 1e-12);
    // The zone mask bounds both the land and the components.
    let left: Vec<bool> = (0..w * h).map(|i| i % w < 20).collect();
    let wk = walkability(&walk_slope_deg(&g, 0.0), w, &left, 46.0);
    assert_eq!((wk.land, wk.components), (19 * 18, 1));
    assert!((wk.walkable - 1.0).abs() < 1e-12);
}

#[test]
fn land_rows_split_two_regions_with_exact_shares() {
    // 40 x 20 at 1 m, two 20 m region pixels: region "flat" (x < 20) a level 10 m plateau, region "slope" (x >= 20)
    // rising east at 40 deg from x = 19. Central differences: x 1..=18 0 deg, x = 19 atan(tan40 / 2) = 22.8 deg,
    // x 20..=38 40 deg. Interior land rows 1..=18: flat region x 1..=19 (19 x 18 = 342), slope x 20..=38 (342).
    let (w, h) = (40, 20);
    let t = 40f64.to_radians().tan();
    let data: Vec<f64> = (0..w * h)
        .map(|i| 10.0 + t * ((i % w) as f64 - 19.0).max(0.0))
        .collect();
    let g = Grid { w, h, spacing: 1.0, data };
    let p = MeasureParams::standard(1.0);
    let mut f = prepare_field("two-region".into(), g, [0.0, 0.0], 0.0, &p).unwrap();
    let rm = RegionMap {
        w: 2,
        h: 1,
        cell_m: 20.0,
        index: vec![1, 2],
        names: vec![String::new(), "flat".into(), "slope".into()],
    };
    let labels = region_labels(&rm, [0.0, 0.0], 1.0, &f);
    assert!((0..w * h).all(|i| labels[i] == if i % w < 20 { 1 } else { 2 }));
    // Base: the field 1 m lower for x >= 30 (only the slope region moved: x 30..=38 = 9 of 19 columns).
    let base: Vec<f64> = f
        .grid
        .data
        .iter()
        .enumerate()
        .map(|(i, &v)| if i % w >= 30 { v - 1.0 } else { v })
        .collect();
    f.base = Some(base);
    let slope = walk_slope_deg(&f.grid, 0.0);
    let dh: Vec<f64> = f.grid.data.iter().zip(f.base.as_ref().unwrap()).map(|(a, b)| a - b).collect();
    let mask = vec![true; w * h];
    let rows = land_rows(&slope, w, &mask, Some((&labels, &rm.names)), Some(&dh), 46.0);
    let names: Vec<&str> = rows.iter().map(|r| r.region.as_str()).collect();
    assert_eq!(names, ["all", "flat", "slope"]);
    let (all, flat, sl) = (&rows[0], &rows[1], &rows[2]);
    assert_eq!((all.walk.land, flat.walk.land, sl.walk.land), (684, 342, 342));
    let near = |a: f64, b: f64| (a - b).abs() < 1e-12;
    // Flat region: 18 of 19 columns under 10 deg, none steep, all walkable in one component, untouched.
    assert!(near(flat.walk.flat, 18.0 / 19.0) && near(flat.walk.steep, 0.0));
    assert!(near(flat.walk.walkable, 1.0) && near(flat.walk.largest, 1.0) && flat.walk.components == 1);
    assert!(near(flat.untouched.unwrap(), 1.0) && near(flat.dh_p95_m.unwrap(), 0.0));
    // Slope region: every sample 40 deg (steep, walkable), 10 of 19 columns untouched.
    assert!(near(sl.walk.flat, 0.0) && near(sl.walk.steep, 1.0) && near(sl.walk.walkable, 1.0));
    assert!(near(sl.untouched.unwrap(), 10.0 / 19.0) && near(sl.dh_p95_m.unwrap(), 1.0));
    // The whole land: one component across the region border, the shares of both halves.
    assert!(near(all.walk.flat, 18.0 / 38.0) && near(all.walk.steep, 19.0 / 38.0));
    assert!(near(all.walk.largest, 1.0) && all.walk.components == 1);
    assert!(near(all.untouched.unwrap(), 29.0 / 38.0));
    // Without regions and base: the whole-land row only, no untouched share.
    let rows = land_rows(&slope, w, &mask, None, None, 46.0);
    assert_eq!(rows.len(), 1);
    assert!(rows[0].untouched.is_none());
}
