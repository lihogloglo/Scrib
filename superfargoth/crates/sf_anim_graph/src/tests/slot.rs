//! Montage slots (animation.md "Slots and montages"): events once, root
//! motion as a `Drive` player, continuous interrupts, queue, groups,
//! priority, masks, late-bound clips.

use super::*;
use crate::slot::{LATE_CLIP, LOOP_FOREVER};

/// A full-body slot over an upper-body slot over a walk, both in the
/// default group.
const SLOTTED: &str = r#"(
    masks: { "upper": (rules: [(joint: Tag("spine"), weight: 0.5), (joint: Joint("chest"), weight: 1.0)]) },
    nodes: {
        "base": Clip(clip: "walk"),
        "upper_slot": Slot(slot: "upper", input: "base", mask: "upper"),
        "full": Slot(slot: "full", input: "upper_slot"),
    },
    layers: [(node: "full")],
    slot_clips: ["attack", "wave"],
)"#;

fn req(rig: &Rig, name: &str) -> SlotRequest {
    SlotRequest::new(MontageClip::Bound(rig.graph.slot_clip(name).unwrap()))
}

fn bound(rig: &Rig, reference: &str) -> MontageClip {
    MontageClip::Bound(u16::try_from(rig.graph.clips.iter().position(|c| c.reference == reference).unwrap()).unwrap())
}

fn named<'a>(rig: &'a Rig, e: &GraphEvent) -> &'a str {
    rig.graph.events.get(usize::from(e.id)).map_or("?", String::as_str)
}

#[test]
fn slot_events_fire_once_and_notices_report_the_lifecycle() {
    for dt in [1.0 / 60.0, 1.0 / 240.0, 0.1] {
        let rig = Rig::new(SLOTTED);
        let b = rig.binding();
        let mut inst = GraphInstance::new(&b);
        let mut out = GraphOutput::new(&rig.graph);
        let full = rig.graph.slot("full").unwrap();
        for _ in 0..5 {
            step(&rig, &mut inst, &mut out, dt);
        }
        let id = inst.play_slot(&b, full, req(&rig, "attack")).unwrap();
        let (mut hits, mut enters, mut exits, mut steps) = (0, 0, 0, 0);
        let mut notices = Vec::new();
        for _ in 0..(1.5 / dt) as usize {
            step(&rig, &mut inst, &mut out, dt);
            for e in &out.events {
                match (named(&rig, e), e.edge, e.montage) {
                    ("hit", EventEdge::Point, Some(m)) if m == id => hits += 1,
                    ("combo", EventEdge::Enter, Some(m)) if m == id => enters += 1,
                    ("combo", EventEdge::Exit, Some(m)) if m == id => exits += 1,
                    ("step", _, None) if inst.montage(full).is_some() => steps += 1,
                    _ => {}
                }
            }
            notices.extend(out.slot_notices.iter().copied());
        }
        assert_eq!((hits, enters, exits), (1, 1, 1), "dt {dt}");
        // The full-body montage hides the walk, which holds: no footsteps.
        assert_eq!(steps, 0, "dt {dt}");
        let kinds: Vec<_> = notices.iter().map(|n| (n.montage, n.kind)).collect();
        assert_eq!(kinds, [(id, NoticeKind::Started), (id, NoticeKind::Ended { interrupted: false })], "dt {dt}");
        assert!(inst.montage(full).is_none());
    }

    // Loops: the loop range repeats `loops` times, each pass firing its
    // events once, then the clip plays out and ends.
    let dt = 1.0 / 60.0;
    let rig = Rig::new(SLOTTED);
    let b = rig.binding();
    let mut inst = GraphInstance::new(&b);
    let mut out = GraphOutput::new(&rig.graph);
    let full = rig.graph.slot("full").unwrap();
    let id = inst.play_slot(&b, full, SlotRequest { loops: 2, ..SlotRequest::new(bound(&rig, "walk")) }).unwrap();
    let (mut steps, mut ended_at) = (0, None);
    for k in 0..240 {
        step(&rig, &mut inst, &mut out, dt);
        steps += out.events.iter().filter(|e| e.montage == Some(id) && named(&rig, e) == "step").count();
        if out.slot_notices.iter().any(|n| n.montage == id && n.kind == NoticeKind::Ended { interrupted: false }) {
            ended_at.get_or_insert(k as f32 * dt);
        }
    }
    assert_eq!(steps, 6, "two events per pass, three passes");
    let t = ended_at.expect("ended");
    assert!((3.0..3.0 + 2.5 * dt).contains(&t), "ended at {t}");

    // Forever: still looping after 5 s.
    let mut inst = GraphInstance::new(&b);
    inst.play_slot(&b, full, SlotRequest { loops: LOOP_FOREVER, ..SlotRequest::new(bound(&rig, "walk")) }).unwrap();
    for _ in 0..300 {
        step(&rig, &mut inst, &mut out, dt);
    }
    assert_eq!(inst.montage(full).map(|m| m.loops_left), Some(LOOP_FOREVER));
}

