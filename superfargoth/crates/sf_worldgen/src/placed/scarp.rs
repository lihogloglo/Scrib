//! `fault_scarp` + `fan` (talus) operators (LF-2, `landforms.md` "Operator catalog"): traces from the unit map, the
//! en-echelon segment geometry with relay ramps, and the pre-filtered cross-section profile.
//!
//! **Profile** (signed distance `d` to the lip, + on the upthrown side, throw `H`): a tilted fault block. Top: `H - d
//! tan(back)` back to 0 at `d_b = H / tan(back)` (the footwall dips gently away from the lip, so the lip is the
//! highest edge); face: `H + d tan(face)` from the lip down to the talus top `d_t = -(1 - fan) H / tan(face)`; talus
//! (the `fan` operator): `fan H + (d - d_t) tan(talus)` down to the plain at `d_0 = d_t - fan H / tan(talus)`; plain:
//! 0. Continuous, piecewise linear; the delta is box-filtered across the strike over `2 r` (`r` = prefilter samples x
//! spacing) through its exact antiderivative, so the bicubic guide sampling never rings at the lip or the foot.
//!
//! **Along strike** each segment's throw is `H_max s(t) s(L - t) g(t)`: `s` a smoothstep over the segment's taper
//! length (tapered tips, Cowie & Scholz 1992: displacement goes to zero at the tips), `g` the water-gap factor (1
//! away from large channels). Segments of one fault overlap by the relay length and step sideways by `overlap /
//! aspect`, so the ground between two overlapping faces is a relay ramp that leads from the hanging wall to the
//! footwall (Peacock & Sanderson 1991; Soliva & Benedicto 2004 for the overlap / separation ratio). Within a fault
//! the segment deltas add (displacement is shared); across faults the larger delta wins.
//!
//! **LF-2b** (eroded faces, [`super::facets`]): a segment with chutes is dissected into triangular facets between
//! ravines with a debris cone at every mouth, its throw modulated along strike ([`Wobble`]), its tips tapered over
//! their own lengths (`taper[0]`, `taper[1]`) and its face angle its own (`tan_face`, from the upthrown unit's
//! hardness). A segment without chutes is the LF-2 profile above, unchanged.

use std::collections::VecDeque;

use super::facets::{self, Chute, Wobble};

/// Cross-section of one face at one point along strike.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Profile {
    /// Throw, metres (>= 0).
    pub h: f64,
    pub tan_face: f64,
    pub tan_talus: f64,
    /// Share of the throw buried under the talus, [0, 1).
    pub fan: f64,
    pub tan_back: f64,
}

/// Where a point lies in a profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Zone {
    Plain = 0,
    Talus = 1,
    Face = 2,
    Top = 3,
}

impl Profile {
    /// Breakpoints `[d_0, d_t, 0, d_b]` (plain / talus / face / top / plain).
    pub fn knots(&self) -> [f64; 4] {
        let d_t = -(1.0 - self.fan) * self.h / self.tan_face;
        let d_0 = d_t - self.fan * self.h / self.tan_talus;
        [d_0, d_t, 0.0, self.h / self.tan_back]
    }

    /// Height of the unfiltered profile at `d`.
    pub fn z(&self, d: f64) -> f64 {
        if self.h <= 0.0 {
            return 0.0;
        }
        let [d0, dt, _, db] = self.knots();
        if d < d0 || d >= db {
            0.0
        } else if d < dt {
            self.fan * self.h + (d - dt) * self.tan_talus
        } else if d < 0.0 {
            self.h + d * self.tan_face
        } else {
            self.h - d * self.tan_back
        }
    }

    /// `int_{-inf}^{d} z`, exact (piecewise quadratic).
    pub fn integral(&self, d: f64) -> f64 {
        if self.h <= 0.0 {
            return 0.0;
        }
        let k = self.knots();
        // Pieces between knots: value at the piece start and slope.
        let starts = [0.0, self.fan * self.h, self.h, 0.0];
        let pieces = [
            (k[0], k[1], starts[0], self.tan_talus),
            (k[1], k[2], starts[1], self.tan_face),
            (k[2], k[3], starts[2], -self.tan_back),
        ];
        let mut acc = 0.0;
        for (p, q, zp, b) in pieces {
            if d <= p {
                break;
            }
            let len = d.min(q) - p;
            acc += zp * len + 0.5 * b * len * len;
        }
        acc
    }

    /// The profile box-filtered over `[d - r, d + r]` (`r = 0`: the profile itself).
    pub fn filtered(&self, d: f64, r: f64) -> f64 {
        if r <= 0.0 {
            return self.z(d);
        }
        (self.integral(d + r) - self.integral(d - r)) / (2.0 * r)
    }

