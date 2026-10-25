//! Item stacks and inventories (rpg.md "Items, stacks, inventories").

use serde::{Deserialize, Serialize};
use sf_records::RecordId;
use smallvec::SmallVec;
use thiserror::Error;

/// A per-instance attribute value. Floats compare by bit pattern, so stack identity is
/// exact and survives a serde round trip.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AttrValue {
    Int(i64),
    Float(f32),
    Id(RecordId),
}

impl PartialEq for AttrValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(a), Self::Int(b)) => a == b,
            (Self::Float(a), Self::Float(b)) => a.to_bits() == b.to_bits(),
            (Self::Id(a), Self::Id(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for AttrValue {}

type AttrList = SmallVec<[(RecordId, AttrValue); 2]>;

/// Per-instance item data (condition, charge, soul, owner, ...), keyed by adapter-defined
/// names. Kept sorted by key, so equal sets compare equal.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(from = "AttrList", into = "AttrList")]
pub struct Attrs(AttrList);

impl Attrs {
    pub fn new() -> Self {
        Self::default()
    }

    /// Builder form of [`Attrs::set`].
    #[must_use]
    pub fn with(mut self, key: impl Into<RecordId>, value: AttrValue) -> Self {
        self.set(key, value);
        self
    }

    /// Sets or replaces `key`.
    pub fn set(&mut self, key: impl Into<RecordId>, value: AttrValue) {
        let key = key.into();
        match self.0.binary_search_by(|(k, _)| k.cmp(&key)) {
            Ok(i) => self.0[i].1 = value,
            Err(i) => self.0.insert(i, (key, value)),
        }
    }

    pub fn get(&self, key: &RecordId) -> Option<&AttrValue> {
        self.0.binary_search_by(|(k, _)| k.cmp(key)).ok().map(|i| &self.0[i].1)
    }

    pub fn remove(&mut self, key: &RecordId) -> Option<AttrValue> {
        self.0.binary_search_by(|(k, _)| k.cmp(key)).ok().map(|i| self.0.remove(i).1)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&RecordId, &AttrValue)> {
        self.0.iter().map(|(k, v)| (k, v))
    }
}

impl From<AttrList> for Attrs {
    /// Sorts by key; on duplicate keys the last value wins.
    fn from(list: AttrList) -> Self {
        let mut attrs = Self::default();
        for (k, v) in list {
            attrs.set(k, v);
        }
        attrs
    }
}

impl From<Attrs> for AttrList {
    fn from(attrs: Attrs) -> Self {
        attrs.0
    }
}

/// A stack's id, unique inside one inventory and stable across saves.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct StackId(pub u32);

/// `count` pieces of one item with identical per-instance data.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemStack {
    pub id: StackId,
    pub item: RecordId,
    pub count: u32,
    pub attrs: Attrs,
}

impl ItemStack {
    /// Stack identity: same item, same instance data.
    pub fn is_same_kind(&self, item: &RecordId, attrs: &Attrs) -> bool {
        self.item == *item && self.attrs == *attrs
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum InventoryError {
    #[error("count must be at least 1")]
    ZeroCount,
    #[error("stack count would overflow")]
    Overflow,
    #[error("not enough {item}: have {have}, want {want}")]
    NotEnough { item: RecordId, have: u64, want: u32 },
    #[error("no stack {0:?}")]
    NoStack(StackId),
    #[error("item can't be equipped")]
    Unequippable,
    #[error("stacks {0:?} and {1:?} hold different items or instance data")]
    NotSameKind(StackId, StackId),
    #[error("a stack can't merge into itself")]
    SameStack,
    #[error("stack {0:?} is equipped and can't merge")]
    Equipped(StackId),
    #[error("no stack {0:?} can merge into")]
    NoMergeTarget(StackId),
}

/// An ordered list of stacks (insertion order is display order; sorting is the UI's).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Inventory {
    stacks: Vec<ItemStack>,
    next_id: u32,
}

impl Inventory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn stacks(&self) -> &[ItemStack] {
        &self.stacks
    }

    pub fn is_empty(&self) -> bool {
        self.stacks.is_empty()
    }

    pub fn get(&self, id: StackId) -> Option<&ItemStack> {
        self.stacks.iter().find(|s| s.id == id)
    }

    fn index_of(&self, id: StackId) -> Result<usize, InventoryError> {
        self.stacks.iter().position(|s| s.id == id).ok_or(InventoryError::NoStack(id))
    }

    /// Total pieces of `item` across stacks.
    pub fn count(&self, item: &RecordId) -> u64 {
        self.stacks.iter().filter(|s| s.item == *item).map(|s| u64::from(s.count)).sum()
    }

    /// Adds `count` pieces, merging into the first identical stack.
    pub fn add(&mut self, item: RecordId, count: u32, attrs: Attrs) -> Result<StackId, InventoryError> {
        self.add_with(item, count, attrs, |_| true)
    }

    /// As [`Inventory::add`], merging only into stacks `can_merge` allows.
    pub fn add_with(
        &mut self,
        item: RecordId,
        count: u32,
        attrs: Attrs,
        can_merge: impl Fn(StackId) -> bool,
    ) -> Result<StackId, InventoryError> {
        if count == 0 {
            return Err(InventoryError::ZeroCount);
        }
        if let Some(stack) = self.stacks.iter_mut().find(|s| s.is_same_kind(&item, &attrs) && can_merge(s.id)) {
            stack.count = stack.count.checked_add(count).ok_or(InventoryError::Overflow)?;
            return Ok(stack.id);
        }
        let id = self.alloc_id()?;
        self.stacks.push(ItemStack { id, item, count, attrs });
        Ok(id)
    }

    fn alloc_id(&mut self) -> Result<StackId, InventoryError> {
        let id = StackId(self.next_id);
        self.next_id = self.next_id.checked_add(1).ok_or(InventoryError::Overflow)?;
        Ok(id)
    }

    /// Removes `count` pieces of `item`: stacks without attributes first, then the rest in
    /// order. Atomic: fails without change if there are fewer. Returns what was taken from
    /// each stack; emptied stacks are gone.
    pub fn remove(&mut self, item: &RecordId, count: u32) -> Result<Vec<(StackId, u32)>, InventoryError> {
        self.remove_ranked(item, count, |s| u8::from(!s.attrs.is_empty()))
    }

    /// As [`Inventory::remove`], taking from lower `rank` stacks first (ties in order).
    pub fn remove_ranked(
        &mut self,
        item: &RecordId,
        count: u32,
        rank: impl Fn(&ItemStack) -> u8,
    ) -> Result<Vec<(StackId, u32)>, InventoryError> {
        if count == 0 {
            return Err(InventoryError::ZeroCount);
        }
        let have = self.count(item);
        if have < u64::from(count) {
            return Err(InventoryError::NotEnough { item: item.clone(), have, want: count });
        }
        let mut order: Vec<(u8, usize)> =
            self.stacks.iter().enumerate().filter(|(_, s)| s.item == *item).map(|(i, s)| (rank(s), i)).collect();
        order.sort_unstable();
        let mut left = count;
        let mut taken = Vec::new();
        for (_, i) in order {
            if left == 0 {
                break;
            }
            let stack = &mut self.stacks[i];
            let n = stack.count.min(left);
            stack.count -= n;
            left -= n;
            taken.push((stack.id, n));
        }
        self.stacks.retain(|s| s.count > 0);
        Ok(taken)
    }

    /// Takes `count` pieces out of one stack, returning them as a detached stack (with the
    /// source's id when the whole stack is taken).
    pub fn take(&mut self, id: StackId, count: u32) -> Result<ItemStack, InventoryError> {
        let i = self.index_of(id)?;
        let stack = &mut self.stacks[i];
        if count == 0 {
            return Err(InventoryError::ZeroCount);
        }
        if count > stack.count {
            return Err(InventoryError::NotEnough { item: stack.item.clone(), have: u64::from(stack.count), want: count });
        }
        if count == stack.count {
            return Ok(self.stacks.remove(i));
        }
        stack.count -= count;
        Ok(ItemStack { id, item: stack.item.clone(), count, attrs: stack.attrs.clone() })
    }

    /// Splits `count` pieces off a stack into a new stack placed right after it.
    pub fn split(&mut self, id: StackId, count: u32) -> Result<StackId, InventoryError> {
        let i = self.index_of(id)?;
        if count == 0 {
            return Err(InventoryError::ZeroCount);
        }
        let have = self.stacks[i].count;
        if count >= have {
            return Err(InventoryError::NotEnough {
                item: self.stacks[i].item.clone(),
                have: u64::from(have),
                want: count.saturating_add(1),
            });
        }
        let new_id = self.alloc_id()?;
        let stack = &mut self.stacks[i];
        stack.count -= count;
        let piece = ItemStack { id: new_id, item: stack.item.clone(), count, attrs: stack.attrs.clone() };
        self.stacks.insert(i + 1, piece);
        Ok(new_id)
    }

    /// Merges stack `id` into the first other identical stack `can_merge` allows. Returns
    /// the stack that now holds the pieces.
    pub fn merge_back(&mut self, id: StackId, can_merge: impl Fn(StackId) -> bool) -> Result<StackId, InventoryError> {
        let i = self.index_of(id)?;
        let (item, attrs, count) = {
            let s = &self.stacks[i];
            (s.item.clone(), s.attrs.clone(), s.count)
        };
        let Some(j) = self.stacks.iter().position(|s| s.id != id && s.is_same_kind(&item, &attrs) && can_merge(s.id))
        else {
            return Ok(id);
        };
        let target = &mut self.stacks[j];
        target.count = target.count.checked_add(count).ok_or(InventoryError::Overflow)?;
        let target_id = target.id;
        self.stacks.remove(i);
        Ok(target_id)
    }

    /// Moves every piece of stack `from` into stack `into` (same item and instance data).
    /// `into` keeps its id and position; `from` is gone. Atomic.
    pub fn merge(&mut self, from: StackId, into: StackId) -> Result<StackId, InventoryError> {
        if from == into {
            return Err(InventoryError::SameStack);
        }
        let i = self.index_of(from)?;
        let j = self.index_of(into)?;
        let (src, dst) = (&self.stacks[i], &self.stacks[j]);
        if !dst.is_same_kind(&src.item, &src.attrs) {
            return Err(InventoryError::NotSameKind(from, into));
        }
        let total = dst.count.checked_add(src.count).ok_or(InventoryError::Overflow)?;
        self.stacks[j].count = total;
        self.stacks.remove(i);
        Ok(into)
    }
}
