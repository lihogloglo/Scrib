//! The save/load screen (ui.md "Stock screens", save.md "Slots and files") over a view-model the
//! app fills: [`SaveLoadModel`], one [`SaveEntryView`] per save. The screen knows nothing about
//! save files: the app lists its save folder into the model (and lists it again after a save or
//! a delete), and maps the [`SaveLoadEvent`]s the screen sends to its save and load requests.
//!
//! A list of saves, newest first (keyed rows: thumbnail, title, subtitle, reason), with "New
//! save" on top in save mode; a footer with the primary action (load or save), delete and back.
//! Focus moves the selection; confirm (or a click on the selected row) runs the primary action.
//! Overwrite and delete ask first (the question's cancel button is focused); Context1 deletes
//! the selected save; Back cancels the question, else sends [`SaveLoadChoice::Back`] and closes
//! the screen. A new save takes the model's generated name (ui.md "Input": default names;
//! typed names wait for text entry in the router).

use std::collections::HashMap;

use bevy_asset::Handle;
use bevy_ecs::prelude::*;
use bevy_image::Image;
use bevy_input_focus::{FocusCause, InputFocus};
use bevy_math::CompassOctant;
use bevy_ui::ScrollPosition;
use bevy_ui::prelude::*;
use bevy_ui_widgets::{Activate, ScrollArea};
use sf_ui_bevy::{
    ScreenStackRes, UiAction, UiActionEvent, UiHandlesBack, UiNavEvent, UiOwnsFocus, UiScreenRoot,
};

use crate::keyed::KeyedChildren;
use crate::labels::ScreenLabels;
use crate::widgets::{
    Question, ScreenColors, button, centred_window, classes, label, label_in, part, relabel,
    set_disabled, show,
};
use crate::{is_target, screen_of};

/// What the screen is for.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SaveLoadMode {
    /// Pick a save to overwrite, or make a new one.
    Save,
    /// Pick a save to load.
    #[default]
    Load,
}

/// One save as the screen shows it; the app fills it from the save's file name and meta.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SaveEntryView {
    /// The app's id for the save (a file name, for example); it comes back in the choices.
    pub id: String,
    /// Sort key, newest highest (a save sequence or a timestamp).
    pub order: u64,
    pub title: String,
    /// Place, time played, date: already formatted and localized by the app.
    pub subtitle: String,
    /// "Quicksave", "Autosave", ...; empty for a manual save.
    pub reason_label: String,
    pub thumbnail: Option<Handle<Image>>,
    /// The save could not be read: listed (and deletable) but not loadable.
    pub damaged: bool,
}

impl SaveEntryView {
    pub fn new(id: impl Into<String>, order: u64, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            order,
            title: title.into(),
            ..Self::default()
        }
    }
}

/// The screen's view-model.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SaveLoadModel {
    pub mode: SaveLoadMode,
    /// Any order: the screen lists them newest first.
    pub entries: Vec<SaveEntryView>,
    pub can_delete: bool,
    /// Save mode: the name a new save gets.
    pub new_name: String,
}

impl SaveLoadModel {
    /// The rows as listed: "New save" first in save mode, then the saves newest first (ties by
    /// id).
    pub fn rows(&self) -> Vec<SaveLoadRow> {
        let mut entries: Vec<&SaveEntryView> = self.entries.iter().collect();
        entries.sort_by(|a, b| b.order.cmp(&a.order).then_with(|| a.id.cmp(&b.id)));
        let new = (self.mode == SaveLoadMode::Save).then_some(SaveLoadRow::New);
        new.into_iter()
            .chain(entries.into_iter().map(|e| SaveLoadRow::Entry(e.id.clone())))
            .collect()
    }

    pub fn entry(&self, id: &str) -> Option<&SaveEntryView> {
        self.entries.iter().find(|e| e.id == id)
    }
}

/// A row of the list.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SaveLoadRow {
    /// Save mode: make a new save.
    New,
    /// A save, by [`SaveEntryView::id`].
    Entry(String),
}

/// What the player chose.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SaveLoadChoice {
    Load(String),
    /// Overwrite this save (confirmed).
    Overwrite(String),
    /// A new save with this name.
    New(String),
    /// Delete this save (confirmed).
    Delete(String),
    /// The screen closed without a choice.
    Back,
}

