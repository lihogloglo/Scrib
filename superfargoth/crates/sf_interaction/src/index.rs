//! The point index (interaction.md "Point index and focus"): every indexed slot of the resident
//! cells in a horizontal grid, queried by `sf_ai` (`find_slots`) and the player's focus. Points are
//! derived, never saved: the shell rebuilds them from placements on cell load and removes a cell's
//! placements on unload. A moving owner (mount, vehicle, carried chair) calls
//! [`PointIndex::move_placement`] each tick; a resting object once when it falls asleep.
//!
//! Y is up; buckets are square columns over X and Z. Maps are ordered, and results are sorted
//! (score descending, then `SlotRef`), so every peer and replay gets the same list.

use std::collections::{BTreeMap, BTreeSet};

use glam::Vec3;
use sf_world::PlacementId;
use smallvec::SmallVec;

use crate::claims::SlotRef;
use crate::def::{Frame, InteractableDef, SlotDef, SlotKind, TagFilter, TagId};

/// Grid bucket edge, m. Changes query cost only, never results.
// tunables: const (layout: grid bucket edge; results are identical for any value, only query cost moves)
const BUCKET_M: f32 = 4.0;

/// A slot kind without its data, for filters and the shell's dispatch.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum KindTag {
    Pose { family: TagId },
    Door,
    Lever,
    Ladder,
    Workstation,
    Light,
}

impl KindTag {
    #[must_use]
    pub fn of(kind: &SlotKind) -> Self {
        match kind {
            SlotKind::Pose { family } => Self::Pose { family: *family },
            SlotKind::Door(_) => Self::Door,
            SlotKind::Lever(_) => Self::Lever,
            SlotKind::Ladder(_) => Self::Ladder,
            SlotKind::Workstation(_) => Self::Workstation,
            SlotKind::Light(_) => Self::Light,
        }
    }
}

/// `object` ∘ `local`: a frame in the object's frame placed in the world.
#[must_use]
pub fn compose(object: &Frame, local: &Frame) -> Frame {
    Frame {
        translation: object.translation + object.rotation * local.translation,
        rotation: (object.rotation * local.rotation).normalize(),
    }
}

/// One indexed slot. `O` is the shell's owner handle (an entity); the core never reads it.
#[derive(Clone, Debug, PartialEq)]
pub struct IndexedSlot<O> {
    /// Slot frame in the object's frame (kept so a moving owner recomputes `world`).
    pub local: Frame,
    /// Slot frame in the world.
    pub world: Frame,
    pub kind: KindTag,
    /// Seat/bed/top height, m.
    pub band: f32,
    pub owner: O,
    /// What [`PointIndex::find_slots`]' filter tests: the slot's verb, and its pose family.
    pub tags: SmallVec<[TagId; 4]>,
    /// Who may use it (the def's `users`).
    pub users: TagFilter,
    /// The object's adverts: (need tag, value).
    pub adverts: SmallVec<[(TagId, f32); 2]>,
}

impl<O> IndexedSlot<O> {
    /// The index entry of `def`'s slot `slot` on an object at `object` (world).
    #[must_use]
    pub fn of(def: &InteractableDef, slot: &SlotDef, object: &Frame, owner: O) -> Self {
        let kind = KindTag::of(&slot.kind);
        let mut tags: SmallVec<[TagId; 4]> = SmallVec::new();
        tags.push(slot.verb.verb);
        if let KindTag::Pose { family } = kind {
            tags.push(family);
        }
        Self {
            local: slot.local,
            world: compose(object, &slot.local),
            kind,
            band: slot.band,
            owner,
            tags,
            users: slot.users.clone(),
            adverts: def.adverts.iter().copied().collect(),
        }
    }

    /// The advert value for `need`: `None` (any use) = 1, a need the object doesn't advertise = 0.
    #[must_use]
    pub fn advert(&self, need: Option<TagId>) -> f32 {
        need.map_or(1.0, |n| self.adverts.iter().filter(|(t, _)| *t == n).map(|(_, v)| *v).sum())
    }
}

/// Who is looking: their tags (tested against each slot's `users`) and the need they score by
/// (`None`: every slot scores 1 × falloff, the player's focus).
#[derive(Clone, Copy, Debug, Default)]
pub struct SlotUser<'a> {
    pub tags: &'a [TagId],
    pub need: Option<TagId>,
}

type Bucket = (i32, i32);

#[allow(clippy::cast_possible_truncation)]
fn bucket_of(p: Vec3) -> Bucket {
    ((p.x / BUCKET_M).floor() as i32, (p.z / BUCKET_M).floor() as i32)
}

/// The slots of the resident cells, bucketed for radius queries.
#[derive(Clone, Debug)]
pub struct PointIndex<O> {
    slots: BTreeMap<SlotRef, IndexedSlot<O>>,
    buckets: BTreeMap<Bucket, BTreeSet<SlotRef>>,
}

