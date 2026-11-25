//! Morrowind's local (object) scripts in the game (scripting-adapter.md "Instances, scheduling,
//! state", DQ-5b1): every placed reference whose base record has a script gets an instance
//! while it is inside the script-active region, ticked by [`GlobalScripts`] before the globals.
//!
//! - **Which references.** [`ScriptRefs`], built once at new game from the `CELL` records: each
//!   reference (deleted ones skipped) whose base record (any kind with a script field) names a
//!   script, keyed by its `PlacementId` (`mw_world::ids::placement_id`, the world source's id)
//!   and grouped by its cell: an exterior reference by its cell's grid (or its `moved_cell`
//!   destination, as the world source delivers it), an interior one by the cell name. The same
//!   pass indexes, by base, the unscripted references a script can name by base id (NPCs,
//!   creatures, containers: rule 7's candidates, [`ObjectLocals::resolve`]); they get no instance.
//! - **The region** (semantics rule 2): the exterior cells within `mw.script.active_radius`
//!   ([`mw_script::ACTIVE_RADIUS`], the tunable's default until the store binds it) of the
//!   player's cell, or the player's interior cell, computed from the player's cell
//!   ([`ScriptRegion`], written by `track_script_region` from the player body's location), never
//!   from render streaming. No player yet (headless, free camera): the region stays where it was.
//! - **Enter and leave.** [`LocalScripts::sync`] diffs the region's references against the
//!   instances only when the centre or radius changed: a reference that leaves is taken
//!   ([`LocalInstances::take`], locals by name) and parked here; one that comes back is restored
//!   with those locals, a new one starts zeroed. Cost: one pass over the region's cells
//!   (`(2r + 1)^2` lookups) per cell change of the player, nothing on other ticks.
//! - **Disabled references keep running** (`OpenMW` `World::disable` never touches
//!   `LocalScripts`): membership comes from the records and the region, never from the
//!   reference's enabled state or its entity.
//! - **Self.** The run's self is the exact reference ([`mw_rpg::Target::Ref`], through
//!   [`mw_rpg::ScriptHost`]); the game resolves it to the placement's key, never by base id.
//!
//! - **Save** (DQ-5c; `mw/scripts` v2): the live instances and the parked ones, each by name
//!   and type ([`LocalScripts::snapshot`]); a load puts both back against the live script set
//!   ([`LocalScripts::load`], every report at load time) and the next sync diffs them against the
//!   region like any cell change: a restored live instance now outside is parked with its
//!   locals, a parked one inside resumes with them.
//!
//! - **`ref.var`** (DQ-5b2): dialogue results, the console and the scripts read and write a
//!   reference's locals through the [`ObjectLocals`] copy (which objects, rule 7, parked ones,
//!   timing: [`ObjectLocals`]). A fresh instance starts with the values the NPC dialogue store or
//!   the skipped chargen left for its reference ([`super::GlobalScripts`]).
//!
//! - **Carried items** (DQ-5b3): every stack of the player's inventory whose item (any carryable
//!   kind, [`ScriptRefs`]'s item index) has a script gets an instance, key
//!   [`LocalKey::Stack`] `{ holder: PLAYER_HOLDER, stack: StackId }` ([`PLAYER_HOLDER`] = the host
//!   player's persistent `PlayerKey`, `HOST_KEY`; an NPC or container holder will be its
//!   `ActorKey`). The key is its own enum variant, so it never collides with a `PlacementId`, and
//!   `LocalKey`'s derived order puts every placement before every stack: the spec's tick order
//!   (placed references first, then inventory stacks by holder, then stack id). Carried items run
//!   wherever the player is (no region). [`LocalScripts::sync_stacks`] diffs the inventory's
//!   scripted stacks against the last sync every tick (one item-index lookup per stack; work only
//!   on a change): a new stack starts zeroed, a stack that left the inventory (moved to a
//!   container, sold, removed) loses its instance and locals (handing them to the world reference
//!   it becomes is DQ-6). Its self is the stack ([`mw_rpg::Target::Stack`]): `ref.var` reaches its
//!   locals; item, sheet and disposition ops on it do nothing; a global it starts without `ref->`
//!   takes the item's id. As `OpenMW` `ContainerStore::add` / `remove`
//!   (`apps/openmw/mwworld/containerstore.cpp` @ 4e718b50: an item with a script entering the
//!   player's store is `LocalScripts::add`ed, removed with it; `localscripts.cpp` `addCell` adds
//!   the items of active containers, later slices), made a deterministic per-tick diff. Saved in
//!   the live locals like any instance; the inventory's `StackId`s are stable across saves.
//!
//! Not yet: NPC and container inventories; spawned references.
//!
//! [`GlobalScripts`]: super::GlobalScripts

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::sync::Arc;

use bevy_ecs::prelude::*;
use mw_game_core::player::PlayerMode;
use mw_script::ast::VarType;
use mw_script::{LoadReport, LocalInstances, LocalKey, LocalsSnapshot, SavedLocalInstance, ScriptSet};
use mw_world::convert::{CELL_UNITS, UNIT_METERS_F64};
use sf_records::{RecordDb, RecordId};
use sf_rpg::StackId;
use sf_world_bevy::WorldPosOf;
use tes3::esp::{
    Activator, Alchemy, Apparatus, Armor, Book, Cell, Clothing, Container, Creature, Door, Ingredient, Light, Lockpick,
    MiscItem, Npc, Probe, RepairItem, Weapon,
};

/// One cell of the script-active region's grid: an exterior grid square, or an interior by its
/// lower-cased cell name (its worldspace id).
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RegionCell {
    Exterior(i32, i32),
    Interior(String),
}

impl RegionCell {
    /// The interior cell `name` (any case).
    pub fn interior(name: &str) -> Self {
        Self::Interior(RecordId::new(name).as_str().to_owned())
    }

    /// The exterior cell holding the engine position `(x, z)` (metres; `mw_world::convert`
    /// axes: Morrowind x = engine x, Morrowind y = -engine z).
    #[allow(clippy::cast_possible_truncation)] // cell indices are far inside i32
    pub fn exterior_at(x: f64, z: f64) -> Self {
        let cell = |units: f64| (units / CELL_UNITS).floor() as i32;
        Self::Exterior(cell(x / UNIT_METERS_F64), cell(-z / UNIT_METERS_F64))
    }

    /// Is this cell inside the region of `radius` cells around `centre` (rule 2: the centre's
    /// interior only, or the exterior square)?
    fn within(&self, centre: &RegionCell, radius: u32) -> bool {
        match (self, centre) {
            (Self::Interior(a), Self::Interior(b)) => a == b,
            (&Self::Exterior(x, y), &Self::Exterior(cx, cy)) => {
                x.abs_diff(cx) <= radius && y.abs_diff(cy) <= radius
            }
            _ => false,
        }
    }
}

/// The player's cell, the centre of the script-active region (`None`: not known yet). Written by
/// `track_script_region` each fixed tick from the player body; [`GlobalScripts`] reads it.
///
/// [`GlobalScripts`]: super::GlobalScripts
#[derive(Resource, Clone, Debug, Default, PartialEq)]
pub struct ScriptRegion {
    pub centre: Option<RegionCell>,
    /// The player body's position (engine metres, in the centre's worldspace): rule 7's
    /// "nearest" for `id.var` ([`ObjectLocals`]).
    pub player: Option<[f64; 3]>,
}

/// A placed reference a script can name: its base record id and script id (both lower-cased;
/// no script: an unscripted actor or container), its cell, and where it is placed (engine
/// metres, from the `CELL` record).
#[derive(Clone, Debug)]
struct ScriptRef {
    base: String,
    script: Option<String>,
    cell: RegionCell,
    pos: [f64; 3],
}

/// Every placed reference whose base has a script, by cell, and every reference a script can
/// target by base id (the scripted ones, NPCs, creatures, containers), by base (see the module
/// docs).
#[derive(Default)]
pub(super) struct ScriptRefs {
    /// Scripted placements by cell.
    cells: HashMap<RegionCell, Vec<u128>>,
    refs: HashMap<u128, ScriptRef>,
    /// Placements by base id, ascending (rule 7's tie-break order).
    by_base: HashMap<String, Vec<u128>>,
    /// Every carryable item record with a script: item id -> script id (both lower-cased).
    items: HashMap<String, String>,
}

/// The holder id of the host player's inventory in [`LocalKey::Stack`] (module docs).
pub const PLAYER_HOLDER: u128 = mw_game_core::player::HOST_KEY.0;

