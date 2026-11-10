//! NPC services in the game host (slice TR1, services-adapter.md "As built (TR1)"): the
//! dialogue's service buttons, `sf_rpg::service`'s host steps over [`Rpg`]'s state, and the
//! trade session's requests, settled into the world. The trade screen over them is
//! `trade_screen.rs` (TR2; [`Rpg::trade_quotes`] gives its money and price columns).
//!
//! List services (slice A3, training first): opening one lists its rows (`EntriesListed`, kept
//! as an [`OpenList`] and re-sent whenever they change, [`relist_services`]: a sheet stepped
//! after a purchase); `BuyEntry` on an open list re-checks the row ([`purchase`]) and applies
//! its effects in order ([`Rpg::apply_service_effects`]: the payment into the provider's pool,
//! the skill raise, the time passed), then `EntryBought`. The list screen is
//! `service_list_screen.rs`.
//!
//! Net-ready (services.md "Co-op"): [`ServiceRequest`]s in ([`Rpg::service_request`]),
//! [`ServiceEvent`]s out ([`Rpg::take_service_events`], sent as [`ServiceEventMessage`]s),
//! sessions and purses owned by the host. Rolls are [`haggle_roll`] over (provider, session,
//! attempt, tick).
//!
//! `OpenMW` behaviour (commit `ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f`, paths under
//! `apps/openmw/`):
//! - `mwgui/dialogue.cpp` `updateTopicsPane`: Persuasion first (NPCs), then the services in
//!   [`MwService::BUTTON_ORDER`], labelled from their GMSTs; `onSelectListItem`: a button does
//!   nothing during a choice or after goodbye, the general refusal check comes first.
//! - `mwdialogue/dialoguemanagerimp.cpp` `checkServiceRefused`: the refusal's INFO is shown as a
//!   dialogue line and its result script runs ([`mw_rpg::MwDialogueRules::service_refusal`]).
//! - `mwdialogue/dialoguemanagerimp.cpp` `applyBarterDispositionChange`: a deal's disposition
//!   change goes on the speaker's base at once, through the conversation's persuasion
//!   bookkeeping, and is temporary (`barter disposition change is permanent` defaults off), so
//!   goodbye drops it ([`Rpg::end_talk`]).
//! - `mwgui/tradewindow.cpp` `onOfferButtonClicked`: an accepted deal closes the barter window
//!   (here: the session), a failed haggle keeps it open; `haggle` 99-111: a won haggle's
//!   Mercantile use goes on the player's sheet ([`mw_rpg::mercantile_use`], slice A2b).
//! - `mwworld/containerstore.cpp` `fill` (the NPC's inventory list, levelled lists resolved;
//!   negative counts restock): a merchant's stock is seeded from its record on first access
//!   ([`seed_store`]), its armor and clothing worn (first piece per slot, as
//!   [`mw_rpg::record_clothing_value`]; `OpenMW`'s `autoEquip` picks by value, not modelled).
//! - `mwgui/trainingwindow.cpp` `onTrainingSelected` / `onTrainingFinished` (rules in
//!   [`mw_rpg::training_purchase`]): the skill rises as a trainer level-up
//!   (`skillLevelUp(player, skill, "trainer")`: `IncreaseSource::Training`, which counts toward
//!   the level), the gold goes into the trainer's pool, then `rest(2, false)` (the player's
//!   awake rest: fatigue; `OpenMW` rests every actor in range, here only the player, logged)
//!   and `advanceTime(2)` (`mwworld/datetimemanager.cpp` `advanceTime`: the `MWScript` calendar
//!   globals, [`mw_rpg::advance_calendar`], and the sky clock, [`advance_sky`], until ROADMAP
//!   1.17's single clock). Modern differences: the list stays open after a purchase (`OpenMW`
//!   closes the training window and the dialogue), no fade or progress bar (2 game hours pass
//!   at once; `OpenMW` steps them 1/60 s apart under a 0.2 s fade).

use bevy_ecs::prelude::*;
use bevy_log::prelude::*;
use mw_rpg::{
    BarterTerms, Calendar, MwDialogueRules, MwEquipKind, MwEquipRules, MwService, MwServiceRules,
    MwSheets, MwTradeHost, MwView, NoSheets, PartyTerms, PersuaderStats, TrainingSheets,
    advance_calendar, is_gold, persuader_stats, record_entry_stack,
};
use sf_records::{RecordDb, RecordId};
use sf_rpg::service::{
    Currency, EntryId, Holdings, MerchantMoney, OpenError, Purchase, ServiceCtx, ServiceEffect,
    ServiceEntry, ServiceEvent, ServiceEventKind, ServiceKind, ServiceOffer, ServiceRequest,
    ServiceRules, ServiceShape, SessionId, Settlement, Side, TradeConfig, TradeEnv, TradeRefusal,
    TradeSession, haggle_roll, open, pane_prices, payment_command, purchase,
};
use sf_rpg::{
    ActorKey, CharacterRpg, ConversationCtx, Conversations, GameTime, Holder, Loadout, PlayerId,
    RpgCommand, RpgEvent, SessionState, StackId,
};
use sf_rpg_ui::{DialogueAction, DialogueRequest, DialogueView};
use sf_sky_bevy::{SkyConfig, SkyTime};
use sf_stats_bevy::sf_stats::{CauseKind, IncreaseSource, StatOp, StatOwner};
use sf_stats_bevy::{StatCommand, StatTick};
use tes3::esp::{LeveledItem, Npc};

use super::persuasion::gmst_text;
use super::{DialogueSheets, Rpg, Talk, View, id, player_key};
use crate::inventory::GOLD;

/// Morrowind's trade settings: gold (`gold_001`) is the money, and the player may haggle.
pub fn trade_config() -> TradeConfig {
    TradeConfig {
        currency: Currency { item: id(GOLD) },
        haggle: true,
    }
}

/// A service button's text without its GMST (Morrowind.esm's English value).
pub(super) fn label_fallback(service: MwService) -> &'static str {
    match service {
        MwService::Barter => "Barter",
        MwService::Spells => "Spells",
        MwService::Travel => "Travel",
        MwService::Spellmaking => "Spellmaking",
        MwService::Enchanting => "Enchanting",
        MwService::Training => "Training",
        MwService::Repair => "Repair",
    }
}

/// A [`ServiceEvent`] for the screens (the trade screen opens on `ServiceOpened`,
/// `trade_screen.rs`).
#[derive(Message, Clone, Debug, PartialEq)]
pub struct ServiceEventMessage(pub ServiceEvent);

/// An open list service: who, with whom, which service, and the rows last listed
/// (`EntriesListed` goes out again only when they change).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpenList {
    pub player: PlayerId,
    pub provider: ActorKey,
    pub kind: ServiceKind,
    pub entries: Vec<ServiceEntry>,
}

/// The `MWScript` globals of the calendar `advanceTime` writes, in [`Calendar`]'s field order.
const CALENDAR_GLOBALS: [&str; 5] = ["gamehour", "dayspassed", "day", "month", "year"];

/// [`Rpg::trade_quotes`]: an open session's provider money and price columns.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TradeQuotes {
    pub money: MerchantMoney,
    /// The provider's stock, buy-side quotes (`None`: not sold).
    pub provider: Vec<(StackId, Option<i64>)>,
    /// The player's items, sell-side quotes (`None`: currency, or not bought).
    pub player: Vec<(StackId, Option<i64>)>,
}

/// The barter rules' view of the host: inventories by holder (one player character until co-op
/// N slices), the barter terms from the sheets and the derived disposition, refusals through the
/// dialogue filters.
impl Holdings for View<'_> {
    fn loadout(&self, holder: Holder) -> Option<&Loadout> {
        match holder {
            Holder::Player(PlayerId::HOST) => Some(&self.s.pc.loadout),
            Holder::Player(_) => None,
            Holder::Actor(key) => self.s.inventories.get(&key).map(|c| &c.loadout),
        }
    }
}

