//! The trade (barter) screen (services.md "Screens"), over `sf_rpg_ui`'s [`TradeView`]: the
//! provider's stock on the left, the player's items on the right, the deal bar between them
//! (staged lines per side, totals, both purses, the balance with the haggle stepper, Accept,
//! Cancel). It reuses the inventory screen's tooltip, quantity dialog and drag ghost
//! (`crate::inventory` shared parts), so a theme styles them alike; its own parts are
//! `sf-trade-*`.
//!
//! The panes are lists with a price column rather than icon grids: every row shows the
//! pieces not staged yet and the host's whole-stack quote, and a stack the provider does not
//! trade is greyed with the reason in its price column. Currency never shows (money moves
//! only through the balance).
//!
//! The screen sends intents only: every input yields at most one `ServiceRequest`, written
//! as a [`ServiceScreenEvent`]; the host applies it on its tick, answers with service events
//! (`screen.view.apply_event`), re-syncs the panes after item events and re-sends the price
//! columns (`screen.view.set_prices`).
//!
//! Input: the d-pad moves up and down a list and sideways between a pane and the deal bar,
//! LB/RB pick the provider or the player pane, A (or a click) stages the focused stack whole
//! or unstages the focused line whole, X asks how many first, Y accepts, LS resets the offer,
//! RS jumps to the bar, B closes the count dialog, then drops a drag, then leaves the bar,
//! then cancels the trade (the screen closes itself once the view is closed). On the offer
//! stepper, left/right step the balance by 1 and LB/RB by 10% of the deal. With a mouse:
//! hover focuses a row, click stages, drag a row onto the bar or the other pane to stage it,
//! drag a bar line back onto its pane to unstage it.

use std::collections::HashMap;

use bevy_color::Color;
use bevy_ecs::prelude::*;
use bevy_input_focus::{FocusCause, InputFocus};
use bevy_math::CompassOctant;
use bevy_picking::Pickable;
use bevy_picking::events::{
    PointerDrag, PointerDragDrop, PointerDragEnd, PointerDragStart, PointerOver,
};
use bevy_text::TextColor;
use bevy_ui::prelude::*;
use bevy_ui::{ComputedNode, ScrollPosition, UiGlobalTransform};
use bevy_ui_widgets::{Activate, ScrollArea};
use sf_rpg::service::{ServiceRequest, Side, TradeRefusal};
use sf_rpg::{RecordId, StackId};
use sf_rpg_ui::{
    BarLine, DealBar, DealProblem, DropTarget, Held, RowBlock, TradeFocus, TradeNav, TradePane,
    TradeRow, TradeToast, TradeView,
};
use sf_ui_bevy::{
    ScreenStackRes, UiAction, UiActionEvent, UiFocusable, UiHandlesBack, UiNavEvent, UiOwnsFocus,
    UiScreenRoot,
};

use crate::inventory::{SplitDialog, Tooltip, move_ghost, spawn_ghost};
use crate::keyed::KeyedChildren;
use crate::labels::ScreenLabels;
use crate::widgets::{
    ScreenColors, StatBar, classes, label, label_in, panel, part, relabel, set_disabled, show,
    spawn_button,
};
use crate::{is_target, screen_of};

// ---------------------------------------------------------------- shared with the list screen

/// What a service screen asks the host for.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ServiceIntent {
    /// Apply this request (from the trade screen or the service list).
    Request(ServiceRequest),
    /// The service list closed itself (back to dialogue; list services hold no session).
    /// The trade screen sends `ServiceRequest::Cancel` instead.
    Close,
}

/// A service screen's intent for the host (net-ready: the host forwards the request as the
/// player's command).
#[derive(Message, Clone, Debug, PartialEq, Eq)]
pub struct ServiceScreenEvent {
    pub screen: Entity,
    pub intent: ServiceIntent,
}

/// Fixed strings of the service screens, already localized by the app (English fallbacks
/// here), and the app's table for the `LabelRef` keys services carry (row labels, titles,
/// refusal reasons). A key missing from `texts` shows as itself.
#[derive(Resource, Clone, Debug)]
pub struct ServiceLabels {
    /// Localized text by label key.
    pub texts: HashMap<RecordId, String>,
    /// Column headings: `service-item`, `-count`, `-price`.
    pub item: String,
    pub count: String,
    pub price: String,
    /// Deal bar side headings.
    pub buying: String,
    pub selling: String,
    /// Price column of a stack the provider does not trade.
    pub not_traded: String,
    /// Count column of a stack wholly on the bar.
    pub all_staged: String,
    pub buy_total: String,
    pub sell_total: String,
    pub your_gold: String,
    pub merchant_gold: String,
    /// The provider's money when it has no limit.
    pub unlimited: String,
    /// Balance lines (`{n}` replaced).
    pub you_pay: String,
    pub you_receive: String,
    pub even: String,
    /// The player's gold after the deal (`{n}` replaced).
    pub after: String,
    pub reset_offer: String,
    pub accept: String,
    pub cancel: String,
    pub how_many: String,
    /// Deal problems and refusals.
    pub not_enough_gold: String,
    pub merchant_cant_afford: String,
    pub haggle_refused: String,
    pub refused: String,
    /// The deal settled.
    pub deal_done: String,
    /// List screen: "are you sure?" (`{entry}`, `{price}` replaced).
    pub confirm_buy: String,
    /// List screen: bought (`{entry}`, `{price}` replaced).
    pub bought: String,
    pub too_expensive: String,
    /// List screen: a travel row's hours (`{n}` replaced).
    pub hours: String,
    pub no_entries: String,
    pub close: String,
}