/// The script id of base record `id` (lower-cased; `None`: no such base, or no script).
fn base_script(db: &RecordDb, id: &RecordId) -> Option<String> {
    macro_rules! scripted {
        ($($ty:ty),*) => {
            $(if let Some(record) = db.get::<$ty>(id) {
                let script = record.script.trim();
                return (!script.is_empty()).then(|| script.to_ascii_lowercase());
            })*
        };
    }
    scripted!(
        Activator, Container, Door, Light, Npc, Creature, MiscItem, Weapon, Armor, Clothing, Book, Alchemy, Apparatus,
        Ingredient, Lockpick, Probe, RepairItem
    );
    None
}

/// Can a script name base `id`'s references by base id without a script on it (rule 7 targets:
/// `"id"->ModDisposition`, `"id"->AddItem`)? NPCs, creatures, containers.
fn base_targetable(db: &RecordDb, id: &RecordId) -> bool {
    db.get::<Npc>(id).is_some() || db.get::<Creature>(id).is_some() || db.get::<Container>(id).is_some()
}

impl ScriptRefs {
    /// One pass over every cell's references (bases resolved once each).
    pub(super) fn build(db: &RecordDb) -> Self {
        let files: Vec<String> = db.sources().iter().map(|s| s.name.to_ascii_lowercase()).collect();
        // Per base: its script, and whether it is indexed at all.
        let mut bases: HashMap<RecordId, (Option<String>, bool)> = HashMap::new();
        let mut out = Self::default();
        for (_, cell) in db.iter::<Cell>() {
            for (&(source, refr_index), reference) in &cell.references {
                if reference.deleted() {
                    continue;
                }
                let base = RecordId::new(&reference.id);
                let (script, indexed) = bases.entry(base.clone()).or_insert_with(|| {
                    let script = base_script(db, &base);
                    let indexed = script.is_some() || base_targetable(db, &base);
                    (script, indexed)
                });
                let (true, Some(file)) = (*indexed, files.get(source as usize)) else { continue };
                let region = if cell.is_interior() {
                    RegionCell::interior(&cell.name)
                } else {
                    let (x, y) = reference.moved_cell.unwrap_or(cell.data.grid);
                    RegionCell::Exterior(x, y)
                };
                let placement = mw_world::ids::placement_id(file, refr_index).0;
                if script.is_some() {
                    out.cells.entry(region.clone()).or_default().push(placement);
                }
                out.by_base.entry(base.as_str().to_owned()).or_default().push(placement);
                let pos = mw_world::convert::engine_f64(reference.translation).to_array();
                let r = ScriptRef { base: base.as_str().to_owned(), script: script.clone(), cell: region, pos };
                out.refs.insert(placement, r);
            }
        }
        for placements in out.by_base.values_mut() {
            placements.sort_unstable();
            placements.dedup();
        }
        macro_rules! items {
            ($($ty:ty),*) => {
                $(for (key, record) in db.iter::<$ty>() {
                    let script = record.script.trim();
                    if !script.is_empty() {
                        out.items.insert(key.as_str().to_ascii_lowercase(), script.to_ascii_lowercase());
                    }
                })*
            };
        }
        items!(MiscItem, Weapon, Armor, Clothing, Book, Alchemy, Apparatus, Ingredient, Lockpick, Probe, RepairItem, Light);
        out
    }

    /// The script of carryable item `item` (lower-cased).
    fn item_script(&self, item: &str) -> Option<&str> {
        self.items.get(item).map(String::as_str)
    }

    /// The base record id of indexed reference `placement`.
    pub(super) fn base(&self, placement: u128) -> Option<String> {
        self.refs.get(&placement).map(|r| r.base.clone())
    }

    /// The script id of scripted reference `placement`.
    fn script(&self, placement: u128) -> Option<&str> {
        self.refs.get(&placement).and_then(|r| r.script.as_deref())
    }

    /// The indexed placements of base `base` (lower-cased), ascending.
    fn placements(&self, base: &str) -> &[u128] {
        self.by_base.get(base).map_or(&[], Vec::as_slice)
    }

    /// The scripted references of the region around `centre`, by placement id.
    fn region(&self, centre: &RegionCell, radius: u32) -> BTreeMap<u128, &str> {
        let mut out = BTreeMap::new();
        let mut add = |cell: &RegionCell| {
            for p in self.cells.get(cell).into_iter().flatten() {
                if let Some(script) = self.script(*p) {
                    out.insert(*p, script);
                }
            }
        };
        match centre {
            RegionCell::Interior(_) => add(centre),
            &RegionCell::Exterior(cx, cy) => {
                let r = i32::try_from(radius).unwrap_or(i32::MAX);
                for x in cx.saturating_sub(r)..=cx.saturating_add(r) {
                    for y in cy.saturating_sub(r)..=cy.saturating_add(r) {
                        add(&RegionCell::Exterior(x, y));
                    }
                }
            }
        }
        out
    }
}

/// Script locals by script id, then by lower-cased name: slot and type (`ScriptLocals::declared`
/// in [`super::global_scripts`], fixed by the script set).
pub(super) type Declared = HashMap<String, HashMap<String, (usize, VarType)>>;

/// The local instances' locals as everything outside the instance set sees them (`ref.var`,
/// DQ-5b2): `ScriptLocals` (in [`super::Rpg`]) holds it next to the globals' copy, so dialogue
/// results, the console and the scripts (through `View`) read it and `Rpg::apply` writes it;
/// [`LocalScripts::publish`] fills it and [`super::GlobalScripts`] takes its writes.
///
/// - **Which objects.** Every scripted reference of [`ScriptRefs`] and every instance (live or
///   parked) is addressed here, NPCs included: their dialogue `Local` filters and results read
///   and write this one store. Any other key (an unscripted reference, an actor not spawned from
///   a placement) keeps the NPC dialogue store (`RpgState::locals`). As `OpenMW`
///   (`apps/openmw/mwscript/interpretercontext.cpp` `getMemberShort/Long/Float`, non-global
///   path: the reference's `RefData::getLocals`, @ 4e718b50), the locals belong to the reference.
/// - **Parked and never-run instances.** A parked one (outside the region) reads its saved
///   locals and a write updates them, as `OpenMW` keeps them in `RefData` whatever the cell's
///   state; a scripted reference that never had an instance reads 0 and a write creates its
///   instance, parked, which the region restores like any other.
/// - **A base id names one reference** (semantics rule 7, [`Self::resolve`]), for `id.var` and
///   for every other script target by base (`"id"->ModDisposition`, `"id"->AddItem`, stats:
///   `Content::target` asks here first): the nearest live reference to the player (by its
///   `CELL` position; lowest `PlacementId` on a tie or without a player position), else the
///   lowest `PlacementId` of the base. Live: a scripted reference's instance is live (not
///   parked); an unscripted one (actor, container) stands in the script-active region the
///   instances were last synced to. As `OpenMW` `World::searchPtr`
///   (`apps/openmw/mwworld/worldimp.cpp` @ 4e718b50: the active cells first, then every cell),
///   made deterministic by distance and id where `OpenMW` takes the first match.
/// - **Timing**, as the globals' copy: a write lands here at once and in the instance at the
///   next tick start (after that tick's ops for a write the tick made, overriding what the
///   instance wrote to itself in that tick); scripts of a tick read the tick-start values plus
///   the tick's earlier `set ref.var` writes (the run overlay).
#[derive(Default)]
pub(super) struct ObjectLocals {
    refs: Arc<ScriptRefs>,
    /// Every live and parked instance's script and slot values, by key, as last published plus
    /// the writes since.
    values: HashMap<LocalKey, ObjectValues>,
    /// The player's position (engine metres) at the last publish, for rule 7.
    player: Option<[f64; 3]>,
    /// The region (centre, radius) the instances were synced to at the last publish: where an
    /// unscripted reference is live for rule 7.
    region: Option<(RegionCell, u32)>,
    /// Writes not yet in the instances, in call order.
    pub(super) writes: Vec<ObjectWrite>,
}

/// One instance's script and slot values in [`ObjectLocals`].
struct ObjectValues {
    script: String,
    /// In the script-active region (not parked).
    live: bool,
    locals: Vec<f64>,
}

/// A queued `set ref.var` (value already converted).
pub(super) struct ObjectWrite {
    pub(super) key: LocalKey,
    pub(super) script: String,
    pub(super) slot: usize,
    pub(super) value: f64,
}

impl ObjectLocals {
    pub(super) fn new(refs: Arc<ScriptRefs>) -> Self {
        Self { refs, ..Self::default() }
    }

