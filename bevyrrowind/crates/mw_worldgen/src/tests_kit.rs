//! S1 tests: the kit catalogue on a fixture (no game data): a box building used twice (once turned a quarter), a
//! door against one wall of the turned one, a wall piece.

// Exact results are the point of these tests.
#![allow(clippy::float_cmp)]

use std::f32::consts::{FRAC_PI_2, PI};

use glam::{DVec3, Quat, Vec3};
use sf_coords::{WorldPos, WorldTransform};

use crate::kit::{self, KitDoor, KitKind, KitMesh, KitParams, KitRef, KitTable, Obb};

const BOX: &str = "meshes/x/ex_hlaalu_b_99.nif";
const DOOR: &str = "meshes/d/door_99.nif";
const WALL: &str = "meshes/x/ex_hlaalu_wall_99.nif";
const TRAP: &str = "meshes/d/trapdoor_99.nif";
const TOWER: &str = "meshes/x/ex_hlaalu_bal_99.nif";
const IMP: &str = "meshes/x/ex_imp_keep_99.nif";

/// Box: x +-2, up -0.5..6.5, z +-1.5 (kit y +-1.5), plus an inner point. Door: x +-0.5, up 0..2.5, z +-0.05.
/// Wall: x +-1, up 0..3, z +-0.2. Trapdoor: x +-0.5, up 0..0.05, z +-0.5.
fn meshes(id: &str) -> Option<KitMesh> {
    let corners = |x: f32, y: [f32; 2], z: f32| {
        let mut v = Vec::new();
        for sx in [-x, x] {
            for &sy in &y {
                for sz in [-z, z] {
                    v.push([sx, sy, sz]);
                }
            }
        }
        v
    };
    let (mut points, hash) = match id {
        BOX | TOWER | IMP => (corners(2.0, [-0.5, 6.5], 1.5), "box"),
        DOOR => (corners(0.5, [0.0, 2.5], 0.05), "door"),
        WALL => (corners(1.0, [0.0, 3.0], 0.2), "wall"),
        TRAP => (corners(0.5, [0.0, 0.05], 0.5), "trap"),
        _ => return None,
    };
    if id == BOX {
        points.push([0.0, 3.0, 0.0]);
    }
    Some(KitMesh { points, blake3: hash.into() })
}

fn tf(pos: [f64; 3], rotation: Quat) -> WorldTransform {
    WorldTransform { pos: WorldPos::from_f64(DVec3::from_array(pos)), rotation, scale: Vec3::ONE }
}

fn r(key: &str, model: Option<&str>, transform: WorldTransform, door: bool) -> KitRef {
    KitRef { key: key.into(), base: format!("base_{key}"), model: model.map(str::to_owned), transform, door }
}

/// k1: the box at (10, 5, -20) engine, turned a quarter about up; k2: the box at (300, 0, 0); kd: the door at kit
/// local (1.0, -1.6), 0.2 m up in k1's frame, turned half round against it (normal pointing in); kw: a wall; kn: no model.
fn refs() -> Vec<KitRef> {
    let q = Quat::from_rotation_y(FRAC_PI_2);
    let k1 = [10.0, 5.0, -20.0];
    // Kit local (x, y, z up) = engine (x, z up, -y).
    let local = q * Vec3::new(1.0, 0.2, 1.6);
    let door_pos = [k1[0] + f64::from(local.x), k1[1] + f64::from(local.y), k1[2] + f64::from(local.z)];
    vec![
        r("k1", Some(BOX), tf(k1, q), false),
        r("k2", Some(BOX), tf([300.0, 0.0, 0.0], Quat::IDENTITY), false),
        r("kd", Some(DOOR), tf(door_pos, q * Quat::from_rotation_y(PI)), true),
        r("kw", Some(WALL), tf([-50.0, 0.0, 40.0], Quat::IDENTITY), false),
        r("kn", None, tf([0.0, 0.0, 0.0], Quat::IDENTITY), false),
    ]
}

fn build(refs: &[KitRef]) -> kit::Built {
    kit::build("Fixture", "test", refs, &meshes, &|_| None, &KitTable::builtin().unwrap(), &KitParams::default())
}

