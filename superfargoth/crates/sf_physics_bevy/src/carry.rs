//! Carry, drag and throw for holder entities (physics.md "Carry and throw", P6).
//!
//! A holder's view is the eye looking along −Z, from one of two sources:
//!
//! - **Camera** (no [`TickView`]): the entity's own `CellCoord + Transform`.
//!   Each frame its final view of the frame is recorded with that frame's
//!   virtual clock (below).
//! - **Tick** ([`TickView`], a character): the host writes the character's
//!   eye at the tick (tick pose + eye height + the command's look) every
//!   fixed step, before [`crate::PhysicsSystems::Hands`]; it is recorded with
//!   the step's time, so the hand follows the simulation, never a render
//!   frame (deterministic, and in third person in front of the body, not of
//!   the camera behind it: S2-b). The pick ray and the throw use it too, and
//!   the item is drawn against the two latest tick views blended by the
//!   fixed overstep, as the character's body is drawn.
//!
//! Before each fixed step the carrier's hand moves to the view at the step's
//! time (for a camera, extrapolated past the newest view), after it the
//! item's pose is recorded relative to the hand. A carried item is drawn in
//! holder space against the view of the frame ([`HeldPoses`], written to its
//! transform), which removes the beat between the frame-rate view and the
//! 60 Hz step. Recording camera views and drawing run in `PostUpdate`,
//! [`crate::PhysicsSystems::HeldPoses`]: the host orders its camera rig
//! before that set, or the item lags the view by a frame (S2-a). Input comes
//! in as [`CarryInput`] messages, results go out as [`CarryFeedback`]. A
//! holder that goes away (a despawned character) lets go of what it held.
//!
//! Limits are the host's: it fills [`Holder::carrier`]`.limits` from its
//! adapter (`HolderLimits` from a strength stat).

use std::collections::HashMap;

use bevy_app::{App, FixedUpdate, PostUpdate, Update};
use bevy_ecs::prelude::*;
use bevy_time::{Fixed, Time, Virtual};
use bevy_transform::components::Transform;
use sf_coords::WorldTransform;
use sf_physics::{BodyId, CarryEnd, Carrier, GrabMode, GrabResult, HolderLimits};
use sf_world::WorldspaceId;
use sf_world_bevy::big_space::prelude::CellCoord;
use sf_world_bevy::from_grid;

use crate::{Physics, PhysicsSystems};

/// Something that can carry, drag and throw.
#[derive(Component)]
pub struct Holder {
    pub carrier: Carrier,
    /// The worldspace of the current grab.
    worldspace: Option<WorldspaceId>,
}

impl Holder {
    pub fn new(limits: HolderLimits) -> Self {
        Self { carrier: Carrier::new(limits), worldspace: None }
    }

    /// The holder's speed factor while dragging (1 otherwise).
    pub fn speed_scale(&self) -> f32 {
        self.carrier.speed_scale()
    }
}

/// A holder's view from the simulation (see the module docs): the eye at the
/// current fixed step, written by the host every step before
/// [`crate::PhysicsSystems::Hands`]. `None` until the first step.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct TickView(pub Option<WorldTransform>);

