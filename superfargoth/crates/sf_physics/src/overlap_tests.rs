//! `IN33b1a`: `PhysicsWorld::body_overlaps_at` ("would this body, placed at `pose`, overlap
//! anything in `layers`?"), the swing door's next-pose check, and a kinematic triangle-mesh
//! leaf pushing an upright capsule through `PhysicsProbe::contacts`.

use std::f32::consts::FRAC_PI_2;
use std::sync::Arc;

use glam::{Affine3A, Quat, Vec3};
use sf_controller::{CollisionBuilder, GroundHit, GroundProbe, ProbeQuery};
use sf_coords::{WorldPos, WorldTransform};

use crate::tests::{at, ground};
use crate::{BodyDesc, BodyId, BodyKind, Layer, MeshShape, PhysicsParams, PhysicsWorld, ShapeRef, SurfaceId};

const RADIUS: f32 = 0.3;
const HALF: f32 = 0.45;
const MARGIN: f32 = 0.01;
/// The door leaf in its hinge frame: 0.1 m thick (x), 2 m tall (y), 1 m wide (+z from the hinge).
const LEAF_MIN: Vec3 = Vec3::new(-0.05, -1.0, 0.0);
const LEAF_MAX: Vec3 = Vec3::new(0.05, 1.0, 1.0);
/// The capsule's centre from the hinge: in front of the leaf (+x), off its face, and inside
/// the leaf's sweep when it swings 90 degrees toward +x.
const CAPSULE_FROM_HINGE: Vec3 = Vec3::new(0.6, 0.0, 0.2);
/// The probe-contact wall (as `probe_contacts_tests`): 0.2 m thick, 2 m tall, 6 m wide.
const WALL: Vec3 = Vec3::new(0.1, 1.0, 3.0);

/// A closed box from `min` to `max` as an outward-wound triangle mesh.
fn box_mesh(min: Vec3, max: Vec3) -> Arc<MeshShape> {
    let corner = |i: usize| {
        let pick = |bit: usize, lo: f32, hi: f32| if i & bit == 0 { lo } else { hi };
        Vec3::new(pick(1, min.x, max.x), pick(2, min.y, max.y), pick(4, min.z, max.z))
    };
    let positions: Vec<[f32; 3]> = (0..8).map(|i| corner(i).to_array()).collect();
    let mut indices = Vec::new();
    for axis in 0..3 {
        for side in 0..2 {
            let q: Vec<usize> = (0..8).filter(|i| (i >> axis) & 1 == side).collect();
            // Z-order to a cycle.
            let quad = [q[0], q[1], q[3], q[2]];
            let mut outward = Vec3::ZERO;
            outward[axis] = if side == 0 { -1.0 } else { 1.0 };
            let (a, b, c) = (corner(quad[0]), corner(quad[1]), corner(quad[2]));
            let flip = (b - a).cross(c - a).dot(outward) < 0.0;
            for tri in [[quad[0], quad[1], quad[2]], [quad[0], quad[2], quad[3]]] {
                let tri = if flip { [tri[0], tri[2], tri[1]] } else { tri };
                indices.extend(tri.map(|i| u32::try_from(i).expect("corner index")));
            }
        }
    }
    let mut builder = CollisionBuilder::new(1e-4);
    builder.add(&positions, &indices, Affine3A::IDENTITY, SurfaceId::DEFAULT);
    MeshShape::new(&builder.finish()).expect("a valid box mesh")
}

fn hinge() -> WorldPos {
    WorldPos::ORIGIN.offset(Vec3::new(5.0, 1.2, -3.0))
}

fn swung(angle: f32) -> WorldTransform {
    WorldTransform { rotation: Quat::from_rotation_y(angle), ..at(hinge()) }
}

/// A world with a kinematic mesh leaf hinged at [`hinge`] (on `Layer::STATIC`) and a
/// kinematic capsule on `Layer::CHARACTER` in front of it.
fn door_world() -> (PhysicsWorld, BodyId) {
    let mut w = PhysicsWorld::new(WorldPos::ORIGIN, PhysicsParams::default());
    let leaf = ShapeRef::Mesh(box_mesh(LEAF_MIN, LEAF_MAX));
    let door = w.insert(&BodyDesc::new(BodyKind::Kinematic, at(hinge()), leaf, Layer::STATIC));
    let capsule = ShapeRef::Capsule { r: RADIUS, half: HALF };
    w.insert(&BodyDesc::new(BodyKind::Kinematic, at(hinge().offset(CAPSULE_FROM_HINGE)), capsule, Layer::CHARACTER));
    w.refresh_queries();
    (w, door)
}

