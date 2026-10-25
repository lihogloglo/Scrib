//! Rig maps, masks and creature profiles on the real game skeletons. Skips
//! (passes) when no Morrowind install is detected. Per-file listing:
//! `cargo run -p mw_anim --example rig_report -- meshes/base_anim.nif --humanoid`.

use glam::{Affine3A, Quat, Vec3};
use mw_anim::rig::{self, BodyGroup, CreatureMover};
use mw_install::MorrowindInstall;
use sf_anim::{AnimImporter, GltfAnimImporter, LegKind, LimbKind, LimbSolver, Retargeter, RigTemplate, Side, Skeleton, Trs};
use sf_character::{HumanoidBone as B, RetargetOptions, RigMap};
use sf_vfs::{Vfs, VfsPath};

const HUMANOIDS: [&str; 4] = ["meshes/base_anim.nif", "meshes/base_anim_female.nif", "meshes/base_animkna.nif", "meshes/base_anim.1st.nif"];

fn install() -> Option<(MorrowindInstall, Vfs)> {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return None;
    };
    let vfs = mw_vfs::mount_install(&install, Vfs::builder()).unwrap().vfs;
    Some((install, vfs))
}

fn load_skeleton(vfs: &Vfs, path: &str) -> Skeleton {
    let bytes = vfs.read(&VfsPath::new(path).unwrap()).unwrap_or_else(|e| panic!("{path}: {e}"));
    rig::skeleton(&mw_nif::load(&bytes).unwrap()).unwrap()
}

fn model_of(skeleton: &Skeleton, local: &[Trs]) -> Vec<Affine3A> {
    let mut out: Vec<Affine3A> = Vec::with_capacity(local.len());
    for (j, t) in local.iter().enumerate() {
        let m = t.to_affine();
        out.push(skeleton.parent(j).map_or(m, |p| out[p] * m));
    }
    out
}

#[test]
fn humanoid_skeletons_map_with_the_hint_table_only() {
    let Some((_, vfs)) = install() else { return };
    for path in HUMANOIDS {
        let s = load_skeleton(&vfs, path);
        let rig = rig::humanoid(&s).unwrap();
        let (map, report) = (&rig.map, &rig.report);
        assert!(report.is_clean() && report.missing_required.is_empty(), "{path}: {report:#?}");
        assert!(map.validate(&s).is_empty(), "{path}");
        assert_eq!(map.root_motion, "Bip01", "{path}");
        assert_eq!(s.name(s.root_motion_joint().unwrap()), "Bip01", "{path}");
        for (bone, joint) in [
            (B::Hips, "Bip01 Pelvis"),
            (B::Spine, "Bip01 Spine"),
            (B::Chest, "Bip01 Spine1"),
            (B::UpperChest, "Bip01 Spine2"),
            (B::Neck, "Bip01 Neck"),
            (B::Head, "Bip01 Head"),
            (B::LeftShoulder, "Bip01 L Clavicle"),
            (B::LeftUpperArm, "Bip01 L UpperArm"),
            (B::LeftLowerArm, "Bip01 L Forearm"),
            (B::RightHand, "Bip01 R Hand"),
                        (B::RightIndexProximal, "Bip01 R Finger1"),
            (B::LeftUpperLeg, "Bip01 L Thigh"),
            (B::LeftLowerLeg, "Bip01 L Calf"),
            (B::RightFoot, "Bip01 R Foot"),
        ] {
            assert_eq!(map.joint(bone), Some(joint), "{path}: {bone:?}");
        }
        // Body-part sockets are not bones: no one-joint extra chains.
        assert!(map.bones.iter().all(|(j, _)| rig::is_bone(j)), "{path}");
        let beast = path.contains("kna");
        let chains: Vec<&str> = map.extra_chains.iter().map(|c| c.name.as_str()).collect();
        assert_eq!(chains, if beast { vec!["tail"] } else { Vec::<&str>::new() }, "{path}");
        if beast {
            assert_eq!(map.extra_chains[0].joints.len(), 4, "Bip01 Tail..Tail3");
            assert_eq!(map.joint(B::LeftToes), Some("Bip01 L Toe0"));
        }
        // Retarget pose: Bip01 faces +Z after the import turn, so no facing fix.
        assert!(rig.align.facing_correction_deg < 0.5, "{path}: {:?}", rig.align.facing_correction_deg);
        assert!(rig.align.max_residual_deg < 1.0, "{path}");
        // Retarget rig and profile build; two support legs.
        map.retarget_rig(&s, RetargetOptions::default()).unwrap();
        let profile = map.profile(&s).unwrap();
        assert_eq!(profile.template, RigTemplate::Biped);
        assert_eq!(profile.support_limbs().count(), 2, "{path}");
        assert_eq!(profile.tails.len(), usize::from(beast), "{path}");
    }
    // The first-person skeleton has full three-segment fingers.
    let first = rig::humanoid(&load_skeleton(&vfs, HUMANOIDS[3])).unwrap();
    assert_eq!(first.map.bones.len(), 52);
    assert_eq!(first.map.joint(B::LeftLittleDistal), Some("Bip01 L Finger42"));
}

