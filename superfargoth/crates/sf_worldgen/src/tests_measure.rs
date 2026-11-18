//! WG-M measurement tests: spectral peak of a known sine, zero roughness of a plane, the land mask.

use crate::Grid;
use crate::measure::{
    MeasureParams, distance_transform, fft, lowpass_local_linear, measure_field, prepare_field,
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
