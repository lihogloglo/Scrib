//! Headless tests for [`AiAllocPlugin`] (slice AI9b): ranking under overridden caps, hysteresis
//! when the player moves, pins, in-view holds, leaving the set; plus the µs-per-allocate
//! measurement for 2000 actors.

use std::time::Instant;

use bevy::time::{Fixed, Time};
use bevy_app::FixedUpdate;
use glam::Vec3;
use sf_ai::{CpuTier, PerceptTarget, Pin, Tier, TierCaps};
use sf_controller_bevy::Dormant;
use sf_ids::{ActorKey, PlayerKey};

use super::*;
use bevy::MinimalPlugins;
use sf_tunables_bevy::{SetBy, SetTunable, TunablesPlugin};

use crate::alloc::{CPU_TIER, CPU_TIER_CHOICES, CpuHost, auto_cpu_tier, cpu_tier_from_name};
use crate::sight_tests::{mover, wpos};

const HZ: f64 = 60.0;

fn app(caps: Option<TierCaps>) -> App {
    let mut app = App::new();
    app.insert_resource(Time::<Fixed>::from_hz(HZ));
    app.add_plugins(AiAllocPlugin { timings: true });
    if let Some(caps) = caps {
        app.world_mut().resource_mut::<AiAllocSettings>().set_caps(caps);
    }
    app
}

fn step(app: &mut App) {
    app.world_mut().run_schedule(FixedUpdate);
}

/// The player, standing at `x` on the X axis (a pose, no mover).
fn player(app: &mut App, x: f32) -> Entity {
    app.world_mut()
        .spawn((
            Perceivable { target: PerceptTarget::Player(PlayerKey(1)), chest_m: 1.2, head_m: 1.7 },
            PerceivablePose { feet: wpos(Vec3::new(x, 0.0, 0.0)), velocity: Vec3::ZERO },
        ))
        .id()
}

fn move_player(app: &mut App, e: Entity, x: f32) {
    app.world_mut().get_mut::<PerceivablePose>(e).unwrap().feet = wpos(Vec3::new(x, 0.0, 0.0));
}

/// Actors 1..=10 at x = 10, 20, ... 100 (entity i-1 has key i).
fn ten_actors(app: &mut App) -> Vec<Entity> {
    (1..=10u8)
        .map(|i| {
            let at = Vec3::new(10.0 * f32::from(i), 0.0, 0.0);
            app.world_mut().spawn((AiActor(ActorKey(u128::from(i))), mover(at, 0.0))).id()
        })
        .collect()
}

fn tier(app: &App, e: Entity) -> Tier {
    app.world().get::<AiTier>(e).expect("allocated").tier
}

fn tiers(app: &App, actors: &[Entity]) -> Vec<Tier> {
    actors.iter().map(|&e| tier(app, e)).collect()
}

fn bands(full: usize, reduced: usize, logical: usize) -> Vec<Tier> {
    [vec![Tier::Full; full], vec![Tier::Reduced; reduced], vec![Tier::Logical; logical]].concat()
}

const SMALL: TierCaps = TierCaps { full: 3, reduced: 4 };

#[test]
fn defaults_match_the_core() {
    let s = AiAllocSettings::default();
    assert_eq!(s.config, sf_ai::AllocConfig::default());
    assert_eq!(CPU_TIER_CHOICES[0], "auto");
    assert_eq!(cpu_tier_from_name("auto"), None);
    for (i, t) in CpuTier::ALL.into_iter().enumerate() {
        assert_eq!(s.caps_table[i], t.caps(), "{t:?}");
        assert_eq!(cpu_tier_from_name(CPU_TIER_CHOICES[i + 1]), Some(t));
    }
    assert_eq!(ALLOC_TUNABLES.len(), 15);
}

