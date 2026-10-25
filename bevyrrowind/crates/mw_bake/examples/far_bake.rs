//! The whole Morrowind bake (models, exterior index and terrain, as
//! `bevyrrowind bake` does) plus the far field (`mw_bake::far`), with
//! far-field timings and sizes. Saves the profile manifest, so the viewer
//! sees the far-field products.
//!
//! `cargo run --release -p mw_bake --example far_bake [-- <Morrowind folder>]`
#![allow(clippy::cast_precision_loss)]

use std::sync::Arc;
use std::time::Instant;

use mw_bake::{VfsInputs, far, world};
use mw_install::MorrowindInstall;
use mw_world::MwTerrainSource;
use sf_bake::Store;
use sf_lod::FarDirectory;
use sf_lod::bake::FarBake;
use sf_vfs::Vfs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let started = Instant::now();
    let explicit = std::env::args().nth(1).map(std::path::PathBuf::from);
    let (install, _) = MorrowindInstall::detect(explicit.as_deref())?;
    let vfs = mw_vfs::mount_install(&install, Vfs::builder())?.vfs;
    let content = mw_records::load(&vfs, &install.game_files())?;
    // `SF_FAR_CACHE=<dir>`: bake into a scratch store (experiments).
    let root = match std::env::var_os("SF_FAR_CACHE") {
        Some(dir) => std::path::PathBuf::from(dir),
        None => mw_bake::default_cache_root().ok_or("no cache root")?,
    };
    let mut models = mw_bake::referenced_models(&content.db);
    let importers = mw_bake::importers();
    models.retain(|p, _| importers.for_path(p).is_some());
    let paths: Vec<String> = models.keys().cloned().collect();
    let store = Store::open(&root)?;
    let profile = mw_bake::profile_name(&install);
    let db = Arc::new(content.db);
    let vfs = Arc::new(vfs);
    let terrain = Arc::new(MwTerrainSource::new(db.clone()));
    let layers = Arc::new(world::MwLayers::new(&db, &vfs));
    let extra: Vec<_> = world::source_tile_fingerprints(&terrain).into_iter().chain(layers.fingerprints()).collect();
    let inputs =
        Arc::new(VfsInputs::new(vfs.clone(), &install, root.join("filehash.memo")).with_records(db.clone()).with_extra_records(extra));
    let loaded = started.elapsed();

    let mut baker = mw_bake::model_baker(store.clone(), inputs.clone());
    let t = Instant::now();
    let report = baker.bake(&mw_bake::model_keys(&paths));
    let models_time = t.elapsed();
    let t = Instant::now();
    let exterior = world::bake_exterior(&mut baker, &db, terrain, layers, &paths);
    let exterior_time = t.elapsed();
    let ext = world::exterior();
    let t = Instant::now();
    let far_report = far::bake_far(&mut baker, &db, std::slice::from_ref(&ext));
    let far_time = t.elapsed();
    let manifest = baker.manifest();
    inputs.save()?;
    store.save_manifest(&profile, &manifest)?;

    println!(
        "load {loaded:.2?}; models {models_time:.2?} ({} products); exterior {exterior_time:.2?}; far field {far_time:.2?}",
        report.products.len()
    );
    for (step, s) in &far_report.steps {
        if step.starts_with("lod.") {
            println!(
                "  {step:14} built {:6} hit {:6} failed {:4} run {:8.2} s (summed)  {:7.1} MiB written",
                s.built,
                s.hits,
                s.failed,
                s.run_time.as_secs_f64(),
                s.bytes_written as f64 / 1_048_576.0
            );
        }
    }
    for (key, error) in far_report.failures().take(10) {
        eprintln!("  failed: {key}: {error}");
    }
    for (key, error) in exterior.failures.iter().take(5) {
        eprintln!("  exterior failed: {key}: {error}");
    }
    let dir = manifest.get(sf_lod::products::DIR_STEP, ext.as_str()).ok_or("no far directory")?;
    let dir = FarDirectory::from_bytes(&store.get(&dir)?)?;
    for level in 1..=3 {
        let nodes: Vec<_> = dir.level(level).collect();
        let tris: u64 = nodes.iter().map(|n| u64::from(n.header.triangles)).sum();
        let inst: u64 = nodes.iter().map(|n| u64::from(n.header.instances)).sum();
        let lights: u64 = nodes.iter().map(|n| u64::from(n.header.lights)).sum();
        let bytes: u64 = nodes.iter().map(|n| n.bytes).sum();
        let max_err = nodes.iter().map(|n| n.header.error).fold(0.0f32, f32::max);
        println!(
            "  L{level}: {:5} nodes, {:9} triangles, {:7} impostors, {:6} lights, {:7.2} MiB, max error {max_err:.2} m",
            nodes.len(),
            tris,
            inst,
            lights,
            bytes as f64 / 1_048_576.0
        );
    }
    println!("  impostor models: {}", dir.impostor_models.len());
    for m in dir.impostor_models.iter().take(12) {
        println!("    {} (r {:.1} m)", m.model, m.radius);
    }
    // `SF_FAR_ATLAS=<substring>`: write that impostor model's atlas
    // (albedo, level 0) as target/far_atlas.png for inspection.
    if let Some(pick) = std::env::var_os("SF_FAR_ATLAS").and_then(|s| s.into_string().ok())
        && let Some(m) = dir.impostor_models.iter().find(|m| m.model.contains(&pick))
        && let Some(d) = manifest.get(sf_lod::products::IMPOSTOR_STEP, &m.model)
    {
        let atlas = sf_lod::ImpostorAtlas::from_bytes(&store.get(&d)?)?;
        let px = atlas.settings.atlas_px();
        let level0 = atlas.albedo[..(px * px * 4) as usize].to_vec();
        let img = image::RgbaImage::from_raw(px, px, level0).ok_or("atlas size")?;
        img.save("target/far_atlas.png")?;
        println!("  wrote target/far_atlas.png ({}, coverage {:.2})", m.model, atlas.coverage());
    }
    println!("total {:.2?}; manifest {profile} saved ({} products)", started.elapsed(), manifest.products.len());
    let _ = FarBake::directory_key(&ext);
    Ok(())
}
