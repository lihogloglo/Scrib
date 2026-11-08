//! Trade view-model: a synthetic merchant and player, host answers written by hand.

use sf_rpg::service::{
    Currency, DealVeto, LabelRef, MerchantMoney, ServiceEventKind, ServiceRequest, SessionId, Side, TradeConfig,
    TradeLine, TradeRefusal,
};
use sf_rpg::{ActorKey, Attrs, Holder, ItemStack, Loadout, PlayerId, RecordId, SlotTable, StackId};
use sf_ui_inventory::{InventoryView, ItemCatalog, ItemInfo, StatDef};

use super::*;

fn id(s: &str) -> RecordId {
    RecordId::new(s)
}

const MERCHANT: ActorKey = ActorKey(7);
const P1: PlayerId = PlayerId::HOST;
const S: SessionId = SessionId(3);

fn merchant() -> Holder {
    Holder::Actor(MERCHANT)
}

fn player() -> Holder {
    Holder::Player(P1)
}

struct Catalog;

impl ItemCatalog for Catalog {
    fn describe(&self, stack: &ItemStack) -> ItemInfo {
        ItemInfo {
            name: stack.item.as_str().to_owned(),
            category: id("misc"),
            weight: 1.0,
            value: 10,
            icon: None,
            stats: Vec::new(),
        }
    }

    fn stats(&self) -> &[StatDef] {
        &[]
    }
}

struct Fx {
    view: TradeView,
    stock: Loadout,
    pack: Loadout,
    table: SlotTable,
    potion: StackId,
    helm: StackId,
    sword: StackId,
    arrow: StackId,
}

fn add(lo: &mut Loadout, item: &str, count: u32) -> StackId {
    lo.add(id(item), count, Attrs::new()).expect("add")
}

/// Merchant: 50 gold, 5 potions, 1 helm. Player: 100 gold, 1 sword, 20 arrows. The host
/// priced potions (10 each), the helm (50) and the sword (30); arrows are not traded.
fn fx_with(haggle: bool, money: MerchantMoney) -> Fx {
    let mut stock = Loadout::new();
    add(&mut stock, "gold", 50);
    let potion = add(&mut stock, "potion", 5);
    let helm = add(&mut stock, "helm", 1);
    let mut pack = Loadout::new();
    let p_gold = add(&mut pack, "gold", 100);
    let sword = add(&mut pack, "sword", 1);
    let arrow = add(&mut pack, "arrow", 20);
    let config = TradeConfig { currency: Currency { item: id("gold") }, haggle };
    let view = TradeView::new(
        S,
        InventoryView::new(merchant(), Vec::new(), 10.0, 100.0),
        InventoryView::new(player(), Vec::new(), 10.0, 100.0),
        &config,
        money,
    );
    let mut fx = Fx { view, stock, pack, table: SlotTable::new(["hand"]).expect("slots"), potion, helm, sword, arrow };
    fx.view.sync(TradePane::Provider, &fx.stock, &fx.table, &Catalog);
    fx.view.sync(TradePane::Player, &fx.pack, &fx.table, &Catalog);
    fx.view.set_prices(TradePane::Provider, [(potion, Some(50)), (helm, Some(50))]);
    fx.view.set_prices(TradePane::Player, [(p_gold, None), (sword, Some(30)), (arrow, None)]);
    fx
}

fn fx() -> Fx {
    fx_with(true, MerchantMoney::Pool(200))
}

fn line(side: Side, source: Holder, stack: StackId, item: &str, count: u32, price: i64) -> TradeLine {
    TradeLine { side, source, stack, item: id(item), attrs: Attrs::new(), count, price }
}

fn quote(view: &mut TradeView, lines: Vec<TradeLine>, merchant_offer: i64, balance: i64) {
    assert!(view.apply_event(&ServiceEventKind::TradeQuoted { session: S, lines, merchant_offer, balance }));
}

