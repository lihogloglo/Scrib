//! superfargoth patch (own, CW-4: lamp shadow cache), ROADMAP 1.25 CW-4 slice S2.
//! Spec: `superfargoth/docs/specs/lamp-shadow-cache.md` ("Design" 3, "S2 as built").
//!
//! The mechanism that lets a host keep a point light's static shadow depth across frames and
//! draw only what changed. It separates "this lamp samples a shadow cube" from "these cube
//! faces are drawn this frame":
//!
//! - A lamp carrying [`CachedPointShadowMap`] (main world) is shaded as a shadowed point light
//!   (persistent cube slot, GPU light record, shadow fade) while its own
//!   `PointLight::shadow_maps_enabled` stays `false`, so Bevy's main-world caster scan
//!   (`bevy_light::check_point_light_mesh_visibility`) and the per-face caster hashing
//!   ([`crate::extract_shadow_caster_changes`]) skip it. The host fills the lamp's
//!   `CubemapVisibleEntities` itself, for the faces it wants.
//! - Faces outside [`CachedPointShadowMap::view_faces`] have no view entity, no shadow phase,
//!   no extracted caster list, no schedule run and no pass. Faces in `draw_faces` are drawn this
//!   frame (cleared first, or over the depth already there when in `load_faces`); the other
//!   view faces keep their view and phase but are skipped (`SkipRootViewRendering`).
//! - [`CachedPointShadowMap::copies`] are whole-layer copies between the live cube array and a
//!   pooled store texture (sized by [`PointShadowCacheStore`]), and clears of live layers, all
//!   recorded in one encoder before any shadow pass of the frame
//!   ([`record_cached_point_shadow_copies`], before `camera_driver`).
//! - A plan executes once: draws and copies run on the frame the component was written
//!   (Bevy change detection); an untouched component keeps its views, skipped, and copies
//!   nothing.
//! - Render-world feedback for the host ([`PointShadowCacheFeedback`]): faces drawn with every
//!   pipeline compiled and nothing pending (a capture can be confirmed), and lamps whose live
//!   layers were lost (new cube slot or a new live texture: everything must be captured again).
//!
//! Lamps without the component take the stock path unchanged (six faces, hashed, skipped when
//! unchanged, CW-3b face culling). The only stock-visible change: once a cached lamp has been
//! seen, the live cube texture gains `COPY_SRC | COPY_DST` (sticky, one reallocation).
//!
//! CW-3b's face test ([`crate::point_shadow_faces_seen`]) is the one function both paths use:
//! the stock path calls it in `extract_shadow_caster_changes`; the host calls it to decide which
//! mover faces it puts in `draw_faces`.

use alloc::sync::Arc;
use std::sync::Mutex;

use bevy_camera::primitives::{face_index_to_name, Frustum};
use bevy_camera::visibility::RenderLayers;
use bevy_core_pipeline::schedule::{Core3dShadows, RootNonCameraView};
use bevy_ecs::{
    entity::EntityHashMap,
    prelude::*,
    schedule::ScheduleLabel,
    system::{lifetimeless::Read, SystemParam},
};
use bevy_light::PointLight;
use bevy_math::{proj, Mat4, UVec4};
use bevy_platform::collections::HashSet;
use bevy_render::{
    batching::gpu_preprocessing::GpuPreprocessingMode,
    render_phase::{BinnedRenderPhase, ViewBinnedRenderPhases},
    render_resource::{
        CachedRenderPipelineId, CommandEncoderDescriptor, Extent3d, LoadOp, Operations, Origin3d,
        PipelineCache, RenderPassDepthStencilAttachment, RenderPassDescriptor, StoreOp,
        TexelCopyTextureInfo, Texture, TextureAspect, TextureDescriptor, TextureDimension,
        TextureId, TextureUsages, TextureViewDescriptor, TextureViewDimension,
    },
    renderer::{PendingCommandBuffers, RenderDevice},
    sync_world::{MainEntity, RenderEntity},
    texture::{CachedTexture, DepthStencilViewAttachment, DepthStencilViews},
    view::{ExtractedView, NoIndirectDrawing, RenderViewLodOrigins, RetainedViewEntity},
    Extract,
};
use bevy_shape::ViewFrustum;
use bevy_transform::components::{GlobalTransform, Transform};

