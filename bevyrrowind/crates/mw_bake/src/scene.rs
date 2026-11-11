//! The Morrowind static scene (`superfargoth/docs/specs/static-scene.md` §1, §5; S1b): which
//! placed objects the `scene.cells` step may put in the GPU table, and how NIF materials map
//! to scene material kinds and traits.
//!
//! The rules are adapter knowledge, kept here so the framework stays game-agnostic:
//! - **Class.** Only `STAT` placements are scenery; a `STAT` placed as a marker (`mw_world`'s
//!   editor markers) is a marker. Doors, containers, activators, lights, actors and items stay
//!   entities.
//! - **Scripts.** A base record with a script (`ACTI`, `DOOR`, `CONT`, `LIGH`; `STAT` has no
//!   script field) is scripted.
//! - **References.** A base id that any script or dialogue result names (the same token scan
//!   as the precombine veto, [`crate::far`]) can be the target of `"id"->Disable`, `SetPos`,
//!   `Position`..., so it stays an entity. A token match: a false positive only keeps an object
//!   out of the scene. A placement that starts disabled is barred by the framework itself
//!   (`sf_scene::bake::BARRED`), and kept here as well: a cheap, local answer.
//! - **Models.** As the cooker reports them: `mesh.cook` skips models with any controller,
//!   billboard, particle system or switch node (`importer.rs` "Dynamic parts"), so they are not
//!   cooked and stay entities.
//! - **Materials** ([`scene_material`]). Opaque and alpha-tested are scene materials, where
//!   "alpha-tested" is what the **runtime** draws, not what the importer wrote: an authored
//!   blend the importer marked a cutout candidate (`nif.blend = "cutout_candidate"`) is promoted
//!   to an alpha test when the cooked model loads **if its texture's alpha is a cutout**, so the
//!   bake promotes exactly those ([`runtime_alpha`], reading the texture bake's
//!   [`AlphaClasses`]): foliage, fences and grates go in the table, glass and gauze do not. True
//!   blends and additive blends stay entities, as do surfaces that skip the depth write or test.
//!   Two-sided from the NIF stencil/double-sided flag. No wind: Morrowind has no wind data yet.

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::sync::Arc;

use sf_bake::{BakeError, BakeReport, Baker, Digest, ProductKey};
use sf_model::{AlphaMode, Material};
use sf_records::RecordDb;
use sf_scene::bake::{CellFacts, SceneBake, SceneMaterial, SceneSettings, SceneSource};
use sf_scene::{ClassKind, MaterialKind, MaterialTraits, SceneEligibility};
use sf_terrain::bake::{TILE_STEP, parse_tile_subject};
use sf_world::{PlacementClass, PlacementFlags, WorldspaceId};
use tes3::esp::{Activator, Container, DialogueInfo, Door, Light, Script};

use crate::delight::{ALPHA_INDEX_STEP, ALPHA_INDEX_SUBJECT, AlphaClass, AlphaClassIndex};
use crate::extras;
use crate::far::script_tokens;

/// Bumped when the rules above change (part of every `scene.cells` key). 2: cutout candidates
/// are alpha-tested here as the runtime promotes them ([`runtime_alpha`]). 3: only candidates
/// whose texture is a cutout, from the texture bake's classes ([`AlphaClasses`]).
const RULES_VERSION: u32 = 3;

/// The texture alpha classes the material rule reads: the texture bake's
/// `texture.alpha_class.index` ([`crate::delight::AlphaClassIndex`]), keyed by the reference a
/// material names.
///
/// Part of [`MwSceneSource::fingerprint`], so a texture whose class changed (or a changed
/// classifier, which re-bakes every class product) re-bakes every cell's table. That is coarse —
/// one changed texture re-bakes a worldspace — and deliberate: the adapter's fingerprint is the
/// framework's key for exactly this, "the record facts the rules read" (`sf_scene::bake`), and a
/// cell's planned inputs cannot name the textures its models use without reading the cooked
/// models first, which `plan` cannot do.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AlphaClasses(BTreeMap<String, AlphaClass>);

impl AlphaClasses {
    pub fn new(classes: BTreeMap<String, AlphaClass>) -> Self {
        Self(classes)
    }

