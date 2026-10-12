//! Skins and geometry morphs across every NIF of the install: what resolves,
//! influence counts, rest-pose agreement with the baked pose, and how long
//! `mesh.cook` takes for the models that now cook as `Cooked::Skinned`.
//!
//! `cargo run --release -p mw_bake --example skin_report [-- <Morrowind folder>]`
#![allow(clippy::cast_precision_loss, clippy::too_many_lines)]

use std::collections::BTreeMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use mw_install::MorrowindInstall;
use mw_nif::glam::Vec3;
use rayon::prelude::*;
use sf_model::{CookSettings, Cooked};
use sf_vfs::Vfs;

#[derive(Default)]
struct Totals {
    models: usize,
    unreadable: usize,
    skinned_models: usize,
    morph_models: usize,
    morph_only_outside_b: Vec<String>,
    skinned_meshes: usize,
    unresolved: Vec<String>,
    offset_bind: Vec<String>,
    /// Influence count -> vertices.
    influences: BTreeMap<usize, usize>,
    no_influence: usize,
    max_weight_error: f32,
    max_rest_error: (f32, String),
    morph_meshes: usize,
    morph_targets: usize,
    /// `NiGeomMorpherController`s vs meshes whose morph converted.
    morph_controllers: usize,
    partitions: usize,
    cooked: BTreeMap<String, usize>,
    cook_cpu: Duration,
    cooked_bytes: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let explicit = std::env::args().nth(1).map(std::path::PathBuf::from);
    let (install, _) = MorrowindInstall::detect(explicit.as_deref())?;
    let vfs = mw_vfs::mount_install(&install, Vfs::builder())?.vfs;
    let is_nif = |p: &&sf_vfs::VfsPath| std::path::Path::new(p.as_str()).extension().is_some_and(|e| e.eq_ignore_ascii_case("nif"));
    let paths: Vec<_> = vfs.paths().filter(is_nif).cloned().collect();
    let totals = Mutex::new(Totals::default());
    let started = Instant::now();
    paths.par_iter().for_each(|path| {
        let name = path.as_str().to_owned();
        let Some(stream) = vfs.read(path).ok().and_then(|b| mw_nif::parse(&b).ok()) else {
            totals.lock().unwrap().unreadable += 1;
            return;
        };
        let types = mw_nif::census::object_types(&stream);
        let Ok(nif) = mw_nif::convert(stream) else {
            totals.lock().unwrap().unreadable += 1;
            return;
        };
        let world = mw_nif::world_transforms(&nif);
        let mut t = Totals { models: 1, ..Totals::default() };
        t.morph_controllers = types.get("NiGeomMorpherController").copied().unwrap_or(0);
        t.partitions = types.get("NiSkinPartition").copied().unwrap_or(0);
        for (index, node) in nif.nodes.iter().enumerate() {
            let Some(mesh) = node.mesh.map(|m| &nif.meshes[m]) else { continue };
            if let Some(morph) = &mesh.morph {
                t.morph_meshes += 1;
                t.morph_targets += morph.targets.len();
            }
            if !mesh.skinned {
                continue;
            }
            t.skinned_meshes += 1;
            let Some(skin) = mesh.skin.as_deref() else {
                t.unresolved.push(format!("{name}: {}", node.name));
                continue;
            };
            if skin.offset_bind {
                t.offset_bind.push(format!("{name}: {}", node.name));
            }
            for influences in &skin.influences {
                *t.influences.entry(influences.len()).or_default() += 1;
                if influences.is_empty() {
                    t.no_influence += 1;
                } else {
                    let sum: f32 = influences.iter().map(|i| i.1).sum();
                    t.max_weight_error = t.max_weight_error.max((sum - 1.0).abs());
                }
            }
            for (p, baked) in skin.rest_positions(&world).iter().zip(&mesh.positions) {
                let error = p.distance(world[index].transform_point3(Vec3::from_array(*baked)));
                if error > t.max_rest_error.0 {
                    t.max_rest_error = (error, format!("{name}: {}", node.name));
                }
            }
        }
        t.skinned_models = usize::from(t.skinned_meshes > 0);
        t.morph_models = usize::from(t.morph_meshes > 0);
        if t.morph_meshes > 0 && !nif.had_skin && !name.starts_with("meshes/b/") {
            t.morph_only_outside_b.push(name.clone());
        }
        if t.skinned_models + t.morph_models > 0 {
            let cook_start = Instant::now();
            let model = mw_bake::to_model(&nif);
            let cooked = sf_model::cook(&model, &CookSettings::default());
            t.cook_cpu = cook_start.elapsed();
            let kind = match &cooked {
                Ok(Cooked::Skinned(_)) => "skinned".to_owned(),
                Ok(Cooked::Static(_)) => "static".to_owned(),
                Ok(Cooked::Skipped { reason }) => format!("skipped: {reason}"),
                Err(e) => format!("error: {e}"),
            };
            if let Ok(cooked) = &cooked {
                t.cooked_bytes = cooked.to_bytes().len();
            }
            t.cooked.insert(kind, 1);
        }
        let mut all = totals.lock().unwrap();
        merge(&mut all, t);
    });
    let wall = started.elapsed();
    let t = totals.into_inner().unwrap();
    println!("{} NIFs ({} unreadable) in {:.1} s", t.models, t.unreadable, wall.as_secs_f32());
    println!(
        "skins: {} models, {} meshes, {} unresolved, {} with an offset bind (M != I)",
        t.skinned_models,
        t.skinned_meshes,
        t.unresolved.len(),
        t.offset_bind.len()
    );
    println!("  influences per vertex -> vertices: {:?} ({} without any)", t.influences, t.no_influence);
    println!("  max |sum(weights) - 1| {:.2e}; max rest-pose error {:.2e} m ({})", t.max_weight_error, t.max_rest_error.0, t.max_rest_error.1);
    println!("  NiSkinPartition objects: {} (ignored)", t.partitions);
    println!(
        "morphs: {} models, {} meshes, {} targets; {} NiGeomMorpherController objects",
        t.morph_models, t.morph_meshes, t.morph_targets, t.morph_controllers
    );
    println!("  morph-only models outside meshes/b/: {} {:?}", t.morph_only_outside_b.len(), t.morph_only_outside_b.iter().take(12).collect::<Vec<_>>());
    println!("cook results: {:?}", t.cooked);
    println!(
        "cook CPU time (import + cook) of skinned/morphing models: {:.2} s; {:.1} MB cooked",
        t.cook_cpu.as_secs_f32(),
        t.cooked_bytes as f32 / 1e6
    );
    for list in [&t.unresolved, &t.offset_bind] {
        for item in list.iter().take(10) {
            println!("  {item}");
        }
    }
    Ok(())
}

fn merge(all: &mut Totals, t: Totals) {
    all.models += t.models;
    all.skinned_models += t.skinned_models;
    all.morph_models += t.morph_models;
    all.morph_only_outside_b.extend(t.morph_only_outside_b);
    all.skinned_meshes += t.skinned_meshes;
    all.unresolved.extend(t.unresolved);
    all.offset_bind.extend(t.offset_bind);
    for (k, v) in t.influences {
        *all.influences.entry(k).or_default() += v;
    }
    all.no_influence += t.no_influence;
    all.max_weight_error = all.max_weight_error.max(t.max_weight_error);
    if t.max_rest_error.0 > all.max_rest_error.0 {
        all.max_rest_error = t.max_rest_error;
    }
    all.morph_meshes += t.morph_meshes;
    all.morph_targets += t.morph_targets;
    all.morph_controllers += t.morph_controllers;
    all.partitions += t.partitions;
    for (k, v) in t.cooked {
        *all.cooked.entry(k).or_default() += v;
    }
    all.cook_cpu += t.cook_cpu;
    all.cooked_bytes += t.cooked_bytes;
}
