//! Global script instances (scripting-adapter.md "Instances, scheduling, state", DQ-4a): the
//! parsed script set and the global instances `StartScript` creates, their per-tick order, faults
//! and save snapshot. Bevy-free; `mw_scripts_bevy` schedules [`GlobalInstances::tick`] and owns
//! the save section (DQ-4b).
//!
//! Ported from `OpenMW` `apps/openmw/mwscript/globalscripts.cpp` (github.com/OpenMW/openmw,
//! commit `c31b4efc01f3d34c305aa6065d729bc840bcf8f3`), which matches the spec on:
//!
//! - `StartScript` on a running script does nothing; on a stopped one it sets running and the new
//!   target and keeps the locals (`GlobalScripts::addScript`); a new instance starts with zeroed
//!   locals.
//! - `StopScript` clears the flag (`removeScript`); a script stopping itself finishes its current
//!   run (the interpreter never checks the flag mid-run), and a script stopped by an earlier one
//!   in the same tick doesn't run (`run` checks the flag per script).
//! - A failed run stops the instance (`run`: `mRunning = false` when the script manager fails).
//! - Loads restore running and target, read locals by name, and drop scripts whose record is gone
//!   (`readRecord`).
//!
//! Where it differs, this module follows the spec, for determinism (net-ready, semantics rule 1):
//! `OpenMW` runs globals in `std::unordered_map` order, so a script started mid-frame may or may
//! not run that frame; here they run in start order (a restart takes a new sequence number) and a
//! script started during a tick first runs on the next. Its ops (`StartScript` / `StopScript`)
//! apply after the run, not at the call; `ScriptRunning` inside the run sees them, so the run reads
//! what `OpenMW` would, and a step-limit fault can drop them.

use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

use crate::aifn::AiCall;
use crate::ast::{Call, Ident, Script, Target, VarRef, VarType};
use crate::functions::{Function, FunctionTable};
use crate::interp::{Host, Interpreter, Locals, MessageArg, RunError, ScriptOp, store};
use crate::locals::LocalKey;
use crate::resolve::Env;
use crate::statfn::StatCall;
use crate::{Diagnostics, Parsed, SourceKind, parse_and_resolve};

/// The lookup form of a script id (as [`Ident::key`]).
pub(crate) fn key(id: &str) -> String {
    id.to_lowercase()
}

/// Every live script by lower-cased id, parsed once at content load. Inert scripts are kept apart
/// with their diagnostics: they are reported and never run.
#[derive(Clone, Debug, Default)]
pub struct ScriptSet {
    scripts: HashMap<String, Script>,
    inert: BTreeMap<String, Diagnostics>,
}

impl ScriptSet {
    /// Adds (or replaces) the script `id`. Returns `false` when it is inert.
    pub fn insert(&mut self, id: &str, parsed: Parsed) -> bool {
        let id = key(id);
        if parsed.is_inert() {
            self.scripts.remove(&id);
            self.inert.insert(id, parsed.diagnostics);
            false
        } else {
            self.inert.remove(&id);
            self.scripts.insert(id, parsed.script);
            true
        }
    }

    /// Parses and resolves `text` as a script record and [`ScriptSet::insert`]s it.
    pub fn parse_insert(
        &mut self,
        id: &str,
        text: &str,
        table: &FunctionTable,
        env: &dyn Env,
    ) -> bool {
        self.insert(id, parse_and_resolve(text, SourceKind::Script, table, env))
    }

    /// A live script (case-insensitive id).
    pub fn get(&self, id: &str) -> Option<&Script> {
        self.scripts.get(&key(id))
    }

    pub fn is_inert(&self, id: &str) -> bool {
        self.inert.contains_key(&key(id))
    }

    /// The inert scripts, by id, with what made them inert.
    pub fn inert(&self) -> impl Iterator<Item = (&str, &Diagnostics)> {
        self.inert.iter().map(|(k, d)| (k.as_str(), d))
    }

    /// Live scripts.
    pub fn len(&self) -> usize {
        self.scripts.len()
    }

