//! Actor animation through the bake on the real game data: every actor's
//! graph compiles from the store and loads with its clips, group tables
//! match `mw_anim`, the cooked skeleton and graph match `mw_graphs`'
//! in-memory path, a warm bake rebuilds nothing, and 1 vs N threads bake
//! the same bytes. Skips (passes) when no Morrowind install is detected.
//! `-- --nocapture` prints the numbers (cold, warm, counts).

use std::collections::BTreeMap;
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;

use mw_anim::Animation;
use mw_bake::VfsInputs;
use mw_bake::anim::{self, ANIM_GROUPS, ActorSets, AnimBakeReport, AnimGroupsStep, AnimLibrary};
use mw_graphs::{AnimSet, HUMANOID_SETS, Template};
use mw_install::MorrowindInstall;
use sf_anim::steps::{ANIM_COOK, SKELETON_COOK};
use sf_anim_graph::steps::ANIMGRAPH_COMPILE;
use sf_bake::{Baker, Manifest, ProductKey, Store};
use sf_records::RecordDb;
use sf_vfs::{Vfs, VfsPath};

struct Content {
    install: MorrowindInstall,
    vfs: Arc<Vfs>,
    db: Arc<RecordDb>,
}

fn content() -> Option<Content> {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return None;
    };
    // The shipped graphs are the lowest layer: mods override them.
    let vfs = mw_vfs::mount_install(&install, Vfs::builder().mount(mw_graphs::builtin_source())).unwrap().vfs;
    let db = mw_records::load(&vfs, &install.game_files()).unwrap().db;
    Some(Content { install, vfs: Arc::new(vfs), db: Arc::new(db) })
}

fn bake(c: &Content, dir: &Path, sets: &ActorSets) -> (AnimBakeReport, Manifest, Baker) {
    let store = Store::open(dir.join("store")).unwrap();
    let inputs = Arc::new(VfsInputs::new(c.vfs.clone(), &c.install, dir.join("filehash.memo")).with_records(c.db.clone()));
    let mut baker = Baker::new(store, inputs.clone());
    let report = anim::bake_animation(&mut baker, sets);
    inputs.save().unwrap();
    let manifest = baker.manifest();
    (report, manifest, baker)
}

fn count(manifest: &Manifest, step: &str) -> usize {
    manifest.products.keys().filter(|k| k.step == step).count()
}

