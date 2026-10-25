//! Rig auto-mapping: a skeleton's joints onto the standard humanoid.
//!
//! As Unity's Humanoid auto-mapper and UE5's IK Rig auto-characterize do,
//! names and hierarchy work together, but here names only *classify*
//! joints (hand, foot, finger, helper…, see names.rs) and *anchor* the
//! search, while the hierarchy decides the bone:
//!
//! 1. **Hips**: the unsided joint named hips/pelvis with the largest
//!    subtree (else the common ancestor of the feet). Only joints below it
//!    are body joints, so IK helpers parented to a root (`ik_foot_l`)
//!    never compete.
//! 2. **Neck and head** by name, the **spine** as the path between hips and
//!    neck: one joint is `spine`, two add `chest`, three or more give
//!    `spine`, a middle `chest` and the topmost `upperChest`.
//! 3. **Hands and feet** by name per side (a hand is the one holding the
//!    most finger joints), else from their fingers or toes.
//! 4. **Limbs** walk up from the hand (foot) to the torso. Helpers (twist,
//!    roll, IK) are skipped; names pick upper/lower segments when they can,
//!    position along the chain otherwise. So Mixamo's `LeftLeg` (a lower
//!    leg), 3ds Max's `Calf` and Rigify's `shin` all land right, and an
//!    extra digitigrade joint stays unmapped.
//! 5. **Fingers** grouped by name (thumb…little, or 3ds-Max-style
//!    `Finger<f><segment>`), segments by depth; non-thumb metacarpals and
//!    end sites are skipped.
//! 6. Eyes and jaw below the head; root motion on the hips' nearest
//!    ancestor named root, else the hips; mirror pairs by side-neutral
//!    name; leftover linear chains (tails) become extra chains.

use std::collections::HashSet;

use glam::Vec3;
use sf_anim::Skeleton;

use crate::humanoid::{Finger, HumanoidBone, Limb, Side};
use crate::names::{Class, NameInfo, classify, is_metacarpal};
use crate::rig::{Chain, JointName, RigMap, RigMapError};

/// What auto-mapping found and what it couldn't decide.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AutoMapReport {
    pub missing_required: Vec<HumanoidBone>,
    pub missing_optional: Vec<HumanoidBone>,
    /// Body joints not mapped to a humanoid bone, outside the torso path,
    /// and not helpers, end sites or finger leftovers. They make up the
    /// extra chains.
    pub unmapped: Vec<JointName>,
    /// Guesses a human should review (sides from position, skipped joints).
    pub warnings: Vec<String>,
    /// [`RigMap::validate`] on the result.
    pub errors: Vec<RigMapError>,
}

impl AutoMapReport {
    /// Nothing for a human to fix: every required bone is mapped, the map
    /// validates, and no guess was needed.
    pub fn is_clean(&self) -> bool {
        self.missing_required.is_empty() && self.warnings.is_empty() && self.errors.is_empty()
    }
}

struct Ctx<'a> {
    skeleton: &'a Skeleton,
    info: Vec<NameInfo>,
    side: Vec<Option<Side>>,
    pos: Vec<Vec3>,
    depth: Vec<usize>,
    warnings: Vec<String>,
}

impl Ctx<'_> {
    fn class(&self, j: usize) -> Class {
        self.info[j].class
    }

    fn skip(&self, j: usize) -> bool {
        matches!(self.class(j), Class::End | Class::Helper)
    }

    fn shallowest(&self, it: impl Iterator<Item = usize>) -> Option<usize> {
        it.min_by_key(|&j| (self.depth[j], j))
    }

    fn below(&self, ancestor: usize) -> impl Iterator<Item = usize> + '_ {
        self.skeleton.subtree(ancestor).skip(1)
    }

    fn name(&self, j: usize) -> &str {
        self.skeleton.name(j)
    }
}

fn lca(skeleton: &Skeleton, a: usize, b: usize) -> Option<usize> {
    let mut x = Some(a);
    while let Some(j) = x {
        if skeleton.is_ancestor(j, b) {
            return Some(j);
        }
        x = skeleton.parent(j);
    }
    None
}

