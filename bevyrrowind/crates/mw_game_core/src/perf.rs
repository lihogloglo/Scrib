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
    /// Bloom on the game camera (Bevy `Bloom`, energy-conserving; Medium and up).
    Bloom,
    /// Dynamic probe GI (`sf_gi_bevy`, gi.md; High and Ultra raster, interiors in slice 1).
    Gi,
    /// Cached point-light shadow maps (`sf_shadow_cache_bevy`; off = the patched Bevy path, perf.md "Shadow reset B1").
    ShadowCache,
    /// Time-sliced sky environment map (`sf_sky_bevy::EnvMapRefresh`): off, Bevy re-filters it every frame.
    EnvSlice,
    Clouds,
    /// Size-based draw distance (industry-audit.md J1).
    SizeCull,
    /// Actor placements (NPCs, creatures) as characters.
    Actors,
    /// Characters past the crowd `C0` threshold (below 20 reference px, ~93 m for a 1.8 m figure; back
    /// above 23 px, ~81 m) hide at their root, animation and gameplay running: the crowd-LOD ceiling
    /// A/B (superfargoth crowd-lod.md CROWD-1, `sf_character_bevy::CrowdSettings::hide_far`).
    FarNpcs,
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
    /// The depth-aware ground-fog bank (FOG-BANK, FG-7): the cloud trace records the bank at the
    /// nearest and furthest depth of its window and the composite interpolates by each pixel's
    /// depth; off, the single furthest-depth value picked by population (the old path, an A/B).
    FogBank,
    /// The froxels' history clip (FOG-CLAMP, WE-10): reprojected history clipped to this frame's 3x3
    /// neighbourhood, history weight 0.95; off, unclipped at 0.9 (the old path, an A/B).
    FogClamp,
    /// The fog split at the clouds' depth (WE12-FIX, WE-12, `sf_atmos_bevy::FogCloudSplit`): only the
    /// fog in front of the deck dims it, the far weather ordered by distance; off, the whole fog
    /// column and the far weather in front of the clouds (the old order, an A/B).
    CloudFog,
    /// The far storm's shape (CW-1, WE-13, `sf_atmos_bevy::FarStormShape`): distant rain under the
    /// clouds' thick cores, a narrow noisy region edge, the far medium integrated analytically; off,
    /// the `WE4d` far weather (the region column, 8 jittered steps; the old path, an A/B).
    StormShape,
    /// Stable cloud silhouettes (S1, WE-14, `sf_atmos_bevy::CloudEdgeStable`): the cloud trace's depth
    /// window over the whole checkerboard block, the resolve's same-population neighbour fallback and
    /// accumulated cloud distance; off, the window on the traced pixel and the raw sample (an A/B).
    CloudEdge,
    /// The storm cloud shape (S3, WE-15, `sf_atmos_bevy::CumulonimbusShape`): in a storm the shape noise
    /// in world altitude, tops that follow coverage, an anvil, a ragged base, cells shrinking by
    /// threshold at the region edge; off, the old storm profile (the extruded footprint, an A/B).
    CbShape,
    /// Screen-space planar reflections on the water (W-SSPR, `sf_water_bevy::WaterReflections`): off,
    /// the environment map only (the old look, an A/B).
    WaterSsr,
    /// Wind wavelets in the water's shading normal up to the shoreline (W-SHORE,
    /// `sf_water::ShoreSettings::normal_floor`): off, the normal flattens with the displacement at the
    /// shore (the old look, an A/B).
    ShoreNormals,
    /// Analytic Gerstner shore waves and swash where the FFT swell fades (W-GERSTNER,
    /// `sf_water::ShoreSettings::shore_waves`): off, the FFT alone fades to flat at the shore (the old
    /// look, an A/B).
    ShoreWaves,
    /// The sea follows the weather's wind (W-WIND, `sf_water::SeaWind`): off, the ocean keeps the fixed
    /// 7 m/s sea breeze (the old look, an A/B).
    SeaWind,
    /// The shore waves shoal over the baked bed slope from `LAND` (W-GERSTNER-DEPTH,
    /// `sf_water::ShoreField::bed_slope`): off, they assume the uniform 1:20 beach (the old look, an A/B).
    ShoreDepth,
    /// Breaker foam (SF-1, `sf_water::SurfFoam`): bore foam born where the shore waves break and
    /// trailing their crests, swash and contact lace, foam look by age; off, the WA-6 shore band and
    /// the constant contact foam (the old look, an A/B).
    SurfFoam,
    /// Surf shape (SF-2, `sf_water::SurfShape`): the shore waves stand up taller (`surf_height_gain`,
    /// tunable `breaker_gamma`), skew to a steep front through a shoaling/steepening/bore lifecycle and
    /// break in patches; off, the symmetric physically capped Gerstner crests (the old look, an A/B).
    SurfSkew,
    /// The underwater post (U3, `sf_water_bevy` `underwater_pass`): on a camera fully under the water,
    /// spectral extinction and in-scatter by depth, with the sky, aerial perspective and fog composite
    /// skipped; off, the camera's medium is still tracked but the frame is drawn as above the water
    /// (the old look, an A/B; the tunable `water.underwater` stays the master switch).
    Underwater,
    /// The weather's sea state (W-SEASTATE, `mw_water::sea_state`): the weather's wind goes through the
    /// authored sea state curve to the spectrum, on the `water.sea_fetch_m` fetch; off, the wind is
    /// copied verbatim on the classic 20 km fetch (the old look, an A/B).
    SeaState,
    /// Rain rings on the water (R1, `sf_water_bevy::WaterRain`): the puddles' drop rings on every
    /// outdoor water surface while it rains, by the weather's rain; off, the water ignores the rain
    /// (the old look, an A/B).
    WaterRain,
    /// Fog on blended and additive NIF materials (`sf_atmos_bevy::FogTranslucentsPlugin`: they draw
    /// after the fog composite); off, they skip the fog (the old look, an A/B).
    TransFog,
    /// Foliage cutouts (`mw_nif::Cutout`): off, authored blend +
    /// "alpha > 0" surfaces draw blended as before (the old look, an A/B).
    LeafMask,
    /// Diffuse transmission on leaf cutouts (`mw_assets::LEAF_DIFFUSE_TRANSMISSION`): off, leaves are
    /// opaque cards as before (an A/B).
    FoliageTransmission,
    /// Camera motion vectors for sky and cloud pixels (`sf_render_order_bevy::SkyMotion`, RO-7); off,
    /// they carry none and DLSS/TAA smear the sky on camera turns (the old look, an A/B).
    SkyMotion,
    /// Near cells drawn from their precombines (superfargoth precombine.md); off, one entity per
    /// static placement, LOD level and part (the old way, an A/B; no rebake needed).
    Precombine,
    /// Material classes for statics (superfargoth static-scene.md S2): template materials fold into
    /// their class (one pipeline key per class) and images share equal samplers (fuller bindless
    /// slabs); off, the mapper's materials as they come and one sampler per image (an A/B; the look
    /// should not change).
    MatClass,
    /// With matclass: statics without a normal map bind one shared 1x1 flat normal map, so normal-mapped
    /// and plain statics of a class share one main-pass key (superfargoth `matclass::FLAT_NORMAL_MAP`);
    /// off, a normal map splits its class (an A/B; the look should not change).
    FlatNormal,
    /// Bench only, reversed: `--off texvariety` turns ON superfargoth's `FoldRules::one_texture_set`, every
    /// image slot of a static material binds one shared image per slot kind, so the bindless material slab
    /// holds a handful of textures while keys, pipelines, batch sets and draws stay the same (the ceiling of
    /// the per-texture binding cost). The look is wrong by design; needs matclass on.
    TexVariety,
    /// Mesh slabs capped at 2 GiB instead of 512 MiB (superfargoth `mesh_slabs::MeshSlabsPlugin`, ROADMAP 1.22
    /// F4), so a vertex layout never spills into a second slab (which splits its batch sets); off, stock
    /// `MeshAllocatorSettings` (an A/B; the look should not change). Log lines `mesh.slabs:` either way.
    MeshSlabs,
    /// Near statics skip Bevy's CPU culling (`NoCpuCulling`, superfargoth
    /// `docs/research/statics-cpu-cost-2026-11-12.md` slice 1): the GPU preprocess culls them by frustum
    /// and range in every view, lamps take them from the shadow cache's lists. Off: the old behaviour,
    /// every static re-culled on the CPU per view each frame (an A/B; the look should not change).
    CpuCullTag,
    /// Point/spot (lamp) shadows filtered with 2x2 hardware PCF while the sun keeps the preset's filter
    /// (vendored `bevy_pbr::PointSpotShadowsHardware2x2`, shadow reset B2); off, lamps use the camera's
    /// filter as stock Bevy does (an A/B).
    LampFilter,
    /// The precipitation occlusion map (`sf_precip_bevy`, WE-5): off, drops fall and blow through roofs,
    /// walls and hills (an A/B).
    PrecipOcc,
    /// Lamp shadow faces redraw the frame their casters move or deform (vendored
    /// `bevy_pbr::ShadowMoverRefresh`, SH-20); off, the B3 rule: skinned casters by root on a coarse
    /// grid, at most 2 dirty faces per frame (animated shadows step; an A/B).
    MoverShadows,
    /// One command encoder per sun cascade (vendored `bevy_pbr::ShadowCascadeEncoders`, ROADMAP 1.22 F1):
    /// their encode finishes as parallel tasks; off, all cascades of a camera share one encoder as stock
    /// Bevy (an A/B; the look should not change). Opt-in since 2026-11-16 (`BEVYRROWIND_CASCADE_ENCODERS=1`, no
    /// measured win with the far-cascade hold on); this part still forces it off.
    CascadeEncoders,
    /// Far sun cascades keep their fit and shadow map between redraws (`sf_sky_bevy::SunCascadeHold`,
    /// ROADMAP 1.22 F3): redrawn after a 4 m move, a 3 degree turn or 4 frames; off, every cascade is
    /// fitted and drawn every frame as stock Bevy (an A/B).
    CascadeHold,
    /// De-lit albedo textures (the bake's `texture.delight` VFS layer, bake.md "De-lighting"): off, the layer
    /// isn't mounted and models load the original textures (an A/B; restart to switch).
    Delight,
    /// Normal maps from texture-pack companions on every material (`mw_assets::MaterialMaps`, TX-lab):
    /// off, materials drop `normal_map_texture` (an A/B; console `flip normalmaps` switches it live).
    NormalMaps,
    /// ORM maps (`_spec` companions, TS-2) on every material: off, no metallic-roughness or occlusion
    /// texture and the 2002 factors (an A/B; console `flip orm`).
    Orm,
    /// Parallax occlusion from `_nh` height companions (`depth_map`, tx-pom) on every material: off,
    /// no depth map and Bevy's default parallax fields (an A/B; console `flip parallax`).
    Parallax,
    /// Surface weather on lit materials (`sf_surface_bevy`, vendored `bevy_pbr` patch "SW"): off, the
    /// hook stays compiled in with its flags cleared (an A/B: wet vs dry).
    Surface,
    /// Wetness alone (surface-weather.md W3): off, the ground and walls stay dry after rain (an A/B).
    Wet,
    /// Puddles alone (surface-weather.md W4): off, no standing water or rain rings (an A/B; the
    /// Low preset runs without them).
    Puddles,
    /// The surface shelter map: off, roofs and trees don't keep the ground dry (a diagnostic; the Low
    /// preset runs without it).
    Shelter,
    /// Snow and ash cover (surface-weather.md W5): off, the ground keeps its own look under any
    /// snow or ash amount (an A/B).
    Cover,
    /// Trails in snow and ash (surface-weather.md W6): off, feet leave no prints (an A/B).
    Trails,
    /// Kick-up in snow and ash (surface-weather.md W7): off, feet throw no puffs (an A/B).
    Kickup,
    /// The raised cover (surface-weather.md W6b, SW-5c): off, snow and ash lie flat (shading only;
    /// an A/B, live as the `raise` switch).
    Raise,
    /// The static scene (superfargoth scene-runtime-plan.md §8): with `--scene`, near statics draw from
    /// the baked cell tables (`sf_scene_bevy`) instead of the precombine; off, the precombine draws
    /// them again (an A/B).
    Scene,
    /// Floating-origin shifts rebase motion vectors (FLICKER-1, ST-2, `bevy_render::view::OriginRebase`):
    /// temporal history (TAA/DLSS) survives each 256 m cell crossing; off, every shift resets it
    /// (the old periodic pop, an A/B).
    ShiftRebase,
    /// Selection gates (CW-2, `sf_world::select_gate`): streaming, terrain and far-field selection re-run
    /// only when the camera moved or turned past a quantum or an input changed. Opt-in until its A/B
    /// (tunable `select.gate`, alias `BEVYRROWIND_SELECT_GATE=1`); this part forces it off at the command-line layer.
    SelectGate,
}

