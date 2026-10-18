//! Bevy-free Luau scripting core (scripting.md): one sandboxed Luau state per mod (plus shared
//! contexts for an adapter's translated legacy scripts), memory and CPU limits, fault isolation,
//! require-by-string over the VFS, plain-data storage, named timers, an event queue, and state
//! snapshots for saves and hot reload. `sf_script_bevy` (L2) runs it in schedule slots.

mod api;
mod entity;
mod interfaces;
mod context;
mod fault;
mod handle;
mod source;
mod state;
mod timers;
mod watchdog;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_api;

use std::fmt::Write as _;
use std::sync::Arc;

pub use api::{ApiModule, CoreModule, EventsModule, InstallCx, TimersModule};
pub use entity::{EntityState, install as install_entity_api};
pub use interfaces::InterfacesModule;
pub use context::{
    ContextConfig, ContextSnapshot, ContextStats, Dispatched, Event, Limits, ScriptContext, Status,
    storage_value,
};
pub use fault::{FaultKind, FaultLog, FaultPolicy, Verdict};
pub use handle::{Handle, handle_methods};
/// Re-exported so API modules and the Bevy layer use the same mlua.
pub use mlua;
pub use source::{ScriptSource, script_root};
pub use state::{StateError, StateKey, StateValue};
pub use timers::{Pcg32, Timer, TimerQueue};
pub use watchdog::{Watchdog, install_every_tick};

/// The framework API version this build provides (`api` in mod manifests). `0.x` until M7.
pub const API_VERSION: semver::Version = semver::Version::new(0, 1, 0);

/// Errors that stop a context from starting, reloading or restoring.
#[derive(Debug, Clone, thiserror::Error)]
pub enum ScriptError {
    #[error("{context}: requires script API {required}, this build provides {provided}")]
    ApiVersion {
        context: String,
        required: String,
        provided: String,
    },
    #[error("{context}: {message}")]
    Start { context: String, message: String },
    #[error("{context}: {message}")]
    Lua { context: String, message: String },
    #[error("{context}: {error}")]
    State { context: String, error: StateError },
    #[error("snapshot: {0}")]
    Snapshot(String),
    #[error("no script context {0:?}")]
    NoContext(String),
}

impl ScriptError {
    pub(crate) fn lua(context: &str, e: &mlua::Error) -> Self {
        ScriptError::Lua {
            context: context.to_owned(),
            message: e.to_string(),
        }
    }
}

/// A fault or notice attributed to a context.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScriptDiagnostic {
    pub context: String,
    /// `None` for notices (suspension, dropped timers).
    pub kind: Option<FaultKind>,
    pub message: String,
}

/// What one `update` did.
#[derive(Clone, Copy, Debug, Default)]
pub struct UpdateReport {
    pub events: u32,
    pub handler_calls: u32,
    pub timers: u32,
}

/// All script contexts, in load order, plus the API registry and the event queue.
pub struct ScriptRuntime {
    api_version: semver::Version,
    modules: Vec<Arc<dyn ApiModule>>,
    contexts: Vec<ScriptContext>,
    /// Events delivered at the next `update`, in emission order.
    pending: Vec<Event>,
    policy: FaultPolicy,
    source: Option<Arc<dyn ScriptSource>>,
    diagnostics: Vec<ScriptDiagnostic>,
    /// `@sf/interfaces` exports of every context.
    interfaces: Arc<interfaces::Registry>,
    game_time: Option<f64>,
}

#[cfg(feature = "send")]
const _: () = {
    const fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ScriptRuntime>();
};

impl Default for ScriptRuntime {
    fn default() -> Self {
        Self::new(API_VERSION)
    }
}

impl ScriptRuntime {
    /// A runtime with the built-in `@sf/core`, `@sf/events`, `@sf/timers` and
    /// `@sf/interfaces` modules.
    pub fn new(api_version: semver::Version) -> Self {
        let interfaces = Arc::new(interfaces::Registry::default());
        let mut rt = Self {
            api_version,
            modules: Vec::new(),
            contexts: Vec::new(),
            pending: Vec::new(),
            policy: FaultPolicy::default(),
            source: None,
            diagnostics: Vec::new(),
            interfaces: Arc::clone(&interfaces),
            game_time: None,
        };
        rt.register_module(Box::new(CoreModule));
        rt.register_module(Box::new(EventsModule));
        rt.register_module(Box::new(TimersModule));
        rt.register_module(Box::new(InterfacesModule {
            registry: interfaces,
        }));
        rt
    }

    /// Sets the game clock scripts read with `core.game_time()` (hours; `None`: no clock).
    pub fn set_game_time(&mut self, hours: Option<f64>) {
        if self.game_time != hours {
            self.game_time = hours;
            for ctx in &self.contexts {
                ctx.set_game_time(hours);
            }
        }
    }

