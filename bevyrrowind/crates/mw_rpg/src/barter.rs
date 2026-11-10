//! Morrowind barter on `sf_rpg::service` (services-adapter.md "Records and flags", "Offers",
//! "Stock and what a merchant buys", "Merchant gold"; slice A2a): [`MwServiceRules`] implements
//! [`ServiceRules`]'s trade path over the record db and a host view ([`MwTradeHost`]). The
//! service bits of the provider (the class's for autocalc NPCs) give the dialogue buttons and
//! what the merchant trades; prices and the haggle go through A1's [`barter_price`] and
//! [`barter_haggle`]. List services (training, travel, spells, repair) are slices A3-A6: their
//! offers show, their `entries` are empty for now.
//!
//! Ported from `OpenMW` (GPL-3.0) at commit `ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f`, paths
//! under `apps/openmw/` (line numbers from the spec where given; the rest name the function):
//! - `mwclass/npc.cpp` `getServices` (1118-1127: the class's services when autocalc, else
//!   `AIDT`), `getBaseGold` (1260-1264); `mwclass/creature.cpp` `getServices` (577-580),
//!   `getBaseGold` (796) ([`provider_facts`]).
//! - `components/esm3/loadnpc.hpp` 37-62: the service bits and `AllItems` (every "buys" bit
//!   except `MagicItems`) ([`ALL_ITEMS`]).
//! - `mwgui/dialogue.cpp` `updateTopicsPane` (586-622: button order, travel from the transport
//!   list) ([`MwService::BUTTON_ORDER`]); `onSelectListItem` (443-465: the general
//!   `checkServiceRefused` first, then the per-service one) with `mwbase/dialoguemanager.hpp`
//!   `ServiceType` and `mwdialogue/dialoguemanagerimp.cpp` `checkServiceRefused` (the first
//!   matching INFO of "Service Refusal", the service as the filter's choice) ([`MwService::choice`]).
//! - `mwclass/{weapon,armor,clothing,book,potion,ingredient,apparatus,lockpick,probe,repair,
//!   light,misc}.cpp` `canSell` ([`can_sell`]); `misc.cpp` `isGold` ([`GOLD_IDS`]).
//! - `mwgui/tradeitemmodel.cpp` `update` (168-215: merchant stock hides gold, items it would not
//!   buy, bound items and what the merchant has equipped); `mwgui/tradewindow.cpp` 199-216 (the
//!   merchant's inventory, then owned containers and items) ([`MwServiceRules`]'s `sells`,
//!   `stock_sources`).
//! - `mwmechanics/mechanicsmanagerimp.cpp` `isBoundItem` (the `sMagicBound*` GMST strings,
//!   lowercased) ([`BoundItems`]).
//! - `mwworld/containerstore.cpp` 1030-1044, `mwgui/containeritemmodel.cpp` ~134: negative
//!   inventory counts are restocking stacks, never used up ([`record_entry_stack`]).
//! - `mwgui/tradewindow.cpp` `onOfferButtonClicked` (447-457): the disposition change after each
//!   haggle for NPC merchants; `haggle` (99-111): a haggle won at a better price than offered
//!   uses Mercantile (`Mercantile_Success`, use 0) scaled by the haggled percentage
//!   ([`MwServiceRules`]'s `haggle`, [`mercantile_use`]).
//! - `mwgui/dialogue.cpp` 525-540 `restock`: the gold pool resets to the base gold after
//!   `fBarterGoldResetDelay` hours ([`MwServiceRules`]'s `purse_base`).
//!
//! Not here: the Bevy and dialogue wiring (`mw_rpg_bevy`), the stolen-goods veto (A7).
//!
//! Status: written 2026-11-08 without compiling: UNTESTED (owner test pass 2026-11-10).

use std::collections::HashSet;

use mw_stats::{MwGmst, SKILLS};
use sf_records::{RecordDb, RecordId};
use sf_rpg::service::{
    DealTotals, DealVeto, EntryId, EntryRefusal, HaggleOutcome, Holdings, LabelRef, PurseRule, QuoteLine, ServiceCtx,
    ServiceEffect, ServiceEntry, ServiceKind, ServiceOffer, ServiceRules, ServiceShape, TradeSession,
};
use sf_rpg::{ActorKey, AttrValue, Attrs, ConversationCtx, Holder, ItemStack, PlayerId};
use smallvec::SmallVec;
use tes3::esp::{
    Alchemy, Apparatus, Armor, Book, Class, Clothing, Creature, GameSetting, GameSettingValue, Ingredient, Light,
    Lockpick, MiscItem, MiscItemFlags, Npc, NpcFlags, Probe, RepairItem, ServiceFlags, Weapon, WeaponType,
};

