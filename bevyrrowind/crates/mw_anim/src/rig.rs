//! Morrowind skeletons as `sf_anim` skeletons, their humanoid rig maps,
//! blend masks and creature rig profiles.
//!
//! - [`skeleton`] turns a converted NIF (`base_anim.nif`, a creature model)
//!   into an `sf_anim::Skeleton`: its transform nodes, engine space.
//! - [`humanoid`] maps a 3ds Max Biped (`Bip01 …`) skeleton onto the
//!   framework's humanoid through `sf_character::auto_map` and
//!   `auto_align`, with root motion on `Bip01` (the joint that travels in
//!   the KFs) and the small hint table [`humanoid_hints`].
//! - [`masks`] gives the four Morrowind blend groups (lower body, torso,
//!   left arm, right arm) as joint masks.
//! - [`creature_hints`] and [`creature_profile`] give `sf_anim::rig`
//!   detection what it can't infer from a creature skeleton alone;
//!   [`CreatureMover`] picks the mover from CREA flags.

use mw_nif::{Model, NodeKind};
use sf_anim::{
    JointMask, LegKind, LimbHint, LimbKind, LimbSolver, RigError, RigHints, RigProfile, Skeleton, SkeletonError, SkeletonRules, SourceJoint, Trs,
    detect_profile,
};
use sf_character::{AlignReport, AutoMapReport, HumanoidBone, RigMap, auto_map};

/// The joint that carries root motion in Biped-rigged actor KFs (every NPC,
/// most creatures).
pub const ROOT_MOTION_JOINT: &str = "Bip01";

/// Names of the joint under which an actor's bones hang, in preference
/// order: [`ROOT_MOTION_JOINT`], else `Root Bone` (15 vanilla creature
/// skeletons: cliff racer, centurion spider and sphere, shalk, scrib,
/// kwama forager, bonelord, netches, dreugh, fish, mudcrab). The first one
/// present is the root-motion joint: in every vanilla CREA KF it is the
/// joint the walk and run cycles translate (`creature_root_motion` census,
/// `mw_bake` tests, 2026-10-12).
pub const SKELETON_ROOTS: [&str; 2] = [ROOT_MOTION_JOINT, "Root Bone"];

/// The root-motion joint among `names` (joint or track names, any case):
/// the first [`SKELETON_ROOTS`] name present, as spelled in `names`.
pub fn root_motion_joint<'a>(names: impl Iterator<Item = &'a str> + Clone) -> Option<&'a str> {
    SKELETON_ROOTS.iter().find_map(|r| names.clone().find(|n| n.eq_ignore_ascii_case(r)))
}

/// Morrowind actors face −Z in engine space (+Y in the NIF's Z-up space);
/// the framework's characters and retargeted clips face +Z. [`skeleton`]
/// puts this turn (180° about Y) on the root joint's rest rotation, so the
/// skeleton, its KF clips (they animate `Bip01` and below, in the root's
/// frame) and their root motion (extracted in model space) all face +Z.
/// Actor spawning adds the same half turn to a reference's yaw to keep the
/// vanilla world orientation.
pub const FACING_TURN: glam::Quat = glam::Quat::from_xyzw(0.0, 1.0, 0.0, 0.0);

/// The actor skeleton of a converted model.
///
/// Joints: the model root, then, when the root holds a [`SKELETON_ROOTS`]
/// joint, only that joint's subtree (the root's other children are meshes,
/// shadow boxes, `Bounding Box` and editor dummies, some placed below the
/// ground), plus the root's transform children that hold unskinned
/// geometry (rigid parts animated outside the bones, such as the storm
/// atronach's orbiting rocks: they need joints to follow their keys);
/// otherwise every transform node. Geometry, particle and
/// collision subtrees are left out. Body-part sockets (`Head`, `Left Hand`,
/// `Weapon Bone`, `Tail`…) stay: parts attach to them. Duplicate names get
/// a ` #n` suffix. The root's rest rotation gains [`FACING_TURN`]. Root
/// motion goes on the bones root ([`root_motion_joint`]: `Bip01`, else
/// `Root Bone`); tags are guessed from names.
pub fn skeleton(model: &Model) -> Result<Skeleton, SkeletonError> {
    let (joints, rules) = skeleton_source(model);
    Ok(Skeleton::with_rules(&joints, &rules)?.0)
}

