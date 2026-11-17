//! Baked tiles into [`NavMeshes`] around one anchor (nav.md "Streaming", `NV4c`): [`NavRingPlugin`]
//! runs [`sf_nav::NavRing`] on a bake store. Each frame it re-plans when [`NavAnchor`] entered
//! another tile, removes the tiles past the unload radius, and reads at most
//! [`NavRingSettings::tiles_per_frame`] queued tiles, nearest first: the `nav.tiles` product
//! (`sf_nav_bake::tile_subject`) from the manifest and store, decoded ([`BakedTile`]) and
//! `add_tile`d. A manifest without the tile means "not baked" (outside the region, or the bake
//! skipped navigation); a zero-byte product is an empty tile. Doorway polygon lists stay unused
//! until the door overlay (nav.md "Doors").
//!
//! **Scaffold `NavRingScaffold`**: one anchor (the host writes [`NavAnchor`], the player in a
//! game), synchronous store reads on the main thread, a per-frame tile cap. Removal path: NV5
//! replaces it with per-anchor, per-portal, budgeted residency on the streaming scheduler.
//!
//! Radii and cap are tunables ([`RING_LOAD_M`], [`RING_UNLOAD_M`], [`RING_TILES_PER_FRAME`]),
//! bound live into [`NavRingSettings`]; they are gameplay (what an NPC can path), never scaled by
//! a graphics preset (nav.md "Streaming").

use std::sync::Arc;
use std::time::Instant;

use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::*;
use bevy_log::{info, warn};
use serde::Serialize;
use sf_bake::{Manifest, Store};
use sf_nav::{BakedTile, NavBackend, NavError, NavRing, RingLayout, RingRadii, TileKey, TileState};
use sf_nav_bake::{TILES, tile_subject};
use sf_tunables::{AnyDef, Def, Lifetime, Net, Scope, Source, Unit, def};
use sf_tunables_bevy::TunablesAppExt;
use sf_world::{Location, WorldspaceId};

use crate::{NavMeshes, NavSystems};

/// `ai.nav.ring.load_m`: [`RingRadii::load_m`]. nav.md "Streaming": the R0 load radius plus
/// 64 m, so an NPC at the edge of the simulated set has mesh around it; not measured.
pub const RING_LOAD_M: Def<f32> = def!(float "ai.nav.ring.load_m" = 256.0)
    .range(16.0, 4096.0)
    .unit(Unit::Meters)
    .help("Navmesh tiles within this distance of the player become resident")
    .source(Source::Guess)
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Live);

/// `ai.nav.ring.unload_m`: [`RingRadii::unload_m`]. nav.md "Streaming": the R0 unload radius
/// plus 64 m (64 m of hysteresis over the load radius); not measured.
pub const RING_UNLOAD_M: Def<f32> = def!(float "ai.nav.ring.unload_m" = 320.0)
    .range(16.0, 4096.0)
    .unit(Unit::Meters)
    .help("Resident navmesh tiles farther than this from the player are dropped (at least the load distance)")
    .source(Source::Guess)
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Live);

/// `ai.nav.ring.tiles_per_frame`: [`NavRingSettings::tiles_per_frame`], the scaffold's budget
/// (a store read, a decode and an `add_tile` each). Not measured: NV5 moves reads to the
/// streaming scheduler's budgets.
pub const RING_TILES_PER_FRAME: Def<u32> = def!(count "ai.nav.ring.tiles_per_frame" = 4)
    .range(1, 256)
    .unit(Unit::Count)
    .help("Navmesh tiles read from the bake and made resident per frame at most")
    .source(Source::Guess)
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Live);

/// This module's tunables.
pub const TUNABLES: &[&dyn AnyDef] = &[&RING_LOAD_M, &RING_UNLOAD_M, &RING_TILES_PER_FRAME];

/// The ring's radii and per-frame cap (bound from the tunables).
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct NavRingSettings {
    pub radii: RingRadii,
    pub tiles_per_frame: u32,
}

impl Default for NavRingSettings {
    fn default() -> Self {
        Self {
            radii: RingRadii {
                load_m: RING_LOAD_M.default_value(),
                unload_m: RING_UNLOAD_M.default_value(),
            },
            tiles_per_frame: RING_TILES_PER_FRAME.default_value(),
        }
    }
}

/// Where tiles are kept resident: the host writes it before [`NavSystems::Residency`] (`None`:
/// no anchor yet, the ring holds what it has).
#[derive(Resource, Clone, Debug, Default, PartialEq)]
pub struct NavAnchor(pub Option<Location>);

/// The baked ring: the bake store and manifest it reads, and its plan.
#[derive(Resource)]
pub struct BakedNavRing {
    store: Store,
    manifest: Arc<Manifest>,
    ring: NavRing,
    loads: u64,
    drops: u64,
    last_ms: f32,
    /// The worldspace of the last settled-ring info line (one line per worldspace entered).
    logged: Option<WorldspaceId>,
}

