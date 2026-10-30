//! Wall brace (action.md B2c): a free hand plants on a wall within arm's reach while the
//! character moves along or into it, the host feeding the target and weight to the same hand IK
//! as the ledge grip ([`crate::LedgeGrip`]; the grip wins during a mantle). Presentation only:
//! nothing here feeds gameplay.
//!
//! The standard technique (AC1's `place_hand`, ALS's wall touch, Mirror's Edge's wall-braced
//! arm; research 04 §4 and 02 §5): a few rays from each shoulder find the wall, a short ray into
//! the wall confirms the palm point, and two-bone IK puts the wrist there, its weight blended
//! in and out by proximity and approach. Per frame:
//!
//! - **Fan.** From each shoulder, [`BRACE_FAN`] rays from straight ahead to that hand's side
//!   find the nearest near-vertical face turned toward the character. The fan reaches the arm
//!   plus a band that grows with the speed into the wall ([`BRACE_LOOKAHEAD`]), so a hand
//!   running into a wall rises before the impact.
//! - **Target.** The wrist sits [`BRACE_PALM_GAP`] off the face, the arm at [`BRACE_BEND`] of
//!   its length (slightly bent): on the face, offset from the shoulder's foot point toward a
//!   preferred direction (down, out to the hand's side, a little forward; trailing back along
//!   the wall with the speed past it, [`BRACE_TRAIL`]). Close to the face the arm straightens
//!   (up to [`BRACE_STRAIGHT`]) until its elbow, bent any way, clears the face by
//!   [`BRACE_ELBOW_ROOM`] ("arms out of walls"); no such length: no brace. A wall farther than the bent arm gets
//!   the arm reaching straight at it, faded by the band. A ray into the face at the palm point
//!   ([`BRACE_CONFIRM`]) snaps the wrist to the real surface; a miss (past the wall's end, a
//!   doorway) drops the hand.
//! - **Weight.** Goal = proximity × level: level [`BRACE_LIGHT`] walking past, rising to 1 with
//!   the speed into the wall (the intent's or the motion's, whichever pushes harder) at
//!   [`BRACE_FIRM_SPEED`]. 0 below [`BRACE_MIN_SPEED`], for a busy hand, or while the host
//!   says the brace is inactive (in the air, an action clip or warp playing). The weight moves
//!   to the goal at [`BRACE_RISE`] / [`BRACE_FALL`] per second; the target follows the
//!   solved point with [`BRACE_HALFLIFE`] once a hand is braced.
//!
//! Positions are character space (the [`CharacterFrame`] the host draws: feet, Y up, facing +Z,
//! +X the character's left), meters.

use glam::Vec3;
use sf_controller::{CharacterFrame, GroundHit, GroundProbe, ProbeQuery};

use crate::grip::{LEFT, RIGHT};

/// Fan ray yaws from straight ahead out to the hand's side, radians (0°, 35°, 70°, 90°).
pub const BRACE_FAN: [f32; 4] = [0.0, 0.61, 1.22, std::f32::consts::FRAC_PI_2];
/// The wrist joint's distance off the face (the palm's thickness under it), m.
pub const BRACE_PALM_GAP: f32 = 0.03;
/// Shoulder-to-wrist distance of a braced arm, fraction of the arm's length.
pub const BRACE_BEND: f32 = 0.9;
/// Most the arm straightens to keep the elbow off a close face, fraction of its length.
pub const BRACE_STRAIGHT: f32 = 0.98;
/// Least room between the elbow (whatever its bend direction) and the face, m.
pub const BRACE_ELBOW_ROOM: f32 = 0.02;
/// Seconds of approach the fan looks past the bent arm (band = 0.1 m + speed into the wall × this).
pub const BRACE_LOOKAHEAD: f32 = 0.25;
/// Fade band past the bent arm at no speed into the wall, m.
pub const BRACE_BAND: f32 = 0.1;
/// Level walking past a wall (no speed into it).
pub const BRACE_LIGHT: f32 = 0.5;
/// Speed into the wall for a firm brace (level 1), m/s.
pub const BRACE_FIRM_SPEED: f32 = 2.5;
/// Below this planar speed (intent and motion) no hand braces, m/s.
pub const BRACE_MIN_SPEED: f32 = 0.4;
/// Weight rise and fall rates, per second.
pub const BRACE_RISE: f32 = 6.0;
pub const BRACE_FALL: f32 = 2.0;
/// Halflife of a braced target following the solved point, s.
pub const BRACE_HALFLIFE: f32 = 0.05;
/// Palm trailing back along the wall: m per m/s of speed past it, and the most, m.
pub const BRACE_TRAIL: (f32, f32) = (0.08, 0.2);
/// The confirm ray starts this far out of the face and reaches this far past it, m.
pub const BRACE_CONFIRM: (f32, f32) = (0.2, 0.08);
/// Faces whose normal has a larger vertical part are not walls.
pub const BRACE_MAX_NORMAL_Y: f32 = 0.3;

