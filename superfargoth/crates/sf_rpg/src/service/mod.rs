//! NPC services and trade (`docs/specs/services.md`): offers, the adapter's [`ServiceRules`]
//! hook, the host-owned [`TradeSession`], list-service rows and effects, merchant purses on
//! the world clock, and the fixed-price rule. Bevy-free and content-free: service ids,
//! formulas and flags come from the adapter. Everything saved or sent is serde data keyed by
//! stable ids; rolls are inputs, never drawn here.

mod host;
mod price;
mod purse;
mod trade;

use serde::{Deserialize, Serialize};
use sf_records::RecordId;
use smallvec::SmallVec;

use crate::equip::Loadout;
use crate::ids::{ActorKey, Holder, PlayerId};
use crate::item::{AttrValue, Attrs, ItemStack, StackId};
use crate::quest::GameTime;
use crate::state::CharacterRpg;

pub use host::{
    OpenError, Opened, Purchase, ServiceEvent, ServiceEventKind, ServiceRequest, open, payment_command, purchase,
};
pub use price::{FixedPrice, FixedPriceIn};
pub use purse::{Purse, PurseRule, SERVICES_SECTION, SERVICES_VERSION, WorldServices};
pub use trade::{
    MerchantMoney, ServiceSessions, SessionId, Settlement, Side, TradeEnv, TradeLine, TradeRefusal, TradeSession,
    pane_prices,
};

/// Opaque service id from content ("trade", "train", "travel"...). The framework never
/// matches on it.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ServiceKind(pub RecordId);

/// A localised text key (button text, row label, refusal reason). The UI resolves it.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct LabelRef(pub RecordId);

/// Which screen a service opens.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServiceShape {
    /// Two-pane trade screen over a [`TradeSession`].
    Trade,
    /// Service-list screen over [`ServiceEntry`] rows.
    List,
    /// An adapter screen id; the framework only opens it and settles payment.
    Custom(RecordId),
}

/// One button in the dialogue window.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceOffer {
    pub provider: ActorKey,
    pub kind: ServiceKind,
    pub shape: ServiceShape,
    /// Localised button text key.
    pub label: LabelRef,
}

/// Who asks whom, and when. Quotes use this player's stats (no singleton player).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceCtx {
    pub player: PlayerId,
    pub provider: ActorKey,
    /// The provider's base record (its flags and stock list live there).
    pub provider_base: RecordId,
    pub now: GameTime,
    pub tick: u64,
}

/// The game's money item (one currency per game; owner 2026-11-08).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Currency {
    pub item: RecordId,
}

/// Adapter trade settings.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TradeConfig {
    pub currency: Currency,
    /// Whether the player may propose a balance (the haggle stepper); off = fixed prices.
    pub haggle: bool,
}

/// One staged line, as the price rule sees it. The adapter derives the per-instance base
/// value (condition-scaled) from `item` and `attrs` when it builds its rule input.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QuoteLine<'a> {
    pub side: Side,
    pub source: Holder,
    pub item: &'a RecordId,
    pub attrs: &'a Attrs,
    pub count: u32,
}

/// A deal's totals, for the haggle rule.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DealTotals {
    /// What the provider offers: `+Sell - Buy` (positive = provider pays).
    pub merchant_offer: i64,
    /// What the player proposes.
    pub balance: i64,
    /// Sum of Buy line prices.
    pub buy_total: i64,
    /// Sum of Sell line prices.
    pub sell_total: i64,
}

/// What the haggle rule decided, with its side effects (skill use, disposition).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HaggleOutcome {
    pub accepted: bool,
    pub effects: Vec<ServiceEffect>,
}

/// The adapter's veto of a deal (stolen goods). Carries its own effects (confiscation); the
/// host closes the session.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DealVeto {
    pub reason: LabelRef,
    pub effects: Vec<ServiceEffect>,
}

/// A list-service row id: skill id, destination id, spell id, or an adapter key for repair.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct EntryId(pub RecordId);

/// One priced row of a list service.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceEntry {
    pub id: EntryId,
    pub label: LabelRef,
    pub detail: EntryDetail,
    pub price: i64,
    pub state: EntryState,
}

/// Whether a row can be bought. Refused rows still show, with why.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntryState {
    Available,
    TooExpensive,
    Refused(LabelRef),
}

/// Display data for a row's second line and tooltip.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntryDetail {
    Stat { stat: RecordId, current: i32, next: i32 },
    Destination { place: LabelRef, hours: u16 },
    /// A spell, a recipe...
    Grant { what: RecordId },
    Item { holder: Holder, stack: StackId, condition: i32, max: i32 },
    Text(LabelRef),
}

/// Why a list purchase failed (re-checked at purchase time).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, thiserror::Error)]
pub enum EntryRefusal {
    #[error("the price is more than the player has")]
    TooExpensive,
    #[error("refused: {0:?}")]
    Refused(LabelRef),
    #[error("no such entry")]
    Unknown,
    #[error("the adapter's effects do not start with a payment")]
    NoPayment,
}

