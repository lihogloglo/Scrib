//! River joins (water-bodies.md "FL-4: joins"): each river's two ends are resolved against the
//! rivers it names as receivers, the lakes and the sea, and every river point gets a
//! [`RiverPoint::river_ness`] (1 = the river's own water, 0 = the receiver's water shows). Pure and
//! Bevy-free; a reader calls [`resolve`] once on a worldspace's rivers before building their bodies.
//!
//! The fade lies only over the overlap of the two bodies, where the receiver's water already is
//! (industry: the UE Water plugin's river-to-lake and river-to-ocean transitions blend over the
//! overlap), so a river never thins over its own dry bed. A tributary's last point is snapped onto
//! the receiver's centreline so the two ribbons meet.

use glam::DVec2;
use sf_tunables::{Def, Group, Lifetime, Scope, Source, Unit, def};

use crate::query::in_ring;
use crate::records::RiverPoint;

/// `water.join.mix_widths`: [`JoinSettings::mix_widths`].
pub const MIX_WIDTHS: Def<f32> = def!("water.join.mix_widths" = 2.5_f32)
    .range(0.5, 8.0)
    .unit(Unit::Ratio)
    .help("River joins: the river's water fades into the receiver's over at most this many river widths")
    .source(Source::Guess)
    .scope(Scope::Dev)
    .lifetime(Lifetime::Restart)
    .group(Group::Water);

/// `water.join.snap_max_m`: [`JoinSettings::snap_max_m`].
pub const SNAP_MAX_M: Def<f64> = def!("water.join.snap_max_m" = 20.0_f64)
    .range(0.0, 50.0)
    .unit(Unit::Meters)
    .help("River joins: a tributary's end this close to its receiver's centreline is snapped onto it; farther, it stays where it is")
    .source(Source::Guess)
    .scope(Scope::Dev)
    .lifetime(Lifetime::Restart)
    .group(Group::Water);

/// `water.join.level_tol_m`: [`JoinSettings::level_tol_m`].
pub const LEVEL_TOL_M: Def<f64> = def!("water.join.level_tol_m" = 0.5_f64)
    .range(0.0, 5.0)
    .unit(Unit::Meters)
    .help("River joins: a river point counts as in a lake or the sea only within this height of its surface")
    .source(Source::Guess)
    .scope(Scope::Dev)
    .lifetime(Lifetime::Restart)
    .group(Group::Water);

/// The join tunables (not yet in the crate's [`crate::TUNABLES`]: the wiring slice adds them).
pub const TUNABLES: &[&dyn sf_tunables::AnyDef] = &[&MIX_WIDTHS, &SNAP_MAX_M, &LEVEL_TOL_M];

/// The join rule's settings. Every value is a tunable (`tunables.md`): unit and range in its doc.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct JoinSettings {
    /// `water.join.mix_widths`: length of the fade, in river widths at the start of the overlap,
    /// ratio, range 0.5-8, default 2.5 (Guess); never longer than the overlap itself.
    pub mix_widths: f32,
    /// `water.join.snap_max_m`: largest distance a tributary's end is moved onto its receiver's
    /// centreline, metres, range 0-50, default 20 (Guess: a few river widths; a farther end means
    /// the receiver was clipped).
    pub snap_max_m: f64,
    /// `water.join.level_tol_m`: height difference within which a point is in a lake or the sea,
    /// metres, range 0-5, default 0.5 (Guess: a course running under a lake is not a mouth).
    pub level_tol_m: f64,
}

impl Default for JoinSettings {
    fn default() -> Self {
        Self {
            mix_widths: MIX_WIDTHS.default_value(),
            snap_max_m: SNAP_MAX_M.default_value(),
            level_tol_m: LEVEL_TOL_M.default_value(),
        }
    }
}

/// One river: its spline upstream to downstream, and the rivers its end may join (indices into the
/// same slice: the pieces of the receiver it names; empty = it names none).
#[derive(Clone, Debug, PartialEq)]
pub struct JoinRiver {
    pub spline: Vec<RiverPoint>,
    pub receivers: Vec<usize>,
}

