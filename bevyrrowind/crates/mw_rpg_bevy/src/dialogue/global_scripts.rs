//! Morrowind's global scripts in the game (scripting-adapter.md "The game API", DQ-4b2a): the
//! script records parsed once, the new game's start list started, and every running global
//! ticked on `FixedUpdate` through [`mw_rpg::ScriptHost`], its ops applied by [`Rpg::apply`]
//! (the applier dialogue results and the console share).
//!
//! - **Parse.** Every `SCPT` record, once, when the game is set up ([`GlobalScripts::new_game`]),
//!   against the game's globals ([`mw_rpg::WorldEnv`]): `StartScript` may name any of them, and
//!   the instance set borrows the script set for the whole tick.
//! - **Start list.** As `OpenMW`'s new game (`apps/openmw/mwscript/globalscripts.cpp`
//!   `GlobalScripts::addStartup`, 0.49.0): `main`, then every `SSCR` record's script in record
//!   order (load order, then file order), none with a target.
//! - **Tick.** Every fixed tick, as `OpenMW` runs the scripts every frame: one [`ScriptHost`] (one
//!   `GameRun`) over the host's view runs the local instances in key order, then the globals in
//!   start order (`OpenMW` `engine.cpp:283-286` @ 4e718b50 runs locals first, DQ-5a; a global a
//!   local starts runs in the same tick's global step), then the committed ops in call order,
//!   each through [`Rpg::apply`] without a conversation. The context player is the host; the
//!   speaker is unused. Which locals exist: [`super::local_scripts`] (DQ-5b1).
//! - **Reports.** Logged once each at warn level: the host's [`ScriptReports`] keep each once,
//!   the instance set's are deduplicated here.
//! - **Save** (DQ-4b2b, DQ-5c; `mw_saves_bevy`'s `mw/scripts` section, [`ScriptsSave`]):
//!   [`GlobalScripts::snapshot`] (every global instance, running or stopped, and every local
//!   one, live or parked, their locals by name and type); [`GlobalScripts::load`] replaces them
//!   wholesale against the live script set, its [`LoadReport`]s logged once at warn level and
//!   kept for the caller ([`GlobalScripts::load_reports`], [`GlobalScripts::local_load_reports`]).
//!   A save from before the section restarts the new game's start list ([`GlobalScripts::restart`]).
//! - **`scriptid.var`** (DQ-4b2c): dialogue results, the console and the globals themselves read
//!   and write a global script's locals by script id, through the [`ScriptLocals`] copy the
//!   instance set publishes into [`Rpg`] (precedence, timing: [`ScriptLocals`]).
//! - **`ref.var`** (DQ-5b2): the same for a placed object's locals, through the
//!   [`ObjectLocals`] copy next to it (which objects, rule 7, parked ones, timing: there). A
//!   fresh object instance (entering the region zeroed, or created parked by a write) starts with
//!   the skipped chargen's locals of its base, then the locals the NPC dialogue store held for its
//!   key (older saves, results before its first run), moved out of that store.

use std::collections::{HashMap, HashSet};

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_log::prelude::*;
use mw_rpg::{GameView, NoSheets, ScriptHost, ScriptReports, Target, WorldEnv};
use mw_script::ast::VarType;
use mw_script::{
    FunctionTable, GlobalInstances, GlobalsSnapshot, Interpreter, LoadReport, LocalInstances, LocalKey, LocalsSnapshot,
    ScriptSet,
};
use serde::{Deserialize, Serialize};
use sf_records::RecordId;
use sf_rpg::{ActorKey, ConversationCtx, PlayerId};
use tes3::esp::{Script, StartScript};

use super::local_scripts::{Declared, LocalScripts, ObjectLocals, PLAYER_HOLDER, RegionCell, ScriptRegion, track_script_region};
use super::{DialogueSheets, Rpg, View, id};

/// The script `OpenMW` starts first at every new game, before the `SSCR` list.
const MAIN_SCRIPT: &str = "main";

/// The speaker of a global tick's context: never a registered actor (the host never names it).
const NO_SPEAKER: ActorKey = ActorKey(u128::MAX - 2);

