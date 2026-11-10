//! Bevy plugin for the static scene: the baked per-cell instance tables as
//! assets, the retained GPU instance batches and the chain geometry's
//! residency. Spec: `superfargoth/docs/specs/static-scene.md`.
//!
//! Thin by design (ARCHITECTURE "Build performance"): everything that is logic
//! — the table format, the chain registry, the slot allocators, the batch
//! ranges and the CPU twin of the per-view cull — is Bevy-free in `sf_scene`.
//! This crate is the Bevy seam: an asset and its loader, the config resource,
//! and (from B3/B4 on) the residency and render systems.
//!
//! **B1 state: [`ScenePlugin`] registers the asset and its loader and nothing
//! else.** It is in no `add_plugins` call and [`baked_extensions`] is in no
//! extensions map, so the tree ships identically until the game side (C1/C2)
//! installs it. The cells' streaming seam is `sf_world_bevy`'s `CellScene`
//! hook, which this crate will implement in B3; `sf_world_bevy` must never
//! depend on this crate (the hook exists so the dependency goes one way).
//!
//! ```ignore
//! app.register_asset_source(BAKED_SOURCE, baked_source(store, manifest, extensions /* + baked_extensions() */))
//!    .add_plugins(ScenePlugin { config: SceneConfig { enabled: true, ..default() } });
//! ```

mod assets;
#[cfg(test)]
mod tests;

pub use assets::{SceneLoadError, SceneTableAsset, SceneTableLoader};

use bevy_app::{App, Plugin};
use bevy_asset::AssetApp;
use bevy_ecs::prelude::*;
use sf_scene::{CELLS_EXT, CELLS_STEP, CHAINS_EXT, CHAINS_STEP, SpaceConfig};
use sf_world::WorldspaceId;

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

/// The static scene plugin.
#[derive(Default)]
pub struct ScenePlugin {
    pub config: SceneConfig,
}

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.config.clone())
            .init_asset::<SceneTableAsset>()
            .register_asset_loader(SceneTableLoader);
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
