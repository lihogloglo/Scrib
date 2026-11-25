//! Headless tests for [`AiSightPlugin`] (slice AI8b2): exact gain per check, chest/head rays
//! against a fake wall world, the ray cap's deferral, a profile's half range, tick-rate
//! invariance of the bands; plus the ms-per-tick measurement.

// Exact assertions on purpose (same inputs, same maths, same bits).
#![allow(clippy::float_cmp)]

use std::time::Instant;

use bevy::time::{Fixed, Time};
use bevy_app::FixedUpdate;
use glam::{DVec3, Vec3};
use sf_ai::{
    AwarenessBand, PerceptClock, PerceptTarget, PerceptionProfile, PerceptionStates, SightCheck, resolve,
    sight_gain,
};
use sf_controller::{GroundHit, GroundProbe, MoverParams, ProbeQuery, SurfaceId};
use sf_controller_bevy::Mover;
use sf_coords::WorldPos;
use sf_ids::{ActorKey, PlayerKey};

use super::*;

const EYE: f32 = 1.6;
const CHEST: f32 = 1.2;
const HEAD: f32 = 1.7;
const PLAYER: PerceptTarget = PerceptTarget::Player(PlayerKey(1));

fn wpos(v: Vec3) -> WorldPos {
    WorldPos::from_f64(DVec3::new(f64::from(v.x), f64::from(v.y), f64::from(v.z)))
}

fn xyz(p: WorldPos) -> Vec3 {
    WorldPos::ORIGIN.delta_f32(p)
}

/// Axis-aligned boxes that block rays (the fake ground probe).
#[derive(Resource, Default)]
struct Walls(Vec<(Vec3, Vec3)>);

impl Walls {
    fn hit(&self, o: Vec3, d: Vec3, max: f32) -> Option<f32> {
        let mut best: Option<f32> = None;
        for &(lo, hi) in &self.0 {
            let (mut t0, mut t1) = (0.0f32, max);
            let mut ok = true;
            for a in 0..3 {
                if d[a].abs() < 1e-9 {
                    if o[a] < lo[a] || o[a] > hi[a] {
                        ok = false;
                        break;
                    }
                } else {
                    let (mut ta, mut tb) = ((lo[a] - o[a]) / d[a], (hi[a] - o[a]) / d[a]);
                    if ta > tb {
                        std::mem::swap(&mut ta, &mut tb);
                    }
                    t0 = t0.max(ta);
                    t1 = t1.min(tb);
                    if t0 > t1 {
                        ok = false;
                        break;
                    }
                }
            }
            if ok {
                best = Some(best.map_or(t0, |b| b.min(t0)));
            }
        }
        best
    }
}

impl GroundProbe for Walls {
    fn probe(&self, queries: &[ProbeQuery], out: &mut [Option<GroundHit>]) {
        for (q, o) in queries.iter().zip(out.iter_mut()) {
            *o = self.hit(xyz(q.origin), q.dir, q.max).map(|t| GroundHit::new(q.origin.offset(q.dir * t), -q.dir, t, SurfaceId::DEFAULT));
        }
    }
}

fn app(hz: f64) -> App {
    let mut app = App::new();
    app.insert_resource(Time::<Fixed>::from_hz(hz));
    app.init_resource::<Walls>();
    app.add_plugins(AiSightPlugin::<Walls>::default());
    app
}

/// Every check on every tick (period = one tick).
fn every_tick(app: &mut App, hz: f64) {
    #[allow(clippy::cast_possible_truncation)]
    let tick_s = (1.0 / hz) as f32;
    app.world_mut().resource_mut::<AiPerceptionSettings>().budget.sight_period_s = tick_s;
}

fn step(app: &mut App) {
    app.world_mut().run_schedule(FixedUpdate);
}

fn mover(at: Vec3, yaw: f32) -> Mover {
    Mover::new(wpos(at), yaw, MoverParams::default(), WorldPos::ORIGIN)
}

/// An observer at `at` facing +Z (yaw 0).
fn observer(app: &mut App, key: u128, at: Vec3) -> Entity {
    app.world_mut().spawn((AiActor(ActorKey(key)), AiObserver { eye_m: EYE }, mover(at, 0.0))).id()
}

fn target(app: &mut App, at: Vec3) -> Entity {
    app.world_mut().spawn((Perceivable { target: PLAYER, chest_m: CHEST, head_m: HEAD }, mover(at, 0.0))).id()
}

