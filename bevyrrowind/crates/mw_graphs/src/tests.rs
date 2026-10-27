use std::collections::HashMap;

use sf_anim_graph::asset::{ConditionSpec, parse_graph};
use sf_anim_graph::{SlotStart, slot::LOOP_FOREVER};

use crate::build::{self, Template, params as p};
use crate::script::{GroupRequest, GroupStart, SCRIPT_PRIORITY};

/// The shipped RON is the builder's output. `MW_GRAPHS_BLESS=1` rewrites it.
#[test]
fn shipped_graphs_match_the_builder() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("data");
    for t in Template::ALL {
        let text = build::to_ron(&build::asset(t));
        if std::env::var_os("MW_GRAPHS_BLESS").is_some() {
            std::fs::write(dir.join(t.path()), &text).unwrap();
            continue;
        }
        let shipped = crate::builtin(t).replace("\r\n", "\n");
        let at = shipped.bytes().zip(text.bytes()).position(|(a, b)| a != b).unwrap_or(shipped.len().min(text.len()));
        assert!(
            shipped == text,
            "{}: stale at byte {at} ({} vs {} bytes): {:?} vs {:?}; rerun with MW_GRAPHS_BLESS=1",
            t.path(),
            shipped.len(),
            text.len(),
            &shipped[at.saturating_sub(40)..(at + 40).min(shipped.len())],
            &text[at.saturating_sub(40)..(at + 40).min(text.len())]
        );
        assert_eq!(parse_graph(&shipped).unwrap(), build::asset(t), "{}: round trip", t.path());
    }
}

fn holds(c: &ConditionSpec, v: &HashMap<&str, f32>) -> bool {
    match c {
        ConditionSpec::IsTrue(n) => v[n.as_str()] > 0.5,
        ConditionSpec::IsFalse(n) => v[n.as_str()] <= 0.5,
        ConditionSpec::Greater(n, x) => v[n.as_str()] > *x,
        ConditionSpec::Less(n, x) => v[n.as_str()] < *x,
        other => panic!("unexpected movement condition {other:?}"),
    }
}

/// Exactly one movement state matches every parameter combination, so the
/// machine always knows where to go and never flip-flops.
#[test]
fn movement_conditions_are_exclusive_and_complete() {
    for t in Template::ALL {
        let moves = build::movement_conditions(t);
        for bits in 0..128u32 {
            for turn in [-1.0, 0.0, 1.0] {
                let b = |i: u32| if bits & (1 << i) != 0 { 1.0 } else { 0.0 };
                let v: HashMap<&str, f32> =
                    [(p::MOVING, b(0)), (p::RUNNING, b(1)), (p::SNEAKING, b(2)), (p::SWIMMING, b(3)), (p::ON_GROUND, b(4)), (p::JUMPING, b(5)), (p::STRAFING, b(6)), (p::TURN, turn)].into();
                let hits: Vec<&str> = moves.iter().filter(|(_, when)| when.iter().all(|c| holds(c, &v))).map(|(s, _)| *s).collect();
                assert_eq!(hits.len(), 1, "{t:?} {v:?}: {hits:?}");
            }
        }
    }
}

#[test]
fn play_and_loop_group_map_onto_slot_requests() {
    let clip = Some((0.5, 2.0));
    let r = GroupRequest::play("Idle3", 0).slot_request(sf_anim_graph::MontageClip::Bound(0), clip);
    assert_eq!((r.start, r.loops, r.from, r.priority), (SlotStart::Queue, 0, 0.0, SCRIPT_PRIORITY));
    let r = GroupRequest::play("Idle3", 1).slot_request(sf_anim_graph::MontageClip::Bound(0), clip);
    assert_eq!((r.start, r.from), (SlotStart::Interrupt, 0.0));
    let r = GroupRequest::looped("Idle3", 3, 2).slot_request(sf_anim_graph::MontageClip::Bound(0), clip);
    assert_eq!((r.start, r.loops), (SlotStart::Interrupt, 2));
    assert!((r.from - 0.5).abs() < 1e-6);
    assert_eq!(GroupStart::from_flag(7), GroupStart::Queued);
    assert_ne!(GroupRequest::looped("x", 0, 0).slot_request(sf_anim_graph::MontageClip::Bound(0), clip).loops, LOOP_FOREVER);
}

