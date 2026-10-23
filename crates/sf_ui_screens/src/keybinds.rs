//! The keybinds screen (ui.md "Keybinds screen (U6)") over the player's keymap
//! (`sf_ui_bevy::UiKeymap`) and the capture flow (`sf_ui_bevy::UiRebinder`).
//!
//! A row per action of the context (label, one cell per keyboard/mouse slot, one per gamepad
//! slot, reset), keyed by action; a footer with "reset all". Confirm on a slot captures the
//! next input (overlay); a conflict asks swap / unbind / keep both / cancel; Context1 clears a
//! slot. Glyphs follow the last-used pad family. Focus is the screen's own (a grid of
//! cells), so it moves predictably by row and column with the d-pad, stick or arrows.

use std::collections::HashMap;

use bevy_ecs::prelude::*;
use bevy_input_focus::{FocusCause, InputFocus};
use bevy_math::CompassOctant;
use bevy_text::TextColor;
use bevy_ui::ScrollPosition;
use bevy_ui::prelude::*;
use bevy_ui_widgets::Activate;
use sf_ui::{ActionRef, Device, KeybindRow, PadFamily, RebindEvent, Resolution, SlotRef};
use sf_ui_bevy::{
    GamepadFamily, ScreenStackRes, UiAction, UiActionEvent, UiHandlesBack, UiKeymap, UiLocalizer,
    UiNavEvent, UiOwnsFocus, UiRebindEvent, UiRebinder, UiScreenRoot,
};

use crate::keyed::KeyedChildren;
use crate::labels::ScreenLabels;
use crate::widgets::{ScreenColors, ScrollFollowsFocus, button, classes, label, panel, part, show};
use crate::{is_target, screen_of};

/// On a screen root: the keybinds screen for one keymap context (`None`: every action).
#[derive(Component, Clone, Debug, Default)]
#[require(Node, UiOwnsFocus, UiHandlesBack)]
pub struct KeybindsScreen {
    pub context: Option<String>,
    focus: KeybindsFocus,
    /// Where focus returns after the conflict question.
    resume: KeybindsFocus,
}

impl KeybindsScreen {
    pub fn new(context: Option<&str>) -> Self {
        Self {
            context: context.map(str::to_owned),
            focus: KeybindsFocus::default(),
            resume: KeybindsFocus::default(),
        }
    }

    pub fn focus(&self) -> KeybindsFocus {
        self.focus
    }
}

/// Where the screen's focus is.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum KeybindsFocus {
    /// Cell `col` of row `row` (slots, then reset); the footer is the row past the last.
    #[default]
    Start,
    Cell {
        row: usize,
        col: usize,
    },
    /// A button of the conflict question (swap, unbind, keep both, cancel).
    Choice(usize),
}

/// What a cell does.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum KeybindCellKind {
    Slot(SlotRef),
    Reset(ActionRef),
    ResetAll,
    Choice(Option<Resolution>),
}

/// A focusable cell of the keybinds screen.
#[derive(Component, Clone, Debug)]
pub struct KeybindCell(pub KeybindCellKind);

/// Fallback geometry (themes may restyle; columns line up as long as cells share a width).
const LABEL_WIDTH: f32 = 200.0;
const CELL_WIDTH: f32 = 140.0;
const CELL_GAP: f32 = 8.0;

/// Width of `count` cells and the gaps between them.
#[allow(clippy::cast_precision_loss)] // a handful of columns
fn columns_width(count: usize) -> f32 {
    count as f32 * CELL_WIDTH + count.saturating_sub(1) as f32 * CELL_GAP
}

const CHOICES: [Option<Resolution>; 4] = [
    Some(Resolution::Swap),
    Some(Resolution::Unbind),
    Some(Resolution::KeepBoth),
    None,
];

/// A drawn row: what it shows (row, pad family, column layout) and its cells by column.
type RowShown = (KeybindRow, PadFamily, [usize; 2], Vec<Option<Entity>>);

