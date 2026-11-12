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

/// Lock-on strafe (combat.md CB-2): locked on, walk and run keep Morrowind's 4-way
/// directional sets (owner, live 2026-10-27: the UAL strafe loops leaned back and felt slow).
/// The shipped manifest has no `loco.*` strafe slots, so no template gets the strafe states;
/// the humanoid's `strafing` parameter only fades the gait set to those directional sets.
#[test]
fn locked_strafing_plays_the_morrowind_directional_sets() {
    use sf_anim_graph::asset::{NodeSpec, ValueSpec};
    let a = build::asset(Template::Humanoid);
    for (state, base) in [("walk", "Walk"), ("run", "Run")] {
        let NodeSpec::Blend { b, alpha, .. } = &a.nodes[&format!("{state}.none")] else { panic!("{state}: gait / locked blend") };
        assert_eq!(*alpha, ValueSpec::Param(p::STRAFING.into()), "{state}: locked on = 1");
        let NodeSpec::BlendSpace2D { samples, .. } = &a.nodes[b] else { panic!("{state}: Morrowind set") };
        let mut clips: Vec<String> = samples.iter().map(|s| s.clip.clone()).collect();
        clips.sort_unstable();
        let mut want = ["Back", "Forward", "Left", "Right"].map(|d| format!("{base}{d}"));
        want.sort_unstable();
        assert_eq!(clips, want, "{state}");
    }
    for t in [Template::Humanoid, Template::FirstPerson, Template::Creature] {
        let a = build::asset(t);
        assert_eq!(a.params.iter().any(|q| q.name == p::STRAFING), t == Template::Humanoid, "{t:?}");
        assert!(!a.nodes.keys().any(|n| n.contains(".locked")), "{t:?}");
        assert!(!a.clips.keys().any(|c| c.starts_with("loco.") && !build::gaits::is_gait_clip(c)), "{t:?}: no strafe slots");
    }
}

/// The gait set (A3): walk and run of the base-set stances (sheathed, spell) play one 1D
/// blend over `speed` of the manifest's walk / jog / sprint loops at the gait speeds, in one
/// sync group across both states, `SpeedMatch` clamped to 0.8-1.25 on its lanes; weapon
/// stances, sneak and swim keep the Morrowind sets; first person and creatures have none.
#[test]
fn free_movement_plays_the_synced_gait_set_on_speed() {
    use sf_anim_graph::asset::{NodeSpec, ValueSpec};
    let a = build::asset(Template::Humanoid);
    let want: Vec<(String, f32)> = vec![("loco.walk.cycle".into(), 1.6), ("loco.jog.cycle".into(), 4.5), ("loco.sprint.cycle".into(), 7.0)];
    let mut sets = 0;
    for (name, node) in &a.nodes {
        let NodeSpec::BlendSpace1D { x, samples, sync, rate_clamp, .. } = node else { continue };
        sets += 1;
        assert!(["walk.none.gait", "run.none.gait", "walk.spell.lower.gait", "run.spell.lower.gait"].contains(&name.as_str()), "{name}");
        assert_eq!(*x, ValueSpec::Param(p::SPEED.into()), "{name}");
        assert_eq!(samples.iter().map(|s| (s.clip.clone(), s.at)).collect::<Vec<_>>(), want, "{name}");
        assert_eq!(sync.as_ref().map(|s| s.group.as_str()), Some(build::gaits::SYNC_GROUP), "{name}");
        assert_eq!(*rate_clamp, Some((0.8, 1.25)), "{name}");
    }
    assert_eq!(sets, 4);
    // The slots: optional, the loop cut at its defaults, dropped without the library.
    for (clip, _) in &want {
        let r = &a.clips[clip];
        assert!(r.optional && r.fallback == Some(sf_anim_graph::asset::ClipFallback::Drop), "{clip}");
        let slot = &crate::sources::SourcesManifest::builtin().slots[clip];
        let rules = slot.rules(&slot.candidates[0]);
        assert_eq!(rules.loop_cut, Some(sf_anim::LoopCutRules::default()), "{clip}");
        assert_eq!(rules.root_motion.mode, sf_anim::RootMotionMode::Match, "{clip}: capsule-driven, the take's heading drift is not applied");
    }
    for weapon in ["walk.1h", "run.2c", "walk.hh", "run.crossbow.lower", "sneak.none"] {
        assert!(matches!(a.nodes[weapon], NodeSpec::BlendSpace2D { .. }), "{weapon}: Morrowind set");
    }
    assert!(a.params.iter().any(|q| q.name == p::SPEED));
    for t in [Template::FirstPerson, Template::Creature] {
        let a = build::asset(t);
        assert!(!a.nodes.values().any(|n| matches!(n, NodeSpec::BlendSpace1D { .. })) && !a.params.iter().any(|q| q.name == p::SPEED), "{t:?}");
    }
    // The gait speeds are sf_action's walk, run and sprint (the same literals).
    let g = sf_action::GaitTable::default();
    #[allow(clippy::float_cmp)]
    let same = build::gaits::SET.map(|(_, at)| at) == [g.walk.forward, g.run.forward, g.sprint.forward];
    assert!(same, "gait speeds");
}

