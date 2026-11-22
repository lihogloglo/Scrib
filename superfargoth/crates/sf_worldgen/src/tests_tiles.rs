//! Tests of the runtime guide tiles ([`super`]): determinism, shared edges, the filter at each sample, the band
//! limit per level, conservative bounds; a measurement on the working island when it exists.

use super::*;
use crate::filter::{detail_offset, detail_offset_banded};

/// A synthetic 96 × 96 guide at 20 m (the filter tests' hills: a sloping coast, sea in the south-west), origin
/// (0, 0), so it covers engine X 0..1900 m, Z 0..1900 m.
fn hills_guide() -> Guide {
    let (w, h) = (96usize, 96usize);
    let mut heights = Vec::with_capacity(w * h);
    for r in 0..h {
        for c in 0..w {
            let (x, y) = (c as f64 * 20.0, -(r as f64) * 20.0);
            let z = 0.25 * x + 0.2 * (y + 1900.0) - 120.0
                + 180.0 * (x / 310.0).sin() * (y / 270.0).cos()
                + 60.0 * ((x + y) / 130.0).sin();
            heights.push(z as f32);
        }
    }
    Guide { w, h, spacing_m: 20.0, sea_level_m: 0.0, fill_m: None, origin_m: [0.0, 0.0], heights, regions: None }
}

fn tiles() -> GuideTiles {
    GuideTiles::new(hills_guide(), WorldspaceId::new("test"), FilterParams::default(), TileSettings::default())
}

fn bits(t: &HeightTile) -> Vec<u32> {
    t.heights.iter().map(|h| h.to_bits()).collect()
}

#[test]
fn same_tile_twice_and_on_one_thread_is_bitwise_equal() {
    let t = tiles();
    let c = TileCoord::new(0, 3, 4);
    let a = t.heights(c);
    let b = t.heights(c);
    let one = rayon::ThreadPoolBuilder::new().num_threads(1).build().unwrap().install(|| t.heights(c));
    assert_eq!(bits(&a), bits(&b));
    assert_eq!(bits(&a), bits(&one));
    // Not trivially flat: the filter added detail somewhere.
    let (lo, hi) = a.range();
    assert!(hi - lo > 10.0, "range {lo}..{hi}");
}

#[test]
fn neighbours_share_edge_samples_bitwise() {
    let t = tiles();
    for level in [0u8, 2, 5] {
        let a = t.heights(TileCoord::new(level, 1, 1));
        let east = t.heights(TileCoord::new(level, 2, 1));
        let south = t.heights(TileCoord::new(level, 1, 2));
        let n = TILE_SAMPLES - 1;
        for k in 0..TILE_SAMPLES {
            assert_eq!(a.get(n, k).to_bits(), east.get(0, k).to_bits(), "level {level} east edge {k}");
            assert_eq!(a.get(k, n).to_bits(), south.get(k, 0).to_bits(), "level {level} south edge {k}");
        }
    }
}

#[test]
fn level_zero_sample_is_the_full_filter_at_its_world_position() {
    let t = tiles();
    let g = t.guide();
    let base = t.base();
    let c = TileCoord::new(0, 4, 3);
    let tile = t.heights(c);
    let mut detailed = 0;
    for (i, j) in [(0, 0), (17, 300), (256, 256), (512, 512), (401, 7), (100, 499)] {
        let (gx, gz) = c.global_sample(i, j);
        let (x, z) = (sample_meters(gx), sample_meters(gz));
        let s = base.sample((x - g.origin_m[0]) / g.spacing_m, (g.origin_m[1] + z) / g.spacing_m);
        let d = detail_offset([x, -z], &s, g.sea_level_m, t.params());
        detailed += usize::from(d != 0.0);
        assert_eq!(tile.get(i, j).to_bits(), (s.h as f32 + d).to_bits(), "sample ({i}, {j})");
    }
    assert!(detailed > 0, "no sample had detail");
}

#[test]
fn band_limit_keeps_fine_levels_whole_and_removes_cut_octaves() {
    let t = tiles();
    let g = t.guide();
    let base = t.base();
    let prm = *t.params();
    // Smallest octave at the defaults: 160 / 2^3 = 20 m.
    let smallest = prm.wavelength_m / prm.lacunarity.powi(prm.octaves as i32 - 1);
    let mut cut_somewhere = false;
    for level in 0u8..10 {
        let band = t.band_m(level);
        let c = containing(900.0, 900.0, level);
        let tile = t.heights(c);
        // Share of the amplitude the kept octaves may carry.
        let (mut kept, mut all, mut lambda, mut wk) = (0.0f32, 0.0f32, prm.wavelength_m, 1.0f32);
        for _ in 0..prm.octaves {
            all += wk;
            if lambda > band[0] {
                kept += wk;
            }
            lambda /= prm.lacunarity;
            wk *= prm.gain;
        }
        for (i, j) in [(10, 20), (256, 256), (300, 100), (480, 400)] {
            let (gx, gz) = c.global_sample(i, j);
            let (x, z) = (sample_meters(gx), sample_meters(gz));
            let s = base.sample((x - g.origin_m[0]) / g.spacing_m, (g.origin_m[1] + z) / g.spacing_m);
            let h = tile.get(i, j);
            let full = detail_offset([x, -z], &s, g.sea_level_m, &prm);
            if smallest >= band[1] {
                // Every octave at full strength: the full filter, bit for bit (level 0 among them).
                assert_eq!(h.to_bits(), (s.h as f32 + full).to_bits(), "level {level}");
            } else if prm.wavelength_m <= band[0] {
                // Every octave cut: the base surface only.
                assert_eq!(h.to_bits(), (s.h as f32).to_bits(), "level {level}");
            } else {
                // Some cut: the detail fits the kept octaves' share of the amplitude bound.
                let banded = detail_offset_banded([x, -z], &s, g.sea_level_m, &prm, Some(band));
                assert_eq!(h.to_bits(), (s.h as f32 + banded).to_bits(), "level {level}");
                let d = h - s.h as f32;
                assert!(d.abs() <= prm.max_amplitude_m * kept / all + 1e-3, "level {level}: {d}");
                cut_somewhere |= banded.to_bits() != full.to_bits();
            }
        }
    }
    assert!(cut_somewhere, "no level changed the detail");
    assert!(t.band_m(0)[1] <= smallest, "level 0 must keep every octave at the defaults");
}

