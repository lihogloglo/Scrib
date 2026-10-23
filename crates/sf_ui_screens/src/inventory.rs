//! The inventory screen (ui.md U5) over `sf_ui_inventory`: tabs, a virtual icon grid
//! ([`sf_ui_inventory::InventoryGrid`]), drag and drop with a focus equivalent, the split
//! dialog, tooltips with compare-with-equipped, and the encumbrance bar.
//!
//! The grid keeps a pool of row widgets (a few more than fit the viewport) and rebinds a cell
//! only when its stack, that stack's row version or its focus/held state changes, so a
//! thousand stacks cost a few dozen nodes and scrolling touches only the rows that enter.
//!
//! The game owns the loadout: this screen sends [`InventoryEvent`]s (use, split, merge, focus)
//! and the game answers with commands to its host, a re-sync of the view, the split picker
//! ([`InventoryScreen::open_split`]) or the comparison ([`InventoryScreen::compare`]).
//!
//! Input: the d-pad moves in the grid, LB/RB switch tabs, A uses, X opens the split dialog,
//! Y picks a stack up and A on another stack drops it there (merge), B cancels, then closes.
//! With a mouse: hover focuses, click uses, drag a stack onto another to merge, the wheel
//! scrolls.

use bevy_asset::AssetServer;
use bevy_ecs::prelude::*;
use bevy_input::mouse::{MouseScrollUnit, MouseWheel};
use bevy_input_focus::{FocusCause, InputFocus};
use bevy_math::{CompassOctant, Vec2};
use bevy_picking::Pickable;
use bevy_picking::events::{
    PointerDrag, PointerDragDrop, PointerDragEnd, PointerDragStart, PointerOver,
};
use bevy_text::{TextColor, TextSpan};
use bevy_ui::ComputedNode;
use bevy_ui::prelude::*;
use bevy_ui_widgets::Activate;
use bevy_ui_widgets::popover::{Popover, PopoverAlign, PopoverPlacement, PopoverSide};
use sf_rpg::{RecordId, StackId};
use sf_ui_bevy::{
    RichImages, ScreenStackRes, UiAction, UiActionEvent, UiHandlesBack, UiNavEvent, UiOwnsFocus,
    UiScreenRoot, WHEEL_LINE,
};
use sf_ui_inventory::{Comparison, InventoryGrid, InventoryView, QuantityPicker, Verdict};

use crate::keyed::KeyedChildren;
use crate::labels::ScreenLabels;
use crate::widgets::{
    ScreenColors, StatBar, classes, label, panel, part, relabel, show, spawn_bar, spawn_button,
};
use crate::{is_target, screen_of};

/// Grid cell geometry in logical pixels (virtualization needs it up front, as the reader's
/// fonts: a theme sets it here, CSS styles the rest).
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct InventoryGridStyle {
    pub cell: Vec2,
    pub gap: f32,
}

impl Default for InventoryGridStyle {
    fn default() -> Self {
        Self {
            cell: Vec2::new(64.0, 64.0),
            gap: 4.0,
        }
    }
}

/// What the player asked for.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InventoryIntent {
    /// The focused stack changed (fill [`InventoryScreen::compare`]).
    Focused(Option<StackId>),
    /// Use, equip or read it (the game's call).
    Use(StackId),
    /// Open the split dialog: answer with [`InventoryScreen::open_split`].
    SplitRequested(StackId),
    /// The split dialog was confirmed.
    Split { stack: StackId, count: u32 },
    /// Dropped onto another stack (drag, or pick up and place); `into: None` is the "merge"
    /// action into the first target.
    Merge {
        from: StackId,
        into: Option<StackId>,
    },
}

#[derive(Message, Clone, Debug, PartialEq, Eq)]
pub struct InventoryEvent {
    pub screen: Entity,
    pub intent: InventoryIntent,
}

/// On a screen root: the inventory screen over one holder's view.
#[derive(Component)]
#[require(Node, UiOwnsFocus, UiHandlesBack)]
pub struct InventoryScreen {
    pub view: InventoryView,
    pub grid: InventoryGrid,
    /// Compare-with-equipped of the focused stack, filled by the game on
    /// [`InventoryIntent::Focused`].
    pub compare: Option<Comparison>,
    /// Display names of stats (the catalog's `StatDef` labels, localized).
    pub stat_labels: Vec<(RecordId, String)>,
    split: Option<(StackId, QuantityPicker)>,
    held: Option<StackId>,
}

impl InventoryScreen {
    pub fn new(view: InventoryView) -> Self {
        Self {
            view,
            grid: InventoryGrid::new(1, 1.0, 1.0),
            compare: None,
            stat_labels: Vec::new(),
            split: None,
            held: None,
        }
    }

