//! Untraced per-stage wall times (`docs/research/cpu-frame-audit-2026-11-15.md` M1), added by
//! [`crate::FramePhasePlugin`]. All `ms`, [`sf_diag::CounterKind::Sum`]:
//!
//! - Main thread, `stage/main/<schedule>`: `first`, `pre_update`, `fixed_main_loop`
//!   (`RunFixedMainLoop`), `update`, `post_update`, `last`. Marker schedules (one system each,
//!   single-threaded executor) sit right before and after each schedule in `MainScheduleOrder`.
//! - Render thread (feature `render`), `stage/render/<set>`: completion-to-completion walls of
//!   the `Render` schedule's sets, in [`render::SETS`] order. A marker system ordered after a set
//!   (`after_ignore_deferred`: no sync point, nothing waits on it) stamps the set's end; a stage's
//!   wall is its end minus the latest end of the stages before it (0 if it finished earlier: the
//!   sets are only weakly chained and may overlap), so the stages sum to the schedule's span up to
//!   `Cleanup`. Command applies between sets fall into the next stage. End stamps lag by the task
//!   spawn of the marker (upper bounds). Inside `render_system`: `stage/render/camera_driver`
//!   (the `RenderGraph` schedule's `Render` set: every view's schedule) and
//!   `stage/render/graph_submit` (its `Submit` set: finishing the pending encoders + submit).
//!
//! Lands in the main frame being recorded when the render thread finishes (as `phases.rs`).

use bevy_app::{App, First, Last, MainScheduleOrder, PostUpdate, PreUpdate, RunFixedMainLoop, Update};
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::{ScheduleLabel, SingleThreadedExecutor};
use bevy_platform::time::Instant;

use crate::DiagCounters;

/// Main-thread stage counter names, in schedule order.
pub(crate) const MAIN_STAGES: [&str; 6] = [
    "stage/main/first",
    "stage/main/pre_update",
    "stage/main/fixed_main_loop",
    "stage/main/update",
    "stage/main/post_update",
    "stage/main/last",
];

#[derive(ScheduleLabel, Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct StageMark {
    stage: usize,
    end: bool,
}

/// The render-stage markers: `report` runs after all of them, so no stamp is read before it is written
/// (render-cleanup 2026-11-15: unordered, `report` often ran first and Cleanup absorbed the Render set).
#[cfg(feature = "render")]
#[derive(SystemSet, Clone, Debug, PartialEq, Eq, Hash)]
struct RenderStageMarks;

#[derive(Resource, Default)]
struct MainStages([Option<Instant>; MAIN_STAGES.len()]);

pub(crate) fn add_main(app: &mut App) {
    app.init_resource::<MainStages>();
    add_stage(app, 0, First);
    add_stage(app, 1, PreUpdate);
    add_stage(app, 2, RunFixedMainLoop);
    add_stage(app, 3, Update);
    add_stage(app, 4, PostUpdate);
    add_stage(app, 5, Last);
}

fn add_stage(app: &mut App, i: usize, label: impl ScheduleLabel + Clone) {
    let start = StageMark { stage: i, end: false };
    let end = StageMark { stage: i, end: true };
    for mark in [start, end] {
        app.init_schedule(mark).edit_schedule(mark, |s| {
            s.set_executor(SingleThreadedExecutor::new());
        });
    }
    app.add_systems(start, (move |mut c: ResMut<MainStages>| c.0[i] = Some(Instant::now())).run_if(crate::gauges_live))
        .add_systems(end, (move |mut c: ResMut<MainStages>, counters: Option<Res<DiagCounters>>| {
            if let (Some(t), Some(counters)) = (c.0[i].take(), counters) {
                counters.add_duration_named(MAIN_STAGES[i], t.elapsed());
            }
        }).run_if(crate::gauges_live));
    let mut order = app.world_mut().resource_mut::<MainScheduleOrder>();
    order.insert_before(label.clone(), start);
    order.insert_after(label, end);
}

/// Completion-to-completion walls: `ends[i]` is stage `i`'s end offset (ms from the schedule
/// start, NaN = not stamped); stage `i` gets its end minus the latest end before it, at least 0.
#[cfg(any(test, feature = "render"))]
pub(crate) fn completion_deltas(ends: &[f64]) -> Vec<f64> {
    let mut latest = 0.0_f64;
    ends.iter()
        .map(|&end| {
            if end.is_nan() {
                return 0.0;
            }
            let delta = (end - latest).max(0.0);
            latest = latest.max(end);
            delta
        })
        .collect()
}

#[cfg(feature = "render")]
pub(crate) mod render {
    use std::sync::atomic::{AtomicU64, Ordering};

    use bevy_app::App;
    use bevy_ecs::prelude::*;
    use bevy_platform::time::Instant;
    use bevy_render::renderer::{RenderGraph, RenderGraphSystems};
    use bevy_render::{Render, RenderApp, RenderSystems};
    use sf_diag::unit;

    use super::completion_deltas;
    use crate::DiagCounters;

