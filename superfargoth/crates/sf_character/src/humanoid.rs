//! The standard humanoid bone set: VRM 1.0's 55 humanoid bones
//! (`VRMC_vrm-1.0/humanoid.md`), which also matches Unity's Humanoid
//! avatar. 15 are required (hips, spine, head, and both upper/lower legs,
//! feet, upper/lower arms and hands).
//!
//! The canonical reference pose is a T-pose, Y up, facing +Z, in meters:
//! the character's left is +X, arms point ±X, legs point −Y.

use serde::{Deserialize, Serialize};

/// Left or right side of the body, or the center line.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Side {
    Center,
    Left,
    Right,
}

impl Side {
    #[must_use]
    pub fn mirror(self) -> Self {
        match self {
            Self::Center => Self::Center,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    /// +1 on the left (+X in the canonical pose), −1 on the right, 0 at the center.
    pub fn sign(self) -> f32 {
        match self {
            Self::Center => 0.0,
            Self::Left => 1.0,
            Self::Right => -1.0,
        }
    }
}

/// A finger, thumb first.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Finger {
    Thumb,
    Index,
    Middle,
    Ring,
    Little,
}

impl Finger {
    pub const ALL: [Self; 5] = [Self::Thumb, Self::Index, Self::Middle, Self::Ring, Self::Little];

    pub fn name(self) -> &'static str {
        match self {
            Self::Thumb => "thumb",
            Self::Index => "index",
            Self::Middle => "middle",
            Self::Ring => "ring",
            Self::Little => "little",
        }
    }
}

macro_rules! humanoid_bones {
    ($( $variant:ident = $name:literal, $parent:expr, $required:literal; )*) => {
        /// A VRM 1.0 humanoid bone. The discriminant is a dense index (0..55).
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        #[repr(u8)]
        pub enum HumanoidBone {
            $( $variant, )*
        }

        impl HumanoidBone {
            /// Every bone, parents before children.
            pub const ALL: [Self; 55] = [$( Self::$variant, )*];

            /// The VRM 1.0 name (`leftUpperArm`).
            pub fn name(self) -> &'static str {
                match self {
                    $( Self::$variant => $name, )*
                }
            }

            /// The VRM 1.0 parent bone (`None` for the hips). When it is
            /// absent from a rig, its own parent stands in (VRM rule).
            pub fn parent(self) -> Option<Self> {
                match self {
                    $( Self::$variant => $parent, )*
                }
            }

            /// Required by VRM 1.0 (15 bones).
            pub fn required(self) -> bool {
                match self {
                    $( Self::$variant => $required, )*
                }
            }
        }
    };
}

use HumanoidBone as B;