#[test]
fn masks_partition_the_skeleton_into_the_four_groups() {
    let Some((_, vfs)) = install() else { return };
    for path in HUMANOIDS {
        let s = load_skeleton(&vfs, path);
        let rig = rig::humanoid(&s).unwrap();
        let masks = rig::masks(&s, &rig.map);
        let group_of = |name: &str| {
            let j = s.find(name).unwrap_or_else(|| panic!("{path}: {name}"));
            let groups: Vec<BodyGroup> = masks.iter().filter(|(_, m)| m.contains(j)).map(|(g, _)| *g).collect();
            assert_eq!(groups.len(), 1, "{path}: {name} in {groups:?}");
            groups[0]
        };
        for j in 0..s.len() {
            group_of(s.name(j));
        }
        for (name, group) in [
            ("Bip01", BodyGroup::LowerBody),
            ("Bip01 Pelvis", BodyGroup::LowerBody),
            ("Bip01 Spine", BodyGroup::LowerBody),
            ("Bip01 L Thigh", BodyGroup::LowerBody),
            ("Bip01 R Foot", BodyGroup::LowerBody),
            ("Groin", BodyGroup::LowerBody),
            ("Bip01 Spine1", BodyGroup::Torso),
            ("Bip01 Spine2", BodyGroup::Torso),
            ("Bip01 Neck", BodyGroup::Torso),
            ("Bip01 Head", BodyGroup::Torso),
            ("Chest", BodyGroup::Torso),
            ("Bip01 L Clavicle", BodyGroup::LeftArm),
            ("Bip01 L Finger21", BodyGroup::LeftArm),
            ("Shield Bone", BodyGroup::LeftArm),
            ("Left Hand", BodyGroup::LeftArm),
            ("Bip01 R UpperArm", BodyGroup::RightArm),
            ("Weapon Bone", BodyGroup::RightArm),
        ] {
            assert_eq!(group_of(name), group, "{path}: {name}");
        }
        if path.contains("kna") {
            assert_eq!(group_of("Bip01 Tail3"), BodyGroup::LowerBody);
        }
    }
}

fn quaternius_universal() -> Skeleton {
    let bytes = include_bytes!("../../../../superfargoth/crates/sf_character/tests/data/quaternius_universal_rig.gltf");
    let src = GltfAnimImporter.import("quaternius_universal_rig.gltf", bytes).expect("fixture imports");
    Skeleton::new(&src.skeleton.expect("skin joints")).expect("valid").0
}

