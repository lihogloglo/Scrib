//! ai.md "Tests" for AI7: exact curve values, compensated products, switch ticks, tie order, NaN
//! inputs, load reports, `ArbiterState` round trips and the traced breakdown of one decision.
//! Floats are compared exactly where the arithmetic is exact (dyadic values), with a 1e-6
//! tolerance against the closed form otherwise.
#![allow(clippy::float_cmp)]

use sf_records::{RecordDb, RecordFormat, RecordId, apply_record_file};

use crate::drive::DriveLoadError as E;
use crate::htn::At;
use crate::*;

/// Inputs in registration order: `[threat_m, health, has_directive, alarm]`.
const THREAT: usize = 0;
const HEALTH: usize = 1;
const CONTENT: usize = 2;
const ALARM: usize = 3;

/// A guard: content (the directive drive, raw 0.25 lifted to a floor of 1), combat (near threat
/// x alarm, weight 2, momentum 0.25, 10-tick commit) and flee (hurt, weight 2).
const DRIVES: &str = r#"[
  define(kind: "sf.ai.consideration", id: "c_near", value: (input: "threat_m", range: (20.0, 0.0),
    curve: Polynomial(slope: 1.0, exponent: 1.0, x_shift: 0.0, y_shift: 0.0))),
  define(kind: "sf.ai.consideration", id: "c_alarm", value: (input: "alarm", range: (0.0, 1.0),
    curve: Polynomial(slope: 1.0, exponent: 1.0, x_shift: 0.0, y_shift: 0.0))),
  define(kind: "sf.ai.consideration", id: "c_hurt", value: (input: "health", range: (1.0, 0.0),
    curve: Polynomial(slope: 1.0, exponent: 2.0, x_shift: 0.0, y_shift: 0.0))),
  define(kind: "sf.ai.consideration", id: "c_content", value: (input: "has_directive", range: (0.0, 1.0),
    curve: Piecewise([(0.0, 0.0), (1.0, 0.25)]))),
  define(kind: "sf.ai.drive", id: "flee", value: (considerations: ["c_hurt"], weight: 2.0, momentum: 0.0,
    commit_ticks: 0)),
  define(kind: "sf.ai.drive", id: "directive", value: (considerations: ["c_content"], weight: 1.0,
    momentum: 0.0, commit_ticks: 0, floor: Some(1.0), directive: true)),
  define(kind: "sf.ai.drive", id: "combat", value: (considerations: ["c_near", "c_alarm"], weight: 2.0,
    momentum: 0.25, commit_ticks: 10)),
]"#;

fn inputs() -> Inputs {
    let mut inputs = Inputs::new();
    for name in ["threat_m", "health", "has_directive", "alarm"] {
        inputs.add(name);
    }
    inputs
}

fn load(text: &str) -> Result<Drives, DriveLoadReport> {
    let mut db = RecordDb::new();
    register_drive_kinds(&mut db).unwrap();
    let src = db.add_source("test", &[]);
    let outcome = apply_record_file(&mut db, src, text, RecordFormat::Ron).unwrap();
    assert_eq!(outcome.errors, []);
    Drives::build(&db, &inputs())
}

fn drives() -> Drives {
    load(DRIVES).unwrap()
}

const COMBAT: DriveId = DriveId(0);
const DIRECTIVE: DriveId = DriveId(1);
const FLEE: DriveId = DriveId(2);

/// World values: threat distance (m), health ratio, a directive to serve, alarm.
fn world(threat_m: f32, health: f32, content: bool, alarm: f32) -> [f32; 4] {
    let mut v = [0.0; 4];
    v[THREAT] = threat_m;
    v[HEALTH] = health;
    v[CONTENT] = if content { 1.0 } else { 0.0 };
    v[ALARM] = alarm;
    v
}

fn close(a: f32, b: f64) {
    assert!((f64::from(a) - b).abs() < 1e-6, "{a} vs {b}");
}

// --- Curves ------------------------------------------------------------------------------------

