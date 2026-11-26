//! Headless tests for tiers taking effect on perception (slice AI9e): an observer's sight checks
//! run at its tier's period (tier 2 and no tier: `sight_period_s`; tier 1: `tier1_period_s`),
//! restart on a tier change, never at tier 0 (whose meter still decays); tier 0 hears nothing;
//! tier 1 hears by straight line in its space only, skipping the propagation (`AI9f`);
//! the executor's trace tier mirrors [`AiTier`].

use bevy::time::{Fixed, Time};
use bevy_app::App;
use glam::Vec3;
use sf_ai::{NoiseKindId, SightTimer, Tier, due};
use sf_controller_bevy::{MoveIntent, Mover};
use sf_ids::ActorKey;
use sf_world::{Location, WorldspaceId};

use super::*;
use crate::exec::AiExecTick;
use crate::sight_tests::{EYE, PLAYER, app, level, observer, step, target, wpos};

const HZ: f64 = 60.0;
/// 600 ticks = 10 s at 60 Hz: 100 tier-2 periods (0.1 s = 6 ticks), 10 tier-1 periods (1 s = 60).
const N: u32 = 600;

fn set_tier(app: &mut App, e: Entity, tier: Tier) {
    app.world_mut().entity_mut(e).insert(AiTier { tier, first_decide_tick: None });
}

fn timer(app: &App, e: Entity) -> SightTimer {
    app.world().get::<AiSightTimer>(e).unwrap().0
}

/// Steps `n` ticks and counts each observer's sight checks (a poll that ran rewrites its timer).
fn count_checks(app: &mut App, observers: &[Entity], n: u32) -> Vec<u32> {
    let mut prev: Vec<SightTimer> = observers.iter().map(|&e| timer(app, e)).collect();
    let mut counts = vec![0; observers.len()];
    for _ in 0..n {
        step(app);
        for (i, &e) in observers.iter().enumerate() {
            let now = timer(app, e);
            if now != prev[i] {
                counts[i] += 1;
                prev[i] = now;
            }
        }
    }
    counts
}

#[test]
fn sight_checks_run_at_the_tiers_period() {
    let mut app = app(HZ);
    let none = observer(&mut app, 7, Vec3::ZERO);
    let full = observer(&mut app, 8, Vec3::new(2.0, 0.0, 0.0));
    let reduced = observer(&mut app, 9, Vec3::new(4.0, 0.0, 0.0));
    let logical = observer(&mut app, 10, Vec3::new(6.0, 0.0, 0.0));
    set_tier(&mut app, full, Tier::Full);
    set_tier(&mut app, reduced, Tier::Reduced);
    set_tier(&mut app, logical, Tier::Logical);
    step(&mut app); // the fresh timers start counting
    let counts = count_checks(&mut app, &[none, full, reduced, logical], N);
    assert_eq!(counts, vec![100, 100, 10, 0]);
}

#[test]
fn a_tier_change_restarts_the_timer_on_the_new_periods_stagger() {
    let mut app = app(HZ);
    let key = ActorKey(11);
    let obs = observer(&mut app, key.0, Vec3::ZERO);
    set_tier(&mut app, obs, Tier::Full);
    step(&mut app);
    assert_eq!(count_checks(&mut app, &[obs], 60), vec![10]);

    set_tier(&mut app, obs, Tier::Reduced);
    let before = timer(&app, obs);
    step(&mut app);
    let tick = app.world().resource::<AiPerceptClock>().tick;
    assert_eq!(timer(&app, obs), SightTimer::starting_at(tick), "restarted on the change tick");
    assert_ne!(timer(&app, obs), before);
    // The first tier-1 check lands on the actor's own stagger over 60 ticks.
    let mut first = None;
    for _ in 0..60 {
        let prev = timer(&app, obs);
        step(&mut app);
        if timer(&app, obs) != prev {
            first = Some(app.world().resource::<AiPerceptClock>().tick);
            break;
        }
    }
    let first = first.expect("checks within one tier-1 period");
    assert!(due(key, first, 60), "tick {first} is not the stagger phase");
    assert_eq!(count_checks(&mut app, &[obs], N), vec![10]);
}

#[test]
fn a_logical_observer_never_gains_but_its_meter_still_decays() {
    let mut app = app(HZ);
    let obs = observer(&mut app, 7, Vec3::ZERO);
    target(&mut app, Vec3::new(0.0, 0.0, 25.0));
    set_tier(&mut app, obs, Tier::Full);
    for _ in 0..60 {
        step(&mut app);
    }
    let seen = level(&app, obs).expect("tier 2 sees the target");
    assert!(seen > 0.0);

    set_tier(&mut app, obs, Tier::Logical);
    let mut last = seen;
    for _ in 0..N {
        step(&mut app);
        let l = level(&app, obs).unwrap_or(0.0);
        assert!(l <= last, "a tier-0 meter never gains ({l} > {last})");
        last = l;
    }
    assert!(last < seen, "past decay_delay_s the meter decays");
}

