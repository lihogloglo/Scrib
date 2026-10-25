//! Readiness for the viewers and agents: the interior viewer's load flag ([`Loading`], moved from
//! `view.rs`), the exterior's "everything settled" check ([`settled`], moved from `exterior.rs`, which
//! still calls it for `--use-door` and the capture gate) and its on-demand system parameter
//! ([`Readiness`], read by [`crate::remote`]). App-split slice 11 (`docs/specs/app-split.md` cycle 4).

use std::time::Instant;

use bevy_asset::{AssetServer, Assets, LoadState};
use bevy_ecs::prelude::*;
use bevy_pbr::StandardMaterial;
use bevy_world_serialization::WorldAssetRoot;
use mw_game_core::perf::{Off, Part};
use mw_world::ids;
use sf_terrain_bevy::TerrainStats;
use sf_world_bevy::{ActiveWorldspace, Streaming};

/// When everything (models, textures, cooked spawns) finished loading (the interior viewer).
#[derive(Resource)]
pub struct Loading {
    pub started: Instant,
    pub ready_at: Option<f32>,
    /// Assets still pending when `ready_at` was set (non-zero: the wait gave up).
    pub pending: usize,
}

impl Loading {
    /// Everything loaded (or the wait gave up).
    #[must_use]
    pub const fn ready(&self) -> bool {
        self.ready_at.is_some()
    }
}

/// Whether streaming, terrain (in the exterior), NIF models and textures
/// have all settled.
#[must_use]
pub fn settled(
    streaming: &Streaming,
    terrain: &TerrainStats,
    server: &AssetServer,
    nifs: &Query<&WorldAssetRoot>,
    materials: &Assets<StandardMaterial>,
    off: &Off,
    active: &ActiveWorldspace,
) -> bool {
    let loading = |id: bevy_asset::UntypedAssetId| matches!(server.get_load_state(id), Some(LoadState::Loading));
    let terrain_ok = off.has(Part::Terrain)
        || active.0.as_ref() != Some(&ids::exterior())
        || (terrain.height_levels == 10 && terrain.loads_in_flight == 0 && terrain.upload_bytes == 0);
    // The start's own index must be in (before it, nothing is requested yet).
    let directory = active.0.as_ref().is_some_and(|w| streaming.has_directory(w));
    directory
        && streaming.is_idle()
        && terrain_ok
        && nifs.iter().all(|r| {
            server.is_loaded_with_dependencies(&r.0) || matches!(server.get_load_state(&r.0), Some(LoadState::Failed(_)))
        })
        && materials.iter().all(|(_, m)| {
            [&m.base_color_texture, &m.emissive_texture].into_iter().flatten().all(|h| !loading(h.id().untyped()))
        })
}

/// [`settled`] as a system parameter, for runs that ask on demand ([`crate::remote`]).
#[cfg(feature = "remote")]
#[derive(bevy_ecs::system::SystemParam)]
pub struct Readiness<'w, 's> {
    streaming: Option<Res<'w, Streaming>>,
    terrain: Option<Res<'w, TerrainStats>>,
    server: Res<'w, AssetServer>,
    nifs: Query<'w, 's, &'static WorldAssetRoot>,
    materials: Res<'w, Assets<StandardMaterial>>,
    off: Option<Res<'w, Off>>,
    active: Option<Res<'w, ActiveWorldspace>>,
}

#[cfg(feature = "remote")]
impl Readiness<'_, '_> {
    /// `None` outside the exterior viewer.
    #[must_use]
    pub fn settled(&self) -> Option<bool> {
        let (Some(streaming), Some(terrain), Some(off), Some(active)) = (&self.streaming, &self.terrain, &self.off, &self.active) else {
            return None;
        };
        Some(settled(streaming, terrain, &self.server, &self.nifs, &self.materials, off, active))
    }
}
