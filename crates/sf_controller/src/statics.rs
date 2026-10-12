//! The static query world: cooked collision meshes of the R0 ring's
//! statics in a parry3d BVH (locomotion.md "Ground probes"). The same
//! shapes move into the physics world in M4.
//!
//! Precision: the BVH lives in f32 coordinates relative to an anchor
//! [`WorldPos`]. R0 is a few hundred meters across, so offsets stay well
//! under a millimeter of error; [`StaticWorld::rebase`] moves the anchor
//! when the ring moves far.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use glam::{Affine3A, Vec3};
use parry3d::bounding_volume::Aabb;
use parry3d::math::Pose;
use parry3d::partitioning::{Bvh, BvhBuildStrategy};
use parry3d::query::{Ray, RayCast, ShapeCastOptions, cast_shapes};
use parry3d::shape::{Ball, Capsule, Shape, TriMesh};
use serde::{Deserialize, Serialize};
use sf_coords::{WorldPos, WorldTransform};

use crate::FormatError;
use crate::probe::{GroundHit, GroundProbe, ProbeQuery, SurfaceId};

/// Version of the cooked collision format (`collision.cook` products).
pub const COLLISION_FORMAT: u32 = 1;
const MAGIC: [u8; 4] = *b"SFCL";

/// Cooked collision geometry of one model, in model space: a welded
/// triangle soup with a surface per triangle.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CollisionMesh {
    pub positions: Vec<[f32; 3]>,
    pub triangles: Vec<[u32; 3]>,
    /// Per triangle.
    pub surfaces: Vec<SurfaceId>,
}

impl CollisionMesh {
    pub fn is_empty(&self) -> bool {
        self.triangles.is_empty()
    }

    pub fn surface(&self, triangle: u32) -> SurfaceId {
        self.surfaces.get(triangle as usize).copied().unwrap_or_default()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = MAGIC.to_vec();
        out.extend_from_slice(&COLLISION_FORMAT.to_le_bytes());
        out.extend(postcard::to_stdvec(self).expect("in-memory serialization"));
        out
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, FormatError> {
        if bytes.len() < 8 || bytes[..4] != MAGIC {
            return Err(FormatError::Magic);
        }
        let found = u32::from_le_bytes(bytes[4..8].try_into().expect("4 bytes"));
        if found != COLLISION_FORMAT {
            return Err(FormatError::Version { found, expected: COLLISION_FORMAT });
        }
        let mesh: Self = postcard::from_bytes(&bytes[8..]).map_err(|e| FormatError::Decode(e.to_string()))?;
        let n = u32::try_from(mesh.positions.len()).map_err(|_| FormatError::Decode("too many vertices".into()))?;
        if mesh.surfaces.len() != mesh.triangles.len() || mesh.triangles.iter().flatten().any(|&i| i >= n) {
            return Err(FormatError::Decode("inconsistent collision mesh".into()));
        }
        Ok(mesh)
    }
}

/// Builds a [`CollisionMesh`] from render or authored triangles: welds
/// vertices on a `weld`-meter grid (render meshes split vertices by normal
/// and UV, collision doesn't care), and drops degenerate and duplicate
/// triangles (either winding). Deterministic: output order follows input
/// order.
pub struct CollisionBuilder {
    weld: f32,
    lookup: HashMap<[i32; 3], u32>,
    seen: HashSet<[u32; 3]>,
    mesh: CollisionMesh,
}

impl CollisionBuilder {
    pub fn new(weld: f32) -> Self {
        Self { weld: weld.max(1e-6), lookup: HashMap::new(), seen: HashSet::new(), mesh: CollisionMesh::default() }
    }

    #[allow(clippy::cast_possible_truncation)]
    fn vertex(&mut self, p: Vec3) -> u32 {
        let key = (p / self.weld).round().as_ivec3().to_array();
        *self.lookup.entry(key).or_insert_with(|| {
            self.mesh.positions.push(p.to_array());
            (self.mesh.positions.len() - 1) as u32
        })
    }

    /// Adds a triangle list, transformed into model space by `transform`.
    pub fn add(&mut self, positions: &[[f32; 3]], indices: &[u32], transform: Affine3A, surface: SurfaceId) {
        for tri in indices.as_chunks::<3>().0 {
            let Some(p) = tri.iter().map(|&i| positions.get(i as usize)).collect::<Option<Vec<_>>>() else {
                continue;
            };
            let t = [0, 1, 2].map(|k| self.vertex(transform.transform_point3(Vec3::from_array(*p[k]))));
            if t[0] == t[1] || t[1] == t[2] || t[0] == t[2] {
                continue;
            }
            let [a, b, c] = t.map(|i| Vec3::from_array(self.mesh.positions[i as usize]));
            if (b - a).cross(c - a).length_squared() < 1e-12 {
                continue;
            }
            let mut key = t;
            key.sort_unstable();
            if self.seen.insert(key) {
                self.mesh.triangles.push(t);
                self.mesh.surfaces.push(surface);
            }
        }
    }

