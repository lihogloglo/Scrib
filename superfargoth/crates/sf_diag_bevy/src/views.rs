//! Per-view render counters (feature `views`; static-scene.md slice S0,
//! diag.md "Render views"). Gauges and their names: [`sf_diag::views`].
//!
//! A render-world system reads Bevy's public render-world state once per
//! frame, after the graph has been encoded:
//! - the binned phases (`ViewBinnedRenderPhases<P>` for `Opaque3d`,
//!   `AlphaMask3d`, `Shadow` and the prepass and deferred phases): queued
//!   entities, bins, batch sets built this frame, and distinct pipelines
//!   among the batch-set keys;
//! - of those batch sets, the GPU-authored instance batches (static-scene.md
//!   S3a2): their draw records, the indirect sub-draws inside them, and the
//!   output instance slots they reserved (vendored patch "own, GS: counters").
//!   The instance batches' bins also join the distinct-pipeline sets, so a
//!   pipeline used only by statics is counted;
//! - the mesh preprocessing work items per view
//!   (`BatchedInstanceBuffers::phase_instance_buffers[*].work_item_buffers`):
//!   reserved, not surviving - an instance batch reserves a work item per
//!   capacity slot and the GPU fills them in;
//! - the preprocessing bind groups created this frame
//!   (`PreprocessBindGroupCount`, vendored patches "own: preprocess bind group
//!   count" and "own, GS: counters": range unpacking and materialization).
//!
//! Views are classified by their render-world components: the
//! lowest-order `ExtractedCamera` is `main` (label `main`), other cameras
//! are `other` (`camera<order>`), `LightEntity::Directional` views are
//! cascades (`cascade<i>`), point-light faces and spot views are lamp faces
//! (`lamp.face<f>`, `lamp.spot`: every lamp shares its face's label). The
//! counts land in the render frame, one frame behind the main world (as
//! `render/thread/ms` does).

use bevy_app::{App, Plugin};
use bevy_core_pipeline::core_3d::{AlphaMask3d, Opaque3d, Opaque3dBatchSetKey};
use bevy_core_pipeline::deferred::{AlphaMask3dDeferred, Opaque3dDeferred};
use bevy_core_pipeline::prepass::{AlphaMask3dPrepass, Opaque3dPrepass, OpaqueNoLightmap3dBatchSetKey};
use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use bevy_pbr::{LightEntity, MeshInputUniform, MeshUniform, PreprocessBindGroupCount, Shadow, ShadowBatchSetKey};
use bevy_platform::collections::HashSet;
use bevy_render::batching::gpu_preprocessing::{BatchedInstanceBuffers, PreprocessWorkItemBuffers};
use bevy_render::camera::ExtractedCamera;
use bevy_render::render_phase::{BinnedPhaseItem, BinnedRenderPhase, ViewBinnedRenderPhases};
use bevy_render::render_resource::CachedRenderPipelineId;
use bevy_render::renderer::render_system;
use bevy_render::view::{ExtractedView, RetainedViewEntity};
use bevy_render::{Render, RenderApp, RenderSystems};
use sf_diag::views::{BindGroups, PhaseCounts, ViewCounts, ViewFrame, ViewKind};

use crate::{DiagCounters, DiagViews};

/// Adds the per-view render counters. Add it after `SfDiagPlugin` (it
/// shares `DiagCounters` and `DiagViews` with the render world); without a
/// render sub-app it does nothing.
#[derive(Default)]
pub struct RenderViewsPlugin;

impl Plugin for RenderViewsPlugin {
    fn build(&self, _app: &mut App) {}

    fn finish(&self, app: &mut App) {
        let world = app.world();
        let (Some(counters), Some(views)) = (
            world.get_resource::<DiagCounters>().cloned(),
            world.get_resource::<DiagViews>().cloned(),
        ) else {
            log::warn!("sf_diag: RenderViewsPlugin needs SfDiagPlugin first");
            return;
        };
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        render_app
            .insert_resource(counters)
            .insert_resource(views)
            .add_systems(Render, count_views.in_set(RenderSystems::Render).after(render_system));
    }
}