    #[must_use]
    pub fn with_stat_labels(mut self, labels: Vec<(RecordId, String)>) -> Self {
        self.stat_labels = labels;
        self
    }

    /// Opens the split dialog for `stack` (the game's answer to
    /// [`InventoryIntent::SplitRequested`], from `InventoryView::split_picker`).
    pub fn open_split(&mut self, stack: StackId, picker: QuantityPicker) {
        self.split = Some((stack, picker));
    }

    pub fn split(&self) -> Option<&(StackId, QuantityPicker)> {
        self.split.as_ref()
    }

    /// The stack picked up for a move (pad) or being dragged (mouse).
    pub fn held(&self) -> Option<StackId> {
        self.held
    }
}

/// A grid cell (pooled): the stack it shows now.
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct InventoryCell(pub Option<StackId>);

/// A button of the inventory screen and what it does.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub enum InventoryButton {
    Tab(usize),
    SplitLess,
    SplitMore,
    SplitOk,
    SplitCancel,
}

#[derive(Debug)]
struct CellSlot {
    entity: Entity,
    icon: Entity,
    name: Entity,
    count: Entity,
    /// (stack, row version, focused, held) bound now.
    bound: Option<(StackId, u64, bool, bool)>,
}

#[derive(Debug)]
struct RowSlot {
    entity: Entity,
    cells: Vec<CellSlot>,
}

/// The screen's parts and pools.
#[derive(Component, Debug)]
pub struct InventoryParts {
    tabs: KeyedChildren<usize>,
    tabs_seen: Option<(u64, usize)>,
    viewport: Entity,
    rows: Vec<RowSlot>,
    thumb: Entity,
    weight_bar: Entity,
    weight_text: Entity,
    value_text: Entity,
    totals_seen: Option<(u64, u64)>,
    tooltip: Entity,
    tooltip_text: Entity,
    tooltip_seen: Option<(Entity, StackId, u64, Option<Comparison>)>,
    hovered: Option<Entity>,
    split: Entity,
    split_text: Entity,
    split_bar: Entity,
    split_ok: Entity,
    ghost: Entity,
    last_focus: Option<Option<StackId>>,
}

impl InventoryParts {
    /// Row widgets in the pool (spawned so far).
    pub fn pool_len(&self) -> usize {
        self.rows.len()
    }

    /// Every cell widget in the pool.
    pub fn cells(&self) -> impl Iterator<Item = Entity> + '_ {
        self.rows
            .iter()
            .flat_map(|r| r.cells.iter().map(|c| c.entity))
    }

    /// The cell showing `stack`, if it is in the window.
    pub fn cell_of(&self, stack: StackId) -> Option<Entity> {
        self.rows
            .iter()
            .flat_map(|r| &r.cells)
            .find(|c| c.bound.is_some_and(|b| b.0 == stack))
            .map(|c| c.entity)
    }

    pub fn tab(&self, index: usize) -> Option<Entity> {
        self.tabs.get(&index)
    }

    pub fn tooltip(&self) -> Entity {
        self.tooltip
    }

    pub fn split_dialog(&self) -> Entity {
        self.split
    }

    pub fn viewport(&self) -> Entity {
        self.viewport
    }
}

