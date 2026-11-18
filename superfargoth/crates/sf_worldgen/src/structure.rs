//! Channel-network **structure** metrics (WG-S4; printed numbers, never pass/fail): channel parallelism, drainage
//! density, junction angles. They replace WG-S2b's direction shares, which stayed ~uniform on fields the owner saw
//! as evenly spaced parallel grooves.
//!
//! The network is extracted as in [`crate::metrics`]: Priority-Flood+ε surface, D8 receivers and D8 drainage area
//! (O'Callaghan & Mark 1984), channels where the area reaches a threshold (Tarboton, Bras, Rodriguez-Iturbe 1991).
//!
//! - **Orientation coherence** (parallelism): each channel sample gets the direction to the sample `k` receivers
//!   downstream (smooths the 8-direction D8 zigzag). Orientations are axial, so they are averaged in the
//!   double-angle representation `(cos 2θ, sin 2θ)`; over a square window of edge `window_m` around each channel
//!   sample the coherence is `|Σ e^{2iθ}| / N`, which equals `(λ1 − λ2)/(λ1 + λ2)` of the orientation (structure)
//!   tensor `Σ v vᵀ` of unit vectors (Bigün & Granlund, "Optimal orientation detection of linear symmetry",
//!   ICCV 1987; Jähne, "Digital Image Processing", ch. 13). 1 = every channel in the window runs the same way
//!   (parallel grooves), lower = branching. Window sums by summed-area tables (Crow, SIGGRAPH 1984).
//!   A window holding a single straight channel also scores 1, so the number compares fields at the same
//!   threshold and window, not absolute.
//! - **Drainage density:** channel length per land area, km/km² (Horton, "Erosional development of streams and
//!   their drainage basins", GSA Bulletin 56, 1945).
//! - **Junction angle:** at each confluence (a channel sample with ≥ 2 channel donors), the angle between the two
//!   largest tributaries, each taken as the vector to the sample `k` steps up its main stem (Seybold, Rothman,
//!   Kirchner, "Climate's watermark in the geometry of stream networks", GRL 44, 2017: ~45° arid to ~72° humid,
//!   measured on real networks; U = unverified against their exact segment length).
//!
//! Deterministic: single-threaded except the routing's pure per-sample receiver map.

use crate::stream::route;
use crate::Grid;

/// Numbers of one channel network's structure.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct Structure {
    pub threshold_km2: f64,
    pub window_m: f64,
    pub dir_steps: usize,
    pub channel_samples: usize,
    pub length_km: f64,
    pub land_km2: f64,
    /// km of channel per km² of land.
    pub drainage_density: f64,
    /// Orientation coherence over channel samples: mean, median, 90th percentile.
    pub coherence_mean: f64,
    pub coherence_median: f64,
    pub coherence_p90: f64,
    pub junctions: usize,
    /// Junction angle, degrees: mean and median.
    pub junction_mean_deg: f64,
    pub junction_median_deg: f64,
}

fn quantile(sorted: &[f64], f: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    sorted[((sorted.len() - 1) as f64 * f).round() as usize]
}

