//! The `texture.demand` bake step (feature `bake`; textures.md §6, slice S3b).
//!
//! | Step | Subject | Reads | Output |
//! |---|---|---|---|
//! | `mesh.uv_density` | model path | `mesh.cook` (static) | `sf_model::uv_density::UvDensity` |
//! | `texture.demand` | `<ws>/<x>,<y>,<z>` (cell, as `scene.cells`) | `world.page`; *dynamic*: `mesh.cook`, `mesh.uv_density` | [`CellDemand`] bytes |
//!
//! **Placements.** Every placement of the cell's index page with a model, except actors and
//! markers (their spawner sets the dynamic `TextureDemand`, S4), editor-only and starts-disabled
//! ones; dynamic ones count at their start position. Models the cook skipped (no cooked
//! materials) are left out and counted.
//!
//! **Textures of a model.** Per material, the adapter's [`DemandSource::textures`] (default: the
//! base colour texture; a game adds its companions, which share the base's UVs), each with that
//! material's ρ from `mesh.uv_density`. A texture on several materials of one model keeps its
//! densest ρ.
//!
//! **Per placement.** Sphere: the model bounds' circumsphere, placed by the transform, its radius
//! grown by the largest axis scale. ρ: the model's divided by the smallest axis scale (the densest
//! direction; `sf_model::uv_density` "Scale").
//!
//! Keyed by the cell's index page, the adapter's fingerprint, the product format and the
//! clustering rule; the cooked models and their densities are dynamic inputs, so a changed model
//! re-runs its own `mesh.uv_density` and the cells that place it, nothing else.

use std::collections::BTreeMap;
use std::fmt;
use std::sync::Arc;
use std::time::{Duration, Instant};

use glam::{Affine3A, Vec3};
use sf_bake::{BakeError, Baker, PlanCx, ProductKey, RunCx, Step};
use sf_coords::WorldPos;
use sf_model::steps::MeshCookStep;
use sf_model::uv_density::{MeshUvDensityStep, UvDensity};
use sf_model::{Cooked, Material};
use sf_world::bake::{DIR_STEP, PAGE_STEP, page_subject, worldspace_subject};
use sf_world::{CellDirectory, IndexPage, Partition, Placement, PlacementClass, PlacementFlags, WorldspaceId};

use super::{CLUSTER_RULE, CellDemand, DEMAND_FORMAT, DEMAND_STEP, DemandCluster, DemandTexture, MAX_CLUSTER_RADIUS, cluster, merge_textures, texture_id};
use crate::bake::{cell_subject, parse_cell_subject};
use crate::table::Sphere;

/// Placements whose own switches keep them out of the demand.
pub const SKIPPED_FLAGS: PlacementFlags = PlacementFlags::EDITOR_ONLY.union(PlacementFlags::DISABLED);

/// The adapter's side: which textures a cooked material draws.
pub trait DemandSource: Send + Sync {
    /// Changes whenever [`Self::textures`] would answer differently (rule version, the companion
    /// tables it reads).
    fn fingerprint(&self) -> [u8; 32];
    /// Pushes the VFS paths of every texture `material` binds that shares its UV channel 0 (base
    /// colour and its companions). Default: the base colour texture.
    fn textures(&self, material: &Material, out: &mut Vec<String>) {
        out.extend(material.base_color_texture.clone());
    }
}

/// [`DemandSource`] with the default answer (base colour textures only).
pub struct BaseTextures;

impl DemandSource for BaseTextures {
    fn fingerprint(&self) -> [u8; 32] {
        *b"sf_scene.demand.base_textures.v1"
    }
}

/// One model's demand, model space: its bounding sphere and `(texture, ρ)` uses.
#[derive(Clone, Debug, PartialEq)]
pub struct ModelDemand {
    pub sphere: Sphere,
    pub textures: Vec<DemandTexture>,
}

/// One model's [`ModelDemand`] from its cook and density (`None` for a skipped cook).
pub fn model_demand(source: &dyn DemandSource, cooked: &Cooked, density: &UvDensity) -> Option<ModelDemand> {
    let model = match cooked {
        Cooked::Static(m) => m,
        Cooked::Skinned(s) => &s.model,
        Cooked::Skipped { .. } => return None,
    };
    let (lo, hi) = (Vec3::from_array(model.bounds.min), Vec3::from_array(model.bounds.max));
    let sphere = Sphere { center: (lo + hi) * 0.5, radius: (hi - lo).length() * 0.5 };
    let mut paths = Vec::new();
    let mut uses = Vec::new();
    for (k, m) in model.materials.iter().enumerate() {
        paths.clear();
        source.textures(m, &mut paths);
        let rho = density.materials.get(k).copied().flatten();
        uses.extend(paths.iter().map(|p| DemandTexture { id: texture_id(p), rho }));
    }
    Some(ModelDemand { sphere, textures: merge_textures(uses) })
}

