//! A procedural world baked through the terrain steps and rendered with
//! `TerrainPlugin`: fBm mountains, valleys and a coast, five procedural
//! layers, the sky plugin's sun and atmosphere.
//!
//! ```text
//! cargo run -p sf_terrain_bevy --example terrain --features bevy/dynamic_linking -- [options]
//!   --size-km N          world edge (default 16)
//!   --screenshot <png>   render, save, exit
//!   --camera x,y,z,tx,ty,tz   camera and target in meters; a y written as +h is h above the ground
//!   --lod --wire         debug views
//!   --hour H             time of day (default 10)
//!   --res WxH            window size (default 1920x1080)
//!   --flythrough         sf_diag scenario: 60 m/s path, report in ./diag
//!   --rebake             ignore the cache
//! ```
//!
//! Keys: WASD/QE fly (Shift ×5), right mouse look, wheel speed, L LOD
//! colors, F wireframe, P freeze selection, T +1 h.
#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::too_many_lines,
    clippy::many_single_char_names,
    clippy::struct_excessive_bools,
    clippy::too_many_arguments
)]

use std::path::PathBuf;
use std::time::Instant;

use bevy::camera::Exposure;
use bevy::core_pipeline::prepass::DepthPrepass;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll};
use bevy::prelude::*;
use bevy::render::diagnostic::RenderDiagnosticsPlugin;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use bevy::window::WindowResolution;
use glam::DVec3;
use sf_sky_bevy::sf_sky::{RenderPath, SkyClock, SkySettings};
use sf_sky_bevy::{SkyCamera, SkyConfig, SkyPlugin, SkyRenderOptions, SkyTime};
use sf_terrain_bevy::{TerrainCamera, TerrainDebug, TerrainPlugin, TerrainSettings, TerrainStats, TerrainWorld};

#[path = "../../sf_terrain/examples/synthetic/mod.rs"]
mod synthetic;
use synthetic::{bake, world_height};

// ---------------------------------------------------------------- app

#[derive(Resource, Clone)]
struct Args {
    size_km: f64,
    screenshot: Option<PathBuf>,
    camera: Option<[String; 6]>,
    lod: bool,
    wire: bool,
    hour: f64,
    res: (u32, u32),
    flythrough: bool,
    rebake: bool,
    no_prepass: bool,
    no_cull: bool,
    lod0_range: Option<f32>,
    min_lod: Option<i8>,
    splat_range: Option<f32>,
}

fn parse_args() -> Args {
    let mut a = Args {
        size_km: 16.0,
        screenshot: None,
        camera: None,
        lod: false,
        wire: false,
        hour: 10.0,
        res: (1920, 1080),
        flythrough: false,
        rebake: false,
        no_prepass: false,
        no_cull: false,
        lod0_range: None,
        min_lod: None,
        splat_range: None,
    };
    let mut it = std::env::args().skip(1);
    while let Some(arg) = it.next() {
        let mut value = || it.next().unwrap_or_else(|| panic!("{arg} needs a value"));
        match arg.as_str() {
            "--size-km" => a.size_km = value().parse().expect("--size-km <km>"),
            "--screenshot" => a.screenshot = Some(PathBuf::from(value())),
            "--camera" => {
                let v: Vec<String> = value().split(',').map(str::to_owned).collect();
                a.camera = Some(v.try_into().expect("--camera x,y,z,tx,ty,tz"));
            }
            "--lod" => a.lod = true,
            "--wire" => a.wire = true,
            "--hour" => a.hour = value().parse().expect("--hour <h>"),
            "--res" => {
                let v = value();
                let (w, h) = v.split_once('x').expect("--res WxH");
                a.res = (w.parse().unwrap(), h.parse().unwrap());
            }
            "--flythrough" => a.flythrough = true,
            "--rebake" => a.rebake = true,
            "--no-prepass" => a.no_prepass = true,
            "--no-cull" => a.no_cull = true,
            "--lod0-range" => a.lod0_range = Some(value().parse().expect("--lod0-range <m>")),
            "--min-lod" => a.min_lod = Some(value().parse().expect("--min-lod <lod>")),
            "--splat-range" => a.splat_range = Some(value().parse().expect("--splat-range <m>")),
            other => panic!("unknown argument {other}"),
        }
    }
    a
}

