//! The widget kit the stock screens are built from (ui.md U3): plain `bevy_ui` nodes with the
//! headless `bevy_ui_widgets` behaviour, each part named and classed for theme sheets.
//!
//! Every part carries the same three things: a `Name` (`#sf-dialogue-topic` in CSS), a
//! [`UiPart`] (the same string, for queries) and a [`UiClass`] holding that name plus state
//! classes (`new`, `selected`, `focused`, `disabled`, `low`, ...). A theme styles
//! `.sf-dialogue-topic.new`; nothing here names a game. The fallback look ([`ScreenColors`])
//! keeps the screens readable without a theme; theme sheets override it.

use bevy_color::Color;
use bevy_ecs::prelude::*;
use bevy_input_focus::InputFocus;
use bevy_text::{FontSize, TextColor, TextFont};
use bevy_ui::prelude::*;
use bevy_ui::{ComputedNode, InteractionDisabled};
use bevy_ui_widgets::{Button, ScrollIntoView};
use sf_ui_bevy::{UiFocusable, UiPart};
use sf_ui_css::UiClass;

/// Name, part and class of a widget part (`name` is also its first class).
pub fn part(name: &'static str) -> (Name, UiPart, UiClass) {
    (Name::new(name), UiPart(name), UiClass::new(name))
}

/// Name and part of `name`, with a class list that starts with it ([`classes`]).
pub fn part_with(name: &'static str, class: UiClass) -> (Name, UiPart, UiClass) {
    (Name::new(name), UiPart(name), class)
}

/// The class list of a part: its name, then each state class that is on.
pub fn classes(name: &str, states: &[(&str, bool)]) -> UiClass {
    let mut out = name.to_owned();
    for (class, on) in states {
        if *on {
            out.push(' ');
            out.push_str(class);
        }
    }
    UiClass(out)
}

/// Fallback colours and sizes, used where no theme sheet says otherwise.
#[derive(Resource, Clone, Debug)]
pub struct ScreenColors {
    pub text: Color,
    /// Text of something not live (a locked topic, a disabled tab).
    pub muted: Color,
    /// Unexplored topics and links, "new" markers.
    pub new: Color,
    /// Explored links.
    pub link: Color,
    /// Links the speaker has nothing on.
    pub unavailable: Color,
    /// Behind the focused link and search hits.
    pub highlight: Color,
    pub panel: Color,
    pub cell: Color,
    pub bar_track: Color,
    pub bar_fill: Color,
    pub font_size: f32,
}

impl Default for ScreenColors {
    fn default() -> Self {
        Self {
            text: Color::srgb(0.93, 0.9, 0.82),
            muted: Color::srgb(0.55, 0.53, 0.5),
            new: Color::srgb(0.98, 0.82, 0.45),
            link: Color::srgb(0.62, 0.75, 0.95),
            unavailable: Color::srgb(0.5, 0.5, 0.58),
            highlight: Color::srgba(0.9, 0.75, 0.3, 0.35),
            panel: Color::srgba(0.06, 0.05, 0.04, 0.88),
            cell: Color::srgba(0.2, 0.18, 0.15, 0.9),
            bar_track: Color::srgba(0.0, 0.0, 0.0, 0.6),
            bar_fill: Color::srgb(0.75, 0.2, 0.15),
            font_size: 18.0,
        }
    }
}

impl ScreenColors {
    pub fn font(&self, scale: f32) -> TextFont {
        TextFont {
            font_size: FontSize::Px(self.font_size * scale),
            ..TextFont::default()
        }
    }
}

/// A text part in another colour than the default.
pub fn label_in(
    name: &'static str,
    text: impl Into<String>,
    color: Color,
    colors: &ScreenColors,
) -> impl Bundle {
    (
        part(name),
        Text::new(text),
        colors.font(1.0),
        TextColor(color),
    )
}

