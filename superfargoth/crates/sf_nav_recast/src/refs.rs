//! Our [`PolyRef`] / [`TileRef`] layout: a Detour 64-bit ref (`polyref64`: salt 16 | tile 28 |
//! poly 20 bits) with the backend's mesh slot and its generation packed into tile-index bits a
//! mesh never uses. One `NavMesh` exists per (worldspace, class, region), so a bare Detour ref
//! would not say which mesh it belongs to; Detour's tile index stays below
//! [`MAX_TILES_PER_MESH`](crate::MAX_TILES_PER_MESH) = 2^13, leaving bits 33..48 free.

use navmesh_rs::detour;
use sf_nav::{PolyRef, TileRef};

/// Detour's poly bits (20) plus the tile bits a mesh uses (13).
const SLOT_SHIFT: u32 = 33;
pub(crate) const SLOT_BITS: u32 = 8;
const GEN_SHIFT: u32 = SLOT_SHIFT + SLOT_BITS;
pub(crate) const GEN_BITS: u32 = 7;
/// Bits 33..48: ours. Detour's salt (48..64) and the low tile and poly fields stay as they are.
const OURS: u64 = ((1 << (SLOT_BITS + GEN_BITS)) - 1) << SLOT_SHIFT;

/// A mesh slot's identity inside a ref.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct MeshTag {
    pub slot: usize,
    pub generation: u8,
}

fn pack(tag: MeshTag, r: detour::PolyRef) -> u64 {
    debug_assert!(r & OURS == 0, "Detour ref {r:#x} uses tile bits past 2^13");
    debug_assert!(tag.slot < 1 << SLOT_BITS && u32::from(tag.generation) < 1 << GEN_BITS);
    r | (tag.slot as u64) << SLOT_SHIFT | u64::from(tag.generation) << GEN_SHIFT
}

#[allow(
    clippy::cast_possible_truncation,
    reason = "both fields are masked to 8 and 7 bits"
)]
fn unpack(r: u64) -> (MeshTag, detour::PolyRef) {
    let tag = MeshTag {
        slot: ((r >> SLOT_SHIFT) & ((1 << SLOT_BITS) - 1)) as usize,
        generation: ((r >> GEN_SHIFT) & ((1 << GEN_BITS) - 1)) as u8,
    };
    (tag, r & !OURS)
}

pub(crate) fn poly(tag: MeshTag, r: detour::PolyRef) -> PolyRef {
    PolyRef(pack(tag, r))
}

pub(crate) fn tile(tag: MeshTag, r: detour::TileRef) -> TileRef {
    TileRef(pack(tag, r))
}

pub(crate) fn split(r: PolyRef) -> (MeshTag, detour::PolyRef) {
    unpack(r.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_keeps_detour_fields() {
        // Salt 0xbeef, tile 8191 (the highest a mesh uses), poly 0xfffff.
        let r: u64 = 0xbeef << 48 | 8191 << 20 | 0xf_ffff;
        let tag = MeshTag {
            slot: 255,
            generation: 127,
        };
        let p = poly(tag, r);
        assert_ne!(p.0, r);
        assert_eq!(split(p), (tag, r));
        let tag0 = MeshTag {
            slot: 0,
            generation: 0,
        };
        assert_eq!(poly(tag0, r).0, r);
    }
}
