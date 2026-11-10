//! First-person roles (`docs/specs/first-person.md`): a character with [`RoleLayers`] on its root
//! is assembled with its outfit split by [`ViewRole`] (one mesh per material and role, the
//! roles from [`crate::CharacterContent::view_role`]), every part mesh carries its [`PartRole`],
//! and each part's `RenderLayers` follow the root's per-role rule: the root's own (worldspace)
//! layers, its world's shadow-only layers ([`ShadowLayers`]), both or neither, plus extra layers
//! a host adds. Held item meshes are [`ViewRole::Held`]. Lights (held light slots, torches)
//! are never role parts: they keep the root's layers plus its shadow layers.
//!
//! **View copies** (FP-2, first-person.md "View model"): a role whose rule asks for one
//! ([`RoleLayer::view_copy`]) gets a [`ViewCopy`] of each of its parts, a child of the part on
//! the root's layers, sharing its mesh, material and `SkinnedMesh` joints (no second rig, one more
//! joint palette), `NotShadowCaster` and out of the ray-traced scene (`NoRayTracing`). The camera
//! draws the copy (with [`crate::ViewModelPlugin`] through its view-model material, FP-2b); the part itself stays on the
//! shadow-only layer and casts. Copies are spawned and despawned with the rule, rebuilt with the
//! outfit and the held props (they are the parts' children), and follow their part's mesh,
//! material, skin, morph weights and bounds ([`follow_view_copies`]).
//!
//! Presentation only: the host (the player's camera view) writes [`RoleLayers`]; nothing here
//! feeds gameplay, and switching changes no outfit mesh, only layers and copies.
//!
//! Why layers, not `Visibility`: a hidden entity casts no shadow either, and the world body has
//! to keep casting (first-person.md "Layer plan"). Bevy 0.20 has no shadow-only flag, so a role
//! off the main view goes to its worldspace's shadow-only layer (FP-1b): its lights carry it, no
//! camera does.

use bevy_camera::primitives::Aabb;
use bevy_camera::visibility::{DynamicSkinnedMeshBounds, NoAutoAabb, NoFrustumCulling, RenderLayers, Visibility};
use bevy_ecs::prelude::*;
use bevy_light::NotShadowCaster;
use bevy_mesh::morph::MeshMorphWeights;
use bevy_mesh::skinning::SkinnedMesh;
use bevy_mesh::Mesh3d;
use bevy_pbr::{MeshMaterial3d, StandardMaterial};
use bevy_transform::components::Transform;
use sf_character::ViewRole;
use sf_model_bevy::NoRayTracing;

use crate::view_model::{ViewCopyMaterial, ViewModelMaterials};
use crate::{Character, CharacterBody, HeldProps};

/// How one role's parts are layered.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RoleLayer {
    /// Drawn on the character root's own layers (its worldspace's: the main camera, its lights).
    pub world: bool,
    /// On the root's [`ShadowLayers`]: its world's lights cast it, no camera draws it.
    pub shadow: bool,
    /// Layers added on top (a host's own view or light layers).
    pub extra: RenderLayers,
    /// Each part gets a [`ViewCopy`] on the root's layers, `NotShadowCaster` (FP-2: the copy is
    /// what the camera draws, with the view-model projection). Pair it with `world: false`, or
    /// the camera draws the part twice.
    pub view_copy: bool,
}

impl RoleLayer {
    /// On the root's layers, nothing added.
    pub const WORLD: Self = Self { world: true, shadow: false, extra: RenderLayers::none(), view_copy: false };
    /// Shadow only: cast by the world's lights, drawn by no camera (first person's head).
    pub const SHADOW: Self = Self { world: false, shadow: true, extra: RenderLayers::none(), view_copy: false };
    /// On no layer: drawn by no view (no shadow either) until `extra` names one.
    pub const OFF: Self = Self { world: false, shadow: false, extra: RenderLayers::none(), view_copy: false };
    /// The view model (FP-2): the part shadow only, a [`ViewCopy`] of it drawn on the root's layers.
    pub const VIEW_MODEL: Self = Self { world: false, shadow: true, extra: RenderLayers::none(), view_copy: true };
}

