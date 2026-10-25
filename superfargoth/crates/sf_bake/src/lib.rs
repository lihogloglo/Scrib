//! The bake pipeline: every expensive derived asset (cooked meshes, LODs,
//! terrain tiles, the world index, …) is a *product* made by a *step*,
//! cached by the content of everything it read.
//!
//! The model is a build system with constructive traces and dynamic
//! dependencies (Mokhov, Mitchell, Peyton Jones, *Build Systems à la Carte*,
//! 2018) over a content-addressed store:
//!
//! - A step's `plan` names the inputs it knows up front; `run` may read more
//!   through its [`RunCx`], and those reads are recorded.
//! - A product is reused when the hash of its planned inputs matches the
//!   last trace and every input read during that run still has the same
//!   fingerprint. Otherwise the step runs again.
//! - Outputs are stored by content hash and shared across mod sets; each
//!   profile keeps a [`Manifest`] of the products it uses.
//!
//! The crate is engine-agnostic: hosts supply file and record fingerprints
//! through [`Inputs`], and steps hold whatever handles they need to read
//! domain data. Spec: `docs/specs/bake.md`.

mod digest;
mod memo;
mod store;

pub use digest::Digest;
pub use memo::{FileHashMemo, StatKey};
pub use store::{GcReport, Manifest, Store};

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, OnceLock, PoisonError};
use std::time::{Duration, Instant};

use digest::KeyHasher;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use store::Trace;

/// Names one product: which step makes it, from which subject.
///
/// Subjects are step-defined strings (a model path, `"ws/0/3/-2"` for a
/// terrain tile), which keeps steps object-safe and traces readable.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ProductKey {
    pub step: String,
    pub subject: String,
}

impl ProductKey {
    pub fn new(step: impl Into<String>, subject: impl Into<String>) -> Self {
        Self { step: step.into(), subject: subject.into() }
    }
}

impl fmt::Debug for ProductKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.step, self.subject)
    }
}

impl fmt::Display for ProductKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

/// Something a product depends on.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Input {
    /// A file as the host resolves it (for example, the winning VFS file).
    File(String),
    /// A record, fingerprinted by the host's per-kind fingerprint function.
    Record { kind: String, id: String },
    /// Another product.
    Product(ProductKey),
}

/// Host-provided access to source data and its fingerprints.
///
/// Fingerprints must change whenever the content changes. Hosts usually
/// memoize file hashes by stat ([`FileHashMemo`]) so warm starts hash nothing.
pub trait Inputs: Send + Sync {
    /// The file's fingerprint, or `None` if it doesn't exist.
    fn file_fingerprint(&self, path: &str) -> Result<Option<Digest>, BakeError>;
    fn read_file(&self, path: &str) -> Result<Vec<u8>, BakeError>;
    /// The record's fingerprint, or `None` if it doesn't exist.
    fn record_fingerprint(&self, kind: &str, id: &str) -> Result<Option<Digest>, BakeError>;
}

/// A kind of product. Implementations must be deterministic: the same
/// inputs give bit-identical output, whatever the thread count.
pub trait Step: Send + Sync + 'static {
    /// Stable identifier, e.g. `"mesh.cook"`.
    fn id(&self) -> &'static str;
    /// Bump to invalidate every product of this step.
    fn version(&self) -> u32;
    /// Declares the inputs and parameters known before running. Cheap.
    fn plan(&self, _subject: &str, _cx: &mut PlanCx) -> Result<(), BakeError> {
        Ok(())
    }
    /// Produces the output. Reads go through `cx` so they are tracked.
    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError>;
    /// An authored file that, when present, *is* the product (mods can ship
    /// hand-made LODs). Authored beats derived.
    fn override_path(&self, _subject: &str) -> Option<String> {
        None
    }
    /// Whether the store should LZ4 the output (no for already-compressed data).
    fn compress(&self) -> bool {
        true
    }
}

/// Collects a step's planned inputs and parameters.
#[derive(Default)]
pub struct PlanCx {
    inputs: BTreeSet<Input>,
    params: BTreeMap<String, Vec<u8>>,
}

