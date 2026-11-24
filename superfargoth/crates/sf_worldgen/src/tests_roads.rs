//! Tests of `roads`: the `roads.json` reader and the draped ribbon.

// Seams, fades and run ends are exact by construction (copied values, not recomputed), so they compare exactly.
#![allow(clippy::float_cmp)]

use super::*;

const SAMPLE: &str = r#"{
  "format": 1,
  "tool": "hand-written",
  "frame": { "west_m": 0, "north_m": 0 },
  "segments": [
    { "id": 0, "class": "main", "surface": "dirt", "profile": "x", "width_m": 6.5, "a": 0, "b": 1,
      "points": [[0, 0], [100, 0], [200, 50]], "crossings": [], "length_m": 212.0, "grade": 0.05,
      "importance": 12 },
    { "id": 1, "class": "motorway", "surface": "paved", "width_m": 9.0, "a": 1, "b": 2,
      "points": [[200, 50], [300, 50], [400, 50]],
      "crossings": [
        { "kind": "bridge", "points": [[240, 50], [260, 50]] },
        { "kind": "ford", "points": [[300, 50], [310, 50]] },
        { "kind": "access", "points": [[390, 50], [400, 50]] },
        { "kind": "ferry", "points": [[395, 50], [400, 50]] }
      ], "length_m": 200.0, "grade": 0.0 }
  ],
  "junctions": [
    { "id": 0, "pos": [0, 0], "degree": 1, "kind": "site", "sites": ["a"] },
    { "id": 1, "pos": [200, 50], "degree": 2, "kind": "junction", "sites": [] },
    { "id": 2, "pos": [400, 50] }
  ],
  "warnings": ["one"],
  "unreachable": ["b"],
  "length_by_class_m": { "main": 212.0 }
}"#;

