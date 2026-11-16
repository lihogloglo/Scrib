//! `pipeline.catalog`: the distinct render-pipeline specimens of a profile's cooked content
//! (pipeline-warmup.md §7, slice W4a).
//!
//! A specimen is what selects a pipeline variant, as data that does not depend on the engine
//! version: the cooked submesh's [`VertexLayout`], the adapter's [`MaterialSignature`] (material
//! type + the fields that select a shader variant, after the adapter's own folding) and the
//! [`EntityFlags`] the spawner sets. Each distinct specimen keeps one [`Representative`] (the first
//! cooked submesh, in model order, that has it), which the runtime loads and draws ahead of time
//! through the normal spawn path (W4b), so the warm-up compiles exactly the pipelines play will
//! ask for. The model of this is UE5's PSO precaching / bundled PSO cache (cook-time
//! (vertex factory, material, pass) tuples) and Unity's `ShaderVariantCollection`; passes and view
//! keys are not in the catalog: the runtime precache crosses specimens with the live views.
//!
//! The adapter ([`CatalogSource`]) lists the cooked models (`mesh.cook` statics and skinned
//! bodies, `mesh.rig` creature bodies, ...), reads each cooked material into a signature and names
//! the specimens that come from no cooked model ([`Catalog::builtins`]: terrain, water, sky, ...).
//! Reads go through the bake, so the catalog changes with content and mods like any product;
//! computing it needs no GPU. Format: postcard [`Catalog`], one product per profile (subject
//! [`CATALOG_SUBJECT`]); [`load_catalog`] reads it back.

use std::collections::BTreeMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use sf_bake::{BakeError, Manifest, PlanCx, ProductKey, RunCx, Step, Store};

use crate::{AlphaMode, Cooked, CookedModel, Material};

/// The step's id.
pub const CATALOG_STEP: &str = "pipeline.catalog";
/// Its only subject (the profile is the manifest it lands in).
pub const CATALOG_SUBJECT: &str = "all";
/// Bump when [`Catalog`]'s encoding or the enumeration rule changes (also the step version).
pub const CATALOG_FORMAT: u32 = 1;

/// The vertex attributes a cooked submesh hands the renderer, beyond the position and normal
/// every one has (the loader computes normals where the source has none).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[allow(clippy::struct_excessive_bools)] // one presence flag per attribute stream
pub struct VertexLayout {
    pub uvs: bool,
    pub colors: bool,
    pub tangents: bool,
    /// Joint indices and weights.
    pub skinned: bool,
    /// Morph targets (a mesh key bit, not a vertex attribute, but per submesh like one).
    pub morphs: bool,
}

/// What the adapter's material reading says selects a variant: a material type (the runtime
/// material it becomes, e.g. `"standard"`) and named fields after the adapter's folding. Two
/// materials with equal signatures must draw with the same pipelines on the same layout; a
/// signature may split more finely than needed (a duplicate specimen costs a hash hit), never
/// less.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct MaterialSignature {
    pub material_type: String,
    pub fields: BTreeMap<String, String>,
}

impl MaterialSignature {
    pub fn new(material_type: impl Into<String>) -> Self {
        Self { material_type: material_type.into(), fields: BTreeMap::new() }
    }

    /// Adds `name = value`.
    #[must_use]
    pub fn with(mut self, name: impl Into<String>, value: impl ToString) -> Self {
        self.fields.insert(name.into(), value.to_string());
        self
    }
}

/// The entity-level bits a spawner sets that select a variant.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct EntityFlags {
    pub shadow_caster: bool,
    /// Drawn with a dithered visibility-range crossfade (LOD levels, fade bands).
    pub crossfade: bool,
}

/// One cooked model the adapter enumerates: a product of a mesh step (`mesh.cook`, `mesh.rig`,
/// ...) whose bytes are a [`Cooked`], and the flags its entities get.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CatalogModel {
    pub step: String,
    pub subject: String,
    pub flags: EntityFlags,
}

