//! LF-2 placement tests: the `fault_scarp` + `fan` profile on a plane (top tilt, face and talus angles, untouched
//! plain), the relay ramp between en-echelon segments keeps the walkable land in one piece (a single unbroken face
//! splits it in two), the region face budget, determinism, wet/dry and frozen anchors on a full placement.

use std::collections::BTreeMap;

use crate::geology::{DipMode, GeologyParams, GeologyTable, RockUnit, UnitLayers};
use crate::measure::{walk_slope_deg, walkability};
use crate::placed::scarp::{Profile, Segment, Shape, eval_fault, segments};
use crate::placed::{PlaceInput, PlaceParams, RegionArrays, declared_defaults, place};
use crate::{Grid, filter::set};

fn shape(spacing: f64) -> Shape {
    Shape {
        tan_face: 70f64.to_radians().tan(),
        tan_talus: 33f64.to_radians().tan(),
        fan: 0.35,
        tan_back: 6f64.to_radians().tan(),
        filter_r: spacing,
        tan_cone: 12f64.to_radians().tan(),
        tan_wall: 40f64.to_radians().tan(),
        taps: 5,
    }
}

/// Heights of `base` plus a fault's delta on a `w` x `h` grid at spacing `s`.
fn raster(segs: &[Segment], sh: &Shape, w: usize, h: usize, s: f64, base: f64) -> Grid {
    let mut g = Grid::filled(w, h, s, base);
    for r in 0..h {
        for c in 0..w {
            if let Some(hit) = eval_fault(segs, sh, [c as f64 * s, r as f64 * s]) {
                g.data[r * w + c] += hit.delta;
            }
        }
    }
    g
}

#[test]
fn filtered_profile_is_the_box_mean() {
    let p = Profile {
        h: 40.0,
        tan_face: 2.7,
        tan_talus: 0.65,
        fan: 0.35,
        tan_back: 0.1,
    };
    for k in -60..450 {
        let d = f64::from(k);
        let r = 3.0;
        let steps = 3000;
        let num: f64 = (0..steps)
            .map(|i| p.z(d - r + (f64::from(i) + 0.5) * 2.0 * r / f64::from(steps)))
            .sum::<f64>()
            / f64::from(steps);
        assert!((p.filtered(d, r) - num).abs() < 1e-3, "d {d}: {} vs {num}", p.filtered(d, r));
    }
}

#[test]
fn scarp_profile_on_plane() {
    let (s, w, h) = (2.0, 100usize, 320usize);
    let sh = shape(s);
    let lip_y = 200.0;
    let throw = 40.0;
    let seg = Segment::new(vec![[-3000.0, lip_y], [3000.0, lip_y]], 1.0, throw, 150.0);
    let g = raster(std::slice::from_ref(&seg), &sh, w, h, s, 100.0);
    let pr = sh.profile(throw);
    let [d0, dt, _, db] = pr.knots();
    let c = 50;
    let r_of = |d: f64| ((lip_y + d) / s).round() as usize;
    let slope = |r: usize| ((g.at(c, r + 1) - g.at(c, r)).abs() / s).atan().to_degrees();
    let margin = sh.filter_r + 2.0 * s;
    // Face: the steepest step is the declared face angle.
    let face_max = (r_of(dt)..r_of(0.0)).map(slope).fold(0.0, f64::max);
    println!("face max {face_max:.2} deg");
    assert!((face_max - 70.0).abs() < 1.0, "face {face_max}");
    // Talus: every step inside the talus zone (filter footprint off) at the talus angle.
    for r in r_of(d0 + margin)..r_of(dt - margin) {
        assert!((slope(r) - 33.0).abs() < 0.5, "talus row {r}: {}", slope(r));
    }
    // Top: tilted back at the declared angle, highest at the lip.
    for r in r_of(margin)..r_of(db - margin) {
        assert!((slope(r) - 6.0).abs() < 0.2, "top row {r}: {}", slope(r));
    }
    let top = (0..h).map(|r| g.at(c, r)).fold(f64::MIN, f64::max) - 100.0;
    assert!(top > throw - 2.0 && top <= throw, "top {top}");
    // Plain on both sides: untouched exactly.
    for r in 0..h {
        let d = r as f64 * s - lip_y;
        if d < d0 - sh.filter_r - s || d > db + sh.filter_r + s {
            assert_eq!(g.at(c, r).to_bits(), 100f64.to_bits(), "row {r} moved");
        }
    }
}