/// A choice, for the app to carry out (and then list the folder again or close the screen).
#[derive(Message, Clone, Debug, PartialEq, Eq)]
pub struct SaveLoadEvent {
    pub screen: Entity,
    pub choice: SaveLoadChoice,
}

/// Where the screen's focus is.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SaveLoadFocus {
    #[default]
    Start,
    /// A row of the list (by index into [`SaveLoadModel::rows`]).
    Row(usize),
    /// A footer button: primary, delete, back.
    Button(usize),
    /// A button of the question: yes, cancel.
    Confirm(usize),
}

/// On a screen root: the save/load screen. The app may replace `model` at any time (rows are
/// keyed by id, so a new listing only touches what changed).
#[derive(Component, Clone, Debug, Default)]
#[require(Node, UiOwnsFocus, UiHandlesBack)]
pub struct SaveLoadScreen {
    pub model: SaveLoadModel,
    focus: SaveLoadFocus,
    selected: Option<SaveLoadRow>,
    /// An overwrite or delete waiting for its confirmation.
    pending: Option<SaveLoadChoice>,
}

impl SaveLoadScreen {
    pub fn new(model: SaveLoadModel) -> Self {
        Self {
            model,
            ..Self::default()
        }
    }

    pub fn focus(&self) -> SaveLoadFocus {
        self.focus
    }

    pub fn selected(&self) -> Option<&SaveLoadRow> {
        self.selected.as_ref()
    }

    /// The overwrite or delete the question asks about.
    pub fn pending(&self) -> Option<&SaveLoadChoice> {
        self.pending.as_ref()
    }

    /// The selected save's id.
    fn selected_entry(&self) -> Option<&str> {
        match &self.selected {
            Some(SaveLoadRow::Entry(id)) => Some(id),
            _ => None,
        }
    }

    /// The primary action on the selection: load, new save, or ask to overwrite.
    fn primary(&mut self) -> Option<SaveLoadChoice> {
        match (self.model.mode, self.selected.clone()?) {
            (SaveLoadMode::Load, SaveLoadRow::Entry(id)) => self
                .model
                .entry(&id)
                .is_some_and(|e| !e.damaged)
                .then_some(SaveLoadChoice::Load(id)),
            (SaveLoadMode::Save, SaveLoadRow::New) => {
                Some(SaveLoadChoice::New(self.model.new_name.clone()))
            }
            (SaveLoadMode::Save, SaveLoadRow::Entry(id)) => {
                self.ask(SaveLoadChoice::Overwrite(id));
                None
            }
            (SaveLoadMode::Load, SaveLoadRow::New) => None,
        }
    }

    /// Ask to delete the selected save.
    fn delete(&mut self) {
        if self.model.can_delete
            && let Some(id) = self.selected_entry()
        {
            let id = id.to_owned();
            self.ask(SaveLoadChoice::Delete(id));
        }
    }

    fn ask(&mut self, choice: SaveLoadChoice) {
        self.pending = Some(choice);
        // Cancel first: a stray confirm never destroys a save.
        self.focus = SaveLoadFocus::Confirm(1);
    }

    fn primary_enabled(&self) -> bool {
        match (self.model.mode, &self.selected) {
            (SaveLoadMode::Load, Some(SaveLoadRow::Entry(id))) => {
                self.model.entry(id).is_some_and(|e| !e.damaged)
            }
            (SaveLoadMode::Save, Some(_)) => true,
            _ => false,
        }
    }
}

/// A focusable part of the save/load screen.
#[derive(Component, Clone, Debug, PartialEq, Eq)]
pub enum SaveLoadPart {
    Row(SaveLoadRow),
    Primary,
    Delete,
    Back,
    Yes,
    Cancel,
}

/// Fallback thumbnail size (16:9).
const THUMB: (f32, f32) = (128.0, 72.0);

/// The screen's parts and what they show.
#[derive(Component, Debug)]
pub struct SaveLoadParts {
    rows: KeyedChildren<SaveLoadRow>,
    order: Vec<SaveLoadRow>,
    /// What each row shows: its entry (`None`: the new-save row) and whether it is selected.
    shown: HashMap<SaveLoadRow, (Option<SaveEntryView>, bool)>,
    seen: Option<SaveLoadModel>,
    title: Entity,
    empty: Entity,
    /// Primary, delete, back.
    buttons: [Entity; 3],
    disabled: [bool; 2],
    /// Overwrite or delete: yes, cancel.
    question: Question,
}

