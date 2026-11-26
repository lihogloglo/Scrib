//! Headless tests for hearing through doors (slice AI8c3): the default [`PortalPaths`] over a
//! synthetic [`NavWorldGraph`].
//!
//! Spaces: exterior `hall`, interior `cellar`, interior `vault`; Y up, everything at ear height so
//! legs are integer 3-4-5 triangles. Links (identity frames, two-way): `hc` hall (0, EYE, 4) -
//! cellar origin, `cv` cellar (0, EYE, 3) - vault origin. The noise is at hall (3, EYE, 0): 5 m to
//! the `hc` opening. A cellar listener at (6, 0, 8) is 10 m from it (path 15 m, straight line 8.5
//! m); a vault listener at (0, 0, 4) is 4 m from the `cv` opening (path 5 + 3 + 4 = 12 m).

// Exact assertions on purpose where the legs are integer.
#![allow(clippy::float_cmp)]

use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

use bevy_app::App;
use glam::{Vec2, Vec3};
use sf_ai::{NoiseKindId, Notify};
use sf_coords::WorldTransform;
use sf_nav::DoorState;
use sf_nav_bevy::{NavDoorStates, NavLinkRules, NavWorldGraph};
use sf_world::{DoorLink, LinkEnd, LinkId, Location, WorldspaceId};

use super::*;
use crate::sight_tests::{EYE, PLAYER, app, level, observer, step, wpos};

/// Within float noise of the integer legs.
const EPS: f32 = 1e-4;

fn end(ws: &str, at: Vec3) -> LinkEnd {
    let frame = WorldTransform::from_pos(wpos(at));
    LinkEnd { worldspace: WorldspaceId::new(ws), frame, arrival: frame, door: None, extent: Vec2::splat(2.0) }
}

fn hc() -> DoorLink {
    DoorLink { a: end("hall", Vec3::new(0.0, EYE, 4.0)), b: end("cellar", Vec3::new(0.0, EYE, 0.0)), two_way: true }
}

fn cv() -> DoorLink {
    DoorLink { a: end("cellar", Vec3::new(0.0, EYE, 3.0)), b: end("vault", Vec3::new(0.0, EYE, 0.0)), two_way: true }
}

/// An app with the hearing system over `links`, the `open` ones reported open (the rest
/// unreported: closed).
fn door_app(links: &[DoorLink], open: &[LinkId]) -> App {
    let mut app = app(60.0);
    app.insert_resource(NavWorldGraph(NavLinkRules::default().build(links)));
    app.insert_resource(NavDoorStates(open.iter().map(|&id| (id, DoorState::Open)).collect()));
    app
}

/// A listener at `at` in `space`.
fn listener(app: &mut App, key: u128, at: Vec3, space: &str) -> Entity {
    let e = observer(app, key, at);
    app.world_mut().entity_mut(e).insert(AiExecutor::new(WorldspaceId::new(space)));
    e
}

/// A revealing footstep by [`PLAYER`] at hall (3, EYE, 0).
fn footstep(loudness_m: f32) -> NoiseEvent {
    NoiseEvent {
        tick: 0,
        at: wpos(Vec3::new(3.0, EYE, 0.0)),
        space: Some(WorldspaceId::new("hall")),
        kind: NoiseKindId(0),
        loudness_m,
        source: Some(PLAYER),
        reveals_source: true,
        notify: None,
    }
}

const CELLAR_AT: Vec3 = Vec3::new(6.0, 0.0, 8.0);
const VAULT_AT: Vec3 = Vec3::new(0.0, 0.0, 4.0);

/// Emits `ev`, steps once and returns the listener's level toward [`PLAYER`].
fn hear(app: &mut App, e: Entity, ev: NoiseEvent) -> Option<f32> {
    app.world_mut().write_message(ev);
    step(app);
    level(app, e)
}