fn stage(side: Side, source: Holder, stack: StackId, count: u32) -> Option<ServiceRequest> {
    Some(ServiceRequest::Stage { session: S, side, source, stack, count })
}

#[test]
fn currency_is_hidden_and_unpriced_stacks_are_greyed() {
    let mut f = fx();
    let rows = f.view.rows(TradePane::Player);
    assert_eq!(
        rows,
        vec![
            TradeRow { stack: f.sword, available: 1, price: Some(30), blocked: None },
            TradeRow { stack: f.arrow, available: 20, price: None, blocked: Some(RowBlock::NotTraded) },
        ]
    );
    assert_eq!(f.view.rows(TradePane::Provider).len(), 2);
    assert_eq!(f.view.player_gold(), 100);
    assert_eq!(f.view.merchant_gold(), Some(200));
    // An unpriced stack is never staged, whatever the input.
    assert_eq!(f.view.activate_row(TradePane::Player, f.arrow), None);
    assert_eq!(f.view.ask_stage_count(TradePane::Player, f.arrow), None);
    assert!(!f.view.grab_stack(TradePane::Player, f.arrow));
    assert!(f.view.count_dialog().is_none());
}

#[test]
fn merchant_money_modes() {
    let f = fx_with(false, MerchantMoney::Inventory);
    assert_eq!(f.view.merchant_gold(), Some(50));
    let f = fx_with(false, MerchantMoney::Unlimited);
    assert_eq!(f.view.merchant_gold(), None);
}

#[test]
fn a_stages_the_whole_stack_and_x_asks_how_many() {
    let mut f = fx();
    assert_eq!(f.view.activate_row(TradePane::Provider, f.potion), stage(Side::Buy, merchant(), f.potion, 5));
    assert_eq!(f.view.focused_stack(), Some((TradePane::Provider, f.potion)));

    assert_eq!(f.view.nav(TradeNav::Quantity), None);
    let dialog = f.view.count_dialog().copied().expect("dialog");
    assert_eq!(dialog.target, CountTarget::Stage { pane: TradePane::Provider, stack: f.potion });
    assert_eq!((dialog.picker.min(), dialog.picker.max(), dialog.picker.value()), (1, 5, 5));
    assert_eq!(f.view.nav(TradeNav::Step(-2)), None);
    assert_eq!(f.view.count_dialog().map(|d| d.picker.value()), Some(3));
    // Other input does nothing while the dialog is open.
    assert_eq!(f.view.nav(TradeNav::Accept), None);
    assert_eq!(f.view.nav(TradeNav::Confirm), stage(Side::Buy, merchant(), f.potion, 3));
    assert!(f.view.count_dialog().is_none());

    // A single piece needs no dialog.
    assert_eq!(f.view.ask_stage_count(TradePane::Player, f.sword), stage(Side::Sell, player(), f.sword, 1));
    assert!(f.view.count_dialog().is_none());
}

#[test]
fn staged_pieces_leave_the_pane_count() {
    let mut f = fx();
    quote(&mut f.view, vec![line(Side::Buy, merchant(), f.potion, "potion", 3, 30)], -30, -30);
    assert_eq!(f.view.staged(TradePane::Provider, f.potion), 3);
    assert_eq!(f.view.available(TradePane::Provider, f.potion), 2);
    assert_eq!(f.view.activate_row(TradePane::Provider, f.potion), stage(Side::Buy, merchant(), f.potion, 2));

    quote(&mut f.view, vec![line(Side::Buy, merchant(), f.potion, "potion", 5, 50)], -50, -50);
    let row = f.view.rows(TradePane::Provider).into_iter().find(|r| r.stack == f.potion).expect("row");
    assert_eq!((row.available, row.blocked), (0, Some(RowBlock::AllStaged)));
    assert_eq!(f.view.activate_row(TradePane::Provider, f.potion), None);
    assert!(!f.view.grab_stack(TradePane::Provider, f.potion));
}

