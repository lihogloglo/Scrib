//! Deformer capture (surface-weather.md "S1 deformer capture"; slice S1a): the CPU twin of what
//! the trail ring pass does with the capture (`sf_surface_bevy` `trail.wgsl`), and the capture's
//! placement, so the GPU math is tested exactly here.
//!
//! - **Capture** ([`Capture`]): an orthographic depth view looking *up* at the deformers' meshes
//!   (whole legs, bodies, creatures, held items) from the slab's bottom, the Arkham Origins method.
//!   Reverse Z: depth 1 at the slab bottom (the view's near plane, and everything below it, by
//!   depth clamp), 0 at its top and in cleared texels, so the stock "greater wins" depth test
//!   keeps each texel's **lowest** point. The window is world-stable: its first texel is a whole
//!   world texel ([`capture_min`]), so a texel holds the same world square whatever the camera.
//! - **Ceiling** (the ring's B channel): per ring texel, how far below the cover's highest
//!   possible surface the deformer's bottom reaches ([`convert`]): `c = clamp(b - H, -H, 0)` with
//!   `b` the bottom's height over the ground and `H` = [`ceiling_ref`] (the ring depth plus its
//!   lip, the highest a raised cover can stand), so 0 means "no deformer here" and the ceiling is
//!   `H + c` meters over the ground ([`ceiling`]). It merges with Min ([`merge`]: order-free,
//!   so stamps and captures compose in any order) and refills like the prints ([`ceiling_update`]).
//! - **Composition** (S1a open point 2): the raised cover never stands above the ceiling,
//!   applied *after* the bury ([`capped_raise`]): `min(raise, ceiling)`. The spec's first sketch
//!   wrote the depression into R (`R = min(R, b - depth)`), which the raise scales by the bury
//!   (`bury × (d + R)`): under a partial bury (or a channel shallower than the ring) a leg above
//!   the snow would still carve. A ceiling taken after the bury is exact for any bury and any
//!   channel depth, and foot stamps keep R and their meaning unchanged.
//! - **Ground** ([`GroundGrid`]): the convert needs each texel's ground height; the host gives a
//!   heightfield sampler and the plugin samples it on a coarse world-snapped grid (bilinear here
//!   and in the shader).
//! - **Sweep gaps** ([`sweep_gap`]): a fast mover at a low frame rate travels farther than its
//!   width between captures; past a threshold the host stamps feet along the segment.
//!
//! Presentation only (design rule "net-ready"): the capture never feeds gameplay.

use glam::camera::rh::{proj::directx::orthographic, view::look_to_mat4};
use glam::{DVec2, IVec2, Mat4, UVec2, Vec2, Vec3};

use crate::trail::LIP_HEIGHT;

/// The capture target's edge (texels): its allocation, and the largest window the convert reads
/// (`window_m / texel_m` is clamped to it; 32 m at the ring's 6.25 cm texel).
// tunables: const (a capacity: the target's allocation, the tunables pick what part is used)
pub const MAX_CAPTURE_SIZE: u32 = 512;

/// The ground grid's largest edge (samples): its storage buffer's capacity.
// tunables: const (a capacity: the window and step tunables pick what part is used)
pub const MAX_GROUND_EDGE: u32 = 160;

/// The capture's edge in texels for a window of `window_m` at `texel_m`.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn capture_size(window_m: f32, texel_m: f64) -> u32 {
    if texel_m <= 0.0 || window_m <= 0.0 {
        return 1;
    }
    ((f64::from(window_m) / texel_m).round() as u32).clamp(1, MAX_CAPTURE_SIZE)
}

/// The first world texel (index of `texel_m` squares, X and Z) of a `size`-texel window centred
/// on `center_xz` (absolute world meters): whole world texels, the same for every centre in one
/// texel.
#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
pub fn capture_min(center_xz: DVec2, texel_m: f64, size: u32) -> IVec2 {
    let t = (center_xz / texel_m).floor();
    IVec2::new(t.x as i32, t.y as i32) - IVec2::splat((size / 2) as i32)
}

/// The highest a raised cover of ring depth `ring_depth` stands over the ground: the depth plus
/// its lip (`trail::LIP_HEIGHT`). B = 0 means a ceiling here: above every raise, i.e. none.
pub fn ceiling_ref(ring_depth: f32) -> f32 {
    ring_depth.max(0.0) * (1.0 + LIP_HEIGHT)
}

