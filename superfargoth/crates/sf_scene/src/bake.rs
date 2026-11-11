//! The `scene.cells` and `scene.chains` bake steps (feature `bake`, static-scene.md §2): one
//! [`StaticCellTable`] per streaming cell, and one mesh per chain.
//!
//! | Step | Subject | Reads | Output |
//! |---|---|---|---|
//! | `scene.cells` | `<ws>/<x>,<y>,<z>` (cell) | `world.page`; *dynamic*: `mesh.cook`, level-0 `terrain.tile` | [`StaticCellTable`] bytes (`table.rs`, format v3) |
//! | `scene.chains` | `<model>/<part>` (chain) | `mesh.cook` (static) | an `sf_model` `.sfmesh`, one LOD level per chain level |
//!
//! Keyed like `lod.near`: the cell's index page (its placements, so its records), the adapter's
//! [`SceneSource::fingerprint`] (eligibility rules and the record facts they read, the mod set),
//! [`SceneSettings::key_bytes`], the table format version and the worldspace's land coverage; the
//! cooked models and terrain tiles it reads are dynamic inputs.
//!
//! **Placement ids.** A scene [`PlacementId`] is the placement's index in its world-index cell
//! (`IndexCell::placements`, a deterministic order). It is **cell-local**: with the table's cell
//! it names one placement of the worldspace, so promotion and picking carry (cell, placement);
//! the runtime reads the same index page to reach the record and physics. (The 128-bit
//! `sf_world::PlacementId` hash does not fit the u32 the GPU record carries without collisions.)
//!
//! **Candidates.** Only `sf_world` [`PlacementClass::Static`] placements with a model are read
//! (their `mesh.cook` product), and never one whose own switches bar it ([`BARRED`]:
//! editor-only, no-LOD, near-only, starts-disabled; the framework reads those, not the adapter).
//! The adapter's [`SceneEligibility`] (through [`SceneSource::eligibility`]) then decides per
//! placement with [`classify_placement`], seeing what the cooker reported for its model
//! ([`ModelFacts`]).
//!
//! **Chains.** One per (model, material part) ([`crate::chain`]), cell-local (`table.rs`):
//! instances name them by index, sorted by mesh key. The **mesh key** is a stable hash of the
//! cooked model's product key and the part's material index ([`mesh_key`]); the **material key**
//! a stable hash of the cooked material's contents ([`material_key`]), so models sharing a
//! material share it; the chain's [`MaterialClass`] comes from the adapter's reading of that
//! material. Since format v3 the chain also carries the **source** of its geometry: the index of
//! its model's `mesh.cook` subject in the table's model section, and the part (material index)
//! inside it — the two inputs of [`mesh_key`], which is a hash and cannot be inverted.
//!
//! **Chain geometry (`scene.chains`).** The concatenate-and-rebase of [`crate::chain`] is a bake
//! step, not stream-in work: at stream-in it would have to decode every level's submeshes with no
//! stage and no budget (scene-runtime-plan.md §3.2). [`build_chain_mesh`] builds one `.sfmesh`
//! per chain with **one LOD level per chain level, one submesh each** — that level's submeshes of
//! the chain's material, concatenated and rebased *within the level*, since each level is its own
//! mesh allocation (`CookedModelAsset::meshes[level]`) and the runtime draws it whole
//! (`first_index: 0, index_count: 0`). A level with no geometry of the part carries a copy of the
//! next finer level's, which is exactly what the table's level range for it says
//! ([`chain_layout`]). The product's single material is the chain's.
//!
//! A table's `LodLevel::first_index` therefore describes a concatenation of the *whole* chain
//! that nothing builds (a logged spec correction, plan §12): it stays in the format — the reader
//! validates against it and [`chain_layout`] produces it — but it is not what the runtime indexes
//! with. Do not "simplify" it away.
//!
//! **Chain subjects come from the baked tables.** [`SceneBake::chain_keys`] reads each baked
//! cell's model section: a v3 table already names the `mesh.cook` subject and material part of
//! every chain ([`chain_subjects`]), and `<model>/<part>` *is* a `scene.chains` subject. So the
//! step needs no new adapter method and no game-specific name, and chains dedup across cells.
//!
//! **Bounds.** Each instance carries its own part's sphere: the circumsphere of the part's LOD0
//! vertex bounds ([`part_spheres`], decoded once per model per cell), placed by the transform and
//! grown by its largest axis scale. Coarser levels need no margin: the cooker's simplifier only
//! drops vertices of the LOD0 vertex set, so every level lies within LOD0's bounds.
//!
//! **Land minimum.** The minimum of the level-0 terrain tiles' block bounds over the cell, minus
//! one quantization step, cell-relative ([`land_min`]); negative infinity when any part of the
//! cell has no terrain tile (no land) or the worldspace has no grid (interiors).

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;
use std::time::Instant;

use glam::{Affine3A, DVec2, IVec3, Vec3};
use sf_bake::{BakeError, BakeReport, Baker, Digest, PlanCx, ProductKey, RunCx, Step};
use sf_coords::{GRID_CELL, WorldPos};
use sf_model::steps::MeshCookStep;
use sf_model::{Aabb as ModelAabb, AlphaMode, Cooked, CookedLod, CookedModel, CookedSubmesh, Material, Vertex};
use sf_terrain::bake::{TILE_STEP, tile_subject};
use sf_terrain::{BASE_SPACING, BLOCK_QUADS, BLOCKS, BlockBounds, MAX_QUANT_STEP, TileCoord, TileProduct};
use sf_world::bake::{DIR_STEP, PAGE_STEP, page_subject, worldspace_subject};
use sf_world::{CellDirectory, IndexPage, Partition, Placement, PlacementClass, PlacementFlags, WorldspaceId};

