//! Tests of the guide ground layers ([`super`]): file round trip, the frame (a texel centre reads its own
//! layer), the stamp switch at weight one half, determinism across threads, the macro pyramids, and the
//! provider's layer maps and macro look.

use super::*;
use sf_terrain::{LayerInfo, TerrainTiles};
use sf_world::WorldspaceId;

use crate::filter::FilterParams;
use crate::guide::Guide;
use crate::tiles::{GuideTiles, TileSettings};

const PALETTE: [&str; 4] = ["sand", "grass", "rock", "road"];

/// Global: 20 × 20 texels of 30 m from (−300, 300), id by (column / 3 + row / 4) mod 3, a hole at (5, 5). One
/// stamp grid: 16 × 16 texels of 5 m from (−40, 40), all "road" but a "grass" cross.
fn layers() -> GuideLayers {
    let global = GuideLayerGrid {
        origin_m: [-300.0, 300.0],
        dims: [20, 20],
        spacing_m: 30.0,
        blend_m: 22.5,
        texels: (0..400).map(|k| if k == 5 * 20 + 5 { NO_LAYER } else { (((k % 20) / 3 + (k / 20) / 4) % 3) as u16 }).collect(),
        tint: None,
    };
    let stamp = GuideLayerGrid {
        origin_m: [-40.0, 40.0],
        dims: [16, 16],
        spacing_m: 5.0,
        blend_m: 3.75,
        texels: (0..256).map(|k| if k % 16 == 8 || k / 16 == 8 { 1 } else { 3 }).collect(),
        tint: None,
    };
    GuideLayers {
        palette: PALETTE.iter().map(|s| (*s).to_owned()).collect(),
        default_layer: "default".into(),
        global,
        stamps: vec![(0, stamp)],
    }
}

/// One stamp centred on the guide origin: 41 × 41 samples at 2 m, weight 1 within 20 m, 0 from 38 m.
fn stamps() -> StampSet {
    let (n, s) = (41usize, 2.0);
    let mut weights = Vec::with_capacity(n * n);
    for r in 0..n {
        for c in 0..n {
            let d = ((c as f64 - 20.0) * s).hypot((r as f64 - 20.0) * s);
            let t = ((38.0 - d) / 18.0).clamp(0.0, 1.0);
            weights.push((t * t * (3.0 - 2.0 * t)) as f32);
        }
    }
    StampSet {
        stamps: vec![Stamp {
            name: "town".into(),
            origin_m: [-40.0, 40.0],
            dims: [n, n],
            spacing_m: s,
            heights: vec![5.0; n * n],
            weights,
        }],
    }
}

fn table() -> LayerTable {
    let info = |record: &str, c: [f32; 3]| LayerInfo {
        record: record.into(),
        id: layer_id(record),
        tiling_m: 4.0,
        triplanar: false,
        displacement_m: 0.0,
        mean_albedo: c,
        mean_height: 0.5,
        p90_height: 0.6,
        source_edge: 64,
    };
    LayerTable {
        layers: vec![
            info("sand", [0.5, 0.4, 0.3]),
            info("grass", [0.1, 0.3, 0.05]),
            info("rock", [0.2, 0.2, 0.2]),
            info("road", [0.6, 0.6, 0.55]),
            info("default", [0.3, 0.25, 0.2]),
        ],
        fill_layer: Some(layer_id("default")),
        fill_height: 0.0,
    }
}

fn field() -> LayerField {
    LayerField::new(layers(), 1).unwrap()
}

/// Level-0 global sample index of guide-frame metres (x or −y), as `sample_meters` inverted.
fn g_of(m: f64) -> i64 {
    ((m + 128.0) * 2.0).round() as i64
}

