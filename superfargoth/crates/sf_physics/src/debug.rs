//! Collider wireframes for debug drawing, culled to a sphere around a point.
//!
//! Rapier's `DebugRenderPipeline` draws every collider whole; an R0 holds
//! about a million triangles of static meshes plus 129² heightfield chunks,
//! so the debug view culls per triangle instead.

use glam::Vec3;
use parry3d::math::Pose;
use parry3d::shape::Shape;
use sf_coords::WorldPos;

use crate::body::BodyKind;
use crate::layer::Layer;
use crate::world::PhysicsWorld;

impl PhysicsWorld {
    /// Appends the edges of every collider triangle within `radius` meters of
    /// `center` (meshes, heightfields, hulls, primitives and their compounds; other shapes as their bounding boxes),
    /// as segments in meters relative to `frame` (e.g. the render origin),
    /// with the body's layer. Moving bodies are drawn at their interpolated pose
    /// (`alpha` as in [`PhysicsWorld::pose_at`]), where the renderer shows them,
    /// so they don't beat against frame-rate visuals; fixed ones where they are.
    /// Stops at `max` segments; returns how many it added.
    pub fn debug_lines(&self, center: WorldPos, radius: f32, frame: WorldPos, max: usize, alpha: f32, out: &mut Vec<(Vec3, Vec3, Layer)>) -> usize {
        let start = out.len();
        let c = self.to_local(center);
        let base = frame.delta_f32(self.anchor());
        let r2 = radius * radius;
        let near = |p: Vec3| p.distance_squared(c) <= r2;
        for (_, collider) in self.engine.colliders.iter() {
            let aabb = collider.compute_aabb();
            if c.clamp(aabb.mins, aabb.maxs).distance_squared(c) > r2 {
                continue;
            }
            let live = u32::try_from(collider.user_data).ok().and_then(|i| self.id_at(i)).and_then(|id| self.live(id));
            let layer = live.map_or(Layer::STATIC, |l| l.layer);
            let moving = live.filter(|l| !matches!(l.kind, BodyKind::Fixed));
            let interpolated = moving.map(|l| {
                let body = Pose::from_parts(l.prev.pos.lerp(l.curr.pos, alpha), l.prev.rot.slerp(l.curr.rot, alpha));
                collider.position_wrt_parent().map_or(body, |rel| body * *rel)
            });
            let pose = interpolated.as_ref().unwrap_or_else(|| collider.position());
            let mut tri = |a: Vec3, b: Vec3, cc: Vec3| {
                let (a, b, cc) = (pose.transform_point(a), pose.transform_point(b), pose.transform_point(cc));
                if near(a) || near(b) || near(cc) {
                    out.push((a + base, b + base, layer));
                    out.push((b + base, cc + base, layer));
                    out.push((cc + base, a + base, layer));
                }
            };
            let shape = collider.shape();
            if let Some(mesh) = shape.as_trimesh() {
                for t in mesh.triangles() {
                    tri(t.a, t.b, t.c);
                }
            } else if let Some(hf) = shape.as_heightfield() {
                for t in hf.triangles() {
                    tri(t.a, t.b, t.c);
                }
            } else if let Some(parts) = convex_parts(shape) {
                for (part_pose, (vertices, triangles)) in parts {
                    for t in triangles {
                        let [a, b, cc] = t.map(|i| part_pose.transform_point(vertices[i as usize]));
                        tri(a, b, cc);
                    }
                }
            } else {
                let (lo, hi) = (aabb.mins + base, aabb.maxs + base);
                for i in 0..4u8 {
                    let (x0, z0) = (if i & 1 == 0 { lo.x } else { hi.x }, if i & 2 == 0 { lo.z } else { hi.z });
                    out.push((Vec3::new(x0, lo.y, z0), Vec3::new(x0, hi.y, z0), layer));
                }
                for y in [lo.y, hi.y] {
                    out.push((Vec3::new(lo.x, y, lo.z), Vec3::new(hi.x, y, lo.z), layer));
                    out.push((Vec3::new(hi.x, y, lo.z), Vec3::new(hi.x, y, hi.z), layer));
                    out.push((Vec3::new(hi.x, y, hi.z), Vec3::new(lo.x, y, hi.z), layer));
                    out.push((Vec3::new(lo.x, y, hi.z), Vec3::new(lo.x, y, lo.z), layer));
                }
            }
            if out.len() - start >= max {
                out.truncate(start + max);
                break;
            }
        }
        out.len() - start
    }

    /// Appends segments for every impulse joint (ragdoll joints) whose anchor
    /// is within `radius` of `center`, in meters relative to `frame`: parent
    /// body origin to the anchor, the anchor to the child body origin, and a
    /// small cross at the anchor along the joint's frame (its X axis is the
    /// twist axis). Bodies at their interpolated poses as in
    /// [`Self::debug_lines`]. Multibody joints are not drawn (nothing uses them
    /// for characters). Returns how many segments it added.
    pub fn debug_joint_lines(&self, center: WorldPos, radius: f32, frame: WorldPos, alpha: f32, out: &mut Vec<(Vec3, Vec3)>) -> usize {
        const CROSS: f32 = 0.04;
        let start = out.len();
        let c = self.to_local(center);
        let base = frame.delta_f32(self.anchor());
        let pose_of = |h| {
            let rb = self.engine.bodies.get(h)?;
            let live = u32::try_from(rb.user_data).ok().and_then(|i| self.id_at(i)).and_then(|id| self.live(id));
            Some(live.map_or(*rb.position(), |l| Pose::from_parts(l.prev.pos.lerp(l.curr.pos, alpha), l.prev.rot.slerp(l.curr.rot, alpha))))
        };
        for (_, joint) in self.engine.impulse_joints.iter() {
            let (Some(p1), Some(p2)) = (pose_of(joint.body1()), pose_of(joint.body2())) else { continue };
            let f1 = joint.data.local_frame1;
            let anchor = p1.transform_point(f1.translation);
            if anchor.distance_squared(c) > radius * radius {
                continue;
            }
            out.push((p1.translation + base, anchor + base));
            out.push((anchor + base, p2.translation + base));
            let q = p1.rotation * f1.rotation;
            for axis in [Vec3::X, Vec3::Y, Vec3::Z] {
                let d = q * axis * CROSS;
                out.push((anchor - d + base, anchor + d + base));
            }
        }
        out.len() - start
    }
}

type Tris = (Vec<Vec3>, Vec<[u32; 3]>);

/// Triangles of a convex shape (hull, box, ball, capsule, cylinder) or of each
/// part of a compound of them, with each part's pose in the collider.
fn convex_parts(shape: &dyn Shape) -> Option<Vec<(Pose, Tris)>> {
    let one = |s: &dyn Shape| -> Option<Tris> {
        s.as_convex_polyhedron()
            .map(parry3d::shape::ConvexPolyhedron::to_trimesh)
            .or_else(|| s.as_cuboid().map(parry3d::shape::Cuboid::to_trimesh))
            .or_else(|| s.as_ball().map(|b| b.to_trimesh(6, 8)))
            .or_else(|| s.as_capsule().map(|c| c.to_trimesh(4, 8)))
            .or_else(|| s.as_cylinder().map(|c| c.to_trimesh(8)))
    };
    if let Some(compound) = shape.as_compound() {
        return Some(compound.shapes().iter().filter_map(|(pose, s)| Some((*pose, one(s.as_ref())?))).collect());
    }
    Some(vec![(Pose::IDENTITY, one(shape)?)])
}