/// Global scripts' locals as everything outside the instance set sees them (`scriptid.var`):
/// [`Rpg`] holds it, so dialogue results, the console and the globals' own tick (through
/// [`View`]) read it and [`Rpg::apply`] writes it; [`GlobalScripts`] publishes the instances
/// into it and takes its writes.
///
/// - **Precedence.** An id that names a script record is that global script, whatever
///   reference shares the id; only another id is an actor's base record. As `OpenMW`'s compiler
///   (`apps/openmw/mwscript/compilercontext.cpp` `CompilerContext::getMemberType`, master
///   `4e718b50`: an `ESM::Script` found by the id makes the member access global) and
///   interpreter (`interpretercontext.cpp` `getMemberLocals`: global, then
///   `GlobalScripts::getLocals` by script id). A script id without the local is an unknown
///   variable (no fallback to an actor); an inert script declares none.
/// - **No instance yet.** A script never started reads its locals as 0; a write creates its
///   instance, stopped, as `GlobalScripts::getLocals` (`globalscripts.cpp`) does (`OpenMW`
///   creates it on reads too, which nothing in play can tell apart).
/// - **Timing.** A write lands here at once (later reads in the same frame see it) and in the
///   instance at the next [`GlobalScripts::tick`] start, or after that tick's ops for a write
///   the tick itself made. Within a tick, a global reading another's locals sees the tick-start
///   values plus the `set id.var` writes earlier runs of the tick made (the tick's one run
///   overlay); what the owner writes to its own locals in the tick shows from the next tick,
///   and a `set id.var` made during the tick overrides it (it applies after the runs, in op
///   order). Deterministic: start order, then op order.
#[derive(Default)]
pub(super) struct ScriptLocals {
    /// Every script record's locals by script id, then by name: slot and type (inert scripts:
    /// none). Fixed by the script set.
    declared: Declared,
    /// The instances' slot values by script id, as last published.
    values: HashMap<String, Vec<f64>>,
    /// Writes not yet in the instances, in call order: script id, slot, stored value.
    writes: Vec<(String, usize, f64)>,
    /// The local (object) instances' locals (`ref.var`, DQ-5b2).
    objects: ObjectLocals,
}

impl ScriptLocals {
    /// Does `id` name a script record (live or inert)?
    pub(super) fn is_script(&self, id: &RecordId) -> bool {
        self.declared.contains_key(id.as_str())
    }

    /// `script.name`'s type and value; `None`: the script declares no such local.
    #[allow(clippy::cast_possible_truncation)] // locals are stored converted (`f32` holds them)
    pub(super) fn get(&self, script: &RecordId, name: &RecordId) -> Option<(VarType, f32)> {
        let &(slot, ty) = self.declared.get(script.as_str())?.get(name.as_str())?;
        let value = self.values.get(script.as_str()).and_then(|v| v.get(slot)).copied().unwrap_or(0.0);
        Some((ty, value as f32))
    }

    /// Writes `script.name`, converted to its type; `false`: the script declares no such local.
    pub(super) fn set(&mut self, script: &RecordId, name: &RecordId, value: f32) -> bool {
        let Some(&(slot, ty)) = self.declared.get(script.as_str()).and_then(|d| d.get(name.as_str())) else {
            return false;
        };
        let value = mw_script::interp::store(ty, f64::from(value));
        let values = self.values.entry(script.as_str().to_owned()).or_default();
        if values.len() <= slot {
            values.resize(slot + 1, 0.0);
        }
        values[slot] = value;
        self.writes.push((script.as_str().to_owned(), slot, value));
        true
    }

    /// The object whose locals `target`'s `.var` names ([`ObjectLocals`]): the speaker, the
    /// exact reference, the base's reference by rule 7 ([`Self::resolve_base`]) or the exact
    /// inventory stack, when its locals are kept there. `None`: the NPC dialogue store's (the
    /// caller checks script ids first: they name globals).
    pub(super) fn object_of(&self, ctx: &ConversationCtx, target: &Target) -> Option<LocalKey> {
        let key = match target {
            Target::Speaker => LocalKey::Placement(ctx.speaker.0),
            Target::Ref(p) => LocalKey::Placement(*p),
            Target::Actor(base) => LocalKey::Placement(self.resolve_base(base)?),
            &Target::Stack { holder, stack } => LocalKey::Stack { holder, stack },
            Target::Player => return None,
        };
        Some(key).filter(|&k| self.objects.addresses(k))
    }

    /// Semantics rule 7, the one resolver of a base-id script target (`id.var`,
    /// `"id"->ModDisposition`, `"id"->AddItem`, ...): the placement it names
    /// ([`ObjectLocals::resolve`]); `None`: no placed reference has base `base`.
    pub(super) fn resolve_base(&self, base: &RecordId) -> Option<u128> {
        self.objects.resolve(base.as_str())
    }

    /// Are `placement`'s locals an object instance's ([`ObjectLocals`])?
    pub(super) fn addresses(&self, placement: u128) -> bool {
        self.objects.addresses(LocalKey::Placement(placement))
    }

    /// Object `key`'s local `name`: type and value; `None`: its script declares none.
    pub(super) fn object_local(&self, key: LocalKey, name: &RecordId) -> Option<(VarType, f32)> {
        self.objects.get(&self.declared, key, name.as_str())
    }

