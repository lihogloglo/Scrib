//! Quality presets as tunables, interim (tunables slices T4, T5a; spec
//! `superfargoth/docs/specs/tunables.md` "Presets are bundles of tunables"): at startup the chosen
//! [`Quality`] writes every value it decides into the `Preset` layer from one table
//! ([`quality_preset_values`], which replaced the `QualityRender` and `Quality` `match` arms); a
//! console `set` sits above it (`Console` layer), `--off` parts, flags and the env aliases at
//! `CommandLine`. T5b turns the table into `sf.tunable` records with `sg.*` group drivers and
//! switches levels live.
//!
//! What a value costs to change (its `Lifetime`): shadows, lights, GTAO, contact shadows, bloom, the
//! temporal resolve (prepasses and MSAA follow), clouds, water, puddles, shelter and parallax are live
//! (some re-specialise pipelines); the LOD bias and size culling apply to cells streamed after the
//! change (`Respawn`); the far-field handoff scale, the texture cap and the probe GI are read once at
//! start (`Restart`, [`start_values`]): so Ultra switched to live is Ultra minus its textures.
//!
//! Also the exterior's CW gates as live tunables ([`add_gates`]): `select.gate` (CW-2: streaming,
//! terrain, far field) and `render.skin.skip_unchanged` (CW-7), with their old env switches as aliases.

use bevy::pbr::SkinUploadSettings;
use bevy::prelude::*;
use mw_devtools_bevy::perf::{Off, Part};
use sf_sky_bevy::quality as q;
use sf_tunables_bevy::overrides::{OverrideSet, apply_overrides};
use sf_tunables_bevy::sf_tunables::Def;
use sf_tunables_bevy::{SetBy, Tunables, TunablesAppExt, Value};

pub use mw_environment_bevy::quality_tunables::{CLOUD_MAP_PER_SAMPLE, GI_PROBES, PUDDLES, SHELTER, TEXTURE_MAX_SIZE};
pub use mw_game_core::exterior::Quality;

/// Bloom's scatter fraction in this game: two thirds of `Bloom::NATURAL`'s 0.15 (the fog's sun glow
/// already halos the disk, screen-space-effects research 4.11).
const BLOOM_INTENSITY: f64 = 0.1;

/// Every tunable the table writes that the render settings (`RenderSettings::insert`: shadows,
/// lights, GTAO, upscaling) don't register: register them before [`apply`].
pub fn register(app: &mut App) {
    // One by one: a def registered here first would keep another crate's env-alias copy from
    // registering fresh (`select.gate`, `mw.matctrl.gate`), and its alias from applying.
    use sf_world::lod_screen as l;
    app.register_tunables(&[
        &l::LOD_BIAS,
        &l::SIZE_CULL_MIN_PX,
        &l::SIZE_CULL_MIN_DISTANCE,
        &sf_lod_bevy::FAR_HANDOFF_SCALE,
        &mw_assets::PARALLAX_LAYERS,
        &mw_assets::PARALLAX_RELIEF_STEPS,
    ])
    .register_tunables(mw_environment_bevy::quality_tunables::TUNABLES);
}

/// `select.gate` (CW-2), alias `BEVYRROWIND_SELECT_GATE=1|0`; `--off selectgate` forces it off at
/// the `CommandLine` layer.
pub const SELECT_GATE: Def<bool> = sf_world::SELECT_GATE.env_alias("BEVYRROWIND_SELECT_GATE");

/// `render.skin.skip_unchanged` (CW-7), alias `BEVYRROWIND_SKIN_SKIP_UNCHANGED=1`.
pub const SKIN_SKIP_UNCHANGED: Def<bool> = sf_character_bevy::SKIN_SKIP_UNCHANGED.env_alias("BEVYRROWIND_SKIN_SKIP_UNCHANGED");

