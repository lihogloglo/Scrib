//! Bevy shell for [`sf_ai`] (`docs/specs/ai.md`). Slice AI3 is the debugger, built before any
//! behaviour so every later slice can be watched the day it lands (UE's Gameplay Debugger, Guerrilla's
//! in-game HTN view): nothing in game decides yet (AI4 directives, AI6 executors, AI7 drives), so
//! today it shows what the trace store holds and says so when that is nothing.
//!
//! - [`AiPlugin`]: the resources and the console commands; with feature `debug`, the gizmos.
//! - [`AiTraces`]: the [`TraceStore`] (enabled, tier-2 admission, timings as the host says).
//! - [`AiLibrary`]: the HTN [`Library`] and its load report. [`load_library`] builds it from the
//!   `RecordDb` once mods are loaded, logs every error and keeps running with an empty library.
//! - [`AiActor`]: the stable [`ActorKey`] of an entity the AI runs (placement or spawn id, never the
//!   `Entity`); [`AiSelected`]: the actor the overlay shows; [`AiDirectives`]: every actor's
//!   directive stack by key (AI4), the `sf/ai` save section's state ([`ai_save`] / [`ai_load`],
//!   AI5; `sf_save_bevy::sections::ai` registers it, so this crate needs no save dependency).
//! - Console (`console.md`): `ai.select [id|off]` (no id: the console's click-selection, feature
//!   `debug`), `ai.trace [n] [id]`, `ai.directives [id]` (AI4: the stack, top first),
//!   `ai.tiers` (provisional until the AI9 allocator), `ai.stats`;
//!   `ai.gizmos` on/off (feature `debug`). `ai.facts` waits for AI13.
//! - BRP: [`brp_trace`] answers [`BRP_TRACE_METHOD`] with the store as JSON. This crate doesn't add
//!   `bevy_remote` (one `RemotePlugin` per app, owned by the game's dev tools, and hakari keeps its
//!   HTTP server out of every other build): the app registers [`brp_trace_system`] (see its docs).
//!
//! Actor keys print and parse as 32 hex digits, like `PlacementId`. The overlay only reads: no
//! system here writes anything gameplay reads (ai.md "Architecture": presentation never feeds
//! gameplay).

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::ops::{Deref, DerefMut};

use bevy_app::{App, Plugin};
use bevy_ecs::prelude::*;
use serde::Deserialize;
use serde_json::{Value, json};
use sf_ai::{
    ActorKey, AiSave, DirectiveStack, LoadReport, Library, MethodId, OperatorId, PlanOutcome, StateKeys, TaskId, Trace,
    TraceNames, TraceStore, format_directives, format_trace_with,
};
use sf_records::RecordDb;

mod console;
#[cfg(feature = "debug")]
mod gizmos;

#[cfg(feature = "debug")]
pub use gizmos::AiGizmoVar;

#[cfg(test)]
mod tests;

/// The BRP method [`brp_trace`] answers.
pub const BRP_TRACE_METHOD: &str = "sf_ai/trace";

/// Lines `ai.trace` prints when no count is given.
pub const DEFAULT_TRACE_LINES: usize = 8;

/// Adds [`AiTraces`], [`AiLibrary`] (empty until [`load_library`]), [`AiSelected`] and the
/// `ai.*` console commands; with feature `debug`, the overlay gizmos (drawn only when the app
/// has Bevy's gizmo plugin) and an inspector row.
#[derive(Clone, Copy, Debug)]
pub struct AiPlugin {
    /// Whether decision steps measure `budget_us` (off for reproducible traces).
    pub timings: bool,
}

impl Default for AiPlugin {
    fn default() -> Self {
        Self { timings: true }
    }
}

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        let mut store = TraceStore::new();
        store.set_timings(self.timings);
        app.insert_resource(AiTraces(store))
            .init_resource::<AiLibrary>()
            .init_resource::<AiSelected>()
            .init_resource::<AiDirectives>();
        console::register(app);
        #[cfg(feature = "debug")]
        gizmos::add(app);
    }
}

/// Every traced actor's decision trace (ai.md "Debugging"). Decision code records through
/// [`TraceStore::record`]; the overlay and BRP only read.
#[derive(Resource, Debug, Default)]
pub struct AiTraces(pub TraceStore);

impl Deref for AiTraces {
    type Target = TraceStore;
    fn deref(&self) -> &TraceStore {
        &self.0
    }
}

impl DerefMut for AiTraces {
    fn deref_mut(&mut self) -> &mut TraceStore {
        &mut self.0
    }
}

