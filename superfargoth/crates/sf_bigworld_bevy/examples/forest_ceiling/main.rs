//! Slot-path ceiling bench (BW-2): what does the vendored `GpuBatchedMesh3d` slot path cost for N instance slots
//! filled once on the CPU? A flat synthetic forest at 400 trees/ha, the N trees nearest the origin drawn as one
//! batch of full tree meshes or of upright cards, under a shadowed sun, from a fixed camera. The `stand` arm is the
//! renderer's cost-model worst case instead: a tree batch (4,096 slots) of every tree inside the mesh tier and a card
//! batch (250,000 slots) of every tree inside the impostor tier within the shadow range or in the frustum, active slots
//! first, the rest zeroed (`fill.rs`). One launch steps through a list of arms (`bench.rs`), measures each, prints
//! `bw2 ceiling |` lines and exits by itself.
//!
//! ```text
//! cargo run -p sf_bigworld_bevy --example forest_ceiling -- [options]
//!   --arms <a,b,..>      arms in order: empty, stand, tree:<n> or card:<n> (default
//!                        empty,card:100000,card:300000,tree:100000,tree:300000,stand,card:1000000,tree:1000000)
//!   --arm <spec>         one arm (same as --arms <spec>)
//!   --warm-s <s>         warm-up per arm, at least 30 frames (default 4)
//!   --measure-s <s>      measured time per arm, at least --min-frames frames (default 12)
//!   --min-frames <n>     fewest measured frames per arm (default 120)
//!   --max-arm-s <s>      a slow arm's measurement ends here, with at least 20 frames (default 60)
//!   --max-total-s <s>    watchdog: the whole run ends here (default 540)
//!   --eye <metres>       camera height (default 60)
//! ```
//!
//! No controls, no UI. Lines (stdout, `key=value`): one header (adapter, limits, settings), per arm `start`,
//! `result` and one `pass=` line per GPU span, `warn=limit` before a batch over a binding limit, and `done` last.
//! `start` and `result` give `slots=` (capacity) and `active=`; `start` adds `dropped=` when a tier overflowed.
#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]

mod bench;
mod fill;
mod meshes;
mod stats;
mod upload;

use bevy::camera::Hdr;
use bevy::core_pipeline::prepass::{DepthPrepass, MotionVectorPrepass};
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::light::{CascadeShadowConfigBuilder, DirectionalLightShadowMap, GlobalAmbientLight};
use bevy::pbr::gpu_instance_batch::{GpuBatchedMesh3d, GpuInstanceBatchPlugin};
use bevy::prelude::*;
use bevy::render::diagnostic::RenderDiagnosticsPlugin;
use bevy::render::occlusion_culling::OcclusionCulling;
use bevy::window::{PresentMode, WindowResolution};

use bench::{Bench, BenchConfig, advance, count_focus, parse_arms, print_header, sample_frame};
use fill::{Arm, MeshKind, StandView};
use sf_diag_bevy::{DiagCounters, FramePhasePlugin};
use upload::{CeilingUploadPlugin, CeilingUploads, CeilingWrite};

