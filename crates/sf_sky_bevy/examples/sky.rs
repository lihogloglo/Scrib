//! Sky, sun, moon and the raster / ray-traced switch on a small test scene.
//!
//! ```text
//! cargo run -p sf_sky_bevy --features solari --example sky
//! cargo run -p sf_sky_bevy --features solari --example sky -- \
//!     --screenshot noon.png --time 12 [--path rt] [--day 172] [--frames 90]
//! ```
//!
//! Keys: R raster/ray-traced, Space pause, Up/Down time speed ×2 / ÷2,
//! 1–5 jump to 6:00, 12:00, 19:30, 21:00, 0:00. Status is in the title bar.

use std::path::PathBuf;

use bevy::camera::Exposure;
use bevy::core_pipeline::prepass::DepthPrepass;
use bevy::post_process::bloom::Bloom;
use bevy::prelude::*;
use bevy::render::occlusion_culling::OcclusionCulling;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use bevy::window::WindowResolution;
use sf_sky_bevy::sf_sky::{RenderPath, SkyClock, SkySettings};
use sf_sky_bevy::{
    ActiveRenderPath, CurrentSky, NoRayTracing, RayTracingSupport, RenderPathSetting, SkyCamera,
    SkyConfig, SkyPlugin, SkyTime,
};

#[derive(Resource, Clone, Debug)]
struct Args {
    screenshot: Option<PathBuf>,
    hour: f64,
    day: f64,
    path: RenderPath,
    frames: u32,
    switch_at: Option<u32>,
}

fn parse_args() -> Args {
    let mut args = Args {
        screenshot: None,
        hour: 9.0,
        day: 172.0,
        path: RenderPath::Raster,
        frames: 90,
        switch_at: None,
    };
    let mut it = std::env::args().skip(1);
    while let Some(arg) = it.next() {
        let mut value = || it.next().unwrap_or_else(|| panic!("{arg} needs a value"));
        match arg.as_str() {
            "--screenshot" => args.screenshot = Some(PathBuf::from(value())),
            "--time" => args.hour = value().parse().expect("--time <hours>"),
            "--day" => args.day = value().parse().expect("--day <day of year>"),
            "--frames" => args.frames = value().parse().expect("--frames <count>"),
            "--switch-at" => args.switch_at = Some(value().parse().expect("--switch-at <frame>")),
            "--path" => {
                args.path = match value().as_str() {
                    "rt" | "solari" | "raytraced" => RenderPath::RayTraced,
                    _ => RenderPath::Raster,
                }
            }
            other => panic!("unknown argument {other}"),
        }
    }
    args
}

fn main() {
    let args = parse_args();
    let settings = SkySettings::default();
    let mut clock = SkyClock::at(args.day, args.hour, &settings.calendar);
    clock.time_scale = 1800.0;
    clock.paused = args.screenshot.is_some();

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "sf_sky".into(),
                resolution: WindowResolution::new(1280, 720).with_scale_factor_override(1.0),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(SkyPlugin {
            settings,
            clock,
            render_path: args.path,
        })
        .insert_resource(args)
        .add_systems(Startup, setup)
        .add_systems(Update, (controls, status, screenshot))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Solari needs position/normal/uv/tangent and u32 indices.
    let mut mesh = |m: Mesh| meshes.add(m.with_generated_tangents().expect("tangents"));
    let ground = mesh(Plane3d::default().mesh().size(400.0, 400.0).build());
    let cube = mesh(Cuboid::default().mesh().build());
    let ball = mesh(Sphere::new(1.0).mesh().uv(32, 18));

    commands.spawn((
        Mesh3d(ground),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.32, 0.36, 0.28),
            perceptual_roughness: 0.9,
            ..default()
        })),
    ));

    let boxes = [
        (
            Vec3::new(0.0, 1.5, 0.0),
            Vec3::new(3.0, 3.0, 3.0),
            Color::srgb(0.8, 0.75, 0.65),
        ),
        (
            Vec3::new(-6.0, 3.0, -4.0),
            Vec3::new(2.5, 6.0, 2.5),
            Color::srgb(0.7, 0.3, 0.25),
        ),
        (
            Vec3::new(5.0, 1.0, -6.0),
            Vec3::new(4.0, 2.0, 2.0),
            Color::srgb(0.3, 0.45, 0.7),
        ),
        (
            Vec3::new(-3.0, 0.5, 6.0),
            Vec3::new(1.0, 1.0, 1.0),
            Color::srgb(0.9, 0.9, 0.9),
        ),
        (
            Vec3::new(9.0, 4.0, 3.0),
            Vec3::new(1.5, 8.0, 1.5),
            Color::srgb(0.55, 0.55, 0.5),
        ),
    ];
    for (pos, size, color) in boxes {
        commands.spawn((
            Mesh3d(cube.clone()),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: color,
                perceptual_roughness: 0.7,
                ..default()
            })),
            Transform::from_translation(pos).with_scale(size),
        ));
    }
    commands.spawn((
        Mesh3d(ball.clone()),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.95, 0.95, 0.95),
            metallic: 1.0,
            perceptual_roughness: 0.15,
            ..default()
        })),
        Transform::from_xyz(3.0, 1.0, 4.0),
    ));

    spawn_lamps(&mut commands, &mut materials, &cube, &ball);

    commands.spawn((
        SkyCamera::default(),
        Camera3d::default(),
        Transform::from_xyz(14.0, 3.5, 12.0).looking_at(Vec3::new(-4.0, 2.5, -4.0), Vec3::Y),
        Bloom::NATURAL,
        DepthPrepass,
        OcclusionCulling,
    ));
}

