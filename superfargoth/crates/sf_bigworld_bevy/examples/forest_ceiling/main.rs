//! Slot-path ceiling bench (BW-2): what does the vendored `GpuBatchedMesh3d` slot path cost for N instance slots
//! filled once on the CPU? One arm per launch: a flat synthetic forest at 400 trees/ha, the N trees nearest the
//! origin drawn as one batch of full tree meshes or of upright cards, under a shadowed sun, from a fixed camera.
//!
//! ```text
//! cargo run -p sf_bigworld_bevy --example forest_ceiling -- [options]
//!   --arm <spec>     tree:<n> or card:<n> (default tree:100000)
//!   --eye <metres>   camera height (default 60)
//! ```
//!
//! No controls, no UI. Prints `bw2 ceiling | arm=<name> slots=<total> batches=<count>` at startup.
#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]

mod fill;
mod meshes;
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

use fill::{Arm, MeshKind, parse_arm};
use upload::{CeilingUploadPlugin, CeilingUploads, CeilingWrite};

/// Fixture: window size in physical pixels (1440p over a 1.5 laptop scale).
const WINDOW_PX: (u32, u32) = (1707, 960);
/// Fixture: the arm run when `--arm` is absent.
const DEFAULT_ARM: &str = "tree:100000";
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
    arm: String,
    eye_m: f32,
}

fn parse_args() -> Result<Args, String> {
    let mut args = Args { arm: DEFAULT_ARM.to_owned(), eye_m: DEFAULT_EYE_M };
    let mut it = std::env::args().skip(1);
    while let Some(flag) = it.next() {
        let mut value = || it.next().ok_or_else(|| format!("{flag} needs a value"));
        match flag.as_str() {
            "--arm" => args.arm = value()?,
            "--eye" => {
                let v = value()?;
                args.eye_m = v.parse().map_err(|e| format!("--eye `{v}`: {e}"))?;
            }
            other => return Err(format!("unknown argument `{other}` (--arm <spec>, --eye <metres>)")),
        }
    }
    Ok(args)
}

/// Marker on the bench's batch entities (the next slice despawns them to switch arms).
#[derive(Component)]
struct CeilingBatch;

/// The arm on screen: its name and each batch's slot count.
#[derive(Resource, Debug)]
#[allow(dead_code)]
struct CurrentArm {
    name: String,
    slots: Vec<u32>,
}

/// The arm to spawn at startup (taken by [`spawn_arm`]).
#[derive(Resource)]
struct PendingArm(Option<Arm>);

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

fn main() {
    let args = match parse_args() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("forest_ceiling: {e}");
            std::process::exit(2);
        }
    };
    let arm = match parse_arm(&args.arm) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("forest_ceiling: {e}");
            std::process::exit(2);
        }
    };

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
    .add_plugins((FrameTimeDiagnosticsPlugin::default(), RenderDiagnosticsPlugin));
    if !app.is_plugin_added::<GpuInstanceBatchPlugin>() {
        app.add_plugins(GpuInstanceBatchPlugin);
    }
    app.add_plugins(CeilingUploadPlugin)
        .insert_resource(DirectionalLightShadowMap { size: SHADOW_MAP_PX })
        .insert_resource(GlobalAmbientLight { color: Color::WHITE, brightness: AMBIENT_BRIGHTNESS, ..default() })
        .insert_resource(PendingArm(Some(arm)))
        .insert_resource(Eye(args.eye_m))
        .add_systems(Startup, (setup_scene, spawn_arm).chain())
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

/// Spawns the pending arm's batches and queues each batch's bytes as one run at slot 0.
fn spawn_arm(
    mut commands: Commands,
    assets: Res<CeilingAssets>,
    mut pending: ResMut<PendingArm>,
    mut uploads: ResMut<CeilingUploads>,
) {
    let Some(arm) = pending.0.take() else { return };
    let total = arm.total_slots();
    let count = arm.batches.len();
    let mut slots = Vec::with_capacity(count);
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
        slots.push(batch.capacity);
    }
    println!("bw2 ceiling | arm={} slots={total} batches={count}", arm.name);
    commands.insert_resource(CurrentArm { name: arm.name, slots });
}
