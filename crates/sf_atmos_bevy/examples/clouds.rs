//! Synthetic cloud test scene (weather.md "Clouds"): a flat 24 km ground
//! under the sky and the clouds, for each genus, the cloud shadow edge and
//! the sun behind cloud.
//!
//! ```sh
//! cargo run --release -p sf_atmos_bevy --example clouds -- [--preset fair] [--time 12]
//!     [--view sky|sun|shadow|up] [--scale 1] [--wind 1] [--fog 0] [--screenshot out.png] [--frames 150]
//!     [--split clear,storm] [--per-ray]
//! ```
//!
//! Views: `sky` (eye height, looking along +X at 20°), `sun` (aimed at the sun:
//! is the disk hidden behind thick cloud?), `shadow` (200 m up, looking down
//! at 35°: cloud shadow patches and their edges on the ground), `up` (zenith).
//! `--fog <visibility m>` adds height fog (100 m falloff) and ground-fog banks
//! (weather.md "Fog", F2): from the `shadow` view the banks lie on the plane.
//! `glass` (eye height along +X): glass panes (`FroxelFogMaterial`, blended) beside
//! stone pillars at 20, 60, 150 and 300 m, and a lantern (point light) by each:
//! with `--fog 400` the panes fade like the pillars (transparent fog), and at
//! night (`--time 23`) the lanterns glow in the froxel fog (F3).
//! Keys: C next genus, Space pause/run the clock, Up/Down wind ×2 / ÷2.

use std::path::PathBuf;

use bevy::camera::visibility::NoFrustumCulling;
use bevy::post_process::bloom::Bloom;
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use bevy::window::WindowResolution;
use sf_atmos_bevy::sf_atmos::{fog, CloudPreset, CloudSettings, FogLayer, FogSettings};
use sf_atmos_bevy::{CloudCamera, CloudShadowLight, CloudWeatherMap, Clouds, CloudsPlugin, FroxelFogExt, FroxelFogMaterial, FroxelVolume};
use bevy::math::DVec2;
use sf_weather::{ClimateIx, ClimateRaster, ClimateRow, MAP_SIZE, WeatherMap, row};
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
    /// Height-fog visibility at the ground, meters (0: no fog).
    fog: f32,
    /// `--split west,east`: two climates side by side on the world weather map (`WE4b`),
    /// border at x = 2.5 km across the view (the camera looks +X from the west one).
    split: Option<(CloudPreset, CloudPreset)>,
    /// `--per-ray`: the Medium/Low map lookup (3 per ray) instead of per sample.
    per_ray: bool,
}

fn parse_args() -> Args {
    let mut a = Args { preset: CloudPreset::Fair, hour: 12.0, view: "sky".into(), scale: 1.0, wind: 1.0, screenshot: None, frames: 150, fog: 0.0, split: None, per_ray: false };
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
            "--fog" => a.fog = value().parse().expect("--fog <visibility m>"),
            "--split" => {
                let v = value();
                let (w, e) = v.split_once(',').expect("--split <west genus>,<east genus>");
                a.split = Some((CloudPreset::parse(w).expect("west genus"), CloudPreset::parse(e).expect("east genus")));
            }
            "--per-ray" => a.per_ray = true,
            other => panic!("unknown flag {other}"),
        }
    }
    a
}