    /// Writes object `key`'s local `name`; `false`: its script declares none.
    pub(super) fn set_object_local(&mut self, key: LocalKey, name: &RecordId, value: f32) -> bool {
        self.objects.set(&self.declared, key, name.as_str(), value)
    }
}

/// The `mw/scripts` save section's value (v2, DQ-5c; v1 held [`Self::globals`] only): the
/// global instances, the local instances inside the script-active region, and the local
/// instances parked outside it, each by name and type, in start and key order.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ScriptsSave {
    pub globals: GlobalsSnapshot,
    /// The live local instances.
    pub locals: LocalsSnapshot,
    /// The local instances taken when their reference left the region (`local_scripts` module
    /// docs), kept as taken.
    pub parked: LocalsSnapshot,
}

/// The game's global scripts: the parsed records, the instances, the report-once logs.
#[derive(Resource)]
pub struct GlobalScripts {
    set: ScriptSet,
    instances: GlobalInstances,
    interp: Interpreter<'static>,
    reports: ScriptReports,
    /// Instance-set reports already logged.
    logged: HashSet<String>,
    /// The last [`Self::load`]'s reports.
    load_reports: Vec<LoadReport>,
    /// The last [`Self::load`]'s local-instance reports, by key.
    local_load_reports: Vec<(LocalKey, LoadReport)>,
    /// The local (object) instances and the scripted references (DQ-5b1).
    locals: LocalScripts,
    /// The script-active region's centre the next tick syncs the locals to.
    centre: Option<RegionCell>,
    /// The player's position (engine metres) the next publish gives `ref.var`'s rule 7.
    player: Option<[f64; 3]>,
}

impl GlobalScripts {
    /// Every script record parsed against `rpg`'s globals, and the new game's start list started
    /// (see the module docs).
    /// `rpg` takes the script set's locals ([`ScriptLocals`]).
    pub fn new_game(rpg: &mut Rpg) -> Self {
        let table = FunctionTable::vanilla();
        let mut set = ScriptSet::default();
        let mut declared = HashMap::new();
        {
            let view = View { c: &rpg.content, s: &rpg.state, g: &rpg.script_locals, sheets: &NoSheets };
            let env = WorldEnv(&view);
            for (key, script) in rpg.content.records.iter::<Script>() {
                set.parse_insert(key.as_str(), &script.text, table, &env);
                let locals = set.get(key.as_str()).map(|s| {
                    s.locals.iter().enumerate().map(|(slot, l)| (l.name.key.clone(), (slot, l.ty))).collect()
                });
                declared.insert(key.as_str().to_lowercase(), locals.unwrap_or_default());
            }
        }
        let locals = LocalScripts::new(&rpg.content.records);
        let objects = ObjectLocals::new(std::sync::Arc::clone(&locals.refs));
        rpg.script_locals = ScriptLocals { declared, objects, ..ScriptLocals::default() };
        let inert = set.inert().count();
        info!("global scripts: {} script records parsed, {inert} inert", set.len() + inert);
        let instances = start_list(&set, rpg);
        let mut scripts = Self {
            set,
            instances,
            interp: Interpreter::new(table),
            reports: ScriptReports::default(),
            logged: HashSet::new(),
            load_reports: Vec::new(),
            local_load_reports: Vec::new(),
            locals,
            centre: None,
            player: None,
        };
        scripts.publish(rpg, true);
        scripts.log_reports();
        scripts
    }

    /// The instances replaced by the new game's start list over `rpg`'s records (a save from
    /// before the script section); `scriptid.var` writes not yet taken are dropped.
    pub fn restart(&mut self, rpg: &mut Rpg) {
        self.instances = start_list(&self.set, rpg);
        self.locals.clear();
        self.load_reports.clear();
        self.local_load_reports.clear();
        rpg.script_locals.writes.clear();
        rpg.script_locals.objects.writes.clear();
        self.publish(rpg, true);
        self.log_reports();
    }

    /// The save section's value: every global instance, running or stopped, with its locals by
    /// name and type ([`GlobalInstances::snapshot`]), `rpg`'s `scriptid.var` writes not yet taken
    /// included; every local instance, live and parked ([`ScriptsSave`]).
    pub fn snapshot(&self, rpg: &Rpg) -> ScriptsSave {
        let writes = &rpg.script_locals.writes;
        let globals = if writes.is_empty() {
            self.instances.snapshot(&self.set)
        } else {
            let mut instances = self.instances.clone();
            for (script, slot, value) in writes {
                set_slot(&self.set, &mut instances, script, *slot, *value);
            }
            instances.snapshot(&self.set)
        };
        let object_writes = &rpg.script_locals.objects.writes;
        let (locals, parked) = if object_writes.is_empty() {
            self.locals.snapshot(&self.set)
        } else {
            let mut locals = self.locals.clone();
            for write in object_writes {
                let seeds = match write.key {
                    LocalKey::Placement(p) if !locals.has(write.key) => seeds(rpg, p),
                    _ => Vec::new(),
                };
                locals.write(&self.set, write, &seeds);
            }
            locals.snapshot(&self.set)
        };
        ScriptsSave { globals, locals, parked }
    }

