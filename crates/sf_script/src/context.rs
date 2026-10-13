//! One script context: a sandboxed Luau state plus its bookkeeping (scripting.md "Contexts").

use std::collections::HashSet;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::Arc;
use std::time::{Duration, Instant};

use mlua::{Function, Lua, LuaOptions, MultiValue, StdLib, Table, Value};
use serde::{Deserialize, Serialize};

use crate::api::{ApiModule, InstallCx, log_fn, random_fn};
use crate::fault::{FaultKind, FaultLog, FaultPolicy, Verdict};
use crate::source::{API_REGISTRY, ModRequirer, ScriptSource, load_text, script_root};
use crate::state::{StateError, StateValue};
use crate::timers::{Pcg32, TimerQueue};
use crate::watchdog::Watchdog;
use crate::{ScriptDiagnostic, ScriptError};

/// Registry table: the context's `storage` (plain data, persisted).
pub(crate) const STORAGE: &str = "sf.storage";
/// Registry table: event name → array of handler functions.
pub(crate) const HANDLERS: &str = "sf.handlers";
/// Registry table: timer name → callback.
pub(crate) const TIMER_CBS: &str = "sf.timer_cbs";

/// Per-context limits (owner decision 5; scripting.md "Error isolation and limits").
#[derive(Clone, Copy, Debug)]
pub struct Limits {
    /// Allocator limit of the Luau state.
    pub memory_bytes: usize,
    /// Hard per-call limit enforced by the watchdog.
    pub call_timeout: Duration,
    /// Soft per-frame budget for this context (reported, never enforced).
    pub frame_soft: Duration,
}

impl Limits {
    /// A mod's own context: 64 MiB, 50 ms per call (release).
    pub const MOD: Limits = Limits {
        memory_bytes: 64 << 20,
        call_timeout: Duration::from_millis(50),
        frame_soft: Duration::from_micros(500),
    };
    /// An adapter's shared legacy context: 256 MiB.
    pub const SHARED: Limits = Limits {
        memory_bytes: 256 << 20,
        ..Limits::MOD
    };
    /// Dev builds allow 500 ms per call (debug output, breakpoints in host code).
    #[must_use]
    pub fn dev(self) -> Self {
        Limits {
            call_timeout: Duration::from_millis(500),
            ..self
        }
    }
}

impl Default for Limits {
    fn default() -> Self {
        Limits::MOD
    }
}

/// How to build a context. The mod pipeline (A1/A3) fills this from manifests and the profile.
#[derive(Clone, Debug)]
pub struct ContextConfig {
    /// Usually the mod id; also the namespace of the context's events.
    pub id: String,
    /// Mods whose `scripts/<mod>/` this context runs (one, or an adapter's legacy set).
    pub mods: Vec<String>,
    /// Declared dependencies: their library files are `require`-able as `@<dep>/...`.
    pub dependencies: Vec<String>,
    /// Entry-point files (VFS paths), run once at start in this order.
    pub entry_points: Vec<String>,
    /// The manifest's `api` requirement.
    pub api: semver::VersionReq,
    /// The mod version, recorded in snapshots for migrations.
    pub version: semver::Version,
    pub limits: Limits,
    /// RNG seed (the runtime derives one from the context id when 0).
    pub seed: u64,
}

impl ContextConfig {
    /// A single-mod context with default limits and `scripts/<id>/main.luau` as entry point.
    pub fn for_mod(id: &str) -> Self {
        Self {
            id: id.to_owned(),
            mods: vec![id.to_owned()],
            dependencies: Vec::new(),
            entry_points: vec![format!("{}/main.luau", script_root(id))],
            api: semver::VersionReq::STAR,
            version: semver::Version::new(0, 0, 0),
            limits: Limits::MOD,
            seed: 0,
        }
    }
}

/// Lifecycle of a context.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Status {
    Running,
    /// Handlers are skipped for the rest of the session; state is kept (and saved).
    Suspended {
        reason: String,
    },
    /// Start failed or a Rust API function panicked.
    Failed {
        error: String,
    },
}