fn level(app: &App, e: Entity) -> Option<f32> {
    app.world().get::<AiAwareness>(e).unwrap().0.get(PLAYER).map(|a| a.level)
}

fn band(app: &App, e: Entity) -> AwarenessBand {
    app.world().get::<AiAwareness>(e).unwrap().0.band(PLAYER)
}

/// The gain the next tick's check should give `obs` toward [`PLAYER`] at `tgt`, from the core maths.
fn expected_gain(app: &App, obs: Entity, obs_at: Vec3, tgt_at: Vec3) -> Option<f32> {
    let s = *app.world().resource::<AiPerceptionSettings>();
    let clock = *app.world().resource::<AiPerceptClock>();
    let tick = clock.tick + 1;
    let tick_s = app.world().resource::<Time<Fixed>>().timestep().as_secs_f32();
    let now = PerceptClock::new(tick, tick_s);
    let eff = resolve(&PerceptionProfile::default(), &PerceptionStates::new(), |_| None, &s.awareness, &s.sight, &s.search, tick);
    let meter = &app.world().get::<AiAwareness>(obs).unwrap().0;
    let period = s.budget.sight_period_ticks(tick_s);
    let check = SightCheck {
        observer: wpos(obs_at).offset(Vec3::Y * EYE),
        facing: sf_controller::forward(0.0),
        target: wpos(tgt_at).offset(Vec3::Y * CHEST),
        seen_before: seen_recently(meter, PLAYER, tick, period),
        visibility: 1.0,
        alertness: meter.alertness(&eff.cfg, PLAYER, now),
    };
    sight_gain(&eff.sight, &s.sight, &s.budget, &check, tick_s)
}

#[test]
fn gain_accumulates_exactly_as_sight_gain_predicts() {
    let mut app = app(60.0);
    every_tick(&mut app, 60.0);
    let (o_at, t_at) = (Vec3::ZERO, Vec3::new(0.0, 0.0, 25.0));
    let obs = observer(&mut app, 7, o_at);
    target(&mut app, t_at);
    step(&mut app); // the fresh timer starts counting
    assert_eq!(level(&app, obs), None);
    let mut expected = 0.0f32;
    for _ in 0..40 {
        let g = expected_gain(&app, obs, o_at, t_at).expect("in range and cone");
        expected = (expected + g).min(1.0);
        step(&mut app);
        assert_eq!(level(&app, obs), Some(expected));
    }
    assert!(band(&app, obs) >= AwarenessBand::Curious);
}

#[test]
fn blocked_chest_with_clear_head_still_sees() {
    let mut app = app(60.0);
    every_tick(&mut app, 60.0);
    // Eye 1.6 to chest 1.2 at 25 m passes z = 10 at 1.44 (blocked), the head ray at 1.64 (clear).
    app.world_mut().resource_mut::<Walls>().0.push((Vec3::new(-5.0, 0.0, 9.9), Vec3::new(5.0, 1.5, 10.1)));
    let obs = observer(&mut app, 7, Vec3::ZERO);
    target(&mut app, Vec3::new(0.0, 0.0, 25.0));
    step(&mut app);
    step(&mut app);
    assert_eq!(app.world().resource::<AiRays>().stats.cast, 2, "chest blocked, then the head ray");
    assert!(level(&app, obs).is_some_and(|l| l > 0.0));
}

#[test]
fn chest_and_head_blocked_sees_nothing() {
    let mut app = app(60.0);
    every_tick(&mut app, 60.0);
    app.world_mut().resource_mut::<Walls>().0.push((Vec3::new(-5.0, 0.0, 9.9), Vec3::new(5.0, 3.0, 10.1)));
    let obs = observer(&mut app, 7, Vec3::ZERO);
    target(&mut app, Vec3::new(0.0, 0.0, 25.0));
    for _ in 0..30 {
        step(&mut app);
    }
    assert_eq!(level(&app, obs), None);
}

