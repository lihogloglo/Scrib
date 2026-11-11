//! `FadeTransition`: the M2 scaffold for crossing a door link
//! (streaming.md, "Transition (M2)").
//!
//! Fade out (0.3 s), move the traveler to the arrival point in the target
//! worldspace (reparent under its root, switch the camera's render layer,
//! make it the active worldspace, burst-load), hold at full black until the
//! arrival is ready to show, fade in. Ready is the [`crate::RevealGate`]'s
//! verdict (pipeline-warmup.md §5.4): the near cells around the arrival are
//! spawned with no near load pending (`Scheduler::arrival_ready`), then every
//! mesh around the traveler has its textures, its mesh and material prepared
//! in the render world, and its pipelines (main, shadow, prepass) compiled,
//! so nothing pops in after the fade lifts. [`FadeTransition::max_hold_secs`]
//! caps the hold. Gates ([`FadeTransition::hold`], e.g. a shadow cache
//! capturing the arrival's lamps) extend it, by at most
//! [`crate::RevealGate::gates_max_secs`].
//! [`TravelArrived`] tells the app the traveler moved (to resync camera
//! controllers). Removal path: M8's portal crossing replaces it without
//! changing the data model.
//!
//! A [`StartBlackout`] fades out with no travel of its own (a save load,
//! whose destination is known only once the loaded state applies): at full
//! black the [`FadeCutHooks`] run in one exclusive step, and a
//! [`StartTravel`] they write is executed in the same frame (the cut), so no
//! gameplay frame runs between the hooks and the move / resident reload.
//! Requests that arrive while a fade runs are queued, never dropped (the
//! latest travel wins; a blackout discards travels requested before its cut).
//!
//! The fade itself is a full-window UI node: UI draws after tonemapping,
//! post-processing and upscaling, so it is truly black (a 3D quad before the
//! camera was fogged and composited over by later passes: DR-4).

use std::sync::Arc;

use bevy_asset::{AssetServer, Assets, Handle};
use bevy_camera::Camera;
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
use crate::reveal::{RevealAsk, RevealCheck, RevealGate};
use crate::stream::{ActiveWorldspace, Streaming};
use crate::worldspace::WorldspaceRegistry;

/// Asks the fade scaffold to move `traveler` (a camera or player entity in
/// a worldspace) to `to`, facing `rotation`.
#[derive(Message, Clone, Debug)]
pub struct StartTravel {
    pub traveler: Entity,
    pub to: Location,
    pub rotation: Quat,
    /// At the cut (full black), reload every resident cell
    /// ([`Streaming::reload_resident`]): a save load, whose overlay replaced
    /// the live one. The hold waits for the reload. False for doors.
    pub reload: bool,
}

/// Asks for a fade to black with no travel of its own: at full black the
/// fade runs the [`FadeCutHooks`] (a save load applies there), then executes
/// a [`StartTravel`] they wrote in the same frame, else fades back in.
#[derive(Message, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StartBlackout;

type CutHook = Arc<dyn Fn(&mut World) + Send + Sync>;

/// Exclusive steps run at a [`StartBlackout`]'s cut (full black), in
/// registration order, right before the fade reads the travel they request.
#[derive(Resource, Default, Clone)]
pub struct FadeCutHooks(Vec<CutHook>);

impl FadeCutHooks {
    pub fn add(&mut self, f: impl Fn(&mut World) + Send + Sync + 'static) {
        self.0.push(Arc::new(f));
    }
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
    /// Fading out; `travel` is `None` for a [`StartBlackout`].
    Out { t: f32, travel: Option<StartTravel> },
    /// A blackout at full black: the [`FadeCutHooks`] run this frame, then
    /// the fade executes the travel they requested or fades back in.
    Cut,
    /// At full black, waiting for the arrival (`frames`: since the move;
    /// `ready`: frames and seconds since the cells and textures were ready; `cells_at`: seconds
    /// waited when the cells were first ready, for the `reveal:` line).
    Hold { waited: f32, frames: u32, traveler: Entity, to: Location, ready: Option<(u32, f32)>, cells_at: Option<f32> },
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
    phase: FadePhase,
    overlay: Option<Entity>,
    camera: Option<Entity>,
    /// Seconds from the travel request to the end of the fade-in, last time.
    pub last_duration: Option<f32>,
    /// Seconds the last hold waited at full black.
    pub last_hold: Option<f32>,
    /// Seconds of the last hold that gates added.
    pub last_gate: Option<f32>,
    /// A gate asked to stay black this frame (cleared by the fade).
    gated: bool,
    elapsed: f32,
    /// Requests that arrived while a fade ran (started once it is idle).
    queued_travel: Option<StartTravel>,
    queued_blackout: bool,
}

