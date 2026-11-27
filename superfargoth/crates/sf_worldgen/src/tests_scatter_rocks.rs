#![allow(clippy::cast_precision_loss)] // counts of test fixtures
#![allow(clippy::float_cmp)] // exact comparisons are the point of these tests
//! WG-E3b: talus candidates. On one straight 60 m scarp of one unit (spec "Tests"): the Pareto size slope of log
//! N(>s) is D within 10 %; size and distance from the scarp foot are rank-correlated with sorting on and not without
//! it; every block lies below the shadow line from its source, on loose ground; a tile built alone matches a larger
//! build bit for bit.

use super::*;

/// Scarp height, m.
const TOP_M: f64 = 60.0;
/// Scarp face angle, deg.
const FACE_DEG: f64 = 80.0;
/// Scarp length along y, m.
const LENGTH_M: f64 = 2000.0;

/// A plateau at `TOP_M` for x < 0, an 80 deg face down to 0 at `x = w`, flat beyond; downslope east; all one unit.
struct Scarp {
    w: f64,
}

impl Scarp {
    fn new() -> Self {
        Self { w: TOP_M / FACE_DEG.to_radians().tan() }
    }
    fn on_face(&self, x: f64, y: f64) -> bool {
        (0.0..self.w).contains(&x) && (0.0..LENGTH_M).contains(&y)
    }
}

impl RockGround for Scarp {
    fn height_m(&self, x: f64, _y: f64) -> f64 {
        if x < 0.0 {
            TOP_M
        } else if x < self.w {
            TOP_M - x * FACE_DEG.to_radians().tan()
        } else {
            0.0
        }
    }
    fn slope_aspect_deg(&self, x: f64, y: f64) -> (f64, f64) {
        (if self.on_face(x, y) { FACE_DEG } else { 0.0 }, 90.0)
    }
    fn cliff(&self, _x: f64, _y: f64) -> bool {
        false
    }
    fn unit(&self, _x: f64, _y: f64) -> Option<&str> {
        Some("basalt")
    }
}

/// A file with one talus row of `size_min..size_max` m and the unit `basalt` (rockfall 1).
fn file(size_min: f64, size_max: f64) -> SpeciesFile {
    let src = format!(
        r#"
[columns]
size_min_m = {{ unit = "m", min = 0.2, max = 40, source = "Guess" }}
size_max_m = {{ unit = "m", min = 0.2, max = 40, source = "Guess" }}

[rocks.talus]
size_min_m = {size_min}
size_max_m = {size_max}

[rock_units.basalt]
talus = [{{ kind = "class", class = "block", weight = 1 }}]
"#
    );
    SpeciesFile::parse_toml(&src).unwrap()
}

/// Dense sources (500/ha, 40 blocks each) so the statistics hold on one scarp.
fn values(size_dim: f64, sort_beta: f64) -> RockValues {
    RockValues { size_dim, sort_beta, talus_parent_per_ha: 500.0, talus_child_mean: 40.0, ..RockValues::default() }
}

/// Every block of the fixture.
const ALL: [f64; 4] = [-200.0, -200.0, 400.0, LENGTH_M + 200.0];

fn talus(file: &SpeciesFile, values: RockValues, rect: [f64; 4]) -> Vec<RockCandidate> {
    RockRules { file, values, seed: 7, tile_m: 256.0 }.talus_candidates(&Scarp::new(), rect)
}

/// Least-squares slope of ln N(>s) on ln s at nine log-spaced sizes from `s_min` to `8 s_min`.
fn ccdf_slope(sizes: &[f64], s_min: f64) -> f64 {
    let pts: Vec<(f64, f64)> = (0..=8)
        .map(|k| {
            let s = s_min * 8f64.powf(f64::from(k) / 8.0);
            let n = sizes.iter().filter(|&&x| x > s).count();
            (s.ln(), (n as f64).ln())
        })
        .collect();
    let n = pts.len() as f64;
    let (mx, my) = (pts.iter().map(|p| p.0).sum::<f64>() / n, pts.iter().map(|p| p.1).sum::<f64>() / n);
    let sxy: f64 = pts.iter().map(|p| (p.0 - mx) * (p.1 - my)).sum();
    let sxx: f64 = pts.iter().map(|p| (p.0 - mx).powi(2)).sum();
    sxy / sxx
}

fn ranks(v: &[f64]) -> Vec<f64> {
    let mut idx: Vec<usize> = (0..v.len()).collect();
    idx.sort_by(|&a, &b| v[a].total_cmp(&v[b]));
    let mut r = vec![0.0; v.len()];
    for (k, &i) in idx.iter().enumerate() {
        r[i] = k as f64;
    }
    r
}

