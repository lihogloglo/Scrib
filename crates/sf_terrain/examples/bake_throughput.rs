//! Throughput of the terrain bake functions on a synthetic 6×6 km world.
//!
//! `cargo run -p sf_terrain --release --example bake_throughput`
//!
//! Source: 52×52 tiles of 64 quads at 128/70 m spacing (a typical legacy
//! heightfield density), 16×16 layer texels and 65² tint samples per tile,
//! and 40 footprints per canonical tile. Level 0: upsample, constrain,
//! bounds, encode, layers, tint. Then the whole pyramid. Per-tile work runs
//! on rayon, as the bake will.
#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]

use std::collections::BTreeMap;
use std::time::Instant;

use glam::{DVec3, UVec2, Vec2};
use rayon::prelude::*;
use sf_coords::WorldPos;
use sf_records::RecordId;
use sf_terrain::{
    BlockBounds, EncodedHeights, FOOTPRINT_FEATHER, Footprint, FootprintMask, HeightTile, LayerId, LayerMap,
    LayerParams, SourceLayers, SourceMosaic, SourceTile, SourceTileId, SourceTint, TileCoord, Triangulation,
    constrain, downsample, downsample_layers, resample_layers, resample_tint, top_level, upsample,
};

fn hash01(i: i64, j: i64) -> f32 {
    let mut x = (i as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ (j as u64).wrapping_mul(0xC2B2_AE3D_27D4_EB4F);
    x ^= x >> 31;
    x = x.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    x ^= x >> 29;
    (x >> 40) as f32 / (1u64 << 24) as f32
}

#[allow(clippy::too_many_lines)]
fn main() {
    let spacing = 128.0f32 / 70.0;
    let quads = 64u32;
    let n_src = 52i64;
    let world0 = -128.0;

    let t = Instant::now();
    let palette: Vec<RecordId> = (0..32).map(|n| RecordId::new(&format!("layer{n}"))).collect();
    let mut tiles = Vec::new();
    for tz in 0..n_src {
        for tx in 0..n_src {
            let (i0, j0) = (tx * i64::from(quads), tz * i64::from(quads));
            let origin = WorldPos::from_f64(DVec3::new(
                world0 + i0 as f64 * f64::from(spacing),
                0.0,
                world0 + j0 as f64 * f64::from(spacing),
            ));
            let n = quads + 1;
            let mut heights = Vec::with_capacity((n * n) as usize);
            for j in 0..i64::from(n) {
                for i in 0..i64::from(n) {
                    let (x, z) = ((i0 + i) as f32, (j0 + j) as f32);
                    heights.push(200.0 * (x / 300.0).sin() * (z / 410.0).cos() + 4.0 * hash01(i0 + i, j0 + j));
                }
            }
            let texels = (0..256).map(|k| (hash01(tx * 16 + k % 16, tz * 16 + k / 16) * 32.0) as u16).collect();
            let layers = SourceLayers {
                origin,
                spacing: spacing * 4.0,
                size: UVec2::splat(16),
                palette: palette.clone(),
                texels,
            };
            let tint = SourceTint {
                origin,
                spacing,
                size: UVec2::splat(n),
                rgb: (0..n * n).map(|k| [(k % 256) as u8, 200, 180]).collect(),
            };
            let tile = SourceTile {
                origin,
                spacing,
                size: UVec2::splat(n),
                heights,
                surface: Triangulation::Alternating(sf_terrain::Diagonal::Main),
                layers: Some(layers),
                tint: Some(tint),
            };
            tiles.push((SourceTileId((tz * n_src + tx) as u64), tile));
        }
    }
    let mut resolve = |id: &RecordId| LayerId(id.as_str()[5..].parse().unwrap());
    let src = SourceMosaic::new(tiles, -50.0, &mut resolve).unwrap();
    let t_mosaic = t.elapsed();

    let extent = n_src as f64 * f64::from(quads) * f64::from(spacing);
    let per_axis = (extent / 256.0).ceil() as i32;
    let coords: Vec<TileCoord> = (0..per_axis).flat_map(|z| (0..per_axis).map(move |x| TileCoord::new(0, x, z))).collect();
    let params = LayerParams { blend_radius: spacing * 4.0, default_layer: LayerId(0) };

    let t = Instant::now();
    let level0: Vec<(HeightTile, EncodedHeights, LayerMap)> = coords
        .par_iter()
        .map(|&c| {
            let mut tile = upsample(c, &src);
            let o = c.origin().to_f64();
            let footprints: Vec<Footprint> = (0..40)
                .map(|k| Footprint {
                    center: WorldPos::from_f64(DVec3::new(
                        o.x + f64::from(hash01(i64::from(c.x), k) * 256.0),
                        0.0,
                        o.z + f64::from(hash01(k, i64::from(c.z)) * 256.0),
                    )),
                    half_extents: Vec2::new(2.0 + hash01(k, k) * 8.0, 2.0 + hash01(k, -k) * 8.0),
                    yaw: hash01(-k, k) * std::f32::consts::TAU,
                })
                .collect();
            let mask = FootprintMask::from_footprints(c, &footprints, FOOTPRINT_FEATHER);
            constrain(&mut tile, &src, &mask);
            let _bounds = BlockBounds::of(&tile);
            let enc = EncodedHeights::encode(&tile);
            let layers = resample_layers(c, &src, &params).unwrap().unwrap();
            let _tint = resample_tint(c, &src).unwrap();
            (tile, enc, layers)
        })
        .collect();
    let t_level0 = t.elapsed();

    let t = Instant::now();
    let top = top_level(coords.iter().copied());
    let mut heights: BTreeMap<TileCoord, HeightTile> = BTreeMap::new();
    let mut layers: BTreeMap<TileCoord, LayerMap> = BTreeMap::new();
    for (h, _, l) in level0 {
        layers.insert(l.coord, l);
        heights.insert(h.coord, h);
    }
    let mut coarse_tiles = 0;
    let mut level_coords = coords.clone();
    for level in 1..=top {
        let mut next: Vec<TileCoord> = level_coords.iter().map(|c| c.parent()).collect();
        next.sort_unstable();
        next.dedup();
        let built: Vec<(HeightTile, LayerMap)> = next
            .par_iter()
            .map(|&c| {
                let kids = c.children().map(|k| heights.get(&k));
                let h = downsample(c, kids, -50.0);
                let _bounds = BlockBounds::of(&h);
                let _enc = EncodedHeights::encode(&h);
                let lookup = |k: TileCoord| layers.get(&k);
                let l = downsample_layers(c, &lookup, &params).unwrap();
                (h, l)
            })
            .collect();
        coarse_tiles += built.len();
        for (h, l) in built {
            layers.insert(l.coord, l);
            heights.insert(h.coord, h);
        }
        level_coords = next;
        assert!(level < 32);
    }
    let t_pyramid = t.elapsed();

    // Single-thread cost per function, over a few tiles.
    let sample: Vec<TileCoord> = coords.iter().copied().step_by(37).take(12).collect();
    let time = |name: &str, f: &dyn Fn(TileCoord)| {
        let t = Instant::now();
        for &c in &sample {
            f(c);
        }
        println!("  {name}: {:.2} ms/tile", t.elapsed().as_secs_f64() * 1e3 / sample.len() as f64);
    };
    println!("single thread:");
    time("upsample", &|c| drop(upsample(c, &src)));
    let first_tile = upsample(coords[0], &src);
    time("mask (40 footprints) + constrain", &|c| {
        let fps: Vec<Footprint> = (0..40u8)
            .map(|k| Footprint {
                center: c.origin().offset(glam::Vec3::new(f32::from(k) * 6.0, 0.0, 128.0)),
                half_extents: Vec2::splat(5.0),
                yaw: 0.3,
            })
            .collect();
        let mut t = first_tile.clone();
        t.coord = c;
        constrain(&mut t, &src, &FootprintMask::from_footprints(c, &fps, FOOTPRINT_FEATHER));
    });
    time("bounds + encode", &|_| drop((BlockBounds::of(&first_tile), EncodedHeights::encode(&first_tile))));
    time("resample_layers", &|c| drop(resample_layers(c, &src, &params)));
    time("resample_tint", &|c| drop(resample_tint(c, &src)));
    time("downsample (heights)", &|c| {
        let p = c.parent();
        drop(downsample(p, p.children().map(|k| heights.get(&k)), 0.0));
    });
    time("downsample_layers", &|c| {
        let lookup = |k: TileCoord| layers.get(&k);
        drop(downsample_layers(c.parent(), &lookup, &params));
    });

    let n0 = coords.len();
    println!("synthetic world: {:.2} km square, {} source tiles", extent / 1000.0, n_src * n_src);
    println!("threads: {}", rayon::current_num_threads());
    println!("mosaic build: {:.1} ms", t_mosaic.as_secs_f64() * 1e3);
    println!(
        "level 0: {n0} tiles in {:.2} s = {:.1} tiles/s (upsample + constrain + bounds + encode + layers + tint)",
        t_level0.as_secs_f64(),
        n0 as f64 / t_level0.as_secs_f64()
    );
    println!(
        "pyramid: {coarse_tiles} tiles over {top} levels in {:.2} s = {:.1} tiles/s",
        t_pyramid.as_secs_f64(),
        coarse_tiles as f64 / t_pyramid.as_secs_f64()
    );
    println!("total: {:.2} s", (t_mosaic + t_level0 + t_pyramid).as_secs_f64());
}
