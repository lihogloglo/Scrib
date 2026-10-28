//! Held items (character.md "Held items"): weapons, shields and torches as
//! rigid props on the skeleton's socket joints.
//!
//! Each [`HeldItem`] of a dressed character's recipe becomes one prop
//! entity, a child of its mount's socket joint with the mount's offset, so
//! it follows the animated joint through Bevy's transform propagation. The
//! prop's meshes are one `Mesh` + material per cooked submesh, uploaded once
//! per model ([`HeldModelCache`]): a hundred guards holding the same sword
//! share those handles, which Bevy's GPU preprocessing batches (and
//! multi-draws) like any repeated static mesh.
//!
//! **Why separate entities, not merged into the outfit.** The outfit is one
//! skinned mesh per material, cached by look; a held item changes far more
//! often (drawn/sheathed, dusk and dawn torches, loot, combat), so merging
//! it would multiply outfit cache entries (look × loadout) and re-assemble
//! on each change. A rigid prop on a joint costs one entity and no skinning;
//! identical props batch across characters. Physics, trails and lights hang
//! off the prop later (UE sockets, Unity attach points, Godot
//! `BoneAttachment3D` work the same way).
//!
//! **Readiness.** [`CharacterReadiness`] on the character root (default
//! sheathed) picks each item's mount ([`HeldItem::mount`]): toggling it moves
//! props between mounts or hides them, nothing is rebuilt.
//!
//! **Lights.** An item with a [`HeldLight`] gets a [`HeldLightSlot`] child
//! at the light's node ([`CharacterContent::node_position`], else the
//! model's bounds center). The host turns slots into lights (it owns the
//! calibration, animation and light budget).

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use bevy_asset::{AssetServer, Assets, Handle, RenderAssetUsages};
use bevy_camera::primitives::Aabb;
use bevy_camera::visibility::{NoAutoAabb, RenderLayers, Visibility};
use bevy_ecs::prelude::*;
use bevy_math::{Mat4, Vec3};
use bevy_mesh::skinning::{SkinnedMesh, SkinnedMeshInverseBindposes};
use bevy_mesh::{Mesh, Mesh3d};
use bevy_pbr::{MeshMaterial3d, StandardMaterial};
use bevy_transform::components::Transform;
use sf_anim_bevy::{AnimAsset, SkeletonBinding};
use sf_character::{HeldItem, HeldLight, Mount, Readiness};
use sf_model::{Cooked, CookedModel, CookedSkinned};
use sf_records::RecordId;
use sf_vfs::VfsPath;

use crate::{CharacterBody, CharacterContent, CharacterStats, Characters, OutfitCache};

/// Whether a character's weapon is out (on its root; default sheathed).
/// Changing it moves its held items to their other mounts.
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct CharacterReadiness(pub Readiness);

/// On a dressed character's root: its held items' props.
#[derive(Component, Clone, Debug, Default)]
pub struct HeldProps {
    /// The body the props hang on (a new body drops them with its rig).
    body: Option<Entity>,
    pub props: Vec<HeldProp>,
}

/// One held item's prop.
#[derive(Clone, Debug)]
pub struct HeldProp {
    pub item: HeldItem,
    /// The prop root (child of a socket joint; mesh and light children).
    pub entity: Entity,
    /// Its light slot, when the item carries a light.
    pub light: Option<Entity>,
    drawn: Option<(Entity, Transform)>,
    sheathed: Option<(Entity, Transform)>,
    parent: Entity,
}

impl HeldProp {
    /// The socket joint and transform the prop takes at a readiness
    /// (`None`: hidden, or the socket is missing).
    pub fn at(&self, readiness: Readiness) -> Option<(Entity, Transform)> {
        match readiness {
            Readiness::Drawn => self.drawn,
            Readiness::Sheathed => self.sheathed,
        }
    }

    /// The entity the prop is parented to now.
    pub fn parent(&self) -> Entity {
        self.parent
    }
}

/// Where a held item's light goes: the host adds the light (calibrated,
/// animated, budgeted) to this entity. Inherits the prop's visibility.
#[derive(Component, Clone, Debug, PartialEq)]
pub struct HeldLightSlot {
    pub item: RecordId,
    pub light: HeldLight,
}

/// A held model's GPU assets (one entry per model, shared by every prop).
#[derive(Debug)]
pub struct HeldModelAssets {
    pub meshes: Vec<(Handle<Mesh>, Handle<StandardMaterial>)>,
    /// A skinned model's skeleton (the meshes are then skinned to it).
    pub skin: Option<HeldSkinAssets>,
}