humanoid_bones! {
    Hips = "hips", None, true;
    Spine = "spine", Some(B::Hips), true;
    Chest = "chest", Some(B::Spine), false;
    UpperChest = "upperChest", Some(B::Chest), false;
    Neck = "neck", Some(B::UpperChest), false;
    Head = "head", Some(B::Neck), true;
    LeftEye = "leftEye", Some(B::Head), false;
    RightEye = "rightEye", Some(B::Head), false;
    Jaw = "jaw", Some(B::Head), false;
    LeftUpperLeg = "leftUpperLeg", Some(B::Hips), true;
    LeftLowerLeg = "leftLowerLeg", Some(B::LeftUpperLeg), true;
    LeftFoot = "leftFoot", Some(B::LeftLowerLeg), true;
    LeftToes = "leftToes", Some(B::LeftFoot), false;
    RightUpperLeg = "rightUpperLeg", Some(B::Hips), true;
    RightLowerLeg = "rightLowerLeg", Some(B::RightUpperLeg), true;
    RightFoot = "rightFoot", Some(B::RightLowerLeg), true;
    RightToes = "rightToes", Some(B::RightFoot), false;
    LeftShoulder = "leftShoulder", Some(B::UpperChest), false;
    LeftUpperArm = "leftUpperArm", Some(B::LeftShoulder), true;
    LeftLowerArm = "leftLowerArm", Some(B::LeftUpperArm), true;
    LeftHand = "leftHand", Some(B::LeftLowerArm), true;
    RightShoulder = "rightShoulder", Some(B::UpperChest), false;
    RightUpperArm = "rightUpperArm", Some(B::RightShoulder), true;
    RightLowerArm = "rightLowerArm", Some(B::RightUpperArm), true;
    RightHand = "rightHand", Some(B::RightLowerArm), true;
    LeftThumbMetacarpal = "leftThumbMetacarpal", Some(B::LeftHand), false;
    LeftThumbProximal = "leftThumbProximal", Some(B::LeftThumbMetacarpal), false;
    LeftThumbDistal = "leftThumbDistal", Some(B::LeftThumbProximal), false;
    LeftIndexProximal = "leftIndexProximal", Some(B::LeftHand), false;
    LeftIndexIntermediate = "leftIndexIntermediate", Some(B::LeftIndexProximal), false;
    LeftIndexDistal = "leftIndexDistal", Some(B::LeftIndexIntermediate), false;
    LeftMiddleProximal = "leftMiddleProximal", Some(B::LeftHand), false;
    LeftMiddleIntermediate = "leftMiddleIntermediate", Some(B::LeftMiddleProximal), false;
    LeftMiddleDistal = "leftMiddleDistal", Some(B::LeftMiddleIntermediate), false;
    LeftRingProximal = "leftRingProximal", Some(B::LeftHand), false;
    LeftRingIntermediate = "leftRingIntermediate", Some(B::LeftRingProximal), false;
    LeftRingDistal = "leftRingDistal", Some(B::LeftRingIntermediate), false;
    LeftLittleProximal = "leftLittleProximal", Some(B::LeftHand), false;
    LeftLittleIntermediate = "leftLittleIntermediate", Some(B::LeftLittleProximal), false;
    LeftLittleDistal = "leftLittleDistal", Some(B::LeftLittleIntermediate), false;
    RightThumbMetacarpal = "rightThumbMetacarpal", Some(B::RightHand), false;
    RightThumbProximal = "rightThumbProximal", Some(B::RightThumbMetacarpal), false;
    RightThumbDistal = "rightThumbDistal", Some(B::RightThumbProximal), false;
    RightIndexProximal = "rightIndexProximal", Some(B::RightHand), false;
    RightIndexIntermediate = "rightIndexIntermediate", Some(B::RightIndexProximal), false;
    RightIndexDistal = "rightIndexDistal", Some(B::RightIndexIntermediate), false;
    RightMiddleProximal = "rightMiddleProximal", Some(B::RightHand), false;
    RightMiddleIntermediate = "rightMiddleIntermediate", Some(B::RightMiddleProximal), false;
    RightMiddleDistal = "rightMiddleDistal", Some(B::RightMiddleIntermediate), false;
    RightRingProximal = "rightRingProximal", Some(B::RightHand), false;
    RightRingIntermediate = "rightRingIntermediate", Some(B::RightRingProximal), false;
    RightRingDistal = "rightRingDistal", Some(B::RightRingIntermediate), false;
    RightLittleProximal = "rightLittleProximal", Some(B::RightHand), false;
    RightLittleIntermediate = "rightLittleIntermediate", Some(B::RightLittleProximal), false;
    RightLittleDistal = "rightLittleDistal", Some(B::RightLittleIntermediate), false;
}

impl HumanoidBone {
    /// Dense index, 0..55.
    pub fn index(self) -> usize {
        self as usize
    }

    pub fn from_index(i: usize) -> Option<Self> {
        Self::ALL.get(i).copied()
    }