/// What a holder should do.
#[derive(Message, Clone, Copy, Debug)]
pub enum CarryInput {
    /// Grab what is under the crosshair.
    Grab(Entity),
    Drop(Entity),
    /// Throw a carried item (a dragged one is released).
    Throw(Entity),
    /// Turn the carried item (radians about the view's up and right axes).
    Rotate { holder: Entity, yaw: f32, pitch: f32 },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CarryOutcome {
    Grabbed { mode: GrabMode, mass: f32 },
    TooHeavy { mass: f32 },
    Nothing,
    Dropped,
    Thrown { speed: f32 },
    /// Lagged past the break distance (too heavy for the grip, snagged).
    Broke,
    /// The body left the world.
    Lost,
}

/// What happened to a holder's grab.
#[derive(Message, Clone, Copy, Debug)]
pub struct CarryFeedback {
    pub holder: Entity,
    pub outcome: CarryOutcome,
}

/// Holder-space render poses of carried bodies in the active world, this frame.
#[derive(Resource, Default)]
pub struct HeldPoses(pub HashMap<BodyId, WorldTransform>);

pub(crate) fn add(app: &mut App) {
    app.add_message::<CarryInput>()
        .add_message::<CarryFeedback>()
        .init_resource::<HeldPoses>()
        .add_observer(let_go_on_remove)
        .add_systems(FixedUpdate, (drive_hands.in_set(PhysicsSystems::Hands), after_step.after(PhysicsSystems::Step)))
        .add_systems(Update, apply_inputs.in_set(PhysicsSystems::Carry).after(PhysicsSystems::Stream).before(PhysicsSystems::WriteBack))
        .add_systems(
            PostUpdate,
            (record_views, held_poses, crate::write_held, crate::stream::follow_held).chain().in_set(PhysicsSystems::HeldPoses),
        );
}

fn view_of(cell: &CellCoord, t: &Transform) -> WorldTransform {
    WorldTransform { pos: from_grid(cell, t), rotation: t.rotation, scale: bevy_math::Vec3::ONE }
}

#[allow(clippy::needless_pass_by_value)]
fn record_views(time: Res<Time<Virtual>>, mut holders: Query<(&mut Holder, &CellCoord, &Transform), Without<TickView>>) {
    let now = time.elapsed_secs_f64();
    for (mut h, cell, t) in &mut holders {
        h.carrier.record_view(now, view_of(cell, t));
    }
}

#[allow(clippy::needless_pass_by_value)]
fn apply_inputs(
    mut inputs: MessageReader<CarryInput>,
    mut feedback: MessageWriter<CarryFeedback>,
    mut holders: Query<&mut Holder>,
    mut physics: ResMut<Physics>,
) {
    let active = physics.worlds.active().cloned();
    for input in inputs.read() {
        let (CarryInput::Grab(e) | CarryInput::Drop(e) | CarryInput::Throw(e) | CarryInput::Rotate { holder: e, .. }) = *input;
        let Ok(mut h) = holders.get_mut(e) else { continue };
        let ws = if matches!(input, CarryInput::Grab(_)) { active.clone() } else { h.worldspace.clone() };
        let Some(w) = ws.as_ref().and_then(|ws| physics.worlds.get_mut(ws)) else { continue };
        let outcome = match *input {
            CarryInput::Grab(_) => match h.carrier.grab(w) {
                GrabResult::Grabbed { mode, mass, .. } => {
                    h.worldspace = ws;
                    Some(CarryOutcome::Grabbed { mode, mass })
                }
                GrabResult::TooHeavy { mass, .. } => Some(CarryOutcome::TooHeavy { mass }),
                GrabResult::Nothing => Some(CarryOutcome::Nothing),
                GrabResult::Busy => None,
            },
            CarryInput::Drop(_) => h.carrier.drop(w).map(|_| CarryOutcome::Dropped),
            CarryInput::Throw(_) => h.carrier.throw(w).map(|(_, speed)| CarryOutcome::Thrown { speed }),
            CarryInput::Rotate { yaw, pitch, .. } => {
                h.carrier.rotate(w, yaw, pitch);
                None
            }
        };
        if let Some(outcome) = outcome {
            feedback.write(CarryFeedback { holder: e, outcome });
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
fn drive_hands(
    time: Res<Time<Fixed>>,
    mut holders: Query<(Entity, &mut Holder, Option<&TickView>)>,
    mut physics: ResMut<Physics>,
    mut feedback: MessageWriter<CarryFeedback>,
) {
    let now = time.elapsed_secs_f64();
    let active = physics.worlds.active().cloned();
    for (e, mut h, tick) in &mut holders {
        if let Some(&TickView(Some(view))) = tick {
            h.carrier.record_view(now, view);
        }
        let Some(ws) = h.worldspace.clone() else { continue };
        let Some(w) = physics.worlds.get_mut(&ws) else {
            h.worldspace = None;
            continue;
        };
        // Changing worldspace lets go (the body stays in its own world).
        if active.as_ref() != Some(&ws) {
            if h.carrier.drop(w).is_some() {
                feedback.write(CarryFeedback { holder: e, outcome: CarryOutcome::Dropped });
            }
            h.worldspace = None;
            continue;
        }
        if let Some(end) = h.carrier.before_step(w, now) {
            feedback.write(CarryFeedback { holder: e, outcome: outcome_of(end) });
        }
    }
}

/// A holder removed or despawned (a character leaving) lets go: no orphaned hand body or joint.
fn let_go_on_remove(remove: On<Remove<Holder>>, mut holders: Query<&mut Holder>, physics: Option<ResMut<Physics>>) {
    let (Ok(mut h), Some(mut physics)) = (holders.get_mut(remove.entity), physics) else { return };
    let Some(ws) = h.worldspace.take() else { return };
    if let Some(w) = physics.worlds.get_mut(&ws) {
        h.carrier.drop(w);
    }
}

fn outcome_of(end: CarryEnd) -> CarryOutcome {
    match end {
        CarryEnd::Broke(_) => CarryOutcome::Broke,
        CarryEnd::Lost(_) => CarryOutcome::Lost,
    }
}

fn after_step(mut holders: Query<(Entity, &mut Holder)>, mut physics: ResMut<Physics>, mut feedback: MessageWriter<CarryFeedback>) {
    for (e, mut h) in &mut holders {
        let Some(w) = h.worldspace.clone().and_then(|ws| physics.worlds.get_mut(&ws)) else { continue };
        if let Some(end) = h.carrier.after_step(w) {
            feedback.write(CarryFeedback { holder: e, outcome: outcome_of(end) });
        }
    }
}

/// A camera holder's view: its grid pose.
type CameraView = (&'static CellCoord, &'static Transform);

#[allow(clippy::needless_pass_by_value)]
fn held_poses(
    time: Res<Time<Fixed>>,
    holders: Query<(&Holder, Option<CameraView>, Has<TickView>)>,
    physics: Res<Physics>,
    mut poses: ResMut<HeldPoses>,
) {
    poses.0.clear();
    let alpha = time.overstep_fraction();
    // A tick view is drawn between the last two steps' views, like the character.
    let blended = time.elapsed_secs_f64() - time.timestep().as_secs_f64() * f64::from(1.0 - alpha.clamp(0.0, 1.0));
    for (h, camera, tick) in &holders {
        if h.worldspace.is_none() || h.worldspace.as_ref() != physics.worlds.active() {
            continue;
        }
        let view = match (tick, camera) {
            (true, _) => h.carrier.view_at(blended),
            (false, Some((cell, t))) => Some(view_of(cell, t)),
            (false, None) => None,
        };
        if let Some((body, pose)) = view.and_then(|view| h.carrier.render_pose(&view, alpha)) {
            poses.0.insert(body, pose);
        }
    }
}
