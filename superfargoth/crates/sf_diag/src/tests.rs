use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Duration;

use glam::{DQuat, DVec3};

use crate::*;

fn approx(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}

/// A recorder with the given frame times and nothing else.
fn frames(ms: &[f64]) -> Recorder {
    let counters = Counters::new();
    let mut rec = Recorder::new();
    for &m in ms {
        rec.commit_frame(m, &counters);
    }
    rec
}

// ---- stats ----

#[test]
fn percentiles_interpolate_linearly() {
    let sorted = [1.0, 2.0, 3.0, 4.0, 5.0];
    assert!(approx(percentile_sorted(&sorted, 50.0), 3.0));
    assert!(approx(percentile_sorted(&sorted, 0.0), 1.0));
    assert!(approx(percentile_sorted(&sorted, 100.0), 5.0));
    // rank = 4 * 0.95 = 3.8 -> 4 + 0.8 * (5 - 4)
    assert!(approx(percentile_sorted(&sorted, 95.0), 4.8));
    assert!(approx(percentile_sorted(&[7.0], 99.0), 7.0));
    // Even count: median between the middle two.
    assert!(approx(percentile_sorted(&[1.0, 2.0, 3.0, 4.0], 50.0), 2.5));
}

#[test]
fn stats_ignore_nan_and_order() {
    let s = Stats::from_samples(&[5.0, f64::NAN, 1.0, 3.0]).unwrap();
    assert_eq!(s.count, 3);
    assert!(approx(s.min, 1.0));
    assert!(approx(s.max, 5.0));
    assert!(approx(s.median, 3.0));
    assert!(approx(s.mean, 3.0));
    assert!(Stats::from_samples(&[f64::NAN]).is_none());
    assert!(Stats::from_samples(&[]).is_none());
}

#[test]
fn stats_over_100_frames() {
    let samples: Vec<f64> = (1..=100).map(f64::from).collect();
    let s = Stats::from_samples(&samples).unwrap();
    assert!(approx(s.median, 50.5));
    assert!(approx(s.p95, 95.05));
    assert!(approx(s.p99, 99.01));
    assert!(approx(s.get(Stat::Max), 100.0));
}

// ---- counters and recorder ----

#[test]
fn sum_counters_accumulate_and_reset_per_frame() {
    let counters = Counters::new();
    let spawn = counters.register("stream/spawn/ms", CounterKind::Sum, unit::MS);
    let mut rec = Recorder::new();
    counters.add(spawn, 0.25);
    counters.add(spawn, 0.5);
    rec.commit_frame(8.0, &counters);
    rec.commit_frame(8.0, &counters); // nothing reported: 0
    counters.add_duration(spawn, Duration::from_micros(1500));
    rec.commit_frame(9.0, &counters);

    let s = rec.get("stream/spawn/ms").unwrap();
    assert_eq!(s.values(), &[0.75, 0.0, 1.5]);
    assert!(approx(s.total(), 2.25));
    assert_eq!(s.reports(), 2);
}

#[test]
fn gauges_are_missing_when_unreported() {
    let counters = Counters::new();
    let mut rec = Recorder::new();
    rec.commit_frame(8.0, &counters);
    counters.set_named("stream/queue", unit::COUNT, 3.0);
    counters.set_named("stream/queue", unit::COUNT, 7.0); // last wins
    rec.commit_frame(8.0, &counters);
    rec.commit_frame(8.0, &counters);

    let s = rec.get("stream/queue").unwrap();
    assert!(s.values()[0].is_nan(), "registered after frame 0");
    assert!(approx(s.values()[1], 7.0));
    assert!(s.values()[2].is_nan());
    assert_eq!(s.last(), Some(7.0));
}

#[test]
fn counters_are_shared_across_threads() {
    let counters = Arc::new(Counters::new());
    let id = counters.register("jobs/items", CounterKind::Sum, unit::COUNT);
    std::thread::scope(|scope| {
        for _ in 0..8 {
            let counters = &counters;
            scope.spawn(move || {
                for _ in 0..1000 {
                    counters.add(id, 1.0);
                }
            });
        }
    });
    let mut rec = Recorder::new();
    rec.commit_frame(1.0, &counters);
    assert!(approx(rec.get("jobs/items").unwrap().values()[0], 8000.0));
}