/// The screen's parts and what they show.
#[derive(Component, Debug)]
pub struct KeybindsParts {
    rows: KeyedChildren<ActionRef>,
    /// What each row shows, and its cells by column (`None`: the row has no such slot).
    shown: HashMap<ActionRef, RowShown>,
    /// Slot columns per device: the most slots any row has, so columns line up.
    columns: [usize; 2],
    /// Column headings per device, sized to their columns.
    headings: [Entity; 2],
    order: Vec<ActionRef>,
    reset_all: Entity,
    overlay: Entity,
    overlay_text: Entity,
    choices: Entity,
    choice_buttons: Vec<Entity>,
    status: Entity,
    seen: Option<(u64, PadFamily)>,
}

impl KeybindsParts {
    /// The row entity of an action.
    pub fn row(&self, action: &ActionRef) -> Option<Entity> {
        self.rows.get(action)
    }

    /// The cells of an action's row by column: keyboard/mouse slots, gamepad slots, reset.
    pub fn cells(&self, action: &ActionRef) -> &[Option<Entity>] {
        self.shown
            .get(action)
            .map_or(&[], |(_, _, _, c)| c.as_slice())
    }

    /// The cell of one binding slot.
    pub fn slot_cell(&self, action: &ActionRef, device: Device, slot: usize) -> Option<Entity> {
        let col = match device {
            Device::Kbm => slot,
            Device::Gamepad => self.columns[0] + slot,
        };
        self.cells(action).get(col).copied().flatten()
    }

    pub fn reset_all(&self) -> Entity {
        self.reset_all
    }

    pub fn overlay(&self) -> Entity {
        self.overlay
    }

    pub fn choice_buttons(&self) -> &[Entity] {
        &self.choice_buttons
    }

    /// The cell at a column, or the nearest one the row has (columns line up across rows;
    /// a row without that slot lends its neighbour).
    fn cell_at(&self, row: usize, col: usize) -> Option<Entity> {
        if row == self.order.len() {
            return Some(self.reset_all);
        }
        let cells = self.cells(self.order.get(row)?);
        (0..cells.len())
            .filter_map(|c| cells[c].map(|e| (c.abs_diff(col), e)))
            .min_by_key(|(d, _)| *d)
            .map(|(_, e)| e)
    }

    /// The column of the cell [`Self::cell_at`] picks.
    fn resolve_col(&self, row: usize, col: usize) -> usize {
        let Some(e) = self.cell_at(row, col) else {
            return col;
        };
        self.order
            .get(row)
            .and_then(|a| self.cells(a).iter().position(|c| *c == Some(e)))
            .unwrap_or(0)
    }

    /// The next column with a cell from `col` in direction `dx`, if any.
    fn next_col(&self, row: usize, col: usize, dx: isize) -> usize {
        let Some(action) = self.order.get(row) else {
            return 0;
        };
        let cells = self.cells(action);
        let mut c = self.resolve_col(row, col);
        loop {
            match c.checked_add_signed(dx) {
                Some(n) if n < cells.len() => {
                    c = n;
                    if cells[c].is_some() {
                        return c;
                    }
                }
                _ => return self.resolve_col(row, col),
            }
        }
    }
}