    pub fn is_empty(&self) -> bool {
        self.scripts.is_empty()
    }
}

/// One global script instance.
#[derive(Clone, Debug, PartialEq)]
pub struct GlobalInstance {
    /// The lower-cased script id.
    pub id: String,
    pub running: bool,
    /// Slot values of [`Script::locals`], stored converted.
    pub locals: Locals,
    /// The `ref->` of the `StartScript` that last started it (caller-supplied reference key).
    pub target: Option<String>,
    /// Start order: taken from a counter at every start and restart.
    pub start_seq: u64,
}

/// Something the instance set reports once, drained with [`GlobalInstances::take_reports`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Report {
    /// The run hit the step limit: its ops were dropped and the instance stopped until the next
    /// `StartScript`.
    StepLimit { script: String, line: u32 },
    /// `StartScript` of an id with no script record: nothing started.
    UnknownScript { script: String },
    /// `StartScript` of an inert script: nothing started (inert scripts never run).
    InertScript { script: String },
}

/// What a load could not restore as saved ([`GlobalInstances::load`]).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LoadReport {
    /// The saved script has no live record any more (removed or now inert): dropped.
    UnknownScript { script: String },
    /// The script has a local the save lacks: it starts at 0.
    MissingLocal { script: String, local: String },
    /// The save has a local the script no longer declares: dropped.
    DroppedLocal { script: String, local: String },
    /// The local changed type: its value is converted to the new type.
    Retyped { script: String, local: String, from: LocalType, to: LocalType },
}

/// A local's declared type in a save (mirrors [`VarType`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LocalType {
    Short,
    Long,
    Float,
}

impl From<VarType> for LocalType {
    fn from(ty: VarType) -> Self {
        match ty {
            VarType::Short => LocalType::Short,
            VarType::Long => LocalType::Long,
            VarType::Float => LocalType::Float,
        }
    }
}

impl From<LocalType> for VarType {
    fn from(ty: LocalType) -> Self {
        match ty {
            LocalType::Short => VarType::Short,
            LocalType::Long => VarType::Long,
            LocalType::Float => VarType::Float,
        }
    }
}

/// A saved local, by name and type.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SavedLocal {
    /// Lower-cased name.
    pub name: String,
    pub ty: LocalType,
    pub value: f64,
}

/// A saved global instance.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SavedGlobal {
    pub id: String,
    pub running: bool,
    pub target: Option<String>,
    pub start_seq: u64,
    pub locals: Vec<SavedLocal>,
}

/// The global part of the script save section, in start order.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct GlobalsSnapshot {
    pub globals: Vec<SavedGlobal>,
}

/// What [`GlobalInstances::tick`] needs from the game beyond [`Host`]: it brackets every run so
/// the host can seed `Random` by instance, resolve the implicit target, and apply the run's
/// buffered ops or, after a fault, drop them.
pub trait InstanceHost: Host {
    /// Before a run of global `script` (`target`: its `StartScript` reference).
    fn begin_run(&mut self, script: &str, target: Option<&str>) {
        let _ = (script, target);
    }
    /// After a run (global or local): `committed` is `false` when the run hit the step limit
    /// (drop its ops).
    fn end_run(&mut self, script: &str, committed: bool) {
        let _ = (script, committed);
    }
    /// Before a run of local `script` on the reference or item `key`
    /// ([`crate::LocalInstances::tick`]): the run's self is that object. Default:
    /// [`InstanceHost::begin_run`] with no target.
    fn begin_local_run(&mut self, script: &str, key: LocalKey) {
        let _ = key;
        self.begin_run(script, None);
    }
    /// The reference id a global started from `key`'s local run takes as its target when the
    /// `StartScript` names no `ref->` (`OpenMW` passes the running object). Default: none.
    fn local_self(&mut self, key: LocalKey) -> Option<String> {
        let _ = key;
        None
    }
}

/// An op a run requested, applied after the run.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Pending {
    Start { id: String, target: Option<String> },
    Stop { id: String },
}