/// Preferred palm direction on the face (character space, before projection): down, out to the
/// hand's side (`x` signed per hand), a little forward.
const PREFER: Vec3 = Vec3::new(0.8, -1.0, 0.3);

/// One frame's body for the brace (character space, meters).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BraceBody {
    /// Shoulder (upper-arm root) per hand, `[left, right]` ([`LEFT`], [`RIGHT`]).
    pub shoulders: [Vec3; 2],
    /// Shoulder-to-wrist length per hand.
    pub arms: [f32; 2],
    /// Planar velocity the intent asks for and the motion has (character axes).
    pub desired: Vec3,
    pub velocity: Vec3,
    /// Hands free to brace (not holding a weapon, torch or item).
    pub free: [bool; 2],
    /// The brace may run (grounded, plain locomotion: no action clip or warp).
    pub active: bool,
}

impl BraceBody {
    /// Shoulders and arms estimated from the character's height (no pose yet): shoulders at
    /// 0.82 h, 0.105 h to each side; arms 0.3 h. Still, both hands free, active.
    #[must_use]
    pub fn estimate(height: f32) -> Self {
        let s = Vec3::new(0.105 * height, 0.82 * height, 0.0);
        Self {
            shoulders: [s, s * Vec3::new(-1.0, 1.0, 1.0)],
            arms: [0.3 * height; 2],
            desired: Vec3::ZERO,
            velocity: Vec3::ZERO,
            free: [true; 2],
            active: true,
        }
    }
}

/// One hand's brace this frame.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HandBrace {
    /// The wrist target (character space); `None` with weight 0.
    pub target: Option<Vec3>,
    /// The face normal at the target, toward the character (character space).
    pub normal: Vec3,
    /// IK weight, 0-1.
    pub weight: f32,
}

/// Both hands' brace state, kept per character across frames.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WallBrace {
    pub hands: [HandBrace; 2],
}

/// A hand's solved point before smoothing.
#[derive(Clone, Copy, Debug)]
struct Solved {
    wrist: Vec3,
    normal: Vec3,
    goal: f32,
}

impl WallBrace {
    /// Probes the walls around `body` (drawn at `frame`) and moves each hand's target and weight
    /// by `dt` seconds. Two batched probe calls (the fan, then the confirm rays).
    pub fn update(&mut self, frame: &CharacterFrame, probe: &(impl GroundProbe + ?Sized), body: &BraceBody, dt: f32) -> [HandBrace; 2] {
        let dt = if dt.is_finite() { dt.max(0.0) } else { 0.0 };
        let solved = solve(frame, probe, body);
        for (h, s) in self.hands.iter_mut().zip(solved) {
            let goal = s.map_or(0.0, |s| s.goal);
            h.weight = if goal > h.weight { (h.weight + BRACE_RISE * dt).min(goal) } else { (h.weight - BRACE_FALL * dt).max(goal) };
            match s {
                Some(s) if h.target.is_some() => {
                    let k = 1.0 - 0.5f32.powf(dt / BRACE_HALFLIFE);
                    h.normal = h.normal.lerp(s.normal, k).normalize_or(s.normal);
                    // Smoothed in character space, but never behind the face: the body moving
                    // toward the wall brings last frame's point closer to it.
                    h.target = h.target.map(|t| {
                        let t = t.lerp(s.wrist, k);
                        t + s.normal * (s.wrist - t).dot(s.normal).max(0.0)
                    });
                }
                Some(s) => {
                    h.target = Some(s.wrist);
                    h.normal = s.normal;
                }
                // No wall: keep the last target while the weight fades out.
                None => {}
            }
            if h.weight <= 0.0 || !h.target.is_some_and(Vec3::is_finite) {
                *h = HandBrace::default();
            }
        }
        self.hands
    }
}

