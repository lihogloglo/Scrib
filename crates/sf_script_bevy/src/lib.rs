//! Bevy plugin for [`sf_script`] (scripting.md "Execution model and threading with Bevy's ECS").
//!
//! - [`Scripts`]: the [`ScriptRuntime`] as an ordinary resource (`sf_script`'s `send` feature).
//! - [`ScriptSlot`]: three exclusive systems at fixed points: `Input` (`PreUpdate`), `Update`
//!   (`Update`) and `Ui` (`PostUpdate`). Apps order their systems against the sets: input and UI
//!   focus before `Input`; fact-emitting gameplay before `Update`; default actions (open a door)
//!   and physics write-back after it; UI layout after `Ui`.
//! - Engine → script facts: [`Activated`] (the activation pipeline, answered by
//!   [`ActivationOutcome`]) and [`ScriptFact`] (any other fact, for the slot it names).
//! - World access: resources moved into the slot (`access.rs`), read by API modules through
//!   [`with_resource`]; `@sf/world` (L2 subset, `world.rs`) over [`ScriptPlacements`].
//! - Per-context time, calls, errors and memory go to `sf_diag` counters (`script/...`); faults
//!   and notices surface as [`ScriptDiagnostics`] messages and in [`Scripts::recent`].

mod access;
mod world;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_api;

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use std::time::{Duration, Instant};

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_ecs::component::Mutable;
use bevy_ecs::system::SystemState;
use bevy_time::Time;
use sf_diag_bevy::DiagCounters;
use sf_diag_bevy::sf_diag::unit;
pub use sf_script;
use sf_script::{ContextConfig, Handle, ScriptDiagnostic, ScriptRuntime, ScriptSource, StateValue, Status};

pub use access::{SlotAccess, mark_changed, with_resource};
use access::Mover;
pub use world::{PLAYER, Placed, ScriptPlacements, ScriptWorld, WorldCommand, WorldModule};

/// The three script slots (scripting.md table "Slot").
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ScriptSlot {
    /// `PreUpdate`: input actions and UI events, synchronously.
    Input,
    /// `Update`: engine facts (activation, cells), queued script events, timers, `update(dt)`.
    Update,
    /// `PostUpdate`: UI-phase facts (view-model writes; grows with `sf_ui_lua`, U7).
    Ui,
}

impl ScriptSlot {
    fn label(self) -> &'static str {
        match self {
            ScriptSlot::Input => "input",
            ScriptSlot::Update => "update",
            ScriptSlot::Ui => "ui",
        }
    }
}

/// An actor activated a placement (the player's activate key on the crosshair target, an AI
/// using a door). Offered to scripts in the `Update` slot as event `"activated"` with payload
/// `{ target = Handle, actor = Handle }`; a handler returning `"handled"` suppresses the default.
#[derive(Message, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Activated {
    pub target: Handle,
    /// [`PLAYER`] for the player.
    pub actor: Handle,
}

/// Written after the `Update` slot for every [`Activated`]: default actions run when
/// `handled` is false (systems ordered `.after(ScriptSlot::Update)`).
#[derive(Message, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActivationOutcome {
    pub activation: Activated,
    pub handled: bool,
}

/// Any other engine fact, dispatched by name in the slot it names (`"cell_loaded"` in `Update`,
/// `"input_action"` in `Input`, ...). Payloads are plain data.
#[derive(Message, Clone, Debug, PartialEq)]
pub struct ScriptFact {
    pub slot: ScriptSlot,
    pub name: String,
    pub payload: StateValue,
}

impl ScriptFact {
    /// `"input_action"` `{ action, pressed }` in the `Input` slot.
    pub fn input_action(action: &str, pressed: bool) -> Self {
        Self {
            slot: ScriptSlot::Input,
            name: "input_action".into(),
            payload: table([("action", StateValue::Str(action.as_bytes().to_vec())), ("pressed", StateValue::Bool(pressed))]),
        }
    }
}

