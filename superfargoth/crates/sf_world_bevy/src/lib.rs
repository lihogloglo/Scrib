//! Bevy glue for `sf_world`: worldspaces on `big_space`, the streaming
//! stages, placement spawning, and the world-origin shader uniform.
//!
//! Specs: `docs/specs/streaming.md`, `docs/specs/world-coordinates.md`.
//!
//! - Each loaded worldspace is one `big_space` `BigSpace` root with a
//!   `Grid(256 m, 32 m)` and its own `RenderLayers` pair, content and
//!   shadow-only ([`WorldspaceRegistry`], [`WorldspaceLayers`]). Positions convert 1:1 between `WorldPos` and
//!   `CellCoord + Transform` ([`coords`]).
//! - [`WorldStreamingPlugin`] drives `sf_world::Scheduler` every frame from
//!   [`StreamingAnchor`] entities: index fetches on the IO pool, cooked
//!   models through the `AssetServer`, budgeted spawn/despawn on the main
//!   thread, `sf_diag` counters per stage.
//! - [`PlacementSpawners`] are the per-class hooks; the framework spawns
//!   statics as cooked LOD chains with `VisibilityRange`s.
//! - [`FadeTransition`] is the M2 door-crossing scaffold; [`RevealGate`] is what it (and the
//!   app's `Loading` → `InGame`) waits on before showing new content.
//!
//! The app must disable Bevy's `TransformPlugin` (`big_space` propagates
//! transforms) and register the `baked://` asset source before
//! `AssetPlugin` (`sf_model_bevy::baked_source`).

mod catalog_warm;
pub mod coords;
mod compile_pool;
#[cfg(feature = "debug")]
mod debug;
mod fade;
mod far;
pub mod matclass;
pub mod mesh_slabs;
mod origin;
mod precache;
mod precombine;
mod reveal;
mod scene;
mod spawn;
mod stream;
mod worldspace;

pub use big_space;
pub use catalog_warm::{BuiltinFn, BuiltinOutcome, BuiltinSpecimens, CATALOG_TAG, CatalogWarmup, RepresentativeLoader, WarmSpecimen, entry_flags};
pub use coords::{GRID_SWITCH, WorldPosOf, from_grid, to_grid, transform_to_grid, world_grid};
#[cfg(feature = "debug")]
pub use debug::{StreamingDebug, level_color};
pub use fade::{FadeCutHooks, FadePhase, FadeTransition, StartBlackout, StartTravel, TravelArrived};
pub use far::{CellProxies, FarProxies, ProxySpawnCx};
pub use compile_pool::{PipelineCompilePoolPlugin, PipelineCompileThreads, compile_pool_threads, precache_limit};
pub use reveal::{RevealAsk, RevealCheck, RevealGate, RevealHolds, RevealInputs, RevealPending, RevealStatus, RevealStep, SETTLE_FRAMES};
pub use origin::{PHASE_PERIODS, WorldOrigin, WorldOriginBuffer, WorldOriginSystems, WorldOriginUniform};
pub use precache::{
    Precache, PrecacheLimit, PrecachePlugin, PrecacheStatus, Progress, Specimen, SpecimenFlags, pipeline_progress, specimen, specimen_flags, specimen_progress,
    tag_status,
};
pub use precombine::{CellPrecombine, PrecombineBucket, PrecombinePath, Precombines, bucket_ranges};
pub use scene::{CellScene, SceneCellCx, SceneCells};
pub use spawn::{
    ActivateCx, LoadedModel, MaterialMapper, ModelMaterials, ModelTemplate, PlacementSpawner, PlacementSpawners, SubtreeLayers,
    SubtreeRange, SpawnCx, StaticGpuCulling, StaticSpawner, RANGE_STEPS_PER_OCTAVE, shared_range, TemplateLod, apply_subtree_layers, apply_subtree_ranges, chain_ranges, model_aabb, size_culled_end, spawn_template, spawn_template_ranges, standard_material,
    visibility_range,
};
pub use stream::{
    ActiveWorldspace, CellLoaded, CellUnloaded, DoorLinks, PromoteToEntity, RayTracingVariants, StreamCell, StreamStats, Streaming,
    StreamingAnchor, StreamingConfig, WorldIndexSource, WorldOverlay, WorldspaceActivated, WorldspaceRecords,
};
pub use worldspace::{OriginProxy, WorldspaceEntry, WorldspaceLayers, WorldspaceRegistry, WorldspaceRoot, dynamic_layer};