#[test]
fn polynomial_curves_are_exact() {
    let poly = |slope, exponent, x_shift, y_shift| Curve::Polynomial { slope, exponent, x_shift, y_shift };
    assert_eq!(poly(1.0, 1.0, 0.0, 0.0).eval(0.25), 0.25);
    assert_eq!(poly(1.0, 2.0, 0.0, 0.0).eval(0.5), 0.25);
    assert_eq!(poly(-1.0, 1.0, 0.0, 1.0).eval(0.25), 0.75);
    // Clamped to 0..=1 on both sides.
    assert_eq!(poly(2.0, 1.0, 0.0, 0.0).eval(0.75), 1.0);
    assert_eq!(poly(1.0, 1.0, 0.0, -0.5).eval(0.25), 0.0);
    // A fractional exponent: right of the shift exact, left of it NaN, so 0.
    assert_eq!(poly(1.0, 0.5, 0.5, 0.0).eval(0.75), 0.5);
    assert_eq!(poly(1.0, 0.5, 0.5, 0.0).eval(0.25), 0.0);
    // An integer exponent of a negative base is defined.
    assert_eq!(poly(1.0, 2.0, 0.5, 0.0).eval(0.0), 0.25);
}

#[test]
fn logistic_curves_are_exact() {
    let s = Curve::Logistic { slope: 1.0, steepness: 10.0, x_shift: 0.5, y_shift: 0.0 };
    assert_eq!(s.eval(0.5), 0.5);
    close(s.eval(0.0), 1.0 / (1.0 + 5.0_f64.exp()));
    close(s.eval(0.75), 1.0 / (1.0 + (-2.5_f64).exp()));
    // Saturation: e^-500 underflows to 0, e^500 to infinity.
    let step = Curve::Logistic { slope: 1.0, steepness: 1000.0, x_shift: 0.5, y_shift: 0.0 };
    assert_eq!(step.eval(1.0), 1.0);
    assert_eq!(step.eval(0.0), 0.0);
    // Falling, and shifted down past 0.
    let fall = Curve::Logistic { slope: 1.0, steepness: -10.0, x_shift: 0.5, y_shift: -0.25 };
    assert_eq!(fall.eval(0.5), 0.25);
}

#[test]
fn logit_curves_are_exact() {
    let l = Curve::Logit { slope: 0.1, x_shift: 0.0, y_shift: 0.5 };
    assert_eq!(l.eval(0.5), 0.5);
    assert_eq!(l.eval(0.0), 0.0);
    assert_eq!(l.eval(1.0), 1.0);
    close(l.eval(0.75), 0.1 * 3.0_f64.ln() + 0.5);
    // Negative slope reverses the ends; slope 0 at an end is NaN, so 0.
    let rev = Curve::Logit { slope: -0.1, x_shift: 0.0, y_shift: 0.5 };
    assert_eq!(rev.eval(0.0), 1.0);
    assert_eq!(rev.eval(1.0), 0.0);
    assert_eq!(Curve::Logit { slope: 0.0, x_shift: 0.0, y_shift: 0.5 }.eval(0.0), 0.0);
    assert_eq!(Curve::Logit { slope: 0.0, x_shift: 0.0, y_shift: 0.5 }.eval(0.5), 0.5);
}

#[test]
fn piecewise_curves_are_exact() {
    let p = Curve::Piecewise(vec![(0.0, 0.0), (0.5, 1.0), (1.0, 0.5)]);
    assert_eq!(p.eval(0.0), 0.0);
    assert_eq!(p.eval(0.25), 0.5);
    assert_eq!(p.eval(0.5), 1.0);
    assert_eq!(p.eval(0.75), 0.75);
    assert_eq!(p.eval(1.0), 0.5);
    // Flat outside the points, clamped to 0..=1.
    let q = Curve::Piecewise(vec![(0.25, 0.5), (0.75, 2.0)]);
    assert_eq!(q.eval(0.0), 0.5);
    assert_eq!(q.eval(1.0), 1.0);
    assert_eq!(Curve::Piecewise(vec![(0.5, 0.25)]).eval(0.9), 0.25);
}