#[test]
fn drag_stages_onto_the_bar_or_other_pane_and_unstages_back() {
    let mut f = fx();
    assert!(f.view.grab_stack(TradePane::Player, f.sword));
    assert_eq!(f.view.drop_on(DropTarget::Pane(TradePane::Provider)), stage(Side::Sell, player(), f.sword, 1));
    assert!(f.view.held().is_none());
    assert!(f.view.grab_stack(TradePane::Provider, f.helm));
    assert_eq!(f.view.drop_on(DropTarget::Bar), stage(Side::Buy, merchant(), f.helm, 1));
    // Dropped on its own pane: nothing.
    assert!(f.view.grab_stack(TradePane::Player, f.sword));
    assert_eq!(f.view.drop_on(DropTarget::Pane(TradePane::Player)), None);

    quote(&mut f.view, vec![line(Side::Sell, player(), f.sword, "sword", 1, 30)], 30, 30);
    assert!(f.view.grab_line(0));
    assert_eq!(f.view.drop_on(DropTarget::Pane(TradePane::Provider)), None);
    assert!(f.view.grab_line(0));
    assert_eq!(
        f.view.drop_on(DropTarget::Pane(TradePane::Player)),
        Some(ServiceRequest::Unstage { session: S, line: 0, count: 1 })
    );
    assert!(!f.view.grab_line(1));
}

#[test]
fn deal_bar_splits_sides_and_totals() {
    let mut f = fx();
    quote(
        &mut f.view,
        vec![
            line(Side::Buy, merchant(), f.potion, "potion", 2, 20),
            line(Side::Sell, player(), f.sword, "sword", 1, 30),
            line(Side::Buy, merchant(), f.helm, "helm", 1, 50),
        ],
        -40,
        -40,
    );
    let bar = f.view.deal();
    assert_eq!(bar.buy.iter().map(|l| l.line).collect::<Vec<_>>(), vec![0, 2]);
    assert_eq!(bar.sell.iter().map(|l| (l.line, l.stack, l.count, l.price)).collect::<Vec<_>>(), vec![(1, f.sword, 1, 30)]);
    assert_eq!((bar.buy_total, bar.sell_total, bar.merchant_offer, bar.balance), (70, 30, -40, -40));
    assert_eq!((bar.player_gold, bar.merchant_gold, bar.player_after), (100, Some(200), 60));
    assert!(bar.haggle);
    assert!(!bar.haggled);
    assert_eq!(bar.problem, None);
}

#[test]
fn deal_bar_flags_money_problems_in_host_order() {
    let mut f = fx();
    assert_eq!(f.view.deal().problem, Some(DealProblem::NothingStaged));
    quote(&mut f.view, vec![line(Side::Buy, merchant(), f.potion, "potion", 5, 150)], -150, -150);
    assert_eq!(f.view.deal().problem, Some(DealProblem::PlayerCantAfford { have: 100, want: 150 }));

    let mut f = fx_with(false, MerchantMoney::Pool(10));
    quote(&mut f.view, vec![line(Side::Sell, player(), f.sword, "sword", 1, 30)], 30, 30);
    assert_eq!(f.view.deal().problem, Some(DealProblem::MerchantCantAfford { have: 10, want: 30 }));
    // An unlimited provider never runs out.
    let mut f = fx_with(false, MerchantMoney::Unlimited);
    quote(&mut f.view, vec![line(Side::Sell, player(), f.sword, "sword", 1, 30)], 30, 30);
    assert_eq!(f.view.deal().problem, None);
}