impl Default for RoleLayer {
    fn default() -> Self {
        Self::WORLD
    }
}

/// On a character root: assemble its outfit split by [`ViewRole`] and layer each part by its role.
/// Insert it with the [`Character`] (the split is chosen at build time); change it freely after
/// (a view switch only re-layers). The default draws every role on the root's layers, as a
/// character without it.
#[derive(Component, Clone, Debug, Default, PartialEq, Eq)]
pub struct RoleLayers {
    /// Indexed by [`ViewRole::index`].
    pub roles: [RoleLayer; ViewRole::COUNT],
}

impl RoleLayers {
    /// Every role on the root's layers (third person).
    pub fn world() -> Self {
        Self::default()
    }

    /// First person (first-person.md "Layer plan"): the head off the main view but still casting
    /// (shadow only), the body on the world layers. With `view_model` (FP-2) arms and held items
    /// are [`RoleLayer::VIEW_MODEL`]: shadow only, their [`ViewCopy`]s drawn; without (FP-1b) they
    /// stay on the world layers, drawn with the body.
    pub fn first_person(view_model: bool) -> Self {
        let fp = Self::world().with(ViewRole::Head, RoleLayer::SHADOW);
        if view_model { fp.with(ViewRole::Arms, RoleLayer::VIEW_MODEL).with(ViewRole::Held, RoleLayer::VIEW_MODEL) } else { fp }
    }

    #[must_use]
    pub fn with(mut self, role: ViewRole, layer: RoleLayer) -> Self {
        self.roles[role.index()] = layer;
        self
    }

    pub fn get(&self, role: ViewRole) -> &RoleLayer {
        &self.roles[role.index()]
    }

    /// The layers a part of `role` takes when its root is on `root` (the root's `RenderLayers`,
    /// layer 0 when it has none) with `shadow` its [`ShadowLayers`] (none when it has none).
    pub fn layers(&self, role: ViewRole, root: &RenderLayers, shadow: &RenderLayers) -> RenderLayers {
        let layer = self.get(role);
        let mut layers = layer.extra.clone();
        if layer.world {
            layers = layers.union(root);
        }
        if layer.shadow {
            layers = layers.union(shadow);
        }
        layers
    }

    /// The layers of a [`ViewCopy`] of a `role` part when its root is on `root`: the root's, if
    /// the role has copies.
    pub fn copy_layers(&self, role: ViewRole, root: &RenderLayers) -> Option<RenderLayers> {
        self.get(role).view_copy.then(|| root.clone())
    }
}

/// On a character root: the shadow-only layers of its world (`sf_world_bevy`
/// `WorldspaceLayers::shadow_only`), carried by that world's lights and by no camera. The
/// character's own lights (its [`crate::HeldLightSlot`]s and what the host hangs under them)
/// carry them besides the root's layers, and [`RoleLayer::shadow`] parts take them. Written by
/// `world::follow_worldspace_layers` (feature `world`) with the root's layers; without it a
/// character has no shadow-only layer and a shadow-only role casts nothing.
#[derive(Component, Clone, Debug, Default, PartialEq, Eq)]
pub struct ShadowLayers(pub RenderLayers);

impl ShadowLayers {
    /// The layers a light of a root on `root` carries: the root's plus these.
    pub fn light(shadow: Option<&Self>, root: &RenderLayers) -> RenderLayers {
        shadow.map_or_else(|| root.clone(), |s| root.union(&s.0))
    }
}

/// On a part mesh of a character with [`RoleLayers`]: its role (outfit meshes: their parts';
/// held item meshes: [`ViewRole::Held`]).
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PartRole(pub ViewRole);

