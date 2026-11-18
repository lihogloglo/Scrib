//! **Peaks and plateaus**.
//!
//! - **Peaks by topographic prominence:** samples added from the highest down (ties by index) into a union-find of
//!   8-connected land; when a sample joins two or more components, every component but the one holding the higher
//!   summit ends there: its summit's prominence is its height minus this sample's (the key col). Components that
//!   never merge (islands, the main land) take their height above sea level. This is the watershed form of the
//!   prominence computation of Kirmse & de Ferranti ("Calculating the prominence and isolation of every mountain in
//!   the world", Progress in Physical Geography 41(6), 2017). Peaks at least `peak_min_prominence_m` prominent are
//!   kept; a summit on the grid's outer ring (where the field is cut) is not a peak.
//! - **Plateaus:** flat ground well above its surroundings: Horn slope ≤ `plateau_max_slope_deg` and topographic
//!   position index `TPI = h − mean(h)` over a square window of half-width `plateau_radius_m` at least
//!   `plateau_min_tpi_m` (Weiss, "Topographic position and landforms analysis", ESRI User Conference 2001: high
//!   TPI + low slope = mesa/plateau top), components of at least `plateau_min_area_m2`; samples nearer the grid
//!   edge than the window are left out (their window is cut).
//!
//! Left out: geomorphons (Jasiewicz & Stepinski, Geomorphology 182, 2013) and ridge lines. Prominence answers
//! "is there a peak here" with one physical threshold and TPI + slope the plateau; geomorphon peak/ridge classes at
//! the lookup distances these landmarks need (hundreds of metres) mark whole hill tops and ridge crests and gave
//! nothing the two above miss, so they are not shipped in this slice.

use super::geom::{box_mean, centroid_bbox, label, members, slope_deg, trace_outline};
use super::{Feature, Field, Params, feature, score};

// tunables: const (sentinel: no parent, no col)
const NONE: u32 = u32::MAX;

fn find(parent: &mut [u32], mut i: u32) -> u32 {
    while parent[i as usize] != i {
        let g = parent[parent[i as usize] as usize];
        parent[i as usize] = g;
        i = g;
    }
    i
}

/// Peaks with their prominence and key col.
pub fn peaks(f: &Field, p: &Params) -> Vec<Feature> {
    let (w, h) = (f.g.w, f.g.h);
    let z = &f.g.data;
    let mut order: Vec<u32> = (0..(w * h) as u32)
        .filter(|&i| !f.wet[i as usize])
        .collect();
    order.sort_unstable_by(|&a, &b| z[b as usize].total_cmp(&z[a as usize]).then(a.cmp(&b)));
    let mut parent = vec![NONE; w * h];
    let mut summit = vec![NONE; w * h];
    // (summit, col sample or NONE)
    let mut found: Vec<(u32, u32)> = Vec::new();
    let higher = |a: u32, b: u32| {
        z[a as usize] > z[b as usize]
            || (z[a as usize].to_bits() == z[b as usize].to_bits() && a < b)
    };
    let mut roots: Vec<u32> = Vec::with_capacity(8);
    for &i in &order {
        roots.clear();
        for k in 0..8 {
            if let Some(j) = crate::grid::neighbour(w, h, i as usize, k)
                && parent[j] != NONE
            {
                let r = find(&mut parent, j as u32);
                if !roots.contains(&r) {
                    roots.push(r);
                }
            }
        }
        parent[i as usize] = i;
        if roots.is_empty() {
            summit[i as usize] = i;
            continue;
        }
        let mut best = roots[0];
        for &r in &roots[1..] {
            if higher(summit[r as usize], summit[best as usize]) {
                best = r;
            }
        }
        for &r in &roots {
            if r != best {
                let sm = summit[r as usize];
                if z[sm as usize] - z[i as usize] >= p.peak_min_prominence_m {
                    found.push((sm, i));
                }
                parent[r as usize] = best;
            }
        }
        parent[i as usize] = best;
    }
    for &i in &order {
        if parent[i as usize] == i {
            let sm = summit[i as usize];
            if z[sm as usize] - f.sea >= p.peak_min_prominence_m {
                found.push((sm, NONE));
            }
        }
    }
    // A summit on the grid's outer ring is where the field is cut, not a peak.
    let border = |i: u32| {
        let (x, y) = (i as usize % w, i as usize / w);
        x == 0 || y == 0 || x + 1 == w || y + 1 == h
    };
    found
        .into_iter()
        .filter(|&(sm, _)| !border(sm))
        .map(|(sm, col)| {
            let (sx, sy) = ((sm as usize % w) as f64, (sm as usize / w) as f64);
            let col_h = if col == NONE { f.sea } else { z[col as usize] };
            let prom = z[sm as usize] - col_h;
            let mut metrics = vec![
                ("height_m", z[sm as usize]),
                ("prominence_m", prom),
                ("col_m", col_h),
            ];
            if col != NONE {
                let (cx, cy) = ((col as usize % w) as f64, (col as usize / w) as f64);
                let cm = f.frame.to_m(cx, cy);
                metrics.push(("col_x_m", cm[0]));
                metrics.push(("col_y_m", cm[1]));
                metrics.push(("col_distance_m", (cx - sx).hypot(cy - sy) * f.g.spacing));
            }
            let conf = score(prom, p.peak_min_prominence_m);
            feature(
                "peak",
                conf,
                f,
                [sx, sy],
                [sx, sy, sx, sy],
                None,
                None,
                metrics,
                prom,
            )
        })
        .collect()
}