impl SaveLoadParts {
    /// The row entity of a row key.
    pub fn row(&self, key: &SaveLoadRow) -> Option<Entity> {
        self.rows.get(key)
    }

    /// The rows in list order.
    pub fn order(&self) -> &[SaveLoadRow] {
        &self.order
    }

    /// Primary (load or save), delete, back.
    pub fn buttons(&self) -> [Entity; 3] {
        self.buttons
    }

    pub fn empty_label(&self) -> Entity {
        self.empty
    }

    pub fn overlay(&self) -> Entity {
        self.question.overlay
    }

    /// Yes, cancel.
    pub fn confirm_buttons(&self) -> [Entity; 2] {
        self.question.buttons
    }

    fn target(&self, focus: SaveLoadFocus) -> Option<Entity> {
        match focus {
            SaveLoadFocus::Start => None,
            SaveLoadFocus::Row(i) => self.rows.get(self.order.get(i)?),
            SaveLoadFocus::Button(i) => self.buttons.get(i).copied(),
            SaveLoadFocus::Confirm(i) => self.question.buttons.get(i).copied(),
        }
    }
}

#[allow(clippy::too_many_lines)]
pub(crate) fn spawn_saveload(
    mut commands: Commands,
    colors: Res<ScreenColors>,
    labels: Res<ScreenLabels>,
    roots: Query<Entity, Added<SaveLoadScreen>>,
) {
    for root in &roots {
        commands.entity(root).insert((
            part("sf-saveload"),
            centred_window(Node {
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(16.0)),
                row_gap: Val::Px(8.0),
                width: Val::Px(720.0),
                max_width: Val::Percent(100.0),
                max_height: Val::Percent(100.0),
                ..Node::default()
            }),
            BackgroundColor(colors.panel),
        ));
        let title = commands
            .spawn((label("sf-saveload-title", "", &colors), ChildOf(root)))
            .id();
        let list = commands
            .spawn((
                part("sf-saveload-list"),
                Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(4.0),
                    max_height: Val::Vh(60.0),
                    overflow: Overflow::scroll_y(),
                    ..Node::default()
                },
                ScrollPosition::default(),
                ScrollArea,
                ChildOf(root),
            ))
            .id();
        let empty = commands
            .spawn((
                label_in("sf-saveload-empty", labels.no_saves.clone(), colors.muted, &colors),
                ChildOf(root),
            ))
            .id();
        let footer = commands
            .spawn((
                part("sf-saveload-footer"),
                Node {
                    column_gap: Val::Px(8.0),
                    justify_content: JustifyContent::FlexEnd,
                    ..Node::default()
                },
                ChildOf(root),
            ))
            .id();
        let buttons = [
            ("sf-saveload-primary", &labels.load, SaveLoadPart::Primary),
            ("sf-saveload-delete", &labels.delete, SaveLoadPart::Delete),
            ("sf-saveload-back", &labels.back, SaveLoadPart::Back),
        ]
        .map(|(name, text, kind)| {
            commands
                .spawn((button(name, text.clone(), &colors), kind, ChildOf(footer)))
                .id()
        });
        let question = Question::spawn(
            &mut commands,
            root,
            [
                "sf-saveload-confirm",
                "sf-saveload-confirm-text",
                "sf-saveload-confirm-choices",
            ],
            [
                ("sf-saveload-yes", labels.yes.clone(), SaveLoadPart::Yes),
                ("sf-saveload-cancel", labels.cancel.clone(), SaveLoadPart::Cancel),
            ],
            &colors,
        );
        commands.entity(root).insert(SaveLoadParts {
            rows: KeyedChildren::new(list),
            order: Vec::new(),
            shown: HashMap::new(),
            seen: None,
            title,
            empty,
            buttons,
            disabled: [false; 2],
            question,
        });
    }
}

