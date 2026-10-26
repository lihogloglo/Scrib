//! The container (loot) screen (ui.md "Stock screens"): two icon grids side by side, the
//! container's stacks on the left and the player's on the right, over two
//! [`InventoryView`]s. It reuses the inventory screen's grid, tooltip, quantity dialog and
//! drag ghost (`crate::inventory` shared parts), so a theme styles both alike
//! (`sf-inventory-*` parts inside `sf-container-pane`s).
//!
//! The app owns both holders: this screen sends [`ContainerEvent`]s (move part or all of a
//! stack to the other side, take all) and the app applies them and re-syncs the views.
//!
//! Input (behaviour after Morrowind's container window and Skyrim's two-pane transfer): the
//! d-pad moves in a grid and crosses to the other pane at the side edge, LB/RB pick the
//! container or the player pane, A moves the focused stack whole, X asks how many first
//! (stacks of more than one), Y takes all, B cancels the dialog, then closes. With a mouse:
//! hover focuses, click moves a stack, drag one onto the other pane to move it, the wheel
//! scrolls the pane in use.

use bevy_asset::AssetServer;
use bevy_ecs::prelude::*;
use bevy_input::mouse::{MouseScrollUnit, MouseWheel};
use bevy_input_focus::{FocusCause, InputFocus};
use bevy_math::CompassOctant;
use bevy_picking::events::{
    PointerDrag, PointerDragDrop, PointerDragEnd, PointerDragStart, PointerOver,
};
use bevy_text::TextColor;
use bevy_ui::prelude::*;
use bevy_ui::{ComputedNode, UiGlobalTransform};
use bevy_ui_widgets::Activate;
use sf_rpg::{Holder, RecordId, StackId};
use sf_ui_bevy::{
    RichImages, ScreenStackRes, UiAction, UiActionEvent, UiHandlesBack, UiNavEvent, UiOwnsFocus,
    UiScreenRoot, WHEEL_LINE,
};
use sf_ui_inventory::{InventoryGrid, InventoryView, QuantityPicker};

use crate::inventory::{
    GridPane, InventoryCell, InventoryGridStyle, SplitDialog, Tooltip, move_ghost, spawn_ghost,
    weight_text,
};
use crate::labels::ScreenLabels;
use crate::widgets::{ScreenColors, StatBar, classes, label, panel, part, show, spawn_button};
use crate::{is_target, screen_of};

/// Which pane: the container (left) or the player (right).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ContainerSide {
    Container,
    Player,
}

impl ContainerSide {
    pub const BOTH: [Self; 2] = [Self::Container, Self::Player];

    #[must_use]
    pub fn other(self) -> Self {
        match self {
            Self::Container => Self::Player,
            Self::Player => Self::Container,
        }
    }

    fn index(self) -> usize {
        match self {
            Self::Container => 0,
            Self::Player => 1,
        }
    }
}

/// What the player asked for.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContainerIntent {
    /// Move `count` pieces of `stack` from holder `from` to holder `to` (the other pane).
    Move {
        from: Holder,
        to: Holder,
        stack: StackId,
        count: u32,
    },
    /// Move every stack of the container to the player.
    TakeAll { from: Holder, to: Holder },
}

#[derive(Message, Clone, Debug, PartialEq, Eq)]
pub struct ContainerEvent {
    pub screen: Entity,
    pub intent: ContainerIntent,
}

/// On a screen root: the container screen over the container's and the player's views.
#[derive(Component)]
#[require(Node, UiOwnsFocus, UiHandlesBack)]
pub struct ContainerScreen {
    /// The container's stacks (left pane).
    pub container: InventoryView,
    /// The player's stacks (right pane).
    pub player: InventoryView,
    /// The container pane's grid and the player pane's.
    pub grids: [InventoryGrid; 2],
    /// The container's display name (its pane's heading).
    pub title: String,
    /// Taking from it is a crime in the app's rules (the pane gets the `owned` class and the
    /// heading the warning colour); the screen itself treats it like any other.
    pub owned: bool,
    /// Display names of stats (the catalog's `StatDef` labels, localized).
    pub stat_labels: Vec<(RecordId, String)>,
    active: ContainerSide,
    count: Option<(ContainerSide, StackId, QuantityPicker)>,
    held: Option<(ContainerSide, StackId)>,
}

