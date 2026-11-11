//! Effect sources and spell books (effects.md "Effect lists and sources", "Spell book"):
//! [`SourceDef`]s in a [`SourceTable`] (built by content from its records), and the per-character
//! [`SpellBook`], the only owner of which abilities exist. Changed only by
//! [`crate::apply_book`] (`Learn`, `Forget`, `Select`, `Cast`, cures of afflictions).

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use sf_ids::GameInstant;
use sf_records::RecordId;
use sf_stats::StatKey;
use smallvec::SmallVec;

use crate::active::{SourceClass, SourceRef};
use crate::def::Tag;
use crate::op::EffectList;

/// The world clock in game minutes since the game started: what power recharges count. A
/// view of the host's one clock (`sf_ids::GameInstant`, clock.md), rounded down; the same
/// minutes as `sf_rpg::GameTime::total_minutes`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct GameMinutes(pub u64);

impl From<GameInstant> for GameMinutes {
    fn from(now: GameInstant) -> Self {
        Self(now.minutes())
    }
}

impl GameMinutes {
    /// One game day (24 game hours).
    pub const DAY: Self = Self(1440);

    /// `days` game days.
    pub fn days(days: u32) -> Self {
        Self(u64::from(days) * Self::DAY.0)
    }
}

/// Per-source switches.
#[allow(clippy::struct_excessive_bools, reason = "independent record flags")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceFlags {
    /// Casting never fails: the chance is 1 whatever the cast model says.
    pub always_succeeds: bool,
    /// The cost is computed from the entries (content sets `cost: None` too).
    pub auto_cost: bool,
    /// A new character may start with it (the host's character creation reads it).
    pub starts_known: bool,
}

/// One effect source (spell, power, ability, affliction, potion, enchantment), built by content
/// from its records.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SourceDef {
    /// The record.
    pub id: RecordId,
    /// Lifetime, layer and default stacking.
    pub class: SourceClass,
    /// What it lands.
    pub effects: EffectList,
    /// Fixed cost per cast; `None` = computed by the cast model from the entries.
    pub cost: Option<f32>,
    pub flags: SourceFlags,
    /// Cure groups (`CureFilter::SourceTag`).
    pub tags: SmallVec<[Tag; 2]>,
    /// The skill that lowers the cost (the source's school), resolved against the stat table at
    /// the cast; `None` = no skill.
    pub skill: Option<StatKey>,
    /// Cooldown after a successful cast, in fixed ticks; 0 = none.
    pub cooldown: u32,
    /// Powers: usable again this long after the last use, on the world clock (24 h powers:
    /// `Some(GameMinutes::DAY)`); `None` = no recharge.
    pub recharge: Option<GameMinutes>,
}

impl SourceDef {
    /// A source with no cost, flags, tags, skill, cooldown or recharge.
    pub fn new(id: &str, class: SourceClass, effects: EffectList) -> Self {
        Self {
            id: RecordId::new(id),
            class,
            effects,
            cost: None,
            flags: SourceFlags::default(),
            tags: SmallVec::new(),
            skill: None,
            cooldown: 0,
            recharge: None,
        }
    }

    /// The landing's source reference (no item).
    pub fn source_ref(&self) -> SourceRef {
        SourceRef { record: self.id.clone(), item: None }
    }

    /// A book may hold it: spells, powers, abilities, afflictions.
    pub fn is_book_source(&self) -> bool {
        matches!(self.class, SourceClass::Cast | SourceClass::Power | SourceClass::Innate | SourceClass::Affliction)
    }

    /// The `Cast` op may cast it: spells and powers.
    pub fn is_castable(&self) -> bool {
        matches!(self.class, SourceClass::Cast | SourceClass::Power)
    }

    /// Its instance lives while it is in the book: abilities and afflictions.
    pub fn lives_in_book(&self) -> bool {
        matches!(self.class, SourceClass::Innate | SourceClass::Affliction)
    }
}

