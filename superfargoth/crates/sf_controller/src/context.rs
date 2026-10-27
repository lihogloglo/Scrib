//! Context probes: batched environment queries classified into traversal
//! affordances (locomotion.md "Context probes", animation.md layer 3).
//!
//! The method is the procedural traversal check of UE's Game Animation
//! Sample and ALS's mantle system: a capsule sweep finds a face ahead, a
//! column of down-casts behind it gives the obstacle's top, depth and far
//! floor, and room checks above (and below, for overhangs) decide what the
//! character can do. Heights are classified against [`ContextParams`],
//! which scale with the character (a step for a guar is a vault for a
//! scrib). Results are data for the graph: parameters (`height`, `depth`,
//! `clearance`) and motion-warping targets ([`ContextHit::warp_target`],
//! [`ContextHit::hang_point`]); the actions that use them are M4.
//!
//! Every request is answered in at most four stages, one batched
//! [`GroundProbe::probe`] call per stage for all requests, so the probes run
//! on terrain + [`StaticWorld`](crate::StaticWorld) now and on M4's physics
//! queries unchanged.

use glam::Vec3;
use sf_coords::WorldPos;

use crate::mover::MoverParams;
use crate::probe::{GroundHit, GroundProbe, ProbeQuery};

/// Thresholds a character's context probes classify with, meters. Scale
/// with the character ([`ContextParams::from_mover`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ContextParams {
    pub height: f32,
    pub radius: f32,
    /// Obstacles up to this are steps (the mover climbs them).
    pub step_up: f32,
    pub step_down: f32,
    /// Cosine of the slope limit (walkable surfaces are not obstacles).
    pub min_normal_y: f32,
    /// Highest obstacle vaulted or hurdled.
    pub vault_max: f32,
    /// Deepest obstacle vaulted (deeper ones are mantled onto).
    pub vault_depth: f32,
    /// Highest obstacle mantled onto.
    pub mantle_max: f32,
    /// Highest ledge climbed or hung from; above it the obstacle is
    /// `Blocked`. The column of down-casts starts here.
    pub climb_max: f32,
    /// Lowest room to crouch in (on a mantled top, under an overhang).
    pub crouch_height: f32,
    /// A hanging character's feet below the lip, and its root off the face.
    pub hang_drop: f32,
    pub hang_offset: f32,
}

impl ContextParams {
    /// The thresholds for a mover's capsule. Ratios from GASP's traversal
    /// chooser on its 1.8 m mannequin (vault 0.5–1.25 m, at most 0.6 m deep;
    /// mantle up to 2.75 m), climbing to 2.5 heights, crouching at 0.6.
    pub fn from_mover(m: &MoverParams) -> Self {
        let h = m.height;
        Self {
            height: h,
            radius: m.radius,
            step_up: m.step_up,
            step_down: m.step_down,
            min_normal_y: m.min_normal_y(),
            vault_max: 0.7 * h,
            vault_depth: 0.33 * h,
            mantle_max: 1.5 * h,
            climb_max: 2.5 * h,
            crouch_height: 0.6 * h,
            hang_drop: 1.15 * h,
            hang_offset: m.radius,
        }
    }
}

impl Default for ContextParams {
    fn default() -> Self {
        Self::from_mover(&MoverParams::default())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

/// One environment query, relative to a character's feet and a horizontal
/// direction (its travel direction, or a trajectory point's).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ContextQuery {
    /// Capsule sweep up to `range`; on a hit, a column of down-casts over
    /// the obstacle gives its top, depth and far floor, then room checks.
    ObstacleAhead { range: f32 },
    /// Down-cast `reach` ahead from `max_height`, then a face ray and an up
    /// clearance ray: a ledge the hands can grab (hang points).
    Ledge { reach: f32, max_height: f32 },
    /// A fall of at least `min_drop` within `range` that the mover would not
    /// walk down (terrain slope edges included).
    DropAhead { range: f32, min_drop: f32 },
    /// A near-vertical wall within `range` to a side (hand on the wall,
    /// wall-run and slide normals; cover later).
    WallBeside { side: Side, range: f32 },
    /// Room overhead below `height` (crouch).
    Ceiling { height: f32 },
}

/// One request: the query, where it is asked from and with which measures.
#[derive(Clone, Copy, Debug)]
pub struct ContextRequest<'a> {
    pub feet: WorldPos,
    /// Horizontal, normalized.
    pub dir: Vec3,
    pub params: &'a ContextParams,
    pub query: ContextQuery,
}