#[test]
fn register_is_idempotent_and_flush_has_no_rows() {
    let counters = Counters::new();
    let a = counters.register("bake/mesh.cook/ms", CounterKind::Sum, unit::MS);
    let b = counters.register("bake/mesh.cook/ms", CounterKind::Gauge, unit::COUNT);
    assert_eq!(a, b);
    assert_eq!(counters.def(a).unwrap().kind, CounterKind::Sum);
    let mut rec = Recorder::new();
    counters.add(a, 40_000.0);
    rec.flush(&counters);
    counters.add(a, 50_000.0);
    rec.flush(&counters);
    assert!(rec.frame_ms().is_empty());
    assert!(approx(
        rec.get("bake/mesh.cook/ms").unwrap().total(),
        90_000.0
    ));

    // A bake budget gate on the total.
    let gate = Gate::MaxCounter {
        counter: "bake/mesh.cook/ms".into(),
        stat: CounterStat::Total,
        max: 120_000.0,
    };
    let report = Report::build("bake", &rec, &[gate], 1.5, BTreeMap::new());
    assert!(report.passed, "{}", report.summary());
    assert!(report.frames.is_none());
}

#[test]
fn discard_drops_values() {
    let counters = Counters::new();
    counters.add_named("x", unit::MS, 3.0);
    Recorder::discard(&counters);
    let mut rec = Recorder::new();
    rec.commit_frame(1.0, &counters);
    assert!(approx(rec.get("x").unwrap().values()[0], 0.0));
    assert!(approx(rec.get("x").unwrap().total(), 0.0));
}

#[test]
fn time_scope_adds_elapsed() {
    let counters = Counters::new();
    let id = counters.register("work", CounterKind::Sum, unit::MS);
    let out = counters.time(id, || 42);
    assert_eq!(out, 42);
    let mut rec = Recorder::new();
    rec.commit_frame(1.0, &counters);
    assert!(rec.get("work").unwrap().values()[0] >= 0.0);
}

// ---- CSV ----

#[test]
fn csv_has_header_rows_and_escaping() {
    let counters = Counters::new();
    let mut rec = Recorder::new();
    counters.add_named("stream/spawn/ms", unit::MS, 0.5);
    counters.set_named("odd,\"name\"", unit::COUNT, 2.0);
    rec.commit_frame(8.0, &counters);
    rec.commit_frame(8.5, &counters);
    let mut out = Vec::new();
    rec.write_csv(&mut out).unwrap();
    let text = String::from_utf8(out).unwrap();
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(
        lines[0],
        "frame,t_s,frame_ms,stream/spawn/ms,\"odd,\"\"name\"\"\""
    );
    assert_eq!(lines[1], "0,0.008,8,0.5,2");
    assert_eq!(lines[2], "1,0.0165,8.5,0,");
    assert_eq!(lines.len(), 3);
}

// ---- gates ----

#[test]
fn wishlist_gates_pass_on_steady_frames() {
    let rec = frames(&[8.0, 8.2, 7.9, 8.1, 10.0]);
    let mut gates = Gate::wishlist_streaming();
    gates.push(Gate::target_fps(120.0));
    let report = Report::build("steady", &rec, &gates, 1.5, BTreeMap::new());
    assert!(report.passed, "{}", report.summary());
    let ratio = report.gates[0].value.unwrap();
    assert!(approx(ratio, 10.0 / 8.1));
}

#[test]
fn wishlist_gates_fail_on_spike_and_hitch() {
    // 13 ms > 1.5 x 8 ms but < 33 ms: only the ratio gate fails.
    let rec = frames(&[8.0, 8.0, 8.0, 8.0, 13.0]);
    let report = Report::build(
        "spike",
        &rec,
        &Gate::wishlist_streaming(),
        1.5,
        BTreeMap::new(),
    );
    assert!(!report.passed);
    assert!(!report.gates[0].passed);
    assert!(report.gates[1].passed);

    let rec = frames(&[30.0, 30.0, 30.0, 40.0]);
    let report = Report::build(
        "hitch",
        &rec,
        &Gate::wishlist_streaming(),
        1.5,
        BTreeMap::new(),
    );
    assert!(report.gates[0].passed, "40/30 < 1.5");
    assert!(!report.gates[1].passed, "40 ms > 33 ms");
}