/// `owned_stock` keeps its default (none): owned containers and world items in loaded cells
/// (`tradewindow.cpp` 199-216) wait for item ownership in the world (logged in TR1's handoff).
impl MwTradeHost for View<'_> {
    /// Modified Mercantile, Luck, Personality and the fatigue term of both sheets, and the
    /// speaker's derived disposition (the conversation's temporary changes are on its base).
    fn barter_terms(&self, player: PlayerId, provider: ActorKey) -> Option<BarterTerms> {
        let actor = self.c.actors.get(&provider)?;
        let t = self.sheets.table()?;
        let pc = persuader_stats(t, &self.sheets.player(player)?.view(&t.table));
        let npc = persuader_stats(t, &self.sheets.actor(provider)?.view(&t.table));
        let g = &self.c.gmst;
        let terms = |s: &PersuaderStats| {
            PartyTerms::new(
                s.mercantile,
                s.luck,
                s.personality,
                s.fatigue_current,
                s.fatigue_max,
                g,
            )
        };
        let ctx = ConversationCtx {
            player,
            speaker: provider,
            speaker_base: actor.facts.id.clone(),
            choice: None,
        };
        Some(BarterTerms {
            merchant_is_creature: !actor.facts.npc,
            disposition: MwView::disposition(self, &ctx),
            pc: terms(&pc),
            npc: terms(&npc),
        })
    }

    fn service_refusal(&self, ctx: &ConversationCtx) -> Option<RecordId> {
        MwDialogueRules::new(&self.c.dialogue, self).service_refusal(ctx)
    }

    fn training_sheets(&self, player: PlayerId, provider: ActorKey) -> Option<TrainingSheets<'_>> {
        Some(TrainingSheets {
            table: self.sheets.table()?,
            player: self.sheets.player(player)?,
            trainer: self.sheets.actor(provider)?,
        })
    }
}

/// An NPC's store seeded from its record's inventory list: levelled lists skipped (no levelled
/// items yet), a negative count a restocking stack ([`record_entry_stack`]), gold as
/// [`GOLD`], armor and clothing worn (module docs).
pub fn seed_store(records: &RecordDb, base: &RecordId) -> CharacterRpg {
    let mut store = CharacterRpg::default();
    let Some(npc) = records.get::<Npc>(base) else {
        return store;
    };
    let equip = MwEquipRules::from_records(records);
    for (count, item) in &npc.inventory {
        let item = id(item.as_str());
        if records.get::<LeveledItem>(&item).is_some() {
            debug!("services: {base}'s levelled list {item} skipped (no levelled items yet)");
            continue;
        }
        let (n, attrs) = record_entry_stack(*count);
        if n == 0 {
            continue;
        }
        let item = if is_gold(&item) { id(GOLD) } else { item };
        let Ok(stack) = store.loadout.add(item.clone(), n, attrs) else {
            continue;
        };
        if *count > 0
            && let Some(kind @ (MwEquipKind::Armor(_) | MwEquipKind::Clothing(_))) =
                equip.kind(&item)
            && kind
                .slots()
                .0
                .iter()
                .any(|s| store.loadout.slot(s.id()).is_none())
        {
            let _ = equip.equip(&mut store.loadout, stack);
        }
    }
    store
}

