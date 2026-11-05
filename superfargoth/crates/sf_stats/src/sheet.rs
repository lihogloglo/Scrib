//! One character's stats (stats.md "Sheet"): stored state, source-tracked modifiers, cached
//! modified values, and the read-only view the rules models get.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use sf_records::RecordId;
use smallvec::SmallVec;

use crate::table::{Rebase, Role, Slot, StatIdx, StatKind, StatTable};

/// A value stat's state; `modified` and `eff_base` are caches.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ValueStat {
    pub base: f32,
    pub damage: f32,
    pub(crate) eff_base: f32,
    pub(crate) modified: f32,
}

/// A pool's state; `max` is a cache (the last recomputed maximum, used for [`Rebase`]).
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PoolStat {
    pub base: f32,
    pub current: f32,
    pub(crate) eff_base: f32,
    pub(crate) max: f32,
}

/// Which bucket a modifier adds to (Creation's permanent and temporary actor-value buckets).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Layer {
    /// Counts as base (`effective_base`): abilities, birthsigns, racial bonuses.
    Permanent,
    /// On top of the effective base: fortify effects, equipment.
    Temporary,
}

/// How a modifier combines: `Add` sums into its layer; `Mul` sums into one `1 + Σmul` factor
/// (Temporary layer only: a `Mul` always applies to the whole modified value).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ModOp {
    Add,
    Mul,
}

/// What added a modifier; removing the source removes all its modifiers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SourceKind {
    Effect,
    Equipment,
    Ability,
    Script,
    Rules,
    Debug,
}

/// A stable modifier source (host-allocated ids, net rule 7; never an `Entity`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SourceId {
    pub kind: SourceKind,
    pub id: u64,
}

/// One modifier on one stat from one source.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Modifier {
    pub stat: StatIdx,
    pub layer: Layer,
    pub op: ModOp,
    pub value: f32,
    pub source: SourceId,
}

/// A character's standing in a group (faction, guild, hold). `rank` is `None` when not a
/// member: reputation and the expelled flag can exist without membership.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Membership {
    pub rank: Option<i32>,
    pub reputation: i32,
    pub expelled: bool,
}

/// How a sheet settles after it was built from ops or loaded (stats.md "Saves").
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Settle {
    /// A new character: every pool's current is set to its max; the sheet counts as untouched.
    NewCharacter,
    /// A loaded sheet whose modifier sources re-registered: maxima are recomputed but currents
    /// are kept as saved (a fortified current above the unfortified max survives the load).
    Loaded,
}

/// One character's stats. Every change goes through [`crate::apply`] or [`crate::tick`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sheet {
    pub(crate) values: Vec<ValueStat>,
    pub(crate) pools: Vec<PoolStat>,
    pub(crate) counters: Vec<f32>,
    pub(crate) derived: Vec<f32>,
    pub(crate) mods: SmallVec<[Modifier; 8]>,
    pub(crate) groups: BTreeMap<RecordId, Membership>,
    /// Some input of the caches changed since the last recompute (whole sheet: the derived
    /// model declares no dependencies, so pools and derived stats always recompute together).
    pub(crate) dirty: bool,
    /// Building (new or loading): maxima move without [`Rebase`] until [`Sheet::settle`].
    pub(crate) building: bool,
    /// The derivation inputs of the last recompute (seed excluded): a change recomputes.
    pub(crate) last_inputs: Option<crate::Inputs>,
    /// Saves store only sheets changed since their init (NPCs re-initialise from records).
    pub(crate) changed_since_init: bool,
}

impl Sheet {
    /// An empty sheet for `table`, in building mode: apply the init ops, then [`Sheet::settle`].
    /// Counters start at 0 clamped to their range.
    pub fn new(table: &StatTable) -> Self {
        let counters = table
            .of_kind(2)
            .iter()
            .map(|&i| match table.def(i).kind {
                StatKind::Counter { min, max } => 0f32.clamp(min, max),
                _ => 0.0,
            })
            .collect();
        Self {
            values: vec![ValueStat::default(); table.count(0)],
            pools: vec![PoolStat::default(); table.count(1)],
            counters,
            derived: vec![0.0; table.count(3)],
            mods: SmallVec::new(),
            groups: BTreeMap::new(),
            dirty: true,
            building: true,
            last_inputs: None,
            changed_since_init: false,
        }
    }

