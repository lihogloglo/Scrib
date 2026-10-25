//! The wind field (weather.md "Wind"), synthetic test scene: arrows from
//! the CPU twin (`WindField::sample`, 2 m above ground) over a ground shaded
//! by the GPU field (`sf_wind_bevy::wind::wind_sample`, speed: calm dark blue,
//! mean green, twice the mean yellow). Gust cells should drift downwind,
//! form and fade; arrows never point upwind; arrows and colors agree.
//!
//! Keys: Space radial burst at the center (25 m/s, 60 m, 1.5 s), V vortex,
//! Up/Down mean speed ×1.5 / ÷1.5, G gustiness 0 / 0.3 / 0.6.
//!
//! ```sh
//! cargo run -p sf_wind_bevy --example wind [-- --screenshot out.png --frames 120 --burst 100]
//! ```

use std::path::PathBuf;

use bevy::asset::uuid_handle;
use bevy::mesh::MeshVertexBufferLayoutRef;
use bevy::pbr::{MaterialPipeline, MaterialPipelineKey, MATERIAL_BIND_GROUP_INDEX};
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, RenderPipelineDescriptor, SpecializedMeshPipelineError};
use bevy::render::storage::ShaderBuffer;
use bevy::render::view::screenshot::{save_to_disk, Screenshot};
use bevy::shader::ShaderRef;
use sf_wind_bevy::sf_wind::{Wind, WindSource};
use sf_wind_bevy::{wind_shader_defs, WindBuffer, WindField, WindPlugin, WindStats};

const GROUND_SHADER: Handle<Shader> = uuid_handle!("31472e0c-a56f-41db-8125-36630889dee9");
/// Arrow grid spacing and half extent, meters.
const SPACING: f32 = 15.0;
const HALF: i32 = 10;

#[derive(Resource, Clone, Debug)]
struct Args {
    screenshot: Option<PathBuf>,
    frames: u32,
    /// Frame to fire a burst at (screenshot runs).
    burst: Option<u32>,
}

fn parse_args() -> Args {
    let mut a = Args { screenshot: None, frames: 120, burst: None };
    let mut it = std::env::args().skip(1);
    while let Some(flag) = it.next() {
        let mut value = || it.next().expect("flag value");
        match flag.as_str() {
            "--screenshot" => a.screenshot = Some(PathBuf::from(value())),
            "--frames" => a.frames = value().parse().expect("--frames N"),
            "--burst" => a.burst = Some(value().parse().expect("--burst FRAME")),
            other => panic!("unknown flag {other}"),
        }
    }
    a
}

/// Unlit ground colored by the GPU wind field.
#[derive(Asset, TypePath, AsBindGroup, Clone, Debug)]
struct WindGround {
    #[storage(0, read_only)]
    wind: Handle<ShaderBuffer>,
}

impl Material for WindGround {
    fn fragment_shader() -> ShaderRef {
        GROUND_SHADER.into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        #[expect(clippy::cast_possible_truncation, reason = "group index 3")]
        let defs = wind_shader_defs(MATERIAL_BIND_GROUP_INDEX as u32, 0);
        if let Some(fragment) = descriptor.fragment.as_mut() {
            fragment.shader_defs.extend(defs);
        }
        Ok(())
    }
}