#[test]
fn files_round_trip() {
    let dir = std::env::temp_dir().join(format!("sf_worldgen_layers_rt_{}", std::process::id()));
    let l = layers();
    l.write(&dir).unwrap();
    let back = GuideLayers::read(&dir).unwrap().unwrap();
    assert_eq!(back, l);
    assert_eq!(std::fs::metadata(dir.join("layers.u16")).unwrap().len(), 2 * l.texel_count() as u64);
    assert!(GuideLayers::read(&dir.join("nothing")).unwrap().is_none());
    // A texel past the palette is refused.
    let mut bad = l.clone();
    bad.global.texels[0] = 9;
    assert!(bad.write(&dir).is_err());
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn a_texel_centre_reads_its_own_layer() {
    let f = field();
    let l = layers();
    let ids = f.ids().to_vec();
    for (c, r) in [(0usize, 0usize), (7, 3), (19, 19), (5, 5), (12, 17)] {
        let (x, y) = (-300.0 + 30.0 * c as f64 + 15.0, 300.0 - 30.0 * r as f64 - 15.0);
        let (gx, gz) = (g_of(x), g_of(-y));
        let coord = TileCoord::new(0, gx.div_euclid(512) as i32, gz.div_euclid(512) as i32);
        let map = f.layer_map(coord, None).unwrap();
        let (gx0, gz0) = coord.first_sample();
        let s = map.sample(((gx - gx0) / 2) as usize, ((gz - gz0) / 2) as usize);
        let t = l.global.texels[r * 20 + c];
        let want = if t == NO_LAYER { layer_id("default") } else { ids[usize::from(t)] };
        assert_eq!(s[0], (want, 255), "texel ({c}, {r})");
    }
}

#[test]
fn stamp_grid_only_where_its_weight_is_over_one_half() {
    let f = field();
    let set = stamps();
    let c = TileCoord::new(0, 0, 0);
    let mixed = f.layer_map(c, Some(&set)).unwrap();
    let global = f.layer_map(c, None).unwrap();
    let only_stamp = LayerField::new(GuideLayers { global: layers().stamps[0].1.clone(), stamps: Vec::new(), ..layers() }, 0)
        .unwrap()
        .layer_map(c, None)
        .unwrap();
    let (mut inside, mut outside) = (0, 0);
    for j in 0..sf_terrain::LAYER_SAMPLES {
        for i in 0..sf_terrain::LAYER_SAMPLES {
            let (gx, gz) = c.global_sample(2 * i, 2 * j);
            let w = set.sample(sample_meters(gx), -sample_meters(gz)).map_or(0.0, |s| s.1);
            if w > STAMP_LAYER_WEIGHT {
                inside += 1;
                assert_eq!(mixed.sample(i, j), only_stamp.sample(i, j), "({i}, {j}) w {w}");
            } else {
                outside += 1;
                assert_eq!(mixed.sample(i, j), global.sample(i, j), "({i}, {j}) w {w}");
            }
        }
    }
    assert!(inside > 1000 && outside > 1000, "{inside} inside, {outside} outside");
    // A tile no stamp reaches is the global map itself.
    assert_eq!(f.layer_map(TileCoord::new(0, -2, -2), Some(&set)).unwrap(), f.layer_map(TileCoord::new(0, -2, -2), None).unwrap());
}

#[test]
fn layer_maps_are_bitwise_across_threads_and_levels_agree_on_edges() {
    let f = std::sync::Arc::new(field());
    let set = std::sync::Arc::new(stamps());
    let coords = [TileCoord::new(0, 0, 0), TileCoord::new(0, -1, 0), TileCoord::new(1, -1, -1), TileCoord::new(2, 0, 0)];
    let serial: Vec<LayerMap> = coords.iter().map(|&c| f.layer_map(c, Some(&set)).unwrap()).collect();
    let threads: Vec<_> = coords
        .iter()
        .map(|&c| {
            let (f, set) = (f.clone(), set.clone());
            std::thread::spawn(move || f.layer_map(c, Some(&set)).unwrap())
        })
        .collect();
    for (t, s) in threads.into_iter().zip(&serial) {
        assert_eq!(&t.join().unwrap(), s);
    }
    let (a, b) = (&serial[1], &serial[0]);
    for n in 0..sf_terrain::LAYER_SAMPLES {
        assert_eq!(a.sample(sf_terrain::LAYER_SAMPLES - 1, n), b.sample(0, n), "shared edge {n}");
    }
}

#[test]
#[allow(clippy::float_cmp)] // the same pyramid texel read twice: bitwise equal
fn macro_albedo_is_the_layer_mean_and_coarsens() {
    let mut f = field();
    assert!(f.albedo(0, 0, 0, None, &[]).is_none(), "no means yet");
    assert!(f.with_means(&table()).is_empty());
    let set = stamps();
    let close = |a: [f32; 3], b: [f32; 3]| (0..3).all(|k| (a[k] - b[k]).abs() <= 0.6 / 1023.0);
    // A global texel centre at macro level 0 (1 m samples): its own mean.
    let (x, y) = (-300.0 + 30.0 * 2.0 + 15.0, 300.0 - 15.0);
    let a = f.albedo(g_of(x), g_of(-y), 0, Some(&set), &[0]).unwrap();
    assert!(close(a, [0.5, 0.4, 0.3]), "{a:?}");
    // The hole: the default layer's mean.
    let (x, y) = (-300.0 + 30.0 * 5.0 + 15.0, 300.0 - 30.0 * 5.0 - 15.0);
    assert!(close(f.albedo(g_of(x), g_of(-y), 0, None, &[]).unwrap(), [0.3, 0.25, 0.2]));
    // The stamp's centre: its "grass" cross (texel (8, 8)) inside; the global grid ("rock" there) without it.
    let (x, y) = (-40.0 + 8.0 * 5.0 + 2.5, 40.0 - 8.0 * 5.0 - 2.5);
    assert!(close(f.albedo(g_of(x), g_of(-y), 0, Some(&set), &[0]).unwrap(), [0.1, 0.3, 0.05]));
    assert!(close(f.albedo(g_of(x), g_of(-y), 0, None, &[]).unwrap(), [0.2, 0.2, 0.2]));
    // Very far (4 km samples): the 1 × 1 top of the pyramid, one colour over the whole grid.
    let far = f.albedo(g_of(-300.0), g_of(-300.0), 12, None, &[]).unwrap();
    assert_eq!(far, f.albedo(g_of(250.0), g_of(250.0), 12, None, &[]).unwrap());
    assert_ne!(far, f.albedo(g_of(-285.0), g_of(-285.0), 0, None, &[]).unwrap());
    assert_eq!(f.gpu_words(1000).len(), f.gpu_word_count());
}

fn small_guide() -> Guide {
    let (w, h) = (64usize, 64usize);
    let heights = (0..w * h).map(|k| 20.0 + (k % w) as f32 * 0.5).collect();
    Guide { w, h, spacing_m: 20.0, sea_level_m: 0.0, fill_m: None, origin_m: [-640.0, 640.0], heights, regions: None }
}

#[test]
fn provider_layer_maps_and_macro_look() {
    let t = GuideTiles::new(small_guide(), WorldspaceId::new("test"), FilterParams::default(), TileSettings::default())
        .with_stamps(stamps())
        .with_layers(layers())
        .unwrap();
    assert!(t.has_layer_maps());
    let c = TileCoord::new(0, 0, 0);
    let map = t.layer_map(c).unwrap().unwrap();
    assert_eq!(map, field().layer_map(c, Some(&stamps())).unwrap());
    assert!(t.product(c).layers.is_none(), "height tiles carry no layers");
    assert!(t.layers_capped(None, &sf_terrain::LayerCap::default()).is_none(), "no arrays handed in");
    let fallback = t.macro_rgba(c);
    let arrays: crate::tiles::LayerArraysFn = std::sync::Arc::new(|_, _| None);
    let (t, missing) = t.with_layer_arrays(&table(), arrays).unwrap();
    assert!(missing.is_empty());
    let look = t.macro_rgba(c);
    assert_ne!(look, fallback, "the layers' look replaces the fallback");
    assert!(t.gpu_data()[crate::tiles::slot::LAYERS as usize] != 0);
    assert_eq!(t.gpu_data().len() * 4, t.gpu_data_bytes());
}

/// The same map at 10x when it exists (`target/worldgen/x10s` with `worldgen-layers` run): layer map times per
/// level at the first stamp and in the open, and the GPU field's size with the layer pyramids.
#[test]
#[ignore = "measurement: needs target/worldgen/x10s with layers.json"]
fn measure_layers_on_x10s() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../target/worldgen/x10s");
    let started = std::time::Instant::now();
    let t = GuideTiles::open(&dir, WorldspaceId::new("worldgen")).expect("x10s guide");
    let open_ms = started.elapsed().as_secs_f64() * 1e3;
    let field = t.layers().expect("layers.json");
    // A table knowing every palette layer (grey ramp): the pyramids' cost, not the look.
    let layers = field
        .layers()
        .palette
        .iter()
        .chain(std::iter::once(&field.layers().default_layer))
        .enumerate()
        .map(|(n, r)| LayerInfo {
            record: r.clone(),
            id: layer_id(r),
            tiling_m: 7.3,
            triplanar: false,
            displacement_m: 0.0,
            mean_albedo: [(n % 10) as f32 * 0.05 + 0.05; 3],
            mean_height: 0.5,
            p90_height: 0.6,
            source_edge: 256,
        })
        .collect();
    let table = LayerTable { layers, fill_layer: None, fill_height: 0.0 };
    let started = std::time::Instant::now();
    let (t, missing) = t.with_layer_arrays(&table, std::sync::Arc::new(|_, _| None)).unwrap();
    let means_ms = started.elapsed().as_secs_f64() * 1e3;
    assert!(missing.is_empty());
    let st = &t.stamps().expect("stamps").set().stamps[0];
    let [x0, y0, x1, y1] = st.rect_m();
    let (sx, sz) = (f64::midpoint(x0, x1), -f64::midpoint(y0, y1));
    for (name, x, z) in [(st.name.as_str(), sx, sz), ("open land", 0.0, 0.0)] {
        for level in 0u8..3 {
            let c = tile_at(x, z, level);
            let started = std::time::Instant::now();
            let m = t.layer_map(c).unwrap().unwrap();
            eprintln!(
                "x10s layer map {name} level {level} {c:?}: {:.1} ms, palette {}",
                started.elapsed().as_secs_f64() * 1e3,
                m.palette.len()
            );
        }
    }
    let started = std::time::Instant::now();
    let _ = t.macro_rgba(TileCoord::new(4, 0, 0));
    eprintln!(
        "x10s: open {open_ms:.0} ms, pyramids {means_ms:.0} ms, a macro tile {:.1} ms; GPU field {:.1} MiB (layers {:.2} MiB)",
        started.elapsed().as_secs_f64() * 1e3,
        t.gpu_data_bytes() as f64 / f64::from(1u32 << 20),
        (t.layers().unwrap().gpu_word_count() * 4) as f64 / f64::from(1u32 << 20),
    );
    assert!(t.gpu_data_bytes() <= 128 << 20, "one storage binding");
}

/// The level-`level` tile holding engine (x, z) metres (`TileCoord::containing` without `sf_coords`).
fn tile_at(x: f64, z: f64, level: u8) -> TileCoord {
    let t = |v: f64| g_of(v).div_euclid(512 << level) as i32;
    TileCoord::new(level, t(x), t(z))
}