/// Spearman's rank correlation (continuous values: no ties).
fn spearman(a: &[f64], b: &[f64]) -> f64 {
    let (ra, rb) = (ranks(a), ranks(b));
    let n = ra.len() as f64;
    let m = (n - 1.0) / 2.0;
    let c: f64 = ra.iter().zip(&rb).map(|(x, y)| (x - m) * (y - m)).sum();
    let v: f64 = ra.iter().map(|x| (x - m).powi(2)).sum();
    c / v
}

#[test]
fn talus_sizes_follow_the_pareto_slope() {
    // The law itself, by inversion: bounds exact, monotone.
    assert_eq!(pareto_size(0.4, 12.0, 2.0, 0.0), 0.4);
    assert!((pareto_size(0.4, 12.0, 2.0, 1.0 - f64::EPSILON) - 12.0).abs() < 1e-6);
    assert_eq!(pareto_size(3.0, 3.0, 2.0, 0.7), 3.0);
    assert!(pareto_size(0.4, 12.0, 2.0, 0.3) < pareto_size(0.4, 12.0, 2.0, 0.6));
    // A `[params]` entry reaches the values.
    let src = r#"
[params]
"scatter.rock.size_dim" = { v = 1.6, unit = "ratio", min = 1, max = 3.5, source = "Guess" }
"#;
    let f = SpeciesFile::parse_toml(src).unwrap();
    assert_eq!(RockValues::from_declared(&f.declared).unwrap().size_dim, 1.6);
    assert_eq!(RockValues::from_declared(&file(0.4, 12.0).declared).unwrap(), RockValues::default());
    // Kept blocks: slope of log N(>s) within 10 % of D (no sorting, so the cuts do not depend on size).
    let f = file(0.4, 40.0);
    for d in [1.6, 2.0, 3.0] {
        let blocks = talus(&f, values(d, 0.0), ALL);
        assert!(blocks.len() > 10_000, "{} blocks", blocks.len());
        let sizes: Vec<f64> = blocks.iter().map(|b| b.size_m).collect();
        assert!(sizes.iter().all(|s| (0.4..=40.0).contains(s)));
        let slope = ccdf_slope(&sizes, 0.4);
        println!("D {d}: {} blocks, fitted slope {slope:.3}", blocks.len());
        assert!((-slope - d).abs() < 0.1 * d, "D {d}: slope {slope}");
    }
}

#[test]
fn talus_big_blocks_roll_further_with_sorting() {
    let f = file(0.4, 12.0);
    let w = Scarp::new().w;
    let rho = |beta: f64| {
        let blocks = talus(&f, values(2.0, beta), ALL);
        let s: Vec<f64> = blocks.iter().map(|b| b.size_m).collect();
        let dist: Vec<f64> = blocks.iter().map(|b| b.pos[0] - w).collect();
        let r = spearman(&s, &dist);
        println!("beta {beta}: {} blocks, Spearman(size, distance from the foot) {r:.3}", blocks.len());
        r
    };
    let (r0, r4, r10) = (rho(0.0), rho(0.4), rho(1.0));
    // Sanity bands (spec "As built" WG-E3b: the exponential run-out keeps the sorting a modest tendency).
    assert!(r0.abs() < 0.03, "beta 0: {r0}");
    assert!(r4 > 0.08, "beta 0.4: {r4}");
    assert!(r10 > r4 + 0.05, "beta 1: {r10} vs beta 0.4: {r4}");
}

#[test]
fn talus_lies_below_the_shadow_line() {
    let g = Scarp::new();
    let f = file(0.4, 12.0);
    let v = values(2.0, 0.4);
    let blocks = talus(&f, v, ALL);
    assert!(blocks.len() > 10_000);
    let tan = v.shadow_angle_deg.to_radians().tan();
    let mut far = 0.0f64;
    for b in &blocks {
        let [sx, sy] = b.source;
        assert!(g.on_face(sx, sy), "source {:?} off the scarp", b.source);
        let x = (b.pos[0] - sx).hypot(b.pos[1] - sy);
        assert!(x <= v.reach_max_m);
        assert!(g.height_m(sx, sy) - g.height_m(b.pos[0], b.pos[1]) >= x * tan, "block {:?} above the line", b.key);
        assert!(g.slope_aspect_deg(b.pos[0], b.pos[1]).0 <= v.loose_slope_max_deg);
        // Downslope +- the spread: east of its source, within the cone.
        let ang = (b.pos[1] - sy).atan2(b.pos[0] - sx).to_degrees().abs();
        assert!(ang <= v.talus_spread_deg + 1e-9, "angle {ang}");
        far = far.max(b.pos[0] - g.w);
    }
    // Nothing past the shadow line from the scarp top.
    assert!(far <= TOP_M / tan, "farthest {far}");
    // Density falls away from the scarp: the first 20 m below the foot hold more than the next 20 m.
    let band = |a: f64, b: f64| blocks.iter().filter(|c| (a..b).contains(&(c.pos[0] - g.w))).count();
    assert!(band(0.0, 20.0) > band(20.0, 40.0));
    // A unit without a table, or a file without a talus row, sheds nothing.
    let none = SpeciesFile::parse_toml("[rock_units.basalt]\n").unwrap();
    assert!(talus(&none, v, ALL).is_empty());
}

