use super::*;
use crate::ktx2_out::{BlockFormat, level_len, mip_count, write_ktx2};

/// 1440p output, 60° vertical field of view, no bias.
const VIEW: View = View { h_out: 1440.0, fov_y: std::f32::consts::FRAC_PI_3, bias: 0, fallback_rho: FALLBACK_RHO };
const BIG_POOL: Settings = Settings::with_pool(u64::MAX);

/// BC7 square texture levels (16 bytes per 4×4 block), capped at `finest`.
fn bc7(size: u32, finest: u32) -> TextureLevels {
    let bytes: Vec<u64> = (0..mip_count(size, size))
        .map(|l| level_len(BlockFormat::Bc7Srgb, level_size(size, l), level_size(size, l)) as u64)
        .collect();
    TextureLevels::new(size, size, (4, 4), &bytes, finest)
}

fn demand(distance: f32) -> Demand {
    Demand { distance, rho: Some(1.0), in_reveal_radius: false }
}

fn near(in_reveal_radius: bool) -> Demand {
    Demand { distance: 0.1, rho: Some(1.0), in_reveal_radius }
}

fn input<'a>(id: u64, levels: &'a TextureLevels, resident: u32, demands: &'a [Demand]) -> TextureInput<'a> {
    TextureInput { id: TextureId(id), levels, resident, demands }
}

#[test]
fn wanted_level_follows_texels_per_pixel_and_doubling_distance_is_one_level() {
    // 2048 texels at 1 UV/m, 1440 px, 60°: 2048·2·tan(30°)/1440 = 1.642 texels per pixel per metre.
    let top = 2048;
    assert_eq!(texel_level(&VIEW, top, 1.0, 0.5), -1);
    assert_eq!(texel_level(&VIEW, top, 1.0, 1.0), 0);
    assert_eq!(texel_level(&VIEW, top, 1.0, 1.3), 1);
    assert_eq!(texel_level(&VIEW, top, 1.0, 2.6), 2);
    assert_eq!(texel_level(&VIEW, top, 1.0, 5.2), 3);
    assert_eq!(texel_level(&VIEW, top, 1.0, 10.4), 4);
    // Twice the UV density is twice the texels: one level finer wanted further.
    assert_eq!(texel_level(&VIEW, top, 2.0, 2.6), 3);
    // Inside the sphere: the finest.
    assert_eq!(texel_level(&VIEW, top, 1.0, 0.0), -64);
    // Preset bias adds levels.
    assert_eq!(texel_level(&View { bias: 1, ..VIEW }, top, 1.0, 1.3), 2);
    assert_eq!(texel_level(&View { bias: -1, ..VIEW }, top, 1.0, 1.3), 0);
}

#[test]
fn wanted_level_clamps_to_cap_tail_and_uses_the_fallback_density() {
    let full = bc7(2048, 0);
    assert_eq!(full.starts(), &[0, 1, 2, 3, 4]); // 2048..128
    assert_eq!(wanted_level(&VIEW, &full, &demand(0.0)), 0);
    assert_eq!(wanted_level(&VIEW, &full, &demand(2.6)), 2);
    assert_eq!(wanted_level(&VIEW, &full, &demand(10_000.0)), 4, "never past the mip tail");
    let capped = bc7(2048, 1);
    assert_eq!(wanted_level(&VIEW, &capped, &demand(0.0)), 1, "never finer than the cap");
    // No density: 1 UV/m.
    let none = Demand { rho: None, ..demand(2.6) };
    assert_eq!(wanted_level(&VIEW, &full, &none), 2);
    let bad = Demand { rho: Some(f32::NAN), ..demand(2.6) };
    assert_eq!(wanted_level(&VIEW, &full, &bad), 2);
    assert_eq!(wanted_level(&View { fallback_rho: 4.0, ..VIEW }, &full, &none), 4);
}

