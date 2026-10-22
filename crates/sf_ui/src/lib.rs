//! Bevy-free core of the game UI (ui.md).
//!
//! Everything here is plain data and logic, testable without an engine:
//!
//! - [`records`]: the UI record kinds mods define through `sf_records`
//!   (`ui.screen`, `ui.slot`, `ui.column`, `ui.theme`, `ui.preset`, `ui.setting`).
//! - [`UiRegistry`]: those records resolved once per mod set, with owners,
//!   ordered slot contributions and columns, and a [`UiConflict`] report.
//! - [`ScreenStack`]: the Unreal Common UI layer model (screens stacked per [`Layer`]);
//!   it answers which screen receives input and whether the game pauses.
//!   `sf_ui_bevy` wraps it in a resource and feeds the input crate's contexts.
//! - [`ViewModel`] and [`ListModel`]: MVVM data that game systems write and
//!   screens read; list rows have stable keys, so sorting and filtering
//!   never rebuild rows.
//! - [`SettingsStore`]: typed setting values over the registry's
//!   `ui.setting` records, read from and written to profile tables.
//! - [`Localizer`]: Fluent strings from every mod in load order, with a
//!   locale fallback chain, built from each mod's `l10n/<locale>/<mod id>.ftl`
//!   ([`l10n_sources`]); settings load from and save to the `sf_mods` profile.
//! - [`Keymap`] and [`Rebinder`]: the keybinds screen (U6). Rebindable actions
//!   with per-device slots, conflicts per scope, swap/unbind resolution,
//!   resets, the profile's `[bindings.<context>]` tables; [`glyph`] labels
//!   input names per device and pad family.

mod ids;
mod input;
mod keymap;
mod l10n;
mod list;
mod mods;
mod rebind;
pub mod records;
mod registry;
mod settings;
mod stack;
mod view_model;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_keymap;
#[cfg(test)]
mod tests_mods;

pub use ids::{
    ColumnId, InputContextId, ListId, ModId, PresetId, ScreenId, SettingKey, SlotId, TemplateId,
    ThemeId,
};
pub use input::{Device, Glyph, Input, InputKind, Mods, Overlap, PadFamily, glyph, overlap};
pub use keymap::{
    ActionDef, ActionRef, ActionShape, Conflict, InputValidator, KeybindRow, Keymap, KeymapError,
    KeymapIssue, Proposal, Resolution, SlotRef,
};
pub use l10n::{
    FluentArgs, FluentValue, L10nIssue, L10nSource, LanguageIdentifier, Localizer, negotiate,
};
pub use list::{ListFilter, ListModel, SortSpec};
pub use mods::{L10N_DIR, UiRecordKinds, available_locales, l10n_sources};
pub use rebind::{CAPTURE_TIMEOUT, CaptureFrame, RebindEvent, Rebinder};
pub use records::{Layer, register_kinds};
pub use registry::{Def, Severity, UiConflict, UiRegistry};
pub use settings::{SettingError, SettingValue, SettingsStore};
pub use stack::{OpenScreen, ScreenStack, StackError};
pub use view_model::{UiValue, ViewModel};
