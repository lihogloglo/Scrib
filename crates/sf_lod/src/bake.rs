//! The far field as `sf_bake` steps (feature `bake`).
//!
//! | Step | Subject | Reads | Output |
//! |---|---|---|---|
//! | `lod.policy` | model path | `mesh.cook`, else `mesh.rest` | `(FarPolicy, ModelSummary)` |
//! | `lod.impostor` | model path | `mesh.cook`, else `mesh.rest` | [`ImpostorAtlas`] |
//! | `lod.l1` | `<ws>/<x>,<y>,<z>` (cell) | `world.page`; *dynamic*: `lod.policy`, `mesh.cook` | [`NodeBlob`] |
//! | `lod.l2` | `<ws>/<x>,<y>,<z>` (node) | `world.dir`; *dynamic*: child `lod.l1` | [`NodeBlob`] |
//! | `lod.l3` | `<ws>/<x>,<y>,<z>` (node) | `world.dir`; *dynamic*: child `lod.l2` | [`NodeBlob`] |
//! | `lod.dir` | `<ws>` | `world.dir`; *dynamic*: every node | [`FarDirectory`] |
//! | `lod.near` | `<ws>/<x>,<y>,<z>` (cell) | `world.page`; *dynamic*: `mesh.cook` | [`NearCombine`] (precombine.md) |
//!
//! Every key includes [`FarBakeSettings::key_bytes`] and the adapter's
//! [`FarSource::fingerprint`]. [`FarBake::bake_worldspaces`] bakes level by
//! level, each level's nodes in parallel.

use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Instant;

use glam::{IVec3, Vec2, Vec3, Vec4};
use sf_bake::{BakeError, BakeReport, Baker, PlanCx, ProductKey, RunCx, Step};
use sf_model::steps::{MeshCookStep, MeshRestStep};
use sf_model::{AlphaMode, Cooked, CookedModel};
use sf_vfs::VfsPath;
use sf_world::bake::{DIR_STEP, PAGE_STEP, page_subject, worldspace_subject};
use sf_world::{CellDirectory, IndexPage, Partition, WorldspaceId};

use crate::blob::{FarDirectory, ImpostorModel, NodeBlob, NodeSummary};
use crate::build::{FarBakeSettings, FarModel, build_l1, build_parent, children_of, impostor_sphere, nodes_of};
use crate::impostor::{CaptureMesh, capture};
use crate::node::NodeKey;
use crate::policy::{FarPolicy, FarSource, ModelSummary, default_policy};
use crate::texture::{TextureDecoder, TextureImage};
pub use crate::products::{DIR_STEP as DIR_STEP_LOD, IMPOSTOR_STEP, L1_STEP, L2_STEP, L3_STEP, NEAR_STEP, POLICY_STEP, level_step, node_subject};
use crate::near::{NearCombine, build_near};
use crate::products::parse_node;

/// The product key of a node.
pub fn node_key(ws: &WorldspaceId, key: NodeKey) -> ProductKey {
    ProductKey::new(level_step(key.level), node_subject(ws, key))
}

fn step_err(step: &str, subject: &str, e: impl std::fmt::Display) -> BakeError {
    BakeError::step(&ProductKey::new(step, subject), e)
}

/// Shared state of the `lod.*` steps.
pub struct FarBake {
    source: Arc<dyn FarSource>,
    settings: FarBakeSettings,
    textures: Option<Arc<dyn TextureDecoder>>,
}

impl FarBake {
    /// Without texture decoding: proxies use base and vertex colors.
    pub fn new(source: Arc<dyn FarSource>, settings: FarBakeSettings) -> Arc<Self> {
        Arc::new(Self { source, settings, textures: None })
    }

    /// With the adapter's texture decoder: proxy albedo includes texture
    /// averages and impostor capture samples textures (alpha-tested leaves).
    pub fn with_textures(source: Arc<dyn FarSource>, settings: FarBakeSettings, textures: Arc<dyn TextureDecoder>) -> Arc<Self> {
        Arc::new(Self { source, settings, textures: Some(textures) })
    }

    /// A material's texture, decoded (the file read is a tracked input).
    fn texture(&self, cx: &mut RunCx<'_>, path: &str) -> Result<Option<TextureImage>, BakeError> {
        let Some(decoder) = &self.textures else { return Ok(None) };
        for candidate in decoder.candidates(path) {
            if cx.file_exists(&candidate)? {
                let bytes = cx.read_file(&candidate)?;
                return Ok(decoder.decode(&candidate, &bytes));
            }
        }
        Ok(None)
    }

