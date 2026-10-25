//! Compare-with-equipped (Diablo/Elden Ring style before/after deltas, ui.md R5).
//!
//! The candidate is compared with what equipping it would take off: the stacks in the
//! slots its [`EquipPlacement`](sf_rpg::EquipPlacement) names, which is exactly what
//! [`Loadout::equip`] displaces. A two-handed weapon is compared with the weapon and the
//! shield together, so each stat's "current" value is the sum over the displaced stacks
//! (the change to the character's totals). Values are per piece.

use std::collections::BTreeSet;

use sf_rpg::{EquipRules, Loadout, RecordId, StackId};

use crate::catalog::{Better, ItemCatalog};
use crate::view::InventoryView;

/// How a stat changes if the candidate is equipped.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    Better,
    Worse,
    Same,
    /// It changes, but the stat has no better direction ([`Better::Neither`]).
    Changed,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StatDelta {
    pub stat: RecordId,
    /// The candidate's value (`None`: it doesn't have the stat).
    pub candidate: Option<f64>,
    /// Sum over the stacks it would replace (`None`: none has the stat).
    pub current: Option<f64>,
    /// `candidate - current`, missing values counting as 0.
    pub delta: f64,
    pub verdict: Verdict,
}

/// The compare panel for one stack.
#[derive(Clone, Debug, PartialEq)]
pub struct Comparison {
    pub candidate: StackId,
    /// The candidate is already equipped: deltas are all [`Verdict::Same`].
    pub worn: bool,
    /// Stacks equipping it would take off (empty slots, or the candidate itself when worn).
    pub replaces: Vec<StackId>,
    /// Stats in the catalog's order that either side has.
    pub stats: Vec<StatDelta>,
}

/// Deltas below this are "same" (float noise from adapter formulas).
const EPSILON: f64 = 1e-6;

impl InventoryView {
    /// Compares a stack with what it would replace. `None` when the stack isn't in the
    /// view or the rules can't equip it (no compare panel). Reads synced rows, so call it
    /// after [`InventoryView::sync`].
    pub fn compare(
        &self,
        stack: StackId,
        loadout: &Loadout,
        rules: &dyn EquipRules,
        catalog: &dyn ItemCatalog,
    ) -> Option<Comparison> {
        let row = self.row(stack)?;
        let source = loadout.inventory().get(stack)?;
        let placement = rules.placement(source)?;
        if placement.slots.is_empty() {
            return None;
        }
        let worn = placement
            .slots
            .iter()
            .all(|s| loadout.slot(*s) == Some(stack));
        let replaces: Vec<StackId> = if worn {
            vec![stack]
        } else {
            let set: BTreeSet<StackId> = placement
                .slots
                .iter()
                .filter_map(|s| loadout.slot(*s))
                .collect();
            set.into_iter().collect()
        };
        let current_rows: Vec<_> = replaces.iter().filter_map(|id| self.row(*id)).collect();
        let stats = catalog
            .stats()
            .iter()
            .filter_map(|def| {
                let candidate = row.info.stat(def.id.as_str());
                let current = current_rows
                    .iter()
                    .filter_map(|r| r.info.stat(def.id.as_str()))
                    .reduce(|a, b| a + b);
                if candidate.is_none() && current.is_none() {
                    return None;
                }
                let delta = candidate.unwrap_or(0.0) - current.unwrap_or(0.0);
                let verdict = if delta.abs() < EPSILON {
                    Verdict::Same
                } else {
                    match (def.better, delta > 0.0) {
                        (Better::Neither, _) => Verdict::Changed,
                        (Better::Higher, true) | (Better::Lower, false) => Verdict::Better,
                        (Better::Higher, false) | (Better::Lower, true) => Verdict::Worse,
                    }
                };
                Some(StatDelta {
                    stat: def.id.clone(),
                    candidate,
                    current,
                    delta,
                    verdict,
                })
            })
            .collect();
        Some(Comparison {
            candidate: stack,
            worn,
            replaces,
            stats,
        })
    }
}
