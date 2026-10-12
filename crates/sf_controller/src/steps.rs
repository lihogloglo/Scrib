//! The `collision.cook` bake step (locomotion.md "Ground probes"), on the
//! conventions of `sf_model::steps` and `sf_anim::steps`.
//!
//! Subject: a model path (the same subject as `mesh.cook`). Output: a
//! [`CollisionMesh`] (`SFCL`). The geometry is the model's authored
//! collision when the adapter has it ([`CollisionRules::authored`], e.g. a
//! legacy format's collision nodes), else a render LOD of the `mesh.cook`
//! product (LOD1 by default: collision needn't follow every bevel). Models
//! `mesh.cook` skipped (animated, skinned) get an empty mesh: no static
//! collision in M3. Mods can ship `collision/<model path>.sfcol`, which
//! wins (authored beats derived).
//!
//! Adapter data that isn't a file enters the key as parameters: the rules'
//! version and the cook settings.

use std::sync::Arc;

use glam::Affine3A;
use serde::{Deserialize, Serialize};
use sf_bake::{BakeError, Baker, PlanCx, ProductKey, RunCx, Step};
use sf_model::steps::MeshCookStep;
use sf_model::{Cooked, Material};

use crate::probe::SurfaceId;
use crate::statics::{COLLISION_FORMAT, CollisionBuilder, CollisionMesh};

pub const COLLISION_COOK: &str = "collision.cook";

/// How render geometry becomes collision.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct CollisionSettings {
    /// Render LOD used when there is no authored collision (clamped to the
    /// chain's length).
    pub lod: u8,
    /// Vertex weld grid, meters.
    pub weld: f32,
}

impl Default for CollisionSettings {
    fn default() -> Self {
        Self { lod: 1, weld: 1e-3 }
    }
}

/// Adapter hooks of `collision.cook`. Must be deterministic.
pub trait CollisionRules: Send + Sync {
    /// Static inputs of [`Self::authored`] (e.g. the model file). The
    /// `mesh.cook` product is always planned.
    fn plan(&self, _subject: &str, _cx: &mut PlanCx) {}
    /// Authored collision geometry for the model, read through `cx`; `None`
    /// derives it from the render LOD.
    fn authored(&self, _subject: &str, _cx: &mut RunCx<'_>) -> Result<Option<CollisionMesh>, BakeError> {
        Ok(None)
    }
    /// The surface of render triangles with this material.
    fn surface(&self, _material: &Material) -> SurfaceId {
        SurfaceId::DEFAULT
    }
    /// Bump when the rules' output changes (part of the key).
    fn version(&self) -> u32 {
        0
    }
}

/// Framework defaults: no authored collision, one surface.
#[derive(Clone, Copy, Debug, Default)]
pub struct DefaultCollisionRules;

impl CollisionRules for DefaultCollisionRules {}

/// `collision.cook`: a model's static collision mesh.
pub struct CollisionCookStep {
    pub rules: Arc<dyn CollisionRules>,
    pub settings: CollisionSettings,
}

impl CollisionCookStep {
    pub fn register(rules: Arc<dyn CollisionRules>, settings: CollisionSettings, baker: &mut Baker) {
        baker.register(Self { rules, settings });
    }
}

/// Collision from a cooked render model: one LOD, welded, surfaces by material.
pub fn from_cooked(cooked: &Cooked, settings: &CollisionSettings, rules: &dyn CollisionRules) -> Result<CollisionMesh, String> {
    let Cooked::Static(model) = cooked else {
        return Ok(CollisionMesh::default());
    };
    let mut builder = CollisionBuilder::new(settings.weld);
    let Some(lod) = model.lods.get(usize::from(settings.lod).min(model.lods.len().saturating_sub(1))) else {
        return Ok(CollisionMesh::default());
    };
    for sub in &lod.submeshes {
        let (vertices, indices) = sub.decode().map_err(|e| e.to_string())?;
        let positions: Vec<[f32; 3]> = vertices.iter().map(|v| v.position).collect();
        let surface = model.materials.get(sub.material as usize).map_or(SurfaceId::DEFAULT, |m| rules.surface(m));
        builder.add(&positions, &indices, Affine3A::IDENTITY, surface);
    }
    Ok(builder.finish())
}

impl Step for CollisionCookStep {
    fn id(&self) -> &'static str {
        COLLISION_COOK
    }
    fn version(&self) -> u32 {
        1
    }
    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        cx.product(MeshCookStep::ID, subject);
        self.rules.plan(subject, cx);
        cx.param("settings", postcard::to_stdvec(&self.settings).expect("in-memory serialization"));
        cx.param("rules", self.rules.version().to_le_bytes().to_vec());
        cx.param("format", COLLISION_FORMAT.to_le_bytes().to_vec());
        Ok(())
    }
    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let key = ProductKey::new(COLLISION_COOK, subject);
        if let Some(mesh) = self.rules.authored(subject, cx)? {
            return Ok(mesh.to_bytes());
        }
        let bytes = cx.product(MeshCookStep::ID, subject)?;
        let cooked = Cooked::from_bytes(&bytes).map_err(|e| BakeError::step(&key, e))?;
        let mesh = from_cooked(&cooked, &self.settings, self.rules.as_ref()).map_err(|e| BakeError::step(&key, e))?;
        Ok(mesh.to_bytes())
    }
    fn override_path(&self, subject: &str) -> Option<String> {
        Some(format!("collision/{subject}.sfcol"))
    }
}
