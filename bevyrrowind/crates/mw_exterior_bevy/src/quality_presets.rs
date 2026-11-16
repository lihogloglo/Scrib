//! Quality presets as tunables, interim (tunables slice T4, spec `superfargoth/docs/specs/tunables.md`
//! "Presets are bundles of tunables"): at startup the chosen [`Quality`] writes its shadow and light
//! values into the `Preset` layer from one table ([`quality_preset_values`], which replaced the
//! `QualityRender` `match` arms for them); a console `set` sits above it (`Console` layer), `--off`
//! parts and the env aliases at `CommandLine`. T5 turns the table into `sf.tunable` records with
//! `sg.*` group drivers and switches levels live.
//!
//! Also the exterior's CW gates as live tunables ([`add_gates`]): `select.gate` (CW-2: streaming,
//! terrain, far field) and `render.skin.skip_unchanged` (CW-7), with their old env switches as aliases.

use bevy::pbr::SkinUploadSettings;
use bevy::prelude::*;
use mw_devtools_bevy::perf::{Off, Part};
use sf_sky_bevy::quality as q;
use sf_tunables_bevy::sf_tunables::Def;
use sf_tunables_bevy::{SetBy, Tunables, TunablesAppExt, Value};

pub use mw_game_core::exterior::Quality;

/// `select.gate` (CW-2), alias `BEVYRROWIND_SELECT_GATE=1|0`; `--off selectgate` forces it off at
/// the `CommandLine` layer.
pub const SELECT_GATE: Def<bool> = sf_world::SELECT_GATE.env_alias("BEVYRROWIND_SELECT_GATE");

/// `render.skin.skip_unchanged` (CW-7), alias `BEVYRROWIND_SKIN_SKIP_UNCHANGED=1`.
pub const SKIN_SKIP_UNCHANGED: Def<bool> = sf_character_bevy::SKIN_SKIP_UNCHANGED.env_alias("BEVYRROWIND_SKIN_SKIP_UNCHANGED");

/// One quality level's shadow and light values, `(tunable, value)`: every key of each level, so a
/// level written over another leaves nothing of it. Sources: the lamp budget High = 8 cached lamps at
/// 512² (Bevy's per-face light-relative cache; perf.md "Shadows v2"); contact shadows off on Low (it
/// costs the depth + motion-vector prepass), 0.3 m = Bevy's default (SH-14), on the lamps on High
/// and Ultra (owner 2026-11-01, SH-16); GTAO 1x2 / 2x2 / 3x3 on Medium / High / Ultra (ledger shadows
/// 2026-10-21, ~0.9 ms at 1440p High); sun cascades a PSSM practical split (ledger shadows
/// 2026-10-19, presets cut to 160-280 m; F2's 2 cascades measured no win, ledger perf 2026-11-16);
/// PCSS on High and Ultra (shadows F5); Hardware2x2 on Low, else temporal under TAA/DLSS (SH-12).
pub fn quality_preset_values(quality: Quality) -> Vec<(&'static str, Value)> {
    let high = matches!(quality, Quality::High | Quality::Ultra);
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
    let text = |s: &str| Value::Str(s.to_owned());
    vec![
        (sf_sky_bevy::LAMP_COUNT.name(), Value::Int(lamps)),
        (sf_sky_bevy::LAMP_MAP_SIZE.name(), Value::Int(face)),
        (sf_sky_bevy::LAMP_CONTACT.name(), Value::Bool(high)),
        (q::CONTACT.name(), Value::Bool(quality != Quality::Low)),
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
        (q::FILTER.name(), text(if quality == Quality::Low { "hardware" } else { "auto" })),
    ]
}

/// The `--off` parts that force a tunable at the `CommandLine` layer: `(part, tunable, value)`.
fn off_values() -> [(Part, &'static str, Value); 4] {
    [
        (Part::ContactShadows, q::CONTACT.name(), Value::Bool(false)),
        (Part::ContactShadows, sf_sky_bevy::LAMP_CONTACT.name(), Value::Bool(false)),
        (Part::Ssao, q::SSAO.name(), Value::Str("off".to_owned())),
        (Part::SoftShadows, q::SUN_SOFT.name(), Value::Bool(false)),
    ]
}

fn set(tunables: &mut Tunables, name: &str, value: Value, layer: SetBy, note: String) {
    let id = tunables.id(name).unwrap_or_else(|| panic!("tunable {name} is not registered"));
    tunables.set_noted(id, value, layer, Some(note)).unwrap_or_else(|e| panic!("tunable {name}: {e}"));
}

/// Writes `quality`'s level into the `Preset` layer and the `--off` parts into `CommandLine`.
/// After `RenderSettings::insert` (it registers the tunables).
pub fn apply(tunables: &mut Tunables, quality: Quality, off: &Off) {
    for (name, value) in quality_preset_values(quality) {
        set(tunables, name, value, SetBy::Preset, format!("quality {quality:?}"));
    }
    for (part, name, value) in off_values() {
        if off.has(part) {
            set(tunables, name, value, SetBy::CommandLine, format!("--off {}", part.name()));
        }
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

    /// A `MinimalPlugins` app with the render settings' tunables, the gates and `quality`'s level.
    fn app(quality: Quality, off: &Off, env: &[(&str, &str)]) -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, sf_tunables_bevy::TunablesPlugin)).insert_resource(sf_tunables_bevy::TunableEnv::from_pairs(env));
        mw_environment_bevy::render::RenderSettings::new(false, Some(sf_sky_bevy::sf_sky::Upscaling::Off), true, true)
            .expect("settings")
            .with_shadow_cache(false)
            .insert(&mut app);
        apply(&mut app.world_mut().resource_mut::<Tunables>(), quality, off);
        app.insert_resource(q::SunShadowQuality::default())
            .init_resource::<SkyRenderOptions>()
            .init_resource::<sf_terrain_bevy::TerrainSettings>()
            .init_resource::<sf_lod_bevy::FarFieldSettings>()
            .insert_resource(sf_world_bevy::Streaming::new(sf_world_bevy::StreamingConfig::default()));
        let _ = add_gates(&mut app, off.has(Part::SelectGate));
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
