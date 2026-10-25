//! Smooth bodies prototype on real Morrowind data (superfargoth
//! `docs/specs/smooth-bodies.md`): one race's naked body (skin parts plus
//! rigid limb parts on `base_anim.nif`) welded and re-weighted with
//! `sf_character::smooth_body`, then an armor set re-weighted from it with
//! `transfer_weights`. Measures seam gaps and armor penetration in a few
//! poses, before and after, and writes posed OBJs.
//!
//! `cargo run --release -p mw_actors --example smooth_body -- [npc id] [out dir]`
//! (defaults: `nileno dorvayn`, a male dark elf; `../shots/smoothbody`).

use std::collections::HashMap;
use std::fmt::Write as _;
use std::sync::Arc;
use std::time::Instant;

use glam::{Affine3A, Quat, Vec3};
use mw_actors::MwActors;
use mw_install::MorrowindInstall;
use sf_anim::{BoundSource, Skeleton, Trs};
use sf_character::{
    CharacterSource, CharacterState, PartKind, PartLookup, SkinSurface, SmoothSettings, TransferSettings, View, find_seams, penetration,
    resolve_recipe, skinning_matrices, smooth_body, transfer_weights,
};
use sf_model::{CookSettings, Cooked, CookedSkinned, ModelImporter};
use sf_records::RecordId;
use sf_vfs::{Vfs, VfsPath};

/// The armor set (item ids): iron cuirass, greaves and both pauldrons.
const ARMOR: [&str; 4] = ["iron_cuirass", "iron_greaves", "iron_pauldron_left", "iron_pauldron_right"];

struct Part {
    id: String,
    surface: SkinSurface,
}

