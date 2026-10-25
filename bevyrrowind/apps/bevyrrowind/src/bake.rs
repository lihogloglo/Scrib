//! `bevyrrowind bake`: cooks every model the records reference, indexes the
//! exterior's placements, bakes its terrain (specs:
//! `superfargoth/docs/specs/bake.md`, `streaming.md`, `terrain.md`), and
//! saves the profile manifest the viewer and streaming load from.

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

use mw_bake::{BakeSummary, VfsInputs, world};
use mw_install::MorrowindInstall;
use mw_world::MwTerrainSource;
use sf_bake::Store;

use crate::Result;

/// The cache root: `--cache`, else the platform's local data dir.
pub fn cache_root(explicit: Option<PathBuf>) -> Result<PathBuf> {
    explicit.or_else(mw_bake::default_cache_root).ok_or_else(|| "no local data directory; pass --cache".into())
}

#[allow(clippy::too_many_lines)]
pub fn run(install: &MorrowindInstall, cache: Option<PathBuf>) -> Result {
    let started = Instant::now();
    let root = cache_root(cache)?;
    let (vfs, content, fingerprint) = crate::mods::load_content(install, false)?;
    let loaded = started.elapsed();
    let mut models = mw_bake::referenced_models(&content.db);
    // Mesh fields no importer handles (vanilla has "Add World Art").
    let importers = mw_bake::importers();
    let unsupported: Vec<String> = models.keys().filter(|p| importers.for_path(p).is_none()).cloned().collect();
    models.retain(|p, _| importers.for_path(p).is_some());
    if !unsupported.is_empty() {
        println!("ignoring {} model paths without an importer: {}", unsupported.len(), unsupported.join(", "));
    }
    let paths: Vec<String> = models.keys().cloned().collect();

    let store = Store::open(&root)?;
    let profile = mw_bake::profile_name(install);
    println!("baking {} models and the exterior for profile {profile} into {}", paths.len(), root.display());
    let prepare = Instant::now();
    let db = Arc::new(content.db);
    let vfs = Arc::new(vfs);
    let terrain = Arc::new(MwTerrainSource::new(db.clone()));
    let layers = Arc::new(world::MwLayers::new(&db, &vfs));
    let extra: Vec<_> = world::source_tile_fingerprints(&terrain).into_iter().chain(layers.fingerprints()).collect();
    let inputs = Arc::new(
        VfsInputs::new(vfs.clone(), install, root.join("filehash.memo"))
            .with_records(db.clone())
            .with_extra_records(extra),
    );
    let prepared = prepare.elapsed();

    let mut baker = mw_bake::model_baker(store.clone(), inputs.clone());
    // Meshes + LODs, and each model's static collision source (bake-adapter.md).
    let mut keys = mw_bake::model_keys(&paths);
    keys.extend(mw_bake::collision::collision_keys(&paths));
    let report = baker.bake(&keys);
    let exterior = world::bake_exterior(&mut baker, &db, terrain, layers, &paths);
    // Far field (far-field.md): L1-L3 proxies and impostors over the exterior.
    let far = mw_bake::far::bake_far(&mut baker, &db, &[world::exterior()]);
    // Animation (bake-adapter.md "Animation"): skeletons, clips and graphs of every actor.
    let anim = bake_actors(&mut baker, &db, &vfs);
    // Item bodies and the settle test of placed items (physics-adapter.md "Items").
    let bodies = mw_bake::body::bake_bodies(&baker, &db, &paths);
    let items = mw_bake::body::check_items(&mut baker, &db);
    // Legacy scripts (scripting-adapter.md, M5 W2): MWScript -> Luau modules + bytecode, coverage report.
    let scripts = mw_bake::script::bake_scripts(&mut baker, &store, &db);
    let manifest = baker.manifest();
    let save = Instant::now();
    inputs.save()?;
    store.save_manifest(&profile, &manifest)?;
    let fingerprint = sf_bake::Digest(fingerprint);
    let profile_changed = store.profile_fingerprint(&profile) != Some(fingerprint);
    store.save_profile_fingerprint(&profile, &fingerprint)?;
    let saved = save.elapsed();

    let analyze = Instant::now();
    let summary = BakeSummary::analyze(&report, &store, inputs.vfs(), &models);
    print!("\n{summary}");
    println!("(summary analysis {:.2?})", analyze.elapsed());
    println!(
        "\nexterior: {} placements ({} grounded); model bounds {:.2?}, source {:.2?}, index {:.2?}, terrain {:.2?} ({} products)",
        exterior.placements,
        exterior.grounded,
        exterior.bounds,
        exterior.source_build,
        exterior.index,
        exterior.terrain,
        exterior.terrain_products
    );
    println!(
        "interiors: {} indexed in {:.2?}; door links {}, unresolved link ends {}",
        exterior.interiors, exterior.interior_index, exterior.door_links, exterior.unresolved_link_ends
    );
    // Every step the model table above doesn't show: world, terrain, far field.
    for (step, s) in baker.step_stats() {
        if !report.steps.contains_key(step) {
            println!(
                "  {step:20} built {:5} hit {:5} run {:7.2} s (summed over threads)",
                s.built,
                s.hits,
                s.run_time.as_secs_f64()
            );
        }
    }
    for (key, error) in exterior.failures.iter().take(10) {
        eprintln!("  failed: {key}: {error}");
    }
    println!("far field: {} products in {:.2?}", far.products.len(), far.elapsed);
    print_rest_cooks(&manifest, &store, &models);
    let far_failures: Vec<_> = far.failures().collect();
    for (key, error) in far_failures.iter().take(10) {
        eprintln!("  failed: {key}: {error}");
    }
    print_animation(&anim);
    print!("{bodies}{items}");
    print!("{scripts}");
    let fingerprint_hex = fingerprint.hex();
    println!("profile fingerprint {} ({})", &fingerprint_hex[..12], if profile_changed { "changed since the last bake" } else { "unchanged" });
    println!(
        "load content {loaded:.2?}, prepare sources {prepared:.2?}, models {:.2?}, save memo + manifest {saved:.2?} ({} products), total {:.2?}",
        report.elapsed,
        manifest.products.len(),
        started.elapsed()
    );
    let mut errors = Vec::new();
    if !summary.failed.is_empty() {
        errors.push(format!("{} models failed to bake", summary.failed.len()));
    }
    if exterior.unresolved_link_ends > 0 {
        errors.push(format!("{} door link ends lead to unindexed worldspaces", exterior.unresolved_link_ends));
    }
    if !exterior.failures.is_empty() {
        errors.push(format!("{} exterior products failed", exterior.failures.len()));
    }
    if !far_failures.is_empty() {
        errors.push(format!("{} far-field products failed", far_failures.len()));
    }
    if !anim.failures.is_empty() {
        errors.push(format!("{} animation products failed", anim.failures.len()));
    }
    if !items.failed.is_empty() {
        errors.push(format!("{} items.check pages failed", items.failed.len()));
    }
    if scripts.errors() > 0 {
        errors.push(format!("{} legacy script products failed to translate or compile", scripts.errors()));
    }
    if errors.is_empty() { Ok(()) } else { Err(errors.join("; ").into()) }
}

