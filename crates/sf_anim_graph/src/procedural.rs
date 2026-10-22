//! The procedural op stack, run after the graph on the model-space pose
//! (animation.md "Procedural pass and IK"). Each op has an animation-LOD
//! cut-off; ops keep small per-instance state (springs, foot locks).
//! Limbs are any joint count with their own solver, and bodies have any
//! number of girdles (rig-profile.md), so the ops serve bipeds,
//! quadrupeds, multipeds and flyers alike.
//!
//! - `SpeedMatch` acts earlier, on playback rates (see the evaluator); it
//!   leaves the residual stride ratio for `StrideWarp`.
//! - `StrideWarp`: foot targets scaled along the velocity about each leg's
//!   girdle, girdles lowered so legs reach (UE Stride Warping, per girdle).
//! - `OrientationWarp`: pelvis turned toward the velocity, spine
//!   counter-rotated so the torso keeps its heading (UE Orientation Warping).
//! - `FootPlacement`: foot height and tilt from ground samples, world
//!   locks while the contact curve is above 0.5 (then the toe, while the
//!   `roll.<effector>` curve says the heel rolls), released into a decaying
//!   offset of half-life `release` / 2 (the swing takes the correction);
//!   one offset per girdle (critical springs): `Lower` drops each
//!   girdle by its legs' largest need; `Fit` follows their mean ground
//!   offset capped by reach, then the body pitches between girdles and
//!   rolls by `roll` (the "pelvis plus chest" extension of Johansen 2009).
//! - `LookAt`: aim chain toward a smoothed target with a smoothed weight.
//! - `SpineBend` / `Lean`: yaw into turns spread over the spine; roll by
//!   lateral acceleration.
//! - `LimbIk`: any limb to a target slot, position and rotation.
//! - `ChainIk`: FABRIK or CCD over a chain.
//! - `FullBodyIk`: several effectors at once (XPBD over the rig profile),
//!   support limbs pinned where the ops before left them.

use glam::{Affine3A, Quat, Vec2, Vec3};
use sf_anim::ik::{Aim, AimJoint, Effector, FullBodyScratch, LimbGoal, LimbSolver, aim_chain, clamp_angle, limb_pole, rotate_joint, solve_limb};
use sf_anim::spring::{critical_spring, damper};
use sf_anim::{LocalPose, ModelPose, Skeleton, update_subtree};

use crate::asset::BodyFit;
use crate::compiled::{CompiledGraph, GirdleSeg, Leg, Op};
use crate::instance::{FootStats, GroundSample, Params, UpdateInputs};
use crate::secondary::{Hit, HitState, SpringState, hit_react, spring_bones};

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct LegState {
    /// The planted effector's (ankle's) character-space position.
    lock: Option<Vec3>,
    /// The toe's, pinned while the heel rolls (the contact curve is down,
    /// the roll curve up): the foot pivots on it.
    toe_lock: Option<Vec3>,
    /// The release: the locked target's offset from the animated one, and
    /// its velocity, a critical spring decaying to zero (the correction
    /// goes into the swing).
    offset: Vec3,
    offset_velocity: Vec3,
    /// The lock drifted past the unplant distance: it releases and does
    /// not re-lock until it has (then it re-plants where the foot is).
    unplanting: bool,
    /// Smoothed ground height offset of the foot.
    height: Spring,
}

/// A critically damped value.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Spring {
    value: f32,
    velocity: f32,
}

impl Spring {
    fn update(&mut self, goal: f32, halflife: f32, dt: f32) -> f32 {
        critical_spring(&mut self.value, &mut self.velocity, goal, halflife, dt);
        self.value
    }
}

#[derive(Clone, Debug)]
pub(crate) enum OpState {
    None,
    Angle(f32),
    /// Roll and pitch (`Lean`).
    Angles(f32, f32),
    /// Per girdle: height offset and roll; per leg its lock; the op's
    /// weight (`None` until the first update sets it without blending).
    Feet {
        girdles: Vec<(Spring, Spring)>,
        legs: Vec<LegState>,
        weight: Option<Spring>,
    },
    Look {
        target: Vec3,
        velocity: Vec3,
        weight: f32,
        weight_velocity: f32,
        init: bool,
    },
    /// Full-body IK working memory and the effector list.
    Body(Box<(FullBodyScratch, Vec<Effector>)>),
    Hit(HitState),
    Springs(SpringState),
}

