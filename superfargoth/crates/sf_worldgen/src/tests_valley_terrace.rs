//! Tests of [`crate::valley_terrace`] (spec `site-prep.md` "Tests" 1-8), synthetic fixtures only: a straight
//! channel through a plain at +25 m (bank walls 30 m wide), a ring channel (a bend everywhere), and the straight
//! channel with a mesa on the east bank higher than the top tread plus `cut_cap_m`.

use std::collections::BTreeMap;
use std::sync::OnceLock;

use crate::stamps::{Stamp, StampSet};
use crate::valley_terrace::{
    Kind, LineKind, ValleyInput, ValleyParams, ValleyReport, ValleyTerrace, declared_defaults, final_surface,
    merge_over, valley_terrace,
};

/// Guide height across a channel at distance `x` from its centreline: bed -20 m to 60 m, rising to the water's edge
/// (0 m) at 75 m, a 30 m bank wall up to the plain at +25 m.
fn across(x: f64) -> f64 {
    if x < 60.0 {
        -20.0
    } else if x < 75.0 {
        -20.0 + 20.0 * (x - 60.0) / 15.0
    } else if x < 105.0 {
        25.0 * (x - 75.0) / 30.0
    } else {
        25.0
    }
}

fn straight(p: [f64; 2]) -> f64 {
    across(p[0].abs())
}

/// Ring channel of centreline radius 400 m around (-400, 0): passes through the origin running north-south.
fn bend(p: [f64; 2]) -> f64 {
    across(((p[0] + 400.0).hypot(p[1]) - 400.0).abs())
}

/// Mesa on the east bank, standing on the stair: top +75 m (radius 20 m), flanks at slope 0.8 (steeper than talus).
fn hill(p: [f64; 2]) -> f64 {
    let r = (p[0] - 260.0).hypot(p[1]);
    straight(p).max(75.0 - 0.8 * (r - 20.0).max(0.0))
}

fn run(guide: &'static (dyn Fn([f64; 2]) -> f64 + Sync), reach: f64, seed: u64) -> ValleyTerrace {
    run_with(guide, reach, seed, &ValleyParams::defaults())
}

fn run_with(guide: &'static (dyn Fn([f64; 2]) -> f64 + Sync), reach: f64, seed: u64, p: &ValleyParams) -> ValleyTerrace {
    let inp = ValleyInput {
        name: "valley".into(),
        guide,
        sea_level_m: 0.0,
        centre_m: [0.0, 0.0],
        reach_m: reach,
        core_radius_m: 150.0,
        seed,
        street_grade: 0.12,
        share_within_m: 300.0,
    };
    valley_terrace(&inp, p).expect("operator runs")
}

fn straight_run() -> &'static ValleyTerrace {
    static R: OnceLock<ValleyTerrace> = OnceLock::new();
    R.get_or_init(|| run(&straight, 300.0, 7))
}

/// The straight channel without ramps.
fn straight_off() -> &'static ValleyTerrace {
    static R: OnceLock<ValleyTerrace> = OnceLock::new();
    R.get_or_init(|| run_with(&straight, 300.0, 7, &ValleyParams { ramps: false, ..ValleyParams::defaults() }))
}

fn bend_run() -> &'static ValleyTerrace {
    static R: OnceLock<ValleyTerrace> = OnceLock::new();
    R.get_or_init(|| run(&bend, 300.0, 11))
}

fn hill_run() -> &'static ValleyTerrace {
    static R: OnceLock<ValleyTerrace> = OnceLock::new();
    R.get_or_init(|| run(&hill, 450.0, 13))
}

