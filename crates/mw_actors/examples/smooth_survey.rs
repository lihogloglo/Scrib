//! Smooth-bodies survey on real Morrowind data (superfargoth
//! `docs/specs/smooth-bodies.md`, "Survey"): every playable race × sex
//! naked, plus armor and clothing outfits on three bodies, in seven poses
//! (elbows 90°, knees 90°, shoulders raised, two walk and two run frames).
//! Per case and pose, vanilla vs smoothed: seam opening (max / median over
//! the seam links) and pokes (vertices of a part that were inside a linked
//! part at rest and come out of it in the pose). Also: smoothing time,
//! determinism (a second run on another thread must be bit-identical) and
//! idempotence (smoothing the smoothed set, which is then already-skinned
//! content, must change nothing). Writes `metrics.txt` and OBJs of the best
//! and worst cases.
//!
//! `cargo run --release -p mw_actors --example smooth_survey -- [out dir]`
//! (default `../shots/smoothbody2`).

#![allow(clippy::too_many_lines, clippy::cast_precision_loss, clippy::many_single_char_names, clippy::cast_possible_truncation)]

use std::collections::HashMap;
use std::fmt::Write as _;
use std::sync::Arc;
use std::time::Instant;

use glam::{Affine3A, Quat, Vec3};
use mw_actors::MwActors;
use mw_install::MorrowindInstall;
use sf_anim::{BoundSource, Skeleton, Trs};
use sf_character::{
    CharacterSource, CharacterState, PartKind, PartLookup, SeamLink, SkinSurface, SmoothSettings, View, guard_seams, penetration, resolve_recipe, seam_openings,
    skinning_matrices,
    smooth_body,
};
use sf_model::{CookSettings, Cooked, CookedSkinned, ModelImporter};
use sf_records::{RecordDb, RecordId};
use sf_vfs::{Vfs, VfsPath};
use tes3::esp::{Armor, Clothing, Npc, NpcFlags, Race, RaceFlags};

/// Outfits (item ids): light, medium and heavy armor, a robe, a skirt, everyday clothing.
const OUTFITS: [(&str, &[&str]); 8] = [
    ("netch", &["netch_leather_cuirass", "netch_leather_greaves", "netch_leather_pauldron_left", "netch_leather_pauldron_right", "netch_leather_boots", "netch_leather_gauntlet_left", "netch_leather_gauntlet_right"]),
    ("glass", &["glass_cuirass", "glass_greaves", "glass_pauldron_left", "glass_pauldron_right", "glass_boots", "glass_bracer_left", "glass_bracer_right"]),
    ("bonemold", &["bonemold_cuirass", "bonemold_greaves", "bonemold_pauldron_l", "bonemold_pauldron_r", "bonemold_boots", "bonemold_bracer_left", "bonemold_bracer_right"]),
    ("iron", &["iron_cuirass", "iron_greaves", "iron_pauldron_left", "iron_pauldron_right", "iron boots", "iron_gauntlet_left", "iron_gauntlet_right"]),
    ("daedric", &["daedric_cuirass", "daedric_greaves", "daedric_pauldron_left", "daedric_pauldron_right", "daedric_boots", "daedric_gauntlet_left", "daedric_gauntlet_right"]),
    ("robe", &["common_robe_01", "common_shoes_01"]),
    ("skirt", &["common_skirt_01", "common_shirt_01", "common_shoes_01"]),
    ("clothes", &["common_shirt_01", "common_pants_01", "common_shoes_01"]),
];

/// Guard tolerance: a seam may open this much (m) more than vanilla before it reverts.
const GUARD_TOLERANCE: f32 = 0.0005;

struct Case {
    name: String,
    skeleton: String,
    ids: Vec<String>,
    parts: Vec<SkinSurface>,
}

struct Rig {
    names: Vec<String>,
    rest: Vec<Affine3A>,
    parents: Vec<Option<usize>>,
    poses: Vec<(String, Vec<Affine3A>)>,
}

