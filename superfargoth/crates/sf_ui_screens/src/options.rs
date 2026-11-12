//! The options (settings) screen (ui.md "Screens": settings + keybinds + mod settings) over the
//! registry's `ui.setting` records: [`OptionsModel::from_settings`] groups them into pages (the
//! record's `page`, else the owning mod's id, in first-definition order, so the framework's
//! audio page comes first), each row a toggle, a choice or a slider (int or float) at the
//! store's current value. Text settings have no editor here and are left out.
//!
//! The screen applies nothing: a change updates the row it shows and goes out as an
//! [`OptionsEvent`] ([`OptionsChange::Set`]); the app writes the store (`UiSettingsStore`),
//! saves the profile and applies the value. A "Key bindings" row asks the app to open the
//! keybinds screen ([`OptionsChange::OpenKeybinds`]); Back (or the back button) sends
//! [`OptionsChange::Back`] and closes the screen.
//!
//! Input: up and down move between the page's enabled rows, then "Key bindings" and Back (no
//! wrap); left and right change the focused row (a toggle flips, a choice cycles, a slider
//! steps, clamped to its range); confirm or a click flips a toggle and cycles a choice forward;
//! the arrow buttons step; tab prev / next (Q / E, the triggers) switch pages, wrapping. A
//! disabled row (the app sets `enabled = false`; `ui.setting` has no condition yet) is shown
//! greyed and skipped.
//!
//! Widgets (`bevy_ui_widgets`): a slider row's bar is a [`Slider`] (drag the thumb or the
//! track, click the track to jump; the value snaps to the row's step grid and goes out as a
//! `Set` like a step), a toggle row's box a [`Checkbox`]; both follow the model (a step moves
//! the thumb, a flip checks the box) and carry `InteractionDisabled` on a disabled row.
//!
//! Description pane (right of the rows): the row under the pointer, else the focused row
//! ([`OptionsScreen::described`]; hover never moves the focus): its label, the record's
//! `description`, its `disabled_reason` while disabled, an "Applies after restart" badge for
//! `apply = Restart`, and the default value; empty lines are hidden.
//!
//! Strings: a row's label is its record's Fluent `label`, a choice option's is
//! `<label>-<option>`, a page's `set-page-<page>`; a missing string reads as the id made
//! readable (`move_speed` reads "Move speed"), except a description or a disabled reason,
//! which reads as none.

use std::collections::HashMap;

use bevy_color::Color;
use bevy_ecs::prelude::*;
use bevy_input_focus::{FocusCause, InputFocus};
use bevy_math::CompassOctant;
use bevy_picking::events::{PointerEnter, PointerLeave};
use bevy_text::TextColor;
use bevy_ui::prelude::*;
use bevy_ui::Checked;
use bevy_ui_widgets::{
    Activate, Checkbox, Slider, SliderOrientation, SliderRange, SliderStep, SliderThumb,
    SliderValue, TrackClick, ValueChange,
};
use sf_ui::records::{SettingApply, SettingKind};
use sf_ui::{Localizer, SettingValue, SettingsStore, UiRegistry};
use sf_ui_bevy::{
    ScreenStackRes, UiAction, UiActionEvent, UiHandlesBack, UiNavEvent, UiOwnsFocus, UiScreenRoot,
};

use crate::keyed::KeyedChildren;
use crate::labels::ScreenLabels;
use crate::widgets::{
    ScreenColors, StatBar, button, centred_window, classes, framed_button, label, part, part_as,
    relabel, role, set_disabled, spawn_bar, spawn_button,
};
use crate::{is_target, screen_of};

/// Steps of a float slider whose record gives no step.
pub const DEFAULT_FLOAT_STEPS: f64 = 20.0;

/// The slider thumb's width (px); it travels the track less its width, so it never overhangs.
const THUMB_WIDTH: f32 = 10.0;

/// The description pane's width (px).
const PANE_WIDTH: f32 = 280.0;

/// How a row edits its value.
#[derive(Clone, Debug, PartialEq)]
pub enum OptionKind {
    Toggle,
    /// Option ids and their labels, in order.
    Choice(Vec<(String, String)>),
    Int {
        min: i64,
        max: i64,
        step: i64,
    },
    Float {
        min: f64,
        max: f64,
        step: f64,
    },
}

/// One setting as the screen shows it.
#[derive(Clone, Debug, PartialEq)]
pub struct OptionRow {
    /// The setting key (the `ui.setting` record id).
    pub key: String,
    pub label: String,
    pub kind: OptionKind,
    pub value: SettingValue,
    /// Disabled rows are shown (class `disabled`), skipped by navigation and not changed.
    pub enabled: bool,
    /// The record's localized `description`; `None` without one or without its string.
    pub description: Option<String>,
    /// The record's localized `disabled_reason`, shown while the row is disabled.
    pub disabled_reason: Option<String>,
    /// `apply = Restart`: the value is read at the next start.
    pub restart: bool,
    /// The record's default value.
    pub default: SettingValue,
}

impl OptionRow {
    /// The value one step in `direction` (negative: left), or `None` when it would not change
    /// (a slider at its bound). A toggle flips either way; a choice cycles, wrapping.
    pub fn stepped(&self, direction: i32) -> Option<SettingValue> {
        let next = match (&self.kind, &self.value) {
            (OptionKind::Toggle, SettingValue::Bool(on)) => SettingValue::Bool(!on),
            (OptionKind::Choice(options), value) => {
                if options.is_empty() {
                    return None;
                }
                let current = match value {
                    SettingValue::Text(id) => options.iter().position(|(o, _)| o == id),
                    _ => None,
                };
                let len = options.len();
                let at = current.map_or(0, |at| {
                    if direction < 0 {
                        (at + len - 1) % len
                    } else {
                        (at + 1) % len
                    }
                });
                SettingValue::Text(options[at].0.clone())
            }
            (OptionKind::Int { min, max, step }, SettingValue::Int(v)) => SettingValue::Int(
                v.saturating_add(step.saturating_mul(i64::from(direction)))
                    .clamp(*min, *max),
            ),
            (OptionKind::Float { min, max, step }, SettingValue::Float(v)) => {
                // On the step grid from `min`, so repeated steps don't drift.
                let n = ((v - min) / step).round() + f64::from(direction);
                let next = (min + n * step).clamp(*min, *max);
                SettingValue::Float((next * 1e9).round() / 1e9)
            }
            _ => return None,
        };
        (next != self.value).then_some(next)
    }

