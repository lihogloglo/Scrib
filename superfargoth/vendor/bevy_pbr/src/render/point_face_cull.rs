//! superfargoth patch (own: point shadow face culling), ROADMAP 1.25 CW-3b.
//!
//! A shadowed point light renders six cube faces. A face can only matter on screen if a
//! visible receiver samples it: a point `P` samples face `f` when the direction `P - L`
//! falls in that face's 90° pyramid and `P` is within the light's range. So a face whose
//! pyramid (apex at the light, out to its range) does not meet any active 3D camera's
//! frustum within the light's range sphere has no visible receiver: it is not drawn
//! (UE and Doom 2016 cull shadow cube faces this way; `docs/reviews/technique-gap-audit-2026-11-16.md`
//! Part 3 item 3).
//!
//! The test is conservative (it may keep a face that is not seen, never drops one that is):
//! a separating-plane test between two convex hulls, using only their face planes (no
//! edge-edge axes):
//! - the face pyramid, widened by an angular margin so that filter taps and the normal-offset
//!   bias near a cube seam still find a drawn face (`tan(45° + margin)` instead of 1), capped at
//!   the range along its axis (every point of the range sphere in the widened cone lies before
//!   the cap);
//! - the camera frustum with its far plane replaced by one just past the light's sphere (a hull
//!   that holds the frustum's part of the sphere).
//!
//! A face is culled when one camera plane has the whole pyramid outside, or one pyramid plane
//! has the whole clipped frustum outside. Faces follow [`CUBE_MAP_FACES`] order (the shadow
//! views' `subview_index`), axis-aligned in world space like the views `prepare_lights` makes.
//!
//! What the renderer does with the mask: [`crate::extract_shadow_caster_changes`] and
//! [`crate::skip_unchanged_shadow_views`] (`light.rs`): a culled face is not hashed, not drawn,
//! and keeps its last map and keys; a face revealed while dirty is drawn the frame it comes back.

use bevy_camera::primitives::CUBE_MAP_FACES;
use bevy_ecs::prelude::*;
use bevy_math::{Vec3, Vec4};
use bevy_platform::collections::HashSet;
use bevy_render::view::RetainedViewEntity;
use bevy_shape::{HalfSpace, ViewFrustum};

/// Every face of a cube map, as a [`point_shadow_faces_seen`] mask.
pub const ALL_POINT_SHADOW_FACES: u8 = 0b11_1111;

/// Widest margin accepted, degrees (the widened pyramid must stay narrower than a half-space).
pub const MAX_POINT_SHADOW_FACE_MARGIN_DEG: f32 = 30.0;

/// Main-world switch for point shadow face culling. Absent or `enabled: false`: stock
/// behaviour (every face of a shadowed light is a candidate for drawing).
///
/// The game inserts it; the framework's tunable `shadow.lamp.face_cull.margin_deg`
/// (`sf_sky_bevy`) writes `margin_deg`.
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct PointShadowFaceCull {
    /// Cull cube faces no camera can see receivers of.
    pub enabled: bool,
    /// Angular widening of each face's pyramid, degrees, clamped to
    /// `0..=`[`MAX_POINT_SHADOW_FACE_MARGIN_DEG`]: covers the shadow filter's kernel and the
    /// normal-offset bias across a cube seam.
    pub margin_deg: f32,
}

impl Default for PointShadowFaceCull {
    fn default() -> Self {
        Self {
            enabled: false,
            margin_deg: 2.0,
        }
    }
}

/// Render-world counts of point-light cube faces for the frame just decided
/// ([`crate::skip_unchanged_shadow_views`]); bench gauges `views/lamp_face/*` (`sf_diag_bevy`).
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PointShadowFaceStats {
    /// Point and spot shadow views (cube faces and spot views) this frame.
    pub faces: u32,
    /// Faces culled: no visible receiver can sample them.
    pub culled: u32,
    /// Faces back in view this frame that were dirty, drawn at once.
    pub revealed: u32,
    /// Faces drawn this frame.
    pub drawn: u32,
}

/// The faces of a point light at `light` with `range` that some receiver inside one of
/// `frusta` may sample: bit `i` set = face `i` (in [`CUBE_MAP_FACES`] order) is seen.
/// No frusta: every face (nothing known, nothing culled).
pub fn point_shadow_faces_seen(
    light: Vec3,
    range: f32,
    margin_deg: f32,
    frusta: &[ViewFrustum],
) -> u8 {
    if frusta.is_empty() || !range.is_finite() || range <= 0.0 {
        return ALL_POINT_SHADOW_FACES;
    }
    let margin = margin_deg.clamp(0.0, MAX_POINT_SHADOW_FACE_MARGIN_DEG).to_radians();
    let spread = (core::f32::consts::FRAC_PI_4 + margin).tan();
    let mut mask = 0;
    for frustum in frusta {
        mask |= faces_seen_by(light, range, spread, frustum);
        if mask == ALL_POINT_SHADOW_FACES {
            break;
        }
    }
    mask
}

