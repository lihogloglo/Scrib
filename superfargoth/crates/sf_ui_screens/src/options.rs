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
//! Footer action bar (menus.md §5): Key bindings, Reset page, then Back at the right edge, under
//! the rows at a fixed height. Reset page asks first (the screen's own yes/no question over the
//! window, No focused; No or Back answers no), then puts the current page's enabled rows whose
//! record has `reset` (not language, bindings) back to their defaults and sends their keys as
//! [`OptionsChange::Reset`] (the app resets each in the store). It is disabled (greyed, skipped
//! by navigation) while no such row differs from its default.
//!
//! Input: up and down move between the page's enabled rows, then "Key bindings", Reset page
//! (while enabled) and Back (no wrap); left and right change the focused row (a toggle flips, a choice cycles, a slider
//! steps, clamped to its range; a preset driver's `custom` option, [`sf_ui::CUSTOM_LEVEL`], is
//! derived, so it is shown but stepped over); confirm or a click flips a toggle and cycles a
//! choice forward; the arrow buttons step; tab prev / next (Q / E, the triggers) switch pages, wrapping. A
//! disabled row (its record's `enabled_if` fails, or the app sets `enabled = false`) is shown
//! greyed and skipped.
//!
//! Record fields (menus.md §4 "Settings v2"): a row whose `visible_if` fails is left out (a page
//! left empty goes with it); one whose `enabled_if` fails is disabled, its `disabled_reason` in
//! the pane. Conditions read the rows' current values (re-applied the frame the model changes:
//! [`OptionsModel::refresh`], run by the screen's sync), other settings as the store held them
//! when the model was built, and the phase and platform traits of
//! [`OptionsModel::set_condition_context`]; nothing reports those yet, so until the app sets
//! them a `phase` or `trait` condition holds. Rows sort within their page by group (in order of
//! first appearance), then `order` (none counts as 0), then load order. `format`: `percent`
//! (0..1 as 0..100 %), `ms` (" ms"), `x` ("1.5x"); `raw` (the default) keeps the plain number,
//! a 0..1 float slider still reading as a percentage.
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
use bevy_text::{TextColor, TextLayout};
use bevy_ui::prelude::*;
use bevy_ui::Checked;
use bevy_ui_widgets::{
    Activate, Checkbox, ScrollArea, Slider, SliderOrientation, SliderRange, SliderStep,
    SliderThumb, SliderValue, TrackClick, ValueChange,
};
use sf_ui::records::{SettingApply, SettingDef, SettingKind, ValueFormat};
use sf_ui::{Condition, Localizer, SettingValue, SettingsStore, UiRegistry};
use sf_ui_bevy::{
    ScreenStackRes, UiAction, UiActionEvent, UiHandlesBack, UiNavEvent, UiOwnsFocus, UiScreenRoot,
};

use crate::keyed::KeyedChildren;
use crate::labels::ScreenLabels;
use crate::widgets::{
    Question, ScreenColors, StatBar, WindowFrame, button, classes, framed_button, framed_window,
    label, part, part_as, relabel, role, set_disabled, spawn_bar, spawn_button,
};
use crate::{is_target, on_target, screen_of};

/// Steps of a float slider whose record gives no step.
pub const DEFAULT_FLOAT_STEPS: f64 = 20.0;

/// The slider thumb's width (px); it travels the track less its width, so it never overhangs.
const THUMB_WIDTH: f32 = 10.0;

/// The description pane's width (px); its height is the body's (the window's frame).
const PANE_WIDTH: f32 = 280.0;

/// The options window's frame (menus.md §7 "Fixed window frame"): 60% x 70% of the viewport
/// (1152 x 756 at 1080p, a little larger than the content-sized window it replaces), clamped.
pub const OPTIONS_FRAME: WindowFrame =
    WindowFrame::share(60.0, 70.0, [640.0, 420.0], [1280.0, 900.0]);

