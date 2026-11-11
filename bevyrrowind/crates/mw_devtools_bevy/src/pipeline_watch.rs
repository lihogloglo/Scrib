//! Pipeline compile watch: logs the render pipelines Bevy creates and how
//! many still compile (`PipelineCache::waiting_pipelines`). A draw whose
//! pipeline is still compiling is skipped, so content "missing" for a few
//! seconds after a load or a door crossing shows up here as a burst of new
//! pipelines (SH-17). `BEVYRROWIND_PIPELINE_DEFS=1` also logs each new render
//! pipeline's fragment shader defs, to tell which key bit made it new.
//!
//! The holds themselves are the framework's reveal gate (`sf_world_bevy::RevealGate`,
//! pipeline-warmup.md W1): the door fade and `Loading` -> `InGame` wait for the arrival's
//! textures, prepared assets and pipelines (shadow views included), and write the `reveal:`
//! lines. The counting lives in the framework too (`sf_diag_bevy::PipelineWatchPlugin`, W0):
//! pending and unprepared draws, compile times (front end vs driver), view-key flips, and what
//! arrived late after each reveal.

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_log::prelude::*;
use bevy_render::render_resource::{PipelineCache, PipelineDescriptor};
use bevy_render::{Render, RenderApp, RenderSystems};
use std::collections::BTreeMap;
use std::time::Instant;

pub use sf_diag_bevy::PendingDraws;

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
    app.add_plugins(sf_diag_bevy::PipelineWatchPlugin);
    if let Some(render) = app.get_sub_app_mut(RenderApp) {
        let mut watch = Watch { defs, ..Watch::default() };
        render.add_systems(Render, (move |cache: Res<PipelineCache>| watch.update(&cache)).in_set(RenderSystems::Cleanup));
    }
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