impl Default for ServiceLabels {
    fn default() -> Self {
        let s = |t: &str| t.to_owned();
        Self {
            texts: HashMap::new(),
            item: s("Item"),
            count: s("Count"),
            price: s("Price"),
            buying: s("Buying"),
            selling: s("Selling"),
            not_traded: s("not traded"),
            all_staged: s("on the bar"),
            buy_total: s("Buy"),
            sell_total: s("Sell"),
            your_gold: s("Your gold"),
            merchant_gold: s("Merchant"),
            unlimited: s("plenty"),
            you_pay: s("You pay {n}"),
            you_receive: s("You receive {n}"),
            even: s("Even trade"),
            after: s("Gold after: {n}"),
            reset_offer: s("Reset"),
            accept: s("Accept"),
            cancel: s("Cancel"),
            how_many: s("How many?"),
            not_enough_gold: s("Not enough gold."),
            merchant_cant_afford: s("The merchant can't afford that."),
            haggle_refused: s("The merchant refuses your offer."),
            refused: s("The merchant won't do that."),
            deal_done: s("Deal done."),
            confirm_buy: s("Buy {entry} for {price}?"),
            bought: s("{entry} bought for {price}."),
            too_expensive: s("Too expensive"),
            hours: s("{n} h"),
            no_entries: s("Nothing on offer."),
            close: s("Close"),
        }
    }
}

impl ServiceLabels {
    /// The localized text of a label key (the key itself when the table has none).
    pub fn text(&self, key: &RecordId) -> String {
        self.texts
            .get(key)
            .cloned()
            .unwrap_or_else(|| key.as_str().to_owned())
    }
}

/// `template` with `{n}` replaced.
fn with_n(template: &str, n: impl std::fmt::Display) -> String {
    template.replace("{n}", &n.to_string())
}

// ---------------------------------------------------------------- the screen

const PANES: [TradePane; 2] = [TradePane::Provider, TradePane::Player];

fn pane_index(pane: TradePane) -> usize {
    match pane {
        TradePane::Provider => 0,
        TradePane::Player => 1,
    }
}

/// On a screen root: the trade screen over a [`TradeView`]. The host owns the view's data
/// (`view.sync`, `view.set_prices`, `view.apply_event`); the screen only reads it and turns
/// input into requests.
#[derive(Component)]
#[require(Node, UiOwnsFocus, UiHandlesBack)]
pub struct TradeScreen {
    pub view: TradeView,
    /// The provider's display name (its pane's heading).
    pub title: String,
    /// Display names of stats (the catalog's `StatDef` labels, localized) for the tooltip.
    pub stat_labels: Vec<(RecordId, String)>,
    close_sent: bool,
}

impl TradeScreen {
    pub fn new(view: TradeView) -> Self {
        Self {
            view,
            title: String::new(),
            stat_labels: Vec::new(),
            close_sent: false,
        }
    }

    #[must_use]
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    #[must_use]
    pub fn with_stat_labels(mut self, labels: Vec<(RecordId, String)>) -> Self {
        self.stat_labels = labels;
        self
    }
}

/// On a pane's root: which pane it shows (a drop target).
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct TradePaneRoot(pub TradePane);

/// On the deal bar's root (a drop target).
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct TradeBarRoot;

/// A pane row: the stack it shows.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct TradeRowPart {
    pub pane: TradePane,
    pub stack: StackId,
}

/// A deal-bar line: its index in the session.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct TradeLinePart(pub u32);

/// A button of the trade screen and what it does.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub enum TradeButton {
    OfferLess,
    /// The balance itself (focusable for the stepper; activating it does nothing).
    Offer,
    OfferMore,
    OfferReset,
    Accept,
    Cancel,
    CountLess,
    CountMore,
    CountOk,
    CountCancel,
}

/// What a pane row shows: its trade state, name, focused, held.
type RowState = (TradeRow, String, bool, bool);
/// What a bar line shows: the line, its item name, focused, held.
type LineState = (BarLine, String, bool, bool);

#[derive(Debug)]
struct PaneParts {
    root: Entity,
    title: Entity,
    rows: KeyedChildren<StackId>,
    shown: HashMap<StackId, RowState>,
    /// (active, title) shown now.
    seen: Option<(bool, String)>,
}

#[derive(Debug)]
struct BarSide {
    lines: KeyedChildren<u32>,
    shown: HashMap<u32, LineState>,
}

/// The screen's parts and what they show.
#[derive(Component, Debug)]
pub struct TradeParts {
    panes: [PaneParts; 2],
    bar: Entity,
    /// Buy lines, then sell lines.
    sides: [BarSide; 2],
    totals: Entity,
    gold: Entity,
    offer_row: Entity,
    offer: Entity,
    after: Entity,
    note: Entity,
    accept: Entity,
    cancel: Entity,
    /// The deal bar and whether an accept is pending, as shown.
    seen: Option<(DealBar, bool)>,
    /// The toast shown and the lines it was raised over (cleared when they change).
    toast: Option<(String, Vec<BarLine>)>,
    note_seen: String,
    tooltip: Tooltip,
    count: SplitDialog,
    ghost: Entity,
}

impl TradeParts {
    /// The pane widget (a drop target).
    pub fn pane(&self, pane: TradePane) -> Entity {
        self.panes[pane_index(pane)].root
    }

