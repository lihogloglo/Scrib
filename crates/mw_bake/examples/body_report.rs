//! Item bodies over the real install (`mw_bake::body`): bakes
//! `model.info`/`mesh.cook` (usually cache hits), then `body.cook` for every
//! item model (MISC, WEAP, ARMO, CLOT, BOOK, INGR, ALCH, APPA, LOCK, PROB,
//! REPA, carryable LIGH), and reports shape kinds, hull vertex counts,
//! decomposition pieces, the mass range and the step's time. Merges the
//! products into the profile manifest. `items.check` needs the world index:
//! `bevyrrowind bake` runs it and prints its report.
//!
//! `cargo run --release -p mw_bake --example body_report [-- <Morrowind folder>]`
//!
//! `SF_BODY_CACHE=<dir>` bakes into a scratch store instead (a cold first
//! bake of models plus bodies).

use std::sync::Arc;
use std::time::Instant;

use mw_bake::VfsInputs;
use mw_install::MorrowindInstall;
use sf_bake::Store;
use sf_vfs::Vfs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let started = Instant::now();
    let explicit = std::env::args().nth(1).map(std::path::PathBuf::from);
    let (install, _) = MorrowindInstall::detect(explicit.as_deref())?;
    let vfs = mw_vfs::mount_install(&install, Vfs::builder())?.vfs;
    let content = mw_records::load(&vfs, &install.game_files())?;
    let root = match std::env::var_os("SF_BODY_CACHE") {
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
    let inputs = Arc::new(VfsInputs::new(Arc::new(vfs), &install, root.join("filehash.memo")).with_records(db.clone()));
    let loaded = started.elapsed();

    let baker = mw_bake::model_baker(store.clone(), inputs.clone());
    let t = Instant::now();
    let items: Vec<String> = mw_bake::body::item_models(&db).into_keys().filter(|p| paths.binary_search(p).is_ok()).collect();
    let models_report = baker.bake(&mw_bake::model_keys(&items));
    let models_time = t.elapsed();
    let report = mw_bake::body::bake_bodies(&baker, &db, &paths);
    let manifest = baker.manifest();
    inputs.save()?;
    let mut saved = store.load_manifest(&profile).unwrap_or_default();
    saved.merge(&manifest);
    store.save_manifest(&profile, &saved)?;

    let mesh = models_report.steps.get("mesh.cook").copied().unwrap_or_default();
    println!("load {loaded:.2?}; item models {models_time:.2?} (mesh.cook built {} hit {})", mesh.built, mesh.hits);
    print!("{report}");
    Ok(())
}
