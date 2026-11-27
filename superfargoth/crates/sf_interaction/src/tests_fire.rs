//! `IN17c1` fire heat core on synthetic grids: downwind ignites before crosswind before upwind, a wet
//! row blocks the spread (and without it the fire crosses), rain puts fires out at exactly
//! `interact.fire.rain_douse`, fuel runs out on its exact tick into `Charred` (never relit), and the same
//! inputs in any body order give the same events and heat bits.

use glam::{Vec2, Vec3};
use sf_world::PlacementId;

use crate::*;

const WOOD: Flammable = Flammable { ignite_heat: 1.0, fuel_s: 60.0, burnt_stage: Some(2), spread: true };

fn params() -> InteractParams {
    InteractParams { tick_hz: 60.0, ..InteractParams::default() }
}

fn body(id: u128, x: f32, z: f32) -> FireBody {
    FireBody { placement: PlacementId(id), pos: Vec3::new(x, 0.0, z), flammable: WOOD, wetness: 0.0 }
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
