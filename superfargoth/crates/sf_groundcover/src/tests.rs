//! Scatter, data and determinism tests (exact assertions; the one measurement is `#[ignore]`).
#![allow(clippy::float_cmp, reason = "logic tests assert exact values (the scatter is deterministic IEEE arithmetic)")]

use std::collections::BTreeMap;
use std::time::Instant;

use glam::{DVec3, Vec3};

use crate::*;

/// A ground from a closure.
pub(crate) struct Ground<F: Fn(f64, f64, &mut GroundSample)>(pub F);

impl<F: Fn(f64, f64, &mut GroundSample)> GroundSampler for Ground<F> {
    fn sample(&self, x: f64, z: f64, out: &mut GroundSample) {
        (self.0)(x, z, out);
    }
}

/// Flat dry ground at height 0 with fixed channel weights.
pub(crate) fn flat(channels: &[f32]) -> impl GroundSampler + '_ {
    Ground(move |_, _, out: &mut GroundSample| {
        out.height = 0.0;
        out.normal = Vec3::Y;
        out.water_distance = f32::MAX;
        out.valid = true;
        out.channels.clear();
        out.channels.extend_from_slice(channels);
    })
}

/// Rolling ground: height, normal and two channels all vary with position (polynomial, no transcendentals).
#[allow(clippy::cast_possible_truncation)]
pub(crate) fn rolling() -> impl GroundSampler {
    Ground(|x: f64, z: f64, out: &mut GroundSample| {
        let (fx, fz) = ((x * 0.013).rem_euclid(1.0) as f32, (z * 0.017).rem_euclid(1.0) as f32);
        out.height = 40.0 * fx * fz;
        out.normal = Vec3::new(fx - 0.5, 1.5, fz - 0.5).normalize();
        out.water_distance = 10.0 * fx - 1.0;
        out.valid = true;
        out.channels.clear();
        out.channels.extend_from_slice(&[fx, 1.0 - fx]);
    })
}

/// An unconstrained species (any slope, height, water).
pub(crate) fn species(id: &str, footprint_m: f32, density: f32) -> Species {
    Species {
        id: id.into(),
        render: RenderKind::Mesh { model: format!("{id}.mesh"), lods: vec![] },
        footprint_m,
        density,
        slope_deg: None,
        altitude_m: None,
        water_m: None,
        scale: Span::new(0.8, 1.25),
        yaw_deg: Span::new(0.0, 360.0),
        tint: [[1.0; 3], [0.8, 0.9, 0.7]],
        align_to_surface: Span::new(0.0, 1.0),
        wind: 1.0,
        trample: 0.5,
        shadow: ShadowClass::None,
        range: RangeClass::Near,
    }
}

/// Every species on channel 0 with weight 1.
pub(crate) fn on_channel0(species: Vec<Species>) -> Groundcover {
    let rules = RuleTable {
        rules: vec![ChannelRule {
            channel: 0,
            species: species.iter().map(|s| WeightedSpecies { species: s.id.clone(), weight: 1.0 }).collect(),
        }],
    };
    Groundcover::new(species, &rules).unwrap()
}

pub(crate) fn scatter_count(sc: &Scatterer<'_>, g: &impl GroundSampler, tiles: &[TileKey], s: SpeciesIndex) -> (u64, u64) {
    let mut out = Vec::new();
    let (mut accepted, mut candidates) = (0, 0);
    for &t in tiles {
        let st = sc.scatter_tile(g, t, s, 0, &mut out);
        assert!(!st.capped);
        accepted += u64::from(st.accepted);
        candidates += u64::from(st.candidates);
    }
    (accepted, candidates)
}

pub(crate) fn tiles(n: i64) -> Vec<TileKey> {
    (0..n).flat_map(|z| (0..n).map(move |x| TileKey::new(x - n / 2, z - n / 2))).collect()
}

fn three_species() -> Groundcover {
    let mut b = species("b", 0.7, 0.8);
    b.range = RangeClass::Mid;
    let rules = RuleTable {
        rules: vec![
            ChannelRule {
                channel: 0,
                species: vec![
                    WeightedSpecies { species: "a".into(), weight: 1.0 },
                    WeightedSpecies { species: "c".into(), weight: 0.5 },
                ],
            },
            ChannelRule { channel: 1, species: vec![WeightedSpecies { species: "b".into(), weight: 1.0 }] },
        ],
    };
    Groundcover::new(vec![species("a", 0.35, 6.0), b, species("c", 1.3, 0.4)], &rules).unwrap()
}