/// The shared HTN method library, and the report of the load that failed (if the last did).
#[derive(Resource, Debug)]
pub struct AiLibrary {
    pub library: Library,
    /// The last [`load_library`]'s errors; `None` when it loaded (or nothing was loaded yet).
    pub report: Option<LoadReport>,
}

impl Default for AiLibrary {
    fn default() -> Self {
        Self { library: empty_library(), report: None }
    }
}

/// An entity the AI runs: its stable key (placement or spawn id). Whoever spawns the actor
/// inserts it (AI6's executors; until then the game's spawner, or a test).
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AiActor(pub ActorKey);

/// Every actor's directive stack (AI4; `sf_ai::directive`), keyed by [`ActorKey`]: the host's one
/// copy, whether or not the actor's entity is spawned (AI5). Tier 0 keeps every actor logically
/// alive in every worldspace, so a stack never unloads with its cell and streaming can't
/// disagree with a save; an [`AiActor`] entity finds its stack by key. The `sf/ai` section
/// snapshots and replaces this store ([`ai_save`], [`ai_load`]). The fixed-tick systems that
/// push, advance and finish stacks come with AI6/AI7; today the spawner or a test fills it.
/// `ai.directives` prints one.
#[derive(Resource, Clone, Debug, Default, PartialEq)]
pub struct AiDirectives {
    stacks: BTreeMap<ActorKey, DirectiveStack>,
}

impl AiDirectives {
    /// Adds or replaces the stack of `stack.actor()`; returns the old one.
    pub fn insert(&mut self, stack: DirectiveStack) -> Option<DirectiveStack> {
        self.stacks.insert(stack.actor(), stack)
    }

    pub fn get(&self, actor: ActorKey) -> Option<&DirectiveStack> {
        self.stacks.get(&actor)
    }

    pub fn get_mut(&mut self, actor: ActorKey) -> Option<&mut DirectiveStack> {
        self.stacks.get_mut(&actor)
    }

    /// Drops an actor's AI state (it was deleted from the world, not unloaded).
    pub fn remove(&mut self, actor: ActorKey) -> Option<DirectiveStack> {
        self.stacks.remove(&actor)
    }

    /// In key order.
    pub fn iter(&self) -> impl Iterator<Item = &DirectiveStack> {
        self.stacks.values()
    }

    pub fn len(&self) -> usize {
        self.stacks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.stacks.is_empty()
    }

    /// The `sf/ai` value of this store.
    pub fn to_save(&self) -> AiSave {
        AiSave::from_stacks(self.iter())
    }

    /// Replaces the whole store with a loaded `sf/ai` (load is a full replace: an actor the save
    /// lacks gets its record defaults when its stack is next built). A stack that breaks its
    /// invariants is skipped with an error line; the section's check refuses those before any
    /// load applies, so this only guards a caller that skipped it.
    pub fn restore(&mut self, save: AiSave) {
        self.stacks.clear();
        for (actor, a) in save.actors {
            match DirectiveStack::from_saved(actor, a.directives) {
                Ok(stack) => {
                    self.stacks.insert(actor, stack);
                }
                Err(e) => log::error!(target: "sf_ai", "sf/ai: actor {}: {e}; record defaults instead", actor_hex(actor)),
            }
        }
    }
}

/// `get` of the `sf/ai` section (`sf_save_bevy::sections::ai`): `None` when no actor has AI state,
/// so a world without AI writes no section (and loads back as one).
pub fn ai_save(world: &World) -> Option<AiSave> {
    world.get_resource::<AiDirectives>().filter(|d| !d.is_empty()).map(AiDirectives::to_save)
}

/// `set` of the `sf/ai` section: replaces [`AiDirectives`] (inserted if absent).
pub fn ai_load(world: &mut World, save: AiSave) {
    world.get_resource_or_insert_with(AiDirectives::default).restore(save);
}

/// `ai.directives`'s output for `actor`: a header line, then [`format_directives`].
pub fn directives_text(world: &mut World, actor: ActorKey) -> Result<String, String> {
    let stack = world
        .get_resource::<AiDirectives>()
        .and_then(|d| d.get(actor))
        .ok_or_else(|| format!("actor {}: no directive stack", actor_hex(actor)))?;
    let mut out = format!("actor {}: directives, top first", actor_hex(actor));
    out.push('\n');
    out.push_str(&format_directives(stack));
    Ok(out)
}

/// The actor the overlay shows and `ai.trace` reads by default (`ai.select`).
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AiSelected(pub Option<ActorKey>);

