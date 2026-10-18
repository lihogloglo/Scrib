//! `cloth.cook` (physics.md "Cloth lattice"): a cloth part (a robe or skirt
//! hem) becomes a bone lattice for `sf_anim::cloth`. Columns go around the
//! body's vertical axis and rows from the attachment line down to the hem,
//! each particle on the part's outermost rest surface (a horizontal ray from
//! the axis, farthest hit, so linings and legs inside don't count). Row 0 is
//! pinned to the attachment (girdle) joint, and every particle rides that
//! joint: its offset in the joint's rest frame gives the animated target
//! (`joint model × offset`) the solver's max distance holds it to.
//!
//! Which parts are cloth and how they're set up is the adapter's
//! [`ClothRules`]; the colliders come from the rig's ragdoll capsules
//! (`sf_ragdoll::RagdollDef::cloth_colliders`). The cook works in rest space
//! like the smooth bodies: positions are the skeleton's rest pose in model
//! space (Y up), joints index the skeleton.

#[cfg(feature = "smooth")]
use glam::Vec3;
use serde::{Deserialize, Serialize};
use sf_anim::cloth::{ClothParams, GridSpec};
use sf_records::RecordId;

use crate::records::BodyPart;

/// How one cloth part is set up (an adapter's [`ClothRules`] answer).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClothSpec {
    /// The joint row 0 is pinned to and every particle rides (a girdle joint).
    pub attach: String,
    /// The attachment line relative to the joint's rest height, m (+ up).
    /// The line is also clamped to the part's top.
    pub attach_offset: f32,
    /// Lattice size, ring or open, mass and max distance.
    pub grid: GridSpec,
    /// Solver settings for this part.
    pub params: ClothParams,
}

impl Default for ClothSpec {
    fn default() -> Self {
        Self { attach: String::new(), attach_offset: 0.0, grid: GridSpec::default(), params: ClothParams::default() }
    }
}

/// Adapter rules for `cloth.cook`: which body parts are cloth and their setup.
pub trait ClothRules: Send + Sync {
    /// The setup for a body part, or `None` when it isn't cloth.
    fn cloth(&self, id: &RecordId, part: &BodyPart) -> Option<ClothSpec>;
}

#[cfg(feature = "smooth")]
/// Unit horizontal direction of column `c` of `cols`, starting at +Z (forward), turning toward +X.
#[allow(clippy::cast_precision_loss)]
pub(crate) fn column_dir(c: usize, cols: usize) -> Vec3 {
    let a = std::f32::consts::TAU * c as f32 / cols as f32;
    Vec3::new(a.sin(), 0.0, a.cos())
}

#[cfg(feature = "smooth")]
pub use cook::{ClothCook, ClothCookError, SkinRebind, cook_cloth};

#[cfg(feature = "smooth")]
mod cook {
    use glam::{Affine3A, Vec3};
    use serde::{Deserialize, Serialize};
    use sf_anim::cloth::{ClothCollider, ClothLattice};
    use sf_model::SkinVertex;

    use super::{ClothSpec, column_dir};
    use crate::smooth::{SkinSurface, SmoothSettings, inpaint_surface, top_influences};

    /// The `cloth.cook` product for one (part, skeleton).
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct ClothCook {
        pub lattice: ClothLattice,
        /// Per particle: its rest position in its joint's rest model frame.
        pub offsets: Vec<Vec3>,
        /// Per particle: the part's original skin where the particle sits
        /// (skeleton joints), so the op can blend the animated target from
        /// riding the girdle (`offsets`) toward following the legs as the
        /// authored skin does (narrow robes: see physics.md "As built (p10b)").
        pub targets: Vec<SkinVertex>,
        /// Attachment line (row 0) and hem (the part's lowest point) heights, model space, m.
        pub top: f32,
        pub hem: f32,
        /// The vertical axis the columns go around (x, z; y = `top`).
        pub center: Vec3,
        /// Particles whose ray missed the surface, placed from their column's nearest row.
        pub filled: usize,
        /// The part's vertices rebound to the lattice joints.
        pub rebind: SkinRebind,
    }

    /// The part's skin after `cloth.cook`: one [`SkinVertex`] per surface
    /// vertex, joints below `base` index the skeleton, joint `base + i` is
    /// lattice particle `i` (an outfit joint appended after the skeleton's:
    /// its bind is the particle's rest position, unrotated).
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct SkinRebind {
        pub base: u16,
        pub skin: Vec<SkinVertex>,
        /// Vertices bound to the lattice alone (below row 1).
        pub on_lattice: usize,
        /// Vertices inpainted in the band around the attachment line.
        pub band: usize,
        pub sweeps: usize,
        pub residual: f32,
    }