impl OpState {
    pub(crate) fn new(op: &Op) -> Self {
        match op {
            Op::OrientationWarp { .. } | Op::SpineBend { .. } => Self::Angle(0.0),
            Op::Lean { .. } => Self::Angles(0.0, 0.0),
            Op::FootPlacement { legs, girdles, .. } => {
                Self::Feet {
                    girdles: vec![(Spring::default(), Spring::default()); girdles.len()],
                    legs: vec![LegState::default(); legs.len()],
                    weight: None,
                }
            }
            Op::FullBodyIk { .. } => Self::Body(Box::default()),
            Op::HitReact(op) => Self::Hit(HitState::new(op)),
            Op::SpringBones(_) => Self::Springs(SpringState::default()),
            Op::LookAt { .. } => Self::Look { target: Vec3::ZERO, velocity: Vec3::ZERO, weight: 0.0, weight_velocity: 0.0, init: false },
            _ => Self::None,
        }
    }
}

pub(crate) struct ProceduralCx<'a> {
    pub graph: &'a CompiledGraph,
    pub skeleton: &'a Skeleton,
    pub params: &'a Params,
    pub inputs: &'a UpdateInputs<'a>,
    pub dt: f32,
    pub lod: usize,
    pub stride_ratio: f32,
    /// Filtered ground grade along the travel (rise over run).
    pub grade: f32,
    pub anim_velocity: Vec3,
    pub curves: &'a [f32],
    /// Guard counts of `FootPlacement` this update.
    pub feet: FootStats,
    /// Hits queued since the last update (`HitReact`).
    pub hits: &'a [Hit],
}

const ROOT: Affine3A = Affine3A::IDENTITY;

/// Moves `joint` by a model-space offset.
fn translate_joint(skeleton: &Skeleton, local: &mut LocalPose, model: &mut ModelPose, joint: usize, delta: Vec3) {
    let linear = skeleton.parent(joint).map_or(ROOT.matrix3, |p| model.mats[p].matrix3);
    let d = linear.inverse() * delta;
    let t = local.get(joint).translation;
    local.set_translation(joint, t + d);
    update_subtree(skeleton, local, model, &ROOT, joint);
}

fn horizontal(v: Vec3) -> Vec3 {
    Vec3::new(v.x, 0.0, v.z)
}

/// Weight of each ground normal against the probe points' spread, m²: the
/// normals rule on a smooth ramp, the points' heights on stairs (whose
/// treads are flat).
const NORMAL_WEIGHT: f32 = 0.01;

/// The ground grade along the travel (rise over run, positive uphill; the
/// facing, +Z, below 0.1 m/s) from the ground samples, character space: a
/// least-squares line through the points' heights along the travel, each
/// walkable normal's own slope counted as an observation of weight
/// [`NORMAL_WEIGHT`]. `None` without a sample; within ±1 (45°).
pub(crate) fn ground_grade(ground: &[Option<GroundSample>], velocity: Vec3) -> Option<f32> {
    let v = Vec2::new(velocity.x, velocity.z);
    let dir = if v.length() > 0.1 { v.normalize() } else { Vec2::Y };
    let samples = || ground.iter().flatten().filter(|g| g.point.is_finite() && g.normal.is_finite());
    #[allow(clippy::cast_precision_loss)]
    let n = samples().count() as f32;
    if n < 1.0 {
        return None;
    }
    let along = |g: &GroundSample| Vec2::new(g.point.x, g.point.z).dot(dir);
    let (ms, my) = samples().fold((0.0, 0.0), |(s, y), g| (s + along(g) / n, y + g.point.y / n));
    let (mut num, mut den) = (0.0f32, 0.0f32);
    for g in samples() {
        let ds = along(g) - ms;
        num += ds * (g.point.y - my);
        den += ds * ds;
        if g.normal.y > 0.5 {
            num -= NORMAL_WEIGHT * Vec2::new(g.normal.x, g.normal.z).dot(dir) / g.normal.y;
            den += NORMAL_WEIGHT;
        }
    }
    (den > 1e-6).then(|| (num / den).clamp(-1.0, 1.0)).filter(|g| g.is_finite())
}

/// The pole of a limb now: its animated bend, else its girdle-space axis.
fn pole_of(model: &ModelPose, leg: &Leg) -> Vec3 {
    let solved: Vec<usize> = leg.solved().iter().map(|&j| usize::from(j)).collect();
    let axis = leg.pole.map(|a| model.rotation(usize::from(leg.girdle)) * a);
    limb_pole(model, &solved, axis)
}

/// Solves a limb to `target`, the effector ending at model rotation `rot`.
fn solve_leg(skeleton: &Skeleton, local: &mut LocalPose, model: &mut ModelPose, leg: &Leg, target: Vec3, rot: Option<Quat>, weight: f32) -> f32 {
    let joints: Vec<usize> = leg.joints.iter().map(|&j| usize::from(j)).collect();
    let goal = LimbGoal { position: target, rotation: rot, pole: pole_of(model, leg), weight };
    solve_limb(skeleton, local, model, &ROOT, &joints, leg.solver, &goal)
}