/// Draws a row's content (the row entity stays).
fn draw_row(
    commands: &mut Commands,
    row: Entity,
    entry: Option<&SaveEntryView>,
    new_name: &str,
    labels: &ScreenLabels,
    colors: &ScreenColors,
) {
    commands.entity(row).despawn_related::<Children>();
    let Some(entry) = entry else {
        commands.spawn((
            label(
                "sf-saveload-new-label",
                format!("{} ({new_name})", labels.new_save),
                colors,
            ),
            ChildOf(row),
        ));
        return;
    };
    let thumb = commands
        .spawn((
            part("sf-saveload-thumb"),
            Node {
                width: Val::Px(THUMB.0),
                height: Val::Px(THUMB.1),
                flex_shrink: 0.0,
                ..Node::default()
            },
            BackgroundColor(colors.bar_track),
            ChildOf(row),
        ))
        .id();
    if let Some(image) = &entry.thumbnail {
        commands.entity(thumb).insert(ImageNode::new(image.clone()));
    }
    let text = commands
        .spawn((
            part("sf-saveload-text"),
            Node {
                flex_direction: FlexDirection::Column,
                flex_grow: 1.0,
                ..Node::default()
            },
            ChildOf(row),
        ))
        .id();
    commands.spawn((
        label("sf-saveload-entry-title", entry.title.clone(), colors),
        ChildOf(text),
    ));
    commands.spawn((
        label_in(
            "sf-saveload-subtitle",
            entry.subtitle.clone(),
            colors.muted,
            colors,
        ),
        ChildOf(text),
    ));
    let reason = if entry.damaged {
        labels.damaged.clone()
    } else {
        entry.reason_label.clone()
    };
    commands.spawn((
        label_in("sf-saveload-reason", reason, colors.muted, colors),
        ChildOf(row),
    ));
}