/// One lake: its outline, holes (islands) and surface height.
#[derive(Clone, Debug, PartialEq)]
pub struct JoinLake {
    pub outline: Vec<DVec2>,
    pub holes: Vec<Vec<DVec2>>,
    pub level: f64,
}

/// How a river's downstream end was resolved.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JoinEnd {
    /// It just stops.
    Open,
    /// It ends in lake `i` (at the lake's level).
    Lake(usize),
    /// It joins river `receiver`; its last point moved `moved_m` onto the receiver's centreline.
    Tributary { receiver: usize, moved_m: f64 },
    /// Its nearest receiver is `distance_m` away, beyond the snap distance: left untouched.
    Unsnapped { receiver: usize, distance_m: f64 },
    /// It ends in the sea.
    Sea,
}

/// How a river's upstream end was resolved.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JoinStart {
    /// It just starts.
    Open,
    /// It leaves lake `i` (at the lake's level).
    Lake(usize),
}

/// Both ends of one river.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Join {
    pub start: JoinStart,
    pub end: JoinEnd,
}

/// Nearest point of a centreline to `p`: (distance, point, segment, t); earliest segment on ties.
fn nearest_on(spline: &[RiverPoint], p: DVec2) -> Option<(f64, DVec2, usize, f64)> {
    let mut best: Option<(f64, DVec2, usize, f64)> = None;
    for (seg, w) in spline.windows(2).enumerate() {
        let from = w[0].center;
        let dir = w[1].center - from;
        let t = ((p - from).dot(dir) / dir.length_squared().max(1e-12)).clamp(0.0, 1.0);
        let foot = from + dir * t;
        let dist = (p - foot).length();
        if best.is_none_or(|b| dist < b.0) {
            best = Some((dist, foot, seg, t));
        }
    }
    best
}

/// Half the width of `spline` at segment `seg`, parameter `t` (the ribbon query's formula).
#[allow(clippy::cast_possible_truncation)]
fn half_width_at(spline: &[RiverPoint], seg: usize, t: f64) -> f64 {
    let (a, c) = (spline[seg], spline[seg + 1]);
    f64::from(a.width + (c.width - a.width) * t as f32) * 0.5
}

/// Height of `spline` at segment `seg`, parameter `t`.
fn height_at(spline: &[RiverPoint], seg: usize, t: f64) -> f64 {
    let (a, c) = (spline[seg], spline[seg + 1]);
    a.height + (c.height - a.height) * t
}

/// Whether `p` (at surface height `height`) is in lake `l`.
fn in_lake(l: &JoinLake, p: DVec2, height: f64, tol: f64) -> bool {
    (height - l.level).abs() <= tol && in_ring(&l.outline, p) && !l.holes.iter().any(|h| in_ring(h, p))
}

/// The first lake (list order) that holds point `pt`.
fn lake_of(lakes: &[JoinLake], pt: &RiverPoint, tol: f64) -> Option<usize> {
    lakes.iter().position(|l| in_lake(l, pt.center, pt.height, tol))
}

/// First index of the longest trailing run of `spline` whose points all pass `keep`.
fn trailing_run(spline: &[RiverPoint], keep: impl Fn(&RiverPoint) -> bool) -> usize {
    spline.iter().rposition(|p| !keep(p)).map_or(0, |i| i + 1)
}

/// Last index of the longest leading run of `spline` whose points all pass `keep` (`None`: empty).
fn leading_run(spline: &[RiverPoint], keep: impl Fn(&RiverPoint) -> bool) -> Option<usize> {
    match spline.iter().position(|p| !keep(p)) {
        Some(0) => None,
        Some(i) => Some(i - 1),
        None => spline.len().checked_sub(1),
    }
}

/// `1 - smooth(clamp(arc / z, 0, 1))`, 0 when `z == 0`.
fn fade(arc: f64, z: f64) -> f64 {
    if z <= 0.0 {
        return 0.0;
    }
    let x = (arc / z).clamp(0.0, 1.0);
    1.0 - x * x * (3.0 - 2.0 * x)
}

/// 2D length of the polyline through `pts`.
fn length(pts: &[RiverPoint]) -> f64 {
    pts.windows(2).map(|w| (w[1].center - w[0].center).length()).sum()
}

