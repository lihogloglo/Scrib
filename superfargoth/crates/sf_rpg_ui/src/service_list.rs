//! The service list screen's view-model (services.md "List services", "Screens"): a title,
//! the player's gold and a virtualized list of priced rows (training, travel, spells,
//! repair). Disabled rows stay in the list with their tooltip and reason (as `OpenMW` keeps
//! the tooltip on greyed buttons; behaviour reference only, no code ported).
//!
//! A on an enabled row yields [`ServiceRequest::BuyEntry`]; rows with lasting or costly
//! effects ask for confirmation first (a modern flow; the adapter's [`ConfirmRule`] decides
//! which). The host re-prices at purchase time, answers `EntryBought` / `EntryRefused` and
//! re-lists the rows (`EntriesListed`). While a purchase waits for that answer, further
//! purchases are ignored (no double buy over the network).

use sf_rpg::ActorKey;
use sf_rpg::service::{
    EntryDetail, EntryId, EntryRefusal, EntryState, LabelRef, ServiceEntry, ServiceEventKind,
    ServiceKind, ServiceRequest,
};
use sf_ui::{ListFrame, ListModel, UiValue, ViewModel, VirtualList};

/// Whether buying a row asks "are you sure?" first.
pub type ConfirmRule = fn(&ServiceEntry) -> bool;

/// The stock rule: travel (a teleport that passes hours) confirms, nothing else does. An
/// adapter whose training passes time adds its own rule.
pub fn confirm_lasting(entry: &ServiceEntry) -> bool {
    matches!(entry.detail, EntryDetail::Destination { .. })
}

/// Why a row is disabled.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RowReason {
    /// The price is more than the player's gold.
    TooExpensive,
    /// The adapter's refusal (skill already at the trainer's level...).
    Refused(LabelRef),
}

/// One row (view-model kind `service.entry`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ServiceRow {
    pub entry: ServiceEntry,
    /// `None` = the row can be bought now.
    pub reason: Option<RowReason>,
}

impl ServiceRow {
    fn new(entry: ServiceEntry, gold: u64) -> Self {
        let reason = match &entry.state {
            EntryState::Refused(why) => Some(RowReason::Refused(why.clone())),
            EntryState::TooExpensive => Some(RowReason::TooExpensive),
            EntryState::Available if u64::try_from(entry.price).unwrap_or(u64::MAX) > gold => {
                Some(RowReason::TooExpensive)
            }
            EntryState::Available => None,
        };
        Self { entry, reason }
    }

    pub fn enabled(&self) -> bool {
        self.reason.is_none()
    }
}

/// Field names a row binding can use.
pub const SERVICE_ROW_FIELDS: &[&str] = &[
    "id", "label", "price", "enabled", "reason", "detail", "stat", "current", "next", "place", "hours", "grant",
    "condition", "max_condition", "text",
];

impl ViewModel for ServiceRow {
    const KIND: &'static str = "service.entry";

    fn fields() -> &'static [&'static str] {
        SERVICE_ROW_FIELDS
    }

    fn field(&self, name: &str) -> Option<UiValue> {
        let e = &self.entry;
        let detail = &e.detail;
        Some(match name {
            "id" => e.id.0.as_str().into(),
            // Label keys; the screen localises them.
            "label" => e.label.0.as_str().into(),
            "price" => e.price.into(),
            "enabled" => self.enabled().into(),
            "reason" => match &self.reason {
                None => UiValue::None,
                Some(RowReason::TooExpensive) => "too_expensive".into(),
                Some(RowReason::Refused(why)) => why.0.as_str().into(),
            },
            "detail" => match detail {
                EntryDetail::Stat { .. } => "stat",
                EntryDetail::Destination { .. } => "destination",
                EntryDetail::Grant { .. } => "grant",
                EntryDetail::Item { .. } => "item",
                EntryDetail::Text(_) => "text",
            }
            .into(),
            "stat" => match detail {
                EntryDetail::Stat { stat, .. } => stat.as_str().into(),
                _ => UiValue::None,
            },
            "current" => match detail {
                EntryDetail::Stat { current, .. } => (*current).into(),
                _ => UiValue::None,
            },
            "next" => match detail {
                EntryDetail::Stat { next, .. } => (*next).into(),
                _ => UiValue::None,
            },
            "place" => match detail {
                EntryDetail::Destination { place, .. } => place.0.as_str().into(),
                _ => UiValue::None,
            },
            "hours" => match detail {
                EntryDetail::Destination { hours, .. } => u32::from(*hours).into(),
                _ => UiValue::None,
            },
            "grant" => match detail {
                EntryDetail::Grant { what } => what.as_str().into(),
                _ => UiValue::None,
            },
            "condition" => match detail {
                EntryDetail::Item { condition, .. } => (*condition).into(),
                _ => UiValue::None,
            },
            "max_condition" => match detail {
                EntryDetail::Item { max, .. } => (*max).into(),
                _ => UiValue::None,
            },
            "text" => match detail {
                EntryDetail::Text(key) => key.0.as_str().into(),
                _ => UiValue::None,
            },
            _ => return None,
        })
    }
}

