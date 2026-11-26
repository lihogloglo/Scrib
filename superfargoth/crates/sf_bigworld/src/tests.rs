//! Exact tests of the tile instance format and the synthetic source.

// Exact float equality is what these tests assert (round-trips, exact corners).
#![allow(clippy::float_cmp)]
// Small cell indices, exact in an f32.
#![allow(clippy::cast_precision_loss)]

use std::f64::consts::{PI, TAU};

use crate::{Instance, InstanceSource, PackedInstance, PackedTile, SyntheticForest, TileKey};

const TILE_M: f32 = 256.0;
const UP: [f32; 3] = [0.0, 1.0, 0.0];

fn norm(v: [f32; 3]) -> [f32; 3] {
    let l = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    [v[0] / l, v[1] / l, v[2] / l]
}

fn inst(pos: [f32; 3], yaw: f32, scale: f32, up: [f32; 3], species: u16, seed: u16, flags: u16) -> Instance {
    Instance { pos, yaw, scale, up, species, seed, flags }
}

fn sample() -> Vec<Instance> {
    vec![
        inst([0.0, -12.5, 0.0], 0.0, 1.0, UP, 0, 0, 0),
        inst([256.0, 140.25, 256.0], 6.282, 0.25, norm([0.3, 0.9, -0.2]), 7, 0xbeef, 1),
        inst([128.0, 37.0, 64.3], 3.0, 4.0, [1.0, 0.0, 0.0], 65_535, 65_535, 65_535),
        inst([0.001, 0.0, 255.999], 1.0, 0.1, [0.0, -1.0, 0.0], 3, 1, 0x8000),
        inst([77.7, 300.0, 191.3], 5.5, 9.0, norm([-0.5, -0.4, 0.7]), 12, 4242, 2),
        inst([200.0, -50.0, 3.3], std::f32::consts::TAU - 1e-4, 1.7, norm([0.05, 0.99, 0.1]), 1, 2, 3),
        inst([13.37, 100.0, 250.0], 0.7, 0.6, norm([-0.7, 0.1, -0.7]), 9, 9, 9),
    ]
}

fn angle(a: [f32; 3], b: [f32; 3]) -> f64 {
    let [ax, ay, az] = a.map(f64::from);
    let [bx, by, bz] = b.map(f64::from);
    let cross = [ay * bz - az * by, az * bx - ax * bz, ax * by - ay * bx];
    let c = (cross[0] * cross[0] + cross[1] * cross[1] + cross[2] * cross[2]).sqrt();
    c.atan2(ax * bx + ay * by + az * bz)
}

fn unpack_all(t: &PackedTile, tile_m: f32) -> Vec<Instance> {
    (0..t.instances.len()).map(|i| t.unpack(i, tile_m)).collect()
}