/// [`PortalPaths`] from the footstep to the ears of a listener at `at` in `space`, with `open` links
/// open and `max_hops` 3.
fn portal(links: &[DoorLink], open: &[LinkId], at: Vec3, space: &str) -> Option<NoisePath> {
    let graph = NavLinkRules::default().build(links);
    let doors = NavDoorStates(open.iter().map(|&id| (id, DoorState::Open)).collect());
    let door_state = |id: LinkId| doors.state(id);
    let world = NoiseWorld {
        graph: Some(&graph),
        door_state: &door_state,
        closed_door_m: HearingConfig::default().closed_door_m,
        max_hops: 3,
    };
    let from = Location::new(WorldspaceId::new("hall"), footstep(1.0).at);
    let to = Location::new(WorldspaceId::new(space), wpos(at + Vec3::Y * EYE));
    PortalPaths.distance(&from, &to, &world)
}

#[test]
fn a_cellar_listener_hears_the_hall_through_the_open_door_by_path_metres() {
    let path = portal(&[hc()], &[hc().id()], CELLAR_AT, "cellar").unwrap();
    assert!((path.path_m - 15.0).abs() < EPS && path.closed_doors == 0, "{path:?}");

    let mut app = door_app(&[hc()], &[hc().id()]);
    let e = listener(&mut app, 1, CELLAR_AT, "cellar");
    step(&mut app);
    // The straight line (8.5 m) would hear 14.9 m; the path (15 m) does not.
    assert_eq!(hear(&mut app, e, footstep(14.9)), None);
    let l = hear(&mut app, e, footstep(15.1)).expect("heard through the opening");
    let cfg = app.world().resource::<AiPerceptionSettings>().awareness;
    let want = cfg.noise_gain(&sf_ai::HeardNoise {
        source: Some(PLAYER),
        reveals_source: true,
        at: footstep(1.0).at,
        loudness_m: 15.1,
        distance_m: 15.0,
    });
    assert!((l - want.unwrap()).abs() < 1e-4, "{l} vs {want:?}");
}

#[test]
fn a_closed_door_adds_closed_door_m_and_counts_once() {
    let path = portal(&[hc()], &[], CELLAR_AT, "cellar").unwrap();
    assert!((path.path_m - 15.0).abs() < EPS && path.closed_doors == 1, "{path:?}");

    let mut app = door_app(&[hc()], &[]);
    let e = listener(&mut app, 1, CELLAR_AT, "cellar");
    step(&mut app);
    assert_eq!(app.world().resource::<AiPerceptionSettings>().hearing.closed_door_m, 8.0);
    // 15 m + 8 m for the shut door.
    assert_eq!(hear(&mut app, e, footstep(15.1)), None);
    assert_eq!(hear(&mut app, e, footstep(22.9)), None);
    assert!(hear(&mut app, e, footstep(23.1)).is_some());
}

#[test]
fn noise_max_hops_bounds_the_chain_of_doors() {
    let open = [hc().id(), cv().id()];
    let path = portal(&[hc(), cv()], &open, VAULT_AT, "vault").unwrap();
    assert!((path.path_m - 12.0).abs() < EPS && path.closed_doors == 0, "{path:?}");

    for (hops, heard) in [(1, false), (2, true)] {
        let mut app = door_app(&[hc(), cv()], &open);
        app.world_mut().resource_mut::<AiPerceptionSettings>().hearing.max_hops = hops;
        let e = listener(&mut app, 1, VAULT_AT, "vault");
        step(&mut app);
        assert_eq!(hear(&mut app, e, footstep(30.0)).is_some(), heard, "max_hops {hops}");
    }
}

#[test]
fn without_a_graph_other_spaces_are_deaf_and_the_same_space_hears_by_the_straight_line() {
    let mut app = app(60.0);
    let cellar = listener(&mut app, 1, CELLAR_AT, "cellar");
    let hall = listener(&mut app, 2, Vec3::new(3.0, 0.0, 10.0), "hall");
    step(&mut app);
    // Hall listener 10 m away: heard at 10.1, not at 9.9.
    assert_eq!(hear(&mut app, hall, footstep(9.9)), None);
    assert!(hear(&mut app, hall, footstep(10.1)).is_some());
    assert_eq!(hear(&mut app, cellar, footstep(100.0)), None);
}

