//! Mesh data for water bodies with a shape (water.md "Water bodies at any height", "Rivers"): a lake
//! or pool polygon becomes a flat triangulated surface at its level, a river ribbon a sloped strip
//! with flow per vertex. Bevy-free: the renderer (`sf_water_bevy`) turns a [`BodyMesh`] into a GPU
//! mesh drawn with the water material.
//!
//! Industry: UE5 Water bodies do the same split (a lake is a polygon at a level; a river is a spline
//! with width, depth and velocity per point, its mesh a ribbon with the flow written per vertex).
//!
//! **Lakes** are a constrained Delaunay triangulation of the outline and its holes (`spade`, MIT/Apache,
//! already in the workspace for blend spaces), refined with Ruppert/Chew Steiner points
//! (Shewchuk's Triangle `-q` default, a 20° minimum angle, plus an area bound) so long thin triangles
//! never smear per-vertex shading across a lake. A CDT beats ear clipping (`earcutr`) here: outlines
//! come from marching squares with thousands of vertices, and ear clipping's slivers would need the
//! same refinement afterwards. Inside/outside is by parity (spade's flood fill across the constraint
//! edges), so holes may wind either way; crossing edges (a bad outline) are skipped, never a panic.
//!
//! **Rivers** sample a centripetal Catmull-Rom spline (Yuksel et al. 2011: no cusps or loops within a
//! segment) of the centre, width, height and speed, `spacing_m` apart and on every control point, so
//! the widths at the control points are exactly the input's. The height is clamped per segment to its
//! two control heights and then made monotone non-increasing downstream (water.md "Generated
//! rivers"), so the spline never lifts water above its source or dips below the next point. Each
//! sample gets `cross_samples` vertices across its width plus `bank_extend_m` past each bank (the
//! terrain hides it, so no gap shows at the shore). At a bend tighter than the half width, the
//! inner side is shortened to just before the point where neighbouring cross sections meet, so no
//! triangle folds over (the standard offset-curve limit: an offset may not exceed the radius of
//! curvature). The spline runs upstream to downstream.
//!
//! Coordinates: world XZ in metres (f64) in, positions relative to `origin` (f32) out, Y up and
//! absolute (the surface height), front faces counter-clockwise seen from above (normal +Y).

use std::collections::HashSet;

use glam::{DVec2, Vec3};
use spade::{
    AngleLimit, ConstrainedDelaunayTriangulation, Point2, RefinementParameters, Triangulation,
};

use crate::records::RiverPoint;

/// Points closer than this are one point (input cleaning), metres.
pub const POINT_EPS_M: f64 = 1e-4; // tunables: const (maths: a duplicate-vertex guard)
/// Rings or triangles with less area than this are degenerate, m².
pub const AREA_EPS_M2: f64 = 1e-8; // tunables: const (maths: a zero-area guard)
/// Cross sections stop at this share of the distance to where neighbouring ones meet.
pub const FOLD_MARGIN: f64 = 0.9; // tunables: const (a no-fold bound, as `SKEW_RATE_LIMIT`)
/// Chords per segment that estimate its length.
const LENGTH_PROBES: usize = 32; // tunables: const (maths: the arc-length table per segment)
/// Smallest spacing honoured, so a zero spacing cannot ask for unbounded samples, metres.
const MIN_SPACING_M: f64 = 0.05; // tunables: const (maths: an allocation guard below the range)
/// Centripetal Catmull-Rom (Yuksel et al. 2011).
const CENTRIPETAL: f64 = 0.5; // tunables: const (maths: the parameterisation itself)

/// Axis-aligned bounds of a [`BodyMesh`] (positions' frame); zero for an empty mesh.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BodyAabb {
    pub min: Vec3,
    pub max: Vec3,
}

/// A water body's surface mesh, ready for a GPU mesh (one vertex per index of each attribute).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BodyMesh {
    /// Relative to the `origin` passed in (XZ); Y is the absolute surface height. Metres.
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    /// Metres, for ripple and flow tiling: lakes the positions' XZ, rivers (across from the centre
    /// line, along its arc length from the source).
    pub uvs: Vec<[f32; 2]>,
    /// Downstream direction × speed in XZ, m/s; zero for lakes.
    pub flow: Vec<[f32; 2]>,
    /// Triangle list, counter-clockwise seen from above.
    pub indices: Vec<u32>,
    pub aabb: BodyAabb,
}

