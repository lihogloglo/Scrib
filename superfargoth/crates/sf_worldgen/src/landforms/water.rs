//! **Water bodies and coast**: lakes, lagoons, islands, bays, headlands, from the wet/dry mask alone.
//!
//! - **Lakes:** 8-connected wet components that do not touch the grid border (the open sea is the wet water that
//!   reaches the edge of the map).
//! - **Lagoons:** water joined to the sea only through necks narrower than `neck_width_m`: the morphological
//!   opening of the wet mask by a disc of radius `neck_width_m / 2` (Serra, "Image Analysis and Mathematical
//!   Morphology", 1982; erosion and dilation by exact Euclidean distance transforms, Felzenszwalb & Huttenlocher
//!   2012, [`crate::measure::distance_transform`]) cuts the necks; an opened component of sea water that no longer
//!   reaches the border is a lagoon (its outline is the opened one, so fringes narrower than the neck are left out).
//! - **Islands:** dry components other than the largest one.
//! - **Bays and headlands:** the coastline's concavities at one scale, by morphological closing (the raster form
//!   of the concavity measure: what a closing by a disc of radius `r` adds is the shape's "convex deficiency" at
//!   that scale, Serra 1982 ch. 2; the convex hull is its limit as `r` grows). A bay is sea water the closing of
//!   the land adds; a headland is land the closing of the sea adds. Each keeps its **mouth** (the samples touching
//!   the unclosed water or land), mouth width (farthest pair of mouth samples) and inland depth (largest distance
//!   from the mouth), and must be deep for its mouth (`bay_min_depth_ratio`, `headland_min_depth_ratio`: a
//!   closing also adds the land corners beside every narrow inlet, so a headland must stick out further than it
//!   is wide). Pieces without a mouth (enclosed)
//!   are lagoons or islands and are skipped here.

use crate::measure::distance_transform;

use super::geom::{centroid_bbox, label, members, touches_border, trace_outline};
use super::{Feature, Field, Params, feature, score};

/// Lakes, lagoons, islands, bays, headlands, and the routing water mask: the wet mask without the sea's ribbons
/// (sea water the neck opening removes: rivers and creeks narrower than the neck, which the channel detector
/// routes through as ground).
pub fn detect(f: &Field, p: &Params) -> (Vec<Feature>, Vec<bool>) {
    let (w, h, s) = (f.g.w, f.g.h, f.g.spacing);
    let mut out = Vec::new();
    let (wlab, wn) = label(&f.wet, w, h);
    let wmem = members(&wlab, wn);
    let mut sea = vec![false; w * h];
    for (li, m) in wmem.iter().enumerate() {
        if touches_border(m, w, h) {
            for &i in m {
                sea[i as usize] = true;
            }
        } else if m.len() as f64 * s * s >= p.lake_min_area_m2 {
            out.push(water_body(f, "lake", &wlab, li as u32 + 1, m, p));
        }
    }

    // Lagoons: opening of the sea by the neck radius.
    let r = p.neck_width_m / 2.0 / s;
    let dry: Vec<bool> = sea.iter().map(|&v| !v).collect();
    let d_dry = distance_transform(&dry, w, h);
    let eroded: Vec<bool> = (0..w * h).map(|i| sea[i] && d_dry[i] > r).collect();
    let d_er = distance_transform(&eroded, w, h);
    let opened: Vec<bool> = (0..w * h).map(|i| sea[i] && d_er[i] <= r).collect();
    let (olab, on) = label(&opened, w, h);
    let mut lagoon = vec![false; w * h];
    for (li, m) in members(&olab, on).iter().enumerate() {
        if touches_border(m, w, h) || (m.len() as f64 * s * s) < p.lake_min_area_m2 {
            continue;
        }
        for &i in m {
            lagoon[i as usize] = true;
        }
        out.push(water_body(f, "lagoon", &olab, li as u32 + 1, m, p));
    }

    // Islands.
    let land: Vec<bool> = f.wet.iter().map(|&v| !v).collect();
    let (llab, ln) = label(&land, w, h);
    let lmem = members(&llab, ln);
    let main = (0..ln).max_by(|&a, &b| lmem[a].len().cmp(&lmem[b].len()).then(b.cmp(&a)));
    for (li, m) in lmem.iter().enumerate() {
        let area = m.len() as f64 * s * s;
        if Some(li) == main || area < p.island_min_area_m2 {
            continue;
        }
        let (c, bb) = centroid_bbox(m, w);
        let top = m
            .iter()
            .map(|&i| f.g.data[i as usize])
            .fold(f.sea, f64::max);
        let chain = trace_outline(&llab, w, h, li as u32 + 1, m[0] as usize);
        let outline = chain.iter().map(|&(x, y)| [x as f64, y as f64]).collect();
        let conf = score(area, p.island_min_area_m2);
        let metrics = vec![("area_m2", area), ("max_height_m", top - f.sea)];
        out.push(feature(
            "island",
            conf,
            f,
            c,
            bb,
            Some(outline),
            None,
            metrics,
            area,
        ));
    }

    // Bays: sea (not lagoon) that the closing of the land adds.
    let open_sea: Vec<bool> = (0..w * h).map(|i| sea[i] && !lagoon[i]).collect();
    let r_bay = p.bay_radius_m / s;
    out.extend(indentations(f, p, &land, &open_sea, r_bay, "bay"));
    // Headlands: land the closing of the sea adds.
    out.extend(indentations(f, p, &sea, &land, r_bay, "headland"));
    let route_wet = (0..w * h)
        .map(|i| f.wet[i] && (!sea[i] || opened[i]))
        .collect();
    (out, route_wet)
}

