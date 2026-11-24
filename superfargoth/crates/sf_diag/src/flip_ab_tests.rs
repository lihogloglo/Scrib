use super::*;
use crate::Scenario;

const DEFAULTS: AbDefaults = AbDefaults { phase_s: 5.0, phases: 4, settle_s: 0.5, noise_ms: 0.5 };

fn spec(a: &str, b: &str) -> AbSpec {
    AbSpec { tunable: "t.x".into(), a: a.into(), b: b.into(), phase_s: None, phases: None, settle_s: None }
}

#[test]
fn schedule_alternates_and_discards_the_first_cycle_and_each_settle() {
    let s = AbSchedule::new(&spec("off", "on"), DEFAULTS).unwrap();
    assert_eq!(s.phase_count(), 10, "4 pairs kept + 1 discarded cycle");
    assert!((s.total_s() - 50.0).abs() < 1e-12);
    // The value active at t: A B A B ..., A before the start, the last phase's after the end.
    assert_eq!(s.value_at(-1.0), "off");
    assert_eq!(s.value_at(0.0), "off");
    assert_eq!(s.value_at(4.999), "off");
    assert_eq!(s.value_at(5.0), "on");
    assert_eq!(s.value_at(12.0), "off");
    assert_eq!(s.value_at(49.9), "on");
    assert_eq!(s.value_at(80.0), "on");
    // Which frames are discarded.
    assert_eq!(s.slot(0.0), Slot::FirstCycle { phase: 0 });
    assert_eq!(s.slot(7.0), Slot::FirstCycle { phase: 1 });
    assert_eq!(s.slot(10.0), Slot::Settle { phase: 2 });
    assert_eq!(s.slot(10.49), Slot::Settle { phase: 2 });
    assert_eq!(s.slot(10.5), Slot::Keep { phase: 2, arm: Arm::A });
    assert_eq!(s.slot(16.0), Slot::Keep { phase: 3, arm: Arm::B });
    assert_eq!(s.slot(45.2), Slot::Settle { phase: 9 });
    assert_eq!(s.slot(49.9), Slot::Keep { phase: 9, arm: Arm::B });
    assert_eq!(s.slot(50.0), Slot::After);
    // A spec's own fields win over the defaults.
    let own = AbSchedule::new(&AbSpec { phase_s: Some(2.0), phases: Some(1), settle_s: Some(0.0), ..spec("0", "1") }, DEFAULTS).unwrap();
    assert_eq!((own.phase_count(), own.total_s()), (4, 8.0));
    assert_eq!(own.slot(4.0), Slot::Keep { phase: 2, arm: Arm::A });
}

#[test]
fn bad_schedules_are_refused() {
    let bad = |s: AbSpec| AbSchedule::new(&s, DEFAULTS).unwrap_err();
    assert!(bad(spec("1", "1")).contains("same value"));
    assert!(bad(AbSpec { phases: Some(0), ..spec("0", "1") }).contains("phases"));
    assert!(bad(AbSpec { phase_s: Some(0.0), ..spec("0", "1") }).contains("phase_s"));
    assert!(bad(AbSpec { settle_s: Some(5.0), ..spec("0", "1") }).contains("settle_s"));
    assert!(bad(AbSpec { tunable: " ".into(), ..spec("0", "1") }).contains("no tunable"));
}

#[test]
fn ab_flag_parses_two_values() {
    let s = AbSpec::parse("groundcover.blades.enabled=false,true").unwrap();
    assert_eq!((s.tunable.as_str(), s.a.as_str(), s.b.as_str()), ("groundcover.blades.enabled", "false", "true"));
    assert_eq!((s.phase_s, s.phases, s.settle_s), (None, None, None));
    let v = AbSpec::parse("sun.dir = 1,2 | 3,4").unwrap();
    assert_eq!((v.tunable.as_str(), v.a.as_str(), v.b.as_str()), ("sun.dir", "1,2", "3,4"));
    for bad in ["x", "x=1", "x=1,2,3", "=1,2", "x=,2", "x=1|2|3"] {
        assert!(AbSpec::parse(bad).is_err(), "{bad:?} parsed");
    }
}