    /// Zone of `d` (filter footprint ignored).
    pub fn zone(&self, d: f64) -> Zone {
        let [d0, dt, _, db] = self.knots();
        if d < d0 || d >= db {
            Zone::Plain
        } else if d < dt {
            Zone::Talus
        } else if d < 0.0 {
            Zone::Face
        } else {
            Zone::Top
        }
    }
}

/// `smoothstep(0, e, x)`; 1 for `e <= 0`.
#[inline]
pub fn ramp01(x: f64, e: f64) -> f64 {
    if e <= 0.0 {
        return 1.0;
    }
    let t = (x / e).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// One straight-ish face of a fault: a polyline (metres, local frame `x = col s`, `y = row s`), its throw along it.
#[derive(Debug, Clone, PartialEq)]
pub struct Segment {
    pub pts: Vec<[f64; 2]>,
    /// Cumulative arc length per vertex.
    pub cum: Vec<f64>,
    /// +1: the upthrown side is the left normal `(-dy, dx)` of the polyline direction; -1: the right one.
    pub side: f64,
    pub h_max: f64,
    /// Taper length at the start and at the end, metres.
    pub taper: [f64; 2],
    /// Water-gap factor per vertex, [0, 1].
    pub gap: Vec<f64>,
    /// Along-strike throw modulation (LF-2b; default off).
    pub wobble: Wobble,
    /// `tan` of this face's angle (`None`: the shape's).
    pub tan_face: Option<f64>,
    /// Ravines of the dissected face, sorted by arc length (LF-2b; empty = the plain LF-2 profile, filtered
    /// analytically).
    pub chutes: Vec<Chute>,
    /// Along-strike reach of the chutes and their cones, metres ([`facets::chute_reach`]).
    pub chute_reach: f64,
}

impl Segment {
    /// Builds a segment (computes the arc lengths; `gap` defaults to 1).
    pub fn new(pts: Vec<[f64; 2]>, side: f64, h_max: f64, taper_m: f64) -> Self {
        let mut cum = Vec::with_capacity(pts.len());
        let mut acc = 0.0;
        for (i, p) in pts.iter().enumerate() {
            if i > 0 {
                acc += (p[0] - pts[i - 1][0]).hypot(p[1] - pts[i - 1][1]);
            }
            cum.push(acc);
        }
        let gap = vec![1.0; pts.len()];
        Self {
            pts,
            cum,
            side,
            h_max,
            taper: [taper_m; 2],
            gap,
            wobble: Wobble::default(),
            tan_face: None,
            chutes: Vec::new(),
            chute_reach: 0.0,
        }
    }

    /// Largest throw the wobble allows, metres.
    pub fn max_throw(&self) -> f64 {
        self.h_max * (1.0 + self.wobble.amp.max(0.0))
    }

    /// The cross-section at throw `h` with this face's angle.
    pub fn profile(&self, shape: &Shape, h: f64) -> Profile {
        Profile {
            tan_face: self.tan_face.unwrap_or(shape.tan_face),
            ..shape.profile(h)
        }
    }

    /// Half-width of the footprint across the strike, metres (cones included).
    pub fn reach(&self, shape: &Shape) -> f64 {
        let h = self.max_throw();
        let pr = self.profile(shape, h);
        let k = pr.knots();
        let mut out = k[3].max(-k[0]);
        if let Some(apex) = self.chutes.iter().map(|c| c.apex).reduce(f64::max) {
            out = out.max(h / pr.tan_face + apex * h / shape.tan_cone);
        }
        out + shape.filter_r
    }

    pub fn len(&self) -> f64 {
        self.cum.last().copied().unwrap_or(0.0)
    }

    /// Throw at arc length `s`.
    pub fn throw_at(&self, s: f64) -> f64 {
        let l = self.len();
        if !(0.0..=l).contains(&s) || l <= 0.0 {
            return 0.0;
        }
        let k = self.cum.partition_point(|&c| c <= s).clamp(1, self.cum.len() - 1);
        let (c0, c1) = (self.cum[k - 1], self.cum[k]);
        let t = if c1 > c0 { (s - c0) / (c1 - c0) } else { 0.0 };
        let g = self.gap[k - 1] + (self.gap[k] - self.gap[k - 1]) * t;
        self.h_max * ramp01(s, self.taper[0]) * ramp01(l - s, self.taper[1]) * g * self.wobble.factor(s)
    }

    /// `(arc length, signed distance + upthrown)` of the nearest point of the polyline to `p`.
    pub fn locate(&self, p: [f64; 2]) -> (f64, f64) {
        let mut best = (f64::INFINITY, 0.0, 0.0);
        for k in 1..self.pts.len() {
            let (a, b) = (self.pts[k - 1], self.pts[k]);
            let (ex, ey) = (b[0] - a[0], b[1] - a[1]);
            let l2 = ex * ex + ey * ey;
            if l2 <= 0.0 {
                continue;
            }
            let t = (((p[0] - a[0]) * ex + (p[1] - a[1]) * ey) / l2).clamp(0.0, 1.0);
            let (qx, qy) = (a[0] + ex * t, a[1] + ey * t);
            let dist2 = (p[0] - qx).powi(2) + (p[1] - qy).powi(2);
            if dist2 < best.0 {
                let l = l2.sqrt();
                // Left normal (-ey, ex): signed perpendicular distance.
                let perp = (-(p[0] - a[0]) * ey + (p[1] - a[1]) * ex) / l;
                best = (dist2, self.cum[k - 1] + t * l, perp);
            }
        }
        (best.1, self.side * best.2)
    }

    /// Axis-aligned bounds of the vertices.
    pub fn bounds(&self) -> [f64; 4] {
        let mut b = [f64::INFINITY, f64::INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY];
        for p in &self.pts {
            b[0] = b[0].min(p[0]);
            b[1] = b[1].min(p[1]);
            b[2] = b[2].max(p[0]);
            b[3] = b[3].max(p[1]);
        }
        b
    }
}

/// Shape values shared by every face.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Shape {
    pub tan_face: f64,
    pub tan_talus: f64,
    pub fan: f64,
    pub tan_back: f64,
    /// Box-filter half-width across the strike, metres.
    pub filter_r: f64,
    /// Debris-cone slope at chute mouths (LF-2b).
    pub tan_cone: f64,
    /// Chute wall slope (LF-2b).
    pub tan_wall: f64,
    /// Box-filter taps per axis of an eroded face (LF-2b).
    pub taps: usize,
}

