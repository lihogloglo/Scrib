//! Native record kinds (`sf.body_part`, `sf.wearable`) and the character
//! recipe. Adapters convert their own records into these at load; mods
//! patch them like any record.

use glam::{Affine3A, Vec3, Vec4};
use sf_records::{RecordDb, RecordError, RecordId};
use sf_vfs::VfsPath;

use crate::rig::JointName;
use crate::slots::SlotId;

/// A [`BodyPart`] record id.
pub type PartRef = RecordId;
/// A [`Wearable`] record id.
pub type WearableRef = RecordId;
/// Which rig map a skeleton uses: the key of [`crate::CharacterSource::rig_maps`]
/// (by convention the skeleton's path).
pub type RigMapRef = VfsPath;
/// Which rig profile a skeleton uses: the `rig.profile` subject (the
/// skeleton path).
pub type RigProfileRef = VfsPath;

/// How a recipe's skeleton is described (rig-profile.md): a humanoid rig
/// map (which also yields a profile and retargets through the humanoid
/// hub), or only a rig profile (creatures: no humanoid map needed).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RigRef {
    Humanoid(RigMapRef),
    Profile(RigProfileRef),
}
/// An animation graph asset.
pub type AnimGraphRef = VfsPath;
/// An attachment point: a socket name or joint name of the skeleton.
pub type Socket = String;

/// How a part binds to the skeleton.
#[derive(Clone, Debug, PartialEq)]
pub enum PartKind {
    /// Skinned to the skeleton by joint name.
    Skinned,
    /// A rigid model bolted to one joint (cooked into a one-joint skin).
    Rigid {
        joint: JointName,
        /// The model's transform relative to the joint: a socket offset, as
        /// engine sockets carry one. A negative determinant mirrors the part
        /// (one model serves both sides); the cook then flips the triangle
        /// winding. Usually [`Affine3A::IDENTITY`].
        offset: Affine3A,
    },
}

impl PartKind {
    /// A rigid part at the joint itself (no offset).
    pub fn rigid(joint: impl Into<JointName>) -> Self {
        Self::Rigid { joint: joint.into(), offset: Affine3A::IDENTITY }
    }
}

/// One visible piece of a character: a skin part, a head, hair, or an
/// item's mesh.
#[derive(Clone, Debug, PartialEq)]
pub struct BodyPart {
    pub model: VfsPath,
    /// The slot it fills when used as a base part.
    pub slot: SlotId,
    pub kind: PartKind,
    /// When set, only the geometry under a node whose name (or an
    /// ancestor's) contains this text, ASCII case-insensitive, belongs to
    /// the part: one model file may hold several parts (both sides of a
    /// limb, a head and its hair), as one glTF file holds several named
    /// meshes. `None`: the whole model.
    pub node_filter: Option<String>,
}

impl BodyPart {
    pub const KIND: &'static str = "sf.body_part";

    /// Whether geometry under a node path (root first, or any order: every
    /// name is checked) belongs to the part ([`Self::node_filter`]).
    pub fn keeps_node<'a>(&self, path: impl IntoIterator<Item = &'a str>) -> bool {
        let Some(filter) = &self.node_filter else { return true };
        let filter = filter.to_ascii_lowercase();
        path.into_iter().any(|name| name.to_ascii_lowercase().contains(&filter))
    }
}

/// Draw order of worn items: higher layers are resolved later, so their
/// parts replace lower ones in shared slots. An open scale; the constants
/// are the conventional bands.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Layer(pub u8);

impl Layer {
    pub const SKIN: Self = Self(0);
    pub const UNDER: Self = Self(50);
    pub const CLOTHING: Self = Self(100);
    pub const ARMOR: Self = Self(150);
    pub const OUTER: Self = Self(200);
}

/// An item's look when worn.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Wearable {
    /// Parts by slot. One item may fill several slots (a cuirass: chest,
    /// abdomen, both shoulders).
    pub parts: Vec<(SlotId, PartRef)>,
    /// Slots cleared before this item's parts are placed (a robe clears the
    /// legs, a helmet the hair).
    pub hides: Vec<SlotId>,
    pub layer: Layer,
    /// Parts for the first-person view model (gauntlets on the arms rig).
    /// Empty: the item is invisible in first person.
    pub first_person: Vec<(SlotId, PartRef)>,
}

impl Wearable {
    pub const KIND: &'static str = "sf.wearable";

    /// Slots the item fills (gameplay can refuse or swap items that share
    /// slots, as biped-slot games do; resolution itself just layers them).
    pub fn occupied(&self) -> impl Iterator<Item = SlotId> + '_ {
        self.parts.iter().map(|(s, _)| *s)
    }

    /// Whether two items fill a common slot.
    pub fn overlaps(&self, other: &Self) -> bool {
        self.occupied().any(|s| other.occupied().any(|t| t == s))
    }
}

/// Which hand holds an item (handedness is the content's: a left-handed
/// character's main hand is its left).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Hand {
    /// The weapon hand.
    #[default]
    Main,
    /// The shield or torch hand.
    Off,
}