#[allow(clippy::too_many_lines)]
pub(crate) fn spawn_inventory(
    mut commands: Commands,
    colors: Res<ScreenColors>,
    labels: Res<ScreenLabels>,
    roots: Query<Entity, Added<InventoryScreen>>,
) {
    for root in &roots {
        commands.entity(root).insert((panel(
            "sf-inventory",
            Node {
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(12.0)),
                row_gap: Val::Px(8.0),
                width: Val::Px(560.0),
                height: Val::Px(520.0),
                ..Node::default()
            },
            &colors,
        ),));
        let tabs = commands
            .spawn((
                part("sf-inventory-tabs"),
                Node {
                    column_gap: Val::Px(6.0),
                    ..Node::default()
                },
                ChildOf(root),
            ))
            .id();
        let body = commands
            .spawn((
                part("sf-inventory-body"),
                Node {
                    flex_grow: 1.0,
                    column_gap: Val::Px(4.0),
                    ..Node::default()
                },
                ChildOf(root),
            ))
            .id();
        let viewport = commands
            .spawn((
                part("sf-inventory-grid"),
                Node {
                    flex_grow: 1.0,
                    overflow: Overflow::clip(),
                    ..Node::default()
                },
                ChildOf(body),
            ))
            .id();
        let track = commands
            .spawn((
                part("sf-inventory-scrollbar"),
                Node {
                    width: Val::Px(6.0),
                    ..Node::default()
                },
                BackgroundColor(colors.bar_track),
                ChildOf(body),
            ))
            .id();
        let thumb = commands
            .spawn((
                part("sf-inventory-scrollbar-thumb"),
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    ..Node::default()
                },
                BackgroundColor(colors.text),
                ChildOf(track),
            ))
            .id();
        let footer = commands
            .spawn((
                part("sf-inventory-footer"),
                Node {
                    column_gap: Val::Px(12.0),
                    align_items: AlignItems::Center,
                    ..Node::default()
                },
                ChildOf(root),
            ))
            .id();
        let weight_text = commands
            .spawn((label("sf-inventory-weight", "", &colors), ChildOf(footer)))
            .id();
        let weight_bar = spawn_bar(
            &mut commands,
            footer,
            "sf-inventory-encumbrance",
            StatBar {
                current: 0.0,
                max: 0.0,
                kind: String::new(),
            },
            &colors,
        );
        let value_text = commands
            .spawn((label("sf-inventory-value", "", &colors), ChildOf(footer)))
            .id();
        let tooltip = commands
            .spawn((
                panel(
                    "sf-tooltip",
                    Node {
                        position_type: PositionType::Absolute,
                        display: Display::None,
                        min_width: Val::Px(180.0),
                        padding: UiRect::all(Val::Px(8.0)),
                        ..Node::default()
                    },
                    &colors,
                ),
                Popover {
                    positions: vec![
                        PopoverPlacement {
                            side: PopoverSide::Right,
                            align: PopoverAlign::Start,
                            gap: 4.0,
                        },
                        PopoverPlacement {
                            side: PopoverSide::Left,
                            align: PopoverAlign::Start,
                            gap: 4.0,
                        },
                    ],
                    window_margin: 8.0,
                },
                GlobalZIndex(20),
                Pickable::IGNORE,
                ChildOf(root),
            ))
            .id();
        let tooltip_text = commands
            .spawn((
                part("sf-tooltip-text"),
                Text::new(""),
                colors.font(0.9),
                TextColor(colors.text),
                ChildOf(tooltip),
            ))
            .id();
        let split = commands
            .spawn((
                panel(
                    "sf-inventory-split",
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(25.0),
                        top: Val::Percent(35.0),
                        width: Val::Percent(50.0),
                        padding: UiRect::all(Val::Px(12.0)),
                        row_gap: Val::Px(8.0),
                        flex_direction: FlexDirection::Column,
                        display: Display::None,
                        ..Node::default()
                    },
                    &colors,
                ),
                GlobalZIndex(15),
                ChildOf(root),
            ))
            .id();
        commands.spawn((
            label("sf-inventory-split-title", labels.split.clone(), &colors),
            ChildOf(split),
        ));
        let split_text = commands
            .spawn((
                label("sf-inventory-split-count", "", &colors),
                ChildOf(split),
            ))
            .id();
        let split_bar = spawn_bar(
            &mut commands,
            split,
            "sf-inventory-split-bar",
            StatBar {
                current: 0.0,
                max: 1.0,
                kind: String::new(),
            },
            &colors,
        );
        let buttons = commands
            .spawn((
                part("sf-inventory-split-buttons"),
                Node {
                    column_gap: Val::Px(6.0),
                    ..Node::default()
                },
                ChildOf(split),
            ))
            .id();
        let mut split_ok = Entity::PLACEHOLDER;
        for (name, text, b) in [
            (
                "sf-inventory-split-less",
                "-".to_owned(),
                InventoryButton::SplitLess,
            ),
            (
                "sf-inventory-split-more",
                "+".to_owned(),
                InventoryButton::SplitMore,
            ),
            (
                "sf-inventory-split-ok",
                labels.ok.clone(),
                InventoryButton::SplitOk,
            ),
            (
                "sf-inventory-split-cancel",
                labels.cancel.clone(),
                InventoryButton::SplitCancel,
            ),
        ] {
            let (e, _) = spawn_button(&mut commands, Some(buttons), name, text, &colors);
            commands.entity(e).insert(b);
            if b == InventoryButton::SplitOk {
                split_ok = e;
            }
        }
        let ghost = commands
            .spawn((
                part("sf-inventory-drag"),
                Node {
                    position_type: PositionType::Absolute,
                    display: Display::None,
                    padding: UiRect::all(Val::Px(4.0)),
                    ..Node::default()
                },
                BackgroundColor(colors.cell),
                GlobalZIndex(30),
                Pickable::IGNORE,
                children![(Text::new(""), colors.font(0.9), TextColor(colors.text))],
            ))
            .id();
        commands.entity(root).insert(InventoryParts {
            tabs: KeyedChildren::new(tabs),
            tabs_seen: None,
            viewport,
            rows: Vec::new(),
            thumb,
            weight_bar,
            weight_text,
            value_text,
            totals_seen: None,
            tooltip,
            tooltip_text,
            tooltip_seen: None,
            hovered: None,
            split,
            split_text,
            split_bar,
            split_ok,
            ghost,
            last_focus: None,
        });
    }
}