/// A plane usable for culling: finite normal and offset (Bevy marks an absent far plane
/// with a zero normal and an infinite offset).
fn active(half_space: &HalfSpace) -> bool {
    let normal_d: Vec4 = half_space.normal_d();
    normal_d.is_finite() && normal_d.truncate().length_squared() > 0.5
}

fn signed(half_space: &HalfSpace, p: Vec3) -> f32 {
    half_space.normal_d().dot(p.extend(1.0))
}

/// The two axes orthogonal to a cube face's (axis-aligned) direction.
fn tangents(axis: Vec3) -> (Vec3, Vec3) {
    if axis.x != 0.0 {
        (Vec3::Y, Vec3::Z)
    } else if axis.y != 0.0 {
        (Vec3::X, Vec3::Z)
    } else {
        (Vec3::X, Vec3::Y)
    }
}

/// Corners of `frustum` with its far plane moved just past the light sphere (along the view
/// direction, the sphere's farthest point): a convex hull holding the frustum's part of the
/// sphere. `None` if the frustum's planes are degenerate.
fn clipped_corners(light: Vec3, range: f32, frustum: &ViewFrustum) -> Option<[Vec3; 8]> {
    let [left, right, top, bottom, near, _] = frustum.half_spaces;
    if ![left, right, top, bottom, near].iter().all(active) {
        return None;
    }
    let forward = Vec3::from(near.normal());
    let far = HalfSpace::new((-forward).extend(forward.dot(light) + range));
    let clipped = ViewFrustum {
        half_spaces: [left, right, top, bottom, near, far],
    };
    let corners = clipped.corners()?;
    corners.iter().all(|c| c.is_finite()).then_some(corners)
}

fn faces_seen_by(light: Vec3, range: f32, spread: f32, frustum: &ViewFrustum) -> u8 {
    // The range sphere outside one plane: nothing of the light is in this view.
    if frustum
        .half_spaces
        .iter()
        .filter(|h| active(h))
        .any(|h| signed(h, light) < -range)
    {
        return 0;
    }
    let hull = clipped_corners(light, range, frustum);
    let mut mask = 0;
    for (index, face) in CUBE_MAP_FACES.iter().enumerate() {
        let axis = face.target;
        let (u, v) = tangents(axis);
        let cap = light + axis * range;
        let reach = range * spread;
        let pyramid = [
            light,
            cap + (u + v) * reach,
            cap + (u - v) * reach,
            cap + (v - u) * reach,
            cap - (u + v) * reach,
        ];
        // A camera plane with the whole (widened) pyramid outside.
        if frustum
            .half_spaces
            .iter()
            .filter(|h| active(h))
            .any(|h| pyramid.iter().all(|&p| signed(h, p) < 0.0))
        {
            continue;
        }
        // A pyramid plane with the whole clipped frustum outside. Inside the pyramid:
        // `n . (p - light) + offset >= 0` for every (n, offset) below.
        if let Some(hull) = hull {
            let planes = [
                (axis * spread - u, 0.0),
                (axis * spread + u, 0.0),
                (axis * spread - v, 0.0),
                (axis * spread + v, 0.0),
                (-axis, range),
            ];
            if planes
                .iter()
                .any(|&(n, offset)| hull.iter().all(|&p| n.dot(p - light) + offset < 0.0))
            {
                continue;
            }
        }
        mask |= 1 << index;
    }
    mask
}

/// Which shadow views were culled last frame, to tell a face coming back into view.
#[derive(Default)]
pub struct PointShadowFaceCullTracker {
    last: HashSet<RetainedViewEntity>,
    now: HashSet<RetainedViewEntity>,
}

impl PointShadowFaceCullTracker {
    /// Records `view`'s state this frame. Returns `true` when it was culled last frame and
    /// is not now: a revealed face, whose map must be redrawn this frame if it is dirty
    /// (it may hold content from long ago).
    pub fn record(&mut self, view: RetainedViewEntity, culled: bool) -> bool {
        if culled {
            self.now.insert(view);
            false
        } else {
            self.last.contains(&view)
        }
    }

    /// Ends the frame: this frame's culled views become "last frame's".
    pub fn end_frame(&mut self) {
        core::mem::swap(&mut self.last, &mut self.now);
        self.now.clear();
    }
}

/// Whether a face must be drawn this frame because of culling: back in view
/// (`revealed`) while its map is out of date (`dirty`). The light's refresh budget never
/// defers it.
pub fn revealed_face_must_draw(revealed: bool, dirty: bool) -> bool {
    revealed && dirty
}