/// What a probe found, classified by height against the character's measures.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Affordance {
    /// Low enough for the mover to step onto.
    Step,
    /// Thin and low enough to vault or hurdle (`landing` says which: a far
    /// floor near the feet is a hurdle, a lower one a vault).
    Vault,
    /// Climbed onto with the hands, room to at least crouch on top.
    Mantle,
    /// Too high to mantle, low enough to grab: hang and climb.
    Climb,
    /// Too high, or no room on top.
    Blocked,
    Ledge,
    Drop,
    Wall,
    LowCeiling,
}

/// A classified context probe result. Heights are relative to the feet.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ContextHit {
    pub kind: Affordance,
    /// Obstacles and ledges: the lip (front top edge). Drops: the edge
    /// walked off. Walls and ceilings: the contact.
    pub point: WorldPos,
    /// The face's normal, unit, toward the character (outward for a drop's
    /// edge, down for a ceiling); horizontal for faces.
    pub normal: Vec3,
    /// Obstacle or ledge top above the feet; drop depth below them; wall
    /// contact or ceiling height above them.
    pub height: f32,
    /// Obstacle depth from the lip along the direction (`f32::INFINITY`:
    /// deeper than the column); for drops and walls, the distance to them.
    pub depth: f32,
    /// Room above the top (overhangs: below the obstacle), capped at the
    /// scan height.
    pub clearance: f32,
    /// The floor beyond an obstacle or below a drop, if within the scan.
    pub landing: Option<WorldPos>,
}

impl ContextHit {
    /// The root (feet) target for a motion-warping window and its yaw (0
    /// faces +Z): for a mantle or a step, on top `inset` past the lip; for
    /// a vault, the landing; facing into the face. World space: convert
    /// with `CharacterFrame::to_local` for the graph's `WarpTarget`.
    pub fn warp_target(&self, inset: f32) -> (WorldPos, f32) {
        let into = -self.normal;
        let yaw = into.x.atan2(into.z);
        match (self.kind, self.landing) {
            (Affordance::Vault, Some(l)) => (l, yaw),
            _ => (self.point.offset(into * inset), yaw),
        }
    }

    /// The root (feet) position of a character hanging from the lip, and
    /// its yaw: off the face by `hang_offset`, `hang_drop` below the lip.
    pub fn hang_point(&self, params: &ContextParams) -> (WorldPos, f32) {
        let into = -self.normal;
        let p = self.point.offset(self.normal * params.hang_offset - Vec3::Y * params.hang_drop);
        (p, into.x.atan2(into.z))
    }

    /// For a wall: the run direction along it closest to `dir`.
    pub fn wall_tangent(&self, dir: Vec3) -> Vec3 {
        let t = self.normal.cross(Vec3::Y).normalize_or_zero();
        if t.dot(dir) < 0.0 { -t } else { t }
    }
}

/// Down-casts in an obstacle column.
const COLUMN: usize = 3;
/// Down-casts a drop probe spreads over its range.
const DROP_SAMPLES: usize = 4;
/// How far rays start inside or past a face, meters.
const NUDGE: f32 = 0.05;

#[derive(Clone, Copy, Debug, Default)]
enum Phase {
    #[default]
    Start,
    /// Obstacle: the face contact (relative to the feet), its horizontal
    /// normal and the contact normal's Y.
    Face {
        at: Vec3,
        normal: Vec3,
        slope: f32,
    },
    /// Obstacle: top, column heights resolved; the room checks pending.
    Column {
        at: Vec3,
        normal: Vec3,
        top: f32,
        off: Option<usize>,
        heights: [Option<f32>; COLUMN],
    },
    /// Obstacle with nothing under its face: the underside pending.
    Overhang {
        hit: ContextHit,
    },
    /// Ledge: the top found `t` above the feet.
    LedgeTop {
        top: f32,
        at: Vec3,
    },
    /// Drop: the fall between column samples `k - 1` and `k`.
    DropEdge {
        k: usize,
        step: f32,
        from: f32,
        landing: Option<GroundHit>,
    },
    Done,
}