use crate::chain::{LayoutError, LevelSource, chain_layout};
use crate::eligibility::{Eligibility, EntityReason, MaterialKind, SceneEligibility, classify};
use crate::material::{MaterialClass, MaterialTraits};
use crate::table::{
    Aabb, InstanceFlags, LodChain, Sphere, StaticCellTable, StaticCellTableView, StaticInstance, TableError,
};
use crate::PlacementId;

pub use crate::product::{CELLS_STEP, CHAINS_STEP};
/// File extension of `scene.cells` products in `baked://` asset paths
/// ([`crate::product::CELLS_EXT`]).
pub use crate::product::CELLS_EXT as EXTENSION;

/// Per-placement switches that bar a placement from the static scene whatever its record says
/// (module docs, "Candidates"): the precombine "Members" bars of `sf_lod::near::near_eligible`.
/// The framework applies them ([`classify_placement`]), so no adapter can forget one.
pub const BARRED: PlacementFlags = PlacementFlags::EDITOR_ONLY
    .union(PlacementFlags::NO_LOD)
    .union(PlacementFlags::NEAR_ONLY)
    .union(PlacementFlags::DISABLED);

/// [`classify`] with the framework's [`BARRED`] switches applied first: the §1 rule as the bake
/// runs it.
///
/// The switches are `sf_world` data the framework already holds, so it reads them itself instead
/// of asking the adapter: routing them through [`SceneEligibility`] is what let `NO_LOD`,
/// `NEAR_ONLY` and `DISABLED` fall through an adapter that answered only the record questions
/// (eligibility must be conservative, static-scene.md §1 and "Risks").
pub fn classify_placement(
    facts: &(impl SceneEligibility + ?Sized),
    id: PlacementId,
    placement: &Placement,
) -> Eligibility {
    if placement.flags.intersects(BARRED) {
        return Eligibility::Entity(EntityReason::Barred);
    }
    classify(facts, id)
}

/// Subject of a cell's table: `<ws>/<x>,<y>,<z>` ([`crate::product::cell_subject`] with the
/// bake's own worldspace type).
pub fn cell_subject(ws: &WorldspaceId, cell: IVec3) -> String {
    crate::product::cell_subject(ws.as_str(), cell)
}

/// Inverse of [`cell_subject`].
pub fn parse_cell_subject(subject: &str) -> Option<(WorldspaceId, IVec3)> {
    let (ws, cell) = crate::product::parse_cell_subject(subject)?;
    Some((WorldspaceId::new(ws), cell))
}

/// Subject of a chain's geometry: `<model>/<part>`, the model's [`MeshCookStep`] subject and the
/// chain's material index in it ([`crate::product::chain_subject`]).
pub fn chain_subject(model: &str, part: u32) -> String {
    crate::product::chain_subject(model, part)
}

/// Inverse of [`chain_subject`].
pub fn parse_chain_subject(subject: &str) -> Option<(&str, u32)> {
    crate::product::parse_chain_subject(subject)
}

/// The opaque mesh key of a chain: blake3 of the `mesh.cook` product key and the material index,
/// first 8 bytes little-endian. Stable across bakes and machines.
pub fn mesh_key(model: &str, material: u32) -> u64 {
    let mut bytes = Vec::with_capacity(MeshCookStep::ID.len() + model.len() + 6);
    bytes.extend_from_slice(MeshCookStep::ID.as_bytes());
    bytes.push(0);
    bytes.extend_from_slice(model.as_bytes());
    bytes.push(0);
    bytes.extend_from_slice(&material.to_le_bytes());
    let d = Digest::of(&bytes).0;
    u64::from_le_bytes(d[..8].try_into().expect("8 bytes"))
}

/// Version of [`material_key`]'s encoding (part of the hashed bytes).
const MATERIAL_KEY_VERSION: u8 = 1;

/// The opaque material key of a chain: blake3 of the cooked material's contents, first 8 bytes
/// little-endian. Stable across bakes and machines; equal for equal materials whichever model
/// they come from (§5: one bindless material slot per key).
///
/// Hashed: base colour texture path, base colour, emissive, alpha mode and cutoff, two-sided,
/// unlit and every extra (adapter hints may change the look). Not hashed: `name`, a label that
/// draws nothing. Floats by bits, with `-0.0` taken as `0.0`.
pub fn material_key(m: &Material) -> u64 {
    fn put_str(out: &mut Vec<u8>, s: &str) {
        out.extend_from_slice(&(s.len() as u64).to_le_bytes());
        out.extend_from_slice(s.as_bytes());
    }
    fn put_f32(out: &mut Vec<u8>, v: f32) {
        out.extend_from_slice(&(v + 0.0).to_bits().to_le_bytes());
    }
    let mut out = b"sf_scene.material\0".to_vec();
    out.push(MATERIAL_KEY_VERSION);
    match &m.base_color_texture {
        None => out.push(0),
        Some(path) => {
            out.push(1);
            put_str(&mut out, path);
        }
    }
    m.base_color.iter().chain(&m.emissive).for_each(|&v| put_f32(&mut out, v));
    match m.alpha {
        AlphaMode::Opaque => out.push(0),
        AlphaMode::Mask(cutoff) => {
            out.push(1);
            put_f32(&mut out, cutoff);
        }
        AlphaMode::Blend => out.push(2),
    }
    out.push(u8::from(m.double_sided));
    out.push(u8::from(m.unlit));
    out.extend_from_slice(&(m.extras.len() as u64).to_le_bytes());
    for (k, v) in &m.extras {
        put_str(&mut out, k);
        put_str(&mut out, v);
    }
    let d = Digest::of(&out).0;
    u64::from_le_bytes(d[..8].try_into().expect("8 bytes"))
}