#[test]
fn bounds_cover_every_finer_level_sample() {
    // The analytic bounds alone (what a GPU-filled world culls with), at the preview's and the runtime's octaves.
    let runtime = TileSettings::default();
    let t8 = GuideTiles::new(hills_guide(), WorldspaceId::new("test"), FilterParams { octaves: runtime.runtime_octaves, ..FilterParams::default() }, runtime);
    for t in [tiles(), t8] {
        bounds_cover(&t);
    }
}

fn bounds_cover(t: &GuideTiles) {
    let check = |coarse: TileCoord, fine: TileCoord| {
        let bounds = t.bounds_only(coarse);
        let tile = t.heights(fine);
        let shift = coarse.level - fine.level;
        let (cx, cz) = coarse.first_sample();
        let (fx, fz) = fine.first_sample();
        let quad = 1i64 << coarse.level;
        let mut checked = 0;
        for j in 0..TILE_SAMPLES {
            for i in 0..TILE_SAMPLES {
                let (gx, gz) = fine.global_sample(i, j);
                let (qx, qz) = ((gx - cx) / quad, (gz - cz) / quad);
                if !(0..512).contains(&qx) || !(0..512).contains(&qz) {
                    continue;
                }
                let (lo, hi) = bounds.get(qx as usize / BLOCK_QUADS, qz as usize / BLOCK_QUADS);
                let h = tile.get(i, j);
                assert!(h >= lo && h <= hi, "{coarse:?} block bounds {lo}..{hi} miss {fine:?} ({i}, {j}) = {h}");
                checked += 1;
            }
        }
        assert!(checked > 0, "{coarse:?} / {fine:?} (shift {shift}, {fx}, {fz})");
    };
    let top = TileCoord::new(2, 0, 0);
    check(top, top);
    for child in top.children() {
        check(top, child);
        check(child, child.children()[3]);
        check(top, child.children()[0]);
    }
    // A coarse level well past the guide spacing (64 m samples) over level-0 tiles in the hills.
    check(TileCoord::new(7, 0, 0), TileCoord::new(0, 3, 3));
    check(TileCoord::new(7, 0, 0), TileCoord::new(0, 6, 5));
    // Where the stamp fixture sits (unstamped tiles: just more coverage).
    check(TileCoord::new(7, 0, 0), TileCoord::new(0, 3, 4));
    check(TileCoord::new(3, 0, 0), TileCoord::new(0, 4, 4));
}

/// A stamp over the hills straddling level-0 tiles (3, 4) | (4, 4) (the edge at X = 896 m): weight 1 inside, a
/// C1 smootherstep to 0 across 30 m at its edges, heights a 1.5 km tower (well past the guide, so the bounds
/// must grow) with a ripple.
fn stamped() -> GuideTiles {
    let (w, h, sp, band) = (120usize, 100usize, 1.8288, 30.0);
    let (mut heights, mut weights) = (Vec::new(), Vec::new());
    for r in 0..h {
        for c in 0..w {
            let d = (c.min(w - 1 - c).min(r).min(h - 1 - r)) as f64 * sp;
            let t = (d / band).clamp(0.0, 1.0);
            weights.push((t * t * t * (t * (6.0 * t - 15.0) + 10.0)) as f32);
            heights.push(1500.0 + 0.1 * c as f32 + 2.0 * ((r as f32) * 0.3).sin());
        }
    }
    let stamp = crate::stamps::Stamp { name: "tower".into(), origin_m: [790.3, -930.7], dims: [w, h], spacing_m: sp, heights, weights };
    tiles().with_stamps(StampSet { stamps: vec![stamp] })
}