impl Part {
    const ALL: [(&'static str, Part); 85] = [
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
        ("bloom", Part::Bloom),
        ("gi", Part::Gi),
        ("shadowcache", Part::ShadowCache),
        ("envslice", Part::EnvSlice),

        ("clouds", Part::Clouds),
        ("sizecull", Part::SizeCull),
        ("actors", Part::Actors),
        ("npcs", Part::Actors),
        ("farnpcs", Part::FarNpcs),
        ("walkers", Part::Walkers),
        ("ai", Part::Walkers),
        ("physics", Part::Physics),
        ("fog", Part::Fog),
        ("froxels", Part::Froxels),
        ("godrays", Part::Froxels),
        ("fogdither", Part::FogDither),
        ("fogjitter", Part::FogJitter),
        ("foghistory", Part::FogHistory),
        ("fogbank", Part::FogBank),
        ("fogclamp", Part::FogClamp),
        ("cloudfog", Part::CloudFog),
        ("stormshape", Part::StormShape),
        ("cloudedge", Part::CloudEdge),
        ("cbshape", Part::CbShape),
        ("waterssr", Part::WaterSsr),
        ("shorenormals", Part::ShoreNormals),
        ("shorewaves", Part::ShoreWaves),
        ("seawind", Part::SeaWind),
        ("shoredepth", Part::ShoreDepth),
        ("surffoam", Part::SurfFoam),
        ("surfskew", Part::SurfSkew),
        ("underwater", Part::Underwater),
        ("seastate", Part::SeaState),
        ("water_rain", Part::WaterRain),
        ("waterrain", Part::WaterRain),
        ("transfog", Part::TransFog),
        ("leafmask", Part::LeafMask),
        ("foliagetransmission", Part::FoliageTransmission),
        ("skymotion", Part::SkyMotion),
        ("precombine", Part::Precombine),
        ("matclass", Part::MatClass),
        ("flatnormal", Part::FlatNormal),
        ("texvariety", Part::TexVariety),
        ("meshslabs", Part::MeshSlabs),
        ("cpuculltag", Part::CpuCullTag),
        ("lampfilter", Part::LampFilter),
        ("movershadows", Part::MoverShadows),
        ("cascadeencoders", Part::CascadeEncoders),
        ("cascadehold", Part::CascadeHold),
        ("precipocc", Part::PrecipOcc),
        ("delight", Part::Delight),
        ("normalmaps", Part::NormalMaps),
        ("orm", Part::Orm),
        ("parallax", Part::Parallax),
        ("surface", Part::Surface),
        ("wet", Part::Wet),
        ("puddles", Part::Puddles),
        ("shelter", Part::Shelter),
        ("cover", Part::Cover),
        ("trails", Part::Trails),
        ("kickup", Part::Kickup),
        ("raise", Part::Raise),
        ("scene", Part::Scene),
        ("shiftrebase", Part::ShiftRebase),
        ("selectgate", Part::SelectGate),
    ];

    pub fn name(self) -> &'static str {
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
        assert!(Off::from_flags(&["farnpcs".into()]).expect("known").has(Part::FarNpcs));
        assert!(Off::from_flags(&["nope".into()]).is_err());
        let fog = Off::from_flags(&["fogbank".into()]).expect("known");
        assert!(fog.has(Part::FogBank) && !fog.has(Part::FogHistory) && !fog.has(Part::Fog));
        assert_eq!(fog.label(), "fogbank");
        let clamp = Off::from_flags(&["fogclamp".into()]).expect("known");
        assert!(clamp.has(Part::FogClamp) && !clamp.has(Part::FogHistory) && !clamp.has(Part::FogBank));
        assert_eq!(clamp.label(), "fogclamp");
        let ssr = Off::from_flags(&["waterssr".into()]).expect("known");
        assert!(ssr.has(Part::WaterSsr) && !ssr.has(Part::FogClamp));
        assert_eq!(ssr.label(), "waterssr");
        let shore = Off::from_flags(&["shorenormals".into()]).expect("known");
        assert!(shore.has(Part::ShoreNormals) && !shore.has(Part::WaterSsr));
        assert_eq!(shore.label(), "shorenormals");
        // SW-5c: `--off raise` leaves the raise alone off, not the cover or its trails.
        let raise = Off::from_flags(&["raise".into()]).expect("known");
        assert!(raise.has(Part::Raise) && !raise.has(Part::Cover) && !raise.has(Part::Trails));
        assert_eq!(raise.label(), "raise");
        let waves = Off::from_flags(&["shorewaves".into()]).expect("known");
        assert!(waves.has(Part::ShoreWaves) && !waves.has(Part::ShoreNormals));
        assert_eq!(waves.label(), "shorewaves");
        let cloud = Off::from_flags(&["cloudfog".into()]).expect("known");
        assert!(cloud.has(Part::CloudFog) && !cloud.has(Part::FogBank) && !cloud.has(Part::Fog));
        assert_eq!(cloud.label(), "cloudfog");
        let storm = Off::from_flags(&["stormshape".into()]).expect("known");
        assert!(storm.has(Part::StormShape) && !storm.has(Part::CloudFog));
        assert_eq!(storm.label(), "stormshape");
        let edge = Off::from_flags(&["cloudedge".into()]).expect("known");
        assert!(edge.has(Part::CloudEdge) && !edge.has(Part::CloudFog) && !edge.has(Part::StormShape));
        assert_eq!(edge.label(), "cloudedge");
        let cb = Off::from_flags(&["cbshape".into()]).expect("known");
        assert!(cb.has(Part::CbShape) && !cb.has(Part::StormShape) && !cb.has(Part::CloudEdge));
        assert_eq!(cb.label(), "cbshape");
        let shift = Off::from_flags(&["shiftrebase".into()]).expect("known");
        assert!(shift.has(Part::ShiftRebase) && !shift.has(Part::Scene));
        assert_eq!(shift.label(), "shiftrebase");
        let gate = Off::from_flags(&["selectgate".into()]).expect("known");
        assert!(gate.has(Part::SelectGate) && !gate.has(Part::Terrain) && !gate.has(Part::FarField));
        assert_eq!(gate.label(), "selectgate");
        let enc = Off::from_flags(&["cascadeencoders".into()]).expect("known");
        assert!(enc.has(Part::CascadeEncoders) && !enc.has(Part::MoverShadows) && !enc.has(Part::Shadows));
        assert_eq!(enc.label(), "cascadeencoders");
        let hold = Off::from_flags(&["cascadehold".into()]).expect("known");
        assert!(hold.has(Part::CascadeHold) && !hold.has(Part::CascadeEncoders) && !hold.has(Part::Shadows));
        assert_eq!(hold.label(), "cascadehold");
        let sea = Off::from_flags(&["seawind".into()]).expect("known");
        assert!(sea.has(Part::SeaWind) && !sea.has(Part::ShoreWaves));
        assert_eq!(sea.label(), "seawind");
        let depth = Off::from_flags(&["shoredepth".into()]).expect("known");
        assert!(depth.has(Part::ShoreDepth) && !depth.has(Part::ShoreWaves) && !depth.has(Part::SeaWind));
        assert_eq!(depth.label(), "shoredepth");
        let surf = Off::from_flags(&["surffoam".into()]).expect("known");
        assert!(surf.has(Part::SurfFoam) && !surf.has(Part::ShoreWaves));
        let state = Off::from_flags(&["seastate".into()]).expect("known");
        assert!(state.has(Part::SeaState) && !state.has(Part::SeaWind));
        assert_eq!(state.label(), "seastate");
        assert_eq!(surf.label(), "surffoam");
        let skew = Off::from_flags(&["surfskew".into()]).expect("known");
        assert!(skew.has(Part::SurfSkew) && !skew.has(Part::SurfFoam) && !skew.has(Part::ShoreWaves));
        assert_eq!(skew.label(), "surfskew");
        let under = Off::from_flags(&["underwater".into()]).expect("known");
        assert!(under.has(Part::Underwater) && !under.has(Part::SurfFoam) && !under.has(Part::ShoreWaves));
        assert_eq!(under.label(), "underwater");
        let rain = Off::from_flags(&["water_rain".into()]).expect("known");
        assert!(rain.has(Part::WaterRain) && !rain.has(Part::Puddles) && !rain.has(Part::ShoreWaves));
        assert_eq!(rain.label(), "water_rain");
        assert_eq!(Off::from_flags(&["waterrain".into()]).expect("alias"), rain);
        let maps = Off::from_flags(&["normalmaps".into(), "ORM".into()]).expect("known");
        assert!(maps.has(Part::NormalMaps) && maps.has(Part::Orm) && !maps.has(Part::Delight));
        assert_eq!(maps.label(), "normalmaps,orm");
        let parallax = Off::from_flags(&["parallax".into()]).expect("known");
        assert!(parallax.has(Part::Parallax) && !parallax.has(Part::NormalMaps));
        assert_eq!(parallax.label(), "parallax");
    }
}
