//! The trade screen's view-model (services.md "Screens"): the container screen's two
//! [`InventoryView`] panes (provider stock, player items), a price column from the host's
//! quotes, and the deal bar (staged lines per side, totals, gold, balance, haggle stepper,
//! Accept, Cancel).
//!
//! The view never changes a session: every input yields at most one [`ServiceRequest`] the
//! host applies on its tick (net-ready: the same requests travel from a co-op guest). The
//! host answers with [`ServiceEventKind`]s ([`TradeView::apply_event`]): `TradeQuoted`
//! replaces the deal bar, `TradeDone` clears it, `TradeRefused` raises a toast. Item moves
//! come back as `RpgEvent`s ([`TradeView::on_rpg_event`]) and the screen re-syncs the pane.
//!
//! Input (after the container screen): A or a click stages the focused stack whole (or
//! unstages the focused bar line whole), X asks how many first, a drag onto the bar or the
//! other pane stages, a bar line dragged back to its pane unstages. LB/RB pick a pane, the
//! bar has its own focus (lines, the offer stepper, Accept, Cancel). B closes the count
//! dialog, then drops a drag, then leaves the bar, then cancels the trade.
//!
//! Currency stacks are hidden from both panes (money moves only through the balance) and
//! a stack the host did not price is greyed (the provider does not trade it), never staged.

use std::collections::BTreeMap;

use sf_rpg::service::{
    MerchantMoney, ServiceEventKind, ServiceRequest, SessionId, Side, TradeConfig, TradeLine,
    TradeRefusal,
};
use sf_rpg::{Holder, Loadout, RecordId, RpgEvent, SlotTable, StackId};
use sf_ui_inventory::{InventoryView, ItemCatalog, QuantityPicker, SyncReport};

/// One of the two panes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TradePane {
    /// The provider's stock: staging from it buys ([`Side::Buy`]).
    Provider,
    /// The player's items: staging from it sells ([`Side::Sell`]).
    Player,
}

impl TradePane {
    /// The deal side a stack staged from this pane goes on.
    pub fn side(self) -> Side {
        match self {
            Self::Provider => Side::Buy,
            Self::Player => Side::Sell,
        }
    }

    /// The pane a line of `side` was staged from.
    pub fn of_side(side: Side) -> Self {
        match side {
            Side::Buy => Self::Provider,
            Side::Sell => Self::Player,
        }
    }

    /// The other pane.
    pub fn other(self) -> Self {
        match self {
            Self::Provider => Self::Player,
            Self::Player => Self::Provider,
        }
    }

    fn index(self) -> usize {
        match self {
            Self::Provider => 0,
            Self::Player => 1,
        }
    }
}

/// Why a pane row cannot be staged (greyed, the reason in its tooltip).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RowBlock {
    /// The host did not price it: the provider does not buy (or sell) this stack.
    NotTraded,
    /// Every piece is already on the deal bar.
    AllStaged,
}

/// One shown pane row: a stack with its trade state. Names, icons and weights come from the
/// pane's [`InventoryView`] row ([`TradeView::pane`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TradeRow {
    pub stack: StackId,
    /// Pieces not staged yet (the count the grid shows).
    pub available: u32,
    /// The host's quote for the whole stack (the price column); `None` = not traded.
    pub price: Option<i64>,
    /// Why it is greyed; `None` = it can be staged.
    pub blocked: Option<RowBlock>,
}

/// One staged line on the deal bar.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BarLine {
    /// The line's index in the session (what [`ServiceRequest::Unstage`] names).
    pub line: u32,
    pub side: Side,
    pub source: Holder,
    pub stack: StackId,
    pub item: RecordId,
    pub count: u32,
    /// Price of the whole line.
    pub price: i64,
}

/// What would stop Accept, as far as the client can tell (the host checks again).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DealProblem {
    NothingStaged,
    PlayerCantAfford { have: u64, want: u64 },
    MerchantCantAfford { have: i64, want: i64 },
}