#[test]
fn a_logical_listener_hears_nothing_a_full_one_hears() {
    let mut app = app(HZ);
    let full = observer(&mut app, 7, Vec3::ZERO);
    let logical = observer(&mut app, 8, Vec3::ZERO);
    set_tier(&mut app, full, Tier::Full);
    set_tier(&mut app, logical, Tier::Logical);
    step(&mut app);
    let ev = NoiseEvent {
        tick: 0,
        at: wpos(Vec3::new(0.0, EYE, 5.0)),
        space: None,
        kind: NoiseKindId(0),
        loudness_m: 20.0,
        source: Some(PLAYER),
        reveals_source: true,
        notify: None,
    };
    app.world_mut().write_message(ev);
    step(&mut app);
    assert!(level(&app, full).is_some_and(|l| l > 0.0), "tier 2 hears the footstep");
    assert_eq!(level(&app, logical), None);
    assert!(!timer(&app, logical).is_forced(), "unheard: no forced check");
}

/// A door detour: every path is 100 m, so only straight-line listeners hear a 20 m footstep.
struct Detour;

impl NoisePaths for Detour {
    fn distance(&self, _from: &Location, _to: &Location, _world: &NoiseWorld<'_>) -> Option<NoisePath> {
        Some(NoisePath { path_m: 100.0, closed_doors: 0 })
    }
}

#[test]
fn a_reduced_listener_hears_by_straight_line_in_its_space_only() {
    let mut app = app(HZ);
    app.insert_resource(NoisePropagation::new(Detour));
    let full = observer(&mut app, 7, Vec3::ZERO);
    let reduced = observer(&mut app, 8, Vec3::ZERO);
    let elsewhere = observer(&mut app, 9, Vec3::ZERO);
    set_tier(&mut app, full, Tier::Full);
    set_tier(&mut app, reduced, Tier::Reduced);
    set_tier(&mut app, elsewhere, Tier::Reduced);
    app.world_mut().entity_mut(reduced).insert(AiExecutor::new(WorldspaceId::new("hall")));
    app.world_mut().entity_mut(elsewhere).insert(AiExecutor::new(WorldspaceId::new("cellar")));
    step(&mut app);
    let ev = NoiseEvent {
        tick: 0,
        at: wpos(Vec3::new(0.0, EYE, 5.0)),
        space: Some(WorldspaceId::new("hall")),
        kind: NoiseKindId(0),
        loudness_m: 20.0,
        source: Some(PLAYER),
        reveals_source: true,
        notify: None,
    };
    app.world_mut().write_message(ev);
    step(&mut app);
    assert_eq!(level(&app, full), None, "tier 2 takes the 100 m detour: out of reach");
    assert!(level(&app, reduced).is_some_and(|l| l > 0.0), "tier 1 hears by the 5 m straight line");
    assert!(timer(&app, reduced).is_forced(), "heard: forced check");
    assert_eq!(level(&app, elsewhere), None, "tier 1 in another space hears nothing");
    assert!(!timer(&app, elsewhere).is_forced());
}

#[test]
fn the_executor_trace_tier_mirrors_ai_tier() {
    let mut app = App::new();
    app.insert_resource(Time::<Fixed>::from_hz(HZ));
    app.init_resource::<Time>();
    app.add_plugins(AiExecPlugin);
    let at = wpos(Vec3::ZERO);
    let exec = |app: &mut App, tier: Option<Tier>| {
        let mut e = app.world_mut().spawn((
            Mover::new(at, 0.0, sf_controller::MoverParams::default(), at),
            MoveIntent::default(),
            AiExecutor::new(WorldspaceId::new("tier_test")),
        ));
        if let Some(tier) = tier {
            e.insert(AiTier { tier, first_decide_tick: None });
        }
        e.id()
    };
    let none = exec(&mut app, None);
    let reduced = exec(&mut app, Some(Tier::Reduced));
    let logical = exec(&mut app, Some(Tier::Logical));
    step(&mut app);
    let tier = |app: &App, e: Entity| app.world().get::<AiExecutor>(e).unwrap().tier;
    assert_eq!((tier(&app, none), tier(&app, reduced), tier(&app, logical)), (2, 1, 0));
    assert!(app.world().resource::<AiExecTick>().0 > 0);
}
