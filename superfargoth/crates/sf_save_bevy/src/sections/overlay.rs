//! `sf/overlay`: the placement overlay (streaming.md "Runtime overlay") plus the **blob version
//! table** (save.md "Overlay blobs"): which schema version each system's per-placement state
//! blobs (`physics`, actor health, container contents) were written at.
//!
//! The overlay keeps its own binary form (`PlacementOverlay::to_bytes`, with its magic and
//! format version), embedded as bytes. Systems declare their blob versions in [`BlobVersions`];
//! a save whose blobs are newer or older than the build's is refused before anything is applied.
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
    /// Refuses blobs of any version other than the build's: newer ones it can't read, and older
    /// ones it can't migrate yet.
    ///
    /// Why older ones are refused rather than loaded (save review C7): the table holds one
    /// version per owner, but after a load the overlay would mix the save's unmigrated blobs with
    /// blobs the running systems rewrite at the current version. Neither version describes both,
    /// so the next save would mislabel one half for good (stamping the build's version re-labels
    /// the old blobs; writing the loaded table back re-labels the new ones). Until per-blob
    /// versions or a migrate-after-decode pass exist (overlay v2, needs `sf_world` blob
    /// iteration), a `declare()` bump must come with that migration, and older saves refuse here
    /// before anything changes. An owner missing from the save's table counts as version 1,
    /// as in [`BlobVersions::get`].
    pub fn check(&self, known: &BlobVersions) -> Result<(), String> {
        let owners = self.blob_versions.keys().chain(known.0.keys());
        for owner in owners {
            let v = self.blob_versions.get(owner).copied().unwrap_or(1);
            let current = known.get(owner);
            if v > current {
                return Err(format!("overlay blobs of {owner:?} are version {v}, newer than this build's {current}"));
            }
            if v < current {
                return Err(format!(
                    "overlay blobs of {owner:?} are version {v}, older than this build's {current}, and overlay blob migration does not exist yet"
                ));
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