/// One quality level's values, `(tunable, value)`: every key of each level, so a level written over
/// another leaves nothing of it. Sources, shadows and lights: the lamp budget High = 8 cached lamps
/// at 512² (Bevy's per-face light-relative cache; perf.md "Shadows v2"); contact shadows off on Low
/// (it costs the depth + motion-vector prepass), 0.3 m = Bevy's default (SH-14), on the lamps on High
/// and Ultra (owner 2026-11-01, SH-16); GTAO 1x2 / 2x2 / 3x3 on Medium / High / Ultra (ledger shadows
/// 2026-10-21, ~0.9 ms at 1440p High); sun cascades a PSSM practical split (ledger shadows
/// 2026-10-19, presets cut to 160-280 m; F2's 2 cascades measured no win, ledger perf 2026-11-16);
/// PCSS on High and Ultra (shadows F5); Hardware2x2 on Low, else temporal under TAA/DLSS (SH-12).
/// View distance (industry-audit.md J1): one bias scales LOD selection, size culling and the far
/// handoff d1 (the baked 400 m x bias, ledger statics 2026-10-12); the size-culling cutoff matches
/// `OpenMW`'s 2 px on High, our 1 px LOD budget on Ultra (culls 3 % at High, 8 % at Low, ledger
/// statics 2026-10-12). Post: bloom off on Low (research 4.11); MSAA on Low, else a temporal resolve
/// (DLSS `auto` in DLSS builds, TAA otherwise; ROADMAP 1.3). Sky: the clouds' weather map per sample
/// on High and Ultra (weather.md, distant weather). Water: the FFT tier of the level (water.md
/// "Performance budgets"). Surface: no puddles, no shelter map on Low (surface-weather.md "Budget").
/// Textures: the load cap Low 512, Medium 1024, High 2048, Ultra none (ledger textures 2026-11-13,
/// textures.md §6); parallax none on Low, 16-layer occlusion on Medium, 32/5 and 64/8 relief on High
/// and Ultra (textures.md §9 T4d). GI: the probe GI on High and Ultra (gi.md slice 1).
pub fn quality_preset_values(quality: Quality) -> Vec<(&'static str, Value)> {
    use mw_environment_bevy::quality_tunables as qt;
    let high = matches!(quality, Quality::High | Quality::Ultra);
    let low = quality == Quality::Low;
    let (lamps, face): (i64, i64) = match quality {
        Quality::Low => (0, 512),
        Quality::Medium => (6, 512),
        Quality::High => (8, 512),
        Quality::Ultra => (12, 1024),
    };
    let ssao = match quality {
        Quality::Low => "off",
        Quality::Medium => "low",
        Quality::High => "medium",
        Quality::Ultra => "high",
    };
    let (cascades, distance, lambda, overlap): (i64, f64, f64, f64) = match quality {
        Quality::Low => (3, 160.0, 0.82, 0.15),
        Quality::Medium => (3, 190.0, 0.80, 0.15),
        Quality::High => (3, 205.0, 0.80, 0.16),
        Quality::Ultra => (4, 280.0, 0.85, 0.2),
    };
    let (bias, min_pixels, min_draw_distance): (f64, f64, f64) = match quality {
        Quality::Low => (0.75, 4.0, 16.0),
        Quality::Medium => (0.875, 3.0, 20.0),
        Quality::High => (1.0, 2.0, 24.0),
        Quality::Ultra => (1.0, 1.0, 32.0),
    };
    let (water, texture_cap, parallax_layers, relief_steps): (&str, i64, i64, i64) = match quality {
        Quality::Low => ("low", 512, 0, 0),
        Quality::Medium => ("medium", 1024, 16, 0),
        Quality::High => ("high", 2048, 32, 5),
        Quality::Ultra => ("ultra", 0, 64, 8),
    };
    let text = |s: &str| Value::Str(s.to_owned());
    vec![
        (sf_sky_bevy::LAMP_COUNT.name(), Value::Int(lamps)),
        (sf_sky_bevy::LAMP_MAP_SIZE.name(), Value::Int(face)),
        (sf_sky_bevy::LAMP_CONTACT.name(), Value::Bool(high)),
        (q::CONTACT.name(), Value::Bool(!low)),
        (q::CONTACT_STEPS.name(), Value::Int(if high { 16 } else { 8 })),
        (q::CONTACT_THICKNESS.name(), Value::Float(0.1)),
        (q::CONTACT_LENGTH.name(), Value::Float(0.3)),
        (q::SSAO.name(), text(ssao)),
        (q::SUN_CASCADES.name(), Value::Int(cascades)),
        (q::SUN_DISTANCE.name(), Value::Float(distance)),
        (q::SUN_LAMBDA.name(), Value::Float(lambda)),
        (q::SUN_OVERLAP.name(), Value::Float(overlap)),
        (q::SUN_SOFT.name(), Value::Bool(high)),
        (q::SUN_SOFT_SIZE.name(), Value::Float(10.0)),
        (q::FILTER.name(), text(if low { "hardware" } else { "auto" })),
        (sf_world::lod_screen::LOD_BIAS.name(), Value::Float(bias)),
        (sf_world::lod_screen::SIZE_CULL_MIN_PX.name(), Value::Float(min_pixels)),
        (sf_world::lod_screen::SIZE_CULL_MIN_DISTANCE.name(), Value::Float(min_draw_distance)),
        (sf_lod_bevy::FAR_HANDOFF_SCALE.name(), Value::Float(bias)),
        (qt::BLOOM.name(), Value::Bool(!low)),
        (qt::BLOOM_INTENSITY.name(), Value::Float(BLOOM_INTENSITY)),
        (q::UPSCALING.name(), text(mw_environment_bevy::render::default_upscaling(low).label())),
        (CLOUD_MAP_PER_SAMPLE.name(), Value::Bool(high)),
        (qt::WATER_QUALITY.name(), text(water)),
        (PUDDLES.name(), Value::Bool(!low)),
        (SHELTER.name(), Value::Bool(!low)),
        (TEXTURE_MAX_SIZE.name(), Value::Int(texture_cap)),
        (mw_assets::PARALLAX_LAYERS.name(), Value::Int(parallax_layers)),
        (mw_assets::PARALLAX_RELIEF_STEPS.name(), Value::Int(relief_steps)),
        (GI_PROBES.name(), Value::Bool(high)),
    ]
}

