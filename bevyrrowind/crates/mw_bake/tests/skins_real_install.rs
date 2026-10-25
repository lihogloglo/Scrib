//! Skins and geometry morphs of vanilla actor models, NIF → `sf_model` →
//! `Cooked::Skinned`. Skips (passes) when no Morrowind install is detected.
//! Whole-install numbers: `cargo run --release -p mw_bake --example skin_report`.
#![allow(clippy::cast_precision_loss)]

use std::collections::HashSet;

use mw_install::MorrowindInstall;
use mw_nif::glam::Vec3;
use sf_model::{CookSettings, Cooked};
use sf_vfs::{Vfs, VfsPath};

fn load(vfs: &Vfs, path: &str) -> mw_nif::Model {
    let bytes = vfs.read(&VfsPath::new(path).unwrap()).unwrap_or_else(|e| panic!("{path}: {e}"));
    mw_nif::load(&bytes).unwrap_or_else(|e| panic!("{path}: {e}"))
}

struct Stats {
    skinned: usize,
    morphing: usize,
    morph_targets: usize,
    max_influences: usize,
    max_weight_error: f32,
    max_rest_error: f32,
}

/// Checks every rendered mesh's skin and morph data; returns the bone names of visible meshes.
fn check(path: &str, nif: &mw_nif::Model) -> (Stats, HashSet<String>) {
    let world = mw_nif::world_transforms(nif);
    let mut stats =
        Stats { skinned: 0, morphing: 0, morph_targets: 0, max_influences: 0, max_weight_error: 0.0, max_rest_error: 0.0 };
    let mut bones = HashSet::new();
    for (index, node) in nif.nodes.iter().enumerate() {
        let Some(mesh) = node.mesh.map(|m| &nif.meshes[m]) else { continue };
        if let Some(morph) = &mesh.morph {
            stats.morphing += 1;
            stats.morph_targets += morph.targets.len();
            assert!(morph.targets.iter().all(|t| t.deltas.len() == mesh.positions.len()), "{path}");
        }
        if !mesh.skinned {
            continue;
        }
        let skin = mesh.skin.as_deref().unwrap_or_else(|| panic!("{path}: {} has no skin data", node.name));
        stats.skinned += 1;
        assert_eq!(skin.positions.len(), mesh.positions.len(), "{path}");
        if node.visible {
            // Hidden shapes (creature "Tri Shadow" meshes) aren't cooked.
            bones.extend(skin.bones.iter().map(|b| nif.nodes[b.node].name.to_ascii_lowercase()));
        }
        for influences in &skin.influences {
            stats.max_influences = stats.max_influences.max(influences.len());
            let sum: f32 = influences.iter().map(|i| i.1).sum();
            stats.max_weight_error = stats.max_weight_error.max((sum - 1.0).abs());
        }
        for (p, baked) in skin.rest_positions(&world).iter().zip(&mesh.positions) {
            let baked = world[index].transform_point3(Vec3::from_array(*baked));
            stats.max_rest_error = stats.max_rest_error.max(p.distance(baked));
        }
    }
    (stats, bones)
}

fn cook(path: &str, nif: &mw_nif::Model) -> sf_model::CookedSkinned {
    let model = mw_bake::to_model(nif);
    match sf_model::cook(&model, &CookSettings::default()).unwrap_or_else(|e| panic!("{path}: {e}")) {
        Cooked::Skinned(cooked) => cooked,
        Cooked::Skipped { reason } => panic!("{path}: skipped: {reason}"),
        Cooked::Static(_) => panic!("{path}: cooked as static"),
    }
}

#[test]
#[allow(clippy::too_many_lines)]
fn vanilla_actor_models_cook_as_skinned() {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return;
    };
    let vfs = mw_vfs::mount_install(&install, Vfs::builder()).unwrap().vfs;
    let skeleton: HashSet<String> =
        load(&vfs, "meshes/base_anim.nif").nodes.iter().map(|n| n.name.to_ascii_lowercase()).collect();

    // Body parts bind to the shared skeleton by bone name.
    for path in ["meshes/b/b_n_dark elf_m_skins.nif", "meshes/b/b_n_nord_f_skins.nif"] {
        let nif = load(&vfs, path);
        assert!(nif.had_skin, "{path}");
        let (stats, bones) = check(path, &nif);
        eprintln!(
            "{path}: {} skinned meshes, {} bones, max {} influences, weight error {:.1e}, rest error {:.1e} m",
            stats.skinned,
            bones.len(),
            stats.max_influences,
            stats.max_weight_error,
            stats.max_rest_error
        );
        assert!(stats.skinned >= 1, "{path}");
        let missing: Vec<&String> = bones.iter().filter(|b| !skeleton.contains(*b)).collect();
        assert!(missing.is_empty(), "{path}: bones not in base_anim.nif: {missing:?}");
        assert!(stats.max_weight_error < 1e-2 && stats.max_rest_error < 1e-4, "{path}");
        let cooked = cook(path, &nif);
        assert!(cooked.joints.iter().all(|j| skeleton.contains(&j.name.to_ascii_lowercase())), "{path}");
        assert!(!cooked.parts.is_empty());
    }

    // Heads are rigid and morph (talk/blink).
    let path = "meshes/b/b_n_dark elf_m_head_01.nif";
    let nif = load(&vfs, path);
    let (stats, _) = check(path, &nif);
    eprintln!("{path}: {} morphing meshes, {} targets", stats.morphing, stats.morph_targets);
    assert!(stats.morphing >= 1 && stats.morph_targets >= 1, "{path}");
    let cooked = cook(path, &nif);
    assert_eq!(cooked.morph_targets.len(), stats.morph_targets, "{path}");

    // Creatures carry their own skeleton.
    for path in ["meshes/r/guar.nif", "meshes/r/cliffracer.nif"] {
        let nif = load(&vfs, path);
        let (stats, bones) = check(path, &nif);
        eprintln!(
            "{path}: {} skinned meshes, {} bones, max {} influences, weight error {:.1e}, rest error {:.1e} m",
            stats.skinned,
            bones.len(),
            stats.max_influences,
            stats.max_weight_error,
            stats.max_rest_error
        );
        assert!(stats.skinned >= 1, "{path}");
        assert!(stats.max_weight_error < 1e-2 && stats.max_rest_error < 1e-3, "{path}");
        let names: Vec<String> = nif.nodes.iter().map(|n| n.name.to_ascii_lowercase()).collect();
        for bone in &bones {
            assert_eq!(names.iter().filter(|n| *n == bone).count(), 1, "{path}: bone {bone} must be unique");
        }
        let cooked = cook(path, &nif);
        assert!(cooked.joints.len() >= bones.len(), "{path}: {} joints, {} bones", cooked.joints.len(), bones.len());
    }
}
