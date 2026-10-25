//! Carry, drag and throw for holder entities (physics.md "Carry and throw", P6).
//!
//! A [`Holder`] entity's own `CellCoord + Transform` is its view (the eye,
//! looking along −Z: a camera, or later a character's head). Each frame its
//! view is recorded with the virtual clock; before each fixed step the
//! carrier's hand moves to the view at the step's time, after it the item's
//! pose is recorded relative to the hand (views are recorded in
//! `RunFixedMainLoop` before the steps, [`crate::PhysicsSystems::Views`]); a carried item is drawn in holder
//! space ([`HeldPoses`], read by the write-back), which removes the beat
//! between the frame-rate view and the 60 Hz step. Input comes in as
//! [`CarryInput`] messages, results go out as [`CarryFeedback`].
//!
//! Limits are the host's: it fills [`Holder::carrier`]`.limits` from its
//! adapter (`HolderLimits` from a strength stat).

use std::collections::HashMap;

use bevy_app::{App, FixedUpdate, RunFixedMainLoop, RunFixedMainLoopSystems, Update};
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
        .add_systems(FixedUpdate, (drive_hands.before(PhysicsSystems::Step), after_step.after(PhysicsSystems::Step)))
        .add_systems(RunFixedMainLoop, record_views.in_set(PhysicsSystems::Views).in_set(RunFixedMainLoopSystems::BeforeFixedMainLoop))
        .add_systems(
            Update,
            (apply_inputs, held_poses).chain().in_set(PhysicsSystems::Carry).after(PhysicsSystems::Stream).before(PhysicsSystems::WriteBack),
        );
}

fn view_of(cell: &CellCoord, t: &Transform) -> WorldTransform {
    WorldTransform { pos: from_grid(cell, t), rotation: t.rotation, scale: bevy_math::Vec3::ONE }
}

#[allow(clippy::needless_pass_by_value)]
fn record_views(time: Res<Time<Virtual>>, mut holders: Query<(&mut Holder, &CellCoord, &Transform)>) {
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
fn drive_hands(time: Res<Time<Fixed>>, mut holders: Query<(Entity, &mut Holder)>, mut physics: ResMut<Physics>, mut feedback: MessageWriter<CarryFeedback>) {
    let now = time.elapsed_secs_f64();
    let active = physics.worlds.active().cloned();
    for (e, mut h) in &mut holders {
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

#[allow(clippy::needless_pass_by_value)]
fn held_poses(time: Res<Time<Fixed>>, holders: Query<(&Holder, &CellCoord, &Transform)>, physics: Res<Physics>, mut poses: ResMut<HeldPoses>) {
    poses.0.clear();
    let alpha = time.overstep_fraction();
    for (h, cell, t) in &holders {
        if h.worldspace.is_none() || h.worldspace.as_ref() != physics.worlds.active() {
            continue;
        }
        if let Some((body, pose)) = h.carrier.render_pose(&view_of(cell, t), alpha) {
            poses.0.insert(body, pose);
        }
    }
}