impl BakedNavRing {
    pub fn new(store: Store, manifest: Arc<Manifest>, layout: RingLayout) -> Self {
        Self {
            store,
            manifest,
            ring: NavRing::new(layout),
            loads: 0,
            drops: 0,
            last_ms: 0.0,
            logged: None,
        }
    }

    pub fn ring(&self) -> &NavRing {
        &self.ring
    }

    pub fn stats(&self) -> RingStats {
        let c = self.ring.counts();
        RingStats {
            resident: c.resident,
            empty: c.empty,
            unbaked: c.unbaked,
            failed: c.failed,
            pending: c.pending,
            loads: self.loads,
            drops: self.drops,
            last_ms: self.last_ms,
        }
    }
}

/// The ring's part of `nav.stats` and the BRP answer.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RingStats {
    pub resident: usize,
    pub empty: usize,
    pub unbaked: usize,
    pub failed: usize,
    pub pending: usize,
    /// Tiles added since start.
    pub loads: u64,
    /// Tiles removed since start.
    pub drops: u64,
    /// Main-thread time of the last frame that loaded or dropped tiles.
    pub last_ms: f32,
}

/// Keeps baked tiles resident around [`NavAnchor`]. Add with [`crate::NavPlugin`] and a
/// [`NavMeshes`] backend whose region origins match `layout`'s.
pub struct NavRingPlugin {
    pub store: Store,
    pub manifest: Arc<Manifest>,
    pub layout: RingLayout,
}

impl Plugin for NavRingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BakedNavRing::new(
            self.store.clone(),
            self.manifest.clone(),
            self.layout.clone(),
        ))
        .init_resource::<NavAnchor>()
        .init_resource::<NavRingSettings>()
        .add_systems(Update, ring_tick.in_set(NavSystems::Residency));
        let _load = app
            .add_tunable(&RING_LOAD_M)
            .bind_resource(|s: &mut NavRingSettings, v| s.radii.load_m = v);
        let _unload = app
            .add_tunable(&RING_UNLOAD_M)
            .bind_resource(|s: &mut NavRingSettings, v| s.radii.unload_m = v);
        let _cap = app
            .add_tunable(&RING_TILES_PER_FRAME)
            .bind_resource(|s: &mut NavRingSettings, v| s.tiles_per_frame = v);
    }
}

fn ring_tick(
    anchor: Res<NavAnchor>,
    settings: Res<NavRingSettings>,
    mut baked: ResMut<BakedNavRing>,
    meshes: Option<ResMut<NavMeshes>>,
) {
    let Some(mut meshes) = meshes else {
        return;
    };
    let started = Instant::now();
    let b = baked.bypass_change_detection();
    let dropped = b.ring.update(anchor.0.as_ref(), settings.radii);
    let next = b
        .ring
        .next(usize::try_from(settings.tiles_per_frame).unwrap_or(usize::MAX));
    if dropped.is_empty() && next.is_empty() {
        return;
    }
    let backend = &mut *meshes.0;
    for k in &dropped {
        backend.remove_tile(k);
    }
    b.drops += dropped.len() as u64;
    for k in next {
        let state = read_tile(&b.store, &b.manifest, backend, &k);
        if state == TileState::Resident {
            b.loads += 1;
        }
        b.ring.settle(k, state);
    }
    b.last_ms = started.elapsed().as_secs_f32() * 1000.0;
    let c = b.ring.counts();
    if let Some(at) = &anchor.0
        && c.pending == 0
        && b.logged.as_ref() != Some(&at.worldspace)
    {
        b.logged = Some(at.worldspace.clone());
        info!(
            "nav ring: {} tiles resident in {} ({} empty, {} unbaked, {} failed)",
            c.resident,
            at.worldspace.as_str(),
            c.empty,
            c.unbaked,
            c.failed
        );
    }
    baked.set_changed();
}

/// Reads one `nav.tiles` product and makes it resident.
fn read_tile(
    store: &Store,
    manifest: &Manifest,
    backend: &mut dyn NavBackend,
    key: &TileKey,
) -> TileState {
    let subject = tile_subject(key);
    let Some(digest) = manifest.get(TILES, &subject) else {
        return TileState::Unbaked;
    };
    let bytes = match store.get(&digest) {
        Ok(b) => b,
        Err(e) => {
            warn!("nav ring: {TILES} {subject}: store read failed: {e}");
            return TileState::Failed;
        }
    };
    let tile = match BakedTile::decode(&bytes) {
        Ok(Some(t)) => t,
        Ok(None) => return TileState::Empty,
        Err(e) => {
            warn!("nav ring: {TILES} {subject}: decode failed: {e}");
            return TileState::Failed;
        }
    };
    match backend.add_tile(key.clone(), &tile.blob) {
        Ok(_) | Err(NavError::TileLoaded(_)) => TileState::Resident,
        Err(e) => {
            warn!("nav ring: {TILES} {subject}: {e}");
            TileState::Failed
        }
    }
}
