//! `sf/overlay`: the placement overlay (streaming.md "Runtime overlay") plus the **blob version
//! table** (save.md "Overlay blobs"): which schema version each system's per-placement state
//! blobs (`physics`, actor health, container contents) were written at.
//!
//! The overlay keeps its own binary form (`PlacementOverlay::to_bytes`, with its magic and
//! format version), embedded as bytes. Systems declare their blob versions in [`BlobVersions`];
//! a save whose blobs are newer than the build's is refused before anything is applied.
//! Migrating older blobs needs `sf_world` to expose blob iteration (overlay v2, see save.md "As
//! built").

use std::collections::BTreeMap;
use std::fmt;

use bevy_ecs::prelude::*;
use serde::de::{self, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sf_world::PlacementOverlay;

/// Schema version of each system's overlay state blobs, by the owner name it passes to
/// `PlacementOverlay::set_state`. A system that stores blobs registers here; unknown owners
/// count as version 1.
#[derive(Resource, Clone, Debug, Default, PartialEq, Eq)]
pub struct BlobVersions(pub BTreeMap<String, u32>);

impl BlobVersions {
    pub fn declare(&mut self, owner: &str, version: u32) {
        self.0.insert(owner.to_owned(), version);
    }

    pub fn get(&self, owner: &str) -> u32 {
        self.0.get(owner).copied().unwrap_or(1)
    }
}

/// The stored form of `sf/overlay`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OverlaySave {
    #[serde(with = "overlay_bytes")]
    pub overlay: PlacementOverlay,
    /// The blob versions at saving.
    pub blob_versions: BTreeMap<String, u32>,
}

impl OverlaySave {
    /// Refuses blobs newer than this build knows.
    pub fn check(&self, known: &BlobVersions) -> Result<(), String> {
        for (owner, &v) in &self.blob_versions {
            let current = known.get(owner);
            if v > current {
                return Err(format!("overlay blobs of {owner:?} are version {v}, newer than this build's {current}"));
            }
        }
        Ok(())
    }
}

/// The overlay as one byte string (its own codec, versioned by its header).
mod overlay_bytes {
    use super::{Deserializer, PlacementOverlay, SeqAccess, Serializer, Visitor, de, fmt};

    pub fn serialize<S: Serializer>(o: &PlacementOverlay, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_bytes(&o.to_bytes())
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<PlacementOverlay, D::Error> {
        let bytes = d.deserialize_byte_buf(BytesVisitor)?;
        PlacementOverlay::from_bytes(&bytes).map_err(de::Error::custom)
    }

    struct BytesVisitor;

    impl<'de> Visitor<'de> for BytesVisitor {
        type Value = Vec<u8>;

        fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("placement overlay bytes")
        }

        fn visit_bytes<E: de::Error>(self, v: &[u8]) -> Result<Vec<u8>, E> {
            Ok(v.to_vec())
        }

        fn visit_byte_buf<E: de::Error>(self, v: Vec<u8>) -> Result<Vec<u8>, E> {
            Ok(v)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Vec<u8>, A::Error> {
            let mut out = Vec::with_capacity(seq.size_hint().unwrap_or(0).min(1 << 20));
            while let Some(b) = seq.next_element()? {
                out.push(b);
            }
            Ok(out)
        }
    }
}
