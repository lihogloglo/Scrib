//! Bevy-free big-world rendering core (spec `superfargoth/docs/specs/big-world-rendering.md`, slice BW-1): millions
//! of trees and dense scatter that are never entities.
//!
//! The world is cut into square tiles on the ground plane. A deterministic instance source fills a tile on demand,
//! the tile is packed into 16-byte instances for one GPU pool block, and a ring planner keeps the tiles around the
//! viewer resident. Which representation an instance gets (mesh, impostor, tile aggregate) comes from its size on
//! screen. The renderer (`sf_bigworld_bevy`) runs these same functions as the CPU twin of its GPU producer.
//!
//! Frame: metres, x and z on the ground plane, y up. This crate names no game format.

mod bands;
mod hash;
mod instance;
mod ring;
mod source;
mod tile;
pub mod tunables;

pub use bands::{BandSettings, Bands, SpeciesDims, TierFades, ViewScale};
pub use instance::{Instance, PackedInstance, PackedTile, TileHeader};
pub use ring::{Ring, RingDelta, RingSettings, ring_radii};
pub use source::{InstanceSource, SyntheticForest};
pub use tile::TileKey;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_bands;
#[cfg(test)]
mod tests_ring;