use bevy_app::{App, Last, Plugin, PostStartup, PostUpdate, Update};
use bevy_asset::{AssetApp, Assets};
use bevy_image::Image;
use bevy_ecs::prelude::*;
use bevy_render::batching::gpu_preprocessing::GpuPreprocessingSupport;
use bevy_render::extract_resource::extract_resource;
use bevy_render::render_asset::RenderAssetBytesPerFrame;
use bevy_render::renderer::render_system;
use bevy_render::texture::SharedImageSamplers;
use bevy_render::{ExtractSchedule, Render, RenderApp, RenderSystems};
use big_space::plugin::{BigSpaceCorePlugin, BigSpaceMinimalPlugins, BigSpaceSystems, BigSpaceTimingStatsPlugin};
use big_space::stationary::BigSpaceStationaryPlugin;
use big_space::timing::PropagationStats;
use sf_diag_bevy::DiagCounters;

/// System sets of the streaming runtime.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum StreamingSystems {
    /// `Update`: the scheduler tick and every main-thread stage. Order your
    /// anchor movement before it.
    Drive,
    /// `Update`, after `Drive`: the fade scaffold.
    Transition,
}

/// Adds `big_space` (minimal + stationary + timing plugins, unless already
/// added), the streaming runtime, the fade scaffold, the world-origin
/// uniform, and (feature `debug`) the ring overlay.
#[derive(Default)]
pub struct WorldStreamingPlugin {
    pub config: StreamingConfig,
}

impl Plugin for WorldStreamingPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<BigSpaceCorePlugin>() {
            app.add_plugins((BigSpaceMinimalPlugins, BigSpaceStationaryPlugin, BigSpaceTimingStatsPlugin));
        }
        if !app.is_plugin_added::<sf_model_bevy::SfModelPlugin>() {
            app.add_plugins(sf_model_bevy::SfModelPlugin);
        }
        // The reveal gate's render-world terms (pipeline-warmup.md §5.4), in every build.
        if !app.is_plugin_added::<sf_diag_bevy::PendingDrawsPlugin>() {
            app.add_plugins(sf_diag_bevy::PendingDrawsPlugin);
        }
        if !app.is_plugin_added::<sf_diag_bevy::PendingMaterialPlugin<bevy_pbr::StandardMaterial>>() {
            app.add_plugins(sf_diag_bevy::PendingMaterialPlugin::<bevy_pbr::StandardMaterial>::default());
        }
        // Wide pipeline-compile pool while a reveal holds (pipeline-warmup.md W2b).
        if !app.is_plugin_added::<PipelineCompilePoolPlugin>() {
            app.add_plugins(PipelineCompilePoolPlugin { threads: self.config.pipeline_compile_threads });
        }
        // Pipeline precache: specimens specialised before their first draw (pipeline-warmup.md W3).
        if !app.is_plugin_added::<PrecachePlugin>() {
            app.add_plugins(PrecachePlugin);
        }
        app.init_resource::<WorldspaceRegistry>()
            .init_resource::<FarProxies>()
            .init_resource::<ActiveWorldspace>()
            .init_resource::<DoorLinks>()
            .init_resource::<WorldOverlay>()
            .init_resource::<WorldspaceRecords>()
            .init_resource::<PlacementSpawners>()
            .init_resource::<ModelMaterials>()
            .init_resource::<FadeTransition>()
            .init_resource::<FadeCutHooks>()
            .init_resource::<RevealGate>()
            .init_resource::<RevealHolds>()
            .init_resource::<CatalogWarmup>()
            .init_resource::<BuiltinSpecimens>()
            .init_asset::<CellPrecombine>()
            .insert_resource(Streaming::new(self.config.clone()))
            .insert_resource(StaticGpuCulling(self.config.gpu_cull_statics))
            .add_message::<CellLoaded>()
            .add_message::<CellUnloaded>()
            .add_message::<PromoteToEntity>()
            .add_message::<WorldspaceActivated>()
            .add_message::<StartTravel>()
            .add_message::<StartBlackout>()
            .add_message::<TravelArrived>()
            // The reveal lines (pipeline-warmup.md W0); logged by `sf_diag_bevy::RevealWatchPlugin`.
            .add_message::<sf_diag_bevy::Reveal>()
            .add_plugins(origin::WorldOriginPlugin)
            .configure_sets(Update, (StreamingSystems::Drive, StreamingSystems::Transition).chain())
            .add_systems(Update, stream::drive_streaming.in_set(StreamingSystems::Drive))
            // The catalog warm-up (pipeline-warmup.md W4b1): before the reveal clients read its term.
            .add_systems(
                Update,
                catalog_warm::drive_catalog_warmup.run_if(catalog_warm::warmup_busy).before(StreamingSystems::Drive),
            )
            .add_systems(Update, (fade::run_cut_hooks, fade::run_fade, fade::fade_overlay).chain().in_set(StreamingSystems::Transition))
            .add_systems(PostStartup, worldspace::assign_origins.before(BigSpaceSystems::RecenterLargeTransforms))
            .add_systems(PostUpdate, worldspace::assign_origins.before(BigSpaceSystems::RecenterLargeTransforms))
            // After scene spawning (runtime-imported models), before visibility.
            .add_systems(
                PostUpdate,
                (apply_subtree_ranges, apply_subtree_layers).before(bevy_camera::visibility::VisibilitySystems::CalculateBounds),
            )
            .add_systems(Last, (report_propagation, reveal::end_reveal_frame))
            // Material-class gauges (static-scene.md S2), only while counters run.
            .init_resource::<matclass::StaticMaterialTracker>()
            .init_resource::<matclass::StaticMaterialIds>()
            .add_systems(Last, matclass::track_static_materials.run_if(resource_exists::<DiagCounters>));

        #[cfg(feature = "debug")]
        app.init_resource::<StreamingDebug>();
    }

    fn finish(&self, app: &mut App) {
        // Bevy's per-frame upload limiter stays off (streaming.md, "Upload"): in
        // Bevy 0.20 a mesh instance collected while its mesh or material upload
        // is deferred caches empty offsets or a stale material slot and is never
        // rebuilt, so objects vanish at random. Uploads are bounded by the
        // in-flight asset caps instead; the upload stage still budgets reveals.
        // Removal path: restore `max_bytes = Some(upload_bytes)` once Bevy
        // re-extracts such instances.
        if let Some(mut limit) = app.world_mut().get_resource_mut::<RenderAssetBytesPerFrame>() {
            limit.max_bytes = None;
        }
        // The flat normal map folded statics bind (`matclass::FLAT_NORMAL_MAP`); headless apps
        // without images never draw it.
        if self.config.fold_rules().is_some_and(|r| r.flat_normal_map)
            && let Some(mut images) = app.world_mut().get_resource_mut::<Assets<Image>>()
        {
            images.insert(&matclass::FLAT_NORMAL_MAP, matclass::flat_normal_image()).expect("a uuid handle");
        }
        // GPU-culled statics need GPU culling in every view ([`StaticGpuCulling`]).
        let support = app
            .get_sub_app(RenderApp)
            .and_then(|r| r.world().get_resource::<GpuPreprocessingSupport>())
            .map(GpuPreprocessingSupport::is_culling_supported);
        if self.config.gpu_cull_statics && support == Some(false) {
            log::info!("streaming: no GPU culling on this adapter, statics stay CPU-culled");
            app.insert_resource(StaticGpuCulling(false));
        }
        let counters = app.world().get_resource::<DiagCounters>().cloned();
        if let Some(render_app) = app.get_sub_app_mut(RenderApp) {
            // `--off matclass` keeps Bevy's one sampler per image.
            if self.config.material_classes {
                render_app.init_resource::<SharedImageSamplers>();
            }
            if let Some(counters) = counters {
                render_app
                    .insert_resource(counters)
                    .add_systems(ExtractSchedule, extract_resource::<matclass::StaticMaterialIds, RenderApp, ()>)
                    .add_systems(Render, matclass::count_static_bind_groups.in_set(RenderSystems::Render).after(render_system));
            }
        }
        // The overlay needs gizmos; headless apps don't have them.
        #[cfg(feature = "debug")]
        if app.world().contains_resource::<bevy_gizmos::config::GizmoConfigStore>() {
            app.add_systems(PostUpdate, debug::draw_rings.after(bevy_transform::TransformSystems::Propagate));
        }
    }
}

