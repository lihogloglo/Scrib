//! UI record kinds (ui.md "Data model and moddability").
//!
//! Mods define these in native record files (RON or TOML) like any other
//! serde kind; replacing a screen is defining or patching its record in a
//! later mod. The record id is the object's id; the defining mod is its owner.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use sf_records::{RecordDb, RecordError};

use crate::conditions::Condition;
use crate::ids::{InputContextId, ListId, PresetId, SlotId, TemplateId};
use crate::settings::SettingValue;

/// Record kind names.
pub const SCREEN_KIND: &str = "ui.screen";
pub const SLOT_KIND: &str = "ui.slot";
pub const COLUMN_KIND: &str = "ui.column";
pub const THEME_KIND: &str = "ui.theme";
pub const PRESET_KIND: &str = "ui.preset";
pub const SETTING_KIND: &str = "ui.setting";
/// Setting presets (graphics Low..Ultra); `ui.preset` is the layout preset.
pub const SETTING_PRESET_KIND: &str = "ui.setting_preset";

/// Registers every UI kind as a serde kind. Call once before loading mods.
pub fn register_kinds(db: &mut RecordDb) -> Result<(), RecordError> {
    db.register_serde::<ScreenDef>(SCREEN_KIND)?;
    db.register_serde::<SlotContribution>(SLOT_KIND)?;
    db.register_serde::<ColumnDef>(COLUMN_KIND)?;
    db.register_serde::<ThemeDef>(THEME_KIND)?;
    db.register_serde::<PresetDef>(PRESET_KIND)?;
    db.register_serde::<SettingDef>(SETTING_KIND)?;
    db.register_serde::<SettingPresetDef>(SETTING_PRESET_KIND)?;
    Ok(())
}

/// UI layers, bottom to top (Unreal Common UI). The order is fixed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Layer {
    Hud,
    Menu,
    Modal,
    Notification,
    Tooltip,
}

impl Layer {
    /// All layers, bottom to top.
    pub const ALL: [Layer; 5] = [
        Layer::Hud,
        Layer::Menu,
        Layer::Modal,
        Layer::Notification,
        Layer::Tooltip,
    ];

    /// Whether screens on this layer receive input actions. Notifications and
    /// tooltips never do; input falls through them.
    pub fn takes_input(self) -> bool {
        matches!(self, Layer::Hud | Layer::Menu | Layer::Modal)
    }

    pub fn index(self) -> usize {
        self as usize
    }
}

/// Whether an open screen pauses the game simulation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum PausePolicy {
    /// The world keeps running (HUD, notifications, a quick-loot panel).
    #[default]
    Never,
    /// The world pauses while the screen is open (most menus).
    WhileOpen,
}

/// How a screen or slot contribution is built.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreenImpl {
    /// A `.bsn` scene through `sf_vfs` (once Bevy 0.21 ships the file format).
    Bsn(String),
    /// A Rust template registered with `sf_ui_bevy` by id.
    Template(TemplateId),
    /// A Luau entry point of the owning mod (`sf_ui_lua`, parked with Lua).
    Lua { entry: String },
}

/// `ui.screen`: a screen any mod can add or replace.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScreenDef {
    pub layer: Layer,
    /// Input context active while this screen is the input target; `None`
    /// uses the screen id as the context id.
    #[serde(default)]
    pub input: Option<InputContextId>,
    #[serde(default)]
    pub pause: PausePolicy,
    /// Presets this screen supports; empty means all.
    #[serde(default)]
    pub presets: Vec<PresetId>,
    /// May open as a classic (draggable) window in presets that allow them.
    #[serde(default)]
    pub windowed: bool,
    pub implementation: ScreenImpl,
    /// Named extension slots this screen offers.
    #[serde(default)]
    pub slots: Vec<SlotId>,
    /// View-model kinds it binds (`ViewModel::KIND`), for diagnostics.
    #[serde(default)]
    pub view_models: Vec<String>,
}

/// `ui.slot`: one mod's contribution to a named slot.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SlotContribution {
    pub slot: SlotId,
    /// Lower first; ties keep load order.
    #[serde(default)]
    pub priority: i32,
    pub content: ScreenImpl,
}