use crate::{
    ExtractedPointLight, LightEntity, PendingShadowQueues, Shadow, ShadowView,
    ALL_POINT_SHADOW_FACES, CORE_3D_SHADOW_MAP_FORMAT,
};

/// One whole-layer operation on a cached lamp's cube face, recorded before any shadow pass of
/// the frame, in list order. `face` follows [`CUBE_MAP_FACES`] order; `layer` is a store layer,
/// `0..`[`PointShadowCacheStore::layers`]. Out-of-range faces or layers are dropped (counted in
/// [`PointShadowCacheStats::dropped_copies`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CachedPointShadowCopy {
    /// Live face -> store layer (the clean static depth, before movers are drawn over it).
    Save { face: u8, layer: u32 },
    /// Store layer -> live face (static depth back, before the movers are drawn again).
    Restore { face: u8, layer: u32 },
    /// Clears the live face (no static caster in it: the face is never drawn).
    Clear { face: u8 },
}

/// Main-world component on a `PointLight` entity: the lamp's shadow cube is managed by a host
/// cache (S3: `sf_sky_bevy::lamp_cache`). Keep `PointLight::shadow_maps_enabled = false` on such
/// a lamp; it is shaded as shadowed while the component is present. Masks are over cube faces,
/// bit `i` = face `i` in [`CUBE_MAP_FACES`] order (the shadow views' `subview_index`).
///
/// Write it every frame there is something to do (a plain mutable access marks it changed):
/// `draw_faces`, `load_faces` and `copies` apply only on a frame it was written. Removing it
/// returns the lamp to the stock path (set `shadow_maps_enabled` as wanted in the same frame);
/// its cached views are despawned.
#[derive(Component, Clone, Debug, Default, PartialEq, Eq)]
pub struct CachedPointShadowMap {
    /// Faces that keep a view entity and a retained shadow phase (drawn, waiting, lingering).
    /// `draw_faces` always have one. The lamp's `CubemapVisibleEntities` are extracted for
    /// these faces only.
    pub view_faces: u8,
    /// Faces drawn this frame, from the face's `CubemapVisibleEntities` list.
    pub draw_faces: u8,
    /// Of `draw_faces`, the faces drawn over the depth already in the live layer (movers over
    /// restored static depth) instead of cleared first (a capture).
    pub load_faces: u8,
    /// This frame's copies and clears, recorded before any shadow pass, in order.
    pub copies: Vec<CachedPointShadowCopy>,
    /// The host's plan number for this frame, echoed in [`CachedPointShadowReport::plan`].
    pub plan: u64,
}

impl CachedPointShadowMap {
    /// The masks as the renderer applies them: six faces only, `draw ⊆ view`, `load ⊆ draw`.
    #[must_use]
    pub fn faces(&self) -> CachedPointShadowFaces {
        let draw = self.draw_faces & ALL_POINT_SHADOW_FACES;
        CachedPointShadowFaces {
            view: (self.view_faces & ALL_POINT_SHADOW_FACES) | draw,
            draw,
            load: self.load_faces & draw,
        }
    }
}

/// The face masks of a cached lamp, sanitised ([`CachedPointShadowMap::faces`]).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct CachedPointShadowFaces {
    pub view: u8,
    pub draw: u8,
    pub load: u8,
}

/// What the renderer does with one cube face of a point light this frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PointShadowFacePlan {
    /// No view entity, no phase, no list, no pass.
    NoView,
    /// A view and its phase exist; not drawn this frame.
    Kept,
    /// Drawn this frame: `load` = over the existing depth, else cleared first.
    Draw { load: bool },
    /// A stock face: a view always; drawing decided by `skip_unchanged_shadow_views`.
    Stock,
}

/// The faces of a shadowed point light that have a view entity and a shadow phase:
/// every face for a stock lamp (`None`), `view` for a cached one.
#[must_use]
pub fn point_shadow_view_faces(cached: Option<&CachedPointShadowFaces>) -> u8 {
    cached.map_or(ALL_POINT_SHADOW_FACES, |faces| faces.view)
}

