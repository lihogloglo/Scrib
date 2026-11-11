//! The game's app phases (menus.md "### 1. App phases", slices F1, F3) on `sf_app_bevy`.
//!
//! - **Boot** leaves for `Frontend` on the release start (`ExteriorOptions::start_in_main_menu`)
//!   and for `Loading` on dev runs (`view`, `scene`: no menu, owner 2026-10-26).
//! - **Frontend**: the main menu (`main_menu.rs`) over a plain dark UI camera, and **no world**:
//!   `exterior.rs` builds the world (camera, player anchor, worldspace roots) on
//!   `OnEnter(WorldAlive)`, and the world plugins' public sets (streaming, terrain, far field,
//!   surface weather) only run while [`WorldAlive`] holds.
//! - `Loading` → `InGame` once streaming has the active worldspace's directory and, on the
//!   release start, a load burst ([`Streaming::begin_burst`]) armed after it has ended: R0 and R1
//!   resident (`sf_world` scheduler). Dev runs keep today's budgets (no burst), so they enter
//!   `InGame` as soon as the directory is in. Both then wait on `sf_world_bevy::RevealGate` (W1).
//! - **Loading a save** (Continue, or Load from the front end: a [`LoadingSave`]): the world's
//!   setup runs as for New game, but streaming holds (the `Drive` set doesn't run) and the
//!   `LoadRequest` goes out on entering `Loading`. The load applies through the game's own path
//!   (`mw_saves_bevy`: a blackout whose cut applies it, the character section's
//!   `StartTravel { reload: true }` moving the player anchor to the saved spot in that same
//!   frame), with the blackout's fade-out at zero length: the loading phase is black already.
//!   Once the player is placed ([`TravelArrived`]) streaming starts there and the `InGame` check
//!   above runs for that worldspace; the fade's hold shows the arrival as for a door. A load that
//!   fails (corrupt or missing file) returns to `Frontend` (`warn!`), whose entry tears the
//!   half-built world down (`teardown.rs`) and opens the main menu again.

use std::path::{Path, PathBuf};

use bevy::prelude::*;
use sf_app_bevy::bevy_state::prelude::{NextState, OnEnter, OnExit, in_state};
use sf_app_bevy::{AppPhase, AppPhasePlugin, WorldAlive};
use sf_save_bevy::{LoadDone, LoadRequest};
use sf_world_bevy::{ActiveWorldspace, FadeTransition, RevealAsk, RevealCheck, Streaming, StreamingSystems, TravelArrived};

/// The front end's UI camera (despawned when the front end exits).
#[derive(Component)]
pub struct FrontendCamera;

/// The front end's background until the backdrop (B1): near black.
const FRONTEND_CLEAR: Color = Color::srgb(0.02, 0.02, 0.025);

/// The release start loads the start area in a burst (budgets ×20 behind the menu's exit).
#[derive(Resource)]
struct LoadingBurst;

/// This `Loading` builds the world at a save's place (Continue, Load from the front end):
/// inserted with the switch to `Loading`, removed once the load has placed the player (or has
/// failed). While it exists streaming holds, so nothing loads at the new-game start.
#[derive(Resource, Debug, Clone, PartialEq)]
pub struct LoadingSave {
    path: PathBuf,
    /// The fade's own fade-out, restored once placed (zero meanwhile).
    fade_secs: Option<f32>,
    /// The load has applied (its `LoadDone` was a success).
    applied: bool,
}

impl LoadingSave {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into(), fade_secs: None, applied: false }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

/// The phases and the world gating. `release`: Boot → Frontend (else Boot → Loading).
pub fn add(app: &mut App, release: bool) {
    app.add_plugins(AppPhasePlugin { after_boot: if release { AppPhase::Frontend } else { AppPhase::Loading } })
        .add_message::<LoadRequest>()
        .add_message::<LoadDone>()
        .add_message::<TravelArrived>()
        .configure_sets(Update, (StreamingSystems::Drive, StreamingSystems::Transition).run_if(in_state(WorldAlive)))
        .configure_sets(Update, StreamingSystems::Drive.run_if(not(resource_exists::<LoadingSave>)))
        .configure_sets(PostUpdate, sf_terrain_bevy::TerrainSystems::Update.run_if(in_state(WorldAlive)))
        .configure_sets(PostUpdate, sf_lod_bevy::FarFieldSystems::FarShadow.run_if(in_state(WorldAlive)))
        .configure_sets(PostUpdate, sf_surface_bevy::SurfaceSystems.run_if(in_state(WorldAlive)))
        .add_systems(OnEnter(AppPhase::Loading), request_save_load)
        .add_systems(
            Update,
            (track_save_load, loading_done).chain().after(StreamingSystems::Transition).run_if(in_state(AppPhase::Loading)),
        );
    if release {
        app.insert_resource(LoadingBurst)
            .add_systems(OnEnter(AppPhase::Frontend), spawn_frontend_camera)
            .add_systems(OnExit(AppPhase::Frontend), despawn_frontend_camera);
    }
}

