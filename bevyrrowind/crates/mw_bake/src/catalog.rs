//! The Morrowind enumeration of `pipeline.catalog` (`superfargoth/docs/specs/pipeline-warmup.md`
//! §9, slice W4a): which cooked models the catalog reads, the flags their entities get, and how a
//! cooked NIF material reads as a [`MaterialSignature`].
//!
//! - **Models:** every referenced model's `mesh.cook` product (statics; skinned body parts and
//!   creatures cook as `Cooked::Skinned`) and every creature body's `mesh.rig` product
//!   ([`crate::anim::ActorSets::body_rig_keys`]). Body parts, creatures and NPC meshes get actor
//!   flags (no crossfade); everything else static flags (cooked levels are drawn with a dithered
//!   visibility range, `mw_assets::cooked`). Models `mesh.cook` skips (animated nodes: the runtime
//!   imports the NIF) are counted, not enumerated (W4b follow-up).
//! - **Material signature:** the fields `mw_assets::Surface::to_material` turns into
//!   variant-selecting `StandardMaterial` state, before the runtime's class folding (a signature
//!   may split finer than the pipelines, never coarser): the runtime alpha mode (cutout candidates
//!   resolved by the texture bake's alpha classes, as `scene.cells` does, [`crate::scene`]),
//!   additive, two-sided, unlit, foliage (diffuse transmission), and whether the base texture has a
//!   normal-map or depth (parallax) companion in the texture cook's index. Runtime A/B switches
//!   (`--off leafmask`, `--off flatnormal`, presets without parallax) are not keys: the
//!   representative loads through the live material path, so it picks them up.
//! - **Builtins:** terrain, water, sky and precipitation, one each by name (W4b maps them).
//!   Particles (`ParticleExt`) and far-field proxies are not enumerated yet (spec follow-up).

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use sf_bake::{BakeError, Baker, Digest, ProductKey, RunCx};
use sf_model::catalog::{
    CATALOG_STEP, CATALOG_SUBJECT, Catalog, CatalogModel, CatalogSource, CatalogStep, EntityFlags, MaterialReader,
    MaterialSignature,
};
use sf_model::steps::MeshCookStep;
use sf_model::{AlphaMode, Material};
use sf_records::RecordDb;
use tes3::esp::{Bodypart, Creature, Npc};

use crate::cook::{CookIndex, INDEX_STEP, INDEX_SUBJECT, companion_of, depth_of};
use crate::delight::{ALPHA_INDEX_STEP, ALPHA_INDEX_SUBJECT, AlphaClassIndex};
use crate::extras;
use crate::scene::{AlphaClasses, runtime_alpha};

/// Bump when the rules above change (part of the catalog's key).
pub const RULES_VERSION: u32 = 1;

/// Statics: shadow casters drawn with a crossfading visibility range.
pub const STATIC_FLAGS: EntityFlags = EntityFlags { shadow_caster: true, crossfade: true };
/// Actor bodies and body parts: shadow casters without a range.
pub const ACTOR_FLAGS: EntityFlags = EntityFlags { shadow_caster: true, crossfade: false };

/// The specimens no cooked model carries ([`sf_model::catalog::Catalog::builtins`]).
pub const BUILTINS: [&str; 4] = ["precipitation", "sky", "terrain", "water"];

/// The catalog models of `paths` (the importable referenced models, [`crate::referenced_models`])
/// and `rig_keys` (`mesh.rig` creature bodies): actor flags for meshes body parts, creatures and
/// NPCs name, static flags otherwise.
pub fn catalog_models(db: &RecordDb, paths: &[String], rig_keys: &[ProductKey]) -> Vec<CatalogModel> {
    let mut actors: BTreeSet<String> = BTreeSet::new();
    actors.extend(db.iter::<Bodypart>().filter_map(|(_, r)| crate::model_path(&r.mesh)));
    actors.extend(db.iter::<Creature>().filter_map(|(_, r)| crate::model_path(&r.mesh)));
    actors.extend(db.iter::<Npc>().filter_map(|(_, r)| crate::model_path(&r.mesh)));
    let cooks = paths.iter().map(|p| CatalogModel {
        step: MeshCookStep::ID.to_owned(),
        subject: p.clone(),
        flags: if actors.contains(p) { ACTOR_FLAGS } else { STATIC_FLAGS },
    });
    let rigs = rig_keys.iter().map(|k| CatalogModel { step: k.step.clone(), subject: k.subject.clone(), flags: ACTOR_FLAGS });
    cooks.chain(rigs).collect()
}