#[allow(clippy::too_many_lines)] // a linear measurement script
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let candidates = ["sovor trandel", "dralval andrano", "nelos onmar", "ilver tels", "galos mathendis", "fargoth"];
    let out_dir = std::path::PathBuf::from(args.get(1).map_or("../shots/smoothbody", String::as_str));
    std::fs::create_dir_all(&out_dir).unwrap();
    let (install, _) = MorrowindInstall::detect(None).expect("Morrowind install");
    let vfs = Arc::new(mw_vfs::mount_install(&install, Vfs::builder()).unwrap().vfs);
    let content = mw_records::load(&vfs, &install.game_files()).unwrap();
    let a = MwActors::new(Arc::new(content.db), Arc::clone(&vfs));

    let naked_state = CharacterState { worn: Some(Vec::new()), held: None };
    let npc = args.first().map_or_else(
        || {
            candidates
                .into_iter()
                .find(|id| a.recipe("NPC_", &RecordId::new(id), &naked_state).is_ok_and(|r| r.base.iter().any(|p| p.as_str().contains("dark elf_m_neck"))))
                .expect("a male dark elf")
        },
        String::as_str,
    );
    let recipe = a.recipe("NPC_", &RecordId::new(npc), &naked_state).unwrap();
    println!("{npc}: skeleton {}, base {:?}", recipe.skeleton.as_str(), recipe.base.iter().map(RecordId::as_str).collect::<Vec<_>>());
    let skeleton = mw_anim::rig::skeleton(&mw_nif::load(&vfs.read(&recipe.skeleton).unwrap()).unwrap()).unwrap();
    let rest = skeleton.rest_model();
    let joint = |n: &str| skeleton.find(n).and_then(|j| u16::try_from(j).ok());

    let mut cooked: HashMap<String, Cooked> = HashMap::new();
    let mut load = |outfit: &sf_character::Outfit| -> Vec<Part> {
        let mut out = Vec::new();
        for (_, p) in &outfit.parts {
            let part = a.part(p).unwrap();
            let path = part.model.as_str().to_owned();
            let c = cooked.entry(path.clone()).or_insert_with(|| {
                let model = mw_bake::NifImporter.import(&path, &vfs.read(&part.model).unwrap()).unwrap();
                sf_model::cook(&model, &CookSettings::default()).unwrap()
            });
            let skinned = match (&part.kind, &*c) {
                (PartKind::Rigid { joint, offset }, c) => CookedSkinned::rigid(c, joint, *offset).unwrap(),
                (PartKind::Skinned, Cooked::Skinned(s)) => Some(s.clone()),
                _ => None,
            };
            let Some(s) = skinned else { continue };
            let surface = SkinSurface::from_cooked(&s, 0, Some(part), joint, &rest).unwrap();
            if !surface.triangles.is_empty() {
                out.push(Part { id: p.as_str().to_owned(), surface });
            }
        }
        out
    };
    let naked = resolve_recipe(&a, a.slots(), &recipe, View::ThirdPerson).unwrap();
    let hair = recipe.hair.as_ref().map(|h| h.as_str().to_owned());
    let mut body_vanilla = load(&naked);
    let hair_part = body_vanilla.iter().position(|p| Some(&p.id) == hair.as_ref()).map(|i| body_vanilla.remove(i));
    let dressed_recipe =
        a.recipe("NPC_", &RecordId::new(npc), &CharacterState { worn: Some(ARMOR.iter().map(|w| RecordId::new(w)).collect()), held: None }).unwrap();
    let dressed = load(&resolve_recipe(&a, a.slots(), &dressed_recipe, View::ThirdPerson).unwrap());
    let body_ids: Vec<&str> = body_vanilla.iter().map(|p| p.id.as_str()).collect();
    let (min, max) = body_vanilla.iter().flat_map(|p| &p.surface.positions).fold((Vec3::MAX, Vec3::MIN), |(lo, hi), p| (lo.min(*p), hi.max(*p)));
    let vertices: usize = body_vanilla.iter().map(|p| p.surface.positions.len()).sum();
    println!("body: {} parts, {vertices} vertices, bounds {min:.3} .. {max:.3}", body_vanilla.len());
    for p in &body_vanilla {
        let joints: std::collections::BTreeSet<&str> =
            p.surface.skin.iter().flat_map(|s| s.joints.iter().zip(s.weights).filter(|(_, w)| *w > 0.0).map(|(j, _)| skeleton.names()[usize::from(*j)].as_str())).collect();
        println!("  {} ({} v): {joints:?}", p.id, p.surface.positions.len());
    }

    for n in ["Left Knee", "Left Upper Leg", "Left Ankle", "Left Forearm"] {
        let mut chain = vec![n.to_owned()];
        let mut j = skeleton.find(n).unwrap();
        while let Some(p) = skeleton.parent(j) {
            chain.push(skeleton.names()[p].clone());
            j = p;
            if chain.len() > 4 { break; }
        }
        println!("  chain {chain:?}");
    }
    // Diagnostic: how adjacent parts meet at rest (abut, gap or overlap).
    for (i, pa) in body_vanilla.iter().enumerate() {
        for pb in &body_vanilla[i + 1..] {
            let d: Vec<f32> = pa.surface.positions.iter().map(|p| pb.surface.positions.iter().map(|q| p.distance(*q)).fold(f32::MAX, f32::min)).collect();
            let near = |r: f32| d.iter().filter(|x| **x <= r).count();
            if near(0.03) > 0 {
                println!(
                    "  meet {} / {}: min {:.1} mm, within 1/5/15/30 mm: {}/{}/{}/{}",
                    pa.id,
                    pb.id,
                    d.iter().copied().fold(f32::MAX, f32::min) * 1e3,
                    near(0.001),
                    near(0.005),
                    near(0.015),
                    near(0.03)
                );
            }
        }
    }

    // Smooth the naked body (head included, hair is a transfer target like clothing).
    let settings = SmoothSettings::default();
    let mut body_smooth: Vec<SkinSurface> = body_vanilla.iter().map(|p| p.surface.clone()).collect();
    let t = Instant::now();
    let parents: Vec<Option<usize>> = (0..skeleton.len()).map(|j| skeleton.parent(j)).collect();
    let report = smooth_body(&mut body_smooth, &parents, &settings);
    let smooth_time = t.elapsed();
    println!(
        "smooth_body: {} seam links ({} linked, {} welded vertices), {} band vertices, {} sweeps (residual {:.1e}), {:.0} ms (indicative)",
        report.seams.len(),
        report.linked,
        report.welded,
        report.band,
        report.iterations,
        report.residual,
        smooth_time.as_secs_f64() * 1e3
    );
    let rest_gaps: Vec<f32> = report.seams.iter().map(|l| pos(&body_vanilla_surfaces(&body_vanilla), l.a).distance(pos(&body_vanilla_surfaces(&body_vanilla), l.b))).collect();
    println!("rest seam pair distances: {}", stats(&rest_gaps));

    // Armor (and hair): vanilla, rigid on the smooth body, transferred from the smooth body.
    let is_body = |id: &str| body_ids.contains(&id);
    let mut armor_vanilla: Vec<Part> = dressed.into_iter().filter(|p| !is_body(&p.id)).collect();
    armor_vanilla.retain(|p| Some(&p.id) != hair.as_ref());
    let dressed_ids: Vec<String> = resolve_recipe(&a, a.slots(), &dressed_recipe, View::ThirdPerson).unwrap().parts.iter().map(|(_, p)| p.as_str().to_owned()).collect();
    let visible: Vec<usize> = (0..body_vanilla.len()).filter(|&i| dressed_ids.contains(&body_vanilla[i].id)).collect();
    println!("armor: {:?}; visible body parts when dressed: {:?}", armor_vanilla.iter().map(|p| &p.id).collect::<Vec<_>>(), visible.iter().map(|&i| &body_vanilla[i].id).collect::<Vec<_>>());
    let mut armor_smooth: Vec<SkinSurface> = armor_vanilla.iter().map(|p| p.surface.clone()).collect();
    let t = Instant::now();
    let tr = transfer_weights(&body_smooth, &mut armor_smooth, &TransferSettings::default());
    let transfer_time = t.elapsed();
    println!(
        "transfer_weights (armor): {} matched, {} inpainted, {} kept, {} sweeps, {:.0} ms (indicative)",
        tr.matched,
        tr.inpainted,
        tr.kept,
        tr.iterations,
        transfer_time.as_secs_f64() * 1e3
    );
    let mut hair_smooth: Vec<SkinSurface> = hair_part.iter().map(|p| p.surface.clone()).collect();
    let hr = transfer_weights(&body_smooth, &mut hair_smooth, &TransferSettings::default());
    println!("transfer_weights (hair): {} matched, {} inpainted, {} kept", hr.matched, hr.inpainted, hr.kept);

    // Dressed sets: visible body parts + armor.
    let vanilla_body: Vec<SkinSurface> = body_vanilla_surfaces(&body_vanilla);
    let dressed_of = |body: &[SkinSurface], armor: &[SkinSurface]| -> Vec<SkinSurface> { visible.iter().map(|&i| body[i].clone()).chain(armor.iter().cloned()).collect() };
    let armor_vanilla_s: Vec<SkinSurface> = armor_vanilla.iter().map(|p| p.surface.clone()).collect();
    let mut outfit_smooth = dressed_of(&vanilla_body, &armor_vanilla_s);
    let t = Instant::now();
    let outfit_report = smooth_body(&mut outfit_smooth, &parents, &settings);
    println!("smooth_body (dressed outfit as one segmented set): {} links, {} band vertices, {:.0} ms", outfit_report.seams.len(), outfit_report.band, t.elapsed().as_secs_f64() * 1e3);
    let configs: [(&str, Vec<SkinSurface>); 4] = [
        ("vanilla", dressed_of(&vanilla_body, &armor_vanilla_s)),
        ("smoothbody-rigidarmor", dressed_of(&body_smooth, &armor_vanilla_s)),
        ("smooth-transfer", dressed_of(&body_smooth, &armor_smooth)),
        ("smooth-outfit", outfit_smooth),
    ];
    let dressed_seams = find_seams(&configs[0].1, &parents, &settings);
    let armor_tris = |set: &[SkinSurface], positions: &[Vec<Vec3>]| -> (Vec<Vec3>, Vec<[u32; 3]>) {
        let mut p = Vec::new();
        let mut t = Vec::new();
        for (s, pos) in set.iter().zip(positions).skip(visible.len()) {
            let base = u32::try_from(p.len()).unwrap();
            p.extend(pos);
            t.extend(s.triangles.iter().map(|x| x.map(|i| i + base)));
        }
        (p, t)
    };

    let mut summary = String::new();
    // Forward: from the foot joints to the feet's mesh centroid (toes point forward).
    let feet: Vec<Vec3> = body_vanilla.iter().filter(|p| p.id.contains("foot")).flat_map(|p| p.surface.positions.iter().copied()).collect();
    let foot_joints = ["Bip01 L Foot", "Bip01 R Foot"].map(|n| Vec3::from(rest[skeleton.find(n).unwrap()].translation));
    #[allow(clippy::cast_precision_loss)]
    let toward_toes = feet.iter().sum::<Vec3>() / feet.len() as f32 - (foot_joints[0] + foot_joints[1]) * 0.5;
    for (pose, posed) in poses(&skeleton, &vfs, toward_toes) {
        let skinning = skinning_matrices(&rest, &posed);
        let identity = skinning_matrices(&rest, &rest);
        // Naked body seams.
        for (name, set) in [("vanilla", &vanilla_body), ("smooth", &body_smooth)] {
            let posed_set: Vec<Vec<Vec3>> = set.iter().map(|s| s.posed(&skinning)).collect();
            let rest_set: Vec<Vec<Vec3>> = set.iter().map(|s| s.posed(&identity)).collect();
            let gaps = openings(&report.seams, &rest_set, &posed_set);
            let line = format!("{pose:<9} naked   {name:<22} seam opening {}", stats(&gaps));
            println!("{line}");
            writeln!(summary, "{line}").unwrap();
            let mut all = set.clone();
            if name == "smooth" {
                all.extend(hair_smooth.iter().cloned());
            } else {
                all.extend(hair_part.iter().map(|p| p.surface.clone()));
            }
            write_obj(&out_dir.join(format!("naked_{pose}_{name}.obj")), &all, &skinning);
        }
        // Dressed: seams (body-armor and armor-armor) and penetration.
        for (name, set) in &configs {
            let posed_set: Vec<Vec<Vec3>> = set.iter().map(|s| s.posed(&skinning)).collect();
            let rest_set: Vec<Vec<Vec3>> = set.iter().map(|s| s.posed(&identity)).collect();
            let gaps = openings(&dressed_seams, &rest_set, &posed_set);
            let (ap, at) = armor_tris(set, &posed_set);
            let (rp, rt) = armor_tris(set, &rest_set);
            // Visible body through armor.
            let vis_rest: Vec<Vec3> = rest_set[..visible.len()].concat();
            let vis_posed: Vec<Vec3> = posed_set[..visible.len()].concat();
            let pen_visible = penetration(&vis_rest, &vis_posed, (&rp, &rt), (&ap, &at), 0.05);
            // Whole naked body (hidden parts too: content that doesn't hide them) through armor.
            let body: &[SkinSurface] = if *name == "vanilla" { &vanilla_body } else { &body_smooth };
            let b_rest: Vec<Vec3> = body.iter().flat_map(|s| s.posed(&identity)).collect();
            let b_posed: Vec<Vec3> = body.iter().flat_map(|s| s.posed(&skinning)).collect();
            let pen_body = penetration(&b_rest, &b_posed, (&rp, &rt), (&ap, &at), 0.05);
            let line = format!(
                "{pose:<9} dressed {name:<22} seam opening {} | visible-body poke {} | whole-body poke {}",
                stats(&gaps),
                poke(&pen_visible),
                poke(&pen_body)
            );
            println!("{line}");
            writeln!(summary, "{line}").unwrap();
            if *name != "smoothbody-rigidarmor" || pose == "elbow90" {
                write_obj(&out_dir.join(format!("dressed_{pose}_{name}.obj")), set, &skinning);
            }
        }
    }
    let size: usize = body_smooth.iter().chain(&armor_smooth).map(|s| s.positions.len()).sum::<usize>() * size_of::<sf_model::SkinVertex>();
    writeln!(
        summary,
        "timing (indicative, owner on machine): smooth_body {:.0} ms, transfer {:.0} ms; weight data {size} B (body + armor)",
        smooth_time.as_secs_f64() * 1e3,
        transfer_time.as_secs_f64() * 1e3
    )
    .unwrap();
    std::fs::write(out_dir.join("metrics.txt"), &summary).unwrap();
    println!("wrote {}", out_dir.display());
}

