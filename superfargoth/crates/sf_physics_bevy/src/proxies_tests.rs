//! Body proxies (character-shapes.md S3), headless: balls hit the shapes of a held body (one impact per contact
//! start, the strongest per ball and step), proxies follow the written poses, never touch the world, characters or
//! each other and are invisible to the mover's ground casts, and leave with their component. Sanity asserts; the sync
//! cost is a measurement (`cargo test -p sf_physics_bevy --features controller proxy_sync_cost -- --ignored --nocapture`).

#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]

use std::sync::Arc;
use std::time::Duration;

use bevy_app::{App, Update};
use bevy_ecs::prelude::*;
use bevy_math::{IVec3, Quat, Vec3};
use bevy_time::{Fixed, Time, TimePlugin, TimeUpdateStrategy, Virtual};
use sf_anim::SkeletonId;
use sf_controller::{GroundProbe, ProbeQuery};
use sf_coords::{WorldPos, WorldTransform};
use sf_physics::{BodyDesc, BodyId, BodyKind, CharacterShape, Layer, PhysicsEvent, PushParams, ShapeRef};
use sf_ragdoll::{BoneShape, Capsule, ShapeSet};
use sf_world::WorldspaceId;
use sf_world_bevy::ActiveWorldspace;

use crate::proxies::Proxies;
use crate::{BodyProxies, ImpactOnCharacter, Physics, PhysicsEvents, PhysicsPlugin, ProxyHolder, ProxyStats, ShapeSystems};

fn base() -> WorldPos {
    WorldPos::new(IVec3::new(1, 0, 2), Vec3::new(5.0, 10.0, -3.0))
}

/// Upright capsules at `(x, y, z)` offsets from [`base`]: (offset, radius, half).
fn set(shapes: &[(Vec3, f32, f32)]) -> (Arc<ShapeSet>, Vec<WorldTransform>) {
    let set = ShapeSet {
        skeleton: SkeletonId(0),
        shapes: shapes
            .iter()
            .enumerate()
            .map(|(i, &(_, radius, half))| BoneShape {
                joint: i as u16 * 2,
                capsule: Capsule { radius, half },
                local: (Vec3::ZERO, Quat::IDENTITY),
                region: 0,
                hurt_scale: 1.0,
            })
            .collect(),
        bound: (0.5, 0.5),
    };
    let poses = shapes.iter().map(|&(at, ..)| WorldTransform::from_pos(base().offset(at))).collect();
    (Arc::new(set), poses)
}

#[derive(Resource, Default)]
struct Seen(Vec<ImpactOnCharacter>);

fn record(mut impacts: MessageReader<ImpactOnCharacter>, mut seen: ResMut<Seen>) {
    seen.0.extend(impacts.read().cloned());
}

/// A physics app with a floor 1 m under [`base`] (which creates the world).
fn app() -> (App, WorldspaceId) {
    let ws = WorldspaceId::new("proxies");
    let mut app = App::new();
    app.add_plugins((TimePlugin, PhysicsPlugin::default()))
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(1.0 / 60.0)))
        .insert_resource(ActiveWorldspace(Some(ws.clone())))
        .init_resource::<Seen>()
        .add_message::<ImpactOnCharacter>()
        .add_systems(Update, record.after(ShapeSystems::Activate));
    let floor = BodyDesc::fixed(WorldTransform::from_pos(base().offset(Vec3::new(0.0, -1.5, 0.0))), ShapeRef::Box(Vec3::new(20.0, 0.5, 20.0)), Layer::STATIC);
    app.world_mut().resource_mut::<Physics>().insert(&ws, &floor);
    (app, ws)
}

fn spawn(app: &mut App, shapes: &[(Vec3, f32, f32)]) -> Entity {
    let (set, poses) = set(shapes);
    let velocities = vec![Vec3::ZERO; poses.len()];
    let root = app.world_mut().spawn(BodyProxies { set, poses, velocities, at: 0.0, holder: ProxyHolder::Layer }).id();
    app.update();
    root
}