impl BodyMesh {
    /// No triangles (degenerate or empty input).
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.indices.is_empty()
    }

    #[must_use]
    pub fn triangle_count(&self) -> usize {
        self.indices.len() / 3
    }

    fn with_bounds(mut self) -> Self {
        if self.indices.is_empty() {
            return Self::default();
        }
        let mut min = Vec3::splat(f32::INFINITY);
        let mut max = Vec3::splat(f32::NEG_INFINITY);
        for p in &self.positions {
            let p = Vec3::from_array(*p);
            min = min.min(p);
            max = max.max(p);
        }
        self.aabb = BodyAabb { min, max };
        self
    }
}

/// Lake and pool triangulation. Every value is a tunable (`tunables.md`): unit and range in its doc.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LakeMeshParams {
    /// `water.lake.max_triangle_m2`: Steiner points split any triangle larger than this, m², range
    /// 0-10 000 (0 = no area bound), default 1024 (Guess: ~45 m triangles, a few per flow-map tile
    /// row of the 2 km clipmap level, cheap next to the terrain).
    pub max_triangle_area_m2: f64,
    /// `water.lake.min_angle_deg`: smallest triangle angle refinement aims for, degrees, range 0-30
    /// (0 = no angle bound), default 20 (Industry: Shewchuk's Triangle `-q` default; Ruppert's
    /// algorithm terminates up to ~20.7°).
    pub min_angle_deg: f64,
    /// `water.lake.max_steiner`: cap on the points refinement adds per lake, count, range 0-1 000 000
    /// (0 = the bare CDT of the outline), default 65 536 (Guess: bounds the bake on a degenerate
    /// outline; a 25 km² lake at the default area needs ~15 000).
    pub max_steiner_vertices: usize,
}

impl Default for LakeMeshParams {
    fn default() -> Self {
        Self {
            max_triangle_area_m2: 1024.0,
            min_angle_deg: 20.0,
            max_steiner_vertices: 65_536,
        }
    }
}

/// River ribbon sampling. Every value is a tunable (`tunables.md`): unit and range in its doc.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RiverMeshParams {
    /// `water.river.spacing_m`: sample spacing along the centre line (every control point is a
    /// sample too), metres, range 0.5-64, default 4 (Guess: water.md's 4 m spline error and 2 m
    /// flow-map texels).
    pub spacing_m: f64,
    /// `water.river.cross_samples`: vertices across the strip, count, range 2-33, default 5 (Guess:
    /// a centre and a quarter line each side carry the flow's shear; odd keeps a centre column).
    pub cross_samples: u32,
    /// `water.river.bank_extend_m`: the strip continues this far past each bank under the terrain,
    /// metres, range 0-16, default 2 (Guess: covers the 4 m terrain grid's half cell plus the shore
    /// simplification).
    pub bank_extend_m: f64,
}

impl Default for RiverMeshParams {
    fn default() -> Self {
        Self {
            spacing_m: 4.0,
            cross_samples: 5,
            bank_extend_m: 2.0,
        }
    }
}

/// One sample of a river's centre line ([`river_samples`]). XZ relative to the `origin` passed in.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RiverSample {
    pub center: DVec2,
    /// Unit downstream direction in XZ.
    pub tangent: DVec2,
    /// Water width (bank to bank, without the bank extension), metres.
    pub width: f64,
    /// Surface height (world Y), monotone non-increasing downstream.
    pub height: f64,
    /// Downstream speed, m/s.
    pub flow_speed: f64,
    /// Arc length from the first sample, metres.
    pub along: f64,
    /// Index of the input point this sample sits on, if any.
    pub control: Option<usize>,
}

