//! The main (title) menu (ui.md "Stock screens") over a view-model the app fills:
//! [`MainMenuModel`], the entries in the app's order, each a stable id, an already-localized
//! label and an enabled flag (a game lists Continue, New game, Load, Options, Credits, Quit; the
//! framework names none of them). The screen sends [`MainMenuEvent`]s with the chosen id and
//! never closes itself: the app starts the game, opens a screen on top or quits.
//!
//! A full-screen root with an optional background image, an optional title image and title
//! text, and a column of entries (keyed by id): focus starts on the first enabled entry, up and
//! down skip disabled entries (clamped, no wrap), confirm or a click chooses. The entry the
//! model names as [`MainMenuModel::quit`] asks first (No focused); No or Back answers no and
//! returns to it. Back otherwise does nothing (the top level), unless the model names a
//! [`MainMenuModel::back`] entry: Back then chooses it as a confirm on it would.
//!
//! Parts: `sf-main-menu`, `-background`, `-title-image`, `-title`, `-entries`, `-entry` (with
//! the entry's id and `disabled` as classes), `-confirm`, `-confirm-text`, `-confirm-choices`,
//! `-yes`, `-no`.

use std::collections::HashMap;

use bevy_asset::Handle;
use bevy_ecs::prelude::*;
use bevy_image::Image;
use bevy_input_focus::{FocusCause, InputFocus};
use bevy_math::CompassOctant;
use bevy_text::TextColor;
use bevy_ui::prelude::*;
use bevy_ui_widgets::Activate;
use sf_ui_bevy::{
    ScreenStackRes, UiAction, UiActionEvent, UiHandlesBack, UiNavEvent, UiOwnsFocus, UiScreenRoot,
};
use sf_ui_css::UiClass;

use crate::keyed::KeyedChildren;
use crate::labels::ScreenLabels;
use crate::widgets::{Question, ScreenColors, button, label, part, relabel, set_disabled, show};
use crate::{is_target, screen_of};

/// One entry as the app lists it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MainMenuEntry {
    /// Stable id, sent back in [`MainMenuEvent`] and added as a class (`.sf-main-menu-entry.quit`).
    pub id: String,
    /// Already localized.
    pub label: String,
    /// Disabled entries are shown (class `disabled`), skipped by navigation and not chosen.
    pub enabled: bool,
}

impl MainMenuEntry {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            enabled: true,
        }
    }

    #[must_use]
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

/// The screen's view-model: the entries in display order (a repeated id is listed once) and
/// the slots the game fills.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MainMenuModel {
    pub entries: Vec<MainMenuEntry>,
    /// The entry that asks a yes/no question before it is sent (the app's quit).
    pub quit: Option<String>,
    /// The entry Back chooses; none: Back does nothing.
    pub back: Option<String>,
    /// The game's logo, above the entries.
    pub title_image: Option<Handle<Image>>,
    /// Text title (already localized; empty: hidden), under the logo.
    pub title_text: String,
    /// Full-screen image behind the menu.
    pub background: Option<Handle<Image>>,
}

impl MainMenuModel {
    pub fn new(entries: impl IntoIterator<Item = MainMenuEntry>) -> Self {
        Self {
            entries: entries.into_iter().collect(),
            ..Self::default()
        }
    }

    /// Names the entry that asks first.
    #[must_use]
    pub fn with_quit(mut self, id: impl Into<String>) -> Self {
        self.quit = Some(id.into());
        self
    }

    /// Names the entry Back chooses.
    #[must_use]
    pub fn with_back(mut self, id: impl Into<String>) -> Self {
        self.back = Some(id.into());
        self
    }

    /// Enables or disables a listed entry.
    pub fn set_enabled(&mut self, id: &str, enabled: bool) -> &mut Self {
        for entry in self.entries.iter_mut().filter(|e| e.id == id) {
            entry.enabled = enabled;
        }
        self
    }

    /// The entry (its first listing).
    pub fn entry(&self, id: &str) -> Option<&MainMenuEntry> {
        self.entries.iter().find(|e| e.id == id)
    }

    /// The entry is listed and enabled.
    pub fn is_enabled(&self, id: &str) -> bool {
        self.entry(id).is_some_and(|e| e.enabled)
    }

    /// The listed ids in order, each once.
    pub fn ids(&self) -> Vec<String> {
        let mut out: Vec<String> = Vec::with_capacity(self.entries.len());
        for entry in &self.entries {
            if !out.contains(&entry.id) {
                out.push(entry.id.clone());
            }
        }
        out
    }

    fn is_quit(&self, id: &str) -> bool {
        self.quit.as_deref() == Some(id)
    }
}