    /// The script of `key`'s instance, else of its scripted reference (a stack has one only
    /// while its instance exists).
    fn script(&self, key: LocalKey) -> Option<&str> {
        self.values.get(&key).map(|v| v.script.as_str()).or_else(|| match key {
            LocalKey::Placement(p) => self.refs.script(p),
            LocalKey::Stack { .. } => None,
        })
    }

    /// Are `key`'s locals kept here (see the type docs)?
    pub(super) fn addresses(&self, key: LocalKey) -> bool {
        self.script(key).is_some()
    }

    /// The base record id of scripted reference `placement` (lower-cased).
    pub(super) fn base(&self, placement: u128) -> Option<&str> {
        self.refs.refs.get(&placement).map(|r| r.base.as_str())
    }

    /// Rule 7: the placement base `base` (lower-cased) names (type docs); `None`: no indexed
    /// reference has that base.
    pub(super) fn resolve(&self, base: &str) -> Option<u128> {
        let all = self.refs.placements(base);
        let dist = |p: u128| match (self.player, self.refs.refs.get(&p)) {
            (Some(a), Some(r)) => (0..3).map(|i| (a[i] - r.pos[i]).powi(2)).sum::<f64>(),
            _ => 0.0,
        };
        let live = |p: u128| match self.values.get(&LocalKey::Placement(p)) {
            Some(v) => v.live,
            None => match (&self.region, self.refs.refs.get(&p)) {
                (Some((centre, radius)), Some(r)) => r.script.is_none() && r.cell.within(centre, *radius),
                _ => false,
            },
        };
        all.iter()
            .copied()
            .filter(|&p| live(p))
            .min_by(|&a, &b| dist(a).total_cmp(&dist(b)).then(a.cmp(&b)))
            .or_else(|| all.first().copied())
    }

    /// `key.name`'s type and value; `None`: its script declares no such local.
    #[allow(clippy::cast_possible_truncation)] // locals are stored converted (`f32` holds them)
    pub(super) fn get(&self, declared: &Declared, key: LocalKey, name: &str) -> Option<(VarType, f32)> {
        let script = self.script(key)?;
        let &(slot, ty) = declared.get(script)?.get(name)?;
        let value = self.values.get(&key).and_then(|v| v.locals.get(slot)).copied().unwrap_or(0.0);
        Some((ty, value as f32))
    }

    /// Writes `key.name`, converted to its type, and queues it; `false`: its script declares no
    /// such local.
    pub(super) fn set(&mut self, declared: &Declared, key: LocalKey, name: &str, value: f32) -> bool {
        let Some(script) = self.script(key).map(str::to_owned) else { return false };
        let Some(locals) = declared.get(&script) else { return false };
        let Some(&(slot, ty)) = locals.get(name) else { return false };
        let value = mw_script::interp::store(ty, f64::from(value));
        let len = locals.len();
        let entry = self
            .values
            .entry(key)
            .or_insert_with(|| ObjectValues { script: script.clone(), live: false, locals: Vec::new() });
        if entry.locals.len() < len {
            entry.locals.resize(len, 0.0);
        }
        entry.locals[slot] = value;
        self.writes.push(ObjectWrite { key, script, slot, value });
        true
    }
}

/// The local instances and the references they come from (owned by [`super::GlobalScripts`]).
#[derive(Clone)]
pub(super) struct LocalScripts {
    /// Shared with the [`ObjectLocals`] mirror.
    pub(super) refs: Arc<ScriptRefs>,
    pub(super) instances: LocalInstances,
    /// Instances taken when their reference left the region, by key (module docs).
    parked: BTreeMap<LocalKey, SavedLocalInstance>,
    /// Instances parked since the last [`Self::publish`].
    left: Vec<LocalKey>,
    /// Instances [`Self::sync`] started zeroed since the last [`Self::take_fresh`].
    fresh: Vec<LocalKey>,
    /// Per holder, its scripted stacks and their scripts at the last [`Self::sync_stacks`]
    /// (absent: not synced since a clear or load).
    stacks: HashMap<u128, BTreeMap<LocalKey, String>>,
    /// Stack instances removed since the last [`Self::publish`].
    gone: Vec<LocalKey>,
    /// The centre and radius the instances were last synced to.
    synced: Option<(RegionCell, u32)>,
    /// `mw.script.active_radius`, in cells.
    pub(super) radius: u32,
}

impl LocalScripts {
    pub(super) fn new(db: &RecordDb) -> Self {
        Self {
            refs: Arc::new(ScriptRefs::build(db)),
            instances: LocalInstances::default(),
            parked: BTreeMap::new(),
            left: Vec::new(),
            fresh: Vec::new(),
            stacks: HashMap::new(),
            gone: Vec::new(),
            synced: None,
            radius: mw_script::ACTIVE_RADIUS.default_value(),
        }
    }

    /// Every instance and parked locals dropped (a restart from a save without any).
    pub(super) fn clear(&mut self) {
        self.instances = LocalInstances::default();
        self.parked.clear();
        self.left.clear();
        self.fresh.clear();
        self.stacks.clear();
        self.gone.clear();
        self.synced = None;
    }

    /// The save's local part: the live instances and the parked ones, each in key order.
    pub(super) fn snapshot(&self, set: &ScriptSet) -> (LocalsSnapshot, LocalsSnapshot) {
        let parked = LocalsSnapshot { locals: self.parked.values().cloned().collect() };
        (self.instances.snapshot(set), parked)
    }

    /// Replaces the live and parked instances with a [`Self::snapshot`]'s, both remapped by name
    /// onto `set` now ([`LocalInstances::restore`]), so a parked instance's reports come at load
    /// time too; a parked key that is also live keeps the live one. The next [`Self::sync`]
    /// diffs them against the region (module docs).
    pub(super) fn load(&mut self, set: &ScriptSet, live: &LocalsSnapshot, parked: &LocalsSnapshot) -> Vec<(LocalKey, LoadReport)> {
        let (instances, mut reports) = LocalInstances::load(live, set);
        self.instances = instances;
        self.parked.clear();
        self.left.clear();
        self.fresh.clear();
        self.stacks.clear();
        self.gone.clear();
        self.synced = None;
        let mut scratch = LocalInstances::default();
        for saved in parked.locals.iter().filter(|s| self.instances.get(s.key).is_none()) {
            reports.extend(scratch.restore(set, saved).into_iter().map(|r| (saved.key, r)));
            if let Some(taken) = scratch.take(set, saved.key) {
                self.parked.insert(saved.key, taken);
            }
        }
        reports
    }

    /// The instances matched to the region around `centre` (`None`: unchanged): references that
    /// left are parked, ones that entered restored or started zeroed. Returns the restore
    /// reports, for the caller to log.
    pub(super) fn sync(&mut self, set: &ScriptSet, centre: Option<&RegionCell>) -> Vec<String> {
        let mut reports = Vec::new();
        let Some(centre) = centre else { return reports };
        if self.synced.as_ref().is_some_and(|(c, r)| c == centre && *r == self.radius) {
            return reports;
        }
        let wanted = self.refs.region(centre, self.radius);
        let leaving: BTreeSet<LocalKey> = self
            .instances
            .iter()
            .map(|(k, _)| k)
            .filter(|k| matches!(k, LocalKey::Placement(p) if !wanted.contains_key(p)))
            .collect();
        for key in leaving {
            if let Some(saved) = self.instances.take(set, key) {
                self.parked.insert(key, saved);
                self.left.push(key);
            }
        }
        for (&placement, &script) in &wanted {
            let key = LocalKey::Placement(placement);
            if self.instances.get(key).is_some() {
                continue;
            }
            match self.parked.remove(&key) {
                Some(saved) if saved.script.eq_ignore_ascii_case(script) => {
                    reports.extend(self.instances.restore(set, &saved).iter().map(|r| format!("{key:?}: {r:?}")));
                }
                _ => {
                    if self.instances.insert(set, key, script) {
                        self.fresh.push(key);
                    }
                }
            }
        }
        self.synced = Some((centre.clone(), self.radius));
        reports
    }

