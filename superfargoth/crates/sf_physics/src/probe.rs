//! `PhysicsProbe`: the controller's [`GroundProbe`] over the engine's query
//! structure, filtered by layer (physics.md "Queries"). It replaces
//! `sf_controller::StaticProbe` (a `StaticWorld`) once statics live here;
//! `StaticWorld` stays the physics-off path and the test oracle.

use glam::Vec3;
use parry3d::bounding_volume::{Aabb, BoundingVolume};
use parry3d::query::{PointQuery, Ray, ShapeCastOptions, ShapeCastStatus};
use parry3d::shape::{Ball, Capsule, FeatureId, Shape};
use rapier3d::geometry::{Collider, ColliderHandle};
use rapier3d::math::Pose;
use rapier3d::pipeline::{QueryFilter, QueryFilterFlags};
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
            // Upright capsules are the movers' wall sweeps: nothing reads their
            // surface, and finding it costs a trimesh point projection per hit
            // (P5, walker probe budget). Spheres (ground casts) keep it.
            let surface = if q.half_height > 0.0 { SurfaceId::DEFAULT } else { self.surface_near(c, hit.witness1) };
            (c, hit.time_of_impact, hit.witness1, hit.normal1, surface)
        };
        normal = normal.normalize_or(-q.dir);
        if normal.dot(q.dir) > 0.0 {
            normal = -normal;
        }
        let body = w.id_of_collider(collider)?;
        let hit = GroundHit::new(w.to_world(point), normal, distance, surface);
        // A moving body's hit carries its pose at the cast: the ride follows the pose delta (IN11c2b).
        let moving = |(v, angular)| {
            let hit = hit.on_body(body.into(), v, angular);
            w.pose(body).map_or(hit, |pose| hit.at_base_pose(pose))
        };
        Some((body, self.moving(body, point).map_or(hit, moving)))
    }

    /// Point velocity at `point` (engine frame) and angular velocity of the
    /// `body`, unless it never moves (fixed, held). Dynamic bodies:
    /// the solver's velocities after the last step. Kinematic bodies: rapier
    /// sets them each step from the target pose delta over the step
    /// (`interpolate_velocity`: linear of the centre of mass, angular as the
    /// delta rotation's scaled axis), so they are the tick's motion, rotation
    /// included, whatever the frame rate.
    fn moving(&self, body: BodyId, point: Vec3) -> Option<(Vec3, Vec3)> {
        let w = self.world;
        let rb = w.engine.bodies.get(w.live(body)?.handle)?;
        (!rb.is_fixed()).then(|| (rb.velocity_at_point(point), rb.angvel()))
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

impl PhysicsProbe<'_> {
    /// How far any awake kinematic body moved in the last step, m (an upper bound: linear
    /// speed plus angular speed times the reach of its colliders from its centre of mass).
    /// It grows each contact query's broad-phase box, so a body that moved through the
    /// capsule this step is still a candidate wherever it ended.
    fn contact_reach(&self, dt: f32) -> f32 {
        let w = self.world;
        let mut reach = 0.0_f32;
        for h in w.engine.islands.active_bodies() {
            let Some(rb) = w.engine.bodies.get(h).filter(|rb| rb.is_kinematic()) else { continue };
            let com = rb.center_of_mass();
            let arm = rb
                .colliders()
                .iter()
                .filter_map(|c| w.engine.colliders.get(*c))
                .map(|c| {
                    let b = c.compute_aabb();
                    (b.center() - com).length() + b.half_extents().length()
                })
                .fold(0.0_f32, f32::max);
            reach = reach.max((rb.linvel().length() + rb.angvel().length() * arm) * dt);
        }
        reach
    }

    /// One [`GroundProbe::contacts`] query against the moving kinematic bodies near it; `reach` from
    /// [`Self::contact_reach`], `dt` the step.
    fn contact(&self, q: &ProbeQuery, reach: f32, dt: f32) -> Option<GroundHit> {
        let w = self.world;
        let c = w.to_local(q.origin);
        let margin = q.max.max(0.0);
        let ball;
        let capsule;
        let shape: &dyn Shape = if q.half_height > 0.0 {
            capsule = Capsule::new_y(q.half_height, q.radius);
            &capsule
        } else {
            ball = Ball::new(q.radius);
            &ball
        };
        let bounds: Aabb = shape.compute_aabb(&Pose::from_translation(c)).loosened(margin + reach);
        let exclude = self.exclude.and_then(|b| w.live(b)).map(|l| l.handle);
        let filter = QueryFilter {
            flags: QueryFilterFlags::ONLY_KINEMATIC,
            groups: Some(self.layers.query_groups()),
            exclude_rigid_body: exclude,
            ..QueryFilter::default()
        };
        let mut best: Option<(f32, Vec3, Vec3, ColliderHandle)> = None;
        for (handle, collider) in w.engine.intersect_aabb_conservative(bounds, filter) {
            let Some((sep, normal, point)) = self.contact_with(collider, shape, c, margin, dt) else { continue };
            if best.is_none_or(|b| sep < b.0) {
                best = Some((sep, normal, point, handle));
            }
        }
        let (sep, normal, point, handle) = best?;
        let body = w.id_of_collider(handle)?;
        let hit = GroundHit::new(w.to_world(point), normal, sep, w.surface(body).unwrap_or_default());
        let (v, angular) = self.moving(body, point)?;
        Some(hit.on_body(body.into(), v, angular))
    }

    /// The capsule `shape` at `c` (engine frame) against one kinematic `collider`: (signed
    /// separation, unit normal facing the capsule, point on the body), when within `margin`.
    ///
    /// In the body's frame the capsule moved by `-v·dt` this step (`v` the body's velocity
    /// at the capsule): a shape cast from `c + v·dt` back to `c` finds where it first came
    /// within `margin`, and the face there is the face the body came in by. Ending clear of
    /// the body on that side, the exact separation and normal at `c` are kept; ending inside
    /// it (or past it), the separation runs on from the touch along that face's normal
    /// (negative: the depth past the face).
    fn contact_with(&self, collider: &Collider, shape: &dyn Shape, c: Vec3, margin: f32, dt: f32) -> Option<(f32, Vec3, Vec3)> {
        let w = self.world;
        let rb = w.engine.bodies.get(collider.parent()?).filter(|rb| rb.is_kinematic())?;
        let v = rb.velocity_at_point(c);
        let speed = v.length();
        let travel = speed * dt;
        if travel <= 1e-6 {
            return None;
        }
        let dir = v / speed;
        let pose = collider.position();
        let body_shape = collider.shape();
        let options = ShapeCastOptions {
            max_time_of_impact: travel,
            target_distance: margin,
            stop_at_penetration: true,
            compute_impact_geometry_on_penetration: true,
        };
        let start = Pose::from_translation(c + v * dt);
        let cast = parry3d::query::cast_shapes(pose, Vec3::ZERO, body_shape, &start, -dir, shape, options).ok()??;
        // The touch: at the cast's time of impact, `margin` away; starting within the margin
        // (or inside), the contact there says how far. A character pushed to the skin last
        // step starts right at the margin (the steady push), so that contact looks a little
        // farther out than the margin and falls back to the cast's own normal.
        let (touch_t, touch_sep, entry) = if cast.status == ShapeCastStatus::PenetratingOrWithinTargetDist || cast.time_of_impact <= 0.0 {
            match parry3d::query::contact(pose, body_shape, &start, shape, margin + travel).ok().flatten() {
                Some(k) => (0.0, k.dist.min(margin), k.normal1),
                None => (0.0, margin, pose.rotation * cast.normal1),
            }
        } else {
            (cast.time_of_impact, margin, pose.rotation * cast.normal1)
        };
        let entry = entry.normalize_or_zero();
        if entry == Vec3::ZERO {
            return None;
        }
        let end = parry3d::query::contact(pose, body_shape, &Pose::from_translation(c), shape, margin).ok().flatten();
        if let Some(k) = end
            && k.dist >= 0.0
            && k.normal1.dot(entry) > 0.0
        {
            return Some((k.dist, k.normal1.normalize_or(entry), k.point1));
        }
        let sep = touch_sep - (travel - touch_t) * dir.dot(entry);
        let point = c - entry * (sep + support(shape, entry));
        (sep <= margin).then_some((sep, entry, point))
    }
}