fn water_body(
    f: &Field,
    kind: &'static str,
    lab: &[u32],
    l: u32,
    m: &[u32],
    p: &Params,
) -> Feature {
    let (w, h, s) = (f.g.w, f.g.h, f.g.spacing);
    let area = m.len() as f64 * s * s;
    let (c, bb) = centroid_bbox(m, w);
    let bottom = m
        .iter()
        .map(|&i| f.g.data[i as usize])
        .fold(f.sea, f64::min);
    let mean = m.iter().map(|&i| f.sea - f.g.data[i as usize]).sum::<f64>() / m.len() as f64;
    let chain = trace_outline(lab, w, h, l, m[0] as usize);
    let outline = chain.iter().map(|&(x, y)| [x as f64, y as f64]).collect();
    let metrics = vec![
        ("area_m2", area),
        ("eq_diameter_m", 2.0 * (area / std::f64::consts::PI).sqrt()),
        ("surface_m", f.sea),
        ("max_depth_m", f.sea - bottom),
        ("mean_depth_m", mean),
    ];
    let conf = score(area, p.lake_min_area_m2);
    feature(kind, conf, f, c, bb, Some(outline), None, metrics, area)
}

/// Pieces of `target` that the closing of `solid` by radius `r` samples adds, with a mouth on unclosed `target`.
fn indentations(
    f: &Field,
    p: &Params,
    solid: &[bool],
    target: &[bool],
    r: f64,
    kind: &'static str,
) -> Vec<Feature> {
    let (w, h, s) = (f.g.w, f.g.h, f.g.spacing);
    let n = w * h;
    let d_solid = distance_transform(solid, w, h);
    let not_dil: Vec<bool> = d_solid.iter().map(|&d| d > r).collect();
    let d_out = distance_transform(&not_dil, w, h);
    let closed: Vec<bool> = (0..n).map(|i| solid[i] || d_out[i] > r).collect();
    let cand: Vec<bool> = (0..n).map(|i| closed[i] && target[i]).collect();
    let (lab, ln) = label(&cand, w, h);
    let mem = members(&lab, ln);
    let mut mouth = vec![false; n];
    for i in 0..n {
        if cand[i]
            && (0..8).any(|k| {
                crate::grid::neighbour(w, h, i, k).is_some_and(|j| target[j] && !closed[j])
            })
        {
            mouth[i] = true;
        }
    }
    let d_mouth = distance_transform(&mouth, w, h);
    let mut out = Vec::new();
    for (li, m) in mem.iter().enumerate() {
        let area = m.len() as f64 * s * s;
        if area < p.bay_min_area_m2 {
            continue;
        }
        let mpts: Vec<(f64, f64)> = m
            .iter()
            .filter(|&&i| mouth[i as usize])
            .map(|&i| ((i as usize % w) as f64, (i as usize / w) as f64))
            .collect();
        if mpts.len() < 2 {
            continue;
        }
        // Farthest mouth pair (a subsample of at most 400 points keeps it O(400²)).
        let step = mpts.len().div_ceil(400);
        let sub: Vec<(f64, f64)> = mpts.iter().step_by(step).copied().collect();
        let mut best = (0.0, 0usize, 0usize);
        for a in 0..sub.len() {
            for b in a + 1..sub.len() {
                let d = (sub[a].0 - sub[b].0).hypot(sub[a].1 - sub[b].1);
                if d > best.0 {
                    best = (d, a, b);
                }
            }
        }
        let mouth_w = (best.0 + 1.0) * s;
        let depth = m.iter().map(|&i| d_mouth[i as usize]).fold(0.0, f64::max) * s;
        let ratio = depth / mouth_w;
        let min_ratio = if kind == "bay" {
            p.bay_min_depth_ratio
        } else {
            p.headland_min_depth_ratio
        };
        if ratio < min_ratio {
            continue;
        }
        let (c, bb) = centroid_bbox(m, w);
        let chain = trace_outline(&lab, w, h, li as u32 + 1, m[0] as usize);
        let outline = chain.iter().map(|&(x, y)| [x as f64, y as f64]).collect();
        let (ma, mb) = (
            f.frame.to_m(sub[best.1].0, sub[best.1].1),
            f.frame.to_m(sub[best.2].0, sub[best.2].1),
        );
        let conf = f64::midpoint(score(area, p.bay_min_area_m2), score(ratio, min_ratio));
        let metrics = vec![
            ("area_m2", area),
            ("mouth_width_m", mouth_w),
            ("inland_depth_m", depth),
            ("depth_ratio", ratio),
            ("mouth_a_x_m", ma[0]),
            ("mouth_a_y_m", ma[1]),
            ("mouth_b_x_m", mb[0]),
            ("mouth_b_y_m", mb[1]),
        ];
        out.push(feature(
            kind,
            conf,
            f,
            c,
            bb,
            Some(outline),
            None,
            metrics,
            area,
        ));
    }
    out
}
