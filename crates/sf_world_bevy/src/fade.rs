//! `FadeTransition`: the M2 scaffold for crossing a door link
//! (streaming.md, "Transition (M2)").
//!
//! Fade out (0.3 s), move the traveler to the arrival point in the target
//! worldspace (reparent under its root, switch the camera's render layer,
//! make it the active worldspace, burst-load), hold at full black until the
//! arrival is ready to show, fade in. Ready means the near cells around the
//! arrival are spawned with no near load pending
//! (`Scheduler::arrival_ready`) and every mesh within
//! [`FadeTransition::ready_radius_m`] of the traveler has its textures (a
//! material whose image is still loading doesn't draw: objects would pop in
//! in front of the player). [`FadeTransition::max_hold_secs`] caps the hold.
//! [`TravelArrived`] tells the app the traveler moved (to resync camera
//! controllers). Removal path: M8's portal crossing replaces it without
//! changing the data model.
//!
//! The fade itself is a full-window UI node: UI draws after tonemapping,
//! post-processing and upscaling, so it is truly black (a 3D quad before the
//! camera was fogged and composited over by later passes: DR-4).

use bevy_asset::{AssetServer, Assets, Handle};
use bevy_camera::Camera;
use bevy_camera::visibility::RenderLayers;
use bevy_color::Color;
use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use bevy_image::Image;
use bevy_math::Quat;
use bevy_pbr::{MeshMaterial3d, StandardMaterial};
use bevy_time::{Real, Time};
use bevy_transform::components::GlobalTransform;
use bevy_ui::{BackgroundColor, GlobalZIndex, Node, PositionType, UiTargetCamera, Val};
use sf_world::Location;

use crate::coords::to_grid;
use crate::stream::{ActiveWorldspace, Streaming};
use crate::worldspace::WorldspaceRegistry;

/// Asks the fade scaffold to move `traveler` (a camera or player entity in
/// a worldspace) to `to`, facing `rotation`.
#[derive(Message, Clone, Debug)]
pub struct StartTravel {
    pub traveler: Entity,
    pub to: Location,
    pub rotation: Quat,
}

/// Written the frame the fade moves the traveler (at full black, before
/// the hold): anything caching the traveler's pose, such as a camera
/// controller's yaw and pitch, resyncs from it.
#[derive(Message, Clone, Debug)]
pub struct TravelArrived {
    pub traveler: Entity,
    pub to: Location,
    pub rotation: Quat,
}

/// Where a transition is.
#[derive(Clone, Debug, Default)]
pub enum FadePhase {
    #[default]
    Idle,
    Out { t: f32, travel: StartTravel },
    /// At full black, waiting for the arrival (`frames`: since the move).
    Hold { waited: f32, frames: u32, traveler: Entity, to: Location },
    In { t: f32 },
}

/// Frames the hold lasts at least: the move's commands apply and the
/// traveler's transform propagates before the readiness checks trust it.
const MIN_HOLD_FRAMES: u32 = 3;

/// The fade state and settings.
#[derive(Resource, Debug)]
pub struct FadeTransition {
    /// Seconds for each of fade-out and fade-in.
    pub fade_secs: f32,
    /// Give up waiting for the destination after this long.
    pub max_hold_secs: f32,
    /// Half size of the square around the arrival whose content cells must
    /// be spawned, meters.
    pub ready_margin_m: f32,
    /// Meshes this close to the traveler must have their textures, meters.
    pub ready_radius_m: f32,
    phase: FadePhase,
    overlay: Option<Entity>,
    camera: Option<Entity>,
    /// Seconds from the travel request to the end of the fade-in, last time.
    pub last_duration: Option<f32>,
    /// Seconds the last hold waited at full black.
    pub last_hold: Option<f32>,
    elapsed: f32,
}

impl Default for FadeTransition {
    fn default() -> Self {
        Self {
            fade_secs: 0.3,
            max_hold_secs: 10.0,
            ready_margin_m: 16.0,
            ready_radius_m: 48.0,
            phase: FadePhase::Idle,
            overlay: None,
            camera: None,
            last_duration: None,
            last_hold: None,
            elapsed: 0.0,
        }
    }
}

impl FadeTransition {
    pub fn phase(&self) -> &FadePhase {
        &self.phase
    }

    pub fn is_active(&self) -> bool {
        !matches!(self.phase, FadePhase::Idle)
    }

    /// Overlay opacity, 0 (clear) to 1 (black).
    pub fn alpha(&self) -> f32 {
        let f = self.fade_secs.max(1e-3);
        match &self.phase {
            FadePhase::Idle => 0.0,
            FadePhase::Out { t, .. } => (t / f).min(1.0),
            FadePhase::Hold { .. } => 1.0,
            FadePhase::In { t } => 1.0 - (t / f).min(1.0),
        }
    }
}

/// Whether the meshes around the traveler can draw: each material's
/// textures are loaded (or failed: they never will be).
#[derive(SystemParam)]
pub(crate) struct NearTextures<'w, 's> {
    meshes: Query<'w, 's, (&'static GlobalTransform, &'static MeshMaterial3d<StandardMaterial>)>,
    places: Query<'w, 's, &'static GlobalTransform>,
    materials: Option<Res<'w, Assets<StandardMaterial>>>,
    images: Option<Res<'w, Assets<Image>>>,
    server: Option<Res<'w, AssetServer>>,
}