/// Fixture: window size in physical pixels (1440p over a 1.5 laptop scale).
const WINDOW_PX: (u32, u32) = (1707, 960);
/// Fixture: the arms run when `--arms` and `--arm` are absent (baseline first; the arms over a binding limit last).
const DEFAULT_ARMS: &str = "empty,card:100000,card:300000,tree:100000,tree:300000,stand,card:1000000,tree:1000000";
/// Fixture: default warm-up per arm, seconds.
const DEFAULT_WARM_S: f64 = 4.0;
/// Fixture: default measured time per arm, seconds.
const DEFAULT_MEASURE_S: f64 = 12.0;
/// Fixture: default cap on a slow arm's measurement, seconds.
const DEFAULT_MAX_ARM_S: f64 = 60.0;
/// Fixture: default fewest measured frames per arm.
const DEFAULT_MIN_FRAMES: u32 = 120;
/// Fixture: default watchdog on the whole run, seconds.
const DEFAULT_MAX_TOTAL_S: f64 = 540.0;
/// Fixture: camera height in metres when `--eye` is absent.
const DEFAULT_EYE_M: f32 = 60.0;
/// Fixture: camera pitch below the horizon, degrees.
const CAMERA_PITCH_DEG: f32 = 10.0;
/// Fixture: vertical field of view, degrees.
const FOV_Y_DEG: f32 = 60.0;
/// Fixture: far plane, metres.
const FAR_M: f32 = 10_000.0;
/// Fixture: ground plane edge, metres.
const GROUND_M: f32 = 8_000.0;
/// Fixture: sun elevation and azimuth, degrees.
const SUN_ELEVATION_DEG: f32 = 45.0;
const SUN_AZIMUTH_DEG: f32 = 30.0;
/// Fixture: sun illuminance, lux.
const SUN_LUX: f32 = 10_000.0;
/// Fixture: ambient brightness (cd/m², Bevy's unit), so shadowed sides are not black.
const AMBIENT_BRIGHTNESS: f32 = 600.0;
/// Fixture: directional shadow map edge, texels.
const SHADOW_MAP_PX: usize = 2048;

/// Command line.
struct Args {
    arms: Vec<String>,
    cfg: BenchConfig,
    max_total_s: f64,
    eye_m: f32,
}

/// A flag's value parsed as `T`.
fn number<T: std::str::FromStr>(flag: &str, v: &str) -> Result<T, String>
where
    T::Err: std::fmt::Display,
{
    v.parse().map_err(|e| format!("{flag} `{v}`: {e}"))
}

fn parse_args() -> Result<Args, String> {
    let mut arms = DEFAULT_ARMS.to_owned();
    let mut cfg = BenchConfig {
        warm_s: DEFAULT_WARM_S,
        measure_s: DEFAULT_MEASURE_S,
        max_arm_s: DEFAULT_MAX_ARM_S,
        min_frames: DEFAULT_MIN_FRAMES,
    };
    let (mut max_total_s, mut eye_m) = (DEFAULT_MAX_TOTAL_S, DEFAULT_EYE_M);
    let mut it = std::env::args().skip(1);
    while let Some(flag) = it.next() {
        let v = it.next().ok_or_else(|| format!("{flag} needs a value"))?;
        match flag.as_str() {
            "--arms" | "--arm" => arms = v,
            "--warm-s" => cfg.warm_s = number(&flag, &v)?,
            "--measure-s" => cfg.measure_s = number(&flag, &v)?,
            "--max-arm-s" => cfg.max_arm_s = number(&flag, &v)?,
            "--min-frames" => cfg.min_frames = number(&flag, &v)?,
            "--max-total-s" => max_total_s = number(&flag, &v)?,
            "--eye" => eye_m = number(&flag, &v)?,
            other => {
                return Err(format!(
                    "unknown argument `{other}` (--arms, --arm, --warm-s, --measure-s, --min-frames, --max-arm-s, \
                     --max-total-s, --eye)"
                ));
            }
        }
    }
    Ok(Args { arms: parse_arms(&arms)?, cfg, max_total_s, eye_m })
}

/// Marker on the bench's batch entities (despawned to switch arms).
#[derive(Component)]
struct CeilingBatch;

