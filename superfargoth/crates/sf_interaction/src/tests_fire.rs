//! `IN17c1` fire heat core on synthetic grids: downwind ignites before crosswind before upwind, a wet
//! row blocks the spread (and without it the fire crosses), rain puts fires out at exactly
//! `interact.fire.rain_douse`, fuel runs out on its exact tick into `Charred` (never relit), and the same
//! inputs in any body order give the same events and heat bits. `IN17c2` budgets: ignitions queue at
//! `interact.fire.burning_max` per region and drain oldest first, a capped grid burns the same set as an
//! uncapped one (later) within `interact.fire.spread_checks_per_tick`, and a settlement `structure`
//! never catches from spread.

use glam::{Vec2, Vec3};
use sf_world::PlacementId;

use crate::*;

const WOOD: Flammable =
    Flammable { ignite_heat: 1.0, fuel_s: 60.0, burnt_stage: Some(2), spread: true, structure: false };

fn params() -> InteractParams {
    InteractParams { tick_hz: 60.0, ..InteractParams::default() }
}

fn body(id: u128, x: f32, z: f32) -> FireBody {
    let pos = Vec3::new(x, 0.0, z);
    FireBody { placement: PlacementId(id), pos, flammable: WOOD, wetness: 0.0, in_settlement: false }
}

/// `side x side` bodies 2 m apart (ids row-major from 0).
fn grid(side: u32) -> Vec<FireBody> {
    let mut out = Vec::new();
    for r in 0..side {
        for c in 0..side {
            #[allow(clippy::cast_precision_loss)]
            out.push(body(u128::from(r * side + c), c as f32 * 2.0, r as f32 * 2.0));
        }
    }
    out
}

fn light(states: &mut ObjectStates, ids: impl IntoIterator<Item = u128>) {
    for id in ids {
        states.add(PlacementId(id), BURNING, Ttl::UntilRemoved, Tick(0), &params());
    }
}

/// Runs ticks `1..=ticks`, `reorder` permuting the bodies before each step; returns the events with
/// their tick.
fn run(
    fire: &mut Fire,
    states: &mut ObjectStates,
    bodies: &[FireBody],
    env: FireEnv,
    ticks: u64,
    reorder: impl Fn(u64, &mut Vec<FireBody>),
) -> Vec<(u64, FireEvent)> {
    let p = params();
    let mut out = Vec::new();
    let mut b = bodies.to_vec();
    for t in 1..=ticks {
        reorder(t, &mut b);
        out.extend(step_fire(fire, states, &b, &env, Tick(t), &p).into_iter().map(|e| (t, e)));
    }
    out
}

fn ignited_at(events: &[(u64, FireEvent)], id: u128) -> Option<u64> {
    events
        .iter()
        .find(|(_, e)| e.placement == PlacementId(id) && e.output == FireOutput::Ignited)
        .map(|(t, _)| *t)
}

#[test]
fn downwind_ignites_before_crosswind_before_upwind() {
    // Source 0 at the origin, 1 downwind (+x), 2 upwind (-x), 3 crosswind (+z), each 2 m away and
    // more than the 2.5 m reach from each other.
    let bodies = [body(0, 0.0, 0.0), body(1, 2.0, 0.0), body(2, -2.0, 0.0), body(3, 0.0, 2.0)];
    let mut states = ObjectStates::default();
    light(&mut states, [0]);
    let mut fire = Fire::new();
    let env = FireEnv { wind: Vec2::new(8.0, 0.0) };
    let events = run(&mut fire, &mut states, &bodies, env, 40 * 60, |_, _| {});
    let (down, cross, up) =
        (ignited_at(&events, 1).unwrap(), ignited_at(&events, 3).unwrap(), ignited_at(&events, 2).unwrap());
    assert!(down < cross && cross < up, "downwind {down}, crosswind {cross}, upwind {up}");
    let p = params();
    assert!((wind_factor(Vec3::ZERO, Vec3::X, &env, &p) - 1.6).abs() < 1e-6);
    assert!((wind_factor(Vec3::ZERO, Vec3::NEG_X, &env, &p) - 0.4).abs() < 1e-6);
    assert!((wind_factor(Vec3::ZERO, Vec3::Z, &env, &p) - 1.0).abs() < 1e-6);
    assert!((wind_factor(Vec3::ZERO, Vec3::Y, &env, &p) - 1.0).abs() < 1e-6, "straight up: no wind bias");
}

