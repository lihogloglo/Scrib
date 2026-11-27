#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)] // counts of test fixtures
//! WG-E2h: the v2 rank budget, and the v1 scatter tests (`tests_scatter.rs`) ported to the v2 passes with halo
//! tiles. On a 512 m square of four tiles (a slope ramp north, the sea on the west edge, a region prior that keeps
//! the shrub out of the east half): every tile built alone, in any order, on any thread count, equals the whole
//! square's build; nothing is too steep for its slope hat, outside its region or on the sea; the budget keeps each
//! group's share exactly, larger budgets keep supersets, and the cut is exactly "smallest `u / size^p` first".

use super::*;
use crate::scatter_fields::FieldSample;
use crate::scatter_stand::StandRules;

/// Three layers; the tree has a slope hat (full to 25 deg, gone at 35 deg).
const SPECIES: &str = r#"
[params]
"scatter.stand_clearing_threshold" = { v = 0.0, unit = "ratio", min = 0, max = 1, source = "Guess" }

[columns]
parent_per_ha = { unit = "1/ha", min = 0, max = 500, source = "Guess" }
offspring_mean = { unit = "count", min = 1, max = 60, source = "Guess" }
cluster_sigma_m = { unit = "m", min = 2, max = 40, source = "Guess" }
size_min_m = { unit = "m", min = 0.2, max = 40, source = "Guess" }
size_max_m = { unit = "m", min = 0.2, max = 40, source = "Guess" }
crown_a_m = { unit = "m", min = 0, max = 5, source = "Guess" }
shade_tolerance = { unit = "ratio", min = 0, max = 1, source = "Guess" }
footprint_ratio = { unit = "ratio", min = 0, max = 0.5, source = "Guess" }
hat_slope_deg = { unit = "deg", min = 0, max = 90, source = "Guess" }

[[species]]
name = "tree"
layer = "canopy"
size_law = "reverse_j"
parent_per_ha = 20.0
offspring_mean = 20.0
cluster_sigma_m = 8.0
footprint_ratio = 0.05
hat_slope_deg = [0, 0, 25, 35]

[[species]]
name = "shrub"
layer = "understorey"
size_law = "weibull"
size_min_m = 0.5
size_max_m = 2.0
crown_a_m = 0.3
shade_tolerance = 0.5
parent_per_ha = 40.0
offspring_mean = 20.0
cluster_sigma_m = 6.0
footprint_ratio = 0.1

[[species]]
name = "fern"
layer = "ground"
size_law = "reverse_j"
size_min_m = 0.2
size_max_m = 0.6
crown_a_m = 0.1
shade_tolerance = 0.7
parent_per_ha = 40.0
offspring_mean = 20.0
cluster_sigma_m = 4.0
footprint_ratio = 0.2
"#;

const SEED: u64 = 91;
const TREE: u32 = 0;
const SHRUB: u32 = 1;
const SIDE: f64 = 512.0;
const SEA_X: f64 = 15.0;
/// Slope of the ramp: `y / 8` deg, so the tree's hat closes at y = 280 m.
const SLOPE_PER_M: f64 = 1.0 / 8.0;

/// Neutral climate; the slope rises north.
struct Ramp {
    names: Vec<String>,
}

impl Fields for Ramp {
    fn sample(&self, _: f64, y: f64) -> FieldSample {
        FieldSample {
            temperature_c: 10.0,
            precipitation_mm: 800.0,
            moisture: 0.5,
            class: Some(0),
            slope_deg: y * SLOPE_PER_M,
            aspect_deg: 0.0,
            altitude_m: 100.0,
            curvature: 0.0,
            water_distance_m: 1000.0,
        }
    }

    fn class_names(&self) -> &[String] {
        &self.names
    }
}

/// The shrub lives only west of x = 256 m.
fn prior(s: usize, x: f64, _: f64) -> f64 {
    if s == SHRUB as usize && x >= 256.0 { 0.0 } else { 1.0 }
}

