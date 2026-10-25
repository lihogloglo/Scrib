//! Morrowind equipment slots and rules (rpg-adapter.md "Items and equipment"; `OpenMW`
//! `getEquipmentSlots`, `ActionEquip`).

use std::collections::HashMap;

use sf_records::{RecordDb, RecordId};
use sf_rpg::{
    EquipChange, EquipPlacement, EquipRules, InventoryError, ItemStack, Loadout, SlotId, SlotTable,
    StackId,
};
use tes3::esp::{
    Armor, ArmorType, Clothing, ClothingType, Light, LightFlags, Lockpick, Probe, Weapon,
    WeaponType,
};

/// Morrowind's equipment slots, in `SlotTable` order (`SlotId(slot as u8)`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum MwSlot {
    Helmet,
    Cuirass,
    Greaves,
    LeftPauldron,
    RightPauldron,
    LeftGauntlet,
    RightGauntlet,
    Boots,
    Shirt,
    Pants,
    Skirt,
    Robe,
    LeftRing,
    RightRing,
    Amulet,
    Belt,
    CarriedRight,
    CarriedLeft,
    Ammunition,
}

pub const SLOT_NAMES: [&str; 19] = [
    "helmet",
    "cuirass",
    "greaves",
    "left_pauldron",
    "right_pauldron",
    "left_gauntlet",
    "right_gauntlet",
    "boots",
    "shirt",
    "pants",
    "skirt",
    "robe",
    "left_ring",
    "right_ring",
    "amulet",
    "belt",
    "carried_right",
    "carried_left",
    "ammunition",
];

impl MwSlot {
    pub fn id(self) -> SlotId {
        SlotId(self as u8)
    }
}

/// The Morrowind `SlotTable`.
pub fn slot_table() -> SlotTable {
    SlotTable::new(SLOT_NAMES).expect("fixed, unique slot names")
}

/// What an equippable item is, for slot selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MwEquipKind {
    Armor(ArmorType),
    Clothing(ClothingType),
    Weapon(WeaponType),
    /// A carryable light (torch, lantern).
    Light,
    /// Lockpicks and probes (held in the weapon hand).
    Tool,
}

impl MwEquipKind {
    /// Candidate slots (more than one: first free wins, see [`MwEquipRules::equip`]) and
    /// whether the whole stack equips.
    pub fn slots(self) -> (&'static [MwSlot], bool) {
        use MwSlot as S;
        match self {
            Self::Armor(t) => (
                match t {
                    ArmorType::Helmet => &[S::Helmet],
                    ArmorType::Cuirass => &[S::Cuirass],
                    ArmorType::LeftPauldron => &[S::LeftPauldron],
                    ArmorType::RightPauldron => &[S::RightPauldron],
                    ArmorType::Greaves => &[S::Greaves],
                    ArmorType::Boots => &[S::Boots],
                    ArmorType::LeftGauntlet | ArmorType::LeftBracer => &[S::LeftGauntlet],
                    ArmorType::RightGauntlet | ArmorType::RightBracer => &[S::RightGauntlet],
                    ArmorType::Shield => &[S::CarriedLeft],
                },
                false,
            ),
            Self::Clothing(t) => (
                match t {
                    ClothingType::Pants => &[S::Pants],
                    ClothingType::Shoes => &[S::Boots],
                    ClothingType::Shirt => &[S::Shirt],
                    ClothingType::Belt => &[S::Belt],
                    ClothingType::Robe => &[S::Robe],
                    ClothingType::RightGlove => &[S::RightGauntlet],
                    ClothingType::LeftGlove => &[S::LeftGauntlet],
                    ClothingType::Skirt => &[S::Skirt],
                    ClothingType::Ring => &[S::LeftRing, S::RightRing],
                    ClothingType::Amulet => &[S::Amulet],
                },
                false,
            ),
            Self::Weapon(WeaponType::Arrow | WeaponType::Bolt) => (&[S::Ammunition], true),
            Self::Weapon(WeaponType::MarksmanThrown) => (&[S::CarriedRight], true),
            // Two-handers use the weapon slot only; a shield stays equipped (hidden while
            // the weapon is drawn), as in OpenMW.
            Self::Weapon(_) | Self::Tool => (&[S::CarriedRight], false),
            Self::Light => (&[S::CarriedLeft], false),
        }
    }
}

/// Item id → equip kind, from the record db.
#[derive(Clone, Debug, Default)]
pub struct MwEquipRules {
    kinds: HashMap<RecordId, MwEquipKind>,
}

impl MwEquipRules {
    pub fn from_records(records: &RecordDb) -> Self {
        let mut rules = Self::default();
        for (id, r) in records.iter::<Armor>() {
            rules.insert(id.clone(), MwEquipKind::Armor(r.data.armor_type));
        }
        for (id, r) in records.iter::<Clothing>() {
            rules.insert(id.clone(), MwEquipKind::Clothing(r.data.clothing_type));
        }
        for (id, r) in records.iter::<Weapon>() {
            rules.insert(id.clone(), MwEquipKind::Weapon(r.data.weapon_type));
        }
        for (id, _) in records
            .iter::<Light>()
            .filter(|(_, l)| l.data.flags.contains(LightFlags::CAN_CARRY))
        {
            rules.insert(id.clone(), MwEquipKind::Light);
        }
        let tools = records
            .iter::<Lockpick>()
            .map(|(id, _)| id)
            .chain(records.iter::<Probe>().map(|(id, _)| id));
        for id in tools {
            rules.insert(id.clone(), MwEquipKind::Tool);
        }
        rules
    }

    pub fn insert(&mut self, item: RecordId, kind: MwEquipKind) {
        self.kinds.insert(item, kind);
    }