#[test]
fn scenario_json_takes_an_optional_ab() {
    let with = Scenario::from_json(
        r#"{"name":"s","measure":{"kind":"duration","seconds":10},"ab":{"tunable":"t.x","a":"0","b":"1","phase_s":3}}"#,
    )
    .unwrap();
    assert_eq!(with.ab, Some(AbSpec { phase_s: Some(3.0), ..spec("0", "1") }));
    let without = Scenario::from_json(r#"{"name":"s","measure":{"kind":"duration","seconds":10}}"#).unwrap();
    assert_eq!(without.ab, None);
}

/// Frames of a synthetic run: each frame's length from its phase and the time into it (the same
/// start-time accumulation as [`phase_stats`]).
fn frames(s: &AbSchedule, ms: impl Fn(u32, f64) -> f64) -> Vec<f64> {
    let mut out = Vec::new();
    let mut t = 0.0;
    while t < s.total_s() {
        let phase = s.phase_at(t);
        let v = ms(phase, t - f64::from(phase) * s.phase_s);
        out.push(v);
        t += v / 1000.0;
    }
    out
}

#[test]
fn phase_stats_and_paired_deltas_over_a_synthetic_run() {
    let s = AbSchedule::new(&AbSpec { phase_s: Some(1.0), phases: Some(2), settle_s: Some(0.25), ..spec("off", "on") }, DEFAULTS).unwrap();
    let frame = frames(&s, |phase, into| match phase {
        0 | 1 => 50.0,                            // first cycle: discarded (would be hitches)
        _ if into < 0.25 => 25.0,                 // settle: discarded
        3 if (0.5..0.52).contains(&into) => 40.0, // one hitch in the first B phase
        p if p.is_multiple_of(2) => 10.0,
        _ => 12.5,
    });
    // main busy: 3 ms with A, 4 ms with B (read from the frame lengths); every 7th frame unmeasured.
    let main: Vec<f64> = frame
        .iter()
        .enumerate()
        .map(|(i, &f)| if i % 7 == 0 { f64::NAN } else if (f - 12.5).abs() < 1e-9 || f > 39.0 { 4.0 } else { 3.0 })
        .collect();
    let cols = Columns { frame_ms: &frame, main_ms: Some(&main), render_ms: None, gpu_ms: None };
    let phases = phase_stats(&s, cols, 30.0);
    assert_eq!(phases.iter().map(|p| (p.phase, p.arm)).collect::<Vec<_>>(), [(2, Arm::A), (3, Arm::B), (4, Arm::A), (5, Arm::B)]);
    let p2 = &phases[0];
    assert_eq!((p2.median_ms, p2.p99_ms, p2.hitches), (10.0, 10.0, 0), "settle frames left out");
    assert!((74..=76).contains(&p2.frames), "~0.75 s of 10 ms frames after the settle: {}", p2.frames);
    assert_eq!((phases[1].median_ms, phases[1].hitches), (12.5, 1));
    assert_eq!((phases[3].median_ms, phases[3].hitches), (12.5, 0));
    assert_eq!((p2.main_ms, phases[1].main_ms, p2.render_ms), (Some(3.0), Some(4.0), None));

    let d = deltas(&phases);
    let get = |m: Metric| d.iter().find(|x| x.metric == m).copied();
    let median = get(Metric::Median).unwrap();
    assert_eq!((median.pairs, median.median, median.min, median.max), (2, 2.5, 2.5, 2.5));
    assert_eq!(median.verdict(s.noise_ms), "B slower");
    let hitches = get(Metric::Hitches).unwrap();
    assert_eq!((hitches.median, hitches.min, hitches.max), (0.5, 0.0, 1.0));
    assert_eq!(get(Metric::Main).map(|x| x.median), Some(1.0));
    assert!(get(Metric::Render).is_none() && get(Metric::Gpu).is_none(), "no column, no delta");

    let md = markdown("gc_ascadian", &s, &phases, &d, 30.0);
    assert!(md.contains("| frame median ms | +2.50 | +2.50..+2.50 | 2 | B slower |"), "{md}");
    assert!(md.contains("| 3 | B | `on` | "), "{md}");
    let csv = csv(&s, &phases);
    assert_eq!(csv.lines().count(), 5);
    let row = csv.lines().nth(1).unwrap();
    assert!(row.starts_with("2,A,\"off\",") && row.ends_with(",10.0000,10.0000,0,3.0000,,"), "{csv}");
    assert!(headline(&s, &d).contains("+2.50 ms"));
}

#[test]
fn a_missing_pair_partner_is_skipped() {
    let s = AbSchedule::new(&AbSpec { phase_s: Some(1.0), phases: Some(2), settle_s: Some(0.0), ..spec("0", "1") }, DEFAULTS).unwrap();
    // The run stops in phase 4: phase 5 (the second B) never runs.
    let all = frames(&s, |p, _| if p.is_multiple_of(2) { 10.0 } else { 20.0 });
    let mut t = 0.0;
    let frame: Vec<f64> = all
        .into_iter()
        .take_while(|ms| {
            let keep = t < 4.5;
            t += ms / 1000.0;
            keep
        })
        .collect();
    let phases = phase_stats(&s, Columns { frame_ms: &frame, ..Columns::default() }, 30.0);
    assert_eq!(phases.iter().map(|p| p.phase).collect::<Vec<_>>(), [2, 3, 4]);
    let median = deltas(&phases).into_iter().find(|d| d.metric == Metric::Median).unwrap();
    assert_eq!(median.pairs, 1);
    assert!((median.median - 10.0).abs() < 1e-12);
}

#[test]
fn verdicts_use_the_noise_floor() {
    let d = |median: f64, min: f64, max: f64| Delta { metric: Metric::Median, pairs: 4, median, min, max };
    assert_eq!(d(0.3, 0.1, 0.4).verdict(0.5), "noise");
    assert_eq!(d(-0.8, -1.0, -0.6).verdict(0.5), "B faster");
    assert_eq!(d(0.9, -0.2, 1.5).verdict(0.5), "B slower (pairs disagree)");
    let h = Delta { metric: Metric::Hitches, pairs: 4, median: 0.0, min: 0.0, max: 0.0 };
    assert_eq!(h.verdict(0.5), "same");
}
