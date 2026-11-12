//! Right-thigh snap at the start of library clips (owner 2026-11-12,
//! `clip-preview --browse generated/loco1`): diagnosis only.
//!
//! Per take, the first frames at 30 Hz of both thighs' model-space rotation
//! in four layers: the cooked source clip on its own skeleton, the clip
//! retargeted onto `base_anim` without leg IK (FK only) and with it (the
//! bake's and `clip-preview`'s path, [`cook_library_clip`]), plus the leg IK
//! weight the retargeter uses (the source's `contact.<foot>` curve eased as
//! `Retargeter::frames` eases it) and the source knee's bend-plane normal
//! (sign flips of the pole). Run with `-- --ignored --nocapture`; skips
//! without a Morrowind install or the files.
#![allow(clippy::similar_names, clippy::items_after_statements)]

use std::path::{Path, PathBuf};
use std::sync::Arc;

use glam::{Quat, Vec3};
use mw_bake::anim::{self, HumanoidTarget, MwAnimRules, cook_library_clip, library_rig};
use mw_bake::sources::AnimSources;
use mw_graphs::HUMANOID_SETS;
use mw_install::MorrowindInstall;
use sf_anim::{Clip, ClipRules, CookSettings, FrameClip, LocalPose, Looping, RigHints, RootMotionMode, RootMotionRules, Skeleton, Trs};
use sf_character::{HumanoidBone as B, RetargetOptions, RigMap};
use sf_vfs::{Vfs, VfsPath};

const RATE: f32 = 30.0;
/// Frames printed per take.
const FRAMES: usize = 24;
/// As `sf_anim::retarget::CONTACT_BLEND`.
const CONTACT_BLEND: f32 = 0.15;

struct Take {
    label: String,
    source: &'static str,
    path: PathBuf,
    clip: Option<String>,
}

fn rules() -> ClipRules {
    // `clip-preview`'s (`mw_viewers_bevy::clip_preview::cook::preview_rules`).
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
        ("loco.walk.cycle.2", ""),
        ("loco.walk.cycle.3", ""),
        ("loco.run.cycle.1", ""),
    ] {
        let d = loco.join(dir).join("act");
        let path = if file.is_empty() {
            let mut v: Vec<PathBuf> = std::fs::read_dir(&d)
                .into_iter()
                .flatten()
                .filter_map(|e| e.ok().map(|e| e.path()))
                .filter(|p| p.extension().is_some_and(|e| e == "bvh"))
                .collect();
            v.sort();
            let Some(p) = v.into_iter().next() else { continue };
            p
        } else {
            d.join(file)
        };
        if path.exists() {
            out.push(Take { label: path.file_stem().unwrap().to_string_lossy().into_owned(), source: "generated", path, clip: None });
        } else {
            eprintln!("{} missing", path.display());
        }
    }
    let ual = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets/animation/quaternius/UAL1_RM.glb");
    for clip in ["Walk_Loop", "Jog_Fwd_Loop"] {
        out.push(Take { label: format!("UAL1:{clip}"), source: "quaternius_ual1", path: ual.clone(), clip: Some(clip.into()) });
    }
    out
}

/// Model-space rotations of every joint (parents precede children).
fn model_rots(skeleton: &Skeleton, pose: &[Trs]) -> Vec<Quat> {
    let mut out: Vec<Quat> = Vec::with_capacity(pose.len());
    for (j, t) in pose.iter().enumerate() {
        let parent = skeleton.parent(j).map_or(Quat::IDENTITY, |p| out[p]);
        out.push((parent * t.rotation).normalize());
    }
    out
}

/// Model-space positions of every joint.
fn model_pos(skeleton: &Skeleton, pose: &[Trs]) -> Vec<Vec3> {
    let mut m: Vec<glam::Affine3A> = Vec::with_capacity(pose.len());
    for (j, t) in pose.iter().enumerate() {
        let parent = skeleton.parent(j).map_or(glam::Affine3A::IDENTITY, |p| m[p]);
        m.push(parent * t.to_affine());
    }
    m.iter().map(|a| Vec3::from(a.translation)).collect()
}

