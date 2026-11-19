//! WG-X2 tests: classification, clustering, the anchor transform and the stamps, on fixtures (no game data).

// Grid maths: sample counts stay far below 2^31 and heights within f32 range (as in sf_worldgen).
#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::many_single_char_names,
    // Exact results are the point of these tests.
    clippy::float_cmp
)]

use sf_worldgen::anchors::AnchorTransform;
use crate::anchor::{self, Field, Member, Params};
use crate::classify::{Class, RefInfo, classify};
use crate::x10::{build_anchors, overlap_samples, scale_heights, stamp_set};
use crate::{MarkersFile, markers::MARKERS_FORMAT};

fn reference(kind: &str, model: Option<&str>, extent_m: f32) -> RefInfo {
    let h = extent_m / 2.0;
    RefInfo {
        key: format!("{kind}-{model:?}"),
        kind: kind.into(),
        base: "base".into(),
        model: model.map(str::to_owned),
        pos: [0.0, 0.0, 10.0],
        scale: 1.0,
        bounds: Some(([-h, -h, -h], [h, h, h])),
    }
}

#[test]
fn classification_of_a_fixture() {
    let p = Params::default();
    let cases = [
        (reference("STAT", Some("meshes/f/flora_tree_ac_01.nif"), 8.0), Class::Nature),
        (reference("STAT", Some("meshes/x/terrain_rock_ac_01.nif"), 30.0), Class::Nature),
        (reference("STAT", Some("meshes/x/ex_ashl_rock_03.nif"), 20.0), Class::Nature),
        (reference("STAT", Some("meshes/x/ex_ashl_rock_03.nif"), 3.0), Class::Other),
        (reference("DOOR", Some("meshes/d/ex_h_door_01.nif"), 3.0), Class::HumanMade),
        (reference("STAT", Some("meshes/x/ex_hlaalu_b_01.nif"), 15.0), Class::HumanMade),
        (reference("LIGH", Some("meshes/l/light_com_lantern_01.nif"), 0.5), Class::HumanMade),
        (reference("CREA", Some("meshes/r/mudcrab.nif"), 1.0), Class::Other),
        (reference("STAT", Some("meshes/x/ex_gg_ghostfence_01.nif"), 10.0), Class::Other),
    ];
    for (r, want) in cases {
        assert_eq!(classify(&r, p.rock_min_m).0, want, "{:?}", r.model);
    }
    let marker = RefInfo { base: "NorthMarker".into(), ..reference("STAT", Some("meshes/marker_north.nif"), 1.0) };
    assert_eq!(classify(&marker, p.rock_min_m).0, Class::Other);
}

fn member(key: &str, x: f64, y: f64, r: f64) -> Member {
    Member { key: key.into(), xy: [x, y], r, on_sea: false, group: String::new() }
}

/// Two towns far apart, a lone hut, and a chain of houses that link.
fn fixture() -> Vec<Member> {
    let mut v = Vec::new();
    for i in 0..10 {
        v.push(member(&format!("a{i:02}"), 200.0 + 15.0 * f64::from(i % 4), 300.0 + 15.0 * f64::from(i / 4), 4.0));
    }
    for i in 0..5 {
        v.push(member(&format!("b{i:02}"), 700.0 + 12.0 * f64::from(i), 900.0, 3.0));
    }
    v.push(member("c00", 1200.0, 400.0, 2.0));
    for i in 0..6 {
        v.push(member(&format!("d{i:02}"), 400.0 + 25.0 * f64::from(i), 1300.0, 3.0));
    }
    v
}

fn key_sets(members: &[Member], clusters: &[Vec<usize>]) -> Vec<Vec<String>> {
    let mut s: Vec<Vec<String>> =
        clusters.iter().map(|g| g.iter().map(|&i| members[i].key.clone()).collect()).collect();
    s.sort();
    s
}

#[test]
fn clustering_is_deterministic_and_order_independent() {
    let p = Params::default();
    let a = fixture();
    let ca = anchor::cluster(&a, &p, 10.0, 1.8288);
    assert_eq!(ca.len(), 4, "{:?}", key_sets(&a, &ca));
    let mut b = a.clone();
    b.reverse();
    b.rotate_left(7);
    let cb = anchor::cluster(&b, &p, 10.0, 1.8288);
    assert_eq!(key_sets(&a, &ca), key_sets(&b, &cb));
    assert_eq!(anchor::cluster(&a, &p, 10.0, 1.8288), ca);
}

