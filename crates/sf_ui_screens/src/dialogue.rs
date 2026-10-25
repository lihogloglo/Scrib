//! The dialogue window (ui.md "Dialogue window view-model", "Dialogue hyperlinks") over
//! `sf_rpg_ui::DialogueView`.
//!
//! The game spawns a screen root with a [`DialogueScreen`] holding the view, feeds the view
//! from its session (`apply_outputs`, `apply_events`, `refresh`) and applies the
//! [`DialogueRequestEvent`]s this screen sends. Parts, all keyed so a new line, a restyled link
//! or a topic that appears touches only its own entities:
//!
//! - the scrollback: one entry per `Entry` (topic heading, then one `Text` with a `TextSpan`
//!   per run; link spans are clickable and hoverable through span picking), the player's
//!   echoed choices and journal notices;
//! - the topic list (sorted rows, `new` and `disabled` classes) with Goodbye under it;
//! - the choices, numbered (keys 1-9 pick them).
//!
//! Focus is the view's: up/down move in a region, left enters the text's link cursor, right
//! returns to the topic list. The focused link gets a background (spans have no focus of
//! their own); everything else mirrors into `InputFocus`, so CSS `:focus` and confirm work.

use std::collections::HashMap;

use bevy_ecs::prelude::*;
use bevy_input::ButtonInput;
use bevy_input::keyboard::KeyCode;
use bevy_input_focus::{FocusCause, InputFocus};
use bevy_math::CompassOctant;
use bevy_picking::events::{PointerClick, PointerOver};
use bevy_text::{TextBackgroundColor, TextColor, TextLayout, TextSpan};
use bevy_ui::prelude::*;
use bevy_ui::{ComputedNode, ScrollPosition, UiGlobalTransform};
use bevy_ui_widgets::{Activate, ScrollArea};
use sf_rpg::RecordId;
use sf_rpg_ui::{
    DialogueFocus, DialogueNav, DialogueRequest, DialogueView, Entry, EntryId, EntryKind, LinkState,
};
use sf_ui_bevy::{
    ScreenStackRes, UiAction, UiActionEvent, UiHandlesBack, UiNavEvent, UiOwnsFocus, UiScreenRoot,
};

use crate::keyed::KeyedChildren;
use crate::labels::ScreenLabels;
use crate::widgets::{
    ScreenColors, classes, label, label_in, part, part_with, relabel, set_disabled, spawn_button,
};
use crate::{is_target, screen_of};

/// On a screen root: the dialogue window over its view.
#[derive(Component)]
#[require(Node, UiOwnsFocus, UiHandlesBack)]
pub struct DialogueScreen {
    pub view: DialogueView,
}

impl DialogueScreen {
    pub fn new(view: DialogueView) -> Self {
        Self { view }
    }
}

/// A request from the player for the host (apply it to the session, feed the view).
#[derive(Message, Clone, Debug, PartialEq, Eq)]
pub struct DialogueRequestEvent {
    /// The screen root it came from.
    pub screen: Entity,
    pub request: DialogueRequest,
}

/// What activating or hovering a part targets.
#[derive(Component, Clone, Debug, PartialEq, Eq)]
pub struct DialogueTarget(pub DialogueFocus);

/// The window's parts and what they show.
#[derive(Component, Debug)]
pub struct DialogueParts {
    speaker: Entity,
    log: Entity,
    entries: KeyedChildren<EntryId>,
    shown: HashMap<EntryId, EntryKind>,
    links: HashMap<(EntryId, usize), Entity>,
    topics: KeyedChildren<RecordId>,
    topic_shown: HashMap<RecordId, (u64, Entity)>,
    choices: KeyedChildren<(usize, String)>,
    goodbye: Entity,
    lit_link: Option<Entity>,
    seen: Option<u64>,
    scroll_to: Option<EntryId>,
}

impl DialogueParts {
    /// The scrollback entry entity of a view entry.
    pub fn entry(&self, id: EntryId) -> Option<Entity> {
        self.entries.get(&id)
    }

