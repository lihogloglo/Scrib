//! Performance A/B switches (`--off`): which parts of the frame a run leaves out. The app's
//! `perf` module (benchmarks, suite runner) re-exports these.

use bevy_ecs::resource::Resource;

/// Environment variable read as the default for `--off`.
const ENV_OFF: &str = "BEVYRROWIND_OFF";

/// A part of the frame an A/B run can leave out.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Part {
    Terrain,
    Near,
    Far,
    Sky,
    Envmap,
    Shadows,
    Lights,
    Particles,
    Nif,
    FarField,
    Denoiser,
    /// The terrain's far shadow past the cascades (horizon march + far map; also on proxies and impostors).
    FarShadows,
    /// Soft contact-hardening sun shadows (shadows F5, Bevy PCSS).
    SoftShadows,
    /// Screen-space contact shadows (shadows F1).
    ContactShadows,
    /// Screen-space ambient occlusion (Bevy GTAO, ROADMAP 1.5; Medium and up).
    Ssao,
    /// Cached point-light shadow maps (`sf_shadow_cache_bevy`; off = the patched Bevy path, perf.md "Shadow reset B1").
    ShadowCache,
    /// Time-sliced sky environment map (`sf_sky_bevy::EnvMapRefresh`): off, Bevy re-filters it every frame.
    EnvSlice,
    Clouds,
    /// Size-based draw distance (industry-audit.md J1).
    SizeCull,
    /// Actor placements (NPCs, creatures) as characters.
    Actors,
    /// NPCs walking (controller, avoidance, wander scaffold; `walkers.rs`).
    Walkers,
    /// Rigid-body physics (`physics.rs`, M4): no physics worlds, no colliders.
    Physics,
    /// Height fog, ground-fog banks and the far fog (weather.md "Fog", F2).
    Fog,
    /// The froxel volume near the camera (god rays, lit fog; weather.md "Fog", F3):
    /// off, the analytic layers fog everything (the Low preset).
    Froxels,
    /// Per-froxel sample rotation (weather.md "Temporal sampling"): off, one Halton
    /// offset for the whole volume, history read at the jittered point (the old
    /// pulsing fog, an A/B).
    FogDither,
    /// The froxels' temporal jitter: off, every froxel samples its centre (diagnosis).
    FogJitter,
    /// The froxels' temporal history (reprojected last frame): off, each frame stands
    /// alone (FG-1 diagnosis: bands on fast turns that vanish here are the history's).
    FogHistory,
    /// Fog on blended and additive NIF materials (`sf_atmos_bevy::FogTranslucentsPlugin`: they draw
    /// after the fog composite); off, they skip the fog (the old look, an A/B).
    TransFog,
    /// Foliage cutouts (`mw_nif::Cutout`): off, authored blend +
    /// "alpha > 0" surfaces draw blended as before (the old look, an A/B).
    LeafMask,
    /// Camera motion vectors for sky and cloud pixels (`sf_render_order_bevy::SkyMotion`, RO-7); off,
    /// they carry none and DLSS/TAA smear the sky on camera turns (the old look, an A/B).
    SkyMotion,
    /// Near cells drawn from their precombines (superfargoth precombine.md); off, one entity per
    /// static placement, LOD level and part (the old way, an A/B; no rebake needed).
    Precombine,
    /// Point/spot (lamp) shadows filtered with 2x2 hardware PCF while the sun keeps the preset's filter
    /// (vendored `bevy_pbr::PointSpotShadowsHardware2x2`, shadow reset B2); off, lamps use the camera's
    /// filter as stock Bevy does (an A/B).
    LampFilter,
    /// The precipitation occlusion map (`sf_precip_bevy`, WE-5): off, drops fall and blow through roofs,
    /// walls and hills (an A/B).
    PrecipOcc,
}

impl Part {
    const ALL: [(&'static str, Part); 41] = [
        ("terrain", Part::Terrain),
        ("near", Part::Near),
        ("statics", Part::Near),
        ("r1", Part::Near),
        ("far", Part::Far),
        ("r2", Part::Far),
        ("sky", Part::Sky),
        ("atmosphere", Part::Sky),
        ("envmap", Part::Envmap),
        ("shadows", Part::Shadows),
        ("lights", Part::Lights),
        ("particles", Part::Particles),
        ("nif", Part::Nif),
        ("farfield", Part::FarField),
        ("lod", Part::FarField),
        ("denoiser", Part::Denoiser),
        ("farshadows", Part::FarShadows),
        ("softshadows", Part::SoftShadows),
        ("contactshadows", Part::ContactShadows),
        ("ssao", Part::Ssao),
        ("shadowcache", Part::ShadowCache),
        ("envslice", Part::EnvSlice),

        ("clouds", Part::Clouds),
        ("sizecull", Part::SizeCull),
        ("actors", Part::Actors),
        ("npcs", Part::Actors),
        ("walkers", Part::Walkers),
        ("ai", Part::Walkers),
        ("physics", Part::Physics),
        ("fog", Part::Fog),
        ("froxels", Part::Froxels),
        ("godrays", Part::Froxels),
        ("fogdither", Part::FogDither),
        ("fogjitter", Part::FogJitter),
        ("foghistory", Part::FogHistory),
        ("transfog", Part::TransFog),
        ("leafmask", Part::LeafMask),
        ("skymotion", Part::SkyMotion),
        ("precombine", Part::Precombine),
        ("lampfilter", Part::LampFilter),
        ("precipocc", Part::PrecipOcc),
    ];

    fn name(self) -> &'static str {
        Self::ALL.iter().find(|(_, p)| *p == self).map_or("?", |(n, _)| n)
    }
}

/// The parts left out of this run.
#[derive(Resource, Clone, Debug, Default, PartialEq, Eq)]
pub struct Off(Vec<Part>);

impl Off {
    /// From `--off` and, when that is empty, `BEVYRROWIND_OFF`.
    pub fn from_flags(flags: &[String]) -> Result<Self, String> {
        let env = std::env::var(ENV_OFF).unwrap_or_default();
        let names: Vec<&str> = if flags.is_empty() {
            env.split(',').collect()
        } else {
            flags.iter().map(String::as_str).collect()
        };
        let mut parts = Vec::new();
        for name in names.into_iter().map(str::trim).filter(|n| !n.is_empty()) {
            let part = Part::ALL
                .iter()
                .find(|(n, _)| n.eq_ignore_ascii_case(name))
                .map(|(_, p)| *p)
                .ok_or_else(|| {
                    let known: Vec<&str> = Part::ALL.iter().map(|(n, _)| *n).collect();
                    format!("unknown --off part {name:?}; known: {}", known.join(", "))
                })?;
            if !parts.contains(&part) {
                parts.push(part);
            }
        }
        parts.sort();
        Ok(Self(parts))
    }

    pub fn has(&self, part: Part) -> bool {
        self.0.contains(&part)
    }

    /// `terrain,shadows`, or `none`.
    pub fn label(&self) -> String {
        if self.0.is_empty() {
            "none".to_owned()
        } else {
            self.0.iter().map(|p| p.name()).collect::<Vec<_>>().join(",")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn off_parses_aliases_dedups_and_labels() {
        let off = Off::from_flags(&["statics".into(), "Shadows".into(), "near".into()]).expect("known parts");
        assert!(off.has(Part::Near) && off.has(Part::Shadows) && !off.has(Part::Sky));
        assert_eq!(off.label(), "near,shadows");
        assert!(Off::from_flags(&["nope".into()]).is_err());
    }
}
