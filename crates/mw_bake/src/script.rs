//! Legacy scripts (scripting-adapter.md "Pipeline", M5 W2): `mwscript.translate` turns every
//! live SCPT script (subject `script/<id>`) and every dialogue topic's INFO results (subject
//! `topic/<topic>`) into a Luau module plus its Luau bytecode (mlua `Compiler`, optimization
//! level 2); `mwscript.coverage` (subject `corpus`) sums them into the coverage report.
//!
//! Inputs are parameters, not record reads: each subject's source text and one digest of the
//! resolution environment ([`RecordEnv`]: globals, object IDs, object → script, every script's
//! locals), so adding a global or a script variable re-translates everything (the whole corpus is
//! about a second of CPU) and nothing else does. Products hold the owner's game text: they live
//! in the bake store only.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt::{self, Write as _};
use std::sync::Arc;
use std::time::{Duration, Instant};

use mw_script::ast::VarType;
use mw_script::{
    CallSites, Env, FunctionTable, MemberLookup, OracleFlags, Origin, Parsed, Severity, SourceKind, emit_dialogue,
    emit_script, oracle_flags, parse, parse_and_resolve,
};
use serde::{Deserialize, Serialize};
use sf_bake::{BakeError, Baker, Digest, PlanCx, ProductKey, RunCx, Step, Store};
use sf_records::RecordDb;
use tes3::esp::{
    Activator, Alchemy, Apparatus, Armor, Birthsign, Bodypart, Book, Class, Clothing, Container, Creature, Dialogue,
    DialogueInfo, Door, Enchanting, Faction, GlobalValue, GlobalVariable, Ingredient, LeveledCreature, LeveledItem,
    Light, Lockpick, MiscItem, Npc, Probe, Race, Region, RepairItem, Script, Sound, Spell, Static, Weapon,
};

pub const TRANSLATE: &str = "mwscript.translate";
pub const COVERAGE: &str = "mwscript.coverage";
const COVERAGE_SUBJECT: &str = "corpus";

/// What resolution knows about the loaded content (all keys lowercase).
#[derive(Default)]
pub struct RecordEnv {
    globals: HashMap<String, VarType>,
    ids: HashSet<String>,
    /// Object ID → its script ID.
    object_scripts: HashMap<String, String>,
    /// Script ID → local name → type.
    locals: HashMap<String, HashMap<String, VarType>>,
}

impl RecordEnv {
    /// Builds the environment; `scripts` are the live script texts (their locals are parsed here).
    pub fn new(db: &RecordDb, scripts: &BTreeMap<String, String>) -> Self {
        let mut env = Self { ids: HashSet::from(["player".to_owned()]), ..Self::default() };
        for (id, g) in db.iter::<GlobalVariable>() {
            let ty = match g.value {
                GlobalValue::Short(_) => VarType::Short,
                GlobalValue::Long(_) => VarType::Long,
                GlobalValue::Float(_) => VarType::Float,
            };
            env.globals.insert(id.as_str().to_lowercase(), ty);
        }
        macro_rules! ids {
            ($($ty:ty),* $(,)?) => {$(
                env.ids.extend(db.iter::<$ty>().map(|(id, _)| id.as_str().to_lowercase()));
            )*};
        }
        ids!(
            GlobalVariable, Class, Faction, Race, Sound, Script, Region, Birthsign, Spell, Static, Door, MiscItem,
            Weapon, Container, Creature, Bodypart, Light, Enchanting, Npc, Armor, Clothing, RepairItem, Activator,
            Apparatus, Lockpick, Probe, Ingredient, Book, Alchemy, LeveledItem, LeveledCreature, Dialogue,
        );
        macro_rules! scripted {
            ($($ty:ty),* $(,)?) => {$(
                for (id, r) in db.iter::<$ty>() {
                    if !r.script.is_empty() {
                        env.object_scripts.insert(id.as_str().to_lowercase(), r.script.to_lowercase());
                    }
                }
            )*};
        }
        scripted!(
            Activator, Alchemy, Apparatus, Armor, Book, Clothing, Container, Creature, Door, Ingredient, Light,
            Lockpick, MiscItem, Npc, Probe, RepairItem, Weapon,
        );
        let table = FunctionTable::vanilla();
        for (id, text) in scripts {
            let p = parse(text, SourceKind::Script, table);
            env.locals.insert(id.clone(), p.script.locals.iter().map(|l| (l.name.key.clone(), l.ty)).collect());
        }
        env
    }

