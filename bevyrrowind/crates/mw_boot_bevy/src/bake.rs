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
    let (vfs, content, fingerprint, textures) = crate::mods::load_content(install, false)?;
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
    // Texture sources (textures-adapter.md §8): which VFS sources read with a non-default profile.
    for (source, pack) in textures.assigned() {
        println!("texture profile: {} -> {}", vfs.source(source).label(), pack.name);
    }
    for warning in &textures.warnings {
        println!("texture profile warning: {warning}");
    }
    let prepare = Instant::now();
    let db = Arc::new(content.db);
    let vfs = Arc::new(vfs);
    let terrain = Arc::new(MwTerrainSource::new(db.clone()));
    let layers = Arc::new(world::MwLayers::with_sources(&db, &vfs, &textures));
    let (layer_normals, layer_arms) = layers.companion_counts();
    let layer_count = layers.len();
    let extra: Vec<_> = world::source_tile_fingerprints(&terrain).into_iter().chain(layers.fingerprints()).collect();
    // The user's animation libraries (animation-sources.md): optional, BYO.
    let anim_sources = Arc::new(mw_bake::sources::AnimSources::local());
    let inputs = Arc::new(
        VfsInputs::new(vfs.clone(), install, root.join("filehash.memo"))
            .with_anim_sources(anim_sources.clone())
            .with_records(db.clone())
            .with_extra_records(extra),
    );
    let prepared = prepare.elapsed();

    let mut baker = mw_bake::model_baker_with(store.clone(), inputs.clone(), &textures);
    // Meshes + LODs, and each model's static collision source (bake-adapter.md).
    let mut keys = mw_bake::model_keys(&paths);
    keys.extend(mw_bake::collision::collision_keys(&paths));
    let report = baker.bake(&keys);
    let exterior = world::bake_exterior(&mut baker, &db, terrain, layers, &paths);
    // Navmeshes (nav.md "Bake"): after the collision cook and the terrain, which the tiles read.
    // Scaffold (2026-11-11, removed once the cold navmesh bake meets the "few minutes" gate, nav.md "Bake"):
    // `BEVYRROWIND_BAKE_SKIP_NAV=1` skips the navmesh steps so the other caches can re-bake meanwhile.
    let nav = if std::env::var_os("BEVYRROWIND_BAKE_SKIP_NAV").is_some() {
        println!("nav: skipped (BEVYRROWIND_BAKE_SKIP_NAV)");
        None
    } else {
        Some(mw_bake::nav::bake_nav(&mut baker, &db, &paths))
    };
    // Far field (far-field.md): L1-L3 proxies and impostors over the exterior.
    let far = mw_bake::far::bake_far(&mut baker, &db, &[world::exterior()]);
    // Animation (bake-adapter.md "Animation"): skeletons, clips and graphs of every actor.
    let anim = bake_actors(&mut baker, &db, &vfs, &anim_sources);
    // Item bodies and the settle test of placed items (physics-adapter.md "Items").
    let bodies = mw_bake::body::bake_bodies(&baker, &db, &paths);
    let items = mw_bake::body::check_items(&mut baker, &db);
    // Legacy scripts (scripting-adapter.md, M5 W2): MWScript -> Luau modules + bytecode, coverage report.
    let scripts = mw_bake::script::bake_scripts(&mut baker, &store, &db);
    // Albedo de-lighting (bake.md "De-lighting"): roles per model, de-lit BC7 KTX2 per texture, the layer's index.
    let delight = mw_bake::delight::bake_delight(&mut baker, &vfs, &paths, mw_bake::delight::DEFAULT_STRENGTH, &textures);
    // Texture cook (textures.md §5): every other referenced texture as BCn KTX2, after de-light (it skips de-lit ones).
    let cook = mw_bake::cook::bake_cook(&mut baker, &vfs, &delight, &textures);
    // Static scene (static-scene.md, scene-runtime-plan.md §3.3): one `scene.cells` table per
    // exterior cell, then the geometry of every chain they name. After the terrain (each cell's
    // land minimum) and the de-light (per-texture alpha classes): `bake_scene` refuses otherwise.
    let ext = [world::exterior()];
    let scene = mw_bake::scene::bake_scene(&mut baker, &db, &ext);
    let chains = scene.as_ref().ok().map(|_| mw_bake::scene::bake_chains(&baker, &ext));
    // Texture demand (textures.md §6, S3b): UV density per model + clusters per cell, after the cook (companions).
    let demand = mw_bake::demand::bake_demand(&mut baker, &ext);
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
    // T5 (textures.md §9): terrain layers with pack companions (`terrain.layers` time is in the step table).
    println!("terrain layers: {layer_count}, {layer_normals} with a normal map, {layer_arms} with ARM");
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
    print!("{delight}");
    for (key, error) in delight.failures.iter().take(10) {
        eprintln!("  failed: {key}: {error}");
    }
    print!("{cook}");
    for (key, error) in cook.failures.iter().take(10) {
        eprintln!("  failed: {key}: {error}");
    }
    let mut scene_failed = 0;
    match &scene {
        Ok(cells) => {
            println!("static scene: {} cell tables in {:.2?}", cells.products.len(), cells.elapsed);
            scene_failed += print_failures(cells);
        }
        Err(e) => eprintln!("static scene: not baked: {e}"),
    }
    if let Some(chains) = &chains {
        println!("static scene: {} chains in {:.2?}", chains.products.len(), chains.elapsed);
        scene_failed += print_failures(chains);
    }
    let demand_failed = match &demand {
        Ok(d) => {
            print!("{d}");
            d.failures.iter().take(10).for_each(|(key, error)| eprintln!("  failed: {key}: {error}"));
            d.failures.len()
        }
        Err(e) => {
            eprintln!("texture.demand: not baked: {e}");
            1
        }
    };
    if let Some(nav) = &nav {
        print!("{nav}");
        for (key, error) in nav.failures.iter().take(10) {
            eprintln!("  failed: {key}: {error}");
        }
    }
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
    let delight_failed = delight.failures.iter().filter(|(k, _)| k.step != mw_bake::delight::ROLES_STEP).count();
    if delight_failed > 0 {
        errors.push(format!("{delight_failed} de-lit texture products failed"));
    }
    if !cook.failures.is_empty() {
        errors.push(format!("{} cooked texture products failed", cook.failures.len()));
    }
    if let Err(e) = &scene {
        errors.push(format!("static scene not baked ({e})"));
    }
    if scene_failed > 0 {
        errors.push(format!("{scene_failed} static scene products failed"));
    }
    if demand_failed > 0 {
        errors.push(format!("{demand_failed} texture demand products failed"));
    }
    if let Some(nav) = nav.as_ref().filter(|n| !n.failures.is_empty()) {
        errors.push(format!("{} navmesh products failed", nav.failures.len()));
    }
    if scripts.errors() > 0 {
        errors.push(format!("{} legacy script products failed to translate or compile", scripts.errors()));
    }
    if errors.is_empty() { Ok(()) } else { Err(errors.join("; ").into()) }
}

/// Prints at most ten of `report`'s failures and returns how many there were.
fn print_failures(report: &sf_bake::BakeReport) -> usize {
    let failures: Vec<_> = report.failures().collect();
    for (key, error) in failures.iter().take(10) {
        eprintln!("  failed: {key}: {error}");
    }
    failures.len()
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
fn bake_actors(
    baker: &mut sf_bake::Baker,
    db: &Arc<sf_records::RecordDb>,
    vfs: &Arc<sf_vfs::Vfs>,
    anim_sources: &mw_bake::sources::AnimSources,
) -> mw_bake::anim::AnimBakeReport {
    let sets = mw_bake::anim::actor_sets(db, vfs);
    let anim = mw_bake::anim::bake_animation(baker, &sets, anim_sources);
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
    // Library slots (animation-sources.md): source used, or the fallback and why.
    println!("animation sources: {} library clips retargeted", anim.library_clips);
    for line in &anim.slots {
        println!("  slot {line}");
    }
    for (key, error) in anim.failures.iter().take(10) {
        eprintln!("  failed: {key}: {error}");
    }
}