#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn frame_times(duration: f32) -> Vec<f32> {
    let n = (duration * RATE).ceil().max(1.0) as usize;
    (0..=n).map(|i| (i as f32 / RATE).min(duration)).collect()
}

fn sample(skeleton: &Skeleton, clip: &Clip, t: f32) -> Vec<Trs> {
    let mut pose = LocalPose::rest(skeleton);
    clip.sample(t, &mut pose, None);
    pose.to_vec()
}

fn angle_deg(a: Quat, b: Quat) -> f32 {
    (a.inverse() * b).normalize().w.abs().min(1.0).acos().to_degrees() * 2.0
}

/// `sf_anim::retarget::eased_weights`, copied (private there).
fn eased_weights(values: &[f32], rate: f32) -> Vec<f32> {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let r = (CONTACT_BLEND * rate).round().max(0.0) as usize;
    let n = values.len();
    (0..n)
        .map(|f| {
            let window = &values[f.saturating_sub(r)..(f + r + 1).min(n)];
            #[allow(clippy::cast_precision_loss)]
            let t = (window.iter().sum::<f32>() / window.len() as f32).clamp(0.0, 1.0);
            t * t * (3.0 - 2.0 * t)
        })
        .collect()
}

fn joint(skeleton: &Skeleton, map: &RigMap, b: B) -> usize {
    map.joint(b).and_then(|j| skeleton.find(j)).unwrap_or_else(|| panic!("{b:?} unmapped"))
}

fn median(v: &[f32]) -> f32 {
    let mut s = v.to_vec();
    s.sort_by(f32::total_cmp);
    s.get(s.len() / 2).copied().unwrap_or(0.0)
}

