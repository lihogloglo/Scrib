//! A glTF character driven through a 2D locomotion blend space by
//! `sf_anim_bevy`, with skeleton gizmos.
//!
//! ```sh
//! cargo run -p sf_anim_bevy --features debug --example blendspace -- <character.glb> [--screenshot out.png]
//! ```
//!
//! The file's skin is cooked by `sf_model` and loaded through
//! `SfMeshLoader` (as the bake does); its skeleton and clips are imported
//! and cooked by `sf_anim`, and a graph with one `BlendSpace2D` over the
//! locomotion clips it has is compiled by `sf_anim_graph`. The rig is
//! spawned by `spawn_rig` and the skin bound to it by joint name. Made for
//! the CC0 Quaternius Universal Animation Library (`UAL1.glb`: `Idle_Loop`,
//! `Walk_Loop`, `Jog_*_Loop`, `Sprint_Loop`); any glTF with some of those
//! names works.
//!
//! Keys: W/S forward/back, A/D strafe, Shift run (the `move` parameter
//! eases toward the input), G skeleton gizmos, 0 automatic LOD, 1-4 force
//! A0-A3, T cycles a crowd of 1 / 25 / 100 copies (LOD and counters).

use std::fmt::Write as _;
use std::path::PathBuf;
use std::sync::Arc;

use bevy::asset::AssetPlugin;
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use sf_anim::{
    AnimImporter, ClipRules, CookSettings, GltfAnimImporter, Looping, RootMotionMode,
    RootMotionRules, Skeleton, SkeletonRules, cook_clip,
};
use sf_anim_bevy::{
    AnimAsset, AnimGizmos, AnimLod, AnimStats, AnimTier, Animator, SfAnimPlugin, SkeletonBinding,
    spawn_rig,
};
use sf_anim_graph::{CompileSource, compile_str};
use sf_bake::ProductKey;
use sf_model::{CookSettings as MeshCookSettings, Cooked, GltfImporter, ModelImporter, cook};
use sf_model_bevy::{CookedModelAsset, SfModelPlugin, resolve_joints, spawn_skinned};

/// Blend-space samples: clip name and (strafe, forward) speed in m/s.
const SAMPLES: [(&str, [f32; 2]); 7] = [
    ("Idle_Loop", [0.0, 0.0]),
    ("Walk_Loop", [0.0, 1.5]),
    ("Jog_Fwd_Loop", [0.0, 3.5]),
    ("Sprint_Loop", [0.0, 6.0]),
    ("Jog_Bwd_Loop", [0.0, -3.0]),
    ("Jog_Left_Loop", [-3.0, 0.0]),
    ("Jog_Right_Loop", [3.0, 0.0]),
];

struct Clips(Vec<(String, sf_anim::Clip)>);

impl CompileSource for Clips {
    fn clip(&mut self, reference: &str) -> Result<(ProductKey, sf_anim::Clip), String> {
        let c = self
            .0
            .iter()
            .find(|(n, _)| n == reference)
            .ok_or_else(|| format!("no clip {reference}"))?;
        Ok((ProductKey::new("anim.cook", reference), c.1.clone()))
    }
    fn subgraph(&mut self, _: &str) -> Result<sf_anim_graph::asset::GraphAsset, String> {
        Err("no subgraphs".into())
    }
}

#[derive(Resource)]
struct Example {
    asset: Arc<AnimAsset>,
    model: Handle<CookedModelAsset>,
    screenshot: Option<PathBuf>,
    frames: u32,
    crowd: u16,
    input: Vec2,
}

#[derive(Component)]
struct Character {
    bound: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let (mut glb, mut screenshot) = (None, None);
    while let Some(arg) = args.next() {
        if arg == "--screenshot" {
            screenshot = args.next().map(PathBuf::from);
        } else {
            glb = Some(PathBuf::from(arg));
        }
    }
    let glb = glb.ok_or("usage: blendspace <character.glb> [--screenshot out.png]")?;
    let bytes = std::fs::read(&glb)?;
    let path = glb.to_string_lossy().into_owned();

