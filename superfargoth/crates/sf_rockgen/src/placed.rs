//! Library rocks placed in a game world (spec `rockgen.md` "(f) Placement in the game", slice RG-P1).
//!
//! The scatter keeps library rocks as (class name, variant hash, size mark, sink). [`Libraries`] holds every
//! class's decoded variant library, loaded once ([`read_libraries`], feature `bake`) and shared by the content
//! builder, the spawner and the physics shape source. [`Libraries::resolve`] turns a scatter rock into the variant
//! index, its uniform scale and the lift that seats it at its sink; [`model_id`] names a variant
//! (`rockgen/<class>/<index>`) so a streamed placement can carry it as its model, and [`parse_model_id`] reads it
//! back. Bevy-free; nothing here names a game.

use std::collections::BTreeMap;

use crate::{Library, Variant};

/// First segment of every library variant's model id. Never a mounted folder: consumers match the prefix before
/// any asset lookup.
// tunables: const (format: the id scheme of library variants)
pub const MODEL_PREFIX: &str = "rockgen";

/// The model id of variant `index` of class `class`: `rockgen/<class>/<index>`. Class names are lowercase ASCII
/// without separators (`crate::CLASSES`), so a lowercasing path type keeps the id unchanged.
#[must_use]
pub fn model_id(class: &str, index: u32) -> String {
    format!("{MODEL_PREFIX}/{class}/{index}")
}

/// Reads a [`model_id`] back into (class, index). `None` for another prefix, an empty class, a class with a
/// separator, or an index that is not a canonical decimal `u32` (digits only, no sign, no leading zero), so
/// `model_id(parse_model_id(id)) == id` for every id it accepts.
#[must_use]
pub fn parse_model_id(id: &str) -> Option<(&str, u32)> {
    let rest = id.strip_prefix(MODEL_PREFIX)?.strip_prefix('/')?;
    let (class, index) = rest.split_once('/')?;
    if class.is_empty() || index.is_empty() || !index.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    if index.len() > 1 && index.starts_with('0') {
        return None;
    }
    Some((class, index.parse().ok()?))
}

/// One scatter rock resolved against its class's library.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Resolved {
    /// The variant: the scatter's hash modulo the class's variant count.
    pub index: u32,
    /// Uniform instance scale: size mark / the library's reference size ([`Library::scale_for`]).
    pub scale: f32,
    /// How far to move the rock along its rotated up so `sink` of its short axis is below the ground, m (the mesh
    /// is centred and its +Z is the short axis: `(0.5 - sink) x extent.z x scale`; negative sinks it deeper).
    pub lift_m: f32,
}

/// Every class's variant library, by class name.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Libraries {
    by_class: BTreeMap<String, Library>,
}

impl Libraries {
    /// Libraries from (class name, library) pairs; a later pair of the same name replaces an earlier one.
    pub fn new(libraries: impl IntoIterator<Item = (String, Library)>) -> Self {
        Self { by_class: libraries.into_iter().collect() }
    }

    #[must_use]
    pub fn get(&self, class: &str) -> Option<&Library> {
        self.by_class.get(class)
    }

    /// The class names, in name order.
    pub fn classes(&self) -> impl Iterator<Item = &str> {
        self.by_class.keys().map(String::as_str)
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.by_class.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.by_class.is_empty()
    }

    /// Variant `index` of class `class`.
    #[must_use]
    pub fn variant(&self, class: &str, index: u32) -> Option<&Variant> {
        self.get(class)?.variants.get(usize::try_from(index).ok()?)
    }

    /// The variant, scale and lift of a scatter rock of class `class` with variant hash `variant_hash`, size mark
    /// `size_mark_m` (longest side, m) and `sink` (share of the short axis below the ground). `None` for an
    /// unknown class or an empty library (the caller drops and counts the rock). Modulo, not [`Library::pick`]:
    /// the scatter's hash is already uniform 64-bit and modulo is its documented contract.
    #[must_use]
    pub fn resolve(&self, class: &str, variant_hash: u64, size_mark_m: f32, sink: f32) -> Option<Resolved> {
        let library = self.get(class)?;
        let count = u64::try_from(library.variants.len()).ok().filter(|&n| n > 0)?;
        let index = u32::try_from(variant_hash % count).ok()?;
        let variant = library.variants.get(usize::try_from(index).ok()?)?;
        let scale = library.scale_for(size_mark_m);
        let lift_m = (0.5 - sink) * variant.lods.bounds.extent()[2] * scale;
        Some(Resolved { index, scale, lift_m })
    }
}

/// Why [`read_libraries`] failed.
#[cfg(feature = "bake")]
#[derive(Debug)]
pub enum ReadError {
    /// The manifest names a product the store cannot read.
    Store { class: String, error: std::io::Error },
    /// The product does not decode as a library.
    Decode { class: String, error: postcard::Error },
}

#[cfg(feature = "bake")]
impl std::fmt::Display for ReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Store { class, error } => {
                write!(f, "rock library `{class}`: store read failed: {error}")
            }
            Self::Decode { class, error } => write!(f, "rock library `{class}`: product does not decode: {error}"),
        }
    }
}

#[cfg(feature = "bake")]
impl std::error::Error for ReadError {}

/// The `rockgen.library` products of `classes` (the product subject is the class name) from a baked store. A class
/// the manifest lacks is absent from the result, not an error; a product the store cannot read or decode is.
#[cfg(feature = "bake")]
pub fn read_libraries<'a>(
    store: &sf_bake::Store,
    manifest: &sf_bake::Manifest,
    classes: impl IntoIterator<Item = &'a str>,
) -> Result<Libraries, ReadError> {
    let mut by_class = BTreeMap::new();
    for class in classes {
        let Some(digest) = manifest.get(crate::bake::LIBRARY_STEP, class) else {
            continue;
        };
        let bytes = store.get(&digest).map_err(|error| ReadError::Store { class: class.to_owned(), error })?;
        let library =
            crate::bake::decode(&bytes).map_err(|error| ReadError::Decode { class: class.to_owned(), error })?;
        by_class.insert(class.to_owned(), library);
    }
    Ok(Libraries { by_class })
}

#[cfg(test)]
mod tests;
