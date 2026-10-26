//! B1 (action.md): `Progress` montages follow the supplied chain time over a slot chain, and the
//! chain warp ([`sf_anim::WarpPlan`]) lands a two-clip chain on its target.

use super::*;
use sf_anim::{RootChain, RootPath, RootPose, WarpPlan, WarpProfile};

/// A slot over an idle, with the `ledge` warp target the traversal clips' windows name.
const LEDGE: &str = r#"(
    nodes: { "base": Clip(clip: "idle"), "full": Slot(slot: "full", input: "base") },
    layers: [(node: "full")],
    warp_targets: ["ledge"],
)"#;

/// A traversal clip: the root goes to `to` and turns `turn` rad over `d` s; a `contact` point
/// event mid-clip and an optional `warp` window.
fn traversal(s: &Skeleton, name: &str, d: f32, to: Vec3, turn: f32, window: Option<(f32, f32)>) -> Clip {
    let root = SourceTrack {
        joint: "root".into(),
        translation: Some(sampled(d, move |t| to * (t / d))),
        rotation: Some(sampled(d, move |t| Quat::from_rotation_y(turn * t / d))),
        ..SourceTrack::default()
    };
    let mut events = vec![Event::point(d * 0.5, "contact")];
    events.extend(window.map(|(a, b)| Event { time: a, end: Some(b), name: "warp".into(), payload: "ledge;skew;y=warp".into() }));
    let src = SourceClip {
        name: name.into(),
        duration: d,
        tracks: vec![root, rot(d, "upperarm_r", |t| Quat::from_rotation_z(0.6 + 0.5 * t))],
        events,
        ..SourceClip::default()
    };
    let mut r = rules(false, RootMotionMode::Progress);
    r.root_motion.vertical = true;
    cook_clip(s, &src, &r, &CookSettings::default()).unwrap()
}

/// Enter (0.5 s, up 0.4 m, forward 0.3 m, turning 0.3 rad) then pull-up (0.8 s, up 0.8 m,
/// forward 0.9 m) with its warp window over [0.1, 0.6].
fn mantle(s: &Skeleton) -> [Clip; 2] {
    [
        traversal(s, "enter", 0.5, Vec3::new(0.0, 0.4, 0.3), 0.3, None),
        traversal(s, "pull", 0.8, Vec3::new(0.0, 0.8, 0.9), 0.0, Some((0.1, 0.6))),
    ]
}

fn progress_req(clip: &Arc<LateClip>, start: SlotStart) -> SlotRequest {
    SlotRequest { start, blend_in: 0.0, blend_out: 0.1, root_motion: RootMotionMode::Progress, ..SlotRequest::new(MontageClip::Late(clip.clone())) }
}

#[test]
fn progress_montages_follow_the_supplied_chain_time() {
    for hz in [30.0_f32, 60.0, 144.0] {
        let rig = Rig::new(LEDGE);
        let b = rig.binding();
        let [enter, pull] = mantle(&rig.skeleton);
        let chain = RootChain::new(vec![Arc::new(enter), Arc::new(pull)]);
        let late: Vec<_> = chain.parts().iter().map(|c| Arc::new(LateClip::new(&rig.graph, &rig.skeleton, c.clone()).unwrap())).collect();
        let mut inst = GraphInstance::new(&b);
        let mut out = GraphOutput::new(&rig.graph);
        let full = rig.graph.slot("full").unwrap();
        let ids = [inst.play_slot(&b, full, progress_req(&late[0], SlotStart::Interrupt)).unwrap(), inst.play_slot(&b, full, progress_req(&late[1], SlotStart::Queue)).unwrap()];
        let dt = 1.0 / hz;
        // The action clock runs at 0.75 of frame time (a hit-stop scale): the playhead must
        // follow it, not dt.
        let rate = 0.75;
        let (mut contacts, mut notices, mut gone) = ([0; 2], Vec::new(), None);
        for i in 1..=((chain.duration() / (dt * rate)) as usize + 3) {
            let p = i as f32 * dt * rate;
            inst.set_slot_progress(full, p);
            inst.update(&b, dt, 0, &UpdateInputs::default(), &mut out);
            for e in &out.events {
                if let Some(k) = ids.iter().position(|&id| e.montage == Some(id))
                    && chain.parts()[k].events[e.event as usize].name == "contact"
                {
                    contacts[k] += 1;
                }
            }
            notices.extend(out.slot_notices.iter().map(|n| (n.montage, n.kind)));
            if p < chain.duration() {
                let m = inst.montage(full).unwrap_or_else(|| panic!("{hz} Hz: montage gone at {p}"));
                assert!((m.chain_time - p).abs() < 1e-5, "{hz} Hz: chain time {} at progress {p}", m.chain_time);
                let (part, local) = chain.locate(p).unwrap();
                assert_eq!(m.id, ids[part], "{hz} Hz: montage at {p}");
                assert!((m.time - local).abs() < 1e-5, "{hz} Hz: clip time {} vs {local} at {p}", m.time);
            } else if inst.montage(full).is_none() {
                gone.get_or_insert(p);
            }
        }
        assert_eq!(contacts, [1, 1], "{hz} Hz: each clip's contact fires once");
        assert!(gone.is_some(), "{hz} Hz: the chain ends past its last clip");
        let ended = NoticeKind::Ended { interrupted: false };
        assert_eq!(notices, [(ids[0], NoticeKind::Started), (ids[0], ended), (ids[1], NoticeKind::Started), (ids[1], ended)], "{hz} Hz");
    }

    // Without a progress the playhead holds; progress may go back inside a montage.
    let rig = Rig::new(LEDGE);
    let b = rig.binding();
    let [enter, _] = mantle(&rig.skeleton);
    let late = Arc::new(LateClip::new(&rig.graph, &rig.skeleton, Arc::new(enter)).unwrap());
    let mut inst = GraphInstance::new(&b);
    let mut out = GraphOutput::new(&rig.graph);
    let full = rig.graph.slot("full").unwrap();
    inst.play_slot(&b, full, progress_req(&late, SlotStart::Interrupt)).unwrap();
    for (p, expect) in [(Some(0.3), 0.3), (None, 0.3), (None, 0.3), (Some(0.1), 0.1), (Some(0.25), 0.25)] {
        if let Some(p) = p {
            inst.set_slot_progress(full, p);
        }
        inst.update(&b, 0.1, 0, &UpdateInputs::default(), &mut out);
        assert!((inst.montage(full).unwrap().time - expect).abs() < 1e-6, "{p:?}");
    }
}