    /// Whether `texture`'s alpha is a cutout: `false` for a texture with no class (one the
    /// texture bake never saw, a reference that resolves to no file) and for no texture at all.
    /// The conservative answer, because it leaves a candidate part blended and so **out** of the
    /// static table (`sf_scene::eligibility::MaterialKind::in_scene`), where a wrong `true` would
    /// draw a translucent surface alpha-tested.
    ///
    /// The key is the reference as the material names it: both sides come from
    /// `mw_nif::texture_path` (lowercased, `/`-separated), so they are the same string by
    /// construction (`delight::AlphaIndexStep::run` asserts it in debug builds).
    pub fn is_cutout(&self, texture: Option<&str>) -> bool {
        texture.and_then(|t| self.0.get(t)) == Some(&AlphaClass::Cutout)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// The classes as key bytes (every reference and its class, in key order).
    fn key_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        for (reference, class) in &self.0 {
            out.extend(reference.as_bytes());
            out.push(0);
            out.push(match class {
                AlphaClass::Cutout => 1,
                AlphaClass::Soft => 2,
                AlphaClass::Undecodable => 3,
            });
        }
        out
    }
}

/// [`SceneSource`] for Morrowind content.
pub struct MwSceneSource {
    /// Base ids (lowercase) whose record carries a script.
    scripted: HashSet<String>,
    /// Every identifier-like token of every script and dialogue result (lowercase).
    tokens: BTreeSet<String>,
    /// The alpha class of every cutout candidate's texture ([`runtime_alpha`]).
    textures: AlphaClasses,
    fingerprint: [u8; 32],
}

impl MwSceneSource {
    pub fn build(db: &RecordDb, textures: AlphaClasses) -> Self {
        let mut scripted = Vec::new();
        macro_rules! scripted {
            ($($ty:ty),*) => {$(
                for (id, record) in db.iter::<$ty>() {
                    if !record.script.trim().is_empty() {
                        scripted.push(id.as_str().to_ascii_lowercase());
                    }
                }
            )*};
        }
        scripted!(Activator, Door, Container, Light);
        let mut tokens = BTreeSet::new();
        for (_, script) in db.iter::<Script>() {
            script_tokens(&script.text, &mut tokens);
        }
        for (_, info) in db.iter::<DialogueInfo>() {
            script_tokens(&info.script_text, &mut tokens);
        }
        Self::from_parts(scripted, tokens, textures)
    }

    /// From the scripted base ids, the script tokens (any case) and the texture alpha classes.
    pub fn from_parts(
        scripted: impl IntoIterator<Item = String>,
        tokens: impl IntoIterator<Item = String>,
        textures: AlphaClasses,
    ) -> Self {
        let scripted: BTreeSet<String> = scripted.into_iter().map(|s| s.to_ascii_lowercase()).collect();
        let tokens: BTreeSet<String> = tokens.into_iter().map(|s| s.to_ascii_lowercase()).collect();
        let mut bytes = RULES_VERSION.to_le_bytes().to_vec();
        for s in &scripted {
            bytes.extend(s.as_bytes());
            bytes.push(0);
        }
        // Separates the lists (no id contains 0xFF: they are ASCII-lowercased UTF-8).
        bytes.push(0xFF);
        for s in &tokens {
            bytes.extend(s.as_bytes());
            bytes.push(0);
        }
        bytes.push(0xFF);
        bytes.extend(textures.key_bytes());
        Self { scripted: scripted.into_iter().collect(), tokens, textures, fingerprint: Digest::of(&bytes).0 }
    }
}

/// The alpha mode the **runtime** will draw this cooked material with, which is not always
/// `m.alpha`: the cooked-model loader promotes an authored blend that the importer marked a
/// cutout candidate to an alpha test at [`mw_nif::CUTOUT_CUTOFF`]. That promotion is how
/// Morrowind's alpha-blended foliage, fences and grates actually render, and the bake has to
/// agree with it — a part baked `Blended` is kept out of the table
/// (`sf_scene::eligibility::MaterialKind::in_scene`) while the runtime draws it masked.
///
/// **Source of truth: `bevyrrowind/crates/mw_assets/src/cooked.rs:134-150`** (`surface`), whose
/// `alpha_mode` match this reproduces. Reproduced and not shared, because `mw_assets` depends on
/// `mw_bake` (`mw_bake::textures`, `mw_bake::delight`), so the dependency cannot go the other
/// way; the test `cooker_promotion_rule_is_unchanged` reads that file and fails if the two ever
/// diverge.
///
/// The cooker's two other inputs, and what the bake does with them:
/// - `mw_assets::texture_is_cutout(vfs, base_color_texture)` — the texture's alpha decides.
///   `textures` is the bake's answer to the same question, classified once per texture file by
///   the texture bake (`delight::AlphaClassStep`, the same `textures::alpha_is_cutout` the loader
///   calls on the same file), so the two agree instead of the bake assuming. A candidate with no
///   class is **not** promoted ([`AlphaClasses::is_cutout`]).
/// - `mw_assets::cutout_foliage` (`--off leafmask`): a runtime A/B, so it cannot be in a cache
///   key; the bake assumes its default (on). That is what makes the `blended` arm below dead
///   here, and it was already so for the `"cutout"` case.
fn runtime_alpha(m: &Material, textures: &AlphaClasses) -> AlphaMode {
    // `cooked.rs:133-137`, with `cutout_foliage()` at its default (on), so `blended` is false.
    let cutout = m.extras.get(extras::BLEND).map(String::as_str);
    let additive = cutout == Some("additive");
    let candidate = cutout == Some("cutout_candidate") && textures.is_cutout(m.base_color_texture.as_deref());
    // `cooked.rs:144-151`, arm for arm. `sf_model::AlphaMode` has no additive form, so the
    // cooker's `AlphaMode::Add` arm keeps the authored `Blend` here and [`scene_material`] reads
    // the `"additive"` extra for the kind; the arm stays, so the order cannot drift.
    match m.alpha {
        AlphaMode::Opaque => AlphaMode::Opaque,
        // `SfAlpha::Mask(_) if blended => AlphaMode::Blend`: `--off leafmask` only.
        AlphaMode::Mask(cutoff) => AlphaMode::Mask(cutoff),
        AlphaMode::Blend if additive => AlphaMode::Blend,
        AlphaMode::Blend if candidate => AlphaMode::Mask(mw_nif::CUTOUT_CUTOFF),
        AlphaMode::Blend => AlphaMode::Blend,
    }
}