/// Per-context counters (feed `sf_diag` in L2).
#[derive(Clone, Debug, Default)]
pub struct ContextStats {
    pub calls: u64,
    pub errors: u64,
    pub aborts: u64,
    /// Total time inside host → script calls.
    pub time: Duration,
    pub memory_high_water: usize,
}

/// An event: an engine fact (`activated`) or a mod event (`<mod>:<name>`), with a plain-data payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub name: String,
    pub payload: StateValue,
    /// Emitting context; `None` for engine facts.
    pub source: Option<String>,
}

/// Rust-side state reachable from API functions through the state's app data.
pub(crate) struct CtxData {
    pub(crate) id: String,
    pub(crate) sim_time: f64,
    pub(crate) timers: TimerQueue,
    pub(crate) outbox: Vec<Event>,
    pub(crate) rng: Pcg32,
    /// True while entry points run (`timers.register` is only legal then).
    pub(crate) starting: bool,
    /// Event names with at least one handler (dispatch skips contexts without a Lua call).
    pub(crate) subscribed: HashSet<String>,
}

/// Everything a context persists: saves (M7) and hot reload use the same blob.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContextSnapshot {
    pub context: String,
    pub version: String,
    pub api_version: String,
    /// FNV-1a of the sorted registered timer names (detects schema drift on load).
    pub timer_schema: u64,
    pub sim_time: f64,
    pub rng: Pcg32,
    pub storage: StateValue,
    pub timers: TimerQueue,
}

impl ContextSnapshot {
    pub fn to_bytes(&self) -> Result<Vec<u8>, ScriptError> {
        postcard::to_stdvec(self).map_err(|e| ScriptError::Snapshot(e.to_string()))
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ScriptError> {
        postcard::from_bytes(bytes).map_err(|e| ScriptError::Snapshot(e.to_string()))
    }
}

/// Result of offering one event to one context.
#[derive(Clone, Copy, Debug, Default)]
pub struct Dispatched {
    pub calls: u32,
    /// A handler returned `"handled"` (suppresses the engine's default action).
    pub handled: bool,
}

/// One Luau state with its config, limits, stats and status.
pub struct ScriptContext {
    config: ContextConfig,
    lua: Lua,
    watchdog: Arc<Watchdog>,
    faults: FaultLog,
    stats: ContextStats,
    status: Status,
    api_version: semver::Version,
    pub(crate) diagnostics: Vec<ScriptDiagnostic>,
    /// Registry tables, cached (no string lookup per dispatch).
    handlers: Table,
    timer_cbs: Table,
}

fn safe_libs() -> StdLib {
    StdLib::COROUTINE
        | StdLib::TABLE
        | StdLib::STRING
        | StdLib::UTF8
        | StdLib::BIT
        | StdLib::MATH
        | StdLib::BUFFER
        | StdLib::VECTOR
        | StdLib::DEBUG
}

/// Base-library globals removed before sandboxing (mlua adds `loadstring` and a filesystem
/// `require`; Luau keeps `getfenv`/`setfenv`/`newproxy`/`gcinfo` for compatibility).
const REMOVED_GLOBALS: [&str; 6] = [
    "loadstring",
    "getfenv",
    "setfenv",
    "newproxy",
    "gcinfo",
    "require",
];

fn is_memory_error(e: &mlua::Error) -> bool {
    match e {
        mlua::Error::MemoryError(_) => true,
        mlua::Error::CallbackError { cause, .. } | mlua::Error::WithContext { cause, .. } => {
            is_memory_error(cause)
        }
        _ => false,
    }
}

/// FNV-1a over sorted names.
fn schema_hash<'a>(names: impl Iterator<Item = &'a str>) -> u64 {
    let mut names: Vec<&str> = names.collect();
    names.sort_unstable();
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for n in names {
        for b in n.bytes().chain([0]) {
            h = (h ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3);
        }
    }
    h
}

/// The default seed: FNV-1a of the id, so contexts get distinct, stable streams.
pub(crate) fn seed_for(id: &str) -> u64 {
    schema_hash(std::iter::once(id))
}

