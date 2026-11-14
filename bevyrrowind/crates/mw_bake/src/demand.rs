//! Texture demand of the static scene (`superfargoth/docs/specs/textures.md` §6, slice S3b): the
//! framework's `mesh.uv_density` + `texture.demand` steps with Morrowind's texture list per
//! material ([`MwDemandSource`]).
//!
//! A material draws its base texture, its glow map ([`extras::GLOW_TEXTURE`]) and the companions
//! the texture cook published for the base (normal map, ORM slot, depth map; `cook::CookIndex`),
//! which is exactly what the runtime material builder binds (`mw_assets` `cooked.rs` `surface`).
//! They all sample UV channel 0, so they share the material's ρ. The companions come from the
//! `texture.cook.index` product, so [`bake_demand`] runs after [`crate::cook::bake_cook`]; their
//! digest is the adapter's fingerprint, so a pack that adds or drops a companion re-bakes the
//! demand of every cell (cheap: no mesh is re-cooked, every density is a hit).

use std::collections::BTreeMap;
use std::sync::Arc;

use sf_bake::{BakeError, Baker, Digest, ProductKey};
use sf_model::Material;
use sf_scene::demand::bake::{DemandBake, DemandReport, DemandSource};
use sf_world::WorldspaceId;

use crate::cook::{CookIndex, INDEX_STEP, INDEX_SUBJECT, OrmCompanion, companion_of, depth_of, orm_of};
use crate::extras;

/// Bump when [`MwDemandSource::textures`] changes.
const RULE_VERSION: u8 = 1;

/// Morrowind's [`DemandSource`]: base + glow + the cook's companions.
pub struct MwDemandSource {
    normals: BTreeMap<String, String>,
    orms: BTreeMap<String, OrmCompanion>,
    depths: BTreeMap<String, String>,
    fingerprint: [u8; 32],
}

impl MwDemandSource {
    /// From the cook index's companion tables.
    pub fn new(normals: BTreeMap<String, String>, orms: BTreeMap<String, OrmCompanion>, depths: BTreeMap<String, String>) -> Self {
        let bytes = postcard::to_stdvec(&(RULE_VERSION, &normals, &orms, &depths)).expect("companion tables serialize");
        Self { fingerprint: Digest::of(&bytes).0, normals, orms, depths }
    }
}

impl DemandSource for MwDemandSource {
    fn fingerprint(&self) -> [u8; 32] {
        self.fingerprint
    }
    fn textures(&self, m: &Material, out: &mut Vec<String>) {
        if let Some(glow) = m.extras.get(extras::GLOW_TEXTURE) {
            out.push(glow.clone());
        }
        let Some(base) = m.base_color_texture.as_deref() else { return };
        out.push(base.to_owned());
        out.extend(companion_of(&self.normals, base).map(str::to_owned));
        out.extend(orm_of(&self.orms, base).map(|o| o.file.clone()));
        out.extend(depth_of(&self.depths, base).map(str::to_owned));
    }
}

/// The companion tables of this baker's texture cook (`texture.cook.index`).
pub fn demand_source(baker: &Baker) -> Result<MwDemandSource, BakeError> {
    let key = ProductKey::new(INDEX_STEP, INDEX_SUBJECT);
    let digest = baker.product(&key).map_err(|e| {
        BakeError::Other(format!(
            "texture.demand: no {INDEX_STEP} product ({e}). Bake the textures (mw_bake::cook::bake_cook) \
             before bake_demand: the companions (normal, ORM, depth maps) it lists are textures the cells draw."
        ))
    })?;
    let index: CookIndex = postcard::from_bytes(&baker.store().get(&digest)?)
        .map_err(|e| BakeError::step(&key, format!("malformed cook index: {e}")))?;
    Ok(MwDemandSource::new(index.normals, index.orms, index.depths))
}

/// Bakes `texture.demand` for every cell of `worldspaces` (and `mesh.uv_density` for every model
/// they place). After the texture cook ([`demand_source`]) and the world index.
pub fn bake_demand(baker: &mut Baker, worldspaces: &[WorldspaceId]) -> Result<DemandReport, BakeError> {
    Arc::new(DemandBake::new(Arc::new(demand_source(baker)?))).register(baker);
    Ok(DemandBake::bake_worldspaces(baker, worldspaces))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_material_draws_its_base_glow_and_companions() {
        let source = MwDemandSource::new(
            BTreeMap::from([("textures/tx_a".into(), "textures/tx_a_n.dds".into())]),
            BTreeMap::from([("textures/tx_a".into(), OrmCompanion { file: "textures/tx_a_spec.dds".into(), occlusion: true })]),
            BTreeMap::from([("textures/tx_a".into(), "textures/tx_a_depth.ktx2".into())]),
        );
        let mut m = Material { base_color_texture: Some("Textures\\TX_A.dds".into()), ..Material::default() };
        m.extras.insert(extras::GLOW_TEXTURE.into(), "textures/tx_a_glow.dds".into());
        let mut out = Vec::new();
        source.textures(&m, &mut out);
        assert_eq!(out, ["textures/tx_a_glow.dds", "Textures\\TX_A.dds", "textures/tx_a_n.dds", "textures/tx_a_spec.dds", "textures/tx_a_depth.ktx2"]);
        // A texture without companions: the base alone; the tables are the fingerprint.
        let plain = MwDemandSource::new(BTreeMap::new(), BTreeMap::new(), BTreeMap::new());
        out.clear();
        plain.textures(&Material { base_color_texture: Some("textures/b.dds".into()), ..Material::default() }, &mut out);
        assert_eq!(out, ["textures/b.dds"]);
        assert_ne!(plain.fingerprint(), source.fingerprint());
    }
}