    /// The mesh, without vertices only dropped triangles used.
    pub fn finish(self) -> CollisionMesh {
        let mut mesh = self.mesh;
        let mut remap = vec![u32::MAX; mesh.positions.len()];
        for &i in mesh.triangles.iter().flatten() {
            remap[i as usize] = 0;
        }
        let mut kept = 0u32;
        let mut positions = Vec::with_capacity(mesh.positions.len());
        for (r, p) in remap.iter_mut().zip(&mesh.positions) {
            if *r == 0 {
                *r = kept;
                kept += 1;
                positions.push(*p);
            }
        }
        for t in &mut mesh.triangles {
            *t = t.map(|i| remap[i as usize]);
        }
        mesh.positions = positions;
        mesh
    }
}

/// A collision mesh ready for queries (parry `TriMesh` with its own BVH).
/// Built once per model, on the task pool as cells activate, and shared by
/// every placement through an `Arc`.
pub struct StaticShape {
    mesh: Arc<TriMesh>,
    surfaces: Vec<SurfaceId>,
}

impl StaticShape {
    /// `None` for an empty mesh or one parry rejects.
    pub fn new(mesh: &CollisionMesh) -> Option<Self> {
        if mesh.is_empty() {
            return None;
        }
        let vertices = mesh.positions.iter().map(|p| Vec3::from_array(*p)).collect();
        let tri = TriMesh::new(vertices, mesh.triangles.clone()).ok()?;
        Some(Self { mesh: Arc::new(tri), surfaces: mesh.surfaces.clone() })
    }

    pub fn triangle_count(&self) -> usize {
        self.mesh.num_triangles()
    }

    /// The parry mesh, shared: a physics collider holds this same copy
    /// (one triangle mesh per model; physics.md "Data model").
    pub fn trimesh(&self) -> &Arc<TriMesh> {
        &self.mesh
    }

    /// The surface of each triangle, in triangle order.
    pub fn surfaces(&self) -> &[SurfaceId] {
        &self.surfaces
    }

    fn surface(&self, triangle: u32) -> SurfaceId {
        self.surfaces.get(triangle as usize).copied().unwrap_or_default()
    }
}

/// A placement's handle in a [`StaticWorld`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StaticId(pub u32);

struct Placed {
    shape: Arc<StaticShape>,
    transform: WorldTransform,
    /// Anchor-relative rigid part.
    pose: Pose,
    /// Uniform scale (non-uniform placements get their own scaled shape).
    scale: f32,
}

impl Placed {
    fn aabb(&self) -> Aabb {
        self.shape.mesh.local_aabb().scaled(Vec3::splat(self.scale)).transform_by(&self.pose)
    }
}

/// Static collision of the resident ring, queried as a [`GroundProbe`].
pub struct StaticWorld {
    anchor: WorldPos,
    bvh: Bvh,
    slots: Vec<Option<Placed>>,
    free: Vec<u32>,
    len: usize,
}

impl StaticWorld {
    pub fn new(anchor: WorldPos) -> Self {
        Self { anchor, bvh: Bvh::new(), slots: Vec::new(), free: Vec::new(), len: 0 }
    }