#[test]
fn stamp_replaces_inside_blends_in_band_and_leaves_outside_bitwise() {
    let plain = tiles();
    let st = stamped();
    let idx = st.stamps().unwrap();
    let (mut inside, mut band, mut outside) = (0, 0, 0);
    for c in [TileCoord::new(0, 3, 4), TileCoord::new(0, 4, 4), TileCoord::new(2, 0, 0)] {
        let (a, b) = (plain.heights(c), st.heights(c));
        for j in 0..TILE_SAMPLES {
            for i in 0..TILE_SAMPLES {
                let (gx, gz) = c.global_sample(i, j);
                let (x, z) = (sample_meters(gx), sample_meters(gz));
                let (hp, hs) = (a.get(i, j), b.get(i, j));
                match idx.sample(x, -z) {
                    // Inside: the stamp's height bit for bit (no detail).
                    Some((h, w)) if w >= 1.0 => {
                        assert_eq!(hs.to_bits(), h.to_bits(), "{c:?} ({i}, {j})");
                        inside += 1;
                    }
                    Some((h, w)) if w > 0.0 => {
                        let (lo, hi) = (hp.min(h) - 25.0, hp.max(h) + 25.0);
                        assert!(hs.is_finite() && hs >= lo && hs <= hi, "{c:?} ({i}, {j}): {hs} vs {hp} / {h}");
                        band += 1;
                    }
                    _ => {
                        assert_eq!(hs.to_bits(), hp.to_bits(), "{c:?} ({i}, {j}) changed outside the stamp");
                        outside += 1;
                    }
                }
            }
        }
    }
    assert!(inside > 1000 && band > 1000 && outside > 1000, "{inside} / {band} / {outside}");
    // The point query is the tile function.
    let (x, z) = (790.3 + 60.0 * 1.8288, 930.7 + 50.0 * 1.8288);
    assert_eq!(st.height(x, z, 0).to_bits(), idx.sample(x, -z).unwrap().0.to_bits());
    assert_eq!(st.height(x, z, 0).to_bits(), st.with_params(*st.params()).height(x, z, 0).to_bits());
}

#[test]
fn stamped_tiles_are_bitwise_across_threads_and_neighbours() {
    let t = stamped();
    for level in [0u8, 1] {
        let c = containing(800.0, 1000.0, level);
        let a = t.heights(c);
        let one = rayon::ThreadPoolBuilder::new().num_threads(1).build().unwrap().install(|| t.heights(c));
        assert_eq!(bits(&a), bits(&one));
        let east = t.heights(TileCoord::new(level, c.x + 1, c.z));
        let n = TILE_SAMPLES - 1;
        for k in 0..TILE_SAMPLES {
            assert_eq!(a.get(n, k).to_bits(), east.get(0, k).to_bits(), "level {level} east edge {k}");
        }
    }
}

#[test]
fn stamped_bounds_cover_every_finer_level_sample() {
    bounds_cover(&stamped());
}

#[test]
fn stamped_gpu_data_packs_the_stamps_after_the_heights() {
    let (plain, st) = (tiles(), stamped());
    let (a, b) = (plain.gpu_data(), st.gpu_data());
    assert_eq!(a[slot::STAMPS as usize], 0);
    assert_eq!(b[slot::STAMPS as usize] as usize, a.len());
    assert_eq!(&b[..slot::STAMPS as usize], &a[..slot::STAMPS as usize]);
    assert_eq!(&b[slot::STAMPS as usize + 1..a.len()], &a[slot::STAMPS as usize + 1..]);
    assert_eq!(b.len() * 4, st.gpu_data_bytes());
    assert_eq!(a.len() * 4, plain.gpu_data_bytes());
}

#[test]
fn every_tile_exists_and_carries_no_layers() {
    let t = tiles();
    let c = TileCoord::new(0, -40, 90);
    assert!(t.has_tile(c));
    let p = t.tile(c).unwrap().unwrap();
    assert!(p.layers.is_none() && p.tint.is_none());
    // Far past the guide its edge extends.
    let h = p.height_tile();
    assert!(h.heights.iter().all(|v| v.is_finite()));
    let m = t.macro_albedo(c).unwrap().unwrap();
    assert_eq!(m.rgba.len(), LAYER_SAMPLES * LAYER_SAMPLES);
    assert!(t.fill_height().is_finite());
}

#[test]
fn declared_tile_values_validate() {
    let mut d = declared_tile_defaults();
    assert_eq!(TileSettings::from_declared(&d).unwrap().band_samples.map(f32::to_bits), [2.0f32, 4.0].map(f32::to_bits));
    d[0].v = 5.0;
    assert!(TileSettings::from_declared(&d).is_err());
}

/// Prints ms per tile per level on the working island `target/worldgen/vo/V3` (skips without it).
#[test]
#[ignore = "measurement: tile generation time per level on the V3 island"]
fn measure_tile_time_per_level_on_v3() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../target/worldgen/vo/V3");
    if !dir.join("meta.json").exists() {
        println!("no guide at {}: skipped", dir.display());
        return;
    }
    // `SF_TWIN=1`: on the CPU twin's own pool (`twin_threads`), as a GPU-filled world generates its query tiles.
    let t = GuideTiles::open(&dir, WorldspaceId::new("worldgen")).unwrap();
    let t = if std::env::var_os("SF_TWIN").is_some() { t.with_twin_pool().unwrap() } else { t };
    let [sx, _, sz] = t.summit();
    println!("summit ({sx:.0}, {sz:.0}), threads {}", rayon::current_num_threads());
    for level in 0u8..10 {
        // A slope of the volcano: 3 km south-east of the summit, three neighbouring tiles.
        let c0 = containing(sx + 3000.0, sz + 3000.0, level);
        let mut ms = Vec::new();
        for dx in 0..3 {
            let c = TileCoord::new(level, c0.x + dx, c0.z);
            let s = Instant::now();
            let _ = t.product(c);
            let heights = s.elapsed().as_secs_f64() * 1000.0;
            let s = Instant::now();
            let _ = t.macro_rgba(c);
            ms.push((heights, s.elapsed().as_secs_f64() * 1000.0));
        }
        let fmt: Vec<String> = ms.iter().map(|(h, m)| format!("{h:.1}+{m:.1}")).collect();
        println!("level {level} ({} m): heights+bounds + macro ms per tile: {}", TileCoord::new(level, 0, 0).spacing(), fmt.join(", "));
    }
}

