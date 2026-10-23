use bevy::MinimalPlugins;
use bevy_asset::Assets;
use bevy_camera::primitives::Aabb;
use bevy_camera::visibility::{InheritedVisibility, ViewVisibility, Visibility};
use bevy_input::InputPlugin;
use bevy_math::{Dir3, Vec3};
use bevy_mesh::{Mesh, Mesh3d};
use bevy_shape::{Cuboid, Ray3d};
use bevy_transform::TransformPlugin;
use bevy_transform::components::Transform;
use sf_console::VarSpec;

use super::*;
use crate::{ConsoleAppExt, ConsolePlugin};

fn app() -> App {
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        InputPlugin,
        TransformPlugin,
        ConsolePlugin,
        ConsoleOverlayPlugin,
    ));
    app.add_console_var(VarSpec::bool("test.flag", true));
    app.update();
    app
}

fn console(app: &App) -> &DevConsole {
    app.world().resource::<DevConsole>()
}

fn log_lines(app: &App) -> Vec<String> {
    console(app).log().lines().map(|l| l.text.clone()).collect()
}

fn last_line(app: &App) -> String {
    log_lines(app).pop().unwrap_or_default()
}

/// One key press (and release) as the window would send it.
fn press(app: &mut App, key_code: KeyCode, logical_key: Key) {
    let text = match &logical_key {
        Key::Character(c) => Some(c.clone()),
        _ => None,
    };
    for state in [ButtonState::Pressed, ButtonState::Released] {
        app.world_mut().write_message(KeyboardInput {
            key_code,
            logical_key: logical_key.clone(),
            state,
            text: text.clone().filter(|_| state == ButtonState::Pressed),
            repeat: false,
            window: Entity::PLACEHOLDER,
        });
    }
}

fn type_text(app: &mut App, text: &str) {
    for c in text.chars() {
        // The physical key doesn't matter for text; one that isn't a toggle key.
        press(app, KeyCode::KeyA, Key::Character(c.to_string().into()));
    }
}

fn prompt(app: &App) -> String {
    app.world()
        .resource::<ConsoleOverlay>()
        .prompt
        .text()
        .to_owned()
}

fn is_open(app: &App) -> bool {
    app.world().resource::<ConsoleOpen>().0
}

#[test]
fn front_end_types_completes_submits_and_browses_history() {
    let mut app = app();
    type_text(&mut app, "x");
    app.update();
    assert!(!is_open(&app));
    assert_eq!(prompt(&app), "", "closed: typing goes to the game");

    // The toggle key opens and isn't typed.
    press(&mut app, KeyCode::Backquote, Key::Character("`".into()));
    type_text(&mut app, "tog");
    press(&mut app, KeyCode::Tab, Key::Tab);
    type_text(&mut app, "test.f");
    press(&mut app, KeyCode::Tab, Key::Tab);
    app.update();
    assert!(is_open(&app));
    assert_eq!(prompt(&app), "toggle test.flag ");

    // While open, game systems reading ButtonInput see nothing.
    app.world_mut().write_message(KeyboardInput {
        key_code: KeyCode::KeyW,
        logical_key: Key::Character("w".into()),
        state: ButtonState::Pressed,
        text: None,
        repeat: false,
        window: Entity::PLACEHOLDER,
    });
    app.update();
    assert!(
        !app.world()
            .resource::<ButtonInput<KeyCode>>()
            .pressed(KeyCode::KeyW)
    );

    // Enter submits: the line runs this frame.
    press(&mut app, KeyCode::Enter, Key::Enter);
    app.update();
    let id = console(&app).var_id("test.flag").unwrap();
    assert!(!console(&app).vars().flag(id));
    assert_eq!(prompt(&app), "");
    assert_eq!(last_line(&app), "test.flag = off (default on) [on|off]");
    assert_eq!(
        console(&app).history().entries().collect::<Vec<_>>(),
        ["toggle test.flag"]
    );

    // The input line and scrollback show it.
    let mut input = app.world_mut().query_filtered::<&Text, With<InputLine>>();
    assert_eq!(input.single(app.world()).unwrap().0, "> |");
    let mut spans = app.world_mut().query::<&TextSpan>();
    let shown: String = spans.iter(app.world()).map(|s| s.0.as_str()).collect();
    assert!(
        shown.contains("> toggle test.flag\ntest.flag = off"),
        "{shown}"
    );

    // Up recalls it; the popup lists completions; Escape clears, then closes.
    press(&mut app, KeyCode::ArrowUp, Key::ArrowUp);
    app.update();
    assert_eq!(prompt(&app), "toggle test.flag");
    press(&mut app, KeyCode::Escape, Key::Escape);
    type_text(&mut app, "re");
    app.update();
    let mut popup = app
        .world_mut()
        .query_filtered::<(&Node, &Text), With<Popup>>();
    let (node, text) = popup.single(app.world()).unwrap();
    assert_eq!(node.display, Display::Flex);
    assert_eq!(text.0, "reset");
    press(&mut app, KeyCode::Escape, Key::Escape);
    press(&mut app, KeyCode::Escape, Key::Escape);
    app.update();
    assert!(!is_open(&app));

    // Closed: input reaches the game again.
    app.world_mut().write_message(KeyboardInput {
        key_code: KeyCode::KeyW,
        logical_key: Key::Character("w".into()),
        state: ButtonState::Pressed,
        text: None,
        repeat: false,
        window: Entity::PLACEHOLDER,
    });
    app.update();
    assert!(
        app.world()
            .resource::<ButtonInput<KeyCode>>()
            .pressed(KeyCode::KeyW)
    );
}

