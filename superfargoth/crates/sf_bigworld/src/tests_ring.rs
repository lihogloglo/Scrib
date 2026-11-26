//! Ring planner tests (spec big-world-rendering.md "Data flow" step 1): tile distance, enter order, hysteresis, walks,
//! determinism, bounds, degenerate settings, ring radii; plus one measurement of ring counts per preset.

use std::collections::BTreeSet;
use std::time::Instant;

use crate::tunables::HYSTERESIS_TILES;
use crate::{
    BandSettings, Bands, Instance, InstanceSource, PackedInstance, PackedTile, Ring, RingDelta, RingSettings,
    SpeciesDims, SyntheticForest, TileKey, ViewScale, ring_radii,
};

// tunables: const (test fixture: the default tile edge)
const TILE: f32 = 256.0;
// tunables: const (test fixture: the High preset's impostor end of a 12 m / 8 m tree, rounded)
const HIGH_RADIUS: f32 = 2217.0;

fn settings(radius_m: f32, hysteresis_tiles: u32) -> RingSettings {
    RingSettings { tile_m: TILE, radius_m, hysteresis_tiles, bounds: None }
}

fn leave_m(s: &RingSettings) -> f64 {
    f64::from(s.radius_m) + f64::from(s.hysteresis_tiles) * f64::from(s.tile_m)
}

/// Every key in a square around the viewer, wider than any ring of `s`, with its distance.
fn brute_square(viewer: [f64; 2], s: &RingSettings) -> Vec<(f64, TileKey)> {
    let c = TileKey::containing(viewer[0], viewer[1], s.tile_m);
    #[allow(clippy::cast_possible_truncation, reason = "test radii are a few km, a few dozen tiles")]
    let half = (leave_m(s) / f64::from(s.tile_m)).ceil() as i32 + 3;
    let mut out = Vec::new();
    for z in c.z - half..=c.z + half {
        for x in c.x - half..=c.x + half {
            let key = TileKey::new(x, z);
            out.push((key.distance_to(viewer[0], viewer[1], s.tile_m), key));
        }
    }
    out
}

/// The tiles within the enter distance, by brute force.
fn brute_enter(viewer: [f64; 2], s: &RingSettings) -> BTreeSet<TileKey> {
    let r = f64::from(s.radius_m);
    brute_square(viewer, s).into_iter().filter(|(d, _)| *d <= r).map(|(_, k)| k).collect()
}

fn resident(ring: &Ring) -> BTreeSet<TileKey> {
    ring.resident().collect()
}

#[test]
#[allow(clippy::float_cmp, reason = "the def's literals are exact")]
fn hysteresis_tunable_golden() {
    assert_eq!(HYSTERESIS_TILES.name(), "bigworld.ring.hysteresis_tiles");
    assert_eq!(HYSTERESIS_TILES.default_value(), 1);
    assert_eq!(HYSTERESIS_TILES.bounds(), Some((0, 4)));
    assert!(crate::tunables::TUNABLES.iter().any(|d| d.name() == HYSTERESIS_TILES.name()), "listed in TUNABLES");
}

#[test]
#[allow(clippy::float_cmp, reason = "3-4-5 and axis distances are exact in f64")]
fn distance_to_square() {
    let t = 10.0;
    let k = TileKey::new(0, 0);
    assert_eq!(k.distance_to(5.0, 5.0, t), 0.0, "inside");
    assert_eq!(k.distance_to(10.0, 5.0, t), 0.0, "on the far edge");
    assert_eq!(k.distance_to(0.0, 0.0, t), 0.0, "on the min corner");
    assert_eq!(k.distance_to(13.0, 5.0, t), 3.0, "beside, +x");
    assert_eq!(k.distance_to(-3.0, 5.0, t), 3.0, "beside, -x");
    assert_eq!(k.distance_to(5.0, -7.0, t), 7.0, "beside, -z");
    assert_eq!(k.distance_to(13.0, 14.0, t), 5.0, "diagonal corner 3-4-5");
    let n = TileKey::new(-1, -2); // [-10, 0] x [-20, -10]
    assert_eq!(n.distance_to(-5.0, -15.0, t), 0.0, "inside a negative tile");
    assert_eq!(n.distance_to(3.0, -6.0, t), 5.0, "corner of a negative tile");
    assert_eq!(n.distance_to(-15.0, -15.0, t), 5.0, "beside a negative tile");
    assert!(k.distance_to(f64::NAN, 0.0, t).is_nan());
}