    /// Parses a VRM 1.0 name (exact, case-sensitive as in the spec).
    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|b| b.name() == name)
    }

    pub fn side(self) -> Side {
        let name = self.name();
        if name.starts_with("left") {
            Side::Left
        } else if name.starts_with("right") {
            Side::Right
        } else {
            Side::Center
        }
    }

    /// The same bone on the other side (itself on the center line).
    #[must_use]
    pub fn mirror(self) -> Self {
        let i = self.index();
        match self.side() {
            Side::Center => self,
            // Left and right blocks have the same layout: eyes 6 / 7, legs
            // 9..13 / 13..17, arms 17..21 / 21..25, fingers 25..40 / 40..55.
            Side::Left | Side::Right => {
                let offset = match i {
                    6 | 7 => 1,
                    9..=24 => 4,
                    _ => 15,
                };
                let j = if self.side() == Side::Left { i + offset } else { i - offset };
                Self::ALL[j]
            }
        }
    }

    /// The nearest ancestor (VRM hierarchy) present in `set`.
    pub fn nearest_parent_in(self, set: BoneSet) -> Option<Self> {
        let mut p = self.parent();
        while let Some(b) = p {
            if set.contains(b) {
                return Some(b);
            }
            p = b.parent();
        }
        None
    }

    /// The bone of a finger segment. Thumbs have metacarpal, proximal and
    /// distal segments; other fingers proximal, intermediate and distal.
    pub fn finger(side: Side, finger: Finger, segment: usize) -> Option<Self> {
        if segment > 2 || side == Side::Center {
            return None;
        }
        let base = if side == Side::Left { 25 } else { 40 };
        Self::from_index(base + finger as usize * 3 + segment)
    }

    /// Which finger and segment this bone is, if any.
    pub fn finger_of(self) -> Option<(Side, Finger, usize)> {
        let i = self.index();
        if i < 25 {
            return None;
        }
        let k = (i - 25) % 15;
        Some((self.side(), Finger::ALL[k / 3], k % 3))
    }

    /// A limb bone by side: `limb(Side::Left, Limb::UpperArm)`.
    pub fn limb(side: Side, limb: Limb) -> Option<Self> {
        let left = match limb {
            Limb::UpperLeg => B::LeftUpperLeg,
            Limb::LowerLeg => B::LeftLowerLeg,
            Limb::Foot => B::LeftFoot,
            Limb::Toes => B::LeftToes,
            Limb::Shoulder => B::LeftShoulder,
            Limb::UpperArm => B::LeftUpperArm,
            Limb::LowerArm => B::LeftLowerArm,
            Limb::Hand => B::LeftHand,
            Limb::Eye => B::LeftEye,
        };
        match side {
            Side::Left => Some(left),
            Side::Right => Some(left.mirror()),
            Side::Center => None,
        }
    }
}

/// Paired (left/right) bones, by role.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Limb {
    UpperLeg,
    LowerLeg,
    Foot,
    Toes,
    Shoulder,
    UpperArm,
    LowerArm,
    Hand,
    Eye,
}

/// A set of humanoid bones (a 64-bit mask), used for body regions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BoneSet(pub u64);

impl BoneSet {
    pub const EMPTY: Self = Self(0);

    pub fn of(bones: &[HumanoidBone]) -> Self {
        bones.iter().fold(Self::EMPTY, |s, b| s.with(*b))
    }

    #[must_use]
    pub fn with(self, bone: HumanoidBone) -> Self {
        Self(self.0 | 1 << bone.index())
    }

    pub fn insert(&mut self, bone: HumanoidBone) {
        self.0 |= 1 << bone.index();
    }

    pub fn contains(self, bone: HumanoidBone) -> bool {
        self.0 & (1 << bone.index()) != 0
    }

    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub fn len(self) -> usize {
        self.0.count_ones() as usize
    }

    #[must_use]
    pub fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    pub fn intersects(self, other: Self) -> bool {
        self.0 & other.0 != 0
    }

    pub fn iter(self) -> impl Iterator<Item = HumanoidBone> {
        HumanoidBone::ALL.into_iter().filter(move |b| self.contains(*b))
    }

    /// The bones a finger chain or limb covers, for region definitions.
    pub fn fingers(side: Side) -> Self {
        let mut s = Self::EMPTY;
        for f in Finger::ALL {
            for k in 0..3 {
                if let Some(b) = HumanoidBone::finger(side, f, k) {
                    s.insert(b);
                }
            }
        }
        s
    }
}
