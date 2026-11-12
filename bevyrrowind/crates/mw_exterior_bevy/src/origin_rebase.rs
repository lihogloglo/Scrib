//! FLICKER-1: what a floating-origin shift does to temporal history (streaming.md "Origin shift
//! and temporal history"). `big_space` re-expresses every render-space position relative to the
//! new origin cell; last frame's view and every Bevy-tracked previous transform stay in the old
//! frame, while the terrain prepass, impostors and GPU-batched statics write "previous position
//! = current position" in the new one. Instead of dropping TAA/DLSS history for the whole frame
//! (the old reset: a pop of every temporal effect each 256 m of travel), the camera carries
//! `OriginRebase` (vendored `bevy_render`, `View::origin_rebase`) for that one frame, and those
//! writers add it to their previous position (UE world-origin rebasing: previous view and
//! primitive data are offset by the origin delta, history kept). `--off shiftrebase` keeps the
//! reset.

use bevy::math::{IVec3, Vec3};
use bevy::prelude::Resource;

/// `--off shiftrebase` = false: origin shifts reset temporal history as before FLICKER-1.
#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ShiftRebase(pub bool);

/// What one origin change does to this frame's motion vectors and temporal history.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ShiftHandling {
    /// Same grid, history kept: the camera's `OriginRebase` for this frame.
    Rebase(Vec3),
    /// History dropped (a teleport into another grid, the first frame, or `--off shiftrebase`).
    Reset,
}

/// Render units from the new origin to the old one: render positions are `world - origin`, so a
/// static point's last-frame render position is `(world - O_old) = (world - O_new) + (O_new -
/// O_old)`: `previous_render = current_render + rebase`, `rebase = (new - old) * cell`.
#[must_use]
#[allow(clippy::cast_precision_loss)] // cell deltas are small integers
pub fn rebase(old_cell: IVec3, new_cell: IVec3, cell_edge: f32) -> Vec3 {
    (new_cell - old_cell).as_vec3() * cell_edge
}

/// The handling of an origin change from `old_cell` to `new_cell`. `same_grid` is false when the
/// camera's grid changed with it (another worldspace: a teleport, whose history the door already
/// resets) or on the first frame (no last frame to rebase onto).
#[must_use]
pub fn on_shift(old_cell: IVec3, new_cell: IVec3, cell_edge: f32, same_grid: bool, enabled: ShiftRebase) -> ShiftHandling {
    if same_grid && enabled.0 {
        ShiftHandling::Rebase(rebase(old_cell, new_cell, cell_edge))
    } else {
        ShiftHandling::Reset
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::math::{Mat4, Quat, Vec4};

    const CELL: f32 = 256.0;

    /// Unjittered clip-from-render-world of a camera at absolute `eye` looking along `rot`, the
    /// origin at `origin_cell`.
    fn clip_from_world(eye: Vec3, rot: Quat, origin_cell: IVec3) -> Mat4 {
        let render_eye = eye - origin_cell.as_vec3() * CELL;
        let view_from_world = Mat4::from_rotation_translation(rot, render_eye).inverse();
        // Bevy's infinite reverse-z perspective (bevy_render `ViewUniform::clip_from_view` docs).
        let f = 1.0 / (35f32.to_radians()).tan();
        let clip_from_view = Mat4::from_cols(
            Vec4::new(f / (16.0 / 9.0), 0.0, 0.0, 0.0),
            Vec4::new(0.0, f, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 0.0, -1.0),
            Vec4::new(0.0, 0.0, 0.1, 0.0),
        );
        clip_from_view * view_from_world
    }

    /// CPU twin of the prepass motion vector (`terrain_prepass.wesl` / `impostor_prepass.wesl`
    /// fragment): `(ndc(now) - ndc(prev)) * (0.5, -0.5)`, previous = current + `origin_rebase`.
    fn motion(now: Mat4, prev: Mat4, render_pos: Vec3, origin_rebase: Vec3) -> Vec3 {
        let c = now * render_pos.extend(1.0);
        let p = prev * (render_pos + origin_rebase).extend(1.0);
        let d = (c / c.w - p / p.w) * Vec4::new(0.5, -0.5, 0.0, 0.0);
        d.truncate()
    }

    #[test]
    fn a_static_point_keeps_a_zero_motion_vector_across_a_shift() {
        let rot = Quat::from_rotation_y(0.7) * Quat::from_rotation_x(-0.2);
        let eye = Vec3::new(-20_480.0 + 255.5, 410.0, 33_000.0);
        let (old, new) = (IVec3::new(-81, 1, 128), IVec3::new(-80, 2, 129));
        // The camera did not move in the world: last frame in the old origin, this frame in the new.
        let prev = clip_from_world(eye, rot, old);
        let now = clip_from_world(eye, rot, new);
        let r = on_shift(old, new, CELL, true, ShiftRebase(true));
        let ShiftHandling::Rebase(v) = r else { panic!("{r:?}") };
        assert_eq!(v, Vec3::new(CELL, CELL, CELL));
        // Distant ground in front of the camera, in this frame's render space.
        let world = eye + rot * Vec3::new(30.0, -300.0, -4_000.0);
        let render = world - new.as_vec3() * CELL;
        let mv = motion(now, prev, render, v);
        assert!(mv.length() < 1e-5, "rebased motion {mv}");
        // Without the rebase the same point reads a cell's jump (what forced the old reset).
        let raw = motion(now, prev, render, Vec3::ZERO);
        assert!(raw.length() > 1e-3, "unrebased motion {raw}");
        // No shift: zero rebase, zero motion.
        assert_eq!(rebase(new, new, CELL), Vec3::ZERO);
        assert!(motion(now, now, render, Vec3::ZERO).length() < 1e-6);
    }

    #[test]
    fn the_switch_and_a_grid_change_select_the_reset() {
        let (old, new) = (IVec3::ZERO, IVec3::X);
        assert_eq!(on_shift(old, new, CELL, true, ShiftRebase(false)), ShiftHandling::Reset);
        assert_eq!(on_shift(old, new, CELL, false, ShiftRebase(true)), ShiftHandling::Reset);
        assert_eq!(on_shift(old, new, CELL, true, ShiftRebase(true)), ShiftHandling::Rebase(Vec3::new(CELL, 0.0, 0.0)));
        // Sign: the origin moved a cell -x, so render positions grew a cell +x; last frame's = this - cell.
        assert_eq!(rebase(IVec3::ZERO, -IVec3::X, CELL), Vec3::new(-CELL, 0.0, 0.0));
    }
}