    /// Replaces every instance, global and local (live and parked), with `save`'s, remapped onto
    /// the live script set ([`GlobalInstances::load`], [`LocalInstances::load`]): each report is
    /// logged once at warn level and kept in [`Self::load_reports`] and
    /// [`Self::local_load_reports`]. `scriptid.var` writes not yet taken are dropped. The next
    /// tick syncs the local instances to the region (`local_scripts` module docs).
    pub fn load(&mut self, save: &ScriptsSave, rpg: &mut Rpg) {
        let (instances, reports) = GlobalInstances::load(&save.globals, &self.set);
        self.instances = instances;
        for report in &reports {
            warn!("global scripts: load: {report:?}");
        }
        self.load_reports = reports;
        let local_reports = self.locals.load(&self.set, &save.locals, &save.parked);
        for (key, report) in &local_reports {
            warn!("local scripts: load: {key:?}: {report:?}");
        }
        self.local_load_reports = local_reports;
        rpg.script_locals.writes.clear();
        rpg.script_locals.objects.writes.clear();
        self.publish(rpg, true);
    }

    /// The last [`Self::load`]'s local-instance reports, by key (empty after a clean load or a
    /// restart).
    pub fn local_load_reports(&self) -> &[(LocalKey, LoadReport)] {
        &self.local_load_reports
    }

    /// A global script's local by script id (`scriptid.var`): its type and value, 0 while the
    /// script has no instance; `None`: no live script `id`, or it declares no `name`.
    pub fn local(&self, id: &str, name: &str) -> Option<(VarType, f64)> {
        let script = self.set.get(id)?;
        let slot = script.locals.iter().position(|l| l.name.key.eq_ignore_ascii_case(name))?;
        let value = self.instances.get(id).and_then(|g| g.locals.0.get(slot)).copied().unwrap_or(0.0);
        Some((script.locals[slot].ty, value))
    }

    /// The last [`Self::load`]'s reports (empty after a clean load or a restart).
    pub fn load_reports(&self) -> &[LoadReport] {
        &self.load_reports
    }

    /// The running and stopped instances (the save section and `scriptid.var` take them).
    pub fn instances(&self) -> &GlobalInstances {
        &self.instances
    }

    /// The instances to change directly; the next [`Self::tick`] publishes them to
    /// `scriptid.var` reads.
    pub fn instances_mut(&mut self) -> &mut GlobalInstances {
        &mut self.instances
    }

    /// The local instances (DQ-5b1): the scripted references inside the script-active region.
    pub fn locals(&self) -> &LocalInstances {
        &self.locals.instances
    }

    /// The player's position (engine metres; `None`: unknown), what the next [`Self::tick`]'s
    /// publish gives `ref.var`'s rule 7 ("nearest", [`ObjectLocals`]).
    pub fn set_player_position(&mut self, player: Option<[f64; 3]>) {
        self.player = player;
    }

    /// Object `placement`'s local `name` as its instance holds it, live or parked: its type and
    /// value; `None`: no instance, or its script declares no `name`.
    pub fn object_local(&self, placement: u128, name: &str) -> Option<(VarType, f64)> {
        self.locals.local(&self.set, LocalKey::Placement(placement), name)
    }

    /// The player's cell, the script-active region's centre the next [`Self::tick`] uses
    /// (`None`: keep the last region).
    pub fn set_region_centre(&mut self, centre: Option<RegionCell>) {
        if centre.is_some() {
            self.centre = centre;
        }
    }

