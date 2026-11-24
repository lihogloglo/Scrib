//! Graphics presets as data (tunables slices T4, T5a, T5b; spec
//! `superfargoth/docs/specs/tunables.md` "Presets are bundles of tunables"): the levels live in the
//! builtin mod's `sf.tunable_preset` records (`mw_mods::BASE_RECORDS`, `graphics_presets.ron`; a mod
//! changes them with its own records), one per UE-style scalability group (`sg.view_distance`,
//! `sg.shadows`, `sg.lighting`, `sg.post`, `sg.textures`, `sg.effects`, `sg.water`, `sg.sky`) and the
//! overall `sf.graphics_preset`, whose levels (`extremely_low`, `low`, `medium`, `high`, `ultra`) set
//! the groups. At startup [`apply`] writes the run's [`Quality`] level at the `Preset` layer; a console
//! `set` sits above it (`Console` layer), `--off` parts, flags and the env aliases at `CommandLine`.
//!
//! **Live switch:** the drivers are tunables (`sf_tunables_bevy::PresetsPlugin`): console
//! `set sf.graphics_preset ultra`, `set sg.shadows low`, BRP `sf.tunables/set`, `--set`, the
//! [`preset`](add_console) console command, the options menu's Graphics page
//! ([`crate::graphics_page`], levels saved per machine, T5d). A driver reads
//! `custom` when a member differs from every level (a hand-set value stays above the preset).
//!
//! What a value costs to change (its `Lifetime`): shadows, lights, GTAO, contact shadows, bloom, the
//! temporal resolve (prepasses and MSAA follow), clouds, water, puddles, shelter and parallax are live
//! (some re-specialise pipelines); the LOD bias and size culling follow their def's lifetime
//! (`Live` since T5c: resident LOD chains re-ranged in place, `crate::restream`); the far-field handoff scale, the
//! texture cap and the probe GI are read once at start (`Restart`, [`start_values`]): so Ultra
//! switched to live is Ultra minus its textures, and the console says so.
//!
//! Also the exterior's CW gates as live tunables ([`add_gates`]): `select.gate` (CW-2: streaming,
//! terrain, far field) and `render.skin.skip_unchanged` (CW-7), with their old env switches as aliases.

use std::collections::BTreeMap;

use bevy::log::warn;
use bevy::pbr::{DepthOnlyMaterialSettings, SkinUploadSettings};
use bevy::prelude::*;
use mw_devtools_bevy::perf::{Off, Part};
use sf_console_bevy::sf_console::{ArgKind, CommandSpec, Param};
use sf_records::RecordDb;
use sf_sky_bevy::quality as q;
use sf_tunables_bevy::overrides::{OverrideSet, apply_overrides};
use sf_tunables_bevy::sf_tunables::presets::{self as tp, MergedPresets, MergedValue, PresetBook, PresetRecord};
use sf_tunables_bevy::sf_tunables::Def;
use sf_tunables_bevy::{Presets, SetBy, Tunables, TunablesAppExt, Value};

pub use mw_environment_bevy::quality_tunables::{BIND_GROUP_WALK_ONCE, CLOUD_MAP_PER_SAMPLE, DEPTH_ONLY_SKIP_MATERIAL, GI_PROBES, PUDDLES, SHELTER, TEXTURE_MAX_SIZE};
pub use mw_game_core::exterior::Quality;

/// The overall driver's name (`sf.graphics_preset`).
pub const OVERALL: &str = tp::GRAPHICS_PRESET.name();

/// Every tunable the presets write that the render settings (`RenderSettings::insert`: shadows,
/// lights, GTAO, upscaling) don't register, and the drivers: register them before [`apply`].
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
    if !app.is_plugin_added::<sf_tunables_bevy::PresetsPlugin>() {
        app.add_plugins(sf_tunables_bevy::PresetsPlugin);
    }
}

/// `select.gate` (CW-2), alias `BEVYRROWIND_SELECT_GATE=1|0`; `--off selectgate` forces it off at
/// the `CommandLine` layer.
pub const SELECT_GATE: Def<bool> = sf_world::SELECT_GATE.env_alias("BEVYRROWIND_SELECT_GATE");

/// `render.skin.skip_unchanged` (CW-7), alias `BEVYRROWIND_SKIN_SKIP_UNCHANGED=1`.
pub const SKIN_SKIP_UNCHANGED: Def<bool> = sf_character_bevy::SKIN_SKIP_UNCHANGED.env_alias("BEVYRROWIND_SKIN_SKIP_UNCHANGED");