/// A skinned held model's own small skeleton: each prop gets one joint
/// entity per name, at its model-space rest transform, as children of the
/// prop root (so the skeleton follows the socket). Clips may later drive
/// the joints by name (a bow's string); until then the prop holds its rest
/// pose.
#[derive(Debug)]
pub struct HeldSkinAssets {
    pub inverse_bindposes: Handle<SkinnedMeshInverseBindposes>,
    /// Joint names and rest transforms (model space).
    pub joints: Vec<(String, Transform)>,
    /// `SkinnedMesh::joints` order: an index into [`Self::joints`] per
    /// palette entry (a name may appear twice with different binds).
    pub palette: Vec<usize>,
}

/// Held models by VFS path.
#[derive(Resource, Default)]
pub struct HeldModelCache {
    models: HashMap<VfsPath, Arc<HeldModelAssets>>,
}

impl HeldModelCache {
    pub fn len(&self) -> usize {
        self.models.len()
    }

    pub fn is_empty(&self) -> bool {
        self.models.is_empty()
    }
}

/// A held model's cooked data and light position, loaded on a build task.
#[derive(Debug)]
pub(crate) struct HeldCooked {
    geometry: HeldGeometry,
    /// Model-space light position (the light node, else the bounds center).
    light_at: Vec3,
}

#[derive(Debug)]
enum HeldGeometry {
    Rigid(CookedModel),
    /// With the model-space rest transform of each palette joint.
    Skinned(Box<CookedSkinned>, Vec<Mat4>),
}

impl HeldGeometry {
    fn model(&self) -> &CookedModel {
        match self {
            Self::Rigid(m) => m,
            Self::Skinned(s, _) => &s.model,
        }
    }
}

/// Loads a held item's model (on a build task): `None` when it is missing
/// or not cooked (logged).
pub(crate) fn load_held(content: &dyn CharacterContent, item: &HeldItem) -> Option<HeldCooked> {
    let geometry = match content.held_model(&item.model) {
        Ok(Cooked::Static(m)) => HeldGeometry::Rigid(m),
        Ok(Cooked::Skipped { reason }) => {
            log::warn!("held item {}: {} not cooked: {reason}", item.item, item.model.as_str());
            return None;
        }
        Ok(Cooked::Skinned(s)) => {
            let names: Vec<&str> = s.joints.iter().map(|j| j.name.as_str()).collect();
            let rest = content.node_transforms(&item.model, &names);
            // A joint the content can't place keeps its bind pose.
            let rest = s.joints.iter().zip(rest).map(|(j, r)| r.unwrap_or_else(|| Mat4::from_cols_array(&j.inverse_bind).inverse())).collect();
            log::debug!("held item {}: {} is a skinned prop ({} joints)", item.item, item.model.as_str(), s.joints.len());
            HeldGeometry::Skinned(Box::new(s), rest)
        }
        Err(e) => {
            log::warn!("held item {}: {e}", item.item);
            return None;
        }
    };
    let model = geometry.model();
    let center = (Vec3::from_array(model.bounds.min) + Vec3::from_array(model.bounds.max)) * 0.5;
    let node = item.light.as_ref().and_then(|l| l.attach_node.as_deref());
    let light_at = node.and_then(|n| content.node_position(&item.model, n)).unwrap_or(center);
    Some(HeldCooked { geometry, light_at })
}

/// A finished build's held items, waiting for [`sync_held`] (the body and
/// its rig exist by then).
#[derive(Component)]
pub(crate) struct HeldPending {
    pub(crate) items: Vec<HeldItem>,
    pub(crate) cooked: Vec<Option<Arc<HeldCooked>>>,
    pub(crate) animation: Arc<AnimAsset>,
}

