//! Bevyrrowind's side of the shadow and light tunables (tunables slice T4, spec
//! `superfargoth/docs/specs/tunables.md` "Migration"; the framework declares and binds them in
//! `sf_sky_bevy::quality`):
//!
//! - **Env aliases.** The game's copies of framework defs carry the old `BEVYRROWIND_*` switches as
//!   deprecated aliases (`Def::env_alias`, `CommandLine` layer). The framework never names them, so
//!   the copies register first ([`register`], before `ShadowQualityPlugin` and `SkyPlugin`): an alias
//!   applies only where its def is registered fresh.
//! - **Mod layer.** Bevyrrowind's own value above a framework default (census "two truths"): the lamp
//!   fade band. Interim code-side write; T5 moves it to a base-mod `sf.tunable` record.
//! - **Preset layer.** The viewers' base preset (the High lamp budget); the exterior's quality table
//!   (`mw_exterior_bevy::quality_presets`) rewrites the layer for its `Quality`.
//! - **Command line.** `--point-shadows` and `--sun-cascades` (and `BEVYRROWIND_SUN_CASCADES`, a
//!   `COUNT:METERS` pair no single alias parses) are sugar onto the `CommandLine` layer.

use bevy_app::App;
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

/// The alias copies, registered before the framework's defs.
pub const ALIASES: &[&dyn AnyDef] =
    &[&LAMP_DAY_GATE, &POINT_FACE_CULL, &SUN_SHADOW_GATE, &POINT_SHADOWS, &LAMP_CONTACT, &SHADOW_FILTER, &SSAO];

/// Bevyrrowind's lamp shadow fade band, m (`Mod` layer over the framework's 15-25 m): 35-45 m covers
/// Balmora street blocks without nearby pop-in.
pub const LAMP_FADE_M: [f32; 2] = [35.0, 45.0];

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
/// base `Preset` lamp budget (`preset`: count, face size) and the command-line sugar.
pub fn register(app: &mut App, preset: (usize, usize), point_shadows: Option<usize>, sun_cascades: Option<(usize, f32)>) {
    app.register_tunables(ALIASES).register_tunables(sf_sky_bevy::SHADOW_BUDGET_TUNABLES).register_tunables(q::TUNABLES);
    let mut tunables = app.world_mut().resource_mut::<Tunables>();
    let t = &mut *tunables;
    set_layer(t, &sf_sky_bevy::LAMP_FADE, LAMP_FADE_M, SetBy::Mod, "bevyrrowind: Balmora street blocks without nearby pop-in");
    set_layer(t, &POINT_SHADOWS, preset.0, SetBy::Preset, "base preset (High)");
    set_layer(t, &sf_sky_bevy::LAMP_MAP_SIZE, preset.1, SetBy::Preset, "base preset (High)");
    if let Some(count) = point_shadows {
        set_layer(t, &POINT_SHADOWS, count, SetBy::CommandLine, "--point-shadows");
    }
    if let Some((count, meters)) = sun_cascades {
        set_layer(t, &q::SUN_CASCADES, count, SetBy::CommandLine, "--sun-cascades / BEVYRROWIND_SUN_CASCADES");
        set_layer(t, &q::SUN_DISTANCE, meters, SetBy::CommandLine, "--sun-cascades / BEVYRROWIND_SUN_CASCADES");
    }
}
