//! Census furniture to [`InteractableDef`] (interaction-adapter.md "Defs from the census"): a pure
//! mapping from a [`Kind`] and its [`Measured`] shape. The bake (`IN32b`) fills [`Measured`] from the
//! census shapes; the shell (`IN32c`) indexes the defs.
//!
//! Conventions (interaction.md "Using a slot", `sf_interaction_bevy::seat`): positions are in the
//! object's frame, metres, Y up; a slot frame is the user's **root on the floor** (y = 0; `band`
//! carries the surface height and picks the clip band) and its +Z is the way the user faces; an
//! entrance's approach point is on the floor where the enter clip starts. Pose families and verbs
//! are the IN2 tags: family `seat` / `bed` / `counter`, verb `sit` / `lie` / `lean`
//! ([`crate::activation::verb`]); clip families `seat.chair`, `seat.stool`, `seat.bench`,
//! `seat.floor`, `seat.bed_edge`, `bed.lie`, `counter.table`, `counter.bar` (the anim contract's
//! `<family>[.<style>].b<band cm>.{enter.<side>, loop, loop.fidget, exit.<side>}`: the animation
//! side resolves a slot's family, band and sides to its nearest clip set).
//!
//! - **Seat** ([`Kind::Seat`]): places of [`FurnitureParams::bench_seat_m`] along the axis across
//!   its front (at least one; a chair is a bench of one). A bench of `n` > 1 places also gets the
//!   `n - 1` places between them, each overlapping its two neighbours: slot `k` of the `2n - 1`
//!   (left to right) has exclusion bits `k - 1` and `k` (a two-place bench: `0b01`, `0b11`,
//!   `0b10`, def.rs's example). Benches too wide for the 8-bit mask (more than 5 places) get the
//!   whole places only, which don't overlap. One entrance, Front. Family: `seat.floor` when the top
//!   is under [`FurnitureParams::floor_seat_band_m`] above the floor (strictly: a top exactly at it
//!   is not a floor seat; floor wins over every other seat family), else one place with a backrest
//!   ([`Basis::Upright`]) `seat.chair`, one place without `seat.stool`, more places `seat.bench`.
//! - **Bed** (anim contract AN25, motion-generation.md "Interaction families"): a `lie` slot at the
//!   centre whose +Z points to the head ([`Measured::head`]): the body lies along +Z, face up. It is
//!   got into from the long sides, entrances Left and Right: the lier's own sides (the user's right
//!   is `Y x head`, +X for a head at +Z), clips `bed.lie.enter.left` / `.right`. Plus one edge `sit`
//!   (`seat.bed_edge`) per long side, facing out, entered from its Front. All three share exclusion
//!   bit 0; a side a wall blocks is dropped at run time by `EndPoseFree`, not here.
//! - **Table** (all four edges) and **counter** (front and back): standing `lean` stations, one per
//!   [`FurnitureParams::counter_spacing_m`] along each usable edge, facing the top, entered from
//!   behind (Back); `reach` covers the station's share of the top up to the centre line (hand
//!   claims on cups and plates). A side a wall blocks in a placement is dropped at run time by
//!   `EndPoseFree`, not here.
//! - **Shelf**: none (later `browse`).
//!
//! [`FurnitureParams::sit`] off (Classic) = no def for any kind: the furniture is inert, as in
//! vanilla.
//!
//! **Class verb** ([`class_verb`]): a def'd record of a class with its own primary use (a census
//! `CONT` opens, an `ACTI` is used: [`crate::activation::primary`]) keeps it, as a slotless verb
//! above every pose verb ([`CLASS_VERB_PRIORITY`] over [`POSE_VERB_PRIORITY`]), so its best verb is
//! the class's and the pose verbs follow (the hold list). The bake adds it ([`def_for`] knows only
//! the census kind).
//!
//! **Swing door** ([`door_def`]): one `pass` doorway slot for NPC users ([`USER_NPC`]) plus the
//! slotless `open` / `close` the player toggles it with (`IN33c1`).

use glam::{Quat, Vec3};
use serde::{Deserialize, Serialize};
use sf_interaction::{
    ClipFamily, DOOR_OPEN, DoorDef, Entrance, Frame, IkTargets, InteractableDef, Occupancy, Side, SlotDef, SlotKind,
    TagFilter, TagId, VERB_CLOSE, VERB_OPEN, VerbDef,
};
use sf_records::RecordId;
use smallvec::{SmallVec, smallvec};

use crate::activation::{self, RecordClass, verb};
use crate::furniture::Kind;
use crate::tunables::FurnitureParams;

/// Float tolerance when fitting places into a length, m.
// tunables: const (maths: rounding tolerance so 2 x 0.55 fits in 1.10, not a feel value)
const FIT_EPS: f32 = 1.0e-4;

