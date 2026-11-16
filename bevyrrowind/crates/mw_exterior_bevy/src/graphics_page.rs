//! The options menu's Graphics page (tunables slice T5d; spec `superfargoth/docs/specs/tunables.md`
//! "Presets are bundles of tunables", menus.md S4): one choice row for the overall graphics preset
//! and one per scalability group, each Extremely low .. Ultra, reading Custom when its values match
//! no level.
//!
//! - **Rows are records**: [`RECORDS`], `ui.setting`s whose keys are the preset drivers' tunable
//!   names (`sf.graphics_preset`, `sg.view_distance`, ...), page `graphics`, options the drivers'
//!   (`extremely_low` .. `ultra` and `custom`, which the screen shows but never steps onto),
//!   `scope = "Machine"` (graphics belong to the machine, UE's `GameUserSettings.ini`; menus.md
//!   "Settings v2"). `pause_menu::add` adds them to the game's registry; labels come from the
//!   builtin mod's Fluent file (`set-graphics-*`, `set-page-graphics`).
//! - **A pick applies live** ([`pick`]): the driver is set at the `User` layer, which the presets
//!   (`sf_tunables_bevy::drive_presets`) take as a level request next frame, as a console `set`:
//!   the level is written at `Preset`, the drivers re-derive. The world renders behind the pause
//!   menu, so the change shows at once; what applies on restart is logged (`PresetApplied`).
//! - **Rows show the drivers** ([`sync_rows`]): each graphics row of an open options screen follows
//!   its driver's effective level, so the overall pick moves the group rows, a group pick makes the
//!   overall read Custom, a console-set member makes its group read Custom. A row whose levels
//!   change a `Restart` tunable (texture cap, far handoff, probe GI) carries the screen's
//!   "Applies after restart" badge ([`touches_restart`]).
//! - **Saved are the picks**, not the shown levels: the store keeps the player's choices, kept
//!   consistent by the menu preset ([`menu_preset`], the overall level's group levels as a
//!   `ui.setting_preset`), and saved to `<user data>/machine.toml` when the menus close
//!   (`sf_ui_bevy::sync_machine_settings`). A command-line level or a console value is a run's
//!   override and is never saved. At start ([`apply_start_levels`]) the saved levels are level
//!   requests at `User`, then an explicit `--quality` at `CommandLine` (so it wins over them), then
//!   the run's `--set`s (`quality_presets::start_values`): `Restart` values start with the saved
//!   levels. Only interactive runs read them (benchmarks and screenshots don't, as the user dev
//!   file `tunables.toml`).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use bevy::log::{info, warn};
use bevy::prelude::*;
use sf_records::RecordDb;
use sf_tunables_bevy::sf_tunables::presets::{self as tp, PresetBook};
use sf_tunables_bevy::sf_tunables::{Lifetime, TunableId};
use sf_tunables_bevy::{Presets, SetBy, Tunables, Value};
use sf_ui::records::SettingPresetDef;
use sf_ui::{SettingValue, SettingsStore, UiRegistry};
use sf_ui_screens::OptionsScreen;

/// The page id (`set-page-graphics`).
pub const PAGE: &str = "graphics";

/// The page's rows: one `ui.setting` per preset driver (keys: the drivers' tunable names), in
/// page order. Defaults: High, the level a run starts at without `--quality` (`Quality::default`).
pub const RECORDS: &str = r#"
[[define]]
kind = "ui.setting"
id = "sf.graphics_preset"
value = { label = "set-graphics-preset", description = "set-graphics-preset-desc", page = "graphics", group = "preset", scope = "Machine", kind = { Choice = { default = "high", options = ["extremely_low", "low", "medium", "high", "ultra", "custom"] } } }

[[define]]
kind = "ui.setting"
id = "sg.view_distance"
value = { label = "set-graphics-view-distance", description = "set-graphics-view-distance-desc", page = "graphics", group = "groups", scope = "Machine", kind = { Choice = { default = "high", options = ["extremely_low", "low", "medium", "high", "ultra", "custom"] } } }

[[define]]
kind = "ui.setting"
id = "sg.shadows"
value = { label = "set-graphics-shadows", description = "set-graphics-shadows-desc", page = "graphics", group = "groups", scope = "Machine", kind = { Choice = { default = "high", options = ["extremely_low", "low", "medium", "high", "ultra", "custom"] } } }

[[define]]
kind = "ui.setting"
id = "sg.lighting"
value = { label = "set-graphics-lighting", description = "set-graphics-lighting-desc", page = "graphics", group = "groups", scope = "Machine", kind = { Choice = { default = "high", options = ["extremely_low", "low", "medium", "high", "ultra", "custom"] } } }

[[define]]
kind = "ui.setting"
id = "sg.post"
value = { label = "set-graphics-post", description = "set-graphics-post-desc", page = "graphics", group = "groups", scope = "Machine", kind = { Choice = { default = "high", options = ["extremely_low", "low", "medium", "high", "ultra", "custom"] } } }