/// Ground under the sea (0 m) west of `SEA_X`.
fn height(x: f64, _: f64) -> f64 {
    if x < SEA_X { -1.0 } else { 10.0 }
}

fn never(_: f64, _: f64) -> bool {
    false
}

struct Fixture {
    file: SpeciesFile,
    suit: Vec<Suitability>,
    water: WaterMask,
    fields: Ramp,
}

impl Fixture {
    fn new() -> Self {
        let file = SpeciesFile::parse_toml(SPECIES).expect("fixture parses");
        let suit = file.species.iter().map(|s| Suitability::new(s, &[])).collect();
        Self { file, suit, water: WaterMask::sea(0.0, 0.3), fields: Ramp { names: Vec::new() } }
    }

    fn rules(&self) -> CandidateRules<'_> {
        CandidateRules {
            file: &self.file,
            suitability: &self.suit,
            seed: SEED,
            tile_m: 256.0,
            prior: &prior,
            water: &self.water,
            height: &height,
            rocks: None,
            canopy: test_targets(&self.file),
        }
    }

    fn stand(&self, rect: [f64; 4]) -> StandMap<'_> {
        StandMap::build(StandRules { values: &self.file.values, seed: SEED, canopy: &self.suit }, &self.fields, &never, rect)
    }

    /// One tile built alone (its own stand map over its own grown rect).
    fn tile(&self, t: (i32, i32)) -> Vec<Candidate> {
        let r = self.rules();
        r.tile_winners(&self.fields, &self.stand(r.tile_rect(t)), t)
    }

    /// The whole square in one build.
    fn whole(&self) -> Vec<Candidate> {
        let r = self.rules();
        let g = r.square_reach_m();
        let stand = self.stand([-g, -g, SIDE + g, SIDE + g]);
        r.square_winners(&self.fields, &stand, [0.0, 0.0, SIDE, SIDE])
    }
}

const TILES: [(i32, i32); 4] = [(0, 0), (1, 0), (0, 1), (1, 1)];

type Bits = (CandidateKey, [u64; 4], u64);

fn bits(c: &[Candidate]) -> Vec<Bits> {
    c.iter()
        .map(|c| (c.key, [c.pos[0].to_bits(), c.pos[1].to_bits(), c.size_m.to_bits(), c.crown_m.to_bits()], c.tiebreak))
        .collect()
}

fn sorted(mut v: Vec<Candidate>) -> Vec<Candidate> {
    v.sort_unstable_by_key(|c| c.key);
    v
}

#[test]
fn tiles_alone_in_any_order_on_any_threads_equal_the_whole_square() {
    let fx = Fixture::new();
    let whole = fx.whole();
    assert!(whole.windows(2).all(|w| w[0].key < w[1].key), "the whole build is in key order");
    let per_tile = |order: &[(i32, i32)], threads: usize| {
        let tiles: Vec<Vec<Candidate>> = rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .unwrap()
            .install(|| order.par_iter().map(|&t| fx.tile(t)).collect());
        sorted(tiles.into_iter().flatten().collect())
    };
    let mut reversed = TILES;
    reversed.reverse();
    let a = per_tile(&TILES, 1);
    println!("{} instances over {SIDE} m; halo {:.1} m", whole.len(), fx.rules().halo_m());
    assert!(whole.len() > 5_000, "{} instances", whole.len());
    assert_eq!(bits(&a), bits(&whole), "tiles alone == the whole square");
    assert_eq!(bits(&per_tile(&reversed, 4)), bits(&a), "order and threads");
    assert_eq!(bits(&per_tile(&TILES, 7)), bits(&a));
}