/// The deal bar between the panes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DealBar {
    /// Lines the player buys, in session order.
    pub buy: Vec<BarLine>,
    /// Lines the player sells, in session order.
    pub sell: Vec<BarLine>,
    pub buy_total: i64,
    pub sell_total: i64,
    /// `sell_total - buy_total` as the host quoted it (positive = the provider pays).
    pub merchant_offer: i64,
    /// The player's proposal; equals `merchant_offer` unless haggled.
    pub balance: i64,
    /// Whether the offer stepper shows (`TradeConfig::haggle` and something staged).
    pub haggle: bool,
    pub haggled: bool,
    /// The player's currency count.
    pub player_gold: u64,
    /// The provider's money; `None` = unlimited.
    pub merchant_gold: Option<i64>,
    /// The player's gold if the deal settles at `balance`.
    pub player_after: i64,
    pub problem: Option<DealProblem>,
}

/// What the count dialog will stage or unstage.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CountTarget {
    Stage { pane: TradePane, stack: StackId },
    Unstage { line: u32 },
}

/// The open count dialog ("how many?").
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CountDialog {
    pub target: CountTarget,
    pub picker: QuantityPicker,
}

/// Keyboard and gamepad focus.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TradeFocus {
    /// A pane; the row is that [`InventoryView`]'s focus.
    Pane(TradePane),
    /// A deal-bar line by session index.
    Line(u32),
    /// The haggle stepper on the balance.
    Offer,
    Accept,
    Cancel,
}

/// What a drag holds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Held {
    Stack { pane: TradePane, stack: StackId },
    Line(u32),
}

/// Where a drag ends.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DropTarget {
    Pane(TradePane),
    Bar,
}

/// A notice on the deal bar.
#[derive(Clone, Debug, PartialEq)]
pub enum TradeToast {
    /// The host refused a staging change or the deal ("not enough gold", the haggle line).
    Refused(TradeRefusal),
    /// The deal settled at this balance.
    Done { balance: i64 },
}

/// Gamepad and keyboard input, already mapped from the device by the screen.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TradeNav {
    /// Moves focus by `n` items in the focused region (the screen turns grid rows into a
    /// row count); clamped, no wrap.
    Move(isize),
    /// LB/RB: focus a pane.
    Pane(TradePane),
    /// Focus the other pane.
    SwitchPane,
    /// Focus the deal bar.
    Bar,
    /// A: stage the focused stack whole, unstage the focused line whole, press a button.
    Confirm,
    /// X: ask how many first.
    Quantity,
    /// Left/right: the count dialog by 1, or the offer by 1 (positive = in the player's
    /// favour).
    Step(i64),
    /// Shoulders while stepping: the count dialog by 10, or the offer by 10% of the deal.
    StepBig(i32),
    /// Back to the provider's offer.
    ResetOffer,
    /// Start: accept the deal.
    Accept,
    /// B.
    Back,
}

/// The trade screen's state. Plain data; the screen binds it and forwards its requests.
pub struct TradeView {
    session: SessionId,
    panes: [InventoryView; 2],
    /// The host's whole-stack quotes per pane (`pane_prices`); missing = not priced yet.
    prices: [BTreeMap<StackId, Option<i64>>; 2],
    currency: RecordId,
    haggle: bool,
    lines: Vec<TradeLine>,
    merchant_offer: i64,
    balance: i64,
    money: MerchantMoney,
    focus: TradeFocus,
    last_pane: TradePane,
    count: Option<CountDialog>,
    held: Option<Held>,
    toast: Option<TradeToast>,
    accepting: bool,
    closed: bool,
}

/// `v` with the sign of the deal direction (the host's rule: the player can't turn "I pay"
/// into "you pay").
fn clamp_sign(v: i64, offer: i64) -> i64 {
    match offer.signum() {
        1 => v.max(0),
        -1 => v.min(0),
        _ => 0,
    }
}