#[test]
fn start_levels_skip_small_textures_and_unaligned_levels() {
    assert_eq!(bc7(512, 0).starts(), &[0], "512² and below: not streamed");
    assert!(!bc7(512, 0).is_streamable());
    assert_eq!(bc7(1024, 1).starts(), &[1], "capped to 512²: not streamed");
    assert_eq!(bc7(1024, 0).starts(), &[0, 1, 2, 3]);
    // 1000×600 in 4×4 blocks: 500×300 aligned, 250×150 and 125×75 not.
    let npot = TextureLevels::new(1000, 600, (4, 4), &[8, 4, 2, 1, 1, 1, 1, 1, 1, 1], 0);
    assert_eq!(npot.starts(), &[0, 1]);
    assert_eq!(npot.snap(3), 1);
    assert_eq!(npot.bytes_at(1), 4 + 2 + 7);
}

#[test]
fn levels_from_a_ktx2_layout_follow_the_load_cap() {
    let levels: Vec<Vec<u8>> = (0..mip_count(2048, 2048))
        .map(|l| vec![0; level_len(BlockFormat::Bc7Srgb, level_size(2048, l), level_size(2048, l))])
        .collect();
    let file = write_ktx2(BlockFormat::Bc7Srgb, 2048, 2048, &levels);
    let layout = Layout::parse(&file).expect("parses");
    assert_eq!(TextureLevels::from_layout(&layout, None), bc7(2048, 0));
    let capped = TextureLevels::from_layout(&layout, Some(1024));
    assert_eq!(capped, bc7(2048, 1));
    assert_eq!(capped.bytes_at(0) - capped.bytes_at(1), level_len(BlockFormat::Bc7Srgb, 2048, 2048) as u64);
}

#[test]
fn pool_drops_cached_levels_then_least_recently_wanted_then_spreads_the_loss() {
    let t = bc7(2048, 0);
    let [d0, d1, d2] = [[near(false)], [near(false)], [near(false)]];
    let mut r = Residency::new();
    // t=0: all wanted at 0.
    r.plan(&VIEW, &BIG_POOL, &[input(1, &t, 0, &d0), input(2, &t, 0, &d1), input(3, &t, 0, &d2)], 0.0);
    // t=1: 1 loses its demand; t=2: 2 does too. Both stay cached at 0 (5 s downgrade wait).
    r.plan(&VIEW, &BIG_POOL, &[input(1, &t, 0, &[]), input(2, &t, 0, &d1), input(3, &t, 0, &d2)], 1.0);
    r.plan(&VIEW, &BIG_POOL, &[input(1, &t, 0, &[]), input(2, &t, 0, &[]), input(3, &t, 0, &d2)], 2.0);
    let now = [input(1, &t, 0, &[]), input(2, &t, 0, &[]), input(3, &t, 0, &d2)];
    let full = t.bytes_at(0);
    let one = full - t.bytes_at(1);
    // Re-planning at the same time leaves the hysteresis state as it is.
    let mut at = |pool: u64| r.plan(&VIEW, &Settings::with_pool(pool), &now, 3.0);

    // One top level too many: the least recently wanted cached texture (1) loses it.
    let plan = at(3 * full - one);
    assert_eq!(plan.targets, [1, 0, 0]);
    // Two levels: 1 and 2 lose one each (2 has more cached levels than 1 has left).
    let plan = at(3 * full - 2 * one);
    assert_eq!(plan.targets, [1, 1, 0]);
    // Cached levels go all the way (to the tail, 4) before the wanted texture 3 loses any.
    let tail = t.bytes_at(4);
    let plan = at(full + 2 * tail);
    assert_eq!(plan.targets, [4, 4, 0]);
    assert_eq!(plan.counters.limited_by_pool, 0, "cached levels are not a limit");
    // Then 3 loses levels too, from the top.
    let plan = at(full - one + 2 * tail);
    assert_eq!(plan.targets, [4, 4, 1]);
    assert_eq!(plan.counters.limited_by_pool, 1);
    assert!(plan.counters.over_budget);
    assert_eq!(plan.counters.bytes_target, full - one + 2 * tail);
}

