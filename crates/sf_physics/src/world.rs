//! The physics world of one worldspace: rapier state in an f32 frame around
//! an anchor grid cell, rebased by whole cells (physics.md "Stepping and the
//! floating origin").

use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::sync::{Arc, Mutex};

use glam::{IVec3, Quat, Vec3};
use rapier3d::dynamics::{LockedAxes, RigidBody, RigidBodyActivation, RigidBodyBuilder, RigidBodyHandle, RigidBodySet, RigidBodyType, SoftBodySet, SoftBodyTearEvent};
use rapier3d::geometry::{ColliderBuilder, ColliderHandle, ColliderSet, CollisionEvent, ContactPair};
use rapier3d::math::Pose;
use rapier3d::parry::mass_properties::MassProperties;
use rapier3d::pipeline::{ActiveEvents, ActiveHooks, EventHandler, PairFilterContext, PhysicsHooks, PhysicsWorld as Engine};
use rapier3d::geometry::SolverFlags;
use sf_controller::SurfaceId;
use sf_coords::{GRID_CELL, WorldPos, WorldTransform};

use crate::body::{BodyDesc, BodyId, BodyKind, PhysicsEvent, PhysicsParams};
use crate::layer::Layer;
use crate::shape::{MeshShape, ShapeRef};

/// A rigid pose in the engine frame.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Local {
    pub pos: Vec3,
    pub rot: Quat,
}

impl Local {
    fn of(rb: &RigidBody) -> Self {
        let p = rb.position();
        Self { pos: p.translation, rot: p.rotation }
    }

    fn shifted(self, d: Vec3) -> Self {
        Self { pos: self.pos - d, rot: self.rot }
    }
}

pub(crate) struct Live {
    pub handle: RigidBodyHandle,
    pub kind: BodyKind,
    pub layer: Layer,
    pub user: u64,
    pub surface: SurfaceId,
    pub scale: Vec3,
    pub mesh: Option<Arc<MeshShape>>,
    /// Interpolation history: the poses after the last two steps.
    pub prev: Local,
    pub curr: Local,
    /// Where a dynamic body last came to rest (fell-out recovery).
    pub rest: Local,
    pub awake: bool,
    /// Inserted (or recovered) asleep, not yet asleep in the engine: frozen, no events.
    pub settling: bool,
    /// The mass a grab decides carry or drag by (a ragdoll limb: the whole ragdoll).
    pub mode_mass: Option<f32>,
    seen: u64,
}

struct Slot {
    generation: u32,
    live: Option<Live>,
}

/// Colliders, total force, deepest point and normal (engine frame).
type ContactReport = (ColliderHandle, ColliderHandle, f32, Vec3, Vec3);

/// Hit-reporting collider, the dynamic collider that hit it, impact
/// momentum (N·s), contact point and normal toward the target (engine frame).
type HitReport = (ColliderHandle, ColliderHandle, f32, Vec3, Vec3);

/// Contact-force and hit reports gathered during a step (rapier calls back through `&self`).
#[derive(Default)]
struct Collector {
    contacts: Mutex<Vec<ContactReport>>,
    hits: Mutex<Vec<HitReport>>,
    /// 1 / the step's dt (a kinematic body's velocity this step).
    inv_dt: f32,
}

impl Collector {
    /// [`PhysicsEvent::Hit`] at a pair's first contact. Rapier detects
    /// collisions at the start of a step, before the solver, so `bodies`
    /// still carry the approach velocities (and CCD substeps clamp positions,
    /// not velocities).
    fn hit(&self, bodies: &RigidBodySet, colliders: &ColliderSet, pair: &ContactPair) {
        let Some((manifold, contact)) = pair.find_deepest_contact() else { return };
        let (Some(c1), Some(c2)) = (colliders.get(pair.collider1), colliders.get(pair.collider2)) else { return };
        let point = c1.position().transform_point(contact.local_p1);
        let normal = manifold.data.normal; // from collider 1 to 2
        let body = |c: &rapier3d::geometry::Collider| c.parent().and_then(|h| bodies.get(h));
        // Either side may report hits; the other side must be dynamic.
        for (target, by, toward) in [(c1, c2, -normal), (c2, c1, normal)] {
            if !target.active_events().contains(ActiveEvents::COLLISION_EVENTS) {
                continue;
            }
            let Some(other) = body(by).filter(|rb| rb.is_dynamic()) else { continue };
            // A kinematic target's velocity this step is toward its pending target pose:
            // rapier derives its `linvel` from that only after collision detection.
            let target_velocity = body(target).map_or(Vec3::ZERO, |rb| {
                if rb.is_kinematic() { (rb.next_position().translation - rb.position().translation) * self.inv_dt } else { rb.velocity_at_point(point) }
            });
            let impulse = crate::impact_impulse(other.mass(), other.velocity_at_point(point), target_velocity, toward);            if impulse > 0.0 {
                let (t, b) = if std::ptr::eq(target, c1) { (pair.collider1, pair.collider2) } else { (pair.collider2, pair.collider1) };
                self.hits.lock().expect("collector poisoned").push((t, b, impulse, point, toward));
            }
        }
    }
}

impl EventHandler for Collector {
    fn handle_collision_event(&self, bodies: &RigidBodySet, colliders: &ColliderSet, event: CollisionEvent, pair: Option<&ContactPair>) {
        if let (CollisionEvent::Started(..), Some(pair)) = (event, pair) {
            self.hit(bodies, colliders, pair);
        }
    }