/// The global instances, keyed by script id. Instances are never removed (a stopped one keeps
/// its locals for the next start), as in `OpenMW`.
#[derive(Clone, Debug, Default)]
pub struct GlobalInstances {
    instances: Vec<GlobalInstance>,
    index: HashMap<String, usize>,
    next_seq: u64,
    reports: Vec<Report>,
}

impl GlobalInstances {
    /// `StartScript id` from outside a run (start scripts at a new game, a dialogue result, the
    /// console): runs from the next tick.
    pub fn start(&mut self, set: &ScriptSet, id: &str, target: Option<&str>) {
        self.apply(
            set,
            Pending::Start {
                id: key(id),
                target: target.map(str::to_owned),
            },
        );
    }

    /// `StopScript id` from outside a run.
    pub fn stop(&mut self, id: &str) {
        self.apply_stop(&key(id));
    }

    /// `ScriptRunning id`.
    pub fn is_running(&self, id: &str) -> bool {
        self.get(id).is_some_and(|g| g.running)
    }

    pub fn get(&self, id: &str) -> Option<&GlobalInstance> {
        self.index.get(&key(id)).map(|&i| &self.instances[i])
    }

    /// For `scriptid.var` writes from other scripts and dialogue.
    pub fn get_mut(&mut self, id: &str) -> Option<&mut GlobalInstance> {
        self.index.get(&key(id)).map(|&i| &mut self.instances[i])
    }

    /// Every instance, in start order.
    pub fn iter(&self) -> impl Iterator<Item = &GlobalInstance> {
        let mut all: Vec<&GlobalInstance> = self.instances.iter().collect();
        all.sort_by_key(|g| g.start_seq);
        all.into_iter()
    }

    /// The reports since the last call.
    pub fn take_reports(&mut self) -> Vec<Report> {
        std::mem::take(&mut self.reports)
    }

    /// One tick: every global running at its start, in start order, one after the other. Each
    /// run's ops apply before the next run starts; a script started (or restarted) during the
    /// tick first runs on the next, and one stopped by an earlier run doesn't run.
    pub fn tick<H: InstanceHost + ?Sized>(
        &mut self,
        set: &ScriptSet,
        interp: &Interpreter,
        host: &mut H,
    ) {
        let mut order: Vec<(u64, usize)> = self
            .instances
            .iter()
            .enumerate()
            .filter(|(_, g)| g.running)
            .map(|(i, g)| (g.start_seq, i))
            .collect();
        order.sort_unstable();
        let mut pending = Vec::new();
        for (seq, i) in order {
            let inst = &self.instances[i];
            if !inst.running || inst.start_seq != seq {
                continue;
            }
            let Some(script) = set.get(&inst.id) else {
                continue;
            };
            let mut locals = std::mem::take(&mut self.instances[i].locals);
            locals.0.resize(script.locals.len(), 0.0);
            let inst = &self.instances[i];
            host.begin_run(&inst.id, inst.target.as_deref());
            let result = {
                let mut scoped = Scoped {
                    inner: &mut *host,
                    globals: self,
                    self_target: inst.target.clone(),
                    pending: &mut pending,
                };
                interp.run(script, &mut locals, &mut scoped)
            };
            self.instances[i].locals = locals;
            host.end_run(&self.instances[i].id, result.is_ok());
            match result {
                Ok(()) => {
                    for op in pending.drain(..) {
                        self.apply(set, op);
                    }
                }
                Err(RunError::StepLimit { line }) => {
                    pending.clear();
                    let inst = &mut self.instances[i];
                    inst.running = false;
                    self.reports.push(Report::StepLimit {
                        script: inst.id.clone(),
                        line,
                    });
                }
            }
        }
    }

    /// The save section's global part: locals by name and type from the script set.
    pub fn snapshot(&self, set: &ScriptSet) -> GlobalsSnapshot {
        let globals = self
            .iter()
            .map(|g| SavedGlobal {
                id: g.id.clone(),
                running: g.running,
                target: g.target.clone(),
                start_seq: g.start_seq,
                locals: save_locals(set.get(&g.id), &g.locals),
            })
            .collect();
        GlobalsSnapshot { globals }
    }

