//! Tests for [`crate::joins`]: the numbers are hand-checked (FL-4a brief).

use glam::DVec2;

use crate::joins::{Join, JoinEnd, JoinLake, JoinRiver, JoinSettings, JoinStart, TUNABLES, resolve};
use crate::records::RiverPoint;

fn pt(x: f64, y: f64, width: f32, height: f64, flow_speed: f32) -> RiverPoint {
    RiverPoint { center: DVec2::new(x, y), width, height, flow_speed, river_ness: 1.0 }
}

fn assert_f32(got: &[f32], want: &[f32]) {
    assert_eq!(got.len(), want.len(), "{got:?} vs {want:?}");
    for (g, w) in got.iter().zip(want) {
        assert!((g - w).abs() <= 1e-6, "{got:?} vs {want:?}");
    }
}

fn ness(r: &JoinRiver) -> Vec<f32> {
    r.spline.iter().map(|p| p.river_ness).collect()
}

fn flow(r: &JoinRiver) -> Vec<f32> {
    r.spline.iter().map(|p| p.flow_speed).collect()
}

fn open() -> Join {
    Join { start: JoinStart::Open, end: JoinEnd::Open }
}

fn receiver() -> JoinRiver {
    JoinRiver {
        spline: vec![pt(0.0, 0.0, 10.0, 10.0, 1.0), pt(50.0, 0.0, 10.0, 9.0, 1.0), pt(100.0, 0.0, 10.0, 8.0, 1.0)],
        receivers: vec![],
    }
}

fn tributary(last_height: f64, receiver: usize) -> JoinRiver {
    let c = [(30.0, 40.0), (30.0, 30.0), (30.0, 20.0), (30.0, 10.0), (30.0, 4.0), (31.0, 1.0)];
    let h = [12.0, 11.0, 10.5, 10.0, 9.6, last_height];
    JoinRiver {
        spline: c.iter().zip(h).map(|(&(x, y), h)| pt(x, y, 4.0, h, 1.5)).collect(),
        receivers: vec![receiver],
    }
}

/// Lake outline (0,0)-(100,100), the given level, no hole.
fn square_lake(level: f64) -> JoinLake {
    JoinLake {
        outline: vec![DVec2::new(0.0, 0.0), DVec2::new(100.0, 0.0), DVec2::new(100.0, 100.0), DVec2::new(0.0, 100.0)],
        holes: vec![],
        level,
    }
}

/// Checks the Y confluence's tributary (`t`) after the join.
fn assert_tributary(t: &JoinRiver, last_height: f64) {
    let last = t.spline[5];
    assert_eq!(last.center, DVec2::new(31.0, 0.0));
    assert!((last.height - last_height).abs() <= 1e-9, "{}", last.height);
    // Run = points 4 and 5; Z = min(10, sqrt(17)).
    assert_f32(&ness(t), &[1.0, 1.0, 1.0, 1.0, 1.0, 0.0]);
    assert_f32(&flow(t), &[1.5, 1.5, 1.5, 1.5, 1.5, 0.0]);
    // Points before the last: untouched (a factor of exactly 1).
    assert_eq!(t.spline[..5], tributary(last_height, 0).spline[..5]);
}

#[test]
fn y_confluence_snaps_the_tributary() {
    let s = JoinSettings::default();
    let mut rivers = vec![receiver(), tributary(9.5, 0)];
    let joins = resolve(&mut rivers, &[], -100.0, &s);
    assert_eq!(joins[0], open());
    assert_eq!(joins[1], Join { start: JoinStart::Open, end: JoinEnd::Tributary { receiver: 0, moved_m: 1.0 } });
    assert_eq!(rivers[0], receiver());
    assert_tributary(&rivers[1], 10.0 - 31.0 / 50.0);
    assert!((rivers[1].spline[5].height - 9.38).abs() <= 1e-9);
}

#[test]
fn y_confluence_keeps_a_lower_own_height() {
    let s = JoinSettings::default();
    let mut rivers = vec![receiver(), tributary(9.0, 0)];
    let joins = resolve(&mut rivers, &[], -100.0, &s);
    assert_eq!(joins[1].end, JoinEnd::Tributary { receiver: 0, moved_m: 1.0 });
    assert_tributary(&rivers[1], 9.0);
}

