//! Synthetic ocean (water.md "Verification": `water_ocean`): a sea state from `--wind`, an island
//! shore with a sandy shelf, a low sun.
//!
//! ```sh
//! cargo run --release -p sf_water_bevy --example water_ocean -- [--wind 14] [--hour-angle 20]
//! cargo run --release -p sf_water_bevy --example water_ocean -- --golden   # GPU vs CPU twin, exits 0/1
//! cargo run --release -p sf_water_bevy --example water_ocean -- --screenshot out.png
//! ```
//!
//! `--golden` freezes time at 37.5 s, reads cascade 0's displacement back from the GPU and compares
//! it on the twin's 64² nodes with `sf_water::CascadeTwin` (same seed, cascades and quantised ω):
//! target 1 cm (water.md "W2 handoff" 3).

use bevy::camera::Hdr;
use bevy::light::CascadeShadowConfigBuilder;
use bevy::prelude::*;
use bevy::render::gpu_readback::{Readback, ReadbackComplete};
use bevy::render::view::screenshot::{save_to_disk, Screenshot};
use sf_water::{CascadeTwin, Cascades, SpectrumParams, WindInput, H0};
use sf_water_bevy::{f16_to_f32, OceanSpectrum, OceanSurface, WaterCamera, WaterFrame, WaterImages, WaterPlugin, RESOLUTION};

const GOLDEN_TIME: f64 = 37.5;
const SEED: u32 = 7;

#[derive(Resource, Clone)]
struct Args {
    wind: f32,
    golden: bool,
    no_water: bool,
    screenshot: Option<String>,
    sun_elevation: f32,
}

fn args() -> Args {
    let mut a = Args { wind: 9.0, golden: false, no_water: false, screenshot: None, sun_elevation: 25.0 };
    let v: Vec<String> = std::env::args().collect();
    let mut i = 1;
    while i < v.len() {
        match v[i].as_str() {
            "--wind" => {
                a.wind = v.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(a.wind);
                i += 1;
            }
            "--hour-angle" => {
                a.sun_elevation = v.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(a.sun_elevation);
                i += 1;
            }
            "--golden" => a.golden = true,
            "--no-water" => a.no_water = true,
            "--screenshot" => {
                a.screenshot = v.get(i + 1).cloned();
                i += 1;
            }
            _ => {}
        }
        i += 1;
    }
    a
}

fn spectrum(a: &Args) -> OceanSpectrum {
    OceanSpectrum {
        params: SpectrumParams { wind: WindInput { speed: a.wind, toward: Vec2::new(0.8, 0.6) }, ..SpectrumParams::default() },
        cascades: Cascades::default(),
        seed: SEED,
    }
}

fn main() {
    let a = args();
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window { title: "water_ocean".into(), resolution: (1600, 900).into(), ..default() }),
            ..default()
        }))
        .add_plugins((WaterPlugin, bevy::render::diagnostic::RenderDiagnosticsPlugin))
        .insert_resource(spectrum(&a))
        .insert_resource(a)
        .add_systems(Startup, setup)
        .add_systems(Update, (drive, finish))
        .run();
}

fn setup(mut commands: Commands, args: Res<Args>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.spawn((
        Camera3d::default(),
        Hdr,
        WaterCamera,
        Projection::Perspective(PerspectiveProjection { fov: 60f32.to_radians(), near: 0.1, far: 60_000.0, ..default() }),
        Transform::from_xyz(0.0, 4.0, 0.0).looking_at(Vec3::new(60.0, 0.0, 120.0), Vec3::Y),
        AmbientLight { color: Color::srgb(0.55, 0.7, 1.0), brightness: 3000.0, ..default() },
    ));
    let elevation = args.sun_elevation.to_radians();
    commands.spawn((
        DirectionalLight { illuminance: 80_000.0, shadow_maps_enabled: true, ..default() },
        Transform::from_rotation(Quat::from_euler(EulerRot::YXZ, 2.6, -elevation, 0.0)),
        CascadeShadowConfigBuilder { maximum_distance: 400.0, ..default() }.build(),
    ));
    let sand = materials.add(StandardMaterial { base_color: Color::srgb(0.76, 0.68, 0.5), perceptual_roughness: 0.9, ..default() });
    // A shelf sloping from the beach to 12 m deep, and an island.
    commands.spawn((Mesh3d(meshes.add(Cuboid::new(800.0, 1.0, 800.0))), MeshMaterial3d(sand.clone()), Transform::from_xyz(60.0, -6.0, 120.0).with_rotation(Quat::from_rotation_x(0.03))));
    commands.spawn((Mesh3d(meshes.add(Sphere::new(60.0).mesh().ico(5).unwrap())), MeshMaterial3d(sand), Transform::from_xyz(90.0, -48.0, 180.0)));
    commands.spawn((Mesh3d(meshes.add(Cuboid::new(1.0, 6.0, 1.0))), MeshMaterial3d(materials.add(Color::srgb(0.35, 0.25, 0.15))), Transform::from_xyz(20.0, -1.0, 40.0)));
    commands.spawn((OceanSurface, Transform::IDENTITY, Visibility::Visible));
}

fn drive(time: Res<Time>, args: Res<Args>, mut frame: ResMut<WaterFrame>) {
    let t = if args.golden { GOLDEN_TIME } else { time.elapsed_secs_f64() };
    *frame = WaterFrame { enabled: !args.no_water, time: t, origin_cell: IVec3::ZERO, level: 0.0 };
}