#[test]
fn nothing_too_steep_off_region_or_on_the_sea() {
    let fx = Fixture::new();
    let won = fx.whole();
    let count = |f: &dyn Fn(&Candidate) -> bool| won.iter().filter(|c| f(c)).count();
    let steep = |c: &Candidate| c.pos[1] * SLOPE_PER_M >= 35.0;
    // Non-vacuous: trees on the gentle half, shrubs in their region, every layer present.
    assert!(count(&|c| c.key.species == TREE && c.pos[1] < 200.0) > 100);
    assert!(count(&|c| c.key.species == SHRUB && c.pos[0] < 256.0) > 100);
    assert!(count(&|c| c.key.species != TREE && steep(c)) > 100, "other species on the steep half");
    assert_eq!(count(&|c| c.key.species == TREE && steep(c)), 0, "a tree beyond its slope hat");
    assert_eq!(count(&|c| c.key.species == SHRUB && c.pos[0] >= 256.0), 0, "a shrub outside its region");
    assert_eq!(count(&|c| c.pos[0] < SEA_X), 0, "an instance on the sea");
}

#[test]
fn budgets_nest_exactly_and_each_group_keeps_its_share() {
    let fx = Fixture::new();
    let won = fx.whole();
    let n = won.len();
    let share = |l: Layer| won.iter().filter(|c| c.key.layer == l).count();
    let layers = [Layer::Canopy, Layer::Understorey, Layer::Ground];
    let budgets = [0.0, 1.0, 100.0, 1_000.0, (n / 3) as f64, (n / 2) as f64, (n - 1) as f64, n as f64, 2.0 * n as f64];
    let mut previous: Option<Vec<bool>> = None;
    for b in budgets {
        let keep = budget_cut(SEED, &won, b, 1.0);
        assert_eq!(keep, budget_cut(SEED, &won, b, 1.0), "pure");
        let kept = keep.iter().filter(|&&k| k).count();
        for l in layers {
            let n_l = share(l);
            let want = if b >= n as f64 { n_l } else { ((b * n_l as f64 / n as f64).round() as usize).min(n_l) };
            let got = won.iter().zip(&keep).filter(|(c, k)| **k && c.key.layer == l).count();
            assert_eq!(got, want, "{l:?} at budget {b}");
        }
        assert!((kept as f64 - b.min(n as f64)).abs() <= 1.5, "total {kept} at budget {b}");
        if let Some(p) = &previous {
            assert!(p.iter().zip(&keep).all(|(a, b)| !a || *b), "budget {b} keeps a superset of the one below");
        }
        previous = Some(keep);
    }
    // The cut is the same on any input order.
    let mut shuffled = won.clone();
    shuffled.reverse();
    let b = (n / 3) as f64;
    let a: Vec<CandidateKey> = won.iter().zip(budget_cut(SEED, &won, b, 1.0)).filter(|x| x.1).map(|x| x.0.key).collect();
    let s: Vec<CandidateKey> =
        sorted(shuffled.iter().zip(budget_cut(SEED, &shuffled, b, 1.0)).filter(|x| x.1).map(|x| *x.0).collect())
            .into_iter()
            .map(|c| c.key)
            .collect();
    assert_eq!(a, s);
}

#[test]
fn the_budget_keeps_smallest_entry_first_so_large_instances_first() {
    let fx = Fixture::new();
    let won = fx.whole();
    let b = (won.len() / 4) as f64;
    for p in [0.0, 1.0] {
        let keep = budget_cut(SEED, &won, b, p);
        let entry = |c: &Candidate| budget_rank(SEED, &c.key) / c.size_m.max(0.1).powf(p);
        for l in [Layer::Canopy, Layer::Understorey, Layer::Ground] {
            let of = |k: bool| won.iter().zip(&keep).filter(move |(c, kk)| c.key.layer == l && **kk == k).map(|x| x.0);
            let worst_kept = of(true).map(entry).fold(f64::NEG_INFINITY, f64::max);
            let best_dropped = of(false).map(entry).fold(f64::INFINITY, f64::min);
            assert!(worst_kept <= best_dropped, "{l:?} p {p}: kept {worst_kept} after dropped {best_dropped}");
            let mean = |it: &mut dyn Iterator<Item = &Candidate>| {
                let (s, k) = it.fold((0.0, 0usize), |(s, k), c| (s + c.size_m, k + 1));
                s / k as f64
            };
            let all = mean(&mut won.iter().filter(|c| c.key.layer == l));
            let kept = mean(&mut of(true));
            println!("{l:?} p {p}: mean size kept {kept:.3} m, all {all:.3} m");
            if p > 0.0 {
                assert!(kept > all * 1.02, "{l:?}: large instances first");
            } else {
                assert!((kept / all - 1.0).abs() < 0.05, "{l:?}: p 0 thins every size alike");
            }
        }
    }
    // The rank is uniform: its mean over the set is near 1/2.
    let mean_u = won.iter().map(|c| budget_rank(SEED, &c.key)).sum::<f64>() / won.len() as f64;
    assert!((mean_u - 0.5).abs() < 0.02, "mean rank {mean_u}");
}