/// The level-`level` tile containing engine position (x, z) metres (level-0 global index `(m + 128) / 0.5`).
fn containing(x: f64, z: f64, level: u8) -> TileCoord {
    let per = f64::from(512u32 << level);
    let t = |m: f64| ((m + 128.0) * 2.0 / per).floor() as i32;
    TileCoord::new(level, t(x), t(z))
}

/// What one sample point sees at level `l` and at `l + 1` (the level that replaces it beyond its range).
struct SwitchSample {
    /// Rendered height change at the point: level `l`'s sample minus level `l + 1`'s bilinear surface there.
    dh: f64,
    /// The point is on level `l + 1`'s lattice too (the change is the band limit alone, no decimation).
    even: bool,
    normal_deg: f64,
    slope_deg: [f64; 2],
    cliff_w: [f64; 2],
    dark: [f64; 2],
}

/// The shader's per-level shading inputs at (x, z) from level `l`'s heights: slope (deg), `cliff_w`
/// (`surface.wesl`, 28°..45° defaults), crevice darkness (`curvature_at_level`: clamp((avg − h)/s), × 2.5), normal.
fn level_shading(t: &GuideTiles, x: f64, z: f64, l: u8) -> (f64, f64, f64, [f64; 3]) {
    let s = f64::from(TileCoord::new(l, 0, 0).spacing());
    let h = |dx: f64, dz: f64| f64::from(t.height(x + dx, z + dz, l));
    let (c, e, w, n, so) = (h(0.0, 0.0), h(s, 0.0), h(-s, 0.0), h(0.0, -s), h(0.0, s));
    let (gx, gz) = ((e - w) / (2.0 * s), (so - n) / (2.0 * s));
    let len = (gx * gx + gz * gz + 1.0).sqrt();
    let nrm = [-gx / len, 1.0 / len, -gz / len];
    let smooth = |a: f64, b: f64, v: f64| {
        let k = ((v - a) / (b - a)).clamp(0.0, 1.0);
        k * k * (3.0 - 2.0 * k)
    };
    let cliff = 1.0 - smooth(45f64.to_radians().cos(), 28f64.to_radians().cos(), nrm[1]);
    let curv = ((0.25 * (e + w + n + so) - c) / s).clamp(-1.0, 1.0);
    ((gx * gx + gz * gz).sqrt().atan().to_degrees(), cliff, (curv * 2.5).clamp(0.0, 1.0), nrm)
}

#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn pctl(v: &mut [f64], q: f64) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    v.sort_by(f64::total_cmp);
    v[((v.len() - 1) as f64 * q).round() as usize]
}

/// The decoupled shading's parameters as `sf_terrain_bevy` sets them by default (`shade_params`: cover = the
/// 256-texel half window less the 16-sample snap, the curvature taps' reach at level 0 and 2 texels of margin;
/// `terrain.shade.blend_texels`; `terrain.cliff.curvature_scale_m`), mirrored here (that crate is Bevy's).
const SHADE_COVER_TEXELS: f64 = 234.0;
const SHADE_BLEND_TEXELS: f64 = 48.0;
const SHADE_CURVATURE_SCALE_M: f64 = 2.0;

/// The GPU's level-`l` normal at (x, z), mirrored: central differences one lattice step apart at the level's
/// lattice nodes (`sf_terrain::clipmap::fill_normals`; unquantized here, the GPU keeps 8 bits), bilinear between
/// the nodes as the filtered sampler (`common.wesl` `normal_at_level`), Y rebuilt.
fn lattice_normal(t: &GuideTiles, x: f64, z: f64, l: u8) -> [f64; 3] {
    let s = 0.5 * f64::from(1u32 << l);
    let (u, v) = ((x + 128.0) / s, (z + 128.0) / s);
    let (i0, j0) = (u.floor(), v.floor());
    let (fx, fz) = (u - i0, v - j0);
    let node = |i: f64, j: f64| {
        let h = |di: f64, dj: f64| f64::from(t.height((i + di) * s - 128.0, (j + dj) * s - 128.0, l));
        let (dx, dz) = ((h(1.0, 0.0) - h(-1.0, 0.0)) / (2.0 * s), (h(0.0, 1.0) - h(0.0, -1.0)) / (2.0 * s));
        let len = (dx * dx + dz * dz + 1.0).sqrt();
        [-dx / len, -dz / len]
    };
    let (a, b, c, d) = (node(i0, j0), node(i0 + 1.0, j0), node(i0, j0 + 1.0), node(i0 + 1.0, j0 + 1.0));
    let mix = |k: usize| (a[k] * (1.0 - fx) + b[k] * fx) * (1.0 - fz) + (c[k] * (1.0 - fx) + d[k] * fx) * fz;
    let (nx, nz) = (mix(0), mix(1));
    [nx, (1.0 - nx * nx - nz * nz).max(0.0).sqrt(), nz]
}