impl TradeView {
    /// A view of an opened trade session over the provider's stock pane and the player's
    /// pane (each already built for its holder; sync them before showing). `money` is the
    /// provider's money as of the open (`PurseRestocked` updates it).
    pub fn new(
        session: SessionId,
        provider: InventoryView,
        player: InventoryView,
        config: &TradeConfig,
        money: MerchantMoney,
    ) -> Self {
        Self {
            session,
            panes: [provider, player],
            prices: [BTreeMap::new(), BTreeMap::new()],
            currency: config.currency.item.clone(),
            haggle: config.haggle,
            lines: Vec::new(),
            merchant_offer: 0,
            balance: 0,
            money,
            focus: TradeFocus::Pane(TradePane::Provider),
            last_pane: TradePane::Provider,
            count: None,
            held: None,
            toast: None,
            accepting: false,
            closed: false,
        }
    }

    pub fn session(&self) -> SessionId {
        self.session
    }

    /// Whether the view sent `Cancel` or the host closed the session (a veto).
    pub fn is_closed(&self) -> bool {
        self.closed
    }

    /// Whether an `Accept` is waiting for the host's answer.
    pub fn is_accepting(&self) -> bool {
        self.accepting
    }

    pub fn pane(&self, pane: TradePane) -> &InventoryView {
        &self.panes[pane.index()]
    }

    /// The pane's inventory view (tabs, search, sort, grid scroll).
    pub fn pane_mut(&mut self, pane: TradePane) -> &mut InventoryView {
        &mut self.panes[pane.index()]
    }

    pub fn focus(&self) -> TradeFocus {
        self.focus
    }

    pub fn count_dialog(&self) -> Option<&CountDialog> {
        self.count.as_ref()
    }

    pub fn held(&self) -> Option<Held> {
        self.held
    }

    pub fn toast(&self) -> Option<&TradeToast> {
        self.toast.as_ref()
    }

    /// Takes the toast (the screen shows it once).
    pub fn take_toast(&mut self) -> Option<TradeToast> {
        self.toast.take()
    }

    /// The staged lines as the host last quoted them.
    pub fn lines(&self) -> &[TradeLine] {
        &self.lines
    }

    // ------------------------------------------------------------------ sync and quotes

    /// Re-syncs one pane from its holder's loadout (after `ItemsChanged`).
    pub fn sync(
        &mut self,
        pane: TradePane,
        loadout: &Loadout,
        table: &SlotTable,
        catalog: &dyn ItemCatalog,
    ) -> SyncReport {
        let report = self.panes[pane.index()].sync(loadout, table, catalog);
        self.fix_focus(pane);
        report
    }

    /// Replaces one pane's price column with the host's quotes (`pane_prices`: `None`
    /// where the stack is not traded).
    pub fn set_prices(&mut self, pane: TradePane, prices: impl IntoIterator<Item = (StackId, Option<i64>)>) {
        self.prices[pane.index()] = prices.into_iter().collect();
    }

    /// The host's quote for a whole stack; `None` = not traded (or not priced yet).
    pub fn price(&self, pane: TradePane, stack: StackId) -> Option<i64> {
        self.prices[pane.index()].get(&stack).copied().flatten()
    }

    fn is_currency(&self, pane: TradePane, stack: StackId) -> bool {
        self.pane(pane).row(stack).is_some_and(|r| r.stack.item == self.currency)
    }

    /// Pieces of a pane stack on the deal bar.
    pub fn staged(&self, pane: TradePane, stack: StackId) -> u32 {
        let holder = self.pane(pane).holder();
        self.lines
            .iter()
            .filter(|l| l.side == pane.side() && l.source == holder && l.stack == stack)
            .fold(0, |n, l| n.saturating_add(l.count))
    }

