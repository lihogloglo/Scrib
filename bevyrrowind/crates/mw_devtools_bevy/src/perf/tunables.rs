//! Tunables overrides of a game run (`superfargoth/docs/specs/tunables.md` "Live in Bevy", slice T3):
//! `--set name=value` and `--tunables <file.toml>` (view flags, [`super::PerfFlags`]) and, in
//! interactive runs only, the user dev file `<user data>/tunables.toml` (UE `ConsoleVariables.ini`),
//! all at the `CommandLine` layer, applied once every plugin registered its tunables
//! ([`sf_tunables_bevy::CommandLineOverrides`]). A refused name or value fails the run before the
//! first frame ([`FailOnRefusals`]): a typo must not silently measure the default. Benchmark runs
//! record every non-default value in their report ([`TUNABLES_META`]).
//!
//! [`CATALOG`] lists every crate's `TUNABLES` slice, for `bevyrrowind tunables --md` (no window, no
//! app); a run warns about a declared tunable missing from it.

use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_log::{error, info, warn};
use sf_diag_bevy::DiagSettings;
use sf_tunables_bevy::overrides::{OverrideSet, parse_file, parse_set};
use sf_tunables_bevy::sf_tunables::{self, AnyDef, SetBy};
use sf_tunables_bevy::{CommandLineOverrides, OverrideRefusals, Tunables};

/// Report metadata key: every tunable not at its default, `name=value (layer: note)`, `; `-separated
/// (`none` when all are defaults), so a ledger line can be reproduced.
pub const TUNABLES_META: &str = "tunables";

/// The file name of the user dev file in the user data folder.
pub const USER_FILE_NAME: &str = "tunables.toml";

/// Every crate's `TUNABLES` list (crate, list). A crate that declares tunables adds its list here, so
/// `bevyrrowind tunables --md` shows them; a run logs a warning for a declared tunable that isn't.
pub const CATALOG: &[(&str, &[&dyn AnyDef])] = &[
    // Game copies carrying the old env names first: a def listed twice keeps its first metadata.
    ("mw_environment_bevy::shadow_tunables", mw_environment_bevy::shadow_tunables::ALIASES),
    ("mw_assets", mw_assets::TUNABLES),
    ("sf_character_bevy", sf_character_bevy::TUNABLES),
    ("sf_world", sf_world::TUNABLES),
    ("sf_water_bevy", mw_environment_bevy::water::WATER_TUNABLES),
    ("mw_water::sea_state", mw_environment_bevy::water::SEA_STATE_TUNABLES),
    ("mw_water::optics", mw_environment_bevy::water::WATER_OPTICS_TUNABLES),
    ("sf_lod_bevy", sf_lod_bevy::TUNABLES),
    ("sf_rockgen::boulder", sf_rockgen::boulder::tunables::TUNABLES),
    ("sf_rockgen_bevy", sf_rockgen_bevy::tunables::TUNABLES),
    ("sf_terrain_bevy", sf_terrain_bevy::TUNABLES),
    ("sf_groundcover", sf_groundcover::tunables::TUNABLES),
    ("sf_groundcover_bevy", sf_groundcover_bevy::tunables::TUNABLES),
    ("sf_atmos_bevy", sf_atmos_bevy::TUNABLES),
    ("sf_sky::lamp_cache", sf_sky_bevy::sf_sky::lamp_cache::TUNABLES),
    ("sf_sky::shadow_atlas", sf_sky_bevy::sf_sky::shadow_atlas::TUNABLES),
    ("sf_sky::sun_gate", sf_sky_bevy::sf_sky::sun_gate::TUNABLES),
    ("sf_sky_bevy::shadow_budget", sf_sky_bevy::SHADOW_BUDGET_TUNABLES),
    ("sf_sky_bevy::quality", sf_sky_bevy::quality::TUNABLES),
    ("mw_devtools_bevy::perf_panel", crate::perf_panel::TUNABLES),
    ("mw_devtools_bevy::switch_probe", crate::switch_probe::TUNABLES),
    ("mw_devtools_bevy::perf::flip_ab", crate::perf::flip_ab::TUNABLES),
    ("mw_environment_bevy::quality_tunables", mw_environment_bevy::quality_tunables::TUNABLES),
    ("mw_environment_bevy::worldgen_water", mw_environment_bevy::worldgen_water::TUNABLES),
    ("sf_water::swim", mw_environment_bevy::swim::TUNABLES),
    ("sf_tunables::presets", sf_tunables::presets::TUNABLES),
    ("sf_ai_bevy::perception", sf_ai_bevy::perception::PERCEPTION_TUNABLES),
    ("sf_ai_bevy::exec", sf_ai_bevy::exec::TUNABLES),
    ("sf_ai_bevy::alloc", sf_ai_bevy::alloc::ALLOC_TUNABLES),
    ("sf_crime_bevy", sf_crime_bevy::TUNABLES),
    ("mw_walkers_bevy::head_track", mw_walkers_bevy::head_track::TUNABLES),
    ("mw_walkers_bevy::light_inputs", mw_walkers_bevy::light_inputs::TUNABLES),
    ("mw_walkers_bevy::noise_emitters", mw_walkers_bevy::noise_emitters::TUNABLES),
    ("mw_walkers_bevy::walkers", mw_walkers_bevy::walkers::TUNABLES),
];