#[test]
fn pool_ties_on_recency_take_the_most_bytes_freed() {
    let big = bc7(2048, 0);
    let small = bc7(1024, 0);
    let d = [near(false)];
    let inputs = [input(1, &small, 0, &d), input(2, &big, 0, &d)];
    let mut r = Residency::new();
    let pool = big.bytes_at(0) + small.bytes_at(0) - 1;
    let plan = r.plan(&VIEW, &Settings::with_pool(pool), &inputs, 0.0);
    assert_eq!(plan.targets, [0, 1]);
    assert_eq!(plan.counters.limited_by_pool, 1);
}

#[test]
fn reveal_radius_textures_drop_last_farthest_first_with_one_warning_per_episode() {
    let t = bc7(2048, 0);
    let out = [near(false)];
    let close = [Demand { distance: 0.1, ..near(true) }];
    let far = [Demand { distance: 0.5, ..near(true) }];
    let inputs = [input(1, &t, 0, &close), input(2, &t, 0, &far), input(3, &t, 0, &out)];
    let full = t.bytes_at(0);
    let one = full - t.bytes_at(1);
    let pool = 2 * full - one + t.bytes_at(4);
    let mut r = Residency::new();
    let plan = r.plan(&VIEW, &Settings::with_pool(pool), &inputs, 0.0);
    assert_eq!(plan.targets, [0, 1, 4], "outside first to the tail, then the farthest in radius");
    assert_eq!(plan.pool_warning, Some(vec![TextureId(2)]));
    assert_eq!(plan.counters.limited_in_radius, 1);
    // Same episode: no new warning.
    let plan = r.plan(&VIEW, &Settings::with_pool(pool), &inputs, 0.25);
    assert_eq!(plan.pool_warning, None);
    assert_eq!(plan.targets, [0, 1, 4]);
    // Pool fits: episode over; squeezed again: a new warning.
    let plan = r.plan(&VIEW, &BIG_POOL, &inputs, 0.5);
    assert_eq!((plan.targets, plan.pool_warning), (vec![0, 0, 0], None));
    let plan = r.plan(&VIEW, &Settings::with_pool(pool), &inputs, 0.75);
    assert_eq!(plan.pool_warning, Some(vec![TextureId(2)]));
}

#[test]
fn upgrades_wait_for_a_stable_wanted_level() {
    let t = bc7(2048, 0);
    let far = [demand(1000.0)];
    let close = [demand(0.0)];
    let mid = [demand(2.6)]; // level 2
    let mut r = Residency::new();
    let at = |r: &mut Residency, d: &[Demand], resident: u32, now: f64| r.plan(&VIEW, &BIG_POOL, &[input(1, &t, resident, d)], now);
    assert_eq!(at(&mut r, &far, 4, 0.0).targets, [4], "first sight takes the wanted level");
    assert_eq!(at(&mut r, &close, 4, 1.0).targets, [4]);
    assert_eq!(at(&mut r, &close, 4, 1.4).targets, [4]);
    // Interrupted: the window restarts.
    assert_eq!(at(&mut r, &far, 4, 1.45).targets, [4]);
    assert_eq!(at(&mut r, &close, 4, 1.5).targets, [4]);
    assert_eq!(at(&mut r, &mid, 4, 1.7).targets, [4]);
    // Stable for 0.5 s: upgraded to the coarsest level wanted over the window.
    let plan = at(&mut r, &close, 4, 2.0);
    assert_eq!(plan.targets, [2]);
    assert_eq!(plan.requests, [Request { id: TextureId(1), from: 4, to: 2 }]);
    assert_eq!(at(&mut r, &close, 2, 2.25).targets, [2]);
    assert_eq!(at(&mut r, &close, 2, 2.75).targets, [0]);
}