#[test]
fn offer_stepper_keeps_the_deal_direction() {
    let mut f = fx();
    quote(&mut f.view, vec![line(Side::Sell, player(), f.sword, "sword", 1, 30)], 30, 30);
    let bal = |balance| Some(ServiceRequest::SetBalance { session: S, balance });
    assert_eq!(f.view.bar_items(), vec![TradeFocus::Line(0), TradeFocus::Offer, TradeFocus::Accept, TradeFocus::Cancel]);
    f.view.nav(TradeNav::Bar);
    f.view.nav(TradeNav::Move(1));
    assert_eq!(f.view.focus(), TradeFocus::Offer);
    assert_eq!(f.view.nav(TradeNav::Step(5)), bal(35));
    assert_eq!(f.view.deal().balance, 35);
    assert!(f.view.deal().haggled);
    // "You pay me" can't become "I pay you".
    assert_eq!(f.view.nav(TradeNav::Step(-100)), bal(0));
    assert_eq!(f.view.nav(TradeNav::Step(-1)), None);
    assert_eq!(f.view.nav(TradeNav::ResetOffer), bal(30));
    assert_eq!(f.view.nav(TradeNav::ResetOffer), None);
    // 10% of 30 is 3.
    assert_eq!(f.view.nav(TradeNav::StepBig(1)), bal(33));
    // A buy deal (negative offer) clamps at zero from below.
    quote(&mut f.view, vec![line(Side::Buy, merchant(), f.helm, "helm", 1, 50)], -50, -50);
    assert_eq!(f.view.step_offer(80), bal(0));
    assert_eq!(f.view.step_offer(-100), bal(-100));
}

#[test]
fn haggle_off_hides_the_stepper() {
    let mut f = fx_with(false, MerchantMoney::Pool(200));
    quote(&mut f.view, vec![line(Side::Sell, player(), f.sword, "sword", 1, 30)], 30, 30);
    assert_eq!(f.view.bar_items(), vec![TradeFocus::Line(0), TradeFocus::Accept, TradeFocus::Cancel]);
    assert!(!f.view.deal().haggle);
    assert_eq!(f.view.set_offer(40), None);
    assert_eq!(f.view.nav(TradeNav::ResetOffer), None);
}

#[test]
fn accept_waits_for_the_host_and_follows_its_answer() {
    let mut f = fx();
    assert_eq!(f.view.accept(), None);
    assert_eq!(f.view.take_toast(), Some(TradeToast::Refused(TradeRefusal::NothingStaged)));

    quote(&mut f.view, vec![line(Side::Sell, player(), f.sword, "sword", 1, 30)], 30, 30);
    assert_eq!(f.view.nav(TradeNav::Accept), Some(ServiceRequest::Accept { session: S }));
    assert!(f.view.is_accepting());
    assert_eq!(f.view.accept(), None);

    let refused = TradeRefusal::MerchantCantAfford { have: 10, want: 30 };
    assert!(f.view.apply_event(&ServiceEventKind::TradeRefused { session: S, reason: refused.clone() }));
    assert!(!f.view.is_accepting());
    assert!(!f.view.is_closed());
    assert_eq!(f.view.take_toast(), Some(TradeToast::Refused(refused)));

    assert_eq!(f.view.accept(), Some(ServiceRequest::Accept { session: S }));
    assert!(f.view.apply_event(&ServiceEventKind::TradeDone { session: S, balance: 30 }));
    assert!(f.view.lines().is_empty());
    assert_eq!(f.view.merchant_gold(), Some(170));
    assert_eq!(f.view.take_toast(), Some(TradeToast::Done { balance: 30 }));
    // Another session's events are not ours.
    assert!(!f.view.apply_event(&ServiceEventKind::TradeDone { session: SessionId(9), balance: 5 }));
    assert!(f.view.apply_event(&ServiceEventKind::PurseRestocked { gold: 400 }));
    assert_eq!(f.view.merchant_gold(), Some(400));
}

#[test]
fn a_veto_closes_the_view() {
    let mut f = fx();
    quote(&mut f.view, vec![line(Side::Sell, player(), f.sword, "sword", 1, 30)], 30, 30);
    f.view.accept();
    let veto = TradeRefusal::Vetoed(DealVeto { reason: LabelRef(id("stolen")), effects: Vec::new() });
    f.view.apply_event(&ServiceEventKind::TradeRefused { session: S, reason: veto });
    assert!(f.view.is_closed());
    assert_eq!(f.view.nav(TradeNav::Back), None);
    assert_eq!(f.view.activate_row(TradePane::Player, f.sword), None);
}

