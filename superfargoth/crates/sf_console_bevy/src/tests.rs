use bevy_app::{App, Update};
use sf_console::{ArgKind, CommandSpec, Level, Param, SetBy, Value, VarSpec};

use super::*;

#[derive(Resource, Default)]
struct Frames(u32);

#[derive(Resource, Default)]
struct Seen(Vec<ConsoleVarChanged>);

fn app() -> (App, VarId) {
    let mut app = App::new();
    // Registration before the plugin works too.
    let shadows = app.add_console_var(VarSpec::bool("render.shadows", true));
    app.add_plugins(ConsolePlugin)
        .init_resource::<Frames>()
        .init_resource::<Seen>()
        .add_console_command(
            CommandSpec::new("frames.add").param(Param::new("n", ArgKind::int())),
            |inv| {
                let n = u32::try_from(inv.args.int("n").unwrap()).map_err(|e| e.to_string())?;
                inv.ctx.resource_mut::<Frames>().0 += n;
                Ok(())
            },
        )
        .add_systems(
            Update,
            (|mut f: ResMut<Frames>| f.0 += 1).run_if(console_flag(shadows)),
        )
        .add_systems(
            Update,
            |mut r: MessageReader<ConsoleVarChanged>, mut seen: ResMut<Seen>| {
                seen.0.extend(r.read().cloned());
            },
        );
    (app, shadows)
}

#[test]
fn submitted_lines_toggle_systems_and_report_changes() {
    let (mut app, shadows) = app();
    app.update();
    assert_eq!(app.world().resource::<Frames>().0, 1);

    app.world_mut()
        .write_message(ConsoleSubmit::typed("render.shadows off"));
    app.update();
    assert_eq!(
        app.world().resource::<Frames>().0,
        1,
        "gated system skipped the same frame"
    );
    let seen = &app.world().resource::<Seen>().0;
    assert_eq!(
        seen.as_slice(),
        [ConsoleVarChanged {
            id: shadows,
            name: "render.shadows".into(),
            value: Value::Bool(false)
        }]
    );
    assert_eq!(
        app.world()
            .resource::<DevConsole>()
            .history()
            .entries()
            .len(),
        1
    );

    // Changes from code are reported too (a plain `set` from code would be outranked by
    // the value typed in the console; `reset` gives it back).
    app.world_mut()
        .resource_mut::<Tunables>()
        .reset(shadows);
    app.update();
    assert_eq!(app.world().resource::<Frames>().0, 2);
    assert_eq!(app.world().resource::<Seen>().0.len(), 2);
}

#[test]
fn commands_get_the_world() {
    let (mut app, _) = app();
    app.world_mut().write_message(ConsoleSubmit::exec(
        "frames.add 10; frames.add -1; frames.add 5",
    ));
    app.update();
    let world = app.world();
    // 10 added, then the chain stopped at the failing call; the gated system ran once.
    assert_eq!(world.resource::<Frames>().0, 11);
    let console = world.resource::<DevConsole>();
    assert!(
        console.history().entries().next().is_none(),
        "exec lines aren't recorded"
    );
    let last = console.log().lines().last().unwrap();
    assert_eq!(last.level, Level::Error);
    assert!(last.text.starts_with("frames.add: "), "{}", last.text);
}

#[test]
#[should_panic(expected = "already taken")]
fn duplicate_registration_panics() {
    let (mut app, _) = app();
    app.add_console_var(VarSpec::int("render.shadows", 1));
}

#[test]
fn flag_is_false_without_console() {
    let mut app = App::new();
    let mut console = Console::<World>::new();
    let id = console.add_var(VarSpec::bool("x", true)).unwrap();
    app.init_resource::<Frames>().add_systems(
        Update,
        (|mut f: ResMut<Frames>| f.0 += 1).run_if(console_flag(id)),
    );
    app.update();
    assert_eq!(app.world().resource::<Frames>().0, 0);
}

#[derive(Resource, Default)]
struct Applied(Vec<bool>);