/// The capture's depth `d` (reverse Z: 1 at the slab bottom `near` (render Y), 0 at `near +
/// range` and in cleared texels) over ground `ground` (render Y) as the ring's B value (≤ 0;
/// 0 = no deformer). A bottom below the ground carves to the ground; one at or above the cover's
/// highest surface carves nothing.
pub fn convert(d: f32, near: f32, range: f32, ground: f32, ring_depth: f32) -> f32 {
    let h = ceiling_ref(ring_depth);
    if d <= 0.0 || h <= 0.0 {
        return 0.0;
    }
    let bottom = near + (1.0 - d.min(1.0)) * range;
    (bottom - ground - h).clamp(-h, 0.0)
}

/// The ceiling (m over the ground) a B value `c` stands for.
pub fn ceiling(c: f32, ring_depth: f32) -> f32 {
    let h = ceiling_ref(ring_depth);
    h + c.clamp(-h, 0.0)
}

/// The merge (Min): commutative, associative, 0 the identity.
pub fn merge(a: f32, b: f32) -> f32 {
    a.min(b)
}

/// One texel's B in the ring pass: last frame's (0 if it entered the ring), refilled by
/// `refill_m` like the prints, merged with this frame's capture value.
pub fn ceiling_update(prev: f32, refill_m: f32, captured: f32) -> f32 {
    merge((prev + refill_m).min(0.0), captured)
}

/// The raise (m; `trail::cover_raise`, summed over the channels) under the ceiling `c` (ring
/// depth `ring_depth`; 0: no trail, uncapped). `layer.wesl` `surface_raise_of`.
pub fn capped_raise(raise: f32, c: f32, ring_depth: f32) -> f32 {
    if ring_depth <= 0.0 { raise } else { raise.min(ceiling(c, ring_depth)) }
}

/// The trail height (m) the *shading* cuts the cover by (`layer.wesl` `surface_trail_cut`): the
/// print's ([`crate::trail::trail_height`]) or, lower, the ceiling's below the cover's surface,
/// which the shading takes as `d × amount` (it doesn't know the bury); never below `-d`.
pub fn shade_height(v: [f32; 3], d: f32, ring_depth: f32, amount: f32) -> f32 {
    let h = crate::trail::trail_height([v[0], v[1]], d, ring_depth);
    h.min(ceiling(v[2], ring_depth) - d * amount.clamp(0.0, 1.0)).max(-d)
}

/// Texels (≥ 0) between two captures of an actor of width `width_m` that travelled `travel_m`:
/// the trench is continuous while it travels less than its width per capture.
pub fn sweep_gap(speed_m_s: f32, dt_s: f32, width_m: f32, texel_m: f32) -> f32 {
    if texel_m <= 0.0 {
        return 0.0;
    }
    (speed_m_s.abs() * dt_s.max(0.0) - width_m.max(0.0)).max(0.0) / texel_m
}

/// The ground under the capture: heights (render Y) on a world-snapped grid, row-major by Z then
/// X, sampled bilinearly (clamped at the edge).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GroundGrid {
    /// Render X, Z of sample (0, 0).
    pub corner: Vec2,
    pub step_m: f32,
    pub edge: u32,
    pub heights: Vec<f32>,
}

impl GroundGrid {
    /// The grid's first sample (world index of `step_m` squares) and edge for a capture window
    /// whose min corner is `window_min_xz` (absolute world m) and edge `window_m`: one sample of
    /// margin each side, capped at [`MAX_GROUND_EDGE`].
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn placement(window_min_xz: DVec2, window_m: f64, step_m: f64) -> (IVec2, u32) {
        let first = (window_min_xz / step_m).floor() - DVec2::ONE;
        let edge = ((window_m / step_m).ceil() as u32 + 3).min(MAX_GROUND_EDGE);
        (IVec2::new(first.x as i32, first.y as i32), edge)
    }

    /// A flat grid (the fallback: a plane through the player's ground).
    pub fn flat(corner: Vec2, step_m: f32, edge: u32, height: f32) -> Self {
        Self { corner, step_m, edge, heights: vec![height; (edge * edge) as usize] }
    }

    /// The ground at render XZ `point`.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]
    pub fn at(&self, point: Vec2) -> f32 {
        if self.edge == 0 || self.heights.is_empty() || self.step_m <= 0.0 {
            return 0.0;
        }
        let last = (self.edge - 1) as f32;
        let grid = ((point - self.corner) / self.step_m).clamp(Vec2::ZERO, Vec2::splat(last));
        let cell = grid.floor().min(Vec2::splat((last - 1.0).max(0.0)));
        let frac = grid - cell;
        let (col, row) = (cell.x as u32, cell.y as u32);
        let sample = |dx: u32, dz: u32| {
            let (sx, sz) = ((col + dx).min(self.edge - 1), (row + dz).min(self.edge - 1));
            self.heights[(sz * self.edge + sx) as usize]
        };
        let near = sample(0, 0) + (sample(1, 0) - sample(0, 0)) * frac.x;
        let far = sample(0, 1) + (sample(1, 1) - sample(0, 1)) * frac.x;
        near + (far - near) * frac.y
    }