#[test]
fn nan_scores_zero_everywhere() {
    let curves = [
        Curve::Polynomial { slope: 1.0, exponent: 1.0, x_shift: 0.0, y_shift: 1.0 },
        Curve::Logistic { slope: 1.0, steepness: 10.0, x_shift: 0.5, y_shift: 0.5 },
        Curve::Logit { slope: 0.1, x_shift: 0.0, y_shift: 0.5 },
        Curve::Piecewise(vec![(0.0, 1.0)]),
    ];
    for c in &curves {
        assert_eq!(c.eval(f32::NAN), 0.0, "{c:?}");
    }
    let d = drives();
    let near = d.consideration(d.consideration_id("c_near").unwrap()).unwrap();
    assert_eq!(near.eval(10.0), 0.5);
    assert_eq!(near.eval(25.0), 0.0); // clamped to the far bookend
    assert_eq!(near.eval(-5.0), 1.0);
    assert_eq!(near.eval(f32::NAN), 0.0);
    assert_eq!(near.eval(f32::INFINITY), 0.0);
    // A NaN input vetoes its drive; a slice shorter than the vocabulary reads NaN.
    let mut w = world(10.0, 1.0, true, 1.0);
    w[THREAT] = f32::NAN;
    assert_eq!(d.raw(COMBAT, &w), 0.0);
    assert_eq!(d.raw(DIRECTIVE, &[]), 0.0);
    let idle = arbitrate(&d, &[], &ArbiterState::default(), 3, None);
    assert_eq!((idle.drive, idle.reason), (None, DriveReason::Idle));
    assert_eq!(idle.scores, [Scored { raw: 0.0, score: 0.0 }; 3]);
}

// --- Scoring -----------------------------------------------------------------------------------

#[test]
fn compensated_product_is_exact() {
    let d = drives();
    // Two considerations (factor 1/2): 0.5 -> 0.625, 1 -> 1; x weight 2.
    assert_eq!(d.raw(COMBAT, &world(10.0, 1.0, false, 1.0)), 1.25);
    // One consideration: no compensation. Health 0.5 -> x 0.5 -> 0.25, x 2.
    assert_eq!(d.raw(FLEE, &world(100.0, 0.5, false, 0.0)), 0.5);
    // Any zero factor zeroes the drive.
    assert_eq!(d.raw(COMBAT, &world(10.0, 1.0, false, 0.0)), 0.0);

    // Four considerations at 0.5 (factor 3/4): 0.6875^4 = 14641/65536, x weight 1.
    let mut inputs = Inputs::new();
    inputs.add("x");
    let half = ConsiderationDef {
        input: "x".into(),
        range: (0.0, 1.0),
        curve: Curve::Polynomial { slope: 1.0, exponent: 1.0, x_shift: 0.0, y_shift: 0.0 },
    };
    let four = DriveDef {
        considerations: vec!["h".into(); 4],
        weight: 1.0,
        momentum: 0.0,
        commit_ticks: 0,
        floor: None,
        directive: false,
    };
    let none = DriveDef { considerations: Vec::new(), weight: 3.0, ..four.clone() };
    let d = Drives::from_defs([(RecordId::new("h"), &half)], [("four".into(), &four), ("none".into(), &none)], &inputs)
        .unwrap();
    assert_eq!(d.raw(d.drive_id("four").unwrap(), &[0.5]), 14641.0 / 65536.0);
    // No considerations: the weight.
    assert_eq!(d.raw(d.drive_id("none").unwrap(), &[0.5]), 3.0);
}

// --- Arbitration -------------------------------------------------------------------------------

fn step(d: &Drives, w: &[f32], state: &mut ArbiterState, tick: u64) -> (Option<DriveId>, DriveReason) {
    let a = arbitrate(d, w, state, tick, None);
    *state = a.state;
    (a.drive, a.reason)
}