/// [`PortalPaths`], counting its queries.
struct Counting(Arc<AtomicU32>);

impl NoisePaths for Counting {
    fn distance(&self, from: &Location, to: &Location, world: &NoiseWorld<'_>) -> Option<NoisePath> {
        self.0.fetch_add(1, Ordering::Relaxed);
        PortalPaths.distance(from, to, world)
    }
}

#[test]
fn the_straight_line_culls_same_space_listeners_and_other_spaces_skip_it() {
    let asked = Arc::new(AtomicU32::new(0));
    let mut app = door_app(&[hc()], &[hc().id()]);
    app.insert_resource(NoisePropagation::new(Counting(asked.clone())));
    let far = listener(&mut app, 1, Vec3::new(3.0, 0.0, 30.0), "hall");
    step(&mut app);
    assert_eq!(hear(&mut app, far, footstep(20.0)), None);
    assert_eq!(asked.load(Ordering::Relaxed), 0, "30 m > 20 m reach: culled before the path query");

    // A cellar listener 8.5 m in straight line but 15 m by the door: asked, unheard at 12 m.
    let cellar = listener(&mut app, 2, CELLAR_AT, "cellar");
    step(&mut app);
    assert_eq!(hear(&mut app, cellar, footstep(12.0)), None);
    assert_eq!(asked.load(Ordering::Relaxed), 1);
}

#[test]
fn a_notify_through_the_door_arrives_after_its_path_metres() {
    let mut app = door_app(&[hc()], &[hc().id()]);
    let e = listener(&mut app, 1, CELLAR_AT, "cellar");
    step(&mut app);
    let notify = Notify { about: PLAYER, lkp: wpos(Vec3::new(3.0, 0.0, 0.0)), lkp_tick: 0, confidence: 1.0, hops: 0 };
    let shout = NoiseEvent { source: None, reveals_source: false, notify: Some(notify), ..footstep(60.0) };
    app.world_mut().write_message(shout);
    step(&mut app);
    let clock = *app.world().resource::<AiPerceptClock>();
    let cfg = app.world().resource::<AiPerceptionSettings>().awareness;
    let path = portal(&[hc()], &[hc().id()], CELLAR_AT, "cellar").unwrap().path_m;
    let due = clock.tick + cfg.notify_delay_ticks(path, clock.tick_s).unwrap();
    // 15 m / notify_speed_mps + notify_react_s, in whole ticks.
    let secs = 15.0 / f64::from(cfg.notify_speed_mps) + f64::from(cfg.notify_react_s);
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let want = clock.tick + (secs / f64::from(clock.tick_s)).ceil() as u64;
    assert_eq!(due, want);
    assert_eq!(app.world().get::<AiAwareness>(e).unwrap().0.pending()[0].due, due);
    while level(&app, e).is_none() {
        step(&mut app);
    }
    assert_eq!(app.world().resource::<AiPerceptClock>().tick, due);
}

#[test]
fn noise_max_hops_tunable_default_matches_the_config() {
    assert_eq!(perception::NOISE_MAX_HOPS.default_value(), HearingConfig::default().max_hops);
    let world = NoiseWorld { graph: None, door_state: &|_| DoorState::Closed, closed_door_m: 8.0, max_hops: 3 };
    let at = |ws: &str| Location::new(WorldspaceId::new(ws), wpos(Vec3::ZERO));
    let default = NoisePropagation::default();
    assert!(default.0.distance(&at("a"), &at("b"), &world).is_none(), "no graph: other spaces unheard");
    assert_eq!(default.0.distance(&at("a"), &at("a"), &world), Some(NoisePath { path_m: 0.0, closed_doors: 0 }));
}