/// The submesh to load as a specimen's stand-in.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Representative {
    /// The mesh step and subject of the cooked model ([`CatalogModel`]).
    pub step: String,
    pub model: String,
    pub lod: u32,
    /// Index into that level's submeshes.
    pub submesh: u32,
    /// Index into the model's materials.
    pub material: u32,
}

/// One distinct specimen.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CatalogEntry {
    pub layout: VertexLayout,
    pub material: MaterialSignature,
    pub flags: EntityFlags,
    pub representative: Representative,
    /// Cooked submeshes (every level) with this specimen: a measurement.
    pub uses: u32,
}

/// Counts of what the enumeration saw (measurements, printed by the bake).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CatalogStats {
    /// Models read (static or skinned cooks).
    pub models: u32,
    pub submeshes: u32,
    /// `Cooked::Skipped`: the runtime imports them, their specimens are not here.
    pub skipped: u32,
    /// Products that failed to bake or decode.
    pub failed: u32,
    /// Submeshes whose material the adapter keeps out (not drawn, or drawn by another path).
    pub excluded: u32,
}

/// The `pipeline.catalog` product.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Catalog {
    pub format: u32,
    /// Sorted by (layout, material, flags), each unique.
    pub entries: Vec<CatalogEntry>,
    /// Specimens no cooked model carries, by the adapter's names (sorted, unique); the runtime
    /// maps each name to its material (terrain, water, sky, ...).
    pub builtins: Vec<String>,
    pub stats: CatalogStats,
}

impl Catalog {
    pub fn decode(bytes: &[u8]) -> Result<Self, postcard::Error> {
        postcard::from_bytes(bytes)
    }

    /// Entries with a skinned layout.
    pub fn skinned(&self) -> usize {
        self.entries.iter().filter(|e| e.layout.skinned).count()
    }
}

/// Reads a cooked material into its signature; `None` keeps it out of the catalog.
pub type MaterialReader<'a> = dyn Fn(&Material) -> Option<MaterialSignature> + 'a;

/// The adapter's side of the step.
pub trait CatalogSource: Send + Sync {
    /// Changes whenever any answer below would (rules version, settings). The model list is keyed
    /// separately.
    fn fingerprint(&self) -> [u8; 32];
    /// The cooked models to enumerate.
    fn models(&self) -> &[CatalogModel];
    /// The material reader for one run; read any product it needs through `cx` (tracked).
    /// Default: [`default_signature`].
    fn reader<'a>(&'a self, _cx: &mut RunCx<'_>) -> Result<Box<MaterialReader<'a>>, BakeError> {
        Ok(Box::new(|m| Some(default_signature(m))))
    }
    /// Specimens from no cooked model, by name.
    fn builtins(&self) -> Vec<String> {
        Vec::new()
    }
}

/// The game-agnostic reading of a material: type `"standard"`, alpha kind, two-sided, unlit.
pub fn default_signature(m: &Material) -> MaterialSignature {
    let alpha = match m.alpha {
        AlphaMode::Opaque => "opaque",
        AlphaMode::Mask(_) => "mask",
        AlphaMode::Blend => "blend",
    };
    MaterialSignature::new("standard").with("alpha", alpha).with("two_sided", m.double_sided).with("unlit", m.unlit)
}

/// The submesh layout of a cooked model (`morphs`: per submesh of a skinned one).
fn layout(model: &CookedModel, tangents: bool, skinned: bool, morphs: bool) -> VertexLayout {
    VertexLayout { uvs: model.has_uvs, colors: model.has_colors, tangents, skinned, morphs }
}

/// Distinct specimens -> their representative and use count, while enumerating.
type Specimens = BTreeMap<(VertexLayout, MaterialSignature, EntityFlags), (Representative, u32)>;

/// `pipeline.catalog` (module docs).
pub struct CatalogStep(pub Arc<dyn CatalogSource>);

