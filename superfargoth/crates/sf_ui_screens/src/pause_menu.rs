//! The pause (escape) menu (ui.md "Stock screens") over a view-model the app fills:
//! [`PauseMenuModel`], the entries in order, each enabled or not (the app disables Save during
//! combat, for example). The screen knows nothing about saving, options or quitting: it sends
//! [`PauseMenuEvent`]s and the app opens the save/load or options screen on top, or quits.
//!
//! A column of entries (keyed by [`PauseChoice`]): focus starts on Resume (else the first
//! enabled entry), up and down skip disabled entries (clamped, no wrap), confirm or a click
//! chooses. Quit asks first (No focused); No or Back answers no and returns to Quit. Back
//! otherwise means Resume. Resume, chosen or by Back, sends [`PauseChoice::Resume`] and closes
//! the screen; the other choices leave it open (the app closes it when it quits, and the
//! save/load or options screen it opens covers it until closed).
//!
//! Pause policy: the screen record declares `pause = "WhileOpen"` (ui.md "Screen" records; the
//! demo's `ui.screen` "pause"), so `ScreenStack::pauses_game` is true while it is open; the app
//! pauses its simulation on that, not on this component.

use std::collections::HashMap;

use bevy_ecs::prelude::*;
use bevy_input_focus::{FocusCause, InputFocus};
use bevy_math::CompassOctant;
use bevy_ui::prelude::*;
use bevy_ui_widgets::Activate;
use sf_ui_bevy::{
    ScreenStackRes, UiAction, UiActionEvent, UiHandlesBack, UiNavEvent, UiOwnsFocus, UiScreenRoot,
};

use crate::keyed::KeyedChildren;
use crate::labels::ScreenLabels;
use crate::widgets::{
    Question, ScreenColors, button, centred_window, classes, label, part, relabel, set_disabled,
};
use crate::{is_target, screen_of};

/// An entry of the pause menu.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PauseChoice {
    Resume,
    Save,
    Load,
    Options,
    Quit,
}

impl PauseChoice {
    /// Every entry, in the default order.
    pub const ALL: [Self; 5] = [
        Self::Resume,
        Self::Save,
        Self::Load,
        Self::Options,
        Self::Quit,
    ];

    fn label(self, labels: &ScreenLabels) -> &str {
        match self {
            Self::Resume => &labels.resume,
            Self::Save => &labels.pause_save,
            Self::Load => &labels.pause_load,
            Self::Options => &labels.options,
            Self::Quit => &labels.quit,
        }
    }

    /// The entry's own class (`.sf-pause-entry.quit`).
    fn class(self) -> &'static str {
        match self {
            Self::Resume => "resume",
            Self::Save => "save",
            Self::Load => "load",
            Self::Options => "options",
            Self::Quit => "quit",
        }
    }
}

/// One entry as the app lists it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PauseEntry {
    pub choice: PauseChoice,
    /// Disabled entries are shown (class `disabled`), skipped by navigation and not chosen.
    pub enabled: bool,
}

impl PauseEntry {
    pub fn new(choice: PauseChoice) -> Self {
        Self {
            choice,
            enabled: true,
        }
    }
}

/// The screen's view-model: the entries in display order (a repeated choice is listed once).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PauseMenuModel {
    pub entries: Vec<PauseEntry>,
}

impl Default for PauseMenuModel {
    /// All five entries, enabled.
    fn default() -> Self {
        Self {
            entries: PauseChoice::ALL.map(PauseEntry::new).to_vec(),
        }
    }
}

impl PauseMenuModel {
    /// Enables or disables a listed entry.
    pub fn set_enabled(&mut self, choice: PauseChoice, enabled: bool) -> &mut Self {
        for entry in self.entries.iter_mut().filter(|e| e.choice == choice) {
            entry.enabled = enabled;
        }
        self
    }

    /// The entry is listed and enabled.
    pub fn is_enabled(&self, choice: PauseChoice) -> bool {
        self.entries
            .iter()
            .find(|e| e.choice == choice)
            .is_some_and(|e| e.enabled)
    }

    /// The listed choices in order, each once.
    pub fn choices(&self) -> Vec<PauseChoice> {
        let mut out = Vec::with_capacity(self.entries.len());
        for entry in &self.entries {
            if !out.contains(&entry.choice) {
                out.push(entry.choice);
            }
        }
        out
    }
}

/// A choice, for the app to carry out. Resume has already closed the screen.
#[derive(Message, Clone, Copy, Debug, PartialEq, Eq)]
pub struct PauseMenuEvent {
    pub screen: Entity,
    pub choice: PauseChoice,
}

/// Where the screen's focus is.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PauseMenuFocus {
    #[default]
    Start,
    Entry(PauseChoice),
    /// An answer of the quit question: yes, no.
    Confirm(usize),
}

/// On a screen root: the pause menu. The app may replace `model` at any time (entries are
/// keyed by choice).
#[derive(Component, Clone, Debug, Default)]
#[require(Node, UiOwnsFocus, UiHandlesBack)]
pub struct PauseMenuScreen {
    pub model: PauseMenuModel,
    focus: PauseMenuFocus,
    /// The quit question is up.
    asking: bool,
}