/// The `--off` parts that force a tunable at the `CommandLine` layer: `(part, tunable, value)`.
fn off_values() -> [(Part, &'static str, Value); 9] {
    use mw_environment_bevy::quality_tunables as qt;
    [
        (Part::ContactShadows, q::CONTACT.name(), Value::Bool(false)),
        (Part::ContactShadows, sf_sky_bevy::LAMP_CONTACT.name(), Value::Bool(false)),
        (Part::Ssao, q::SSAO.name(), Value::Str("off".to_owned())),
        (Part::SoftShadows, q::SUN_SOFT.name(), Value::Bool(false)),
        (Part::SizeCull, sf_world::lod_screen::SIZE_CULL_MIN_PX.name(), Value::Float(0.0)),
        (Part::Bloom, qt::BLOOM.name(), Value::Bool(false)),
        (Part::Puddles, PUDDLES.name(), Value::Bool(false)),
        (Part::Shelter, SHELTER.name(), Value::Bool(false)),
        (Part::Gi, GI_PROBES.name(), Value::Bool(false)),
    ]
}

fn set(tunables: &mut Tunables, name: &str, value: Value, layer: SetBy, note: String) {
    let id = tunables.id(name).unwrap_or_else(|| panic!("tunable {name} is not registered"));
    tunables.set_noted(id, value, layer, Some(note)).unwrap_or_else(|e| panic!("tunable {name}: {e}"));
}

/// Writes `quality`'s level into the `Preset` layer, the `--off` parts and `--texture-cap <px>`
/// (`texture_cap`, 0: none) into `CommandLine`. After `RenderSettings::insert` and [`register`]
/// (they register the tunables).
pub fn apply(tunables: &mut Tunables, quality: Quality, off: &Off, texture_cap: Option<u32>) {
    for (name, value) in quality_preset_values(quality) {
        set(tunables, name, value, SetBy::Preset, format!("quality {quality:?}"));
    }
    for (part, name, value) in off_values() {
        if off.has(part) {
            set(tunables, name, value, SetBy::CommandLine, format!("--off {}", part.name()));
        }
    }
    if let Some(px) = texture_cap {
        set(tunables, TEXTURE_MAX_SIZE.name(), Value::Int(i64::from(px)), SetBy::CommandLine, "--texture-cap".to_owned());
    }
}

/// The parallax new loads bind at a level ([`quality_preset_values`]' `texture.parallax.*`), for
/// `mw_assets::set_parallax` before the app is built (the exterior then follows the tunables live).
pub fn level_parallax(quality: Quality) -> Option<mw_assets::Parallax> {
    let values = quality_preset_values(quality);
    let get = |name: &str| match values.iter().find(|(n, _)| *n == name) {
        Some((_, Value::Int(v))) => u8::try_from(*v).unwrap_or(u8::MAX),
        _ => 0,
    };
    let (layers, steps) = (get(mw_assets::PARALLAX_LAYERS.name()), get(mw_assets::PARALLAX_RELIEF_STEPS.name()));
    (layers > 0).then(|| mw_assets::Parallax::new(layers, (steps > 0).then_some(steps)))
}

/// The parallax the tunables give (after [`apply`] and the overrides): `mw_assets::set_parallax` before
/// models load, so the first loads bind what the run's level says.
pub fn tunables_parallax(tunables: &Tunables) -> Option<mw_assets::Parallax> {
    let get = |def: &Def<u32>| -> u32 { tunables.read(tunables.id_of(def).expect("registered")) };
    let clamp = |v: u32| u8::try_from(v).unwrap_or(u8::MAX);
    let (layers, steps) = (get(&mw_assets::PARALLAX_LAYERS), get(&mw_assets::PARALLAX_RELIEF_STEPS));
    (layers > 0).then(|| mw_assets::Parallax::new(clamp(layers), (steps > 0).then(|| clamp(steps))))
}

/// The `Restart` tunables' values for this start (read while the app is built).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StartValues {
    /// `texture.max_size_px` (`None`: no cap).
    pub texture_cap: Option<u32>,
    /// `far.handoff_scale`.
    pub far_handoff_scale: f32,
    /// `render.gi.probes`.
    pub gi: bool,
}

