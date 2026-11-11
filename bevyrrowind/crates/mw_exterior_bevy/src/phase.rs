//! The game's app phases (menus.md "### 1. App phases", slice F1) on `sf_app_bevy`.
//!
//! - **Boot** leaves for `Frontend` on the release start (`ExteriorOptions::start_in_main_menu`)
//!   and for `Loading` on dev runs (`view`, `scene`: no menu, owner 2026-10-26).
//! - **Frontend**: the main menu (`main_menu.rs`) over a plain dark UI camera, and **no world**:
//!   `exterior.rs` builds the world (camera, player anchor, worldspace roots) on
//!   `OnEnter(WorldAlive)`, and the world plugins' public sets (streaming, terrain, far field,
//!   surface weather) only run while [`WorldAlive`] holds.
//! - `Loading` → `InGame` once streaming has the start worldspace's directory and, on the
//!   release start, a load burst ([`Streaming::begin_burst`]) armed after it has ended: R0 and R1
//!   resident (`sf_world` scheduler). Dev runs keep today's budgets (no burst), so they enter
//!   `InGame` as soon as the directory is in.

use bevy::prelude::*;
use sf_app_bevy::bevy_state::prelude::{NextState, OnEnter, OnExit, in_state};
use sf_app_bevy::{AppPhase, AppPhasePlugin, WorldAlive};
use sf_world_bevy::{ActiveWorldspace, Streaming, StreamingSystems};

/// The front end's UI camera (despawned when the front end exits).
#[derive(Component)]
pub struct FrontendCamera;

/// The front end's background until the backdrop (B1): near black.
const FRONTEND_CLEAR: Color = Color::srgb(0.02, 0.02, 0.025);

/// The release start loads the start area in a burst (budgets ×20 behind the menu's exit).
#[derive(Resource)]
struct LoadingBurst;

/// The phases and the world gating. `release`: Boot → Frontend (else Boot → Loading).
pub fn add(app: &mut App, release: bool) {
    app.add_plugins(AppPhasePlugin { after_boot: if release { AppPhase::Frontend } else { AppPhase::Loading } })
        .configure_sets(Update, (StreamingSystems::Drive, StreamingSystems::Transition).run_if(in_state(WorldAlive)))
        .configure_sets(PostUpdate, sf_terrain_bevy::TerrainSystems::Update.run_if(in_state(WorldAlive)))
        .configure_sets(PostUpdate, sf_lod_bevy::FarFieldSystems::FarShadow.run_if(in_state(WorldAlive)))
        .configure_sets(PostUpdate, sf_surface_bevy::SurfaceSystems.run_if(in_state(WorldAlive)))
        .add_systems(Update, loading_done.after(StreamingSystems::Drive).run_if(in_state(AppPhase::Loading)));
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

/// `Loading` → `InGame`: the start worldspace's directory is in and (release) the burst armed
/// after it has ended. `armed` resets on the way out, for the next world (F2).
#[allow(clippy::needless_pass_by_value)] // system parameters
fn loading_done(
    mut streaming: ResMut<Streaming>,
    active: Res<ActiveWorldspace>,
    burst: Option<Res<LoadingBurst>>,
    mut next: ResMut<NextState<AppPhase>>,
    mut armed: Local<bool>,
) {
    if let Some(ws) = active.0.as_ref()
        && !streaming.has_directory(ws)
    {
        return;
    }
    // Armed only now: before the directory the scheduler has no request to wait for, so a
    // burst begun earlier may already have ended.
    if burst.is_some() && !*armed {
        streaming.begin_burst();
        *armed = true;
        return;
    }
    if streaming.in_burst() {
        return;
    }
    *armed = false;
    info!("app phase: start area resident, in game");
    next.set(AppPhase::InGame);
}
