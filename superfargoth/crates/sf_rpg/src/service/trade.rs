//! The host-owned trade session (services.md "Trade session"): two staged item lists, priced
//! per line through the adapter's rules, settled on accept as plain [`RpgCommand`]s. Nothing
//! in the world changes until the host applies the settlement, so cancel is free.
//!
//! The accept order follows `OpenMW`'s `TradeWindow::onOfferButtonClicked`
//! (`apps/openmw/mwgui/tradewindow.cpp`, commit ca5e2576): nothing staged, counts, money,
//! deal check, haggle, then the transfer. Behaviour reference only; no code is ported.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use sf_records::RecordId;
use thiserror::Error;

use super::purse::{PurseRule, WorldServices};
use super::{DealTotals, DealVeto, Holdings, QuoteLine, ServiceCtx, ServiceEffect, ServiceKind, ServiceRules, TradeConfig};
use crate::ids::{ActorKey, Holder, PlayerId};
use crate::item::{Attrs, StackId};
use crate::state::RpgCommand;

/// A trade session id: monotonic per host run, not saved.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SessionId(pub u32);

/// Which way a line goes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Side {
    /// Provider to player.
    Buy,
    /// Player to provider.
    Sell,
}

/// One staged line: `count` pieces of one stack, priced as a whole.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TradeLine {
    pub side: Side,
    pub source: Holder,
    pub stack: StackId,
    pub item: RecordId,
    pub attrs: Attrs,
    pub count: u32,
    /// Price of the whole line, `>= 0`.
    pub price: i64,
}

/// What a session reads while staging and accepting.
#[derive(Clone, Copy)]
pub struct TradeEnv<'a> {
    pub ctx: &'a ServiceCtx,
    pub rules: &'a dyn ServiceRules,
    pub holdings: &'a dyn Holdings,
    pub config: &'a TradeConfig,
}

/// The provider's money at accept time, resolved by the host from its [`PurseRule`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MerchantMoney {
    /// A pool with this much gold.
    Pool(i64),
    Unlimited,
    /// The provider's own currency stacks.
    Inventory,
}

impl MerchantMoney {
    /// The money a provider has under `rule` (a pool not opened yet has none).
    pub fn of(rule: PurseRule, services: &WorldServices, provider: ActorKey) -> Self {
        match rule {
            PurseRule::Pool { .. } => Self::Pool(services.gold(provider).unwrap_or(0)),
            PurseRule::Unlimited => Self::Unlimited,
            PurseRule::Inventory => Self::Inventory,
        }
    }
}

/// An accepted deal, for the host to apply in order: `commands` (items and currency), then
/// `purse_delta` on the provider's pool, then `effects`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Settlement {
    pub commands: Vec<RpgCommand>,
    /// `-balance` for a pool, `0` otherwise.
    pub purse_delta: i64,
    pub effects: Vec<ServiceEffect>,
}

/// Why a staging change or an accept was refused. Nothing changed (an accept that rolled a
/// haggle counts the attempt).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Error)]
pub enum TradeRefusal {
    #[error("nothing is staged")]
    NothingStaged,
    #[error("line {line}'s stack changed since it was staged")]
    StockChanged { line: usize },
    #[error("the player has {have} and needs {want}")]
    PlayerCantAfford { have: u64, want: u64 },
    #[error("the provider has {have} and needs {want}")]
    MerchantCantAfford { have: i64, want: i64 },
    #[error("the provider refuses the deal: {0:?}")]
    Vetoed(DealVeto),
    #[error("the provider refuses the offer")]
    HaggleFailed { effects: Vec<ServiceEffect> },
    #[error("no stack {stack:?} in {holder:?}")]
    NoStack { holder: Holder, stack: StackId },
    #[error("{0:?} is not a source for this side")]
    WrongSource(Holder),
    #[error("the provider does not trade this stack")]
    NotTraded,
    #[error("currency is never staged")]
    Currency,
    #[error("the whole stack is already staged")]
    NothingLeft,
    #[error("count must be at least 1")]
    ZeroCount,
    #[error("no line {0}")]
    NoLine(usize),
    #[error("haggling is off")]
    HaggleDisabled,
    #[error("the balance does not fit a stack count")]
    AmountTooLarge,
}

impl TradeRefusal {
    /// A veto closes the session; every other refusal keeps it open.
    pub fn closes(&self) -> bool {
        matches!(self, Self::Vetoed(_))
    }
}