impl PauseMenuScreen {
    pub fn new(model: PauseMenuModel) -> Self {
        Self {
            model,
            ..Self::default()
        }
    }

    pub fn focus(&self) -> PauseMenuFocus {
        self.focus
    }

    /// The quit question is up.
    pub fn asking_quit(&self) -> bool {
        self.asking
    }

    /// Choosing an entry: Quit asks first; a disabled entry does nothing.
    fn choose(&mut self, choice: PauseChoice) -> Option<PauseChoice> {
        if !self.model.is_enabled(choice) {
            return None;
        }
        if choice == PauseChoice::Quit {
            self.asking = true;
            // No first: a stray confirm never quits.
            self.focus = PauseMenuFocus::Confirm(1);
            return None;
        }
        Some(choice)
    }

    /// Closes the question, focus back on Quit.
    fn answer(&mut self) {
        self.asking = false;
        self.focus = PauseMenuFocus::Entry(PauseChoice::Quit);
    }

    /// Where focus lands: Resume, else the first enabled entry.
    fn home(&self) -> PauseMenuFocus {
        if self.model.is_enabled(PauseChoice::Resume) {
            return PauseMenuFocus::Entry(PauseChoice::Resume);
        }
        self.model
            .entries
            .iter()
            .find(|e| e.enabled)
            .map_or(PauseMenuFocus::Start, |e| PauseMenuFocus::Entry(e.choice))
    }

    /// One focus step: up and down to the next enabled entry, left and right in the question.
    fn step(&self, direction: CompassOctant) -> PauseMenuFocus {
        match (self.focus, direction) {
            (PauseMenuFocus::Confirm(_), CompassOctant::West) => PauseMenuFocus::Confirm(0),
            (PauseMenuFocus::Confirm(_), CompassOctant::East) => PauseMenuFocus::Confirm(1),
            (PauseMenuFocus::Entry(current), CompassOctant::North | CompassOctant::South) => {
                let choices = self.model.choices();
                let Some(at) = choices.iter().position(|c| *c == current) else {
                    return self.focus;
                };
                let enabled = |c: &&PauseChoice| self.model.is_enabled(**c);
                let next = if direction == CompassOctant::South {
                    choices[at + 1..].iter().find(enabled)
                } else {
                    choices[..at].iter().rev().find(enabled)
                };
                next.map_or(self.focus, |c| PauseMenuFocus::Entry(*c))
            }
            _ => self.focus,
        }
    }
}

/// A focusable part of the pause menu.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub enum PauseMenuPart {
    Entry(PauseChoice),
    Yes,
    No,
}

/// The screen's parts and what they show.
#[derive(Component, Debug)]
pub struct PauseMenuParts {
    entries: KeyedChildren<PauseChoice>,
    order: Vec<PauseChoice>,
    /// Whether each entry is shown enabled.
    shown: HashMap<PauseChoice, bool>,
    seen: Option<PauseMenuModel>,
    /// Quit: yes, no.
    question: Question,
}

impl PauseMenuParts {
    /// The entity of an entry.
    pub fn entry(&self, choice: PauseChoice) -> Option<Entity> {
        self.entries.get(&choice)
    }

    /// The entries in display order.
    pub fn order(&self) -> &[PauseChoice] {
        &self.order
    }

    pub fn overlay(&self) -> Entity {
        self.question.overlay
    }

    /// Yes, no.
    pub fn confirm_buttons(&self) -> [Entity; 2] {
        self.question.buttons
    }

    fn target(&self, focus: PauseMenuFocus) -> Option<Entity> {
        match focus {
            PauseMenuFocus::Start => None,
            PauseMenuFocus::Entry(choice) => self.entries.get(&choice),
            PauseMenuFocus::Confirm(i) => self.question.buttons.get(i).copied(),
        }
    }
}

pub(crate) fn spawn_pause_menu(
    mut commands: Commands,
    colors: Res<ScreenColors>,
    labels: Res<ScreenLabels>,
    roots: Query<Entity, Added<PauseMenuScreen>>,
) {
    for root in &roots {
        commands.entity(root).insert((
            part("sf-pause"),
            centred_window(Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Stretch,
                padding: UiRect::all(Val::Px(16.0)),
                row_gap: Val::Px(8.0),
                min_width: Val::Px(280.0),
                max_width: Val::Percent(100.0),
                max_height: Val::Percent(100.0),
                ..Node::default()
            }),
            BackgroundColor(colors.panel),
        ));
        commands.spawn((
            label("sf-pause-title", labels.pause_title.clone(), &colors),
            ChildOf(root),
        ));
        let list = commands
            .spawn((
                part("sf-pause-entries"),
                Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(4.0),
                    ..Node::default()
                },
                ChildOf(root),
            ))
            .id();
        let question = Question::spawn(
            &mut commands,
            root,
            [
                "sf-pause-confirm",
                "sf-pause-confirm-text",
                "sf-pause-confirm-choices",
            ],
            [
                ("sf-pause-yes", labels.yes.clone(), PauseMenuPart::Yes),
                ("sf-pause-no", labels.no.clone(), PauseMenuPart::No),
            ],
            &colors,
        );
        commands.entity(root).insert(PauseMenuParts {
            entries: KeyedChildren::new(list),
            order: Vec::new(),
            shown: HashMap::new(),
            seen: None,
            question,
        });
    }
}