/// What [`skeleton`] builds from: the joints and the rules (root motion on
/// [`root_motion_joint`], guessed tags). The bake's
/// `skeleton.cook` reads NIFs through this too (`NifAnimImporter`), so its
/// product equals [`skeleton`].
pub fn skeleton_source(model: &Model) -> (Vec<SourceJoint>, SkeletonRules) {
    let (joints, _) = source_joints(model);
    let root_motion_joint = root_motion_joint(joints.iter().map(|j| j.name.as_str())).map(str::to_owned);
    let rules = SkeletonRules { root_motion_joint, guess_tags: true, ..SkeletonRules::default() };
    (joints, rules)
}

/// The skeleton joints that are `NiBillboardNode`s: the game turns them to
/// face the camera after the pose (nif-runtime.md "Billboards in a rig").
pub fn billboard_joints(model: &Model) -> Vec<String> {
    let (joints, nodes) = source_joints(model);
    joints.into_iter().zip(nodes).filter(|(_, n)| model.nodes[*n].kind == NodeKind::Billboard).map(|(j, _)| j.name).collect()
}

/// The joints of [`skeleton_source`], with the node each one comes from.
/// Billboard nodes are joints too: their subtrees (`heart_akulakhan`'s rays)
/// hang from them and the game turns them to face the camera.
fn source_joints(model: &Model) -> (Vec<SourceJoint>, Vec<usize>) {
    let is_transform = |k: &NodeKind| matches!(k, NodeKind::Node | NodeKind::Switch { .. } | NodeKind::Billboard);
    let bones_root = model.nodes.iter().position(|n| {
        is_transform(&n.kind) && n.parent.is_some_and(|p| model.nodes[p].parent.is_none()) && SKELETON_ROOTS.contains(&n.name.as_str())
    });
    // Root children holding unskinned geometry: rigid parts on their own branch.
    let mut rigid_branch = vec![false; model.nodes.len()];
    for (i, node) in model.nodes.iter().enumerate() {
        if node.mesh.is_none_or(|m| model.meshes.get(m).is_none_or(|m| m.skinned)) {
            continue;
        }
        let mut at = i;
        for _ in 0..model.nodes.len() {
            match model.nodes[at].parent {
                Some(p) if model.nodes[p].parent.is_some() => at = p,
                Some(_) => {
                    rigid_branch[at] = true;
                    break;
                }
                None => break,
            }
        }
    }
    let mut index = vec![None; model.nodes.len()];
    let mut joints: Vec<SourceJoint> = Vec::new();
    let mut nodes: Vec<usize> = Vec::new();
    let mut seen = std::collections::HashMap::<String, usize>::new();
    for (i, node) in model.nodes.iter().enumerate() {
        if !is_transform(&node.kind) {
            continue;
        }
        let parent = match node.parent {
            Some(p) => match index[p] {
                // Under the root, only the bones root when there is one
                // (and branches of rigid parts).
                Some(j) if model.nodes[p].parent.is_some() || bones_root.is_none_or(|b| b == i) || rigid_branch[i] => Some(j),
                _ => continue,
            },
            None => None,
        };
        let count = seen.entry(node.name.clone()).or_insert(0);
        *count += 1;
        let name = if *count == 1 { node.name.clone() } else { format!("{} #{count}", node.name) };
        let t = &node.transform;
        let mut rest = Trs {
            translation: t.translation.into(),
            rotation: glam::Quat::from_array(t.rotation).normalize(),
            scale: glam::Vec3::splat(t.scale),
        };
        if parent.is_none() {
            rest.translation = FACING_TURN * rest.translation;
            rest.rotation = FACING_TURN * rest.rotation;
        }
        index[i] = Some(joints.len());
        joints.push(SourceJoint { name, parent, rest });
        nodes.push(i);
    }
    (joints, nodes)
}

// ---------------------------------------------------------------------------
// Humanoid rig maps.

