//! Bevyrrowind's side of the shadow and light tunables (tunables slice T4, spec
//! `superfargoth/docs/specs/tunables.md` "Migration"; the framework declares and binds them in
//! `sf_sky_bevy::quality`):
//!
//! - **Env aliases.** The game's copies of framework defs carry the old `BEVYRROWIND_*` switches as
//!   deprecated aliases (`Def::env_alias`, `CommandLine` layer). The framework never names them, so
//!   the copies register first ([`register`], before `ShadowQualityPlugin` and `SkyPlugin`): an alias
//!   applies only where its def is registered fresh.
//! - **Mod layer.** Bevyrrowind's own values above framework defaults (census "two truths": the lamp
//!   fade band) are `sf.tunable` records of the builtin mod (`mw_mods::BASE_RECORDS`, slice T5b):
//!   [`register`] applies the shipped ones ([`base_records`]), the exterior re-applies the loaded
//!   profile's ([`apply_mod_records`]), so a mod's record wins.
//! - **Preset layer.** The viewers' base preset (the High lamp budget); the exterior's graphics preset
//!   (`mw_exterior_bevy::quality_presets`, `sf.graphics_preset`) rewrites the layer for its level.
//! - **Command line.** `--point-shadows` and `--sun-cascades` (and `BEVYRROWIND_SUN_CASCADES`, a
//!   `COUNT:METERS` pair no single alias parses) are sugar onto the `CommandLine` layer.

use std::sync::OnceLock;

use bevy_app::App;
use sf_records::RecordDb;
use sf_sky_bevy::quality as q;
use sf_sky_bevy::sf_sky::sun_gate as sun;
use sf_tunables_bevy::sf_tunables::{AnyDef, Def, SetBy, Type};
use sf_tunables_bevy::{Tunables, TunablesAppExt};

/// `shadow.lamp.day_gate` (ROADMAP 1.23b), alias `BEVYRROWIND_LAMP_DAY_GATE=1`.
pub const LAMP_DAY_GATE: Def<bool> = sf_sky_bevy::DAY_GATE.env_alias("BEVYRROWIND_LAMP_DAY_GATE");
/// `shadow.lamp.face_cull` (CW-3b), alias `BEVYRROWIND_POINT_FACE_CULL=1`.
pub const POINT_FACE_CULL: Def<bool> = sf_sky_bevy::FACE_CULL.env_alias("BEVYRROWIND_POINT_FACE_CULL");
/// `shadow.sun.gate` (CW-8), alias `BEVYRROWIND_SUN_SHADOW_GATE=1`.
pub const SUN_SHADOW_GATE: Def<bool> = sun::GATE.env_alias("BEVYRROWIND_SUN_SHADOW_GATE");
/// `shadow.lamp.count`, alias `BEVYRROWIND_POINT_SHADOWS=<n>` (`--point-shadows` wins).
pub const POINT_SHADOWS: Def<usize> = sf_sky_bevy::LAMP_COUNT.env_alias("BEVYRROWIND_POINT_SHADOWS");
/// `shadow.lamp.contact`, alias `BEVYRROWIND_LAMP_CONTACT=1|0`.
pub const LAMP_CONTACT: Def<bool> = sf_sky_bevy::LAMP_CONTACT.env_alias("BEVYRROWIND_LAMP_CONTACT");
/// `shadow.filter`, alias `BEVYRROWIND_SHADOW_FILTER=hardware|gaussian|temporal`.
pub const SHADOW_FILTER: Def<&'static str> = q::FILTER.env_alias("BEVYRROWIND_SHADOW_FILTER");
/// `render.ssao.quality`, alias `BEVYRROWIND_SSAO=low|medium|high|ultra`.
pub const SSAO: Def<&'static str> = q::SSAO.env_alias("BEVYRROWIND_SSAO");

/// `render.upscaling`, alias `BEVYRROWIND_DLSS=<mode>` (`--dlss` wins).
pub const UPSCALING: Def<&'static str> = q::UPSCALING.env_alias("BEVYRROWIND_DLSS");

/// The alias copies, registered before the framework's defs.
pub const ALIASES: &[&dyn AnyDef] =
    &[&LAMP_DAY_GATE, &POINT_FACE_CULL, &SUN_SHADOW_GATE, &POINT_SHADOWS, &LAMP_CONTACT, &SHADOW_FILTER, &SSAO, &UPSCALING];

