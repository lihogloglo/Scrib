//! Arms tucked against the chest on retargeted library clips (owner
//! 2026-11-12, `clip-preview --browse generated/loco1`): diagnosis only.
//!
//! Per take (Kimodo SOMA-77 loco1 BVH walk/run/start, the `.free` walk, UAL1
//! `Walk_Loop`) on its own rig and retargeted onto the Morrowind humanoid
//! ([`sf_anim::Retargeter`] with `HumanoidTarget::rig`, the bake's and
//! `clip-preview`'s path), and Morrowind's own `WalkForward`/`RunForward`
//! played natively as the reference, per arm over the middle half of the
//! clip: upper-arm abduction in the frontal plane (outward +), forward/back
//! swing range, elbow flexion, elbow and wrist distance outward from the
//! chest joint's mid-sagittal plane (in half shoulder widths: 1.0 = under the
//! shoulder joint) and to the chest joint (in shoulder widths). On the
//! target, the elbow and wrist clearance to the skinned Morrowind chest mesh
//! (posed by linear blend skinning, signed by the vertex normal) next to the
//! arm meshes' radius, and the same with the clavicle held at its bind
//! rotation (upper arm kept in model space): the clavicle's share. Then the
//! rig proportions and the arm chain mapping and alignment, and the
//! source clavicle's mean raise/forward against its retarget pose (with the
//! source's `pose_offsets`: the residual). `measure_generated_clavicle_offset`
//! averages it over every loco1 take per gait against the auto-aligned pose:
//! the source's shoulder `pose_offsets` in `sources.ron`. Run with
//! `-- --ignored --nocapture`; skips without a Morrowind install or files.
#![allow(clippy::similar_names, clippy::many_single_char_names, clippy::items_after_statements, clippy::cast_precision_loss, clippy::too_many_lines)]

use std::path::{Path, PathBuf};

use glam::{Affine3A, Quat, Vec3};
use mw_bake::anim::{HumanoidTarget, MwAnimRules, library_rig};
use mw_bake::sources::AnimSources;
use mw_graphs::HUMANOID_SETS;
use mw_install::MorrowindInstall;
use sf_anim::{Clip, ClipRules, CookSettings, LocalPose, Looping, RetargetRig, RigHints, RootMotionMode, RootMotionRules, Skeleton, Trs};
use sf_character::{HumanoidBone as B, RigMap};
use sf_vfs::{DirEntry, Vfs, VfsPath};

const RATE: f32 = 30.0;

struct Take {
    label: String,
    source: &'static str,
    path: PathBuf,
    clip: Option<String>,
}

fn rules() -> ClipRules {
    ClipRules { looping: Looping::Once, root_motion: RootMotionRules { mode: RootMotionMode::Drive, ..RootMotionRules::default() }, ..ClipRules::default() }
}

fn install_vfs() -> Option<Vfs> {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return None;
    };
    Some(mw_vfs::mount_install(&install, Vfs::builder().mount(mw_graphs::builtin_source())).unwrap().vfs)
}

fn takes() -> Vec<Take> {
    let mut out = Vec::new();
    let loco = AnimSources::local().dir.join("generated").join("loco1");
    for (dir, file) in [
        ("loco.walk.cycle.1", "loco.walk.cycle.1.act.ground0.00.bvh"),
        ("loco.walk.cycle.free", "loco.walk.cycle.free.act.ground0.00.bvh"),
        ("loco.run.cycle.3", "loco.run.cycle.3.act.ground0.04.bvh"),
        ("loco.walk.start.f.1", "loco.walk.start.f.1.act.ground0.00.bvh"),
    ] {
        let path = loco.join(dir).join("act").join(file);
        if path.exists() {
            out.push(Take { label: path.file_stem().unwrap().to_string_lossy().into_owned(), source: "generated", path, clip: None });
        } else {
            eprintln!("{} missing", path.display());
        }
    }
    let ual = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets/animation/quaternius/UAL1_RM.glb");
    out.push(Take { label: "UAL1:Walk_Loop".into(), source: "quaternius_ual1", path: ual, clip: Some("Walk_Loop".into()) });
    out
}

fn model_aff(skeleton: &Skeleton, pose: &[Trs]) -> Vec<Affine3A> {
    let mut m: Vec<Affine3A> = Vec::with_capacity(pose.len());
    for (j, t) in pose.iter().enumerate() {
        let parent = skeleton.parent(j).map_or(Affine3A::IDENTITY, |p| m[p]);
        m.push(parent * t.to_affine());
    }
    m
}

fn model_rots(skeleton: &Skeleton, pose: &[Trs]) -> Vec<Quat> {
    let mut out: Vec<Quat> = Vec::with_capacity(pose.len());
    for (j, t) in pose.iter().enumerate() {
        let parent = skeleton.parent(j).map_or(Quat::IDENTITY, |p| out[p]);
        out.push((parent * t.rotation).normalize());
    }
    out
}

