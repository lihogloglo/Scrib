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
