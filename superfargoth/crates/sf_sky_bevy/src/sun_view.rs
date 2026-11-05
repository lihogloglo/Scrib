//! Extra depth views drawn by Bevy's own shadow pass (casters with their alpha
//! mask, GPU preprocessing, its pipelines): orthographic, depth-clamped views
//! of a shadow-casting directional light (`sf_precip_bevy`'s rain occlusion
//! map), or of a [`DepthViewAnchor`], which needs no light at all
//! (`sf_surface_bevy`'s top-down shelter and canopy maps, drawn at night too).
//!
//! Bevy 0.20 has no depth-only view API; its shadow pass is the stock
//! facility that draws mesh casters into a depth target, and it reads a view's
//! casters from the `RenderShadowMapVisibleEntities` table of the render
//! entity its `LightEntity` names (only that: `queue_shadows`,
//! `specialize_shadows`; `LightEntity::Directional` picks the orthographic,
//! unclipped-depth pipeline key). A light owns such a table; an anchor is a
//! main-world entity synced to the render world whose render entity gets the
//! same two public tables and nothing else, so Bevy's visibility collection
//! (`collect_visible_cpu_culled_entities`, `collect_gpu_culled_meshes`) and the
//! shadow queue treat it like a light with no cascades of its own. No private
//! state or ordering is touched.
//!
//! A host system fills a [`SunViewSource`] resource in the main world on a
//! redraw frame; [`add_sun_view`] mirrors it into the render world, adds the
//! request's casters as one more subview of the light's visible-entity tables
//! (before Bevy collects them), and spawns the view (a [`ShadowView`] whose
//! depth attachment is the host's image) after `prepare_lights`, appended to
//! the camera's [`ViewLightEntities`]. Between redraws no view exists.
//!
//! Invariant (`bevy_pbr`'s shadow queue panics with "Failed to get directional
//! light visible entities" otherwise): a view tagged
//! `LightEntity::Directional { light_entity }` may sit in a camera's
//! `ViewLightEntities` only if that light's render entity has
//! [`RenderShadowMapVisibleEntities`] with an entry for the view's retained
//! key. Bevy extracts that table only for a visible light with shadow maps on,
//! and the main-world request (made in `PreUpdate`) can be stale by render
//! time: the sky turns the sun's shadows off in `Update` (interiors, night).
//! [`view_allowed`] is checked at spawn, so such a request is dropped. An
//! anchor's tables exist from its first extraction on, whatever the sky does.

use core::any::TypeId;

use bevy_app::App;
use bevy_asset::AssetId;
use bevy_camera::primitives::Frustum;
use bevy_camera::visibility::RenderLayers;
use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemChangeTick;
use bevy_image::Image;
use bevy_math::{Mat4, UVec4};
use bevy_mesh::Mesh3d;
use bevy_pbr::{prepare_lights, ExtractedDirectionalLight, LightEntity, Shadow, ShadowView, ViewLightEntities};
use bevy_render::batching::gpu_preprocessing::{GpuPreprocessingMode, GpuPreprocessingSupport};
use bevy_render::render_asset::RenderAssets;
use bevy_render::render_phase::ViewBinnedRenderPhases;
use bevy_render::sync_world::{MainEntity, RenderEntity, SyncToRenderWorld};
use bevy_render::texture::{DepthStencilViewAttachment, DepthStencilViews, GpuImage};
use bevy_render::view::{
    collect_visible_cpu_culled_entities, ExtractedView, NoIndirectDrawing, RenderExtractedShadowMapVisibleEntities,
    RenderExtractedVisibleEntities, RenderExtractedVisibleEntitiesClass, RenderShadowMapVisibleEntities,
    RenderVisibleEntities, RetainedViewEntity,
};
use bevy_render::{Extract, ExtractSchedule, Render, RenderApp, RenderSystems};
use bevy_shape::{HalfSpace, ViewFrustum};
use bevy_transform::components::GlobalTransform;

/// Owns depth views that belong to no light (module docs). Spawn it with
/// [`depth_view_anchor`]; requests name it as their `light`.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct DepthViewAnchor;

/// A [`DepthViewAnchor`] bundle (synced to the render world).
pub fn depth_view_anchor() -> (DepthViewAnchor, SyncToRenderWorld) {
    (DepthViewAnchor, SyncToRenderWorld::default())
}