#[test]
fn y_confluence_does_not_depend_on_river_order() {
    let s = JoinSettings::default();
    let mut ab = vec![receiver(), tributary(9.5, 0)];
    let mut ba = vec![tributary(9.5, 1), receiver()];
    resolve(&mut ab, &[], -100.0, &s);
    let joins = resolve(&mut ba, &[], -100.0, &s);
    assert_eq!(joins[0].end, JoinEnd::Tributary { receiver: 1, moved_m: 1.0 });
    assert_eq!(joins[1], open());
    assert_eq!(ab[0].spline, ba[1].spline);
    assert_eq!(ab[1].spline, ba[0].spline);
}

#[test]
fn far_receiver_is_unsnapped() {
    let s = JoinSettings { snap_max_m: 0.5, ..JoinSettings::default() };
    let mut rivers = vec![receiver(), tributary(9.5, 0)];
    let joins = resolve(&mut rivers, &[], -100.0, &s);
    assert_eq!(joins[1].end, JoinEnd::Unsnapped { receiver: 0, distance_m: 1.0 });
    assert_eq!(rivers[1], tributary(9.5, 0));
    assert_eq!(rivers[0], receiver());
}

/// 11 points at y = 50, x = -55..45, heights 5 + 0.1 * max(0, 6 - i).
fn mouth_river() -> JoinRiver {
    JoinRiver {
        spline: (0..11)
            .map(|i| {
                let x = -55.0 + 10.0 * f64::from(i);
                pt(x, 50.0, 8.0, 5.0 + 0.1 * f64::from((6 - i).max(0)), 2.0)
            })
            .collect(),
        receivers: vec![],
    }
}

#[test]
fn lake_mouth_fades_over_the_lake() {
    let mut rivers = vec![mouth_river()];
    let joins = resolve(&mut rivers, &[square_lake(5.0)], -100.0, &JoinSettings::default());
    assert_eq!(joins[0], Join { start: JoinStart::Open, end: JoinEnd::Lake(0) });
    let n = ness(&rivers[0]);
    assert_f32(&n, &[1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.5, 0.0, 0.0, 0.0]);
    assert_f32(&flow(&rivers[0]), &[2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 1.0, 0.0, 0.0, 0.0]);
    assert!(n.windows(2).all(|w| w[1] <= w[0]), "river_ness rises downstream: {n:?}");
}

#[test]
fn lake_above_the_river_is_no_mouth() {
    let mut rivers = vec![mouth_river()];
    let joins = resolve(&mut rivers, &[square_lake(9.0)], -100.0, &JoinSettings::default());
    assert_eq!(joins[0], open());
    assert_eq!(rivers[0], mouth_river());
}

#[test]
fn lake_inlet_fades_in_from_the_lake() {
    let h = [5.0, 5.0, 5.0, 5.0, 4.9, 4.8, 4.7];
    let mut rivers = vec![JoinRiver {
        spline: (0..7_u32).map(|i| pt(65.0 + 10.0 * f64::from(i), 50.0, 8.0, h[i as usize], 1.0)).collect(),
        receivers: vec![],
    }];
    let joins = resolve(&mut rivers, &[square_lake(5.0)], -100.0, &JoinSettings::default());
    assert_eq!(joins[0], Join { start: JoinStart::Lake(0), end: JoinEnd::Open });
    let n = ness(&rivers[0]);
    assert_f32(&n, &[0.0, 0.0, 0.5, 1.0, 1.0, 1.0, 1.0]);
    assert!(n.windows(2).all(|w| w[1] >= w[0]), "river_ness falls downstream: {n:?}");
}

#[test]
fn sea_mouth_fades_below_the_sea_tolerance() {
    let h = [3.0, 2.0, 1.0, 0.4, 0.2, 0.0, 0.0];
    let mut rivers = vec![JoinRiver {
        spline: (0..7_u32).map(|i| pt(10.0 * f64::from(i), 0.0, 6.0, h[i as usize], 1.0)).collect(),
        receivers: vec![],
    }];
    let joins = resolve(&mut rivers, &[], 0.0, &JoinSettings::default());
    assert_eq!(joins[0], Join { start: JoinStart::Open, end: JoinEnd::Sea });
    assert_f32(&ness(&rivers[0]), &[1.0, 1.0, 1.0, 1.0, 7.0 / 27.0, 0.0, 0.0]);
}

#[test]
fn short_rivers_and_bad_receivers_are_open() {
    let one = JoinRiver { spline: vec![pt(0.0, 0.0, 4.0, 1.0, 1.0)], receivers: vec![1] };
    let selfish = JoinRiver { spline: vec![pt(0.0, 5.0, 4.0, 1.0, 1.0), pt(0.0, 6.0, 4.0, 1.0, 1.0)], receivers: vec![1, 7] };
    let mut rivers = vec![one.clone(), selfish.clone()];
    let joins = resolve(&mut rivers, &[], -100.0, &JoinSettings::default());
    assert_eq!(joins, vec![open(), open()]);
    assert_eq!(rivers, vec![one, selfish]);
}

