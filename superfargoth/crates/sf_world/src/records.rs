//! Record kinds owned by this crate: worldspaces and door links. Adapters
//! define them from their own formats; mods patch them like any record.

use std::f64::consts::PI;
use std::fmt;
use std::sync::Arc;

use glam::{DQuat, IVec3, Vec2, Vec3};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sf_coords::{WorldPos, WorldTransform, partition_extent, partition_origin};
use sf_records::RecordId;

use crate::rings::RingRadii;
use crate::{PlacementId, WorldspaceId};

/// Shift between streaming cells and index pages: a page is 8×8×8 cells
/// (1 km with 128 m cells).
pub(crate) const PAGE_SHIFT: i32 = 3;

/// How a worldspace is cut into streaming cells.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Partition {
    /// Cells of edge `GRID_CELL · 2^shift` (shift -1 = 128 m), aligned with
    /// the coordinate grid.
    Grid { shift: i32 },
    /// One cell holding everything: the worldspace loads whole (small
    /// interiors). Its key is `IVec3::ZERO`, its origin `WorldPos::ORIGIN`.
    Single,
}

impl Default for Partition {
    fn default() -> Self {
        Self::STREAMING
    }
}

impl Partition {
    /// The default: 128 m streaming cells.
    pub const STREAMING: Self = Self::Grid { shift: -1 };

    /// The streaming cell containing `pos`.
    pub fn cell_of(self, pos: WorldPos) -> IVec3 {
        match self {
            Self::Grid { shift } => pos.partition(shift),
            Self::Single => IVec3::ZERO,
        }
    }

    /// The minimum corner of `cell`: the frame cell bounds are stored in.
    pub fn cell_origin(self, cell: IVec3) -> WorldPos {
        match self {
            Self::Grid { shift } => partition_origin(cell, shift),
            Self::Single => WorldPos::ORIGIN,
        }
    }

    /// Cell edge length in meters, `None` for [`Partition::Single`].
    pub fn cell_extent(self) -> Option<f32> {
        match self {
            Self::Grid { shift } => Some(partition_extent(shift)),
            Self::Single => None,
        }
    }

    /// The index page holding `cell`.
    pub fn page_of(cell: IVec3) -> IVec3 {
        IVec3::new(cell.x >> PAGE_SHIFT, cell.y >> PAGE_SHIFT, cell.z >> PAGE_SHIFT)
    }
}

/// Which worldspaces get ray-tracing mesh variants (gi.md "Scene and
/// lights"): the ray tracer needs its own uncompressed copy of each mesh,
/// so a renderer that traces only small worldspaces (probe GI indoors)
/// spares the streamed ones that memory and BLAS cost.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RayTracedWorldspaces {
    /// Nothing is ray traced.
    #[default]
    None,
    /// Only worldspaces that load whole ([`Partition::Single`]).
    Single,
    /// Every worldspace.
    All,
}

impl RayTracedWorldspaces {
    /// The LOD level to build ray-tracing variants from for a model loaded
    /// into a worldspace of `partition`, or `None` for no variants.
    pub fn variant_lod(self, partition: Partition, lod: usize) -> Option<usize> {
        let traced = match self {
            Self::None => false,
            Self::Single => partition == Partition::Single,
            Self::All => true,
        };
        traced.then_some(lod)
    }
}

/// A reference to another record: its kind name plus id.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RecordRef {
    pub kind: Arc<str>,
    pub id: RecordId,
}

impl RecordRef {
    pub fn new(kind: &str, id: impl Into<RecordId>) -> Self {
        Self { kind: kind.into(), id: id.into() }
    }
}

impl fmt::Debug for RecordRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.kind, self.id)
    }
}

#[derive(Serialize, Deserialize)]
struct RecordRefRepr<'a> {
    kind: std::borrow::Cow<'a, str>,
    id: std::borrow::Cow<'a, str>,
}

impl Serialize for RecordRef {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        RecordRefRepr { kind: (&*self.kind).into(), id: self.id.as_str().into() }.serialize(s)
    }
}