fn body_vanilla_surfaces(parts: &[Part]) -> Vec<SkinSurface> {
    parts.iter().map(|p| p.surface.clone()).collect()
}

fn pos(set: &[SkinSurface], (p, v): (u32, u32)) -> Vec3 {
    set[p as usize].positions[v as usize]
}

fn pos2(set: &[Vec<Vec3>], (p, v): (u32, u32)) -> Vec3 {
    set[p as usize][v as usize]
}

/// `max / p95 / mean` in millimeters.
fn stats(values: &[f32]) -> String {
    if values.is_empty() {
        return "n/a".into();
    }
    let mut v = values.to_vec();
    v.sort_by(f32::total_cmp);
    let p95 = v[(v.len() - 1) * 95 / 100];
    #[allow(clippy::cast_precision_loss)]
    let mean = v.iter().sum::<f32>() / v.len() as f32;
    format!("max {:6.1} p95 {:5.1} mean {:4.1} mm (n {})", v[v.len() - 1] * 1e3, p95 * 1e3, mean * 1e3, v.len())
}

fn poke(depths: &[f32]) -> String {
    let n = depths.iter().filter(|d| **d > 0.001).count();
    let max = depths.iter().copied().fold(0.0, f32::max);
    format!("{n:4} v > 1 mm, max {:5.1} mm", max * 1e3)
}