    /// Runs queued `on_load` handlers (after start, reload or restore), in load order. Hosts
    /// call it at the start of a slot with world access (`sf_script_bevy`'s `Update` slot);
    /// [`ScriptRuntime::update`] calls it too. Returns the number of handler calls.
    pub fn run_loads(&mut self) -> u32 {
        let mut calls = 0;
        for ctx in &mut self.contexts {
            calls += ctx.run_load(&self.policy);
        }
        self.collect();
        calls
    }

    /// The activation pipeline (scripting.md "Activation pipeline"): `"activated"` goes first
    /// to the entity scripts attached to `target` (contexts in load order), then to global
    /// subscribers. `handled` if any handler returned `"handled"`.
    pub fn activate(&mut self, target: Handle, payload: StateValue) -> Dispatched {
        let mut out = Dispatched::default();
        for ctx in &mut self.contexts {
            let d = ctx.dispatch_entity(target, "activated", &payload, &self.policy);
            out.calls += d.calls;
            out.handled |= d.handled;
        }
        let d = self.dispatch("activated", payload);
        out.calls += d.calls;
        out.handled |= d.handled;
        out
    }

    /// Entity scripts' `tick(h, dt, state)`, contexts in load order, handles in order, for
    /// handles `loaded` accepts. Hosts call it after [`ScriptRuntime::update`].
    pub fn tick_entities(&mut self, dt: f64, loaded: &dyn Fn(Handle) -> bool) -> u32 {
        let mut calls = 0;
        for ctx in &mut self.contexts {
            calls += ctx.tick_entities(dt, loaded, &self.policy);
        }
        self.collect();
        calls
    }

    /// Every handle with an entity script attached (any context), sorted, deduplicated.
    pub fn attached(&self) -> Vec<Handle> {
        let mut all: Vec<Handle> = self.contexts.iter().flat_map(ScriptContext::attached).collect();
        all.sort_unstable();
        all.dedup();
        all
    }

    /// Registers (or replaces, by name) an API module. Affects contexts started afterwards.
    pub fn register_module(&mut self, module: Box<dyn ApiModule>) {
        let module: Arc<dyn ApiModule> = Arc::from(module);
        self.modules.retain(|m| m.name() != module.name());
        self.modules.push(module);
    }

    pub fn set_policy(&mut self, policy: FaultPolicy) {
        self.policy = policy;
    }
    pub fn policy(&self) -> &FaultPolicy {
        &self.policy
    }

    /// The `.d.luau` definitions file of every registered module, for `luau-lsp`
    /// (`--definitions`): the `Handle` class with every module's handle methods, then each
    /// module, sorted by name, as a typed global `sf_<name>` (`@sf/world` → `sf_world`).
    pub fn type_definitions(&self) -> String {
        let mut mods: Vec<_> = self.modules.iter().collect();
        mods.sort_by_key(|m| m.name());
        let mut out = format!(
            "--!strict\n-- Generated by sf_script::ScriptRuntime::type_definitions (API {}). Do not edit.\n\
             -- `local world = require(\"@sf/world\")` has the type of `sf_world` below.\n\n\
             declare class Handle\n  function id(self): string\n",
            self.api_version
        );
        for m in &mods {
            out.push_str(m.handle_types());
        }
        out.push_str("end\n");
        for m in &mods {
            let _ = write!(out, "\n-- {} {}\n{}", m.name(), m.version(), m.types());
        }
        out
    }

    /// Builds and starts contexts in the given (load) order. A context that fails to start is
    /// kept as `Failed` and reported; the others start normally.
    pub fn start(
        &mut self,
        configs: Vec<ContextConfig>,
        source: Arc<dyn ScriptSource>,
    ) -> Vec<ScriptDiagnostic> {
        self.source = Some(Arc::clone(&source));
        for config in configs {
            match self.start_one(config, &source) {
                Ok(ctx) => self.contexts.push(ctx),
                Err((Some(ctx), _)) => self.contexts.push(*ctx),
                Err((None, e)) => self.diagnostics.push(ScriptDiagnostic {
                    context: String::new(),
                    kind: None,
                    message: e.to_string(),
                }),
            }
        }
        self.collect();
        self.take_diagnostics()
    }

    fn start_one(
        &self,
        config: ContextConfig,
        source: &Arc<dyn ScriptSource>,
    ) -> Result<ScriptContext, (Option<Box<ScriptContext>>, ScriptError)> {
        let mut ctx =
            ScriptContext::build(config, Arc::clone(source), &self.modules, &self.api_version)
                .map_err(|e| (None, e))?;
        ctx.set_game_time(self.game_time);
        match ctx.run_entries(source.as_ref(), &self.policy) {
            Ok(()) => {
                self.interfaces.publish(&ctx);
                ctx.queue_load("start", None);
                Ok(ctx)
            }
            Err(e) => Err((Some(Box::new(ctx)), e)),
        }
    }