/// On a part's view copy (FP-2, [`RoleLayer::view_copy`]): a child of the part it redraws.
/// Despawned with it; the copy's mesh, material, skin, morph weights and bounds follow it.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ViewCopy {
    pub source: Entity,
}

/// Where part layers are written (`PostUpdate`, after the worldspace layer push and before
/// `CheckVisibility`). A host that changes [`RoleLayers`] in `PostUpdate` runs before it.
#[derive(SystemSet, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CharacterLayerSystems;

type Roots<'a> =
    (Entity, Ref<'a, RoleLayers>, Option<Ref<'a, RenderLayers>>, Option<Ref<'a, ShadowLayers>>, Option<Ref<'a, CharacterBody>>, Option<Ref<'a, HeldProps>>);

/// What a view copy is made from (its part).
type Source<'a> = (
    &'a Mesh3d,
    Option<&'a MeshMaterial3d<StandardMaterial>>,
    Option<&'a SkinnedMesh>,
    Option<&'a MeshMorphWeights>,
    Option<&'a Aabb>,
    Has<NoAutoAabb>,
    Has<DynamicSkinnedMeshBounds>,
);

/// Re-layers the role parts of every root whose rule, layers, outfit or held props changed, and
/// spawns or despawns their view copies (re-layering the ones that stay).
pub(crate) fn sync_role_layers(
    mut commands: Commands,
    roots: Query<Roots<'_>, With<Character>>,
    children: Query<&Children>,
    parts: Query<(&PartRole, Option<&RenderLayers>)>,
    sources: Query<Source<'_>>,
    copies: Query<Option<&RenderLayers>, With<ViewCopy>>,
    view_model: Option<Res<ViewModelMaterials>>,
) {
    for (root, rule, layers, shadow, body, held) in &roots {
        let changed = rule.is_changed()
            || layers.as_ref().is_some_and(Ref::is_changed)
            || shadow.as_ref().is_some_and(Ref::is_changed)
            || body.as_ref().is_some_and(Ref::is_changed)
            || held.as_ref().is_some_and(Ref::is_changed);
        if !changed {
            continue;
        }
        let base = layers.as_deref().cloned().unwrap_or_default();
        let shadow = shadow.map_or_else(RenderLayers::none, |s| s.0.clone());
        for part in children.iter_descendants(root) {
            let Ok((role, current)) = parts.get(part) else { continue };
            let want = rule.layers(role.0, &base, &shadow);
            if current.unwrap_or(&RenderLayers::default()) != &want {
                commands.entity(part).try_insert(want);
            }
            let copy = children.get(part).ok().and_then(|c| c.iter().find(|&c| copies.contains(c)));
            match (copy, rule.copy_layers(role.0, &base)) {
                (None, Some(layers)) => {
                    if let Ok(source) = sources.get(part) {
                        spawn_copy(&mut commands, part, source, layers, view_model.is_some());
                    }
                }
                (Some(copy), Some(layers)) => {
                    if copies.get(copy).ok().flatten() != Some(&layers) {
                        commands.entity(copy).try_insert(layers);
                    }
                }
                (Some(copy), None) => commands.entity(copy).try_despawn(),
                (None, None) => {}
            }
        }
    }
}

