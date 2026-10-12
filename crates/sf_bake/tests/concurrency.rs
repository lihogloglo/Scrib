//! Regression test for the bake deadlock of 2026-10-12 (bake.md "Concurrency").
//!
//! Before the fix, a product's slot was a `OnceLock` whose initializer ran a
//! nested rayon `par_iter` over the planned inputs. While that worker waited
//! for the stolen half, rayon let it steal an unrelated top-level job; when
//! that job needed the product the worker was still initializing further down
//! its own stack, `get_or_init` re-entered and blocked forever. Slow input
//! fingerprints, shared dependencies, steps with internal rayon work and
//! dynamic (unplanned) product reads make that interleaving likely here.
//!
//! The bake runs on a helper thread under a watchdog, so a regression fails
//! with the event log instead of hanging CI.

use std::collections::{BTreeMap, HashMap};
use std::fmt::Write as _;
use std::sync::mpsc;
use std::sync::{Arc, Mutex, PoisonError};
use std::thread;
use std::time::Duration;

use rayon::prelude::*;
use sf_bake::{BakeError, Baker, Digest, Inputs, PlanCx, ProductKey, RunCx, Step, Store};

/// Files `f<n>`; fingerprints of `slow*` files take a little while, so the
/// worker that plans a product waits on the stolen half of its `par_iter`.
struct Files;
impl Inputs for Files {
    fn file_fingerprint(&self, path: &str) -> Result<Option<Digest>, BakeError> {
        if path.starts_with("slow") {
            thread::sleep(Duration::from_millis(2));
        }
        Ok(Some(Digest::of(path.as_bytes())))
    }
    fn read_file(&self, path: &str) -> Result<Vec<u8>, BakeError> {
        Ok(path.as_bytes().to_vec())
    }
    fn record_fingerprint(&self, _: &str, _: &str) -> Result<Option<Digest>, BakeError> {
        Ok(None)
    }
}

/// What happened, for the watchdog's report. With `SF_BAKE_STACKS=1`, also
/// each worker's stack at its last `plan` (diagnosis only: slow).
#[derive(Default)]
struct Log(Mutex<Vec<String>>, Mutex<BTreeMap<String, String>>);
impl Log {
    fn note(&self, event: String) {
        let t = rayon::current_thread_index().map_or_else(|| "main".to_owned(), |i| format!("t{i}"));
        if event.starts_with("plan") && std::env::var_os("SF_BAKE_STACKS").is_some() {
            let frames: Vec<String> = std::backtrace::Backtrace::force_capture()
                .to_string()
                .lines()
                .map(str::trim)
                .filter(|l| l.contains("sf_bake::") || l.contains("get_or_init") || l.contains("steal") || l.contains("join_context"))
                .map(str::to_owned)
                .collect();
            self.1.lock().unwrap_or_else(PoisonError::into_inner).insert(t.clone(), format!("{event}
    {}", frames.join("
    ")));
        }
        self.0.lock().unwrap_or_else(PoisonError::into_inner).push(format!("{t}: {event}"));
    }
}

/// `leaf(n)`: two slow files, internal rayon work.
struct Leaf(Arc<Log>);
impl Step for Leaf {
    fn id(&self) -> &'static str {
        "leaf"
    }
    fn version(&self) -> u32 {
        1
    }
    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        self.0.note(format!("plan leaf({subject})"));
        cx.file(format!("slow-a-{subject}")).file(format!("slow-b-{subject}"));
        Ok(())
    }
    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        self.0.note(format!("run leaf({subject})"));
        let mut bytes = cx.read_file(&format!("slow-a-{subject}"))?;
        // A step that parallelizes its own work (like terrain layer arrays).
        let sum: u64 = (0..64u64).into_par_iter().map(|i| i * i + bytes.len() as u64).sum();
        bytes.extend_from_slice(&sum.to_le_bytes());
        Ok(bytes)
    }
}

/// `mid(n)`: plans `leaf(n)` and `leaf(n+1)`, two slow files.
struct Mid(Arc<Log>);
impl Step for Mid {
    fn id(&self) -> &'static str {
        "mid"
    }
    fn version(&self) -> u32 {
        1
    }
    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        self.0.note(format!("plan mid({subject})"));
        let n: u32 = subject.parse().map_err(|e| BakeError::Other(format!("{e}")))?;
        cx.product("leaf", n.to_string()).product("leaf", (n + 1).to_string());
        cx.file(format!("slow-m-{subject}")).file(format!("slow-n-{subject}"));
        Ok(())
    }
    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        self.0.note(format!("run mid({subject})"));
        let n: u32 = subject.parse().map_err(|e| BakeError::Other(format!("{e}")))?;
        let mut out = cx.product("leaf", &n.to_string())?;
        out.extend(cx.product("leaf", &(n + 1).to_string())?);
        Ok(out)
    }
}