fn scatter_bytes(sc: &Scatterer<'_>, g: &impl GroundSampler, order: &[(TileKey, SpeciesIndex)]) -> BTreeMap<(TileKey, u16), Vec<u8>> {
    let mut out = Vec::new();
    let mut map = BTreeMap::new();
    for &(t, s) in order {
        sc.scatter_tile(g, t, s, 0, &mut out);
        map.insert((t, s.0), out.iter().flat_map(Instance::to_le_bytes).collect());
    }
    map
}

#[test]
fn same_inputs_give_byte_identical_instances_in_any_tile_order() {
    let g = rolling();
    let (c1, c2) = (three_species(), three_species());
    let none = Overrides::none();
    let s1 = Scatterer::new(&c1, &none, ScatterSettings::default());
    let s2 = Scatterer::new(&c2, &none, ScatterSettings::default());
    let mut order: Vec<_> = tiles(4).into_iter().flat_map(|t| c1.indices().map(move |s| (t, s))).collect();
    let a = scatter_bytes(&s1, &g, &order);
    order.reverse();
    let b = scatter_bytes(&s2, &g, &order);
    // Species-major instead of tile-major, and a scrambled stride.
    let mut scrambled = order.clone();
    scrambled.sort_by_key(|&(t, s)| (s, (t.x * 7 + t.z * 3).rem_euclid(5), t));
    let c = scatter_bytes(&s1, &g, &scrambled);
    assert_eq!(a.len(), 16 * 3);
    assert!(a.values().map(Vec::len).sum::<usize>() > 0);
    assert_eq!(a, b);
    assert_eq!(a, c);
}

#[test]
fn different_tiles_and_species_differ() {
    let g = flat(&[1.0, 1.0]);
    let cover = three_species();
    let none = Overrides::none();
    let sc = Scatterer::new(&cover, &none, ScatterSettings::default());
    let m = scatter_bytes(&sc, &g, &[(TileKey::new(0, 0), SpeciesIndex(0)), (TileKey::new(1, 0), SpeciesIndex(0)), (TileKey::new(0, 1), SpeciesIndex(0))]);
    let v: Vec<_> = m.values().collect();
    assert_ne!(v[0], v[1]);
    assert_ne!(v[0], v[2]);
    assert_ne!(v[1], v[2]);
}

#[test]
fn far_tile_from_two_floating_origins_gives_the_same_world_positions() {
    let g = flat(&[1.0]);
    let cover = on_channel0(vec![species("a", 0.5, 2.0)]);
    let none = Overrides::none();
    let settings = ScatterSettings::default();
    let sc = Scatterer::new(&cover, &none, settings);
    let size = settings.tile_size_m;
    // ~16 000 km from the world origin: a world-space f32 would round to 1-2 m here.
    let tile = TileKey::new(1_000_000, -1_000_003);
    let mut out = Vec::new();
    sc.scatter_tile(&g, tile, SpeciesIndex(0), 0, &mut out);
    assert!(out.len() > 100);
    let corner = tile.min_corner(size);
    let o1 = DVec3::new(corner[0] + 3.25, 0.0, corner[1] - 5.5);
    let o2 = DVec3::new(corner[0] - 200.0, 12.0, corner[1] + 90.0);
    for i in &out {
        assert!((0.0..=size).contains(&i.local[0]) && (0.0..=size).contains(&i.local[2]));
        let w = i.world_position(tile, size);
        let w1 = i.translation_from(tile, size, o1).as_dvec3() + o1;
        let w2 = i.translation_from(tile, size, o2).as_dvec3() + o2;
        // f32 near the origin (< 300 m) is good to ~30 µm; 1 mm is the bound.
        assert!((w1 - w).abs().max_element() < 1e-3, "{w1} vs {w}");
        assert!((w2 - w).abs().max_element() < 1e-3, "{w2} vs {w}");
        assert!(TileKey::containing(w.x, w.z, size) == tile || i.local[0] == size || i.local[2] == size);
    }
}

