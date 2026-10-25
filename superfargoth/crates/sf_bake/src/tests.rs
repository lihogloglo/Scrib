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