#[test]
#[allow(clippy::too_many_lines)]
fn every_actor_graph_compiles_from_the_store() {
    let Some(c) = content() else { return };
    for (path, _) in mw_graphs::BUILTIN {
        assert!(c.vfs.contains(&VfsPath::new(path).unwrap()), "builtin layer: {path}");
    }
    let t = Instant::now();
    let sets = anim::actor_sets(&c.db, &c.vfs);
    let sets_time = t.elapsed();
    let pairs = sets.graph_pairs().count();
    eprintln!(
        "actor sets: {} skeletons, {} files, {} (graph, skeleton) pairs, {} actors in {sets_time:.2?}; skipped {}, conflicts {:?}",
        sets.sets.len(),
        sets.files().len(),
        pairs,
        sets.sets.values().map(|s| s.actors).sum::<u32>(),
        sets.skipped.len(),
        sets.conflicts
    );
    assert!(sets.sets.len() >= 60, "humanoid + creature skeletons");
    for stock in &HUMANOID_SETS {
        assert!(sets.sets.contains_key(stock.skeleton), "{} used", stock.skeleton);
    }

    let dir = tempfile::tempdir().unwrap();
    let t = Instant::now();
    let (report, manifest, baker) = bake(&c, dir.path(), &sets);
    let cold = t.elapsed();
    let stats = baker.step_stats();
    eprintln!(
        "cold: {cold:.2?} (phase 1 {:.2?}, phase 2 {:.2?}); {} group clips, {} graphs; products: {} skeletons, {} group tables, {} clips, {} graphs",
        report.phase1,
        report.phase2,
        report.group_clips,
        report.graphs,
        count(&manifest, SKELETON_COOK),
        count(&manifest, ANIM_GROUPS),
        count(&manifest, ANIM_COOK),
        count(&manifest, ANIMGRAPH_COMPILE),
    );
    for (step, s) in &stats {
        eprintln!(
            "  {step:18} built {:5} hits {:5} dup {:3} run {:.2?} (summed over threads)",
            s.built, s.hits, s.duplicates, s.run_time
        );
    }
    for (k, e) in report.failures.iter().take(20) {
        eprintln!("  FAILED {k:?}: {e}");
    }
    assert!(report.failures.is_empty(), "{} failures", report.failures.len());
    // Cooked rates and errors (the cook tries 30, 60, 120 Hz until the error bound holds).
    let mut rates: BTreeMap<u32, (usize, usize, f32)> = BTreeMap::new();
    for (key, digest) in manifest.products.iter().filter(|(k, _)| k.step == ANIM_COOK) {
        let clip = sf_anim::Clip::from_bytes(&baker.store().get(digest).unwrap()).unwrap_or_else(|e| panic!("{key:?}: {e}"));
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let e = rates.entry(clip.rate.round() as u32).or_default();
        e.0 += 1;
        e.1 += usize::from(clip.meta.max_error > sf_anim::CookSettings::default().error_bound);
        e.2 = e.2.max(clip.meta.max_error);
    }
    eprintln!("clips by rate (Hz: count, over the bound, worst error m): {rates:?}");
    assert_eq!(report.graphs, pairs, "every pair compiled");

    // Group tables equal a direct parse (mw_anim), file by file.
    let mut groups_total = 0;
    for file in sets.files() {
        let digest = manifest.get(ANIM_GROUPS, file).unwrap();
        let table = AnimGroupsStep::decode(&baker.store().get(&digest).unwrap()).unwrap();
        let bytes = c.vfs.read(&VfsPath::new(file).unwrap()).unwrap();
        let direct = Animation::from_file(file, &bytes).unwrap().groups();
        assert_eq!(table.len(), direct.len(), "{file}");
        groups_total += table.len();
    }
    let kf_used = sets.files().iter().filter(|f| Path::new(f).extension().is_some_and(|e| e.eq_ignore_ascii_case("kf"))).count();
    eprintln!("groups in the {} files actors use: {groups_total} ({kf_used} KFs; mw_anim: 2330 groups in all 162 KFs)", sets.files().len());

    // Every actor's graph and clips load through the library.
    let library = AnimLibrary::new(baker.store().clone(), manifest.clone());
    let t = Instant::now();
    let mut slots = 0;
    for (graph, skeleton) in sets.graph_pairs() {
        let a = library.load(skeleton, graph).unwrap_or_else(|e| panic!("{graph} on {skeleton}: {e}"));
        assert_eq!(a.clips.len(), a.graph.clips.len());
        assert!(!a.clips.is_empty(), "{graph} on {skeleton}");
        slots += a.clips.len();
        assert!(library.groups(skeleton).is_some_and(|g| !g.is_empty()), "{skeleton} groups");
    }
    eprintln!("loaded {pairs} graphs, {slots} clip slots in {:.2?}", t.elapsed());
    let male = HUMANOID_SETS[0].skeleton;
    assert!(library.group_clip(male, "walkforward").is_ok());
    // Authored ground speed of the locomotion cycles (animfix 2026-10-12):
    // what `SpeedMatch` divides the mover's measured speed by. Movers walk
    // at fMinWalkSpeed + Speed/100 * (fMaxWalkSpeed - fMinWalkSpeed) units/s.
    eprintln!("clip            root speed m/s  cycle m  loop s  contacts");
    for group in ["walkforward", "walkback", "walkleft", "walkright", "runforward", "runback", "sneakforward", "idle", "turnleft"] {
        let clip = library.group_clip(male, group).unwrap();
        let (start, end) = clip.loop_range.unwrap_or((0.0, clip.duration));
        let contacts = clip.curves.iter().filter(|c| c.name.starts_with("contact.")).count();
        let vel = clip.meta.average_velocity;
        eprintln!("{group:<15} {:>14.3} {:>8.3} {:>7.3} {contacts:>9}", glam::Vec2::new(vel.x, vel.z).length(), clip.meta.cycle_distance, end - start);
        if group.starts_with("walk") || group.starts_with("run") {
            assert!(vel.length() > 0.5, "{group}: the cycle carries its root speed ({vel})");
        }
    }

    // Same skeleton and graph shape as mw_graphs' in-memory path.
    let model = mw_nif::load(&c.vfs.read(&VfsPath::new(male).unwrap()).unwrap()).unwrap();
    let skeleton = mw_anim::rig::skeleton(&model).unwrap();
    assert_eq!(library.skeleton(male).unwrap().to_bytes(), skeleton.to_bytes(), "skeleton.cook == rig::skeleton");
    let anims: Vec<_> = HUMANOID_SETS[0]
        .animations
        .iter()
        .map(|f| ((*f).to_owned(), Animation::from_file(f, &c.vfs.read(&VfsPath::new(f).unwrap()).unwrap()).unwrap()))
        .collect();
    let mut set = AnimSet::new("male", skeleton, anims).with_humanoid_rig();
    let v = mw_graphs::validate(&mut set, Template::Humanoid).unwrap();
    let cooked = library.load(male, Template::Humanoid.path()).unwrap();
    assert_eq!(cooked.graph.clips.len(), v.graph.clips.len());
    assert_eq!(cooked.graph.nodes.len(), v.graph.nodes.len());
    for (a, b) in cooked.graph.clips.iter().zip(&v.graph.clips) {
        assert_eq!(a.reference, b.reference);
        assert!((a.duration - b.duration).abs() < 1e-4, "{}", a.reference);
    }

    // Warm: nothing rebuilds.
    let t = Instant::now();
    let sets_again = anim::actor_sets(&c.db, &c.vfs);
    let (warm, warm_manifest, warm_baker) = bake(&c, dir.path(), &sets_again);
    let warm_time = t.elapsed();
    let built: u32 = warm_baker.step_stats().values().map(|s| s.built).sum();
    eprintln!("warm: {warm_time:.2?} (actor sets + phase 1 {:.2?}, phase 2 {:.2?}), built {built}", warm.phase1, warm.phase2);
    assert_eq!(built, 0);
    assert_eq!(warm_manifest, manifest);
}