#[derive(Clone, Default)]
struct PoseStats {
    open_max: f32,
    open_median: f32,
    worst_link: Option<SeamLink>,
    pokes: usize,
    poke_max: f32,
    poke_pair: (usize, usize),
    openings: Vec<f32>,
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let out_dir = std::path::PathBuf::from(args.first().map_or("../shots/smoothbody2", String::as_str));
    std::fs::create_dir_all(&out_dir).unwrap();
    let (install, _) = MorrowindInstall::detect(None).expect("Morrowind install");
    let vfs = Arc::new(mw_vfs::mount_install(&install, Vfs::builder()).unwrap().vfs);
    let content = mw_records::load(&vfs, &install.game_files()).unwrap();
    let db = Arc::new(content.db);
    let a = MwActors::new(Arc::clone(&db), Arc::clone(&vfs));
    let naked = CharacterState { worn: Some(Vec::new()), held: None };

    // One NPC per playable race × sex (first in load order whose recipe resolves).
    let mut races: Vec<(&RecordId, &Race)> = db.iter::<Race>().filter(|(_, r)| r.data.flags.contains(RaceFlags::PLAYABLE)).collect();
    races.sort_by_key(|(id, _)| id.as_str().to_owned());
    let mut picks: Vec<(String, String)> = Vec::new();
    for (race, _) in &races {
        for female in [false, true] {
            let npc = db.iter::<Npc>().find(|(id, n)| {
                n.race.eq_ignore_ascii_case(race.as_str()) && n.npc_flags.contains(NpcFlags::FEMALE) == female && a.recipe("NPC_", id, &naked).is_ok()
            });
            match npc {
                Some((id, _)) => picks.push((format!("{} {}", race.as_str(), if female { "f" } else { "m" }), id.as_str().to_owned())),
                None => println!("no NPC for {} {}", race.as_str(), if female { "female" } else { "male" }),
            }
        }
    }
    let mut cooked: HashMap<String, Cooked> = HashMap::new();
    let mut rigs: HashMap<String, Rig> = HashMap::new();
    let mut cases: Vec<Case> = Vec::new();
    for (name, npc) in &picks {
        if let Some(c) = load_case(&a, &vfs, &mut cooked, &mut rigs, format!("naked {name} ({npc})"), npc, Vec::new()) {
            cases.push(c);
        }
    }
    let outfit_bodies: Vec<&(String, String)> =
        ["dark elf m", "imperial f", "argonian m"].iter().filter_map(|w| picks.iter().find(|(n, _)| n.eq_ignore_ascii_case(w))).collect();
    for (outfit, items) in OUTFITS {
        let (found, missing): (Vec<&str>, Vec<&str>) = items.iter().partition(|id| item_exists(&db, id));
        if !missing.is_empty() {
            println!("outfit {outfit}: missing items {missing:?}");
        }
        for (body, npc) in &outfit_bodies {
            let worn = found.iter().map(|id| RecordId::new(id)).collect();
            if let Some(c) = load_case(&a, &vfs, &mut cooked, &mut rigs, format!("{outfit} on {body} ({npc})"), npc, worn) {
                cases.push(c);
            }
        }
    }