/// Every [`CATALOG`] def registered in a fresh store (defaults only): the source of
/// `bevyrrowind tunables --md` / `--json`. A def listed twice is registered once.
pub fn catalog_store() -> Result<sf_tunables::Tunables, String> {
    let mut store = sf_tunables::Tunables::default();
    for (krate, def) in catalog_defs() {
        if store.id(def.name()).is_none() {
            store.register_any(def).map_err(|e| format!("{krate}: {}: {e}", def.name()))?;
        }
    }
    Ok(store)
}

/// `docs/tunables-index.md`: a generated header, the catalog's dump table, then the graphics preset
/// levels (the builtin mod's `sf.tunable_preset` records, this build's capabilities).
pub fn index_markdown() -> Result<String, String> {
    let store = catalog_store()?;
    let crates: Vec<&str> = CATALOG.iter().map(|(k, _)| *k).collect();
    Ok(format!(
        "# Tunables index\n\n\
         Generated by `bevyrrowind tunables --md` (do not edit by hand; regenerate after adding or changing a tunable). \
         Every tunable a crate declares in its `TUNABLES` list ({}), at its default: set one for a run with \
         `--set name=value` or a `--tunables <file.toml>`, live in the console (`<name> <value>`, `help <name>`, \
         `tunables diff`). Spec: `superfargoth/docs/specs/tunables.md`.\n\n{}\n{}",
        crates.join(", "),
        store.dump_markdown(),
        levels_markdown(&store)
    ))
}

/// The value each graphics preset level writes, per `sg.*` group (the `Preset` layer; switch with
/// `preset <level>`, `set sg.<group> <level>`, the options menu's Graphics page).
fn levels_markdown(store: &sf_tunables::Tunables) -> String {
    use sf_tunables::presets::{LEVELS, MergedPresets, PresetBook};
    let db = mw_environment_bevy::shadow_tunables::base_records();
    let records = sf_tunables_bevy::presets::preset_records(db);
    let merged = MergedPresets::merge(records.iter().map(|(r, m)| (r, m.as_str())), mw_environment_bevy::render::capabilities());
    let (book, _) = PresetBook::build(store, &merged);
    let mut out = format!(
        "## Graphics preset levels\n\n\
         `sf.graphics_preset` sets every `sg.*` group to its level; each group writes these values at the `Preset` \
         layer (records: `bevyrrowind/crates/mw_mods/data/records/graphics_presets.ron`; this build: {}).\n\n\
         | group | tunable | {} |\n|---|---|{}\n",
        if cfg!(feature = "dlss") { "with DLSS" } else { "without DLSS" },
        LEVELS.join(" | "),
        "---|".repeat(LEVELS.len())
    );
    for driver in book.drivers.iter().filter(|d| d.levels.iter().all(|l| l.values.iter().all(|m| !m.driver))) {
        let mut names: Vec<&str> = driver.levels.iter().flat_map(|l| l.values.iter().map(|m| m.name.as_str())).collect();
        names.sort_unstable();
        names.dedup();
        for name in names {
            let cells: Vec<String> = LEVELS
                .iter()
                .map(|level| {
                    let value = driver.level(level).and_then(|l| l.values.iter().find(|m| m.name == name));
                    value.map_or_else(|| "-".to_owned(), |m| m.value.to_string())
                })
                .collect();
            let _ = writeln!(out, "| `{}` | `{name}` | {} |", driver.name, cells.join(" | "));
        }
    }
    out
}