    /// A slider's value nearest `value` on its step grid from `min`, clamped to its range (a
    /// drag or a track click lands here); `None` for toggles and choices.
    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
    pub fn snapped(&self, value: f64) -> Option<SettingValue> {
        match &self.kind {
            OptionKind::Int { min, max, step } => {
                let n = ((value - *min as f64) / *step as f64).round() as i64;
                Some(SettingValue::Int(
                    min.saturating_add(n.saturating_mul(*step))
                        .clamp(*min, *max),
                ))
            }
            OptionKind::Float { min, max, step } => {
                let n = ((value - min) / step).round();
                let next = (min + n * step).clamp(*min, *max);
                Some(SettingValue::Float((next * 1e9).round() / 1e9))
            }
            OptionKind::Toggle | OptionKind::Choice(_) => None,
        }
    }

    /// A slider's widget state: range, step and value, in f32 (the widget's unit).
    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
    fn slider(&self) -> Option<(SliderRange, SliderStep, SliderValue)> {
        match (&self.kind, &self.value) {
            (OptionKind::Int { min, max, step }, SettingValue::Int(v)) => Some((
                SliderRange::new(*min as f32, *max as f32),
                SliderStep(*step as f32),
                SliderValue(*v as f32),
            )),
            (OptionKind::Float { min, max, step }, SettingValue::Float(v)) => Some((
                SliderRange::new(*min as f32, *max as f32),
                SliderStep(*step as f32),
                SliderValue(*v as f32),
            )),
            _ => None,
        }
    }

    /// A toggle's state; `None` for the other kinds.
    fn checked(&self) -> Option<bool> {
        match (&self.kind, &self.value) {
            (OptionKind::Toggle, SettingValue::Bool(on)) => Some(*on),
            _ => None,
        }
    }

    /// The value as shown: on / off, the option's label, the number (a 0..1 slider as a
    /// percentage).
    pub fn value_text(&self, labels: &ScreenLabels) -> String {
        self.format(&self.value, labels)
    }

    /// The default value as shown (as [`OptionRow::value_text`]).
    pub fn default_text(&self, labels: &ScreenLabels) -> String {
        self.format(&self.default, labels)
    }

    fn format(&self, value: &SettingValue, labels: &ScreenLabels) -> String {
        match (&self.kind, value) {
            (OptionKind::Toggle, SettingValue::Bool(on)) => {
                if *on {
                    labels.on.clone()
                } else {
                    labels.off.clone()
                }
            }
            (OptionKind::Choice(options), SettingValue::Text(id)) => options
                .iter()
                .find(|(o, _)| o == id)
                .map_or_else(|| id.clone(), |(_, l)| l.clone()),
            (OptionKind::Float { min, max, step }, SettingValue::Float(v)) => {
                if min.abs() < f64::EPSILON && (max - 1.0).abs() < f64::EPSILON {
                    format!("{}%", (v * 100.0).round())
                } else {
                    // As many decimals as the step has (0.25: two), at most three.
                    let decimals = [1.0, 10.0, 100.0]
                        .iter()
                        .position(|scale| {
                            let scaled = step * scale;
                            (scaled - scaled.round()).abs() < 1e-6
                        })
                        .unwrap_or(3);
                    format!("{v:.decimals$}")
                }
            }
            (_, SettingValue::Int(v)) => v.to_string(),
            (_, SettingValue::Float(v)) => v.to_string(),
            (_, SettingValue::Bool(v)) => v.to_string(),
            (_, SettingValue::Text(v)) => v.clone(),
        }
    }

    /// A slider's position, `(value - min, max - min)`; `None` for toggles and choices.
    /// For the bar only, so f32 precision is plenty.
    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
    fn fraction(&self) -> Option<(f32, f32)> {
        match (&self.kind, &self.value) {
            (OptionKind::Int { min, max, .. }, SettingValue::Int(v)) => {
                Some(((v - min) as f32, (max - min) as f32))
            }
            (OptionKind::Float { min, max, .. }, SettingValue::Float(v)) => {
                Some(((v - min) as f32, (max - min) as f32))
            }
            _ => None,
        }
    }

    fn kind_class(&self) -> &'static str {
        match self.kind {
            OptionKind::Toggle => "toggle",
            OptionKind::Choice(_) => "choice",
            OptionKind::Int { .. } | OptionKind::Float { .. } => "slider",
        }
    }
}

/// A page (tab) of settings.
#[derive(Clone, Debug, PartialEq)]
pub struct OptionsPage {
    /// The record's `page`, or the owning mod's id.
    pub id: String,
    pub label: String,
    pub rows: Vec<OptionRow>,
}

/// The screen's view-model: pages in order, each with its rows.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct OptionsModel {
    pub pages: Vec<OptionsPage>,
}

