use super::*;

/// A 1 km square: sea west of x = 60 m, a ramp rising east with a ripple north-south, region 0 west of x = 500 m and
/// 1 east of it, one exclusion disc.
struct Ramp {
    exclusion: [f64; 3],
}

impl Ramp {
    fn h(x: f64, y: f64) -> f64 {
        (x - 60.0) * 0.15 + 4.0 * (y / 40.0).sin()
    }
}

impl Terrain for Ramp {
    fn regions_in(&self, rect: [f64; 4], out: &mut Vec<u16>) {
        if rect[0] < 500.0 {
            out.push(0);
        }
        if rect[2] >= 500.0 {
            out.push(1);
        }
    }

    fn region(&self, x: f64, y: f64) -> Option<u16> {
        ((0.0..1000.0).contains(&x) && (0.0..1000.0).contains(&y)).then_some(u16::from(x >= 500.0))
    }

    fn site(&self, x: f64, y: f64) -> Option<Site> {
        self.region(x, y)?;
        let (gx, gy) = (0.15, 0.1 * (y / 40.0).cos());
        let n = norm3([-gx as f32, -gy as f32, 1.0]);
        Some(Site { slope_deg: f64::hypot(gx, gy).atan().to_degrees() as f32, band_m: Self::h(x, y) as f32, normal: n })
    }

    fn excluded(&self, x: f64, y: f64) -> bool {
        (x - self.exclusion[0]).hypot(y - self.exclusion[1]) < self.exclusion[2]
    }

    fn height(&self, x: f64, y: f64) -> f64 {
        Self::h(x, y)
    }
}

fn terrain() -> Ramp {
    Ramp { exclusion: [700.0, 600.0, 120.0] }
}

fn prm() -> Params {
    Params { tile_m: 64.0, census_step_m: 4.0, ..Params::default() }
}

fn entry(species: u32, density_m2: f64, prm: &Params) -> Entry {
    Entry {
        species,
        count: 100,
        density_m2,
        occupancy: vec![1.0; prm.classes()],
        max_slope_deg: 30.0,
        scale: [0.8, 1.2],
        align: 0.5,
        tilt_jitter_deg: 3.0,
        offset_m: -0.2,
    }
}

/// Tree (10 m) in both regions, shrub (1 m) east, rock (group 1) west.
fn table(prm: &Params) -> Table {
    let sp = |name: &str, size_m, group| Species { name: name.into(), size_m, group };
    let mut tree_east = entry(0, 0.004, prm);
    // The tree avoids the lowest band in the east.
    for c in 0..prm.classes() {
        if c % prm.bands == 0 {
            tree_east.occupancy[c] = 0.0;
        }
    }
    tree_east.occupancy[prm.class(9.0, 100.0)] = 3.0;
    Table {
        species: vec![sp("tree", 10.0, 0), sp("shrub", 1.0, 0), sp("rock", 3.0, 1)],
        regions: vec![vec![entry(0, 0.002, prm), entry(2, 0.001, prm)], vec![tree_east, entry(1, 0.02, prm)]],
    }
}

const RECT: [f64; 4] = [0.0, 0.0, 1000.0, 1000.0];

fn full(table: &Table, prm: &Params) -> (Vec<Vec<f64>>, f64) {
    let area = area_census(&terrain(), prm, RECT, table.regions.len());
    let exp = expected(table, prm, &area);
    let total = exp.iter().flatten().sum();
    (exp, total)
}

/// An item as its bit patterns (bitwise comparisons).
type Bits = (u32, u16, [u64; 3], u32, u32, [u32; 3]);

fn bits(items: &[Item]) -> Vec<Bits> {
    items
        .iter()
        .map(|i| (i.species, i.region, i.pos.map(f64::to_bits), i.yaw.to_bits(), i.scale.to_bits(), i.up.map(f32::to_bits)))
        .collect()
}