use crate::services::{
    BarterPriceIn, BarterTerms, EffectiveValueIn, HaggleIn, HaggleOut, barter_haggle, barter_price, effective_value,
};

/// The ESM skill index of Mercantile (`ESM::Skill::Mercantile`).
const MERCANTILE: usize = 24;
/// SKIL use 0 of Mercantile, `ESM::Skill::Mercantile_Success`.
const MERCANTILE_SUCCESS: u8 = 0;

/// The player's Mercantile use after a haggle (`tradewindow.cpp` 99-111): an accepted haggle at a
/// better price than offered calls `skillUsageSucceeded(player, Mercantile, Mercantile_Success,
/// skillGain)` with [`HaggleOut::mercantile_scale`] as `skillGain`. A deal at the merchant's own
/// offer (accepted before the roll), a refused haggle and a creature merchant use nothing; a won
/// haggle whose `floor`ed gain is 0 is skipped too (`OpenMW` calls with 0, which progresses
/// nothing). The stat is the skill's sheet key (`mw_stats::SKILLS`); the host maps it.
#[must_use]
pub fn mercantile_use(out: &HaggleOut) -> Option<ServiceEffect> {
    (out.accepted && out.mercantile_scale > 0.0).then(|| ServiceEffect::UseSkill {
        stat: RecordId::new(SKILLS[MERCANTILE]),
        use_kind: MERCANTILE_SUCCESS,
        scale: out.mercantile_scale,
    })
}

/// The dialogue topic whose first matching INFO refuses a service (`checkServiceRefused`).
pub const SERVICE_REFUSAL_TOPIC: &str = "service refusal";
/// Item attr marking a restocking stack (a negative inventory-list count): infinite stock.
pub const ATTR_RESTOCK: &str = "mw.restock";
/// Item attr holding an item's current health (condition, or uses left for tools); absent means
/// full health (`OpenMW`'s cell-ref charge `-1`).
pub const ATTR_HEALTH: &str = "mw.health";
/// `Miscellaneous::isGold`: the gold piles, never bought or sold.
pub const GOLD_IDS: [&str; 5] = ["gold_001", "gold_005", "gold_010", "gold_025", "gold_100"];

/// `ESM::NPC::AllItems`: every "buys" bit except `MagicItems` (a merchant with only
/// `MagicItems` gets no Barter button).
pub const ALL_ITEMS: ServiceFlags = ServiceFlags::BARTERS_WEAPONS
    .union(ServiceFlags::BARTERS_ARMOR)
    .union(ServiceFlags::BARTERS_CLOTHING)
    .union(ServiceFlags::BARTERS_BOOKS)
    .union(ServiceFlags::BARTERS_INGREDIENTS)
    .union(ServiceFlags::BARTERS_LOCKPICKS)
    .union(ServiceFlags::BARTERS_PROBES)
    .union(ServiceFlags::BARTERS_LIGHTS)
    .union(ServiceFlags::BARTERS_APPARATUS)
    .union(ServiceFlags::BARTERS_REPAIR_ITEMS)
    .union(ServiceFlags::BARTERS_MISC_ITEMS)
    .union(ServiceFlags::BARTERS_ALCHEMY);

/// Morrowind's services, one per dialogue button.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MwService {
    Barter,
    Spells,
    Travel,
    Spellmaking,
    Enchanting,
    Training,
    Repair,
}

impl MwService {
    /// `updateTopicsPane`'s button order (persuasion and companion share are not services).
    pub const BUTTON_ORDER: [Self; 7] =
        [Self::Barter, Self::Spells, Self::Travel, Self::Spellmaking, Self::Enchanting, Self::Training, Self::Repair];

