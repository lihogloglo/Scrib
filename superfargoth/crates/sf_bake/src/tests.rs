use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};

use super::*;

/// In-memory host: files and record fingerprints the tests can edit.
#[derive(Default)]
struct World {
    files: RwLock<HashMap<String, Vec<u8>>>,
    records: RwLock<HashMap<(String, String), Digest>>,
}

impl World {
    fn set(&self, path: &str, content: &str) {
        self.files.write().unwrap().insert(path.to_owned(), content.as_bytes().to_vec());
    }
    fn remove(&self, path: &str) {
        self.files.write().unwrap().remove(path);
    }
    fn set_record(&self, kind: &str, id: &str, value: &str) {
        self.records.write().unwrap().insert((kind.to_owned(), id.to_owned()), Digest::of(value.as_bytes()));
    }
}

impl Inputs for World {
    fn file_fingerprint(&self, path: &str) -> Result<Option<Digest>, BakeError> {
        Ok(self.files.read().unwrap().get(path).map(|b| Digest::of(b)))
    }
    fn read_file(&self, path: &str) -> Result<Vec<u8>, BakeError> {
        self.files.read().unwrap().get(path).cloned().ok_or_else(|| BakeError::MissingInput(Input::File(path.into())))
    }
    fn record_fingerprint(&self, kind: &str, id: &str) -> Result<Option<Digest>, BakeError> {
        Ok(self.records.read().unwrap().get(&(kind.to_owned(), id.to_owned())).copied())
    }
}

type Runs = Arc<AtomicUsize>;

/// Concatenates the comma-separated files named by the subject.
struct Concat(Runs);
impl Step for Concat {
    fn id(&self) -> &'static str {
        "concat"
    }
    fn version(&self) -> u32 {
        1
    }
    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        for path in subject.split(',') {
            cx.file(path);
        }
        Ok(())
    }
    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        self.0.fetch_add(1, Ordering::SeqCst);
        let mut out = Vec::new();
        for path in subject.split(',') {
            out.extend(cx.read_file(path)?);
        }
        Ok(out)
    }
}

/// Uppercases a `concat` product: a product-on-product dependency.
struct Upper(Runs);
impl Step for Upper {
    fn id(&self) -> &'static str {
        "upper"
    }
    fn version(&self) -> u32 {
        1
    }
    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        cx.product("concat", subject);
        Ok(())
    }
    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        self.0.fetch_add(1, Ordering::SeqCst);
        Ok(cx.product("concat", subject)?.to_ascii_uppercase())
    }
}

/// Follows `include <path>` lines: dependencies discovered while running.
struct Include(Runs);
impl Step for Include {
    fn id(&self) -> &'static str {
        "include"
    }
    fn version(&self) -> u32 {
        1
    }
    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        self.0.fetch_add(1, Ordering::SeqCst);
        let text = String::from_utf8(cx.read_file(subject)?).unwrap();
        let mut out = String::new();
        for line in text.lines() {
            match line.strip_prefix("include ") {
                Some(path) => out.push_str(&String::from_utf8(cx.read_file(path)?).unwrap()),
                None => out.push_str(line),
            }
        }
        Ok(out.into_bytes())
    }
}

/// Depends on a record and a parameter, with a configurable version.
struct FromRecord {
    runs: Runs,
    version: u32,
    quality: u8,
}
impl Step for FromRecord {
    fn id(&self) -> &'static str {
        "from_record"
    }
    fn version(&self) -> u32 {
        self.version
    }
    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        cx.record("npc", subject).param("quality", vec![self.quality]);
        Ok(())
    }
    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        self.runs.fetch_add(1, Ordering::SeqCst);
        let fp = cx.record("npc", subject)?;
        Ok([fp.0.as_slice(), &[self.quality]].concat())
    }
}

/// A product that depends on itself.
struct Loop;
impl Step for Loop {
    fn id(&self) -> &'static str {
        "loop"
    }
    fn version(&self) -> u32 {
        1
    }
    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        cx.product("loop", subject);
        Ok(())
    }
    fn run(&self, _: &str, _: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        unreachable!("a cycle never runs")
    }
}