/// The humanoid hint table: everything `sf_character::auto_map` needs
/// beyond the skeleton itself, for `base_anim.nif`, `base_anim_female.nif`,
/// `base_animkna.nif` and `base_anim.1st.nif` (2026-10-12: all four map
/// with no warnings once these apply).
pub mod humanoid_hints {
    /// Only `Bip01*` joints are bones. The rest are body-part sockets
    /// (`Head`, `Neck`, `Chest`, `Groin`, `Left Hand`, `Right Upper Leg`,
    /// `Tail`…: the part slots' attachment nodes), `Weapon Bone`/`Shield
    /// Bone`, the first-person `Camera` and an unnamed node under it, and
    /// `Bounding Box`. Mapped with them, the sockets become one-joint extra
    /// chains (one named `head`, colliding with the head chain) and cut the
    /// beast tail chain at its `Tail` socket.
    pub const BONE_PREFIX: &str = "Bip01";
    /// Root motion: `Bip01` travels in the KFs (auto-map would pick the
    /// hips, `Bip01 Pelvis`).
    pub const ROOT_MOTION: &str = super::ROOT_MOTION_JOINT;
}

/// A Morrowind humanoid skeleton's rig map and what auto-mapping and
/// auto-aligning it reported.
#[derive(Clone, Debug)]
pub struct HumanoidRig {
    /// Joint names of the full skeleton; retarget pose filled.
    pub map: RigMap,
    pub report: AutoMapReport,
    pub align: AlignReport,
}

/// Whether a joint name is a humanoid bone ([`humanoid_hints::BONE_PREFIX`]).
pub fn is_bone(name: &str) -> bool {
    name.get(..humanoid_hints::BONE_PREFIX.len()).is_some_and(|p| p.eq_ignore_ascii_case(humanoid_hints::BONE_PREFIX))
}

/// The bones of a skeleton as a skeleton of their own: the root plus every
/// [`is_bone`] joint, each under its nearest kept ancestor, with the same
/// model-space rest pose. Joint names are unchanged, so a rig map built on
/// it holds for the full skeleton.
pub fn bone_skeleton(skeleton: &Skeleton) -> Result<Skeleton, SkeletonError> {
    let model = skeleton.rest_model();
    let mut index = vec![None; skeleton.len()];
    let mut joints: Vec<SourceJoint> = Vec::new();
    for j in 0..skeleton.len() {
        let mut p = skeleton.parent(j);
        if p.is_some() && !is_bone(skeleton.name(j)) {
            continue;
        }
        while let Some(q) = p.filter(|&q| index[q].is_none()) {
            p = skeleton.parent(q);
        }
        let rest = match p {
            Some(q) => {
                let (scale, rotation, translation) = (model[q].inverse() * model[j]).to_scale_rotation_translation();
                Trs { translation, rotation: rotation.normalize(), scale }
            }
            None => skeleton.rest()[j],
        };
        index[j] = Some(joints.len());
        joints.push(SourceJoint { name: skeleton.name(j).to_owned(), parent: p.and_then(|q| index[q]), rest });
    }
    Ok(Skeleton::new(&joints)?.0)
}

/// Maps a 3ds Max Biped skeleton (from [`skeleton`]) onto the humanoid:
/// `auto_map` on its [`bone_skeleton`], root motion on `Bip01`, then
/// `auto_align` for the retarget pose (Bip01's rest is a relaxed A-pose).
pub fn humanoid(skeleton: &Skeleton) -> Result<HumanoidRig, SkeletonError> {
    let bones = bone_skeleton(skeleton)?;
    let (mut map, report) = auto_map(&bones);
    if skeleton.find(humanoid_hints::ROOT_MOTION).is_some() {
        humanoid_hints::ROOT_MOTION.clone_into(&mut map.root_motion);
    }
    let align = map.auto_align(skeleton);
    Ok(HumanoidRig { map, report, align })
}

// ---------------------------------------------------------------------------
// Blend masks.

/// Morrowind's four animation blend groups: a weapon or spell layer plays
/// on the arms and torso over locomotion on the lower body.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BodyGroup {
    /// Everything not in another group: root, `Bip01`, pelvis, lowest
    /// spine joint, legs, tail.
    LowerBody,
    /// The chest bone's subtree (`Bip01 Spine1`: upper spine, neck, head)
    /// minus the arms.
    Torso,
    /// The left shoulder's subtree (`Bip01 L Clavicle`: arm, hand, fingers,
    /// shield bone, sockets).
    LeftArm,
    RightArm,
}