    /// The [`ServiceKind`] id.
    pub const fn id(self) -> &'static str {
        match self {
            Self::Barter => "mw.barter",
            Self::Spells => "mw.spells",
            Self::Travel => "mw.travel",
            Self::Spellmaking => "mw.spellmaking",
            Self::Enchanting => "mw.enchanting",
            Self::Training => "mw.training",
            Self::Repair => "mw.repair",
        }
    }

    /// The button text GMST (`updateTopicsPane`).
    pub const fn label_gmst(self) -> &'static str {
        match self {
            Self::Barter => "sBarter",
            Self::Spells => "sSpells",
            Self::Travel => "sTravel",
            Self::Spellmaking => "sSpellmakingMenuTitle",
            Self::Enchanting => "sEnchanting",
            Self::Training => "sServiceTrainingTitle",
            Self::Repair => "sRepair",
        }
    }

    /// `DialogueManager::ServiceType`: the dialogue filter's `Choice` value while checking this
    /// service's refusal (the general check uses `Any`, -1).
    pub const fn choice(self) -> i32 {
        match self {
            Self::Barter => 1,
            Self::Repair => 2,
            Self::Spells => 3,
            Self::Training => 4,
            Self::Travel => 5,
            Self::Spellmaking => 6,
            Self::Enchanting => 7,
        }
    }

    /// Which screen it opens.
    pub fn shape(self) -> ServiceShape {
        match self {
            Self::Barter => ServiceShape::Trade,
            Self::Spellmaking | Self::Enchanting => ServiceShape::Custom(RecordId::new(self.id())),
            Self::Spells | Self::Travel | Self::Training | Self::Repair => ServiceShape::List,
        }
    }

    pub fn kind(self) -> ServiceKind {
        ServiceKind(RecordId::new(self.id()))
    }

    pub fn from_kind(kind: &ServiceKind) -> Option<Self> {
        Self::BUTTON_ORDER.into_iter().find(|s| s.id() == kind.0.as_str())
    }

    /// Whether `updateTopicsPane` shows this button for the provider.
    pub fn offered(self, p: &ProviderFacts) -> bool {
        match self {
            Self::Barter => p.services.intersects(ALL_ITEMS),
            Self::Spells => p.services.contains(ServiceFlags::OFFERS_SPELLS),
            Self::Travel => p.travel,
            Self::Spellmaking => p.services.contains(ServiceFlags::OFFERS_SPELLMAKING),
            Self::Enchanting => p.services.contains(ServiceFlags::OFFERS_ENCHANTING),
            Self::Training => p.services.contains(ServiceFlags::OFFERS_TRAINING),
            Self::Repair => p.services.contains(ServiceFlags::OFFERS_REPAIRS),
        }
    }
}

/// What the provider's base record says about its services.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ProviderFacts {
    /// `getServices`.
    pub services: ServiceFlags,
    /// `getBaseGold` (`NPDT`/`CRDT` gold): the pool's base.
    pub gold: u32,
    pub creature: bool,
    /// A non-empty transport list (`DODT`).
    pub travel: bool,
}

/// The provider's [`ProviderFacts`] from its `NPC_` or `CREA` record. An autocalc NPC takes its
/// class's services (none if the class is missing; `OpenMW` would throw), others their `AIDT`.
pub fn provider_facts(records: &RecordDb, base: &RecordId) -> Option<ProviderFacts> {
    if let Some(npc) = records.get::<Npc>(base) {
        let services = if npc.npc_flags.contains(NpcFlags::AUTO_CALCULATE) {
            records.get::<Class>(&RecordId::new(&npc.class)).map_or(ServiceFlags::empty(), |c| c.data.services)
        } else {
            npc.ai_data.services
        };
        return Some(ProviderFacts {
            services,
            gold: npc.data.gold,
            creature: false,
            travel: !npc.travel_destinations.is_empty(),
        });
    }
    records.get::<Creature>(base).map(|c| ProviderFacts {
        services: c.ai_data.services,
        gold: c.data.gold,
        creature: true,
        travel: !c.travel_destinations.is_empty(),
    })
}

/// The item classes a merchant may trade.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MwItemClass {
    Weapon,
    Armor,
    Clothing,
    Book,
    Potion,
    Ingredient,
    Apparatus,
    Lockpick,
    Probe,
    RepairTool,
    Light,
    Misc,
}

