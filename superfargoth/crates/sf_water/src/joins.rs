//! River joins (water-bodies.md "FL-4: joins"): each river's two ends are resolved against the
//! rivers it names as receivers, the lakes and the sea, and every river point gets a
//! [`RiverPoint::river_ness`] (1 = the river's own water, 0 = the receiver's water shows). Pure and
//! Bevy-free; a reader calls [`resolve`] once on a worldspace's rivers before building their bodies.
//!
//! The fade lies only over the overlap of the two bodies, where the receiver's water already is
//! (industry: the UE Water plugin's river-to-lake and river-to-ocean transitions blend over the
//! overlap), so a river never thins over its own dry bed. A tributary's last point is snapped onto
//! the receiver's centreline so the two ribbons meet. In a lake the fade follows the course, not
//! the record: where one river ends at the lake's edge and another starts at that point (a
//! generator's separate piece through the lake), the fade-out runs on along the second, and the
//! fade-in toward the lake's outlet starts on the piece before the river that leaves it.

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

/// The arc length of each point of `pts` from the first.
fn arcs(pts: &[RiverPoint]) -> Vec<f64> {
    let mut arc = 0.0;
    let mut out = Vec::with_capacity(pts.len());
    for (j, p) in pts.iter().enumerate() {
        if j > 0 {
            arc += (p.center - pts[j - 1].center).length();
        }
        out.push(arc);
    }
    out
}

/// Resolves every river's two ends, in place; one `Join` per river, in order.
///
/// Receivers are read from a snapshot of the input splines, so the result does not depend on the
/// order of the rivers. A river with fewer than 2 points is `Open`/`Open` and untouched; a receiver
/// index equal to the river itself or out of range is ignored. A river in a lake with neither a
/// way in nor a way out along its course (a stray piece) gets `river_ness` 0 there.
#[allow(clippy::cast_possible_truncation)] // river_ness and flow are f32 by record layout
pub fn resolve(rivers: &mut [JoinRiver], lakes: &[JoinLake], sea_level: f64, s: &JoinSettings) -> Vec<Join> {
    let input: Vec<Vec<RiverPoint>> = rivers.iter().map(|r| r.spline.clone()).collect();
    let tol = s.level_tol_m;
    let mut joins = Vec::with_capacity(rivers.len());
    // Per river: the first index of its end's overlap run, the last index of its start's.
    let mut runs: Vec<(Option<usize>, Option<usize>)> = Vec::with_capacity(rivers.len());
    for (me, river) in rivers.iter_mut().enumerate() {
        let n = river.spline.len();
        if n < 2 {
            runs.push((None, None));
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

        runs.push((end_run, start_run));
        joins.push(Join { start, end });
    }

    // river_ness and flow, once every end is known.
    let factors = fades(rivers, &joins, &runs, s);
    for (river, factor) in rivers.iter_mut().zip(factors) {
        for (p, f) in river.spline.iter_mut().zip(factor) {
            p.river_ness *= f as f32;
            p.flow_speed *= f as f32;
        }
    }
    joins
}

/// The factor on each point's `river_ness` and flow, per river, once every end is known (`runs`: the
/// first index of each end's overlap run, the last index of each start's). A lake's fades run across
/// the seam between two rivers of one course (`next`).
fn fades(rivers: &[JoinRiver], joins: &[Join], runs: &[(Option<usize>, Option<usize>)], s: &JoinSettings) -> Vec<Vec<f64>> {
    let count = rivers.len();
    let cum: Vec<Vec<f64>> = rivers.iter().map(|r| arcs(&r.spline)).collect();
    // The end's run in its lake (first index, length) and the start's run in its lake (last index, length).
    let trail = |r: usize| match (joins[r].end, runs[r].0) {
        (JoinEnd::Lake(_), Some(k)) => Some((k, cum[r][cum[r].len() - 1] - cum[r][k])),
        _ => None,
    };
    let lead = |r: usize| match (joins[r].start, runs[r].1) {
        (JoinStart::Lake(_), Some(m)) => Some((m, cum[r][m])),
        _ => None,
    };
    // The river that carries river `r`'s course on through the lake it ends in: the nearest one starting
    // in that lake within `r`'s half width of `r`'s last point (lowest index on ties), with the distance.
    let next: Vec<Option<(usize, f64)>> = (0..count)
        .map(|r| {
            let JoinEnd::Lake(i) = joins[r].end else { return None };
            let last = rivers[r].spline.last()?;
            (0..count)
                .filter(|&o| o != r && joins[o].start == JoinStart::Lake(i))
                .map(|o| (o, (rivers[o].spline[0].center - last.center).length()))
                .filter(|&(_, d)| d <= f64::from(last.width) * 0.5)
                .min_by(|a, b| a.1.total_cmp(&b.1).then(a.0.cmp(&b.0)))
        })
        .collect();
    let zone = |river: usize, at: usize, room: f64| f64::from(s.mix_widths * rivers[river].spline[at].width).min(room);
    (0..count)
        .map(|r| {
            let (c, n) = (&cum[r], rivers[r].spline.len());
            let mut factor = vec![1.0_f64; n];
            // A tributary's or the sea's end fade over points k..=n-1.
            if let (Some(k), JoinEnd::Tributary { .. } | JoinEnd::Sea) = (runs[r].0, joins[r].end) {
                let z = zone(r, k, c[n - 1] - c[k]);
                for (f, arc) in factor[k..].iter_mut().zip(&c[k..]) {
                    *f *= fade(arc - c[k], z);
                }
            }
            // In a lake: the larger of the fade-out from where the course enters the lake and the fade-in
            // to where it leaves it, each on this river or carried over the seam; neither = the lake's water.
            let mut lake: Vec<Option<f64>> = vec![None; n];
            let mut raise = |j: usize, v: f64| lake[j] = Some(lake[j].map_or(v, |l| l.max(v)));
            if let Some((k, len)) = trail(r) {
                let after = next[r].and_then(|(o, _)| lead(o).map(|(m, lead_len)| (o, m, lead_len)));
                for j in k..n {
                    raise(j, 0.0);
                    if k > 0 {
                        raise(j, fade(c[j] - c[k], zone(r, k, len + after.map_or(0.0, |a| a.2))));
                    }
                    if let Some((o, m, lead_len)) = after
                        && m + 1 < rivers[o].spline.len()
                    {
                        raise(j, fade(lead_len + c[n - 1] - c[j], zone(o, m, lead_len + len)));
                    }
                }
            }
            if let Some((m, len)) = lead(r) {
                // The rivers whose course runs on into this one, nearest first.
                let mut before: Vec<(f64, usize)> = (0..count).filter_map(|p| next[p].filter(|&(o, _)| o == r).map(|(_, d)| (d, p))).collect();
                before.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
                let before: Vec<(usize, usize, f64)> = before.iter().filter_map(|&(_, p)| trail(p).map(|(k, trail_len)| (p, k, trail_len))).collect();
                for j in 0..=m {
                    raise(j, 0.0);
                    if m + 1 < n {
                        raise(j, fade(c[m] - c[j], zone(r, m, len + before.first().map_or(0.0, |b| b.2))));
                    }
                    for &(p, k, trail_len) in before.iter().filter(|b| b.1 > 0) {
                        raise(j, fade(trail_len + c[j], zone(p, k, trail_len + len)));
                    }
                }
            }
            for (f, l) in factor.iter_mut().zip(lake) {
                *f *= l.unwrap_or(1.0);
            }
            factor
        })
        .collect()
}