/// A notice on the list screen.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ListToast {
    Bought { entry: EntryId, price: i64 },
    Refused { entry: EntryId, reason: EntryRefusal },
}

/// Gamepad and keyboard input.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ListNav {
    /// Moves focus by rows (clamped).
    Move(isize),
    /// Page up/down.
    Page(isize),
    /// A: buy the focused row (or answer yes to the confirmation).
    Confirm,
    /// B: dismiss the confirmation, else close.
    Back,
}

/// What an input asks for.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ListOutcome {
    /// Send this to the host.
    Request(ServiceRequest),
    /// Close the screen, back to dialogue (list services hold no session).
    Close,
}

/// The service list screen's state.
pub struct ServiceListView {
    provider: ActorKey,
    kind: ServiceKind,
    title: LabelRef,
    rows: ListModel<EntryId, ServiceRow>,
    nav: VirtualList<EntryId>,
    /// The entries as last listed, in host order (rows re-derive from them when gold changes).
    entries: Vec<ServiceEntry>,
    gold: u64,
    confirm: ConfirmRule,
    asking: Option<EntryId>,
    in_flight: Option<EntryId>,
    toast: Option<ListToast>,
    closed: bool,
}

impl ServiceListView {
    /// An empty list for `kind` at `provider`, with list geometry (row extent and viewport in
    /// layout units). Rows arrive with `EntriesListed`.
    pub fn new(provider: ActorKey, kind: ServiceKind, title: LabelRef, row_extent: f32, viewport: f32) -> Self {
        Self {
            provider,
            kind,
            title,
            rows: ListModel::new(),
            nav: VirtualList::new(row_extent, viewport),
            entries: Vec::new(),
            gold: 0,
            confirm: confirm_lasting,
            asking: None,
            in_flight: None,
            toast: None,
            closed: false,
        }
    }

    /// Replaces the confirmation rule.
    #[must_use]
    pub fn with_confirm(mut self, rule: ConfirmRule) -> Self {
        self.confirm = rule;
        self
    }

    pub fn provider(&self) -> ActorKey {
        self.provider
    }

    pub fn kind(&self) -> &ServiceKind {
        &self.kind
    }

    pub fn title(&self) -> &LabelRef {
        &self.title
    }

    pub fn player_gold(&self) -> u64 {
        self.gold
    }

    pub fn is_closed(&self) -> bool {
        self.closed
    }

    /// The row model (bindings read it).
    pub fn rows(&self) -> &ListModel<EntryId, ServiceRow> {
        &self.rows
    }

    pub fn row(&self, entry: &EntryId) -> Option<&ServiceRow> {
        self.rows.get(entry)
    }

    /// Row ids in host order.
    pub fn order(&mut self) -> &[EntryId] {
        self.rows.view()
    }

    pub fn focus(&self) -> Option<&EntryId> {
        self.nav.focus()
    }

    /// The row awaiting "are you sure?".
    pub fn asking(&self) -> Option<&EntryId> {
        self.asking.as_ref()
    }

    /// Whether a purchase waits for the host.
    pub fn is_buying(&self) -> bool {
        self.in_flight.is_some()
    }

    pub fn toast(&self) -> Option<&ListToast> {
        self.toast.as_ref()
    }

    pub fn take_toast(&mut self) -> Option<ListToast> {
        self.toast.take()
    }

    /// The virtualized list's scroll and pool state.
    pub fn nav_list(&mut self) -> &mut VirtualList<EntryId> {
        &mut self.nav
    }

    /// The rows to show this frame.
    pub fn frame(&mut self) -> ListFrame<EntryId> {
        self.nav.update(&mut self.rows)
    }

    // ------------------------------------------------------------------ data

    /// The player's currency count (the host's loadout count); rows priced above it disable.
    pub fn set_player_gold(&mut self, gold: u64) {
        if gold != self.gold {
            self.gold = gold;
            self.rebuild();
        }
    }

