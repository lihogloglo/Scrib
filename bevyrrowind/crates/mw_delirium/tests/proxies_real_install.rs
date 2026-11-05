//! Held-body proxies (superfargoth character-shapes.md S3, PH-15) on Morrowind's humanoid skeleton, headless: the
//! parts as kinematic `BODY_PROXY` capsules in an `sf_physics` world, synced to the takeover every tick; balls
//! thrown at the held body hit them and each contact start pushes the struck part again. Skips (passes) without a
//! Morrowind install. Sanity asserts; numbers printed with `-- --nocapture`.
//!
//! One process per test binary: Delirium's asset is process-global.

#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]

use std::sync::OnceLock;

use glam::{Quat, Vec3};

use mw_delirium::delirium::prelude::Transform;
use mw_delirium::proxies::{PartShapes, part_shapes, posed_frames, shape_frames};
use mw_delirium::sim::DT;
use mw_delirium::{Body, Options, Rules, Takeover, authoring_from_model, bones_to_parts, character_from_skeleton, pose_to_bones};
use mw_install::MorrowindInstall;
use sf_anim::Skeleton;
use sf_coords::{WorldPos, WorldTransform};
use sf_physics::{BodyDesc, BodyId, BodyKind, Layer, PhysicsEvent, PhysicsParams, PhysicsWorld, ShapeRef};
use sf_vfs::{Vfs, VfsPath};

struct Fixture {
    skeleton: Skeleton,
    body: Body,
}

fn fixture() -> Option<&'static Fixture> {
    static F: OnceLock<Option<Fixture>> = OnceLock::new();
    F.get_or_init(|| {
        let Ok((install, _)) = MorrowindInstall::detect(None) else {
            eprintln!("no Morrowind install detected; skipping");
            return None;
        };
        let vfs: Vfs = mw_vfs::mount_install(&install, Vfs::builder()).unwrap().vfs;
        let bytes = vfs.read(&VfsPath::new("meshes/base_anim.nif").unwrap()).unwrap();
        let skeleton = mw_anim::rig::skeleton(&mw_nif::load(&bytes).unwrap()).unwrap();
        let body = character_from_skeleton(&skeleton, &Options::default()).unwrap();
        Some(Fixture { skeleton, body })
    })
    .as_ref()
}

/// A takeover of the rest pose (sim = authoring space, feet at z = 0), standing still.
fn takeover(f: &Fixture) -> Takeover {
    let parts = bones_to_parts(&f.body.table, &f.skeleton, f.skeleton.rest(), &authoring_from_model());
    Takeover::start(&f.body, &parts, Some((&parts, DT)), None, 0.0, 0.3, Rules::default())
}

/// World (Y up) from sim (Z up): the authoring rotation is its own inverse.
fn to_world(v: Vec3) -> Vec3 {
    authoring_from_model().transform_vector3(v)
}

fn world_rotation() -> Quat {
    Quat::from_mat3a(&authoring_from_model().matrix3)
}

/// The physics side: a world with the takeover's parts as proxies (user = part).
struct Bridge {
    world: PhysicsWorld,
    shapes: PartShapes,
    ids: Vec<BodyId>,
    impacts: Vec<(usize, f32)>,
}

impl Bridge {
    fn new(f: &Fixture, t: &Takeover) -> Self {
        let shapes = part_shapes(&f.body.table, &t.build, &f.skeleton);
        let mut world = PhysicsWorld::new(WorldPos::ORIGIN, PhysicsParams::default());
        let ids = shape_frames(&t.frames(), &shapes.turns)
            .iter()
            .zip(&shapes.set.shapes)
            .enumerate()
            .map(|(k, ((c, r), s))| {
                let pose = WorldTransform { pos: WorldPos::ORIGIN.offset(to_world(*c)), rotation: world_rotation() * *r, scale: Vec3::ONE };
                let shape = ShapeRef::Capsule { r: s.capsule.radius, half: s.capsule.half };
                world.insert(&BodyDesc::new(BodyKind::Kinematic, pose, shape, Layer::BODY_PROXY).with_user(k as u64))
            })
            .collect();
        Self { world, shapes, ids, impacts: Vec::new() }
    }

    /// A 3 kg, 10 cm ball at 12 m/s along `dir` (sim), aimed at part `part`'s centre from 1.5 m.
    fn throw(&mut self, t: &Takeover, part: usize, dir: Vec3) {
        let target = to_world(t.frames()[part].translation);
        let dir = to_world(dir).normalize();
        // Lifted by the drop over the flight (1.5 m at 12 m/s).
        let from = target - dir * 1.5 + Vec3::Y * 0.08;
        let desc = BodyDesc { mass: Some(3.0), ccd: true, ..BodyDesc::new(BodyKind::Dynamic { asleep: false }, WorldTransform::from_pos(WorldPos::ORIGIN.offset(from)), ShapeRef::Sphere(0.1), Layer::PROP) };
        let id = self.world.insert(&desc);
        self.world.set_velocity(id, dir * 12.0, Vec3::ZERO);
    }