fn sample(skeleton: &Skeleton, clip: &Clip, t: f32) -> Vec<Trs> {
    let mut pose = LocalPose::rest(skeleton);
    clip.sample(t, &mut pose, None);
    pose.to_vec()
}

fn joint(skeleton: &Skeleton, map: &RigMap, b: B) -> Option<usize> {
    map.joint(b).and_then(|j| skeleton.find(j))
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn frame_times(duration: f32) -> Vec<f32> {
    let n = (duration * RATE).ceil().max(1.0) as usize;
    (0..=n).map(|i| (i as f32 / RATE).min(duration)).collect()
}

fn retarget_pose(skeleton: &Skeleton, rig: &RetargetRig) -> Vec<Trs> {
    let mut pose = skeleton.rest().to_vec();
    for (name, q) in &rig.retarget_pose {
        pose[skeleton.find(name).unwrap()].rotation = q.normalize();
    }
    pose
}

fn short(n: &str) -> String {
    n.trim_start_matches("Bip01 ").replace(' ', "")
}

#[derive(Clone, Copy)]
struct Arm {
    clav: usize,
    upper: usize,
    lower: usize,
    hand: usize,
    /// +1 left, -1 right (outward = side * left).
    side: f32,
}

/// Bind position, normal, `(target joint, inverse bind, weight)` influences.
type SkinVert = (Vec3, Vec3, Vec<(usize, Affine3A, f32)>);

/// The skinned Morrowind chest mesh: per vertex bind position, normal and
/// `(target joint, inverse bind, weight)` influences.
struct Torso {
    verts: Vec<SkinVert>,
}

impl Torso {
    /// Vertex positions and normals posed by linear blend skinning.
    fn posed(&self, model: &[Affine3A]) -> Vec<(Vec3, Vec3)> {
        self.verts
            .iter()
            .map(|(p, n, inf)| {
                let mut pos = Vec3::ZERO;
                let mut nor = Vec3::ZERO;
                let mut w = 0.0;
                for (j, ib, wt) in inf {
                    let m = model[*j] * *ib;
                    pos += m.transform_point3(*p) * *wt;
                    nor += m.transform_vector3(*n) * *wt;
                    w += wt;
                }
                (pos / w.max(1e-6), nor.normalize_or_zero())
            })
            .collect()
    }

    /// Signed distance from `p` to the nearest vertex (outside +).
    fn clearance(posed: &[(Vec3, Vec3)], p: Vec3) -> f32 {
        let (v, n) = posed.iter().min_by(|a, b| a.0.distance_squared(p).total_cmp(&b.0.distance_squared(p))).unwrap();
        let d = v.distance(p);
        if (p - *v).dot(*n) < 0.0 { -d } else { d }
    }
}

struct Body {
    arms: [Arm; 2],
    chest: usize,
    thighs: (usize, usize),
    hips: usize,
    /// Shoulder width (upper-arm joint to upper-arm joint) at rest.
    sw: f32,
}

impl Body {
    fn new(skeleton: &Skeleton, map: &RigMap) -> Self {
        let need = |b: B| joint(skeleton, map, b).unwrap_or_else(|| panic!("{b:?} unmapped"));
        let arm = |c, u, l, h, side| Arm { clav: need(c), upper: need(u), lower: need(l), hand: need(h), side };
        let arms = [
            arm(B::LeftShoulder, B::LeftUpperArm, B::LeftLowerArm, B::LeftHand, 1.0),
            arm(B::RightShoulder, B::RightUpperArm, B::RightLowerArm, B::RightHand, -1.0),
        ];
        let chest = joint(skeleton, map, B::UpperChest).or_else(|| joint(skeleton, map, B::Chest)).unwrap_or_else(|| need(B::Spine));
        let rest = model_aff(skeleton, skeleton.rest());
        let sw = Vec3::from(rest[arms[0].upper].translation).distance(Vec3::from(rest[arms[1].upper].translation));
        Self { arms, chest, thighs: (need(B::LeftUpperLeg), need(B::RightUpperLeg)), hips: need(B::Hips), sw }
    }

    fn frame(&self, pos: &[Vec3]) -> (Vec3, Vec3) {
        let mut left = pos[self.thighs.0] - pos[self.thighs.1];
        left.y = 0.0;
        let left = left.normalize_or(Vec3::X);
        (left, left.cross(Vec3::Y))
    }

    fn measure(&self, model: &[Affine3A], torso: Option<&[(Vec3, Vec3)]>) -> [ArmM; 2] {
        let pos: Vec<Vec3> = model.iter().map(|a| Vec3::from(a.translation)).collect();
        let (left, fwd) = self.frame(&pos);
        let c = pos[self.chest];
        let hsw = self.sw * 0.5;
        self.arms.map(|a| {
            let out = left * a.side;
            let (s, e, w) = (pos[a.upper], pos[a.lower], pos[a.hand]);
            let u = (e - s).normalize_or_zero();
            let f = (w - e).normalize_or_zero();
            ArmM {
                abd: u.dot(out).atan2(-u.y).to_degrees(),
                swing: u.dot(fwd).atan2(-u.y).to_degrees(),
                flex: u.dot(f).clamp(-1.0, 1.0).acos().to_degrees(),
                sh_lat: (s - c).dot(out) / hsw,
                el_lat: (e - c).dot(out) / hsw,
                wr_lat: (w - c).dot(out) / hsw,
                wr_fwd: (w - c).dot(fwd) / hsw,
                wr_chest: w.distance(c) / self.sw,
                el_clear: torso.map_or(f32::NAN, |t| Torso::clearance(t, e)),
                wr_clear: torso.map_or(f32::NAN, |t| Torso::clearance(t, w)),
                fa_clear: torso.map_or(f32::NAN, |t| Torso::clearance(t, e.lerp(w, 0.5))),
            }
        })
    }
}

#[derive(Clone, Copy, Default)]
struct ArmM {
    abd: f32,
    swing: f32,
    flex: f32,
    sh_lat: f32,
    el_lat: f32,
    wr_lat: f32,
    wr_fwd: f32,
    wr_chest: f32,
    el_clear: f32,
    wr_clear: f32,
    fa_clear: f32,
}

/// Prints mean (min..max for swing; min for clearances) over the mid half.
fn report(tag: &str, rows: &[[ArmM; 2]]) {
    let n = rows.len();
    let mid = &rows[n / 4..(3 * n / 4).max(n / 4 + 1)];
    let k = mid.len() as f32;
    for (i, side) in ["L", "R"].iter().enumerate() {
        let get = |f: &dyn Fn(&ArmM) -> f32| mid.iter().map(|r| f(&r[i])).collect::<Vec<f32>>();
        let mean = |v: &[f32]| v.iter().sum::<f32>() / k;
        let min = |v: &[f32]| v.iter().copied().fold(f32::INFINITY, f32::min);
        let max = |v: &[f32]| v.iter().copied().fold(f32::NEG_INFINITY, f32::max);
        let sw = get(&|m| m.swing);
        let ec = get(&|m| m.el_clear);
        let wc = get(&|m| m.wr_clear);
        let fc = get(&|m| m.fa_clear);
        let clear = if ec[0].is_nan() {
            String::new()
        } else {
            format!(
                " | clear cm el {:>5.1} (min {:>5.1}) fa {:>5.1} (min {:>5.1}) wr {:>5.1} (min {:>5.1})",
                mean(&ec) * 100.0,
                min(&ec) * 100.0,
                mean(&fc) * 100.0,
                min(&fc) * 100.0,
                mean(&wc) * 100.0,
                min(&wc) * 100.0
            )
        };
        println!(
            "    {tag:<14}{side} abd {:>5.1} (min {:>5.1}) swing {:>6.1}..{:>5.1} flex {:>5.1} | lat/hsw sh {:>4.2} el {:>5.2} wr {:>5.2} (min {:>5.2}) | wr fwd/hsw {:>5.2} wr-chest/sw {:>4.2}{clear}",
            mean(&get(&|m| m.abd)),
            min(&get(&|m| m.abd)),
            min(&sw),
            max(&sw),
            mean(&get(&|m| m.flex)),
            mean(&get(&|m| m.sh_lat)),
            mean(&get(&|m| m.el_lat)),
            mean(&get(&|m| m.wr_lat)),
            min(&get(&|m| m.wr_lat)),
            mean(&get(&|m| m.wr_fwd)),
            mean(&get(&|m| m.wr_chest)),
        );
    }
}

/// The pose with each clavicle at its bind (rest) local rotation and the
/// upper arm kept at its model rotation.
fn clav_at_bind(skeleton: &Skeleton, body: &Body, pose: &[Trs]) -> Vec<Trs> {
    let mut out = pose.to_vec();
    let rot = model_rots(skeleton, pose);
    for a in body.arms {
        out[a.clav].rotation = skeleton.rest()[a.clav].rotation;
        let parent = skeleton.parent(a.clav).map_or(Quat::IDENTITY, |p| rot[p]);
        let clav_model = parent * out[a.clav].rotation;
        // Upper arm's parent is the clavicle in both rigs (checked below).
        debug_assert_eq!(skeleton.parent(a.upper), Some(a.clav));
        out[a.upper].rotation = (clav_model.inverse() * rot[a.upper]).normalize();
    }
    out
}

fn proportions(label: &str, skeleton: &Skeleton, body: &Body) {
    let m = model_aff(skeleton, skeleton.rest());
    let p = |j: usize| Vec3::from(m[j].translation);
    let [l, r] = body.arms;
    let trunk = p(body.hips).distance((p(l.upper) + p(r.upper)) * 0.5);
    let hipw = p(body.thighs.0).distance(p(body.thighs.1));
    let ua = p(l.upper).distance(p(l.lower));
    let fa = p(l.lower).distance(p(l.hand));
    let cl = p(l.clav).distance(p(l.upper));
    println!(
        "  {label:<8} trunk(hips->shoulders) {trunk:.3} | clav-clav {:.3} sh-sh {:.3} hip-w {hipw:.3} | clav len {cl:.3} upper {ua:.3} fore {fa:.3} | per trunk: sh-sh {:.3} upper {:.3} fore {:.3} hip-w {:.3} | sh-sh/arm {:.3}",
        p(l.clav).distance(p(r.clav)),
        body.sw,
        body.sw / trunk,
        ua / trunk,
        fa / trunk,
        hipw / trunk,
        body.sw / (ua + fa),
    );
}

fn arm_map(label: &str, skeleton: &Skeleton, map: &RigMap, rig: &RetargetRig, body: &Body) {
    let a = body.arms[0];
    let names: Vec<String> = [a.clav, a.upper, a.lower, a.hand].iter().map(|&j| skeleton.name(j).to_owned()).collect();
    let mut kids = Vec::new();
    for j in 0..skeleton.len() {
        if skeleton.parent(j).is_some_and(|p| [a.clav, a.upper, a.lower].contains(&p)) {
            kids.push(skeleton.name(j).to_owned());
        }
    }
    println!("  {label} left arm: {names:?}; children of clav/upper/lower: {kids:?}");
    for c in rig.chains.iter().filter(|c| c.name.contains("arm") || c.name.contains("shoulder") || c.name.contains("hand")) {
        if c.name.starts_with("left") || c.name.starts_with('l') {
            println!("    chain {:<16} {:?}", c.name, c.joints);
        }
    }
    let _ = map;
    // Rest vs retarget pose: upper/lower arm direction (frontal elevation
    // below horizontal, forward), clavicle direction.
    let rest = model_aff(skeleton, skeleton.rest());
    let rp = model_aff(skeleton, &retarget_pose(skeleton, rig));
    let dir = |m: &[Affine3A], j: usize, k: usize| (Vec3::from(m[k].translation) - Vec3::from(m[j].translation)).normalize_or_zero();
    let desc = |v: Vec3| format!("down {:>5.1} fwd {:>5.1}", (-v.y).asin().to_degrees(), v.z.asin().to_degrees());
    for (n, m) in [("rest", &rest), ("retarget", &rp)] {
        println!(
            "    {n:<9} clav {} | upper {} | fore {} | hand twist-free",
            desc(dir(m, a.clav, a.upper)),
            desc(dir(m, a.upper, a.lower)),
            desc(dir(m, a.lower, a.hand))
        );
    }
}

fn torso_mesh(vfs: &Vfs, skeleton: &Skeleton) -> Option<(String, Torso, f32, f32)> {
    let files: Vec<String> = vfs
        .read_dir("meshes/b")
        .into_iter()
        .filter_map(|e| if let DirEntry::File(p) = e { Some(p.as_str().to_owned()) } else { None })
        .collect();
    // Vanilla bodies: one `B_N_<race>_<sex>_Skins.nif` per race, one shape per part ("Tri Chest", ...).
    let skins = ["imperial", "nord", "dark elf"]
        .iter()
        .find_map(|race| files.iter().find(|f| f.contains(&format!("b_n_{race}_m_skins"))).cloned())
        .or_else(|| files.iter().find(|f| f.contains("_m_skins")).cloned())?;
    let find = |n: &str| skeleton.find(n).or_else(|| (0..skeleton.len()).find(|&j| skeleton.name(j).eq_ignore_ascii_case(n)));
    let bind = model_aff(skeleton, skeleton.rest());
    let model = mw_nif::load(&vfs.read(&VfsPath::new(&skins).ok()?).ok()?).ok()?;
    let shape_name = |m: usize| model.nodes.iter().find(|n| n.mesh == Some(m)).map_or(String::new(), |n| n.name.to_ascii_lowercase());
    let names: Vec<String> = (0..model.meshes.len()).map(shape_name).collect();
    println!("  body {skins}: shapes {names:?}");
    let mut worst: f32 = 0.0;
    let mut world: Vec<Affine3A> = Vec::with_capacity(model.nodes.len());
    for n in &model.nodes {
        let parent = n.parent.map_or(Affine3A::IDENTITY, |p| world[p]);
        world.push(parent * aff(n.transform.to_affine()));
    }
    let mut load = |part: &str| -> Vec<SkinVert> {
        let mut verts = Vec::new();
        for (mi, mesh) in model.meshes.iter().enumerate() {
            if !names[mi].contains(part) || (part == "upper arm" && !names[mi].contains("left")) || (part == "forearm" && !names[mi].contains("left")) {
                continue;
            }
            let Some(skin) = &mesh.skin else { continue };
            let bones: Vec<Option<usize>> = skin.bones.iter().map(|b| find(&model.nodes[b.node].name)).collect();
            for (b, j) in skin.bones.iter().zip(&bones) {
                if let Some(j) = j {
                    worst = worst.max(Vec3::from(world[b.node].translation).distance(Vec3::from(bind[*j].translation)));
                }
            }
            for (i, p) in skin.positions.iter().enumerate() {
                let n = skin.normals.get(i).map_or(Vec3::ZERO, |n| Vec3::from(*n));
                let inf: Vec<(usize, Affine3A, f32)> =
                    skin.influences[i].iter().filter_map(|&(b, w)| bones[b as usize].map(|j| (j, aff(skin.bones[b as usize].inverse_bind), w))).collect();
                if !inf.is_empty() {
                    verts.push((Vec3::from(*p), n, inf));
                }
            }
        }
        verts
    };
    let chest = load("chest");
    let upper = load("upper arm");
    let fore = load("forearm");
    // Arm radius: median vertex distance from the bone line at bind.
    let radius = |verts: Vec<SkinVert>, a: &str, b: &str| -> f32 {
        let (Some(ja), Some(jb)) = (find(a), find(b)) else { return f32::NAN };
        let posed = Torso { verts }.posed(&bind);
        let (pa, pb) = (Vec3::from(bind[ja].translation), Vec3::from(bind[jb].translation));
        let axis = (pb - pa).normalize_or_zero();
        let mut d: Vec<f32> = posed
            .iter()
            .filter_map(|(p, _)| {
                let t = (*p - pa).dot(axis) / pa.distance(pb);
                (0.2..0.8).contains(&t).then(|| (*p - pa).reject_from(axis).length())
            })
            .collect();
        d.sort_by(f32::total_cmp);
        d.get(d.len() / 2).copied().unwrap_or(f32::NAN)
    };
    let ua = if upper.is_empty() { part_radius(vfs, &skins.replace("_skins", "_upper arm")) } else { radius(upper, "Bip01 L UpperArm", "Bip01 L Forearm") };
    let fa = if fore.is_empty() { part_radius(vfs, &skins.replace("_skins", "_forearm")) } else { radius(fore, "Bip01 L Forearm", "Bip01 L Hand") };
    if chest.is_empty() {
        return None;
    }
    // Torso half-width / depth at bind around the chest joint, for scale.
    let posed = Torso { verts: chest.clone() }.posed(&bind);
    let (lo, hi) = posed.iter().fold((Vec3::splat(f32::INFINITY), Vec3::splat(f32::NEG_INFINITY)), |(a, b), (p, _)| (a.min(*p), b.max(*p)));
    println!("  chest mesh: {} verts, bind bounds {lo:.3} .. {hi:.3}; nif bone vs base_anim bind max offset {worst:.4}", chest.len());
    Some((skins, Torso { verts: chest }, ua, fa))
}

fn measure_take(take: &Take, target: &HumanoidTarget, tbody: &Body, torso: Option<&Torso>, first: &mut bool) -> Result<(), String> {
    let manifest = mw_graphs::sources::SourcesManifest::builtin();
    let library = &manifest.sources[take.source];
    let name = take.path.file_name().unwrap().to_string_lossy().into_owned();
    let virtual_path = AnimSources::virtual_path(take.source, &name);
    let importers = mw_bake::anim::anim_importers();
    let bytes = std::fs::read(&take.path).map_err(|e| e.to_string())?;
    let imported = importers.for_path(&virtual_path).unwrap().import(&virtual_path, &bytes).map_err(|e| e.to_string())?;
    let joints_in = imported.skeleton.as_ref().ok_or("no skeleton")?;
    let mut skeleton_rules = MwAnimRules::skeleton_rules();
    skeleton_rules.root_motion_joint.clone_from(&imported.root_motion);
    let (skeleton, _) = Skeleton::with_rules(joints_in, &skeleton_rules).map_err(|e| e.to_string())?;
    let (map, source_rig) = library_rig(&skeleton, take.source).ok_or("no humanoid rig")?;
    let hints = RigHints { profile: map.profile(&skeleton).ok(), ..RigHints::default() };
    let profile = sf_anim::detect_profile(&skeleton, &hints).map_err(|e| e.to_string())?;
    let mut cook_rules = rules();
    if library.root_from_pelvis {
        cook_rules.root_motion.extract = false;
        cook_rules.root_motion.anchor_end = false;
    }
    let src = match &take.clip {
        Some(c) => imported.clips.iter().find(|x| &x.name == c),
        None => imported.clips.first(),
    }
    .ok_or("no clip")?;
    let cooked = sf_anim::cook_clip_for(&skeleton, Some(&profile), src, &cook_rules, &CookSettings::default()).map_err(|e| e.to_string())?;
    let r = sf_anim::Retargeter::new(&skeleton, &source_rig, &target.skeleton, &target.rig).map_err(|e| e.to_string())?;
    let sbody = Body::new(&skeleton, &map);

    println!("\n== {} ({:.2} s)", take.label, cooked.duration);
    if take.label.starts_with("UAL1:") || *first {
        *first = false;
        proportions("source", &skeleton, &sbody);
        arm_map("source", &skeleton, &map, &source_rig, &sbody);
        for p in r.pairs() {
            if tbody.arms.iter().any(|a| [a.clav, a.upper, a.lower, a.hand].contains(&p.target)) {
                println!(
                    "    pair {:<12} <- {} / {} alpha {:.2}",
                    short(target.skeleton.name(p.target)),
                    skeleton.name(p.source.0),
                    skeleton.name(p.source.1),
                    p.alpha
                );
            }
        }
        // Source rest -> target arms.
        let mut out = vec![Trs::IDENTITY; target.skeleton.len()];
        r.pose(skeleton.rest(), &[0.0, 0.0], &mut out);
        report("src rest->tgt", &[tbody.measure(&model_aff(&target.skeleton, &out), None)]);
    }
    let times = frame_times(cooked.duration);
    let (mut s_rows, mut t_rows, mut c_rows) = (Vec::new(), Vec::new(), Vec::new());
    let mut out = vec![Trs::IDENTITY; target.skeleton.len()];
    let mut dir_err = 0.0f32;
    for &t in &times {
        let sp = sample(&skeleton, &cooked, t);
        let sm = model_aff(&skeleton, &sp);
        s_rows.push(sbody.measure(&sm, None));
        r.pose(&sp, &[1.0, 1.0], &mut out);
        let tm = model_aff(&target.skeleton, &out);
        let posed = torso.map(|t| t.posed(&tm));
        t_rows.push(tbody.measure(&tm, posed.as_deref()));
        let cp = clav_at_bind(&target.skeleton, tbody, &out);
        let cm = model_aff(&target.skeleton, &cp);
        let posed = torso.map(|t| t.posed(&cm));
        c_rows.push(tbody.measure(&cm, posed.as_deref()));
        // Upper-arm direction source vs target in their own facing frames.
        let spos: Vec<Vec3> = sm.iter().map(|a| Vec3::from(a.translation)).collect();
        let tpos: Vec<Vec3> = tm.iter().map(|a| Vec3::from(a.translation)).collect();
        let (sl, sf) = sbody.frame(&spos);
        let (tl, tf) = tbody.frame(&tpos);
        for (sa, ta) in sbody.arms.iter().zip(&tbody.arms) {
            let su = (spos[sa.lower] - spos[sa.upper]).normalize();
            let tu = (tpos[ta.lower] - tpos[ta.upper]).normalize();
            let s3 = Vec3::new(su.dot(sl), su.y, su.dot(sf));
            let t3 = Vec3::new(tu.dot(tl), tu.y, tu.dot(tf));
            dir_err = dir_err.max(s3.angle_between(t3).to_degrees());
        }
    }
    let mid = &times[times.len() / 4..(3 * times.len() / 4).max(times.len() / 4 + 1)];
    let poses: Vec<Vec<Trs>> = mid.iter().map(|&t| sample(&skeleton, &cooked, t)).collect();
    for (side, (raise, fwd)) in ["L", "R"].iter().zip(clav_deltas(&skeleton, &source_rig, &sbody, &poses)) {
        println!("    source clavicle {side} vs its retarget pose (parent-relative, mid-half mean): raise {raise:>6.1} forward {fwd:>6.1} deg");
    }
    report("source", &s_rows);
    report("target", &t_rows);
    report("tgt clav@bind", &c_rows);
    println!("    upper-arm direction source vs target (facing frames): max {dir_err:.1} deg");
    Ok(())
}

/// Per arm (L, R), the clavicle's rotation relative to its parent as a delta
/// from `rig`'s retarget pose (canonical frame), as the elevation (raise) and
/// forward change of the clavicle direction, mean over `poses`: the
/// shoulder `PoseOffset` (`sources.ron`) that would zero it on a source.
fn clav_deltas(skeleton: &Skeleton, rig: &RetargetRig, body: &Body, poses: &[Vec<Trs>]) -> [(f32, f32); 2] {
    let pose = retarget_pose(skeleton, rig);
    let rs = model_rots(skeleton, &pose);
    let rs_pos: Vec<Vec3> = model_aff(skeleton, &pose).iter().map(|a| Vec3::from(a.translation)).collect();
    let qs: Vec<Vec<Quat>> = poses.iter().map(|p| model_rots(skeleton, p)).collect();
    body.arms.map(|a| {
        let p = skeleton.parent(a.clav).unwrap();
        let d0 = (rs_pos[a.upper] - rs_pos[a.clav]).normalize();
        let angles = |d: Vec3| (d.y.clamp(-1.0, 1.0).asin().to_degrees(), d.z.atan2(d.x * a.side).to_degrees());
        let (e0, f0) = angles(d0);
        let (mut de, mut df) = (0.0, 0.0);
        for q in &qs {
            let (dc, dp) = (q[a.clav] * rs[a.clav].inverse(), q[p] * rs[p].inverse());
            let (e, f) = angles((dp.inverse() * dc) * d0);
            de += e - e0;
            df += f - f0;
        }
        let k = qs.len().max(1) as f32;
        (de / k, df / k)
    })
}

/// A library take's skeleton and cooked clip, as `measure_take` cooks it.
fn cook_take(source: &str, path: &Path) -> Result<(Skeleton, Clip), String> {
    let library = &mw_graphs::sources::SourcesManifest::builtin().sources[source];
    let name = path.file_name().unwrap().to_string_lossy().into_owned();
    let virtual_path = AnimSources::virtual_path(source, &name);
    let bytes = std::fs::read(path).map_err(|e| e.to_string())?;
    let imported = mw_bake::anim::anim_importers().for_path(&virtual_path).unwrap().import(&virtual_path, &bytes).map_err(|e| e.to_string())?;
    let mut skeleton_rules = MwAnimRules::skeleton_rules();
    skeleton_rules.root_motion_joint.clone_from(&imported.root_motion);
    let (skeleton, _) = Skeleton::with_rules(imported.skeleton.as_ref().ok_or("no skeleton")?, &skeleton_rules).map_err(|e| e.to_string())?;
    let (map, _) = library_rig(&skeleton, source).ok_or("no humanoid rig")?;
    let profile = sf_anim::detect_profile(&skeleton, &RigHints { profile: map.profile(&skeleton).ok(), ..RigHints::default() }).map_err(|e| e.to_string())?;
    let mut cook_rules = rules();
    if library.root_from_pelvis {
        cook_rules.root_motion.extract = false;
        cook_rules.root_motion.anchor_end = false;
    }
    let src = imported.clips.first().ok_or("no clip")?;
    let cooked = sf_anim::cook_clip_for(&skeleton, Some(&profile), src, &cook_rules, &CookSettings::default()).map_err(|e| e.to_string())?;
    Ok((skeleton, cooked))
}

#[test]
#[ignore = "measurement: Kimodo loco1 clavicle raise/forward vs its auto-aligned retarget pose, mean per gait over every take (sets sources.ron generated pose_offsets)"]
fn measure_generated_clavicle_offset() {
    let loco = AnimSources::local().dir.join("generated").join("loco1");
    let Ok(dirs) = std::fs::read_dir(&loco) else {
        eprintln!("{} missing; skipping", loco.display());
        return;
    };
    // (gait.phase) -> (sum raise, sum forward, count) over both sides.
    let mut sums: std::collections::BTreeMap<String, (f32, f32, f32, usize)> = std::collections::BTreeMap::new();
    for dir in dirs.flatten() {
        let name = dir.file_name().to_string_lossy().into_owned();
        let parts: Vec<&str> = name.split('.').collect();
        if parts.len() < 3 || parts[0] != "loco" {
            continue;
        }
        let key = format!("{}.{}", parts[1], parts[2]);
        let Ok(files) = std::fs::read_dir(dir.path().join("act")) else { continue };
        for file in files.flatten().filter(|f| f.path().extension().is_some_and(|e| e == "bvh")) {
            let Ok((skeleton, clip)) = cook_take("generated", &file.path()) else { continue };
            // Against the auto-aligned pose: no source offsets.
            let (map, rig) = library_rig(&skeleton, "").unwrap();
            let body = Body::new(&skeleton, &map);
            let times = frame_times(clip.duration);
            let mid = &times[times.len() / 4..(3 * times.len() / 4).max(times.len() / 4 + 1)];
            let poses: Vec<Vec<Trs>> = mid.iter().map(|&t| sample(&skeleton, &clip, t)).collect();
            let d = clav_deltas(&skeleton, &rig, &body, &poses);
            let e = sums.entry(key.clone()).or_default();
            e.0 += d[0].0 + d[1].0;
            e.1 += d[0].1 + d[1].1;
            e.2 += (d[0].0 - d[1].0).abs();
            e.3 += 1;
        }
    }
    println!("== Kimodo loco1 clavicle vs auto-aligned retarget pose (mean over takes and both sides, mid half)");
    let (mut walk, mut all) = ((0.0, 0.0, 0usize), (0.0, 0.0, 0usize));
    for (key, (r, f, lr, n)) in &sums {
        let k = 2.0 * *n as f32;
        println!("  {key:<14} takes {n:>3} raise {:>6.1} forward {:>6.1} | |L-R| raise {:>5.1}", r / k, f / k, lr / *n as f32);
        if key.starts_with("walk.") {
            walk = (walk.0 + r, walk.1 + f, walk.2 + 2 * n);
        }
        all = (all.0 + r, all.1 + f, all.2 + 2 * n);
    }
    println!("  every walk take: raise {:.1} forward {:.1}", walk.0 / walk.2.max(1) as f32, walk.1 / walk.2.max(1) as f32);
    println!("  every take:      raise {:.1} forward {:.1}", all.0 / all.2.max(1) as f32, all.1 / all.2.max(1) as f32);
}

fn native(vfs: &Vfs, target: &HumanoidTarget, tbody: &Body, torso: Option<&Torso>, group: &str) {
    let file = HUMANOID_SETS[0].animations[0];
    let anim = mw_anim::Animation::from_file(file, &vfs.read(&VfsPath::new(file).unwrap()).unwrap()).unwrap();
    let Some(g) = anim.groups().into_iter().find(|g| g.name.eq_ignore_ascii_case(group)) else {
        println!("\n== MW {group}: not found");
        return;
    };
    let src = anim.source_clip(&g);
    let clip = sf_anim::cook_clip_for(&target.skeleton, None, &src, &mw_anim::clip_rules(&g), &CookSettings::default()).unwrap();
    println!("\n== MW native {group} ({:.2} s)", clip.duration);
    let rows: Vec<[ArmM; 2]> = frame_times(clip.duration)
        .iter()
        .map(|&t| {
            let m = model_aff(&target.skeleton, &sample(&target.skeleton, &clip, t));
            let posed = torso.map(|t| t.posed(&m));
            tbody.measure(&m, posed.as_deref())
        })
        .collect();
    report("MW native", &rows);
    let times = frame_times(clip.duration);
    let mid = &times[times.len() / 4..(3 * times.len() / 4).max(times.len() / 4 + 1)];
    let poses: Vec<Vec<Trs>> = mid.iter().map(|&t| sample(&target.skeleton, &clip, t)).collect();
    for (side, (raise, fwd)) in ["L", "R"].iter().zip(clav_deltas(&target.skeleton, &target.rig, tbody, &poses)) {
        println!("    MW clavicle {side} vs bind (parent-relative, mid-half mean): raise {raise:>6.1} forward {fwd:>6.1} deg");
    }
}

#[test]
#[ignore = "measurement: arm spacing (abduction, swing, elbow, wrist to chest plane and mesh) source vs retargeted vs MW native, proportions, clavicle share"]
fn measure_generated_arms() {
    let Some(vfs) = install_vfs() else { return };
    let set = &HUMANOID_SETS[0];
    let bytes = vfs.read(&VfsPath::new(set.skeleton).unwrap()).unwrap();
    let skeleton = mw_anim::rig::skeleton(&mw_nif::load(&bytes).unwrap()).unwrap();
    let target = HumanoidTarget::new(skeleton).unwrap();
    let tbody = Body::new(&target.skeleton, &target.map);
    println!("== target {} (lat/hsw: outward from the chest mid-plane in half shoulder widths; clear: to the chest mesh, outside +)", set.skeleton);
    proportions("target", &target.skeleton, &tbody);
    arm_map("target", &target.skeleton, &target.map, &target.rig, &tbody);
    let torso = torso_mesh(&vfs, &target.skeleton);
    if let Some((_, _, ua, fa)) = &torso {
        println!("  arm mesh radius (median, mid bone): upper arm {:.1} cm, forearm {:.1} cm", ua * 100.0, fa * 100.0);
    }
    let torso = torso.map(|t| t.1);
    let bind = model_aff(&target.skeleton, target.skeleton.rest());
    let posed = torso.as_ref().map(|t| t.posed(&bind));
    report("MW bind", &[tbody.measure(&bind, posed.as_deref())]);
    native(&vfs, &target, &tbody, torso.as_ref(), "WalkForward");
    native(&vfs, &target, &tbody, torso.as_ref(), "RunForward");
    native(&vfs, &target, &tbody, torso.as_ref(), "Idle");
    let mut first = true;
    for take in takes() {
        if let Err(e) = measure_take(&take, &target, &tbody, torso.as_ref(), &mut first) {
            println!("\n== {}: failed: {e}", take.label);
        }
    }
}

/// `mw_nif`'s glam (another version) to ours.
fn aff(a: mw_nif::glam::Affine3A) -> Affine3A {
    Affine3A::from_cols_array(&a.to_cols_array())
}

/// A rigid body part's radius: half the mean of its two smaller bounding-box
/// extents in the part's own frame (the long one runs along the bone).
fn part_radius(vfs: &Vfs, path: &str) -> f32 {
    let Some(model) = VfsPath::new(path).ok().and_then(|p| vfs.read(&p).ok()).and_then(|b| mw_nif::load(&b).ok()) else { return f32::NAN };
    let mut world: Vec<Affine3A> = Vec::with_capacity(model.nodes.len());
    for n in &model.nodes {
        let parent = n.parent.map_or(Affine3A::IDENTITY, |p| world[p]);
        world.push(parent * aff(n.transform.to_affine()));
    }
    let (mut lo, mut hi) = (Vec3::splat(f32::INFINITY), Vec3::splat(f32::NEG_INFINITY));
    for (i, n) in model.nodes.iter().enumerate() {
        let Some(m) = n.mesh else { continue };
        if model.meshes[m].collision {
            continue;
        }
        for p in &model.meshes[m].positions {
            let v = world[i].transform_point3(Vec3::from(*p));
            (lo, hi) = (lo.min(v), hi.max(v));
        }
    }
    let mut e = (hi - lo).to_array();
    e.sort_by(f32::total_cmp);
    println!("  part {path}: bbox extents {e:.3?}");
    (e[0] + e[1]) / 4.0
}