fn finish(mut commands: Commands, args: Res<Args>, images: Option<Res<WaterImages>>, spectrum: Res<OceanSpectrum>, mut frames: Local<u32>, mut started: Local<bool>, store: Res<bevy::diagnostic::DiagnosticsStore>) {
    *frames += 1;
    if args.golden && sf_water_bevy::fft_frames() >= 5 && !*started {
        *started = true;
        let Some(images) = images else { return };
        let s = spectrum.clone();
        commands.spawn(Readback::texture(images.displacement.clone())).observe(move |event: On<ReadbackComplete>, mut commands: Commands| {
            commands.entity(event.entity).despawn();
            std::process::exit(golden(&event.data, &s));
        });
    }
    if let Some(path) = args.screenshot.clone()
        && (sf_water_bevy::fft_frames() == 240 || (args.no_water && *frames == 240))
    {
        commands.spawn(Screenshot::primary_window()).observe(save_to_disk(path));
    }
    if sf_water_bevy::fft_frames() == 290 {
        timings(&store);
    }
    if args.screenshot.is_some() && (sf_water_bevy::fft_frames() >= 300 || *frames >= 2000) {
        std::process::exit(0);
    }
}

/// Cascade 0 (array layer 0) on the twin's nodes vs `CascadeTwin`. Returns the exit code.
#[allow(clippy::many_single_char_names)]
fn golden(data: &[u8], s: &OceanSpectrum) -> i32 {
    let twin_n = 64usize;
    let h0 = H0::generate(&s.params, &s.cascades, 0, twin_n, s.seed);
    let mut twin = CascadeTwin::new(&h0, &s.params);
    twin.evaluate(GOLDEN_TIME);
    let g = twin.grids();
    let step = RESOLUTION / twin_n;
    let texel = |x: usize, z: usize, ch: usize| {
        let at = ((z * RESOLUTION + x) * 4 + ch) * 2;
        f16_to_f32(u16::from_le_bytes([data[at], data[at + 1]]))
    };
    let (mut worst, mut rms, mut amp) = (0.0f32, 0.0f64, 0.0f32);
    for ix in 0..twin_n {
        for iz in 0..twin_n {
            let j = ix * twin_n + iz;
            let (x, z) = (ix * step, iz * step);
            let errs = [texel(x, z, 1) - g.height[j], texel(x, z, 0) - g.dx[j], texel(x, z, 2) - g.dz[j]];
            for e in errs {
                worst = worst.max(e.abs());
                rms += f64::from(e * e);
            }
            amp = amp.max(g.height[j].abs());
        }
    }
    #[allow(clippy::cast_precision_loss)]
    let rms = (rms / (3 * twin_n * twin_n) as f64).sqrt();
    println!("golden: cascade 0 ({} m), t = {GOLDEN_TIME} s, {}² nodes: max |GPU - twin| = {:.4} m, rms {:.4} m (max |h| {:.2} m)", s.cascades.size(0), twin_n, worst, rms, amp);
    if worst > 0.01 {
        golden_diagnose(data, g, twin_n);
    }
    i32::from(!(worst <= 0.01 && amp > 0.05))
}

/// Diagnostics when the golden fails: which convention the GPU field matches.
fn golden_diagnose(data: &[u8], g: &sf_water::twin::WaveGrids, twin_n: usize) {
    let step = RESOLUTION / twin_n;
    let texel = |x: usize, z: usize, ch: usize| {
        let at = ((z * RESOLUTION + x) * 4 + ch) * 2;
        f16_to_f32(u16::from_le_bytes([data[at], data[at + 1]]))
    };
    let mut gpu_max = 0.0f32;
    let mut e = [0.0f32; 4];
    for ix in 0..twin_n {
        for iz in 0..twin_n {
            let gh = texel(ix * step, iz * step, 1);
            gpu_max = gpu_max.max(gh.abs());
            let a = g.height[ix * twin_n + iz];
            let b = g.height[iz * twin_n + ix];
            for (k, v) in [gh - a, gh - b, gh + a, gh + b].iter().enumerate() {
                e[k] = e[k].max(v.abs());
            }
        }
    }
    println!("diagnose: gpu max |h| {gpu_max:.3}; max err direct {:.3}, transposed {:.3}, negated {:.3}, neg+transposed {:.3}", e[0], e[1], e[2], e[3]);
    println!("diagnose: gpu h at (0,0),(4,0),(0,4) = {:.3} {:.3} {:.3}; twin {:.3} {:.3} {:.3}", texel(0, 0, 1), texel(4, 0, 1), texel(0, 4, 1), g.height[0], g.height[twin_n], g.height[1]);
}

/// GPU pass timings (`render/<label>/elapsed_gpu`, smoothed): the water's own passes and the
/// transparent pass that holds the water draw (compare with `--no-water`).
fn timings(store: &bevy::diagnostic::DiagnosticsStore) {
    for d in store.iter() {
        let path = d.path().as_str();
        if path.ends_with("elapsed_gpu") && (path.contains("water") || path.contains("transparent") || path.contains("opaque")) {
            println!("timing: {path} = {:.3} ms", d.smoothed().unwrap_or(f64::NAN));
        }
    }
}