    // `SMOOTH_ONLY=<substring>`: only matching cases (iteration aid).
    if let Ok(only) = std::env::var("SMOOTH_ONLY") {
        cases.retain(|c| only.split(',').any(|o| c.name.contains(o)));
    }
    let settings = SmoothSettings::default();
    let guard_on = std::env::var("SMOOTH_GUARD").map_or(true, |v| v != "0");
    let mut lines = String::new();
    let mut table = String::new();
    let mut failures = String::new();
    let mut worst: Option<(f32, usize, usize)> = None;
    let mut best: Option<(f32, usize)> = None;
    let mut smoothed: Vec<Vec<SkinSurface>> = Vec::new();
    let (mut total_ms, mut nondeterministic, mut not_idempotent) = (0.0, 0, 0);
    let (mut guard_seams_total, mut guard_reverted_total, mut guard_lines, mut worse_than_vanilla) = (0, 0, String::new(), 0);
    writeln!(table, "| case | parts / vertices | links (overlap) | ms | opening max vanilla -> smooth (pose) | unguarded | guard reverted seams | median smooth | pokes vanilla -> smooth (max mm) |").unwrap();
    writeln!(table, "|---|---|---|---|---|---|---|---|---|").unwrap();
    for (ci, case) in cases.iter().enumerate() {
        let rig = &rigs[&case.skeleton];
        let skinnings: Vec<Vec<Affine3A>> = rig.poses.iter().map(|(_, p)| skinning_matrices(&rig.rest, p)).collect();
        // Smooth, then the per-seam guard against the original weights in the survey poses.
        let run = |parts: &[SkinSurface]| {
            let mut p = parts.to_vec();
            let report = smooth_body(&mut p, &rig.parents, &settings);
            let unguarded = p.clone();
            let guard = if guard_on { guard_seams(parts, &mut p, &report.seams, &skinnings, GUARD_TOLERANCE, settings.band) } else { sf_character::GuardReport::default() };
            (p, unguarded, report, guard)
        };
        let t = Instant::now();
        let (smooth, unguarded, report, guard) = run(&case.parts);
        let ms = t.elapsed().as_secs_f64() * 1e3;
        total_ms += ms;
        guard_seams_total += guard.seams;
        guard_reverted_total += guard.reverted.len();
        for (x, y) in &guard.reverted {
            writeln!(guard_lines, "- {}: {} / {}", case.name, short(&case.ids[*x as usize]), short(&case.ids[*y as usize])).unwrap();
        }
        // Determinism: the same input on another thread, bit for bit.
        let again = std::thread::scope(|s| s.spawn(|| run(&case.parts).0).join().unwrap());
        nondeterministic += usize::from(again != smooth);
        // Idempotence: smoothed output is already-skinned content.
        let twice = run(&smooth).0;
        let changed: usize = twice.iter().zip(&smooth).map(|(x, y)| x.skin.iter().zip(&y.skin).filter(|(p, q)| p != q).count()).sum();
        not_idempotent += usize::from(changed > 0);
        // Seams between two authored skins are left as authored (not measured: the step does not touch them).
        let authored = |p: u32| !is_rigid(&case.parts[p as usize]);
        let seams: Vec<SeamLink> = report.seams.iter().copied().filter(|l| !(authored(l.a.0) && authored(l.b.0))).collect();
        let pairs: Vec<(usize, usize)> = {
            let mut p: Vec<(usize, usize)> = seams.iter().flat_map(|l| [(l.a.0 as usize, l.b.0 as usize), (l.b.0 as usize, l.a.0 as usize)]).collect();
            p.sort_unstable();
            p.dedup();
            p
        };
        for joint in std::env::var("SMOOTH_PROBE").unwrap_or_default().split(',').filter(|j| !j.is_empty()) {
            probe(case, rig, &smooth, &seams, joint);
        }
        let mut u_max = 0.0_f32;
        let pair_of = |l: &SeamLink| (l.a.0.min(l.b.0), l.a.0.max(l.b.0));
        let mut per_seam: HashMap<(u32, u32), (f32, f32)> = HashMap::new();
        let (mut v_max, mut s_max, mut s_med, mut v_pokes, mut s_pokes, mut s_poke_max) = ((0.0_f32, ""), (0.0_f32, ""), 0.0_f32, 0, 0, 0.0_f32);
        for (pi, (pose, posed)) in rig.poses.iter().enumerate() {
            let skinning = skinning_matrices(&rig.rest, posed);
            let v = measure(&case.parts, &seams, &pairs, &skinning);
            let s = measure(&smooth, &seams, &pairs, &skinning);
            u_max = u_max.max(measure(&unguarded, &seams, &[], &skinning).open_max);
            for (l, (x, y)) in seams.iter().zip(v.openings.iter().zip(&s.openings)) {
                let e = per_seam.entry(pair_of(l)).or_default();
                *e = (e.0.max(*x), e.1.max(*y));
            }
            let link_name = |st: &PoseStats| st.worst_link.map_or(String::new(), |l| format!("{} / {}", short(&case.ids[l.a.0 as usize]), short(&case.ids[l.b.0 as usize])));
            writeln!(
                lines,
                "{:<48} {pose:<8} opening max {:6.1} -> {:5.1} median {:4.1} -> {:4.1} mm | pokes {:3} ({:5.1}) -> {:3} ({:5.1}) | worst smooth link {}",
                case.name,
                v.open_max * 1e3,
                s.open_max * 1e3,
                v.open_median * 1e3,
                s.open_median * 1e3,
                v.pokes,
                v.poke_max * 1e3,
                s.pokes,
                s.poke_max * 1e3,
                link_name(&s)
            )
            .unwrap();
            if v.open_max > v_max.0 {
                v_max = (v.open_max, pose);
            }
            if s.open_max > s_max.0 {
                s_max = (s.open_max, pose);
            }
            s_med = s_med.max(s.open_median);
            v_pokes = v_pokes.max(v.pokes);
            s_pokes = s_pokes.max(s.pokes);
            s_poke_max = s_poke_max.max(s.poke_max);
            if s.open_max > 0.040
                && let Some(l) = s.worst_link
            {
                let w = |set: &[SkinSurface], (p, v): (u32, u32)| {
                    let k = set[p as usize].skin[v as usize];
                    format!("{:?}", k.joints.iter().zip(k.weights).filter(|x| x.1 > 0.0).map(|(j, x)| (*j, (x * 100.0).round())).collect::<Vec<_>>())
                };
                writeln!(
                    lines,
                    "    detail: inside {} rest {:.1} mm, a {} -> {}, b {} -> {}",
                    l.inside,
                    case.parts[l.a.0 as usize].positions[l.a.1 as usize].distance(case.parts[l.b.0 as usize].positions[l.b.1 as usize]) * 1e3,
                    w(&case.parts, l.a),
                    w(&smooth, l.a),
                    w(&case.parts, l.b),
                    w(&smooth, l.b)
                )
                .unwrap();
            }
            if s.open_max > 0.010 {
                writeln!(failures, "- {} {pose}: smooth seam opens {:.1} mm (vanilla max {:.1}) at {}", case.name, s.open_max * 1e3, v.open_max * 1e3, link_name(&s)).unwrap();
            }
            if s.pokes > v.pokes && s.poke_max > 0.005 {
                let (x, y) = s.poke_pair;
                writeln!(failures, "- {} {pose}: {} new pokes (max {:.1} mm), {} through {}", case.name, s.pokes - v.pokes, s.poke_max * 1e3, short(&case.ids[x]), short(&case.ids[y])).unwrap();
            }
            if worst.is_none_or(|w| s.open_max > w.0) {
                worst = Some((s.open_max, ci, pi));
            }
        }
        // Never worse than vanilla, per seam (the guard's promise, on the survey's seam set).
        let mut keys: Vec<_> = per_seam.keys().copied().collect();
        keys.sort_unstable();
        for k in keys {
            let (x, y) = per_seam[&k];
            if y > x + GUARD_TOLERANCE {
                worse_than_vanilla += 1;
                writeln!(failures, "- {}: seam {} / {} opens {:.1} mm, vanilla {:.1}", case.name, short(&case.ids[k.0 as usize]), short(&case.ids[k.1 as usize]), y * 1e3, x * 1e3).unwrap();
            }
        }
        if best.is_none_or(|b| s_max.0 < b.0) {
            best = Some((s_max.0, ci));
        }
        let vertices: usize = case.parts.iter().map(|p| p.positions.len()).sum();
        let overlap = report.seams.iter().filter(|l| l.inside).count();
        writeln!(
            table,
            "| {} | {} / {vertices} | {} ({overlap}) | {ms:.0} | {:.1} ({}) -> {:.1} ({}) | {:.1} | {} / {} | {:.1} | {v_pokes} -> {s_pokes} ({:.1}) |",
            case.name,
            case.parts.len(),
            report.seams.len(),
            v_max.0 * 1e3,
            v_max.1,
            s_max.0 * 1e3,
            s_max.1,
            u_max * 1e3,
            guard.reverted.len(),
            guard.seams,
            s_med * 1e3,
            s_poke_max * 1e3
        )
        .unwrap();
        println!("{} done: {} links ({overlap} overlap), {} band, {} sweeps, {ms:.0} ms, idempotent changes {changed}", case.name, report.seams.len(), report.band, report.iterations);
        smoothed.push(smooth);
    }
    let summary = format!(
        "{} cases, smooth_body total {total_ms:.0} ms (mean {:.1} ms, indicative, BelowNormal, owner on machine); nondeterministic {nondeterministic}; not idempotent {not_idempotent}; guard reverted {guard_reverted_total} of {guard_seams_total} seams; seams worse than vanilla {worse_than_vanilla}\n",
        cases.len(),
        total_ms / cases.len().max(1) as f64
    );
    // OBJs: the worst case in its worst pose, and the best case in that pose.
    if let (Some((_, wc, wp)), Some((_, bc))) = (worst, best) {
        for (tag, ci) in [("worst", wc), ("best", bc)] {
            let case = &cases[ci];
            let rig = &rigs[&case.skeleton];
            let (pose, posed) = &rig.poses[wp.min(rig.poses.len() - 1)];
            let skinning = skinning_matrices(&rig.rest, posed);
            let stem = format!("{tag}_{}_{pose}", case.name.split(" (").next().unwrap_or("").replace(' ', "-"));
            write_obj(&out_dir.join(format!("{stem}_vanilla.obj")), &case.parts, &skinning);
            write_obj(&out_dir.join(format!("{stem}_smooth.obj")), &smoothed[ci], &skinning);
            println!("{tag}: {stem}");
        }
    }
    let text = format!("{summary}\n## Per case (worst over poses)\n\n{table}\n## Failures\n\n{failures}\n## Guard (seams reverted to the original weights)\n\n{guard_lines}\n## Per pose\n\n{lines}");
    std::fs::write(out_dir.join("metrics.txt"), &text).unwrap();
    println!("{summary}{failures}wrote {}", out_dir.display());
}