#[test]
fn chain_warps_land_on_the_target_and_refuse_above_the_ratio() {
    let s = biped();
    let [enter, pull] = mantle(&s);
    let chain = RootChain::new(vec![&enter, &pull]);
    let total = chain.duration();
    // The correction lives in the pull-up's warp window.
    assert_eq!(chain.carrier(), Some(1));
    let (w0, w1) = chain.warp_window().unwrap();
    assert!((w0 - 0.6).abs() < 1e-5 && (w1 - 1.1).abs() < 1e-5, "window {w0}..{w1}");

    let start = RootPose::new(Vec3::new(3.0, 1.0, -2.0), 0.7);
    let baked_end = start.then(chain.root_at(total));
    let target = RootPose::new(baked_end.position + Vec3::new(0.12, 0.25, -0.08), baked_end.yaw + 0.15);
    for hz in [30.0_f32, 60.0, 144.0] {
        for profile in [WarpProfile::Linear, WarpProfile::Late] {
            let plan = WarpPlan::try_new(&chain, start, target, profile, 0.35).unwrap();
            let c = plan.correction.length();
            let share = |p: f32| (plan.root_at(&chain, p).position - start.then(chain.root_at(p)).position).length() / c;
            // The action clock on ticks of 1/hz, ending exactly at the chain's end.
            let ticks = (total * hz).ceil() as usize;
            let mut end = start;
            for i in 0..=ticks {
                let p = (i as f32 / hz).min(total);
                end = plan.root_at(&chain, p);
                if p < w0 {
                    assert!(share(p) < 1e-6, "{profile:?}: no correction before the window ({p})");
                }
            }
            let err = (end.position - target.position).length();
            let yaw_err = sf_anim::angle_between(Quat::from_rotation_y(end.yaw), Quat::from_rotation_y(target.yaw)).to_degrees();
            assert!(err <= 0.02 && yaw_err <= 2.0, "{hz} Hz {profile:?}: {err} m, {yaw_err} deg");
            let mid = share(f32::midpoint(w0, w1));
            match profile {
                WarpProfile::Linear => assert!((mid - 0.5).abs() < 0.01, "linear mid share {mid}"),
                WarpProfile::Late => assert!(mid <= 0.15 && share(w1) > 0.99, "late mid share {mid}"),
            }
        }
    }

    // Zero correction reproduces the baked root exactly.
    let plan = WarpPlan::new(&chain, start, baked_end, WarpProfile::Late);
    assert_eq!((plan.correction, plan.yaw_correction), (Vec3::ZERO, 0.0));
    for i in 0..=26 {
        let p = i as f32 * 0.05;
        assert_eq!(plan.root_at(&chain, p), start.then(chain.root_at(p)), "at {p}");
    }

    // Refusal: correction / root distance against the rule's ratio.
    let dist = chain.root_at(total).position.length();
    let off = |share: f32| RootPose::new(baked_end.position + Vec3::X * dist * share, baked_end.yaw);
    assert!(WarpPlan::try_new(&chain, start, off(0.30), WarpProfile::Late, 0.35).is_ok());
    let refused = WarpPlan::try_new(&chain, start, off(0.40), WarpProfile::Late, 0.35).unwrap_err();
    assert!((refused.ratio - 0.40).abs() < 1e-3, "{refused}");
}