    /// Pieces of a pane stack not staged yet (0 for an unknown stack).
    pub fn available(&self, pane: TradePane, stack: StackId) -> u32 {
        self.pane(pane)
            .row(stack)
            .map_or(0, |r| r.stack.count.saturating_sub(self.staged(pane, stack)))
    }

    fn block(&self, pane: TradePane, stack: StackId) -> Option<RowBlock> {
        if self.price(pane, stack).is_none() {
            Some(RowBlock::NotTraded)
        } else if self.available(pane, stack) == 0 {
            Some(RowBlock::AllStaged)
        } else {
            None
        }
    }

    /// Stack ids a pane shows: the inventory view's tab, search and sort, minus currency.
    pub fn shown(&mut self, pane: TradePane) -> Vec<StackId> {
        let currency = self.currency.clone();
        let view = &mut self.panes[pane.index()];
        let keys: Vec<StackId> = view.visible().to_vec();
        keys.into_iter()
            .filter(|k| view.row(*k).is_some_and(|r| r.stack.item != currency))
            .collect()
    }

    /// The shown rows of a pane with their trade state.
    pub fn rows(&mut self, pane: TradePane) -> Vec<TradeRow> {
        self.shown(pane)
            .into_iter()
            .map(|stack| TradeRow {
                stack,
                available: self.available(pane, stack),
                price: self.price(pane, stack),
                blocked: self.block(pane, stack),
            })
            .collect()
    }

    fn currency_in(&self, pane: TradePane) -> u64 {
        self.pane(pane)
            .rows()
            .iter()
            .filter(|(_, r)| r.stack.item == self.currency)
            .map(|(_, r)| u64::from(r.stack.count))
            .sum()
    }

    /// The player's money: their currency stacks.
    pub fn player_gold(&self) -> u64 {
        self.currency_in(TradePane::Player)
    }

    /// The provider's money; `None` = unlimited.
    pub fn merchant_gold(&self) -> Option<i64> {
        match self.money {
            MerchantMoney::Pool(gold) => Some(gold),
            MerchantMoney::Unlimited => None,
            MerchantMoney::Inventory => Some(i64::try_from(self.currency_in(TradePane::Provider)).unwrap_or(i64::MAX)),
        }
    }

    /// The deal bar model.
    pub fn deal(&self) -> DealBar {
        let bar_line = |(i, l): (usize, &TradeLine)| BarLine {
            line: u32::try_from(i).unwrap_or(u32::MAX),
            side: l.side,
            source: l.source,
            stack: l.stack,
            item: l.item.clone(),
            count: l.count,
            price: l.price,
        };
        let side = |s: Side| -> Vec<BarLine> {
            self.lines.iter().enumerate().filter(|(_, l)| l.side == s).map(bar_line).collect()
        };
        let (buy, sell) = (side(Side::Buy), side(Side::Sell));
        let total = |v: &[BarLine]| v.iter().map(|l| l.price).fold(0, i64::saturating_add);
        let player_gold = self.player_gold();
        let merchant_gold = self.merchant_gold();
        DealBar {
            buy_total: total(&buy),
            sell_total: total(&sell),
            buy,
            sell,
            merchant_offer: self.merchant_offer,
            balance: self.balance,
            haggle: self.haggle && !self.lines.is_empty(),
            haggled: self.balance != self.merchant_offer,
            player_gold,
            merchant_gold,
            player_after: i64::try_from(player_gold).unwrap_or(i64::MAX).saturating_add(self.balance),
            problem: self.problem(player_gold, merchant_gold),
        }
    }

    /// The host's money checks, in its order (`NothingStaged`, player, provider).
    fn problem(&self, player_gold: u64, merchant_gold: Option<i64>) -> Option<DealProblem> {
        if self.lines.is_empty() {
            return Some(DealProblem::NothingStaged);
        }
        let want = self.balance.unsigned_abs();
        if self.balance < 0 && player_gold < want {
            return Some(DealProblem::PlayerCantAfford { have: player_gold, want });
        }
        match merchant_gold {
            Some(have) if self.balance > 0 && have < self.balance => {
                Some(DealProblem::MerchantCantAfford { have, want: self.balance })
            }
            _ => None,
        }
    }