/// Quaternius-style humanoid → each Bip01 skeleton → back: mapped bones
/// body bones within 1° (fingers within 5°: fewer segments), hips, left hand
/// and right foot within 1 cm.
#[test]
#[allow(clippy::many_single_char_names)]
fn retarget_round_trip_quaternius_bip01() {
    let Some((_, vfs)) = install() else { return };
    let s = quaternius_universal();
    let (mut sm, sr): (RigMap, _) = sf_character::auto_map(&s);
    assert!(sr.is_clean(), "{sr:#?}");
    sm.auto_align(&s);
    let options = RetargetOptions { ik_legs: false };
    let s_rig = sm.retarget_rig(&s, options).unwrap();

    // A source pose: the retarget pose with every mapped joint bent, the hips moved.
    let mut pose = sm.retarget_pose_local(&s);
    for (k, (name, _)) in sm.bones.iter().enumerate() {
        let j = s.find(name).unwrap();
        #[allow(clippy::cast_precision_loss)]
        let a = 0.3 + 0.05 * k as f32;
        pose[j].rotation = (pose[j].rotation * Quat::from_euler(glam::EulerRot::XYZ, 0.4 * a.sin(), 0.3 * a.cos(), 0.2)).normalize();
    }
    let hips = s.find(sm.joint(B::Hips).unwrap()).unwrap();
    pose[hips].translation += Vec3::new(0.05, -0.1, 0.2);
    let a = model_of(&s, &pose);
    let rot = |m: &Affine3A| m.to_scale_rotation_translation().1;

    for path in HUMANOIDS {
        let t = load_skeleton(&vfs, path);
        let tm = rig::humanoid(&t).unwrap().map;
        let t_rig = tm.retarget_rig(&t, options).unwrap();
        let fwd = Retargeter::new(&s, &s_rig, &t, &t_rig).unwrap();
        let back = Retargeter::new(&t, &t_rig, &s, &s_rig).unwrap();
        let mut mid = vec![Trs::IDENTITY; t.len()];
        let mut out = vec![Trs::IDENTITY; s.len()];
        fwd.pose(&pose, &[], &mut mid);
        back.pose(&mid, &[], &mut out);
        let b = model_of(&s, &out);
        // Body bones, and fingers apart: Bip01 fingers have two segments (three in first person) against
        // Quaternius's three, so the chain pairing by arc length can't carry the source's distal bend through.
        let (mut worst, mut worst_bone, mut worst_finger) = (0.0f32, B::Hips, 0.0f32);
        for (name, bone) in &sm.bones {
            if tm.joint(*bone).is_none() {
                continue;
            }
            let j = s.find(name).unwrap();
            let e = rot(&a[j]).angle_between(rot(&b[j])).to_degrees();
            if bone.finger_of().is_some() {
                worst_finger = worst_finger.max(e);
            } else if e > worst {
                (worst, worst_bone) = (e, *bone);
            }
        }
        assert!(worst < 1.0, "{path}: worst mapped bone {worst_bone:?} {worst}°");
        assert!(worst_finger < 5.0, "{path}: worst finger {worst_finger}°");
        for (what, j) in [("hips", hips), ("left hand", s.find(sm.joint(B::LeftHand).unwrap()).unwrap()), ("right foot", s.find(sm.joint(B::RightFoot).unwrap()).unwrap())] {
            let d = Vec3::from(a[j].translation).distance(Vec3::from(b[j].translation));
            assert!(d < 0.01, "{path}: {what} drift {d} m");
        }
        eprintln!("{path}: round trip worst body bone {worst:.3}°, finger {worst_finger:.2}°");
    }
}

/// CREA `(id, model, flags)` from `Morrowind.esm`.
fn creatures(install: &MorrowindInstall) -> Vec<(String, String, u32)> {
    let bytes = std::fs::read(install.data_file("Morrowind.esm").expect("Morrowind.esm")).unwrap();
    let mut plugin = tes3::esp::Plugin::new();
    plugin.load_bytes(&bytes).unwrap();
    plugin.objects_of_type::<tes3::esp::Creature>().map(|c| (c.id.to_ascii_lowercase(), c.mesh.clone(), u32::from(c.creature_flags.bits()))).collect()
}

