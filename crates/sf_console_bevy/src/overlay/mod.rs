//! The in-game console front end (feature `overlay`), on `bevy_ui`.
//!
//! - Toggle with [`ConsoleKeys`] (backtick or F1 by default, rebindable). While open,
//!   [`ConsoleOpen`] is true: game systems gate on [`crate::console_closed`], and the
//!   overlay clears `ButtonInput` and mouse motion so ungated systems see nothing.
//! - Input line ([`Prompt`]): editing, Up/Down history, Tab completion with a popup of the
//!   candidates, Enter submits as a typed [`ConsoleSubmit`] (runs the same frame),
//!   PageUp/PageDown or the wheel scroll the output, Escape clears the line then closes.
//! - Click an object to select it ([`select`]), boxed by gizmos; the [`inspect`] panel shows
//!   it; commands (`prid`, `getpos`, `setpos`, `moveto`, `disable`, `enable`, `delete`,
//!   `find`) act on it.
//!
//! Systems: input in `PreUpdate` ([`ConsoleInputSystems`], after `bevy_input`, before the
//! console runs lines); drawing in `PostUpdate` before `bevy_ui` lays out.

use std::fmt::Write as _;

use bevy_app::{App, Plugin, PostUpdate, PreUpdate, Startup};
use bevy_color::Color;
use bevy_ecs::prelude::*;
use bevy_input::keyboard::{Key, KeyCode, KeyboardInput};
use bevy_input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll, MouseButton, MouseWheel};
use bevy_input::{ButtonInput, ButtonState, InputSystems};
use bevy_text::{TextColor, TextFont, TextSpan};
use bevy_ui::widget::Text;
use bevy_ui::{
    BackgroundColor, ComputedNode, Display, GlobalZIndex, Node, Overflow, PositionType,
    UiGlobalTransform, UiRect, UiSystems, Val,
};
use bevy_window::{CursorGrabMode, CursorOptions, PrimaryWindow};
use sf_console::{Edit, Level, Prompt};

use crate::{ConsoleOpen, ConsoleSubmit, ConsoleSystems, DevConsole};

pub mod inspect;
pub mod select;

#[cfg(test)]
mod tests;

pub use inspect::{ConsoleOverlayAppExt, InspectorRows, inspect};
pub use select::{ConsoleCamera, ConsoleMoveTo, ConsolePick, ConsoleSelection, PickRoot};

/// The console overlay: front end, click-select, inspector, selection commands.
/// Needs [`crate::ConsolePlugin`]; the app supplies `bevy_ui`, input, windows and gizmos.
pub struct ConsoleOverlayPlugin;

impl Plugin for ConsoleOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ConsoleKeys>()
            .init_resource::<ConsoleStyle>()
            .init_resource::<ConsoleOverlay>()
            .init_resource::<ConsoleOpen>()
            .configure_sets(
                PreUpdate,
                ConsoleInputSystems
                    .after(InputSystems)
                    .before(ConsoleSystems),
            )
            .add_systems(Startup, spawn_overlay)
            .add_systems(
                PreUpdate,
                (
                    handle_keys,
                    select::click_to_pick,
                    block_game_input.run_if(|o: Res<ConsoleOpen>| o.0),
                )
                    .chain()
                    .in_set(ConsoleInputSystems),
            )
            .add_systems(
                PostUpdate,
                (release_cursor, draw_overlay)
                    .chain()
                    .before(UiSystems::Prepare),
            );
        select::build(app);
        inspect::build(app);
    }
}

/// Where the overlay reads keys and mouse clicks, in `PreUpdate` after `bevy_input` and before
/// [`ConsoleSystems`]. Order other input readers (e.g. `bevy_enhanced_input`'s update) after it,
/// so they see the cleared `ButtonInput` while the console is open.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ConsoleInputSystems;

/// Keys that open and close the console (physical keys, so layouts don't move them).
#[derive(Resource, Clone, Debug, PartialEq, Eq)]
pub struct ConsoleKeys {
    pub toggle: Vec<KeyCode>,
}

impl Default for ConsoleKeys {
    fn default() -> Self {
        Self {
            toggle: vec![KeyCode::Backquote, KeyCode::F1],
        }
    }
}