#[test]
fn a_wet_row_blocks_the_spread() {
    // 3 x 3, column 0 burning; diagonals (2.83 m) are out of reach, so column 2 only takes heat from
    // column 1.
    let bodies = grid(3);
    let column = |c: u128| [c, 3 + c, 6 + c];
    let burn = |wet: bool| {
        let mut states = ObjectStates::default();
        light(&mut states, column(0));
        if wet {
            for id in column(1) {
                states.add(PlacementId(id), WET, Ttl::UntilRemoved, Tick(0), &params());
            }
        }
        let mut fire = Fire::new();
        let events = run(&mut fire, &mut states, &bodies, FireEnv::default(), 50 * 60, |_, _| {});
        (events, fire)
    };

    let (events, fire) = burn(true);
    assert!(events.is_empty(), "nothing ignites past the wet column: {events:?}");
    for id in column(1).into_iter().chain(column(2)) {
        assert_eq!(fire.heat(PlacementId(id)).to_bits(), 0.0f32.to_bits(), "wet and out of reach take no heat");
    }

    let (events, _) = burn(false);
    for id in column(1).into_iter().chain(column(2)) {
        assert!(ignited_at(&events, id).is_some(), "dry: {id} ignites");
    }
    for r in 0..3 {
        assert!(ignited_at(&events, 3 * r + 1) < ignited_at(&events, 3 * r + 2), "row {r}: column 1 first");
    }
}

#[test]
fn rain_puts_fires_out_at_the_douse_threshold() {
    let p = params();
    let mut bodies = [body(0, 0.0, 0.0), body(1, 10.0, 0.0), body(2, 20.0, 0.0)];
    bodies[0].wetness = 1.0;
    bodies[1].wetness = p.fire_rain_douse;
    bodies[2].wetness = p.fire_rain_douse - 0.01;
    let mut states = ObjectStates::default();
    light(&mut states, [0, 1, 2]);
    let mut fire = Fire::new();
    let events = step_fire(&mut fire, &mut states, &bodies, &FireEnv::default(), Tick(5), &p);
    assert_eq!(
        events,
        vec![
            FireEvent { placement: PlacementId(0), output: FireOutput::Doused },
            FireEvent { placement: PlacementId(1), output: FireOutput::Doused },
        ]
    );
    for id in [0, 1] {
        assert!(!states.has(PlacementId(id), BURNING) && states.has(PlacementId(id), WET), "{id} out and wet");
        assert_eq!(fire.burning_since(PlacementId(id)), None);
    }
    assert!(states.has(PlacementId(2), BURNING) && !states.has(PlacementId(2), WET), "below the threshold burns on");
    assert_eq!(fire.burning_since(PlacementId(2)), Some(Tick(5)));
    // Wet lasts interact.rules.wet_ttl_s.
    let expiry = Tick(5 + p.ticks(p.rules_wet_ttl_s));
    assert!(states.expire(Tick(expiry.0 - 1)).is_empty());
    assert_eq!(states.expire(expiry), vec![(PlacementId(0), WET), (PlacementId(1), WET)]);
}

#[test]
fn fuel_runs_out_on_its_tick_and_charred_never_relights() {
    let p = params();
    let short = Flammable { fuel_s: 2.0, ..WOOD };
    let mut a = body(0, 0.0, 0.0);
    a.flammable = short;
    let b = body(1, 1.0, 0.0); // burns 60 s next to a
    let bodies = [a, b];
    let mut states = ObjectStates::default();
    light(&mut states, [0, 1]);
    let mut fire = Fire::new();
    let env = FireEnv::default();
    let fuel = p.ticks(2.0);
    for t in 10..10 + fuel {
        assert!(step_fire(&mut fire, &mut states, &bodies, &env, Tick(t), &p).is_empty(), "tick {t}");
    }
    assert_eq!(fire.burning_since(PlacementId(0)), Some(Tick(10)));
    assert_eq!(
        step_fire(&mut fire, &mut states, &bodies, &env, Tick(10 + fuel), &p),
        vec![FireEvent { placement: PlacementId(0), output: FireOutput::Burnt { stage: Some(2) } }]
    );
    assert!(!states.has(PlacementId(0), BURNING) && states.has(PlacementId(0), CHARRED));
    for t in 11 + fuel..11 + fuel + 600 {
        assert!(step_fire(&mut fire, &mut states, &bodies, &env, Tick(t), &p).is_empty(), "tick {t}");
        assert_eq!(fire.heat(PlacementId(0)).to_bits(), 0.0f32.to_bits(), "charred takes no heat");
    }
}