#[test]
fn reaction_switches_after_commit_and_only_past_momentum() {
    let d = drives();
    let mut s = ArbiterState::default();
    // Content, nothing near: the directive drive (raw 0.25 floored to 1, beating zeros).
    assert_eq!(step(&d, &world(100.0, 1.0, true, 0.0), &mut s, 90), (Some(DIRECTIVE), DriveReason::Best));
    assert_eq!(s, ArbiterState { running: Some(DIRECTIVE), since: 90 });
    // An alarm at 10 m: combat 1.25 beats the floor 1.0.
    assert_eq!(step(&d, &world(10.0, 1.0, true, 1.0), &mut s, 100), (Some(COMBAT), DriveReason::Best));
    assert_eq!(s, ArbiterState { running: Some(COMBAT), since: 100 });
    // Badly hurt: flee 2.0 beats combat's 1.25 x 1.25, but combat is committed until tick 110.
    let hurt = world(10.0, 0.0, true, 1.0);
    for tick in [101, 105, 109] {
        assert_eq!(step(&d, &hurt, &mut s, tick), (Some(COMBAT), DriveReason::Commit), "tick {tick}");
    }
    assert_eq!(s.since, 100);
    // Commit over: health 0.125 gives flee 1.53125, above combat's raw but below its momentum.
    let a = arbitrate(&d, &world(10.0, 0.125, true, 1.0), &s, 110, None);
    assert_eq!((a.drive, a.reason), (Some(COMBAT), DriveReason::Momentum));
    assert_eq!(
        a.scores,
        [Scored { raw: 1.25, score: 1.5625 }, Scored { raw: 0.25, score: 1.0 }, Scored { raw: 1.53125, score: 1.53125 }]
    );
    s = a.state;
    // Flee 2.0 beats 1.5625: the switch happens on the first decision past the commit.
    assert_eq!(step(&d, &hurt, &mut s, 111), (Some(FLEE), DriveReason::Best));
    assert_eq!(s, ArbiterState { running: Some(FLEE), since: 111 });
    // A coarser tier deciding first at 113 with the same inputs switches at 113 (tier invariance).
    let mut coarse = ArbiterState { running: Some(COMBAT), since: 100 };
    assert_eq!(step(&d, &hurt, &mut coarse, 105), (Some(COMBAT), DriveReason::Commit));
    assert_eq!(step(&d, &hurt, &mut coarse, 113), (Some(FLEE), DriveReason::Best));
}

#[test]
fn commit_releases_when_the_running_drive_stops_applying() {
    let d = drives();
    let mut s = ArbiterState { running: Some(COMBAT), since: 100 };
    // Threat gone at tick 102: combat's raw is 0, so its commit doesn't hold.
    assert_eq!(step(&d, &world(100.0, 1.0, true, 1.0), &mut s, 102), (Some(DIRECTIVE), DriveReason::Best));
    assert_eq!(s, ArbiterState { running: Some(DIRECTIVE), since: 102 });
    // Nothing applies: idle, and an unknown running id is ignored.
    let a = arbitrate(&d, &world(100.0, 1.0, false, 0.0), &ArbiterState { running: Some(DriveId(9)), since: 0 }, 7, None);
    assert_eq!((a.drive, a.reason, a.state), (None, DriveReason::Idle, ArbiterState { running: None, since: 7 }));
}

#[test]
fn directive_floor_holds_against_a_weaker_reaction() {
    let d = drives();
    let mut s = ArbiterState { running: Some(DIRECTIVE), since: 0 };
    // Threat at 15 m: near 0.25 -> 0.34375, x 2 = 0.6875: above the directive's raw 0.25, below its floor.
    let a = arbitrate(&d, &world(15.0, 1.0, true, 1.0), &s, 5, None);
    assert_eq!((a.drive, a.reason), (Some(DIRECTIVE), DriveReason::Floor));
    assert_eq!(a.scores[usize::from(COMBAT.0)], Scored { raw: 0.6875, score: 0.6875 });
    assert_eq!(a.scores[usize::from(DIRECTIVE.0)], Scored { raw: 0.25, score: 1.0 });
    assert_eq!(serving_for(&d, a.drive, Serving::Top), Serving::Top);
    // A clear reaction (threat at 0 m: combat 2.0) beats the floor and suspends the stack.
    assert_eq!(step(&d, &world(0.0, 1.0, true, 1.0), &mut s, 6), (Some(COMBAT), DriveReason::Best));
    assert_eq!(serving_for(&d, Some(COMBAT), Serving::Top), Serving::Suspended);
    // No directive to serve: the floor doesn't apply (raw 0 stays 0), the weak reaction wins.
    let a = arbitrate(&d, &world(15.0, 1.0, false, 1.0), &ArbiterState::default(), 7, None);
    assert_eq!((a.drive, a.reason), (Some(COMBAT), DriveReason::Best));
    assert_eq!(a.scores[usize::from(DIRECTIVE.0)], Scored { raw: 0.0, score: 0.0 });
}

