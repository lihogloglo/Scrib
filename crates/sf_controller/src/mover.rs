//! The kinematic mover (locomotion.md "Movers", M3): springs toward the
//! intent, blends `Drive` root motion, collides and slides the capsule
//! against statics, follows the ground (step up/down, slope limit, falls),
//! and predicts the trajectory. The canonical shape of it is Unity's
//! Kinematic Character Controller and Rapier's `KinematicCharacterController`
//! (collide-and-slide, autostep, snap to ground), built on our batched
//! [`GroundProbe`] instead of a physics scene so terrain and statics answer
//! together and every character's casts go out in one call per stage.

use glam::{Quat, Vec2, Vec3};
use serde::{Deserialize, Serialize};
use sf_anim::spring::{critical_spring, damper};
use sf_anim::{RigProfile, RigTemplate, RootDelta, attitude_rotation};
use sf_coords::WorldPos;

use crate::frame::CharacterFrame;
use crate::intent::LocomotionIntent;
use crate::motion::{MotionState, SpringParams, wrap_angle};
use crate::probe::{GroundHit, GroundProbe, ProbeQuery};
use crate::trajectory::{LocomotionParams, Trajectory};

/// Fraction of the asked speed a deflecting surface must leave for a
/// character without an explicit facing to turn along it
/// ([`Locomotion::deflect`]): about 84° incidence.
pub const DEFLECT_FACING: f32 = 0.1;

/// Shape and tuning of one character's mover. Defaults are generic
/// human-scale values; adapters set their own (race scale, creature size).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MoverParams {
    pub springs: SpringParams,
    /// Smoothing of the drawn height after a step up or down, seconds.
    pub halflife_height: f32,
    pub radius: f32,
    pub height: f32,
    /// Highest ledge climbed without blocking, meters.
    pub step_up: f32,
    /// Deepest drop followed without falling, meters.
    pub step_down: f32,
    /// Steepest walkable slope, degrees.
    pub max_slope_deg: f32,
    /// Gap kept from walls, meters.
    pub skin: f32,
    pub gravity: f32,
    pub max_fall_speed: f32,
    /// Radius of the downward ground cast; 0 is a ray.
    pub ground_radius: f32,
    /// How far below the water surface the feet hang while swimming at it
    /// (`MoverMode::Swim`), meters.
    pub swim_depth: f32,
}

impl Default for MoverParams {
    fn default() -> Self {
        Self {
            springs: SpringParams::default(),
            halflife_height: 0.08,
            radius: 0.3,
            height: 1.8,
            step_up: 0.45,
            step_down: 0.45,
            max_slope_deg: 46.0,
            skin: 0.01,
            gravity: 9.81,
            max_fall_speed: 50.0,
            ground_radius: 0.0,
            swim_depth: 1.35,
        }
    }
}

impl MoverParams {
    /// The shape for a skeleton's rig profile (rig-profile.md "Mover by
    /// template", audit item 14), other tuning at the defaults. Bipeds keep
    /// a human-proportioned upright capsule (radius a sixth of the height);
    /// every other template gets the M3 stand-in for a horizontal body, an
    /// upright capsule half the body length across (M4's physics controller
    /// adds the horizontal capsule). Steps scale with the support legs (half
    /// a leg up and down, the human 0.45 m at a 0.9 m leg) and the swim
    /// depth with the height. A profile without measures keeps the defaults.
    pub fn from_profile(profile: &RigProfile) -> Self {
        let m = &profile.measures;
        let mut p = Self::default();
        if m.height <= 0.0 {
            return p;
        }
        let across = match profile.template {
            RigTemplate::Biped => m.height / 6.0,
            _ => m.body_length / 4.0,
        };
        p.radius = across.max(m.body_width / 2.0).max(0.05);
        p.height = m.height.max(2.0 * p.radius);
        let leg = if m.leg_length > 0.0 { m.leg_length } else { m.hip_height.max(m.height / 2.0) };
        p.step_up = 0.5 * leg;
        p.step_down = 0.5 * leg;
        p.swim_depth = 0.75 * m.height;
        p
    }

    /// Cosine of the slope limit: the lowest walkable normal Y.
    pub fn min_normal_y(&self) -> f32 {
        self.max_slope_deg.to_radians().cos()
    }