    #[derive(Clone, Debug, PartialEq, thiserror::Error)]
    pub enum ClothCookError {
        #[error("cloth lattice needs at least 2 rows and 3 columns")]
        Size,
        #[error("open (cape) lattices aren't placed yet")]
        Open,
        #[error("attachment joint {0} not in the skeleton")]
        Joint(u16),
        #[error("cloth part has no geometry below the attachment line")]
        Empty,
        #[error("cloth part hangs {0:.3} m below the attachment line, under 5 cm")]
        Short(f32),
        #[error("cloth column {0} found no surface")]
        Column(usize),
    }

    /// Places the lattice for one cloth part (module docs). `attach` is the
    /// spec's joint, resolved by the caller; `colliders` come from the rig.
    ///
    /// # Errors
    /// A lattice under 2 × 3, an open lattice (capes: not placed yet), an
    /// unknown joint, or a part with no surface hanging 5 cm below the line.
    #[allow(clippy::cast_precision_loss)]
    pub fn cook_cloth(
        surface: &SkinSurface,
        rest_model: &[Affine3A],
        attach: u16,
        spec: &ClothSpec,
        colliders: Vec<ClothCollider>,
    ) -> Result<ClothCook, ClothCookError> {
        let grid = &spec.grid;
        let (rows, cols) = (usize::from(grid.rows), usize::from(grid.cols));
        if rows < 2 || cols < 3 {
            return Err(ClothCookError::Size);
        }
        if !grid.ring {
            return Err(ClothCookError::Open);
        }
        let joint = *rest_model.get(usize::from(attach)).ok_or(ClothCookError::Joint(attach))?;
        let line = joint.translation.y + spec.attach_offset;
        let (mut lo, mut hi) = (Vec3::splat(f32::MAX), Vec3::splat(f32::MIN));
        let mut top_of_part = f32::MIN;
        for &p in &surface.positions {
            top_of_part = top_of_part.max(p.y);
            if p.y < line {
                lo = lo.min(p);
                hi = hi.max(p);
            }
        }
        if lo.y > hi.y {
            return Err(ClothCookError::Empty);
        }
        let (top, hem) = (line.min(top_of_part), lo.y);
        let span = top - hem;
        if span < 0.05 {
            return Err(ClothCookError::Short(span.max(0.0)));
        }
        let center = Vec3::new(lo.x.midpoint(hi.x), top, lo.z.midpoint(hi.z));
        // Rows: the ends pulled 1 % inside, so the rays meet the band, not its rim.
        let height = |r: usize| top - span * (0.01 + 0.98 * r as f32 / (rows - 1) as f32);
        let mut radius: Vec<Option<(f32, SkinVertex)>> = (0..rows * cols)
            .map(|i| farthest_hit(surface, Vec3::new(center.x, height(i / cols), center.z), column_dir(i % cols, cols)))
            .collect();
        let mut filled = 0;
        for c in 0..cols {
            let known: Vec<(usize, (f32, SkinVertex))> = (0..rows).filter_map(|r| radius[r * cols + c].map(|x| (r, x))).collect();
            if known.is_empty() {
                return Err(ClothCookError::Column(c));
            }
            for r in 0..rows {
                if radius[r * cols + c].is_none() {
                    let nearest = known.iter().min_by_key(|(k, _)| k.abs_diff(r)).expect("non-empty");
                    radius[r * cols + c] = Some(nearest.1);
                    filled += 1;
                }
            }
        }
        let rest: Vec<Vec3> = (0..rows * cols)
            .map(|i| {
                let (r, c) = (i / cols, i % cols);
                Vec3::new(center.x, height(r), center.z) + column_dir(c, cols) * radius[i].expect("filled").0
            })
            .collect();
        let mut lattice = ClothLattice::grid(grid, &rest, &vec![attach; rest.len()]);
        lattice.colliders = colliders;
        let inverse = joint.inverse();
        let offsets = rest.iter().map(|&p| inverse.transform_point3(p)).collect();
        let base = u16::try_from(rest_model.len()).map_err(|_| ClothCookError::Joint(attach))?;
        let rebind = rebind(surface, &Placement { rows, cols, center, first: height(0), last: height(rows - 1) }, base);
        let targets = radius.into_iter().map(|x| x.expect("filled").1).collect();
        Ok(ClothCook { lattice, offsets, targets, top, hem, center, filled, rebind })
    }

    /// Where the rows and columns are, for the rebind.
    struct Placement {
        rows: usize,
        cols: usize,
        center: Vec3,
        /// Heights of row 0 and of the last row.
        first: f32,
        last: f32,
    }

