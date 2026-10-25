//! Binary forms of node blobs (`.sfnode`) and the per-worldspace far
//! directory. Each starts with an 8-byte magic; the node header comes
//! first so summaries can be read without decoding geometry.

use glam::{IVec3, Vec3};
use serde::{Deserialize, Serialize};
use sf_coords::WorldPos;
use sf_model::{Cooked, CookedModel};
use sf_world::{Partition, WorldspaceId};

use crate::FormatError;
use crate::node::NodeKey;

const NODE_MAGIC: [u8; 8] = *b"SFNODE1\0";
const DIR_MAGIC: [u8; 8] = *b"SFFARD1\0";

/// Exact `WorldPos` in binary forms.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct PosRepr(pub [i32; 3], pub [f32; 3]);

impl From<WorldPos> for PosRepr {
    fn from(p: WorldPos) -> Self {
        Self(p.cell.to_array(), p.local.to_array())
    }
}

impl From<PosRepr> for WorldPos {
    fn from(r: PosRepr) -> Self {
        WorldPos::new(IVec3::from_array(r.0), Vec3::from_array(r.1))
    }
}

/// An impostor model a node references.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImpostorModel {
    /// VFS path (the `lod.impostor` subject).
    pub model: String,
    /// Capture sphere radius at scale 1 (meters).
    pub radius: f32,
}

/// What every node blob starts with (and the directory repeats).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeHeader {
    pub key: NodeKey,
    /// The node center: entity origin, frame of every position below.
    pub center: PosRepr,
    /// Bounds of everything drawn, relative to the center (empty: min > max).
    pub min: [f32; 3],
    pub max: [f32; 3],
    /// Geometric error of the mesh (meters, ≤ the level's budget).
    pub error: f32,
    /// Largest distance from the center to a child's origin (lead band).
    pub lead: f32,
    pub triangles: u32,
    pub instances: u32,
    pub lights: u32,
    pub impostor_models: Vec<ImpostorModel>,
}

impl NodeHeader {
    pub fn center(&self) -> WorldPos {
        self.center.into()
    }

    pub fn is_empty(&self) -> bool {
        self.triangles == 0 && self.instances == 0 && self.lights == 0
    }
}

/// One impostor instance (node-relative).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImpostorInstance {
    /// Capture-sphere center, relative to the node center.
    pub pos: [f32; 3],
    /// Rotation about +Y, radians.
    pub yaw: f32,
    /// Uniform scale (the sphere radius is `model.radius * scale`).
    pub scale: f32,
    /// Index into [`NodeHeader::impostor_models`].
    pub model: u16,
}

/// One light sprite (node-relative).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct LightPoint {
    pub pos: [f32; 3],
    /// Linear RGB.
    pub color: [f32; 3],
    pub intensity: f32,
    pub radius: f32,
}

/// A node's full content.
#[derive(Clone, Debug, PartialEq)]
pub struct NodeBlob {
    pub header: NodeHeader,
    /// The proxy mesh (one level), positions relative to the center.
    pub mesh: Option<CookedModel>,
    pub impostors: Vec<ImpostorInstance>,
    pub lights: Vec<LightPoint>,
}

#[derive(Serialize, Deserialize)]
struct Body {
    mesh: Option<Vec<u8>>,
    impostors: Vec<ImpostorInstance>,
    lights: Vec<LightPoint>,
}

impl NodeBlob {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = NODE_MAGIC.to_vec();
        out.extend(postcard::to_stdvec(&self.header).expect("node headers serialize"));
        let body = Body {
            mesh: self.mesh.as_ref().map(|m| Cooked::Static(m.clone()).to_bytes()),
            impostors: self.impostors.clone(),
            lights: self.lights.clone(),
        };
        out.extend(postcard::to_stdvec(&body).expect("node bodies serialize"));
        out
    }

    /// The header alone (directory building, residency).
    pub fn header(bytes: &[u8]) -> Result<NodeHeader, FormatError> {
        let rest = bytes.strip_prefix(&NODE_MAGIC).ok_or(FormatError::BadMagic("node blob"))?;
        Ok(postcard::take_from_bytes::<NodeHeader>(rest)?.0)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, FormatError> {
        let rest = bytes.strip_prefix(&NODE_MAGIC).ok_or(FormatError::BadMagic("node blob"))?;
        let (header, rest) = postcard::take_from_bytes::<NodeHeader>(rest)?;
        let body: Body = postcard::from_bytes(rest)?;
        let mesh = match body.mesh {
            Some(bytes) => match Cooked::from_bytes(&bytes).map_err(|e| FormatError::Mesh(e.to_string()))? {
                Cooked::Static(m) => Some(m),
                Cooked::Skipped { .. } | Cooked::Skinned(_) => None,
            },
            None => None,
        };
        Ok(Self { header, mesh, impostors: body.impostors, lights: body.lights })
    }
}

/// A node as the directory lists it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeSummary {
    pub header: NodeHeader,
    /// Blob size (bytes), for residency budgets.
    pub bytes: u64,
}

/// Every non-empty node of a worldspace (`lod.dir`), sorted by key
/// (level, then index).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FarDirectory {
    pub worldspace: WorldspaceId,
    pub partition: Partition,
    /// Handoff distances the nodes were baked for (`FarLevels::handoff`).
    pub handoff: [f32; 3],
    pub band: f32,
    /// How the impostor atlases were captured (the shader must match).
    pub impostor: crate::impostor::ImpostorSettings,
    pub nodes: Vec<NodeSummary>,
    /// Every impostor model any node uses (sorted, unique).
    pub impostor_models: Vec<ImpostorModel>,
}

impl FarDirectory {
    pub fn find(&self, key: &NodeKey) -> Option<&NodeSummary> {
        self.nodes.binary_search_by(|n| n.header.key.cmp(key)).ok().map(|i| &self.nodes[i])
    }

    pub fn level(&self, level: u8) -> impl Iterator<Item = &NodeSummary> {
        self.nodes.iter().filter(move |n| n.header.key.level == level)
    }

    /// Total blob bytes of the nodes of `level`.
    pub fn level_bytes(&self, level: u8) -> u64 {
        self.level(level).map(|n| n.bytes).sum()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = DIR_MAGIC.to_vec();
        out.extend(postcard::to_stdvec(self).expect("far directories serialize"));
        out
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, FormatError> {
        let rest = bytes.strip_prefix(&DIR_MAGIC).ok_or(FormatError::BadMagic("far directory"))?;
        Ok(postcard::from_bytes(rest)?)
    }
}