/// The batch-set key's pipeline, for the distinct-pipeline counts.
trait KeyPipeline {
    fn pipeline(&self) -> CachedRenderPipelineId;
}

impl KeyPipeline for Opaque3dBatchSetKey {
    fn pipeline(&self) -> CachedRenderPipelineId {
        self.pipeline
    }
}

impl KeyPipeline for OpaqueNoLightmap3dBatchSetKey {
    fn pipeline(&self) -> CachedRenderPipelineId {
        self.pipeline
    }
}

impl KeyPipeline for ShadowBatchSetKey {
    fn pipeline(&self) -> CachedRenderPipelineId {
        self.pipeline
    }
}

/// Every binned phase we count; a missing one (plugin not added) is skipped.
#[derive(SystemParam)]
struct Phases<'w> {
    opaque: Option<Res<'w, ViewBinnedRenderPhases<Opaque3d>>>,
    alpha_mask: Option<Res<'w, ViewBinnedRenderPhases<AlphaMask3d>>>,
    shadow: Option<Res<'w, ViewBinnedRenderPhases<Shadow>>>,
    opaque_prepass: Option<Res<'w, ViewBinnedRenderPhases<Opaque3dPrepass>>>,
    alpha_mask_prepass: Option<Res<'w, ViewBinnedRenderPhases<AlphaMask3dPrepass>>>,
    opaque_deferred: Option<Res<'w, ViewBinnedRenderPhases<Opaque3dDeferred>>>,
    alpha_mask_deferred: Option<Res<'w, ViewBinnedRenderPhases<AlphaMask3dDeferred>>>,
}

fn small(n: usize) -> u32 {
    u32::try_from(n).unwrap_or(u32::MAX)
}

/// One phase of one view; adds its pipelines to the view's and the frame's sets.
fn phase_counts<P>(
    name: &'static str,
    phases: Option<&ViewBinnedRenderPhases<P>>,
    view: &RetainedViewEntity,
    sets: (&mut HashSet<CachedRenderPipelineId>, &mut HashSet<CachedRenderPipelineId>),
) -> Option<PhaseCounts>
where
    P: BinnedPhaseItem,
    P::BatchSetKey: KeyPipeline,
{
    let phase: &BinnedRenderPhase<P> = phases?.get(view)?;
    let keys = phase
        .multidrawable_meshes
        .keys()
        .chain(phase.batchable_meshes.keys().map(|(k, _)| k))
        .chain(phase.unbatchable_meshes.keys().map(|(k, _)| k))
        .chain(phase.non_mesh_items.keys().map(|(k, _)| k))
        // static-scene.md S3a2: without this a pipeline used only by the
        // GPU-authored instance batches vanishes from `views/*/pipelines`.
        .chain(phase.instance_batches.keys().map(|(k, _)| k));
    let mut pipelines: HashSet<CachedRenderPipelineId> = HashSet::default();
    for key in keys {
        pipelines.insert(key.pipeline());
    }
    let (view_set, frame_set) = sets;
    view_set.extend(pipelines.iter().copied());
    frame_set.extend(pipelines.iter().copied());
    Some(PhaseCounts {
        phase: name,
        items: small(phase.entity_count()),
        bins: small(phase.bin_count()),
        batch_sets: small(phase.batch_set_count()),
        pipelines: small(pipelines.len()),
        instance_batches: small(phase.instance_batch_draw_count()),
        instance_draws: small(phase.instance_batch_indirect_draw_count()),
        instance_slots: phase.instance_batch_slot_count(),
        // One draw call per entity (`render_unbatchable_meshes` / `render_non_meshes`).
        unbatchable: small(phase.unbatchable_meshes.values().map(|bin| bin.entities.len()).sum()),
        non_mesh: small(phase.non_mesh_items.values().map(|bin| bin.entities.len()).sum()),
    })
}