/// A chosen entry, for the app to carry out. The screen stays open.
#[derive(Message, Clone, Debug, PartialEq, Eq)]
pub struct MainMenuEvent {
    pub screen: Entity,
    pub id: String,
}

/// Where the screen's focus is.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum MainMenuFocus {
    #[default]
    Start,
    Entry(String),
    /// An answer of the quit question: yes, no.
    Confirm(usize),
}

/// On a screen root: the main menu. The app may replace `model` at any time (entries are keyed
/// by id).
#[derive(Component, Clone, Debug, Default)]
#[require(Node, UiOwnsFocus, UiHandlesBack)]
pub struct MainMenuScreen {
    pub model: MainMenuModel,
    focus: MainMenuFocus,
    /// The quit question is up.
    asking: bool,
}

impl MainMenuScreen {
    pub fn new(model: MainMenuModel) -> Self {
        Self {
            model,
            ..Self::default()
        }
    }

    pub fn focus(&self) -> &MainMenuFocus {
        &self.focus
    }

    /// The quit question is up.
    pub fn asking_quit(&self) -> bool {
        self.asking
    }

    /// Choosing an entry: the quit entry asks first; a disabled entry does nothing.
    fn choose(&mut self, id: &str) -> Option<String> {
        if !self.model.is_enabled(id) {
            return None;
        }
        if self.model.is_quit(id) {
            self.asking = true;
            // No first: a stray confirm never quits.
            self.focus = MainMenuFocus::Confirm(1);
            return None;
        }
        Some(id.to_owned())
    }

    /// Closes the question, focus back on the quit entry.
    fn answer(&mut self) {
        self.asking = false;
        self.focus = self
            .model
            .quit
            .clone()
            .map_or(MainMenuFocus::Start, MainMenuFocus::Entry);
    }

    /// Where focus lands: the first enabled entry.
    fn home(&self) -> MainMenuFocus {
        self.model
            .entries
            .iter()
            .find(|e| e.enabled)
            .map_or(MainMenuFocus::Start, |e| MainMenuFocus::Entry(e.id.clone()))
    }

    /// One focus step: up and down to the next enabled entry, left and right in the question.
    fn step(&self, direction: CompassOctant) -> MainMenuFocus {
        match (&self.focus, direction) {
            (MainMenuFocus::Confirm(_), CompassOctant::West) => MainMenuFocus::Confirm(0),
            (MainMenuFocus::Confirm(_), CompassOctant::East) => MainMenuFocus::Confirm(1),
            (MainMenuFocus::Entry(current), CompassOctant::North | CompassOctant::South) => {
                let ids = self.model.ids();
                let Some(at) = ids.iter().position(|id| id == current) else {
                    return self.focus.clone();
                };
                let enabled = |id: &&String| self.model.is_enabled(id);
                let next = if direction == CompassOctant::South {
                    ids[at + 1..].iter().find(enabled)
                } else {
                    ids[..at].iter().rev().find(enabled)
                };
                next.map_or_else(|| self.focus.clone(), |id| MainMenuFocus::Entry(id.clone()))
            }
            _ => self.focus.clone(),
        }
    }
}

/// A focusable part of the main menu.
#[derive(Component, Clone, Debug, PartialEq, Eq)]
pub enum MainMenuPart {
    Entry(String),
    Yes,
    No,
}

/// The screen's parts and what they show.
#[derive(Component, Debug)]
pub struct MainMenuParts {
    background: Entity,
    title_image: Entity,
    title: Entity,
    entries: KeyedChildren<String>,
    order: Vec<String>,
    /// What each entry shows: enabled, label.
    shown: HashMap<String, (bool, String)>,
    seen: Option<MainMenuModel>,
    /// Quit: yes, no.
    question: Question,
}

impl MainMenuParts {
    /// The entity of an entry.
    pub fn entry(&self, id: &str) -> Option<Entity> {
        self.entries.get(&id.to_owned())
    }

    /// The entries in display order.
    pub fn order(&self) -> &[String] {
        &self.order
    }

    pub fn background(&self) -> Entity {
        self.background
    }

    pub fn title_image(&self) -> Entity {
        self.title_image
    }

    pub fn title(&self) -> Entity {
        self.title
    }

    pub fn overlay(&self) -> Entity {
        self.question.overlay
    }

    /// Yes, no.
    pub fn confirm_buttons(&self) -> [Entity; 2] {
        self.question.buttons
    }

    fn target(&self, focus: &MainMenuFocus) -> Option<Entity> {
        match focus {
            MainMenuFocus::Start => None,
            MainMenuFocus::Entry(id) => self.entries.get(id),
            MainMenuFocus::Confirm(i) => self.question.buttons.get(*i).copied(),
        }
    }
}