/// Named model-space poses: rest, elbows 90°, knees 90°, shoulders raised,
/// and a frame of the walk cycle from `xbase_anim.kf`.
fn poses(skeleton: &Skeleton, vfs: &Vfs, toward_toes: Vec3) -> Vec<(&'static str, Vec<Affine3A>)> {
    let rest = skeleton.rest_model();
    let at = |n: &str| Vec3::from(rest[skeleton.find(n).unwrap_or_else(|| panic!("joint {n}"))].translation);
    let up = (at("Bip01 Head") - at("Bip01 Pelvis")).normalize();
    let forward = toward_toes.reject_from(up).normalize();
    // Rotates `joint`'s subtree about its pivot by `angle` around an axis
    // perpendicular to its bone and `forward`, in the direction where `tip` moves along `wanted`.
    let bend = |pose: &mut Vec<Affine3A>, joint: &str, tip: &str, axis: Option<Vec3>, angle: f32, wanted: Vec3| {
        let j = skeleton.find(joint).unwrap();
        let pivot = Vec3::from(pose[j].translation);
        let tip_p = Vec3::from(pose[skeleton.find(tip).unwrap()].translation);
        let axis = axis.unwrap_or_else(|| (tip_p - pivot).cross(forward).normalize());
        let best = [angle, -angle]
            .into_iter()
            .max_by(|a, b| {
                let m = |x: f32| (Quat::from_axis_angle(axis, x) * (tip_p - pivot)).dot(wanted);
                m(*a).total_cmp(&m(*b))
            })
            .unwrap();
        let r = Affine3A::from_translation(pivot) * Affine3A::from_quat(Quat::from_axis_angle(axis, best)) * Affine3A::from_translation(-pivot);
        for k in skeleton.subtree(j) {
            pose[k] = r * pose[k];
        }
    };
    let half = std::f32::consts::FRAC_PI_2;
    let mut elbow = rest.clone();
    for s in ["R", "L"] {
        bend(&mut elbow, &format!("Bip01 {s} Forearm"), &format!("Bip01 {s} Hand"), None, half, forward);
    }
    let mut knee = rest.clone();
    for s in ["R", "L"] {
        bend(&mut knee, &format!("Bip01 {s} Calf"), &format!("Bip01 {s} Foot"), None, half, -forward);
    }
    let mut shoulder = rest.clone();
    for s in ["R", "L"] {
        bend(&mut shoulder, &format!("Bip01 {s} UpperArm"), &format!("Bip01 {s} Hand"), Some(forward), 1.0, up);
    }
    let mut out = vec![("rest", rest.clone()), ("elbow90", elbow), ("knee90", knee), ("shoulder", shoulder)];
    if let Ok(bytes) = vfs.read(&VfsPath::new("meshes/xbase_anim.kf").unwrap()) {
        let anim = mw_anim::Animation::from_kf(&bytes).unwrap();
        if let Some(group) = anim.groups().into_iter().find(|g| g.name.eq_ignore_ascii_case("WalkForward")) {
            let clip = anim.source_clip(&group);
            let bound = BoundSource::new(&clip, skeleton);
            let mut local = vec![Trs::IDENTITY; skeleton.len()];
            bound.sample(clip.duration * 0.25, skeleton.rest(), &mut local);
            let mut model: Vec<Affine3A> = Vec::with_capacity(local.len());
            for (j, t) in local.iter().enumerate() {
                let m = t.to_affine();
                model.push(skeleton.parent(j).map_or(m, |p| model[p] * m));
            }
            out.push(("walk", model));
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


/// How much each seam pair moved apart from its rest distance (0 if closer).
fn openings(seams: &[sf_character::SeamLink], rest: &[Vec<Vec3>], posed: &[Vec<Vec3>]) -> Vec<f32> {
    seams.iter().map(|l| (pos2(posed, l.a).distance(pos2(posed, l.b)) - pos2(rest, l.a).distance(pos2(rest, l.b))).max(0.0)).collect()
}