    /// Restores a [`GlobalInstances::snapshot`] against the current script set: locals are
    /// remapped by name (missing ones start at 0, extras are dropped, a changed type converts),
    /// and scripts without a live record are dropped; each case is reported.
    pub fn load(snapshot: &GlobalsSnapshot, set: &ScriptSet) -> (Self, Vec<LoadReport>) {
        let mut out = Self::default();
        let mut reports = Vec::new();
        for saved in &snapshot.globals {
            let id = key(&saved.id);
            let Some(script) = set.get(&id) else {
                reports.push(LoadReport::UnknownScript { script: id });
                continue;
            };
            let locals = load_locals(&id, script, &saved.locals, &mut reports);
            out.next_seq = out.next_seq.max(saved.start_seq + 1);
            let instance = GlobalInstance {
                id: id.clone(),
                running: saved.running,
                locals,
                target: saved.target.clone(),
                start_seq: saved.start_seq,
            };
            if let Some(&i) = out.index.get(&id) {
                out.instances[i] = instance;
            } else {
                out.index.insert(id, out.instances.len());
                out.instances.push(instance);
            }
        }
        (out, reports)
    }

    fn take_seq(&mut self) -> u64 {
        let seq = self.next_seq;
        self.next_seq += 1;
        seq
    }

    pub(crate) fn apply(&mut self, set: &ScriptSet, op: Pending) {
        match op {
            Pending::Stop { id } => self.apply_stop(&id),
            Pending::Start { id, target } => {
                if let Some(&i) = self.index.get(&id) {
                    if !self.instances[i].running {
                        let seq = self.take_seq();
                        let inst = &mut self.instances[i];
                        inst.running = true;
                        inst.target = target;
                        inst.start_seq = seq;
                    }
                } else if let Some(script) = set.get(&id) {
                    let start_seq = self.take_seq();
                    self.index.insert(id.clone(), self.instances.len());
                    self.instances.push(GlobalInstance {
                        id,
                        running: true,
                        locals: Locals::new(script),
                        target,
                        start_seq,
                    });
                } else if set.is_inert(&id) {
                    self.reports.push(Report::InertScript { script: id });
                } else {
                    self.reports.push(Report::UnknownScript { script: id });
                }
            }
        }
    }

    fn apply_stop(&mut self, id: &str) {
        if let Some(&i) = self.index.get(id) {
            self.instances[i].running = false;
        }
    }

    /// `ScriptRunning` as a run sees it: the flags, then the run's own pending ops in order.
    pub(crate) fn running_with(&self, pending: &[Pending], id: &str) -> bool {
        let mut running = self.is_running(id);
        for op in pending {
            match op {
                Pending::Start { id: p, .. } if p == id => running = true,
                Pending::Stop { id: p } if p == id => running = false,
                _ => {}
            }
        }
        running
    }
}

/// A run's locals by name and type from its script (none without a live record).
pub(crate) fn save_locals(script: Option<&Script>, locals: &Locals) -> Vec<SavedLocal> {
    script.map_or_else(Vec::new, |s| {
        s.locals
            .iter()
            .zip(&locals.0)
            .map(|(l, &value)| SavedLocal {
                name: l.name.key.clone(),
                ty: l.ty.into(),
                value,
            })
            .collect()
    })
}

