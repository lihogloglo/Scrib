//! The service list screen (services.md "Screens"), over `sf_rpg_ui`'s [`ServiceListView`]:
//! a title, the player's gold, the priced rows (label, detail line, price) and an optional
//! progress bar for a purchase that passes time. Disabled rows stay in the list, greyed,
//! with their reason (as `OpenMW` keeps the tooltip on greyed buttons; behaviour reference
//! only). Rows with lasting effects ask "are you sure?" first (the view's confirm rule).
//!
//! The screen sends intents only, as [`ServiceScreenEvent`]s: `BuyEntry` requests, and
//! [`ServiceIntent::Close`] when it closes itself (back to dialogue). The host answers with
//! service events (`screen.view.apply_event`), keeps `view.set_player_gold` current and
//! drives `progress` while time passes.
//!
//! Input: up/down move, LB/RB page, A (or a click) buys the focused row or answers the
//! question, left/right pick the answer, B dismisses the question, then closes. Hover
//! focuses a row.
//!
//! Every row is spawned (keyed, so a re-listing touches only what changed); the lists are
//! short (a trainer's skills, a guide's destinations). The view's `VirtualList` drives the
//! focus and paging; scrolling follows the focus (`scroll_to_focus`).

use std::collections::HashMap;

use bevy_ecs::prelude::*;
use bevy_input_focus::{FocusCause, InputFocus};
use bevy_math::CompassOctant;
use bevy_picking::Pickable;
use bevy_picking::events::PointerOver;
use bevy_ui::ScrollPosition;
use bevy_ui::prelude::*;
use bevy_ui_widgets::{Activate, ScrollArea};
use sf_rpg::service::{EntryDetail, EntryId, EntryRefusal};
use sf_rpg_ui::{ListNav, ListOutcome, ListToast, RowReason, ServiceListView, ServiceRow};
use sf_ui_bevy::{
    ScreenStackRes, UiAction, UiActionEvent, UiFocusable, UiHandlesBack, UiNavEvent, UiOwnsFocus,
    UiScreenRoot,
};

use crate::labels::ScreenLabels;
use crate::trade::{ServiceIntent, ServiceLabels, ServiceScreenEvent};
use crate::widgets::{
    Question, ScreenColors, StatBar, button, centred_window, classes, label, label_in, part,
    show, spawn_bar,
};
use crate::{is_target, screen_of};

/// On a screen root: the service list over a [`ServiceListView`]. The host owns the view's
/// data; the screen reads it and turns input into requests.
#[derive(Component)]
#[require(Node, UiOwnsFocus, UiHandlesBack)]
pub struct ServiceListScreen {
    pub view: ServiceListView,
    /// The host's progress (0 to 1) of a purchase that passes time; `None` hides the bar.
    pub progress: Option<f32>,
    /// The focused answer of the question: 0 yes, 1 no.
    answer: usize,
    close_sent: bool,
}

impl ServiceListScreen {
    pub fn new(view: ServiceListView) -> Self {
        Self {
            view,
            progress: None,
            answer: 0,
            close_sent: false,
        }
    }
}

/// A focusable part of the service list.
#[derive(Component, Clone, Debug, PartialEq, Eq)]
pub enum ServicePart {
    Row(EntryId),
    Yes,
    No,
    Close,
}

/// The screen's parts and what they show.
#[derive(Component, Debug)]
pub struct ServiceListParts {
    title: Entity,
    gold: Entity,
    /// (title, gold) shown now.
    head_seen: Option<(String, u64)>,
    rows: crate::keyed::KeyedChildren<EntryId>,
    /// Each row as drawn, and whether it was focused.
    shown: HashMap<EntryId, (ServiceRow, bool)>,
    empty: Entity,
    progress: Entity,
    note: Entity,
    note_text: String,
    close: Entity,
    question: Question,
    asked: bool,
}

impl ServiceListParts {
    /// The row showing `entry`.
    pub fn row(&self, entry: &EntryId) -> Option<Entity> {
        self.rows.get(entry)
    }

    /// Rows in display order.
    pub fn rows(&self) -> impl Iterator<Item = (&EntryId, Entity)> {
        self.rows.iter()
    }

    pub fn gold(&self) -> Entity {
        self.gold
    }

    pub fn empty_label(&self) -> Entity {
        self.empty
    }

    pub fn progress_bar(&self) -> Entity {
        self.progress
    }

    /// The bought / refused line.
    pub fn note(&self) -> Entity {
        self.note
    }

    pub fn close(&self) -> Entity {
        self.close
    }

    pub fn overlay(&self) -> Entity {
        self.question.overlay
    }

    /// Yes, then no.
    pub fn confirm_buttons(&self) -> [Entity; 2] {
        self.question.buttons
    }
}