    /// The wall sweep: an upright capsule from `step_up` to `height`, so
    /// anything lower is a step for the ground probe to climb.
    fn sweep(&self, feet: WorldPos, d: Vec3) -> Option<ProbeQuery> {
        let len = d.length();
        if len < 1e-6 {
            return None;
        }
        let span = (self.height - self.step_up).max(2.0 * self.radius);
        let radius = self.radius.min(span / 2.0);
        let center = feet.offset(Vec3::Y * (self.step_up + span / 2.0));
        Some(ProbeQuery::capsule(center, d / len, len + self.skin, radius, span / 2.0 - radius))
    }
}

/// Root motion from the animation graph's last update.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RootMotion {
    /// In the character's frame at that update (the graph's `root`).
    pub delta: RootDelta,
    /// The `Drive` share of the pose (the graph's `drive_weight`): 0 is
    /// capsule-driven, 1 fully root-motion driven.
    pub drive_weight: f32,
    /// A warp window with `y=warp` is open (the graph's
    /// `ActiveWarp::vertical`): a traversal (vault, mantle, climb). Ground
    /// snap and gravity are suspended and the full root delta, vertical
    /// included, is applied; walls and floors still collide. The character
    /// re-grounds with the normal step-down probe when the window closes
    /// (UE runs a mantle montage in flying mode the same way).
    pub traversal: bool,
}

/// How a character moves through space (locomotion.md "Mover modes";
/// gameplay switches it, e.g. on entering water or taking off).
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum MoverMode {
    /// Walking: horizontal intent, gravity, ground snap, steps and slopes.
    #[default]
    Ground,
    /// Flying or floating: 3D intent (its Y is used), no gravity or ground
    /// snap; the ground and ceilings still stop the capsule. With `hover`,
    /// the feet are kept that many meters above the ground under them by a
    /// critical spring (floating creatures; intent Y is ignored), holding
    /// altitude where no ground is within `hover + step_down`.
    Fly { hover: Option<f32> },
    /// Swimming: 3D intent without gravity; `surface` is the water surface
    /// height above the feet at the start of this step, refreshed by the
    /// adapter every step until water queries exist (M6). The feet stay at
    /// least [`MoverParams::swim_depth`] below it; the bottom stops them.
    Swim { surface: f32 },
}

/// A character's controller state (saved).
#[derive(Clone, Debug, PartialEq)]
pub struct Locomotion {
    /// Feet: the bottom center of the capsule, exactly on the ground.
    pub position: WorldPos,
    pub motion: MotionState,
    /// Vertical speed while airborne, m/s (negative falling).
    pub vertical_speed: f32,
    pub grounded: Option<GroundHit>,
    pub trajectory: Trajectory,
    /// Drawn height minus `position`: steps are smoothed out over
    /// `halflife_height` and foot IK puts the feet on the real steps.
    pub height_offset: f32,
    pub height_offset_velocity: f32,
    /// How the drawn root moved in the last step, in its frame before the
    /// step: the graph's `UpdateInputs::moved`.
    pub moved: RootDelta,
    pub mode: MoverMode,
    /// The drawn root's attitude `(pitch, roll)` in its heading frame,
    /// radians: `Drive` root motion's tilt accumulates here (flyers and
    /// swimmers whose clips extract attitude); on the ground it settles back
    /// to level with the facing half-life when not driven.
    pub attitude: Vec2,
    /// The last step was a traversal ([`RootMotion::traversal`]); the next
    /// non-traversal step re-grounds with the step-down probe.
    pub traversing: bool,
    /// Horizontal unit normal of the surface that turned the last step's
    /// motion aside (a wall slid along, ground too steep to climb), else
    /// zero. With no facing in the intent, the next step faces the motion
    /// that surface leaves (orient to movement), not into it.
    pub deflect: Vec3,
}

impl Locomotion {
    /// Standing at `position` facing `yaw`, not yet grounded (the first
    /// step finds the ground within `step_up + step_down`).
    pub fn new(position: WorldPos, yaw: f32) -> Self {
        Self {
            position,
            motion: MotionState::facing(yaw),
            vertical_speed: 0.0,
            grounded: None,
            trajectory: Trajectory::default(),
            height_offset: 0.0,
            height_offset_velocity: 0.0,
            moved: RootDelta::ZERO,
            mode: MoverMode::Ground,
            attitude: Vec2::ZERO,
            traversing: false,
            deflect: Vec3::ZERO,
        }
    }