/// The plan of face `face` (`0..6`). `fresh`: the host wrote the component this frame (a stale
/// component draws nothing).
#[must_use]
pub fn point_shadow_face_plan(
    cached: Option<&CachedPointShadowFaces>,
    fresh: bool,
    face: u32,
) -> PointShadowFacePlan {
    let Some(faces) = cached else {
        return PointShadowFacePlan::Stock;
    };
    let bit = 1u8.checked_shl(face).unwrap_or(0);
    if faces.view & bit == 0 {
        PointShadowFacePlan::NoView
    } else if fresh && faces.draw & bit != 0 {
        PointShadowFacePlan::Draw {
            load: faces.load & bit != 0,
        }
    } else {
        PointShadowFacePlan::Kept
    }
}

/// Main-world resource: layers in the store texture (each one cube face at the point shadow
/// map size, `Depth32Float`). Absent or 0: no store; `Save`/`Restore` copies are dropped.
/// A new size or map size recreates the store empty.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PointShadowCacheStore {
    pub layers: u32,
}

/// What the render world saw of one cached lamp since the host last drained the feedback.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CachedPointShadowReport {
    /// The latest [`CachedPointShadowMap::plan`] the render world executed.
    pub plan: u64,
    /// Faces drawn whose shadow phase had every pipeline compiled and nothing pending
    /// (mesh or material not loaded): a capture of these faces holds every queued caster.
    pub confirmed: u8,
    /// The lamp's live layers were lost: it moved to another cube slot, or the live texture
    /// was reallocated. Nothing cached in its live faces is valid any more.
    pub lost: bool,
}

/// Main-world resource (cloned into the render world by `PbrPlugin`): render-world reports per
/// cached lamp (main-world entity), accumulated until [`Self::drain`]. With pipelined rendering
/// a report arrives a frame after its plan.
#[derive(Resource, Clone, Default)]
pub struct PointShadowCacheFeedback(Arc<Mutex<EntityHashMap<CachedPointShadowReport>>>);

impl PointShadowCacheFeedback {
    /// Takes every report since the last call.
    #[must_use]
    pub fn drain(&self) -> Vec<(Entity, CachedPointShadowReport)> {
        self.0
            .lock()
            .map(|mut map| map.drain().collect())
            .unwrap_or_default()
    }

    fn report(&self, light: Entity, update: impl FnOnce(&mut CachedPointShadowReport)) {
        if let Ok(mut map) = self.0.lock() {
            update(map.entry(light).or_default());
        }
    }
}

/// Render-world copy of [`CachedPointShadowMap`] on the light's render entity.
#[derive(Component, Clone, Debug, Default)]
pub struct ExtractedCachedPointShadowMap {
    pub faces: CachedPointShadowFaces,
    /// Written by the host this frame: draws and copies apply.
    pub fresh: bool,
    /// This frame's copies (empty when not fresh).
    pub copies: Vec<CachedPointShadowCopy>,
    pub plan: u64,
}

/// Render-world counts for the frame just prepared (gauges for the host).
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PointShadowCacheStats {
    /// Cached lamps with a cube slot.
    pub lamps: u32,
    /// Their face views.
    pub views: u32,
    /// Their faces drawn (`load` ones included).
    pub drawn: u32,
    /// Of `drawn`, over existing depth.
    pub loaded: u32,
    /// Copies and clears recorded.
    pub copies: u32,
    /// Copies dropped: face or store layer out of range, no store, or a `Save` from a lost lamp.
    pub dropped_copies: u32,
    /// Lamps that lost their live layers this frame.
    pub lost: u32,
}