/// A staged deal between one player and one provider.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TradeSession {
    pub id: SessionId,
    pub player: PlayerId,
    pub provider: ActorKey,
    pub kind: ServiceKind,
    /// Staged lines; a line index is its position here (unstaging a whole line shifts later ones).
    pub lines: Vec<TradeLine>,
    /// `+Sell - Buy` (positive = provider pays the player).
    pub merchant_offer: i64,
    /// The player's proposal; equals `merchant_offer` unless haggled.
    pub balance: i64,
    pub haggled: bool,
    /// Haggle rolls taken (feeds the roll seed).
    pub attempts: u16,
}

/// `v` with the sign of the deal direction: the player can't turn "I pay" into "you pay".
fn clamp_sign(v: i64, offer: i64) -> i64 {
    match offer.signum() {
        1 => v.max(0),
        -1 => v.min(0),
        _ => 0,
    }
}

impl TradeSession {
    pub fn new(id: SessionId, player: PlayerId, provider: ActorKey, kind: ServiceKind) -> Self {
        Self {
            id,
            player,
            provider,
            kind,
            lines: Vec::new(),
            merchant_offer: 0,
            balance: 0,
            haggled: false,
            attempts: 0,
        }
    }

    /// Pieces of `(source, stack)` staged on any line.
    pub fn staged(&self, source: Holder, stack: StackId) -> u64 {
        self.lines.iter().filter(|l| l.source == source && l.stack == stack).map(|l| u64::from(l.count)).sum()
    }

    /// The deal's totals, for the haggle rule and the deal bar.
    pub fn totals(&self) -> DealTotals {
        let sum = |side: Side| self.lines.iter().filter(|l| l.side == side).map(|l| l.price).fold(0, i64::saturating_add);
        DealTotals {
            merchant_offer: self.merchant_offer,
            balance: self.balance,
            buy_total: sum(Side::Buy),
            sell_total: sum(Side::Sell),
        }
    }

    /// Stages up to `count` pieces of a stack (capped by what the holder has minus what other
    /// lines stage from it); the same `(side, source, stack)` merges into one line. Returns the
    /// pieces actually staged.
    pub fn stage(
        &mut self,
        env: &TradeEnv<'_>,
        side: Side,
        source: Holder,
        stack: StackId,
        count: u32,
    ) -> Result<u32, TradeRefusal> {
        if count == 0 {
            return Err(TradeRefusal::ZeroCount);
        }
        let source_ok = match side {
            Side::Sell => source == Holder::Player(self.player),
            Side::Buy => env.rules.stock_sources(env.ctx).contains(&source),
        };
        if !source_ok {
            return Err(TradeRefusal::WrongSource(source));
        }
        let found = env
            .holdings
            .loadout(source)
            .and_then(|l| l.inventory().get(stack))
            .ok_or(TradeRefusal::NoStack { holder: source, stack })?;
        if found.item == env.config.currency.item {
            return Err(TradeRefusal::Currency);
        }
        let traded = match side {
            Side::Buy => env.rules.sells(env.ctx, source, found),
            Side::Sell => env.rules.buys(env.ctx, found),
        };
        if !traded {
            return Err(TradeRefusal::NotTraded);
        }
        let left = u64::from(found.count).saturating_sub(self.staged(source, stack));
        let n = u32::try_from(left).unwrap_or(u32::MAX).min(count);
        if n == 0 {
            return Err(TradeRefusal::NothingLeft);
        }
        let index = match self.lines.iter().position(|l| l.side == side && l.source == source && l.stack == stack) {
            Some(i) => {
                if !found.is_same_kind(&self.lines[i].item, &self.lines[i].attrs) {
                    return Err(TradeRefusal::StockChanged { line: i });
                }
                // `n <= found.count - staged`, so the sum fits the stack's own count.
                self.lines[i].count += n;
                i
            }
            None => {
                self.lines.push(TradeLine {
                    side,
                    source,
                    stack,
                    item: found.item.clone(),
                    attrs: found.attrs.clone(),
                    count: n,
                    price: 0,
                });
                self.lines.len() - 1
            }
        };
        self.requote(env, index);
        self.update_offer();
        Ok(n)
    }

    /// Takes up to `count` pieces off a line (the whole line goes at zero). Returns the pieces
    /// removed.
    pub fn unstage(&mut self, env: &TradeEnv<'_>, line: usize, count: u32) -> Result<u32, TradeRefusal> {
        if count == 0 {
            return Err(TradeRefusal::ZeroCount);
        }
        let l = self.lines.get_mut(line).ok_or(TradeRefusal::NoLine(line))?;
        let n = count.min(l.count);
        l.count -= n;
        if l.count == 0 {
            self.lines.remove(line);
        } else {
            self.requote(env, line);
        }
        self.update_offer();
        Ok(n)
    }