/// A table payload from string keys (sorted: the canonical order).
pub fn table<const N: usize>(fields: [(&str, StateValue); N]) -> StateValue {
    let mut entries: Vec<_> = fields.into_iter().map(|(k, v)| (sf_script::StateKey::Str(k.as_bytes().to_vec()), v)).collect();
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    StateValue::Table(entries)
}

/// Faults and notices of one slot (also logged by `sf_script`).
#[derive(Message, Clone, Debug)]
pub struct ScriptDiagnostics(pub Vec<ScriptDiagnostic>);

/// How many diagnostics [`Scripts::recent`] keeps.
const RECENT: usize = 64;

/// Last reported totals of one context (counters report per-slot deltas).
#[derive(Clone, Copy, Default)]
struct Totals {
    time: Duration,
    calls: u64,
    errors: u64,
    aborts: u64,
}

/// The script runtime as a resource, plus what the slots need around it.
#[derive(Resource)]
pub struct Scripts {
    pub runtime: ScriptRuntime,
    access: SlotAccess,
    movers: Vec<Mover>,
    totals: HashMap<String, Totals>,
    recent: VecDeque<ScriptDiagnostic>,
    /// Diagnostics not yet sent as messages (start-up, reloads).
    unsent: Vec<ScriptDiagnostic>,
}

impl Default for Scripts {
    fn default() -> Self {
        Self::new(ScriptRuntime::default())
    }
}

impl Scripts {
    /// Wraps a runtime and registers `@sf/world` (which installs the slot guard in every context
    /// started afterwards) and [`ScriptPlacements`] as a slot resource.
    pub fn new(mut runtime: ScriptRuntime) -> Self {
        let access = SlotAccess::default();
        runtime.register_module(Box::new(WorldModule { access: access.clone() }));
        let mut s = Self {
            runtime,
            access,
            movers: Vec::new(),
            totals: HashMap::new(),
            recent: VecDeque::new(),
            unsent: Vec::new(),
        };
        s.register_slot_resource::<ScriptPlacements>();
        s.register_slot_resource::<ScriptWorld>();
        s
    }

    /// Declares a resource API functions may borrow during slots ([`with_resource`]).
    pub fn register_slot_resource<T: Resource<Mutability = Mutable> + Default>(&mut self) {
        let m = Mover::of::<T>();
        if !self.movers.iter().any(|x| x.id == m.id) {
            self.movers.push(m);
        }
    }

    /// Starts contexts in load order (no world access during start: entry points register
    /// handlers). Failures are kept as diagnostics and sent at the next slot.
    pub fn start(&mut self, configs: Vec<ContextConfig>, source: Arc<dyn ScriptSource>) {
        let diags = self.runtime.start(configs, source);
        self.note(diags);
    }

    /// Hot reload of one context (A4 drives it). The error is also kept as a diagnostic.
    pub fn reload(&mut self, id: &str, source: Option<Arc<dyn ScriptSource>>) -> Result<(), sf_script::ScriptError> {
        let r = self.runtime.reload(id, source);
        let mut diags = self.runtime.take_diagnostics();
        if let Err(e) = &r {
            diags.push(ScriptDiagnostic { context: id.to_owned(), kind: None, message: format!("reload failed: {e}") });
        }
        self.note(diags);
        r
    }