fn upload(
    cooked: &HeldCooked,
    content: &dyn CharacterContent,
    outfits: &mut OutfitCache,
    server: &AssetServer,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    bindposes: &mut Assets<SkinnedMeshInverseBindposes>,
) -> HeldModelAssets {
    let model = cooked.geometry.model();
    let mut out = Vec::new();
    for (i, sub) in model.lods.first().map(|l| l.submeshes.as_slice()).unwrap_or_default().iter().enumerate() {
        let Some(material) = model.materials.get(sub.material as usize) else { continue };
        let mesh = match &cooked.geometry {
            HeldGeometry::Rigid(m) => sf_model_bevy::submesh_to_mesh(m, sub, RenderAssetUsages::RENDER_WORLD, true).map_err(|e| e.to_string()),
            HeldGeometry::Skinned(s, _) => {
                sf_model_bevy::skinned_submesh_to_mesh(s, 0, i, RenderAssetUsages::RENDER_WORLD, false).map(|(m, _)| m).map_err(|e| e.to_string())
            }
        };
        match mesh {
            Ok(mesh) => {
                let material =
                    outfits.materials.entry(format!("{material:?}")).or_insert_with(|| materials.add(content.material(material, server))).clone();
                out.push((meshes.add(mesh), material));
            }
            Err(e) => log::warn!("held model submesh: {e}"),
        }
    }
    let skin = match &cooked.geometry {
        HeldGeometry::Rigid(_) => None,
        HeldGeometry::Skinned(s, rest) => match sf_model_bevy::inverse_bindposes(s) {
            Ok(inverse) => {
                let (joints, palette) = skin_joints(s.joints.iter().map(|j| j.name.as_str()).zip(rest.iter().copied()));
                Some(HeldSkinAssets { inverse_bindposes: bindposes.add(inverse), joints, palette })
            }
            Err(e) => {
                log::warn!("held model skin: {e}");
                out.clear();
                None
            }
        },
    };
    HeldModelAssets { meshes: out, skin }
}

/// One joint per name (the first rest transform wins) and each palette
/// entry's index into them.
fn skin_joints<'a>(palette: impl Iterator<Item = (&'a str, Mat4)>) -> (Vec<(String, Transform)>, Vec<usize>) {
    let mut joints: Vec<(String, Transform)> = Vec::new();
    let order = palette
        .map(|(name, rest)| {
            joints.iter().position(|(n, _)| n == name).unwrap_or_else(|| {
                joints.push((name.to_owned(), Transform::from_matrix(rest)));
                joints.len() - 1
            })
        })
        .collect();
    (joints, order)
}

/// A mount's socket joint and transform (`None`: no mount, or the skeleton
/// lacks the socket: warned once per socket).
fn resolve(mount: Option<&Mount>, binding: &SkeletonBinding, animation: &AnimAsset, warned: &mut HashSet<String>) -> Option<(Entity, Transform)> {
    let mount = mount?;
    let Some(joint) = binding.find(&animation.skeleton, &mount.socket) else {
        if warned.insert(mount.socket.clone()) {
            log::warn!("held item socket {:?} is not in the skeleton; items mounted there are skipped", mount.socket);
        }
        return None;
    };
    Some((joint, Transform::from_matrix(Mat4::from(mount.offset))))
}

