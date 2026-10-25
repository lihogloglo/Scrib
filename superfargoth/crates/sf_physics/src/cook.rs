//! Body shapes for movable objects (physics.md "Bake products": `body.cook`).
//!
//! A [`BodyShape`] is a set of convex parts with the mass properties of the
//! whole, cooked once per model and shared by every placement. The fit, first
//! match (the industry's usual ladder: UE Physics Asset primitives / auto
//! convex collision, Havok hulls and primitives):
//!
//! 1. Authored collision ([`fit_authored`]): one hull per authored part.
//! 2. A primitive (box, capsule, cylinder, sphere) that covers the render
//!    hull with at most [`FitSettings::primitive_tolerance`] more volume
//!    (bottles roll, swords lie flat). Thin models (an extent under
//!    [`FitSettings::min_extent`]) become a box padded to that extent.
//! 3. One convex hull when the mesh isn't closed or the hull's volume is
//!    within [`FitSettings::hull_tolerance`] of the closed mesh's.
//! 4. Else a convex decomposition (parry's V-HACD), at most
//!    [`FitSettings::max_hulls`] hulls.
//!
//! Every hull keeps at most [`FitSettings::max_hull_vertices`] points
//! (farthest-point subset of the hull's vertices, then re-hulled): contact
//! cost grows with hull size, and a few millimeters don't show on a cup.

use std::collections::HashMap;
use std::f32::consts::PI;

use glam::{Quat, Vec3};
use parry3d::math::Pose;
use parry3d::transformation::vhacd::{VHACD, VHACDParameters};
use rapier3d::geometry::SharedShape;
use serde::{Deserialize, Serialize};

/// Version of the cooked body format (`body.cook` products).
pub const BODY_FORMAT: u32 = 1;
const BODY_MAGIC: [u8; 4] = *b"SFBD";

/// How [`fit_render`] picks a shape.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct FitSettings {
    /// A primitive may exceed the hull's volume by this fraction.
    pub primitive_tolerance: f32,
    /// A single hull may exceed a closed mesh's volume by this fraction.
    pub hull_tolerance: f32,
    /// Most hulls of a decomposition.
    pub max_hulls: u32,
    /// Most vertices per hull.
    pub max_hull_vertices: u32,
    /// Smallest extent per axis, meters (thinner shapes are padded).
    pub min_extent: f32,
    /// V-HACD voxel resolution.
    pub resolution: u32,
}

impl Default for FitSettings {
    fn default() -> Self {
        Self {
            primitive_tolerance: 0.15,
            hull_tolerance: 0.25,
            max_hulls: 8,
            max_hull_vertices: 32,
            min_extent: 0.01,
            resolution: 64,
        }
    }
}

/// One convex part, in model space.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PartShape {
    /// Hull vertices (already convex-hull points).
    Hull(Vec<[f32; 3]>),
    /// Half extents.
    Box([f32; 3]),
    /// Along local Y: radius, half length of the segment.
    Capsule { r: f32, half: f32 },
    /// Along local Y: radius, half height.
    Cylinder { r: f32, half: f32 },
    Sphere(f32),
}

/// A part and where it sits in the model.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShapePart {
    pub shape: PartShape,
    pub translation: [f32; 3],
    /// Quaternion (x, y, z, w).
    pub rotation: [f32; 4],
}

/// Where a body shape came from.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum BodySource {
    /// The adapter's authored collision parts.
    Authored,
    /// A primitive fit of the render mesh.
    Primitive,
    /// A thin render mesh padded to the minimum extent (a box).
    Thin,
    /// The render mesh's convex hull.
    Hull,
    /// A convex decomposition of the render mesh.
    Decomposition,
}

/// A movable object's collision and unit-density mass properties.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BodyShape {
    pub parts: Vec<ShapePart>,
    /// Cubic meters (mass at 1 kg/m³).
    pub volume: f32,
    pub center_of_mass: [f32; 3],
    /// Principal inertia per kilogram of mass, and its frame (x, y, z, w).
    pub unit_inertia: [f32; 3],
    pub inertia_frame: [f32; 4],
    /// Model-space bounds (min, max).
    pub aabb: [[f32; 3]; 2],
    pub source: BodySource,
}

impl ShapePart {
    fn new(shape: PartShape, translation: Vec3, rotation: Quat) -> Self {
        Self { shape, translation: translation.to_array(), rotation: rotation.to_array() }
    }

