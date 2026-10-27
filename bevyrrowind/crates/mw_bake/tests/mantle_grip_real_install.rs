//! Hands on the lip (action.md B2b) end to end: the real baked humanoid graph plays the UAL2
//! mantles progress-driven along their warp (planned by `sf_action` with the shipped rules, as the
//! game does), with the grip targets and weights fed to its hand IK the way
//! `mw_player_bevy::grip` feeds them. Sanity only (rule 5): finite, the held hands not inside the
//! box. Distances are printed by the ignored measurement (`-- --ignored --nocapture`).
//! Skips (passes) without a Morrowind install or the animation library.

use std::collections::BTreeSet;
use std::sync::Arc;

use glam::{Affine3A, Vec2, Vec3};
use mw_bake::VfsInputs;
use mw_bake::anim::{self, ActorAnimSet, ActorSets};
use mw_bake::sources::AnimSources;
use mw_graphs::build::{ik, params as p, slots};
use mw_graphs::{HUMANOID_SETS, Template};
use mw_install::MorrowindInstall;
use sf_action::{
    ActionClass, ActionOutput, ActionState, BASE_RULES, CharacterId, CharacterParams, Command,
    EventKind, LEFT, LedgeGrip, Obstacle, ObstacleClass, Origin, RIGHT, RuleKey, RuleSetBuilder,
    Sense, SlotInfo, SlotTable, StepCtx, TRAVERSAL_RULES, Tick, WarpState,
};
use sf_anim::{Clip, ContactKind, RootMotionMode, RootPath, RootPose, Side, Skeleton};
use sf_anim_graph::steps::{ANIMGRAPH_COMPILE, graph_subject};
use sf_anim_graph::{CompiledGraph, IkTarget, MontageClip, SlotRequest};
use sf_bake::{Baker, ProductKey, Store};
use sf_character::HumanoidBone as B;
use sf_vfs::Vfs;

const WAIST: &str = "traverse.mantle.waist";
const HIGH: &str = "traverse.mantle.high";
/// Box depth behind the face, m.
const DEPTH: f32 = 1.5;
const DT: f32 = 1.0 / 60.0;

/// The baked humanoid graph of `base_anim` with its clips, skeleton and rig map.
struct Baked {
    graph: CompiledGraph,
    skeleton: Skeleton,
    clips: Vec<Arc<Clip>>,
    _dir: tempfile::TempDir,
}

fn bake() -> Option<Baked> {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return None;
    };
    let sources = Arc::new(AnimSources::local());
    let ual2 = sources
        .references()
        .into_iter()
        .find(|(_, r)| r.reference == "quaternius_ual2:ClimbUp_1m");
    if ual2.is_none_or(|(file, _)| sources.real_path(&file).is_none()) {
        eprintln!("UAL2 not in {}; skipping", sources.dir.display());
        return None;
    }
    let vfs = Arc::new(
        mw_vfs::mount_install(&install, Vfs::builder().mount(mw_graphs::builtin_source()))
            .unwrap()
            .vfs,
    );
    let set = &HUMANOID_SETS[0];
    let mut sets = ActorSets::default();
    let s = ActorAnimSet {
        skeleton: set.skeleton.to_owned(),
        files: set.animations.iter().map(|f| (*f).to_owned()).collect(),
        humanoid: true,
        graphs: BTreeSet::from([Template::Humanoid.path().to_owned()]),
        actors: 1,
        ..ActorAnimSet::default()
    };
    sets.sets.insert(s.skeleton.clone(), s);
    let dir = tempfile::tempdir().unwrap();
    let store = Store::open(dir.path().join("store")).unwrap();
    let inputs = Arc::new(
        VfsInputs::new(vfs, &install, dir.path().join("filehash.memo"))
            .with_anim_sources(sources.clone()),
    );
    let mut baker = Baker::new(store, inputs);
    let report = anim::bake_animation(&mut baker, &sets, &sources);
    assert!(report.failures.is_empty(), "{:?}", report.failures);
    let bytes = |key: &ProductKey| {
        baker
            .store()
            .get(&baker.product(key).unwrap_or_else(|e| panic!("{key}: {e}")))
            .unwrap()
    };
    let graph = CompiledGraph::from_bytes(&bytes(&ProductKey::new(
        ANIMGRAPH_COMPILE,
        graph_subject(Template::Humanoid.path(), set.skeleton),
    )))
    .unwrap();
    let skeleton = Skeleton::from_bytes(&bytes(&ProductKey::new(
        sf_anim::steps::SKELETON_COOK,
        set.skeleton,
    )))
    .unwrap();
    let clips = graph
        .clips
        .iter()
        .map(|c| Arc::new(Clip::from_bytes(&bytes(&c.product)).unwrap()))
        .collect();
    Some(Baked {
        graph,
        skeleton,
        clips,
        _dir: dir,
    })
}