/// The preset records to use: the loaded profile's (`db`, when it holds any), else the builtin
/// mod's (apps without a profile, tests).
pub fn preset_db(db: Option<&RecordDb>) -> &RecordDb {
    match db {
        Some(db) if !db.is_empty::<PresetRecord>() => db,
        _ => mw_environment_bevy::shadow_tunables::base_records(),
    }
}

/// `db`'s presets merged for this build ([`mw_environment_bevy::render::capabilities`]: the
/// `sg.post` record for builds without DLSS).
pub fn merged(db: &RecordDb) -> MergedPresets {
    let records = sf_tunables_bevy::presets::preset_records(db);
    MergedPresets::merge(records.iter().map(|(r, m)| (r, m.as_str())), mw_environment_bevy::render::capabilities())
}

/// Every non-driver value `quality`'s level writes, from the builtin mod's records (before any app
/// exists: the first loads' parallax).
pub fn level_values(quality: Quality) -> BTreeMap<String, MergedValue> {
    merged(mw_environment_bevy::shadow_tunables::base_records()).flatten(OVERALL, quality.name())
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

/// Builds the presets from `db` ([`preset_db`]) and the loaded profile's `sf.tunable` values (at the
/// `Mod` layer), writes `quality`'s level at the `Preset` layer, the `--off` parts and
/// `--texture-cap <px>` (`texture_cap`, 0: none) at `CommandLine`. After `RenderSettings::insert`
/// and [`register`] (they register the tunables). Returns the book for `Presets`; problems in the
/// records (a mod's bad value, a missing level) are warnings.
pub fn apply(tunables: &mut Tunables, db: Option<&RecordDb>, quality: Quality, off: &Off, texture_cap: Option<u32>) -> PresetBook {
    if let Some(db) = db {
        for refused in mw_environment_bevy::shadow_tunables::apply_mod_records(tunables, db) {
            warn!("{refused}");
        }
    }
    let (book, issues) = PresetBook::build(tunables, &merged(preset_db(db)));
    for issue in issues {
        warn!("{issue}");
    }
    if let Err(e) = book.apply(tunables, OVERALL, quality.name()) {
        warn!("graphics preset {}: {e}", quality.name());
    }
    book.sync(tunables);
    for (part, name, value) in off_values() {
        if off.has(part) {
            set(tunables, name, value, SetBy::CommandLine, format!("--off {}", part.name()));
        }
    }
    if let Some(px) = texture_cap {
        set(tunables, TEXTURE_MAX_SIZE.name(), Value::Int(i64::from(px)), SetBy::CommandLine, "--texture-cap".to_owned());
    }
    book
}

/// The parallax new loads bind at a level (its `texture.parallax.*`), for `mw_assets::set_parallax`
/// before the app is built (the exterior then follows the tunables live).
pub fn level_parallax(quality: Quality) -> Option<mw_assets::Parallax> {
    let values = level_values(quality);
    let get = |name: &str| match values.get(name).map(|v| &v.value) {
        Some(tp::RecordValue::Int(v)) => u8::try_from(*v).unwrap_or(u8::MAX),
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
/// them and for the preset drivers (`--set sf.graphics_preset=ultra` starts with Ultra's texture cap;
/// the other overrides apply in `Plugin::finish`, once every plugin registered its tunables; these
/// are read before that). A refused override is left to `finish`, which reports it and fails the run.
pub fn start_values(tunables: &mut Tunables, book: &PresetBook, overrides: &[OverrideSet]) -> StartValues {
    // `render.depth_only.skip_material` too (vendored `superfargoth patch (own: depth-only no material)`):
    // `add_gates` reads it while the app is built.
    let restart = [TEXTURE_MAX_SIZE.name(), sf_lod_bevy::FAR_HANDOFF_SCALE.name(), GI_PROBES.name(), DEPTH_ONLY_SKIP_MATERIAL.name(), BIND_GROUP_WALK_ONCE.name()];
    let early = |name: &str| restart.contains(&name) || book.driver(name).is_some();
    for set in overrides {
        let sets: Vec<_> = set.overrides.iter().filter(|o| early(&o.name)).cloned().collect();
        let _refused = apply_overrides(tunables, &sets, SetBy::CommandLine, Some(&set.origin));
    }
    for (driver, level, result) in book.take_requests(tunables) {
        if let Err(e) = result {
            warn!("preset {driver} {level}: {e}");
        }
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

/// Binds the presets' game-side live tunables into what consumes them (each writes once its target
/// exists, then on a change): the streaming screen (live, resident chains re-ranged by T5c), the clouds'
/// weather-map sampling, puddles and shelter, the parallax of loaded materials, bloom on the views.
pub fn bind_live(app: &mut App) {
    use sf_world::lod_screen as l;
    use sf_world_bevy::Streaming;
    crate::restream::bind(app);
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
    // The trail ring's press (GC-T1, GC-T2b, GC-T3): live, for the owner's trample look.
    {
        use sf_surface_bevy::{TrailPressSettings as P, trail as t};
        let _recovery = app.add_tunable(&t::PRESS_RECOVERY_S).bind_resource(|s: &mut P, v| s.recovery_s = v);
        let _radius = app.add_tunable(&t::PRESS_RADIUS_M).bind_resource(|s: &mut P, v| s.radius_m = v);
        let _height = app.add_tunable(&t::PRESS_HEIGHT_M).bind_resource(|s: &mut P, v| s.height_m = v);
        let _spread = app.add_tunable(&t::PRESS_SPREAD_M).bind_resource(|s: &mut P, v| s.spread_m = v);
    }
    app.add_feature_switch(&mw_environment_bevy::quality_tunables::PUDDLES_SWITCH)
        .add_feature_switch(&mw_environment_bevy::quality_tunables::SHELTER_SWITCH);
    mw_assets::add_parallax_tunables(app);
    if !app.is_plugin_added::<mw_environment_bevy::quality_tunables::BloomPlugin>() {
        app.add_plugins(mw_environment_bevy::quality_tunables::BloomPlugin);
    }
}

/// The `preset` console command: no argument lists every driver's level (and what sits above the
/// preset); `preset <level>` switches the overall preset; `preset <group> <level>` one group
/// (`shadows` or `sg.shadows`). Prints what applies on restart or to new spawns.
pub fn add_console(app: &mut App) {
    sf_console_bevy::ConsoleAppExt::add_console_command(
        app,
        CommandSpec::new("preset")
            .help(
                "Graphics presets: `preset` lists the levels, `preset <level>` sets sf.graphics_preset \
                 (extremely_low, low, medium, high, ultra), `preset <group> <level>` one sg.* group",
            )
            .param(Param::new("level_or_group", ArgKind::Str).optional())
            .param(Param::new("level", ArgKind::Str).optional()),
        |inv| {
            let Some(book) = inv.ctx.get_resource::<Presets>().map(|p| p.0.clone()) else {
                return Err("no graphics presets in this app".to_owned());
            };
            let (first, second) = (inv.args.str("level_or_group"), inv.args.str("level"));
            let (driver, level) = match (first, second) {
                (None, _) => {
                    for line in preset_status(&book, inv.vars) {
                        inv.log.info(&line);
                    }
                    return Ok(());
                }
                (Some(level), None) => (OVERALL.to_owned(), level.to_ascii_lowercase()),
                (Some(group), Some(level)) => {
                    let group = group.to_ascii_lowercase();
                    let name = if book.driver(&group).is_some() { group } else { format!("sg.{group}") };
                    (name, level.to_ascii_lowercase().replace('-', "_"))
                }
            };
            let level = level.replace('-', "_");
            let applied = book.apply(inv.vars, &driver, &level)?;
            book.sync(inv.vars);
            inv.log.info(&format!("{driver} {level}: {}", applied.summary(inv.vars)));
            Ok(())
        },
    );
}

/// One line per driver: its level, and for a group the members set above the preset.
pub fn preset_status(book: &PresetBook, tunables: &sf_tunables_bevy::sf_tunables::Tunables) -> Vec<String> {
    book.drivers
        .iter()
        .map(|d| {
            let level = book.level_of(tunables, &d.name).unwrap_or(tp::CUSTOM);
            let above: Vec<String> = d
                .members
                .iter()
                .map(|id| tunables.var(*id))
                .filter(|t| t.set_by() > SetBy::Preset)
                .map(|t| format!("{} {} ({})", t.name(), t.value(), t.set_by().describe()))
                .collect();
            if above.is_empty() { format!("{}: {level}", d.name) } else { format!("{}: {level}; above the preset: {}", d.name, above.join(", ")) }
        })
        .collect()
}

/// The exterior's CW gates: registers their alias copies (env switches apply now), `--off selectgate`
/// (`select_gate_off`) at `CommandLine`, and binds them live into terrain, far field and streaming
/// settings, Bevy's skin upload settings and worldspace parking (CW-9). Returns `select.gate`'s effective value (the plugins'
/// starting settings). Also inserts Bevy's depth-only material switch (`render.depth_only.skip_material`,
/// read at start, not live): call it before the app runs.
pub fn add_gates(app: &mut App, select_gate_off: bool) -> bool {
    app.register_tunables(&[&SELECT_GATE, &SKIN_SKIP_UNCHANGED]);
    if select_gate_off {
        let mut tunables = app.world_mut().resource_mut::<Tunables>();
        set(&mut tunables, SELECT_GATE.name(), Value::Bool(false), SetBy::CommandLine, "--off selectgate".to_owned());
    }
    let _terrain = app.add_tunable(&SELECT_GATE).bind_resource(|t: &mut sf_terrain_bevy::TerrainSettings, on| t.select_gate.enabled = on);
    let _hex_on = app
        .add_tunable(&sf_terrain_bevy::TERRAIN_HEX_TILING)
        .bind_resource(|t: &mut sf_terrain_bevy::TerrainSettings, on| t.hex_tiling.enabled = on);
    let _hex_rot = app
        .add_tunable(&sf_terrain_bevy::TERRAIN_HEX_ROT_STRENGTH)
        .bind_resource(|t: &mut sf_terrain_bevy::TerrainSettings, v| t.hex_tiling.rot_strength = v);
    let _hex_contrast = app
        .add_tunable(&sf_terrain_bevy::TERRAIN_HEX_CONTRAST)
        .bind_resource(|t: &mut sf_terrain_bevy::TerrainSettings, v| t.hex_tiling.contrast = v);
    let _hex_scale = app
        .add_tunable(&sf_terrain_bevy::TERRAIN_HEX_SCALE)
        .bind_resource(|t: &mut sf_terrain_bevy::TerrainSettings, v| t.hex_tiling.scale = v);
    let _cliff_on = app
        .add_tunable(&sf_terrain_bevy::TERRAIN_CLIFF_ENABLED)
        .bind_resource(|t: &mut sf_terrain_bevy::TerrainSettings, on| t.cliff.enabled = on);
    let _cliff_start = app
        .add_tunable(&sf_terrain_bevy::TERRAIN_CLIFF_SLOPE_START_DEG)
        .bind_resource(|t: &mut sf_terrain_bevy::TerrainSettings, v| t.cliff.slope_start_deg = v);
    let _cliff_full = app
        .add_tunable(&sf_terrain_bevy::TERRAIN_CLIFF_SLOPE_FULL_DEG)
        .bind_resource(|t: &mut sf_terrain_bevy::TerrainSettings, v| t.cliff.slope_full_deg = v);
    let _cliff_crev = app
        .add_tunable(&sf_terrain_bevy::TERRAIN_CLIFF_CREVICE_STRENGTH)
        .bind_resource(|t: &mut sf_terrain_bevy::TerrainSettings, v| t.cliff.crevice_strength = v);
    // WG-LOD: shading from the finest resident height level (an A/B against the mesh-LOD coupling), its
    // cross-fade and curvature scale, and the CDLOD morph start; all live.
    let _shade_on = app
        .add_tunable(&sf_terrain_bevy::TERRAIN_SHADE_DECOUPLED)
        .bind_resource(|t: &mut sf_terrain_bevy::TerrainSettings, on| t.shade.decoupled = on);
    let _shade_blend = app
        .add_tunable(&sf_terrain_bevy::TERRAIN_SHADE_BLEND_TEXELS)
        .bind_resource(|t: &mut sf_terrain_bevy::TerrainSettings, v| t.shade.blend_texels = v);
    let _curv_scale = app
        .add_tunable(&sf_terrain_bevy::TERRAIN_CLIFF_CURVATURE_SCALE_M)
        .bind_resource(|t: &mut sf_terrain_bevy::TerrainSettings, v| t.shade.curvature_scale_m = v);
    let _morph_start = app
        .add_tunable(&sf_terrain_bevy::TERRAIN_MORPH_START)
        .bind_resource(|t: &mut sf_terrain_bevy::TerrainSettings, v| t.cdlod = t.cdlod.with_morph_start(v));
    let _far = app.add_tunable(&SELECT_GATE).bind_resource(|f: &mut sf_lod_bevy::FarFieldSettings, on| f.select_gate.enabled = on);
    let _streaming = app.add_tunable(&SELECT_GATE).bind_resource(|s: &mut sf_world_bevy::Streaming, on| {
        let quanta = sf_world::GateQuanta { enabled: on, ..s.config().scheduler.select_gate };
        s.set_select_gate(quanta);
    });
    // CW-9: worldspaces the player left (the exterior while indoors) park out of every query; live
    // both ways (`sf_world_bevy::WorldspaceParking`, park.rs).
    let _park = app
        .add_tunable(&sf_world::park::PARK_LEFT)
        .bind_resource(|p: &mut sf_world_bevy::WorldspaceParking, on| p.enabled = on);
    let _settle = app
        .add_tunable(&sf_world::park::PARK_SETTLE_S)
        .bind_resource(|p: &mut sf_world_bevy::WorldspaceParking, s| p.settle_s = s);
    app.init_resource::<SkinUploadSettings>();
    let _skin = app.add_tunable(&SKIN_SKIP_UNCHANGED).bind_resource(|s: &mut SkinUploadSettings, on| s.skip_unchanged = on);
    // `render.depth_only.skip_material` (vendored `superfargoth patch (own: depth-only no material)`): not
    // live. Bevy reads the resource once, when its material plugin finishes, so the value of this start
    // (`start_values` applied its `--set`) goes in now; unregistered (a host without `register`): off.
    let depth_only = {
        let tunables = app.world().resource::<Tunables>();
        tunables.id_of(&DEPTH_ONLY_SKIP_MATERIAL).is_ok_and(|id| tunables.read(id))
    };
    app.insert_resource(DepthOnlyMaterialSettings { skip_material: depth_only });
    // `render.wgpu.bind_group_walk_once` (vendored wgpu-core, `superfargoth patch (own: bind group walk
    // once)`): a process-wide switch, set once from the value of this start (default on); an unregistered
    // tunable (a host without `register`) leaves wgpu-core stock.
    let walk_once = {
        let tunables = app.world().resource::<Tunables>();
        tunables.id_of(&BIND_GROUP_WALK_ONCE).is_ok_and(|id| tunables.read(id))
    };
    bevy::render::renderer::encode_timing::set_bind_group_walk_once(walk_once);
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
        mw_environment_bevy::render::RenderSettings::new(false, None, true, quality)
            .expect("settings")
            .with_shadow_cache(false)
            .insert(&mut app);
        register(&mut app);
        let book = apply(&mut app.world_mut().resource_mut::<Tunables>(), None, quality, off, None);
        app.insert_resource(Presets(book));
        app.insert_resource(q::SunShadowQuality::default())
            .init_resource::<SkyRenderOptions>()
            .init_resource::<sf_terrain_bevy::TerrainSettings>()
            .init_resource::<sf_lod_bevy::FarFieldSettings>()
            .insert_resource(sf_world_bevy::Streaming::new(sf_world_bevy::StreamingConfig::default()))
            .insert_resource(mw_assets::MaterialMaps::new(true, true, true))
            .init_resource::<sf_surface_bevy::SurfaceSettings>()
            .init_resource::<sf_surface_bevy::TrailPressSettings>()
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
            let want = mw_environment_bevy::render::default_upscaling(quality);
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
            let book = app.world().resource::<Presets>().0.clone();
            let start = start_values(&mut app.world_mut().resource_mut::<Tunables>(), &book, &[]);
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

    /// The trail ring's press tunables (GC-T1, GC-T2b, GC-T3) reach `TrailPressSettings` live, and
    /// every one is in the framework crate's `TUNABLES`.
    #[test]
    #[allow(clippy::float_cmp)] // exact: parsed console values
    fn press_bindings_are_live() {
        use sf_surface_bevy::trail as t;
        let names = [&t::PRESS_RECOVERY_S, &t::PRESS_RADIUS_M, &t::PRESS_HEIGHT_M, &t::PRESS_SPREAD_M].map(Def::name);
        for name in names {
            assert!(sf_surface_bevy::TUNABLES.iter().any(|d| d.name() == name), "{name} registered");
        }
        let mut app = app(Quality::High, &Off::default(), &[]);
        assert_eq!(*app.world().resource::<sf_surface_bevy::TrailPressSettings>(), sf_surface_bevy::TrailPressSettings::default());
        for (name, value) in names.into_iter().zip(["3", "0.5", "1.2", "0.8"]) {
            console(&mut app, name, value);
        }
        let s = *app.world().resource::<sf_surface_bevy::TrailPressSettings>();
        assert_eq!((s.recovery_s, s.radius_m, s.height_m, s.spread_m), (3.0, 0.5, 1.2, 0.8));
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
        let book = apply(&mut t, None, Quality::Ultra, &off, Some(1024));
        let sets = [OverrideSet {
            origin: "--set".into(),
            overrides: vec![sf_tunables_bevy::overrides::Override { name: "far.handoff_scale".into(), value: "1.5".into() }],
        }];
        let start = start_values(&mut t, &book, &sets);
        assert_eq!(start, StartValues { texture_cap: Some(1024), far_handoff_scale: 1.5, gi: false });
        let _ = apply(&mut t, None, Quality::Ultra, &Off::default(), Some(0));
        assert_eq!(start_values(&mut t, &book, &[]).texture_cap, None, "--texture-cap 0: none");
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
        assert_eq!(read(&plain), (true, true, true, true), "on by default (owner 2026-11-17)");
        console(&mut plain, "render.skin.skip_unchanged", "off");
        assert!(!read(&plain).3);
    }

    /// The old `Quality` table (T5a's `quality_preset_values`, copied as it was): the golden fixture the
    /// shipped records must equal for Low..Ultra.
    fn old_table(quality: Quality) -> Vec<(&'static str, Value)> {
        use mw_environment_bevy::quality_tunables as qt;
        let high = matches!(quality, Quality::High | Quality::Ultra);
        let low = quality == Quality::Low;
        let (lamps, face): (i64, i64) = match quality {
            Quality::Low => (0, 512),
            Quality::Medium => (6, 512),
            Quality::High => (8, 512),
            _ => (12, 1024),
        };
        let ssao = match quality {
            Quality::Low => "off",
            Quality::Medium => "low",
            Quality::High => "medium",
            _ => "high",
        };
        let (cascades, distance, lambda, overlap): (i64, f64, f64, f64) = match quality {
            Quality::Low => (3, 160.0, 0.82, 0.15),
            Quality::Medium => (3, 190.0, 0.80, 0.15),
            Quality::High => (3, 205.0, 0.80, 0.16),
            _ => (4, 280.0, 0.85, 0.2),
        };
        let (bias, min_pixels, min_draw_distance): (f64, f64, f64) = match quality {
            Quality::Low => (0.75, 4.0, 16.0),
            Quality::Medium => (0.875, 3.0, 20.0),
            Quality::High => (1.0, 2.0, 24.0),
            _ => (1.0, 1.0, 32.0),
        };
        let (water, texture_cap, parallax_layers, relief_steps): (&str, i64, i64, i64) = match quality {
            Quality::Low => ("low", 512, 0, 0),
            Quality::Medium => ("medium", 1024, 16, 0),
            Quality::High => ("high", 2048, 32, 5),
            _ => ("ultra", 0, 64, 8),
        };
        // `default_upscaling`: MSAA on Low, else DLSS in DLSS builds (Quality on High and Ultra since owner 2026-11-17,
        // `auto` on Medium), TAA without.
        let upscaling = match (low, high, mw_environment_bevy::render::capabilities().contains(&"dlss")) {
            (true, _, _) => "off",
            (false, true, true) => "quality",
            (false, false, true) => "auto",
            (false, _, false) => "taa",
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
            (qt::BLOOM_INTENSITY.name(), Value::Float(0.1)),
            (q::UPSCALING.name(), text(upscaling)),
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

    /// Golden (T5b, presets as data): each of Low..Ultra in the builtin mod's records, flattened
    /// through `sf.graphics_preset` and its groups, equals the old table key for key (no more, no less).
    #[test]
    fn shipped_records_equal_the_old_table() {
        let app = app(Quality::High, &Off::default(), &[]);
        let t = app.world().resource::<Tunables>();
        for quality in [Quality::Low, Quality::Medium, Quality::High, Quality::Ultra] {
            let records = level_values(quality);
            let old = old_table(quality);
            assert_eq!(records.len(), old.len(), "{quality:?}: {:?}", records.keys().collect::<Vec<_>>());
            for (name, want) in old {
                let got = records.get(name).unwrap_or_else(|| panic!("{quality:?}: {name} missing"));
                let parsed = t.var(t.id(name).expect(name)).kind().parse(&got.value.text()).expect(name);
                assert!(tp::values_match(&parsed, &want), "{quality:?} {name}: records {parsed}, old {want}");
            }
        }
    }

    /// Extremely low: every group has it, writing the same keys as its other levels, each value in its
    /// def's range (the book refuses one that isn't); every key belongs to exactly one group, the one
    /// its def names; `default_upscaling` agrees with `sg.post` on every level.
    #[test]
    fn extremely_low_is_complete_valid_and_groups_match_their_defs() {
        let app = app(Quality::High, &Off::default(), &[]);
        let t = app.world().resource::<Tunables>();
        let (book, issues) = PresetBook::build(t, &merged(mw_environment_bevy::shadow_tunables::base_records()));
        assert!(issues.is_empty(), "{issues:#?}");
        assert_eq!(book.drivers.len(), 9, "the overall driver and eight groups");
        for d in &book.drivers {
            let names = |level: &str| -> Vec<String> {
                let mut n: Vec<String> =
                    d.level(level).unwrap_or_else(|| panic!("{} {level}", d.name)).values.iter().map(|m| m.name.clone()).collect();
                n.sort();
                n
            };
            for level in tp::LEVELS {
                assert_eq!(names(level), names("ultra"), "{} {level}: same keys", d.name);
            }
            for m in d.level("extremely_low").expect("level").values.iter().filter(|m| !m.driver) {
                assert_eq!(t.var(m.id).meta().group.map(sf_tunables_bevy::sf_tunables::Group::driver), Some(d.name.as_str()), "{}: its def's group", m.name);
                assert!(!m.source.is_empty(), "{} has a source", m.name);
            }
        }
        let xlow = level_values(Quality::ExtremelyLow);
        assert_eq!(xlow.len(), old_table(Quality::Low).len(), "every key set");
        let get = |name: &str| xlow[name].value.text();
        assert_eq!(
            (get("shadow.lamp.count"), get("shadow.sun.cascades"), get("lod.bias"), get("texture.max_size_px")),
            ("0".to_owned(), "1".to_owned(), "0.5".to_owned(), "256".to_owned())
        );
        for quality in Quality::ALL {
            let records = level_values(quality)[q::UPSCALING.name()].value.text();
            assert_eq!(records, mw_environment_bevy::render::default_upscaling(quality).label(), "{quality:?} upscaling");
        }
    }

    /// Extremely low starts as its records say, read where the game consumes them.
    #[test]
    #[allow(clippy::float_cmp)] // exact: values from the records
    fn extremely_low_reaches_the_consumers() {
        let app = app(Quality::ExtremelyLow, &Off::default(), &[]);
        let w = app.world();
        let b = w.resource::<sf_sky_bevy::PointShadowBudget>();
        assert_eq!((b.max, b.map_size, b.contact_shadows), (0, 256, false));
        assert_eq!(w.resource::<q::SunShadowQuality>().splits.num_cascades, 1);
        assert_eq!(w.resource::<SkyRenderOptions>().cascades.bounds.len(), 1);
        let lod = w.resource::<sf_world_bevy::Streaming>().config().lod;
        assert_eq!((lod.bias, lod.min_pixels, lod.min_draw_distance), (0.5, 8.0, 12.0));
        let t = w.resource::<Tunables>();
        assert_eq!(t.text(t.id(tp::GRAPHICS_PRESET.name()).expect("driver")), "extremely_low");
        assert_eq!(t.text(t.id("sg.water").expect("driver")), "extremely_low", "the level asked for, though Low's water is the same");
    }

    /// T5b live switch: the overall driver from the console reaches the consumers the next update;
    /// a hand change reads custom (overall and its group), its reset goes back; a `Restart` key is
    /// stored (and the summary says so); the `Mod` layer stays under the preset.
    #[test]
    #[allow(clippy::float_cmp)] // exact: values from the records
    fn the_overall_preset_switches_live() {
        let mut app = app(Quality::Low, &Off::default(), &[]);
        let driver = |app: &App, name: &str| {
            let t = app.world().resource::<Tunables>();
            t.text(t.id(name).expect(name)).to_owned()
        };
        assert_eq!(driver(&app, "sf.graphics_preset"), "low");
        console(&mut app, "sf.graphics_preset", "ultra");
        let w = app.world();
        assert_eq!(w.resource::<sf_sky_bevy::PointShadowBudget>().max, 12);
        assert_eq!(w.resource::<SkyRenderOptions>().cascades.bounds.len(), 4);
        assert_eq!(*w.resource::<mw_environment_bevy::quality_tunables::WaterQuality>(), mw_environment_bevy::quality_tunables::WaterQuality::Ultra);
        assert!(w.resource::<sf_surface_bevy::SurfaceSettings>().puddles);
        let b = w.resource::<sf_sky_bevy::PointShadowBudget>();
        assert_eq!((b.fade_start, b.fade_end), (35.0, 45.0), "the Mod layer stays");
        assert_eq!(driver(&app, "sf.graphics_preset"), "ultra");
        let t = w.resource::<Tunables>();
        let cap = t.var(t.id(TEXTURE_MAX_SIZE.name()).expect("cap"));
        assert_eq!((cap.value(), cap.set_by()), (&Value::Int(0), SetBy::Preset), "stored; read at the next start");
        let applied = app.world_mut().resource_mut::<Messages<sf_tunables_bevy::PresetApplied>>().drain().last().expect("applied");
        assert!(applied.summary.as_ref().is_ok_and(|s| s.contains("applies on restart: ") && s.contains("texture.max_size_px")), "{applied:?}");
        console(&mut app, "shadow.lamp.count", "2");
        assert_eq!((driver(&app, "sg.shadows"), driver(&app, "sf.graphics_preset"), driver(&app, "sg.water")), ("custom".into(), "custom".into(), "ultra".into()));
        let status = preset_status(&app.world().resource::<Presets>().0, app.world().resource::<Tunables>());
        assert!(status.iter().any(|l| l == "sg.shadows: custom; above the preset: shadow.lamp.count 2 (console)"), "{status:#?}");
        app.world_mut().write_message(sf_tunables_bevy::PeelTunable { name: "shadow.lamp.count".into(), layer: Some(SetBy::Console) });
        app.update();
        assert_eq!(driver(&app, "sf.graphics_preset"), "ultra", "back to the level");
        console(&mut app, "sg.shadows", "extremely_low");
        assert_eq!(app.world().resource::<sf_sky_bevy::PointShadowBudget>().max, 0);
        assert_eq!((driver(&app, "sg.shadows"), driver(&app, "sf.graphics_preset")), ("extremely_low".into(), "custom".into()));
    }

    /// `--set sf.graphics_preset=ultra` is a start level: the `Restart` keys start with Ultra's values.
    #[test]
    fn a_preset_set_on_the_command_line_sets_the_start_values() {
        let mut app = app(Quality::Low, &Off::default(), &[]);
        let book = app.world().resource::<Presets>().0.clone();
        let sets = [OverrideSet {
            origin: "--set".into(),
            overrides: vec![sf_tunables_bevy::overrides::Override { name: "sf.graphics_preset".into(), value: "ultra".into() }],
        }];
        let start = start_values(&mut app.world_mut().resource_mut::<Tunables>(), &book, &sets);
        assert_eq!(start, StartValues { texture_cap: None, far_handoff_scale: 1.0, gi: true });
    }

    /// FS-2b: the live bindings register the puddles, shelter and bloom switches; one follows a
    /// Console-layer set.
    #[test]
    fn feature_switches_registered() {
        let mut app = app(Quality::High, &Off::default(), &[]);
        let running = |app: &App, name: &str| {
            let switches = app.world().resource::<sf_tunables_bevy::FeatureSwitches>();
            assert!(switches.get(name).is_some(), "{name} registered");
            switches.running(&app.world().resource::<Tunables>().0, name)
        };
        for name in ["surface.puddles", "surface.shelter", "render.bloom"] {
            let _ = running(&app, name);
        }
        console(&mut app, "surface.puddles", "on");
        assert!(running(&app, "surface.puddles"));
        console(&mut app, "surface.puddles", "off");
        assert!(!running(&app, "surface.puddles"), "off at the console layer");
    }
}
