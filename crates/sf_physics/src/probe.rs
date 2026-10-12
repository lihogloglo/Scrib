//! `PhysicsProbe`: the controller's [`GroundProbe`] over the engine's query
//! structure, filtered by layer (physics.md "Queries"). It replaces
//! `sf_controller::StaticProbe` (a `StaticWorld`) once statics live here;
//! `StaticWorld` stays the physics-off path and the test oracle.

use glam::Vec3;
use parry3d::query::{PointQuery, Ray, ShapeCastOptions};
use parry3d::shape::{Ball, Capsule, FeatureId, Shape};
use rapier3d::geometry::ColliderHandle;
use rapier3d::math::Pose;
use rapier3d::pipeline::QueryFilter;
use sf_controller::{GroundHit, GroundProbe, ProbeQuery, SurfaceId};

use crate::body::BodyId;
use crate::layer::Layer;
use crate::world::PhysicsWorld;

/// Casts against the bodies of a [`PhysicsWorld`] whose layer is in `layers`.
/// Excludes one body when set (the caster's own capsule, a held item).
#[derive(Clone, Copy)]
pub struct PhysicsProbe<'a> {
    world: &'a PhysicsWorld,
    layers: Layer,
    exclude: Option<BodyId>,
}

impl PhysicsWorld {
    /// A probe over the bodies in `layers`. Call
    /// [`PhysicsWorld::refresh_queries`] after inserts made since the last step.
    pub fn probe(&self, layers: Layer) -> PhysicsProbe<'_> {
        PhysicsProbe { world: self, layers, exclude: None }
    }
}

impl PhysicsProbe<'_> {
    #[must_use]
    pub fn excluding(mut self, body: BodyId) -> Self {
        self.exclude = Some(body);
        self
    }

    /// The nearest hit and the body it belongs to (picks, activation, hit tests).
    pub fn cast(&self, q: &ProbeQuery) -> Option<(BodyId, GroundHit)> {
        let w = self.world;
        let exclude = self.exclude.and_then(|b| w.live(b)).map(|l| l.handle);
        let filter = QueryFilter { groups: Some(self.layers.query_groups()), exclude_rigid_body: exclude, ..QueryFilter::default() };
        let o = w.to_local(q.origin);
        let (collider, distance, point, mut normal, surface) = if q.radius <= 0.0 {
            let ray = Ray::new(o, q.dir);
            let (c, hit) = w.engine.cast_ray_and_get_normal(&ray, q.max, true, filter)?;
            let surface = self.mesh_surface(c, hit.subshape);
            (c, hit.time_of_impact, o + q.dir * hit.time_of_impact, hit.normal, surface)
        } else {
            let ball;
            let capsule;
            let shape: &dyn Shape = if q.half_height > 0.0 {
                capsule = Capsule::new_y(q.half_height, q.radius);
                &capsule
            } else {
                ball = Ball::new(q.radius);
                &ball
            };
            let options = ShapeCastOptions {
                max_time_of_impact: q.max,
                target_distance: 0.0,
                stop_at_penetration: false,
                compute_impact_geometry_on_penetration: true,
            };
            let (c, hit) = w.engine.cast_shape(&Pose::from_translation(o), q.dir, shape, options, filter)?;
            // Composite (the world) first: witness1/normal1 are on the hit collider, in the engine frame.
            let surface = self.surface_near(c, hit.witness1);
            (c, hit.time_of_impact, hit.witness1, hit.normal1, surface)
        };
        normal = normal.normalize_or(-q.dir);
        if normal.dot(q.dir) > 0.0 {
            normal = -normal;
        }
        let body = w.id_of_collider(collider)?;
        Some((body, GroundHit { point: w.to_world(point), normal, distance, surface }))
    }

    fn mesh_surface(&self, c: ColliderHandle, triangle: u32) -> SurfaceId {
        let w = self.world;
        w.id_of_collider(c).and_then(|b| w.live(b)?.mesh.as_ref().map(|m| m.surface(triangle))).unwrap_or_default()
    }

    /// The surface of the mesh triangle nearest `point` (sweeps report the collider, not the triangle).
    fn surface_near(&self, c: ColliderHandle, point: Vec3) -> SurfaceId {
        let w = self.world;
        let Some(collider) = w.engine.colliders.get(c) else {
            return SurfaceId::DEFAULT;
        };
        let Some(mesh) = collider.shape().as_trimesh() else {
            return SurfaceId::DEFAULT;
        };
        let local = collider.position().inverse_transform_point(point);
        match mesh.project_local_point_and_get_feature(local).1 {
            FeatureId::Face(t) => {
                #[allow(clippy::cast_possible_truncation)]
                let n = mesh.num_triangles() as u32;
                self.mesh_surface(c, t % n.max(1))
            }
            _ => SurfaceId::DEFAULT,
        }
    }
}

impl GroundProbe for PhysicsProbe<'_> {
    fn probe(&self, queries: &[ProbeQuery], out: &mut [Option<GroundHit>]) {
        for (q, o) in queries.iter().zip(out.iter_mut()) {
            *o = self.cast(q).map(|(_, h)| h);
        }
    }
}
