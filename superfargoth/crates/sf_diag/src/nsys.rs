//! NVIDIA Nsight Systems GPU pass timings.
//!
//! `nsys stats --report vulkan_gpu_marker_sum --format csv` sums the GPU
//! time of every Vulkan debug label. wgpu emits one per pass label when
//! its instance has `InstanceFlags::DEBUG` (`WGPU_DEBUG=1`), so this times
//! every render and compute pass, including those Bevy's
//! `RenderDiagnosticsPlugin` leaves untimed.

use std::fmt::Write as _;

use crate::DiagError;

/// One label's row of `vulkan_gpu_marker_sum`.
#[derive(Clone, Debug, PartialEq)]
pub struct MarkerSum {
    pub name: String,
    pub total_ns: f64,
    pub instances: u64,
    pub avg_ns: f64,
    pub med_ns: f64,
    pub min_ns: f64,
    pub max_ns: f64,
}

/// Splits one CSV line (RFC 4180 quotes).
fn split_csv(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut field = String::new();
    let mut quoted = false;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        match (c, quoted) {
            ('"', true) if chars.peek() == Some(&'"') => {
                field.push('"');
                chars.next();
            }
            ('"', _) => quoted = !quoted,
            (',', false) => fields.push(std::mem::take(&mut field)),
            _ => field.push(c),
        }
    }
    fields.push(field);
    fields
}

/// Parses the CSV, finding columns by their header names.
pub fn parse_marker_sum(csv: &str) -> Result<Vec<MarkerSum>, DiagError> {
    let mut lines = csv.lines().filter(|l| !l.trim().is_empty());
    let header = lines
        .next()
        .ok_or_else(|| DiagError::Scenario("empty marker CSV".into()))?;
    let header = split_csv(header.trim_start_matches('\u{feff}'));
    let col = |name: &str| {
        header
            .iter()
            .position(|h| h.trim() == name)
            .ok_or_else(|| DiagError::Scenario(format!("marker CSV has no `{name}` column")))
    };
    let (total, inst, avg, med, min, max, range) = (
        col("Total Time (ns)")?,
        col("Instances")?,
        col("Avg (ns)")?,
        col("Med (ns)")?,
        col("Min (ns)")?,
        col("Max (ns)")?,
        col("Range")?,
    );
    let mut rows = Vec::new();
    for line in lines {
        let fields = split_csv(line);
        let num = |i: usize| fields.get(i).and_then(|v| v.trim().parse::<f64>().ok());
        let (Some(t), Some(n), Some(a), Some(m), Some(lo), Some(hi), Some(name)) = (
            num(total),
            num(inst),
            num(avg),
            num(med),
            num(min),
            num(max),
            fields.get(range),
        ) else {
            continue;
        };
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // a count
        let instances = n as u64;
        rows.push(MarkerSum {
            name: name.trim().to_owned(),
            total_ns: t,
            instances,
            avg_ns: a,
            med_ns: m,
            min_ns: lo,
            max_ns: hi,
        });
    }
    Ok(rows)
}

/// The likely frame count: the most common instance count (most passes
/// run once per frame); ties go to the larger count.
pub fn frames_estimate(rows: &[MarkerSum]) -> Option<u64> {
    let mut counts: Vec<(u64, usize)> = Vec::new();
    for r in rows {
        match counts.iter_mut().find(|(n, _)| *n == r.instances) {
            Some((_, c)) => *c += 1,
            None => counts.push((r.instances, 1)),
        }
    }
    counts
        .into_iter()
        .max_by_key(|&(n, c)| (c, n))
        .map(|(n, _)| n)
}

/// Per-pass GPU time per frame, largest first. `per frame (median)` is
/// the median instance × instances per frame: robust against labels that
/// sometimes span a wait. `per frame (mean)` is the total / frames.
pub fn marker_markdown(rows: &[MarkerSum], frames: Option<u64>) -> String {
    let frames = frames.or_else(|| frames_estimate(rows)).unwrap_or(1).max(1);
    #[allow(clippy::cast_precision_loss)]
    let nf = frames as f64;
    let mut sorted: Vec<&MarkerSum> = rows.iter().collect();
    #[allow(clippy::cast_precision_loss)]
    let per_frame_med = |r: &MarkerSum| r.med_ns * r.instances as f64 / nf / 1e6;
    sorted.sort_by(|a, b| per_frame_med(b).total_cmp(&per_frame_med(a)));
    let sum_med: f64 = sorted.iter().map(|r| per_frame_med(r)).sum();
    let mut out = String::new();
    let _ = writeln!(
        out,
        "{frames} frames (most common instance count). Sum of per-frame medians: {sum_med:.2} ms \
         (nested labels count twice; `?` marks labels whose mean is far above their median: they \
         sometimes include a wait, trust the median)\n"
    );
    let _ = writeln!(
        out,
        "| pass | per frame ms (median) | per frame ms (mean) | calls/frame | median µs | max µs | |"
    );
    let _ = writeln!(out, "|---|---:|---:|---:|---:|---:|---|");
    for r in sorted {
        let mean = r.total_ns / nf / 1e6;
        let med = per_frame_med(r);
        let suspect = mean > 2.0 * med + 0.05 && r.max_ns > 10.0 * r.med_ns;
        #[allow(clippy::cast_precision_loss)]
        let calls = r.instances as f64 / nf;
        let _ = writeln!(
            out,
            "| {} | {med:.3} | {mean:.3} | {calls:.2} | {:.1} | {:.1} | {} |",
            r.name,
            r.med_ns / 1e3,
            r.max_ns / 1e3,
            if suspect { "?" } else { "" }
        );
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Time (%),Total Time (ns),Instances,Avg (ns),Med (ns),Min (ns),Max (ns),StdDev (ns),Range
83.3,18679262225,290,64411249.1,11784523.5,1,1959810455,228054932.8,bloom
2.9,651085497,290,2245122.4,2084187.5,2062074,5881446,427549.6,main_opaque_pass_3d
0.5,108395365,580,186888.6,174758.0,9474,238760,28740.2,ssao
0.1,31577708,177,178405.1,171814.0,162949,485102,43266.2,shadow_directional_light_0_cascade_3
0.0,4467273,870,5134.8,3841.0,3489,267752,16862.3,\"(wgpu internal) Indirect draw validation pass, \"\"x\"\"\"
";

    #[test]
    fn parses_marker_sum_and_estimates_frames() {
        let rows = parse_marker_sum(SAMPLE).unwrap();
        assert_eq!(rows.len(), 5);
        assert_eq!(rows[1].name, "main_opaque_pass_3d");
        assert_eq!(rows[2].instances, 580);
        assert_eq!(
            rows[4].name,
            "(wgpu internal) Indirect draw validation pass, \"x\""
        );
        assert_eq!(frames_estimate(&rows), Some(290));
        let md = marker_markdown(&rows, None);
        // Opaque: 2.084 ms median, once per frame.
        assert!(md.contains("| main_opaque_pass_3d | 2.084 |"), "{md}");
        // SSAO runs twice per frame: 2 × 0.175 ms.
        assert!(md.contains("| ssao | 0.350 |"), "{md}");
        // Bloom's mean (64 ms) is far above its median: flagged.
        assert!(
            md.lines()
                .any(|l| l.starts_with("| bloom |") && l.ends_with("| ? |")),
            "{md}"
        );
        assert!(parse_marker_sum("a,b\n1,2").is_err());
    }
}