/// The builtin mod's records (`mw_mods::BASE_RECORDS`), parsed once: apps without a loaded profile
/// (viewers, tests) take Bevyrrowind's `Mod`-layer values and the graphics presets from them.
pub fn base_records() -> &'static RecordDb {
    static DB: OnceLock<RecordDb> = OnceLock::new();
    DB.get_or_init(mw_mods::base_record_db)
}

/// Writes `db`'s `sf.tunable` records at the `Mod` layer, replacing what the layer held (load order:
/// the later mod wins). Returns the refused ones (unknown names, bad values).
pub fn apply_mod_records(tunables: &mut Tunables, db: &RecordDb) -> Vec<String> {
    tunables.clear_layer(SetBy::Mod);
    let values = sf_tunables_bevy::presets::mod_values(db);
    sf_tunables_bevy::sf_tunables::presets::apply_mod_values(tunables, values.iter().map(|(n, r, m)| (n.as_str(), r, m.as_str())))
}

/// Bevyrrowind's lamp shadow fade band, m: the builtin mod's `shadow.lamp.fade_m` (over the
/// framework's 15-25 m), for budgets built without the tunables (the character viewer) or before they bind.
pub fn base_lamp_fade() -> [f32; 2] {
    let record = base_records().get::<sf_tunables_bevy::sf_tunables::presets::TunableRecord>(&sf_records::RecordId::new(sf_sky_bevy::LAMP_FADE.name()));
    let parsed = record.and_then(|r| {
        let (a, b) = r.value.text().split_once(',').map(|(a, b)| (a.trim().parse().ok(), b.trim().parse().ok()))?;
        Some([a?, b?])
    });
    parsed.unwrap_or_else(|| sf_sky_bevy::LAMP_FADE.default_value())
}

/// Sets `layer` of a registered tunable from game code, with a note (who and why). A refused value is
/// a bug in the caller (a table out of the def's range): it panics.
pub fn set_layer<T: Type + Sync>(tunables: &mut Tunables, def: &Def<T>, value: T, layer: SetBy, note: &str) {
    let name = AnyDef::name(def);
    let id = tunables.id_of(def).unwrap_or_else(|e| panic!("tunable {name}: {e}"));
    tunables
        .set_noted(id.untyped(), value.to_value(), layer, Some(note.to_owned()))
        .unwrap_or_else(|e| panic!("tunable {name}: {e}"));
}

/// Registers the alias copies (their environment switches apply now), Bevyrrowind's `Mod` layer, the
/// base `Preset` lamp budget (`preset`: count, face size) and the command-line sugar (`--dlss`:
/// `upscaling`; `upscaling_base`: the start's resolved upscaling, the base preset).
pub fn register(
    app: &mut App,
    preset: (usize, usize),
    point_shadows: Option<usize>,
    sun_cascades: Option<(usize, f32)>,
    (upscaling_base, upscaling): (sf_sky_bevy::sf_sky::Upscaling, Option<sf_sky_bevy::sf_sky::Upscaling>),
) {
    app.register_tunables(ALIASES).register_tunables(sf_sky_bevy::SHADOW_BUDGET_TUNABLES).register_tunables(q::TUNABLES);
    let mut tunables = app.world_mut().resource_mut::<Tunables>();
    let t = &mut *tunables;
    // The shipped `sf.tunable` records; a name another plugin registers later is the host's to apply
    // (the exterior re-applies the profile's records once everything registered).
    let _later = apply_mod_records(t, base_records());
    set_layer(t, &POINT_SHADOWS, preset.0, SetBy::Preset, "base preset (High)");
    set_layer(t, &sf_sky_bevy::LAMP_MAP_SIZE, preset.1, SetBy::Preset, "base preset (High)");
    set_layer(t, &UPSCALING, upscaling_base.label(), SetBy::Preset, "base preset (the start's upscaling)");
    if let Some(count) = point_shadows {
        set_layer(t, &POINT_SHADOWS, count, SetBy::CommandLine, "--point-shadows");
    }
    if let Some((count, meters)) = sun_cascades {
        set_layer(t, &q::SUN_CASCADES, count, SetBy::CommandLine, "--sun-cascades / BEVYRROWIND_SUN_CASCADES");
        set_layer(t, &q::SUN_DISTANCE, meters, SetBy::CommandLine, "--sun-cascades / BEVYRROWIND_SUN_CASCADES");
    }
    if let Some(upscaling) = upscaling {
        set_layer(t, &UPSCALING, upscaling.label(), SetBy::CommandLine, "--dlss");
    }
}