/// The decoupled shading inputs at (x, z) from level `l` (`surface.wesl` `curvature_fixed`): the normal and the
/// curvature at the fixed scale (the gradient's divergence across ± the scale, / 8).
fn fixed_shading(t: &GuideTiles, x: f64, z: f64, l: u8) -> ([f64; 3], f64) {
    let s = SHADE_CURVATURE_SCALE_M;
    let g = |x: f64, z: f64| {
        let n = lattice_normal(t, x, z, l);
        [-n[0] / n[1].max(0.05), -n[2] / n[1].max(0.05)]
    };
    let curv = ((g(x + s, z)[0] - g(x - s, z)[0]) + (g(x, z + s)[1] - g(x, z - s)[1])) * 0.125;
    (lattice_normal(t, x, z, l), curv.clamp(-1.0, 1.0))
}

/// `surface.wesl`'s cliff rock weight (28°..45° defaults) and crevice darkness (curvature × 2.5) of a normal and
/// a curvature.
fn cliff_and_dark(n: [f64; 3], curv: f64) -> (f64, f64) {
    let k = ((n[1] - 45f64.to_radians().cos()) / (28f64.to_radians().cos() - 45f64.to_radians().cos())).clamp(0.0, 1.0);
    (1.0 - k * k * (3.0 - 2.0 * k), (curv * 2.5).clamp(0.0, 1.0))
}

/// The decoupled selection (`common.wesl` `shade_levels`) at Chebyshev distance `d` m from the camera: the level
/// and its cross-fade weight into the next.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn shade_level_at(d: f64) -> (u8, f64) {
    let r = (2.0 * d).max(1e-3);
    let k = (r / SHADE_COVER_TEXELS).log2().ceil().clamp(0.0, 9.0);
    let u = r * (-k).exp2();
    let a = ((u - (SHADE_COVER_TEXELS - SHADE_BLEND_TEXELS)) / SHADE_BLEND_TEXELS).clamp(0.0, 1.0);
    (k as u8, if k >= 9.0 { 0.0 } else { a * a * (3.0 - 2.0 * a) })
}