/// Derived output unless an authored file overrides it.
struct Overridable(Runs);
impl Step for Overridable {
    fn id(&self) -> &'static str {
        "lod"
    }
    fn version(&self) -> u32 {
        1
    }
    fn override_path(&self, subject: &str) -> Option<String> {
        Some(format!("authored/{subject}"))
    }
    fn run(&self, _: &str, _: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        self.0.fetch_add(1, Ordering::SeqCst);
        Ok(b"derived".to_vec())
    }
}

struct Fixture {
    dir: tempfile::TempDir,
    world: Arc<World>,
    concat: Runs,
    upper: Runs,
    include: Runs,
    record: Runs,
    lod: Runs,
}

impl Fixture {
    fn new() -> Self {
        let world = Arc::new(World::default());
        world.set("a", "alpha ");
        world.set("b", "beta ");
        world.set("c", "gamma ");
        world.set_record("npc", "guard", "v1");
        Self {
            dir: tempfile::tempdir().unwrap(),
            world,
            concat: Runs::default(),
            upper: Runs::default(),
            include: Runs::default(),
            record: Runs::default(),
            lod: Runs::default(),
        }
    }

    /// A fresh session over the same store, as after a restart.
    fn baker(&self) -> Baker {
        self.baker_with(1, 7)
    }

    fn baker_with(&self, version: u32, quality: u8) -> Baker {
        let mut b = Baker::new(Store::open(self.dir.path()).unwrap(), self.world.clone());
        b.register(Concat(self.concat.clone()))
            .register(Upper(self.upper.clone()))
            .register(Include(self.include.clone()))
            .register(FromRecord { runs: self.record.clone(), version, quality })
            .register(Loop)
            .register(Overridable(self.lod.clone()));
        b
    }
}

fn get(baker: &Baker, step: &str, subject: &str) -> Vec<u8> {
    let d = baker.product(&ProductKey::new(step, subject)).unwrap();
    baker.store().get(&d).unwrap()
}

fn runs(r: &Runs) -> usize {
    r.load(Ordering::SeqCst)
}

#[test]
fn second_session_hits_everything() {
    let f = Fixture::new();
    assert_eq!(get(&f.baker(), "upper", "a,b"), b"ALPHA BETA ");
    assert_eq!((runs(&f.concat), runs(&f.upper)), (1, 1));

    let baker = f.baker();
    assert_eq!(get(&baker, "upper", "a,b"), b"ALPHA BETA ");
    assert_eq!((runs(&f.concat), runs(&f.upper)), (1, 1), "nothing reruns");
    assert_eq!(baker.step_stats()["upper"].hits, 1);
}

#[test]
fn changing_a_file_rebuilds_exactly_its_dependents() {
    let f = Fixture::new();
    let b = f.baker();
    get(&b, "upper", "a,b");
    get(&b, "concat", "c");
    assert_eq!(runs(&f.concat), 2);

    f.world.set("a", "ALEPH ");
    let b = f.baker();
    assert_eq!(get(&b, "upper", "a,b"), b"ALEPH BETA ");
    get(&b, "concat", "c");
    assert_eq!(runs(&f.concat), 3, "concat(a,b) reran, concat(c) did not");
    assert_eq!(runs(&f.upper), 2);
}

#[test]
fn unchanged_product_output_stops_propagation() {
    let f = Fixture::new();
    get(&f.baker(), "upper", "a,b");
    // Same bytes rewritten: concat's fingerprint input is unchanged.
    f.world.set("a", "alpha ");
    get(&f.baker(), "upper", "a,b");
    assert_eq!((runs(&f.concat), runs(&f.upper)), (1, 1));
}

#[test]
fn records_params_and_versions_invalidate() {
    let f = Fixture::new();
    get(&f.baker(), "from_record", "guard");
    get(&f.baker(), "from_record", "guard");
    assert_eq!(runs(&f.record), 1);

    f.world.set_record("npc", "guard", "v2");
    get(&f.baker(), "from_record", "guard");
    assert_eq!(runs(&f.record), 2, "record change");

    get(&f.baker_with(1, 9), "from_record", "guard");
    assert_eq!(runs(&f.record), 3, "param change");

    get(&f.baker_with(2, 9), "from_record", "guard");
    assert_eq!(runs(&f.record), 4, "version bump");
}