impl Rpg {
    /// The game clock as the services read it: the `DaysPassed` and `GameHour` globals
    /// (`OpenMW`'s `getTimeStamp`).
    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "game clock globals, clamped to the u32/u16 ranges first"
    )]
    pub fn game_time(&self) -> GameTime {
        let v = View {
            c: &self.content,
            s: &self.state,
            sheets: &NoSheets,
        };
        let read = |name: &str| {
            v.global_value(&id(name))
                .map_or(0.0, mw_rpg::GlobalNumber::as_f32)
        };
        let day = read("dayspassed").clamp(0.0, u32::MAX as f32) as u32;
        let minute = (read("gamehour") * 60.0).clamp(0.0, 1439.0) as u16;
        GameTime { day, minute }
    }

    /// The provider's store, seeded from its record on first access ([`seed_store`]).
    pub(super) fn npc_store(&mut self, key: ActorKey) -> &mut CharacterRpg {
        let records = &self.content.records;
        let base = self.content.actors.get(&key).map(|a| a.facts.id.clone());
        self.state
            .inventories
            .entry(key)
            .or_insert_with(|| base.map(|b| seed_store(records, &b)).unwrap_or_default())
    }

    /// The service context of `talk`'s player with its speaker, now.
    fn service_ctx(&self, talk: &Talk, tick: u64) -> Option<ServiceCtx> {
        let ctx = talk.session.ctx();
        let actor = self.content.actors.get(&talk.speaker)?;
        Some(ServiceCtx {
            player: ctx.player,
            provider: talk.speaker,
            provider_base: actor.facts.id.clone(),
            now: self.game_time(),
            tick,
        })
    }

    /// `talk`'s speaker's offers, in button order (`MwServiceRules::offers`: the record's
    /// service flags; the class's for autocalc NPCs).
    pub fn service_offers(&self, talk: &Talk) -> Vec<ServiceOffer> {
        let Some(ctx) = self.service_ctx(talk, 0) else {
            return Vec::new();
        };
        let v = View {
            c: &self.content,
            s: &self.state,
            sheets: &NoSheets,
        };
        let rules = MwServiceRules::new(
            &self.content.records,
            &self.content.gmst,
            &self.content.bound,
            &v,
        );
        rules.offers(&ctx).into_vec()
    }

    /// One button per offer ([`DialogueAction::from_offer`]), labelled from its GMST.
    pub(super) fn service_actions(&self, talk: &Talk) -> Vec<DialogueAction> {
        let db = &self.content.records;
        let offers = self.service_offers(talk);
        offers
            .iter()
            .map(|offer| {
                let fallback = match MwService::from_kind(&offer.kind) {
                    Some(service) => label_fallback(service),
                    None => offer.kind.0.as_str(),
                };
                DialogueAction::from_offer(offer, gmst_text(db, offer.label.0.as_str(), fallback))
            })
            .collect()
    }

    /// A pressed service button ([`DialogueRequest::service_open`]): `true` if it was one.
    pub(super) fn route_service_action(
        &mut self,
        talk: &mut Talk,
        view: &mut DialogueView,
        request: &DialogueRequest,
        sheets: &dyn MwSheets,
        tick: u64,
    ) -> bool {
        let Some(open) = request.service_open(&self.service_offers(talk)) else {
            return false;
        };
        self.service_request(talk, view, &open, sheets, tick);
        true
    }

    /// Closes every open trade session and list (a load: sessions are never saved, services.md
    /// "Saves"; staged items never moved, so nothing is lost).
    pub fn close_service_sessions(&mut self) {
        self.sessions.close_all();
        self.lists.clear();
    }

    /// Closes `player`'s list with `provider` (the list screen's Back).
    pub fn close_service_list(&mut self, player: PlayerId, provider: ActorKey) {
        self.lists
            .retain(|l| l.player != player || l.provider != provider);
    }

    /// Closes `player`'s lists (goodbye).
    pub(super) fn close_service_lists(&mut self, player: PlayerId) {
        self.lists.retain(|l| l.player != player);
    }

    /// `player`'s open list with `provider`, if any.
    pub fn service_list(&self, player: PlayerId, provider: ActorKey) -> Option<&OpenList> {
        self.lists
            .iter()
            .find(|l| l.player == player && l.provider == provider)
    }

    /// The rows of list service `kind` with `provider` for `player` now (`ServiceRules::entries`
    /// with `sheets`).
    pub fn service_entries(
        &self,
        player: PlayerId,
        provider: ActorKey,
        kind: &ServiceKind,
        sheets: &dyn MwSheets,
        tick: u64,
    ) -> Vec<ServiceEntry> {
        let Some(actor) = self.content.actors.get(&provider) else {
            return Vec::new();
        };
        let ctx = ServiceCtx {
            player,
            provider,
            provider_base: actor.facts.id.clone(),
            now: self.game_time(),
            tick,
        };
        let v = View {
            c: &self.content,
            s: &self.state,
            sheets,
        };
        let rules = MwServiceRules::new(
            &self.content.records,
            &self.content.gmst,
            &self.content.bound,
            &v,
        );
        rules.entries(&ctx, kind)
    }

    /// The open lists whose rows changed since last listed, with their new rows.
    fn stale_lists(&self, sheets: &dyn MwSheets, tick: u64) -> Vec<(usize, Vec<ServiceEntry>)> {
        self.lists
            .iter()
            .enumerate()
            .filter_map(|(i, l)| {
                let entries = self.service_entries(l.player, l.provider, &l.kind, sheets, tick);
                (entries != l.entries).then_some((i, entries))
            })
            .collect()
    }

    /// Re-lists open lists whose rows changed: `EntriesListed` for each.
    pub fn relist(&mut self, sheets: &dyn MwSheets, tick: u64) {
        for (i, entries) in self.stale_lists(sheets, tick) {
            let l = &mut self.lists[i];
            l.entries.clone_from(&entries);
            let e = ServiceEvent {
                tick,
                player: l.player,
                provider: l.provider,
                kind: ServiceEventKind::EntriesListed {
                    kind: l.kind.clone(),
                    entries,
                },
            };
            self.service_events.push(e);
        }
    }

    /// Service events since the last call (`ServiceOpened`, `TradeQuoted`, ...).
    pub fn take_service_events(&mut self) -> Vec<ServiceEvent> {
        std::mem::take(&mut self.service_events)
    }

    /// An open trade session.
    pub fn trade_session(&self, id: SessionId) -> Option<&TradeSession> {
        self.sessions.get(id)
    }

    /// What the trade screen shows of open session `id` besides its panes (TR2): the
    /// provider's money and both price columns ([`pane_prices`]: the provider's stock on the
    /// buy side, the player's items on the sell side), quoted with `sheets` at tick `tick`.
    /// `None` without that session or its provider.
    pub fn trade_quotes(
        &self,
        id: SessionId,
        sheets: &dyn MwSheets,
        tick: u64,
    ) -> Option<TradeQuotes> {
        let session = self.sessions.get(id)?;
        let actor = self.content.actors.get(&session.provider)?;
        let ctx = ServiceCtx {
            player: session.player,
            provider: session.provider,
            provider_base: actor.facts.id.clone(),
            now: self.game_time(),
            tick,
        };
        let config = trade_config();
        let v = View {
            c: &self.content,
            s: &self.state,
            sheets,
        };
        let rules = MwServiceRules::new(
            &self.content.records,
            &self.content.gmst,
            &self.content.bound,
            &v,
        );
        let env = TradeEnv {
            ctx: &ctx,
            rules: &rules,
            holdings: &v,
            config: &config,
        };
        Some(TradeQuotes {
            money: MerchantMoney::of(rules.purse_base(&ctx), &self.services, ctx.provider),
            provider: pane_prices(&env, Side::Buy, Holder::Actor(ctx.provider)),
            player: pane_prices(&env, Side::Sell, Holder::Player(ctx.player)),
        })
    }

    /// Applies one [`ServiceRequest`] of `talk`'s player at simulation tick `tick` (the haggle
    /// roll's seed) and queues its events. Requests for another provider than `talk`'s speaker,
    /// or for another player's session, are ignored (logged).
    pub fn service_request(
        &mut self,
        talk: &mut Talk,
        view: &mut DialogueView,
        request: &ServiceRequest,
        sheets: &dyn MwSheets,
        tick: u64,
    ) {
        let Some(ctx) = self.service_ctx(talk, tick) else {
            warn!("services: {request:?} for an unregistered speaker");
            return;
        };
        match request {
            ServiceRequest::Open { provider, kind } if *provider == ctx.provider => {
                self.open_service(talk, view, &ctx, kind, sheets);
            }
            ServiceRequest::BuyEntry {
                provider,
                kind,
                entry,
            } if *provider == ctx.provider => {
                let gold = self.state.pc.loadout.count(&id(GOLD));
                let v = View {
                    c: &self.content,
                    s: &self.state,
                    sheets,
                };
                let rules = MwServiceRules::new(
                    &self.content.records,
                    &self.content.gmst,
                    &self.content.bound,
                    &v,
                );
                let open = self
                    .service_list(ctx.player, ctx.provider)
                    .is_some_and(|l| l.kind == *kind);
                if !open {
                    warn!("services: {request:?} without its list open");
                    return;
                }
                match purchase(&rules, &ctx, kind, entry, gold) {
                    Err(reason) => {
                        let kind = ServiceEventKind::EntryRefused {
                            kind: kind.clone(),
                            entry: entry.clone(),
                            reason,
                        };
                        self.service_events.push(event(&ctx, kind));
                    }
                    Ok(p) => self.buy_listed(talk, &ctx, kind, entry, p, sheets),
                }
            }
            ServiceRequest::Open { .. } | ServiceRequest::BuyEntry { .. } => {
                warn!(
                    "services: {request:?} names another provider than {:?}",
                    ctx.provider
                );
            }
            ServiceRequest::Stage { session, .. }
            | ServiceRequest::Unstage { session, .. }
            | ServiceRequest::SetBalance { session, .. }
            | ServiceRequest::Accept { session }
            | ServiceRequest::Cancel { session } => {
                self.trade_request(talk, &ctx, *session, request, sheets);
            }
        }
    }

    /// `ServiceRequest::Open` (`onSelectListItem`): only while talking; refusal first (a dialogue
    /// line), then the purse restock and, for barter, the session.
    fn open_service(
        &mut self,
        talk: &mut Talk,
        view: &mut DialogueView,
        ctx: &ServiceCtx,
        kind: &ServiceKind,
        sheets: &dyn MwSheets,
    ) {
        if *talk.session.state() != SessionState::Talking {
            debug!(
                "services: {kind:?} ignored outside talking ({:?})",
                talk.session.state()
            );
            return;
        }
        // The merchant's stock exists before the session reads it.
        self.npc_store(ctx.provider);
        // TODO(co-op): the host's claim table (rpg.md "Co-op"); one conversation for now.
        let mut claims = Conversations::default();
        let _ = claims.claim(ctx.provider, ctx.player);
        let opened = {
            let v = View {
                c: &self.content,
                s: &self.state,
                sheets,
            };
            let rules = MwServiceRules::new(
                &self.content.records,
                &self.content.gmst,
                &self.content.bound,
                &v,
            );
            open(
                &rules,
                ctx,
                kind,
                &claims,
                &mut self.services,
                &mut self.sessions,
            )
        };
        match opened {
            Ok(o) => {
                if let Some(gold) = o.restocked {
                    self.service_events
                        .push(event(ctx, ServiceEventKind::PurseRestocked { gold }));
                }
                debug!(
                    "services: {kind:?} opened with {:?} ({:?})",
                    ctx.provider, o.session
                );
                let list = o.shape == ServiceShape::List;
                let opened = ServiceEventKind::ServiceOpened {
                    kind: kind.clone(),
                    shape: o.shape,
                    session: o.session,
                };
                self.service_events.push(event(ctx, opened));
                if list {
                    let entries =
                        self.service_entries(ctx.player, ctx.provider, kind, sheets, ctx.tick);
                    self.close_service_lists(ctx.player);
                    self.lists.push(OpenList {
                        player: ctx.player,
                        provider: ctx.provider,
                        kind: kind.clone(),
                        entries: entries.clone(),
                    });
                    let listed = ServiceEventKind::EntriesListed {
                        kind: kind.clone(),
                        entries,
                    };
                    self.service_events.push(event(ctx, listed));
                }
            }
            Err(OpenError::Refused { topic, response }) => {
                let out = {
                    let v = View {
                        c: &self.content,
                        s: &self.state,
                        sheets,
                    };
                    let rules = MwDialogueRules::new(&self.content.dialogue, &v);
                    talk.session
                        .say(&self.content.dialogue.db, &rules, &topic, &response)
                };
                match out {
                    Ok(out) => self.after(talk, view, &out, sheets),
                    Err(e) => warn!("services: refusal {response} not shown: {e}"),
                }
                self.service_events.push(event(
                    ctx,
                    ServiceEventKind::ServiceRefusedLine { topic, response },
                ));
            }
            Err(e) => warn!("services: {kind:?} with {:?}: {e}", ctx.provider),
        }
    }

    /// `Stage`, `Unstage`, `SetBalance`, `Accept` and `Cancel` on the player's session with `talk`'s
    /// speaker: every change is re-quoted (`TradeQuoted`), refusals reported (`TradeRefused`),
    /// an accepted deal settled into the world and closed (`TradeDone`).
    fn trade_request(
        &mut self,
        talk: &mut Talk,
        ctx: &ServiceCtx,
        id: SessionId,
        request: &ServiceRequest,
        sheets: &dyn MwSheets,
    ) {
        if !self
            .sessions
            .get(id)
            .is_some_and(|s| s.player == ctx.player && s.provider == ctx.provider)
        {
            warn!(
                "services: {request:?}: no session {id:?} of {:?} with {:?}",
                ctx.player, ctx.provider
            );
            return;
        }
        if matches!(request, ServiceRequest::Cancel { .. }) {
            self.sessions.close(id);
            return;
        }
        let config = trade_config();
        let (result, kind) = {
            let v = View {
                c: &self.content,
                s: &self.state,
                sheets,
            };
            let rules = MwServiceRules::new(
                &self.content.records,
                &self.content.gmst,
                &self.content.bound,
                &v,
            );
            let env = TradeEnv {
                ctx,
                rules: &rules,
                holdings: &v,
                config: &config,
            };
            let money = MerchantMoney::of(rules.purse_base(ctx), &self.services, ctx.provider);
            let session = self
                .sessions
                .get_mut(id, ctx.player)
                .expect("checked above");
            let result: Result<Option<Settlement>, TradeRefusal> = match request {
                ServiceRequest::Stage {
                    side,
                    source,
                    stack,
                    count,
                    ..
                } => session
                    .stage(&env, *side, *source, *stack, *count)
                    .map(|_| None),
                ServiceRequest::Unstage { line, count, .. } => {
                    let line = usize::try_from(*line).unwrap_or(usize::MAX);
                    session.unstage(&env, line, *count).map(|_| None)
                }
                ServiceRequest::SetBalance { balance, .. } => {
                    session.set_balance(&config, *balance).map(|_| None)
                }
                ServiceRequest::Accept { .. } => {
                    let roll = haggle_roll(ctx.provider, id, session.attempts, ctx.tick);
                    session.accept(&env, money, roll).map(Some)
                }
                _ => unreachable!("routed by service_request"),
            };
            let kind = match &result {
                Ok(None) => ServiceEventKind::TradeQuoted {
                    session: id,
                    lines: session.lines.clone(),
                    merchant_offer: session.merchant_offer,
                    balance: session.balance,
                },
                Ok(Some(_)) => ServiceEventKind::TradeDone {
                    session: id,
                    balance: session.balance,
                },
                Err(reason) => ServiceEventKind::TradeRefused {
                    session: id,
                    reason: reason.clone(),
                },
            };
            (result, kind)
        };
        self.service_events.push(event(ctx, kind));
        match result {
            Ok(Some(settlement)) => {
                self.sessions.close(id);
                self.settle(talk, ctx, settlement, sheets);
            }
            Ok(None) => {}
            Err(TradeRefusal::HaggleFailed { effects }) => {
                self.apply_service_effects(talk, ctx, effects, sheets);
            }
            Err(TradeRefusal::Vetoed(veto)) => {
                self.sessions.close(id);
                self.apply_service_effects(talk, ctx, veto.effects, sheets);
            }
            Err(e) => debug!("services: {request:?} refused: {e}"),
        }
    }

    /// An accepted deal, in the settlement's order: item and gold commands, the purse, effects.
    fn settle(
        &mut self,
        talk: &mut Talk,
        ctx: &ServiceCtx,
        settlement: Settlement,
        sheets: &dyn MwSheets,
    ) {
        for command in settlement.commands {
            if let Err(e) = self.apply_item_command(command.clone()) {
                warn!("services: settling {command:?}: {e}");
            }
        }
        self.services
            .apply_delta(ctx.provider, settlement.purse_delta);
        self.apply_service_effects(talk, ctx, settlement.effects, sheets);
    }

    /// A list purchase that passed its checks: its effects in order, then `EntryBought` and the
    /// re-listed rows (a sheet change lands on the next stat tick: [`relist_services`]).
    fn buy_listed(
        &mut self,
        talk: &mut Talk,
        ctx: &ServiceCtx,
        kind: &ServiceKind,
        entry: &EntryId,
        bought: Purchase,
        sheets: &dyn MwSheets,
    ) {
        info!(
            "services: {kind:?} {entry:?} bought from {:?} for {}",
            ctx.provider, bought.price
        );
        self.apply_service_effects(talk, ctx, bought.effects, sheets);
        let kind = ServiceEventKind::EntryBought {
            kind: kind.clone(),
            entry: entry.clone(),
            price: bought.price,
        };
        self.service_events.push(event(ctx, kind));
        self.relist(sheets, ctx.tick);
    }

    /// `advanceTime(hours)` on the `MWScript` calendar globals the game declares (`GameHour`,
    /// `DaysPassed`, `Day`, `Month`, `Year`; [`Rpg::game_time`] reads the first two), and the
    /// hours queued for the sky clock ([`advance_sky`]). Single player: time passes.
    /// TODO(co-op, services.md "Co-op"): a party rest request, the time skipped when the vote
    /// fails or is pending.
    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        reason = "calendar globals are small whole numbers stored as f32"
    )]
    fn pass_time(&mut self, hours: f32) {
        let names = CALENDAR_GLOBALS.map(id);
        let read = |rpg: &Rpg, name: &RecordId| {
            View {
                c: &rpg.content,
                s: &rpg.state,
                sheets: &NoSheets,
            }
            .global_value(name)
            .map(mw_rpg::GlobalNumber::as_f32)
        };
        let [hour, days, day, month, year] = names.each_ref().map(|n| read(self, n));
        let before = Calendar {
            hour: hour.unwrap_or(0.0),
            days_passed: days.unwrap_or(0.0) as i32,
            day: day.unwrap_or(1.0) as i32,
            month: month.unwrap_or(0.0) as i32,
            year: year.unwrap_or(0.0) as i32,
        };
        let after = advance_calendar(before, hours);
        let values = [
            after.hour,
            after.days_passed as f32,
            after.day as f32,
            after.month as f32,
            after.year as f32,
        ];
        for ((name, value), declared) in names
            .into_iter()
            .zip(values)
            .zip([hour, days, day, month, year])
        {
            if declared.is_some() {
                self.state.globals.insert(name, value);
            } else {
                debug!("services: no {name} global to advance");
            }
        }
        self.sky_hours += hours;
        info!("services: {hours} h passed ({before:?} -> {after:?})");
    }

    /// An item command on the host's inventories (`MoveStack` between two of them).
    fn apply_item_command(&mut self, command: RpgCommand) -> Result<Vec<RpgEvent>, String> {
        let holder = command.holder().ok_or("no holder")?;
        if let Some(to) = command.target() {
            if let Holder::Actor(key) = to {
                self.npc_store(key);
            }
            // Take the source out so both sides can be borrowed; put it back whatever happens.
            let mut source = self.take_character(holder).ok_or("no source")?;
            let result = self
                .character_mut(to)
                .ok_or_else(|| "no target".to_owned())
                .and_then(|target| {
                    CharacterRpg::apply_move(&mut source, target, command)
                        .map_err(|e| e.to_string())
                });
            self.put_character(holder, source);
            return result;
        }
        if let Holder::Actor(key) = holder {
            self.npc_store(key);
        }
        self.character_mut(holder)
            .ok_or("no holder")?
            .apply(command)
            .map_err(|e| e.to_string())
    }

    fn character_mut(&mut self, holder: Holder) -> Option<&mut CharacterRpg> {
        match holder {
            Holder::Player(PlayerId::HOST) => Some(&mut self.state.pc),
            Holder::Player(_) => None,
            Holder::Actor(key) => self.state.inventories.get_mut(&key),
        }
    }

    fn take_character(&mut self, holder: Holder) -> Option<CharacterRpg> {
        match holder {
            Holder::Player(PlayerId::HOST) => Some(std::mem::take(&mut self.state.pc)),
            Holder::Player(_) => None,
            Holder::Actor(key) => self.state.inventories.remove(&key),
        }
    }

    fn put_character(&mut self, holder: Holder, character: CharacterRpg) {
        match holder {
            Holder::Player(_) => self.state.pc = character,
            Holder::Actor(key) => {
                self.state.inventories.insert(key, character);
            }
        }
    }

    /// A deal's or a list purchase's effects, in order. `Pay`: the currency leaves the player
    /// ([`payment_command`]) into the provider's pool. `Disposition`:
    /// `applyBarterDispositionChange` (module docs). `UseSkill` (a won haggle's Mercantile use,
    /// [`mw_rpg::mercantile_use`]) and `RaiseStat` (training: `RaiseSkill` from
    /// `IncreaseSource::Training`): stat ops on the player's sheet, the stat looked up by key,
    /// sent with the dialogue's other stat commands. `PassTime`: the player's rest
    /// (`StatOp::Rest`) and the clocks ([`Rpg::pass_time`]). Teleport, grants and attrs are
    /// A4-A6.
    fn apply_service_effects(
        &mut self,
        talk: &mut Talk,
        ctx: &ServiceCtx,
        effects: Vec<ServiceEffect>,
        sheets: &dyn MwSheets,
    ) {
        for effect in effects {
            match effect {
                ServiceEffect::Disposition { delta } => {
                    let Some(actor) = self
                        .content
                        .actors
                        .get(&ctx.provider)
                        .filter(|a| a.facts.npc)
                    else {
                        continue;
                    };
                    let record_base = actor.base_disposition;
                    let base = record_base + self.state.npcs.modifier(ctx.provider);
                    talk.persuasion.refresh(base);
                    talk.persuasion.current += delta;
                    self.state
                        .npcs
                        .set(ctx.provider, record_base, talk.persuasion.current);
                    debug!(
                        "services: {:?}'s disposition {delta:+} (temporary)",
                        ctx.provider
                    );
                }
                ServiceEffect::UseSkill {
                    stat,
                    use_kind,
                    scale,
                } => {
                    let skill = sheets.table().and_then(|t| t.table.find(stat.as_str()));
                    let (Some(skill), Some(pk)) = (skill, player_key(ctx.player)) else {
                        warn!("services: no sheet stat {stat:?} for {:?}", ctx.player);
                        continue;
                    };
                    let ops = vec![StatOp::UseSkill {
                        skill,
                        use_kind,
                        scale,
                    }];
                    self.stat_commands.push(StatCommand::new(
                        StatOwner::Player(pk),
                        CauseKind::Dialogue,
                        ops,
                    ));
                }
                ServiceEffect::Pay { amount } => {
                    let pay = payment_command(ctx.player, &trade_config().currency, amount);
                    if let Some(Err(e)) = pay.map(|c| self.apply_item_command(c)) {
                        warn!("services: paying {amount}: {e}");
                        continue;
                    }
                    self.services.apply_delta(ctx.provider, amount);
                }
                ServiceEffect::RaiseStat { stat } => {
                    let skill = sheets.table().and_then(|t| t.table.find(stat.as_str()));
                    let (Some(skill), Some(pk)) = (skill, player_key(ctx.player)) else {
                        warn!("services: no sheet stat {stat:?} for {:?}", ctx.player);
                        continue;
                    };
                    let ops = vec![StatOp::RaiseSkill {
                        skill,
                        source: IncreaseSource::Training,
                    }];
                    self.stat_commands.push(StatCommand::new(
                        StatOwner::Player(pk),
                        CauseKind::Dialogue,
                        ops,
                    ));
                }
                ServiceEffect::PassTime { hours, sleep } => {
                    let hours = f32::from(hours);
                    if let Some(pk) = player_key(ctx.player) {
                        let ops = vec![StatOp::Rest { hours, sleep }];
                        self.stat_commands.push(StatCommand::new(
                            StatOwner::Player(pk),
                            CauseKind::Rest,
                            ops,
                        ));
                    }
                    debug!("services: only the player rests (OpenMW rests every actor in range)");
                    self.pass_time(hours);
                }
                other => debug!("services: effect {other:?} not applied yet (A4-A6)"),
            }
        }
    }
}