/// Model → entries (keyed; the enabled state written when it flips), the question and focus.
/// Writes only what changed.
pub(crate) fn sync_pause_menu(
    mut commands: Commands,
    stack: Res<ScreenStackRes>,
    colors: Res<ScreenColors>,
    labels: Res<ScreenLabels>,
    mut screens: Query<(&UiScreenRoot, &mut PauseMenuScreen, &mut PauseMenuParts)>,
    mut nodes: Query<&mut Node>,
    mut focus: ResMut<InputFocus>,
) {
    for (root, mut screen, mut parts) in &mut screens {
        let parts = &mut *parts;
        let colors = &*colors;
        if parts.seen.as_ref() != Some(&screen.model) {
            let model = screen.model.clone();
            parts.order = model.choices();
            parts.entries.sync(
                &mut commands,
                parts.order.iter().copied(),
                |commands, choice| {
                    commands
                        .spawn((
                            button("sf-pause-entry", choice.label(&labels).to_owned(), colors),
                            PauseMenuPart::Entry(*choice),
                        ))
                        .id()
                },
            );
            parts.shown.retain(|c, _| parts.order.contains(c));
            for &choice in &parts.order {
                let enabled = model.is_enabled(choice);
                if parts.shown.get(&choice) == Some(&enabled) {
                    continue;
                }
                parts.shown.insert(choice, enabled);
                let Some(entry) = parts.entries.get(&choice) else {
                    continue;
                };
                set_disabled(&mut commands, entry, !enabled);
                commands.entity(entry).insert(classes(
                    "sf-pause-entry",
                    &[(choice.class(), true), ("disabled", !enabled)],
                ));
                relabel(
                    &mut commands,
                    entry,
                    choice.label(&labels).to_owned(),
                    if enabled { colors.text } else { colors.muted },
                );
            }
            if screen.asking && !model.is_enabled(PauseChoice::Quit) {
                screen.asking = false;
            }
            parts.seen = Some(model);
        }

        // Focus: the question takes it; otherwise an enabled entry.
        let next = match (screen.asking, screen.focus) {
            (true, PauseMenuFocus::Confirm(_)) => screen.focus,
            (true, _) => PauseMenuFocus::Confirm(1),
            (false, PauseMenuFocus::Entry(c)) if screen.model.is_enabled(c) => screen.focus,
            (false, _) => screen.home(),
        };
        if next != screen.focus {
            screen.focus = next;
        }

        let question = if screen.asking {
            labels.confirm_quit.clone()
        } else {
            String::new()
        };
        parts
            .question
            .set(&mut commands, &mut nodes, question, colors);

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

/// Directions move the focus; Back answers no to the question, else resumes (and closes).
pub(crate) fn pause_menu_input(
    mut navs: MessageReader<UiNavEvent>,
    mut actions: MessageReader<UiActionEvent>,
    mut stack: ResMut<ScreenStackRes>,
    mut screens: Query<(Entity, &UiScreenRoot, &mut PauseMenuScreen)>,
    mut out: MessageWriter<PauseMenuEvent>,
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
            } else {
                out.write(PauseMenuEvent {
                    screen: entity,
                    choice: PauseChoice::Resume,
                });
                stack.0.close(root.0.as_str());
            }
        }
    }
}

/// A part activated (confirm, click): choose an entry or answer the question.
pub(crate) fn on_pause_menu_part(
    activate: On<Activate>,
    kinds: Query<&PauseMenuPart>,
    parents: Query<&ChildOf>,
    mut screens: Query<(&UiScreenRoot, &mut PauseMenuScreen)>,
    mut stack: ResMut<ScreenStackRes>,
    mut out: MessageWriter<PauseMenuEvent>,
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
    let choice = match *kind {
        PauseMenuPart::Yes if screen.asking => {
            screen.answer();
            Some(PauseChoice::Quit)
        }
        PauseMenuPart::No if screen.asking => {
            screen.answer();
            None
        }
        // The question is modal.
        _ if screen.asking => None,
        PauseMenuPart::Entry(choice) => {
            if screen.model.is_enabled(choice) {
                screen.focus = PauseMenuFocus::Entry(choice);
            }
            screen.choose(choice)
        }
        PauseMenuPart::Yes | PauseMenuPart::No => None,
    };
    if let Some(choice) = choice {
        if choice == PauseChoice::Resume {
            stack.0.close(root.0.as_str());
        }
        out.write(PauseMenuEvent {
            screen: entity,
            choice,
        });
    }
}