impl Shape {
    pub fn profile(&self, h: f64) -> Profile {
        Profile {
            h,
            tan_face: self.tan_face,
            tan_talus: self.tan_talus,
            fan: self.fan,
            tan_back: self.tan_back,
        }
    }

    /// Half-width of a face's footprint across the strike at throw `h`, metres.
    pub fn reach(&self, h: f64) -> f64 {
        let k = self.profile(h).knots();
        k[3].max(-k[0]) + self.filter_r
    }
}

/// One sample's value from one fault: the summed delta and the dominant segment's zone and signed distance.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hit {
    pub delta: f64,
    pub zone: Zone,
    pub d: f64,
}

/// Evaluates a fault (its segments, deltas summed) at `p`.
pub fn eval_fault(segs: &[Segment], shape: &Shape, p: [f64; 2]) -> Option<Hit> {
    let mut sum = 0.0;
    let mut best: Option<(f64, Zone, f64)> = None;
    for s in segs {
        let (arc, d) = s.locate(p);
        let (v, zone) = if s.chutes.is_empty() {
            let h = s.throw_at(arc);
            if h <= 0.0 {
                continue;
            }
            let pr = s.profile(shape, h);
            (pr.filtered(d, shape.filter_r), pr.zone(d))
        } else {
            facets::filtered(s, shape, arc, d)
        };
        if v <= 0.0 {
            continue;
        }
        sum += v;
        if best.is_none_or(|b| v > b.0) {
            best = Some((v, zone, d));
        }
    }
    best.map(|(_, zone, d)| Hit { delta: sum, zone, d })
}

/// Splits a fault trace (arc range `[s0, s0 + len]` of `trace`) into en-echelon segments: `n = round(len /
/// segment_m)` (>= 1) pieces overlapping by `overlap` of a piece, alternately shifted `+-sep / 2` across the strike,
/// each tapering over the overlap length at both ends (at least `min_taper_m`).
#[allow(clippy::too_many_arguments)]
pub fn segments(
    trace: &[[f64; 2]],
    s0: f64,
    len: f64,
    side: f64,
    h_max: f64,
    segment_m: f64,
    overlap: f64,
    aspect: f64,
    min_taper_m: f64,
) -> Vec<Segment> {
    let base = Segment::new(trace.to_vec(), side, 0.0, 0.0);
    let n = (len / segment_m).round().max(1.0) as usize;
    let nf = n as f64;
    let ls = len / (nf - (nf - 1.0) * overlap);
    let ov = overlap * ls;
    let sep = if aspect > 0.0 { ov / aspect } else { 0.0 };
    let step_s = (ls - ov).max(1.0);
    let mut out = Vec::with_capacity(n);
    for k in 0..n {
        let a = s0 + k as f64 * step_s;
        let b = (a + ls).min(s0 + len);
        let shift = if n == 1 {
            0.0
        } else if k % 2 == 0 {
            -0.5 * sep
        } else {
            0.5 * sep
        };
        let pts = sub_polyline(&base, a, b, shift);
        if pts.len() < 2 {
            continue;
        }
        let taper = ov.max(min_taper_m).min(0.5 * (b - a));
        out.push(Segment::new(pts, side, h_max, taper));
    }
    out
}