/// A hand's wrist on the nearest face, before the confirm ray.
#[derive(Clone, Copy, Debug)]
struct Planned {
    wrist: Vec3,
    normal: Vec3,
    goal: f32,
    /// The palm's point on the face (the shoulder's foot point for a reaching arm).
    palm: Vec3,
    /// Within the bent arm (else reaching at the wall).
    on_wall: bool,
}

/// The hands' solved points: the fan, the target on the face, the confirm ray.
fn solve(frame: &CharacterFrame, probe: &(impl GroundProbe + ?Sized), body: &BraceBody) -> [Option<Solved>; 2] {
    let speed = body.desired.length().max(body.velocity.length());
    let usable = |i: usize| {
        body.active && body.free[i] && speed >= BRACE_MIN_SPEED && body.arms[i] > 0.0 && body.shoulders[i].is_finite() && body.arms[i].is_finite()
    };
    if !(usable(LEFT) || usable(RIGHT)) {
        return [None, None];
    }
    let rot = frame.rotation();
    // Speed into a face with normal `n` (toward the character).
    let into = |n: Vec3| (-body.desired.dot(n)).max(-body.velocity.dot(n)).max(0.0);
    let band_max = BRACE_BAND + BRACE_FIRM_SPEED.max(speed) * BRACE_LOOKAHEAD;

    // The fan, one batch for both hands.
    let fan = BRACE_FAN.len();
    let mut queries = Vec::with_capacity(2 * fan);
    for (i, side) in [(LEFT, 1.0), (RIGHT, -1.0)] {
        let reach = BRACE_BEND * body.arms[i] + band_max;
        for &a in &BRACE_FAN {
            let dir = Vec3::new(side * a.sin(), 0.0, a.cos());
            queries.push(ProbeQuery::ray(frame.to_world(body.shoulders[i]), rot * dir, reach));
        }
    }
    let mut hits = vec![None; queries.len()];
    probe.probe(&queries, &mut hits);

    // Per hand: the nearest face, the wrist on it.
    let mut planned: [Option<Planned>; 2] = [None, None];
    for (i, side) in [(LEFT, 1.0f32), (RIGHT, -1.0)] {
        if !usable(i) {
            continue;
        }
        let shoulder = body.shoulders[i];
        let face = hits[i * fan..(i + 1) * fan]
            .iter()
            .zip(&queries[i * fan..(i + 1) * fan])
            .filter_map(|(h, q)| h.map(|h| (local_hit(frame, &h), frame.dir_to_local(q.dir))))
            .filter(|((p, n), dir)| p.is_finite() && n.y.abs() < BRACE_MAX_NORMAL_Y && n.dot(*dir) < -0.2)
            .map(|((p, n), _)| {
                let n = Vec3::new(n.x, 0.0, n.z).normalize_or_zero();
                (p, n, (shoulder - p).dot(n))
            })
            .filter(|(_, n, d)| *n != Vec3::ZERO && *d > 0.0)
            .min_by(|a, b| a.2.total_cmp(&b.2));
        let Some((_, n, d)) = face else { continue };
        let bent = BRACE_BEND * body.arms[i];
        let reach = d - BRACE_PALM_GAP; // shoulder to the wrist's plane
        let v_in = into(n);
        let band = BRACE_BAND + v_in * BRACE_LOOKAHEAD;
        if reach > bent + band {
            continue;
        }
        // Proximity: 1 within the bent arm, fading over the band past it.
        let near = smooth((bent + band - reach) / band);
        let level = BRACE_LIGHT + (1.0 - BRACE_LIGHT) * smooth(v_in / BRACE_FIRM_SPEED);
        // Preferred direction on the face, trailing back with the speed past it.
        let past = body.velocity - n * body.velocity.dot(n);
        let past = Vec3::new(past.x, 0.0, past.z);
        let trail = (BRACE_TRAIL.0 * past.length()).min(BRACE_TRAIL.1);
        let mut prefer = PREFER * Vec3::new(side, 1.0, 1.0) - past.normalize_or_zero() * (trail / BRACE_TRAIL.1);
        prefer -= n * prefer.dot(n);
        let u = prefer.normalize_or(Vec3::NEG_Y);
        let foot = shoulder - n * reach.max(0.0);
        let on_wall = reach <= bent;
        let wrist = if on_wall {
            // The bent arm, straightened as far as needed for the elbow to clear the face.
            let Some(w) = (0..=4).find_map(|k| {
                #[allow(clippy::cast_precision_loss)]
                let len = bent + (BRACE_STRAIGHT * body.arms[i] - bent).max(0.0) * k as f32 / 4.0;
                let w = foot + u * (len * len - reach * reach).max(0.0).sqrt();
                (elbow_room(shoulder, w, body.arms[i], n, d) >= BRACE_ELBOW_ROOM).then_some(w)
            }) else {
                continue;
            };
            w
        } else {
            shoulder - n * bent
        };
        let palm = if on_wall { wrist } else { foot } - n * BRACE_PALM_GAP;
        planned[i] = Some(Planned { wrist, normal: n, goal: near * level, palm, on_wall });
    }

    // Confirm each in-reach palm on the real surface.
    let mut confirm = Vec::with_capacity(2);
    let mut owner = Vec::with_capacity(2);
    for (i, p) in planned.iter().enumerate() {
        if let Some(p) = p {
            let n = p.normal;
            confirm.push(ProbeQuery::ray(frame.to_world(p.palm + n * BRACE_CONFIRM.0), rot * -n, BRACE_CONFIRM.0 + BRACE_CONFIRM.1));
            owner.push(i);
        }
    }
    let mut checks = vec![None; confirm.len()];
    if !confirm.is_empty() {
        probe.probe(&confirm, &mut checks);
    }
    let mut out = [None, None];
    for (k, i) in owner.into_iter().enumerate() {
        let Some(plan) = planned[i] else { continue };
        let Some((p, hn)) = checks[k].map(|h| local_hit(frame, &h)) else { continue };
        let hn = Vec3::new(hn.x, 0.0, hn.z).normalize_or_zero();
        if hn == Vec3::ZERO || hn.dot(plan.normal) < 0.5 || !p.is_finite() {
            continue;
        }
        // A reaching arm (wall past the bent arm) keeps its point in the air; an arm on the
        // wall puts the wrist the palm's gap off the surface found.
        let wrist = if plan.on_wall { p + hn * BRACE_PALM_GAP } else { plan.wrist };
        out[i] = Some(Solved { wrist, normal: hn, goal: plan.goal });
    }
    out
}

/// A hit's point and normal in character space.
fn local_hit(frame: &CharacterFrame, h: &GroundHit) -> (Vec3, Vec3) {
    (frame.to_local(h.point), frame.dir_to_local(h.normal))
}

/// Smoothstep of `x` clamped to 0-1.
fn smooth(x: f32) -> f32 {
    let x = x.clamp(0.0, 1.0);
    x * x * (3.0 - 2.0 * x)
}

/// Least distance from the face to the elbow of a two-bone arm (`arm` long, bones taken equal)
/// from `shoulder` to `wrist`, whatever its bend direction: the elbow circles the segment's
/// midpoint, perpendicular to it. `n` is the face normal, `d` the shoulder's distance to it.
fn elbow_room(shoulder: Vec3, wrist: Vec3, arm: f32, n: Vec3, d: f32) -> f32 {
    let span = wrist - shoulder;
    let half = span.length() / 2.0;
    let radius = ((arm / 2.0).powi(2) - half * half).max(0.0).sqrt();
    let cos = span.normalize_or_zero().dot(n);
    let mid = d + span.dot(n) / 2.0;
    mid - radius * (1.0 - cos * cos).max(0.0).sqrt()
}