/// Spawns a pooled row widget under the viewport.
fn spawn_row(
    commands: &mut Commands,
    viewport: Entity,
    columns: usize,
    style: &InventoryGridStyle,
    colors: &ScreenColors,
) -> RowSlot {
    let entity = commands
        .spawn((
            part("sf-inventory-row"),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                column_gap: Val::Px(style.gap),
                ..Node::default()
            },
            ChildOf(viewport),
        ))
        .id();
    let mut row = RowSlot {
        entity,
        cells: Vec::new(),
    };
    fit_cells(commands, &mut row, columns, style, colors);
    row
}

/// Grows or shrinks a row widget to `columns` cells.
fn fit_cells(
    commands: &mut Commands,
    row: &mut RowSlot,
    columns: usize,
    style: &InventoryGridStyle,
    colors: &ScreenColors,
) {
    while row.cells.len() > columns {
        if let Some(cell) = row.cells.pop() {
            commands.entity(cell.entity).despawn();
        }
    }
    while row.cells.len() < columns {
        let entity = commands
            .spawn((
                part("sf-inventory-cell"),
                Node {
                    width: Val::Px(style.cell.x),
                    height: Val::Px(style.cell.y),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceBetween,
                    overflow: Overflow::clip(),
                    ..Node::default()
                },
                BackgroundColor(colors.cell),
                bevy_ui_widgets::Button,
                sf_ui_bevy::UiFocusable,
                InventoryCell(None),
                ChildOf(row.entity),
            ))
            .id();
        let icon = commands
            .spawn((
                part("sf-inventory-icon"),
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    display: Display::None,
                    ..Node::default()
                },
                ImageNode::default(),
                Pickable::IGNORE,
                ChildOf(entity),
            ))
            .id();
        let name = commands
            .spawn((
                part("sf-inventory-name"),
                Text::new(""),
                colors.font(0.6),
                TextColor(colors.text),
                Pickable::IGNORE,
                ChildOf(entity),
            ))
            .id();
        let count = commands
            .spawn((
                part("sf-inventory-count"),
                Text::new(""),
                colors.font(0.7),
                TextColor(colors.text),
                Node {
                    align_self: AlignSelf::FlexEnd,
                    ..Node::default()
                },
                Pickable::IGNORE,
                ChildOf(entity),
            ))
            .id();
        row.cells.push(CellSlot {
            entity,
            icon,
            name,
            count,
            bound: None,
        });
    }
}