impl OptionsModel {
    /// Every `ui.setting` of `registry` (built-ins first) grouped by page, at the store's values
    /// (defaults without a store), labelled through `localizer` (readable ids without one).
    pub fn from_settings(
        registry: &UiRegistry,
        store: Option<&SettingsStore>,
        localizer: Option<&Localizer>,
    ) -> Self {
        let string = |key: &str| match localizer {
            Some(l) if !key.is_empty() && l.has(key) => Some(l.format(key, None).into_owned()),
            _ => None,
        };
        let text = |key: &str, fallback: &str| string(key).unwrap_or_else(|| readable(fallback));
        // Descriptions and reasons have no readable-id fallback: a missing string reads as none.
        let optional = |key: Option<&str>| key.and_then(string);
        let mut pages: Vec<OptionsPage> = Vec::new();
        for def in registry.settings() {
            let key = def.id.as_str();
            let (kind, default) = match &def.def.kind {
                SettingKind::Bool { default } => (OptionKind::Toggle, SettingValue::Bool(*default)),
                SettingKind::Int {
                    default,
                    min,
                    max,
                    step,
                } => (
                    OptionKind::Int {
                        min: *min,
                        max: *max,
                        step: (*step).max(1),
                    },
                    SettingValue::Int(*default),
                ),
                SettingKind::Float {
                    default,
                    min,
                    max,
                    step,
                    ..
                } => {
                    let step = step
                        .filter(|s| *s > 0.0)
                        .unwrap_or((max - min) / DEFAULT_FLOAT_STEPS);
                    if step <= 0.0 {
                        continue;
                    }
                    (
                        OptionKind::Float {
                            min: *min,
                            max: *max,
                            step,
                        },
                        SettingValue::Float(*default),
                    )
                }
                SettingKind::Choice {
                    default, options, ..
                } => (
                    OptionKind::Choice(
                        options
                            .iter()
                            .map(|o| (o.clone(), text(&format!("{}-{o}", def.def.label), o)))
                            .collect(),
                    ),
                    SettingValue::Text(default.clone()),
                ),
                SettingKind::Text { .. } | SettingKind::Action => continue,
            };
            let local = key.strip_prefix(&format!("{}.", def.owner)).unwrap_or(key);
            let row = OptionRow {
                key: key.to_owned(),
                label: text(&def.def.label, local),
                kind,
                value: store
                    .and_then(|s| s.get(key))
                    .cloned()
                    .unwrap_or_else(|| default.clone()),
                enabled: true,
                description: optional(def.def.description.as_deref()),
                disabled_reason: optional(def.def.disabled_reason.as_deref()),
                restart: def.def.apply == SettingApply::Restart,
                default,
            };
            let page_id = def
                .def
                .page
                .clone()
                .unwrap_or_else(|| def.owner.to_string());
            match pages.iter_mut().find(|p| p.id == page_id) {
                Some(page) => page.rows.push(row),
                None => pages.push(OptionsPage {
                    label: text(&format!("set-page-{page_id}"), &page_id),
                    id: page_id,
                    rows: vec![row],
                }),
            }
        }
        Self { pages }
    }

    /// The row of a setting, on any page.
    pub fn row(&self, key: &str) -> Option<&OptionRow> {
        self.pages
            .iter()
            .flat_map(|p| &p.rows)
            .find(|r| r.key == key)
    }

    fn row_mut(&mut self, key: &str) -> Option<&mut OptionRow> {
        self.pages
            .iter_mut()
            .flat_map(|p| &mut p.rows)
            .find(|r| r.key == key)
    }

    /// Enables or disables a setting's row.
    pub fn set_enabled(&mut self, key: &str, enabled: bool) -> &mut Self {
        if let Some(row) = self.row_mut(key) {
            row.enabled = enabled;
        }
        self
    }
}

/// `move_speed` / `move-speed` / `ui.move_speed` as "Move speed".
fn readable(id: &str) -> String {
    let id = id.rsplit('.').next().unwrap_or(id);
    let spaced = id.replace(['_', '-'], " ");
    let mut chars = spaced.chars();
    chars.next().map_or_else(String::new, |first| {
        first.to_uppercase().chain(chars).collect()
    })
}

/// What the player did.
#[derive(Clone, Debug, PartialEq)]
pub enum OptionsChange {
    /// A setting's new value (the screen already shows it).
    Set(String, SettingValue),
    /// Open the keybinds screen.
    OpenKeybinds,
    /// The screen has closed.
    Back,
}

/// A change, for the app to apply.
#[derive(Message, Clone, Debug, PartialEq)]
pub struct OptionsEvent {
    pub screen: Entity,
    pub change: OptionsChange,
}

/// Where the screen's focus is.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum OptionsFocus {
    #[default]
    Start,
    /// A row of the current page, by setting key.
    Row(String),
    Keybinds,
    Back,
}

/// On a screen root: the options screen. The app may replace `model` at any time (rows are
/// keyed by setting key; the page is kept by index).
#[derive(Component, Clone, Debug, Default)]
#[require(Node, UiOwnsFocus, UiHandlesBack)]
pub struct OptionsScreen {
    pub model: OptionsModel,
    page: usize,
    focus: OptionsFocus,
    /// The row under the pointer, by setting key.
    hovered: Option<String>,
}

impl OptionsScreen {
    pub fn new(model: OptionsModel) -> Self {
        Self {
            model,
            ..Self::default()
        }
    }

    /// The current page's index.
    pub fn page(&self) -> usize {
        self.page
    }

    pub fn focus(&self) -> &OptionsFocus {
        &self.focus
    }

    /// Shows page `page` (clamped), focus on its first enabled row.
    pub fn set_page(&mut self, page: usize) {
        self.page = page.min(self.model.pages.len().saturating_sub(1));
        self.focus = self.home();
        self.hovered = None;
    }

    /// The row under the pointer (the screen's pointer observers set it), by setting key.
    pub fn hovered(&self) -> Option<&str> {
        self.hovered.as_deref()
    }

    /// Sets the row under the pointer (`None`: none). It never moves the focus.
    pub fn set_hovered(&mut self, key: Option<String>) {
        self.hovered = key;
    }

    /// The row the description pane shows: the hovered row of the current page (a disabled
    /// one included, so its reason can be read), else the focused row; `None` on Key bindings
    /// and Back.
    pub fn described(&self) -> Option<&OptionRow> {
        let hovered = self
            .hovered
            .as_deref()
            .and_then(|key| self.rows().iter().find(|r| r.key == key));
        hovered.or_else(|| match &self.focus {
            OptionsFocus::Row(key) => self.rows().iter().find(|r| r.key == *key),
            _ => None,
        })
    }