#[test]
fn talus_tile_alone() {
    let f = file(0.4, 12.0);
    let v = values(2.0, 0.4);
    let all = talus(&f, v, ALL);
    for rect in [[0.0, 0.0, 256.0, 256.0], [-37.5, 900.0, 64.0, 1100.25], [8.0, -10.0, 300.0, 12.0]] {
        let alone = talus(&f, v, rect);
        let inside: Vec<RockCandidate> = all
            .iter()
            .filter(|c| c.pos[0] >= rect[0] && c.pos[0] < rect[2] && c.pos[1] >= rect[1] && c.pos[1] < rect[3])
            .copied()
            .collect();
        assert!(!alone.is_empty());
        assert_eq!(alone, inside, "rect {rect:?}");
    }
}

// WG-E3c: outcrop clusters and field stones on a plain: basalt (outcrops, field stones) for y < 1000, tuff (neither)
// beyond; flat for x < 800, a 50 deg slope (past the loose limit) from x = 800.

/// Where the plain turns steep, m.
const STEEP_X: f64 = 800.0;
/// Where basalt gives way to tuff, m.
const TUFF_Y: f64 = 1000.0;

struct Plain;

impl RockGround for Plain {
    fn height_m(&self, _x: f64, _y: f64) -> f64 {
        0.0
    }
    fn slope_aspect_deg(&self, x: f64, _y: f64) -> (f64, f64) {
        (if x >= STEEP_X { 50.0 } else { 0.0 }, 90.0)
    }
    fn cliff(&self, _x: f64, _y: f64) -> bool {
        false
    }
    fn unit(&self, _x: f64, y: f64) -> Option<&str> {
        Some(if y < TUFF_Y { "basalt" } else { "tuff" })
    }
}

/// Outcrop 1-20 m and field 0.3-2 m rows; basalt outcrops and has 100 field stones/ha; tuff has neither.
fn plain_file() -> SpeciesFile {
    let src = r#"
[columns]
size_min_m = { unit = "m", min = 0.2, max = 40, source = "Guess" }
size_max_m = { unit = "m", min = 0.2, max = 40, source = "Guess" }
field_stone_per_ha = { unit = "1/ha", min = 0, max = 500, source = "Guess" }

[rocks.outcrop]
size_min_m = 1
size_max_m = 20

[rocks.field]
size_min_m = 0.3
size_max_m = 2

[rock_units.basalt]
field_stone_per_ha = 100
outcrop = [{ kind = "class", class = "block", weight = 1 }]
field = [{ kind = "class", class = "block", weight = 1 }]

[rock_units.tuff]
field_stone_per_ha = 0
"#;
    SpeciesFile::parse_toml(src).unwrap()
}

/// Dense outcrops (10/ha) so the statistics hold.
fn plain_values() -> RockValues {
    RockValues { outcrop_parent_per_ha: 10.0, ..RockValues::default() }
}

const PLAIN: [f64; 4] = [-100.0, -100.0, 1100.0, 1300.0];

fn plain_rules(file: &SpeciesFile) -> RockRules<'_> {
    RockRules { file, values: plain_values(), seed: 11, tile_m: 256.0 }
}

/// g(r) over `[r0, r1)` of the points in `[0, side)^2`, centres kept `r1` from the border (edge correction).
fn pair_correlation(pts: &[[f64; 2]], side: f64, r0: f64, r1: f64) -> f64 {
    let pts: Vec<[f64; 2]> =
        pts.iter().filter(|p| (0.0..side).contains(&p[0]) && (0.0..side).contains(&p[1])).copied().collect();
    let lambda = pts.len() as f64 / (side * side);
    let centres: Vec<&[f64; 2]> =
        pts.iter().filter(|p| p[0] >= r1 && p[0] <= side - r1 && p[1] >= r1 && p[1] <= side - r1).collect();
    let mut pairs = 0usize;
    for c in &centres {
        pairs += pts.iter().filter(|p| (r0..r1).contains(&(p[0] - c[0]).hypot(p[1] - c[1]))).count();
    }
    let ring = std::f64::consts::PI * (r1 * r1 - r0 * r0);
    pairs as f64 / (centres.len() as f64 * lambda * ring)
}