#[test]
fn constraints_outside_range_give_zero_instances() {
    let mut out = Vec::new();
    let none = Overrides::none();
    let tile = TileKey::new(2, 5);
    // 45° ground, wet (submerged by 1 m), at 100 m.
    let ground = Ground(|_, _, o: &mut GroundSample| {
        o.height = 100.0;
        o.normal = Vec3::new(1.0, 1.0, 0.0).normalize();
        o.water_distance = -1.0;
        o.valid = true;
        o.channels.clear();
        o.channels.push(1.0);
    });
    let run = |s: Species, out: &mut Vec<Instance>| {
        let cover = on_channel0(vec![s]);
        Scatterer::new(&cover, &none, ScatterSettings::default()).scatter_tile(&ground, tile, SpeciesIndex(0), 0, out)
    };
    // Control: no constraints → instances.
    assert!(run(species("a", 0.5, 2.0), &mut out).accepted > 0);
    let mut s = species("a", 0.5, 2.0);
    s.slope_deg = Some(Span::new(0.0, 30.0));
    assert_eq!(run(s, &mut out).accepted, 0, "slope");
    let mut s = species("a", 0.5, 2.0);
    s.slope_deg = Some(Span::new(40.0, 50.0));
    assert!(run(s, &mut out).accepted > 0, "slope in range");
    let mut s = species("a", 0.5, 2.0);
    s.altitude_m = Some(Span::new(0.0, 50.0));
    assert_eq!(run(s, &mut out).accepted, 0, "altitude");
    let mut s = species("a", 0.5, 2.0);
    s.water_m = Some(Span::new(0.2, 1.0e9));
    assert_eq!(run(s, &mut out).accepted, 0, "water");
    let mut s = species("a", 0.5, 2.0);
    s.water_m = Some(Span::new(-2.0, 0.0));
    assert!(run(s, &mut out).accepted > 0, "submerged species in range");
    let st = run(species("a", 0.5, 0.0), &mut out);
    assert_eq!((st.accepted, out.len()), (0, 0), "density 0");
    // No ground.
    let hole = Ground(|_, _, o: &mut GroundSample| {
        *o = GroundSample { valid: false, channels: vec![1.0], ..GroundSample::blank() };
    });
    let cover = on_channel0(vec![species("a", 0.5, 2.0)]);
    let st = Scatterer::new(&cover, &none, ScatterSettings::default()).scatter_tile(&hole, tile, SpeciesIndex(0), 0, &mut out);
    assert_eq!(st.accepted, 0, "hole");
}

/// Binomial bound: with `N` candidates of acceptance `p`, the count must lie within 5σ = 5·√(N·p·(1-p)) of `N·p`
/// (a 1-in-1.7-million miss for a fair hash; the scatter is deterministic, so this pins the hash's fairness once).
#[test]
fn instance_count_tracks_density_within_a_binomial_bound() {
    let g = flat(&[1.0]);
    let ts = tiles(8); // 64 tiles × 32² cells = 65 536 candidates
    let none = Overrides::none();
    let check = |density: f32, scale: f32| {
        let cover = on_channel0(vec![species("a", 0.5, density)]);
        let sc = Scatterer::new(&cover, &none, ScatterSettings { density_scale: scale, ..ScatterSettings::default() });
        assert_eq!(sc.grid(SpeciesIndex(0)), Grid { n: 32, cell_m: 0.5 });
        let (accepted, candidates) = scatter_count(&sc, &g, &ts, SpeciesIndex(0));
        assert_eq!(candidates, 65_536);
        let p = f64::from(density * scale) * 0.25;
        #[allow(clippy::cast_precision_loss)]
        let (n, k) = (candidates as f64, accepted as f64);
        let bound = 5.0 * (n * p * (1.0 - p)).sqrt();
        assert!((k - n * p).abs() <= bound, "density {density} × {scale}: {k} vs {} ± {bound}", n * p);
        accepted
    };
    let full = check(1.2, 1.0); // p = 0.30: 19 661 ± 587
    let half = check(1.2, 0.5); // p = 0.15:  9 830 ± 457
    let tenth = check(0.4, 1.0); // p = 0.10:  6 554 ± 384
    assert!(half < full && tenth < half);
    // Saturation: density ≥ 1 / footprint² fills every cell.
    let cover = on_channel0(vec![species("a", 0.5, 10.0)]);
    let sc = Scatterer::new(&cover, &none, ScatterSettings::default());
    assert_eq!(scatter_count(&sc, &g, &ts, SpeciesIndex(0)), (65_536, 65_536));
}

#[test]
fn density_twin_reproduces_every_acceptance() {
    let g = rolling();
    let cover = three_species();
    let none = Overrides::none();
    let sc = Scatterer::new(&cover, &none, ScatterSettings::default());
    let mut out = Vec::new();
    for t in tiles(2) {
        for s in cover.indices() {
            sc.scatter_tile(&g, t, s, 0, &mut out);
            let Grid { n, cell_m } = sc.grid(s);
            let corner = t.min_corner(sc.settings.tile_size_m);
            let id = species_hash(&cover.species(s).id);
            let mut expected = Vec::new();
            for cell in 0..n * n {
                let mut r = Stream::new(candidate_seed(t, id, cell));
                #[allow(clippy::cast_precision_loss)]
                let (lx, lz) = (((cell % n) as f32 + r.next_unit()) * cell_m, ((cell / n) as f32 + r.next_unit()) * cell_m);
                let h = r.next_u32();
                let d = sc.density_at(&g, corner[0] + f64::from(lx), corner[1] + f64::from(lz), s);
                if accepts(h, d * cell_m * cell_m) {
                    expected.push((lx, lz));
                }
            }
            let got: Vec<_> = out.iter().map(|i| (i.local[0], i.local[2])).collect();
            assert_eq!(got, expected);
        }
    }
}