#[test]
fn fps_spike_and_stat_gates() {
    let mut ms = vec![8.0; 98];
    ms.extend([20.0, 25.0]);
    let rec = frames(&ms);
    let gates = [
        Gate::target_fps(120.0),
        Gate::MinFps {
            stat: Stat::P99,
            fps: 60.0,
        },
        Gate::MaxSpikes {
            factor: 2.0,
            count: 1,
        },
        Gate::MaxFrameStat {
            stat: Stat::P95,
            ms: 8.5,
        },
    ];
    let report = Report::build("fps", &rec, &gates, 1.5, BTreeMap::new());
    assert!(report.gates[0].passed, "median 8 ms = 125 fps");
    assert!(!report.gates[1].passed, "p99 ~20 ms = ~50 fps");
    assert!(!report.gates[2].passed, "two spikes > 16 ms");
    assert!(approx(report.gates[2].value.unwrap(), 2.0));
    assert!(report.gates[3].passed);
    let f = report.frames.as_ref().unwrap();
    assert_eq!(f.spikes, 2);
    assert!(approx(f.fps_median, 125.0));
}

#[test]
fn gates_without_data_fail() {
    let rec = Recorder::new();
    let gates = [
        Gate::MaxFrameMs { ms: 33.0 },
        Gate::MaxCounter {
            counter: "missing".into(),
            stat: CounterStat::Max,
            max: 1.0,
        },
    ];
    let report = Report::build("empty", &rec, &gates, 1.5, BTreeMap::new());
    assert!(report.gates.iter().all(|g| !g.passed && g.value.is_none()));
    assert!(!report.passed);
}

#[test]
fn counter_gate_checks_per_frame_budget() {
    let counters = Counters::new();
    let spawn = counters.register("stream/spawn/ms", CounterKind::Sum, unit::MS);
    let mut rec = Recorder::new();
    for v in [0.4, 0.9, 1.3] {
        counters.add(spawn, v);
        rec.commit_frame(8.0, &counters);
    }
    let gate = Gate::MaxCounter {
        counter: "stream/spawn/ms".into(),
        stat: CounterStat::Max,
        max: 1.0,
    };
    let report = Report::build("budget", &rec, &[gate], 1.5, BTreeMap::new());
    assert!(!report.passed);
    assert!(approx(report.gates[0].value.unwrap(), 1.3));
}

// ---- report ----

#[test]
fn spikes_name_their_culprits() {
    let counters = Counters::new();
    let mut rec = Recorder::new();
    for i in 0..20 {
        counters.add_named("stream/spawn/ms", unit::MS, 0.2);
        counters.add_named("stream/items", unit::COUNT, 50.0);
        if i == 10 {
            counters.add_named("stream/activate/ms", unit::MS, 9.0);
        }
        rec.commit_frame(if i == 10 { 18.0 } else { 8.0 }, &counters);
    }
    let report = Report::build("spikes", &rec, &[], 1.5, BTreeMap::new());
    assert_eq!(report.spikes.len(), 1);
    let spike = &report.spikes[0];
    assert_eq!(spike.frame, 10);
    assert!(approx(spike.frame_ms, 18.0));
    assert!(approx(spike.t_s, (10.0 * 8.0 + 18.0) / 1000.0));
    assert_eq!(spike.culprits[0].counter, "stream/activate/ms");
    assert_eq!(spike.culprits[1].counter, "stream/spawn/ms");
    assert!(
        spike.culprits.iter().all(|c| c.counter != "stream/items"),
        "only ms counters are culprits"
    );
}

#[test]
fn report_json_round_trips() {
    let counters = Counters::new();
    let mut rec = Recorder::new();
    for i in 0..10 {
        counters.add_named("a/ms", unit::MS, f64::from(i));
        counters.set_named("b/depth", unit::COUNT, 3.0);
        rec.commit_frame(8.0 + f64::from(i % 3), &counters);
    }
    let mut meta = BTreeMap::new();
    meta.insert("machine".to_owned(), "test".to_owned());
    let report = Report::build("rt", &rec, &Gate::wishlist_streaming(), 1.5, meta);
    let json = report.to_json();
    assert!(json.contains("\"gate\": \"worst_over_median\""));
    let back = Report::from_json(&json).unwrap();
    assert_eq!(back, report);
    assert_eq!(back.counters["b/depth"].kind, CounterKind::Gauge);
    assert!(back.counters["b/depth"].total.is_none());

    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("nested/rt.json");
    report.save(&path).unwrap();
    assert_eq!(Report::load(&path).unwrap(), report);
}

