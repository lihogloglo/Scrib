//! Headless App tests of the search mode (`AI8f2`, [`crate::search`]) on the wander drive's harness
//! ([`super::drive_tests`]: the real [`super::drive`] over a flat 40 m navmesh room) plus
//! `sf_ai_bevy`'s perception plugin (its clock, settings and meter decay). The walker is an observer
//! as `attach` makes it; its meter is raised to a band with `raise_to_floor` (what a loaded search
//! does), so the tests need no sight target.

use bevy_app::App;
use bevy_math::Vec3;
use sf_ai::{AwarenessBand, LastSeen, PerceptTarget};
use sf_ai_bevy::{AiActor, AiAwareness, AiPerceptClock, AiPerceptionSettings, AiSightPlugin};
use sf_controller_bevy::{Mover, MoverParams};
use sf_coords::WorldPos;
use sf_ids::{ActorKey, PlayerKey};

use super::Walker;
use super::drive_tests::{Floor, app_with, home, nav_ws, spawn_wanderer};
use crate::search::Searcher;

/// Fixed ticks per second (the harness's 60 Hz).
const HZ: usize = 60;
const TARGET: PerceptTarget = PerceptTarget::Player(PlayerKey(1));
/// The last known position: 6 m east of the wanderer's home.
const LKP_OFFSET: Vec3 = Vec3::new(6.0, 0.0, 0.0);
/// A short search for the tests, s.
const TTL_S: f32 = 20.0;
/// [`TTL_S`] in ticks.
const TTL_TICKS: usize = 20 * HZ;

/// `ticks` in seconds.
fn secs(ticks: usize) -> f32 {
    f32::from(u16::try_from(ticks).unwrap()) / 60.0
}

/// The drive harness with perception; `hold_band` turns awareness decay off (the band stays
/// until the test changes it).
fn app(hold_band: bool) -> App {
    let mut app = app_with(|app| {
        app.add_plugins(AiSightPlugin::<Floor>::default());
    });
    let mut s = app.world_mut().resource_mut::<AiPerceptionSettings>();
    s.search.search_ttl_s = TTL_S;
    if hold_band {
        s.awareness.decay_delay_s = 1.0e6;
    }
    app
}

fn spawn(app: &mut App) -> bevy_ecs::entity::Entity {
    let e = spawn_wanderer(app, &nav_ws(), home(), 1, 11);
    app.world_mut()
        .entity_mut(e)
        .insert((AiActor(ActorKey(1)), crate::sight::observer(MoverParams::default().height)));
    e
}

fn lkp() -> WorldPos {
    home().offset(LKP_OFFSET)
}

/// Raises `e`'s awareness of [`TARGET`] to `band`'s floor at [`lkp`] (a sighting: confidence 1).
fn raise(app: &mut App, e: bevy_ecs::entity::Entity, band: AwarenessBand) {
    let now = app.world().resource::<AiPerceptClock>().clock();
    let cfg = app.world().resource::<AiPerceptionSettings>().awareness;
    let seen = LastSeen { pos: lkp(), tick: now.tick, velocity: Vec3::ZERO, confidence: 1.0 };
    let mut meter = app.world_mut().get_mut::<AiAwareness>(e).unwrap();
    let _ = meter.0.raise_to_floor(&cfg, TARGET, band, seen, now);
    assert_eq!(meter.0.band(TARGET), band);
}

fn planar(a: WorldPos, b: WorldPos) -> f32 {
    a.delta_f32(b).with_y(0.0).length()
}

/// Sanity of one tick: finite, on the floor, inside the room.
fn sane(app: &App, e: bevy_ecs::entity::Entity, tick: usize) -> (WorldPos, f32) {
    let mover = app.world().get::<Mover>(e).unwrap();
    let pos = mover.loco.position;
    let off = home().delta_f32(pos);
    assert!(off.is_finite() && mover.loco.motion.yaw.is_finite(), "tick {tick}: NaN");
    assert!(off.y.abs() < 0.5, "tick {tick}: left the floor ({off:?})");
    (pos, mover.loco.motion.yaw)
}

