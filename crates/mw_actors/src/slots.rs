//! Morrowind's 27 body-part slots (`BipedObjectType`, "PRT") on the
//! framework's slot registry, with the bone each part attaches to.
//!
//! Attach bones and the hair filter follow the original engine's part list
//! (as `OpenMW`'s `NpcAnimation::sPartList` documents it). Slots reuse the
//! framework's humanoid defaults where the region matches; Morrowind adds
//! skirt, wrists, ankles, shield, weapon, and `body` (a creature's whole model).

use sf_character::{BoneSet, HumanoidBone as B, SlotId, SlotRegistry, slot as hs};
use tes3::esp::BodypartId;

/// Slots Morrowind adds to the humanoid defaults.
pub mod slot {
    pub const SKIRT: &str = "skirt";
    pub const LEFT_WRIST: &str = "left_wrist";
    pub const RIGHT_WRIST: &str = "right_wrist";
    pub const LEFT_ANKLE: &str = "left_ankle";
    pub const RIGHT_ANKLE: &str = "right_ankle";
    pub const SHIELD: &str = "shield";
    pub const WEAPON: &str = "weapon";
    /// A creature's whole model (it is its own skeleton).
    pub const BODY: &str = "body";
}

/// Number of part slots (`BipedObjectType` 0..=26).
pub const PRT_COUNT: usize = 27;

/// `BipedObjectType` indices used by the rules.
pub mod prt {
    pub const HEAD: usize = 0;
    pub const HAIR: usize = 1;
    pub const NECK: usize = 2;
    pub const CUIRASS: usize = 3;
    pub const GROIN: usize = 4;
    pub const SKIRT: usize = 5;
    pub const R_HAND: usize = 6;
    pub const L_HAND: usize = 7;
    pub const R_WRIST: usize = 8;
    pub const L_WRIST: usize = 9;
    pub const SHIELD: usize = 10;
    pub const R_FOREARM: usize = 11;
    pub const L_FOREARM: usize = 12;
    pub const R_UPPER_ARM: usize = 13;
    pub const L_UPPER_ARM: usize = 14;
    pub const R_FOOT: usize = 15;
    pub const L_FOOT: usize = 16;
    pub const R_ANKLE: usize = 17;
    pub const L_ANKLE: usize = 18;
    pub const R_KNEE: usize = 19;
    pub const L_KNEE: usize = 20;
    pub const R_LEG: usize = 21;
    pub const L_LEG: usize = 22;
    pub const R_PAULDRON: usize = 23;
    pub const L_PAULDRON: usize = 24;
    pub const WEAPON: usize = 25;
    pub const TAIL: usize = 26;
}

/// One part slot: the framework slot it fills and the bone it attaches to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PartSlot {
    pub slot: &'static str,
    /// Joint a rigid part is bolted to.
    pub bone: &'static str,
    /// Node filter of a skinned part (one skin file holds both sides).
    pub filter: &'static str,
}

const fn ps(slot: &'static str, bone: &'static str) -> PartSlot {
    PartSlot { slot, bone, filter: bone }
}

/// Indexed by `BipedObjectType`.
pub const PART_SLOTS: [PartSlot; PRT_COUNT] = [
    ps(hs::HEAD, "Head"),
    // The one slot whose filter differs from its bone.
    PartSlot { slot: hs::HAIR, bone: "Head", filter: "Hair" },
    ps(hs::NECK, "Neck"),
    ps(hs::CHEST, "Chest"),
    ps(hs::PELVIS, "Groin"),
    ps(slot::SKIRT, "Groin"),
    ps(hs::RIGHT_HAND, "Right Hand"),
    ps(hs::LEFT_HAND, "Left Hand"),
    ps(slot::RIGHT_WRIST, "Right Wrist"),
    ps(slot::LEFT_WRIST, "Left Wrist"),
    ps(slot::SHIELD, "Shield Bone"),
    ps(hs::RIGHT_FOREARM, "Right Forearm"),
    ps(hs::LEFT_FOREARM, "Left Forearm"),
    ps(hs::RIGHT_UPPER_ARM, "Right Upper Arm"),
    ps(hs::LEFT_UPPER_ARM, "Left Upper Arm"),
    ps(hs::RIGHT_FOOT, "Right Foot"),
    ps(hs::LEFT_FOOT, "Left Foot"),
    ps(slot::RIGHT_ANKLE, "Right Ankle"),
    ps(slot::LEFT_ANKLE, "Left Ankle"),
    // Morrowind's knee part is the lower leg.
    ps(hs::RIGHT_LOWER_LEG, "Right Knee"),
    ps(hs::LEFT_LOWER_LEG, "Left Knee"),
    ps(hs::RIGHT_UPPER_LEG, "Right Upper Leg"),
    ps(hs::LEFT_UPPER_LEG, "Left Upper Leg"),
    ps(hs::RIGHT_SHOULDER, "Right Clavicle"),
    ps(hs::LEFT_SHOULDER, "Left Clavicle"),
    ps(slot::WEAPON, "Weapon Bone"),
    ps(hs::TAIL, "Tail"),
];

