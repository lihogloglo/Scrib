//! World streaming core: worldspaces, door links, placements, the world
//! index, the runtime overlay, and the streaming scheduler.
//!
//! A *worldspace* is the streaming unit; interiors are worldspaces too.
//! Worldspaces are linked by *door links* (portals). Inside a worldspace,
//! content is a set of *placements* (placed objects), delivered by an
//! adapter's [`WorldSource`] at their final positions and indexed into a
//! regular grid of streaming cells (128 m by default), grouped in 1 km
//! pages. The [`Scheduler`] decides, as pure deterministic data, which cells
//! become resident at which [`Level`] around a set of anchors, in what
//! order, and how much work each frame may do.
//!
//! The crate is Bevy-free. `sf_world_bevy` spawns entities from the
//! scheduler's requests; the bake pipeline wraps the index builder.
//! Spec: `docs/specs/streaming.md`.

#[cfg(feature = "bake")]
pub mod bake;
mod codec;
mod index;
mod overlay;
mod placement;
mod reader;
mod records;
mod rings;
mod scheduler;
mod serde_util;
mod source;

pub use codec::CodecError;
pub use index::{
    Aabb, CellDirectory, CellSummary, IndexCell, IndexPage, IndexParams, IndexStats, IndexedPlacement, ModelInfo,
    PLACEMENT_BYTES, PageSummary, WorldIndex, build_directory, build_index, build_pages, bucket_by_page,
    placement_bounds,
};
pub use overlay::PlacementOverlay;
pub use placement::{Placement, PlacementClass, PlacementFlags, PlacementId};
pub use reader::{IndexReader, LevelContent, MemoryIndex, PageCache};
pub use records::{DoorLink, LinkEnd, Partition, RecordRef, RingOverrides, Worldspace};
pub use rings::{Level, RingConfig, RingRadii};
pub use scheduler::{
    Anchor, AnchorId, AnchorRole, Budgets, Cancelled, CellChange, Request, RequestId, RequestKind, Scheduler,
    SchedulerConfig, Stage, StageReport, StageStep, TickInput, TickReport, WorldspaceState,
};
pub use source::{Fingerprint, GroupPlacement, NativeWorldSource, PlacementGroup, RegionId, WorldSource};

use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sf_coords::WorldPos;
use sf_records::{RecordDb, RecordError, RecordId};

/// A worldspace: the id of its [`Worldspace`] record.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WorldspaceId(pub RecordId);

impl WorldspaceId {
    pub fn new(id: &str) -> Self {
        Self(RecordId::new(id))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl fmt::Debug for WorldspaceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Display for WorldspaceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl From<&str> for WorldspaceId {
    fn from(id: &str) -> Self {
        Self::new(id)
    }
}

impl Serialize for WorldspaceId {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for WorldspaceId {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self(RecordId::from(String::deserialize(d)?)))
    }
}

/// A position in a specific worldspace: what saves, links, and gameplay
/// store when the worldspace isn't implied by context.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub worldspace: WorldspaceId,
    pub pos: WorldPos,
}

impl Location {
    pub fn new(worldspace: WorldspaceId, pos: WorldPos) -> Self {
        Self { worldspace, pos }
    }
}

/// Registers the record kinds this crate owns: [`Worldspace`],
/// [`DoorLink`], and [`PlacementGroup`].
pub fn register_kinds(db: &mut RecordDb) -> Result<(), RecordError> {
    db.register::<Worldspace>(Worldspace::KIND)?;
    db.register::<DoorLink>(DoorLink::KIND)?;
    db.register::<PlacementGroup>(PlacementGroup::KIND)?;
    Ok(())
}

#[cfg(test)]
mod tests;
