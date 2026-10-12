//! Synthetic cloud test scene (weather.md "Clouds"): a flat 24 km ground
//! under the sky and the clouds, for each genus, the cloud shadow edge and
//! the sun behind cloud.
//!
//! ```sh
//! cargo run --release -p sf_atmos_bevy --example clouds -- [--preset fair] [--time 12]
//!     [--view sky|sun|shadow|up] [--scale 1] [--wind 1] [--screenshot out.png] [--frames 150]
//! ```
//!
//! Views: `sky` (eye height, looking along +X at 20°), `sun` (aimed at the sun:
//! is the disk hidden behind thick cloud?), `shadow` (200 m up, looking down
//! at 35°: cloud shadow patches and their edges on the ground), `up` (zenith).
//! Keys: C next genus, Space pause/run the clock, Up/Down wind ×2 / ÷2.

use std::path::PathBuf;

use bevy::camera::visibility::NoFrustumCulling;
use bevy::post_process::bloom::Bloom;
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use bevy::window::WindowResolution;
use sf_atmos_bevy::sf_atmos::{CloudPreset, CloudSettings};
use sf_atmos_bevy::{CloudCamera, CloudShadowLight, Clouds, CloudsPlugin};
use sf_sky_bevy::sf_sky::{RenderPath, SkyClock, SkySettings};
use sf_sky_bevy::{CurrentSky, SkyCamera, SkyPlugin, SkyTime, SunLight};

#[derive(Resource, Clone)]
struct Args {
    preset: CloudPreset,
    hour: f64,
    view: String,
    scale: f32,
    wind: f32,
    screenshot: Option<PathBuf>,
    frames: u32,
}

fn parse_args() -> Args {
    let mut a = Args { preset: CloudPreset::Fair, hour: 12.0, view: "sky".into(), scale: 1.0, wind: 1.0, screenshot: None, frames: 150 };
    let mut it = std::env::args().skip(1);
    while let Some(flag) = it.next() {
        let mut value = || it.next().unwrap_or_else(|| panic!("{flag} needs a value"));
        match flag.as_str() {
            "--preset" => a.preset = CloudPreset::parse(&value()).expect("--preset <genus>"),
            "--time" => a.hour = value().parse().expect("--time <hours>"),
            "--view" => a.view = value(),
            "--scale" => a.scale = value().parse().expect("--scale <factor>"),
            "--wind" => a.wind = value().parse().expect("--wind <factor>"),
            "--screenshot" => a.screenshot = Some(PathBuf::from(value())),
            "--frames" => a.frames = value().parse().expect("--frames <count>"),
            other => panic!("unknown flag {other}"),
        }
    }
    a
}

fn settings(preset: CloudPreset, scale: f32, wind: f32) -> CloudSettings {
    let mut s = CloudSettings::preset(preset);
    s.weather = s.weather.scaled_heights(scale);
    s.look = s.look.scaled(scale);
    s.weather.wind *= wind;
    s
}

fn main() {
    let args = parse_args();
    let sky = SkySettings::default();
    let mut clock = SkyClock::at(172.0, args.hour, &sky.calendar);
    clock.time_scale = 30.0;
    clock.paused = true;
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "sf_atmos clouds".into(),
                resolution: WindowResolution::new(1280, 720).with_scale_factor_override(1.0),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(SkyPlugin { settings: sky, clock, render_path: RenderPath::Raster })
        .add_plugins(CloudsPlugin { settings: settings(args.preset, args.scale, args.wind), seed: 0x5EED_C10D })
        .insert_resource(args)
        .add_systems(Startup, setup)
        .add_systems(Update, (shadow_light, aim, keys, screenshot))
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>, args: Res<Args>) {
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(24_000.0, 24_000.0).build())),
        MeshMaterial3d(materials.add(StandardMaterial { base_color: Color::srgb(0.30, 0.34, 0.26), perceptual_roughness: 0.95, ..default() })),
        NoFrustumCulling,
    ));
    // Scale markers: 20 m pillars every 500 m along +X.
    let pillar = meshes.add(Cuboid::new(6.0, 20.0, 6.0));
    let stone = materials.add(StandardMaterial { base_color: Color::srgb(0.75, 0.72, 0.66), ..default() });
    for i in 1..=12u8 {
        commands.spawn((Mesh3d(pillar.clone()), MeshMaterial3d(stone.clone()), Transform::from_xyz(f32::from(i) * 500.0, 10.0, -60.0)));
    }
    let transform = match args.view.as_str() {
        "shadow" => Transform::from_xyz(0.0, 200.0, 0.0).looking_to(Vec3::new(1.0, -0.7, 0.2), Vec3::Y),
        "up" => Transform::from_xyz(0.0, 2.0, 0.0).looking_to(Vec3::Y, Vec3::X),
        _ => Transform::from_xyz(0.0, 2.0, 0.0).looking_to(Vec3::new(1.0, 0.36, 0.0), Vec3::Y),
    };
    commands.spawn((SkyCamera::default(), CloudCamera, Camera3d::default(), transform, Bloom::NATURAL));
}

/// The sun receives the cloud shadow map (as in the app).
fn shadow_light(mut commands: Commands, suns: Query<Entity, (With<SunLight>, Without<CloudShadowLight>)>) {
    for sun in &suns {
        commands.entity(sun).insert(CloudShadowLight);
    }
}

/// `--view sun`: keep the camera on the sun.
fn aim(args: Res<Args>, sky: Res<CurrentSky>, mut cameras: Query<&mut Transform, With<CloudCamera>>) {
    if args.view != "sun" {
        return;
    }
    let to_sun = sky.0.sun.direction;
    for mut t in &mut cameras {
        *t = t.looking_to(to_sun, Vec3::Y);
    }
}

fn keys(input: Res<ButtonInput<KeyCode>>, mut clouds: ResMut<Clouds>, args: Res<Args>, mut time: ResMut<SkyTime>, mut genus: Local<Option<usize>>) {
    if input.just_pressed(KeyCode::KeyC) {
        let i = genus.unwrap_or_else(|| CloudPreset::ALL.iter().position(|(_, p)| *p == args.preset).unwrap_or(0));
        let next = (i + 1) % CloudPreset::ALL.len();
        *genus = Some(next);
        let (name, preset) = CloudPreset::ALL[next];
        clouds.settings = settings(preset, args.scale, args.wind);
        info!("clouds: {name}");
    }
    if input.just_pressed(KeyCode::Space) {
        time.0.paused = !time.0.paused;
    }
    if input.just_pressed(KeyCode::ArrowUp) {
        clouds.settings.weather.wind *= 2.0;
    }
    if input.just_pressed(KeyCode::ArrowDown) {
        clouds.settings.weather.wind *= 0.5;
    }
}

/// `--screenshot`: render `--frames` frames (noise + temporal history), save, exit.
fn screenshot(mut commands: Commands, args: Res<Args>, mut frame: Local<u32>, shots: Query<(), With<Screenshot>>, mut exit: MessageWriter<AppExit>) {
    let Some(path) = &args.screenshot else { return };
    *frame += 1;
    if *frame == args.frames {
        commands.spawn(Screenshot::primary_window()).observe(save_to_disk(path.clone()));
    } else if *frame > args.frames + 5 && shots.is_empty() {
        exit.write(AppExit::Success);
    }
}
