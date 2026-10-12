//! Body slots: a registry, not an enum. Content registers slot names (as
//! it registers record kinds); each slot names the humanoid region it
//! covers (for hiding now, cloth later). A default humanoid set ships for
//! native content; adapters register their own game's slots.

use std::collections::HashMap;

use crate::humanoid::{BoneSet, HumanoidBone as B, Side};

/// A registered slot. Ids are dense registration indices: valid for one
/// registry (one session), never saved. Saves and caches use slot names.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SlotId(pub u16);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SlotDef {
    pub name: String,
    /// The humanoid bones the slot's parts cover (empty for accessories).
    pub region: BoneSet,
}

/// The slot registry.
#[derive(Clone, Debug, Default)]
pub struct SlotRegistry {
    slots: Vec<SlotDef>,
    by_name: HashMap<String, SlotId>,
}

/// Names of the default humanoid slots ([`SlotRegistry::with_humanoid_defaults`]).
pub mod slot {
    pub const HEAD: &str = "head";
    pub const HAIR: &str = "hair";
    pub const FACE: &str = "face";
    pub const NECK: &str = "neck";
    pub const CHEST: &str = "chest";
    pub const ABDOMEN: &str = "abdomen";
    pub const PELVIS: &str = "pelvis";
    pub const BACK: &str = "back";
    pub const TAIL: &str = "tail";
    pub const LEFT_SHOULDER: &str = "left_shoulder";
    pub const LEFT_UPPER_ARM: &str = "left_upper_arm";
    pub const LEFT_FOREARM: &str = "left_forearm";
    pub const LEFT_HAND: &str = "left_hand";
    pub const LEFT_UPPER_LEG: &str = "left_upper_leg";
    pub const LEFT_LOWER_LEG: &str = "left_lower_leg";
    pub const LEFT_FOOT: &str = "left_foot";
    pub const RIGHT_SHOULDER: &str = "right_shoulder";
    pub const RIGHT_UPPER_ARM: &str = "right_upper_arm";
    pub const RIGHT_FOREARM: &str = "right_forearm";
    pub const RIGHT_HAND: &str = "right_hand";
    pub const RIGHT_UPPER_LEG: &str = "right_upper_leg";
    pub const RIGHT_LOWER_LEG: &str = "right_lower_leg";
    pub const RIGHT_FOOT: &str = "right_foot";
}

impl SlotRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// A registry with the default humanoid slot set.
    pub fn with_humanoid_defaults() -> Self {
        let mut r = Self::new();
        r.register_humanoid_defaults();
        r
    }

    /// Registers the default humanoid slots: head, hair, face, neck, chest,
    /// abdomen, pelvis, back, tail, and per side shoulder, upper arm,
    /// forearm, hand, upper leg, lower leg, foot.
    pub fn register_humanoid_defaults(&mut self) {
        let head = BoneSet::of(&[B::Head, B::LeftEye, B::RightEye, B::Jaw]);
        self.register(slot::HEAD, head);
        self.register(slot::HAIR, BoneSet::of(&[B::Head]));
        self.register(slot::FACE, head);
        self.register(slot::NECK, BoneSet::of(&[B::Neck]));
        self.register(slot::CHEST, BoneSet::of(&[B::Chest, B::UpperChest]));
        self.register(slot::ABDOMEN, BoneSet::of(&[B::Spine]));
        self.register(slot::PELVIS, BoneSet::of(&[B::Hips]));
        self.register(slot::BACK, BoneSet::of(&[B::Chest, B::UpperChest]));
        self.register(slot::TAIL, BoneSet::EMPTY);
        for (side, names) in [
            (
                Side::Left,
                [
                    slot::LEFT_SHOULDER,
                    slot::LEFT_UPPER_ARM,
                    slot::LEFT_FOREARM,
                    slot::LEFT_HAND,
                    slot::LEFT_UPPER_LEG,
                    slot::LEFT_LOWER_LEG,
                    slot::LEFT_FOOT,
                ],
            ),
            (
                Side::Right,
                [
                    slot::RIGHT_SHOULDER,
                    slot::RIGHT_UPPER_ARM,
                    slot::RIGHT_FOREARM,
                    slot::RIGHT_HAND,
                    slot::RIGHT_UPPER_LEG,
                    slot::RIGHT_LOWER_LEG,
                    slot::RIGHT_FOOT,
                ],
            ),
        ] {
            let m = |b: B| if side == Side::Left { b } else { b.mirror() };
            let regions = [
                BoneSet::of(&[m(B::LeftShoulder)]),
                BoneSet::of(&[m(B::LeftUpperArm)]),
                BoneSet::of(&[m(B::LeftLowerArm)]),
                BoneSet::of(&[m(B::LeftHand)]).union(BoneSet::fingers(side)),
                BoneSet::of(&[m(B::LeftUpperLeg)]),
                BoneSet::of(&[m(B::LeftLowerLeg)]),
                BoneSet::of(&[m(B::LeftFoot), m(B::LeftToes)]),
            ];
            for (name, region) in names.into_iter().zip(regions) {
                self.register(name, region);
            }
        }
    }

    /// Registers a slot, or returns the existing one of that name (its
    /// region grows to include `region`, so mods can extend a slot).
    pub fn register(&mut self, name: &str, region: BoneSet) -> SlotId {
        if let Some(&id) = self.by_name.get(name) {
            let def = &mut self.slots[usize::from(id.0)];
            def.region = def.region.union(region);
            return id;
        }
        let id = SlotId(u16::try_from(self.slots.len()).expect("fewer than 65536 slots"));
        self.slots.push(SlotDef { name: name.to_owned(), region });
        self.by_name.insert(name.to_owned(), id);
        id
    }

    pub fn get(&self, name: &str) -> Option<SlotId> {
        self.by_name.get(name).copied()
    }

    pub fn def(&self, id: SlotId) -> Option<&SlotDef> {
        self.slots.get(usize::from(id.0))
    }

    pub fn name(&self, id: SlotId) -> Option<&str> {
        self.def(id).map(|d| d.name.as_str())
    }

    pub fn len(&self) -> usize {
        self.slots.len()
    }

    pub fn is_empty(&self) -> bool {
        self.slots.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = (SlotId, &SlotDef)> {
        self.slots.iter().enumerate().map(|(i, d)| (SlotId(u16::try_from(i).expect("checked at registration")), d))
    }

    /// Slots whose region includes `bone`.
    pub fn covering(&self, bone: B) -> impl Iterator<Item = SlotId> + '_ {
        self.iter().filter(move |(_, d)| d.region.contains(bone)).map(|(id, _)| id)
    }
}
