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
//!   never rebuild rows; sorts chain keys (weight, then name) and filters combine
//!   word search, exact matches and value sets (tabs).
//! - [`VirtualList`]: scroll, keyed focus and a recycled row pool over a
//!   `ListModel`, so thousands of rows cost a few dozen widgets.
//! - [`SettingsStore`]: typed setting values over the registry's
//!   `ui.setting` records, read from and written to profile tables.
//! - [`Localizer`]: Fluent strings from every mod in load order, with a
//!   locale fallback chain, built from each mod's `l10n/<locale>/<mod id>.ftl`
//!   ([`l10n_sources`]); settings load from and save to the `sf_mods` profile.
//! - [`RichDoc`] and [`Pagination`]: books, scrolls and notes (ui-text.md): a
//!   rich-text document model that content packs' markup parsers build, cut
//!   into pages (a two-page spread, or one long scroll page) over a
//!   [`TextMeasure`] that the text engine implements.

mod ids;
mod l10n;
mod list;
mod mods;
mod paginate;
pub mod records;
mod registry;
mod rich_text;
mod settings;
mod stack;
mod view_model;
mod virtual_list;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_list;
#[cfg(test)]
mod tests_mods;
#[cfg(test)]
mod tests_text;

pub use ids::{
    ColumnId, InputContextId, ListId, ModId, PresetId, ScreenId, SettingKey, SlotId, TemplateId,
    ThemeId,
};
pub use l10n::{
    FluentArgs, FluentValue, L10nIssue, L10nSource, LanguageIdentifier, Localizer, negotiate,
};
pub use list::{ListFilter, ListModel, SortSpec};
pub use mods::{L10N_DIR, UiRecordKinds, available_locales, l10n_sources};
pub use paginate::{
    DocPos, Fragment, LineMetrics, Page, PageFormat, PageView, Pagination, Placed, Size,
    TextMeasure,
};
pub use records::{Layer, register_kinds};
pub use registry::{Def, Severity, UiConflict, UiRegistry};
pub use rich_text::{
    Align, Block, ImageRef, Inline, OBJECT_REPLACEMENT, Paragraph, Rgba8, RichDoc, RunStyle,
};
pub use settings::{SettingError, SettingValue, SettingsStore};
pub use stack::{OpenScreen, ScreenStack, StackError};
pub use view_model::{UiValue, ViewModel};
pub use virtual_list::{ListFrame, RowBinding, VirtualList};