/// The anim products of a few sets, baked on one thread and on many, are identical.
#[test]
fn one_thread_and_many_bake_the_same_bytes() {
    let Some(c) = content() else { return };
    let keep = ["meshes/base_anim.nif", "meshes/base_anim_female.nif", "meshes/base_anim.1st.nif", "meshes/r/guar.nif", "meshes/r/skeleton.nif"];
    let sets = anim::actor_sets(&c.db, &c.vfs).filtered(|s| keep.contains(&s));
    assert!(sets.sets.len() >= 4, "{:?}", sets.sets.keys());
    let run = |threads: usize| -> BTreeMap<ProductKey, sf_bake::Digest> {
        let dir = tempfile::tempdir().unwrap();
        let pool = rayon::ThreadPoolBuilder::new().num_threads(threads).build().unwrap();
        let (report, manifest, _) = pool.install(|| bake(&c, dir.path(), &sets));
        assert!(report.failures.is_empty(), "{:?}", report.failures);
        manifest.products
    };
    let t = Instant::now();
    let one = run(1);
    let one_time = t.elapsed();
    let t = Instant::now();
    let many = run(0);
    eprintln!("{} products: 1 thread {one_time:.2?}, all threads {:.2?}", one.len(), t.elapsed());
    assert_eq!(one, many);
}