fn event(ctx: &ServiceCtx, kind: ServiceEventKind) -> ServiceEvent {
    ServiceEvent {
        tick: ctx.tick,
        player: ctx.player,
        provider: ctx.provider,
        kind,
    }
}

/// Open lists re-listed when their rows changed ([`Rpg::relist`]; checked through `&Rpg`
/// first, so `Rpg` isn't marked changed every frame).
#[allow(clippy::needless_pass_by_value)] // system parameters
pub(super) fn relist_services(
    mut rpg: ResMut<Rpg>,
    sheets: DialogueSheets,
    tick: Option<Res<StatTick>>,
) {
    if rpg.lists.is_empty() {
        return;
    }
    let tick = tick.map_or(0, |t| u64::from(t.0));
    if !rpg.stale_lists(&sheets, tick).is_empty() {
        rpg.relist(&sheets, tick);
    }
}

/// Hours services passed move the sky clock (`SkyTime`, what the sky, held lights, Lua's
/// `core.game_time()` and saves read): `days += hours / hours_per_day`. Without a sky clock
/// (headless, interiors before the exterior plugin) only the `MWScript` calendar moved.
#[allow(clippy::needless_pass_by_value)] // system parameters
pub(super) fn advance_sky(
    mut rpg: ResMut<Rpg>,
    sky: Option<ResMut<SkyTime>>,
    config: Option<Res<SkyConfig>>,
) {
    if rpg.sky_hours <= 0.0 {
        return;
    }
    let hours = std::mem::take(&mut rpg.sky_hours);
    let Some(mut sky) = sky else {
        debug!("services: no sky clock; {hours} h passed on the MWScript calendar only");
        return;
    };
    let per_day = config.map_or(24.0, |c| c.0.calendar.hours_per_day);
    sky.0.days += f64::from(hours) / f64::from(per_day);
}

