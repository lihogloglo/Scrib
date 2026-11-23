//! The options menu's Advanced page (menus.md §7 MF-6; spec `superfargoth/docs/specs/feature-switches.md`
//! "Player view", owner 2026-11-23): one row per player-safe feature switch (Cyberpunk / Nixxes
//! style), after Graphics; presets and groups stay the main path.
//!
//! - **Rows are generated** ([`registry`]) from the [`FeatureSwitches`] registry: every switch whose
//!   tunable is `Scope::Player` and that is not startup-only ([`row_switches`]), grouped by
//!   `Category` (panel order), page `graphics_advanced`, `scope = "Machine"`, `apply = Restart` for a
//!   `Restart` tunable (the screen's restart badge). Generated, not a fixed list, so a new player
//!   switch gets its row with no edit here: [`AdvancedPagePlugin`] builds them in `Plugin::cleanup`,
//!   after every plugin's `build` and `finish` registered its switches, adds them to the UI registry
//!   and gives the settings store their slots (`SettingsStore::add_settings`, which takes the
//!   machine file's values read before the rows existed).
//! - **A row is a choice** of the tunable's values (a `Bool` switch: `off` / `on`) plus
//!   `custom` ([`sf_ui::CUSTOM_LEVEL`], stepped over by the screen), the stored default: "no pick,
//!   the preset decides". A stored default is never saved, so a pick of `on` under a preset that
//!   turns the feature off is still a pick (a `Bool` setting defaulting to the tunable's default
//!   would drop it at the next start).
//! - **A pick applies live** ([`pick`]): the store keeps it and the tunable is set at `User`
//!   (`custom`, a reset: the `User` layer is cleared, the preset's value shows again). Saved to
//!   `<user data>/machine.toml` when the menus close, with the Graphics page's levels; at start
//!   ([`apply_saved`], interactive runs only, as the graphics levels) the saved picks are `User`
//!   values again. A `Restart` tunable read before `cleanup` takes a saved pick from the next start.
//! - **Rows show the effective value** ([`sync_rows`]): the console, a preset or a pick all show; a
//!   row's default is the value without the `User` layer (the preset's), so Reset page offers the
//!   rows the player moved off it and puts them back. Labels: the builtin mod's Fluent
//!   `set-feature-<name>` (dots as `-`) and `-desc` when present, else the switch's label and the
//!   tunable's help. A switch not built this run, or blocked by a required switch, is disabled
//!   with its reason.

use bevy::log::{info, warn};
use bevy::prelude::*;
use sf_records::{RecordDb, RecordId};
use sf_tunables_bevy::sf_tunables::{ArgKind, Availability, FeatureSwitch, Lifetime, OffValue, Scope};
use sf_tunables_bevy::{FeatureSwitches, SetBy, Tunables, Value};
use sf_ui::records::{SettingApply, SettingDef, SettingKind, SettingScope};
use sf_ui::{CUSTOM_LEVEL, Localizer, SettingValue, SettingsStore, UiRegistry};
use sf_ui_bevy::{UiLocalizer, UiRegistryRes, UiSettingsStore};
use sf_ui_screens::{OptionKind, OptionsScreen};

/// The page id (`set-page-graphics_advanced`).
pub const PAGE: &str = "graphics_advanced";

/// The tunable layer note of a pick.
const NOTE: &str = "options menu (advanced)";

/// The player's switches, in page order: `Scope::Player` tunables, live (not startup-only), by
/// category in panel order, then registration order.
pub fn row_switches(switches: &sf_tunables_bevy::sf_tunables::FeatureSwitches, tunables: &Tunables) -> Vec<&'static FeatureSwitch> {
    let mut rows: Vec<&'static FeatureSwitch> = switches.iter().filter(|s| is_player_switch(switches, tunables, s.name())).collect();
    rows.sort_by_key(|s| sf_tunables_bevy::sf_tunables::Category::ALL.iter().position(|c| *c == s.category_of()));
    rows
}

/// Whether `key` names a switch with a row on the page.
pub fn is_player_switch(switches: &sf_tunables_bevy::sf_tunables::FeatureSwitches, tunables: &Tunables, key: &str) -> bool {
    let (Some(switch), Some(id)) = (switches.get(key), switches.tunable(key)) else { return false };
    switch.startup_hint().is_none() && tunables.var(id).meta().scope == Scope::Player
}

/// A row's option ids: the tunable's values (`off` / `on` for a bool), then `custom`.
fn options(switch: &FeatureSwitch, tunables: &Tunables, id: sf_tunables_bevy::TunableId) -> Vec<String> {
    let mut out: Vec<String> = match (switch.off_value(), tunables.var(id).kind()) {
        (OffValue::Choice(_), ArgKind::Choice(choices)) => choices.clone(),
        _ => vec!["off".to_owned(), "on".to_owned()],
    };
    out.push(CUSTOM_LEVEL.to_owned());
    out
}