#[test]
fn first_update_matches_brute_force() {
    for (viewer, s) in [
        ([128.0, 128.0], settings(HIGH_RADIUS, 1)),
        ([-1000.5, 3333.3], RingSettings { tile_m: 100.0, radius_m: 700.0, hysteresis_tiles: 2, bounds: None }),
    ] {
        let mut ring = Ring::new();
        let delta = ring.update(viewer, &s);
        assert!(delta.leave.is_empty());
        let r = f64::from(s.radius_m);
        let mut want: Vec<(f64, TileKey)> = brute_square(viewer, &s).into_iter().filter(|(d, _)| *d <= r).collect();
        want.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
        let want: Vec<TileKey> = want.into_iter().map(|(_, k)| k).collect();
        assert_eq!(delta.enter, want, "enter is nearest first, then key");
        assert_eq!(resident(&ring), want.iter().copied().collect(), "resident = brute force");
        assert_eq!(ring.len(), want.len());
    }
    // Radius 2217 m, tile 256 m, viewer at the centre of tile (0, 0): 277 tiles.
    let mut ring = Ring::new();
    let delta = ring.update([128.0, 128.0], &settings(HIGH_RADIUS, 1));
    assert_eq!(delta.enter.len(), 277);
    assert_eq!(delta.enter[0], TileKey::new(0, 0), "the viewer's own tile first");
    // The four edge neighbours tie at 128 m, so they come in key order (x, then z).
    assert_eq!(&delta.enter[1..5], &[TileKey::new(-1, 0), TileKey::new(0, -1), TileKey::new(0, 1), TileKey::new(1, 0)]);
}

#[test]
fn hysteresis_stops_edge_churn() {
    // Oscillate ±1 m across the edge x = 256 between tiles 0 and 1.
    let walk = |h: u32| {
        let s = settings(1000.0, h);
        let mut ring = Ring::new();
        (0..100).map(|i| ring.update([if i % 2 == 0 { 255.0 } else { 257.0 }, 128.0], &s)).collect::<Vec<_>>()
    };
    let kept = walk(1);
    // The two sides are mirror images, so each side's first visit enters its own few tiles; after that nothing moves.
    assert!(!kept[0].enter.is_empty() && kept[0].leave.is_empty());
    assert!(!kept[1].enter.is_empty() && kept[1].leave.is_empty(), "the other side's first visit tops up");
    assert!(kept[2..].iter().all(RingDelta::is_empty), "no churn with hysteresis 1");
    let churn = walk(0);
    let busy = churn[1..].iter().filter(|d| !d.enter.is_empty() && !d.leave.is_empty()).count();
    assert_eq!(busy, 99, "without hysteresis every step reloads a tile");
}

#[test]
fn straight_walk_keeps_invariants() {
    let s = settings(1000.0, 1);
    let r = f64::from(s.radius_m);
    let leave = leave_m(&s);
    let mut ring = Ring::new();
    let mut left: BTreeSet<TileKey> = BTreeSet::new();
    let start = [-1700.0, -2300.0];
    for step in 0..=500 {
        // 5 km along (0.6, 0.8) in 10 m steps.
        let t = f64::from(step) * 10.0;
        let viewer = [start[0] + 0.6 * t, start[1] + 0.8 * t];
        let before = resident(&ring);
        let d = ring.update(viewer, &s);
        let after = resident(&ring);
        for key in brute_enter(viewer, &s) {
            assert!(after.contains(&key), "step {step}: {key:?} within the enter distance");
        }
        for key in &after {
            assert!(key.distance_to(viewer[0], viewer[1], s.tile_m) <= leave, "step {step}: {key:?} beyond leave");
        }
        let enter: BTreeSet<TileKey> = d.enter.iter().copied().collect();
        let gone: BTreeSet<TileKey> = d.leave.iter().copied().collect();
        assert_eq!(enter.len(), d.enter.len(), "no duplicate enters");
        assert!(enter.is_disjoint(&gone), "step {step}: enter and leave disjoint");
        assert!(enter.is_disjoint(&before) && gone.is_subset(&before) && gone.is_disjoint(&after));
        let mut expect = before.clone();
        expect.retain(|k| !gone.contains(k));
        expect.extend(enter.iter().copied());
        assert_eq!(after, expect, "step {step}: resident = before − leave + enter");
        let dist = |k: TileKey| k.distance_to(viewer[0], viewer[1], s.tile_m);
        assert!(d.enter.windows(2).all(|w| dist(w[0]).total_cmp(&dist(w[1])).then(w[0].cmp(&w[1])).is_lt()));
        assert!(d.leave.windows(2).all(|w| w[0] < w[1]), "leave in key order");
        assert!(d.enter.iter().all(|&k| dist(k) <= r));
        left.extend(gone);
    }
    assert!(!left.is_empty(), "the walk unloads tiles");
    // Back at the start (no clear): tiles that left come in again.
    let back = ring.update(start, &s);
    assert!(back.enter.iter().any(|k| left.contains(k)), "a tile that left can enter again");
}