    /// The part's shape and pose, at a placement's `scale` (non-uniform scale
    /// is exact for hulls; primitives take its largest component).
    fn shared(&self, scale: Vec3) -> Option<(Pose, SharedShape)> {
        let s = scale.max_element();
        let shape = match &self.shape {
            PartShape::Hull(points) => {
                let pts: Vec<Vec3> = points.iter().map(|p| Vec3::from_array(*p) * scale).collect();
                SharedShape::convex_hull(&pts)?
            }
            PartShape::Box(h) => {
                let h = Vec3::from_array(*h) * s;
                SharedShape::cuboid(h.x, h.y, h.z)
            }
            PartShape::Capsule { r, half } => SharedShape::capsule_y(half * s, r * s),
            PartShape::Cylinder { r, half } => SharedShape::cylinder(half * s, r * s),
            PartShape::Sphere(r) => SharedShape::ball(r * s),
        };
        let pose = Pose::from_parts(Vec3::from_array(self.translation) * scale, Quat::from_array(self.rotation));
        Some((pose, shape))
    }

    /// Vertex count of a hull part (0 for primitives).
    pub fn hull_vertices(&self) -> usize {
        match &self.shape {
            PartShape::Hull(p) => p.len(),
            _ => 0,
        }
    }
}

/// Why a body couldn't be fitted.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FitError(pub String);

impl std::fmt::Display for FitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for FitError {}

impl BodyShape {
    /// The engine shape at a placement's scale: the single part, or a compound.
    pub fn shared(&self, scale: Vec3) -> Option<SharedShape> {
        let mut parts: Vec<(Pose, SharedShape)> = self.parts.iter().filter_map(|p| p.shared(scale)).collect();
        match parts.len() {
            0 => None,
            1 if parts[0].0 == Pose::IDENTITY => Some(parts.pop()?.1),
            _ => Some(SharedShape::compound(parts)),
        }
    }

