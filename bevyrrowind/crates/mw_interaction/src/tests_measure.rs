//! `measure` on synthetic triangle soups (boxes): exact facings, footprints and fallbacks.

use std::f32::consts::FRAC_PI_2;

use glam::{Quat, Vec3};
use sf_interaction::Side;
use sf_records::RecordId;

use crate::defs::{Basis, Facing, Measured, def_for};
use crate::furniture::Kind;
use crate::measure::measure;
use crate::tunables::{FurnitureParams, MeasureParams};

const EPS: f32 = 1.0e-4;

type Soup = Vec<[Vec3; 3]>;

/// The 12 triangles of an axis-aligned box.
fn cube(lo: [f32; 3], hi: [f32; 3]) -> Soup {
    let corner = |ix: usize, iy: usize, iz: usize| Vec3::new([lo[0], hi[0]][ix], [lo[1], hi[1]][iy], [lo[2], hi[2]][iz]);
    let quads = [
        [corner(0, 0, 0), corner(1, 0, 0), corner(1, 1, 0), corner(0, 1, 0)],
        [corner(0, 0, 1), corner(1, 0, 1), corner(1, 1, 1), corner(0, 1, 1)],
        [corner(0, 0, 0), corner(0, 0, 1), corner(0, 1, 1), corner(0, 1, 0)],
        [corner(1, 0, 0), corner(1, 0, 1), corner(1, 1, 1), corner(1, 1, 0)],
        [corner(0, 0, 0), corner(1, 0, 0), corner(1, 0, 1), corner(0, 0, 1)],
        [corner(0, 1, 0), corner(1, 1, 0), corner(1, 1, 1), corner(0, 1, 1)],
    ];
    quads.iter().flat_map(|&[p0, p1, p2, p3]| [[p0, p1, p2], [p0, p2, p3]]).collect()
}

fn soup(boxes: &[([f32; 3], [f32; 3])]) -> Soup {
    boxes.iter().flat_map(|&(lo, hi)| cube(lo, hi)).collect()
}

/// The soup turned `quarters` quarter turns about +Y.
fn turned(shape: &Soup, quarters: u8) -> Soup {
    let turn = Quat::from_rotation_y(FRAC_PI_2 * f32::from(quarters));
    shape.iter().map(|tri| tri.map(|p| turn * p)).collect()
}

/// `f` turned `quarters` quarter turns.
fn turned_facing(f: Facing, quarters: u8) -> Facing {
    let v = Quat::from_rotation_y(FRAC_PI_2 * f32::from(quarters)) * f.dir();
    [Facing::PosX, Facing::NegX, Facing::PosZ, Facing::NegZ]
        .into_iter()
        .find(|c| c.dir().dot(v) > 0.9)
        .expect("axis-aligned")
}

fn measured(kind: Kind, shape: &Soup, top: f32) -> Measured {
    measure(kind, shape, top, &MeasureParams::default()).expect("measured")
}

fn near2(a: [f32; 2], b: [f32; 2]) -> bool {
    (a[0] - b[0]).abs() < EPS && (a[1] - b[1]).abs() < EPS
}

/// A 0.5 m chair, seat top 0.45 m, backrest at -Z (to 1.0 m), arms on both sides (to 0.7 m).
fn chair() -> Soup {
    soup(&[
        ([-0.25, 0.0, -0.25], [0.25, 0.45, 0.25]),
        ([-0.25, 0.45, -0.25], [0.25, 1.0, -0.2]),
        ([-0.25, 0.45, -0.25], [-0.2, 0.7, 0.25]),
        ([0.2, 0.45, -0.25], [0.25, 0.7, 0.25]),
    ])
}