#[test]
fn baseline_flags_regressions_beyond_both_tolerances() {
    let build = |frame: f64, stage: f64| {
        let counters = Counters::new();
        let mut rec = Recorder::new();
        for _ in 0..50 {
            counters.add_named("stream/spawn/ms", unit::MS, stage);
            counters.add_named("stream/items", unit::COUNT, stage * 100.0);
            rec.commit_frame(frame, &counters);
        }
        Report::build("fly", &rec, &[], 1.5, BTreeMap::new())
    };
    let baseline = build(8.0, 0.5);
    let tol = Tolerance::default();

    // Same numbers: nothing changes.
    let mut same = build(8.0, 0.5);
    same.compare(&baseline, "base.json", &tol).unwrap();
    assert!(same.passed && same.regressions.is_empty() && same.improvements.is_empty());

    // Stage per-frame p95 +40 % but only +0.2 ms: below the absolute floor.
    // Its total (+10 ms over 50 frames) and the frame time do regress.
    let mut slower = build(9.0, 0.7);
    slower.compare(&baseline, "base.json", &tol).unwrap();
    let names: Vec<&str> = slower
        .regressions
        .iter()
        .map(|r| r.metric.as_str())
        .collect();
    assert!(names.contains(&"frame_ms.median"));
    assert!(names.contains(&"counter.stream/spawn/ms.total"));
    assert!(!names.contains(&"counter.stream/spawn/ms.p95"));
    assert!(
        !names.iter().any(|n| n.contains("items")),
        "count counters are not compared"
    );
    assert!(!slower.passed);
    assert_eq!(slower.baseline.as_deref(), Some("base.json"));
    assert!(slower.summary().contains("REGRESSED frame_ms.median"));

    // Faster: improvements, still passes.
    let mut faster = build(6.0, 0.5);
    faster.compare(&baseline, "base.json", &tol).unwrap();
    assert!(faster.passed);
    assert!(!faster.improvements.is_empty());

    // Wrong scenario or format is an error, not a pass.
    let mut other = build(8.0, 0.5);
    other.scenario = "other".into();
    assert!(other.compare(&baseline, "b", &tol).is_err());
    let mut old = baseline.clone();
    old.format = 0;
    assert!(build(8.0, 0.5).compare(&old, "b", &tol).is_err());
}

// ---- camera path ----

fn zigzag() -> Vec<Keyframe> {
    vec![
        Keyframe::at(DVec3::new(0.0, 0.0, 0.0)),
        Keyframe::at(DVec3::new(10.0, 0.0, 0.0)),
        Keyframe::at(DVec3::new(11.0, 0.0, 1.0)), // close to the previous key
        Keyframe::at(DVec3::new(40.0, 5.0, 30.0)),
    ]
}

#[test]
fn path_passes_through_keyframes_and_ends() {
    let path = CameraPath::new(zigzag()).unwrap();
    assert!(path.pose_at_distance(0.0).position.distance(DVec3::ZERO) < 1e-9);
    let end = path.pose_at_distance(path.length() + 100.0).position;
    assert!(end.distance(DVec3::new(40.0, 5.0, 30.0)) < 1e-9);
    // Every keyframe lies on the curve (closest sample within a few mm).
    for key in path.keyframes() {
        let closest = (0..=20_000)
            .map(|i| {
                path.pose_at_distance(path.length() * f64::from(i) / 20_000.0)
                    .position
            })
            .map(|p| p.distance(key.position))
            .fold(f64::INFINITY, f64::min);
        assert!(
            closest < 0.01,
            "key {:?} off the curve by {closest}",
            key.position
        );
    }
}

#[test]
fn path_moves_at_constant_speed() {
    let path = CameraPath::new(zigzag()).unwrap();
    let speed = 7.0;
    let dt = 1.0 / 120.0;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let steps = (path.duration(speed) / dt) as u32;
    for i in 0..steps {
        // Arc length traveled in this frame, by fine sub-chords (a single
        // chord undercuts the arc in the tight corner near keyframe 2).
        let t0 = f64::from(i) * dt;
        let sub = 32;
        let mut arc = 0.0;
        let mut prev = path.pose_at_time(t0, speed).position;
        for k in 1..=sub {
            let p = path
                .pose_at_time(t0 + dt * f64::from(k) / f64::from(sub), speed)
                .position;
            arc += p.distance(prev);
            prev = p;
        }
        assert!(
            (arc - speed * dt).abs() < 0.005 * speed * dt,
            "frame {i}: {arc} vs {}",
            speed * dt
        );
    }
}