/// `big_space`'s propagation timings as `sf_diag` counters.
fn report_propagation(stats: Option<Res<PropagationStats>>, counters: Option<Res<DiagCounters>>) {
    let (Some(stats), Some(c)) = (stats, counters) else { return };
    c.add_duration_named("bigspace/high_precision/ms", stats.high_precision_propagation());
    c.add_duration_named("bigspace/low_precision/ms", stats.low_precision_propagation());
    c.add_duration_named("bigspace/total/ms", stats.total());
}

#[cfg(test)]
mod tests;
// Sanity checks for the vendored bevyengine/bevy#25978 port (static-scene.md S3a).
#[cfg(test)]
mod gpu_batch_tests;
// Patch GS change detection: the range cache and allocator generation (static-scene perf fix 2).
#[cfg(test)]
mod gpu_batch_cache_tests;
// Worldspace layer pairs freed and reused when a worldspace streams out (ROADMAP 1.1).
#[cfg(test)]
mod worldspace_tests;
// Pipeline warm-up W2: the vendored `bevy_shader` front-end split and the compile pool size.
#[cfg(test)]
mod warmup_tests;
// Pipeline warm-up W3: the precache's shared key functions (draw path = precache) and status accounting.
#[cfg(test)]
mod precache_tests;
// Pipeline warm-up W4b1: the catalog warm-up requests its specimens; the gate waits on them.
#[cfg(test)]
mod catalog_warm_tests;