    /// The `Render` schedule's sets, in completion order, with their counter names.
    pub(crate) const SETS: [(&str, RenderSystems); 17] = [
        ("stage/render/extract_commands", RenderSystems::ExtractCommands),
        ("stage/render/prepare_assets", RenderSystems::PrepareAssets),
        ("stage/render/prepare_meshes", RenderSystems::PrepareMeshes),
        ("stage/render/create_views", RenderSystems::CreateViews),
        ("stage/render/specialize", RenderSystems::Specialize),
        ("stage/render/prepare_views", RenderSystems::PrepareViews),
        ("stage/render/queue", RenderSystems::Queue),
        ("stage/render/phase_sort", RenderSystems::PhaseSort),
        ("stage/render/prepare_resources", RenderSystems::PrepareResources),
        ("stage/render/prepare_batch_phases", RenderSystems::PrepareResourcesBatchPhases),
        ("stage/render/prepare_write_phase_buffers", RenderSystems::PrepareResourcesWritePhaseBuffers),
        ("stage/render/prepare_collect_phase_buffers", RenderSystems::PrepareResourcesCollectPhaseBuffers),
        ("stage/render/prepare_flush", RenderSystems::PrepareResourcesFlush),
        ("stage/render/prepare_bind_groups", RenderSystems::PrepareBindGroups),
        ("stage/render/prepare", RenderSystems::Prepare),
        ("stage/render/render", RenderSystems::Render),
        ("stage/render/cleanup", RenderSystems::Cleanup),
    ];
    const CLEANUP: usize = SETS.len() - 1;

    /// Stamps as f64 ms bits since `base` (NaN = unset). Atomics, so the markers only read the
    /// resource and never conflict with each other or with anything else.
    #[derive(Resource)]
    struct Stamps {
        base: Instant,
        start: AtomicU64,
        ends: [AtomicU64; SETS.len()],
        /// `RenderGraph`: start of `Render`, end of `Render`, end of `Submit`.
        graph: [AtomicU64; 3],
    }

    const UNSET: u64 = 0x7ff8_0000_0000_0000;

    impl Stamps {
        fn now(&self) -> f64 {
            self.base.elapsed().as_secs_f64() * 1000.0
        }
        fn stamp(&self, slot: &AtomicU64) {
            slot.store(self.now().to_bits(), Ordering::Relaxed);
        }
        fn take(slot: &AtomicU64) -> f64 {
            f64::from_bits(slot.swap(UNSET, Ordering::Relaxed))
        }
    }

    pub(crate) fn add(app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else { return };
        if !render_app.world().contains_resource::<DiagCounters>() {
            return;
        }
        crate::encode::install();
        render_app.insert_resource(Stamps {
            base: Instant::now(),
            start: AtomicU64::new(UNSET),
            ends: core::array::from_fn(|_| AtomicU64::new(UNSET)),
            graph: core::array::from_fn(|_| AtomicU64::new(UNSET)),
        });
        // Exclusive, like `apply_extract_commands` that opens the schedule: no extra barrier.
        render_app.add_systems(
            Render,
            (|world: &mut World| {
                let s = world.resource::<Stamps>();
                s.stamp(&s.start);
            })
            .run_if(crate::gauges_live)
            .before_ignore_deferred(RenderSystems::ExtractCommands),
        );
        for (i, (_, set)) in SETS.iter().enumerate().take(CLEANUP) {
            render_app.add_systems(
                Render,
                (move |s: Res<Stamps>| s.stamp(&s.ends[i])).run_if(crate::gauges_live).after_ignore_deferred(set.clone()).in_set(super::RenderStageMarks),
            );
        }
        render_app.add_systems(
            Render,
            report.run_if(crate::gauges_live).after_ignore_deferred(RenderSystems::Cleanup).after_ignore_deferred(super::RenderStageMarks),
        );
        // `RenderGraph` runs exclusive systems (`camera_driver`, the submit): exclusive markers
        // strictly between its sets add no barrier.
        let graph_mark = |slot: usize| {
            move |world: &mut World| {
                let s = world.resource::<Stamps>();
                s.stamp(&s.graph[slot]);
            }
        };
        render_app.add_systems(
            RenderGraph,
            (
                graph_mark(0)
                    .run_if(crate::gauges_live)
                    .after_ignore_deferred(RenderGraphSystems::Begin)
                    .before_ignore_deferred(RenderGraphSystems::Render),
                graph_mark(1)
                    .run_if(crate::gauges_live)
                    .after_ignore_deferred(RenderGraphSystems::Render)
                    .before_ignore_deferred(RenderGraphSystems::Submit),
                graph_mark(2)
                    .run_if(crate::gauges_live)
                    .after_ignore_deferred(RenderGraphSystems::Submit)
                    .before_ignore_deferred(RenderGraphSystems::Finish),
            ),
        );
    }

    fn report(s: Res<Stamps>, counters: Res<DiagCounters>) {
        s.stamp(&s.ends[CLEANUP]);
        let start = Stamps::take(&s.start);
        let ends: Vec<f64> = s.ends.iter().map(|e| Stamps::take(e) - start).collect();
        let graph: [f64; 3] = core::array::from_fn(|i| Stamps::take(&s.graph[i]));
        if start.is_nan() {
            return;
        }
        for ((name, _), ms) in SETS.iter().zip(completion_deltas(&ends)) {
            counters.add_named(name, unit::MS, ms);
        }
        let span = |a: f64, b: f64| if a.is_nan() || b.is_nan() { 0.0 } else { (b - a).max(0.0) };
        counters.add_named("stage/render/camera_driver", unit::MS, span(graph[0], graph[1]));
        counters.add_named("stage/render/graph_submit", unit::MS, span(graph[1], graph[2]));
        crate::encode::report(&counters);
    }
}