/// Extracts [`CachedPointShadowMap`] and [`PointShadowCacheStore`]. Runs with
/// [`crate::extract_lights`] (which reads the component for the lamp's shadow flag and lists).
/// A lamp whose component was removed gets its stock shadow flag back here.
pub fn extract_cached_point_shadow_maps(
    mut commands: Commands,
    lamps: Extract<Query<(Entity, RenderEntity, Ref<CachedPointShadowMap>), With<PointLight>>>,
    point_lights: Extract<Query<&PointLight>>,
    store: Extract<Option<Res<PointShadowCacheStore>>>,
    mut render_store: Option<ResMut<PointShadowCacheStore>>,
    mut extracted_lights: Query<&mut ExtractedPointLight>,
    mut last: Local<EntityHashMap<Entity>>,
    mut now: Local<EntityHashMap<Entity>>,
) {
    let layers = store.as_deref().map_or(0, |s| s.layers);
    match render_store.as_deref_mut() {
        Some(render) if render.layers != layers => render.layers = layers,
        Some(_) => {}
        None => commands.insert_resource(PointShadowCacheStore { layers }),
    }

    now.clear();
    for (main, render, cached) in &lamps {
        now.insert(main, render);
        let fresh = cached.is_changed();
        let Ok(mut entity) = commands.get_entity(render) else {
            continue;
        };
        entity.insert(ExtractedCachedPointShadowMap {
            faces: cached.faces(),
            fresh,
            copies: if fresh { cached.copies.clone() } else { Vec::new() },
            plan: cached.plan,
        });
    }
    for (main, render) in last.iter() {
        if now.contains_key(main) {
            continue;
        }
        if let Ok(mut entity) = commands.get_entity(*render) {
            entity.remove::<ExtractedCachedPointShadowMap>();
        }
        // Stock flag back; `extract_lights` overwrites this the same frame if the light changed.
        if let Ok(mut light) = extracted_lights.get_mut(*render) {
            let stock = point_lights.get(*main).is_ok_and(|l| l.shadow_maps_enabled);
            if light.shadow_maps_enabled != stock {
                light.shadow_maps_enabled = stock;
            }
        }
    }
    core::mem::swap(&mut *last, &mut *now);
}

/// One recorded operation, resolved to texture layers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ResolvedCopy {
    LiveToStore { live: u32, store: u32 },
    StoreToLive { store: u32, live: u32 },
    ClearLive { live: u32 },
}

/// Render-world state of the cache: the store texture, this frame's copies, and what each lamp
/// had last frame (slot, face views).
#[derive(Resource, Default)]
pub struct PointShadowCacheGpu {
    live: Option<Texture>,
    live_size: u32,
    store: Option<(Texture, u32, u32)>,
    copies: Vec<ResolvedCopy>,
    /// Per cached lamp (render entity): the face view entities.
    face_views: EntityHashMap<[Option<Entity>; 6]>,
    /// Per cached lamp (render entity): the live texture and cube slot its faces were in.
    slots: EntityHashMap<(TextureId, usize)>,
    /// A cached lamp has existed: the live texture keeps the copy usages.
    copy_usage: bool,
    /// Face views given a LOD origin this frame and last frame.
    origin_views_now: HashSet<RetainedViewEntity>,
    origin_views_last: HashSet<RetainedViewEntity>,
}

/// The per-face state a cached face view was last set up with.
#[derive(Component, Clone, PartialEq)]
pub struct CachedPointShadowFaceState {
    texture_id: TextureId,
    array_layer: u32,
    viewport_size: u32,
    world_from_view: GlobalTransform,
    range: f32,
    near_z: f32,
    render_layers: RenderLayers,
    light_entity: Entity,
    load: bool,
}

/// `prepare_lights`' access to the cache (one parameter, so the stock signature keeps its
/// shape).
#[derive(SystemParam)]
pub struct PointShadowCachePrepare<'w, 's> {
    cached: Query<'w, 's, Read<ExtractedCachedPointShadowMap>>,
    face_states: Query<'w, 's, Read<CachedPointShadowFaceState>>,
    gpu: ResMut<'w, PointShadowCacheGpu>,
    stats: ResMut<'w, PointShadowCacheStats>,
    store: Option<Res<'w, PointShadowCacheStore>>,
    feedback: Option<Res<'w, PointShadowCacheFeedback>>,
    lod_origins: Option<ResMut<'w, RenderViewLodOrigins>>,
    /// Point and spot lights that still exist (their render entity is synced).
    lights: Query<'w, 's, (), With<crate::PointAndSpotLightViewEntities>>,
}

/// One cached lamp's inputs from `prepare_lights`' light loop.
pub(crate) struct CachedLightInputs<'a> {
    pub light_entity: Entity,
    pub light_main_entity: MainEntity,
    pub light: &'a ExtractedPointLight,
    pub light_render_layers: &'a RenderLayers,
    pub light_index: Option<usize>,
    pub cube_face_rotations: &'a [Transform],
    pub texture: &'a CachedTexture,
    pub map_size: u32,
    pub gpu_mode: GpuPreprocessingMode,
}