/// The option id of a tunable value (`on` / `off` for a bool).
fn option_of(value: &Value) -> String {
    match value {
        Value::Bool(true) => "on".to_owned(),
        Value::Bool(false) => "off".to_owned(),
        Value::Str(s) => s.clone(),
        other => other.to_string(),
    }
}

/// The Fluent key of a switch's label (`set-feature-sky-froxels-enabled`).
pub fn label_key(name: &str) -> String {
    format!("set-feature-{}", name.replace(['.', '_'], "-"))
}

/// The page's settings (one per [`row_switches`]) as a registry, owner `bevyrrowind` (beside the
/// framework's built-ins, which `extend_settings` skips); none
/// without player switches.
pub fn registry(switches: &sf_tunables_bevy::sf_tunables::FeatureSwitches, tunables: &Tunables) -> UiRegistry {
    let mut db = RecordDb::new();
    sf_ui::register_kinds(&mut db).expect("UI record kinds register");
    let base = db.add_source("bevyrrowind", &[]);
    for switch in row_switches(switches, tunables) {
        let Some(id) = switches.tunable(switch.name()) else { continue };
        let label = label_key(switch.name());
        let kind = SettingKind::Choice { default: CUSTOM_LEVEL.to_owned(), options: options(switch, tunables, id), options_from: None };
        let def = SettingDef {
            description: Some(format!("{label}-desc")),
            page: Some(PAGE.to_owned()),
            group: Some(switch.category_of().name().to_owned()),
            scope: SettingScope::Machine,
            apply: if tunables.var(id).meta().lifetime == Lifetime::Restart { SettingApply::Restart } else { SettingApply::default() },
            ..SettingDef::new(label, kind)
        };
        db.define::<SettingDef>(base, RecordId::from(switch.name()), def).expect("ui.setting is registered");
    }
    let (registry, conflicts) = UiRegistry::build(&db);
    assert!(conflicts.is_empty(), "advanced page: {conflicts:?}");
    registry
}

/// Sets switch `key`'s tunable for option `option` at `User` (`custom`: clears the `User` layer).
fn apply(tunables: &mut Tunables, switches: &sf_tunables_bevy::sf_tunables::FeatureSwitches, key: &str, option: &str) -> Result<(), String> {
    let switch = switches.get(key).ok_or_else(|| format!("{key} is not a feature switch"))?;
    let id = switches.tunable(key).ok_or_else(|| format!("{key} is not a feature switch"))?;
    if option == CUSTOM_LEVEL {
        tunables.clear(id, SetBy::User);
        return Ok(());
    }
    let value = match switch.off_value() {
        OffValue::Bool(_) => Value::Bool(option == "on"),
        OffValue::Choice(_) => Value::Str(option.to_owned()),
    };
    tunables.set_noted(id, value, SetBy::User, Some(NOTE.to_owned())).map(|_| ())
}

/// A pick on the page: the store keeps it and the switch's tunable is set at `User` (`custom`:
/// the `User` layer cleared, the preset decides again), live.
///
/// # Errors
/// The store refuses the value (not one of the row's options) or the tunable does.
pub fn pick(
    store: &mut SettingsStore,
    live: Option<(&sf_tunables_bevy::sf_tunables::FeatureSwitches, &mut Tunables)>,
    key: &str,
    option: &str,
) -> Result<(), String> {
    store.set(key, SettingValue::Text(option.to_owned())).map_err(|e| format!("{e:?}"))?;
    if let Some((switches, tunables)) = live {
        apply(tunables, switches, key, option)?;
        info!("options: {key} = {option} (feature switch, applies live)");
    }
    Ok(())
}

/// The saved picks of the page's rows (`custom` left out) applied at `User`; what applied, for the log.
pub fn apply_saved(store: &SettingsStore, switches: &sf_tunables_bevy::sf_tunables::FeatureSwitches, tunables: &mut Tunables) -> Vec<String> {
    let mut out = Vec::new();
    for switch in row_switches(switches, tunables) {
        let Some(SettingValue::Text(option)) = store.get(switch.name()) else { continue };
        if option == CUSTOM_LEVEL {
            continue;
        }
        match apply(tunables, switches, switch.name(), option) {
            Ok(()) => out.push(format!("{} {option}", switch.name())),
            Err(e) => warn!("advanced options: {}: {e}", switch.name()),
        }
    }
    out
}

/// Generates the page once every plugin registered its switches; `apply_saved`: the saved picks
/// apply at start (interactive runs; benchmarks and screenshots don't read them).
pub struct AdvancedPagePlugin {
    pub apply_saved: bool,
}

impl Plugin for AdvancedPagePlugin {
    fn build(&self, _app: &mut App) {}

