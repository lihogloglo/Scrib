//! Headless tests for the hearing system (slice AI8c1): the reach edge, a closed door through a fake
//! propagation, a profile's and the weather's reach, distraction vs source entry, a shout's notify
//! on the core's tick, tick-rate invariance.

// Exact assertions on purpose (same inputs, same maths, same bits).
#![allow(clippy::float_cmp)]

use bevy_app::App;
use glam::Vec3;
use sf_ai::{HeardNoise, NoiseKindId, Notify, PerceptTarget, PerceptionProfile};
use sf_controller_bevy::Mover;
use sf_coords::WorldPos;
use sf_ids::ActorKey;
use sf_world::Location;

use super::*;
use crate::sight_tests::{EYE, PLAYER, app, level, observer, step, wpos};

const KIND: NoiseKindId = NoiseKindId(0);

/// A revealing footstep by [`PLAYER`] `d` meters ahead of a listener at the origin (+Z, ear height).
fn footstep(loudness_m: f32, d: f32) -> NoiseEvent {
    NoiseEvent {
        tick: 0,
        at: wpos(Vec3::new(0.0, EYE, d)),
        space: None,
        kind: KIND,
        loudness_m,
        source: Some(PLAYER),
        reveals_source: true,
        notify: None,
    }
}

fn emit(app: &mut App, ev: NoiseEvent) {
    app.world_mut().write_message(ev);
}

fn ears(app: &App, e: Entity) -> WorldPos {
    app.world().get::<Mover>(e).unwrap().loco.position.offset(Vec3::Y * EYE)
}

/// Emits `ev`, steps once and returns the listener's level toward [`PLAYER`].
fn hear(app: &mut App, listener: Entity, ev: NoiseEvent) -> Option<f32> {
    emit(app, ev);
    step(app);
    level(app, listener)
}

#[test]
fn heard_just_inside_reach_not_just_outside() {
    let mut out = app(60.0);
    let near = observer(&mut out, 1, Vec3::ZERO);
    step(&mut out);
    assert_eq!(hear(&mut out, near, footstep(20.0, 20.1)), None);

    let mut app = app(60.0);
    let near = observer(&mut app, 1, Vec3::ZERO);
    let far = observer(&mut app, 2, Vec3::new(100.0, 0.0, 0.0));
    step(&mut app);
    let ev = footstep(20.0, 19.9);
    let at = ev.at;
    assert!(hear(&mut app, near, ev).is_some());
    // Exactly the core's gain for the attenuated reach and the straight-line distance.
    let cfg = app.world().resource::<AiPerceptionSettings>().awareness;
    let d = at.delta_f32(ears(&app, near)).length();
    let noise = HeardNoise { source: Some(PLAYER), reveals_source: true, at, loudness_m: 20.0, distance_m: d };
    assert_eq!(level(&app, near), cfg.noise_gain(&noise));
    assert_eq!(level(&app, far), None);
}

/// Straight line plus one closed door on every path.
struct OneDoor;

impl NoisePaths for OneDoor {
    fn distance(&self, from: &Location, to: &Location, _world: &NoiseWorld<'_>) -> Option<NoisePath> {
        Some(NoisePath { path_m: from.pos.delta_f32(to.pos).length(), closed_doors: 1 })
    }
}

#[test]
fn a_closed_door_makes_a_heard_noise_unheard() {
    // 5 m in the open is heard at 10 m of reach; 5 + closed_door_m (8) = 13 m is not.
    let mut open = app(60.0);
    let a = observer(&mut open, 1, Vec3::ZERO);
    step(&mut open);
    assert!(hear(&mut open, a, footstep(10.0, 5.0)).is_some());

    let mut shut = app(60.0);
    shut.insert_resource(NoisePropagation::new(OneDoor));
    let b = observer(&mut shut, 1, Vec3::ZERO);
    step(&mut shut);
    assert_eq!(shut.world().resource::<AiPerceptionSettings>().hearing.closed_door_m, 8.0);
    assert_eq!(hear(&mut shut, b, footstep(10.0, 5.0)), None);
    // Through the door, 4 m: 4 + 8 = 12 > 10 still unheard; 1 m: 9 ≤ 10 heard.
    assert_eq!(hear(&mut shut, b, footstep(10.0, 4.0)), None);
    assert!(hear(&mut shut, b, footstep(10.0, 1.0)).is_some());
}

#[test]
fn half_hearing_profile_halves_reach() {
    let mut app = app(60.0);
    let normal = observer(&mut app, 1, Vec3::ZERO);
    let half = observer(&mut app, 2, Vec3::new(100.0, 0.0, 0.0));
    app.world_mut().entity_mut(half).insert(AiPerceptionProfile(PerceptionProfile { hearing: 0.5, ..Default::default() }));
    step(&mut app);
    assert_eq!(app.world().get::<AiEffectivePerception>(half).unwrap().0.hearing_mul, 0.5);

    let at_half = |d: f32| NoiseEvent { at: wpos(Vec3::new(100.0, EYE, d)), ..footstep(20.0, d) };
    assert_eq!(hear(&mut app, half, at_half(10.1)), None);
    assert!(hear(&mut app, half, at_half(9.9)).is_some());
    assert!(hear(&mut app, normal, footstep(20.0, 10.1)).is_some());
}

