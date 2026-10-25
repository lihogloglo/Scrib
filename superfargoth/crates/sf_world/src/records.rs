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

    /// [`DoorLink::cross`] for a direction (velocity, view direction).
    pub fn cross_direction(from: &LinkEnd, to: &LinkEnd, dir: Vec3) -> Vec3 {
        to.frame.rotation * (half_turn() * (from.frame.rotation.inverse() * dir))
    }
}

fn half_turn() -> glam::Quat {
    glam::Quat::from_rotation_y(std::f32::consts::PI)
}
