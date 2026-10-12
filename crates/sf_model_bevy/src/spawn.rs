//! Spawning a loaded skinned model: the seed of `sf_anim_bevy` /
//! `sf_character_bevy`, kept small on purpose (no animation runtime).
//!
//! The pattern is `bevy_gltf`'s: mesh entities carry `Mesh3d`,
//! `SkinnedMesh { inverse_bindposes, joints }` (joint entities in palette
//! order), `DynamicSkinnedMeshBounds` (Bevy refits the `Aabb` from per-joint
//! bind-space boxes every frame, so posed characters aren't culled from their
//! bind-pose box), and morph weights. Unlike glTF primitives our meshes carry
//! only their own targets (bake.md "Morph layout"), so each gets
//! `MeshMorphWeights::Value`, refreshed from the model-wide
//! [`ModelMorphWeights`] by [`sync_morph_weights`].

use std::collections::HashMap;

use bevy_camera::visibility::{DynamicSkinnedMeshBounds, Visibility};
use bevy_ecs::prelude::*;
use bevy_ecs::system::EntityCommands;
use bevy_mesh::Mesh3d;
use bevy_mesh::morph::MeshMorphWeights;
use bevy_mesh::skinning::SkinnedMesh;
use bevy_transform::components::Transform;
use sf_model::Model;

use crate::CookedModelAsset;
use crate::skinned::{CookedSkin, gather_weights};

/// Model-wide morph weights of a spawned skinned model, one per
/// [`CookedSkin::morph_targets`], on the entity passed to [`spawn_skinned`].
/// Write here (animation curves, a slider); [`sync_morph_weights`] pushes
/// the values to the meshes.
#[derive(Component, Clone, Debug, Default, PartialEq)]
pub struct ModelMorphWeights(pub Vec<f32>);

/// On each morphing mesh entity: the model root holding the
/// [`ModelMorphWeights`] and the mesh's target list (Bevy target `k` is
/// model target `targets[k]`).
#[derive(Component, Clone, Debug, PartialEq)]
pub struct MorphWeightSource {
    pub root: Entity,
    pub targets: Vec<u16>,
}

/// Palette joints with no entity: the skeleton doesn't fit the mesh.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[error("skeleton lacks joints {0:?}")]
pub struct MissingJoints(pub Vec<String>);

/// The palette's joint entities in `SkinnedMesh::joints` order, by name
/// (repeated names map to the same entity). Every missing name is reported.
pub fn resolve_joints(
    skin: &CookedSkin,
    lookup: impl Fn(&str) -> Option<Entity>,
) -> Result<Vec<Entity>, MissingJoints> {
    let mut missing = Vec::new();
    let joints = skin
        .joints
        .iter()
        .map(|name| {
            lookup(name).unwrap_or_else(|| {
                missing.push(name.clone());
                Entity::PLACEHOLDER
            })
        })
        .collect();
    if missing.is_empty() {
        Ok(joints)
    } else {
        Err(MissingJoints(missing))
    }
}

/// **Scaffold** (`rest skeleton`): spawns `model`'s node tree under `parent`
/// as named joint entities at their rest transforms and returns them by
/// name (the first node of a repeated name wins). This stands in for the rig
/// asset `sf_anim_bevy` will spawn (animation.md); removal: replace callers
/// with that skeleton and resolve joints against it the same way.
pub fn spawn_rest_skeleton(
    commands: &mut Commands,
    parent: Entity,
    model: &Model,
) -> HashMap<String, Entity> {
    let entities: Vec<Entity> = model
        .nodes
        .iter()
        .map(|node| {
            let t = node.local_transform();
            commands
                .spawn((
                    Name::new(node.name.clone()),
                    Transform::from_matrix(t.into()),
                ))
                .id()
        })
        .collect();
    let mut by_name = HashMap::with_capacity(entities.len());
    for (node, &entity) in model.nodes.iter().zip(&entities) {
        let up = node
            .parent
            .and_then(|p| entities.get(p as usize).copied())
            .unwrap_or(parent);
        commands.entity(entity).insert(ChildOf(up));
        by_name.entry(node.name.clone()).or_insert(entity);
    }
    by_name
}

/// Spawns LOD `level` of a skinned `asset` as children of `root`, bound to
/// `joints` (from [`resolve_joints`]). `material` inserts each mesh's
/// material (index into `asset.materials`). Inserts [`ModelMorphWeights`]
/// (default weights) on `root` when the model morphs. Returns the mesh
/// entities; empty for a static or skipped asset.
pub fn spawn_skinned(
    commands: &mut Commands,
    root: Entity,
    asset: &CookedModelAsset,
    level: usize,
    joints: &[Entity],
    mut material: impl FnMut(&mut EntityCommands, u32),
) -> Vec<Entity> {
    let (Some(skin), Some(lod)) = (&asset.skin, asset.lods.get(level)) else {
        return Vec::new();
    };
    let weights = skin.default_weights();
    let mut spawned = Vec::with_capacity(lod.parts.len());
    for &(mesh, material_index) in &lod.parts {
        let mut entity = commands.spawn((
            Mesh3d(asset.meshes[mesh].clone()),
            Transform::IDENTITY,
            Visibility::default(),
            SkinnedMesh {
                inverse_bindposes: skin.inverse_bindposes.clone(),
                joints: joints.to_vec(),
            },
            DynamicSkinnedMeshBounds,
            ChildOf(root),
        ));
        let targets = &skin.meshes[mesh].targets;
        if !targets.is_empty() {
            entity.insert((
                MeshMorphWeights::Value {
                    weights: gather_weights(targets, &weights),
                },
                MorphWeightSource {
                    root,
                    targets: targets.clone(),
                },
            ));
        }
        material(&mut entity, material_index);
        spawned.push(entity.id());
    }
    if !weights.is_empty() {
        commands.entity(root).insert(ModelMorphWeights(weights));
    }
    spawned
}

/// Copies changed [`ModelMorphWeights`] into the meshes'
/// `MeshMorphWeights::Value`. Registered by [`crate::SfModelPlugin`].
pub fn sync_morph_weights(
    roots: Query<&ModelMorphWeights, Changed<ModelMorphWeights>>,
    mut meshes: Query<(&MorphWeightSource, &mut MeshMorphWeights)>,
) {
    if roots.is_empty() {
        return;
    }
    for (source, mut mesh) in &mut meshes {
        if let Ok(weights) = roots.get(source.root) {
            *mesh = MeshMorphWeights::Value {
                weights: gather_weights(&source.targets, &weights.0),
            };
        }
    }
}