/// Exclusion mask width (`SlotDef::group` is a `u8`).
const MASK_BITS: u8 = 8;

/// Focus order of every pose slot's verb.
// tunables: const (ordering: the rank of the pose verbs among a def's verbs, not a feel value)
pub const POSE_VERB_PRIORITY: i16 = 0;

/// Focus order of a record class's own primary verb ([`class_verb`]): above every pose verb.
// tunables: const (ordering: the class's own use ranks first, interaction-adapter.md)
pub const CLASS_VERB_PRIORITY: i16 = POSE_VERB_PRIORITY + 1;

/// The user tag of every NPC user of a slot (`SlotDef::users`); the player has none.
pub const USER_NPC: &str = "npc";

/// A vanilla swing door's open angle, degrees (`OpenMW` `worldimp.cpp:1427-1440`).
// tunables: const (content convention: the record format's door turns 90 degrees, not a feel value)
const DOOR_OPEN_DEG: f32 = 90.0;

/// A horizontal direction in the object's frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Facing {
    PosX,
    NegX,
    PosZ,
    NegZ,
}

impl Facing {
    /// The unit vector.
    pub fn dir(self) -> Vec3 {
        match self {
            Self::PosX => Vec3::X,
            Self::NegX => Vec3::NEG_X,
            Self::PosZ => Vec3::Z,
            Self::NegZ => Vec3::NEG_Z,
        }
    }
}

/// A furniture model's measured shape, in its own frame (plain data: the bake fills it from the
/// census shapes).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Measured {
    /// Centre of the footprint (x, z), m.
    pub center: [f32; 2],
    /// Full extent of the footprint along x and z, m.
    pub footprint: [f32; 2],
    /// Height of the floor (the model's lowest point) in its frame, m: slot roots and approach points
    /// stand at this y, so a model authored with its origin above or below its floor still seats its
    /// user on the ground (`IN32b`). 0 for most vanilla furniture.
    pub floor: f32,
    /// Height of the usable top surface (seat, bed, table top) above the floor, m.
    pub top: f32,
    /// The side it is used from: a seat faces it, a table or counter is served across it (a counter's
    /// stations stand on this side and the opposite one). Beds don't read it ([`crate::measure()`]
    /// sets it to `head`).
    pub front: Facing,
    /// A bed's head end: its `lie` slot's +Z. Seats, tables, counters and shelves don't read it
    /// ([`crate::measure()`] sets it to `front`).
    pub head: Facing,
    /// Where `front` / `head` came from (reported by the bake; [`def_for`] reads it for a seat's
    /// family: a single place with a backrest is a chair, without one a stool).
    pub basis: Basis,
}

/// Which [`crate::measure()`] rule set a [`Measured`] facing.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Basis {
    /// Measured from the geometry: a seat's backrest, a bed's head- or footboard.
    Upright,
    /// The + side of the short axis: a long backless seat (fallback); every table, counter and shelf
    /// (their rule).
    LongSide,
    /// A short backless seat (stool; fallback): +Z.
    PlusZ,
    /// A bed with no clear head (bedroll, hammock; fallback): the + end of the long axis.
    LongEnd,
}

/// The footprint as a box seen from the front.
struct Box2 {
    center: Vec3,
    /// Front direction.
    front: Vec3,
    /// Across the front (`Y x front`).
    across: Vec3,
    /// Half extent along `front`, m.
    half_depth: f32,
    /// Half extent along `across`, m.
    half_width: f32,
}

impl Box2 {
    /// The footprint of `m` seen from `side` (its `front` for most kinds, its `head` for beds).
    fn of(m: &Measured, side: Facing) -> Self {
        let front = side.dir();
        let (half_depth, half_width) = match side {
            Facing::PosX | Facing::NegX => (m.footprint[0] / 2.0, m.footprint[1] / 2.0),
            Facing::PosZ | Facing::NegZ => (m.footprint[1] / 2.0, m.footprint[0] / 2.0),
        };
        Self {
            center: Vec3::new(m.center[0], m.floor, m.center[1]),
            front,
            across: Vec3::Y.cross(front),
            half_depth,
            half_width,
        }
    }

    /// Half extent along a unit axis of the box (`front`, `across` or their opposites).
    fn half_along(&self, axis: Vec3) -> f32 {
        if axis.dot(self.front).abs() > 0.5 { self.half_depth } else { self.half_width }
    }
}

/// A frame at `root` facing `dir` (yaw 0 faces +Z).
fn facing(root: Vec3, dir: Vec3) -> Frame {
    Frame { translation: root, rotation: Quat::from_rotation_y(dir.x.atan2(dir.z)) }
}