impl<'de> Deserialize<'de> for RecordRef {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let r = RecordRefRepr::deserialize(d)?;
        Ok(Self { kind: (&*r.kind).into(), id: RecordId::new(&r.id) })
    }
}

/// Per-worldspace ring radius overrides (an interior might stream its whole
/// content at R0, a vast desert might push R2 out).
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RingOverrides {
    #[serde(default)]
    pub active: Option<RingRadii>,
    #[serde(default)]
    pub near: Option<RingRadii>,
    #[serde(default)]
    pub far: Option<RingRadii>,
    #[serde(default)]
    pub horizon: Option<f32>,
}

/// A worldspace record: one coordinate space with its own streaming grid.
/// Interiors are worldspaces too.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Worldspace {
    pub name: String,
    #[serde(default)]
    pub partition: Partition,
    #[serde(default)]
    pub rings: Option<RingOverrides>,
    #[serde(default)]
    pub has_terrain: bool,
    /// Until water bodies replace it.
    #[serde(default)]
    pub sea_level: Option<f32>,
    /// Ambient, fog, and sky preset. `None` uses the game's default.
    #[serde(default)]
    pub environment: Option<RecordRef>,
}

impl Worldspace {
    pub const KIND: &'static str = "sf.worldspace";

    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            partition: Partition::default(),
            rings: None,
            has_terrain: false,
            sea_level: None,
            environment: None,
        }
    }
}

/// One end of a door link.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkEnd {
    pub worldspace: WorldspaceId,
    /// The portal frame: on the door plane, +Z pointing into this
    /// worldspace. Its scale is ignored (portals are rigid).
    pub frame: WorldTransform,
    /// Where a traveler appears when arriving at this end.
    pub arrival: WorldTransform,
    /// The door object, if any.
    #[serde(default)]
    pub door: Option<PlacementId>,
    /// Portal width and height in meters; zero is a point link (teleport).
    #[serde(default)]
    pub extent: Vec2,
}

/// A link between two worldspaces (or two places in one).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DoorLink {
    pub a: LinkEnd,
    pub b: LinkEnd,
    #[serde(default = "yes")]
    pub two_way: bool,
}

fn yes() -> bool {
    true
}

impl DoorLink {
    pub const KIND: &'static str = "sf.door_link";

    /// The ends a traveler can leave from, each paired with where it leads.
    pub fn exits(&self) -> impl Iterator<Item = (&LinkEnd, &LinkEnd)> {
        std::iter::once((&self.a, &self.b)).chain(self.two_way.then_some((&self.b, &self.a)))
    }

    /// Maps a pose leaving through `from` to the pose entering through `to`:
    /// `to.frame · R_y(π) · from.frame⁻¹`, the standard portal transform. The
    /// half-turn makes stepping out behind `from`'s plane (−Z) arrive in
    /// front of `to`'s plane (+Z), facing on through. Exact for poses near
    /// the portal; the relative offset is computed in f64.
    pub fn cross(from: &LinkEnd, to: &LinkEnd, pose: &WorldTransform) -> WorldTransform {
        let to_rot = to.frame.rotation.as_dquat();
        let rel = from.frame.rotation.as_dquat().inverse() * from.frame.pos.delta(pose.pos);
        let out = to_rot * (DQuat::from_rotation_y(PI) * rel);
        let rotation = (to.frame.rotation
            * half_turn()
            * from.frame.rotation.inverse()
            * pose.rotation)
            .normalize();
        WorldTransform { pos: to.frame.pos.offset(out.as_vec3()), rotation, scale: pose.scale }
    }

    /// This link's stable identity: what routes, saves and the wire name it
    /// by. Hashes both ends' worldspace and frame (see [`LinkId`]); arrival
    /// poses, doors and extents don't take part, so retuning them keeps
    /// saved routes valid.
    pub fn id(&self) -> LinkId {
        let (a, b) = (end_key(&self.a), end_key(&self.b));
        // A two-way link has no direction: order its ends canonically.
        let (first, second) = if self.two_way && b < a { (b, a) } else { (a, b) };
        let mut h = blake3::Hasher::new();
        h.update(LINK_ID_DOMAIN);
        h.update(&[u8::from(self.two_way)]);
        h.update(&first);
        h.update(&second);
        let bytes: [u8; 8] = h.finalize().as_bytes()[..8].try_into().expect("8-byte prefix");
        LinkId(u64::from_le_bytes(bytes))
    }

