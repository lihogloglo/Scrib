//! Local (object) script instances (scripting-adapter.md "Instances, scheduling, state", DQ-5a):
//! one instance per placed reference or inventory item whose base record has a script, keyed by
//! a caller-supplied [`LocalKey`], with its locals, per-tick order, faults and save snapshot.
//! Bevy-free; `mw_rpg_bevy` (`dialogue/local_scripts.rs`) inserts and takes instances as
//! references enter and leave the script-active region and schedules [`LocalInstances::tick`]
//! before the globals (DQ-5b).
//!
//! Ported from `OpenMW` `apps/openmw/mwworld/localscripts.cpp`, `refdata.cpp`, `worldimp.cpp`
//! and `engine.cpp` (github.com/OpenMW/openmw, commit
//! `4e718b500ffee6657db90297d7b8a08a57d439bc`), which matches the spec on:
//!
//! - An instance is added when its cell becomes active (`LocalScripts::addCell`) and dropped when
//!   it is unloaded (`clearCell`), but the locals live in the reference's `RefData` and survive:
//!   re-adding keeps them (`Locals::configure` only zeroes uninitialised locals). Here
//!   [`LocalInstances::take`] hands the caller the locals by name to keep with the placement and
//!   [`LocalInstances::restore`] brings them back; [`LocalInstances::insert`] of a present key
//!   is a no-op (`add` twice keeps the locals).
//! - A script without a record is not added (`add`: "the script does not exist"); here an inert
//!   one isn't either, and both are reported.
//! - **Disabled references keep running their local scripts** (spec open question 4). `World::
//!   disable` only clears `RefData::mEnabled` and removes the object from the scene; it never
//!   touches `LocalScripts`, and `LocalScripts::getNext` skips only references whose separate
//!   `Flag_LocalScriptDisabled` is set (`RefData::isLocalScriptEnabled`, reachable only from Lua
//!   `pauseLocalScript`). That is how Morrowind scripts that `Disable` their object later
//!   `Enable` it again. So the reference's enabled state is not this module's business: the
//!   caller keeps a disabled reference inserted. [`LocalInstances::set_paused`] is the
//!   equivalent of `Flag_LocalScriptDisabled` (saved, as `RefData` saves it).
//! - A run's `StartScript` with no `ref->` gives the global the running object as its target
//!   (`miscextensions.cpp` `OpStartScript`: `R()(runtime, false)`), through
//!   [`InstanceHost::local_self`].
//!
//! Where it differs, this module follows the spec, for determinism (net-ready, semantics rule 1):
//! `OpenMW` runs locals in the order their cells were added; here they run in [`LocalKey`] order
//! (placements, then inventory stacks). `StartScript` / `StopScript` from a local run queue and
//! apply to the globals after the run, as for global runs. A step-limit fault pauses the instance
//! until the caller resumes it (`OpenMW` marks the script and object inactive for the session).
//! `OpenMW` seeds a new local instance's locals from a running global of the same script
//! (`Locals::configure`); here they start at zero, as the spec says.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::instances::{
    GlobalInstances, InstanceHost, LoadReport, SavedLocal, Scoped, ScriptSet, key, load_locals,
    save_locals,
};
use crate::interp::{Interpreter, Locals, RunError, store};
use sf_tunables::{Def, Lifetime, Net, Scope, Source, Unit, def};

/// `mw.script.active_radius`: the script-active region's radius around the player's exterior
/// cell, in Morrowind cells (1 = the 3x3 block; an interior is always just its own cell). Local
/// scripts of references inside it run; the region follows the player's cell, not render
/// streaming (scripting-adapter.md "Script-active region").
pub const ACTIVE_RADIUS: Def<u32> = def!(count "mw.script.active_radius" = 1)
    .range(1, 3)
    .unit(Unit::Count)
    .help("Morrowind cells around the player's exterior cell whose references run their local scripts")
    .source(Source::Reference("OpenMW setting [Cells] exterior cell load distance (default 1)"))
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Live);

/// What a local instance belongs to. The caller maps its own ids onto these numbers; they must
/// be stable across saves. Ordering is total: every placement, by id, then every inventory
/// stack, by holder then stack (the spec's tick order).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum LocalKey {
    /// A placed (or spawned) reference: its `PlacementId` (128 bits, `sf_world::PlacementId`).
    Placement(u128),
    /// An item in an inventory: the holder's id and the item's `StackId`.
    Stack { holder: u128, stack: u64 },
}

