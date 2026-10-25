//! Every `CREA` model's geometry, from the NIF to what the game shows
//! (nif-runtime.md "Actor parts"): each triangle shape of the source file
//! (visible or not) against the cooked rig (`mesh.rig`, LOD0) and against
//! what stays shown in the idle pose, where the effect curves of the clip
//! the creature idles with hide joints (`actor_fx`) or fade materials.
//! Clips come from the files the bake layers for the creature
//! (`mw_graphs::creature_animations`: its own, then `xbase_anim.kf` for
//! biped creatures), and a curve no clip carries reads 0, as the graph
//! outputs it. Skips (passes) without a Morrowind install. `-- --nocapture`
//! prints the census.
#![allow(clippy::too_many_lines)]

use std::collections::{BTreeMap, BTreeSet, HashMap};

use mw_anim::effects::{self, Channel, Effect};
use mw_install::MorrowindInstall;
use mw_nif::anim::Target;
use sf_anim::{AnimImporter, AnimSource};
use sf_model::{CookSettings, Cooked};
use sf_records::RecordDb;
use sf_vfs::{Vfs, VfsPath};
use tes3::esp::Creature;

/// One file's clips as the bake imports them (`.kf` with its `x<model>.nif` companion).
fn import(vfs: &Vfs, path: &str) -> Option<AnimSource> {
    let read = |p: &str| VfsPath::new(p).ok().and_then(|p| vfs.read(&p).ok());
    let bytes = read(path)?;
    if std::path::Path::new(path).extension().is_some_and(|e| e.eq_ignore_ascii_case("kf")) {
        let companion = path.strip_suffix(".kf").map(|s| format!("{s}.nif"));
        let companion = companion.and_then(|c| read(&c));
        mw_anim::KfAnimImporter.import_with(path, &bytes, companion.as_deref()).ok()
    } else {
        mw_anim::NifAnimImporter.import(path, &bytes).ok()
    }
}