    /// Scrollback entries in order.
    pub fn entries(&self) -> impl Iterator<Item = (EntryId, Entity)> + '_ {
        self.entries.iter().map(|(k, e)| (*k, e))
    }

    /// The span of a link run.
    pub fn link(&self, entry: EntryId, run: usize) -> Option<Entity> {
        self.links.get(&(entry, run)).copied()
    }

    pub fn topic(&self, topic: &RecordId) -> Option<Entity> {
        self.topics.get(topic)
    }

    /// Topic rows in display order.
    pub fn topics(&self) -> impl Iterator<Item = (&RecordId, Entity)> {
        self.topics.iter()
    }

    pub fn choice(&self, index: usize) -> Option<Entity> {
        self.choices
            .iter()
            .find(|((i, _), _)| *i == index)
            .map(|(_, e)| e)
    }

    pub fn goodbye(&self) -> Entity {
        self.goodbye
    }

    pub fn log(&self) -> Entity {
        self.log
    }

    /// The part a focus shows on (`None` for a link: spans take no focus).
    fn focus_entity(&self, focus: &DialogueFocus) -> Option<Entity> {
        match focus {
            DialogueFocus::Topic(t) => self.topic(t),
            DialogueFocus::Choice(i) => self.choice(*i),
            DialogueFocus::Goodbye => Some(self.goodbye),
            DialogueFocus::Link { .. } => None,
        }
    }
}

#[allow(clippy::too_many_lines)]
pub(crate) fn spawn_dialogue(
    mut commands: Commands,
    colors: Res<ScreenColors>,
    labels: Res<ScreenLabels>,
    roots: Query<(Entity, &DialogueScreen), Added<DialogueScreen>>,
) {
    for (root, screen) in &roots {
        commands.entity(root).insert((
            part("sf-dialogue"),
            Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(12.0),
                padding: UiRect::all(Val::Px(12.0)),
                width: Val::Px(900.0),
                height: Val::Px(560.0),
                ..Node::default()
            },
            BackgroundColor(colors.panel),
        ));
        let main = commands
            .spawn((
                part("sf-dialogue-main"),
                Node {
                    flex_direction: FlexDirection::Column,
                    flex_grow: 1.0,
                    row_gap: Val::Px(8.0),
                    ..Node::default()
                },
                ChildOf(root),
            ))
            .id();
        let speaker = commands
            .spawn((
                label("sf-dialogue-speaker", screen.view.speaker(), &colors),
                ChildOf(main),
            ))
            .id();
        let log = commands
            .spawn((
                part("sf-dialogue-log"),
                Node {
                    flex_direction: FlexDirection::Column,
                    flex_grow: 1.0,
                    row_gap: Val::Px(10.0),
                    overflow: Overflow::scroll_y(),
                    ..Node::default()
                },
                ScrollArea,
                ChildOf(main),
            ))
            .id();
        let choices = commands
            .spawn((
                part("sf-dialogue-choices"),
                Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(4.0),
                    ..Node::default()
                },
                ChildOf(main),
            ))
            .id();
        let side = commands
            .spawn((
                part("sf-dialogue-side"),
                Node {
                    flex_direction: FlexDirection::Column,
                    width: Val::Px(220.0),
                    row_gap: Val::Px(8.0),
                    ..Node::default()
                },
                ChildOf(root),
            ))
            .id();
        let topics = commands
            .spawn((
                part("sf-dialogue-topics"),
                ScrollArea,
                Node {
                    flex_direction: FlexDirection::Column,
                    flex_grow: 1.0,
                    overflow: Overflow::scroll_y(),
                    ..Node::default()
                },
                ScrollPosition::default(),
                ChildOf(side),
            ))
            .id();
        let (goodbye, _) = spawn_button(
            &mut commands,
            Some(side),
            "sf-dialogue-goodbye",
            labels.goodbye.clone(),
            &colors,
        );
        commands
            .entity(goodbye)
            .insert(DialogueTarget(DialogueFocus::Goodbye));
        commands.entity(root).insert(DialogueParts {
            speaker,
            log,
            entries: KeyedChildren::new(log),
            shown: HashMap::new(),
            links: HashMap::new(),
            topics: KeyedChildren::new(topics),
            topic_shown: HashMap::new(),
            choices: KeyedChildren::new(choices),
            goodbye,
            lit_link: None,
            seen: None,
            scroll_to: None,
        });
    }
}

fn link_color(state: LinkState, colors: &ScreenColors) -> bevy_color::Color {
    match state {
        LinkState::New => colors.new,
        LinkState::Known => colors.link,
        LinkState::Unavailable => colors.unavailable,
    }
}

