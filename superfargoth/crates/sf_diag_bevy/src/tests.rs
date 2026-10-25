use std::time::Duration;

use bevy_app::TaskPoolPlugin;
use bevy_diagnostic::{
    Diagnostic, DiagnosticPath, Diagnostics, DiagnosticsPlugin, FrameCountPlugin,
    RegisterDiagnostic,
};
use bevy_time::{TimePlugin, TimeUpdateStrategy};
use sf_diag::glam::{DVec3, Vec3};
use sf_diag::{Keyframe, Measure, Tolerance, Warmup};

use super::*;

const ENTITIES: DiagnosticPath = DiagnosticPath::const_new("test/entities");

/// Bevy's `MinimalPlugins` minus the schedule runner (the test drives
/// `update` itself), plus diagnostics, with exact 1/64 s frames.
fn app(settings: DiagSettings, scenario: Option<Scenario>) -> App {
    let mut app = App::new();
    app.add_plugins((
        TaskPoolPlugin::default(),
        FrameCountPlugin,
        TimePlugin,
        DiagnosticsPlugin,
    ))
    .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_micros(
        15_625,
    )))
    .register_diagnostic(Diagnostic::new(ENTITIES).with_suffix("count"))
    .add_systems(
        bevy_app::Update,
        |counters: Res<DiagCounters>, mut diagnostics: Diagnostics| {
            counters.add_named("test/work/ms", sf_diag::unit::MS, 0.5);
            diagnostics.add_measurement(&ENTITIES, || 42.0);
        },
    );
    let mut plugin = SfDiagPlugin::new(settings);
    if let Some(scenario) = scenario {
        plugin = plugin.with_scenario(scenario).unwrap();
    }
    app.add_plugins(plugin);
    app
}

/// Updates until the app asks to exit, at most `max` frames.
fn run(app: &mut App, max: usize) -> Option<AppExit> {
    for _ in 0..max {
        app.update();
        if let Some(exit) = app.should_exit() {
            return Some(exit);
        }
    }
    None
}

fn path_scenario(gates: Vec<sf_diag::Gate>) -> Scenario {
    Scenario {
        name: "fly test".into(),
        warmup: Warmup {
            seconds: 0.05,
            frames: 3,
        },
        measure: Measure::Path {
            keyframes: vec![
                Keyframe::at(DVec3::new(0.0, 2.0, 0.0)),
                Keyframe::at(DVec3::new(0.0, 2.0, -1.0)),
            ],
            speed: 8.0,
        },
        gates,
        spike_factor: 1.5,
        tolerance: Tolerance::default(),
    }
}

#[test]
fn headless_fly_through_writes_reports_and_exits() {
    let dir = tempfile::tempdir().unwrap();
    let settings = DiagSettings {
        output_dir: dir.path().to_path_buf(),
        exit_on_finish: true,
        ..DiagSettings::default()
    };
    let mut gates = sf_diag::Gate::wishlist_streaming();
    gates.push(sf_diag::Gate::target_fps(60.0));
    let mut app = app(settings, Some(path_scenario(gates)));
    let camera = app
        .world_mut()
        .spawn((Transform::default(), FlyThroughCamera))
        .id();

    let exit = run(&mut app, 200).expect("scenario should finish");
    assert_eq!(exit, AppExit::Success);

    let finished = app.world().resource::<DiagReport>();
    assert!(finished.passed(), "{}", finished.report.summary());
    let frames = finished.report.frames.as_ref().unwrap();
    // 1 m at 8 m/s in 1/64 s frames: 8 moving frames plus the end frame.
    assert_eq!(frames.stats.count, 9);
    assert!((frames.stats.median - 15.625).abs() < 1e-6);

    // Counters from a system and from Bevy's DiagnosticsStore.
    let work = &finished.report.counters["test/work/ms"];
    assert!((work.total.unwrap() - 0.5 * 9.0).abs() < 1e-9);
    let entities = &finished.report.counters["test/entities"];
    assert_eq!(entities.unit, "count");
    assert!((entities.per_frame.unwrap().median - 42.0).abs() < 1e-9);
    assert!(!finished.report.counters.contains_key("frame_time"));

    // Files on disk.
    let json = std::fs::read_to_string(dir.path().join("fly_test.json")).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["scenario"], "fly test");
    assert_eq!(parsed["passed"], true);
    let csv = std::fs::read_to_string(dir.path().join("fly_test.csv")).unwrap();
    assert!(csv.starts_with("frame,t_s,frame_ms,"));
    assert_eq!(csv.lines().count(), 1 + 9);

    // Markdown report, warm-up frames, hitch table, entity churn, wall clock.
    let md = std::fs::read_to_string(dir.path().join("fly_test.md")).unwrap();
    assert!(
        md.contains("# `fly test`") && md.contains("hitches (> 39.1 ms): **0** measured"),
        "{md}"
    );
    let warm = std::fs::read_to_string(dir.path().join("fly_test.warmup.csv")).unwrap();
    assert!(warm.lines().count() > 3, "warm-up frames recorded");
    let report = &finished.report;
    assert_eq!(report.hitches.as_ref().unwrap().rows.len(), 0);
    assert!(report.warmup.as_ref().unwrap().frames >= 3);
    assert!(
        report.counters.contains_key(crate::ENTITIES)
            && report.counters.contains_key(ENTITIES_DELTA)
    );
    let (start, end) = (
        report.wall_start_unix_s.unwrap(),
        report.wall_end_unix_s.unwrap(),
    );
    assert!(start > 1.0e9 && end >= start);

    // The camera ended at the last keyframe, looking along the path (-Z).
    let transform = app.world().get::<Transform>(camera).unwrap();
    assert!(transform.translation.distance(Vec3::new(0.0, 2.0, -1.0)) < 1e-5);
    let forward = transform.rotation * Vec3::NEG_Z;
    assert!(forward.distance(Vec3::NEG_Z) < 1e-5);
}