/// Lamps: a post, a visible glowing bulb (raster only, so it doesn't occlude
/// the ray tracer's proxy inside it), and the point light.
fn spawn_lamps(
    commands: &mut Commands,
    materials: &mut Assets<StandardMaterial>,
    cube: &Handle<Mesh>,
    ball: &Handle<Mesh>,
) {
    let post_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.12, 0.12, 0.12),
        ..default()
    });
    for (i, pos) in [
        Vec3::new(-4.0, 0.0, 2.5),
        Vec3::new(3.5, 0.0, -2.5),
        Vec3::new(-8.0, 0.0, 4.0),
        Vec3::new(6.0, 0.0, 6.0),
    ]
    .into_iter()
    .enumerate()
    {
        let color = if i % 2 == 0 {
            Color::srgb(1.0, 0.72, 0.42)
        } else {
            Color::srgb(0.75, 0.85, 1.0)
        };
        commands.spawn((
            Mesh3d(cube.clone()),
            MeshMaterial3d(post_material.clone()),
            Transform::from_translation(pos + Vec3::Y * 1.1).with_scale(Vec3::new(0.12, 2.2, 0.12)),
        ));
        commands.spawn((
            Mesh3d(ball.clone()),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::BLACK,
                emissive: color.to_linear() * 20_000.0,
                ..default()
            })),
            Transform::from_translation(pos + Vec3::Y * 2.4).with_scale(Vec3::splat(0.12)),
            NoRayTracing,
        ));
        commands.spawn((
            PointLight {
                color,
                intensity: 3_000.0,
                range: 20.0,
                radius: 0.05,
                shadow_maps_enabled: true,
                ..default()
            },
            Transform::from_translation(pos + Vec3::Y * 2.4),
        ));
    }
}

fn controls(
    keys: Res<ButtonInput<KeyCode>>,
    mut setting: ResMut<RenderPathSetting>,
    mut time: ResMut<SkyTime>,
    config: Res<SkyConfig>,
) {
    if keys.just_pressed(KeyCode::KeyR) {
        setting.0 = match setting.0 {
            RenderPath::Raster => RenderPath::RayTraced,
            RenderPath::RayTraced => RenderPath::Raster,
        };
    }
    let clock = &mut time.0;
    if keys.just_pressed(KeyCode::Space) {
        clock.paused = !clock.paused;
    }
    if keys.just_pressed(KeyCode::ArrowUp) {
        clock.time_scale *= 2.0;
    }
    if keys.just_pressed(KeyCode::ArrowDown) {
        clock.time_scale /= 2.0;
    }
    for (key, hour) in [
        (KeyCode::Digit1, 6.0),
        (KeyCode::Digit2, 12.0),
        (KeyCode::Digit3, 19.5),
        (KeyCode::Digit4, 21.0),
        (KeyCode::Digit5, 0.0),
    ] {
        if keys.just_pressed(key) {
            clock.set_hour(hour, &config.0.calendar);
        }
    }
}

fn status(
    sky: Res<CurrentSky>,
    path: Res<ActiveRenderPath>,
    support: Res<RayTracingSupport>,
    time: Res<SkyTime>,
    mut window: Single<&mut Window>,
    exposure: Single<&Exposure, With<SkyCamera>>,
) {
    let s = &sky.0;
    #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let (h, m) = (s.hour as u32, (s.hour.fract() * 60.0) as u32);
    window.title = format!(
        "sf_sky  {h:02}:{m:02}  x{:.0}{}  sun {:.1}°  moon {:.1}° ({:.0}%)  EV {:.1}  {:?}{}",
        time.0.time_scale,
        if time.0.paused { " paused" } else { "" },
        s.celestial.sun.elevation_deg(),
        s.celestial.moon.elevation_deg(),
        s.celestial.moon_illuminated * 100.0,
        exposure.ev100,
        path.0,
        if support.0 { "" } else { " (no RT support)" },
    );
}

/// `--screenshot`: render `--frames` frames, save, wait for the write, exit.
fn screenshot(
    mut commands: Commands,
    args: Res<Args>,
    mut frame: Local<u32>,
    shots: Query<(), With<Screenshot>>,
    mut exit: MessageWriter<AppExit>,
    mut setting: ResMut<RenderPathSetting>,
) {
    let Some(path) = &args.screenshot else {
        return;
    };
    *frame += 1;
    // `--switch-at`: flip the render path mid-run (tests the runtime switch).
    if args.switch_at == Some(*frame) {
        setting.0 = match setting.0 {
            RenderPath::Raster => RenderPath::RayTraced,
            RenderPath::RayTraced => RenderPath::Raster,
        };
    }
    if *frame == args.frames {
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(path.clone()));
    } else if *frame > args.frames + 5 && shots.is_empty() {
        exit.write(AppExit::Success);
    }
}