/// WG-E2i.3: the four groups of pass 10. Synthetic rocks beside the fixture's plants: each group (rocks, canopy,
/// understorey, ground) keeps `round(budget x n_g / n)`, budgets nest, the rocks' cut is the same on reversed input,
/// and with no rocks the plants' cut is [`budget_cut`]'s.
#[test]
fn the_rock_group_keeps_its_share_beside_the_plant_layers() {
    use crate::scatter_rocks::{RockCandidate, RockKey};
    use crate::scatter_species::RockContext;
    let fx = Fixture::new();
    let plants = fx.whole();
    let contexts = [RockContext::Talus, RockContext::Outcrop, RockContext::Field];
    let rocks: Vec<RockCandidate> = (0..3000u32)
        .map(|i| RockCandidate {
            key: RockKey { context: contexts[(i % 3) as usize], tile: ((i % 7) as i32, 0), parent: i / 21, offspring: i % 21 },
            pos: [f64::from(i % 50), f64::from(i / 50)],
            size_m: 0.3 + f64::from(i % 17),
            tiebreak: u64::from(i),
            source: [0.0, 0.0],
        })
        .collect();
    let w = LayerWinners { rocks, plants };
    let n = w.rocks.len() + w.plants.len();
    let mut previous: Option<(Vec<bool>, Vec<bool>)> = None;
    for b in [0.0, 10.0, 500.0, (n / 4) as f64, (n / 2) as f64, (n - 1) as f64, n as f64] {
        let (rk, pk) = budget_cut_layers(SEED, &w, b, 1.0);
        assert_eq!((rk.len(), pk.len()), (w.rocks.len(), w.plants.len()));
        let want = |n_g: usize| if b >= n as f64 { n_g } else { ((b * n_g as f64 / n as f64).round() as usize).min(n_g) };
        assert_eq!(rk.iter().filter(|&&k| k).count(), want(w.rocks.len()), "rocks at budget {b}");
        for l in [Layer::Canopy, Layer::Understorey, Layer::Ground] {
            let n_l = w.plants.iter().filter(|c| c.key.layer == l).count();
            let got = w.plants.iter().zip(&pk).filter(|(c, k)| **k && c.key.layer == l).count();
            assert_eq!(got, want(n_l), "{l:?} at budget {b}");
        }
        if let Some((pr, pp)) = &previous {
            assert!(pr.iter().zip(&rk).chain(pp.iter().zip(&pk)).all(|(a, b)| !a || *b), "budget {b} nests");
        }
        previous = Some((rk, pk));
    }
    // Order independent: the reversed rocks keep the same keys.
    let b = (n / 3) as f64;
    let kept = |w: &LayerWinners| {
        let (rk, _) = budget_cut_layers(SEED, w, b, 1.0);
        let mut k: Vec<RockKey> = w.rocks.iter().zip(rk).filter(|x| x.1).map(|x| x.0.key).collect();
        k.sort_unstable();
        k
    };
    let mut rev = w.clone();
    rev.rocks.reverse();
    assert_eq!(kept(&w), kept(&rev));
    // No rocks: the plants' cut alone.
    let only = LayerWinners { rocks: Vec::new(), plants: w.plants.clone() };
    assert_eq!(budget_cut_layers(SEED, &only, b, 1.0).1, budget_cut(SEED, &w.plants, b, 1.0));
}