    /// Replaces the rows (host order kept; focus stays on its row, or the row now at its index).
    pub fn set_entries(&mut self, entries: Vec<ServiceEntry>) {
        self.entries = entries;
        self.rebuild();
    }

    fn rebuild(&mut self) {
        let keys: Vec<EntryId> = self.entries.iter().map(|e| e.id.clone()).collect();
        for e in &self.entries {
            self.rows.upsert(e.id.clone(), ServiceRow::new(e.clone(), self.gold));
        }
        let gone: Vec<EntryId> = self.rows.iter().map(|(k, _)| k.clone()).filter(|k| !keys.contains(k)).collect();
        for k in &gone {
            self.rows.remove(k);
        }
        self.rows.reorder(&keys);
        if self.asking.as_ref().is_some_and(|a| self.rows.get(a).is_none_or(|r| !r.enabled())) {
            self.asking = None;
        }
    }

    /// Applies one host event for this player (the caller routes them). Events for another
    /// kind are ignored. Returns whether anything shown changed.
    pub fn apply_event(&mut self, event: &ServiceEventKind) -> bool {
        match event {
            ServiceEventKind::EntriesListed { kind, entries } if *kind == self.kind => {
                self.in_flight = None;
                self.set_entries(entries.clone());
                true
            }
            ServiceEventKind::EntryBought { kind, entry, price } if *kind == self.kind => {
                self.in_flight = None;
                self.set_player_gold(self.gold.saturating_sub(u64::try_from(*price).unwrap_or(0)));
                self.toast = Some(ListToast::Bought { entry: entry.clone(), price: *price });
                true
            }
            ServiceEventKind::EntryRefused { kind, entry, reason } if *kind == self.kind => {
                self.in_flight = None;
                self.toast = Some(ListToast::Refused { entry: entry.clone(), reason: reason.clone() });
                true
            }
            _ => false,
        }
    }

    // ------------------------------------------------------------------ input

    /// Hover or click: focuses a row.
    pub fn focus_row(&mut self, entry: &EntryId) {
        if self.rows.get(entry).is_some() {
            self.nav.set_focus(Some(entry.clone()));
        }
    }

    fn buy_request(&mut self, entry: EntryId) -> ListOutcome {
        self.in_flight = Some(entry.clone());
        ListOutcome::Request(ServiceRequest::BuyEntry { provider: self.provider, kind: self.kind.clone(), entry })
    }

    /// A or a click on a row. Disabled rows and a purchase in flight do nothing; a row the
    /// confirmation rule picks opens the question instead.
    pub fn activate(&mut self, entry: &EntryId) -> Option<ListOutcome> {
        self.focus_row(entry);
        if self.closed || self.in_flight.is_some() {
            return None;
        }
        let row = self.rows.get(entry).filter(|r| r.enabled())?;
        if (self.confirm)(&row.entry) {
            self.asking = Some(entry.clone());
            return None;
        }
        Some(self.buy_request(entry.clone()))
    }

    /// Yes to "are you sure?".
    pub fn confirm(&mut self) -> Option<ListOutcome> {
        let entry = self.asking.take()?;
        let enabled = self.rows.get(&entry).is_some_and(ServiceRow::enabled);
        (enabled && !self.closed && self.in_flight.is_none()).then(|| self.buy_request(entry))
    }

    /// No to "are you sure?".
    pub fn dismiss(&mut self) {
        self.asking = None;
    }

    /// Closes the screen.
    pub fn close(&mut self) -> Option<ListOutcome> {
        if self.closed {
            return None;
        }
        self.closed = true;
        self.asking = None;
        Some(ListOutcome::Close)
    }

    /// Gamepad and keyboard input.
    pub fn nav(&mut self, nav: ListNav) -> Option<ListOutcome> {
        if self.asking.is_some() {
            return match nav {
                ListNav::Confirm => self.confirm(),
                ListNav::Back => {
                    self.dismiss();
                    None
                }
                ListNav::Move(_) | ListNav::Page(_) => None,
            };
        }
        match nav {
            ListNav::Move(d) => self.nav.move_focus(&mut self.rows, d),
            ListNav::Page(p) => self.nav.page(&mut self.rows, p),
            ListNav::Confirm => {
                let entry = self.nav.focus().cloned()?;
                return self.activate(&entry);
            }
            ListNav::Back => return self.close(),
        }
        None
    }
}