#[allow(clippy::too_many_lines)]
pub(crate) fn spawn_keybinds(
    mut commands: Commands,
    colors: Res<ScreenColors>,
    labels: Res<ScreenLabels>,
    roots: Query<Entity, Added<KeybindsScreen>>,
) {
    for root in &roots {
        commands.entity(root).insert((panel(
            "sf-keybinds",
            Node {
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(16.0)),
                row_gap: Val::Px(6.0),
                min_width: Val::Px(640.0),
                ..Node::default()
            },
            &colors,
        ),));
        let header = commands
            .spawn((
                part("sf-keybinds-header"),
                Node {
                    column_gap: Val::Px(CELL_GAP),
                    ..Node::default()
                },
                ChildOf(root),
            ))
            .id();
        commands.spawn((
            part("sf-keybinds-header-label"),
            Node {
                width: Val::Px(LABEL_WIDTH),
                ..Node::default()
            },
            ChildOf(header),
        ));
        let headings = [labels.kbm.clone(), labels.gamepad.clone()].map(|text| {
            commands
                .spawn((
                    label("sf-keybinds-heading", text, &colors),
                    Node::default(),
                    ChildOf(header),
                ))
                .id()
        });
        let rows = commands
            .spawn((
                part("sf-keybinds-rows"),
                Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(2.0),
                    max_height: Val::Vh(70.0),
                    overflow: Overflow::scroll_y(),
                    ..Node::default()
                },
                ScrollPosition::default(),
                ScrollFollowsFocus,
                ChildOf(root),
            ))
            .id();
        let reset_all = commands
            .spawn((
                button("sf-keybinds-reset-all", labels.reset_all.clone(), &colors),
                KeybindCell(KeybindCellKind::ResetAll),
                ChildOf(root),
            ))
            .id();
        let status = commands
            .spawn((label("sf-keybinds-status", "", &colors), ChildOf(root)))
            .id();
        let overlay = commands
            .spawn((
                panel(
                    "sf-keybinds-overlay",
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(20.0),
                        top: Val::Percent(35.0),
                        padding: UiRect::all(Val::Px(16.0)),
                        row_gap: Val::Px(8.0),
                        flex_direction: FlexDirection::Column,
                        display: Display::None,
                        ..Node::default()
                    },
                    &colors,
                ),
                ZIndex(10),
                ChildOf(root),
            ))
            .id();
        let overlay_text = commands
            .spawn((
                label("sf-keybinds-overlay-text", "", &colors),
                ChildOf(overlay),
            ))
            .id();
        let choices = commands
            .spawn((
                part("sf-keybinds-choices"),
                Node {
                    column_gap: Val::Px(8.0),
                    ..Node::default()
                },
                ChildOf(overlay),
            ))
            .id();
        let choice_buttons = CHOICES
            .iter()
            .map(|choice| {
                let text = match choice {
                    Some(Resolution::Swap) => &labels.swap,
                    Some(Resolution::Unbind) => &labels.unbind,
                    Some(Resolution::KeepBoth) => &labels.keep_both,
                    None => &labels.cancel,
                };
                commands
                    .spawn((
                        button("sf-keybinds-choice", text.clone(), &colors),
                        KeybindCell(KeybindCellKind::Choice(*choice)),
                        ChildOf(choices),
                    ))
                    .id()
            })
            .collect();
        commands.entity(root).insert(KeybindsParts {
            rows: KeyedChildren::new(rows),
            shown: HashMap::new(),
            columns: [0, 0],
            headings,
            order: Vec::new(),
            reset_all,
            overlay,
            overlay_text,
            choices,
            choice_buttons,
            status,
            seen: None,
        });
    }
}

/// An action's display name: its Fluent label, else the action name.
fn action_label(row: &KeybindRow, localizer: Option<&UiLocalizer>) -> String {
    match localizer {
        Some(l) if !row.label.is_empty() && l.0.has(&row.label) => {
            l.0.format(&row.label, None).into_owned()
        }
        _ => row.action.action.clone(),
    }
}

/// What a slot shows: the input's glyph label, or the unbound mark.
fn slot_text(input: Option<&String>, family: PadFamily, labels: &ScreenLabels) -> String {
    input.map_or_else(
        || labels.unbound.clone(),
        |name| sf_ui::glyph(name, family).map_or_else(|| name.clone(), |g| g.label),
    )
}