impl ContainerScreen {
    pub fn new(container: InventoryView, player: InventoryView) -> Self {
        Self {
            container,
            player,
            grids: [
                InventoryGrid::new(1, 1.0, 1.0),
                InventoryGrid::new(1, 1.0, 1.0),
            ],
            title: String::new(),
            owned: false,
            stat_labels: Vec::new(),
            active: ContainerSide::Container,
            count: None,
            held: None,
        }
    }

    #[must_use]
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    #[must_use]
    pub fn with_owned(mut self, owned: bool) -> Self {
        self.owned = owned;
        self
    }

    #[must_use]
    pub fn with_stat_labels(mut self, labels: Vec<(RecordId, String)>) -> Self {
        self.stat_labels = labels;
        self
    }

    pub fn view(&self, side: ContainerSide) -> &InventoryView {
        match side {
            ContainerSide::Container => &self.container,
            ContainerSide::Player => &self.player,
        }
    }

    pub fn view_mut(&mut self, side: ContainerSide) -> &mut InventoryView {
        match side {
            ContainerSide::Container => &mut self.container,
            ContainerSide::Player => &mut self.player,
        }
    }

    /// The pane the pad and the wheel act on.
    pub fn active(&self) -> ContainerSide {
        self.active
    }

    pub fn set_active(&mut self, side: ContainerSide) {
        self.active = side;
    }

    /// The "how many?" dialog: the stack it moves, from which side, and the count picked.
    pub fn count_dialog(&self) -> Option<&(ContainerSide, StackId, QuantityPicker)> {
        self.count.as_ref()
    }

    /// The stack being dragged.
    pub fn held(&self) -> Option<(ContainerSide, StackId)> {
        self.held
    }

    /// The move of `count` pieces of `stack` from `side` to the other.
    fn move_intent(&self, side: ContainerSide, stack: StackId, count: u32) -> ContainerIntent {
        ContainerIntent::Move {
            from: self.view(side).holder(),
            to: self.view(side.other()).holder(),
            stack,
            count,
        }
    }

    /// Moves the whole stack (A, click, drop on the other pane).
    fn move_whole(&self, side: ContainerSide, stack: StackId) -> Option<ContainerIntent> {
        let count = self.view(side).row(stack)?.stack.count;
        Some(self.move_intent(side, stack, count))
    }

    /// X: one piece moves at once, a larger stack asks how many (all of it to start with).
    fn ask_count(&mut self, side: ContainerSide, stack: StackId) -> Option<ContainerIntent> {
        let count = self.view(side).row(stack)?.stack.count;
        if count <= 1 {
            return Some(self.move_intent(side, stack, count));
        }
        self.count = QuantityPicker::new(1, count, count).map(|p| (side, stack, p));
        None
    }

    fn take_all(&self) -> Option<ContainerIntent> {
        (!self.container.rows().is_empty()).then(|| ContainerIntent::TakeAll {
            from: self.container.holder(),
            to: self.player.holder(),
        })
    }
}

/// On a pane's root: which side it shows.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ContainerPane(pub ContainerSide);

/// A button of the container screen and what it does.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContainerButton {
    TakeAll,
    Close,
    CountLess,
    CountMore,
    CountOk,
    CountCancel,
}

#[derive(Debug)]
struct PaneParts {
    root: Entity,
    title: Entity,
    grid: GridPane,
    /// (active, owned, title) shown now.
    seen: Option<(bool, bool, String)>,
}

/// The screen's parts and pools.
#[derive(Component, Debug)]
pub struct ContainerParts {
    panes: [PaneParts; 2],
    weight_text: Entity,
    weight_seen: Option<(u64, u64)>,
    take_all: Entity,
    close: Entity,
    tooltip: Tooltip,
    count: SplitDialog,
    ghost: Entity,
}

impl ContainerParts {
    /// The pane widget of `side`.
    pub fn pane(&self, side: ContainerSide) -> Entity {
        self.panes[side.index()].root
    }

    /// Every cell widget in `side`'s pool.
    pub fn cells(&self, side: ContainerSide) -> impl Iterator<Item = Entity> + '_ {
        self.panes[side.index()].grid.cells()
    }

    /// The cell showing `stack` on `side`, if it is in the window.
    pub fn cell_of(&self, side: ContainerSide, stack: StackId) -> Option<Entity> {
        self.panes[side.index()].grid.cell_of(stack)
    }

    pub fn take_all(&self) -> Entity {
        self.take_all
    }

    pub fn close(&self) -> Entity {
        self.close
    }

    pub fn tooltip(&self) -> Entity {
        self.tooltip.root
    }

    pub fn count_dialog(&self) -> Entity {
        self.count.root
    }
}