/// How a list column sorts.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum SortKind {
    #[default]
    None,
    Text,
    Number,
}

/// Where a column's cell value comes from.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValueSource {
    /// A view-model field of the row (`ViewModel::field`).
    Field(String),
    /// A Luau function of the owning mod (parked with Lua).
    Lua { function: String },
}

/// `ui.column`: a column of a keyed list (SkyUI-style columns as data).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ColumnDef {
    pub list: ListId,
    /// Fluent key of the header text.
    pub header: String,
    /// CSS length (`5rem`); `None` lets the layout decide.
    #[serde(default)]
    pub width: Option<String>,
    #[serde(default)]
    pub sort: SortKind,
    pub value: ValueSource,
    /// Lower first; ties keep load order.
    #[serde(default)]
    pub priority: i32,
}

/// `ui.theme`: a look, swappable live. Tokens are CSS custom properties in its stylesheet.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThemeDef {
    /// Fluent key of the display name.
    pub name: String,
    /// VFS path of the theme stylesheet (`ui/themes/<id>/theme.css`).
    pub stylesheet: String,
    /// Font family lists with fallbacks, by role (`body`, `display`, `mono`).
    #[serde(default)]
    pub fonts: BTreeMap<String, Vec<String>>,
    #[serde(default)]
    pub icons: Option<String>,
    #[serde(default)]
    pub sounds: Option<String>,
    /// Presets it supports; empty means all.
    #[serde(default)]
    pub presets: Vec<PresetId>,
    /// Accessibility sub-sheets by token set (`high_contrast`, `deuteranopia`).
    #[serde(default)]
    pub variants: BTreeMap<String, String>,
}

/// `ui.preset`: a layout preset over the shared view-models.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PresetDef {
    /// Root class (`preset-pc`).
    pub root_class: String,
    #[serde(default)]
    pub stylesheet: Option<String>,
    /// Body text size in px at the 1080p reference (XAG 101: 18 PC, 26 TV).
    pub text_px: f32,
    /// Classic draggable windows allowed.
    #[serde(default)]
    pub classic_windows: bool,
}

impl PresetDef {
    /// Built-in presets (ui.md "Data model": `pc_dense` and `controller`).
    /// A mod record with the same id replaces them.
    pub fn builtins() -> [(&'static str, PresetDef); 2] {
        [
            (
                "pc_dense",
                PresetDef {
                    root_class: "preset-pc".into(),
                    stylesheet: None,
                    text_px: 18.0,
                    classic_windows: true,
                },
            ),
            (
                "controller",
                PresetDef {
                    root_class: "preset-controller".into(),
                    stylesheet: None,
                    text_px: 26.0,
                    classic_windows: false,
                },
            ),
        ]
    }
}

/// `ui.setting`: a setting the framework renders (MCM equivalent).
///
/// The record id is the setting key. Its value lives in the profile under
/// the owning mod's table (`[settings.<owner>]`), keyed by the record id
/// with a leading `<owner>.` stripped.
///
/// Every field after `kind` is optional (menus.md "Settings v2") and
/// defaults to the v1 behaviour, so older records load unchanged.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SettingDef {
    /// Fluent key of the label.
    pub label: String,
    /// Fluent key of a description/tooltip.
    #[serde(default)]
    pub description: Option<String>,
    /// Settings page; `None` puts it on the owning mod's page.
    #[serde(default)]
    pub page: Option<String>,
    #[serde(default)]
    pub group: Option<String>,
    pub kind: SettingKind,
    /// When a change takes effect.
    #[serde(default)]
    pub apply: SettingApply,
    /// Where the value is stored.
    #[serde(default)]
    pub scope: SettingScope,
    /// The row is enabled only while every condition holds (empty: always).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub enabled_if: Vec<Condition>,
    /// The row is shown only while every condition holds (empty: always).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub visible_if: Vec<Condition>,
    /// Fluent key shown on the row while it is disabled.
    #[serde(default)]
    pub disabled_reason: Option<String>,
    /// Position within the group; `None` keeps load order.
    #[serde(default)]
    pub order: Option<i32>,
    /// Fluent key of search words.
    #[serde(default)]
    pub keywords: Option<String>,
    /// Included in "Reset page" (`false` for language, bindings).
    #[serde(default = "yes")]
    pub reset: bool,
    /// How the value is displayed.
    #[serde(default)]
    pub format: ValueFormat,
}

