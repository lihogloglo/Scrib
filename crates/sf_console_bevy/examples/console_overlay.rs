//! The console overlay over a small scene: three named cubes on a floor.
//!
//! `cargo run -p sf_console_bevy --example console_overlay --features overlay`
//!
//! Backtick or F1 opens the console. Click a cube to select it (yellow box, inspector on the
//! right), then `getpos`, `setpos 0 2 0`, `moveto`, `disable`, `enable`, `delete`. `prid
//! crate_b` selects by name, `find crate` lists names. `demo.spin off` stops the cubes the
//! frame it is typed; `demo.speed 3`, `list`, `reset`. Start with `-- --no-spin` to see a
//! command-line switch (`set by command line`) that a typed value still overrides.
//!
//! `-- --autotest [--screenshot <file.png>]` drives it without a person: opens the console,
//! runs lines, picks `crate_b` with a ray, leaves `re` in the input line (completion popup),
//! saves a screenshot, checks the results and exits non-zero on a failure.

use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use sf_console_bevy::overlay::{
    ConsoleCamera, ConsoleOverlay, ConsoleOverlayAppExt, ConsoleOverlayPlugin, ConsolePick,
    ConsoleSelection, PickRoot,
};
use sf_console_bevy::sf_console::{VarId, VarSpec};
use sf_console_bevy::{
    ConsoleAppExt, ConsoleOpen, ConsolePlugin, ConsoleSubmit, DevConsole, SetBy, console_closed,
};

#[derive(Resource)]
struct Switches {
    spin: VarId,
    speed: VarId,
}

/// A game component the inspector hook reads.
#[derive(Component)]
struct Kind(&'static str);

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, ConsolePlugin, ConsoleOverlayPlugin));
    let spin = app.add_console_var(VarSpec::bool("demo.spin", true).help("Rotate the cubes"));
    let speed = app.add_console_var(
        VarSpec::float("demo.speed", 1.0)
            .float_range(0.0, 10.0)
            .help("Rotation speed, turns per ten seconds"),
    );
    if std::env::args().any(|a| a == "--no-spin") {
        app.set_console_var(spin, false, SetBy::CommandLine);
    }
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--autotest") {
        let shot = args
            .iter()
            .position(|a| a == "--screenshot")
            .and_then(|i| args.get(i + 1))
            .cloned()
            .unwrap_or_else(|| "console_overlay.png".into());
        app.insert_resource(Shot(shot))
            .add_systems(Update, autotest);
    }
    app.insert_resource(Switches { spin, speed })
        .add_inspector_rows(|world, e, rows| {
            if let Some(kind) = world.get::<Kind>(e) {
                rows.push(("kind".into(), kind.0.into()));
            }
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate, fly.run_if(console_closed)))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name::new("camera"),
        Camera3d::default(),
        ConsoleCamera,
        Transform::from_xyz(0.0, 4.0, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(3.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        Name::new("floor"),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.32, 0.3))),
    ));
    let cube = meshes.add(Cuboid::default());
    for (i, (name, color)) in [
        ("crate_a", Color::srgb(0.8, 0.3, 0.2)),
        ("crate_b", Color::srgb(0.2, 0.6, 0.8)),
        ("crate_c", Color::srgb(0.6, 0.8, 0.2)),
    ]
    .into_iter()
    .enumerate()
    {
        #[allow(clippy::cast_precision_loss)]
        let x = (i as f32 - 1.0) * 2.5;
        // The pick root holds the mesh one level down, as a loaded model would.
        commands
            .spawn((
                Name::new(name),
                PickRoot,
                Kind("crate"),
                Transform::from_xyz(x, 0.5, 0.0),
                Visibility::default(),
            ))
            .with_child((
                Name::new(format!("{name}_mesh")),
                Mesh3d(cube.clone()),
                MeshMaterial3d(materials.add(color)),
            ));
    }
}

fn rotate(
    time: Res<Time>,
    console: Res<DevConsole>,
    switches: Res<Switches>,
    mut crates: Query<&mut Transform, With<Kind>>,
) {
    if !console.vars().flag(switches.spin) {
        return;
    }
    #[allow(clippy::cast_possible_truncation)]
    let speed = console.vars().float(switches.speed) as f32;
    for mut t in &mut crates {
        t.rotate_y(time.delta_secs() * speed * std::f32::consts::TAU / 10.0);
    }
}

/// Game input: WASD moves the camera. Gated, so typing in the console doesn't fly it.
fn fly(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut camera: Query<&mut Transform, With<ConsoleCamera>>,
) {
    let Ok(mut t) = camera.single_mut() else {
        return;
    };
    let mut v = Vec3::ZERO;
    for (key, dir) in [
        (KeyCode::KeyW, t.forward().as_vec3()),
        (KeyCode::KeyS, -t.forward().as_vec3()),
        (KeyCode::KeyA, -t.right().as_vec3()),
        (KeyCode::KeyD, t.right().as_vec3()),
    ] {
        if keys.pressed(key) {
            v += dir;
        }
    }
    t.translation += v * 5.0 * time.delta_secs();
}

#[derive(Resource)]
struct Shot(String);

/// Scripted run: frame numbers leave time for startup, layout and the screenshot.
#[allow(clippy::too_many_arguments)]
fn autotest(
    mut frame: Local<u32>,
    mut commands: Commands,
    shot: Res<Shot>,
    mut open: ResMut<ConsoleOpen>,
    mut overlay: ResMut<ConsoleOverlay>,
    mut submit: MessageWriter<ConsoleSubmit>,
    mut picks: MessageWriter<ConsolePick>,
    console: Res<DevConsole>,
    switches: Res<Switches>,
    selection: Res<ConsoleSelection>,
    camera: Query<&GlobalTransform, With<ConsoleCamera>>,
    named: Query<(Entity, &Name, &GlobalTransform)>,
    mut exit: MessageWriter<AppExit>,
) {
    *frame += 1;
    let crate_b = named
        .iter()
        .find(|(_, n, _)| n.as_str() == "crate_b")
        .map(|(e, _, t)| (e, t.translation()));
    match *frame {
        30 => {
            open.0 = true;
            submit.write(ConsoleSubmit::typed("help prid; list demo"));
        }
        40 => {
            if let (Ok(cam), Some((_, target))) = (camera.single(), crate_b) {
                let from = cam.translation();
                let dir = Dir3::new(target - from).expect("camera isn't at the crate");
                picks.write(ConsolePick {
                    ray: Ray3d::new(from, dir),
                });
            }
        }
        50 => {
            submit.write(ConsoleSubmit::typed("getpos; demo.spin off; frobnicate"));
            overlay.prompt.set("re");
        }
        80 => {
            commands
                .spawn(Screenshot::primary_window())
                .observe(save_to_disk(shot.0.clone()));
        }
        110 => {
            let mut failures = Vec::new();
            if selection.0 != crate_b.map(|(e, _)| e) {
                failures.push(format!("selection is {:?}, not crate_b", selection.0));
            }
            if console.vars().flag(switches.spin) {
                failures.push("demo.spin is still on".to_owned());
            }
            let log: Vec<&str> = console.log().lines().map(|l| l.text.as_str()).collect();
            if !log.iter().any(|l| l.starts_with("crate_b (")) {
                failures.push("getpos printed nothing for crate_b".to_owned());
            }
            for f in &failures {
                error!("autotest: {f}");
            }
            info!(
                "autotest: {} failure(s), screenshot {}",
                failures.len(),
                shot.0
            );
            exit.write(if failures.is_empty() {
                AppExit::Success
            } else {
                AppExit::error()
            });
        }
        _ => {}
    }
}