    /// Where the character is drawn.
    pub fn render_position(&self) -> WorldPos {
        self.position.offset(Vec3::Y * self.height_offset)
    }

    /// How the character is drawn: facing, then attitude. The animation
    /// graph's character space stays the heading frame ([`Self::frame`]).
    pub fn render_rotation(&self) -> Quat {
        Quat::from_rotation_y(self.motion.yaw) * attitude_rotation(self.attitude)
    }

    /// The drawn root and facing: the animation graph's character space.
    pub fn frame(&self) -> CharacterFrame {
        CharacterFrame::new(self.render_position(), self.motion.yaw)
    }

    pub fn params(&self) -> LocomotionParams {
        LocomotionParams::of(self)
    }
}

/// One character's input to a mover step.
pub struct MoveInput<'a> {
    pub loco: &'a mut Locomotion,
    pub intent: LocomotionIntent,
    pub params: &'a MoverParams,
    pub root: RootMotion,
}

/// The ground cast of one character this step.
#[derive(Clone, Copy)]
enum Cast {
    /// Standing still on known ground: no cast.
    Skipped,
    Done(Option<GroundHit>),
}

/// Per-character work between stages.
struct Pending {
    frame: CharacterFrame,
    start: WorldPos,
    /// Horizontal displacement this step.
    d: Vec3,
    /// Vertical displacement asked for this step (free movement only).
    dy: f32,
    /// Vertical speed before the springs ran (the hover spring's state).
    vy0: f32,
    free: Option<Free>,
    v_goal: Vec3,
    yaw_goal: f32,
    /// Ground probe origin height above the feet, and the probe's reach.
    probe: Option<ProbeQuery>,
    /// A surface turned this step aside (sets [`Locomotion::deflect`]).
    deflected: bool,
}

/// Movement without ground snap or gravity this step.
#[derive(Clone, Copy, PartialEq)]
enum Free {
    /// A warped traversal on the ground mover.
    Traversal,
    Fly {
        hover: Option<f32>,
    },
    Swim {
        surface: f32,
    },
}

impl Free {
    fn of(it: &MoveInput<'_>) -> Option<Self> {
        match it.loco.mode {
            MoverMode::Fly { hover } => Some(Self::Fly { hover }),
            MoverMode::Swim { surface } => Some(Self::Swim { surface }),
            MoverMode::Ground if it.root.traversal => Some(Self::Traversal),
            MoverMode::Ground => None,
        }
    }
}

/// Steps characters; holds scratch buffers so batches don't allocate.
#[derive(Default)]
pub struct KinematicMover {
    pending: Vec<Pending>,
    queries: Vec<ProbeQuery>,
    owners: Vec<usize>,
    hits: Vec<Option<GroundHit>>,
}

impl KinematicMover {
    pub fn new() -> Self {
        Self::default()
    }

    /// Steps one character (one probe call per stage).
    pub fn step(&mut self, input: MoveInput<'_>, probe: &dyn GroundProbe, dt: f32) {
        self.step_batch(&mut [input], probe, dt);
    }

