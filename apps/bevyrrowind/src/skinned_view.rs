//! **Scaffold `skinned view`** (`view --model <nif> --skinned [--skeleton <nif>]`):
//! shows a `Cooked::Skinned` bake product (actor body part, creature, head)
//! in bind pose, bound by joint name to the skeleton of the NIF loaded
//! through the runtime path, with that NIF's own meshes hidden. Keys:
//! K flips between the cooked skinned meshes and the NIF meshes (A/B: they
//! should match), M cycles one morph target at weight 1.
//!
//! Removal: actor spawning (`sf_character_bevy` with a rig from
//! `sf_anim_bevy`) replaces the NIF skeleton and this module; the framework
//! side (`sf_model_bevy::spawn_skinned`) stays.

use std::collections::{HashMap, HashSet};

use bevy::prelude::*;
use bevy::world_serialization::{WorldAsset, WorldAssetRoot};
use mw_assets::SharedVfs;
use sf_bake::{Manifest, Store};
use sf_model::steps::{MeshCookStep, ModelInfoStep};
use sf_model_bevy::{CookedModelAsset, ModelMorphWeights, resolve_joints, spawn_skinned};

pub struct SkinnedViewPlugin;

impl Plugin for SkinnedViewPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MorphCycle>().add_systems(Update, (bind, keys));
    }
}

/// The given model paths whose bake product may be a cooked skinned model:
/// skinned, or animated (morph-only models such as heads cook as skinned too;
/// the rest cook as skipped and are reported when they load).
pub fn skinned_models<'a>(store: &Store, manifest: &Manifest, models: impl IntoIterator<Item = &'a str>) -> HashSet<String> {
    models
        .into_iter()
        .filter(|path| manifest.get(MeshCookStep::ID, path).is_some())
        .filter(|path| {
            let info = manifest.get(ModelInfoStep::ID, path).and_then(|d| store.get(&d).ok());
            info.and_then(|bytes| ModelInfoStep::decode(&bytes).ok()).is_some_and(|i| i.skinned || i.animated)
        })
        .map(str::to_owned)
        .collect()
}

#[derive(Component)]
struct SkinnedViewRoot {
    model: Handle<CookedModelAsset>,
    skeleton: Handle<WorldAsset>,
    /// The skeleton scene's root entity (a child of this root).
    scene: Entity,
}

/// Marks the cooked skinned meshes and the hidden NIF meshes (`K`).
#[derive(Component)]
struct CookedSkinnedMesh;
#[derive(Component)]
struct NifMesh;

#[derive(Resource, Default)]
struct MorphCycle {
    show_nif: bool,
    morph: Option<usize>,
}

/// Spawns the root (with `bundle`: transform, visibility, name), the
/// skeleton NIF scene under it, and starts loading the cooked product.
pub fn spawn(commands: &mut Commands, assets: &AssetServer, model: &str, skeleton: Option<&str>, bundle: impl Bundle) -> Entity {
    let skeleton: Handle<WorldAsset> = assets.load(skeleton.unwrap_or(model).to_owned());
    let model = mw_assets::cooked::load(assets, model, None);
    let root = commands.spawn(bundle).id();
    let scene = commands.spawn((WorldAssetRoot(skeleton.clone()), Transform::IDENTITY, Visibility::Inherited, ChildOf(root))).id();
    commands.entity(root).insert(SkinnedViewRoot { model, skeleton, scene });
    root
}

/// Once the cooked model and the skeleton scene are in: resolve joints by
/// name among the scene's descendants, spawn the skinned meshes, hide the NIF's.
#[allow(clippy::too_many_arguments)]
fn bind(
    mut commands: Commands,
    roots: Query<(Entity, &SkinnedViewRoot)>,
    server: Res<AssetServer>,
    models: Res<Assets<CookedModelAsset>>,
    children: Query<&Children>,
    names: Query<&Name>,
    meshes: Query<(), With<Mesh3d>>,
    vfs: Res<SharedVfs>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (root, view) in &roots {
        let Some(asset) = models.get(&view.model) else { continue };
        if !server.is_loaded_with_dependencies(&view.skeleton) || children.get(view.scene).is_err() {
            continue;
        }
        commands.entity(root).remove::<SkinnedViewRoot>();
        let Some(skin) = &asset.skin else {
            warn!("{}: the cooked product is not skinned ({:?})", names.get(root).map_or("?", Name::as_str), asset.skipped);
            continue;
        };
        let mut by_name = HashMap::new();
        for entity in children.iter_descendants(view.scene) {
            if let Ok(name) = names.get(entity) {
                by_name.entry(name.as_str().to_owned()).or_insert(entity);
            }
            if meshes.contains(entity) {
                commands.entity(entity).insert((NifMesh, Visibility::Hidden));
            }
        }
        let joints = match resolve_joints(skin, |name| by_name.get(name).copied()) {
            Ok(joints) => joints,
            Err(err) => {
                warn!("skinned view: {err} (body parts need --skeleton meshes/base_anim.nif)");
                continue;
            }
        };
        let mats: Vec<Handle<StandardMaterial>> =
            asset.materials.iter().map(|m| materials.add(mw_assets::cooked::cooked_material(m, &vfs.0, &server))).collect();
        let spawned = spawn_skinned(&mut commands, root, asset, 0, &joints, |entity, material| {
            entity.insert(CookedSkinnedMesh);
            if let Some(handle) = mats.get(material as usize) {
                entity.insert(MeshMaterial3d(handle.clone()));
            }
        });
        let targets: Vec<&str> = skin.morph_targets.iter().map(|t| t.name.as_str()).collect();
        let b = &asset.bounds;
        info!(
            "skinned view: {} meshes, {} joints, morph targets {targets:?}, bind bounds {:?}..{:?}",
            spawned.len(),
            joints.len(),
            b.min,
            b.max
        );
    }
}

fn keys(
    input: Res<ButtonInput<KeyCode>>,
    mut cycle: ResMut<MorphCycle>,
    mut cooked: Query<&mut Visibility, (With<CookedSkinnedMesh>, Without<NifMesh>)>,
    mut nif: Query<&mut Visibility, (With<NifMesh>, Without<CookedSkinnedMesh>)>,
    mut weights: Query<&mut ModelMorphWeights>,
) {
    if input.just_pressed(KeyCode::KeyK) {
        cycle.show_nif = !cycle.show_nif;
        let (shown, hidden) = if cycle.show_nif { (Visibility::Inherited, Visibility::Hidden) } else { (Visibility::Hidden, Visibility::Inherited) };
        nif.iter_mut().for_each(|mut v| *v = shown);
        cooked.iter_mut().for_each(|mut v| *v = hidden);
        info!("skinned view: showing {}", if cycle.show_nif { "the NIF runtime meshes" } else { "the cooked skinned meshes" });
    }
    if input.just_pressed(KeyCode::KeyM) {
        let n = weights.iter().map(|w| w.0.len()).max().unwrap_or(0);
        cycle.morph = match cycle.morph {
            None if n > 0 => Some(0),
            Some(i) if i + 1 < n => Some(i + 1),
            _ => None,
        };
        for mut w in &mut weights {
            w.0.iter_mut().for_each(|x| *x = 0.0);
            if let Some(slot) = cycle.morph.and_then(|i| w.0.get_mut(i)) {
                *slot = 1.0;
            }
        }
        info!("skinned view: morph target {:?} of {n} at weight 1 (others 0)", cycle.morph);
    }
}