/// A 3 kg ball (radius `r`) at `from` (offset from [`base`]) flying at `velocity`.
fn ball(app: &mut App, ws: &WorldspaceId, from: Vec3, r: f32, velocity: Vec3) -> BodyId {
    let desc = BodyDesc { mass: Some(3.0), ccd: true, ..BodyDesc::new(BodyKind::Dynamic { asleep: false }, WorldTransform::from_pos(base().offset(from)), ShapeRef::Sphere(r), Layer::PROP) };
    let mut physics = app.world_mut().resource_mut::<Physics>();
    let w = physics.worlds.get_mut(ws).unwrap();
    let id = w.insert(&desc);
    w.set_velocity(id, velocity, Vec3::ZERO);
    id
}

fn run(app: &mut App, frames: usize) {
    for _ in 0..frames {
        app.update();
    }
}

#[test]
fn balls_at_a_held_body_each_hit_its_shapes() {
    let (mut app, ws) = app();
    // A torso, and two thin legs side by side 24 cm apart.
    let root = spawn(&mut app, &[(Vec3::new(0.0, 0.3, 0.0), 0.15, 0.25), (Vec3::new(0.0, -0.5, 0.12), 0.08, 0.3), (Vec3::new(0.0, -0.5, -0.12), 0.08, 0.3)]);
    assert_eq!(app.world().resource::<ProxyStats>().proxies, 3);

    // Two balls at the torso, 0.25 s apart: two impacts on shape 0.
    ball(&mut app, &ws, Vec3::new(-2.0, 0.35, 0.0), 0.1, Vec3::X * 12.0);
    run(&mut app, 15);
    ball(&mut app, &ws, Vec3::new(-2.0, 0.45, 0.0), 0.1, Vec3::X * 12.0);
    run(&mut app, 30);
    let seen = std::mem::take(&mut app.world_mut().resource_mut::<Seen>().0);
    assert_eq!(seen.len(), 2, "one impact per ball: {seen:?}");
    for hit in &seen {
        assert_eq!((hit.entity, hit.shape), (root, Some(0)));
        assert!(hit.impulse > 20.0 && hit.impulse < 40.0, "3 kg at 12 m/s: {}", hit.impulse);
        assert!(hit.direction.x > 0.9, "along the throw: {:?}", hit.direction);
    }

    // A ball between the legs touches both in one step: one impact, on either leg.
    ball(&mut app, &ws, Vec3::new(-2.0, -0.6, 0.0), 0.1, Vec3::X * 12.0);
    run(&mut app, 30);
    let seen = std::mem::take(&mut app.world_mut().resource_mut::<Seen>().0);
    assert_eq!(seen.len(), 1, "the strongest of the step only: {seen:?}");
    assert!(matches!(seen[0].shape, Some(1 | 2)));
}

#[test]
fn proxies_follow_the_written_poses() {
    let (mut app, ws) = app();
    let root = spawn(&mut app, &[(Vec3::ZERO, 0.1, 0.2)]);
    let id = app.world().resource::<Proxies>().ids(root).unwrap()[0];
    let v = Vec3::new(1.5, 0.0, 0.5);
    let mut worst = 0.0_f32;
    for frame in 0..120 {
        // The writer: the pose at this frame's virtual time (a held body's interpolated pose is a step behind it).
        let t = app.world().resource::<Time<Virtual>>().elapsed_secs_f64();
        let dt = app.world().resource::<Time<Fixed>>().timestep().as_secs_f64();
        let at = t - dt;
        let mut b = app.world_mut().get_mut::<BodyProxies>(root).unwrap();
        b.poses[0] = WorldTransform::from_pos(base().offset(v * at as f32));
        b.velocities[0] = v;
        b.at = at;
        app.update();
        if frame > 10 {
            // After the step: the body at the step's end, the written pose extrapolated there.
            let now = app.world().resource::<Time<Fixed>>().elapsed_secs_f64();
            let body = app.world().resource::<Physics>().world(&ws).unwrap().pose(id).unwrap();
            worst = worst.max(body.pos.delta_f32(base().offset(v * now as f32)).length());
        }
    }
    assert!(worst < 0.03, "proxy {worst:.3} m off its extrapolated pose");
}