/// How many places of `room` fit along `len` (at least one, at most 255).
fn places(len: f32, room: f32) -> u8 {
    let mut n: u8 = 1;
    if room > 0.0 {
        while n < u8::MAX && f32::from(n + 1) * room <= len + FIT_EPS {
            n += 1;
        }
    }
    n
}

/// Offsets of `n` equal shares of `len`, centred on 0, at their middles.
fn shares(len: f32, n: u8) -> impl Iterator<Item = f32> {
    let pitch = len / f32::from(n);
    (0..n).map(move |i| -len / 2.0 + pitch * (f32::from(i) + 0.5))
}

/// A pose slot with no entrance, reach or group yet.
fn pose(family: &str, clips: &str, verb: &str, local: Frame, band: f32) -> SlotDef {
    SlotDef {
        kind: SlotKind::Pose { family: TagId::of(family) },
        local,
        band,
        verb: VerbDef { verb: TagId::of(verb), priority: POSE_VERB_PRIORITY, requires: TagFilter::default() },
        entrances: SmallVec::new(),
        clips: ClipFamily(clips.to_owned()),
        ik: IkTargets::default(),
        reach: 0.0,
        occupancy: Occupancy::Pose,
        control: None,
        users: TagFilter::default(),
        group: None,
        queue: None,
    }
}

/// The def of a census furniture base record, or `None` (shelves; the `sit` switch off; a
/// degenerate shape: non-positive or non-finite footprint or top).
pub fn def_for(base: &RecordId, kind: Kind, m: &Measured, p: &FurnitureParams) -> Option<InteractableDef> {
    if !p.sit {
        return None;
    }
    let sane = |v: f32| v.is_finite() && v > 0.0;
    if !(sane(m.footprint[0])
        && sane(m.footprint[1])
        && sane(m.top)
        && m.floor.is_finite()
        && m.center.iter().all(|c| c.is_finite()))
    {
        return None;
    }
    let b = Box2::of(m, m.front);
    let slots = match kind {
        Kind::Seat => seat_slots(&b, m.top, m.basis, p),
        Kind::Bed => bed_slots(&Box2::of(m, m.head), m.top, p),
        Kind::Table => station_slots(&b, m.top, p, "counter.table", &[b.front, -b.front, b.across, -b.across]),
        Kind::Counter => station_slots(&b, m.top, p, "counter.bar", &[b.front, -b.front]),
        Kind::Shelf => return None,
    };
    Some(InteractableDef {
        id: base.clone(),
        slots,
        verbs: SmallVec::new(),
        destructible: None,
        carry: None,
        adverts: Vec::new(),
    })
}

/// The slot verb a def'd record of `class` keeps from its class: its primary offered row's verb
/// (`CONT` open, `ACTI` use) at [`CLASS_VERB_PRIORITY`], slotless; `None` for a class with no offered
/// unconditional row (furniture, a creature) or whose rows are all conditional.
#[must_use]
pub fn class_verb(class: RecordClass) -> Option<VerbDef> {
    if matches!(class, RecordClass::Furniture(_)) {
        return None;
    }
    activation::primary(class, |_| false, |_| true).map(|row| VerbDef {
        verb: TagId::of(row.verb),
        priority: CLASS_VERB_PRIORITY,
        requires: TagFilter::default(),
    })
}

/// The def of a Morrowind swing door (a `DOOR` without a destination); `doorway` is the closed
/// leaf's centre in the object's frame (the model bounds' centre).
///
/// Vanilla door meshes pivot at their origin and `OpenMW` turns the reference 90 degrees about its
/// vertical axis, one way (behaviour source: `apps/openmw/mwworld/worldimp.cpp:1427-1440`; no code
/// copied): hinge at the origin, axis +Y, one-way, closing behind a user that found it closed. The
/// player never claims the doorway: it toggles the door through the slotless `open` (closed) and
/// `close` (open, [`DOOR_OPEN`]); NPC users ([`USER_NPC`]) claim and queue on the `pass` slot.
#[must_use]
pub fn door_def(base: &RecordId, doorway: Vec3) -> InteractableDef {
    let pass = SlotDef {
        kind: SlotKind::Door(DoorDef {
            hinge: Vec3::ZERO,
            axis: Vec3::Y,
            open_deg: DOOR_OPEN_DEG,
            both_ways: false,
            close_behind: true,
        }),
        local: Frame { translation: doorway, rotation: Quat::IDENTITY },
        band: 0.0,
        verb: VerbDef { verb: TagId::of(verb::PASS), priority: POSE_VERB_PRIORITY, requires: TagFilter::default() },
        entrances: SmallVec::new(),
        clips: ClipFamily::default(),
        ik: IkTargets::default(),
        reach: 0.0,
        occupancy: Occupancy::Pose,
        control: None,
        users: TagFilter { all: smallvec![TagId::of(USER_NPC)], ..TagFilter::default() },
        group: None,
        // A door slot queues by default (`SlotDef::queues_by_default`).
        queue: None,
    };
    InteractableDef {
        id: base.clone(),
        slots: vec![pass],
        verbs: smallvec![
            VerbDef {
                verb: VERB_OPEN,
                priority: CLASS_VERB_PRIORITY,
                requires: TagFilter { none: smallvec![DOOR_OPEN], ..TagFilter::default() },
            },
            VerbDef {
                verb: VERB_CLOSE,
                priority: CLASS_VERB_PRIORITY,
                requires: TagFilter { all: smallvec![DOOR_OPEN], ..TagFilter::default() },
            },
        ],
        destructible: None,
        carry: None,
        adverts: Vec::new(),
    }
}