/// WG-E2p: the full radius (spec "Design" 10). With a focus at the fixture square's centre and a radius over about a
/// quarter of it: every instance inside is kept at every budget (inside == the unbudgeted set there); the total is
/// `max(budget, inside)` capped by the set (up to the per-group rounding); the outside set nests with the budget and
/// keeps its groups' shares of the global remainder; no focus, or a radius 0, is [`budget_cut_layers`]' cut.
#[test]
fn the_full_radius_keeps_everything_inside_and_one_remainder_outside() {
    use crate::scatter_rocks::{RockCandidate, RockKey};
    use crate::scatter_species::RockContext;
    let fx = Fixture::new();
    let plants = fx.whole();
    let rocks: Vec<RockCandidate> = (0..2000u32)
        .map(|i| RockCandidate {
            key: RockKey { context: RockContext::Field, tile: ((i % 2) as i32, 0), parent: i / 10, offspring: i % 10 },
            pos: [f64::from(i % 40) * 12.8, f64::from(i / 40) * 10.24],
            size_m: 0.3 + f64::from(i % 13),
            tiebreak: u64::from(i),
            source: [0.0, 0.0],
        })
        .collect();
    let w = LayerWinners { rocks, plants };
    let n = w.rocks.len() + w.plants.len();
    let focus = Focus { at: [SIDE / 2.0, SIDE / 2.0], radius_m: SIDE / 3.5 };
    let near = |p: [f64; 2]| (p[0] - focus.at[0]).powi(2) + (p[1] - focus.at[1]).powi(2) <= focus.radius_m.powi(2);
    let group = |c: &Candidate| layer_group(c.key.layer) as usize;
    let mut inside = [w.rocks.iter().filter(|c| near(c.pos)).count(), 0, 0, 0];
    for c in w.plants.iter().filter(|c| near(c.pos)) {
        inside[group(c)] += 1;
    }
    let n_in: usize = inside.iter().sum();
    assert!(n_in > 0 && n_in < n / 2, "the fixture's radius holds a part of the set: {n_in} of {n}");
    let mut outside = [w.rocks.len() - inside[0], 0, 0, 0];
    for c in &w.plants {
        outside[group(c)] += 1;
    }
    for g in 1..4 {
        outside[g] -= inside[g];
    }
    let n_out = n - n_in;
    let budgets = [0.0, 10.0, (n_in / 2) as f64, n_in as f64, (n_in + n_out / 3) as f64, (n - 1) as f64, n as f64, 2.0 * n as f64];
    let mut previous: Option<FocusCut> = None;
    for b in budgets {
        let cut = budget_cut_focus(SEED, &w, b, 1.0, Some(focus));
        assert_eq!(cut, budget_cut_focus(SEED, &w, b, 1.0, Some(focus)), "pure");
        assert_eq!(cut.inside, inside, "inside per group at budget {b}");
        let rocks_in = w.rocks.iter().zip(&cut.rocks).filter(|(c, _)| near(c.pos));
        let plants_in = w.plants.iter().zip(&cut.plants).filter(|(c, _)| near(c.pos));
        assert!(rocks_in.map(|x| *x.1).chain(plants_in.map(|x| *x.1)).all(|k| k), "everything inside kept at budget {b}");
        // Outside: each group keeps its share of the one remainder.
        let remainder = (b - n_in as f64).max(0.0);
        let want = |g: usize| {
            if remainder >= n_out as f64 { outside[g] } else { ((remainder * outside[g] as f64 / n_out as f64).round() as usize).min(outside[g]) }
        };
        let mut got = [w.rocks.iter().zip(&cut.rocks).filter(|(c, k)| **k && !near(c.pos)).count(), 0, 0, 0];
        for (c, _) in w.plants.iter().zip(&cut.plants).filter(|(c, k)| **k && !near(c.pos)) {
            got[group(c)] += 1;
        }
        for (g, &n_g) in got.iter().enumerate() {
            assert_eq!(n_g, want(g), "outside group {g} at budget {b}");
        }
        let kept = cut.rocks.iter().chain(&cut.plants).filter(|&&k| k).count();
        let total = b.max(n_in as f64).min(n as f64);
        assert!((kept as f64 - total).abs() <= 2.0, "total {kept} at budget {b}, want {total}");
        if let Some(p) = &previous {
            let nests = p.rocks.iter().zip(&cut.rocks).chain(p.plants.iter().zip(&cut.plants)).all(|(a, b)| !a || *b);
            assert!(nests, "budget {b} keeps a superset of the one below");
        }
        previous = Some(cut);
    }
    // No focus, or radius 0: today's cut, nothing inside.
    for b in [0.0, (n / 3) as f64, n as f64] {
        let (rk, pk) = budget_cut_layers(SEED, &w, b, 1.0);
        for f in [None, Some(Focus { radius_m: 0.0, ..focus })] {
            let cut = budget_cut_focus(SEED, &w, b, 1.0, f);
            assert_eq!((&cut.rocks, &cut.plants, cut.inside), (&rk, &pk, [0; 4]), "no full radius at budget {b}");
        }
    }
}

