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
    // `SF_ITEMS_CELL="<interior cell>"`: that cell's items with their
    // `items.check` result and the items' bounds (spot checks, scene cameras).
    if let Ok(cell) = std::env::var("SF_ITEMS_CELL") {
        let ws = mw_world::ids::interior(&cell);
        let items = mw_bake::body::worldspace_items(&store, &saved, &ws);
        let (mut lo, mut hi) = ([f64::MAX; 3], [f64::MIN; 3]);
        for it in &items {
            let c = &it.check;
            println!(
                "  {:<28} {:<40} at {:>7.2} {:>6.2} {:>7.2}  {:?} drop {:.3} rise {:.3} shift {:.3}",
                it.base, it.model, it.pos[0], it.pos[1], it.pos[2], c.hold, c.drop, c.rise, c.shift
            );
            for i in 0..3 {
                lo[i] = lo[i].min(it.pos[i]);
                hi[i] = hi[i].max(it.pos[i]);
            }
        }
        println!("{cell}: {} items, bounds {lo:.2?} .. {hi:.2?}", items.len());
        // `SF_ITEMS_NEAR="x,y,z"`: the cell's other placements whose origin is
        // within 3 m of that point, with their collision mesh (triangles and
        // bounds): what an unsupported item should have rested on.
        if let Some(near) = std::env::var("SF_ITEMS_NEAR").ok().and_then(|s| parse_point(&s)) {
            statics_near(&store, &saved, &ws, near);
        }
    }
    Ok(())
}

fn parse_point(s: &str) -> Option<[f64; 3]> {
    let v: Vec<f64> = s.split(',').filter_map(|x| x.trim().parse().ok()).collect();
    (v.len() == 3).then(|| [v[0], v[1], v[2]])
}

fn statics_near(store: &Store, manifest: &sf_bake::Manifest, ws: &sf_world::WorldspaceId, near: [f64; 3]) {
    let prefix = format!("{}/", ws.as_str());
    let read = |step: &str, subject: &str| manifest.get(step, subject).and_then(|d| store.get(&d).ok());
    for key in manifest.products.keys().filter(|k| k.step == sf_world::bake::PAGE_STEP && k.subject.starts_with(&prefix)) {
        let Some(Ok(page)) = read(&key.step, &key.subject).map(|b| sf_world::IndexPage::from_bytes(&b)) else { continue };
        for p in page.cells.iter().flat_map(|c| c.placements.iter().map(|p| &p.placement)) {
            let pos = p.transform.pos.to_f64().to_array();
            let d = (0..3).map(|i| (pos[i] - near[i]).powi(2)).sum::<f64>().sqrt();
            let Some(model) = p.model.as_ref().map(sf_vfs::VfsPath::as_str) else { continue };
            if d > 3.0 {
                continue;
            }
            let mesh = read(sf_controller::steps::COLLISION_COOK, model)
                .and_then(|b| sf_controller::CollisionMesh::from_bytes(&b).ok());
            let info = mesh.map_or_else(
                || "no collision.cook".to_owned(),
                |m| {
                    let (mut lo, mut hi) = ([f32::MAX; 3], [f32::MIN; 3]);
                    for q in &m.positions {
                        for i in 0..3 {
                            lo[i] = lo[i].min(q[i]);
                            hi[i] = hi[i].max(q[i]);
                        }
                    }
                    format!("{} tris, local {lo:.2?} .. {hi:.2?}", m.triangles.len())
                },
            );
            println!(
                "  near {:<28} {:<40} {:?} at {:>7.2} {:>6.2} {:>7.2} rot {:?} scale {:.2}: {info}",
                p.base.id.as_str(),
                model,
                p.class,
                pos[0],
                pos[1],
                pos[2],
                p.transform.rotation,
                p.transform.scale.x
            );
        }
    }
}