/// Saved locals remapped onto `script` (lower-cased id `id`) by name: missing ones start at 0,
/// extras are dropped, a changed type converts; each case is reported.
pub(crate) fn load_locals(
    id: &str,
    script: &Script,
    saved: &[SavedLocal],
    reports: &mut Vec<LoadReport>,
) -> Locals {
    let mut locals = Locals::new(script);
    for (slot, local) in script.locals.iter().enumerate() {
        let name = &local.name.key;
        match saved.iter().find(|s| key(&s.name) == *name) {
            Some(s) => {
                let to = LocalType::from(local.ty);
                if s.ty != to {
                    reports.push(LoadReport::Retyped {
                        script: id.to_owned(),
                        local: name.clone(),
                        from: s.ty,
                        to,
                    });
                }
                locals.0[slot] = store(local.ty, s.value);
            }
            None => reports.push(LoadReport::MissingLocal {
                script: id.to_owned(),
                local: name.clone(),
            }),
        }
    }
    for s in saved {
        if script.local(&key(&s.name)).is_none() {
            reports.push(LoadReport::DroppedLocal {
                script: id.to_owned(),
                local: key(&s.name),
            });
        }
    }
    locals
}

/// The host a run sees (global or local): script control goes to the global instance set (as
/// pending ops), everything else to the game's host.
pub(crate) struct Scoped<'a, H: ?Sized> {
    pub(crate) inner: &'a mut H,
    pub(crate) globals: &'a GlobalInstances,
    /// The implicit `StartScript` reference: a global's own target, a local's object.
    pub(crate) self_target: Option<String>,
    pub(crate) pending: &'a mut Vec<Pending>,
}

impl<H: Host + ?Sized> Host for Scoped<'_, H> {
    fn get(&mut self, var: &VarRef) -> f64 {
        self.inner.get(var)
    }
    fn set(&mut self, var: &VarRef, value: f64) {
        self.inner.set(var, value);
    }
    fn journal(&mut self, quest: &Ident, index: i32) {
        self.inner.journal(quest, index);
    }
    fn set_journal_index(&mut self, quest: &Ident, index: i32) {
        self.inner.set_journal_index(quest, index);
    }
    fn get_journal_index(&mut self, quest: &Ident) -> i32 {
        self.inner.get_journal_index(quest)
    }
    fn add_topic(&mut self, topic: &Ident) {
        self.inner.add_topic(topic);
    }
    fn choice(&mut self, choices: Vec<(String, i32)>) {
        self.inner.choice(choices);
    }
    fn goodbye(&mut self) {
        self.inner.goodbye();
    }
    fn add_item(&mut self, target: &Target, item: &Ident, count: i32) {
        self.inner.add_item(target, item, count);
    }
    fn remove_item(&mut self, target: &Target, item: &Ident, count: i32) {
        self.inner.remove_item(target, item, count);
    }
    fn get_item_count(&mut self, target: &Target, item: &Ident) -> i32 {
        self.inner.get_item_count(target, item)
    }
    fn set_disposition(&mut self, target: &Target, value: i32) {
        self.inner.set_disposition(target, value);
    }
    fn mod_disposition(&mut self, target: &Target, delta: i32) {
        self.inner.mod_disposition(target, delta);
    }
    fn message_box(&mut self, format: &str, args: &[MessageArg]) {
        self.inner.message_box(format, args);
    }
    fn play_sound(&mut self, sound: &Ident) {
        self.inner.play_sound(sound);
    }
    fn stat(&mut self, target: &Target, call: &StatCall) -> Option<f64> {
        self.inner.stat(target, call)
    }
    fn ai(&mut self, target: &Target, call: &AiCall) -> Option<f64> {
        self.inner.ai(target, call)
    }
    fn script(&mut self, target: &Target, op: ScriptOp, script: &Ident) -> Option<f64> {
        let id = script.key.clone();
        match op {
            ScriptOp::Start => {
                let target = match target {
                    Target::Implicit => self.self_target.clone(),
                    Target::Explicit(r) => Some(r.key.clone()),
                };
                self.pending.push(Pending::Start { id, target });
                Some(0.0)
            }
            ScriptOp::Stop => {
                self.pending.push(Pending::Stop { id });
                Some(0.0)
            }
            ScriptOp::Running => Some(f64::from(u8::from(
                self.globals.running_with(self.pending, &id),
            ))),
        }
    }
    fn unhandled(&mut self, function: &Function, call: &Call) -> f64 {
        self.inner.unhandled(function, call)
    }
}