fn spawn_frontend_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("frontend camera"),
        FrontendCamera,
        Camera2d,
        Camera { clear_color: ClearColorConfig::Custom(FRONTEND_CLEAR), ..Camera::default() },
    ));
}

fn despawn_frontend_camera(mut commands: Commands, cameras: Query<Entity, With<FrontendCamera>>) {
    for camera in &cameras {
        commands.entity(camera).despawn();
    }
}

/// Entering `Loading` for a save: the load request goes out at once, the fade-out at zero.
fn request_save_load(loading: Option<ResMut<LoadingSave>>, fade: Option<ResMut<FadeTransition>>, mut loads: MessageWriter<LoadRequest>) {
    let Some(mut loading) = loading else { return };
    if let Some(mut fade) = fade {
        loading.fade_secs = Some(fade.fade_secs);
        fade.fade_secs = 0.0;
    }
    info!("app phase: loading {} (the world builds at the save's place)", loading.path.display());
    loads.write(LoadRequest::path(loading.path.clone()));
}

/// The save's load: placed (the travel arrived, or applied with no travel to make) → streaming
/// runs there; failed → back to the front end.
fn track_save_load(
    mut commands: Commands,
    loading: Option<ResMut<LoadingSave>>,
    mut done: MessageReader<LoadDone>,
    mut arrived: MessageReader<TravelArrived>,
    fade: Option<ResMut<FadeTransition>>,
    mut next: ResMut<NextState<AppPhase>>,
) {
    let Some(mut loading) = loading else {
        done.clear();
        arrived.clear();
        return;
    };
    let mut failed = None;
    for d in done.read() {
        match &d.0 {
            Ok(_) => loading.applied = true,
            Err(e) => failed = Some(e.to_string()),
        }
    }
    let travelled = arrived.read().count() > 0;
    let placed = travelled || (loading.applied && !fade.as_ref().is_some_and(|f| f.is_active()));
    if failed.is_none() && !placed {
        return;
    }
    if let (Some(mut fade), Some(secs)) = (fade, loading.fade_secs) {
        fade.fade_secs = secs;
    }
    commands.remove_resource::<LoadingSave>();
    if let Some(e) = failed {
        warn!("app phase: loading {} failed ({e}); back to the main menu", loading.path.display());
        next.set(AppPhase::Frontend);
    } else {
        info!("app phase: save loaded, streaming at the player's place");
    }
}

/// `Loading` → `InGame`: the active worldspace's directory is in and (release) the burst armed
/// after it has ended, then the reveal gate opens (pipeline-warmup.md §5.4, W1: the start area's
/// textures, prepared assets and pipelines, shadow views included; capped at 10 s with a `warn!`).
/// `armed` resets on the way out, for the next world (F2). A save's load waits for the player's
/// placement first ([`LoadingSave`]); its arrival is the fade's reveal (the fade holds it black).
#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)] // system parameters
fn loading_done(
    mut streaming: ResMut<Streaming>,
    active: Res<ActiveWorldspace>,
    burst: Option<Res<LoadingBurst>>,
    loading_save: Option<Res<LoadingSave>>,
    fade: Option<Res<FadeTransition>>,
    mut reveal: RevealCheck,
    mut next: ResMut<NextState<AppPhase>>,
    mut armed: Local<bool>,
) {
    if loading_save.is_some() || !matches!(*next, NextState::Unchanged) {
        return;
    }
    let resident = start_area_resident(&mut streaming, &active, burst.is_some(), &mut armed);
    let revealed = if fade.is_some_and(|f| f.is_active()) {
        resident
    } else {
        let place = active.0.as_ref().map_or("", |w| w.as_str());
        let ask = RevealAsk { kind: "loading", place, focus: None, cells: resident, gated: false, give_up: false };
        reveal.step(ask).open
    };
    if !revealed {
        return;
    }
    *armed = false;
    info!("app phase: start area resident and revealed, in game");
    next.set(AppPhase::InGame);
}

/// The streaming half of [`loading_done`]: the directory is in and the (armed) burst has ended.
fn start_area_resident(streaming: &mut Streaming, active: &ActiveWorldspace, burst: bool, armed: &mut bool) -> bool {
    if let Some(ws) = active.0.as_ref()
        && !streaming.has_directory(ws)
    {
        return false;
    }
    // Armed only now: before the directory the scheduler has no request to wait for, so a
    // burst begun earlier may already have ended.
    if burst && !*armed {
        streaming.begin_burst();
        *armed = true;
        return false;
    }
    !streaming.in_burst()
}
