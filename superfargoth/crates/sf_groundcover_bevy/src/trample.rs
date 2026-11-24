//! The trample's CPU twin (slice GC-M3; `shaders/groundcover_common.wesl` `trample_of`, `trample`, `pressed`):
//! mesh groundcover pushed down and aside by the trail ring's press (A, GC-T1; `sf_surface_bevy::view`
//! `surface_press_now` / `surface_press_then`). One press per instance, read at its root (every vertex of a
//! plant agrees): the press and its forward differences one ring texel to +X and +Z, three fetches when pressed
//! (one when not), and the same again from last frame's ring for the prepass's motion vectors.
//!
//! The pressed plant is rotated about its root toward the push direction by `press × trample_max_deg`, after
//! the wind bend, which fades with `1 − press`: a rotation, so a plant lies down without stretching (the
//! usual interactive-foliage bend: a per-instance tilt about the root driven by an interaction map around the
//! camera). The push runs down the press gradient, away from the
//! foot or body; where the press has no slope (a stamp's centre, under a lying body) each plant falls its own
//! way, by its wind phase, blended in at the weight of the press's resolution (`sf_surface::trail::PRESS_FLOOR`),
//! so the direction turns smoothly instead of snapping.

use bevy_math::{Vec2, Vec3};

/// `trample_of`: the trample of an instance from the ring's `slope` at its root (x press 0..1, y the press one
/// texel to +X minus x, z one texel to +Z minus x), scaled by `amount` (`groundcover.render.trample_amount`).
/// Returns x the press 0..1 and yz the unit push direction (XZ), or zero when nothing presses. `phase` is the
/// instance's wind phase 0..1 (its own fall direction where the press is flat) and `floor` the press resolution.
#[must_use]
pub fn push(slope: Vec3, amount: f32, phase: f32, floor: f32) -> Vec3 {
    let p = (slope.x * amount).clamp(0.0, 1.0);
    if p <= 0.0 {
        return Vec3::ZERO;
    }
    let a = phase * std::f32::consts::TAU;
    let v = -Vec2::new(slope.y, slope.z) + Vec2::new(a.cos(), a.sin()) * floor;
    let d = v / v.length().max(1e-6);
    Vec3::new(p, d.x, d.y)
}

/// `trample`: the offset `offset` of a vertex from its instance root, rotated about the root toward the push
/// direction `t.yz` by `t.x × max_rad` (about the horizontal axis up × push; lengths kept).
#[must_use]
pub fn tilt(offset: Vec3, t: Vec3, max_rad: f32) -> Vec3 {
    let (sin, cos) = (t.x * max_rad).sin_cos();
    let axis = Vec3::new(t.z, 0.0, -t.y);
    offset * cos + axis.cross(offset) * sin + axis * (axis.dot(offset) * (1.0 - cos))
}

/// `pressed`: a vertex at `world` (instance root `root`, wind bend `bend`) under the trample `t` ([`push`]):
/// the bend faded by the press, then tilted about the root.
#[must_use]
pub fn pressed(world: Vec3, root: Vec3, bend: Vec3, t: Vec3, max_rad: f32) -> Vec3 {
    let w = world + bend * (1.0 - t.x);
    if t.x <= 0.0 {
        return w;
    }
    root + tilt(w - root, t, max_rad)
}
