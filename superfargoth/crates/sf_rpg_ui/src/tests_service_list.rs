//! Service list view-model: synthetic training and travel rows, host answers by hand.

use sf_rpg::service::{
    EntryDetail, EntryId, EntryRefusal, EntryState, LabelRef, ServiceEntry, ServiceEventKind, ServiceKind,
    ServiceRequest,
};
use sf_rpg::{ActorKey, RecordId};
use sf_ui::{UiValue, ViewModel};

use super::*;

fn id(s: &str) -> RecordId {
    RecordId::new(s)
}

const TRAINER: ActorKey = ActorKey(4);

fn kind() -> ServiceKind {
    ServiceKind(id("train"))
}

fn eid(s: &str) -> EntryId {
    EntryId(id(s))
}

fn stat(name: &str, price: i64, state: EntryState) -> ServiceEntry {
    ServiceEntry {
        id: eid(name),
        label: LabelRef(id(name)),
        detail: EntryDetail::Stat { stat: id(name), current: 10, next: 11 },
        price,
        state,
    }
}

fn travel(name: &str, price: i64) -> ServiceEntry {
    ServiceEntry {
        id: eid(name),
        label: LabelRef(id(name)),
        detail: EntryDetail::Destination { place: LabelRef(id(name)), hours: 6 },
        price,
        state: EntryState::Available,
    }
}

/// Blade 20, block 80 (more than the 50 gold), athletics refused, travel to the port 30.
fn view() -> ServiceListView {
    let mut v = ServiceListView::new(TRAINER, kind(), LabelRef(id("train-title")), 10.0, 100.0);
    v.set_player_gold(50);
    assert!(v.apply_event(&ServiceEventKind::EntriesListed {
        kind: kind(),
        entries: vec![
            stat("blade", 20, EntryState::Available),
            stat("block", 80, EntryState::Available),
            stat("athletics", 10, EntryState::Refused(LabelRef(id("too-skilled")))),
            travel("port", 30),
        ],
    }));
    v
}

fn buy(entry: &str) -> Option<ListOutcome> {
    Some(ListOutcome::Request(ServiceRequest::BuyEntry { provider: TRAINER, kind: kind(), entry: eid(entry) }))
}

#[test]
fn rows_keep_host_order_and_disabled_rows_keep_their_reason() {
    let mut v = view();
    assert_eq!(v.order().to_vec(), vec![eid("blade"), eid("block"), eid("athletics"), eid("port")]);
    let reason = |v: &ServiceListView, e: &str| v.row(&eid(e)).expect("row").reason.clone();
    assert_eq!(reason(&v, "blade"), None);
    assert_eq!(reason(&v, "block"), Some(RowReason::TooExpensive));
    assert_eq!(reason(&v, "athletics"), Some(RowReason::Refused(LabelRef(id("too-skilled")))));
    // The tooltip fields stay on a disabled row.
    let row = v.row(&eid("athletics")).expect("row");
    assert_eq!(row.field("enabled"), Some(UiValue::Bool(false)));
    assert_eq!(row.field("reason"), Some(UiValue::Text("too-skilled".into())));
    assert_eq!(row.field("detail"), Some(UiValue::Text("stat".into())));
    assert_eq!(row.field("current"), Some(UiValue::Int(10)));
    assert_eq!(row.field("next"), Some(UiValue::Int(11)));
    assert_eq!(row.field("price"), Some(UiValue::Int(10)));
    assert_eq!(row.field("hours"), Some(UiValue::None));
    assert_eq!(row.field("nope"), None);
    let port = v.row(&eid("port")).expect("row");
    assert_eq!(port.field("hours"), Some(UiValue::Int(6)));
    assert_eq!(port.field("place"), Some(UiValue::Text("port".into())));
    // More gold enables the expensive row.
    v.set_player_gold(80);
    assert_eq!(reason(&v, "block"), None);
}

#[test]
fn a_buys_an_enabled_row_once_until_the_host_answers() {
    let mut v = view();
    assert_eq!(v.nav(ListNav::Move(1)), None);
    assert_eq!(v.focus(), Some(&eid("blade")));
    assert_eq!(v.nav(ListNav::Confirm), buy("blade"));
    assert!(v.is_buying());
    assert_eq!(v.nav(ListNav::Confirm), None);
    assert!(v.apply_event(&ServiceEventKind::EntryBought { kind: kind(), entry: eid("blade"), price: 20 }));
    assert!(!v.is_buying());
    assert_eq!(v.player_gold(), 30);
    assert_eq!(v.take_toast(), Some(ListToast::Bought { entry: eid("blade"), price: 20 }));
    assert_eq!(v.activate(&eid("blade")), buy("blade"));
    assert!(v.apply_event(&ServiceEventKind::EntryRefused {
        kind: kind(),
        entry: eid("blade"),
        reason: EntryRefusal::TooExpensive,
    }));
    assert!(!v.is_buying());
    assert_eq!(v.take_toast(), Some(ListToast::Refused { entry: eid("blade"), reason: EntryRefusal::TooExpensive }));
}

#[test]
fn disabled_rows_do_nothing() {
    let mut v = view();
    assert_eq!(v.activate(&eid("block")), None);
    assert_eq!(v.activate(&eid("athletics")), None);
    assert_eq!(v.activate(&eid("missing")), None);
    assert!(!v.is_buying());
    // Clicking still focuses it (the tooltip shows).
    assert_eq!(v.focus(), Some(&eid("athletics")));
}

#[test]
fn travel_asks_first() {
    let mut v = view();
    assert_eq!(v.activate(&eid("port")), None);
    assert_eq!(v.asking(), Some(&eid("port")));
    // Moving does nothing while asking; B dismisses.
    assert_eq!(v.nav(ListNav::Move(1)), None);
    assert_eq!(v.nav(ListNav::Back), None);
    assert_eq!(v.asking(), None);
    assert!(!v.is_closed());
    v.activate(&eid("port"));
    assert_eq!(v.nav(ListNav::Confirm), buy("port"));
    assert_eq!(v.asking(), None);
}

#[test]
fn a_custom_rule_confirms_training_too() {
    let mut v = view().with_confirm(|_| true);
    assert_eq!(v.activate(&eid("blade")), None);
    assert_eq!(v.confirm(), buy("blade"));
    let mut v = view().with_confirm(|_| false);
    assert_eq!(v.activate(&eid("port")), buy("port"));
}

#[test]
fn relisting_drops_gone_rows_and_a_stale_question() {
    let mut v = view();
    v.activate(&eid("port"));
    assert!(v.apply_event(&ServiceEventKind::EntriesListed {
        kind: kind(),
        entries: vec![stat("blade", 25, EntryState::Available)],
    }));
    assert_eq!(v.order().to_vec(), vec![eid("blade")]);
    assert_eq!(v.asking(), None);
    assert_eq!(v.row(&eid("blade")).map(|r| r.entry.price), Some(25));
    // Another service's rows are not ours.
    assert!(!v.apply_event(&ServiceEventKind::EntriesListed { kind: ServiceKind(id("travel")), entries: Vec::new() }));
    assert_eq!(v.rows().len(), 1);
}

#[test]
fn back_closes_once() {
    let mut v = view();
    assert_eq!(v.nav(ListNav::Back), Some(ListOutcome::Close));
    assert!(v.is_closed());
    assert_eq!(v.nav(ListNav::Back), None);
    assert_eq!(v.activate(&eid("blade")), None);
}