/// The `--off` wiring the spec shows: register, command line, apply on change in `Update`.
#[test]
fn set_by_priority_and_same_frame_apply() {
    let mut app = App::new();
    app.add_plugins(ConsolePlugin).init_resource::<Applied>();
    let ssao = app.add_console_var(VarSpec::bool("render.ssao", true).help("SSAO"));
    app.set_console_var(ssao, false, SetBy::CommandLine);
    app.add_systems(
        Update,
        (move |tunables: Res<Tunables>, mut applied: ResMut<Applied>| {
            applied.0.push(tunables.flag(ssao));
        })
        .run_if(console_var_changed(ssao)),
    );
    let applied = |app: &App| app.world().resource::<Applied>().0.clone();

    app.update();
    assert_eq!(applied(&app), [false], "the starting value applies once");
    app.update();
    assert_eq!(applied(&app), [false], "no change, no run");

    // A config file read later can't undo the command line.
    app.world_mut()
        .write_message(ConsoleSubmit::from(SetBy::Config, "render.ssao on"));
    app.update();
    assert_eq!(applied(&app), [false]);

    // Typed in the console: applies the same frame.
    app.world_mut()
        .write_message(ConsoleSubmit::typed("render.ssao on"));
    app.update();
    assert_eq!(applied(&app), [false, true]);
    let tunables = app.world().resource::<Tunables>();
    assert_eq!(tunables.var(ssao).set_by(), SetBy::Console);

    // `reset` takes back the typed value: the command line's is back.
    app.world_mut()
        .write_message(ConsoleSubmit::exec("reset render.ssao"));
    app.update();
    assert_eq!(applied(&app), [false, true, false]);
    let tunables = app.world().resource::<Tunables>();
    assert_eq!(tunables.var(ssao).set_by(), SetBy::CommandLine);
    // `reset <name> all` hands the switch back to every source: now the config file may set it.
    app.world_mut()
        .write_message(ConsoleSubmit::exec("reset render.ssao all"));
    app.update();
    app.world_mut()
        .write_message(ConsoleSubmit::from(SetBy::Config, "render.ssao off"));
    app.update();
    assert_eq!(applied(&app), [false, true, false, true, false]);
}

#[test]
fn console_closed_without_overlay() {
    let mut app = App::new();
    app.add_plugins(ConsolePlugin)
        .init_resource::<Frames>()
        .add_systems(
            Update,
            (|mut f: ResMut<Frames>| f.0 += 1).run_if(console_closed),
        );
    app.update();
    app.world_mut().resource_mut::<ConsoleOpen>().0 = true;
    app.update();
    assert_eq!(app.world().resource::<Frames>().0, 1);
}

mod feature_commands {
    use bevy::MinimalPlugins;
    use sf_tunables_bevy::sf_tunables::{Group, Lifetime, def};
    use sf_tunables_bevy::{Def, FeatureSwitch, FeatureSwitches, TunablesAppExt};

    use super::*;

    const FOG: Def<&'static str> = def!(choice "test.fog" = "high")
        .choices(&["off", "low", "high"])
        .group(Group::Sky);
    const FROXELS: Def<bool> = def!(bool "test.froxels.enabled" = true).group(Group::Sky);
    const GI: Def<bool> = def!(bool "test.gi.enabled" = true)
        .group(Group::Lighting)
        .lifetime(Lifetime::Restart);
    static FOG_SW: FeatureSwitch = FeatureSwitch::choice(&FOG, "off").label("Height fog");
    static FROXELS_SW: FeatureSwitch =
        FeatureSwitch::new(&FROXELS).requires(&["test.fog"]);
    static GI_SW: FeatureSwitch = FeatureSwitch::new(&GI).label("Probe GI");