/// `bevyrrowind tunables --json`: the catalog's dump rows.
pub fn index_json() -> Result<String, String> {
    Ok(sf_tunables_bevy::brp::rows_json(&catalog_store()?))
}

static USER_FILE: OnceLock<PathBuf> = OnceLock::new();

/// Names the user dev file (`<user data>/tunables.toml`); the app calls it once, for interactive
/// commands only. Benchmark runs ignore it whatever is set ([`run_overrides`]).
pub fn set_user_file(path: PathBuf) {
    let _ = USER_FILE.set(path);
}

/// The user dev file named by [`set_user_file`], if any.
pub fn user_file() -> Option<&'static Path> {
    USER_FILE.get().map(PathBuf::as_path)
}

/// Checks a `--set` value's shape, `name=value` (the perf harness, before it starts runs; the game
/// checks the name and value).
pub fn check_set(arg: &str) -> Result<(), String> {
    parse_set(arg).map(|_| ()).map_err(|e| e.to_string())
}

/// A run's override sets, weakest first: the user dev file (interactive runs only: `bench` false,
/// and only if it exists), each `--tunables` file in order, then `--set` (on the same layer a
/// later set wins). A bad `--set`, an unreadable or malformed file is an error.
pub fn run_overrides(set: &[String], files: &[PathBuf], bench: bool, user_file: Option<&Path>) -> Result<Vec<OverrideSet>, String> {
    let mut sets = Vec::new();
    let read = |path: &Path| -> Result<Vec<sf_tunables_bevy::overrides::Override>, String> {
        let text = std::fs::read_to_string(path).map_err(|e| format!("--tunables {}: {e}", path.display()))?;
        parse_file(&text).map_err(|e| format!("{}: {e}", path.display()))
    };
    if let Some(path) = user_file.filter(|p| !bench && p.is_file()) {
        sets.push(OverrideSet { origin: format!("user file {}", path.display()), overrides: read(path)? });
    }
    for path in files {
        sets.push(OverrideSet { origin: format!("file {}", path.display()), overrides: read(path)? });
    }
    if !set.is_empty() {
        let overrides = set.iter().map(|a| parse_set(a).map_err(|e| e.to_string())).collect::<Result<Vec<_>, _>>()?;
        sets.push(OverrideSet { origin: "--set".to_owned(), overrides });
    }
    Ok(sets)
}

/// The run's overrides, the refusal check, the catalog check and (benchmarks) the report section.
pub(super) fn add(app: &mut App, sets: Vec<OverrideSet>, bench: bool) {
    // Plugin order is `finish` order: apply, then check.
    app.add_plugins((CommandLineOverrides(sets), FailOnRefusals));
    if bench {
        app.add_systems(Update, record_tunables);
    }
}

/// Fails the run when an override was refused (`--set`, a tunables file, an env alias): logs every
/// refusal and exits with code 2 before the first frame. Add after [`CommandLineOverrides`].
pub struct FailOnRefusals;

impl Plugin for FailOnRefusals {
    fn build(&self, _: &mut App) {}