    fn cleanup(&self, app: &mut App) {
        let world = app.world_mut();
        let (Some(switches), Some(tunables)) = (world.get_resource::<FeatureSwitches>(), world.get_resource::<Tunables>()) else { return };
        let page = registry(switches, tunables);
        if !page.settings().any(|d| d.source.is_some()) {
            return;
        }
        if let Some(mut ui) = world.get_resource_mut::<UiRegistryRes>() {
            ui.0.extend_settings(&page);
        }
        let Some(mut store) = world.get_resource_mut::<UiSettingsStore>() else { return };
        for issue in store.bypass_change_detection().0.add_settings(&page) {
            warn!("advanced options: {issue:?}");
        }
        if self.apply_saved {
            world.resource_scope(|world, store: Mut<UiSettingsStore>| {
                world.resource_scope(|world, mut tunables: Mut<Tunables>| {
                    let switches = world.resource::<FeatureSwitches>();
                    for line in apply_saved(&store.0, switches, &mut tunables) {
                        info!("advanced options at start: {line}");
                    }
                });
            });
        }
    }
}

/// What a row should read now: value, default (the value without the `User` layer), enabled,
/// disabled reason, label, description, the `custom` option's label.
struct Want {
    value: SettingValue,
    default: SettingValue,
    enabled: bool,
    reason: Option<String>,
    label: String,
    description: Option<String>,
    custom: String,
}

fn want(key: &str, switches: &FeatureSwitches, tunables: &Tunables, localizer: Option<&Localizer>) -> Option<Want> {
    if !is_player_switch(switches, tunables, key) {
        return None;
    }
    let switch = switches.get(key)?;
    let id = switches.tunable(key)?;
    let var = tunables.var(id);
    let below_user = var.layers().filter(|(l, _)| *l != SetBy::User).last().map_or_else(|| var.default().clone(), |(_, v)| v.clone());
    let text = |key: &str| localizer.filter(|l| l.has(key)).map(|l| l.format(key, None).into_owned());
    let state = switches.state(tunables, key)?;
    let reason = match state.availability {
        Availability::Available => None,
        Availability::NotBuilt { reason } => Some(reason.to_owned()),
        Availability::Blocked { by } => {
            let needs = text("set-feature-needs").unwrap_or_else(|| "Needs".to_owned());
            let by_label = switches.get(by).map_or(by, FeatureSwitch::label_text);
            Some(format!("{needs}: {by_label}"))
        }
    };
    let label = label_key(key);
    Some(Want {
        value: SettingValue::Text(option_of(var.value())),
        default: SettingValue::Text(option_of(&below_user)),
        enabled: reason.is_none(),
        reason,
        label: text(&label).unwrap_or_else(|| switch.label_text().to_owned()),
        description: text(&format!("{label}-desc")).or_else(|| Some(var.help().to_owned()).filter(|h| !h.is_empty())),
        custom: text("set-feature-preset").unwrap_or_else(|| "Preset".to_owned()),
    })
}

/// The Advanced rows of every open options screen follow their switch: the effective value, the
/// preset's value as the default, availability (disabled with its reason), labels. Writes a
/// screen only when a row differs.
#[allow(clippy::needless_pass_by_value)] // system parameters
pub fn sync_rows(
    tunables: Option<Res<Tunables>>,
    switches: Option<Res<FeatureSwitches>>,
    localizer: Option<Res<UiLocalizer>>,
    mut screens: Query<&mut OptionsScreen>,
) {
    let (Some(tunables), Some(switches)) = (tunables, switches) else { return };
    let localizer = localizer.as_deref().map(|l| &l.0);
    for mut screen in &mut screens {
        let Some(p) = screen.model.pages.iter().position(|p| p.id == PAGE) else { continue };
        let wants: Vec<Option<Want>> = screen.model.pages[p].rows.iter().map(|r| want(&r.key, &switches, &tunables, localizer)).collect();
        let stale = screen.model.pages[p].rows.iter().zip(&wants).any(|(r, w)| {
            w.as_ref().is_some_and(|w| {
                let custom_label = match &r.kind {
                    OptionKind::Choice(options) => options.iter().find(|(o, _)| o == CUSTOM_LEVEL).map(|(_, l)| l.as_str()),
                    _ => None,
                };
                r.value != w.value
                    || r.default != w.default
                    || r.enabled != w.enabled
                    || r.disabled_reason != w.reason
                    || r.label != w.label
                    || r.description != w.description
                    || custom_label.is_some_and(|l| l != w.custom)
            })
        });
        if !stale {
            continue;
        }
        for (row, w) in screen.model.pages[p].rows.iter_mut().zip(wants) {
            let Some(w) = w else { continue };
            row.value = w.value;
            row.default = w.default;
            row.enabled = w.enabled;
            row.disabled_reason = w.reason;
            row.label = w.label;
            row.description = w.description;
            if let OptionKind::Choice(options) = &mut row.kind {
                for (o, l) in options.iter_mut() {
                    if o == CUSTOM_LEVEL {
                        l.clone_from(&w.custom);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
#[path = "advanced_page_tests.rs"]
mod tests;
