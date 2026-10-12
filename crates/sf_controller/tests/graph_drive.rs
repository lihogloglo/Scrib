//! The controller drives a real `sf_anim_graph` graph (locomotion.md
//! "Verification"): speed matching from the controller's velocity, foot
//! placement from probe hits converted to character space, and `Drive`
//! root motion fed back into the mover. Only public APIs; the controller
//! doesn't depend on the graph, so this is the glue `sf_anim_bevy` /
//! `sf_controller_bevy` will run per frame.
#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::many_single_char_names)]

use std::f32::consts::{FRAC_PI_2, TAU};
use std::sync::Arc;

use glam::{IVec3, Quat, Vec3};
use sf_anim::{
    Clip, ClipRules, ContactRules, CookSettings, Curve, Looping, RootMotionMode, RootMotionRules, Skeleton, SkeletonRules, SourceClip, SourceJoint,
    SourceTrack, Trs, cook_clip,
};
use sf_anim_graph::asset::GraphAsset;
use sf_anim_graph::{Binding, CompileSource, CompiledGraph, GraphInstance, GraphOutput, GroundSample, UpdateInputs, WarpTarget, compile_str};
use sf_bake::ProductKey;
use sf_controller::{
    Affordance, CollisionBuilder, ContextParams, ContextProbes, ContextQuery, ContextRequest, GroundHit, GroundProbe, KinematicMover, Locomotion,
    LocomotionIntent, MoveInput, MoverParams, Nearest, ProbeQuery, RootMotion, StaticShape, StaticWorld, SurfaceId, TerrainProbe,
};
use sf_coords::{WorldPos, WorldTransform};

// --- Fixtures (a 17-joint biped facing +Z, as in sf_anim_graph's tests) -------------------

fn j(name: &str, parent: Option<usize>, t: [f32; 3]) -> SourceJoint {
    SourceJoint { name: name.to_owned(), parent, rest: Trs::from_translation(Vec3::from(t)) }
}

fn biped() -> Skeleton {
    let v = vec![
        j("root", None, [0.0, 0.0, 0.0]),
        j("hips", Some(0), [0.0, 1.0, 0.0]),
        j("spine", Some(1), [0.0, 0.1, 0.0]),
        j("chest", Some(2), [0.0, 0.25, 0.0]),
        j("head", Some(3), [0.0, 0.3, 0.0]),
        j("thigh_l", Some(1), [0.1, -0.05, 0.0]),
        j("shin_l", Some(5), [0.0, -0.45, 0.02]),
        j("foot_l", Some(6), [0.0, -0.45, -0.02]),
        j("thigh_r", Some(1), [-0.1, -0.05, 0.0]),
        j("shin_r", Some(8), [0.0, -0.45, 0.02]),
        j("foot_r", Some(9), [0.0, -0.45, -0.02]),
    ];
    let rules = SkeletonRules { root_motion_joint: Some("root".into()), ..SkeletonRules::default() };
    Skeleton::with_rules(&v, &rules).unwrap().0
}

fn sampled<T: sf_anim::CurveValue>(duration: f32, f: impl Fn(f32) -> T) -> Curve<T> {
    let n = (duration / 0.025).round() as usize;
    let times: Vec<f32> = (0..=n).map(|i| duration * i as f32 / n as f32).collect();
    let values = times.iter().map(|&t| f(t)).collect();
    Curve::linear(times, values)
}

fn clip(s: &Skeleton, name: &str, d: f32, looping: bool, mode: RootMotionMode, tracks: Vec<SourceTrack>) -> Clip {
    let src = SourceClip { name: name.into(), duration: d, tracks, ..SourceClip::default() };
    let rules = ClipRules {
        looping: if looping { Looping::Whole } else { Looping::Once },
        root_motion: RootMotionRules { mode, ..RootMotionRules::default() },
        contacts: ContactRules { enabled: false, ..ContactRules::default() },
        ..ClipRules::default()
    };
    cook_clip(s, &src, &rules, &CookSettings::default()).unwrap()
}

fn root_forward(d: f32, speed: f32) -> SourceTrack {
    SourceTrack { joint: "root".into(), translation: Some(sampled(d, move |t| Vec3::new(0.0, 0.0, speed * t))), ..SourceTrack::default() }
}

