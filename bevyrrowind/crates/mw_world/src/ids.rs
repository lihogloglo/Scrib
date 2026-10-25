//! Stable identities: worldspaces, regions, and placements.
//!
//! Spec: `docs/specs/world-adapter.md` ("Identity").

use sf_records::RecordId;
use sf_world::{PlacementId, RegionId, WorldspaceId};
use tes3::esp::Cell;

/// Record id of the one exterior worldspace. `@` never starts an interior
/// cell name in practice (the same rule `mw_records::keys` relies on), so it
/// can't collide with an interior's worldspace id.
pub const EXTERIOR: &str = "@exterior";

/// Namespace of every Morrowind reference's [`PlacementId`].
pub const PLACEMENT_NAMESPACE: &str = "mw.ref";

/// The exterior worldspace.
pub fn exterior() -> WorldspaceId {
    WorldspaceId::new(EXTERIOR)
}

/// An interior's worldspace: its cell name (case-insensitive, like the cell
/// record id).
pub fn interior(cell_name: &str) -> WorldspaceId {
    WorldspaceId::new(cell_name)
}

/// The worldspace a cell belongs to.
pub fn worldspace_of(cell: &Cell) -> WorldspaceId {
    if cell.is_interior() { interior(&cell.name) } else { exterior() }
}

/// A reference's id, from its load-order-stable key: the content file that
/// *first* placed it (lowercased file name) and its index in that file.
///
/// This is the original engine's and `OpenMW`'s reference identity (`FRMR` =
/// master index + 24-bit index, where the master index is file-local and is
/// mapped to a file *name*; `OpenMW`'s `RefNum` stores a content-file index
/// and saves map it through the save's list of file names). Plugins that
/// edit or move a master's reference keep its key, so the id survives edits,
/// moves, and load-order changes. References a plugin adds get the plugin's
/// name. Renaming a content file changes its references' ids, as it breaks
/// saves in the original engine.
pub fn placement_id(file: &str, refr_index: u32) -> PlacementId {
    let mut key = Vec::with_capacity(file.len() + 5);
    key.extend(file.bytes().map(|b| b.to_ascii_lowercase()));
    key.push(0);
    key.extend_from_slice(&refr_index.to_le_bytes());
    PlacementId::from_key(PLACEMENT_NAMESPACE, &key)
}

/// A source region (one cell record or exterior grid square) by its cell
/// record id (`mw_records::keys::cell`).
pub fn region_id(cell: &RecordId) -> RegionId {
    RegionId::from_key(cell.as_str())
}