/// The view → tabs, the grid window, totals, the tooltip, the split dialog and `InputFocus`.
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub(crate) fn sync_inventory(
    mut commands: Commands,
    colors: Res<ScreenColors>,
    labels: Res<ScreenLabels>,
    style: Res<InventoryGridStyle>,
    stack: Res<ScreenStackRes>,
    server: Option<Res<AssetServer>>,
    mut images: Option<ResMut<RichImages>>,
    computed: Query<&ComputedNode>,
    mut screens: Query<(
        Entity,
        &UiScreenRoot,
        &mut InventoryScreen,
        &mut InventoryParts,
    )>,
    mut nodes: Query<&mut Node>,
    mut bars: Query<&mut StatBar>,
    mut focus: ResMut<InputFocus>,
    mut out: MessageWriter<InventoryEvent>,
) {
    let colors = &*colors;
    for (root_entity, root, mut screen, mut parts) in &mut screens {
        let screen = &mut *screen;
        let parts = &mut *parts;
        // Geometry from the laid-out viewport (logical pixels).
        if let Ok(node) = computed.get(parts.viewport) {
            let size = node.size() * node.inverse_scale_factor();
            if size.x > 0.0 && size.y > 0.0 {
                let pitch = style.cell + Vec2::splat(style.gap);
                // Whole cells that fit; at least one.
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                let columns = (((size.x + style.gap) / pitch.x).floor() as usize).max(1);
                screen.grid.set_columns(columns);
                screen.grid.set_viewport(size.y);
                if (screen.grid.cell_extent() - pitch.y).abs() > f32::EPSILON {
                    screen.grid.set_cell_extent(pitch.y);
                }
            }
        }

        // Tabs with counts (badges), on list or tab changes only.
        let tabs_key = (screen.view.rows().version(), screen.view.tab());
        if parts.tabs_seen != Some(tabs_key) {
            parts.tabs_seen = Some(tabs_key);
            let counts = screen.view.tab_counts();
            parts
                .tabs
                .sync(&mut commands, 0..counts.len(), |commands, i| {
                    let (e, _) = spawn_button(commands, None, "sf-inventory-tab", "", colors);
                    commands.entity(e).insert(InventoryButton::Tab(*i));
                    e
                });
            for (i, count) in counts.iter().enumerate() {
                let Some(tab) = parts.tabs.get(&i) else {
                    continue;
                };
                let selected = i == screen.view.tab();
                commands.entity(tab).insert(classes(
                    "sf-inventory-tab",
                    &[("selected", selected), ("empty", *count == 0)],
                ));
                let name = screen.view.tabs()[i].label.clone();
                let color = if selected { colors.new } else { colors.text };
                relabel(&mut commands, tab, format!("{name} ({count})"), color);
            }
        }

        // The window. Nothing focused yet: the first stack (a pad needs a starting point).
        if screen.view.focus().is_none() {
            screen.view.move_focus(0);
        }
        let frame = screen.grid.frame(&mut screen.view);
        for slot in &frame.released {
            if let Some(row) = parts.rows.get(*slot)
                && let Ok(mut node) = nodes.get_mut(row.entity)
            {
                show(&mut node, false);
            }
        }
        let focused = frame.focus.map(|(_, k)| k);
        let mut focus_cell = None;
        for binding in &frame.rows {
            while parts.rows.len() <= binding.slot {
                let row = spawn_row(&mut commands, parts.viewport, frame.columns, &style, colors);
                parts.rows.push(row);
            }
            let row = &mut parts.rows[binding.slot];
            if row.cells.len() != frame.columns {
                fit_cells(&mut commands, row, frame.columns, &style, colors);
            }
            if let Ok(mut node) = nodes.get_mut(row.entity) {
                show(&mut node, true);
                let top = Val::Px(binding.offset);
                if node.top != top {
                    node.top = top;
                }
            }
            for (i, cell) in row.cells.iter_mut().enumerate() {
                let stack = binding.stacks.get(i).copied();
                let state = stack.and_then(|s| {
                    let version = screen.view.rows().row_version(&s)?;
                    Some((s, version, Some(s) == focused, Some(s) == screen.held))
                });
                if focused.is_some() && stack == focused {
                    focus_cell = Some(cell.entity);
                }
                if cell.bound == state {
                    continue;
                }
                cell.bound = state;
                commands.entity(cell.entity).insert(InventoryCell(stack));
                // Only the last grid row has empty cells: hiding them shifts nothing.
                if let Ok(mut node) = nodes.get_mut(cell.entity) {
                    show(&mut node, state.is_some());
                }
                let Some((s, _, is_focused, is_held)) = state else {
                    continue;
                };
                let Some(row_data) = screen.view.row(s) else {
                    continue;
                };
                commands.entity(cell.entity).insert(classes(
                    "sf-inventory-cell",
                    &[
                        ("equipped", row_data.is_equipped()),
                        ("focused", is_focused),
                        ("held", is_held),
                    ],
                ));
                let handle = row_data.info.icon.as_ref().and_then(|icon| {
                    images
                        .as_mut()
                        .and_then(|images| images.resolve(icon, server.as_deref()))
                });
                if let Ok(mut node) = nodes.get_mut(cell.icon) {
                    show(&mut node, handle.is_some());
                }
                if let Some(handle) = handle {
                    commands.entity(cell.icon).insert(ImageNode::new(handle));
                }
                commands
                    .entity(cell.name)
                    .insert(Text::new(row_data.info.name.clone()));
                let count = row_data.stack.count;
                commands.entity(cell.count).insert(Text::new(if count > 1 {
                    count.to_string()
                } else {
                    String::new()
                }));
            }
        }

        // Scrollbar thumb.
        if let Ok(mut node) = nodes.get_mut(parts.thumb) {
            let (start, length) = frame.thumb.unwrap_or((0.0, 1.0));
            node.top = Val::Percent(start * 100.0);
            node.height = Val::Percent(length * 100.0);
        }

        // Totals.
        let totals = screen.view.totals();
        let enc = totals.encumbrance;
        let totals_key = (
            enc.carried.to_bits(),
            enc.capacity.unwrap_or(-1.0).to_bits(),
        );
        if parts.totals_seen != Some(totals_key) {
            parts.totals_seen = Some(totals_key);
            let weight = match enc.capacity {
                Some(cap) => format!("{} {:.0}/{cap:.0}", labels.weight, enc.carried),
                None => format!("{} {:.0}", labels.weight, enc.carried),
            };
            commands.entity(parts.weight_text).insert(Text::new(weight));
            commands
                .entity(parts.value_text)
                .insert(Text::new(format!("{} {}", labels.value, totals.value)));
            if let Ok(mut bar) = bars.get_mut(parts.weight_bar) {
                // Bars are f32; carried weights are small.
                #[allow(clippy::cast_possible_truncation)]
                bar.set_if_neq(StatBar {
                    current: enc.carried as f32,
                    max: enc.capacity.unwrap_or(0.0) as f32,
                    kind: "encumbrance".into(),
                });
            }
        }

        // Focus changes go to the game (for the comparison).
        if parts.last_focus != Some(focused) {
            parts.last_focus = Some(focused);
            if screen
                .compare
                .as_ref()
                .is_some_and(|c| Some(c.candidate) != focused)
            {
                screen.compare = None;
            }
            out.write(InventoryEvent {
                screen: root_entity,
                intent: InventoryIntent::Focused(focused),
            });
        }

        // Tooltip: on the hovered cell, else the focused one.
        let tip_cell = parts
            .hovered
            .filter(|h| parts.cells().any(|c| c == *h))
            .or(focus_cell);
        let tip = tip_cell.and_then(|cell| {
            let stack = parts
                .rows
                .iter()
                .flat_map(|r| &r.cells)
                .find(|c| c.entity == cell)?
                .bound?
                .0;
            Some((cell, stack, screen.view.rows().row_version(&stack)?))
        });
        if let Ok(mut node) = nodes.get_mut(parts.tooltip) {
            show(&mut node, tip.is_some() && screen.split.is_none());
        }
        if let Some((cell, stack, version)) = tip {
            let compare = screen.compare.clone().filter(|c| c.candidate == stack);
            let key = (cell, stack, version, compare.clone());
            if parts.tooltip_seen.as_ref() != Some(&key) {
                commands.entity(parts.tooltip).insert(ChildOf(cell));
                commands
                    .entity(parts.tooltip_text)
                    .despawn_related::<Children>();
                if let Some(row) = screen.view.row(stack) {
                    let mut lines: Vec<(String, Option<Verdict>)> =
                        vec![(row.info.name.clone(), None)];
                    lines.push((
                        format!(
                            "{} {:.1}  {} {}",
                            labels.weight, row.info.weight, labels.value, row.info.value
                        ),
                        None,
                    ));
                    let stat_name = |id: &RecordId| {
                        screen
                            .stat_labels
                            .iter()
                            .find(|(s, _)| s == id)
                            .map_or_else(|| id.as_str().to_owned(), |(_, l)| l.clone())
                    };
                    match &compare {
                        Some(c) => {
                            for d in &c.stats {
                                let value = d.candidate.unwrap_or(0.0);
                                let text = if d.verdict == Verdict::Same {
                                    format!("{} {value}", stat_name(&d.stat))
                                } else {
                                    format!("{} {value} ({:+})", stat_name(&d.stat), d.delta)
                                };
                                lines.push((text, Some(d.verdict)));
                            }
                        }
                        None => {
                            for (id, value) in &row.info.stats {
                                lines.push((format!("{} {value}", stat_name(id)), None));
                            }
                        }
                    }
                    for (i, (text, verdict)) in lines.into_iter().enumerate() {
                        let text = if i == 0 { text } else { format!("\n{text}") };
                        let (class, color) = match verdict {
                            Some(Verdict::Better) => ("better", colors.link),
                            Some(Verdict::Worse) => ("worse", colors.bar_fill),
                            Some(Verdict::Changed) => ("changed", colors.new),
                            Some(Verdict::Same) | None => ("", colors.text),
                        };
                        commands.spawn((
                            TextSpan::new(text),
                            colors.font(0.9),
                            TextColor(color),
                            classes("sf-tooltip-line", &[(class, !class.is_empty())]),
                            ChildOf(parts.tooltip_text),
                        ));
                    }
                }
                parts.tooltip_seen = Some(key);
            }
        } else {
            parts.tooltip_seen = None;
        }

        // Split dialog.
        if let Ok(mut node) = nodes.get_mut(parts.split) {
            show(&mut node, screen.split.is_some());
        }
        if let Some((_, picker)) = &screen.split {
            commands.entity(parts.split_text).insert(Text::new(format!(
                "{} / {}",
                picker.value(),
                picker.max() + 1
            )));
            if let Ok(mut bar) = bars.get_mut(parts.split_bar) {
                bar.set_if_neq(StatBar {
                    current: picker.fraction(),
                    max: 1.0,
                    kind: String::new(),
                });
            }
        }

        // Ghost follows the drag (see the observers); hidden when nothing is dragged.
        if screen.held.is_none()
            && let Ok(mut node) = nodes.get_mut(parts.ghost)
        {
            show(&mut node, false);
        }

        if is_target(&stack, root) {
            let wanted = if screen.split.is_some() {
                Some(parts.split_ok)
            } else {
                focus_cell
            };
            if let Some(wanted) = wanted
                && focus.get() != Some(wanted)
            {
                focus.set(wanted, FocusCause::Navigated);
            }
        }
    }
}