#[test]
fn outcrop_children_are_smaller_than_their_parent() {
    let f = plain_file();
    let r = plain_rules(&f);
    let v = r.values;
    let rocks = r.outcrop_candidates(&Plain, PLAIN);
    let parents: std::collections::BTreeMap<_, _> =
        rocks.iter().filter(|c| c.key.offspring == 0).map(|c| ((c.key.tile, c.key.parent), *c)).collect();
    assert!(parents.len() > 500, "{} parents", parents.len());
    let mut children = 0usize;
    for c in &rocks {
        assert_eq!(c.key.context, RockContext::Outcrop);
        // Only outcropping units; parents are exempt from the loose-slope drop, children are not.
        assert!(c.source[1] < TUFF_Y, "parent {:?} in tuff", c.source);
        if c.key.offspring == 0 {
            assert!((v.outcrop_parent_min_m..=20.0).contains(&c.size_m), "parent size {}", c.size_m);
            continue;
        }
        assert!(c.pos[0] < STEEP_X, "loose child on steep ground at {:?}", c.pos);
        let d = (c.pos[0] - c.source[0]).hypot(c.pos[1] - c.source[1]);
        assert!(d <= THOMAS_REACH_SIGMAS * v.cluster_sigma_m);
        assert!(c.size_m >= 1.0);
        if let Some(p) = parents.get(&(c.key.tile, c.key.parent)) {
            assert_eq!(p.pos, c.source);
            assert!(c.size_m <= v.child_size_ratio * p.size_m, "child {} vs parent {}", c.size_m, p.size_m);
            children += 1;
        }
    }
    println!("{} parents, {children} children checked against theirs", parents.len());
    assert!(children > 2000, "{children} children");
    assert!(rocks.iter().any(|c| c.key.offspring == 0 && c.source[0] >= STEEP_X), "a parent on steep ground");
}

#[test]
fn outcrops_and_field_stones_clump_at_sigma() {
    let f = plain_file();
    let r = plain_rules(&f);
    let v = r.values;
    let side = 700.0;
    let outcrop: Vec<[f64; 2]> = r.outcrop_candidates(&Plain, PLAIN).iter().map(|c| c.pos).collect();
    let field_c = r.field_candidates(&Plain, PLAIN);
    let field: Vec<[f64; 2]> = field_c.iter().map(|c| c.pos).collect();
    let (so, sf) = (v.cluster_sigma_m, v.field_cluster_sigma_m);
    let (go, gf) =
        (pair_correlation(&outcrop, side, 0.5 * so, so), pair_correlation(&field, side, 0.5 * sf, sf));
    println!("g(0.5-1 sigma): outcrop {go:.2}, field {gf:.2}");
    assert!(go > 1.5, "outcrop g {go}");
    assert!(gf > 1.5, "field g {gf}");
    // Field stones: none on ground past the loose slope or in a unit without them; sizes in the row's range;
    // the flat basalt count near the declared density (sanity band).
    assert!(field_c.iter().all(|c| c.pos[0] < STEEP_X && c.source[1] < TUFF_Y && (0.3..=2.0).contains(&c.size_m)));
    let n = field_c.iter().filter(|c| (0.0..STEEP_X).contains(&c.pos[0]) && (100.0..900.0).contains(&c.pos[1])).count();
    let want = 100.0 * STEEP_X * 800.0 / M2_PER_HA;
    println!("{n} field stones on {want} expected");
    assert!((n as f64 - want).abs() < 0.1 * want, "{n} vs {want}");
    // No row, no rocks.
    let none = SpeciesFile::parse_toml("[rock_units.basalt]\n").unwrap();
    let rn = RockRules { file: &none, values: v, seed: 11, tile_m: 256.0 };
    assert!(rn.outcrop_candidates(&Plain, PLAIN).is_empty() && rn.field_candidates(&Plain, PLAIN).is_empty());
}

#[test]
fn outcrops_and_field_stones_tile_alone() {
    let f = plain_file();
    let r = plain_rules(&f);
    for build in [RockRules::outcrop_candidates, RockRules::field_candidates] {
        let all = build(&r, &Plain, PLAIN);
        for rect in [[0.0, 0.0, 256.0, 256.0], [-37.5, 700.0, 64.0, 1100.25], [500.0, -10.0, 900.0, 12.0]] {
            let alone = build(&r, &Plain, rect);
            let inside: Vec<RockCandidate> = all
                .iter()
                .filter(|c| c.pos[0] >= rect[0] && c.pos[0] < rect[2] && c.pos[1] >= rect[1] && c.pos[1] < rect[3])
                .copied()
                .collect();
            assert!(!alone.is_empty(), "rect {rect:?}");
            assert_eq!(alone, inside, "rect {rect:?}");
        }
    }
}