/// Maps `skeleton` onto the humanoid. The map always comes back; the
/// report says what is missing or was guessed.
#[allow(clippy::too_many_lines)]
pub fn auto_map(skeleton: &Skeleton) -> (RigMap, AutoMapReport) {
    let n = skeleton.len();
    let info: Vec<NameInfo> = skeleton.names().iter().map(|s| classify(s)).collect();
    let mut side: Vec<Option<Side>> = info.iter().map(|i| i.side).collect();
    let mut depth = vec![0usize; n];
    for j in 0..n {
        if let Some(p) = skeleton.parent(j) {
            depth[j] = depth[p] + 1;
            // Unsided joints below a sided one (fingers named `index_01`) inherit its side.
            if side[j].is_none() {
                side[j] = side[p];
            }
        }
    }
    let pos = skeleton.rest_model().iter().map(|m| Vec3::from(m.translation)).collect();
    let mut cx = Ctx { skeleton, info, side, pos, depth, warnings: Vec::new() };
    let mut bones: Vec<(usize, HumanoidBone)> = Vec::new();

    // 1. Hips.
    // Several candidates: the one with the largest subtree.
    let hips = (0..n)
        .filter(|&j| cx.class(j) == Class::Hips && cx.info[j].side.is_none())
        .max_by_key(|&j| (skeleton.subtree(j).len(), std::cmp::Reverse(j)))
        .or_else(|| {
            let feet: Vec<usize> = (0..n).filter(|&j| cx.class(j) == Class::Foot).collect();
            let (l, r) = (feet.iter().find(|&&j| cx.side[j] == Some(Side::Left))?, feet.iter().find(|&&j| cx.side[j] == Some(Side::Right))?);
            let h = lca(skeleton, *l, *r)?;
            cx.warnings.push(format!("no joint named hips/pelvis: using {:?}, the feet's common ancestor", skeleton.name(h)));
            Some(h)
        });
    let Some(hips) = hips else {
        return finish(skeleton, &cx, bones, Vec::new(), None);
    };
    bones.push((hips, HumanoidBone::Hips));
    let body: Vec<bool> = (0..n).map(|j| j != hips && skeleton.is_ancestor(hips, j)).collect();

    // 2. Neck, head, spine.
    let center = |cx: &Ctx<'_>, j: usize, class: Class| body[j] && cx.class(j) == class && cx.info[j].side.is_none();
    let neck = cx.shallowest((0..n).filter(|&j| center(&cx, j, Class::Neck)));
    let head = cx
        .shallowest((0..n).filter(|&j| center(&cx, j, Class::Head) && neck.is_none_or(|k| skeleton.is_ancestor(k, j))))
        .or_else(|| cx.shallowest((0..n).filter(|&j| center(&cx, j, Class::Head))));
    let mut torso: HashSet<usize> = HashSet::from([hips]);
    let mut spine_path = Vec::new();
    if let Some(top) = neck.or(head) {
        let mut j = skeleton.parent(top);
        while let Some(k) = j {
            if k == hips {
                break;
            }
            spine_path.push(k);
            j = skeleton.parent(k);
        }
        spine_path.reverse();
        torso.insert(top);
    }
    torso.extend(&spine_path);
    let spine: Vec<usize> = spine_path.iter().copied().filter(|&j| !cx.skip(j)).collect();
    match spine.len() {
        0 => {}
        1 => bones.push((spine[0], HumanoidBone::Spine)),
        2 => bones.extend([(spine[0], HumanoidBone::Spine), (spine[1], HumanoidBone::Chest)]),
        k => bones.extend([(spine[0], HumanoidBone::Spine), (spine[(k - 1) / 2], HumanoidBone::Chest), (spine[k - 1], HumanoidBone::UpperChest)]),
    }
    if let Some(k) = neck {
        bones.push((k, HumanoidBone::Neck));
    }
    if let Some(h) = head {
        bones.push((h, HumanoidBone::Head));
        if let Some(k) = neck {
            let mut j = skeleton.parent(h);
            while let Some(x) = j.filter(|&x| x != k && skeleton.is_ancestor(k, x)) {
                torso.insert(x);
                j = skeleton.parent(x);
            }
        }
    }

    // 3. Sides from position when names carry none (assumes facing +Z: left is +X).
    let anchors: Vec<usize> = (0..n).filter(|&j| body[j] && matches!(cx.class(j), Class::Hand | Class::Foot)).collect();
    if !anchors.is_empty() && anchors.iter().all(|&j| cx.side[j].is_none()) {
        cx.warnings.push("joint names carry no side: sides guessed from position (left = +X)".into());
        let hx = cx.pos[hips].x;
        for (j, side) in cx.side.iter_mut().enumerate() {
            if body[j] && side.is_none() && !torso.contains(&j) {
                *side = Some(if cx.pos[j].x >= hx { Side::Left } else { Side::Right });
            }
        }
    }

    // 4. Limbs.
    let mut chained: HashSet<usize> = torso.clone();
    for s in [Side::Left, Side::Right] {
        let sided = |cx: &Ctx<'_>, j: usize, class: Class| body[j] && cx.class(j) == class && cx.side[j] == Some(s);
        let fingers_below = |cx: &Ctx<'_>, j: usize| cx.below(j).filter(|&k| matches!(cx.class(k), Class::Finger(_))).count();
        let hand =
            (0..n).filter(|&j| sided(&cx, j, Class::Hand)).max_by_key(|&j| (fingers_below(&cx, j), std::cmp::Reverse(cx.depth[j]))).or_else(|| {
                let roots: Vec<usize> = (0..n)
                    .filter(|&j| body[j] && matches!(cx.class(j), Class::Finger(_)) && cx.side[j] == Some(s))
                    .filter(|&j| skeleton.parent(j).is_none_or(|p| !matches!(cx.class(p), Class::Finger(_))))
                    .collect();
                let first = *roots.first()?;
                roots.iter().try_fold(first, |a, &b| lca(skeleton, a, b)).and_then(|h| if roots.len() == 1 { skeleton.parent(h) } else { Some(h) })
            });
        let foot = cx
            .shallowest((0..n).filter(|&j| sided(&cx, j, Class::Foot)))
            .or_else(|| cx.shallowest((0..n).filter(|&j| sided(&cx, j, Class::Toe))).and_then(|t| skeleton.parent(t)));
        let limb = |l: Limb| HumanoidBone::limb(s, l).expect("sided");
        if let Some(h) = hand {
            bones.push((h, limb(Limb::Hand)));
            let path = limb_path(&cx, h, &torso, &body);
            assign_arm(&mut cx, &path, s, &mut bones);
            map_fingers(&mut cx, h, s, &mut bones);
        }
        if let Some(f) = foot {
            bones.push((f, limb(Limb::Foot)));
            let path = limb_path(&cx, f, &torso, &body);
            assign_leg(&mut cx, &path, s, &mut bones);
            let children: Vec<usize> = cx.below(f).filter(|&k| skeleton.parent(k) == Some(f) && !cx.skip(k)).collect();
            let toes = children.iter().copied().find(|&k| cx.class(k) == Class::Toe).or(if children.len() == 1 { Some(children[0]) } else { None });
            if let Some(t) = toes {
                bones.push((t, limb(Limb::Toes)));
            }
        }
        if let Some(h) = head
            && let Some(e) = cx.shallowest(cx.below(h).filter(|&j| cx.class(j) == Class::Eye && cx.side[j] == Some(s)))
        {
            bones.push((e, limb(Limb::Eye)));
        }
    }
    if let Some(h) = head
        && let Some(jaw) = cx.shallowest(cx.below(h).filter(|&j| cx.class(j) == Class::Jaw))
    {
        bones.push((jaw, HumanoidBone::Jaw));
    }

    // 6. Root motion, extra chains.
    let mut root = hips;
    let mut j = skeleton.parent(hips);
    while let Some(k) = j {
        if cx.class(k) == Class::Root {
            root = k;
            break;
        }
        j = skeleton.parent(k);
    }
    for (j, _) in &bones {
        chained.insert(*j);
    }
    // Unmapped body joints (they also form the extra chains).
    let unmapped: Vec<usize> =
        (0..n).filter(|&j| body[j] && !chained.contains(&j) && !cx.skip(j) && !matches!(cx.class(j), Class::Finger(_))).collect();
    let extra = extra_chains(&cx, &body, &chained);
    let (mut map, mut report) = finish_parts(skeleton, &cx, bones, extra, Some(root));
    report.unmapped = unmapped.iter().map(|&j| cx.name(j).to_owned()).collect();
    map.mirror = mirror_pairs(&cx);
    report.errors = map.validate(skeleton);
    (map, report)
}

