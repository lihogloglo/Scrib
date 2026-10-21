//! Data-driven equipment slots and the inventory + equipment pair (rpg.md "Equipment").

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use sf_records::RecordId;
use smallvec::SmallVec;
use thiserror::Error;

use crate::item::{Attrs, Inventory, InventoryError, ItemStack, StackId};

/// Index into a [`SlotTable`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SlotId(pub u8);

/// Most slots a table may define.
pub const MAX_SLOTS: usize = 64;

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum SlotTableError {
    #[error("more than {MAX_SLOTS} slots")]
    TooMany,
    #[error("duplicate slot {0}")]
    Duplicate(RecordId),
}

/// The game's slot set, in display order (adapter data).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SlotTable {
    names: Vec<RecordId>,
}

impl SlotTable {
    pub fn new(names: impl IntoIterator<Item = impl Into<RecordId>>) -> Result<Self, SlotTableError> {
        let mut table = Self::default();
        for name in names {
            let name = name.into();
            if table.names.contains(&name) {
                return Err(SlotTableError::Duplicate(name));
            }
            if table.names.len() == MAX_SLOTS {
                return Err(SlotTableError::TooMany);
            }
            table.names.push(name);
        }
        Ok(table)
    }

    pub fn id(&self, name: &str) -> Option<SlotId> {
        let name = RecordId::new(name);
        self.names.iter().position(|n| *n == name).and_then(|i| u8::try_from(i).ok()).map(SlotId)
    }

    pub fn name(&self, slot: SlotId) -> Option<&RecordId> {
        self.names.get(usize::from(slot.0))
    }

    pub fn len(&self) -> usize {
        self.names.len()
    }

    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }
}

/// Where an item goes when equipped.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EquipPlacement {
    /// Every slot the item occupies (a two-handed weapon names two).
    pub slots: SmallVec<[SlotId; 2]>,
    /// The whole stack equips (ammunition); otherwise one piece is split off.
    pub whole_stack: bool,
}

/// Adapter hook: the game's equip rules. Requirements (skills, race) are checked before.
pub trait EquipRules {
    fn placement(&self, stack: &ItemStack) -> Option<EquipPlacement>;
}

/// What an equip did, for the adapter's equip events.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EquipChange {
    pub equipped: StackId,
    pub item: RecordId,
    /// Items taken off to make room.
    pub displaced: Vec<RecordId>,
}

/// A character's items: inventory plus what is equipped, kept consistent.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Loadout {
    inventory: Inventory,
    slots: BTreeMap<SlotId, StackId>,
    /// Split-equipped stacks (one piece taken off a stack): they never take merges.
    locked: BTreeSet<StackId>,
}

impl Loadout {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn inventory(&self) -> &Inventory {
        &self.inventory
    }

    pub fn slot(&self, slot: SlotId) -> Option<StackId> {
        self.slots.get(&slot).copied()
    }

    pub fn equipped(&self) -> impl Iterator<Item = (SlotId, StackId)> + '_ {
        self.slots.iter().map(|(s, id)| (*s, *id))
    }

    pub fn is_equipped(&self, stack: StackId) -> bool {
        self.slots.values().any(|s| *s == stack)
    }

    pub fn count(&self, item: &RecordId) -> u64 {
        self.inventory.count(item)
    }

    /// Adds items; never merges into a split-equipped stack.
    pub fn add(&mut self, item: RecordId, count: u32, attrs: Attrs) -> Result<StackId, InventoryError> {
        let locked = &self.locked;
        self.inventory.add_with(item, count, attrs, |id| !locked.contains(&id))
    }

    /// Removes by item id: unequipped plain stacks first, then unequipped with attributes,
    /// then equipped ones (which are unequipped if emptied).
    pub fn remove(&mut self, item: &RecordId, count: u32) -> Result<Vec<(StackId, u32)>, InventoryError> {
        let slots = &self.slots;
        let taken = self.inventory.remove_ranked(item, count, |s| {
            let equipped = slots.values().any(|id| *id == s.id);
            u8::from(equipped) * 2 + u8::from(!s.attrs.is_empty())
        })?;
        self.drop_missing();
        Ok(taken)
    }

    /// Takes pieces out of one stack (drop, give, sell).
    pub fn take(&mut self, stack: StackId, count: u32) -> Result<ItemStack, InventoryError> {
        let taken = self.inventory.take(stack, count)?;
        self.drop_missing();
        Ok(taken)
    }

    fn drop_missing(&mut self) {
        let inv = &self.inventory;
        self.slots.retain(|_, id| inv.get(*id).is_some());
        self.locked.retain(|id| inv.get(*id).is_some());
    }

    /// Equips a stack per the rules, displacing whatever occupies its slots.
    pub fn equip(&mut self, stack: StackId, rules: &dyn EquipRules) -> Result<EquipChange, InventoryError> {
        let source = self.inventory.get(stack).ok_or(InventoryError::NoStack(stack))?;
        let placement = rules.placement(source).ok_or(InventoryError::Unequippable)?;
        if placement.slots.is_empty() {
            return Err(InventoryError::Unequippable);
        }
        let item = source.item.clone();
        let count = source.count;
        // Already worn exactly there: nothing to do.
        if placement.slots.iter().all(|s| self.slots.get(s) == Some(&stack))
            && self.slots.values().filter(|id| **id == stack).count() == placement.slots.len()
        {
            return Ok(EquipChange { equipped: stack, item, displaced: Vec::new() });
        }
        let mut displaced = Vec::new();
        if self.is_equipped(stack) {
            self.unequip_stack(stack);
        }
        let occupants: BTreeSet<StackId> = placement.slots.iter().filter_map(|s| self.slots.get(s).copied()).collect();
        for other in occupants {
            if let Some(s) = self.inventory.get(other) {
                displaced.push(s.item.clone());
            }
            self.unequip_stack(other);
        }
        // The stack may have merged back into another while unequipping itself.
        let stack = self.find_unlocked_stack(stack, &item);
        let equipped = if !placement.whole_stack && count > 1 {
            self.inventory.split(stack, 1)?
        } else {
            stack
        };
        for slot in &placement.slots {
            self.slots.insert(*slot, equipped);
        }
        if !placement.whole_stack {
            self.locked.insert(equipped);
        }
        Ok(EquipChange { equipped, item, displaced })
    }

    fn find_unlocked_stack(&self, stack: StackId, item: &RecordId) -> StackId {
        if self.inventory.get(stack).is_some() {
            return stack;
        }
        self.inventory
            .stacks()
            .iter()
            .find(|s| s.item == *item && !self.locked.contains(&s.id) && !self.is_equipped(s.id))
            .map_or(stack, |s| s.id)
    }

    /// Takes off whatever is in `slot` (all slots it occupies). Returns its item.
    pub fn unequip(&mut self, slot: SlotId) -> Option<RecordId> {
        let stack = self.slots.get(&slot).copied()?;
        let item = self.inventory.get(stack).map(|s| s.item.clone());
        self.unequip_stack(stack);
        item
    }

    fn unequip_stack(&mut self, stack: StackId) {
        self.slots.retain(|_, id| *id != stack);
        if self.locked.remove(&stack) {
            let slots = &self.slots;
            let locked = &self.locked;
            // A split piece rejoins an identical, not split-equipped stack.
            let _ = self.inventory.merge_back(stack, |id| !locked.contains(&id) && !slots.values().any(|s| *s == id));
        }
    }
}