#[test]
fn dynamic_dependencies_are_tracked() {
    let f = Fixture::new();
    f.world.set("main", "include part\nend");
    f.world.set("part", "one ");
    assert_eq!(get(&f.baker(), "include", "main"), b"one end");
    assert_eq!(get(&f.baker(), "include", "main"), b"one end");
    assert_eq!(runs(&f.include), 1);

    f.world.set("part", "two ");
    assert_eq!(get(&f.baker(), "include", "main"), b"two end");
    assert_eq!(runs(&f.include), 2, "a file discovered at run time invalidates");
}

#[test]
fn cycles_are_errors() {
    let f = Fixture::new();
    let err = f.baker().product(&ProductKey::new("loop", "x")).unwrap_err();
    assert!(matches!(err, BakeError::Cycle(ref c) if c.len() == 2), "{err}");
}

#[test]
fn authored_overrides_win() {
    let f = Fixture::new();
    assert_eq!(get(&f.baker(), "lod", "rock"), b"derived");

    f.world.set("authored/rock", "hand made");
    assert_eq!(get(&f.baker(), "lod", "rock"), b"hand made");
    assert_eq!(runs(&f.lod), 1, "the step didn't run");
    assert_eq!(get(&f.baker(), "lod", "rock"), b"hand made");

    f.world.remove("authored/rock");
    assert_eq!(get(&f.baker(), "lod", "rock"), b"derived");
}

#[test]
fn missing_inputs_fail_without_poisoning_others() {
    let f = Fixture::new();
    let report = f.baker().bake(&[ProductKey::new("concat", "a,nope"), ProductKey::new("concat", "b")]);
    assert_eq!(report.failures().count(), 1);
    assert!(report.products.iter().any(|(k, r)| k.subject == "b" && r.is_ok()));
}

#[test]
fn identical_manifests_for_any_thread_count() {
    let f = Fixture::new();
    let other = Fixture::new();
    let keys: Vec<ProductKey> = (0..200)
        .map(|i| {
            let subject = ["a", "b", "c"].iter().cycle().skip(i % 3).take(1 + i % 3).copied().collect::<Vec<_>>();
            ProductKey::new(if i % 2 == 0 { "concat" } else { "upper" }, subject.join(","))
        })
        .collect();
    let bake_with = |fixture: &Fixture, threads: usize| {
        let pool = rayon::ThreadPoolBuilder::new().num_threads(threads).build().unwrap();
        pool.install(|| {
            let baker = fixture.baker();
            let report = baker.bake(&keys);
            assert_eq!(report.failures().count(), 0);
            baker.manifest()
        })
    };
    assert_eq!(bake_with(&f, 1), bake_with(&other, 8));
}

#[test]
fn verify_reproduces_cached_outputs() {
    let f = Fixture::new();
    let b = f.baker();
    let keys = [ProductKey::new("upper", "a,b"), ProductKey::new("include", "c")];
    for (key, ok) in b.verify(&keys) {
        assert!(ok.unwrap(), "{key}");
    }
}

#[test]
fn manifests_round_trip_and_gc_keeps_referenced_objects() {
    let f = Fixture::new();
    let b = f.baker();
    let kept = b.product(&ProductKey::new("concat", "a")).unwrap();
    let mut manifest = b.manifest();
    let dropped = b.product(&ProductKey::new("concat", "b")).unwrap();
    manifest.products.retain(|_, d| *d == kept);
    b.store().save_manifest("default", &manifest).unwrap();
    assert_eq!(b.store().load_manifest("default").unwrap(), manifest);
    assert_eq!(manifest.get("concat", "a"), Some(kept));

    let report = b.store().gc(0).unwrap();
    assert_eq!(report.deleted_objects, 1);
    assert!(b.store().contains(&kept));
    assert!(!b.store().contains(&dropped));
    // A product whose object was collected is rebuilt, not served stale.
    assert_eq!(get(&f.baker(), "concat", "b"), b"beta ");
}

#[test]
fn store_encodings_round_trip() {
    let dir = tempfile::tempdir().unwrap();
    let store = Store::open(dir.path()).unwrap();
    let data = b"repetitive repetitive repetitive repetitive data".repeat(100);
    let lz = store.put(&data, true).unwrap();
    assert_eq!(store.get(&lz).unwrap(), data);
    assert!(store.verify(&lz).unwrap());
    let raw_dir = tempfile::tempdir().unwrap();
    let raw_store = Store::open(raw_dir.path()).unwrap();
    let raw = raw_store.put(&data, false).unwrap();
    assert_eq!(raw, lz, "names depend on content, not encoding");
    assert_eq!(raw_store.get(&raw).unwrap(), data);
}

