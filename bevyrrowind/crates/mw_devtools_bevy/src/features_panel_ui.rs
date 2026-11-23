//! The Features panel's `bevy_ui` tree (slice FS-UI1): spawned when the panel opens, despawned
//! when it closes; the tab strip and the rows are respawned only when the registry, a switch's
//! state or the tab changes (a few dozen nodes), never per frame.
//!
//! Layout (top centre, dark translucent, like the F3 panel):
//! ```text
//! Features (Ctrl+F3)  ms: frame 9.8  main 6.1  render 4.0  gpu n/a   [reset all] [close]
//! last: sky.froxels.enabled: off (console; skips)
//! [Shadows] [Lighting] [Sky] [Water] ...          (only categories with switches)
//!                                                  frame change when flipped
//! +----------------------------------------------------------------------------------+
//! | [on ] Volumetric fog   sky.froxels.enabled   -1.3 ms r -0.9 g -0.4   [cost 1.4 ms (r 0.9, 1.1..1.7)] console |
//! | [off] Groundcover      groundcover.enabled   measuring...            [A/B] restart ...                   |
//! +----------------------------------------------------------------------------------+
//! writes the console layer only: nothing here is saved
//! ```
//! The live ms text (slice FS-UI2), each row's delta cell and its quick A/B button (slice FS-C1b)
//! are rewritten in place ([`set_delta`], [`set_ab`]); only a registry, state or tab change, or an
//! A/B starting or ending, respawns rows. While an A/B runs the other buttons are greyed.
//! Buttons are `bevy_ui_widgets::Button`s: a click (or the router's Confirm on the focused one)
//! sends `Activate`, which [`super::on_activate`] maps through [`PanelAction`].

use bevy_color::prelude::*;
use bevy_ecs::prelude::*;
use bevy_text::prelude::*;
use bevy_ui::prelude::*;
use bevy_ui_widgets::{Button, ScrollArea};
use sf_tunables_bevy::{Category, SetBy};

use crate::features_panel_model::{AbCell, DELTA_HEADING, DeltaCell, Row, tab_label};

// Layout (px at UI scale 1).
const PANEL_W: f32 = 1040.0; // tunables: const (layout)
const PANEL_TOP: f32 = 40.0; // tunables: const (layout)
const ROWS_MAX_H: f32 = 520.0; // tunables: const (layout)
const TOGGLE_W: f32 = 44.0; // tunables: const (layout)
const LABEL_W: f32 = 210.0; // tunables: const (layout)
const NAME_W: f32 = 230.0; // tunables: const (layout)
const DELTA_W: f32 = 170.0; // tunables: const (layout)
const GAP: f32 = 6.0; // tunables: const (layout)
const PAD: f32 = 8.0; // tunables: const (layout)
const TITLE_PX: f32 = 15.0; // tunables: const (layout)
const ROW_PX: f32 = 12.0; // tunables: const (layout)
const SMALL_PX: f32 = 10.0; // tunables: const (layout)
/// Above the F3 panel (1000).
const Z: i32 = 1001; // tunables: const (layout)

/// The panel's greys, as the F3 panel's; the state colours come from [`Look`].
const TEXT: Color = Color::srgb(0.88, 0.9, 0.92); // tunables: const (layout: panel greys)
const MUTED: Color = Color::srgb(0.55, 0.57, 0.6); // tunables: const (layout: panel greys)
const BUTTON: Color = Color::srgba(0.16, 0.17, 0.19, 0.95); // tunables: const (layout: panel greys)
const SELECTED: Color = Color::srgba(0.3, 0.32, 0.36, 0.95); // tunables: const (layout: panel greys)