    fn handle_contact_force_event(
        &self,
        _dt: f32,
        _bodies: &RigidBodySet,
        colliders: &ColliderSet,
        pair: &ContactPair,
        total_force_magnitude: f32,
    ) {
        // The deepest contact point, in the engine frame (contacts are collider-local),
        // and its manifold's normal (world, from collider 1 to 2).
        let normal = pair.find_deepest_contact().map_or(Vec3::ZERO, |(m, _)| m.data.normal);
        let point = colliders.get(pair.collider1).map_or(Vec3::ZERO, |c| match pair.find_deepest_contact() {
            Some((_, contact)) => c.position().transform_point(contact.local_p1),
            None => c.translation(),
        });
        self.contacts.lock().expect("collector poisoned").push((pair.collider1, pair.collider2, total_force_magnitude, point, normal));
    }

    fn handle_soft_body_tear_event(&self, _: &SoftBodySet, _: &SoftBodyTearEvent) {}
}

/// The physics world of one worldspace.
///
/// Engine positions are f32 offsets from the anchor cell's center; every
/// public position is a [`WorldPos`]. [`PhysicsWorld::rebase`] moves the
/// anchor by whole cells, which shifts every engine position by a multiple
/// of 256 m: exact for positions that end up nearer the anchor, and the
/// only rounding is one half-ulp at the new distance otherwise.
pub struct PhysicsWorld {
    pub(crate) engine: Engine,
    /// Colliders inserted since the last step, not yet in the broad-phase BVH
    /// (see `refresh_queries`).
    query_pending: Vec<ColliderHandle>,
    /// Every collider's broad-phase AABB is stale (a rebase since the last step).
    query_all: bool,
    anchor: IVec3,
    params: PhysicsParams,
    slots: Vec<Slot>,
    free: Vec<u32>,
    len: usize,
    awake: Vec<u32>,
    epoch: u64,
    events: Vec<PhysicsEvent>,
    /// Colliders inserted or moved since the query structure was last refreshed.
    queries_dirty: bool,
    /// Lowest point of any fixed body (absolute Y): the automatic kill height
    /// sits `kill_margin` below it.
    floor: Option<f64>,
    /// Bodies inserted asleep whose first contacts haven't been detected yet.
    settling: Vec<BodyId>,
    /// Collider pairs that never touch (a held item and its holder).
    excluded: Vec<(ColliderHandle, ColliderHandle)>,
}

impl PhysicsWorld {
    /// A world anchored at the cell containing `anchor`.
    pub fn new(anchor: WorldPos, params: PhysicsParams) -> Self {
        let mut engine = Engine::new();
        engine.gravity = params.gravity;
        engine.integration_parameters.dt = params.dt;
        engine.integration_parameters.normalized_max_corrective_velocity = params.max_corrective_velocity;
        Self {
            engine,
            anchor: anchor.cell,
            params,
            slots: Vec::new(),
            free: Vec::new(),
            len: 0,
            awake: Vec::new(),
            epoch: 0,
            events: Vec::new(),
            queries_dirty: false,
            settling: Vec::new(),
            query_pending: Vec::new(),
            query_all: false,
            floor: None,
            excluded: Vec::new(),
        }
    }

    pub fn params(&self) -> &PhysicsParams {
        &self.params
    }

    /// Absolute Y under which a dynamic body counts as fallen out:
    /// [`PhysicsParams::kill_height`] if set, else `kill_margin` below the
    /// lowest fixed body inserted so far (statics and terrain; it never rises,
    /// so removals keep it conservative). `None` while the world has no fixed body.
    pub fn kill_height(&self) -> Option<f64> {
        self.params.kill_height.or_else(|| self.floor.map(|f| f - f64::from(self.params.kill_margin)))
    }