    fn rows(&self) -> &[OptionRow] {
        self.model
            .pages
            .get(self.page)
            .map_or(&[], |p| p.rows.as_slice())
    }

    /// The focus stops in order: enabled rows, then Key bindings and Back.
    fn stops(&self) -> Vec<OptionsFocus> {
        self.rows()
            .iter()
            .filter(|r| r.enabled)
            .map(|r| OptionsFocus::Row(r.key.clone()))
            .chain([OptionsFocus::Keybinds, OptionsFocus::Back])
            .collect()
    }

    fn home(&self) -> OptionsFocus {
        self.stops().swap_remove(0)
    }

    fn step(&self, south: bool) -> OptionsFocus {
        let stops = self.stops();
        let Some(at) = stops.iter().position(|s| *s == self.focus) else {
            return self.home();
        };
        let next = if south {
            (at + 1).min(stops.len() - 1)
        } else {
            at.saturating_sub(1)
        };
        stops[next].clone()
    }

    /// Steps an enabled row of the current page; the new value, when it changed.
    fn change(&mut self, key: &str, direction: i32) -> Option<OptionsChange> {
        let page = self.model.pages.get_mut(self.page)?;
        let row = page.rows.iter_mut().find(|r| r.key == key && r.enabled)?;
        let value = row.stepped(direction)?;
        row.value = value.clone();
        Some(OptionsChange::Set(key.to_owned(), value))
    }

    /// Sets an enabled row of the current page (a widget's value); the change, when it changed.
    fn set(&mut self, key: &str, value: SettingValue) -> Option<OptionsChange> {
        let page = self.model.pages.get_mut(self.page)?;
        let row = page.rows.iter_mut().find(|r| r.key == key && r.enabled)?;
        if row.value == value {
            return None;
        }
        row.value = value.clone();
        Some(OptionsChange::Set(key.to_owned(), value))
    }
}

/// On a slider row's [`Slider`]: the setting it edits.
#[derive(Component, Clone, Debug)]
pub(crate) struct OptionsSlider(String);

/// On a toggle row's [`Checkbox`]: the setting it edits.
#[derive(Component, Clone, Debug)]
pub(crate) struct OptionsCheckbox(String);

/// A clickable part of the options screen.
#[derive(Component, Clone, Debug, PartialEq, Eq)]
pub enum OptionsPart {
    Tab(usize),
    Row(String),
    /// A row's left / right arrow.
    Less(String),
    More(String),
    Keybinds,
    Back,
}

/// The parts of a row whose content changes.
#[derive(Debug, Clone, Copy)]
struct RowParts {
    value: Entity,
    /// Left, right (sliders and choices; a toggle has its checkbox instead).
    arrows: Option<[Entity; 2]>,
    /// A slider row's [`Slider`] (also its [`StatBar`]) and thumb.
    slider: Option<(Entity, Entity)>,
    /// A toggle row's [`Checkbox`].
    checkbox: Option<Entity>,
}

/// The description pane's text parts (right of the rows). An empty one is hidden
/// (`Display::None`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OptionsPane {
    /// The pane itself.
    pub root: Entity,
    pub label: Entity,
    pub description: Entity,
    /// The disabled reason, while the row is disabled.
    pub reason: Entity,
    /// "Applies after restart" (`apply = Restart`).
    pub restart: Entity,
    /// "Default: <value>".
    pub default: Entity,
}

/// What the pane shows, in [`OptionsPane`] order.
type PaneText = [String; 5];

impl OptionsPane {
    fn texts(&self) -> [Entity; 5] {
        [
            self.label,
            self.description,
            self.reason,
            self.restart,
            self.default,
        ]
    }
}

/// The pane's text for `row` (all empty without one).
fn pane_text(row: Option<&OptionRow>, labels: &ScreenLabels) -> PaneText {
    let Some(row) = row else {
        return PaneText::default();
    };
    [
        row.label.clone(),
        row.description.clone().unwrap_or_default(),
        row.disabled_reason
            .clone()
            .filter(|_| !row.enabled)
            .unwrap_or_default(),
        if row.restart {
            labels.restart.clone()
        } else {
            String::new()
        },
        format!("{}: {}", labels.default, row.default_text(labels)),
    ]
}

/// The screen's parts and what they show.
#[derive(Component, Debug)]
pub struct OptionsParts {
    tabs: KeyedChildren<usize>,
    rows: KeyedChildren<String>,
    row_parts: HashMap<String, RowParts>,
    /// Per row: the value text and enabled state on screen.
    shown: HashMap<String, (String, bool)>,
    /// The tab shown active.
    active_tab: Option<usize>,
    keybinds: Entity,
    back: Entity,
    pane: OptionsPane,
    /// The pane's text on screen.
    pane_shown: Option<PaneText>,
    seen: Option<(usize, OptionsModel)>,
}

impl OptionsParts {
    /// The description pane.
    pub fn pane(&self) -> OptionsPane {
        self.pane
    }

    /// The tab of page `page`.
    pub fn tab(&self, page: usize) -> Option<Entity> {
        self.tabs.get(&page)
    }

    /// A row of the current page.
    pub fn row(&self, key: &str) -> Option<Entity> {
        self.rows.get(&key.to_owned())
    }

    /// A row's value text.
    pub fn value(&self, key: &str) -> Option<Entity> {
        self.row_parts.get(key).map(|p| p.value)
    }

    /// A row's left and right arrows (sliders and choices; `None` for a toggle).
    pub fn arrows(&self, key: &str) -> Option<[Entity; 2]> {
        self.row_parts.get(key).and_then(|p| p.arrows)
    }

    /// A slider row's [`Slider`] (`bevy_ui_widgets`).
    pub fn slider(&self, key: &str) -> Option<Entity> {
        self.row_parts.get(key).and_then(|p| p.slider).map(|(s, _)| s)
    }