impl ScriptContext {
    /// Creates the sandboxed state and installs the API. Entry points don't run yet.
    pub(crate) fn build(
        config: ContextConfig,
        source: Arc<dyn ScriptSource>,
        modules: &[Arc<dyn ApiModule>],
        api_version: &semver::Version,
    ) -> Result<Self, ScriptError> {
        if !config.api.matches(api_version) {
            return Err(ScriptError::ApiVersion {
                context: config.id.clone(),
                required: config.api.to_string(),
                provided: api_version.to_string(),
            });
        }
        let lua = Lua::new_with(safe_libs(), LuaOptions::default())
            .map_err(|e| ScriptError::lua(&config.id, &e))?;
        let watchdog = Watchdog::new();
        Self::setup(&lua, &config, source, modules, api_version)
            .map_err(|e| ScriptError::lua(&config.id, &e))?;
        watchdog.install(&lua);
        lua.set_memory_limit(config.limits.memory_bytes)
            .map_err(|e| ScriptError::lua(&config.id, &e))?;
        let used = lua.used_memory();
        let lua_err = |e: mlua::Error| ScriptError::lua(&config.id, &e);
        let handlers: Table = lua.named_registry_value(HANDLERS).map_err(lua_err)?;
        let timer_cbs: Table = lua.named_registry_value(TIMER_CBS).map_err(lua_err)?;
        Ok(Self {
            config,
            lua,
            watchdog,
            faults: FaultLog::default(),
            stats: ContextStats {
                memory_high_water: used,
                ..ContextStats::default()
            },
            status: Status::Running,
            api_version: api_version.clone(),
            diagnostics: Vec::new(),
            handlers,
            timer_cbs,
        })
    }

    fn setup(
        lua: &Lua,
        config: &ContextConfig,
        source: Arc<dyn ScriptSource>,
        modules: &[Arc<dyn ApiModule>],
        api_version: &semver::Version,
    ) -> mlua::Result<()> {
        let seed = if config.seed == 0 {
            seed_for(&config.id)
        } else {
            config.seed
        };
        lua.set_app_data(CtxData {
            id: config.id.clone(),
            sim_time: 0.0,
            timers: TimerQueue::default(),
            outbox: Vec::new(),
            rng: Pcg32::new(seed, 0x5f5f),
            starting: false,
            subscribed: HashSet::new(),
        });
        let g = lua.globals();
        for name in REMOVED_GLOBALS {
            g.raw_set(name, Value::Nil)?;
        }
        // `debug`: only `traceback`.
        let debug: Table = g.raw_get("debug")?;
        let slim = lua.create_table()?;
        slim.raw_set("traceback", debug.raw_get::<Value>("traceback")?)?;
        g.raw_set("debug", slim)?;
        g.raw_set("print", log_fn(lua)?)?;
        // `math`: a copy with the context's seeded PCG instead of the VM's global generator.
        let math: Table = g.raw_get("math")?;
        let ours = lua.create_table()?;
        for pair in math.pairs::<Value, Value>() {
            let (k, v) = pair?;
            ours.raw_set(k, v)?;
        }
        ours.raw_set("random", random_fn(lua)?)?;
        ours.raw_set(
            "randomseed",
            lua.create_function(|lua, seed: i64| {
                if let Some(mut d) = lua.app_data_mut::<CtxData>() {
                    d.rng = Pcg32::new(seed.cast_unsigned(), 0x5f5f);
                }
                Ok(())
            })?,
        )?;
        g.raw_set("math", ours)?;

        lua.set_named_registry_value(STORAGE, lua.create_table()?)?;
        lua.set_named_registry_value(HANDLERS, lua.create_table()?)?;
        lua.set_named_registry_value(TIMER_CBS, lua.create_table()?)?;

        // API modules (`@sf/*`, adapter aliases), read-only.
        let registry = lua.create_table()?;
        let cx = InstallCx {
            context: &config.id,
            mods: &config.mods,
            api_version,
        };
        let mut names = HashSet::new();
        for m in modules {
            let t = m.install(lua, &cx)?;
            t.set_readonly(true);
            if m.name() == "@sf/core" {
                g.raw_set("sf", &t)?;
            }
            registry.raw_set(m.name(), t)?;
            names.insert(m.name().to_owned());
        }
        lua.set_named_registry_value(API_REGISTRY, registry)?;

        let readable: HashSet<String> = config
            .mods
            .iter()
            .chain(&config.dependencies)
            .cloned()
            .collect();
        let require = lua.create_require_function(ModRequirer::new(source, readable, names))?;
        g.raw_set("require", require)?;
        lua.sandbox(true)
    }