/// The flat triangulated surface of a lake or pool at `level`: `outline` and `holes` are closed rings
/// in world XZ (either winding, closing point optional). Empty for fewer than three distinct points
/// or a zero-area outline; degenerate holes are skipped.
#[must_use]
pub fn lake_mesh(
    outline: &[DVec2],
    holes: &[Vec<DVec2>],
    level: f64,
    origin: DVec2,
    params: &LakeMeshParams,
) -> BodyMesh {
    let Some(outline) = clean_ring(outline, origin) else {
        return BodyMesh::default();
    };
    let rings: Vec<Vec<DVec2>> = std::iter::once(outline)
        .chain(holes.iter().filter_map(|h| clean_ring(h, origin)))
        .collect();

    let mut vertices = Vec::new();
    let mut edges = Vec::new();
    for ring in &rings {
        let base = vertices.len();
        let n = ring.len();
        vertices.extend(ring.iter().map(|p| Point2::new(p.x, p.y)));
        edges.extend((0..n).map(|k| [base + k, base + (k + 1) % n]));
    }
    // Crossing constraints (a self-intersecting or overlapping ring) are skipped, not fatal.
    let Ok(mut cdt) =
        ConstrainedDelaunayTriangulation::<Point2<f64>>::try_bulk_load_cdt(vertices, edges, |_| {})
    else {
        return BodyMesh::default();
    };

    let mut refine = RefinementParameters::<f64>::new()
        .exclude_outer_faces(true)
        .with_max_additional_vertices(params.max_steiner_vertices)
        .with_angle_limit(AngleLimit::from_deg(params.min_angle_deg.clamp(0.0, 30.0)));
    if params.max_triangle_area_m2 > 0.0 {
        refine = refine.with_max_allowed_area(params.max_triangle_area_m2);
    }
    let outer: HashSet<usize> = cdt
        .refine(refine)
        .excluded_faces
        .iter()
        .map(spade::handles::FixedFaceHandle::index)
        .collect();

    let mut mesh = BodyMesh::default();
    let mut remap = vec![u32::MAX; cdt.num_vertices()];
    #[allow(clippy::cast_possible_truncation)] // f64 → f32: the output format
    let y = level as f32;
    for face in cdt.inner_faces() {
        if outer.contains(&face.fix().index()) {
            continue;
        }
        let vs = face.vertices();
        let p = vs.map(|v| DVec2::new(v.position().x, v.position().y));
        let area = up_area(p[0], p[1], p[2]);
        if area.abs() <= AREA_EPS_M2 {
            continue;
        }
        let order = if area > 0.0 { [0, 1, 2] } else { [0, 2, 1] };
        for k in order {
            let slot = &mut remap[vs[k].fix().index()];
            if *slot == u32::MAX {
                *slot = u32::try_from(mesh.positions.len()).expect("lake mesh above u32 vertices");
                #[allow(clippy::cast_possible_truncation)]
                let (x, z) = (p[k].x as f32, p[k].y as f32);
                mesh.positions.push([x, y, z]);
                mesh.normals.push([0.0, 1.0, 0.0]);
                mesh.uvs.push([x, z]);
                mesh.flow.push([0.0, 0.0]);
            }
            mesh.indices.push(*slot);
        }
    }
    mesh.with_bounds()
}