/// Walkable components of a fault on a plane (slope <= 46 deg, 4-connected).
fn components(segs: &[Segment], s: f64, w: usize, h: usize) -> usize {
    let g = raster(segs, &shape(s), w, h, s, 100.0);
    let slope = walk_slope_deg(&g, 0.0);
    let mask = vec![true; w * h];
    walkability(&slope, w, &mask, 46.0).components
}

#[test]
fn relay_ramp_keeps_one_component() {
    let (s, w, h) = (4.0, 500usize, 300usize);
    let trace = [[-2000.0, 400.0], [4000.0, 400.0]];
    // Two en-echelon segments overlapping inside the grid: the relay ramp joins hanging wall and footwall.
    let relay = segments(&trace, 0.0, 6000.0, 1.0, 60.0, 3000.0, 0.2, 4.0, 150.0);
    assert_eq!(relay.len(), 2);
    let n_relay = components(&relay, s, w, h);
    // One segment across the whole grid: the face cuts the land in two.
    let single = segments(&trace, 0.0, 6000.0, 1.0, 60.0, 1.0e9, 0.2, 4.0, 150.0);
    assert_eq!(single.len(), 1);
    let n_single = components(&single, s, w, h);
    println!("components: relay {n_relay}, single face {n_single}");
    assert_eq!(n_relay, 1);
    assert_eq!(n_single, 2);
}

fn table() -> GeologyTable {
    let u = |name: &str, hardness: f64, layered: bool| RockUnit {
        name: name.into(),
        hardness,
        k: 1.0,
        layered,
        bed_m: [3.0, 9.0],
        soft_bed_share: 0.3,
        dip_deg: 0.0,
        dip_mode: DipMode::Surface,
        repose_hard_deg: 70.0,
        repose_soft_deg: 33.0,
        outcrop: false,
        layers: UnitLayers::default(),
    };
    GeologyTable {
        units: vec![u("hard", 0.85, true), u("soft", 0.3, false)],
        shares: BTreeMap::from([("hard".into(), 1.0), ("soft".into(), 1.0)]),
        ..GeologyTable::default()
    }
}

struct Fixture {
    w: usize,
    h: usize,
    s: f64,
    heights: Vec<f64>,
    frozen: Vec<bool>,
}

fn fixture() -> Fixture {
    let (w, h, s) = (260usize, 220usize, 18.288);
    let mut heights = vec![0.0; w * h];
    let mut frozen = vec![false; w * h];
    for r in 0..h {
        for c in 0..w {
            let (x, y) = (c as f64 * s, r as f64 * s);
            let i = r * w + c;
            heights[i] = if c < 8 {
                -5.0
            } else {
                40.0 + 0.01 * x + 3.0 * (x / 300.0).sin() * (y / 400.0).cos()
            };
            frozen[i] = (150..160).contains(&c) && (100..110).contains(&r);
        }
    }
    Fixture {
        w,
        h,
        s,
        heights,
        frozen,
    }
}

fn run(f: &Fixture, sets: &[(&str, f64)]) -> crate::placed::Placed {
    let mut d = declared_defaults();
    for &(k, v) in [("size_min_m", 400.0), ("size_max_m", 3000.0), ("fault_share", 1.0), ("face_budget", 0.02)]
        .iter()
        .chain(sets)
    {
        set(&mut d, k, v, None).unwrap();
    }
    let p = PlaceParams::from_declared(&d).unwrap();
    let t = table();
    let gp = GeologyParams::from_declared(&t.declared().unwrap()).unwrap();
    let arrays = RegionArrays::uniform(&p);
    place(
        &PlaceInput {
            heights: &f.heights,
            w: f.w,
            h: f.h,
            spacing_m: f.s,
            origin_m: [0.0, 0.0],
            sea_level_m: 0.0,
            regions: None,
            frozen: &f.frozen,
            weight: &arrays.weight,
            budget: &arrays.budget,
            size_m: &arrays.size_m,
            throw_ratio: &arrays.throw_ratio,
        },
        &t,
        &gp,
        &p,
    )
    .unwrap()
}