#[test]
fn rule_weight_sums_channels_and_ignores_missing_ones() {
    let rules = RuleTable {
        rules: vec![
            ChannelRule { channel: 0, species: vec![WeightedSpecies { species: "a".into(), weight: 0.5 }] },
            ChannelRule { channel: 2, species: vec![WeightedSpecies { species: "a".into(), weight: 2.0 }] },
            ChannelRule { channel: 9, species: vec![WeightedSpecies { species: "a".into(), weight: 4.0 }] },
        ],
    };
    let cover = Groundcover::new(vec![species("a", 1.0, 1.0)], &rules).unwrap();
    assert_eq!(cover.rule_weight(SpeciesIndex(0), &[1.0, 7.0, 0.25]), 0.5 + 0.5);
    let none = Overrides::none();
    let sc = Scatterer::new(&cover, &none, ScatterSettings { density_scale: 2.0, ..ScatterSettings::default() });
    assert_eq!(sc.density_at(&flat(&[1.0, 7.0, 0.25]), 0.0, 0.0, SpeciesIndex(0)), 2.0);
}

#[test]
fn build_rejects_bad_tables() {
    let r = |id: &str| RuleTable { rules: vec![ChannelRule { channel: 0, species: vec![WeightedSpecies { species: id.into(), weight: 1.0 }] }] };
    assert_eq!(Groundcover::new(vec![species("a", 1.0, 1.0)], &r("zz")).unwrap_err(), BuildError::UnknownSpecies("zz".into()));
    assert_eq!(
        Groundcover::new(vec![species("a", 1.0, 1.0), species("a", 2.0, 1.0)], &r("a")).unwrap_err(),
        BuildError::DuplicateSpecies("a".into())
    );
    assert!(matches!(Groundcover::new(vec![species("a", 0.0, 1.0)], &r("a")), Err(BuildError::InvalidSpecies { .. })));
    let mut s = species("a", 1.0, 1.0);
    s.slope_deg = Some(Span::new(40.0, 10.0));
    assert!(matches!(Groundcover::new(vec![s], &r("a")), Err(BuildError::InvalidSpecies { .. })));
}

#[test]
fn tag_packing_round_trips() {
    for tag in [
        Tag::default(),
        Tag { wind_phase: 1, tint: 2, fade_seed: 3, batch_slot: 4 },
        Tag { wind_phase: 255, tint: 0, fade_seed: 255, batch_slot: 0 },
        Tag { wind_phase: 0xa5, tint: 0x5a, fade_seed: 0xff, batch_slot: 0x80 },
    ] {
        assert_eq!(Tag::unpack(tag.pack()), tag);
    }
    assert_eq!(Tag { wind_phase: 0x11, tint: 0x22, fade_seed: 0x33, batch_slot: 0x44 }.pack(), 0x4433_2211);
    assert_eq!(Tag::unpack(0xdead_beef).pack(), 0xdead_beef);
    // The scatter writes the caller's batch slot.
    let cover = on_channel0(vec![species("a", 0.5, 2.0)]);
    let none = Overrides::none();
    let mut out = Vec::new();
    Scatterer::new(&cover, &none, ScatterSettings::default()).scatter_tile(&flat(&[1.0]), TileKey::new(0, 0), SpeciesIndex(0), 0x7e, &mut out);
    assert!(!out.is_empty() && out.iter().all(|i| i.tag().batch_slot == 0x7e));
}

#[test]
fn instance_cap_stops_the_tile() {
    let cover = on_channel0(vec![species("a", 0.5, 100.0)]);
    let none = Overrides::none();
    let sc = Scatterer::new(&cover, &none, ScatterSettings { max_instances_per_tile: 100, ..ScatterSettings::default() });
    let mut out = Vec::new();
    let st = sc.scatter_tile(&flat(&[1.0]), TileKey::new(0, 0), SpeciesIndex(0), 0, &mut out);
    assert_eq!((st.accepted, st.capped, out.len()), (100, true, 100));
}

