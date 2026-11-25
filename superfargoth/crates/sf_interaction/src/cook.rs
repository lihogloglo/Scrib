//! `interact.cook` (interaction.md "Definitions"): the one-time, mod-aware bake (seconds) that
//! completes defs from their models. Stub: returns the defs unchanged.
//!
//! The real cook will measure, where the adapter or mod gave none:
//! - slot frames and `band` heights from the model's collision top surfaces (seat, bed, counter
//!   tops: the MG4 furniture census logic, motion-generation.md "Furniture and race census",
//!   promoted to a cook step);
//! - door hinges (point, axis) and ladder edge chains from the model's bounds;
//!
//! and report unknown tags, kinds or clip slots as load errors naming the mod (mods.md).

use crate::def::InteractableDef;

/// Completes `defs` from their models. Identity until the measuring cook lands.
#[must_use]
pub fn cook(defs: Vec<InteractableDef>) -> Vec<InteractableDef> {
    defs
}
