//! Bevy glue for `sf_world`: worldspaces on `big_space`, the streaming
//! stages, placement spawning, and the world-origin shader uniform.
//!
//! Specs: `docs/specs/streaming.md`, `docs/specs/world-coordinates.md`.
//!
//! - Each loaded worldspace is one `big_space` `BigSpace` root with a
//!   `Grid(256 m, 32 m)` and its own `RenderLayers` layer
//!   ([`WorldspaceRegistry`]). Positions convert 1:1 between `WorldPos` and
//!   `CellCoord + Transform` ([`coords`]).
//! - [`WorldStreamingPlugin`] drives `sf_world::Scheduler` every frame from
//!   [`StreamingAnchor`] entities: index fetches on the IO pool, cooked
//!   models through the `AssetServer`, budgeted spawn/despawn on the main
//!   thread, `sf_diag` counters per stage.
//! - [`PlacementSpawners`] are the per-class hooks; the framework spawns
//!   statics as cooked LOD chains with `VisibilityRange`s.
//! - [`FadeTransition`] is the M2 door-crossing scaffold.
//!
//! The app must disable Bevy's `TransformPlugin` (`big_space` propagates
//! transforms) and register the `baked://` asset source before
//! `AssetPlugin` (`sf_model_bevy::baked_source`).

pub mod coords;
#[cfg(feature = "debug")]
mod debug;
mod fade;
mod far;
mod origin;
mod spawn;
mod stream;
mod worldspace;

pub use big_space;
pub use coords::{GRID_SWITCH, WorldPosOf, from_grid, to_grid, transform_to_grid, world_grid};
#[cfg(feature = "debug")]
pub use debug::{StreamingDebug, level_color};
pub use fade::{FadePhase, FadeTransition, StartTravel, TravelArrived};
pub use far::{CellProxies, FarProxies, ProxySpawnCx};
pub use origin::{PHASE_PERIODS, WorldOrigin, WorldOriginBuffer, WorldOriginUniform};
pub use spawn::{
    ActivateCx, GroundBlend, GroundBlendModels, GroundBlendRule, LoadedModel, MaterialMapper, ModelMaterials, ModelTemplate, PlacementSpawner, PlacementSpawners, SubtreeLayers,
    SubtreeRange, SpawnCx, StaticSpawner, RANGE_STEPS_PER_OCTAVE, shared_range, TemplateLod, apply_subtree_layers, apply_subtree_ranges, chain_ranges, model_aabb, size_culled_end, spawn_template, standard_material,
    visibility_range,
};
pub use stream::{
    ActiveWorldspace, CellLoaded, CellUnloaded, DoorLinks, StreamCell, StreamStats, Streaming, StreamingAnchor,
    StreamingConfig, WorldIndexSource, WorldOverlay, WorldspaceActivated, WorldspaceRecords,
};
pub use worldspace::{OriginProxy, WorldspaceEntry, WorldspaceRegistry, WorldspaceRoot};

use bevy_app::{App, Last, Plugin, PostStartup, PostUpdate, Update};
use bevy_ecs::prelude::*;
use bevy_render::render_asset::RenderAssetBytesPerFrame;
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
        app.init_resource::<WorldspaceRegistry>()
            .init_resource::<FarProxies>()
            .init_resource::<ActiveWorldspace>()
            .init_resource::<DoorLinks>()
            .init_resource::<WorldOverlay>()
            .init_resource::<WorldspaceRecords>()
            .init_resource::<PlacementSpawners>()
            .init_resource::<ModelMaterials>()
            .init_resource::<FadeTransition>()
            .insert_resource(Streaming::new(self.config.clone()))
            .add_message::<CellLoaded>()
            .add_message::<CellUnloaded>()
            .add_message::<WorldspaceActivated>()
            .add_message::<StartTravel>()
            .add_message::<TravelArrived>()
            .add_plugins(origin::WorldOriginPlugin)
            .configure_sets(Update, (StreamingSystems::Drive, StreamingSystems::Transition).chain())
            .add_systems(Update, stream::drive_streaming.in_set(StreamingSystems::Drive))
            .add_systems(Update, (fade::run_fade, fade::fade_overlay).chain().in_set(StreamingSystems::Transition))
            .add_systems(PostStartup, worldspace::assign_origins.before(BigSpaceSystems::RecenterLargeTransforms))
            .add_systems(PostUpdate, worldspace::assign_origins.before(BigSpaceSystems::RecenterLargeTransforms))
            // After scene spawning (runtime-imported models), before visibility.
            .add_systems(
                PostUpdate,
                (apply_subtree_ranges, apply_subtree_layers).before(bevy_camera::visibility::VisibilitySystems::CalculateBounds),
            )
            .add_systems(Last, report_propagation);

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
