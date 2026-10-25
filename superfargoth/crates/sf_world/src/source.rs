//! Where placements come from: the adapter-facing [`WorldSource`] trait,
//! and the native implementation over [`PlacementGroup`] records, so mods
//! can place objects without any adapter.

use std::collections::BTreeMap;
use std::fmt;

use serde::{Deserialize, Serialize};
use sf_coords::WorldTransform;
use sf_records::{RecordDb, RecordId};
use sf_vfs::VfsPath;

use crate::serde_util::opt_path;
use crate::{Placement, PlacementClass, PlacementFlags, PlacementId, RecordRef, WorldspaceId};

/// One of an adapter's own source regions (for example, its cells), as a
/// stable hash of the adapter's region key.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct RegionId(pub u64);

impl RegionId {
    pub fn from_key(key: &str) -> Self {
        let hash = blake3::hash(key.as_bytes());
        Self(u64::from_le_bytes(hash.as_bytes()[..8].try_into().expect("8-byte prefix")))
    }
}

/// A content fingerprint (blake3). Unchanged fingerprint ⇒ unchanged
/// content ⇒ cached index pages are reused.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Fingerprint(pub [u8; 32]);

impl Fingerprint {
    pub fn of(bytes: &[u8]) -> Self {
        Self(*blake3::hash(bytes).as_bytes())
    }

    /// Fingerprint of several parts, each length-prefixed so part
    /// boundaries count.
    pub fn of_parts(parts: &[&[u8]]) -> Self {
        let mut h = blake3::Hasher::new();
        for part in parts {
            h.update(&(part.len() as u64).to_le_bytes());
            h.update(part);
        }
        Self(*h.finalize().as_bytes())
    }
}

impl fmt::Debug for Fingerprint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for b in &self.0[..8] {
            write!(f, "{b:02x}")?;
        }
        f.write_str("…")
    }
}

/// Implemented by adapters (Bevy-free), and by [`NativeWorldSource`].
///
/// Terrain is not part of this trait: the adapter hands its terrain source
/// to `sf_terrain` directly (that crate depends on this one for
/// `WorldspaceId`, so a hook here would be a dependency cycle).
pub trait WorldSource: Send + Sync {
    /// The source regions of one worldspace with a fingerprint each, in a
    /// stable order.
    fn regions(&self, ws: &WorldspaceId, out: &mut dyn FnMut(RegionId, Fingerprint));
    /// Every placement of one region, at its final position, in a stable
    /// order. A placement may lie anywhere in the worldspace (moved
    /// objects): the index assigns cells by position, not by region.
    fn placements(&self, ws: &WorldspaceId, region: RegionId, out: &mut dyn FnMut(Placement));
}

/// A native record kind: a named group of placements in one worldspace.
/// Mods add groups, or patch other mods' groups like any record.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlacementGroup {
    pub worldspace: WorldspaceId,
    #[serde(default)]
    pub placements: Vec<GroupPlacement>,
}

/// One placement in a [`PlacementGroup`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupPlacement {
    /// Unique within the group; the placement id hashes group id and key,
    /// so ids survive reordering and edits.
    pub key: String,
    pub base: RecordRef,
    #[serde(default, with = "opt_path")]
    pub model: Option<VfsPath>,
    pub transform: WorldTransform,
    #[serde(default = "static_class")]
    pub class: PlacementClass,
    #[serde(default)]
    pub flags: PlacementFlags,
}

fn static_class() -> PlacementClass {
    PlacementClass::Static
}

impl PlacementGroup {
    pub const KIND: &'static str = "sf.placement_group";

    /// The id of placement `key` in group `group`.
    pub fn placement_id(group: &RecordId, key: &str) -> PlacementId {
        let mut bytes = Vec::with_capacity(group.as_str().len() + 1 + key.len());
        bytes.extend_from_slice(group.as_str().as_bytes());
        bytes.push(0);
        bytes.extend_from_slice(key.as_bytes());
        PlacementId::from_key(Self::KIND, &bytes)
    }

    /// Content fingerprint: blake3 of the group's binary serde form.
    pub fn fingerprint(&self) -> Fingerprint {
        Fingerprint::of(&postcard::to_stdvec(self).expect("placement groups always serialize"))
    }

    /// The group's placements, with their ids.
    pub fn resolve<'a>(&'a self, group: &'a RecordId) -> impl Iterator<Item = Placement> + 'a {
        self.placements.iter().map(move |p| Placement {
            id: Self::placement_id(group, &p.key),
            base: p.base.clone(),
            model: p.model.clone(),
            transform: p.transform,
            class: p.class,
            flags: p.flags,
        })
    }
}

/// [`WorldSource`] over the live [`PlacementGroup`] records: one region per
/// group. A snapshot, so it can move to bake threads.
#[derive(Clone, Debug, Default)]
pub struct NativeWorldSource {
    groups: BTreeMap<WorldspaceId, Vec<NativeRegion>>,
}

#[derive(Clone, Debug)]
struct NativeRegion {
    region: RegionId,
    id: RecordId,
    group: PlacementGroup,
    fingerprint: Fingerprint,
}

impl NativeWorldSource {
    /// Snapshots every live group (in record order, so regions are stable).
    pub fn from_db(db: &RecordDb) -> Self {
        let mut groups: BTreeMap<WorldspaceId, Vec<NativeRegion>> = BTreeMap::new();
        for (id, group) in db.iter::<PlacementGroup>() {
            groups.entry(group.worldspace.clone()).or_default().push(NativeRegion {
                region: RegionId::from_key(id.as_str()),
                id: id.clone(),
                group: group.clone(),
                fingerprint: group.fingerprint(),
            });
        }
        Self { groups }
    }

    /// Worldspaces that have at least one group.
    pub fn worldspaces(&self) -> impl Iterator<Item = &WorldspaceId> {
        self.groups.keys()
    }
}

impl WorldSource for NativeWorldSource {
    fn regions(&self, ws: &WorldspaceId, out: &mut dyn FnMut(RegionId, Fingerprint)) {
        for r in self.groups.get(ws).into_iter().flatten() {
            out(r.region, r.fingerprint);
        }
    }

    fn placements(&self, ws: &WorldspaceId, region: RegionId, out: &mut dyn FnMut(Placement)) {
        for r in self.groups.get(ws).into_iter().flatten().filter(|r| r.region == region) {
            r.group.resolve(&r.id).for_each(&mut *out);
        }
    }
}
