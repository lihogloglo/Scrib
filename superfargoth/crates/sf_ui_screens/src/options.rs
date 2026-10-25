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
//! Strings: a row's label is its record's Fluent `label`, a choice option's is
//! `<label>-<option>`, a page's `set-page-<page>`; a missing string reads as the id made
//! readable (`move_speed` reads "Move speed").

use std::collections::HashMap;

use bevy_ecs::prelude::*;
use bevy_input_focus::{FocusCause, InputFocus};
use bevy_math::CompassOctant;
use bevy_text::TextColor;
use bevy_ui::prelude::*;
use bevy_ui_widgets::Activate;
use sf_ui::records::SettingKind;
use sf_ui::{Localizer, SettingValue, SettingsStore, UiRegistry};
use sf_ui_bevy::{
    ScreenStackRes, UiAction, UiActionEvent, UiHandlesBack, UiNavEvent, UiOwnsFocus, UiScreenRoot,
};

use crate::keyed::KeyedChildren;
use crate::labels::ScreenLabels;
use crate::widgets::{
    ScreenColors, StatBar, button, centred_window, classes, label, part, relabel, set_disabled,
    spawn_bar, spawn_button,
};
use crate::{is_target, screen_of};

/// Steps of a float slider whose record gives no step.
pub const DEFAULT_FLOAT_STEPS: f64 = 20.0;

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

    /// The value as shown: on / off, the option's label, the number (a 0..1 slider as a
    /// percentage).
    pub fn value_text(&self, labels: &ScreenLabels) -> String {
        match (&self.kind, &self.value) {
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
        let text = |key: &str, fallback: &str| match localizer {
            Some(l) if !key.is_empty() && l.has(key) => l.format(key, None).into_owned(),
            _ => readable(fallback),
        };
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
                SettingKind::Choice { default, options } => (
                    OptionKind::Choice(
                        options
                            .iter()
                            .map(|o| (o.clone(), text(&format!("{}-{o}", def.def.label), o)))
                            .collect(),
                    ),
                    SettingValue::Text(default.clone()),
                ),
                SettingKind::Text { .. } => continue,
            };
            let local = key.strip_prefix(&format!("{}.", def.owner)).unwrap_or(key);
            let row = OptionRow {
                key: key.to_owned(),
                label: text(&def.def.label, local),
                kind,
                value: store.and_then(|s| s.get(key)).cloned().unwrap_or(default),
                enabled: true,
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
}

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
    /// Left, right.
    arrows: [Entity; 2],
    bar: Option<Entity>,
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
    seen: Option<(usize, OptionsModel)>,
}

impl OptionsParts {
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

    /// A row's left and right arrows.
    pub fn arrows(&self, key: &str) -> Option<[Entity; 2]> {
        self.row_parts.get(key).map(|p| p.arrows)
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
            part("sf-options"),
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
                        flex_direction: if name == "sf-options-rows" {
                            FlexDirection::Column
                        } else {
                            FlexDirection::Row
                        },
                        column_gap: Val::Px(gap),
                        row_gap: Val::Px(4.0),
                        ..Node::default()
                    },
                    ChildOf(root),
                ))
                .id()
        };
        let tabs = row_of(&mut commands, "sf-options-tabs", 4.0);
        let rows = row_of(&mut commands, "sf-options-rows", 8.0);
        let footer = row_of(&mut commands, "sf-options-footer", 8.0);
        let keybinds = commands
            .spawn((
                button("sf-options-keybinds", labels.keybinds.clone(), &colors),
                OptionsPart::Keybinds,
                ChildOf(footer),
            ))
            .id();
        let back = commands
            .spawn((
                button("sf-options-back", labels.back.clone(), &colors),
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
            seen: None,
        });
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
    let bar = row.fraction().map(|(current, max)| {
        spawn_bar(
            commands,
            controls,
            "sf-options-bar",
            StatBar {
                current,
                max,
                kind: "setting".into(),
            },
            colors,
        )
    });
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
    (
        row_node,
        RowParts {
            value,
            arrows: [less, more],
            bar,
        },
    )
}

/// Model and page → tabs, rows (keyed by setting key), values, and focus. Writes only what
/// changed.
#[allow(clippy::too_many_lines)]
pub(crate) fn sync_options(
    mut commands: Commands,
    stack: Res<ScreenStackRes>,
    colors: Res<ScreenColors>,
    labels: Res<ScreenLabels>,
    mut screens: Query<(&UiScreenRoot, &mut OptionsScreen, &mut OptionsParts)>,
    mut bars: Query<&mut StatBar>,
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
                    if let (Some(bar), Some((current, max))) = (made.bar, row.fraction())
                        && let Ok(mut b) = bars.get_mut(bar)
                    {
                        b.current = current;
                        b.max = max;
                    }
                }
                if was.as_ref().map(|(_, e)| *e) != Some(row.enabled) {
                    set_disabled(&mut commands, entity, !row.enabled);
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