#[test]
fn every_creature_shows_all_its_geometry_when_idle() {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return;
    };
    let vfs = mw_vfs::mount_install(&install, Vfs::builder()).unwrap().vfs;
    let db: RecordDb = mw_records::load(&vfs, &install.game_files()).unwrap().db;
    let exists = |p: &str| VfsPath::new(p).is_ok_and(|p| vfs.contains(&p));

    // Unique models: the records using them, and whether any is biped.
    let mut models: BTreeMap<String, (Vec<String>, bool, String)> = BTreeMap::new();
    for (id, crea) in db.iter::<Creature>() {
        let Some(path) = mw_bake::model_path(&crea.mesh) else { continue };
        let biped = mw_graphs::creature_template(u32::from(crea.creature_flags.bits())) == mw_graphs::Template::Humanoid;
        let entry = models.entry(path).or_insert_with(|| (Vec::new(), false, crea.mesh.clone()));
        entry.0.push(id.as_str().to_owned());
        entry.1 |= biped;
    }

    let mut clip_cache: HashMap<String, Option<AnimSource>> = HashMap::new();
    let (mut source_total, mut rig_total, mut shown_total) = (0usize, 0usize, 0usize);
    let mut dropped_by_reason: BTreeMap<&str, (usize, usize)> = BTreeMap::new();
    let mut rig_losses: Vec<String> = Vec::new();
    let mut hidden_at_idle: Vec<String> = Vec::new();
    let mut faded_at_idle: Vec<String> = Vec::new();
    let mut no_idle: Vec<String> = Vec::new();
    let mut absent_material: Vec<String> = Vec::new();
    let mut hidden_stems: BTreeMap<String, usize> = BTreeMap::new();
    let mut controller_shown: Vec<String> = Vec::new();
    for (path, (records, biped, mesh)) in &models {
        let Some(nif) = VfsPath::new(path).ok().and_then(|p| vfs.read(&p).ok()).and_then(|b| mw_nif::load(&b).ok()) else {
            continue;
        };
        // Source: every triangle shape, and why the cook leaves some out.
        let model = mw_bake::to_model(&nif);
        // Visibility controllers of the animation companion (`x<model>.nif`), by node name.
        let companion_shows: BTreeSet<String> = {
            let (dir, file) = path.rsplit_once('/').unwrap_or(("", path));
            let x = format!("{dir}/x{file}");
            VfsPath::new(&x).ok().and_then(|p| vfs.read(&p).ok()).and_then(|b| mw_nif::load(&b).ok()).map_or_else(BTreeSet::new, |x| {
                x.controllers
                    .iter()
                    .filter_map(|c| match &c.target {
                        Target::Visibility { node, keys } if keys.keys().iter().any(|k| k.1) => Some(x.nodes[*node].name.clone()),
                        _ => None,
                    })
                    .collect()
            })
        };
        let mut source = 0;
        let mut rendered: BTreeMap<&str, usize> = BTreeMap::new();
        for (i, node) in nif.nodes.iter().enumerate() {
            let Some(mesh) = node.mesh.map(|m| &nif.meshes[m]) else { continue };
            let tris = mesh.indices.len() / 3;
            source += tris;
            let reason = if mesh.collision {
                "collision"
            } else if nif.marker {
                "marker model"
            } else if !node.visible {
                // Can a visibility controller on it or an ancestor show it?
                let mut at = Some(i);
                let mut shown_by = None;
                while let Some(n) = at {
                    let shows = companion_shows.contains(&nif.nodes[n].name)
                        || nif.controllers.iter().any(|c| matches!(&c.target, Target::Visibility { node, keys } if *node == n && keys.keys().iter().any(|k| k.1)));
                    if shows {
                        shown_by = Some(nif.nodes[n].name.clone());
                    }
                    at = nif.nodes[n].parent;
                }
                let stem: String = node.name.trim_end_matches(|c: char| c.is_ascii_digit() || c == ' ').chars().take(16).collect();
                *hidden_stems.entry(stem).or_default() += tris;
                if let Some(by) = shown_by {
                    controller_shown.push(format!("{path}: {:?} ({tris} tris) shown by the visibility controller on {by:?}", node.name));
                    "hidden in the file, shown by a visibility controller"
                } else {
                    "hidden in the file"
                }
            } else {
                *rendered.entry(node.name.as_str()).or_default() += tris;
                continue;
            };
            let entry = dropped_by_reason.entry(reason).or_default();
            entry.0 += 1;
            entry.1 += tris;
        }
        source_total += source;

        // The cooked rig: every rendered shape keeps its triangles at LOD0.
        let Ok(Cooked::Skinned(rig)) = sf_model::cook_rig(&model, &CookSettings::default()) else {
            rig_losses.push(format!("{path}: no rig"));
            continue;
        };
        // Per part: triangles and the palette joints its vertices use.
        let mut parts: BTreeMap<u32, (usize, BTreeSet<u16>)> = BTreeMap::new();
        for i in 0..rig.model.lods[0].submeshes.len() {
            let d = rig.decode_submesh(0, i).unwrap();
            let entry = parts.entry(d.part).or_default();
            entry.0 += d.indices.len() / 3;
            entry.1.extend(d.skin.iter().flat_map(|s| s.joints.iter().zip(s.weights).filter(|(_, w)| *w > 0.0).map(|(j, _)| *j)));
        }
        let part_tris = |name: &str| -> usize {
            parts.iter().filter(|(p, _)| rig.parts[**p as usize].node_path.last().is_some_and(|n| n == name)).map(|(_, (t, _))| t).sum()
        };
        for (name, &tris) in &rendered {
            if part_tris(name) != tris {
                rig_losses.push(format!("{path}: {name:?} {tris} -> {} tris", part_tris(name)));
            }
        }
        let rig_tris: usize = parts.values().map(|(t, _)| t).sum();
        rig_total += rig_tris;

        // The files the bake layers for the creature, highest priority first;
        // a clip name plays from the first file that has it.
        let (_, files) = mw_graphs::creature_animations(mesh, *biped, exists);
        for f in &files {
            clip_cache.entry(f.clone()).or_insert_with(|| import(&vfs, f));
        }
        let sources: Vec<(&String, &AnimSource)> = files.iter().filter_map(|f| clip_cache[f].as_ref().map(|s| (f, s))).collect();
        let mut playable: BTreeMap<String, (&String, &sf_anim::SourceClip)> = BTreeMap::new();
        for (f, s) in &sources {
            for c in &s.clips {
                playable.entry(c.name.to_ascii_lowercase()).or_insert((f, c));
            }
        }
        // Every effect curve the creature's graph knows. A clip without one
        // reads 0 there (the graph's value for an absent curve): right for
        // `hide:` (0 shows), wrong for an absolute material value.
        let curves: BTreeSet<&str> = sources.iter().flat_map(|(_, s)| &s.clips).flat_map(|c| &c.curves).map(|c| c.name.as_str()).collect();
        for (f, clip) in playable.values() {
            for name in curves.iter().filter(|n| matches!(effects::parse(n), Some(Effect::Material { .. }))) {
                if !clip.curves.iter().any(|c| c.name == *name) && model.materials.iter().any(|m| name.contains(m.name.as_str())) {
                    absent_material.push(format!("{path}: {name} absent from {f}|{}", clip.name));
                }
            }
        }
        let Some((idle_file, idle)) = playable.get("idle").copied() else {
            no_idle.push(format!("{path} (files {files:?})"));
            shown_total += rig_tris;
            continue;
        };
        // Joints hidden throughout the idle, and materials its own keys fade out
        // throughout it (authored: the heart's rays flash in one window only).
        let samples = |name: &str| -> Vec<f32> {
            #[allow(clippy::cast_precision_loss)]
            let at = |i: usize| idle.duration * i as f32 / 15.0;
            idle.curves.iter().find(|c| c.name == name).map_or_else(|| vec![0.0; 16], |c| (0..16).map(|i| c.curve.sample(at(i))).collect())
        };
        let mut hidden_nodes = Vec::new();
        for name in &curves {
            match effects::parse(name) {
                Some(effect @ (Effect::Hide { node } | Effect::Show { node }))
                    if samples(name).into_iter().all(|v| effects::hidden(&effect, v) == Some(true)) =>
                {
                    hidden_nodes.push(node);
                }
                Some(Effect::Material { material, channel: Channel::Alpha })
                    if samples(name).into_iter().all(|a| a < 0.05) && model.materials.iter().any(|m| m.name.eq_ignore_ascii_case(material)) =>
                {
                    faded_at_idle.push(format!("{path}: {material} alpha 0 throughout {idle_file}|{}", idle.name));
                }
                _ => {}
            }
        }
        // A joint is hidden when it or an ancestor is (scale 0); a part when every joint it uses is.
        let hidden = |joint: &str| -> bool {
            let mut at = model.nodes.iter().position(|n| n.name == joint);
            while let Some(i) = at {
                if hidden_nodes.contains(&model.nodes[i].name.as_str()) {
                    return true;
                }
                at = model.nodes[i].parent.map(|p| p as usize);
            }
            false
        };
        let mut shown = rig_tris;
        for (p, (tris, joints)) in &parts {
            if !joints.is_empty() && joints.iter().all(|&j| hidden(&rig.joints[usize::from(j)].name)) {
                shown -= tris;
                let name = rig.parts[*p as usize].node_path.last().unwrap();
                hidden_at_idle.push(format!("{path} ({records:?}): {name:?} ({tris} tris) hidden in {idle_file}|{}", idle.name));
            }
        }
        shown_total += shown;
    }

    println!("CREA models: {}; triangles: source {source_total}, cooked rig {rig_total}, shown at idle {shown_total}", models.len());
    for (reason, (shapes, tris)) in &dropped_by_reason {
        println!("  left out by the cook ({reason}): {shapes} shapes, {tris} tris");
    }
    let mut stems: Vec<(&String, &usize)> = hidden_stems.iter().collect();
    stems.sort_by_key(|(_, t)| std::cmp::Reverse(**t));
    println!("  hidden shapes by name (tris): {:?}", &stems[..stems.len().min(16)]);
    println!("  hidden shapes a visibility controller shows (dropped: missing while shown): {}", controller_shown.len());
    for l in &controller_shown {
        println!("    {l}");
    }
    println!("rig losses: {}", rig_losses.len());
    for l in &rig_losses {
        println!("    {l}");
    }
    println!("parts hidden at idle: {}", hidden_at_idle.len());
    for l in &hidden_at_idle {
        println!("    {l}");
    }
    println!("materials faded out throughout the idle by their own keys (authored flashes): {}", faded_at_idle.len());
    for l in &faded_at_idle {
        println!("    {l}");
    }
    println!("material curves absent from a playable clip (read 0 there): {}", absent_material.len());
    for l in absent_material.iter().take(20) {
        println!("    {l}");
    }
    println!("no idle clip: {no_idle:?}");
    assert!(models.len() > 80, "install read");
    assert!(rig_losses.is_empty(), "rendered shapes the rig cook loses: {rig_losses:?}");
    assert!(hidden_at_idle.is_empty(), "parts the idle pose hides: {hidden_at_idle:?}");
    assert!(absent_material.is_empty(), "absolute material curves some clips lack: {absent_material:?}");
}
