//! Snow and ash cover looks (surface-weather.md "Records", slice W5a): one
//! `sf_surface::cover::SurfaceLayer` record per channel, [`SNOW_LAYER`] and [`ASH_LAYER`], made
//! from the install's own land textures (`LTEX`), and the adapter's pick ([`active_layers`]):
//! those two for every worldspace. Mods patch the records (another texture, tiling, roughness)
//! or define their own and patch the pick's ids in.
//!
//! - **Snow:** Bloodmoon's plain snow, `Tx_BM_snow_01.dds` ([`SNOW_LTEX`] in order); absent
//!   without Bloodmoon, then the first `LTEX` whose texture names snow alone ([`keyword_ltex`]).
//! - **Ash:** the Ashlands' ash ground, `AL_ash_01` (`Tx_ashlands_08.tga`, [`ASH_LTEX`] in
//!   order), same fallback.
//!
//! Morrowind textures carry no height or normal map, and no bake step derives them (the terrain
//! layer arrays use the same flat defaults): the albedo's alpha is the height (opaque: flat, the
//! height blend becomes a coverage threshold), the normal is flat, roughness comes from the
//! record. A derived height (luminance) and normal is a logged gap (surface-weather-as-built.md
//! W5a), not built here.

use sf_records::{RecordDb, RecordId};
use sf_surface::cover::{ActiveLayers, CoverChannel, SurfaceLayer};
use tes3::esp::LandscapeTexture;

use crate::UNIT_METERS;

/// The snow layer record.
pub const SNOW_LAYER: &str = "mw_surface/snow";
/// The ash layer record.
pub const ASH_LAYER: &str = "mw_surface/ash";

/// Preferred snow `LTEX` ids, in order (Bloodmoon).
pub const SNOW_LTEX: &[&str] = &["Tx_BM_snow_01.dds", "Tx_BM_snow_02.dds", "Tx_BM_snow_03.dds", "Tx_BM_snow_04.dds", "Tx_BM_snow_05.dds"];
/// Preferred ash `LTEX` ids, in order (Morrowind's Ashlands).
pub const ASH_LTEX: &[&str] = &["AL_ash_01", "AL_ash_02", "AL_ash_03", "AL_ash_04"];

/// Words that mark a mixed texture (snowy rock, ash road), skipped by the keyword fallback.
const MIXED: &[&str] = &["road", "rock", "grass", "dirt", "ice", "mud", "stone"];

/// Morrowind repeats a land texture once per land texel (512 units ≈ 7.3 m); the cover tiles alike.
const TILING_M: f32 = 512.0 * UNIT_METERS;

/// Full snow cover stands knee deep (owner goal, surface-weather W6b: raised geometry near the
/// camera, feet sink into it, prints carve to the ground). Ash stays a thin crust.
pub const SNOW_DEPTH_M: f32 = 0.45;
/// Full ash cover depth.
pub const ASH_DEPTH_M: f32 = 0.03;

/// The cover layer records from the loaded `LTEX`s (none for a channel without a texture).
pub fn cover_layers(db: &RecordDb) -> Vec<(RecordId, SurfaceLayer)> {
    let mut out = Vec::new();
    if let Some(ltex) = pick_ltex(db, SNOW_LTEX, "snow") {
        let mut layer = SurfaceLayer::new(CoverChannel::Snow, texture_path(&ltex.file_name));
        layer.tiling_m = TILING_M;
        layer.roughness = 0.65;
        layer.sparkle = 1.0;
        layer.max_depth_m = Some(SNOW_DEPTH_M);
        out.push((RecordId::new(SNOW_LAYER), layer));
    }
    if let Some(ltex) = pick_ltex(db, ASH_LTEX, "ash") {
        let mut layer = SurfaceLayer::new(CoverChannel::Ash, texture_path(&ltex.file_name));
        layer.tiling_m = TILING_M;
        layer.roughness = 0.9;
        layer.max_depth_m = Some(ASH_DEPTH_M);
        out.push((RecordId::new(ASH_LAYER), layer));
    }
    out
}

/// The adapter's pick: [`SNOW_LAYER`] and [`ASH_LAYER`] everywhere (a missing record draws no
/// cover look on its channel).
pub fn active_layers() -> ActiveLayers {
    ActiveLayers { default: [Some(RecordId::new(SNOW_LAYER)), Some(RecordId::new(ASH_LAYER))], ..ActiveLayers::default() }
}

/// The first of `preferred` that is loaded, else [`keyword_ltex`].
fn pick_ltex<'a>(db: &'a RecordDb, preferred: &[&str], keyword: &str) -> Option<&'a LandscapeTexture> {
    preferred
        .iter()
        .find_map(|id| db.get::<LandscapeTexture>(&RecordId::new(id)))
        .or_else(|| keyword_ltex(db, keyword))
}

/// The `LTEX` (lowest id) whose texture file names `keyword` and no mixed surface ([`MIXED`]).
pub fn keyword_ltex<'a>(db: &'a RecordDb, keyword: &str) -> Option<&'a LandscapeTexture> {
    let mut hits: Vec<(&RecordId, &LandscapeTexture)> = db
        .iter::<LandscapeTexture>()
        .filter(|(_, l)| {
            let file = l.file_name.to_ascii_lowercase();
            file.contains(keyword) && !MIXED.iter().any(|m| file.contains(m))
        })
        .collect();
    hits.sort_by(|a, b| a.0.cmp(b.0));
    hits.first().map(|(_, l)| *l)
}

/// A texture reference as a VFS path (`mw_nif::texture_path`'s rule: `textures/`, lowercase,
/// forward slashes). The reader resolves `.dds` over `.tga` as the original engine does.
fn texture_path(raw: &str) -> String {
    let normalized = raw.trim().replace('\\', "/").to_ascii_lowercase();
    let normalized = normalized.trim_start_matches('/');
    if normalized.starts_with("textures/") { normalized.to_owned() } else { format!("textures/{normalized}") }
}
