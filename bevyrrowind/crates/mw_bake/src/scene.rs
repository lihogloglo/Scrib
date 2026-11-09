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
//!   `Position`..., so it stays an entity; so does a placement that starts disabled. A token
//!   match: a false positive only keeps an object out of the scene.
//! - **Models.** As the cooker reports them: `mesh.cook` skips models with any controller,
//!   billboard, particle system or switch node (`importer.rs` "Dynamic parts"), so they are not
//!   cooked and stay entities.
//! - **Materials** ([`scene_material`]). Opaque and alpha-tested (including cutouts promoted
//!   from authored blends) are scene materials; blends, additive blends and blends whose
//!   texture decides at load (`nif.blend = "cutout_candidate"`) stay entities, as do surfaces
//!   that skip the depth write or test. Two-sided from the NIF stencil/double-sided flag. No
//!   wind: Morrowind has no wind data yet.

use std::collections::{BTreeSet, HashSet};
use std::sync::Arc;

use sf_bake::{BakeReport, Baker, Digest};
use sf_model::{AlphaMode, Material};
use sf_records::RecordDb;
use sf_scene::bake::{CellFacts, SceneBake, SceneMaterial, SceneSettings, SceneSource};
use sf_scene::{ClassKind, MaterialKind, MaterialTraits, SceneEligibility};
use sf_terrain::bake::{TILE_STEP, parse_tile_subject};
use sf_world::{PlacementClass, PlacementFlags, WorldspaceId};
use tes3::esp::{Activator, Container, DialogueInfo, Door, Light, Script};

use crate::extras;
use crate::far::script_tokens;

/// Bumped when the rules above change (part of every `scene.cells` key).
const RULES_VERSION: u32 = 1;

/// [`SceneSource`] for Morrowind content.
pub struct MwSceneSource {
    /// Base ids (lowercase) whose record carries a script.
    scripted: HashSet<String>,
    /// Every identifier-like token of every script and dialogue result (lowercase).
    tokens: BTreeSet<String>,
    fingerprint: [u8; 32],
}

impl MwSceneSource {
    pub fn build(db: &RecordDb) -> Self {
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
        Self::from_parts(scripted, tokens)
    }

    /// From the scripted base ids and the script tokens (any case).
    pub fn from_parts(scripted: impl IntoIterator<Item = String>, tokens: impl IntoIterator<Item = String>) -> Self {
        let scripted: BTreeSet<String> = scripted.into_iter().map(|s| s.to_ascii_lowercase()).collect();
        let tokens: BTreeSet<String> = tokens.into_iter().map(|s| s.to_ascii_lowercase()).collect();
        let mut bytes = RULES_VERSION.to_le_bytes().to_vec();
        for s in &scripted {
            bytes.extend(s.as_bytes());
            bytes.push(0);
        }
        // Separates the two lists (no id contains 0xFF: they are ASCII-lowercased UTF-8).
        bytes.push(0xFF);
        for s in &tokens {
            bytes.extend(s.as_bytes());
            bytes.push(0);
        }
        Self { scripted: scripted.into_iter().collect(), tokens, fingerprint: Digest::of(&bytes).0 }
    }
}

/// How a cooked NIF material enters the scene (module docs, "Materials").
pub fn scene_material(m: &Material) -> SceneMaterial {
    let extra = |key: &str| m.extras.get(key).map(String::as_str);
    let no_depth = extra(extras::DEPTH_WRITE) == Some("false") || extra(extras::DEPTH_TEST) == Some("false");
    let kind = match m.alpha {
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
        scene_material(material)
    }

    fn eligibility<'a>(&'a self, cell: CellFacts<'a>) -> Box<dyn SceneEligibility + 'a> {
        Box::new(MwCell { source: self, cell })
    }
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
pub fn bake_scene(baker: &mut Baker, db: &RecordDb, worldspaces: &[WorldspaceId]) -> BakeReport {
    let source = Arc::new(MwSceneSource::build(db));
    let mut bake = SceneBake::new(source, SceneSettings::default());
    for ws in worldspaces {
        bake = bake.with_land_tiles(ws.clone(), land_tiles(baker, ws));
    }
    Arc::new(bake).register(baker);
    SceneBake::bake_worldspaces(baker, worldspaces)
}

#[cfg(test)]
mod tests;