/// Whether a placement enters the demand (module docs, "Placements").
pub fn wants_demand(p: &Placement) -> bool {
    p.model.is_some() && !matches!(p.class, PlacementClass::Actor | PlacementClass::Marker) && !p.flags.intersects(SKIPPED_FLAGS)
}

/// The step's pure core: a cell's demand from its placements (already filtered by
/// [`wants_demand`]) and each one's model demand (same length; `None` = no cooked model).
pub fn build_cell_demand(origin_cell: glam::IVec3, placements: &[Placement], models: &[Option<&ModelDemand>]) -> CellDemand {
    assert_eq!(placements.len(), models.len(), "one model slot per placement");
    let origin = WorldPos::new(origin_cell, Vec3::ZERO);
    let mut spheres = Vec::new();
    let mut uses: Vec<(&ModelDemand, f32)> = Vec::new();
    for (p, m) in placements.iter().zip(models) {
        let Some(m) = m else { continue };
        let t = &p.transform;
        let scale = t.scale.abs();
        let (min_scale, max_scale) = (scale.min_element(), scale.max_element());
        if min_scale.is_nan() || min_scale <= 1e-6 || !max_scale.is_finite() {
            continue;
        }
        let affine = Affine3A::from_scale_rotation_translation(t.scale, t.rotation, origin.delta_f32(t.pos));
        spheres.push(Sphere { center: affine.transform_point3(m.sphere.center), radius: m.sphere.radius * max_scale });
        uses.push((m, min_scale));
    }
    let clusters = cluster(&spheres)
        .into_iter()
        .map(|(sphere, members)| {
            let textures = merge_textures(members.iter().flat_map(|&i| {
                let (m, s) = uses[i];
                m.textures.iter().map(move |t| DemandTexture { id: t.id, rho: t.rho.map(|r| r / s) })
            }));
            DemandCluster { sphere, textures }
        })
        .collect();
    CellDemand { origin_cell, clusters }
}

/// What [`DemandBake::bake_worldspaces`] made (the bake's stats line is its `Display`).
#[derive(Debug, Default)]
pub struct DemandReport {
    pub cells: usize,
    pub clusters: usize,
    /// Clusters of one placement larger than [`MAX_CLUSTER_RADIUS`].
    pub oversized: usize,
    /// (cluster, texture) entries, and of them without a UV density.
    pub entries: usize,
    pub no_density: usize,
    /// Distinct texture ids over every cell.
    pub textures: usize,
    pub failures: Vec<(ProductKey, String)>,
    pub elapsed: Duration,
}

impl fmt::Display for DemandReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[allow(clippy::cast_precision_loss)] // a ratio for a log line
        let mean = if self.clusters == 0 { 0.0 } else { self.entries as f64 / self.clusters as f64 };
        writeln!(
            f,
            "texture.demand: {} cells, {} clusters ({} single placements over {MAX_CLUSTER_RADIUS} m), {mean:.1} textures per cluster, {} distinct textures, {} entries without UV density, in {:.2?}",
            self.cells, self.clusters, self.oversized, self.textures, self.no_density, self.elapsed
        )
    }
}

/// Shared state of the step.
pub struct DemandBake {
    source: Arc<dyn DemandSource>,
}

impl DemandBake {
    pub fn new(source: Arc<dyn DemandSource>) -> Self {
        Self { source }
    }

    /// Registers `mesh.uv_density` and `texture.demand` (the baker also needs `mesh.cook` and
    /// `world.*`).
    pub fn register(self: &Arc<Self>, baker: &mut Baker) {
        baker.register(MeshUvDensityStep);
        baker.register(DemandStep(self.clone()));
    }

    /// Every cell's product key, from the worldspaces' `world.dir`.
    pub fn cell_keys(baker: &Baker, worldspaces: &[WorldspaceId]) -> Result<Vec<ProductKey>, BakeError> {
        let mut keys = Vec::new();
        for ws in worldspaces {
            let digest = baker.product(&ProductKey::new(DIR_STEP, worldspace_subject(ws)))?;
            let dir = CellDirectory::from_bytes(&baker.store().get(&digest)?).map_err(|e| BakeError::Other(e.to_string()))?;
            keys.extend(dir.cells.iter().map(|c| ProductKey::new(DEMAND_STEP, cell_subject(ws, c.cell))));
        }
        Ok(keys)
    }