/// The centre line of a river ribbon, resampled ([`RiverMeshParams::spacing_m`], plus every input
/// point), with width, height and speed interpolated (see the module doc). Empty for fewer than two
/// distinct finite points.
#[must_use]
pub fn river_samples(
    spline: &[RiverPoint],
    params: &RiverMeshParams,
    origin: DVec2,
) -> Vec<RiverSample> {
    let ctrl = clean_spline(spline, origin);
    let n = ctrl.len();
    if n < 2 {
        return Vec::new();
    }
    // Mirrored phantom points at both ends (the standard open Catmull-Rom end condition).
    let at = |i: isize| -> Channels {
        match usize::try_from(i) {
            Err(_) => ctrl[0].scale_add(2.0, &ctrl[1], -1.0),
            Ok(i) if i >= n => ctrl[n - 1].scale_add(2.0, &ctrl[n - 2], -1.0),
            Ok(i) => ctrl[i],
        }
    };
    let spacing = params.spacing_m.max(MIN_SPACING_M);
    let mut out: Vec<RiverSample> = Vec::new();
    for k in 0..n - 1 {
        let ki = isize::try_from(k).expect("river above isize points");
        let quad = [at(ki - 1), at(ki), at(ki + 1), at(ki + 2)];
        let seg = Segment::new(quad);
        // Cumulative chord lengths: the arc-length table that spaces the samples evenly.
        let mut cum = [0.0; LENGTH_PROBES + 1];
        let mut prev = seg.eval(0.0).xz();
        for j in 1..=LENGTH_PROBES {
            #[allow(clippy::cast_precision_loss)]
            let next = seg.eval(j as f64 / LENGTH_PROBES as f64).xz();
            cum[j] = cum[j - 1] + prev.distance(next);
            prev = next;
        }
        let len = cum[LENGTH_PROBES];
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let steps = ((len / spacing).ceil() as usize).max(1);
        // The segment parameter at arc length `l` (linear in the table).
        let param_at = |l: f64| -> f64 {
            let j = cum.partition_point(|&c| c < l).clamp(1, LENGTH_PROBES);
            let w = if cum[j] > cum[j - 1] {
                (l - cum[j - 1]) / (cum[j] - cum[j - 1])
            } else {
                0.0
            };
            #[allow(clippy::cast_precision_loss)]
            let s = (j as f64 - 1.0 + w.clamp(0.0, 1.0)) / LENGTH_PROBES as f64;
            s
        };
        let (a, b) = (ctrl[k], ctrl[k + 1]);
        let last = if k == n - 2 { steps } else { steps - 1 };
        for j in 0..=last {
            let (c, control) = match j {
                0 => (a, Some(k)),
                j if j == steps => (b, Some(k + 1)),
                #[allow(clippy::cast_precision_loss)]
                j => (seg.eval(param_at(len * j as f64 / steps as f64)), None),
            };
            out.push(RiverSample {
                center: c.xz(),
                tangent: DVec2::ZERO,
                width: c.0[2].clamp(a.0[2].min(b.0[2]), a.0[2].max(b.0[2])),
                height: c.0[3].clamp(b.0[3], a.0[3]),
                flow_speed: c.0[4].clamp(a.0[4].min(b.0[4]), a.0[4].max(b.0[4])),
                along: 0.0,
                control,
            });
        }
    }
    // Monotone heights, arc length, and the downstream tangent (central differences: the bisector
    // of the two neighbouring chords, i.e. the mitre direction).
    let m = out.len();
    for i in 1..m {
        out[i].height = out[i].height.min(out[i - 1].height);
        out[i].along = out[i - 1].along + out[i].center.distance(out[i - 1].center);
    }
    let mut fallback = (out[1].center - out[0].center).normalize_or(DVec2::X);
    for i in 0..m {
        let d = out[(i + 1).min(m - 1)].center - out[i.saturating_sub(1)].center;
        let t = d.normalize_or(fallback);
        out[i].tangent = t;
        fallback = t;
    }
    out
}