#[test]
fn file_hash_memo_reads_only_new_stats() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("memo");
    let key = StatKey { source: "mod".into(), path: "a.txt".into(), size: 3, mtime_ns: 42 };
    let reads = AtomicUsize::new(0);
    let read = || {
        reads.fetch_add(1, Ordering::SeqCst);
        Ok(b"abc".to_vec())
    };
    let memo = FileHashMemo::load(&file);
    let d = memo.fingerprint(key.clone(), read).unwrap();
    memo.save().unwrap();
    let memo = FileHashMemo::load(&file);
    assert_eq!(memo.fingerprint(key.clone(), read).unwrap(), d);
    assert_eq!(reads.load(Ordering::SeqCst), 1);
    let touched = StatKey { mtime_ns: 43, ..key };
    memo.fingerprint(touched, read).unwrap();
    assert_eq!(reads.load(Ordering::SeqCst), 2);
}

/// `NV4f`: the packed trace log and the in-memory object index (`store.rs`).
mod trace_log {
    use std::fs;
    use std::io::Write as _;

    use super::*;
    use crate::store::TRACE_LOG;

    fn key(i: usize) -> ProductKey {
        ProductKey::new("tiles", format!("tile {i}"))
    }

    fn trace(i: usize, output: &str) -> Trace {
        let dynamic = vec![(Input::File(format!("f{i}")), Digest::of(output.as_bytes()))];
        Trace::new(&key(i).subject, Digest::of(&i.to_le_bytes()), dynamic, Digest::of(output.as_bytes()))
    }

    fn output(store: &Store, i: usize) -> Option<Digest> {
        store.load_trace(&key(i)).map(|t| t.output)
    }

    #[allow(clippy::unnecessary_wraps, reason = "compared against `load_trace` results")]
    fn out(s: &str) -> Option<Digest> {
        Some(Digest::of(s.as_bytes()))
    }

    #[test]
    fn round_trip_and_last_record_wins() {
        let dir = tempfile::tempdir().unwrap();
        let store = Store::open(dir.path()).unwrap();
        assert!(store.load_trace(&key(0)).is_none());
        store.save_trace(&key(0), &trace(0, "first")).unwrap();
        store.save_trace(&key(1), &trace(1, "one")).unwrap();
        let got = store.load_trace(&key(0)).unwrap();
        assert_eq!(got.subject, "tile 0");
        assert_eq!(got.static_key, Digest::of(&0usize.to_le_bytes()));
        assert_eq!(got.dynamic, trace(0, "first").dynamic);
        assert_eq!(Some(got.output), out("first"));
        store.save_trace(&key(0), &trace(0, "second")).unwrap();
        assert_eq!(output(&store, 0), out("second"));
        assert_eq!(output(&store, 1), out("one"));
        assert_eq!(store.trace_log_counts(), (2, 3));
        // A different step with the same subject is a different key.
        assert!(store.load_trace(&ProductKey::new("other", "tile 0")).is_none());
    }

    #[test]
    fn reopen_sees_appended_traces() {
        let dir = tempfile::tempdir().unwrap();
        let store = Store::open(dir.path()).unwrap();
        store.save_trace(&key(0), &trace(0, "a")).unwrap();
        store.save_trace(&key(0), &trace(0, "b")).unwrap();
        drop(store);
        let store = Store::open(dir.path()).unwrap();
        assert_eq!(output(&store, 0), out("b"));
        store.save_trace(&key(1), &trace(1, "c")).unwrap();
        drop(store);
        let store = Store::open(dir.path()).unwrap();
        assert_eq!(output(&store, 0), out("b"));
        assert_eq!(output(&store, 1), out("c"));
        assert_eq!(store.trace_log_counts(), (2, 3));
        assert!(!dir.path().join("traces").exists(), "no per-file traces are written");
    }

