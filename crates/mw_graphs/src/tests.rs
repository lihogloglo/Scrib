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
        for bits in 0..32u32 {
            for turn in [-1.0, 0.0, 1.0] {
                let b = |i: u32| if bits & (1 << i) != 0 { 1.0 } else { 0.0 };
                let v: HashMap<&str, f32> =
                    [(p::MOVING, b(0)), (p::RUNNING, b(1)), (p::SNEAKING, b(2)), (p::SWIMMING, b(3)), (p::ON_GROUND, b(4)), (p::TURN, turn)].into();
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