/// LOD-pop diagnosis (wg-lodpop, 2026-11-22): at each CDLOD switch (LOD `l` hands over to `l + 1` at
/// `CdlodSettings::range(l)`), how much the rendered surface changes (metres, and pixels at that distance for
/// 1440 rows at a 70° vertical FOV), and how the per-level shading inputs change (normal, cliff rock weight,
/// crevice darkening). Three areas of the guide `SF_GUIDE` (default `x10s`; area A and C positions, or
/// `SF_AREAS=x,z;x,z;...`), gully-only (sharp scale 0) and with the sharp operators (1). Points inside stamps
/// are left out (stamps carry no runtime detail).
#[test]
#[ignore = "measurement: height/normal/crevice change at each CDLOD level switch (wg-lodpop)"]
#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::too_many_lines,
    clippy::similar_names,
    clippy::type_complexity
)]
fn measure_lod_switch_error() {
    let guide = std::env::var("SF_GUIDE").unwrap_or_else(|_| "x10s".into());
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../target/worldgen").join(&guide);
    if !dir.join("meta.json").exists() {
        println!("no guide at {}: skipped", dir.display());
        return;
    }
    let base = GuideTiles::open(&dir, WorldspaceId::new("worldgen")).unwrap();
    let cdlod = sf_terrain::cdlod::CdlodSettings::default();
    let focal_px = 720.0 / 35f64.to_radians().tan();
    let [sx, _, sz] = base.summit();
    // Engine (x, z): x east, z south. Area B: 4 km south of the guide's summit.
    let mut areas = vec![(-2926.0, 2048.0), (sx, sz + 4000.0), (4000.0, 7000.0)];
    if let Ok(s) = std::env::var("SF_AREAS") {
        areas = s
            .split(';')
            .filter_map(|p| p.split_once(',').map(|(a, b)| (a.trim().parse().unwrap(), b.trim().parse().unwrap())))
            .collect();
    }
    println!(
        "guide {guide}; summit ({sx:.0}, {sz:.0}); focal {focal_px:.0} px/rad; lod0_range {} m, morph {}",
        cdlod.lod0_range, cdlod.morph_fraction
    );
    let side = 64usize;
    for sharp in [0.0f32, 1.0] {
        // From the runtime parameters explicitly (not the open default), so both arms exist whatever it is.
        let prm = FilterParams { octaves: base.settings().runtime_octaves, ..FilterParams::default().with_sharp_defaults() };
        let mut prm = prm.with_sharp_scale(sharp);
        // `SF_ONLY=strata|rock|ridge`: that sharp operator alone (the others at 0), to attribute the change.
        match std::env::var("SF_ONLY").as_deref() {
            Ok("strata") => (prm.rock_amp_m, prm.ridge_amp_m) = (0.0, 0.0),
            Ok("rock") => (prm.strata_amp_m, prm.ridge_amp_m) = (0.0, 0.0),
            Ok("ridge") => (prm.strata_amp_m, prm.rock_amp_m) = (0.0, 0.0),
            _ => {}
        }
        let t = base.with_params(prm);
        let p = t.params();
        println!(
            "\n=== sharp {sharp} (gully cap {} m, relief {}, strata {} m, rock {} m, ridge {} m, bound {} m) ===",
            p.max_amplitude_m,
            p.relief_fraction,
            p.strata_amp_m,
            p.rock_amp_m,
            p.ridge_amp_m,
            p.max_total_amplitude_m()
        );
        for (cx, cz) in areas.iter().copied() {
            println!("\n-- area ({cx:.0}, {cz:.0}), h {:.0} m --", t.height(cx, cz, 0));
            println!(
                "L>L+1 | switch d m | quad px | n | |dh| mean/p99/max m | px mean/p99/max | band-only p99 m/px | \
                 normal deg mean/p99 | slope mean L/L+1 | cliff_w mean L/L+1, |d| p99 | dark mean L/L+1, albedo d p99"
            );
            for l in 0u8..9 {
                let step = 1i64 << l;
                let s1 = f64::from(TileCoord::new(l + 1, 0, 0).spacing());
                let half = 1500f64.max(32.0 * s1);
                let d_switch = f64::from(cdlod.range(l as i8));
                let samples: Vec<SwitchSample> = (0..side * side)
                    .into_par_iter()
                    .filter_map(|i| {
                        let mut r = (i as u64 + 1).wrapping_mul(0x9e37_79b9_7f4a_7c15) ^ (u64::from(l) << 40);
                        let mut next = || {
                            r ^= r >> 30;
                            r = r.wrapping_mul(0xbf58_476d_1ce4_e5b9);
                            r ^= r >> 27;
                            r = r.wrapping_mul(0x94d0_49bb_1331_11eb);
                            r ^= r >> 31;
                            (r >> 11) as f64 / (1u64 << 53) as f64
                        };
                        let (u, v) = (next(), next());
                        let lat = |m: f64| (((m + 128.0) * 2.0) / step as f64).floor() as i64;
                        let (kx, kz) = (lat(cx + (u - 0.5) * 2.0 * half), lat(cz + (v - 0.5) * 2.0 * half));
                        let (x, z) = (sample_meters(kx * step), sample_meters(kz * step));
                        if t.stamps().and_then(|st| st.sample(x, -z)).is_some_and(|(_, w)| w > 0.0) {
                            return None;
                        }
                        let hl = f64::from(t.height(x, z, l));
                        // Level l + 1's bilinear surface at (x, z), from its lattice (spacing 2 · step).
                        let (bx, bz) = (kx.div_euclid(2), kz.div_euclid(2));
                        let (fx, fz) = (0.5 * kx.rem_euclid(2) as f64, 0.5 * kz.rem_euclid(2) as f64);
                        let hc = |i: i64, j: i64| {
                            f64::from(t.height(
                                sample_meters((bx + i) * 2 * step),
                                sample_meters((bz + j) * 2 * step),
                                l + 1,
                            ))
                        };
                        let h00 = hc(0, 0);
                        let h10 = if fx > 0.0 { hc(1, 0) } else { h00 };
                        let h01 = if fz > 0.0 { hc(0, 1) } else { h00 };
                        let h11 = if fx > 0.0 && fz > 0.0 {
                            hc(1, 1)
                        } else if fx > 0.0 {
                            h10
                        } else {
                            h01
                        };
                        let hb = (h00 * (1.0 - fx) + h10 * fx) * (1.0 - fz) + (h01 * (1.0 - fx) + h11 * fx) * fz;
                        let (sa, ca, da, na) = level_shading(&t, x, z, l);
                        let (sb, cb, db, nb) = level_shading(&t, x, z, l + 1);
                        let dot = (na[0] * nb[0] + na[1] * nb[1] + na[2] * nb[2]).clamp(-1.0, 1.0);
                        Some(SwitchSample {
                            dh: hl - hb,
                            even: fx == 0.0 && fz == 0.0,
                            normal_deg: dot.acos().to_degrees(),
                            slope_deg: [sa, sb],
                            cliff_w: [ca, cb],
                            dark: [da, db],
                        })
                    })
                    .collect();
                let n = samples.len();
                if n == 0 {
                    println!("{l}>{} | all stamped", l + 1);
                    continue;
                }
                let mean = |f: &dyn Fn(&SwitchSample) -> f64| samples.iter().map(f).sum::<f64>() / n as f64;
                let mut adh: Vec<f64> = samples.iter().map(|p| p.dh.abs()).collect();
                let dh_mean = adh.iter().sum::<f64>() / n as f64;
                let dh_max = adh.iter().copied().fold(0.0, f64::max);
                let dh_p99 = pctl(&mut adh, 0.99);
                let mut band: Vec<f64> = samples.iter().filter(|p| p.even).map(|p| p.dh.abs()).collect();
                let band_p99 = pctl(&mut band, 0.99);
                let mut nd: Vec<f64> = samples.iter().map(|p| p.normal_deg).collect();
                let nd_mean = nd.iter().sum::<f64>() / n as f64;
                let nd_p99 = pctl(&mut nd, 0.99);
                let mut dc: Vec<f64> = samples.iter().map(|p| (p.cliff_w[0] - p.cliff_w[1]).abs()).collect();
                let dc_p99 = pctl(&mut dc, 0.99);
                // Crevice darkening: albedo × (1 − 0.45 · dark) (crevice strength default 0.45).
                let mut dd: Vec<f64> = samples.iter().map(|p| 0.45 * (p.dark[0] - p.dark[1]).abs()).collect();
                let dd_p99 = pctl(&mut dd, 0.99);
                let px = |m: f64| m * focal_px / d_switch;
                println!(
                    "{l}>{} | {d_switch:6.0} | {:4.1} | {n} | {dh_mean:.3}/{dh_p99:.3}/{dh_max:.3} | {:.2}/{:.2}/{:.2} | \
                     {band_p99:.3}/{:.2} | {nd_mean:.2}/{nd_p99:.2} | {:.1}/{:.1} | {:.3}/{:.3}, {dc_p99:.2} | \
                     {:.3}/{:.3}, {dd_p99:.3}",
                    l + 1,
                    f64::from(TileCoord::new(l, 0, 0).spacing()) * focal_px / d_switch,
                    px(dh_mean),
                    px(dh_p99),
                    px(dh_max),
                    px(band_p99),
                    mean(&|p| p.slope_deg[0]),
                    mean(&|p| p.slope_deg[1]),
                    mean(&|p| p.cliff_w[0]),
                    mean(&|p| p.cliff_w[1]),
                    mean(&|p| p.dark[0]),
                    mean(&|p| p.dark[1]),
                );
            }
            // WG-LOD after: the decoupled shading (`shade_levels`) reads no mesh LOD, so a geometry switch changes
            // no shading; the shading level changes where the camera's Chebyshev distance crosses a level's cover,
            // cross-fading over the blend band. Per level change k>k+1: what changes there (mirror of the GPU's
            // filtered lattice normals and the fixed-scale curvature), where, over how much approach, at what size.
            println!(
                "after (terrain.shade.decoupled): geometry switches change no shading. Shading level k>k+1 \
                 (cover {SHADE_COVER_TEXELS} texels, blend {SHADE_BLEND_TEXELS}, curvature scale {SHADE_CURVATURE_SCALE_M} m):"
            );
            println!(
                "k>k+1 | fade d m | band m (% of d) | texel px | n | normal deg mean/p99 | \
                 cliff_w mean k/k+1, |d| p99 | dark mean k/k+1, albedo d p99"
            );
            let after_side = side / 2;
            for k in 0u8..9 {
                let s1 = f64::from(TileCoord::new(k + 1, 0, 0).spacing());
                let half = 1500f64.max(32.0 * s1);
                let pts: Vec<(f64, f64, f64, f64, f64)> = (0..after_side * after_side)
                    .into_par_iter()
                    .filter_map(|i| {
                        let mut r = (i as u64 + 7).wrapping_mul(0x9e37_79b9_7f4a_7c15) ^ (u64::from(k) << 40);
                        let mut next = || {
                            r ^= r >> 30;
                            r = r.wrapping_mul(0xbf58_476d_1ce4_e5b9);
                            r ^= r >> 27;
                            r = r.wrapping_mul(0x94d0_49bb_1331_11eb);
                            r ^= r >> 31;
                            (r >> 11) as f64 / (1u64 << 53) as f64
                        };
                        let (x, z) = (cx + (next() - 0.5) * 2.0 * half, cz + (next() - 0.5) * 2.0 * half);
                        if t.stamps().and_then(|st| st.sample(x, -z)).is_some_and(|(_, w)| w > 0.0) {
                            return None;
                        }
                        let (na, ca) = fixed_shading(&t, x, z, k);
                        let (nb, cb) = fixed_shading(&t, x, z, k + 1);
                        let dot = (na[0] * nb[0] + na[1] * nb[1] + na[2] * nb[2]).clamp(-1.0, 1.0);
                        let ((wa, da), (wb, db)) = (cliff_and_dark(na, ca), cliff_and_dark(nb, cb));
                        Some((dot.acos().to_degrees(), wa, wb, da, db))
                    })
                    .collect();
                let n = pts.len();
                if n == 0 {
                    continue;
                }
                let mean = |f: &dyn Fn(&(f64, f64, f64, f64, f64)) -> f64| pts.iter().map(f).sum::<f64>() / n as f64;
                let mut nd: Vec<f64> = pts.iter().map(|p| p.0).collect();
                let nd_p99 = pctl(&mut nd, 0.99);
                let mut dc: Vec<f64> = pts.iter().map(|p| (p.1 - p.2).abs()).collect();
                let dc_p99 = pctl(&mut dc, 0.99);
                let mut dd: Vec<f64> = pts.iter().map(|p| 0.45 * (p.3 - p.4).abs()).collect();
                let dd_p99 = pctl(&mut dd, 0.99);
                let scale = 0.5 * f64::from(1u32 << k);
                let fade_end = SHADE_COVER_TEXELS * scale;
                let band = SHADE_BLEND_TEXELS * scale;
                println!(
                    "{k}>{} | {fade_end:6.0} | {band:5.0} ({:.0} %) | {:4.1} | {n} | {:.2}/{nd_p99:.2} | {:.3}/{:.3}, {dc_p99:.2} | \
                     {:.3}/{:.3}, {dd_p99:.3}",
                    k + 1,
                    100.0 * band / fade_end,
                    scale * focal_px / fade_end,
                    mean(&|p| p.0),
                    mean(&|p| p.1),
                    mean(&|p| p.2),
                    mean(&|p| p.3),
                    mean(&|p| p.4),
                );
            }
            // Rock cover by distance (the "grows in" measure): mean cliff_w a hillside shows at the middle of each
            // LOD band (36·2^L m), before (level L, the mesh LOD's) and after (the shading level there, blended).
            let mut cover_row = Vec::new();
            for l in 0u8..9 {
                let d = 0.75 * f64::from(cdlod.range(l as i8));
                let (k, a) = shade_level_at(d);
                let half = 1500f64.max(32.0 * f64::from(TileCoord::new(l + 1, 0, 0).spacing()));
                let both: Vec<(f64, f64)> = (0..after_side * after_side)
                    .into_par_iter()
                    .filter_map(|i| {
                        let (u, v) = (((i as f64) * 0.618_034).fract(), ((i as f64) * 0.754_878 + 0.31).fract());
                        let (x, z) = (cx + (u - 0.5) * 2.0 * half, cz + (v - 0.5) * 2.0 * half);
                        if t.stamps().and_then(|st| st.sample(x, -z)).is_some_and(|(_, w)| w > 0.0) {
                            return None;
                        }
                        let before = cliff_and_dark(lattice_normal(&t, x, z, l), 0.0).0;
                        let na = lattice_normal(&t, x, z, k);
                        let nb = if a > 0.0 && k < 9 { lattice_normal(&t, x, z, k + 1) } else { na };
                        let m: Vec<f64> = (0..3).map(|j| na[j] * (1.0 - a) + nb[j] * a).collect();
                        let len = (m[0] * m[0] + m[1] * m[1] + m[2] * m[2]).sqrt();
                        let after = cliff_and_dark([m[0] / len, m[1] / len, m[2] / len], 0.0).0;
                        Some((before, after))
                    })
                    .collect();
                let n = both.len().max(1) as f64;
                cover_row.push(format!(
                    "{d:.0} m L{l}/k{k}{}: {:.3}/{:.3}",
                    if a > 0.0 { format!("+{a:.2}") } else { String::new() },
                    both.iter().map(|p| p.0).sum::<f64>() / n,
                    both.iter().map(|p| p.1).sum::<f64>() / n
                ));
            }
            println!("rock cover by distance, before/after: {}", cover_row.join(" | "));
        }
    }
}