    /// A digest of everything resolution reads (sorted, so it is deterministic).
    pub fn digest(&self) -> Digest {
        let mut s = String::new();
        let mut globals: Vec<_> = self.globals.iter().map(|(k, t)| format!("g {k} {t:?}\n")).collect();
        let mut ids: Vec<_> = self.ids.iter().map(|k| format!("i {k}\n")).collect();
        let mut objects: Vec<_> = self.object_scripts.iter().map(|(o, sc)| format!("o {o} {sc}\n")).collect();
        let mut locals: Vec<_> = self
            .locals
            .iter()
            .flat_map(|(sc, vars)| vars.iter().map(move |(v, t)| format!("l {sc} {v} {t:?}\n")))
            .collect();
        for list in [&mut globals, &mut ids, &mut objects, &mut locals] {
            list.sort_unstable();
            s.extend(list.iter().map(String::as_str));
        }
        Digest::of(s.as_bytes())
    }
}

impl Env for RecordEnv {
    fn global(&self, key: &str) -> Option<VarType> {
        self.globals.get(key).copied()
    }
    fn object_exists(&self, key: &str) -> Option<bool> {
        Some(self.ids.contains(key))
    }
    fn member(&self, object: &str, var: &str) -> MemberLookup {
        let script = self.object_scripts.get(object).map_or(object, String::as_str);
        match self.locals.get(script) {
            Some(vars) => vars.get(var).map_or(MemberLookup::NoSuchVar, |t| MemberLookup::Found(*t)),
            None => MemberLookup::NoSuchObject,
        }
    }
}

/// Every translatable text of the record database.
pub struct ScriptCorpus {
    /// Script ID (lowercase) → text, live versions.
    pub scripts: BTreeMap<String, String>,
    /// Topic (lowercase) → (topic as written, [(info id, result text)] sorted by info id).
    pub topics: BTreeMap<String, (String, Vec<(String, String)>)>,
    pub env: RecordEnv,
    env_digest: String,
}

impl ScriptCorpus {
    pub fn new(db: &RecordDb) -> Self {
        let scripts: BTreeMap<String, String> =
            db.iter::<Script>().map(|(id, s)| (id.as_str().to_lowercase(), s.text.clone())).collect();
        let mut topics: BTreeMap<String, (String, Vec<(String, String)>)> = BTreeMap::new();
        for (key, info) in db.iter::<DialogueInfo>() {
            if info.script_text.trim().is_empty() {
                continue;
            }
            let Some((topic, info_id)) = mw_records::keys::split_info(key) else { continue };
            let entry = topics.entry(topic.to_lowercase()).or_insert_with(|| (topic.to_owned(), Vec::new()));
            entry.1.push((info_id.to_owned(), info.script_text.clone()));
        }
        for (_, infos) in topics.values_mut() {
            infos.sort();
        }
        let env = RecordEnv::new(db, &scripts);
        let env_digest = env.digest().hex();
        Self { scripts, topics, env, env_digest }
    }

    /// Every `mwscript.translate` subject.
    pub fn subjects(&self) -> Vec<String> {
        self.scripts.keys().map(|id| format!("script/{id}")).chain(self.topics.keys().map(|t| format!("topic/{t}"))).collect()
    }

    pub fn keys(&self) -> Vec<ProductKey> {
        let mut keys: Vec<ProductKey> = self.subjects().into_iter().map(|s| ProductKey::new(TRANSLATE, s)).collect();
        keys.push(ProductKey::new(COVERAGE, COVERAGE_SUBJECT));
        keys
    }
}

