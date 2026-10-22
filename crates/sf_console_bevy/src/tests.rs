use bevy_app::{App, Update};
use sf_console::{ArgKind, CommandSpec, Level, Param, Value, VarSpec};

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

    // Changes from code are reported too.
    app.world_mut()
        .resource_mut::<DevConsole>()
        .vars_mut()
        .set(shadows, Value::Bool(true))
        .unwrap();
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