#[test]
fn a_tile_alone_equals_the_same_tile_of_a_whole_run() {
    let prm = prm();
    let table = table(&prm);
    let s = Scatter::new(&table, vec![1.0; 3], prm);
    let all = s.run(&terrain(), RECT);
    assert!(all.len() > 1000, "{} items", all.len());
    for c in [(3, 4), (7, 9), (0, 0), (15, 15)] {
        let mut alone = Vec::new();
        s.tile(&terrain(), c, &mut alone);
        let t = prm.tile_m;
        let inside: Vec<Item> = all
            .iter()
            .copied()
            .filter(|i| (i.pos[0] / t).floor() as i32 == c.0 && (i.pos[1] / t).floor() as i32 == c.1)
            .collect();
        assert!(!alone.is_empty() || c == (0, 0), "tile {c:?} empty");
        assert_eq!(bits(&alone), bits(&inside), "tile {c:?}");
    }
}

#[test]
fn order_and_threads_do_not_change_the_result() {
    let prm = prm();
    let table = table(&prm);
    let s = Scatter::new(&table, vec![1.0; 3], prm);
    let run_on = |threads| {
        rayon::ThreadPoolBuilder::new().num_threads(threads).build().unwrap().install(|| s.run(&terrain(), RECT))
    };
    let one = run_on(1);
    assert_eq!(bits(&one), bits(&run_on(4)));
    // Tiles generated in reverse order, then put back in tile order.
    let mut rev: Vec<((i32, i32), Vec<Item>)> = s
        .tiles(RECT)
        .into_iter()
        .rev()
        .map(|c| {
            let mut v = Vec::new();
            s.tile(&terrain(), c, &mut v);
            (c, v)
        })
        .collect();
    rev.reverse();
    let rev: Vec<Item> = rev.into_iter().flat_map(|(_, v)| v).collect();
    assert_eq!(bits(&one), bits(&rev));
}

#[test]
fn nothing_inside_an_exclusion_on_water_or_too_steep() {
    let prm = prm();
    let table = table(&prm);
    let t = terrain();
    let items = Scatter::new(&table, vec![1.0; 3], prm).run(&t, RECT);
    assert!(!items.is_empty());
    for i in &items {
        assert!(!t.excluded(i.pos[0], i.pos[1]), "{i:?} inside the exclusion");
        assert!(Ramp::h(i.pos[0], i.pos[1]) >= prm.sea_level_m + prm.water_margin_m, "{i:?} on water");
        assert!(i.pos.iter().all(|v| v.is_finite()) && i.up.iter().all(|v| v.is_finite()));
        let up_len = i.up.iter().map(|v| v * v).sum::<f32>().sqrt();
        assert!((up_len - 1.0).abs() < 1e-4 && i.up[2] > 0.8, "{i:?}");
        // The shrub lives east only, the rock west only.
        assert!(i.species != 1 || i.pos[0] >= 500.0);
        assert!(i.species != 2 || i.pos[0] < 500.0);
        assert!((0.8..=1.2).contains(&i.scale));
    }
    // The east tree avoids the lowest band: none east below band_base_m.
    assert!(!items.iter().any(|i| i.species == 0 && i.region == 1 && Ramp::h(i.pos[0], i.pos[1]) < f64::from(prm.band_base_m)));
}

#[test]
fn full_density_matches_the_census_and_the_budget_its_total() {
    let mut prm = prm();
    let table = table(&prm);
    let (exp, total) = full(&table, &prm);
    let s = Scatter::new(&table, budget_keep(&table, &prm, &exp), prm);
    let n = s.run(&terrain(), RECT).len() as f64;
    assert!((n / total - 1.0).abs() < 0.05, "full density: {n} items, {total:.0} expected");
    for share in [0.1, 0.4] {
        prm.budget = (total * share).round();
        let keep = budget_keep(&table, &prm, &exp);
        let n = Scatter::new(&table, keep, prm).run(&terrain(), RECT).len() as f64;
        assert!((n / prm.budget - 1.0).abs() < 0.05, "budget {}: {n} items", prm.budget);
    }
}