#[test]
fn chair_faces_away_from_its_backrest_in_every_rotation() {
    let m = measured(Kind::Seat, &chair(), 0.45);
    assert_eq!(m.front, Facing::PosZ);
    assert_eq!((m.head, m.basis), (Facing::PosZ, Basis::Upright));
    assert!(near2(m.center, [0.0, 0.0]) && near2(m.footprint, [0.5, 0.5]));
    assert!((m.top - 0.45).abs() < EPS);
    assert!(m.floor.abs() < EPS);
    for quarters in 0..4 {
        let m = measured(Kind::Seat, &turned(&chair(), quarters), 0.45);
        assert_eq!(m.front, turned_facing(Facing::PosZ, quarters), "{quarters} quarter turns");
    }
    // Turned once, the backrest is at -X: the front is +X.
    assert_eq!(measured(Kind::Seat, &turned(&chair(), 1), 0.45).front, Facing::PosX);
}

#[test]
fn bench_with_a_backrest_faces_away_from_it() {
    // Long along z, backrest on the +X side.
    let s = soup(&[([-0.2, 0.0, -0.8], [0.2, 0.45, 0.8]), ([0.15, 0.45, -0.8], [0.2, 0.9, 0.8])]);
    assert_eq!(measured(Kind::Seat, &s, 0.45).front, Facing::NegX);
}

#[test]
fn backless_seats_get_the_fallback() {
    // Stool: square, nothing above the top: +Z.
    let stool = soup(&[([-0.2, 0.0, -0.2], [0.2, 0.6, 0.2])]);
    assert_eq!(measured(Kind::Seat, &stool, 0.6).front, Facing::PosZ);
    assert_eq!(measured(Kind::Seat, &stool, 0.6).basis, Basis::PlusZ);
    // Benches: the + side of the short axis.
    let along_z = soup(&[([-0.2, 0.0, -0.8], [0.2, 0.45, 0.8])]);
    let m = measured(Kind::Seat, &along_z, 0.45);
    assert_eq!((m.front, m.head, m.basis), (Facing::PosX, Facing::PosX, Basis::LongSide));
    let along_x = soup(&[([-0.8, 0.0, -0.2], [0.8, 0.45, 0.2])]);
    assert_eq!(measured(Kind::Seat, &along_x, 0.45).front, Facing::PosZ);
    // Below the bench aspect (0.5 x 0.4 = 1.25): a stool, +Z, though longer along x.
    let squat = soup(&[([-0.25, 0.0, -0.2], [0.25, 0.45, 0.2])]);
    assert_eq!(measured(Kind::Seat, &squat, 0.45).front, Facing::PosZ);
    // A centred post above the seat (a parasol, a lamp) is no backrest.
    let post = soup(&[([-0.2, 0.0, -0.2], [0.2, 0.45, 0.2]), ([-0.05, 0.45, -0.05], [0.05, 1.5, 0.05])]);
    assert_eq!(measured(Kind::Seat, &post, 0.45).front, Facing::PosZ);
}

/// A 2 m bed along x, top 0.6 m, headboard at +X (to 1.2 m), footboard at -X (to 0.8 m).
fn bed() -> Soup {
    soup(&[
        ([-1.0, 0.0, -0.5], [1.0, 0.6, 0.5]),
        ([0.95, 0.6, -0.5], [1.0, 1.2, 0.5]),
        ([-1.0, 0.6, -0.5], [-0.95, 0.8, 0.5]),
    ])
}

#[test]
fn bed_head_is_its_taller_end() {
    let m = measured(Kind::Bed, &bed(), 0.6);
    assert_eq!((m.head, m.front, m.basis), (Facing::PosX, Facing::PosX, Basis::Upright));
    assert!(near2(m.footprint, [2.0, 1.0]));
    for quarters in 0..4 {
        let m = measured(Kind::Bed, &turned(&bed(), quarters), 0.6);
        assert_eq!(m.head, turned_facing(Facing::PosX, quarters), "{quarters} quarter turns");
    }
    // Into a def: lie +Z = +X, entered from the long sides; the lier's right is Y x X = -Z.
    let d = def_for(&RecordId::new("furn_bed"), Kind::Bed, &m, &FurnitureParams::default()).expect("a def");
    let lie = &d.slots[0];
    assert!((lie.local.rotation * Vec3::Z).abs_diff_eq(Vec3::X, EPS));
    let sides: Vec<(Side, Vec3)> = lie.entrances.iter().map(|e| (e.side, e.approach)).collect();
    assert_eq!(sides[0].0, Side::Left);
    assert!(sides[0].1.abs_diff_eq(Vec3::new(0.0, 0.0, 0.8), EPS));
    assert_eq!(sides[1].0, Side::Right);
    assert!(sides[1].1.abs_diff_eq(Vec3::new(0.0, 0.0, -0.8), EPS));
}