/// Queued service events go out as [`ServiceEventMessage`]s (checked before taking, so `Rpg`
/// isn't marked changed every frame).
pub(super) fn send_service_events(
    mut rpg: ResMut<Rpg>,
    mut messages: MessageWriter<ServiceEventMessage>,
) {
    if rpg.service_events.is_empty() {
        return;
    }
    for e in rpg.take_service_events() {
        debug!("services: {e:?}");
        messages.write(ServiceEventMessage(e));
    }
}

#[cfg(test)]
pub(super) mod tests {
    //! The service host, headless: synthetic records, exact assertions. Prices without sheets
    //! are the items' base values (`MwServiceRules::quote` without barter terms).
    use std::sync::Arc;

    use mw_game_core::player::HOST_KEY;
    use mw_rpg::ATTR_RESTOCK;
    use mw_stats::{MwGmst, MwTable, SkillInfo, openmw_rules};
    use sf_rpg::service::{ServiceShape, Side};
    use sf_rpg::{AttrValue, Attrs, StackId};
    use sf_rpg_ui::ActionId;
    use sf_stats_bevy::sf_stats::{Inputs, Sheet, StatOp, StatOwner};
    use tes3::esp::{
        Dialogue, DialogueInfo, DialogueType2, Filter, FilterComparison, FilterFunction,
        FilterType, FilterValue, GameSetting, GameSettingValue, MiscItem, ServiceFlags,
    };