    /// Runs the entry points (handler registration). A failure fails the context.
    pub(crate) fn run_entries(
        &mut self,
        source: &dyn ScriptSource,
        policy: &FaultPolicy,
    ) -> Result<(), ScriptError> {
        self.set_starting(true);
        let result = (|| {
            for path in self.config.entry_points.clone() {
                let text = source.read(&path).ok_or_else(|| ScriptError::Start {
                    context: self.config.id.clone(),
                    message: format!("entry point {path} not found"),
                })?;
                let f = load_text(&self.lua, &text, &path)
                    .map_err(|e| ScriptError::lua(&self.config.id, &e))?;
                let label = format!("entry {path}");
                if self
                    .call::<()>(&f, MultiValue::new(), &label, policy)
                    .is_none()
                {
                    let message = self
                        .diagnostics
                        .last()
                        .map_or_else(|| "entry point failed".into(), |d| d.message.clone());
                    return Err(ScriptError::Start {
                        context: self.config.id.clone(),
                        message,
                    });
                }
            }
            Ok(())
        })();
        self.set_starting(false);
        if let Err(e) = &result {
            self.status = Status::Failed {
                error: e.to_string(),
            };
        }
        result
    }

    fn set_starting(&self, starting: bool) {
        if let Some(mut d) = self.lua.app_data_mut::<CtxData>() {
            d.starting = starting;
        }
    }

    pub fn id(&self) -> &str {
        &self.config.id
    }
    pub fn config(&self) -> &ContextConfig {
        &self.config
    }
    pub fn status(&self) -> &Status {
        &self.status
    }
    pub fn stats(&self) -> &ContextStats {
        &self.stats
    }
    pub fn is_running(&self) -> bool {
        self.status == Status::Running
    }
    /// The Luau state, for API layers (`sf_script_bevy`) and tests.
    pub fn lua(&self) -> &Lua {
        &self.lua
    }
    pub fn used_memory(&self) -> usize {
        self.lua.used_memory()
    }
    /// Calls `f` under the watchdog and fault policy. `None` if the context isn't running or
    /// the call failed (the failure is recorded as a diagnostic and may suspend the context).
    pub fn call<R: mlua::FromLuaMulti>(
        &mut self,
        f: &Function,
        args: impl mlua::IntoLuaMulti,
        label: &str,
        policy: &FaultPolicy,
    ) -> Option<R> {
        self.guarded(label, policy, |_| f.call::<R>(args))
    }

    /// One host entry into the context: runs `body` under the watchdog (one deadline for the
    /// whole entry), times it, and applies the fault policy to its error, if any.
    pub fn guarded<R>(
        &mut self,
        label: &str,
        policy: &FaultPolicy,
        body: impl FnOnce(&Lua) -> mlua::Result<R>,
    ) -> Option<R> {
        let mut errors = Vec::new();
        let out = self.entry(&mut errors, |lua, _| body(lua));
        self.settle(label, errors, policy);
        out.and_then(Result::ok)
    }