    /// Ends building: recomputes without rebasing, then fills pools (new character) or keeps the
    /// saved currents (load).
    pub fn settle(&mut self, table: &StatTable, rules: &crate::Rules, inputs: &crate::Inputs, how: Settle) {
        self.building = true;
        self.dirty = true;
        self.recompute(table, rules, inputs);
        self.building = false;
        if how == Settle::NewCharacter {
            for p in &mut self.pools {
                p.current = p.max;
            }
            self.changed_since_init = false;
        }
    }

    pub fn is_building(&self) -> bool {
        self.building
    }

    pub fn changed_since_init(&self) -> bool {
        self.changed_since_init
    }

    pub fn modifiers(&self) -> &[Modifier] {
        &self.mods
    }

    pub fn view<'a>(&'a self, table: &'a StatTable) -> SheetRead<'a> {
        SheetRead { table, sheet: self }
    }

    /// Recomputes the caches when dirty or when the inputs changed: values, then pools (base from
    /// `DerivedModel::pool_base`, max, [`Rebase`] unless building), then derived stats in table order.
    pub(crate) fn recompute(&mut self, table: &StatTable, rules: &crate::Rules, inputs: &crate::Inputs) {
        let inputs_changed = self.last_inputs.as_ref().is_none_or(|l| !l.same_derivation(inputs));
        if !self.dirty && !inputs_changed {
            return;
        }
        self.dirty = false;
        if inputs_changed {
            self.last_inputs = Some(inputs.for_derivation());
        }
        for (slot, &idx) in table.of_kind(0).iter().enumerate() {
            let (min, cap) = match table.def(idx).kind {
                StatKind::Value { min, cap } => (min, cap),
                _ => (f32::MIN, None),
            };
            let sums = self.sums(idx);
            let v = &mut self.values[slot];
            v.eff_base = v.base + sums.permanent;
            let raw = (v.eff_base - v.damage + sums.temp) * (1.0 + sums.mul);
            let lo = raw.max(min);
            v.modified = cap.map_or(lo, |c| lo.min(c));
        }
        for (slot, &idx) in table.of_kind(1).iter().enumerate() {
            if let Some(b) = rules.derived.pool_base(&self.view(table), idx, inputs) {
                self.pools[slot].base = b;
            }
            let rebase = match table.def(idx).kind {
                StatKind::Pool { rebase, .. } => rebase,
                _ => Rebase::KeepCurrent,
            };
            let sums = self.sums(idx);
            let building = self.building;
            let p = &mut self.pools[slot];
            p.eff_base = p.base + sums.permanent;
            let max = ((p.eff_base + sums.temp) * (1.0 + sums.mul)).max(0.0);
            if max.to_bits() != p.max.to_bits() && !building {
                p.current = rebased(rebase, p.current, p.max, max);
            }
            p.max = max;
        }
        for (slot, &idx) in table.of_kind(3).iter().enumerate() {
            self.derived[slot] = rules.derived.derived(&self.view(table), idx, inputs);
        }
    }

    pub(crate) fn sums(&self, idx: StatIdx) -> Sums {
        let mut s = Sums::default();
        for m in self.mods.iter().filter(|m| m.stat == idx) {
            match (m.layer, m.op) {
                (Layer::Permanent, ModOp::Add) => s.permanent += m.value,
                (Layer::Temporary, ModOp::Add) => s.temp += m.value,
                (_, ModOp::Mul) => s.mul += m.value,
            }
        }
        s
    }
}

#[derive(Default)]
pub(crate) struct Sums {
    pub(crate) permanent: f32,
    pub(crate) temp: f32,
    pub(crate) mul: f32,
}