    /// Simulated mass of this shape at `density`, kg (before the world's clamp).
    pub fn mass(&self, density: f32) -> f32 {
        self.volume * density
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = BODY_MAGIC.to_vec();
        out.extend_from_slice(&BODY_FORMAT.to_le_bytes());
        out.extend(postcard::to_stdvec(self).expect("in-memory serialization"));
        out
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, FitError> {
        let bad = |why: &str| FitError(format!("body shape: {why}"));
        if bytes.len() < 8 || bytes[..4] != BODY_MAGIC {
            return Err(bad("bad magic"));
        }
        let format = u32::from_le_bytes(bytes[4..8].try_into().expect("4 bytes"));
        if format != BODY_FORMAT {
            return Err(bad(&format!("format {format}, expected {BODY_FORMAT}")));
        }
        postcard::from_bytes(&bytes[8..]).map_err(|e| bad(&e.to_string()))
    }

    /// Fills the mass properties and bounds from the parts.
    fn finish(parts: Vec<ShapePart>, source: BodySource) -> Result<Self, FitError> {
        let mut body = Self {
            parts,
            volume: 0.0,
            center_of_mass: [0.0; 3],
            unit_inertia: [0.0; 3],
            inertia_frame: Quat::IDENTITY.to_array(),
            aabb: [[0.0; 3]; 2],
            source,
        };
        let shape = body.shared(Vec3::ONE).ok_or_else(|| FitError("no valid part".into()))?;
        let props = shape.mass_properties(1.0);
        let mass = props.mass();
        if !(mass.is_finite() && mass > 0.0) {
            return Err(FitError(format!("degenerate volume {mass}")));
        }
        let aabb = shape.compute_local_aabb();
        body.volume = mass;
        body.center_of_mass = props.local_com.to_array();
        body.unit_inertia = (props.principal_inertia() / mass).to_array();
        body.inertia_frame = props.principal_inertia_local_frame.to_array();
        body.aabb = [aabb.mins.to_array(), aabb.maxs.to_array()];
        Ok(body)
    }
}

/// One hull per authored point set (a set too flat for a hull becomes a box
/// padded to the minimum extent).
pub fn fit_authored(parts: &[Vec<Vec3>], settings: &FitSettings) -> Result<BodyShape, FitError> {
    let parts: Vec<ShapePart> = parts.iter().filter(|p| !p.is_empty()).filter_map(|p| hull_part(p, settings)).collect();
    if parts.is_empty() {
        return Err(FitError("authored collision has no geometry".into()));
    }
    BodyShape::finish(parts, BodySource::Authored)
}

/// Fits a render mesh (module docs, steps 2 to 4).
pub fn fit_render(positions: &[Vec3], triangles: &[[u32; 3]], settings: &FitSettings) -> Result<BodyShape, FitError> {
    if positions.len() < 3 || triangles.is_empty() {
        return Err(FitError("no render geometry".into()));
    }
    let Ok((hull, _)) = parry3d::transformation::try_convex_hull(positions) else {
        let part = thin_box(positions, settings).ok_or_else(|| FitError("degenerate geometry".into()))?;
        return BodyShape::finish(vec![part], BodySource::Thin);
    };
    let (_, half) = obb(&hull);
    if half.min_element() * 2.0 < settings.min_extent {
        let part = thin_box(&hull, settings).ok_or_else(|| FitError("degenerate geometry".into()))?;
        return BodyShape::finish(vec![part], BodySource::Thin);
    }
    let hull_volume = SharedShape::convex_hull(&hull).map_or(0.0, |s| s.mass_properties(1.0).mass());
    if hull_volume <= 0.0 {
        let part = thin_box(&hull, settings).ok_or_else(|| FitError("degenerate geometry".into()))?;
        return BodyShape::finish(vec![part], BodySource::Thin);
    }
    if let Some(part) = best_primitive(&hull, hull_volume * (1.0 + settings.primitive_tolerance)) {
        return BodyShape::finish(vec![part], BodySource::Primitive);
    }
    let single = || -> Result<BodyShape, FitError> {
        let part = hull_part(&hull, settings).ok_or_else(|| FitError("hull failed".into()))?;
        BodyShape::finish(vec![part], BodySource::Hull)
    };
    let Some(mesh_volume) = closed_volume(positions, triangles) else { return single() };
    if hull_volume <= mesh_volume * (1.0 + settings.hull_tolerance) {
        return single();
    }
    let params = VHACDParameters {
        max_convex_hulls: settings.max_hulls,
        resolution: settings.resolution,
        ..VHACDParameters::default()
    };
    let decomposition = VHACD::decompose(&params, positions, triangles, false);
    let parts: Vec<ShapePart> = decomposition
        .compute_convex_hulls(4)
        .iter()
        .take(settings.max_hulls as usize)
        .filter_map(|(points, _)| hull_part(points, settings))
        .collect();
    if parts.len() < 2 {
        return single();
    }
    BodyShape::finish(parts, BodySource::Decomposition)
}

/// The oriented box of a point set (parry's covariance frame), or the
/// model-axis box when that one is smaller: (pose, half extents) with the
/// pose at the box center.
fn obb(points: &[Vec3]) -> (Pose, Vec3) {
    let frame_box = |rotation: Quat| {
        let inv = rotation.inverse();
        let (mut lo, mut hi) = (Vec3::splat(f32::MAX), Vec3::splat(f32::MIN));
        for p in points {
            let q = inv * *p;
            lo = lo.min(q);
            hi = hi.max(q);
        }
        (Pose::from_parts(rotation * ((lo + hi) * 0.5), rotation), (hi - lo) * 0.5)
    };
    let (pose, _) = parry3d::utils::obb(points);
    let oriented = frame_box(pose.rotation);
    let axis = frame_box(Quat::IDENTITY);
    if oriented.1.x * oriented.1.y * oriented.1.z < axis.1.x * axis.1.y * axis.1.z * 0.98 { oriented } else { axis }
}

/// A box padded to the minimum extent around a (near) planar set.
fn thin_box(points: &[Vec3], settings: &FitSettings) -> Option<ShapePart> {
    if points.is_empty() {
        return None;
    }
    let (pose, half) = obb(points);
    let half = half.max(Vec3::splat(settings.min_extent * 0.5));
    Some(ShapePart::new(PartShape::Box(half.to_array()), pose.translation, pose.rotation))
}

/// The hull of `points`, at most `max_hull_vertices` vertices.
fn hull_part(points: &[Vec3], settings: &FitSettings) -> Option<ShapePart> {
    let Ok((mut hull, _)) = parry3d::transformation::try_convex_hull(points) else { return thin_box(points, settings) };
    let max = settings.max_hull_vertices.max(8) as usize;
    if hull.len() > max {
        let subset = farthest_points(&hull, max);
        hull = parry3d::transformation::try_convex_hull(&subset).map(|(h, _)| h).unwrap_or(subset);
    }
    let (_, half) = obb(&hull);
    if half.min_element() * 2.0 < settings.min_extent {
        return thin_box(&hull, settings);
    }
    Some(ShapePart::new(PartShape::Hull(hull.iter().map(Vec3::to_array).collect()), Vec3::ZERO, Quat::IDENTITY))
}

/// Deterministic farthest-point subset: the six axis extremes, then the point
/// farthest from everything chosen so far, until `n`.
fn farthest_points(points: &[Vec3], n: usize) -> Vec<Vec3> {
    let mut chosen: Vec<usize> = Vec::with_capacity(n);
    for axis in 0..3 {
        let by = |i: &usize| points[*i][axis];
        let idx = 0..points.len();
        let lo = idx.clone().min_by(|a, b| by(a).total_cmp(&by(b)));
        let hi = idx.max_by(|a, b| by(a).total_cmp(&by(b)));
        for i in [lo, hi].into_iter().flatten() {
            if !chosen.contains(&i) {
                chosen.push(i);
            }
        }
    }
    let mut dist: Vec<f32> =
        points.iter().map(|p| chosen.iter().map(|&c| p.distance_squared(points[c])).fold(f32::MAX, f32::min)).collect();
    while chosen.len() < n.min(points.len()) {
        let (next, _) = dist.iter().enumerate().max_by(|a, b| a.1.total_cmp(b.1)).expect("non-empty");
        chosen.push(next);
        for (i, d) in dist.iter_mut().enumerate() {
            *d = d.min(points[i].distance_squared(points[next]));
        }
    }
    chosen.sort_unstable();
    chosen.into_iter().map(|i| points[i]).collect()
}

/// The smallest primitive covering `hull` whose volume is at most `limit`.
fn best_primitive(hull: &[Vec3], limit: f32) -> Option<ShapePart> {
    let mut best: Option<(f32, ShapePart)> = None;
    let mut offer = |volume: f32, part: ShapePart| {
        if volume <= limit && best.as_ref().is_none_or(|(v, _)| volume < *v) {
            best = Some((volume, part));
        }
    };
    let (pose, half) = obb(hull);
    let (center, rotation) = (pose.translation, pose.rotation);
    let inv = rotation.inverse();
    let local: Vec<Vec3> = hull.iter().map(|p| inv * (*p - center)).collect();

    offer(8.0 * half.x * half.y * half.z, ShapePart::new(PartShape::Box(half.to_array()), center, rotation));
    let r = local.iter().map(|q| q.length()).fold(0.0, f32::max);
    offer(4.0 / 3.0 * PI * r * r * r, ShapePart::new(PartShape::Sphere(r), center, Quat::IDENTITY));
    for axis in 0..3 {
        // Maps local Y onto the box axis.
        let to_axis = match axis {
            0 => Quat::from_rotation_z(-PI / 2.0),
            1 => Quat::IDENTITY,
            _ => Quat::from_rotation_x(PI / 2.0),
        };
        let along = |q: &Vec3| q[axis];
        let radial = |q: &Vec3| {
            let mut v = *q;
            v[axis] = 0.0;
            v.length()
        };
        let r = local.iter().map(radial).fold(0.0, f32::max);
        let h = half[axis];
        offer(2.0 * PI * r * r * h, ShapePart::new(PartShape::Cylinder { r, half: h }, center, rotation * to_axis));
        // Capsule: the segment shortened by the radial radius, the radius
        // grown until every point is covered.
        let seg = (h - r).max(0.0);
        let rc = local
            .iter()
            .map(|q| {
                let t = along(q).clamp(-seg, seg);
                let mut on = Vec3::ZERO;
                on[axis] = t;
                q.distance(on)
            })
            .fold(0.0, f32::max);
        offer(
            PI * rc * rc * 2.0 * seg + 4.0 / 3.0 * PI * rc * rc * rc,
            ShapePart::new(PartShape::Capsule { r: rc, half: seg }, center, rotation * to_axis),
        );
    }
    best.map(|(_, p)| p)
}

/// Volume of a closed (watertight after welding) mesh, else `None`.
fn closed_volume(positions: &[Vec3], triangles: &[[u32; 3]]) -> Option<f32> {
    // Weld split vertices (UV and normal seams) on a 0.1 mm grid.
    let mut ids: HashMap<[i32; 3], u32> = HashMap::new();
    #[allow(clippy::cast_possible_truncation)]
    let key = |p: Vec3| (p * 1e4).round().to_array().map(|c| c as i32);
    let welded: Vec<u32> = positions
        .iter()
        .map(|p| {
            let n = u32::try_from(ids.len()).expect("vertex count fits u32");
            *ids.entry(key(*p)).or_insert(n)
        })
        .collect();
    let mut edges: HashMap<(u32, u32), u32> = HashMap::new();
    let mut volume = 0.0;
    for t in triangles {
        let w = t.map(|i| welded[i as usize]);
        if w[0] == w[1] || w[1] == w[2] || w[0] == w[2] {
            continue;
        }
        for (a, b) in [(w[0], w[1]), (w[1], w[2]), (w[2], w[0])] {
            *edges.entry((a.min(b), a.max(b))).or_default() += 1;
        }
        let [a, b, c] = t.map(|i| positions[i as usize]);
        volume += a.dot(b.cross(c)) / 6.0;
    }
    (!edges.is_empty() && edges.values().all(|&n| n == 2)).then_some(volume.abs()).filter(|v| *v > 0.0)
}