/// Look and behavior of the overlay. Change before `Startup` (sizes are read at spawn).
#[derive(Resource, Clone, Debug)]
pub struct ConsoleStyle {
    /// Set a monospace face here if the app has one.
    pub font: TextFont,
    /// Height of the console panel, percent of the window.
    pub height_percent: f32,
    /// Output lines kept on screen (older ones scroll).
    pub lines: usize,
    /// Candidates shown in the completion popup.
    pub popup_lines: usize,
    pub background: Color,
    /// Text colors for [`Level::Echo`], `Info`, `Warn`, `Error`.
    pub level_colors: [Color; 4],
    /// Clear `ButtonInput<KeyCode>`, `ButtonInput<MouseButton>` and mouse motion and scroll
    /// while open, for game systems that don't gate on [`crate::console_closed`].
    pub block_game_input: bool,
    /// Show and free the cursor while open, restore it on close.
    pub release_cursor: bool,
}

impl Default for ConsoleStyle {
    fn default() -> Self {
        Self {
            font: TextFont::from_font_size(14.0),
            height_percent: 45.0,
            lines: 40,
            popup_lines: 8,
            background: Color::srgba(0.04, 0.05, 0.07, 0.92),
            level_colors: [
                Color::srgb(0.55, 0.75, 1.0),
                Color::srgb(0.88, 0.88, 0.88),
                Color::srgb(1.0, 0.8, 0.35),
                Color::srgb(1.0, 0.45, 0.4),
            ],
            block_game_input: true,
            release_cursor: true,
        }
    }
}

/// Front end state: the input line and the scroll position (lines up from the newest).
#[derive(Resource, Debug, Default)]
pub struct ConsoleOverlay {
    pub prompt: Prompt,
    pub scroll: usize,
    /// The cursor as the game had it before the console opened.
    saved_cursor: Option<(bool, CursorGrabMode)>,
    /// What the scrollback shows (next log seq, lines kept, scroll): redraw when it changes.
    drawn: Option<(u64, usize, usize)>,
}

/// Overlay nodes, found by marker.
#[derive(Component)]
struct OverlayRoot;
#[derive(Component)]
struct Scrollback;
#[derive(Component)]
struct InputLine;
#[derive(Component)]
struct Popup;
/// A panel that takes clicks: no click-select through it.
#[derive(Component)]
pub(crate) struct BlocksPick;

fn spawn_overlay(mut commands: Commands, style: Res<ConsoleStyle>) {
    let font = style.font.clone();
    let text_color = TextColor(style.level_colors[1]);
    let panel_height = Val::Percent(style.height_percent);
    commands
        .spawn((
            OverlayRoot,
            Node {
                display: Display::None,
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Node::default()
            },
            GlobalZIndex(i32::MAX - 1),
        ))
        .with_children(|root| {
            root.spawn((
                BlocksPick,
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: panel_height,
                    flex_direction: bevy_ui::FlexDirection::Column,
                    justify_content: bevy_ui::JustifyContent::FlexEnd,
                    padding: UiRect::all(Val::Px(6.0)),
                    overflow: Overflow::clip(),
                    ..Node::default()
                },
                BackgroundColor(style.background),
            ))
            .with_children(|panel| {
                panel
                    .spawn((Scrollback, Text::default(), font.clone(), text_color))
                    .with_children(|text| {
                        for _ in 0..style.lines {
                            text.spawn((TextSpan::default(), font.clone(), text_color));
                        }
                    });
                panel.spawn((
                    InputLine,
                    Text::new("> "),
                    font.clone(),
                    TextColor(Color::WHITE),
                ));
            });
            root.spawn((
                Popup,
                BlocksPick,
                Node {
                    display: Display::None,
                    position_type: PositionType::Absolute,
                    top: panel_height,
                    left: Val::Px(6.0),
                    padding: UiRect::all(Val::Px(4.0)),
                    ..Node::default()
                },
                BackgroundColor(style.background),
                Text::default(),
                font.clone(),
                text_color,
            ));
            inspect::spawn_panel(root, &style, panel_height);
        });
}