impl SettingDef {
    /// A setting with a label and a kind, every other field at its default.
    pub fn new(label: impl Into<String>, kind: SettingKind) -> Self {
        Self {
            label: label.into(),
            description: None,
            page: None,
            group: None,
            kind,
            apply: SettingApply::default(),
            scope: SettingScope::default(),
            enabled_if: Vec::new(),
            visible_if: Vec::new(),
            disabled_reason: None,
            order: None,
            keywords: None,
            reset: true,
            format: ValueFormat::default(),
        }
    }
}

/// When a setting change takes effect.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum SettingApply {
    /// The consumer applies it on change.
    #[default]
    Live,
    /// Staged behind Apply/Revert; with `keep_seconds`, a keep-or-revert
    /// countdown follows Apply.
    Confirm {
        #[serde(default)]
        keep_seconds: Option<u32>,
    },
    /// Stored now, read at the next start.
    Restart,
}

/// Where a setting's value is stored.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum SettingScope {
    /// `[settings.<owner>]` in the profile.
    #[default]
    Profile,
    /// Per machine (display, graphics).
    Machine,
    /// In the save game.
    Save,
}

/// How a value is displayed (`raw`, `percent`, `ms`, `x`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ValueFormat {
    /// As stored.
    #[default]
    Raw,
    /// 0..1 shown as 0..100 %.
    Percent,
    /// Milliseconds.
    Ms,
    /// A multiplier ("1.5x").
    #[serde(rename = "x")]
    Times,
}

fn yes() -> bool {
    true
}

/// A `ui.setting_preset` record (menus.md "Settings v2", Presets): named
/// levels of setting values behind one Choice setting, the driver.
///
/// The driver's options are the level ids plus `"custom"`. Picking a level
/// writes its values ([`crate::apply_level`]); the driver's value is always
/// derived from the current values ([`crate::derived_level`]), never stored
/// as a separate truth.
///
/// Records naming the same driver merge, in load order, into the first one:
/// a mod adds keys to a level by defining its own record with that driver
/// (`levels = { low = { "x.key" = 1 } }`), and the last mod writing a key
/// wins. (A field-path patch can't: merge keys may not contain dots, and
/// setting keys do.)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SettingPresetDef {
    /// Key of the Choice setting that shows the levels (`sf.graphics_preset`).
    pub driver: String,
    /// Level id to the values it writes (setting key to value). Levels are
    /// matched in level-id order (byte-wise), so of two levels holding the
    /// same current values the one whose id sorts first is shown.
    #[serde(default)]
    pub levels: BTreeMap<String, BTreeMap<String, SettingValue>>,
}

/// A setting's type, default and range.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SettingKind {
    Bool {
        default: bool,
    },
    Int {
        default: i64,
        min: i64,
        max: i64,
        #[serde(default = "one")]
        step: i64,
    },
    Float {
        default: f64,
        min: f64,
        max: f64,
        #[serde(default)]
        step: Option<f64>,
        /// The slider moves on a logarithmic scale.
        #[serde(default)]
        log_scale: bool,
    },
    /// One of a set of option ids (labels are Fluent keys `<setting>-<option>`).
    Choice {
        default: String,
        /// The static options, used when `options_from` is absent.
        #[serde(default)]
        options: Vec<String>,
        /// Id of a registered options provider (`display.monitors`, ...).
        #[serde(default)]
        options_from: Option<String>,
    },
    Text {
        default: String,
        #[serde(default)]
        max_len: Option<usize>,
    },
    /// A button row with no value (opens a sub-screen, runs a calibration).
    Action,
}

fn one() -> i64 {
    1
}
