//! CSS styling for `sf_ui` screens through `bevy_flair` (ui.md "Data model and moddability", R2b).
//!
//! The composer serves one generated root sheet per (theme, preset, mod set): framework, preset,
//! theme, one layer per mod in load order, then the player's sheet, each `@import`ed into its own
//! cascade layer so later layers win as later mods do. Every screen root (`UiScreenRoot`) gets
//! that sheet, the root classes (`preset-*`, `theme-*`, `high-contrast`, `reduce-motion`) and the
//! `input=kbm|gamepad` attribute. A sheet's CSS errors are reported against the mod that ships it
//! ([`UiCssReport`]). Edited sheets hot-reload through the content source's watcher.
//!
//! This crate is the only one that links `bevy_flair` and Servo's MPL-2.0 `cssparser`/`selectors`
//! (unmodified, owner decision 2026-10-11); flair types stay inside it. The mod contract is the CSS
//! subset documented in ui.md, not flair's API.
//!
//! ```ignore
//! app.add_plugins(UiCssSourcePlugin::new(content_source_builder)) // before DefaultPlugins
//!     .add_plugins((DefaultPlugins, EnhancedInputPlugin, SfUiPlugin, UiCssPlugin));
//! app.insert_resource(UiCssSheets { mods: mods_from_profile(&loaded), ..default() });
//! ```

mod compose;
mod plugin;

pub use compose::{ComposedSheet, Composition, LayerKind, SheetSource, asset_url, mod_layer_name};
pub use plugin::{
    BUTTON_CLASS, CONTENT_SOURCE, CssIssue, HIGH_CONTRAST_VARIANT, ROOT_SHEET, ROOT_SOURCE, UiClass, UiCssPlugin,
    UiCssReport, UiCssRootDir, UiCssSheets, UiCssSourcePlugin, UiCssState, UiCssSystems,
    mods_from_profile,
};

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_compose;
