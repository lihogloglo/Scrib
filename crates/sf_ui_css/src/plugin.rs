//! Bevy side: asset sources, the composed root sheet on every screen root, root classes and the
//! `input` attribute, per-sheet error attribution, recomposition and hot reload.

use std::path::Path;
use std::sync::{Mutex, PoisonError};

use bevy_app::{App, Plugin, Update};
use bevy_asset::io::memory::{Dir, MemoryAssetReader};
use bevy_asset::io::{AssetSourceBuilder, AssetSourceId};
use bevy_asset::{AssetApp, AssetEvent, AssetLoadFailedEvent, AssetServer, Handle};
use bevy_ecs::prelude::*;
use bevy_flair::FlairPlugin;
use bevy_flair::parser::{CssStyleLoaderErrorMode, CssStyleLoaderSetting};
use bevy_flair::style::components::{AttributeList, ClassList, Styled};
use bevy_flair::style::{FlairStylePlugin, StyleSheet};
use bevy_log::warn;
use sf_ui_bevy::{Device, InputMode, UiRegistryRes, UiScreenRoot, UiSettings};

use crate::compose::{ComposedSheet, Composition, LayerKind, asset_url};

/// Asset source of the generated root sheet (in memory).
pub const ROOT_SOURCE: &str = "sf_ui";
/// Path of the generated root sheet in [`ROOT_SOURCE`].
pub const ROOT_SHEET: &str = "root.css";
/// Named asset source the root sheet imports content from: the app registers its content
/// (normally the VFS) under this name too, because a path can't name Bevy's default source.
pub const CONTENT_SOURCE: &str = "sf_ui_content";
/// Theme variant imported after the theme sheet while [`UiSettings::high_contrast`] is on.
pub const HIGH_CONTRAST_VARIANT: &str = "high_contrast";

/// Registers the two asset sources. Add it **before** `AssetPlugin` (`DefaultPlugins`), with
/// the same kind of source the app uses for content (`sf_vfs::bevy::shared_asset_source` in a
/// game, `AssetSourceBuilder::platform_default("assets", None)` in an example).
pub struct UiCssSourcePlugin {
    content: Mutex<Option<AssetSourceBuilder>>,
}

impl UiCssSourcePlugin {
    pub fn new(content: AssetSourceBuilder) -> Self {
        Self {
            content: Mutex::new(Some(content)),
        }
    }
}

/// The in-memory directory holding the generated root sheet.
#[derive(Resource, Clone)]
pub struct UiCssRootDir(pub Dir);

impl Plugin for UiCssSourcePlugin {
    fn build(&self, app: &mut App) {
        assert!(
            !app.world().contains_resource::<AssetServer>(),
            "UiCssSourcePlugin must be added before AssetPlugin (DefaultPlugins)"
        );
        let Some(content) = self
            .content
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .take()
        else {
            return;
        };
        let dir = Dir::new(Path::new("").to_path_buf());
        let reader_dir = dir.clone();
        app.register_asset_source(CONTENT_SOURCE, content)
            .register_asset_source(
                AssetSourceId::from(ROOT_SOURCE),
                AssetSourceBuilder::new(move || {
                    Box::new(MemoryAssetReader {
                        root: reader_dir.clone(),
                    })
                }),
            )
            .insert_resource(UiCssRootDir(dir));
    }
}

/// Styling for `sf_ui_bevy` screens (after `DefaultPlugins` and `SfUiPlugin`; needs
/// [`UiCssSourcePlugin`] first). Adds `bevy_flair`'s `FlairPlugin` unless present.
pub struct UiCssPlugin;

impl Plugin for UiCssPlugin {
    fn build(&self, app: &mut App) {
        assert!(
            app.world().contains_resource::<UiCssRootDir>(),
            "UiCssPlugin needs UiCssSourcePlugin, added before DefaultPlugins"
        );
        if !app.is_plugin_added::<FlairStylePlugin>() {
            app.add_plugins(FlairPlugin);
        }
        app.init_resource::<UiCssSheets>()
            .init_resource::<UiCssReport>()
            .init_resource::<UiCssState>()
            .init_resource::<InputMode>()
            .init_resource::<UiSettings>()
            .init_resource::<UiRegistryRes>()
            .add_systems(
                Update,
                (compose_root, (attribute_errors, style_roots, apply_classes))
                    .chain()
                    .in_set(UiCssSystems),
            );
    }
}

/// The styling systems (`Update`).
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UiCssSystems;

/// Sheets beside the active theme and preset (those come from [`UiSettings`] and the registry).
/// Paths are VFS paths in [`CONTENT_SOURCE`].
#[derive(Resource, Clone, Debug, Default, PartialEq, Eq)]
pub struct UiCssSheets {
    /// Framework base sheets (layer `sf`).
    pub framework: Vec<String>,
    /// `(mod id, path)` in load order: `sf_mods::LoadedProfile::stylesheets()` (see [`mods_from_profile`]).
    pub mods: Vec<(String, String)>,
    /// The player's overrides (layer `user`).
    pub user: Vec<String>,
}