#[test]
fn proxies_touch_only_what_moves() {
    let (mut app, ws) = app();
    // Two overlapping proxies sunk into the floor and through a character capsule.
    let root = spawn(&mut app, &[(Vec3::new(0.0, -1.0, 0.0), 0.2, 0.3), (Vec3::new(0.1, -1.0, 0.0), 0.2, 0.3)]);
    let character = BodyDesc::character(base().offset(Vec3::new(0.0, -1.0, 0.0)), CharacterShape { radius: 0.3, height: 1.8 }, &PushParams::default());
    app.world_mut().resource_mut::<Physics>().insert(&ws, &character);
    let ids = app.world().resource::<Proxies>().ids(root).unwrap().to_vec();
    for _ in 0..10 {
        app.update();
        let events = &app.world().resource::<PhysicsEvents>().0;
        let touches = |a: BodyId, b: BodyId| ids.contains(&a) || ids.contains(&b);
        let bad = events.iter().any(|e| match *e {
            PhysicsEvent::Contact { a, b, .. } | PhysicsEvent::ContactForce { a, b, .. } => touches(a, b),
            PhysicsEvent::Hit { target, by, .. } => touches(target, by),
            _ => false,
        });
        assert!(!bad, "a proxy touched the world, a capsule or a proxy: {events:?}");
    }
    // The mover's ground cast through a proxy finds the floor under it.
    let hit = app.world().resource::<Physics>().probe_one(ProbeQuery::down(base().offset(Vec3::new(0.0, -0.3, 0.0)), 3.0)).expect("the floor");
    assert!((base().delta_f32(hit.point).y + 1.0).abs() < 0.01, "ground cast hit {:?}", base().delta_f32(hit.point));
    assert!(app.world().resource::<Seen>().0.is_empty());
}

#[test]
fn proxies_leave_with_their_component() {
    let (mut app, ws) = app();
    let root = spawn(&mut app, &[(Vec3::ZERO, 0.1, 0.2), (Vec3::Y, 0.1, 0.2)]);
    let ids = app.world().resource::<Proxies>().ids(root).unwrap().to_vec();
    assert!(ids.iter().all(|id| app.world().resource::<Physics>().world(&ws).unwrap().contains(*id)));
    app.world_mut().entity_mut(root).remove::<BodyProxies>();
    app.update();
    assert!(ids.iter().all(|id| !app.world().resource::<Physics>().world(&ws).unwrap().contains(*id)), "bodies left behind");
    assert_eq!(app.world().resource::<ProxyStats>().proxies, 0);
    // Despawning works the same way.
    let root = spawn(&mut app, &[(Vec3::ZERO, 0.1, 0.2)]);
    let ids = app.world().resource::<Proxies>().ids(root).unwrap().to_vec();
    app.world_mut().despawn(root);
    app.update();
    assert!(!app.world().resource::<Physics>().world(&ws).unwrap().contains(ids[0]));
}

/// Sync cost: N held sets of 21 proxies (a Delirium body), each moving; µs per tick, per set and per proxy, and the
/// step's own cost with and without them (no balls: the broadphase update only).
#[test]
#[ignore = "measurement: proxy sync cost"]
fn proxy_sync_cost() {
    for sets in [1usize, 4, 16] {
        let (mut app, _) = app();
        let shapes: Vec<(Vec3, f32, f32)> = (0..21).map(|k| (Vec3::new(0.0, k as f32 * 0.08, 0.0), 0.07, 0.12)).collect();
        let roots: Vec<Entity> = (0..sets)
            .map(|s| {
                let (set, mut poses) = set(&shapes);
                for p in &mut poses {
                    p.pos = p.pos.offset(Vec3::X * s as f32 * 1.5);
                }
                let velocities = vec![Vec3::X * 0.5; poses.len()];
                app.world_mut().spawn(BodyProxies { set, poses, velocities, at: 0.0, holder: ProxyHolder::Layer }).id()
            })
            .collect();
        run(&mut app, 30);
        let (mut sync, mut step, mut n) = (0.0_f32, 0.0_f32, 0);
        for _ in 0..300 {
            let at = app.world().resource::<Time<Virtual>>().elapsed_secs_f64();
            for r in &roots {
                app.world_mut().get_mut::<BodyProxies>(*r).unwrap().at = at;
            }
            app.update();
            sync += app.world().resource::<ProxyStats>().sync_us;
            step += app.world().resource::<crate::PhysicsStats>().step_ms * 1e3;
            n += 1;
        }
        let (sync, step) = (sync / n as f32, step / n as f32);
        println!(
            "proxies: {sets} sets x 21: sync {sync:.1} us/tick ({:.2} us/set, {:.3} us/proxy), step {step:.1} us",
            sync / sets as f32,
            sync / (sets * 21) as f32
        );
    }
}