    /// Bakes every cell of `worldspaces` in parallel and sums the products into a report.
    pub fn bake_worldspaces(baker: &Baker, worldspaces: &[WorldspaceId]) -> DemandReport {
        let started = Instant::now();
        let mut report = DemandReport::default();
        let keys = match Self::cell_keys(baker, worldspaces) {
            Ok(keys) => keys,
            Err(e) => {
                report.failures.push((ProductKey::new(DIR_STEP, ""), e.to_string()));
                report.elapsed = started.elapsed();
                return report;
            }
        };
        let outcome = baker.bake(&keys);
        let mut ids = std::collections::BTreeSet::new();
        for (key, result) in &outcome.products {
            let demand = result
                .as_ref()
                .map_err(ToString::to_string)
                .and_then(|d| baker.store().get(d).map_err(|e| e.to_string()))
                .and_then(|b| CellDemand::read(&b).map_err(|e| e.to_string()));
            match demand {
                Ok(d) => {
                    report.cells += 1;
                    report.clusters += d.clusters.len();
                    for c in &d.clusters {
                        report.oversized += usize::from(c.sphere.radius > MAX_CLUSTER_RADIUS);
                        report.entries += c.textures.len();
                        report.no_density += c.textures.iter().filter(|t| t.rho.is_none()).count();
                        ids.extend(c.textures.iter().map(|t| t.id));
                    }
                }
                Err(e) => report.failures.push((key.clone(), e)),
            }
        }
        report.textures = ids.len();
        report.elapsed = started.elapsed();
        report
    }
}

fn step_err(subject: &str, e: impl fmt::Display) -> BakeError {
    BakeError::step(&ProductKey::new(DEMAND_STEP, subject), e)
}

/// `texture.demand`.
pub struct DemandStep(pub Arc<DemandBake>);

impl Step for DemandStep {
    fn id(&self) -> &'static str {
        DEMAND_STEP
    }
    fn version(&self) -> u32 {
        1
    }
    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        let (ws, cell) = parse_cell_subject(subject).ok_or_else(|| step_err(subject, "malformed subject"))?;
        cx.product(PAGE_STEP, page_subject(&ws, Partition::page_of(cell)));
        cx.param("source", self.0.source.fingerprint().to_vec());
        cx.param("format", DEMAND_FORMAT.to_le_bytes().to_vec());
        cx.param("cluster", CLUSTER_RULE.to_le_bytes().to_vec());
        Ok(())
    }
    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let (ws, cell) = parse_cell_subject(subject).ok_or_else(|| step_err(subject, "malformed subject"))?;
        let page = IndexPage::from_bytes(&cx.product(PAGE_STEP, &page_subject(&ws, Partition::page_of(cell)))?)
            .map_err(|e| step_err(subject, e))?;
        let placements: Vec<Placement> = page
            .cell(cell)
            .map(|c| c.placements.iter().map(|p| p.placement.clone()).filter(wants_demand).collect())
            .unwrap_or_default();
        // Each model once: its cook and density.
        let mut models: BTreeMap<&str, Option<ModelDemand>> = BTreeMap::new();
        for path in placements.iter().filter_map(|p| p.model.as_ref()) {
            let path = path.as_str();
            if models.contains_key(path) {
                continue;
            }
            let cook_err = |e| BakeError::step(&ProductKey::new(MeshCookStep::ID, path), e);
            let cooked = Cooked::from_bytes(&cx.product(MeshCookStep::ID, path)?).map_err(cook_err)?;
            let density = UvDensity::from_bytes(&cx.product(MeshUvDensityStep::ID, path)?)
                .map_err(|e| BakeError::step(&ProductKey::new(MeshUvDensityStep::ID, path), e))?;
            models.insert(path, model_demand(&*self.0.source, &cooked, &density));
        }
        let slots: Vec<Option<&ModelDemand>> =
            placements.iter().map(|p| p.model.as_ref().and_then(|m| models.get(m.as_str())).and_then(Option::as_ref)).collect();
        let origin_cell = page.partition.cell_origin(cell).cell;
        build_cell_demand(origin_cell, &placements, &slots).write().map_err(|e| step_err(subject, e))
    }
}
