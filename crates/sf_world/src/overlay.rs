//! The runtime overlay: gameplay changes to placements (removed, moved,
//! per-system state) and objects spawned at runtime. A cell load applies it
//! after reading the index, so streaming never reverts gameplay. Saves
//! serialize it.

use std::collections::{BTreeMap, BTreeSet};

use glam::{IVec3, Quat};
use serde::{Deserialize, Serialize};

use crate::codec::{CodecError, Header, OVERLAY_MAGIC, PlacementRepr, Strings};
use crate::index::IndexedPlacement;
use crate::{Location, Partition, Placement, PlacementId, WorldspaceId};

const SPAWN_NAMESPACE: &str = "sf.spawned";

/// Changes to the world since it was baked. Ordered maps throughout, so
/// iteration and serialization are deterministic.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PlacementOverlay {
    removed: BTreeSet<PlacementId>,
    /// Moved and spawned placements, at their current location.
    placed: BTreeMap<PlacementId, (WorldspaceId, Placement)>,
    /// (worldspace, 256 m grid cell) → ids in `placed`.
    buckets: BTreeMap<(WorldspaceId, [i32; 3]), BTreeSet<PlacementId>>,
    /// Per-placement state blobs, keyed by owning system.
    state: BTreeMap<PlacementId, BTreeMap<String, Vec<u8>>>,
    spawned: BTreeSet<PlacementId>,
    next_spawn: u64,
}

impl PlacementOverlay {
    pub fn new() -> Self {
        Self::default()
    }

    /// Removes a placement from the world (picked up, destroyed). A spawned
    /// object is forgotten entirely, state included.
    pub fn remove(&mut self, id: PlacementId) {
        self.unplace(id);
        if self.spawned.remove(&id) {
            self.state.remove(&id);
        } else {
            self.removed.insert(id);
        }
    }

    pub fn is_removed(&self, id: PlacementId) -> bool {
        self.removed.contains(&id)
    }

    /// Records `placement` (with its new transform) as living in
    /// `worldspace` from now on, wherever the index put it. Also undoes a
    /// removal.
    pub fn place(&mut self, worldspace: WorldspaceId, placement: Placement) {
        let id = placement.id;
        self.unplace(id);
        self.removed.remove(&id);
        self.buckets.entry((worldspace.clone(), placement.transform.pos.cell.to_array())).or_default().insert(id);
        self.placed.insert(id, (worldspace, placement));
    }

    /// Moves a placement to `to` with `rotation`, keeping its scale.
    pub fn move_to(&mut self, placement: &Placement, to: &Location, rotation: Quat) {
        let mut moved = placement.clone();
        moved.transform.pos = to.pos;
        moved.transform.rotation = rotation;
        self.place(to.worldspace.clone(), moved);
    }

    /// Adds a new object; its `id` is replaced by a fresh one, returned.
    pub fn spawn(&mut self, worldspace: WorldspaceId, mut placement: Placement) -> PlacementId {
        let id = PlacementId::from_key(SPAWN_NAMESPACE, &self.next_spawn.to_le_bytes());
        self.next_spawn += 1;
        placement.id = id;
        self.spawned.insert(id);
        self.place(worldspace, placement);
        id
    }

    pub fn is_spawned(&self, id: PlacementId) -> bool {
        self.spawned.contains(&id)
    }

    /// The current location of a moved or spawned placement.
    pub fn placed(&self, id: PlacementId) -> Option<(&WorldspaceId, &Placement)> {
        self.placed.get(&id).map(|(ws, p)| (ws, p))
    }

    /// Stores `owner`'s state blob for a placement (replacing the old one).
    pub fn set_state(&mut self, id: PlacementId, owner: &str, bytes: Vec<u8>) {
        self.state.entry(id).or_default().insert(owner.to_owned(), bytes);
    }

    pub fn state(&self, id: PlacementId, owner: &str) -> Option<&[u8]> {
        self.state.get(&id)?.get(owner).map(Vec::as_slice)
    }

    /// Every state blob of a placement, by owner.
    pub fn states(&self, id: PlacementId) -> impl Iterator<Item = (&str, &[u8])> {
        self.state.get(&id).into_iter().flatten().map(|(k, v)| (k.as_str(), v.as_slice()))
    }