/// Directions, tabs, split, pick up / put down, back, the wheel.
#[allow(clippy::too_many_arguments)]
pub(crate) fn inventory_input(
    mut navs: MessageReader<UiNavEvent>,
    mut actions: MessageReader<UiActionEvent>,
    mut wheel: MessageReader<MouseWheel>,
    mut stack: ResMut<ScreenStackRes>,
    style: Res<InventoryGridStyle>,
    mut screens: Query<(Entity, &UiScreenRoot, &mut InventoryScreen)>,
    mut out: MessageWriter<InventoryEvent>,
) {
    let navs: Vec<UiNavEvent> = navs.read().cloned().collect();
    let actions: Vec<UiActionEvent> = actions.read().cloned().collect();
    let lines: f32 = wheel
        .read()
        .map(|w| match w.unit {
            MouseScrollUnit::Line => -w.y,
            MouseScrollUnit::Pixel => -w.y / WHEEL_LINE,
        })
        .sum();
    for (entity, root, mut screen) in &mut screens {
        let screen = &mut *screen;
        for nav in navs.iter().filter(|n| n.screen == root.0) {
            let (dx, dy): (isize, isize) = match nav.direction {
                CompassOctant::North => (0, -1),
                CompassOctant::South => (0, 1),
                CompassOctant::East => (1, 0),
                CompassOctant::West => (-1, 0),
                _ => continue,
            };
            if let Some((_, picker)) = &mut screen.split {
                picker.step(i64::try_from(dx - 10 * dy).unwrap_or(0));
            } else {
                screen.grid.move_focus(&mut screen.view, dx, dy);
            }
        }
        for action in actions.iter().filter(|a| a.screen == root.0) {
            let focused = screen.view.focus();
            let intent = match action.action {
                UiAction::TabPrev if screen.split.is_none() => {
                    screen.view.cycle_tab(-1);
                    None
                }
                UiAction::TabNext if screen.split.is_none() => {
                    screen.view.cycle_tab(1);
                    None
                }
                UiAction::Context1 if screen.split.is_none() => {
                    focused.map(InventoryIntent::SplitRequested)
                }
                UiAction::Context2 if screen.split.is_none() => {
                    screen.held = match screen.held {
                        Some(_) => None,
                        None => focused,
                    };
                    None
                }
                UiAction::Back => {
                    // Cancel the dialog, then the held stack, then close.
                    if screen.split.take().is_none() && screen.held.take().is_none() {
                        stack.0.close(root.0.as_str());
                    }
                    None
                }
                _ => None,
            };
            if let Some(intent) = intent {
                out.write(InventoryEvent {
                    screen: entity,
                    intent,
                });
            }
        }
        if lines != 0.0 && is_target(&stack, root) && screen.split.is_none() {
            screen.grid.scroll_by(lines * (style.cell.y + style.gap));
        }
    }
}