#[allow(clippy::too_many_lines, clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn measure(take: &Take, target: &HumanoidTarget, fk_rig: &sf_anim::RetargetRig, check: bool) -> Result<(), String> {
    let manifest = mw_graphs::sources::SourcesManifest::builtin();
    let library = &manifest.sources[take.source];
    let name = take.path.file_name().unwrap().to_string_lossy().into_owned();
    let virtual_path = AnimSources::virtual_path(take.source, &name);
    let importers = anim::anim_importers();
    let bytes = std::fs::read(&take.path).map_err(|e| e.to_string())?;
    let imported = importers.for_path(&virtual_path).unwrap().import(&virtual_path, &bytes).map_err(|e| e.to_string())?;
    let joints_in = imported.skeleton.as_ref().ok_or("no skeleton")?;
    let mut skeleton_rules = MwAnimRules::skeleton_rules();
    skeleton_rules.root_motion_joint.clone_from(&imported.root_motion);
    let (skeleton, _) = Skeleton::with_rules(joints_in, &skeleton_rules).map_err(|e| e.to_string())?;
    let (map, source_rig) = library_rig(&skeleton, take.source).ok_or("no humanoid rig")?;
    let hints = RigHints { profile: map.profile(&skeleton).ok(), ..RigHints::default() };
    let profile = sf_anim::detect_profile(&skeleton, &hints).map_err(|e| e.to_string())?;
    let rules = rules();
    let retarget_root = library.root_from_pelvis.then(|| rules.root_motion.clone());
    let mut cook_rules = rules.clone();
    if retarget_root.is_some() {
        cook_rules.root_motion.extract = false;
        cook_rules.root_motion.anchor_end = false;
    }
    let src = match &take.clip {
        Some(c) => imported.clips.iter().find(|x| &x.name == c),
        None => imported.clips.first(),
    }
    .ok_or("no clip")?;
    let settings = CookSettings::default();
    let cooked = sf_anim::cook_clip_for(&skeleton, Some(&profile), src, &cook_rules, &settings).map_err(|e| e.to_string())?;
    let retarget = |rig: &sf_anim::RetargetRig| -> Result<Clip, String> {
        let r = sf_anim::Retargeter::new(&skeleton, &source_rig, &target.skeleton, rig).map_err(|e| e.to_string())?;
        match &retarget_root {
            Some(root) => r.clip_with_root(&cooked, root, &settings),
            None => r.clip(&cooked, &settings),
        }
        .map_err(|e| e.to_string())
    };
    let ik = retarget(&target.rig)?;
    let fk = retarget(fk_rig)?;
    if check {
        let mut ours = ik.clone();
        sf_anim::transfer_contacts(&cooked.meta.contacts, &target.skeleton, &target.profile, &mut ours);
        let mut baked = cook_library_clip(take.source, &take.path, take.clip.as_deref(), &rules, target)?;
        baked.name.clone_from(&ours.name);
        assert!(baked.to_bytes() == ours.to_bytes(), "{}: the replicated cook differs from cook_library_clip", take.label);
    }

    // Contact curves and the eased IK weights, at the cooked clip's own rate.
    let fc = FrameClip::from_clip(&cooked);
    let contact = |b: B| -> (Vec<f32>, Vec<f32>) {
        let n = format!("contact.{}", skeleton.name(joint(&skeleton, &map, b)));
        let raw: Vec<f32> = fc
            .curves
            .iter()
            .find(|c| c.name == n)
            .map_or_else(|| vec![1.0; fc.frame_count], |c| (0..fc.frame_count).map(|f| c.values.get(f).copied().unwrap_or(0.0)).collect());
        let eased = eased_weights(&raw, fc.rate);
        (raw, eased)
    };
    let has_contact = fc.curves.iter().any(|c| c.name.starts_with("contact."));
    let (r_raw, r_w) = contact(B::RightFoot);
    let (l_raw, l_w) = contact(B::LeftFoot);
    let at = |v: &[f32], t: f32| v[((t * fc.rate).round() as usize).min(v.len() - 1)];

    let (s_rt, s_lt, s_rk, s_ra) =
        (joint(&skeleton, &map, B::RightUpperLeg), joint(&skeleton, &map, B::LeftUpperLeg), joint(&skeleton, &map, B::RightLowerLeg), joint(&skeleton, &map, B::RightFoot));
    let (s_lk, s_la) = (joint(&skeleton, &map, B::LeftLowerLeg), joint(&skeleton, &map, B::LeftFoot));
    let (t_rt, t_lt) = (joint(&target.skeleton, &target.map, B::RightUpperLeg), joint(&target.skeleton, &target.map, B::LeftUpperLeg));
    let t_pelvis = joint(&target.skeleton, &target.map, B::Hips);

    let times = frame_times(ik.duration);
    struct Row {
        src: [Quat; 2],
        fk: [Quat; 2],
        ik: [Quat; 2],
        ik_local: [Quat; 2],
        pelvis: Quat,
        normal: [Vec3; 2],
        bend: [f32; 2],
    }
    let rows: Vec<Row> = times
        .iter()
        .map(|&t| {
            let sp = sample(&skeleton, &cooked, t);
            let sr = model_rots(&skeleton, &sp);
            let spos = model_pos(&skeleton, &sp);
            let fp = sample(&target.skeleton, &fk, t);
            let fr = model_rots(&target.skeleton, &fp);
            let ip = sample(&target.skeleton, &ik, t);
            let ir = model_rots(&target.skeleton, &ip);
            let leg = |h: usize, k: usize, a: usize| {
                let (u, l) = (spos[k] - spos[h], spos[a] - spos[k]);
                let n = u.cross(l);
                (n.normalize_or_zero(), n.length() / (u.length() * l.length()).max(1e-12))
            };
            let (rn, rb) = leg(s_rt, s_rk, s_ra);
            let (ln, lb) = leg(s_lt, s_lk, s_la);
            Row {
                src: [sr[s_rt], sr[s_lt]],
                fk: [fr[t_rt], fr[t_lt]],
                ik: [ir[t_rt], ir[t_lt]],
                ik_local: [ip[t_rt].rotation, ip[t_lt].rotation],
                pelvis: ir[t_pelvis],
                normal: [rn, ln],
                bend: [rb, lb],
            }
        })
        .collect();
    let step = |f: usize, g: &dyn Fn(&Row) -> Quat| if f == 0 { 0.0 } else { angle_deg(g(&rows[f - 1]), g(&rows[f])) };
    let steps_of = |g: &dyn Fn(&Row) -> Quat| (1..rows.len()).map(|f| step(f, g)).collect::<Vec<f32>>();
    let med = |g: &dyn Fn(&Row) -> Quat| median(&steps_of(g));

    println!(
        "\n== {} ({} frames @ {:.0} Hz cooked, {:.2} s; contact curves: {has_contact}; root_from_pelvis: {})",
        take.label, fc.frame_count, fc.rate, ik.duration, library.root_from_pelvis
    );
    println!(
        "  median step deg/frame: src R {:.1} L {:.1} | fk R {:.1} L {:.1} | ik R {:.1} L {:.1} | pelvis {:.1}",
        med(&|r| r.src[0]),
        med(&|r| r.src[1]),
        med(&|r| r.fk[0]),
        med(&|r| r.fk[1]),
        med(&|r| r.ik[0]),
        med(&|r| r.ik[1]),
        med(&|r| r.pelvis)
    );
    println!("  thigh model-space step (deg/frame) | IK minus FK (deg) | contact raw/eased | src knee normal dot prev, bend sin");
    println!(
        "  {:>3} {:>6} {:>6} {:>6} {:>6} {:>6} {:>6} {:>6} {:>6} | {:>5} {:>5} | {:>9} {:>9} | {:>6} {:>5} {:>6} {:>5}",
        "f", "srcR", "srcL", "fkR", "fkL", "ikR", "ikL", "ikRloc", "pelv", "dR", "dL", "R c/w", "L c/w", "nR·", "bR", "nL·", "bL"
    );
    for f in 0..rows.len().min(FRAMES) {
        let r = &rows[f];
        let dot = |i: usize| if f == 0 { 1.0 } else { rows[f - 1].normal[i].dot(r.normal[i]) };
        let t = times[f];
        println!(
            "  {:>3} {:>6.1} {:>6.1} {:>6.1} {:>6.1} {:>6.1} {:>6.1} {:>6.1} {:>6.1} | {:>5.1} {:>5.1} | {:>4.2}/{:<4.2} {:>4.2}/{:<4.2} | {:>6.2} {:>5.2} {:>6.2} {:>5.2}",
            f,
            step(f, &|r| r.src[0]),
            step(f, &|r| r.src[1]),
            step(f, &|r| r.fk[0]),
            step(f, &|r| r.fk[1]),
            step(f, &|r| r.ik[0]),
            step(f, &|r| r.ik[1]),
            step(f, &|r| r.ik_local[0]),
            step(f, &|r| r.pelvis),
            angle_deg(r.fk[0], r.ik[0]),
            angle_deg(r.fk[1], r.ik[1]),
            at(&r_raw, t),
            at(&r_w, t),
            at(&l_raw, t),
            at(&l_w, t),
            dot(0),
            r.bend[0],
            dot(1),
            r.bend[1],
        );
    }
    // Where in the whole clip is the largest right-thigh IK step?
    let ik_r = steps_of(&|r| r.ik[0]);
    let (fmax, smax) = ik_r.iter().enumerate().fold((0, 0.0f32), |a, (i, &s)| if s > a.1 { (i + 1, s) } else { a });
    let ik_l = steps_of(&|r| r.ik[1]);
    let lmax = ik_l.iter().copied().fold(0.0f32, f32::max);
    let fk_r = steps_of(&|r| r.fk[0]);
    println!(
        "  whole clip: max ik R step {smax:.1} deg at frame {fmax} (fk R there {:.1}), max ik L step {lmax:.1}; max |IK-FK| R {:.1} L {:.1}",
        fk_r.get(fmax.saturating_sub(1)).copied().unwrap_or(0.0),
        rows.iter().map(|r| angle_deg(r.fk[0], r.ik[0])).fold(0.0f32, f32::max),
        rows.iter().map(|r| angle_deg(r.fk[1], r.ik[1])).fold(0.0f32, f32::max)
    );
    Ok(())
}

