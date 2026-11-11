//! Bevy plugin for the static scene: the baked per-cell instance tables as
//! assets, the retained GPU instance batches and the chain geometry's
//! residency. Spec: `superfargoth/docs/specs/static-scene.md`; runtime plan:
//! `superfargoth/docs/specs/scene-runtime-plan.md`.
//!
//! Thin by design (ARCHITECTURE "Build performance"): everything that is logic
//! — the table format, the chain registry, the slot allocators, the batch
//! ranges and the CPU twin of the per-view cull — is Bevy-free in `sf_scene`.
//! This crate is the Bevy seam: an asset and its loader, the config resource,
//! the cell residency ([`residency`]: the `CellScene` hook streaming drives)
//! and the render-world slab feedback ([`slabs`]).
//!
//! **B3b-1 state.** Cells load, resolve their chains and slabs, plan, commit and
//! stream out; `covered` follows the plan; every batch a committed plan names
//! is a `GpuBatchedMesh3d` entity with its range list. No material or GPU
//! write exists yet (B3b-2/B4), so an app with [`ScenePlugin`] enabled draws the
//! covered statics *nowhere*. [`ScenePlugin`] is in no `add_plugins` call and
//! [`baked_extensions`] is in no extensions map, so the tree ships identically
//! until the game side (C1/C2) installs it. `sf_world_bevy` must never depend
//! on this crate (the hook exists so the dependency goes one way).
//!
//! An app that enables the scene must also clear
//! `StreamingConfig::precombine` (`sf_world_bevy::scene` module docs), and
//! needs `sf_model_bevy`'s `SfModelPlugin` (the chain products are `.sfmesh`).
//!
//! ```ignore
//! app.register_asset_source(BAKED_SOURCE, baked_source(store, manifest, extensions /* + baked_extensions() */))
//!    .add_plugins(ScenePlugin { config: SceneConfig { enabled: true, ..default() } });
//! ```

mod assets;
pub mod chains;
pub mod residency;
pub mod slabs;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod residency_tests;

pub use assets::{SceneLoadError, SceneTableAsset, SceneTableLoader};
pub use chains::ChainAssets;
pub use residency::{
    BatchEntity, BatchZeroFill, CellLoad, CellUpload, LoadPhase, SLAB_WAIT_FRAMES, SceneBatch, SceneFrame, SceneHook,
    SceneInbox, SceneSpaces, SceneUploads, SpaceState, despawn_empty_batches, poll_cell_loads, tick_frame,
};
pub use slabs::{ChainSlabTable, extract_chain_slabs};

use std::sync::Arc;

use bevy_app::{App, First, Plugin, Update};
use bevy_asset::AssetApp;
use bevy_ecs::prelude::*;
use bevy_render::{ExtractSchedule, RenderApp};
use sf_scene::{CELLS_EXT, CELLS_STEP, CHAINS_EXT, CHAINS_STEP, SpaceConfig};
use sf_world::WorldspaceId;
use sf_world_bevy::{SceneCells, StreamingSystems};

/// Settings of the static scene (one per app).
///
/// Holds a [`SpaceConfig`] rather than re-declaring its fields: one source of
/// truth for the allocator's granularity, frames in flight and batch capacity,
/// whose defaults `sf_scene` already chose. `SpaceConfig::space` is per space,
/// so the plugin overrides it per worldspace when it builds one (B3).
#[derive(Resource, Clone, Debug, PartialEq, Eq)]
pub struct SceneConfig {
    /// Whether the static scene draws at all. Off by default: until the game
    /// side turns it on, streaming behaves exactly as it did.
    pub enabled: bool,
    /// How each worldspace's space allocates.
    pub space: SpaceConfig,
    /// Worldspaces the scene is enabled for. `None`: all those with a
    /// `scene.cells` product.
    pub worldspaces: Option<Vec<WorldspaceId>>,
    /// Bytes an origin shift may rewrite per frame.
    pub shift_rewrite_bytes: u64,
}

impl Default for SceneConfig {
    fn default() -> Self {
        Self { enabled: false, space: SpaceConfig::default(), worldspaces: None, shift_rewrite_bytes: 1 << 20 }
    }
}

impl SceneConfig {
    /// Whether the scene is enabled for `worldspace`.
    pub fn admits(&self, worldspace: &WorldspaceId) -> bool {
        self.enabled && self.worldspaces.as_ref().is_none_or(|list| list.contains(worldspace))
    }
}

/// System sets of the static scene (B3 brief §4.3).
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SceneSystems {
    /// `First`: [`tick_frame`], before anything else of the scene.
    Frame,
    /// `Update`, after `StreamingSystems::Drive`: [`poll_cell_loads`].
    Residency,
    /// `Update`, after [`Self::Residency`]: promotions (B5; empty until then).
    Promote,
}

/// The static scene plugin.
///
/// With [`SceneConfig::enabled`] it also installs [`SceneHook`] as
/// `sf_world_bevy`'s `SceneCells`; off, streaming never sees the scene.
#[derive(Default)]
pub struct ScenePlugin {
    pub config: SceneConfig,
}

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.config.clone())
            .init_asset::<SceneTableAsset>()
            .register_asset_loader(SceneTableLoader)
            .init_resource::<SceneFrame>()
            .init_resource::<SceneSpaces>()
            .init_resource::<SceneInbox>()
            .init_resource::<ChainSlabTable>()
            .init_resource::<SceneUploads>()
            .configure_sets(
                Update,
                (
                    SceneSystems::Residency.after(StreamingSystems::Drive),
                    SceneSystems::Promote.after(SceneSystems::Residency),
                ),
            )
            .add_systems(First, (tick_frame, despawn_empty_batches).chain().in_set(SceneSystems::Frame))
            .add_systems(Update, poll_cell_loads.in_set(SceneSystems::Residency));
        if self.config.enabled {
            app.insert_resource(SceneCells(Some(Arc::new(SceneHook))));
        }
    }

    fn finish(&self, app: &mut App) {
        // The render world answers which slabs each chain mesh landed in (`slabs` module docs).
        // Without one nothing ever resolves, so cells plan without waiting for slabs.
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else { return };
        render_app.add_systems(ExtractSchedule, extract_chain_slabs);
        app.world_mut().resource_mut::<ChainSlabTable>().tracking = true;
    }
}

/// File extensions of static-scene products for `sf_model_bevy::baked_source`.
///
/// Both steps are here, including `scene.chains` — whose extension is the
/// `sf_model` one, read by the existing `.sfmesh` loader: `BakedReader` strips
/// the extension through this map before the manifest lookup, so without the
/// entry a chain's subject would keep its `.sfmesh` suffix and the lookup would
/// miss. Two steps mapping to one extension is fine (the map is keyed by step).
///
/// (`sf_lod_bevy::baked_extensions` iterates an `EXTENSIONS` table; `sf_scene`
/// has none, so the pairs are built from its four consts.)
pub fn baked_extensions() -> impl Iterator<Item = (String, String)> {
    [(CELLS_STEP, CELLS_EXT), (CHAINS_STEP, CHAINS_EXT)].into_iter().map(|(s, e)| (s.to_owned(), e.to_owned()))
}