    /// The anchor: the center of the anchor cell.
    pub fn anchor(&self) -> WorldPos {
        WorldPos { cell: self.anchor, local: Vec3::ZERO }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Dynamic bodies awake after the last step.
    pub fn awake_count(&self) -> usize {
        self.awake.len()
    }

    pub(crate) fn to_local(&self, p: WorldPos) -> Vec3 {
        self.anchor().delta_f32(p)
    }

    pub(crate) fn to_world(&self, p: Vec3) -> WorldPos {
        self.anchor().offset(p)
    }

    pub(crate) fn live(&self, id: BodyId) -> Option<&Live> {
        let slot = self.slots.get(id.index as usize)?;
        if slot.generation != id.generation {
            return None;
        }
        slot.live.as_ref()
    }

    fn live_mut(&mut self, id: BodyId) -> Option<&mut Live> {
        let slot = self.slots.get_mut(id.index as usize)?;
        if slot.generation != id.generation {
            return None;
        }
        slot.live.as_mut()
    }

    /// The id of the body in slot `index` (from engine user data).
    pub(crate) fn id_at(&self, index: u32) -> Option<BodyId> {
        let slot = self.slots.get(index as usize)?;
        slot.live.as_ref().map(|_| BodyId { index, generation: slot.generation })
    }

    pub(crate) fn id_of_collider(&self, c: ColliderHandle) -> Option<BodyId> {
        let index = u32::try_from(self.engine.colliders.get(c)?.user_data).ok()?;
        self.id_at(index)
    }

    /// Inserts a body.
    pub fn insert(&mut self, desc: &BodyDesc) -> BodyId {
        let index = self.free.pop().unwrap_or_else(|| {
            self.slots.push(Slot { generation: 0, live: None });
            u32::try_from(self.slots.len() - 1).expect("fewer than 2^32 bodies")
        });
        let local = Local { pos: self.to_local(desc.pose.pos), rot: desc.pose.rotation };
        let pose = Pose::from_parts(local.pos, local.rot);
        let p = &self.params;
        let rb = match desc.kind {
            BodyKind::Fixed => RigidBodyBuilder::fixed(),
            BodyKind::Held => RigidBodyBuilder::fixed().linear_damping(desc.damping.0).angular_damping(desc.damping.1),
            BodyKind::Kinematic => RigidBodyBuilder::kinematic_position_based(),
            BodyKind::Dynamic { asleep } => RigidBodyBuilder::dynamic()
                .sleeping(asleep)
                .ccd_enabled(desc.ccd)
                .linear_damping(desc.damping.0)
                .angular_damping(desc.damping.1)
                .additional_solver_iterations(desc.extra_iterations),
        }
        .pose(pose)
        .user_data(u128::from(index));

        let shape = desc.shape.shared(desc.pose.scale);
        let mut cb = ColliderBuilder::new(shape.clone())
            .collision_groups(desc.layer.groups())
            .sensor(desc.layer == Layer::SENSOR)
            .user_data(u128::from(index));
        if matches!(desc.kind, BodyKind::Dynamic { .. } | BodyKind::Held) {
            let mass = desc.mass.unwrap_or_else(|| shape.mass_properties(p.density).mass());
            // Ragdoll bodies keep their cooked mass (a guar's torso is far over the item range).
            let mass = if desc.layer.contains(Layer::RAGDOLL) { mass.max(0.01) } else { mass.clamp(p.mass_range.0, p.mass_range.1) };
            if desc.min_inertia > 0.0 {
                let mut mp = shape.mass_properties(1.0);
                mp.set_mass(mass, true);
                let inertia = mp.principal_inertia().max(Vec3::splat(desc.min_inertia));
                cb = cb.mass_properties(MassProperties::with_principal_inertia_frame(mp.local_com, mass, inertia, mp.principal_inertia_local_frame));
            } else {
                cb = cb.mass(mass);
            }
            let aabb = shape.compute_local_aabb();
            if (aabb.maxs - aabb.mins).min_element() < p.thin_extent {
                cb = cb.contact_skin(p.thin_skin);
            }
        }
        let mut events = ActiveEvents::empty();
        if desc.layer.intersects(p.contact_force_layers) {
            events |= ActiveEvents::CONTACT_FORCE_EVENTS;
            cb = cb.contact_force_event_threshold(p.contact_force_threshold);
        }
        if desc.layer.intersects(p.hit_layers) {
            events |= ActiveEvents::COLLISION_EVENTS;
        }
        cb = cb.active_events(events);
        let (handle, collider) = self.engine.insert(rb, cb);
        self.query_pending.push(collider);
        if desc.kind == BodyKind::Fixed {
            let low = f64::from(shape.compute_aabb(&pose).mins.y) + self.anchor().to_f64().y;
            self.floor = Some(self.floor.map_or(low, |f| f.min(low)));
        }
        let mesh = match &desc.shape {
            ShapeRef::Mesh(m) => Some(m.clone()),
            _ => None,
        };
        let slot = &mut self.slots[index as usize];
        slot.live = Some(Live {
            handle,
            kind: desc.kind,
            layer: desc.layer,
            user: desc.user,
            surface: desc.surface,
            scale: desc.pose.scale,
            mesh,
            prev: local,
            curr: local,
            rest: local,
            awake: false,
            settling: false,
            mode_mass: None,
            seen: 0,
        });
        self.len += 1;
        self.queries_dirty = true;
        let id = BodyId { index, generation: slot.generation };
        if desc.kind == (BodyKind::Dynamic { asleep: true }) {
            self.begin_settle(id);
        }
        id
    }

    /// Removes a body (and its joints); false if it wasn't there.
    pub fn remove(&mut self, id: BodyId) -> bool {
        let Some(live) = self.live_mut(id).map(|l| l.handle) else {
            return false;
        };
        if !self.excluded.is_empty() {
            let gone = self.colliders_of(id);
            self.excluded.retain(|(x, y)| !gone.contains(x) && !gone.contains(y));
        }
        self.engine.remove_body(live);
        let slot = &mut self.slots[id.index as usize];
        slot.live = None;
        slot.generation = slot.generation.wrapping_add(1);
        self.free.push(id.index);
        self.awake.retain(|&i| i != id.index);
        self.len -= 1;
        self.queries_dirty = true;
        true
    }

    pub fn contains(&self, id: BodyId) -> bool {
        self.live(id).is_some()
    }

    pub fn user(&self, id: BodyId) -> Option<u64> {
        self.live(id).map(|l| l.user)
    }

    /// A body's own surface ([`BodyDesc::surface`]).
    pub fn surface(&self, id: BodyId) -> Option<SurfaceId> {
        self.live(id).map(|l| l.surface)
    }

    /// Retags a body's surface (later [`PhysicsEvent::Contact`]s carry it).
    pub fn set_surface(&mut self, id: BodyId, surface: SurfaceId) -> bool {
        self.live_mut(id).map(|l| l.surface = surface).is_some()
    }

    pub fn layer(&self, id: BodyId) -> Option<Layer> {
        self.live(id).map(|l| l.layer)
    }

    /// Simulated mass, kg (0 for fixed and kinematic bodies).
    pub fn mass(&self, id: BodyId) -> Option<f32> {
        let l = self.live(id)?;
        Some(self.engine.bodies.get(l.handle)?.mass())
    }

    /// The mass a grab of this body decides carry, drag or too heavy by:
    /// [`Self::set_mode_mass`] if set (a ragdoll limb answers for the whole
    /// ragdoll), else the simulated mass.
    pub fn mode_mass(&self, id: BodyId) -> Option<f32> {
        match self.live(id)?.mode_mass {
            Some(m) => Some(m),
            None => self.mass(id),
        }
    }

    pub fn set_mode_mass(&mut self, id: BodyId, mass: Option<f32>) -> bool {
        let Some(l) = self.live_mut(id) else { return false };
        l.mode_mass = mass;
        true
    }

    /// The pose after the last step.
    pub fn pose(&self, id: BodyId) -> Option<WorldTransform> {
        let l = self.live(id)?;
        Some(WorldTransform { pos: self.to_world(l.curr.pos), rotation: l.curr.rot, scale: l.scale })
    }

    /// The pose interpolated between the last two steps; `alpha` is the
    /// fixed-step overstep fraction (`Time<Fixed>::overstep_fraction`).
    pub fn pose_at(&self, id: BodyId, alpha: f32) -> Option<WorldTransform> {
        let l = self.live(id)?;
        let pos = l.prev.pos.lerp(l.curr.pos, alpha);
        let rotation = l.prev.rot.slerp(l.curr.rot, alpha);
        Some(WorldTransform { pos: self.to_world(pos), rotation, scale: l.scale })
    }

    /// Linear and angular velocity.
    pub fn velocity(&self, id: BodyId) -> Option<(Vec3, Vec3)> {
        let rb = self.engine.bodies.get(self.live(id)?.handle)?;
        Some((rb.linvel(), rb.angvel()))
    }

    pub fn is_sleeping(&self, id: BodyId) -> Option<bool> {
        Some(self.engine.bodies.get(self.live(id)?.handle)?.is_sleeping())
    }

    /// Teleports a body (no sweep); it keeps its sleep state.
    pub fn set_pose(&mut self, id: BodyId, pose: WorldTransform) -> bool {
        let pos = self.to_local(pose.pos);
        let Some(l) = self.live_mut(id) else {
            return false;
        };
        let local = Local { pos, rot: pose.rotation };
        l.prev = local;
        l.curr = local;
        l.rest = local;
        let handle = l.handle;
        if let Some(rb) = self.engine.bodies.get_mut(handle) {
            rb.set_position(Pose::from_parts(local.pos, local.rot), false);
            self.query_pending.extend_from_slice(rb.colliders());
        }
        self.queries_dirty = true;
        true
    }

    /// Sets a dynamic body's velocities (waking it).
    pub fn set_velocity(&mut self, id: BodyId, linear: Vec3, angular: Vec3) -> bool {
        self.unsettle(id);
        let Some(h) = self.live(id).map(|l| l.handle) else {
            return false;
        };
        let Some(rb) = self.engine.bodies.get_mut(h) else {
            return false;
        };
        rb.set_linvel(linear, true);
        rb.set_angvel(angular, true);
        true
    }

    /// Makes a [`BodyKind::Held`] body an awake dynamic one (grabbed, or
    /// impulsed by a script or spell); false for any other body.
    pub fn release(&mut self, id: BodyId) -> bool {
        let Some(l) = self.live_mut(id).filter(|l| l.kind == BodyKind::Held) else {
            return false;
        };
        l.kind = BodyKind::Dynamic { asleep: false };
        let handle = l.handle;
        if let Some(rb) = self.engine.bodies.get_mut(handle) {
            rb.set_body_type(RigidBodyType::Dynamic, true);
            rb.enable_ccd(true);
        }
        self.queries_dirty = true;
        true
    }

    /// Applies an impulse (N·s) at a world point, or at the center of mass.
    /// A held body is released first.
    pub fn apply_impulse(&mut self, id: BodyId, impulse: Vec3, at: Option<WorldPos>) -> bool {
        self.release(id);
        self.unsettle(id);
        let Some(h) = self.live(id).map(|l| l.handle) else {
            return false;
        };
        let at = at.map(|p| self.to_local(p));
        let Some(rb) = self.engine.bodies.get_mut(h) else {
            return false;
        };
        match at {
            Some(p) => rb.apply_impulse_at_point(impulse, p, true),
            None => rb.apply_impulse(impulse, true),
        }
        true
    }

    pub fn wake(&mut self, id: BodyId) {
        self.unsettle(id);
        if let Some(h) = self.live(id).map(|l| l.handle) {
            self.engine.wake_up(h, true);
        }
    }

    /// Where a kinematic body should be at the end of the next step.
    pub fn set_kinematic_target(&mut self, id: BodyId, target: WorldTransform) -> bool {
        let pos = self.to_local(target.pos);
        let Some(h) = self.live(id).filter(|l| l.kind == BodyKind::Kinematic).map(|l| l.handle) else {
            return false;
        };
        let Some(rb) = self.engine.bodies.get_mut(h) else {
            return false;
        };
        rb.set_next_kinematic_position(Pose::from_parts(pos, target.rotation));
        true
    }

    /// Refreshes the query structure after inserts, removals or teleports
    /// without stepping (an activation batch before the first step). Steps
    /// refresh it anyway.
    ///
    /// Only the broad-phase BVH (what queries read) is touched: each collider
    /// inserted since the last step gets its AABB set there. Bodies, islands,
    /// contacts and the solver graph are left to the next step. Rapier's
    /// collision-only pass (`detect_collisions`) must never run on this world:
    /// it consumes body changes without the island manager while still linking
    /// contacts into the persistent islands and the solver graph, which breaks
    /// their invariants (the 2026-10-19 `member(link.body1) || member(link.body2)`
    /// panic; physics.md "Query refresh"). Removed colliders keep their stale
    /// leaf until the step; queries skip it (the collider is gone). Teleported
    /// and rebased bodies get their collider poses synced first (rapier's
    /// query-only helper); kinematic targets are seen after the next step.
    pub fn refresh_queries(&mut self) {
        if !self.queries_dirty {
            return;
        }
        let e = &mut self.engine;
        e.bodies.propagate_modified_body_positions_to_colliders(&mut e.colliders);
        if std::mem::take(&mut self.query_all) {
            self.query_pending.clear();
            self.query_pending.extend(e.colliders.iter().map(|(h, _)| h));
        }
        for h in self.query_pending.drain(..) {
            let Some(co) = e.colliders.get(h).filter(|c| c.is_enabled()) else { continue };
            let aabb = co.compute_broad_phase_aabb(&e.integration_parameters, &e.bodies);
            if aabb.mins.is_finite() && aabb.maxs.is_finite() {
                e.broad_phase.set_aabb(&e.integration_parameters, h, aabb);
            }
        }
        self.queries_dirty = false;
    }

    /// Freezes a body that must start asleep until the engine has put it to
    /// sleep with its first contacts in place.
    ///
    /// Rapier wakes both sides of every new contact, and a placed item resting
    /// on a table is a new contact the moment it is inserted. So the body
    /// goes in with every axis locked (it takes part in contacts but can't
    /// move) and a zero sleep delay: the first step wakes it, it stays exactly
    /// where it was placed, and the engine's own sleep pass puts it (and its
    /// island) back to sleep in the same step. Then it is unlocked. A body
    /// whose island stays awake (something is touching it) is released awake.
    /// physics.md "Sleeping from the start".
    fn begin_settle(&mut self, id: BodyId) {
        let Some(l) = self.live_mut(id) else { return };
        l.settling = true;
        let handle = l.handle;
        if let Some(rb) = self.engine.bodies.get_mut(handle) {
            rb.set_locked_axes(LockedAxes::all(), false);
            rb.activation_mut().time_until_sleep = 0.0;
        }
        self.settling.push(id);
    }

    /// Releases one settling body now (the owner disturbed it).
    fn unsettle(&mut self, id: BodyId) {
        let Some(l) = self.live_mut(id).filter(|l| l.settling) else { return };
        l.settling = false;
        let handle = l.handle;
        if let Some(rb) = self.engine.bodies.get_mut(handle) {
            rb.set_locked_axes(LockedAxes::empty(), false);
            rb.activation_mut().time_until_sleep = RigidBodyActivation::default_time_until_sleep();
        }
        self.settling.retain(|s| *s != id);
    }

    /// After a step: releases settling bodies the engine put to sleep (or
    /// that are legitimately awake because something touches them).
    fn finish_settle(&mut self) {
        for id in std::mem::take(&mut self.settling) {
            let Some(l) = self.live_mut(id) else { continue };
            l.settling = false;
            let handle = l.handle;
            if let Some(rb) = self.engine.bodies.get_mut(handle) {
                rb.set_locked_axes(LockedAxes::empty(), false);
                rb.activation_mut().time_until_sleep = RigidBodyActivation::default_time_until_sleep();
            }
        }
    }

    /// Advances one fixed step of [`PhysicsParams::dt`].
    pub fn step(&mut self) {
        self.engine.integration_parameters.dt = self.params.dt;
        let collector = Collector { inv_dt: 1.0 / self.params.dt.max(1e-6), ..Collector::default() };
        self.engine.step_with_events(&PairFilter(&self.excluded), &collector);
        self.query_pending.clear();
        self.query_all = false;
        self.resist_rolling();
        let contacts = self.contact_reports();
        self.queries_dirty = false;
        let dt = self.params.dt;
        for (c1, c2, force, point, normal) in collector.contacts.into_inner().expect("collector poisoned") {
            if let (Some(a), Some(b)) = (self.id_of_collider(c1), self.id_of_collider(c2)) {
                let point = self.to_world(point);
                self.events.push(PhysicsEvent::ContactForce { a, b, impulse: force * dt, point, normal });
            }
        }
        for (target, by, impulse, point, direction) in collector.hits.into_inner().expect("collector poisoned") {
            if let (Some(target), Some(by)) = (self.id_of_collider(target), self.id_of_collider(by)) {
                let point = self.to_world(point);
                self.events.push(PhysicsEvent::Hit { target, by, impulse, point, direction });
            }
        }
        self.finish_settle();
        self.track_motion();
        // After `Woke`: a body's first contact report follows its wake.
        self.events.extend(contacts);
    }

    /// [`PhysicsEvent::Contact`] for the touching pairs of awake dynamic bodies
    /// on [`PhysicsParams::contact_report_layers`] (each pair once).
    fn contact_reports(&self) -> Vec<PhysicsEvent> {
        let layers = self.params.contact_report_layers;
        if layers.is_empty() {
            return Vec::new();
        }
        let min = self.params.contact_report_speed;
        let bodies = &self.engine.bodies;
        let colliders = &self.engine.colliders;
        let parent = |c: ColliderHandle| colliders.get(c).and_then(rapier3d::geometry::Collider::parent).and_then(|h| bodies.get(h));
        let inv_mass = |rb: Option<&RigidBody>| rb.filter(|rb| rb.is_dynamic()).map_or(0.0, |rb| 1.0 / rb.mass().max(1e-6));
        // A mesh's triangle (the manifold's subshape) has its own surface.
        let surface = |c: ColliderHandle, triangle: u32| {
            let index = colliders.get(c).and_then(|c| u32::try_from(c.user_data).ok());
            let live = index.and_then(|i| self.slots.get(i as usize)?.live.as_ref());
            live.map_or(SurfaceId::DEFAULT, |l| l.mesh.as_ref().map_or(l.surface, |m| m.surface(triangle)))
        };
        let mut seen = HashSet::new();
        let mut reports = Vec::new();
        for (_, rb) in self.engine.active_bodies() {
            let live = u32::try_from(rb.user_data).ok().and_then(|i| self.slots.get(i as usize)?.live.as_ref());
            if !rb.is_dynamic() || !live.is_some_and(|l| l.layer.intersects(layers) && !l.settling) {
                continue;
            }
            for &c in rb.colliders() {
                for pair in self.engine.narrow_phase.contact_pairs_with(c) {
                    if !pair.has_any_active_contact() || !seen.insert((pair.collider1, pair.collider2)) {
                        continue;
                    }
                    let Some((manifold, contact)) = pair.find_deepest_contact() else { continue };
                    let (Some(c1), Some(c2)) = (colliders.get(pair.collider1), colliders.get(pair.collider2)) else { continue };
                    let other = if pair.collider1 == c { pair.collider2 } else { pair.collider1 };
                    let other_rb = parent(other);
                    let impulse = pair.total_impulse_magnitude();
                    let speed = impulse * (inv_mass(Some(rb)) + inv_mass(other_rb));
                    // Contacts are found before the step integrates, so a collider-local point has
                    // turned with its body since (a rolling ball's by a large angle). The point on a
                    // side that doesn't move is exact; between two moving bodies, take the middle.
                    let p1 = c1.position().transform_point(contact.local_p1);
                    let p2 = c2.position().transform_point(contact.local_p2);
                    let point = match (parent(pair.collider1).is_some_and(RigidBody::is_dynamic), parent(pair.collider2).is_some_and(RigidBody::is_dynamic)) {
                        (true, false) => p2,
                        (false, true) => p1,
                        _ => (p1 + p2) * 0.5,
                    };
                    // The manifold's normal points from collider 1 to 2; ours from `a` (this body) out.
                    let normal = if pair.collider1 == c { manifold.data.normal } else { -manifold.data.normal };
                    let relative = rb.velocity_at_point(point) - other_rb.map_or(Vec3::ZERO, |o| o.velocity_at_point(point));
                    let slide = (relative - normal * relative.dot(normal)).length();
                    if speed < min && slide < min {
                        continue;
                    }
                    let (sub_c, sub_other) = if pair.collider1 == c { (manifold.subshape1, manifold.subshape2) } else { (manifold.subshape2, manifold.subshape1) };
                    reports.push((c, other, impulse, speed, slide, point, normal, [surface(c, sub_c), surface(other, sub_other)]));
                }
            }
        }
        let event = |(c, other, impulse, speed, slide, point, normal, surfaces)| {
            let (a, b) = (self.id_of_collider(c)?, self.id_of_collider(other)?);
            Some(PhysicsEvent::Contact { a, b, impulse, speed, slide, point: self.to_world(point), normal, surfaces })
        };
        reports.into_iter().filter_map(event).collect()
    }

    /// [`PhysicsParams::rolling_resistance`] on awake item bodies in contact.
    fn resist_rolling(&mut self) {
        let decel = self.params.rolling_resistance * self.params.dt;
        if decel <= 0.0 {
            return;
        }
        let items = Layer::DEBRIS | Layer::PROP;
        let rolling: Vec<RigidBodyHandle> = self
            .engine
            .active_bodies()
            .filter(|(_, rb)| {
                let slot = u32::try_from(rb.user_data).ok().and_then(|i| self.slots.get(i as usize));
                slot.and_then(|s| s.live.as_ref()).is_some_and(|l| l.layer.intersects(items) && !l.settling)
                    && rb.angvel().length_squared() > 0.0
                    && rb.colliders().iter().any(|&c| self.engine.narrow_phase.contact_pairs_with(c).any(ContactPair::has_any_active_contact))
            })
            .map(|(h, _)| h)
            .collect();
        for h in rolling {
            if let Some(rb) = self.engine.bodies.get_mut(h) {
                let w = rb.angvel();
                let speed = w.length();
                rb.set_angvel(w * ((speed - decel).max(0.0) / speed), false);
            }
        }
    }

    /// Interpolation history, Woke/Slept transitions and fell-out recovery.
    fn track_motion(&mut self) {
        self.epoch += 1;
        let epoch = self.epoch;
        let active: Vec<(u32, Local)> = self
            .engine
            .active_bodies()
            .filter_map(|(_, rb)| Some((u32::try_from(rb.user_data).ok()?, Local::of(rb))))
            .collect();
        let previous = std::mem::take(&mut self.awake);
        let kill = self.kill_height().map(|k| k - self.anchor().to_f64().y);
        let mut fell = Vec::new();
        for (index, pose) in active {
            let Some(id) = self.id_at(index) else { continue };
            let l = self.slots[index as usize].live.as_mut().expect("id_at checked");
            if l.settling {
                continue;
            }
            l.prev = l.curr;
            l.curr = pose;
            if !matches!(l.kind, BodyKind::Dynamic { .. }) {
                continue;
            }
            l.seen = epoch;
            if !l.awake {
                l.awake = true;
                self.events.push(PhysicsEvent::Woke(id));
            }
            if kill.is_some_and(|k| f64::from(pose.pos.y) < k) {
                fell.push(id);
            }
            self.awake.push(index);
        }
        for index in previous {
            let Some(id) = self.id_at(index) else { continue };
            let l = self.slots[index as usize].live.as_mut().expect("id_at checked");
            if l.seen != epoch {
                // Asleep now: its pose didn't change this step.
                l.awake = false;
                l.prev = l.curr;
                l.rest = l.curr;
                self.events.push(PhysicsEvent::Slept(id));
            }
        }
        for id in fell {
            self.recover(id);
        }
    }

    /// Puts a fallen body back at its rest pose, asleep.
    fn recover(&mut self, id: BodyId) {
        let Some(l) = self.live_mut(id) else { return };
        let rest = l.rest;
        l.prev = rest;
        l.curr = rest;
        l.awake = false;
        let handle = l.handle;
        if let Some(rb) = self.engine.bodies.get_mut(handle) {
            rb.set_position(Pose::from_parts(rest.pos, rest.rot), false);
            rb.set_linvel(Vec3::ZERO, false);
            rb.set_angvel(Vec3::ZERO, false);
        }
        self.awake.retain(|&i| i != id.index);
        self.begin_settle(id);
        self.events.push(PhysicsEvent::FellOut(id));
    }

    /// Events since the last drain, in order.
    pub fn drain_events(&mut self) -> std::vec::Drain<'_, PhysicsEvent> {
        self.events.drain(..)
    }