#[test]
fn creature_profiles_and_movers() {
    let Some((install, vfs)) = install() else { return };
    let crea = creatures(&install);
    let find = |id: &str| crea.iter().find(|c| c.0 == id).unwrap_or_else(|| panic!("CREA {id}"));
    // (id, template, support legs, legs with solver TwoBoneAnkle, wings, mover)
    let table: &[(&str, RigTemplate, usize, usize, usize, CreatureMover)] = &[
        ("guar", RigTemplate::Biped, 2, 2, 0, CreatureMover::Ground),
        ("nix-hound", RigTemplate::Quadruped, 4, 4, 0, CreatureMover::Ground),
        ("kagouti", RigTemplate::Biped, 2, 2, 0, CreatureMover::Ground),
        ("cliff racer", RigTemplate::Flyer, 0, 0, 2, CreatureMover::Fly),
        ("centurion_spider", RigTemplate::Multiped, 6, 0, 0, CreatureMover::Ground),
    ];
    for &(id, template, support, ankle, wings, mover) in table {
        let (_, mesh, flags) = find(id);
        let path = format!("meshes/{}", mesh.replace('\\', "/"));
        let s = load_skeleton(&vfs, &path);
        let p = rig::creature_profile(&s, mesh).unwrap();
        let summary = rig::profile_summary(&s, &p);
        assert_eq!(p.template, template, "{id}\n{summary}");
        assert_eq!(p.support_limbs().count(), support, "{id}\n{summary}");
        assert_eq!(p.support_limbs().filter(|l| l.solver == LimbSolver::TwoBoneAnkle).count(), ankle, "{id}\n{summary}");
        assert_eq!(p.limbs.iter().filter(|l| l.kind == LimbKind::Wing).count(), wings, "{id}\n{summary}");
        assert_eq!(CreatureMover::from_crea_flags(*flags), mover, "{id}");
        // Sides: a left/right pair per support row.
        let left = p.support_limbs().filter(|l| l.side == Side::Left).count();
        assert_eq!(left * 2, support, "{id}\n{summary}");
    }
    // The guar's hind legs: Thigh, Calf, HorseLink, Foot, digitigrade (detected, no hints).
    let guar = load_skeleton(&vfs, "meshes/r/guar.nif");
    let p = rig::creature_profile(&guar, "r\\Guar.NIF").unwrap();
    let leg = p.limb("leg_l").expect("leg_l");
    assert_eq!(leg.kind, LimbKind::Leg(LegKind::Digitigrade));
    assert_eq!(guar.name(usize::from(leg.joints[2])), "Bip01 L HorseLink");
    assert_eq!(p.tails.len(), 1);
    // The cliff racer: wings to the tip caps, a tail, the rudder a fin.
    let racer = load_skeleton(&vfs, "meshes/r/cliffracer.nif");
    let p = rig::creature_profile(&racer, "r\\CliffRacer.NIF").unwrap();
    assert_eq!(p.limb("wing_l").unwrap().joints.len(), 5);
    assert_eq!(p.limb("wing_l").unwrap().side, Side::Left);
    assert_eq!(p.limb("rudder").unwrap().kind, LimbKind::Fin);
    assert_eq!(p.tails.len(), 1);
    // The centurion spider (no hints): six three-joint legs on the body joint.
    let spider = load_skeleton(&vfs, "meshes/r/g_centurionspider.nif");
    assert!(rig::creature_hints("r\\G_CenturionSpider.NIF").limbs.is_empty());
    let p = rig::creature_profile(&spider, "r\\G_CenturionSpider.NIF").unwrap();
    assert_eq!(p.girdles.iter().map(|g| spider.name(usize::from(g.joint))).collect::<Vec<_>>(), ["Bone00"]);
    for l in p.support_limbs() {
        assert_eq!((l.joints.len(), l.solver), (3, LimbSolver::TwoBone), "{}", l.name);
        assert!(spider.name(usize::from(l.joints[0])).ends_with("_A"), "{}", l.name);
    }
    // The kagouti stands on its Clavicle..Hand chains (its Thigh..Toe0 are a
    // 4 cm stub inside the body).
    let kagouti = load_skeleton(&vfs, "meshes/r/leastkagouti.nif");
    let p = rig::creature_profile(&kagouti, "r\\LeastKagouti.NIF").unwrap();
    assert_eq!(kagouti.name(usize::from(p.limb("leg_l").expect("leg_l").joints[0])), "Bip01 L Clavicle");
    // Floaters and swimmers by flags.
    assert_eq!(CreatureMover::from_crea_flags(find("netch_betty").2), CreatureMover::Fly);
    assert_eq!(CreatureMover::from_crea_flags(find("slaughterfish").2), CreatureMover::Swim);
}