    /// Runs `body` with the watchdog armed; panics become `FaultKind::Panic` in `errors`. A
    /// body may record several handler errors itself and keep going (error isolation per
    /// handler, one deadline and one pair of clock reads per entry).
    fn entry<R>(
        &mut self,
        errors: &mut Vec<(FaultKind, String)>,
        body: impl FnOnce(&Lua, &mut Vec<(FaultKind, String)>) -> mlua::Result<R>,
    ) -> Option<mlua::Result<R>> {
        if !self.is_running() {
            return None;
        }
        let start = Instant::now();
        self.watchdog
            .arm_from(start, self.config.limits.call_timeout);
        let result = catch_unwind(AssertUnwindSafe(|| body(&self.lua, errors)));
        let tripped = self.watchdog.disarm();
        self.stats.time += start.elapsed();
        self.stats.calls += 1;
        self.stats.memory_high_water = self.stats.memory_high_water.max(self.lua.used_memory());
        if let Ok(r) = result {
            if let Err(e) = &r {
                errors.push(classify(e));
            }
            if tripped {
                // Whatever the body reported last was the timeout surfacing.
                match errors.last_mut() {
                    Some(last) => last.0 = FaultKind::Timeout,
                    None => errors.push((
                        FaultKind::Timeout,
                        "script exceeded its time limit (watchdog)".into(),
                    )),
                }
            }
            Some(r)
        } else {
            errors.push((FaultKind::Panic, "a Rust API function panicked".into()));
            None
        }
    }

    fn settle(&mut self, label: &str, errors: Vec<(FaultKind, String)>, policy: &FaultPolicy) {
        for (kind, message) in errors {
            self.fault(kind, label, message, policy);
        }
    }

    fn fault(&mut self, kind: FaultKind, label: &str, message: String, policy: &FaultPolicy) {
        match kind {
            FaultKind::Timeout => self.stats.aborts += 1,
            _ => self.stats.errors += 1,
        }
        log::warn!(target: "sf_script", "[{}] {label}: {message}", self.config.id);
        self.diagnostics.push(ScriptDiagnostic {
            context: self.config.id.clone(),
            kind: Some(kind),
            message: format!("{label}: {message}"),
        });
        if self.status != Status::Running {
            return;
        }
        match self.faults.record(kind, Instant::now(), policy) {
            Verdict::Continue => {}
            Verdict::Suspend(reason) => self.suspend(reason),
            Verdict::Fail(error) => self.status = Status::Failed { error },
        }
    }

    /// Suspends the context for the session.
    pub fn suspend(&mut self, reason: String) {
        log::warn!(target: "sf_script", "[{}] suspended: {reason}", self.config.id);
        self.diagnostics.push(ScriptDiagnostic {
            context: self.config.id.clone(),
            kind: None,
            message: format!("suspended: {reason}"),
        });
        self.status = Status::Suspended { reason };
    }

    pub(crate) fn advance(&self, dt: f64) {
        if let Some(mut d) = self.lua.app_data_mut::<CtxData>() {
            d.sim_time += dt;
        }
    }

    pub(crate) fn take_outbox(&self) -> Vec<Event> {
        self.lua
            .app_data_mut::<CtxData>()
            .map(|mut d| std::mem::take(&mut d.outbox))
            .unwrap_or_default()
    }

    /// Whether any handler subscribed to `name` (a Rust-side set: no Lua call for misses).
    pub fn subscribes(&self, name: &str) -> bool {
        self.lua
            .app_data_ref::<CtxData>()
            .is_some_and(|d| d.subscribed.contains(name))
    }

    /// Offers an event to this context's handlers, in registration order: one watchdog entry
    /// for all of them, the payload converted once (handlers of one mod share the table).
    pub fn dispatch(&mut self, event: &Event, policy: &FaultPolicy) -> Dispatched {
        let mut out = Dispatched::default();
        if !self.is_running() || !self.subscribes(&event.name) {
            return out;
        }
        let handlers = self.handlers.clone();
        let mut errors = Vec::new();
        self.entry(&mut errors, |lua, errors| {
            let Some(list) = handlers.raw_get::<Option<Table>>(event.name.as_str())? else {
                return Ok(());
            };
            let payload = unlimited(lua, || event.payload.to_lua(lua))?;
            for h in list.sequence_values::<Function>() {
                out.calls += 1;
                match h?.call::<Value>(&payload) {
                    Ok(Value::String(s)) if s.as_bytes().as_ref() == b"handled" => {
                        out.handled = true;
                    }
                    Ok(_) => {}
                    Err(e) => {
                        let fault = classify(&e);
                        let stop = fault.0 != FaultKind::Error;
                        errors.push(fault);
                        if stop {
                            return Ok(()); // timeout or memory: stop this entry
                        }
                    }
                }
            }
            Ok(())
        });
        self.settle(&event.name, errors, policy);
        out
    }