    /// A slider row's thumb ([`SliderThumb`]).
    pub fn thumb(&self, key: &str) -> Option<Entity> {
        self.row_parts.get(key).and_then(|p| p.slider).map(|(_, t)| t)
    }

    /// A toggle row's [`Checkbox`] (`bevy_ui_widgets`).
    pub fn checkbox(&self, key: &str) -> Option<Entity> {
        self.row_parts.get(key).and_then(|p| p.checkbox)
    }

    /// The setting keys of the rows shown, in order.
    pub fn row_keys(&self) -> Vec<String> {
        self.rows.iter().map(|(k, _)| k.clone()).collect()
    }

    pub fn keybinds(&self) -> Entity {
        self.keybinds
    }

    pub fn back(&self) -> Entity {
        self.back
    }

    fn target(&self, focus: &OptionsFocus) -> Option<Entity> {
        match focus {
            OptionsFocus::Start => None,
            OptionsFocus::Row(key) => self.rows.get(key),
            OptionsFocus::Keybinds => Some(self.keybinds),
            OptionsFocus::Back => Some(self.back),
        }
    }
}

pub(crate) fn spawn_options(
    mut commands: Commands,
    colors: Res<ScreenColors>,
    labels: Res<ScreenLabels>,
    roots: Query<Entity, Added<OptionsScreen>>,
) {
    for root in &roots {
        commands.entity(root).insert((
            part_as("sf-options", role::WINDOW),
            centred_window(Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Stretch,
                padding: UiRect::all(Val::Px(16.0)),
                row_gap: Val::Px(8.0),
                min_width: Val::Px(480.0),
                max_width: Val::Percent(100.0),
                max_height: Val::Percent(100.0),
                ..Node::default()
            }),
            BackgroundColor(colors.panel),
        ));
        commands.spawn((
            label("sf-options-title", labels.options.clone(), &colors),
            ChildOf(root),
        ));
        let row_of = |commands: &mut Commands, name: &'static str, gap: f32| {
            commands
                .spawn((
                    part(name),
                    Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(gap),
                        row_gap: Val::Px(4.0),
                        ..Node::default()
                    },
                    ChildOf(root),
                ))
                .id()
        };
        let tabs = row_of(&mut commands, "sf-options-tabs", 4.0);
        // The body: the rows, then the description pane to their right (a bottom pane by
        // layout preset comes with the presets).
        let body = row_of(&mut commands, "sf-options-body", 16.0);
        let rows = commands
            .spawn((
                part_as("sf-options-rows", role::BOX),
                Node {
                    flex_direction: FlexDirection::Column,
                    flex_grow: 1.0,
                    row_gap: Val::Px(4.0),
                    ..Node::default()
                },
                ChildOf(body),
            ))
            .id();
        let pane = spawn_pane(&mut commands, body, &colors);
        let footer = row_of(&mut commands, "sf-options-footer", 8.0);
        let keybinds = commands
            .spawn((
                framed_button("sf-options-keybinds", labels.keybinds.clone(), &colors),
                OptionsPart::Keybinds,
                ChildOf(footer),
            ))
            .id();
        let back = commands
            .spawn((
                framed_button("sf-options-back", labels.back.clone(), &colors),
                OptionsPart::Back,
                ChildOf(footer),
            ))
            .id();
        commands.entity(root).insert(OptionsParts {
            tabs: KeyedChildren::new(tabs),
            rows: KeyedChildren::new(rows),
            row_parts: HashMap::new(),
            shown: HashMap::new(),
            active_tab: None,
            keybinds,
            back,
            pane,
            pane_shown: None,
            seen: None,
        });
    }
}

/// The description pane under `body`: label, description, disabled reason, restart badge,
/// default value, each hidden until it has text.
fn spawn_pane(commands: &mut Commands, body: Entity, colors: &ScreenColors) -> OptionsPane {
    let root = commands
        .spawn((
            part_as("sf-options-pane", role::BOX),
            Node {
                flex_direction: FlexDirection::Column,
                width: Val::Px(PANE_WIDTH),
                flex_shrink: 0.0,
                row_gap: Val::Px(6.0),
                padding: UiRect::all(Val::Px(8.0)),
                ..Node::default()
            },
            BackgroundColor(colors.cell),
            ChildOf(body),
        ))
        .id();
    let mut text = |name: &'static str, color: Color| {
        commands
            .spawn((
                part(name),
                Text::new(String::new()),
                colors.font(1.0),
                TextColor(color),
                Node {
                    display: Display::None,
                    ..Node::default()
                },
                ChildOf(root),
            ))
            .id()
    };
    OptionsPane {
        root,
        label: text("sf-options-pane-label", colors.highlight),
        description: text("sf-options-pane-description", colors.text),
        reason: text("sf-options-pane-reason", colors.unavailable),
        restart: text("sf-options-pane-restart", colors.new),
        default: text("sf-options-pane-default", colors.muted),
    }
}