/// The value column's width (px): the longest choice label at 1080p fits; longer ones clip, on
/// one line, so a value change never resizes its row.
const VALUE_WIDTH: f32 = 160.0; // tunables: const (UI layout width, px)

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
    /// How the value reads (the record's `format`).
    pub format: ValueFormat,
    /// The record's `reset`: Reset page puts it back to its default (`false`: language,
    /// bindings).
    pub reset: bool,
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
                // A preset driver's `custom` (sf_ui::CUSTOM_LEVEL) is derived, never picked:
                // shown, stepped over.
                let pickable = |i: usize| options[i].0 != sf_ui::CUSTOM_LEVEL;
                if !(0..options.len()).any(pickable) {
                    return None;
                }
                let len = options.len();
                // An unknown value goes to the first option (either way), as before.
                let (mut at, direction) = current.map_or((len - 1, 1), |at| (at, direction));
                loop {
                    at = if direction < 0 {
                        (at + len - 1) % len
                    } else {
                        (at + 1) % len
                    };
                    if pickable(at) {
                        break;
                    }
                }
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
                let unit_range = min.abs() < f64::EPSILON && (max - 1.0).abs() < f64::EPSILON;
                match self.format {
                    ValueFormat::Percent => format!("{}%", (v * 100.0).round()),
                    ValueFormat::Raw if unit_range => format!("{}%", (v * 100.0).round()),
                    format => {
                        // As many decimals as the step has (0.25: two), at most three.
                        let decimals = [1.0, 10.0, 100.0]
                            .iter()
                            .position(|scale| {
                                let scaled = step * scale;
                                (scaled - scaled.round()).abs() < 1e-6
                            })
                            .unwrap_or(3);
                        suffixed(format!("{v:.decimals$}"), format)
                    }
                }
            }
            (_, SettingValue::Int(v)) => suffixed(v.to_string(), self.format),
            (_, SettingValue::Float(v)) => suffixed(v.to_string(), self.format),
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

/// A number with its `format`'s unit: `50%`, `16 ms`, `1.5x`; raw as is.
fn suffixed(number: String, format: ValueFormat) -> String {
    match format {
        ValueFormat::Raw => number,
        ValueFormat::Percent => format!("{number}%"),
        ValueFormat::Ms => format!("{number} ms"),
        ValueFormat::Times => format!("{number}x"),
    }
}

/// What a row's record says about whether and where it shows (menus.md §4).
#[derive(Clone, Debug, PartialEq)]
struct RowRule {
    /// The page the row belongs on.
    page: String,
    /// Group rank within the page (first appearance), `order` (none: 0), load index.
    rank: (usize, i32, usize),
    enabled_if: Vec<Condition>,
    visible_if: Vec<Condition>,
}

impl RowRule {
    /// The rule of the `load`th record, on `page` whose groups so far are `groups`.
    fn new(page: &str, groups: &mut Vec<Option<String>>, def: &SettingDef, load: usize) -> Self {
        let group = groups.iter().position(|g| *g == def.group);
        let group = group.unwrap_or_else(|| {
            groups.push(def.group.clone());
            groups.len() - 1
        });
        Self {
            page: page.to_owned(),
            rank: (group, def.order.unwrap_or(0), load),
            enabled_if: def.enabled_if.clone(),
            visible_if: def.visible_if.clone(),
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

/// The screen's view-model: pages in order, each with its shown rows. Rows built from records
/// keep their record's conditions; [`OptionsModel::refresh`] re-applies them.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct OptionsModel {
    pub pages: Vec<OptionsPage>,
    /// Record rows by key: their page, sort rank and conditions.
    rules: HashMap<String, RowRule>,
    /// Rows whose `visible_if` fails, in load order.
    hidden: Vec<OptionRow>,
    /// Every record page, `(id, label)`, in first-definition order.
    page_order: Vec<(String, String)>,
    /// Store values of settings a condition reads that have no row (as at build).
    outside: Vec<(String, SettingValue)>,
    /// The menu phase conditions see; `None`: unknown, a `phase` condition holds.
    phase: Option<String>,
    /// The platform traits present; `None`: unknown, a `trait` condition holds.
    traits: Option<Vec<String>>,
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
        let mut rules: HashMap<String, RowRule> = HashMap::new();
        // Per page, its groups in order of first appearance.
        let mut groups: HashMap<String, Vec<Option<String>>> = HashMap::new();
        for (load, def) in registry.settings().enumerate() {
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
                format: def.def.format,
                reset: def.def.reset,
            };
            let page_id = def
                .def
                .page
                .clone()
                .unwrap_or_else(|| def.owner.to_string());
            let page_groups = groups.entry(page_id.clone()).or_default();
            rules.insert(key.to_owned(), RowRule::new(&page_id, page_groups, &def.def, load));
            match pages.iter_mut().find(|p| p.id == page_id) {
                Some(page) => page.rows.push(row),
                None => pages.push(OptionsPage {
                    label: text(&format!("set-page-{page_id}"), &page_id),
                    id: page_id,
                    rows: vec![row],
                }),
            }
        }
        Self::with_rules(pages, rules, store)
    }