/// A fixture: name, the operator's result, the guide.
type Fixture = (&'static str, &'static ValleyTerrace, &'static (dyn Fn([f64; 2]) -> f64 + Sync));

fn all() -> [Fixture; 3] {
    [("straight", straight_run(), &straight), ("bend", bend_run(), &bend), ("hill", hill_run(), &hill)]
}

/// Relative tolerance of slope bounds: the operator corrects riser runs and tread falls for oblique lines to first
/// order; the residual is the noise curvature (about 2 % at the defaults) plus the perpendicular's estimate from
/// the distance field's central differences.
const SECOND_ORDER: f64 = 1.03;

fn pos(st: &Stamp, k: usize) -> [f64; 2] {
    let w = st.dims[0];
    [st.origin_m[0] + (k % w) as f64 * st.spacing_m, st.origin_m[1] - (k / w) as f64 * st.spacing_m]
}

/// Central-difference gradient (gx east, gy north) of a grid at interior sample `k`.
fn grad(z: &[f64], w: usize, k: usize, s: f64) -> (f64, f64) {
    ((z[k + 1] - z[k - 1]) / (2.0 * s), (z[k - w] - z[k + w]) / (2.0 * s))
}

fn interior(w: usize, h: usize, k: usize) -> bool {
    let (i, j) = (k % w, k / w);
    i > 0 && j > 0 && i + 1 < w && j + 1 < h
}

/// Sample `k` and its four neighbours share `kind` and weight 1.
fn core_of(r: &ValleyTerrace, k: usize, kind: Kind) -> bool {
    let w = r.stamp.dims[0];
    [k, k - 1, k + 1, k - w, k + w].iter().all(|&n| r.diag.kind[n] == kind && r.stamp.weights[n] >= 1.0)
}

#[test]
fn declared_defaults_are_valid_and_blend_rule_is_checked() {
    let p = ValleyParams::defaults();
    assert_eq!(p.treads_max, 5);
    assert_eq!(declared_defaults().len(), 25);
    assert!(p.ramps);
    // cut_cap 25 / talus 0.4 = 62.5 m > blend 40 m: refused at load.
    let bad: BTreeMap<String, f64> = [("cut_cap_m".to_string(), 25.0), ("talus_slope".to_string(), 0.4)].into();
    assert!(ValleyParams::from_values(&bad).is_err());
    let unknown: BTreeMap<String, f64> = [("nope".to_string(), 1.0)].into();
    assert!(ValleyParams::from_values(&unknown).is_err());
}

/// Test 1: weight 0 on every wet sample and beyond the extent plus blend.
#[test]
fn weight_zero_on_wet_and_outside() {
    let p = ValleyParams::defaults();
    for (name, r, guide) in all() {
        let st = &r.stamp;
        for k in 0..st.weights.len() {
            let pt = pos(st, k);
            if guide(pt) <= 0.0 {
                assert_eq!(r.diag.kind[k], Kind::Wet, "{name}: wet sample {k} not classed wet");
                assert_eq!(st.weights[k].to_bits(), 0f32.to_bits(), "{name}: wet sample {k} weighted");
            }
            if pt[0].hypot(pt[1]) >= r.report.reach_m + p.edge_blend_m() {
                assert_eq!(st.weights[k].to_bits(), 0f32.to_bits(), "{name}: sample {k} beyond extent + blend weighted");
            }
        }
        assert!(st.weights.iter().any(|&v| v >= 1.0), "{name}: nothing weighted");
    }
}

/// Test 2: along any perpendicular from the wet edge the target never decreases; tread levels rise by at most
/// `riser_h_m (1 + vary_amp)`. The straight channel's rows are its perpendiculars (each row's samples share a foot).
#[test]
fn target_is_monotone_and_rises_are_bounded() {
    let p = ValleyParams::defaults();
    let r = straight_run();
    let st = &r.stamp;
    let (w, h) = (st.dims[0], st.dims[1]);
    let bound = p.riser_h_m * (1.0 + p.vary_amp) + 1e-4;
    let mut rows = 0;
    for j in 0..h {
        for dir in [1i64, -1] {
            // From the wet edge outward: first dry sample east (dir 1) or west (dir -1) of the centreline.
            let mut i = (w / 2) as i64;
            while (0..w as i64).contains(&i) && r.diag.kind[j * w + i as usize] == Kind::Wet {
                i += dir;
            }
            let (mut prev, mut levels, mut run_max): (Option<f32>, Vec<f64>, Option<f64>) = (None, Vec::new(), None);
            while (0..w as i64).contains(&i) {
                let k = j * w + i as usize;
                if st.weights[k] < 1.0 {
                    break;
                }
                let z = st.heights[k];
                if let Some(pz) = prev {
                    assert!(z >= pz, "row {j} dir {dir}: target falls {pz} -> {z} at column {i}");
                }
                prev = Some(z);
                if matches!(r.diag.kind[k], Kind::Tread | Kind::Quay) {
                    run_max = Some(run_max.map_or(f64::from(z), |m: f64| m.max(f64::from(z))));
                } else if let Some(m) = run_max.take() {
                    levels.push(m);
                }
                i += dir;
            }
            levels.extend(run_max);
            for pair in levels.windows(2) {
                let rise = pair[1] - pair[0];
                assert!(rise > 0.0 && rise <= bound, "row {j} dir {dir}: tread levels {pair:?} rise {rise}");
            }
            rows += usize::from(levels.len() >= 3);
        }
    }
    assert!(rows > h, "too few perpendiculars with a stair: {rows}");
}

/// Test 3: slopes. Treads (along the perpendicular) <= `tread_fall`, and <= 0.12 in any direction; risers (along
/// the perpendicular) <= their kind's slope; natural banks <= max(talus, guide slope).
#[test]
fn slopes_hold_per_kind() {
    let p = ValleyParams::defaults();
    // A 5x5 chamfer cone (directions 0, 26.6, 45 deg) is steeper than its slope by at most 1 / cos(13.3 deg).
    let cone = 1.0 / (0.5f64.atan() / 2.0).cos();
    for (name, r, _) in all() {
        let st = &r.stamp;
        let (w, h, s) = (st.dims[0], st.dims[1], st.spacing_m);
        let z: Vec<f64> = st.heights.iter().map(|&v| f64::from(v)).collect();
        let g: Vec<f64> = r.diag.guide_m.iter().map(|&v| f64::from(v)).collect();
        let tz: Vec<f64> = r.diag.top_m.iter().map(|&v| f64::from(v)).collect();
        let mut seen = [0usize; 4];
        for k in (0..z.len()).filter(|&k| interior(w, h, k)) {
            let (gx, gy) = grad(&z, w, k, s);
            let slope = gx.hypot(gy);
            // Perpendicular derivative: along the gradient of the distance field.
            let d: Vec<f64> = [k - 1, k + 1, k - w, k + w].iter().map(|&n| f64::from(r.diag.d_m[n])).collect();
            let (dx, dy) = ((d[1] - d[0]) / (2.0 * s), (d[2] - d[3]) / (2.0 * s));
            let dl = dx.hypot(dy);
            let perp = if dl > 0.0 { (gx * dx + gy * dy) / dl } else { slope };
            let kind = r.diag.kind[k];
            match kind {
                Kind::Tread if core_of(r, k, kind) => {
                    seen[0] += 1;
                    // The fall's own variation over a tread's width is a larger second-order term (drainage only).
                    assert!(perp <= p.tread_fall * 1.1, "{name}: tread {k} perpendicular slope {perp}");
                    assert!(slope <= 0.12, "{name}: tread {k} slope {slope}");
                }
                Kind::Wall | Kind::Bank if core_of(r, k, kind) => {
                    seen[1 + usize::from(kind == Kind::Bank)] += 1;
                    let max = if kind == Kind::Wall { p.riser_slope } else { p.bank_slope };
                    assert!(
                        perp <= max * SECOND_ORDER,
                        "{name}: {kind:?} riser {k} at {:?} d {} perpendicular slope {perp} > {max}; row {:?} up {:?} down {:?}",
                        pos(st, k),
                        r.diag.d_m[k],
                        &st.heights[k - 3..=k + 3],
                        &st.heights[k - w - 3..=k - w + 3],
                        &st.heights[k + w - 3..=k + w + 3],
                    );
                }
                Kind::Natural if core_of(r, k, kind) => {
                    seen[3] += 1;
                    let gmax = [k, k - 1, k + 1, k - w, k + w]
                        .iter()
                        .filter(|&&n| interior(w, h, n))
                        .map(|&n| {
                            let (a, b) = grad(&g, w, n, s);
                            a.hypot(b)
                        })
                        .fold(0.0, f64::max);
                    // The bank rises from a top tread whose level follows the bank top along the valley (+ its
                    // gradient); central differences straddle the cone's facets and the cone / guide junction.
                    let (tx, ty) = grad(&tz, w, k, s);
                    let dtop = if tx.is_finite() && ty.is_finite() { tx.hypot(ty) } else { 0.0 };
                    let max = (p.talus_slope * cone + dtop).max(gmax) * SECOND_ORDER;
                    assert!(slope <= max, "{name}: natural {k} slope {slope} > {max}");
                }
                _ => {}
            }
        }
        assert!(seen[0] > 0 && seen[1] > 0 && seen[2] > 0, "{name}: kinds seen {seen:?}");
        if name == "hill" {
            assert!(seen[3] > 0, "hill: no natural bank");
        }
    }
}

/// Test 4: the cut cap holds where `w = 1` (ground above the top tread is cut at most `cut_cap_m`), and the report's
/// cut and fill equal the sums recomputed from the stamp.
#[test]
fn cut_cap_holds_and_report_sums_match() {
    let p = ValleyParams::defaults();
    for (name, r, _) in all() {
        let st = &r.stamp;
        let fin = final_surface(st, &r.diag.guide_m);
        let mut capped = 0;
        for (k, &f) in fin.iter().enumerate() {
            let top = f64::from(r.diag.top_m[k]);
            if st.weights[k] < 1.0 || !top.is_finite() {
                continue;
            }
            let g = f64::from(r.diag.guide_m[k]);
            assert!(g - f.max(top) <= p.cut_cap_m + 1e-4, "{name}: sample {k} cut {} past the cap", g - f);
            capped += usize::from(g > top + p.cut_cap_m);
        }
        if name == "hill" {
            assert!(capped > 0, "hill: the cap never engaged");
        }
        let area = st.spacing_m * st.spacing_m;
        let mut sums: BTreeMap<u32, (f64, f64)> = BTreeMap::new();
        for (k, &f) in fin.iter().enumerate() {
            let b = r.diag.bank[k];
            if b == u32::MAX {
                continue;
            }
            let e = sums.entry(b).or_default();
            let dz = f - f64::from(r.diag.guide_m[k]);
            e.0 += (-dz).max(0.0) * area;
            e.1 += dz.max(0.0) * area;
        }
        assert!(r.report.banks.len() >= 2, "{name}: banks {:?}", r.report.banks.len());
        for b in &r.report.banks {
            let (cut, fill) = sums[&b.id];
            assert_eq!(b.cut_m3.to_bits(), cut.to_bits(), "{name}: bank {} cut", b.id);
            assert_eq!(b.fill_m3.to_bits(), fill.to_bits(), "{name}: bank {} fill", b.id);
        }
        assert!(r.report.cut_m3 > 0.0);
    }
}

/// Test 5: no step between weighted neighbours steeper than the riser slope (no seam at the extent edge).
#[test]
fn no_seam_steeper_than_riser_slope() {
    let p = ValleyParams::defaults();
    for (name, r, _) in all() {
        let st = &r.stamp;
        let (w, s) = (st.dims[0], st.spacing_m);
        let fin = final_surface(st, &r.diag.guide_m);
        let mut worst = (0.0f64, 0usize);
        for k in 0..fin.len() {
            for nb in [k + 1, k + w] {
                if nb >= fin.len() || (nb == k + 1 && (k + 1) % w == 0) {
                    continue;
                }
                if st.weights[k] > 0.0 && st.weights[nb] > 0.0 {
                    let step = (fin[k] - fin[nb]).abs() / s;
                    if step > worst.0 {
                        worst = (step, k);
                    }
                }
            }
        }
        assert!(worst.0 <= p.riser_slope * SECOND_ORDER, "{name}: step {} at sample {} ({:?})", worst.0, worst.1, r.diag.kind[worst.1]);
    }
}

/// Test 6: the written stamp read back through `stamps` returns every grid value bit for bit; same input, same
/// blake3; `valley.json` reads back equal.
#[test]
fn stamp_round_trips_bit_for_bit_and_is_deterministic() {
    let r = straight_run();
    let dir = std::env::temp_dir().join(format!("sf_worldgen_valley_rt_{}", std::process::id()));
    let dir2 = dir.join("again");
    r.write(&dir).expect("write");
    let set = StampSet::read(&dir).expect("read").expect("stamps.json present");
    assert_eq!(set.stamps.len(), 1);
    let back = &set.stamps[0];
    let st = &r.stamp;
    assert_eq!((back.dims, back.origin_m, back.spacing_m.to_bits()), (st.dims, st.origin_m, st.spacing_m.to_bits()));
    for k in 0..st.heights.len() {
        assert_eq!(back.heights[k].to_bits(), st.heights[k].to_bits());
        assert_eq!(back.weights[k].to_bits(), st.weights[k].to_bits());
        let pt = pos(st, k);
        let (hh, ww) = set.sample(pt[0], pt[1]).expect("inside the stamp");
        assert_eq!((hh.to_bits(), ww.to_bits()), (st.heights[k].to_bits(), st.weights[k].to_bits()), "sample {k}");
    }
    // serde_json parses floats to within an ulp; compare the text it writes back instead.
    let text = std::fs::read_to_string(dir.join("valley.json")).expect("valley.json");
    let report: ValleyReport = serde_json::from_str(&text).expect("parse");
    assert_eq!(report.lines, r.report.lines.iter().map(|l| {
        let mut l = l.clone();
        for p in &mut l.points { *p = [round_trip(p[0]), round_trip(p[1])]; }
        l
    }).collect::<Vec<_>>());
    assert_eq!(report.banks.len(), r.report.banks.len());
    run(&straight, 300.0, 7).write(&dir2).expect("write again");
    let hash = |d: &std::path::Path| blake3::hash(&std::fs::read(d.join("stamps.f32")).expect("stamps.f32")).to_hex();
    assert_eq!(hash(&dir), hash(&dir2));
    let _ = std::fs::remove_dir_all(&dir);
}

fn round_trip(v: f64) -> f64 {
    serde_json::from_str(&serde_json::to_string(&v).expect("ser")).expect("de")
}

/// Test 7: merge exactness, `mix(mix(h, h_s, w_s), h_t, w_t) == mix(h, h_m, w_m)` within 1e-4 m on the common grid.
#[test]
fn merge_composes_exactly() {
    let grid = |name: &str, origin: [f64; 2], dims: [usize; 2], f: &dyn Fn(f64, f64) -> (f32, f32)| {
        let mut st = Stamp { name: name.into(), origin_m: origin, dims, spacing_m: 1.0, heights: vec![], weights: vec![] };
        for j in 0..dims[1] {
            for i in 0..dims[0] {
                let (hh, ww) = f(origin[0] + i as f64, origin[1] - j as f64);
                st.heights.push(hh);
                st.weights.push(ww);
            }
        }
        st
    };
    let wave = |x: f64, y: f64, a: f64| (0.5 + 0.5 * (x * a).sin() * (y * a * 0.7).cos()).clamp(0.0, 1.0) as f32;
    let base = grid("valley", [-50.0, 40.0], [120, 90], &|x, y| ((20.0 + 0.1 * x - 0.05 * y) as f32, wave(x, y, 0.05)));
    let top = grid("town", [10.0, 15.0], [100, 70], &|x, y| ((30.0 + 0.02 * x * y / 50.0) as f32, wave(x, y, 0.09)));
    let m = merge_over(&base, &top).expect("same lattice");
    let guide = |x: f64, y: f64| 5.0 + 0.3 * x - 0.2 * y + (x * 0.1).sin() * 4.0;
    let mix = |a: f64, b: f64, t: f64| a + (b - a) * t;
    let get = |st: &Stamp, x: f64, y: f64| st.sample(x, y).map_or((0.0, 0.0), |(a, b)| (f64::from(a), f64::from(b)));
    for k in 0..m.heights.len() {
        let [x, y] = pos(&m, k);
        let h = guide(x, y);
        let (hs, ws) = get(&base, x, y);
        let (ht, wt) = get(&top, x, y);
        let (hm, wm) = (f64::from(m.heights[k]), f64::from(m.weights[k]));
        assert!((0.0..=1.0).contains(&wm));
        let a = mix(mix(h, hs, ws), ht, wt);
        let b = mix(h, hm, wm);
        assert!((a - b).abs() <= 1e-4, "({x}, {y}): {a} vs {b}");
    }
    let mut off = top.clone();
    off.origin_m[0] += 0.5;
    assert!(merge_over(&base, &off).is_err(), "off-lattice merge refused");
}

/// Test 8: riser kinds: walls inside `core_radius_m`, earth banks outside, the quay line at the quay edge.
#[test]
fn riser_kinds_follow_the_core() {
    let p = ValleyParams::defaults();
    for (name, r, _) in all() {
        let core = r.report.core_radius_m;
        let st = &r.stamp;
        let mut kinds = [0usize; 4];
        for line in &r.report.lines {
            for pt in &line.points {
                let rr = pt[0].hypot(pt[1]);
                match line.kind {
                    LineKind::Wall => assert!(rr <= core + 1e-9, "{name}: wall line point at {rr}"),
                    LineKind::Bank => assert!(rr > core, "{name}: bank line point at {rr}"),
                    LineKind::Quay => {
                        let i = ((pt[0] - st.origin_m[0]) / st.spacing_m).round() as usize;
                        let j = ((st.origin_m[1] - pt[1]) / st.spacing_m).round() as usize;
                        let k = j * st.dims[0] + i;
                        assert!(
                            matches!(r.diag.kind[k], Kind::Wet | Kind::Quay),
                            "{name}: quay point {pt:?} on {:?}",
                            r.diag.kind[k]
                        );
                    }
                    LineKind::Ramp => {}
                }
            }
            kinds[line.kind as usize] += 1;
        }
        assert!(kinds.iter().all(|&c| c > 0), "{name}: lines per kind (wall, bank, quay, ramp) {kinds:?}");
        // Per sample: a wall riser starts inside the core, a bank riser outside (a run is at most rise / slope,
        // stretched by under 2x where the riser line is oblique).
        let rise = 2.0 * p.riser_h_m * (1.0 + p.vary_amp);
        for k in 0..st.heights.len() {
            let rr = pos(st, k)[0].hypot(pos(st, k)[1]);
            match r.diag.kind[k] {
                Kind::Wall => assert!(rr <= core + rise / p.riser_slope + st.spacing_m, "{name}: wall sample at {rr}"),
                Kind::Bank => assert!(rr >= core - rise / p.bank_slope - st.spacing_m, "{name}: bank sample at {rr}"),
                _ => {}
            }
        }
    }
}

/// Street-grade step of the reachability flood: the planner's 0.12 x 1.05, set once: a ramp floor rises at
/// `ramp_grade` (0.10), and where the floor meets a cut face or a tread's cross-fall a 1 m step can add a few
/// hundredths (the operator's first-order corrections, as `SECOND_ORDER`).
const STREET_STEP: f64 = 0.12 * 1.05;

/// Samples reachable from any quay sample over 4-neighbour steps of at most `STREET_STEP` x spacing on the final
/// surface, through dry samples inside the extent (the flood never leaves the town site through the edge fade).
fn reachable(r: &ValleyTerrace, centre: [f64; 2], reach: f64) -> Vec<bool> {
    let st = &r.stamp;
    let (w, h, s) = (st.dims[0], st.dims[1], st.spacing_m);
    let fin = final_surface(st, &r.diag.guide_m);
    let open = |k: usize| {
        let p = pos(st, k);
        r.diag.bank[k] != u32::MAX && (p[0] - centre[0]).hypot(p[1] - centre[1]) <= reach
    };
    let mut seen = vec![false; fin.len()];
    let mut stack: Vec<usize> = (0..fin.len()).filter(|&k| open(k) && r.diag.kind[k] == Kind::Quay).collect();
    for &k in &stack {
        seen[k] = true;
    }
    while let Some(k) = stack.pop() {
        let (i, j) = (k % w, k / w);
        let mut nbs = Vec::with_capacity(4);
        if i > 0 {
            nbs.push(k - 1);
        }
        if i + 1 < w {
            nbs.push(k + 1);
        }
        if j > 0 {
            nbs.push(k - w);
        }
        if j + 1 < h {
            nbs.push(k + w);
        }
        for nb in nbs {
            if !seen[nb] && open(nb) && (fin[nb] - fin[k]).abs() <= STREET_STEP * s {
                seen[nb] = true;
                stack.push(nb);
            }
        }
    }
    seen
}

/// Ramps test 1 (spec "Ramps (SP-1c)"): from the quay, every tread and the untouched plain beyond the stair on both
/// banks are reachable at the street grade; without ramps the plain is not.
#[test]
fn ramps_connect_quay_to_plain() {
    for (label, r, on) in [("ramps", straight_run(), true), ("no ramps", straight_off(), false)] {
        let st = &r.stamp;
        let reach = r.report.reach_m;
        let seen = reachable(r, [0.0, 0.0], reach);
        let inside = |k: usize| pos(st, k)[0].hypot(pos(st, k)[1]) <= reach;
        for east in [true, false] {
            let side = |k: usize| (pos(st, k)[0] > 0.0) == east;
            let (mut all, mut hit) = (0usize, 0usize);
            for k in (0..seen.len()).filter(|&k| inside(k) && side(k) && r.diag.kind[k] == Kind::Untouched) {
                all += 1;
                hit += usize::from(seen[k]);
            }
            assert!(all > 0, "{label}: no plain inside the extent (east {east})");
            assert_eq!(hit > 0, on, "{label}: plain reached {hit} of {all} (east {east})");
            if on {
                let treads: Vec<usize> = (0..seen.len())
                    .filter(|&k| inside(k) && side(k) && r.diag.kind[k] == Kind::Tread && st.weights[k] >= 1.0)
                    .collect();
                let missed: Vec<[f64; 2]> = treads.iter().filter(|&&k| !seen[k]).map(|&k| pos(st, k)).collect();
                assert!(missed.is_empty(), "{label}: {} of {} treads unreached (east {east}), e.g. {:?}", missed.len(), treads.len(), missed.iter().step_by(missed.len() / 8 + 1).collect::<Vec<_>>());
            }
        }
    }
}

/// Final surface at a guide-frame point, bilinear on the stamp grid.
fn fin_at(st: &Stamp, fin: &[f64], pt: [f64; 2]) -> f64 {
    let (w, s) = (st.dims[0], st.spacing_m);
    let u = (pt[0] - st.origin_m[0]) / s;
    let v = (st.origin_m[1] - pt[1]) / s;
    let (i, j) = (u.floor() as usize, v.floor() as usize);
    let (tx, ty) = (u - i as f64, v - j as f64);
    let at = |a: usize, b: usize| fin[b * w + a];
    let top = at(i, j) + (at(i + 1, j) - at(i, j)) * tx;
    let bot = at(i, j + 1) + (at(i + 1, j + 1) - at(i, j + 1)) * tx;
    top + (bot - top) * ty
}

/// Ramps test 2: along every ramp centreline the final surface rises at most `ramp_grade` (x 1.05, as
/// `STREET_STEP`); each end meets a tread (or the quay) nearby within 0.05 m.
#[test]
fn ramp_centrelines_hold_the_grade_and_meet_their_treads() {
    let p = ValleyParams::defaults();
    let near = p.ramp_w_m + p.riser_h_m * (1.0 + p.vary_amp) / p.bank_slope;
    for (name, r, _) in all() {
        let st = &r.stamp;
        let fin = final_surface(st, &r.diag.guide_m);
        let ramps: Vec<_> = r.report.lines.iter().filter(|l| l.kind == LineKind::Ramp).collect();
        assert!(!ramps.is_empty(), "{name}: no ramps");
        for l in ramps {
            let z: Vec<f64> = l.points.iter().map(|&pt| fin_at(st, &fin, pt)).collect();
            for (pp, zz) in l.points.windows(2).zip(z.windows(2)) {
                let d = (pp[1][0] - pp[0][0]).hypot(pp[1][1] - pp[0][1]);
                let slope = (zz[1] - zz[0]).abs() / d;
                assert!(slope <= p.ramp_grade * 1.05, "{name}: ramp of riser {} slope {slope} at {:?}", l.riser, pp[0]);
            }
            assert!(z[z.len() - 1] > z[0], "{name}: ramp of riser {} does not climb", l.riser);
            // The floor's level at each end, read 1.5 samples inside the ramp and carried back at the grade: the
            // bottom end is the corner where the floor meets the end face behind it, so a bilinear read on the end
            // itself mixes in the face (up to ~0.1 m on an oblique line).
            let delta = 1.5 * st.spacing_m;
            let floor_at = |end: [f64; 2], toward: [f64; 2], sign: f64| {
                let (dx, dy) = (toward[0] - end[0], toward[1] - end[1]);
                let l = dx.hypot(dy);
                fin_at(st, &fin, [end[0] + dx / l * delta, end[1] + dy / l * delta]) + sign * p.ramp_grade * delta
            };
            let last = l.points.len() - 1;
            let ends = [
                (l.points[0], floor_at(l.points[0], l.points[1], -1.0)),
                (l.points[last], floor_at(l.points[last], l.points[last - 1], 1.0)),
            ];
            for (end, ze) in ends {
                let best = (0..fin.len())
                    .filter(|&k| matches!(r.diag.kind[k], Kind::Tread | Kind::Quay))
                    .filter(|&k| (pos(st, k)[0] - end[0]).hypot(pos(st, k)[1] - end[1]) <= near)
                    .map(|k| (fin[k] - ze).abs())
                    .fold(f64::INFINITY, f64::min);
                // 0.05 m x 1.05, set once: the nearest tread sample stands up to ~2 m from the riser's foot, down
                // the tread's 0.02 cross-fall.
                assert!(best <= 0.05 * 1.05, "{name}: ramp of riser {} end {end:?} misses its tread by {best}", l.riser);
            }
        }
    }
}

/// Ramps test 3: ramps only cut (the target never rises), touch nothing else, and leave the weights bit for bit.
#[test]
fn ramps_only_cut_and_keep_the_weights() {
    let (on, off) = (straight_run(), straight_off());
    assert_eq!(on.stamp.dims, off.stamp.dims);
    let mut cut = 0usize;
    for k in 0..on.stamp.heights.len() {
        assert_eq!(on.stamp.weights[k].to_bits(), off.stamp.weights[k].to_bits(), "weight {k}");
        let (a, b) = (on.stamp.heights[k], off.stamp.heights[k]);
        if on.diag.kind[k] == Kind::Ramp {
            assert!(a <= b, "ramp sample {k} at {a} above {b}");
            cut += 1;
        } else {
            assert_eq!(a.to_bits(), b.to_bits(), "sample {k} changed outside the ramps");
            assert_eq!(on.diag.kind[k], off.diag.kind[k]);
        }
    }
    assert!(cut > 0, "no ramp cut");
    assert!(off.report.lines.iter().all(|l| l.kind != LineKind::Ramp));
}

/// Measurement on a real reach exported by the settlement tool's site exporter, if present: cut/fill per bank and
/// the share of dry ground under 0.12 within 300 m of the water before and after.
#[test]
#[ignore = "measurement: valley_terrace on target/worldgen/sites/balmora-x10s (cut/fill, grade share, run time)"]
#[allow(clippy::too_many_lines)]
fn measure_real_reach() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../target/worldgen/sites/balmora-x10s");
    let Ok(text) = std::fs::read_to_string(dir.join("site.json")) else {
        println!("no site at {}", dir.display());
        return;
    };
    let site: serde_json::Value = serde_json::from_str(&text).expect("site.json");
    let f = |k: &str, i: usize| site[k][i].as_f64().expect("site field");
    let (ox, oy, cell) = (f("origin_m", 0), f("origin_m", 1), site["cell_m"].as_f64().expect("cell_m"));
    let (w, h) = (f("dims", 0) as usize, f("dims", 1) as usize);
    let sea = site["water_level_m"].as_f64().expect("water_level_m");
    let centre = [f("centre_m", 0), f("centre_m", 1)];
    let raw = std::fs::read(dir.join("heights.f32")).expect("heights.f32");
    let z: Vec<f32> = raw.as_chunks::<4>().0.iter().map(|b| f32::from_le_bytes(*b)).collect();
    assert_eq!(z.len(), w * h);
    let guide = move |p: [f64; 2]| {
        let u = ((p[0] - ox) / cell).clamp(0.0, (w - 1) as f64);
        let v = ((oy - p[1]) / cell).clamp(0.0, (h - 1) as f64);
        let (i, j) = ((u as usize).min(w - 2), (v as usize).min(h - 2));
        let (tx, ty) = (u - i as f64, v - j as f64);
        let at = |a: usize, b: usize| f64::from(z[b * w + a]);
        let top = at(i, j) + (at(i + 1, j) - at(i, j)) * tx;
        let bot = at(i, j + 1) + (at(i + 1, j + 1) - at(i, j + 1)) * tx;
        top + (bot - top) * ty
    };
    for (reach, core) in [(600.0, 300.0)] {
        let inp = ValleyInput {
            name: "valley".into(),
            guide: &guide,
            sea_level_m: sea,
            centre_m: centre,
            reach_m: reach,
            core_radius_m: core,
            seed: 1,
            street_grade: 0.12,
            share_within_m: 300.0,
        };
        let t0 = std::time::Instant::now();
        let r = valley_terrace(&inp, &ValleyParams::defaults()).expect("operator");
        let dt = t0.elapsed();
        let rep = &r.report;
        println!(
            "reach {reach} m core {core} m: grid {}x{} in {:.2} s; cut {:.3} M m3, fill {:.3} M m3; tread share {:.3}; \
             under {} within {} m: {:.3} -> {:.3}",
            r.stamp.dims[0],
            r.stamp.dims[1],
            dt.as_secs_f64(),
            rep.cut_m3 / 1e6,
            rep.fill_m3 / 1e6,
            rep.tread_share,
            rep.street_grade,
            rep.share_within_m,
            rep.grade_share_before,
            rep.grade_share_after
        );
        for b in &rep.banks {
            println!(
                "  bank {} centroid ({:.0}, {:.0}) rel. centre ({:+.0}, {:+.0}): cut {:.3} M m3 fill {:.3} M m3 area {:.1} ha",
                b.id,
                b.centroid_m[0],
                b.centroid_m[1],
                b.centroid_m[0] - centre[0],
                b.centroid_m[1] - centre[1],
                b.cut_m3 / 1e6,
                b.fill_m3 / 1e6,
                b.area_m2 / 1e4
            );
        }
        let mut per = BTreeMap::new();
        for l in &rep.lines {
            let e = per.entry(format!("{:?}", l.kind)).or_insert((0usize, 0.0f64));
            e.0 += 1;
            e.1 += l.points.windows(2).map(|p| (p[1][0] - p[0][0]).hypot(p[1][1] - p[0][1])).sum::<f64>();
        }
        let mut kinds = BTreeMap::new();
        for k in &r.diag.kind {
            *kinds.entry(format!("{k:?}")).or_insert(0usize) += 1;
        }
        let fin = final_surface(&r.stamp, &r.diag.guide_m);
        let mut fill_by: BTreeMap<String, f64> = BTreeMap::new();
        for (k, &f) in fin.iter().enumerate() {
            let dz = f - f64::from(r.diag.guide_m[k]);
            *fill_by.entry(format!("{:?}", r.diag.kind[k])).or_default() += dz.max(0.0) / 1e6;
        }
        println!("  fill per kind (M m3): {fill_by:?}");
        let off = valley_terrace(&inp, &ValleyParams { ramps: false, ..ValleyParams::defaults() }).expect("operator");
        println!(
            "  without ramps: under {} within {} m: {:.3} -> {:.3}",
            rep.street_grade, rep.share_within_m, off.report.grade_share_before, off.report.grade_share_after
        );
        // Ramps per bank and the street-grade flood from the quay (to every tread and the ground beyond the stair).
        let st = &r.stamp;
        let bank_of = |pt: [f64; 2]| {
            let i = ((pt[0] - st.origin_m[0]) / st.spacing_m).round() as usize;
            let j = ((st.origin_m[1] - pt[1]) / st.spacing_m).round() as usize;
            r.diag.bank[j * st.dims[0] + i]
        };
        let inside = |k: usize| {
            let q = pos(st, k);
            (q[0] - centre[0]).hypot(q[1] - centre[1]) <= reach
        };
        for (label, res) in [("ramps", &r), ("no ramps", &off)] {
            let seen = reachable(res, centre, reach);
            for b in &rep.banks {
                let ramps = res.report.lines.iter().filter(|l| l.kind == LineKind::Ramp && bank_of(l.points[0]) == b.id).count();
                let count = |kind: Kind| {
                    let ks: Vec<usize> =
                        (0..seen.len()).filter(|&k| inside(k) && res.diag.bank[k] == b.id && res.diag.kind[k] == kind).collect();
                    (ks.iter().filter(|&&k| seen[k]).count(), ks.len())
                };
                let (t, u) = (count(Kind::Tread), count(Kind::Untouched));
                println!(
                    "  {label}: bank {}: {ramps} ramps; reached from the quay: treads {}/{}, ground beyond {}/{}",
                    b.id, t.0, t.1, u.0, u.1
                );
            }
        }
        println!("  lines (count, length m): {per:?}");
        println!("  samples per kind: {kinds:?}");
    }
}