/// The entry's classes: `sf-main-menu-entry <id> [disabled]` (an id that is not a valid class
/// name is left out).
fn entry_class(id: &str, enabled: bool) -> UiClass {
    let mut out = String::from("sf-main-menu-entry");
    if !id.is_empty() && !id.contains(char::is_whitespace) {
        out.push(' ');
        out.push_str(id);
    }
    if !enabled {
        out.push_str(" disabled");
    }
    UiClass::new(out)
}

/// A hidden image part.
fn image_part(name: &'static str, node: Node) -> impl Bundle {
    (
        part(name),
        Node {
            display: Display::None,
            ..node
        },
    )
}

pub(crate) fn spawn_main_menu(
    mut commands: Commands,
    colors: Res<ScreenColors>,
    labels: Res<ScreenLabels>,
    roots: Query<Entity, Added<MainMenuScreen>>,
) {
    for root in &roots {
        commands.entity(root).insert((
            part("sf-main-menu"),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(16.0),
                ..Node::default()
            },
        ));
        // First child: drawn behind the rest.
        let background = commands
            .spawn((
                image_part(
                    "sf-main-menu-background",
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(0.0),
                        top: Val::Px(0.0),
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..Node::default()
                    },
                ),
                ChildOf(root),
            ))
            .id();
        let title_image = commands
            .spawn((
                image_part(
                    "sf-main-menu-title-image",
                    Node {
                        max_width: Val::Percent(80.0),
                        max_height: Val::Percent(40.0),
                        ..Node::default()
                    },
                ),
                ChildOf(root),
            ))
            .id();
        let title = commands
            .spawn((
                label("sf-main-menu-title", "", &colors),
                Node {
                    display: Display::None,
                    ..Node::default()
                },
                ChildOf(root),
            ))
            .id();
        let list = commands
            .spawn((
                part("sf-main-menu-entries"),
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Stretch,
                    padding: UiRect::all(Val::Px(16.0)),
                    row_gap: Val::Px(4.0),
                    min_width: Val::Px(280.0),
                    max_width: Val::Percent(100.0),
                    ..Node::default()
                },
                BackgroundColor(colors.panel),
                ChildOf(root),
            ))
            .id();
        let question = Question::spawn(
            &mut commands,
            root,
            [
                "sf-main-menu-confirm",
                "sf-main-menu-confirm-text",
                "sf-main-menu-confirm-choices",
            ],
            [
                ("sf-main-menu-yes", labels.yes.clone(), MainMenuPart::Yes),
                ("sf-main-menu-no", labels.no.clone(), MainMenuPart::No),
            ],
            &colors,
        );
        commands.entity(root).insert(MainMenuParts {
            background,
            title_image,
            title,
            entries: KeyedChildren::new(list),
            order: Vec::new(),
            shown: HashMap::new(),
            seen: None,
            question,
        });
    }
}

/// Shows `image` on `entity` (none: hidden).
fn set_image(
    commands: &mut Commands,
    nodes: &mut Query<&mut Node>,
    entity: Entity,
    image: Option<&Handle<Image>>,
) {
    if let Some(image) = image {
        commands
            .entity(entity)
            .insert(ImageNode::new(image.clone()));
    }
    if let Ok(mut node) = nodes.get_mut(entity) {
        show(&mut node, image.is_some());
    }
}