/// `mw_actors` recipes name the shipped paths.
#[test]
fn actor_config_names_the_shipped_graphs() {
    let c = mw_actors::ActorConfig::default();
    assert_eq!(c.humanoid_graph.as_str(), Template::Humanoid.path());
    assert_eq!(c.first_person_graph.as_str(), Template::FirstPerson.path());
    assert_eq!(c.creature_graph.as_str(), Template::Creature.path());
}

/// The builtin layer serves every template at its `ActorConfig` path, and a
/// mod layer mounted above it wins (everything is a mod).
#[test]
fn builtin_layer_resolves_and_mods_override() {
    use sf_vfs::{MemSource, Vfs, VfsPath};
    let path = VfsPath::new(Template::Humanoid.path()).unwrap();
    let vfs = Vfs::builder().mount(crate::builtin_source()).build();
    for t in Template::ALL {
        assert_eq!(vfs.read(&VfsPath::new(t.path()).unwrap()).unwrap(), crate::builtin(t).as_bytes());
    }
    let modded = Vfs::builder().mount(crate::builtin_source()).mount(MemSource::new("mod").with(Template::Humanoid.path(), "patched")).build();
    assert_eq!(modded.read(&path).unwrap(), b"patched");
}

/// Gameplay glue: the wielded weapon type and readiness pick the `stance` parameter.
#[test]
fn stance_of_weapon() {
    use sf_character::Readiness::{Drawn, Sheathed};
    use tes3::esp::WeaponType as W;

    use crate::Stance as S;
    assert_eq!(S::of_weapon(Sheathed, Some(W::LongBladeTwoClose)), S::None);
    assert_eq!(S::of_weapon(Drawn, None), S::HandToHand);
    for (w, s) in [
        (W::ShortBladeOneHand, S::OneHand),
        (W::LongBladeOneHand, S::OneHand),
        (W::BluntOneHand, S::OneHand),
        (W::AxeOneHand, S::OneHand),
        (W::MarksmanBow, S::OneHand),
        (W::MarksmanThrown, S::OneHand),
        (W::LongBladeTwoClose, S::TwoClose),
        (W::BluntTwoClose, S::TwoClose),
        (W::AxeTwoHand, S::TwoClose),
        (W::BluntTwoWide, S::TwoWide),
        (W::SpearTwoWide, S::TwoWide),
        (W::MarksmanCrossbow, S::Crossbow),
        (W::Arrow, S::HandToHand),
    ] {
        assert_eq!(S::of_weapon(Drawn, Some(w)), s, "{w:?}");
    }
}

/// Humanoids loop `Torch` on the left arm, weighted by `torch`, between the
/// `upper` slot (weapon actions) and the `full` slot (scripted groups).
#[test]
fn torch_arm_layers_over_weapon_actions_and_under_scripted_groups() {
    use sf_anim_graph::asset::{LayerMode, NodeSpec, ValueSpec};
    for t in [Template::Humanoid, Template::FirstPerson] {
        let a = build::asset(t);
        assert!(a.params.iter().any(|q| q.name == p::TORCH), "{t:?}");
        assert!(a.masks.contains_key("left_arm"), "{t:?}");
        let NodeSpec::Slot { input, .. } = &a.nodes["full_slot"] else { panic!("{t:?}: full slot") };
        assert_eq!(input, "torch_layer", "{t:?}");
        let NodeSpec::LayeredPerBone { base, layers } = &a.nodes["torch_layer"] else { panic!("{t:?}: torch layer") };
        assert_eq!(base, "upper_slot", "{t:?}");
        let [layer] = layers.as_slice() else { panic!("{t:?}: one layer") };
        assert!(layer.mode == LayerMode::Override && layer.mask.as_deref() == Some("left_arm"));
        assert_eq!(layer.weight, ValueSpec::Param(p::TORCH.into()));
        let NodeSpec::Clip { clip, looping, .. } = &a.nodes[&layer.node] else { panic!("{t:?}: torch clip") };
        assert!(clip == "Torch" && *looping == Some(true) && a.clips["Torch"].optional, "{t:?}");
    }
    let creature = build::asset(Template::Creature);
    assert!(!creature.nodes.contains_key("torch_layer") && !creature.params.iter().any(|q| q.name == p::TORCH));
}

