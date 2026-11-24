use clap::Parser;

use super::*;
use crate::perf::PerfFlags;

/// A command with `--bench` and the perf flags, as the game's `view`.
#[derive(Parser)]
struct Cli {
    #[arg(long)]
    bench: Option<std::path::PathBuf>,
    #[command(flatten)]
    perf: PerfFlags,
}

fn parse(args: &[&str]) -> Result<Cli, clap::Error> {
    Cli::try_parse_from(std::iter::once("view").chain(args.iter().copied()))
}

fn standing() -> Scenario {
    Scenario::from_json(r#"{"name":"gc","measure":{"kind":"duration","seconds":10}}"#).unwrap()
}

#[test]
fn ab_flags_parse_and_need_a_bench() {
    let cli = parse(&["--bench", "b.json", "--ab", "groundcover.blades.enabled=false,true", "--ab-phase-s", "3", "--ab-phases", "2"]).unwrap();
    assert_eq!(cli.perf.ab.as_deref(), Some("groundcover.blades.enabled=false,true"));
    assert_eq!((cli.perf.ab_phase_s, cli.perf.ab_phases, cli.perf.ab_settle_s), (Some(3.0), Some(2), None));
    assert!(parse(&["--ab", "x=0,1"]).is_err(), "--ab without --bench");
    let p = &cli.perf;
    let spec = spec(None, p.ab.as_deref(), p.ab_phase_s, p.ab_phases, p.ab_settle_s).unwrap().unwrap();
    assert_eq!((spec.tunable.as_str(), spec.a.as_str(), spec.b.as_str()), ("groundcover.blades.enabled", "false", "true"));
    assert_eq!((spec.phase_s, spec.phases, spec.settle_s), (Some(3.0), Some(2), None));
}

#[test]
fn ab_flag_replaces_the_scenario_field_and_phase_flags_override_it() {
    let file = AbSpec { tunable: "a.b".into(), a: "1".into(), b: "2".into(), phase_s: Some(4.0), phases: Some(3), settle_s: None };
    assert_eq!(spec(Some(&file), None, None, None, None).unwrap(), Some(file.clone()));
    let over = spec(Some(&file), None, None, Some(6), Some(0.25)).unwrap().unwrap();
    assert_eq!((over.phase_s, over.phases, over.settle_s), (Some(4.0), Some(6), Some(0.25)));
    let replaced = spec(Some(&file), Some("c.d=x,y"), None, None, None).unwrap().unwrap();
    assert_eq!((replaced.tunable.as_str(), replaced.phase_s), ("c.d", None));
    assert_eq!(spec(None, None, None, None, None).unwrap(), None);
    assert!(spec(None, None, Some(3.0), None, None).is_err(), "a phase flag without an A/B");
    assert!(spec(None, Some("c.d=x"), None, None, None).is_err());
}

#[test]
fn defaults_come_from_the_tunables_and_their_set_values() {
    let s = AbSpec::parse("t.x=0,1").unwrap();
    let d = schedule(&s, &[]).unwrap();
    assert_eq!((d.phase_s, d.phases, d.settle_s, d.noise_ms), (5.0, 4, 0.5, 0.5));
    assert!((d.total_s() - 50.0).abs() < 1e-9, "1 discarded + 4 kept cycles of 2 x 5 s");
    let set = schedule(&s, &["bench.ab.phase_s=2".to_owned(), "bench.ab.phases=3".to_owned(), "other.name=7".to_owned()]).unwrap();
    assert_eq!((set.phase_s, set.phases), (2.0, 3));
    assert!(schedule(&s, &["bench.ab.phases=0".to_owned()]).is_err(), "out of the tunable's range");
}

#[test]
fn the_scenario_measures_the_whole_schedule_and_paths_are_refused() {
    let s = AbSpec::parse("t.x=0,1").unwrap();
    let sched = schedule(&s, &[]).unwrap();
    let mut standing = standing();
    fit_scenario(&mut standing, s.clone(), &sched, false).unwrap();
    assert_eq!(standing.measure, Measure::Duration { seconds: 50.0 });
    assert_eq!(standing.ab, Some(s.clone()));
    assert!(fit_scenario(&mut self::standing(), s.clone(), &sched, true).unwrap_err().contains("--bench-seconds"));
    let mut path = Scenario::from_json(
        r#"{"name":"walk","measure":{"kind":"path","speed":7,"keyframes":[{"position":[0,0,0]},{"position":[1,0,0]}]}}"#,
    )
    .unwrap();
    assert!(fit_scenario(&mut path, s, &sched, false).unwrap_err().contains("path scenario"));
}

#[test]
fn only_live_tunables_are_flipped() {
    assert_eq!(live_refusal("a", Lifetime::Live, None, None), None);
    assert_eq!(live_refusal("a", Lifetime::Respawn, None, None), None);
    assert_eq!(live_refusal("a", Lifetime::Recompile, None, None), None);
    let restart = live_refusal("shadow.cascades", Lifetime::Restart, None, None).unwrap();
    assert!(restart.contains("shadow.cascades") && restart.contains("not live") && restart.contains("`restart`"), "{restart}");
    let bake = live_refusal("terrain.lod", Lifetime::Bake, None, None).unwrap();
    assert!(bake.contains("`bake`"), "{bake}");
    assert!(live_refusal("gi", Lifetime::Live, Some("needs a restart"), None).unwrap().contains("startup-only"));
    assert!(live_refusal("gi", Lifetime::Live, None, Some("--off gi")).unwrap().contains("not built"));
}

/// A live dummy switch for the running-app test.
const FLAG: Def<bool> = def!(bool "test.ab.flag" = false);

/// The running system, headless, as a benchmark builds it (no console, so nothing else brings the
/// tunable store's plugin): the scenario's warm-up runs with A, then the value flips at every phase
/// boundary of the measured time, A B A B, through the store.
#[test]
fn a_running_bench_flips_the_tunable_at_each_phase_boundary() {
    use std::time::Duration;

    use bevy_app::TaskPoolPlugin;
    use bevy_time::{TimePlugin, TimeUpdateStrategy};
    use sf_diag_bevy::{DiagSettings, SfDiagPlugin};

    let ab = spec(None, Some("test.ab.flag=false,true"), Some(0.25), Some(1), Some(0.0)).unwrap().unwrap();
    let sched = schedule(&ab, &[]).unwrap();
    assert_eq!(sched.phase_count(), 4);
    let mut scenario = Scenario::from_json(r#"{"name":"ab","warmup":{"seconds":0.05,"frames":3},"measure":{"kind":"duration","seconds":1}}"#).unwrap();
    fit_scenario(&mut scenario, ab, &sched, false).unwrap();

    let mut app = App::new();
    // Exact 1/64 s frames: every phase boundary (0.25 s) falls on a frame start.
    app.add_plugins((TaskPoolPlugin::default(), TimePlugin))
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_micros(15_625)))
        .register_tunables(&[&FLAG])
        .add_plugins(SfDiagPlugin::new(DiagSettings::default()).with_scenario(scenario).unwrap())
        .add_plugins(FlipAbPlugin(sched.clone()));
    app.finish();
    app.cleanup();

    let flag = |app: &App| {
        let store = &app.world().resource::<Tunables>().0;
        store.read(store.id_of(&FLAG).unwrap())
    };
    let (mut flips, mut last, mut warm_frames) = (Vec::new(), false, 0);
    // Stop half-way through the last phase: the run's end writes the report files.
    for _ in 0..200 {
        app.update();
        let (phase, t) = {
            let run = app.world().resource::<DiagScenario>();
            (run.phase(), run.measured_s())
        };
        let on = flag(&app);
        match phase {
            Phase::Warmup => {
                warm_frames += 1;
                assert!(!on, "the warm-up runs with A");
            }
            Phase::Measure => {
                assert_eq!(on, sched.arm_at(t) == Arm::B, "the frame starting at {t} s runs {:?}", sched.arm_at(t));
                if on != last {
                    flips.push(t);
                }
                if t >= 0.875 {
                    break;
                }
            }
            Phase::Done => panic!("stopped before the end"),
        }
        last = on;
    }
    assert!(warm_frames >= 3, "{warm_frames} warm-up frames");
    assert_eq!(flips, [0.25, 0.5, 0.75], "B, A, B at the phase boundaries");
    let store = &app.world().resource::<Tunables>().0;
    assert_eq!(store.var(store.id("test.ab.flag").unwrap()).set_by(), SetBy::CommandLine, "at the command-line layer");
}

#[test]
fn the_module_tunables_are_catalogued() {
    let store = crate::perf::tunables::catalog_store().unwrap();
    for def in TUNABLES {
        assert!(store.id(def.name()).is_some(), "{} not in CATALOG", def.name());
    }
}