/// Model → images, title, entries (keyed; enabled state and label written when they change),
/// the question and focus. Writes only what changed.
#[allow(clippy::too_many_lines)]
pub(crate) fn sync_main_menu(
    mut commands: Commands,
    stack: Res<ScreenStackRes>,
    colors: Res<ScreenColors>,
    labels: Res<ScreenLabels>,
    mut screens: Query<(&UiScreenRoot, &mut MainMenuScreen, &mut MainMenuParts)>,
    mut nodes: Query<&mut Node>,
    mut focus: ResMut<InputFocus>,
) {
    for (root, mut screen, mut parts) in &mut screens {
        let parts = &mut *parts;
        let colors = &*colors;
        if parts.seen.as_ref() != Some(&screen.model) {
            let model = screen.model.clone();
            let seen = parts.seen.take();
            let first = seen.is_none();
            let was = seen.unwrap_or_default();
            if first || was.background != model.background {
                set_image(
                    &mut commands,
                    &mut nodes,
                    parts.background,
                    model.background.as_ref(),
                );
            }
            if first || was.title_image != model.title_image {
                set_image(
                    &mut commands,
                    &mut nodes,
                    parts.title_image,
                    model.title_image.as_ref(),
                );
            }
            if first || was.title_text != model.title_text {
                commands
                    .entity(parts.title)
                    .insert((Text::new(model.title_text.clone()), TextColor(colors.text)));
                if let Ok(mut node) = nodes.get_mut(parts.title) {
                    show(&mut node, !model.title_text.is_empty());
                }
            }

            parts.order = model.ids();
            parts.entries.sync(
                &mut commands,
                parts.order.iter().cloned(),
                |commands, id| {
                    let text = model.entry(id).map(|e| e.label.clone()).unwrap_or_default();
                    commands
                        .spawn((
                            button("sf-main-menu-entry", text, colors),
                            MainMenuPart::Entry(id.clone()),
                        ))
                        .id()
                },
            );
            parts.shown.retain(|id, _| parts.order.contains(id));
            for id in &parts.order {
                let Some(entry) = model.entry(id) else {
                    continue;
                };
                let now = (entry.enabled, entry.label.clone());
                if parts.shown.get(id) == Some(&now) {
                    continue;
                }
                parts.shown.insert(id.clone(), now);
                let Some(button) = parts.entries.get(id) else {
                    continue;
                };
                set_disabled(&mut commands, button, !entry.enabled);
                commands
                    .entity(button)
                    .insert(entry_class(id, entry.enabled));
                relabel(
                    &mut commands,
                    button,
                    entry.label.clone(),
                    if entry.enabled {
                        colors.text
                    } else {
                        colors.muted
                    },
                );
            }
            if screen.asking && !model.quit.as_deref().is_some_and(|q| model.is_enabled(q)) {
                screen.asking = false;
            }
            parts.seen = Some(model);
        }

        // Focus: the question takes it; otherwise an enabled entry.
        let next = match (screen.asking, &screen.focus) {
            (true, MainMenuFocus::Confirm(_)) => screen.focus.clone(),
            (true, _) => MainMenuFocus::Confirm(1),
            (false, MainMenuFocus::Entry(id)) if screen.model.is_enabled(id) => {
                screen.focus.clone()
            }
            (false, _) => screen.home(),
        };
        if next != screen.focus {
            screen.focus = next;
        }

        let question = if screen.asking {
            labels.main_confirm_quit.clone()
        } else {
            String::new()
        };
        parts
            .question
            .set(&mut commands, &mut nodes, question, colors);

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

/// Directions move the focus; Back answers no to the question, else chooses the model's back
/// entry (none: nothing).
pub(crate) fn main_menu_input(
    mut navs: MessageReader<UiNavEvent>,
    mut actions: MessageReader<UiActionEvent>,
    mut screens: Query<(Entity, &UiScreenRoot, &mut MainMenuScreen)>,
    mut out: MessageWriter<MainMenuEvent>,
) {
    let navs: Vec<UiNavEvent> = navs.read().cloned().collect();
    let actions: Vec<UiActionEvent> = actions.read().cloned().collect();
    for (entity, root, mut screen) in &mut screens {
        for nav in navs.iter().filter(|n| n.screen == root.0) {
            let next = screen.step(nav.direction);
            if next != screen.focus {
                screen.focus = next;
            }
        }
        for action in actions.iter().filter(|a| a.screen == root.0) {
            if action.action != UiAction::Back {
                continue;
            }
            if screen.asking {
                screen.answer();
                continue;
            }
            let Some(back) = screen.model.back.clone() else {
                continue;
            };
            if let Some(id) = screen.choose(&back) {
                out.write(MainMenuEvent { screen: entity, id });
            }
        }
    }
}

/// A part activated (confirm, click): choose an entry or answer the question.
pub(crate) fn on_main_menu_part(
    activate: On<Activate>,
    kinds: Query<&MainMenuPart>,
    parents: Query<&ChildOf>,
    mut screens: Query<&mut MainMenuScreen>,
    mut out: MessageWriter<MainMenuEvent>,
) {
    let Ok(kind) = kinds.get(activate.entity) else {
        return;
    };
    let Some(entity) = screen_of(activate.entity, &parents, |e| screens.contains(e)) else {
        return;
    };
    let Ok(mut screen) = screens.get_mut(entity) else {
        return;
    };
    let chosen = match kind {
        MainMenuPart::Yes if screen.asking => {
            screen.answer();
            screen.model.quit.clone()
        }
        MainMenuPart::No if screen.asking => {
            screen.answer();
            None
        }
        // The question is modal.
        _ if screen.asking => None,
        MainMenuPart::Entry(id) => {
            if screen.model.is_enabled(id) {
                screen.focus = MainMenuFocus::Entry(id.clone());
            }
            screen.choose(id)
        }
        MainMenuPart::Yes | MainMenuPart::No => None,
    };
    if let Some(id) = chosen {
        out.write(MainMenuEvent { screen: entity, id });
    }
}