/// How far the capsule (or ball) reaches from its centre toward `-n`.
fn support(shape: &dyn Shape, n: Vec3) -> f32 {
    if let Some(cap) = shape.as_capsule() {
        cap.radius + cap.half_height() * n.y.abs()
    } else {
        shape.as_ball().map_or(0.0, |b| b.radius)
    }
}

impl GroundProbe for PhysicsProbe<'_> {
    fn probe(&self, queries: &[ProbeQuery], out: &mut [Option<GroundHit>]) {
        for (q, o) in queries.iter().zip(out.iter_mut()) {
            *o = self.cast(q).map(|(_, h)| h);
        }
    }

    /// Moving kinematic bodies (doors, lifts, platforms, carts) in `layers` against upright
    /// capsules (slice `IN11c2a`): per query, a broad-phase box around the capsule grown by
    /// the margin and by the farthest any awake kinematic body moved this step, then per
    /// kinematic candidate a relative shape cast over its motion ([`Self::contact_with`]);
    /// the nearest wins. Fixed bodies never push (the movers' sweeps keep them out), nor
    /// bodies at rest. Dynamic bodies don't either: the solver already bounces them off the
    /// character's kinematic capsule and their hit reaches gameplay as an impact
    /// (`PhysicsEvent::Hit`), so a positional push on top would shove the character as if a
    /// thrown ball were a wall and hit it twice (Jolt `CharacterVirtual` leaves this to the
    /// app, `mCanPushCharacter`). Characters' capsules are outside the movers' layers.
    fn contacts(&self, queries: &[ProbeQuery], out: &mut [Option<GroundHit>]) {
        let dt = self.world.params().dt();
        let reach = self.contact_reach(dt);
        for (q, o) in queries.iter().zip(out.iter_mut()) {
            *o = self.contact(q, reach, dt);
        }
    }
}