/// What a panel button does when activated.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub enum PanelAction {
    /// Flip this switch at the console layer.
    Toggle(&'static str),
    /// Run a quick A/B of this switch ([`crate::switch_probe::QuickAb`]).
    Ab(&'static str),
    /// Show this category's rows.
    Tab(Category),
    /// Peel the console layer of every switch.
    ResetAll,
    Close,
}

/// Colours read from the F3 panel's tunables when the panel opens.
#[derive(Clone, Copy, Debug)]
pub(super) struct Look {
    pub alpha: f32,
    pub on: Color,
    pub off: Color,
    pub warn: Color,
}

/// The entities the panel rewrites.
#[derive(Clone, Copy, Debug)]
pub(super) struct PanelUi {
    pub root: Entity,
    pub note: Entity,
    pub tabs: Entity,
    pub rows: Entity,
    /// The header's live frame/main/render/GPU ms.
    pub live: Entity,
}

/// One row's delta texts, rewritten in place when a probe result lands.
#[derive(Clone, Copy, Debug)]
pub(super) struct DeltaTexts {
    pub name: &'static str,
    pub value: Entity,
    pub small: Entity,
    /// The A/B button.
    pub ab: Entity,
    pub can_ab: bool,
}

/// The value's colour: greyed under the noise floor.
fn delta_colour(cell: &DeltaCell) -> Color {
    if cell.noise { MUTED } else { TEXT }
}

/// Rewrites one row's delta texts from `cell`.
pub(super) fn set_delta(texts: &mut Query<(&mut Text, &mut TextColor)>, at: DeltaTexts, cell: &DeltaCell) {
    if let Ok((mut text, mut colour)) = texts.get_mut(at.value) {
        if text.0 != cell.value {
            text.0.clone_from(&cell.value);
        }
        let want = delta_colour(cell);
        if colour.0 != want {
            colour.0 = want;
        }
    }
    let small = cell.small();
    if let Ok((mut text, _)) = texts.get_mut(at.small)
        && text.0 != small
    {
        text.0 = small;
    }
}

/// The A/B button's colour: greyed when it can't start or under the noise floor.
fn ab_colour(cell: &AbCell) -> Color {
    if !cell.enabled || cell.noise { MUTED } else { TEXT }
}

/// Rewrites one row's A/B button from `cell`.
pub(super) fn set_ab(texts: &mut Query<(&mut Text, &mut TextColor)>, at: DeltaTexts, cell: &AbCell) {
    if let Ok((mut text, mut colour)) = texts.get_mut(at.ab) {
        if text.0 != cell.text {
            text.0.clone_from(&cell.text);
        }
        let want = ab_colour(cell);
        if colour.0 != want {
            colour.0 = want;
        }
    }
}

fn button(commands: &mut Commands, parent: Entity, text: &str, colour: Color, background: Color, action: PanelAction, width: Option<f32>) -> Entity {
    commands
        .spawn((
            Button,
            action,
            Text::new(text),
            TextFont::from_font_size(ROW_PX),
            TextColor(colour),
            Node {
                width: width.map_or(Val::Auto, px),
                padding: UiRect::axes(px(GAP), px(2.0)),
                justify_content: JustifyContent::Center,
                ..Node::default()
            },
            BackgroundColor(background),
            ChildOf(parent),
        ))
        .id()
}

fn text(commands: &mut Commands, parent: Entity, value: &str, size: f32, colour: Color, node: Node) -> Entity {
    commands
        .spawn((Text::new(value), TextFont::from_font_size(size), TextColor(colour), node, ChildOf(parent)))
        .id()
}

fn row_node() -> Node {
    Node {
        flex_direction: FlexDirection::Row,
        align_items: AlignItems::Center,
        column_gap: px(GAP),
        ..Node::default()
    }
}

/// Spawns the frame: header, note, an empty tab strip and an empty row list.
pub(super) fn spawn(commands: &mut Commands, look: Look) -> PanelUi {
    let root = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: px(PANEL_TOP),
                left: percent(50.0),
                margin: UiRect::left(px(-PANEL_W / 2.0)),
                width: px(PANEL_W),
                flex_direction: FlexDirection::Column,
                row_gap: px(GAP),
                padding: UiRect::all(px(PAD)),
                ..Node::default()
            },
            BackgroundColor(Color::srgba(0.02, 0.025, 0.03, look.alpha)),
            GlobalZIndex(Z),
        ))
        .id();
    let header = commands.spawn((row_node(), ChildOf(root))).id();
    text(commands, header, "Features (Ctrl+F3)", TITLE_PX, TEXT, Node::default());
    let live = text(commands, header, "ms: ...", SMALL_PX, TEXT, Node { flex_grow: 1.0, ..Node::default() });
    button(commands, header, "reset all", TEXT, BUTTON, PanelAction::ResetAll, None);
    button(commands, header, "close", TEXT, BUTTON, PanelAction::Close, None);
    let note = text(commands, root, "", SMALL_PX, MUTED, Node::default());
    let tabs = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                flex_wrap: FlexWrap::Wrap,
                column_gap: px(GAP / 2.0),
                row_gap: px(GAP / 2.0),
                ..Node::default()
            },
            ChildOf(root),
        ))
        .id();
    let heading = commands.spawn((row_node(), ChildOf(root))).id();
    text(commands, heading, "", SMALL_PX, MUTED, Node { width: px(TOGGLE_W + LABEL_W + NAME_W + 2.0 * GAP), ..Node::default() });
    text(commands, heading, DELTA_HEADING, SMALL_PX, MUTED, Node { width: px(DELTA_W), ..Node::default() });
    let rows = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                row_gap: px(GAP / 2.0),
                max_height: px(ROWS_MAX_H),
                overflow: Overflow::scroll_y(),
                ..Node::default()
            },
            ScrollArea,
            ChildOf(root),
        ))
        .id();
    text(commands, root, "writes the console layer only: nothing here is saved", SMALL_PX, MUTED, Node::default());
    PanelUi { root, note, tabs, rows, live }
}