#[test]
fn centripetal_path_does_not_overshoot() {
    // Uneven spacing: uniform Catmull-Rom loops past the short segment;
    // centripetal stays near the keyframes' hull.
    let path = CameraPath::new(zigzag()).unwrap();
    for i in 0..=1000 {
        let p = path
            .pose_at_distance(path.length() * f64::from(i) / 1000.0)
            .position;
        assert!(p.x > -0.5 && p.x < 40.5, "x {p}");
        assert!(p.z > -1.0 && p.z < 30.5, "z {p}");
    }
}

#[test]
fn path_orientation_follows_tangent_or_keys() {
    let path = CameraPath::new(vec![
        Keyframe::at(DVec3::ZERO),
        Keyframe::at(DVec3::new(0.0, 0.0, -10.0)),
    ])
    .unwrap();
    let forward = path.pose_at_distance(5.0).rotation * DVec3::NEG_Z;
    assert!(
        forward.distance(DVec3::NEG_Z) < 1e-6,
        "looks along the path: {forward}"
    );

    let yaw = DQuat::from_rotation_y(std::f64::consts::FRAC_PI_2);
    let path = CameraPath::new(vec![
        Keyframe {
            position: DVec3::ZERO,
            rotation: Some(DQuat::IDENTITY),
        },
        Keyframe {
            position: DVec3::X,
            rotation: Some(yaw),
        },
    ])
    .unwrap();
    let mid = path.pose_at_distance(path.length() / 2.0).rotation;
    let expected = DQuat::from_rotation_y(std::f64::consts::FRAC_PI_4);
    assert!(mid.angle_between(expected) < 1e-6);
}

#[test]
fn invalid_paths_are_rejected() {
    assert!(CameraPath::new(vec![Keyframe::at(DVec3::ZERO)]).is_err());
    assert!(CameraPath::new(vec![Keyframe::at(DVec3::ZERO), Keyframe::at(DVec3::ZERO)]).is_err());
    assert!(CameraPath::new(vec![Keyframe::at(DVec3::ZERO), Keyframe::at(DVec3::NAN)]).is_err());
}

// ---- scenario ----

fn scenario(measure: Measure) -> Scenario {
    Scenario {
        name: "s".into(),
        warmup: Warmup {
            seconds: 0.02,
            frames: 3,
        },
        measure,
        gates: Gate::wishlist_streaming(),
        spike_factor: 1.5,
        tolerance: Tolerance::default(),
    }
}

#[test]
fn scenario_warms_up_then_measures_frames() {
    let mut run = ScenarioRun::new(scenario(Measure::Frames { count: 4 })).unwrap();
    let steps: Vec<Step> = (0..10).map(|_| run.advance(10.0, false)).collect();
    // Frames 1-2: warm-up; frame 3 meets both minimums (3 frames, 30 ms).
    assert!(steps[..3].iter().all(|s| !s.record));
    assert_eq!(steps[2].phase, Phase::Measure);
    assert_eq!(steps.iter().filter(|s| s.record).count(), 4);
    assert_eq!(steps.iter().filter(|s| s.just_finished).count(), 1);
    assert!(steps[6].just_finished);
    assert_eq!(steps[9].phase, Phase::Done);
}

#[test]
fn scenario_hold_extends_warmup() {
    let mut run = ScenarioRun::new(scenario(Measure::Duration { seconds: 0.0625 })).unwrap();
    for _ in 0..10 {
        assert!(!run.advance(15.625, true).record);
    }
    assert_eq!(run.phase(), Phase::Warmup);
    run.advance(15.625, false);
    assert_eq!(run.phase(), Phase::Measure);
    // 15.625 ms = 1/64 s, exact in binary, so the sum hits 0.0625 exactly.
    let measured = (0..20)
        .map(|_| run.advance(15.625, false))
        .take_while(|s| s.record)
        .count();
    assert_eq!(measured, 4, "62.5 ms of 15.625 ms frames");
}