    /// Fires the timers due at the context's current simulation time (one watchdog entry).
    pub fn fire_timers(&mut self, policy: &FaultPolicy) -> u32 {
        let due = match self.lua.app_data_mut::<CtxData>() {
            Some(mut d) if d.timers.timers.iter().any(|t| t.due <= d.sim_time) => {
                let now = d.sim_time;
                d.timers.take_due(now)
            }
            _ => return 0,
        };
        let cbs = self.timer_cbs.clone();
        let mut fired = 0;
        let mut errors = Vec::new();
        self.entry(&mut errors, |lua, errors| {
            for t in &due {
                let Some(cb) = cbs.raw_get::<Option<Function>>(t.name.as_str())? else {
                    errors.push((
                        FaultKind::Error,
                        format!("timer {:?}: callback no longer registered", t.name),
                    ));
                    continue;
                };
                fired += 1;
                let payload = unlimited(lua, || t.payload.to_lua(lua))?;
                if let Err(e) = cb.call::<()>(payload) {
                    let (kind, message) = classify(&e);
                    errors.push((kind, format!("timer {:?}: {message}", t.name)));
                    if kind != FaultKind::Error {
                        return Ok(());
                    }
                }
            }
            Ok(())
        });
        self.settle("timers", errors, policy);
        fired
    }

    fn timer_schema(&self) -> u64 {
        let names: Vec<String> = self
            .lua
            .named_registry_value::<Table>(TIMER_CBS)
            .map(|t| {
                t.pairs::<String, Value>()
                    .filter_map(|p| p.ok().map(|(k, _)| k))
                    .collect()
            })
            .unwrap_or_default();
        schema_hash(names.iter().map(String::as_str))
    }

    /// Captures storage, timers, RNG and clock as plain data.
    pub fn snapshot(&self) -> Result<ContextSnapshot, ScriptError> {
        let storage: Value = self
            .lua
            .named_registry_value(STORAGE)
            .map_err(|e| ScriptError::lua(&self.config.id, &e))?;
        // Walk with the allocator limit lifted (see `unlimited`): reads only, but mlua protects
        // table iteration while a limit is set.
        let walked = unlimited(&self.lua, || Ok(StateValue::from_lua(&storage, "storage")))
            .map_err(|e| ScriptError::lua(&self.config.id, &e))?;
        let storage = walked.map_err(|e| ScriptError::State {
            context: self.config.id.clone(),
            error: e,
        })?;
        let d = self
            .lua
            .app_data_ref::<CtxData>()
            .ok_or_else(|| ScriptError::Start {
                context: self.config.id.clone(),
                message: "context data missing".into(),
            })?;
        Ok(ContextSnapshot {
            context: self.config.id.clone(),
            version: self.config.version.to_string(),
            api_version: self.api_version.to_string(),
            timer_schema: self.timer_schema(),
            sim_time: d.sim_time,
            rng: d.rng,
            storage,
            timers: d.timers.clone(),
        })
    }

    /// Restores a snapshot. Timers whose callback this code no longer registers are dropped
    /// with a diagnostic.
    pub fn restore(&mut self, snap: &ContextSnapshot) -> Result<(), ScriptError> {
        self.restore_data(snap)?;
        self.restore_timers(snap)
    }