    use super::super::PERSUASION_ACTION;
    use super::*;

    /// The ESM skill index of Mercantile.
    const MERCANTILE: usize = 24;
    pub(crate) const MERCHANT: ActorKey = ActorKey(1);
    const FARGOTH: ActorKey = ActorKey(2);
    pub(crate) const PLAYER: Holder = Holder::Player(PlayerId::HOST);
    pub(crate) const SELLER: Holder = Holder::Actor(MERCHANT);

    fn misc(value: u32) -> MiscItem {
        let mut m = MiscItem::default();
        m.data.value = value;
        m
    }

    /// Records: `arrille` (barters misc items, 500 gold, a bottle and a restocking pile of 5
    /// candles), `fargoth` (no services), three misc items, `sBarter` = " Trade " (trimmed on
    /// use), and optionally a "Service Refusal" INFO.
    pub(crate) fn records(refusal: Option<DialogueInfo>) -> RecordDb {
        let mut db = RecordDb::new();
        mw_records::register_kinds(&mut db).unwrap();
        let src = db.add_source("test", &[]);
        let mut arrille = Npc {
            id: "arrille".into(),
            name: "Arrille".into(),
            ..Default::default()
        };
        arrille.ai_data.services = ServiceFlags::BARTERS_MISC_ITEMS;
        arrille.data.gold = 500;
        arrille.data.disposition = 50;
        arrille.inventory = vec![
            (1, String::from("misc_bottle").into()),
            (-5, String::from("misc_candle").into()),
        ];
        db.define(src, id("arrille"), arrille).unwrap();
        db.define(
            src,
            id("fargoth"),
            Npc {
                id: "fargoth".into(),
                name: "Fargoth".into(),
                ..Default::default()
            },
        )
        .unwrap();
        db.define(src, id("misc_bottle"), misc(10)).unwrap();
        db.define(src, id("misc_candle"), misc(3)).unwrap();
        db.define(src, id("misc_spoon"), misc(4)).unwrap();
        db.define(src, id(GOLD), misc(1)).unwrap();
        let label = GameSetting {
            id: "sBarter".into(),
            value: GameSettingValue::String(" Trade ".into()),
            ..Default::default()
        };
        db.define(src, id("sBarter"), label).unwrap();
        if let Some(info) = refusal {
            let dial = Dialogue {
                id: "Service Refusal".into(),
                dialogue_type: DialogueType2::Persuasion,
                ..Default::default()
            };
            db.define(src, mw_records::keys::info(&dial, &info), info)
                .unwrap();
            db.define(src, id("Service Refusal"), dial).unwrap();
        }
        db
    }

    /// A host over [`records`] with both NPCs registered and the player holding 100 gold and a
    /// spoon, talking to `speaker`.
    pub(crate) fn host(
        refusal: Option<DialogueInfo>,
        speaker: ActorKey,
        sheets: &dyn MwSheets,
    ) -> (Rpg, Talk, DialogueView) {
        let mut rpg = Rpg::new(Arc::new(records(refusal))).unwrap();
        assert!(rpg.add_actor(MERCHANT, &id("arrille"), "Balmora"));
        assert!(rpg.add_actor(FARGOTH, &id("fargoth"), "Seyda Neen"));
        rpg.state
            .pc
            .loadout
            .add(id(GOLD), 100, Attrs::new())
            .unwrap();
        rpg.state
            .pc
            .loadout
            .add(id("misc_spoon"), 1, Attrs::new())
            .unwrap();
        let (talk, view) = rpg.open(speaker, sheets).unwrap();
        (rpg, talk, view)
    }

    pub(crate) fn barter() -> DialogueRequest {
        DialogueRequest::Action {
            action: ActionId::new(MwService::Barter.id()),
            option: None,
        }
    }

    pub(crate) fn stack_of(rpg: &Rpg, holder: Holder, item: &str) -> StackId {
        let v = View {
            c: &rpg.content,
            s: &rpg.state,
            sheets: &NoSheets,
        };
        v.loadout(holder)
            .unwrap()
            .inventory()
            .stacks()
            .iter()
            .find(|s| s.item == id(item))
            .unwrap()
            .id
    }

    pub(crate) fn count(rpg: &Rpg, holder: Holder, item: &str) -> u64 {
        let v = View {
            c: &rpg.content,
            s: &rpg.state,
            sheets: &NoSheets,
        };
        v.loadout(holder).map_or(0, |l| l.count(&id(item)))
    }

    /// Presses Barter and returns the session it opened.
    fn open_barter(
        rpg: &mut Rpg,
        talk: &mut Talk,
        view: &mut DialogueView,
        sheets: &dyn MwSheets,
    ) -> SessionId {
        rpg.request_at(talk, view, &barter(), sheets, 3);
        let events = rpg.take_service_events();
        let opened = events.iter().find_map(|e| match &e.kind {
            ServiceEventKind::ServiceOpened { session, .. } => *session,
            _ => None,
        });
        opened.unwrap_or_else(|| panic!("no session: {events:?}"))
    }

    fn request(
        rpg: &mut Rpg,
        talk: &mut Talk,
        view: &mut DialogueView,
        r: &ServiceRequest,
        sheets: &dyn MwSheets,
    ) {
        rpg.service_request(talk, view, r, sheets, 4);
    }

    #[test]
    fn a_merchant_shows_barter_after_persuasion() {
        let (rpg, talk, view) = host(None, MERCHANT, &NoSheets);
        let ids: Vec<_> = view
            .actions()
            .iter()
            .map(|a| (a.id.clone(), a.label.clone()))
            .collect();
        assert_eq!(
            ids,
            [
                (ActionId::new(PERSUASION_ACTION), "Persuasion".into()),
                (ActionId::new("mw.barter"), "Trade".into())
            ]
        );
        assert!(view.actions()[1].is_live() && view.actions()[1].submenu.is_empty());
        assert_eq!(rpg.service_offers(&talk).len(), 1);
    }

    #[test]
    fn a_non_merchant_shows_no_service_button() {
        let (mut rpg, mut talk, mut view) = host(None, FARGOTH, &NoSheets);
        let ids: Vec<_> = view.actions().iter().map(|a| a.id.clone()).collect();
        assert_eq!(ids, [ActionId::new(PERSUASION_ACTION)]);
        // A forged Barter press for him opens nothing.
        rpg.request_at(&mut talk, &mut view, &barter(), &NoSheets, 3);
        assert_eq!(
            (rpg.take_service_events(), rpg.sessions.len()),
            (Vec::new(), 0)
        );
    }