/// `LoadedProfile::stylesheets()` as [`UiCssSheets::mods`].
pub fn mods_from_profile(profile: &sf_mods::LoadedProfile) -> Vec<(String, String)> {
    profile
        .stylesheets()
        .into_iter()
        .map(|(id, path)| (id.as_str().to_owned(), path.as_str().to_owned()))
        .collect()
}

/// A CSS problem in one sheet, attributed to whoever ships it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CssIssue {
    pub kind: LayerKind,
    /// The owning mod (`None` for framework and user sheets).
    pub mod_id: Option<String>,
    pub path: String,
    /// flair's report (rule positions and messages), or the load error.
    pub message: String,
}

/// Current CSS issues, one entry per failing sheet; a sheet's entry goes when it loads clean.
/// For the mod manager and `sf_diag`.
#[derive(Resource, Clone, Debug, Default)]
pub struct UiCssReport {
    pub issues: Vec<CssIssue>,
    /// Bumped on every change.
    pub version: u64,
}

impl UiCssReport {
    pub fn for_mod<'a>(&'a self, mod_id: &'a str) -> impl Iterator<Item = &'a CssIssue> + 'a {
        self.issues
            .iter()
            .filter(move |i| i.mod_id.as_deref() == Some(mod_id))
    }
}

/// Classes for a styled node (`.class` selectors), space separated. `sf_ui_css` owns flair's
/// class list; screen roots also get the managed root classes.
#[derive(Component, Clone, Debug, Default, PartialEq, Eq)]
pub struct UiClass(pub String);

impl UiClass {
    pub fn new(classes: impl Into<String>) -> Self {
        Self(classes.into())
    }
}

/// Composer state: the current composition, the root handle and one probe handle per sheet.
#[derive(Resource, Default)]
pub struct UiCssState {
    composed: Option<ComposedSheet>,
    root: Option<Handle<StyleSheet>>,
    /// Each source sheet loaded on its own with `ReturnError`, so a broken sheet fails with its
    /// own path (the root imports it leniently and keeps its valid rules).
    probes: Vec<(String, Handle<StyleSheet>)>,
    root_classes: Vec<String>,
    /// Root classes changed since `apply_classes` last ran.
    roots_dirty: bool,
    input: &'static str,
}

impl UiCssState {
    /// The root sheet handle (after the first update).
    pub fn root(&self) -> Option<&Handle<StyleSheet>> {
        self.root.as_ref()
    }

    pub fn composed(&self) -> Option<&ComposedSheet> {
        self.composed.as_ref()
    }

    /// Classes every screen root carries: preset, theme, accessibility flags.
    pub fn root_classes(&self) -> &[String] {
        &self.root_classes
    }
}

fn composition(
    sheets: &UiCssSheets,
    settings: &UiSettings,
    registry: &UiRegistryRes,
) -> (Composition, Vec<String>) {
    let mut c = Composition::new();
    let mut classes = Vec::new();
    for path in &sheets.framework {
        c.framework(path.clone());
    }
    if let Some(preset) = registry.0.preset(settings.preset.as_str()) {
        if let Some(path) = &preset.def.stylesheet {
            c.preset(Some(preset.owner.as_str()), path.clone());
        }
        classes.push(preset.def.root_class.clone());
    } else {
        classes.push(format!("preset-{}", settings.preset.as_str()));
    }
    if let Some(theme) = registry.0.theme(settings.theme.as_str()) {
        c.theme(Some(theme.owner.as_str()), theme.def.stylesheet.clone());
        if settings.high_contrast
            && let Some(path) = theme.def.variants.get(HIGH_CONTRAST_VARIANT)
        {
            c.theme(Some(theme.owner.as_str()), path.clone());
        }
    }
    classes.push(format!("theme-{}", settings.theme.as_str()));
    if settings.high_contrast {
        classes.push("high-contrast".into());
    }
    if settings.reduce_motion {
        classes.push("reduce-motion".into());
    }
    for (id, path) in &sheets.mods {
        c.mod_sheet(id.clone(), path.clone());
    }
    for path in &sheets.user {
        c.user(path.clone());
    }
    (c, classes)
}

fn load_strict(server: &AssetServer, path: &str) -> Handle<StyleSheet> {
    server
        .load_builder()
        .with_settings(|s: &mut CssStyleLoaderSetting| {
            s.error_mode = CssStyleLoaderErrorMode::ReturnError;
        })
        .load(asset_url(CONTENT_SOURCE, path))
}