/// A synthetic stand for WG-E2q: 1 ha tiles (100 m) of `canopy[t]` canopy and 300 understorey instances each, at
/// hashed positions inside their tile, plus 200 rocks.
#[allow(clippy::cast_possible_wrap)] // tile indices 0..2
fn stand_fixture(canopy: [u32; 4]) -> LayerWinners {
    use crate::scatter_rocks::{RockCandidate, RockKey};
    use crate::scatter_species::RockContext;
    let at = |t: u32, i: u32, salt: u64| {
        let h = mix(u64::from(t) << 40 | u64::from(i) << 8 | salt);
        let (ox, oy) = (f64::from(t % 2) * 100.0, f64::from(t / 2) * 100.0);
        [ox + unit(h) * 100.0, oy + unit(mix(h)) * 100.0]
    };
    let mut plants = Vec::new();
    for t in 0..4u32 {
        for (layer, n) in [(Layer::Canopy, canopy[t as usize]), (Layer::Understorey, 300)] {
            for i in 0..n {
                let key = CandidateKey { layer, species: t, tile: ((t % 2) as i32, (t / 2) as i32), parent: i / 7, offspring: i % 7 };
                let size_m = if layer == Layer::Canopy { 4.0 + f64::from(i % 17) } else { 0.5 + f64::from(i % 5) * 0.3 };
                plants.push(Candidate { key, pos: at(t, i, layer as u64), size_m, crown_m: 1.0, tiebreak: u64::from(i) });
            }
        }
    }
    let rocks = (0..200u32)
        .map(|i| RockCandidate {
            key: RockKey { context: RockContext::Field, tile: (0, 0), parent: i / 10, offspring: i % 10 },
            pos: at(i % 4, i, 9),
            size_m: 0.3 + f64::from(i % 13),
            tiebreak: u64::from(i),
            source: [0.0, 0.0],
        })
        .collect();
    LayerWinners { rocks, plants }
}