#[test]
#[ignore = "measurement: right-thigh snap at clip start (loco1 Kimodo + UAL), per frame: source, FK, IK, contact weight, knee pole"]
fn measure_thigh_snap() {
    let Some(vfs) = install_vfs() else { return };
    let set = &HUMANOID_SETS[0];
    let bytes = vfs.read(&VfsPath::new(set.skeleton).unwrap()).unwrap();
    let skeleton = mw_anim::rig::skeleton(&mw_nif::load(&bytes).unwrap()).unwrap();
    let target = HumanoidTarget::new(skeleton).unwrap();
    let fk_rig = target.map.retarget_rig(&target.skeleton, RetargetOptions { ik_legs: false }).unwrap();
    for (i, take) in takes().iter().enumerate() {
        if let Err(e) = measure(take, &target, &fk_rig, i == 0) {
            println!("\n== {}: failed: {e}", take.label);
        }
    }
}

// ---------------------------------------------------------------------------
// Hypothesis 4: the graph. The clip as `clip-preview` plays it (`play_full`:
// slot FULL, no blend in, priority 9) through the humanoid graph's op stack,
// against the clip sampled raw at the montage's time; then whether the jump
// at the start is the hard cut from the pose before (idle, or the clip's own
// end when browse replays it) and how the two thighs compare over every
// generated clip.

