//! Definitions (interaction.md "Definitions", "Slot kinds"): `sf.interact.def` records, plain data,
//! mod-patchable. Behaviour of each slot kind (door swing, lever signals, ladder docking, job loops) is
//! a later slice; here they are data only.

use glam::{Quat, Vec3};
use serde::{Deserialize, Serialize};
use sf_records::RecordId;
use sf_world::PlacementId;
use smallvec::SmallVec;

use crate::light::LightDef;

/// 32-bit FNV-1a, the same hash `sf_action` keys its tags with, so a tag hashed here and there
/// is the same value on every peer.
#[must_use]
pub const fn fnv1a32(bytes: &[u8]) -> u32 {
    let mut h: u32 = 0x811c_9dc5;
    let mut i = 0;
    while i < bytes.len() {
        h ^= bytes[i] as u32;
        h = h.wrapping_mul(0x0100_0193);
        i += 1;
    }
    h
}

/// A tag (`"sit"`, `"lit"`, `"seat"`, `"wood"`) as a small stable key.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TagId(pub u32);

impl TagId {
    #[must_use]
    pub const fn of(id: &str) -> Self {
        Self(fnv1a32(id.as_bytes()))
    }
}

/// Which tag sets pass: every `all` tag present, at least one `any` tag present (when `any` is not
/// empty), no `none` tag present. The empty filter passes everything.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TagFilter {
    pub all: SmallVec<[TagId; 2]>,
    pub any: SmallVec<[TagId; 2]>,
    pub none: SmallVec<[TagId; 2]>,
}

impl TagFilter {
    #[must_use]
    pub fn matches(&self, tags: &[TagId]) -> bool {
        self.all.iter().all(|t| tags.contains(t))
            && (self.any.is_empty() || self.any.iter().any(|t| tags.contains(t)))
            && !self.none.iter().any(|t| tags.contains(t))
    }
}

/// A frame in the object's frame (a slot's pelvis/root target, a grip).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Frame {
    pub translation: Vec3,
    pub rotation: Quat,
}

impl Default for Frame {
    fn default() -> Self {
        Self { translation: Vec3::ZERO, rotation: Quat::IDENTITY }
    }
}

/// One interactable base object: its slots, slotless verbs and optional carry and destruction data.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InteractableDef {
    pub id: RecordId,
    pub slots: Vec<SlotDef>,
    /// Slotless uses: carry, take, break, read, the pack's defaults.
    pub verbs: SmallVec<[VerbDef; 2]>,
    /// (b) destruction, interaction-destruction.md.
    pub destructible: Option<DestructibleDef>,
    /// (a) carrying, interaction-carry.md.
    pub carry: Option<CarryDef>,
    /// What using it satisfies (rest, social, work), Sims-style: (need tag, value).
    pub adverts: Vec<(TagId, f32)>,
}

impl InteractableDef {
    /// The first door slot: its index, the slot (its `local` is the doorway) and the door data.
    #[must_use]
    pub fn door(&self) -> Option<(u8, &SlotDef, &DoorDef)> {
        self.slots.iter().enumerate().find_map(|(i, s)| match &s.kind {
            SlotKind::Door(d) => Some((u8::try_from(i).ok()?, s, d)),
            _ => None,
        })
    }
}

/// One place on an object a body uses (a seat, a doorway, a ladder, a stool at a workstation).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SlotDef {
    pub kind: SlotKind,
    /// Slot frame in the object's frame (pelvis/root target; door: the doorway).
    pub local: Frame,
    /// Seat/bed/top height, m (picks the clip band).
    pub band: f32,
    pub verb: VerbDef,
    pub entrances: SmallVec<[Entrance; 4]>,
    pub clips: ClipFamily,
    pub ik: IkTargets,
    /// Hand claims allowed within this radius while Occupied (a seat at a table), m; 0 = none.
    pub reach: f32,
    pub occupancy: Occupancy,
    /// (c) mounts and vehicles.
    pub control: Option<Control>,
    /// Who may use it (body type, size class, player/NPC, armed), UE user tags.
    pub users: TagFilter,
    /// Exclusion groups as a bit mask: two slots of one object exclude each other when their masks
    /// share a bit (a bench's middle `0b11` excludes both ends, `0b01` and `0b10`, which don't
    /// exclude each other). `None` = excludes nothing.
    pub group: Option<u8>,
    /// Ticket capacity; `None` = the kind's default (door, workstation: `interact.queue.capacity`;
    /// others: no queue). `Some(0)` = no tickets.
    pub queue: Option<u8>,
}

impl SlotDef {
    /// Whether this kind queues by default when the def gives no capacity.
    #[must_use]
    pub fn queues_by_default(&self) -> bool {
        matches!(self.kind, SlotKind::Door(_) | SlotKind::Workstation(_))
    }
}