/// A cell activated: use it, or drop the held stack there. Tabs and split buttons too.
pub(crate) fn on_inventory_button(
    activate: On<Activate>,
    cells: Query<&InventoryCell>,
    buttons: Query<&InventoryButton>,
    parents: Query<&ChildOf>,
    mut screens: Query<&mut InventoryScreen>,
    mut out: MessageWriter<InventoryEvent>,
) {
    let entity = activate.entity;
    let (cell, button) = (cells.get(entity).ok(), buttons.get(entity).ok());
    if cell.is_none() && button.is_none() {
        return;
    }
    let Some(root) = screen_of(entity, &parents, |e| screens.contains(e)) else {
        return;
    };
    let Ok(mut screen) = screens.get_mut(root) else {
        return;
    };
    let mut send = |intent| {
        out.write(InventoryEvent {
            screen: root,
            intent,
        });
    };
    if let Some(InventoryCell(Some(stack))) = cell {
        if screen.split.is_some() {
            return;
        }
        match screen.held.take() {
            Some(held) if held != *stack => send(InventoryIntent::Merge {
                from: held,
                into: Some(*stack),
            }),
            Some(_) => {}
            None => {
                screen.view.set_focus(Some(*stack));
                send(InventoryIntent::Use(*stack));
            }
        }
        return;
    }
    match button {
        Some(InventoryButton::Tab(i)) => screen.view.set_tab(*i),
        Some(InventoryButton::SplitLess) => {
            if let Some((_, p)) = &mut screen.split {
                p.step(-1);
            }
        }
        Some(InventoryButton::SplitMore) => {
            if let Some((_, p)) = &mut screen.split {
                p.step(1);
            }
        }
        Some(InventoryButton::SplitOk) => {
            if let Some((stack, picker)) = screen.split.take() {
                send(InventoryIntent::Split {
                    stack,
                    count: picker.value(),
                });
            }
        }
        Some(InventoryButton::SplitCancel) => screen.split = None,
        None => {}
    }
}