const DT: f32 = 1.0 / 60.0;
/// Graph steps recorded after the montage starts.
const GRAPH_STEPS: usize = 30;
const CUT_BONES: [B; 8] = [B::Hips, B::Spine, B::RightUpperLeg, B::LeftUpperLeg, B::RightLowerLeg, B::LeftLowerLeg, B::RightFoot, B::LeftFoot];

fn humanoid_set(vfs: &Vfs) -> mw_graphs::AnimSet {
    let read = |path: &str| vfs.read(&VfsPath::new(path).ok()?).ok();
    let set = &HUMANOID_SETS[0];
    let skeleton = mw_anim::rig::skeleton(&mw_nif::load(&read(set.skeleton).unwrap()).unwrap()).unwrap();
    let mut anims = Vec::new();
    for a in set.animations {
        let Some(bytes) = read(a) else { continue };
        let anim = mw_anim::Animation::from_file(a, &bytes).unwrap();
        if !anim.text_keys.is_empty() {
            anims.push(((*a).to_owned(), anim));
        }
    }
    mw_graphs::AnimSet::new(set.name, skeleton, anims).with_humanoid_rig()
}

fn op_name(o: &sf_anim_graph::asset::OpSpec) -> String {
    let s = format!("{o:?}");
    s.split(['(', ' ', '{']).next().unwrap_or("?").to_owned()
}

fn pick(r: &[Quat], bones: &[usize]) -> Vec<Quat> {
    bones.iter().map(|&j| r[j]).collect()
}

struct GraphRun {
    /// Model rotations of `bones` before the play, then per step after it.
    before: Vec<Quat>,
    graph: Vec<Vec<Quat>>,
    /// The clip sampled raw at `k * DT`.
    raw: Vec<Vec<Quat>>,
}

