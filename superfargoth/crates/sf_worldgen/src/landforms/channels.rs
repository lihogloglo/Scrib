//! **Channels and trenches** (rivers, foyadas, gorges): the drainage network and the valley it runs in.
//!
//! - Network: D8 steepest descent over the Priority-Flood+ε surface, wet ribbons narrower than `neck_width_m`
//!   (rivers that are water) routed as ground so their valleys are measured too ([`crate::stream::route`]; O'Callaghan &
//!   Mark 1984), channels where the drainage area reaches `channel_area_m2` (Tarboton, Bras, Rodriguez-Iturbe,
//!   "On the extraction of channel networks from digital elevation data", Hydrological Processes 5, 1991).
//! - Stems: from every outlet upstream, always into the donor with the largest area (the main stem, Hack's
//!   ordering); the other donors start their own stems. Consecutive links of one main stem are one stem.
//! - Valley cross-section at every stem sample, perpendicular to the stem direction (over ±`valley_smooth_m / 2`),
//!   out to `valley_halfwidth_m` each side: **valley depth** `D = min(max left, max right) − h_channel` (the lower
//!   wall, so a channel along one cliff is not a valley: the left/right half-window maxima form of a cross-section
//!   valley depth; SAGA's "Valley Depth" interpolates a ridge-level surface instead, and Gallant & Dowling's `MrVBF`,
//!   "A multiresolution index of valley bottom flatness", Water Resources Research 39(12), 2003, maps flat valley
//!   floors by relative height in windows, the same idea for areas), the widths at 25, 50 and 75 % of `D`, the mean wall slope between 25 and 75 % of `D`, and the
//!   **floor ratio** `w25 / w75` (1/3 for a V, near 1 for a flat floor between steep walls).
//! - Runs: the depth smoothed along the stem (moving mean over `valley_smooth_m`); a run is a stretch where it is at
//!   least `valley_min_depth_m` (gaps up to `valley_smooth_m` bridged), kept when at least `valley_min_length_m`
//!   long. The channel level is the filled (spill) height, so pits in a lumpy floor (lava channels are full of
//!   them) neither deepen the valley nor break the run.
//! - Classes (medians over the run): `trench` (floor ratio ≥ `trench_min_floor_ratio`: flat floor, the lava
//!   channel form), else `gorge` (wall slope ≥ `gorge_min_slope_deg`), else `valley`.

use crate::stream::route;

use super::geom::{bilinear, simplify};
use super::{Feature, Field, Params, feature, score};

/// Valley cross-section at one sample.
#[derive(Clone, Copy, Default)]
struct Section {
    depth: f64,
    w25: f64,
    w50: f64,
    w75: f64,
    slope_deg: f64,
    valid: bool,
}

/// Channel runs of `f` (`fill_depth`: per-sample depth below the spill height, from the depression detector;
/// `route_wet`: base-level water, the wet mask without narrow sea ribbons, so a river that is water is routed
/// along its bed).
pub fn detect(f: &Field, p: &Params, fill_depth: &[f64], route_wet: &[bool]) -> Vec<Feature> {
    let (w, h, s) = (f.g.w, f.g.h, f.g.spacing);
    let n = w * h;
    let mut z = Vec::new();
    let r = route(&f.g, route_wet, f.sea, &mut z);
    let chan: Vec<bool> = (0..n)
        .map(|i| !route_wet[i] && r.area[i] >= p.channel_area_m2)
        .collect();
    // Channel donors (CSR).
    let mut start = vec![0u32; n + 1];
    for i in 0..n {
        let rc = r.receiver[i] as usize;
        if chan[i] && rc != i && chan[rc] {
            start[rc + 1] += 1;
        }
    }
    for i in 0..n {
        start[i + 1] += start[i];
    }
    let mut fill = start.clone();
    let mut donors = vec![0u32; start[n] as usize];
    for i in 0..n {
        let rc = r.receiver[i] as usize;
        if chan[i] && rc != i && chan[rc] {
            donors[fill[rc] as usize] = i as u32;
            fill[rc] += 1;
        }
    }
    // Stems, outlets in index order, tributaries queued in the order met.
    let mut queue: std::collections::VecDeque<usize> = (0..n)
        .filter(|&i| {
            let rc = r.receiver[i] as usize;
            chan[i] && (rc == i || !chan[rc])
        })
        .collect();
    let mut out = Vec::new();
    while let Some(s0) = queue.pop_front() {
        let mut stem = vec![s0];
        let mut cur = s0;
        loop {
            let ds = &donors[start[cur] as usize..start[cur + 1] as usize];
            let Some(&main) = ds.iter().max_by(|&&a, &&b| {
                r.area[a as usize]
                    .total_cmp(&r.area[b as usize])
                    .then(b.cmp(&a))
            }) else {
                break;
            };
            for &d in ds {
                if d != main {
                    queue.push_back(d as usize);
                }
            }
            stem.push(main as usize);
            cur = main as usize;
        }
        stem.reverse(); // source first
        if (stem.len() as f64) * s < p.valley_min_length_m {
            continue;
        }
        runs(f, p, &stem, &r.area, fill_depth, &mut out);
    }
    out
}