    // Animation: skeleton + clips, cooked as the bake would.
    let source = GltfAnimImporter.import(&path, &bytes)?;
    let joints = source.skeleton.as_ref().ok_or("the glTF has no skin")?;
    let rules = SkeletonRules {
        root_motion_joint: source.root_motion.clone(),
        guess_tags: true,
        ..SkeletonRules::default()
    };
    let (skeleton, _) = Skeleton::with_rules(joints, &rules)?;
    let clip_rules = ClipRules {
        looping: Looping::Whole,
        root_motion: RootMotionRules {
            mode: RootMotionMode::Ignore,
            ..RootMotionRules::default()
        },
        ..ClipRules::default()
    };
    let mut clips = Vec::new();
    let mut samples = String::new();
    for (name, at) in SAMPLES {
        let Some(src) = source.clips.iter().find(|c| c.name == name) else {
            continue;
        };
        clips.push((
            name.to_owned(),
            cook_clip(&skeleton, src, &clip_rules, &CookSettings::default())?,
        ));
        let _ = write!(
            samples,
            "(clip: \"{name}\", at: ({:.1}, {:.1})), ",
            at[0], at[1]
        );
    }
    if clips.len() < 3 {
        return Err(format!("need at least 3 of {:?} in the file", SAMPLES.map(|s| s.0)).into());
    }
    let ron = format!(
        r#"(params: [(name: "move", kind: Vec2(0.0, 0.0))],
            nodes: {{"locomotion": BlendSpace2D(x: "move.x", y: "move.y", samples: [{samples}])}},
            layers: [(node: "locomotion")])"#
    );
    let graph = compile_str("locomotion", &ron, &skeleton, &mut Clips(clips.clone()))?;
    let bound: Vec<Arc<sf_anim::Clip>> = graph
        .clips
        .iter()
        .map(|slot| {
            Arc::new(
                clips
                    .iter()
                    .find(|(n, _)| *n == slot.product.subject)
                    .map(|c| c.1.clone())
                    .expect("compiled from these clips"),
            )
        })
        .collect();
    println!(
        "skeleton: {} joints; blend space over {:?}",
        skeleton.len(),
        clips.iter().map(|c| &c.0).collect::<Vec<_>>()
    );
    let asset = Arc::new(AnimAsset::new(Arc::new(skeleton), Arc::new(graph), bound)?);

    // Mesh: cooked skin, loaded through the asset loader.
    let model = GltfImporter.import(&path, &bytes)?;
    let cooked = cook(&model, &MeshCookSettings::default())?;
    if !matches!(cooked, Cooked::Skinned(_)) {
        return Err("the glTF didn't cook as a skinned model".into());
    }
    let dir = std::env::temp_dir().join("sf_anim_bevy_blendspace");
    std::fs::create_dir_all(&dir)?;
    std::fs::write(dir.join("model.sfmesh"), cooked.to_bytes())?;

    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            file_path: dir.to_string_lossy().into_owned(),
            ..default()
        }))
        .add_plugins((SfModelPlugin, SfAnimPlugin))
        .insert_resource(AnimGizmos { enabled: true })
        .insert_resource(Example {
            asset,
            model: Handle::default(),
            screenshot,
            frames: 0,
            crowd: 1,
            input: Vec2::ZERO,
        })
        .add_systems(Startup, (setup, gizmos_on_top))
        .add_systems(Update, (bind, keys, report, screenshot_when_ready))
        .run();
    Ok(())
}

fn gizmos_on_top(mut store: ResMut<GizmoConfigStore>) {
    store.config_mut::<DefaultGizmoConfigGroup>().0.depth_bias = -1.0;
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut example: ResMut<Example>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    example.model = assets.load("model.sfmesh");
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(1.6, 1.4, 3.2).looking_at(Vec3::new(0.0, 0.9, 0.0), Vec3::Y),
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: 8000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(2.0, 4.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.insert_resource(GlobalAmbientLight {
        brightness: 400.0,
        ..default()
    });
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(60.0, 60.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.32, 0.3))),
    ));
    spawn_character(&mut commands, &example.asset, Vec3::ZERO);
}

fn spawn_character(commands: &mut Commands, asset: &Arc<AnimAsset>, at: Vec3) {
    let root = commands
        .spawn((
            Character { bound: false },
            Transform::from_translation(at),
            Visibility::default(),
            Animator::new(asset.clone()),
            AnimLod::new(asset.rest_height()),
        ))
        .id();
    spawn_rig(commands, root, &asset.skeleton);
}

