//! Morrowind `ClothRules` for `cloth.cook` (physics-adapter.md "Cloth").
//! Census (p10b): a robe (CLOT *Robe*) is one skinned mesh in the *Chest*
//! slot from the shoulders to the ankles (plus two ankle parts); a skirt
//! (CLOT *Skirt*) is a skinned mesh in the *Skirt* slot (a few use *Groin*,
//! left as they are). The part below `Bip01 Pelvis` hangs as a ring lattice:
//! robes 10 × 5, skirts 8 × 3. Vanilla has no capes. Part ids are
//! `mw_actors`' `"<BODY id>#<slot name>"`.

use std::collections::HashMap;

use sf_anim::cloth::{ClothParams, GridSpec};
use sf_character::{BodyPart, ClothRules, ClothSpec};
use sf_records::{RecordDb, RecordId};
use tes3::esp::{BipedObjectType, Clothing, ClothingType};

/// The joint robes and skirts hang from.
pub const CLOTH_ATTACH: &str = "Bip01 Pelvis";
/// `mw_actors`' slot names (`sf_character::slot::CHEST`, `mw_actors::slot::SKIRT`).
pub const CHEST_SLOT: &str = "chest";
pub const SKIRT_SLOT: &str = "skirt";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ClothKind {
    Robe,
    Skirt,
}

impl ClothKind {
    /// The slot whose part is the cloth, and its `BIPD` type.
    pub fn slot(self) -> (&'static str, BipedObjectType) {
        match self {
            Self::Robe => (CHEST_SLOT, BipedObjectType::Chest),
            Self::Skirt => (SKIRT_SLOT, BipedObjectType::Skirt),
        }
    }
}

/// The lattice setup of a kind: a robe reaches the ankles (5 rows of 10),
/// a skirt the knees or calves (3 rows of 8); a robe's hem carries more cloth.
pub fn cloth_spec(kind: ClothKind) -> ClothSpec {
    let grid = match kind {
        ClothKind::Robe => GridSpec { rows: 5, cols: 10, ring: true, mass: 1.5, max_distance_scale: 0.6 },
        ClothKind::Skirt => GridSpec { rows: 3, cols: 8, ring: true, mass: 0.6, max_distance_scale: 0.6 },
    };
    ClothSpec { attach: CLOTH_ATTACH.to_owned(), attach_offset: 0.0, grid, params: ClothParams::default() }
}

/// Which `(BODY, slot)` parts are cloth: the chest part of every robe and
/// the skirt part of every skirt, male and female.
#[derive(Clone, Debug, Default)]
pub struct MwClothRules {
    parts: HashMap<(String, &'static str), ClothKind>,
}

impl MwClothRules {
    pub fn new(db: &RecordDb) -> Self {
        let mut parts = HashMap::new();
        for (_, c) in db.iter::<Clothing>() {
            let kind = match c.data.clothing_type {
                ClothingType::Robe => ClothKind::Robe,
                ClothingType::Skirt => ClothKind::Skirt,
                _ => continue,
            };
            let (slot, bipd) = kind.slot();
            for o in c.biped_objects.iter().filter(|o| o.biped_object_type == bipd) {
                for body in [&o.male_bodypart, &o.female_bodypart].into_iter().filter(|b| !b.is_empty()) {
                    parts.insert((body.to_ascii_lowercase(), slot), kind);
                }
            }
        }
        Self { parts }
    }

    /// The kind of a `BODY` id (any case) in a slot, if it is cloth there.
    pub fn kind(&self, body: &str, slot: &str) -> Option<ClothKind> {
        let slot = [CHEST_SLOT, SKIRT_SLOT].into_iter().find(|s| s.eq_ignore_ascii_case(slot))?;
        self.parts.get(&(body.to_ascii_lowercase(), slot)).copied()
    }

    /// The cloth parts, sorted: `(BODY id, slot, kind)`.
    pub fn parts(&self) -> Vec<(&str, &'static str, ClothKind)> {
        let mut out: Vec<_> = self.parts.iter().map(|((b, s), k)| (b.as_str(), *s, *k)).collect();
        out.sort_unstable_by_key(|&(b, s, _)| (b, s));
        out
    }

    /// Number of cloth parts.
    pub fn len(&self) -> usize {
        self.parts.len()
    }

    pub fn is_empty(&self) -> bool {
        self.parts.is_empty()
    }
}

impl ClothRules for MwClothRules {
    fn cloth(&self, id: &RecordId, _part: &BodyPart) -> Option<ClothSpec> {
        let (body, slot) = id.as_str().rsplit_once('#')?;
        self.kind(body, slot).map(cloth_spec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn robe_and_skirt_specs() {
        let robe = cloth_spec(ClothKind::Robe);
        let skirt = cloth_spec(ClothKind::Skirt);
        assert_eq!((robe.grid.rows, robe.grid.cols, robe.grid.ring), (5, 10, true));
        assert_eq!((skirt.grid.rows, skirt.grid.cols, skirt.grid.ring), (3, 8, true));
        assert_eq!(robe.attach, CLOTH_ATTACH);
    }

    #[test]
    fn rules_match_the_cloth_slot_only() {
        let rules = MwClothRules {
            parts: HashMap::from([(("c_robe_a".to_owned(), CHEST_SLOT), ClothKind::Robe), (("c_skirt_b".to_owned(), SKIRT_SLOT), ClothKind::Skirt)]),
        };
        let part = BodyPart {
            model: sf_vfs::VfsPath::new("meshes/c/robe.nif").unwrap(),
            slot: sf_character::SlotId(0),
            kind: sf_character::PartKind::Skinned,
            node_filter: None,
        };
        let at = |id: &str| rules.cloth(&RecordId::new(id), &part).map(|s| s.grid.rows);
        assert_eq!(at("C_Robe_A#chest"), Some(5));
        assert_eq!(at("c_skirt_b#skirt"), Some(3));
        assert_eq!(at("c_robe_a#skirt"), None, "a robe's cloth is its chest part");
        assert_eq!(at("c_skirt_b#chest"), None);
        assert_eq!(at("c_shirt#chest"), None);
        assert_eq!(at("no_slot"), None);
        assert_eq!(rules.parts().len(), 2);
    }
}