/// One item record, as trading reads it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TradeItem {
    pub class: MwItemClass,
    /// The record's per-piece value (`getValue`).
    pub value: i32,
    /// `getItemMaxHealth` for classes with item health (`hasItemHealth`): weapons with health,
    /// armor, lockpicks, probes, repair tools (their uses).
    pub max_health: Option<i32>,
    /// A non-empty enchantment id (weapons, armor, clothing, books).
    pub enchanted: bool,
    /// A misc item with the `Key` flag.
    pub key: bool,
}

impl TradeItem {
    const fn plain(class: MwItemClass, value: i32) -> Self {
        Self { class, value, max_health: None, enchanted: false, key: false }
    }
}

fn clamp_u32(v: u32) -> i32 {
    i32::try_from(v).unwrap_or(i32::MAX)
}

/// Thrown weapons, arrows and bolts have no item health (`mwmechanics/weapontype.hpp`: no
/// `HasHealth` flag).
fn weapon_has_health(t: WeaponType) -> bool {
    !matches!(t, WeaponType::MarksmanThrown | WeaponType::Arrow | WeaponType::Bolt)
}

/// The [`TradeItem`] of an item record, `None` for ids that are not tradeable item records.
pub fn trade_item(records: &RecordDb, item: &RecordId) -> Option<TradeItem> {
    use MwItemClass as C;
    if let Some(w) = records.get::<Weapon>(item) {
        return Some(TradeItem {
            max_health: weapon_has_health(w.data.weapon_type).then_some(i32::from(w.data.health)),
            enchanted: !w.enchanting.is_empty(),
            ..TradeItem::plain(C::Weapon, clamp_u32(w.data.value))
        });
    }
    if let Some(a) = records.get::<Armor>(item) {
        return Some(TradeItem {
            max_health: Some(clamp_u32(a.data.health)),
            enchanted: !a.enchanting.is_empty(),
            ..TradeItem::plain(C::Armor, clamp_u32(a.data.value))
        });
    }
    if let Some(c) = records.get::<Clothing>(item) {
        return Some(TradeItem { enchanted: !c.enchanting.is_empty(), ..TradeItem::plain(C::Clothing, i32::from(c.data.value)) });
    }
    if let Some(b) = records.get::<Book>(item) {
        return Some(TradeItem { enchanted: !b.enchanting.is_empty(), ..TradeItem::plain(C::Book, clamp_u32(b.data.value)) });
    }
    if let Some(m) = records.get::<MiscItem>(item) {
        return Some(TradeItem {
            key: m.data.flags.contains(MiscItemFlags::KEY),
            ..TradeItem::plain(C::Misc, clamp_u32(m.data.value))
        });
    }
    if let Some(p) = records.get::<Lockpick>(item) {
        return Some(TradeItem { max_health: Some(clamp_u32(p.data.uses)), ..TradeItem::plain(C::Lockpick, clamp_u32(p.data.value)) });
    }
    if let Some(p) = records.get::<Probe>(item) {
        return Some(TradeItem { max_health: Some(clamp_u32(p.data.uses)), ..TradeItem::plain(C::Probe, clamp_u32(p.data.value)) });
    }
    if let Some(r) = records.get::<RepairItem>(item) {
        return Some(TradeItem {
            max_health: Some(clamp_u32(r.data.uses)),
            ..TradeItem::plain(C::RepairTool, clamp_u32(r.data.value))
        });
    }
    records
        .get::<Alchemy>(item)
        .map(|p| TradeItem::plain(C::Potion, clamp_u32(p.data.value)))
        .or_else(|| records.get::<Ingredient>(item).map(|i| TradeItem::plain(C::Ingredient, clamp_u32(i.data.value))))
        .or_else(|| records.get::<Apparatus>(item).map(|a| TradeItem::plain(C::Apparatus, clamp_u32(a.data.value))))
        .or_else(|| records.get::<Light>(item).map(|l| TradeItem::plain(C::Light, clamp_u32(l.data.value))))
}

/// `Miscellaneous::isGold`.
pub fn is_gold(item: &RecordId) -> bool {
    GOLD_IDS.contains(&item.as_str())
}

