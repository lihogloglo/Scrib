# services-adapter history

Dated "As built" blocks moved verbatim from [services-adapter.md](../services-adapter.md) (2026-11-09, doc budget).

## As built (A1)

`mw_rpg::services` (`crates/mw_rpg/src/services.rs`, tests `tests_services.rs`, 25 tests): `PartyTerms`, `BarterTerms`,
`barter_offer`, `effective_value`, and the six rules as pure `fn(&In) -> Out` (`barter_price`, `barter_haggle`,
`training_price`, `travel_price`, `spell_price`, `repair_price`; ids in `RULE_IDS`); each `In` holds its GMSTs and has
`In::new(.., &MwGmst)`. The 12 GMSTs above are in `mw_stats::MwGmst` (the fatigue ones already were). Changes from the
design: travel takes `player_pos`/`dest_pos` (fare distance in `f64` as `std::pow` promotes, hours from the `f32` 2D
length); the training skill and attribute are `f32` (OpenMW compares floats); `HaggleOut::disposition_delta` is 0 for
creatures. Not wired (A2). Real-install GMST check: `mw_stats` test `real_install` (ignored) prints every GMST against
`vanilla()`.

**As built (A2a)**, written 2026-11-08, compiled + unit tests + pedantic clippy 2026-11-09; UNTESTED in game (owner test pass 2026-11-10). `mw_rpg::barter`
(`barter.rs`, tests `tests_barter.rs`, 12 tests): `MwServiceRules` over a host `MwTradeHost` (terms, owned stock, the
"Service Refusal" filter); `AllItems` excludes `MagicItems` (no Barter button for it); bound items from `sMagicBound*`.


## As built (TR1)

Barter in game, host side (2026-11-10, headless tests; not run in game). `mw_rpg_bevy` `dialogue/services.rs`:
- `MwTradeHost` on the dialogue `View`: `Holdings` (player = `RpgState::pc`, NPCs = `RpgState::inventories`);
  `barter_terms` from both sheets (`persuader_stats`: modified Mercantile, Luck, Personality, fatigue) and the derived
  disposition; `service_refusal` = new `MwDialogueRules::service_refusal` (first "Service Refusal" INFO, inverted
  disposition, a 0 always passes). `owned_stock` stays the default (none): world ownership does not exist yet.
- `mw_rpg` filter fix: a `Choice` condition fails outside a choice (`Filter::testSelectStruct`, `mChoice == -1`), for
  every topic, as `OpenMW`. `sf_rpg` gained `DialogueSession::say` (show a host-chosen response: the refusal line).
- Buttons: Persuasion first, then `MwServiceRules::offers` in `MwService::BUTTON_ORDER` (`updateTopicsPane` order; the
  TR1 brief said services first, OpenMW says Persuasion first), labels from the GMSTs. Every offer gets a button; list
  and custom services open (`ServiceOpened`) but have no entries until A3-A6.
- Routing: `DialogueRequest::service_open` → `Rpg::service_request` → `sf_rpg::service::open` (claim, refusal, purse
  restock, session). Only while talking. A refusal is shown via `say` and its result script runs; no session opens.
- Sessions in `Rpg::sessions` (not saved), purses in `Rpg::services` (TODO: `sf/services` save section in
  `mw_saves_bevy`). Stage/Unstage/SetBalance re-quote (`TradeQuoted`); Accept rolls `haggle_roll(provider, session,
  attempts, tick)`; a settlement applies `MoveStack`/`AddItem`/`RemoveItem` to the host's inventories, the purse delta,
  then effects, and closes the session (`onOfferButtonClicked` leaves barter mode). Goodbye closes the player's sessions.