/// A reflected game component, to check the inspector's formatting.
#[derive(Component, bevy_reflect::Reflect)]
#[reflect(Component)]
enum Mood {
    Calm,
    Angry { level: u8, targets: Vec<String> },
}

/// A cell with an NPC (pick root) holding a mesh, and a crate elsewhere.
fn scene(app: &mut App) -> (Entity, Entity, Entity) {
    let world = app.world_mut();
    let cell = world
        .spawn((
            Name::new("Cell"),
            Transform::from_xyz(10.0, 0.0, 0.0),
            Visibility::default(),
        ))
        .id();
    let npc = world
        .spawn((
            Name::new("Guard"),
            PickRoot,
            Transform::from_xyz(1.0, 2.0, 3.0),
            Visibility::default(),
            ChildOf(cell),
        ))
        .id();
    let mesh = world
        .spawn((Name::new("Head"), Transform::default(), ChildOf(npc)))
        .id();
    world.spawn((Name::new("crate_01"), Transform::default()));
    (cell, npc, mesh)
}

fn run(app: &mut App, line: &str) {
    app.world_mut().write_message(ConsoleSubmit::typed(line));
    app.update();
}

#[test]
fn commands_act_on_the_selection() {
    let mut app = app();
    app.register_type::<Name>().register_type::<Mood>();
    app.add_inspector_rows(|world, e, rows| {
        if world.get::<PickRoot>(e).is_some() {
            rows.push(("record".into(), "npc_guard".into()));
        }
    });
    let (cell, npc, _) = scene(&mut app);
    app.world_mut().entity_mut(npc).insert(Mood::Angry {
        level: 3,
        targets: vec!["player".into(), "dog".into()],
    });
    let _ = Mood::Calm;

    run(&mut app, "getpos");
    assert_eq!(
        last_line(&app),
        "getpos: nothing selected: click an object, or `prid <name>`"
    );
    run(&mut app, "prid nobody");
    assert_eq!(
        last_line(&app),
        "prid: no entity named \"nobody\"; try `find nobody`"
    );

    run(&mut app, "prid guard; getpos");
    assert_eq!(app.world().resource::<ConsoleSelection>().0, Some(npc));
    assert_eq!(
        last_line(&app),
        format!("Guard ({npc}): 11.000 2.000 3.000")
    );

    // World position in, local position kept under the parent; same-line getpos sees it.
    run(&mut app, "setpos 0 0 -1.5; getpos");
    assert_eq!(
        app.world().get::<Transform>(npc).unwrap().translation,
        Vec3::new(-10.0, 0.0, -1.5)
    );
    assert_eq!(
        last_line(&app),
        format!("Guard ({npc}): 0.000 0.000 -1.500")
    );
    run(&mut app, "setpos 1 2");
    assert!(last_line(&app).starts_with("setpos: missing <z>"));

    run(&mut app, "disable");
    assert_eq!(
        app.world().get::<Visibility>(npc),
        Some(&Visibility::Hidden)
    );
    run(&mut app, "enable");
    assert_eq!(
        app.world().get::<Visibility>(npc),
        Some(&Visibility::Inherited)
    );

    run(&mut app, "clear; find \"D\"");
    assert_eq!(
        log_lines(&app),
        [
            "2 entities match \"d\"".to_owned(),
            format!("  Guard ({npc})"),
            format!("  Head ({})", app.world().get::<Children>(npc).unwrap()[0]),
        ]
    );

    // The inspector: name, id, transform, parents, the app's game row, components.
    let rows = inspect(app.world(), npc);
    let get = |k: &str| {
        rows.iter()
            .find(|(key, _)| key == k)
            .map(|(_, v)| v.as_str())
    };
    assert_eq!(get("name"), Some("Guard"));
    assert_eq!(get("entity"), Some(npc.to_string().as_str()));
    assert_eq!(get("translation"), Some("-10.000 0.000 -1.500"));
    assert_eq!(get("world"), Some("0.000 0.000 -1.500"));
    assert_eq!(get("  Name"), None, "shown above, not again");
    assert_eq!(get("parents"), Some(format!("Cell ({cell})").as_str()));
    assert_eq!(get("children"), Some("1"));
    assert_eq!(get("record"), Some("npc_guard"));
    // Registered components show their name and value; others say so.
    assert_eq!(get("  PickRoot"), Some("marker"), "{rows:?}");
    assert_eq!(
        get("  Mood"),
        Some("Angry { level: 3, targets: [2: \"player\", \"dog\"] }"),
        "{rows:?}"
    );

    // The panel shows while open with a selection.
    app.world_mut().resource_mut::<ConsoleOpen>().0 = true;
    app.update();
    let mut panel = app
        .world_mut()
        .query_filtered::<(&Node, &Children), With<inspect::InspectorPanel>>();
    let (node, children) = panel.single(app.world()).unwrap();
    assert_eq!(node.display, Display::Flex);
    let title = app.world().get::<Text>(children[0]).unwrap();
    assert_eq!(title.0, format!("Guard ({npc})\n"));

    run(&mut app, "delete");
    assert!(app.world().get_entity(npc).is_err());
    assert_eq!(app.world().resource::<ConsoleSelection>().0, None);
    assert_eq!(last_line(&app), format!("deleted Guard ({npc})"));
}