/// A box whose face is `dist` m ahead (+Z) of the feet, `height` m tall, [`DEPTH`] deep.
fn ledge(dist: f32, height: f32) -> Obstacle {
    Obstacle {
        class: ObstacleClass::Mantle,
        lip: Vec3::new(0.0, height, dist),
        normal: Vec3::NEG_Z,
        height,
        depth: DEPTH,
        clearance: 3.0,
    }
}

/// The warp `sf_action` plans for a jump forward at the ledge with the baked clip of `slot`.
fn plan(clip: &Arc<Clip>, slot: &str, dist: f32, height: f32) -> Option<WarpState> {
    let mut b = RuleSetBuilder::new();
    b.add_ron(&Origin::Framework, BASE_RULES)
        .add_ron(&Origin::Framework, TRAVERSAL_RULES);
    let rules = b.build().expect("rules load");
    let table = SlotTable::new()
        .with("loco.move", SlotInfo::looping())
        .with("air.fall", SlotInfo::looping())
        .with("air.jump", SlotInfo::once(0.3))
        .with("air.land.normal", SlotInfo::once(0.25))
        .with("air.land.hard", SlotInfo::once(0.5))
        .with(slot, SlotInfo::once(clip.duration))
        .with_root(slot, clip.clone());
    let params = CharacterParams::default();
    let mut state = ActionState::new(RuleKey::of("loco"));
    let mut out = ActionOutput::default();
    let mut cmd = Command {
        stick: Vec2::new(0.0, 1.0),
        tick: Tick(1),
        ..Command::default()
    };
    cmd.buttons.pressed.insert(ActionClass::Jump);
    cmd.buttons.held.insert(ActionClass::Jump);
    let ctx = StepCtx {
        rules: &rules,
        slots: &table,
        params: &params,
        source: CharacterId(3),
    };
    let sense = Sense {
        grounded: true,
        obstacle: Some(ledge(dist, height)),
        ..Sense::default()
    };
    sf_action::step(&ctx, &mut state, &cmd, &sense, &mut out);
    let started = out
        .events
        .iter()
        .any(|e| e.kind == EventKind::MoveStart(RuleKey::of(slot)));
    state.warp.filter(|_| started)
}

fn joint(b: &Baked, bone: B) -> usize {
    let map = mw_anim::rig::humanoid(&b.skeleton).unwrap().map;
    b.skeleton.find(map.joint(bone).expect("mapped")).unwrap()
}

fn arm_length(b: &Baked, bones: [B; 3]) -> f32 {
    let mut rest: Vec<Affine3A> = Vec::new();
    for (j, t) in b.skeleton.rest().iter().enumerate() {
        let m = t.to_affine();
        rest.push(b.skeleton.parent(j).map_or(m, |p| rest[p] * m));
    }
    let at = |bone| Vec3::from(rest[joint(b, bone)].translation);
    at(bones[0]).distance(at(bones[1])) + at(bones[1]).distance(at(bones[2]))
}

/// One hand's numbers over a mantle.
#[derive(Default)]
struct HandTrace {
    /// Hand to target through the full-weight hold, m (min, max).
    held: (f32, f32),
    /// Shoulder (upper arm) to target at the hold's start, m.
    shoulder: f32,
    /// Frames the hand was inside the box at full IK weight.
    inside: usize,
    /// The clip contact carried by the end-aligned root (the target pose), to the target, m.
    end_gap: f32,
    /// Frames inside the box on the weight ramps.
    inside_ramp: usize,
    /// Frames with a non-finite hand.
    nan: usize,
}

