//! Variant libraries as an `sf_bake` product (feature `bake`; rockgen.md "When generation runs": boulder libraries
//! bake once, keyed by the parameters and the seed; slice RG-2b).
//!
//! | Step | Subject | Reads | Output |
//! |---|---|---|---|
//! | `rockgen.library` | class name (e.g. `"talus"`) | nothing (a pure function of its planned parameters) | postcard [`Library`] |
//!
//! The host builds a [`LibraryStep`] from the class parameters it resolved (the tunables' values, the adapter's
//! presets), registers it and bakes [`LibraryStep::keys`]. The plan carries [`library_key`] of each class (every
//! clamped parameter, the settings, the seed and the generator's format version) and whether hulls are built, so
//! changing any of them re-bakes that class and nothing else. The step version is the library format version
//! (`library.rs`), bumped with any generator change. Consumers read the product with [`decode`].

use std::collections::BTreeMap;

use sf_bake::{BakeError, PlanCx, ProductKey, RunCx, Step};

use crate::library::LIBRARY_FORMAT;
use crate::noise::mix64;
use crate::{BoulderParams, Library, LibrarySettings, RockClass, build_library, library_key};

/// The step id.
pub const LIBRARY_STEP: &str = "rockgen.library";

/// Library seed of the default library set (the RG-2 measurement's seed). Each class derives its own seed from it
/// ([`class_seed`]).
// tunables: const (seed: names the library set, not a quality/cost trade-off)
pub const LIBRARY_SEED: u64 = 1;

/// Whether this build puts collision hulls in the product (feature `collision`): planned, so a build with the
/// other setting never reuses a product without (or with) hulls.
// tunables: const (build configuration: mirrors the `collision` feature)
const HULLS: bool = cfg!(feature = "collision");

/// Everything one class's library is made from.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LibrarySpec {
    pub params: BoulderParams,
    pub settings: LibrarySettings,
    pub seed: u64,
}

impl LibrarySpec {
    /// The library's key ([`library_key`] of the spec).
    pub fn key(&self) -> u64 {
        library_key(&self.params, &self.settings, self.seed)
    }
}

/// The seed of class `name` in the library set seeded `seed`: classes never share variant seeds, so two classes
/// with close parameters don't come out as near-copies.
pub fn class_seed(seed: u64, name: &str) -> u64 {
    // FNV-1a 64 of the name, mixed with the set seed.
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in name.bytes() {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x0100_0000_01b3);
    }
    mix64(seed ^ mix64(h))
}

/// The `rockgen.library` step: one product per named class.
pub struct LibraryStep {
    specs: BTreeMap<String, LibrarySpec>,
}

impl LibraryStep {
    /// A step over explicit specs, keyed by subject.
    pub fn new(specs: impl IntoIterator<Item = (String, LibrarySpec)>) -> Self {
        Self { specs: specs.into_iter().collect() }
    }

    /// One library per class: its parameters over `base` (the live `rockgen.boulder.*` values), the shared
    /// `settings`, and [`class_seed`]`(seed, class.name)`.
    pub fn from_classes(classes: &[RockClass], base: BoulderParams, settings: LibrarySettings, seed: u64) -> Self {
        Self::new(classes.iter().map(|c| {
            (c.name.to_owned(), LibrarySpec { params: c.params(base), settings, seed: class_seed(seed, c.name) })
        }))
    }

    /// The product keys of every class, in name order.
    pub fn keys(&self) -> Vec<ProductKey> {
        self.specs.keys().map(|name| ProductKey::new(LIBRARY_STEP, name.as_str())).collect()
    }

    pub fn spec(&self, subject: &str) -> Option<&LibrarySpec> {
        self.specs.get(subject)
    }

    fn spec_or_err(&self, subject: &str) -> Result<&LibrarySpec, BakeError> {
        self.spec(subject)
            .ok_or_else(|| BakeError::step(&ProductKey::new(LIBRARY_STEP, subject), "no such rock class in this step"))
    }
}

impl Step for LibraryStep {
    fn id(&self) -> &'static str {
        LIBRARY_STEP
    }

    fn version(&self) -> u32 {
        LIBRARY_FORMAT
    }

    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        let spec = self.spec_or_err(subject)?;
        cx.param("key", spec.key().to_le_bytes().to_vec());
        cx.param("hulls", vec![u8::from(HULLS)]);
        Ok(())
    }

    fn run(&self, subject: &str, _cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let spec = self.spec_or_err(subject)?;
        let library = build_library(&spec.params, &spec.settings, spec.seed);
        encode(&library).map_err(|e| BakeError::step(&ProductKey::new(LIBRARY_STEP, subject), e))
    }
}

/// The product bytes of a library (postcard; the store compresses them).
pub fn encode(library: &Library) -> Result<Vec<u8>, postcard::Error> {
    postcard::to_stdvec(library)
}

/// Reads a `rockgen.library` product.
pub fn decode(bytes: &[u8]) -> Result<Library, postcard::Error> {
    postcard::from_bytes(bytes)
}

#[cfg(test)]
mod tests;
