//! Far sun-cascade hold (ROADMAP 1.22 F3): the hold decision and the padded
//! cascade fit, Bevy-free. The Bevy glue (`sf_sky_bevy::cascade_hold`) writes
//! the held fit back into Bevy's `Cascades` and skips the held cascade's
//! shadow pass.
//!
//! Industry standard: far shadow cascades update at a lower rate than near
//! ones (RAGE, Decima, UE's cached CSM). A held cascade keeps the
//! light-space fit (and so the shadow map) it was last drawn with until the
//! camera moved [`CascadeHoldConfig::move_threshold`] or turned
//! [`CascadeHoldConfig::turn_threshold_deg`], the light turned
//! [`CascadeHoldConfig::sun_threshold_deg`], the map is
//! [`CascadeHoldConfig::max_age`] frames old (casters that move, NPCs in a far
//! cascade, then update at that reduced rate), the slice's shape changed
//! (cascade config, projection, map size), or the current view slice is no
//! longer inside the held box (a safety net, [`CascadeFit::covers`]).
//! SPUD (MIT, github.com/chasmlol/spud @ e5f2de4, `src/engine/app.cpp`
//! `hold_far_cascade`) holds its last cascade with the same 4 m / 3 degree
//! thresholds between world ticks; no code is taken from it.
//!
//! Coverage: a held fit must still contain the view slice of later frames.
//! [`fit_cascade`] grows Bevy's fit by [`hold_pad`] on every side: a corner of
//! the slice moves at most `move + r * turn` (`r` its distance from the
//! camera) while the camera stays within the thresholds, so the padded box
//! holds it. Held cascades use the padded fit on draw frames too, so a
//! release never changes the texel size; the box centre stays snapped to whole
//! texels (as Bevy's), so a refit moves the map by whole texels and static
//! shadows don't swim.
//!
//! [`fit_cascade`] is a port of Bevy 0.20.0's `calculate_cascade`
//! (`bevy_light/src/cascade.rs`, tag v0.20.0, MIT OR Apache-2.0) with the pad
//! added; with `pad == 0` it returns Bevy's fit exactly.

use glam::{Mat4, Quat, Vec3, Vec3A, Vec4};

/// When a far sun cascade keeps last frame's fit and shadow map.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CascadeHoldConfig {
    /// Off: every cascade is fitted and drawn every frame (stock Bevy).
    pub enabled: bool,
    /// The first cascade index that may be held (0 = all, 1 = all but the nearest).
    pub hold_from: usize,
    /// Camera travel that releases a hold, meters.
    pub move_threshold: f32,
    /// Camera turn (any axis) that releases a hold, degrees.
    pub turn_threshold_deg: f32,
    /// Light turn that releases a hold, degrees.
    pub sun_threshold_deg: f32,
    /// The oldest a held map may get, in frames: at this age it is redrawn,
    /// so a far cascade redraws at least once every `max_age` frames.
    /// 0 or 1 never holds.
    pub max_age: u32,
}

impl Default for CascadeHoldConfig {
    fn default() -> Self {
        Self { enabled: true, hold_from: 1, move_threshold: 4.0, turn_threshold_deg: 3.0, sun_threshold_deg: 0.1, max_age: 4 }
    }
}

/// A cascade's light-space fit: what Bevy's `Cascade` holds.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CascadeFit {
    /// The light's view to world matrix, centred on the snapped near plane.
    pub world_from_cascade: Mat4,
    /// The orthographic projection (reverse Z).
    pub clip_from_cascade: Mat4,
    /// World to light clip space.
    pub clip_from_world: Mat4,
    /// Shadow map texel size, meters.
    pub texel_size: f32,
}

impl CascadeFit {
    /// Whether every point is inside this cascade's box (clip `x`, `y` in
    /// [-1, 1], `z` in [0, 1]), with a small tolerance.
    #[must_use]
    pub fn covers(&self, world_points: &[Vec3]) -> bool {
        const EPS: f32 = 1e-4;
        world_points.iter().all(|&p| {
            let c = self.clip_from_world.project_point3(p);
            c.x.abs() <= 1.0 + EPS && c.y.abs() <= 1.0 + EPS && c.z >= -EPS && c.z <= 1.0 + EPS
        })
    }
}