/// One `mwscript.translate` product.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Translated {
    /// The Luau module (`return function(mw) ... end`).
    pub source: String,
    /// Luau bytecode of `source` (empty if it failed to compile).
    pub bytecode: Vec<u8>,
    pub compile_error: Option<String>,
    /// Texts translated (1 for a script, the number of results for a topic).
    pub texts: u32,
    /// Inert texts: `"<script or info id>: <first error>"`.
    pub inert: Vec<String>,
    /// Call sites: (canonical function name, count).
    pub calls: Vec<(String, u32)>,
    /// Leniency rules that fired: (rule, hits).
    pub rules: Vec<(String, u32)>,
    /// Numeric spots pending the `OpenMW` oracle: (integer divisions, short/float comparisons).
    pub oracle: (u32, u32),
}

impl Translated {
    pub fn decode(bytes: &[u8]) -> Result<Self, postcard::Error> {
        postcard::from_bytes(bytes)
    }
}

/// `mwscript.translate`.
pub struct TranslateStep {
    pub corpus: Arc<ScriptCorpus>,
}

impl Step for TranslateStep {
    fn id(&self) -> &'static str {
        TRANSLATE
    }
    fn version(&self) -> u32 {
        // Emitter version (generated shape) and product layout version (`Translated`).
        mw_script::emit::EMITTER_VERSION << 8 | 2
    }
    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        let text = match subject.split_once('/') {
            Some(("script", id)) => self.corpus.scripts.get(id).map(|t| Digest::of(t.as_bytes())),
            Some(("topic", t)) => self.corpus.topics.get(t).map(|(_, infos)| {
                let mut all = String::new();
                for (id, text) in infos {
                    all.push_str(id);
                    all.push('\u{1f}');
                    all.push_str(text);
                    all.push('\u{1e}');
                }
                Digest::of(all.as_bytes())
            }),
            _ => None,
        };
        let text = text.ok_or_else(|| BakeError::Other(format!("{TRANSLATE}: no script or topic {subject:?}")))?;
        cx.param("text", text.hex()).param("env", self.corpus.env_digest.clone()).param("table", table_digest());
        Ok(())
    }
    fn run(&self, subject: &str, _cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let table = FunctionTable::vanilla();
        let env = &self.corpus.env;
        let mut out = Translated::default();
        let mut calls = CallSites::default();
        let mut rules: BTreeMap<&'static str, u32> = BTreeMap::new();
        let mut oracle = OracleFlags::default();
        let mut note = |parsed: &Parsed, origin: Origin, what: &str, out: &mut Translated| {
            out.texts += 1;
            calls.add_parsed(parsed, origin, table);
            oracle.add(oracle_flags(parsed, table, env));
            for d in parsed.diagnostics.iter() {
                *rules.entry(d.rule.name()).or_default() += 1;
            }
            if parsed.is_inert() {
                let first = parsed.diagnostics.iter().find(|d| d.severity() == Severity::Error);
                out.inert.push(format!("{what}: {}", first.map(ToString::to_string).unwrap_or_default()));
            }
        };
        let source = match subject.split_once('/') {
            Some(("script", id)) => {
                let text = &self.corpus.scripts[id];
                let parsed = parse_and_resolve(text, SourceKind::Script, table, env);
                note(&parsed, Origin::Script, subject, &mut out);
                emit_script(id, "live record", &parsed, table, env)
            }
            Some(("topic", t)) => {
                let (topic, infos) = &self.corpus.topics[t];
                let parsed: Vec<(&str, Parsed)> = infos
                    .iter()
                    .map(|(id, text)| (id.as_str(), parse_and_resolve(text, SourceKind::DialogueResult, table, env)))
                    .collect();
                for (id, p) in &parsed {
                    note(p, Origin::DialogueResult, &format!("{topic}/{id}"), &mut out);
                }
                let refs: Vec<(&str, &Parsed)> = parsed.iter().map(|(id, p)| (*id, p)).collect();
                emit_dialogue(topic, "live records", &refs, table, env)
            }
            _ => return Err(BakeError::Other(format!("{TRANSLATE}: bad subject {subject:?}"))),
        };
        out.source = source;
        match compile(&out.source) {
            Ok(bytecode) => out.bytecode = bytecode,
            Err(e) => out.compile_error = Some(e),
        }
        out.calls = calls.by_function.into_iter().map(|(f, (s, r))| (f, s + r)).collect();
        out.rules = rules.into_iter().map(|(r, n)| (r.to_owned(), n)).collect();
        out.oracle = (oracle.int_div, oracle.mixed_cmp);
        postcard::to_allocvec(&out).map_err(|e| BakeError::Other(format!("{TRANSLATE} {subject}: {e}")))
    }
}