impl BodyGroup {
    pub const ALL: [Self; 4] = [Self::LowerBody, Self::Torso, Self::LeftArm, Self::RightArm];
}

/// The four groups as joint masks over `skeleton`, from the humanoid map
/// (group roots: chest, left shoulder, right shoulder, which are the
/// original engine's `Bip01 Spine1`, `Bip01 L Clavicle`, `Bip01 R
/// Clavicle`). They partition the skeleton. A group whose root bone isn't
/// mapped is empty (its joints stay in the group above).
pub fn masks(skeleton: &Skeleton, map: &RigMap) -> [(BodyGroup, JointMask); 4] {
    let root = |bone: HumanoidBone| map.joint(bone).and_then(|j| skeleton.find(j));
    let mut group = vec![BodyGroup::LowerBody; skeleton.len()];
    for (g, bone) in [(BodyGroup::Torso, HumanoidBone::Chest), (BodyGroup::LeftArm, HumanoidBone::LeftShoulder), (BodyGroup::RightArm, HumanoidBone::RightShoulder)] {
        if let Some(r) = root(bone) {
            for j in skeleton.subtree(r) {
                group[j] = g;
            }
        }
    }
    BodyGroup::ALL.map(|g| {
        let mut mask = JointMask::empty(skeleton.len());
        for (j, _) in group.iter().enumerate().filter(|(_, x)| **x == g) {
            mask.insert(j);
        }
        (g, mask)
    })
}

// ---------------------------------------------------------------------------
// Creatures.

/// The lowercase file stem of a model path: `r\NixHound.NIF` → `nixhound`.
pub fn model_key(model_path: &str) -> String {
    let file = model_path.rsplit(['\\', '/']).next().unwrap_or(model_path);
    let stem = file.rfind('.').map_or(file, |i| &file[..i]);
    stem.to_ascii_lowercase()
}

fn limb(name: &str, joints: &[&str], kind: LimbKind, solver: LimbSolver, support: bool) -> LimbHint {
    LimbHint {
        name: name.to_owned(),
        joints: joints.iter().map(|&j| j.to_owned()).collect(),
        kind: Some(kind),
        solver: Some(solver),
        support: Some(support),
        pole: None,
    }
}

/// Rig hints for a creature model (by [`model_key`]) where `sf_anim::rig`
/// detection alone gets a limb wrong (checked on the vanilla data
/// 2026-10-12; actors-adapter.md "Rig maps as built"). Other models get
/// none.
///
/// - `nixhound`: its rest pose is mid-stride, so detection read one front
///   leg as plantigrade and stopped the hind legs at `Foot` (the hock,
///   0.7 m up). All four are digitigrade to the middle toe or finger.
/// - `cliffracer`: the wings branch at every joint (membrane struts), so
///   detection kept only their first joint; the main spar runs to the tip
///   cap (the right one's third joint is named `R Wing_4` in the data). The
///   lower tail rudder is a fin, not an arm.
///
/// The centurion spider needed its six legs listed until `sf_anim` learned
/// to stop the leg walk at a joint carrying legs on both sides (rigfix,
/// 2026-10-12); it now detects alone.
pub fn creature_hints(model_path: &str) -> RigHints {
    use LimbKind::{Fin, Leg, Wing};
    use LimbSolver::{Fabrik, TwoBoneAnkle};
    let digi = Leg(LegKind::Digitigrade);
    let limbs = match model_key(model_path).as_str() {
        "nixhound" => vec![
            limb("leg_fl", &["Bip01 L Clavicle", "Bip01 L UpperArm", "Bip01 L Forearm", "Bip01 L Hand", "Bip01 L Finger1"], digi, TwoBoneAnkle, true),
            limb("leg_fr", &["Bip01 R Clavicle", "Bip01 R UpperArm", "Bip01 R Forearm", "Bip01 R Hand", "Bip01 R Finger1"], digi, TwoBoneAnkle, true),
            limb("leg_bl", &["Bip01 L Thigh", "Bip01 L Calf", "Bip01 L Foot", "Bip01 L Toe1"], digi, TwoBoneAnkle, true),
            limb("leg_br", &["Bip01 R Thigh", "Bip01 R Calf", "Bip01 R Foot", "Bip01 R Toe1"], digi, TwoBoneAnkle, true),
        ],
        "cliffracer" => vec![
            limb("wing_l", &["L Wing", "L Wing_1", "L Wing_2", "L Wing_3", "L WingCap"], Wing, Fabrik, false),
            limb("wing_r", &["R Wing", "R Wing_1", "R Wing_2", "R Wing_4", "R WingCap"], Wing, Fabrik, false),
            limb("rudder", &["BotRudder", "BotRudder_1", "BotRudder_2", "BotRudder_3", "BotRudderCap"], Fin, LimbSolver::None, false),
        ],
        _ => Vec::new(),
    };
    RigHints { limbs, ..RigHints::default() }
}