#[allow(clippy::too_many_lines)]
pub(crate) fn spawn_service_list(
    mut commands: Commands,
    colors: Res<ScreenColors>,
    labels: Res<ScreenLabels>,
    service: Res<ServiceLabels>,
    roots: Query<Entity, Added<ServiceListScreen>>,
) {
    let colors = &*colors;
    for root in &roots {
        commands.entity(root).insert((
            part("sf-service"),
            centred_window(Node {
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(16.0)),
                row_gap: Val::Px(8.0),
                width: Val::Px(640.0),
                max_width: Val::Percent(100.0),
                max_height: Val::Percent(100.0),
                ..Node::default()
            }),
            BackgroundColor(colors.panel),
        ));
        let head = commands
            .spawn((
                part("sf-service-head"),
                Node {
                    column_gap: Val::Px(12.0),
                    justify_content: JustifyContent::SpaceBetween,
                    ..Node::default()
                },
                ChildOf(root),
            ))
            .id();
        let title = commands
            .spawn((label("sf-service-title", "", colors), ChildOf(head)))
            .id();
        let gold = commands
            .spawn((label("sf-service-gold", "", colors), ChildOf(head)))
            .id();
        let list = commands
            .spawn((
                part("sf-service-list"),
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
                label_in(
                    "sf-service-empty",
                    service.no_entries.clone(),
                    colors.muted,
                    colors,
                ),
                ChildOf(root),
            ))
            .id();
        let progress = spawn_bar(
            &mut commands,
            root,
            "sf-service-progress",
            StatBar {
                current: 0.0,
                max: 1.0,
                kind: String::new(),
            },
            colors,
        );
        let note = commands
            .spawn((label("sf-service-note", "", colors), ChildOf(root)))
            .id();
        let footer = commands
            .spawn((
                part("sf-service-footer"),
                Node {
                    justify_content: JustifyContent::FlexEnd,
                    ..Node::default()
                },
                ChildOf(root),
            ))
            .id();
        let close = commands
            .spawn((
                button("sf-service-close", service.close.clone(), colors),
                ServicePart::Close,
                ChildOf(footer),
            ))
            .id();
        let question = Question::spawn(
            &mut commands,
            root,
            [
                "sf-service-confirm",
                "sf-service-confirm-text",
                "sf-service-confirm-choices",
            ],
            [
                ("sf-service-yes", labels.yes.clone(), ServicePart::Yes),
                ("sf-service-no", labels.cancel.clone(), ServicePart::No),
            ],
            colors,
        );
        commands.entity(root).insert(ServiceListParts {
            title,
            gold,
            head_seen: None,
            rows: crate::keyed::KeyedChildren::new(list),
            shown: HashMap::new(),
            empty,
            progress,
            note,
            note_text: String::new(),
            close,
            question,
            asked: false,
        });
    }
}

/// A row's second line.
fn detail_text(detail: &EntryDetail, labels: &ServiceLabels) -> String {
    match detail {
        EntryDetail::Stat {
            stat,
            current,
            next,
        } => format!("{} {current} -> {next}", labels.text(stat)),
        EntryDetail::Destination { place, hours } => format!(
            "{}, {}",
            labels.text(&place.0),
            labels.hours.replace("{n}", &hours.to_string())
        ),
        EntryDetail::Grant { what } => labels.text(what),
        EntryDetail::Item { condition, max, .. } => format!("{condition} / {max}"),
        EntryDetail::Text(key) => labels.text(&key.0),
    }
}

/// Draws a row's content (the row entity stays).
fn draw_row(
    commands: &mut Commands,
    entity: Entity,
    row: &ServiceRow,
    focused: bool,
    labels: &ServiceLabels,
    colors: &ScreenColors,
) {
    let enabled = row.enabled();
    commands.entity(entity).insert(classes(
        "sf-service-row",
        &[("disabled", !enabled), ("focused", focused)],
    ));
    commands.entity(entity).despawn_related::<Children>();
    let color = if enabled {
        colors.text
    } else {
        colors.unavailable
    };
    let text = commands
        .spawn((
            part("sf-service-text"),
            Node {
                flex_direction: FlexDirection::Column,
                flex_grow: 1.0,
                ..Node::default()
            },
            Pickable::IGNORE,
            ChildOf(entity),
        ))
        .id();
    commands.spawn((
        label_in(
            "sf-service-label",
            labels.text(&row.entry.label.0),
            color,
            colors,
        ),
        Pickable::IGNORE,
        ChildOf(text),
    ));
    commands.spawn((
        label_in(
            "sf-service-detail",
            detail_text(&row.entry.detail, labels),
            colors.muted,
            colors,
        ),
        Pickable::IGNORE,
        ChildOf(text),
    ));
    let reason = match &row.reason {
        None => String::new(),
        Some(RowReason::TooExpensive) => labels.too_expensive.clone(),
        Some(RowReason::Refused(why)) => labels.text(&why.0),
    };
    commands.spawn((
        label_in("sf-service-reason", reason, colors.muted, colors),
        Pickable::IGNORE,
        ChildOf(entity),
    ));
    commands.spawn((
        label_in("sf-service-price", row.entry.price.to_string(), color, colors),
        Pickable::IGNORE,
        ChildOf(entity),
    ));
}

