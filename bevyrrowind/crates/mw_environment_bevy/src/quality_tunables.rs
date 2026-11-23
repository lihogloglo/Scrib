//! Quality values the `Quality` preset decides, as live tunables (tunables slice T5a, spec
//! `superfargoth/docs/specs/tunables.md` "Migration"), declared here because their framework crates
//! don't take `sf_tunables` yet (`sf_sky_bevy` has no `bevy_post_process`; `sf_atmos`, `sf_water_bevy`,
//! `sf_surface_bevy`, `sf_texture_bevy` none) or are the game's start-time choices:
//!
//! - `render.bloom`, `render.bloom.intensity`: Bevy's `Bloom` on the managed views
//!   (`sf_sky_bevy::ViewQualityCamera`), inserted, retuned and removed live ([`BloomPlugin`]);
//! - `sky.clouds.map_per_sample`, `surface.puddles`, `surface.shelter` (live, bound by the exterior),
//!   `render.gi.probes` and `texture.max_size_px` (read at start);
//! - `water.quality`: the water's FFT tier (`sf_water_bevy::WaterQuality`, which re-sizes its wave
//!   images when it changes), bound by `water::add`.
//!
//! The exterior's quality table (`mw_exterior_bevy::quality_presets`) writes their `Preset` layer.

use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::*;
use bevy_post_process::bloom::Bloom;
use sf_sky_bevy::ViewQualityCamera;
use sf_tunables_bevy::sf_tunables::{AnyDef, Cost, Def, Effect, FeatureSwitch, Group, Id, Lifetime, Source, Unit, def};
use sf_tunables_bevy::{Tunables, TunablesAppExt};

/// `render.bloom`: Bevy's `Bloom` (`Bloom::NATURAL`: COD-AW dual filter, energy-conserving, no
/// threshold) on the managed views; a post-process node, live.
pub const BLOOM: Def<bool> = def!(bool "render.bloom" = false)
    .help("Bloom on the camera (Bevy Bloom::NATURAL, energy-conserving)")
    .source(Source::BevyDefault("bevy_post_process::bloom::Bloom (opt-in per camera)"))
    .group(Group::Post);

/// The feature switch on [`BLOOM`] (slice FS-2b, registered by [`BloomPlugin`]): off removes `Bloom`
/// from the views, so Bevy's bloom node skips them.
pub const BLOOM_SWITCH: FeatureSwitch = FeatureSwitch::new(&BLOOM)
    .label("Bloom")
    .effect(Effect::Skips)
    .cost(&[Cost::Gpu("bloom")])
    .at(file!(), line!());

/// `render.bloom.intensity`: `Bloom::intensity`, the scatter fraction.
pub const BLOOM_INTENSITY: Def<f32> = def!(float "render.bloom.intensity" = 0.15)
    .range(0.0, 1.0)
    .unit(Unit::Ratio)
    .help("Bloom scatter fraction (Bevy Bloom::intensity)")
    .source(Source::BevyDefault("bevy_post_process::bloom::Bloom::NATURAL.intensity"))
    .group(Group::Post);

/// `water.quality`: the FFT tier (grid, cascades, amortised longest cascade, SSPR scale).
pub const WATER_QUALITY: Def<&'static str> = def!(choice "water.quality" = "high")
    .choices(&["low", "medium", "high", "ultra"])
    .help("Water FFT tier: wave grid, cascades, the amortised longest cascade and the reflection scale")
    .source(Source::BevyDefault("sf_water_bevy::WaterQuality::default (High)"))
    .group(Group::Water);

/// `sky.clouds.map_per_sample`: the clouds read the world weather map's layer at every density
/// sample (`sf_atmos::CloudQuality::map_per_sample`; with the distant precipitation shafts) or 3
/// times per ray, lerped. A per-frame uniform, live. Declared game-side until `sf_atmos` takes
/// `sf_tunables` (T8a).
pub const CLOUD_MAP_PER_SAMPLE: Def<bool> = def!(bool "sky.clouds.map_per_sample" = true)
    .help("Clouds sample the world weather map at every density sample (sharp region edges, far shafts) or 3 times per ray")
    .source(Source::BevyDefault("sf_atmos::CloudQuality::HIGH.map_per_sample"))
    .group(Group::Sky);

/// `surface.puddles`: standing water and rain rings (`sf_surface_bevy::SurfaceSettings::puddles`), live.
pub const PUDDLES: Def<bool> = def!(bool "surface.puddles" = true)
    .help("Puddles: standing water and rain rings on wet ground")
    .source(Source::BevyDefault("sf_surface_bevy::SurfaceSettings::default (on)"))
    .group(Group::Effects);

