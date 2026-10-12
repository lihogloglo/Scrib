// Exact float equality is what several tests check: seams and pass-through are bitwise.
#![allow(clippy::float_cmp, clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]

use std::sync::Arc;

use glam::{DVec3, IVec3, UVec2, Vec2, Vec3};
use sf_coords::WorldPos;
use sf_records::RecordId;

use super::*;

/// Deterministic pseudo-random value in [0, 1) for a lattice point.
fn hash01(i: i64, j: i64) -> f32 {
    let mut x = (i as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ (j as u64).wrapping_mul(0xC2B2_AE3D_27D4_EB4F);
    x ^= x >> 31;
    x = x.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    x ^= x >> 29;
    (x >> 40) as f32 / (1u64 << 24) as f32
}

/// Rough terrain: smooth hills plus spiky noise, so Catmull-Rom overshoots.
fn rough(i: i64, j: i64) -> f32 {
    let (x, z) = (i as f32, j as f32);
    30.0 * (x / 23.0).sin() * (z / 31.0).cos() + 8.0 * hash01(i, j) + if hash01(j, i) > 0.97 { 25.0 } else { 0.0 }
}

/// A grid of `nx`×`nz` source tiles with `quads` quads each (shared borders),
/// lattice `spacing` with sample (0, 0) of tile (0, 0) at absolute
/// (`x0`, `z0`). Heights are `f(i, j)` of the global lattice index.
fn source_tiles(
    (nx, nz): (i64, i64),
    quads: u32,
    spacing: f32,
    (x0, z0): (f64, f64),
    surface: Triangulation,
    f: impl Fn(i64, i64) -> f32,
) -> Vec<(SourceTileId, SourceTile)> {
    let n = quads + 1;
    let mut tiles = Vec::new();
    for tz in 0..nz {
        for tx in 0..nx {
            let (i0, j0) = (tx * i64::from(quads), tz * i64::from(quads));
            let origin = WorldPos::from_f64(DVec3::new(
                x0 + i0 as f64 * f64::from(spacing),
                0.0,
                z0 + j0 as f64 * f64::from(spacing),
            ));
            let mut heights = Vec::new();
            for j in 0..i64::from(n) {
                for i in 0..i64::from(n) {
                    heights.push(f(i0 + i, j0 + j));
                }
            }
            let id = SourceTileId((tz * 1000 + tx) as u64);
            let tile = SourceTile {
                origin,
                spacing,
                size: UVec2::splat(n),
                heights,
                surface,
                layers: None,
                tint: None,
            };
            tiles.push((id, tile));
        }
    }
    tiles
}

fn no_layers(_: &RecordId) -> LayerId {
    LayerId(0)
}

fn mosaic(tiles: Vec<(SourceTileId, SourceTile)>) -> SourceMosaic {
    SourceMosaic::new(tiles, -10.0, &mut no_layers).unwrap()
}

/// Absolute meters of level-0 global sample `g`.
fn meters(g: i64) -> f64 {
    g as f64 * 0.5 - 128.0
}

/// Source covering canonical tiles (0..2, 0..2) and their surroundings at
/// an awkward spacing and phase.
fn awkward_world(surface: Triangulation) -> SourceMosaic {
    // 128/70 m spacing, origin off the 0.5 m lattice.
    let s = 128.0 / 70.0;
    mosaic(source_tiles((10, 10), 64, s, (-140.3, -141.7), surface, rough))
}

#[test]
fn upsample_passes_through_source_samples() {
    for (spacing, phase) in [(2.0f32, 1.0f64), (1.5, -0.5), (4.0, 0.0)] {
        let src = mosaic(source_tiles((10, 10), 32, spacing, (-200.0 + phase, -200.0 + phase), Triangulation::Bilinear, rough));
        let tile = upsample(TileCoord::new(0, 0, 0), &src);
        let mut checked = 0;
        for j in 0..TILE_SAMPLES {
            for i in 0..TILE_SAMPLES {
                let (gx, gz) = tile.coord.global_sample(i, j);
                let (u, v) = ((meters(gx) - (-200.0 + phase)) / f64::from(spacing), (meters(gz) - (-200.0 + phase)) / f64::from(spacing));
                if u.fract() == 0.0 && v.fract() == 0.0 {
                    let expected = rough(u as i64, v as i64);
                    assert!((tile.get(i, j) - expected).abs() <= 1e-3, "spacing {spacing} at ({i}, {j})");
                    checked += 1;
                }
            }
        }
        assert!(checked > 1000, "spacing {spacing}: only {checked} coincident samples");
    }
}

#[test]
fn upsample_never_leaves_the_neighborhood_range() {
    let src = awkward_world(Triangulation::Bilinear);
    let lattice = src.heights.lattice;
    let tile = upsample(TileCoord::new(0, 1, 0), &src);
    let mut clamped = 0;
    for j in 0..TILE_SAMPLES {
        for i in 0..TILE_SAMPLES {
            // The 4×4 around the sample on the mosaic's own lattice. (At an exact
            // sample either neighbor pair is "the" 4×4, so compute it the same way.)
            let (gx, gz) = tile.coord.global_sample(i, j);
            let (u, v) = (lattice.coord(gx, 0), lattice.coord(gz, 1));
            let w = src.heights.gather([u.floor() as i64 - 1, v.floor() as i64 - 1], [4, 4], f32::NAN);
            let lo = w.data.iter().copied().fold(f32::INFINITY, f32::min);
            let hi = w.data.iter().copied().fold(f32::NEG_INFINITY, f32::max);
            assert!(w.data.iter().all(|h| !h.is_nan()), "inside the source");
            let h = tile.get(i, j);
            assert!(h >= lo && h <= hi, "({i}, {j}): {h} outside [{lo}, {hi}]");
            if h == lo || h == hi {
                clamped += 1;
            }
        }
    }
    // The spikes make the unclamped filter overshoot somewhere.
    assert!(clamped > 0);
}

#[test]
fn finer_sources_are_prefiltered() {
    // 0.25 m source: a downsample, so it averages instead of aliasing.
    let src = mosaic(source_tiles((5, 5), 256, 0.25, (-140.0, -140.0), Triangulation::Bilinear, |i, j| {
        if (i + j) % 2 == 0 { 1.0 } else { -1.0 }
    }));
    let tile = upsample(TileCoord::new(0, 0, 0), &src);
    for h in &tile.heights {
        assert!(h.abs() < 0.05, "checkerboard should average out, got {h}");
    }
}

fn assert_seams(a: &HeightTile, b: &HeightTile) {
    // `b` is east of `a` or north of `a`.
    let east = b.coord.x == a.coord.x + 1;
    for n in 0..TILE_SAMPLES {
        let (ha, hb) = if east { (a.get(TILE_QUADS, n), b.get(0, n)) } else { (a.get(n, TILE_QUADS), b.get(n, 0)) };
        assert_eq!(ha.to_bits(), hb.to_bits(), "{:?} / {:?} at {n}", a.coord, b.coord);
    }
}

fn footprints() -> Vec<Footprint> {
    // One straddling the seam x = tile (0,0) | (1,0), rotated; one inside.
    let at = |x: f64, z: f64| WorldPos::from_f64(DVec3::new(x, 0.0, z));
    vec![
        Footprint { center: at(meters(512), meters(100)), half_extents: Vec2::new(6.0, 3.0), yaw: 0.7 },
        Footprint { center: at(meters(200), meters(300)), half_extents: Vec2::new(10.0, 10.0), yaw: 0.0 },
        Footprint { center: at(meters(300), meters(512)), half_extents: Vec2::new(2.0, 8.0), yaw: -1.2 },
    ]
}

fn bake0(src: &SourceMosaic, coord: TileCoord) -> HeightTile {
    let mut tile = upsample(coord, src);
    let mask = FootprintMask::from_footprints(coord, &footprints(), FOOTPRINT_FEATHER);
    constrain(&mut tile, src, &mask);
    tile
}

#[test]
fn shared_edges_are_bitwise_equal_at_every_level() {
    let src = awkward_world(Triangulation::Alternating(Diagonal::Main));
    let mut level0 = std::collections::BTreeMap::new();
    for z in 0..4 {
        for x in 0..4 {
            let c = TileCoord::new(0, x, z);
            level0.insert(c, bake0(&src, c));
        }
    }
    assert_seams(&level0[&TileCoord::new(0, 0, 0)], &level0[&TileCoord::new(0, 1, 0)]);
    assert_seams(&level0[&TileCoord::new(0, 0, 0)], &level0[&TileCoord::new(0, 0, 1)]);
    assert_seams(&level0[&TileCoord::new(0, 2, 1)], &level0[&TileCoord::new(0, 3, 1)]);

    let parent = |c: TileCoord, tiles: &std::collections::BTreeMap<TileCoord, HeightTile>| {
        let kids = c.children().map(|k| tiles.get(&k));
        downsample(c, kids, -10.0)
    };
    let mut level1 = std::collections::BTreeMap::new();
    for z in 0..2 {
        for x in 0..2 {
            let c = TileCoord::new(1, x, z);
            level1.insert(c, parent(c, &level0));
        }
    }
    assert_seams(&level1[&TileCoord::new(1, 0, 0)], &level1[&TileCoord::new(1, 1, 0)]);
    assert_seams(&level1[&TileCoord::new(1, 0, 1)], &level1[&TileCoord::new(1, 1, 1)]);
    assert_seams(&level1[&TileCoord::new(1, 0, 0)], &level1[&TileCoord::new(1, 0, 1)]);

    // Stored form: decoded borders stay bitwise equal whatever each tile's step.
    let a = EncodedHeights::encode(&level0[&TileCoord::new(0, 0, 0)]).decode(TileCoord::new(0, 0, 0));
    let b = EncodedHeights::encode(&level0[&TileCoord::new(0, 1, 0)]).decode(TileCoord::new(0, 1, 0));
    assert_seams(&a, &b);
}

#[test]
fn coarse_samples_equal_fine_samples_at_even_indices() {
    let src = awkward_world(Triangulation::Bilinear);
    let kids = TileCoord::new(1, 0, 0).children().map(|c| upsample(c, &src));
    let parent = downsample(TileCoord::new(1, 0, 0), [Some(&kids[0]), Some(&kids[1]), Some(&kids[2]), Some(&kids[3])], 0.0);
    for j in 0..TILE_SAMPLES {
        for i in 0..TILE_SAMPLES {
            let (gx, gz) = parent.coord.global_sample(i, j);
            // Find the child that owns this global sample and compare.
            for kid in &kids {
                let (kx, kz) = kid.coord.first_sample();
                let (di, dj) = (gx - kx, gz - kz);
                if (0..=512).contains(&di) && (0..=512).contains(&dj) {
                    assert_eq!(parent.get(i, j).to_bits(), kid.get(di as usize, dj as usize).to_bits());
                }
            }
        }
    }
    assert_eq!(top_level([TileCoord::new(0, 0, 0)]), 0);
    // Tiles on both sides of the grid corner never merge: up to 2×2 roots.
    assert_eq!(top_level([TileCoord::new(0, -1, 0), TileCoord::new(0, 0, 0)]), 0);
    assert_eq!(top_level([TileCoord::new(0, -3, 0), TileCoord::new(0, 0, 0)]), 2);
    assert_eq!(top_level([TileCoord::new(0, 0, 0), TileCoord::new(0, 23, 23)]), 4);
}

#[test]
fn footprints_match_the_source_surface() {
    for surface in [
        Triangulation::Bilinear,
        Triangulation::Fixed(Diagonal::Anti),
        Triangulation::Alternating(Diagonal::Main),
    ] {
        let src = awkward_world(surface);
        let coord = TileCoord::new(0, 0, 0);
        let raw = upsample(coord, &src);
        let tile = bake0(&src, coord);
        let fp = footprints()[1];
        let c = fp.center.to_f64();
        let (mut inside, mut far) = (0, 0);
        for j in 0..TILE_SAMPLES {
            for i in 0..TILE_SAMPLES {
                let (gx, gz) = coord.global_sample(i, j);
                let (x, z) = (meters(gx), meters(gz));
                let d = (x - c.x).abs().max((z - c.z).abs());
                if d <= 10.0 {
                    let reference = src.reference_height(x, z);
                    assert!((tile.get(i, j) - reference).abs() <= 0.01, "{surface:?} ({i}, {j})");
                    inside += 1;
                } else if d > 10.0 + f64::from(FOOTPRINT_FEATHER) + 0.01 && d < 30.0 {
                    assert_eq!(tile.get(i, j), raw.get(i, j), "{surface:?}: outside the feather");
                    far += 1;
                }
            }
        }
        assert!(inside > 1000 && far > 1000);
    }
}

#[test]
fn reference_surface_follows_the_triangulation() {
    // One quad with corners 0, 0, 0, 1 (h11 = 1): the diagonals differ at its center.
    let tiles = |surface| {
        let t = SourceTile {
            origin: WorldPos::ORIGIN,
            spacing: 1.0,
            size: UVec2::splat(2),
            heights: vec![0.0, 0.0, 0.0, 1.0],
            surface,
            layers: None,
            tint: None,
        };
        mosaic(vec![(SourceTileId(0), t)])
    };
    let at = |m: &SourceMosaic, x, z| m.reference_height(x, z);
    assert!((at(&tiles(Triangulation::Bilinear), 0.5, 0.5) - 0.25).abs() < 1e-6);
    assert!((at(&tiles(Triangulation::Fixed(Diagonal::Main)), 0.5, 0.5) - 0.5).abs() < 1e-6);
    assert!(at(&tiles(Triangulation::Fixed(Diagonal::Anti)), 0.5, 0.5).abs() < 1e-6);
    assert!((at(&tiles(Triangulation::Fixed(Diagonal::Anti)), 0.75, 0.75) - 0.5).abs() < 1e-6);
}

#[test]
fn results_are_independent_of_tile_sets_and_threads() {
    let all = awkward_world(Triangulation::Fixed(Diagonal::Main));
    let coord = TileCoord::new(0, 1, 1);
    let reference = bake0(&all, coord);

    // Only the source tiles overlapping the canonical tile (what a bake step reads).
    let s = 128.0 / 70.0;
    let subset: Vec<_> = source_tiles((10, 10), 64, s, (-140.3, -141.7), Triangulation::Fixed(Diagonal::Main), rough)
        .into_iter()
        .filter(|(_, t)| {
            let o = t.origin.to_f64();
            let span = 64.0 * f64::from(s);
            o.x < meters(1024) + 5.0 && o.x + span > meters(512) - 5.0 && o.z < meters(1024) + 5.0 && o.z + span > meters(512) - 5.0
        })
        .collect();
    assert!(subset.len() < 20);
    assert_eq!(bake0(&mosaic(subset), coord), reference);

    // Any thread count gives the same bits.
    let coords: Vec<_> = (0..4).flat_map(|z| (0..4).map(move |x| TileCoord::new(0, x, z))).collect();
    let run = |threads| {
        let pool = rayon::ThreadPoolBuilder::new().num_threads(threads).build().unwrap();
        pool.install(|| {
            use rayon::prelude::*;
            coords.par_iter().map(|&c| EncodedHeights::encode(&bake0(&all, c))).collect::<Vec<_>>()
        })
    };
    assert_eq!(run(1), run(4));
}

#[test]
fn lattice_noise_is_canonicalized() {
    // Origins carrying f32 noise from `WorldPos` still form one lattice.
    let mut tiles = source_tiles((3, 3), 64, 128.0 / 70.0, (1000.0, -3000.0), Triangulation::Bilinear, rough);
    let jitter = WorldPos::new(IVec3::ZERO, Vec3::new(3e-6, 0.0, -2e-6));
    tiles[4].1.origin = tiles[4].1.origin.offset(jitter.local);
    assert!(SourceMosaic::new(tiles, 0.0, &mut no_layers).is_ok());

    let mut bad = source_tiles((2, 1), 64, 2.0, (0.0, 0.0), Triangulation::Bilinear, rough);
    bad[1].1.origin = bad[1].1.origin.offset(Vec3::new(0.3, 0.0, 0.0));
    assert!(matches!(SourceMosaic::new(bad, 0.0, &mut no_layers), Err(TerrainError::OffLattice { .. })));
}

#[test]
fn quantization_stays_within_two_centimeters() {
    let src = awkward_world(Triangulation::Bilinear);
    let tile = upsample(TileCoord::new(0, 0, 0), &src);
    let enc = EncodedHeights::encode(&tile);
    let EncodedHeights::U16 { step, .. } = &enc else { panic!("small range must quantize") };
    assert!(*step <= MAX_QUANT_STEP);
    let dec = enc.decode(tile.coord);
    for (a, b) in tile.heights.iter().zip(&dec.heights) {
        assert!((a - b).abs() <= enc.max_error() * 1.01 + 1e-5);
    }

    let mut cliff = HeightTile::flat(TileCoord::new(0, 0, 0), 0.0);
    cliff.set(10, 10, 2000.0);
    let enc = EncodedHeights::encode(&cliff);
    assert!(matches!(enc, EncodedHeights::F32(_)));
    assert_eq!(enc.decode(cliff.coord), cliff);

    let flat = HeightTile::flat(TileCoord::new(0, 0, 0), 12.5);
    assert_eq!(EncodedHeights::encode(&flat).decode(flat.coord), flat);
}

#[test]
fn block_bounds_cover_their_samples() {
    let src = awkward_world(Triangulation::Bilinear);
    let tile = upsample(TileCoord::new(0, 0, 0), &src);
    let bounds = BlockBounds::of(&tile);
    for bz in [0, 7, 31] {
        for bx in [0, 13, 31] {
            let (mut lo, mut hi) = (f32::INFINITY, f32::NEG_INFINITY);
            for j in bz * BLOCK_QUADS..=(bz + 1) * BLOCK_QUADS {
                for i in bx * BLOCK_QUADS..=(bx + 1) * BLOCK_QUADS {
                    lo = lo.min(tile.get(i, j));
                    hi = hi.max(tile.get(i, j));
                }
            }
            assert_eq!(bounds.get(bx, bz), (lo, hi), "block ({bx}, {bz})");
        }
    }
}

/// Four source tiles with layer texels: id = texel x band (8 m wide).
fn layered_world() -> SourceMosaic {
    let mut tiles = source_tiles((4, 4), 64, 2.0, (-200.0, -200.0), Triangulation::Bilinear, |_, _| 0.0);
    for (_, t) in &mut tiles {
        let o = t.origin.to_f64();
        let texels = 16u32;
        let palette: Vec<RecordId> = (0..64).map(|n| RecordId::new(&format!("layer{n}"))).collect();
        let mut ids = Vec::new();
        for j in 0..texels {
            for i in 0..texels {
                let x = o.x + f64::from(i) * 8.0 + 4.0;
                let z = o.z + f64::from(j) * 8.0 + 4.0;
                let band = ((x / 8.0).floor() as i64 + (z / 24.0).floor() as i64).rem_euclid(64);
                ids.push(band as u16);
            }
        }
        t.layers = Some(SourceLayers { origin: t.origin, spacing: 8.0, size: UVec2::splat(texels), palette, texels: ids });
        t.tint = Some(SourceTint {
            origin: t.origin,
            spacing: 8.0,
            size: UVec2::splat(17),
            rgb: (0..17 * 17).map(|n| [(n % 256) as u8, 128, 64]).collect(),
        });
    }
    let mut resolve = |id: &RecordId| LayerId(id.as_str()[5..].parse().unwrap());
    SourceMosaic::new(tiles, 0.0, &mut resolve).unwrap()
}

const LAYER_PARAMS: LayerParams = LayerParams { blend_radius: 8.0, default_layer: LayerId(999) };

#[test]
fn layer_maps_blend_and_agree_on_seams() {
    let src = layered_world();
    let maps: Vec<LayerMap> = [(0, 0), (1, 0), (0, 1), (1, 1), (-1, 0), (-1, 1), (0, -1)]
        .into_iter()
        .map(|(x, z)| resample_layers(TileCoord::new(0, x, z), &src, &LAYER_PARAMS).unwrap().unwrap())
        .collect();
    let a = &maps[0];
    for n in 0..LAYER_SAMPLES {
        assert_eq!(a.sample(LAYER_SAMPLES - 1, n), maps[1].sample(0, n));
        assert_eq!(a.sample(n, LAYER_SAMPLES - 1), maps[2].sample(n, 0));
    }
    for j in 0..LAYER_SAMPLES {
        for i in 0..LAYER_SAMPLES {
            let s = a.sample(i, j);
            assert_eq!(s.iter().map(|e| u32::from(e.1)).sum::<u32>(), 255);
            assert!(s.windows(2).all(|w| w[0].1 >= w[1].1), "heaviest first");
        }
    }
    // At a texel center the texel's own layer dominates; between texels, a blend.
    assert!(a.sample(4, 4).iter().filter(|e| e.1 > 0).count() >= 1);
    assert!((0..LAYER_SAMPLES).any(|i| a.sample(i, 4).iter().filter(|e| e.1 > 0).count() >= 2));

    // Coarser level: seams agree too, and weights still sum to 255.
    let lookup = |c: TileCoord| {
        maps.iter().find(|m| m.coord == c)
    };
    let p0 = downsample_layers(TileCoord::new(1, 0, 0), &lookup, &LAYER_PARAMS).unwrap();
    let p1 = downsample_layers(TileCoord::new(1, -1, 0), &lookup, &LAYER_PARAMS).unwrap();
    for n in 0..LAYER_SAMPLES {
        assert_eq!(p1.sample(LAYER_SAMPLES - 1, n), p0.sample(0, n));
        assert_eq!(p0.sample(n, 7).iter().map(|e| u32::from(e.1)).sum::<u32>(), 255);
    }

    // Tint: level 0 from the source, level 1 at even indices.
    let tints: Vec<TintMap> = TileCoord::new(1, 0, 0).children().map(|c| resample_tint(c, &src).unwrap()).into();
    let t1 = downsample_tint(TileCoord::new(1, 0, 0), [Some(&tints[0]), Some(&tints[1]), Some(&tints[2]), Some(&tints[3])]);
    assert_eq!(t1.rgb[2 * TINT_SAMPLES + 3], tints[0].rgb[4 * TINT_SAMPLES + 6]);
    assert_eq!(tints[0].rgb[0][1], 128);
}

fn resident(src: &SourceMosaic, coords: &[TileCoord]) -> TerrainHeights {
    let mut h = TerrainHeights::new();
    for &c in coords {
        h.insert(Arc::new(ResidentTile::new(upsample(c, src))));
    }
    h
}

#[test]
fn heights_query_reads_the_finest_resident_level() {
    let src = awkward_world(Triangulation::Bilinear);
    let fine = upsample(TileCoord::new(0, 0, 0), &src);
    let kids = TileCoord::new(1, 0, 0).children().map(|c| upsample(c, &src));
    let coarse = downsample(TileCoord::new(1, 0, 0), [Some(&kids[0]), Some(&kids[1]), Some(&kids[2]), Some(&kids[3])], 0.0);
    let mut h = TerrainHeights::new();
    h.insert(Arc::new(ResidentTile::new(coarse)));
    h.insert(Arc::new(ResidentTile::new(fine.clone())));

    let at = |i: i64, j: i64| WorldPos::from_f64(DVec3::new(meters(i), 0.0, meters(j)));
    // Odd sample of level 0: only the fine tile has it.
    assert_eq!(h.height(at(101, 77)), Some(fine.get(101, 77)));
    // Inside tile (1, 0) of level 0, which isn't resident: level 1 answers.
    assert_eq!(h.height(at(600, 20)), Some(kids[1].get(88, 20)));
    assert_eq!(h.height(at(5000, 0)), None);

    // Between samples: on the canonical triangles.
    let p = WorldPos::from_f64(DVec3::new(meters(10) + 0.4, 0.0, meters(20) + 0.1));
    let (h00, h10, h11) = (fine.get(10, 20), fine.get(11, 20), fine.get(11, 21));
    let expected = h00 + (h10 - h00) * 0.8 + (h11 - h10) * 0.2;
    assert!((h.height(p).unwrap() - expected).abs() < 1e-4);

    let n = h.normal(p).unwrap();
    assert!((n.length() - 1.0).abs() < 1e-5 && n.y > 0.0);

    let rect = WorldRect { min: at(500, 0), size: Vec2::new(20.0, 3.0) };
    let (nx, nz) = rect.sample_count(1.0);
    assert_eq!((nx, nz), (21, 4));
    let mut out = vec![0.0; nx * nz];
    assert_eq!(h.sample_rect(rect, 1.0, &mut out), 0);
    assert_eq!(out[0], fine.get(500, 0));
    assert_eq!(out[2], fine.get(504, 0));
}

#[test]
fn raycast_hits_the_surface() {
    let src = awkward_world(Triangulation::Bilinear);
    let h = resident(&src, &[TileCoord::new(0, 0, 0), TileCoord::new(0, 1, 0)]);

    // Straight down.
    let p = WorldPos::from_f64(DVec3::new(meters(300) + 0.2, 500.0, meters(40) + 0.3));
    let hit = h.raycast(WorldRay { origin: p, dir: Vec3::NEG_Y }, 1000.0).unwrap();
    let ground = h.height(p).unwrap();
    assert!((hit.pos.to_f64().y - f64::from(ground)).abs() < 1e-3);
    assert!(hit.normal.y > 0.0);

    // A long, shallow ray across the tile border: the hit lies on the surface.
    let from = WorldPos::from_f64(DVec3::new(meters(-10), 60.0, meters(256)));
    let dir = Vec3::new(1.0, -0.12, 0.05).normalize();
    let hit = h.raycast(WorldRay { origin: from, dir }, 2000.0).unwrap();
    let y = h.height(hit.pos).unwrap();
    assert!((hit.pos.to_f64().y - f64::from(y)).abs() < 2e-3, "hit {:?} vs ground {y}", hit.pos);
    // Nothing nearer: sample the ray before the hit.
    let steps = (hit.distance / 0.1) as usize;
    for k in 0..steps.saturating_sub(1) {
        let q = from.offset(dir * (k as f32 * 0.1));
        if let Some(g) = h.height(q) {
            assert!(q.to_f64().y >= f64::from(g) - 1e-3, "ray went under the ground before the hit");
        }
    }

    // Pointing up from above: no hit.
    assert!(h.raycast(WorldRay { origin: p, dir: Vec3::Y }, 1000.0).is_none());
}

#[test]
fn edits_compose_on_top_of_the_base() {
    let src = awkward_world(Triangulation::Bilinear);
    let base = upsample(TileCoord::new(0, 0, 0), &src);
    let mut h = resident(&src, &[TileCoord::new(0, 0, 0)]);
    let at = |i: i64, j: i64| WorldPos::from_f64(DVec3::new(meters(i), 0.0, meters(j)));

    let mut brick = EditBrick::zero();
    brick.deltas[3 * BRICK_SAMPLES + 5] = 2.5; // global sample (64 + 5, 128 + 3)
    let coord = BrickCoord { x: 1, z: 2 };
    let rev0 = h.edits().revision();
    let changed = h.edits_mut().set_brick(coord, brick);
    assert_eq!(changed.revision, rev0 + 1);
    assert_eq!(h.height(at(69, 131)), Some(base.get(69, 131) + 2.5));
    assert_eq!(h.height(at(70, 131)), Some(base.get(70, 131)));
    assert!(h.edits().touches(TileCoord::new(0, 0, 0)));
    assert!(!h.edits().touches(TileCoord::new(0, 1, 0)));

    // The changed rect covers the brick and the quads around it.
    let min = changed.min.to_f64();
    assert!(min.x <= meters(64 - 1) + 1e-6 && min.x + f64::from(changed.size.x) >= meters(128) - 1e-6);

    // A raycast sees the edited surface (bounds culling is off for this tile).
    let p = at(69, 131).offset(Vec3::Y * 100.0);
    let hit = h.raycast(WorldRay { origin: p, dir: Vec3::NEG_Y }, 500.0).unwrap();
    assert!((hit.pos.to_f64().y - f64::from(base.get(69, 131) + 2.5)).abs() < 1e-3);

    h.edits_mut().remove_brick(coord).unwrap();
    assert_eq!(h.height(at(69, 131)), Some(base.get(69, 131)));
    assert!(h.edits().revision() > changed.revision);
}