    // ------------------------------------------------------------------ host answers

    /// Applies one host event for this player and provider (the caller routes them).
    /// Returns whether anything the screen shows changed.
    pub fn apply_event(&mut self, event: &ServiceEventKind) -> bool {
        match event {
            ServiceEventKind::TradeQuoted { session, lines, merchant_offer, balance } if *session == self.session => {
                self.lines.clone_from(lines);
                self.merchant_offer = *merchant_offer;
                self.balance = *balance;
                self.fix_bar();
                true
            }
            ServiceEventKind::TradeDone { session, balance } if *session == self.session => {
                if let MerchantMoney::Pool(gold) = self.money {
                    self.money = MerchantMoney::Pool(gold.saturating_sub(*balance));
                }
                self.lines.clear();
                self.merchant_offer = 0;
                self.balance = 0;
                self.accepting = false;
                self.toast = Some(TradeToast::Done { balance: *balance });
                self.fix_bar();
                true
            }
            ServiceEventKind::TradeRefused { session, reason } if *session == self.session => {
                self.accepting = false;
                if reason.closes() {
                    self.closed = true;
                }
                self.toast = Some(TradeToast::Refused(reason.clone()));
                true
            }
            ServiceEventKind::PurseRestocked { gold } => {
                self.money = MerchantMoney::Pool(*gold);
                true
            }
            _ => false,
        }
    }

    /// Follows an item event (focus after a split or merge). Returns the pane whose holder
    /// changed, which the screen re-syncs.
    pub fn on_rpg_event(&mut self, event: &RpgEvent) -> Option<TradePane> {
        let provider = self.panes[0].on_event(event);
        let player = self.panes[1].on_event(event);
        if provider {
            Some(TradePane::Provider)
        } else if player {
            Some(TradePane::Player)
        } else {
            None
        }
    }

    /// Keeps bar focus, a held line and the count dialog on lines that still exist.
    fn fix_bar(&mut self) {
        let len = self.lines.len();
        let gone = |line: u32| usize::try_from(line).ok().is_none_or(|l| l >= len);
        match self.focus {
            TradeFocus::Line(l) if gone(l) => self.focus = self.bar_items().first().copied().unwrap_or(TradeFocus::Accept),
            TradeFocus::Offer if !self.haggle || len == 0 => self.focus = TradeFocus::Accept,
            _ => {}
        }
        if matches!(self.held, Some(Held::Line(l)) if gone(l)) {
            self.held = None;
        }
        if matches!(self.count, Some(CountDialog { target: CountTarget::Unstage { line }, .. }) if gone(line)) {
            self.count = None;
        }
    }

    /// Moves a pane's row focus off a currency stack (after a sync).
    fn fix_focus(&mut self, pane: TradePane) {
        let Some(stack) = self.pane(pane).focus() else {
            return;
        };
        if self.is_currency(pane, stack) {
            let first = self.shown(pane).first().copied();
            self.panes[pane.index()].set_focus(first);
        }
    }

    // ------------------------------------------------------------------ staging

    fn holder(&self, pane: TradePane) -> Holder {
        self.pane(pane).holder()
    }

    /// Whether a stack can be staged at all: shown, priced, not currency, session open.
    fn tradable(&self, pane: TradePane, stack: StackId) -> bool {
        !self.closed && !self.is_currency(pane, stack) && self.price(pane, stack).is_some()
    }

    fn stage_request(&self, pane: TradePane, stack: StackId, count: u32) -> Option<ServiceRequest> {
        if !self.tradable(pane, stack) {
            return None;
        }
        let count = count.min(self.available(pane, stack));
        (count > 0).then(|| ServiceRequest::Stage {
            session: self.session,
            side: pane.side(),
            source: self.holder(pane),
            stack,
            count,
        })
    }

