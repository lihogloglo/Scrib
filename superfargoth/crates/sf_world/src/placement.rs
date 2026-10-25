//! Placements: what streaming spawns. They aren't records, because each
//! adapter expresses them in its own records; adapters deliver them through
//! a `WorldSource`.

use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sf_coords::WorldTransform;
use sf_vfs::VfsPath;

use crate::RecordRef;

/// A placement's identity: a 128-bit hash of an adapter key that is stable
/// across load orders (for example, the defining content file plus the
/// object's index in it). Saves and the overlay key on it.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PlacementId(pub u128);

impl PlacementId {
    /// Hashes `key` in `namespace` (an adapter or source name, so two
    /// adapters can't collide by accident). blake3, truncated to 128 bits.
    pub fn from_key(namespace: &str, key: &[u8]) -> Self {
        let mut h = blake3::Hasher::new();
        // Length-prefix the namespace so ("ab", "c") and ("a", "bc") differ.
        h.update(&(namespace.len() as u64).to_le_bytes());
        h.update(namespace.as_bytes());
        h.update(key);
        let bytes: [u8; 16] = h.finalize().as_bytes()[..16].try_into().expect("16-byte prefix");
        Self(u128::from_le_bytes(bytes))
    }
}

impl fmt::Debug for PlacementId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:032x}", self.0)
    }
}

impl fmt::Display for PlacementId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

/// Hex string in text formats (JSON numbers can't hold 128 bits), a plain
/// `u128` in binary ones.
impl Serialize for PlacementId {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        if s.is_human_readable() { s.collect_str(self) } else { s.serialize_u128(self.0) }
    }
}

impl<'de> Deserialize<'de> for PlacementId {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        if d.is_human_readable() {
            let s = String::deserialize(d)?;
            u128::from_str_radix(&s, 16).map(Self).map_err(serde::de::Error::custom)
        } else {
            u128::deserialize(d).map(Self)
        }
    }
}

/// What a placement is, which decides who spawns it (`PlacementSpawner`
/// hooks per class on the Bevy side) and its order within a cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum PlacementClass {
    /// Never moves: framework-spawned, batched under its stream cell.
    Static,
    /// Can move (physics props, items).
    Dynamic,
    Actor,
    Light,
    Door,
    /// Invisible gameplay point (spawn points, markers).
    Marker,
    /// Content-pack defined.
    Custom(u16),
}

bitflags::bitflags! {
    /// Per-placement switches.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct PlacementFlags: u32 {
        /// Snap to terrain after terrain edits.
        const GROUNDED = 1;
        /// Editor-only helper; never spawned in play.
        const EDITOR_ONLY = 1 << 1;
        /// Keep full detail at every distance (no LOD chain).
        const NO_LOD = 1 << 2;
        /// Starts disabled (scripts enable it).
        const DISABLED = 1 << 3;
        /// Never part of the far ring, whatever its size.
        const NEAR_ONLY = 1 << 4;
        /// Never culled by size before its ring or far-field handoff ends it
        /// (industry-audit.md J1): quest items, landmarks, anything an
        /// adapter wants seen from as far as it is loaded.
        const NO_SIZE_CULL = 1 << 5;
    }
}

impl Serialize for PlacementFlags {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        bitflags::serde::serialize(self, s)
    }
}

impl<'de> Deserialize<'de> for PlacementFlags {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bitflags::serde::deserialize(d)
    }
}

/// One placed object, at its final position in a worldspace.
#[derive(Clone, Debug, PartialEq)]
pub struct Placement {
    pub id: PlacementId,
    /// The base object record.
    pub base: RecordRef,
    pub model: Option<VfsPath>,
    pub transform: WorldTransform,
    pub class: PlacementClass,
    pub flags: PlacementFlags,
}

impl Placement {
    /// Whether far-field proxies (far-field.md) may stand in for this
    /// placement: a visual static (statics, doors, lights) with a model and
    /// no flag keeping it near or out of play. The far bake represents
    /// exactly these, and streaming ends their own LOD chain at the
    /// far-field handoff.
    pub fn far_eligible(&self) -> bool {
        let excluded =
            PlacementFlags::NO_LOD | PlacementFlags::NEAR_ONLY | PlacementFlags::EDITOR_ONLY | PlacementFlags::DISABLED;
        self.model.is_some()
            && !self.flags.intersects(excluded)
            && matches!(self.class, PlacementClass::Static | PlacementClass::Door | PlacementClass::Light)
    }
}
