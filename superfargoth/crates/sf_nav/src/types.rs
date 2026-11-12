//! The data model (nav.md "Data model"): agent classes, the area table, polygon flags, tile keys,
//! off-mesh links and the backend-opaque refs.
//!
//! Everything here is serde except [`PolyRef`], [`TileRef`] and [`NavPoint`]: backend refs never
//! leave memory (nav.md "Co-op and saves" rule 7), saves and the wire carry `Location`s.

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sf_world::{Location, WorldspaceId};

use crate::NavError;

/// Most agent classes a game may define (one navmesh per worldspace and class).
pub const MAX_CLASSES: usize = 8;

/// Most area ids, Detour's `DT_MAX_AREAS` (a polygon's area is 6 bits there).
pub const MAX_AREAS: usize = 64;

/// An agent class: one navmesh per (worldspace, class). Ids are dense, `0..MAX_CLASSES`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct AgentClassId(pub u8);

impl AgentClassId {
    pub fn index(self) -> usize {
        usize::from(self.0)
    }
}

/// A set of agent classes (who may take an off-mesh link).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ClassMask(pub u8);

impl ClassMask {
    pub const NONE: Self = Self(0);
    pub const ALL: Self = Self(u8::MAX);

    pub fn single(class: AgentClassId) -> Self {
        Self::NONE.with(class)
    }

    #[must_use]
    pub fn with(self, class: AgentClassId) -> Self {
        Self(self.0 | 1 << class.0)
    }

    pub fn contains(self, class: AgentClassId) -> bool {
        self.0 & 1 << class.0 != 0
    }
}

/// What an agent class is (adapter data, record `sf.nav.agent`): the bake's walkable radius,
/// height, climb and slope.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AgentClass {
    pub id: AgentClassId,
    /// Meters; the bake erodes walls by this.
    pub radius: f32,
    /// Meters of clearance.
    pub height: f32,
    /// Highest step in meters (Recast `walkableClimb`).
    pub max_climb: f32,
    /// Steepest walkable slope in degrees (Recast `walkableSlopeAngle`).
    pub max_slope_deg: f32,
    /// [`QueryFilter::for_agent`](crate::QueryFilter::for_agent) excludes [`PolyFlags::SWIM`]
    /// when false.
    pub can_swim: bool,
}

/// A polygon's area type. The adapter's [`AreaTable`] names them; the framework names none.
/// `0` is [`AreaId::NULL`] (Recast's `RC_NULL_AREA`: not walkable, never a polygon).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct AreaId(pub u8);

impl AreaId {
    pub const NULL: Self = Self(0);

    pub fn index(self) -> usize {
        usize::from(self.0)
    }
}

bitflags::bitflags! {
    /// Per-polygon flags that query filters include or exclude (Detour `dtPoly::flags`, UE
    /// navigation filter flags). The bake sets them from the area's [`AreaDef::flags`] (not the
    /// backend's own defaults); runtime patches change them (doors). Bits `0x0100..=0x4000` are
    /// free for the adapter (kept by serde and [`PolyFlags::from_bits_retain`]).
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct PolyFlags: u16 {
        /// Ground a walker can use.
        const WALK = 1;
        /// Water: only for classes that swim.
        const SWIM = 1 << 1;
        /// A doorway's own polygons.
        const DOOR = 1 << 2;
        /// Polygons at the end of jump or drop links.
        const JUMP = 1 << 3;
        /// Hazards that careful agents exclude.
        const AVOID = 1 << 4;
        /// Shut off (a locked door): every default filter excludes it.
        const DISABLED = 1 << 15;
    }
}

impl Serialize for PolyFlags {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_u16(self.bits())
    }
}

impl<'de> Deserialize<'de> for PolyFlags {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from_bits_retain(u16::deserialize(d)?))
    }
}

/// A polygon's patchable state: what doors switch at runtime (Detour `setPolyArea` and
/// `setPolyFlags`, no rebuild).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PolyState {
    pub area: AreaId,
    pub flags: PolyFlags,
}

/// One area type in the adapter's table (`sf.nav.area`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AreaDef {
    pub id: AreaId,
    /// For debug draw and console output.
    pub name: String,
    /// Cost multiplier per meter, per agent class (index = [`AgentClassId`]). At least 1: Detour's
    /// A* heuristic assumes it (UE `NavArea::DefaultCost` has the same floor).
    pub cost: [f32; MAX_CLASSES],
    /// Flags the bake gives this area's polygons.
    pub flags: PolyFlags,
}