fn build_on(refs: &[KitRef], terrain: kit::Terrain<'_>) -> kit::Built {
    kit::build("Fixture", "test", refs, &meshes, terrain, &KitTable::builtin().unwrap(), &KitParams::default())
}

/// Three box references on flat ground at 10 m: g1 5 m up, g2 8 m up, g3 4 m up turned a quarter at scale 2; a
/// wall 1 m up.
fn grounded() -> Vec<KitRef> {
    let mut g3 = tf([200.0, 4.0, 0.0], Quat::from_rotation_y(FRAC_PI_2));
    g3.scale = Vec3::splat(2.0);
    vec![
        r("g1", Some(BOX), tf([0.0, 5.0, 0.0], Quat::IDENTITY), false),
        r("g2", Some(BOX), tf([100.0, 8.0, 0.0], Quat::IDENTITY), false),
        r("g3", Some(BOX), g3, false),
        r("gw", Some(WALL), tf([-50.0, 1.0, 40.0], Quat::IDENTITY), false),
    ]
}

#[test]
fn ground_is_the_median_over_references_in_model_frame() {
    let flat = |_: [f64; 2]| Some(10.0);
    let built = build_on(&grounded(), &flat);
    let (b, w) = (&built.kit.models[0], &built.kit.models[1]);
    // Grounds 10-5 = 5, 10-8 = 2, (10-4)/2 = 3: median 3; the box's base is 0.5 below its pivot.
    assert_eq!((b.ground_z_m, b.foundation_depth_m), (Some(3.0), Some(3.5)));
    assert_eq!((w.ground_z_m, w.foundation_depth_m), (Some(9.0), Some(9.0)));
    let g = &built.stats.ground[0];
    assert_eq!((g.id.as_str(), g.at_door, g.pad.as_slice()), (BOX, false, [5.0, 2.0, 3.0].as_slice()));
    assert_eq!((g.hull_min.as_slice(), g.hull_max.as_slice()), ([5.0, 2.0, 3.0].as_slice(), [5.0, 2.0, 3.0].as_slice()));
    assert_eq!(g.spread(), 3.0);
    assert_eq!(built.stats.ground[1].spread(), 0.0);
    let report = kit::report(&built, &KitParams::default());
    assert!(report.contains("buildings: ground_z_m 3.00..3.00 over 1 of 1"), "{report}");
    assert!(report.contains("spread over 0.5 m between references: 1 models: ex_hlaalu_b_99.nif 3.00"), "{report}");

    // Deterministic for any order; written right after `base_offset_m`.
    let mut reversed = grounded();
    reversed.reverse();
    assert_eq!(build_on(&reversed, &flat), built);
    let dir = tempfile::tempdir().unwrap();
    kit::write(&built.kit, dir.path()).unwrap();
    let text = std::fs::read_to_string(dir.path().join(kit::KIT_FILE)).unwrap();
    let at = |s: &str| text.find(s).unwrap_or_else(|| panic!("{s} in {text}"));
    assert!(at("\"base_offset_m\": -0.5") < at("\"ground_z_m\": 3.0") && at("\"ground_z_m\": 3.0") < at("\"foundation_depth_m\": 3.5"));
    assert_eq!(kit::read(dir.path()).unwrap(), built.kit);
}