#[test]
fn auto_tier_follows_the_spec_reference_rows() {
    // Steam Deck 8 threads, 6-core desktop 12, the 7435HS laptop 16, 12 cores 24.
    assert_eq!(auto_cpu_tier(8, false), CpuTier::Floor);
    assert_eq!(auto_cpu_tier(12, false), CpuTier::Medium);
    assert_eq!(auto_cpu_tier(16, false), CpuTier::High);
    assert_eq!(auto_cpu_tier(24, false), CpuTier::Ultra);
    // Boundaries and edges.
    assert_eq!(auto_cpu_tier(1, false), CpuTier::Floor);
    assert_eq!(auto_cpu_tier(9, false), CpuTier::Medium);
    assert_eq!(auto_cpu_tier(13, false), CpuTier::High);
    assert_eq!(auto_cpu_tier(17, false), CpuTier::Ultra);
    // Headless is Dedicated at any count.
    for threads in [1, 8, 16, 24, 128] {
        assert_eq!(auto_cpu_tier(threads, true), CpuTier::Dedicated, "{threads}");
    }
}

#[test]
fn default_is_auto_and_resolves_from_the_host() {
    assert_eq!(CPU_TIER.default_value(), "auto");
    let mut s = AiAllocSettings::default();
    assert_eq!(s.cpu_tier_choice, None);
    assert_eq!(s.host, CpuHost::detect());
    assert!(!s.host.headless);
    assert_eq!(s.cpu_tier(), auto_cpu_tier(s.host.logical_threads, false));
    s.host = CpuHost { logical_threads: 12, headless: false };
    assert_eq!((s.cpu_tier(), s.caps()), (CpuTier::Medium, CpuTier::Medium.caps()));
    s.host.headless = true;
    assert_eq!((s.cpu_tier(), s.caps()), (CpuTier::Dedicated, CpuTier::Dedicated.caps()));

    // In an app, on the first tick: the stats carry the resolved tier and its caps.
    let mut app = app(None);
    app.world_mut().resource_mut::<AiAllocSettings>().host = CpuHost { logical_threads: 16, headless: false };
    app.update();
    step(&mut app);
    let stats = *app.world().resource::<AiAllocStats>();
    assert_eq!((stats.cpu_tier, stats.caps), (CpuTier::High, TierCaps { full: 128, reduced: 400 }));
}

#[test]
fn an_explicit_cpu_tier_overrides_auto() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, TunablesPlugin, AiAllocPlugin { timings: false }));
    app.world_mut().resource_mut::<AiAllocSettings>().host = CpuHost { logical_threads: 24, headless: false };
    app.update();
    step(&mut app);
    assert_eq!(app.world().resource::<AiAllocStats>().cpu_tier, CpuTier::Ultra);

    app.world_mut().write_message(SetTunable::new("sim.cpu_tier", "floor", SetBy::Console));
    app.update();
    let s = *app.world().resource::<AiAllocSettings>();
    assert_eq!(s.cpu_tier_choice, Some(CpuTier::Floor));
    // An explicit tier wins over headless too.
    app.world_mut().resource_mut::<AiAllocSettings>().host.headless = true;
    step(&mut app);
    let stats = *app.world().resource::<AiAllocStats>();
    assert_eq!((stats.cpu_tier, stats.caps), (CpuTier::Floor, TierCaps { full: 48, reduced: 160 }));

    // Back to auto: headless now resolves to Dedicated.
    app.world_mut().write_message(SetTunable::new("sim.cpu_tier", "auto", SetBy::Console));
    app.update();
    step(&mut app);
    assert_eq!(app.world().resource::<AiAllocStats>().cpu_tier, CpuTier::Dedicated);
}

#[test]
fn nearest_get_the_bands_and_rerank_with_hysteresis() {
    let mut app = app(Some(SMALL));
    let p = player(&mut app, 0.0);
    let a = ten_actors(&mut app);
    step(&mut app);
    assert_eq!(tiers(&app, &a), bands(3, 4, 3));
    let s = *app.world().resource::<AiAllocStats>();
    assert_eq!((s.stats.full, s.stats.reduced, s.stats.logical), (3, 4, 3));
    assert_eq!(s.caps, SMALL);
    assert_eq!(s.tick, 1);
    assert!(s.allocate_us.is_some());
    // Tiers 2 and 1 decide on a staggered tick after this one; tier 0 on events only.
    for &e in &a {
        let t = *app.world().get::<AiTier>(e).unwrap();
        assert_eq!(t.first_decide_tick.is_some(), t.tier != Tier::Logical);
        assert!(t.first_decide_tick.is_none_or(|d| d > 1));
    }

    // Actor 4 (x 40) is 1 m nearer than actor 1 (x 10): inside the 5 m margin, actor 1 keeps tier 2.
    move_player(&mut app, p, 25.5);
    step(&mut app);
    assert_eq!(tiers(&app, &a), bands(3, 4, 3));

    // 7 m nearer: past the margin, actor 4 takes the slot and actor 1 drops to tier 1.
    move_player(&mut app, p, 28.5);
    step(&mut app);
    let t = tiers(&app, &a);
    assert_eq!(t[0], Tier::Reduced);
    assert_eq!(&t[1..4], &[Tier::Full; 3]);
    assert_eq!(&t[4..7], &[Tier::Reduced; 3]);
    assert_eq!(&t[7..], &[Tier::Logical; 3]);
}