#[derive(Resource)]
struct WorldSize(f64);

/// A coordinate, or `+h`: h meters above the ground at (x, z).
fn resolve(v: &str, ground: f64) -> f64 {
    v.strip_prefix('+').map_or_else(|| v.parse().expect("number"), |h| ground + h.parse::<f64>().expect("number"))
}

fn main() {
    let args = parse_args();
    let (baked, size) = bake(args.size_km, args.rebake);
    let settings = SkySettings::default();
    let calendar = settings.calendar;
    let mut clock = SkyClock::at(172.0, args.hour, &calendar);
    clock.paused = true;

    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "sf_terrain".into(),
            resolution: WindowResolution::new(args.res.0, args.res.1).with_scale_factor_override(1.0),
            present_mode: bevy::window::PresentMode::AutoNoVsync,
            ..default()
        }),
        ..default()
    }))
    .add_plugins((FrameTimeDiagnosticsPlugin::default(), RenderDiagnosticsPlugin))
    .add_plugins(SkyPlugin { settings, clock, render_path: RenderPath::Raster })
    .insert_resource(SkyRenderOptions {
        cascades: bevy::light::CascadeShadowConfigBuilder {
            num_cascades: 4,
            minimum_distance: 0.1,
            first_cascade_far_bound: 30.0,
            maximum_distance: 3000.0,
            overlap_proportion: 0.2,
        }
        .build(),
        ..SkyRenderOptions::default()
    })
    .add_plugins(TerrainPlugin { settings: terrain_settings(&args) })
    .insert_resource(TerrainWorld { baked })
    .insert_resource(TerrainDebug {
        lod_colors: args.lod,
        wireframe: args.wire,
        freeze: false,
        no_culling: args.no_cull,
        no_far_shadows: false,
        no_ground_blend: false,
    })
    .insert_resource(WorldSize(size))
    .add_systems(Startup, setup)
    .add_systems(Update, (fly, keys, title, screenshot));

    if args.flythrough {
        let s = size;
        let h = |x: f64, z: f64| world_height(x, z, s);
        let keys: Vec<sf_diag_bevy::sf_diag::Keyframe> =
            [(0.25, 0.45), (0.33, 0.5), (0.4, 0.45), (0.45, 0.55), (0.38, 0.62)]
                .iter()
                .map(|&(u, v)| {
                    let (x, z) = (u * s, v * s);
                    sf_diag_bevy::sf_diag::Keyframe::at(DVec3::new(x, h(x, z).max(0.0) + 120.0, z))
                })
                .collect();
        let scenario = sf_diag_bevy::sf_diag::Scenario {
            name: "terrain_flythrough".into(),
            warmup: sf_diag_bevy::sf_diag::Warmup { seconds: 3.0, frames: 60 },
            measure: sf_diag_bevy::sf_diag::Measure::Path { keyframes: keys, speed: 60.0 },
            gates: Vec::new(),
            spike_factor: 1.5,
            tolerance: sf_diag_bevy::sf_diag::Tolerance::default(),
        };
        let plugin = sf_diag_bevy::SfDiagPlugin::new(sf_diag_bevy::DiagSettings {
            output_dir: PathBuf::from("diag"),
            exit_on_finish: true,
            ..default()
        })
        .with_scenario(scenario)
        .expect("scenario");
        app.add_plugins(plugin).add_systems(Update, terrain_counters);
    }
    app.insert_resource(args);
    app.run();
}