    /// Haggling: the player's proposed balance, sign-clamped to the deal direction.
    pub fn set_balance(&mut self, config: &TradeConfig, balance: i64) -> Result<i64, TradeRefusal> {
        if !config.haggle {
            return Err(TradeRefusal::HaggleDisabled);
        }
        if self.lines.is_empty() {
            return Err(TradeRefusal::NothingStaged);
        }
        self.balance = clamp_sign(balance, self.merchant_offer);
        self.haggled = self.balance != self.merchant_offer;
        Ok(self.balance)
    }

    fn requote(&mut self, env: &TradeEnv<'_>, index: usize) {
        let l = &self.lines[index];
        let line = QuoteLine { side: l.side, source: l.source, item: &l.item, attrs: &l.attrs, count: l.count };
        let price = env.rules.quote(env.ctx, &line).max(0);
        self.lines[index].price = price;
    }

    /// Recomputes `merchant_offer`; a haggled balance keeps its margin (`OpenMW` `updateOffer`).
    fn update_offer(&mut self) {
        let old = self.merchant_offer;
        self.merchant_offer = self
            .lines
            .iter()
            .map(|l| match l.side {
                Side::Sell => l.price,
                Side::Buy => -l.price,
            })
            .fold(0, i64::saturating_add);
        if self.lines.is_empty() {
            self.haggled = false;
            self.balance = 0;
        } else if self.haggled {
            let moved = self.balance.saturating_add(self.merchant_offer.saturating_sub(old));
            self.balance = clamp_sign(moved, self.merchant_offer);
            self.haggled = self.balance != self.merchant_offer;
        } else {
            self.balance = self.merchant_offer;
        }
    }

    /// Accepts the deal: pure over its inputs (the only session change is the haggle attempt
    /// count). See the module docs for the order.
    #[allow(clippy::too_many_lines, reason = "one linear pass in the spec's accept order")]
    pub fn accept(&mut self, env: &TradeEnv<'_>, money: MerchantMoney, roll: u8) -> Result<Settlement, TradeRefusal> {
        if self.lines.is_empty() {
            return Err(TradeRefusal::NothingStaged);
        }
        for (i, line) in self.lines.iter().enumerate() {
            let staged = self.staged(line.source, line.stack);
            let ok = env
                .holdings
                .loadout(line.source)
                .and_then(|l| l.inventory().get(line.stack))
                .is_some_and(|s| s.is_same_kind(&line.item, &line.attrs) && u64::from(s.count) >= staged);
            if !ok {
                return Err(TradeRefusal::StockChanged { line: i });
            }
        }
        let player = Holder::Player(self.player);
        let provider = Holder::Actor(self.provider);
        let currency = &env.config.currency.item;
        let amount = u32::try_from(self.balance.unsigned_abs()).map_err(|_| TradeRefusal::AmountTooLarge)?;
        if self.balance < 0 {
            let want = self.balance.unsigned_abs();
            let have = env.holdings.loadout(player).map_or(0, |l| l.count(currency));
            if have < want {
                return Err(TradeRefusal::PlayerCantAfford { have, want });
            }
        } else if self.balance > 0 {
            let have = match money {
                MerchantMoney::Pool(gold) => Some(gold),
                MerchantMoney::Unlimited => None,
                MerchantMoney::Inventory => Some(
                    env.holdings.loadout(provider).map_or(0, |l| i64::try_from(l.count(currency)).unwrap_or(i64::MAX)),
                ),
            };
            if let Some(have) = have.filter(|h| *h < self.balance) {
                return Err(TradeRefusal::MerchantCantAfford { have, want: self.balance });
            }
        }
        env.rules.vet_deal(env.ctx, self).map_err(TradeRefusal::Vetoed)?;
        let mut effects = Vec::new();
        if self.balance != self.merchant_offer {
            let outcome = env.rules.haggle(env.ctx, &self.totals(), roll);
            self.attempts = self.attempts.saturating_add(1);
            if !outcome.accepted {
                return Err(TradeRefusal::HaggleFailed { effects: outcome.effects });
            }
            effects = outcome.effects;
        }
        let mut commands = Vec::with_capacity(self.lines.len() + 2);
        for line in &self.lines {
            let cmd = match line.side {
                Side::Sell => RpgCommand::MoveStack { from: line.source, to: provider, stack: line.stack, count: line.count },
                Side::Buy => {
                    let restocking = env
                        .holdings
                        .loadout(line.source)
                        .and_then(|l| l.inventory().get(line.stack))
                        .is_some_and(|s| env.rules.restocking(env.ctx, line.source, s));
                    if restocking {
                        RpgCommand::AddItem { holder: player, item: line.item.clone(), count: line.count, attrs: line.attrs.clone() }
                    } else {
                        RpgCommand::MoveStack { from: line.source, to: player, stack: line.stack, count: line.count }
                    }
                }
            };
            commands.push(cmd);
        }
        let from_inventory = money == MerchantMoney::Inventory;
        if self.balance < 0 {
            commands.push(RpgCommand::RemoveItem { holder: player, item: currency.clone(), count: amount });
            if from_inventory {
                commands.push(RpgCommand::AddItem { holder: provider, item: currency.clone(), count: amount, attrs: Attrs::new() });
            }
        } else if self.balance > 0 {
            if from_inventory {
                commands.push(RpgCommand::RemoveItem { holder: provider, item: currency.clone(), count: amount });
            }
            commands.push(RpgCommand::AddItem { holder: player, item: currency.clone(), count: amount, attrs: Attrs::new() });
        }
        let purse_delta = if matches!(money, MerchantMoney::Pool(_)) { -self.balance } else { 0 };
        Ok(Settlement { commands, purse_delta, effects })
    }
}