#[test]
fn placement_budget_determinism_and_invariants() {
    let f = fixture();
    let a = run(&f, &[]);
    let b = run(&f, &[]);
    let r = &a.report;
    println!(
        "traces {}, candidates {}, placed {} ({} segments), rejected budget {} spacing {}, ramps {}",
        r.traces, r.candidates, r.placed, r.segments, r.rejected_budget, r.rejected_spacing, r.ramps
    );
    for row in &r.regions {
        println!(
            "  {}: land {:.2} km2, budget {:.4} km2, face {:.4} km2, cliff {:.4} km2, largest {:.3} -> {:.3}",
            row.region,
            row.land_km2,
            row.budget_km2,
            row.face_km2,
            row.cliff_km2,
            row.route.largest_before,
            row.route.largest_after
        );
        assert!(row.face_km2 <= row.budget_km2 + 1e-12, "budget exceeded in {}", row.region);
    }
    assert!(r.placed >= 1, "fixture places no fault");
    assert_eq!(r.hash, b.report.hash, "not deterministic");
    assert_eq!((r.new_wet, r.new_dry, r.sea_moved, r.frozen_moved), (0, 0, 0, 0));
    for i in 0..f.heights.len() {
        if f.frozen[i] || f.heights[i] < 0.0 {
            assert_eq!(a.heights[i].to_bits(), f.heights[i].to_bits(), "sample {i} moved");
        }
    }
    // A smaller budget is respected too.
    let small = run(&f, &[("face_budget", 0.005)]);
    assert!(small.report.regions[0].face_km2 <= small.report.regions[0].budget_km2 + 1e-12);
    // No budget and no ramps: the base comes back bit for bit.
    let none = run(&f, &[("face_budget", 0.0), ("route.max_ramps", 0.0)]);
    assert_eq!(none.report.placed, 0);
    assert!(none.heights.iter().zip(&f.heights).all(|(x, y)| x.to_bits() == y.to_bits()));
}

#[test]
fn repair_ramp_climbs_a_step_but_never_bridges_a_ravine() {
    use crate::placed::routes::{RouteParams, cut_ramp};
    let p = RouteParams {
        max_walk_deg: 46.0,
        min_largest: 0.95,
        ramp_deg: 30.0,
        ramp_width_m: 24.0,
        max_ramps: 10,
        min_component_m2: 0.0,
        max_cross_m: 300.0,
        max_ramp_m: 800.0,
        shoulder_m: 30.0,
        max_cut_m: 25.0,
        rounds: 2,
    };
    let (w, h, s) = (120usize, 60usize, 10.0);
    let frozen = vec![false; w * h];
    // A 30 m step at x = 600 m: the ramp fits.
    let mut step = Grid::filled(w, h, s, 100.0);
    for r in 0..h {
        for c in 60..w {
            step.data[r * w + c] = 130.0;
        }
    }
    let r = cut_ramp(&mut step, 0.0, &frozen, [590.0, 300.0], [610.0, 300.0], 0, &p).expect("step ramp");
    assert!(r.grade_deg <= 30.0 + 1e-9, "grade {}", r.grade_deg);
    // A 150 m deep ravine between two rims: no causeway.
    let mut ravine = Grid::filled(w, h, s, 300.0);
    for r in 0..h {
        for c in 50..70 {
            ravine.data[r * w + c] = 150.0;
        }
    }
    let before = ravine.clone();
    assert!(cut_ramp(&mut ravine, 0.0, &frozen, [480.0, 300.0], [720.0, 300.0], 0, &p).is_none());
    assert_eq!(ravine, before, "a refused ramp wrote heights");
}

