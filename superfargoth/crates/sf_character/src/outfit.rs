//! Slot resolution: base parts plus worn items into the visible part
//! list and its outfit key.
//!
//! Start from the base parts (skin, then head, then hair; a later base part
//! in the same slot replaces an earlier one). Then take the worn items by
//! ascending layer, ties by equip order. Each item first clears the slots it
//! `hides`, then places its parts, replacing what is in their slots. So an
//! item hides what lies *under* it (a robe hides skin legs and trousers);
//! an item on a higher layer can still show in a slot a lower one hid.
//!
//! The **outfit key** hashes the visible (slot name, part id) pairs in
//! slot-name order: equal for equal looks however they were reached, and
//! stable across sessions (slot ids are not used). It keys the
//! merged-mesh cache together with the skeleton and LOD.

use sf_records::{RecordDb, RecordId};

use crate::records::{BodyPart, CharacterRecipe, PartRef, Wearable};
use crate::slots::{SlotId, SlotRegistry};
use crate::source::CharacterError;

/// Which view a character is resolved for.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum View {
    #[default]
    ThirdPerson,
    /// The first-person view model (arms rig, [`Wearable::first_person`] parts).
    FirstPerson,
}

/// Hash of a visible part list (see the module docs).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OutfitKey(pub u64);

/// The resolved look: visible parts in slot order, and its key.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Outfit {
    pub parts: Vec<(SlotId, PartRef)>,
    pub key: OutfitKey,
}

/// Looks up part and wearable records. Implemented for [`RecordDb`];
/// tests and tools can supply maps.
pub trait PartLookup {
    fn part(&self, id: &RecordId) -> Option<&BodyPart>;
    fn wearable(&self, id: &RecordId) -> Option<&Wearable>;
}

impl PartLookup for RecordDb {
    fn part(&self, id: &RecordId) -> Option<&BodyPart> {
        self.get::<BodyPart>(id)
    }

    fn wearable(&self, id: &RecordId) -> Option<&Wearable> {
        self.get::<Wearable>(id)
    }
}

/// Resolves base parts (already slotted) and worn items (in equip order).
/// Pure: no record lookups.
pub fn resolve(slots: &SlotRegistry, base: &[(SlotId, PartRef)], worn: &[&Wearable], view: View) -> Outfit {
    let mut visible: Vec<Option<PartRef>> = vec![None; slots.len()];
    let put = |visible: &mut Vec<Option<PartRef>>, slot: SlotId, part: &PartRef| {
        let i = usize::from(slot.0);
        if i >= visible.len() {
            visible.resize(i + 1, None);
        }
        visible[i] = Some(part.clone());
    };
    for (slot, part) in base {
        put(&mut visible, *slot, part);
    }
    let mut order: Vec<usize> = (0..worn.len()).collect();
    // Stable: equal layers keep equip order, so the later item wins.
    order.sort_by_key(|&i| worn[i].layer);
    for i in order {
        let item = worn[i];
        for slot in &item.hides {
            if let Some(v) = visible.get_mut(usize::from(slot.0)) {
                *v = None;
            }
        }
        let parts = match view {
            View::ThirdPerson => &item.parts,
            View::FirstPerson => &item.first_person,
        };
        for (slot, part) in parts {
            put(&mut visible, *slot, part);
        }
    }
    let parts: Vec<(SlotId, PartRef)> =
        visible.into_iter().enumerate().filter_map(|(i, p)| p.map(|p| (SlotId(u16::try_from(i).expect("slot index fits u16")), p))).collect();
    let key = outfit_key(slots, &parts);
    Outfit { parts, key }
}

/// The outfit key of a visible part list.
pub fn outfit_key(slots: &SlotRegistry, parts: &[(SlotId, PartRef)]) -> OutfitKey {
    let mut named: Vec<(&str, &str)> = parts.iter().map(|(s, p)| (slots.name(*s).unwrap_or("?"), p.as_str())).collect();
    named.sort_unstable();
    let mut h = blake3::Hasher::new();
    for (slot, part) in named {
        h.update(slot.as_bytes());
        h.update(&[0]);
        h.update(part.as_bytes());
        h.update(&[0xff]);
    }
    let bytes = h.finalize();
    OutfitKey(u64::from_le_bytes(bytes.as_bytes()[..8].try_into().expect("8 bytes")))
}

/// Resolves a recipe's outfit for `view`, looking records up in `lookup`.
pub fn resolve_recipe(lookup: &dyn PartLookup, slots: &SlotRegistry, recipe: &CharacterRecipe, view: View) -> Result<Outfit, CharacterError> {
    let part = |id: &PartRef| {
        lookup.part(id).map(|p| (p.slot, id.clone())).ok_or_else(|| CharacterError::MissingRecord { kind: BodyPart::KIND, id: id.clone() })
    };
    let base: Vec<(SlotId, PartRef)> = match view {
        View::ThirdPerson => recipe.base.iter().chain(&recipe.head).chain(&recipe.hair).map(part).collect::<Result<_, _>>()?,
        View::FirstPerson => match &recipe.first_person {
            Some(fp) => fp.parts.iter().map(part).collect::<Result<_, _>>()?,
            None => Vec::new(),
        },
    };
    let worn: Vec<&Wearable> = recipe
        .worn
        .iter()
        .map(|id| lookup.wearable(id).ok_or_else(|| CharacterError::MissingRecord { kind: Wearable::KIND, id: id.clone() }))
        .collect::<Result<_, _>>()?;
    Ok(resolve(slots, &base, &worn, view))
}