/// The style seam (ROADMAP 3.4d): `loco.<style>.<gait>.<part>` first, else the base set's
/// `loco.<gait>.<part>`, else nothing; only slots of the asked graph with candidates count.
#[test]
fn gait_slots_look_up_the_style_then_the_base_set() {
    use crate::sources::SourcesManifest;
    let text = r#"(
      sources: { "g": (license: CC0, page: "", files: [(name: "a.bvh", sha256: "")]) },
      slots: {
        "loco.walk.cycle": (graph: "humanoid", candidates: [(source: "g", clip: "w")]),
        "loco.jog.cycle": (graph: "humanoid", candidates: [(source: "g", clip: "j")]),
        "loco.tired.walk.cycle": (graph: "humanoid", candidates: [(source: "g", clip: "tw")]),
        "loco.tired.jog.cycle": (graph: "humanoid", candidates: []),
        "loco.sprint.cycle": (graph: "creature", candidates: [(source: "g", clip: "s")]),
      },
    )"#;
    let m = SourcesManifest::parse(text).unwrap();
    let slot = |style, gait| m.gait_slot("humanoid", style, gait, "cycle");
    assert_eq!(slot(Some("tired"), "walk").as_deref(), Some("loco.tired.walk.cycle"));
    assert_eq!(slot(Some("tired"), "jog").as_deref(), Some("loco.jog.cycle"), "no candidate: the base set");
    assert_eq!(slot(Some("drunk"), "walk").as_deref(), Some("loco.walk.cycle"), "unknown style: the base set");
    assert_eq!(slot(None, "walk").as_deref(), Some("loco.walk.cycle"));
    assert_eq!(slot(Some("tired"), "sprint"), None, "another graph's slot");
    assert_eq!(m.gait_slot("creature", None, "sprint", "cycle").as_deref(), Some("loco.sprint.cycle"));
    // The shipped manifest: base set only.
    let shipped = SourcesManifest::builtin();
    assert_eq!(build::gaits::STYLE, None);
    for (gait, _) in build::gaits::SET {
        assert_eq!(shipped.gait_slot("humanoid", Some("any"), gait, "cycle"), Some(format!("loco.{gait}.cycle")));
    }
    assert!(build::gaits::is_gait_clip("loco.tired.walk.cycle.alt2") && !build::gaits::is_gait_clip("loco.walk.fwd"));
}

/// Classic (the seam for the Classic option) is the graph as before the gait set: the
/// Morrowind walk and run sets, no gait clips, params or nodes; its digest is pinned
/// (pre-A3 shipped humanoid graph). Change it only on purpose.
#[test]
fn classic_keeps_the_morrowind_graph() {
    use sf_anim_graph::asset::NodeSpec;
    let a = build::asset_for(Template::Humanoid, build::Preset::Classic);
    assert!(!a.clips.keys().any(|c| c.starts_with("loco.")));
    assert!(!a.params.iter().any(|q| q.name == p::SPEED || q.name == p::STRAFING));
    assert!(!a.nodes.values().any(|n| matches!(n, NodeSpec::BlendSpace1D { .. })));
    for state in ["walk.none", "run.none"] {
        assert!(matches!(a.nodes[state], NodeSpec::BlendSpace2D { .. }), "{state}");
    }
    assert_eq!(a.slot_clips, build::asset(Template::Humanoid).slot_clips, "the montage slots are the same");
    for t in [Template::FirstPerson, Template::Creature] {
        assert_eq!(build::asset_for(t, build::Preset::Classic), build::asset(t), "{t:?}");
    }
    // FNV-1a 64 of the asset's Debug text.
    let digest = format!("{a:?}").bytes().fold(0xcbf2_9ce4_8422_2325u64, |h, b| (h ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3));
    if let Some(path) = std::env::var_os("MW_GRAPHS_PRE_A3") {
        let pre = parse_graph(&std::fs::read_to_string(path).unwrap()).unwrap();
        assert_eq!(pre, a, "Classic vs the pre-A3 shipped graph");
    }
    assert_eq!(digest, CLASSIC_DIGEST, "Classic humanoid graph changed: digest {digest:#x}");
}

const CLASSIC_DIGEST: u64 = 0xe03c_50a5_73ac_8c28;