/// The sloped strip of a river ribbon ([`river_samples`] across its width plus the bank extension),
/// with flow per vertex. Empty for fewer than two distinct finite points.
#[must_use]
pub fn river_mesh(spline: &[RiverPoint], params: &RiverMeshParams, origin: DVec2) -> BodyMesh {
    let samples = river_samples(spline, params, origin);
    let m = samples.len();
    if m < 2 {
        return BodyMesh::default();
    }
    let cols = usize::try_from(params.cross_samples.max(2)).expect("u32 fits usize");
    let bank = params.bank_extend_m.max(0.0);
    let normal = |s: &RiverSample| s.tangent.perp();

    // Half widths per side ([0] = −normal, [1] = +normal), shortened where neighbouring cross
    // sections would meet within the strip.
    let mut half: Vec<[f64; 2]> = samples
        .iter()
        .map(|s| [s.width.max(0.0) * 0.5 + bank; 2])
        .collect();
    for i in 0..m - 1 {
        let (s0, s1) = (&samples[i], &samples[i + 1]);
        let (n0, n1, d) = (normal(s0), normal(s1), s1.center - s0.center);
        // s0 + a·n0 = s1 + b·n1
        let det = n1.perp_dot(n0);
        if det.abs() <= f64::EPSILON {
            continue;
        }
        let a = n1.perp_dot(d) / det;
        let b = n0.perp_dot(d) / det;
        if a * b <= 0.0 {
            continue;
        }
        let side = usize::from(a > 0.0);
        half[i][side] = half[i][side].min(a.abs() * FOLD_MARGIN);
        half[i + 1][side] = half[i + 1][side].min(b.abs() * FOLD_MARGIN);
    }

    let mut mesh = BodyMesh::default();
    for (s, h) in samples.iter().zip(&half) {
        let n = normal(s);
        let flow = s.tangent * s.flow_speed;
        for j in 0..cols {
            #[allow(clippy::cast_precision_loss)]
            let u = j as f64 / (cols - 1) as f64;
            let off = if u < 0.5 {
                -h[0] * (1.0 - 2.0 * u)
            } else {
                h[1] * (2.0 * u - 1.0)
            };
            let p = s.center + n * off;
            #[allow(clippy::cast_possible_truncation)]
            {
                mesh.positions
                    .push([p.x as f32, s.height as f32, p.y as f32]);
                mesh.uvs.push([off as f32, s.along as f32]);
                mesh.flow.push([flow.x as f32, flow.y as f32]);
            }
        }
    }
    let xz = |v: u32| {
        let p = mesh.positions[v as usize];
        DVec2::new(f64::from(p[0]), f64::from(p[2]))
    };
    let vid =
        |i: usize, j: usize| u32::try_from(i * cols + j).expect("river mesh above u32 vertices");
    let mut indices = Vec::with_capacity((m - 1) * (cols - 1) * 6);
    for i in 0..m - 1 {
        for j in 0..cols - 1 {
            let mut q = [vid(i, j), vid(i, j + 1), vid(i + 1, j + 1), vid(i + 1, j)];
            let p = q.map(xz);
            if up_area(p[0], p[1], p[2]) + up_area(p[0], p[2], p[3]) < 0.0 {
                q.swap(1, 3);
            }
            let p = q.map(xz);
            // The diagonal whose worse triangle is larger (the only valid one if the quad is concave).
            let diag_a = [[0, 1, 2], [0, 2, 3]];
            let diag_b = [[0, 1, 3], [1, 2, 3]];
            let worst = |d: &[[usize; 3]; 2]| {
                d.iter()
                    .map(|t| up_area(p[t[0]], p[t[1]], p[t[2]]))
                    .fold(f64::INFINITY, f64::min)
            };
            let tris = if worst(&diag_a) >= worst(&diag_b) {
                diag_a
            } else {
                diag_b
            };
            for t in tris {
                if up_area(p[t[0]], p[t[1]], p[t[2]]) > AREA_EPS_M2 {
                    indices.extend(t.map(|k| q[k]));
                }
            }
        }
    }
    mesh.normals = vertex_normals(&mesh.positions, &indices);
    mesh.indices = indices;
    mesh.with_bounds()
}

/// The signed area of the XZ triangle, positive when it faces +Y (counter-clockwise
/// seen from above; the Y of `(b − a) × (c − a)`), m².
fn up_area(p0: DVec2, p1: DVec2, p2: DVec2) -> f64 {
    let (e1, e2) = (p1 - p0, p2 - p0);
    0.5 * (e1.y * e2.x - e1.x * e2.y)
}

/// Area-weighted vertex normals (+Y where a vertex has no triangle).
fn vertex_normals(positions: &[[f32; 3]], indices: &[u32]) -> Vec<[f32; 3]> {
    let mut acc = vec![Vec3::ZERO; positions.len()];
    for t in indices.as_chunks::<3>().0 {
        let [a, b, c] = t.map(|i| Vec3::from_array(positions[i as usize]));
        let n = (b - a).cross(c - a);
        for &i in t {
            acc[i as usize] += n;
        }
    }
    acc.into_iter()
        .map(|n| n.try_normalize().unwrap_or(Vec3::Y).to_array())
        .collect()
}

