//! Bake steps: `model.info`, `mesh.cook`, `mesh.rest` and `mesh.rig`. Subjects are model paths.

use std::sync::Arc;

use sf_bake::{BakeError, PlanCx, ProductKey, RunCx, Step};

use crate::cooked::KEY_FORMAT;
use crate::{CookSettings, Model, ModelImporter, ModelInfo, cook, cook_rest, cook_rig};

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
        Ok(())
    }
    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let model = self.importers.import(Self::ID, subject, cx)?;
        let cooked = cook(&model, &self.settings).map_err(|e| BakeError::step(&ProductKey::new(Self::ID, subject), e))?;
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
        Ok(())
    }
    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let model = self.importers.import(Self::ID, subject, cx)?;
        let cooked = cook_rest(&model, &self.settings).map_err(|e| BakeError::step(&ProductKey::new(Self::ID, subject), e))?;
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
        Ok(())
    }
    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let model = self.importers.import(Self::ID, subject, cx)?;
        let cooked = cook_rig(&model, &self.settings).map_err(|e| BakeError::step(&ProductKey::new(Self::ID, subject), e))?;
        Ok(cooked.to_bytes())
    }
}