#[test]
fn path_scenario_reaches_the_end_and_records_it() {
    let keys = vec![
        Keyframe::at(DVec3::ZERO),
        Keyframe::at(DVec3::new(1.0, 0.0, 0.0)),
    ];
    let mut run = ScenarioRun::new(scenario(Measure::Path {
        keyframes: keys,
        speed: 8.0,
    }))
    .unwrap();
    let mut last_pose = None;
    let mut recorded = 0;
    for _ in 0..100 {
        // 1/64 s frames at 8 m/s: exactly 0.125 m per frame.
        let step = run.advance(15.625, false);
        recorded += usize::from(step.record);
        if step.phase == Phase::Warmup {
            assert!(
                step.pose.unwrap().position.length() < 1e-9,
                "holds the start"
            );
        }
        if let Some(pose) = step.pose {
            last_pose = Some(pose);
        }
        if step.just_finished {
            break;
        }
    }
    // 1 m in 0.125 m steps: 8 moving frames plus the one showing the end.
    assert_eq!(recorded, 9);
    assert!(last_pose.unwrap().position.distance(DVec3::X) < 1e-9);
}

#[test]
fn scenario_json_and_validation() {
    let json = r#"{
        "name": "census",
        "warmup": { "seconds": 3.0 },
        "measure": { "kind": "path", "speed": 7.0, "keyframes": [
            { "position": [0.0, 1.7, 0.0] },
            { "position": [5.0, 1.7, -3.0], "rotation": [0.0, 0.0, 0.0, 1.0] }
        ] },
        "gates": [
            { "gate": "worst_over_median", "factor": 1.5 },
            { "gate": "max_frame_ms", "ms": 33.0 },
            { "gate": "min_fps", "stat": "median", "fps": 144.0 },
            { "gate": "max_counter", "counter": "stream/spawn/ms", "stat": "max", "max": 1.0 }
        ]
    }"#;
    let s = Scenario::from_json(json).unwrap();
    assert_eq!(s.warmup.frames, Warmup::default().frames);
    assert!(approx(s.spike_factor, 1.5));
    assert_eq!(s.gates.len(), 4);
    ScenarioRun::new(s).unwrap();

    assert!(ScenarioRun::new(scenario(Measure::Frames { count: 0 })).is_err());
    assert!(ScenarioRun::new(scenario(Measure::Duration { seconds: 0.0 })).is_err());
    let mut bad = scenario(Measure::Duration { seconds: 1.0 });
    bad.spike_factor = 1.0;
    assert!(ScenarioRun::new(bad).is_err());
}

// ---- hitches and markdown ----

#[test]
fn hitch_table_lists_slow_frames_with_churn_from_both_phases() {
    let counters = Counters::new();
    let mut warm = Recorder::new();
    for i in 0..5 {
        if i == 3 {
            counters.add_named("assets/Mesh/added", unit::COUNT, 120.0);
            counters.add_named("pipelines/created", unit::COUNT, 4.0);
        }
        warm.commit_frame(if i == 3 { 80.0 } else { 10.0 }, &counters);
    }
    let mut rec = Recorder::new();
    for i in 0..40 {
        counters.add_named("stream/spawn/ms", unit::MS, 0.2);
        counters.set_named("ecs/entities", unit::COUNT, 1000.0);
        if i == 19 {
            counters.add_named("ecs/entities_delta", unit::COUNT, -300.0);
        }
        if i == 20 {
            counters.add_named("stream/spawn/ms", unit::MS, 30.0);
            counters.add_named("ecs/entities_delta", unit::COUNT, 500.0);
        }
        rec.commit_frame(if i == 20 { 45.0 } else { 10.0 }, &counters);
    }
    let mut report = Report::build("hitch", &rec, &[], 1.5, BTreeMap::new());
    report.add_hitches(&rec, Some(&warm), None);
    let h = report.hitches.as_ref().unwrap();
    assert!(
        approx(h.threshold_ms, 25.0),
        "2.5 x 10 ms median, floored at 25 ms"
    );
    assert_eq!((h.warmup, h.measured), (1, 1));
    assert_eq!(h.rows.len(), 2);
    assert_eq!(
        h.rows[0].phase,
        RunPhase::Warmup,
        "time order: warm-up first"
    );
    assert!(
        h.rows[0]
            .churn
            .iter()
            .any(|c| c.counter == "assets/Mesh/added" && approx(c.value, 120.0))
    );
    let m = &h.rows[1];
    assert_eq!((m.phase, m.frame), (RunPhase::Measure, 20));
    // ms counters first; this frame + the previous one summed.
    assert_eq!(m.churn[0].counter, "stream/spawn/ms");
    assert!(approx(m.churn[0].value, 30.4));
    let delta = m
        .churn
        .iter()
        .find(|c| c.counter == "ecs/entities_delta")
        .unwrap();
    assert!(approx(delta.value, 200.0));
    assert!(
        m.churn.iter().all(|c| c.counter != "ecs/entities"),
        "gauges are not churn"
    );
    let w = report.warmup.as_ref().unwrap();
    assert_eq!(w.frames, 5);
    assert!(approx(w.worst_ms, 80.0));

    // A fixed threshold, and the JSON keeps it all.
    report.add_hitches(&rec, None, Some(5.0));
    assert_eq!(report.hitches.as_ref().unwrap().measured, 40);
    // (serde_json float parsing may differ in the last bit of derived fps.)
    let back = Report::from_json(&report.to_json()).unwrap();
    assert_eq!(back.hitches, report.hitches);
    assert_eq!(back.warmup, report.warmup);
}