/// Model → rows (keyed, changed rows redrawn), selection, buttons, the question and focus.
/// Writes only what changed.
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub(crate) fn sync_saveload(
    mut commands: Commands,
    stack: Res<ScreenStackRes>,
    colors: Res<ScreenColors>,
    labels: Res<ScreenLabels>,
    mut screens: Query<(&UiScreenRoot, &mut SaveLoadScreen, &mut SaveLoadParts)>,
    mut nodes: Query<&mut Node>,
    mut focus: ResMut<InputFocus>,
) {
    for (root, mut screen, mut parts) in &mut screens {
        let parts = &mut *parts;
        let colors = &*colors;
        if parts.seen.as_ref() != Some(&screen.model) {
            let model = screen.model.clone();
            if parts.seen.as_ref().map(|m| m.mode) != Some(model.mode) {
                let (title, primary) = match model.mode {
                    SaveLoadMode::Save => (&labels.save_title, &labels.save),
                    SaveLoadMode::Load => (&labels.load_title, &labels.load),
                };
                commands
                    .entity(parts.title)
                    .insert(Text::new(title.clone()));
                relabel(&mut commands, parts.buttons[0], primary.clone(), colors.text);
            }
            let new_name_changed = parts.seen.as_ref().map(|m| &m.new_name) != Some(&model.new_name);
            parts.order = model.rows();
            parts
                .rows
                .sync(&mut commands, parts.order.iter().cloned(), |commands, key| {
                    commands
                        .spawn((
                            part("sf-saveload-row"),
                            Node {
                                column_gap: Val::Px(10.0),
                                align_items: AlignItems::Center,
                                padding: UiRect::all(Val::Px(4.0)),
                                ..Node::default()
                            },
                            BackgroundColor(colors.cell),
                            bevy_ui_widgets::Button,
                            sf_ui_bevy::UiFocusable,
                            SaveLoadPart::Row(key.clone()),
                        ))
                        .id()
                });
            parts.shown.retain(|k, _| parts.order.contains(k));
            for key in &parts.order {
                let Some(row) = parts.rows.get(key) else {
                    continue;
                };
                let entry = match key {
                    SaveLoadRow::New => None,
                    SaveLoadRow::Entry(id) => model.entry(id).cloned(),
                };
                let fresh = parts.shown.get(key).is_none_or(|(shown, _)| *shown != entry)
                    || (entry.is_none() && new_name_changed);
                if fresh {
                    draw_row(&mut commands, row, entry.as_ref(), &model.new_name, &labels, colors);
                    // The row entity keeps its selection state (a new one starts unselected).
                    let selected = parts.shown.get(key).is_some_and(|(_, s)| *s);
                    parts.shown.insert(key.clone(), (entry, selected));
                }
            }
            if let Ok(mut node) = nodes.get_mut(parts.empty) {
                show(&mut node, parts.order.is_empty());
            }
            // A selection that left the list: the same index, else the first row.
            if let SaveLoadFocus::Row(i) = screen.focus
                && i >= parts.order.len()
            {
                screen.focus = if parts.order.is_empty() {
                    SaveLoadFocus::Button(2)
                } else {
                    SaveLoadFocus::Row(parts.order.len() - 1)
                };
            }
            if screen
                .selected
                .as_ref()
                .is_some_and(|s| !parts.order.contains(s))
            {
                screen.selected = None;
            }
            if screen.pending.as_ref().is_some_and(|p| match p {
                SaveLoadChoice::Overwrite(id) | SaveLoadChoice::Delete(id) => {
                    model.entry(id).is_none()
                }
                _ => false,
            }) {
                screen.pending = None;
            }
            parts.seen = Some(model);
        }

        // Focus: the question takes it; otherwise a row or a footer button. Focus on a row
        // selects it.
        match (screen.pending.is_some(), screen.focus) {
            (true, SaveLoadFocus::Confirm(_)) => {}
            (true, _) => screen.focus = SaveLoadFocus::Confirm(1),
            (false, SaveLoadFocus::Confirm(_) | SaveLoadFocus::Start) => {
                let index = screen
                    .selected
                    .as_ref()
                    .and_then(|s| parts.order.iter().position(|k| k == s));
                screen.focus = match index {
                    Some(i) => SaveLoadFocus::Row(i),
                    None if parts.order.is_empty() => SaveLoadFocus::Button(2),
                    None => SaveLoadFocus::Row(0),
                };
            }
            (false, _) => {}
        }
        if let SaveLoadFocus::Row(i) = screen.focus
            && screen.selected.as_ref() != parts.order.get(i)
        {
            screen.selected = parts.order.get(i).cloned();
        }

        // Selected class and fallback highlight.
        for key in &parts.order {
            let selected = screen.selected.as_ref() == Some(key);
            let Some((_, shown)) = parts.shown.get_mut(key) else {
                continue;
            };
            if *shown == selected {
                continue;
            }
            *shown = selected;
            if let Some(row) = parts.rows.get(key) {
                commands.entity(row).insert((
                    classes("sf-saveload-row", &[("selected", selected)]),
                    BackgroundColor(if selected { colors.highlight } else { colors.cell }),
                ));
            }
        }

        let disabled = [
            !screen.primary_enabled(),
            !(screen.model.can_delete && screen.selected_entry().is_some()),
        ];
        for (i, off) in disabled.into_iter().enumerate() {
            if parts.disabled[i] != off {
                parts.disabled[i] = off;
                set_disabled(&mut commands, parts.buttons[i], off);
                commands.entity(parts.buttons[i]).insert(classes(
                    if i == 0 {
                        "sf-saveload-primary"
                    } else {
                        "sf-saveload-delete"
                    },
                    &[("disabled", off)],
                ));
            }
        }

        // The question.
        let question = match &screen.pending {
            Some(SaveLoadChoice::Overwrite(id)) => labels.confirm_overwrite.replace(
                "{name}",
                screen.model.entry(id).map_or(id.as_str(), |e| e.title.as_str()),
            ),
            Some(SaveLoadChoice::Delete(id)) => labels.confirm_delete.replace(
                "{name}",
                screen.model.entry(id).map_or(id.as_str(), |e| e.title.as_str()),
            ),
            _ => String::new(),
        };
        parts.question.set(&mut commands, &mut nodes, question, colors);

        if !is_target(&stack, root) {
            continue;
        }
        if let Some(target) = parts.target(screen.focus)
            && focus.get() != Some(target)
        {
            focus.set(target, FocusCause::Navigated);
        }
    }
}

