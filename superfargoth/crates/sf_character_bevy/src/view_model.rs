//! The first-person view model's material (`docs/specs/first-person.md` "View model", FP-2b):
//! [`ViewModelMaterial`], an `ExtendedMaterial<StandardMaterial, ViewModel>` whose two vertex
//! stages (`shaders/view_model.wesl`, `shaders/view_model_prepass.wesl`: Bevy 0.20's
//! `render/mesh.wesl` and `prepass/prepass.wesl` vertex stages, copied) draw a mesh with its own
//! FOV and in the nearest depth slice, with the main camera, in the main pass. Lighting keeps
//! the real world position; the prepass outputs pseudo-world positions so motion vectors (TAA,
//! DLSS) measure the drawn motion. Math: `sf_camera::view_model` (tested there).
//!
//! With [`ViewModelPlugin`], every [`ViewCopy`] (module `roles`) draws its part's material
//! through a view-model material derived from it: one per part material handle (cached by
//! asset id), following handle swaps (`follow_view_copies` writes [`ViewCopyMaterial`], never a
//! `StandardMaterial` component on the copy, so other material swappers never see a copy) and
//! in-place edits of the source (`AssetEvent::Modified`). The derived base is forward-rendered
//! (a deferred G-buffer would relight the copy from its remapped depth). The projection's two
//! numbers come from [`ViewModelSettings`], which a host (the player camera's settings) writes;
//! every derived material follows a change. Without the plugin copies keep the stock material
//! (FP-2a: drawn exactly where the part is).
//!
//! Bindless: the extension is bindless like `StandardMaterial` (Bevy 0.20's
//! `extended_material_bindless` layout: data index 100, index table at binding 101, data array
//! at 102; non-bindless, a uniform at 100), so the copies batch with one bind group per slab.

use std::collections::HashMap;

use bevy_app::{App, Plugin, PostUpdate};
use bevy_asset::{Asset, AssetEvent, AssetId, Assets, Handle, embedded_asset};
use bevy_ecs::prelude::*;
use bevy_material::OpaqueRendererMethod;
use bevy_pbr::{
    ExtendedMaterial, MaterialExtension, MaterialPlugin, MeshMaterial3d, StandardMaterial,
};
use bevy_reflect::TypePath;
use bevy_render::render_resource::{AsBindGroup, ShaderType};
use bevy_shader::{Shader, ShaderRef};

use crate::roles::{CharacterLayerSystems, ViewCopy};

const MAIN: &str = "embedded://sf_character_bevy/shaders/view_model.wesl";
const PREPASS: &str = "embedded://sf_character_bevy/shaders/view_model_prepass.wesl";

/// The view model's projection: the view-space x/y scale `k` (`sf_camera::fov_scale`) and the
/// depth slice `R` (`sf_camera::ViewModelProjection`). A host writes it from its camera settings;
/// the default draws copies exactly over the world body (no FOV change, no slice).
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct ViewModelSettings {
    pub fov_scale: f32,
    pub depth: f32,
}

impl Default for ViewModelSettings {
    fn default() -> Self {
        Self {
            fov_scale: 1.0,
            depth: 0.0,
        }
    }
}

/// The view-model half of [`ViewModelMaterial`] (`view_model_common.wesl` `ViewModel`).
#[derive(Asset, TypePath, AsBindGroup, Clone, Copy, Debug, PartialEq)]
#[data(100, ViewModelUniform, binding_array(102))]
#[bindless(index_table(range(100..101), binding(101)))]
pub struct ViewModel {
    pub fov_scale: f32,
    pub depth: f32,
}

impl From<ViewModelSettings> for ViewModel {
    fn from(s: ViewModelSettings) -> Self {
        Self {
            fov_scale: s.fov_scale,
            depth: s.depth,
        }
    }
}

/// The GPU layout of [`ViewModel`] (WGSL `ViewModel`: two `f32`, in this order).
#[derive(Clone, Copy, Debug, Default, PartialEq, ShaderType)]
pub struct ViewModelUniform {
    pub fov_scale: f32,
    pub depth: f32,
}

impl From<&ViewModel> for ViewModelUniform {
    fn from(e: &ViewModel) -> Self {
        Self {
            fov_scale: e.fov_scale,
            depth: e.depth,
        }
    }
}

impl MaterialExtension for ViewModel {
    fn vertex_shader() -> ShaderRef {
        MAIN.into()
    }

    fn prepass_vertex_shader() -> ShaderRef {
        PREPASS.into()
    }

    /// Never used while the base is forward ([`derive_base`]); kept on the view model anyway.
    fn deferred_vertex_shader() -> ShaderRef {
        PREPASS.into()
    }
}

/// A view copy's material: its part's `StandardMaterial` drawn as the view model.
pub type ViewModelMaterial = ExtendedMaterial<StandardMaterial, ViewModel>;

/// On a [`ViewCopy`] while [`ViewModelPlugin`] runs: the part material its
/// `MeshMaterial3d<ViewModelMaterial>` is derived from (written by `roles` on spawn and on the
/// part's material swap, in place of the stock material component).
#[derive(Component, Clone, Debug, PartialEq, Eq)]
pub struct ViewCopyMaterial(pub Handle<StandardMaterial>);