fn rebased(rebase: Rebase, current: f32, old_max: f32, new_max: f32) -> f32 {
    match rebase {
        // current / max kept as the max moves; a 0 max before leaves 0 (no ratio to keep).
        Rebase::KeepRatio => {
            if old_max > 0.0 {
                current * new_max / old_max
            } else {
                0.0
            }
        }
        Rebase::KeepCurrent => {
            if new_max < old_max && current > new_max {
                new_max
            } else {
                current
            }
        }
        Rebase::KeepDeficit => current + (new_max - old_max),
    }
}

/// The read-only view of a sheet with its table: what the rules models and consumers read.
/// Caches are current whenever a model is called (`apply` recomputes first).
#[derive(Clone, Copy)]
pub struct SheetRead<'a> {
    table: &'a StatTable,
    sheet: &'a Sheet,
}

impl<'a> SheetRead<'a> {
    pub fn table(&self) -> &'a StatTable {
        self.table
    }

    pub fn sheet(&self) -> &'a Sheet {
        self.sheet
    }

    fn value(&self, idx: StatIdx) -> Option<&'a ValueStat> {
        match self.table.try_slot(idx)? {
            Slot::Value(s) => self.sheet.values.get(usize::from(s)),
            _ => None,
        }
    }

    fn pool(&self, idx: StatIdx) -> Option<&'a PoolStat> {
        match self.table.try_slot(idx)? {
            Slot::Pool(s) => self.sheet.pools.get(usize::from(s)),
            _ => None,
        }
    }

    /// Stored base: a value's or pool's base, a counter's value; 0 for a derived stat.
    pub fn base(&self, idx: StatIdx) -> f32 {
        match self.table.try_slot(idx) {
            Some(Slot::Value(s)) => self.sheet.values[usize::from(s)].base,
            Some(Slot::Pool(s)) => self.sheet.pools[usize::from(s)].base,
            Some(Slot::Counter(s)) => self.sheet.counters[usize::from(s)],
            _ => 0.0,
        }
    }

    /// Base plus the Permanent layer (values and pools); the counter or derived value otherwise.
    pub fn effective_base(&self, idx: StatIdx) -> f32 {
        match self.table.try_slot(idx) {
            Some(Slot::Value(s)) => self.sheet.values[usize::from(s)].eff_base,
            Some(Slot::Pool(s)) => self.sheet.pools[usize::from(s)].eff_base,
            _ => self.modified(idx),
        }
    }

    /// The value consumers use: a value's modified value, a pool's max, a counter's value, a
    /// derived stat's value.
    pub fn modified(&self, idx: StatIdx) -> f32 {
        match self.table.try_slot(idx) {
            Some(Slot::Value(s)) => self.sheet.values[usize::from(s)].modified,
            Some(Slot::Pool(s)) => self.sheet.pools[usize::from(s)].max,
            Some(Slot::Counter(s)) => self.sheet.counters[usize::from(s)],
            Some(Slot::Derived(s)) => self.sheet.derived[usize::from(s)],
            None => 0.0,
        }
    }

    pub fn damage(&self, idx: StatIdx) -> f32 {
        self.value(idx).map_or(0.0, |v| v.damage)
    }

    pub fn current(&self, idx: StatIdx) -> f32 {
        self.pool(idx).map_or(0.0, |p| p.current)
    }

    pub fn max(&self, idx: StatIdx) -> f32 {
        self.pool(idx).map_or(0.0, |p| p.max)
    }

    /// current / max, 0 for an empty pool.
    pub fn ratio(&self, idx: StatIdx) -> f32 {
        self.pool(idx).map_or(0.0, |p| if p.max > 0.0 { p.current / p.max } else { 0.0 })
    }

    pub fn role(&self, role: Role) -> Option<StatIdx> {
        self.table.role(role)
    }

    pub fn membership(&self, group: &RecordId) -> Option<&'a Membership> {
        self.sheet.groups.get(group)
    }

    pub fn groups(&self) -> impl Iterator<Item = (&'a RecordId, &'a Membership)> {
        self.sheet.groups.iter()
    }

    /// The modifiers on one stat (stats-screen tooltips).
    pub fn modifiers_on(&self, idx: StatIdx) -> impl Iterator<Item = &'a Modifier> {
        self.sheet.mods.iter().filter(move |m| m.stat == idx)
    }
}