    /// Moves the anchor to `cell`: every engine position shifts by the
    /// whole-cell delta. Velocities, sleep states, contacts and joints are
    /// untouched (their data is body-relative); nothing wakes.
    pub fn rebase(&mut self, cell: IVec3) {
        if cell == self.anchor {
            return;
        }
        // Integer cells × 256 m: exact in f32 for any shift inside ±2^15 cells.
        #[allow(clippy::cast_precision_loss)]
        let d = (cell - self.anchor).as_vec3() * GRID_CELL;
        // Moving a collider wakes its contact partners in the engine; sleepers re-settle (locked for
        // one step, put back to sleep by the engine in that step), so nothing wakes and nothing moves.
        let mut sleepers = Vec::new();
        for (index, slot) in self.slots.iter_mut().enumerate() {
            let Some(l) = slot.live.as_mut() else { continue };
            l.prev = l.prev.shifted(d);
            l.curr = l.curr.shifted(d);
            l.rest = l.rest.shifted(d);
            let Some(rb) = self.engine.bodies.get_mut(l.handle) else { continue };
            let mut pose = *rb.position();
            pose.translation -= d;
            rb.set_position(pose, false);
            if matches!(l.kind, BodyKind::Dynamic { .. }) && rb.is_sleeping() && !l.settling {
                sleepers.push(BodyId { index: u32::try_from(index).expect("u32 slots"), generation: slot.generation });
            }
            if l.kind == BodyKind::Kinematic {
                let mut next = *rb.next_position();
                next.translation -= d;
                rb.set_next_kinematic_position(next);
            }
        }
        self.anchor = cell;
        self.query_all = true;
        self.queries_dirty = true;
        for id in sleepers {
            self.begin_settle(id);
        }
    }