    /// Each material's texture (`None` without one or without a decoder).
    fn material_textures(&self, cx: &mut RunCx<'_>, model: &CookedModel) -> Result<Vec<Option<TextureImage>>, BakeError> {
        let mut out = Vec::with_capacity(model.materials.len());
        for m in &model.materials {
            out.push(match &m.base_color_texture {
                Some(path) => self.texture(cx, path)?,
                None => None,
            });
        }
        Ok(out)
    }

    pub fn settings(&self) -> &FarBakeSettings {
        &self.settings
    }

    /// Registers the seven steps (the baker also needs `mesh.cook` and the
    /// `world.*` steps).
    pub fn register(self: &Arc<Self>, baker: &mut Baker) {
        baker
            .register(PolicyStep(self.clone()))
            .register(ImpostorStep(self.clone()))
            .register(NodeStep { far: self.clone(), level: 1 })
            .register(NodeStep { far: self.clone(), level: 2 })
            .register(NodeStep { far: self.clone(), level: 3 })
            .register(DirStep(self.clone()))
            .register(NearStep(self.clone()));
    }

    /// The far directory's product key.
    pub fn directory_key(ws: &WorldspaceId) -> ProductKey {
        ProductKey::new(DIR_STEP_LOD, worldspace_subject(ws))
    }

    /// Bakes whole worldspaces: every L1 node in parallel, then L2, then
    /// L3, then the directory and the impostor atlases it names. The world
    /// index (`world.dir`) must be bakeable on the same baker.
    pub fn bake_worldspaces(baker: &Baker, worldspaces: &[WorldspaceId]) -> BakeReport {
        let started = Instant::now();
        let mut products = Vec::new();
        let mut cells: BTreeMap<WorldspaceId, Vec<IVec3>> = BTreeMap::new();
        for ws in worldspaces {
            let key = ProductKey::new(DIR_STEP, worldspace_subject(ws));
            match baker.product(&key).and_then(|d| Ok(baker.store().get(&d)?)) {
                Ok(bytes) => {
                    if let Ok(dir) = CellDirectory::from_bytes(&bytes) {
                        cells.insert(ws.clone(), dir.cells.iter().map(|c| c.cell).collect());
                    }
                }
                Err(e) => products.push((key, Err(Arc::new(e)))),
            }
        }
        for level in 1..=3 {
            let keys: Vec<ProductKey> =
                cells.iter().flat_map(|(ws, c)| nodes_of(level, c).into_iter().map(move |k| node_key(ws, k))).collect();
            products.extend(baker.bake(&keys).products);
        }
        // Near precombines, one per cell (precombine.md).
        let near: Vec<ProductKey> = cells
            .iter()
            .flat_map(|(ws, c)| nodes_of(1, c).into_iter().map(move |k| ProductKey::new(NEAR_STEP, node_subject(ws, k))))
            .collect();
        products.extend(baker.bake(&near).products);
        let dirs: Vec<ProductKey> = worldspaces.iter().map(Self::directory_key).collect();
        let report = baker.bake(&dirs);
        let mut atlases = Vec::new();
        for (_, digest) in &report.products {
            let Ok(digest) = digest else { continue };
            let Ok(bytes) = baker.store().get(digest) else { continue };
            if let Ok(dir) = FarDirectory::from_bytes(&bytes) {
                atlases.extend(dir.impostor_models.iter().map(|m| ProductKey::new(IMPOSTOR_STEP, m.model.as_str())));
            }
        }
        products.extend(report.products);
        atlases.sort();
        atlases.dedup();
        products.extend(baker.bake(&atlases).products);
        BakeReport { products, steps: baker.step_stats(), elapsed: started.elapsed() }
    }

    fn plan_common(&self, cx: &mut PlanCx) {
        cx.param("far", self.settings.key_bytes()).param("source", self.source.fingerprint().to_vec());
        let textures = self.textures.as_ref().map_or(0, |t| t.version() + 1);
        cx.param("textures", textures.to_le_bytes().to_vec());
    }
}

/// A `lod.policy` product.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PolicyProduct {
    pub policy: FarPolicy,
    pub summary: ModelSummary,
    /// Per material: linear albedo (base color × texture average over its
    /// opaque texels) and texture coverage in the last channel.
    pub albedo: Vec<[f32; 4]>,
}