fn section(
    f: &Field,
    p: &Params,
    stem: &[usize],
    fill_depth: &[f64],
    k: usize,
    i: usize,
) -> Section {
    let w = f.g.w;
    let pos = |j: usize| ((stem[j] % w) as f64, (stem[j] / w) as f64);
    let (a, b) = (pos(i.saturating_sub(k)), pos((i + k).min(stem.len() - 1)));
    let (tx, ty) = (b.0 - a.0, b.1 - a.1);
    let tl = tx.hypot(ty);
    if tl == 0.0 {
        return Section::default();
    }
    let (nx, ny) = (-ty / tl, tx / tl);
    let c = pos(i);
    // The channel level is the water level: pits in a lumpy floor are filled to their spill height.
    let hc = f.g.data[stem[i]] + fill_depth[stem[i]];
    let steps = (p.valley_halfwidth_m / f.g.spacing).ceil().max(2.0) as usize;
    let mut sides = [Vec::with_capacity(steps + 1), Vec::with_capacity(steps + 1)];
    for (si, sign) in [-1.0, 1.0].into_iter().enumerate() {
        sides[si].push(hc);
        for d in 1..=steps {
            let t = d as f64 * sign;
            match bilinear(&f.g, c.0 + nx * t, c.1 + ny * t) {
                Some(v) => sides[si].push(v),
                None => return Section::default(),
            }
        }
    }
    let wall_left = sides[0].iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let wall_right = sides[1].iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let depth = (wall_left.min(wall_right) - hc).max(0.0);
    if depth <= 0.0 {
        return Section {
            valid: true,
            ..Section::default()
        };
    }
    let width_at = |frac: f64| -> f64 {
        let lvl = hc + frac * depth;
        let mut tot = 0.0;
        for prof in &sides {
            let mut d = prof.len() as f64 - 1.0;
            for j in 1..prof.len() {
                if prof[j] >= lvl {
                    let (h0, h1) = (prof[j - 1], prof[j]);
                    let t = if h1 > h0 {
                        ((lvl - h0) / (h1 - h0)).clamp(0.0, 1.0)
                    } else {
                        1.0
                    };
                    d = (j - 1) as f64 + t;
                    break;
                }
            }
            tot += d;
        }
        tot * f.g.spacing
    };
    let (w25, w50, w75) = (width_at(0.25), width_at(0.5), width_at(0.75));
    let run = ((w75 - w25) / 2.0).max(f.g.spacing * 0.25);
    Section {
        depth,
        w25,
        w50,
        w75,
        slope_deg: (0.5 * depth / run).atan().to_degrees(),
        valid: true,
    }
}

