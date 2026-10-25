//! Rig report for Morrowind skeletons: joint tree, humanoid auto-map and
//! rig profile.
//!
//! `cargo run -p mw_anim --example rig_report -- meshes/base_anim.nif --humanoid [--tree]` (humanoid map,
//! masks, profile) or `-- meshes/r/guar.nif` (creature profile with the adapter hints).
//! `cargo run -p mw_anim --example rig_report -- --creatures guar racer` lists matching CREA records
//! (id, model, flags) from `Morrowind.esm`.

use mw_install::MorrowindInstall;
use sf_vfs::{Vfs, VfsPath};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (install, _) = MorrowindInstall::detect(None).expect("no Morrowind install detected");
    let vfs = mw_vfs::mount_install(&install, Vfs::builder()).expect("mount").vfs;
    if args.first().is_some_and(|a| a == "--creatures") {
        for esm in ["Morrowind.esm", "Tribunal.esm", "Bloodmoon.esm"] {
            let Some(bytes) = install.data_file(esm).and_then(|p| std::fs::read(p).ok()) else { continue };
            let mut plugin = tes3::esp::Plugin::new();
            plugin.load_bytes(&bytes).expect("parse esm");
            for c in plugin.objects_of_type::<tes3::esp::Creature>() {
            let id = c.id.to_ascii_lowercase();
            if args[1..].iter().any(|a| id.contains(&a.to_ascii_lowercase())) {
                println!("{:<32} {:<32} flags {:#06x}", c.id, c.mesh, c.creature_flags.bits());
            }
        }
        }
        return;
    }
    let tree = args.iter().any(|a| a == "--tree");
    for path in args.iter().filter(|a| !a.starts_with("--")) {
        let bytes = vfs.read(&VfsPath::new(path).unwrap()).expect("read");
        let model = mw_nif::load(&bytes).expect("nif");
        let skeleton = mw_anim::rig::skeleton(&model).expect("skeleton");
        println!("== {path}: {} joints", skeleton.len());
        if tree {
            let model_space = skeleton.rest_model();
            for (j, m) in model_space.iter().enumerate() {
                let mut depth = 0;
                let mut p = skeleton.parent(j);
                while let Some(q) = p {
                    depth += 1;
                    p = skeleton.parent(q);
                }
                let t = m.translation;
                let tags: Vec<&str> = skeleton.joint_tags(j).collect();
                println!("{:indent$}{} ({:.2} {:.2} {:.2}) {tags:?}", "", skeleton.name(j), t.x, t.y, t.z, indent = depth * 2);
            }
        }
        if args.iter().any(|a| a == "--humanoid") {
            let rig = mw_anim::rig::humanoid(&skeleton).expect("bone skeleton");
            let r = &rig.report;
            println!(
                "humanoid: {} bones, clean {}, root motion {}, missing required {:?}, warnings {:?}, errors {:?}",
                rig.map.bones.len(),
                r.is_clean(),
                rig.map.root_motion,
                r.missing_required,
                r.warnings,
                r.errors
            );
            println!("  unmapped bones {:?}", r.unmapped);
            println!("  extra chains {:?}", rig.map.extra_chains);
            println!("  align: facing {:.1} deg, residual {:.2} deg", rig.align.facing_correction_deg, rig.align.max_residual_deg);
            for (group, mask) in mw_anim::rig::masks(&skeleton, &rig.map) {
                let names: Vec<&str> = mask.iter().map(|j| skeleton.name(j)).collect();
                println!("  mask {group:?} ({}): {}", names.len(), names.join(", "));
            }
            match rig.map.profile(&skeleton) {
                Ok(p) => print!("{}", mw_anim::rig::profile_summary(&skeleton, &p)),
                Err(e) => println!("profile error: {e}"),
            }
        } else {
            match mw_anim::rig::creature_profile(&skeleton, path) {
                Ok(p) => print!("{}", mw_anim::rig::profile_summary(&skeleton, &p)),
                Err(e) => println!("profile error: {e}"),
            }
        }
    }
}