    fn line(&self, line: u32) -> Option<&TradeLine> {
        self.lines.get(usize::try_from(line).ok()?)
    }

    fn unstage_request(&self, line: u32, count: u32) -> Option<ServiceRequest> {
        let l = self.line(line)?;
        let count = count.min(l.count);
        (!self.closed && count > 0).then(|| ServiceRequest::Unstage { session: self.session, line, count })
    }

    /// A or a click on a pane row: stages every piece not staged yet.
    pub fn activate_row(&mut self, pane: TradePane, stack: StackId) -> Option<ServiceRequest> {
        self.focus_row(pane, stack);
        self.stage_request(pane, stack, u32::MAX)
    }

    /// X on a pane row: opens the count dialog (all pieces preselected), or stages a single
    /// piece directly.
    pub fn ask_stage_count(&mut self, pane: TradePane, stack: StackId) -> Option<ServiceRequest> {
        let available = self.available(pane, stack);
        if !self.tradable(pane, stack) || available == 0 {
            return None;
        }
        if available == 1 {
            return self.stage_request(pane, stack, 1);
        }
        self.count = QuantityPicker::new(1, available, available)
            .map(|picker| CountDialog { target: CountTarget::Stage { pane, stack }, picker });
        None
    }

    /// A or a click on a bar line: unstages it whole.
    pub fn activate_line(&mut self, line: u32) -> Option<ServiceRequest> {
        if self.line(line).is_some() {
            self.focus = TradeFocus::Line(line);
        }
        self.unstage_request(line, u32::MAX)
    }

    /// X on a bar line: opens the count dialog, or unstages a single piece directly.
    pub fn ask_unstage_count(&mut self, line: u32) -> Option<ServiceRequest> {
        let count = self.line(line)?.count;
        if self.closed {
            return None;
        }
        if count == 1 {
            return self.unstage_request(line, 1);
        }
        self.count = QuantityPicker::new(1, count, count)
            .map(|picker| CountDialog { target: CountTarget::Unstage { line }, picker });
        None
    }

    /// The count dialog's slider or spinner (pointer input).
    pub fn set_count(&mut self, value: u32) {
        if let Some(dialog) = &mut self.count {
            dialog.picker.set(value);
        }
    }

    /// OK in the count dialog.
    pub fn confirm_count(&mut self) -> Option<ServiceRequest> {
        let dialog = self.count.take()?;
        let n = dialog.picker.value();
        match dialog.target {
            CountTarget::Stage { pane, stack } => self.stage_request(pane, stack, n),
            CountTarget::Unstage { line } => self.unstage_request(line, n),
        }
    }

    /// Cancel in the count dialog.
    pub fn close_count(&mut self) {
        self.count = None;
    }

    // ------------------------------------------------------------------ drag

    /// Drag start on a pane row; `false` (nothing held) for a stack that can't be staged.
    pub fn grab_stack(&mut self, pane: TradePane, stack: StackId) -> bool {
        let ok = self.tradable(pane, stack) && self.available(pane, stack) > 0;
        self.held = ok.then_some(Held::Stack { pane, stack });
        ok
    }

    /// Drag start on a bar line.
    pub fn grab_line(&mut self, line: u32) -> bool {
        let ok = !self.closed && self.line(line).is_some();
        self.held = ok.then_some(Held::Line(line));
        ok
    }

    /// Drop: a stack onto the bar or the other pane stages it whole; a line onto the pane
    /// it came from unstages it whole. Anything else just ends the drag.
    pub fn drop_on(&mut self, target: DropTarget) -> Option<ServiceRequest> {
        match (self.held.take()?, target) {
            (Held::Stack { pane, stack }, DropTarget::Bar) => self.stage_request(pane, stack, u32::MAX),
            (Held::Stack { pane, stack }, DropTarget::Pane(to)) if to != pane => {
                self.stage_request(pane, stack, u32::MAX)
            }
            (Held::Line(line), DropTarget::Pane(to)) if self.line(line).is_some_and(|l| TradePane::of_side(l.side) == to) => {
                self.unstage_request(line, u32::MAX)
            }
            _ => None,
        }
    }