fn finish(skeleton: &Skeleton, cx: &Ctx<'_>, bones: Vec<(usize, HumanoidBone)>, extra: Vec<Chain>, root: Option<usize>) -> (RigMap, AutoMapReport) {
    let (map, mut report) = finish_parts(skeleton, cx, bones, extra, root);
    report.errors = map.validate(skeleton);
    (map, report)
}

fn finish_parts(
    skeleton: &Skeleton,
    cx: &Ctx<'_>,
    mut bones: Vec<(usize, HumanoidBone)>,
    extra: Vec<Chain>,
    root: Option<usize>,
) -> (RigMap, AutoMapReport) {
    bones.sort_by_key(|(_, b)| *b);
    let set = crate::humanoid::BoneSet::of(&bones.iter().map(|(_, b)| *b).collect::<Vec<_>>());
    let missing = |required: bool| HumanoidBone::ALL.into_iter().filter(|b| b.required() == required && !set.contains(*b)).collect();
    let report = AutoMapReport {
        missing_required: missing(true),
        missing_optional: missing(false),
        warnings: cx.warnings.clone(),
        ..AutoMapReport::default()
    };
    let map = RigMap {
        bones: bones.iter().map(|(j, b)| (skeleton.name(*j).to_owned(), *b)).collect(),
        root_motion: root.map(|r| skeleton.name(r).to_owned()).unwrap_or_default(),
        extra_chains: extra,
        ..RigMap::default()
    };
    (map, report)
}

