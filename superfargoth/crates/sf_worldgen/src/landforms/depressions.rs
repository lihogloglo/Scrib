//! **Closed depressions** (craters, basins, pools) from the fill depth.
//!
//! Technique: depression filling by Priority-Flood (Barnes, Lehman, Mulla 2014, [`crate::flood::priority_fill`],
//! no ε) and the fill depth `filled − h` (the "sink depth" of GIS terrain analysis, e.g. Lindsay, "Efficient
//! hybrid breaching-filling sink removal methods for flow path enforcement in digital elevation models",
//! Hydrological Processes 30, 2016; `WhiteboxTools` `DepthInSink`). A connected set of filled samples shares one
//! level (the spill height): each component is one depression, nested pits inside it are not split.
//!
//! Per depression: area, equivalent diameter `2√(A/π)`, max depth, floor (lowest height), spill height and spill
//! point (the lowest sample just outside), circularity `4πA/P²` (perimeter by [`super::geom::chain_perimeter`]),
//! and a **rim**: `rim_rays` rays from the centroid; each leaves the depression and finds the crest (highest point) in the ring of
//! width `rim_width_ratio × r_eq` beyond its edge. A ray "has a rim" when the crest stands at least
//! `rim_rise_ratio × depth` above the outer flank (the lowest ground within one ring width beyond the crest): the ground falls away
//! outside, as it does from a raised rim and not from a valley side; completeness is the share of such rays (a raised rim that
//! goes all round is the morphometric mark of an impact or explosion crater, Pike, "Size-dependence in the shape
//! of fresh impact craters on the moon", Impact and Explosion Cratering, 1977: fresh simple craters have
//! depth/diameter ≈ 0.2 and a continuous raised rim). The ring at 2-3 `r_eq` gives the surroundings: a floor above
//! them marks a summit crater.
//!
//! Classes: `crater` (round, deep for its width, rim nearly complete), `pool` (shallow, floor near sea level),
//! `basin` (every other depression above the size floor).

use crate::flood::priority_fill;

use super::geom::{bilinear, centroid_bbox, chain_perimeter, label, members, trace_outline};
use super::{Feature, Field, Params, feature, score};

/// The depressions of `f` and the fill depth per sample (metres, 0 outside depressions).
pub fn detect(f: &Field, p: &Params) -> (Vec<Feature>, Vec<f64>) {
    let (w, h) = (f.g.w, f.g.h);
    let mut filled = Vec::new();
    priority_fill(w, h, &f.g.data, &f.wet, f.sea, &mut filled);
    let depth: Vec<f64> = filled
        .iter()
        .zip(&f.g.data)
        .zip(&f.wet)
        .map(|((&z, &v), &wet)| if wet { 0.0 } else { (z - v).max(0.0) })
        .collect();
    let mask: Vec<bool> = depth.iter().map(|&d| d > 0.0).collect();
    let (lab, n) = label(&mask, w, h);
    let s = f.g.spacing;
    let mut out = Vec::new();
    for (li, m) in members(&lab, n).iter().enumerate() {
        let area = m.len() as f64 * s * s;
        if area < p.dep_min_area_m2 {
            continue;
        }
        let max_depth = m.iter().map(|&i| depth[i as usize]).fold(0.0, f64::max);
        if max_depth < p.dep_min_depth_m {
            continue;
        }
        let l = li as u32 + 1;
        out.push(measure(
            f,
            p,
            &lab,
            l,
            m,
            &depth,
            filled[m[0] as usize],
            max_depth,
        ));
    }
    (out, depth)
}