/// Resolves every river's two ends, in place; one `Join` per river, in order.
///
/// Receivers are read from a snapshot of the input splines, so the result does not depend on the
/// order of the rivers. A river with fewer than 2 points is `Open`/`Open` and untouched; a receiver
/// index equal to the river itself or out of range is ignored.
#[allow(clippy::cast_possible_truncation)] // river_ness and flow are f32 by record layout
pub fn resolve(rivers: &mut [JoinRiver], lakes: &[JoinLake], sea_level: f64, s: &JoinSettings) -> Vec<Join> {
    let input: Vec<Vec<RiverPoint>> = rivers.iter().map(|r| r.spline.clone()).collect();
    let tol = s.level_tol_m;
    let mut joins = Vec::with_capacity(rivers.len());
    for (me, river) in rivers.iter_mut().enumerate() {
        let n = river.spline.len();
        if n < 2 {
            joins.push(Join { start: JoinStart::Open, end: JoinEnd::Open });
            continue;
        }
        let last = river.spline[n - 1];

        // End: lake, receiver, sea, open (first match); `end_run` = first index of the overlap.
        let mut end_run: Option<usize> = None;
        let end = if let Some(i) = lake_of(lakes, &last, tol) {
            end_run = Some(trailing_run(&river.spline, |p| in_lake(&lakes[i], p.center, p.height, tol)));
            JoinEnd::Lake(i)
        } else if let Some((d, foot, receiver, seg, t)) = river
            .receivers
            .iter()
            .copied()
            .filter(|&r| r != me && r < input.len())
            .filter_map(|r| nearest_on(&input[r], last.center).map(|(d, q, seg, t)| (d, q, r, seg, t)))
            .min_by(|a, b| a.0.total_cmp(&b.0).then(a.2.cmp(&b.2)).then(a.3.cmp(&b.3)))
        {
            if d <= s.snap_max_m {
                let rec = &input[receiver];
                let tip = &mut river.spline[n - 1];
                tip.center = foot;
                tip.height = tip.height.min(height_at(rec, seg, t));
                let before = &river.spline[..n - 1];
                let k = trailing_run(before, |p| {
                    nearest_on(rec, p.center).is_some_and(|(pd, _, ps, pt)| pd <= half_width_at(rec, ps, pt))
                });
                end_run = Some(k);
                JoinEnd::Tributary { receiver, moved_m: d }
            } else {
                JoinEnd::Unsnapped { receiver, distance_m: d }
            }
        } else if last.height <= sea_level + tol {
            end_run = Some(trailing_run(&river.spline, |p| p.height <= sea_level + tol));
            JoinEnd::Sea
        } else {
            JoinEnd::Open
        };

        // Start: lake or open; `start_run` = last index of the overlap.
        let mut start_run: Option<usize> = None;
        let start = if let Some(i) = lake_of(lakes, &river.spline[0], tol) {
            start_run = leading_run(&river.spline, |p| in_lake(&lakes[i], p.center, p.height, tol));
            JoinStart::Lake(i)
        } else {
            JoinStart::Open
        };

        // river_ness and flow: the end fade over points k..=n-1, the start fade over 0..=m.
        let mut factor = vec![1.0_f64; n];
        if let Some(k) = end_run {
            let run = &river.spline[k..];
            let z = f64::from(s.mix_widths * run[0].width).min(length(run));
            let mut arc = 0.0;
            for (j, f) in factor[k..].iter_mut().enumerate() {
                if j > 0 {
                    arc += (run[j].center - run[j - 1].center).length();
                }
                *f *= fade(arc, z);
            }
        }
        if let Some(m) = start_run {
            let run = &river.spline[..=m];
            let z = f64::from(s.mix_widths * run[m].width).min(length(run));
            let mut arc = 0.0;
            for (j, f) in factor[..=m].iter_mut().enumerate().rev() {
                if j < m {
                    arc += (run[j + 1].center - run[j].center).length();
                }
                *f *= fade(arc, z);
            }
        }
        for (p, f) in river.spline.iter_mut().zip(factor) {
            p.river_ness *= f as f32;
            p.flow_speed *= f as f32;
        }
        joins.push(Join { start, end });
    }
    joins
}
