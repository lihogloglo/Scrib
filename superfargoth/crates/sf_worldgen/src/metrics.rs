//! Channel-network measurements of a finished height field (printed numbers for look tests, never pass/fail).
//!
//! The network is extracted the standard way, the same for every arm whatever routing eroded it: Priority-Flood+ε
//! routing surface ([`crate::flood`]), D8 steepest descent and D8 drainage area on it (O'Callaghan & Mark,
//! "The extraction of drainage networks from digital elevation data", CVGIP 28, 1984), channels = samples whose
//! area reaches a threshold (Tarboton, Bras, Rodriguez-Iturbe, "On the extraction of channel networks from digital
//! elevation data", Hydrological Processes 5, 1991).
//!
//! - **Direction shares:** the channel length (each channel sample's link to its receiver: 1 or √2 spacings) in
//!   each of the 8 grid directions, as shares of the total. Parallel grid-aligned channels pile up in a few
//!   directions.
//! - **Sinuosity:** for the 20 largest channels (outlets into a sink, by area), the main stem is traced upstream
//!   (donor of largest area while it stays a channel); sinuosity = path length / straight distance from head to
//!   outlet (Schumm, "Sinuosity of alluvial rivers on the Great Plains", GSA Bulletin 74, 1963). Mean of the 20.

use crate::grid::NEIGHBOUR_DIST;
use crate::stream::route;
use crate::{Grid, grid::neighbour};

/// Channels whose main stem is traced for the sinuosity mean.
// tunables: const (metric definition: "the 20 largest channels", fixed by the WG-S2b brief)
pub const SINUOSITY_CHANNELS: usize = 20;

/// Numbers of one channel network.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct ChannelMetrics {
    /// Channel threshold, km².
    pub threshold_km2: f64,
    /// Channel samples.
    pub channel_samples: usize,
    /// Total channel length, km.
    pub length_km: f64,
    /// Share of channel length per direction, in [`crate::grid::NEIGHBOURS`] order: N, NE, E, SE, S, SW, W, NW.
    pub direction_share: [f64; 8],
    /// Mean sinuosity of the main stems of the largest channels.
    pub mean_sinuosity: f64,
    /// Main stems traced (≤ [`SINUOSITY_CHANNELS`]).
    pub stems: usize,
    /// Mean main-stem length of those, km.
    pub mean_stem_km: f64,
}

/// Direction names in [`crate::grid::NEIGHBOURS`] order.
pub const DIRECTION_NAMES: [&str; 8] = ["N", "NE", "E", "SE", "S", "SW", "W", "NW"];

/// Measures the channel network of `g` (samples below `sea_level` are wet) at `threshold_m2` drainage area.
pub fn channel_metrics(g: &Grid, sea_level: f64, threshold_m2: f64) -> ChannelMetrics {
    let (w, h, n) = (g.w, g.h, g.len());
    let wet: Vec<bool> = g.data.iter().map(|&v| v < sea_level).collect();
    let mut z = Vec::with_capacity(n);
    let r = route(g, &wet, sea_level, &mut z);
    let is_channel = |i: usize| !wet[i] && r.area[i] >= threshold_m2;
    let mut m = ChannelMetrics {
        threshold_km2: threshold_m2 * 1e-6,
        ..ChannelMetrics::default()
    };
    let mut len = [0.0f64; 8];
    // Largest-area donor per sample (ties: lowest index), for tracing main stems.
    let mut best_donor = vec![u32::MAX; n];
    for i in 0..n {
        let rc = r.receiver[i] as usize;
        if rc == i || !is_channel(i) {
            continue;
        }
        m.channel_samples += 1;
        let k = (0..8)
            .find(|&k| neighbour(w, h, i, k) == Some(rc))
            .expect("receiver is a neighbour");
        len[k] += NEIGHBOUR_DIST[k] * g.spacing;
        let b = best_donor[rc];
        if b == u32::MAX || r.area[i] > r.area[b as usize] {
            best_donor[rc] = i as u32;
        }
    }
    let total: f64 = len.iter().sum();
    m.length_km = total * 1e-3;
    if total > 0.0 {
        for (share, l) in m.direction_share.iter_mut().zip(len) {
            *share = l / total;
        }
    }
    // Outlets: channel samples draining into a sink (wet receiver or their own receiver on the border).
    let mut outlets: Vec<usize> = (0..n)
        .filter(|&i| {
            let rc = r.receiver[i] as usize;
            is_channel(i) && (rc == i || wet[rc])
        })
        .collect();
    outlets.sort_by(|&a, &b| r.area[b].total_cmp(&r.area[a]).then(a.cmp(&b)));
    let (mut sin_sum, mut len_sum) = (0.0, 0.0);
    for &o in outlets.iter().take(SINUOSITY_CHANNELS) {
        let (mut c, mut path) = (o, 0.0);
        loop {
            let d = best_donor[c];
            if d == u32::MAX {
                break;
            }
            let d = d as usize;
            let (dx, dy) = (
                (d % w) as f64 - (c % w) as f64,
                (d / w) as f64 - (c / w) as f64,
            );
            path += dx.hypot(dy) * g.spacing;
            c = d;
        }
        let (dx, dy) = (
            (c % w) as f64 - (o % w) as f64,
            (c / w) as f64 - (o / w) as f64,
        );
        let straight = dx.hypot(dy) * g.spacing;
        if straight > 0.0 {
            sin_sum += path / straight;
            len_sum += path;
            m.stems += 1;
        }
    }
    if m.stems > 0 {
        m.mean_sinuosity = sin_sum / m.stems as f64;
        m.mean_stem_km = len_sum / m.stems as f64 * 1e-3;
    }
    m
}