#[test]
fn deterministic_and_path_independent() {
    let s0 = settings(900.0, 0);
    let end = [1234.5, -678.9];
    let mut a = Ring::new();
    for p in [[0.0, 0.0], [600.0, -300.0], end] {
        a.update(p, &s0);
    }
    let mut b = Ring::new();
    for p in [[-3000.0, 2000.0], [5000.0, 5000.0], [2000.0, -900.0], end] {
        b.update(p, &s0);
    }
    assert_eq!(resident(&a), resident(&b), "hysteresis 0: same end point, same set");
    assert_eq!(resident(&a), brute_enter(end, &s0), "and it is the enter set there");

    let s1 = settings(HIGH_RADIUS, 1);
    let run = || {
        let mut ring = Ring::new();
        (0..200).map(|i| ring.update([f64::from(i) * 37.0, f64::from(i) * -11.0], &s1)).collect::<Vec<_>>()
    };
    assert_eq!(run(), run(), "same calls, same deltas");
}

#[test]
fn bounds_clear_shrink_and_degenerate() {
    // Bounds clip: every tile of a 6 x 4 world is within 2 km of its middle.
    let bounds = [TileKey::new(-2, -1), TileKey::new(3, 2)];
    let s = RingSettings { bounds: Some(bounds), ..settings(2000.0, 1) };
    let mut ring = Ring::new();
    ring.update([0.0, 0.0], &s);
    let all: BTreeSet<TileKey> = (-1..=2).flat_map(|z| (-2..=3).map(move |x| TileKey::new(x, z))).collect();
    assert_eq!(resident(&ring), all);
    let mut far = Ring::new();
    assert!(far.update([1.0e5, 0.0], &s).is_empty(), "a viewer far outside the world loads nothing");
    let inverted = RingSettings { bounds: Some([bounds[1], bounds[0]]), ..s };
    assert!(Ring::new().update([0.0, 0.0], &inverted).is_empty(), "inverted bounds are empty");

    // Clear: everything leaves in key order.
    let cleared = ring.clear();
    assert_eq!(cleared, all.iter().copied().collect::<Vec<_>>());
    assert!(ring.is_empty() && !ring.contains(TileKey::new(0, 0)));
    assert_eq!(ring.len(), 0);

    // Shrink: tiles past the new leave distance go on the next update.
    let mut ring = Ring::new();
    ring.update([128.0, 128.0], &settings(1500.0, 1));
    let before = ring.len();
    let small = settings(500.0, 1);
    let d = ring.update([128.0, 128.0], &small);
    assert!(d.enter.is_empty() && !d.leave.is_empty());
    assert_eq!(ring.len(), before - d.leave.len());
    assert!(ring.resident().all(|k| k.distance_to(128.0, 128.0, TILE) <= leave_m(&small)));
    let d0 = ring.update([128.0, 128.0], &settings(500.0, 0));
    assert!(!d0.leave.is_empty(), "lowering the hysteresis also unloads");
    assert_eq!(resident(&ring), brute_enter([128.0, 128.0], &small));

    // Degenerate: nothing enters, nothing leaves.
    let kept = resident(&ring);
    let bad_settings = [
        RingSettings { tile_m: 0.0, ..small },
        RingSettings { tile_m: -256.0, ..small },
        RingSettings { tile_m: f32::NAN, ..small },
        RingSettings { tile_m: f32::INFINITY, ..small },
        RingSettings { radius_m: f32::NAN, ..small },
        RingSettings { radius_m: f32::INFINITY, ..small },
    ];
    for bad in bad_settings {
        assert!(ring.update([128.0, 128.0], &bad).is_empty(), "{bad:?}");
    }
    for viewer in [[f64::NAN, 0.0], [0.0, f64::INFINITY], [f64::NEG_INFINITY, 0.0]] {
        assert!(ring.update(viewer, &small).is_empty(), "{viewer:?}");
    }
    assert_eq!(resident(&ring), kept);
    // A negative radius admits nothing; the residents leave by the rule.
    let d = ring.update([128.0, 128.0], &settings(-10.0, 0));
    assert!(d.enter.is_empty() && ring.is_empty());
}