/// Directions move the focus; Back cancels the question, else closes; Context1 asks to delete.
pub(crate) fn saveload_input(
    mut navs: MessageReader<UiNavEvent>,
    mut actions: MessageReader<UiActionEvent>,
    mut stack: ResMut<ScreenStackRes>,
    mut screens: Query<(Entity, &UiScreenRoot, &mut SaveLoadScreen, &SaveLoadParts)>,
    mut out: MessageWriter<SaveLoadEvent>,
) {
    let navs: Vec<UiNavEvent> = navs.read().cloned().collect();
    let actions: Vec<UiActionEvent> = actions.read().cloned().collect();
    for (entity, root, mut screen, parts) in &mut screens {
        for nav in navs.iter().filter(|n| n.screen == root.0) {
            let next = step(screen.focus, nav.direction, parts.order.len());
            if next != screen.focus {
                screen.focus = next;
            }
        }
        for action in actions.iter().filter(|a| a.screen == root.0) {
            match action.action {
                UiAction::Back => {
                    if screen.pending.is_some() {
                        screen.pending = None;
                    } else {
                        out.write(SaveLoadEvent {
                            screen: entity,
                            choice: SaveLoadChoice::Back,
                        });
                        stack.0.close(root.0.as_str());
                    }
                }
                UiAction::Context1 if screen.pending.is_none() => screen.delete(),
                _ => {}
            }
        }
    }
}

/// One focus step: up and down through the rows and onto the footer, left and right along
/// the footer or the question (clamped, no wrap).
fn step(focus: SaveLoadFocus, direction: CompassOctant, rows: usize) -> SaveLoadFocus {
    let (dx, dy): (isize, isize) = match direction {
        CompassOctant::North => (0, -1),
        CompassOctant::South => (0, 1),
        CompassOctant::East => (1, 0),
        CompassOctant::West => (-1, 0),
        _ => (0, 0),
    };
    match focus {
        SaveLoadFocus::Confirm(i) => SaveLoadFocus::Confirm(i.saturating_add_signed(dx).min(1)),
        SaveLoadFocus::Start => SaveLoadFocus::Start,
        SaveLoadFocus::Row(i) if dy > 0 && i + 1 >= rows => SaveLoadFocus::Button(0),
        SaveLoadFocus::Row(i) => SaveLoadFocus::Row(i.saturating_add_signed(dy)),
        SaveLoadFocus::Button(_) if dy < 0 && rows > 0 => SaveLoadFocus::Row(rows - 1),
        SaveLoadFocus::Button(i) => SaveLoadFocus::Button(i.saturating_add_signed(dx).min(2)),
    }
}

/// A part activated (confirm, click): select a row (or run the primary action on the selected
/// one), a footer button, or answer the question.
pub(crate) fn on_saveload_part(
    activate: On<Activate>,
    kinds: Query<&SaveLoadPart>,
    parents: Query<&ChildOf>,
    mut screens: Query<(&UiScreenRoot, &mut SaveLoadScreen, &SaveLoadParts)>,
    mut stack: ResMut<ScreenStackRes>,
    mut out: MessageWriter<SaveLoadEvent>,
) {
    let Ok(kind) = kinds.get(activate.entity) else {
        return;
    };
    let Some(entity) = screen_of(activate.entity, &parents, |e| screens.contains(e)) else {
        return;
    };
    let Ok((root, mut screen, parts)) = screens.get_mut(entity) else {
        return;
    };
    let choice = match kind {
        SaveLoadPart::Yes => screen.pending.take(),
        SaveLoadPart::Cancel => {
            screen.pending = None;
            None
        }
        // The question is modal.
        _ if screen.pending.is_some() => None,
        SaveLoadPart::Row(key) => {
            if let Some(i) = parts.order.iter().position(|k| k == key) {
                screen.focus = SaveLoadFocus::Row(i);
            }
            if screen.selected.as_ref() == Some(key) {
                screen.primary()
            } else {
                screen.selected = Some(key.clone());
                None
            }
        }
        SaveLoadPart::Primary => screen.primary(),
        SaveLoadPart::Delete => {
            screen.delete();
            None
        }
        SaveLoadPart::Back => {
            stack.0.close(root.0.as_str());
            Some(SaveLoadChoice::Back)
        }
    };
    if let Some(choice) = choice {
        out.write(SaveLoadEvent {
            screen: entity,
            choice,
        });
    }
}