    fn finish(&self, app: &mut App) {
        let world = app.world();
        if let Some(tunables) = world.get_resource::<Tunables>() {
            for name in uncatalogued(tunables) {
                warn!("tunable {name} is declared but not in mw_devtools_bevy::perf::tunables::CATALOG: `bevyrrowind tunables --md` won't list it");
            }
        }
        let Some(refused) = world.get_resource::<OverrideRefusals>().filter(|r| !r.0.is_empty()) else { return };
        for r in &refused.0 {
            error!("tunable override refused: {r}");
            eprintln!("error: tunable override refused: {r}");
        }
        eprintln!("error: {} tunable override(s) refused; fix the name or value (`bevyrrowind tunables --md` lists them)", refused.0.len());
        std::process::exit(2);
    }
}

/// Every catalogued def: the [`CATALOG`] slices, then the live A/B switches' tunables
/// (`live_switches::LIVE`, built per row, so not a `TUNABLES` slice of their own).
fn catalog_defs() -> impl Iterator<Item = (&'static str, &'static dyn AnyDef)> {
    let listed = CATALOG.iter().flat_map(|(krate, defs)| defs.iter().map(move |d| (*krate, *d)));
    let live = crate::live_switches::LIVE.iter().map(|row| ("mw_devtools_bevy::live_switches", row.def as &'static dyn AnyDef));
    listed.chain(live)
}

/// Registered tunables declared with `def!` (they carry a source location) that no catalogued list
/// holds ([`catalog_defs`]). Console variables (no location) are not defs.
fn uncatalogued(tunables: &sf_tunables::Tunables) -> Vec<String> {
    tunables
        .iter()
        .filter(|(_, t)| t.meta().location.is_some())
        .filter(|(_, t)| !catalog_defs().any(|(_, d)| d.name() == t.name()))
        .map(|(_, t)| t.name().to_owned())
        .collect()
}

/// `name=value (layer: note)` for every tunable not at its default, sorted; `none` when all are.
pub fn non_default_label(tunables: &sf_tunables::Tunables) -> String {
    let parts: Vec<String> = tunables
        .rows()
        .into_iter()
        .filter(|r| r.set_by != SetBy::Default.describe())
        .map(|r| match r.note {
            Some(n) => format!("{}={} ({}: {n})", r.name, r.value, r.set_by),
            None => format!("{}={} ({})", r.name, r.value, r.set_by),
        })
        .collect();
    if parts.is_empty() { "none".to_owned() } else { parts.join("; ") }
}

/// Keeps the report's [`TUNABLES_META`] current (it is written at the end of the run).
fn record_tunables(tunables: Option<Res<Tunables>>, settings: Option<ResMut<DiagSettings>>, mut seen: Local<Option<u64>>) {
    let (Some(tunables), Some(mut settings)) = (tunables, settings) else { return };
    if *seen == Some(tunables.revision()) {
        return;
    }
    *seen = Some(tunables.revision());
    let label = non_default_label(&tunables);
    if *seen == Some(0) || settings.meta.get(TUNABLES_META) != Some(&label) {
        info!("tunables not at default: {label}");
    }
    settings.meta.insert(TUNABLES_META.to_owned(), label);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp(name: &str, text: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("mw-tunables-{}-{name}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join(USER_FILE_NAME);
        std::fs::write(&path, text).unwrap();
        path
    }

    #[test]
    fn a_bench_run_ignores_the_user_dev_file() {
        let user = temp("bench", "shadow.lamp.count = 4\n");
        let interactive = run_overrides(&[], &[], false, Some(&user)).unwrap();
        assert_eq!(interactive.len(), 1);
        assert!(interactive[0].origin.starts_with("user file "));
        assert_eq!(interactive[0].overrides[0].name, "shadow.lamp.count");
        assert!(run_overrides(&[], &[], true, Some(&user)).unwrap().is_empty());
        // Passed explicitly, a bench run reads it.
        let explicit = run_overrides(&[], std::slice::from_ref(&user), true, Some(&user)).unwrap();
        assert_eq!(explicit.len(), 1);
        assert!(explicit[0].origin.starts_with("file "));
        // A missing user file is no error.
        assert!(run_overrides(&[], &[], false, Some(Path::new("no/such/tunables.toml"))).unwrap().is_empty());
    }

    #[test]
    fn sets_come_last_and_bad_ones_are_errors() {
        let file = temp("order", "[select]\ngate = true\n");
        let sets = run_overrides(&["select.gate=off".into()], std::slice::from_ref(&file), false, None).unwrap();
        assert_eq!(sets.iter().map(|s| s.origin.split(' ').next().unwrap()).collect::<Vec<_>>(), ["file", "--set"]);
        assert_eq!(sets[1].overrides[0].value, "off");
        assert!(run_overrides(&["select.gate".into()], &[], false, None).unwrap_err().contains("expected name=value"));
        let bad = temp("bad", "not toml = = 1\n");
        assert!(run_overrides(&[], &[bad], false, None).is_err());
    }

    #[test]
    fn the_catalog_registers_and_every_tunable_is_listed_once() {
        let store = catalog_store().unwrap();
        assert!(store.len() >= CATALOG.len());
        assert!(uncatalogued(&store).is_empty());
        assert_eq!(non_default_label(&store), "none");
    }

    /// AI48: the AI perception, light and noise tunables the game registers are catalogued.
    #[test]
    fn the_catalog_lists_the_ai_perception_light_and_noise_tunables() {
        const PERCEPTION: &[&str] = &[
            "sight_range_m", "lose_sight_range_m", "near_m", "central_half_deg", "peripheral_half_deg",
            "peripheral_mul", "rate", "skip_gain", "sight_period_s", "tier1_period_s", "max_rays_per_tick",
            "sight_cache_s", "cache_move_m", "search_ttl_s", "search_error_m", "search_widen_mps",
            "search_ring_points", "search_look_half_deg", "search_uncertain_mul", "search_inner_frac",
            "closed_door_m", "light_search_m", "light_rays", "dark_threshold", "roof_sky_mul", "light_falloff_exp",
            "noise_max_hops",
        ];
        const AWARENESS: &[&str] = &[
            "curious", "suspicious", "searching", "band_hysteresis", "alert_hold_s", "decay_delay_s",
            "decay_curious", "decay_suspicious", "decay_searching", "noise_gain", "noise_cap", "noise_confidence",
            "asleep_wake_m", "groggy_s", "max_entries", "distraction_ttl_s", "alert_alertness", "groggy_alertness",
            "notify_speed_mps", "notify_react_s", "notify_confidence_mul", "notify_max_hops",
        ];
        const LIGHT: &[&str] = &["sky_dark_lux", "sky_day_lux", "interior_ambient_ref", "source_lift_m", "lamp_intensity"];
        const NOISE: &[&str] = &[
            "sneak_step_m", "walk_step_m", "run_step_m", "sprint_step_m", "sneak_stride_m", "walk_stride_m",
            "run_stride_m", "sprint_stride_m", "land_min_drop_m", "land_base_m", "land_per_drop", "land_max_m",
        ];
        let store = catalog_store().unwrap();
        let groups = [("ai.perception.", PERCEPTION), ("ai.awareness.", AWARENESS), ("mw.ai.light.", LIGHT), ("mw.ai.noise.", NOISE)];
        for (prefix, keys) in groups {
            for key in keys {
                let name = format!("{prefix}{key}");
                assert!(store.id(&name).is_some(), "{name} is not in CATALOG");
            }
        }
        for name in ["ai.exec.stuck_timeout_s", "mw.ai.wander.walk_slack_s"] {
            assert!(store.id(name).is_some(), "{name} is not in CATALOG");
        }
    }

    #[test]
    fn a_refused_set_is_collected_for_the_run_to_fail() {
        let mut app = App::new();
        app.add_plugins(CommandLineOverrides(run_overrides(&["select.gat=on".into(), "select.gate=maybe".into()], &[], true, None).unwrap()));
        app.world_mut().get_resource_or_init::<Tunables>().register_all(sf_world::TUNABLES).unwrap();
        app.finish();
        let refused = &app.world().resource::<OverrideRefusals>().0;
        assert_eq!(refused.iter().map(|r| r.name.as_str()).collect::<Vec<_>>(), ["select.gat", "select.gate"]);
        let tunables = app.world().resource::<Tunables>();
        assert_eq!(non_default_label(tunables), "none");
    }
}
