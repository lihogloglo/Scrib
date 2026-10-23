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
        .resource_mut::<DevConsole>()
        .vars_mut()
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
        (move |console: Res<DevConsole>, mut applied: ResMut<Applied>| {
            applied.0.push(console.vars().flag(ssao));
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
    let console = app.world().resource::<DevConsole>();
    assert_eq!(console.vars().var(ssao).set_by(), SetBy::Console);

    // `reset` hands the switch back: now the config file may set it.
    app.world_mut()
        .write_message(ConsoleSubmit::exec("reset render.ssao"));
    app.update();
    app.world_mut()
        .write_message(ConsoleSubmit::from(SetBy::Config, "render.ssao off"));
    app.update();
    assert_eq!(applied(&app), [false, true, false]);
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