/// A unit cube mesh at `at` that the ray cast can see.
fn cube(app: &mut App, at: Vec3, visible: bool) -> Entity {
    let mesh = Mesh::from(Cuboid::default());
    let aabb = Aabb::from_min_max(Vec3::splat(-0.5), Vec3::splat(0.5));
    let handle = app.world_mut().resource_mut::<Assets<Mesh>>().add(mesh);
    let visibility = if visible {
        InheritedVisibility::VISIBLE
    } else {
        InheritedVisibility::HIDDEN
    };
    app.world_mut()
        .spawn((
            Mesh3d(handle),
            aabb,
            Transform::from_translation(at),
            visibility,
            ViewVisibility::default(),
        ))
        .id()
}

fn pick(app: &mut App, direction: Dir3) -> Option<Entity> {
    app.world_mut().write_message(ConsolePick {
        ray: Ray3d::new(Vec3::ZERO, direction),
    });
    app.update();
    app.world().resource::<ConsoleSelection>().0
}

#[test]
fn pick_ray_selects_the_nearest_visible_mesh_root() {
    let mut app = app();
    app.init_resource::<Assets<Mesh>>();
    // Down -Z: a hidden cube at 3, the NPC's mesh at 5, a wall at 9.
    let npc = app
        .world_mut()
        .spawn((PickRoot, Transform::from_xyz(0.0, 0.0, -5.0)))
        .id();
    let body = cube(&mut app, Vec3::ZERO, true);
    app.world_mut().entity_mut(body).insert(ChildOf(npc));
    cube(&mut app, Vec3::new(0.0, 0.0, -3.0), false);
    let wall = cube(&mut app, Vec3::new(0.0, 0.0, -9.0), true);
    // Down +X: a loose mesh with no pick root.
    let rock = cube(&mut app, Vec3::new(4.0, 0.0, 0.0), true);
    app.update(); // GlobalTransform

    assert_eq!(pick(&mut app, Dir3::NEG_Z), Some(npc));
    assert_eq!(pick(&mut app, Dir3::X), Some(rock));
    assert_eq!(pick(&mut app, Dir3::Y), None, "a miss deselects");
    app.world_mut().despawn(npc);
    assert_eq!(pick(&mut app, Dir3::NEG_Z), Some(wall));
}