/// The `Restart` tunables' effective values, after the run's `--set` / `--tunables` overrides for
/// them (the others apply in `Plugin::finish`, once every plugin registered its tunables; these are
/// read before that). A refused override is left to `finish`, which reports it and fails the run.
pub fn start_values(tunables: &mut Tunables, overrides: &[OverrideSet]) -> StartValues {
    let restart = [TEXTURE_MAX_SIZE.name(), sf_lod_bevy::FAR_HANDOFF_SCALE.name(), GI_PROBES.name()];
    for set in overrides {
        let early: Vec<_> = set.overrides.iter().filter(|o| restart.contains(&o.name.as_str())).cloned().collect();
        let _refused = apply_overrides(tunables, &early, SetBy::CommandLine, Some(&set.origin));
    }
    let cap: u32 = tunables.read(tunables.id_of(&TEXTURE_MAX_SIZE).expect("registered"));
    StartValues {
        texture_cap: (cap > 0).then_some(cap),
        far_handoff_scale: tunables.read(tunables.id_of(&sf_lod_bevy::FAR_HANDOFF_SCALE).expect("registered")),
        gi: tunables.read(tunables.id_of(&GI_PROBES).expect("registered")),
    }
}

/// The streaming screen from the `lod.*` tunables' effective values.
pub fn lod_screen(tunables: &Tunables) -> sf_model::LodScreen {
    use sf_world::lod_screen as l;
    let get = |def: &Def<f32>| -> f32 { tunables.read(tunables.id_of(def).expect("registered")) };
    sf_model::LodScreen {
        bias: get(&l::LOD_BIAS),
        min_pixels: get(&l::SIZE_CULL_MIN_PX),
        min_draw_distance: get(&l::SIZE_CULL_MIN_DISTANCE),
        ..sf_model::LodScreen::default()
    }
}

/// Binds the table's game-side live tunables into what consumes them (each writes once its target
/// exists, then on a change): the streaming screen (`Respawn`: cells streamed after it), the clouds'
/// weather-map sampling, puddles and shelter, the parallax of loaded materials, bloom on the views.
pub fn bind_live(app: &mut App) {
    use sf_world::lod_screen as l;
    use sf_world_bevy::Streaming;
    let _bias = app
        .add_tunable(&l::LOD_BIAS)
        .bind_resource(|s: &mut Streaming, v| s.set_lod_screen(sf_model::LodScreen { bias: v, ..s.config().lod }));
    let _px = app
        .add_tunable(&l::SIZE_CULL_MIN_PX)
        .bind_resource(|s: &mut Streaming, v| s.set_lod_screen(sf_model::LodScreen { min_pixels: v, ..s.config().lod }));
    let _floor = app
        .add_tunable(&l::SIZE_CULL_MIN_DISTANCE)
        .bind_resource(|s: &mut Streaming, v| s.set_lod_screen(sf_model::LodScreen { min_draw_distance: v, ..s.config().lod }));
    let _clouds = app
        .add_tunable(&CLOUD_MAP_PER_SAMPLE)
        .bind_resource(|c: &mut sf_atmos_bevy::Clouds, v| c.settings.quality.map_per_sample = v);
    let _puddles = app.add_tunable(&PUDDLES).bind_resource(|s: &mut sf_surface_bevy::SurfaceSettings, v| s.puddles = v);
    let _shelter = app.add_tunable(&SHELTER).bind_resource(|s: &mut sf_surface_bevy::SurfaceSettings, v| s.shelter = v);
    mw_assets::add_parallax_tunables(app);
    if !app.is_plugin_added::<mw_environment_bevy::quality_tunables::BloomPlugin>() {
        app.add_plugins(mw_environment_bevy::quality_tunables::BloomPlugin);
    }
}