#[test]
fn a_searching_walker_goes_to_the_lkp_looks_widens_and_gives_up_at_ttl() {
    let mut app = app(true);
    let e = spawn(&mut app);
    for _ in 0..30 {
        app.update();
    }
    raise(&mut app, e, AwarenessBand::Searching);
    let arrivals_before = app.world().get::<Walker>(e).unwrap().driver.arrival_count();

    let mut first_search_tick = None;
    let mut lkp_tick = None;
    let mut lkp_dist = f32::NAN;
    let mut yaw_at_lkp = None;
    let mut turned = 0.0_f32;
    let mut max_from_lkp = 0.0_f32;
    let mut end_tick = None;
    let ttl_ticks = TTL_TICKS;
    // ttl plus 30 s of wander after it.
    for tick in 0..(ttl_ticks + 30 * HZ) {
        app.update();
        let (pos, yaw) = sane(&app, e, tick);
        let s = app.world().get::<Searcher>(e).unwrap();
        if s.is_searching() {
            first_search_tick.get_or_insert(tick);
            max_from_lkp = max_from_lkp.max(planar(pos, lkp()));
            if lkp_tick.is_none() && s.stats.reached >= 1 {
                lkp_tick = Some(tick);
                lkp_dist = planar(pos, lkp());
                yaw_at_lkp = Some(yaw);
            }
            if let Some(y0) = yaw_at_lkp
                && s.stats.reached == 1
            {
                turned = turned.max(super::drive_tests::wrap(yaw - y0).abs());
            }
        } else if first_search_tick.is_some() && end_tick.is_none() {
            end_tick = Some(tick);
        }
    }
    let world = app.world();
    let s = world.get::<Searcher>(e).unwrap();
    let walker = world.get::<Walker>(e).unwrap();
    let stats = s.stats;
    let (first, lkp_tick, end) = (first_search_tick.unwrap(), lkp_tick.unwrap(), end_tick.unwrap());
    let wander_arrivals = walker.driver.arrival_count() - arrivals_before;
    println!(
        "AI8f2 search: started tick {first}, LKP at tick {lkp_tick} ({lkp_dist:.2} m off, {:.1} s), turned \
         {turned:.2} rad there, {} goals / {} reached / {} failed, {} looks, max {max_from_lkp:.1} m from the LKP, \
         ended tick {end} (ttl {ttl_ticks}), {wander_arrivals} wander arrivals after",
        secs(lkp_tick),
        stats.goals,
        stats.reached,
        stats.failed,
        stats.looks,
    );
    assert_eq!(first, 0, "the search did not interrupt the wander at once");
    assert!(lkp_dist <= 1.0, "reached the LKP {lkp_dist} m off");
    assert!(lkp_tick < ttl_ticks, "never reached the LKP before the ttl");
    assert!(turned > 0.5, "no look-around at the LKP ({turned} rad)");
    assert!(stats.looks >= 1, "{stats:?}");
    assert!(stats.reached >= 2, "no ring point visited: {stats:?}");
    // Radii only grow: the farthest ring is r(ttl) = error + widen x ttl (confidence 1).
    let r_max = 3.0 + 0.5 * TTL_S;
    assert!(max_from_lkp <= r_max + 1.0, "{max_from_lkp} m from the LKP (r(ttl) = {r_max})");
    assert_eq!((stats.started, stats.gave_up, stats.dropped), (1, 1, 0), "{stats:?}");
    // Gave up within a tick of the ttl (perception and drive tick together).
    assert!(end.abs_diff(ttl_ticks) <= 2, "ended on tick {end}, ttl {ttl_ticks}");
    // The band is still Searching (decay held): no restart without fresh evidence; wander resumed.
    assert_eq!(world.get::<AiAwareness>(e).unwrap().0.band(TARGET), AwarenessBand::Searching);
    assert!(wander_arrivals >= 1, "the wander did not resume after the search");
}