/// `top(n)`: plans `mid(n)`, reads `mid(n+1)` and `leaf(n+3)` dynamically.
struct Top(Arc<Log>);
impl Step for Top {
    fn id(&self) -> &'static str {
        "top"
    }
    fn version(&self) -> u32 {
        1
    }
    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        self.0.note(format!("plan top({subject})"));
        cx.product("mid", subject).file(format!("slow-t-{subject}"));
        Ok(())
    }
    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        self.0.note(format!("run top({subject})"));
        let n: u32 = subject.parse().map_err(|e| BakeError::Other(format!("{e}")))?;
        let mut out = cx.product("mid", subject)?;
        out.extend(cx.product("mid", &(n + 1).to_string())?);
        out.extend(cx.product("leaf", &(n + 3).to_string())?);
        Ok(Digest::of(&out).0.to_vec())
    }
}

/// Every product, dependents interleaved with their dependencies so rayon
/// splits put them in different jobs.
fn keys(n: u32) -> Vec<ProductKey> {
    (0..n)
        .flat_map(|i| {
            let s = i.to_string();
            [ProductKey::new("top", s.clone()), ProductKey::new("leaf", s.clone()), ProductKey::new("mid", s)]
        })
        .collect()
}

/// Bakes the graph once in a fresh store on `threads` workers; the manifest.
fn bake_once(threads: usize, log: &Arc<Log>) -> Vec<(ProductKey, Digest)> {
    let dir = tempfile::tempdir().unwrap();
    let mut baker = Baker::new(Store::open(dir.path()).unwrap(), Arc::new(Files));
    baker.register(Leaf(log.clone())).register(Mid(log.clone())).register(Top(log.clone()));
    let pool = rayon::ThreadPoolBuilder::new().num_threads(threads).build().unwrap();
    let report = pool.install(|| baker.bake(&keys(24)));
    assert_eq!(report.failures().count(), 0, "{:?}", report.failures().collect::<Vec<_>>());
    let mut products: Vec<(ProductKey, Digest)> = baker.manifest().products.into_iter().collect();
    products.sort();
    products
}

/// Runs `f` on a helper thread; fails (with `log`) if it takes longer than `limit`.
fn with_watchdog<T: Send + 'static>(limit: Duration, log: &Arc<Log>, f: impl FnOnce() -> T + Send + 'static) -> T {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let _ = tx.send(f());
    });
    match rx.recv_timeout(limit) {
        Ok(value) => return value,
        Err(mpsc::RecvTimeoutError::Disconnected) => panic!("the bake thread panicked (see its output above)"),
        Err(mpsc::RecvTimeoutError::Timeout) => {}
    }
    let events = log.0.lock().unwrap_or_else(PoisonError::into_inner);
    let mut report = String::new();
    for e in events.iter().rev().take(40).collect::<Vec<_>>().into_iter().rev() {
        let _ = writeln!(report, "  {e}");
    }
    for (t, stack) in log.1.lock().unwrap_or_else(PoisonError::into_inner).iter() {
        let _ = writeln!(report, "{t} last plan: {stack}");
    }
    panic!("bake did not finish in {limit:?} (deadlock); last events:\n{report}");
}

#[test]
fn nested_parallel_bake_never_deadlocks_and_is_deterministic() {
    let log = Arc::new(Log::default());
    let reference = with_watchdog(Duration::from_secs(30), &log, {
        let log = log.clone();
        move || bake_once(1, &log)
    });
    let mut seen: HashMap<usize, usize> = HashMap::new();
    for round in 0..12 {
        let threads = [2, 3, 4, 8][round % 4];
        let log = Arc::new(Log::default());
        let products = with_watchdog(Duration::from_secs(30), &log, {
            let log = log.clone();
            move || bake_once(threads, &log)
        });
        assert_eq!(products, reference, "{threads} threads differ from 1 thread");
        *seen.entry(threads).or_default() += 1;
    }
    assert_eq!(seen.values().sum::<usize>(), 12);
}