#[test]
fn downgrades_wait_five_seconds() {
    let t = bc7(2048, 0);
    let far = [demand(1000.0)];
    let close = [demand(0.0)];
    let mut r = Residency::new();
    let at = |r: &mut Residency, d: &[Demand], now: f64| r.plan(&VIEW, &BIG_POOL, &[input(1, &t, 0, d)], now).targets[0];
    assert_eq!(at(&mut r, &close, 0.0), 0);
    assert_eq!(at(&mut r, &far, 10.0), 0);
    assert_eq!(at(&mut r, &far, 14.9), 0);
    // Wanted again: the wait restarts.
    assert_eq!(at(&mut r, &close, 15.0), 0);
    assert_eq!(at(&mut r, &far, 16.0), 0);
    assert_eq!(at(&mut r, &far, 20.9), 0);
    assert_eq!(at(&mut r, &far, 21.0), 4);
    // Dropped textures lose their state.
    r.plan(&VIEW, &BIG_POOL, &[], 22.0);
    assert_eq!(r.tracked(), 0);
}

#[test]
fn requests_list_downgrades_first_then_upgrades_by_priority() {
    let t = bc7(2048, 0);
    let close = [demand(0.0)];
    let mid = [demand(2.6)];
    let in_radius = [near(true)];
    let far = [demand(1000.0)];
    let small = bc7(1024, 0);
    let inputs = [
        input(1, &t, 4, &mid),       // up 4 -> 2
        input(2, &t, 0, &far),       // down 0 -> 4 (first sight takes the wanted level)
        input(3, &t, 4, &close),     // up 4 -> 0, bigger deficit
        input(4, &t, 2, &in_radius), // up 2 -> 0, in the radius
        input(5, &small, 0, &far),   // down 0 -> 3, fewer bytes freed
        input(6, &t, 2, &mid),       // unchanged
    ];
    let plan = Residency::new().plan(&VIEW, &BIG_POOL, &inputs, 0.0);
    let order: Vec<u64> = plan.requests.iter().map(|q| q.id.0).collect();
    assert_eq!(order, [2, 5, 4, 3, 1]);
    assert!(plan.requests[0].is_downgrade() && !plan.requests[2].is_downgrade());
    assert_eq!((plan.counters.downgrades, plan.counters.upgrades), (2, 3));
    // The in-radius demand wants 0 and 4 is resident at 2.
    assert_eq!(plan.counters.mip_deficit_visible, 1);
    assert_eq!(plan.counters.streamable, 6);
}

/// Deterministic pseudo-random inputs.
struct Lcg(u64);

impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
        self.0 >> 33
    }
    #[allow(clippy::cast_precision_loss, reason = "test fixture: small values")]
    fn unit(&mut self) -> f32 {
        (self.next() % 10_000) as f32 / 10_000.0
    }
}

fn world(n: usize, per: usize, seed: u64) -> (Vec<TextureLevels>, Vec<Vec<Demand>>, Vec<u32>) {
    let mut rng = Lcg(seed);
    let sizes = [bc7(512, 0), bc7(1024, 0), bc7(2048, 0), bc7(2048, 1), bc7(4096, 0)];
    let mut levels = Vec::with_capacity(n);
    let mut demands = Vec::with_capacity(n);
    let mut resident = Vec::with_capacity(n);
    for _ in 0..n {
        let l = sizes[usize::try_from(rng.next()).unwrap_or(0) % sizes.len()].clone();
        resident.push(l.coarsest());
        levels.push(l);
        demands.push(
            (0..per)
                .map(|_| Demand {
                    distance: rng.unit() * 60.0,
                    rho: if rng.next().is_multiple_of(4) { None } else { Some(0.25 + rng.unit() * 4.0) },
                    in_reveal_radius: rng.next().is_multiple_of(8),
                })
                .collect(),
        );
    }
    (levels, demands, resident)
}