    /// One tick over `rpg`: the locals synced to the script-active region, then every local
    /// instance in key order and every running global in start order, the ops applied in call
    /// order; `scriptid.var` writes are taken before the runs and after the ops, and the global
    /// instances published at the end ([`ScriptLocals`]).
    pub fn tick(&mut self, rpg: &mut Rpg, sheets: &dyn mw_rpg::MwSheets) {
        let ctx = ConversationCtx { player: PlayerId::HOST, speaker: NO_SPEAKER, speaker_base: id(""), choice: None };
        self.take_writes(rpg);
        for report in self.locals.sync(&self.set, self.centre.as_ref()) {
            warn!("local scripts: restore {report}");
        }
        let carried = rpg.state.pc.loadout.inventory().stacks().iter().map(|s| (s.id, &s.item));
        self.locals.sync_stacks(&self.set, PLAYER_HOLDER, carried);
        for key in self.locals.take_fresh() {
            let LocalKey::Placement(placement) = key else { continue };
            for (name, value) in take_seeds(rpg, placement) {
                self.locals.instances.set_local(&self.set, key, &name, value);
            }
        }
        let ops = {
            let view = View { c: &rpg.content, s: &rpg.state, g: &rpg.script_locals, sheets };
            let game = GameView { ctx: &ctx, world: &rpg.state.world, player: &rpg.state.pc, env: &view };
            let LocalScripts { refs, instances: locals, .. } = &mut self.locals;
            let carried = rpg.state.pc.loadout.inventory().stacks();
            let bases = |key: LocalKey| match key {
                LocalKey::Placement(placement) => refs.base(placement),
                LocalKey::Stack { holder, stack } => (holder == PLAYER_HOLDER)
                    .then(|| carried.iter().find(|s| u64::from(s.id.0) == stack))
                    .flatten()
                    .map(|s| s.item.as_str().to_owned()),
            };
            let mut host = ScriptHost::new(game, &mut self.reports).with_ref_bases(&bases);
            locals.tick(&self.set, &self.interp, &mut self.instances, &mut host);
            self.instances.tick(&self.set, &self.interp, &mut host);
            host.into_ops()
        };
        for op in ops {
            // Journal and topic events reach the journal screen on its next change check.
            let _ = rpg.apply(None, &ctx, op, sheets);
        }
        self.take_writes(rpg);
        self.publish(rpg, false);
        self.log_reports();
    }

    /// `rpg`'s `scriptid.var` and `ref.var` writes into the instances, in call order (each kind).
    fn take_writes(&mut self, rpg: &mut Rpg) {
        for (script, slot, value) in std::mem::take(&mut rpg.script_locals.writes) {
            set_slot(&self.set, &mut self.instances, &script, slot, value);
        }
        for write in std::mem::take(&mut rpg.script_locals.objects.writes) {
            let seeds = match write.key {
                LocalKey::Placement(p) if !self.locals.has(write.key) => take_seeds(rpg, p),
                _ => Vec::new(),
            };
            self.locals.write(&self.set, &write, &seeds);
        }
    }

    /// Every instance's locals into `rpg`'s [`ScriptLocals`] (stale globals dropped; `full`:
    /// the object copy rebuilt, [`LocalScripts::publish`]).
    fn publish(&mut self, rpg: &mut Rpg, full: bool) {
        self.locals.publish(&self.set, &mut rpg.script_locals.objects, full, self.player);
        let values = &mut rpg.script_locals.values;
        values.retain(|id, _| self.instances.get(id).is_some());
        for instance in self.instances.iter() {
            match values.get_mut(&instance.id) {
                Some(v) => v.clone_from(&instance.locals.0),
                None => {
                    values.insert(instance.id.clone(), instance.locals.0.clone());
                }
            }
        }
    }

    fn log_reports(&mut self) {
        for report in self.reports.drain() {
            warn!("{report}");
        }
        for report in self.instances.take_reports() {
            let line = format!("global scripts: {report:?}");
            if self.logged.insert(line.clone()) {
                warn!("{line}");
            }
        }
        for report in self.locals.instances.take_reports() {
            let line = format!("local scripts: {report:?}");
            if self.logged.insert(line.clone()) {
                warn!("{line}");
            }
        }
    }
}

/// What a fresh object instance of `placement` starts with (module docs): the skipped chargen's
/// locals of its base, then the NPC dialogue store's for its key (later wins), by name.
fn seeds(rpg: &Rpg, placement: u128) -> Vec<(String, f64)> {
    let key = ActorKey(placement);
    let base = rpg.script_locals.objects.base(placement).map(id);
    let chargen = base.as_ref().and_then(|b| rpg.content.chargen_locals.get(b)).into_iter().flatten();
    let store = rpg.state.locals.iter().filter(|((k, _), _)| *k == key).map(|((_, n), v)| (n, v));
    chargen.map(|(n, v)| (n, v)).chain(store).map(|(n, v)| (n.as_str().to_owned(), f64::from(*v))).collect()
}

/// [`seeds`], the dialogue store's entries for `placement` moved out (one store).
fn take_seeds(rpg: &mut Rpg, placement: u128) -> Vec<(String, f64)> {
    let out = seeds(rpg, placement);
    let key = ActorKey(placement);
    rpg.state.locals.retain(|(k, _), _| *k != key);
    out
}

/// The new game's start list (see the module docs): `main` only when the content has it (a
/// content set without it is no error); a start record naming no script is reported.
fn start_list(set: &ScriptSet, rpg: &Rpg) -> GlobalInstances {
    let mut instances = GlobalInstances::default();
    let main = (set.get(MAIN_SCRIPT).is_some() || set.is_inert(MAIN_SCRIPT)).then_some(MAIN_SCRIPT);
    for script in main.into_iter().chain(rpg.content.records.iter::<StartScript>().map(|(_, s)| s.script.as_str())) {
        instances.start(set, script, None);
    }
    instances
}

