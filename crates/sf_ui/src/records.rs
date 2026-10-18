//! UI record kinds (ui.md "Data model and moddability").
//!
//! Mods define these in native record files (RON or TOML) like any other
//! serde kind; replacing a screen is defining or patching its record in a
//! later mod. The record id is the object's id; the defining mod is its owner.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use sf_records::{RecordDb, RecordError};

use crate::ids::{InputContextId, ListId, PresetId, SlotId, TemplateId};

/// Record kind names.
pub const SCREEN_KIND: &str = "ui.screen";
pub const SLOT_KIND: &str = "ui.slot";
pub const COLUMN_KIND: &str = "ui.column";
pub const THEME_KIND: &str = "ui.theme";
pub const PRESET_KIND: &str = "ui.preset";
pub const SETTING_KIND: &str = "ui.setting";

/// Registers every UI kind as a serde kind. Call once before loading mods.
pub fn register_kinds(db: &mut RecordDb) -> Result<(), RecordError> {
    db.register_serde::<ScreenDef>(SCREEN_KIND)?;
    db.register_serde::<SlotContribution>(SLOT_KIND)?;
    db.register_serde::<ColumnDef>(COLUMN_KIND)?;
    db.register_serde::<ThemeDef>(THEME_KIND)?;
    db.register_serde::<PresetDef>(PRESET_KIND)?;
    db.register_serde::<SettingDef>(SETTING_KIND)?;
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
    },
    /// One of a fixed set of option ids (labels are Fluent keys `<setting>-<option>`).
    Choice {
        default: String,
        options: Vec<String>,
    },
    Text {
        default: String,
        #[serde(default)]
        max_len: Option<usize>,
    },
}

fn one() -> i64 {
    1
}