    pub fn anchor(&self) -> WorldPos {
        self.anchor
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn pose_of(&self, t: &WorldTransform) -> Pose {
        Pose::from_parts(self.anchor.delta_f32(t.pos), t.rotation)
    }

    /// Places `shape` at `transform`. Non-uniform scale bakes a scaled copy.
    pub fn insert(&mut self, shape: &Arc<StaticShape>, transform: WorldTransform) -> StaticId {
        let s = transform.scale;
        let (shape, scale) = if (s.x - s.y).abs() <= 1e-4 * s.x.abs() && (s.x - s.z).abs() <= 1e-4 * s.x.abs() {
            (shape.clone(), s.x)
        } else {
            let scaled = StaticShape { mesh: Arc::new((*shape.mesh).clone().scaled(s)), surfaces: shape.surfaces.clone() };
            (Arc::new(scaled), 1.0)
        };
        let placed = Placed { shape, transform, pose: self.pose_of(&transform), scale };
        let aabb = placed.aabb();
        let index = self.free.pop().unwrap_or_else(|| {
            self.slots.push(None);
            u32::try_from(self.slots.len() - 1).expect("fewer than 2^32 statics")
        });
        self.slots[index as usize] = Some(placed);
        self.bvh.insert(aabb, index);
        self.len += 1;
        StaticId(index)
    }

    /// Removes a placement; false if it wasn't there.
    pub fn remove(&mut self, id: StaticId) -> bool {
        let Some(slot) = self.slots.get_mut(id.0 as usize) else {
            return false;
        };
        if slot.take().is_none() {
            return false;
        }
        self.bvh.remove(id.0);
        self.free.push(id.0);
        self.len -= 1;
        true
    }

    /// Rebuilds the BVH from scratch (binned SAH). Worth it after a large
    /// batch of inserts and removals, e.g. when a cell activates.
    pub fn rebuild(&mut self) {
        let leaves = self.slots.iter().enumerate().filter_map(|(i, p)| p.as_ref().map(|p| (i, p.aabb())));
        self.bvh = Bvh::from_iter(BvhBuildStrategy::Binned, leaves.collect::<Vec<_>>());
    }

    /// Moves the anchor (keep it near the queries) and rebuilds.
    pub fn rebase(&mut self, anchor: WorldPos) {
        self.anchor = anchor;
        for p in self.slots.iter_mut().flatten() {
            p.pose = Pose::from_parts(anchor.delta_f32(p.transform.pos), p.transform.rotation);
        }
        self.rebuild();
    }

    fn ray(&self, q: &ProbeQuery, o: Vec3) -> Option<GroundHit> {
        let ray = Ray::new(o, q.dir);
        let (leaf, hit) = self.bvh.find_best(
            q.max,
            |node, best| node.cast_ray(&ray, best),
            |leaf, best| {
                let p = self.slots[leaf as usize].as_ref()?;
                let local = Ray::new(p.pose.inverse_transform_point(o) / p.scale, p.pose.rotation.inverse() * q.dir);
                let mut h = p.shape.mesh.cast_local_ray_and_get_normal(&local, best / p.scale, true)?;
                h.time_of_impact *= p.scale;
                Some(h)
            },
        )?;
        let p = self.slots[leaf as usize].as_ref()?;
        let mut normal = (p.pose.rotation * hit.normal).normalize_or(Vec3::Y);
        if normal.dot(q.dir) > 0.0 {
            normal = -normal;
        }
        let point = self.anchor.offset(o + q.dir * hit.time_of_impact);
        Some(GroundHit { point, normal, distance: hit.time_of_impact, surface: p.shape.surface(hit.subshape) })
    }

    fn sweep(&self, q: &ProbeQuery, o: Vec3) -> Option<GroundHit> {
        let ray = Ray::new(o, q.dir);
        let grow = Vec3::new(q.radius, q.radius + q.half_height, q.radius);
        let (leaf, hit) = self.bvh.find_best(
            q.max,
            |node, best| {
                let aabb = node.aabb();
                Aabb::new(aabb.mins - grow, aabb.maxs + grow).cast_local_ray(&ray, best, true).unwrap_or(f32::MAX)
            },
            |leaf, best| {
                let p = self.slots[leaf as usize].as_ref()?;
                let s = p.scale;
                let inv = p.pose.rotation.inverse();
                // The query shape in the mesh's (unscaled) frame; its axes stay world-aligned.
                let at = Pose::from_parts(p.pose.inverse_transform_point(o) / s, inv);
                let options = ShapeCastOptions {
                    max_time_of_impact: best / s,
                    target_distance: 0.0,
                    stop_at_penetration: false,
                    compute_impact_geometry_on_penetration: true,
                };
                let ball;
                let capsule;
                let shape: &dyn Shape = if q.half_height > 0.0 {
                    capsule = Capsule::new_y(q.half_height / s, q.radius / s);
                    &capsule
                } else {
                    ball = Ball::new(q.radius / s);
                    &ball
                };
                let mut h = cast_shapes(&at, inv * q.dir, shape, &Pose::IDENTITY, Vec3::ZERO, &*p.shape.mesh, options).ok()??;
                h.time_of_impact *= s;
                Some(h)
            },
        )?;
        let p = self.slots[leaf as usize].as_ref()?;
        // Shape-1 results are in the query's frame, which is world-aligned.
        let normal = (-hit.normal1).normalize_or(-q.dir);
        let point = self.anchor.offset(o + q.dir * hit.time_of_impact + hit.witness1 * p.scale);
        Some(GroundHit { point, normal, distance: hit.time_of_impact, surface: p.shape.surface(hit.subshape2) })
    }
}

impl GroundProbe for StaticWorld {
    fn probe(&self, queries: &[ProbeQuery], out: &mut [Option<GroundHit>]) {
        for (q, o) in queries.iter().zip(out.iter_mut()) {
            let origin = self.anchor.delta_f32(q.origin);
            *o = if self.len == 0 {
                None
            } else if q.radius <= 0.0 {
                self.ray(q, origin)
            } else {
                self.sweep(q, origin)
            };
        }
    }
}