fn inputs<'a>(levels: &'a [TextureLevels], demands: &'a [Vec<Demand>], resident: &[u32]) -> Vec<TextureInput<'a>> {
    (0..levels.len()).map(|i| input(i as u64, &levels[i], resident[i], &demands[i])).collect()
}

#[test]
fn same_input_same_plan_whatever_the_input_order() {
    let (levels, demands, resident) = world(300, 3, 7);
    let forward = inputs(&levels, &demands, &resident);
    let mut backward = forward.clone();
    backward.reverse();
    let wanted = Residency::new().plan(&VIEW, &BIG_POOL, &forward, 0.0).counters.bytes_wanted;
    let settings = Settings::with_pool(wanted / 2);
    let (mut a, mut b) = (Residency::new(), Residency::new());
    for step in 0..6 {
        let now = f64::from(step) * 0.25;
        let pa = a.plan(&VIEW, &settings, &forward, now);
        let pb = b.plan(&VIEW, &settings, &backward, now);
        let by_id = |p: &Plan, ins: &[TextureInput<'_>]| {
            let mut v: Vec<(u64, u32)> = ins.iter().zip(&p.targets).map(|(i, t)| (i.id.0, *t)).collect();
            v.sort_unstable();
            v
        };
        assert_eq!(by_id(&pa, &forward), by_id(&pb, &backward));
        assert_eq!(pa.requests, pb.requests);
        assert_eq!(pa.counters, pb.counters);
        assert_eq!(pa.pool_warning, pb.pool_warning);
        assert!(pa.counters.over_budget);
        assert!(pa.counters.bytes_target < pa.counters.bytes_wanted);
        // Re-planning the same state at the same time changes nothing.
        let again = a.plan(&VIEW, &settings, &forward, now);
        assert_eq!(again.targets, pa.targets);
    }
}

#[test]
fn effective_distance_takes_the_nearer_of_now_and_ahead() {
    assert!((sphere_distance([10.0, 0.0, 0.0], 2.0, [0.0; 3]) - 8.0).abs() < 1e-6);
    assert!(sphere_distance([1.0, 0.0, 0.0], 2.0, [0.0; 3]).abs() < f32::EPSILON);
    let d = effective_distance([10.0, 0.0, 0.0], 2.0, [0.0; 3], [4.0, 0.0, 0.0], 1.0);
    assert!((d - 4.0).abs() < 1e-6);
    let away = effective_distance([10.0, 0.0, 0.0], 2.0, [0.0; 3], [-4.0, 0.0, 0.0], 1.0);
    assert!((away - 8.0).abs() < 1e-6);
}

#[test]
#[ignore = "measurement: planner time for 10k textures x 4 demands"]
fn measure_plan_10k_textures() {
    let (levels, demands, resident) = world(10_000, 4, 11);
    let ins = inputs(&levels, &demands, &resident);
    let total: u64 = levels.iter().map(|l| l.bytes_at(l.finest())).sum();
    let wanted = Residency::new().plan(&VIEW, &BIG_POOL, &ins, 0.0).counters.bytes_wanted;
    for (label, pool) in [("fits", u64::MAX), ("pool = half the wanted bytes", wanted / 2)] {
        let settings = Settings::with_pool(pool);
        let mut r = Residency::new();
        let mut times = Vec::new();
        let mut plan = None;
        for step in 0..201 {
            let start = std::time::Instant::now();
            plan = Some(r.plan(&VIEW, &settings, &ins, f64::from(step) * 0.25));
            times.push(start.elapsed());
        }
        times.sort_unstable();
        let plan = plan.expect("planned");
        println!(
            "residency plan ({label}): median {:?}, max {:?}; {} requests, {} limited by pool, target {} MiB of {} MiB finest",
            times[times.len() / 2],
            times[times.len() - 1],
            plan.requests.len(),
            plan.counters.limited_by_pool,
            plan.counters.bytes_target >> 20,
            total >> 20,
        );
    }
}