#[test]
fn same_inputs_any_body_order_same_result() {
    let bodies = grid(5);
    let env = FireEnv { wind: Vec2::new(3.0, 1.0) };
    let burn = |reorder: &dyn Fn(u64, &mut Vec<FireBody>)| {
        let mut states = ObjectStates::default();
        light(&mut states, [12]);
        let mut fire = Fire::new();
        let events = run(&mut fire, &mut states, &bodies, env, 30 * 60, reorder);
        let heat: Vec<u32> = (0..25).map(|id| fire.heat(PlacementId(id)).to_bits()).collect();
        let tags: Vec<_> = (0..25).map(|id| states.tags(PlacementId(id))).collect();
        (events, heat, tags)
    };
    let base = burn(&|_, _| {});
    assert!(base.0.iter().filter(|(_, e)| e.output == FireOutput::Ignited).count() >= 4, "the fire spreads");
    let reversed = burn(&|_, b: &mut Vec<FireBody>| b.reverse());
    #[allow(clippy::cast_possible_truncation)]
    let rotated = burn(&|t, b: &mut Vec<FireBody>| {
        let n = b.len();
        b.rotate_left((t as usize * 7) % n);
    });
    assert_eq!(base, reversed);
    assert_eq!(base, rotated);
    // The jitter is per target and per second: the same within a second, different across targets.
    assert_eq!(heat_jitter(PlacementId(3), 9).to_bits(), heat_jitter(PlacementId(3), 9).to_bits());
    assert_ne!(heat_jitter(PlacementId(3), 9).to_bits(), heat_jitter(PlacementId(4), 9).to_bits());
    assert!((-1.0..1.0).contains(&heat_jitter(PlacementId(3), 9)));
}

/// `body` at `(x, z)` shifted to the middle of the region at the origin (64 m squares).
fn mid(id: u128, x: f32, z: f32) -> FireBody {
    body(id, 32.0 + x, 32.0 + z)
}

#[test]
fn ignitions_queue_at_the_region_cap_and_drain_oldest_first() {
    let p = InteractParams { fire_burning_max: 2, fire_jitter: 0.0, ..params() };
    let mut source = mid(0, 0.0, 0.0);
    source.flammable.fuel_s = 6.0;
    // Four targets 1 m from the source, a burning pair in another region.
    let bodies = [
        source,
        mid(1, 1.0, 0.0),
        mid(2, -1.0, 0.0),
        mid(3, 0.0, 1.0),
        mid(4, 0.0, -1.0),
        mid(10, 100.0, 0.0),
        mid(11, 101.0, 0.0),
    ];
    assert_eq!(fire_region(bodies[0].pos, &p), (0, 0));
    assert_eq!(fire_region(bodies[5].pos, &p), (2, 0));
    let mut states = ObjectStates::default();
    light(&mut states, [0, 10]);
    let mut fire = Fire::new();
    let env = FireEnv::default();
    let ignited = |id| FireEvent { placement: PlacementId(id), output: FireOutput::Ignited };
    let mut t = 1;
    let first = loop {
        let events = step_fire(&mut fire, &mut states, &bodies, &env, Tick(t), &p);
        if !events.is_empty() {
            break events;
        }
        t += 1;
        assert!(t < 360, "something ignites before the source burns out");
    };
    // The four reach ignite_heat on the same tick: room for one in region (0, 0), the rest queue.
    assert_eq!(first, vec![ignited(1), ignited(11)], "tick {t}");
    assert_eq!(fire.queued(), vec![PlacementId(2), PlacementId(3), PlacementId(4)]);
    for id in [2, 3, 4] {
        assert!(!states.has(PlacementId(id), BURNING));
        assert_eq!(fire.heat(PlacementId(id)).to_bits(), 0.0f32.to_bits(), "a queued body takes no heat");
    }
    // The source (first seen burning on tick 1) burns out on tick 1 + 6 s: the oldest queued one lights.
    let out = 1 + p.ticks(6.0);
    for t in t + 1..out {
        assert!(step_fire(&mut fire, &mut states, &bodies, &env, Tick(t), &p).is_empty(), "tick {t}");
    }
    assert_eq!(
        step_fire(&mut fire, &mut states, &bodies, &env, Tick(out), &p),
        vec![FireEvent { placement: PlacementId(0), output: FireOutput::Burnt { stage: Some(2) } }, ignited(2)]
    );
    assert_eq!(fire.queued(), vec![PlacementId(3), PlacementId(4)]);
    // A wet body leaves the queue and never lights, even with room.
    states.add(PlacementId(3), WET, Ttl::UntilRemoved, Tick(out), &p);
    states.remove(PlacementId(1), BURNING);
    assert_eq!(step_fire(&mut fire, &mut states, &bodies, &env, Tick(out + 1), &p), vec![ignited(4)]);
    assert!(fire.queued().is_empty() && !states.has(PlacementId(3), BURNING));
}