/// One local script instance.
#[derive(Clone, Debug, PartialEq)]
pub struct LocalInstance {
    /// The lower-cased script id.
    pub script: String,
    /// Skipped by [`LocalInstances::tick`] (`OpenMW` `Flag_LocalScriptDisabled`, never the
    /// reference's own `Disable`).
    pub paused: bool,
    /// Slot values of [`crate::Script::locals`], stored converted.
    pub locals: Locals,
}

/// Something the local set reports once, drained with [`LocalInstances::take_reports`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LocalReport {
    /// The run hit the step limit: its ops were dropped and the instance paused.
    StepLimit { key: LocalKey, script: String, line: u32 },
    /// An insert named a script with no record: nothing inserted.
    UnknownScript { key: LocalKey, script: String },
    /// An insert named an inert script: nothing inserted (inert scripts never run).
    InertScript { key: LocalKey, script: String },
}

/// A saved local instance, its locals by name and type.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SavedLocalInstance {
    pub key: LocalKey,
    pub script: String,
    pub paused: bool,
    pub locals: Vec<SavedLocal>,
}

/// The local part of the script save section, in key order.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct LocalsSnapshot {
    pub locals: Vec<SavedLocalInstance>,
}

/// The local instances, in key order.
#[derive(Clone, Debug, Default)]
pub struct LocalInstances {
    instances: BTreeMap<LocalKey, LocalInstance>,
    reports: Vec<LocalReport>,
}

impl LocalInstances {
    /// The object `key` entered the script-active region with script `script`: a new instance
    /// with zeroed locals, which runs from the next tick. Returns `false` when nothing changed:
    /// the key already runs that script (its locals are kept), or the script has no live record
    /// (reported). A key running another script gets the new one, zeroed.
    pub fn insert(&mut self, set: &ScriptSet, key: LocalKey, script: &str) -> bool {
        let id = crate::instances::key(script);
        if self.instances.get(&key).is_some_and(|i| i.script == id) {
            return false;
        }
        let Some(parsed) = set.get(&id) else {
            self.reports.push(if set.is_inert(&id) {
                LocalReport::InertScript { key, script: id }
            } else {
                LocalReport::UnknownScript { key, script: id }
            });
            return false;
        };
        let locals = Locals::new(parsed);
        self.instances.insert(
            key,
            LocalInstance {
                script: id,
                paused: false,
                locals,
            },
        );
        true
    }

    /// Drops `key`'s instance and its locals (the object is gone for good). Returns whether it
    /// was there.
    pub fn remove(&mut self, key: LocalKey) -> bool {
        self.instances.remove(&key).is_some()
    }

    /// Removes `key`'s instance and returns it by name, for the caller to keep with the
    /// placement while it is outside the script-active region ([`LocalInstances::restore`]).
    pub fn take(&mut self, set: &ScriptSet, key: LocalKey) -> Option<SavedLocalInstance> {
        self.instances
            .remove(&key)
            .map(|inst| saved(set, key, &inst))
    }

    /// Puts back a [`LocalInstances::take`]n (or saved) instance against the current script
    /// set, replacing any instance of its key: locals are remapped by name, a script without a
    /// live record is not restored; each case is reported.
    pub fn restore(&mut self, set: &ScriptSet, saved: &SavedLocalInstance) -> Vec<LoadReport> {
        let mut reports = Vec::new();
        let id = key(&saved.script);
        let Some(script) = set.get(&id) else {
            reports.push(LoadReport::UnknownScript { script: id });
            return reports;
        };
        let locals = load_locals(&id, script, &saved.locals, &mut reports);
        self.instances.insert(
            saved.key,
            LocalInstance {
                script: id,
                paused: saved.paused,
                locals,
            },
        );
        reports
    }

    /// Pauses or resumes `key`'s script (its locals are kept). Not for the reference's own
    /// `Disable`: disabled references keep running (module docs). Returns whether `key` exists.
    pub fn set_paused(&mut self, key: LocalKey, paused: bool) -> bool {
        self.instances
            .get_mut(&key)
            .map(|i| i.paused = paused)
            .is_some()
    }

    /// Present and not paused: the next tick runs it.
    pub fn is_running(&self, key: LocalKey) -> bool {
        self.instances.get(&key).is_some_and(|i| !i.paused)
    }

    pub fn get(&self, key: LocalKey) -> Option<&LocalInstance> {
        self.instances.get(&key)
    }

