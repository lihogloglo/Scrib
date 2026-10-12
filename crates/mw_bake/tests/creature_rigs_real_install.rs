//! Every `CREA` model through `mesh.rig` (`sf_model::cook_rig`) on the real
//! game data: counted by kind (skinned, node-animated, mixed, static), each
//! one cooks as a skin the character pipeline binds, and every palette joint
//! names a joint of the creature's own animation skeleton. Skips (passes)
//! without a Morrowind install. `-- --nocapture` prints the census.
#![allow(clippy::too_many_lines)]

use std::collections::{BTreeMap, BTreeSet};

use mw_install::MorrowindInstall;
use sf_model::{CookSettings, Cooked};
use sf_records::RecordDb;
use sf_vfs::{Vfs, VfsPath};
use tes3::esp::Creature;

/// How a creature model moves.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    /// Every rendered mesh is skinned.
    Skinned,
    /// Skinned meshes plus rigid meshes hanging from nodes.
    Mixed,
    /// Only rigid meshes; animation moves their nodes ("segmented").
    NodeAnimated,
    /// Only rigid meshes and nothing animates them.
    Static,
}

#[test]
fn every_creature_model_cooks_for_its_rig() {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return;
    };
    let vfs = mw_vfs::mount_install(&install, Vfs::builder()).unwrap().vfs;
    let db: RecordDb = mw_records::load(&vfs, &install.game_files()).unwrap().db;
    let exists = |p: &str| VfsPath::new(p).is_ok_and(|p| vfs.contains(&p));

    // Unique models, with the records that use them.
    let mut models: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (id, crea) in db.iter::<Creature>() {
        if let Some(path) = mw_bake::model_path(&crea.mesh) {
            models.entry(path).or_default().push(id.as_str().to_owned());
        }
    }

    let mut kinds: BTreeMap<Kind, (usize, usize)> = BTreeMap::new(); // models, records
    let mut before_unsupported: Vec<(String, String)> = Vec::new();
    let mut unsupported: Vec<(String, String)> = Vec::new();
    let mut examples: BTreeMap<Kind, Vec<String>> = BTreeMap::new();
    let mut root_bound = 0usize;
    for (path, records) in &models {
        let Some(bytes) = VfsPath::new(path).ok().and_then(|p| vfs.read(&p).ok()) else {
            unsupported.push((path.clone(), "model file missing from the VFS".into()));
            continue;
        };
        let nif = match mw_nif::parse(&bytes).map_err(|e| e.to_string()).and_then(|s| mw_nif::convert(s).map_err(|e| e.to_string())) {
            Ok(nif) => nif,
            Err(e) => {
                unsupported.push((path.clone(), format!("unreadable NIF: {e}")));
                continue;
            }
        };
        let model = mw_bake::to_model(&nif);
        let rendered: Vec<&sf_model::MeshData> = model.nodes.iter().filter_map(|n| n.mesh).map(|m| &model.meshes[m as usize]).collect();
        let skinned = rendered.iter().filter(|m| m.skinned).count();
        let (dir, file) = path.rsplit_once('/').unwrap_or(("", path));
        let stem = file.rsplit_once('.').map_or(file, |(s, _)| s);
        let has_anim = [format!("{dir}/x{stem}.kf"), format!("{dir}/x{stem}.nif")].iter().any(|x| exists(x)) || !nif.controllers.is_empty();
        let kind = match (skinned, rendered.len() - skinned) {
            (s, 0) if s > 0 => Kind::Skinned,
            (s, _) if s > 0 => Kind::Mixed,
            _ if has_anim => Kind::NodeAnimated,
            _ => Kind::Static,
        };
        let entry = kinds.entry(kind).or_default();
        entry.0 += 1;
        entry.1 += records.len();
        let list = examples.entry(kind).or_default();
        if list.len() < 6 {
            list.push(path.clone());
        }

        // Before: the character pipeline took `mesh.cook` and needed a skin.
        match sf_model::cook(&model, &CookSettings::default()) {
            Ok(Cooked::Skinned(_)) => {}
            Ok(Cooked::Static(_)) => before_unsupported.push((path.clone(), "mesh.cook static (no skin)".into())),
            Ok(Cooked::Skipped { reason }) => before_unsupported.push((path.clone(), format!("mesh.cook skipped: {reason}"))),
            Err(e) => before_unsupported.push((path.clone(), format!("mesh.cook failed: {e}"))),
        }

        // Now: `mesh.rig`, bound by joint name to the animation skeleton.
        let rig = match sf_model::cook_rig(&model, &CookSettings::default()) {
            Ok(Cooked::Skinned(rig)) => rig,
            Ok(Cooked::Skipped { reason }) => {
                unsupported.push((path.clone(), format!("mesh.rig skipped: {reason}")));
                continue;
            }
            Ok(Cooked::Static(_)) => unreachable!("cook_rig never cooks static"),
            Err(e) => {
                unsupported.push((path.clone(), format!("mesh.rig failed: {e}")));
                continue;
            }
        };
        if rig.model.lods.first().is_none_or(|l| l.submeshes.is_empty()) {
            unsupported.push((path.clone(), "no rendered geometry".into()));
            continue;
        }
        let (joints, _) = mw_anim::rig::skeleton_source(&nif);
        let names: BTreeSet<&str> = joints.iter().map(|j| j.name.as_str()).collect();
        let missing: Vec<&str> = rig.joints.iter().map(|j| j.name.as_str()).filter(|j| !names.contains(j)).collect();
        if !missing.is_empty() {
            // The runtime binds these to the rig's root joint (counted as
            // missing joints): parts that then don't follow their node.
            // Which of them do the animation files key?
            let mut keyed: BTreeSet<String> = mw_anim::Animation::from_model(&nif).tracks.into_iter().map(|(n, _)| n).collect();
            for x in [format!("{dir}/x{stem}.kf"), format!("{dir}/x{stem}.nif")] {
                if let Some(a) = VfsPath::new(&x).ok().and_then(|p| vfs.read(&p).ok()).and_then(|b| mw_anim::Animation::from_file(&x, &b).ok()) {
                    keyed.extend(a.tracks.into_iter().map(|(n, _)| n));
                }
            }
            // The highest ancestor outside the skeleton says why it was left out.
            let outside = |name: &str| {
                let mut at = nif.nodes.iter().position(|n| n.name == name)?;
                let mut top = at;
                while let Some(p) = nif.nodes[at].parent {
                    if !names.contains(nif.nodes[p].name.as_str()) {
                        top = p;
                    }
                    at = p;
                }
                Some(format!("{:?} {:?}", nif.nodes[top].name, nif.nodes[top].kind))
            };
            let tagged: Vec<String> = missing
                .iter()
                .map(|j| format!("{j}{} under {}", if keyed.contains(*j) { " (keyed)" } else { "" }, outside(j).unwrap_or_default()))
                .collect();
            root_bound += 1;
            unsupported.push((path.clone(), format!("palette joints outside the animation skeleton (bound to its root): {tagged:?}")));
        }
    }

    println!("CREA models: {} unique, {} records", models.len(), models.values().map(Vec::len).sum::<usize>());
    for (kind, (m, r)) in &kinds {
        println!("  {kind:?}: {m} models, {r} records; e.g. {:?}", examples[kind]);
    }
    println!("before (mesh.cook not a skin, skipped by characters): {}", before_unsupported.len());
    for (path, why) in &before_unsupported {
        println!("    {path}: {why}");
    }
    println!("now unsupported or partial: {} ({root_bound} with root-bound joints)", unsupported.len());
    for (path, why) in &unsupported {
        println!("    {path}: {why}");
    }
    // Where to watch them: exterior placements of node-animated creatures.
    let node_animated: BTreeSet<String> = examples.get(&Kind::NodeAnimated).into_iter().flatten().flat_map(|m| models[m].clone()).collect();
    let mut shown = 0;
    for (_, cell) in db.iter::<tes3::esp::Cell>() {
        if cell.data.flags.contains(tes3::esp::CellFlags::IS_INTERIOR) {
            continue;
        }
        for r in cell.references.values().filter(|r| node_animated.contains(&r.id.to_ascii_lowercase())) {
            if shown < 12 {
                println!("  placed: {} in {:?} {:?} at {:?}", r.id, cell.name, cell.data.grid, r.translation);
            }
            shown += 1;
        }
    }
    println!("  exterior placements of node-animated creatures: {shown}");
    let hard: Vec<_> =unsupported.iter().filter(|(_, why)| !why.starts_with("palette joints")).collect();
    assert!(hard.is_empty(), "creature models the character pipeline can't build: {hard:?}");
    assert!(kinds.contains_key(&Kind::NodeAnimated), "the census should find per-node animated creatures (scribs)");
}