/// The feature switch on [`PUDDLES`] (slice FS-2b, registered where the exterior binds the tunable).
/// Off clears the puddle bit of the surface layer's flags (`SurfaceSettings::flag_mask`): the shader
/// skips that branch, but the layer's passes run as before, so it only hides.
pub const PUDDLES_SWITCH: FeatureSwitch = FeatureSwitch::new(&PUDDLES)
    .label("Puddles")
    .effect(Effect::Hides)
    .at(file!(), line!());

/// `surface.shelter`: the shelter map (roofs and canopies keep the ground under them dry), live.
pub const SHELTER: Def<bool> = def!(bool "surface.shelter" = true)
    .help("Shelter map: roofs, arches and canopies keep the ground under them dry")
    .source(Source::BevyDefault("sf_surface_bevy::SurfaceSettings::default (on)"))
    .group(Group::Effects);

/// The feature switch on [`SHELTER`] (slice FS-2b, registered where the exterior binds the tunable).
/// Off stops the shelter map's redraws (`sf_surface_bevy::shelter::update_shelter`) and its exposure pass.
pub const SHELTER_SWITCH: FeatureSwitch = FeatureSwitch::new(&SHELTER)
    .label("Shelter (dry ground under roofs)")
    .effect(Effect::Skips)
    .cost(&[Cost::Cpu("sf_surface_bevy::shelter::update_shelter"), Cost::Gpu("surface_exposure")])
    .at(file!(), line!());

/// `render.gi.probes`: the dynamic probe GI plugin in this run (interiors, raster path). The plugin is
/// added at start (`Restart`); the console's `render.gi` turns it off and on live where it runs.
pub const GI_PROBES: Def<bool> = def!(bool "render.gi.probes" = false)
    .help("Dynamic probe GI in interiors (raster, ray-query GPUs); read at start, `render.gi` toggles it live where it runs")
    .source(Source::Owner("gi.md slice 1: High and Ultra"))
    .group(Group::Lighting)
    .lifetime(Lifetime::Restart);

/// `texture.max_size_px`: the load-time texture size cap, largest side of level 0 in texels
/// (`sf_texture_bevy::TextureCapPlugin`, and the terrain layer arrays); 0: none. Read at start: the
/// loader is built with it, and a live change would reload every texture.
pub const TEXTURE_MAX_SIZE: Def<u32> = def!(count "texture.max_size_px" = 0)
    .range(0, 16384)
    .unit(Unit::Texels)
    .help("Texture size cap at load, largest side of level 0 (0: none); read at start")
    .source(Source::Industry("UE texture-group MaxLODSize (textures.md §6 \"Pool per preset\")"))
    .group(Group::Textures)
    .lifetime(Lifetime::Restart);

/// `render.depth_only.skip_material`: `bevy_pbr::DepthOnlyMaterialSettings::skip_material` (vendored
/// `material.rs`, own patch "depth-only no material"): opaque `StandardMaterial` draws in shadow views
/// and in a prepass with depth only bind no material group (Bevy's stock material-less depth-only
/// path). Read at start (`Restart`): Bevy reads it once when its material plugin finishes, and the
/// pipelines and draw functions of a run must all agree.
pub const DEPTH_ONLY_SKIP_MATERIAL: Def<bool> = def!(bool "render.depth_only.skip_material" = true)
    .help("Opaque shadow and depth-only draws bind no material group (alpha-masked ones keep it); read at start")
    .source(Source::Owner("2026-11-17: on by default after the owner's play session with every gate on; A/B docs/ledger/perf.md 2026-11-17"))
    .group(Group::Shadows)
    .lifetime(Lifetime::Restart);

/// `render.wgpu.bind_group_walk_once`: the vendored wgpu-core switch (own patch "bind group walk once",
/// `wgpu_core::command::bind_group_walk`, reached through `bevy_render`'s `encode_timing`): a render pass
/// merges a bind group into its usage scope and registers its memory-init actions once, not again on
/// every re-set or rebind (the bindless material slab holds thousands of textures). Read at start. On by
/// default (owner 2026-11-17, after the interleaved A/B); `--set render.wgpu.bind_group_walk_once=off` is
/// stock wgpu-core.
pub const BIND_GROUP_WALK_ONCE: Def<bool> = def!(bool "render.wgpu.bind_group_walk_once" = true)
    .help("A render pass walks each bind group's textures once, not on every rebind (vendored wgpu-core); read at start")
    .source(Source::Measured("perf 2026-11-17: walk -1.15 ms frame, -1.37 ms render thread; on by default, owner 2026-11-17"))
    .group(Group::Textures)
    .lifetime(Lifetime::Restart);

