//! Material classes (static-scene.md §5): the few pipeline-relevant kinds every scene material folds into.
//!
//! In a GPU-driven renderer the draw count is the number of batch sets (pipeline × bind group × mesh slab), so
//! statics share a handful of classes instead of the union of their source materials. A class fixes only what
//! changes the pipeline: alpha mode (opaque or masked), the wind vertex function, and face culling. Everything
//! else (textures, tint, roughness, alpha cutoff, UV transform) is per-instance data in the bindless material
//! table. Blended and additive materials are never scene materials (§1): they stay entities, sorted.
//!
//! The class is a pure function of the material, so format v2 stores it once per **chain**, as a `u16`
//! ([`crate::LodChain::material_class`]), not per instance (v1 did). Values are stable across versions: a new
//! class takes a new value, an old value never changes meaning.

/// One of the scene's material classes. `repr(u16)` values are the on-disk and GPU encoding.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u16)]
pub enum MaterialClass {
    Opaque = 0,
    OpaqueTwoSided = 1,
    Masked = 2,
    MaskedTwoSided = 3,
    OpaqueWind = 4,
    OpaqueWindTwoSided = 5,
    MaskedWind = 6,
    MaskedWindTwoSided = 7,
}

/// The pipeline-relevant traits of a source material, as the adapter reads them.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MaterialTraits {
    /// Alpha-tested (cutout). False: opaque.
    pub masked: bool,
    /// Both faces drawn (no back-face culling).
    pub two_sided: bool,
    /// Sways with the wind field (a vertex function of time).
    pub wind: bool,
}

impl MaterialClass {
    /// Every class, in value order.
    pub const ALL: [Self; 8] = [
        Self::Opaque,
        Self::OpaqueTwoSided,
        Self::Masked,
        Self::MaskedTwoSided,
        Self::OpaqueWind,
        Self::OpaqueWindTwoSided,
        Self::MaskedWind,
        Self::MaskedWindTwoSided,
    ];

    /// The class of a material with these traits.
    #[must_use]
    pub fn of(traits: MaterialTraits) -> Self {
        let bits = u16::from(traits.two_sided) | u16::from(traits.masked) << 1 | u16::from(traits.wind) << 2;
        Self::ALL[usize::from(bits)]
    }

    /// The traits this class stands for.
    #[must_use]
    pub fn traits(self) -> MaterialTraits {
        let bits = self as u16;
        MaterialTraits { two_sided: bits & 1 != 0, masked: bits & 2 != 0, wind: bits & 4 != 0 }
    }

    /// The class encoded as `value`, if it is one.
    #[must_use]
    pub fn from_u16(value: u16) -> Option<Self> {
        Self::ALL.get(usize::from(value)).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_trait_combination_round_trips() {
        for masked in [false, true] {
            for two_sided in [false, true] {
                for wind in [false, true] {
                    let traits = MaterialTraits { masked, two_sided, wind };
                    let class = MaterialClass::of(traits);
                    assert_eq!(class.traits(), traits);
                    assert_eq!(MaterialClass::from_u16(class as u16), Some(class));
                }
            }
        }
    }

    #[test]
    fn values_are_stable_and_unknown_values_rejected() {
        assert_eq!(MaterialClass::Opaque as u16, 0);
        assert_eq!(MaterialClass::MaskedWindTwoSided as u16, 7);
        for (i, class) in MaterialClass::ALL.iter().enumerate() {
            assert_eq!(*class as usize, i);
        }
        assert_eq!(MaterialClass::from_u16(8), None);
        assert_eq!(MaterialClass::from_u16(u16::MAX), None);
    }
}