/// Plateaus: low slope and high TPI.
pub fn plateaus(f: &Field, p: &Params) -> Vec<Feature> {
    let (w, h, s) = (f.g.w, f.g.h, f.g.spacing);
    let slope = slope_deg(&f.g, &f.wet);
    let base: Vec<f64> =
        f.g.data
            .iter()
            .zip(&f.wet)
            .map(|(&v, &wet)| if wet { f.sea } else { v })
            .collect();
    let r = (p.plateau_radius_m / s).round().max(1.0) as usize;
    let mean = box_mean(&base, w, h, r);
    let tpi: Vec<f64> = base.iter().zip(&mean).map(|(a, b)| a - b).collect();
    let mask: Vec<bool> = (0..w * h)
        .map(|i| {
            // TPI needs its whole window: samples nearer the grid edge than the radius are left out.
            let (x, y) = (i % w, i / w);
            let inner = x >= r && y >= r && x + r < w && y + r < h;
            inner
                && !f.wet[i]
                && slope[i] <= p.plateau_max_slope_deg
                && tpi[i] >= p.plateau_min_tpi_m
        })
        .collect();
    let (lab, n) = label(&mask, w, h);
    let mut out = Vec::new();
    for (li, m) in members(&lab, n).iter().enumerate() {
        let area = m.len() as f64 * s * s;
        if area < p.plateau_min_area_m2 {
            continue;
        }
        let k = m.len() as f64;
        let mh = m.iter().map(|&i| base[i as usize]).sum::<f64>() / k;
        let mt = m.iter().map(|&i| tpi[i as usize]).sum::<f64>() / k;
        let ms = m.iter().map(|&i| slope[i as usize]).sum::<f64>() / k;
        let top = m
            .iter()
            .map(|&i| base[i as usize])
            .fold(f64::NEG_INFINITY, f64::max);
        let (c, bb) = centroid_bbox(m, w);
        let chain = trace_outline(&lab, w, h, li as u32 + 1, m[0] as usize);
        let outline = chain.iter().map(|&(x, y)| [x as f64, y as f64]).collect();
        let conf = f64::midpoint(
            score(area, p.plateau_min_area_m2),
            score(mt, p.plateau_min_tpi_m),
        );
        let metrics = vec![
            ("area_m2", area),
            ("mean_height_m", mh),
            ("max_height_m", top),
            ("mean_tpi_m", mt),
            ("mean_slope_deg", ms),
        ];
        out.push(feature(
            "plateau",
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