    #[test]
    fn compaction_keeps_live_traces_only_and_survives_reopen() {
        let dir = tempfile::tempdir().unwrap();
        let store = Store::open(dir.path()).unwrap();
        for round in 0..4 {
            store.save_trace(&key(0), &trace(0, &format!("zero {round}"))).unwrap();
        }
        store.save_trace(&key(1), &trace(1, "one")).unwrap();
        assert_eq!(store.trace_log_counts(), (2, 5));
        let before = fs::metadata(dir.path().join(TRACE_LOG)).unwrap().len();
        drop(store);

        // 3 dead of 5 records: the load rewrites the log.
        let store = Store::open(dir.path()).unwrap();
        assert_eq!(store.trace_log_counts(), (2, 2));
        assert_eq!(output(&store, 0), out("zero 3"));
        assert_eq!(output(&store, 1), out("one"));
        let after = fs::metadata(dir.path().join(TRACE_LOG)).unwrap().len();
        assert!(after < before, "{after} < {before}");
        store.save_trace(&key(2), &trace(2, "two")).unwrap();
        drop(store);

        let store = Store::open(dir.path()).unwrap();
        assert_eq!(store.trace_log_counts(), (3, 3));
        assert_eq!(output(&store, 0), out("zero 3"));
        assert_eq!(output(&store, 1), out("one"));
        assert_eq!(output(&store, 2), out("two"));
        let leftovers: Vec<String> = fs::read_dir(dir.path())
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
            .filter(|n| n.contains(".tmp"))
            .collect();
        assert!(leftovers.is_empty(), "{leftovers:?}");
    }

    #[test]
    fn torn_last_record_is_skipped_and_cut_off() {
        let dir = tempfile::tempdir().unwrap();
        let store = Store::open(dir.path()).unwrap();
        store.save_trace(&key(0), &trace(0, "a")).unwrap();
        store.save_trace(&key(1), &trace(1, "b")).unwrap();
        drop(store);
        let path = dir.path().join(TRACE_LOG);
        let good = fs::metadata(&path).unwrap().len();
        // A crash mid-append: a header promising 200 bytes, then only some of them.
        let mut file = fs::OpenOptions::new().append(true).open(&path).unwrap();
        file.write_all(&200u32.to_le_bytes()).unwrap();
        file.write_all(&[0xAB; 8 + 50]).unwrap();
        drop(file);

        let store = Store::open(dir.path()).unwrap();
        assert_eq!(output(&store, 0), out("a"));
        assert_eq!(output(&store, 1), out("b"));
        assert_eq!(store.trace_log_counts(), (2, 2));
        assert_eq!(fs::metadata(&path).unwrap().len(), good, "torn tail cut off");
        store.save_trace(&key(2), &trace(2, "c")).unwrap();
        drop(store);

        let store = Store::open(dir.path()).unwrap();
        assert_eq!(output(&store, 2), out("c"), "appends after the cut are readable");
        assert_eq!(store.trace_log_counts(), (3, 3));
    }

    #[test]
    fn corrupt_last_record_is_not_served() {
        let dir = tempfile::tempdir().unwrap();
        let store = Store::open(dir.path()).unwrap();
        store.save_trace(&key(0), &trace(0, "a")).unwrap();
        store.save_trace(&key(1), &trace(1, "b")).unwrap();
        drop(store);
        let path = dir.path().join(TRACE_LOG);
        let mut bytes = fs::read(&path).unwrap();
        let last = bytes.len() - 1;
        bytes[last] ^= 0xFF;
        fs::write(&path, &bytes).unwrap();

        let store = Store::open(dir.path()).unwrap();
        assert_eq!(output(&store, 0), out("a"));
        assert_eq!(output(&store, 1), None, "a checksum mismatch counts as torn");
    }