/// Keymap, pad family and capture state → rows, overlay and focus.
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub(crate) fn sync_keybinds(
    mut commands: Commands,
    keymap: Res<UiKeymap>,
    family: Res<GamepadFamily>,
    rebinder: Res<UiRebinder>,
    stack: Res<ScreenStackRes>,
    colors: Res<ScreenColors>,
    labels: Res<ScreenLabels>,
    localizer: Option<Res<UiLocalizer>>,
    mut screens: Query<(&UiScreenRoot, &mut KeybindsScreen, &mut KeybindsParts)>,
    mut nodes: Query<&mut Node>,
    mut focus: ResMut<InputFocus>,
) {
    for (root, mut screen, mut parts) in &mut screens {
        let parts = &mut *parts;
        let key = (keymap.0.version(), family.0);
        if parts.seen != Some(key) {
            parts.seen = Some(key);
            let rows = keymap.0.rows(screen.context.as_deref());
            parts.order = rows.iter().map(|r| r.action.clone()).collect();
            parts.columns =
                [0, 1].map(|d| rows.iter().map(|r| r.slots[d].len()).max().unwrap_or(0));
            let columns = parts.columns;
            for (heading, count) in parts.headings.iter().zip(columns) {
                if let Ok(mut node) = nodes.get_mut(*heading) {
                    node.width = Val::Px(columns_width(count));
                }
            }
            let colors = &*colors;
            parts
                .rows
                .sync(&mut commands, parts.order.iter().cloned(), |commands, _| {
                    commands
                        .spawn((
                            part("sf-keybinds-row"),
                            Node {
                                column_gap: Val::Px(CELL_GAP),
                                align_items: AlignItems::Center,
                                ..Node::default()
                            },
                        ))
                        .id()
                });
            parts.shown.retain(|a, _| parts.order.contains(a));
            for row in rows {
                let Some(entity) = parts.rows.get(&row.action) else {
                    continue;
                };
                if parts
                    .shown
                    .get(&row.action)
                    .is_some_and(|(r, f, c, _)| *r == row && *f == family.0 && *c == columns)
                {
                    continue;
                }
                // Changed row: redraw its content in place (the row entity stays).
                commands.entity(entity).despawn_related::<Children>();
                commands.entity(entity).insert(classes(
                    "sf-keybinds-row",
                    &[("conflicted", row.conflicted), ("modified", row.modified)],
                ));
                commands.spawn((
                    label(
                        "sf-keybinds-label",
                        action_label(&row, localizer.as_deref()),
                        colors,
                    ),
                    Node {
                        width: Val::Px(LABEL_WIDTH),
                        ..Node::default()
                    },
                    ChildOf(entity),
                ));
                let mut cells = Vec::new();
                for device in [Device::Kbm, Device::Gamepad] {
                    let slots = &row.slots[device_index(device)];
                    for slot in 0..columns[device_index(device)] {
                        let Some(input) = slots.get(slot) else {
                            // A slot other rows have: a spacer keeps the columns aligned.
                            commands.spawn((
                                part("sf-keybinds-spacer"),
                                Node {
                                    width: Val::Px(CELL_WIDTH),
                                    ..Node::default()
                                },
                                ChildOf(entity),
                            ));
                            cells.push(None);
                            continue;
                        };
                        let text = slot_text(input.as_ref(), family.0, &labels);
                        let cell = commands
                            .spawn((
                                button("sf-keybinds-slot", text, colors),
                                KeybindCell(KeybindCellKind::Slot(SlotRef {
                                    action: row.action.clone(),
                                    device,
                                    slot,
                                })),
                                ChildOf(entity),
                            ))
                            .insert(Node {
                                width: Val::Px(CELL_WIDTH),
                                padding: UiRect::axes(Val::Px(8.0), Val::Px(4.0)),
                                ..Node::default()
                            })
                            .insert(classes(
                                "sf-keybinds-slot",
                                &[(device.name(), true), ("unbound", input.is_none())],
                            ))
                            .id();
                        cells.push(Some(cell));
                    }
                }
                cells.push(Some(
                    commands
                        .spawn((
                            button("sf-keybinds-reset", labels.reset.clone(), colors),
                            KeybindCell(KeybindCellKind::Reset(row.action.clone())),
                            ChildOf(entity),
                        ))
                        .id(),
                ));
                parts
                    .shown
                    .insert(row.action.clone(), (row, family.0, columns, cells));
            }
        }

        // Overlay: capturing, or a conflict waiting for an answer.
        let capturing = rebinder.0.capturing().is_some();
        let pending = rebinder.0.pending();
        if let Ok(mut node) = nodes.get_mut(parts.overlay) {
            show(&mut node, capturing || pending.is_some());
        }
        if let Ok(mut node) = nodes.get_mut(parts.choices) {
            show(&mut node, pending.is_some());
        }
        let overlay_text = if let Some(p) = pending {
            let other = p
                .conflicts
                .first()
                .map(|c| {
                    let row = KeybindRow {
                        action: c.other.clone(),
                        label: keymap
                            .0
                            .def(&c.other)
                            .map(|d| d.label.clone())
                            .unwrap_or_default(),
                        slots: [Vec::new(), Vec::new()],
                        conflicted: false,
                        modified: false,
                    };
                    action_label(&row, localizer.as_deref())
                })
                .unwrap_or_default();
            labels
                .conflict
                .replace("{input}", &slot_text(Some(&p.input), family.0, &labels))
                .replace("{action}", &other)
        } else {
            labels.press_input.clone()
        };
        commands
            .entity(parts.overlay_text)
            .insert((Text::new(overlay_text), TextColor(colors.text)));

        // Focus: a conflict question takes it; otherwise a cell, clamped to the grid.
        if pending.is_some() {
            if !matches!(screen.focus, KeybindsFocus::Choice(_)) {
                screen.resume = screen.focus;
                screen.focus = KeybindsFocus::Choice(0);
            }
        } else if let KeybindsFocus::Choice(_) = screen.focus {
            screen.focus = screen.resume;
        }
        if screen.focus == KeybindsFocus::Start {
            screen.focus = KeybindsFocus::Cell { row: 0, col: 0 };
        }
        if let KeybindsFocus::Cell { row, col } = screen.focus {
            screen.focus = KeybindsFocus::Cell {
                row: row.min(parts.order.len()),
                col,
            };
        }
        if !is_target(&stack, root) {
            continue;
        }
        let target = match screen.focus {
            KeybindsFocus::Choice(i) => parts.choice_buttons.get(i).copied(),
            KeybindsFocus::Cell { row, col } => parts.cell_at(row, col),
            KeybindsFocus::Start => None,
        };
        if let Some(target) = target
            && focus.get() != Some(target)
        {
            focus.set(target, FocusCause::Navigated);
        }
    }
}