    /// Drag cancelled (released outside any target).
    pub fn release(&mut self) {
        self.held = None;
    }

    // ------------------------------------------------------------------ haggle, accept, cancel

    /// Proposes a balance (the offer stepper), sign-clamped like the host. The view shows it
    /// at once; the next `TradeQuoted` overwrites it. `None` when haggling is off, nothing
    /// is staged or the balance would not change.
    pub fn set_offer(&mut self, balance: i64) -> Option<ServiceRequest> {
        if !self.haggle || self.closed || self.lines.is_empty() {
            return None;
        }
        let balance = clamp_sign(balance, self.merchant_offer);
        if balance == self.balance {
            return None;
        }
        self.balance = balance;
        Some(ServiceRequest::SetBalance { session: self.session, balance })
    }

    /// Steps the offer by `delta` (positive = more for the player).
    pub fn step_offer(&mut self, delta: i64) -> Option<ServiceRequest> {
        self.set_offer(self.balance.saturating_add(delta))
    }

    /// Steps the offer by `steps` tenths of the provider's offer (at least 1 each).
    pub fn step_offer_percent(&mut self, steps: i32) -> Option<ServiceRequest> {
        let unit = i64::try_from((self.merchant_offer.unsigned_abs() / 10).max(1)).unwrap_or(i64::MAX);
        self.step_offer(unit.saturating_mul(i64::from(steps)))
    }

    /// Back to the provider's offer.
    pub fn reset_offer(&mut self) -> Option<ServiceRequest> {
        self.set_offer(self.merchant_offer)
    }

    /// Accept. Nothing staged gives a local `NothingStaged` toast and no request; a second
    /// press while the host has not answered is ignored. Money is the host's to check.
    pub fn accept(&mut self) -> Option<ServiceRequest> {
        if self.closed || self.accepting {
            return None;
        }
        if self.lines.is_empty() {
            self.toast = Some(TradeToast::Refused(TradeRefusal::NothingStaged));
            return None;
        }
        self.accepting = true;
        Some(ServiceRequest::Accept { session: self.session })
    }

    /// Cancel: closes the trade (nothing was moved).
    pub fn cancel(&mut self) -> Option<ServiceRequest> {
        if self.closed {
            return None;
        }
        self.closed = true;
        self.count = None;
        self.held = None;
        Some(ServiceRequest::Cancel { session: self.session })
    }

    // ------------------------------------------------------------------ focus and navigation

    /// Hover or click: focuses a pane row.
    pub fn focus_row(&mut self, pane: TradePane, stack: StackId) {
        if self.pane(pane).row(stack).is_some() && !self.is_currency(pane, stack) {
            self.focus = TradeFocus::Pane(pane);
            self.last_pane = pane;
            self.panes[pane.index()].set_focus(Some(stack));
        }
    }

    /// Focus a pane (its row focus is kept).
    pub fn focus_pane(&mut self, pane: TradePane) {
        self.focus = TradeFocus::Pane(pane);
        self.last_pane = pane;
    }

    /// The bar's focusable items in order: buy lines, sell lines, the offer stepper, Accept,
    /// Cancel.
    pub fn bar_items(&self) -> Vec<TradeFocus> {
        let mut items: Vec<TradeFocus> = Vec::with_capacity(self.lines.len() + 3);
        for side in [Side::Buy, Side::Sell] {
            items.extend(
                self.lines
                    .iter()
                    .enumerate()
                    .filter(|(_, l)| l.side == side)
                    .filter_map(|(i, _)| u32::try_from(i).ok().map(TradeFocus::Line)),
            );
        }
        if self.haggle && !self.lines.is_empty() {
            items.push(TradeFocus::Offer);
        }
        items.push(TradeFocus::Accept);
        items.push(TradeFocus::Cancel);
        items
    }

