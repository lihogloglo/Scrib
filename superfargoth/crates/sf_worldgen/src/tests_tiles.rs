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
