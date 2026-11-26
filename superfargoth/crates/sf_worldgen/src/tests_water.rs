#![allow(clippy::float_cmp)] // exact comparisons are the point of these tests
//! Tests of the `water.json` reader: the join keys are kept as written, and stay optional.

use super::*;

const FILE: &str = r#"{
  "format": 1, "sea_level_m": 2.5, "frame": "guide", "stats": { "rivers": 3 },
  "lakes": [
    { "id": 4, "kind": "open", "level_m": 120.0, "outline": [[0, 0], [100, 0], [100, 100]], "holes": [], "area_m2": 5000.0 },
    { "id": 5, "kind": "closed", "level_m": 80.0, "outline": [[0, 0], [10, 0], [10, 10]], "from_course": 2 },
    { "level_m": 1.0, "outline": [[0, 0], [1, 0], [1, 1]] }
  ],
  "rivers": [
    { "id": 1, "kind": "river", "mu_id": 0, "joins": -1, "outlet": 2225161, "clipped": true,
      "points": [[0, 0, 10, 130], [0, 100, 12, 128]] },
    { "id": 2, "kind": "river", "mu_id": 1, "joins": 0, "clipped": false, "points": [[5, 0, 4, 140], [0, 50, 4, 129]] },
    { "id": 60, "kind": "lake_link", "joins": 1, "clipped": false, "points": [[9, 9, 3, 150], [5, 0, 3, 140]] },
    { "points": [[0, 0, 1, 1], [1, 1, 1, 1]] }
  ]
}"#;

#[test]
fn keeps_the_join_keys() {
    let w = GuideWater::from_reader(FILE.as_bytes()).unwrap();
    assert_eq!(w.sea_level_m, 2.5);
    assert_eq!(w.lakes.len(), 3);
    assert_eq!((w.lakes[0].id, w.lakes[0].kind.as_str(), w.lakes[0].from_course), (4, "open", None));
    assert_eq!((w.lakes[1].id, w.lakes[1].kind.as_str(), w.lakes[1].from_course), (5, "closed", Some(2)));
    assert_eq!(w.lakes[2], Lake { level_m: 1.0, outline: vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0]], ..Lake::default() });

    let keys = |r: &River| (r.id, r.kind.clone(), r.mu_id, r.joins, r.outlet, r.clipped);
    assert_eq!(keys(&w.rivers[0]), (1, "river".into(), Some(0), Some(-1), Some(2_225_161), true));
    assert_eq!(keys(&w.rivers[1]), (2, "river".into(), Some(1), Some(0), None, false));
    assert_eq!(keys(&w.rivers[2]), (60, "lake_link".into(), None, Some(1), None, false));
    // A river with none of the keys (an older file) still reads.
    assert_eq!(
        w.rivers[3],
        River { points: vec![[0.0, 0.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0]], ..River::default() }
    );
    assert_eq!(w.rivers[0].points[1], [0.0, 100.0, 12.0, 128.0]);
}

#[test]
fn refuses_another_format() {
    let err = GuideWater::from_reader(r#"{ "format": 2 }"#.as_bytes()).unwrap_err().to_string();
    assert!(err.contains("water format 2"), "{err}");
}