#[test]
fn river_point_without_river_ness_reads_full() {
    let p: RiverPoint =
        serde_json::from_str(r#"{"center":[1.0,2.0],"width":4.0,"height":3.0,"flow_speed":1.5}"#).unwrap();
    assert_eq!(p, pt(1.0, 2.0, 4.0, 3.0, 1.5));
}

#[test]
fn default_settings_read_the_tunables() {
    assert_eq!(JoinSettings::default(), JoinSettings { mix_widths: 2.5, snap_max_m: 20.0, level_tol_m: 0.5 });
    assert_eq!(TUNABLES.len(), 3);
}

/// FL-4c: the fade follows the course across a lake, not the record. River A ends 4 m inside the lake's
/// edge; link L starts at that point and lies wholly in the lake (88 m); river B starts at L's end, 4 m
/// before it leaves the lake. Width 8, so the zone is 2.5 x 8 = 20 m: A's fade-out runs on along L
/// (arcs 0, 4 | 4, 12, 20: 1, 0.896 | 0.896, 0.352, 0), and B's fade-in starts on L's end (arcs 4 | 4, 0:
/// 0.896 | 0.896, 1). The values meet at both seams; the flow scales alike. L alone is a stray piece: 0.
#[test]
fn a_lake_link_carries_its_inflow_and_outflow_zones() {
    let at = |xs: &[f64], hs: &[f64]| JoinRiver { spline: xs.iter().zip(hs).map(|(&x, &h)| pt(x, 50.0, 8.0, h, 2.0)).collect(), receivers: vec![] };
    let a = at(&[-30.0, -10.0, 2.0, 6.0], &[6.0, 5.6, 5.0, 5.0]);
    let l = at(&[6.0, 14.0, 22.0, 30.0, 70.0, 94.0], &[5.0; 6]);
    let b = at(&[94.0, 98.0, 110.0, 130.0], &[5.0, 5.0, 4.4, 4.0]);
    let lake = [square_lake(5.0)];
    // The order of the records does not matter.
    for order in [[0, 1, 2], [2, 1, 0], [1, 2, 0]] {
        let all = [a.clone(), l.clone(), b.clone()];
        let mut rivers: Vec<JoinRiver> = order.iter().map(|&i| all[i].clone()).collect();
        let joins = resolve(&mut rivers, &lake, -100.0, &JoinSettings::default());
        let of = |i: usize| order.iter().position(|&o| o == i).expect("in the order");
        assert_eq!(joins[of(0)], Join { start: JoinStart::Open, end: JoinEnd::Lake(0) });
        assert_eq!(joins[of(1)], Join { start: JoinStart::Lake(0), end: JoinEnd::Lake(0) });
        assert_eq!(joins[of(2)], Join { start: JoinStart::Lake(0), end: JoinEnd::Open });
        assert_f32(&ness(&rivers[of(0)]), &[1.0, 1.0, 1.0, 0.896]);
        assert_f32(&ness(&rivers[of(1)]), &[0.896, 0.352, 0.0, 0.0, 0.0, 0.896]);
        assert_f32(&ness(&rivers[of(2)]), &[0.896, 1.0, 1.0, 1.0]);
        assert_f32(&flow(&rivers[of(1)]), &[1.792, 0.704, 0.0, 0.0, 0.0, 1.792]);
    }
    // A link 5 m off the mouth's last point (more than the 4 m half width) does not carry it on.
    let far = at(&[6.0, 14.0, 22.0], &[5.0; 3]);
    let mut rivers = vec![a.clone(), JoinRiver { spline: far.spline.iter().map(|p| RiverPoint { center: p.center + DVec2::new(0.0, 5.0), ..*p }).collect(), receivers: vec![] }];
    resolve(&mut rivers, &lake, -100.0, &JoinSettings::default());
    assert_f32(&ness(&rivers[0]), &[1.0, 1.0, 1.0, 0.0]);
    assert_f32(&ness(&rivers[1]), &[0.0, 0.0, 0.0]);
    // A stray piece in the lake shows the lake.
    let mut rivers = vec![l.clone()];
    resolve(&mut rivers, &lake, -100.0, &JoinSettings::default());
    assert_f32(&ness(&rivers[0]), &[0.0; 6]);
}