/// Toggle key, line editing, scrolling. Submitted lines go out before [`ConsoleSystems`]
/// runs them, so a typed switch applies to this frame's `Update`.
#[allow(clippy::too_many_arguments)]
fn handle_keys(
    mut keys: MessageReader<KeyboardInput>,
    mut wheel: MessageReader<MouseWheel>,
    bindings: Res<ConsoleKeys>,
    style: Res<ConsoleStyle>,
    mut open: ResMut<ConsoleOpen>,
    mut overlay: ResMut<ConsoleOverlay>,
    mut console: ResMut<DevConsole>,
    mut submit: MessageWriter<ConsoleSubmit>,
    mut ctrl: Local<[bool; 2]>,
) {
    let overlay = &mut *overlay;
    for key in keys.read() {
        let pressed = key.state == ButtonState::Pressed;
        // Tracked here: `ButtonInput` is cleared while the console is open.
        match key.key_code {
            KeyCode::ControlLeft => ctrl[0] = pressed,
            KeyCode::ControlRight => ctrl[1] = pressed,
            _ => {}
        }
        if !pressed {
            continue;
        }
        if bindings.toggle.contains(&key.key_code) {
            if !key.repeat {
                open.0 = !open.0;
            }
            continue;
        }
        if !open.0 {
            continue;
        }
        let ctrl = ctrl[0] || ctrl[1];
        let page = style.lines.saturating_sub(2).max(1);
        let edit = match (&key.logical_key, key.key_code) {
            (Key::Enter, _) => Edit::Submit,
            (Key::Tab, _) => Edit::Complete,
            (Key::ArrowUp, _) => Edit::Older,
            (Key::ArrowDown, _) => Edit::Newer,
            (Key::ArrowLeft, _) if ctrl => Edit::WordLeft,
            (Key::ArrowLeft, _) => Edit::Left,
            (Key::ArrowRight, _) if ctrl => Edit::WordRight,
            (Key::ArrowRight, _) => Edit::Right,
            (Key::Home, _) => Edit::Home,
            (Key::End, _) => Edit::End,
            (Key::Backspace, _) if ctrl => Edit::DeleteWordBack,
            (Key::Backspace, _) => Edit::Backspace,
            (Key::Delete, _) => Edit::Delete,
            (_, KeyCode::KeyW) if ctrl => Edit::DeleteWordBack,
            (_, KeyCode::KeyL) if ctrl => {
                console.log_mut().clear();
                continue;
            }
            (Key::Escape, _) if overlay.prompt.text().is_empty() => {
                open.0 = false;
                continue;
            }
            (Key::Escape, _) => Edit::Clear,
            (Key::PageUp, _) => {
                overlay.scroll += page;
                continue;
            }
            (Key::PageDown, _) => {
                overlay.scroll = overlay.scroll.saturating_sub(page);
                continue;
            }
            _ if ctrl => continue,
            _ => match &key.text {
                Some(text) => Edit::Insert(text.to_string()),
                None => continue,
            },
        };
        if let Some(line) = overlay.prompt.apply(edit, &mut console) {
            overlay.scroll = 0;
            submit.write(ConsoleSubmit::typed(line));
        }
    }
    let lines: f32 = wheel.read().map(|w| w.y.signum()).sum();
    if open.0 && lines != 0.0 {
        // Wheel up shows older lines.
        #[allow(clippy::cast_possible_truncation)]
        let step = (lines * 3.0) as isize;
        overlay.scroll = overlay.scroll.saturating_add_signed(step);
    }
    let max = console.log().lines().len().saturating_sub(1);
    if overlay.scroll > max {
        overlay.scroll = max;
    }
}

/// While open, game systems that read `ButtonInput` or mouse motion directly see nothing.
fn block_game_input(
    style: Res<ConsoleStyle>,
    keys: Option<ResMut<ButtonInput<KeyCode>>>,
    buttons: Option<ResMut<ButtonInput<MouseButton>>>,
    motion: Option<ResMut<AccumulatedMouseMotion>>,
    scroll: Option<ResMut<AccumulatedMouseScroll>>,
) {
    if !style.block_game_input {
        return;
    }
    if let Some(mut k) = keys {
        k.reset_all();
    }
    if let Some(mut b) = buttons {
        b.reset_all();
    }
    if let Some(mut m) = motion {
        m.delta = bevy_math::Vec2::ZERO;
    }
    if let Some(mut s) = scroll {
        s.delta = bevy_math::Vec2::ZERO;
    }
}

/// Frees the cursor while open (click-select needs it), restores the game's on close.
fn release_cursor(
    open: Res<ConsoleOpen>,
    style: Res<ConsoleStyle>,
    mut overlay: ResMut<ConsoleOverlay>,
    mut cursors: Query<&mut CursorOptions, With<PrimaryWindow>>,
) {
    if !open.is_changed() || !style.release_cursor {
        return;
    }
    let Ok(mut cursor) = cursors.single_mut() else {
        return;
    };
    if open.0 {
        overlay.saved_cursor = Some((cursor.visible, cursor.grab_mode));
        cursor.visible = true;
        cursor.grab_mode = CursorGrabMode::None;
    } else if let Some((visible, grab_mode)) = overlay.saved_cursor.take() {
        cursor.visible = visible;
        cursor.grab_mode = grab_mode;
    }
}