#[test]
fn a_pin_makes_a_far_actor_full() {
    let mut app = app(Some(SMALL));
    player(&mut app, 0.0);
    let a = ten_actors(&mut app);
    step(&mut app);
    app.world_mut().entity_mut(a[9]).insert(AiPin(Pin::Combat));
    step(&mut app);
    assert_eq!(tier(&app, a[9]), Tier::Full);
    // The pin takes a tier-2 slot: the third nearest drops to tier 1, the cap holds.
    assert_eq!(&tiers(&app, &a)[..3], &[Tier::Full, Tier::Full, Tier::Reduced]);
    let s = app.world().resource::<AiAllocStats>().stats;
    assert_eq!((s.full, s.pinned, s.over_cap_full), (3, 1, 0));
}

#[test]
fn in_view_actors_never_drop_to_logical() {
    for in_view in [false, true] {
        let mut app = app(Some(SMALL));
        let p = player(&mut app, 0.0);
        let a = ten_actors(&mut app);
        if in_view {
            for &e in &a {
                app.world_mut().entity_mut(e).insert(AiInView);
            }
        }
        step(&mut app);
        assert_eq!(tiers(&app, &a), bands(3, 4, 3));
        // The player walks past the far end: actors 8-10 are now the nearest.
        move_player(&mut app, p, 200.0);
        step(&mut app);
        let t = tiers(&app, &a);
        assert_eq!(&t[7..], &[Tier::Full; 3]);
        assert_eq!(&t[3..7], &[Tier::Reduced; 4]);
        let s = app.world().resource::<AiAllocStats>().stats;
        if in_view {
            // Seen tier-2 incumbents (actors 1-3) hold tier 1, past its cap.
            assert_eq!(&t[..3], &[Tier::Reduced; 3]);
            assert_eq!((s.held_in_view, s.over_cap_reduced), (3, 3));
        } else {
            assert_eq!(&t[..3], &[Tier::Logical; 3]);
            assert_eq!((s.held_in_view, s.over_cap_reduced), (0, 0));
        }
    }
}

#[test]
fn leaving_the_set_drops_the_tier_and_frees_the_slot() {
    let mut app = app(Some(SMALL));
    player(&mut app, 0.0);
    let a = ten_actors(&mut app);
    step(&mut app);
    app.world_mut().despawn(a[0]);
    app.world_mut().entity_mut(a[1]).insert(Dormant);
    step(&mut app);
    // Actors 3, 4, 5 are the nearest left.
    assert_eq!(app.world().get::<AiTier>(a[1]), None);
    assert_eq!(tiers(&app, &a[2..]), bands(3, 4, 1));
    let s = app.world().resource::<AiAllocStats>().stats;
    assert_eq!(s.full + s.reduced + s.logical, 8);
    // Awake again: back in the set, ranked fresh.
    app.world_mut().entity_mut(a[1]).remove::<Dormant>();
    step(&mut app);
    assert_eq!(tier(&app, a[1]), Tier::Full);
}

#[test]
fn no_player_ranks_by_key() {
    let mut app = app(Some(SMALL));
    let a = ten_actors(&mut app);
    step(&mut app);
    assert_eq!(tiers(&app, &a), bands(3, 4, 3));
}

