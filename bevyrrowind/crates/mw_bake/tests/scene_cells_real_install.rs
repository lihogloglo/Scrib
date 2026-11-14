//! `scene.cells` over Vvardenfell (static-scene.md S1b, table format v2 since S1c): bakes models, the exterior index and
//! terrain (as `bevyrrowind bake` does), then one static scene table per exterior cell, and
//! prints counts, sizes and time. Uses the default bake cache (a warm cache makes the first
//! stages hits) or `SF_SCENE_CACHE=<dir>`; `MORROWIND_DIR` or the detected install.
//!
//! `cargo test --release -p mw_bake --test scene_cells_real_install -- --ignored --nocapture`
#![allow(clippy::cast_precision_loss)]

use std::collections::{BTreeMap, HashSet};
use std::sync::Arc;
use std::time::Instant;

use mw_bake::{VfsInputs, scene, world};
use mw_install::MorrowindInstall;
use mw_world::MwTerrainSource;
use sf_bake::Store;
use sf_scene::StaticCellTableView;
use sf_scene::bake::CELLS_STEP;
use sf_vfs::Vfs;

#[test]
#[ignore = "measurement: needs a Morrowind install; bakes the whole exterior"]
fn scene_cells_over_vvardenfell() {
    let started = Instant::now();
    let explicit = std::env::var_os("MORROWIND_DIR").map(std::path::PathBuf::from);
    let (install, _) = MorrowindInstall::detect(explicit.as_deref()).expect("Morrowind install");
    let vfs = mw_vfs::mount_install(&install, Vfs::builder()).expect("vfs").vfs;
    let content = mw_records::load(&vfs, &install.game_files()).expect("records");
    let root = std::env::var_os("SF_SCENE_CACHE")
        .map_or_else(|| mw_bake::default_cache_root().expect("cache root"), std::path::PathBuf::from);
    let mut models = mw_bake::referenced_models(&content.db);
    let importers = mw_bake::importers();
    models.retain(|p, _| importers.for_path(p).is_some());
    let paths: Vec<String> = models.keys().cloned().collect();
    let store = Store::open(&root).expect("store");
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
    baker.bake(&mw_bake::model_keys(&paths));
    world::bake_exterior(&mut baker, &db, terrain, layers, &paths);
    // The texture bake too: its per-texture alpha classes are what tell a cutout candidate
    // (foliage, fences) from a translucent blend, and `bake_scene` requires them (`scene.rs`).
    let delight = mw_bake::delight::bake_delight(&mut baker, &vfs, &paths, mw_bake::delight::DEFAULT_STRENGTH, &mw_bake::texture_profiles::TextureSources::default());
    let before = t.elapsed();
    let ext = world::exterior();
    let t = Instant::now();
    // `bake_scene` errors out when the terrain bake above left no level-0 tiles for the
    // worldspace (it would otherwise cache tables with horizon culling off, `scene.rs` docs).
    let report = scene::bake_scene(&mut baker, &db, std::slice::from_ref(&ext)).expect("terrain baked first");
    let scene_time = t.elapsed();
    let t = Instant::now();
    let chain_report = scene::bake_chains(&baker, std::slice::from_ref(&ext));
    let chains_time = t.elapsed();
    inputs.save().expect("memo");

    let (mut cells, mut empty, mut instances, mut chain_refs, mut bytes, mut no_land) = (0usize, 0usize, 0usize, 0usize, 0usize, 0usize);
    let mut placements_in_scene = 0usize;
    let mut chains: HashSet<u64> = HashSet::new();
    let mut materials: HashSet<u64> = HashSet::new();
    let mut classes: BTreeMap<u16, usize> = BTreeMap::new();
    for (key, digest) in &report.products {
        let Ok(digest) = digest else { continue };
        if key.step != CELLS_STEP {
            continue;
        }
        let data = store.get(digest).expect("table bytes");
        let view = StaticCellTableView::parse(&data).expect("valid table");
        cells += 1;
        bytes += data.len();
        empty += usize::from(view.is_empty());
        no_land += usize::from(view.land_min().is_infinite());
        instances += view.len();
        let mut last = None;
        for i in view.instances() {
            *classes.entry(view.material_class(i.chain).map_or(u16::MAX, |c| c as u16)).or_default() += 1;
            placements_in_scene += usize::from(last != Some(i.placement));
            last = Some(i.placement);
        }
        for c in view.chains() {
            chain_refs += 1;
            chains.insert(c.mesh);
            materials.insert(c.material);
        }
    }
    println!(
        "load {loaded:.2?}; models + exterior + textures {before:.2?}; scene.cells {scene_time:.2?}; scene.chains {chains_time:.2?} ({} chains, {} failed)",
        chain_report.products.len(),
        chain_report.failures().count()
    );
    // How many textures the candidate rule rests on, and how they split: the share of Morrowind's
    // authored-blend cards that are really cutouts (foliage, fences, grates) is what the static
    // table gains over baking every candidate blended, and `Soft` is what it must not promote.
    println!("alpha classes: {} cutout-candidate textures ({} built this run)", delight.alpha_classes, delight.alpha_built);
    println!(
        "cells {cells} ({empty} empty, {no_land} without land); instances {instances} from {placements_in_scene} placements; \
         chains {} distinct ({chain_refs} cell copies), {} distinct material keys; {:.1} MiB of tables",
        chains.len(),
        materials.len(),
        bytes as f64 / 1_048_576.0
    );
    println!("instances per material class: {classes:?}");
    if let Some(s) = report.steps.get(CELLS_STEP) {
        println!("{CELLS_STEP}: built {} hit {} failed {} run {:.2} s (summed)", s.built, s.hits, s.failed, s.run_time.as_secs_f64());
    }
    for (key, error) in report.failures().take(10) {
        eprintln!("failed: {key}: {error}");
    }
}