    /// Rebases to `focus`'s cell when it is farther than
    /// [`PhysicsParams::rebase_distance`] from the anchor. Call between steps.
    pub fn follow(&mut self, focus: WorldPos) -> bool {
        if self.anchor().delta(focus).length() <= f64::from(self.params.rebase_distance) {
            return false;
        }
        self.rebase(focus.cell);
        true
    }
}

/// The physics worlds of every loaded worldspace; only the active one steps
/// (others hold their bodies for door prefetch and portal views).
pub struct PhysicsWorlds<K> {
    worlds: HashMap<K, PhysicsWorld>,
    active: Option<K>,
}

impl<K> Default for PhysicsWorlds<K> {
    fn default() -> Self {
        Self { worlds: HashMap::new(), active: None }
    }
}

impl<K: Clone + Eq + Hash> PhysicsWorlds<K> {
    pub fn get(&self, ws: &K) -> Option<&PhysicsWorld> {
        self.worlds.get(ws)
    }

    pub fn get_mut(&mut self, ws: &K) -> Option<&mut PhysicsWorld> {
        self.worlds.get_mut(ws)
    }

    /// The world of `ws`, created anchored at `anchor` if missing.
    pub fn get_or_create(&mut self, ws: &K, anchor: WorldPos, params: &PhysicsParams) -> &mut PhysicsWorld {
        self.worlds.entry(ws.clone()).or_insert_with(|| PhysicsWorld::new(anchor, params.clone()))
    }