#[test]
fn beds_without_a_clear_head_get_the_fallback() {
    // Bedroll along z, top 0.22 m, a pillow 8 cm high at -Z: below the board line, + long axis.
    let bedroll = soup(&[([-0.3, 0.0, -0.9], [0.3, 0.22, 0.9]), ([-0.25, 0.22, -0.9], [0.25, 0.3, -0.6])]);
    let m = measured(Kind::Bed, &bedroll, 0.22);
    assert_eq!((m.head, m.front, m.basis), (Facing::PosZ, Facing::PosZ, Basis::LongEnd));
    // Equal posts at both ends (a hammock frame, a canopy): a tie, + long axis (x).
    let posts = soup(&[
        ([-1.0, 0.0, -0.5], [1.0, 0.6, 0.5]),
        ([-1.0, 0.6, -0.5], [-0.9, 2.0, -0.4]),
        ([-1.0, 0.6, 0.4], [-0.9, 2.0, 0.5]),
        ([0.9, 0.6, -0.5], [1.0, 2.0, -0.4]),
        ([0.9, 0.6, 0.4], [1.0, 2.0, 0.5]),
    ]);
    assert_eq!(measured(Kind::Bed, &posts, 0.6).head, Facing::PosX);
}

#[test]
fn tables_and_counters_face_a_long_side() {
    // A table off the origin: centre and footprint from the bounds.
    let table = soup(&[([1.4, 0.0, 2.6], [2.6, 0.84, 3.4])]);
    let m = measured(Kind::Table, &table, 0.84);
    assert!(near2(m.center, [2.0, 3.0]) && near2(m.footprint, [1.2, 0.8]));
    assert_eq!((m.front, m.head, m.basis), (Facing::PosZ, Facing::PosZ, Basis::LongSide));
    // Its origin at the top: the floor is the lowest point, 0.84 m below.
    let lifted: Soup = table.iter().map(|t| t.map(|p| p - Vec3::Y * 0.84)).collect();
    assert!((measured(Kind::Table, &lifted, 0.84).floor + 0.84).abs() < EPS);
    // A counter along z, with a raised back rail: still the long side, +X.
    let counter = soup(&[([-0.3, 0.0, -1.0], [0.3, 0.94, 1.0]), ([-0.3, 0.94, -1.0], [-0.25, 1.2, 1.0])]);
    assert_eq!(measured(Kind::Counter, &counter, 0.94).front, Facing::PosX);
}

#[test]
fn degenerate_input_is_none() {
    let p = MeasureParams::default();
    assert!(measure(Kind::Seat, &[], 0.45, &p).is_none());
    // A flat panel in the x-y plane: no extent along z.
    let panel = vec![[Vec3::ZERO, Vec3::X, Vec3::new(1.0, 1.0, 0.0)]];
    assert!(measure(Kind::Table, &panel, 0.8, &p).is_none());
    let chair = chair();
    for top in [0.0, -0.1, f32::NAN, f32::INFINITY] {
        assert!(measure(Kind::Seat, &chair, top, &p).is_none(), "top {top}");
    }
    let mut broken = chair;
    broken[0][0] = Vec3::new(f32::NAN, 0.0, 0.0);
    assert!(measure(Kind::Seat, &broken, 0.45, &p).is_none());
}

#[test]
fn measure_params_default_to_the_declarations() {
    let p = MeasureParams::default();
    let want = [0.2, 0.3, 0.1, 1.5, 0.15, 0.25, 0.6];
    let got = [
        p.backrest_above_m,
        p.backrest_offset,
        p.upright_min_area,
        p.bench_aspect,
        p.headboard_above_m,
        p.bed_end_share,
        p.head_share,
    ];
    for (g, w) in got.iter().zip(want) {
        assert!((g - w).abs() < EPS, "{got:?}");
    }
}