/// One graph run: 1 s of idle (then, with `replay`, the clip for its whole
/// length, as browse's loop does before it replays), then the clip.
fn graph_run(set: &mut mw_graphs::AnimSet, asset: &sf_anim_graph::asset::GraphAsset, clip: &Arc<Clip>, bones: &[usize], replay: bool) -> Result<GraphRun, String> {
    use sf_anim_graph::{Binding, LateClip, MontageClip, SlotRequest, Variety};
    let graph = set.compile("thigh_snap", asset)?;
    let clips = set.binding_clips(&graph)?;
    let mut sim = mw_graphs::sim::Sim::new(graph, set.skeleton.clone(), clips).map_err(|e| format!("{e:?}"))?;
    let late = Arc::new(LateClip::new(&sim.graph, &sim.skeleton, Arc::clone(clip)).map_err(|e| format!("late clip: {e:?}"))?);
    let play = |sim: &mut mw_graphs::sim::Sim| -> Result<(), String> {
        let b = Binding { graph: &sim.graph, skeleton: &sim.skeleton, clips: &sim.clips };
        sim.instance.set_variety(&b, Variety::default());
        let req = SlotRequest { blend_in: 0.0, priority: 9, ..SlotRequest::new(MontageClip::Late(Arc::clone(&late))) };
        sim.play_slot(mw_graphs::build::slots::FULL, req).map(|_| ()).map_err(|e| format!("{e:?}"))
    };
    sim.run(1.0, DT);
    if replay {
        play(&mut sim)?;
        sim.run(clip.duration.max(0.1), DT);
    }
    let before = pick(&model_rots(&sim.skeleton, &sim.out.pose.to_vec()), bones);
    play(&mut sim)?;
    let (mut graph_q, mut raw_q) = (Vec::new(), Vec::new());
    for k in 0..GRAPH_STEPS {
        sim.run(DT, DT);
        graph_q.push(pick(&model_rots(&sim.skeleton, &sim.out.pose.to_vec()), bones));
        #[allow(clippy::cast_precision_loss)]
        let t = (k as f32 * DT).min(clip.duration);
        raw_q.push(pick(&model_rots(&sim.skeleton, &sample(&sim.skeleton, clip, t)), bones));
    }
    Ok(GraphRun { before, graph: graph_q, raw: raw_q })
}

/// The largest single step in the first three graph steps (the cut), per bone.
fn cut(run: &GraphRun, b: usize) -> f32 {
    let mut prev = run.before[b];
    let mut m = 0.0f32;
    for g in run.graph.iter().take(3) {
        m = m.max(angle_deg(prev, g[b]));
        prev = g[b];
    }
    m
}