#[test]
fn slot_root_motion_matches_a_drive_player() {
    let dt = 1.0 / 60.0;
    let slotted = Rig::new(SLOTTED);
    let player = Rig::new(r#"(nodes: {"atk": Clip(clip: "attack")}, layers: [(node: "atk")])"#);
    let (bs, bp) = (slotted.binding(), player.binding());
    let full = slotted.graph.slot("full").unwrap();
    let late = Arc::new(LateClip::new(&slotted.graph, &slotted.skeleton, Arc::new(attack(&slotted.skeleton))).unwrap());
    for clip in [bound(&slotted, "attack"), MontageClip::Late(late)] {
        let late_clip = matches!(clip, MontageClip::Late(_));
        let mut si = GraphInstance::new(&bs);
        let mut pi = GraphInstance::new(&bp);
        let (mut so, mut po) = (GraphOutput::new(&slotted.graph), GraphOutput::new(&player.graph));
        si.play_slot(&bs, full, SlotRequest::new(clip)).unwrap();
        let (mut total, mut hits) = (RootDelta::ZERO, 0);
        for k in 0..60 {
            step(&slotted, &mut si, &mut so, dt);
            step(&player, &mut pi, &mut po, dt);
            assert!(so.root.translation.distance(po.root.translation) < 1e-6, "update {k}: {:?} vs {:?}", so.root, po.root);
            assert!((so.root.yaw - po.root.yaw).abs() < 1e-6);
            if k < 48 {
                assert_eq!(so.drive_weight, 1.0);
            }
            total = total.then(so.root);
            for e in so.events.iter().filter(|e| e.montage.is_some()) {
                assert_eq!(e.clip == LATE_CLIP, late_clip);
                hits += usize::from(named(&slotted, e) == "hit" && e.edge == EventEdge::Point);
            }
        }
        // The whole authored path, once.
        assert!((total.translation.z - 0.48).abs() < 2e-3, "{total:?}");
        assert_eq!(hits, 1);
    }
}

/// Foot + hand track of a walk with a montage at 0.5 s interrupted by
/// another at 0.62 s, both blending in over `blend` seconds.
fn slot_switch_run(dt: f32, blend: f32, total: f32) -> Vec<Vec3> {
    let rig = Rig::new(SLOTTED);
    let b = rig.binding();
    let mut inst = GraphInstance::new(&b);
    let mut out = GraphOutput::new(&rig.graph);
    let full = rig.graph.slot("full").unwrap();
    let (foot, hand) = (rig.joint("foot_l"), rig.joint("hand_l"));
    let n = (total / dt).round() as usize;
    let (k1, k2) = ((0.5 / dt).round() as usize, (0.62 / dt).round() as usize);
    let mut track = Vec::with_capacity(n);
    for k in 0..n {
        if k == k1 {
            inst.play_slot(&b, full, SlotRequest { blend_in: blend, root_motion: RootMotionMode::Ignore, ..req(&rig, "attack") }).unwrap();
        }
        if k == k2 {
            inst.play_slot(&b, full, SlotRequest { blend_in: blend, from: 0.3, ..req(&rig, "wave") }).unwrap();
        }
        step(&rig, &mut inst, &mut out, dt);
        track.push(out.model.translation(foot) + out.model.translation(hand));
    }
    track
}

#[test]
fn slot_interrupt_mid_clip_is_continuous() {
    let (coarse, fine) = (1.0 / 60.0, 1.0 / 240.0);
    let at = |dt: f32, t: f32| (t / dt).round() as usize;
    let a = slot_switch_run(coarse, 0.3, 1.2);
    let b = slot_switch_run(fine, 0.3, 1.2);
    let cut = slot_switch_run(fine, 0.0, 1.2);
    for t in [0.5, 0.62] {
        let (pa, va) = jumps(&a, coarse, at(coarse, t), 2);
        let (pb, vb) = jumps(&b, fine, at(fine, t), 2);
        let (pc, _) = jumps(&cut, fine, at(fine, t), 2);
        assert!(pa / coarse < 8.0 && pb / fine < 8.0, "pose step {pa} @60 Hz, {pb} @240 Hz at {t}");
        assert!(pc > 10.0 * pb, "cut {pc} vs inertialized {pb} at {t}");
        assert!(vb < va * 0.45, "velocity jump {va} @60 Hz, {vb} @240 Hz at {t}");
    }
}

#[test]
fn slot_queue_groups_priority_masks_and_stop() {
    let dt = 1.0 / 60.0;
    let rig = Rig::new(SLOTTED);
    let b = rig.binding();
    let mut inst = GraphInstance::new(&b);
    let mut reference = GraphInstance::new(&b);
    let (mut out, mut ref_out) = (GraphOutput::new(&rig.graph), GraphOutput::new(&rig.graph));
    let (full, upper) = (rig.graph.slot("full").unwrap(), rig.graph.slot("upper").unwrap());
    let (thigh, arm) = (rig.joint("thigh_l"), rig.joint("upperarm_l"));

    // Masked slot: the legs keep walking (the input plays on), the arm waves.
    let wave = inst.play_slot(&b, upper, SlotRequest { loops: LOOP_FOREVER, ..req(&rig, "wave") }).unwrap();
    for _ in 0..30 {
        step(&rig, &mut inst, &mut out, dt);
        step(&rig, &mut reference, &mut ref_out, dt);
    }
    assert!(!inst.inertializing());
    assert_eq!(out.pose.get(thigh), ref_out.pose.get(thigh));
    assert!(out.pose.get(arm).rotation.angle_between(ref_out.pose.get(arm).rotation) > 0.05);

    // Same group: a full-body montage ends the upper-body one.
    let atk = inst.play_slot(&b, full, SlotRequest { priority: 5, ..req(&rig, "attack") }).unwrap();
    step(&rig, &mut inst, &mut out, dt);
    assert!(inst.montage(upper).is_none());
    assert!(out.slot_notices.contains(&SlotNotice { slot: upper, montage: wave, kind: NoticeKind::Ended { interrupted: true } }));
    // Priority: a weaker interrupt is refused; a queued request waits.
    assert_eq!(inst.play_slot(&b, full, req(&rig, "wave")).unwrap_err(), SlotError::Busy);
    let next = inst.play_slot(&b, full, SlotRequest { start: SlotStart::Queue, ..req(&rig, "wave") }).unwrap();
    assert_eq!(inst.montage(full).map(|m| (m.id, m.queued)), Some((atk, 1)));
    let mut order = Vec::new();
    for _ in 0..120 {
        step(&rig, &mut inst, &mut out, dt);
        order.extend(out.slot_notices.iter().filter(|n| n.slot == full).map(|n| (n.montage, n.kind)));
        if order.len() < 3 {
            assert!(inst.montage(full).is_some(), "no gap between queued montages");
        }
    }
    assert_eq!(
        order,
        [(atk, NoticeKind::Ended { interrupted: false }), (next, NoticeKind::Started), (next, NoticeKind::Ended { interrupted: false })]
    );

    // Ending early, and a stop inside a window closes it.
    let early = inst.play_slot(&b, full, SlotRequest { end_early: 0.3, ..req(&rig, "attack") }).unwrap();
    let mut last = 0.0;
    while let Some(m) = inst.montage(full) {
        last = m.time;
        step(&rig, &mut inst, &mut out, dt);
    }
    assert!((last - 0.5).abs() < 1e-4, "ended at {last}");
    assert!(out.slot_notices.contains(&SlotNotice { slot: full, montage: early, kind: NoticeKind::Ended { interrupted: false } }));
    let stopped = inst.play_slot(&b, full, req(&rig, "attack")).unwrap();
    for _ in 0..33 {
        step(&rig, &mut inst, &mut out, dt);
    }
    inst.stop_slot(&b, full, 0.2);
    step(&rig, &mut inst, &mut out, dt);
    assert!(out.events.iter().any(|e| e.montage == Some(stopped) && named(&rig, e) == "combo" && e.edge == EventEdge::Exit));
    assert!(out.slot_notices.contains(&SlotNotice { slot: full, montage: stopped, kind: NoticeKind::Ended { interrupted: true } }));
    assert!(inst.inertializing());
}

#[test]
fn slot_compile_rules_and_round_trip() {
    let skeleton = biped();
    let mut src = source(&skeleton);
    let two_groups = r#"(nodes: {"a": Clip(clip: "walk"), "s1": Slot(slot: "x", input: "a", group: "g1"),
        "s2": Slot(slot: "x", input: "s1", group: "g2")}, layers: [(node: "s2")])"#;
    let e = compile_str("t", two_groups, &skeleton, &mut src).unwrap_err();
    assert!(e.message.contains("two slot groups"), "{e}");
    let additive = r#"(nodes: {"a": Clip(clip: "walk"), "s": Slot(slot: "x", input: "a")}, layers: [(node: "s")], slot_clips: ["lean"])"#;
    let e = compile_str("t", additive, &skeleton, &mut src).unwrap_err();
    assert!(e.message.contains("additive"), "{e}");

    let rig = Rig::new(SLOTTED);
    let back = CompiledGraph::from_bytes(&rig.graph.to_bytes()).unwrap();
    assert_eq!(back.slots, rig.graph.slots);
    assert_eq!(back.slot_clips, rig.graph.slot_clips);
    assert_eq!(rig.graph.slot_groups, ["default"]);
    assert!(rig.graph.slot("nope").is_none());
    let b = rig.binding();
    let mut inst = GraphInstance::new(&b);
    assert_eq!(inst.play_slot(&b, SlotId(9), req(&rig, "wave")).unwrap_err(), SlotError::NoSlot);
    assert_eq!(inst.play_slot(&b, SlotId(0), SlotRequest::new(MontageClip::Bound(99))).unwrap_err(), SlotError::NoClip);
}