#[test]
fn the_budget_keeps_large_species_first_and_nests() {
    let mut prm = prm();
    let table = table(&prm);
    let (exp, total) = full(&table, &prm);
    prm.budget = total * 0.2;
    let keep = budget_keep(&table, &prm, &exp);
    assert!(keep[0] > keep[1], "tree {} vs shrub {}", keep[0], keep[1]);
    assert!(keep.iter().all(|&k| (0.0..=1.0).contains(&k)));
    let small = Scatter::new(&table, keep, prm).run(&terrain(), RECT);
    prm.budget = total * 0.5;
    let big = Scatter::new(&table, budget_keep(&table, &prm, &exp), prm).run(&terrain(), RECT);
    let big: std::collections::HashSet<_> = bits(&big).into_iter().collect();
    assert!(bits(&small).iter().all(|i| big.contains(i)), "the smaller budget's items are all in the larger one");
}

#[test]
fn learner_densities_occupancy_and_pose() {
    let prm = Params::default();
    let mut l = Learner::new(prm, 2, 4.0);
    // Region 0: 100 flat samples at 3 m (band 1) and 100 at 12 deg; region 1 has no land.
    for _ in 0..100 {
        l.add_land(0, 2.0, 3.0);
        l.add_land(0, 12.0, 3.0);
    }
    let tilted = norm3([0.0, -(20f32.to_radians().sin()), 20f32.to_radians().cos()]);
    let obs = |slope_deg, up, normal, scale| Observed { species: 0, region: 0, slope_deg, band_m: 3.0, scale, up, normal, offset_m: -0.5 * scale };
    for k in 0..30 {
        l.add(obs(2.0, [0.0, 0.0, 1.0], [0.0, 0.0, 1.0], 1.0 + 0.01 * k as f32));
    }
    for _ in 0..10 {
        // On the slope, standing along the normal.
        l.add(obs(12.0, tilted, tilted, 1.0));
    }
    l.add(Observed { region: 1, ..obs(2.0, [0.0, 0.0, 1.0], [0.0, 0.0, 1.0], 1.0) });
    let t = l.finish(vec![Species { name: "a".into(), size_m: 2.0, group: 0 }]);
    assert!(t.regions[1].is_empty(), "no land, no entry");
    let e = &t.regions[0][0];
    assert_eq!(e.count, 40);
    assert!((e.density_m2 - 40.0 / 800.0).abs() < 1e-12);
    let flat = e.occupancy[prm.class(2.0, 3.0)];
    let steep = e.occupancy[prm.class(12.0, 3.0)];
    assert!((flat - (31.0 / 42.0) / 0.5).abs() < 1e-5, "flat {flat}");
    assert!((steep - (11.0 / 42.0) / 0.5).abs() < 1e-5, "steep {steep}");
    assert!(e.occupancy[prm.class(2.0, 50.0)].abs() < 1e-9, "no land in that band");
    assert!(e.occupancy[prm.class(16.0, 3.0)].abs() < 1e-9, "steeper than seen");
    assert!((e.max_slope_deg - 12.0).abs() < 1e-6);
    assert!((e.scale[0] - 1.0).abs() < 1e-6 && (e.scale[1] - 1.29).abs() < 1e-5);
    assert!((e.align - 1.0).abs() < 1e-4, "align {}", e.align);
    assert!(e.tilt_jitter_deg < 0.1);
    assert!((e.offset_m + 0.5).abs() < 1e-6);
}

#[test]
fn classes_and_bands() {
    let p = Params::default();
    assert_eq!(p.band(-3.0), 0);
    assert_eq!(p.band(1.9), 0);
    assert_eq!(p.band(2.0), 1);
    assert_eq!(p.band(3.9), 1);
    assert_eq!(p.band(4.0), 2);
    assert_eq!(p.band(1.0e6), p.bands - 1);
    assert_eq!(p.slope_class(4.9), 0);
    assert_eq!(p.slope_class(80.0), p.slope_classes - 1);
}