/// The exterior's CW gates: registers their alias copies (env switches apply now), `--off selectgate`
/// (`select_gate_off`) at `CommandLine`, and binds them live into terrain, far field and streaming
/// settings and Bevy's skin upload settings. Returns `select.gate`'s effective value (the plugins'
/// starting settings).
pub fn add_gates(app: &mut App, select_gate_off: bool) -> bool {
    app.register_tunables(&[&SELECT_GATE, &SKIN_SKIP_UNCHANGED]);
    if select_gate_off {
        let mut tunables = app.world_mut().resource_mut::<Tunables>();
        set(&mut tunables, SELECT_GATE.name(), Value::Bool(false), SetBy::CommandLine, "--off selectgate".to_owned());
    }
    let _terrain = app.add_tunable(&SELECT_GATE).bind_resource(|t: &mut sf_terrain_bevy::TerrainSettings, on| t.select_gate.enabled = on);
    let _far = app.add_tunable(&SELECT_GATE).bind_resource(|f: &mut sf_lod_bevy::FarFieldSettings, on| f.select_gate.enabled = on);
    let _streaming = app.add_tunable(&SELECT_GATE).bind_resource(|s: &mut sf_world_bevy::Streaming, on| {
        let quanta = sf_world::GateQuanta { enabled: on, ..s.config().scheduler.select_gate };
        s.set_select_gate(quanta);
    });
    app.init_resource::<SkinUploadSettings>();
    let _skin = app.add_tunable(&SKIN_SKIP_UNCHANGED).bind_resource(|s: &mut SkinUploadSettings, on| s.skip_unchanged = on);
    let tunables = app.world().resource::<Tunables>();
    tunables.read(tunables.id_of(&SELECT_GATE).expect("registered"))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A `MinimalPlugins` app with the render settings' tunables, the table's, the gates and
    /// `quality`'s level, and every consumer the game-side bindings reach.
    fn app(quality: Quality, off: &Off, env: &[(&str, &str)]) -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, sf_tunables_bevy::TunablesPlugin)).insert_resource(sf_tunables_bevy::TunableEnv::from_pairs(env));
        mw_environment_bevy::render::RenderSettings::new(false, None, true, quality == Quality::Low)
            .expect("settings")
            .with_shadow_cache(false)
            .insert(&mut app);
        register(&mut app);
        apply(&mut app.world_mut().resource_mut::<Tunables>(), quality, off, None);
        app.insert_resource(q::SunShadowQuality::default())
            .init_resource::<SkyRenderOptions>()
            .init_resource::<sf_terrain_bevy::TerrainSettings>()
            .init_resource::<sf_lod_bevy::FarFieldSettings>()
            .insert_resource(sf_world_bevy::Streaming::new(sf_world_bevy::StreamingConfig::default()))
            .insert_resource(mw_assets::MaterialMaps::new(true, true, true))
            .init_resource::<sf_surface_bevy::SurfaceSettings>()
            .insert_resource(sf_atmos_bevy::Clouds {
                settings: sf_atmos_bevy::sf_atmos::CloudSettings::default(),
                enabled: true,
                shadows: true,
                glows: [Vec4::ZERO; sf_atmos_bevy::GLOWS],
                temporal: sf_atmos_bevy::sf_atmos::CloudTemporal::default(),
            });
        let tier = mw_environment_bevy::water::bind_quality(&mut app);
        app.insert_resource(tier);
        let _ = add_gates(&mut app, off.has(Part::SelectGate));
        bind_live(&mut app);
        app.update();
        app
    }

    fn console(app: &mut App, name: &str, value: &str) {
        app.world_mut().write_message(sf_tunables_bevy::SetTunable::new(name, value, SetBy::Console));
        app.update();
    }

    use sf_sky_bevy::SkyRenderOptions;

    /// Golden (no behaviour change): each level's effective values equal the old `QualityRender` arms
    /// (copied here as they were), read where the game consumes them.
    #[test]
    fn preset_levels_equal_the_old_quality_arms() {
        use bevy::light::ShadowFilteringMethod as F;
        use bevy::pbr::ScreenSpaceAmbientOcclusionQualityLevel as L;
        // (lamps, face, lamp contact, contact steps or none, ssao, (cascades, distance, lambda, overlap), pcss, filter w/o and with TAA)
        type Row = (usize, usize, bool, Option<u32>, Option<L>, (usize, f32, f32, f32), Option<f32>, (F, F));
        let old: [(Quality, Row); 4] = [
            (Quality::Low, (0, 512, false, None, None, (3, 160.0, 0.82, 0.15), None, (F::Hardware2x2, F::Hardware2x2))),
            (Quality::Medium, (6, 512, false, Some(8), Some(L::Low), (3, 190.0, 0.80, 0.15), None, (F::Gaussian, F::Temporal))),
            (Quality::High, (8, 512, true, Some(16), Some(L::Medium), (3, 205.0, 0.80, 0.16), Some(10.0), (F::Gaussian, F::Temporal))),
            (Quality::Ultra, (12, 1024, true, Some(16), Some(L::High), (4, 280.0, 0.85, 0.2), Some(10.0), (F::Gaussian, F::Temporal))),
        ];
        for (quality, (lamps, face, lamp_contact, steps, ssao, (n, distance, lambda, overlap), pcss, filters)) in old {
            let app = app(quality, &Off::default(), &[]);
            let w = app.world();
            let b = w.resource::<sf_sky_bevy::PointShadowBudget>();
            assert_eq!((b.max, b.map_size, b.contact_shadows, b.fade_start, b.fade_end), (lamps, face, lamp_contact, 35.0, 45.0), "{quality:?} lamps");
            let sun = w.resource::<q::SunShadowQuality>();
            let splits = sf_sky_bevy::CascadeSplits { num_cascades: n, minimum_distance: 0.1, split_near: 1.0, maximum_distance: distance, lambda, overlap_proportion: overlap };
            assert_eq!((sun.splits, sun.soft_size()), (splits, pcss), "{quality:?} sun");
            assert_eq!(w.resource::<SkyRenderOptions>().cascades.bounds, splits.build().bounds, "{quality:?} options");
            let tunables = w.resource::<Tunables>();
            let ids = *w.resource::<q::ViewQualityIds>();
            for (temporal, filter) in [(false, filters.0), (true, filters.1)] {
                let view = q::ViewQuality::read(tunables, &ids, temporal);
                assert_eq!(view.contact.map(|c| (c.linear_steps, c.thickness, c.length)), steps.map(|s| (s, 0.1, 0.3)), "{quality:?} contact");
                assert_eq!((view.ssao, view.filter), (ssao, filter), "{quality:?} ssao, filter");
            }
        }
    }

    /// Golden (T5a, no behaviour change at startup): the rest of each level equals the old `Quality`
    /// and `QualityRender` arms and `default_upscaling` (copied here as they were), read where the
    /// game consumes them.
    #[test]
    #[allow(clippy::float_cmp)] // exact: copied constants
    fn t5a_levels_equal_the_old_quality_arms() {
        use mw_environment_bevy::quality_tunables::WaterQuality as W;
        // (bias, min_pixels, min_draw), bloom, map_per_sample, water, (puddles, shelter), texture cap,
        // parallax (layers, relief steps), gi
        type Row = ((f32, f32, f32), bool, bool, W, bool, Option<u32>, Option<(u8, Option<u8>)>, bool);
        let old: [(Quality, Row); 4] = [
            (Quality::Low, ((0.75, 4.0, 16.0), false, false, W::Low, false, Some(512), None, false)),
            (Quality::Medium, ((0.875, 3.0, 20.0), true, false, W::Medium, true, Some(1024), Some((16, None)), false)),
            (Quality::High, ((1.0, 2.0, 24.0), true, true, W::High, true, Some(2048), Some((32, Some(5))), true)),
            (Quality::Ultra, ((1.0, 1.0, 32.0), true, true, W::Ultra, true, None, Some((64, Some(8))), true)),
        ];
        for (quality, ((bias, min_pixels, min_draw), bloom, per_sample, water, surface, cap, parallax, gi)) in old {
            let mut app = app(quality, &Off::default(), &[]);
            let w = app.world();
            let lod = w.resource::<sf_world_bevy::Streaming>().config().lod;
            assert_eq!((lod.bias, lod.min_pixels, lod.min_draw_distance), (bias, min_pixels, min_draw), "{quality:?} screen");
            assert_eq!(lod_screen(w.resource::<Tunables>()), lod, "{quality:?} start screen");
            let t = w.resource::<Tunables>();
            let read_bool = |name: &str| t.flag(t.id(name).expect(name));
            assert_eq!(read_bool("render.bloom"), bloom, "{quality:?} bloom");
            assert!((t.float(t.id("render.bloom.intensity").expect("intensity")) - 0.1).abs() < 1e-6, "{quality:?} bloom intensity");
            let upscaling = t.var(t.id(q::UPSCALING.name()).expect("upscaling")).layer(SetBy::Preset).cloned();
            let want = mw_environment_bevy::render::default_upscaling(quality == Quality::Low);
            assert_eq!(upscaling, Some(Value::Str(want.label().to_owned())), "{quality:?} upscaling");
            assert_eq!(w.resource::<sf_sky_bevy::UpscalingSetting>().0, want, "{quality:?} upscaling setting");
            assert_eq!(w.resource::<sf_atmos_bevy::Clouds>().settings.quality.map_per_sample, per_sample, "{quality:?} clouds");
            assert_eq!(*w.resource::<W>(), water, "{quality:?} water");
            let s = w.resource::<sf_surface_bevy::SurfaceSettings>();
            assert_eq!((s.puddles, s.shelter), (surface, surface), "{quality:?} surface");
            let maps = w.resource::<mw_assets::MaterialMaps>();
            let old_parallax = parallax.map(|(layers, steps)| mw_assets::Parallax::new(layers, steps));
            assert_eq!(maps.preset(), old_parallax, "{quality:?} parallax");
            assert_eq!(level_parallax(quality), old_parallax, "{quality:?} level parallax");
            let start = start_values(&mut app.world_mut().resource_mut::<Tunables>(), &[]);
            assert_eq!(start, StartValues { texture_cap: cap, far_handoff_scale: bias, gi }, "{quality:?} start values");
        }
    }

    /// T5a: every game-side binding reaches its consumer live from the console: the streaming screen
    /// (new spawns), the clouds, water, puddles, shelter, parallax, bloom on a managed view, and the
    /// temporal resolve.
    #[test]
    fn t5a_bindings_are_live() {
        let mut app = app(Quality::Low, &Off::default(), &[]);
        let view = app.world_mut().spawn(sf_sky_bevy::ViewQualityCamera).id();
        app.update();
        assert!(app.world().get::<bevy::post_process::bloom::Bloom>(view).is_none(), "Low: no bloom");
        for (name, value) in [
            ("lod.bias", "1.5"),
            ("lod.size_cull.min_px", "0"),
            ("lod.size_cull.min_distance_m", "40"),
            ("sky.clouds.map_per_sample", "on"),
            ("water.quality", "ultra"),
            ("surface.puddles", "on"),
            ("surface.shelter", "on"),
            ("texture.parallax.layers", "64"),
            ("texture.parallax.relief_steps", "8"),
            ("render.bloom", "on"),
            ("render.upscaling", "taa"),
        ] {
            console(&mut app, name, value);
        }
        let w = app.world();
        let lod = w.resource::<sf_world_bevy::Streaming>().config().lod;
        assert_eq!((lod.bias, lod.min_pixels, lod.min_draw_distance), (1.5, 0.0, 40.0));
        assert!(w.resource::<sf_atmos_bevy::Clouds>().settings.quality.map_per_sample);
        assert_eq!(*w.resource::<mw_environment_bevy::quality_tunables::WaterQuality>(), mw_environment_bevy::quality_tunables::WaterQuality::Ultra);
        let s = w.resource::<sf_surface_bevy::SurfaceSettings>();
        assert!(s.puddles && s.shelter);
        assert_eq!(w.resource::<mw_assets::MaterialMaps>().preset(), Some(mw_assets::Parallax::new(64, Some(8))));
        let bloom = w.get::<bevy::post_process::bloom::Bloom>(view).expect("bloom on");
        assert!((bloom.intensity - 0.1).abs() < 1e-6, "the level's intensity");
        assert_eq!(w.resource::<sf_sky_bevy::UpscalingSetting>().0, sf_sky_bevy::sf_sky::Upscaling::Taa);
    }

    /// `--off sizecull|bloom|puddles|shelter|gi` and `--texture-cap` sit at `CommandLine`; a `--set` of a
    /// `Restart` tunable reaches the start values (read before `finish` applies the rest).
    #[test]
    fn t5a_off_parts_flags_and_early_sets() {
        let off = Off::from_flags(&["sizecull", "bloom", "puddles", "shelter", "gi"].map(String::from)).expect("off parts");
        let mut app = app(Quality::Ultra, &off, &[]);
        let w = app.world();
        assert!(w.resource::<sf_world_bevy::Streaming>().config().lod.min_pixels.abs() < f32::EPSILON, "--off sizecull");
        let s = w.resource::<sf_surface_bevy::SurfaceSettings>();
        assert!(!s.puddles && !s.shelter);
        let mut t = app.world_mut().resource_mut::<Tunables>();
        apply(&mut t, Quality::Ultra, &off, Some(1024));
        let sets = [OverrideSet {
            origin: "--set".into(),
            overrides: vec![sf_tunables_bevy::overrides::Override { name: "far.handoff_scale".into(), value: "1.5".into() }],
        }];
        let start = start_values(&mut t, &sets);
        assert_eq!(start, StartValues { texture_cap: Some(1024), far_handoff_scale: 1.5, gi: false });
        apply(&mut t, Quality::Ultra, &Off::default(), Some(0));
        assert_eq!(start_values(&mut t, &[]).texture_cap, None, "--texture-cap 0: none");
    }

    /// A value set at the console sits above the preset (and `reset` peels back to it).
    #[test]
    fn console_sits_above_the_preset() {
        let mut app = app(Quality::High, &Off::default(), &[]);
        console(&mut app, "shadow.lamp.count", "2");
        console(&mut app, "shadow.sun.cascades", "2");
        assert_eq!(app.world().resource::<sf_sky_bevy::PointShadowBudget>().max, 2);
        assert_eq!(app.world().resource::<SkyRenderOptions>().cascades.bounds.len(), 2, "options follow");
        let tunables = app.world().resource::<Tunables>();
        let lamps = tunables.var(tunables.id("shadow.lamp.count").expect("lamps"));
        assert_eq!((lamps.set_by(), lamps.layer(SetBy::Preset)), (SetBy::Console, Some(&Value::Int(8))));
        app.world_mut().write_message(sf_tunables_bevy::PeelTunable { name: "shadow.lamp.count".into(), layer: Some(SetBy::Console) });
        app.update();
        assert_eq!(app.world().resource::<sf_sky_bevy::PointShadowBudget>().max, 8, "back to the preset");
    }

    /// `--off` parts sit at `CommandLine`, above the preset.
    #[test]
    fn off_parts_force_their_tunables() {
        let off = Off::from_flags(&["contactshadows", "ssao", "softshadows", "selectgate"].map(String::from)).expect("off parts");
        let app = app(Quality::Ultra, &off, &[("BEVYRROWIND_SELECT_GATE", "1")]);
        let w = app.world();
        let view = q::ViewQuality::read(w.resource::<Tunables>(), w.resource::<q::ViewQualityIds>(), true);
        assert!(view.contact.is_none() && view.ssao.is_none());
        assert!(!w.resource::<sf_sky_bevy::PointShadowBudget>().contact_shadows);
        assert_eq!(w.resource::<q::SunShadowQuality>().soft_size(), None);
        assert!(!w.resource::<sf_world_bevy::Streaming>().config().scheduler.select_gate.enabled, "--off selectgate beats the env");
    }

    /// CW-2 and CW-7 are live: the env alias turns them on at start, a console set flips them both ways
    /// in every consumer (terrain, far field, streaming; skin uploads).
    #[test]
    fn cw_gates_are_live() {
        let read = |app: &App| {
            let w = app.world();
            (
                w.resource::<sf_terrain_bevy::TerrainSettings>().select_gate.enabled,
                w.resource::<sf_lod_bevy::FarFieldSettings>().select_gate.enabled,
                w.resource::<sf_world_bevy::Streaming>().config().scheduler.select_gate.enabled,
                w.resource::<SkinUploadSettings>().skip_unchanged,
            )
        };
        let mut gated = app(Quality::High, &Off::default(), &[("BEVYRROWIND_SELECT_GATE", "1"), ("BEVYRROWIND_SKIN_SKIP_UNCHANGED", "1")]);
        assert_eq!(read(&gated), (true, true, true, true), "env aliases");
        console(&mut gated, "select.gate", "off");
        console(&mut gated, "render.skin.skip_unchanged", "off");
        assert_eq!(read(&gated), (false, false, false, false), "live off");
        let quanta = gated.world().resource::<sf_world_bevy::Streaming>().config().scheduler.select_gate;
        assert_eq!(quanta, sf_world::SchedulerConfig::select_gate_quanta(false), "quanta kept");
        console(&mut gated, "select.gate", "on");
        assert!(read(&gated).0);
        let mut plain = app(Quality::Low, &Off::default(), &[]);
        assert_eq!(read(&plain), (false, false, false, false), "off by default");
        console(&mut plain, "render.skin.skip_unchanged", "on");
        assert!(read(&plain).3);
    }
}