/// The class's `canSell(item, services)`: weapons, armor, clothing and books by their bit or
/// (`MagicItems` and enchanted); the rest by their own bit; misc never a key or gold.
pub fn can_sell(item: &TradeItem, id: &RecordId, services: ServiceFlags) -> bool {
    use MwItemClass as C;
    let magic = services.contains(ServiceFlags::BARTERS_ENCHANTED_ITEMS) && item.enchanted;
    let bit = |flag| services.contains(flag);
    match item.class {
        C::Weapon => bit(ServiceFlags::BARTERS_WEAPONS) || magic,
        C::Armor => bit(ServiceFlags::BARTERS_ARMOR) || magic,
        C::Clothing => bit(ServiceFlags::BARTERS_CLOTHING) || magic,
        C::Book => bit(ServiceFlags::BARTERS_BOOKS) || magic,
        C::Potion => bit(ServiceFlags::BARTERS_ALCHEMY),
        C::Ingredient => bit(ServiceFlags::BARTERS_INGREDIENTS),
        C::Apparatus => bit(ServiceFlags::BARTERS_APPARATUS),
        C::Lockpick => bit(ServiceFlags::BARTERS_LOCKPICKS),
        C::Probe => bit(ServiceFlags::BARTERS_PROBES),
        C::RepairTool => bit(ServiceFlags::BARTERS_REPAIR_ITEMS),
        C::Light => bit(ServiceFlags::BARTERS_LIGHTS),
        C::Misc => !item.key && bit(ServiceFlags::BARTERS_MISC_ITEMS) && !is_gold(id),
    }
}

/// The bound item ids (`isBoundItem`): the values of every `sMagicBound*` string GMST.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BoundItems(pub HashSet<RecordId>);

impl BoundItems {
    pub fn from_records(records: &RecordDb) -> Self {
        Self(
            records
                .iter::<GameSetting>()
                .filter(|(id, _)| id.as_str().starts_with("smagicbound"))
                .filter_map(|(_, g)| match &g.value {
                    GameSettingValue::String(s) => Some(RecordId::new(s)),
                    _ => None,
                })
                .collect(),
        )
    }

    pub fn contains(&self, item: &RecordId) -> bool {
        self.0.contains(item)
    }
}

/// An inventory-list entry's count as a stack: a negative count is a restocking stack of
/// `|count|` pieces, marked with [`ATTR_RESTOCK`] (`sf_rpg` counts are unsigned).
pub fn record_entry_stack(count: i32) -> (u32, Attrs) {
    let attrs = if count < 0 { Attrs::new().with(ATTR_RESTOCK, AttrValue::Int(1)) } else { Attrs::new() };
    (count.unsigned_abs(), attrs)
}

/// Whether a stack's attrs mark it as restocking.
pub fn is_restocking(attrs: &Attrs) -> bool {
    attrs.get(&RecordId::new(ATTR_RESTOCK)).is_some()
}

/// The item's current health from [`ATTR_HEALTH`], `max` when absent (`getItemHealth`).
pub fn item_health(attrs: &Attrs, max: i32) -> i32 {
    match attrs.get(&RecordId::new(ATTR_HEALTH)) {
        Some(AttrValue::Int(v)) => i32::try_from(*v).unwrap_or(if *v < 0 { 0 } else { i32::MAX }),
        _ => max,
    }
}

/// The host's runtime state the barter rules read: inventories (for what the merchant has
/// equipped), the barter terms from the sheets and the derived disposition, the merchant's
/// owned stock, and the dialogue filter for refusals.
pub trait MwTradeHost: Holdings {
    /// The [`BarterTerms`] of this player with this provider (stats-adapter "Fatigue term", the
    /// disposition port). `None` prices at the base value.
    fn barter_terms(&self, player: PlayerId, provider: ActorKey) -> Option<BarterTerms>;
    /// Containers and world items owned by the provider in loaded cells (`getContainersOwnedBy`,
    /// `getItemsOwnedBy`), as holders.
    fn owned_stock(&self, provider: ActorKey) -> SmallVec<[Holder; 4]> {
        let _ = provider;
        SmallVec::new()
    }
    /// The first INFO of [`SERVICE_REFUSAL_TOPIC`] that passes the filters for this
    /// conversation, `choice` being the service (`None` = `ServiceType::Any`, where `OpenMW`
    /// rejects every `Choice` condition), with `OpenMW`'s inverted disposition check
    /// (`filter.list(dialogue, false, false, true)`). Returns the INFO id.
    fn service_refusal(&self, ctx: &ConversationCtx) -> Option<RecordId>;
}