/// Finite, deduplicated, open ring relative to `origin`; `None` below three points or zero area.
fn clean_ring(ring: &[DVec2], origin: DVec2) -> Option<Vec<DVec2>> {
    let eps2 = POINT_EPS_M * POINT_EPS_M;
    let mut out: Vec<DVec2> = Vec::with_capacity(ring.len());
    for &p in ring.iter().filter(|p| p.is_finite()) {
        let q = p - origin;
        if out.last().is_none_or(|l| l.distance_squared(q) > eps2) {
            out.push(q);
        }
    }
    while out.len() > 1 && out[0].distance_squared(out[out.len() - 1]) <= eps2 {
        out.pop();
    }
    let area: f64 = (0..out.len())
        .map(|k| out[k].perp_dot(out[(k + 1) % out.len()]))
        .sum::<f64>()
        * 0.5;
    (out.len() >= 3 && area.abs() > AREA_EPS_M2).then_some(out)
}

/// Finite, deduplicated control points relative to `origin`, heights made non-increasing.
fn clean_spline(spline: &[RiverPoint], origin: DVec2) -> Vec<Channels> {
    let eps2 = POINT_EPS_M * POINT_EPS_M;
    let mut out: Vec<Channels> = Vec::with_capacity(spline.len());
    for p in spline {
        let c = p.center - origin;
        let ch = Channels([
            c.x,
            c.y,
            f64::from(p.width).max(0.0),
            p.height,
            f64::from(p.flow_speed),
        ]);
        if !ch.0.iter().all(|v| v.is_finite()) {
            continue;
        }
        if out.last().is_none_or(|l| l.xz().distance_squared(c) > eps2) {
            out.push(ch);
        }
    }
    for i in 1..out.len() {
        out[i].0[3] = out[i].0[3].min(out[i - 1].0[3]);
    }
    out
}

/// The interpolated channels of a river point: x, z, width, height, speed.
#[derive(Clone, Copy, Debug)]
struct Channels([f64; 5]);

impl Channels {
    fn xz(&self) -> DVec2 {
        DVec2::new(self.0[0], self.0[1])
    }

    /// `self·s + other·t`, per channel.
    fn scale_add(&self, s: f64, other: &Self, t: f64) -> Self {
        Self(std::array::from_fn(|k| self.0[k] * s + other.0[k] * t))
    }

    fn lerp(a: &Self, b: &Self, t0: f64, t1: f64, t: f64) -> Self {
        let w = (t - t0) / (t1 - t0);
        a.scale_add(1.0 - w, b, w)
    }
}

/// One centripetal Catmull-Rom segment between `p[1]` and `p[2]` (Barry-Goldman pyramid); the
/// knots come from the XZ distances, every channel follows the same weights.
struct Segment {
    p: [Channels; 4],
    t: [f64; 4],
}

impl Segment {
    fn new(p: [Channels; 4]) -> Self {
        let mut t = [0.0; 4];
        for k in 1..4 {
            // Points are deduplicated (and the phantoms mirror a real chord), so every step is > 0.
            t[k] = t[k - 1]
                + p[k]
                    .xz()
                    .distance(p[k - 1].xz())
                    .powf(CENTRIPETAL)
                    .max(f64::EPSILON);
        }
        Self { p, t }
    }

    /// The point at `s` in 0..=1 between `p[1]` (0) and `p[2]` (1).
    fn eval(&self, s: f64) -> Channels {
        let (p, t) = (&self.p, &self.t);
        let u = t[1] + (t[2] - t[1]) * s;
        let a1 = Channels::lerp(&p[0], &p[1], t[0], t[1], u);
        let a2 = Channels::lerp(&p[1], &p[2], t[1], t[2], u);
        let a3 = Channels::lerp(&p[2], &p[3], t[2], t[3], u);
        let b1 = Channels::lerp(&a1, &a2, t[0], t[2], u);
        let b2 = Channels::lerp(&a2, &a3, t[1], t[3], u);
        Channels::lerp(&b1, &b2, t[1], t[2], u)
    }
}
