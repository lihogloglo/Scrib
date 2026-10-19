//! The Morrowind far field (`superfargoth/docs/specs/far-field.md`): which
//! models are foliage (octahedral impostors beyond the object range), what
//! lights look like from across the map, and texture decoding for proxy
//! albedo and impostor capture.
//!
//! The rules are adapter knowledge, kept here so the framework stays
//! game-agnostic:
//! - **Foliage.** Morrowind ships its trees and large plants as
//!   `meshes/f/flora_*` (Bitter Coast, Ascadian Isles, West Gash, Grazelands
//!   trees, Emperor parasols, …). A flora model taller than
//!   [`IMPOSTOR_MIN_HEIGHT`] becomes an impostor; smaller flora (bushes,
//!   grass, kelp) merges like any static and leaves the proxies once it is
//!   under two pixels.
//! - **Lights.** `LIGH` color and radius drive the sprite; lights flagged
//!   negative or off by default don't glow.
//! - **Precombine veto** (precombine.md). A static whose base id appears in
//!   any script or dialogue result text can be the target of `"id"->Disable`,
//!   `SetPos`, `Position`…, so it stays its own entity. A token match, so a
//!   false positive (an id that is also a variable name) only keeps an object
//!   out of the merge.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use mw_world::convert::UNIT_METERS_F64;
use sf_bake::{Baker, BakeReport, Digest};
use sf_lod::bake::FarBake;
use sf_lod::{FarBakeSettings, FarLight, FarPolicy, FarSource, ModelSummary, TextureDecoder, TextureImage};
use sf_records::RecordDb;
use sf_world::{Placement, WorldspaceId};
use tes3::esp::{DialogueInfo, Light, LightFlags, Script};

use crate::textures::decode_rgba;

/// Flora at least this tall (meters) is drawn as an impostor.
pub const IMPOSTOR_MIN_HEIGHT: f32 = 3.0;

/// Bumped when the rules below change (part of every far bake key).
const RULES_VERSION: u32 = 2;

/// [`FarSource`] for Morrowind content.
pub struct MwFarSource {
    /// Light parameters by base record id (lowercase).
    lights: BTreeMap<String, FarLight>,
    /// Every identifier-like token of every script and dialogue result (lowercase).
    script_tokens: BTreeSet<String>,
    fingerprint: [u8; 32],
}

/// The ids a script text can name, lowercase: quoted strings whole (ids with
/// spaces, `"chargen boat"->Disable`) and every identifier-like word.
fn script_tokens(text: &str, out: &mut BTreeSet<String>) {
    for quoted in text.split('"').skip(1).step_by(2) {
        let t = quoted.trim();
        if !t.is_empty() {
            out.insert(t.to_ascii_lowercase());
        }
    }
    let text = text.replace("->", " ");
    for word in text.split(|c: char| !(c.is_alphanumeric() || matches!(c, '_' | '-' | '.' | '\''))) {
        if !word.is_empty() {
            out.insert(word.to_ascii_lowercase());
        }
    }
}

impl MwFarSource {
    pub fn build(db: &RecordDb) -> Self {
        let mut lights = BTreeMap::new();
        for (id, light) in db.iter::<Light>() {
            lights.insert(id.as_str().to_ascii_lowercase(), far_light(light));
        }
        let mut script_tokens_set = BTreeSet::new();
        for (_, script) in db.iter::<Script>() {
            script_tokens(&script.text, &mut script_tokens_set);
        }
        for (_, info) in db.iter::<DialogueInfo>() {
            script_tokens(&info.script_text, &mut script_tokens_set);
        }
        let mut bytes = RULES_VERSION.to_le_bytes().to_vec();
        for t in &script_tokens_set {
            bytes.extend(t.as_bytes());
            bytes.push(0);
        }
        for (id, l) in &lights {
            bytes.extend(id.as_bytes());
            for f in l.color.iter().chain(&[l.intensity, l.radius]) {
                bytes.extend(f.to_le_bytes());
            }
        }
        Self { lights, script_tokens: script_tokens_set, fingerprint: Digest::of(&bytes).0 }
    }

    pub fn light_count(&self) -> usize {
        self.lights.len()
    }
}