#[allow(clippy::too_many_lines)]
pub(crate) fn spawn_container(
    mut commands: Commands,
    colors: Res<ScreenColors>,
    labels: Res<ScreenLabels>,
    roots: Query<Entity, Added<ContainerScreen>>,
) {
    for root in &roots {
        commands.entity(root).insert(panel(
            "sf-container",
            Node {
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(12.0)),
                row_gap: Val::Px(8.0),
                width: Val::Px(920.0),
                height: Val::Px(540.0),
                ..Node::default()
            },
            &colors,
        ));
        let row = commands
            .spawn((
                part("sf-container-panes"),
                Node {
                    flex_grow: 1.0,
                    column_gap: Val::Px(12.0),
                    ..Node::default()
                },
                ChildOf(root),
            ))
            .id();
        let panes = ContainerSide::BOTH.map(|side| {
            let pane = commands
                .spawn((
                    part("sf-container-pane"),
                    Node {
                        flex_grow: 1.0,
                        flex_basis: Val::Px(0.0),
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(4.0),
                        ..Node::default()
                    },
                    ContainerPane(side),
                    ChildOf(row),
                ))
                .id();
            let title = commands
                .spawn((label("sf-container-pane-title", "", &colors), ChildOf(pane)))
                .id();
            let body = commands
                .spawn((
                    part("sf-inventory-body"),
                    Node {
                        flex_grow: 1.0,
                        column_gap: Val::Px(4.0),
                        ..Node::default()
                    },
                    ChildOf(pane),
                ))
                .id();
            PaneParts {
                root: pane,
                title,
                grid: GridPane::spawn(&mut commands, body, &colors),
                seen: None,
            }
        });
        let footer = commands
            .spawn((
                part("sf-container-footer"),
                Node {
                    column_gap: Val::Px(12.0),
                    align_items: AlignItems::Center,
                    ..Node::default()
                },
                ChildOf(root),
            ))
            .id();
        let weight_text = commands
            .spawn((label("sf-container-weight", "", &colors), ChildOf(footer)))
            .id();
        commands.spawn((
            Node {
                flex_grow: 1.0,
                ..Node::default()
            },
            ChildOf(footer),
        ));
        let (take_all, _) = spawn_button(
            &mut commands,
            Some(footer),
            "sf-container-take-all",
            labels.take_all.clone(),
            &colors,
        );
        commands.entity(take_all).insert(ContainerButton::TakeAll);
        let (close, _) = spawn_button(
            &mut commands,
            Some(footer),
            "sf-container-close",
            labels.back.clone(),
            &colors,
        );
        commands.entity(close).insert(ContainerButton::Close);
        let tooltip = Tooltip::spawn(&mut commands, root, &colors);
        let count = SplitDialog::spawn(
            &mut commands,
            root,
            labels.take_count.clone(),
            [
                ContainerButton::CountLess,
                ContainerButton::CountMore,
                ContainerButton::CountOk,
                ContainerButton::CountCancel,
            ],
            &labels,
            &colors,
        );
        let ghost = spawn_ghost(&mut commands, &colors);
        commands.entity(root).insert(ContainerParts {
            panes,
            weight_text,
            weight_seen: None,
            take_all,
            close,
            tooltip,
            count,
            ghost,
        });
    }
}