/// Shows or hides the overlay and redraws what changed: scrollback, input line, popup.
#[allow(clippy::too_many_arguments)]
fn draw_overlay(
    open: Res<ConsoleOpen>,
    style: Res<ConsoleStyle>,
    console: Res<DevConsole>,
    mut overlay: ResMut<ConsoleOverlay>,
    mut roots: Query<&mut Node, (With<OverlayRoot>, Without<Popup>)>,
    scrollback: Query<&Children, With<Scrollback>>,
    mut spans: Query<(&mut TextSpan, &mut TextColor)>,
    mut input: Query<&mut Text, (With<InputLine>, Without<Popup>)>,
    mut popup: Query<PopupItem, (With<Popup>, Without<OverlayRoot>)>,
) {
    if open.is_changed() {
        for mut node in &mut roots {
            node.display = if open.0 { Display::Flex } else { Display::None };
        }
    }
    if !open.0 {
        return;
    }
    let log = console.log();
    // The line count catches `clear` (Ctrl+L), which doesn't advance the sequence.
    let key = (log.next_seq(), log.lines().len(), overlay.scroll);
    if overlay.drawn != Some(key) {
        overlay.drawn = Some(key);
        let lines: Vec<_> = log.lines().collect();
        let end = lines.len().saturating_sub(overlay.scroll);
        let shown = &lines[end.saturating_sub(style.lines)..end];
        for children in &scrollback {
            // Newest at the bottom: the last span holds the newest shown line.
            let pad = children.len().saturating_sub(shown.len());
            for (i, child) in children.iter().enumerate() {
                let Ok((mut span, mut color)) = spans.get_mut(child) else {
                    continue;
                };
                let text = match i.checked_sub(pad).and_then(|k| shown.get(k)) {
                    Some(line) => {
                        color.0 = style.level_colors[level_index(line.level)];
                        let prefix = if line.level == Level::Echo { "> " } else { "" };
                        let newline = if i + 1 < children.len() { "\n" } else { "" };
                        format!("{prefix}{}{newline}", line.text)
                    }
                    None => String::new(),
                };
                if span.0 != text {
                    span.0 = text;
                }
            }
        }
    }
    if overlay.is_changed() || console.is_changed() {
        let prompt = &overlay.prompt;
        let (head, tail) = prompt.text().split_at(prompt.cursor());
        let scrolled = if overlay.scroll > 0 {
            format!("  [+{} older]", overlay.scroll)
        } else {
            String::new()
        };
        let line = format!("> {head}|{tail}{scrolled}");
        for mut text in &mut input {
            if text.0 != line {
                text.0.clone_from(&line);
            }
        }
        let completion = prompt.completion(&console.0);
        let typed = &prompt.text()[completion.start.min(prompt.cursor())..prompt.cursor()];
        let show = !prompt.text().is_empty()
            && match completion.candidates.as_slice() {
                [] => false,
                [only] => only != typed,
                _ => true,
            };
        for (mut node, mut text) in &mut popup {
            node.display = if show { Display::Flex } else { Display::None };
            if show {
                let n = completion.candidates.len();
                let mut s = completion
                    .candidates
                    .iter()
                    .take(style.popup_lines)
                    .cloned()
                    .collect::<Vec<_>>()
                    .join("\n");
                if n > style.popup_lines {
                    let _ = write!(s, "\n(+{} more, Tab extends)", n - style.popup_lines);
                }
                text.0 = s;
            }
        }
    }
}

type PopupItem<'a> = (&'a mut Node, &'a mut Text);

fn level_index(level: Level) -> usize {
    match level {
        Level::Echo => 0,
        Level::Info => 1,
        Level::Warn => 2,
        Level::Error => 3,
    }
}

/// Whether a physical-pixel point is over a panel that blocks click-select.
pub(crate) fn over_panel(
    point: bevy_math::Vec2,
    panels: &Query<(&ComputedNode, &UiGlobalTransform, &Node), With<BlocksPick>>,
) -> bool {
    panels.iter().any(|(computed, transform, node)| {
        node.display != Display::None && computed.contains_point(*transform, point)
    })
}