    #[test]
    fn legacy_per_file_traces_are_found_and_migrated() {
        let dir = tempfile::tempdir().unwrap();
        let store = Store::open(dir.path()).unwrap();
        store.save_trace(&key(0), &trace(0, "legacy zero")).unwrap();
        store.save_trace(&key(1), &trace(1, "legacy one")).unwrap();
        store.downgrade_to_legacy_traces();
        assert!(!dir.path().join(TRACE_LOG).exists());
        assert_eq!(fs::read_dir(dir.path().join("traces/tiles")).unwrap().count(), 2);

        let store = Store::open(dir.path()).unwrap();
        assert_eq!(output(&store, 0), out("legacy zero"));
        assert_eq!(store.trace_log_counts(), (1, 1), "the legacy trace is copied into the log");
        assert!(store.load_trace(&ProductKey::new("no-legacy-dir", "tile 0")).is_none());
        drop(store);

        // Once migrated, the log alone serves it.
        fs::remove_dir_all(dir.path().join("traces")).unwrap();
        let store = Store::open(dir.path()).unwrap();
        assert_eq!(output(&store, 0), out("legacy zero"));
        assert_eq!(output(&store, 1), None, "never read, so never migrated");
    }

    #[test]
    fn legacy_store_keeps_its_cache_through_a_bake() {
        let f = Fixture::new();
        let keys = [ProductKey::new("concat", "a"), ProductKey::new("upper", "a,b"), ProductKey::new("include", "c")];
        let b = f.baker();
        assert_eq!(b.bake(&keys).failures().count(), 0);
        let store = b.store().clone();
        drop(b);
        store.downgrade_to_legacy_traces();

        for pass in ["migrating", "from the log"] {
            let b = f.baker();
            assert_eq!(b.bake(&keys).failures().count(), 0);
            let stats = b.step_stats();
            let built: u32 = stats.values().map(|s| s.built).sum();
            let hits: u32 = stats.values().map(|s| s.hits).sum();
            assert_eq!(built, 0, "{pass}: nothing rebuilt");
            assert!(hits >= 3, "{pass}: {hits} hits");
        }
        assert!(Store::open(f.dir.path()).unwrap().trace_log_counts().0 >= 3);
    }

    #[test]
    fn contains_after_put_and_after_reopen() {
        let dir = tempfile::tempdir().unwrap();
        let store = Store::open(dir.path()).unwrap();
        let missing = Digest::of(b"never stored");
        assert!(!store.contains(&missing));
        let d = store.put(b"some object", false).unwrap();
        assert!(store.contains(&d));
        assert!(!store.contains(&missing));
        drop(store);
        let store = Store::open(dir.path()).unwrap();
        assert!(store.contains(&d));
        assert!(!store.contains(&missing));
        // Stored through another handle (another process) after this index was listed.
        let other = Store::open(dir.path()).unwrap();
        let late = other.put(b"late object", true).unwrap();
        assert!(store.contains(&late));
        assert_eq!(store.get(&late).unwrap(), b"late object");
    }

    #[test]
    fn parallel_saves_and_loads_agree() {
        let dir = tempfile::tempdir().unwrap();
        let store = Store::open(dir.path()).unwrap();
        (0..2000).into_par_iter().for_each(|i| {
            store.save_trace(&key(i), &trace(i, &format!("out {i}"))).unwrap();
            assert_eq!(output(&store, i), out(&format!("out {i}")));
        });
        drop(store);
        let store = Store::open(dir.path()).unwrap();
        assert_eq!(store.trace_log_counts(), (2000, 2000));
        let ok = (0..2000).into_par_iter().filter(|&i| output(&store, i) == out(&format!("out {i}"))).count();
        assert_eq!(ok, 2000);
    }
}

/// NV4e-diag: what a warm re-plan of many products with many dynamic product inputs costs, split
/// into the parts of [`Baker::hit`]. Synthetic store shaped like `nav.tiles`: `N_TOP` products with
/// no planned products and `DYN_PER_TOP` dynamic reads each, of `N_LEAF` shared leaf products.
mod hit_cost {
    use std::time::Instant;

    use super::*;

    const N_TOP: usize = 20_000;
    const N_LEAF: usize = 4_000;
    const DYN_PER_TOP: usize = 20;

    fn leaf_subject(i: usize) -> String {
        format!("meshes/x/some_model_{i:05}.nif")
    }