fn empty_library() -> Library {
    Library::from_defs(std::iter::empty(), std::iter::empty(), &StateKeys::new(), &sf_ai::Operators::new())
        .expect("an empty HTN library always builds")
}

/// Builds the HTN library from `db`'s `sf.ai.task` / `sf.ai.method` records (registered with
/// `sf_ai::register_kinds` before mods load) against the vocabulary the executors and
/// perception declare, and stores it in [`AiLibrary`]. On errors it logs one `error` line per
/// problem (target `sf_ai`, text from [`report_lines`]), keeps an **empty** library (NPCs plan
/// nothing rather than half a library) and returns `false`; the app keeps running.
pub fn load_library(world: &mut World, db: &RecordDb, keys: &StateKeys, operators: &sf_ai::Operators) -> bool {
    let (library, report) = match Library::build(db, keys, operators) {
        Ok(library) => {
            log::info!(
                target: "sf_ai",
                "HTN library: {} tasks, {} methods",
                library.task_count(),
                library.method_count()
            );
            (library, None)
        }
        Err(report) => {
            for line in report_lines(&report) {
                log::error!(target: "sf_ai", "{line}");
            }
            (empty_library(), Some(report))
        }
    };
    let ok = report.is_none();
    world.insert_resource(AiLibrary { library, report });
    ok
}

/// The log lines of a failed load: one per error, then the consequence.
pub fn report_lines(report: &LoadReport) -> Vec<String> {
    let mut lines: Vec<String> = report.errors.iter().map(|e| format!("HTN library: {e}")).collect();
    lines.push(format!("HTN library: {} error(s); NPCs plan with an empty library", report.errors.len()));
    lines
}

/// `key` as 32 hex digits (the way `PlacementId` prints).
pub fn actor_hex(key: ActorKey) -> String {
    format!("{:032x}", key.0)
}

/// Parses an actor key typed as hex (up to 32 digits, optional `0x`).
pub fn parse_actor(text: &str) -> Option<ActorKey> {
    let digits = text.trim().trim_start_matches("0x");
    if digits.is_empty() || digits.len() > 32 {
        return None;
    }
    u128::from_str_radix(digits, 16).ok().map(ActorKey)
}

/// Names through the library, falling back to indices (`t3`) for ids it doesn't know (a trace
/// recorded against another library, or synthetic entries).
pub struct Names<'a>(pub &'a Library);

impl TraceNames for Names<'_> {
    fn task(&self, out: &mut String, id: TaskId) {
        match self.0.task(id) {
            Some(_) => out.push_str(self.0.task_name(id)),
            None => {
                let _ = write!(out, "{id}");
            }
        }
    }
    fn method(&self, out: &mut String, id: MethodId) {
        match self.0.method(id) {
            Some(_) => out.push_str(self.0.method_name(id)),
            None => {
                let _ = write!(out, "{id}");
            }
        }
    }
    fn operator(&self, out: &mut String, id: OperatorId) {
        if usize::from(id.0) < self.0.operators().len() {
            out.push_str(self.0.operators().name(id));
        } else {
            let _ = write!(out, "{id}");
        }
    }
}

/// `ai.trace`'s output for `actor`: a header line, then the last `n` entries
/// ([`format_trace_with`] through [`Names`]).
pub fn trace_text(traces: &TraceStore, library: &Library, actor: ActorKey, n: usize) -> Result<String, String> {
    let Some(trace) = traces.get(actor) else {
        let why = if !traces.enabled() {
            "tracing is off"
        } else if traces.is_watched(actor) {
            "watched, no decision step yet"
        } else {
            "not traced; `ai.select` it to trace it from its next step"
        };
        return Err(format!("actor {}: {why}", actor_hex(actor)));
    };
    let shown = trace.len().min(n);
    let mut out = format!("actor {}: last {shown} of {} entries", actor_hex(actor), trace.len());
    out.push('\n');
    out.push_str(&format_trace_with(trace, n, &Names(library)));
    Ok(out)
}