/// A row: label, left arrow, value (and a bar for sliders), right arrow.
fn spawn_row(
    commands: &mut Commands,
    row: &OptionRow,
    labels: &ScreenLabels,
    colors: &ScreenColors,
) -> (Entity, RowParts) {
    let row_node = commands
        .spawn((
            button("sf-options-row", row.label.clone(), colors),
            OptionsPart::Row(row.key.clone()),
        ))
        .insert(Node {
            padding: UiRect::axes(Val::Px(8.0), Val::Px(4.0)),
            column_gap: Val::Px(16.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            ..Node::default()
        })
        .id();
    // The label is the button's first child (relabelled in place); the controls follow it.
    let controls = commands
        .spawn((
            part("sf-options-controls"),
            Node {
                column_gap: Val::Px(8.0),
                align_items: AlignItems::Center,
                ..Node::default()
            },
            ChildOf(row_node),
        ))
        .id();
    // A toggle: its checkbox, then the value; no arrows.
    let checkbox = row.checked().map(|_| {
        commands
            .spawn((
                part("sf-options-check"),
                Checkbox,
                OptionsCheckbox(row.key.clone()),
                Node {
                    width: Val::Px(18.0),
                    height: Val::Px(18.0),
                    border: UiRect::all(Val::Px(2.0)),
                    flex_shrink: 0.0,
                    ..Node::default()
                },
                BorderColor::all(colors.text),
                BackgroundColor(colors.bar_track),
                ChildOf(controls),
            ))
            .id()
    });
    let less = checkbox.is_none().then(|| {
        let (less, _) = spawn_button(
            commands,
            Some(controls),
            "sf-options-less",
            &labels.prev,
            colors,
        );
        commands
            .entity(less)
            .insert(OptionsPart::Less(row.key.clone()));
        less
    });
    let value = commands
        .spawn((
            label("sf-options-value", String::new(), colors),
            Node {
                min_width: Val::Px(80.0),
                ..Node::default()
            },
            ChildOf(controls),
        ))
        .id();
    let slider = row.fraction().zip(row.slider()).map(|((current, max), widget)| {
        spawn_slider(commands, controls, &row.key, (current, max), widget, colors)
    });
    let arrows = less.map(|less| {
        let (more, _) = spawn_button(
            commands,
            Some(controls),
            "sf-options-more",
            &labels.next,
            colors,
        );
        commands
            .entity(more)
            .insert(OptionsPart::More(row.key.clone()));
        [less, more]
    });
    (
        row_node,
        RowParts {
            value,
            arrows,
            slider,
            checkbox,
        },
    )
}

/// A slider row's bar as a [`Slider`]: the [`StatBar`] track (its fill shows the value), a
/// travel node inset by the thumb's width, and the [`SliderThumb`] in it (slider.rs: the
/// widget measures the thumb and reduces the travel by it). Returns the slider and the thumb.
fn spawn_slider(
    commands: &mut Commands,
    controls: Entity,
    key: &str,
    (current, max): (f32, f32),
    (range, step, value): (SliderRange, SliderStep, SliderValue),
    colors: &ScreenColors,
) -> (Entity, Entity) {
    let slider = spawn_bar(
        commands,
        controls,
        "sf-options-slider",
        StatBar {
            current,
            max,
            kind: "setting".into(),
        },
        colors,
    );
    commands.entity(slider).insert((
        Slider {
            // Click the track to jump there (the settings-slider norm), then drag on.
            track_click: TrackClick::Snap,
            orientation: SliderOrientation::Horizontal,
        },
        range,
        step,
        value,
        OptionsSlider(key.to_owned()),
    ));
    let travel = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                right: Val::Px(THUMB_WIDTH),
                top: Val::Px(0.0),
                bottom: Val::Px(0.0),
                ..Node::default()
            },
            ChildOf(slider),
        ))
        .id();
    let thumb = commands
        .spawn((
            part("sf-options-thumb"),
            SliderThumb,
            Node {
                position_type: PositionType::Absolute,
                left: thumb_left(range, value),
                top: Val::Px(-4.0),
                width: Val::Px(THUMB_WIDTH),
                height: Val::Px(18.0),
                ..Node::default()
            },
            BackgroundColor(colors.text),
            ChildOf(travel),
        ))
        .id();
    (slider, thumb)
}

/// The thumb's offset in its travel node.
fn thumb_left(range: SliderRange, value: SliderValue) -> Val {
    Val::Percent(range.thumb_position(value.0).clamp(0.0, 1.0) * 100.0)
}

/// The row widgets `sync_options` writes: bars, slider values, thumbs, checkbox states.
type RowWidgets<'w, 's> = (
    Query<'w, 's, &'static mut StatBar>,
    Query<'w, 's, &'static SliderValue>,
    Query<'w, 's, &'static mut Node, With<SliderThumb>>,
    Query<'w, 's, Has<Checked>, With<Checkbox>>,
);