#[test]
fn serving_follows_the_choice() {
    let d = drives();
    assert_eq!(serving_for(&d, Some(DIRECTIVE), Serving::Suspended), Serving::Top);
    assert_eq!(serving_for(&d, Some(DIRECTIVE), Serving::Directive(DirectiveId(3))), Serving::Directive(DirectiveId(3)));
    assert_eq!(serving_for(&d, None, Serving::Suspended), Serving::Top);
    assert_eq!(serving_for(&d, Some(FLEE), Serving::Directive(DirectiveId(3))), Serving::Suspended);
    assert_eq!(d.directive(), Some(DIRECTIVE));
}

#[test]
fn ties_go_to_record_id_order_whatever_the_load_order() {
    let inputs = Inputs::new();
    let flat = DriveDef {
        considerations: Vec::new(),
        weight: 1.0,
        momentum: 0.0,
        commit_ticks: 0,
        floor: None,
        directive: false,
    };
    let forward = Drives::from_defs([], [("alpha".into(), &flat), ("beta".into(), &flat)], &inputs).unwrap();
    let backward = Drives::from_defs([], [("beta".into(), &flat), ("alpha".into(), &flat)], &inputs).unwrap();
    for d in [&forward, &backward] {
        assert_eq!(d.drive_id("alpha"), Some(DriveId(0)));
        assert_eq!(d.drive_name(DriveId(1)), "beta");
        let a = arbitrate(d, &[], &ArbiterState::default(), 1, None);
        assert_eq!((a.drive, a.reason), (Some(DriveId(0)), DriveReason::Tie));
        // Equal scores, beta running with no momentum: still the lower id.
        let a = arbitrate(d, &[], &ArbiterState { running: Some(DriveId(1)), since: 0 }, 1, None);
        assert_eq!((a.drive, a.reason, a.state.since), (Some(DriveId(0)), DriveReason::Tie, 1));
    }
}

// --- Loading -----------------------------------------------------------------------------------

#[test]
fn unknown_names_and_bad_values_fail_at_load_with_a_report() {
    const BAD: &str = r#"[
      define(kind: "sf.ai.consideration", id: "c_smell", value: (input: "smell", range: (1.0, 1.0),
        curve: Piecewise([(0.5, 0.0), (0.25, 1.0)]))),
      define(kind: "sf.ai.consideration", id: "c_ok", value: (input: "alarm", range: (0.0, 1.0),
        curve: Logistic(slope: 1.0, steepness: inf, x_shift: 0.5, y_shift: 0.0))),
      define(kind: "sf.ai.drive", id: "work", value: (considerations: ["c_ok", "c_missing"], weight: -1.0,
        momentum: 0.0, commit_ticks: 0, directive: true)),
      define(kind: "sf.ai.drive", id: "talk", value: (considerations: [], weight: 1.0, momentum: -0.5,
        commit_ticks: 0, floor: Some(-1.0), directive: true)),
    ]"#;
    let report = load(BAD).unwrap_err();
    let at = |kind, id: &str| At { kind, id: RecordId::new(id) };
    assert_eq!(
        report.errors,
        [
            // Considerations first, in record id order: c_ok, c_smell.
            E::BadValue { at: at(CONSIDERATION_KIND, "c_ok"), field: "curve", why: "has a non-finite parameter" },
            E::UnknownInput { at: at(CONSIDERATION_KIND, "c_smell"), name: "smell".into() },
            E::BadValue {
                at: at(CONSIDERATION_KIND, "c_smell"),
                field: "range",
                why: "must be two finite, different values"
            },
            E::BadValue {
                at: at(CONSIDERATION_KIND, "c_smell"),
                field: "curve",
                why: "points are not in strictly increasing x"
            },
            // Then drives: talk, work.
            E::BadValue { at: at(DRIVE_KIND, "talk"), field: "momentum", why: "must be finite and >= 0" },
            E::BadValue { at: at(DRIVE_KIND, "talk"), field: "floor", why: "must be finite and >= 0" },
            E::UnknownConsideration { at: at(DRIVE_KIND, "work"), name: "c_missing".into() },
            E::BadValue { at: at(DRIVE_KIND, "work"), field: "weight", why: "must be finite and >= 0" },
            E::SecondDirective { at: at(DRIVE_KIND, "work"), first: RecordId::new("talk") },
        ]
    );
    assert_eq!(
        report.to_string().lines().take(3).collect::<Vec<_>>(),
        [
            "AI drives: 9 error(s)",
            "  sf.ai.consideration `c_ok`: curve has a non-finite parameter",
            "  sf.ai.consideration `c_smell`: unknown input `smell`",
        ]
    );
    assert!(report.to_string().ends_with("sf.ai.drive `work`: a second directive drive (the first is `talk`)"));
}