fn bind(
    mut commands: Commands,
    example: Res<Example>,
    models: Res<Assets<CookedModelAsset>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut chars: Query<(Entity, &mut Character, &SkeletonBinding)>,
) {
    let Some(asset) = models.get(&example.model) else {
        return;
    };
    let Some(skin) = &asset.skin else { return };
    let material = materials.add(Color::srgb(0.75, 0.7, 0.62));
    for (root, mut character, binding) in &mut chars {
        if character.bound {
            continue;
        }
        character.bound = true;
        match resolve_joints(skin, |name| binding.find(&example.asset.skeleton, name)) {
            Ok(joints) => {
                spawn_skinned(&mut commands, root, asset, 0, &joints, |e, _| {
                    e.insert(MeshMaterial3d(material.clone()));
                });
            }
            Err(e) => error!("{e}"),
        }
    }
}

fn keys(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut commands: Commands,
    mut example: ResMut<Example>,
    mut gizmos: ResMut<AnimGizmos>,
    mut chars: Query<(Entity, &mut Animator, &mut AnimLod)>,
) {
    let run = if input.pressed(KeyCode::ShiftLeft) {
        2.0
    } else {
        1.0
    };
    let axis = |pos: KeyCode, neg: KeyCode| {
        f32::from(u8::from(input.pressed(pos))) - f32::from(u8::from(input.pressed(neg)))
    };
    let target = Vec2::new(
        axis(KeyCode::KeyD, KeyCode::KeyA) * 3.0,
        axis(KeyCode::KeyW, KeyCode::KeyS)
            * if axis(KeyCode::KeyW, KeyCode::KeyS) > 0.0 {
                1.5 * run * run
            } else {
                3.0
            },
    );
    let t = 1.0 - (-6.0 * time.delta_secs()).exp();
    example.input = example.input.lerp(target, t);
    if input.just_pressed(KeyCode::KeyG) {
        gizmos.enabled = !gizmos.enabled;
    }
    let forced = [
        KeyCode::Digit0,
        KeyCode::Digit1,
        KeyCode::Digit2,
        KeyCode::Digit3,
        KeyCode::Digit4,
    ]
    .iter()
    .position(|k| input.just_pressed(*k))
    .map(|i| i.checked_sub(1).map(AnimTier::from_index));
    for (_, mut animator, mut lod) in &mut chars {
        animator.set_vec2("move", example.input);
        if let Some(f) = forced {
            lod.forced = f;
        }
    }
    if input.just_pressed(KeyCode::KeyT) {
        example.crowd = match example.crowd {
            1 => 25,
            25 => 100,
            _ => 1,
        };
        for (e, ..) in &chars {
            commands.entity(e).despawn();
        }
        let n = example.crowd;
        let side = (1..=n).find(|s| s * s >= n).unwrap_or(1);
        for i in 0..n {
            let at = if n == 1 {
                Vec3::ZERO
            } else {
                Vec3::new(
                    f32::from(i % side) * 1.5 - f32::from(side) * 0.75,
                    0.0,
                    -f32::from(i / side) * 2.0,
                )
            };
            spawn_character(&mut commands, &example.asset, at);
        }
        info!("crowd: {n}");
    }
}

fn report(stats: Res<AnimStats>, time: Res<Time>, mut last: Local<f32>, example: Res<Example>) {
    *last += time.delta_secs();
    if *last > 2.0 {
        *last = 0.0;
        info!(
            "move {:?}: tiers {:?}, evaluated {}, evaluate {:.0} µs wall / {:.0} µs cpu, apply {:.0} µs, bones written {}, events {}",
            example.input,
            stats.per_tier,
            stats.evaluated,
            stats.evaluate_us,
            stats.cpu_us,
            stats.apply_us,
            stats.bones_written,
            stats.events
        );
    }
}

fn screenshot_when_ready(
    mut commands: Commands,
    mut example: ResMut<Example>,
    mut exit: MessageWriter<AppExit>,
) {
    let Some(path) = example.screenshot.clone() else {
        return;
    };
    example.frames += 1;
    // Walk forward-right for the shot, so the blend is visible.
    example.input = Vec2::new(1.0, 1.5);
    if example.frames == 150 {
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(path));
    }
    if example.frames == 160 {
        exit.write(AppExit::Success);
    }
}