fn clips(s: &Skeleton) -> Vec<Clip> {
    let swing = |joint: &str, sign: f32| SourceTrack {
        joint: joint.into(),
        rotation: Some(sampled(1.0, move |t| Quat::from_rotation_x(sign * 0.5 * (TAU * t).sin()))),
        ..SourceTrack::default()
    };
    vec![
        // Walk: 1.2 m/s, capsule-driven (`Match`).
        clip(s, "walk", 1.0, true, RootMotionMode::Match, vec![root_forward(1.0, 1.2), swing("thigh_l", 1.0), swing("thigh_r", -1.0)]),
        clip(s, "idle", 2.0, true, RootMotionMode::Ignore, vec![]),
        // Attack: lunges 0.6 m/s for 0.8 s, root-motion driven.
        clip(s, "attack", 0.8, false, RootMotionMode::Drive, vec![root_forward(0.8, 0.6)]),
        mantle(s),
    ]
}

/// Mantle: the root rises 1 m over 0.5 s, then moves 0.9 m forward over
/// 0.5 s; one `y=warp` window over [0, 1] onto the "ledge" target.
fn mantle(s: &Skeleton) -> Clip {
    let root = SourceTrack {
        joint: "root".into(),
        translation: Some(sampled(1.2, |t| Vec3::new(0.0, (t / 0.5).min(1.0), 0.9 * ((t - 0.5) / 0.5).clamp(0.0, 1.0)))),
        ..SourceTrack::default()
    };
    let src = SourceClip {
        name: "mantle".into(),
        duration: 1.2,
        tracks: vec![root],
        events: vec![sf_anim::Event { time: 0.0, end: Some(1.0), name: "warp".into(), payload: "ledge;skew;y=warp;rot=none".into() }],
        ..SourceClip::default()
    };
    let rules = ClipRules {
        root_motion: RootMotionRules { mode: RootMotionMode::Drive, vertical: true, ..RootMotionRules::default() },
        contacts: ContactRules { enabled: false, ..ContactRules::default() },
        ..ClipRules::default()
    };
    cook_clip(s, &src, &rules, &CookSettings::default()).unwrap()
}

struct Source(Vec<Clip>);

impl CompileSource for Source {
    fn clip(&mut self, reference: &str) -> Result<(ProductKey, Clip), String> {
        let c = self.0.iter().find(|c| c.name == reference).cloned().ok_or_else(|| format!("no clip {reference}"))?;
        Ok((ProductKey::new("anim.cook", reference), c))
    }
    fn subgraph(&mut self, _path: &str) -> Result<GraphAsset, String> {
        Err("no subgraphs".into())
    }
}

struct Rig {
    skeleton: Skeleton,
    graph: CompiledGraph,
    clips: Vec<Arc<Clip>>,
}

impl Rig {
    fn new(nodes: &str, procedural: &str) -> Self {
        let ron = format!(
            r#"(
                nodes: {{ {nodes} }},
                warp_targets: ["ledge"],
                layers: [(node: "main")],
                legs: [
                    (name: "l", hip: Joint("thigh_l"), knee: Joint("shin_l"), foot: Joint("foot_l")),
                    (name: "r", hip: Joint("thigh_r"), knee: Joint("shin_r"), foot: Joint("foot_r")),
                ],
                procedural: [{procedural}],
            )"#
        );
        let skeleton = biped();
        let mut src = Source(clips(&skeleton));
        let graph = compile_str("test", &ron, &skeleton, &mut src).unwrap();
        let clips = graph.clips.iter().map(|c| Arc::new(src.0.iter().find(|x| x.name == c.reference).unwrap().clone())).collect();
        Self { skeleton, graph, clips }
    }

    fn binding(&self) -> Binding<'_> {
        Binding::new(&self.graph, &self.skeleton, &self.clips).unwrap()
    }
}

// --- World -------------------------------------------------------------------------------

struct Flat;

impl sf_controller::HeightSurface for Flat {
    fn height(&self, _p: WorldPos) -> Option<f32> {
        Some(0.0)
    }
    fn normal(&self, _p: WorldPos) -> Option<Vec3> {
        Some(Vec3::Y)
    }
    fn raycast(&self, _o: WorldPos, _d: Vec3, _m: f32) -> Option<(f32, Vec3)> {
        None
    }
}

struct World {
    terrain: TerrainProbe<Flat>,
    statics: StaticWorld,
}