#[test]
fn a_curious_walker_keeps_wandering() {
    let mut app = app(true);
    let e = spawn(&mut app);
    for _ in 0..30 {
        app.update();
    }
    raise(&mut app, e, AwarenessBand::Curious);
    let before = app.world().get::<Walker>(e).unwrap().driver.arrival_count();
    for tick in 0..(30 * HZ) {
        app.update();
        sane(&app, e, tick);
        assert!(!app.world().get::<Searcher>(e).unwrap().is_searching(), "tick {tick}: a Curious walker searched");
    }
    let world = app.world();
    let arrivals = world.get::<Walker>(e).unwrap().driver.arrival_count() - before;
    assert_eq!(world.get::<Searcher>(e).unwrap().stats.started, 0);
    assert!(arrivals >= 1, "the wander stopped ({arrivals} arrivals in 30 s)");
}

#[test]
fn a_running_search_holds_searching_to_its_ttl_then_decays_out_and_wanders() {
    // AI8f3. Default decay alone would leave the Searching floor after decay_delay_s + hysteresis /
    // decay_searching (3 + 0.05 / 0.02 = 5.5 s), before the 20 s ttl: the search holds the band
    // instead; once it gives up the entry decays out in 0.05 / 0.02 = 2.5 s (the delay is long past).
    let mut app = app(false);
    let e = spawn(&mut app);
    for _ in 0..30 {
        app.update();
    }
    raise(&mut app, e, AwarenessBand::Searching);
    let cfg = app.world().resource::<AiPerceptionSettings>().awareness;
    let before = app.world().get::<Walker>(e).unwrap().driver.arrival_count();
    let (mut end_tick, mut exit_tick, mut arrivals_at_exit) = (None, None, 0);
    for tick in 0..(TTL_TICKS + 30 * HZ) {
        app.update();
        sane(&app, e, tick);
        let world = app.world();
        let searching = world.get::<Searcher>(e).unwrap().is_searching();
        let entry = world.get::<AiAwareness>(e).unwrap().0.get(TARGET).copied();
        let band = entry.map_or(AwarenessBand::Unaware, |a| a.band);
        if tick == 0 {
            assert!(searching, "the search did not start");
        }
        if searching {
            let a = entry.expect("a searched target is tracked");
            assert!(a.search_held(), "tick {tick}: the search does not hold its entry");
            assert!(
                a.band == AwarenessBand::Searching && a.level >= cfg.searching,
                "tick {tick}: the band decayed under a running search ({a:?})"
            );
        } else if end_tick.is_none() {
            end_tick = Some(tick);
        }
        if let Some(end) = end_tick {
            assert!(!searching, "tick {tick}: searched again without fresh evidence");
            assert!(entry.is_none_or(|a| !a.search_held()), "tick {tick}: the hold outlived the search");
            if tick == end {
                assert_eq!(band, AwarenessBand::Searching, "the band left before the search gave up");
            }
            if band != AwarenessBand::Searching && exit_tick.is_none() {
                exit_tick = Some(tick);
                arrivals_at_exit = world.get::<Walker>(e).unwrap().driver.arrival_count() - before;
            }
        }
    }
    let world = app.world();
    let stats = world.get::<Searcher>(e).unwrap().stats;
    let end = end_tick.expect("the search never ended");
    let exit = exit_tick.expect("the band never decayed out of Searching");
    let arrivals = world.get::<Walker>(e).unwrap().driver.arrival_count() - before;
    println!(
        "AI8f3 hold: search gave up on tick {end} ({:.2} s, ttl {TTL_TICKS}), band left Searching on tick {exit} \
         ({:.2} s after), {stats:?}, {arrivals_at_exit} -> {arrivals} wander arrivals",
        secs(end),
        secs(exit - end),
    );
    assert_eq!((stats.started, stats.gave_up, stats.dropped), (1, 1, 0), "{stats:?}");
    assert!(end.abs_diff(TTL_TICKS) <= 2, "ended on tick {end}, ttl {TTL_TICKS}");
    // 2.5 s = 150 ticks of decay from the release tick (f32 steps: within a tick).
    assert!((149..=152).contains(&(exit - end)), "left Searching {} ticks after the give-up", exit - end);
    assert!(arrivals >= 1, "the wander did not resume");
}