/// Mesh preprocessing work items written for one view, all phases: the
/// early (or only) pass; the late occlusion pass re-reads the same items.
///
/// **Reserved, not surviving.** `PartialBufferVec::len` includes the
/// uninitialised tail, which is exactly what a GPU-authored instance batch
/// reserves (one slot per capacity slot, filled in by the range-unpacking
/// dispatch), so a view with statics reports the dispatch width, not the
/// instances the cull let through. That number is GPU-side only
/// (`early_instance_count`).
fn work_items(buffers: &BatchedInstanceBuffers<MeshUniform, MeshInputUniform>, view: &RetainedViewEntity) -> u32 {
    let n: usize = buffers
        .phase_instance_buffers
        .values()
        .filter_map(|phase| phase.work_item_buffers.get(view))
        .map(|items| match items {
            PreprocessWorkItemBuffers::Direct(items) => items.len(),
            PreprocessWorkItemBuffers::Indirect { indexed, non_indexed, .. } => indexed.len() + non_indexed.len(),
        })
        .sum();
    small(n)
}

fn classify(camera: Option<&ExtractedCamera>, light: Option<&LightEntity>, main_order: Option<isize>) -> (ViewKind, String) {
    match (light, camera) {
        (Some(LightEntity::Directional { cascade_index, .. }), _) => (ViewKind::Cascade, format!("cascade{cascade_index}")),
        (Some(LightEntity::Point { face_index, .. }), _) => (ViewKind::LampFace, format!("lamp.face{face_index}")),
        (Some(LightEntity::Spot { .. }), _) => (ViewKind::LampFace, "lamp.spot".to_owned()),
        (None, Some(c)) if Some(c.order) == main_order => (ViewKind::Main, "main".to_owned()),
        (None, Some(c)) => (ViewKind::Other, format!("camera{}", c.order)),
        (None, None) => (ViewKind::Other, "other".to_owned()),
    }
}

fn count_views(
    views: Query<(&ExtractedView, Option<&ExtractedCamera>, Option<&LightEntity>)>,
    phases: Phases,
    buffers: Option<Res<BatchedInstanceBuffers<MeshUniform, MeshInputUniform>>>,
    bind_groups: Option<Res<PreprocessBindGroupCount>>,
    counters: Res<DiagCounters>,
    pending: Res<DiagViews>,
) {
    let main_order = views.iter().filter(|(_, _, light)| light.is_none()).filter_map(|(_, camera, _)| camera.map(|c| c.order)).min();
    let mut frame_set: HashSet<CachedRenderPipelineId> = HashSet::default();
    let mut frame = ViewFrame {
        bind_groups: bind_groups.map(|b| BindGroups {
            created: b.created,
            reused: b.reused,
            ranges: b.ranges,
            materialize: b.materialize,
        }),
        ..ViewFrame::default()
    };
    for (view, camera, light) in &views {
        let key = &view.retained_view_entity;
        let mut view_set: HashSet<CachedRenderPipelineId> = HashSet::default();
        let list = [
            phase_counts("opaque3d", phases.opaque.as_deref(), key, (&mut view_set, &mut frame_set)),
            phase_counts("alpha_mask3d", phases.alpha_mask.as_deref(), key, (&mut view_set, &mut frame_set)),
            phase_counts("shadow", phases.shadow.as_deref(), key, (&mut view_set, &mut frame_set)),
            phase_counts("opaque3d_prepass", phases.opaque_prepass.as_deref(), key, (&mut view_set, &mut frame_set)),
            phase_counts("alpha_mask3d_prepass", phases.alpha_mask_prepass.as_deref(), key, (&mut view_set, &mut frame_set)),
            phase_counts("opaque3d_deferred", phases.opaque_deferred.as_deref(), key, (&mut view_set, &mut frame_set)),
            phase_counts("alpha_mask3d_deferred", phases.alpha_mask_deferred.as_deref(), key, (&mut view_set, &mut frame_set)),
        ];
        let view_phases: Vec<PhaseCounts> = list.into_iter().flatten().collect();
        if view_phases.is_empty() {
            continue; // no binned phase: a UI or 2D camera
        }
        let (kind, label) = classify(camera, light, main_order);
        frame.views.push(ViewCounts {
            label,
            kind,
            phases: view_phases,
            work_items: buffers.as_deref().map_or(0, |b| work_items(b, key)),
            pipelines: small(view_set.len()),
        });
    }
    frame.distinct_pipelines = small(frame_set.len());
    frame.record(&counters);
    pending.put(frame);
}