    /// The built pages with their records' rules: rows sorted, conditions applied.
    fn with_rules(
        mut pages: Vec<OptionsPage>,
        rules: HashMap<String, RowRule>,
        store: Option<&SettingsStore>,
    ) -> Self {
        for page in &mut pages {
            page.rows.sort_by_key(|r| rules.get(&r.key).map(|rule| rule.rank));
        }
        // Settings a condition reads that have no row (text settings, actions): their stored
        // value, as at build.
        let mut outside: Vec<(String, SettingValue)> = Vec::new();
        for rule in rules.values() {
            for cond in rule.enabled_if.iter().chain(&rule.visible_if) {
                if let Condition::Setting { setting, .. } = cond
                    && !rules.contains_key(setting)
                    && !outside.iter().any(|(k, _)| k == setting)
                    && let Some(value) = store.and_then(|s| s.get(setting))
                {
                    outside.push((setting.clone(), value.clone()));
                }
            }
        }
        let mut model = Self {
            page_order: pages.iter().map(|p| (p.id.clone(), p.label.clone())).collect(),
            pages,
            rules,
            outside,
            ..Self::default()
        };
        model.refresh();
        model
    }

    /// Sets the menu phase ([`sf_ui::PHASE_FRONTEND`], [`sf_ui::PHASE_IN_GAME`]) and the
    /// platform traits (`dlss`, `rt`, ...) the conditions read, and re-applies them. `None`
    /// leaves that input unknown: its conditions hold.
    pub fn set_condition_context(&mut self, phase: Option<&str>, traits: Option<&[&str]>) {
        self.phase = phase.map(str::to_owned);
        self.traits = traits.map(|t| t.iter().map(|s| (*s).to_owned()).collect());
        self.refresh();
    }

    /// Re-applies the record rows' `visible_if` (a failing row leaves its page, a passing one
    /// comes back in its sorted place) and `enabled_if` (rows without one keep their
    /// `enabled`) against the rows' current values; whether anything changed. Cheap, and a
    /// no-op once applied: the screen runs it whenever its model changed.
    pub fn refresh(&mut self) -> bool {
        if self
            .rules
            .values()
            .all(|r| r.enabled_if.is_empty() && r.visible_if.is_empty())
        {
            return false;
        }
        let verdicts = self.verdicts();
        let shown = |key: &str| verdicts.get(key).is_none_or(|(visible, _)| *visible);
        let mut changed = false;
        let apply = |row: &mut OptionRow, changed: &mut bool| {
            if let Some((_, Some(enabled))) = verdicts.get(&row.key)
                && row.enabled != *enabled
            {
                row.enabled = *enabled;
                *changed = true;
            }
        };
        for page in &mut self.pages {
            let mut kept = Vec::with_capacity(page.rows.len());
            for mut row in page.rows.drain(..) {
                if shown(&row.key) {
                    apply(&mut row, &mut changed);
                    kept.push(row);
                } else {
                    self.hidden.push(row);
                    changed = true;
                }
            }
            page.rows = kept;
        }
        let mut back = Vec::new();
        for mut row in std::mem::take(&mut self.hidden) {
            apply(&mut row, &mut changed);
            if shown(&row.key) {
                back.push(row);
            } else {
                self.hidden.push(row);
            }
        }
        let rules = &self.rules;
        self.hidden
            .sort_by_key(|r| rules.get(&r.key).map(|rule| rule.rank));
        for row in back {
            changed = true;
            self.reveal(row);
        }
        // A record page left with no row goes (it comes back with its first row).
        let before = self.pages.len();
        let order = &self.page_order;
        self.pages
            .retain(|p| !p.rows.is_empty() || !order.iter().any(|(id, _)| *id == p.id));
        changed || self.pages.len() != before
    }