#[test]
fn clusters_whose_bands_touch_merge() {
    let p = Params { band_min_m: 60.0, band_max_m: 60.0, point_max_radius_m: 0.0, ..Params::default() };
    // Two groups 100 m apart: separate at 10× (their world gap is ~1 km), one anchor at 1:1 (bands overlap).
    let v: Vec<Member> = (0..3)
        .flat_map(|i| {
            let x = 5.0 * f64::from(i);
            [member(&format!("a{i}"), x, 0.0, 2.0), member(&format!("b{i}"), 110.0 + x, 0.0, 2.0)]
        })
        .collect();
    assert_eq!(anchor::cluster(&v, &p, 10.0, 1.8288).len(), 2);
    assert_eq!(anchor::cluster(&v, &p, 1.0, 1.8288).len(), 1);
}

const SP: f64 = 1.8288;

/// A sloped source terrain (source frame) and its plain 10× scale.
fn fields() -> (Vec<f32>, Vec<f32>, usize, [f64; 2]) {
    let (w, origin) = (900usize, [0.0, 1645.0]);
    let src: Vec<f32> = (0..w * w)
        .map(|k| {
            let (c, r) = ((k % w) as f64, (k / w) as f64);
            (5.0 + 0.02 * c * SP + 0.01 * r * SP + 3.0 * (c * 0.01).sin()) as f32
        })
        .collect();
    let fin = scale_heights(&src, 10.0);
    (src, fin, w, origin)
}

fn markers() -> MarkersFile {
    MarkersFile { format: MARKERS_FORMAT, frame: String::new(), cell_size_m: 117.0, summit: None, markers: vec![] }
}

#[test]
fn stamps_of_distinct_anchors_never_overlap() {
    let (src_h, fin_h, w, origin) = fields();
    let src = Field { w, h: w, origin_m: origin, spacing_m: SP, data: &src_h };
    let fin = Field { w, h: w, origin_m: [origin[0] * 10.0, origin[1] * 10.0], spacing_m: SP * 10.0, data: &fin_h };
    let built = build_anchors(&fixture(), &src, &fin, 10.0, &Params::default(), &markers());
    assert_eq!(built.len(), 4);
    // The lone hut is a point anchor: no stamp; the stamps are numbered in order.
    let set = stamp_set(&built);
    assert_eq!(set.stamps.len(), 3);
    let indices: Vec<Option<usize>> = built.iter().map(|b| b.place.stamp).collect();
    assert_eq!(indices, [Some(0), Some(1), Some(2), None]);
    assert!(built[3].stats.point);
    assert_eq!(overlap_samples(&set), 0);
    let names: Vec<&str> = built.iter().map(|b| b.place.name.as_str()).collect();
    assert_eq!(names[..2], ["anchor", "anchor 2"]);
}

#[test]
fn anchor_transform_moves_members_rigidly() {
    let (src_h, fin_h, w, origin) = fields();
    let src = Field { w, h: w, origin_m: origin, spacing_m: SP, data: &src_h };
    let fin = Field { w, h: w, origin_m: [origin[0] * 10.0, origin[1] * 10.0], spacing_m: SP * 10.0, data: &fin_h };
    let members = fixture();
    let built = build_anchors(&members, &src, &fin, 10.0, &Params::default(), &markers());
    let t = AnchorTransform::new(10.0, built.iter().map(|b| b.place.clone()).collect());
    for m in &members {
        let i = t.anchor_of(m.xy).expect("a member lies in its anchor's footprint");
        let a = &t.anchors[i];
        assert!(a.members.contains(&m.key));
        let c = a.source_centre_m;
        let (wp, h) = t.source_to_world(m.xy, 12.0);
        assert_eq!(wp, [10.0 * c[0] + (m.xy[0] - c[0]), 10.0 * c[1] + (m.xy[1] - c[1])]);
        assert_eq!(h, 12.0 + a.lift_m);
        // A sloped plane scaled 10×: the lift is about 9× the ground height at the centre.
        assert!(a.lift_m > 8.0 * src.bilinear(c[0], c[1]) && a.lift_m < 10.0 * src.bilinear(c[0], c[1]), "{}", a.lift_m);
    }
    let dir = tempfile::tempdir().unwrap();
    t.write(dir.path()).unwrap();
    assert_eq!(AnchorTransform::read(dir.path()).unwrap(), Some(t));
}