/// Plays `slot`'s clip along its warp at the ledge with hand IK fed from the grip: per hand,
/// `None` if the grip doesn't hold it.
#[allow(clippy::many_single_char_names)]
fn mantle(
    b: &Baked,
    slot: &str,
    dist: f32,
    height: f32,
) -> Option<(LedgeGrip, [Option<HandTrace>; 2])> {
    let clip = b.clips[usize::from(b.graph.slot_clip(slot)?)].clone();
    let w = plan(&clip, slot, dist, height)?;
    let grip = w.grip?;
    let mut sim =
        mw_graphs::sim::Sim::new(b.graph.clone(), b.skeleton.clone(), b.clips.clone()).unwrap();
    let full = sim.graph.slot(slots::FULL).expect("full-body slot");
    let req = SlotRequest {
        root_motion: RootMotionMode::Progress,
        blend_in: 0.0,
        priority: 1,
        ..SlotRequest::new(MontageClip::Bound(sim.graph.slot_clip(slot)?))
    };
    sim.play_slot(slots::FULL, req).unwrap();
    let hands = [
        (B::LeftUpperArm, B::LeftHand),
        (B::RightUpperArm, B::RightHand),
    ]
    .map(|(u, h)| (joint(b, u), joint(b, h)));
    // Held and skipped holds alike (a skipped hand stays animated; its reach is still measured).
    let all = [LEFT, RIGHT].map(|i| grip.hands[i].or(grip.skipped[i]));
    let mut traces: [Option<HandTrace>; 2] = [LEFT, RIGHT].map(|i| {
        all[i].map(|h| HandTrace {
            held: (f32::INFINITY, 0.0),
            end_gap: end_aligned_gap(&clip, &w, &h, i),
            ..HandTrace::default()
        })
    });
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_precision_loss
    )]
    let frames = (clip.duration / DT).ceil() as usize;
    for f in 0..=frames {
        #[allow(clippy::cast_precision_loss)]
        let t = (f as f32 * DT).min(clip.duration);
        // The drawn body: the warped root (yaw 0 facing +Z here); character space = root frame.
        let root: RootPose = w.plan.root_at(&*clip, t);
        let to_local =
            |world: Vec3| glam::Quat::from_rotation_y(-root.yaw) * (world - root.position);
        let to_world = |local: Vec3| root.position + glam::Quat::from_rotation_y(root.yaw) * local;
        let holds = grip.at(t);
        for (i, target, weight) in [
            (LEFT, ik::HAND_L, p::HAND_IK_L),
            (RIGHT, ik::HAND_R, p::HAND_IK_R),
        ] {
            let hold = holds[i].filter(|(_, w)| *w > 0.0);
            sim.set_ik(
                target,
                hold.map(|(world, _)| IkTarget::from(to_local(world))),
            );
            sim.set(weight, hold.map_or(0.0, |(_, w)| w));
        }
        sim.instance.set_slot_progress(full, t);
        sim.run(DT, DT);
        for (i, h) in all.iter().enumerate() {
            let (Some(h), Some(trace)) = (h, traces[i].as_mut()) else {
                continue;
            };
            let (shoulder, hand) = (
                to_world(sim.out.model.translation(hands[i].0)),
                to_world(sim.out.model.translation(hands[i].1)),
            );
            if !hand.is_finite() {
                trace.nan += 1;
                continue;
            }
            let in_box =
                hand.z > dist + 0.02 && hand.z < dist + DEPTH - 0.02 && hand.y < height - 0.02;
            if grip.hands[i].is_some() && in_box {
                // IK-owned (the full-weight hold) or blended in (the ramps: the clip's own arm).
                if h.weight(t) >= 0.999 {
                    trace.inside += 1;
                } else if h.weight(t) > 0.0 {
                    trace.inside_ramp += 1;
                }
            }
            if (h.start..=h.end).contains(&t) {
                let d = hand.distance(h.target);
                trace.held = (trace.held.0.min(d), trace.held.1.max(d));
                if trace.shoulder == 0.0 {
                    trace.shoulder = shoulder.distance(h.target);
                }
            }
        }
    }
    Some((grip, traces))
}