/// Prices of every stack of one pane, sent once per open (the price column): `None` where
/// the stack is currency or not traded on that side. A whole stack is quoted as one line.
pub fn pane_prices(env: &TradeEnv<'_>, side: Side, source: Holder) -> Vec<(StackId, Option<i64>)> {
    let Some(loadout) = env.holdings.loadout(source) else {
        return Vec::new();
    };
    loadout
        .inventory()
        .stacks()
        .iter()
        .map(|s| {
            let traded = s.item != env.config.currency.item
                && match side {
                    Side::Buy => env.rules.sells(env.ctx, source, s),
                    Side::Sell => env.rules.buys(env.ctx, s),
                };
            let price = traded.then(|| {
                let line = QuoteLine { side, source, item: &s.item, attrs: &s.attrs, count: s.count };
                env.rules.quote(env.ctx, &line).max(0)
            });
            (s.id, price)
        })
        .collect()
}

/// The host's open trade sessions (not saved).
#[derive(Clone, Debug, Default)]
pub struct ServiceSessions {
    next: u32,
    sessions: BTreeMap<SessionId, TradeSession>,
}

impl ServiceSessions {
    pub fn new() -> Self {
        Self::default()
    }

    /// Opens a session, replacing the player's earlier one with the same provider.
    pub fn open(&mut self, player: PlayerId, provider: ActorKey, kind: ServiceKind) -> SessionId {
        self.sessions.retain(|_, s| !(s.player == player && s.provider == provider));
        let id = SessionId(self.next);
        self.next = self.next.wrapping_add(1);
        self.sessions.insert(id, TradeSession::new(id, player, provider, kind));
        id
    }

    /// Another player's session with `provider`, if any.
    pub fn busy(&self, provider: ActorKey, player: PlayerId) -> Option<PlayerId> {
        self.sessions.values().find(|s| s.provider == provider && s.player != player).map(|s| s.player)
    }

    pub fn get(&self, id: SessionId) -> Option<&TradeSession> {
        self.sessions.get(&id)
    }

    /// The session if `player` owns it (a request naming another player's session is ignored).
    pub fn get_mut(&mut self, id: SessionId, player: PlayerId) -> Option<&mut TradeSession> {
        self.sessions.get_mut(&id).filter(|s| s.player == player)
    }

    /// Drops a session (cancel, accept, veto); nothing was moved.
    pub fn close(&mut self, id: SessionId) -> Option<TradeSession> {
        self.sessions.remove(&id)
    }

    /// Drops every session of a player (left the game, dialogue closed).
    pub fn close_player(&mut self, player: PlayerId) {
        self.sessions.retain(|_, s| s.player != player);
    }

    /// Drops every session (saving: staged items never moved, so nothing is lost).
    pub fn close_all(&mut self) {
        self.sessions.clear();
    }

    pub fn len(&self) -> usize {
        self.sessions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.sessions.is_empty()
    }
}