fn cutoff(alpha: AlphaMode) -> f32 {
    match alpha {
        AlphaMode::Mask(c) => c,
        AlphaMode::Blend => 0.5,
        AlphaMode::Opaque => 0.0,
    }
}

/// A model's cooked form for far use: `mesh.cook`, or, when that skipped
/// the model because something animates, its rest pose (`mesh.rest`,
/// weather.md bug B2). `None` when neither is static (skinned).
fn cooked(cx: &mut RunCx<'_>, model: &str) -> Result<Option<CookedModel>, BakeError> {
    let bytes = cx.product(MeshCookStep::ID, model)?;
    match Cooked::from_bytes(&bytes).map_err(|e| step_err(MeshCookStep::ID, model, e))? {
        Cooked::Static(m) => Ok(Some(m)),
        // Deforming (skinned or morphed): only a rest pose could join the far field.
        Cooked::Skipped { .. } | Cooked::Skinned(_) => {
            let bytes = cx.product(MeshRestStep::ID, model)?;
            match Cooked::from_bytes(&bytes).map_err(|e| step_err(MeshRestStep::ID, model, e))? {
                Cooked::Static(m) => Ok(Some(m)),
                Cooked::Skipped { .. } | Cooked::Skinned(_) => Ok(None),
            }
        }
    }
}

/// Decodes a `lod.policy` product.
pub fn decode_policy(bytes: &[u8]) -> Result<PolicyProduct, postcard::Error> {
    postcard::from_bytes(bytes)
}

// -- lod.policy ---------------------------------------------------------------

pub struct PolicyStep(pub Arc<FarBake>);

impl Step for PolicyStep {
    fn id(&self) -> &'static str {
        POLICY_STEP
    }
    fn version(&self) -> u32 {
        3
    }
    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        cx.product(MeshCookStep::ID, subject);
        self.0.plan_common(cx);
        Ok(())
    }
    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let product = match cooked(cx, subject)? {
            Some(model) => {
                let summary = ModelSummary::of(&model);
                let policy = self.0.source.policy(subject, &summary).unwrap_or_else(|| default_policy(&summary));
                let textures = self.0.material_textures(cx, &model)?;
                let albedo = model
                    .materials
                    .iter()
                    .zip(&textures)
                    .map(|(m, t)| {
                        let base = Vec4::from_array(m.base_color);
                        let tex = t.as_ref().map_or(Vec4::ONE, |t| t.average(cutoff(m.alpha).max(0.01)));
                        (base * Vec4::new(tex.x, tex.y, tex.z, 1.0)).truncate().extend(tex.w).to_array()
                    })
                    .collect();
                PolicyProduct { policy, summary, albedo }
            }
            // Neither cooked nor rest-cookable (skinned): not in the far field.
            None => PolicyProduct { policy: FarPolicy::Drop, summary: ModelSummary::default(), albedo: Vec::new() },
        };
        postcard::to_stdvec(&product).map_err(|e| step_err(POLICY_STEP, subject, e))
    }
}

// -- lod.impostor -------------------------------------------------------------

pub struct ImpostorStep(pub Arc<FarBake>);

/// The capture input of a cooked model's LOD0: vertex color × base color,
/// alpha-tested parts tested against their cutoff.
pub fn capture_mesh(model: &CookedModel) -> Result<(CaptureMesh, Vec<f32>), crate::FormatError> {
    let mut mesh = CaptureMesh::default();
    let mut cutoffs = Vec::new();
    let Some(lod) = model.lods.first() else { return Ok((mesh, cutoffs)) };
    for sub in &lod.submeshes {
        let material = model.materials.get(sub.material as usize).cloned().unwrap_or_default();
        let (v, i) = sub.decode().map_err(|e| crate::FormatError::Mesh(e.to_string()))?;
        let base = u32::try_from(mesh.positions.len()).map_err(|_| crate::FormatError::TooLarge)?;
        let m = u32::try_from(cutoffs.len()).map_err(|_| crate::FormatError::TooLarge)?;
        cutoffs.push(cutoff(material.alpha));
        let tint = Vec4::from_array(material.base_color);
        for vert in &v {
            mesh.positions.push(Vec3::from_array(vert.position));
            mesh.normals.push(Vec3::from_array(vert.normal));
            mesh.uvs.push(Vec2::from_array(vert.uv));
            mesh.colors.push(Vec4::from_array(vert.color.map(|c| f32::from(c) / 255.0)) * tint);
        }
        mesh.indices.extend(i.iter().map(|k| k + base));
        mesh.triangle_material.extend(std::iter::repeat_n(m, i.len() / 3));
    }
    Ok((mesh, cutoffs))
}