const CASES: [(&str, f32); 2] = [(WAIST, 1.2), (HIGH, 2.0)];

/// The held hands come out finite and outside the box, for both mantles from 0.3 to 0.9 m out.
#[test]
fn mantle_hands_stay_finite_and_out_of_the_box() {
    let Some(b) = bake() else { return };
    for (slot, height) in CASES {
        for dist in [0.3, 0.6, 0.9] {
            let Some((grip, traces)) = mantle(&b, slot, dist, height) else {
                panic!("{slot} at {dist} m: no grip planned")
            };
            // The near lip (the face climbed, at `dist`), never across the top to the far side.
            for h in grip.hands.iter().chain(&grip.skipped).flatten() {
                let (near, far) = (h.target.z - dist, dist + DEPTH - h.target.z);
                assert!(
                    h.target.is_finite() && (0.0..0.08).contains(&near) && far > DEPTH - 0.1,
                    "{slot} at {dist} m: {h:?}"
                );
                assert!(
                    (h.target.y - height).abs() < 0.1,
                    "{slot} at {dist} m: off the top {h:?}"
                );
            }
            for t in traces.iter().flatten() {
                assert_eq!(
                    (t.nan, t.inside),
                    (0, 0),
                    "{slot} at {dist} m: non-finite or inside-the-box frames"
                );
            }
        }
    }
}

#[test]
#[ignore = "measurement: hand-to-target distance through the hold, real graph and clips, by ledge distance"]
fn measure_mantle_hands_against_their_targets() {
    let Some(b) = bake() else { return };
    let arm = arm_length(&b, [B::LeftUpperArm, B::LeftLowerArm, B::LeftHand]);
    println!("arm (upper arm to wrist, rest) {arm:.3} m");
    for (slot, height) in CASES {
        #[allow(clippy::cast_precision_loss)]
        for k in 3..=10 {
            let dist = k as f32 * 0.1;
            let Some((grip, traces)) = mantle(&b, slot, dist, height) else {
                println!("{slot} lip {dist:.1} m: no grip");
                continue;
            };
            for (i, name) in [(LEFT, "left"), (RIGHT, "right")] {
                let (Some(h), Some(t)) = (grip.hands[i].or(grip.skipped[i]), &traces[i]) else {
                    println!("{slot} lip {dist:.1} m {name:5}: no hold");
                    continue;
                };
                println!(
                    "{slot} lip {dist:.1} m {name:5} {}: {:.3}-{:.3} s, hand to target {:.3}-{:.3} m, shoulder to target {:.3} m ({:.2} arm), plan gap {:.3}, end-aligned gap {:.3}, inside {} (ramps {}), nan {}{}",
                    if grip.hands[i].is_some() {
                        "held"
                    } else {
                        "SKIPPED"
                    },
                    h.start,
                    h.end,
                    t.held.0,
                    t.held.1,
                    t.shoulder,
                    t.shoulder / arm,
                    h.gap,
                    t.end_gap,
                    t.inside,
                    t.inside_ramp,
                    t.nan,
                    if h.mirrored { " (mirrored)" } else { "" }
                );
            }
        }
    }
}

/// The hand's baked contact placed by the end-aligned root (the clip's end on the warp target),
/// to the hold's target.
fn end_aligned_gap(clip: &Clip, w: &WarpState, h: &sf_action::HandHold, i: usize) -> f32 {
    let side = if (i == LEFT) ^ h.mirrored {
        Side::Left
    } else {
        Side::Right
    };
    let Some(c) = clip
        .meta
        .contacts
        .iter()
        .find(|c| c.kind == ContactKind::Hand && c.side == side)
    else {
        return f32::NAN;
    };
    let local = if h.mirrored {
        c.local * Vec3::new(-1.0, 1.0, 1.0)
    } else {
        c.local
    };
    let rot = glam::Quat::from_rotation_y;
    let at = RootPath::root_at(clip, c.start);
    let end = RootPath::root_at(clip, clip.duration);
    let rel = rot(-end.yaw) * (at.position + rot(at.yaw) * local - end.position);
    let world = w.plan.target.position + rot(w.plan.target.yaw) * rel;
    world.distance(h.target)
}