/// How a cooked NIF material enters the scene (module docs, "Materials"). `textures` are the
/// texture bake's alpha classes, which decide whether a cutout candidate is alpha-tested
/// ([`runtime_alpha`]).
pub fn scene_material(m: &Material, textures: &AlphaClasses) -> SceneMaterial {
    let extra = |key: &str| m.extras.get(key).map(String::as_str);
    let no_depth = extra(extras::DEPTH_WRITE) == Some("false") || extra(extras::DEPTH_TEST) == Some("false");
    // The alpha mode the runtime will use, promotions included, not the one the importer wrote.
    let kind = match runtime_alpha(m, textures) {
        AlphaMode::Blend if extra(extras::BLEND) == Some("additive") => MaterialKind::Additive,
        AlphaMode::Blend => MaterialKind::Blended,
        _ if no_depth => MaterialKind::Blended,
        AlphaMode::Mask(_) => MaterialKind::AlphaMasked,
        AlphaMode::Opaque => MaterialKind::Opaque,
    };
    SceneMaterial {
        kind,
        traits: MaterialTraits { masked: kind == MaterialKind::AlphaMasked, two_sided: m.double_sided, wind: false },
    }
}

/// One cell's facts under the Morrowind rules.
struct MwCell<'a> {
    source: &'a MwSceneSource,
    cell: CellFacts<'a>,
}

impl MwCell<'_> {
    fn placement(&self, p: u32) -> &sf_world::Placement {
        &self.cell.placements[p as usize]
    }

    fn base(&self, p: u32) -> String {
        self.placement(p).base.id.as_str().to_ascii_lowercase()
    }
}

impl SceneEligibility for MwCell<'_> {
    fn class_kind(&self, p: u32) -> ClassKind {
        let placement = self.placement(p);
        if placement.class == PlacementClass::Marker {
            return ClassKind::Marker;
        }
        match &*placement.base.kind {
            "STAT" if placement.class == PlacementClass::Static => ClassKind::Static,
            "STAT" => ClassKind::Marker,
            "DOOR" => ClassKind::Door,
            "CONT" => ClassKind::Container,
            "ACTI" => ClassKind::Activator,
            "LIGH" => ClassKind::Light,
            "NPC_" | "CREA" => ClassKind::Actor,
            _ => ClassKind::Item,
        }
    }

    fn has_script(&self, p: u32) -> bool {
        self.source.scripted.contains(&self.base(p))
    }

    fn is_referenced(&self, p: u32) -> bool {
        self.placement(p).flags.contains(PlacementFlags::DISABLED) || self.source.tokens.contains(&self.base(p))
    }

    fn model_cooked(&self, p: u32) -> bool {
        self.cell.models[p as usize].cooked
    }

    fn has_animated_controllers(&self, p: u32) -> bool {
        self.cell.models[p as usize].animated
    }

    fn part_materials(&self, p: u32) -> &[MaterialKind] {
        &self.cell.models[p as usize].parts
    }
}

impl SceneSource for MwSceneSource {
    fn fingerprint(&self) -> [u8; 32] {
        self.fingerprint
    }

    fn material(&self, material: &Material) -> SceneMaterial {
        scene_material(material, &self.textures)
    }

    fn eligibility<'a>(&'a self, cell: CellFacts<'a>) -> Box<dyn SceneEligibility + 'a> {
        Box::new(MwCell { source: self, cell })
    }
}