/// `SMOOTH_PROBE=<joint name>`: per part near that joint, vanilla -> smooth
/// weight (%) on the joint's subtree along its parent bone in 2 cm bins (`t`
/// in cm, > 0 past the pivot; `r` max radius), and the seam links between them.
fn probe(case: &Case, rig: &Rig, smooth: &[SkinSurface], seams: &[SeamLink], joint: &str) {
    let Some(j) = rig.names.iter().position(|n| n == joint) else { return };
    let Some(parent) = rig.parents[j] else { return };
    let pivot = Vec3::from(rig.rest[j].translation);
    let axis = (pivot - Vec3::from(rig.rest[parent].translation)).normalize();
    let sub: Vec<bool> = (0..rig.names.len()).map(|k| std::iter::successors(Some(k), |&x| rig.parents[x]).any(|x| x == j)).collect();
    let on = |s: &sf_model::SkinVertex| s.joints.iter().zip(s.weights).filter(|(jj, _)| sub[usize::from(**jj)]).map(|x| x.1).sum::<f32>();
    let t = |p: Vec3| (p - pivot).dot(axis);
    let r = |p: Vec3| (p - pivot).reject_from(axis).length();
    let near: Vec<usize> = (0..case.parts.len()).filter(|&i| case.parts[i].positions.iter().any(|p| p.distance(pivot) < 0.15)).collect();
    println!("probe {} at {joint}:", case.name);
    for &i in &near {
        let part = &case.parts[i];
        let dom = part.skin.first().map_or(0, |s| usize::from(s.joints[0]));
        let mut bins: std::collections::BTreeMap<i32, (usize, f32, f32, f32)> = std::collections::BTreeMap::new();
        for (v, p) in part.positions.iter().enumerate() {
            let e = bins.entry((t(*p) / 0.02).floor() as i32).or_default();
            e.0 += 1;
            e.1 += on(&part.skin[v]);
            e.2 += on(&smooth[i].skin[v]);
            e.3 = e.3.max(r(*p));
        }
        let row: Vec<String> = bins.iter().map(|(b, (n, v, s, rr))| format!("{}:{:.0}->{:.0}(r{:.0})", b * 2, v / *n as f32 * 100.0, s / *n as f32 * 100.0, rr * 100.0)).collect();
        println!("  part {i} {} [{}] {}", short(&case.ids[i]), rig.names[dom], row.join(" "));
    }
    for l in seams.iter().filter(|l| near.contains(&(l.a.0 as usize)) && near.contains(&(l.b.0 as usize))) {
        let (pa, pb) = (case.parts[l.a.0 as usize].positions[l.a.1 as usize], case.parts[l.b.0 as usize].positions[l.b.1 as usize]);
        let w = |set: &[SkinSurface], (p, v): (u32, u32)| on(&set[p as usize].skin[v as usize]) * 100.0;
        println!(
            "  link {}->{} inside {} a t{:.1} r{:.1} w{:.0}->{:.0} | b t{:.1} r{:.1} w{:.0}->{:.0}",
            l.a.0,
            l.b.0,
            l.inside,
            t(pa) * 100.0,
            r(pa) * 100.0,
            w(&case.parts, l.a),
            w(smooth, l.a),
            t(pb) * 100.0,
            r(pb) * 100.0,
            w(&case.parts, l.b),
            w(smooth, l.b)
        );
    }
}

