//! Frame phases: busy time vs swap-chain waits (diag.md "Frame pacing").
//!
//! Counters, all `ms`:
//! - `main/busy/ms` ([`sf_diag::pacing::MAIN_BUSY`]): the main schedule,
//!   from a schedule inserted before `First` to one after `Last`. The main
//!   thread's wait for the render thread (pipelined rendering) is outside.
//! - Render thread (feature `render`), landing in the main frame being
//!   recorded when the render thread finishes (one frame behind):
//!   `render/thread/ms` (render schedule, extract commands to cleanup),
//!   `render/acquire/ms` (`prepare_windows`: swap-chain acquire, blocks while
//!   the GPU or the compositor holds every image), `render/graph/ms` (the
//!   `RenderGraph` schedule: encoding and its submits), `render/present/ms`
//!   (the rest of `render_system`: the final submit and `present`) and
//!   `render/busy/ms` ([`sf_diag::pacing::RENDER_BUSY`] = thread − acquire −
//!   present).
//!
//! `sf_diag::pacing::derive_work` turns them into `frame/work/ms` at the end
//! of a run.

use bevy_app::{App, First, Last, MainScheduleOrder, Plugin};
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::ScheduleLabel;
use bevy_platform::time::Instant;
use sf_diag::pacing::MAIN_BUSY;


use crate::DiagCounters;

/// Adds the frame-phase counters (module docs). Add it with the app's
/// `DiagCounters` in place (after `SfDiagPlugin`); the render-thread half
/// needs feature `render` and a render sub-app.
pub struct FramePhasePlugin;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
struct FrameStart;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
struct FrameEnd;

#[derive(Resource, Default)]
struct MainClock(Option<Instant>);

impl Plugin for FramePhasePlugin {
    fn build(&self, app: &mut App) {
        app.init_schedule(FrameStart)
            .init_schedule(FrameEnd)
            .init_resource::<MainClock>()
            .add_systems(FrameStart, |mut clock: ResMut<MainClock>| clock.0 = Some(Instant::now()))
            .add_systems(FrameEnd, main_end);
        let mut order = app.world_mut().resource_mut::<MainScheduleOrder>();
        order.insert_before(First, FrameStart);
        order.insert_after(Last, FrameEnd);
    }

    #[cfg(feature = "render")]
    fn finish(&self, app: &mut App) {
        render::add(app);
    }
}

fn main_end(mut clock: ResMut<MainClock>, counters: Option<Res<DiagCounters>>) {
    if let (Some(t), Some(counters)) = (clock.0.take(), counters) {
        counters.add_duration_named(MAIN_BUSY, t.elapsed());
    }
}

#[cfg(feature = "render")]
mod render {
    use bevy_app::App;
    use bevy_ecs::prelude::*;
    use bevy_platform::time::Instant;
    use bevy_render::renderer::{RenderGraph, RenderGraphSystems, render_system};
    use bevy_render::view::prepare_windows;
    use bevy_render::{Render, RenderApp, RenderSystems};
    use sf_diag::pacing::RENDER_BUSY;
    use sf_diag::unit;

    use crate::DiagCounters;

    #[derive(Resource, Default)]
    struct Clock {
        frame: Option<Instant>,
        acquire: Option<Instant>,
        submit: Option<Instant>,
        graph: Option<Instant>,
        acquire_ms: f64,
        render_ms: f64,
        graph_ms: f64,
    }

    pub(super) fn add(app: &mut App) {
        let Some(counters) = app.world().get_resource::<DiagCounters>().cloned() else { return };
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else { return };
        render_app
            .insert_resource(counters)
            .init_resource::<Clock>()
            .add_systems(Render, frame_start.in_set(RenderSystems::ExtractCommands))
            .add_systems(Render, (|mut c: ResMut<Clock>| c.acquire = Some(Instant::now())).in_set(RenderSystems::PrepareViews).before(prepare_windows))
            .add_systems(Render, acquire_end.in_set(RenderSystems::PrepareViews).after(prepare_windows))
            .add_systems(Render, (|mut c: ResMut<Clock>| c.submit = Some(Instant::now())).in_set(RenderSystems::Render).before(render_system))
            .add_systems(Render, submit_end.in_set(RenderSystems::Render).after(render_system))
            .add_systems(Render, frame_end.in_set(RenderSystems::Cleanup))
            .add_systems(RenderGraph, (|mut c: ResMut<Clock>| c.graph = Some(Instant::now())).in_set(RenderGraphSystems::Begin))
            .add_systems(RenderGraph, graph_end.in_set(RenderGraphSystems::Finish));
    }

    fn ms(t: Option<Instant>) -> f64 {
        t.map_or(0.0, |t| t.elapsed().as_secs_f64() * 1000.0)
    }

    fn frame_start(mut c: ResMut<Clock>) {
        *c = Clock { frame: Some(Instant::now()), ..Clock::default() };
    }

    fn acquire_end(mut c: ResMut<Clock>) {
        c.acquire_ms = ms(c.acquire.take());
    }

    fn graph_end(mut c: ResMut<Clock>) {
        c.graph_ms = ms(c.graph.take());
    }

    fn submit_end(mut c: ResMut<Clock>) {
        c.render_ms = ms(c.submit.take());
    }

    fn frame_end(mut c: ResMut<Clock>, counters: Res<DiagCounters>) {
        let Some(start) = c.frame.take() else { return };
        let thread = ms(Some(start));
        let present = (c.render_ms - c.graph_ms).max(0.0);
        counters.add_named("render/thread/ms", unit::MS, thread);
        counters.add_named("render/acquire/ms", unit::MS, c.acquire_ms);
        counters.add_named("render/graph/ms", unit::MS, c.graph_ms);
        counters.add_named("render/present/ms", unit::MS, present);
        counters.add_named(RENDER_BUSY, unit::MS, (thread - c.acquire_ms - present).max(0.0));
    }
}