    /// `ref.var` reads: the local `name` of `key`'s instance (`None`: no instance or no such
    /// local).
    pub fn local(&self, set: &ScriptSet, key: LocalKey, name: &str) -> Option<f64> {
        let inst = self.instances.get(&key)?;
        let slot = set.get(&inst.script)?.local(&crate::instances::key(name))?;
        Some(inst.locals.0.get(slot).copied().unwrap_or(0.0))
    }

    /// `set ref.var` from dialogue and other scripts, stored converted to the local's type.
    /// Returns `false` when there is no instance or no such local.
    pub fn set_local(&mut self, set: &ScriptSet, key: LocalKey, name: &str, value: f64) -> bool {
        let Some(inst) = self.instances.get_mut(&key) else {
            return false;
        };
        let Some(script) = set.get(&inst.script) else {
            return false;
        };
        let Some(slot) = script.local(&crate::instances::key(name)) else {
            return false;
        };
        inst.locals.0.resize(script.locals.len(), 0.0);
        inst.locals.0[slot] = store(script.locals[slot].ty, value);
        true
    }

    /// Every instance, in key order.
    pub fn iter(&self) -> impl Iterator<Item = (LocalKey, &LocalInstance)> {
        self.instances.iter().map(|(&k, i)| (k, i))
    }

    pub fn len(&self) -> usize {
        self.instances.len()
    }

    pub fn is_empty(&self) -> bool {
        self.instances.is_empty()
    }

    /// The reports since the last call.
    pub fn take_reports(&mut self) -> Vec<LocalReport> {
        std::mem::take(&mut self.reports)
    }

    /// One tick, before the caller ticks `globals` (`OpenMW` `engine.cpp` runs locals first, DQ-5a):
    /// every instance not paused, in key order, one after the other. Each run's ops apply
    /// before the next run starts; its `StartScript` / `StopScript` go to `globals` (so a global
    /// started here runs in the same tick's global step; its reports land in `globals`).
    /// Instances the caller inserts after the tick run on the next.
    pub fn tick<H: InstanceHost + ?Sized>(
        &mut self,
        set: &ScriptSet,
        interp: &Interpreter,
        globals: &mut GlobalInstances,
        host: &mut H,
    ) {
        let mut pending = Vec::new();
        for (&key, inst) in &mut self.instances {
            if inst.paused {
                continue;
            }
            let Some(script) = set.get(&inst.script) else {
                continue;
            };
            inst.locals.0.resize(script.locals.len(), 0.0);
            let self_target = host.local_self(key);
            host.begin_local_run(&inst.script, key);
            let result = {
                let mut scoped = Scoped {
                    inner: &mut *host,
                    globals: &*globals,
                    self_target,
                    pending: &mut pending,
                };
                interp.run(script, &mut inst.locals, &mut scoped)
            };
            host.end_run(&inst.script, result.is_ok());
            match result {
                Ok(()) => {
                    for op in pending.drain(..) {
                        globals.apply(set, op);
                    }
                }
                Err(RunError::StepLimit { line }) => {
                    pending.clear();
                    inst.paused = true;
                    self.reports.push(LocalReport::StepLimit {
                        key,
                        script: inst.script.clone(),
                        line,
                    });
                }
            }
        }
    }

    /// The save section's local part: locals by name and type from the script set.
    pub fn snapshot(&self, set: &ScriptSet) -> LocalsSnapshot {
        LocalsSnapshot {
            locals: self
                .instances
                .iter()
                .map(|(&k, inst)| saved(set, k, inst))
                .collect(),
        }
    }

    /// Restores a [`LocalInstances::snapshot`] against the current script set
    /// ([`LocalInstances::restore`] per instance); each report comes with its key.
    pub fn load(snapshot: &LocalsSnapshot, set: &ScriptSet) -> (Self, Vec<(LocalKey, LoadReport)>) {
        let mut out = Self::default();
        let mut reports = Vec::new();
        for saved in &snapshot.locals {
            reports.extend(out.restore(set, saved).into_iter().map(|r| (saved.key, r)));
        }
        (out, reports)
    }
}

fn saved(set: &ScriptSet, key: LocalKey, inst: &LocalInstance) -> SavedLocalInstance {
    SavedLocalInstance {
        key,
        script: inst.script.clone(),
        paused: inst.paused,
        locals: save_locals(set.get(&inst.script), &inst.locals),
    }
}