/// Spawns an entry's content under `parent` (an entry node), recording link spans.
fn spawn_entry_content(
    commands: &mut Commands,
    parent: Entity,
    entry: &Entry,
    colors: &ScreenColors,
    labels: &ScreenLabels,
    links: &mut HashMap<(EntryId, usize), Entity>,
) {
    match &entry.kind {
        EntryKind::Line(line) => {
            if let Some(heading) = &line.heading {
                commands.spawn((
                    label_in("sf-dialogue-heading", heading.clone(), colors.new, colors),
                    ChildOf(parent),
                ));
            }
            let text = commands
                .spawn((
                    part("sf-dialogue-text"),
                    Text::new(""),
                    TextLayout::default(),
                    colors.font(1.0),
                    TextColor(colors.text),
                    ChildOf(parent),
                ))
                .id();
            for (i, run) in line.runs.iter().enumerate() {
                let mut span = commands.spawn((
                    TextSpan::new(run.text.clone()),
                    colors.font(1.0),
                    ChildOf(text),
                ));
                match &run.link {
                    Some(topic_link) => {
                        let state = match topic_link.state {
                            LinkState::New => "new",
                            LinkState::Known => "known",
                            LinkState::Unavailable => "unavailable",
                        };
                        span.insert((
                            part_with(
                                "sf-dialogue-link",
                                classes("sf-dialogue-link", &[(state, true)]),
                            ),
                            TextColor(link_color(topic_link.state, colors)),
                            DialogueTarget(DialogueFocus::Link {
                                entry: entry.id,
                                run: i,
                            }),
                        ));
                        links.insert((entry.id, i), span.id());
                    }
                    None => {
                        span.insert(TextColor(colors.text));
                    }
                }
            }
        }
        EntryKind::Chosen { label: chosen } => {
            commands.spawn((
                label_in("sf-dialogue-chosen", chosen.clone(), colors.muted, colors),
                ChildOf(parent),
            ));
        }
        EntryKind::JournalUpdated { .. } => {
            commands.spawn((
                label_in(
                    "sf-dialogue-notice",
                    labels.journal_updated.clone(),
                    colors.new,
                    colors,
                ),
                ChildOf(parent),
            ));
        }
    }
}

fn entry_class(kind: &EntryKind) -> &'static str {
    match kind {
        EntryKind::Line(_) => "line",
        EntryKind::Chosen { .. } => "chosen",
        EntryKind::JournalUpdated { .. } => "notice",
    }
}