/// One redraw of an extra sun view (main-world entities).
#[derive(Clone, Debug)]
pub struct SunViewRequest {
    /// The directional light the view belongs to, or a [`DepthViewAnchor`].
    pub light: Entity,
    /// The camera whose shadow pass draws it.
    pub camera: Entity,
    /// Subview index among the light's subviews (cascades use 0..4).
    pub subview: u32,
    /// The `Depth32Float` target (cleared to 0, reverse Z).
    pub image: AssetId<Image>,
    /// Target edge in texels.
    pub size: u32,
    pub view_from_world: Mat4,
    pub clip_from_world: Mat4,
    /// Casters as `(render entity, main entity)`.
    pub casters: Vec<(Entity, MainEntity)>,
}

impl SunViewRequest {
    /// The view's key in the light's visible-entity tables and in the phases.
    pub fn retained(&self) -> RetainedViewEntity {
        RetainedViewEntity::new(MainEntity::from(self.light), Some(MainEntity::from(self.camera)), self.subview)
    }
}

/// A resource holding this frame's request (or none), in both worlds.
pub trait SunViewSource: Resource + Clone + Default {
    /// Shadow pass label.
    const PASS_NAME: &'static str;
    fn request(&self) -> Option<&SunViewRequest>;
}

/// Installs the render side for `S` (and `S` in both worlds).
pub fn add_sun_view<S: SunViewSource>(app: &mut App) {
    app.init_resource::<S>();
    let Some(render_app) = app.get_sub_app_mut(RenderApp) else { return };
    if !render_app.world().contains_resource::<AnchorsExtracted>() {
        render_app.init_resource::<AnchorsExtracted>().add_systems(ExtractSchedule, extract_anchors);
    }
    render_app.init_resource::<S>().add_systems(ExtractSchedule, extract::<S>).add_systems(
        Render,
        (
            inject_casters::<S>.in_set(RenderSystems::PrepareAssets).before(collect_visible_cpu_culled_entities),
            spawn_view::<S>.in_set(RenderSystems::CreateViews).after(prepare_lights),
        ),
    );
}

/// Whether a view keyed `key` may be handed to Bevy's shadow pass for a light
/// with these extracted state: shadow maps on and a visible-entities entry.
pub fn view_allowed(shadow_maps_enabled: bool, visible: Option<&RenderShadowMapVisibleEntities>, key: &RetainedViewEntity) -> bool {
    shadow_maps_enabled && visible.is_some_and(|v| v.subviews.contains_key(key))
}

fn extract<S: SunViewSource>(mut commands: Commands, source: Extract<Res<S>>) {
    commands.insert_resource(source.clone());
}

/// The anchor extraction is installed once, whatever the number of sources.
#[derive(Resource, Default)]
struct AnchorsExtracted;

/// Gives each anchor's render entity the marker and the two (empty) visibility
/// tables a light would have; the sources fill their subviews.
fn extract_anchors(mut commands: Commands, anchors: Extract<Query<&RenderEntity, With<DepthViewAnchor>>>) {
    for render in &anchors {
        if let Ok(mut e) = commands.get_entity(**render) {
            e.insert_if_new((
                DepthViewAnchor,
                RenderShadowMapVisibleEntities::default(),
                RenderExtractedShadowMapVisibleEntities::default(),
            ));
        }
    }
}

/// Whether a render entity may own the request's view: an anchor always, a
/// directional light while its shadow maps are on.
fn owner_enabled(light: Option<&ExtractedDirectionalLight>, anchor: bool) -> bool {
    anchor || light.is_some_and(|l| l.shadow_maps_enabled)
}

/// The request's visible list, fresh each redraw (every caster "added", so
/// Bevy specialises and bins them all for the new view). Lights without the
/// tables (shadows off, invisible) get nothing, and no view is spawned.
#[allow(clippy::type_complexity)]
fn inject_casters<S: SunViewSource>(
    source: Res<S>,
    ticks: SystemChangeTick,
    mut lights: Query<(
        &MainEntity,
        Option<&ExtractedDirectionalLight>,
        Has<DepthViewAnchor>,
        &mut RenderShadowMapVisibleEntities,
        &mut RenderExtractedShadowMapVisibleEntities,
    )>,
) {
    let Some(req) = source.request() else { return };
    let Some((_, _, _, mut visible, mut extracted)) =
        lights.iter_mut().find(|(main, light, anchor, _, _)| main.id() == req.light && owner_enabled(*light, *anchor))
    else {
        return;
    };
    let key = req.retained();
    visible.subviews.insert(key, (ticks.this_run(), RenderVisibleEntities::default()));
    let mut list = RenderExtractedVisibleEntities::default();
    list.classes.insert(TypeId::of::<Mesh3d>(), RenderExtractedVisibleEntitiesClass { entities: req.casters.clone() });
    extracted.subviews.insert(key, list);
}