fn entry_label(view: &ServiceListView, entry: &EntryId, labels: &ServiceLabels) -> String {
    view.row(entry)
        .map_or_else(|| labels.text(&entry.0), |r| labels.text(&r.entry.label.0))
}

fn toast_text(view: &ServiceListView, toast: &ListToast, labels: &ServiceLabels) -> String {
    match toast {
        ListToast::Bought { entry, price } => labels
            .bought
            .replace("{entry}", &entry_label(view, entry, labels))
            .replace("{price}", &price.to_string()),
        ListToast::Refused { reason, .. } => match reason {
            EntryRefusal::TooExpensive => labels.too_expensive.clone(),
            EntryRefusal::Refused(why) => labels.text(&why.0),
            _ => labels.refused.clone(),
        },
    }
}

/// The view → head, rows (keyed, changed rows redrawn), progress, note, question and focus.
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub(crate) fn sync_service_list(
    mut commands: Commands,
    colors: Res<ScreenColors>,
    service: Res<ServiceLabels>,
    stack: Res<ScreenStackRes>,
    mut screens: Query<(&UiScreenRoot, &mut ServiceListScreen, &mut ServiceListParts)>,
    mut nodes: Query<&mut Node>,
    mut bars: Query<&mut StatBar>,
    mut focus: ResMut<InputFocus>,
) {
    let colors = &*colors;
    let service = &*service;
    for (root, mut screen, mut parts) in &mut screens {
        let screen = &mut *screen;
        let parts = &mut *parts;
        let view = &mut screen.view;

        let head = (service.text(&view.title().0), view.player_gold());
        if parts.head_seen.as_ref() != Some(&head) {
            commands
                .entity(parts.title)
                .insert(Text::new(head.0.clone()));
            commands.entity(parts.gold).insert(Text::new(format!(
                "{} {}",
                service.your_gold, head.1
            )));
            parts.head_seen = Some(head);
        }

        let order: Vec<EntryId> = view.order().to_vec();
        // Nothing focused yet: the first row (a pad needs a starting point).
        if let Some(first) = order.first()
            && view.focus().is_none_or(|f| !order.contains(f))
        {
            view.focus_row(first);
        }
        parts.rows.sync(&mut commands, order.iter().cloned(), |commands, key| {
            commands
                .spawn((
                    part("sf-service-row"),
                    Node {
                        column_gap: Val::Px(10.0),
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(4.0)),
                        flex_shrink: 0.0,
                        ..Node::default()
                    },
                    BackgroundColor(colors.cell),
                    bevy_ui_widgets::Button,
                    UiFocusable,
                    ServicePart::Row(key.clone()),
                ))
                .id()
        });
        parts.shown.retain(|k, _| order.contains(k));
        let focused = view.focus().cloned();
        for key in &order {
            let (Some(entity), Some(row)) = (parts.rows.get(key), view.row(key)) else {
                continue;
            };
            let is_focused = focused.as_ref() == Some(key);
            if parts
                .shown
                .get(key)
                .is_some_and(|(r, f)| r == row && *f == is_focused)
            {
                continue;
            }
            draw_row(&mut commands, entity, row, is_focused, service, colors);
            parts.shown.insert(key.clone(), (row.clone(), is_focused));
        }
        if let Ok(mut node) = nodes.get_mut(parts.empty) {
            show(&mut node, order.is_empty());
        }

        if let Ok(mut node) = nodes.get_mut(parts.progress) {
            show(&mut node, screen.progress.is_some());
        }
        if let Some(p) = screen.progress
            && let Ok(mut bar) = bars.get_mut(parts.progress)
        {
            bar.set_if_neq(StatBar {
                current: p.clamp(0.0, 1.0),
                max: 1.0,
                kind: String::new(),
            });
        }

        if let Some(toast) = view.take_toast() {
            let text = toast_text(view, &toast, service);
            if parts.note_text != text {
                commands.entity(parts.note).insert(Text::new(text.clone()));
                parts.note_text = text;
            }
        }

        let question = view.asking().map_or_else(String::new, |entry| {
            let price = view.row(entry).map_or(0, |r| r.entry.price);
            service
                .confirm_buy
                .replace("{entry}", &entry_label(view, entry, service))
                .replace("{price}", &price.to_string())
        });
        let asking = !question.is_empty();
        if asking && !parts.asked {
            // A new question starts on yes.
            screen.answer = 0;
        }
        parts.asked = asking;
        parts
            .question
            .set(&mut commands, &mut nodes, question, colors);

        if is_target(&stack, root) {
            let wanted = if asking {
                parts.question.buttons[screen.answer.min(1)]
            } else {
                focused
                    .as_ref()
                    .and_then(|f| parts.rows.get(f))
                    .unwrap_or(parts.close)
            };
            if focus.get() != Some(wanted) {
                focus.set(wanted, FocusCause::Navigated);
            }
        }
    }
}