    /// The instances of `holder`'s scripted stacks matched to its inventory `stacks` (module
    /// docs): a stack new since the last call starts zeroed, one gone since (or whose script
    /// changed) loses its instance. The first call after a clear or load diffs against the
    /// instances, so loaded stack instances keep their locals.
    pub(super) fn sync_stacks<'a>(&mut self, set: &ScriptSet, holder: u128, stacks: impl IntoIterator<Item = (StackId, &'a RecordId)>) {
        let refs = &self.refs;
        let wanted: BTreeMap<LocalKey, &str> = stacks
            .into_iter()
            .filter_map(|(id, item)| Some((LocalKey::Stack { holder, stack: u64::from(id.0) }, refs.item_script(item.as_str())?)))
            .collect();
        let instances = &self.instances;
        let previous = self.stacks.entry(holder).or_insert_with(|| {
            instances
                .iter()
                .filter(|(k, _)| matches!(k, LocalKey::Stack { holder: h, .. } if *h == holder))
                .map(|(k, i)| (k, i.script.clone()))
                .collect()
        });
        if previous.len() == wanted.len() && previous.iter().zip(&wanted).all(|((a, s), (b, t))| a == b && s == t) {
            return;
        }
        for (key, script) in previous.iter() {
            if wanted.get(key) != Some(&script.as_str()) && self.instances.remove(*key) {
                self.gone.push(*key);
            }
        }
        for (&key, &script) in &wanted {
            if previous.get(&key).map(String::as_str) != Some(script) {
                self.instances.insert(set, key, script);
            }
        }
        *previous = wanted.into_iter().map(|(k, s)| (k, s.to_owned())).collect();
    }

    /// The instances [`Self::sync`] started zeroed since the last call, for the caller to seed.
    pub(super) fn take_fresh(&mut self) -> Vec<LocalKey> {
        std::mem::take(&mut self.fresh)
    }

    /// `key`'s local `name`, live or parked: type and value; `None`: no instance, or its script
    /// declares no `name`.
    pub(super) fn local(&self, set: &ScriptSet, key: LocalKey, name: &str) -> Option<(VarType, f64)> {
        let (script, value) = match (self.instances.get(key), self.parked.get(&key)) {
            (Some(instance), _) => (instance.script.clone(), self.instances.local(set, key, name)),
            (None, Some(saved)) => {
                let value = saved.locals.iter().find(|l| l.name.eq_ignore_ascii_case(name)).map(|l| l.value);
                (saved.script.to_lowercase(), value)
            }
            (None, None) => return None,
        };
        let ty = set.get(&script)?.locals.iter().find(|l| l.name.key.eq_ignore_ascii_case(name))?.ty;
        Some((ty, value.unwrap_or(0.0)))
    }

    /// Has `key` an instance, live or parked?
    pub(super) fn has(&self, key: LocalKey) -> bool {
        self.instances.get(key).is_some() || self.parked.contains_key(&key)
    }

    /// One local of `write.key`'s instance by slot (`value` already converted): the live
    /// instance, else the parked one (by name), else a new parked instance of `write.script`
    /// with `seeds` (name, value) set first ([`ObjectLocals`]). Nothing for a slot the instance's
    /// script lacks, nor for a stack without a live instance (it left the inventory).
    pub(super) fn write(&mut self, set: &ScriptSet, write: &ObjectWrite, seeds: &[(String, f64)]) {
        let key = write.key;
        let live = self.instances.get(key).map(|i| i.script.clone());
        if live.is_none() && matches!(key, LocalKey::Stack { .. }) {
            return;
        }
        let parked = self.parked.get(&key).map(|s| s.script.to_lowercase());
        let script = live.clone().or(parked).unwrap_or_else(|| write.script.clone());
        let Some(name) = set.get(&script).and_then(|s| s.locals.get(write.slot)).map(|l| l.name.key.clone()) else {
            return;
        };
        if live.is_some() {
            self.instances.set_local(set, key, &name, write.value);
            return;
        }
        let mut scratch = LocalInstances::default();
        if let Some(saved) = self.parked.get(&key) {
            let _ = scratch.restore(set, saved);
        } else {
            scratch.insert(set, key, &script);
            for (seed, value) in seeds {
                scratch.set_local(set, key, seed, *value);
            }
        }
        scratch.set_local(set, key, &name, write.value);
        if let Some(saved) = scratch.take(set, key) {
            self.parked.insert(key, saved);
        }
    }

    /// The instances into `objects` ([`ObjectLocals`]): every live one's values; the ones parked
    /// since the last call marked parked (their values are the last published ones, the queued
    /// writes having been taken before the park); with `full` (new game, restart, load)
    /// everything rebuilt, parked ones from their saved locals.
    pub(super) fn publish(&mut self, set: &ScriptSet, objects: &mut ObjectLocals, full: bool, player: Option<[f64; 3]>) {
        objects.player = player;
        objects.region.clone_from(&self.synced);
        let values = &mut objects.values;
        if full {
            values.clear();
            self.gone.clear();
            for (&key, saved) in &self.parked {
                let Some(script) = set.get(&saved.script) else { continue };
                let by_name = |name: &str| saved.locals.iter().find(|s| s.name.eq_ignore_ascii_case(name)).map_or(0.0, |s| s.value);
                let locals = script.locals.iter().map(|l| by_name(&l.name.key)).collect();
                values.insert(key, ObjectValues { script: saved.script.to_lowercase(), live: false, locals });
            }
        }
        for key in self.gone.drain(..) {
            values.remove(&key);
        }
        for key in self.left.drain(..) {
            if let Some(v) = values.get_mut(&key) {
                v.live = false;
            }
        }
        for (key, instance) in self.instances.iter() {
            let v = values
                .entry(key)
                .or_insert_with(|| ObjectValues { script: instance.script.clone(), live: true, locals: Vec::new() });
            if v.script != instance.script {
                v.script.clone_from(&instance.script);
            }
            v.live = true;
            v.locals.clone_from(&instance.locals.0);
        }
    }
}

/// The player body's cell into [`ScriptRegion`] (no body: unchanged).
pub(super) fn track_script_region(mode: Option<Res<PlayerMode>>, positions: WorldPosOf, mut region: ResMut<ScriptRegion>) {
    let Some(location) = mode.and_then(|m| m.body).and_then(|body| positions.location(body)) else { return };
    let p = location.pos.to_f64();
    let cell = if location.worldspace.as_str() == mw_world::ids::EXTERIOR {
        RegionCell::exterior_at(p.x, p.z)
    } else {
        RegionCell::interior(location.worldspace.as_str())
    };
    if region.centre.as_ref() != Some(&cell) {
        region.centre = Some(cell);
    }
    region.player = Some(p.to_array());
}

#[cfg(test)]
#[allow(clippy::float_cmp, reason = "script values here are small integers, stored exactly")]
mod tests {
    //! Headless, over fixture records: a scripted chest runs while the player is near, parks its
    //! locals when the player leaves and resumes with them; it keeps running after its own
    //! `Disable`; the global it starts runs the same tick; its self-targeted `AddItem` reaches
    //! that exact reference, not another chest of the same base.
    use std::sync::Arc;

    use mw_rpg::{NoSheets, Target};
    use sf_rpg::ActorKey;
    use tes3::esp::{CellFlags, GlobalValue, GlobalVariable, Reference, Script};

    use super::super::{GlobalScripts, Rpg, id};
    use super::*;

    /// The chest near the start, the one far away, the one in the interior.
    const NEAR: u32 = 1;
    const FAR: u32 = 2;
    const INSIDE: u32 = 3;
    const ROOM: &str = "DQ Room";

    fn placement(refr_index: u32) -> u128 {
        mw_world::ids::placement_id("test", refr_index).0
    }

    fn reference(refr_index: u32, base: &str) -> Reference {
        Reference { mast_index: 0, refr_index, id: base.into(), ..Reference::default() }
    }

    fn exterior(db: &mut RecordDb, src: sf_records::SourceId, grid: (i32, i32), refs: Vec<Reference>) {
        let mut cell = Cell::default();
        cell.data.grid = grid;
        for r in refs {
            cell.references.insert((r.mast_index, r.refr_index), r);
        }
        db.define(src, mw_records::keys::exterior(grid), cell).unwrap();
    }

    /// `dq_obj` on the container `dq_chest`: counts its runs in `n` and copies them to `dq_count`,
    /// on its first run disables itself and starts `dq_glob` (which counts its runs in
    /// `dq_glob_ran`), and adds one `dq_coin` to itself every run. Chest [`NEAR`] in (0, 0),
    /// [`FAR`] in (5, 5), [`INSIDE`] in the interior [`ROOM`].
    fn records() -> RecordDb {
        fixture(false)
    }

    /// The second chest in (0, 0), far from the first.
    const NEAR2: u32 = 5;
    /// The scripted NPC in (0, 0).
    const NPC: u32 = 4;
    /// Where [`NEAR2`] stands (Morrowind units, inside cell (0, 0)).
    const NEAR2_AT: [f32; 3] = [6000.0, 6000.0, 0.0];