/// A creature's rig profile: `sf_anim::detect_profile` with
/// [`creature_hints`].
pub fn creature_profile(skeleton: &Skeleton, model_path: &str) -> Result<RigProfile, RigError> {
    detect_profile(skeleton, &creature_hints(model_path))
}

/// CREA `FLAG` bits that pick the mover.
pub mod crea_flags {
    pub const BIPED: u32 = 0x01;
    pub const SWIMS: u32 = 0x10;
    pub const FLIES: u32 = 0x20;
    pub const WALKS: u32 = 0x40;
}

/// How a creature starts moving. The controller glue maps it onto
/// `sf_controller::MoverMode` (this crate stays free of the controller and
/// its physics dependency).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CreatureMover {
    /// Walking on the ground (`MoverMode::Ground`).
    Ground,
    /// Flying or floating in 3D (`MoverMode::Fly`; netches float).
    Fly,
    /// Swimming only (`MoverMode::Swim`): slaughterfish, dreugh.
    Swim,
}

impl CreatureMover {
    /// From CREA flags: `Flies` → [`Fly`](Self::Fly); `Swims` without
    /// `Walks` → [`Swim`](Self::Swim); anything else walks, including
    /// creatures that set none of the three (vanilla biped creatures such
    /// as dremora carry only `Biped`) and amphibious ones (`Walks | Swims`
    /// start on the ground; switching in water is a gameplay rule).
    pub fn from_crea_flags(flags: u32) -> Self {
        if flags & crea_flags::FLIES != 0 {
            Self::Fly
        } else if flags & crea_flags::SWIMS != 0 && flags & crea_flags::WALKS == 0 {
            Self::Swim
        } else {
            Self::Ground
        }
    }
}

/// A multi-line, human-readable summary of a profile (template, limbs with
/// kind, solver, side and joints, girdles, tails, extra chains).
pub fn profile_summary(skeleton: &Skeleton, profile: &RigProfile) -> String {
    use std::fmt::Write as _;
    let names = |joints: &[u16]| joints.iter().map(|&j| skeleton.name(usize::from(j))).collect::<Vec<_>>().join(" > ");
    let mut out = String::new();
    let m = &profile.measures;
    let _ = writeln!(
        out,
        "template {:?}, {} limbs ({} support), height {:.2} m, hip {:.2} m, body {:.2} m",
        profile.template,
        profile.limbs.len(),
        profile.support_limbs().count(),
        m.height,
        m.hip_height,
        m.body_length
    );
    for g in &profile.girdles {
        let _ = writeln!(out, "  girdle {} carries {:?}", skeleton.name(usize::from(g.joint)), g.limbs);
    }
    if let Some(spine) = &profile.spine {
        let _ = writeln!(out, "  spine {}", names(&spine.joints));
    }
    if let Some(nh) = &profile.neck_head {
        let _ = writeln!(out, "  neck {} | head {}", names(&nh.neck.joints), skeleton.name(usize::from(nh.head)));
    }
    for l in &profile.limbs {
        let _ = writeln!(
            out,
            "  limb {} {:?} {:?} {:?} order {} support {} toe {:?}: {}",
            l.name,
            l.kind,
            l.solver,
            l.side,
            l.order,
            l.support,
            l.foot.as_ref().and_then(|f| f.toe).map(|t| skeleton.name(usize::from(t))),
            names(&l.joints)
        );
    }
    for c in &profile.tails {
        let _ = writeln!(out, "  tail {}: {}", c.name, names(&c.joints));
    }
    for c in &profile.extra {
        let _ = writeln!(out, "  extra {}: {}", c.name, names(&c.joints));
    }
    out
}