#[test]
fn a_capped_grid_burns_the_same_set_later() {
    let env = FireEnv { wind: Vec2::new(2.0, 1.0) };
    let bodies: Vec<FireBody> = grid(6)
        .into_iter()
        .map(|mut b| {
            b.flammable.fuel_s = 600.0;
            b
        })
        .collect();
    let burn = |checks: u16| {
        let p = InteractParams { fire_burning_max: 128, fire_spread_checks_per_tick: checks, ..params() };
        let mut states = ObjectStates::default();
        light(&mut states, [0]);
        let mut fire = Fire::new();
        let mut events = Vec::new();
        for t in 1..=300 * 60 {
            let tick = step_fire(&mut fire, &mut states, &bodies, &env, Tick(t), &p);
            events.extend(tick.into_iter().map(|e| (t, e)));
            assert!(fire.checks() <= usize::from(checks), "tick {t}: {} checks", fire.checks());
            if (0..36).all(|id| states.has(PlacementId(id), BURNING)) {
                break;
            }
        }
        events
    };
    let set = |events: &[(u64, FireEvent)]| {
        let mut ids: Vec<u128> = events.iter().map(|(_, e)| e.placement.0).collect();
        ids.sort_unstable();
        ids
    };
    let free = burn(512);
    let capped = burn(16);
    assert_eq!(set(&free), (1..36).collect::<Vec<_>>(), "uncapped: the whole grid ignites");
    assert_eq!(set(&capped), set(&free), "the cap never redirects the fire");
    let last = |events: &[(u64, FireEvent)]| events.last().map(|(t, _)| *t).unwrap();
    assert!(last(&capped) > last(&free), "capped {} vs uncapped {}", last(&capped), last(&free));
    assert_eq!(burn(u16::MAX), free, "room for every check: exactly the uncapped fire");
}

#[test]
fn a_settlement_structure_never_catches_from_spread() {
    let p = params();
    let wall = Flammable { structure: true, ..WOOD };
    let mut in_town = mid(1, 1.0, 0.0);
    in_town.flammable = wall;
    in_town.in_settlement = true;
    let mut in_wild = mid(2, -1.0, 0.0);
    in_wild.flammable = wall;
    let mut crate_in_town = mid(3, 0.0, 1.0);
    crate_in_town.in_settlement = true;
    let bodies = [mid(0, 0.0, 0.0), in_town, in_wild, crate_in_town];
    let mut states = ObjectStates::default();
    light(&mut states, [0]);
    let mut fire = Fire::new();
    let env = FireEnv::default();
    let mut lit = Vec::new();
    for t in 1..=30 * 60 {
        for e in step_fire(&mut fire, &mut states, &bodies, &env, Tick(t), &p) {
            lit.push(e.placement.0);
        }
        assert_eq!(fire.heat(PlacementId(1)).to_bits(), 0.0f32.to_bits(), "tick {t}: the town wall takes no heat");
    }
    lit.sort_unstable();
    assert_eq!(lit, vec![2, 3], "the wild wall and the town crate catch, the town wall never");
    // An authored stage still burns it.
    states.add(PlacementId(1), BURNING, Ttl::UntilRemoved, Tick(30 * 60), &p);
    step_fire(&mut fire, &mut states, &bodies, &env, Tick(30 * 60 + 1), &p);
    assert_eq!(fire.burning_since(PlacementId(1)), Some(Tick(30 * 60 + 1)));
}