#[test]
fn ray_cap_defers_the_25th_request_to_the_next_tick() {
    let mut app = app(60.0);
    every_tick(&mut app, 60.0);
    assert_eq!(app.world().resource::<AiPerceptionSettings>().budget.max_rays_per_tick, 24);
    let t_at = Vec3::new(0.0, 0.0, 20.0);
    target(&mut app, t_at);
    let observers: Vec<(Entity, Vec3)> = (0..25u8)
        .map(|i| {
            let at = Vec3::new(f32::from(i) - 12.0, 0.0, 0.0);
            (observer(&mut app, 100 + u128::from(i), at), at)
        })
        .collect();
    step(&mut app); // timers start
    let gains: Vec<f32> = observers.iter().map(|&(e, at)| expected_gain(&app, e, at, t_at).unwrap()).collect();
    step(&mut app);
    let stats = app.world().resource::<AiRays>().stats;
    assert_eq!((stats.cast, stats.deferred), (24, 1));
    let unseen: Vec<usize> = (0..25).filter(|&i| level(&app, observers[i].0).is_none()).collect();
    assert_eq!(unseen.len(), 1, "exactly one request waits");
    let late = unseen[0];
    let g2 = expected_gain(&app, observers[late].0, observers[late].1, t_at).unwrap();
    step(&mut app);
    let stats = app.world().resource::<AiRays>().stats;
    assert_eq!((stats.cast, stats.deferred), (1, 0), "the others reuse their cached line of sight");
    assert!(observers.iter().all(|&(e, _)| level(&app, e).is_some()));
    // The deferred request carries both checks' gain.
    assert_eq!(level(&app, observers[late].0), Some(gains[late] + g2));
}

#[test]
fn half_sight_range_profile_stops_seeing_at_half_range() {
    let mut app = app(60.0);
    every_tick(&mut app, 60.0);
    target(&mut app, Vec3::new(0.0, 0.0, 20.0));
    let normal = observer(&mut app, 1, Vec3::ZERO);
    let half = observer(&mut app, 2, Vec3::new(1.0, 0.0, 0.0));
    let half_near = observer(&mut app, 3, Vec3::new(0.0, 0.0, 10.0));
    let profile = AiPerceptionProfile(PerceptionProfile { sight_range: 0.5, ..PerceptionProfile::default() });
    app.world_mut().entity_mut(half).insert(profile);
    app.world_mut().entity_mut(half_near).insert(profile);
    for _ in 0..30 {
        step(&mut app);
    }
    assert!(app.world().get::<AiEffectivePerception>(normal).is_none(), "no profile, no cached resolve");
    let eff = app.world().get::<AiEffectivePerception>(half).unwrap().0;
    assert_eq!(eff.sight.range_m, 15.0);
    assert!(level(&app, normal).is_some(), "30 m range sees at 20 m");
    assert_eq!(level(&app, half), None, "15 m range does not");
    assert!(level(&app, half_near).is_some(), "but sees at 10 m");
    // Dropping the profile drops the cached resolve.
    app.world_mut().entity_mut(half).remove::<AiPerceptionProfile>();
    step(&mut app);
    assert!(app.world().get::<AiEffectivePerception>(half).is_none());
}

#[test]
fn same_band_at_the_same_time_at_30_and_144_hz() {
    // Default budget: 0.1 s staggered checks (3 ticks at 30 Hz, 14 at 144 Hz).
    let times = [0.55, 1.0, 1.45, 2.1];
    let run = |hz: f64| -> Vec<AwarenessBand> {
        let mut app = app(hz);
        let obs = observer(&mut app, 0xABCD, Vec3::ZERO);
        target(&mut app, Vec3::new(0.0, 0.0, 28.0));
        let mut ticks = 0u32;
        times
            .iter()
            .map(|&t| {
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                let until = (t * hz).round() as u32;
                while ticks < until {
                    step(&mut app);
                    ticks += 1;
                }
                band(&app, obs)
            })
            .collect()
    };
    let slow = run(30.0);
    let fast = run(144.0);
    assert_eq!(slow, fast);
    assert_eq!(slow, [AwarenessBand::Curious, AwarenessBand::Suspicious, AwarenessBand::Searching, AwarenessBand::Alerted]);
}