/// The joints between an end joint (hand, foot) and the torso, top down,
/// the end joint excluded, helpers included.
fn limb_path(cx: &Ctx<'_>, end: usize, torso: &HashSet<usize>, body: &[bool]) -> Vec<usize> {
    let mut path = Vec::new();
    let mut j = cx.skeleton.parent(end);
    while let Some(k) = j {
        if !body[k] || torso.contains(&k) {
            break;
        }
        path.push(k);
        j = cx.skeleton.parent(k);
    }
    path.reverse();
    path
}

fn assign_arm(cx: &mut Ctx<'_>, path: &[usize], s: Side, bones: &mut Vec<(usize, HumanoidBone)>) {
    let p: Vec<usize> = path.iter().copied().filter(|&j| !cx.skip(j)).collect();
    let limb = |l: Limb| HumanoidBone::limb(s, l).expect("sided");
    let Some(lower) = p.iter().rposition(|&j| cx.class(j) == Class::LowerArm).or_else(|| p.len().checked_sub(1)) else {
        cx.warnings.push(format!("{s:?} arm: no joints between the hand and the torso"));
        return;
    };
    let Some(upper) = p[..lower].iter().rposition(|&j| cx.class(j) == Class::UpperArm).or_else(|| lower.checked_sub(1)) else {
        cx.warnings.push(format!("{s:?} arm: only one joint between the hand and the torso"));
        return;
    };
    let shoulder = p[..upper].iter().rposition(|&j| cx.class(j) == Class::Shoulder).or_else(|| upper.checked_sub(1));
    bones.push((p[lower], limb(Limb::LowerArm)));
    bones.push((p[upper], limb(Limb::UpperArm)));
    if let Some(sh) = shoulder {
        bones.push((p[sh], limb(Limb::Shoulder)));
    }
    let used = [Some(lower), Some(upper), shoulder];
    for (i, &j) in p.iter().enumerate() {
        if !used.contains(&Some(i)) && i > shoulder.unwrap_or(0) {
            let name = cx.name(j).to_owned();
            cx.warnings.push(format!("{s:?} arm: extra joint {name:?} left unmapped"));
        }
    }
}

fn assign_leg(cx: &mut Ctx<'_>, path: &[usize], s: Side, bones: &mut Vec<(usize, HumanoidBone)>) {
    let p: Vec<usize> = path.iter().copied().filter(|&j| !cx.skip(j)).collect();
    let limb = |l: Limb| HumanoidBone::limb(s, l).expect("sided");
    let Some(upper) = p.iter().position(|&j| cx.class(j) == Class::UpperLeg).or(if p.is_empty() { None } else { Some(0) }) else {
        cx.warnings.push(format!("{s:?} leg: no joints between the foot and the hips"));
        return;
    };
    let rest = &p[upper + 1..];
    let Some(lower) = rest.iter().position(|&j| cx.class(j) == Class::LowerLeg).or(if rest.is_empty() { None } else { Some(0) }) else {
        cx.warnings.push(format!("{s:?} leg: only one joint between the foot and the hips"));
        return;
    };
    // A joint between the lower leg and the foot (a digitigrade leg) is
    // expected on some rigs: it stays unmapped and becomes an extra chain.
    bones.push((p[upper], limb(Limb::UpperLeg)));
    bones.push((rest[lower], limb(Limb::LowerLeg)));
}