/// The camera and light this frame, for one cascade.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HoldView {
    /// The camera's view to world matrix.
    pub world_from_view: Mat4,
    /// The light's rotation.
    pub light_rotation: Quat,
}

impl HoldView {
    fn position(&self) -> Vec3 {
        self.world_from_view.w_axis.truncate()
    }

    fn rotation(&self) -> Quat {
        self.world_from_view.to_scale_rotation_translation().1
    }
}

/// A cascade's view slice in camera space (Bevy's `get_frustum_corners`
/// order: near plane bottom right, top right, top left, bottom left, then the
/// far plane) and the shadow map size: what the fit's shape depends on.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SliceShape {
    /// Slice corners in camera space.
    pub corners: [Vec3A; 8],
    /// Shadow map size, texels.
    pub map_size: u32,
}

/// What a held cascade was last drawn with.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HoldCapture {
    /// The camera and light at the draw.
    pub view: HoldView,
    /// The slice shape at the draw.
    pub slice: SliceShape,
    /// The fit drawn (and used for shading while held).
    pub fit: CascadeFit,
    /// Frames since the draw.
    pub age: u32,
}

/// Why a held cascade is drawn this frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Redraw {
    /// No earlier draw.
    First,
    /// The slice shape (cascade config, projection) or the map size changed.
    Changed,
    /// The camera travelled the move threshold.
    Moved,
    /// The camera turned the turn threshold.
    Turned,
    /// The light turned the sun threshold.
    SunMoved,
    /// The map reached the max age.
    MaxAge,
    /// The view slice left the held box.
    Uncovered,
}

impl Redraw {
    /// Every reason, in [`decide`]'s order.
    pub const ALL: [Self; 7] =
        [Self::First, Self::Changed, Self::Moved, Self::Turned, Self::SunMoved, Self::MaxAge, Self::Uncovered];

    /// Index into [`Self::ALL`].
    #[must_use]
    pub fn index(self) -> usize {
        Self::ALL.iter().position(|r| *r == self).unwrap_or(0)
    }
}

/// A held cascade's decision this frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HoldDecision {
    /// Keep the last fit and map.
    Hold,
    /// Refit and draw.
    Redraw(Redraw),
}

/// The angle of the rotation from `a` to `b`, radians (precise for small angles).
#[must_use]
pub fn rotation_angle(a: Quat, b: Quat) -> f32 {
    let d = b * a.inverse();
    2.0 * d.xyz().length().atan2(d.w.abs())
}

/// Whether a cascade last drawn as `prev` may keep its map this frame.
#[must_use]
pub fn decide(config: &CascadeHoldConfig, prev: Option<&HoldCapture>, view: &HoldView, slice: &SliceShape) -> HoldDecision {
    let Some(prev) = prev else { return HoldDecision::Redraw(Redraw::First) };
    if prev.slice != *slice {
        return HoldDecision::Redraw(Redraw::Changed);
    }
    if view.position().distance(prev.view.position()) >= config.move_threshold {
        return HoldDecision::Redraw(Redraw::Moved);
    }
    if rotation_angle(prev.view.rotation(), view.rotation()) >= config.turn_threshold_deg.to_radians() {
        return HoldDecision::Redraw(Redraw::Turned);
    }
    if rotation_angle(prev.view.light_rotation, view.light_rotation) >= config.sun_threshold_deg.to_radians() {
        return HoldDecision::Redraw(Redraw::SunMoved);
    }
    if prev.age.saturating_add(1) >= config.max_age {
        return HoldDecision::Redraw(Redraw::MaxAge);
    }
    if !prev.fit.covers(&world_corners(view.world_from_view, &slice.corners)) {
        return HoldDecision::Redraw(Redraw::Uncovered);
    }
    HoldDecision::Hold
}