    /// Vertices below row 1 go to the lattice (bilinear in row height and
    /// column angle; below the hem row they ride the hem); vertices above
    /// the attachment line keep their weights; the band from the line to
    /// row 1 is harmonic (`smooth_body`'s inpainting), so the waist blends
    /// from the body's weights into the pinned row and on into the hem.
    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn rebind(surface: &SkinSurface, at: &Placement, base: u16) -> SkinRebind {
        let spacing = (at.first - at.last) / (at.rows - 1) as f32;
        let joint = |r: usize, c: usize| base + u16::try_from(r * at.cols + c % at.cols).expect("lattice fits u16 joints");
        let lattice_weights = |p: Vec3| {
            let t = ((at.first - p.y) / spacing).clamp(0.0, (at.rows - 1) as f32);
            let r0 = (t.floor() as usize).min(at.rows - 2);
            let fr = t - r0 as f32;
            let d = p - at.center;
            let u = d.x.atan2(d.z).rem_euclid(std::f32::consts::TAU) / std::f32::consts::TAU * at.cols as f32;
            let c0 = (u.floor() as usize) % at.cols;
            let fc = u - u.floor();
            let mut w = Vec::with_capacity(4);
            for (j, x) in [
                (joint(r0, c0), (1.0 - fr) * (1.0 - fc)),
                (joint(r0, c0 + 1), (1.0 - fr) * fc),
                (joint(r0 + 1, c0), fr * (1.0 - fc)),
                (joint(r0 + 1, c0 + 1), fr * fc),
            ] {
                if x > 1e-6 {
                    w.push((j, x));
                }
            }
            w
        };
        // The band: from a quarter row above the line down to row 1.
        let (band_top, band_bottom) = (at.first + 0.25 * spacing, at.first - spacing);
        let mut weights = Vec::with_capacity(surface.positions.len());
        let mut unknown = Vec::with_capacity(surface.positions.len());
        let (mut on_lattice, mut band) = (0, 0);
        for (&p, s) in surface.positions.iter().zip(&surface.skin) {
            if p.y <= band_bottom {
                weights.push(lattice_weights(p));
                unknown.push(false);
                on_lattice += 1;
            } else {
                weights.push(s.joints.iter().zip(s.weights).filter(|(_, x)| *x > 0.0).map(|(&j, x)| (j, x)).collect());
                let inside = p.y < band_top;
                unknown.push(inside);
                band += usize::from(inside);
            }
        }
        let settings = SmoothSettings::default();
        let (skin, sweeps, residual) = inpaint_surface(surface, &weights, &unknown, settings.weld, settings.iterations, settings.tolerance);
        SkinRebind { base, skin, on_lattice, band, sweeps, residual }
    }

    /// Distance along `dir` (horizontal, unit) from `origin` to the farthest
    /// triangle of `surface` it crosses (Möller–Trumbore, both faces), and
    /// the skin there (the corners' weights, barycentric).
    #[allow(clippy::many_single_char_names)] // the textbook Möller–Trumbore names
    fn farthest_hit(surface: &SkinSurface, origin: Vec3, dir: Vec3) -> Option<(f32, SkinVertex)> {
        let mut best: Option<(f32, usize, f32, f32)> = None;
        for (k, t) in surface.triangles.iter().enumerate() {
            let [a, b, c] = t.map(|i| surface.positions[i as usize]);
            let (e1, e2) = (b - a, c - a);
            let p = dir.cross(e2);
            let det = e1.dot(p);
            if det.abs() < 1e-12 {
                continue;
            }
            let inv = 1.0 / det;
            let s = origin - a;
            let u = s.dot(p) * inv;
            if !(0.0..=1.0).contains(&u) {
                continue;
            }
            let q = s.cross(e1);
            let v = dir.dot(q) * inv;
            if v < 0.0 || u + v > 1.0 {
                continue;
            }
            let d = e2.dot(q) * inv;
            if d > 0.0 && best.is_none_or(|x| d > x.0) {
                best = Some((d, k, u, v));
            }
        }
        let (d, k, u, v) = best?;
        let mut w = Vec::new();
        for (corner, x) in surface.triangles[k].into_iter().zip([1.0 - u - v, u, v]) {
            let s = surface.skin[corner as usize];
            for (&j, y) in s.joints.iter().zip(s.weights) {
                if let Some(e) = w.iter_mut().find(|e: &&mut (u16, f32)| e.0 == j) {
                    e.1 += x * y;
                } else {
                    w.push((j, x * y));
                }
            }
        }
        Some((d, top_influences(&w)))
    }
}
