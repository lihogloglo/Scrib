//! Memory per cached outfit on real NPCs (character.md "Morph-bearing
//! parts"): every visible part cooked as the bake does, then assembled with
//! `sf_character::assemble`, against the old layout (morphing parts merged
//! per material, every model target dense over the merged mesh). Skips
//! (passes) when no Morrowind install is detected. Prints the numbers.

use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

use mw_actors::MwActors;
use mw_install::MorrowindInstall;
use sf_character::{
    AssembledOutfit, AssemblyPart, BEVY_JOINT_BYTES, BEVY_MORPH_BYTES, CharacterSource, CharacterState, PartKind, PartLookup, ShapeWeights, View,
    assemble, resolve_recipe,
};
use sf_model::{CookSettings, Cooked, CookedSkinned, ModelImporter};
use sf_records::RecordId;
use sf_vfs::Vfs;

/// The old layout's bytes: one mesh per material (morphing parts merged in),
/// carrying the most targets any of its parts had, densely.
fn before_bytes(outfit: &AssembledOutfit, targets_by_material: &BTreeMap<String, usize>) -> usize {
    let after = outfit.gpu_bytes();
    let mut per_material: BTreeMap<&str, usize> = BTreeMap::new();
    for m in outfit.meshes() {
        *per_material.entry(m.material.name.as_str()).or_default() += m.vertices.len();
    }
    let morphs: usize = per_material.iter().map(|(name, v)| v * targets_by_material.get(*name).copied().unwrap_or(0) * BEVY_MORPH_BYTES).sum();
    after.vertices + after.indices + morphs + per_material.len() * outfit.joints.len() * BEVY_JOINT_BYTES
}

#[test]
fn outfit_memory_before_and_after_on_real_npcs() {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return;
    };
    let vfs = Arc::new(mw_vfs::mount_install(&install, Vfs::builder()).unwrap().vfs);
    let content = mw_records::load(&vfs, &install.game_files()).unwrap();
    let a = MwActors::new(Arc::new(content.db), Arc::clone(&vfs));
    let mut cache: HashMap<String, Cooked> = HashMap::new();
    let mut checked = 0;
    for id in ["fargoth", "ajira", "caius cosades", "sellus gravius", "hasphat antabolis"] {
        let Ok(recipe) = a.recipe("NPC_", &RecordId::new(id), &CharacterState::default()) else { continue };
        let outfit = resolve_recipe(&a, a.slots(), &recipe, View::ThirdPerson).unwrap();
        let mut owned: Vec<(CookedSkinned, &sf_character::BodyPart)> = Vec::new();
        for (_, p) in &outfit.parts {
            let part = a.part(p).unwrap();
            let path = part.model.as_str().to_owned();
            let cooked = cache.entry(path.clone()).or_insert_with(|| {
                let model = mw_bake::NifImporter.import(&path, &vfs.read(&part.model).unwrap()).unwrap();
                sf_model::cook(&model, &CookSettings::default()).unwrap()
            });
            let skinned = match (&part.kind, &*cooked) {
                (PartKind::Rigid { joint, offset }, c) => CookedSkinned::rigid(c, joint, *offset).unwrap(),
                (PartKind::Skinned, Cooked::Skinned(s)) => Some(s.clone()),
                _ => None,
            };
            if let Some(s) = skinned {
                owned.push((s, part));
            }
        }
        let mut targets_by_material: BTreeMap<String, usize> = BTreeMap::new();
        for (s, _) in &owned {
            // v2 carried every model target on each morphing submesh.
            for (i, _) in s.streams[0].iter().enumerate().filter(|(_, st)| !st.targets.is_empty()) {
                let name = &s.model.materials[s.model.lods[0].submeshes[i].material as usize].name;
                let e = targets_by_material.entry(name.clone()).or_default();
                *e = (*e).max(s.morph_targets.len());
            }
        }
        let parts: Vec<AssemblyPart<'_>> = owned.iter().map(|(cooked, part)| AssemblyPart::new(cooked, Some(*part))).collect();
        for lod in [0, 2] {
            let assembled = assemble(&parts, lod, &ShapeWeights::default()).unwrap();
            let after = assembled.gpu_bytes();
            let before = if lod == 0 { before_bytes(&assembled, &targets_by_material) } else { 0 };
            let vertices: usize = assembled.meshes().map(|m| m.vertices.len()).sum();
            let morphed: Vec<String> =
                assembled.morphed.iter().map(|m| format!("{} v × {} targets", m.vertices.len(), m.morph_targets.len())).collect();
            eprintln!(
                "{id} LOD {lod}: {} parts, {vertices} vertices, {} merged + {} morphed meshes {morphed:?}, {} joints; after {} B (morphs {} B){}",
                owned.len(),
                assembled.merged.len(),
                assembled.morphed.len(),
                assembled.joints.len(),
                after.total(),
                after.morphs,
                if lod == 0 { format!(", before {before} B") } else { String::new() }
            );
            assert!(assembled.merged.iter().all(|m| m.morph_targets.is_empty()));
            if lod >= sf_model::RUNTIME_MORPH_LODS {
                assert!(assembled.morphed.is_empty(), "{id}: expression morphs past LOD 1");
            }
        }
        checked += 1;
    }
    eprintln!("{checked} NPCs measured");
}
