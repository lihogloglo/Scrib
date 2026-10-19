//! Morrowind rules for the framework physics (`docs/specs/physics-adapter.md`,
//! "Items"); Bevy-free, shared by the bake (`mw_bake::body`) and the app.
//!
//! - Which placements are items: `PlacementClass::Dynamic` (MISC, WEAP,
//!   ARMO, CLOT, BOOK, ALCH, APPA, INGR, LOCK, PROB, REPA, resolved LEVI) and
//!   lights with the *Can Carry* flag. Carryable lights are held until
//!   grabbed (most hang or stand where nothing would hold them).
//! - Mass: [`item_mass`] from the record weight, or the shape volume for
//!   weightless items.

mod cloth;

pub use cloth::{CHEST_SLOT, CLOTH_ATTACH, ClothKind, ClothMode, MwClothRules, SKIRT_SLOT, cloth_spec};

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

/// Strength of a holder with no stat yet (the dev camera): an average character.
pub const DEFAULT_STRENGTH: f32 = 50.0;

/// Holder limits from the holder's current Strength attribute (Fortify and
/// Drain apply), physics-adapter.md "Holder limits from Strength": carry
/// force `40 + 4S` N, carry mass `5 + 0.2S` kg, drag mass `40 + 1.6S` kg,
/// throw impulse `2 + 0.08S` N·s. Constants are tuning (PH-4). Strength
/// below 0 counts as 0.
pub fn holder_limits(strength: f32) -> sf_physics::HolderLimits {
    let s = strength.max(0.0);
    sf_physics::HolderLimits {
        carry_force: 40.0 + 4.0 * s,
        carry_mass: 5.0 + 0.2 * s,
        drag_mass: 40.0 + 1.6 * s,
        throw_impulse: 2.0 + 0.08 * s,
    }
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

/// A humanoid's ragdoll mass at scale 1, kg (physics-adapter.md "Ragdolls"):
/// Morrowind records carry no body mass; race height and weight scale it at
/// runtime (the cube of the uniform scale).
pub const HUMANOID_MASS: f32 = 75.0;

/// Creature models that keep their death clip instead of a ragdoll:
/// incorporeal or particle bodies (ghosts, wraiths, wisps).
const DEATH_CLIP_ONLY: [&str; 4] = ["ghost", "spect", "wraith", "wisp"];

/// Creature ragdoll masses at scale 1, kg (physics-adapter.md "Ragdolls"), by
/// a fragment of the skeleton model's name (lowercase, letters and digits
/// only; first match wins, so specific names come first). Real-animal
/// analogs sized to the models: the P7 capsule-volume masses ran from 1 kg
/// (horker 4.6 kg) to 12.9 t (netch bull) because limb radii come from bone
/// lengths, not skin extents. Unlisted creatures keep the volume mass.
pub const CREATURE_MASSES: [(&str, f32); 52] = [
    ("netchbull", 600.0), ("netchbetty", 450.0), ("netch", 500.0), ("guar", 400.0), ("cliffracer", 30.0), ("racer", 30.0),
    ("kwamaqueen", 1500.0), ("kwamawar", 250.0), ("kwamawork", 120.0), ("kwamafor", 15.0), ("kwama", 60.0), ("scrib", 10.0),
    ("scamp", 45.0), ("alit", 250.0), ("kagouti", 300.0), ("nix", 60.0), ("mudcrab", 25.0), ("crab", 25.0),
    ("slaughterfish", 15.0), ("babelfish", 1.0), ("dreugh", 200.0), ("durzog", 90.0), ("shalk", 8.0), ("rat", 6.0),
    ("atronachflame", 150.0), ("atronachfrost", 300.0), ("atronachstorm", 600.0), ("atronach", 300.0), ("daedroth", 250.0),
    ("clannfear", 180.0), ("hunger", 120.0), ("ogrim", 600.0), ("twilight", 70.0), ("bonelord", 60.0), ("bonewalker", 50.0),
    ("skeleton", 30.0), ("lich", 60.0), ("corprus", 90.0), ("spider", 80.0), ("sphere", 250.0), ("steam", 900.0),
    ("fabricant", 150.0), ("werewolf", 120.0), ("wolf", 45.0), ("bear", 350.0), ("boar", 120.0), ("horker", 400.0),
    ("spriggan", 70.0), ("riekling", 30.0), ("grahl", 300.0), ("karstaag", 900.0), ("giant", 600.0),
];

/// A creature skeleton's ragdoll mass from [`CREATURE_MASSES`].
pub fn creature_mass(skeleton: &str) -> Option<f32> {
    let key: String = skeleton.chars().filter(char::is_ascii_alphanumeric).map(|c| c.to_ascii_lowercase()).collect();
    let name = key.rsplit("nif").nth(1).unwrap_or(&key);
    CREATURE_MASSES.iter().find(|(k, _)| name.contains(k)).map(|&(_, m)| m)
}

/// `ragdoll.cook` rules for a skeleton model (`MwRagdollRules` in `mw_bake`):
/// humanoids weigh [`HUMANOID_MASS`], creatures [`creature_mass`] (else
/// their capsules' volume × water density).
pub fn ragdoll_params(skeleton: &str, humanoid: bool) -> sf_ragdoll::CookParams {
    let lower = skeleton.to_ascii_lowercase();
    let policy = if DEATH_CLIP_ONLY.iter().any(|s| lower.contains(s)) {
        sf_ragdoll::RagdollPolicy::DeathClip
    } else {
        sf_ragdoll::RagdollPolicy::Ragdoll
    };
    sf_ragdoll::CookParams { policy, total_mass: if humanoid { Some(HUMANOID_MASS) } else { creature_mass(skeleton) }, ..sf_ragdoll::CookParams::default() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creature_masses_are_plausible() {
        let m = |s: &str| creature_mass(s).unwrap_or_else(|| panic!("{s}"));
        assert!(CREATURE_MASSES.iter().all(|&(_, kg)| (0.5..=3000.0).contains(&kg)));
        assert!((m("r/Netch_Bull.NIF") - 600.0).abs() < 1e-3);
        assert!((m("r/CliffRacer.nif") - 30.0).abs() < 1e-3);
        assert!((m("r/Guar.nif") - 400.0).abs() < 1e-3);
        assert!((m("r/Kwama Queen.NIF") - 1500.0).abs() < 1e-3);
        assert!((m("r/Werewolf.nif") - 120.0).abs() < 1e-3);
        assert!(m("r/Guar.nif") > 10.0 * m("r/CliffRacer.nif"));
        assert!(m("r/Horker.nif") > m("r/Rat.nif"));
        assert_eq!(creature_mass("r/Unknown.nif"), None);
    }

    /// The physics-adapter.md table (S = 30, 50, 100) and the thresholds it implies.
    #[test]
    fn strength_maps_to_the_table() {
        for (s, force, carry, drag, throw) in [(30.0, 160.0, 11.0, 88.0, 4.4), (50.0, 240.0, 15.0, 120.0, 6.0), (100.0, 440.0, 25.0, 200.0, 10.0)] {
            let l = holder_limits(s);
            for (got, want) in [(l.carry_force, force), (l.carry_mass, carry), (l.drag_mass, drag), (l.throw_impulse, throw)] {
                assert!((got - want).abs() < 1e-4, "S {s}: {l:?}");
            }
        }
        // A bucket-sized 12 kg item: the weak drag it, the average carry it.
        assert_eq!(holder_limits(30.0).mode_for(12.0), Some(sf_physics::GrabMode::Drag));
        assert_eq!(holder_limits(DEFAULT_STRENGTH).mode_for(12.0), Some(sf_physics::GrabMode::Carry));
        // A heavy 100 kg body: too heavy at S 30, dragged at S 50.
        assert_eq!(holder_limits(30.0).mode_for(100.0), None);
        assert_eq!(holder_limits(50.0).mode_for(100.0), Some(sf_physics::GrabMode::Drag));
    }

    #[test]
    fn mass_from_weight_or_volume_clamped() {
        assert!((item_mass(10.0, 0.0) - 5.0).abs() < 1e-6);
        assert!((item_mass(0.0, 0.001) - 0.4).abs() < 1e-6, "weightless: volume × 400");
        assert!((item_mass(0.1, 0.0) - MASS_RANGE.0).abs() < 1e-6);
        assert!((item_mass(500.0, 0.0) - MASS_RANGE.1).abs() < 1e-6);
    }
}