/// A [`Source`] query item.
type SourceItem<'a> =
    (&'a Mesh3d, Option<&'a MeshMaterial3d<StandardMaterial>>, Option<&'a SkinnedMesh>, Option<&'a MeshMorphWeights>, Option<&'a Aabb>, bool, bool);

/// A view copy of `part` on `layers`: same mesh, material, joints, morph weights and bounds.
/// With `view_model` (`ViewModelPlugin`) the material is a [`ViewCopyMaterial`], turned into the
/// derived view-model material by `view_model::derive_view_model_materials`.
fn spawn_copy(commands: &mut Commands, part: Entity, source: SourceItem<'_>, layers: RenderLayers, view_model: bool) {
    let (mesh, material, skin, morph, aabb, no_auto_aabb, dynamic) = source;
    let mut e = commands.spawn((
        Name::new("view copy"),
        ViewCopy { source: part },
        mesh.clone(),
        Transform::IDENTITY,
        Visibility::Inherited,
        layers,
        // The part casts (on the shadow-only layer); the copy is only seen.
        NotShadowCaster,
        // Skinned meshes are not traced (rendering.md DL-3), and the copy's pose is a view's.
        NoRayTracing,
        // Drawn with another projection than the bounds are tested with (a `fov_body` wider than
        // the world's draws the copy inside a frustum its real bounds leave); a handful of meshes.
        NoFrustumCulling,
        ChildOf(part),
    ));
    if let Some(material) = material {
        if view_model {
            e.insert(ViewCopyMaterial(material.0.clone()));
        } else {
            e.insert(material.clone());
        }
    }
    if let Some(skin) = skin {
        // The part's joint entities: one rig poses both.
        e.insert(skin.clone());
    }
    if let Some(morph) = morph {
        e.insert(morph.clone());
    }
    if no_auto_aabb {
        e.insert(NoAutoAabb);
    }
    if dynamic {
        e.insert(DynamicSkinnedMeshBounds);
    } else if let Some(aabb) = aabb {
        e.insert(*aabb);
    }
}

type Followed<'a> = (
    Ref<'a, Mesh3d>,
    Option<Ref<'a, MeshMaterial3d<StandardMaterial>>>,
    Option<Ref<'a, SkinnedMesh>>,
    Option<Ref<'a, MeshMorphWeights>>,
    Option<Ref<'a, Aabb>>,
    Has<DynamicSkinnedMeshBounds>,
);

/// Keeps every [`ViewCopy`] on its part's mesh, material (a host's swap, e.g. a self-lit torch
/// variant), skin, morph weights and bounds (`PostUpdate`, after [`sync_role_layers`]). Copies
/// with dynamic skinned bounds compute their own, as their part does. With `ViewModelPlugin` a
/// material swap rewrites the copy's [`ViewCopyMaterial`].
pub(crate) fn follow_view_copies(
    mut commands: Commands,
    copies: Query<(Entity, &ViewCopy, Has<DynamicSkinnedMeshBounds>)>,
    parts: Query<Followed<'_>>,
    view_model: Option<Res<ViewModelMaterials>>,
) {
    for (copy, of, has_dynamic) in &copies {
        let Ok((mesh, material, skin, morph, aabb, dynamic)) = parts.get(of.source) else { continue };
        let changed = mesh.is_changed()
            || material.as_ref().is_some_and(Ref::is_changed)
            || skin.as_ref().is_some_and(Ref::is_changed)
            || morph.as_ref().is_some_and(Ref::is_changed)
            || (!dynamic && aabb.as_ref().is_some_and(Ref::is_changed))
            || dynamic != has_dynamic;
        if !changed {
            continue;
        }
        let mut e = commands.entity(copy);
        if mesh.is_changed() {
            e.try_insert(Mesh3d::clone(&mesh));
        }
        if let Some(material) = material.filter(Ref::is_changed) {
            if view_model.is_some() {
                e.try_insert(ViewCopyMaterial(material.0.clone()));
            } else {
                e.try_insert(MeshMaterial3d::clone(&material));
            }
        }
        if let Some(skin) = skin.filter(Ref::is_changed) {
            e.try_insert(SkinnedMesh::clone(&skin));
        }
        if let Some(morph) = morph.filter(Ref::is_changed) {
            e.try_insert(MeshMorphWeights::clone(&morph));
        }
        if dynamic && !has_dynamic {
            e.try_insert(DynamicSkinnedMeshBounds);
        } else if !dynamic {
            if has_dynamic {
                e.try_remove::<DynamicSkinnedMeshBounds>();
            }
            if let Some(aabb) = aabb.filter(|a| a.is_changed() || has_dynamic) {
                e.try_insert(*aabb);
            }
        }
    }
}
