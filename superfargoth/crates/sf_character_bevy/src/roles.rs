//! First-person roles (`docs/specs/first-person.md`): a character with [`RoleLayers`] on its root
//! is assembled with its outfit split by [`ViewRole`] (one mesh per material and role, the
//! roles from [`crate::CharacterContent::view_role`]), every part mesh carries its [`PartRole`],
//! and each part's `RenderLayers` follow the root's per-role rule: the root's own (worldspace)
//! layers, its world's shadow-only layers ([`ShadowLayers`]), both or neither, plus extra layers
//! (a view model's). Held item meshes are [`ViewRole::Held`]. Lights (held light slots, torches)
//! are never role parts: they keep the root's layers plus its shadow layers.
//!
//! Presentation only: the host (the player's camera view) writes [`RoleLayers`]; nothing here
//! feeds gameplay, and switching changes no mesh, only layers.
//!
//! Why layers, not `Visibility`: a hidden entity casts no shadow either, and the world body has
//! to keep casting (first-person.md "Layer plan"). Bevy 0.20 has no shadow-only flag, so a role
//! off the main view goes to its worldspace's shadow-only layer (FP-1b): its lights carry it, no
//! camera does.

use bevy_camera::visibility::RenderLayers;
use bevy_ecs::prelude::*;
use sf_character::ViewRole;

use crate::{Character, CharacterBody, HeldProps};

/// How one role's parts are layered.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RoleLayer {
    /// Drawn on the character root's own layers (its worldspace's: the main camera, its lights).
    pub world: bool,
    /// On the root's [`ShadowLayers`]: its world's lights cast it, no camera draws it.
    pub shadow: bool,
    /// Layers added on top (a view model camera's, FP-2).
    pub extra: RenderLayers,
}

impl RoleLayer {
    /// On the root's layers, nothing added.
    pub const WORLD: Self = Self { world: true, shadow: false, extra: RenderLayers::none() };
    /// Shadow only: cast by the world's lights, drawn by no camera (first person's head).
    pub const SHADOW: Self = Self { world: false, shadow: true, extra: RenderLayers::none() };
    /// On no layer: drawn by no view (no shadow either) until `extra` names one.
    pub const OFF: Self = Self { world: false, shadow: false, extra: RenderLayers::none() };
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

    /// First person, FP-1b (first-person.md "Layer plan"): the head off the main view but still
    /// casting (shadow only); body, arms and held items stay on the world layers, so the player
    /// sees their own arms and weapon until the view model (FP-2) draws them.
    pub fn first_person() -> Self {
        Self::world().with(ViewRole::Head, RoleLayer::SHADOW)
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

/// Where part layers are written (`PostUpdate`, after the worldspace layer push and before
/// `CheckVisibility`). A host that changes [`RoleLayers`] in `PostUpdate` runs before it.
#[derive(SystemSet, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CharacterLayerSystems;

type Roots<'a> =
    (Entity, Ref<'a, RoleLayers>, Option<Ref<'a, RenderLayers>>, Option<Ref<'a, ShadowLayers>>, Option<Ref<'a, CharacterBody>>, Option<Ref<'a, HeldProps>>);

/// Re-layers the role parts of every root whose rule, layers, outfit or held props changed.
pub(crate) fn sync_role_layers(
    mut commands: Commands,
    roots: Query<Roots<'_>, With<Character>>,
    children: Query<&Children>,
    parts: Query<(&PartRole, Option<&RenderLayers>)>,
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
        }
    }
}