    /// The row showing `stack` in `pane`.
    pub fn row_of(&self, pane: TradePane, stack: StackId) -> Option<Entity> {
        self.panes[pane_index(pane)].rows.get(&stack)
    }

    /// Rows of `pane` in display order.
    pub fn rows(&self, pane: TradePane) -> impl Iterator<Item = (StackId, Entity)> + '_ {
        self.panes[pane_index(pane)].rows.iter().map(|(k, e)| (*k, e))
    }

    /// The deal-bar widget (a drop target).
    pub fn bar(&self) -> Entity {
        self.bar
    }

    /// The bar entry of session line `line`.
    pub fn line_of(&self, line: u32) -> Option<Entity> {
        self.sides.iter().find_map(|s| s.lines.get(&line))
    }

    pub fn totals(&self) -> Entity {
        self.totals
    }

    pub fn offer(&self) -> Entity {
        self.offer
    }

    /// The problem or toast line on the bar.
    pub fn note(&self) -> Entity {
        self.note
    }

    pub fn accept(&self) -> Entity {
        self.accept
    }

    pub fn cancel(&self) -> Entity {
        self.cancel
    }

    pub fn tooltip(&self) -> Entity {
        self.tooltip.root
    }

    pub fn count_dialog(&self) -> Entity {
        self.count.root
    }
}

/// A column's node: the name column grows, the others keep a fixed width.
fn column(grow: bool) -> Node {
    if grow {
        Node {
            flex_grow: 1.0,
            ..Node::default()
        }
    } else {
        Node {
            width: Val::Px(72.0),
            flex_shrink: 0.0,
            ..Node::default()
        }
    }
}

/// A scrolling list node under `parent`.
fn spawn_list(commands: &mut Commands, parent: Entity, name: &'static str) -> Entity {
    commands
        .spawn((
            part(name),
            Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(2.0),
                flex_grow: 1.0,
                flex_basis: Val::Px(0.0),
                min_height: Val::Px(0.0),
                overflow: Overflow::scroll_y(),
                ..Node::default()
            },
            ScrollPosition::default(),
            ScrollArea,
            ChildOf(parent),
        ))
        .id()
}