fn median(v: &mut [f64]) -> f64 {
    if v.is_empty() {
        return 0.0;
    }
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

fn runs(
    f: &Field,
    p: &Params,
    stem: &[usize],
    area: &[f64],
    fill_depth: &[f64],
    out: &mut Vec<Feature>,
) {
    let s = f.g.spacing;
    let k = ((p.valley_smooth_m / 2.0) / s).round().max(1.0) as usize;
    let secs: Vec<Section> = (0..stem.len())
        .map(|i| section(f, p, stem, fill_depth, k, i))
        .collect();
    let ok: Vec<bool> = (0..stem.len()).map(|i| secs[i].valid).collect();
    // Moving mean of the depth over valid samples.
    let sm: Vec<f64> = (0..stem.len())
        .map(|i| {
            let (a, b) = (i.saturating_sub(k), (i + k + 1).min(stem.len()));
            let (mut sum, mut cnt) = (0.0, 0usize);
            for j in a..b {
                if ok[j] {
                    sum += secs[j].depth;
                    cnt += 1;
                }
            }
            if cnt > 0 { sum / cnt as f64 } else { 0.0 }
        })
        .collect();
    let deep: Vec<bool> = (0..stem.len())
        .map(|i| ok[i] && sm[i] >= p.valley_min_depth_m)
        .collect();
    let max_gap = (2 * k).max(1);
    let mut i = 0;
    while i < stem.len() {
        if !deep[i] {
            i += 1;
            continue;
        }
        let a = i;
        let mut b = i;
        let mut j = i + 1;
        while j < stem.len() {
            if deep[j] {
                b = j;
            } else if j - b > max_gap {
                break;
            }
            j += 1;
        }
        emit(f, p, stem, area, &secs, &ok, &sm, a, b, out);
        i = b + 1;
    }
}

#[allow(clippy::too_many_arguments)]
fn emit(
    f: &Field,
    p: &Params,
    stem: &[usize],
    area: &[f64],
    secs: &[Section],
    ok: &[bool],
    sm: &[f64],
    a: usize,
    b: usize,
    out: &mut Vec<Feature>,
) {
    let (w, s) = (f.g.w, f.g.spacing);
    let pts: Vec<[f64; 2]> = stem[a..=b]
        .iter()
        .map(|&i| [(i % w) as f64, (i / w) as f64])
        .collect();
    let length = pts
        .windows(2)
        .map(|q| (q[1][0] - q[0][0]).hypot(q[1][1] - q[0][1]))
        .sum::<f64>()
        * s;
    if length < p.valley_min_length_m {
        return;
    }
    let idx: Vec<usize> = (a..=b).filter(|&i| ok[i]).collect();
    if idx.is_empty() {
        return;
    }
    let mean_depth = idx.iter().map(|&i| secs[i].depth).sum::<f64>() / idx.len() as f64;
    let max_depth = idx.iter().map(|&i| sm[i]).fold(0.0, f64::max);
    let mut w50: Vec<f64> = idx.iter().map(|&i| secs[i].w50).collect();
    let mut slope: Vec<f64> = idx.iter().map(|&i| secs[i].slope_deg).collect();
    let mut floor: Vec<f64> = idx
        .iter()
        .map(|&i| {
            if secs[i].w75 > 0.0 {
                secs[i].w25 / secs[i].w75
            } else {
                0.0
            }
        })
        .collect();
    let (w50, slope, floor) = (median(&mut w50), median(&mut slope), median(&mut floor));
    let (h0, h1) = (f.g.data[stem[a]], f.g.data[stem[b]]);
    let kind = if floor >= p.trench_min_floor_ratio {
        "trench"
    } else if slope >= p.gorge_min_slope_deg {
        "gorge"
    } else {
        "valley"
    };
    let conf = f64::midpoint(
        score(mean_depth, p.valley_min_depth_m),
        score(length, p.valley_min_length_m),
    );
    let mid = pts[pts.len() / 2];
    let bb = pts.iter().fold(
        [
            f64::INFINITY,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::NEG_INFINITY,
        ],
        |b, q| {
            [
                b[0].min(q[0]),
                b[1].min(q[1]),
                b[2].max(q[0]),
                b[3].max(q[1]),
            ]
        },
    );
    let line = simplify(&pts, false, 0.75, 200);
    let metrics = vec![
        ("length_m", length),
        ("mean_depth_m", mean_depth),
        ("max_depth_m", max_depth),
        ("width_half_depth_m", w50),
        ("wall_slope_deg", slope),
        ("floor_ratio", floor),
        ("start_m", h0),
        ("end_m", h1),
        ("gradient", (h0 - h1) / length),
        ("end_area_km2", area[stem[b]] / 1e6),
    ];
    out.push(feature(
        kind,
        conf,
        f,
        [mid[0], mid[1]],
        bb,
        None,
        Some(line),
        metrics,
        length * mean_depth,
    ));
}