/// A text part.
pub fn label(name: &'static str, text: impl Into<String>, colors: &ScreenColors) -> impl Bundle {
    (
        part(name),
        Text::new(text),
        colors.font(1.0),
        TextColor(colors.text),
    )
}

/// A focusable button part with a text label (`Activate` on click, Enter, or the router's
/// confirm).
pub fn button(name: &'static str, text: impl Into<String>, colors: &ScreenColors) -> impl Bundle {
    (
        part(name),
        Node {
            padding: UiRect::axes(Val::Px(8.0), Val::Px(4.0)),
            ..Node::default()
        },
        Button,
        UiFocusable,
        children![(Text::new(text), colors.font(1.0), TextColor(colors.text))],
    )
}

/// [`button`] spawned under `parent`, returning the button and its text entity (to relabel
/// it in place).
pub fn spawn_button(
    commands: &mut Commands,
    parent: Option<Entity>,
    name: &'static str,
    text: impl Into<String>,
    colors: &ScreenColors,
) -> (Entity, Entity) {
    let button = commands
        .spawn((
            part(name),
            Node {
                padding: UiRect::axes(Val::Px(8.0), Val::Px(4.0)),
                flex_shrink: 0.0,
                ..Node::default()
            },
            Button,
            UiFocusable,
        ))
        .id();
    if let Some(parent) = parent {
        commands.entity(button).insert(ChildOf(parent));
    }
    let text = commands
        .spawn((
            Text::new(text),
            colors.font(1.0),
            TextColor(colors.text),
            ChildOf(button),
        ))
        .id();
    (button, text)
}

/// Sets the text and colour of `entity`'s first child (a button's label), in place.
pub fn relabel(commands: &mut Commands, entity: Entity, text: String, color: Color) {
    commands.queue(move |world: &mut World| {
        let Some(child) = world
            .get::<Children>(entity)
            .and_then(|c| c.first().copied())
        else {
            return;
        };
        if let Ok(mut e) = world.get_entity_mut(child) {
            e.insert((Text::new(text), TextColor(color)));
        }
    });
}

/// A panel part: `node` with the fallback background.
pub fn panel(name: &'static str, node: Node, colors: &ScreenColors) -> impl Bundle {
    (part(name), node, BackgroundColor(colors.panel))
}

/// Disabled state: `InteractionDisabled` (headless widgets ignore it) and the `disabled` class.
pub fn set_disabled(commands: &mut Commands, entity: Entity, disabled: bool) {
    if disabled {
        commands.entity(entity).insert(InteractionDisabled);
    } else {
        commands.entity(entity).remove::<InteractionDisabled>();
    }
}

/// Shows or hides a node (`display: none` keeps its entity and children). Writes (and so
/// triggers change detection and relayout) only when the value changes.
pub fn show(node: &mut Mut<Node>, visible: bool) {
    let display = if visible {
        Display::Flex
    } else {
        Display::None
    };
    if node.display != display {
        node.display = display;
    }
}

/// Sets `node`'s position (`left`, `top`) and size (`width`, `height`) fields that are `Some`,
/// writing only what changes.
pub fn place(
    node: &mut Mut<Node>,
    left: Option<Val>,
    top: Option<Val>,
    width: Option<Val>,
    height: Option<Val>,
) {
    if left.is_some_and(|v| node.left != v)
        || top.is_some_and(|v| node.top != v)
        || width.is_some_and(|v| node.width != v)
        || height.is_some_and(|v| node.height != v)
    {
        let node = &mut **node;
        node.left = left.unwrap_or(node.left);
        node.top = top.unwrap_or(node.top);
        node.width = width.unwrap_or(node.width);
        node.height = height.unwrap_or(node.height);
    }
}

/// A value bar (health, magicka, fatigue, encumbrance): a track with a fill child whose width
/// is `current / max`. Classes: `sf-bar`, the bar's part name, its `kind`, `low` under a
/// quarter, `over` above the maximum (encumbrance).
#[derive(Component, Clone, Debug, PartialEq)]
#[require(Node)]
pub struct StatBar {
    pub current: f32,
    pub max: f32,
    /// An extra class naming the value (`health`); may be empty.
    pub kind: String,
}