    /// Per record row with conditions: (visible, enabled when it has an `enabled_if`). Values
    /// are the rows' (the screen's own changes land there before the store), then the settings
    /// with no row; an unknown phase or trait holds.
    fn verdicts(&self) -> HashMap<String, (bool, Option<bool>)> {
        let mut values: HashMap<&str, &SettingValue> =
            self.outside.iter().map(|(k, v)| (k.as_str(), v)).collect();
        for row in self.pages.iter().flat_map(|p| &p.rows).chain(&self.hidden) {
            values.insert(&row.key, &row.value);
        }
        let value = |key: &str| values.get(key).map(|v| (*v).clone());
        let traits = |t: &str| {
            self.traits
                .as_ref()
                .is_none_or(|have| have.iter().any(|h| h == t))
        };
        let phase = self.phase.as_deref();
        let holds = |conds: &[Condition]| {
            conds.iter().all(|c| match c {
                Condition::Phase { .. } if phase.is_none() => true,
                c => c.holds(&value, &traits, phase.unwrap_or_default()),
            })
        };
        self.rules
            .iter()
            .filter(|(_, r)| !(r.enabled_if.is_empty() && r.visible_if.is_empty()))
            .map(|(key, rule)| {
                let enabled = (!rule.enabled_if.is_empty()).then(|| holds(&rule.enabled_if));
                (key.clone(), (holds(&rule.visible_if), enabled))
            })
            .collect()
    }