#[test]
fn failed_gate_exits_with_error_and_baseline_round_trips() {
    let dir = tempfile::tempdir().unwrap();
    let baseline = dir.path().join("baselines/fly.json");

    // First run records the baseline.
    let settings = DiagSettings {
        output_dir: dir.path().join("out"),
        baseline: Some(baseline.clone()),
        update_baseline: true,
        exit_on_finish: true,
        ..DiagSettings::default()
    };
    let mut first = app(settings.clone(), Some(path_scenario(Vec::new())));
    assert_eq!(run(&mut first, 200), Some(AppExit::Success));
    assert!(baseline.exists());

    // Second run compares (identical frames: no regression) but fails a
    // 10 ms gate against 15.6 ms frames.
    let settings = DiagSettings {
        update_baseline: false,
        ..settings
    };
    let gates = vec![sf_diag::Gate::MaxFrameMs { ms: 10.0 }];
    let mut second = app(settings, Some(path_scenario(gates)));
    assert_eq!(run(&mut second, 200), Some(AppExit::error()));
    let finished = second.world().resource::<DiagReport>();
    assert!(finished.errors.is_empty(), "{:?}", finished.errors);
    assert!(finished.report.baseline.is_some());
    assert!(finished.report.regressions.is_empty());
    assert!(!finished.passed());
}

#[test]
fn hold_delays_measurement() {
    let mut app = app(
        DiagSettings {
            output_dir: tempfile::tempdir().unwrap().path().to_path_buf(),
            ..DiagSettings::default()
        },
        Some(path_scenario(Vec::new())),
    );
    app.world_mut().resource_mut::<DiagScenario>().hold = true;
    for _ in 0..50 {
        app.update();
    }
    assert_eq!(
        app.world().resource::<DiagScenario>().phase(),
        Phase::Warmup
    );
    app.world_mut().resource_mut::<DiagScenario>().hold = false;
    for _ in 0..3 {
        app.update();
    }
    assert_eq!(
        app.world().resource::<DiagScenario>().phase(),
        Phase::Measure
    );
}

#[test]
fn counters_work_without_a_scenario() {
    let mut app = app(DiagSettings::default(), None);
    for _ in 0..5 {
        app.update();
    }
    assert!(app.should_exit().is_none());
    assert!(app.world().get_resource::<DiagReport>().is_none());
    let counters = app.world().resource::<DiagCounters>();
    assert!(counters.id("test/work/ms").is_some());
}

#[test]
fn entity_churn_is_counted_per_frame() {
    let dir = tempfile::tempdir().unwrap();
    let settings = DiagSettings {
        output_dir: dir.path().to_path_buf(),
        exit_on_finish: true,
        hitch_ms: Some(1.0),
        ..DiagSettings::default()
    };
    let mut app = app(settings, Some(path_scenario(Vec::new())));
    // Two entities per frame.
    app.add_systems(bevy_app::Update, |mut commands: Commands| {
        commands.spawn_batch([(), ()]);
    });
    assert_eq!(run(&mut app, 200), Some(AppExit::Success));
    let report = &app.world().resource::<DiagReport>().report;
    let delta = report.counters[ENTITIES_DELTA].per_frame.unwrap();
    assert!((delta.median - 2.0).abs() < 1e-9, "{delta:?}");
    // Every 15.6 ms frame is over a 1 ms threshold, with its churn.
    let hitches = report.hitches.as_ref().unwrap();
    assert_eq!(hitches.measured, 9);
    let row = hitches
        .rows
        .iter()
        .find(|h| h.phase == sf_diag::RunPhase::Measure)
        .unwrap();
    assert!(row.churn.iter().any(|c| c.counter == ENTITIES_DELTA));
}

#[test]
fn short_type_names() {
    assert_eq!(short_type_name("bevy_mesh::mesh::Mesh"), "Mesh");
    assert_eq!(
        short_type_name("bevy_pbr::extended_material::ExtendedMaterial<a::B, c::D>"),
        "ExtendedMaterial"
    );
    assert_eq!(short_type_name("Plain"), "Plain");
}