    pub fn remove(&mut self, ws: &K) -> Option<PhysicsWorld> {
        if self.active.as_ref() == Some(ws) {
            self.active = None;
        }
        self.worlds.remove(ws)
    }

    pub fn set_active(&mut self, ws: Option<K>) {
        self.active = ws;
    }

    pub fn active(&self) -> Option<&K> {
        self.active.as_ref()
    }

    pub fn active_world(&self) -> Option<&PhysicsWorld> {
        self.worlds.get(self.active.as_ref()?)
    }

    pub fn active_world_mut(&mut self) -> Option<&mut PhysicsWorld> {
        self.worlds.get_mut(self.active.as_ref()?)
    }

    /// Every world with its key.
    pub fn iter(&self) -> impl Iterator<Item = (&K, &PhysicsWorld)> {
        self.worlds.iter()
    }

    /// Bodies in every world.
    pub fn body_count(&self) -> usize {
        self.worlds.values().map(PhysicsWorld::len).sum()
    }

    /// Steps the active world once.
    pub fn step(&mut self) {
        if let Some(w) = self.active_world_mut() {
            w.step();
        }
    }
}

/// Drops contacts between excluded collider pairs (rapier calls it for
/// colliders flagged `FILTER_CONTACT_PAIRS`): the `PhysX` filter shader, Jolt's
/// group filter.
struct PairFilter<'a>(&'a [(ColliderHandle, ColliderHandle)]);