/// Index of a device in [`KeybindRow::slots`].
fn device_index(device: Device) -> usize {
    match device {
        Device::Kbm => 0,
        Device::Gamepad => 1,
    }
}

/// Directions move the focus; Back cancels a capture question or closes; Context1 clears the
/// focused slot. Rebind events set the status line.
#[allow(clippy::too_many_arguments)]
pub(crate) fn keybinds_input(
    mut navs: MessageReader<UiNavEvent>,
    mut actions: MessageReader<UiActionEvent>,
    mut rebinds: MessageReader<UiRebindEvent>,
    mut stack: ResMut<ScreenStackRes>,
    mut keymap: ResMut<UiKeymap>,
    mut rebinder: ResMut<UiRebinder>,
    mut screens: Query<(&UiScreenRoot, &mut KeybindsScreen, &KeybindsParts)>,
    cells: Query<&KeybindCell>,
    mut commands: Commands,
) {
    let navs: Vec<UiNavEvent> = navs.read().cloned().collect();
    let actions: Vec<UiActionEvent> = actions.read().cloned().collect();
    let rebinds: Vec<UiRebindEvent> = rebinds.read().cloned().collect();
    for (root, mut screen, parts) in &mut screens {
        for nav in navs.iter().filter(|n| n.screen == root.0) {
            screen.focus = step(screen.focus, nav.direction, parts);
        }
        for action in actions.iter().filter(|a| a.screen == root.0) {
            match action.action {
                UiAction::Back => {
                    if rebinder.0.pending().is_some() {
                        rebinder.0.cancel();
                    } else {
                        stack.0.close(root.0.as_str());
                    }
                }
                UiAction::Context1 => {
                    if let KeybindsFocus::Cell { row, col } = screen.focus
                        && let Some(cell) = parts.cell_at(row, col)
                        && let Ok(KeybindCell(KeybindCellKind::Slot(slot))) = cells.get(cell)
                        && let Err(e) = keymap.0.unbind(slot)
                    {
                        commands
                            .entity(parts.status)
                            .insert(Text::new(e.to_string()));
                    }
                }
                _ => {}
            }
        }
        for event in &rebinds {
            let text = match &event.0 {
                RebindEvent::Refused { error, .. } => error.to_string(),
                RebindEvent::Applied { .. }
                | RebindEvent::Cancelled { .. }
                | RebindEvent::Conflict(_) => String::new(),
            };
            commands.entity(parts.status).insert(Text::new(text));
        }
    }
}