/// Luau bytecode, as the runtime compiles it (optimization level 2, scripting-adapter.md "Compile").
pub fn compile(source: &str) -> Result<Vec<u8>, String> {
    sf_script::mlua::chunk::Compiler::new().set_optimization_level(2).compile(source).map_err(|e| e.to_string())
}

fn table_digest() -> String {
    // Statuses don't change the translation; names, arities and kinds do.
    let table = FunctionTable::vanilla();
    let mut s = String::new();
    for f in &table.functions {
        let _ = writeln!(s, "{} {:?} {:?} {}", f.name, f.args, f.returns, f.on_reference);
    }
    Digest::of(s.as_bytes()).hex()
}

/// The `mwscript.coverage` product: corpus totals plus call sites by function.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CorpusReport {
    pub scripts: u32,
    pub results: u32,
    pub topics: u32,
    pub inert: Vec<String>,
    /// `"<subject>: <error>"`.
    pub compile_errors: Vec<String>,
    /// (function, call sites in scripts, in dialogue results).
    pub call_sites: Vec<(String, u32, u32)>,
    pub rules: Vec<(String, u32)>,
    pub source_bytes: u64,
    pub bytecode_bytes: u64,
    /// Numeric spots pending the `OpenMW` oracle (integer divisions, short/float comparisons).
    pub oracle: (u32, u32),
}

impl CorpusReport {
    pub fn decode(bytes: &[u8]) -> Result<Self, postcard::Error> {
        postcard::from_bytes(bytes)
    }

    /// The coverage report against the vanilla function table (statuses, families).
    pub fn coverage(&self) -> mw_script::Coverage {
        let mut sites = CallSites::default();
        for (f, s, r) in &self.call_sites {
            sites.add(f, Origin::Script, *s);
            sites.add(f, Origin::DialogueResult, *r);
        }
        sites.report(FunctionTable::vanilla())
    }
}

/// `mwscript.coverage`: reads every translate product.
pub struct CoverageStep {
    pub corpus: Arc<ScriptCorpus>,
}

impl Step for CoverageStep {
    fn id(&self) -> &'static str {
        COVERAGE
    }
    fn version(&self) -> u32 {
        2
    }
    fn plan(&self, _subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        for s in self.corpus.subjects() {
            cx.product(TRANSLATE, s);
        }
        // Statuses are part of the report.
        let mut statuses = String::new();
        for f in &FunctionTable::vanilla().functions {
            let _ = write!(statuses, "{:?}", f.status);
        }
        cx.param("status", Digest::of(statuses.as_bytes()).hex());
        Ok(())
    }
    fn run(&self, _subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let mut report = CorpusReport::default();
        let mut sites: BTreeMap<String, (u32, u32)> = BTreeMap::new();
        let mut rules: BTreeMap<String, u32> = BTreeMap::new();
        for subject in self.corpus.subjects() {
            let bytes = cx.product(TRANSLATE, &subject)?;
            let t = Translated::decode(&bytes).map_err(|e| BakeError::Other(format!("{subject}: {e}")))?;
            let is_script = subject.starts_with("script/");
            if is_script {
                report.scripts += t.texts;
            } else {
                report.topics += 1;
                report.results += t.texts;
            }
            report.inert.extend(t.inert);
            if let Some(e) = t.compile_error {
                report.compile_errors.push(format!("{subject}: {e}"));
            }
            for (f, n) in t.calls {
                let e = sites.entry(f).or_default();
                if is_script { e.0 += n } else { e.1 += n }
            }
            for (r, n) in t.rules {
                *rules.entry(r).or_default() += n;
            }
            report.source_bytes += t.source.len() as u64;
            report.bytecode_bytes += t.bytecode.len() as u64;
            report.oracle = (report.oracle.0 + t.oracle.0, report.oracle.1 + t.oracle.1);
        }
        report.call_sites = sites.into_iter().map(|(f, (s, r))| (f, s, r)).collect();
        report.rules = rules.into_iter().collect();
        postcard::to_allocvec(&report).map_err(|e| BakeError::Other(format!("{COVERAGE}: {e}")))
    }
}