/// Rest-pose cooks (weather.md bug B2): exterior models `mesh.cook` skips
/// for animation that now have far-field proxies.
fn print_rest_cooks(manifest: &sf_bake::Manifest, store: &Store, models: &std::collections::BTreeMap<String, mw_bake::ModelUse>) {
    let (mut rest_models, mut rest_placements, mut rest_skipped) = (0u32, 0u64, 0u32);
    let mut rest_top: Vec<(u32, &str)> = Vec::new();
    for (key, digest) in &manifest.products {
        if key.step != sf_model::steps::MeshRestStep::ID {
            continue;
        }
        match store.get(digest).ok().and_then(|b| sf_model::Cooked::from_bytes(&b).ok()) {
            Some(sf_model::Cooked::Static(_)) => {
                rest_models += 1;
                let uses = models.get(&key.subject).map_or(0, |u| u.placements);
                rest_placements += u64::from(uses);
                rest_top.push((uses, key.subject.as_str()));
            }
            _ => rest_skipped += 1,
        }
    }
    println!(
        "rest-pose cooks (mesh.rest): {rest_models} models ({rest_placements} placements, all cells) now far-field eligible; {rest_skipped} not static"
    );
    rest_top.sort_unstable_by(|a, b| b.cmp(a));
    for (uses, model) in rest_top.iter().take(8) {
        println!("  {uses:6} placements  {model}");
    }
}

