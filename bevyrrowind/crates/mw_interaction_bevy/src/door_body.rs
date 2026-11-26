//! A swing door's body and visuals follow its leaf (crate docs, "Doors").
//!
//! The shell (`sf_interaction_bevy::Doors`) publishes the leaf's world frame; this module puts the
//! door's kinematic body there and hangs the visuals on that body:
//!
//! - **Bodies** ([`move_bodies`], `FixedUpdate`, after `DoorSystems::Commit`, before
//!   `PhysicsSystems::Step`): each `DoorMoved` of a resident swing door whose body is in sets the
//!   body's kinematic target to the leaf pose (a swing step: the step sweeps it there and pushes
//!   characters) or, with `snap`, teleports it (a restored rest state).
//! - **Late bodies** ([`pose_new_bodies`], `Update`, after `PhysicsSystems::Stream`): a body the
//!   physics drain inserted since the last frame is teleported to the leaf pose when the door is
//!   not at its closed frame (a rest restored before the body existed).
//! - **Visuals** ([`follow_pivots`], `Update`, after the late bodies): each resident swing door's
//!   [`DoorPivot`] takes its body's pose interpolated by the fixed step's overstep (as item pivots
//!   do); without a body (physics off, or not inserted yet) the shell's leaf now. A pivot's
//!   `Transform` is written only when it changed.
//!
//! The leaf pose keeps the placement's full-precision position ([`leaf_pose`]): the f32 leaf frame
//! only gives the hinge's offset and the rotation. Every physics resource is optional: without
//! physics the bodies' systems do nothing and the pivots follow the shell.

use bevy_ecs::prelude::*;
use bevy_time::{Fixed, Time};
use bevy_transform::components::Transform;
use sf_coords::WorldTransform;
use sf_interaction::Frame;
use sf_interaction_bevy::{DoorMoved, Doors};
use sf_physics_bevy::{Physics, PhysicsStreaming};

use crate::door::{DoorPivot, DoorPivots, SwingDoors};

/// The body pose of a door closed at `closed` (full precision) whose shell frame is `object` and
/// whose leaf is at `leaf`: the closed position moved by the leaf's offset from the object, the
/// leaf's rotation, the placement's scale.
#[must_use]
pub fn leaf_pose(closed: &WorldTransform, object: &Frame, leaf: &Frame) -> WorldTransform {
    WorldTransform { pos: closed.pos.offset(leaf.translation - object.translation), rotation: leaf.rotation, scale: closed.scale }
}

/// Each `DoorMoved` of a resident swing door moves its body (module docs).
#[allow(clippy::needless_pass_by_value)]
pub(crate) fn move_bodies(
    mut moved: MessageReader<DoorMoved>,
    swing: Res<SwingDoors>,
    doors: Res<Doors>,
    streaming: Option<Res<PhysicsStreaming>>,
    physics: Option<ResMut<Physics>>,
) {
    let (Some(streaming), Some(mut physics)) = (streaming, physics) else { return };
    for m in moved.read() {
        let (Some(door), Some(shell)) = (swing.get(m.placement), doors.get(m.placement)) else { continue };
        let Some(body) = streaming.body(&door.worldspace, m.placement) else { continue };
        let Some(world) = physics.world_mut(&door.worldspace) else { continue };
        let pose = leaf_pose(&door.transform, &shell.object, &m.leaf);
        if m.snap {
            world.set_pose(body, pose);
        } else {
            world.set_kinematic_target(body, pose);
        }
    }
}

/// A body inserted since the last frame is put at the door's leaf (module docs).
#[allow(clippy::needless_pass_by_value)]
pub(crate) fn pose_new_bodies(
    mut swing: ResMut<SwingDoors>,
    doors: Res<Doors>,
    streaming: Option<Res<PhysicsStreaming>>,
    physics: Option<ResMut<Physics>>,
) {
    let (Some(streaming), Some(mut physics)) = (streaming, physics) else { return };
    for (id, door) in swing.bypass_change_detection().iter_mut() {
        let body = streaming.body(&door.worldspace, id);
        if body == door.posed {
            continue;
        }
        let Some(body) = body else {
            door.posed = None;
            continue;
        };
        // Not registered by the shell yet: retried next frame.
        let Some(shell) = doors.get(id) else { continue };
        door.posed = Some(body);
        let leaf = shell.leaf();
        if leaf == shell.object {
            continue;
        }
        if let Some(world) = physics.world_mut(&door.worldspace) {
            world.set_pose(body, leaf_pose(&door.transform, &shell.object, &leaf));
        }
    }
}

/// Each resident swing door's pivot follows its body, else the shell's leaf (module docs).
#[allow(clippy::needless_pass_by_value)]
pub(crate) fn follow_pivots(
    swing: Res<SwingDoors>,
    doors: Res<Doors>,
    map: Res<DoorPivots>,
    streaming: Option<Res<PhysicsStreaming>>,
    physics: Option<Res<Physics>>,
    time: Option<Res<Time<Fixed>>>,
    mut pivots: Query<(&DoorPivot, &mut Transform)>,
) {
    let alpha = time.map_or(1.0, |t| t.overstep_fraction());
    for (id, door) in swing.iter() {
        let Some(entity) = map.get(id) else { continue };
        let Ok((pivot, mut t)) = pivots.get_mut(entity) else { continue };
        let body = streaming.as_ref().and_then(|s| s.body(&door.worldspace, id));
        let world = physics.as_ref().and_then(|p| p.world(&door.worldspace));
        let pose = match (body, world) {
            (Some(body), Some(world)) => world.pose_at(body, alpha),
            _ => doors.get(id).map(|shell| leaf_pose(&door.transform, &shell.object, &shell.leaf())),
        };
        let Some(pose) = pose else { continue };
        let translation = pivot.origin.delta_f32(pose.pos);
        if t.translation != translation || t.rotation != pose.rotation {
            t.translation = translation;
            t.rotation = pose.rotation;
        }
    }
}