#[test]
fn weather_mask_halves_reach() {
    let mut app = app(60.0);
    app.insert_resource(AiWeatherMask(0.5));
    let a = observer(&mut app, 1, Vec3::ZERO);
    step(&mut app);
    assert_eq!(hear(&mut app, a, footstep(20.0, 10.1)), None);
    assert!(hear(&mut app, a, footstep(20.0, 9.9)).is_some());
}

#[test]
fn non_revealing_noise_fills_the_distraction_not_the_source() {
    let mut app = app(60.0);
    let a = observer(&mut app, 1, Vec3::ZERO);
    step(&mut app);
    let ev = NoiseEvent { reveals_source: false, ..footstep(20.0, 5.0) };
    let at = ev.at;
    assert_eq!(hear(&mut app, a, ev), None);
    let d = app.world().get::<AiAwareness>(a).unwrap().0.distraction().copied().unwrap();
    assert_eq!((d.at, d.loudness_m), (at, 20.0));
}

#[test]
fn an_actor_never_hears_itself() {
    let mut app = app(60.0);
    let a = observer(&mut app, 7, Vec3::ZERO);
    step(&mut app);
    let ev = NoiseEvent { source: Some(PerceptTarget::Actor(ActorKey(7))), ..footstep(20.0, 1.0) };
    emit(&mut app, ev);
    step(&mut app);
    let meter = &app.world().get::<AiAwareness>(a).unwrap().0;
    assert!(meter.entries().is_empty() && meter.distraction().is_none());
}

/// A guard's shout `d` m ahead of the listener, about [`PLAYER`] (no source: the shout's own sound
/// is a distraction at the shouter).
fn shout(d: f32, at_x: f32) -> NoiseEvent {
    let notify = Notify { about: PLAYER, lkp: wpos(Vec3::new(at_x, 0.0, d + 3.0)), lkp_tick: 0, confidence: 1.0, hops: 0 };
    NoiseEvent {
        at: wpos(Vec3::new(at_x, EYE, d)),
        source: None,
        reveals_source: false,
        notify: Some(notify),
        ..footstep(60.0, d)
    }
}

/// Game time the shout is made at, seconds (a whole number of ticks at 30 and 144 Hz).
const SHOUT_AT_S: f64 = 0.5;

/// A shout made 24 m away on the fixed tick at [`SHOUT_AT_S`] (written just before that tick's
/// hearing, as a fixed-tick emitter does); steps until its notify lands. Returns (heard tick,
/// delivery tick, the core's due tick, the level delivered).
fn shout_delivery(hz: f64) -> (u64, u64, u64, f32) {
    let mut app = app(hz);
    let a = observer(&mut app, 1, Vec3::ZERO);
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let made = (SHOUT_AT_S * hz).round() as u64;
    for _ in 1..made {
        step(&mut app);
    }
    emit(&mut app, shout(24.0, 0.0));
    step(&mut app);
    let clock = *app.world().resource::<AiPerceptClock>();
    assert_eq!(clock.tick, made);
    let cfg = app.world().resource::<AiPerceptionSettings>().awareness;
    let d = wpos(Vec3::new(0.0, EYE, 24.0)).delta_f32(ears(&app, a)).length();
    let due = clock.tick + cfg.notify_delay_ticks(d, clock.tick_s).unwrap();
    assert_eq!(app.world().get::<AiAwareness>(a).unwrap().0.pending()[0].due, due);
    loop {
        if let Some(l) = level(&app, a) {
            let tick = app.world().resource::<AiPerceptClock>().tick;
            return (clock.tick, tick, due, l);
        }
        step(&mut app);
    }
}

#[test]
fn a_shouts_notify_lands_on_the_cores_tick() {
    let (heard, landed, due, l) = shout_delivery(60.0);
    assert!(due > heard);
    assert_eq!(landed, due);
    let cfg = AiPerceptionSettings::default().awareness;
    assert_eq!(l, (1.0 * cfg.notify_confidence_mul).min(cfg.searching));
}

#[test]
fn shout_and_footstep_land_on_the_same_game_time_at_30_and_144_hz() {
    let (_, l30, _, _) = shout_delivery(30.0);
    let (_, l144, _, _) = shout_delivery(144.0);
    #[allow(clippy::cast_precision_loss)]
    let secs = |t: u64, hz: f64| t as f64 / hz;
    assert!((secs(l30, 30.0) - secs(l144, 144.0)).abs() < 1.0 / 30.0, "{l30} @30 vs {l144} @144");
    // A footstep is heard on the tick it is emitted at both rates.
    for hz in [30.0, 144.0] {
        let mut app = app(hz);
        let a = observer(&mut app, 1, Vec3::ZERO);
        step(&mut app);
        assert!(hear(&mut app, a, footstep(20.0, 5.0)).is_some());
    }
}

#[test]
fn closed_door_tunable_default_matches_the_config() {
    assert_eq!(perception::CLOSED_DOOR_M.default_value(), HearingConfig::default().closed_door_m);
    assert_eq!(AiPerceptionSettings::default().hearing, HearingConfig::default());
    assert_eq!(AiWeatherMask::default().0, 1.0);
}