fn main() {
    let args = parse_args();
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window { title: "sf_wind: wind field".into(), resolution: (1280, 720).into(), ..default() }),
        ..default()
    }))
    .add_plugins(WindPlugin {
        wind: Wind { velocity: Vec2::new(6.0, 2.0), gustiness: 0.5, gust_length: 80.0 },
        ..default()
    })
    .add_plugins(MaterialPlugin::<WindGround>::default())
    .insert_resource(args)
    .add_systems(Startup, setup)
    .add_systems(Update, (keys, arrows, report, screenshot));
    app.world_mut()
        .resource_mut::<Assets<Shader>>()
        .insert(&GROUND_SHADER, Shader::from_wesl(include_str!("wind.wesl"), "embedded://wind_example/ground.wesl"))
        .expect("uuid handle");
    app.run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<WindGround>>, buffer: Res<WindBuffer>) {
    #[expect(clippy::cast_precision_loss, reason = "small")]
    let size = (2 * HALF + 1) as f32 * SPACING;
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(size, size))),
        MeshMaterial3d(materials.add(WindGround { wind: buffer.0.clone() })),
    ));
    commands.spawn((Camera3d::default(), Transform::from_xyz(0.0, 230.0, 200.0).looking_at(Vec3::new(0.0, 0.0, 15.0), Vec3::Y)));
}

/// A source at the center (the scene has no floating origin: world = relative).
fn fire(field: &mut WindField, vortex: bool) {
    let pos = sf_coords::WorldPos::default();
    let source = if vortex {
        WindSource::vortex(pos, Vec3::Y, 20.0, 70.0).with_inner_radius(10.0).with_lifetime(0.3, 3.0, 1.5)
    } else {
        WindSource::radial(pos, 25.0, 60.0).with_inner_radius(5.0).with_lifetime(0.05, 1.5, 1.0)
    };
    if field.state.add_source(source).is_none() {
        warn!("all wind sources in use");
    }
}

fn keys(input: Res<ButtonInput<KeyCode>>, mut field: ResMut<WindField>, args: Res<Args>, mut frame: Local<u32>) {
    *frame += 1;
    if input.just_pressed(KeyCode::Space) || args.burst == Some(*frame) {
        fire(&mut field, false);
    }
    if input.just_pressed(KeyCode::KeyV) {
        fire(&mut field, true);
    }
    let wind = &mut field.state.wind;
    if input.just_pressed(KeyCode::ArrowUp) {
        wind.velocity *= 1.5;
    }
    if input.just_pressed(KeyCode::ArrowDown) {
        wind.velocity /= 1.5;
    }
    if input.just_pressed(KeyCode::KeyG) {
        wind.gustiness = if wind.gustiness < 0.15 { 0.3 } else if wind.gustiness < 0.45 { 0.6 } else { 0.0 };
    }
}

/// CPU-twin arrows, 2 m above the ground, one second of travel long (capped
/// below the spacing).
fn arrows(field: Res<WindField>, mut gizmos: Gizmos) {
    let mean = field.state.wind.speed().max(0.1);
    for i in -HALF..=HALF {
        for j in -HALF..=HALF {
            #[expect(clippy::cast_precision_loss, reason = "small")]
            let p = Vec3::new(i as f32 * SPACING, 2.0, j as f32 * SPACING);
            let v = field.sample(p, 2.0);
            let len = v.length().min(SPACING * 0.9);
            let t = (v.length() / (2.0 * mean)).min(1.0);
            gizmos.arrow(p, p + v.normalize_or_zero() * len, Color::srgb(1.0, 1.0 - 0.6 * t, 1.0 - t));
        }
    }
}

fn report(stats: Res<WindStats>, time: Res<Time>, mut next: Local<f32>) {
    if time.elapsed_secs() > *next {
        *next = time.elapsed_secs() + 2.0;
        info!("wind upload: {:.4} ms (mean {:.4} ms)", stats.last_ms, stats.mean_ms);
    }
}

/// `--screenshot`: render `--frames` frames, save, exit.
fn screenshot(mut commands: Commands, args: Res<Args>, mut frame: Local<u32>, shots: Query<(), With<Screenshot>>, mut exit: MessageWriter<AppExit>) {
    let Some(path) = &args.screenshot else { return };
    *frame += 1;
    if *frame == args.frames {
        commands.spawn(Screenshot::primary_window()).observe(save_to_disk(path.clone()));
    } else if *frame > args.frames + 5 && shots.is_empty() {
        exit.write(AppExit::Success);
    }
}
