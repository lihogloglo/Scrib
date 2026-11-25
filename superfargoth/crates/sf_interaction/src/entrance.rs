//! Entrance and exit choice (interaction.md "Using a slot", steps 1 and 4). The checks behind
//! `valid` are the shell's (nav-validated approach point for NPCs, the `EndPoseFree` capsule test);
//! the core only orders. Positions are in the object's frame: [`to_local`] brings a world point there.

use glam::Vec3;

use crate::def::{Entrance, Frame, Side, SlotDef};

/// Distances this close are a tie, broken by side order, m.
// tunables: const (maths: float tie tolerance for equal-distance entrances, not a feel value)
const TIE_EPS: f32 = 1.0e-3;

/// A world point in the frame of an object at `object` (world).
#[must_use]
pub fn to_local(object: &Frame, world: Vec3) -> Vec3 {
    object.rotation.inverse() * (world - object.translation)
}

/// The nearest entry among `candidates` to `from`; ties (within a millimetre) by [`Side`] order,
/// then def order.
fn nearest<'a>(candidates: impl Iterator<Item = &'a Entrance>, from: Vec3) -> Option<Entrance> {
    let scored: Vec<(f32, &Entrance)> = candidates.map(|e| (e.approach.distance(from), e)).collect();
    let d0 = scored.iter().map(|(d, _)| *d).min_by(f32::total_cmp)?;
    scored.into_iter().filter(|(d, _)| *d <= d0 + TIE_EPS).map(|(_, e)| *e).min_by_key(|e| e.side)
}

/// Step 1, approach: the entrance whose approach point is nearest `user_pos` (object frame) among
/// those `valid` accepts; ties by side order Front, Left, Right, Back, Bottom, Top. `None` = no way in.
pub fn pick_entrance(slot: &SlotDef, user_pos: Vec3, valid: impl Fn(&Entrance) -> bool) -> Option<Entrance> {
    nearest(slot.entrances.iter().filter(|e| valid(e)), user_pos)
}

/// Step 4, exit: the entry side's entrance when `valid` accepts it, else the valid entrance nearest
/// the slot frame (ties by side order), else `None` (the shell places the user on the nearest
/// navmesh point).
pub fn pick_exit(slot: &SlotDef, entry_side: Side, valid: impl Fn(&Entrance) -> bool) -> Option<Entrance> {
    let from = slot.local.translation;
    nearest(slot.entrances.iter().filter(|e| e.side == entry_side && valid(e)), from)
        .or_else(|| nearest(slot.entrances.iter().filter(|e| valid(e)), from))
}