/// One frame of a held cascade: the decision and the fit to use (and, on a
/// redraw, draw). `state` is the cascade's last capture, updated in place.
pub fn step(config: &CascadeHoldConfig, state: &mut Option<HoldCapture>, view: &HoldView, slice: &SliceShape) -> (CascadeFit, HoldDecision) {
    let decision = decide(config, state.as_ref(), view, slice);
    if let (HoldDecision::Hold, Some(prev)) = (decision, state.as_mut()) {
        prev.age += 1;
        return (prev.fit, decision);
    }
    #[allow(clippy::cast_precision_loss)] // Shadow map sizes are powers of two well under 2^24.
    let fit = fit_cascade(
        &slice.corners,
        slice.map_size as f32,
        Mat4::from_quat(view.light_rotation),
        view.world_from_view,
        hold_pad(config, &slice.corners),
    );
    *state = Some(HoldCapture { view: *view, slice: *slice, fit, age: 0 });
    (fit, decision)
}

/// The margin a held fit gets on every side, meters: the farthest a slice
/// corner moves while the camera stays within the thresholds.
#[must_use]
pub fn hold_pad(config: &CascadeHoldConfig, corners: &[Vec3A; 8]) -> f32 {
    let reach = corners.iter().map(|c| c.length()).fold(0.0, f32::max);
    config.move_threshold.max(0.0) + reach * config.turn_threshold_deg.max(0.0).to_radians()
}

/// The slice corners in world space.
#[must_use]
pub fn world_corners(world_from_view: Mat4, corners: &[Vec3A; 8]) -> [Vec3; 8] {
    corners.map(|c| world_from_view.transform_point3(c.into()))
}

/// Bevy's cascade fit (bounding sphere of the slice, texel-snapped centre)
/// grown by `pad` meters on every side, in light space and depth.
#[must_use]
pub fn fit_cascade(corners: &[Vec3A; 8], map_size: f32, world_from_light: Mat4, world_from_view: Mat4, pad: f32) -> CascadeFit {
    let light_from_camera = world_from_light.transpose() * world_from_view;
    let mut min = Vec3A::splat(f32::MAX);
    let mut max = Vec3A::splat(f32::MIN);
    for &corner in corners {
        let corner_light_view = light_from_camera.transform_point3a(corner);
        min = min.min(corner_light_view);
        max = max.max(corner_light_view);
    }
    min.z -= pad;
    max.z += pad;

    // Bevy: the larger of the slice's body and far-plane diagonals, from the
    // camera-space corners, ceiled to a whole meter for float stability.
    let body_diagonal = (corners[0] - corners[6]).length_squared();
    let far_plane_diagonal = (corners[4] - corners[6]).length_squared();
    let cascade_diameter = (body_diagonal.max(far_plane_diagonal).sqrt() + 2.0 * pad).ceil();
    let cascade_texel_size = cascade_diameter / map_size;
    #[allow(clippy::manual_midpoint)] // Bevy's exact float ops: the pad-0 fit equals Bevy's bit for bit.
    let near_plane_center = Vec3A::new(
        (0.5 * (min.x + max.x) / cascade_texel_size).floor() * cascade_texel_size,
        (0.5 * (min.y + max.y) / cascade_texel_size).floor() * cascade_texel_size,
        max.z,
    );

    let world_from_light_transpose = world_from_light.transpose();
    let cascade_from_world = Mat4::from_cols(
        world_from_light_transpose.x_axis,
        world_from_light_transpose.y_axis,
        world_from_light_transpose.z_axis,
        (-near_plane_center).extend(1.0),
    );
    let world_from_cascade = Mat4::from_cols(
        world_from_light.x_axis,
        world_from_light.y_axis,
        world_from_light.z_axis,
        world_from_light * near_plane_center.extend(1.0),
    );
    let r = (max.z - min.z).recip();
    let clip_from_cascade = Mat4::from_cols(
        Vec4::new(2.0 / cascade_diameter, 0.0, 0.0, 0.0),
        Vec4::new(0.0, 2.0 / cascade_diameter, 0.0, 0.0),
        Vec4::new(0.0, 0.0, r, 0.0),
        Vec4::new(0.0, 0.0, 1.0, 1.0),
    );
    CascadeFit {
        world_from_cascade,
        clip_from_cascade,
        clip_from_world: clip_from_cascade * cascade_from_world,
        texel_size: cascade_texel_size,
    }
}

#[cfg(test)]
#[path = "cascade_hold_tests.rs"]
mod tests;