#[test]
fn ground_is_sampled_out_of_the_street_door_else_at_the_centroid() {
    // A plane rising 1 m per metre east and 2 per metre north, 10 m at the origin.
    let plane = |p: [f64; 2]| Some(10.0 + p[0] + 2.0 * p[1]);
    let b = [0.0, 0.0, 0.0];
    let refs = vec![
        r("b1", Some(BOX), tf(b, Quat::IDENTITY), false),
        door_at("d", DOOR, b, (0.5, 1.55), 0.2, 0.0),
        r("gw", Some(WALL), tf([-50.0, 0.0, 40.0], Quat::IDENTITY), false),
    ];
    let built = build_on(&refs, &plane);
    let (m, w) = (&built.kit.models[0], &built.kit.models[1]);
    // The door at (0.5, 1.5) faces north: the pad 1 m out, at (0.5, 2.5).
    assert_eq!(m.doors, vec![door([0.5, 1.5], 90.0, 0.2)]);
    assert_eq!((m.ground_z_m, m.foundation_depth_m), (Some(15.5), Some(16.0)));
    let g = &built.stats.ground[0];
    assert!(g.at_door);
    // Under the hull (x +-2, y +-1.5): 10 - 2 - 3 .. 10 + 2 + 3.
    assert_eq!((g.hull_min.as_slice(), g.hull_max.as_slice()), ([5.0].as_slice(), [15.0].as_slice()));
    // The doorless wall: its centroid at the pivot, ground (-50, -40).
    assert!(!built.stats.ground[1].at_door);
    assert_eq!((w.ground_z_m, w.foundation_depth_m), (Some(-120.0), Some(-120.0)));
    // Without terrain: null.
    let bare = build(&refs);
    assert_eq!((bare.kit.models[0].ground_z_m, bare.kit.models[0].foundation_depth_m), (None, None));
}

#[test]
fn kit_of_a_turned_box_with_a_door() {
    let built = build(&refs());
    let k = &built.kit;
    assert_eq!(k.format, kit::FORMAT);
    assert_eq!(k.units, "m");
    let ids: Vec<&str> = k.models.iter().map(|m| m.id.as_str()).collect();
    assert_eq!(ids, [BOX, WALL]);
    let b = &k.models[0];
    assert_eq!(b.kind, KitKind::Building);
    assert_eq!(b.footprint, vec![[-2.0, -1.5], [2.0, -1.5], [2.0, 1.5], [-2.0, 1.5]]);
    assert_eq!(b.obb, Some(Obb { centre: [0.0, 0.0], half: [2.0, 1.5], yaw_deg: 0.0 }));
    assert_eq!(b.base_offset_m, -0.5);
    assert_eq!(b.height_m, 7.0);
    assert_eq!(b.weight, 2.0);
    assert_eq!(b.blake3, "box");
    assert_eq!(b.tags, ["base:base_k1", "base:base_k2"]);
    assert!(b.roles.is_empty());
    assert_eq!((b.floors, b.foundation_depth_m, b.yaw_step_deg), (None, None, 0.0));
    // The door of the first reference (k1, turned): on the south edge, facing out of it (-y = 270 deg).
    assert_eq!(b.doors, vec![KitDoor { at: [1.0, -1.5], facing_deg: 270.0, width_m: 1.0, sill_z_m: 0.2 }]);
    assert_eq!(kit::front_depth(b), Some((4.0, 3.0)));

    let w = &k.models[1];
    assert_eq!(w.kind, KitKind::Wall);
    assert_eq!((w.weight, w.base_offset_m, w.height_m), (1.0, 0.0, 3.0));
    assert!(w.doors.is_empty());

    let s = &built.stats;
    assert_eq!((s.door_refs, s.doors_assigned, s.no_model, s.scaled_refs), (1, 1, 1, 0));
    assert!(s.no_vertices.is_empty());
    assert_eq!(s.agreement.len(), 1);
    assert_eq!(s.agreement[0].doors_per_ref, [1, 0]);
    assert_eq!(s.agreement[0].agreeing, 0);
}