#[test]
fn back_closes_dialog_then_drag_then_bar_then_trade() {
    let mut f = fx();
    f.view.ask_stage_count(TradePane::Provider, f.potion);
    assert_eq!(f.view.nav(TradeNav::Back), None);
    assert!(f.view.count_dialog().is_none());
    assert!(f.view.grab_stack(TradePane::Provider, f.potion));
    assert_eq!(f.view.nav(TradeNav::Back), None);
    assert!(f.view.held().is_none());
    f.view.nav(TradeNav::Bar);
    assert_eq!(f.view.focus(), TradeFocus::Accept);
    assert_eq!(f.view.nav(TradeNav::Back), None);
    assert_eq!(f.view.focus(), TradeFocus::Pane(TradePane::Provider));
    assert_eq!(f.view.nav(TradeNav::Back), Some(ServiceRequest::Cancel { session: S }));
    assert!(f.view.is_closed());
    assert_eq!(f.view.cancel(), None);
}

#[test]
fn bar_lists_buys_then_sells_and_survives_requotes() {
    let mut f = fx();
    quote(
        &mut f.view,
        vec![
            line(Side::Sell, player(), f.sword, "sword", 1, 30),
            line(Side::Buy, merchant(), f.potion, "potion", 4, 40),
        ],
        -10,
        -10,
    );
    assert_eq!(
        f.view.bar_items(),
        vec![TradeFocus::Line(1), TradeFocus::Line(0), TradeFocus::Offer, TradeFocus::Accept, TradeFocus::Cancel]
    );
    f.view.nav(TradeNav::Bar);
    assert_eq!(f.view.focus(), TradeFocus::Line(1));
    assert_eq!(f.view.nav(TradeNav::Confirm), Some(ServiceRequest::Unstage { session: S, line: 1, count: 4 }));
    // X on a line asks how many to take back.
    assert_eq!(f.view.nav(TradeNav::Quantity), None);
    assert_eq!(f.view.count_dialog().map(|d| d.target), Some(CountTarget::Unstage { line: 1 }));
    f.view.nav(TradeNav::Step(-3));
    assert_eq!(f.view.nav(TradeNav::Confirm), Some(ServiceRequest::Unstage { session: S, line: 1, count: 1 }));
    // The host drops line 1: focus moves to the bar's first item, a dialog on it closes.
    f.view.nav(TradeNav::Quantity);
    quote(&mut f.view, vec![line(Side::Sell, player(), f.sword, "sword", 1, 30)], 30, 30);
    assert_eq!(f.view.focus(), TradeFocus::Line(0));
    assert!(f.view.count_dialog().is_none());
    f.view.nav(TradeNav::Move(10));
    assert_eq!(f.view.focus(), TradeFocus::Cancel);
}

#[test]
fn pane_focus_skips_currency() {
    let mut f = fx();
    f.view.nav(TradeNav::Pane(TradePane::Player));
    f.view.nav(TradeNav::Move(1));
    assert_eq!(f.view.focused_stack(), Some((TradePane::Player, f.sword)));
    f.view.nav(TradeNav::Move(5));
    assert_eq!(f.view.focused_stack(), Some((TradePane::Player, f.arrow)));
    f.view.nav(TradeNav::Move(-5));
    assert_eq!(f.view.focused_stack(), Some((TradePane::Player, f.sword)));
    f.view.nav(TradeNav::SwitchPane);
    assert_eq!(f.view.focus(), TradeFocus::Pane(TradePane::Provider));
    // Hovering the hidden currency stack focuses nothing new.
    let gold = f.pack.inventory().stacks().iter().find(|s| s.item == id("gold")).expect("gold").id;
    f.view.focus_row(TradePane::Player, gold);
    assert_eq!(f.view.focus(), TradeFocus::Pane(TradePane::Provider));
}