/// The derived material per part material (asset id), forgotten when the source is removed.
#[derive(Resource, Default, Debug)]
pub struct ViewModelMaterials {
    by_source: HashMap<AssetId<StandardMaterial>, Handle<ViewModelMaterial>>,
}

impl ViewModelMaterials {
    /// The view-model material derived from `source`, if one exists.
    pub fn get(&self, source: AssetId<StandardMaterial>) -> Option<&Handle<ViewModelMaterial>> {
        self.by_source.get(&source)
    }

    /// Every derived material.
    pub fn by_source_handles(&self) -> impl Iterator<Item = &Handle<ViewModelMaterial>> {
        self.by_source.values()
    }

    pub fn len(&self) -> usize {
        self.by_source.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_source.is_empty()
    }
}

/// The view model's material, shaders and systems. Add it with a renderer (it adds
/// `MaterialPlugin::<ViewModelMaterial>`); headless, the material asset and the derivation still
/// run, the shaders are only registered.
pub struct ViewModelPlugin;

impl Plugin for ViewModelPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "shaders/view_model.wesl");
        embedded_asset!(app, "shaders/view_model_prepass.wesl");
        embedded_asset!(app, "shaders/view_model_common.wesl");
        // The import library must stay loaded (as `load_shader_library!`); only a renderer
        // registers the `Shader` asset.
        if app.world().contains_resource::<Assets<Shader>>() {
            std::mem::forget(
                bevy_asset::load_embedded_asset!(app, "shaders/view_model_common.wesl")
                    as Handle<Shader>,
            );
        }
        if !app.is_plugin_added::<MaterialPlugin<ViewModelMaterial>>() {
            app.add_plugins(MaterialPlugin::<ViewModelMaterial>::default());
        }
        app.init_resource::<ViewModelSettings>()
            .init_resource::<ViewModelMaterials>()
            .add_systems(
                PostUpdate,
                (follow_sources, derive_view_model_materials, follow_settings)
                    .chain()
                    .in_set(CharacterLayerSystems)
                    .after(crate::roles::follow_view_copies),
            );
    }
}

/// The derived base: the part's material, forward-rendered (see the module docs).
fn derive_base(source: &StandardMaterial) -> StandardMaterial {
    StandardMaterial {
        opaque_render_method: OpaqueRendererMethod::Forward,
        ..source.clone()
    }
}

/// Gives every copy whose [`ViewCopyMaterial`] changed (spawn, part material swap) the
/// view-model material derived from it, creating it on first use. A source not loaded yet is
/// retried each frame.
#[allow(clippy::too_many_arguments)] // a system's parameters
pub(crate) fn derive_view_model_materials(
    mut commands: Commands,
    changed: Query<Entity, (With<ViewCopy>, Changed<ViewCopyMaterial>)>,
    copies: Query<&ViewCopyMaterial, With<ViewCopy>>,
    standard: Res<Assets<StandardMaterial>>,
    mut materials: ResMut<Assets<ViewModelMaterial>>,
    mut derived: ResMut<ViewModelMaterials>,
    settings: Res<ViewModelSettings>,
    mut pending: Local<Vec<Entity>>,
) {
    let retry = std::mem::take(&mut *pending);
    for copy in changed.iter().chain(retry) {
        let Ok(ViewCopyMaterial(source)) = copies.get(copy) else {
            continue;
        };
        let handle = if let Some(h) = derived.by_source.get(&source.id()) {
            h.clone()
        } else {
            let Some(base) = standard.get(source) else {
                if !pending.contains(&copy) {
                    pending.push(copy);
                }
                continue;
            };
            let h = materials.add(ViewModelMaterial {
                base: derive_base(base),
                extension: ViewModel::from(*settings),
            });
            derived.by_source.insert(source.id(), h.clone());
            h
        };
        commands.entity(copy).try_insert(MeshMaterial3d(handle));
    }
}

/// Copies in-place edits of a source material into its derived one; forgets removed sources.
pub(crate) fn follow_sources(
    mut events: MessageReader<AssetEvent<StandardMaterial>>,
    standard: Res<Assets<StandardMaterial>>,
    mut materials: ResMut<Assets<ViewModelMaterial>>,
    mut derived: ResMut<ViewModelMaterials>,
) {
    for event in events.read() {
        match event {
            AssetEvent::Modified { id } => {
                if let (Some(handle), Some(source)) = (derived.by_source.get(id), standard.get(*id))
                    && let Some(mut m) = materials.get_mut(handle)
                {
                    m.base = derive_base(source);
                }
            }
            AssetEvent::Removed { id } => {
                derived.by_source.remove(id);
            }
            _ => {}
        }
    }
}

/// Writes a [`ViewModelSettings`] change into every derived material.
pub(crate) fn follow_settings(
    settings: Res<ViewModelSettings>,
    derived: Res<ViewModelMaterials>,
    mut materials: ResMut<Assets<ViewModelMaterial>>,
) {
    if !settings.is_changed() {
        return;
    }
    let want = ViewModel::from(*settings);
    for handle in derived.by_source.values() {
        if let Some(mut m) = materials.get_mut(handle)
            && m.extension != want
        {
            m.extension = want;
        }
    }
}