fn spawn_pane(
    commands: &mut Commands,
    parent: Entity,
    pane: TradePane,
    labels: &ServiceLabels,
    colors: &ScreenColors,
) -> PaneParts {
    let root = commands
        .spawn((
            part("sf-trade-pane"),
            Node {
                flex_grow: 1.0,
                flex_basis: Val::Px(0.0),
                min_height: Val::Px(0.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(4.0),
                ..Node::default()
            },
            TradePaneRoot(pane),
            ChildOf(parent),
        ))
        .id();
    let title = commands
        .spawn((label("sf-trade-pane-title", "", colors), ChildOf(root)))
        .id();
    let header = commands
        .spawn((
            part("sf-trade-header"),
            Node {
                column_gap: Val::Px(8.0),
                padding: UiRect::horizontal(Val::Px(6.0)),
                ..Node::default()
            },
            ChildOf(root),
        ))
        .id();
    for (name, text, grow) in [
        ("sf-trade-header-name", &labels.item, true),
        ("sf-trade-header-count", &labels.count, false),
        ("sf-trade-header-price", &labels.price, false),
    ] {
        commands.spawn((
            label_in(name, text.clone(), colors.muted, colors),
            column(grow),
            ChildOf(header),
        ));
    }
    let list = spawn_list(commands, root, "sf-trade-list");
    PaneParts {
        root,
        title,
        rows: KeyedChildren::new(list),
        shown: HashMap::new(),
        seen: None,
    }
}

/// A focusable row (pane row or bar line) with the fallback look.
fn row_node(colors: &ScreenColors, name: &'static str) -> impl Bundle {
    (
        part(name),
        Node {
            column_gap: Val::Px(8.0),
            padding: UiRect::axes(Val::Px(6.0), Val::Px(2.0)),
            align_items: AlignItems::Center,
            flex_shrink: 0.0,
            ..Node::default()
        },
        BackgroundColor(colors.cell),
        bevy_ui_widgets::Button,
        UiFocusable,
    )
}

/// Redraws a row's three columns (the row entity stays).
fn draw_columns(
    commands: &mut Commands,
    row: Entity,
    names: [&'static str; 3],
    texts: [String; 3],
    color: Color,
    colors: &ScreenColors,
) {
    commands.entity(row).despawn_related::<Children>();
    for (i, (name, text)) in names.into_iter().zip(texts).enumerate() {
        commands.spawn((
            label_in(name, text, color, colors),
            column(i == 0),
            Pickable::IGNORE,
            ChildOf(row),
        ));
    }
}

#[allow(clippy::too_many_lines)]
pub(crate) fn spawn_trade(
    mut commands: Commands,
    colors: Res<ScreenColors>,
    labels: Res<ScreenLabels>,
    service: Res<ServiceLabels>,
    roots: Query<Entity, Added<TradeScreen>>,
) {
    let colors = &*colors;
    for root in &roots {
        commands.entity(root).insert(panel(
            "sf-trade",
            Node {
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(12.0)),
                row_gap: Val::Px(8.0),
                width: Val::Px(1100.0),
                height: Val::Px(600.0),
                max_width: Val::Percent(100.0),
                max_height: Val::Percent(100.0),
                ..Node::default()
            },
            colors,
        ));
        let row = commands
            .spawn((
                part("sf-trade-panes"),
                Node {
                    flex_grow: 1.0,
                    min_height: Val::Px(0.0),
                    column_gap: Val::Px(12.0),
                    ..Node::default()
                },
                ChildOf(root),
            ))
            .id();
        // Children in spawn order: provider pane, deal bar, player pane.
        let provider = spawn_pane(&mut commands, row, TradePane::Provider, &service, colors);
        let bar = commands
            .spawn((
                part("sf-trade-bar"),
                Node {
                    width: Val::Px(320.0),
                    flex_shrink: 0.0,
                    min_height: Val::Px(0.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(4.0),
                    ..Node::default()
                },
                TradeBarRoot,
                ChildOf(row),
            ))
            .id();
        let sides = [
            ("sf-trade-bar-buy-title", &service.buying, "sf-trade-bar-buy"),
            ("sf-trade-bar-sell-title", &service.selling, "sf-trade-bar-sell"),
        ]
        .map(|(title, text, list)| {
            commands.spawn((label(title, text.clone(), colors), ChildOf(bar)));
            BarSide {
                lines: KeyedChildren::new(spawn_list(&mut commands, bar, list)),
                shown: HashMap::new(),
            }
        });
        let totals = commands
            .spawn((label("sf-trade-totals", "", colors), ChildOf(bar)))
            .id();
        let gold = commands
            .spawn((label("sf-trade-gold", "", colors), ChildOf(bar)))
            .id();
        let offer_row = commands
            .spawn((
                part("sf-trade-offer-row"),
                Node {
                    column_gap: Val::Px(4.0),
                    align_items: AlignItems::Center,
                    display: Display::None,
                    ..Node::default()
                },
                ChildOf(bar),
            ))
            .id();
        let mut offer = Entity::PLACEHOLDER;
        for (name, text, kind) in [
            ("sf-trade-offer-less", "-".to_owned(), TradeButton::OfferLess),
            ("sf-trade-offer", String::new(), TradeButton::Offer),
            ("sf-trade-offer-more", "+".to_owned(), TradeButton::OfferMore),
            ("sf-trade-offer-reset", service.reset_offer.clone(), TradeButton::OfferReset),
        ] {
            let (e, _) = spawn_button(&mut commands, Some(offer_row), name, text, colors);
            commands.entity(e).insert(kind);
            if kind == TradeButton::Offer {
                offer = e;
            }
        }
        let after = commands
            .spawn((label("sf-trade-after", "", colors), ChildOf(bar)))
            .id();
        let note = commands
            .spawn((
                label_in("sf-trade-note", "", colors.bar_fill, colors),
                ChildOf(bar),
            ))
            .id();
        let buttons = commands
            .spawn((
                part("sf-trade-buttons"),
                Node {
                    column_gap: Val::Px(8.0),
                    justify_content: JustifyContent::FlexEnd,
                    ..Node::default()
                },
                ChildOf(bar),
            ))
            .id();
        let (accept, _) = spawn_button(
            &mut commands,
            Some(buttons),
            "sf-trade-accept",
            service.accept.clone(),
            colors,
        );
        commands.entity(accept).insert(TradeButton::Accept);
        let (cancel, _) = spawn_button(
            &mut commands,
            Some(buttons),
            "sf-trade-cancel",
            service.cancel.clone(),
            colors,
        );
        commands.entity(cancel).insert(TradeButton::Cancel);
        let player = spawn_pane(&mut commands, row, TradePane::Player, &service, colors);

        let tooltip = Tooltip::spawn(&mut commands, root, colors);
        let count = SplitDialog::spawn(
            &mut commands,
            root,
            service.how_many.clone(),
            [
                TradeButton::CountLess,
                TradeButton::CountMore,
                TradeButton::CountOk,
                TradeButton::CountCancel,
            ],
            &labels,
            colors,
        );
        let ghost = spawn_ghost(&mut commands, colors);
        commands.entity(root).insert(TradeParts {
            panes: [provider, player],
            bar,
            sides,
            totals,
            gold,
            offer_row,
            offer,
            after,
            note,
            accept,
            cancel,
            seen: None,
            toast: None,
            note_seen: String::new(),
            tooltip,
            count,
            ghost,
        });
    }
}

/// The display name of a pane stack (the item id when the pane no longer has it).
fn stack_name(view: &TradeView, pane: TradePane, stack: StackId, item: &RecordId) -> String {
    view.pane(pane)
        .row(stack)
        .map_or_else(|| item.as_str().to_owned(), |r| r.info.name.clone())
}

/// A pane focus with nothing to point at: the pane's first shown stack, else the other
/// pane's, else the deal bar.
fn settle_focus(view: &mut TradeView) {
    let TradeFocus::Pane(pane) = view.focus() else {
        return;
    };
    if view.focused_stack().is_some() {
        return;
    }
    for p in [pane, pane.other()] {
        if let Some(first) = view.shown(p).first().copied() {
            view.focus_row(p, first);
            return;
        }
    }
    view.nav(TradeNav::Bar);
}

fn balance_text(balance: i64, labels: &ServiceLabels) -> String {
    match balance.signum() {
        1 => with_n(&labels.you_receive, balance),
        -1 => with_n(&labels.you_pay, balance.unsigned_abs()),
        _ => labels.even.clone(),
    }
}

fn toast_text(toast: &TradeToast, labels: &ServiceLabels) -> String {
    match toast {
        TradeToast::Done { .. } => labels.deal_done.clone(),
        TradeToast::Refused(reason) => match reason {
            TradeRefusal::PlayerCantAfford { .. } => labels.not_enough_gold.clone(),
            TradeRefusal::MerchantCantAfford { .. } => labels.merchant_cant_afford.clone(),
            TradeRefusal::HaggleFailed { .. } => labels.haggle_refused.clone(),
            TradeRefusal::Vetoed(veto) => labels.text(&veto.reason.0),
            TradeRefusal::NotTraded => labels.not_traded.clone(),
            _ => labels.refused.clone(),
        },
    }
}

fn problem_text(problem: Option<DealProblem>, labels: &ServiceLabels) -> String {
    match problem {
        Some(DealProblem::PlayerCantAfford { .. }) => labels.not_enough_gold.clone(),
        Some(DealProblem::MerchantCantAfford { .. }) => labels.merchant_cant_afford.clone(),
        Some(DealProblem::NothingStaged) | None => String::new(),
    }
}

/// The view → both panes, the deal bar, the tooltip, the count dialog and `InputFocus`.
/// Writes only what changed.
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub(crate) fn sync_trade(
    mut commands: Commands,
    colors: Res<ScreenColors>,
    labels: Res<ScreenLabels>,
    service: Res<ServiceLabels>,
    stack: Res<ScreenStackRes>,
    geometry: Query<(&ComputedNode, &UiGlobalTransform)>,
    mut screens: Query<(Entity, &UiScreenRoot, &mut TradeScreen, &mut TradeParts)>,
    mut nodes: Query<&mut Node>,
    mut bars: Query<&mut StatBar>,
    mut focus: ResMut<InputFocus>,
) {
    let colors = &*colors;
    let service = &*service;
    for (root_entity, root, mut screen, mut parts) in &mut screens {
        let screen = &mut *screen;
        let parts = &mut *parts;
        let view = &mut screen.view;
        settle_focus(view);
        let focus_now = view.focus();
        let held = view.held();

        for pane in PANES {
            let p = &mut parts.panes[pane_index(pane)];
            let is_active = focus_now == TradeFocus::Pane(pane);
            let title = match pane {
                TradePane::Provider => screen.title.clone(),
                TradePane::Player => labels.inventory.clone(),
            };
            let key = (is_active, title);
            if p.seen.as_ref() != Some(&key) {
                commands
                    .entity(p.root)
                    .insert(classes("sf-trade-pane", &[("active", is_active)]));
                let color = if is_active { colors.new } else { colors.text };
                commands
                    .entity(p.title)
                    .insert((Text::new(key.1.clone()), TextColor(color)));
                p.seen = Some(key);
            }

            let rows = view.rows(pane);
            let focused = view.pane(pane).focus();
            p.rows
                .sync(&mut commands, rows.iter().map(|r| r.stack), |commands, stack| {
                    commands
                        .spawn((
                            row_node(colors, "sf-trade-row"),
                            TradeRowPart {
                                pane,
                                stack: *stack,
                            },
                        ))
                        .id()
                });
            p.shown.retain(|k, _| rows.iter().any(|r| r.stack == *k));
            for row in &rows {
                let Some(entity) = p.rows.get(&row.stack) else {
                    continue;
                };
                let name = view
                    .pane(pane)
                    .row(row.stack)
                    .map(|r| r.info.name.clone())
                    .unwrap_or_default();
                let is_focused = is_active && focused == Some(row.stack);
                let is_held = held
                    == Some(Held::Stack {
                        pane,
                        stack: row.stack,
                    });
                let state: RowState = (*row, name, is_focused, is_held);
                if p.shown.get(&row.stack) == Some(&state) {
                    continue;
                }
                let blocked = row.blocked.is_some();
                commands.entity(entity).insert(classes(
                    "sf-trade-row",
                    &[("blocked", blocked), ("focused", is_focused), ("held", is_held)],
                ));
                let count = if row.blocked == Some(RowBlock::AllStaged) {
                    service.all_staged.clone()
                } else {
                    row.available.to_string()
                };
                let price = match (row.blocked, row.price) {
                    (Some(RowBlock::NotTraded), _) | (_, None) => service.not_traded.clone(),
                    (_, Some(price)) => price.to_string(),
                };
                let color = if blocked {
                    colors.unavailable
                } else {
                    colors.text
                };
                draw_columns(
                    &mut commands,
                    entity,
                    ["sf-trade-row-name", "sf-trade-row-count", "sf-trade-row-price"],
                    [state.1.clone(), count, price],
                    color,
                    colors,
                );
                p.shown.insert(row.stack, state);
            }
        }

        // The deal bar.
        let deal = view.deal();
        for (s, lines) in [(0, &deal.buy), (1, &deal.sell)] {
            let side = &mut parts.sides[s];
            side.lines
                .sync(&mut commands, lines.iter().map(|l| l.line), |commands, line| {
                    commands
                        .spawn((row_node(colors, "sf-trade-line"), TradeLinePart(*line)))
                        .id()
                });
            side.shown.retain(|k, _| lines.iter().any(|l| l.line == *k));
            for line in lines {
                let Some(entity) = side.lines.get(&line.line) else {
                    continue;
                };
                let name = stack_name(view, TradePane::of_side(line.side), line.stack, &line.item);
                let is_focused = focus_now == TradeFocus::Line(line.line);
                let is_held = held == Some(Held::Line(line.line));
                let state: LineState = (line.clone(), name, is_focused, is_held);
                if side.shown.get(&line.line) == Some(&state) {
                    continue;
                }
                commands.entity(entity).insert(classes(
                    "sf-trade-line",
                    &[
                        ("buy", line.side == Side::Buy),
                        ("focused", is_focused),
                        ("held", is_held),
                    ],
                ));
                draw_columns(
                    &mut commands,
                    entity,
                    ["sf-trade-line-name", "sf-trade-line-count", "sf-trade-line-price"],
                    [state.1.clone(), line.count.to_string(), line.price.to_string()],
                    colors.text,
                    colors,
                );
                side.shown.insert(line.line, state);
            }
        }

        let accepting = view.is_accepting();
        let deal_key = (deal.clone(), accepting);
        if parts.seen.as_ref() != Some(&deal_key) {
            commands.entity(parts.totals).insert(Text::new(format!(
                "{} {}   {} {}",
                service.buy_total, deal.buy_total, service.sell_total, deal.sell_total
            )));
            let merchant = deal
                .merchant_gold
                .map_or_else(|| service.unlimited.clone(), |g| g.to_string());
            commands.entity(parts.gold).insert(Text::new(format!(
                "{} {}   {} {merchant}",
                service.your_gold, deal.player_gold, service.merchant_gold
            )));
            if let Ok(mut node) = nodes.get_mut(parts.offer_row) {
                show(&mut node, deal.haggle);
            }
            commands
                .entity(parts.offer)
                .insert(classes("sf-trade-offer", &[("haggled", deal.haggled)]));
            let offer_color = if deal.haggled { colors.new } else { colors.text };
            relabel(
                &mut commands,
                parts.offer,
                balance_text(deal.balance, service),
                offer_color,
            );
            let after = if deal.buy.is_empty() && deal.sell.is_empty() {
                String::new()
            } else {
                with_n(&service.after, deal.player_after)
            };
            commands.entity(parts.after).insert(Text::new(after));
            let disabled = deal.problem.is_some() || accepting;
            set_disabled(&mut commands, parts.accept, disabled);
            commands
                .entity(parts.accept)
                .insert(classes("sf-trade-accept", &[("disabled", disabled)]));
            parts.seen = Some(deal_key);
        }

        // A toast stays until the deal changes; else the deal's problem shows.
        let lines_now: Vec<BarLine> = deal.buy.iter().chain(&deal.sell).cloned().collect();
        if let Some(toast) = view.take_toast() {
            parts.toast = Some((toast_text(&toast, service), lines_now));
        } else if parts.toast.as_ref().is_some_and(|(_, at)| *at != lines_now) {
            parts.toast = None;
        }
        let note = parts
            .toast
            .as_ref()
            .map_or_else(|| problem_text(deal.problem, service), |(t, _)| t.clone());
        if parts.note_seen != note {
            commands.entity(parts.note).insert(Text::new(note.clone()));
            parts.note_seen = note;
        }

        // The tooltip over the focused pane row.
        let tip_pane = match focus_now {
            TradeFocus::Pane(pane) if view.count_dialog().is_none() => Some(pane),
            _ => None,
        };
        let tip = tip_pane.and_then(|pane| {
            let (_, stack) = view.focused_stack()?;
            Some((parts.panes[pane_index(pane)].rows.get(&stack)?, stack))
        });
        parts.tooltip.sync(
            &mut commands,
            &mut nodes,
            &geometry,
            root_entity,
            tip,
            view.pane(tip_pane.unwrap_or(TradePane::Provider)),
            None,
            &screen.stat_labels,
            &labels,
            colors,
        );

        parts.count.sync(
            &mut commands,
            &mut nodes,
            &mut bars,
            view.count_dialog().map(|d| (&d.picker, d.picker.max())),
        );

        if held.is_none()
            && let Ok(mut node) = nodes.get_mut(parts.ghost)
        {
            show(&mut node, false);
        }

        if is_target(&stack, root) {
            let wanted = if view.count_dialog().is_some() {
                Some(parts.count.ok)
            } else {
                match focus_now {
                    TradeFocus::Pane(pane) => view
                        .focused_stack()
                        .and_then(|(_, s)| parts.panes[pane_index(pane)].rows.get(&s)),
                    TradeFocus::Line(line) => parts.line_of(line),
                    TradeFocus::Offer => Some(parts.offer),
                    TradeFocus::Accept => Some(parts.accept),
                    TradeFocus::Cancel => Some(parts.cancel),
                }
            }
            .unwrap_or(parts.cancel);
            if focus.get() != Some(wanted) {
                focus.set(wanted, FocusCause::Navigated);
            }
        }
    }
}

/// A d-pad direction as the view's navigation (`None`: nothing to do).
fn direction_nav(view: &TradeView, direction: CompassOctant) -> Option<TradeNav> {
    // (sideways step, upward step)
    let (step, rise): (i64, i32) = match direction {
        CompassOctant::North => (0, 1),
        CompassOctant::South => (0, -1),
        CompassOctant::East => (1, 0),
        CompassOctant::West => (-1, 0),
        _ => return None,
    };
    if view.count_dialog().is_some() {
        // Left/right by 1, up/down by 10.
        return Some(if rise == 0 {
            TradeNav::Step(step)
        } else {
            TradeNav::StepBig(rise)
        });
    }
    if rise != 0 {
        return Some(TradeNav::Move(if rise > 0 { -1 } else { 1 }));
    }
    let right = step > 0;
    Some(match view.focus() {
        TradeFocus::Offer => TradeNav::Step(step),
        // From a pane towards the middle: the deal bar.
        TradeFocus::Pane(TradePane::Provider) if right => TradeNav::Bar,
        TradeFocus::Pane(TradePane::Player) if !right => TradeNav::Bar,
        TradeFocus::Pane(_) => return None,
        // From the bar: the pane on that side.
        _ if right => TradeNav::Pane(TradePane::Player),
        _ => TradeNav::Pane(TradePane::Provider),
    })
}

/// A UI action as the view's navigation. Confirm arrives as `Activate` on the focused part.
fn action_nav(view: &TradeView, action: UiAction) -> Option<TradeNav> {
    let stepping = view.count_dialog().is_some() || view.focus() == TradeFocus::Offer;
    Some(match action {
        UiAction::TabPrev if stepping => TradeNav::StepBig(-1),
        UiAction::TabNext if stepping => TradeNav::StepBig(1),
        UiAction::TabPrev => TradeNav::Pane(TradePane::Provider),
        UiAction::TabNext => TradeNav::Pane(TradePane::Player),
        UiAction::Context1 => TradeNav::Quantity,
        UiAction::Context2 => TradeNav::Accept,
        UiAction::Context3 => TradeNav::ResetOffer,
        UiAction::Context4 => TradeNav::Bar,
        UiAction::Back => TradeNav::Back,
        _ => return None,
    })
}

fn send(out: &mut MessageWriter<ServiceScreenEvent>, screen: Entity, request: ServiceRequest) {
    out.write(ServiceScreenEvent {
        screen,
        intent: ServiceIntent::Request(request),
    });
}

/// Directions, pane switch, count, accept, reset, back; closes the screen once the view is
/// closed (cancelled, or a host veto).
pub(crate) fn trade_input(
    mut navs: MessageReader<UiNavEvent>,
    mut actions: MessageReader<UiActionEvent>,
    mut stack: ResMut<ScreenStackRes>,
    mut screens: Query<(Entity, &UiScreenRoot, &mut TradeScreen)>,
    mut out: MessageWriter<ServiceScreenEvent>,
) {
    let navs: Vec<UiNavEvent> = navs.read().cloned().collect();
    let actions: Vec<UiActionEvent> = actions.read().cloned().collect();
    for (entity, root, mut screen) in &mut screens {
        let view = &mut screen.view;
        let mut requests = Vec::new();
        for nav in navs.iter().filter(|n| n.screen == root.0) {
            if let Some(nav) = direction_nav(view, nav.direction) {
                requests.extend(view.nav(nav));
            }
        }
        for action in actions.iter().filter(|a| a.screen == root.0) {
            if let Some(nav) = action_nav(view, action.action) {
                requests.extend(view.nav(nav));
            }
        }
        for request in requests {
            send(&mut out, entity, request);
        }
        if screen.view.is_closed() && !screen.close_sent {
            screen.close_sent = true;
            stack.0.close(root.0.as_str());
        }
    }
}

/// A button of the bar or the count dialog.
fn press(view: &mut TradeView, button: TradeButton) -> Option<ServiceRequest> {
    let counting = view.count_dialog().is_some();
    match (button, counting) {
        (TradeButton::CountLess, true) => view.nav(TradeNav::Step(-1)),
        (TradeButton::CountMore, true) => view.nav(TradeNav::Step(1)),
        (TradeButton::CountOk, true) => view.confirm_count(),
        (TradeButton::CountCancel, true) => {
            view.close_count();
            None
        }
        (TradeButton::OfferLess, false) => view.step_offer(-1),
        (TradeButton::OfferMore, false) => view.step_offer(1),
        (TradeButton::OfferReset, false) => view.reset_offer(),
        (TradeButton::Accept, false) => view.accept(),
        (TradeButton::Cancel, false) => view.cancel(),
        // The count dialog is modal; the balance itself and stray count buttons do nothing.
        _ => None,
    }
}

/// A row, a bar line or a button activated (A, Enter, click).
pub(crate) fn on_trade_activate(
    activate: On<Activate>,
    rows: Query<&TradeRowPart>,
    lines: Query<&TradeLinePart>,
    buttons: Query<&TradeButton>,
    parents: Query<&ChildOf>,
    mut screens: Query<&mut TradeScreen>,
    mut out: MessageWriter<ServiceScreenEvent>,
) {
    let entity = activate.entity;
    let row = rows.get(entity).ok().copied();
    let line = lines.get(entity).ok().copied();
    let button = buttons.get(entity).ok().copied();
    if row.is_none() && line.is_none() && button.is_none() {
        return;
    }
    let Some(root) = screen_of(entity, &parents, |e| screens.contains(e)) else {
        return;
    };
    let Ok(mut screen) = screens.get_mut(root) else {
        return;
    };
    let view = &mut screen.view;
    let counting = view.count_dialog().is_some();
    let request = match (row, line, button) {
        (Some(r), _, _) if !counting => view.activate_row(r.pane, r.stack),
        (_, Some(TradeLinePart(l)), _) if !counting => view.activate_line(l),
        (_, _, Some(b)) => press(view, b),
        _ => None,
    };
    if let Some(request) = request {
        send(&mut out, root, request);
    }
}

/// Hover focuses a pane row.
pub(crate) fn on_trade_over(
    over: On<PointerOver>,
    rows: Query<&TradeRowPart>,
    parents: Query<&ChildOf>,
    mut screens: Query<&mut TradeScreen>,
) {
    let Ok(row) = rows.get(over.entity).copied() else {
        return;
    };
    let Some(root) = screen_of(over.entity, &parents, |e| screens.contains(e)) else {
        return;
    };
    if let Ok(mut screen) = screens.get_mut(root)
        && screen.view.count_dialog().is_none()
        && screen.view.focused_stack() != Some((row.pane, row.stack))
    {
        screen.view.focus_row(row.pane, row.stack);
    }
}

/// Drag start on a row or a bar line: held, and its ghost shows.
#[allow(clippy::too_many_arguments)]
pub(crate) fn on_trade_drag_start(
    start: On<PointerDragStart>,
    rows: Query<&TradeRowPart>,
    lines: Query<&TradeLinePart>,
    parents: Query<&ChildOf>,
    mut screens: Query<(&mut TradeScreen, &TradeParts)>,
    mut nodes: Query<&mut Node>,
    children: Query<&Children>,
    mut commands: Commands,
) {
    let row = rows.get(start.entity).ok().copied();
    let line = lines.get(start.entity).ok().copied();
    if row.is_none() && line.is_none() {
        return;
    }
    let Some(root) = screen_of(start.entity, &parents, |e| screens.contains(e)) else {
        return;
    };
    let Ok((mut screen, parts)) = screens.get_mut(root) else {
        return;
    };
    let view = &mut screen.view;
    let name = if let Some(r) = row {
        view.grab_stack(r.pane, r.stack).then(|| {
            view.pane(r.pane)
                .row(r.stack)
                .map(|x| x.info.name.clone())
                .unwrap_or_default()
        })
    } else if let Some(TradeLinePart(l)) = line {
        let staged = usize::try_from(l)
            .ok()
            .and_then(|i| view.lines().get(i))
            .map(|t| (t.side, t.stack, t.item.clone()));
        match staged {
            Some((side, stack, item)) if view.grab_line(l) => {
                Some(stack_name(view, TradePane::of_side(side), stack, &item))
            }
            _ => None,
        }
    } else {
        None
    };
    if let Some(name) = name {
        move_ghost(
            &mut commands,
            &mut nodes,
            &children,
            parts.ghost,
            start.pointer.position,
            Some(name),
        );
    }
}

/// The ghost follows the pointer.
#[allow(clippy::too_many_arguments)]
pub(crate) fn on_trade_drag(
    drag: On<PointerDrag>,
    rows: Query<&TradeRowPart>,
    lines: Query<&TradeLinePart>,
    parents: Query<&ChildOf>,
    screens: Query<&TradeParts>,
    mut nodes: Query<&mut Node>,
    children: Query<&Children>,
    mut commands: Commands,
) {
    if !rows.contains(drag.entity) && !lines.contains(drag.entity) {
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

/// The drop target `entity` is in: a pane or the deal bar.
fn target_of(
    entity: Entity,
    parents: &Query<&ChildOf>,
    panes: &Query<&TradePaneRoot>,
    bars: &Query<(), With<TradeBarRoot>>,
) -> Option<DropTarget> {
    let mut current = entity;
    loop {
        if let Ok(pane) = panes.get(current) {
            return Some(DropTarget::Pane(pane.0));
        }
        if bars.contains(current) {
            return Some(DropTarget::Bar);
        }
        current = parents.get(current).ok()?.parent();
    }
}

/// Dropped on the bar or a pane: the view decides (stage, unstage, or nothing).
#[allow(clippy::too_many_arguments)]
pub(crate) fn on_trade_drop(
    drop: On<PointerDragDrop>,
    rows: Query<&TradeRowPart>,
    lines: Query<&TradeLinePart>,
    panes: Query<&TradePaneRoot>,
    bars: Query<(), With<TradeBarRoot>>,
    parents: Query<&ChildOf>,
    mut screens: Query<&mut TradeScreen>,
    mut out: MessageWriter<ServiceScreenEvent>,
) {
    if !rows.contains(drop.dropped) && !lines.contains(drop.dropped) {
        return;
    }
    let Some(root) = screen_of(drop.entity, &parents, |e| screens.contains(e)) else {
        return;
    };
    let Some(target) = target_of(drop.entity, &parents, &panes, &bars) else {
        return;
    };
    let Ok(mut screen) = screens.get_mut(root) else {
        return;
    };
    if screen.view.held().is_none() {
        return;
    }
    if let Some(request) = screen.view.drop_on(target) {
        send(&mut out, root, request);
    }
}

/// Drag over: nothing held any more.
pub(crate) fn on_trade_drag_end(
    end: On<PointerDragEnd>,
    rows: Query<&TradeRowPart>,
    lines: Query<&TradeLinePart>,
    parents: Query<&ChildOf>,
    mut screens: Query<&mut TradeScreen>,
) {
    if !rows.contains(end.entity) && !lines.contains(end.entity) {
        return;
    }
    if let Some(root) = screen_of(end.entity, &parents, |e| screens.contains(e))
        && let Ok(mut screen) = screens.get_mut(root)
        && screen.view.held().is_some()
    {
        screen.view.release();
    }
}

/// The drag ghost is a window-level node: it goes with its screen.
pub(crate) fn on_trade_removed(
    removed: On<Remove<TradeParts>>,
    parts: Query<&TradeParts>,
    mut commands: Commands,
) {
    if let Ok(parts) = parts.get(removed.entity)
        && let Ok(mut ghost) = commands.get_entity(parts.ghost)
    {
        ghost.despawn();
    }
}
