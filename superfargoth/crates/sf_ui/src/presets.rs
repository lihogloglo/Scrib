//! Setting presets (menus.md "Settings v2", Presets): a `ui.setting_preset`
//! record names levels of setting values behind one Choice setting, the driver.
//!
//! Picking a level writes its values; the driver then shows the level the
//! current values match, or [`CUSTOM_LEVEL`] when they match none. The
//! driver's value is recomputed after every change ([`sync_driver`]), never
//! kept as a separate truth, so a member changed by hand, by a profile load
//! or by another preset is reflected the same way (Unreal's overall
//! scalability level reads "Custom" the same way once one group differs).

use std::fmt;

use crate::records::SettingPresetDef;
use crate::settings::{SettingError, SettingValue, SettingsStore};

/// The driver option shown when the current values match no level.
pub const CUSTOM_LEVEL: &str = "custom";

/// Why [`apply_level`] failed.
#[derive(Clone, Debug, PartialEq)]
pub enum PresetError {
    /// The preset has no such level (and it isn't [`CUSTOM_LEVEL`]).
    UnknownLevel(String),
    /// The store refused a value (a store built from another registry).
    Setting(SettingError),
}

impl fmt::Display for PresetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PresetError::UnknownLevel(level) => write!(f, "unknown preset level `{level}`"),
            PresetError::Setting(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for PresetError {}

impl From<SettingError> for PresetError {
    fn from(e: SettingError) -> Self {
        PresetError::Setting(e)
    }
}

/// Writes a level's values, then re-derives the driver. Choosing
/// [`CUSTOM_LEVEL`] writes nothing (the driver is still re-derived, so it
/// reads a level again if the values match one).
pub fn apply_level(
    preset: &SettingPresetDef,
    level: &str,
    store: &mut SettingsStore,
) -> Result<(), PresetError> {
    if level != CUSTOM_LEVEL {
        let values = preset
            .levels
            .get(level)
            .ok_or_else(|| PresetError::UnknownLevel(level.to_owned()))?;
        for (key, value) in values {
            store.set(key, value.clone())?;
        }
    }
    sync_driver(preset, store)?;
    Ok(())
}

/// The first level, in level-id order, whose every value equals the store's
/// current value; `None` means Custom. A level with no values never matches.
pub fn derived_level(preset: &SettingPresetDef, store: &SettingsStore) -> Option<String> {
    preset
        .levels
        .iter()
        .find(|(_, values)| {
            !values.is_empty() && values.iter().all(|(k, v)| store.get(k) == Some(v))
        })
        .map(|(level, _)| level.clone())
}

/// Sets the driver to the derived level, or [`CUSTOM_LEVEL`]. Returns the
/// derived level. Call it after any change to a member setting.
pub fn sync_driver(
    preset: &SettingPresetDef,
    store: &mut SettingsStore,
) -> Result<Option<String>, SettingError> {
    let level = derived_level(preset, store);
    let shown = level.clone().unwrap_or_else(|| CUSTOM_LEVEL.to_owned());
    store.set(&preset.driver, SettingValue::Text(shown))?;
    Ok(level)
}
