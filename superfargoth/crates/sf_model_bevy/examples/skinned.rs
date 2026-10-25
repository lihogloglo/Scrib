//! Skinned cooked model vs. `bevy_gltf`, side by side, in bind pose.
//!
//! ```sh
//! cargo run -p sf_model_bevy --example skinned -- <character.glb> [--screenshot out.png]
//! ```
//!
//! The glTF is imported and cooked by `sf_model` (`Cooked::Skinned`, as the
//! bake does), written as `.sfmesh` to a temp folder and loaded back through
//! `SfMeshLoader`; its node tree becomes the skeleton (`spawn_rest_skeleton`
//! scaffold) and `spawn_skinned` binds the meshes by joint name. Left: the
//! cooked model. Right: the same file through `bevy_gltf`. Both should look
//! identical (pose, proportions, normals; materials are flat colors on the
//! left). Test assets: any CC0 Quaternius Universal-rig character (e.g.
//! `Mannequin_F.glb`), see docs/visual-checks.md.
//!
//! Keys: M cycles one morph target at weight 1 (if the model has any),
//! B toggles the cooked model's skeleton between rest and a 30° test bend of
//! every joint (shows the skin follows its joints; not an animation system).

use std::path::PathBuf;

use bevy::asset::AssetPlugin;
use bevy::gltf::GltfAssetLabel;
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use bevy::world_serialization::WorldAssetRoot;
use sf_model::{CookSettings, Cooked, GltfImporter, Model, ModelImporter, cook};
use sf_model_bevy::{
    CookedModelAsset, ModelMorphWeights, SfModelPlugin, resolve_joints, spawn_rest_skeleton,
    spawn_skinned,
};

#[derive(Resource)]
struct Example {
    model: Model,
    cooked: Handle<CookedModelAsset>,
    screenshot: Option<PathBuf>,
}

#[derive(Component)]
struct CookedRoot;

#[derive(Component)]
struct RestPose(Transform);

#[derive(Default, Resource)]
struct State {
    spawned: bool,
    frames: u32,
    morph: Option<usize>,
    bent: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let mut glb = None;
    let mut screenshot = None;
    while let Some(arg) = args.next() {
        if arg == "--screenshot" {
            screenshot = args.next().map(PathBuf::from);
        } else {
            glb = Some(PathBuf::from(arg));
        }
    }
    let glb = glb.ok_or("usage: skinned <character.glb> [--screenshot out.png]")?;
    let bytes = std::fs::read(&glb)?;
    let model = GltfImporter.import(&glb.to_string_lossy(), &bytes)?;
    let cooked = cook(&model, &CookSettings::default())?;
    let Cooked::Skinned(skinned) = &cooked else {
        return Err("the glTF didn't cook as a skinned model".into());
    };
    println!(
        "cooked: {} joints, {} morph targets, {} LODs, {} submeshes at LOD0",
        skinned.joints.len(),
        skinned.morph_targets.len(),
        skinned.model.lods.len(),
        skinned.model.lods[0].submeshes.len()
    );
    let dir = std::env::temp_dir().join("sf_model_bevy_skinned");
    std::fs::create_dir_all(&dir)?;
    std::fs::write(dir.join("model.sfmesh"), cooked.to_bytes())?;
    std::fs::write(dir.join("model.glb"), &bytes)?;

    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(AssetPlugin {
        file_path: dir.to_string_lossy().into_owned(),
        ..default()
    }))
    .add_plugins(SfModelPlugin)
    .init_resource::<State>()
    .add_systems(Startup, setup)
    .add_systems(Update, (spawn_cooked, keys, screenshot_when_ready));
    app.insert_resource(Example {
        model,
        cooked: Handle::default(),
        screenshot,
    });
    app.run();
    Ok(())
}

fn setup(mut commands: Commands, assets: Res<AssetServer>, mut example: ResMut<Example>) {
    example.cooked = assets.load("model.sfmesh");
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 1.2, 4.0).looking_at(Vec3::new(0.0, 0.9, 0.0), Vec3::Y),
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
        WorldAssetRoot(assets.load(GltfAssetLabel::Scene(0).from_asset("model.glb"))),
        Transform::from_xyz(1.0, 0.0, 0.0),
    ));
}

fn spawn_cooked(
    mut commands: Commands,
    example: Res<Example>,
    models: Res<Assets<CookedModelAsset>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut state: ResMut<State>,
) {
    if state.spawned {
        return;
    }
    let Some(asset) = models.get(&example.cooked) else {
        return;
    };
    state.spawned = true;
    let Some(skin) = &asset.skin else {
        error!("model.sfmesh is not skinned");
        return;
    };
    let root = commands
        .spawn((
            CookedRoot,
            Transform::from_xyz(-1.0, 0.0, 0.0),
            Visibility::default(),
        ))
        .id();
    let skeleton = spawn_rest_skeleton(&mut commands, root, &example.model);
    let joints = match resolve_joints(skin, |name| skeleton.get(name).copied()) {
        Ok(joints) => joints,
        Err(err) => {
            error!("{err}");
            return;
        }
    };
    let mats: Vec<Handle<StandardMaterial>> = asset
        .materials
        .iter()
        .map(|m| {
            let [r, g, b, a] = m.base_color;
            materials.add(StandardMaterial {
                base_color: Color::linear_rgba(r, g, b, a),
                double_sided: m.double_sided,
                ..default()
            })
        })
        .collect();
    let meshes = spawn_skinned(
        &mut commands,
        root,
        asset,
        0,
        &joints,
        |entity, material| {
            if let Some(handle) = mats.get(material as usize) {
                entity.insert(MeshMaterial3d(handle.clone()));
            }
        },
    );
    for &entity in skeleton.values() {
        commands
            .entity(entity)
            .insert(RestPose(Transform::IDENTITY));
    }
    info!(
        "spawned {} skinned meshes, {} joints, {} morph targets",
        meshes.len(),
        joints.len(),
        skin.morph_targets.len()
    );
}

fn keys(
    input: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<State>,
    mut weights: Query<&mut ModelMorphWeights>,
    mut joints: Query<(&mut Transform, &mut RestPose)>,
) {
    if input.just_pressed(KeyCode::KeyM) {
        for mut w in &mut weights {
            let n = w.0.len();
            if n == 0 {
                continue;
            }
            state.morph = match state.morph {
                None => Some(0),
                Some(i) if i + 1 < n => Some(i + 1),
                Some(_) => None,
            };
            w.0.iter_mut().for_each(|x| *x = 0.0);
            if let Some(i) = state.morph {
                w.0[i] = 1.0;
            }
            info!("morph target {:?} of {n} at weight 1", state.morph);
        }
    }
    if input.just_pressed(KeyCode::KeyB) {
        state.bent = !state.bent;
        for (mut t, mut rest) in &mut joints {
            if state.bent {
                rest.0 = *t;
                t.rotate_local_x(30f32.to_radians());
            } else {
                *t = rest.0;
            }
        }
        info!("test bend {}", if state.bent { "on" } else { "off" });
    }
}

fn screenshot_when_ready(
    mut commands: Commands,
    example: Res<Example>,
    mut state: ResMut<State>,
    mut exit: MessageWriter<AppExit>,
) {
    let Some(path) = &example.screenshot else {
        return;
    };
    if !state.spawned {
        return;
    }
    state.frames += 1;
    if state.frames == 120 {
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(path.clone()));
    } else if state.frames == 150 {
        exit.write(AppExit::Success);
    }
}