/// One local of global `id` by slot (`value` already converted); the instance is created,
/// stopped, when the script has none (see [`ScriptLocals`]).
fn set_slot(set: &ScriptSet, instances: &mut GlobalInstances, id: &str, slot: usize, value: f64) {
    let Some(len) = set.get(id).map(|s| s.locals.len()).filter(|&len| slot < len) else { return };
    if instances.get(id).is_none() {
        instances.start(set, id, None);
        instances.stop(id);
    }
    if let Some(instance) = instances.get_mut(id) {
        instance.locals.0.resize(len, 0.0);
        instance.locals.0[slot] = value;
    }
}

/// Starts the new game's globals over `rpg` and ticks them, with the local scripts of the
/// player's region, on `FixedUpdate`.
pub fn add(app: &mut App, rpg: &mut Rpg) {
    app.insert_resource(GlobalScripts::new_game(rpg))
        .init_resource::<ScriptRegion>()
        .add_systems(FixedUpdate, (track_script_region, super::sync_ai_runs, tick_global_scripts).chain());
}

fn tick_global_scripts(
    mut rpg: ResMut<Rpg>,
    mut scripts: ResMut<GlobalScripts>,
    region: Res<ScriptRegion>,
    sheets: DialogueSheets,
) {
    scripts.set_region_centre(region.centre.clone());
    scripts.set_player_position(region.player);
    scripts.tick(&mut rpg, &sheets);
}

#[cfg(test)]
mod tests {
    //! Headless, over fixture records: a start-list global journals once; its locals read and
    //! written from outside (`scriptid.var`, DQ-4b2c).
    use std::sync::Arc;
    use std::time::Duration;

    use bevy_time::{Fixed, Time, TimePlugin, TimeUpdateStrategy};
    use sf_records::RecordDb;
    use tes3::esp::{Dialogue, DialogueInfo, DialogueType2, GlobalValue, GlobalVariable};

    use super::*;

    /// `dq_start` (journals `dq_q` 10 once, guarded by its local `done`), on the start list;
    /// the `dq_q` journal with its stage 10; the short global `dq_seen`.
    fn records() -> RecordDb {
        fixture(false)
    }

    /// [`records`], and with `watch` the global `dq_watch` started after `dq_start`, copying
    /// `dq_start.done` into `dq_seen` every tick.
    fn fixture(watch: bool) -> RecordDb {
        let mut db = RecordDb::new();
        mw_records::register_kinds(&mut db).unwrap();
        let src = db.add_source("test", &[]);
        let seen = GlobalVariable { id: "dq_seen".into(), value: GlobalValue::Short(0), ..Default::default() };
        db.define(src, id("dq_seen"), seen).unwrap();
        if watch {
            let text = "Begin dq_watch\nset dq_seen to dq_start.done\nEnd\n";
            let script = Script { id: "dq_watch".into(), text: text.into(), ..Default::default() };
            db.define(src, id("dq_watch"), script).unwrap();
            let start = StartScript { id: "1".into(), script: "dq_watch".into(), ..Default::default() };
            db.define(src, id("1"), start).unwrap();
        }
        let text = "Begin dq_start\nshort done\nif ( done == 0 )\n\tJournal \"dq_q\" 10\n\tset done to 1\nendif\nEnd\n";
        let script = Script { id: "dq_start".into(), text: text.into(), ..Default::default() };
        db.define(src, id("dq_start"), script).unwrap();
        let start = StartScript { id: "0".into(), script: "dq_start".into(), ..Default::default() };
        db.define(src, id("0"), start).unwrap();
        let dial = Dialogue { id: "dq_q".into(), dialogue_type: DialogueType2::Journal, ..Default::default() };
        let mut stage = DialogueInfo { id: "dq_q10".into(), text: "It began.".into(), ..Default::default() };
        stage.data.disposition = 10;
        db.define(src, mw_records::keys::info(&dial, &stage), stage).unwrap();
        db.define(src, id("dq_q"), dial).unwrap();
        db
    }