impl PhysicsHooks for PairFilter<'_> {
    fn filter_contact_pair(&self, c: &PairFilterContext) -> Option<SolverFlags> {
        let (a, b) = (c.collider1, c.collider2);
        if self.0.iter().any(|&(x, y)| (x, y) == (a, b) || (x, y) == (b, a)) {
            None
        } else {
            Some(SolverFlags::COMPUTE_RIGID_IMPULSES)
        }
    }
}

/// Grab support (task P6, `carry`).
impl PhysicsWorld {
    pub fn kind(&self, id: BodyId) -> Option<BodyKind> {
        self.live(id).map(|l| l.kind)
    }

    pub(crate) fn handle(&self, id: BodyId) -> Option<RigidBodyHandle> {
        self.live(id).map(|l| l.handle)
    }

    fn colliders_of(&self, id: BodyId) -> Vec<ColliderHandle> {
        self.live(id).and_then(|l| self.engine.bodies.get(l.handle)).map(|rb| rb.colliders().to_vec()).unwrap_or_default()
    }

    /// The center of mass in the body's frame and the radius of a sphere
    /// around it that holds every collider.
    pub fn mass_center(&self, id: BodyId) -> Option<(Vec3, f32)> {
        let rb = self.engine.bodies.get(self.live(id)?.handle)?;
        let com = rb.local_center_of_mass();
        let inv = rb.position().inverse();
        let mut radius: f32 = 0.0;
        for &c in rb.colliders() {
            let col = self.engine.colliders.get(c)?;
            let local = inv * *col.position();
            let s = col.shape().compute_local_bounding_sphere();
            radius = radius.max((local * s.center()).distance(com) + s.radius());
        }
        Some((com, radius))
    }