impl GroundProbe for World {
    fn probe(&self, queries: &[ProbeQuery], out: &mut [Option<GroundHit>]) {
        Nearest(&[&self.terrain, &self.statics]).probe(queries, out);
    }
}

fn world() -> World {
    World { terrain: TerrainProbe(Flat), statics: StaticWorld::new(WorldPos::ORIGIN) }
}

fn y(p: WorldPos) -> f32 {
    WorldPos::ORIGIN.delta_f32(p).y
}

/// One frame of glue: mover (with last update's root motion), foot probes at
/// last update's probe origins in the new frame, graph update.
struct Character<'r> {
    rig: &'r Rig,
    instance: GraphInstance,
    out: GraphOutput,
    loco: Locomotion,
    mover: KinematicMover,
    /// Per warp slot: a world target (position, yaw), converted into the
    /// character frame every update.
    warp: Vec<Option<(WorldPos, f32)>>,
}

impl<'r> Character<'r> {
    fn new(rig: &'r Rig, at: WorldPos, yaw: f32) -> Self {
        let b = rig.binding();
        Self {
            rig,
            instance: GraphInstance::new(&b),
            out: GraphOutput::new(&rig.graph),
            loco: Locomotion::new(at, yaw),
            mover: KinematicMover::new(),
            warp: vec![None; rig.graph.warp_targets.len()],
        }
    }

    fn frame(&mut self, world: &World, intent: LocomotionIntent, params: &MoverParams, dt: f32) {
        let root = RootMotion { delta: self.out.root, drive_weight: self.out.drive_weight, traversal: self.out.warp.is_some_and(|w| w.vertical) };
        self.mover.step(MoveInput { loco: &mut self.loco, intent, params, root }, world, dt);
        let frame = self.loco.frame();
        let queries: Vec<ProbeQuery> = frame.foot_queries(&self.out.probe_origins, 0.5, 0.5).collect();
        let mut hits = vec![None; queries.len()];
        world.probe(&queries, &mut hits);
        let mut ground: Vec<Option<GroundSample>> = hits
            .iter()
            .map(|h| {
                h.map(|h| {
                    let (point, normal) = frame.ground_sample(&h);
                    GroundSample { point, normal }
                })
            })
            .collect();
        ground.resize(self.rig.graph.legs.len(), None);
        let p = self.loco.params();
        let warp: Vec<Option<WarpTarget>> = self
            .warp
            .iter()
            .map(|t| t.map(|(at, yaw)| WarpTarget { position: frame.to_local(at), yaw: yaw - frame.yaw, velocity: Vec3::ZERO }))
            .collect();
        let inputs =
            UpdateInputs { velocity: p.local_velocity, moved: self.loco.moved, ground: &ground, warp_targets: &warp, ..UpdateInputs::default() };
        self.instance.update(&self.rig.binding(), dt, 0, &inputs, &mut self.out);
    }

    /// A joint's world height.
    fn joint_y(&self, name: &str) -> f32 {
        let j = self.rig.skeleton.find(name).unwrap();
        y(self.loco.frame().to_world(self.out.model.translation(j)))
    }
}

const DT: f32 = 1.0 / 60.0;