impl Step for ImpostorStep {
    fn id(&self) -> &'static str {
        IMPOSTOR_STEP
    }
    fn version(&self) -> u32 {
        3
    }
    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        cx.product(MeshCookStep::ID, subject);
        self.0.plan_common(cx);
        Ok(())
    }
    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let model = cooked(cx, subject)?.ok_or_else(|| step_err(IMPOSTOR_STEP, subject, "model is not cooked"))?;
        let (mesh, cutoffs) = capture_mesh(&model).map_err(|e| step_err(IMPOSTOR_STEP, subject, e))?;
        let textures = self.0.material_textures(cx, &model)?;
        let (center, radius) = impostor_sphere(&ModelSummary::of(&model));
        let shade = |m: u32, uv: Vec2, c: Vec4| {
            let tex = textures.get(m as usize).and_then(Option::as_ref).map_or(Vec4::ONE, |t| t.sample(uv));
            let c = c * tex;
            (c.w >= cutoffs.get(m as usize).copied().unwrap_or(0.0)).then_some(c)
        };
        Ok(capture(&mesh, center, radius, &self.0.settings.impostor, &shade).to_bytes())
    }
}

// -- lod.l1 / l2 / l3 -----------------------------------------------------------

pub struct NodeStep {
    pub far: Arc<FarBake>,
    pub level: u8,
}

impl NodeStep {
    fn run_l1(&self, subject: &str, ws: &WorldspaceId, key: NodeKey, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let cell = key.index();
        let page = IndexPage::from_bytes(&cx.product(PAGE_STEP, &page_subject(ws, Partition::page_of(cell)))?)
            .map_err(|e| step_err(L1_STEP, subject, e))?;
        let placements = page.cell(cell).map(|c| c.placements.clone()).unwrap_or_default();
        let mut models: BTreeMap<VfsPath, FarModel> = BTreeMap::new();
        for p in placements.iter().filter(|p| crate::far_eligible(&p.placement)) {
            let Some(path) = &p.placement.model else { continue };
            if models.contains_key(path) {
                continue;
            }
            let Ok(bytes) = cx.product(POLICY_STEP, path.as_str()) else { continue };
            let PolicyProduct { policy, summary, albedo } = decode_policy(&bytes).map_err(|e| step_err(L1_STEP, subject, e))?;
            let cooked = if policy == FarPolicy::Merge { cooked(cx, path.as_str())?.map(Arc::new) } else { None };
            models.insert(path.clone(), FarModel { policy, summary, cooked, albedo });
        }
        let blob = build_l1(page.partition, cell, &placements, &|m: &VfsPath| models.get(m).cloned(), &*self.far.source, &self.far.settings)
            .map_err(|e| step_err(L1_STEP, subject, e))?;
        Ok(blob.to_bytes())
    }

    fn run_parent(&self, subject: &str, ws: &WorldspaceId, key: NodeKey, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let step = level_step(key.level);
        let dir = CellDirectory::from_bytes(&cx.product(DIR_STEP, &worldspace_subject(ws))?).map_err(|e| step_err(step, subject, e))?;
        let cells: Vec<IVec3> = dir.cells.iter().map(|c| c.cell).filter(|c| key.contains_cell(*c)).collect();
        let mut children = Vec::new();
        for child in children_of(key, &cells) {
            let blob = NodeBlob::from_bytes(&cx.product(level_step(child.level), &node_subject(ws, child))?)
                .map_err(|e| step_err(step, subject, e))?;
            if !blob.header.is_empty() {
                children.push(blob);
            }
        }
        let blob = build_parent(key, dir.partition, children, &self.far.settings).map_err(|e| step_err(step, subject, e))?;
        Ok(blob.to_bytes())
    }
}

impl Step for NodeStep {
    fn id(&self) -> &'static str {
        level_step(self.level)
    }
    fn version(&self) -> u32 {
        4
    }
    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        let (ws, key) = parse_node(self.level, subject).ok_or_else(|| step_err(self.id(), subject, "malformed subject"))?;
        if self.level == 1 {
            cx.product(PAGE_STEP, page_subject(&ws, Partition::page_of(key.index())));
        } else {
            cx.product(DIR_STEP, worldspace_subject(&ws));
        }
        self.far.plan_common(cx);
        Ok(())
    }
    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let (ws, key) = parse_node(self.level, subject).ok_or_else(|| step_err(self.id(), subject, "malformed subject"))?;
        if self.level == 1 { self.run_l1(subject, &ws, key, cx) } else { self.run_parent(subject, &ws, key, cx) }
    }
}