/// Rebuilds the root sheet when its inputs change; the asset server reloads it and flair
/// restyles every subtree that uses it (live theme, preset and mod-set swaps).
fn compose_root(
    sheets: Res<UiCssSheets>,
    settings: Res<UiSettings>,
    registry: Res<UiRegistryRes>,
    dir: Res<UiCssRootDir>,
    server: Res<AssetServer>,
    mut state: ResMut<UiCssState>,
    mut report: ResMut<UiCssReport>,
) {
    let first = state.root.is_none();
    if !first && !sheets.is_changed() && !settings.is_changed() && !registry.is_changed() {
        return;
    }
    let (composition, classes) = composition(&sheets, &settings, &registry);
    let composed = composition.compose(CONTENT_SOURCE);
    if state.root_classes != classes {
        state.root_classes = classes;
        state.roots_dirty = true;
    }
    if state
        .composed
        .as_ref()
        .is_some_and(|c| c.css == composed.css)
    {
        return;
    }
    dir.0
        .insert_asset_text(Path::new(ROOT_SHEET), &composed.css);
    let root_path = asset_url(ROOT_SOURCE, ROOT_SHEET);
    if first {
        state.root = Some(server.load(root_path));
    } else {
        server.reload(root_path);
    }
    // Probes for the new sheet set; issues of sheets that left the composition go.
    let keep: Vec<(String, Handle<StyleSheet>)> = std::mem::take(&mut state.probes);
    state.probes = composed
        .sheets
        .iter()
        .map(|s| {
            let handle = keep
                .iter()
                .find(|(p, _)| *p == s.path)
                .map_or_else(|| load_strict(&server, &s.path), |(_, h)| h.clone());
            (s.path.clone(), handle)
        })
        .collect();
    let before = report.issues.len();
    report.issues.retain(|i| composed.sheet(&i.path).is_some());
    if report.issues.len() != before {
        report.version += 1;
    }
    state.composed = Some(composed);
}

/// Load failures of probe sheets become [`CssIssue`]s for their owners; a clean (re)load clears them.
fn attribute_errors(
    mut failed: MessageReader<AssetLoadFailedEvent<StyleSheet>>,
    mut events: MessageReader<AssetEvent<StyleSheet>>,
    state: Res<UiCssState>,
    mut report: ResMut<UiCssReport>,
) {
    let Some(composed) = &state.composed else {
        failed.clear();
        events.clear();
        return;
    };
    for event in events.read() {
        let (AssetEvent::Added { id }
        | AssetEvent::Modified { id }
        | AssetEvent::LoadedWithDependencies { id }) = event
        else {
            continue;
        };
        if let Some((path, _)) = state.probes.iter().find(|(_, h)| h.id() == *id) {
            let before = report.issues.len();
            report.issues.retain(|i| i.path != *path);
            if report.issues.len() != before {
                report.version += 1;
            }
        }
    }
    for event in failed.read() {
        if event.path.source().as_str() != Some(CONTENT_SOURCE) {
            continue;
        }
        let path = event.path.path().to_string_lossy().replace('\\', "/");
        let Some(sheet) = composed.sheet(&path) else {
            continue;
        };
        let message = event.error.to_string();
        warn!(
            "CSS errors in {} ({path}, layer {}):\n{message}",
            sheet.mod_id.as_deref().map_or_else(
                || format!("{:?} sheet", sheet.kind),
                |m| format!("mod `{m}`")
            ),
            sheet.layer
        );
        report.issues.retain(|i| i.path != path);
        report.issues.push(CssIssue {
            kind: sheet.kind,
            mod_id: sheet.mod_id.clone(),
            path,
            message,
        });
        report.version += 1;
    }
}

/// Every screen root gets the composed root sheet; its subtree inherits it.
fn style_roots(state: Res<UiCssState>, mut roots: Query<&mut Styled, Added<UiScreenRoot>>) {
    let Some(root) = &state.root else { return };
    for mut styled in &mut roots {
        *styled = Styled::new(root.clone());
    }
}

fn input_value(mode: InputMode) -> &'static str {
    match mode.0 {
        Device::Kbm => "kbm",
        Device::Gamepad => "gamepad",
    }
}

/// `UiClass` → flair's `ClassList`; screen roots also get the root classes and `input=kbm|gamepad`
/// (focus visuals per device are CSS: `[input=gamepad] button:focus { .. }`).
#[allow(clippy::type_complexity)]
fn apply_classes(
    mut commands: Commands,
    mode: Res<InputMode>,
    mut state: ResMut<UiCssState>,
    changed: Query<
        (Entity, Option<&UiClass>, Has<UiScreenRoot>),
        Or<(Changed<UiClass>, Added<UiScreenRoot>)>,
    >,
    roots: Query<(Entity, Option<&UiClass>), With<UiScreenRoot>>,
) {
    let input = input_value(*mode);
    let roots_changed = state.roots_dirty || state.input != input;
    if roots_changed {
        state.roots_dirty = false;
        state.input = input;
    }
    let class_list = |class: Option<&UiClass>, root: bool| {
        let mut list = ClassList::new(class.map_or("", |c| c.0.as_str()));
        if root {
            for c in &state.root_classes {
                list.add(c.clone());
            }
        }
        list
    };
    let mut apply = |entity: Entity, class: Option<&UiClass>, root: bool| {
        let mut e = commands.entity(entity);
        e.insert(class_list(class, root));
        if root {
            let mut attributes = AttributeList::new();
            attributes.set_attribute("input", input);
            e.insert(attributes);
        }
    };
    if roots_changed {
        for (entity, class) in &roots {
            apply(entity, class, true);
        }
    }
    for (entity, class, root) in &changed {
        if !(root && roots_changed) {
            apply(entity, class, root);
        }
    }
}