/// A position in a worldspace: `WorldPos`'s canonical form (cell plus offset), kept as plain
/// arrays because this crate does not depend on `sf_coords`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TravelPos {
    pub cell: [i32; 3],
    pub local: [f32; 3],
}

/// Where travel puts the buyer.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TravelTarget {
    /// An interior cell, or `None` for the exterior worldspace.
    pub cell: Option<RecordId>,
    pub pos: TravelPos,
    pub yaw: f32,
}

/// What the host executes after payment, in order.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ServiceEffect {
    /// Player to provider purse (always first in a list purchase).
    Pay { amount: i64 },
    /// The host maps it to the stats crate's training raise.
    RaiseStat { stat: RecordId },
    UseSkill { stat: RecordId, use_kind: u8, scale: f32 },
    Teleport { to: TravelTarget, with_followers: bool },
    /// In co-op a party rest request; skipped if the vote fails (owner 2026-11-08).
    PassTime { hours: u16, sleep: bool },
    /// The adapter applies it (learn a spell).
    Grant { what: RecordId },
    SetAttr { holder: Holder, stack: StackId, key: RecordId, value: Option<AttrValue> },
    /// The adapter's disposition state.
    Disposition { delta: i32 },
    /// Escape hatch, applied by the adapter.
    Adapter { id: RecordId, payload: AttrValue },
}

/// Read access to inventories by holder (the host's character map, a test fixture).
pub trait Holdings {
    fn loadout(&self, holder: Holder) -> Option<&Loadout>;
}

impl Holdings for std::collections::BTreeMap<Holder, CharacterRpg> {
    fn loadout(&self, holder: Holder) -> Option<&Loadout> {
        self.get(&holder).map(|c| &c.loadout)
    }
}

/// Adapter hook: the game's services. Every method reads; only the host applies what it
/// returns. Prices and rolls go through the adapter's pure rules (`fn(&In) -> Out`); these
/// methods only gather the inputs.
pub trait ServiceRules {
    /// Offers for this provider, in button order. Derived from content flags; cheap.
    fn offers(&self, ctx: &ServiceCtx) -> SmallVec<[ServiceOffer; 8]>;
    /// A refusal line `(topic, response)` instead of opening; `None` = open.
    fn refusal(&self, ctx: &ServiceCtx, kind: &ServiceKind) -> Option<(RecordId, RecordId)>;
    /// Trade: where the provider's stock comes from.
    fn stock_sources(&self, ctx: &ServiceCtx) -> SmallVec<[Holder; 4]>;
    /// Trade: whether the provider buys this player stack.
    fn buys(&self, ctx: &ServiceCtx, stack: &ItemStack) -> bool;
    /// Trade: whether the provider sells this stack (hides equipped, bound...).
    fn sells(&self, ctx: &ServiceCtx, source: Holder, stack: &ItemStack) -> bool;
    /// A restocking stack is never used up by a sale (content's infinite stock).
    fn restocking(&self, ctx: &ServiceCtx, source: Holder, stack: &ItemStack) -> bool;
    /// The price of a whole line, `>= 0`.
    fn quote(&self, ctx: &ServiceCtx, line: &QuoteLine<'_>) -> i64;
    /// Whether a haggled balance is accepted; `roll` is the host's seeded draw.
    fn haggle(&self, ctx: &ServiceCtx, deal: &DealTotals, roll: u8) -> HaggleOutcome;
    /// Deal check beyond money and counts (stolen goods).
    fn vet_deal(&self, ctx: &ServiceCtx, session: &TradeSession) -> Result<(), DealVeto>;
    /// List services: rows, priced, enabled or not.
    fn entries(&self, ctx: &ServiceCtx, kind: &ServiceKind) -> Vec<ServiceEntry>;
    /// List services: what buying a row does, re-checked now. Starts with [`ServiceEffect::Pay`].
    fn buy_entry(&self, ctx: &ServiceCtx, kind: &ServiceKind, entry: &EntryId)
    -> Result<Vec<ServiceEffect>, EntryRefusal>;
    /// How this provider's money works.
    fn purse_base(&self, ctx: &ServiceCtx) -> PurseRule;
}

/// The seed for a provider's roll: a pure mix of `(provider, session, attempt, tick)` (net
/// rule 4). Each step is a bijection, so changing any one input changes the seed.
pub fn service_seed(provider: ActorKey, session: SessionId, attempt: u16, tick: u64) -> u64 {
    #[allow(clippy::cast_possible_truncation, reason = "folding the 128-bit key into two 64-bit halves")]
    let (lo, hi) = (provider.0 as u64, (provider.0 >> 64) as u64);
    let mut s = mix(lo);
    s = mix(s ^ hi);
    s = mix(s ^ u64::from(session.0));
    s = mix(s ^ u64::from(attempt));
    mix(s ^ tick)
}

/// A haggle roll in `0..100` from [`service_seed`].
pub fn haggle_roll(provider: ActorKey, session: SessionId, attempt: u16, tick: u64) -> u8 {
    u8::try_from(service_seed(provider, session, attempt, tick) % 100).unwrap_or(0)
}

/// splitmix64's finalizer (a bijection on `u64`).
const fn mix(mut z: u64) -> u64 {
    z = z.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}