impl PlanCx {
    pub fn input(&mut self, input: Input) -> &mut Self {
        self.inputs.insert(input);
        self
    }

    pub fn file(&mut self, path: impl Into<String>) -> &mut Self {
        self.input(Input::File(path.into()))
    }

    pub fn record(&mut self, kind: impl Into<String>, id: impl Into<String>) -> &mut Self {
        self.input(Input::Record { kind: kind.into(), id: id.into() })
    }

    pub fn product(&mut self, step: impl Into<String>, subject: impl Into<String>) -> &mut Self {
        self.input(Input::Product(ProductKey::new(step, subject)))
    }

    /// A setting that changes the output (quality tier, target error, …).
    pub fn param(&mut self, name: impl Into<String>, value: impl Into<Vec<u8>>) -> &mut Self {
        self.params.insert(name.into(), value.into());
        self
    }
}

/// A running step's window onto its inputs. Every read is recorded.
pub struct RunCx<'a> {
    baker: &'a Baker,
    chain: &'a [ProductKey],
    planned: &'a BTreeSet<Input>,
    dynamic: BTreeMap<Input, Digest>,
}

impl RunCx<'_> {
    pub fn read_file(&mut self, path: &str) -> Result<Vec<u8>, BakeError> {
        let input = Input::File(path.to_owned());
        let fp = self.baker.fingerprint(&input, self.chain)?;
        if fp == Digest::MISSING {
            return Err(BakeError::MissingInput(input));
        }
        let bytes = self.baker.inputs.read_file(path)?;
        self.note(input, fp);
        Ok(bytes)
    }

    /// Whether a file exists, recorded as a dependency (so a file appearing
    /// later invalidates this product).
    pub fn file_exists(&mut self, path: &str) -> Result<bool, BakeError> {
        let input = Input::File(path.to_owned());
        let fp = self.baker.fingerprint(&input, self.chain)?;
        self.note(input, fp);
        Ok(fp != Digest::MISSING)
    }

    /// Records a dependency on a record; the step reads its value through
    /// its own handle. Returns the fingerprint.
    pub fn record(&mut self, kind: &str, id: &str) -> Result<Digest, BakeError> {
        let input = Input::Record { kind: kind.to_owned(), id: id.to_owned() };
        let fp = self.baker.fingerprint(&input, self.chain)?;
        self.note(input, fp);
        Ok(fp)
    }

    /// Another product's bytes, baking it first if needed.
    pub fn product(&mut self, step: &str, subject: &str) -> Result<Vec<u8>, BakeError> {
        let input = Input::Product(ProductKey::new(step, subject));
        let digest = self.baker.fingerprint(&input, self.chain)?;
        self.note(input, digest);
        Ok(self.baker.store.get(&digest)?)
    }

    fn note(&mut self, input: Input, fp: Digest) {
        if !self.planned.contains(&input) {
            self.dynamic.insert(input, fp);
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum BakeError {
    #[error("bake store I/O: {0}")]
    Io(#[from] std::io::Error),
    #[error("no step named {0:?} is registered")]
    UnknownStep(String),
    #[error("missing input {0:?}")]
    MissingInput(Input),
    #[error("dependency cycle: {0:?}")]
    Cycle(Vec<ProductKey>),
    #[error("{key}: {message}")]
    Step { key: ProductKey, message: String },
    #[error("{key} needs {source}")]
    Dependency { key: ProductKey, source: Arc<BakeError> },
    #[error("{0}")]
    Other(String),
}

impl BakeError {
    /// Convenience for step implementations.
    pub fn step(key: &ProductKey, message: impl fmt::Display) -> Self {
        Self::Step { key: key.clone(), message: message.to_string() }
    }
}

/// How a product was obtained this session.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// The trace matched; nothing ran.
    Hit,
    /// The step ran.
    Built,
    /// An authored override file was used.
    Override,
}

/// Per-step counters for one session.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StepStats {
    pub hits: u32,
    pub built: u32,
    pub overrides: u32,
    pub failed: u32,
    /// Products this step resolved while another thread had the same product
    /// in flight (also counted above). The bake never waits on an in-flight
    /// product (bake.md "Concurrency"); it resolves it again, with the same
    /// result. Nonzero means a dependency was not scheduled ahead of its
    /// dependents (usually a dynamic read the host could bake first).
    pub duplicates: u32,
    /// Time spent running the step itself (not waiting for dependencies).
    pub run_time: Duration,
    pub bytes_written: u64,
}

/// The result of [`Baker::bake`].
#[derive(Debug)]
pub struct BakeReport {
    pub products: Vec<(ProductKey, Result<Digest, Arc<BakeError>>)>,
    pub steps: BTreeMap<&'static str, StepStats>,
    pub elapsed: Duration,
}

impl BakeReport {
    pub fn failures(&self) -> impl Iterator<Item = (&ProductKey, &Arc<BakeError>)> {
        self.products.iter().filter_map(|(k, r)| r.as_ref().err().map(|e| (k, e)))
    }
}

type SlotResult = Result<(Digest, Outcome), Arc<BakeError>>;

/// One product's result for the session.
///
/// Never waited on (bake.md "Concurrency"): a thread that needs a product
/// that is claimed but not finished resolves it itself instead of blocking,
/// because inside rayon the claim holder may be suspended under the waiter
/// (work stealing) and a blocking wait could never end. Steps are
/// deterministic, so whichever result is published first is the result.
#[derive(Default)]
struct Slot {
    claimed: AtomicBool,
    done: OnceLock<SlotResult>,
}

/// What a product needs before its cache check, decided once per product.
enum Prep {
    /// An authored override file and its fingerprint.
    Override(String, Digest),
    Plan(PlanCx),
}

impl Prep {
    fn planned_products(&self) -> impl Iterator<Item = &ProductKey> {
        let inputs = match self {
            Self::Override(..) => None,
            Self::Plan(plan) => Some(&plan.inputs),
        };
        inputs.into_iter().flatten().filter_map(|input| match input {
            Input::Product(key) => Some(key),
            _ => None,
        })
    }
}

/// The planned dependency graph of one [`Baker::bake`] call.
struct Graph {
    keys: Vec<ProductKey>,
    slots: Vec<Arc<Slot>>,
    preps: Vec<Mutex<Option<Result<Prep, BakeError>>>>,
    /// Planned dependencies not yet finished, per product.
    pending: Vec<AtomicUsize>,
    dependents: Vec<Vec<usize>>,
}

/// Runs steps against a store. One `Baker` per bake session (per profile);
/// it remembers what it resolved, so each product is planned once.
pub struct Baker {
    store: Store,
    inputs: Arc<dyn Inputs>,
    steps: HashMap<&'static str, Arc<dyn Step>>,
    resolved: Mutex<HashMap<ProductKey, Arc<Slot>>>,
    stats: Mutex<BTreeMap<&'static str, StepStats>>,
}

impl Baker {
    pub fn new(store: Store, inputs: Arc<dyn Inputs>) -> Self {
        Self {
            store,
            inputs,
            steps: HashMap::new(),
            resolved: Mutex::new(HashMap::new()),
            stats: Mutex::new(BTreeMap::new()),
        }
    }

    /// Registers a step; replaces any step with the same id.
    pub fn register(&mut self, step: impl Step) -> &mut Self {
        self.steps.insert(step.id(), Arc::new(step));
        self
    }

    pub fn store(&self) -> &Store {
        &self.store
    }

    /// Resolves one product (baking it and its dependencies if needed).
    pub fn product(&self, key: &ProductKey) -> Result<Digest, BakeError> {
        self.schedule(std::slice::from_ref(key));
        self.resolve(key, &[]).map(|(d, _)| d)
    }

    /// Resolves many products in parallel. Failures don't stop the others.
    pub fn bake(&self, keys: &[ProductKey]) -> BakeReport {
        let start = Instant::now();
        self.schedule(keys);
        let products = keys
            .iter()
            .map(|k| {
                let r = self.resolve(k, &[]).map(|(d, _)| d).map_err(Arc::new);
                (k.clone(), r)
            })
            .collect();
        BakeReport { products, steps: self.step_stats(), elapsed: start.elapsed() }
    }

    /// Every product resolved successfully this session, as a manifest.
    pub fn manifest(&self) -> Manifest {
        let resolved = self.resolved.lock().unwrap_or_else(PoisonError::into_inner);
        let products = resolved
            .iter()
            .filter_map(|(k, slot)| match slot.done.get() {
                Some(Ok((d, _))) => Some((k.clone(), *d)),
                _ => None,
            })
            .collect();
        Manifest { products }
    }

    pub fn step_stats(&self) -> BTreeMap<&'static str, StepStats> {
        self.stats.lock().unwrap_or_else(PoisonError::into_inner).clone()
    }

    /// Re-runs the given products from scratch (ignoring traces) and reports
    /// whether each reproduced its cached output bit for bit.
    pub fn verify(&self, keys: &[ProductKey]) -> Vec<(ProductKey, Result<bool, BakeError>)> {
        self.schedule(keys);
        keys.par_iter()
            .map(|key| {
                let check = || -> Result<bool, BakeError> {
                    let (cached, _) = self.resolve(key, &[])?;
                    let step = self.step(key)?;
                    let mut plan = PlanCx::default();
                    step.plan(&key.subject, &mut plan)?;
                    let chain = [key.clone()];
                    let mut cx = RunCx { baker: self, chain: &chain, planned: &plan.inputs, dynamic: BTreeMap::new() };
                    let bytes = step.run(&key.subject, &mut cx)?;
                    Ok(Digest::of(&bytes) == cached)
                };
                (key.clone(), check())
            })
            .collect()
    }

    fn step(&self, key: &ProductKey) -> Result<Arc<dyn Step>, BakeError> {
        self.steps.get(key.step.as_str()).cloned().ok_or_else(|| BakeError::UnknownStep(key.step.clone()))
    }

    fn slot(&self, key: &ProductKey) -> Arc<Slot> {
        let mut resolved = self.resolved.lock().unwrap_or_else(PoisonError::into_inner);
        resolved.entry(key.clone()).or_default().clone()
    }

    /// Bakes `roots` and their planned dependencies, each product only after
    /// its planned dependencies finished (a dependency-counting scheduler, as
    /// in cargo's job queue), so no product waits on another. Products left
    /// over (a planned cycle) are resolved inline, where cycles are reported.
    fn schedule(&self, roots: &[ProductKey]) {
        let graph = self.discover(roots);
        rayon::in_place_scope(|s| {
            for (i, pending) in graph.pending.iter().enumerate() {
                if pending.load(Ordering::Acquire) == 0 {
                    let graph = &graph;
                    s.spawn(move |s| self.run_node(graph, i, s));
                }
            }
        });
        for (i, key) in graph.keys.iter().enumerate() {
            if graph.slots[i].done.get().is_none() {
                // Errors are reported through the slot.
                let _ = self.resolve(key, &[]);
            }
        }
    }

    /// Plans every unfinished product reachable from `roots` through planned
    /// product inputs (plans are cheap; each is planned once).
    fn discover(&self, roots: &[ProductKey]) -> Graph {
        let mut index: HashMap<ProductKey, usize> = HashMap::new();
        let mut keys: Vec<ProductKey> = Vec::new();
        let mut slots: Vec<Arc<Slot>> = Vec::new();
        let mut frontier: Vec<usize> = Vec::new();
        // A new graph node for `key`, unless it is known or already finished.
        let mut add = |key: &ProductKey, keys: &mut Vec<ProductKey>, frontier: &mut Vec<usize>| -> Option<usize> {
            if let Some(&i) = index.get(key) {
                return Some(i);
            }
            let slot = self.slot(key);
            if slot.done.get().is_some() {
                return None;
            }
            let i = keys.len();
            index.insert(key.clone(), i);
            keys.push(key.clone());
            slots.push(slot);
            frontier.push(i);
            Some(i)
        };
        for key in roots {
            add(key, &mut keys, &mut frontier);
        }
        let mut deps: Vec<Vec<usize>> = Vec::new();
        let mut preps: Vec<Option<Result<Prep, BakeError>>> = Vec::new();
        while !frontier.is_empty() {
            let prepared: Vec<(usize, Result<Prep, BakeError>)> = frontier
                .par_iter()
                .map(|&i| (i, self.step(&keys[i]).and_then(|step| self.prepare(&*step, &keys[i]))))
                .collect();
            let mut next = Vec::new();
            for (i, prep) in prepared {
                let mut planned = Vec::new();
                if let Ok(prep) = &prep {
                    planned.extend(prep.planned_products().filter_map(|key| add(key, &mut keys, &mut next)));
                }
                deps.resize_with(keys.len(), Vec::new);
                preps.resize_with(keys.len(), || None);
                deps[i] = planned;
                preps[i] = Some(prep);
            }
            frontier = next;
        }
        let mut dependents = vec![Vec::new(); keys.len()];
        let pending = deps
            .iter()
            .enumerate()
            .map(|(i, d)| {
                for &j in d {
                    dependents[j].push(i);
                }
                AtomicUsize::new(d.len())
            })
            .collect();
        let preps = preps.into_iter().map(Mutex::new).collect();
        Graph { keys, slots, preps, pending, dependents }
    }

    /// Resolves graph node `i` (its planned dependencies are finished), then
    /// starts every dependent whose last dependency this was.
    fn run_node<'s>(&'s self, graph: &'s Graph, i: usize, s: &rayon::Scope<'s>) {
        let prep = graph.preps[i].lock().unwrap_or_else(PoisonError::into_inner).take();
        let _ = self.complete(&graph.keys[i], &graph.slots[i], &[], prep);
        for &d in &graph.dependents[i] {
            if graph.pending[d].fetch_sub(1, Ordering::AcqRel) == 1 {
                s.spawn(move |s| self.run_node(graph, d, s));
            }
        }
    }

    fn resolve(&self, key: &ProductKey, chain: &[ProductKey]) -> Result<(Digest, Outcome), BakeError> {
        if chain.contains(key) {
            let mut cycle = chain.to_vec();
            cycle.push(key.clone());
            return Err(BakeError::Cycle(cycle));
        }
        let slot = self.slot(key);
        self.complete(key, &slot, chain, None).map_err(|source| match &*source {
            // Keep cycles visible as cycles to callers up the chain.
            BakeError::Cycle(c) => BakeError::Cycle(c.clone()),
            _ => BakeError::Dependency { key: key.clone(), source },
        })
    }

    /// The slot's result, building the product here if it isn't finished:
    /// never blocks on another thread's build (see [`Slot`]).
    fn complete(
        &self,
        key: &ProductKey,
        slot: &Slot,
        chain: &[ProductKey],
        prep: Option<Result<Prep, BakeError>>,
    ) -> SlotResult {
        if let Some(done) = slot.done.get() {
            return done.clone();
        }
        let duplicate = slot.claimed.swap(true, Ordering::AcqRel);
        let mut chain = chain.to_vec();
        chain.push(key.clone());
        let result = self.build(key, &chain, prep, duplicate).map_err(Arc::new);
        // First result wins. The initializer only moves a value, so a
        // concurrent publisher waits for nothing longer than that.
        slot.done.get_or_init(|| result).clone()
    }

    fn fingerprint(&self, input: &Input, chain: &[ProductKey]) -> Result<Digest, BakeError> {
        Ok(match input {
            Input::File(path) => self.inputs.file_fingerprint(path)?.unwrap_or(Digest::MISSING),
            Input::Record { kind, id } => self.inputs.record_fingerprint(kind, id)?.unwrap_or(Digest::MISSING),
            Input::Product(key) => self.resolve(key, chain)?.0,
        })
    }

    /// The override decision, or the plan.
    fn prepare(&self, step: &dyn Step, key: &ProductKey) -> Result<Prep, BakeError> {
        // Authored overrides win and need no planning.
        if let Some(path) = step.override_path(&key.subject)
            && let Some(fp) = self.inputs.file_fingerprint(&path)?
        {
            return Ok(Prep::Override(path, fp));
        }
        let mut plan = PlanCx::default();
        step.plan(&key.subject, &mut plan)?;
        Ok(Prep::Plan(plan))
    }

    fn build(
        &self,
        key: &ProductKey,
        chain: &[ProductKey],
        prep: Option<Result<Prep, BakeError>>,
        duplicate: bool,
    ) -> Result<(Digest, Outcome), BakeError> {
        let step = self.step(key)?;
        let result = prep
            .unwrap_or_else(|| self.prepare(&*step, key))
            .and_then(|prep| self.build_with(&*step, key, chain, prep));
        let mut stats = self.stats.lock().unwrap_or_else(PoisonError::into_inner);
        let s = stats.entry(step.id()).or_default();
        s.duplicates += u32::from(duplicate);
        match &result {
            Ok((_, Outcome::Hit, ..)) => s.hits += 1,
            Ok((_, Outcome::Built, ..)) => s.built += 1,
            Ok((_, Outcome::Override, ..)) => s.overrides += 1,
            Err(_) => s.failed += 1,
        }
        result.map(|(d, outcome, run_time, bytes)| {
            s.run_time += run_time;
            s.bytes_written += bytes;
            (d, outcome)
        })
    }

    /// Returns the digest, how it was obtained, run time, and bytes written.
    fn build_with(
        &self,
        step: &dyn Step,
        key: &ProductKey,
        chain: &[ProductKey],
        prep: Prep,
    ) -> Result<(Digest, Outcome, Duration, u64), BakeError> {
        let plan = match prep {
            Prep::Override(path, fp) => {
                let static_key = KeyHasher::new("sf_bake/override/1").str(&path).digest(&fp).finish();
                if let Some(trace) = self.hit(key, static_key, chain) {
                    return Ok((trace, Outcome::Override, Duration::ZERO, 0));
                }
                let bytes = self.inputs.read_file(&path)?;
                let digest = self.store.put(&bytes, step.compress())?;
                self.store.save_trace(key, &Trace::new(&key.subject, static_key, Vec::new(), digest))?;
                return Ok((digest, Outcome::Override, Duration::ZERO, bytes.len() as u64));
            }
            Prep::Plan(plan) => plan,
        };

        // Sequential on purpose. Scheduled, the planned products are already
        // finished; inline (a dynamic read, a cycle check), a `par_iter` here
        // would let this thread steal unrelated work in the middle of a build.
        let fingerprints: Vec<(Input, Digest)> = plan
            .inputs
            .iter()
            .map(|input| Ok((input.clone(), self.fingerprint(input, chain)?)))
            .collect::<Result<_, BakeError>>()?;

        let mut h = KeyHasher::new("sf_bake/key/1");
        h.str(step.id()).u32(step.version()).str(&key.subject);
        for (name, value) in &plan.params {
            h.str(name).bytes(value);
        }
        for (input, fp) in &fingerprints {
            h.bytes(&postcard::to_stdvec(input).map_err(|e| BakeError::Other(e.to_string()))?).digest(fp);
        }
        let static_key = h.finish();

        if let Some(output) = self.hit(key, static_key, chain) {
            return Ok((output, Outcome::Hit, Duration::ZERO, 0));
        }

        let started = Instant::now();
        let mut cx = RunCx { baker: self, chain, planned: &plan.inputs, dynamic: BTreeMap::new() };
        let bytes = step.run(&key.subject, &mut cx)?;
        let dynamic: Vec<(Input, Digest)> = cx.dynamic.into_iter().collect();
        let run_time = started.elapsed();
        let digest = self.store.put(&bytes, step.compress())?;
        self.store.save_trace(key, &Trace::new(&key.subject, static_key, dynamic, digest))?;
        Ok((digest, Outcome::Built, run_time, bytes.len() as u64))
    }

    /// The cached output, if the trace for `key` is still valid.
    fn hit(&self, key: &ProductKey, static_key: Digest, chain: &[ProductKey]) -> Option<Digest> {
        let trace = self.store.load_trace(key)?;
        if trace.static_key != static_key || !self.store.contains(&trace.output) {
            return None;
        }
        let unchanged = trace.dynamic.iter().all(|(input, fp)| self.fingerprint(input, chain).ok() == Some(*fp));
        unchanged.then_some(trace.output)
    }
}

#[cfg(test)]
mod tests;
