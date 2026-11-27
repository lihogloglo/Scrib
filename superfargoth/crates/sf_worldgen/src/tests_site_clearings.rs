//! Tests of the clearing query (WG-V1b.1): exact in/out at both margins of a ring site, a disc site and a straight
//! road; dropped sites and kept towns give no clearing; the bucketed query equals one bucket (brute force) over a
//! grid of points, asked in two orders and with the input shapes in two orders.

use super::*;
use crate::sites::SitesFile;

/// A ring site (20 m square round (100, 100), clearing 12), a disc site (r 5 at (300, 100), clearing 10), a dropped
/// site and a kept town on top of the road's far end, all format 2.
fn sites() -> Vec<Site> {
    let json = serde_json::json!({ "format": 2, "seed": 1, "guide_blake3": "g", "table_blake3": "t",
        "packs_blake3": "p", "sites": [
        { "key": "ring", "centre_m": [100.0, 100.0], "kind": "remote", "kept": false, "status": "placed",
          "clearing_m": 12.0, "footprint": [[90.0, 90.0], [110.0, 90.0], [110.0, 110.0], [90.0, 110.0]] },
        { "key": "disc", "centre_m": [300.0, 100.0], "radius_m": 5.0, "kind": "hamlet", "kept": false,
          "status": "reseated", "clearing_m": 10.0 },
        { "key": "gone", "centre_m": [500.0, 100.0], "radius_m": 5.0, "kind": "remote", "kept": false,
          "status": "dropped", "clearing_m": 10.0 },
        { "key": "town", "centre_m": [700.0, 300.0], "radius_m": 200.0, "kind": "town", "kept": true }
    ]});
    SitesFile::from_reader(json.to_string().as_bytes()).unwrap().sites
}

/// One straight 4 m road from (0, -100) to (1000, -100): cleared to 2 + 6 = 8 m from its centreline.
fn roads() -> RoadNetwork {
    RoadNetwork::from_reader(
        r#"{ "format": 1, "segments": [{ "id": 1, "class": "local", "surface": "dirt", "width_m": 4.0, "a": 0,
              "b": 1, "points": [[0.0, -100.0], [400.0, -100.0], [1000.0, -100.0]], "crossings": [],
              "length_m": 1000.0, "grade": 0.0 }], "junctions": [] }"#
            .as_bytes(),
    )
    .unwrap()
}

#[test]
fn margins_are_exact() {
    let (sites, roads) = (sites(), roads());
    let c = Clearings::new(&sites, Some(&roads), &ClearingValues::default());
    assert_eq!((c.sites, c.roads), (2, 1));
    let cases: &[([f64; 2], bool, &str)] = &[
        ([100.0, 100.0], true, "ring centre"),
        ([121.9, 100.0], true, "ring, 11.9 m beyond the east edge"),
        ([122.1, 100.0], false, "ring, 12.1 m beyond the east edge"),
        ([100.0, 77.9], false, "ring, 12.1 m beyond the south edge"),
        ([100.0, 78.1], true, "ring, 11.9 m beyond the south edge"),
        ([118.0, 118.0], true, "ring corner, 11.3 m"),
        ([119.0, 119.0], false, "ring corner, 12.7 m"),
        ([314.9, 100.0], true, "disc, 14.9 m"),
        ([300.0, 115.1], false, "disc, 15.1 m"),
        ([500.0, 100.0], false, "dropped site"),
        ([700.0, 300.0], false, "kept town"),
        ([500.0, -92.1], true, "road, 7.9 m"),
        ([500.0, -91.9], false, "road, 8.1 m"),
        ([400.0, -107.9], true, "road at a knot, 7.9 m"),
        ([400.0, -108.1], false, "road at a knot, 8.1 m"),
        ([1007.9, -100.0], true, "road end cap, 7.9 m"),
        ([1008.1, -100.0], false, "road end cap, 8.1 m"),
    ];
    for &(p, want, what) in cases {
        assert_eq!(c.contains(p[0], p[1]), want, "{what} at {p:?}");
    }
}

#[test]
fn no_roads_and_zero_margins() {
    let sites = sites();
    let v = ClearingValues { road_m: 0.0, ..ClearingValues::default() };
    let c = Clearings::new(&sites, None, &v);
    assert_eq!((c.sites, c.roads), (2, 0));
    assert!(!c.contains(500.0, -100.0), "no road given");
    let c = Clearings::new(&[], Some(&roads()), &v);
    assert!(c.contains(500.0, -98.1) && !c.contains(500.0, -97.9), "road_m 0 clears the ribbon only");
    assert!(Clearings::new(&[], None, &v).is_empty());
}

#[test]
fn bucketed_equals_brute_force_in_any_order() {
    let (sites, roads) = (sites(), roads());
    let v = ClearingValues::default();
    let fast = Clearings::new(&sites, Some(&roads), &v);
    let brute = Clearings::with_bucket(&sites, Some(&roads), &v, f64::INFINITY);
    let mut rev = sites.clone();
    rev.reverse();
    let reordered = Clearings::with_bucket(&rev, Some(&roads), &v, 37.0);
    let points: Vec<[f64; 2]> =
        (0..=220).flat_map(|j| (0..=330).map(move |i| [-20.0 + f64::from(i) * 3.1, -140.0 + f64::from(j) * 2.3])).collect();
    let forward: Vec<bool> = points.iter().map(|p| fast.contains(p[0], p[1])).collect();
    let backward: Vec<bool> = points.iter().rev().map(|p| fast.contains(p[0], p[1])).collect::<Vec<_>>().into_iter().rev().collect();
    let want: Vec<bool> = points.iter().map(|p| brute.contains(p[0], p[1])).collect();
    let other: Vec<bool> = points.iter().map(|p| reordered.contains(p[0], p[1])).collect();
    assert_eq!(forward, want);
    assert_eq!(backward, want);
    assert_eq!(other, want);
    let hits = want.iter().filter(|&&b| b).count();
    assert!(hits > 1000 && hits < points.len() / 2, "the grid crosses every shape ({hits} hits)");
}