fn item_exists(db: &RecordDb, id: &str) -> bool {
    let id = RecordId::new(id);
    db.get::<Armor>(&id).is_some() || db.get::<Clothing>(&id).is_some()
}

/// `b_n_dark elf_m_knee#left_lower_leg` -> `knee#left_lower_leg`.
fn short(id: &str) -> String {
    id.split_once('#').map_or_else(|| id.to_owned(), |(head, slot)| format!("{}#{slot}", head.rsplit('_').next().unwrap_or(head)))
}

/// The visible parts of `npc` wearing `worn` (hair left out: a transfer
/// target, not a segment), on the recipe's skeleton (loaded and posed once).
fn load_case(
    a: &MwActors,
    vfs: &Vfs,
    cooked: &mut HashMap<String, Cooked>,
    rigs: &mut HashMap<String, Rig>,
    name: String,
    npc: &str,
    worn: Vec<RecordId>,
) -> Option<Case> {
    let recipe = a.recipe("NPC_", &RecordId::new(npc), &CharacterState { worn: Some(worn), held: None }).ok()?;
    let skeleton = mw_anim::rig::skeleton(&mw_nif::load(&vfs.read(&recipe.skeleton).ok()?).ok()?).ok()?;
    let rest = skeleton.rest_model();
    let joint = |n: &str| skeleton.find(n).and_then(|j| u16::try_from(j).ok());
    let outfit = resolve_recipe(a, a.slots(), &recipe, View::ThirdPerson).ok()?;
    let hair = recipe.hair.as_ref().map(RecordId::as_str);
    let (mut ids, mut parts) = (Vec::new(), Vec::new());
    for (_, p) in &outfit.parts {
        if Some(p.as_str()) == hair {
            continue;
        }
        let part = a.part(p)?;
        let path = part.model.as_str().to_owned();
        if !cooked.contains_key(&path) {
            let model = mw_bake::NifImporter.import(&path, &vfs.read(&part.model).ok()?).ok()?;
            cooked.insert(path.clone(), sf_model::cook(&model, &CookSettings::default()).ok()?);
        }
        let skinned = match (&part.kind, &cooked[&path]) {
            (PartKind::Rigid { joint, offset }, c) => CookedSkinned::rigid(c, joint, *offset).ok()?,
            (PartKind::Skinned, Cooked::Skinned(s)) => Some(s.clone()),
            _ => None,
        };
        let Some(s) = skinned else { continue };
        let Ok(surface) = SkinSurface::from_cooked(&s, 0, Some(part), joint, &rest) else {
            println!("{name}: {} skipped (joint not in skeleton)", p.as_str());
            continue;
        };
        if !surface.triangles.is_empty() {
            ids.push(p.as_str().to_owned());
            parts.push(surface);
        }
    }
    let key = recipe.skeleton.as_str().to_owned();
    if !rigs.contains_key(&key) {
        let feet: Vec<Vec3> = ids.iter().zip(&parts).filter(|(id, _)| id.contains("foot")).flat_map(|(_, p)| p.positions.iter().copied()).collect();
        let foot_joints: Vec<Vec3> = ["Bip01 L Foot", "Bip01 R Foot"].iter().filter_map(|n| skeleton.find(n)).map(|j| Vec3::from(rest[j].translation)).collect();
        if feet.is_empty() || foot_joints.is_empty() {
            println!("{name}: no feet to orient the poses, skipped");
            return None;
        }
        let toward_toes = feet.iter().sum::<Vec3>() / feet.len() as f32 - foot_joints.iter().sum::<Vec3>() / foot_joints.len() as f32;
        let kf = clip_path(&key, vfs);
        let poses = poses(&skeleton, vfs, kf.as_deref(), toward_toes);
        println!("rig {key}: clip {kf:?}, poses {:?}", poses.iter().map(|p| p.0.as_str()).collect::<Vec<_>>());
        rigs.insert(key.clone(), Rig { names: skeleton.names().to_vec(), rest: rest.clone(), parents: (0..skeleton.len()).map(|j| skeleton.parent(j)).collect(), poses });
    }
    Some(Case { name, skeleton: key, ids, parts })
}