/// Spawns, keeps or despawns props so they match the finished build's held
/// items (an unchanged item keeps its prop).
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub(crate) fn sync_held(
    mut commands: Commands,
    characters: Option<Res<Characters>>,
    mut cache: ResMut<HeldModelCache>,
    mut outfits: ResMut<OutfitCache>,
    mut stats: ResMut<CharacterStats>,
    server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut bindposes: ResMut<Assets<SkinnedMeshInverseBindposes>>,
    mut roots: Query<(Entity, &HeldPending, &CharacterBody, Option<&mut HeldProps>, Option<&CharacterReadiness>, Option<&RenderLayers>)>,
    bindings: Query<&SkeletonBinding>,
    joints: Query<(Option<&ChildOf>, Has<Visibility>)>,
    mut warned: Local<HashSet<String>>,
) {
    let Some(characters) = characters else { return };
    let content = characters.content().as_ref();
    for (root, pending, body, props, readiness, layers) in &mut roots {
        commands.entity(root).try_remove::<HeldPending>();
        let Ok(binding) = bindings.get(body.body) else { continue };
        let readiness = readiness.map(|r| r.0).unwrap_or_default();
        let mut old = match &props {
            // Props on a replaced body went with its rig.
            Some(p) if p.body == Some(body.body) => p.props.clone(),
            Some(p) => {
                stats.held = stats.held.saturating_sub(p.props.len());
                Vec::new()
            }
            None => Vec::new(),
        };
        let mut next = Vec::with_capacity(pending.items.len());
        for (item, cooked) in pending.items.iter().zip(&pending.cooked) {
            if let Some(i) = old.iter().position(|p| p.item == *item) {
                next.push(old.swap_remove(i));
                continue;
            }
            let Some(cooked) = cooked else {
                stats.missing_held += 1;
                continue;
            };
            let drawn = resolve(item.drawn.as_ref(), binding, &pending.animation, &mut warned);
            let sheathed = resolve(item.sheathed.as_ref(), binding, &pending.animation, &mut warned);
            let Some(fallback) = drawn.or(sheathed) else {
                stats.missing_held += 1;
                continue;
            };
            let assets = Arc::clone(
                cache
                    .models
                    .entry(item.model.clone())
                    .or_insert_with(|| Arc::new(upload(cooked, content, &mut outfits, &server, &mut meshes, &mut materials, &mut bindposes))),
            );
            let shown = match readiness {
                Readiness::Drawn => drawn,
                Readiness::Sheathed => sheathed,
            };
            let ((parent, transform), visibility) = match shown {
                Some(at) => (at, Visibility::Inherited),
                None => (fallback, Visibility::Hidden),
            };
            for (joint, _) in [drawn, sheathed].into_iter().flatten() {
                visible_chain(&mut commands, &joints, joint, body.body);
            }
            let entity = commands.spawn((Name::new(format!("held {}", item.item)), transform, visibility, ChildOf(parent))).id();
            // A skinned prop's own skeleton: joints under the prop root.
            let skinned = assets.skin.as_ref().map(|skin| {
                let joints: Vec<Entity> = skin
                    .joints
                    .iter()
                    .map(|(name, rest)| commands.spawn((Name::new(name.clone()), *rest, ChildOf(entity))).id())
                    .collect();
                SkinnedMesh { inverse_bindposes: skin.inverse_bindposes.clone(), joints: skin.palette.iter().map(|&i| joints[i]).collect() }
            });
            for (mesh, material) in &assets.meshes {
                let mut e = commands.spawn((Mesh3d(mesh.clone()), MeshMaterial3d(material.clone()), Transform::IDENTITY, Visibility::Inherited, ChildOf(entity)));
                if let Some(skinned) = &skinned {
                    e.insert((
                        skinned.clone(),
                        Aabb::from_min_max(Vec3::splat(-1.5), Vec3::splat(1.5)),
                        NoAutoAabb,
                    ));
                }
                if let Some(layers) = layers {
                    e.insert(layers.clone());
                }
            }
            let light = item.light.as_ref().map(|light| {
                let mut e = commands.spawn((
                    HeldLightSlot { item: item.item.clone(), light: light.clone() },
                    Transform::from_translation(cooked.light_at),
                    Visibility::Inherited,
                    ChildOf(entity),
                ));
                if let Some(layers) = layers {
                    e.insert(layers.clone());
                }
                e.id()
            });
            stats.held += 1;
            next.push(HeldProp { item: item.clone(), entity, light, drawn, sheathed, parent });
        }
        for gone in old {
            commands.entity(gone.entity).try_despawn();
            stats.held = stats.held.saturating_sub(1);
        }
        let held = HeldProps { body: Some(body.body), props: next };
        match props {
            Some(mut p) => *p = held,
            None => {
                commands.entity(root).try_insert(held);
            }
        }
    }
}

/// Gives the joints from `joint` up to `body` visibility components, so a
/// prop under them inherits the character's visibility (rig joints carry
/// none: only the chains that hold props pay for it).
fn visible_chain(commands: &mut Commands, joints: &Query<(Option<&ChildOf>, Has<Visibility>)>, joint: Entity, body: Entity) {
    let mut at = joint;
    while at != body {
        let Ok((parent, has)) = joints.get(at) else { return };
        if !has {
            commands.entity(at).try_insert(Visibility::Inherited);
        }
        let Some(parent) = parent else { return };
        at = parent.parent();
    }
}

/// Moves props to their mount for the character's readiness (or hides them).
pub(crate) fn apply_readiness(mut commands: Commands, mut roots: Query<(&CharacterReadiness, &mut HeldProps), Changed<CharacterReadiness>>) {
    for (readiness, mut props) in &mut roots {
        for prop in &mut props.props {
            match prop.at(readiness.0) {
                Some((joint, transform)) => {
                    if prop.parent != joint {
                        commands.entity(prop.entity).try_insert(ChildOf(joint));
                        prop.parent = joint;
                    }
                    commands.entity(prop.entity).try_insert((transform, Visibility::Inherited));
                }
                None => {
                    commands.entity(prop.entity).try_insert(Visibility::Hidden);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skinned_prop_joints_are_one_per_name_and_the_palette_points_at_them() {
        let at = |x: f32| Mat4::from_translation(Vec3::X * x);
        let (joints, palette) = skin_joints([("Bow", at(0.0)), ("String", at(1.0)), ("Bow", at(5.0))].into_iter());
        assert_eq!(joints.iter().map(|(n, _)| n.as_str()).collect::<Vec<_>>(), ["Bow", "String"]);
        assert_eq!(palette, [0, 1, 0]);
        assert_eq!(joints[1].1.translation, Vec3::X, "rest transform kept");
        assert_eq!(joints[0].1.translation, Vec3::ZERO, "first rest wins");
    }
}