#[test]
fn old_reports_without_new_fields_still_load() {
    let json = r#"{"format":1,"scenario":"old","duration_s":1.0,"passed":true}"#;
    let r = Report::from_json(json).unwrap();
    assert!(r.hitches.is_none() && r.warmup.is_none() && r.wall_start_unix_s.is_none());
}

#[test]
fn markdown_shows_frames_passes_costs_gauges_and_hitches() {
    let counters = Counters::new();
    let mut rec = Recorder::new();
    for i in 0..30 {
        counters.set_named("render/main_opaque_pass_3d/elapsed_gpu", "ms", 2.0);
        counters.set_named("render/shadows/elapsed_gpu", "ms", 1.0);
        counters.set_named("render/shadows/cascade_0/elapsed_gpu", "ms", 0.5);
        counters.set_named("render/shadows/elapsed_cpu", "ms", 0.1);
        counters.add_named("terrain/select/ms", unit::MS, 0.25);
        counters.set_named("world/meshes", unit::COUNT, 337_000.0);
        counters.add_named("stream/spawn/items", unit::COUNT, 3.0);
        rec.commit_frame(if i == 7 { 90.0 } else { 20.0 }, &counters);
    }
    let mut report = Report::build(
        "md",
        &rec,
        &Gate::wishlist_streaming(),
        1.5,
        BTreeMap::from([("resolution".to_owned(), "1280x720".to_owned())]),
    );
    report.add_hitches(&rec, None, None);
    let timed = report.timed_gpu_ms().unwrap();
    assert!(
        approx(timed, 3.0),
        "nested spans don't count twice: {timed}"
    );
    let md = report.to_markdown();
    for needle in [
        "# `md`",
        "resolution: 1280x720",
        "**median 20.00 ms (50 fps)**",
        "| main_opaque_pass_3d | 2.000 |",
        "| shadows/cascade_0 | 0.500 |",
        "| terrain/select/ms | 0.250 |",
        "| world/meshes | count | 337.0 k |",
        "| stream/spawn/items | count | 3 |",
        "hitches (> 50.0 ms): **1** measured",
        "| measure | 0.23 | 90.0 | terrain/select/ms 0.50, stream/spawn/items 6 |",
        "**FAIL**",
    ] {
        assert!(md.contains(needle), "missing {needle:?} in\n{md}");
    }
}

// ---- frame pacing ----

/// A recorder with frame times and the given gauge columns (NaN = not reported).
fn columns(ms: &[f64], cols: &[(&str, &[f64])]) -> Recorder {
    let mut rec = frames(ms);
    for (name, values) in cols {
        let def = CounterDef { name: (*name).to_owned(), kind: CounterKind::Gauge, unit: unit::MS.to_owned() };
        rec.set_derived(def, values.to_vec());
    }
    rec
}