    /// Hot reload of changed script files (mods.md "Hot reload"): reloads, in load order, every
    /// context that runs one of `changed` (under `scripts/<mod>/` of its mods) or can `require` it
    /// (a dependency's library), from `source` (the new VFS snapshot). Returns each reloaded
    /// context with its result; failures keep the old state running and are also diagnostics.
    pub fn reload_changed<'a>(
        &mut self,
        changed: impl IntoIterator<Item = &'a str>,
        source: &Arc<dyn ScriptSource>,
    ) -> Vec<(String, Result<(), sf_script::ScriptError>)> {
        let changed: Vec<&str> = changed.into_iter().collect();
        let affected: Vec<String> = self
            .runtime
            .contexts()
            .iter()
            .filter(|c| {
                let config = c.config();
                config.mods.iter().chain(&config.dependencies).any(|m| {
                    let root = format!("{}/", sf_script::script_root(m));
                    changed.iter().any(|p| p.starts_with(&root))
                })
            })
            .map(|c| c.id().to_owned())
            .collect();
        affected
            .into_iter()
            .map(|id| {
                let r = self.reload(&id, Some(Arc::clone(source)));
                (id, r)
            })
            .collect()
    }

    /// The last [`RECENT`] faults and notices, oldest first (dev overlay, mod manager badge).
    pub fn recent(&self) -> impl Iterator<Item = &ScriptDiagnostic> {
        self.recent.iter()
    }

    /// Ids of suspended or failed contexts.
    pub fn stopped(&self) -> impl Iterator<Item = (&str, &Status)> {
        self.runtime.contexts().iter().filter(|c| !c.is_running()).map(|c| (c.id(), c.status()))
    }

    fn note(&mut self, diags: Vec<ScriptDiagnostic>) {
        for d in &diags {
            if self.recent.len() == RECENT {
                self.recent.pop_front();
            }
            self.recent.push_back(d.clone());
        }
        self.unsent.extend(diags);
    }

    /// Per-slot report: counters (`script/<slot>/ms`, `script/ctx/<id>/{ms,calls,errors,aborts}`,
    /// memory gauges at the `Update` slot) and diagnostics.
    fn report(&mut self, slot: ScriptSlot, wall: Duration, counters: Option<&DiagCounters>) -> Vec<ScriptDiagnostic> {
        let diags = self.runtime.take_diagnostics();
        self.note(diags);
        if let Some(c) = counters {
            c.add_duration_named(&format!("script/{}/ms", slot.label()), wall);
            for ctx in self.runtime.contexts() {
                let s = ctx.stats();
                let last = self.totals.entry(ctx.id().to_owned()).or_default();
                let id = ctx.id();
                if s.time > last.time {
                    c.add_duration_named(&format!("script/ctx/{id}/ms"), s.time.saturating_sub(last.time));
                }
                #[allow(clippy::cast_precision_loss, reason = "per-frame counts")]
                for (name, now, before) in
                    [("calls", s.calls, last.calls), ("errors", s.errors, last.errors), ("aborts", s.aborts, last.aborts)]
                {
                    if now > before {
                        c.add_named(&format!("script/ctx/{id}/{name}"), unit::COUNT, (now - before) as f64);
                    }
                }
                if slot == ScriptSlot::Update {
                    #[allow(clippy::cast_precision_loss, reason = "bytes as MiB")]
                    c.set_named(&format!("script/ctx/{id}/mib"), unit::COUNT, ctx.used_memory() as f64 / f64::from(1 << 20));
                }
                *last = Totals { time: s.time, calls: s.calls, errors: s.errors, aborts: s.aborts };
            }
        }
        std::mem::take(&mut self.unsent)
    }
}

/// Adds the slots, messages and (unless the app inserted one) an empty [`Scripts`].
#[derive(Default)]
pub struct ScriptsPlugin;

impl Plugin for ScriptsPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<Activated>()
            .add_message::<ActivationOutcome>()
            .add_message::<ScriptFact>()
            .add_message::<ScriptDiagnostics>()
            .add_message::<WorldCommand>()
            .init_resource::<ScriptWorld>()
            .init_resource::<ScriptPlacements>()
            .configure_sets(PreUpdate, ScriptSlot::Input)
            .configure_sets(Update, ScriptSlot::Update)
            .configure_sets(PostUpdate, ScriptSlot::Ui)
            .add_systems(PreUpdate, input_slot.in_set(ScriptSlot::Input))
            .add_systems(Update, update_slot.in_set(ScriptSlot::Update))
            .add_systems(PostUpdate, ui_slot.in_set(ScriptSlot::Ui));
        if !app.world().contains_resource::<Scripts>() {
            app.insert_resource(Scripts::default());
        }
    }
}

type FactReader = SystemState<MessageReader<'static, 'static, ScriptFact>>;
type UpdateReaders = SystemState<(MessageReader<'static, 'static, Activated>, MessageReader<'static, 'static, ScriptFact>)>;