impl<O> Default for PointIndex<O> {
    fn default() -> Self {
        Self { slots: BTreeMap::new(), buckets: BTreeMap::new() }
    }
}

impl<O: Clone> PointIndex<O> {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.slots.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.slots.is_empty()
    }

    #[must_use]
    pub fn get(&self, slot: SlotRef) -> Option<&IndexedSlot<O>> {
        self.slots.get(&slot)
    }

    /// Indexes (or replaces) one slot.
    pub fn insert(&mut self, slot: SlotRef, entry: IndexedSlot<O>) {
        self.remove(slot);
        self.buckets.entry(bucket_of(entry.world.translation)).or_default().insert(slot);
        self.slots.insert(slot, entry);
    }

    /// Indexes every slot of `def` placed at `object` (world), replacing the placement's old slots.
    /// Slots the adapter switched off are the caller's to skip: pass `enabled(i) == false`.
    pub fn insert_placement(
        &mut self,
        placement: PlacementId,
        def: &InteractableDef,
        object: &Frame,
        owner: &O,
        enabled: impl Fn(u8) -> bool,
    ) {
        self.remove_placement(placement);
        for (i, slot_def) in def.slots.iter().enumerate() {
            let Ok(i) = u8::try_from(i) else { break };
            if enabled(i) {
                self.insert(SlotRef { placement, slot: i }, IndexedSlot::of(def, slot_def, object, owner.clone()));
            }
        }
    }

    /// Drops one slot; returns its entry.
    pub fn remove(&mut self, slot: SlotRef) -> Option<IndexedSlot<O>> {
        let entry = self.slots.remove(&slot)?;
        let b = bucket_of(entry.world.translation);
        if let Some(set) = self.buckets.get_mut(&b) {
            set.remove(&slot);
            if set.is_empty() {
                self.buckets.remove(&b);
            }
        }
        Some(entry)
    }

    /// Drops every slot of `placement` (destroyed, unloaded with its cell); returns how many.
    pub fn remove_placement(&mut self, placement: PlacementId) -> usize {
        let refs: SmallVec<[SlotRef; 8]> = self.placement_slots(placement).collect();
        for s in &refs {
            self.remove(*s);
        }
        refs.len()
    }

    /// A moved owner: recomputes the world frame of each of `placement`'s slots from `object`
    /// (world) and rebuckets it. Returns how many slots moved.
    pub fn move_placement(&mut self, placement: PlacementId, object: &Frame) -> usize {
        let refs: SmallVec<[SlotRef; 8]> = self.placement_slots(placement).collect();
        for s in &refs {
            let Some(entry) = self.slots.get_mut(s) else { continue };
            let old = bucket_of(entry.world.translation);
            entry.world = compose(object, &entry.local);
            let new = bucket_of(entry.world.translation);
            if old != new {
                if let Some(set) = self.buckets.get_mut(&old) {
                    set.remove(s);
                    if set.is_empty() {
                        self.buckets.remove(&old);
                    }
                }
                self.buckets.entry(new).or_default().insert(*s);
            }
        }
        refs.len()
    }

    fn placement_slots(&self, placement: PlacementId) -> impl Iterator<Item = SlotRef> + '_ {
        let lo = SlotRef { placement, slot: 0 };
        let hi = SlotRef { placement, slot: u8::MAX };
        self.slots.range(lo..=hi).map(|(s, _)| *s)
    }

    /// Slots within `radius` (m, 3D) of `near` whose tags pass `filter` and whose `users` pass the
    /// user's tags, scored `advert(user.need) × falloff(distance_m)` (the adapter's curve). Slots
    /// scoring 0 or less (or NaN) are dropped. Sorted by score descending, then `SlotRef`.
    pub fn find_slots(
        &self,
        near: Vec3,
        radius: f32,
        filter: &TagFilter,
        user: SlotUser<'_>,
        falloff: impl Fn(f32) -> f32,
    ) -> std::vec::IntoIter<(SlotRef, f32)> {
        let mut out: Vec<(SlotRef, f32)> = Vec::new();
        if radius.is_finite() && radius >= 0.0 {
            let (x0, z0) = bucket_of(near - Vec3::splat(radius));
            let (x1, z1) = bucket_of(near + Vec3::splat(radius));
            for (_, set) in self.buckets.range((x0, z0)..=(x1, z1)).filter(|((_, z), _)| (z0..=z1).contains(z)) {
                for s in set {
                    let e = &self.slots[s];
                    let d = e.world.translation.distance(near);
                    if d > radius || !filter.matches(&e.tags) || !e.users.matches(user.tags) {
                        continue;
                    }
                    let score = e.advert(user.need) * falloff(d);
                    if score > 0.0 {
                        out.push((*s, score));
                    }
                }
            }
        }
        out.sort_by(|a, b| b.1.total_cmp(&a.1).then(a.0.cmp(&b.0)));
        out.into_iter()
    }
}