    #[test]
    fn a_start_list_global_journals_once_on_the_fixed_tick() {
        let mut rpg = Rpg::new(Arc::new(records())).unwrap();
        let mut app = App::new();
        app.add_plugins(TimePlugin)
            .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(1.0 / 60.0)))
            .insert_resource(Time::<Fixed>::from_hz(60.0));
        add(&mut app, &mut rpg);
        app.insert_resource(rpg);
        let quest = id("dq_q");
        assert_eq!(app.world().resource::<Rpg>().state.world.quests.stage(&quest), 0, "nothing before a tick");
        for _ in 0..6 {
            app.update();
        }
        let rpg = app.world().resource::<Rpg>();
        assert_eq!(rpg.state.world.quests.stage(&quest), 10);
        let stages: Vec<_> = rpg.state.world.journal.for_quest(&quest).map(|e| e.stage).collect();
        assert_eq!(stages, [Some(10)]);
        let scripts = app.world().resource::<GlobalScripts>();
        let instance = scripts.instances().get("dq_start").expect("started at the new game");
        assert!(instance.running);
        assert_eq!(instance.locals.0, [1.0], "the guard ran once");
        assert!(scripts.instances().get(MAIN_SCRIPT).is_none(), "no main record: nothing started");
    }

    /// A new game over `db`: its `Rpg` and globals, nothing run.
    fn new_game(db: RecordDb) -> (Rpg, GlobalScripts) {
        let mut rpg = Rpg::new(Arc::new(db)).unwrap();
        let scripts = GlobalScripts::new_game(&mut rpg);
        (rpg, scripts)
    }

    /// One console line, its notes (what it did left out).
    fn console(rpg: &mut Rpg, line: &str) -> Vec<String> {
        let out = rpg.run_console(line, None, &NoSheets).unwrap();
        out.into_iter().filter(|l| l.starts_with("note:")).collect()
    }

    /// `dq_seen`'s value.
    fn seen(rpg: &Rpg) -> Option<f32> {
        rpg.state.globals.get(&id("dq_seen")).copied()
    }

    #[test]
    fn a_console_line_reads_a_global_scripts_local_by_script_id() {
        let (mut rpg, mut scripts) = new_game(records());
        assert!(console(&mut rpg, "set dq_seen to dq_start.done").is_empty(), "known before any run");
        assert_eq!(seen(&rpg), Some(0.0));
        scripts.tick(&mut rpg, &NoSheets);
        assert!(console(&mut rpg, "set dq_seen to dq_start.done").is_empty());
        assert_eq!(seen(&rpg), Some(1.0), "the guard ran");
        assert_eq!(scripts.local("DQ_Start", "Done"), Some((VarType::Short, 1.0)));
        assert_eq!(scripts.local("dq_start", "nope"), None);
    }

    #[test]
    fn a_console_write_to_a_global_scripts_local_reruns_its_guarded_journal() {
        let (mut rpg, mut scripts) = new_game(records());
        let quest = id("dq_q");
        scripts.tick(&mut rpg, &NoSheets);
        assert_eq!(rpg.state.world.quests.stage(&quest), 10);
        scripts.tick(&mut rpg, &NoSheets);
        console(&mut rpg, "SetJournalIndex \"dq_q\" 0");
        assert_eq!(rpg.state.world.quests.stage(&quest), 0, "the guard keeps it at 0");

        assert!(console(&mut rpg, "set dq_start.done to 0").is_empty());
        console(&mut rpg, "set dq_seen to dq_start.done");
        assert_eq!(seen(&rpg), Some(0.0), "the write is seen at once");
        assert_eq!(scripts.local("dq_start", "done"), Some((VarType::Short, 1.0)), "the instance takes it at the tick");
        let saved = scripts.snapshot(&rpg);
        let saved: Vec<_> = saved.globals.globals[0].locals.iter().map(|l| l.value).collect();
        assert_eq!(saved, [0.0], "a save before the tick keeps the write");

        scripts.tick(&mut rpg, &NoSheets);
        assert_eq!(rpg.state.world.quests.stage(&quest), 10, "Journal ran again");
        assert_eq!(scripts.local("dq_start", "done"), Some((VarType::Short, 1.0)), "and the guard closed again");
        let stages: Vec<_> = rpg.state.world.journal.for_quest(&quest).map(|e| e.stage).collect();
        assert_eq!(stages, [Some(10)], "the stage's entry is not repeated");
    }

    #[test]
    fn an_unknown_script_or_local_reads_as_an_unknown_variable() {
        let (mut rpg, mut scripts) = new_game(records());
        scripts.tick(&mut rpg, &NoSheets);
        let notes = console(&mut rpg, "set dq_seen to dq_nope.done");
        assert_eq!(notes, ["note: unknown variable dq_nope.done"]);
        let notes = console(&mut rpg, "set dq_seen to dq_start.nope");
        assert_eq!(notes, ["note: unknown variable dq_start.nope"]);
        assert_eq!(seen(&rpg), Some(0.0), "unknown reads are 0");
        let notes = console(&mut rpg, "set dq_start.nope to 3");
        assert_eq!(notes, ["note: unknown variable dq_start.nope"], "writes too");
        assert_eq!(scripts.local("dq_start", "done"), Some((VarType::Short, 1.0)));
    }

    #[test]
    fn a_global_reads_anothers_locals_as_they_were_at_the_tick_start() {
        let (mut rpg, mut scripts) = new_game(fixture(true));
        scripts.tick(&mut rpg, &NoSheets);
        assert_eq!(seen(&rpg), Some(0.0), "dq_start set done = 1 earlier in the same tick: not seen yet");
        scripts.tick(&mut rpg, &NoSheets);
        assert_eq!(seen(&rpg), Some(1.0), "seen from the next tick");
    }

    /// `dq_go` (on the start list) sends `dq_npc` to a point once, then copies its AI reads into
    /// `dq_seen` (`GetAIPackageDone`) and `dq_pkg` (`GetCurrentAIPackage`) every tick; `dq_npc` a
    /// registered NPC.
    fn ai_fixture() -> RecordDb {
        let mut db = RecordDb::new();
        mw_records::register_kinds(&mut db).unwrap();
        let src = db.add_source("test", &[]);
        for name in ["dq_seen", "dq_pkg"] {
            let g = GlobalVariable { id: name.into(), value: GlobalValue::Short(0), ..Default::default() };
            db.define(src, id(name), g).unwrap();
        }
        let npc = tes3::esp::Npc { id: "dq_npc".into(), name: "Dee".into(), ..Default::default() };
        db.define(src, id("dq_npc"), npc).unwrap();
        let text = concat!(
            "Begin dq_go\nshort sent\nif ( sent == 0 )\n\t\"dq_npc\"->AITravel 100 200 0\n\tset sent to 1\nendif\n",
            "set dq_seen to \"dq_npc\"->GetAIPackageDone\nset dq_pkg to \"dq_npc\"->GetCurrentAIPackage\nEnd\n",
        );
        let script = Script { id: "dq_go".into(), text: text.into(), ..Default::default() };
        db.define(src, id("dq_go"), script).unwrap();
        let start = StartScript { id: "0".into(), script: "dq_go".into(), ..Default::default() };
        db.define(src, id("0"), start).unwrap();
        db
    }

    /// `MA2c2`, the host side: a global's `"npc"->AITravel` reaches the walkers as one
    /// [`ScriptAiPush`] for the NPC's key, carrying the travel package, and its AI reads answer
    /// from the walkers' [`AiRuns`] (copied each fixed tick before the scripts run).
    #[test]
    fn a_global_scripts_ai_push_reaches_the_walkers_and_its_reads_follow_their_runs() {
        use mw_walkers_bevy::script_ai::{AiPackageKind, AiRun, AiRuns, ScriptAiPush};
        use tes3::esp::{AiPackage, AiTravelPackage};
        const NPC: ActorKey = ActorKey(5);
        #[derive(Resource, Default)]
        struct Sent(Vec<ScriptAiPush>);
        fn collect(mut pushes: MessageReader<ScriptAiPush>, mut sent: ResMut<Sent>) {
            sent.0.extend(pushes.read().cloned());
        }
        let mut rpg = Rpg::new(Arc::new(ai_fixture())).unwrap();
        assert!(rpg.add_actor(NPC, &id("dq_npc"), "Balmora"));
        let mut app = App::new();
        app.add_plugins(TimePlugin)
            .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(1.0 / 60.0)))
            .insert_resource(Time::<Fixed>::from_hz(60.0))
            .init_resource::<AiRuns>()
            .add_message::<ScriptAiPush>()
            .init_resource::<Sent>()
            .add_systems(Update, (super::super::send_ai_pushes, collect).chain());
        add(&mut app, &mut rpg);
        app.insert_resource(rpg);
        let global = |app: &App, name: &str| app.world().resource::<Rpg>().state.globals.get(&id(name)).copied();
        for _ in 0..3 {
            app.update();
        }
        let pushes = &app.world().resource::<Sent>().0;
        let travel = AiPackage::Travel(AiTravelPackage { location: [100.0, 200.0, 0.0], reset: 0 });
        assert_eq!(*pushes, [ScriptAiPush { actor: NPC, package: travel }], "one push, once");
        assert_eq!((global(&app, "dq_seen"), global(&app, "dq_pkg")), (Some(0.0), Some(-1.0)), "no AI ran yet");

        let walking = AiRun { done: false, last: Some(AiPackageKind::Travel) };
        app.world_mut().resource_mut::<AiRuns>().0.insert(NPC, walking);
        app.update();
        assert_eq!((global(&app, "dq_seen"), global(&app, "dq_pkg")), (Some(0.0), Some(1.0)), "walking: Travel");
        let arrived = AiRun { done: true, last: None };
        app.world_mut().resource_mut::<AiRuns>().0.insert(NPC, arrived);
        app.update();
        assert_eq!((global(&app, "dq_seen"), global(&app, "dq_pkg")), (Some(1.0), Some(-1.0)), "arrived: done");
    }
}