    pub fn contexts(&self) -> &[ScriptContext] {
        &self.contexts
    }
    pub fn context(&self, id: &str) -> Option<&ScriptContext> {
        self.contexts.iter().find(|c| c.id() == id)
    }
    pub fn context_mut(&mut self, id: &str) -> Option<&mut ScriptContext> {
        self.contexts.iter_mut().find(|c| c.id() == id)
    }

    /// Queues an engine fact for the next `update`.
    pub fn post(&mut self, name: impl Into<String>, payload: StateValue) {
        self.pending.push(Event {
            name: name.into(),
            payload,
            source: None,
        });
    }

    /// Offers an event to every running context now, in load order (engine facts handled
    /// inside a slot, e.g. activation). Returns whether any handler returned `"handled"`.
    pub fn dispatch(&mut self, name: &str, payload: StateValue) -> Dispatched {
        let event = Event {
            name: name.to_owned(),
            payload,
            source: None,
        };
        let mut out = Dispatched::default();
        for ctx in &mut self.contexts {
            let d = ctx.dispatch(&event, &self.policy);
            out.calls += d.calls;
            out.handled |= d.handled;
        }
        self.collect();
        out
    }

    /// The `Update` slot: advance simulation time by `dt`, deliver queued events (emission
    /// order × load order), fire due timers, then `update(dt)` handlers.
    pub fn update(&mut self, dt: f64) -> UpdateReport {
        let mut report = UpdateReport::default();
        report.handler_calls += self.run_loads();
        for ctx in &self.contexts {
            ctx.advance(dt);
        }
        let events = std::mem::take(&mut self.pending);
        for event in &events {
            report.events += 1;
            for ctx in &mut self.contexts {
                report.handler_calls += ctx.dispatch(event, &self.policy).calls;
            }
        }
        let tick = Event {
            name: "update".into(),
            payload: StateValue::Number(dt),
            source: None,
        };
        for ctx in &mut self.contexts {
            report.timers += ctx.fire_timers(&self.policy);
            report.handler_calls += ctx.dispatch(&tick, &self.policy).calls;
        }
        self.collect();
        report
    }

    /// Moves context outboxes to the queue and context diagnostics to the runtime.
    fn collect(&mut self) {
        for ctx in &mut self.contexts {
            self.pending.extend(ctx.take_outbox());
            self.diagnostics.append(&mut ctx.diagnostics);
        }
    }

    /// Faults and notices since the last call.
    pub fn take_diagnostics(&mut self) -> Vec<ScriptDiagnostic> {
        self.collect();
        std::mem::take(&mut self.diagnostics)
    }

    /// Serialized snapshot of one context (postcard).
    pub fn snapshot(&self, id: &str) -> Result<Vec<u8>, ScriptError> {
        self.context(id)
            .ok_or_else(|| ScriptError::NoContext(id.to_owned()))?
            .snapshot()?
            .to_bytes()
    }

    /// Restores a context from [`ScriptRuntime::snapshot`] bytes.
    pub fn restore(&mut self, id: &str, bytes: &[u8]) -> Result<(), ScriptError> {
        let snap = ContextSnapshot::from_bytes(bytes)?;
        let ctx = self
            .context_mut(id)
            .ok_or_else(|| ScriptError::NoContext(id.to_owned()))?;
        ctx.restore(&snap)?;
        ctx.queue_load("restore", Some(&snap.version));
        self.collect();
        Ok(())
    }

    /// Hot reload of one context: snapshot → new state from current sources → entry points →
    /// restore. On any error the old state keeps running and the error is returned.
    pub fn reload(
        &mut self,
        id: &str,
        source: Option<Arc<dyn ScriptSource>>,
    ) -> Result<(), ScriptError> {
        let Some(source) = source.or_else(|| self.source.clone()) else {
            return Err(ScriptError::NoContext(id.to_owned()));
        };
        let index = self
            .contexts
            .iter()
            .position(|c| c.id() == id)
            .ok_or_else(|| ScriptError::NoContext(id.to_owned()))?;
        let snap = self.contexts[index].snapshot()?;
        let config = self.contexts[index].config().clone();
        // Storage, clock and RNG before entry points (start code sees its data), timers after
        // (their callbacks are registered by the entry points).
        let mut fresh = ScriptContext::build(
            config,
            Arc::clone(&source),
            &self.modules,
            &self.api_version,
        )?;
        fresh.set_game_time(self.game_time);
        fresh.restore_data(&snap)?;
        fresh.run_entries(source.as_ref(), &self.policy)?;
        fresh.restore_timers(&snap)?;
        fresh.restore_entities(&snap)?;
        fresh.queue_load("reload", Some(&snap.version));
        self.interfaces.publish(&fresh);
        self.diagnostics.append(&mut fresh.diagnostics);
        self.contexts[index] = fresh;
        self.source = Some(source);
        Ok(())
    }
}