/// The bench's shared meshes and material, kept so arms can be respawned.
#[derive(Resource)]
struct CeilingAssets {
    tree: Handle<Mesh>,
    card: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

/// Camera height.
#[derive(Resource)]
struct Eye(f32);

/// The bench camera as the `stand` arm's frustum test sees it (the camera `setup_scene` spawns).
fn stand_view(eye: &Eye) -> StandView {
    StandView {
        eye_m: eye.0,
        pitch_rad: CAMERA_PITCH_DEG.to_radians(),
        fov_y_rad: FOV_Y_DEG.to_radians(),
        aspect: WINDOW_PX.0 as f32 / WINDOW_PX.1 as f32,
    }
}

fn main() {
    let args = match parse_args() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("forest_ceiling: {e}");
            std::process::exit(2);
        }
    };

    let eye = Eye(args.eye_m);
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "forest_ceiling".into(),
            resolution: WindowResolution::new(WINDOW_PX.0, WINDOW_PX.1).with_scale_factor_override(1.0),
            present_mode: PresentMode::AutoNoVsync,
            ..default()
        }),
        ..default()
    }))
    .add_plugins((FrameTimeDiagnosticsPlugin::default(), RenderDiagnosticsPlugin))
    // The phase counters need their registry in place first.
    .init_resource::<DiagCounters>()
    .add_plugins(FramePhasePlugin);
    if !app.is_plugin_added::<GpuInstanceBatchPlugin>() {
        app.add_plugins(GpuInstanceBatchPlugin);
    }
    app.add_plugins(CeilingUploadPlugin)
        .insert_resource(DirectionalLightShadowMap { size: SHADOW_MAP_PX })
        .insert_resource(GlobalAmbientLight { color: Color::WHITE, brightness: AMBIENT_BRIGHTNESS, ..default() })
        .insert_resource(Bench::new(args.cfg, args.max_total_s, args.arms, stand_view(&eye)))
        .insert_resource(eye)
        .add_systems(Startup, (setup_scene, print_header))
        .add_systems(Update, (sample_frame, count_focus, advance).chain())
        .run();
}

/// Camera, sun, ground and the shared bench assets.
fn setup_scene(
    mut commands: Commands,
    eye: Res<Eye>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    commands.spawn((
        Camera3d::default(),
        Hdr,
        Msaa::Off,
        DepthPrepass,
        MotionVectorPrepass,
        OcclusionCulling,
        Projection::Perspective(PerspectiveProjection {
            fov: FOV_Y_DEG.to_radians(),
            far: FAR_M,
            ..default()
        }),
        Transform::from_xyz(0.0, eye.0, 0.0).with_rotation(Quat::from_rotation_x(-CAMERA_PITCH_DEG.to_radians())),
    ));

    commands.spawn((
        DirectionalLight { illuminance: SUN_LUX, shadow_maps_enabled: true, ..default() },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::YXZ,
            SUN_AZIMUTH_DEG.to_radians(),
            -SUN_ELEVATION_DEG.to_radians(),
            0.0,
        )),
        CascadeShadowConfigBuilder {
            num_cascades: 4,
            minimum_distance: 0.1,
            first_cascade_far_bound: 30.0,
            maximum_distance: 500.0,
            overlap_proportion: 0.2,
        }
        .build(),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(GROUND_M, GROUND_M))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.28, 0.31, 0.22),
            perceptual_roughness: 1.0,
            ..default()
        })),
    ));

    let mask = images.add(meshes::leaf_mask_image());
    let material = materials.add(StandardMaterial {
        base_color_texture: Some(mask),
        alpha_mode: AlphaMode::Mask(0.5),
        double_sided: true,
        cull_mode: None,
        perceptual_roughness: 0.9,
        metallic: 0.0,
        ..default()
    });
    commands.insert_resource(CeilingAssets {
        tree: meshes.add(meshes::tree_mesh()),
        card: meshes.add(meshes::card_mesh()),
        material,
    });
}

/// Spawns an arm's batches and queues each batch's bytes as one run at slot 0.
fn spawn_arm(commands: &mut Commands, assets: &CeilingAssets, uploads: &mut CeilingUploads, arm: Arm) {
    for batch in arm.batches {
        let mesh = match batch.kind {
            MeshKind::Tree => assets.tree.clone(),
            MeshKind::Card => assets.card.clone(),
        };
        let entity = commands
            .spawn((
                GpuBatchedMesh3d { mesh, max_capacity: batch.capacity, ranges: Vec::new() },
                MeshMaterial3d(assets.material.clone()),
                CeilingBatch,
            ))
            .id();
        uploads.writes.push(CeilingWrite { entity, first_slot: 0, bytes: batch.bytes });
    }
}
