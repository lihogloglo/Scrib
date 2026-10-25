//! Pipeline compile watch: logs the render pipelines Bevy creates and how
//! many still compile (`PipelineCache::waiting_pipelines`). A draw whose
//! pipeline is still compiling is skipped, so content "missing" for a few
//! seconds after a load or a door crossing shows up here as a burst of new
//! pipelines (SH-17). `BEVYRROWIND_PIPELINE_DEFS=1` also logs each new render
//! pipeline's fragment shader defs, to tell which key bit made it new.
//!
//! The door fade holds at black while a mesh the arrival's camera sees still
//! waits for its main-pass pipeline ([`PendingDraws`]), within the fade's gate
//! cap: the loading-screen practice of not showing a frame before the
//! pipelines (PSOs) it draws with are compiled (Bevy's `loading_screen`
//! example waits on the same cache; Unreal's PSO precaching hides such
//! primitives instead).

use bevy::pbr::SpecializedMaterialPipelineCache;
use bevy::prelude::*;
use bevy::render::camera::ExtractedCamera;
use bevy::render::render_resource::{CachedPipelineState, PipelineCache, PipelineDescriptor};
use bevy::render::view::{ExtractedView, RenderVisibleEntities};
use bevy::render::{Render, RenderApp, RenderSystems};
use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

/// Meshes visible to a camera whose main-pass pipeline is still compiling
/// (Bevy skips their draws until it is ready). Written in the render world,
/// read in the main world (one frame late, which the fade's hold absorbs).
/// Also carries `PipelineCache::waiting_pipelines` (queued or compiling)
/// for the screenshot readiness gate ([`sf_diag_bevy::ReadyGate`]).
#[derive(Resource, Clone, Default)]
pub struct PendingDraws {
    draws: Arc<AtomicUsize>,
    pipelines: Arc<AtomicUsize>,
}

impl PendingDraws {
    pub fn get(&self) -> usize {
        self.draws.load(Ordering::Relaxed)
    }

    /// Pipelines still compiling and the visible draws they hold back.
    pub fn gpu(&self) -> sf_diag_bevy::GpuPending {
        sf_diag_bevy::GpuPending { pipelines: self.pipelines.load(Ordering::Relaxed), draws: self.get() }
    }
}

/// Seconds a burst of new pipelines is gathered into one log line.
const FLUSH_SECS: f32 = 0.5;

#[derive(Default)]
struct Watch {
    seen: usize,
    waiting: usize,
    pending: BTreeMap<String, u32>,
    since: Option<Instant>,
    defs: bool,
}

pub fn add(app: &mut App) {
    let defs = std::env::var_os("BEVYRROWIND_PIPELINE_DEFS").is_some();
    let pending = PendingDraws::default();
    app.insert_resource(pending.clone())
        .add_systems(Update, pipelines_at_black.before(sf_world_bevy::StreamingSystems::Transition));
    if let Some(render) = app.get_sub_app_mut(RenderApp) {
        let mut watch = Watch { defs, ..Watch::default() };
        render.insert_resource(pending).add_systems(
            Render,
            ((move |cache: Res<PipelineCache>| watch.update(&cache)), count_pending).in_set(RenderSystems::Cleanup),
        );
    }
}

/// Holds the door fade at black while the arrival's visible meshes wait for pipelines.
fn pipelines_at_black(fade: Option<ResMut<sf_world_bevy::FadeTransition>>, pending: Res<PendingDraws>) {
    let Some(mut fade) = fade else { return };
    if fade.is_holding() && pending.get() > 0 {
        fade.hold();
    }
}

/// Counts [`PendingDraws`]: the cameras' visible meshes whose specialized
/// main-pass pipeline is queued or compiling (as `queue_material_meshes`
/// looks them up; shadow views are left out).
fn count_pending(
    pending: Res<PendingDraws>,
    cache: Res<PipelineCache>,
    specialized: Option<Res<SpecializedMaterialPipelineCache>>,
    views: Query<(&ExtractedView, &RenderVisibleEntities), With<ExtractedCamera>>,
) {
    let mut n = 0;
    if let Some(specialized) = specialized {
        for (view, visible) in &views {
            let (Some(pipelines), Some(meshes)) = (specialized.get(&view.retained_view_entity), visible.get::<Mesh3d>()) else {
                continue;
            };
            let main = meshes.entities_cpu_culling.iter().map(|(_, m)| m).chain(meshes.entities_gpu_culling.keys());
            n += main
                .filter_map(|m| pipelines.get(m))
                .filter(|id| matches!(cache.get_render_pipeline_state(**id), CachedPipelineState::Queued | CachedPipelineState::Creating(_)))
                .count();
        }
    }
    pending.draws.store(n, Ordering::Relaxed);
    pending.pipelines.store(cache.waiting_pipelines().count(), Ordering::Relaxed);
}

impl Watch {
    fn update(&mut self, cache: &PipelineCache) {
        let all: Vec<_> = cache.pipelines().collect();
        for p in all.iter().skip(self.seen) {
            let (label, defs) = match &p.descriptor {
                PipelineDescriptor::RenderPipelineDescriptor(d) => {
                    (d.label.as_deref().unwrap_or("render"), d.fragment.as_ref().map(|f| format!("{:?}", f.shader_defs)))
                }
                PipelineDescriptor::ComputePipelineDescriptor(d) => (d.label.as_deref().unwrap_or("compute"), None),
                PipelineDescriptor::MeshPipelineDescriptor(d) => (d.label.as_deref().unwrap_or("mesh"), None),
            };
            *self.pending.entry(label.to_owned()).or_default() += 1;
            if self.defs
                && let Some(defs) = defs
            {
                info!("pipeline: new {label}: {defs}");
            }
        }
        let new = all.len() > self.seen;
        self.seen = all.len();
        let waiting = cache.waiting_pipelines().count();
        if new && self.since.is_none() {
            self.since = Some(Instant::now());
        }
        let due = self.since.is_some_and(|t| t.elapsed().as_secs_f32() >= FLUSH_SECS);
        if (due || (waiting == 0 && self.waiting > 0)) && !self.pending.is_empty() {
            let total: u32 = self.pending.values().sum();
            let labels: Vec<String> = self.pending.iter().map(|(l, n)| format!("{l} x{n}")).collect();
            info!("pipelines: +{total} new ({}), {waiting} compiling", labels.join(", "));
            self.pending.clear();
            self.since = None;
        } else if waiting == 0 && self.waiting > 0 {
            info!("pipelines: all compiled");
        }
        self.waiting = waiting;
    }
}