/// LF-2b steep-face gully fade: a world without `filter.toml` keeps every per-world value off (so `x10s` and every
/// existing world fill bit for bit as before); with it, samples under the fade's low slope are bit-identical, samples
/// past its high slope carry no gully offset at all, and a name that isn't per-world is refused.
#[test]
fn steep_fade_is_per_world_and_off_when_absent() {
    let dir = std::env::temp_dir().join(format!("sf_worldgen_steep_fade_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let absent = world_filter(&dir).unwrap();
    assert_eq!(absent.steep_fade, None);
    assert_eq!(absent, FilterParams::default());
    std::fs::write(dir.join(WORLD_FILTER_FILE), "steep_fade = 1\nsteep_fade_lo_deg = 25\nsteep_fade_hi_deg = 35\n")
        .unwrap();
    let on = world_filter(&dir).unwrap();
    let fade = on.steep_fade.expect("fade on");
    std::fs::write(dir.join(WORLD_FILTER_FILE), "relief_fraction = 0.5\n").unwrap();
    assert!(world_filter(&dir).is_err(), "a global filter value in a world file must be refused");
    let _ = std::fs::remove_dir_all(&dir);

    let off_t = tiles();
    let on_t = off_t.with_params(FilterParams { steep_fade: on.steep_fade, ..*off_t.params() });
    let base = off_t.base();
    let (mut flat, mut steep, mut mid) = (0usize, 0usize, 0usize);
    for k in 0..4000 {
        let (x, z) = (5.0 + f64::from(k % 80) * 23.7, 5.0 + f64::from(k / 80) * 37.1);
        let s = base.sample(x / 20.0, z / 20.0);
        let slope = (s.grad[0] * s.grad[0] + s.grad[1] * s.grad[1]).sqrt();
        let (a, b) = (off_t.height(x, z, 0), on_t.height(x, z, 0));
        if slope <= fade[0] {
            assert_eq!(a.to_bits(), b.to_bits(), "flat sample moved at ({x}, {z})");
            flat += 1;
        } else if slope >= fade[1] {
            assert_eq!(b.to_bits(), (s.h as f32).to_bits(), "steep sample kept a gully at ({x}, {z})");
            steep += 1;
        } else {
            mid += 1;
        }
    }
    println!("steep fade: {flat} flat (unchanged), {mid} in the ramp, {steep} steep (no gully)");
    assert!(flat > 0 && steep > 0, "fixture misses a slope class");
}