/// The leaf moved so it cuts through the capsule: its face plane at the capsule's axis, its
/// width centred on it.
fn onto_capsule() -> WorldTransform {
    at(hinge().offset(Vec3::new(CAPSULE_FROM_HINGE.x, 0.0, CAPSULE_FROM_HINGE.z - 0.5)))
}

#[test]
fn a_mesh_leaf_overlaps_the_capsule_only_where_it_cuts_it() {
    let (w, door) = door_world();
    let here = w.pose(door).expect("door pose");
    assert!(!w.body_overlaps_at(door, here, Layer::CHARACTER), "free where it stands");
    assert!(w.body_overlaps_at(door, onto_capsule(), Layer::CHARACTER), "translated onto the capsule");
    assert!(w.body_overlaps_at(door, swung(FRAC_PI_2), Layer::CHARACTER), "swung toward the capsule");
    assert!(!w.body_overlaps_at(door, swung(-FRAC_PI_2), Layer::CHARACTER), "swung away from it");
}

#[test]
fn overlaps_respect_layers_and_never_the_body_itself() {
    let (w, door) = door_world();
    assert!(!w.body_overlaps_at(door, onto_capsule(), Layer::PROP), "the capsule is not a prop");
    assert!(w.body_overlaps_at(door, onto_capsule(), Layer::all()), "every layer sees the capsule");

    let mut alone = PhysicsWorld::new(WorldPos::ORIGIN, PhysicsParams::default());
    let leaf = ShapeRef::Mesh(box_mesh(LEAF_MIN, LEAF_MAX));
    let door = alone.insert(&BodyDesc::new(BodyKind::Kinematic, at(hinge()), leaf, Layer::STATIC));
    alone.refresh_queries();
    let here = alone.pose(door).expect("door pose");
    assert!(!alone.body_overlaps_at(door, here, Layer::all()), "never overlaps itself");
    assert!(!alone.body_overlaps_at(door, here, Layer::STATIC), "not even on its own layer");
}

#[test]
fn an_unknown_or_removed_body_overlaps_nothing() {
    let (mut w, door) = door_world();
    assert!(w.remove(door));
    w.refresh_queries();
    assert!(!w.body_overlaps_at(door, onto_capsule(), Layer::all()));
}

fn probe_base() -> WorldPos {
    WorldPos::ORIGIN.offset(Vec3::new(5.0, 0.0, -3.0))
}

fn probe_capsule() -> WorldPos {
    probe_base().offset(Vec3::Y * 1.2)
}

/// A kinematic wall of `shape` whose +x face starts 0.03 m behind the capsule's surface,
/// stepped 0.05 m toward it (it ends 0.02 m inside): the capsule's contact.
fn shoved_contact(shape: ShapeRef) -> Option<GroundHit> {
    let mut w = PhysicsWorld::new(WorldPos::ORIGIN, PhysicsParams { hz: 60.0, ..PhysicsParams::default() });
    ground(&mut w, probe_base());
    let start = probe_capsule().offset(Vec3::new(-RADIUS - 0.03 - WALL.x, 0.0, 0.0));
    let wall = w.insert(&BodyDesc::new(BodyKind::Kinematic, at(start), shape, Layer::STATIC));
    w.refresh_queries();
    let pos = w.pose(wall).expect("wall").pos.offset(Vec3::X * 0.05);
    assert!(w.set_kinematic_target(wall, WorldTransform::from_pos(pos)));
    w.step();
    let mut out = [None];
    let q = ProbeQuery::contact(probe_capsule(), MARGIN, RADIUS, HALF);
    w.probe(Layer::STATIC | Layer::PROP).contacts(&[q], &mut out);
    out[0]
}

#[test]
fn a_kinematic_mesh_pushes_like_a_box() {
    let boxed = shoved_contact(ShapeRef::Box(WALL)).expect("a box-wall contact");
    let mesh = shoved_contact(ShapeRef::Mesh(box_mesh(-WALL, WALL)));
    println!("box: normal {}, distance {:.5}; mesh: {:?}", boxed.normal, boxed.distance, mesh.map(|h| (h.normal, h.distance)));
    let mesh = mesh.expect("a mesh-wall contact");
    assert!(mesh.normal.dot(Vec3::X) > 0.999, "normal {}", mesh.normal);
    assert!((mesh.distance - boxed.distance).abs() < 1e-3, "mesh {} vs box {}", mesh.distance, boxed.distance);
}