/// The view → the window, when its version changed: keyed entries, topics and choices, mode
/// classes, the lit link and `InputFocus`.
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub(crate) fn sync_dialogue(
    mut commands: Commands,
    colors: Res<ScreenColors>,
    labels: Res<ScreenLabels>,
    stack: Res<ScreenStackRes>,
    mut screens: Query<(
        Entity,
        &UiScreenRoot,
        &mut DialogueScreen,
        &mut DialogueParts,
    )>,
    mut focus: ResMut<InputFocus>,
) {
    for (root_entity, root, mut screen, mut parts) in &mut screens {
        // A window taken off the stack this frame (goodbye, Back) may be despawned by the game's
        // commands before ours apply: leave it alone so no command targets a despawned entity.
        if !stack.0.is_open(root.0.as_str()) {
            continue;
        }
        let parts = &mut *parts;
        let view = &mut screen.view;
        if let Some(entry) = view.take_scroll_request() {
            parts.scroll_to = Some(entry);
        }
        let version = view.version();
        if parts.seen != Some(version) {
            parts.seen = Some(version);
            let colors = &*colors;
            commands
                .entity(parts.speaker)
                .insert(Text::new(view.speaker()));
            commands
                .entity(root_entity)
                .insert(classes("sf-dialogue", &[(view.mode().as_str(), true)]));

            // Scrollback: new entries spawn, trimmed ones go, changed ones redraw inside.
            let ids: Vec<EntryId> = view.entries().map(|e| e.id).collect();
            parts
                .entries
                .sync(&mut commands, ids.iter().copied(), |commands, id| {
                    commands
                        .spawn((
                            part("sf-dialogue-entry"),
                            Node {
                                flex_direction: FlexDirection::Column,
                                ..Node::default()
                            },
                            DialogueEntry(*id),
                        ))
                        .id()
                });
            parts.shown.retain(|id, _| ids.contains(id));
            parts.links.retain(|(id, _), _| ids.contains(id));
            for entry in view.entries() {
                if parts.shown.get(&entry.id) == Some(&entry.kind) {
                    continue;
                }
                let Some(node) = parts.entries.get(&entry.id) else {
                    continue;
                };
                if parts.shown.contains_key(&entry.id) {
                    commands.entity(node).despawn_related::<Children>();
                    parts.links.retain(|(id, _), _| *id != entry.id);
                }
                commands.entity(node).insert(classes(
                    "sf-dialogue-entry",
                    &[(entry_class(&entry.kind), true)],
                ));
                spawn_entry_content(
                    &mut commands,
                    node,
                    entry,
                    colors,
                    &labels,
                    &mut parts.links,
                );
                parts.shown.insert(entry.id, entry.kind.clone());
            }

            // Topic list, sorted by the view; rows redraw on their row version.
            let order = view.topic_order().to_vec();
            parts
                .topics
                .sync(&mut commands, order.iter().cloned(), |commands, topic| {
                    let (row, _) = spawn_button(commands, None, "sf-dialogue-topic", "", colors);
                    commands
                        .entity(row)
                        .insert(DialogueTarget(DialogueFocus::Topic(topic.clone())));
                    row
                });
            parts.topic_shown.retain(|t, _| order.contains(t));
            for topic in &order {
                let (Some(row), Some(data), Some(row_version)) = (
                    parts.topics.get(topic),
                    view.topics().get(topic),
                    view.topics().row_version(topic),
                ) else {
                    continue;
                };
                if parts
                    .topic_shown
                    .get(topic)
                    .is_some_and(|(v, _)| *v == row_version)
                {
                    continue;
                }
                parts.topic_shown.insert(topic.clone(), (row_version, row));
                commands.entity(row).insert(classes(
                    "sf-dialogue-topic",
                    &[("new", data.new), ("disabled", !data.enabled)],
                ));
                set_disabled(&mut commands, row, !data.enabled);
                let color = if !data.enabled {
                    colors.muted
                } else if data.new {
                    colors.new
                } else {
                    colors.text
                };
                relabel(&mut commands, row, data.name.clone(), color);
            }

            // Choices, numbered.
            let choices: Vec<(usize, String)> = view
                .choices()
                .iter()
                .enumerate()
                .map(|(i, c)| (i, c.label.clone()))
                .collect();
            parts
                .choices
                .sync(&mut commands, choices, |commands, (i, text)| {
                    let (row, _) = spawn_button(
                        commands,
                        None,
                        "sf-dialogue-choice",
                        format!("{}. {text}", i + 1),
                        colors,
                    );
                    commands
                        .entity(row)
                        .insert(DialogueTarget(DialogueFocus::Choice(*i)));
                    row
                });
            set_disabled(&mut commands, parts.goodbye, !view.can_goodbye());
            commands.entity(parts.goodbye).insert(classes(
                "sf-dialogue-goodbye",
                &[("disabled", !view.can_goodbye())],
            ));
        }

        // The link cursor: a background on its span.
        let lit = match view.focus() {
            Some(DialogueFocus::Link { entry, run }) => parts.link(*entry, *run),
            _ => None,
        };
        if lit != parts.lit_link {
            if let Some(old) = parts.lit_link {
                commands.entity(old).try_remove::<TextBackgroundColor>();
            }
            if let Some(new) = lit {
                commands
                    .entity(new)
                    .insert(TextBackgroundColor(colors.highlight));
            }
            parts.lit_link = lit;
        }
        if is_target(&stack, root) {
            let wanted = view.focus().and_then(|f| parts.focus_entity(f));
            if focus.get() != wanted {
                match wanted {
                    Some(e) => focus.set(e, FocusCause::Navigated),
                    None => focus.clear(),
                }
            }
        }
    }
}

/// On a scrollback entry node.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct DialogueEntry(pub EntryId);

/// Scrolls the log to a requested entry once it is laid out: its top to the viewport top.
pub(crate) fn scroll_dialogue(
    mut screens: Query<&mut DialogueParts>,
    nodes: Query<(&ComputedNode, &UiGlobalTransform)>,
    mut scrolls: Query<&mut ScrollPosition>,
) {
    for mut parts in &mut screens {
        let Some(id) = parts.scroll_to else {
            continue;
        };
        let Some(entry) = parts.entry(id) else {
            parts.scroll_to = None;
            continue;
        };
        let (Ok((node, at)), Ok((log, log_at))) = (nodes.get(entry), nodes.get(parts.log)) else {
            continue;
        };
        if node.size().y <= 0.0 {
            continue; // not laid out yet
        }
        let top = at.translation.y - node.size().y / 2.0;
        let log_top = log_at.translation.y - log.size().y / 2.0;
        if let Ok(mut pos) = scrolls.get_mut(parts.log) {
            // bevy_ui clamps to the content during layout.
            pos.y = (pos.y + (top - log_top) * log.inverse_scale_factor()).max(0.0);
        }
        parts.scroll_to = None;
    }
}