/// Spawns this redraw's view (despawning the last one) if [`view_allowed`].
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn spawn_view<S: SunViewSource>(
    mut commands: Commands,
    source: Res<S>,
    images: Res<RenderAssets<GpuImage>>,
    lights: Query<
        (
            Entity,
            &MainEntity,
            Option<&ExtractedDirectionalLight>,
            Has<DepthViewAnchor>,
            Option<&RenderShadowMapVisibleEntities>,
            Option<&RenderLayers>,
        ),
        Or<(With<ExtractedDirectionalLight>, With<DepthViewAnchor>)>,
    >,
    mut views: Query<(&MainEntity, &mut ViewLightEntities, Has<NoIndirectDrawing>)>,
    mut phases: ResMut<ViewBinnedRenderPhases<Shadow>>,
    support: Res<GpuPreprocessingSupport>,
    mut spawned: Local<Option<Entity>>,
) {
    if let Some(old) = spawned.take() {
        commands.entity(old).despawn();
    }
    let Some(req) = source.request() else { return };
    let Some(gpu) = images.get(req.image) else { return };
    let key = req.retained();
    let Some((light, _, _, _, _, layers)) = lights
        .iter()
        .find(|(_, main, extracted, anchor, visible, _)| main.id() == req.light && view_allowed(owner_enabled(*extracted, *anchor), *visible, &key))
    else {
        return;
    };
    let Some((_, mut view_lights, no_indirect)) = views.iter_mut().find(|(m, _, _)| m.id() == req.camera) else { return };
    let mode = support.min(if no_indirect { GpuPreprocessingMode::PreprocessingOnly } else { GpuPreprocessingMode::Culling });
    let world_from_view = req.view_from_world.inverse();
    let mut frustum = ViewFrustum::from_clip_from_world(&req.clip_from_world);
    // Like Bevy's cascades: casters between the light and the near plane still cast.
    let near = frustum.half_spaces[ViewFrustum::NEAR_PLANE_IDX].normal();
    frustum.half_spaces[ViewFrustum::NEAR_PLANE_IDX] = HalfSpace::new(near.extend(f32::INFINITY));
    let depth_attachment =
        DepthStencilViewAttachment::new(DepthStencilViews::DepthOnly { depth_view: gpu.texture_view.clone() }, Some(0.0), None);
    let mut entity = commands.spawn((
        ShadowView { depth_attachment, pass_name: S::PASS_NAME.into() },
        ExtractedView {
            retained_view_entity: key,
            viewport: UVec4::new(0, 0, req.size, req.size),
            world_from_view: GlobalTransform::from(world_from_view),
            clip_from_view: req.clip_from_world * world_from_view,
            clip_from_world: Some(req.clip_from_world),
            target_format: bevy_pbr::CORE_3D_SHADOW_MAP_FORMAT,
            color_grading: bevy_render::view::ColorGrading::default(),
            invert_culling: false,
        },
        Frustum(frustum),
        LightEntity::Directional { light_entity: light, cascade_index: 0 },
        layers.cloned().unwrap_or_default(),
    ));
    if mode != GpuPreprocessingMode::Culling {
        entity.insert(NoIndirectDrawing);
    }
    view_lights.lights.push(entity.id());
    phases.prepare_for_new_frame(key, mode);
    *spawned = Some(entity.id());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(subview: u32) -> RetainedViewEntity {
        RetainedViewEntity::new(MainEntity::from(Entity::from_raw_u32(7).unwrap()), None, subview)
    }

    #[test]
    fn an_anchor_owns_views_without_a_light() {
        assert!(owner_enabled(None, true));
        assert!(!owner_enabled(None, false));
    }

    #[test]
    fn view_needs_shadows_and_its_entry() {
        let mut visible = RenderShadowMapVisibleEntities::default();
        // Interior case: the sun's shadows went off, Bevy extracted no table.
        assert!(!view_allowed(false, None, &key(0xFA)));
        assert!(!view_allowed(true, None, &key(0xFA)));
        // Table present (cascades only) but our subview not injected.
        visible.subviews.insert(key(0), Default::default());
        assert!(!view_allowed(true, Some(&visible), &key(0xFA)));
        visible.subviews.insert(key(0xFA), Default::default());
        assert!(view_allowed(true, Some(&visible), &key(0xFA)));
        assert!(!view_allowed(false, Some(&visible), &key(0xFA)));
    }
}