/// Reach of a limb's solved span (slightly short of straight). A
/// `TwoBoneAnkle` leg reaches as thigh plus the shin-to-toe virtual bone:
/// its metatarsal only folds further, never straightens.
fn reach(model: &ModelPose, leg: &Leg) -> f32 {
    let p = |j: u16| model.translation(usize::from(j));
    let solved = leg.solved();
    let length = match (leg.solver, solved) {
        (LimbSolver::TwoBoneAnkle, &[a, b, _, d]) => p(a).distance(p(b)) + p(b).distance(p(d)),
        _ => solved.windows(2).map(|w| p(w[0]).distance(p(w[1]))).sum::<f32>(),
    };
    length * 0.999
}

/// Highest rise of a limb's root joint (`hip`) that still lets the limb
/// reach `target`.
fn rise_cap(model: &ModelPose, leg: &Leg, target: Vec3) -> f32 {
    let hip = model.translation(usize::from(leg.solved()[0]));
    let r = reach(model, leg);
    let d = horizontal(target - hip).length();
    target.y - hip.y + if d < r { (r * r - d * d).sqrt() } else { 0.0 }
}

/// Moves the body so girdle `k` rises by `offsets[k]` and rolls by
/// `rolls[k]`: the root girdle translates; each later girdle is reached by
/// pitching the spine joints before it, the rotation spread over them.
fn fit_body(s: &Skeleton, local: &mut LocalPose, model: &mut ModelPose, girdles: &[GirdleSeg], offsets: &[f32], rolls: &[f32]) {
    let Some(root) = girdles.first() else { return };
    let original: Vec<Vec3> = girdles.iter().map(|g| model.translation(usize::from(g.joint))).collect();
    if offsets[0].abs() > 1e-6 {
        translate_joint(s, local, model, usize::from(root.joint), Vec3::Y * offsets[0]);
    }
    // Body forward: from the root girdle toward the front-most one.
    let forward = if girdles.len() > 1 { horizontal(original[girdles.len() - 1] - original[0]).normalize_or(Vec3::Z) } else { Vec3::Z };
    let forward = if forward.dot(Vec3::Z) < 0.0 { -forward } else { forward };
    if rolls[0].abs() > 1e-6 {
        rotate_joint(s, local, model, &ROOT, usize::from(root.joint), Quat::from_axis_angle(forward, rolls[0]));
    }
    for (k, g) in girdles.iter().enumerate().skip(1) {
        let desired = original[k] + Vec3::Y * offsets[k];
        let n = g.spine.len();
        for (i, &j) in g.spine.iter().enumerate() {
            let pivot = model.translation(usize::from(j));
            let now = model.translation(usize::from(g.joint)) - pivot;
            let want = desired - pivot;
            if now.length_squared() < 1e-10 || want.length_squared() < 1e-10 {
                continue;
            }
            let arc = Quat::from_rotation_arc(now.normalize(), want.normalize());
            #[allow(clippy::cast_precision_loss)]
            let share = 1.0 / (n - i) as f32;
            rotate_joint(s, local, model, &ROOT, usize::from(j), Quat::IDENTITY.slerp(arc, share));
        }
        let extra = rolls[k] - rolls[0];
        if extra.abs() > 1e-6 {
            rotate_joint(s, local, model, &ROOT, usize::from(g.joint), Quat::from_axis_angle(forward, extra));
        }
    }
}