    fn app() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, ConsolePlugin))
            .register_tunables(&[&FOG, &FROXELS, &GI])
            .add_feature_switch(&FOG_SW)
            .add_feature_switch(&FROXELS_SW)
            .add_feature_switch(&GI_SW);
        app.add_console_var(VarSpec::bool("render.shadows", true));
        app.add_console_var(VarSpec::float("time.scale", 1.0));
        app.update();
        app
    }

    /// Runs `line`, returns the lines it logged.
    fn run(app: &mut App, line: &str) -> Vec<String> {
        let before = app.world().resource::<DevConsole>().log().lines().len();
        app.world_mut().write_message(ConsoleSubmit::exec(line));
        app.update();
        let log = app.world().resource::<DevConsole>().log();
        log.lines().skip(before).map(|l| l.text.clone()).collect()
    }

    fn value(app: &App, name: &str) -> (String, SetBy) {
        let t = app.world().resource::<Tunables>();
        let v = t.var(t.id(name).unwrap());
        (v.value().to_string(), v.set_by())
    }

    #[test]
    fn toggle_round_trips_a_switch_at_the_console_layer() {
        let mut app = app();
        assert_eq!(
            run(&mut app, "toggle test.froxels.enabled"),
            ["test.froxels.enabled: off (console; skips)"]
        );
        assert_eq!(value(&app, "test.froxels.enabled"), ("off".into(), SetBy::Console));
        let switches = app.world().resource::<FeatureSwitches>();
        assert!(!switches.running_cached(1), "applied the same frame");
        assert_eq!(
            run(&mut app, "toggle test.froxels.enabled"),
            ["test.froxels.enabled: on (console; skips)"]
        );
        assert_eq!(value(&app, "test.froxels.enabled"), ("on".into(), SetBy::Console));

        // A choice switch: off is its off choice, on is the default.
        assert_eq!(run(&mut app, "toggle test.fog"), ["test.fog: off (console; skips)"]);
        assert_eq!(value(&app, "test.fog"), ("off".into(), SetBy::Console));
        assert_eq!(run(&mut app, "toggle TEST.FOG"), ["test.fog: high (console; skips)"]);

        // Startup-only: set, and says when it applies.
        assert_eq!(
            run(&mut app, "toggle test.gi.enabled"),
            ["test.gi.enabled: off (console; skips); applies on restart"]
        );
        assert_eq!(value(&app, "test.gi.enabled"), ("off".into(), SetBy::Console));

        // `reset` takes the console layer back as for any variable.
        run(&mut app, "reset test.gi.enabled");
        assert_eq!(value(&app, "test.gi.enabled"), ("on".into(), SetBy::Default));
    }

    #[test]
    fn toggle_reports_blocked_and_refuses_not_built() {
        let mut app = app();
        run(&mut app, "toggle test.fog");
        assert_eq!(
            run(&mut app, "toggle test.froxels.enabled"),
            ["test.froxels.enabled: off (console; skips); blocked by test.fog"]
        );
        assert_eq!(
            run(&mut app, "toggle test.froxels.enabled"),
            ["test.froxels.enabled: on (console; skips); blocked by test.fog"],
            "a blocked switch is still set"
        );
        assert_eq!(value(&app, "test.froxels.enabled"), ("on".into(), SetBy::Console));

        app.world_mut()
            .resource_mut::<FeatureSwitches>()
            .mark_not_built("test.gi.enabled", "--off gi")
            .unwrap();
        let out = run(&mut app, "toggle test.gi.enabled");
        assert_eq!(out.len(), 1);
        assert!(
            out[0].ends_with(
                "test.gi.enabled: not built this run (--off gi); unchanged, it comes back on a \
                 restart without it"
            ),
            "{out:?}"
        );
        assert_eq!(value(&app, "test.gi.enabled"), ("on".into(), SetBy::Default), "unchanged");
    }

    /// FS-2c: a startup-only switch refuses every flip with its hint and changes nothing.
    #[test]
    fn toggle_refuses_a_startup_only_switch() {
        const PART: Def<bool> = def!(bool "test.part" = true).lifetime(Lifetime::Restart);
        static PART_SW: FeatureSwitch =
            FeatureSwitch::new(&PART).startup_only("relaunch with --off part");
        let mut app = app();
        app.register_tunables(&[&PART]).add_feature_switch(&PART_SW);
        assert_eq!(
            run(&mut app, "toggle test.part"),
            ["toggle: test.part: startup only: relaunch with --off part; unchanged"]
        );
        assert_eq!(value(&app, "test.part"), ("on".into(), SetBy::Default), "unchanged");
    }

    #[test]
    fn toggle_still_flips_plain_variables_and_suggests_names() {
        let mut app = app();
        let out = run(&mut app, "toggle render.shadows");
        assert_eq!(value(&app, "render.shadows"), ("off".into(), SetBy::Console));
        let t = app.world().resource::<Tunables>();
        assert_eq!(out, [t.var(t.id("render.shadows").unwrap()).describe()]);
        let out = run(&mut app, "toggle time.scale");
        assert!(out[0].contains("not a feature switch or an on/off variable"), "{out:?}");
        let out = run(&mut app, "toggle froxels");
        assert!(out[0].contains("did you mean test.froxels.enabled"), "{out:?}");
    }

    #[test]
    fn features_lists_by_category_with_state_layer_and_availability() {
        let mut app = app();
        assert_eq!(
            run(&mut app, "features"),
            [
                "lighting:",
                "  test.gi.enabled: on (default; skips; restart) - Probe GI",
                "sky:",
                "  test.fog: high (default; skips) - Height fog",
                "  test.froxels.enabled: on (default; skips)",
            ]
        );
        run(&mut app, "toggle test.fog");
        app.world_mut()
            .resource_mut::<FeatureSwitches>()
            .mark_not_built("test.gi.enabled", "--off gi")
            .unwrap();
        assert_eq!(
            run(&mut app, "features sky"),
            [
                "sky:",
                "  test.fog: off (console; skips) - Height fog",
                "  test.froxels.enabled: on (default; skips; blocked by test.fog)",
            ]
        );
        assert_eq!(
            run(&mut app, "features probe"),
            [
                "lighting:",
                "  test.gi.enabled: on (default; skips; restart; not built: --off gi) - Probe GI",
            ],
            "the label matches too"
        );
        assert_eq!(run(&mut app, "features zzz"), ["no feature switch matches \"zzz\""]);
    }
}