/// One focus step on the grid (clamped, no wrap) or along the conflict choices.
fn step(focus: KeybindsFocus, direction: CompassOctant, parts: &KeybindsParts) -> KeybindsFocus {
    let (dx, dy): (isize, isize) = match direction {
        CompassOctant::North => (0, -1),
        CompassOctant::South => (0, 1),
        CompassOctant::East => (1, 0),
        CompassOctant::West => (-1, 0),
        _ => (0, 0),
    };
    match focus {
        KeybindsFocus::Choice(i) => {
            KeybindsFocus::Choice(i.saturating_add_signed(dx).min(CHOICES.len() - 1))
        }
        KeybindsFocus::Start => KeybindsFocus::Cell { row: 0, col: 0 },
        // Up/down keep the column (rows lacking it lend the nearest cell); left/right skip
        // to the next cell the row has.
        KeybindsFocus::Cell { row, col } if dx == 0 => KeybindsFocus::Cell {
            row: row.saturating_add_signed(dy).min(parts.order.len()),
            col,
        },
        KeybindsFocus::Cell { row, col } => KeybindsFocus::Cell {
            row,
            col: parts.next_col(row, col, dx),
        },
    }
}

/// A cell activated (confirm, click): start a capture, reset, or answer the conflict.
pub(crate) fn on_keybind_cell(
    activate: On<Activate>,
    cells: Query<&KeybindCell>,
    parents: Query<&ChildOf>,
    mut screens: Query<(&mut KeybindsScreen, &KeybindsParts)>,
    mut keymap: ResMut<UiKeymap>,
    mut rebinder: ResMut<UiRebinder>,
) {
    let Ok(KeybindCell(kind)) = cells.get(activate.entity) else {
        return;
    };
    let Some(screen) = screen_of(activate.entity, &parents, |e| screens.contains(e)) else {
        return;
    };
    let Ok((mut screen, parts)) = screens.get_mut(screen) else {
        return;
    };
    // Pointer activation moves the focus there too.
    for (row, action) in parts.order.iter().enumerate() {
        if let Some(col) = parts
            .cells(action)
            .iter()
            .position(|c| *c == Some(activate.entity))
        {
            screen.focus = KeybindsFocus::Cell { row, col };
        }
    }
    match kind {
        KeybindCellKind::Slot(slot) => {
            let _ = rebinder.start(&keymap, slot.clone());
        }
        KeybindCellKind::Reset(action) => {
            keymap.0.reset(action, None);
        }
        KeybindCellKind::ResetAll => {
            keymap.0.reset_all(None);
        }
        KeybindCellKind::Choice(choice) => match choice {
            Some(resolution) => {
                rebinder.resolve(&mut keymap, *resolution);
            }
            None => {
                rebinder.0.cancel();
            }
        },
    }
}
