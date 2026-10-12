//! Static collision over the real install (`mw_bake::collision`): bakes
//! `model.info`/`mesh.cook` (usually cache hits), then `collision.cook`
//! for every referenced model, and reports sources, triangle totals and
//! the step's time. Merges the products into the profile manifest.
//!
//! `cargo run --release -p mw_bake --example collision_report [-- <Morrowind folder>]`
//!
//! `SF_COLLISION_CACHE=<dir>` bakes into a scratch store instead (a cold
//! first bake of models plus collision).

use std::sync::Arc;
use std::time::Instant;

use mw_bake::{VfsInputs, collision};
use mw_install::MorrowindInstall;
use sf_bake::Store;
use sf_vfs::Vfs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let started = Instant::now();
    let explicit = std::env::args().nth(1).map(std::path::PathBuf::from);
    let (install, _) = MorrowindInstall::detect(explicit.as_deref())?;
    let vfs = mw_vfs::mount_install(&install, Vfs::builder())?.vfs;
    let content = mw_records::load(&vfs, &install.game_files())?;
    let scratch = std::env::var_os("SF_COLLISION_CACHE").map(std::path::PathBuf::from);
    let root = match &scratch {
        Some(dir) => dir.clone(),
        None => mw_bake::default_cache_root().ok_or("no cache root")?,
    };
    let mut models = mw_bake::referenced_models(&content.db);
    let importers = mw_bake::importers();
    models.retain(|p, _| importers.for_path(p).is_some());
    let paths: Vec<String> = models.keys().cloned().collect();
    let store = Store::open(&root)?;
    let profile = mw_bake::profile_name(&install);
    let db = Arc::new(content.db);
    let inputs = Arc::new(VfsInputs::new(Arc::new(vfs), &install, root.join("filehash.memo")).with_records(db));
    let loaded = started.elapsed();

    let baker = mw_bake::model_baker(store.clone(), inputs.clone());
    let t = Instant::now();
    let models_report = baker.bake(&mw_bake::model_keys(&paths));
    let models_time = t.elapsed();
    let t = Instant::now();
    let collision_report = baker.bake(&collision::collision_keys(&paths));
    let collision_time = t.elapsed();
    let manifest = baker.manifest();
    inputs.save()?;
    let mut saved = store.load_manifest(&profile).unwrap_or_default();
    saved.merge(&manifest);
    store.save_manifest(&profile, &saved)?;

    let mesh = models_report.steps.get("mesh.cook").copied().unwrap_or_default();
    println!(
        "load {loaded:.2?}; models {models_time:.2?} (mesh.cook built {} hit {}); collision {collision_time:.2?}",
        mesh.built, mesh.hits
    );
    let t = Instant::now();
    let mut report = collision::report(&store, &manifest, inputs.as_ref(), &models);
    report.step = collision_report.steps.get("collision.cook").map(|s| (*s, collision_time));
    print!("{report}");
    println!("(report classification {:.2?})", t.elapsed());
    Ok(())
}