/// Respawns the tab strip, `selected` highlighted.
pub(super) fn fill_tabs(commands: &mut Commands, ui: &PanelUi, tabs: &[Category], selected: Option<Category>) {
    commands.entity(ui.tabs).despawn_children();
    for &category in tabs {
        let background = if Some(category) == selected { SELECTED } else { BUTTON };
        button(commands, ui.tabs, &tab_label(category), TEXT, background, PanelAction::Tab(category), None);
    }
}

/// Respawns the rows: toggle, label, name, flip delta, layer when not the default, badges.
/// `busy`: an A/B runs, the toggles are greyed. Returns each row's delta texts and A/B button for
/// [`set_delta`] and [`set_ab`].
pub(super) fn fill_rows(
    commands: &mut Commands,
    ui: &PanelUi,
    rows: &[Row],
    look: Look,
    busy: bool,
    ab: impl Fn(&Row) -> AbCell,
) -> Vec<DeltaTexts> {
    commands.entity(ui.rows).despawn_children();
    if rows.is_empty() {
        text(commands, ui.rows, "no feature switches registered", ROW_PX, MUTED, Node::default());
        return Vec::new();
    }
    let mut cells = Vec::with_capacity(rows.len());
    for row in rows {
        let line = commands.spawn((row_node(), ChildOf(ui.rows))).id();
        let state = if !row.can_toggle() || busy {
            MUTED
        } else if row.on {
            look.on
        } else {
            look.off
        };
        button(commands, line, row.state_word(), state, BUTTON, PanelAction::Toggle(row.name), Some(TOGGLE_W));
        let label_colour = if row.running { TEXT } else { MUTED };
        text(commands, line, row.label, ROW_PX, label_colour, Node { width: px(LABEL_W), ..Node::default() });
        text(commands, line, row.name, SMALL_PX, MUTED, Node { width: px(NAME_W), ..Node::default() });
        let cell = commands.spawn((Node { width: px(DELTA_W), ..row_node() }, ChildOf(line))).id();
        let value = text(commands, cell, &row.delta.value, ROW_PX, delta_colour(&row.delta), Node::default());
        let small = text(commands, cell, &row.delta.small(), SMALL_PX, MUTED, Node::default());
        let ab_cell = ab(row);
        let ab = button(commands, line, &ab_cell.text, ab_colour(&ab_cell), BUTTON, PanelAction::Ab(row.name), None);
        cells.push(DeltaTexts { name: row.name, value, small, ab, can_ab: row.can_ab() });
        if row.layer != SetBy::Default {
            text(commands, line, row.layer.describe(), SMALL_PX, TEXT, Node::default());
        }
        for badge in &row.badges {
            let colour = if badge.is_warning() { look.warn } else { MUTED };
            commands.spawn((
                Text::new(badge.text()),
                TextFont::from_font_size(SMALL_PX),
                TextColor(colour),
                Node { padding: UiRect::axes(px(GAP / 2.0), px(1.0)), ..Node::default() },
                BackgroundColor(BUTTON),
                ChildOf(line),
            ));
        }
    }
    cells
}