/// Lock-on strafe (combat.md CB-2): humanoid walk and run hand over on `strafing` to the
/// manifest's 8-way `loco.walk.*` / `loco.jog.*` loops on the unit circle (the stance idle
/// over a drawn weapon's upper body); free movement keeps the Morrowind sets. The loops play
/// in the strafe states, never as montage slot clips. First person and creatures have none.
#[test]
fn humanoid_walk_and_run_switch_to_the_eight_way_strafe_loops() {
    use sf_anim_graph::asset::{NodeSpec, StateSpec};
    let a = build::asset(Template::Humanoid);
    assert!(a.params.iter().any(|q| q.name == p::STRAFING));
    let NodeSpec::StateMachine { states, .. } = &a.nodes[build::LOCOMOTION] else { panic!("locomotion") };
    let node_of = |state: &str| &states.iter().find(|s: &&StateSpec| s.name == state).unwrap().node;
    for (state, set, morrowind) in [("walk", "walk", "WalkForward"), ("run", "jog", "RunForward")] {
        let strafe_state = format!("strafe_{state}");
        assert_eq!(node_of(state), state, "{state}: the free set");
        assert_eq!(node_of(&strafe_state), &format!("{state}.locked"), "{state}: the strafe set");
        let moves = build::movement_conditions(Template::Humanoid);
        let when = |s: &str| &moves.iter().find(|(n, _)| *n == s).unwrap().1;
        assert!(when(state).contains(&ConditionSpec::IsFalse(p::STRAFING.into())), "{state}");
        assert!(when(&strafe_state).contains(&ConditionSpec::IsTrue(p::STRAFING.into())), "{strafe_state}");
        // Free: the Morrowind directional set, unarmed.
        let NodeSpec::BlendSpace2D { samples, .. } = &a.nodes[&format!("{state}.none")] else { panic!("{state}: free set") };
        assert_eq!(samples[0].clip, morrowind);
        // Locked, unarmed: eight loops a unit from the centre, 45 degrees apart.
        let NodeSpec::BlendSpace2D { samples, x, y, .. } = &a.nodes[&format!("{state}.locked.none")] else { panic!("{state}: strafe set") };
        assert_eq!((x, y), (&sf_anim_graph::asset::ValueSpec::Param("move.x".into()), &sf_anim_graph::asset::ValueSpec::Param("move.y".into())));
        assert_eq!(samples.len(), 8, "{state}");
        let mut angles: Vec<f32> = samples.iter().map(|s| s.at.0.atan2(s.at.1).to_degrees().rem_euclid(360.0)).collect();
        for s in samples {
            assert!(s.clip.starts_with(&format!("loco.{set}.")), "{state}: {}", s.clip);
            assert!((glam::Vec2::new(s.at.0, s.at.1).length() - 1.0).abs() < 1e-5, "{state}: {}", s.clip);
            assert!(a.clips[&s.clip].optional, "{}", s.clip);
            assert!(!a.slot_clips.contains(&s.clip), "{} is no montage clip", s.clip);
        }
        angles.sort_by(f32::total_cmp);
        for (ang, k) in angles.iter().zip(0u8..) {
            assert!((ang - 45.0 * f32::from(k)).abs() < 1e-3, "{state}: {angles:?}");
        }
        let left = samples.iter().find(|s| s.at == (-1.0, 0.0)).unwrap();
        assert_eq!(left.clip, format!("loco.{set}.left"));
        // Locked, one-handed: the strafe set on the lower body under the stance idle.
        let NodeSpec::LayeredPerBone { base, layers } = &a.nodes[&format!("{state}.locked.1h")] else { panic!("{state}: 1h strafe") };
        assert_eq!(base, &format!("{state}.locked.1h.lower"));
        let NodeSpec::Clip { clip, .. } = &a.nodes[&layers[0].node] else { panic!() };
        assert_eq!((clip.as_str(), layers[0].mask.as_deref()), ("Idle1h", Some("upper")));
    }
    for t in [Template::FirstPerson, Template::Creature] {
        let a = build::asset(t);
        assert!(!a.params.iter().any(|q| q.name == p::STRAFING), "{t:?}");
        assert!(!a.nodes.keys().any(|n| n.contains(".locked")), "{t:?}");
    }
}