    struct Leaf;
    impl Step for Leaf {
        fn id(&self) -> &'static str {
            "leaf.cook"
        }
        fn version(&self) -> u32 {
            1
        }
        fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
            cx.file(subject);
            Ok(())
        }
        fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
            cx.read_file(subject)
        }
    }

    struct Top;
    impl Step for Top {
        fn id(&self) -> &'static str {
            "top.tiles"
        }
        fn version(&self) -> u32 {
            1
        }
        fn plan(&self, _: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
            cx.param("class", vec![7; 64]);
            Ok(())
        }
        fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
            let j: usize = subject.parse().map_err(|_| BakeError::Other("subject".into()))?;
            let mut out = Vec::new();
            for k in 0..DYN_PER_TOP {
                out.extend_from_slice(&cx.product_digest("leaf.cook", &leaf_subject((j * 7 + k * 131) % N_LEAF))?.0);
            }
            Ok(out)
        }
    }

    fn baker(dir: &std::path::Path, world: &Arc<World>) -> Baker {
        let mut b = Baker::new(Store::open(dir).unwrap(), world.clone());
        b.register(Leaf).register(Top);
        b
    }

    #[allow(clippy::cast_precision_loss, reason = "printed measurements")]
    fn per_call_us(wall: Duration, calls: usize) -> f64 {
        let threads = rayon::current_num_threads();
        wall.as_secs_f64() * 1e6 * threads as f64 / calls.max(1) as f64
    }

    #[test]
    #[ignore = "measurement: bake hit cost"]
    fn warm_replan_split() {
        let dir = tempfile::tempdir().unwrap();
        let world = Arc::new(World::default());
        for i in 0..N_LEAF {
            world.set(&leaf_subject(i), &format!("leaf content {i}"));
        }
        let keys: Vec<ProductKey> = (0..N_TOP).map(|j| ProductKey::new("top.tiles", j.to_string())).collect();
        let threads = rayon::current_num_threads();

        let t = Instant::now();
        let r = baker(dir.path(), &world).bake(&keys);
        assert_eq!(r.failures().count(), 0);
        println!("threads {threads}; cold bake {N_TOP} x {DYN_PER_TOP} dyn over {N_LEAF} leaves: {:?}", t.elapsed());

        for round in 0..2 {
            let b = baker(dir.path(), &world);
            let t = Instant::now();
            let r = b.bake(&keys);
            let wall = t.elapsed();
            assert_eq!(r.failures().count(), 0);
            let s = b.step_stats();
            println!(
                "warm re-plan #{round}: {wall:?} = {:.1} us/top-hit x threads; top (hit, built, dup) {:?}; leaf {:?}",
                per_call_us(wall, N_TOP),
                (s["top.tiles"].hits, s["top.tiles"].built, s["top.tiles"].duplicates),
                (s["leaf.cook"].hits, s["leaf.cook"].built, s["leaf.cook"].duplicates),
            );
        }

        // The split, each phase over all top products in parallel like the scheduler, on a fresh
        // session (empty memo) so leaf resolution is counted where the real hit path pays it.
        let b = baker(dir.path(), &world);
        let store = b.store().clone();
        let t = Instant::now();
        let traces: Vec<Trace> = keys.par_iter().map(|k| store.load_trace(k).unwrap()).collect();
        let load = t.elapsed();
        let t = Instant::now();
        let present = traces.par_iter().filter(|tr| store.contains(&tr.output)).count();
        let contains = t.elapsed();
        assert_eq!(present, N_TOP);
        let n_dyn: usize = traces.iter().map(|tr| tr.dynamic.len()).sum();
        let fp_pass = || {
            let t = Instant::now();
            let ok = traces
                .par_iter()
                .filter(|tr| tr.dynamic.iter().all(|(input, fp)| b.fingerprint(input, &[]).ok() == Some(*fp)))
                .count();
            assert_eq!(ok, N_TOP);
            t.elapsed()
        };
        let fp_first = fp_pass();
        let leaf_first = b.step_stats()["leaf.cook"];
        let fp_memo = fp_pass();
        let log_bytes = std::fs::metadata(dir.path().join(store::TRACE_LOG)).unwrap().len();
        let (_, records) = store.trace_log_counts();
        println!(
            "load_trace: {load:?} ({:.1} us/call x threads; trace log {log_bytes} B, avg record {} B)",
            per_call_us(load, N_TOP),
            log_bytes / records.max(1) as u64
        );
        println!("contains:   {contains:?} ({:.1} us/call x threads)", per_call_us(contains, N_TOP));
        println!(
            "fingerprint, first touch (resolves {} leaves: hits {}, dups {}): {fp_first:?} ({:.2} us/input x threads)",
            leaf_first.hits + leaf_first.built,
            leaf_first.hits,
            leaf_first.duplicates,
            per_call_us(fp_first, n_dyn)
        );
        println!("fingerprint, memo (slot hit): {fp_memo:?} ({:.2} us/input x threads)", per_call_us(fp_memo, n_dyn));
    }

    /// Reads a sample of the real store's `nav.tiles` traces: per-file read cost (first read in
    /// this process, then again) and dynamic inputs per tile by step. `SF_BAKE_STORE` is the bake
    /// root (e.g. `%LOCALAPPDATA%/bevyrrowind/bake`); `SF_BAKE_SAMPLE` files (default 3000).
    #[test]
    #[ignore = "measurement: real nav.tiles traces"]
    #[allow(clippy::cast_precision_loss, reason = "printed measurements")]
    fn real_nav_tile_traces() {
        let Ok(root) = std::env::var("SF_BAKE_STORE") else {
            println!("SF_BAKE_STORE not set");
            return;
        };
        let sample: usize = std::env::var("SF_BAKE_SAMPLE").ok().and_then(|s| s.parse().ok()).unwrap_or(3000);
        let root = std::path::PathBuf::from(root);
        let store = Store::open(&root).unwrap();
        let mut paths: Vec<_> =
            std::fs::read_dir(root.join("traces/nav.tiles")).unwrap().map(|e| e.unwrap().path()).collect();
        paths.sort();
        if std::env::var("SF_BAKE_FULL").is_ok() {
            // The whole warm hit path's file work for every tile, at the scheduler's parallelism.
            let t = Instant::now();
            let traces: Vec<Trace> =
                paths.par_iter().map(|p| postcard::from_bytes(&std::fs::read(p).unwrap()).unwrap()).collect();
            let read = t.elapsed();
            let t = Instant::now();
            let present = traces.par_iter().filter(|tr| store.contains(&tr.output)).count();
            println!(
                "full parallel ({} threads): {} trace reads {read:?}, contains {:?} ({present} present)",
                rayon::current_num_threads(),
                traces.len(),
                t.elapsed()
            );
        }
        let stride = (paths.len() / sample.max(1)).max(1);
        let paths: Vec<_> = paths.into_iter().step_by(stride).take(sample).collect();
        for pass in ["first", "again"] {
            let t = Instant::now();
            let mut slow = 0;
            let mut worst = Duration::ZERO;
            let mut traces = Vec::new();
            for p in &paths {
                let t1 = Instant::now();
                let bytes = std::fs::read(p).unwrap();
                let d = t1.elapsed();
                worst = worst.max(d);
                slow += usize::from(d > Duration::from_millis(1));
                traces.push(postcard::from_bytes::<Trace>(&bytes).unwrap());
            }
            let read = t.elapsed();
            let t = Instant::now();
            let present = traces.iter().filter(|tr| store.contains(&tr.output)).count();
            let contains = t.elapsed();
            println!(
                "{pass}: {} trace reads serial {read:?} ({:.0} us each, {slow} over 1 ms, worst {worst:?}); \
                 contains {contains:?} ({:.0} us each, {present} present)",
                paths.len(),
                read.as_secs_f64() * 1e6 / paths.len() as f64,
                contains.as_secs_f64() * 1e6 / paths.len() as f64,
            );
            if pass == "first" {
                let mut by_step: BTreeMap<String, usize> = BTreeMap::new();
                let mut other = 0;
                for tr in &traces {
                    for (input, _) in &tr.dynamic {
                        match input {
                            Input::Product(k) => *by_step.entry(k.step.clone()).or_default() += 1,
                            _ => other += 1,
                        }
                    }
                }
                let n: usize = traces.iter().map(|t| t.dynamic.len()).sum();
                let max = traces.iter().map(|t| t.dynamic.len()).max().unwrap_or(0);
                let per: Vec<String> =
                    by_step.iter().map(|(s, c)| format!("{s} {:.1}", *c as f64 / traces.len() as f64)).collect();
                println!(
                    "dynamic inputs per tile: avg {:.1}, max {max}; by step: {}; non-product {other}",
                    n as f64 / traces.len() as f64,
                    per.join(", ")
                );
            }
        }
    }
}