// -- lod.near -------------------------------------------------------------------

/// A cell's near precombine. An empty [`NearCombine`] (no buckets) when
/// nothing in the cell qualifies.
pub struct NearStep(pub Arc<FarBake>);

impl Step for NearStep {
    fn id(&self) -> &'static str {
        NEAR_STEP
    }
    fn version(&self) -> u32 {
        1
    }
    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        let (ws, key) = parse_node(1, subject).ok_or_else(|| step_err(NEAR_STEP, subject, "malformed subject"))?;
        cx.product(PAGE_STEP, page_subject(&ws, Partition::page_of(key.index())));
        cx.param("source", self.0.source.fingerprint().to_vec());
        Ok(())
    }
    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let (ws, key) = parse_node(1, subject).ok_or_else(|| step_err(NEAR_STEP, subject, "malformed subject"))?;
        let cell = key.index();
        let page = IndexPage::from_bytes(&cx.product(PAGE_STEP, &page_subject(&ws, Partition::page_of(cell)))?)
            .map_err(|e| step_err(NEAR_STEP, subject, e))?;
        let placements = page.cell(cell).map(|c| c.placements.clone()).unwrap_or_default();
        // Only `mesh.cook` statics (no rest poses: their near copies animate).
        let mut models: BTreeMap<VfsPath, Option<Arc<CookedModel>>> = BTreeMap::new();
        for p in placements.iter().filter(|p| p.placement.class == sf_world::PlacementClass::Static) {
            let Some(path) = &p.placement.model else { continue };
            if models.contains_key(path) {
                continue;
            }
            let bytes = cx.product(MeshCookStep::ID, path.as_str())?;
            let model = match Cooked::from_bytes(&bytes).map_err(|e| step_err(MeshCookStep::ID, path.as_str(), e))? {
                Cooked::Static(m) => Some(Arc::new(m)),
                Cooked::Skipped { .. } | Cooked::Skinned(_) => None,
            };
            models.insert(path.clone(), model);
        }
        let source = &self.0.source;
        let near = build_near(page.partition, cell, &placements, &|m: &VfsPath| models.get(m).cloned().flatten(), &|p| {
            source.precombine_exclude(p)
        })
        .map_err(|e| step_err(NEAR_STEP, subject, e))?;
        Ok(near.unwrap_or(NearCombine { center: [0.0; 3], buckets: Vec::new() }).to_bytes())
    }
}

// -- lod.dir --------------------------------------------------------------------

pub struct DirStep(pub Arc<FarBake>);

impl Step for DirStep {
    fn id(&self) -> &'static str {
        DIR_STEP_LOD
    }
    fn version(&self) -> u32 {
        1
    }
    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        cx.product(DIR_STEP, subject);
        self.0.plan_common(cx);
        Ok(())
    }
    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let ws = WorldspaceId::new(subject);
        let dir = CellDirectory::from_bytes(&cx.product(DIR_STEP, subject)?).map_err(|e| step_err(DIR_STEP_LOD, subject, e))?;
        let cells: Vec<IVec3> = dir.cells.iter().map(|c| c.cell).collect();
        let mut nodes = Vec::new();
        let mut models: BTreeMap<String, f32> = BTreeMap::new();
        for level in 1..=3 {
            for key in nodes_of(level, &cells) {
                let bytes = cx.product(level_step(level), &node_subject(&ws, key))?;
                let header = NodeBlob::header(&bytes).map_err(|e| step_err(DIR_STEP_LOD, subject, e))?;
                if header.is_empty() {
                    continue;
                }
                for m in &header.impostor_models {
                    models.insert(m.model.clone(), m.radius);
                }
                nodes.push(NodeSummary { header, bytes: bytes.len() as u64 });
            }
        }
        nodes.sort_by_key(|n| n.header.key);
        let l = &self.0.settings.levels;
        let far = FarDirectory {
            worldspace: ws,
            partition: dir.partition,
            handoff: l.handoff,
            band: l.band,
            impostor: self.0.settings.impostor,
            nodes,
            impostor_models: models.into_iter().map(|(model, radius)| ImpostorModel { model, radius }).collect(),
        };
        Ok(far.to_bytes())
    }
}