/// Where an item is attached: a socket (joint) of the skeleton and the
/// model's transform relative to it, as engine sockets carry one (a UE
/// socket's relative transform, a Unity attach-point child). A negative
/// determinant mirrors the model, as for [`PartKind::Rigid`].
#[derive(Clone, Debug, PartialEq)]
pub struct Mount {
    pub socket: Socket,
    pub offset: Affine3A,
}

impl Mount {
    /// A mount at the socket itself (no offset).
    pub fn at(socket: impl Into<Socket>) -> Self {
        Self { socket: socket.into(), offset: Affine3A::IDENTITY }
    }
}

/// Whether a character's weapon is out, as gameplay drives it (drawn, or
/// sheathed: idle, casting, talking). Not part of the recipe: toggling it
/// moves held items between their mounts without re-resolving anything.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Readiness {
    #[default]
    Sheathed,
    Drawn,
}

/// A light an item carries (a torch, a lantern): attached at a node of the
/// item's model, spawned and removed with the item.
#[derive(Clone, Debug, PartialEq)]
pub struct HeldLight {
    /// Node of the item's model the light sits at (the flame). `None` or
    /// not found: the model's bounds center.
    pub attach_node: Option<String>,
    /// sRGB color, 0..=1.
    pub color: Vec3,
    /// Reach in meters (the content's radius; the host calibrates intensity).
    pub radius: f32,
}

/// An item held at a socket: a weapon in hand, a shield on the arm, a torch,
/// a sheathed blade, a quiver. Rigid, never skinned; spawned as a child of
/// its socket joint (it changes often and later carries physics, trails and
/// lights), never merged into the outfit, so it is not in the outfit key.
#[derive(Clone, Debug, PartialEq)]
pub struct HeldItem {
    /// The item record (gameplay, debugging, adapters' own lookups).
    pub item: RecordId,
    pub model: VfsPath,
    pub hand: Hand,
    /// Where the item is while the weapon is drawn. `None`: hidden while
    /// drawn (an off-hand item next to a drawn two-handed weapon).
    pub drawn: Option<Mount>,
    /// Where it is while sheathed. `None`: hidden while sheathed (games
    /// without visible sheaths show a put-away weapon nowhere).
    pub sheathed: Option<Mount>,
    /// A light the item carries.
    pub light: Option<HeldLight>,
}

impl HeldItem {
    /// An item at one mount whatever the readiness (a shield, a torch).
    pub fn always(item: RecordId, model: VfsPath, hand: Hand, mount: Mount) -> Self {
        Self { item, model, hand, drawn: Some(mount.clone()), sheathed: Some(mount), light: None }
    }

    /// Where the item shows at a readiness (`None`: hidden).
    pub fn mount(&self, readiness: Readiness) -> Option<&Mount> {
        match readiness {
            Readiness::Drawn => self.drawn.as_ref(),
            Readiness::Sheathed => self.sheathed.as_ref(),
        }
    }
}

/// Material parameters for skin and hair (linear RGBA tints).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TintParams {
    pub skin: Vec4,
    pub hair: Vec4,
}

impl Default for TintParams {
    fn default() -> Self {
        Self { skin: Vec4::ONE, hair: Vec4::ONE }
    }
}

/// The first-person view model: an arms rig with its own parts and graph.
#[derive(Clone, Debug, PartialEq)]
pub struct FirstPersonRecipe {
    pub skeleton: VfsPath,
    pub rig: Option<RigMapRef>,
    /// Base arm parts (skin), resolved like the body with worn items'
    /// [`Wearable::first_person`] parts on top.
    pub parts: Vec<PartRef>,
    pub graph: AnimGraphRef,
    /// Vertical field of view of the view-model camera, degrees.
    pub fov_y_deg: f32,
}

/// Everything needed to build one character.
#[derive(Clone, Debug, PartialEq)]
pub struct CharacterRecipe {
    pub skeleton: VfsPath,
    /// A humanoid map or, for creatures, only a rig profile.
    pub rig: RigRef,
    /// Skin parts (from species and gender).
    pub base: Vec<PartRef>,
    /// Head and hair are base parts too; kept apart so adapters and UIs can
    /// swap them. A creature may have neither.
    pub head: Option<PartRef>,
    pub hair: Option<PartRef>,
    /// Worn items in equip order (ties within a layer go to the later one).
    pub worn: Vec<WearableRef>,
    pub held: Vec<HeldItem>,
    /// Root scale; non-uniform allowed (height on Y, build on X and Z).
    pub scale: Vec3,
    pub graph: AnimGraphRef,
    pub tint: TintParams,
    /// Weights of the parts' shape morph targets (sliders), baked into the
    /// meshes at assembly; empty for content without shape targets.
    pub shape: crate::assemble::ShapeWeights,
    pub first_person: Option<FirstPersonRecipe>,
}

/// Registers the record kinds this crate owns: [`BodyPart`] and [`Wearable`].
pub fn register_kinds(db: &mut RecordDb) -> Result<(), RecordError> {
    db.register::<BodyPart>(BodyPart::KIND)?;
    db.register::<Wearable>(Wearable::KIND)?;
    Ok(())
}
