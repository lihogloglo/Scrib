//! Pipeline warm-up W2b (`docs/specs/pipeline-warmup.md` §5.7, W2): a wide pipeline-compile pool used
//! while the [`RevealGate`] holds.
//!
//! Bevy creates pipelines on `AsyncComputeTaskPool` (25 % of the cores, at most 4 threads), shared
//! with every other async job; on Vulkan the driver compile (`create_*_pipeline`) is ~93 % of a
//! pipeline's cost and runs in parallel, so a door's or a load's burst queues behind 4 threads (W0:
//! queue wait ~15x the compile time on a Balmora walk). While a hold covers the screen (fade at black,
//! `Loading`), new pipeline tasks go to a pool with all cores but the main and render threads
//! (vendored `bevy_render` patch `own, W2b`); in play they stay on the 4 async threads, so a late
//! compile never competes with the frame's own workers. UE5 does the same with its PSO precache tasks
//! (background workers, all cores during loading screens).

use bevy_app::{App, Last, Plugin};
use bevy_ecs::prelude::*;
use bevy_render::render_resource::{install_pipeline_compile_pool, pipeline_compile_pool_installed, set_pipeline_compile_boost};
use bevy_tasks::TaskPoolBuilder;

use crate::{Precache, PrecacheLimit, RevealGate};

/// Installs the compile pool and boosts pipeline tasks onto it while the reveal gate holds.
/// `WorldStreamingPlugin` adds it with [`crate::StreamingConfig::pipeline_compile_threads`].
#[derive(Debug, Clone, Copy, Default)]
pub struct PipelineCompilePoolPlugin {
    /// Pool threads: `None` = [`compile_pool_threads`] of the machine; `Some(0)` = no pool (stock).
    pub threads: Option<usize>,
}

/// The default pool size: every logical core but the main and render threads, never under Bevy's
/// own 4 async-compute threads.
#[must_use]
pub fn compile_pool_threads(available: usize) -> usize {
    available.saturating_sub(2).max(4)
}

impl Plugin for PipelineCompilePoolPlugin {
    fn build(&self, app: &mut App) {
        let threads = self.threads.unwrap_or_else(|| compile_pool_threads(bevy_tasks::available_parallelism()));
        app.insert_resource(PipelineCompileThreads { pool: threads }).add_systems(Last, limit_precache);
        if threads == 0 {
            log::info!("sf_world_bevy: pipeline compile pool off (stock: async-compute threads)");
            return;
        }
        // Process-wide: a second app (tests) reuses the first pool.
        if !pipeline_compile_pool_installed() {
            let pool = TaskPoolBuilder::new().num_threads(threads).thread_name("Pipeline Compile".into()).build();
            if install_pipeline_compile_pool(pool) {
                log::info!("sf_world_bevy: pipeline compile pool {threads} threads while a reveal holds");
            }
        }
        app.add_systems(Last, boost_while_holding);
    }
}

/// The compile threads a reveal's pipelines get: the pool's (0: no pool, Bevy's async-compute threads).
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub struct PipelineCompileThreads {
    pub pool: usize,
}

impl PipelineCompileThreads {
    /// Threads pipeline tasks run on now: the pool while a reveal holds, else `async_threads`.
    #[must_use]
    pub fn now(&self, holding: bool, async_threads: usize) -> usize {
        if holding && self.pool > 0 { self.pool } else { async_threads }
    }
}

/// The precache's limits (W4b1-throughput; UE5 compiles precache PSOs at low priority, never ahead
/// of the ones a draw needs, and does not cap them when nothing else waits). While a reveal holds:
/// every compile thread while no draw-path pipeline is queued or compiling (never unlimited, so a
/// draw arriving later waits for one task to finish, not behind the whole catalog), half of them
/// while one is, so a visible draw's pipeline always finds a free thread. In play: one, so
/// background precaching never takes more than one of Bevy's async-compute threads from the
/// frame's late compiles.
#[must_use]
pub fn precache_limit(threads: usize, holding: bool) -> PrecacheLimit {
    if holding {
        PrecacheLimit { idle: threads.max(1), draw_waiting: (threads / 2).max(1) }
    } else {
        PrecacheLimit::uniform(1)
    }
}

/// Sends the limits to the precache each frame (`Last`, after the clients stepped the gate).
fn limit_precache(gate: Option<Res<RevealGate>>, threads: Res<PipelineCompileThreads>, precache: Option<Res<Precache>>) {
    let Some(precache) = precache else { return };
    let holding = gate.is_some_and(|g| g.is_holding());
    let async_threads = bevy_tasks::AsyncComputeTaskPool::try_get().map_or(4, |pool| pool.thread_num());
    precache.set_limits(Some(precache_limit(threads.now(holding, async_threads), holding)));
}

/// Pipeline tasks spawned while a reveal holds go to the compile pool.
fn boost_while_holding(gate: Res<RevealGate>, mut boosted: Local<bool>) {
    let holding = gate.is_holding();
    if holding != *boosted {
        set_pipeline_compile_boost(holding);
        *boosted = holding;
    }
}