/// Trade switches (services-adapter.md "Options").
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BarterOptions {
    /// `OpenMW`'s 75% barter cap (on in the modern rules; off is classic).
    pub cap_75: bool,
}

impl Default for BarterOptions {
    fn default() -> Self {
        Self { cap_75: true }
    }
}

/// Morrowind's [`ServiceRules`] (the trade path; list services come in A3-A6).
pub struct MwServiceRules<'a, H: ?Sized> {
    pub records: &'a RecordDb,
    pub gmst: &'a MwGmst,
    pub bound: &'a BoundItems,
    pub host: &'a H,
    pub options: BarterOptions,
}

impl<'a, H: MwTradeHost + ?Sized> MwServiceRules<'a, H> {
    pub fn new(records: &'a RecordDb, gmst: &'a MwGmst, bound: &'a BoundItems, host: &'a H) -> Self {
        Self { records, gmst, bound, host, options: BarterOptions::default() }
    }

    fn facts(&self, ctx: &ServiceCtx) -> Option<ProviderFacts> {
        provider_facts(self.records, &ctx.provider_base)
    }

    /// Whether the provider would trade this stack at all (class filter and bound items).
    fn trades(&self, ctx: &ServiceCtx, stack: &ItemStack) -> bool {
        let Some(facts) = self.facts(ctx) else { return false };
        if self.bound.contains(&stack.item) {
            return false;
        }
        trade_item(self.records, &stack.item).is_some_and(|item| can_sell(&item, &stack.item, facts.services))
    }
}

/// `fBarterGoldResetDelay` in whole hours, rounded up (`OpenMW` compares float hours; the
/// vanilla 24 is whole).
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, reason = "clamped to >= 0 first; NaN gives 0")]
fn reset_hours(g: &MwGmst) -> u32 {
    let h = g.barter_gold_reset_delay.ceil();
    if h <= 0.0 { 0 } else { h as u32 }
}

fn clamp_i64(v: i64) -> i32 {
    i32::try_from(v).unwrap_or(if v < 0 { i32::MIN } else { i32::MAX })
}