    /// The effective content of one streaming cell: the index's placements
    /// minus removed and moved ones, then moved and spawned placements that
    /// now live in the cell (sorted by id). Overlay additions get radius 0,
    /// so they are near-ring content.
    pub fn apply(
        &self,
        worldspace: &WorldspaceId,
        partition: Partition,
        cell: IVec3,
        indexed: &[IndexedPlacement],
    ) -> Vec<IndexedPlacement> {
        let mut out: Vec<IndexedPlacement> = indexed
            .iter()
            .filter(|p| !self.removed.contains(&p.placement.id) && !self.placed.contains_key(&p.placement.id))
            .cloned()
            .collect();
        let mut added: Vec<&Placement> = Vec::new();
        let mut take = |bucket: &BTreeSet<PlacementId>| {
            for id in bucket {
                let (_, p) = &self.placed[id];
                if partition.cell_of(p.transform.pos) == cell {
                    added.push(p);
                }
            }
        };
        match partition {
            Partition::Single => {
                let range = (worldspace.clone(), [i32::MIN; 3])..=(worldspace.clone(), [i32::MAX; 3]);
                self.buckets.range(range).for_each(|(_, b)| take(b));
            }
            Partition::Grid { shift } if shift <= 0 => {
                let bits = shift.unsigned_abs();
                let grid = [cell.x >> bits, cell.y >> bits, cell.z >> bits];
                if let Some(b) = self.buckets.get(&(worldspace.clone(), grid)) {
                    take(b);
                }
            }
            Partition::Grid { shift } => {
                let span = 1i32 << shift;
                let base = cell * span;
                for x in base.x..base.x + span {
                    for z in base.z..base.z + span {
                        // A range over y for this column.
                        let range = (worldspace.clone(), [x, base.y, z])..=(worldspace.clone(), [x, base.y + span - 1, z]);
                        for ((_, k), b) in self.buckets.range(range) {
                            if k[0] == x && k[2] == z {
                                take(b);
                            }
                        }
                    }
                }
            }
        }
        added.sort_by_key(|p| p.id);
        out.extend(added.into_iter().map(|p| IndexedPlacement { placement: p.clone(), radius: 0.0 }));
        out
    }

    fn unplace(&mut self, id: PlacementId) {
        if let Some((ws, p)) = self.placed.remove(&id) {
            let key = (ws, p.transform.pos.cell.to_array());
            if let Some(bucket) = self.buckets.get_mut(&key) {
                bucket.remove(&id);
                if bucket.is_empty() {
                    self.buckets.remove(&key);
                }
            }
        }
    }

    /// Binary form for saves.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut strings = Strings::default();
        let placed = self
            .placed
            .values()
            .map(|(ws, p)| (strings.intern(ws.as_str()), PlacementRepr::encode(p, &mut strings)))
            .collect();
        let repr = OverlayRepr {
            header: Header::new(OVERLAY_MAGIC),
            removed: self.removed.iter().map(|id| id.0).collect(),
            spawned: self.spawned.iter().map(|id| id.0).collect(),
            next_spawn: self.next_spawn,
            state: self
                .state
                .iter()
                .map(|(id, owners)| (id.0, owners.iter().map(|(k, v)| (k.clone(), v.clone())).collect()))
                .collect(),
            placed,
            strings: strings.list,
        };
        postcard::to_stdvec(&repr).expect("overlays always serialize")
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, CodecError> {
        Header::expect(bytes, OVERLAY_MAGIC, "placement overlay")?;
        let repr: OverlayRepr = postcard::from_bytes(bytes)?;
        let mut out = Self {
            removed: repr.removed.into_iter().map(PlacementId).collect(),
            spawned: repr.spawned.into_iter().map(PlacementId).collect(),
            next_spawn: repr.next_spawn,
            state: repr
                .state
                .into_iter()
                .map(|(id, owners)| (PlacementId(id), owners.into_iter().collect()))
                .collect(),
            ..Self::default()
        };
        for (ws, p) in &repr.placed {
            let ws = repr.strings.get(*ws as usize).ok_or(CodecError::StringIndex(*ws))?;
            let p = p.decode(&repr.strings)?;
            let id = p.id;
            out.buckets.entry((WorldspaceId::new(ws), p.transform.pos.cell.to_array())).or_default().insert(id);
            out.placed.insert(id, (WorldspaceId::new(ws), p));
        }
        Ok(out)
    }
}

#[derive(Serialize, Deserialize)]
struct OverlayRepr {
    header: Header,
    removed: Vec<u128>,
    spawned: Vec<u128>,
    next_spawn: u64,
    state: Vec<(u128, OwnerStates)>,
    strings: Vec<String>,
    placed: Vec<(u32, PlacementRepr)>,
}

/// A placement's state blobs by owner.
type OwnerStates = Vec<(String, Vec<u8>)>;