/// The Morrowind [`CatalogSource`].
pub struct MwCatalogSource {
    models: Vec<CatalogModel>,
    /// Read the texture bake's alpha classes and cook index (the game bake); without them (tests)
    /// no candidate is a cutout and no texture has a companion.
    textures: bool,
}

impl MwCatalogSource {
    pub fn new(models: Vec<CatalogModel>) -> Self {
        Self { models, textures: true }
    }

    /// Without the texture products (tests, or a bake that ran no texture steps).
    #[must_use]
    pub fn without_textures(mut self) -> Self {
        self.textures = false;
        self
    }
}

/// What the material reading needs from the texture bake.
#[derive(Default)]
struct TextureFacts {
    alpha: AlphaClasses,
    normals: BTreeMap<String, String>,
    depths: BTreeMap<String, String>,
}

impl TextureFacts {
    fn read(cx: &mut RunCx<'_>) -> Result<Self, BakeError> {
        let key = || ProductKey::new(CATALOG_STEP, CATALOG_SUBJECT);
        let alpha: AlphaClassIndex = postcard::from_bytes(&cx.product(ALPHA_INDEX_STEP, ALPHA_INDEX_SUBJECT)?)
            .map_err(|e| BakeError::step(&key(), format!("malformed alpha class index: {e}")))?;
        let index: CookIndex = postcard::from_bytes(&cx.product(INDEX_STEP, INDEX_SUBJECT)?)
            .map_err(|e| BakeError::step(&key(), format!("malformed texture cook index: {e}")))?;
        Ok(Self { alpha: AlphaClasses::new(alpha.classes), normals: index.normals, depths: index.depths })
    }
}

/// The signature of a cooked NIF material (module docs).
fn signature(m: &Material, textures: &TextureFacts) -> MaterialSignature {
    let blend = m.extras.get(extras::BLEND).map(String::as_str);
    let alpha = runtime_alpha(m, &textures.alpha);
    let alpha_name = match alpha {
        AlphaMode::Opaque => "opaque",
        AlphaMode::Mask(_) => "mask",
        AlphaMode::Blend if blend == Some("additive") => "additive",
        AlphaMode::Blend => "blend",
    };
    // `mw_assets::cooked::surface`: the leaf rule; `Surface::to_material`: unlit maps nothing.
    let foliage = matches!(blend, Some("cutout" | "cutout_candidate")) && matches!(alpha, AlphaMode::Mask(_));
    let lit = !m.unlit;
    let base = m.base_color_texture.as_deref();
    let normal = lit && base.is_some_and(|b| companion_of(&textures.normals, b).is_some());
    let depth = lit && base.is_some_and(|b| depth_of(&textures.depths, b).is_some());
    MaterialSignature::new("standard")
        .with("alpha", alpha_name)
        .with("two_sided", m.double_sided)
        .with("unlit", m.unlit)
        .with("foliage", foliage && lit)
        .with("normal_map", normal)
        .with("depth_map", depth)
}

impl CatalogSource for MwCatalogSource {
    fn fingerprint(&self) -> [u8; 32] {
        let mut bytes = RULES_VERSION.to_le_bytes().to_vec();
        bytes.push(u8::from(self.textures));
        Digest::of(&bytes).0
    }
    fn models(&self) -> &[CatalogModel] {
        &self.models
    }
    fn reader<'a>(&'a self, cx: &mut RunCx<'_>) -> Result<Box<MaterialReader<'a>>, BakeError> {
        let textures = if self.textures { TextureFacts::read(cx)? } else { TextureFacts::default() };
        Ok(Box::new(move |m| Some(signature(m, &textures))))
    }
    fn builtins(&self) -> Vec<String> {
        BUILTINS.iter().map(|&b| b.to_owned()).collect()
    }
}

/// Registers `pipeline.catalog` over `source` and bakes it (after the model, rig and texture
/// steps). Returns the catalog for the bake's summary line.
pub fn bake_catalog(baker: &mut Baker, source: MwCatalogSource) -> Result<Catalog, BakeError> {
    baker.register(CatalogStep(Arc::new(source)));
    let key = ProductKey::new(CATALOG_STEP, CATALOG_SUBJECT);
    if let Some((_, error)) = baker.bake(std::slice::from_ref(&key)).failures().next() {
        return Err(BakeError::Other(error.to_string()));
    }
    let bytes = baker.store().get(&baker.product(&key)?)?;
    Catalog::decode(&bytes).map_err(|e| BakeError::step(&key, e))
}

#[cfg(test)]
mod tests;