    /// Puts a hidden row back on its page in its sorted place (the page too, if it went).
    fn reveal(&mut self, row: OptionRow) {
        let Some(rule) = self.rules.get(&row.key) else {
            return;
        };
        let page_rank = |id: &str| {
            self.page_order
                .iter()
                .position(|(p, _)| p == id)
                .unwrap_or(usize::MAX)
        };
        let at = if let Some(at) = self.pages.iter().position(|p| p.id == rule.page) {
            at
        } else {
            let want = page_rank(&rule.page);
            let at = self
                .pages
                .iter()
                .position(|p| page_rank(&p.id) > want)
                .unwrap_or(self.pages.len());
            let label = self
                .page_order
                .iter()
                .find(|(p, _)| *p == rule.page)
                .map_or_else(|| rule.page.clone(), |(_, l)| l.clone());
            self.pages.insert(
                at,
                OptionsPage {
                    id: rule.page.clone(),
                    label,
                    rows: Vec::new(),
                },
            );
            at
        };
        let rank = rule.rank;
        let rules = &self.rules;
        let rows = &mut self.pages[at].rows;
        let slot = rows
            .iter()
            .position(|r| rules.get(&r.key).is_some_and(|other| other.rank > rank))
            .unwrap_or(rows.len());
        rows.insert(slot, row);
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
    /// Reset page: these settings of the current page go back to their defaults (the screen
    /// already shows them); the app resets each in the store.
    Reset(Vec<String>),
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
    /// The footer's Reset page button.
    ResetPage,
    Back,
    /// An answer of the Reset page question: yes, no.
    Confirm(usize),
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
    /// The Reset page question is up.
    asking: bool,
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

    /// The Reset page question is up.
    pub fn asking_reset(&self) -> bool {
        self.asking
    }

    /// The rows Reset page would put back: the current page's enabled rows with `reset` whose
    /// value is not their default.
    fn resettable(&self) -> impl Iterator<Item = &OptionRow> {
        self.rows()
            .iter()
            .filter(|r| r.reset && r.enabled && r.value != r.default)
    }

    /// Reset page has something to do (its button is enabled).
    pub fn can_reset(&self) -> bool {
        self.resettable().next().is_some()
    }

    /// Reset page asks first, No focused (a stray confirm never resets).
    fn ask_reset(&mut self) {
        if self.can_reset() {
            self.asking = true;
            self.focus = OptionsFocus::Confirm(1);
        }
    }

    /// Closes the question: focus back on Reset page, or on Back once there is nothing left.
    fn answer(&mut self) {
        self.asking = false;
        self.focus = if self.can_reset() {
            OptionsFocus::ResetPage
        } else {
            OptionsFocus::Back
        };
    }

    /// Puts the resettable rows back to their defaults; the change, when any row changed.
    fn reset_page(&mut self) -> Option<OptionsChange> {
        let keys: Vec<String> = self.resettable().map(|r| r.key.clone()).collect();
        if keys.is_empty() {
            return None;
        }
        let page = self.model.pages.get_mut(self.page)?;
        for row in page.rows.iter_mut().filter(|r| keys.contains(&r.key)) {
            row.value = row.default.clone();
        }
        Some(OptionsChange::Reset(keys))
    }

    /// The focus stops in order: enabled rows, then Key bindings, Reset page (while it has
    /// something to reset) and Back; while the question is up, its answers.
    fn stops(&self) -> Vec<OptionsFocus> {
        if self.asking {
            return vec![OptionsFocus::Confirm(0), OptionsFocus::Confirm(1)];
        }
        let reset = self.can_reset().then_some(OptionsFocus::ResetPage);
        self.rows()
            .iter()
            .filter(|r| r.enabled)
            .map(|r| OptionsFocus::Row(r.key.clone()))
            .chain([OptionsFocus::Keybinds])
            .chain(reset)
            .chain([OptionsFocus::Back])
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
    ResetPage,
    Back,
    /// The Reset page question's answers.
    Yes,
    No,
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

/// The description pane's text parts (right of the rows). An empty one draws nothing and keeps
/// its place (never `Display::None`, so the lines never shift).
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
    reset: Entity,
    back: Entity,
    /// Reset page's enabled state on screen.
    reset_shown: Option<bool>,
    /// The Reset page question.
    question: Question,
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

    /// The footer's Reset page button.
    pub fn reset_page(&self) -> Entity {
        self.reset
    }

    pub fn back(&self) -> Entity {
        self.back
    }

    /// The Reset page question's overlay.
    pub fn overlay(&self) -> Entity {
        self.question.overlay
    }

    /// The question's Yes, No.
    pub fn confirm_buttons(&self) -> [Entity; 2] {
        self.question.buttons
    }

    fn target(&self, focus: &OptionsFocus) -> Option<Entity> {
        match focus {
            OptionsFocus::Start => None,
            OptionsFocus::Row(key) => self.rows.get(key),
            OptionsFocus::Keybinds => Some(self.keybinds),
            OptionsFocus::ResetPage => Some(self.reset),
            OptionsFocus::Back => Some(self.back),
            OptionsFocus::Confirm(i) => self.question.buttons.get(*i).copied(),
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
            framed_window(
                OPTIONS_FRAME,
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Stretch,
                    padding: UiRect::all(Val::Px(16.0)),
                    row_gap: Val::Px(8.0),
                    ..Node::default()
                },
            ),
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
        // It takes the window's height left by the title, tabs and footer; the rows scroll in it.
        let body = row_of(&mut commands, "sf-options-body", 16.0);
        commands.entity(body).insert(Node {
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(16.0),
            flex_grow: 1.0,
            flex_basis: Val::Px(0.0),
            min_height: Val::Px(0.0),
            ..Node::default()
        });
        let rows = commands
            .spawn((
                part_as("sf-options-rows", role::BOX),
                Node {
                    flex_direction: FlexDirection::Column,
                    flex_grow: 1.0,
                    min_width: Val::Px(0.0),
                    min_height: Val::Px(0.0),
                    row_gap: Val::Px(4.0),
                    overflow: Overflow::scroll_y(),
                    ..Node::default()
                },
                // Wheel scrolling; `scroll_to_focus` keeps the focused row in view.
                ScrollArea,
                ChildOf(body),
            ))
            .id();
        let pane = spawn_pane(&mut commands, body, &colors);
        // The footer action bar (menus.md §5): Key bindings and Reset page, Back at the right
        // edge. Content-sized buttons that never change size, never shrunk by the body.
        let footer = row_of(&mut commands, "sf-options-footer", 8.0);
        let ([keybinds, reset, back], question) =
            spawn_footer(&mut commands, root, footer, &labels, &colors);
        commands.entity(root).insert(OptionsParts {
            tabs: KeyedChildren::new(tabs),
            rows: KeyedChildren::new(rows),
            row_parts: HashMap::new(),
            shown: HashMap::new(),
            active_tab: None,
            keybinds,
            reset,
            back,
            reset_shown: None,
            question,
            pane,
            pane_shown: None,
            seen: None,
        });
    }
}

/// The footer's buttons under `footer` (Key bindings, Reset page, Back at the right edge) and
/// the Reset page question over the window `root`.
fn spawn_footer(
    commands: &mut Commands,
    root: Entity,
    footer: Entity,
    labels: &ScreenLabels,
    colors: &ScreenColors,
) -> ([Entity; 3], Question) {
    commands.entity(footer).insert(Node {
        flex_direction: FlexDirection::Row,
        column_gap: Val::Px(8.0),
        flex_shrink: 0.0,
        ..Node::default()
    });
    let keybinds = commands
        .spawn((
            framed_button("sf-options-keybinds", labels.keybinds.clone(), colors),
            OptionsPart::Keybinds,
            ChildOf(footer),
        ))
        .id();
    let reset = commands
        .spawn((
            framed_button("sf-options-reset", labels.reset_page.clone(), colors),
            OptionsPart::ResetPage,
            ChildOf(footer),
        ))
        .id();
    let back = commands
        .spawn((
            framed_button("sf-options-back", labels.back.clone(), colors),
            OptionsPart::Back,
            ChildOf(footer),
        ))
        .id();
    commands.queue(move |world: &mut World| {
        if let Some(mut node) = world.get_mut::<Node>(back) {
            node.margin.left = Val::Auto;
        }
    });
    // Over the window (absolute, so it never moves the rows), hidden until asked.
    let question = Question::spawn(
        commands,
        root,
        [
            "sf-options-confirm",
            "sf-options-confirm-text",
            "sf-options-confirm-choices",
        ],
        [
            ("sf-options-yes", labels.yes.clone(), OptionsPart::Yes),
            ("sf-options-no", labels.no.clone(), OptionsPart::No),
        ],
        colors,
    );
    ([keybinds, reset, back], question)
}

/// The description pane under `body`: label, description, disabled reason, restart badge,
/// default value, each empty until it has text. Fixed width, the body's height, its overflow
/// clipped: what it shows never resizes it.
fn spawn_pane(commands: &mut Commands, body: Entity, colors: &ScreenColors) -> OptionsPane {
    let root = commands
        .spawn((
            part_as("sf-options-pane", role::BOX),
            Node {
                flex_direction: FlexDirection::Column,
                width: Val::Px(PANE_WIDTH),
                flex_shrink: 0.0,
                min_height: Val::Px(0.0),
                row_gap: Val::Px(6.0),
                padding: UiRect::all(Val::Px(8.0)),
                overflow: Overflow::clip(),
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
            TextLayout::no_wrap(),
            Node {
                width: Val::Px(VALUE_WIDTH),
                flex_shrink: 0.0,
                overflow: Overflow::clip(),
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
    // Slider thumbs and the question's overlay.
    Query<'w, 's, &'static mut Node>,
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
    (mut bars, sliders, mut nodes, checks): RowWidgets,
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
            // A changed value may flip a row's conditions: re-apply them, keeping the page shown
            // (by id: a page before it may come or go). Change detection only when they did.
            let shown = screen.model.pages.get(screen.page).map(|p| p.id.clone());
            if screen.bypass_change_detection().model.refresh() {
                if let Some(at) =
                    shown.and_then(|id| screen.model.pages.iter().position(|p| p.id == id))
                {
                    screen.bypass_change_detection().page = at;
                }
                screen.set_changed();
            }
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
                    if let Ok(mut node) = nodes.get_mut(thumb)
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

        // The question goes when there is nothing left to reset (the app's new model).
        if screen.asking && !screen.can_reset() {
            screen.answer();
        }
        // Focus: a stop of the current page, else its home.
        if !screen.stops().contains(&screen.focus) {
            screen.focus = screen.home();
        }
        let can_reset = screen.can_reset();
        if parts.reset_shown != Some(can_reset) {
            set_disabled(&mut commands, parts.reset, !can_reset);
            commands
                .entity(parts.reset)
                .insert(classes("sf-options-reset", &[("disabled", !can_reset)]));
            let color = if can_reset { colors.text } else { colors.muted };
            relabel(&mut commands, parts.reset, labels.reset_page.clone(), color);
            parts.reset_shown = Some(can_reset);
        }
        let question = if screen.asking {
            labels.confirm_reset_page.clone()
        } else {
            String::new()
        };
        parts
            .question
            .set(&mut commands, &mut nodes, question, colors);
        // The pane: the hovered row, else the focused one.
        let pane = pane_text(screen.described(), labels);
        if parts.pane_shown.as_ref() != Some(&pane) {
            // Spawned empty and hidden, so "nothing shown yet" compares as all empty.
            let was = parts.pane_shown.take().unwrap_or_default();
            for ((entity, text), old) in parts.pane.texts().into_iter().zip(&pane).zip(&was) {
                if text == old {
                    continue;
                }
                // Never `Display::None`: an empty line draws nothing and keeps its place.
                commands.entity(entity).insert(Text::new(text.clone()));
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
            // The question is modal: left and right pick an answer.
            if screen.asking {
                let answer = match nav.direction {
                    CompassOctant::West => OptionsFocus::Confirm(0),
                    CompassOctant::East => OptionsFocus::Confirm(1),
                    _ => continue,
                };
                if screen.focus != answer {
                    screen.focus = answer;
                }
                continue;
            }
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
            // Back answers no; nothing else reaches the screen under the question.
            if screen.asking {
                if action.action == UiAction::Back {
                    screen.answer();
                }
                continue;
            }
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
    roots: Query<&UiScreenRoot>,
    mut screens: Query<(&UiScreenRoot, &mut OptionsScreen)>,
    mut stack: ResMut<ScreenStackRes>,
    mut out: MessageWriter<OptionsEvent>,
) {
    let Ok(kind) = kinds.get(activate.entity) else {
        return;
    };
    if !on_target(activate.entity, &parents, &roots, &stack) {
        return;
    }
    let Some(entity) = screen_of(activate.entity, &parents, |e| screens.contains(e)) else {
        return;
    };
    let Ok((root, mut screen)) = screens.get_mut(entity) else {
        return;
    };
    let change = match kind {
        OptionsPart::Yes if screen.asking => {
            let change = screen.reset_page();
            screen.answer();
            change
        }
        OptionsPart::No if screen.asking => {
            screen.answer();
            None
        }
        // The question is modal.
        _ if screen.asking => None,
        OptionsPart::Yes | OptionsPart::No => None,
        OptionsPart::ResetPage => {
            screen.ask_reset();
            None
        }
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
    roots: Query<&UiScreenRoot>,
    stack: Res<ScreenStackRes>,
    mut screens: Query<&mut OptionsScreen>,
) {
    let Ok(OptionsPart::Row(key)) = kinds.get(enter.entity) else {
        return;
    };
    if !on_target(enter.entity, &parents, &roots, &stack) {
        return;
    }
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
#[allow(clippy::too_many_arguments)]
pub(crate) fn on_options_slider(
    change: On<ValueChange<f32>>,
    sliders: Query<&OptionsSlider>,
    parents: Query<&ChildOf>,
    roots: Query<&UiScreenRoot>,
    stack: Res<ScreenStackRes>,
    mut screens: Query<&mut OptionsScreen>,
    mut commands: Commands,
    mut out: MessageWriter<OptionsEvent>,
) {
    let Ok(OptionsSlider(key)) = sliders.get(change.source) else {
        return;
    };
    if !on_target(change.source, &parents, &roots, &stack) {
        return;
    }
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
    roots: Query<&UiScreenRoot>,
    stack: Res<ScreenStackRes>,
    mut screens: Query<&mut OptionsScreen>,
    mut out: MessageWriter<OptionsEvent>,
) {
    let Ok(OptionsCheckbox(key)) = checkboxes.get(change.source) else {
        return;
    };
    if !on_target(change.source, &parents, &roots, &stack) {
        return;
    }
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
