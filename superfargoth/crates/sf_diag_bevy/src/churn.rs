//! What changed each frame, as counters for the hitch table: entities
//! (always), asset events (feature `assets`), render pipelines (feature
//! `render`).

use bevy_app::{App, Plugin};
#[cfg(any(feature = "assets", feature = "render"))]
use bevy_ecs::prelude::*;

#[cfg(any(feature = "assets", feature = "render"))]
use crate::DiagCounters;

/// Entity counters, recorded at each frame boundary: the net change over
/// the frame that just ended (spawns minus despawns) and the total.
pub const ENTITIES: &str = "ecs/entities";
pub const ENTITIES_DELTA: &str = "ecs/entities_delta";

/// Asset events of one asset type per frame, as `Sum` counters
/// `assets/<Type>/added`, `/modified`, `/removed` (`Unused` counts as
/// removed). A burst of `modified` materials rebuilds bind groups on the
/// render thread; a burst of `added` meshes or images queues uploads.
#[cfg(feature = "assets")]
pub struct AssetChurnPlugin<A: bevy_asset::Asset>(std::marker::PhantomData<A>);

#[cfg(feature = "assets")]
impl<A: bevy_asset::Asset> Default for AssetChurnPlugin<A> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

#[cfg(feature = "assets")]
impl<A: bevy_asset::Asset> Plugin for AssetChurnPlugin<A> {
    fn build(&self, app: &mut App) {
        app.add_systems(bevy_app::Last, asset_churn::<A>);
    }
}

/// The last path segment of a type name, generics dropped:
/// `bevy_mesh::mesh::Mesh` → `Mesh`,
/// `bevy_pbr::ExtendedMaterial<a::B, c::D>` → `ExtendedMaterial`.
pub fn short_type_name(full: &str) -> &str {
    let base = full.split('<').next().unwrap_or(full);
    base.rsplit("::").next().unwrap_or(base)
}

#[cfg(feature = "assets")]
fn asset_churn<A: bevy_asset::Asset>(
    mut events: MessageReader<bevy_asset::AssetEvent<A>>,
    counters: Option<Res<DiagCounters>>,
    mut names: Local<Option<[String; 3]>>,
) {
    use bevy_asset::AssetEvent;
    let mut n = [0u32; 3];
    for e in events.read() {
        match e {
            AssetEvent::Added { .. } => n[0] += 1,
            AssetEvent::Modified { .. } => n[1] += 1,
            AssetEvent::Removed { .. } | AssetEvent::Unused { .. } => n[2] += 1,
            AssetEvent::LoadedWithDependencies { .. } => {}
        }
    }
    let Some(counters) = counters else { return };
    let names = names.get_or_insert_with(|| {
        let short = short_type_name(std::any::type_name::<A>());
        ["added", "modified", "removed"].map(|k| format!("assets/{short}/{k}"))
    });
    for (name, count) in names.iter().zip(n) {
        if count > 0 {
            counters.add_named(name, sf_diag::unit::COUNT, f64::from(count));
        }
    }
}

/// Render pipelines per frame, from the render world's `PipelineCache`:
/// `pipelines/created` (`Sum`, new pipelines this frame) and
/// `pipelines/waiting` (`Gauge`, queued or compiling). Shader compilation
/// is a classic hitch; these name it. The render world runs a frame
/// behind the main world, which the hitch table's "this frame + the one
/// before" covers.
#[cfg(feature = "render")]
#[derive(Default)]
pub struct PipelineChurnPlugin;

#[cfg(feature = "render")]
impl Plugin for PipelineChurnPlugin {
    fn build(&self, _app: &mut App) {}

    fn finish(&self, app: &mut App) {
        // `DiagCounters` (inserted by `SfDiagPlugin::build`) is one shared
        // registry: the render world reports into the same `Arc`.
        let Some(counters) = app.world().get_resource::<DiagCounters>().cloned() else {
            log::warn!("sf_diag: PipelineChurnPlugin needs SfDiagPlugin first");
            return;
        };
        let Some(render_app) = app.get_sub_app_mut(bevy_render::RenderApp) else {
            return;
        };
        render_app.insert_resource(counters).add_systems(
            bevy_render::Render,
            pipeline_churn.in_set(bevy_render::RenderSystems::Cleanup),
        );
    }
}

#[cfg(feature = "render")]
fn pipeline_churn(
    cache: Res<bevy_render::render_resource::PipelineCache>,
    counters: Res<DiagCounters>,
    mut known: Local<usize>,
) {
    let total = cache.pipelines().count();
    let waiting = cache.waiting_pipelines().count();
    #[allow(clippy::cast_precision_loss)] // pipeline counts are small
    {
        if total > *known {
            counters.add_named(
                "pipelines/created",
                sf_diag::unit::COUNT,
                (total - *known) as f64,
            );
        }
        counters.set_named("pipelines/waiting", sf_diag::unit::COUNT, waiting as f64);
        counters.set_named("pipelines/total", sf_diag::unit::COUNT, total as f64);
    }
    *known = total;
}

/// Adds the churn counters available in this build: entities are always
/// recorded by `SfDiagPlugin`; this adds pipelines (feature `render`).
/// Asset churn is per type: add [`AssetChurnPlugin`] for each.
#[derive(Default)]
pub struct ChurnPlugin;

impl Plugin for ChurnPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "render")]
        app.add_plugins(PipelineChurnPlugin);
        #[cfg(not(feature = "render"))]
        let _ = app;
    }
}