#[test]
fn tunable_defaults_match_the_core_defaults() {
    use crate::perception as p;
    let settings = AiPerceptionSettings::default();
    let (a, v, b, q) = (settings.awareness, settings.sight, settings.budget, settings.search);
    assert_eq!(
        [
            p::SIGHT_RANGE_M, p::LOSE_SIGHT_RANGE_M, p::NEAR_M, p::CENTRAL_HALF_DEG, p::PERIPHERAL_HALF_DEG,
            p::PERIPHERAL_MUL, p::RATE, p::SKIP_GAIN, p::SIGHT_PERIOD_S, p::TIER1_PERIOD_S, p::SIGHT_CACHE_S,
            p::CACHE_MOVE_M, p::SEARCH_TTL_S, p::SEARCH_ERROR_M, p::SEARCH_WIDEN_MPS, p::CURIOUS, p::SUSPICIOUS,
            p::SEARCHING, p::BAND_HYSTERESIS, p::ALERT_HOLD_S, p::DECAY_DELAY_S, p::DECAY_CURIOUS,
            p::DECAY_SUSPICIOUS, p::DECAY_SEARCHING, p::NOISE_GAIN, p::NOISE_CAP, p::NOISE_CONFIDENCE,
            p::ASLEEP_WAKE_M, p::GROGGY_S, p::DISTRACTION_TTL_S, p::ALERT_ALERTNESS, p::GROGGY_ALERTNESS,
            p::NOTIFY_SPEED_MPS, p::NOTIFY_REACT_S, p::NOTIFY_CONFIDENCE_MUL,
        ]
        .map(|d| d.default_value()),
        [
            v.sight_range_m, v.lose_sight_range_m, v.near_m, v.central_half_deg, v.peripheral_half_deg,
            v.peripheral_mul, v.rate, b.skip_gain, b.sight_period_s, b.tier1_period_s, b.sight_cache_s,
            b.cache_move_m, q.search_ttl_s, q.search_error_m, q.search_widen_mps, a.curious, a.suspicious,
            a.searching, a.band_hysteresis, a.alert_hold_s, a.decay_delay_s, a.decay_curious, a.decay_suspicious,
            a.decay_searching, a.noise_gain, a.noise_cap, a.noise_confidence, a.asleep_wake_m, a.groggy_s,
            a.distraction_ttl_s, a.alert_alertness, a.groggy_alertness, a.notify_speed_mps, a.notify_react_s,
            a.notify_confidence_mul,
        ]
    );
    assert_eq!(p::MAX_RAYS_PER_TICK.default_value(), b.max_rays_per_tick);
    assert_eq!(p::MAX_ENTRIES.default_value() as usize, a.max_entries);
    assert_eq!(p::NOTIFY_MAX_HOPS.default_value(), u32::from(a.notify_max_hops));
}

#[test]
fn perception_record_kinds_register() {
    let mut db = RecordDb::new();
    register_perception_kinds(&mut db).unwrap();
    let names = db.kind_names();
    assert!(names.contains(&PERCEPTION_PROFILE_KIND) && names.contains(&PERCEPTION_STATE_KIND));
}

#[test]
#[ignore = "measurement: AI8b2 sight ms"]
fn measure_sight_ms_per_tick() {
    let mut app = app(60.0);
    for i in 0..4u8 {
        let a = f32::from(i) * std::f32::consts::FRAC_PI_2;
        app.world_mut().spawn((
            Perceivable { target: PerceptTarget::Player(PlayerKey(u128::from(i))), chest_m: CHEST, head_m: HEAD },
            mover(Vec3::new(a.cos() * 3.0, 0.0, a.sin() * 3.0), 0.0),
        ));
    }
    for i in 0..48u8 {
        let a = f32::from(i) / 48.0 * std::f32::consts::TAU;
        let at = Vec3::new(a.cos() * 15.0, 0.0, a.sin() * 15.0);
        let yaw = (-at.x).atan2(-at.z); // facing the centre
        app.world_mut().spawn((AiActor(ActorKey(1000 + u128::from(i))), AiObserver { eye_m: EYE }, mover(at, yaw)));
    }
    for _ in 0..60 {
        step(&mut app);
    }
    let ticks = 3600u32;
    let mut rays = 0u64;
    let start = Instant::now();
    for _ in 0..ticks {
        step(&mut app);
        rays += u64::from(app.world().resource::<AiRays>().stats.cast);
    }
    let ms = start.elapsed().as_secs_f64() * 1000.0 / f64::from(ticks);
    println!(
        "AI8b2 sight: {ms:.4} ms/tick, 48 observers x 4 targets, fake LOS, 60 Hz, {:.2} rays/tick",
        f64::from(u32::try_from(rays).unwrap_or(u32::MAX)) / f64::from(ticks)
    );
}