    /// The focused pane stack, if a pane has focus.
    pub fn focused_stack(&self) -> Option<(TradePane, StackId)> {
        let TradeFocus::Pane(pane) = self.focus else {
            return None;
        };
        self.pane(pane).focus().filter(|s| !self.is_currency(pane, *s)).map(|s| (pane, s))
    }

    fn move_focus(&mut self, delta: isize) {
        match self.focus {
            TradeFocus::Pane(pane) => {
                let shown = self.shown(pane);
                let current = self.pane(pane).focus().and_then(|f| shown.iter().position(|s| *s == f));
                let next = match current {
                    Some(i) => i.saturating_add_signed(delta).min(shown.len().saturating_sub(1)),
                    None => 0,
                };
                let key = shown.get(next).copied();
                self.panes[pane.index()].set_focus(key);
            }
            bar => {
                let items = self.bar_items();
                let i = items.iter().position(|f| *f == bar).unwrap_or(0);
                let next = i.saturating_add_signed(delta).min(items.len().saturating_sub(1));
                if let Some(f) = items.get(next) {
                    self.focus = *f;
                }
            }
        }
    }

    fn confirm_focus(&mut self) -> Option<ServiceRequest> {
        match self.focus {
            TradeFocus::Pane(_) => {
                let (pane, stack) = self.focused_stack()?;
                self.activate_row(pane, stack)
            }
            TradeFocus::Line(line) => self.activate_line(line),
            TradeFocus::Offer => None,
            TradeFocus::Accept => self.accept(),
            TradeFocus::Cancel => self.cancel(),
        }
    }

    fn quantity_focus(&mut self) -> Option<ServiceRequest> {
        match self.focus {
            TradeFocus::Pane(_) => {
                let (pane, stack) = self.focused_stack()?;
                self.ask_stage_count(pane, stack)
            }
            TradeFocus::Line(line) => self.ask_unstage_count(line),
            _ => None,
        }
    }

    /// Gamepad and keyboard input. See the module docs for the mapping.
    pub fn nav(&mut self, nav: TradeNav) -> Option<ServiceRequest> {
        if self.closed {
            return None;
        }
        if self.count.is_some() {
            let step = match nav {
                TradeNav::Step(d) => d,
                TradeNav::StepBig(s) => i64::from(s).saturating_mul(10),
                TradeNav::Confirm => return self.confirm_count(),
                TradeNav::Back => {
                    self.count = None;
                    return None;
                }
                _ => return None,
            };
            if let Some(dialog) = &mut self.count {
                dialog.picker.step(step);
            }
            return None;
        }
        match nav {
            TradeNav::Move(d) => self.move_focus(d),
            TradeNav::Pane(pane) => self.focus_pane(pane),
            TradeNav::SwitchPane => self.focus_pane(self.last_pane.other()),
            TradeNav::Bar => {
                if matches!(self.focus, TradeFocus::Pane(_)) {
                    self.focus = self.bar_items().first().copied().unwrap_or(TradeFocus::Accept);
                }
            }
            TradeNav::Confirm => return self.confirm_focus(),
            TradeNav::Quantity => return self.quantity_focus(),
            TradeNav::Step(d) if self.focus == TradeFocus::Offer => return self.step_offer(d),
            TradeNav::StepBig(s) if self.focus == TradeFocus::Offer => return self.step_offer_percent(s),
            TradeNav::Step(_) | TradeNav::StepBig(_) => {}
            TradeNav::ResetOffer => return self.reset_offer(),
            TradeNav::Accept => return self.accept(),
            TradeNav::Back => {
                if self.held.take().is_none() {
                    if matches!(self.focus, TradeFocus::Pane(_)) {
                        return self.cancel();
                    }
                    self.focus = TradeFocus::Pane(self.last_pane);
                }
            }
        }
        None
    }
}