#[test]
fn stamp_weights_and_heights() {
    let (src_h, fin_h, w, origin) = fields();
    let src = Field { w, h: w, origin_m: origin, spacing_m: SP, data: &src_h };
    let fin = Field { w, h: w, origin_m: [origin[0] * 10.0, origin[1] * 10.0], spacing_m: SP * 10.0, data: &fin_h };
    let p = Params::default();
    let town: Vec<Member> = fixture().into_iter().filter(|m| m.key.starts_with('a')).collect();
    let refs: Vec<&Member> = town.iter().collect();
    let b = anchor::build_anchor("town".into(), &refs, &src, &fin, 10.0, &p);
    let s = b.stamp.as_ref().expect("a town has a stamp");
    let band = b.stats.band_m;
    assert!(band >= p.band_min_m && band <= p.band_max_m);
    let [sw, sh] = s.dims;
    let off = [b.place.world_centre_m[0] - b.place.source_centre_m[0], b.place.world_centre_m[1] - b.place.source_centre_m[1]];
    let source_point = |k: usize| [s.origin_m[0] + (k % sw) as f64 * SP - off[0], s.origin_m[1] - (k / sw) as f64 * SP - off[1]];
    let mut footprint = 0;
    for k in 0..sw * sh {
        let q = source_point(k);
        let d = b.place.footprint.distance(q);
        let signed = b.place.footprint.discs.iter().map(|&[x, y, r]| (q[0] - x).hypot(q[1] - y) - r).fold(f64::INFINITY, f64::min);
        let wk = f64::from(s.weights[k]);
        if signed.abs() < 1e-6 {
            continue; // on a disc edge: float noise decides
        }
        if signed < 0.0 {
            footprint += 1;
            assert_eq!(wk, 1.0);
            let want = src.bilinear(q[0], q[1]) + b.place.lift_m;
            assert!((f64::from(s.heights[k]) - want).abs() < 1e-3, "{} vs {want}", s.heights[k]);
        } else if d >= band {
            assert_eq!(wk, 0.0, "beyond the band at {d} m");
        } else if d <= SP {
            // Flat start: within two samples of the footprint the weight has fallen by at most 10 t³ (t = 2 sp / band).
            assert!(wk >= 1.0 - 10.0 * (2.0 * SP / band).powi(3), "C1 at the footprint edge: {wk} at {d} m");
        } else if d >= band - SP {
            assert!(wk <= 10.0 * (SP / band).powi(3), "C1 at the band edge: {wk} at {d} m");
        }
    }
    assert!(footprint > 100);
    assert!(b.stats.footprint_samples >= footprint);
    // No jump: neighbour steps stay under the smootherstep's steepest slope (15/8 per band width).
    let max_step = 1.875 * SP / band * 1.01;
    for r in 0..sh {
        for c in 0..sw - 1 {
            let k = r * sw + c;
            assert!(f64::from((s.weights[k] - s.weights[k + 1]).abs()) <= max_step);
            if r + 1 < sh {
                assert!(f64::from((s.weights[k] - s.weights[k + sw]).abs()) <= max_step);
            }
        }
    }
    // A member on sea-connected water keeps the sea: L = 0.
    let mut docked = town.clone();
    docked[0].on_sea = true;
    let refs: Vec<&Member> = docked.iter().collect();
    assert_eq!(anchor::build_anchor("dock".into(), &refs, &src, &fin, 10.0, &p).place.lift_m, 0.0);
}

#[test]
fn scale_is_an_f32_multiply() {
    let h = [0.0_f32, 1.5, -29.2608, 270.8123, 3.3e-3, -0.0];
    let s = scale_heights(&h, 10.0);
    for (a, b) in h.iter().zip(&s) {
        assert_eq!((a * 10.0).to_bits(), b.to_bits());
    }
}

#[test]
fn sea_connectivity_ignores_inland_depressions() {
    // 5x5: a wet border column on the west, an inland wet sample in the middle.
    let mut h = vec![10.0_f32; 25];
    for r in 0..5 {
        h[r * 5] = -5.0;
    }
    h[12] = -1.0;
    let f = Field { w: 5, h: 5, origin_m: [0.0, 0.0], spacing_m: 1.0, data: &h };
    let sea = anchor::sea_connected(&f);
    assert!(sea[10] && !sea[12] && !sea[11]);
}

#[test]
fn open_sea_leaves_out_narrow_rivers() {
    // 60x40 at 1 m: sea over the west 20 columns, a 3 m river running east from it along row 20.
    let (w, h) = (60usize, 40usize);
    let mut d = vec![10.0_f32; w * h];
    for r in 0..h {
        for c in 0..20 {
            d[r * w + c] = -5.0;
        }
    }
    for r in 19..22 {
        for c in 20..w - 5 {
            d[r * w + c] = -1.0;
        }
    }
    let f = Field { w, h, origin_m: [0.0, 0.0], spacing_m: 1.0, data: &d };
    let open = anchor::open_sea(&f, 4.0);
    assert!(open[10 * w + 5] && open[20 * w + 19], "the sea and its shore are open");
    assert!(!open[20 * w + 40], "a 3 m river is not");
    assert!(anchor::sea_connected(&f)[20 * w + 40]);
}

#[test]
fn harvestable_plants_are_nature() {
    let r = RefInfo { base: "flora_muckspunge_01".into(), ..reference("CONT", Some("meshes/o/flora_muckspunge_01.nif"), 1.0) };
    assert_eq!(classify(&r, 14.0).0, Class::Nature);
    let fence = reference("STAT", Some("meshes/x/ex_gg_fence_01.nif"), 20.0);
    assert_eq!(classify(&fence, 14.0).0, Class::Other);
}