/// WG-E2q: the stand cap keeps at most `round(cap x tile area)` canopy instances per tile, the lowest budget ranks,
/// before the full radius (inside it too) and the budget; lower layers and rocks are never capped; a cap above every
/// tile's density is the uncapped cut; budgets still nest.
#[test]
fn the_stand_cap_keeps_the_lowest_ranks_per_tile_before_the_radius_and_budget() {
    let w = stand_fixture([600, 400, 100, 1000]);
    let stand = Stand { tile_m: 100.0, stems_per_ha_max: 400.0 };
    let capped = stand_cap(SEED, &w.plants, stand);
    assert_eq!(capped, stand_cap(SEED, &w.plants, stand), "pure");
    let tile = |c: &Candidate| ((c.pos[0] / 100.0).floor() as i32, (c.pos[1] / 100.0).floor() as i32);
    let canopy_kept = |keep: &[bool], t: (i32, i32)| {
        w.plants.iter().zip(keep).filter(|(c, k)| **k && c.key.layer == Layer::Canopy && tile(c) == t).count()
    };
    for (t, n) in [((0, 0), 600), ((1, 0), 400), ((0, 1), 100), ((1, 1), 1000)] {
        assert_eq!(canopy_kept(&capped, t), n.min(400), "tile {t:?}");
        // The survivors are the lowest ranks of their tile.
        let rank = |keep: bool| {
            w.plants.iter().zip(&capped).filter(move |(c, k)| **k == keep && c.key.layer == Layer::Canopy && tile(c) == t)
                .map(|(c, _)| budget_rank(SEED, &c.key))
        };
        let (top, low) = (rank(true).fold(0.0, f64::max), rank(false).fold(1.0, f64::min));
        assert!(top < low, "tile {t:?}: survivor rank {top} under dropped {low}");
    }
    let under = w.plants.iter().zip(&capped).filter(|(c, _)| c.key.layer != Layer::Canopy);
    assert!(under.clone().all(|(_, k)| *k) && under.count() == 1200, "lower layers never capped");
    let prm = |budget: f64, cap: f64, focus: Option<[f64; 2]>| Params {
        seed: SEED,
        tile_m: 100.0,
        budget,
        size_priority: 1.0,
        full_radius_m: 300.0,
        focus,
        canopy_stems_per_ha_max: cap,
        ..Params::default()
    };
    let centre = Some([100.0, 100.0]);
    let n = w.rocks.len() + w.plants.len();
    let n_capped = w.rocks.len() + capped.iter().filter(|&&k| k).count();
    // Inside the radius (here the whole fixture) and with any budget, no tile above the cap.
    for b in [0.0, 500.0, n as f64] {
        let cut = budget_cut_load(&w, &prm(b, 400.0, centre));
        assert!(cut.plants.iter().zip(&capped).all(|(k, c)| !k || *c), "a capped plant is never kept at budget {b}");
        assert_eq!(cut.plants, capped, "the radius covers the fixture: every survivor kept at budget {b}");
        assert_eq!(cut.inside.iter().sum::<usize>(), n_capped, "inside counts the survivors only");
        assert!(cut.rocks.iter().all(|&k| k), "rocks never capped");
    }
    // Without a focus: the budget cuts the survivors (their count is the set), and budgets nest.
    let mut previous: Option<FocusCut> = None;
    for b in [0.0, 300.0, 1000.0, (n_capped - 1) as f64, n_capped as f64, n as f64] {
        let cut = budget_cut_load(&w, &prm(b, 400.0, None));
        assert!(cut.plants.iter().zip(&capped).all(|(k, c)| !k || *c), "capped never kept at budget {b}");
        let kept = cut.rocks.iter().chain(&cut.plants).filter(|&&k| k).count();
        assert!((kept as f64 - b.min(n_capped as f64)).abs() <= 2.0, "kept {kept} at budget {b} of {n_capped}");
        if let Some(p) = &previous {
            let nests = p.rocks.iter().zip(&cut.rocks).chain(p.plants.iter().zip(&cut.plants)).all(|(a, b)| !a || *b);
            assert!(nests, "budget {b} keeps a superset of the one below");
        }
        previous = Some(cut);
    }
    // A cap above every tile's density (1000/ha here) is the uncapped cut, with a focus or without.
    for b in [0.0, 700.0, n as f64] {
        for (focus, f) in [(None, None), (centre, Some(Focus { at: [100.0, 100.0], radius_m: 300.0 }))] {
            assert_eq!(budget_cut_load(&w, &prm(b, 2000.0, focus)), budget_cut_focus(SEED, &w, b, 1.0, f), "cap 2000 at {b}");
        }
    }
}