/// Model-space bounding sphere of each material part's LOD0 geometry (the circumsphere of its
/// vertex AABB), indexed by material; `None` for a part without LOD0 vertices. Decodes LOD0.
pub fn part_spheres(model: &CookedModel) -> Result<Vec<Option<Sphere>>, sf_model::FormatError> {
    let mut boxes: Vec<Option<(Vec3, Vec3)>> = vec![None; model.materials.len()];
    for sub in model.lods.first().map_or(&[][..], |l| &l.submeshes[..]) {
        let Some(slot) = boxes.get_mut(sub.material as usize) else { continue };
        let (vertices, _) = sub.decode()?;
        for v in &vertices {
            let p = Vec3::from_array(v.position);
            *slot = Some(slot.map_or((p, p), |(lo, hi)| (lo.min(p), hi.max(p))));
        }
    }
    Ok(boxes
        .into_iter()
        .map(|b| b.map(|(lo, hi)| Sphere { center: (lo + hi) * 0.5, radius: (hi - lo).length() * 0.5 }))
        .collect())
}

/// How a source material enters the scene: its blend kind and, if it is a scene material, the
/// traits its class is made of.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SceneMaterial {
    pub kind: MaterialKind,
    pub traits: MaterialTraits,
}

/// The game-agnostic reading of an `sf_model` material: alpha mask → masked, blend → blended,
/// double-sided → two-sided, no wind. Adapters refine it ([`SceneSource::material`]).
pub fn default_material(m: &Material) -> SceneMaterial {
    let kind = match m.alpha {
        AlphaMode::Opaque => MaterialKind::Opaque,
        AlphaMode::Mask(_) => MaterialKind::AlphaMasked,
        AlphaMode::Blend => MaterialKind::Blended,
    };
    SceneMaterial { kind, traits: MaterialTraits { masked: kind == MaterialKind::AlphaMasked, two_sided: m.double_sided, wind: false } }
}

/// What the cooker reported for a placement's model.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ModelFacts {
    /// `mesh.cook` made a cooked model (`Cooked::Static` or `Cooked::Skinned`).
    pub cooked: bool,
    /// It deforms or moves at runtime (`Cooked::Skinned`; models with animated nodes are
    /// `Cooked::Skipped`, so not cooked).
    pub animated: bool,
    /// Blend kind per material of a static cook (the part order), from [`SceneSource::material`].
    pub parts: Vec<MaterialKind>,
}

/// One cell, as the step hands it to the adapter: placement `k` has [`PlacementId`] `k`.
#[derive(Clone, Copy, Debug)]
pub struct CellFacts<'a> {
    pub placements: &'a [Placement],
    /// Same length as `placements`.
    pub models: &'a [ModelFacts],
}

/// The adapter's side of the step: eligibility rules and material reading.
pub trait SceneSource: Send + Sync {
    /// Changes whenever any answer below would (rules version, the records they read).
    fn fingerprint(&self) -> [u8; 32];
    /// How a cooked material enters the scene. Default: [`default_material`].
    fn material(&self, material: &Material) -> SceneMaterial {
        default_material(material)
    }
    /// The eligibility facts of one cell's placements (indexed as in [`CellFacts`]).
    fn eligibility<'a>(&'a self, cell: CellFacts<'a>) -> Box<dyn SceneEligibility + 'a>;
}

/// Bake settings (part of every key).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SceneSettings {
    /// Layer mask of every table (worldspace layer; [`StaticCellTable::layers`]).
    pub layers: u32,
    /// Whether instances cast shadows.
    pub casts_shadow: bool,
}

impl Default for SceneSettings {
    fn default() -> Self {
        Self { layers: 1, casts_shadow: true }
    }
}

/// An invalid [`SceneSettings`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum SettingsError {
    #[error("SceneSettings.layers = 0: every table of this bake would be invisible to every view (a view draws a table only when `table.layers & view.layers != 0`); set the space's layer bit")]
    NoLayers,
}

impl SceneSettings {
    /// The settings' own invariant: an all-zero layer mask is a table no view can ever draw.
    ///
    /// Checked where the settings **enter** the bake ([`build_cell_table`] and the step's
    /// `plan`), so the error names the caller that chose the value instead of surfacing later
    /// as a [`TableError`] from the writer. [`StaticCellTable::write`] and the reader keep
    /// their own check: a table also arrives from disk, where no [`SceneSettings`] was involved.
    pub fn validate(&self) -> Result<(), SettingsError> {
        if self.layers == 0 {
            return Err(SettingsError::NoLayers);
        }
        Ok(())
    }

    pub fn key_bytes(&self) -> Vec<u8> {
        let mut out = self.layers.to_le_bytes().to_vec();
        out.push(u8::from(self.casts_shadow));
        out
    }
}

