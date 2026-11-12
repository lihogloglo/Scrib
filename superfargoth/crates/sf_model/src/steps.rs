//! Bake steps: `model.info`, `mesh.cook`, `mesh.rest` and `mesh.rig`. Subjects are model paths.

use std::sync::Arc;

use sf_bake::{BakeError, PlanCx, ProductKey, RunCx, Step};

use crate::cooked::KEY_FORMAT;
use crate::{CookSettings, Material, Model, ModelImporter, ModelInfo, cook_rest_with, cook_rig_with, cook_with};

/// The registered importers, chosen by file extension.
#[derive(Clone, Default)]
pub struct Importers(Vec<Arc<dyn ModelImporter>>);

impl Importers {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an importer; later ones win for the same extension.
    #[must_use]
    pub fn with(mut self, importer: Arc<dyn ModelImporter>) -> Self {
        self.0.push(importer);
        self
    }

    pub fn for_path(&self, path: &str) -> Option<&Arc<dyn ModelImporter>> {
        let ext = path.rsplit_once('.')?.1.to_ascii_lowercase();
        self.0.iter().rev().find(|i| i.extensions().contains(&ext.as_str()))
    }

    fn import(&self, step: &'static str, path: &str, cx: &mut RunCx<'_>) -> Result<Model, BakeError> {
        let key = || ProductKey::new(step, path);
        let importer = self.for_path(path).ok_or_else(|| BakeError::step(&key(), "no importer for this extension"))?;
        let bytes = cx.read_file(path)?;
        importer.import(path, &bytes).map_err(|e| BakeError::step(&key(), e))
    }

    fn plan(&self, path: &str, cx: &mut PlanCx) {
        cx.file(path);
        let version = self.for_path(path).map_or(0, |i| i.version());
        cx.param("importer", version.to_le_bytes().to_vec());
    }
}

/// Which materials of a model bind a normal map, so the mesh steps give their
/// submeshes `MikkTSpace` tangents. An adapter hook: the rule that pairs a base
/// texture with its normal map is the game's (a suffix convention, a material
/// record, ...), not the framework's. Asked once per material while a step
/// runs: check files through `cx` ([`RunCx::file_exists`]) so they become the
/// product's dependencies, and a pack that adds a normal map re-cooks only
/// the models whose materials it pairs with.
#[derive(Clone)]
pub struct TangentRule {
    /// Bump when the rule changes: it is part of the bake key.
    pub version: u32,
    pub wants: Arc<TangentFn>,
}

/// [`TangentRule::wants`]: true when `material` binds a normal map.
pub type TangentFn = dyn Fn(&Material, &mut RunCx<'_>) -> Result<bool, BakeError> + Send + Sync;

impl TangentRule {
    fn plan(rule: Option<&Self>, cx: &mut PlanCx) {
        cx.param("tangents", rule.map_or(0, |r| u64::from(r.version) + 1).to_le_bytes().to_vec());
    }

    /// Per material of `model`: whether it gets tangents (all false without a rule).
    fn materials(rule: Option<&Self>, model: &Model, cx: &mut RunCx<'_>) -> Result<Vec<bool>, BakeError> {
        let Some(rule) = rule else { return Ok(Vec::new()) };
        model.materials.iter().map(|m| (rule.wants)(m, cx)).collect()
    }
}

/// Bounds and counts of a model (postcard [`ModelInfo`]).
pub struct ModelInfoStep {
    pub importers: Importers,
}

impl ModelInfoStep {
    pub const ID: &'static str = "model.info";

    pub fn decode(bytes: &[u8]) -> Result<ModelInfo, postcard::Error> {
        postcard::from_bytes(bytes)
    }
}

impl Step for ModelInfoStep {
    fn id(&self) -> &'static str {
        Self::ID
    }
    fn version(&self) -> u32 {
        1
    }
    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        self.importers.plan(subject, cx);
        Ok(())
    }
    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let model = self.importers.import(Self::ID, subject, cx)?;
        postcard::to_stdvec(&ModelInfo::of(&model)).map_err(|e| BakeError::Other(e.to_string()))
    }
}