/// Height fog of `visibility` m at the ground plus banks at their default shape.
fn fog_settings(visibility: f32) -> FogSettings {
    let mut f = FogSettings::default();
    if visibility > 0.0 {
        f.height = FogLayer { density: fog::extinction_for_visibility(visibility), falloff: 100.0, base: 0.0 };
        f.banks.layer = FogLayer { density: 0.03, falloff: 12.0, base: 0.0 };
    }
    f
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
        .add_plugins(bevy::render::diagnostic::RenderDiagnosticsPlugin)
        .add_plugins(CloudsPlugin { settings: cloud_settings(&args), fog: fog_settings(args.fog), seed: 0x5EED_C10D })
        .insert_resource(args)
        .add_systems(Startup, setup)
        .add_systems(Update, (shadow_light, aim, keys, split_map, screenshot))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut glass: ResMut<Assets<FroxelFogMaterial>>,
    volume: Res<FroxelVolume>,
    args: Res<Args>,
) {
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
    if args.view == "glass" {
        let pane = meshes.add(Cuboid::new(0.5, 6.0, 4.0));
        let base = StandardMaterial { base_color: Color::srgba(0.85, 0.95, 1.0, 0.35), alpha_mode: AlphaMode::Blend, perceptual_roughness: 0.1, ..default() };
        let glass = glass.add(FroxelFogMaterial { base, extension: FroxelFogExt::new(&volume) });
        for d in [20.0, 60.0, 150.0, 300.0] {
            let side = 3.0 + d * 0.25;
            commands.spawn((Mesh3d(pane.clone()), MeshMaterial3d(glass.clone()), Transform::from_xyz(d, 3.0, side)));
            commands.spawn((Mesh3d(pane.clone()), MeshMaterial3d(stone.clone()), Transform::from_xyz(d, 3.0, -side)));
            commands.spawn((PointLight { color: Color::srgb(1.0, 0.7, 0.4), intensity: 40_000.0, range: 25.0, ..default() }, Transform::from_xyz(d - 3.0, 3.0, 0.0)));
        }
    }
    let transform = match args.view.as_str() {
        "shadow" => Transform::from_xyz(0.0, 200.0, 0.0).looking_to(Vec3::new(1.0, -0.7, 0.2), Vec3::Y),
        "up" => Transform::from_xyz(0.0, 2.0, 0.0).looking_to(Vec3::Y, Vec3::X),
        "glass" => Transform::from_xyz(0.0, 2.0, 0.0).looking_to(Vec3::new(1.0, 0.02, 0.0), Vec3::Y),
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
fn screenshot(
    mut commands: Commands,
    args: Res<Args>,
    diagnostics: Res<bevy::diagnostic::DiagnosticsStore>,
    mut frame: Local<u32>,
    shots: Query<(), With<Screenshot>>,
    mut exit: MessageWriter<AppExit>,
) {
    let Some(path) = &args.screenshot else { return };
    *frame += 1;
    if *frame == args.frames {
        log_gpu_timers(&diagnostics);
        commands.spawn(Screenshot::primary_window()).observe(save_to_disk(path.clone()));
    } else if *frame > args.frames + 5 && shots.is_empty() {
        exit.write(AppExit::Success);
    }
}

/// The plugin settings: the camera's (west) climate when split, as the app
/// does (the map gives every other place its own layer), the map lookup per quality.
fn cloud_settings(args: &Args) -> CloudSettings {
    let mut s = settings(args.split.map_or(args.preset, |p| p.0), args.scale, args.wind);
    s.quality.map_per_sample = !args.per_ray;
    s
}

/// `--split`: a two-climate world weather map (`WE4b`), recentered on the camera.
fn split_map(
    args: Res<Args>,
    cameras: Query<&GlobalTransform, With<CloudCamera>>,
    mut gpu_map: ResMut<CloudWeatherMap>,
    mut map: Local<Option<WeatherMap>>,
) {
    let Some((west, east)) = args.split else { return };
    let Ok(camera) = cameras.single() else { return };
    let map = map.get_or_insert_with(|| {
        // 32 × 32 squares of 500 m from (-8 km, -8 km); x < 2.5 km west, 400 m band.
        let cells: Vec<Option<ClimateIx>> = (0..32 * 32).map(|i| Some(ClimateIx(u32::from(i % 32 >= 21)))).collect();
        let raster = ClimateRaster::new(DVec2::splat(-8000.0), 500.0, 32, 32, &cells, 400.0).expect("climates");
        let mut m = WeatherMap::new(raster, MAP_SIZE, 64.0);
        m.set_cpu_texels(false);
        let row = |p: CloudPreset| {
            let w = settings(p, args.scale, args.wind).weather;
            let mut ch = [0.0; row::LEN];
            (ch[row::COVERAGE], ch[row::CLOUD_TYPE], ch[row::WETNESS], ch[row::STORM]) = (w.coverage, w.cloud_type, w.wetness, w.storm);
            (ch[row::BASE], ch[row::THICKNESS], ch[row::DENSITY], ch[row::EROSION]) = (w.base, w.thickness, w.density, w.erosion);
            ch[row::CIRRUS] = w.cirrus;
            ClimateRow { ch, look: None }
        };
        m.set_rows(&[row(west), row(east)]);
        m
    });
    let p = camera.translation();
    let update = map.update_window(DVec2::new(f64::from(p.x), f64::from(p.z)));
    gpu_map.sync(map, update);
}

/// GPU pass timers (`RenderDiagnosticsPlugin`), averaged, at the screenshot frame.
fn log_gpu_timers(diagnostics: &bevy::diagnostic::DiagnosticsStore) {
    for pass in ["weather_map", "clouds_shadow", "clouds_trace", "clouds_resolve", "clouds_composite"] {
        let path = bevy::diagnostic::DiagnosticPath::new(format!("render/{pass}/elapsed_gpu"));
        if let Some(ms) = diagnostics.get(&path).and_then(bevy::diagnostic::Diagnostic::average) {
            info!("gpu {pass}: {ms:.3} ms");
        }
    }
}