#[test]
fn derive_work_takes_max_and_forward_fills_gpu() {
    let nan = f64::NAN;
    let mut rec = columns(
        &[10.0, 10.0, 10.0],
        &[
            (pacing::MAIN_BUSY, &[4.0, 9.0, 3.0]),
            (pacing::RENDER_BUSY, &[5.0, 6.0, nan]),
            ("main_pass/elapsed_gpu", &[2.0, nan, nan]),
            ("shadow/elapsed_gpu", &[nan, 1.5, 6.0]),
        ],
    );
    pacing::derive_work(&mut rec);
    let gpu = rec.get(pacing::GPU_TIMED).unwrap().values();
    // Frame 0: only main_pass known; 1: main_pass carried + shadow; 2: carried + new shadow.
    assert!(approx(gpu[0], 2.0) && approx(gpu[1], 3.5) && approx(gpu[2], 8.0), "{gpu:?}");
    let work = rec.get(pacing::WORK).unwrap().values();
    assert!(approx(work[0], 5.0) && approx(work[1], 9.0) && approx(work[2], 8.0), "{work:?}");
}

#[test]
fn derive_work_needs_a_busy_column() {
    let mut rec = columns(&[10.0], &[("main_pass/elapsed_gpu", &[2.0])]);
    pacing::derive_work(&mut rec);
    assert!(rec.get(pacing::WORK).is_none());
    let gate = Gate::CounterWorstOverMedian { counter: pacing::WORK.into(), factor: 1.5 };
    let report = Report::build("none", &rec, &[gate], 1.5, BTreeMap::new());
    assert!(!report.passed, "missing work column must fail");
    assert!(report.gates[0].value.is_none());
}

#[test]
fn counter_worst_over_median_gate() {
    let gate = Gate::CounterWorstOverMedian { counter: pacing::WORK.into(), factor: 1.5 };
    let pass = columns(&[8.0; 5], &[(pacing::WORK, &[6.0, 6.0, 6.0, 6.0, 8.0])]);
    let report = Report::build("pass", &pass, std::slice::from_ref(&gate), 1.5, BTreeMap::new());
    assert!(report.passed, "{}", report.summary());
    // The presented frames are steady; the work spike fails the gate anyway.
    let fail = columns(&[8.0; 5], &[(pacing::WORK, &[6.0, 6.0, 6.0, 6.0, 9.5])]);
    let report = Report::build("fail", &fail, &[gate], 1.5, BTreeMap::new());
    assert!(!report.passed);
    assert!(approx(report.gates[0].value.unwrap(), 9.5 / 6.0));
    assert_eq!(report.work_spikes.len(), 1);
    assert_eq!(report.work_spikes[0].frame, 4);
}

#[test]
fn stutters_follow_a_moving_median() {
    // A ramp from 4 ms to 12 ms (a scene getting heavier) is no stutter
    // against a moving median; a 20 ms frame on the 12 ms plateau is.
    let mut v: Vec<f64> = (0..80).map(|i| 4.0 + 0.1 * f64::from(i)).collect();
    v.extend([12.0; 40]);
    v[100] = 20.0;
    assert_eq!(pacing::stutters(&v, 1.5, 20), vec![100]);
    // The first frames have no reference; NaN frames are skipped.
    assert!(pacing::stutters(&[1.0, 50.0, f64::NAN], 1.5, 60).is_empty());
}

#[test]
fn max_stutters_gate_counts_per_minute() {
    // 1,000 frames of ~6 ms = 0.1 min, two 10 ms spikes = ~20/min.
    let mut ms = vec![6.0; 1000];
    ms[300] = 10.0;
    ms[600] = 10.0;
    let rec = frames(&ms);
    let gate = |per_minute| Gate::MaxStutters { counter: None, factor: 1.5, window: 60, per_minute };
    let report = Report::build("s", &rec, &[gate(20.0), gate(19.0)], 1.5, BTreeMap::new());
    assert!(approx(report.gates[0].value.unwrap(), 2.0 / (6008.0 / 60_000.0)), "{}", report.gates[0].detail);
    assert!(report.gates[0].passed && !report.gates[1].passed);
    // On a missing counter it fails.
    let missing = Gate::MaxStutters { counter: Some(pacing::WORK.into()), factor: 1.5, window: 60, per_minute: 100.0 };
    assert!(!Report::build("m", &rec, &[missing], 1.5, BTreeMap::new()).passed);
    // Round-trips through JSON (benchmarks/*.json), window defaulted.
    let json = r#"{ "gate": "max_stutters", "factor": 1.5, "per_minute": 1.0 }"#;
    let parsed: Gate = serde_json::from_str(json).unwrap();
    assert_eq!(parsed, Gate::wishlist_streaming_stutters()[0]);
}