/// Runs every op whose LOD cut-off admits `cx.lod`.
#[allow(clippy::too_many_lines)]
pub(crate) fn run_ops(cx: &mut ProceduralCx<'_>, states: &mut [OpState], local: &mut LocalPose, model: &mut ModelPose) {
    let g = cx.graph;
    let s = cx.skeleton;
    for (entry, state) in g.ops.iter().zip(states.iter_mut()) {
        if cx.lod > usize::from(entry.max_lod) {
            if let OpState::Feet { legs, girdles, weight } = state {
                // Back in range, the op blends in from the animated pose.
                legs.fill(LegState::default());
                girdles.fill((Spring::default(), Spring::default()));
                *weight = Some(Spring::default());
            }
            match state {
                OpState::Hit(h) => h.clear(),
                OpState::Springs(sp) => sp.reset(),
                _ => {}
            }
            continue;
        }
        match (&entry.op, state) {
            (Op::StrideWarp { legs, girdles, min, max, .. }, _) => {
                let ratio = cx.stride_ratio.clamp(*min, *max);
                let v = horizontal(cx.inputs.velocity);
                if (ratio - 1.0).abs() < 1e-3 || v.length() < 1e-3 {
                    continue;
                }
                let dir = v.normalize();
                let mut targets = Vec::with_capacity(legs.len());
                let mut drops = vec![0.0f32; girdles.len()];
                for (gi, gs) in girdles.iter().enumerate() {
                    for &k in &gs.legs {
                        let leg = &g.legs[usize::from(legs[usize::from(k)])];
                        let center = horizontal(model.translation(usize::from(leg.girdle)));
                        let f = model.translation(usize::from(leg.effector()));
                        let along = (horizontal(f) - center).dot(dir);
                        let target = f + dir * along * (ratio - 1.0);
                        let hip = model.translation(usize::from(leg.solved()[0]));
                        let reach = reach(model, leg);
                        let rel = target - hip;
                        let r = horizontal(rel).length();
                        let height = -rel.y;
                        if r < reach && r * r + height * height > reach * reach {
                            drops[gi] = drops[gi].max(height - (reach * reach - r * r).sqrt());
                        }
                        targets.push((leg, target, model.rotation(usize::from(leg.effector()))));
                    }
                }
                if drops.iter().any(|&d| d > 0.0) {
                    let offsets: Vec<f32> = drops.iter().map(|d| -d).collect();
                    fit_body(s, local, model, girdles, &offsets, &vec![0.0; girdles.len()]);
                }
                for (leg, target, rot) in targets {
                    solve_leg(s, local, model, leg, target, Some(rot), 1.0);
                }
            }
            (Op::OrientationWarp { pelvis, spine, max_angle, halflife }, OpState::Angle(angle)) => {
                let (v, a) = (horizontal(cx.inputs.velocity), horizontal(cx.anim_velocity));
                let goal = if v.length() > 0.1 && a.length() > 0.1 { a.cross(v).y.atan2(a.dot(v)).clamp(-*max_angle, *max_angle) } else { 0.0 };
                *angle = damper(*angle, goal, *halflife, cx.dt);
                if angle.abs() < 1e-5 {
                    continue;
                }
                rotate_joint(s, local, model, &ROOT, usize::from(*pelvis), Quat::from_rotation_y(*angle));
                for &(j, w) in spine {
                    rotate_joint(s, local, model, &ROOT, usize::from(j), Quat::from_rotation_y(-*angle * w));
                }
            }
            (Op::FootPlacement { .. }, OpState::Feet { girdles: springs, legs: ls, weight }) => {
                foot_placement(cx, &entry.op, (springs, ls, weight), local, model);
            }
            (Op::LookAt { chain, forward, total_limit, halflife }, OpState::Look { target, velocity, weight, weight_velocity, init }) => {
                let goal = if cx.inputs.look_target.is_some() { 1.0 } else { 0.0 };
                if let Some(t) = cx.inputs.look_target {
                    if *init {
                        critical_spring(target, velocity, t, *halflife, cx.dt);
                    } else {
                        *target = t;
                        *init = true;
                    }
                }
                critical_spring(weight, weight_velocity, goal, *halflife, cx.dt);
                let w = weight.clamp(0.0, 1.0);
                if w < 1e-3 || chain.is_empty() {
                    continue;
                }
                let joints: Vec<AimJoint> = chain.iter().map(|&(j, weight, limit)| AimJoint { joint: usize::from(j), weight, limit }).collect();
                let end = usize::from(chain[chain.len() - 1].0);
                aim_chain(s, local, model, &ROOT, &joints, end, *target, Aim { forward: *forward, total_limit: *total_limit, weight: w });
            }
            (Op::SpineBend { spine, gain, max_angle, halflife }, OpState::Angle(angle)) => {
                let goal = (cx.inputs.turn_rate * gain).clamp(-*max_angle, *max_angle);
                *angle = damper(*angle, goal, *halflife, cx.dt);
                if angle.abs() < 1e-5 {
                    continue;
                }
                for &(j, w) in spine {
                    rotate_joint(s, local, model, &ROOT, usize::from(j), Quat::from_rotation_y(*angle * w));
                }
            }
            (Op::Lean { joints, gain, max_angle, grade, halflife }, OpState::Angles(roll, pitch)) => {
                // Lateral acceleration: positive to the left (+X) leans left, into the turn.
                let goal = (cx.inputs.acceleration.x * gain).clamp(-*max_angle, *max_angle);
                *roll = damper(*roll, goal, *halflife, cx.dt);
                // Ground grade: forward (+Z over +Y, about +X) uphill, back downhill.
                *pitch = damper(*pitch, grade.pitch(cx.grade), *halflife, cx.dt);
                if roll.abs() < 1e-5 && pitch.abs() < 1e-5 {
                    continue;
                }
                for &(j, w) in joints {
                    rotate_joint(s, local, model, &ROOT, usize::from(j), Quat::from_rotation_z(-*roll * w) * Quat::from_rotation_x(*pitch * w));
                }
            }
            (Op::LimbIk { limb, target, weight, rotation }, _) => {
                let w = cx.params.value(*weight).clamp(0.0, 1.0);
                let Some(Some(t)) = cx.inputs.ik_targets.get(usize::from(*target)) else {
                    continue;
                };
                if w < 1e-3 {
                    continue;
                }
                let keep = model.rotation(usize::from(limb.effector()));
                let rot = if *rotation { t.rotation.or(Some(keep)) } else { Some(keep) };
                solve_leg(s, local, model, limb, t.position, rot, w);
            }
            (Op::FullBodyIk { body, effectors, pins, iterations }, OpState::Body(state)) => {
                let (scratch, list) = &mut **state;
                list.clear();
                for e in effectors {
                    let w = cx.params.value(e.weight).clamp(0.0, 1.0);
                    let Some(Some(t)) = cx.inputs.ik_targets.get(usize::from(e.target)) else { continue };
                    if w >= 1e-3 {
                        let rotation = if e.rotation { t.rotation } else { None };
                        list.push(Effector { joint: usize::from(e.joint), position: t.position, rotation, weight: w, pull: e.pull });
                    }
                }
                if list.is_empty() {
                    continue;
                }
                for &j in pins {
                    let j = usize::from(j);
                    list.push(Effector { joint: j, position: model.translation(j), rotation: Some(model.rotation(j)), weight: 1.0, pull: 0.0 });
                }
                body.solve(s, local, model, &ROOT, list, usize::from(*iterations), scratch);
            }
            (Op::ChainIk { joints, solver, target, weight }, _) => {
                let w = cx.params.value(*weight).clamp(0.0, 1.0);
                let Some(Some(t)) = cx.inputs.ik_targets.get(usize::from(*target)) else {
                    continue;
                };
                if w < 1e-3 {
                    continue;
                }
                let chain: Vec<usize> = joints.iter().map(|&j| usize::from(j)).collect();
                let solver = if *solver == LimbSolver::Ccd { LimbSolver::Ccd } else { LimbSolver::Fabrik };
                let goal = LimbGoal { position: t.position, rotation: t.rotation, pole: Vec3::ZERO, weight: w };
                solve_limb(s, local, model, &ROOT, &chain, solver, &goal);
            }
            (Op::HitReact(op), OpState::Hit(state)) => hit_react(op, state, cx.hits, s, local, model, cx.dt),
            (Op::SpringBones(op), OpState::Springs(state)) => spring_bones(op, state, &cx.inputs.moved, s, local, model, cx.dt),
            _ => {}
        }
    }
}