impl NearTextures<'_, '_> {
    fn ready(&self, traveler: Entity, radius: f32) -> bool {
        let (Some(materials), Some(images)) = (&self.materials, &self.images) else { return true };
        let Ok(at) = self.places.get(traveler) else { return true };
        let at = at.translation();
        let r2 = radius * radius;
        let loaded = |h: &Option<Handle<Image>>| {
            h.as_ref().is_none_or(|h| {
                images.contains(h.id())
                    || self.server.as_ref().is_some_and(|s| s.load_state(h.id()).is_failed())
            })
        };
        self.meshes.iter().filter(|(t, _)| t.translation().distance_squared(at) <= r2).all(|(_, m)| {
            materials.get(&m.0).is_none_or(|m| {
                loaded(&m.base_color_texture)
                    && loaded(&m.normal_map_texture)
                    && loaded(&m.emissive_texture)
                    && loaded(&m.metallic_roughness_texture)
                    && loaded(&m.occlusion_texture)
            })
        })
    }
}

#[allow(clippy::too_many_arguments)] // a system's parameters
pub(crate) fn run_fade(
    mut commands: Commands,
    mut requests: MessageReader<StartTravel>,
    mut arrived: MessageWriter<TravelArrived>,
    time: Res<Time<Real>>,
    mut fade: ResMut<FadeTransition>,
    mut streaming: ResMut<Streaming>,
    mut active: ResMut<ActiveWorldspace>,
    mut registry: ResMut<WorldspaceRegistry>,
    cameras: Query<(), With<Camera>>,
    near: NearTextures,
) {
    let dt = time.delta_secs();
    let latest = requests.read().last().cloned();
    if let (FadePhase::Idle, Some(travel)) = (&fade.phase, latest) {
        fade.camera = cameras.contains(travel.traveler).then_some(travel.traveler);
        fade.elapsed = 0.0;
        fade.phase = FadePhase::Out { t: 0.0, travel };
        return;
    }
    if !fade.is_active() {
        return;
    }
    fade.elapsed += dt;
    let fade_secs = fade.fade_secs;
    let next = match std::mem::take(&mut fade.phase) {
        FadePhase::Out { t, travel } if t + dt >= fade_secs => {
            let entry = registry.ensure(&mut commands, &travel.to.worldspace);
            let (cell, mut transform) = to_grid(travel.to.pos);
            transform.rotation = travel.rotation;
            commands.entity(travel.traveler).insert((ChildOf(entry.root), cell, transform));
            if cameras.contains(travel.traveler) {
                commands.entity(travel.traveler).insert(RenderLayers::from_layers(&[0, entry.layer]));
            }
            active.0 = Some(travel.to.worldspace.clone());
            streaming.begin_burst();
            let (traveler, to) = (travel.traveler, travel.to.clone());
            arrived.write(TravelArrived { traveler: travel.traveler, to: travel.to, rotation: travel.rotation });
            FadePhase::Hold { waited: 0.0, frames: 0, traveler, to }
        }
        FadePhase::Out { t, travel } => FadePhase::Out { t: t + dt, travel },
        FadePhase::Hold { waited, frames, traveler, to } => {
            let ws = &to.worldspace;
            let settled = frames >= MIN_HOLD_FRAMES;
            let cells = streaming.has_directory(ws) && streaming.scheduler().arrival_ready(ws, to.pos, fade.ready_margin_m);
            // Burst mode can end before the traveler's anchor reached the
            // destination (it ends when nothing near is pending): restart it.
            if settled && !cells && !streaming.in_burst() {
                streaming.begin_burst();
            }
            let ready = settled && cells && near.ready(traveler, fade.ready_radius_m);
            let waited = waited + dt;
            if ready || waited >= fade.max_hold_secs {
                if !ready {
                    log::warn!("sf_world_bevy: fade: {} not ready after {waited:.1} s, fading in", ws.as_str());
                }
                fade.last_hold = Some(waited);
                FadePhase::In { t: 0.0 }
            } else {
                FadePhase::Hold { waited, frames: frames + 1, traveler, to }
            }
        }
        FadePhase::In { t } if t + dt >= fade_secs => {
            fade.last_duration = Some(fade.elapsed);
            FadePhase::Idle
        }
        FadePhase::In { t } => FadePhase::In { t: t + dt },
        FadePhase::Idle => FadePhase::Idle,
    };
    fade.phase = next;
}

/// Shows the fade: spawns the full-window node over the traveling camera's
/// view (the default UI camera when the traveler isn't one), sets its
/// opacity, removes it when the transition ends.
pub(crate) fn fade_overlay(mut commands: Commands, mut fade: ResMut<FadeTransition>, mut nodes: Query<&mut BackgroundColor>) {
    let color = Color::srgba(0.0, 0.0, 0.0, fade.alpha());
    match (fade.is_active(), fade.overlay) {
        (true, None) => {
            let mut e = commands.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    top: Val::Px(0.0),
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..Node::default()
                },
                BackgroundColor(color),
                // Over every other UI (HUD, menus, debug text).
                GlobalZIndex(i32::MAX),
            ));
            if let Some(camera) = fade.camera {
                e.insert(UiTargetCamera(camera));
            }
            fade.overlay = Some(e.id());
        }
        (true, Some(entity)) => {
            if let Ok(mut bg) = nodes.get_mut(entity) {
                bg.0 = color;
            }
        }
        (false, Some(entity)) => {
            commands.entity(entity).try_despawn();
            fade.overlay = None;
        }
        (false, None) => {}
    }
}