/// Vertices of `base` between arc lengths `a` and `b` (end points interpolated), shifted by `shift` metres along the
/// left normal times `base.side` (toward the upthrown side for `shift > 0`).
fn sub_polyline(base: &Segment, a: f64, b: f64, shift: f64) -> Vec<[f64; 2]> {
    let at = |s: f64| -> ([f64; 2], [f64; 2]) {
        let k = base.cum.partition_point(|&c| c <= s).clamp(1, base.cum.len() - 1);
        let (p, q) = (base.pts[k - 1], base.pts[k]);
        let (c0, c1) = (base.cum[k - 1], base.cum[k]);
        let t = if c1 > c0 { ((s - c0) / (c1 - c0)).clamp(0.0, 1.0) } else { 0.0 };
        let l = (q[0] - p[0]).hypot(q[1] - p[1]).max(1e-9);
        (
            [p[0] + (q[0] - p[0]) * t, p[1] + (q[1] - p[1]) * t],
            [-(q[1] - p[1]) / l, (q[0] - p[0]) / l],
        )
    };
    let mut ss = vec![a];
    ss.extend(base.cum.iter().copied().filter(|&c| c > a && c < b));
    ss.push(b);
    ss.iter()
        .map(|&s| {
            let (p, nrm) = at(s);
            [p[0] + nrm[0] * shift * base.side, p[1] + nrm[1] * shift * base.side]
        })
        .collect()
}

/// A unit-boundary trace: the harder unit `a` (the side its pixels lie on), the softer unit `b` most often across
/// it, and its polyline in sample coordinates (col, row).
#[derive(Debug, Clone, PartialEq)]
pub struct Trace {
    pub a: u8,
    pub b: u8,
    pub pts: Vec<[f64; 2]>,
}