// --- Saves -------------------------------------------------------------------------------------

#[test]
fn arbiter_state_round_trips() {
    let d = drives();
    let state = ArbiterState { running: Some(FLEE), since: 1_234_567 };
    let json = serde_json::to_string(&state).unwrap();
    assert_eq!(json, r#"{"running":2,"since":1234567}"#);
    assert_eq!(serde_json::from_str::<ArbiterState>(&json).unwrap(), state);
    let bytes = postcard::to_allocvec(&state).unwrap();
    assert_eq!(postcard::from_bytes::<ArbiterState>(&bytes).unwrap(), state);

    // The save form names the drive, so a reordered or shrunk drive set still resolves it.
    let saved = state.to_saved(&d);
    assert_eq!(saved, SavedArbiter { drive: Some(RecordId::new("flee")), since: 1_234_567 });
    assert_eq!(serde_json::to_string(&saved).unwrap(), r#"{"drive":"flee","since":1234567}"#);
    let back: SavedArbiter = postcard::from_bytes(&postcard::to_allocvec(&saved).unwrap()).unwrap();
    assert_eq!(back.resolve(&d), state);
    let gone = SavedArbiter { drive: Some(RecordId::new("dance")), since: 5 };
    assert_eq!(gone.resolve(&d), ArbiterState { running: None, since: 5 });
    assert_eq!(ArbiterState::default().to_saved(&d), SavedArbiter::default());
}

// --- Trace -------------------------------------------------------------------------------------

#[test]
fn one_decision_is_traced_in_full() {
    let d = drives();
    let prev = ArbiterState { running: Some(COMBAT), since: 100 };
    let w = world(10.0, 0.125, true, 1.0);
    let mut entry = TraceEntry::new(110, 2);
    // Stale content from an earlier step is replaced.
    entry.drives.push(DriveScore { drive: DriveId(7), raw: 1.0, score: 1.0, considerations: Vec::new() });
    let traced = arbitrate(&d, &w, &prev, 110, Some(&mut entry));
    assert_eq!(traced, arbitrate(&d, &w, &prev, 110, None));
    let c = |name: &str| d.consideration_id(name).unwrap();
    let cs = |name, input, value| ConsiderationScore { consideration: c(name), input, value };
    assert_eq!(
        entry.drives,
        [
            DriveScore {
                drive: COMBAT,
                raw: 1.25,
                score: 1.5625,
                considerations: vec![cs("c_near", 10.0, 0.5), cs("c_alarm", 1.0, 1.0)],
            },
            DriveScore { drive: DIRECTIVE, raw: 0.25, score: 1.0, considerations: vec![cs("c_content", 1.0, 0.25)] },
            DriveScore { drive: FLEE, raw: 1.53125, score: 1.53125, considerations: vec![cs("c_hurt", 0.125, 0.765_625)] },
        ]
    );
    assert_eq!((entry.drive, entry.drive_reason), (Some(COMBAT), Some(DriveReason::Momentum)));
    let mut trace = Trace::new();
    trace.push(entry);
    assert_eq!(format_trace(&trace, 1), "#110 t2 scheduled drive d0 (momentum) via - rej - op - path -\n");

    // Traced, every consideration is listed even after a zero; untraced, the product stops early.
    let mut entry = TraceEntry::new(1, 2);
    let quiet = world(10.0, 1.0, false, 0.0);
    let a = arbitrate(&d, &quiet, &ArbiterState::default(), 1, Some(&mut entry));
    assert_eq!(a, arbitrate(&d, &quiet, &ArbiterState::default(), 1, None));
    assert_eq!(entry.drives[0].considerations, [cs("c_near", 10.0, 0.5), cs("c_alarm", 0.0, 0.0)]);
    assert_eq!((entry.drive, entry.drive_reason), (None, Some(DriveReason::Idle)));
}
