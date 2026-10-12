//! Morrowind rules for the framework physics (`docs/specs/physics-adapter.md`,
//! "Items"); Bevy-free, shared by the bake (`mw_bake::body`) and the app.
//!
//! - Which placements are items: `PlacementClass::Dynamic` (MISC, WEAP,
//!   ARMO, CLOT, BOOK, ALCH, APPA, INGR, LOCK, PROB, REPA, resolved LEVI) and
//!   lights with the *Can Carry* flag. Carryable lights are held until
//!   grabbed (most hang or stand where nothing would hold them).
//! - Mass: [`item_mass`] from the record weight, or the shape volume for
//!   weightless items.

use std::collections::{HashMap, HashSet};

use sf_records::{RecordDb, RecordId};
use sf_world::{Placement, PlacementClass};
use tes3::esp::{
    Alchemy, Apparatus, Armor, Book, Clothing, Ingredient, Light, LightFlags, Lockpick, MiscItem, Probe, RepairItem, Weapon,
};

/// Morrowind weights read as roughly pounds (physics-adapter.md "Mass"): a
/// visual calibration constant, not a rule.
pub const KG_PER_WEIGHT: f32 = 0.5;
/// Density for weightless items (gold, clutter), kg/m³.
pub const WEIGHTLESS_DENSITY: f32 = 400.0;
/// The framework's simulated mass clamp (`PhysicsParams::mass_range`).
pub const MASS_RANGE: (f32, f32) = (0.2, 60.0);

/// Simulated mass of an item: weight × [`KG_PER_WEIGHT`], or volume ×
/// [`WEIGHTLESS_DENSITY`] for weight 0, clamped to [`MASS_RANGE`]. Burden
/// and encumbrance keep the record weight.
pub fn item_mass(weight: f32, volume: f32) -> f32 {
    let kg = if weight > 0.0 { weight * KG_PER_WEIGHT } else { volume * WEIGHTLESS_DENSITY };
    kg.clamp(MASS_RANGE.0, MASS_RANGE.1)
}

/// Base ids of lights with the *Can Carry* flag, sorted.
pub fn carryable_lights(db: &RecordDb) -> Vec<RecordId> {
    let mut ids: Vec<RecordId> =
        db.iter::<Light>().filter(|(_, l)| l.data.flags.contains(LightFlags::CAN_CARRY)).map(|(id, _)| id.clone()).collect();
    ids.sort();
    ids
}

/// Whether a placement gets an item body (`carryable`: [`carryable_lights`]).
pub fn is_item<S: std::hash::BuildHasher>(p: &Placement, carryable: &HashSet<RecordId, S>) -> bool {
    p.class == PlacementClass::Dynamic || (p.class == PlacementClass::Light && carryable.contains(&p.base.id))
}

/// Items held until grabbed whatever `items.check` says: carryable lights.
pub fn always_hold(p: &Placement) -> bool {
    p.class == PlacementClass::Light
}

/// Item records' weights and the carryable lights: what the runtime needs to
/// give an item placement its body.
#[derive(Clone, Debug, Default)]
pub struct ItemRecords {
    weights: HashMap<RecordId, f32>,
    carryable: HashSet<RecordId>,
}

impl ItemRecords {
    pub fn new(db: &RecordDb) -> Self {
        let mut weights = HashMap::new();
        macro_rules! collect {
            ($($ty:ty),* $(,)?) => {$(
                for (id, r) in db.iter::<$ty>() {
                    weights.insert(id.clone(), r.data.weight);
                }
            )*};
        }
        collect!(MiscItem, Weapon, Armor, Clothing, Book, Ingredient, Alchemy, Apparatus, Lockpick, Probe, RepairItem, Light);
        Self { weights, carryable: carryable_lights(db).into_iter().collect() }
    }

    /// Record weight of an item base id.
    pub fn weight(&self, id: &RecordId) -> Option<f32> {
        self.weights.get(id).copied()
    }

    pub fn is_item(&self, p: &Placement) -> bool {
        is_item(p, &self.carryable)
    }

    /// Simulated mass of an item placement whose shape has unit-scale
    /// `volume` (m³): the placement's scale applies to the volume, not the
    /// record weight.
    pub fn mass(&self, p: &Placement, volume: f32) -> f32 {
        let s = p.transform.scale.abs();
        item_mass(self.weight(&p.base.id).unwrap_or(0.0), volume * s.x * s.y * s.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mass_from_weight_or_volume_clamped() {
        assert!((item_mass(10.0, 0.0) - 5.0).abs() < 1e-6);
        assert!((item_mass(0.0, 0.001) - 0.4).abs() < 1e-6, "weightless: volume × 400");
        assert!((item_mass(0.1, 0.0) - MASS_RANGE.0).abs() < 1e-6);
        assert!((item_mass(500.0, 0.0) - MASS_RANGE.1).abs() < 1e-6);
    }
}