    /// The lowest and highest sample.
    pub fn range(&self) -> (f32, f32) {
        self.heights.iter().fold((f32::INFINITY, f32::NEG_INFINITY), |(lo, hi), &h| (lo.min(h), hi.max(h)))
    }
}

/// One frame's capture, as the ring pass reads it (render space).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Capture {
    /// Render X, Z of the capture's first texel's min corner.
    pub corner: Vec2,
    pub texel_m: f32,
    /// The window's edge in texels (≤ [`MAX_CAPTURE_SIZE`]): the target's first `size` columns
    /// and rows ([`Capture::view`]).
    pub size: u32,
    /// Render Y of the slab's bottom (the view's eye and near plane: depth 1).
    pub near_y: f32,
    /// The slab's height (m; depth 0 at `near_y + range_m`).
    pub range_m: f32,
    pub ground: GroundGrid,
}

impl Capture {
    /// The view looking up from the slab's bottom at the window's centre: `(view_from_world,
    /// clip_from_world)`, orthographic, reverse Z. Its right is +X and its up +Z, so target
    /// column x runs along +X and row y against Z ([`Capture::texel_of`]).
    ///
    /// The projection spans the whole [`MAX_CAPTURE_SIZE`]² target, the window's `size`² texels
    /// in its first columns and rows: Bevy's shadow pass, which draws the capture, sets no
    /// viewport (vendored `bevy_pbr` `view_shadow_pass`; `ExtractedView::viewport` isn't read
    /// there), so clip space always covers the full attachment. trail-offset (2026-11-17): a
    /// projection over the window alone drew a 16 m window (256² of 512²) at twice its scale, and
    /// only its (-X, +Z) quarter landed where the ring reads, 8 m over in +X and -Z: trails off
    /// the player and patches beside them.
    #[allow(clippy::cast_precision_loss)]
    pub fn view(&self) -> (Mat4, Mat4) {
        let h = self.size as f32 * self.texel_m * 0.5;
        let edge = MAX_CAPTURE_SIZE as f32 * self.texel_m;
        let eye = Vec3::new(self.corner.x + h, self.near_y, self.corner.y + h);
        let view = look_to_mat4(eye, Vec3::Y, Vec3::Z);
        // Column 0 at the window's -X edge, row 0 at its +Z edge; the target's rest lies past +X
        // and -Z, outside the window (nothing reads it).
        let proj = orthographic(-h, edge - h, h - edge, h, self.range_m, 0.0);
        (view, proj * view)
    }

    /// The target texel holding render XZ `q`, if inside.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]
    pub fn texel_of(&self, q: Vec2) -> Option<UVec2> {
        if self.texel_m <= 0.0 {
            return None;
        }
        let t = ((q - self.corner) / self.texel_m).floor();
        let n = self.size as f32;
        if t.x < 0.0 || t.y < 0.0 || t.x >= n || t.y >= n {
            return None;
        }
        Some(UVec2::new(t.x as u32, self.size - 1 - t.y as u32))
    }

    /// The B value at render XZ `q` from the drawn target `depth` (the whole target,
    /// [`MAX_CAPTURE_SIZE`]², row-major by row y then column x) and the ring depth: [`convert`]
    /// at the texel holding `q` over the ground there; 0 outside. `trail.wgsl` `captured`.
    pub fn value_at(&self, q: Vec2, depth: &[f32], ring_depth: f32) -> f32 {
        let Some(t) = self.texel_of(q) else { return 0.0 };
        let d = depth[(t.y * MAX_CAPTURE_SIZE + t.x) as usize];
        convert(d, self.near_y, self.range_m, self.ground.at(q), ring_depth)
    }
}

/// The slab (render Y of its bottom, its height) over ground ranging `ground` (lowest, highest):
/// from `below_m` under the lowest ground to the highest raise over the highest ([`ceiling_ref`]).
pub fn slab(ground: (f32, f32), below_m: f32, ring_depth: f32) -> (f32, f32) {
    let near = ground.0 - below_m.max(0.0);
    let far = ground.1 + ceiling_ref(ring_depth);
    (near, (far - near).max(1e-3))
}