/// A `LIGH` record seen from afar.
fn far_light(light: &Light) -> FarLight {
    let d = &light.data;
    let off = d.flags.intersects(LightFlags::NEGATIVE | LightFlags::OFF_BY_DEFAULT);
    let srgb = |c: u8| {
        let s = f32::from(c) / 255.0;
        if s <= 0.040_45 { s / 12.92 } else { ((s + 0.055) / 1.055).powf(2.4) }
    };
    let mut color = [srgb(d.color[0]), srgb(d.color[1]), srgb(d.color[2])];
    // Normalize hue to unit brightness; intensity carries the strength.
    let max = color.iter().copied().fold(0.0f32, f32::max).max(1e-3);
    for c in &mut color {
        *c /= max;
    }
    #[allow(clippy::cast_possible_truncation)]
    let radius = (f64::from(d.radius) * UNIT_METERS_F64) as f32;
    FarLight { color, intensity: if off { 0.0 } else { (radius / 4.0).clamp(0.3, 2.0) }, radius: radius.clamp(1.0, 12.0), offset: [0.0; 3] }
}

/// Whether a model path is Morrowind flora.
pub fn is_flora(model: &str) -> bool {
    let m = model.to_ascii_lowercase();
    m.contains("/flora_") || m.contains("/f/flora")
}

impl FarSource for MwFarSource {
    fn fingerprint(&self) -> [u8; 32] {
        self.fingerprint
    }

    fn policy(&self, model: &str, summary: &ModelSummary) -> Option<FarPolicy> {
        (is_flora(model) && summary.height() >= IMPOSTOR_MIN_HEIGHT).then_some(FarPolicy::Impostor)
    }

    fn light(&self, placement: &Placement) -> Option<FarLight> {
        Some(self.lights.get(&placement.base.id.as_str().to_ascii_lowercase()).copied().unwrap_or(FarLight { intensity: 0.0, ..FarLight::default() }))
    }

    fn precombine_exclude(&self, placement: &Placement) -> bool {
        self.script_tokens.contains(&placement.base.id.as_str().to_ascii_lowercase())
    }
}

/// [`TextureDecoder`] for Morrowind textures (DDS, TGA, BMP). Like the
/// original engine, a texture named `.tga`/`.bmp` is looked up as `.dds`
/// first.
pub struct MwTextures;

impl TextureDecoder for MwTextures {
    fn version(&self) -> u32 {
        1
    }

    fn candidates(&self, path: &str) -> Vec<String> {
        let p = path.replace('\\', "/").to_ascii_lowercase();
        let p = if p.starts_with("textures/") { p } else { format!("textures/{p}") };
        match p.rsplit_once('.') {
            Some((stem, ext)) if ext != "dds" => vec![format!("{stem}.dds"), p],
            _ => vec![p],
        }
    }

    fn decode(&self, path: &str, bytes: &[u8]) -> Option<TextureImage> {
        let img = decode_rgba(path, bytes).ok()?;
        Some(TextureImage { width: img.width, height: img.height, rgba: img.rgba })
    }
}

/// Registers the far-field steps on `baker` (which already has the model and
/// world steps) and bakes the far field of `worldspaces`.
pub fn bake_far(baker: &mut Baker, db: &RecordDb, worldspaces: &[WorldspaceId]) -> BakeReport {
    let source = Arc::new(MwFarSource::build(db));
    FarBake::with_textures(source, FarBakeSettings::default(), Arc::new(MwTextures)).register(baker);
    FarBake::bake_worldspaces(baker, worldspaces)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flora_paths_and_texture_candidates() {
        assert!(is_flora("meshes/f/flora_tree_ai_02.nif"));
        assert!(is_flora("meshes/f/Flora_BC_tree_05.nif"));
        assert!(!is_flora("meshes/x/ex_hlaalu_b_01.nif"));
        assert_eq!(MwTextures.candidates("textures/tx_wood.tga"), ["textures/tx_wood.dds", "textures/tx_wood.tga"]);
        assert_eq!(MwTextures.candidates("Tx_Leaf.DDS"), ["textures/tx_leaf.dds"]);
    }

    #[test]
    fn lights_from_records() {
        let mut light = Light::default();
        light.data.radius = 256;
        light.data.color = [255, 128, 0, 0];
        let l = far_light(&light);
        assert!((l.color[0] - 1.0).abs() < 1e-6 && l.color[2] < 1e-6 && l.intensity > 0.0);
        light.data.flags = LightFlags::NEGATIVE;
        assert!(far_light(&light).intensity.abs() < f32::EPSILON);
    }
}

#[cfg(test)]
mod precombine_tests {
    use std::collections::BTreeSet;

    #[test]
    fn script_tokens_find_quoted_and_bare_ids() {
        let mut t = BTreeSet::new();
        super::script_tokens("if ( GetJournalIndex A1 >= 10 )\n\"chargen boat\"->Disable\nex_rock_01->SetPos x 10\nendif", &mut t);
        for id in ["chargen boat", "ex_rock_01", "getjournalindex", "a1"] {
            assert!(t.contains(id), "{id}");
        }
    }
}
