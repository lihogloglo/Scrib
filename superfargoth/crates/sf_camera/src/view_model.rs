//! First-person view model projection (`docs/specs/first-person.md` "View model", FP-2): the
//! arms and held items are redrawn by the main camera with their own vertical FOV and squeezed
//! into the nearest slice of depth, in one render (the UE 5.5 "First Person Rendering" shape).
//!
//! The math the view-model material's two vertex stages run (FP-2b, WESL), written here so it
//! is tested exactly and so the shaders can mirror it line for line:
//!
//! - [`ViewModelProjection::pseudo_world`]: the world position moved so that, seen through the
//!   camera's own `view_from_world`, its view-space x and y are scaled by
//!   [`ViewModelProjection::k`] (`k = tan(fov_world/2) / tan(fov_view/2)`, [`fov_scale`]) and its
//!   view depth is unchanged. Projected by the camera's own `clip_from_world` it lands where a
//!   `fov_view` camera would draw it, with the camera's aspect, near plane and TAA/DLSS jitter.
//!   The prepass outputs it as the vertex's world position (this frame with the view, the
//!   previous frame with the previous view), so Bevy's stock motion-vector fragment
//!   (`unjittered_clip_from_world * world_position`) measures the motion the screen shows.
//! - [`ViewModelProjection::depth_slice`]: clip depth remapped into the nearest slice,
//!   `z' = R w + (1 - R) z` (reverse Z: `ndc' = R + (1 - R) ndc`), so only geometry closer than
//!   [`ViewModelProjection::cover_distance`] can cover the view model; depth order inside it and
//!   the near plane are kept.
//!
//! The lit position stays the real one (the main pass keeps `world_position`); only the clip
//! position, and the prepass's motion-vector inputs, use these.

use glam::{Mat4, Vec3, Vec4};

/// The view-space x/y scale that turns a `fov_world` projection into a `fov_view` one (vertical
/// FOVs, radians): `tan(fov_world/2) / tan(fov_view/2)`. Above 1 when the view model's FOV is
/// narrower (it looks larger). 1 for a FOV outside `(0, π)` or not finite.
pub fn fov_scale(fov_world: f32, fov_view: f32) -> f32 {
    let valid = |f: f32| f.is_finite() && f > 0.0 && f < std::f32::consts::PI;
    if !valid(fov_world) || !valid(fov_view) {
        return 1.0;
    }
    (fov_world * 0.5).tan() / (fov_view * 0.5).tan()
}

/// The view-model projection parameters: the FOV scale and the depth slice.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ViewModelProjection {
    /// View-space x/y scale ([`fov_scale`]).
    pub k: f32,
    /// `R`, the share of reverse-Z depth kept free for the view model, `0 <= R < 1` (0: no slice).
    pub depth: f32,
}

impl ViewModelProjection {
    /// No FOV change, no slice: the copy draws exactly over the world body.
    pub const IDENTITY: Self = Self { k: 1.0, depth: 0.0 };
    /// Largest `R`: at 1 every view-model depth would collapse onto the near plane.
    pub const MAX_DEPTH: f32 = 0.99;

    /// From vertical FOVs in degrees (the settings' unit) and `R` (clamped to
    /// `0..=`[`Self::MAX_DEPTH`]; not finite: 0).
    pub fn new(fov_world_deg: f32, fov_view_deg: f32, depth: f32) -> Self {
        let depth = if depth.is_finite() { depth.clamp(0.0, Self::MAX_DEPTH) } else { 0.0 };
        Self { k: fov_scale(fov_world_deg.to_radians(), fov_view_deg.to_radians()), depth }
    }

    /// The pseudo-world position of `world` for a camera with `view_from_world` (rigid: rotation
    /// and translation, as Bevy's camera views are): `world + (k - 1) (v.x right + v.y up)`, with
    /// `v` the view-space position and `right`/`up` the camera's world axes (rows 0 and 1 of the
    /// view rotation). Equal to `world_from_view * S_k * view_from_world * world` with
    /// `S_k = diag(k, k, 1, 1)`, but needs only `view_from_world`, which Bevy's previous-view
    /// uniforms carry (they have no `world_from_view`).
    ///
    /// WESL mirror (FP-2b):
    /// ```text
    /// let v = view_from_world * vec4(p, 1.0);
    /// let right = vec3(view_from_world[0].x, view_from_world[1].x, view_from_world[2].x);
    /// let up = vec3(view_from_world[0].y, view_from_world[1].y, view_from_world[2].y);
    /// return p + (k - 1.0) * (v.x * right + v.y * up);
    /// ```
    pub fn pseudo_world(&self, world: Vec3, view_from_world: Mat4) -> Vec3 {
        let m = view_from_world;
        let v = m * world.extend(1.0);
        let right = Vec3::new(m.x_axis.x, m.y_axis.x, m.z_axis.x);
        let up = Vec3::new(m.x_axis.y, m.y_axis.y, m.z_axis.y);
        world + (self.k - 1.0) * (v.x * right + v.y * up)
    }

    /// The depth slice on a clip position: `z' = R w + (1 - R) z`, x, y and w kept.
    ///
    /// WESL mirror: `vec4(clip.xy, r * clip.w + (1.0 - r) * clip.z, clip.w)`.
    pub fn depth_slice(&self, clip: Vec4) -> Vec4 {
        let r = self.depth;
        Vec4::new(clip.x, clip.y, r * clip.w + (1.0 - r) * clip.z, clip.w)
    }

    /// The view model's clip position: [`Self::depth_slice`] of `clip_from_world` (the camera's
    /// own, jittered) times [`Self::pseudo_world`]. What both vertex stages write as `position`.
    pub fn clip(&self, world: Vec3, view_from_world: Mat4, clip_from_world: Mat4) -> Vec4 {
        self.depth_slice(clip_from_world * self.pseudo_world(world, view_from_world).extend(1.0))
    }

    /// The view distance under which world geometry can cover the view model for a reverse-Z
    /// projection with `near`: `near / R` (infinite without a slice).
    pub fn cover_distance(&self, near: f32) -> f32 {
        if self.depth > 0.0 { near / self.depth } else { f32::INFINITY }
    }
}

impl Default for ViewModelProjection {
    fn default() -> Self {
        Self::IDENTITY
    }
}