    /// One tick: proxies to the parts, the physics step, its proxy hits (the strongest per ball) as pushes, the
    /// Delirium step.
    fn tick(&mut self, t: &mut Takeover) {
        for (id, (c, r)) in self.ids.iter().zip(shape_frames(&t.frames(), &self.shapes.turns)) {
            self.world.set_kinematic_target(*id, WorldTransform { pos: WorldPos::ORIGIN.offset(to_world(c)), rotation: world_rotation() * r, scale: Vec3::ONE });
        }
        self.world.step();
        let mut best: Vec<(BodyId, usize, f32, WorldPos, Vec3)> = Vec::new();
        let events: Vec<PhysicsEvent> = self.world.drain_events().collect();
        for e in events {
            let PhysicsEvent::Hit { target, by, impulse, point, direction } = e else { continue };
            let Some(part) = self.ids.iter().position(|id| *id == target) else { continue };
            match best.iter_mut().find(|b| b.0 == by) {
                Some(b) if b.2 < impulse => *b = (by, part, impulse, point, direction),
                Some(_) => {}
                None => best.push((by, part, impulse, point, direction)),
            }
        }
        for (_, part, impulse, point, direction) in best {
            // Sim from world: the same rotation.
            t.push(part, to_world(WorldPos::ORIGIN.delta_f32(point)), to_world(direction * impulse));
            self.impacts.push((part, impulse));
        }
        t.step(DT);
    }
}

/// Runs `secs`, throwing at the chest (Spine3, part 10) along sim +X at each of `throws` (s); returns the centre of
/// mass travel along +X and the impacts.
fn run(f: &Fixture, throws: &[f32], secs: f32) -> (f32, Vec<(usize, f32)>) {
    let mut t = takeover(f);
    let mut b = Bridge::new(f, &t);
    let start = t.com().0;
    for tick in 0..(secs / DT) as u32 {
        let now = tick as f32 * DT;
        if throws.iter().any(|s| (s - now).abs() < DT * 0.5) {
            b.throw(&t, 10, Vec3::X);
        }
        b.tick(&mut t);
        assert!(t.frames().iter().all(|p| p.translation.is_finite()), "NaN at {now:.2} s");
    }
    (t.com().0.x - start.x, b.impacts)
}

/// PH-15: a ball at a held, staggering body hits its proxies and pushes it; a second ball pushes it further.
#[test]
fn balls_keep_pushing_a_held_body() {
    let Some(f) = fixture() else { return };
    let (one, hits_one) = run(f, &[0.2], 1.6);
    let (two, hits_two) = run(f, &[0.2, 0.6], 1.6);
    println!("one ball: {one:+.2} m, impacts {hits_one:?}; two balls: {two:+.2} m, impacts {hits_two:?}");
    // Each ball's first contact is the strong one; a ball that drops off the body may graze another part later (a new
    // contact start, a light push of its own).
    let strong = |hits: &[(usize, f32)]| hits.iter().filter(|h| h.1 > 10.0).count();
    assert_eq!(strong(&hits_one), 1, "one strong impact per ball: {hits_one:?}");
    assert_eq!(strong(&hits_two), 2, "the second ball hits the held body too: {hits_two:?}");
    assert!(hits_two.iter().all(|&(part, impulse)| part < 21 && impulse.is_finite()));
    assert!(one > 0.0, "pushed along the throw ({one:+.2} m)");
    assert!(two > one, "the second ball pushes it further ({two:+.2} vs {one:+.2} m)");
}

/// The proxies are where the body is drawn: the shapes posed from the copy-back's skeleton (`pose_to_bones` of the
/// parts, then each shape's `local` on its bone) agree with the proxies (the parts) within a few cm, standing and
/// mid-stagger.
#[test]
fn proxies_match_the_shown_pose() {
    let Some(f) = fixture() else { return };
    let mut t = takeover(f);
    let shapes = part_shapes(&f.body.table, &t.build, &f.skeleton);
    assert_eq!(shapes.set.len(), 21);
    assert!(shapes.set.shapes.iter().all(|s| s.capsule.radius > 0.0 && s.capsule.half >= 0.0 && s.local.0.is_finite() && s.local.1.is_finite()));
    let (r, half) = shapes.set.bound;
    assert!(r > 0.1 && r < 1.5 && half > 0.2 && half < 1.5, "bound {:?}", shapes.set.bound);
    let at = t.frames()[10].translation;
    t.push(10, at, Vec3::X * 40.0);
    let mut worst = 0.0_f32;
    for tick in 0..60 {
        if tick % 15 == 0 {
            let parts: Vec<Transform> = t.frames();
            let mut local = f.skeleton.rest().to_vec();
            pose_to_bones(&f.body.table, &f.skeleton, &parts, &authoring_from_model(), &mut local);
            let model = mw_delirium::pose::model_pose(&f.skeleton, &local);
            let posed = posed_frames(&shapes.set, &model, &authoring_from_model());
            for (k, ((a, _), (b, _))) in shape_frames(&parts, &shapes.turns).iter().zip(&posed).enumerate() {
                let d = a.distance(*b);
                if d > worst {
                    worst = d;
                    println!("t {:.2} s: {} off by {:.1} cm", tick as f32 * DT, mw_delirium::body::NAMES[k], d * 100.0);
                }
            }
        }
        t.step(DT);
    }
    println!("proxies vs the shown pose: worst {:.1} cm", worst * 100.0);
    assert!(worst < 0.05, "a proxy {:.1} cm off the drawn bone", worst * 100.0);
}