/// Model and page → tabs, rows (keyed by setting key), values, and focus. Writes only what
/// changed.
#[allow(clippy::too_many_lines)]
pub(crate) fn sync_options(
    mut commands: Commands,
    stack: Res<ScreenStackRes>,
    colors: Res<ScreenColors>,
    labels: Res<ScreenLabels>,
    mut screens: Query<(&UiScreenRoot, &mut OptionsScreen, &mut OptionsParts)>,
    (mut bars, sliders, mut thumbs, checks): RowWidgets,
    mut focus: ResMut<InputFocus>,
) {
    for (root, mut screen, mut parts) in &mut screens {
        let parts = &mut *parts;
        let colors = &*colors;
        let labels = &*labels;
        let stale = parts
            .seen
            .as_ref()
            .is_none_or(|(page, model)| *page != screen.page || *model != screen.model);
        if stale {
            if screen.page >= screen.model.pages.len() && !screen.model.pages.is_empty() {
                screen.page = screen.model.pages.len() - 1;
            }
            let model = &screen.model;
            let page = screen.page;
            let pages_changed = parts.seen.as_ref().is_none_or(|(_, m)| {
                m.pages.len() != model.pages.len()
                    || m.pages
                        .iter()
                        .zip(&model.pages)
                        .any(|(a, b)| a.label != b.label)
            });
            if pages_changed {
                parts.tabs.clear(&mut commands);
                parts.active_tab = None;
            }
            parts
                .tabs
                .sync(&mut commands, 0..model.pages.len(), |commands, i| {
                    commands
                        .spawn((
                            button("sf-options-tab", model.pages[*i].label.clone(), colors),
                            OptionsPart::Tab(*i),
                        ))
                        .id()
                });
            if parts.active_tab != Some(page) {
                for (i, tab) in parts.tabs.iter() {
                    commands
                        .entity(tab)
                        .insert(classes("sf-options-tab", &[("active", *i == page)]));
                }
                parts.active_tab = Some(page);
            }

            let rows = model.pages.get(page).map_or(&[][..], |p| &p.rows[..]);
            let row_parts = &mut parts.row_parts;
            parts.rows.sync(
                &mut commands,
                rows.iter().map(|r| r.key.clone()),
                |commands, key| {
                    let row = rows.iter().find(|r| r.key == *key).expect("synced key");
                    let (entity, made) = spawn_row(commands, row, labels, colors);
                    row_parts.insert(key.clone(), made);
                    entity
                },
            );
            parts
                .row_parts
                .retain(|k, _| rows.iter().any(|r| r.key == *k));
            parts.shown.retain(|k, _| rows.iter().any(|r| r.key == *k));
            for row in rows {
                let (Some(entity), Some(made)) = (
                    parts.rows.get(&row.key),
                    parts.row_parts.get(&row.key).copied(),
                ) else {
                    continue;
                };
                let text = row.value_text(labels);
                let was = parts.shown.get(&row.key).cloned();
                if was.as_ref().map(|(t, _)| t) != Some(&text) {
                    let color = if row.enabled {
                        colors.text
                    } else {
                        colors.muted
                    };
                    commands
                        .entity(made.value)
                        .insert((Text::new(text.clone()), TextColor(color)));
                }
                // The widgets follow the model (a step, a flip, an app's new model).
                if let (Some((slider, thumb)), Some((current, max)), Some((range, _, value))) =
                    (made.slider, row.fraction(), row.slider())
                {
                    if let Ok(mut b) = bars.get_mut(slider)
                        && (b.current, b.max) != (current, max)
                    {
                        b.current = current;
                        b.max = max;
                    }
                    if sliders.get(slider).ok() != Some(&value) {
                        commands.entity(slider).insert(value);
                    }
                    let left = thumb_left(range, value);
                    if let Ok(mut node) = thumbs.get_mut(thumb)
                        && node.left != left
                    {
                        node.left = left;
                    }
                }
                if let (Some(checkbox), Some(on)) = (made.checkbox, row.checked())
                    && checks.get(checkbox).ok() != Some(on)
                {
                    if on {
                        commands.entity(checkbox).insert(Checked);
                    } else {
                        commands.entity(checkbox).remove::<Checked>();
                    }
                    commands.entity(checkbox).insert((
                        classes("sf-options-check", &[("checked", on)]),
                        BackgroundColor(if on { colors.text } else { colors.bar_track }),
                    ));
                }
                if was.as_ref().map(|(_, e)| *e) != Some(row.enabled) {
                    set_disabled(&mut commands, entity, !row.enabled);
                    for widget in made.slider.map(|(s, _)| s).into_iter().chain(made.checkbox) {
                        set_disabled(&mut commands, widget, !row.enabled);
                    }
                    commands.entity(entity).insert(classes(
                        "sf-options-row",
                        &[(row.kind_class(), true), ("disabled", !row.enabled)],
                    ));
                    let color = if row.enabled {
                        colors.text
                    } else {
                        colors.muted
                    };
                    relabel(&mut commands, entity, row.label.clone(), color);
                    commands.entity(made.value).insert(TextColor(color));
                }
                parts.shown.insert(row.key.clone(), (text, row.enabled));
            }
            parts.seen = Some((screen.page, screen.model.clone()));
        }

        // Focus: a stop of the current page, else its home.
        if !screen.stops().contains(&screen.focus) {
            screen.focus = screen.home();
        }
        // The pane: the hovered row, else the focused one.
        let pane = pane_text(screen.described(), labels);
        if parts.pane_shown.as_ref() != Some(&pane) {
            // Spawned empty and hidden, so "nothing shown yet" compares as all empty.
            let was = parts.pane_shown.take().unwrap_or_default();
            for ((entity, text), old) in parts.pane.texts().into_iter().zip(&pane).zip(&was) {
                if text == old {
                    continue;
                }
                let display = if text.is_empty() {
                    Display::None
                } else {
                    Display::Flex
                };
                commands.entity(entity).insert((
                    Text::new(text.clone()),
                    Node {
                        display,
                        ..Node::default()
                    },
                ));
            }
            parts.pane_shown = Some(pane);
        }
        if !is_target(&stack, root) {
            continue;
        }
        if let Some(target) = parts.target(&screen.focus)
            && focus.get() != Some(target)
        {
            focus.set(target, FocusCause::Navigated);
        }
    }
}

/// Directions move the focus and change values; tabs switch pages; Back closes.
pub(crate) fn options_input(
    mut navs: MessageReader<UiNavEvent>,
    mut actions: MessageReader<UiActionEvent>,
    mut stack: ResMut<ScreenStackRes>,
    mut screens: Query<(Entity, &UiScreenRoot, &mut OptionsScreen)>,
    mut out: MessageWriter<OptionsEvent>,
) {
    let navs: Vec<UiNavEvent> = navs.read().cloned().collect();
    let actions: Vec<UiActionEvent> = actions.read().cloned().collect();
    for (entity, root, mut screen) in &mut screens {
        for nav in navs.iter().filter(|n| n.screen == root.0) {
            match nav.direction {
                CompassOctant::North | CompassOctant::South => {
                    let next = screen.step(nav.direction == CompassOctant::South);
                    if next != screen.focus {
                        screen.focus = next;
                    }
                }
                CompassOctant::West | CompassOctant::East => {
                    let OptionsFocus::Row(key) = screen.focus.clone() else {
                        continue;
                    };
                    let direction = if nav.direction == CompassOctant::East {
                        1
                    } else {
                        -1
                    };
                    if let Some(change) = screen.change(&key, direction) {
                        out.write(OptionsEvent {
                            screen: entity,
                            change,
                        });
                    }
                }
                _ => {}
            }
        }
        for action in actions.iter().filter(|a| a.screen == root.0) {
            let pages = screen.model.pages.len();
            match action.action {
                UiAction::TabNext | UiAction::TabPrev if pages > 1 => {
                    let delta = if action.action == UiAction::TabNext {
                        1
                    } else {
                        pages - 1
                    };
                    let page = (screen.page + delta) % pages;
                    screen.set_page(page);
                }
                UiAction::Back => {
                    out.write(OptionsEvent {
                        screen: entity,
                        change: OptionsChange::Back,
                    });
                    stack.0.close(root.0.as_str());
                }
                _ => {}
            }
        }
    }
}