- `Disposition` effects follow `applyBarterDispositionChange`: on the base at once through the conversation's
  `PersuasionSession`, temporary (OpenMW's `barter disposition change is permanent` defaults off), so goodbye drops it.
  A failed haggle applies `iBarterFailDisposition`. `UseSkill` (A2b) and list effects (A3-A6) are logged, not applied.
- Merchant stock: `seed_store` fills an NPC's store from its record on first access (barter open, result-script item
  changes): levelled lists skipped, negative counts restocking (`mw.restock`), positive-count armor and clothing worn
  (first per slot, so the merchant does not sell what it wears; OpenMW's `autoEquip` by value not modelled).
- Clock: `Rpg::game_time` reads the `DaysPassed` and `GameHour` globals (OpenMW `getTimeStamp`); nothing advances them
  yet, so a purse restocks only on first open.
- Events out: `Rpg::take_service_events`, sent each frame as `ServiceEventMessage` (Bevy message) for TR2.

## As built (TR2)

2026-11-10, headless tests only (not run in game; no Morrowind install on the build machine).
- `mw_rpg_bevy` `dialogue/trade_screen.rs` (registered from `inventory_ui::add`, which holds the item records and
  slots): `ServiceOpened { shape: Trade, session: Some(id) }` for the conversation's speaker pushes the `trade` screen
  (Menu layer) over `dialogue`, only when the dialogue is the top menu. `TradeView` panes: the speaker's store
  (`container_view`) and the player's loadout (`player_view`); title the actor's name.
- `Rpg::trade_quotes(session, sheets, tick)` -> `TradeQuotes { money, provider, player }`: `MerchantMoney::of` the
  purse rule and `pane_prices` (buy side over the store, sell side over the player) through the private `View`.
- `host_trade`: the screen's `ServiceScreenEvent` requests (only from roots with `OpenTrade`) go to
  `Rpg::service_request` with the active `Talk` and the dialogue's view (refusal lines land there), at `StatTick`.
  Ordered after `host_dialogue`, before `send_service_events`, so a request's events reach the view the same frame.
- `trade_events` (after `send_service_events`): the provider's events feed `TradeView::apply_event`; `TradeDone`
  closes the screen (OpenMW closes the barter window on an accepted offer; the host already closed the session).
  Whenever `Rpg` changed, both panes re-sync and are re-priced (so after staging, a deal, a script).
- `close_trade`: no `dialogue` on the stack (goodbye drops the sessions) or a closed view (Cancel, a veto) closes
  `trade`; a root off the stack despawns. `service` (list screen) id registered, not opened yet (A3-A6).
- Gaps: `ServiceLabels` are the screens' English fallbacks (GMST strings such as `sBarter*` not mapped yet); the
  "deal done" toast dies with the screen; re-pricing on every `Rpg` change quotes every stack (cheap at merchant
  sizes, not measured).

## As built (A2b)

2026-11-10, unit + headless tests (not run in game).
- `mw_rpg::mercantile_use(&HaggleOut) -> Option<ServiceEffect>`: `UseSkill { stat: "mercantile", use_kind: 0
  (Mercantile_Success), scale: mercantile_scale }`, only for an accepted haggle with a scale above 0. Ported from
  `apps/openmw/mwgui/tradewindow.cpp` `haggle` 99-111 (OpenMW `ca5e2576`): `skillUsageSucceeded` runs only after a
  won roll, with `floor(100 x haggled difference / larger price)`; a deal at or under the merchant's offer returns
  before the roll (no use), a lost roll and a creature merchant use nothing. A floored gain of 0 is skipped (OpenMW
  calls with 0, which progresses nothing). `MwServiceRules::haggle` puts it before the disposition change (OpenMW's
  order: inside `haggle`, then `onOfferButtonClicked` 447-457).
- Host (`dialogue/services.rs` `apply_service_effects`, now taking the sheets): `UseSkill` looks the stat key up in
  the sheet table and pushes `StatCommand(Player, CauseKind::Dialogue, [StatOp::UseSkill])`, as persuasion's
  Speechcraft use; the next fixed tick applies it. Skills at base 100 do not progress (`OpenMwSkillUse`).
- Tests: `tests_barter::a_won_haggle_uses_mercantile_by_the_haggled_percentage` (exact effects, buy and sell, lost,
  at-offer, zero gain, creature); `a_won_haggle_raises_the_players_mercantile_progress` (headless: one gold off,
  accept until the roll wins, one command, progress rises when applied).


## Barter disposition (A2a design)

Moved verbatim from the "Formulas" section (2026-11-10, doc budget).

The disposition change is applied on **every** accept attempt that reaches the haggle step, including the
no-haggle path (OpenMW `onOfferButtonClicked` 447-457 applies it after `haggle` returns, and `haggle` accepts at once
when the player asks no more than offered). Our `TradeSession` skips `haggle` when `balance == merchant_offer`, so
the adapter's settlement adds `Disposition { delta: iBarterSuccessDisposition }` for NPC merchants on every accepted
deal, and a failed haggle emits `iBarterFailDisposition`. Temporary vs permanent: OpenMW's dialogue manager applies
it as the conversation's temporary change unless `barter disposition change is permanent` ([settings
doc](https://openmw.readthedocs.io/en/latest/reference/modding/settings/game.html)); the disposition port owns
where it lands.

## As built (A3)

2026-11-10, unit + headless tests (not run in game; PUI-SV4 awaiting owner). OpenMW `ca5e2576`.
- `mw_rpg::training` (`training.rs`, tests `tests_training.rs`): `trainer_skills` (`setPtr` 75-104: three best in
  record order, a later skill replaces the lowest only when strictly greater, stable sort), `training_entries` (rows:
  id = skill key, label = `sSkill*` GMST, `EntryDetail::Stat { current, next }`, A1's `training_price`, refused rows
  `EntryState::Refused(sServiceTrainingWords | sNotifyMessage17)`), `training_purchase` (the same checks now; effects
  `Pay`, `RaiseStat`, `PassTime { 2, awake }`; `Unknown` for a skill not in the three), `TrainingOptions { base_skill:
  true }`, `advance_calendar` (`datetimemanager.cpp` `advanceTime`/`setHour`/`setDay`: hour wraps, month lengths,
  year, `DaysPassed`). Without barter terms the price is the base (as `quote`).
- `MwTradeHost::training_sheets` (default `None`: no stats, no rows); `MwServiceRules::entries`/`buy_entry` dispatch
  `mw.training` when the provider's flags still offer it; `MwServiceRules.training` options.
- Host (`mw_rpg_bevy` `dialogue/services.rs`): opening a `List` offer stores an `OpenList` (player, provider, kind,
  rows) and sends `ServiceOpened` then `EntriesListed`; `BuyEntry` needs that list open, then `purchase` (gold check)
  and the effects in order: `Pay` (`payment_command` out of the loadout, `+price` on the provider's pool), `RaiseStat`
  (`StatCommand(Player, Dialogue, [RaiseSkill { Training }])`: `OpenMwSkillUse::raise` adds the class level-up
  progress and attribute tallies, keeps the skill's progress), `PassTime` (`StatCommand(Player, Rest, [Rest { 2,
  awake }])` and `Rpg::pass_time`: the declared `GameHour`/`DaysPassed`/`Day`/`Month`/`Year` globals advanced, the
  hours queued for `advance_sky`, which adds `hours / hours_per_day` to `sf_sky_bevy::SkyTime`), then `EntryBought`.
  `relist_services` re-sends `EntriesListed` whenever a list's rows change (the raise lands on the next stat tick).
  Back closes the host list (`close_service_list`), goodbye all of a player's lists (`end_talk`), a load all.
- Screen glue (`dialogue/service_list_screen.rs`, after `trade_screen.rs`): `ServiceListScreen` over the dialogue on a
  `List` `ServiceOpened` (rows from the host's `OpenList`, title `sServiceTrainingTitle`, gold from the loadout),
  `confirm_time_passing` (skill and destination rows ask first), label keys' GMST texts into `ServiceLabels`, the
  view fed from `EntriesListed`/`EntryBought`/`EntryRefused`, gold synced on `Rpg` changes, `close_list` follows the
  dialogue and the view. `host_trade` now ignores list screens' events.
- What moves the clocks: the MWScript calendar globals (so `Rpg::game_time`, the effects clock and services' purse
  restock) and the sky clock (`SkyTime`: sky, held lights, Lua `core.game_time()`, saves). No NPC schedule reads a
  clock yet (ROADMAP 1.17 unchanged). Only the player rests (OpenMW rests every actor in range: NPC fatigue gap).
- Gaps: no fade or progress bar (OpenMW: 1/60 s per hour under a 0.2 s fade; the screen's `progress` is unused);
  co-op `PassTime` (party rest vote) not built (single player only); refusal GMST texts are the content's (the
  Morrowind.esm strings are not checked here, no install).
- Tests: `tests_training` (7: top three with ties, rows/prices/reasons, Mercantile difference and no terms, skill 0
  costs 1, base vs modified option, purchase effects and refusals, calendar); `service_list_screen` (7 headless: the
  Training button; the list's top three at A1 prices with reasons and labels; a confirmed purchase pays, fills the
  pool, raises Long Blade 20 → 21 with one major level-up point, moves globals and sky by 2 h, re-lists at 21; both
  refusals and a non-offered skill; short gold; Back to the dialogue; goodbye).