impl CatalogStep {
    /// The catalog of `models` read with `read`, deterministic in the model order given (the step
    /// sorts it first). `cook` yields each model's cooked bytes or the error.
    fn enumerate(
        models: &[CatalogModel],
        read: &MaterialReader<'_>,
        mut cook: impl FnMut(&CatalogModel) -> Result<Vec<u8>, BakeError>,
    ) -> (Specimens, CatalogStats) {
        let mut found = Specimens::new();
        let mut stats = CatalogStats::default();
        for entry in models {
            let Ok(bytes) = cook(entry) else {
                stats.failed += 1;
                continue;
            };
            let Ok(cooked) = Cooked::from_bytes(&bytes) else {
                stats.failed += 1;
                continue;
            };
            let (model, skin) = match &cooked {
                Cooked::Static(model) => (model, None),
                Cooked::Skinned(skinned) => (&skinned.model, Some(skinned)),
                Cooked::Skipped { .. } => {
                    stats.skipped += 1;
                    continue;
                }
            };
            stats.models += 1;
            let signatures: Vec<Option<MaterialSignature>> = model.materials.iter().map(read).collect();
            for (lod, level) in model.lods.iter().enumerate() {
                for (index, sub) in level.submeshes.iter().enumerate() {
                    stats.submeshes += 1;
                    let Some(Some(signature)) = signatures.get(sub.material as usize) else {
                        stats.excluded += 1;
                        continue;
                    };
                    let morphs = skin
                        .and_then(|s| s.streams.get(lod).and_then(|l| l.get(index)))
                        .is_some_and(|streams| !streams.targets.is_empty());
                    let key = (layout(model, sub.has_tangents(), skin.is_some(), morphs), signature.clone(), entry.flags);
                    #[allow(clippy::cast_possible_truncation)] // cooked level and submesh counts are tiny
                    let representative = Representative {
                        step: entry.step.clone(),
                        model: entry.subject.clone(),
                        lod: lod as u32,
                        submesh: index as u32,
                        material: sub.material,
                    };
                    found.entry(key).or_insert((representative, 0)).1 += 1;
                }
            }
        }
        (found, stats)
    }
}

/// The model list in the step's order: sorted, unique.
fn sorted_models(source: &dyn CatalogSource) -> Vec<CatalogModel> {
    let mut models = source.models().to_vec();
    models.sort();
    models.dedup();
    models
}

impl Step for CatalogStep {
    fn id(&self) -> &'static str {
        CATALOG_STEP
    }
    /// 1: first catalog (W4a).
    fn version(&self) -> u32 {
        CATALOG_FORMAT
    }
    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        let models = postcard::to_stdvec(&sorted_models(&*self.0))
            .map_err(|e| BakeError::step(&ProductKey::new(CATALOG_STEP, subject), e))?;
        cx.param("models", models).param("source", self.0.fingerprint().to_vec());
        Ok(())
    }
    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let models = sorted_models(&*self.0);
        let read = self.0.reader(cx)?;
        // Products are read (and baked if needed) as they come, not planned: one model that fails
        // to cook leaves it out instead of failing the catalog. A failed read records nothing, so
        // the source file is noted instead (a fixed file re-runs the catalog).
        let (found, stats) = Self::enumerate(&models, &*read, |m| {
            cx.product(&m.step, &m.subject).inspect_err(|_| {
                let _ = cx.file_fingerprint(&m.subject);
            })
        });
        let mut builtins = self.0.builtins();
        builtins.sort();
        builtins.dedup();
        let entries = found
            .into_iter()
            .map(|((layout, material, flags), (representative, uses))| CatalogEntry { layout, material, flags, representative, uses })
            .collect();
        let catalog = Catalog { format: CATALOG_FORMAT, entries, builtins, stats };
        postcard::to_stdvec(&catalog).map_err(|e| BakeError::step(&ProductKey::new(CATALOG_STEP, subject), e))
    }
}

/// The profile's baked catalog, if its manifest has one that decodes.
pub fn load_catalog(store: &Store, manifest: &Manifest) -> Option<Catalog> {
    let digest = manifest.get(CATALOG_STEP, CATALOG_SUBJECT)?;
    Catalog::decode(&store.get(&digest).ok()?).ok()
}

#[cfg(test)]
mod tests;