/// The animation bake's line (bake-adapter.md "Animation") and its first failures.
/// Actor animation, then creature bodies cooked for their rig (`mesh.rig`,
/// character.md "Node-animated creatures"; failures named).
fn bake_actors(baker: &mut sf_bake::Baker, db: &Arc<sf_records::RecordDb>, vfs: &Arc<sf_vfs::Vfs>) -> mw_bake::anim::AnimBakeReport {
    let sets = mw_bake::anim::actor_sets(db, vfs);
    let anim = mw_bake::anim::bake_animation(baker, &sets);
    let mut keys = sets.body_rig_keys();
    keys.extend(held_rest_keys(db, vfs));
    let report = baker.bake(&keys);
    let failures: Vec<_> = report.failures().collect();
    println!("creature rigs: {} bodies, {} failed", sets.bodies.len(), failures.len());
    for (key, error) in failures.iter().take(10) {
        eprintln!("  failed: {key}: {error}");
    }
    print_ragdolls(&mw_bake::ragdoll::bake_ragdolls(baker, &sets));
    anim
}

/// The `ragdoll.cook` census (physics-adapter.md "Ragdolls"); every row with
/// `BEVYRROWIND_RAGDOLL_REPORT=1`.
fn print_ragdolls(r: &mw_bake::ragdoll::RagdollReport) {
    let humanoid = r.rows.iter().filter(|x| x.humanoid).count();
    let clip_only = r.rows.iter().filter(|x| x.policy != sf_ragdoll::RagdollPolicy::Ragdoll).count();
    let (lo, hi) = r.rows.iter().fold((usize::MAX, 0), |(lo, hi), x| (lo.min(x.bodies), hi.max(x.bodies)));
    let (mlo, mhi) = r.rows.iter().fold((f32::MAX, 0.0f32), |(lo, hi), x| (lo.min(x.mass), hi.max(x.mass)));
    println!(
        "ragdolls: {} skeletons ({humanoid} humanoid, {} creature), {} failed, {clip_only} death-clip only; {lo}-{hi} bodies, {mlo:.1}-{mhi:.1} kg, {:.2} s",
        r.rows.len(),
        r.rows.len() - humanoid,
        r.failed.len(),
        r.wall.as_secs_f32()
    );
    for (s, e) in &r.failed {
        eprintln!("  ragdoll failed: {s}: {e}");
    }
    if std::env::var_os("BEVYRROWIND_RAGDOLL_REPORT").is_some() {
        for x in &r.rows {
            println!("  {:<44} {:>3} actors {:>2} bodies {:>2} hinges {:>7.1} kg {:?} {:?}", x.skeleton, x.actors, x.bodies, x.hinges, x.mass, x.policy, x.regions);
        }
    }
}

/// Held models (weapons, carried lights) as `mesh.rest` too: the step cooks
/// only models the plain cook skips for an animated node (a torch's flame),
/// which held props then show in their rest pose (`MwCharacters::held_model`).
fn held_rest_keys(db: &sf_records::RecordDb, vfs: &sf_vfs::Vfs) -> Vec<sf_bake::ProductKey> {
    use tes3::esp::{Light, LightFlags, Weapon};
    let lights = db.iter::<Light>().filter(|(_, l)| l.data.flags.contains(LightFlags::CAN_CARRY)).map(|(_, l)| l.mesh.as_str());
    let weapons = db.iter::<Weapon>().map(|(_, w)| w.mesh.as_str());
    let mut models: Vec<String> =
        lights.chain(weapons).filter_map(mw_world::model_path).filter(|m| sf_vfs::VfsPath::new(m).is_ok_and(|p| vfs.contains(&p))).collect();
    models.sort();
    models.dedup();
    models.iter().map(|m| sf_bake::ProductKey::new(sf_model::steps::MeshRestStep::ID, m.as_str())).collect()
}

fn print_animation(anim: &mw_bake::anim::AnimBakeReport) {
    println!(
        "animation: {} actor sets, {} files, {} group clips, {} graphs; phase 1 {:.2?}, phase 2 {:.2?}",
        anim.sets, anim.files, anim.group_clips, anim.graphs, anim.phase1, anim.phase2
    );
    for (key, error) in anim.failures.iter().take(10) {
        eprintln!("  failed: {key}: {error}");
    }
}