/// Traces of the unit map: the outline of the harder rock (a fault-line scarp faces the softer side). Boundary
/// pixels are dry samples of unit `a` with a 4-neighbour of a softer unit (order: `hardness`, then the lower index is
/// harder), 8-connected components per unit `a` (whatever softer unit lies across), each reduced to its longest path
/// (two BFS), moving-averaged over `smooth` samples and resampled every `step` samples; `b` is the softer unit met
/// most often. Components shorter than `min_len` samples are dropped. `blocked` samples (sea, coast guard, anchor
/// bands) break traces.
#[allow(clippy::too_many_lines, clippy::too_many_arguments)]
pub fn traces(
    unit: &[u8],
    hardness: &[f64],
    blocked: &[bool],
    w: usize,
    h: usize,
    smooth: f64,
    step: f64,
    min_len: f64,
) -> Vec<Trace> {
    // tunables: const (sentinel: "not a boundary pixel")
    const NONE: u16 = u16::MAX;
    let n = w * h;
    let harder = |a: u8, b: u8| -> bool {
        match hardness[usize::from(a)].total_cmp(&hardness[usize::from(b)]) {
            std::cmp::Ordering::Greater => true,
            std::cmp::Ordering::Less => false,
            std::cmp::Ordering::Equal => a < b,
        }
    };
    let mut key = vec![NONE; n];
    let mut across = vec![0u8; n];
    for y in 0..h {
        for x in 0..w {
            let i = y * w + x;
            if blocked[i] {
                continue;
            }
            let u = unit[i];
            let mut best: Option<u8> = None;
            let mut look = |j: usize| {
                if !blocked[j] && harder(u, unit[j]) && best.is_none_or(|b| harder(b, unit[j])) {
                    best = Some(unit[j]);
                }
            };
            if x + 1 < w {
                look(i + 1);
            }
            if x > 0 {
                look(i - 1);
            }
            if y + 1 < h {
                look(i + w);
            }
            if y > 0 {
                look(i - w);
            }
            if let Some(b) = best {
                key[i] = u16::from(u);
                across[i] = b;
            }
        }
    }
    let mut comp = vec![u32::MAX; n];
    let mut dist = vec![u32::MAX; n];
    let mut parent = vec![u32::MAX; n];
    let mut out = Vec::new();
    let mut queue = VecDeque::new();
    let neighbours8 = |i: usize| {
        let (x, y) = ((i % w) as isize, (i / w) as isize);
        crate::grid::NEIGHBOURS.iter().filter_map(move |&(dx, dy)| {
            let (nx, ny) = (x + dx, y + dy);
            (nx >= 0 && ny >= 0 && (nx as usize) < w && (ny as usize) < h).then(|| ny as usize * w + nx as usize)
        })
    };
    let mut members: Vec<usize> = Vec::new();
    let mut ncomp = 0u32;
    for start in 0..n {
        if key[start] == NONE || comp[start] != u32::MAX {
            continue;
        }
        let k = key[start];
        // Component flood (records members).
        members.clear();
        comp[start] = ncomp;
        queue.push_back(start);
        while let Some(c) = queue.pop_front() {
            members.push(c);
            for j in neighbours8(c) {
                if key[j] == k && comp[j] == u32::MAX {
                    comp[j] = ncomp;
                    queue.push_back(j);
                }
            }
        }
        ncomp += 1;
        if (members.len() as f64) < min_len {
            continue;
        }
        // Two BFS: farthest from start, then farthest from that with parents.
        let bfs = |from: usize, dist: &mut [u32], parent: &mut [u32], queue: &mut VecDeque<usize>| -> usize {
            for &m in &members {
                dist[m] = u32::MAX;
                parent[m] = u32::MAX;
            }
            dist[from] = 0;
            queue.push_back(from);
            let mut far = from;
            while let Some(c) = queue.pop_front() {
                if dist[c] > dist[far] || (dist[c] == dist[far] && c < far) {
                    far = c;
                }
                for j in neighbours8(c) {
                    if key[j] == k && dist[j] == u32::MAX {
                        dist[j] = dist[c] + 1;
                        parent[j] = c as u32;
                        queue.push_back(j);
                    }
                }
            }
            far
        };
        let a = bfs(start, &mut dist, &mut parent, &mut queue);
        let b = bfs(a, &mut dist, &mut parent, &mut queue);
        let mut path = Vec::new();
        let mut c = b;
        loop {
            path.push([(c % w) as f64, (c / w) as f64]);
            if parent[c] == u32::MAX {
                break;
            }
            c = parent[c] as usize;
        }
        let pts = resample(&moving_average(&path, smooth), step);
        if pts.len() >= 2 && arc_len(&pts) >= min_len {
            let mut count = [0usize; 256];
            for &m in &members {
                count[usize::from(across[m])] += 1;
            }
            let b = (0..256).max_by_key(|&v| (count[v], std::cmp::Reverse(v))).unwrap_or(0) as u8;
            out.push(Trace { a: k as u8, b, pts });
        }
    }
    out
}

/// Polyline length.
pub fn arc_len(p: &[[f64; 2]]) -> f64 {
    p.windows(2).map(|w| (w[1][0] - w[0][0]).hypot(w[1][1] - w[0][1])).sum()
}

/// Moving average over a window of `win` points (centred, clamped at the ends; end points kept).
fn moving_average(p: &[[f64; 2]], win: f64) -> Vec<[f64; 2]> {
    let half = (win * 0.5).floor().max(0.0) as usize;
    if half == 0 || p.len() < 3 {
        return p.to_vec();
    }
    let n = p.len();
    (0..n)
        .map(|i| {
            if i == 0 || i + 1 == n {
                return p[i];
            }
            let r = half.min(i).min(n - 1 - i);
            let (mut sx, mut sy) = (0.0, 0.0);
            for q in &p[i - r..=i + r] {
                sx += q[0];
                sy += q[1];
            }
            let c = (2 * r + 1) as f64;
            [sx / c, sy / c]
        })
        .collect()
}

/// Uniform resampling every `step` along the polyline (last point kept).
fn resample(p: &[[f64; 2]], step: f64) -> Vec<[f64; 2]> {
    if p.len() < 2 || step <= 0.0 {
        return p.to_vec();
    }
    let mut out = vec![p[0]];
    let mut carry = 0.0;
    for w in p.windows(2) {
        let (a, b) = (w[0], w[1]);
        let l = (b[0] - a[0]).hypot(b[1] - a[1]);
        let mut s = step - carry;
        while s <= l {
            let t = s / l;
            out.push([a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t]);
            s += step;
        }
        carry = l - (s - step);
    }
    let last = p[p.len() - 1];
    if out.last().is_none_or(|q| (q[0] - last[0]).hypot(q[1] - last[1]) > 1e-6) {
        out.push(last);
    }
    out
}