    /// The link named `id` in `links`: a linear scan hashing each link, fine
    /// for occasional lookups (route resolve, save load); keep an index (the
    /// world graph's) for anything per tick.
    pub fn find(links: &[DoorLink], id: LinkId) -> Option<&DoorLink> {
        links.iter().find(|link| link.id() == id)
    }

    /// [`DoorLink::cross`] for a direction (velocity, view direction).
    pub fn cross_direction(from: &LinkEnd, to: &LinkEnd, dir: Vec3) -> Vec3 {
        to.frame.rotation * (half_turn() * (from.frame.rotation.inverse() * dir))
    }
}

/// A door link's stable identity: blake3 over both ends' worldspace id and
/// portal frame, truncated to 64 bits. Frames are quantised first (position
/// to [`LINK_ID_POS_STEP`] in absolute coordinates, rotation to
/// [`LINK_ID_ROT_STEP`] per unit-quaternion component, sign canonical), so
/// float noise from a re-bake keeps the id. A two-way link's id ignores the
/// order of its ends; a one-way link's doesn't. The ids are part of the save
/// and wire formats: changing the hash or the steps changes every id.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LinkId(pub u64);

/// Position quantum of [`LinkId`], meters (1 mm). Format, not a tunable.
pub const LINK_ID_POS_STEP: f64 = 1.0e-3;
/// Rotation quantum of [`LinkId`], per quaternion component. Format, not a
/// tunable.
pub const LINK_ID_ROT_STEP: f64 = 1.0e-4;
/// Domain tag, so a link id never equals another hash of the same bytes.
const LINK_ID_DOMAIN: &[u8] = b"sf.door_link.id/1";

impl fmt::Debug for LinkId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:016x}", self.0)
    }
}

impl fmt::Display for LinkId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

/// Hex string in text formats (like [`PlacementId`]), a plain `u64` in
/// binary ones.
impl Serialize for LinkId {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        if s.is_human_readable() { s.collect_str(self) } else { s.serialize_u64(self.0) }
    }
}

impl<'de> Deserialize<'de> for LinkId {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        if d.is_human_readable() {
            let s = String::deserialize(d)?;
            u64::from_str_radix(&s, 16).map(Self).map_err(serde::de::Error::custom)
        } else {
            u64::deserialize(d).map(Self)
        }
    }
}

/// One end's hashed bytes: length-prefixed worldspace id, then the quantised
/// position (3 × i64) and rotation (4 × i64, w x y z), little-endian.
fn end_key(end: &LinkEnd) -> Vec<u8> {
    let name = end.worldspace.as_str().as_bytes();
    let mut out = Vec::with_capacity(8 + name.len() + 7 * 8);
    out.extend_from_slice(&(name.len() as u64).to_le_bytes());
    out.extend_from_slice(name);
    for v in end.frame.pos.to_f64().to_array() {
        out.extend_from_slice(&quantise(v, LINK_ID_POS_STEP).to_le_bytes());
    }
    let q = end.frame.rotation.as_dquat().normalize();
    let mut r = [q.w, q.x, q.y, q.z].map(|c| quantise(c, LINK_ID_ROT_STEP));
    // q and -q are one rotation: make the first non-zero component positive.
    if r.iter().find(|&&c| c != 0).is_some_and(|&c| c < 0) {
        r = r.map(|c| -c);
    }
    for c in r {
        out.extend_from_slice(&c.to_le_bytes());
    }
    out
}

#[allow(clippy::cast_possible_truncation)] // world coordinates / step fit an i64
fn quantise(v: f64, step: f64) -> i64 {
    (v / step).round() as i64
}

fn half_turn() -> glam::Quat {
    glam::Quat::from_rotation_y(std::f32::consts::PI)
}