/// The views → both grids, headings, the player's weight, the tooltip, the count dialog and
/// `InputFocus`.
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub(crate) fn sync_container(
    mut commands: Commands,
    colors: Res<ScreenColors>,
    labels: Res<ScreenLabels>,
    style: Res<InventoryGridStyle>,
    stack: Res<ScreenStackRes>,
    server: Option<Res<AssetServer>>,
    mut images: Option<ResMut<RichImages>>,
    computed: Query<&ComputedNode>,
    geometry: Query<(&ComputedNode, &UiGlobalTransform)>,
    mut screens: Query<(
        Entity,
        &UiScreenRoot,
        &mut ContainerScreen,
        &mut ContainerParts,
    )>,
    mut nodes: Query<&mut Node>,
    mut bars: Query<&mut StatBar>,
    mut focus: ResMut<InputFocus>,
) {
    let colors = &*colors;
    for (root_entity, root, mut screen, mut parts) in &mut screens {
        let screen = &mut *screen;
        let parts = &mut *parts;
        // An emptied pane hands the pad to the other one.
        let active = screen.active;
        if screen.view(active).rows().is_empty() && !screen.view(active.other()).rows().is_empty() {
            screen.active = active.other();
        }

        let mut focus_cell = None;
        for side in ContainerSide::BOTH {
            let i = side.index();
            let is_active = side == screen.active;
            let pane = &mut parts.panes[i];
            pane.grid.fit(&mut screen.grids[i], &computed, &style);

            let title = match side {
                ContainerSide::Container => screen.title.clone(),
                ContainerSide::Player => labels.inventory.clone(),
            };
            let owned = screen.owned && side == ContainerSide::Container;
            let key = (is_active, owned, title);
            if pane.seen.as_ref() != Some(&key) {
                commands.entity(pane.root).insert(classes(
                    "sf-container-pane",
                    &[("active", is_active), ("owned", owned)],
                ));
                let color = if owned {
                    colors.bar_fill
                } else if is_active {
                    colors.new
                } else {
                    colors.text
                };
                commands
                    .entity(pane.title)
                    .insert((Text::new(key.2.clone()), TextColor(color)));
                pane.seen = Some(key);
            }

            let held = screen.held.filter(|(s, _)| *s == side).map(|(_, k)| k);
            let view = match side {
                ContainerSide::Container => &mut screen.container,
                ContainerSide::Player => &mut screen.player,
            };
            // Nothing focused yet: the first stack (a pad needs a starting point).
            if view.focus().is_none() {
                view.move_focus(0);
            }
            let mut frame = screen.grids[i].frame(view);
            if !is_active {
                // Only the pane in use shows a focused cell.
                frame.focus = None;
            }
            let cell = pane.grid.bind(
                &mut commands,
                &mut nodes,
                images.as_deref_mut(),
                server.as_deref(),
                &style,
                colors,
                &frame,
                view,
                held,
            );
            if is_active {
                focus_cell = cell;
            }
        }

        // The player's load: what taking more would cost.
        let enc = screen.player.encumbrance();
        let weight_key = (
            enc.carried.to_bits(),
            enc.capacity.unwrap_or(-1.0).to_bits(),
        );
        if parts.weight_seen != Some(weight_key) {
            parts.weight_seen = Some(weight_key);
            commands
                .entity(parts.weight_text)
                .insert(Text::new(weight_text(&labels, enc)));
        }

        let active = screen.active;
        let tip = focus_cell
            .filter(|_| screen.count.is_none())
            .and_then(|cell| Some((cell, parts.panes[active.index()].grid.stack_of(cell)?)));
        parts.tooltip.sync(
            &mut commands,
            &mut nodes,
            &geometry,
            root_entity,
            tip,
            screen.view(active),
            None,
            &screen.stat_labels,
            &labels,
            colors,
        );

        parts.count.sync(
            &mut commands,
            &mut nodes,
            &mut bars,
            screen.count.as_ref().map(|(_, _, p)| (p, p.max())),
        );

        if screen.held.is_none()
            && let Ok(mut node) = nodes.get_mut(parts.ghost)
        {
            show(&mut node, false);
        }

        if is_target(&stack, root) {
            let wanted = if screen.count.is_some() {
                parts.count.ok
            } else {
                // Both panes empty: the close button.
                focus_cell.unwrap_or(parts.close)
            };
            if focus.get() != Some(wanted) {
                focus.set(wanted, FocusCause::Navigated);
            }
        }
    }
}