/// This module's tunables.
pub const TUNABLES: &[&dyn AnyDef] = &[
    &BLOOM,
    &BLOOM_INTENSITY,
    &WATER_QUALITY,
    &CLOUD_MAP_PER_SAMPLE,
    &PUDDLES,
    &SHELTER,
    &GI_PROBES,
    &TEXTURE_MAX_SIZE,
    &DEPTH_ONLY_SKIP_MATERIAL,
    &BIND_GROUP_WALK_ONCE,
];

pub use sf_water_bevy::WaterQuality;

/// `water.quality` text to the tier.
pub fn water_quality(name: &str) -> WaterQuality {
    use WaterQuality as Q;
    match name {
        "low" => Q::Low,
        "medium" => Q::Medium,
        "ultra" => Q::Ultra,
        _ => Q::High,
    }
}

/// Bloom on the managed views follows `render.bloom` and its intensity.
pub struct BloomPlugin;

impl Plugin for BloomPlugin {
    fn build(&self, app: &mut App) {
        let ids = BloomIds { on: app.add_tunable(&BLOOM).id(), intensity: app.add_tunable(&BLOOM_INTENSITY).id() };
        app.add_feature_switch(&BLOOM_SWITCH);
        app.insert_resource(ids).add_systems(Update, sync_bloom);
    }
}

#[derive(Resource, Clone, Copy)]
struct BloomIds {
    on: Id<bool>,
    intensity: Id<f32>,
}

impl BloomIds {
    /// The bloom a view should carry.
    fn wanted(self, tunables: &Tunables) -> Option<f32> {
        tunables.read(self.on).then(|| tunables.read(self.intensity))
    }
}

/// Inserts, retunes or removes `Bloom` on the managed views when a bloom tunable changed or a view
/// appeared. A view's own bloom settings (composite mode, scatter) stay; only the intensity follows.
#[allow(clippy::needless_pass_by_value, clippy::float_cmp)] // exact: change detection
fn sync_bloom(
    mut commands: Commands,
    tunables: Res<Tunables>,
    ids: Res<BloomIds>,
    cameras: Query<(Entity, Option<&Bloom>), With<ViewQualityCamera>>,
    added: Query<(), Added<ViewQualityCamera>>,
    mut seen: Local<Option<(u64, u64)>>,
) {
    let key = (tunables.var(ids.on.untyped()).generation(), tunables.var(ids.intensity.untyped()).generation());
    if *seen == Some(key) && added.is_empty() {
        return;
    }
    *seen = Some(key);
    let want = ids.wanted(&tunables);
    for (camera, have) in &cameras {
        match (want, have) {
            (Some(intensity), Some(b)) if b.intensity != intensity => {
                commands.entity(camera).insert(Bloom { intensity, ..b.clone() });
            }
            (Some(intensity), None) => {
                commands.entity(camera).insert(Bloom { intensity, ..Bloom::NATURAL });
            }
            (None, Some(_)) => {
                commands.entity(camera).remove::<Bloom>();
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sf_tunables_bevy::{SetBy, SetTunable, TunablesPlugin};

    fn set(app: &mut App, name: &str, value: &str) {
        app.world_mut().write_message(SetTunable::new(name, value, SetBy::Console));
        app.update();
    }

    /// `render.bloom` and its intensity reach the managed view live, both ways.
    #[test]
    fn bloom_follows_its_tunables() {
        let mut app = App::new();
        app.add_plugins((TunablesPlugin, BloomPlugin));
        let camera = app.world_mut().spawn(ViewQualityCamera).id();
        app.update();
        assert!(app.world().get::<Bloom>(camera).is_none(), "off by default");
        set(&mut app, "render.bloom", "on");
        set(&mut app, "render.bloom.intensity", "0.3");
        let b = app.world().get::<Bloom>(camera).expect("on");
        assert!((b.intensity - 0.3).abs() < 1e-6);
        set(&mut app, "render.bloom", "off");
        assert!(app.world().get::<Bloom>(camera).is_none(), "live off");
    }

    /// FS-2b: `BloomPlugin` registers the `render.bloom` switch; it follows a Console-layer set.
    #[test]
    fn bloom_switch_registered() {
        let mut app = App::new();
        app.add_plugins((TunablesPlugin, BloomPlugin));
        app.update();
        let running = |app: &App| {
            let switches = app.world().resource::<sf_tunables_bevy::FeatureSwitches>();
            assert_eq!(switches.get("render.bloom"), Some(&BLOOM_SWITCH), "registered");
            switches.running(&app.world().resource::<Tunables>().0, "render.bloom")
        };
        assert!(!running(&app), "off by default");
        set(&mut app, "render.bloom", "on");
        assert!(running(&app));
    }
}