/// Hover focuses a cell (and shows its tooltip).
pub(crate) fn on_cell_over(
    over: On<PointerOver>,
    cells: Query<&InventoryCell>,
    parents: Query<&ChildOf>,
    mut screens: Query<(&mut InventoryScreen, &mut InventoryParts)>,
) {
    let Ok(InventoryCell(Some(stack))) = cells.get(over.entity) else {
        return;
    };
    let Some(root) = screen_of(over.entity, &parents, |e| screens.contains(e)) else {
        return;
    };
    if let Ok((mut screen, mut parts)) = screens.get_mut(root)
        && screen.split.is_none()
    {
        parts.hovered = Some(over.entity);
        if screen.view.focus() != Some(*stack) {
            screen.view.set_focus(Some(*stack));
        }
    }
}

/// Drag start: the stack is held and its ghost shows.
pub(crate) fn on_cell_drag_start(
    start: On<PointerDragStart>,
    cells: Query<&InventoryCell>,
    parents: Query<&ChildOf>,
    mut screens: Query<(&mut InventoryScreen, &InventoryParts)>,
    mut nodes: Query<&mut Node>,
    children: Query<&Children>,
    mut commands: Commands,
) {
    let Ok(InventoryCell(Some(stack))) = cells.get(start.entity) else {
        return;
    };
    let Some(root) = screen_of(start.entity, &parents, |e| screens.contains(e)) else {
        return;
    };
    let Ok((mut screen, parts)) = screens.get_mut(root) else {
        return;
    };
    screen.held = Some(*stack);
    if let Ok(mut node) = nodes.get_mut(parts.ghost) {
        show(&mut node, true);
        node.left = Val::Px(start.pointer.position.x + 8.0);
        node.top = Val::Px(start.pointer.position.y + 8.0);
    }
    let name = screen
        .view
        .row(*stack)
        .map(|r| r.info.name.clone())
        .unwrap_or_default();
    if let Some(text) = children
        .get(parts.ghost)
        .ok()
        .and_then(|c| c.first().copied())
    {
        commands.entity(text).insert(Text::new(name));
    }
}

/// The ghost follows the pointer.
pub(crate) fn on_cell_drag(
    drag: On<PointerDrag>,
    cells: Query<&InventoryCell>,
    parents: Query<&ChildOf>,
    screens: Query<&InventoryParts>,
    mut nodes: Query<&mut Node>,
) {
    if !cells.contains(drag.entity) {
        return;
    }
    let Some(root) = screen_of(drag.entity, &parents, |e| screens.contains(e)) else {
        return;
    };
    if let Ok(parts) = screens.get(root)
        && let Ok(mut node) = nodes.get_mut(parts.ghost)
    {
        node.left = Val::Px(drag.pointer.position.x + 8.0);
        node.top = Val::Px(drag.pointer.position.y + 8.0);
    }
}

/// Dropped on another cell: merge.
pub(crate) fn on_cell_drop(
    drop: On<PointerDragDrop>,
    cells: Query<&InventoryCell>,
    parents: Query<&ChildOf>,
    mut screens: Query<&mut InventoryScreen>,
    mut out: MessageWriter<InventoryEvent>,
) {
    let (Ok(InventoryCell(Some(into))), Ok(InventoryCell(Some(from)))) =
        (cells.get(drop.entity), cells.get(drop.dropped))
    else {
        return;
    };
    let Some(root) = screen_of(drop.entity, &parents, |e| screens.contains(e)) else {
        return;
    };
    if let Ok(mut screen) = screens.get_mut(root) {
        screen.held = None;
        if from != into {
            out.write(InventoryEvent {
                screen: root,
                intent: InventoryIntent::Merge {
                    from: *from,
                    into: Some(*into),
                },
            });
        }
    }
}

/// Drag over: nothing held any more.
pub(crate) fn on_cell_drag_end(
    end: On<PointerDragEnd>,
    cells: Query<&InventoryCell>,
    parents: Query<&ChildOf>,
    mut screens: Query<&mut InventoryScreen>,
) {
    if !cells.contains(end.entity) {
        return;
    }
    if let Some(root) = screen_of(end.entity, &parents, |e| screens.contains(e))
        && let Ok(mut screen) = screens.get_mut(root)
    {
        screen.held = None;
    }
}