#[test]
fn speed_match_follows_the_controller_velocity() {
    let rig = Rig::new(r#""main": Clip(clip: "walk")"#, "SpeedMatch()");
    let w = world();
    let params = MoverParams::default();
    let mut c = Character::new(&rig, WorldPos::ORIGIN, FRAC_PI_2);
    let intent = LocomotionIntent { desired_velocity: Vec3::new(1.5, 0.0, 0.0), ..LocomotionIntent::IDLE };
    let node = rig.graph.node("main").unwrap();
    let time = |c: &Character<'_>| c.instance.player(node).unwrap().0[0].0;
    for _ in 0..180 {
        c.frame(&w, intent, &params, DT);
    }
    let t0 = time(&c);
    c.frame(&w, intent, &params, DT);
    let advanced = (time(&c) - t0).rem_euclid(1.0);
    // 1.5 m/s on a 1.2 m/s clip plays at 1.25×.
    assert!((advanced - DT * 1.25).abs() < 1e-3 * DT + 1e-5, "advanced {advanced} s per frame");
    // `moved` is forward in character space at the actual speed.
    assert!((c.loco.moved.translation.z - 1.5 * DT).abs() < 1e-3, "{:?}", c.loco.moved);
    assert!(c.loco.moved.translation.x.abs() < 1e-4);
}

#[test]
fn foot_placement_lands_on_ground_the_controller_probed() {
    let rig = Rig::new(r#""main": Clip(clip: "idle")"#, r#"FootPlacement(pelvis: Joint("hips"), legs: ["l", "r"])"#);
    let plain = Rig::new(r#""main": Clip(clip: "idle")"#, "");
    let mut w = world();
    // Turned 1 rad, standing still; a 0.1 m block under the right foot only.
    let (at, yaw) = (WorldPos::new(IVec3::new(3, 0, -2), Vec3::new(10.0, 0.0, 20.0)), 1.0);
    let foot_r = Quat::from_rotation_y(yaw) * Vec3::new(-0.1, 0.0, 0.0);
    let mut b = CollisionBuilder::new(1e-3);
    let h = 0.05;
    let corners = [[-h, 0.0, -h], [h, 0.0, -h], [h, 0.0, h], [-h, 0.0, h], [-h, 0.1, -h], [h, 0.1, -h], [h, 0.1, h], [-h, 0.1, h]];
    b.add(
        &corners,
        &[4, 5, 6, 4, 6, 7, 0, 1, 5, 0, 5, 4, 1, 2, 6, 1, 6, 5, 2, 3, 7, 2, 7, 6, 3, 0, 4, 3, 4, 7],
        glam::Affine3A::IDENTITY,
        SurfaceId(1),
    );
    let block = Arc::new(StaticShape::new(&b.finish()).unwrap());
    w.statics.insert(&block, WorldTransform::from_pos(at.offset(foot_r)));
    let params = MoverParams::default();
    let (mut c, mut p) = (Character::new(&rig, at, yaw), Character::new(&plain, at, yaw));
    for _ in 0..30 {
        c.frame(&w, LocomotionIntent::IDLE, &params, DT);
        p.frame(&w, LocomotionIntent::IDLE, &params, DT);
    }
    assert!(c.loco.grounded.is_some() && y(c.loco.position).abs() < 1e-5, "the body stands on the floor, beside the block");
    let (l, r) = (c.joint_y("foot_l"), c.joint_y("foot_r"));
    let (pl, pr) = (p.joint_y("foot_l"), p.joint_y("foot_r"));
    assert!((r - (pr + 0.1)).abs() < 0.01, "right foot {r}, animated {pr}: should stand on the block");
    assert!((l - pl).abs() < 0.01, "left foot {l}, animated {pl}");
}

#[test]
fn drive_root_motion_flows_back_into_the_mover() {
    let rig = Rig::new(r#""main": Clip(clip: "attack", root_motion: Drive)"#, "");
    let w = world();
    let params = MoverParams::default();
    let mut c = Character::new(&rig, WorldPos::ORIGIN, FRAC_PI_2);
    for _ in 0..60 {
        c.frame(&w, LocomotionIntent::IDLE, &params, DT);
    }
    let at = WorldPos::ORIGIN.delta_f32(c.loco.position);
    // 0.6 m/s for 0.8 s along the facing (+X), one frame late.
    assert!((at.x - 0.48).abs() < 0.015 && at.z.abs() < 1e-3, "lunged to {at}");
    assert!((c.out.drive_weight - 1.0).abs() < 1e-6);
}

#[test]
fn a_warped_mantle_lands_on_the_box_through_the_traversal_flag() {
    let rig = Rig::new(r#""main": Clip(clip: "mantle", root_motion: Drive)"#, "");
    let mut w = world();
    // A 1.2 m box ahead (+Z), its face 0.9 m away: taller than `step_up`.
    let (half, center) = (Vec3::new(1.0, 0.6, 0.55), Vec3::new(0.0, 0.6, 1.45));
    let mut b = CollisionBuilder::new(1e-3);
    let corners: Vec<[f32; 3]> = (0..8)
        .map(|i| [[-1.0, 0.0, -1.0], [1.0, 0.0, -1.0], [1.0, 0.0, 1.0], [-1.0, 0.0, 1.0]][i % 4])
        .enumerate()
        .map(|(i, [x, _, z])| (Vec3::new(x, if i < 4 { -1.0 } else { 1.0 }, z) * half).to_array())
        .collect();
    let sides = [4, 5, 6, 4, 6, 7, 0, 1, 5, 0, 5, 4, 1, 2, 6, 1, 6, 5, 2, 3, 7, 2, 7, 6, 3, 0, 4, 3, 4, 7];
    b.add(&corners, &sides, glam::Affine3A::IDENTITY, SurfaceId::DEFAULT);
    let shape = Arc::new(StaticShape::new(&b.finish()).unwrap());
    w.statics.insert(&shape, WorldTransform::from_pos(WorldPos::ORIGIN.offset(center)));
    let params = MoverParams::default();
    let mut c = Character::new(&rig, WorldPos::ORIGIN, 0.0);
    let ledge = WorldPos::ORIGIN.offset(Vec3::new(0.0, 1.2, 1.5));
    c.warp[rig.graph.warp_target("ledge").unwrap()] = Some((ledge, 0.0));
    let mut saw_traversal = false;
    for _ in 0..90 {
        c.frame(&w, LocomotionIntent::IDLE, &params, DT);
        saw_traversal |= c.loco.traversing;
        let at = WorldPos::ORIGIN.delta_f32(c.loco.position);
        // The swept capsule (from `step_up` to the top) never enters the box.
        if at.y + params.step_up < 1.2 {
            assert!(at.z + params.radius <= 0.9 + params.skin + 1e-3, "capsule in the box at {at}");
        }
    }
    let at = WorldPos::ORIGIN.delta_f32(c.loco.position);
    println!("mantle: feet at {at}");
    assert!(saw_traversal && !c.loco.traversing && c.loco.grounded.is_some(), "{:?}", c.loco.grounded);
    assert!(at.distance(Vec3::new(0.0, 1.2, 1.5)) <= 0.02, "feet {at}");
}

/// Context probes feed motion warping: an obstacle probe classifies the box
/// as a mantle, its warp target (on top, past the lip) drives the same
/// `y=warp` window, and the feet land on the probed target within the warp
/// gate (animation.md: ≤ 2 cm at the window's end).
#[test]
fn a_probed_mantle_target_lands_the_warp_on_the_ledge() {
    let rig = Rig::new(r#""main": Clip(clip: "mantle", root_motion: Drive)"#, "");
    let mut w = world();
    // A 1.2 m box ahead (+Z), its face 0.9 m away, 1.1 m deep.
    let mut b = CollisionBuilder::new(1e-3);
    let (lo, hi) = (Vec3::new(-1.0, 0.0, 0.9), Vec3::new(1.0, 1.2, 2.0));
    let corners: Vec<[f32; 3]> =
        (0..8).map(|i| [if i & 1 == 0 { lo.x } else { hi.x }, if i & 2 == 0 { lo.y } else { hi.y }, if i & 4 == 0 { lo.z } else { hi.z }]).collect();
    let faces = [0, 1, 3, 0, 3, 2, 4, 6, 7, 4, 7, 5, 0, 4, 5, 0, 5, 1, 2, 3, 7, 2, 7, 6, 0, 2, 6, 0, 6, 4, 1, 5, 7, 1, 7, 3];
    b.add(&corners, &faces, glam::Affine3A::IDENTITY, SurfaceId::DEFAULT);
    w.statics.insert(&Arc::new(StaticShape::new(&b.finish()).unwrap()), WorldTransform::IDENTITY);
    let params = MoverParams::default();
    let mut c = Character::new(&rig, WorldPos::ORIGIN, 0.0);

    let context = ContextParams::from_mover(&params);
    let request = ContextRequest { feet: c.loco.position, dir: Vec3::Z, params: &context, query: ContextQuery::ObstacleAhead { range: 2.0 } };
    let mut hits = [None];
    ContextProbes::new().run(&[request], &w, &mut hits);
    let hit = hits[0].expect("the box is found");
    assert_eq!(hit.kind, Affordance::Mantle, "{hit:?}");
    let (target, yaw) = hit.warp_target(0.6);
    assert!(y(target) > 1.19 && WorldPos::ORIGIN.delta_f32(target).distance(Vec3::new(0.0, 1.2, 1.5)) < 0.01, "{target:?}");
    c.warp[rig.graph.warp_target("ledge").unwrap()] = Some((target, yaw));

    for _ in 0..90 {
        c.frame(&w, LocomotionIntent::IDLE, &params, DT);
    }
    let at = c.loco.position.delta_f32(target);
    println!("probed mantle: feet {at} from the probed target");
    assert!(!c.loco.traversing && c.loco.grounded.is_some() && at.length() <= 0.02, "feet {at} from the target");
}