/// A use as the prompt shows it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerbDef {
    pub verb: TagId,
    /// Focus order (higher first).
    pub priority: i16,
    /// State tags the object must have (`Lit` for snuff).
    pub requires: TagFilter,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SlotKind {
    /// Seat, bed, lean, counter, idle; saddle and vehicle seat in (c).
    Pose { family: TagId },
    Door(DoorDef),
    Lever(LeverDef),
    Ladder(LadderDef),
    Workstation(WorkstationDef),
    /// Snuff/relight a light-bearing object ((d) World rules, IN28).
    Light(LightDef),
}

/// A hinged leaf (IN33 builds the swing).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DoorDef {
    /// Hinge point in the object's frame.
    pub hinge: Vec3,
    /// Hinge axis in the object's frame (unit).
    pub axis: Vec3,
    /// Open angle, degrees.
    pub open_deg: f32,
    /// Swings away from the user either way; `false` = one way.
    pub both_ways: bool,
    /// Closes behind a user that found it closed.
    pub close_behind: bool,
}

/// Positions and linked targets (IN34 builds the toggle).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LeverDef {
    /// Number of positions (2 = on/off).
    pub positions: u8,
    pub links: SmallVec<[LeverLink; 2]>,
}

/// A target a lever signals.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LeverLink {
    pub target: PlacementId,
    pub signal: TagId,
}

/// A climbable edge chain (IN43 builds the docking).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LadderDef {
    /// Bottom-to-top points in the object's frame (an edge chain, at least two).
    pub chain: SmallVec<[Vec3; 4]>,
    /// Rung spacing, m.
    pub rung_spacing: f32,
}

/// A pose slot plus a job set (IN42 builds the job loop).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkstationDef {
    /// Which job records run here.
    pub jobs: TagFilter,
    /// The pack's menu the player's use opens (alchemy, repair).
    pub player_verb: Option<TagId>,
}

/// Which side an entrance is on; the declaration order is the tie order (Front, Left, Right, Back,
/// then Bottom, Top for ladders).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Side {
    Front,
    Left,
    Right,
    Back,
    Bottom,
    Top,
}

/// A way in and out of a slot; its enter/exit clips are `<clips>.enter.<side>` / `<clips>.exit.<side>`.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Entrance {
    pub side: Side,
    /// Where the user stands to start the enter clip, in the object's frame.
    pub approach: Vec3,
}

/// The clip slot prefix (`"seat.chair"`): slots `<prefix>.{enter.<side>,loop,exit.<side>}`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClipFamily(pub String);

/// Contact points in the object's frame.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IkTargets {
    /// Armrests, table edge, rungs, reins.
    pub hands: SmallVec<[Vec3; 2]>,
    /// Floor, stirrups.
    pub feet: SmallVec<[Vec3; 2]>,
    pub look: Option<Vec3>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Occupancy {
    /// Aligned to the slot, mover off.
    #[default]
    Pose,
    /// The rider follows a moving owner ((c)).
    Attach,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Control {
    Driver,
    Passenger,
}

/// (a) How an object is carried (interaction-carry.md).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CarryDef {
    pub grips: SmallVec<[Grip; 2]>,
    /// A [`CarryClass`] record.
    pub class: RecordId,
    /// May be moved and persist as a moved placement.
    pub relocatable: bool,
    /// What it pairs with when relocated (a table edge).
    pub pairs_with: TagFilter,
}

/// A hand hold on a carried object, in its frame.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Grip {
    pub local: Frame,
}

/// `sf.interact.carry_class` record, mod data.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CarryClass {
    pub id: RecordId,
    /// `carry.one_hand`, `carry.two_hand`, `carry.shoulder`, `drag.chair`.
    pub clips: ClipFamily,
    pub hands: u8,
    pub drag: bool,
    /// Speed multiplier, ratio 0.2-1.0.
    pub speed_mul: f32,
    pub can_sprint: bool,
    pub can_roll: bool,
}

/// (b) Destructible props (interaction-destruction.md); types only.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DestructibleDef {
    pub health: f32,
    /// Wood, clay, glass, stone.
    pub material: TagId,
    pub stages: SmallVec<[Stage; 4]>,
    pub ignore_external: bool,
    /// Rolled on break: the pack's levelled list (adapter).
    pub loot: Option<RecordId>,
    /// A barrel yields planks: (item record, count).
    pub salvage: SmallVec<[(RecordId, u16); 2]>,
}

/// One damage stage.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Stage {
    /// Health fraction at which this stage starts.
    pub at: f32,
    /// Model record; `None` removes the object.
    pub model: Option<RecordId>,
    /// Debris set records.
    pub debris: SmallVec<[RecordId; 4]>,
    /// Tier 2 pre-fractured chunk set record.
    pub chunks: Option<RecordId>,
    pub effect: Option<RecordId>,
    pub on_break: BreakAction,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum BreakAction {
    SpillContents,
    Loot,
    #[default]
    None,
}