    pub fn kind(&self, item: &RecordId) -> Option<MwEquipKind> {
        self.kinds.get(item).copied()
    }

    /// Equips with Morrowind's slot choice: the first free candidate slot; rings with both
    /// slots full cycle (the left ring comes off, the right moves left, the new ring goes
    /// right; `OpenMW` `ActionEquip`).
    pub fn equip(
        &self,
        loadout: &mut Loadout,
        stack: StackId,
    ) -> Result<EquipChange, InventoryError> {
        let source = loadout
            .inventory()
            .get(stack)
            .ok_or(InventoryError::NoStack(stack))?;
        let item = source.item.clone();
        let (slots, whole_stack) = self
            .kind(&item)
            .ok_or(InventoryError::Unequippable)?
            .slots();
        let fixed = |slot: MwSlot| Fixed {
            slot: slot.id(),
            whole_stack,
        };
        let [first, second] = slots else {
            return loadout.equip(stack, self);
        };
        if [first, second]
            .iter()
            .any(|s| loadout.slot(s.id()) == Some(stack))
        {
            return Ok(EquipChange {
                equipped: stack,
                item,
                displaced: Vec::new(),
            });
        }
        if let Some(free) = [first, second]
            .into_iter()
            .find(|s| loadout.slot(s.id()).is_none())
        {
            return loadout.equip(stack, &fixed(*free));
        }
        let mut change = loadout.equip(stack, &fixed(*second))?;
        let moved = std::mem::take(&mut change.displaced);
        if let Some(moved_item) = moved.first() {
            let free_stack = loadout
                .inventory()
                .stacks()
                .iter()
                .find(|s| s.item == *moved_item && !loadout.is_equipped(s.id))
                .map(|s| s.id);
            if let Some(free_stack) = free_stack {
                change.displaced = loadout.equip(free_stack, &fixed(*first))?.displaced;
            }
        }
        Ok(change)
    }
}

/// Occupancy-free placement: the first candidate slot. Prefer [`MwEquipRules::equip`].
impl EquipRules for MwEquipRules {
    fn placement(&self, stack: &ItemStack) -> Option<EquipPlacement> {
        let (slots, whole_stack) = self.kind(&stack.item)?.slots();
        Some(EquipPlacement {
            slots: vec![slots.first()?.id()].into(),
            whole_stack,
        })
    }
}

struct Fixed {
    slot: SlotId,
    whole_stack: bool,
}

impl EquipRules for Fixed {
    fn placement(&self, _stack: &ItemStack) -> Option<EquipPlacement> {
        Some(EquipPlacement {
            slots: vec![self.slot].into(),
            whole_stack: self.whole_stack,
        })
    }
}

#[cfg(test)]
mod tests {
    use sf_rpg::Attrs;

    use super::*;

    fn id(s: &str) -> RecordId {
        RecordId::new(s)
    }

    #[test]
    fn table_matches_enum() {
        let table = slot_table();
        assert_eq!(table.id("left_ring"), Some(MwSlot::LeftRing.id()));
        assert_eq!(table.id("ammunition"), Some(MwSlot::Ammunition.id()));
        assert_eq!(table.len(), SLOT_NAMES.len());
    }

    #[test]
    fn rings_fill_left_then_right_then_cycle() {
        let mut rules = MwEquipRules::default();
        for r in ["ring_a", "ring_b", "ring_c"] {
            rules.insert(id(r), MwEquipKind::Clothing(ClothingType::Ring));
        }
        let mut lo = Loadout::new();
        let a = lo.add(id("ring_a"), 1, Attrs::new()).unwrap();
        let b = lo.add(id("ring_b"), 1, Attrs::new()).unwrap();
        let c = lo.add(id("ring_c"), 1, Attrs::new()).unwrap();
        rules.equip(&mut lo, a).unwrap();
        assert_eq!(lo.slot(MwSlot::LeftRing.id()), Some(a));
        rules.equip(&mut lo, b).unwrap();
        assert_eq!(lo.slot(MwSlot::RightRing.id()), Some(b));
        let change = rules.equip(&mut lo, c).unwrap();
        assert_eq!(change.displaced, vec![id("ring_a")]);
        assert_eq!(lo.slot(MwSlot::LeftRing.id()), Some(b));
        assert_eq!(lo.slot(MwSlot::RightRing.id()), Some(c));
        assert!(!lo.is_equipped(a));
    }

    #[test]
    fn ammo_whole_stack_and_shield_with_two_hander() {
        let mut rules = MwEquipRules::default();
        rules.insert(id("arrow"), MwEquipKind::Weapon(WeaponType::Arrow));
        rules.insert(
            id("claymore"),
            MwEquipKind::Weapon(WeaponType::LongBladeTwoClose),
        );
        rules.insert(id("shield"), MwEquipKind::Armor(ArmorType::Shield));
        let mut lo = Loadout::new();
        let arrows = lo.add(id("arrow"), 20, Attrs::new()).unwrap();
        let sword = lo.add(id("claymore"), 1, Attrs::new()).unwrap();
        let shield = lo.add(id("shield"), 1, Attrs::new()).unwrap();
        rules.equip(&mut lo, arrows).unwrap();
        assert_eq!(
            lo.inventory()
                .get(lo.slot(MwSlot::Ammunition.id()).unwrap())
                .unwrap()
                .count,
            20
        );
        rules.equip(&mut lo, shield).unwrap();
        rules.equip(&mut lo, sword).unwrap();
        assert_eq!(lo.slot(MwSlot::CarriedLeft.id()), Some(shield));
        assert_eq!(lo.slot(MwSlot::CarriedRight.id()), Some(sword));
    }
}