    /// Steps every character by `dt`. Probes: one batched wall sweep for the
    /// moving characters, one batched ground cast, plus individual casts
    /// only for characters that slide along walls or hit an unclimbable slope.
    pub fn step_batch(&mut self, items: &mut [MoveInput<'_>], probe: &dyn GroundProbe, dt: f32) {
        self.pending.clear();
        self.queries.clear();
        self.owners.clear();
        for (i, it) in items.iter_mut().enumerate() {
            let p = begin(it, dt);
            if let Some(q) = it.params.sweep(p.start, p.d) {
                self.queries.push(q);
                self.owners.push(i);
            }
            self.pending.push(p);
        }

        // Stage 1: wall sweeps, then collide-and-slide.
        self.hits.clear();
        self.hits.resize(self.queries.len(), None);
        probe.probe(&self.queries, &mut self.hits);
        for (&i, hit) in self.owners.iter().zip(&self.hits) {
            slide(&mut items[i], &mut self.pending[i], *hit, probe);
        }

        // Stage 2: ground casts.
        self.queries.clear();
        self.owners.clear();
        for (i, (it, p)) in items.iter().zip(self.pending.iter_mut()).enumerate() {
            p.probe = ground_query(it, p, dt);
            if let Some(q) = p.probe {
                self.queries.push(q);
                self.owners.push(i);
            }
        }
        self.hits.clear();
        self.hits.resize(self.queries.len(), None);
        probe.probe(&self.queries, &mut self.hits);
        let mut next = 0;
        for (i, (it, p)) in items.iter_mut().zip(&self.pending).enumerate() {
            let cast = if p.probe.is_some() && self.owners.get(next) == Some(&i) {
                next += 1;
                Cast::Done(self.hits[next - 1])
            } else {
                Cast::Skipped
            };
            finish(it, p, cast, probe, dt);
        }
    }
}

/// Springs, root-motion blend; the horizontal displacement to try.
fn begin(it: &mut MoveInput<'_>, dt: f32) -> Pending {
    let free = Free::of(it);
    let loco = &mut *it.loco;
    let frame = loco.frame();
    let yaw0 = loco.motion.yaw;
    let (mut v_goal, mut yaw_goal) = loco.motion.goals(&it.intent);
    // Orient to movement around obstacles: when the last step was turned
    // aside (a wall slid along, ground too steep to climb) and the intent
    // names no facing, face the motion that surface leaves. A walker pushed
    // along a slope's contour then walks along it instead of side-stepping
    // while facing into it (its legs would play a strafe at a fraction of
    // the speed). Nearly head-on (the rest under `DEFLECT_FACING` of the
    // speed, below the graph's moving threshold) it keeps facing the goal.
    // The surface stays in effect while the intent pushes into it (a step
    // that only grazes it doesn't reach the sweep, see `finish`).
    if horizontal(v_goal).dot(loco.deflect) >= 0.0 {
        loco.deflect = Vec3::ZERO;
    }
    let n = loco.deflect;
    if it.intent.desired_facing.is_none() && n != Vec3::ZERO {
        let v = horizontal(v_goal);
        let along = v - n * v.dot(n).min(0.0);
        if along.length() > DEFLECT_FACING * v.length() {
            yaw_goal = along.x.atan2(along.z);
            loco.motion.facing_goal = yaw_goal;
        }
    }
    match free {
        // 3D intent.
        Some(Free::Fly { hover: None } | Free::Swim { .. }) => v_goal.y = it.intent.desired_velocity.y,
        // Hovering springs the height in `finish_free`; walking and
        // traversals have no spring-driven vertical motion: a flyer that
        // switched to walking in the air falls with its vertical speed.
        Some(Free::Fly { hover: Some(_) }) => {}
        Some(Free::Traversal) | None => {
            if free.is_none() && loco.grounded.is_none() && loco.motion.velocity.y != 0.0 {
                loco.vertical_speed = loco.motion.velocity.y;
            }
            loco.motion.velocity.y = 0.0;
            loco.motion.acceleration.y = 0.0;
        }
    }
    let vy0 = loco.motion.velocity.y;
    let mut d = loco.motion.advance(v_goal, yaw_goal, &it.params.springs, dt);
    let w = it.root.drive_weight.clamp(0.0, 1.0);
    if w > 0.0 && dt > 0.0 {
        // `Drive` root motion (animation.md "Root motion modes"): one frame
        // late. Its vertical part is dropped on the ground (the probe keeps
        // the feet on it) and applied when moving freely.
        let mut root = frame.rotation() * it.root.delta.translation;
        if free.is_none() {
            root.y = 0.0;
        }
        let m = &mut loco.motion;
        d = d * (1.0 - w) + root * w;
        m.velocity = m.velocity * (1.0 - w) + root / dt * w;
        m.acceleration *= 1.0 - w;
        if free == Some(Free::Traversal) {
            // A traversal's climb is not momentum to keep after the window.
            m.velocity.y = 0.0;
        }
        let spring_turn = wrap_angle(m.yaw - yaw0);
        m.yaw = wrap_angle(yaw0 + spring_turn * (1.0 - w) + it.root.delta.yaw * w);
        m.angular_velocity = m.angular_velocity * (1.0 - w) + it.root.delta.yaw / dt * w;
    }
    // Attitude: authored tilt while driven; the undriven share levels out on
    // the ground and holds while moving freely.
    let settled = if free.is_none() || free == Some(Free::Traversal) {
        Vec2::new(
            damper(loco.attitude.x, 0.0, it.params.springs.halflife_facing, dt),
            damper(loco.attitude.y, 0.0, it.params.springs.halflife_facing, dt),
        )
    } else {
        loco.attitude
    };
    loco.attitude = loco.attitude.lerp(settled, 1.0 - w) + it.root.delta.tilt * w;
    let dy = d.y;
    d.y = 0.0;
    Pending { frame, start: loco.position, d, dy, vy0, free, v_goal, yaw_goal, probe: None, deflected: false }
}

fn horizontal(v: Vec3) -> Vec3 {
    Vec3::new(v.x, 0.0, v.z)
}

/// Collide and slide (Fauerby, *Improved Collision Detection and
/// Response*, 2003): move to the contact, keep the tangential remainder,
/// at most four sweeps. Velocity into the wall is removed too, so the
/// springs don't keep pushing.
fn slide(it: &mut MoveInput<'_>, p: &mut Pending, first: Option<GroundHit>, probe: &dyn GroundProbe) {
    let params = it.params;
    let mut deflect = Vec3::ZERO;
    let motion = &mut it.loco.motion;
    let mut pos = p.start;
    let mut rem = p.d;
    let mut hit = first;
    for iter in 0..4 {
        let Some(h) = hit else {
            pos = pos.offset(rem);
            break;
        };
        let len = rem.length();
        let dir = rem / len;
        let travel = (h.distance - params.skin).clamp(0.0, len);
        pos = pos.offset(dir * travel);
        rem = dir * (len - travel);
        let n = horizontal(h.normal).normalize_or_zero();
        if n == Vec3::ZERO {
            break;
        }
        deflect = n;
        rem -= n * rem.dot(n).min(0.0);
        motion.velocity -= n * motion.velocity.dot(n).min(0.0);
        motion.acceleration -= n * motion.acceleration.dot(n).min(0.0);
        if iter == 3 {
            break;
        }
        match params.sweep(pos, rem) {
            Some(q) => hit = probe.probe_one(q),
            None => break,
        }
    }
    p.d = horizontal(p.start.delta_f32(pos));
    if deflect != Vec3::ZERO {
        it.loco.deflect = deflect;
        p.deflected = true;
    }
}

/// The fall over this step from the current vertical speed: (distance, new speed).
fn fall(loco: &Locomotion, params: &MoverParams, dt: f32) -> (f32, f32) {
    let v1 = (loco.vertical_speed - params.gravity * dt).max(-params.max_fall_speed);
    (-(loco.vertical_speed + v1) * 0.5 * dt, v1)
}

/// The downward cast at the slid position, or `None` when a grounded
/// character stands still (its ground can't have moved: R0 statics never
/// unload under a character).
fn ground_query(it: &MoveInput<'_>, p: &Pending, dt: f32) -> Option<ProbeQuery> {
    let (loco, params) = (&*it.loco, it.params);
    let reach = match p.free {
        // Free movement: the floor under this step's descent (and, hovering,
        // the ground to hover above).
        Some(f) => {
            let hover = if let Free::Fly { hover: Some(h) } = f { h + params.step_down } else { 0.0 };
            (-p.dy).max(0.0) + hover + params.skin
        }
        None if loco.grounded.is_some() && p.d.length_squared() < 1e-12 => return None,
        None if loco.grounded.is_some() || loco.traversing => params.step_down,
        None => fall(loco, params, dt).0.max(0.0) + 1e-3,
    };
    let r = params.ground_radius;
    let origin = p.start.offset(p.d + Vec3::Y * (params.step_up + r));
    Some(if r > 0.0 { ProbeQuery::sphere(origin, Vec3::NEG_Y, params.step_up + reach, r) } else { ProbeQuery::down(origin, params.step_up + reach) })
}

/// Lands, snaps or falls; then height smoothing, trajectory, `moved`.
/// `cast` is the ground cast at the slid position.
fn finish(it: &mut MoveInput<'_>, p: &Pending, cast: Cast, probe: &dyn GroundProbe, dt: f32) {
    if let Some(free) = p.free {
        finish_free(it, p, free, cast, probe, dt);
        return;
    }
    it.loco.traversing = false;
    let params = it.params;
    let loco = &mut *it.loco;
    let min_ny = params.min_normal_y();
    // Unwalkable ground blocks motion that climbs it, however small the step
    // (a rise threshold lets the springs creep up in tiny steps).
    let climbs = |h: &GroundHit, d: Vec3| !h.walkable(min_ny) && horizontal(h.normal).dot(d) < -1e-7;
    let mut d = p.d;
    let mut deflected = p.deflected;
    let mut feet = p.start.offset(d);
    if let Cast::Done(mut hit) = cast {
        let mut blocked = false;
        if loco.grounded.is_some()
            && let Some(h) = hit.filter(|h| climbs(h, d))
            && let Some(ahead) = step_over(params, p, &h, feet, d, probe)
        {
            // A short unwalkable bit (a kerb's bevel, a rock's lip) with
            // walkable ground ahead within `step_up`: stepped over like a
            // ledge (UE's StepUp; OpenMW's stepper skips tiny unwalkable
            // slopes). The feet ride its height, the ground ahead's normal.
            hit = Some(GroundHit { normal: ahead.normal, ..h });
        } else if loco.grounded.is_some()
            && let Some(h) = hit.filter(|h| climbs(h, d))
        {
            // Too steep to climb: keep only the motion along the slope's contour.
            let n = horizontal(h.normal).normalize_or_zero();
            d -= n * d.dot(n).min(0.0);
            loco.deflect = n;
            deflected = true;
            loco.motion.velocity -= n * loco.motion.velocity.dot(n).min(0.0);
            feet = p.start.offset(d);
            hit = p.probe.and_then(|q| probe.probe_one(ProbeQuery { origin: feet.offset(Vec3::Y * (params.step_up + params.ground_radius)), ..q }));
            if hit.as_ref().is_some_and(|h| climbs(h, d) || (!h.walkable(min_ny) && rise(params, h) > 1e-3)) {
                // Still climbing: stay put on the old ground.
                d = Vec3::ZERO;
                feet = p.start;
                blocked = true;
            }
        }
        match hit {
            _ if blocked => {}
            Some(h) => {
                let up = rise(params, &h);
                // The height change a planar continuation of the previous ground
                // explains is slope, not a step: only the rest is smoothed.
                if let Some(g) = loco.grounded {
                    let step = up + horizontal(g.normal).dot(d) / g.normal.y.max(0.05);
                    if step.abs() > 0.02 {
                        let limit = params.step_up + params.step_down;
                        loco.height_offset = (loco.height_offset - step).clamp(-limit, limit);
                    }
                }
                feet = feet.offset(Vec3::Y * up);
                loco.vertical_speed = 0.0;
                loco.grounded = Some(h);
            }
            None => {
                let (drop, v1) =
                    if loco.grounded.is_some() { (0.5 * params.gravity * dt * dt, -params.gravity * dt) } else { fall(loco, params, dt) };
                feet = feet.offset(Vec3::NEG_Y * drop);
                loco.vertical_speed = v1;
                loco.grounded = None;
            }
        }
    }
    // Moved into where the surface was (past a wall's end, off the slope):
    // it no longer turns the motion aside.
    if !deflected && d.dot(loco.deflect) < -params.skin {
        loco.deflect = Vec3::ZERO;
    }
    loco.position = feet;
    critical_spring(&mut loco.height_offset, &mut loco.height_offset_velocity, 0.0, params.halflife_height, dt);
    loco.trajectory = loco.motion.predict(p.v_goal, p.yaw_goal, &params.springs);
    loco.moved = p.frame.delta_to(&loco.frame());
}

/// Free movement (flying, swimming, traversals): the vertical displacement
/// is clamped by a ceiling sweep going up, the floor going down and the
/// swim surface; hovering springs the height toward the ground plus the
/// hover height. Nothing snaps and nothing falls.
fn finish_free(it: &mut MoveInput<'_>, p: &Pending, free: Free, cast: Cast, probe: &dyn GroundProbe, dt: f32) {
    let params = it.params;
    let loco = &mut *it.loco;
    let slid = p.start.offset(p.d);
    // The floor under the slid position, relative to the starting feet height.
    let hit = match cast {
        Cast::Done(hit) => hit,
        Cast::Skipped => None,
    };
    let floor = hit.map(|h| rise(params, &h));
    let mut dy = p.dy;
    if let (Free::Fly { hover: Some(hover) }, Some(h), Some(g), true) = (free, hit, floor, dt > 0.0) {
        // The spring runs on the height error above the ground, in the
        // ground's moving frame: the slope under the motion is fed forward,
        // so a constant slope is followed without lag.
        let n = h.normal;
        let ground_rate = if n.y > 0.05 { -(n.x * p.d.x + n.z * p.d.z) / (n.y * dt) } else { 0.0 };
        // The error at the start of the step: the floor under the slid
        // position, carried back along the slope to under the start.
        let (mut error, mut rate) = (ground_rate * dt - g - hover, p.vy0 - ground_rate);
        critical_spring(&mut error, &mut rate, 0.0, params.springs.halflife_velocity, dt);
        dy = g + hover + error;
        loco.motion.velocity.y = rate + ground_rate;
        loco.motion.acceleration.y = 0.0;
    }
    if let Free::Swim { surface } = free {
        let top = surface - params.swim_depth;
        if dy > top {
            dy = top;
            loco.motion.velocity.y = loco.motion.velocity.y.min(0.0);
        }
    }
    if dy > 0.0
        && let Some(h) = params.sweep(slid, Vec3::Y * dy).and_then(|q| probe.probe_one(q))
    {
        dy = (h.distance - params.skin).clamp(0.0, dy);
        loco.motion.velocity.y = loco.motion.velocity.y.min(0.0);
    }
    if let Some(g) = floor
        && dy < g
    {
        dy = g;
        loco.motion.velocity.y = loco.motion.velocity.y.max(0.0);
    }
    loco.position = slid.offset(Vec3::Y * dy);
    loco.vertical_speed = 0.0;
    loco.grounded = None;
    loco.traversing = free == Free::Traversal;
    critical_spring(&mut loco.height_offset, &mut loco.height_offset_velocity, 0.0, params.halflife_height, dt);
    loco.trajectory = loco.motion.predict(p.v_goal, p.yaw_goal, &params.springs);
    loco.moved = p.frame.delta_to(&loco.frame());
}

/// Walkable ground one capsule radius past `feet` along the motion `d`,
/// found by the step's own ground cast (so within `step_up` above the start
/// and `step_down` below), when the unwalkable hit `h` is no higher than
/// `step_up` either: the steep bit is short and can be stepped over.
fn step_over(params: &MoverParams, p: &Pending, h: &GroundHit, feet: WorldPos, d: Vec3, probe: &dyn GroundProbe) -> Option<GroundHit> {
    let dir = d.normalize_or_zero();
    if dir == Vec3::ZERO || rise(params, h) > params.step_up {
        return None;
    }
    let query = p.probe?;
    let origin = feet.offset(dir * params.radius + Vec3::Y * (params.step_up + params.ground_radius));
    // The ground ahead stands at least as high as the steep bit (a bevel or
    // lip up onto a ledge): ground below it is under the slope, not past it.
    probe
        .probe_one(ProbeQuery { origin, ..query })
        .filter(|a| a.walkable(params.min_normal_y()) && rise(params, a) >= rise(params, h) - 0.02)
}

/// How far above the probed feet position the hit ground is.
fn rise(params: &MoverParams, h: &GroundHit) -> f32 {
    params.step_up - h.distance
}
/// Pushes overlapping characters apart: the cheap separation that stands
/// in for actor-to-actor collision until M4. `out[i]` receives a
/// horizontal correction resolving each overlap half per side. Characters
/// more than 1.5 m apart vertically (different floors) don't interact.
/// Sort-and-sweep along X, so a crowd costs about n log n.
pub fn separate(positions: &[WorldPos], radii: &[f32], out: &mut [Vec3]) {
    out.fill(Vec3::ZERO);
    let Some(&anchor) = positions.first() else {
        return;
    };
    let local: Vec<Vec3> = positions.iter().map(|&p| anchor.delta_f32(p)).collect();
    let r_max = radii.iter().copied().fold(0.0, f32::max);
    let mut order: Vec<usize> = (0..positions.len()).collect();
    order.sort_by(|&a, &b| local[a].x.total_cmp(&local[b].x).then(a.cmp(&b)));
    for (k, &i) in order.iter().enumerate() {
        for &j in &order[k + 1..] {
            if local[j].x - local[i].x >= radii[i] + r_max {
                break;
            }
            let delta = local[j] - local[i];
            let reach = radii[i] + radii[j];
            let flat = horizontal(delta);
            let dist = flat.length();
            if delta.y.abs() > 1.5 || dist >= reach {
                continue;
            }
            // Coincident characters split along X by index, deterministically.
            let dir = if dist > 1e-5 { flat / dist } else { Vec3::X };
            let push = dir * ((reach - dist) * 0.5);
            out[i] -= push;
            out[j] += push;
        }
    }
}