/// The texture alpha classes the material rule reads, from the texture bake's
/// `texture.alpha_class.index` ([`crate::delight::bake_delight`]).
///
/// An error when that product cannot be resolved (the texture bake never ran on this baker, so
/// the step is not even registered). Not "no classes": without them no cutout candidate is
/// promoted, so every leaf card, fence and grate in the world would quietly stay out of the
/// static table and the `--scene` A/B would look like a broken GPU path — the same trap as a
/// scene baked before the terrain, and the same answer (fail the bake, say what to run).
pub fn alpha_classes(baker: &Baker) -> Result<AlphaClasses, BakeError> {
    let key = ProductKey::new(ALPHA_INDEX_STEP, ALPHA_INDEX_SUBJECT);
    let digest = baker.product(&key).map_err(|e| {
        BakeError::Other(format!(
            "scene.cells: no {ALPHA_INDEX_STEP} product ({e}). Bake the textures \
             (mw_bake::delight::bake_delight) before bake_scene: without the per-texture alpha \
             classes the material rule cannot tell a cutout candidate (foliage, fences, grates) \
             from a translucent blend, and every candidate stays out of the table \
             (mw_bake::scene::runtime_alpha)."
        ))
    })?;
    let bytes = baker.store().get(&digest)?;
    let index: AlphaClassIndex =
        postcard::from_bytes(&bytes).map_err(|e| BakeError::step(&key, format!("malformed alpha class index: {e}")))?;
    Ok(AlphaClasses::new(index.classes))
}

/// The level-0 terrain tiles this baker session resolved for `ws` (the terrain bake's coverage).
pub fn land_tiles(baker: &Baker, ws: &WorldspaceId) -> Vec<(i32, i32)> {
    baker
        .manifest()
        .products
        .keys()
        .filter(|k| k.step == TILE_STEP)
        .filter_map(|k| parse_tile_subject(&k.subject))
        .filter(|(w, c)| w == ws && c.level == 0)
        .map(|(_, c)| (c.x, c.z))
        .collect()
}

/// Registers `scene.cells` on `baker` (which already has the model, world and terrain steps and
/// their products) and bakes every cell of `worldspaces`.
///
/// **Order matters:** the terrain bake must have run on this baker first. Its level-0 tiles are
/// where each cell's land minimum comes from, and a cell baked without them gets `land_min = -∞`
/// — a table that validates, keeps its digest and turns horizon culling off for that cell for as
/// long as the cache lives (static-scene.md §4 step 3). So a worldspace with no level-0
/// `terrain.tile` product in the baker's manifest is an error here, not a silent landless bake:
/// every Morrowind worldspace that gets a table is an exterior with `LAND` records.
///
/// The **texture** bake must have run first too, for the same kind of reason
/// ([`alpha_classes`]): its per-texture alpha classes are what tell a cutout candidate from a
/// translucent blend, and without them foliage silently stays out of the table.
pub fn bake_scene(baker: &mut Baker, db: &RecordDb, worldspaces: &[WorldspaceId]) -> Result<BakeReport, BakeError> {
    let source = Arc::new(MwSceneSource::build(db, alpha_classes(baker)?));
    let mut bake = SceneBake::new(source, SceneSettings::default());
    for ws in worldspaces {
        let tiles = land_tiles(baker, ws);
        if tiles.is_empty() {
            return Err(BakeError::Other(format!(
                "scene.cells: worldspace {} has no level-0 {TILE_STEP} product in this baker's manifest. \
                 Bake the terrain (mw_bake::world::bake_exterior) before bake_scene: without it every cell \
                 bakes with no land minimum and horizon culling stays off in the cache (static-scene.md §4).",
                ws.as_str()
            )));
        }
        bake = bake.with_land_tiles(ws.clone(), tiles);
    }
    Arc::new(bake).register(baker);
    Ok(SceneBake::bake_worldspaces(baker, worldspaces))
}

/// Bakes the geometry of every chain the baked `scene.cells` tables of `worldspaces` name
/// (`scene.chains`: one `.sfmesh` per chain, one LOD level per chain level; scene-runtime-plan.md
/// §3.3). Chains dedup across cells, and one whose cooked model did not change is a cache hit.
///
/// Run it right after [`bake_scene`] on the same baker: the subjects come from the cells'
/// products, and the step is registered there. Called first, it reports one failure (no cell
/// directory) instead of an empty, successful bake.
pub fn bake_chains(baker: &Baker, worldspaces: &[WorldspaceId]) -> BakeReport {
    SceneBake::bake_chains(baker, worldspaces)
}

#[cfg(test)]
mod tests;