#[test]
#[ignore = "measurement: µs per allocate and per tick for 2000 actors, 3 players, High caps"]
#[allow(clippy::cast_precision_loss)]
fn measure_2000_actors() {
    const ACTORS: u32 = 2000;
    const TICKS: u32 = 200;
    let mut app = app(None);
    // High caps whatever this machine's auto tier.
    app.world_mut().resource_mut::<AiAllocSettings>().cpu_tier_choice = Some(CpuTier::High);
    let players: Vec<Entity> = (0..3u8).map(|i| player(&mut app, f32::from(i) * 150.0)).collect();
    for i in 0..ACTORS {
        // A deterministic scatter over ~600 x 600 m.
        let f = i as f32;
        let at = Vec3::new((f * 37.0) % 600.0 - 150.0, 0.0, (f * 91.0) % 600.0 - 300.0);
        let mut e = app.world_mut().spawn((AiActor(ActorKey(u128::from(i) + 1)), mover(at, 0.0)));
        if i % 10 == 0 {
            e.insert(AiInView);
        }
        if i % 97 == 0 {
            e.insert(AiPin(Pin::Dialogue));
        }
    }
    step(&mut app);
    let (mut sum_us, mut max_us) = (0.0f32, 0.0f32);
    let started = Instant::now();
    for t in 0..TICKS {
        for (k, &p) in players.iter().enumerate() {
            move_player(&mut app, p, k as f32 * 150.0 + t as f32 * 0.5);
        }
        step(&mut app);
        let us = app.world().resource::<AiAllocStats>().allocate_us.unwrap();
        sum_us += us;
        max_us = max_us.max(us);
    }
    let tick_us = started.elapsed().as_secs_f32() * 1e6 / TICKS as f32;
    let s = app.world().resource::<AiAllocStats>().stats;
    println!(
        "alloc {ACTORS} actors: allocate {:.1} us mean, {max_us:.1} us max; whole system tick {tick_us:.1} us; \
         full {} reduced {} logical {} pinned {} held {}",
        sum_us / TICKS as f32,
        s.full,
        s.reduced,
        s.logical,
        s.pinned,
        s.held_in_view
    );
}

#[test]
fn decide_ticks_follow_each_tier_period() {
    // Caps 1/1, no player: key 1 is Full, key 2 Reduced, the rest Logical (rank by key).
    let mut app = app(Some(TierCaps { full: 1, reduced: 1 }));
    let actors = ten_actors(&mut app);
    step(&mut app);
    #[allow(clippy::cast_possible_truncation)]
    let tick_s = (1.0 / HZ) as f32;
    let config = app.world().resource::<AiAllocSettings>().config;
    let mut decides = [0u32; 3];
    for _ in 0..600 {
        step(&mut app);
        let tick = app.world().resource::<AiAllocStats>().tick;
        for (n, &e) in decides.iter_mut().zip(&actors) {
            let t = *app.world().get::<AiTier>(e).expect("allocated");
            *n += u32::from(t.decides_at(tick, &config, tick_s));
        }
    }
    println!("decide ticks in 600 at 60 Hz: full {}, reduced {}, logical {}", decides[0], decides[1], decides[2]);
    assert_eq!(tiers(&app, &actors[..3]), [Tier::Full, Tier::Reduced, Tier::Logical]);
    assert!((66..=67).contains(&decides[0]), "Full: every 9 ticks (0.15 s)");
    assert_eq!(decides[1], 10, "Reduced: every 60 ticks (1 s)");
    assert_eq!(decides[2], 0, "Logical: no planner");
}

#[test]
fn decide_stats_roll_once_a_second() {
    let mut app = app(None);
    {
        let mut d = app.world_mut().resource_mut::<AiDecideStats>();
        d.record_plan(Some(Tier::Full), true);
        d.record_plan(Some(Tier::Reduced), false);
        d.record_plan(None, true);
    }
    for _ in 0..59 {
        step(&mut app);
    }
    assert_eq!(app.world().resource::<AiDecideStats>().last_second(), DecideCounts::default(), "not a second yet");
    step(&mut app);
    assert_eq!(
        app.world().resource::<AiDecideStats>().last_second(),
        DecideCounts { full: 1, reduced: 1, untiered: 1, events: 1 }
    );
}