/// Registers both steps for `corpus`.
pub fn register(baker: &mut Baker, corpus: &Arc<ScriptCorpus>) {
    baker.register(TranslateStep { corpus: corpus.clone() }).register(CoverageStep { corpus: corpus.clone() });
}

/// What `bevyrrowind bake` prints for legacy scripts.
pub struct ScriptBakeReport {
    pub report: CorpusReport,
    /// Products that failed to bake (not translation problems: those are in `report`).
    pub failures: Vec<String>,
    pub built: u32,
    pub hits: u32,
    pub prepare: Duration,
    pub elapsed: Duration,
}

/// Builds the corpus from `db`, registers the steps and bakes every script and topic plus the
/// coverage report.
pub fn bake_scripts(baker: &mut Baker, store: &Store, db: &RecordDb) -> ScriptBakeReport {
    let started = Instant::now();
    let corpus = Arc::new(ScriptCorpus::new(db));
    register(baker, &corpus);
    let prepare = started.elapsed();
    let bake = baker.bake(&corpus.keys());
    let mut failures: Vec<String> = bake.failures().map(|(k, e)| format!("{k}: {e}")).collect();
    let report = bake
        .products
        .iter()
        .find(|(k, _)| k.step == COVERAGE)
        .and_then(|(_, r)| r.as_ref().ok())
        .and_then(|d| store.get(d).ok())
        .and_then(|b| CorpusReport::decode(&b).ok());
    let report = report.unwrap_or_else(|| {
        failures.push(format!("{COVERAGE}: no report"));
        CorpusReport::default()
    });
    let stats = |step: &str| bake.steps.get(step).copied().unwrap_or_default();
    let (t, c) = (stats(TRANSLATE), stats(COVERAGE));
    ScriptBakeReport {
        report,
        failures,
        built: t.built + c.built,
        hits: t.hits + c.hits,
        prepare,
        elapsed: started.elapsed(),
    }
}

impl ScriptBakeReport {
    /// Translation errors that should fail the bake: compile errors (inert texts are reported, not failures).
    pub fn errors(&self) -> usize {
        self.failures.len() + self.report.compile_errors.len()
    }
}

impl fmt::Display for ScriptBakeReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let r = &self.report;
        writeln!(
            f,
            "scripts: {} scripts + {} dialogue results ({} topics) -> Luau; {} inert, {} compile errors; {} KB Luau, {} KB bytecode; built {} hit {}; prepare {:.2?}, total {:.2?}",
            r.scripts,
            r.results,
            r.topics,
            r.inert.len(),
            r.compile_errors.len(),
            r.source_bytes / 1024,
            r.bytecode_bytes / 1024,
            self.built,
            self.hits,
            self.prepare,
            self.elapsed
        )?;
        writeln!(f, "  coverage: {}", r.coverage().split())?;
        let (int_div, mixed_cmp) = r.oracle;
        writeln!(f, "  numeric semantics: {}", OracleFlags { int_div, mixed_cmp })?;
        for line in self.failures.iter().chain(&r.compile_errors).chain(&r.inert).take(10) {
            writeln!(f, "  {line}")?;
        }
        Ok(())
    }
}