impl StatBar {
    /// Filled fraction, clamped to 0..=1 (0 when `max` is not positive).
    pub fn fraction(&self) -> f32 {
        if self.max > 0.0 {
            (self.current / self.max).clamp(0.0, 1.0)
        } else {
            0.0
        }
    }
}

/// The fill node of a [`StatBar`] (its first child).
#[derive(Component, Clone, Copy, Debug)]
pub struct StatBarFill;

/// Spawns a bar under `parent`; returns the bar entity.
pub fn spawn_bar(
    commands: &mut Commands,
    parent: Entity,
    name: &'static str,
    bar: StatBar,
    colors: &ScreenColors,
) -> Entity {
    commands
        .spawn((
            part(name),
            bar,
            Node {
                width: Val::Px(160.0),
                height: Val::Px(10.0),
                ..Node::default()
            },
            BackgroundColor(colors.bar_track),
            ChildOf(parent),
            children![(
                part("sf-bar-fill"),
                StatBarFill,
                Node {
                    width: Val::Percent(0.0),
                    height: Val::Percent(100.0),
                    ..Node::default()
                },
                BackgroundColor(colors.bar_fill),
            )],
        ))
        .id()
}

/// Bars → fill widths and state classes, on change only.
pub(crate) fn sync_bars(
    mut commands: Commands,
    bars: Query<(Entity, &StatBar, &UiPart, &Children), Changed<StatBar>>,
    mut fills: Query<&mut Node, With<StatBarFill>>,
) {
    for (entity, bar, part, children) in &bars {
        for child in children {
            if let Ok(mut node) = fills.get_mut(*child) {
                let width = Val::Percent(bar.fraction() * 100.0);
                if node.width != width {
                    node.width = width;
                }
            }
        }
        let ratio = if bar.max > 0.0 {
            bar.current / bar.max
        } else {
            0.0
        };
        commands.entity(entity).insert(classes(
            &format!("sf-bar {} {}", part.0, bar.kind),
            &[("low", ratio < 0.25), ("over", ratio > 1.0)],
        ));
    }
}

/// The fallback focus ring: an outline on the focused part (themes restyle `:focus`).
pub(crate) fn focus_ring(
    mut commands: Commands,
    focus: Res<InputFocus>,
    colors: Res<ScreenColors>,
    parts: Query<(), With<UiPart>>,
    mut ringed: Local<Option<Entity>>,
) {
    let now = focus.get().filter(|e| parts.contains(*e));
    if now == *ringed {
        return;
    }
    if let Some(old) = ringed.take()
        && let Ok(mut e) = commands.get_entity(old)
    {
        e.try_remove::<Outline>();
    }
    if let Some(new) = now {
        commands
            .entity(new)
            .insert(Outline::new(Val::Px(2.0), Val::Px(1.0), colors.new));
    }
    *ringed = now;
}

/// Keeps the focused part in view: a newly focused part that is laid out gets Bevy's
/// `ScrollIntoView`, which its nearest `ScrollArea` ancestor answers (the minimal scroll).
pub(crate) fn scroll_to_focus(
    mut commands: Commands,
    focus: Res<InputFocus>,
    nodes: Query<&ComputedNode, With<UiPart>>,
    mut done: Local<Option<Entity>>,
) {
    let Some(focused) = focus.get() else {
        *done = None;
        return;
    };
    if *done == Some(focused) {
        return;
    }
    // Not one of ours, or not laid out yet (retried next frame).
    let Ok(node) = nodes.get(focused) else {
        return;
    };
    if node.size().y > 0.0 {
        commands.trigger(ScrollIntoView { entity: focused });
        *done = Some(focused);
    }
}