/// The cooked `.sfmesh` (a [`crate::Cooked`]). Mods can ship
/// `cooked/<model path>.sfmesh` to replace it.
pub struct MeshCookStep {
    pub importers: Importers,
    pub settings: CookSettings,
    /// Tangents for normal-mapped materials; `None`: no tangents.
    pub tangents: Option<TangentRule>,
}

impl MeshCookStep {
    pub const ID: &'static str = "mesh.cook";
}

impl Step for MeshCookStep {
    fn id(&self) -> &'static str {
        Self::ID
    }
    fn version(&self) -> u32 {
        3
    }
    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        self.importers.plan(subject, cx);
        cx.param("settings", self.settings.key_bytes()).param("format", KEY_FORMAT.to_le_bytes().to_vec());
        TangentRule::plan(self.tangents.as_ref(), cx);
        Ok(())
    }
    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let model = self.importers.import(Self::ID, subject, cx)?;
        let tangents = TangentRule::materials(self.tangents.as_ref(), &model, cx)?;
        let cooked = cook_with(&model, &self.settings, &tangents).map_err(|e| BakeError::step(&ProductKey::new(Self::ID, subject), e))?;
        Ok(cooked.to_bytes())
    }
    fn override_path(&self, subject: &str) -> Option<String> {
        Some(format!("cooked/{subject}.sfmesh"))
    }
}

/// The model frozen in its rest pose (a [`crate::Cooked`] from
/// [`cook_rest`]), for far-field use of models `mesh.cook` skips because
/// something animates (weather.md bug B2). Static models don't need it:
/// their `mesh.cook` product is the same thing. Request it only after
/// `mesh.cook` came back skipped; register it wherever [`MeshCookStep`] is.
pub struct MeshRestStep {
    pub importers: Importers,
    pub settings: CookSettings,
    /// Tangents for normal-mapped materials; `None`: no tangents.
    pub tangents: Option<TangentRule>,
}

impl MeshRestStep {
    pub const ID: &'static str = "mesh.rest";
}

impl Step for MeshRestStep {
    fn id(&self) -> &'static str {
        Self::ID
    }
    fn version(&self) -> u32 {
        1
    }
    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        self.importers.plan(subject, cx);
        cx.param("settings", self.settings.key_bytes()).param("format", KEY_FORMAT.to_le_bytes().to_vec());
        TangentRule::plan(self.tangents.as_ref(), cx);
        Ok(())
    }
    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let model = self.importers.import(Self::ID, subject, cx)?;
        let tangents = TangentRule::materials(self.tangents.as_ref(), &model, cx)?;
        let cooked = cook_rest_with(&model, &self.settings, &tangents).map_err(|e| BakeError::step(&ProductKey::new(Self::ID, subject), e))?;
        Ok(cooked.to_bytes())
    }
}

/// A character body cooked for a rig ([`cook_rig`], a [`crate::Cooked`]):
/// rigid meshes become single-joint parts of their nearest transform node,
/// so models animated per node (no skin) take the same skinned path as
/// skinned ones. Request it for the models an adapter uses as character
/// bodies; their `mesh.cook` product stays what the world path uses.
pub struct MeshRigStep {
    pub importers: Importers,
    pub settings: CookSettings,
    /// Tangents for normal-mapped materials; `None`: no tangents.
    pub tangents: Option<TangentRule>,
}

impl MeshRigStep {
    pub const ID: &'static str = "mesh.rig";
}

impl Step for MeshRigStep {
    fn id(&self) -> &'static str {
        Self::ID
    }
    fn version(&self) -> u32 {
        1
    }
    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        self.importers.plan(subject, cx);
        cx.param("settings", self.settings.key_bytes()).param("format", KEY_FORMAT.to_le_bytes().to_vec());
        TangentRule::plan(self.tangents.as_ref(), cx);
        Ok(())
    }
    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let model = self.importers.import(Self::ID, subject, cx)?;
        let tangents = TangentRule::materials(self.tangents.as_ref(), &model, cx)?;
        let cooked = cook_rig_with(&model, &self.settings, &tangents).map_err(|e| BakeError::step(&ProductKey::new(Self::ID, subject), e))?;
        Ok(cooked.to_bytes())
    }
}