/// A placement's model, as the step loaded it.
#[derive(Clone, Copy, Debug)]
pub enum CellModel<'a> {
    /// No model, or not a candidate (module docs).
    None,
    /// A static cook: `path` is the `mesh.cook` subject, `parts` its [`part_spheres`].
    Static { path: &'a str, model: &'a CookedModel, parts: &'a [Option<Sphere>] },
    /// `Cooked::Skinned`.
    Skinned,
    /// `Cooked::Skipped`.
    Skipped,
}

impl CellModel<'_> {
    fn facts(&self, source: &dyn SceneSource) -> ModelFacts {
        match self {
            Self::None | Self::Skipped => ModelFacts::default(),
            Self::Skinned => ModelFacts { cooked: true, animated: true, parts: Vec::new() },
            Self::Static { model, .. } => ModelFacts {
                cooked: true,
                animated: false,
                parts: model.materials.iter().map(|m| source.material(m).kind).collect(),
            },
        }
    }
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum BuildError {
    #[error("{model} part {part}: {error}")]
    Layout { model: String, part: u32, error: LayoutError },
    #[error("{model} part {part}: no bounds (part_spheres)")]
    PartBounds { model: String, part: u32 },
    #[error("more placements than 32-bit ids")]
    TooManyPlacements,
    #[error("one cell draws more than {} models", crate::table::MAX_MODELS)]
    TooManyModels,
    #[error("table: {0}")]
    Table(#[from] TableError),
    #[error("settings: {0}")]
    Settings(#[from] SettingsError),
}

/// The step's pure core: one cell's table from its placements (index = [`PlacementId`]) and
/// their loaded models (same length).
///
/// `settings` are checked first ([`SceneSettings::validate`]): settings no table could ever be
/// drawn with fail here, naming them, not later in [`StaticCellTable::write`].
pub fn build_cell_table(
    source: &dyn SceneSource,
    settings: &SceneSettings,
    origin_cell: IVec3,
    land_min: f32,
    placements: &[Placement],
    models: &[CellModel<'_>],
) -> Result<StaticCellTable, BuildError> {
    assert_eq!(placements.len(), models.len(), "one model slot per placement");
    settings.validate()?;
    let facts: Vec<ModelFacts> = models.iter().map(|m| m.facts(source)).collect();
    let eligibility = source.eligibility(CellFacts { placements, models: &facts });
    let origin = WorldPos::new(origin_cell, Vec3::ZERO);
    // Instances name their chain by mesh key until the chains are known (then by index).
    let mut instances: Vec<(StaticInstance, u64)> = Vec::new();
    // Mesh key → its chain and the `mesh.cook` subject it came from (the table's model section
    // is built from them once every chain is known, and the chain's `model` index set then).
    let mut chains: BTreeMap<u64, (LodChain, &str)> = BTreeMap::new();
    let mut bounds: Option<(Vec3, Vec3)> = None;
    for (k, (placement, model)) in placements.iter().zip(models).enumerate() {
        let id = PlacementId::try_from(k).map_err(|_| BuildError::TooManyPlacements)?;
        let CellModel::Static { path, model, parts: part_bounds } = *model else { continue };
        let parts: Vec<usize> = match classify_placement(&*eligibility, id, placement) {
            Eligibility::Scene => (0..model.materials.len()).collect(),
            Eligibility::Split { scene, .. } => scene,
            Eligibility::Entity(_) => continue,
        };
        let t = &placement.transform;
        let affine = Affine3A::from_scale_rotation_translation(t.scale, t.rotation, origin.delta_f32(t.pos));
        let scale = t.scale.abs().max_element();
        for part in parts {
            let material = u32::try_from(part).map_err(|_| BuildError::TooManyPlacements)?;
            let levels: Vec<LevelSource> = model
                .lods
                .iter()
                .map(|l| LevelSource {
                    error: l.error,
                    submeshes: l.submeshes.iter().filter(|s| s.material == material).map(|s| (s.vertex_count, s.index_count)).collect(),
                })
                .collect();
            let layout = match chain_layout(&levels) {
                Ok(layout) => layout,
                // The part has no geometry at full detail: nothing to draw.
                Err(LayoutError::EmptyFinest) => continue,
                Err(error) => return Err(BuildError::Layout { model: path.to_owned(), part: material, error }),
            };
            let local = part_bounds
                .get(part)
                .copied()
                .flatten()
                .ok_or_else(|| BuildError::PartBounds { model: path.to_owned(), part: material })?;
            let sphere = Sphere { center: affine.transform_point3(local.center), radius: local.radius * scale };
            let scene = source.material(&model.materials[part]);
            let mesh = mesh_key(path, material);
            chains.entry(mesh).or_insert_with(|| {
                let chain = LodChain {
                    mesh,
                    material: material_key(&model.materials[part]),
                    material_class: MaterialClass::of(scene.traits),
                    // Set below, once the model section's order is known.
                    model: 0,
                    part: material,
                    levels: layout.levels,
                };
                (chain, path)
            });
            let mut flags = InstanceFlags::empty();
            flags.set(InstanceFlags::CASTS_SHADOW, settings.casts_shadow);
            let instance = StaticInstance {
                placement: id,
                transform: StaticInstance::rows(&affine),
                sphere,
                chain: 0,
                material_override: 0,
                flags,
            };
            instances.push((instance, mesh));
            let (lo, hi) = (sphere.center - sphere.radius, sphere.center + sphere.radius);
            bounds = Some(bounds.map_or((lo, hi), |(a, b)| (a.min(lo), b.max(hi))));
        }
    }
    // Dense cell-local indices in mesh key order (the table's chain order).
    let index: BTreeMap<u64, u32> = chains.keys().copied().zip(0u32..).collect();
    let instances = instances.into_iter().map(|(i, mesh)| StaticInstance { chain: index[&mesh], ..i }).collect();
    // The model section: every chain's `mesh.cook` subject, sorted and unique (the table's own
    // rule), so a chain's model index is its subject's rank.
    let names: BTreeSet<&str> = chains.values().map(|(_, path)| *path).collect();
    let models: Vec<String> = names.iter().map(|&s| s.to_owned()).collect();
    let model_index: BTreeMap<&str, u32> = names.into_iter().zip(0u32..).collect();
    let chains = chains
        .into_values()
        .map(|(chain, path)| {
            let model = u16::try_from(model_index[path]).map_err(|_| BuildError::TooManyModels)?;
            Ok(LodChain { model, ..chain })
        })
        .collect::<Result<Vec<LodChain>, BuildError>>()?;
    let (min, max) = bounds.unwrap_or((Vec3::ZERO, Vec3::ZERO));
    Ok(StaticCellTable {
        origin_cell,
        aabb: Aabb { min, max },
        land_min,
        layers: settings.layers,
        instances,
        chains,
        models,
    })
}

#[derive(Debug, thiserror::Error)]
pub enum ChainMeshError {
    #[error("the model has no material part {0}")]
    NoPart(u32),
    #[error("layout: {0}")]
    Layout(#[from] LayoutError),
    #[error("mesh: {0}")]
    Mesh(#[from] sf_model::FormatError),
    /// The level's concatenation is not the length the table's level range says: the two sides of
    /// [`chain_layout`] disagree, which no input should be able to cause.
    #[error("level {level}: {indices} indices, the layout says {expected}")]
    LevelLength { level: usize, indices: u32, expected: u32 },
}

/// The `scene.chains` product's core: one chain's geometry as an `sf_model` model with **one LOD
/// level per chain level, one submesh each** and the chain's material as its only material
/// (module docs, "Chain geometry").
///
/// Each level holds that level's submeshes of `part`, in submesh order, concatenated and rebased
/// within the level (`index + the vertices before it in this level`), by the same
/// [`chain_layout`] the table's level ranges come from; a level with no geometry of the part
/// carries a copy of the next finer level's, as its table range does. Decodes and re-encodes
/// through meshopt once per level, offline.
pub fn build_chain_mesh(model: &CookedModel, part: u32) -> Result<CookedModel, ChainMeshError> {
    let material = model.materials.get(part as usize).ok_or(ChainMeshError::NoPart(part))?.clone();
    // The chain's source levels, exactly as `build_cell_table` reads them.
    let parts: Vec<Vec<&CookedSubmesh>> =
        model.lods.iter().map(|l| l.submeshes.iter().filter(|s| s.material == part).collect()).collect();
    let levels: Vec<LevelSource> = model
        .lods
        .iter()
        .zip(&parts)
        .map(|(l, subs)| LevelSource {
            error: l.error,
            submeshes: subs.iter().map(|s| (s.vertex_count, s.index_count)).collect(),
        })
        .collect();
    let layout = chain_layout(&levels)?;
    let mut lods: Vec<CookedLod> = Vec::with_capacity(layout.levels.len());
    let mut bounds: Option<(Vec3, Vec3)> = None;
    for (level, target) in layout.levels.iter().enumerate() {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        for segment in layout.segments.iter().filter(|s| s.level == level) {
            let (v, i) = parts[level][segment.submesh].decode()?;
            // Level-local, not chain-global: this level is its own mesh allocation.
            let base = u32::try_from(vertices.len()).map_err(|_| sf_model::FormatError::TooLarge)?;
            indices.extend(i.iter().map(|&k| k + base));
            for v in &v {
                let p = Vec3::from_array(v.position);
                bounds = Some(bounds.map_or((p, p), |(lo, hi)| (lo.min(p), hi.max(p))));
            }
            vertices.extend(v);
        }
        if vertices.is_empty() {
            // No geometry of this part at this level: the table's range repeats the next finer
            // level, so the mesh must hold that level's geometry.
            let finer = lods.last().expect("chain_layout rejects an empty finest level").clone();
            lods.push(CookedLod { error: target.error, submeshes: finer.submeshes });
            continue;
        }
        let count = u32::try_from(indices.len()).map_err(|_| sf_model::FormatError::TooLarge)?;
        if count != target.index_count {
            return Err(ChainMeshError::LevelLength { level, indices: count, expected: target.index_count });
        }
        lods.push(CookedLod { error: target.error, submeshes: vec![CookedSubmesh::encode(0, &vertices, &indices)?] });
    }
    let (min, max) = bounds.unwrap_or((Vec3::ZERO, Vec3::ZERO));
    Ok(CookedModel {
        bounds: ModelAabb { min: min.to_array(), max: max.to_array() },
        has_normals: model.has_normals,
        has_uvs: model.has_uvs,
        has_colors: model.has_colors,
        materials: vec![material],
        lods,
    })
}

/// The `scene.chains` subjects of a baked table's chains, in chain order: each chain's model
/// (its `mesh.cook` subject, from the table's model section) and material part
/// ([`chain_subject`]). The whole of what the chain bake needs from a cell.
pub fn chain_subjects(view: &StaticCellTableView<'_>) -> Vec<String> {
    (0..u32::try_from(view.chain_count()).unwrap_or(u32::MAX))
        .filter_map(|index| view.chain_source(index))
        .map(|(model, part)| chain_subject(model, part))
        .collect()
}

/// The level-0 terrain tiles (x, z) under a cell's XZ footprint; empty without a grid.
pub fn land_tiles(partition: Partition, cell: IVec3) -> Vec<(i32, i32)> {
    let Some((lo, hi)) = cell_rect(partition, cell) else { return Vec::new() };
    let tile = f64::from(GRID_CELL);
    #[allow(clippy::cast_possible_truncation)] // tile indices of i32 cells
    let range = |a: f64, b: f64| ((a + tile / 2.0) / tile).floor() as i32..=((b + tile / 2.0) / tile).ceil() as i32 - 1;
    let mut out = Vec::new();
    for z in range(lo.y, hi.y) {
        for x in range(lo.x, hi.x) {
            out.push((x, z));
        }
    }
    out
}

/// The cell's absolute XZ rectangle in metres (min, max).
fn cell_rect(partition: Partition, cell: IVec3) -> Option<(DVec2, DVec2)> {
    let extent = f64::from(partition.cell_extent()?);
    let o = partition.cell_origin(cell);
    let lo = DVec2::new(
        f64::from(o.cell.x) * f64::from(GRID_CELL) + f64::from(o.local.x),
        f64::from(o.cell.z) * f64::from(GRID_CELL) + f64::from(o.local.z),
    );
    Some((lo, lo + DVec2::splat(extent)))
}

/// A level-0 terrain tile (x, z) under a cell, with its block bounds (`None`: no tile).
pub type LandTile<'a> = ((i32, i32), Option<&'a BlockBounds>);

/// The cell's land minimum, cell-relative to `origin_cell` (module docs). `tiles` are the
/// [`land_tiles`] of the cell with their block bounds, `None` where the worldspace has no
/// terrain tile.
#[allow(clippy::cast_precision_loss)] // block counts and sizes, small
pub fn land_min(partition: Partition, cell: IVec3, origin_cell: IVec3, tiles: &[LandTile<'_>]) -> f32 {
    let Some((lo, hi)) = cell_rect(partition, cell) else { return f32::NEG_INFINITY };
    let tile = f64::from(GRID_CELL);
    let block = f64::from(BASE_SPACING) * BLOCK_QUADS as f64;
    let mut min = f32::INFINITY;
    for &((tx, tz), bounds) in tiles {
        let Some(bounds) = bounds else { return f32::NEG_INFINITY };
        let t0 = DVec2::new(f64::from(tx), f64::from(tz)) * tile - DVec2::splat(tile / 2.0);
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // clamped to 0..BLOCKS
        let blocks = |a: f64, b: f64| {
            let first = (a / block).floor().clamp(0.0, BLOCKS as f64) as usize;
            let end = (b / block).ceil().clamp(0.0, BLOCKS as f64) as usize;
            first..end
        };
        for bz in blocks(lo.y - t0.y, hi.y - t0.y) {
            for bx in blocks(lo.x - t0.x, hi.x - t0.x) {
                if let Some(&h) = bounds.min.get(bz * BLOCKS + bx) {
                    min = min.min(h);
                }
            }
        }
    }
    if !min.is_finite() {
        return f32::NEG_INFINITY;
    }
    #[allow(clippy::cast_possible_truncation)] // a height in metres
    let relative = (f64::from(min) - f64::from(origin_cell.y) * f64::from(GRID_CELL)) as f32;
    relative - MAX_QUANT_STEP
}

struct LandCoverage {
    tiles: BTreeSet<(i32, i32)>,
    digest: [u8; 32],
}

/// Shared state of the step.
pub struct SceneBake {
    source: Arc<dyn SceneSource>,
    settings: SceneSettings,
    /// Per worldspace: the level-0 terrain tiles that exist, and their set's digest (a key param).
    land: BTreeMap<WorldspaceId, LandCoverage>,
}

impl SceneBake {
    pub fn new(source: Arc<dyn SceneSource>, settings: SceneSettings) -> Self {
        Self { source, settings, land: BTreeMap::new() }
    }

    /// Declares the level-0 terrain tiles of `ws` (the terrain bake's coverage). Declaring an
    /// empty set says "this worldspace has no land": every cell's land minimum is then negative
    /// infinity and the horizon test hides nothing there (interiors).
    ///
    /// Every worldspace baked must be declared, even a landless one: a worldspace the caller
    /// never declared is a wiring error ([`Self::coverage`]), not "no land", because the two
    /// look the same in the output and the wrong one is cached ([`land_min`]).
    #[must_use]
    pub fn with_land_tiles(mut self, ws: WorldspaceId, tiles: impl IntoIterator<Item = (i32, i32)>) -> Self {
        let tiles: BTreeSet<(i32, i32)> = tiles.into_iter().collect();
        let mut bytes = Vec::with_capacity(tiles.len() * 8);
        for (x, z) in &tiles {
            bytes.extend_from_slice(&x.to_le_bytes());
            bytes.extend_from_slice(&z.to_le_bytes());
        }
        self.land.insert(ws, LandCoverage { digest: Digest::of(&bytes).0, tiles });
        self
    }

    /// Registers `scene.cells` and `scene.chains` (the baker also needs `mesh.cook`, `world.*`
    /// and, for land, `terrain.*`).
    pub fn register(self: &Arc<Self>, baker: &mut Baker) {
        baker.register(CellsStep(self.clone()));
        baker.register(ChainsStep);
    }

    /// The declared land coverage of `ws`, or the wiring error. An undeclared worldspace fails
    /// the product instead of baking it with no land: a table baked before the terrain validates
    /// like any other, and its `land_min = -∞` would turn horizon culling off for that cell for
    /// as long as the cache lives (static-scene.md §4 step 3; silently disabling a culling
    /// system is exactly what CONTRIBUTING "Industry standard, never kludge" forbids).
    fn coverage(&self, ws: &WorldspaceId) -> Result<&LandCoverage, BakeError> {
        self.land.get(ws).ok_or_else(|| {
            step_err(
                ws.as_str(),
                format!(
                    "no land coverage declared for worldspace {}: call SceneBake::with_land_tiles for it \
                     (with the terrain bake's level-0 tiles, or an empty set for a landless worldspace) \
                     before baking, and bake {TILE_STEP} first",
                    ws.as_str()
                ),
            )
        })
    }

    /// Every cell's product key, from the worldspaces' `world.dir` (errors where it fails).
    pub fn cell_keys(baker: &Baker, worldspaces: &[WorldspaceId]) -> Result<Vec<ProductKey>, BakeError> {
        let mut keys = Vec::new();
        for ws in worldspaces {
            let digest = baker.product(&ProductKey::new(DIR_STEP, worldspace_subject(ws)))?;
            let dir = CellDirectory::from_bytes(&baker.store().get(&digest)?).map_err(|e| BakeError::Other(e.to_string()))?;
            keys.extend(dir.cells.iter().map(|c| ProductKey::new(CELLS_STEP, cell_subject(ws, c.cell))));
        }
        Ok(keys)
    }

    /// Every chain's product key, from the **baked tables' model sections** (module docs, "Chain
    /// subjects come from the baked tables"), deduplicated across cells: a chain shared by two
    /// cells is baked once, and one whose model did not change is a cache hit.
    ///
    /// Run **after** the cells ([`Self::bake_worldspaces`]): it reads their products. A cell
    /// whose own table failed to bake is skipped — it has already been reported there, and one
    /// broken cell must not cost every chain of the worldspace.
    pub fn chain_keys(baker: &Baker, worldspaces: &[WorldspaceId]) -> Result<Vec<ProductKey>, BakeError> {
        let mut subjects: BTreeSet<String> = BTreeSet::new();
        for key in Self::cell_keys(baker, worldspaces)? {
            let Ok(digest) = baker.product(&key) else { continue };
            let bytes = baker.store().get(&digest)?;
            let view = StaticCellTableView::parse(&bytes).map_err(|e| BakeError::step(&key, e))?;
            subjects.extend(chain_subjects(&view));
        }
        Ok(subjects.into_iter().map(|s| ProductKey::new(CHAINS_STEP, s)).collect())
    }

    /// Bakes every chain the baked cells of `worldspaces` name, in parallel. Call it right after
    /// [`Self::bake_worldspaces`], in the same baker (scene-runtime-plan.md §3.3).
    pub fn bake_chains(baker: &Baker, worldspaces: &[WorldspaceId]) -> BakeReport {
        let started = Instant::now();
        match Self::chain_keys(baker, worldspaces) {
            Ok(keys) => {
                let mut report = baker.bake(&keys);
                report.elapsed = started.elapsed();
                report
            }
            Err(e) => BakeReport {
                products: vec![(ProductKey::new(CHAINS_STEP, ""), Err(Arc::new(e)))],
                steps: baker.step_stats(),
                elapsed: started.elapsed(),
            },
        }
    }

    /// Bakes every cell of `worldspaces` in parallel.
    pub fn bake_worldspaces(baker: &Baker, worldspaces: &[WorldspaceId]) -> BakeReport {
        let started = Instant::now();
        match Self::cell_keys(baker, worldspaces) {
            Ok(keys) => {
                let mut report = baker.bake(&keys);
                report.elapsed = started.elapsed();
                report
            }
            Err(e) => BakeReport {
                products: vec![(ProductKey::new(DIR_STEP, ""), Err(Arc::new(e)))],
                steps: baker.step_stats(),
                elapsed: started.elapsed(),
            },
        }
    }
}

fn step_err(subject: &str, e: impl std::fmt::Display) -> BakeError {
    BakeError::step(&ProductKey::new(CELLS_STEP, subject), e)
}

fn chain_err(subject: &str, e: impl std::fmt::Display) -> BakeError {
    BakeError::step(&ProductKey::new(CHAINS_STEP, subject), e)
}

/// `scene.chains`: one chain's geometry (module docs, "Chain geometry").
///
/// Stateless, unlike [`CellsStep`]: a chain's mesh is a function of the cooked model and the
/// layout rule alone — no eligibility, no settings, no land — so two mod sets that cook a model
/// the same way share its chains.
pub struct ChainsStep;

impl Step for ChainsStep {
    fn id(&self) -> &'static str {
        CHAINS_STEP
    }
    /// 1: one `.sfmesh` per chain, one LOD level per chain level, one submesh each.
    fn version(&self) -> u32 {
        1
    }
    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        let (model, _part) = parse_chain_subject(subject).ok_or_else(|| chain_err(subject, "malformed subject"))?;
        // A static input: the cook is known from the subject, so invalidation is exact.
        cx.product(MeshCookStep::ID, model);
        // The layout rule decides the concatenation, and the table's level ranges address it.
        cx.param("layout", crate::chain::LAYOUT_VERSION.to_le_bytes().to_vec());
        Ok(())
    }
    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let (model, part) = parse_chain_subject(subject).ok_or_else(|| chain_err(subject, "malformed subject"))?;
        let bytes = cx.product(MeshCookStep::ID, model)?;
        let cook_err = |e| BakeError::step(&ProductKey::new(MeshCookStep::ID, model), e);
        let Cooked::Static(cooked) = Cooked::from_bytes(&bytes).map_err(cook_err)? else {
            // Only a static cook has chains; `scene.cells` never names any other kind.
            return Err(chain_err(subject, "the chain's model is not a static cook"));
        };
        let chain = build_chain_mesh(&cooked, part).map_err(|e| chain_err(subject, e))?;
        Ok(Cooked::Static(chain).to_bytes())
    }
}

/// `scene.cells`.
pub struct CellsStep(pub Arc<SceneBake>);

impl Step for CellsStep {
    fn id(&self) -> &'static str {
        CELLS_STEP
    }
    /// 3: table format v3 (the chain source and the model section).
    fn version(&self) -> u32 {
        3
    }
    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        let (ws, cell) = parse_cell_subject(subject).ok_or_else(|| step_err(subject, "malformed subject"))?;
        // The settings came from whoever built this `SceneBake`: reject theirs here, before a
        // cell is planned, rather than per cell from the table writer.
        self.0.settings.validate().map_err(|e| step_err(subject, e))?;
        cx.product(PAGE_STEP, page_subject(&ws, Partition::page_of(cell)));
        cx.param("source", self.0.source.fingerprint().to_vec()).param("settings", self.0.settings.key_bytes());
        cx.param("land", self.0.coverage(&ws)?.digest.to_vec());
        cx.param("table", crate::table::FORMAT_VERSION.to_le_bytes().to_vec());
        // The chain layout rule decides every level range in the table (and, in `scene.chains`,
        // the geometry those ranges address): a changed rule is a stale table, so it is a key.
        cx.param("layout", crate::chain::LAYOUT_VERSION.to_le_bytes().to_vec());
        Ok(())
    }
    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let (ws, cell) = parse_cell_subject(subject).ok_or_else(|| step_err(subject, "malformed subject"))?;
        let page = IndexPage::from_bytes(&cx.product(PAGE_STEP, &page_subject(&ws, Partition::page_of(cell)))?)
            .map_err(|e| step_err(subject, e))?;
        let placements: Vec<Placement> =
            page.cell(cell).map(|c| c.placements.iter().map(|p| p.placement.clone()).collect()).unwrap_or_default();
        // Each model once: its cook and, for a static cook, its parts' LOD0 spheres.
        let mut cooked: BTreeMap<&str, (Cooked, Vec<Option<Sphere>>)> = BTreeMap::new();
        for p in placements.iter().filter(|p| p.class == PlacementClass::Static) {
            let Some(path) = &p.model else { continue };
            if cooked.contains_key(path.as_str()) {
                continue;
            }
            let bytes = cx.product(MeshCookStep::ID, path.as_str())?;
            let cook_err = |e| BakeError::step(&ProductKey::new(MeshCookStep::ID, path.as_str()), e);
            let model = Cooked::from_bytes(&bytes).map_err(cook_err)?;
            let parts = match &model {
                Cooked::Static(m) => part_spheres(m).map_err(cook_err)?,
                _ => Vec::new(),
            };
            cooked.insert(path.as_str(), (model, parts));
        }
        let models: Vec<CellModel<'_>> = placements
            .iter()
            .map(|p| match p.model.as_ref().filter(|_| p.class == PlacementClass::Static).and_then(|m| cooked.get_key_value(m.as_str())) {
                None => CellModel::None,
                Some((&path, (Cooked::Static(model), parts))) => CellModel::Static { path, model, parts },
                Some((_, (Cooked::Skinned(_), _))) => CellModel::Skinned,
                Some((_, (Cooked::Skipped { .. }, _))) => CellModel::Skipped,
            })
            .collect();
        let origin_cell = page.partition.cell_origin(cell).cell;
        let tiles = land_tiles(page.partition, cell);
        let coverage = &self.0.coverage(&ws)?.tiles;
        let mut products = Vec::with_capacity(tiles.len());
        for &(x, z) in &tiles {
            products.push(if coverage.contains(&(x, z)) {
                let bytes = cx.product(TILE_STEP, &tile_subject(&ws, TileCoord::new(0, x, z)))?;
                Some(TileProduct::from_bytes(&bytes).map_err(|e| step_err(subject, e))?)
            } else {
                None
            });
        }
        let land: Vec<LandTile<'_>> =
            tiles.iter().copied().zip(products.iter().map(|p| p.as_ref().map(|t| &t.bounds))).collect();
        let land_min = land_min(page.partition, cell, origin_cell, &land);
        let table = build_cell_table(&*self.0.source, &self.0.settings, origin_cell, land_min, &placements, &models)
            .map_err(|e| step_err(subject, e))?;
        // The product is what the runtime will trust: check the chains' mesh keys against the
        // sources the table now carries, which only this side can (`mesh_key` is a hash).
        table.verify_chain_sources().map_err(|e| step_err(subject, e))?;
        table.write().map_err(|e| step_err(subject, e))
    }
    /// Never: a table is dense binary already, and the runtime is to read it in place and upload
    /// [`crate::StaticCellTableView::instance_bytes`] straight to the GPU (static-scene.md §2,
    /// "mmap-read"). An LZ4'd product can only be decompressed into a heap buffer, which is the
    /// opposite of what the format is for, so it must not be stored compressed even while
    /// `sf_bake::Store` still hands out owned bytes.
    fn compress(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests;