[[define]]
kind = "ui.setting"
id = "sg.textures"
value = { label = "set-graphics-textures", description = "set-graphics-textures-desc", page = "graphics", group = "groups", scope = "Machine", kind = { Choice = { default = "high", options = ["extremely_low", "low", "medium", "high", "ultra", "custom"] } } }

[[define]]
kind = "ui.setting"
id = "sg.effects"
value = { label = "set-graphics-effects", description = "set-graphics-effects-desc", page = "graphics", group = "groups", scope = "Machine", kind = { Choice = { default = "high", options = ["extremely_low", "low", "medium", "high", "ultra", "custom"] } } }

[[define]]
kind = "ui.setting"
id = "sg.water"
value = { label = "set-graphics-water", description = "set-graphics-water-desc", page = "graphics", group = "groups", scope = "Machine", kind = { Choice = { default = "high", options = ["extremely_low", "low", "medium", "high", "ultra", "custom"] } } }

[[define]]
kind = "ui.setting"
id = "sg.sky"
value = { label = "set-graphics-sky", description = "set-graphics-sky-desc", page = "graphics", group = "groups", scope = "Machine", kind = { Choice = { default = "high", options = ["extremely_low", "low", "medium", "high", "ultra", "custom"] } } }
"#;

/// The page's settings as a registry (for `UiRegistry::extend_settings`).
pub fn registry() -> UiRegistry {
    let mut db = RecordDb::new();
    sf_ui::register_kinds(&mut db).expect("UI record kinds register");
    let base = db.add_source("bevyrrowind", &[]);
    let outcome = sf_records::apply_record_file(&mut db, base, RECORDS, sf_records::RecordFormat::Toml).expect("graphics page");
    assert!(outcome.errors.is_empty(), "graphics page: {:?}", outcome.errors);
    let (registry, conflicts) = UiRegistry::build(&db);
    assert!(conflicts.is_empty(), "graphics page: {conflicts:?}");
    registry
}

/// The machine settings file, `<user data>/machine.toml` (beside `profiles/`), once a profile
/// loaded (`None` in tests and tools, which then read and write none).
pub fn machine_file() -> Option<PathBuf> {
    let (profiles, _) = mw_boot_bevy::mods::bindings_profile()?;
    Some(profiles.parent()?.join("machine.toml"))
}

/// The overall driver's levels as group levels, for the menu's store: picking an overall level
/// writes its group levels, and the overall reads the level whose group levels all hold, else
/// `custom` (`sf_ui::apply_level` / `sync_driver`). `None` without an overall driver.
pub fn menu_preset(book: &PresetBook) -> Option<SettingPresetDef> {
    let overall = book.driver(tp::GRAPHICS_PRESET.name())?;
    let levels = overall
        .levels
        .iter()
        .map(|l| {
            let groups: BTreeMap<String, SettingValue> = l
                .values
                .iter()
                .filter(|m| m.driver)
                .map(|m| (m.name.clone(), SettingValue::Text(m.value.as_str().unwrap_or(tp::CUSTOM).to_owned())))
                .collect();
            (l.name.clone(), groups)
        })
        .collect();
    Some(SettingPresetDef { driver: overall.name.clone(), levels })
}

/// The store's levels for `book`'s drivers, in book order (overall first), `custom` left out.
pub fn saved_levels(store: &SettingsStore, book: &PresetBook) -> Vec<(String, String)> {
    book.drivers
        .iter()
        .filter_map(|d| match store.get(&d.name) {
            Some(SettingValue::Text(level)) if level != tp::CUSTOM => Some((d.name.clone(), level.clone())),
            _ => None,
        })
        .collect()
}

/// The levels saved in the machine file at `path` (a missing file: the defaults). A corrupt file
/// or a refused value is a warning and reads as the defaults.
pub fn load_saved(path: &Path, book: &PresetBook) -> Vec<(String, String)> {
    let mut store = SettingsStore::new(&registry());
    match store.load_machine(path) {
        Ok(issues) => {
            for issue in issues {
                warn!("graphics settings {}: {issue:?}", path.display());
            }
        }
        Err(e) => warn!("graphics settings {}: {e}", path.display()),
    }
    saved_levels(&store, book)
}