    #[test]
    fn pressing_barter_opens_a_session_and_the_purse() {
        let (mut rpg, mut talk, mut view) = host(None, MERCHANT, &NoSheets);
        rpg.request_at(&mut talk, &mut view, &barter(), &NoSheets, 3);
        let ev = |kind| ServiceEvent {
            tick: 3,
            player: PlayerId::HOST,
            provider: MERCHANT,
            kind,
        };
        let session = SessionId(0);
        assert_eq!(
            rpg.take_service_events(),
            [
                ev(ServiceEventKind::PurseRestocked { gold: 500 }),
                ev(ServiceEventKind::ServiceOpened {
                    kind: MwService::Barter.kind(),
                    shape: ServiceShape::Trade,
                    session: Some(session)
                }),
            ]
        );
        let s = rpg.trade_session(session).unwrap();
        assert_eq!(
            (s.player, s.provider, s.lines.len()),
            (PlayerId::HOST, MERCHANT, 0)
        );
        assert_eq!(rpg.services.gold(MERCHANT), Some(500));
        // The stock was seeded from the record: the candles restock.
        assert_eq!(
            (
                count(&rpg, SELLER, "misc_bottle"),
                count(&rpg, SELLER, "misc_candle")
            ),
            (1, 5)
        );
        let candles = rpg.state.inventories[&MERCHANT]
            .loadout
            .inventory()
            .get(stack_of(&rpg, SELLER, "misc_candle"))
            .unwrap()
            .attrs
            .clone();
        assert_eq!(candles, Attrs::new().with(ATTR_RESTOCK, AttrValue::Int(1)));
        // Goodbye closes it.
        rpg.request(&mut talk, &mut view, &DialogueRequest::Goodbye, &NoSheets);
        assert!(rpg.trade_session(session).is_none());
    }

    #[test]
    fn staging_and_accepting_moves_items_and_gold() {
        let (mut rpg, mut talk, mut view) = host(None, MERCHANT, &NoSheets);
        let session = open_barter(&mut rpg, &mut talk, &mut view, &NoSheets);
        let bottle = stack_of(&rpg, SELLER, "misc_bottle");
        let candles = stack_of(&rpg, SELLER, "misc_candle");
        let spoon = stack_of(&rpg, PLAYER, "misc_spoon");
        let stage = |side, source, stack, count| ServiceRequest::Stage {
            session,
            side,
            source,
            stack,
            count,
        };
        for r in [
            stage(Side::Buy, SELLER, bottle, 1),
            stage(Side::Buy, SELLER, candles, 2),
            stage(Side::Sell, PLAYER, spoon, 1),
        ] {
            request(&mut rpg, &mut talk, &mut view, &r, &NoSheets);
        }
        // Base prices: bottle 10, two candles 6, the spoon 4: the player pays 12.
        let events = rpg.take_service_events();
        let Some(ServiceEventKind::TradeQuoted {
            lines,
            merchant_offer,
            balance,
            ..
        }) = events.last().map(|e| &e.kind)
        else {
            panic!("{events:?}")
        };
        assert_eq!(
            (
                lines.iter().map(|l| l.price).collect::<Vec<_>>(),
                *merchant_offer,
                *balance
            ),
            (vec![10, 6, 4], -12, -12)
        );
        // Staging moved nothing yet.
        assert_eq!(
            (
                count(&rpg, PLAYER, GOLD),
                count(&rpg, PLAYER, "misc_bottle")
            ),
            (100, 0)
        );

        request(
            &mut rpg,
            &mut talk,
            &mut view,
            &ServiceRequest::Accept { session },
            &NoSheets,
        );
        let done = ServiceEvent {
            tick: 4,
            player: PlayerId::HOST,
            provider: MERCHANT,
            kind: ServiceEventKind::TradeDone {
                session,
                balance: -12,
            },
        };
        assert_eq!(rpg.take_service_events(), [done]);
        assert!(
            rpg.trade_session(session).is_none(),
            "an accepted deal closes the session"
        );
        let have = |rpg: &Rpg, h, items: [&str; 4]| items.map(|i| count(rpg, h, i));
        let items = [GOLD, "misc_bottle", "misc_candle", "misc_spoon"];
        assert_eq!(have(&rpg, PLAYER, items), [88, 1, 2, 0]);
        // The candles restock (still 5); the merchant's money is its pool, not its inventory.
        assert_eq!(have(&rpg, SELLER, items), [0, 0, 5, 1]);
        assert_eq!(rpg.services.gold(MERCHANT), Some(512));
        // The bought candles are ordinary candles (one plain stack).
        let plain = rpg
            .state
            .pc
            .loadout
            .inventory()
            .get(stack_of(&rpg, PLAYER, "misc_candle"))
            .unwrap();
        assert_eq!(plain.attrs, Attrs::new());
        // Without barter terms there is no disposition effect.
        assert_eq!(rpg.state.npcs.modifier(MERCHANT), 0);
    }

    #[test]
    fn a_refused_accept_keeps_the_session() {
        let (mut rpg, mut talk, mut view) = host(None, MERCHANT, &NoSheets);
        let session = open_barter(&mut rpg, &mut talk, &mut view, &NoSheets);
        request(
            &mut rpg,
            &mut talk,
            &mut view,
            &ServiceRequest::Accept { session },
            &NoSheets,
        );
        let refused = ServiceEventKind::TradeRefused {
            session,
            reason: TradeRefusal::NothingStaged,
        };
        assert_eq!(
            rpg.take_service_events().pop().map(|e| e.kind),
            Some(refused)
        );
        let bottle = stack_of(&rpg, SELLER, "misc_bottle");
        let stage = ServiceRequest::Stage {
            session,
            side: Side::Buy,
            source: SELLER,
            stack: bottle,
            count: 1,
        };
        request(&mut rpg, &mut talk, &mut view, &stage, &NoSheets);
        // The player can't pay 10 with 5 gold.
        rpg.state.pc.loadout.remove(&id(GOLD), 95).unwrap();
        request(
            &mut rpg,
            &mut talk,
            &mut view,
            &ServiceRequest::Accept { session },
            &NoSheets,
        );
        let refused = ServiceEventKind::TradeRefused {
            session,
            reason: TradeRefusal::PlayerCantAfford { have: 5, want: 10 },
        };
        assert_eq!(
            rpg.take_service_events().pop().map(|e| e.kind),
            Some(refused)
        );
        assert_eq!(
            (
                count(&rpg, PLAYER, GOLD),
                count(&rpg, SELLER, "misc_bottle")
            ),
            (5, 1)
        );
        request(
            &mut rpg,
            &mut talk,
            &mut view,
            &ServiceRequest::Cancel { session },
            &NoSheets,
        );
        assert!(rpg.trade_session(session).is_none());
    }

    #[test]
    fn a_service_refusal_is_a_dialogue_line() {
        // Barter (choice 1) refused below disposition 100: always here.
        let mut info = DialogueInfo {
            id: "sr1".into(),
            text: "I don't trade with outlanders.".into(),
            ..Default::default()
        };
        info.data.speaker_rank = -1;
        info.data.player_rank = -1;
        info.data.disposition = 100;
        info.filters = vec![Filter {
            filter_type: FilterType::Function,
            function: FilterFunction::Choice,
            comparison: FilterComparison::Equal,
            value: FilterValue::Integer(1),
            ..Default::default()
        }];
        let (mut rpg, mut talk, mut view) = host(Some(info), MERCHANT, &NoSheets);
        let lines = talk.session.history().len();
        rpg.request_at(&mut talk, &mut view, &barter(), &NoSheets, 3);
        assert_eq!(talk.session.history().len(), lines + 1);
        assert_eq!(
            talk.session.history().last().unwrap().text,
            "I don't trade with outlanders."
        );
        let events = rpg.take_service_events();
        assert!(
            matches!(
                &events[..],
                [ServiceEvent {
                    kind: ServiceEventKind::ServiceRefusedLine { .. },
                    ..
                }]
            ),
            "{events:?}"
        );
        assert_eq!(rpg.sessions.len(), 0);
        assert_eq!(
            rpg.services.gold(MERCHANT),
            None,
            "a refusal restocks nothing"
        );
    }