/// Runs one slot: opens world access, runs `body` on the runtime, closes it, reports.
fn run_slot(world: &mut World, slot: ScriptSlot, body: impl FnOnce(&mut Scripts)) {
    if !world.contains_resource::<Scripts>() {
        return;
    }
    let diags = world.resource_scope(|world, mut scripts: Mut<Scripts>| {
        if scripts.runtime.contexts().is_empty() && scripts.unsent.is_empty() {
            return Vec::new();
        }
        let start = Instant::now();
        let movers = scripts.movers.clone();
        scripts.access.open(world, &movers);
        body(&mut scripts);
        scripts.access.close(world, &movers);
        let counters = world.get_resource::<DiagCounters>().cloned();
        scripts.report(slot, start.elapsed(), counters.as_ref())
    });
    if !diags.is_empty() {
        world.write_message(ScriptDiagnostics(diags));
    }
    // Structural changes scripts queued (spawn, despawn): the content pack applies them.
    let commands = world.get_resource_mut::<ScriptWorld>().map(|mut w| w.bypass_change_detection().take_commands());
    for c in commands.unwrap_or_default() {
        world.write_message(c);
    }
}

fn dispatch_facts(scripts: &mut Scripts, facts: Vec<ScriptFact>) {
    for f in facts {
        scripts.runtime.dispatch(&f.name, f.payload);
    }
}

fn input_slot(world: &mut World, reader: &mut FactReader) {
    let facts: Vec<ScriptFact> = reader.get_mut(world).map(|mut r| r.read().filter(|f| f.slot == ScriptSlot::Input).cloned().collect()).unwrap_or_default();
    run_slot(world, ScriptSlot::Input, |s| dispatch_facts(s, facts));
}

fn ui_slot(world: &mut World, reader: &mut FactReader) {
    let facts: Vec<ScriptFact> = reader.get_mut(world).map(|mut r| r.read().filter(|f| f.slot == ScriptSlot::Ui).cloned().collect()).unwrap_or_default();
    run_slot(world, ScriptSlot::Ui, |s| dispatch_facts(s, facts));
}

/// `on_load` handlers queued by start, reload or restore (here, so they have world access), then
/// engine facts (activations first, in arrival order: the target's entity scripts, then global
/// subscribers), then queued script events, due timers and `update(dt)` handlers
/// (`ScriptRuntime::update`), then entity scripts' `tick` for loaded placements.
fn update_slot(world: &mut World, readers: &mut UpdateReaders) {
    let Ok((mut activated, mut facts)) = readers.get_mut(world) else { return };
    let activations: Vec<Activated> = activated.read().copied().collect();
    let facts: Vec<ScriptFact> = facts.read().filter(|f| f.slot == ScriptSlot::Update).cloned().collect();
    let dt = world.get_resource::<Time>().map_or(0.0, Time::delta_secs_f64);
    let mut outcomes: Vec<ActivationOutcome> =
        activations.into_iter().map(|activation| ActivationOutcome { activation, handled: false }).collect();
    run_slot(world, ScriptSlot::Update, |s| {
        s.runtime.run_loads();
        for o in &mut outcomes {
            let a = o.activation;
            let payload = table([("target", StateValue::Handle(a.target)), ("actor", StateValue::Handle(a.actor))]);
            o.handled = s.runtime.activate(a.target, payload).handled;
        }
        dispatch_facts(s, facts);
        s.runtime.update(dt);
        let attached = s.runtime.attached();
        if !attached.is_empty() {
            // Read before ticking: handlers borrow the cell themselves.
            let loaded: HashSet<Handle> = s
                .access
                .with::<ScriptPlacements, _>(false, |p| attached.into_iter().filter(|h| p.loaded.contains_key(h)).collect())
                .unwrap_or_default();
            s.runtime.tick_entities(dt, &|h| loaded.contains(&h));
        }
    });
    for o in outcomes {
        world.write_message(o);
    }
}