/// `meshes/base_anim_female.nif` -> `meshes/xbase_anim_female.kf`, falling back to the male clips.
fn clip_path(skeleton: &str, vfs: &Vfs) -> Option<String> {
    let (dir, file) = skeleton.rsplit_once('/')?;
    let own = format!("{dir}/x{}", file.replace(".nif", ".kf"));
    [own, "meshes/xbase_anim.kf".to_owned()].into_iter().find(|p| VfsPath::new(p).is_ok_and(|v| vfs.read(&v).is_ok()))
}

/// Seam opening (per link, `sf_character::seam_openings`: the metric the
/// guard uses) and pokes of one set in one pose.
fn measure(set: &[SkinSurface], seams: &[SeamLink], pairs: &[(usize, usize)], skinning: &[Affine3A]) -> PoseStats {
    let posed: Vec<Vec<Vec3>> = set.iter().map(|s| s.posed(skinning)).collect();
    let mut out = PoseStats { openings: seam_openings(set, seams, skinning), ..PoseStats::default() };
    let mut sorted: Vec<(f32, SeamLink)> = out.openings.iter().copied().zip(seams.iter().copied()).collect();
    sorted.sort_by(|x, y| x.0.total_cmp(&y.0));
    if let Some(last) = sorted.last() {
        out.open_max = last.0;
        out.worst_link = Some(last.1);
        out.open_median = sorted[sorted.len() / 2].0;
    }
    for &(x, y) in pairs {
        let depth = penetration(&set[x].positions, &posed[x], (&set[y].positions, &set[y].triangles), (&posed[y], &set[y].triangles), 0.05);
        let n = depth.iter().filter(|d| **d > 0.001).count();
        out.pokes += n;
        let max = depth.iter().copied().fold(0.0, f32::max);
        if max > out.poke_max {
            out.poke_max = max;
            out.poke_pair = (x, y);
        }
    }
    out
}