    /// Restores storage (in place: scripts that captured `sf.storage()` keep a valid table),
    /// clock and RNG. Hot reload calls this before entry points run, so start code sees its data.
    pub(crate) fn restore_data(&mut self, snap: &ContextSnapshot) -> Result<(), ScriptError> {
        let id = self.config.id.clone();
        let lua_err = |e: mlua::Error| ScriptError::lua(&id, &e);
        let storage: Table = self.lua.named_registry_value(STORAGE).map_err(lua_err)?;
        unlimited(&self.lua, || {
            storage.clear()?;
            if let Value::Table(src) = snap.storage.to_lua(&self.lua)? {
                for pair in src.pairs::<Value, Value>() {
                    let (k, v) = pair?;
                    storage.raw_set(k, v)?;
                }
            }
            Ok(())
        })
        .map_err(lua_err)?;
        let mut d = self
            .lua
            .app_data_mut::<CtxData>()
            .ok_or_else(|| ScriptError::Start {
                context: id.clone(),
                message: "context data missing".into(),
            })?;
        d.sim_time = snap.sim_time;
        d.rng = snap.rng;
        Ok(())
    }

    /// Restores pending timers, dropping (with a diagnostic) those whose callback the current
    /// code no longer registers.
    pub(crate) fn restore_timers(&mut self, snap: &ContextSnapshot) -> Result<(), ScriptError> {
        let id = self.config.id.clone();
        let cbs: Table = self
            .lua
            .named_registry_value(TIMER_CBS)
            .map_err(|e| ScriptError::lua(&id, &e))?;
        let mut timers = snap.timers.clone();
        let mut dropped = Vec::new();
        timers.timers.retain(|t| {
            let keep = cbs
                .raw_get::<Option<Function>>(t.name.as_str())
                .ok()
                .flatten()
                .is_some();
            if !keep {
                dropped.push(t.name.clone());
            }
            keep
        });
        for name in dropped {
            self.diagnostics.push(ScriptDiagnostic {
                context: id.clone(),
                kind: None,
                message: format!("restore: timer {name:?} dropped (callback no longer registered)"),
            });
        }
        if let Some(mut d) = self.lua.app_data_mut::<CtxData>() {
            d.timers = timers;
        }
        Ok(())
    }

    /// Runs a source snippet in this context (tests, dev console). Same guards as handlers.
    pub fn eval<R: mlua::FromLuaMulti>(&mut self, code: &str, policy: &FaultPolicy) -> Option<R> {
        let name = format!("=console:{}", self.config.id);
        self.guarded("console", policy, |lua| {
            lua.load(code)
                .set_name(name)
                .set_mode(mlua::chunk::ChunkMode::Text)
                .call::<R>(())
        })
    }

    /// The last diagnostic message (tests).
    pub fn last_error(&self) -> Option<&str> {
        self.diagnostics.last().map(|d| d.message.as_str())
    }
}

/// Reads a value out of a context's storage table by key (tests, tools).
pub fn storage_value(ctx: &ScriptContext, key: &str) -> Option<StateValue> {
    let storage: Table = ctx.lua.named_registry_value(STORAGE).ok()?;
    let v: Value = storage.raw_get(key).ok()?;
    StateValue::from_lua(&v, key).ok()
}

impl From<(String, StateError)> for ScriptError {
    fn from((context, error): (String, StateError)) -> Self {
        ScriptError::State { context, error }
    }
}

/// Splits an error into fault kind and message.
fn classify(e: &mlua::Error) -> (FaultKind, String) {
    let kind = if is_memory_error(e) {
        FaultKind::Memory
    } else {
        FaultKind::Error
    };
    (kind, e.to_string())
}

/// Runs host-side bulk conversion (payloads, restores) with the allocator limit lifted.
///
/// With a memory limit set, mlua wraps every allocating operation in a protected call (an
/// allocation failure must not unwind through Rust frames); the spike measured that as the
/// dominant cost of building payload tables. Host data is bounded and trusted, so it is built
/// unprotected and the limit is re-applied afterwards: if the state is then over its limit, the
/// script's next allocation fails and the context is suspended as usual.
pub(crate) fn unlimited<R>(lua: &Lua, f: impl FnOnce() -> mlua::Result<R>) -> mlua::Result<R> {
    let limit = lua.set_memory_limit(0)?;
    let out = f();
    lua.set_memory_limit(limit)?;
    out
}