fn map_fingers(cx: &mut Ctx<'_>, hand: usize, s: Side, bones: &mut Vec<(usize, HumanoidBone)>) {
    let skeleton = cx.skeleton;
    let joints: Vec<usize> = cx.below(hand).filter(|&j| matches!(cx.class(j), Class::Finger(_)) && cx.side[j] == Some(s)).collect();
    if joints.iter().any(|&j| cx.class(j) == Class::Finger(None)) {
        cx.warnings.push(format!("{s:?} hand: finger joints without a finger name left unmapped"));
    }
    for f in Finger::ALL {
        let group: Vec<usize> = joints.iter().copied().filter(|&j| cx.class(j) == Class::Finger(Some(f))).collect();
        let Some(&first) = group.iter().min_by_key(|&&j| (cx.depth[j], j)) else {
            continue;
        };
        // Follow one chain from the shallowest joint.
        let mut chain = vec![first];
        let mut cur = first;
        while let Some(next) = group.iter().copied().filter(|&k| skeleton.parent(k) == Some(cur)).min() {
            chain.push(next);
            cur = next;
        }
        if f != Finger::Thumb {
            chain.retain(|&j| !is_metacarpal(&cx.info[j]));
        }
        let segments: &[usize] = match (f, chain.len()) {
            (_, 0) => &[],
            (Finger::Thumb, 1) => &[1],
            (Finger::Thumb, 2) => &[1, 2],
            (_, 1) => &[0],
            (_, 2) => &[0, 1],
            _ => &[0, 1, 2],
        };
        for (&j, &k) in chain.iter().zip(segments) {
            if let Some(b) = HumanoidBone::finger(s, f, k) {
                bones.push((j, b));
            }
        }
    }
}

fn mirror_pairs(cx: &Ctx<'_>) -> Vec<(JointName, JointName)> {
    let n = cx.skeleton.len();
    let mut out = Vec::new();
    for l in (0..n).filter(|&j| cx.info[j].side == Some(Side::Left)) {
        if let Some(r) = (0..n).find(|&k| cx.info[k].side == Some(Side::Right) && cx.info[k].neutral == cx.info[l].neutral) {
            out.push((cx.name(l).to_owned(), cx.name(r).to_owned()));
        }
    }
    out
}

/// Leftover linear chains of non-helper body joints (tails, ears, skirt
/// joints), named by their side-neutral words plus the side.
fn extra_chains(cx: &Ctx<'_>, body: &[bool], chained: &HashSet<usize>) -> Vec<Chain> {
    let skeleton = cx.skeleton;
    let n = skeleton.len();
    let candidate = |j: usize| body[j] && !chained.contains(&j) && !cx.skip(j) && !matches!(cx.class(j), Class::Finger(_));
    let mut out: Vec<Chain> = Vec::new();
    for start in (0..n).filter(|&j| candidate(j) && skeleton.parent(j).is_none_or(|p| !candidate(p))) {
        let mut joints = vec![start];
        let mut cur = start;
        loop {
            let next: Vec<usize> = cx.below(cur).filter(|&k| skeleton.parent(k) == Some(cur) && candidate(k)).collect();
            if next.len() != 1 {
                break;
            }
            cur = next[0];
            joints.push(cur);
        }
        let info = &cx.info[start];
        let base = if info.core.is_empty() { info.neutral.replace('|', "_") } else { info.core.clone() };
        let mut name = match cx.side[start] {
            Some(Side::Left) => format!("{base}_left"),
            Some(Side::Right) => format!("{base}_right"),
            _ => base,
        };
        if out.iter().any(|c| c.name == name) {
            name = format!("{name}#{start}");
        }
        out.push(Chain { name, joints: joints.iter().map(|&j| skeleton.name(j).to_owned()).collect() });
    }
    out
}