    /// Moves a body to another layer (contacts and queries).
    pub fn set_layer(&mut self, id: BodyId, layer: Layer) -> bool {
        let colliders = self.colliders_of(id);
        let Some(l) = self.live_mut(id) else { return false };
        l.layer = layer;
        for c in colliders {
            if let Some(col) = self.engine.colliders.get_mut(c) {
                col.set_collision_groups(layer.groups());
            }
        }
        self.queries_dirty = true;
        true
    }

    /// While grabbed: at least `extra_iterations` extra solver iterations,
    /// and CCD. Returns the iterations it had (restored on release with
    /// [`Self::set_iterations`]).
    pub(crate) fn set_grabbed(&mut self, id: BodyId, extra_iterations: usize) -> usize {
        let Some(h) = self.handle(id) else { return 0 };
        let Some(rb) = self.engine.bodies.get_mut(h) else { return 0 };
        let had = rb.additional_solver_iterations();
        rb.set_additional_solver_iterations(had.max(extra_iterations));
        rb.enable_ccd(true);
        had
    }

    /// Sets a body's extra solver iterations (rapier solves its substep
    /// group with the most of its bodies').
    pub fn set_iterations(&mut self, id: BodyId, extra_iterations: usize) {
        let Some(h) = self.handle(id) else { return };
        if let Some(rb) = self.engine.bodies.get_mut(h) {
            rb.set_additional_solver_iterations(extra_iterations);
        }
    }

    /// No contacts between two bodies' colliders until [`Self::include_pair`].
    pub fn exclude_pair(&mut self, a: BodyId, b: BodyId) {
        let (ca, cb) = (self.colliders_of(a), self.colliders_of(b));
        for &x in &ca {
            for &y in &cb {
                self.excluded.push((x, y));
            }
        }
        for c in ca.into_iter().chain(cb) {
            if let Some(col) = self.engine.colliders.get_mut(c) {
                col.set_active_hooks(col.active_hooks() | ActiveHooks::FILTER_CONTACT_PAIRS);
            }
        }
    }

    pub fn include_pair(&mut self, a: BodyId, b: BodyId) {
        let (ca, cb) = (self.colliders_of(a), self.colliders_of(b));
        self.excluded.retain(|&(x, y)| !((ca.contains(&x) && cb.contains(&y)) || (ca.contains(&y) && cb.contains(&x))));
        for c in ca.into_iter().chain(cb) {
            let still = self.excluded.iter().any(|&(x, y)| x == c || y == c);
            if !still && let Some(col) = self.engine.colliders.get_mut(c) {
                col.set_active_hooks(col.active_hooks() - ActiveHooks::FILTER_CONTACT_PAIRS);
            }
        }
    }
}