/// Toe ground above the heel's ground plane by more than this is a step
/// edge (`FootPlacement` lifts the foot onto it), meters.
const STEP_EDGE: f32 = 0.02;

/// A rest-pose height as a ground clearance: rigs whose rest pose doesn't
/// stand on y = 0 (beyond a quarter of the leg) get none.
fn clearance(rest_height: f32, reach: f32) -> f32 {
    if (0.0..=0.25 * reach).contains(&rest_height) { rest_height } else { 0.0 }
}

/// A foot target before the body fit: the leg, its index among the op's
/// legs, the target, the ground normal and the animated foot rotation.
type FootTarget<'a> = (&'a Leg, usize, Vec3, Option<Vec3>, Quat);

/// `FootPlacement` (see the module docs) with its guards ([`FootGuards`](crate::asset::FootGuards),
/// UE Foot Placement / Leg IK practice): samples beyond the step limit are
/// rejected, locks release once they drift past the unplant distance (and
/// re-plant), a jump in `moved` resets them, girdle offsets stay within the
/// drop and rise limits, every target stays within its leg's reach of the
/// hip, and the whole op blends out (critically damped) while no leg has a
/// ground sample or the character is fast. Non-finite results skip the op.
#[allow(clippy::too_many_lines)]
fn foot_placement(
    cx: &mut ProceduralCx<'_>,
    op: &Op,
    (springs, ls, weight): (&mut [(Spring, Spring)], &mut [LegState], &mut Option<Spring>),
    local: &mut LocalPose,
    model: &mut ModelPose,
) {
    let Op::FootPlacement { legs, girdles, max_foot_angle, release, pelvis_halflife, body, roll, guards } = op else { return };
    let (g, s, dt) = (cx.graph, cx.skeleton, cx.dt);
    if legs.is_empty() {
        return;
    }
    let reaches: Vec<f32> = legs.iter().map(|&li| reach(model, &g.legs[usize::from(li)])).collect();
    let min_reach = reaches.iter().copied().fold(f32::INFINITY, f32::min);
    if !min_reach.is_finite() || min_reach < 1e-4 {
        return;
    }
    let moved = cx.inputs.moved;
    let finite_moved = moved.translation.is_finite() && moved.yaw.is_finite();
    if !finite_moved || horizontal(moved.translation).length() > min_reach {
        cx.feet.teleports += 1;
        ls.fill(LegState::default());
    }

    // The op's weight: on while ground samples arrive and the character is
    // slower than `max_speed`; the first update starts where it should be.
    let sampled = legs.iter().any(|&li| matches!(cx.inputs.ground.get(usize::from(li)), Some(Some(_))));
    let speed = horizontal(cx.inputs.velocity).length();
    let goal = if sampled && speed < guards.max_speed { 1.0 } else { 0.0 };
    let fresh = weight.is_none();
    let w = weight.get_or_insert(Spring { value: goal, velocity: 0.0 }).update(goal, guards.blend_halflife, dt).clamp(0.0, 1.0);
    if w < 1e-3 {
        ls.fill(LegState::default());
        springs.fill((Spring::default(), Spring::default()));
        return;
    }

    let unmove = Quat::from_rotation_y(-moved.yaw);
    let mut targets: Vec<FootTarget<'_>> = Vec::with_capacity(legs.len());
    for (k, &li) in legs.iter().enumerate() {
        let leg = &g.legs[usize::from(li)];
        let st = &mut ls[k];
        if finite_moved {
            for l in st.lock.iter_mut().chain(&mut st.toe_lock) {
                *l = unmove * (*l - moved.translation);
            }
            // A displacement between two points of the character's frame.
            st.offset = unmove * st.offset;
            st.offset_velocity = unmove * st.offset_velocity;
        }
        let foot = usize::from(leg.effector());
        let f = model.translation(foot);
        let mut lift = 0.0;
        let mut normal = None;
        if let Some(Some(gs)) = cx.inputs.ground.get(usize::from(li)) {
            let n = gs.normal.normalize_or(Vec3::Y);
            let ground = gs.point.y - (n.x * (f.x - gs.point.x) + n.z * (f.z - gs.point.z)) / n.y;
            if n.y > 0.1 && ground.is_finite() && ground.abs() <= guards.max_step * reaches[k] {
                // The sole keeps its clearance along the normal of the
                // ground it tilts to (vertically `sole / cos tilt`).
                let tilt = n.y.clamp(-1.0, 1.0).acos().min(*max_foot_angle);
                // Only rigid feet (a tip beyond the effector); point contacts keep theirs.
                let sole = if leg.toe.is_some() { clearance(leg.sole, reaches[k]) } else { 0.0 };
                lift = ground + sole * (1.0 / tilt.cos() - 1.0);
                normal = Some(n);
                // Step-aware: a tip over higher ground than the heel's plane
                // (a riser between them) lifts the foot until the tip clears
                // that step at its animated height, so toes never sink into
                // the next tread; the heel then hangs over the nosing.
                if let Some((toe, slot, rest)) = leg.toe
                    && let Some(Some(ts)) = cx.inputs.ground.get(usize::from(slot))
                {
                    let t = model.translation(usize::from(toe));
                    let tn = ts.normal.normalize_or(Vec3::Y);
                    let at_toe = ts.point.y - (tn.x * (t.x - ts.point.x) + tn.z * (t.z - ts.point.z)) / tn.y;
                    let plane = gs.point.y - (n.x * (t.x - gs.point.x) + n.z * (t.z - gs.point.z)) / n.y;
                    let need = at_toe - (t.y - clearance(rest, reaches[k]));
                    if tn.y > 0.1 && at_toe - plane > STEP_EDGE && need > lift && need.abs() <= guards.max_step * reaches[k] {
                        lift = need;
                    }
                }
            } else {
                cx.feet.rejected += 1;
            }
        }
        if fresh {
            st.height = Spring { value: lift, velocity: 0.0 };
        }
        let mut target = f + Vec3::Y * st.height.update(lift, guards.foot_halflife, dt);
        let curve = |c: Option<u16>| c.and_then(|c| cx.curves.get(usize::from(c)).copied()).unwrap_or(0.0);
        let contact = curve(leg.contact);
        // Heel roll: the contact is over (heel-off) but the toe is still
        // down: the toe pins, the ankle rises and turns about it as animated.
        let toe = leg.toe.map(|(j, _, _)| model.translation(usize::from(j)));
        let rolling = contact <= 0.5 && curve(leg.roll) > 0.5 && !st.unplanting;
        if let (true, Some(l), Some(t)) = (rolling, st.lock, toe) {
            // The toe under the locked foot (lifted as the foot is).
            st.toe_lock.get_or_insert(l + (t - f));
        } else if contact > 0.5 {
            st.toe_lock = None;
        }
        // The locked target: the ankle's lock, or the animated foot moved
        // onto the pinned toe.
        let toe_pin = st.toe_lock.zip(toe).map(|(l, t)| l + (f - t));
        let pin = if contact > 0.5 { st.lock } else if rolling { toe_pin } else { None };
        if let Some(p) = pin {
            let drift = horizontal(p - target).length();
            cx.feet.drift_max = cx.feet.drift_max.max(drift);
            cx.feet.drift_sum += drift;
            cx.feet.drift_count += 1;
            if !st.unplanting && drift > guards.unplant * reaches[k] {
                st.unplanting = true;
                cx.feet.unplanted += 1;
            }
        }
        if contact > 0.5 && !st.unplanting && st.lock.is_none() {
            // A foot plants on the ground it touches, not on the smoothed
            // height still catching up (stair edges); where it is, with what
            // is left of a release.
            st.height = Spring { value: lift, velocity: 0.0 };
            let l = f + Vec3::Y * lift + st.offset;
            (st.lock, st.offset, st.offset_velocity) = (Some(l), Vec3::ZERO, Vec3::ZERO);
            target = l;
        } else if let (Some(p), false) = (pin, st.unplanting) {
            target = p;
        } else {
            // Released (toe-off, no roll curve, unplanting): the locked
            // target's offset from the animation decays in a critical spring
            // of half-life `release / 2`, so the swing takes the correction
            // rather than the foot sliding at weight transfer.
            if let Some(l) = st.lock.take() {
                st.offset = toe_pin.unwrap_or(l) - target;
                st.offset_velocity = Vec3::ZERO;
                st.toe_lock = None;
            }
            critical_spring(&mut st.offset, &mut st.offset_velocity, Vec3::ZERO, 0.5 * release.max(1e-3), dt);
            if st.offset.length() < 1e-3 {
                *st = LegState { height: st.height, ..LegState::default() };
            } else {
                target += st.offset;
            }
        }
        targets.push((leg, k, f.lerp(target, w), normal, model.rotation(foot)));
    }
    // Per girdle: the offset goal from its legs, and the side-slope roll.
    let mut offsets = Vec::with_capacity(girdles.len());
    let mut rolls = Vec::with_capacity(girdles.len());
    let mut lows = Vec::with_capacity(girdles.len());
    for (gi, gs) in girdles.iter().enumerate() {
        let mut lowest = 0.0f32;
        let (mut sum, mut cap, mut girdle_reach) = (0.0f32, f32::INFINITY, f32::INFINITY);
        let (mut left, mut right) = ((0.0f32, 0.0f32, 0u32), (0.0f32, 0.0f32, 0u32));
        for &k in &gs.legs {
            let (leg, i, target, _, _) = targets[usize::from(k)];
            let f = model.translation(usize::from(leg.effector()));
            let delta = target.y - f.y;
            lowest = lowest.min(delta);
            sum += delta;
            cap = cap.min(rise_cap(model, leg, target));
            girdle_reach = girdle_reach.min(reaches[i]);
            let side = f.x - model.translation(usize::from(leg.girdle)).x;
            if side > 1e-3 {
                left = (left.0 + delta, left.1 + f.x, left.2 + 1);
            } else if side < -1e-3 {
                right = (right.0 + delta, right.1 + f.x, right.2 + 1);
            }
        }
        if !girdle_reach.is_finite() {
            girdle_reach = min_reach;
        }
        let (lo, hi) = (-guards.max_drop * girdle_reach, guards.max_rise * girdle_reach);
        #[allow(clippy::cast_precision_loss)]
        let goal = match body {
            BodyFit::Lower => lowest.min(cap),
            BodyFit::Fit if gs.legs.is_empty() => 0.0,
            BodyFit::Fit => (sum / gs.legs.len() as f32).min(cap),
        };
        let clamped = goal.clamp(lo, hi);
        if clamped < goal - 1e-4 || clamped > goal + 1e-4 {
            cx.feet.pelvis_clamped += 1;
        }
        #[allow(clippy::cast_precision_loss)]
        let roll_goal = if *roll > 0.0 && left.2 > 0 && right.2 > 0 {
            let (dl, xl) = (left.0 / left.2 as f32, left.1 / left.2 as f32);
            let (dr, xr) = (right.0 / right.2 as f32, right.1 / right.2 as f32);
            ((dl - dr).atan2((xl - xr).max(1e-3)) * *roll).clamp(-0.6, 0.6)
        } else {
            0.0
        };
        let (o, r) = &mut springs[gi];
        // The spring smooths; reach and the drop limit bound its output
        // (the drop limit wins: a leg that still can't reach is clamped below).
        offsets.push(o.update(clamped, *pelvis_halflife, dt).min(cap).clamp(lo, hi));
        rolls.push(r.update(roll_goal, *pelvis_halflife, dt));
        lows.push(lo);
    }
    let finite = targets.iter().all(|t| t.2.is_finite()) && offsets.iter().chain(&rolls).all(|v| v.is_finite());
    if !finite {
        cx.feet.non_finite += 1;
        ls.fill(LegState::default());
        springs.fill((Spring::default(), Spring::default()));
        *weight = Some(Spring::default());
        return;
    }
    fit_body(s, local, model, girdles, &offsets, &rolls);
    if *body == BodyFit::Fit {
        // Pitching and rolling moved the hips: if a leg can no longer
        // reach, lower the whole body (every girdle by the worst
        // shortfall, within the drop limit). A pure translation moves each
        // hip by exactly that much, so one pass is exact and the fitted
        // pitch is kept; lowering only the short girdle re-pitched the
        // spine, over-pitched downhill and converged slowly.
        let floor = offsets.iter().zip(&lows).map(|(o, lo)| lo - o).fold(f32::NEG_INFINITY, f32::max).min(0.0);
        let excess = targets.iter().map(|&(leg, _, target, _, _)| rise_cap(model, leg, target)).fold(0.0f32, f32::min).max(floor);
        if excess < -1e-4 {
            fit_body(s, local, model, girdles, &vec![excess; girdles.len()], &vec![0.0; girdles.len()]);
        }
    }
    for (leg, k, mut target, normal, rot) in targets {
        // Never ask a leg for more than it reaches from where its hip is now.
        let hip = model.translation(usize::from(leg.solved()[0]));
        let d = target - hip;
        if d.length() > reaches[k] {
            // Counted when the pull is real (a straight animated leg is
            // already at full length every stride).
            if d.length() > reaches[k] * 1.02 {
                cx.feet.reach_clamped += 1;
            }
            target = hip + d * (reaches[k] / d.length());
        }
        let tilt = normal.map_or(Quat::IDENTITY, |n| clamp_angle(Quat::from_rotation_arc(Vec3::Y, n), *max_foot_angle));
        solve_leg(s, local, model, leg, target, Some(Quat::IDENTITY.slerp(tilt, w) * rot), 1.0);
    }
}