#[test]
fn kit_is_deterministic_and_round_trips() {
    let a = build(&refs());
    let mut reversed = refs();
    reversed.reverse();
    let b = build(&reversed);
    assert_eq!(a, b);
    let (da, db) = (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
    kit::write(&a.kit, da.path()).unwrap();
    kit::write(&b.kit, db.path()).unwrap();
    let (ta, tb) = (std::fs::read(da.path().join(kit::KIT_FILE)).unwrap(), std::fs::read(db.path().join(kit::KIT_FILE)).unwrap());
    assert_eq!(ta, tb);
    assert_eq!(kit::read(da.path()).unwrap(), a.kit);
    // The contract's spellings.
    let text = String::from_utf8(ta).unwrap();
    for s in ["\"kind\": \"building\"", "\"floors\": null", "\"facing_deg\": 270.0", "\"base_offset_m\": -0.5"] {
        assert!(text.contains(s), "{s} in {text}");
    }
    // A wrong format is refused.
    std::fs::write(da.path().join(kit::KIT_FILE), text.replace("\"format\": 1", "\"format\": 2")).unwrap();
    assert!(kit::read(da.path()).is_err());
}

#[test]
fn kit_table_kinds_and_rectangle() {
    let t = KitTable::builtin().unwrap();
    assert_eq!(t.kind_of("meshes/x/Ex_Hlaalu_B_01.NIF"), KitKind::Building);
    assert_eq!(t.kind_of("meshes/x/ex_hlaalu_bridge_01.nif"), KitKind::Bridge);
    assert_eq!(t.kind_of("meshes/x/ex_hlaalu_wall_gate_01.nif"), KitKind::Arch);
    assert_eq!(t.kind_of("meshes/x/ex_hlaalu_wall_01.nif"), KitKind::Wall);
    assert_eq!(t.kind_of("meshes/f/furn_de_lightpost_01.nif"), KitKind::Lamp);
    assert_eq!(t.kind_of("meshes/o/contain_crate_01.nif"), KitKind::Prop);
    // A rectangle turned 30 deg: the minimum-area rectangle finds it back.
    let (c, s) = (30f64.to_radians().cos(), 30f64.to_radians().sin());
    let rect: Vec<[f64; 2]> = [[-3.0, -1.0], [3.0, -1.0], [3.0, 1.0], [-3.0, 1.0]]
        .iter()
        .map(|p| [5.0 + c * p[0] - s * p[1], 7.0 + s * p[0] + c * p[1]])
        .collect();
    let hull = crate::site::convex_hull(&rect).unwrap();
    assert_eq!(kit::min_area_rect(&hull), Some(Obb { centre: [5.0, 7.0], half: [3.0, 1.0], yaw_deg: 30.0 }));
    let p = KitParams::default();
    assert_eq!((p.door_radius_m, p.door_corner_m, p.door_agree_m, p.door_agree_deg, p.bin_m), (1.5, 1.0, 0.25, 5.0, 2.0));
    assert_eq!((p.pad_probe_m, p.hull_probe_m, p.report_spread_m, p.report_sill_m), (1.0, 1.0, 0.5, 0.3));
    // Doorless landmarks are props, not lot houses.
    for id in ["meshes/x/ex_hlaalu_fb_01.nif", "meshes/x/Ex_Hlaalu_StriderPort_01.nif", "meshes/x/ex_velothi_temple_02.nif"] {
        assert_eq!(t.kind_of(id), KitKind::Prop, "{id}");
    }
    // Dropped by the style, still buildings so their doors stay theirs.
    assert_eq!(t.kind_of("meshes/x/ex_velothi_temple_01.nif"), KitKind::Building);
    // K1: Balmora's two `ex_hlaalu_bal_` pieces are the Hlaalu towers.
    assert_eq!(t.kind_of("meshes/x/Ex_Hlaalu_Bal_01.nif"), KitKind::Tower);
    assert_eq!(serde_json::to_string(&KitKind::Tower).unwrap(), "\"tower\"");
    // The Hlaalu style: Hlaalu, common Dunmer and dressing kept; Imperial, Velothi and the rest dropped.
    for id in ["meshes/x/ex_hlaalu_wall_curve_01.nif", "meshes/x/ex_de_docks_end.nif", "meshes/l/light_de_lantern_03.nif", "meshes/r/siltstrider.nif"] {
        assert!(t.keeps(id), "{id}");
    }
    for id in ["meshes/x/ex_imp_towerb_base_01.nif", "meshes/x/ex_velothi_temple_01.nif", "meshes/x/Ex_Vivec_G_R_01.nif", "meshes/x/ex_ropebridge_1024_01.nif", "meshes/x/ex_gnisis_roadmarker_01.nif", "meshes/x/flora_tree_01.nif"] {
        assert!(!t.keeps(id), "{id}");
    }
    assert!(KitTable::default().keeps("meshes/x/ex_imp_keep_01.nif"));
}

/// A door reference `key` (mesh `model`) at kit local `(x, y)`, `sill` m up, in the frame of a building standing
/// unturned at engine `b`, turned `turn` radians about up against it (0: its mesh normal along kit y).
fn door_at(key: &str, model: &str, b: [f64; 3], (x, y): (f64, f64), sill: f64, turn: f32) -> KitRef {
    r(key, Some(model), tf([b[0] + x, b[1] + sill, b[2] - y], Quat::from_rotation_y(turn)), true)
}

fn door(at: [f64; 2], facing_deg: f64, sill_z_m: f64) -> KitDoor {
    KitDoor { at, facing_deg, width_m: 1.0, sill_z_m }
}

#[test]
fn doors_face_their_edge_street_door_first() {
    // One door per edge, each mesh turned a quarter from its wall (normal along the edge), sills out of order.
    let b = [0.0, 0.0, 0.0];
    let refs = vec![
        r("b1", Some(BOX), tf(b, Quat::IDENTITY), false),
        door_at("da", DOOR, b, (2.05, 0.3), 3.0, 0.0),
        door_at("db", DOOR, b, (-0.5, 1.45), 0.2, FRAC_PI_2),
        door_at("dc", DOOR, b, (-2.1, 0.0), 1.0, 0.0),
        door_at("dd", DOOR, b, (0.0, -1.55), 2.0, FRAC_PI_2),
    ];
    let built = build(&refs);
    assert_eq!(
        built.kit.models[0].doors,
        vec![door([-0.5, 1.5], 90.0, 0.2), door([-2.0, 0.0], 180.0, 1.0), door([0.0, -1.5], 270.0, 2.0), door([2.0, 0.3], 0.0, 3.0)]
    );
    assert_eq!(built.stats.doors_assigned, 4);
    assert!(built.stats.trapdoors.is_empty());
}

#[test]
fn corner_door_takes_the_edge_its_mesh_agrees_with() {
    // 0.2 m from the south edge, 0.3 m from the east one: both within the corner distance.
    let b = [0.0, 0.0, 0.0];
    let doors_with = |turn: f32| {
        let refs = vec![r("b1", Some(BOX), tf(b, Quat::IDENTITY), false), door_at("d", DOOR, b, (1.7, -1.3), 0.2, turn)];
        build(&refs).kit.models[0].doors.clone()
    };
    // Mesh normal along x: the east edge, though the south one is nearer.
    assert_eq!(doors_with(FRAC_PI_2), vec![door([2.0, -1.3], 0.0, 0.2)]);
    // Mesh normal along y (either sign): the nearest, south edge.
    assert_eq!(doors_with(0.0), vec![door([1.7, -1.5], 270.0, 0.2)]);
    assert_eq!(doors_with(PI), vec![door([1.7, -1.5], 270.0, 0.2)]);
}

#[test]
fn trapdoors_are_left_out() {
    let b = [0.0, 0.0, 0.0];
    let refs = vec![
        r("b1", Some(BOX), tf(b, Quat::IDENTITY), false),
        door_at("dt", TRAP, b, (0.0, 1.0), 0.0, 0.0),
        door_at("dw", DOOR, b, (0.0, 1.55), 0.2, 0.0),
    ];
    let built = build(&refs);
    assert_eq!(built.kit.models[0].doors, vec![door([0.0, 1.5], 90.0, 0.2)]);
    assert_eq!((built.stats.door_refs, built.stats.doors_assigned), (2, 1));
    assert_eq!(built.stats.trapdoors, [TRAP]);
}

#[test]
fn doors_come_from_the_reference_with_the_most() {
    // a1 has no door, a2 one on its north edge: the model keeps a2's.
    let (b1, b2) = ([0.0, 0.0, 0.0], [100.0, 0.0, 0.0]);
    let refs = vec![
        r("a1", Some(BOX), tf(b1, Quat::IDENTITY), false),
        r("a2", Some(BOX), tf(b2, Quat::IDENTITY), false),
        door_at("d", DOOR, b2, (0.5, 1.55), 0.2, 0.0),
    ];
    let built = build(&refs);
    assert_eq!(built.kit.models[0].doors, vec![door([0.5, 1.5], 90.0, 0.2)]);
    let a = &built.stats.agreement[0];
    assert_eq!((a.doors_per_ref.as_slice(), a.kept, a.agreeing), ([0, 1].as_slice(), 1, 0));
}

/// K1: two anchors merge into one kit (weights summed, one entry per model, a shared reference once), the style
/// drops a model (which still takes its door), `ex_hlaalu_bal_` is a tower, and one anchor builds as before.
#[test]
fn anchors_merge_in_one_style() {
    let id = Quat::IDENTITY;
    let a = vec![
        r("a1", Some(BOX), tf([0.0, 0.0, 0.0], id), false),
        r("a2", Some(WALL), tf([-50.0, 0.0, 40.0], id), false),
        // The keep 5 m east of the box, a door against its west wall (1 m from the box): the keep's, though dropped.
        r("ai", Some(IMP), tf([5.0, 0.0, 0.0], id), false),
        door_at("ad", DOOR, [5.0, 0.0, 0.0], (-2.05, 0.0), 0.0, FRAC_PI_2),
    ];
    let b = vec![
        r("b1", Some(BOX), tf([1000.0, 0.0, 0.0], id), false),
        r("b2", Some(BOX), tf([1100.0, 0.0, 0.0], id), false),
        r("bt", Some(TOWER), tf([1200.0, 0.0, 0.0], id), false),
        door_at("bd", DOOR, [1200.0, 0.0, 0.0], (0.5, 1.55), 0.2, 0.0),
        // Shared with the first anchor: counted once.
        r("a2", Some(WALL), tf([-50.0, 0.0, 40.0], id), false),
    ];
    let t = KitTable::builtin().unwrap();
    let p = KitParams::default();
    let built = kit::build_anchors("test", &[("A", &a), ("B", &b)], &meshes, &|_| None, &t, &p);
    assert_eq!(built.kit.name, "A + B");
    let ids: Vec<(&str, KitKind, f64)> = built.kit.models.iter().map(|m| (m.id.as_str(), m.kind, m.weight)).collect();
    assert_eq!(ids, [(BOX, KitKind::Building, 3.0), (TOWER, KitKind::Tower, 1.0), (WALL, KitKind::Wall, 1.0)]);
    assert_eq!(built.kit.models[0].tags, ["base:base_a1", "base:base_b1", "base:base_b2"]);
    assert_eq!(built.kit.models[1].doors.len(), 1, "the tower takes its door");
    assert!(built.kit.models[0].doors.is_empty(), "the dropped keep's door stays with the keep");
    assert_eq!(built.stats.dropped, [(IMP.to_owned(), 1)]);
    let per: Vec<(&str, usize, usize, usize)> =
        built.stats.anchors.iter().map(|s| (s.name.as_str(), s.kept, s.dropped, s.refs)).collect();
    assert_eq!(per, [("A", 2, 1, 4), ("B", 3, 0, 5)]);
    let report = kit::report(&built, &p);
    assert!(report.contains("anchor A: 4 references, models kept 2 dropped 1"), "{report}");
    assert!(report.contains("style dropped 1 models: ex_imp_keep_99.nif x1"), "{report}");
    // Order of anchors and references does not change the models.
    let mut rb = b.clone();
    rb.reverse();
    assert_eq!(kit::build_anchors("test", &[("B", &rb), ("A", &a)], &meshes, &|_| None, &t, &p).kit.models, built.kit.models);

    // One anchor: the same kit as `build`, whatever the style keeps.
    let one = refs();
    let single = kit::build_anchors("test", &[("Fixture", &one)], &meshes, &|_| None, &t, &p);
    assert_eq!(single.kit, build(&one).kit);
    assert_eq!(single.kit, kit::build("Fixture", "test", &one, &meshes, &|_| None, &KitTable { style: None, ..t.clone() }, &p).kit);
}