/// The overlay text for `actor` (the `debug` gizmos draw these lines above it; plain so tests
/// check them): who, then the last decision's tick, tier and plan outcome, the plan with the
/// running step in brackets, the operator. Nothing recorded yet says so; nothing is invented.
pub fn overlay_lines(traces: &TraceStore, library: &Library, actor: ActorKey) -> Vec<String> {
    let names = Names(library);
    let watched = if traces.is_watched(actor) { " (watched)" } else { "" };
    let mut lines = vec![format!("ai {}{watched}", actor_hex(actor))];
    let Some(e) = traces.get(actor).and_then(Trace::last) else {
        lines.push(if traces.enabled() { "no decision traced yet".into() } else { "tracing is off".into() });
        return lines;
    };
    let outcome = e.outcome.map_or_else(|| "-".to_owned(), outcome_name);
    lines.push(format!("#{} t{} plan {outcome}", e.tick, e.tier));
    let mut plan = String::from("plan");
    if e.plan.steps.is_empty() {
        plan.push_str(" -");
    }
    for (i, step) in e.plan.steps.iter().enumerate() {
        plan.push_str(if i == 0 { " " } else { " > " });
        let running = i == usize::from(e.plan.cursor);
        if running {
            plan.push('[');
        }
        names.task(&mut plan, step.task);
        if running {
            plan.push(']');
        }
    }
    if e.plan.partial {
        plan.push_str(" | paused");
    }
    lines.push(plan);
    let mut op = String::from("op ");
    match e.operator {
        Some(o) => {
            names.operator(&mut op, o.operator);
            op.push(' ');
            op.push_str(&snake_name(&o.status));
        }
        None => op.push('-'),
    }
    lines.push(op);
    lines
}

/// `snake_case` name of a plan outcome, as the trace's JSON and `format_trace` spell it.
pub fn outcome_name(outcome: PlanOutcome) -> String {
    snake_name(&outcome)
}

/// A unit enum's serde name (`snake_case` in `sf_ai`). Through serde rather than a `match`, so a
/// variant `sf_ai` adds later (`AI2b` backtracking, ...) prints without a change here.
pub(crate) fn snake_name<T: serde::Serialize>(value: &T) -> String {
    match serde_json::to_value(value) {
        Ok(Value::String(s)) => s,
        Ok(other) => other.to_string(),
        Err(_) => "?".to_owned(),
    }
}

/// [`brp_trace`]'s parameters (all optional).
#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
struct TraceParams {
    /// Only this actor (hex key).
    actor: Option<String>,
    /// Only the last `n` entries of each trace.
    last: Option<usize>,
}

/// The [`BRP_TRACE_METHOD`] answer: `{"enabled", "cap", "selected", "actors": [{"actor",
/// "watched", "entries": [...]}]}`, actors in key order, entries oldest first in
/// `TraceEntry`'s field order. Keys are 32-hex **strings**, not JSON numbers: a 128-bit
/// placement id doesn't fit a `serde_json::Value` number or a JavaScript client's.
/// Params: `{"actor": "<hex>", "last": n}`, both optional; `Err` for bad params.
pub fn brp_trace(world: &World, params: Option<Value>) -> Result<Value, String> {
    let params: TraceParams = match params {
        None | Some(Value::Null) => TraceParams::default(),
        Some(v) => serde_json::from_value(v).map_err(|e| format!("{BRP_TRACE_METHOD}: {e}"))?,
    };
    let only = match params.actor.as_deref() {
        Some(text) => {
            Some(parse_actor(text).ok_or_else(|| format!("{BRP_TRACE_METHOD}: bad actor key {text:?} (hex)"))?)
        }
        None => None,
    };
    let traces = &world.get_resource::<AiTraces>().ok_or("sf_ai: AiPlugin is not in this app")?.0;
    let selected = world.get_resource::<AiSelected>().and_then(|s| s.0);
    let mut actors = Vec::new();
    for (actor, trace) in traces.iter().filter(|(a, _)| only.is_none_or(|o| o == *a)) {
        let skip = params.last.map_or(0, |n| trace.len().saturating_sub(n));
        let entries: Vec<_> = trace.iter().skip(skip).collect();
        actors.push(json!({
            "actor": actor_hex(actor),
            "watched": traces.is_watched(actor),
            "entries": serde_json::to_value(entries).map_err(|e| e.to_string())?,
        }));
    }
    Ok(json!({
        "enabled": traces.enabled(),
        "cap": traces.cap(),
        "selected": selected.map(actor_hex),
        "actors": actors,
    }))
}

/// [`brp_trace`] as a one-shot system, the shape `bevy_remote`'s `with_method_main` takes once
/// the error is mapped. In the app that owns BRP:
///
/// ```ignore
/// RemotePlugin::default().with_method_main(sf_ai_bevy::BRP_TRACE_METHOD,
///     |In(p): In<Option<Value>>, w: &World| sf_ai_bevy::brp_trace_system(In(p), w)
///         .map_err(|message| BrpError { code: error_codes::INVALID_PARAMS, message, data: None }))
/// ```
pub fn brp_trace_system(In(params): In<Option<Value>>, world: &World) -> Result<Value, String> {
    brp_trace(world, params)
}