fn setup(mut commands: Commands, args: Res<Args>, size: Res<WorldSize>) {
    let s = size.0;
    let ground = |x: f64, z: f64| world_height(x, z, s).max(0.0);
    let (eye, target) = if let Some(c) = &args.camera {
        let (x, z, tx, tz) =
            (c[0].parse().unwrap(), c[2].parse().unwrap(), c[3].parse().unwrap(), c[5].parse().unwrap());
        let eye = DVec3::new(x, resolve(&c[1], ground(x, z)), z);
        let target = DVec3::new(tx, resolve(&c[4], ground(tx, tz)), tz);
        (eye, target)
    } else {
        let (x, z) = (s * 0.3, s * 0.35);
        (DVec3::new(x, ground(x, z) + 120.0, z), DVec3::new(s * 0.5, ground(s * 0.5, s * 0.5), s * 0.5))
    };
    let mut cam = commands.spawn((
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection {
            far: 80_000.0,
            near: 0.1,
            fov: 60f32.to_radians(),
            ..default()
        }),
        SkyCamera::default(),
        TerrainCamera,
        Transform::from_translation(eye.as_vec3()).looking_at(target.as_vec3(), Vec3::Y),
        Exposure::default(),
    ));
    if !args.no_prepass {
        cam.insert(DepthPrepass);
    }
    if args.flythrough {
        cam.insert(sf_diag_bevy::FlyThroughCamera);
    }
}

fn fly(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    buttons: Res<ButtonInput<MouseButton>>,
    motion: Res<AccumulatedMouseMotion>,
    scroll: Res<AccumulatedMouseScroll>,
    mut speed: Local<Option<f32>>,
    mut cam: Single<&mut Transform, With<TerrainCamera>>,
) {
    let speed = speed.get_or_insert(30.0);
    if scroll.delta.y != 0.0 {
        *speed = (*speed * 1.2f32.powf(scroll.delta.y)).clamp(1.0, 5000.0);
    }
    if buttons.pressed(MouseButton::Right) {
        let (mut yaw, mut pitch, _) = cam.rotation.to_euler(EulerRot::YXZ);
        yaw -= motion.delta.x * 0.003;
        pitch = (pitch - motion.delta.y * 0.003).clamp(-1.54, 1.54);
        cam.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
    }
    let mut dir = Vec3::ZERO;
    for (k, d) in [
        (KeyCode::KeyW, cam.forward().as_vec3()),
        (KeyCode::KeyS, -cam.forward().as_vec3()),
        (KeyCode::KeyD, cam.right().as_vec3()),
        (KeyCode::KeyA, -cam.right().as_vec3()),
        (KeyCode::KeyE, Vec3::Y),
        (KeyCode::KeyQ, -Vec3::Y),
    ] {
        if keys.pressed(k) {
            dir += d;
        }
    }
    let boost = if keys.pressed(KeyCode::ShiftLeft) { 5.0 } else { 1.0 };
    cam.translation += dir.normalize_or_zero() * *speed * boost * time.delta_secs();
}

fn keys(
    keys: Res<ButtonInput<KeyCode>>,
    mut debug: ResMut<TerrainDebug>,
    mut sky: ResMut<SkyTime>,
    config: Res<SkyConfig>,
) {
    if keys.just_pressed(KeyCode::KeyL) {
        debug.lod_colors = !debug.lod_colors;
    }
    if keys.just_pressed(KeyCode::KeyF) {
        debug.wireframe = !debug.wireframe;
    }
    if keys.just_pressed(KeyCode::KeyP) {
        debug.freeze = !debug.freeze;
    }
    if keys.just_pressed(KeyCode::KeyT) {
        let h = (f64::from(sky.0.hour(&config.0.calendar)) + 1.0) % 24.0;
        sky.0.set_hour(h, &config.0.calendar);
    }
}

/// GPU time of the passes that draw terrain (the scene holds nothing else).
fn gpu_ms(store: &DiagnosticsStore) -> f64 {
    store
        .iter()
        .filter(|d| {
            let p = d.path().as_str();
            p.ends_with("elapsed_gpu") && (p.contains("prepass") || p.contains("opaque") || p.contains("shadow"))
        })
        .filter_map(bevy::diagnostic::Diagnostic::smoothed)
        .sum()
}