#[test]
fn packed_instance_layout() {
    assert_eq!(size_of::<PackedInstance>(), 16);
    assert_eq!(PackedInstance::BYTES, 16);
    let p = PackedInstance([0x0403_0201, 0x0807_0605, 0x0c0b_0a09, 0x100f_0e0d]);
    assert_eq!(p.to_le_bytes(), [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);

    let tile = PackedTile::pack(TileKey::new(0, 0), TILE_M, &sample());
    let mut bytes = Vec::new();
    tile.write_le_bytes(&mut bytes);
    assert_eq!(bytes.len(), 16 * tile.header.count as usize);
    assert_eq!(tile.header.count as usize, sample().len());
    assert_eq!(&bytes[16..32], &tile.instances[1].to_le_bytes());
}

#[test]
fn packed_word_layout() {
    // x = tile_m → 65535, z = 0, y = y_max → 65535, yaw π → 128, scale 4 → 255, up +y → (127, 127).
    let i = inst([256.0, 10.0, 0.0], std::f32::consts::PI, 4.0, UP, 0x1234, 0x5678, 0x9abc);
    let lo = inst([0.0, 0.0, 0.0], 0.0, 0.25, UP, 0, 0, 0);
    let t = PackedTile::pack(TileKey::new(0, 0), TILE_M, &[i, lo]);
    assert_eq!(t.instances[0].0, [0x0000_ffff, 0xff80_ffff, 0x5678_1234, 0x7f7f_9abc]);
    assert_eq!(t.instances[1].0, [0, 0, 0, 0x7f7f_0000]);
}

#[test]
fn pack_round_trip_within_half_a_step() {
    let input = sample();
    let tile = PackedTile::pack(TileKey::new(3, -4), TILE_M, &input);
    assert_eq!(tile.header.y_min, -50.0);
    assert_eq!(tile.header.y_max, 300.0);
    let range = f64::from(tile.header.y_max - tile.header.y_min);
    let tile_m = f64::from(TILE_M);
    // Half a step of each field, plus the f32 spacing of the decoded value (≤ 3.1e-5 below 512).
    let f32_slack = 3.1e-5;
    let xz_bound = tile_m / 65_535.0 / 2.0 + f32_slack;
    let y_bound = range / 65_535.0 / 2.0 + f32_slack;
    let yaw_bound = PI / 256.0 + 1e-6;
    let log_scale_bound = 2.0 / 255.0 + 1e-6;
    // Octahedral, 8 bits per coordinate: |Δp| ≤ 1/254 per axis moves the L1-sphere point by ≤ √6/254 and its
    // distance to the origin is ≥ 1/√3, so the angle is ≤ √18/254 = 3√2/254 rad (≈ 0.96°).
    let up_bound = 3.0 * 2.0_f64.sqrt() / 254.0;
    for (i, (a, b)) in input.iter().zip(unpack_all(&tile, TILE_M)).enumerate() {
        let dx = (f64::from(a.pos[0]) - f64::from(b.pos[0])).abs();
        let dz = (f64::from(a.pos[2]) - f64::from(b.pos[2])).abs();
        let dy = (f64::from(a.pos[1]) - f64::from(b.pos[1])).abs();
        assert!(dx <= xz_bound && dz <= xz_bound, "instance {i}: x/z error {dx}, {dz} > {xz_bound}");
        assert!(dy <= y_bound, "instance {i}: y error {dy} > {y_bound}");
        let dyaw = (f64::from(a.yaw) - f64::from(b.yaw)).rem_euclid(TAU);
        let dyaw = dyaw.min(TAU - dyaw);
        assert!(dyaw <= yaw_bound, "instance {i}: yaw error {dyaw} > {yaw_bound}");
        assert!((0.0..TAU).contains(&f64::from(b.yaw)), "instance {i}: yaw {} out of [0, 2π)", b.yaw);
        let clamped = f64::from(a.scale).clamp(0.25, 4.0);
        let dscale = (f64::from(b.scale) / clamped).log2().abs();
        assert!(dscale <= log_scale_bound, "instance {i}: log2 scale error {dscale} > {log_scale_bound}");
        let dup = angle(a.up, b.up);
        assert!(dup <= up_bound, "instance {i}: up error {dup} rad > {up_bound}");
        let len = b.up.iter().map(|c| f64::from(*c).powi(2)).sum::<f64>().sqrt();
        assert!((len - 1.0).abs() <= 1e-6, "instance {i}: decoded up not unit ({len})");
        assert_eq!((a.species, a.seed, a.flags), (b.species, b.seed, b.flags), "instance {i}");
    }
    // Clamped scales decode to the ends of the range.
    assert_eq!(tile.unpack(3, TILE_M).scale, 0.25);
    assert_eq!(tile.unpack(4, TILE_M).scale, 4.0);

    let repacked = PackedTile::pack(tile.header.key, TILE_M, &unpack_all(&tile, TILE_M));
    assert_eq!(repacked, tile, "pack(unpack(pack(x))) must equal pack(x) bit for bit");
}

#[test]
fn upright_and_flat_tile_are_exact() {
    let input = vec![
        inst([0.0, 5.0, 256.0], 0.0, 1.0, UP, 1, 2, 3),
        inst([256.0, 5.0, 0.0], 2.0, 2.0, UP, 4, 5, 6),
        inst([100.0, 5.0, 100.0], 4.0, 0.5, UP, 7, 8, 9),
    ];
    let tile = PackedTile::pack(TileKey::new(-1, 2), TILE_M, &input);
    assert_eq!((tile.header.y_min, tile.header.y_max), (5.0, 5.0));
    for (i, (a, b)) in input.iter().zip(unpack_all(&tile, TILE_M)).enumerate() {
        assert_eq!(b.up, [0.0, 1.0, 0.0], "instance {i}");
        assert_eq!(b.pos[1], 5.0, "instance {i}");
        assert_eq!((b.species, b.seed, b.flags), (a.species, a.seed, a.flags), "instance {i}");
    }
    // Tile corners are exact too.
    assert_eq!(tile.unpack(0, TILE_M).pos, [0.0, 5.0, 256.0]);
    assert_eq!(tile.unpack(1, TILE_M).pos, [256.0, 5.0, 0.0]);
    assert_eq!(tile.unpack(0, TILE_M).yaw, 0.0);

    let empty = PackedTile::pack(TileKey::new(9, 9), TILE_M, &[]);
    assert_eq!(empty.header.count, 0);
    assert_eq!((empty.header.y_min, empty.header.y_max), (0.0, 0.0));
    assert!(empty.instances.is_empty());
    let mut bytes = vec![0xaa];
    empty.write_le_bytes(&mut bytes);
    assert_eq!(bytes, [0xaa]);
}

#[test]
fn world_position_is_corner_plus_local() {
    let input = [inst([0.0, -3.0, 256.0], 0.0, 1.0, UP, 0, 0, 0), inst([256.0, 7.0, 0.0], 0.0, 1.0, UP, 0, 0, 0)];
    let neg = PackedTile::pack(TileKey::new(-3, -2), TILE_M, &input);
    assert_eq!(neg.world_position(0, TILE_M), [-768.0, -3.0, -256.0]);
    assert_eq!(neg.world_position(1, TILE_M), [-512.0, 7.0, -512.0]);
    let far = PackedTile::pack(TileKey::new(100_000, -100_000), TILE_M, &input);
    assert_eq!(far.world_position(0, TILE_M), [25_600_000.0, -3.0, -25_599_744.0]);
    assert_eq!(far.world_position(1, TILE_M), [25_600_256.0, 7.0, -25_600_000.0]);
    // Any instance: min corner + decoded local, in f64.
    let tile = PackedTile::pack(TileKey::new(100_000, -7), TILE_M, &sample());
    let [cx, cz] = tile.header.key.min_corner(TILE_M);
    for i in 0..tile.instances.len() {
        let p = tile.unpack(i, TILE_M).pos;
        let expected = [cx + f64::from(p[0]), f64::from(p[1]), cz + f64::from(p[2])];
        assert_eq!(tile.world_position(i, TILE_M), expected, "instance {i}");
    }
}

#[test]
fn tile_key_containing_floors() {
    assert_eq!(TileKey::containing(0.0, 0.0, TILE_M), TileKey::new(0, 0));
    assert_eq!(TileKey::containing(255.999, 0.001, TILE_M), TileKey::new(0, 0));
    assert_eq!(TileKey::containing(256.0, 512.0, TILE_M), TileKey::new(1, 2));
    assert_eq!(TileKey::containing(-0.001, -256.0, TILE_M), TileKey::new(-1, -1));
    assert_eq!(TileKey::containing(-256.001, -512.0, TILE_M), TileKey::new(-2, -2));
    assert_eq!(TileKey::containing(-1000.0, 1000.0, TILE_M), TileKey::new(-4, 3));
    assert_eq!(TileKey::new(-4, 3).min_corner(TILE_M), [-1024.0, 768.0]);
    let k = TileKey::new(-123, 77);
    let [x, z] = k.min_corner(TILE_M);
    assert_eq!(TileKey::containing(x, z, TILE_M), k);
}

fn forest(density_per_ha: f32, relief_m: f32) -> SyntheticForest {
    SyntheticForest {
        seed: 0x5eed,
        density_per_ha,
        species_count: 5,
        scale: [0.8, 1.6],
        relief_m,
        relief_wavelength_m: 700.0,
    }
}

fn fill(src: &SyntheticForest, key: TileKey) -> Vec<Instance> {
    let mut out = Vec::new();
    src.tile(key, TILE_M, &mut out);
    out
}

#[test]
fn synthetic_forest_counts_and_bounds() {
    let src = forest(60.0, 40.0);
    let key = TileKey::new(-2, 5);
    let a = fill(&src, key);
    assert_eq!(a.len(), 400);
    assert_eq!(fill(&forest(400.0, 40.0), key).len(), 2601);
    for (i, t) in a.iter().enumerate() {
        assert!((0.0..TILE_M).contains(&t.pos[0]) && (0.0..TILE_M).contains(&t.pos[2]), "instance {i}: {:?}", t.pos);
        assert!(t.pos[1].abs() <= 40.0, "instance {i}");
        assert!((0.0..std::f32::consts::TAU).contains(&t.yaw), "instance {i}");
        assert!((0.8..=1.6).contains(&t.scale), "instance {i}");
        assert!(t.species < 5, "instance {i}");
        assert_eq!((t.up, t.flags), (UP, 0), "instance {i}");
    }
    // Row-major: cell (ix, iz) = (i % n, i / n) holds instance i.
    let cell = 256.0 / 20.0;
    for (i, t) in a.iter().enumerate() {
        let (ix, iz) = ((i % 20) as f32, (i / 20) as f32);
        assert!(t.pos[0] >= ix * cell - 1e-3 && t.pos[0] <= (ix + 1.0) * cell + 1e-3, "instance {i}");
        assert!(t.pos[2] >= iz * cell - 1e-3 && t.pos[2] <= (iz + 1.0) * cell + 1e-3, "instance {i}");
    }
    // Appends.
    let mut out = a.clone();
    src.tile(key, TILE_M, &mut out);
    assert_eq!(out.len(), 800);
    assert_eq!(out[400..], a[..]);
}

#[test]
fn synthetic_forest_is_deterministic_and_order_free() {
    let src = forest(120.0, 25.0);
    let keys = [TileKey::new(0, 0), TileKey::new(1, 0), TileKey::new(-1, -1), TileKey::new(100_000, -3)];
    let forward: Vec<_> = keys.iter().map(|k| fill(&src, *k)).collect();
    let backward: Vec<_> = keys.iter().rev().map(|k| fill(&src, *k)).collect();
    for (i, k) in keys.iter().enumerate() {
        assert_eq!(fill(&src, *k), forward[i], "same key twice, tile {k:?}");
        assert_eq!(forward[i], backward[keys.len() - 1 - i], "call order, tile {k:?}");
    }
    assert_ne!(forward[0], forward[1], "neighbouring tiles must differ");
    // Packing is deterministic too.
    assert_eq!(PackedTile::pack(keys[0], TILE_M, &forward[0]), PackedTile::pack(keys[0], TILE_M, &forward[0]));
}

#[test]
fn synthetic_forest_degenerate_inputs() {
    let key = TileKey::new(4, -9);
    assert!(fill(&forest(0.0, 30.0), key).is_empty());
    assert!(fill(&forest(-5.0, 30.0), key).is_empty());
    let flat = fill(&forest(80.0, 0.0), key);
    assert!(!flat.is_empty());
    assert!(flat.iter().all(|t| t.pos[1] == 0.0));
    let hilly = fill(&forest(80.0, 30.0), key);
    assert!(hilly.iter().any(|t| t.pos[1] != 0.0));
}