/// Named model-space poses: elbows 90°, knees 90°, shoulders raised, and
/// walk and run frames at 25% and 75% of the cycle. Poses whose joints the
/// skeleton lacks are left out.
fn poses(skeleton: &Skeleton, vfs: &Vfs, kf: Option<&str>, toward_toes: Vec3) -> Vec<(String, Vec<Affine3A>)> {
    let rest = skeleton.rest_model();
    let at = |n: &str| skeleton.find(n).map(|j| Vec3::from(rest[j].translation));
    let (Some(head), Some(pelvis)) = (at("Bip01 Head"), at("Bip01 Pelvis")) else { return Vec::new() };
    let up = (head - pelvis).normalize();
    let forward = toward_toes.reject_from(up).normalize();
    // Rotates `joint`'s subtree about its pivot by `angle`, around `axis` or
    // the axis perpendicular to its bone and `forward`, in the direction
    // that moves `tip` along `wanted`.
    let bend = |pose: &mut Vec<Affine3A>, joint: &str, tip: &str, axis: Option<Vec3>, angle: f32, wanted: Vec3| -> Option<()> {
        let j = skeleton.find(joint)?;
        let pivot = Vec3::from(pose[j].translation);
        let tip_p = Vec3::from(pose[skeleton.find(tip)?].translation);
        let axis = axis.unwrap_or_else(|| (tip_p - pivot).cross(forward).normalize());
        let m = |x: f32| (Quat::from_axis_angle(axis, x) * (tip_p - pivot)).dot(wanted);
        let best = if m(angle) >= m(-angle) { angle } else { -angle };
        let r = Affine3A::from_translation(pivot) * Affine3A::from_quat(Quat::from_axis_angle(axis, best)) * Affine3A::from_translation(-pivot);
        for k in skeleton.subtree(j) {
            pose[k] = r * pose[k];
        }
        Some(())
    };
    let half = std::f32::consts::FRAC_PI_2;
    let mut out = Vec::new();
    let mut add = |name: &str, f: &dyn Fn(&mut Vec<Affine3A>, &str) -> Option<()>| {
        let mut pose = rest.clone();
        if ["R", "L"].iter().all(|s| f(&mut pose, s).is_some()) {
            out.push((name.to_owned(), pose));
        }
    };
    add("elbow90", &|p, s| bend(p, &format!("Bip01 {s} Forearm"), &format!("Bip01 {s} Hand"), None, half, forward));
    add("knee90", &|p, s| bend(p, &format!("Bip01 {s} Calf"), &format!("Bip01 {s} Foot"), None, half, -forward));
    add("shoulder", &|p, s| bend(p, &format!("Bip01 {s} UpperArm"), &format!("Bip01 {s} Hand"), Some(forward), 1.0, up));
    let Some(bytes) = kf.and_then(|k| VfsPath::new(k).ok()).and_then(|p| vfs.read(&p).ok()) else { return out };
    let Ok(anim) = mw_anim::Animation::from_kf(&bytes) else { return out };
    for (group_name, tag) in [("WalkForward", "walk"), ("RunForward", "run")] {
        let Some(group) = anim.groups().into_iter().find(|g| g.name.eq_ignore_ascii_case(group_name)) else { continue };
        let clip = anim.source_clip(&group);
        let bound = BoundSource::new(&clip, skeleton);
        for phase in [0.25, 0.75] {
            let mut local = vec![Trs::IDENTITY; skeleton.len()];
            bound.sample(clip.duration * phase, skeleton.rest(), &mut local);
            let mut model: Vec<Affine3A> = Vec::with_capacity(local.len());
            for (j, t) in local.iter().enumerate() {
                let m = t.to_affine();
                model.push(skeleton.parent(j).map_or(m, |p| model[p] * m));
            }
            // Root motion off: compare shapes, not where the body went.
            let root = model.first().copied().unwrap_or(Affine3A::IDENTITY);
            let fix = rest.first().copied().unwrap_or(Affine3A::IDENTITY) * root.inverse();
            out.push((format!("{tag}{}", if phase < 0.5 { 25 } else { 75 }), model.into_iter().map(|m| fix * m).collect()));
        }
    }
    out
}

fn write_obj(path: &std::path::Path, set: &[SkinSurface], skinning: &[Affine3A]) {
    let mut s = String::new();
    let mut base = 1;
    for (i, part) in set.iter().enumerate() {
        writeln!(s, "o part{i}").unwrap();
        for p in part.posed(skinning) {
            writeln!(s, "v {:.5} {:.5} {:.5}", p.x, p.y, p.z).unwrap();
        }
        for t in &part.triangles {
            writeln!(s, "f {} {} {}", t[0] + base, t[1] + base, t[2] + base).unwrap();
        }
        base += u32::try_from(part.positions.len()).unwrap();
    }
    std::fs::write(path, s).unwrap();
}

/// Rigid on one joint (a segmented piece), as `smooth_body` classifies parts.
fn is_rigid(s: &SkinSurface) -> bool {
    let joint = |v: &sf_model::SkinVertex| v.weights.iter().position(|w| *w >= 0.999).map(|k| v.joints[k]);
    let first = s.skin.first().and_then(joint);
    first.is_some() && s.skin.iter().all(|v| joint(v) == first)
}
