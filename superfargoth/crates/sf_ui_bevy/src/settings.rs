//! The settings change contract and the machine settings file (menus.md "Settings v2").
//!
//! - [`SettingChanged`]: one message per effective change of a [`UiSettingsStore`] value,
//!   whoever wrote it (options screen, presets, scripts, a load). [`emit_setting_changes`]
//!   drains the store's change log (`sf_ui::SettingsStore::take_changes`) once per frame in
//!   `PostUpdate`; writes made in `Update` or earlier arrive the same frame. Consumers are
//!   ordinary systems, one per setting family, owned by the crate that applies them; order
//!   after [`emit_setting_changes`] to apply the same frame, or read in the next frame's `Update`.
//! - **Machine scope**: `Machine` settings (display, graphics) live in a per-machine file, not
//!   the profile. [`load_machine_settings`] reads it into the store at app build and inserts
//!   [`UiMachineSettings`]; [`sync_machine_settings`] writes it back on the profile's rule
//!   (when the player leaves the menus, or on exit).
//!
//! **Where the app calls it**: right after it inserts [`UiSettingsStore`] at app build (after
//! [`crate::insert_from_profile`], or the game's own store), before plugins that read settings
//! at build (`Restart` settings) are added. The file is `<user dir>/machine.toml`, the user dir
//! being the app's own (beside its profiles folder). Bevy 0.20's `bevy-settings` crate keeps
//! the same TOML layout and the same per-user folder but stores compile-time typed
//! `SettingsGroup` resources (one reflected Rust type per group); our settings are records,
//! added by mods at load, so the store keeps its own file (menus.md "Reuse").

use std::path::PathBuf;

use bevy_app::AppExit;
use bevy_ecs::prelude::*;
use sf_ui::{Layer, MachineFileError, SettingError, SettingKey, SettingValue};

use crate::{ScreenStackRes, UiSettingsStore};

/// A setting's value changed (menus.md "Change contract").
#[derive(Message, Debug, Clone, PartialEq)]
pub struct SettingChanged {
    pub key: SettingKey,
    /// The value before the change (`Some` for every registered setting today).
    pub old: Option<SettingValue>,
    pub new: SettingValue,
}

/// Turns the store's change log into [`SettingChanged`] messages (`PostUpdate`).
/// Draining doesn't mark the store changed, so profile saves only follow real writes.
pub fn emit_setting_changes(
    store: Option<ResMut<UiSettingsStore>>,
    mut out: MessageWriter<SettingChanged>,
) {
    let Some(mut store) = store.filter(|s| s.0.has_changes()) else {
        return;
    };
    let changes = store.bypass_change_detection().0.take_changes();
    out.write_batch(changes.into_iter().map(|c| SettingChanged {
        key: c.key,
        old: c.old,
        new: c.new,
    }));
}

/// Where `Machine` settings are saved, and the save state.
#[derive(Resource, Debug)]
pub struct UiMachineSettings {
    /// The machine file (`<user dir>/machine.toml`).
    pub path: PathBuf,
    /// The stored tables as last read or written (TOML text), to tell whether a save is needed.
    pub written: String,
    /// Changes not yet written.
    pub dirty: bool,
    /// Why the last save failed, for the options screen or a log.
    pub last_error: Option<String>,
}

/// Reads the machine file at `path` into the world's [`UiSettingsStore`] and inserts
/// [`UiMachineSettings`] so changes are saved there. A missing file is a first run; a
/// corrupt one leaves every `Machine` setting at its default and returns the error (the
/// next save replaces it). With no store in the world, only the resource is inserted.
pub fn load_machine_settings(
    world: &mut World,
    path: PathBuf,
) -> Result<Vec<SettingError>, MachineFileError> {
    let (result, written) = match world.get_resource_mut::<UiSettingsStore>() {
        Some(mut store) => {
            let store = &mut store.bypass_change_detection().0;
            let result = store.load_machine(&path);
            (result, store.to_machine().to_string())
        }
        None => (Ok(Vec::new()), String::new()),
    };
    world.insert_resource(UiMachineSettings {
        path,
        // A corrupt file reads as "nothing stored": the first change rewrites it.
        written,
        dirty: false,
        last_error: result.as_ref().err().map(ToString::to_string),
    });
    result
}

/// Writes changed `Machine` settings to [`UiMachineSettings::path`] once no Menu or
/// Modal screen takes input (or on exit), like the profile (`Last`).
pub fn sync_machine_settings(
    machine: Option<ResMut<UiMachineSettings>>,
    store: Option<Res<UiSettingsStore>>,
    stack: Res<ScreenStackRes>,
    mut exit: MessageReader<AppExit>,
) {
    let Some(mut machine) = machine else { return };
    let machine = machine.bypass_change_detection();
    let Some(store) = store else { return };
    if store.is_changed()
        && !store.is_added()
        && store.0.to_machine().to_string() != machine.written
    {
        machine.dirty = true;
    }
    let exiting = exit.read().count() > 0;
    let in_menus = matches!(
        stack.0.input_target().map(|s| s.layer),
        Some(Layer::Menu | Layer::Modal)
    );
    if !machine.dirty || (in_menus && !exiting) {
        return;
    }
    // On failure the flag clears too: the next change retries, not every frame.
    machine.dirty = false;
    match store.0.save_machine(&machine.path) {
        Ok(_) => {
            machine.written = store.0.to_machine().to_string();
            machine.last_error = None;
        }
        Err(e) => machine.last_error = Some(e.to_string()),
    }
}
