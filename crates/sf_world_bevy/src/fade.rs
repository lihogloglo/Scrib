//! `FadeTransition`: the M2 scaffold for crossing a door link
//! (streaming.md, "Transition (M2)").
//!
//! Fade out (0.3 s), move the traveler to the arrival point in the target
//! worldspace (reparent under its root, switch the camera's render layer,
//! make it the active worldspace, burst-load), hold while the destination's
//! R0 and R1 aren't resident, fade in. [`TravelArrived`] tells the app the
//! traveler moved (to resync camera controllers). Removal path: M8's portal crossing
//! replaces it without changing the data model. The fade itself is an
//! unlit black quad parented to the traveler's camera.

use bevy_asset::{Assets, Handle};
use bevy_camera::Camera;
use bevy_camera::visibility::RenderLayers;
use bevy_color::Color;
use bevy_light::NotShadowCaster;
use bevy_ecs::prelude::*;
use bevy_material::AlphaMode;
use bevy_math::Quat;
use bevy_shape::Rectangle;
use bevy_mesh::{Mesh, Mesh3d};
use bevy_pbr::{MeshMaterial3d, StandardMaterial};
use bevy_time::{Real, Time};
use bevy_transform::components::Transform;
use sf_render_order_bevy::DrawBand;
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
    Hold { waited: f32 },
    In { t: f32 },
}

/// The fade state and settings.
#[derive(Resource, Debug)]
pub struct FadeTransition {
    /// Seconds for each of fade-out and fade-in.
    pub fade_secs: f32,
    /// Give up waiting for the destination after this long.
    pub max_hold_secs: f32,
    phase: FadePhase,
    overlay: Option<(Entity, Handle<StandardMaterial>)>,
    camera: Option<Entity>,
    /// Seconds from the travel request to the end of the fade-in, last time.
    pub last_duration: Option<f32>,
    elapsed: f32,
}

impl Default for FadeTransition {
    fn default() -> Self {
        Self { fade_secs: 0.3, max_hold_secs: 10.0, phase: FadePhase::Idle, overlay: None, camera: None, last_duration: None, elapsed: 0.0 }
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

/// Spawns the overlay quad in front of a camera.
fn spawn_overlay(
    commands: &mut Commands,
    camera: Entity,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> (Entity, Handle<StandardMaterial>) {
    let material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.0, 0.0, 0.0, 0.0),
        unlit: true,
        alpha_mode: AlphaMode::Blend,
        ..StandardMaterial::default()
    });
    let entity = commands
        .spawn((
            Mesh3d(meshes.add(Rectangle::new(8.0, 8.0))),
            MeshMaterial3d(material.clone()),
            // Just past the default near plane (0.1 m), facing the camera.
            Transform::from_xyz(0.0, 0.0, -0.2),
            RenderLayers::layer(0),
            // Last of all translucents (over precipitation), casts no shadow.
            DrawBand::Overlay { order: i16::MAX },
            NotShadowCaster,
            ChildOf(camera),
        ))
        .id();
    (entity, material)
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
            arrived.write(TravelArrived { traveler: travel.traveler, to: travel.to, rotation: travel.rotation });
            FadePhase::Hold { waited: 0.0 }
        }
        FadePhase::Out { t, travel } => FadePhase::Out { t: t + dt, travel },
        FadePhase::Hold { waited } => {
            let ready = active.0.as_ref().is_some_and(|w| streaming.has_directory(w)) && !streaming.in_burst();
            if ready || waited + dt >= fade.max_hold_secs {
                FadePhase::In { t: 0.0 }
            } else {
                FadePhase::Hold { waited: waited + dt }
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

/// Shows the fade: spawns the quad on the traveling camera, sets its
/// opacity, removes it when the transition ends.
pub(crate) fn fade_overlay(
    mut commands: Commands,
    mut fade: ResMut<FadeTransition>,
    meshes: Option<ResMut<Assets<Mesh>>>,
    materials: Option<ResMut<Assets<StandardMaterial>>>,
) {
    let alpha = fade.alpha();
    match (fade.is_active(), fade.overlay.clone(), fade.camera) {
        (true, None, Some(camera)) => {
            if let (Some(mut meshes), Some(mut materials)) = (meshes, materials) {
                fade.overlay = Some(spawn_overlay(&mut commands, camera, &mut meshes, &mut materials));
            }
        }
        (true, Some((_, material)), _) => {
            if let Some(mut materials) = materials
                && let Some(mut m) = materials.get_mut(&material)
            {
                m.base_color = Color::srgba(0.0, 0.0, 0.0, alpha);
            }
        }
        (false, Some((entity, _)), _) => {
            commands.entity(entity).try_despawn();
            fade.overlay = None;
        }
        _ => {}
    }
}