/// Runs batches of [`ContextRequest`]s. Keep one and reuse it (it owns
/// the batch's scratch).
#[derive(Default)]
pub struct ContextProbes {
    phases: Vec<Phase>,
    queries: Vec<ProbeQuery>,
    hits: Vec<Option<GroundHit>>,
    ranges: Vec<(u32, u32)>,
}

fn rel(feet: WorldPos, p: WorldPos) -> Vec3 {
    feet.delta_f32(p)
}

fn horizontal(v: Vec3, fallback: Vec3) -> Vec3 {
    Vec3::new(v.x, 0.0, v.z).try_normalize().unwrap_or(fallback)
}

impl ContextProbes {
    pub fn new() -> Self {
        Self::default()
    }

    /// Answers every request: `out[i]` receives request `i`'s hit (`None`:
    /// nothing there, or nothing worth reporting). At most four batched
    /// probe calls in all.
    pub fn run(&mut self, requests: &[ContextRequest<'_>], probe: &dyn GroundProbe, out: &mut [Option<ContextHit>]) {
        out.fill(None);
        self.phases.clear();
        self.phases.resize(requests.len(), Phase::Start);
        for _ in 0..4 {
            if !self.stage(requests, probe, out) {
                break;
            }
        }
    }

    /// One stage: every pending request's casts in one probe call. False
    /// when no request had casts left.
    fn stage(&mut self, requests: &[ContextRequest<'_>], probe: &dyn GroundProbe, out: &mut [Option<ContextHit>]) -> bool {
        self.queries.clear();
        self.ranges.clear();
        for (r, phase) in requests.iter().zip(&self.phases) {
            let start = self.queries.len();
            push(r, phase, &mut self.queries);
            #[allow(clippy::cast_possible_truncation)]
            self.ranges.push((start as u32, self.queries.len() as u32));
        }
        if self.queries.is_empty() {
            return false;
        }
        self.hits.clear();
        self.hits.resize(self.queries.len(), None);
        probe.probe(&self.queries, &mut self.hits);
        for (i, r) in requests.iter().enumerate() {
            let (a, b) = self.ranges[i];
            if a < b {
                self.phases[i] = take(r, self.phases[i], &self.hits[a as usize..b as usize], &mut out[i]);
            }
        }
        true
    }
}

/// The column's down-cast offsets past the face, meters: two close ones
/// (the top is flat, not a ramp) and one past the deepest vault (the far
/// side; the back ray then measures the depth exactly).
fn column_offsets(p: &ContextParams) -> [f32; COLUMN] {
    [NUDGE, NUDGE + 0.1, 1.5 * p.vault_depth]
}

/// The height of the ray that checks for an overhang (nothing under the
/// lip), if one is needed: not when the sweep touched the face below it.
fn mid_height(contact: Vec3, top: f32, p: &ContextParams) -> Option<f32> {
    let mid = 0.5 * top.max(p.step_up);
    (top > p.step_up && contact.y >= mid).then_some(mid)
}

/// A request's casts for its current phase.
#[allow(clippy::cast_precision_loss)]
fn push(r: &ContextRequest<'_>, phase: &Phase, q: &mut Vec<ProbeQuery>) {
    let p = r.params;
    let (f, d) = (r.feet, r.dir);
    let at = |v: Vec3| f.offset(v);
    match (*phase, r.query) {
        (Phase::Start, ContextQuery::ObstacleAhead { range }) => {
            // The mover's wall sweep, from a quarter step up (lower is a curb).
            let lift = 0.25 * p.step_up;
            let span = (p.height - lift).max(2.0 * p.radius);
            let radius = p.radius.min(span / 2.0);
            q.push(ProbeQuery::capsule(at(Vec3::Y * (lift + span / 2.0)), d, range, radius, span / 2.0 - radius));
        }
        (Phase::Face { at: c, normal, .. }, ContextQuery::ObstacleAhead { .. }) => {
            let into = -normal;
            let base = Vec3::new(c.x, p.climb_max + NUDGE, c.z);
            // From the climb limit down past the feet (a vault's far floor).
            for o in column_offsets(p) {
                q.push(ProbeQuery::down(at(base + into * o), p.climb_max + NUDGE + p.step_down + p.height));
            }
        }
        (Phase::Column { at: c, normal, top, off, .. }, ContextQuery::ObstacleAhead { .. }) => {
            let into = -normal;
            let lip = Vec3::new(c.x, top, c.z);
            // Room on top, where the character would stand.
            q.push(ProbeQuery::ray(at(lip + into * p.radius.min(0.5 * p.vault_depth) + Vec3::Y * 0.02), Vec3::Y, p.height + NUDGE));
            // The face's middle: a miss means nothing stands under the lip.
            // Its length reaches the face's plane along `d` (a slanted approach meets it farther
            // than the sweep's contact point is; that length missed and read a box as an overhang).
            if let Some(mid) = mid_height(c, top, p) {
                let flat = Vec3::new(c.x, 0.0, c.z);
                let slant = d.dot(normal);
                let reach = if slant < -0.05 { flat.dot(normal) / slant } else { flat.length() };
                q.push(ProbeQuery::ray(at(Vec3::Y * mid), d, reach.max(flat.length()) + NUDGE));
            }
            // The far face, back from the first sample off the top.
            if let Some(k) = off {
                let o = column_offsets(p)[k];
                let back = lip + into * o + Vec3::Y * (-0.03);
                q.push(ProbeQuery::ray(at(back), normal, o));
            }
        }
        (Phase::Overhang { hit }, _) => {
            // The underside, from the feet just inside the face.
            let c = rel(f, hit.point);
            q.push(ProbeQuery::ray(at(Vec3::new(c.x, 0.02, c.z) - hit.normal * NUDGE), Vec3::Y, hit.height));
        }
        (Phase::Start, ContextQuery::Ledge { reach, max_height }) => {
            q.push(ProbeQuery::down(at(d * reach + Vec3::Y * max_height), max_height - p.step_up));
            q.push(ProbeQuery::ray(at(Vec3::Y * max_height), d, reach + NUDGE));
        }
        (Phase::LedgeTop { top, at: c }, ContextQuery::Ledge { reach, .. }) => {
            q.push(ProbeQuery::ray(at(Vec3::Y * (top - NUDGE)), d, reach + NUDGE));
            q.push(ProbeQuery::ray(at(c + Vec3::Y * 0.02), Vec3::Y, p.height + NUDGE));
        }
        (Phase::Start, ContextQuery::DropAhead { range, min_drop }) => {
            let step = range / DROP_SAMPLES as f32;
            for k in 1..=DROP_SAMPLES {
                q.push(ProbeQuery::down(at(d * (step * k as f32) + Vec3::Y * p.step_up), p.step_up + min_drop.max(p.height) * 4.0));
            }
            q.push(ProbeQuery::ray(at(Vec3::Y * (0.5 * p.height)), d, range));
        }
        (Phase::DropEdge { k, step, from, .. }, ContextQuery::DropAhead { .. }) => {
            // Back from the sample past the edge, just below the top: the edge's face.
            q.push(ProbeQuery::ray(at(d * (step * (k + 1) as f32) + Vec3::Y * (from - NUDGE)), -d, step + NUDGE));
        }
        (Phase::Start, ContextQuery::WallBeside { side, range }) => {
            let right = d.cross(Vec3::Y);
            let s = if side == Side::Right { right } else { -right };
            q.push(ProbeQuery::ray(at(Vec3::Y * (0.6 * p.height)), s, range));
        }
        (Phase::Start, ContextQuery::Ceiling { height }) => {
            let r = 0.9 * p.radius;
            let from = p.step_up + r;
            q.push(ProbeQuery::sphere(at(Vec3::Y * from), Vec3::Y, (height - from - r).max(0.0), r));
        }
        _ => {}
    }
}

/// Consumes a request's hits for its phase; the next phase.
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::too_many_lines)]
fn take(r: &ContextRequest<'_>, phase: Phase, h: &[Option<GroundHit>], out: &mut Option<ContextHit>) -> Phase {
    let p = r.params;
    let (f, d) = (r.feet, r.dir);
    let y = |hit: &GroundHit| rel(f, hit.point).y;
    match (phase, r.query) {
        (Phase::Start, ContextQuery::ObstacleAhead { .. }) => match h[0] {
            Some(hit) => Phase::Face { at: rel(f, hit.point), normal: horizontal(hit.normal, -d), slope: hit.normal.y },
            None => Phase::Done,
        },
        (Phase::Face { at, normal, slope }, ContextQuery::ObstacleAhead { .. }) => {
            let heights: [Option<f32>; COLUMN] = std::array::from_fn(|k| h[k].as_ref().map(y));
            // A top below where the sweep touched the face means the column
            // started inside the obstacle (or past a thin wall): taller than
            // the climb limit.
            if heights[0].is_none_or(|top| top < at.y - NUDGE) {
                *out = Some(ContextHit {
                    kind: Affordance::Blocked,
                    point: f.offset(Vec3::new(at.x, p.climb_max, at.z)),
                    normal,
                    height: p.climb_max,
                    depth: f32::INFINITY,
                    clearance: 0.0,
                    landing: None,
                });
                return Phase::Done;
            }
            let Some(top) = heights[0] else { return Phase::Done };
            // A walkable slope rises on past the face: the mover's, not an obstacle.
            let flat = heights[1].is_none_or(|h1| (h1 - top).abs() < 0.01);
            if !flat && slope >= p.min_normal_y {
                return Phase::Done;
            }
            let off = heights.iter().position(|hk| hk.is_none_or(|hk| top - hk > 0.5 * p.step_up));
            Phase::Column { at, normal, top, off, heights }
        }
        (Phase::Column { at, normal, top, off, heights }, ContextQuery::ObstacleAhead { .. }) => {
            let lip = f.offset(Vec3::new(at.x, top, at.z));
            let clearance = h[0].map_or(p.height + NUDGE, |c| c.distance + 0.02);
            let offsets = column_offsets(p);
            let mid = mid_height(at, top, p).map(|_| h[1]);
            let back = if off.is_some() { h[1 + usize::from(mid.is_some())] } else { None };
            let depth = match (off, back) {
                // The far face: the back ray hits it `distance` short of its sample.
                (Some(k), Some(back)) => (offsets[k] - back.distance).max(0.0),
                (Some(k), None) => offsets[k],
                (None, _) => f32::INFINITY,
            };
            let landing = off.and_then(|k| heights[k].map(|hk| lip.offset(-normal * offsets[k] + Vec3::Y * (hk - top))));
            let mut hit = ContextHit { kind: Affordance::Blocked, point: lip, normal, height: top, depth, clearance, landing };
            if mid.is_some_and(|m| m.is_none()) {
                // Nothing under the face: an overhang to duck under (or climb).
                hit.kind = Affordance::LowCeiling;
                *out = Some(hit);
                return Phase::Overhang { hit };
            }
            let lands_lower = landing.is_some_and(|l| top - rel(f, l).y >= p.step_up) || off.is_some() && landing.is_none();
            hit.kind = if top <= p.step_up {
                Affordance::Step
            } else if top <= p.vault_max && depth <= p.vault_depth && lands_lower {
                Affordance::Vault
            } else if top <= p.mantle_max && clearance >= p.crouch_height {
                Affordance::Mantle
            } else if top <= p.climb_max && clearance >= p.crouch_height {
                Affordance::Climb
            } else {
                Affordance::Blocked
            };
            *out = Some(hit);
            Phase::Done
        }
        (Phase::Overhang { mut hit }, _) => {
            // Room under the obstacle; a crouch fits or it blocks.
            let under = h[0].map_or(hit.height, |u| u.distance + 0.02);
            hit.kind = if under >= p.height {
                // Tall enough to walk under: not in the way after all.
                *out = None;
                return Phase::Done;
            } else if under >= p.crouch_height {
                hit.clearance = under;
                Affordance::LowCeiling
            } else if hit.height <= p.mantle_max && hit.clearance >= p.crouch_height {
                // Too low to duck under: a slab to climb onto (clearance stays the top's).
                Affordance::Mantle
            } else if hit.height <= p.climb_max && hit.clearance >= p.crouch_height {
                Affordance::Climb
            } else {
                Affordance::Blocked
            };
            *out = Some(hit);
            Phase::Done
        }
        (Phase::Start, ContextQuery::Ledge { .. }) => match (h[0], h[1]) {
            (Some(top), None) => Phase::LedgeTop { top: y(&top), at: rel(f, top.point) },
            _ => Phase::Done,
        },
        (Phase::LedgeTop { top, at }, ContextQuery::Ledge { reach, .. }) => {
            let Some(face) = h[0] else { return Phase::Done };
            let c = rel(f, face.point);
            let normal = horizontal(face.normal, -d);
            let face_dist = Vec3::new(c.x, 0.0, c.z).length();
            *out = Some(ContextHit {
                kind: Affordance::Ledge,
                point: f.offset(Vec3::new(c.x, top, c.z)),
                normal,
                height: top,
                depth: (reach - face_dist).max(0.0),
                clearance: h[1].map_or(p.height + NUDGE, |u| u.distance + 0.02),
                landing: Some(f.offset(at)),
            });
            Phase::Done
        }
        (Phase::Start, ContextQuery::DropAhead { range, min_drop }) => {
            if h[DROP_SAMPLES].is_some() {
                return Phase::Done; // A wall comes first.
            }
            let step = range / DROP_SAMPLES as f32;
            // A fall the mover would not walk: more than a step plus the slope limit's rise.
            let tan = (1.0 - p.min_normal_y * p.min_normal_y).sqrt() / p.min_normal_y.max(1e-3);
            let limit = p.step_down + step * tan;
            let mut from = 0.0f32;
            for k in 0..DROP_SAMPLES {
                let hk = h[k].as_ref().map(y);
                if hk.is_none_or(|hk| from - hk > limit) {
                    // The fall goes on while the samples keep falling steeply (a cliff
                    // slope spans several): land on the last of them.
                    let mut last = k;
                    while last + 1 < DROP_SAMPLES {
                        match (h[last], h[last + 1]) {
                            (Some(a), Some(b)) if y(&a) - y(&b) > limit => last += 1,
                            _ => break,
                        }
                    }
                    let drop = h[last].as_ref().map_or(f32::INFINITY, |l| from - y(l));
                    if drop < min_drop {
                        return Phase::Done;
                    }
                    return Phase::DropEdge { k, step, from, landing: h[last] };
                }
                from = hk.unwrap_or(from);
            }
            Phase::Done
        }
        (Phase::DropEdge { k, step, from, landing }, ContextQuery::DropAhead { .. }) => {
            // The edge's face if the back ray found one, else midway between samples.
            let (edge, normal) = match h[0] {
                Some(face) => (rel(f, face.point), horizontal(face.normal, d)),
                None => (d * (step * (k as f32 + 0.5)), d),
            };
            let edge = Vec3::new(edge.x, from, edge.z);
            *out = Some(ContextHit {
                kind: Affordance::Drop,
                point: f.offset(edge),
                normal,
                height: landing.as_ref().map_or(f32::INFINITY, |l| from - y(l)),
                depth: Vec3::new(edge.x, 0.0, edge.z).length(),
                clearance: f32::INFINITY,
                landing: landing.map(|l| l.point),
            });
            Phase::Done
        }
        (Phase::Start, ContextQuery::WallBeside { .. }) => {
            if let Some(w) = h[0].filter(|w| w.normal.y.abs() < 0.3) {
                *out = Some(ContextHit {
                    kind: Affordance::Wall,
                    point: w.point,
                    normal: horizontal(w.normal, -d),
                    height: y(&w),
                    depth: w.distance,
                    clearance: f32::INFINITY,
                    landing: None,
                });
            }
            Phase::Done
        }
        (Phase::Start, ContextQuery::Ceiling { .. }) => {
            if let Some(c) = h[0] {
                let height = y(&c);
                *out = Some(ContextHit {
                    kind: Affordance::LowCeiling,
                    point: c.point,
                    normal: c.normal,
                    height,
                    depth: 0.0,
                    clearance: height,
                    landing: None,
                });
            }
            Phase::Done
        }
        _ => Phase::Done,
    }
}