    /// The host player's and the merchant's sheets: Personality 50 for both.
    struct TestSheets {
        table: MwTable,
        player: Sheet,
        npc: Sheet,
    }

    impl MwSheets for TestSheets {
        fn table(&self) -> Option<&MwTable> {
            Some(&self.table)
        }

        fn player(&self, player: PlayerId) -> Option<&Sheet> {
            (player == PlayerId::HOST).then_some(&self.player)
        }

        fn actor(&self, actor: ActorKey) -> Option<&Sheet> {
            (actor == MERCHANT).then_some(&self.npc)
        }
    }

    fn test_sheets() -> TestSheets {
        test_sheets_with(0.0)
    }

    /// [`test_sheets`] with the player's base Mercantile at `mercantile`.
    fn test_sheets_with(mercantile: f32) -> TestSheets {
        let table = MwTable::with_skills(std::array::from_fn(|_| SkillInfo {
            governing: 0,
            specialization: 0,
            use_values: [1.0, 0.0, 0.0, 0.0],
        }))
        .unwrap();
        let rules = openmw_rules(&table, &MwGmst::vanilla());
        let sheet = |owner| {
            let inputs = Inputs {
                is_player: matches!(owner, StatOwner::Player(_)),
                ..Inputs::default()
            };
            let mut ops = vec![StatOp::SetBase {
                stat: table.idx.attributes[6],
                value: 50.0,
            }];
            if matches!(owner, StatOwner::Player(_)) {
                ops.push(StatOp::SetBase {
                    stat: table.idx.skills[MERCANTILE],
                    value: mercantile,
                });
            }
            mw_stats::new_sheet(&table, &rules, owner, &inputs, &ops).0
        };
        let (player, npc) = (
            sheet(StatOwner::Player(HOST_KEY)),
            sheet(StatOwner::Actor(MERCHANT)),
        );
        TestSheets { table, player, npc }
    }

    #[test]
    fn a_plain_offer_deal_raises_disposition_by_one() {
        let sheets = test_sheets();
        let (mut rpg, mut talk, mut view) = host(None, MERCHANT, &sheets);
        let disposition = |rpg: &Rpg, talk: &Talk| {
            MwView::disposition(
                &View {
                    c: &rpg.content,
                    s: &rpg.state,
                    sheets: &sheets,
                },
                talk.session.ctx(),
            )
        };
        let before = disposition(&rpg, &talk);
        assert!((0..100).contains(&before), "room for +1: {before}");
        let session = open_barter(&mut rpg, &mut talk, &mut view, &sheets);
        let bottle = stack_of(&rpg, SELLER, "misc_bottle");
        let stage = ServiceRequest::Stage {
            session,
            side: Side::Buy,
            source: SELLER,
            stack: bottle,
            count: 1,
        };
        request(&mut rpg, &mut talk, &mut view, &stage, &sheets);
        let price = rpg.trade_session(session).unwrap().balance;
        assert!(price < 0, "the player pays: {price}");
        request(
            &mut rpg,
            &mut talk,
            &mut view,
            &ServiceRequest::Accept { session },
            &sheets,
        );
        assert!(
            rpg.trade_session(session).is_none(),
            "{:?}",
            rpg.take_service_events()
        );
        assert_eq!(count(&rpg, PLAYER, "misc_bottle"), 1);
        assert_eq!(count(&rpg, PLAYER, GOLD), 100 - price.unsigned_abs());
        // iBarterSuccessDisposition (1) on the base at once...
        assert_eq!(
            (rpg.state.npcs.modifier(MERCHANT), disposition(&rpg, &talk)),
            (1, before + 1)
        );
        // ...and temporary: goodbye drops it.
        rpg.request(&mut talk, &mut view, &DialogueRequest::Goodbye, &sheets);
        assert_eq!(rpg.state.npcs.modifier(MERCHANT), 0);
    }

    #[test]
    fn a_won_haggle_raises_the_players_mercantile_progress() {
        // Mercantile 75 (under 100, where skills stop progressing) makes the roll likely.
        let sheets = test_sheets_with(75.0);
        let (mut rpg, mut talk, mut view) = host(None, MERCHANT, &sheets);
        let session = open_barter(&mut rpg, &mut talk, &mut view, &sheets);
        let bottle = stack_of(&rpg, SELLER, "misc_bottle");
        let stage = ServiceRequest::Stage {
            session,
            side: Side::Buy,
            source: SELLER,
            stack: bottle,
            count: 1,
        };
        request(&mut rpg, &mut talk, &mut view, &stage, &sheets);
        let price = rpg.trade_session(session).unwrap().balance;
        assert!(price < -1, "the player pays: {price}");
        // One gold off: a haggle (skillGain = floor(100 / |price|) > 0 for any price up to 100).
        let ask = ServiceRequest::SetBalance {
            session,
            balance: price + 1,
        };
        request(&mut rpg, &mut talk, &mut view, &ask, &sheets);
        let before = rpg.stat_commands.len();
        let accept = ServiceRequest::Accept { session };
        for _ in 0..16 {
            request(&mut rpg, &mut talk, &mut view, &accept, &sheets);
            if rpg.trade_session(session).is_none() {
                break;
            }
            assert_eq!(
                rpg.stat_commands.len(),
                before,
                "a lost haggle uses no skill"
            );
        }
        assert!(
            rpg.trade_session(session).is_none(),
            "no roll won in 16 tries"
        );
        let used: Vec<_> = rpg.stat_commands.drain(before..).collect();
        assert_eq!(used.len(), 1, "one Mercantile use: {used:?}");
        let t = &sheets.table;
        let gain = f32::from(u8::try_from(100 / price.unsigned_abs()).unwrap());
        assert_eq!(
            (used[0].owner, used[0].cause, used[0].ops.as_slice()),
            (
                StatOwner::Player(HOST_KEY),
                CauseKind::Dialogue,
                [StatOp::UseSkill {
                    skill: t.idx.skills[MERCANTILE],
                    use_kind: 0,
                    scale: gain,
                }]
                .as_slice()
            )
        );
        // Applied to the player's sheet, the progress rises.
        let rules = openmw_rules(t, &MwGmst::vanilla());
        let mut sheet = sheets.player.clone();
        let progress = t.idx.skill_progress[MERCANTILE];
        let inputs = Inputs {
            is_player: true,
            ..Inputs::default()
        };
        let ctx = sf_stats_bevy::sf_stats::ApplyCtx {
            owner: StatOwner::Player(HOST_KEY),
            cause: sf_stats_bevy::sf_stats::Cause {
                kind: CauseKind::Dialogue,
                source: None,
                tick: 0,
            },
            inputs: &inputs,
        };
        let start = sheet.view(&t.table).base(progress);
        sf_stats_bevy::sf_stats::apply(
            &t.table,
            &rules,
            &mut sheet,
            &used[0].ops,
            &ctx,
            &mut Vec::new(),
        );
        let end = sheet.view(&t.table).base(progress);
        assert!(end > start, "Mercantile progress {start} -> {end}");
    }
}