#[cfg(test)]
mod grade_tests {
    use glam::Vec3;

    use super::ground_grade;
    use crate::asset::{GradeLean, GradeStride};
    use crate::instance::GroundSample;

    fn ramp(deg: f32, xs: &[f32]) -> Vec<Option<GroundSample>> {
        let (s, c) = deg.to_radians().sin_cos();
        // Rising along +Z: the normal leans back, toward -Z.
        xs.iter().map(|&z| Some(GroundSample { point: Vec3::new(0.1, z * s / c, z), normal: Vec3::new(0.0, c, -s) })).collect()
    }

    #[test]
    fn grade_reads_ramps_stairs_and_flat_along_the_travel() {
        let forward = Vec3::new(0.0, 0.0, 1.4);
        assert_eq!(ground_grade(&ramp(0.0, &[-0.3, 0.2, 0.0, 0.4]), forward), Some(0.0));
        for deg in [10.0f32, 20.0, 30.0] {
            let g = ground_grade(&ramp(deg, &[-0.3, 0.2, -0.1, 0.4]), forward).unwrap();
            assert!((g - deg.to_radians().tan()).abs() < 1e-4, "{deg}°: {g}");
            // Walking down it, and across it.
            let down = ground_grade(&ramp(deg, &[-0.3, 0.2]), -forward).unwrap();
            assert!((down + deg.to_radians().tan()).abs() < 1e-4, "{deg}° down: {down}");
            let across = ground_grade(&ramp(deg, &[-0.3, 0.2]), Vec3::new(1.4, 0.0, 0.0)).unwrap();
            assert!(across.abs() < 1e-4, "{deg}° across: {across}");
        }
        // Stairs (rise 0.15 per 0.3 tread, flat treads): the heights carry the grade, the
        // flat normals pull it down only a little.
        let tread = |z: f32| Some(GroundSample { point: Vec3::new(0.0, (z / 0.3).floor() * 0.15, z), normal: Vec3::Y });
        let g = ground_grade(&[tread(-0.35), tread(0.05), tread(0.4), tread(0.7)], forward).unwrap();
        assert!((0.4..=0.6).contains(&g), "stairs: {g}");
        assert_eq!(ground_grade(&[None, None], forward), None);
    }

    #[test]
    fn grade_scales_stride_and_pitch_within_caps() {
        let (stride, lean) = (GradeStride { up: 0.45, down: 0.25 }, GradeLean { up: 0.12, down: 0.06, max: 0.07 });
        let g30 = 30f32.to_radians().tan();
        assert!((stride.scale(0.0) - 1.0).abs() < 1e-6 && lean.pitch(0.0).abs() < 1e-6);
        assert!(stride.scale(g30) < stride.scale(0.1) && stride.scale(0.1) < 1.0);
        assert!(stride.scale(-g30) > stride.scale(-0.1) && stride.scale(-0.1) > 1.0);
        assert!(lean.pitch(g30) > 0.0 && lean.pitch(g30) <= 0.07 && lean.pitch(-g30) < 0.0 && lean.pitch(-g30) > -lean.pitch(g30));
        assert!(lean.pitch(1.0) <= 0.07 + 1e-6 && lean.pitch(-1.0) >= -0.07 - 1e-6);
        assert!((GradeStride::default().scale(g30) - 1.0).abs() < 1e-6 && GradeLean::default().pitch(g30).abs() < 1e-6);
    }
}