impl<H: MwTradeHost + ?Sized> ServiceRules for MwServiceRules<'_, H> {
    fn offers(&self, ctx: &ServiceCtx) -> SmallVec<[ServiceOffer; 8]> {
        let Some(facts) = self.facts(ctx) else { return SmallVec::new() };
        MwService::BUTTON_ORDER
            .into_iter()
            .filter(|s| s.offered(&facts))
            .map(|s| ServiceOffer {
                provider: ctx.provider,
                kind: s.kind(),
                shape: s.shape(),
                label: LabelRef(RecordId::new(s.label_gmst())),
            })
            .collect()
    }

    /// The general check (`ServiceType::Any`) first, then the service's own.
    fn refusal(&self, ctx: &ServiceCtx, kind: &ServiceKind) -> Option<(RecordId, RecordId)> {
        let topic = RecordId::new(SERVICE_REFUSAL_TOPIC);
        let mut conv = ConversationCtx {
            player: ctx.player,
            speaker: ctx.provider,
            speaker_base: ctx.provider_base.clone(),
            choice: None,
        };
        if let Some(info) = self.host.service_refusal(&conv) {
            return Some((topic, info));
        }
        conv.choice = Some(MwService::from_kind(kind)?.choice());
        self.host.service_refusal(&conv).map(|info| (topic, info))
    }

    /// The merchant's own inventory first, then what it owns in loaded cells.
    fn stock_sources(&self, ctx: &ServiceCtx) -> SmallVec<[Holder; 4]> {
        let own = Holder::Actor(ctx.provider);
        let mut sources: SmallVec<[Holder; 4]> = SmallVec::new();
        sources.push(own);
        for holder in self.host.owned_stock(ctx.provider) {
            if !sources.contains(&holder) {
                sources.push(holder);
            }
        }
        sources
    }

    /// `canSell` with the provider's services; never gold, a key or a bound item. Equipped
    /// player items are tradeable (the transfer unequips).
    fn buys(&self, ctx: &ServiceCtx, stack: &ItemStack) -> bool {
        self.trades(ctx, stack)
    }

    /// `TradeItemModel::update`: hides gold, what the merchant would not buy, bound items and
    /// what the source has equipped.
    fn sells(&self, ctx: &ServiceCtx, source: Holder, stack: &ItemStack) -> bool {
        if !self.trades(ctx, stack) {
            return false;
        }
        !self.host.loadout(source).is_some_and(|l| l.is_equipped(stack.id))
    }

    fn restocking(&self, _ctx: &ServiceCtx, _source: Holder, stack: &ItemStack) -> bool {
        is_restocking(&stack.attrs)
    }

    /// The bought copy drops [`ATTR_RESTOCK`], so it stacks with the player's ordinary items.
    fn restocked_copy_attrs(&self, _ctx: &ServiceCtx, attrs: &Attrs) -> Attrs {
        let mut copy = attrs.clone();
        copy.remove(&RecordId::new(ATTR_RESTOCK));
        copy
    }

    /// `OpenMW` applies `iBarterSuccessDisposition` on every accepted deal: at the merchant's
    /// offer the haggle accepts at once (`tradewindow.cpp` `onOfferButtonClicked`), so this is
    /// the haggle's success effects.
    fn plain_deal_effects(&self, ctx: &ServiceCtx, deal: &DealTotals) -> Vec<ServiceEffect> {
        self.haggle(ctx, deal, 0).effects
    }

    /// `mw.barter.price` of the line's effective value (condition-scaled, whole line).
    fn quote(&self, ctx: &ServiceCtx, line: &QuoteLine<'_>) -> i64 {
        let Some(item) = trade_item(self.records, line.item) else { return 0 };
        let health = item.max_health.map(|max| (item_health(line.attrs, max), max));
        let base = effective_value(&EffectiveValueIn { value: item.value, health, count: line.count });
        let price = match self.host.barter_terms(ctx.player, ctx.provider) {
            Some(terms) => barter_price(&BarterPriceIn { terms, base, side: line.side, cap_75: self.options.cap_75 }),
            None => base,
        };
        i64::from(price.max(0))
    }

    /// `mw.barter.haggle` with `roll + 1` as `rollDice(100) + 1`. Effects in `OpenMW`'s order:
    /// the Mercantile use of a won haggle ([`mercantile_use`], inside `haggle`), then an NPC
    /// merchant's disposition change (`onOfferButtonClicked`, after it).
    fn haggle(&self, ctx: &ServiceCtx, deal: &DealTotals, roll: u8) -> HaggleOutcome {
        let Some(terms) = self.host.barter_terms(ctx.player, ctx.provider) else {
            return HaggleOutcome { accepted: deal.balance <= deal.merchant_offer, effects: Vec::new() };
        };
        let input = HaggleIn::new(
            terms,
            clamp_i64(deal.balance),
            clamp_i64(deal.merchant_offer),
            i32::from(roll.min(99)) + 1,
            self.gmst,
        );
        let out = barter_haggle(&input);
        let mut effects = Vec::new();
        effects.extend(mercantile_use(&out));
        if out.disposition_delta != 0 {
            effects.push(ServiceEffect::Disposition { delta: out.disposition_delta });
        }
        HaggleOutcome { accepted: out.accepted, effects }
    }

    /// The stolen-goods veto is A7 (needs the crime port's stolen-items map).
    fn vet_deal(&self, _ctx: &ServiceCtx, _session: &TradeSession) -> Result<(), DealVeto> {
        Ok(())
    }

    /// List services are A3-A6.
    fn entries(&self, _ctx: &ServiceCtx, _kind: &ServiceKind) -> Vec<ServiceEntry> {
        Vec::new()
    }

    fn buy_entry(
        &self,
        _ctx: &ServiceCtx,
        _kind: &ServiceKind,
        _entry: &EntryId,
    ) -> Result<Vec<ServiceEffect>, EntryRefusal> {
        Err(EntryRefusal::Unknown)
    }

    /// A pool of the base gold, reset after `fBarterGoldResetDelay` hours.
    fn purse_base(&self, ctx: &ServiceCtx) -> PurseRule {
        let base = self.facts(ctx).map_or(0, |f| i64::from(f.gold));
        PurseRule::Pool { base, reset_hours: reset_hours(self.gmst) }
    }
}