#[test]
#[ignore = "measurement: right-thigh snap at clip start through the humanoid graph (play_full), per op removed, and the start cut over every loco1 clip"]
#[allow(clippy::too_many_lines, clippy::cast_precision_loss)]
fn measure_thigh_snap_graph() {
    let Some(vfs) = install_vfs() else { return };
    let mut set = humanoid_set(&vfs);
    let target = HumanoidTarget::new(set.skeleton.clone()).unwrap();
    let bones: Vec<usize> = CUT_BONES.iter().map(|b| joint(&target.skeleton, &target.map, *b)).collect();
    let (ri, li) = (2, 3);
    let base = mw_graphs::asset(mw_graphs::Template::Humanoid);
    let mut variants: Vec<(String, sf_anim_graph::asset::GraphAsset)> = vec![("full graph".into(), base.clone())];
    let mut none = base.clone();
    none.procedural.clear();
    variants.push(("no procedural ops".into(), none));
    for i in 0..base.procedural.len() {
        let mut a = base.clone();
        let name = op_name(&a.procedural.remove(i));
        variants.push((format!("without {name}#{i}"), a));
    }
    for take in takes().iter().filter(|t| t.label.contains("walk.cycle.1") || t.label.contains("Walk_Loop")) {
        let clip = match cook_library_clip(take.source, &take.path, take.clip.as_deref(), &rules(), &target) {
            Ok(c) => Arc::new(c),
            Err(e) => {
                println!("{}: {e}", take.label);
                continue;
            }
        };
        for replay in [false, true] {
            println!("\n== {} through the graph, from {} (60 Hz steps; deg)", take.label, if replay { "its own end (browse loop replay)" } else { "idle" });
            println!("  {:<26} | R thigh steps after play, graph/raw (k=0..6)                     | max |graph-raw| R L, k>=2", "variant");
            for (name, asset) in &variants {
                match graph_run(&mut set, asset, &clip, &bones, replay) {
                    Ok(run) => {
                        let mut prev = run.before[ri];
                        let mut prev_raw = run.raw[0][ri];
                        let steps: String = run
                            .graph
                            .iter()
                            .zip(&run.raw)
                            .take(7)
                            .map(|(g, r)| {
                                let s = format!("{:>4.1}/{:<4.1}", angle_deg(prev, g[ri]), angle_deg(prev_raw, r[ri]));
                                (prev, prev_raw) = (g[ri], r[ri]);
                                s
                            })
                            .collect::<Vec<_>>()
                            .join(" ");
                        let dmax = |i: usize| run.graph.iter().zip(&run.raw).skip(2).map(|(a, b)| angle_deg(a[i], b[i])).fold(0.0f32, f32::max);
                        println!("  {name:<26} | {steps} | {:.1} {:.1}", dmax(ri), dmax(li));
                    }
                    Err(e) => println!("  {name:<26} failed: {e}"),
                }
            }
            if let Ok(run) = graph_run(&mut set, &base, &clip, &bones, replay) {
                let row: String = CUT_BONES.iter().enumerate().map(|(b, bone)| format!("{bone:?} {:.1}", cut(&run, b))).collect::<Vec<_>>().join(", ");
                println!("  cut per bone (largest step in the first 3 updates, full graph): {row}");
            }
        }
    }

    // Every generated loco1 clip: the start cut from idle and from its own end.
    let loco = AnimSources::local().dir.join("generated").join("loco1");
    let mut files = Vec::new();
    let mut stack = vec![loco];
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(&d).into_iter().flatten().flatten() {
            let p = e.path();
            if p.is_dir() && p.file_name().is_some_and(|n| n != "_cache") {
                stack.push(p);
            } else if p.extension().is_some_and(|x| x == "bvh") {
                files.push(p);
            }
        }
    }
    files.sort();
    let ual_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets/animation/quaternius/UAL1_RM.glb");
    let ual = ["Walk_Loop", "Jog_Fwd_Loop", "Sprint_Loop", "Idle_Loop", "Walk_Formal_Loop", "Crouch_Fwd_Loop"]
        .into_iter()
        .map(|c| Take { label: format!("UAL1:{c}"), source: "quaternius_ual1", path: ual_path.clone(), clip: Some(c.into()) });
    let all: Vec<Take> = files
        .into_iter()
        .map(|p| Take { label: p.file_stem().unwrap().to_string_lossy().into_owned(), source: "generated", path: p, clip: None })
        .chain(ual)
        .collect();
    println!("\n== start cut over every clip (full graph; thigh = largest step in the first 3 updates; deg)");
    println!("  {:<58} {:>7} {:>7} {:>7} {:>7}", "clip", "idle R", "idle L", "loop R", "loop L");
    let (mut n, mut r_big, mut sums) = (0usize, 0usize, [0.0f32; 4]);
    let mut knees = [0.0f32; 2];
    for take in &all {
        let Ok(clip) = cook_library_clip(take.source, &take.path, take.clip.as_deref(), &rules(), &target) else {
            println!("  {:<58} failed", take.label);
            continue;
        };
        let clip = Arc::new(clip);
        let (Ok(a), Ok(b)) = (graph_run(&mut set, &base, &clip, &bones, false), graph_run(&mut set, &base, &clip, &bones, true)) else { continue };
        let v = [cut(&a, ri), cut(&a, li), cut(&b, ri), cut(&b, li)];
        if n == 0 {
            // The graph's idle pose: knee bend (thigh to shin model rotation), per side.
            let bend = |t: usize, k: usize| angle_deg(a.before[t], a.before[k]);
            println!("  graph idle pose: knee bend R {:.1} L {:.1} deg", bend(ri, 4), bend(li, 5));
        }
        knees[0] += cut(&a, 4);
        knees[1] += cut(&a, 5);
        n += 1;
        r_big += usize::from(v[0] > v[1]);
        for (s, x) in sums.iter_mut().zip(v) {
            *s += x;
        }
        if n <= 40 || take.label.starts_with("UAL") {
            println!("  {:<58} {:>7.1} {:>7.1} {:>7.1} {:>7.1}", take.label, v[0], v[1], v[2], v[3]);
        }
    }
    let m = |i: usize| sums[i] / n.max(1) as f32;
    println!("  {n} clips: mean idle cut R {:.1} L {:.1}, loop cut R {:.1} L {:.1}; right thigh's idle cut larger in {r_big}/{n}; mean idle knee cut R {:.1} L {:.1}",
        m(0),
        m(1),
        m(2),
        m(3),
        knees[0] / n.max(1) as f32,
        knees[1] / n.max(1) as f32
    );
}