/// LF-2b eroded face on a plane: the face is dissected (the steepest slope across the strike varies along it: steep
/// facets, gentler chutes), a debris cone stands below each chute mouth (higher than the ground below the facet
/// beside it), the throw varies along strike, and a full placement with the defaults builds chutes.
#[test]
fn eroded_face_has_facets_chutes_and_cones() {
    use crate::placed::facets::{self, ChuteParams, Wobble};
    let sh = shape(0.0);
    let (len, h) = (3000.0, 120.0);
    let mut seg = Segment::new(vec![[0.0, 0.0], [len, 0.0]], 1.0, h, 600.0);
    seg.tan_face = Some(65f64.to_radians().tan());
    seg.wobble = Wobble { amp: 0.3, wavelength_m: 800.0, seed: 3 };
    let cp = ChuteParams {
        min_m: 50.0,
        max_m: 300.0,
        b: 1.0,
        width_share: 0.9,
        depth_max: 0.85,
        fan_share: 0.35,
        facet_talus: 0.1,
    };
    seg.chutes = facets::chutes(len, h, sh.tan_wall, &cp, 17);
    seg.chute_reach = facets::chute_reach(&seg, &sh);
    assert!(seg.chutes.len() >= 10, "chutes {}", seg.chutes.len());
    let z = |s: f64, d: f64| facets::point(&seg, &sh, s, d).0;
    // Steepest slope across the strike per column, middle of the fault (tapers off).
    let mut steepest = Vec::new();
    let mut s = 700.0;
    while s < len - 700.0 {
        let mut m = 0.0f64;
        let mut d = -150.0;
        while d < 60.0 {
            m = m.max(((z(s, d + 1.0) - z(s, d - 1.0)) / 2.0).abs());
            d += 1.0;
        }
        steepest.push(m.atan().to_degrees());
        s += 5.0;
    }
    let lo = steepest.iter().copied().fold(f64::INFINITY, f64::min);
    let hi = steepest.iter().copied().fold(0.0, f64::max);
    println!("steepest face slope along strike: {lo:.1} .. {hi:.1} deg over {} columns", steepest.len());
    assert!(hi > 60.0 && lo < 50.0, "face not dissected: {lo} .. {hi}");
    // Cones: below each mouth the ground stands higher than below the facet midway to the next chute.
    let (mut higher, mut pairs) = (0, 0);
    for w in seg.chutes.windows(2) {
        let (a, b) = (w[0], w[1]);
        if a.s < 700.0 || b.s > len - 700.0 || b.s - a.s < 60.0 {
            continue;
        }
        let ha = seg.throw_at(a.s);
        let dm = -(ha - a.apex * ha) / seg.tan_face.unwrap();
        let d = dm - 15.0;
        pairs += 1;
        if z(a.s, d) > z(f64::midpoint(a.s, b.s), d) {
            higher += 1;
        }
    }
    println!("cones higher than the facet foot beside them: {higher} / {pairs}");
    assert!(pairs > 3 && higher * 10 >= pairs * 8, "{higher} / {pairs}");
    // Throw along strike varies.
    let th: Vec<f64> = (0..100).map(|k| seg.throw_at(700.0 + f64::from(k) * 16.0)).collect();
    let (tmin, tmax) = th.iter().fold((f64::INFINITY, 0.0f64), |(a, b), &v| (a.min(v), b.max(v)));
    println!("throw along strike {tmin:.1} .. {tmax:.1} m (nominal {h})");
    assert!(tmax - tmin > 0.2 * h);
    // Never below the plain, finite everywhere.
    for k in 0..4000 {
        let (s, d) = (f64::from(k % 100) * 30.0, f64::from(k / 100) * 10.0 - 250.0);
        let v = z(s, d);
        assert!(v.is_finite() && v >= 0.0, "z({s}, {d}) = {v}");
    }
    // A full placement with the defaults (eroded 1) builds chutes.
    let f = fixture();
    let placed = run(&f, &[]);
    let chutes: usize = placed.features.iter().map(|x| x.chutes).sum();
    println!("fixture: {} faults, {chutes} chutes", placed.features.len());
    assert!(chutes > 0);
    let plain = run(&f, &[("eroded", 0.0)]);
    assert!(plain.features.iter().all(|x| x.chutes == 0));
}