fn title(
    stats: Res<TerrainStats>,
    store: Res<DiagnosticsStore>,
    mut window: Single<&mut Window>,
    cam: Single<&Transform, With<TerrainCamera>>,
    mut last: Local<f32>,
    time: Res<Time>,
) {
    *last += time.delta_secs();
    if *last < 0.25 {
        return;
    }
    *last = 0.0;
    let fps =
        store.get(&FrameTimeDiagnosticsPlugin::FPS).and_then(bevy::diagnostic::Diagnostic::smoothed).unwrap_or(0.0);
    let p = cam.translation;
    window.title = format!(
        "sf_terrain  {fps:.0} fps  terrain GPU {:.2} ms  patches {}  tiles {} (+{})  levels {}  select {:.2} ms  update {:.2} ms  up {} KiB  ({:.0}, {:.0}, {:.0})",
        gpu_ms(&store),
        stats.patches,
        stats.resident_tiles,
        stats.loads_in_flight,
        stats.height_levels,
        stats.select_ms,
        stats.update_ms,
        stats.upload_bytes / 1024,
        p.x,
        p.y,
        p.z
    );
}

fn terrain_counters(
    stats: Res<TerrainStats>,
    counters: Res<sf_diag_bevy::DiagCounters>,
    mut scenario: Option<ResMut<sf_diag_bevy::DiagScenario>>,
) {
    counters.add_named("terrain/select/ms", "ms", f64::from(stats.select_ms));
    counters.add_named("terrain/update/ms", "ms", f64::from(stats.update_ms));
    counters.add_named("terrain/upload/bytes", "bytes", stats.upload_bytes as f64);
    counters.set_named("terrain/patches/count", "count", stats.patches as f64);
    if let Some(s) = scenario.as_mut() {
        s.hold = stats.height_levels < 10 || stats.loads_in_flight > 0;
    }
}

/// `--screenshot`: wait until the terrain is resident, settle, save, exit.
fn screenshot(
    mut commands: Commands,
    args: Res<Args>,
    stats: Res<TerrainStats>,
    store: Res<DiagnosticsStore>,
    mut ready: Local<Option<(u32, Instant)>>,
    mut shot: Local<Option<u32>>,
    mut frames: Local<u32>,
    shots: Query<(), With<Screenshot>>,
    patches: Query<(&sf_terrain_bevy::TerrainPatch, &ViewVisibility)>,
    mut exit: MessageWriter<AppExit>,
) {
    let Some(path) = &args.screenshot else { return };
    *frames += 1;
    let settled = stats.height_levels == 10 && stats.loads_in_flight == 0 && stats.upload_bytes == 0;
    let Some((ready_frame, ready_at)) = *ready else {
        if settled || *frames > 4000 {
            *ready = Some((*frames, Instant::now()));
        }
        return;
    };
    // Let pipelines compile (they do so asynchronously) and exposure settle.
    if shot.is_none() && *frames >= ready_frame + 300 && ready_at.elapsed().as_secs_f32() > 5.0 {
        *shot = Some(*frames);
        println!(
            "screenshot {}: terrain GPU {:.2} ms, {} patches, {} tiles, select {:.2} ms",
            path.display(),
            gpu_ms(&store),
            stats.patches,
            stats.resident_tiles,
            stats.select_ms
        );
        let mut hist = std::collections::BTreeMap::<i8, (usize, usize)>::new();
        for (p, v) in &patches {
            let e = hist.entry(p.0.lod).or_default();
            e.0 += 1;
            e.1 += usize::from(v.get());
        }
        println!("  patches by LOD (total, visible): {hist:?}");
        for d in store.iter() {
            if d.path().as_str().ends_with("elapsed_gpu")
                && let Some(v) = d.smoothed()
            {
                println!("  {} {:.3} ms", d.path().as_str(), v);
            }
        }
        commands.spawn(Screenshot::primary_window()).observe(save_to_disk(path.clone()));
    } else if shot.is_some_and(|f| *frames > f + 6) && shots.is_empty() {
        exit.write(AppExit::Success);
    }
}

fn terrain_settings(args: &Args) -> TerrainSettings {
    let mut s = TerrainSettings::default();
    if let Some(r) = args.lod0_range {
        s.cdlod.lod0_range = r;
    }
    if let Some(l) = args.min_lod {
        s.cdlod.min_lod = l;
    }
    if let Some(r) = args.splat_range {
        s.splat_range = r;
    }
    s
}