/// A source table that can't be built.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum SourceTableError {
    #[error("duplicate effect source {0:?}")]
    Duplicate(RecordId),
}

/// Every source content defines, looked up by record id.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SourceTable {
    defs: Vec<SourceDef>,
    index: BTreeMap<RecordId, usize>,
}

impl SourceTable {
    /// The table of `defs` (definition order kept); a record id twice is an error.
    pub fn new(defs: Vec<SourceDef>) -> Result<Self, SourceTableError> {
        let mut index = BTreeMap::new();
        for (i, def) in defs.iter().enumerate() {
            if index.insert(def.id.clone(), i).is_some() {
                return Err(SourceTableError::Duplicate(def.id.clone()));
            }
        }
        Ok(Self { defs, index })
    }

    /// The source of `id`.
    pub fn get(&self, id: &RecordId) -> Option<&SourceDef> {
        self.index.get(id).map(|&i| &self.defs[i])
    }

    /// All sources in definition order.
    pub fn defs(&self) -> &[SourceDef] {
        &self.defs
    }

    /// How many sources.
    pub fn len(&self) -> usize {
        self.defs.len()
    }

    /// No sources.
    pub fn is_empty(&self) -> bool {
        self.defs.is_empty()
    }
}

/// Why a book op was refused (reported as `EffectEvent::BookRejected`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BookRejection {
    /// No `SourceDef` for the record.
    UnknownSource,
    /// The class is not one a book holds (`Learn`) or casts (`Select`).
    NotABookSource,
    /// `Select` of a source the book does not hold.
    NotKnown,
}

/// One character's spells, powers, abilities and afflictions (effects.md "Spell book"). Innate
/// and affliction sources here have exactly one instance in the character's active set: the op
/// that adds or removes them starts or ends it.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SpellBook {
    /// Insertion order.
    pub(crate) known: Vec<RecordId>,
    pub(crate) selected: Option<RecordId>,
    /// The world time of each power's last use.
    pub(crate) power_used: BTreeMap<RecordId, GameMinutes>,
    /// The fixed tick each source is castable again.
    pub(crate) cooldowns: BTreeMap<RecordId, u32>,
}

impl SpellBook {
    /// An empty book.
    pub fn new() -> Self {
        Self::default()
    }

    /// Known sources in the order they were learned.
    pub fn known(&self) -> &[RecordId] {
        &self.known
    }

    /// The book holds `id`.
    pub fn knows(&self, id: &RecordId) -> bool {
        self.known.contains(id)
    }

    /// The source the cast input uses.
    pub fn selected(&self) -> Option<&RecordId> {
        self.selected.as_ref()
    }

    /// When a power was last used.
    pub fn power_used(&self, id: &RecordId) -> Option<GameMinutes> {
        self.power_used.get(id).copied()
    }

    /// The fixed tick `id` is castable again, if a cooldown was started.
    pub fn ready_at(&self, id: &RecordId) -> Option<u32> {
        self.cooldowns.get(id).copied()
    }

    /// A power with `recharge` is still recharging at `now`.
    pub fn recharging(&self, id: &RecordId, recharge: Option<GameMinutes>, now: GameMinutes) -> bool {
        match (recharge, self.power_used(id)) {
            (Some(span), Some(used)) => now.0 < used.0.saturating_add(span.0),
            _ => false,
        }
    }

    /// Adds `id`; false if already known.
    pub(crate) fn add(&mut self, id: &RecordId) -> bool {
        if self.knows(id) {
            return false;
        }
        self.known.push(id.clone());
        true
    }

    /// Removes `id` (and its selection); false if not known. Recharges and cooldowns are kept:
    /// re-learning does not reset them.
    pub(crate) fn remove(&mut self, id: &RecordId) -> bool {
        let Some(pos) = self.known.iter().position(|k| k == id) else { return false };
        self.known.remove(pos);
        if self.selected.as_ref() == Some(id) {
            self.selected = None;
        }
        true
    }
}