/// Directions, back, link confirm and the number keys → the view; requests out.
pub(crate) fn dialogue_input(
    mut navs: MessageReader<UiNavEvent>,
    mut actions: MessageReader<UiActionEvent>,
    keys: Option<Res<ButtonInput<KeyCode>>>,
    stack: Res<ScreenStackRes>,
    mut screens: Query<(Entity, &UiScreenRoot, &mut DialogueScreen)>,
    mut out: MessageWriter<DialogueRequestEvent>,
) {
    let navs: Vec<UiNavEvent> = navs.read().cloned().collect();
    let actions: Vec<UiActionEvent> = actions.read().cloned().collect();
    for (entity, root, mut screen) in &mut screens {
        let view = &mut screen.view;
        let mut send = |request: Option<DialogueRequest>| {
            if let Some(request) = request {
                out.write(DialogueRequestEvent {
                    screen: entity,
                    request,
                });
            }
        };
        for nav in navs.iter().filter(|n| n.screen == root.0) {
            let nav = match nav.direction {
                CompassOctant::North => DialogueNav::Prev,
                CompassOctant::South => DialogueNav::Next,
                CompassOctant::West => DialogueNav::PrevRegion,
                CompassOctant::East => DialogueNav::NextRegion,
                _ => continue,
            };
            send(view.nav(nav));
        }
        for action in actions.iter().filter(|a| a.screen == root.0) {
            match action.action {
                UiAction::Back => send(view.nav(DialogueNav::Back)),
                // Other targets are buttons: the router's confirm activates them.
                UiAction::Confirm if matches!(view.focus(), Some(DialogueFocus::Link { .. })) => {
                    send(view.nav(DialogueNav::Confirm));
                }
                _ => {}
            }
        }
        // Numbered choices (BG3, Disco Elysium): 1-9 while choosing.
        if is_target(&stack, root)
            && !view.choices().is_empty()
            && let Some(keys) = &keys
        {
            const DIGITS: [KeyCode; 9] = [
                KeyCode::Digit1,
                KeyCode::Digit2,
                KeyCode::Digit3,
                KeyCode::Digit4,
                KeyCode::Digit5,
                KeyCode::Digit6,
                KeyCode::Digit7,
                KeyCode::Digit8,
                KeyCode::Digit9,
            ];
            for (i, key) in DIGITS.iter().enumerate() {
                if keys.just_pressed(*key) {
                    send(view.activate(DialogueFocus::Choice(i)));
                }
            }
        }
    }
}

/// A topic row, choice or goodbye activated (click, confirm).
pub(crate) fn on_dialogue_button(
    activate: On<Activate>,
    targets: Query<&DialogueTarget>,
    parents: Query<&ChildOf>,
    mut screens: Query<&mut DialogueScreen>,
    mut out: MessageWriter<DialogueRequestEvent>,
) {
    let Ok(DialogueTarget(target)) = targets.get(activate.entity) else {
        return;
    };
    activate_target(activate.entity, target, &parents, &mut screens, &mut out);
}

fn activate_target(
    entity: Entity,
    target: &DialogueFocus,
    parents: &Query<&ChildOf>,
    screens: &mut Query<&mut DialogueScreen>,
    out: &mut MessageWriter<DialogueRequestEvent>,
) {
    let Some(screen) = screen_of(entity, parents, |e| screens.contains(e)) else {
        return;
    };
    let Ok(mut s) = screens.get_mut(screen) else {
        return;
    };
    if let Some(request) = s.view.activate(target.clone()) {
        out.write(DialogueRequestEvent { screen, request });
    }
}

/// A click on a link span (span-level text picking).
pub(crate) fn on_link_click(
    mut click: On<PointerClick>,
    targets: Query<&DialogueTarget>,
    parents: Query<&ChildOf>,
    mut screens: Query<&mut DialogueScreen>,
    mut out: MessageWriter<DialogueRequestEvent>,
) {
    let Ok(DialogueTarget(target @ DialogueFocus::Link { .. })) = targets.get(click.entity) else {
        return;
    };
    click.propagate(false);
    activate_target(click.entity, target, &parents, &mut screens, &mut out);
}

/// Hover moves the view's focus (links and rows alike).
pub(crate) fn on_dialogue_hover(
    over: On<PointerOver>,
    targets: Query<&DialogueTarget>,
    parents: Query<&ChildOf>,
    mut screens: Query<&mut DialogueScreen>,
) {
    let Ok(DialogueTarget(target)) = targets.get(over.entity) else {
        return;
    };
    let Some(screen) = screen_of(over.entity, &parents, |e| screens.contains(e)) else {
        return;
    };
    if let Ok(mut s) = screens.get_mut(screen) {
        s.view.set_focus(target.clone());
    }
}