/// Directions (crossing panes at the side edge), pane switch, count, take all, back, the wheel.
#[allow(clippy::too_many_arguments)]
pub(crate) fn container_input(
    mut navs: MessageReader<UiNavEvent>,
    mut actions: MessageReader<UiActionEvent>,
    mut wheel: MessageReader<MouseWheel>,
    mut stack: ResMut<ScreenStackRes>,
    style: Res<InventoryGridStyle>,
    mut screens: Query<(Entity, &UiScreenRoot, &mut ContainerScreen)>,
    mut out: MessageWriter<ContainerEvent>,
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
            if let Some((_, _, picker)) = &mut screen.count {
                picker.step(i64::try_from(dx - 10 * dy).unwrap_or(0));
                continue;
            }
            let side = screen.active;
            let i = side.index();
            let view = match side {
                ContainerSide::Container => &mut screen.container,
                ContainerSide::Player => &mut screen.player,
            };
            let before = view.focus();
            screen.grids[i].move_focus(view, dx, dy);
            // At the side edge towards the other pane: cross over.
            let towards_other = match side {
                ContainerSide::Container => dx > 0,
                ContainerSide::Player => dx < 0,
            };
            if towards_other
                && view.focus() == before
                && !screen.view(side.other()).rows().is_empty()
            {
                screen.active = side.other();
            }
        }
        for action in actions.iter().filter(|a| a.screen == root.0) {
            let side = screen.active;
            let focused = screen.view(side).focus();
            let intent = match action.action {
                UiAction::TabPrev if screen.count.is_none() => {
                    screen.active = ContainerSide::Container;
                    None
                }
                UiAction::TabNext if screen.count.is_none() => {
                    screen.active = ContainerSide::Player;
                    None
                }
                UiAction::Context1 if screen.count.is_none() => {
                    focused.and_then(|stack| screen.ask_count(side, stack))
                }
                UiAction::Context2 if screen.count.is_none() => screen.take_all(),
                UiAction::Back => {
                    // Cancel the dialog, then the drag, then close.
                    if screen.count.take().is_none() && screen.held.take().is_none() {
                        stack.0.close(root.0.as_str());
                    }
                    None
                }
                _ => None,
            };
            if let Some(intent) = intent {
                out.write(ContainerEvent {
                    screen: entity,
                    intent,
                });
            }
        }
        if lines != 0.0 && is_target(&stack, root) && screen.count.is_none() {
            screen.grids[screen.active.index()].scroll_by(lines * (style.cell.y + style.gap));
        }
    }
}

/// The pane `entity` is in (a cell, or the pane's empty space).
fn pane_of(
    entity: Entity,
    parents: &Query<&ChildOf>,
    panes: &Query<&ContainerPane>,
) -> Option<ContainerSide> {
    let pane = screen_of(entity, parents, |e| panes.contains(e))?;
    panes.get(pane).ok().map(|p| p.0)
}

/// A cell activated: move its stack whole. The footer and count dialog buttons too.
#[allow(clippy::too_many_arguments)]
pub(crate) fn on_container_button(
    activate: On<Activate>,
    cells: Query<&InventoryCell>,
    buttons: Query<&ContainerButton>,
    panes: Query<&ContainerPane>,
    parents: Query<&ChildOf>,
    mut screens: Query<(&UiScreenRoot, &mut ContainerScreen)>,
    mut stack: ResMut<ScreenStackRes>,
    mut out: MessageWriter<ContainerEvent>,
) {
    let entity = activate.entity;
    let (cell, button) = (cells.get(entity).ok(), buttons.get(entity).ok());
    if cell.is_none() && button.is_none() {
        return;
    }
    let Some(root) = screen_of(entity, &parents, |e| screens.contains(e)) else {
        return;
    };
    let Ok((id, mut screen)) = screens.get_mut(root) else {
        return;
    };
    let intent = if let Some(InventoryCell(Some(stack))) = cell {
        let Some(side) = pane_of(entity, &parents, &panes) else {
            return;
        };
        if screen.count.is_some() {
            return;
        }
        screen.active = side;
        screen.view_mut(side).set_focus(Some(*stack));
        screen.move_whole(side, *stack)
    } else {
        match button {
            Some(ContainerButton::TakeAll) if screen.count.is_none() => screen.take_all(),
            Some(ContainerButton::Close) => {
                stack.0.close(id.0.as_str());
                None
            }
            Some(ContainerButton::CountLess) => {
                if let Some((_, _, p)) = &mut screen.count {
                    p.step(-1);
                }
                None
            }
            Some(ContainerButton::CountMore) => {
                if let Some((_, _, p)) = &mut screen.count {
                    p.step(1);
                }
                None
            }
            Some(ContainerButton::CountOk) => screen
                .count
                .take()
                .map(|(side, stack, picker)| screen.move_intent(side, stack, picker.value())),
            Some(ContainerButton::CountCancel) => {
                screen.count = None;
                None
            }
            _ => None,
        }
    };
    if let Some(intent) = intent {
        out.write(ContainerEvent {
            screen: root,
            intent,
        });
    }
}

