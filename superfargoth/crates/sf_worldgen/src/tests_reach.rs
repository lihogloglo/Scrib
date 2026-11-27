//! Tests of the reachability flood core (WG-V1.1; spec `sites.md` "Tests").

use super::*;

const C: f64 = 10.0;

fn values(max_walk_deg: f64) -> ReachValues {
    ReachValues { cell_m: C, max_walk_deg, ..ReachValues::default() }
}

/// Ground from a cell function `(cx, cy) -> (height, water)`.
fn ground(w: usize, h: usize, f: impl Fn(usize, usize) -> (f64, bool) + Sync) -> ReachGround {
    ReachGround::sample([0.0, 0.0], w, h, C, |x, y| f((x / C).round() as usize, (-y / C).round() as usize)).unwrap()
}

/// 30 x 12 cells: mainland x 0-7, strait x 8-11 with a bridge deck on row 5, island x 12-19, sea x 20-29 with an
/// islet at x 24-25, y 5-6. Flat, water surface at the land's height.
fn archipelago(bridge: bool) -> ReachGround {
    ground(30, 12, |x, y| {
        let strait = (8..=11).contains(&x) && !(bridge && y == 5);
        let islet = (24..=25).contains(&x) && (5..=6).contains(&y);
        let sea = x >= 20 && !islet;
        (0.0, strait || sea)
    })
}

fn is_islet(i: usize) -> bool {
    let (x, y) = (i % 30, i / 30);
    (24..=25).contains(&x) && (5..=6).contains(&y)
}

#[test]
fn island_bridge_islet_reached_sets_exact() {
    let g = archipelago(true);
    let m = ReachMap::flood(&g, &[0], &ReachValues::default()).unwrap();
    // On foot: every dry cell (mainland, deck, island) and no water; the islet has no dry path.
    let expected: Vec<bool> = (0..g.height.len()).map(|i| !g.water[i] && !is_islet(i)).collect();
    let foot: Vec<bool> = (0..g.height.len()).map(|i| m.foot.reached(i)).collect();
    assert_eq!(foot, expected);
    // Swimming: everything; water cells swam, the islet too; the island is cheaper by the bridge than across.
    assert!((0..g.height.len()).all(|i| m.swim.reached(i)));
    for i in 0..g.height.len() {
        assert_eq!(m.swim.swam[i], g.water[i] || is_islet(i), "cell {i}");
    }
    let islet = 5 * 30 + 24;
    assert_eq!(m.best(islet, Reach::Foot), None);
    assert_eq!(m.best(islet, Reach::Swim).map(|b| b.1), Some(Via::Swim));
    assert_eq!(m.best(islet, Reach::Any).map(|b| b.1), Some(Via::Swim));
    assert_eq!(m.best(5 * 30 + 15, Reach::Foot).map(|b| b.1), Some(Via::Foot));

    // No bridge: the island is off the foot map and reached swimming only.
    let g = archipelago(false);
    let m = ReachMap::flood(&g, &[0], &ReachValues::default()).unwrap();
    let island = 12;
    assert!(!m.foot.reached(island));
    assert_eq!(m.best(island, Reach::Swim).map(|b| b.1), Some(Via::Swim));
    assert!((0..8).all(|x| m.foot.reached(x) && !m.swim.swam[x]));
}

#[test]
fn step_costs_exact() {
    let g = archipelago(false);
    let v = ReachValues { cell_m: C, ..ReachValues::default() };
    // Source on the shore (7, 0): the flat step east into the strait swims at swim_factor x flat walking.
    let m = ReachMap::flood(&g, &[7], &v).unwrap();
    let flat = C / tobler_ms(0.0);
    assert_eq!(m.foot.cost_us[6], (flat * 1e6).round() as u64);
    assert_eq!(m.swim.cost_us[8], (flat * v.swim_factor * 1e6).round() as u64);
    assert_eq!(m.foot.cost_us[8], UNREACHED);
    // Diagonal flat step: sqrt 2 x the distance.
    assert_eq!(m.foot.cost_us[30 + 6], (C * std::f64::consts::SQRT_2 / tobler_ms(0.0) * 1e6).round() as u64);
    // Tobler: fastest just downhill, uphill slower than the same downhill.
    assert!((tobler_ms(-0.05) - 6.0 / 3.6).abs() < 1e-12);
    assert!(tobler_ms(0.3) < tobler_ms(-0.3));
}

/// 21 x 9 cells, flat plains either side of a ridge x 6-14 whose two faces slope at `deg`; peak on x = 10.
fn ridge(deg: f64) -> ReachGround {
    let t = deg.to_radians().tan();
    ground(21, 9, |x, _| ((4.0 - (x as f64 - 10.0).abs()).max(0.0) * C * t, false))
}