/// A seat's clip family (module docs, "Seat").
fn seat_family(whole: u8, top: f32, basis: Basis, p: &FurnitureParams) -> &'static str {
    if top < p.floor_seat_band_m {
        "seat.floor"
    } else if whole > 1 {
        "seat.bench"
    } else if basis == Basis::Upright {
        "seat.chair"
    } else {
        "seat.stool"
    }
}

fn seat_slots(b: &Box2, top: f32, basis: Basis, p: &FurnitureParams) -> Vec<SlotDef> {
    let width = b.half_width * 2.0;
    let whole = places(width, p.bench_seat_m);
    let clips = seat_family(whole, top, basis, p);
    // Whole places and the ones between them (half a pitch apart) while their masks fit.
    let between = whole > 1 && 2 * u16::from(whole) - 2 <= u16::from(MASK_BITS);
    let pitch = width / f32::from(whole);
    let (count, step) = if between { (2 * whole - 1, pitch / 2.0) } else { (whole, pitch) };
    let first = -width / 2.0 + pitch / 2.0;
    let edge = b.front * (b.half_depth + p.approach_m);
    (0..count)
        .map(|k| (usize::from(k), first + step * f32::from(k)))
        .map(|(k, off)| {
            let root = b.center + b.across * off;
            let mut slot = pose("seat", clips, verb::SIT, facing(root, b.front), top);
            slot.entrances = smallvec![Entrance { side: Side::Front, approach: root + edge }];
            slot.group = between.then(|| path_mask(k, usize::from(count)));
            slot
        })
        .collect()
}

/// Slot `k` of `count` in a row where each overlaps only its neighbours: bit `k - 1` (shared with
/// the left one) and bit `k` (shared with the right one).
fn path_mask(k: usize, count: usize) -> u8 {
    let left = if k > 0 { 1u8 << (k - 1) } else { 0 };
    let right = if k + 1 < count { 1u8 << k } else { 0 };
    left | right
}

/// `b` is the bed seen from its head (`b.front` = head, `b.across` = the lier's right).
fn bed_slots(b: &Box2, top: f32, p: &FurnitureParams) -> Vec<SlotDef> {
    let out = b.half_width + p.approach_m;
    let inset = (b.half_width - p.edge_seat_inset_m).max(0.0);
    let mut lie = pose("bed", "bed.lie", verb::LIE, facing(b.center, b.front), top);
    lie.entrances = smallvec![
        Entrance { side: Side::Left, approach: b.center - b.across * out },
        Entrance { side: Side::Right, approach: b.center + b.across * out },
    ];
    lie.group = Some(0b1);
    let mut slots = vec![lie];
    // The edge seats, left then right, each facing out of its long side.
    for side in [-b.across, b.across] {
        let root = b.center + side * inset;
        let mut edge = pose("seat", "seat.bed_edge", verb::SIT, facing(root, side), top);
        edge.entrances = smallvec![Entrance { side: Side::Front, approach: b.center + side * out }];
        edge.group = Some(0b1);
        slots.push(edge);
    }
    slots
}

/// Standing stations along each of `edges` (outward unit normals), facing the top.
fn station_slots(b: &Box2, top: f32, p: &FurnitureParams, clips: &str, edges: &[Vec3]) -> Vec<SlotDef> {
    let mut slots = Vec::new();
    for &out in edges {
        let along = Vec3::Y.cross(out);
        let len = b.half_along(along) * 2.0;
        let to_edge = b.half_along(out);
        let count = places(len, p.counter_spacing_m);
        let reach = (to_edge + p.counter_stand_m).hypot(len / f32::from(count) / 2.0);
        for off in shares(len, count) {
            let root = b.center + out * (to_edge + p.counter_stand_m) + along * off;
            let mut slot = pose("counter", clips, verb::LEAN, facing(root, -out), top);
            slot.entrances = smallvec![Entrance { side: Side::Back, approach: root + out * p.approach_m }];
            slot.reach = reach;
            slots.push(slot);
        }
    }
    slots
}
