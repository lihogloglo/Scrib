//! Census furniture to [`InteractableDef`] (interaction-adapter.md "Defs from the census"): a pure
//! mapping from a [`Kind`] and its [`Measured`] shape. The bake (`IN32b`) fills [`Measured`] from the
//! census shapes; the shell (`IN32c`) indexes the defs.
//!
//! Conventions (interaction.md "Using a slot", `sf_interaction_bevy::seat`): positions are in the
//! object's frame, metres, Y up; a slot frame is the user's **root on the floor** (y = 0; `band`
//! carries the surface height and picks the clip band) and its +Z is the way the user faces; an
//! entrance's approach point is on the floor where the enter clip starts. Pose families and verbs
//! are the IN2 tags: family `seat` / `bed` / `counter`, verb `sit` / `lie` / `lean`
//! ([`crate::activation::verb`]); clip prefixes `seat.chair`, `seat.bench`, `seat.bed_edge`,
//! `bed.lie`, `counter.table`, `counter.bar`.
//!
//! - **Seat** ([`Kind::Seat`]): places of [`FurnitureParams::bench_seat_m`] along the axis across
//!   its front (at least one; a chair is a bench of one). A bench of `n` > 1 places also gets the
//!   `n - 1` places between them, each overlapping its two neighbours: slot `k` of the `2n - 1`
//!   (left to right) has exclusion bits `k - 1` and `k` (a two-place bench: `0b01`, `0b11`,
//!   `0b10`, def.rs's example). Benches too wide for the 8-bit mask (more than 5 places) get the
//!   whole places only, which don't overlap. One entrance, Front.
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

use glam::{Quat, Vec3};
use serde::{Deserialize, Serialize};
use sf_interaction::{
    ClipFamily, Entrance, Frame, IkTargets, InteractableDef, Occupancy, Side, SlotDef, SlotKind, TagFilter, TagId,
    VerbDef,
};
use sf_records::RecordId;
use smallvec::{SmallVec, smallvec};

use crate::activation::verb;
use crate::furniture::Kind;
use crate::tunables::FurnitureParams;

/// Float tolerance when fitting places into a length, m.
// tunables: const (maths: rounding tolerance so 2 x 0.55 fits in 1.10, not a feel value)
const FIT_EPS: f32 = 1.0e-4;

/// Exclusion mask width (`SlotDef::group` is a `u8`).
const MASK_BITS: u8 = 8;

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
    /// Height of the usable top surface (seat, bed, table top) above the floor, m.
    pub top: f32,
    /// The side it is used from: a seat faces it, a table or counter is served across it (a counter's
    /// stations stand on this side and the opposite one). Beds don't read it ([`crate::measure()`]
    /// sets it to `head`).
    pub front: Facing,
    /// A bed's head end: its `lie` slot's +Z. Seats, tables, counters and shelves don't read it
    /// ([`crate::measure()`] sets it to `front`).
    pub head: Facing,
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
            center: Vec3::new(m.center[0], 0.0, m.center[1]),
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
        verb: VerbDef { verb: TagId::of(verb), priority: 0, requires: TagFilter::default() },
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
    if !(sane(m.footprint[0]) && sane(m.footprint[1]) && sane(m.top) && m.center.iter().all(|c| c.is_finite())) {
        return None;
    }
    let b = Box2::of(m, m.front);
    let slots = match kind {
        Kind::Seat => seat_slots(&b, m.top, p),
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

fn seat_slots(b: &Box2, top: f32, p: &FurnitureParams) -> Vec<SlotDef> {
    let width = b.half_width * 2.0;
    let whole = places(width, p.bench_seat_m);
    let clips = if whole == 1 { "seat.chair" } else { "seat.bench" };
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