    /// [`records`]; with `more` (DQ-5b2) also chest [`NEAR2`] at [`NEAR2_AT`] and the NPC
    /// `dq_npc` ([`NPC`]) in (0, 0), whose script `dq_talk` copies its local `said` to `dq_said`
    /// every run and turns a 2 into a 3. The short globals `dq_seen` and `dq_said` either way.
    fn fixture(more: bool) -> RecordDb {
        let mut db = RecordDb::new();
        mw_records::register_kinds(&mut db).unwrap();
        let src = db.add_source("test", &[]);
        for name in ["dq_count", "dq_glob_ran", "dq_seen", "dq_said"] {
            let global = GlobalVariable { id: name.into(), value: GlobalValue::Short(0), ..Default::default() };
            db.define(src, id(name), global).unwrap();
        }
        let text = "Begin dq_obj\nshort n\nset n to n + 1\nset dq_count to n\nif ( n == 1 )\n\tDisable\n\tStartScript dq_glob\nendif\nAddItem \"dq_coin\" 1\nEnd\n";
        db.define(src, id("dq_obj"), Script { id: "dq_obj".into(), text: text.into(), ..Default::default() }).unwrap();
        let text = "Begin dq_glob\nset dq_glob_ran to dq_glob_ran + 1\nEnd\n";
        db.define(src, id("dq_glob"), Script { id: "dq_glob".into(), text: text.into(), ..Default::default() }).unwrap();
        let chest = Container { id: "dq_chest".into(), script: "dq_obj".into(), ..Default::default() };
        db.define(src, id("dq_chest"), chest).unwrap();
        let plain = Container { id: "dq_plain".into(), ..Default::default() };
        db.define(src, id("dq_plain"), plain).unwrap();
        let mut near = vec![reference(NEAR, "DQ_Chest"), reference(9, "dq_plain")];
        if more {
            let text = "Begin dq_talk\nshort said\nset dq_said to said\nif ( said == 2 )\n\tset said to 3\nendif\nEnd\n";
            let script = Script { id: "dq_talk".into(), text: text.into(), ..Default::default() };
            db.define(src, id("dq_talk"), script).unwrap();
            let npc = Npc { id: "dq_npc".into(), script: "dq_talk".into(), ..Default::default() };
            db.define(src, id("dq_npc"), npc).unwrap();
            near.push(Reference { translation: NEAR2_AT, ..reference(NEAR2, "dq_chest") });
            near.push(reference(NPC, "dq_npc"));
        }
        exterior(&mut db, src, (0, 0), near);
        exterior(&mut db, src, (5, 5), vec![reference(FAR, "dq_chest")]);
        let mut room = Cell { name: ROOM.into(), ..Cell::default() };
        room.data.flags = CellFlags::IS_INTERIOR;
        room.references.insert((0, INSIDE), reference(INSIDE, "dq_chest"));
        db.define(src, RecordId::new(ROOM), room).unwrap();
        db
    }

    fn new_game() -> (Rpg, GlobalScripts) {
        let mut rpg = Rpg::new(Arc::new(records())).unwrap();
        let scripts = GlobalScripts::new_game(&mut rpg);
        (rpg, scripts)
    }

    fn global(rpg: &Rpg, name: &str) -> f32 {
        rpg.state.globals.get(&id(name)).copied().unwrap_or(0.0)
    }

    fn coins(rpg: &Rpg, refr_index: u32) -> u64 {
        rpg.state.inventories.get(&ActorKey(placement(refr_index))).map_or(0, |inv| inv.loadout.count(&id("dq_coin")))
    }

    fn n(scripts: &GlobalScripts, refr_index: u32) -> Option<f64> {
        scripts.locals().get(LocalKey::Placement(placement(refr_index))).map(|i| i.locals.0[0])
    }

    fn tick_at(rpg: &mut Rpg, scripts: &mut GlobalScripts, centre: RegionCell) {
        scripts.set_region_centre(Some(centre));
        scripts.tick(rpg, &NoSheets);
    }

    #[test]
    fn only_scripted_references_are_indexed_by_their_cell() {
        let refs = ScriptRefs::build(&records());
        assert_eq!(refs.refs.len(), 4, "the unscripted chest too (a rule 7 target by base)");
        assert_eq!(refs.script(placement(9)), None, "with no script");
        assert_eq!(refs.cells.values().flatten().count(), 3, "and in no cell: it never runs");
        assert_eq!(refs.base(placement(NEAR)).as_deref(), Some("dq_chest"));
        assert_eq!(refs.region(&RegionCell::Exterior(1, -1), 1).keys().copied().collect::<Vec<_>>(), [placement(NEAR)]);
        assert!(refs.region(&RegionCell::Exterior(2, 0), 1).is_empty(), "two cells away: outside radius 1");
        assert_eq!(refs.region(&RegionCell::interior("dq room"), 1).keys().copied().collect::<Vec<_>>(), [placement(INSIDE)]);
    }

    #[test]
    fn exterior_cells_follow_the_world_conversion() {
        let at = |p: [f32; 3]| {
            let e = mw_world::convert::engine_f64(p);
            RegionCell::exterior_at(e.x, e.z)
        };
        assert_eq!(at([100.0, 100.0, 0.0]), RegionCell::Exterior(0, 0));
        assert_eq!(at([-1.0, 8192.0 * 3.0 + 5.0, 0.0]), RegionCell::Exterior(-1, 3));
        let (x, y) = mw_world::convert::grid_of([-20000.0, -9000.0, 0.0]);
        assert_eq!(at([-20000.0, -9000.0, 0.0]), RegionCell::Exterior(x, y));
    }

    #[test]
    fn nothing_runs_before_the_players_cell_is_known() {
        let (mut rpg, mut scripts) = new_game();
        scripts.tick(&mut rpg, &NoSheets);
        assert!(scripts.locals().is_empty());
        assert_eq!(global(&rpg, "dq_count"), 0.0);
    }

    #[test]
    fn a_local_runs_in_the_region_parks_its_locals_outside_and_resumes_with_them() {
        let (mut rpg, mut scripts) = new_game();
        tick_at(&mut rpg, &mut scripts, RegionCell::Exterior(0, 0));
        assert_eq!(n(&scripts, NEAR), Some(1.0), "it ran on its first tick in the region");
        assert_eq!(global(&rpg, "dq_count"), 1.0);
        assert_eq!(n(&scripts, FAR), None, "the far chest is outside the region");
        tick_at(&mut rpg, &mut scripts, RegionCell::Exterior(1, 1));
        assert_eq!(n(&scripts, NEAR), Some(2.0), "a neighbour cell keeps it in the 3x3 region");

        for _ in 0..3 {
            tick_at(&mut rpg, &mut scripts, RegionCell::Exterior(2, 0));
        }
        assert_eq!(n(&scripts, NEAR), None, "two cells away: taken");
        assert_eq!(global(&rpg, "dq_count"), 2.0, "and not run");

        tick_at(&mut rpg, &mut scripts, RegionCell::Exterior(0, -1));
        assert_eq!(n(&scripts, NEAR), Some(3.0), "back: resumed with its locals");
        assert_eq!(global(&rpg, "dq_count"), 3.0);

        tick_at(&mut rpg, &mut scripts, RegionCell::interior(ROOM));
        assert_eq!(n(&scripts, NEAR), None, "an interior's region is that cell only");
        assert_eq!(n(&scripts, INSIDE), Some(1.0));
    }