/// Measures the structure of the channel network of `g` at `threshold_m2` drainage area.
#[allow(clippy::too_many_lines, clippy::needless_range_loop)]
pub fn structure(
    g: &Grid,
    sea_level: f64,
    threshold_m2: f64,
    window_m: f64,
    dir_steps: usize,
) -> Structure {
    let (w, h, n) = (g.w, g.h, g.len());
    let wet: Vec<bool> = g.data.iter().map(|&v| v < sea_level).collect();
    let mut z = Vec::with_capacity(n);
    let r = route(g, &wet, sea_level, &mut z);
    let recv = |i: usize| r.receiver[i] as usize;
    let is_channel = |i: usize| !wet[i] && r.area[i] >= threshold_m2 && recv(i) != i;
    let land = wet.iter().filter(|v| !**v).count();
    let mut s = Structure {
        threshold_km2: threshold_m2 * 1e-6,
        window_m,
        dir_steps,
        land_km2: land as f64 * g.spacing * g.spacing * 1e-6,
        ..Structure::default()
    };
    let xy = |i: usize| ((i % w) as f64, (i / w) as f64);
    // Double-angle orientation per channel sample.
    let mut c2 = vec![0.0; n];
    let mut s2 = vec![0.0; n];
    let mut one = vec![0.0; n];
    let mut len = 0.0;
    // Largest-area channel donor per sample (ties: lowest index), and the second one.
    let mut d1 = vec![u32::MAX; n];
    let mut d2 = vec![u32::MAX; n];
    let mut ndon = vec![0u8; n];
    for i in 0..n {
        if !is_channel(i) {
            continue;
        }
        s.channel_samples += 1;
        let rc = recv(i);
        let (xi, yi) = xy(i);
        let (xr, yr) = xy(rc);
        len += (xr - xi).hypot(yr - yi) * g.spacing;
        let mut j = i;
        for _ in 0..dir_steps.max(1) {
            let nj = recv(j);
            if nj == j {
                break;
            }
            j = nj;
        }
        let (xj, yj) = xy(j);
        let (vx, vy) = (xj - xi, yj - yi);
        let m2 = vx * vx + vy * vy;
        if m2 > 0.0 {
            c2[i] = (vx * vx - vy * vy) / m2;
            s2[i] = 2.0 * vx * vy / m2;
            one[i] = 1.0;
        }
        ndon[rc] = ndon[rc].saturating_add(1);
        let better = |a: usize, b: u32| b == u32::MAX || r.area[a] > r.area[b as usize];
        if better(i, d1[rc]) {
            d2[rc] = d1[rc];
            d1[rc] = i as u32;
        } else if better(i, d2[rc]) {
            d2[rc] = i as u32;
        }
    }
    s.length_km = len * 1e-3;
    if s.land_km2 > 0.0 {
        s.drainage_density = s.length_km / s.land_km2;
    }
    // Summed-area tables, (w + 1) × (h + 1).
    let sat = |f: &[f64]| {
        let mut t = vec![0.0; (w + 1) * (h + 1)];
        for y in 0..h {
            let mut row = 0.0;
            for x in 0..w {
                row += f[y * w + x];
                t[(y + 1) * (w + 1) + x + 1] = t[y * (w + 1) + x + 1] + row;
            }
        }
        t
    };
    let (tc, ts, tn) = (sat(&c2), sat(&s2), sat(&one));
    let half = ((window_m / 2.0) / g.spacing).round().max(1.0) as usize;
    let boxsum = |t: &[f64], x0: usize, y0: usize, x1: usize, y1: usize| {
        t[y1 * (w + 1) + x1] - t[y0 * (w + 1) + x1] - t[y1 * (w + 1) + x0] + t[y0 * (w + 1) + x0]
    };
    let mut coh = Vec::with_capacity(s.channel_samples);
    for i in 0..n {
        if one[i] == 0.0 {
            continue;
        }
        let (x, y) = (i % w, i / w);
        let (x0, y0) = (x.saturating_sub(half), y.saturating_sub(half));
        let (x1, y1) = ((x + half + 1).min(w), (y + half + 1).min(h));
        let cn = boxsum(&tn, x0, y0, x1, y1);
        if cn > 0.0 {
            let cc = boxsum(&tc, x0, y0, x1, y1);
            let cs = boxsum(&ts, x0, y0, x1, y1);
            coh.push(cc.hypot(cs) / cn);
        }
    }
    if !coh.is_empty() {
        s.coherence_mean = coh.iter().sum::<f64>() / coh.len() as f64;
        coh.sort_by(f64::total_cmp);
        s.coherence_median = quantile(&coh, 0.5);
        s.coherence_p90 = quantile(&coh, 0.9);
    }
    // Junction angles.
    let up = |start: usize| {
        let mut j = start;
        for _ in 1..dir_steps.max(1) {
            let d = d1[j];
            if d == u32::MAX {
                break;
            }
            j = d as usize;
        }
        j
    };
    let mut ang = Vec::new();
    for c in 0..n {
        if ndon[c] < 2 || wet[c] || d2[c] == u32::MAX {
            continue;
        }
        let (xc, yc) = xy(c);
        let (a, b) = (up(d1[c] as usize), up(d2[c] as usize));
        let ((xa, ya), (xb, yb)) = (xy(a), xy(b));
        let (ax, ay, bx, by) = (xa - xc, ya - yc, xb - xc, yb - yc);
        let den = ax.hypot(ay) * bx.hypot(by);
        if den > 0.0 {
            ang.push(((ax * bx + ay * by) / den).clamp(-1.0, 1.0).acos().to_degrees());
        }
    }
    s.junctions = ang.len();
    if !ang.is_empty() {
        s.junction_mean_deg = ang.iter().sum::<f64>() / ang.len() as f64;
        ang.sort_by(f64::total_cmp);
        s.junction_median_deg = quantile(&ang, 0.5);
    }
    s
}