fn temp_dir(tag: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("sf_worldgen_roads_{tag}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn seg(points: Vec<[f64; 2]>, width_m: f64) -> Segment {
    Segment {
        id: 7,
        class: RoadClass::Local,
        surface: "dirt".into(),
        width_m,
        a: 0,
        b: 1,
        points,
        crossings: Vec::new(),
        length_m: 0.0,
        grade: 0.0,
    }
}

/// A winding road over ~1.3 km at the router's coarse spacing.
fn winding() -> Segment {
    let pts = (0..15)
        .map(|i| {
            let x = f64::from(i) * 90.0;
            [x, 120.0 * (x / 300.0).sin() + if i % 3 == 0 { 25.0 } else { 0.0 }]
        })
        .collect();
    seg(pts, 4.5)
}

fn slope(x: f64, y: f64) -> f64 {
    0.2 * x - 0.15 * y + 3.0 * (x / 17.0).sin() * (y / 23.0).cos() + 40.0
}

fn world(ch: &RibbonChunk, k: usize) -> [f64; 3] {
    let p = ch.positions[k];
    [ch.origin[0] + f64::from(p[0]), ch.origin[1] + f64::from(p[1]), ch.origin[2] + f64::from(p[2])]
}

#[test]
fn reads_hand_written_file() {
    let dir = temp_dir("read");
    assert!(RoadNetwork::read(&dir).unwrap().is_none(), "absent file reads as None");
    std::fs::write(dir.join("roads.json"), SAMPLE).unwrap();
    let net = RoadNetwork::read(&dir).unwrap().expect("file present");
    assert_eq!(net.segments.len(), 2);
    assert_eq!(net.junctions.len(), 3);
    assert_eq!(net.warnings, vec!["one".to_string()]);
    assert_eq!(net.unreachable, vec!["b".to_string()]);
    let s0 = &net.segments[0];
    assert_eq!((s0.id, s0.class, s0.a, s0.b), (0, RoadClass::Main, 0, 1));
    assert_eq!(s0.points, vec![[0.0, 0.0], [100.0, 0.0], [200.0, 50.0]]);
    assert!((s0.width_m - 6.5).abs() < 1e-12 && (s0.grade - 0.05).abs() < 1e-12);
    let s1 = &net.segments[1];
    assert_eq!(s1.class, RoadClass::Other, "unknown class");
    let kinds: Vec<_> = s1.crossings.iter().map(|c| c.kind).collect();
    assert_eq!(kinds, [CrossingKind::Bridge, CrossingKind::Ford, CrossingKind::Access, CrossingKind::Other]);
    assert_eq!(net.junctions[0].sites, vec!["a".to_string()]);
    assert_eq!(net.junctions[2].kind, "", "missing keys default");
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn refuses_wrong_format_and_bad_segments() {
    let wrong = SAMPLE.replacen("\"format\": 1", "\"format\": 2", 1);
    let err = RoadNetwork::from_reader(wrong.as_bytes()).unwrap_err();
    assert!(err.to_string().contains("format 2"), "{err}");
    let one_point = r#"{"format":1,"segments":[{"id":3,"class":"trail","width_m":1.5,"a":0,"b":0,"points":[[1,2]]}]}"#;
    assert!(RoadNetwork::from_reader(one_point.as_bytes()).is_err());
    let zero_width = r#"{"format":1,"segments":[{"id":3,"class":"trail","width_m":0,"a":0,"b":0,"points":[[1,2],[3,4]]}]}"#;
    assert!(RoadNetwork::from_reader(zero_width.as_bytes()).is_err());
}

#[test]
fn params_defaults_and_ranges() {
    let p = RibbonParams::default();
    assert_eq!(p.cross_samples, 5);
    let mut d = declared_defaults();
    d.iter_mut().find(|v| v.name == "spacing_m").unwrap().v = 0.0;
    assert!(RibbonParams::from_declared(&d).is_err(), "spacing below its range");
}

#[test]
fn flat_height_gives_flat_ribbon_at_lift() {
    let p = RibbonParams::default();
    let s = seg(vec![[0.0, 0.0], [120.0, 0.0], [240.0, 0.0]], 6.0);
    let r = ribbon(&s, &p, &|_, _| 10.0);
    assert!((r.length_m - 240.0).abs() < 1e-6, "straight input stays straight: {}", r.length_m);
    assert!(r.spans.is_empty());
    assert!(!r.chunks.is_empty());
    for ch in &r.chunks {
        assert_eq!(ch.positions.len(), ch.rows * ch.cols);
        for k in 0..ch.positions.len() {
            let w = world(ch, k);
            assert!((w[2] - (10.0 + p.lift_m)).abs() < 1e-5, "z {}", w[2]);
            assert!(w[1].abs() <= 3.0 + 1e-5, "within half the width");
            let n = ch.normals[k];
            assert!((n[2] - 1.0).abs() < 1e-6 && n[0].abs() < 1e-6 && n[1].abs() < 1e-6, "{n:?}");
        }
        for r in 0..ch.rows {
            let row = r * ch.cols;
            assert_eq!(ch.fade[row], 0.0);
            assert_eq!(ch.fade[row + ch.cols - 1], 0.0);
            assert!(ch.fade[row + 1..row + ch.cols - 1].iter().all(|&f| f == 1.0));
            assert_eq!(ch.uvs[row][0], 0.0);
            assert_eq!(ch.uvs[row + ch.cols - 1][0], 1.0);
            // Left of +x travel is +y: column 0 is the north edge.
            assert!(world(ch, row)[1] > world(ch, row + ch.cols - 1)[1]);
            // The shoulder lines sit `shoulder_m` inside the edges.
            let edge = world(ch, row)[1] - world(ch, row + 1)[1];
            assert!((edge - p.shoulder_m).abs() < 1e-5, "{edge}");
        }
    }
}

#[test]
fn slope_every_vertex_lift_above_its_own_ground() {
    let p = RibbonParams::default();
    let r = ribbon(&winding(), &p, &slope);
    let mut n = 0;
    for ch in &r.chunks {
        for k in 0..ch.positions.len() {
            let w = world(ch, k);
            let dz = w[2] - slope(w[0], w[1]);
            // f32 positions relative to the chunk origin: sub-millimetre.
            assert!((dz - p.lift_m).abs() < 2e-3, "vertex {k}: {dz}");
            let nm = ch.normals[k];
            let len = (nm[0] * nm[0] + nm[1] * nm[1] + nm[2] * nm[2]).sqrt();
            assert!((len - 1.0).abs() < 1e-4 && nm[2] > 0.5, "{nm:?}");
            n += 1;
        }
    }
    assert!(n > 1000, "{n} vertices");
}

#[test]
fn triangles_wind_counter_clockwise_from_above() {
    let r = ribbon(&winding(), &RibbonParams::default(), &slope);
    for ch in &r.chunks {
        assert_eq!(ch.indices.len(), (ch.rows - 1) * (ch.cols - 1) * 6);
        for t in ch.indices.chunks(3) {
            let [a, b, c] = [t[0], t[1], t[2]].map(|i| ch.positions[i as usize]);
            let area = (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]);
            assert!(area > 0.0, "clockwise triangle {t:?}");
        }
    }
}

#[test]
fn chunks_cover_the_whole_length_with_shared_seams() {
    let p = RibbonParams { chunk_m: 100.0, ..RibbonParams::default() };
    let r = ribbon(&winding(), &p, &slope);
    assert!(r.chunks.len() >= 10, "{} chunks over {} m", r.chunks.len(), r.length_m);
    assert_eq!(r.chunks[0].s0_m, 0.0);
    assert_eq!(r.chunks.last().unwrap().s1_m, r.length_m);
    for ch in &r.chunks {
        assert!(ch.s1_m - ch.s0_m <= p.chunk_m + 1e-9);
        let rows_v: Vec<f32> = (0..ch.rows).map(|r| ch.uvs[r * ch.cols][1]).collect();
        let step = (rows_v[1] - rows_v[0]) * p.texture_length_m as f32;
        assert!(step <= p.spacing_m as f32 + 1e-3, "row step {step}");
    }
    for w in r.chunks.windows(2) {
        let (a, b) = (&w[0], &w[1]);
        assert_eq!(a.s1_m, b.s0_m, "no gap in arc length");
        let last = (a.rows - 1) * a.cols;
        for j in 0..a.cols {
            let (pa, pb) = (world(a, last + j), world(b, j));
            for k in 0..3 {
                assert!((pa[k] - pb[k]).abs() < 1e-4, "seam vertex {j}: {pa:?} vs {pb:?}");
            }
            assert_eq!(a.normals[last + j], b.normals[j]);
            assert_eq!(a.uvs[last + j], b.uvs[j]);
            assert_eq!(a.fade[last + j], b.fade[j]);
        }
    }
}

#[test]
fn smoothing_passes_through_the_input_points() {
    let s = winding();
    let line = Centreline::new(&s.points, 2.0).unwrap();
    for &p in &s.points {
        let (q, _) = line.eval(line.project(p));
        let d = (q[0] - p[0]).hypot(q[1] - p[1]);
        assert!(d < 1e-3, "input {p:?} is {d} m off the centreline");
    }
    let poly: f64 = s.points.windows(2).map(|w| (w[1][0] - w[0][0]).hypot(w[1][1] - w[0][1])).sum();
    assert!(line.length() >= poly - 1e-6 && line.length() < poly * 1.1, "{} vs {poly}", line.length());
    // Unit tangents that turn smoothly: no kink between rows 2 m apart.
    let steps = (line.length() / 2.0) as usize;
    let mut prev = line.eval(0.0).1;
    for i in 1..=steps {
        let t = line.eval(i as f64 * 2.0).1;
        assert!((t[0].hypot(t[1]) - 1.0).abs() < 1e-9);
        let turn = (prev[0] * t[1] - prev[1] * t[0]).atan2(prev[0] * t[0] + prev[1] * t[1]);
        assert!(turn.abs() < 0.2, "turn of {turn} rad at {} m", i * 2);
        prev = t;
    }
}

#[test]
fn bridge_spans_are_skipped_and_fords_kept() {
    let net = RoadNetwork::from_reader(SAMPLE.as_bytes()).unwrap();
    let s = &net.segments[1];
    let r = ribbon(s, &RibbonParams::default(), &|_, _| 0.0);
    assert_eq!(r.spans.len(), 1, "one bridge; ford, access and unknown kinds stay on the ground");
    let sp = &r.spans[0];
    assert_eq!(sp.kind, CrossingKind::Bridge);
    assert!((sp.s0_m - 40.0).abs() < 1e-3 && (sp.s1_m - 60.0).abs() < 1e-3, "{sp:?}");
    assert_eq!(sp.points, vec![[240.0, 50.0], [260.0, 50.0]]);
    let mut covered: Vec<(f64, f64)> = r.chunks.iter().map(|c| (c.s0_m, c.s1_m)).collect();
    covered.sort_by(|a, b| a.0.total_cmp(&b.0));
    assert_eq!(covered[0].0, 0.0);
    assert_eq!(covered.last().unwrap().1, r.length_m);
    for ch in &r.chunks {
        assert!(ch.s1_m <= sp.s0_m + 1e-9 || ch.s0_m >= sp.s1_m - 1e-9, "chunk {ch:?} under the bridge");
    }
    assert!(covered.windows(2).any(|w| (w[0].1 - sp.s0_m).abs() < 1e-9 && (w[1].0 - sp.s1_m).abs() < 1e-9));
    // The ford (s 100..110) is on the ground ribbon.
    assert!(r.chunks.iter().any(|c| c.s0_m <= 100.0 && c.s1_m >= 110.0));
}

#[test]
fn ribbon_is_deterministic() {
    let p = RibbonParams::default();
    let a = ribbon(&winding(), &p, &slope);
    let b = ribbon(&winding(), &p, &slope);
    assert_eq!(a, b);
}

#[test]
fn degenerate_segment_gives_empty_ribbon() {
    let r = ribbon(&seg(vec![[5.0, 5.0], [5.0, 5.0]], 3.0), &RibbonParams::default(), &|_, _| 0.0);
    assert!(r.chunks.is_empty() && r.spans.is_empty());
}

/// Prints the ribbon size of a real network: `SF_ROADS_GUIDE=<guide dir> cargo test -p sf_worldgen roads_measure
/// -- --ignored --nocapture`.
#[test]
#[ignore = "measurement: ribbon vertex/chunk counts and build time of a real roads.json"]
fn roads_measure() {
    let Ok(dir) = std::env::var("SF_ROADS_GUIDE") else {
        eprintln!("SF_ROADS_GUIDE unset");
        return;
    };
    let net = RoadNetwork::read(Path::new(&dir)).unwrap().expect("roads.json");
    let p = RibbonParams::default();
    let t = std::time::Instant::now();
    let (mut verts, mut tris, mut chunks, mut len, mut spans) = (0usize, 0usize, 0usize, 0.0, 0usize);
    for s in &net.segments {
        let r = ribbon(s, &p, &|x, y| 0.01 * x + 0.02 * y);
        chunks += r.chunks.len();
        spans += r.spans.len();
        len += r.length_m;
        for c in &r.chunks {
            verts += c.positions.len();
            tris += c.indices.len() / 3;
        }
    }
    let routed: f64 = net.segments.iter().map(|s| s.length_m).sum();
    eprintln!(
        "{} segments, {:.1} km smoothed ({:.1} km routed), {chunks} chunks, {spans} spans, {verts} vertices, {tris} triangles, {:.2} s",
        net.segments.len(),
        len / 1000.0,
        routed / 1000.0,
        t.elapsed().as_secs_f64()
    );
}