impl PointShadowCachePrepare<'_, '_> {
    /// Extra usages of the live cube texture: copies once a cached lamp has existed.
    pub(crate) fn live_texture_usage(&mut self) -> TextureUsages {
        if !self.cached.is_empty() {
            self.gpu.copy_usage = true;
        }
        if self.gpu.copy_usage {
            TextureUsages::COPY_SRC | TextureUsages::COPY_DST
        } else {
            TextureUsages::empty()
        }
    }

    /// Before the light loop: this frame's live texture, the store texture, counters.
    pub(crate) fn begin_frame(&mut self, device: &RenderDevice, live: &CachedTexture, size: u32) {
        *self.stats = PointShadowCacheStats::default();
        let gpu = &mut *self.gpu;
        gpu.copies.clear();
        gpu.live = Some(live.texture.clone());
        gpu.live_size = size;
        let layers = self.store.as_deref().map_or(0, |s| s.layers);
        let current = gpu.store.as_ref().map(|(_, s, l)| (*s, *l));
        if layers == 0 {
            gpu.store = None;
        } else if current != Some((size, layers)) {
            let texture = device.create_texture(&TextureDescriptor {
                label: Some("point_light_shadow_cache_store"),
                size: Extent3d {
                    width: size,
                    height: size,
                    depth_or_array_layers: layers,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: CORE_3D_SHADOW_MAP_FORMAT,
                usage: TextureUsages::COPY_SRC | TextureUsages::COPY_DST,
                view_formats: &[],
            });
            gpu.store = Some((texture, size, layers));
        }
    }

    /// In the light loop, for every shadowed point light. A cached lamp's face views, phases
    /// and copies are set up here and `true` is returned (the stock code skips it). A stock
    /// lamp that was cached loses its cached views (the stock code then spawns six) and `false`
    /// is returned.
    pub(crate) fn prepare_light(
        &mut self,
        commands: &mut Commands,
        inputs: &CachedLightInputs,
        view_entities: &mut Vec<Entity>,
        shadow_render_phases: &mut ViewBinnedRenderPhases<Shadow>,
        live_shadow_mapping_lights: &mut HashSet<RetainedViewEntity>,
    ) -> bool {
        let light_entity = inputs.light_entity;
        let Ok(cached) = self.cached.get(light_entity) else {
            if self.gpu.face_views.remove(&light_entity).is_some() {
                despawn_views(commands, core::mem::take(view_entities));
            }
            return false;
        };
        let faces = cached.faces;
        let main = inputs.light_main_entity;
        let gpu = &mut *self.gpu;
        let stats = &mut *self.stats;

        // The view entities must be the ones this cache spawned (a stock lamp turning cached
        // has six stock views; another system may have taken them).
        let slots = gpu.face_views.entry(light_entity).or_default();
        let mut known: Vec<Entity> = slots.iter().flatten().copied().collect();
        let mut listed = view_entities.clone();
        known.sort_unstable();
        listed.sort_unstable();
        if known != listed {
            despawn_views(commands, core::mem::take(view_entities));
            *slots = [None; 6];
        }

        stats.lamps += 1;

        // A lamp without a slot index this frame keeps nothing.
        let Some(light_index) = inputs.light_index else {
            let stale: Vec<Entity> = slots.iter_mut().filter_map(Option::take).collect();
            despawn_views(commands, stale);
            view_entities.clear();
            return true;
        };

        // Lost: the lamp's faces now live in other layers or another texture.
        let texture_id = inputs.texture.texture.id();
        let lost = gpu
            .slots
            .insert(light_entity, (texture_id, light_index))
            .is_some_and(|previous| previous != (texture_id, light_index));
        if lost {
            stats.lost += 1;
        }
        if let Some(feedback) = self.feedback.as_ref().filter(|_| lost || cached.fresh) {
            feedback.report(main.id(), |report| {
                report.lost |= lost;
                if cached.fresh {
                    report.plan = cached.plan;
                }
            });
        }

        // Copies, in order, before any shadow pass.
        if cached.fresh {
            let store = gpu.store.as_ref().map_or(0, |(_, _, layers)| *layers);
            for copy in &cached.copies {
                let face = match *copy {
                    CachedPointShadowCopy::Save { face, .. }
                    | CachedPointShadowCopy::Restore { face, .. }
                    | CachedPointShadowCopy::Clear { face } => u32::from(face),
                };
                let live = (light_index * 6) as u32 + face;
                let resolved = match *copy {
                    _ if face >= 6 => None,
                    CachedPointShadowCopy::Save { layer, .. } if layer < store && !lost => {
                        Some(ResolvedCopy::LiveToStore { live, store: layer })
                    }
                    CachedPointShadowCopy::Restore { layer, .. } if layer < store => {
                        Some(ResolvedCopy::StoreToLive { store: layer, live })
                    }
                    CachedPointShadowCopy::Clear { .. } => Some(ResolvedCopy::ClearLive { live }),
                    _ => None,
                };
                match resolved {
                    Some(resolved) => {
                        gpu.copies.push(resolved);
                        stats.copies += 1;
                    }
                    None => stats.dropped_copies += 1,
                }
            }
        }

        // Face views: exactly the faces in `view`.
        let light = inputs.light;
        let view_translation = GlobalTransform::from_translation(light.transform.translation());
        let cube_face_projection = proj::perspective_infinite_reverse(
            core::f32::consts::FRAC_PI_2,
            1.0,
            light.shadow_map_near_z,
        );
        let view_backward = light.transform.back();
        let slots = gpu.face_views.entry(light_entity).or_default();
        for (face_index, view_rotation) in inputs.cube_face_rotations.iter().enumerate().take(6) {
            let face = face_index as u32;
            let retained = RetainedViewEntity::new(main, None, face);
            let plan = point_shadow_face_plan(Some(&faces), cached.fresh, face);
            if plan == PointShadowFacePlan::NoView {
                if let Some(view) = slots[face_index].take() {
                    despawn_views(commands, vec![view]);
                }
                continue;
            }
            let view_entity = *slots[face_index].get_or_insert_with(|| commands.spawn_empty().id());
            stats.views += 1;

            let base_array_layer = (light_index * 6 + face_index) as u32;
            let world_from_view = view_translation * *view_rotation;
            // A stale component keeps the attachment it was set up with.
            let previous = self.face_states.get(view_entity).ok();
            let load = match plan {
                PointShadowFacePlan::Draw { load } => load,
                _ => previous.is_some_and(|state| state.load),
            };
            let state = CachedPointShadowFaceState {
                texture_id,
                array_layer: base_array_layer,
                viewport_size: inputs.map_size,
                world_from_view,
                range: light.range,
                near_z: light.shadow_map_near_z,
                render_layers: inputs.light_render_layers.clone(),
                light_entity,
                load,
            };

            if previous != Some(&state) {
                let needs_shadow_view = !previous.is_some_and(|p| {
                    p.texture_id == texture_id && p.array_layer == base_array_layer && p.load == load
                });
                if needs_shadow_view {
                    let depth_view = inputs.texture.texture.create_view(&TextureViewDescriptor {
                        label: Some("cached_point_light_shadow_map_texture_view"),
                        format: None,
                        dimension: Some(TextureViewDimension::D2),
                        usage: None,
                        aspect: TextureAspect::All,
                        base_mip_level: 0,
                        mip_level_count: None,
                        base_array_layer,
                        array_layer_count: Some(1u32),
                    });
                    commands.entity(view_entity).insert(ShadowView {
                        // `None`: loads the depth already there (movers over static depth).
                        depth_attachment: DepthStencilViewAttachment::new(
                            DepthStencilViews::DepthOnly { depth_view },
                            (!load).then_some(0.0),
                            None,
                        ),
                        pass_name: format!(
                            "shadow_point_light_{}_{}_cached",
                            light_index,
                            face_index_to_name(face_index)
                        ),
                    });
                }

                let clip_from_world =
                    cube_face_projection * Mat4::from(world_from_view.affine().inverse());
                let frustum = Frustum(ViewFrustum::from_clip_from_world_custom_far(
                    &clip_from_world,
                    &light.transform.translation(),
                    &view_backward,
                    light.range,
                ));
                commands.entity(view_entity).insert((
                    ExtractedView {
                        retained_view_entity: retained,
                        viewport: UVec4::new(0, 0, inputs.map_size, inputs.map_size),
                        world_from_view,
                        clip_from_world: None,
                        clip_from_view: cube_face_projection,
                        target_format: CORE_3D_SHADOW_MAP_FORMAT,
                        color_grading: Default::default(),
                        invert_culling: false,
                    },
                    frustum,
                    LightEntity::Point {
                        light_entity,
                        face_index,
                    },
                    inputs.light_render_layers.clone(),
                    state,
                    RootNonCameraView(Core3dShadows.intern()),
                ));
                if !matches!(inputs.gpu_mode, GpuPreprocessingMode::Culling) {
                    commands.entity(view_entity).insert(NoIndirectDrawing);
                }
            }

            // Levels of detail resolve from the lamp, not the viewer: a cached face does not
            // depend on where the camera stood when it was drawn.
            if let Some(origins) = self.lod_origins.as_deref_mut() {
                origins.0.insert(retained, light.transform.translation());
                gpu.origin_views_now.insert(retained);
            }
            shadow_render_phases.prepare_for_new_frame(retained, inputs.gpu_mode);
            live_shadow_mapping_lights.insert(retained);
        }
        view_entities.clear();
        view_entities.extend(slots.iter().flatten().copied());
        true
    }

    /// After the light loop: forgets lamps that no longer exist (their views went with them),
    /// and the LOD origins of face views that no longer exist. A lamp that exists but was not
    /// prepared this frame (hidden, no shadow) keeps its entry, so the stock path never meets
    /// its cached views (the next `prepare_light` despawns them).
    pub(crate) fn end_frame(&mut self) {
        let gpu = &mut *self.gpu;
        let lights = &self.lights;
        gpu.face_views.retain(|light, _| lights.contains(*light));
        gpu.slots.retain(|light, _| lights.contains(*light));
        if let Some(origins) = self.lod_origins.as_deref_mut() {
            for retained in gpu.origin_views_last.difference(&gpu.origin_views_now) {
                origins.0.remove(retained);
            }
        }
        core::mem::swap(&mut gpu.origin_views_last, &mut gpu.origin_views_now);
        gpu.origin_views_now.clear();
    }
}

fn despawn_views(commands: &mut Commands, views: Vec<Entity>) {
    for view in views {
        if let Ok(mut entity) = commands.get_entity(view) {
            entity.try_despawn();
        }
    }
}

/// `skip_unchanged_shadow_views`' access to the cache: the mask decides for cached lamps.
#[derive(SystemParam)]
pub struct PointShadowCacheSkip<'w, 's> {
    cached: Query<'w, 's, Read<ExtractedCachedPointShadowMap>>,
    pipeline_cache: Res<'w, PipelineCache>,
    pending: Res<'w, PendingShadowQueues>,
    feedback: Option<Res<'w, PointShadowCacheFeedback>>,
    stats: ResMut<'w, PointShadowCacheStats>,
}

impl PointShadowCacheSkip<'_, '_> {
    /// For a face view of a cached lamp: whether it renders this frame (and, if it does and its
    /// phase is complete, confirms the face to the host). `None`: a stock lamp.
    pub(crate) fn draws(
        &mut self,
        light_entity: Entity,
        main_light: Option<MainEntity>,
        retained: RetainedViewEntity,
        phase: Option<&BinnedRenderPhase<Shadow>>,
    ) -> Option<bool> {
        let cached = self.cached.get(light_entity).ok()?;
        let face = retained.subview_index;
        let plan = point_shadow_face_plan(Some(&cached.faces), cached.fresh, face);
        let PointShadowFacePlan::Draw { load } = plan else {
            return Some(false);
        };
        self.stats.drawn += 1;
        if load {
            self.stats.loaded += 1;
        }
        let complete = phase.is_some_and(|phase| phase_ready(phase, &self.pipeline_cache))
            && self
                .pending
                .get(&retained)
                .is_none_or(|queues| queues.current_frame.is_empty());
        if complete
            && let (Some(feedback), Some(main)) = (&self.feedback, main_light)
        {
            feedback.report(main.id(), |report| report.confirmed |= 1 << face);
        }
        Some(true)
    }
}

/// Every pipeline the phase's bins use is compiled (a draw whose pipeline is not ready is
/// skipped by Bevy, so its casters would be missing from a capture).
fn phase_ready(phase: &BinnedRenderPhase<Shadow>, cache: &PipelineCache) -> bool {
    let ready = |id: CachedRenderPipelineId| cache.get_render_pipeline(id).is_some();
    phase.multidrawable_meshes.keys().all(|key| ready(key.pipeline))
        && phase.batchable_meshes.keys().all(|(key, _)| ready(key.pipeline))
        && phase.unbatchable_meshes.keys().all(|(key, _)| ready(key.pipeline))
        && phase.non_mesh_items.keys().all(|(key, _)| ready(key.pipeline))
        && phase.instance_batches.keys().all(|(key, _)| ready(key.pipeline))
}

/// Records this frame's copies and clears in one encoder, queued before `camera_driver` runs
/// any shadow pass (render graph schedule, [`bevy_render::renderer::RenderGraphSystems::Render`]).
pub fn record_cached_point_shadow_copies(
    device: Res<RenderDevice>,
    mut gpu: ResMut<PointShadowCacheGpu>,
    mut pending: ResMut<PendingCommandBuffers>,
) {
    if gpu.copies.is_empty() {
        return;
    }
    let copies = core::mem::take(&mut gpu.copies);
    let Some(live) = gpu.live.clone() else {
        return;
    };
    let size = gpu.live_size;
    let store = gpu.store.as_ref().map(|(texture, _, _)| texture.clone());
    let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
        label: Some("point_shadow_cache_copies"),
    });
    fn layer(texture: &Texture, z: u32) -> TexelCopyTextureInfo<'_> {
        TexelCopyTextureInfo {
            texture,
            mip_level: 0,
            origin: Origin3d { x: 0, y: 0, z },
            aspect: TextureAspect::All,
        }
    }
    let extent = Extent3d {
        width: size,
        height: size,
        depth_or_array_layers: 1,
    };
    for copy in copies {
        match (copy, &store) {
            (ResolvedCopy::LiveToStore { live: l, store: s }, Some(store)) => {
                encoder.copy_texture_to_texture(layer(&live, l), layer(store, s), extent);
            }
            (ResolvedCopy::StoreToLive { store: s, live: l }, Some(store)) => {
                encoder.copy_texture_to_texture(layer(store, s), layer(&live, l), extent);
            }
            (ResolvedCopy::ClearLive { live: l }, _) => {
                let view = live.create_view(&TextureViewDescriptor {
                    label: Some("point_shadow_cache_clear_view"),
                    format: None,
                    dimension: Some(TextureViewDimension::D2),
                    usage: None,
                    aspect: TextureAspect::DepthOnly,
                    base_mip_level: 0,
                    mip_level_count: None,
                    base_array_layer: l,
                    array_layer_count: Some(1),
                });
                // A pass that only clears: the shadow passes' clear value (reverse Z, far = 0).
                let _pass = encoder.begin_render_pass(&RenderPassDescriptor {
                    label: Some("point_shadow_cache_clear"),
                    color_attachments: &[],
                    depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                        view: &view,
                        depth_ops: Some(Operations {
                            load: LoadOp::Clear(0.0),
                            store: StoreOp::Store,
                        }),
                        stencil_ops: None,
                    }),
                    timestamp_writes: None,
                    occlusion_query_set: None,
                    multiview_mask: None,
                });
            }
            _ => {}
        }
    }
    pending.push_encoder(encoder, "point_shadow_cache_copies");
}

/// Whether `extract_lights` keeps a light's list for shadow subview `subview`: every face of a
/// stock lamp, the view faces of a cached one.
pub(crate) fn cached_face_listed(cached: Option<CachedPointShadowFaces>, subview: u32) -> bool {
    cached.is_none_or(|faces| subview < 6 && u32::from(faces.view) & (1 << subview) != 0)
}

/// Whether `lists` holds a visible-entity list for every face in `faces` of the light `main`.
/// `extract_lights` keeps a light's lists only while they cover every face `prepare_lights`
/// gives a view and a phase (six on the stock path, the view faces when cached): a shadow view
/// never exists without its list.
#[must_use]
pub fn point_shadow_lists_cover(
    lists: &bevy_render::view::RenderShadowMapVisibleEntities,
    main: MainEntity,
    faces: u8,
) -> bool {
    (0..6u32)
        .filter(|face| faces & (1 << face) != 0)
        .all(|face| {
            lists.subviews.contains_key(&RetainedViewEntity {
                main_entity: main,
                auxiliary_entity: None,
                subview_index: face,
            })
        })
}