#[test]
#[allow(clippy::float_cmp, reason = "products of small integers by 2 are exact")]
fn ring_radii_cover_every_species() {
    let a = Bands { mesh_end_m: 100.0, impostor_end_m: 2000.0, fade_band: 0.1 };
    let b = Bands { mesh_end_m: 150.0, impostor_end_m: 1500.0, fade_band: 0.1 };
    assert_eq!(ring_radii(&[a, b], 2.0), (300.0, 4000.0));
    assert_eq!(ring_radii(&[], 2.0), (0.0, 0.0));
    assert_eq!(ring_radii(&[a], f32::NAN), (0.0, 0.0));
}

#[test]
#[ignore = "measurement: ring counts per preset and density"]
#[allow(clippy::cast_precision_loss, reason = "printed counts and timings")]
fn measure_ring_counts() {
    let view = ViewScale::new(960.0, 60.0_f32.to_radians());
    let tree = SpeciesDims { height_m: 12.0, crown_m: 8.0 };
    let presets = [("Ultra", 50.0, 2.0), ("High", 80.0, 3.0), ("Medium", 120.0, 4.0), ("Low", 200.0, 6.0)];
    let densities = [("avg", 60.0), ("stand", 400.0)];
    let viewer = [0.0, 0.0];
    for (preset, mesh_px, agg_px) in presets {
        let band_settings = BandSettings { mesh_switch_px: mesh_px, aggregate_switch_px: agg_px, fade_band: 0.1 };
        let bands = Bands::new(tree, view, band_settings);
        let (mesh_r, imp_r) = ring_radii(&[bands], 1.0);
        let mut imp_ring = Ring::new();
        imp_ring.update(viewer, &settings(imp_r, HYSTERESIS_TILES.default_value()));
        let mut mesh_ring = Ring::new();
        mesh_ring.update(viewer, &settings(mesh_r, HYSTERESIS_TILES.default_value()));
        for (label, density) in densities {
            let forest = SyntheticForest {
                seed: 7,
                density_per_ha: density,
                species_count: 1,
                scale: [1.0, 1.0],
                relief_m: 0.0,
                relief_wavelength_m: 1.0,
            };
            let mut buf: Vec<Instance> = Vec::new();
            let mut packed: Vec<PackedTile> = Vec::with_capacity(imp_ring.len());
            let t0 = Instant::now();
            for key in imp_ring.resident() {
                buf.clear();
                forest.tile(key, TILE, &mut buf);
                packed.push(PackedTile::pack(key, TILE, &buf));
            }
            let ms = t0.elapsed().as_secs_f64() * 1e3;
            let (mut total, mut in_mesh_ring, mut meshed, mut impostored) = (0_usize, 0_usize, 0_usize, 0_usize);
            for tile in &packed {
                let key = tile.header.key;
                let in_mesh = mesh_ring.contains(key);
                for i in 0..tile.instances.len() {
                    let inst = tile.unpack(i, TILE);
                    let [wx, _, wz] = tile.world_position(i, TILE);
                    #[allow(clippy::cast_possible_truncation, reason = "distances of a few km fit f32")]
                    let dist = (wx - viewer[0]).hypot(wz - viewer[1]) as f32;
                    let f = bands.fades(dist, inst.scale);
                    total += 1;
                    in_mesh_ring += usize::from(in_mesh);
                    meshed += usize::from(f.mesh > 0.0);
                    impostored += usize::from(f.impostor > 0.0);
                }
            }
            let mb = (total * PackedInstance::BYTES) as f64 / (1024.0 * 1024.0);
            println!(
                "bw1 ring | {preset} | {density}/ha ({label}) | mesh r {mesh_r:.0} m | impostor r {imp_r:.0} m | \
                 tiles impostor {} mesh {} | instances {total} ({mb:.1} MB) in mesh ring {in_mesh_ring} | \
                 mesh>0 {meshed} | impostor>0 {impostored} | gen+pack {ms:.1} ms ({:.3} ms/tile)",
                imp_ring.len(),
                mesh_ring.len(),
                ms / imp_ring.len().max(1) as f64,
            );
        }
    }
}