/// Start levels: `saved` as level requests at the `User` layer (overall first, then the groups:
/// a group differing from the overall makes it read custom again), then `quality` (an explicit `--quality`) at `CommandLine`, so it wins
/// over them; each applied at once (`PresetBook::take_requests`). The run's `--set`s of a driver
/// follow in `quality_presets::start_values`. Returns what applied, for the log.
pub fn apply_start_levels(tunables: &mut Tunables, book: &PresetBook, saved: &[(String, String)], quality: Option<&str>) -> Vec<String> {
    let mut out = Vec::new();
    let mut request = |tunables: &mut Tunables, driver: &str, level: &str, layer: SetBy, note: &str| {
        let Some(id) = tunables.id(driver) else { return };
        if let Err(e) = tunables.set_noted(id, Value::Str(level.to_owned()), layer, Some(note.to_owned())) {
            warn!("graphics preset {driver} {level}: {e}");
            return;
        }
        for (driver, level, result) in book.take_requests(tunables) {
            match result {
                Ok(_) => out.push(format!("{driver} {level} ({note})")),
                Err(e) => warn!("graphics preset {driver} {level}: {e}"),
            }
        }
    };
    for (driver, level) in saved {
        request(tunables, driver, level, SetBy::User, "options menu");
    }
    if let Some(level) = quality {
        request(tunables, tp::GRAPHICS_PRESET.name(), level, SetBy::CommandLine, "--quality");
    }
    out
}

/// How deep drivers may nest (as `sf_tunables::presets`).
const MAX_DEPTH: usize = 4; // tunables: const (a recursion guard against driver cycles, not a trade-off)

/// Whether some level of `driver` (through the drivers it sets) writes a `Restart` tunable.
pub fn touches_restart(book: &PresetBook, tunables: &Tunables, driver: &str) -> bool {
    fn walk(book: &PresetBook, tunables: &Tunables, driver: &str, depth: usize) -> bool {
        let Some(d) = book.driver(driver) else { return false };
        d.levels.iter().flat_map(|l| &l.values).any(|m| {
            if m.driver {
                depth < MAX_DEPTH && walk(book, tunables, &m.name, depth + 1)
            } else {
                restart(tunables, m.id)
            }
        })
    }
    fn restart(tunables: &Tunables, id: TunableId) -> bool {
        tunables.var(id).meta().lifetime == Lifetime::Restart
    }
    walk(book, tunables, driver, 0)
}

/// A pick on the page: the store keeps it (an overall level writes its group levels; a group
/// re-derives the overall), and with presets the driver is set at `User`, a level request the
/// presets apply next frame. Without a book (no presets in this app) the store alone changes.
///
/// # Errors
/// The store refuses the value (not one of the row's options).
pub fn pick(store: &mut SettingsStore, presets: Option<(&PresetBook, &mut Tunables)>, key: &str, level: &str) -> Result<(), String> {
    let menu = presets.as_ref().and_then(|(book, _)| menu_preset(book));
    let value = SettingValue::Text(level.to_owned());
    match &menu {
        Some(menu) if menu.driver == key => sf_ui::apply_level(menu, level, store).map_err(|e| e.to_string())?,
        Some(menu) => {
            store.set(key, value).map_err(|e| format!("{e:?}"))?;
            sf_ui::sync_driver(menu, store).map_err(|e| format!("{e:?}"))?;
        }
        None => {
            store.set(key, value).map_err(|e| format!("{e:?}"))?;
        }
    }
    if let Some((_, tunables)) = presets {
        let id = tunables.id(key).ok_or_else(|| format!("{key} is not a tunable"))?;
        tunables.set_noted(id, Value::Str(level.to_owned()), SetBy::User, Some("options menu".to_owned()))?;
        info!("options: {key} = {level} (graphics preset, applies live)");
    }
    Ok(())
}

/// Whether `key` is one of the page's rows.
pub fn is_row(key: &str) -> bool {
    tp::DRIVERS.iter().any(|d| d.name() == key)
}

/// The graphics rows of every open options screen follow their driver's effective level (a
/// derived `custom` included) and carry the restart badge when their levels touch a `Restart`
/// tunable. Writes a screen only when a row differs.
#[allow(clippy::needless_pass_by_value)] // system parameters
pub fn sync_rows(tunables: Option<Res<Tunables>>, presets: Option<Res<Presets>>, mut screens: Query<&mut OptionsScreen>) {
    let (Some(tunables), Some(presets)) = (tunables, presets) else { return };
    let book = &presets.0;
    let want = |key: &str| -> Option<(SettingValue, bool)> {
        book.driver(key)?;
        let id = tunables.id(key)?;
        Some((SettingValue::Text(tunables.text(id).to_owned()), touches_restart(book, &tunables, key)))
    };
    for mut screen in &mut screens {
        let stale: Vec<(String, SettingValue, bool)> = screen
            .model
            .pages
            .iter()
            .flat_map(|p| &p.rows)
            .filter(|r| is_row(&r.key))
            .filter_map(|r| want(&r.key).filter(|(v, restart)| (v, *restart) != (&r.value, r.restart)).map(|(v, x)| (r.key.clone(), v, x)))
            .collect();
        if stale.is_empty() {
            continue;
        }
        for row in screen.model.pages.iter_mut().flat_map(|p| &mut p.rows) {
            if let Some((_, value, restart)) = stale.iter().find(|(k, ..)| *k == row.key) {
                row.value = value.clone();
                row.restart = *restart;
            }
        }
    }
}

#[cfg(test)]
#[path = "graphics_page_tests.rs"]
mod tests;
