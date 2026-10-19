//! Extra depth views of a shadow-casting directional light, drawn by Bevy's
//! own shadow pass: the far static sun map ([`crate::FarShadowPlugin`]) and
//! `sf_precip_bevy`'s rain occlusion map share this one mechanism.
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
//! [`view_allowed`] is checked at spawn, so such a request is dropped.

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
use bevy_render::sync_world::MainEntity;
use bevy_render::texture::{DepthStencilViewAttachment, DepthStencilViews, GpuImage};
use bevy_render::view::{
    collect_visible_cpu_culled_entities, ExtractedView, NoIndirectDrawing, RenderExtractedShadowMapVisibleEntities,
    RenderExtractedVisibleEntities, RenderExtractedVisibleEntitiesClass, RenderShadowMapVisibleEntities,
    RenderVisibleEntities, RetainedViewEntity,
};
use bevy_render::{Extract, ExtractSchedule, Render, RenderApp, RenderSystems};
use bevy_shape::{HalfSpace, ViewFrustum};
use bevy_transform::components::GlobalTransform;

/// One redraw of an extra sun view (main-world entities).
#[derive(Clone, Debug)]
pub struct SunViewRequest {
    /// The directional light the view belongs to.
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

/// The request's visible list, fresh each redraw (every caster "added", so
/// Bevy specialises and bins them all for the new view). Lights without the
/// tables (shadows off, invisible) get nothing, and no view is spawned.
fn inject_casters<S: SunViewSource>(
    source: Res<S>,
    ticks: SystemChangeTick,
    mut lights: Query<
        (&MainEntity, &ExtractedDirectionalLight, &mut RenderShadowMapVisibleEntities, &mut RenderExtractedShadowMapVisibleEntities),
    >,
) {
    let Some(req) = source.request() else { return };
    let Some((_, _, mut visible, mut extracted)) =
        lights.iter_mut().find(|(main, light, _, _)| main.id() == req.light && light.shadow_maps_enabled)
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
    lights: Query<(
        Entity,
        &MainEntity,
        &ExtractedDirectionalLight,
        Option<&RenderShadowMapVisibleEntities>,
        Option<&RenderLayers>,
    )>,
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
    let Some((light, _, _, _, layers)) = lights
        .iter()
        .find(|(_, main, extracted, visible, _)| main.id() == req.light && view_allowed(extracted.shadow_maps_enabled, *visible, &key))
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
