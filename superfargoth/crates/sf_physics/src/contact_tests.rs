//! `PhysicsEvent::Contact` reports (audio.md "Impacts"): hits report their
//! speed, resting and rolling contacts stay silent, sliding reports its speed.

use glam::{Affine3A, IVec3, Vec3};
use sf_controller::CollisionBuilder;
use sf_coords::WorldPos;

use crate::tests::{at, ground};
use crate::{BodyDesc, BodyId, BodyKind, Layer, PhysicsEvent, PhysicsParams, PhysicsWorld, ShapeRef, SurfaceId, MeshShape};

struct Report {
    step: usize,
    a: BodyId,
    speed: f32,
    slide: f32,
    surfaces: [SurfaceId; 2],
}

fn world(params: PhysicsParams) -> (PhysicsWorld, WorldPos) {
    let base = WorldPos::new(IVec3::ZERO, Vec3::ZERO);
    let mut w = PhysicsWorld::new(base, params);
    let floor = ground(&mut w, base);
    w.set_surface(floor, SurfaceId(2));
    (w, base)
}

fn ball(w: &mut PhysicsWorld, pos: WorldPos) -> BodyId {
    let desc = BodyDesc::new(BodyKind::Dynamic { asleep: false }, at(pos), ShapeRef::Sphere(0.06), Layer::DEBRIS);
    w.insert(&BodyDesc { mass: Some(0.3), ccd: true, ..desc }.with_surface(SurfaceId(1)))
}

fn run(w: &mut PhysicsWorld, steps: usize) -> Vec<Report> {
    let mut out = Vec::new();
    for step in 0..steps {
        w.step();
        for e in w.drain_events() {
            if let PhysicsEvent::Contact { a, speed, slide, surfaces, .. } = e {
                out.push(Report { step, a, speed, slide, surfaces });
            }
        }
    }
    out
}

#[test]
fn a_dropped_ball_reports_its_landing_speed_then_goes_quiet() {
    let (mut w, base) = world(PhysicsParams::default());
    let b = ball(&mut w, base.offset(Vec3::new(0.0, 2.06, 0.0)));
    let reports = run(&mut w, 240);
    let first = reports.first().expect("the landing reports");
    // 2 m free fall: about 6.3 m/s at the floor (no restitution: the speed change is the approach speed).
    assert!((5.5..7.5).contains(&first.speed), "landing speed {}", first.speed);
    assert_eq!(first.a, b);
    assert_eq!(first.surfaces, [SurfaceId(1), SurfaceId(2)]);
    // Resting on the floor holds it up by g·dt a step: below the report speed.
    let last = reports.last().map_or(0, |r| r.step);
    assert!(last < first.step + 30, "still reporting at step {last} (landed at {})", first.step);
}

#[test]
fn a_rolling_ball_does_not_report() {
    let (mut w, base) = world(PhysicsParams::default());
    let b = ball(&mut w, base.offset(Vec3::new(0.0, 0.06, 0.0)));
    // Rolling without slipping at 3 m/s along +X: spin about -Z at v / r.
    w.set_velocity(b, Vec3::new(3.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -3.0 / 0.06));
    let reports = run(&mut w, 120);
    // The first steps slip a little while friction matches the spin to the speed (a few
    // tenths of m/s); rolling after that touches the floor at rest: nothing to report.
    let summary: Vec<_> = reports.iter().map(|r| (r.step, r.speed, r.slide)).collect();
    assert!(reports.iter().all(|r| r.step < 10 && r.slide < 0.5 && r.speed < 0.5), "{summary:?}");
}

#[test]
fn a_sliding_box_reports_its_slide_speed() {
    let (mut w, base) = world(PhysicsParams::default());
    let desc = BodyDesc::new(BodyKind::Dynamic { asleep: false }, at(base.offset(Vec3::new(0.0, 0.2, 0.0))), ShapeRef::Box(Vec3::splat(0.2)), Layer::PROP);
    let b = w.insert(&BodyDesc { mass: Some(5.0), ..desc });
    w.set_velocity(b, Vec3::new(3.0, 0.0, 0.0), Vec3::ZERO);
    let reports = run(&mut w, 120);
    let early: Vec<_> = reports.iter().filter(|r| r.step < 5).collect();
    assert!(!early.is_empty());
    assert!(early.iter().all(|r| r.slide > 2.0 && r.speed < 1.0), "{:?}", early.iter().map(|r| (r.speed, r.slide)).collect::<Vec<_>>());
    assert!(reports.windows(2).all(|p| p[1].slide <= p[0].slide + 0.05), "friction slows the slide");
    // Friction stops it: the reports end once it slides slower than the report speed.
    assert!(reports.last().is_some_and(|r| r.step < 100));
}

#[test]
fn no_report_layers_no_reports() {
    let params = PhysicsParams { contact_report_layers: Layer::empty(), ..PhysicsParams::default() };
    let (mut w, base) = world(params);
    ball(&mut w, base.offset(Vec3::new(0.0, 2.06, 0.0)));
    assert!(run(&mut w, 120).is_empty());
}

#[test]
fn a_mesh_reports_the_touched_triangles_surface() {
    // Two 4 m quads side by side: x < 0 is surface 3, x > 0 surface 4.
    let mut builder = CollisionBuilder::new(1e-3);
    let quad = |x0: f32| [[x0, 0.0, -2.0], [x0, 0.0, 2.0], [x0 + 4.0, 0.0, 2.0], [x0 + 4.0, 0.0, -2.0]];
    builder.add(&quad(-4.0), &[0, 1, 2, 0, 2, 3], Affine3A::IDENTITY, SurfaceId(3));
    builder.add(&quad(0.0), &[0, 1, 2, 0, 2, 3], Affine3A::IDENTITY, SurfaceId(4));
    let mesh = MeshShape::new(&builder.finish()).expect("a valid mesh");
    let base = WorldPos::new(IVec3::ZERO, Vec3::ZERO);
    let mut w = PhysicsWorld::new(base, PhysicsParams::default());
    w.insert(&BodyDesc::fixed(at(base), ShapeRef::Mesh(mesh), Layer::STATIC));
    let left = ball(&mut w, base.offset(Vec3::new(-2.0, 1.0, 0.0)));
    let right = ball(&mut w, base.offset(Vec3::new(2.0, 1.0, 0.0)));
    let reports = run(&mut w, 60);
    let first = |b| reports.iter().find(|r| r.a == b).map(|r| r.surfaces);
    assert_eq!(first(left), Some([SurfaceId(1), SurfaceId(3)]));
    assert_eq!(first(right), Some([SurfaceId(1), SurfaceId(4)]));
}