#[test]
fn ridge_47_deg_is_a_wall_45_passes() {
    let west: Vec<usize> = (0..9).map(|y| y * 21).collect();
    let g = ridge(47.0);
    let m = ReachMap::flood(&g, &west, &ReachValues::default()).unwrap();
    // The west plain up to the foot of the face (x 6), nothing on the face nor beyond; diagonals don't leak.
    let reached: Vec<bool> = (0..g.height.len()).map(|i| m.foot.reached(i)).collect();
    let expected: Vec<bool> = (0..g.height.len()).map(|i| i % 21 <= 6).collect();
    assert_eq!(reached, expected);
    assert_eq!(m.swim, m.foot, "no water: both floods agree");

    let g = ridge(45.0);
    let m = ReachMap::flood(&g, &west, &ReachValues::default()).unwrap();
    assert!((0..g.height.len()).all(|i| m.foot.reached(i)));
    // Over the ridge costs more than the flat plain's width.
    assert!(m.foot.cost_us[20] > (20.0 * C / tobler_ms(0.0) * 1e6) as u64);
}

#[test]
fn a_face_is_a_wall_across_it_too() {
    // A uniform 47 deg face (rows rise southwards); a source on it can step nowhere, not even along the contour.
    let t = 47f64.to_radians().tan();
    let g = ground(9, 9, |_, y| (y as f64 * C * t, false));
    let m = ReachMap::flood(&g, &[4 * 9 + 4], &values(46.0)).unwrap();
    assert_eq!((0..81).filter(|&i| m.foot.reached(i)).count(), 1);
    let m = ReachMap::flood(&g, &[4 * 9 + 4], &values(48.0)).unwrap();
    assert_eq!((0..81).filter(|&i| m.foot.reached(i)).count(), 81);
}

/// A hilly field with lakes, from a hash: deterministic, irregular.
fn hills(x: f64, y: f64) -> (f64, bool) {
    let hash = |a: i64, b: i64| {
        let mut z = (a as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15) ^ (b as u64).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z ^= z >> 31;
        z = z.wrapping_mul(0x94d0_49bb_1331_11eb);
        (z >> 11) as f64 / (1u64 << 53) as f64
    };
    let (cx, cy) = ((x / C) as i64, (y / C) as i64);
    let h = 40.0 * (x / 170.0).sin() * (y / 130.0).cos() + 8.0 * hash(cx, cy);
    (h.max(-12.0), h < -12.0)
}

#[test]
fn same_result_twice_any_thread_count_any_source_order() {
    let run = |threads: usize, sources: &[usize]| {
        rayon::ThreadPoolBuilder::new().num_threads(threads).build().unwrap().install(|| {
            let g = ReachGround::sample([0.0, 0.0], 80, 60, C, hills).unwrap();
            let m = ReachMap::flood(&g, sources, &ReachValues::default()).unwrap();
            (g, m)
        })
    };
    let (g1, m1) = run(1, &[0, 2401, 4799, 2401]);
    let (g4, m4) = run(4, &[4799, 2401, 0]);
    assert_eq!(g1, g4);
    assert_eq!(m1, m4);
    assert_eq!(run(4, &[0, 2401, 4799]).1, m4);
    // The field is irregular enough to test something: walls and water both bite.
    let foot = (0..g1.height.len()).filter(|&i| m1.foot.reached(i)).count();
    let swam = (0..g1.height.len()).filter(|&i| m1.swim.swam[i]).count();
    assert!(foot > 0 && foot < g1.height.len() && swam > 0, "foot {foot}, swam {swam}");
}

#[test]
fn frame_and_checks() {
    let g = ReachGround::sample([100.0, 500.0], 4, 3, C, |_, _| (0.0, false)).unwrap();
    let [cx, cy] = g.centre(5);
    assert!((cx - 110.0).abs() < 1e-9 && (cy - 490.0).abs() < 1e-9);
    assert_eq!(g.cell_at(110.0, 490.0), Some(5));
    assert_eq!(g.cell_at(114.9, 485.1), Some(5));
    assert_eq!(g.cell_at(94.0, 500.0), None);
    assert_eq!(g.cell_at(100.0, 510.0), None);
    let e = ReachMap::flood(&g, &[12], &ReachValues::default()).unwrap_err();
    assert!(e.to_string().contains("source cell 12 off"), "{e}");
    assert!(ReachGround::sample([0.0, 0.0], 0, 3, C, |_, _| (0.0, false)).is_err());
    assert!(ReachGround::sample([0.0, 0.0], 2, 2, C, |_, _| (f64::NAN, false)).is_err());
    let mut d = declared();
    d.iter_mut().find(|p| p.name == "reach.swim_factor").unwrap().v = 0.5;
    assert!(ReachValues::from_declared(&d).unwrap_err().to_string().contains("reach.swim_factor"));
}
