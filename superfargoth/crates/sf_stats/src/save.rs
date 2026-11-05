//! The saved form of a sheet (stats.md "Saves"): stored state by [`StatKey`], groups; no
//! modifiers (each source re-registers on load) and no caches. Wiring into the save sections
//! is slice S8.

use serde::{Deserialize, Serialize};
use sf_records::RecordId;

use crate::sheet::{Membership, Sheet};
use crate::table::{Slot, StatKey, StatTable};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValueSave {
    pub key: StatKey,
    pub base: f32,
    pub damage: f32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PoolSave {
    pub key: StatKey,
    pub base: f32,
    pub current: f32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CounterSave {
    pub key: StatKey,
    pub value: f32,
}

/// One sheet's saved state, in table order.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SheetSave {
    pub values: Vec<ValueSave>,
    pub pools: Vec<PoolSave>,
    pub counters: Vec<CounterSave>,
    pub groups: Vec<(RecordId, Membership)>,
}

/// Keys a load could not place: `dropped` were saved but are unknown to the table (or now of
/// another kind: their mod was removed or changed); `missing` are in the table but not in the
/// save (the adapter's init hook fills them).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LoadReport {
    pub dropped: Vec<StatKey>,
    pub missing: Vec<StatKey>,
}

impl Sheet {
    pub fn to_save(&self, table: &StatTable) -> SheetSave {
        let key = |i| table.def(i).key.clone();
        SheetSave {
            values: table
                .of_kind(0)
                .iter()
                .zip(&self.values)
                .map(|(&i, v)| ValueSave { key: key(i), base: v.base, damage: v.damage })
                .collect(),
            pools: table
                .of_kind(1)
                .iter()
                .zip(&self.pools)
                .map(|(&i, p)| PoolSave { key: key(i), base: p.base, current: p.current })
                .collect(),
            counters: table
                .of_kind(2)
                .iter()
                .zip(&self.counters)
                .map(|(&i, &value)| CounterSave { key: key(i), value })
                .collect(),
            groups: self.groups.iter().map(|(g, m)| (g.clone(), *m)).collect(),
        }
    }

    /// A sheet from its save, in building mode with no modifiers: the sources re-register their
    /// modifiers through `apply`, then [`Sheet::settle`] with [`crate::Settle::Loaded`].
    pub fn from_save(table: &StatTable, save: &SheetSave) -> (Self, LoadReport) {
        let mut sheet = Self::new(table);
        let mut report = LoadReport::default();
        let mut seen = vec![false; table.len()];
        // The slot of a saved key, marked seen when its kind matches `kind`; dropped otherwise.
        let mut place = |key: &StatKey, kind: usize, report: &mut LoadReport| {
            let found = table.idx(key).map(|i| (i, table.slot(i))).filter(|(_, s)| slot_kind(*s) == kind);
            if let Some((i, s)) = found {
                seen[usize::from(i.0)] = true;
                Some(s)
            } else {
                report.dropped.push(key.clone());
                None
            }
        };
        for v in &save.values {
            if let Some(Slot::Value(s)) = place(&v.key, 0, &mut report) {
                let st = &mut sheet.values[usize::from(s)];
                st.base = v.base;
                st.damage = v.damage;
            }
        }
        for p in &save.pools {
            if let Some(Slot::Pool(s)) = place(&p.key, 1, &mut report) {
                let st = &mut sheet.pools[usize::from(s)];
                st.base = p.base;
                st.current = p.current;
            }
        }
        for c in &save.counters {
            if let Some(Slot::Counter(s)) = place(&c.key, 2, &mut report) {
                sheet.counters[usize::from(s)] = c.value;
            }
        }
        sheet.groups = save.groups.iter().cloned().collect();
        for (i, def) in table.defs() {
            if !seen[usize::from(i.0)] && !matches!(table.slot(i), Slot::Derived(_)) {
                report.missing.push(def.key.clone());
            }
        }
        // Saved because it was changed (NPC overlays); the player's flag is not used.
        sheet.changed_since_init = true;
        (sheet, report)
    }
}

const fn slot_kind(s: Slot) -> usize {
    match s {
        Slot::Value(_) => 0,
        Slot::Pool(_) => 1,
        Slot::Counter(_) => 2,
        Slot::Derived(_) => 3,
    }
}