/// Part slots shown in the first-person view: arms, pauldrons, shield, weapon.
pub const FIRST_PERSON: [usize; 11] = [
    prt::R_HAND,
    prt::L_HAND,
    prt::R_WRIST,
    prt::L_WRIST,
    prt::R_FOREARM,
    prt::L_FOREARM,
    prt::R_UPPER_ARM,
    prt::L_UPPER_ARM,
    prt::R_PAULDRON,
    prt::L_PAULDRON,
    prt::SHIELD,
];

/// The part slots a skin part (`BODY` of type skin) fills, by its mesh part.
/// Head and hair come from the NPC record; clavicles have no skin.
pub fn skin_slots(part: BodypartId) -> &'static [usize] {
    use prt::{NECK, CUIRASS, GROIN, R_HAND, L_HAND, R_WRIST, L_WRIST, R_FOREARM, L_FOREARM, R_UPPER_ARM, L_UPPER_ARM, R_FOOT, L_FOOT, R_ANKLE, L_ANKLE, R_KNEE, L_KNEE, R_LEG, L_LEG, TAIL};
    match part {
        BodypartId::Neck => &[NECK],
        BodypartId::Chest => &[CUIRASS],
        BodypartId::Groin => &[GROIN],
        BodypartId::Hand => &[R_HAND, L_HAND],
        BodypartId::Wrist => &[R_WRIST, L_WRIST],
        BodypartId::Forearm => &[R_FOREARM, L_FOREARM],
        BodypartId::UpperArm => &[R_UPPER_ARM, L_UPPER_ARM],
        BodypartId::Foot => &[R_FOOT, L_FOOT],
        BodypartId::Ankle => &[R_ANKLE, L_ANKLE],
        BodypartId::Knee => &[R_KNEE, L_KNEE],
        BodypartId::UpperLeg => &[R_LEG, L_LEG],
        BodypartId::Tail => &[TAIL],
        BodypartId::Head | BodypartId::Hair | BodypartId::Clavicle => &[],
    }
}

/// Every part slot a `BODY` of this mesh part can fill (skin or item).
pub fn part_slots(part: BodypartId) -> &'static [usize] {
    use prt::{HEAD, HAIR, R_PAULDRON, L_PAULDRON, GROIN, SKIRT};
    match part {
        BodypartId::Head => &[HEAD],
        BodypartId::Hair => &[HAIR],
        BodypartId::Clavicle => &[R_PAULDRON, L_PAULDRON],
        BodypartId::Groin => &[GROIN, SKIRT],
        other => skin_slots(other),
    }
}

/// Arm parts: the only third-person parts allowed in first person when no
/// `.1st` part exists.
pub fn is_arm(part: BodypartId) -> bool {
    matches!(part, BodypartId::Hand | BodypartId::Wrist | BodypartId::Forearm | BodypartId::UpperArm)
}

/// Registers the humanoid defaults plus Morrowind's slots; returns the
/// slot of each part slot and the creature `body` slot.
pub fn register(reg: &mut SlotRegistry) -> ([SlotId; PRT_COUNT], SlotId) {
    reg.register_humanoid_defaults();
    reg.register(slot::SKIRT, BoneSet::of(&[B::Hips, B::LeftUpperLeg, B::RightUpperLeg]));
    reg.register(slot::LEFT_WRIST, BoneSet::of(&[B::LeftLowerArm]));
    reg.register(slot::RIGHT_WRIST, BoneSet::of(&[B::RightLowerArm]));
    reg.register(slot::LEFT_ANKLE, BoneSet::of(&[B::LeftLowerLeg]));
    reg.register(slot::RIGHT_ANKLE, BoneSet::of(&[B::RightLowerLeg]));
    reg.register(slot::SHIELD, BoneSet::EMPTY);
    reg.register(slot::WEAPON, BoneSet::EMPTY);
    let body = reg.register(slot::BODY, BoneSet::EMPTY);
    let ids = PART_SLOTS.map(|p| reg.get(p.slot).expect("registered above"));
    (ids, body)
}