/// The adapter's area table: costs and flags per area id. Validated on construction and on
/// deserialization: ids unique and in `1..MAX_AREAS`, costs finite and at least 1.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "Vec<AreaDef>", into = "Vec<AreaDef>")]
pub struct AreaTable {
    areas: Vec<AreaDef>,
}

impl AreaTable {
    pub fn new(mut areas: Vec<AreaDef>) -> Result<Self, NavError> {
        areas.sort_by_key(|a| a.id);
        for (i, a) in areas.iter().enumerate() {
            if a.id == AreaId::NULL || a.id.index() >= MAX_AREAS {
                return Err(NavError::InvalidConfig(format!(
                    "area id {} outside 1..{MAX_AREAS}",
                    a.id.0
                )));
            }
            if i > 0 && areas[i - 1].id == a.id {
                return Err(NavError::InvalidConfig(format!(
                    "area id {} defined twice",
                    a.id.0
                )));
            }
            if let Some(c) = a.cost.iter().find(|c| !c.is_finite() || **c < 1.0) {
                return Err(NavError::InvalidConfig(format!(
                    "area {} cost {c} is not finite and >= 1",
                    a.name
                )));
            }
        }
        Ok(Self { areas })
    }

    pub fn get(&self, id: AreaId) -> Option<&AreaDef> {
        self.areas
            .binary_search_by_key(&id, |a| a.id)
            .ok()
            .map(|i| &self.areas[i])
    }

    /// The bake's flags for `id`; empty for an unknown area (no default filter passes it).
    pub fn flags(&self, id: AreaId) -> PolyFlags {
        self.get(id).map_or(PolyFlags::empty(), |a| a.flags)
    }

    pub fn iter(&self) -> impl Iterator<Item = &AreaDef> {
        self.areas.iter()
    }
}

impl TryFrom<Vec<AreaDef>> for AreaTable {
    type Error = NavError;

    fn try_from(areas: Vec<AreaDef>) -> Result<Self, NavError> {
        Self::new(areas)
    }
}

impl From<AreaTable> for Vec<AreaDef> {
    fn from(t: AreaTable) -> Self {
        t.areas
    }
}

/// One tile of one navmesh: the bake product's key (`nav.tiles`).
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TileKey {
    pub ws: WorldspaceId,
    pub class: AgentClassId,
    /// Nav region (a mesh in local coordinates, nav.md "Nav regions").
    pub region: u16,
    pub x: i32,
    pub z: i32,
}

/// A stable off-mesh link id: a hash of (kind, ends), so saves and the wire can name it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct NavLinkId(pub u64);

/// A doorway the bake gave its own polygons (nav.md "Doors"): the adapter's stable id (a hash of
/// the door's record and placement), so the runtime door overlay finds its polygons. Joins the
/// `sf_world::LinkId` ends with NV11.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct NavDoorId(pub u64);

/// How an agent crosses an off-mesh link. `Door(LinkId)` arrives with `sf_world::LinkId` (NV11).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TraverseKind {
    Walk,
    Jump,
    Drop,
    Climb,
    Ladder,
    Swim,
}

/// A link the mesh can't express (Detour off-mesh connection, UE `NavLinkProxy`): pathgrid edges,
/// jumps, drops, ladders. Both ends are in one worldspace.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OffMeshLink {
    pub id: NavLinkId,
    pub a: Location,
    pub b: Location,
    /// Meters around each end that snap onto the link.
    pub radius: f32,
    pub two_way: bool,
    pub kind: TraverseKind,
    pub area: AreaId,
    pub classes: ClassMask,
}

/// A backend polygon: opaque, 64-bit (Detour `dtPolyRef` with `DT_POLYREF64`). Valid while its
/// tile is resident; never saved or sent.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PolyRef(pub u64);

/// A backend tile handle (Detour `dtTileRef`): opaque, never saved or sent.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TileRef(pub u64);

/// A point on the navmesh: the location and the polygon it lies on.
#[derive(Clone, Debug, PartialEq)]
pub struct NavPoint {
    pub loc: Location,
    pub poly: PolyRef,
}