#[test]
fn species_rules_and_overrides_round_trip_through_ron() {
    let mut blade = species("meadow", 0.3, 5.0);
    blade.render = RenderKind::Blade { style: "meadow.grass".into() };
    blade.slope_deg = Some(Span::new(0.0, 35.0));
    blade.water_m = Some(Span::new(-0.5, 2.0));
    blade.shadow = ShadowClass::Near;
    let mut fern = species("fern", 1.25, 0.5);
    fern.render = RenderKind::Mesh { model: "fern.glb".into(), lods: vec!["fern_lod1.glb".into()] };
    fern.range = RangeClass::Mid;
    let list = vec![blade, fern];
    let rules = RuleTable {
        rules: vec![ChannelRule {
            channel: 3,
            species: vec![WeightedSpecies { species: "meadow".into(), weight: 1.0 }, WeightedSpecies { species: "fern".into(), weight: 0.25 }],
        }],
    };
    let overrides = OverrideSet {
        paints: vec![DensityPaint { origin: [0.0, 16.0], size_m: 16.0, resolution: 2, texels: vec![0, 64, 128, 255], species: vec!["fern".into()] }],
        exclusions: vec![Region::circle([4.0, 4.0], 2.5)],
        only: vec![OnlyRegion { region: Region::rect([0.0, 0.0], [8.0, 8.0]), priority: 3, species: vec![WeightedSpecies { species: "fern".into(), weight: 1.0 }] }],
        explicit: vec![ExplicitRegion {
            region: Region { shape: Shape::Box { min: [1.0, 1.0], max: [2.0, 2.0] }, mask: Some(Mask { origin: [1.0, 1.0], texel_m: 0.5, width: 2, height: 2, texels: vec![255, 0, 0, 255] }) },
            range: RangeClass::Near,
        }],
    };
    let text = ron::ser::to_string_pretty(&(&list, &rules, &overrides), ron::ser::PrettyConfig::default()).unwrap();
    let back: (Vec<Species>, RuleTable, OverrideSet) = ron::from_str(&text).unwrap();
    assert_eq!(back, (list.clone(), rules.clone(), overrides.clone()));
    // Optional fields may be left out of hand-written data.
    let minimal = r#"(id: "x", render: Blade(style: "s"), footprint_m: 1.0, density: 1.0, scale: (min: 1.0, max: 1.0),
        yaw_deg: (min: 0.0, max: 360.0), tint: ((1.0, 1.0, 1.0), (1.0, 1.0, 1.0)), align_to_surface: (min: 0.0, max: 0.0),
        wind: 1.0, trample: 1.0)"#;
    let s: Species = ron::from_str(minimal).unwrap();
    assert_eq!((s.slope_deg, s.shadow, s.range), (None, ShadowClass::None, RangeClass::Near));
    // And the round-tripped data builds.
    let cover = Groundcover::new(back.0, &back.1).unwrap();
    Overrides::new(&cover, &back.2).unwrap();
}

#[test]
fn hash_is_pinned() {
    // Changing the hash moves every instance in every world: these values are a format.
    assert_eq!(species_hash(""), 0xcbf2_9ce4_8422_2325);
    assert_eq!(species_hash("a"), 0xaf63_dc4c_8601_ec8c);
    assert_eq!(mix64(0), 0);
    let first = Stream::new(candidate_seed(TileKey::new(0, 0), species_hash("a"), 0)).next_u32();
    let again = Stream::new(candidate_seed(TileKey::new(0, 0), species_hash("a"), 0)).next_u32();
    assert_eq!(first, again);
    assert_ne!(first, Stream::new(candidate_seed(TileKey::new(-1, 0), species_hash("a"), 0)).next_u32());
}

#[test]
#[ignore = "measurement: instances and µs per tile, 3 dense species"]
fn measure_tile_scatter() {
    let g = rolling();
    let cover = three_species();
    let none = Overrides::none();
    let sc = Scatterer::new(&cover, &none, ScatterSettings::default());
    let ts = tiles(8);
    let mut out = Vec::new();
    for s in cover.indices() {
        let start = Instant::now();
        let mut total = 0u64;
        for &t in &ts {
            total += u64::from(sc.scatter_tile(&g, t, s, 0, &mut out).accepted);
        }
        let us = start.elapsed().as_secs_f64() * 1e6;
        #[allow(clippy::cast_precision_loss)]
        let (n, per) = (ts.len() as f64, total as f64 / ts.len() as f64);
        println!(
            "species {:>2} ({}, footprint {} m): {per:.0} instances/tile, {:.1} µs/tile, grid {:?}",
            s.0,
            cover.species(s).id,
            cover.species(s).footprint_m,
            us / n,
            sc.grid(s)
        );
    }
}