impl Default for FadeTransition {
    fn default() -> Self {
        Self {
            fade_secs: 0.3,
            max_hold_secs: 10.0,
            ready_margin_m: 16.0,
            phase: FadePhase::Idle,
            overlay: None,
            camera: None,
            last_duration: None,
            last_hold: None,
            last_gate: None,
            gated: false,
            elapsed: 0.0,
            queued_travel: None,
            queued_blackout: false,
        }
    }
}

impl FadeTransition {
    /// Keeps the screen black one more frame (call each frame, before
    /// [`crate::StreamingSystems::Transition`], while something the arrival
    /// needs isn't ready: shadow maps, GPU caches). Honoured only during the
    /// hold, for at most [`crate::RevealGate::gates_max_secs`] after the cells and textures.
    pub fn hold(&mut self) {
        self.gated = true;
    }

    /// At full black, waiting for the arrival.
    pub fn is_holding(&self) -> bool {
        matches!(self.phase, FadePhase::Hold { .. })
    }

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
            FadePhase::Hold { .. } | FadePhase::Cut => 1.0,
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
    /// Where `focus` is (world space), for the reveal gate's render-world probe.
    pub(crate) fn at(&self, focus: Entity) -> Option<[f32; 3]> {
        self.places.get(focus).ok().map(|t| t.translation().to_array())
    }