fn intent_of(outcome: ListOutcome) -> ServiceIntent {
    match outcome {
        ListOutcome::Request(request) => ServiceIntent::Request(request),
        ListOutcome::Close => ServiceIntent::Close,
    }
}

/// Up/down move, left/right pick the answer, LB/RB page, B dismisses or closes; closes the
/// screen once the view is closed.
pub(crate) fn service_list_input(
    mut navs: MessageReader<UiNavEvent>,
    mut actions: MessageReader<UiActionEvent>,
    mut stack: ResMut<ScreenStackRes>,
    mut screens: Query<(Entity, &UiScreenRoot, &mut ServiceListScreen)>,
    mut out: MessageWriter<ServiceScreenEvent>,
) {
    let navs: Vec<UiNavEvent> = navs.read().cloned().collect();
    let actions: Vec<UiActionEvent> = actions.read().cloned().collect();
    for (entity, root, mut screen) in &mut screens {
        let screen = &mut *screen;
        let mut outcomes = Vec::new();
        for nav in navs.iter().filter(|n| n.screen == root.0) {
            let asking = screen.view.asking().is_some();
            match nav.direction {
                CompassOctant::East if asking => screen.answer = 1,
                CompassOctant::West if asking => screen.answer = 0,
                CompassOctant::North => outcomes.extend(screen.view.nav(ListNav::Move(-1))),
                CompassOctant::South => outcomes.extend(screen.view.nav(ListNav::Move(1))),
                _ => {}
            }
        }
        for action in actions.iter().filter(|a| a.screen == root.0) {
            let nav = match action.action {
                UiAction::TabPrev => ListNav::Page(-1),
                UiAction::TabNext => ListNav::Page(1),
                UiAction::Back => ListNav::Back,
                _ => continue,
            };
            outcomes.extend(screen.view.nav(nav));
        }
        for outcome in outcomes {
            out.write(ServiceScreenEvent {
                screen: entity,
                intent: intent_of(outcome),
            });
        }
        if screen.view.is_closed() && !screen.close_sent {
            screen.close_sent = true;
            stack.0.close(root.0.as_str());
        }
    }
}

/// A part activated (A, Enter, click): buy a row, answer the question, close.
pub(crate) fn on_service_part(
    activate: On<Activate>,
    kinds: Query<&ServicePart>,
    parents: Query<&ChildOf>,
    mut screens: Query<&mut ServiceListScreen>,
    mut out: MessageWriter<ServiceScreenEvent>,
) {
    let Ok(kind) = kinds.get(activate.entity) else {
        return;
    };
    let Some(root) = screen_of(activate.entity, &parents, |e| screens.contains(e)) else {
        return;
    };
    let Ok(mut screen) = screens.get_mut(root) else {
        return;
    };
    let view = &mut screen.view;
    let outcome = match kind {
        ServicePart::Yes => view.confirm(),
        ServicePart::No => {
            view.dismiss();
            None
        }
        // The question is modal.
        _ if view.asking().is_some() => None,
        ServicePart::Row(entry) => view.activate(entry),
        ServicePart::Close => view.close(),
    };
    if let Some(outcome) = outcome {
        out.write(ServiceScreenEvent {
            screen: root,
            intent: intent_of(outcome),
        });
    }
}

/// Hover focuses a row.
pub(crate) fn on_service_row_over(
    over: On<PointerOver>,
    kinds: Query<&ServicePart>,
    parents: Query<&ChildOf>,
    mut screens: Query<&mut ServiceListScreen>,
) {
    let Ok(ServicePart::Row(entry)) = kinds.get(over.entity) else {
        return;
    };
    let Some(root) = screen_of(over.entity, &parents, |e| screens.contains(e)) else {
        return;
    };
    if let Ok(mut screen) = screens.get_mut(root)
        && screen.view.asking().is_none()
        && screen.view.focus() != Some(entry)
    {
        screen.view.focus_row(entry);
    }
}