/// Hover focuses a cell and makes its pane the one in use.
pub(crate) fn on_container_cell_over(
    over: On<PointerOver>,
    cells: Query<&InventoryCell>,
    panes: Query<&ContainerPane>,
    parents: Query<&ChildOf>,
    mut screens: Query<&mut ContainerScreen>,
) {
    let Ok(InventoryCell(Some(stack))) = cells.get(over.entity) else {
        return;
    };
    let Some(root) = screen_of(over.entity, &parents, |e| screens.contains(e)) else {
        return;
    };
    let Some(side) = pane_of(over.entity, &parents, &panes) else {
        return;
    };
    if let Ok(mut screen) = screens.get_mut(root)
        && screen.count.is_none()
    {
        screen.active = side;
        if screen.view(side).focus() != Some(*stack) {
            screen.view_mut(side).set_focus(Some(*stack));
        }
    }
}

/// Drag start: the stack is held and its ghost shows.
#[allow(clippy::too_many_arguments)]
pub(crate) fn on_container_drag_start(
    start: On<PointerDragStart>,
    cells: Query<&InventoryCell>,
    panes: Query<&ContainerPane>,
    parents: Query<&ChildOf>,
    mut screens: Query<(&mut ContainerScreen, &ContainerParts)>,
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
    let Some(side) = pane_of(start.entity, &parents, &panes) else {
        return;
    };
    let Ok((mut screen, parts)) = screens.get_mut(root) else {
        return;
    };
    screen.held = Some((side, *stack));
    let name = screen
        .view(side)
        .row(*stack)
        .map(|r| r.info.name.clone())
        .unwrap_or_default();
    move_ghost(
        &mut commands,
        &mut nodes,
        &children,
        parts.ghost,
        start.pointer.position,
        Some(name),
    );
}

/// The ghost follows the pointer.
pub(crate) fn on_container_drag(
    drag: On<PointerDrag>,
    cells: Query<&InventoryCell>,
    parents: Query<&ChildOf>,
    screens: Query<&ContainerParts>,
    mut nodes: Query<&mut Node>,
    children: Query<&Children>,
    mut commands: Commands,
) {
    if !cells.contains(drag.entity) {
        return;
    }
    let Some(root) = screen_of(drag.entity, &parents, |e| screens.contains(e)) else {
        return;
    };
    if let Ok(parts) = screens.get(root) {
        move_ghost(
            &mut commands,
            &mut nodes,
            &children,
            parts.ghost,
            drag.pointer.position,
            None,
        );
    }
}

/// Dropped on the other pane (a cell or its empty space): move the stack whole.
pub(crate) fn on_container_drop(
    drop: On<PointerDragDrop>,
    cells: Query<&InventoryCell>,
    panes: Query<&ContainerPane>,
    parents: Query<&ChildOf>,
    mut screens: Query<&mut ContainerScreen>,
    mut out: MessageWriter<ContainerEvent>,
) {
    if !cells.contains(drop.dropped) {
        return;
    }
    let Some(root) = screen_of(drop.entity, &parents, |e| screens.contains(e)) else {
        return;
    };
    let Some(onto) = pane_of(drop.entity, &parents, &panes) else {
        return;
    };
    let Ok(mut screen) = screens.get_mut(root) else {
        return;
    };
    let Some((from, stack)) = screen.held.take() else {
        return;
    };
    if from != onto
        && let Some(intent) = screen.move_whole(from, stack)
    {
        out.write(ContainerEvent {
            screen: root,
            intent,
        });
    }
}

/// Drag over: nothing held any more.
pub(crate) fn on_container_drag_end(
    end: On<PointerDragEnd>,
    cells: Query<&InventoryCell>,
    parents: Query<&ChildOf>,
    mut screens: Query<&mut ContainerScreen>,
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

/// The drag ghost is a window-level node: it goes with its screen.
pub(crate) fn on_container_removed(
    removed: On<Remove<ContainerParts>>,
    parts: Query<&ContainerParts>,
    mut commands: Commands,
) {
    if let Ok(parts) = parts.get(removed.entity)
        && let Ok(mut ghost) = commands.get_entity(parts.ghost)
    {
        ghost.despawn();
    }
}
