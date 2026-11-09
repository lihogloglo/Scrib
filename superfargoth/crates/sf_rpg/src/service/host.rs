//! Requests, events and the host steps around a session (services.md "Dialogue integration",
//! "Commands and events"). The host applies requests on its tick; screens only send them.

use serde::{Deserialize, Serialize};
use sf_records::RecordId;
use thiserror::Error;

use super::purse::WorldServices;
use super::trade::{ServiceSessions, SessionId, Side, TradeLine, TradeRefusal};
use super::{
    Currency, EntryId, EntryRefusal, ServiceCtx, ServiceEffect, ServiceEntry, ServiceKind, ServiceRules, ServiceShape,
};
use crate::dialogue::Conversations;
use crate::ids::{ActorKey, Holder, PlayerId};
use crate::item::StackId;
use crate::state::RpgCommand;

/// Client to host, per player (carried in that player's command stream).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServiceRequest {
    Open { provider: ActorKey, kind: ServiceKind },
    Stage { session: SessionId, side: Side, source: Holder, stack: StackId, count: u32 },
    Unstage { session: SessionId, line: u32, count: u32 },
    SetBalance { session: SessionId, balance: i64 },
    Accept { session: SessionId },
    Cancel { session: SessionId },
    BuyEntry { provider: ActorKey, kind: ServiceKind, entry: EntryId },
}

/// Host to clients: one service event, stamped with tick, player and provider.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceEvent {
    pub tick: u64,
    pub player: PlayerId,
    pub provider: ActorKey,
    pub kind: ServiceEventKind,
}

/// What a [`ServiceEvent`] reports.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ServiceEventKind {
    ServiceOpened { kind: ServiceKind, shape: ServiceShape, session: Option<SessionId> },
    ServiceRefusedLine { topic: RecordId, response: RecordId },
    /// The whole priced deal, to its player only.
    TradeQuoted { session: SessionId, lines: Vec<TradeLine>, merchant_offer: i64, balance: i64 },
    TradeDone { session: SessionId, balance: i64 },
    TradeRefused { session: SessionId, reason: TradeRefusal },
    EntriesListed { kind: ServiceKind, entries: Vec<ServiceEntry> },
    EntryBought { kind: ServiceKind, entry: EntryId, price: i64 },
    EntryRefused { kind: ServiceKind, entry: EntryId, reason: EntryRefusal },
    PurseRestocked { gold: i64 },
}

/// Why a service did not open.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Error)]
pub enum OpenError {
    #[error("{provider:?} is busy with {holder:?}")]
    Busy { provider: ActorKey, holder: PlayerId },
    #[error("the provider does not offer this service")]
    NotOffered,
    /// A normal dialogue line instead of the service.
    #[error("refused with {topic}/{response}")]
    Refused { topic: RecordId, response: RecordId },
}

/// A service that opened.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Opened {
    pub shape: ServiceShape,
    /// The trade session, for [`ServiceShape::Trade`].
    pub session: Option<SessionId>,
    /// The purse's gold if it was created or restocked (`PurseRestocked`).
    pub restocked: Option<i64>,
}

/// `ServiceRequest::Open`: refuses a provider claimed by another player, asks the adapter for
/// a refusal line, restocks the purse lazily and opens a trade session for trade offers.
pub fn open(
    rules: &dyn ServiceRules,
    ctx: &ServiceCtx,
    kind: &ServiceKind,
    claims: &Conversations,
    services: &mut WorldServices,
    sessions: &mut ServiceSessions,
) -> Result<Opened, OpenError> {
    let other = claims.holder(ctx.provider).filter(|p| *p != ctx.player).or_else(|| sessions.busy(ctx.provider, ctx.player));
    if let Some(holder) = other {
        return Err(OpenError::Busy { provider: ctx.provider, holder });
    }
    let offer = rules.offers(ctx).into_iter().find(|o| o.kind == *kind).ok_or(OpenError::NotOffered)?;
    if let Some((topic, response)) = rules.refusal(ctx, kind) {
        return Err(OpenError::Refused { topic, response });
    }
    let restocked = services.open(ctx.provider, rules.purse_base(ctx), ctx.now);
    let session = (offer.shape == ServiceShape::Trade).then(|| sessions.open(ctx.player, ctx.provider, kind.clone()));
    Ok(Opened { shape: offer.shape, session, restocked })
}

/// A list purchase that passed its checks: the host applies `effects` in order (`Pay` first).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Purchase {
    pub price: i64,
    pub effects: Vec<ServiceEffect>,
}

/// `ServiceRequest::BuyEntry`: the adapter re-prices and re-checks the row now (the list may be
/// stale by a tick); the purchase needs a leading `Pay` the player can afford.
pub fn purchase(
    rules: &dyn ServiceRules,
    ctx: &ServiceCtx,
    kind: &ServiceKind,
    entry: &EntryId,
    player_gold: u64,
) -> Result<Purchase, EntryRefusal> {
    let effects = rules.buy_entry(ctx, kind, entry)?;
    let Some(&ServiceEffect::Pay { amount }) = effects.first() else {
        return Err(EntryRefusal::NoPayment);
    };
    if amount < 0 {
        return Err(EntryRefusal::NoPayment);
    }
    if u64::try_from(amount).unwrap_or(u64::MAX) > player_gold {
        return Err(EntryRefusal::TooExpensive);
    }
    Ok(Purchase { price: amount, effects })
}

/// The player's side of a `Pay`: the currency leaves their inventory (the host adds `amount`
/// to the provider's pool with [`WorldServices::apply_delta`]). `None` for a free row.
pub fn payment_command(player: PlayerId, currency: &Currency, amount: i64) -> Option<RpgCommand> {
    let count = u32::try_from(amount).ok().filter(|c| *c > 0)?;
    Some(RpgCommand::RemoveItem { holder: Holder::Player(player), item: currency.item.clone(), count })
}
