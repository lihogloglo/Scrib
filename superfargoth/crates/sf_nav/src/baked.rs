//! The navmesh bake's products as the runtime reads them (nav.md "Bake"): one [`BakedTile`] per
//! `nav.tiles` subject, one [`NavLinks`] per worldspace (`nav.links`) and one [`CoarseGraph`] per
//! worldspace and agent class (`nav.coarse`). Serde (postcard in the store); the steps that make
//! them live in `sf_nav_bake`, so the runtime never links the bake.

use glam::Vec3;
use serde::{Deserialize, Serialize};
use sf_world::Location;

use crate::{AgentClassId, NavDoorId, OffMeshLink, TileSide};

/// One doorway's polygons in a tile (nav.md "Doors"): what the door overlay patches.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoorPolys {
    pub door: NavDoorId,
    /// In-tile polygon indices ([`BlobTile::polys`](crate::BlobTile::polys) order), ascending.
    pub polys: Vec<u32>,
}

/// A `nav.tiles` product: the backend's tile blob and its doorways. A tile with no polygons is
/// stored as zero bytes, not as a `BakedTile` ([`BakedTile::decode`] gives `None`).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct BakedTile {
    pub blob: Vec<u8>,
    /// Sorted by door id; a doorway spanning two tiles is listed in both.
    pub doors: Vec<DoorPolys>,
}

impl BakedTile {
    /// The product bytes: empty for an empty tile.
    pub fn encode(&self) -> Result<Vec<u8>, postcard::Error> {
        if self.blob.is_empty() {
            return Ok(Vec::new());
        }
        postcard::to_stdvec(self)
    }

    /// `None` for an empty tile.
    pub fn decode(bytes: &[u8]) -> Result<Option<Self>, postcard::Error> {
        if bytes.is_empty() {
            return Ok(None);
        }
        postcard::from_bytes(bytes).map(Some)
    }
}

/// A door's approach point in one worldspace (nav.md "Doors"): the walkable point in front of it
/// that the worldspace graph's edge ends at.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DoorApproach {
    pub door: NavDoorId,
    pub at: Location,
}

/// A `nav.links` product: one worldspace's off-mesh links and door approach points, sorted by id.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct NavLinks {
    pub links: Vec<OffMeshLink>,
    pub doors: Vec<DoorApproach>,
}

/// What a [`CoarseNode`] stands for.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CoarseNodeKind {
    /// A run of tile-border edges where the mesh continues into the next tile (HPA* entrance).
    Entrance(TileSide),
    /// A door's approach point.
    Door(NavDoorId),
}

/// A node of the coarse graph, in its tile.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CoarseNode {
    pub region: u16,
    pub tile: (i32, i32),
    /// Connected polygon set of the tile the node lies on (nodes of one tile connect only within
    /// a component).
    pub component: u32,
    pub kind: CoarseNodeKind,
    /// Region-local meters.
    pub at: Vec3,
}

/// A walking-cost edge (both ways).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct CoarseEdge {
    pub a: u32,
    pub b: u32,
    pub cost: f32,
}

/// A `nav.coarse` product: HPA*'s abstract graph for one worldspace and class (nav.md "Coarse
/// graph"). Until NV12 the costs are straight-line meters between nodes on a connected polygon set;
/// NV12 replaces them with Detour path lengths and area costs.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CoarseGraph {
    pub class: AgentClassId,
    /// Entrances in tile order (region, x, z), then component, side and position along it; then
    /// door nodes by door id.
    pub nodes: Vec<CoarseNode>,
    /// Sorted, `a < b`.
    pub edges: Vec<CoarseEdge>,
    /// Doors whose approach point lies on no polygon of this class's mesh.
    pub unplaced: Vec<NavDoorId>,
}