// One depression's metrics and class read best as one pass.
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn measure(
    f: &Field,
    p: &Params,
    lab: &[u32],
    l: u32,
    m: &[u32],
    depth: &[f64],
    spill: f64,
    max_depth: f64,
) -> Feature {
    let (w, h, s) = (f.g.w, f.g.h, f.g.spacing);
    let area = m.len() as f64 * s * s;
    let floor = m
        .iter()
        .map(|&i| f.g.data[i as usize])
        .fold(f64::INFINITY, f64::min);
    let mean_depth = m.iter().map(|&i| depth[i as usize]).sum::<f64>() / m.len() as f64;
    let eq_d = 2.0 * (area / std::f64::consts::PI).sqrt();
    let (c, bb) = centroid_bbox(m, w);
    // Spill point: the lowest sample outside the depression next to it (ties by index).
    let mut spill_pt = (f64::INFINITY, 0usize);
    for &i in m {
        for k in 0..8 {
            if let Some(j) = crate::grid::neighbour(w, h, i as usize, k)
                && lab[j] != l
            {
                let v = if f.wet[j] { f.sea } else { f.g.data[j] };
                if v < spill_pt.0 || (v.to_bits() == spill_pt.0.to_bits() && j < spill_pt.1) {
                    spill_pt = (v, j);
                }
            }
        }
    }
    let chain = trace_outline(lab, w, h, l, m[0] as usize);
    let perim = chain_perimeter(&chain) * s;
    let circ = if perim > 0.0 {
        (4.0 * std::f64::consts::PI * area / (perim * perim)).min(1.0)
    } else {
        1.0
    };
    // Rim rays: leave the depression, find the crest within the ring, compare with the outer flank.
    let r_eq = eq_d / 2.0 / s;
    let ring_w = (p.rim_width_ratio * r_eq).max(2.0);
    let rays = p.rim_rays.max(4);
    let at = |dx: f64, dy: f64, t: f64| bilinear(&f.g, c[0] + dx * t, c[1] + dy * t);
    let (mut rim_rays, mut crest_sum, mut drop_sum, mut nray) = (0usize, 0.0, 0.0, 0usize);
    let (mut sur_sum, mut sur_n) = (0.0, 0usize);
    let max_r = w.max(h) as f64;
    for k in 0..rays {
        let th = std::f64::consts::TAU * k as f64 / rays as f64;
        let (dx, dy) = (th.sin(), -th.cos());
        // The last step inside the depression (beyond any island of higher ground in it).
        let (mut r, mut last_in) = (0.0, None);
        while r < max_r {
            let (x, y) = ((c[0] + dx * r).round(), (c[1] + dy * r).round());
            if x < 0.0 || y < 0.0 || x >= w as f64 || y >= h as f64 {
                break;
            }
            if lab[y as usize * w + x as usize] == l {
                last_in = Some(r);
            } else if last_in.is_some_and(|li| r - li > ring_w) {
                break;
            }
            r += 0.5;
        }
        let Some(li) = last_in else { continue };
        let r0 = li + 0.5;
        let (mut crest, mut crest_r) = (f64::NEG_INFINITY, r0);
        let mut t = r0;
        while t <= r0 + ring_w {
            if let Some(v) = at(dx, dy, t)
                && v > crest
            {
                crest = v;
                crest_r = t;
            }
            t += 0.5;
        }
        // The outer flank: the lowest ground within one ring width beyond the crest.
        let mut outer = f64::INFINITY;
        let mut t = crest_r;
        while t <= crest_r + ring_w {
            if let Some(v) = at(dx, dy, t) {
                outer = outer.min(v);
            }
            t += 0.5;
        }
        if !crest.is_finite() || !outer.is_finite() {
            continue;
        }
        nray += 1;
        crest_sum += crest - spill;
        drop_sum += crest - outer;
        if crest - outer >= p.rim_rise_ratio * max_depth {
            rim_rays += 1;
        }
        let mut t = 2.0 * r_eq;
        while t <= 3.0 * r_eq {
            if let Some(v) = at(dx, dy, t) {
                sur_sum += v;
                sur_n += 1;
            }
            t += 0.5;
        }
    }
    let completeness = rim_rays as f64 / rays as f64;
    let (rim_rise, rim_drop) = if nray > 0 {
        (crest_sum / nray as f64, drop_sum / nray as f64)
    } else {
        (0.0, 0.0)
    };
    let surround = if sur_n > 0 {
        sur_sum / sur_n as f64
    } else {
        floor
    };
    let dd = max_depth / eq_d;
    let (kind, conf) = if circ >= p.crater_min_circularity
        && dd >= p.crater_min_depth_ratio
        && completeness >= p.crater_min_rim
    {
        let c = (score(circ, p.crater_min_circularity)
            + score(dd, p.crater_min_depth_ratio)
            + score(completeness, p.crater_min_rim))
            / 3.0;
        ("crater", c)
    } else if floor - f.sea <= p.pool_max_floor_m && max_depth <= p.pool_max_depth_m {
        ("pool", score(area, p.dep_min_area_m2) * 0.5 + 0.25)
    } else {
        (
            "basin",
            f64::midpoint(
                score(max_depth, p.dep_min_depth_m),
                score(area, p.dep_min_area_m2),
            ),
        )
    };
    let sp = ((spill_pt.1 % w) as f64, (spill_pt.1 / w) as f64);
    let spill_m = f.frame.to_m(sp.0, sp.1);
    let outline: Vec<[f64; 2]> = chain.iter().map(|&(x, y)| [x as f64, y as f64]).collect();
    feature(
        kind,
        conf,
        f,
        c,
        bb,
        Some(outline),
        None,
        vec![
            ("area_m2", area),
            ("eq_diameter_m", eq_d),
            ("max_depth_m", max_depth),
            ("mean_depth_m", mean_depth),
            ("depth_ratio", dd),
            ("floor_m", floor),
            ("spill_m", spill),
            ("spill_x_m", spill_m[0]),
            ("spill_y_m", spill_m[1]),
            ("circularity", circ),
            ("rim_completeness", completeness),
            ("rim_rise_m", rim_rise),
            ("rim_drop_m", rim_drop),
            ("floor_above_surround_m", floor - surround),
            ("summit", f64::from(u8::from(floor > surround))),
        ],
        area,
    )
}