    #[test]
    fn the_fixed_tick_runs_locals_of_the_region_resource() {
        use std::time::Duration;

        use bevy_app::App;
        use bevy_time::{Fixed, Time, TimePlugin, TimeUpdateStrategy};

        let mut rpg = Rpg::new(Arc::new(records())).unwrap();
        let mut app = App::new();
        app.add_plugins(TimePlugin)
            .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(1.0 / 60.0)))
            .insert_resource(Time::<Fixed>::from_hz(60.0));
        super::super::global_scripts::add(&mut app, &mut rpg);
        app.insert_resource(rpg);
        app.update();
        assert!(app.world().resource::<GlobalScripts>().locals().is_empty(), "no player cell: nothing runs");
        app.world_mut().resource_mut::<ScriptRegion>().centre = Some(RegionCell::Exterior(0, 0));
        for _ in 0..6 {
            app.update();
        }
        let ran = n(app.world().resource::<GlobalScripts>(), NEAR).expect("the near chest has an instance");
        assert!(ran >= 1.0, "it ran on the fixed tick");
        assert_eq!(f64::from(global(app.world().resource::<Rpg>(), "dq_count")), ran);
    }

    #[test]
    fn a_disabled_reference_keeps_running() {
        let (mut rpg, mut scripts) = new_game();
        for _ in 0..4 {
            tick_at(&mut rpg, &mut scripts, RegionCell::Exterior(0, 0));
        }
        assert_eq!(n(&scripts, NEAR), Some(4.0), "its first run's Disable did not stop it");
        assert!(scripts.locals().is_running(LocalKey::Placement(placement(NEAR))));
    }

    #[test]
    fn a_global_a_local_starts_runs_the_same_tick() {
        let (mut rpg, mut scripts) = new_game();
        tick_at(&mut rpg, &mut scripts, RegionCell::Exterior(0, 0));
        assert!(scripts.instances().is_running("dq_glob"));
        assert_eq!(global(&rpg, "dq_glob_ran"), 1.0, "started by the local, run in the tick's global step");
        let target = scripts.instances().get("dq_glob").and_then(|g| g.target.clone());
        assert_eq!(target.as_deref(), Some("dq_chest"), "its implicit target is the chest's base");
        tick_at(&mut rpg, &mut scripts, RegionCell::Exterior(0, 0));
        assert_eq!(global(&rpg, "dq_glob_ran"), 2.0);
    }

    #[test]
    fn a_locals_self_is_its_exact_reference() {
        let (mut rpg, mut scripts) = new_game();
        for _ in 0..2 {
            tick_at(&mut rpg, &mut scripts, RegionCell::Exterior(0, 0));
        }
        assert_eq!(coins(&rpg, NEAR), 2, "AddItem on self reached the running chest");
        assert_eq!(coins(&rpg, FAR), 0, "not another chest of the same base");
        assert_eq!(coins(&rpg, INSIDE), 0);
        tick_at(&mut rpg, &mut scripts, RegionCell::Exterior(5, 5));
        assert_eq!((coins(&rpg, NEAR), coins(&rpg, FAR)), (2, 1), "the far chest's own run hits itself");
    }

    // ------------------------------------------------------------ DQ-5b2: `ref.var`

    fn new_game_with(db: RecordDb) -> (Rpg, GlobalScripts) {
        let mut rpg = Rpg::new(Arc::new(db)).unwrap();
        let scripts = GlobalScripts::new_game(&mut rpg);
        (rpg, scripts)
    }

    /// One console line; its notes.
    fn console(rpg: &mut Rpg, line: &str) -> Vec<String> {
        let out = rpg.run_console(line, None, &NoSheets).unwrap();
        out.into_iter().filter(|l| l.starts_with("note:")).collect()
    }

    /// A dialogue context with `speaker`.
    fn ctx(speaker: u128) -> sf_rpg::ConversationCtx {
        sf_rpg::ConversationCtx { player: sf_rpg::PlayerId::HOST, speaker: ActorKey(speaker), speaker_base: id(""), choice: None }
    }

    fn view(rpg: &Rpg) -> super::super::View<'_> {
        super::super::View { c: &rpg.content, s: &rpg.state, g: &rpg.script_locals, sheets: &NoSheets }
    }

    /// `target.name` as dialogue results and scripts read it (`View::var`).
    fn var(rpg: &Rpg, speaker: u128, target: &Target, name: &str) -> Option<f32> {
        mw_rpg::MwWorld::var(&view(rpg), &ctx(speaker), target, &id(name)).map(|v| v.value)
    }

    /// A result's `set target.name to value` (`Rpg::apply`), `speaker` speaking.
    fn set(rpg: &mut Rpg, speaker: u128, target: Target, name: &str, value: f32) {
        let effect = mw_rpg::ResultEffect::SetLocal { target, name: id(name), value };
        let _ = rpg.apply(None, &ctx(speaker), effect, &NoSheets);
    }

    #[test]
    fn a_console_line_reads_and_resets_a_placed_objects_local() {
        let (mut rpg, mut scripts) = new_game_with(records());
        for _ in 0..3 {
            tick_at(&mut rpg, &mut scripts, RegionCell::Exterior(0, 0));
        }
        assert!(console(&mut rpg, "set dq_seen to dq_chest.n").is_empty());
        assert_eq!(global(&rpg, "dq_seen"), 3.0, "the live chest's local (the others are outside)");

        assert!(console(&mut rpg, "set dq_chest.n to 0").is_empty());
        console(&mut rpg, "set dq_seen to dq_chest.n");
        assert_eq!(global(&rpg, "dq_seen"), 0.0, "the write is seen at once");
        assert_eq!(n(&scripts, NEAR), Some(3.0), "the instance takes it at the tick");
        tick_at(&mut rpg, &mut scripts, RegionCell::Exterior(0, 0));
        assert_eq!(n(&scripts, NEAR), Some(1.0), "the script continued from 0");
        assert_eq!(global(&rpg, "dq_count"), 1.0);
        let notes = console(&mut rpg, "set dq_seen to dq_chest.nope");
        assert_eq!(notes, ["note: unknown variable dq_chest.nope"]);
    }

    #[test]
    fn an_actor_base_picks_the_nearest_live_instance_and_a_ref_the_exact_one() {
        let (mut rpg, mut scripts) = new_game_with(fixture(true));
        let (a, b) = (placement(NEAR), placement(NEAR2));
        tick_at(&mut rpg, &mut scripts, RegionCell::Exterior(0, 0));
        set(&mut rpg, 0, Target::Ref(b), "n", 50.0);
        tick_at(&mut rpg, &mut scripts, RegionCell::Exterior(0, 0));
        assert_eq!((n(&scripts, NEAR), n(&scripts, NEAR2)), (Some(2.0), Some(51.0)), "the write hit the exact reference");
        assert_eq!(var(&rpg, 0, &Target::Ref(a), "n"), Some(2.0));
        assert_eq!(var(&rpg, 0, &Target::Ref(b), "n"), Some(51.0));

        let base = Target::Actor(id("dq_chest"));
        let lowest = if a < b { 2.0 } else { 51.0 };
        assert_eq!(var(&rpg, 0, &base, "n"), Some(lowest), "no player position: the lowest PlacementId");

        let at = |p: [f32; 3]| Some(mw_world::convert::engine_f64(p).to_array());
        scripts.set_player_position(at([5900.0, 5900.0, 0.0]));
        tick_at(&mut rpg, &mut scripts, RegionCell::Exterior(0, 0));
        assert_eq!(var(&rpg, 0, &base, "n"), Some(52.0), "the nearest live instance");
        set(&mut rpg, 0, base.clone(), "n", 7.0);
        assert_eq!(var(&rpg, 0, &Target::Ref(b), "n"), Some(7.0), "a write by base reaches the same one");
        assert_eq!(var(&rpg, 0, &Target::Ref(a), "n"), Some(3.0));

        scripts.set_player_position(at([10.0, 10.0, 0.0]));
        tick_at(&mut rpg, &mut scripts, RegionCell::Exterior(0, 0));
        assert_eq!(var(&rpg, 0, &base, "n"), Some(4.0), "now the first chest is nearer");
        assert_eq!(n(&scripts, NEAR2), Some(8.0));
    }

    #[test]
    fn an_npcs_dialogue_local_and_its_script_share_one_value() {
        let (mut rpg, mut scripts) = new_game_with(fixture(true));
        let npc = placement(NPC);
        assert!(rpg.add_actor(ActorKey(npc), &id("dq_npc"), ""));
        let local = |rpg: &Rpg| mw_rpg::MwView::local(&view(rpg), ActorKey(npc), &id("said"));
        set(&mut rpg, npc, Target::Speaker, "said", 2.0);
        assert_eq!(local(&rpg), Some(2.0), "the `Local` filter sees the result's write at once");
        assert!(rpg.state.locals.is_empty(), "not a second store");

        tick_at(&mut rpg, &mut scripts, RegionCell::Exterior(0, 0));
        assert_eq!(global(&rpg, "dq_said"), 2.0, "the script's first run read it");
        assert_eq!(local(&rpg), Some(3.0), "and dialogue reads what the script wrote");
        assert_eq!(var(&rpg, npc, &Target::Speaker, "said"), Some(3.0));
        assert_eq!(var(&rpg, 0, &Target::Actor(id("dq_npc")), "said"), Some(3.0));
        assert_eq!(var(&rpg, npc, &Target::Speaker, "nope"), None, "undeclared: unknown");
    }

    #[test]
    fn a_fresh_instance_starts_with_the_dialogue_stores_locals() {
        let (mut rpg, mut scripts) = new_game_with(fixture(true));
        let npc = ActorKey(placement(NPC));
        rpg.state.locals.insert((npc, id("said")), 2.0);
        tick_at(&mut rpg, &mut scripts, RegionCell::Exterior(0, 0));
        assert_eq!(global(&rpg, "dq_said"), 2.0, "an older save's value reached the instance");
        assert!(rpg.state.locals.is_empty(), "moved, not copied");
        assert_eq!(scripts.object_local(npc.0, "said"), Some((VarType::Short, 3.0)));
    }

    #[test]
    fn a_parked_instances_local_reads_its_saved_value_and_takes_writes() {
        let (mut rpg, mut scripts) = new_game_with(records());
        let (near, far) = (placement(NEAR), placement(FAR));
        for _ in 0..2 {
            tick_at(&mut rpg, &mut scripts, RegionCell::Exterior(0, 0));
        }
        tick_at(&mut rpg, &mut scripts, RegionCell::interior(ROOM));
        assert_eq!(n(&scripts, NEAR), None, "parked");
        assert_eq!(var(&rpg, 0, &Target::Ref(near), "n"), Some(2.0), "reads its saved locals");
        assert_eq!(var(&rpg, 0, &Target::Actor(id("dq_chest")), "n"), Some(1.0), "the base names the live one");
        assert_eq!(var(&rpg, 0, &Target::Ref(far), "n"), Some(0.0), "never run: 0");

        set(&mut rpg, 0, Target::Ref(near), "n", 10.0);
        assert_eq!(var(&rpg, 0, &Target::Ref(near), "n"), Some(10.0), "seen at once");
        let saved = scripts.snapshot(&rpg);
        let parked = saved.parked.locals.iter().find(|s| s.key == LocalKey::Placement(near)).unwrap();
        assert_eq!(parked.locals[0].value, 10.0, "a save before the tick keeps the write");
        tick_at(&mut rpg, &mut scripts, RegionCell::interior(ROOM));
        assert_eq!(scripts.object_local(near, "n"), Some((VarType::Short, 10.0)), "the parked instance took it");
        tick_at(&mut rpg, &mut scripts, RegionCell::Exterior(0, 0));
        assert_eq!(n(&scripts, NEAR), Some(11.0), "and resumes from it");

        set(&mut rpg, 0, Target::Ref(far), "n", 4.0);
        tick_at(&mut rpg, &mut scripts, RegionCell::Exterior(0, 0));
        assert_eq!(scripts.object_local(far, "n"), Some((VarType::Short, 4.0)), "a write created it, parked");
        tick_at(&mut rpg, &mut scripts, RegionCell::Exterior(5, 5));
        assert_eq!(n(&scripts, FAR), Some(5.0), "restored with it when the region reaches it");
    }

    // ------------------------------------------------------------ DQ-5d: unopened containers

    /// An item change by a result or script, `rpg.apply`'s path, on `target`.
    fn items(rpg: &mut Rpg, target: Target, item: &str, count: u32, add: bool) {
        let item = id(item);
        let effect =
            if add { mw_rpg::ResultEffect::AddItem { target, item, count } } else { mw_rpg::ResultEffect::RemoveItem { target, item, count } };
        let _ = rpg.apply(None, &ctx(0), effect, &NoSheets);
    }

    #[test]
    fn an_item_change_on_an_unopened_container_keeps_its_record_contents() {
        const ADDED: u32 = 10;
        const TAKEN: u32 = 11;
        let mut db = records();
        let src = db.add_source("test2", &[]);
        let stocked = Container {
            id: "dq_stocked".into(),
            inventory: vec![(2, tes3::esp::FixedString("dq_gem".into())), (1, tes3::esp::FixedString("dq_ring".into()))],
            ..Default::default()
        };
        db.define(src, id("dq_stocked"), stocked).unwrap();
        let mut store = Cell { name: "DQ Store".into(), ..Cell::default() };
        store.data.flags = CellFlags::IS_INTERIOR;
        for r in [ADDED, TAKEN] {
            store.references.insert((0, r), reference(r, "dq_stocked"));
        }
        db.define(src, RecordId::new("DQ Store"), store).unwrap();
        let mut rpg = Rpg::new(Arc::new(db)).unwrap();
        let open = |rpg: &mut Rpg, refr_index: u32| {
            let key = ActorKey(placement(refr_index));
            let inv = &crate::inventory::container_store(&mut rpg.state, &rpg.content.records, key, &id("dq_stocked")).loadout;
            ["dq_gem", "dq_ring", "dq_coin"].map(|i| inv.count(&id(i)))
        };

        items(&mut rpg, Target::Ref(placement(ADDED)), "dq_coin", 3, true);
        assert_eq!(open(&mut rpg, ADDED), [2, 1, 3], "the record's items and the added one");

        items(&mut rpg, Target::Ref(placement(TAKEN)), "dq_gem", 1, false);
        assert_eq!(open(&mut rpg, TAKEN), [1, 1, 0], "removed from the seeded contents");
        items(&mut rpg, Target::Ref(placement(TAKEN)), "dq_ring", 5, false);
        assert_eq!(open(&mut rpg, TAKEN), [1, 0, 0], "at most what it holds");
    }

    // ------------------------------------------------------------ DQ-5e: one rule 7 resolver

    /// The second `dq_npc` and the second `dq_plain`, both in (1, 0) at [`EAST_AT`].
    const NPC2: u32 = 6;
    const PLAIN2: u32 = 7;
    /// Morrowind units, inside cell (1, 0).
    const EAST_AT: [f32; 3] = [8300.0, 100.0, 0.0];

    /// [`fixture`]`(true)` plus a second reference of the NPC `dq_npc` and of the unscripted
    /// container `dq_plain` in cell (1, 0), the first ones standing at the origin of (0, 0).
    fn two_of_each() -> RecordDb {
        let mut db = fixture(true);
        let src = db.add_source("test2", &[]);
        let east = [(NPC2, "dq_npc"), (PLAIN2, "dq_plain")].map(|(r, base)| Reference { translation: EAST_AT, ..reference(r, base) });
        exterior(&mut db, src, (1, 0), east.into());
        db
    }

    /// The player body at Morrowind position `p`, then one tick with the region on (0, 0).
    fn player_at(rpg: &mut Rpg, scripts: &mut GlobalScripts, p: [f32; 3]) {
        scripts.set_player_position(Some(mw_world::convert::engine_f64(p).to_array()));
        tick_at(rpg, scripts, RegionCell::Exterior(0, 0));
    }

    #[test]
    fn a_base_ids_local_and_its_disposition_hit_the_same_nearest_reference() {
        let (mut rpg, mut scripts) = new_game_with(two_of_each());
        let (west, east) = (placement(NPC), placement(NPC2));
        for p in [west, east] {
            assert!(rpg.add_actor(ActorKey(p), &id("dq_npc"), ""));
        }
        tick_at(&mut rpg, &mut scripts, RegionCell::Exterior(0, 0));
        set(&mut rpg, 0, Target::Ref(west), "said", 10.0);
        set(&mut rpg, 0, Target::Ref(east), "said", 20.0);
        let base = Target::Actor(id("dq_npc"));
        let disposition = |rpg: &mut Rpg, delta| {
            let _ = rpg.apply(None, &ctx(0), mw_rpg::ResultEffect::ModDisposition { target: base.clone(), delta }, &NoSheets);
        };
        let modifiers = |rpg: &Rpg| (rpg.state.npcs.modifier(ActorKey(west)), rpg.state.npcs.modifier(ActorKey(east)));

        player_at(&mut rpg, &mut scripts, [8200.0, 100.0, 0.0]);
        assert_eq!(var(&rpg, 0, &base, "said"), Some(20.0), "`dq_npc.said`: the nearest reference's");
        disposition(&mut rpg, 5);
        assert_eq!(modifiers(&rpg), (0, 5), "`\"dq_npc\"->ModDisposition`: the same one");

        player_at(&mut rpg, &mut scripts, [10.0, 10.0, 0.0]);
        assert_eq!(var(&rpg, 0, &base, "said"), Some(10.0), "now the first one is nearer");
        disposition(&mut rpg, 3);
        assert_eq!(modifiers(&rpg), (3, 5), "and the disposition follows it");
    }

    #[test]
    fn a_container_base_ids_add_item_lands_in_its_nearest_placement() {
        let (mut rpg, mut scripts) = new_game_with(two_of_each());
        let gold = |rpg: &Rpg, refr_index: u32| {
            rpg.state.inventories.get(&ActorKey(placement(refr_index))).map_or(0, |inv| inv.loadout.count(&id("gold_001")))
        };

        player_at(&mut rpg, &mut scripts, [8200.0, 100.0, 0.0]);
        assert!(console(&mut rpg, "\"dq_plain\"->AddItem \"gold_001\" 3").is_empty());
        assert_eq!((gold(&rpg, 9), gold(&rpg, PLAIN2)), (0, 3), "the nearest placement of the container");

        player_at(&mut rpg, &mut scripts, [10.0, 10.0, 0.0]);
        console(&mut rpg, "\"dq_plain\"->AddItem \"gold_001\" 2");
        assert_eq!((gold(&rpg, 9), gold(&rpg, PLAIN2)), (2, 3), "now the one at the origin");

        tick_at(&mut rpg, &mut scripts, RegionCell::Exterior(5, 5));
        console(&mut rpg, "\"dq_plain\"->AddItem \"gold_001\" 1");
        let lowest = if placement(9) < placement(PLAIN2) { (3, 3) } else { (2, 4) };
        assert_eq!((gold(&rpg, 9), gold(&rpg, PLAIN2)), lowest, "none in the region: the lowest PlacementId");
    }

    // ------------------------------------------------------------ DQ-5b3: carried items

    /// The unscripted container holding the ring (any key: it needs no `CELL` reference).
    const BOX: u32 = 30;

    /// [`records`] plus the MISC item `dq_ring`, scripted `dq_carry` (counts its runs in `c`,
    /// copies `dq_count` to `dq_seen` every run, starts the global `dq_held` on its first run),
    /// in the unscripted container `dq_box`.
    fn carried() -> RecordDb {
        let mut db = records();
        let src = db.add_source("test2", &[]);
        let text =
            "Begin dq_carry\nshort c\nset c to c + 1\nset dq_seen to dq_count\nif ( c == 1 )\n\tStartScript dq_held\nendif\nEnd\n";
        db.define(src, id("dq_carry"), Script { id: "dq_carry".into(), text: text.into(), ..Default::default() }).unwrap();
        let text = "Begin dq_held\nEnd\n";
        db.define(src, id("dq_held"), Script { id: "dq_held".into(), text: text.into(), ..Default::default() }).unwrap();
        let ring = MiscItem { id: "dq_ring".into(), script: "dq_carry".into(), ..Default::default() };
        db.define(src, id("dq_ring"), ring).unwrap();
        let boxed = Container { id: "dq_box".into(), inventory: vec![(1, tes3::esp::FixedString("dq_ring".into()))], ..Default::default() };
        db.define(src, id("dq_box"), boxed).unwrap();
        db
    }

    fn the_box() -> sf_rpg::Holder {
        sf_rpg::Holder::Actor(ActorKey(placement(BOX)))
    }

    /// The container window's take of the ring from [`BOX`]: the ring's stack in the player's
    /// inventory, as its local key.
    fn pick_up(rpg: &mut Rpg) -> LocalKey {
        let key = ActorKey(placement(BOX));
        let store = crate::inventory::container_store(&mut rpg.state, &rpg.content.records, key, &id("dq_box"));
        let stack = store.loadout.inventory().stacks()[0].id;
        let player = sf_rpg::Holder::Player(sf_rpg::PlayerId::HOST);
        crate::inventory::move_stack(&mut rpg.state, the_box(), player, stack, 1).expect("the ring moves");
        let ring = rpg.state.pc.loadout.inventory().stacks().iter().find(|s| s.item == id("dq_ring")).expect("carried");
        LocalKey::Stack { holder: PLAYER_HOLDER, stack: u64::from(ring.id.0) }
    }

    fn c(scripts: &GlobalScripts, key: LocalKey) -> Option<f64> {
        scripts.locals().get(key).map(|i| i.locals.0[0])
    }

    fn stack_target(key: LocalKey) -> Target {
        let LocalKey::Stack { holder, stack } = key else { panic!("a stack key") };
        Target::Stack { holder, stack }
    }

    #[test]
    fn a_carried_items_script_runs_every_tick_after_the_placed_references() {
        let (mut rpg, mut scripts) = new_game_with(carried());
        tick_at(&mut rpg, &mut scripts, RegionCell::Exterior(0, 0));
        assert_eq!(scripts.locals().iter().count(), 1, "the chest only: the ring is in its box");
        let ring = pick_up(&mut rpg);
        assert!(ring > LocalKey::Placement(u128::MAX), "every stack key orders after every placement");
        for tick in 1..=3 {
            tick_at(&mut rpg, &mut scripts, RegionCell::Exterior(0, 0));
            assert_eq!(c(&scripts, ring), Some(f64::from(tick)), "picked up: it runs from that tick, once a tick");
            assert_eq!(global(&rpg, "dq_seen"), global(&rpg, "dq_count"), "it saw the chest's write of the same tick");
        }
        assert_eq!(global(&rpg, "dq_count"), 4.0);
        tick_at(&mut rpg, &mut scripts, RegionCell::Exterior(5, 5));
        assert_eq!(c(&scripts, ring), Some(4.0), "carried items run wherever the player is");
        assert_eq!(n(&scripts, NEAR), None, "while the chest is parked");
    }

    #[test]
    fn a_carried_items_self_is_its_stack() {
        let (mut rpg, mut scripts) = new_game_with(carried());
        let ring = pick_up(&mut rpg);
        let me = stack_target(ring);
        for _ in 0..2 {
            tick_at(&mut rpg, &mut scripts, RegionCell::Exterior(0, 0));
        }
        let target = scripts.instances().get("dq_held").and_then(|g| g.target.clone());
        assert_eq!(target.as_deref(), Some("dq_ring"), "a global it starts takes the item's id");
        assert_eq!(var(&rpg, 0, &me, "c"), Some(2.0), "`ref.var` of the stack reads its instance");
        assert_eq!(var(&rpg, 0, &me, "nope"), None);
        set(&mut rpg, 0, me.clone(), "c", 10.0);
        assert_eq!(var(&rpg, 0, &me, "c"), Some(10.0), "a write is seen at once");
        tick_at(&mut rpg, &mut scripts, RegionCell::Exterior(0, 0));
        assert_eq!(c(&scripts, ring), Some(11.0), "and reaches the instance at the next tick");
        let other = stack_target(LocalKey::Stack { holder: PLAYER_HOLDER, stack: 999 });
        assert_eq!(var(&rpg, 0, &other, "c"), None, "another stack id names nothing");
    }

    #[test]
    fn an_item_leaving_the_inventory_drops_its_instance() {
        let (mut rpg, mut scripts) = new_game_with(carried());
        let ring = pick_up(&mut rpg);
        tick_at(&mut rpg, &mut scripts, RegionCell::Exterior(0, 0));
        assert_eq!(c(&scripts, ring), Some(1.0));
        let LocalKey::Stack { stack, .. } = ring else { unreachable!() };
        let player = sf_rpg::Holder::Player(sf_rpg::PlayerId::HOST);
        let stack = StackId(u32::try_from(stack).unwrap());
        crate::inventory::move_stack(&mut rpg.state, player, the_box(), stack, 1).expect("put back");
        let seen = global(&rpg, "dq_seen");
        tick_at(&mut rpg, &mut scripts, RegionCell::Exterior(0, 0));
        assert_eq!(c(&scripts, ring), None, "put in the box: no instance");
        assert_eq!(var(&rpg, 0, &stack_target(ring), "c"), None, "nor locals");
        assert_eq!(global(&rpg, "dq_seen"), seen, "and no run");
        let again = pick_up(&mut rpg);
        tick_at(&mut rpg, &mut scripts, RegionCell::Exterior(0, 0));
        assert_eq!(c(&scripts, again), Some(1.0), "taken again: a fresh instance");
    }

    #[test]
    fn a_save_keeps_a_carried_items_locals() {
        let (mut rpg, mut scripts) = new_game_with(carried());
        let ring = pick_up(&mut rpg);
        for _ in 0..3 {
            tick_at(&mut rpg, &mut scripts, RegionCell::Exterior(0, 0));
        }
        let bytes = postcard::to_allocvec(&scripts.snapshot(&rpg)).unwrap();
        let save: super::super::ScriptsSave = postcard::from_bytes(&bytes).unwrap();
        assert!(save.locals.locals.iter().any(|s| s.key == ring), "in the live locals");

        let (mut loaded, mut again) = new_game_with(carried());
        loaded.state.pc = rpg.state.pc.clone();
        again.load(&save, &mut loaded);
        assert!(again.local_load_reports().is_empty());
        assert_eq!(var(&loaded, 0, &stack_target(ring), "c"), Some(3.0), "readable before the first tick");
        tick_at(&mut loaded, &mut again, RegionCell::Exterior(0, 0));
        assert_eq!(c(&again, ring), Some(4.0), "the counter goes on");
    }
}