/// Focus on a row of the current page, if it is enabled.
fn focus_row(screen: &mut OptionsScreen, key: &str) {
    let stop = OptionsFocus::Row(key.to_owned());
    if screen.stops().contains(&stop) {
        screen.focus = stop;
    }
}

/// A part activated (confirm, click): switch page, flip or cycle a row, step it, open the
/// keybinds or go back.
pub(crate) fn on_options_part(
    activate: On<Activate>,
    kinds: Query<&OptionsPart>,
    parents: Query<&ChildOf>,
    mut screens: Query<(&UiScreenRoot, &mut OptionsScreen)>,
    mut stack: ResMut<ScreenStackRes>,
    mut out: MessageWriter<OptionsEvent>,
) {
    let Ok(kind) = kinds.get(activate.entity) else {
        return;
    };
    let Some(entity) = screen_of(activate.entity, &parents, |e| screens.contains(e)) else {
        return;
    };
    let Ok((root, mut screen)) = screens.get_mut(entity) else {
        return;
    };
    let change = match kind {
        OptionsPart::Tab(page) => {
            if *page != screen.page {
                screen.set_page(*page);
            }
            None
        }
        OptionsPart::Row(key) => {
            focus_row(&mut screen, key);
            // Confirm flips a toggle and cycles a choice; a slider steps with the arrows.
            let pick = screen.rows().iter().any(|r| {
                r.key == *key
                    && !matches!(r.kind, OptionKind::Int { .. } | OptionKind::Float { .. })
            });
            if pick { screen.change(key, 1) } else { None }
        }
        OptionsPart::Less(key) => {
            focus_row(&mut screen, key);
            screen.change(key, -1)
        }
        OptionsPart::More(key) => {
            focus_row(&mut screen, key);
            screen.change(key, 1)
        }
        OptionsPart::Keybinds => {
            screen.focus = OptionsFocus::Keybinds;
            Some(OptionsChange::OpenKeybinds)
        }
        OptionsPart::Back => {
            stack.0.close(root.0.as_str());
            Some(OptionsChange::Back)
        }
    };
    if let Some(change) = change {
        out.write(OptionsEvent {
            screen: entity,
            change,
        });
    }
}

/// The pointer entered a row (or a part of it, from outside the row): the pane shows that row,
/// the focus stays.
pub(crate) fn on_options_enter(
    enter: On<PointerEnter>,
    kinds: Query<&OptionsPart>,
    parents: Query<&ChildOf>,
    mut screens: Query<&mut OptionsScreen>,
) {
    let Ok(OptionsPart::Row(key)) = kinds.get(enter.entity) else {
        return;
    };
    let Some(root) = screen_of(enter.entity, &parents, |e| screens.contains(e)) else {
        return;
    };
    if let Ok(mut screen) = screens.get_mut(root)
        && screen.hovered() != Some(key.as_str())
    {
        screen.set_hovered(Some(key.clone()));
    }
}

/// The pointer left a row and its parts: the pane falls back to the focused row.
pub(crate) fn on_options_leave(
    leave: On<PointerLeave>,
    kinds: Query<&OptionsPart>,
    parents: Query<&ChildOf>,
    mut screens: Query<&mut OptionsScreen>,
) {
    let Ok(OptionsPart::Row(key)) = kinds.get(leave.entity) else {
        return;
    };
    let Some(root) = screen_of(leave.entity, &parents, |e| screens.contains(e)) else {
        return;
    };
    if let Ok(mut screen) = screens.get_mut(root)
        && screen.hovered() == Some(key.as_str())
    {
        screen.set_hovered(None);
    }
}

/// A slider dragged or its track clicked: snap to the row's step grid, show it and send it
/// (a disabled row ignores it; `InteractionDisabled` already stops the widget's own input).
pub(crate) fn on_options_slider(
    change: On<ValueChange<f32>>,
    sliders: Query<&OptionsSlider>,
    parents: Query<&ChildOf>,
    mut screens: Query<&mut OptionsScreen>,
    mut commands: Commands,
    mut out: MessageWriter<OptionsEvent>,
) {
    let Ok(OptionsSlider(key)) = sliders.get(change.source) else {
        return;
    };
    let Some(entity) = screen_of(change.source, &parents, |e| screens.contains(e)) else {
        return;
    };
    let Ok(mut screen) = screens.get_mut(entity) else {
        return;
    };
    let Some(row) = screen.rows().iter().find(|r| r.key == *key && r.enabled) else {
        return;
    };
    let Some(value) = row.snapped(f64::from(change.value)) else {
        return;
    };
    focus_row(&mut screen, key);
    let set = screen.set(key, value);
    // At once, not at the next sync: a drag starts from the widget's value.
    if let Some((_, _, value)) = screen.model.row(key).and_then(OptionRow::slider) {
        commands.entity(change.source).insert(value);
    }
    if let Some(change) = set {
        out.write(OptionsEvent {
            screen: entity,
            change,
        });
    }
}

/// A checkbox clicked: flip the row and send it (the sync checks the box).
pub(crate) fn on_options_checkbox(
    change: On<ValueChange<bool>>,
    checkboxes: Query<&OptionsCheckbox>,
    parents: Query<&ChildOf>,
    mut screens: Query<&mut OptionsScreen>,
    mut out: MessageWriter<OptionsEvent>,
) {
    let Ok(OptionsCheckbox(key)) = checkboxes.get(change.source) else {
        return;
    };
    let Some(entity) = screen_of(change.source, &parents, |e| screens.contains(e)) else {
        return;
    };
    let Ok(mut screen) = screens.get_mut(entity) else {
        return;
    };
    focus_row(&mut screen, key);
    if let Some(change) = screen.set(key, SettingValue::Bool(change.value)) {
        out.write(OptionsEvent {
            screen: entity,
            change,
        });
    }
}