    pub(crate) fn ready(&self, traveler: Entity, radius: f32) -> bool {
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

/// Runs the [`FadeCutHooks`] at a blackout's cut, right before [`run_fade`]
/// (same frame) reads the travel they request.
pub(crate) fn run_cut_hooks(world: &mut World) {
    if !matches!(world.get_resource::<FadeTransition>().map(FadeTransition::phase), Some(FadePhase::Cut)) {
        return;
    }
    let hooks = world.get_resource::<FadeCutHooks>().map(|h| h.0.clone()).unwrap_or_default();
    for hook in hooks {
        hook(world);
    }
}

/// What the cut (full black) changes for a travel.
#[derive(SystemParam)]
pub(crate) struct Cut<'w, 's> {
    commands: Commands<'w, 's>,
    arrived: MessageWriter<'w, TravelArrived>,
    streaming: ResMut<'w, Streaming>,
    active: ResMut<'w, ActiveWorldspace>,
    registry: ResMut<'w, WorldspaceRegistry>,
    cameras: Query<'w, 's, (), With<Camera>>,
}

impl Cut<'_, '_> {
    /// Moves the traveler into the target worldspace, burst-loads (and
    /// reloads the resident cells for a load), then holds.
    fn travel(&mut self, travel: StartTravel) -> FadePhase {
        let entry = self.registry.ensure(&mut self.commands, &travel.to.worldspace);
        let (cell, mut transform) = to_grid(travel.to.pos);
        transform.rotation = travel.rotation;
        self.commands.entity(travel.traveler).insert((ChildOf(entry.root), cell, transform));
        if self.cameras.contains(travel.traveler) {
            self.commands.entity(travel.traveler).insert(entry.layers().camera());
        }
        self.active.0 = Some(travel.to.worldspace.clone());
        self.streaming.begin_burst();
        if travel.reload {
            self.streaming.reload_resident();
        }
        let (traveler, to) = (travel.traveler, travel.to.clone());
        self.arrived.write(TravelArrived { traveler: travel.traveler, to: travel.to, rotation: travel.rotation });
        FadePhase::Hold { waited: 0.0, frames: 0, traveler, to, ready: None, cells_at: None }
    }
}

pub(crate) fn run_fade(
    mut requests: MessageReader<StartTravel>,
    mut blackouts: MessageReader<StartBlackout>,
    time: Res<Time<Real>>,
    mut fade: ResMut<FadeTransition>,
    mut cut: Cut,
    mut reveal: RevealCheck,
) {
    let dt = time.delta_secs();
    if !fade.is_holding() {
        fade.gated = false;
    }
    let latest = requests.read().last().cloned();
    if blackouts.read().count() > 0 {
        fade.queued_blackout = true;
    }
    if matches!(fade.phase, FadePhase::Cut) {
        // The hooks ran just before (`run_cut_hooks`): their travel moves the traveler now.
        if fade.queued_travel.take().is_some() {
            log::info!("sf_world_bevy: fade: a travel requested before the blackout's cut is dropped");
        }
        fade.phase = match latest {
            Some(travel) => cut.travel(travel),
            None => FadePhase::In { t: 0.0 },
        };
        return;
    }
    if let Some(travel) = latest {
        fade.queued_travel = Some(travel);
    }
    if !fade.is_active() {
        if std::mem::take(&mut fade.queued_blackout) {
            fade.camera = None;
            fade.elapsed = 0.0;
            fade.phase = FadePhase::Out { t: 0.0, travel: None };
        } else if let Some(travel) = fade.queued_travel.take() {
            fade.camera = cut.cameras.contains(travel.traveler).then_some(travel.traveler);
            fade.elapsed = 0.0;
            fade.phase = FadePhase::Out { t: 0.0, travel: Some(travel) };
        }
        return;
    }
    fade.elapsed += dt;
    let fade_secs = fade.fade_secs;
    let next = match std::mem::take(&mut fade.phase) {
        FadePhase::Out { t, travel } if t + dt >= fade_secs => match travel {
            Some(travel) => cut.travel(travel),
            None => FadePhase::Cut,
        },
        FadePhase::Out { t, travel } => FadePhase::Out { t: t + dt, travel },
        FadePhase::Hold { waited, frames, traveler, to, ready: prior, cells_at } => {
            let ws = &to.worldspace;
            let settled = frames >= MIN_HOLD_FRAMES;
            let cells = cut.streaming.has_directory(ws) && cut.streaming.scheduler().arrival_ready(ws, to.pos, fade.ready_margin_m);
            // Burst mode can end before the traveler's anchor reached the
            // destination (it ends when nothing near is pending): restart it.
            if settled && !cells && !cut.streaming.in_burst() {
                cut.streaming.begin_burst();
            }
            let gated = std::mem::take(&mut fade.gated);
            let waited = waited + dt;
            let cells_at = cells_at.or((settled && cells).then_some(waited));
            // The reveal gate (pipeline-warmup.md §5.4): cells, then textures, assets and
            // pipelines (shadow views included) around the traveler, then the gates (shadow
            // captures, ...) up to their cap. Once ready the arrival stays ready (no unloading
            // at black): the gate remembers cells and textures.
            let status = reveal.step(RevealAsk {
                kind: "door",
                place: ws.as_str(),
                focus: Some(traveler),
                cells: settled && cells,
                gated,
                give_up: waited >= fade.max_hold_secs,
            });
            let ready = prior.or(status.cpu_ready.then_some((0, 0.0)));
            if status.open {
                fade.last_hold = Some(waited);
                fade.last_gate = reveal.gate().and_then(RevealGate::last_reveal).and_then(|r| r.terms.last()).and_then(|t| t.secs);
                FadePhase::In { t: 0.0 }
            } else {
                let ready = ready.map(|(n, secs)| (n + 1, secs + dt));
                FadePhase::Hold { waited, frames: frames + 1, traveler, to, ready, cells_at }
            }
        }
        FadePhase::In { t } if t + dt >= fade_secs => {
            fade.last_duration = Some(fade.elapsed);
            FadePhase::Idle
        }
        FadePhase::In { t } => FadePhase::In { t: t + dt },
        FadePhase::Idle => FadePhase::Idle,
        FadePhase::Cut => FadePhase::Cut,
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

