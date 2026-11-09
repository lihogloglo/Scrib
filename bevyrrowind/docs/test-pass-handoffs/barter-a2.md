# barter-a2 (services adapter A2a: barter rules without Bevy, 2026-11-08)
Scope: NEW bevyrrowind/crates/mw_rpg/src/barter.rs + tests_barter.rs; module/re-export lines in mw_rpg/src/lib.rs;
services-adapter.md status line + "As built (A2a)" (3 lines). Read-only: services.rs, sf_rpg::service. No cargo.

## Handoff (status: code complete, NOT compiled; UNTESTED until owner pass 2026-11-10)
Files:
- NEW bevyrrowind/crates/mw_rpg/src/barter.rs: MwService (ids, label GMSTs, ServiceType choice, shape, button order),
  ALL_ITEMS, ProviderFacts/provider_facts (autocalc -> class services), MwItemClass/TradeItem/trade_item, can_sell,
  is_gold/GOLD_IDS, BoundItems (sMagicBound* GMSTs), record_entry_stack/is_restocking (ATTR_RESTOCK "mw.restock"),
  item_health (ATTR_HEALTH "mw.health"), MwTradeHost (Holdings + barter_terms, owned_stock, service_refusal),
  BarterOptions{cap_75}, MwServiceRules<'a, H> impl ServiceRules (trade path; entries empty, buy_entry Unknown,
  vet_deal Ok).
- NEW bevyrrowind/crates/mw_rpg/src/tests_barter.rs: 12 tests.
- EDIT mw_rpg/src/lib.rs: `mod barter;`, `pub use barter::{..}`, `#[cfg(test)] mod tests_barter;`.
- EDIT bevyrrowind/docs/specs/services-adapter.md: status line 3, "As built (A2a)" (spec now 26.2 KB, over budget).

COMPILE BLOCKER: mw_rpg/Cargo.toml needs `smallvec.workspace = true` under [dependencies] (ServiceRules returns
SmallVec; Cargo.toml is another agent's file). Then `cargo hakari generate` + check.

Owner test commands:
- cargo test -p mw_rpg tests_barter
- cargo clippy -p mw_rpg --all-targets -- -D warnings
- cargo fmt -p mw_rpg (lines written at ~120 cols)

Unverified guesses / risk spots:
- Bound items also refused on the player's side (`buys`); OpenMW source for the player pane not checked.
- Weapon item health: thrown/arrow/bolt have none (weapontype.hpp HasHealth, from memory).
- `mw.health` attr key (current condition/uses, absent = full) is a guess; the item port must adopt or rename it.
- `showsInInventory` (OpenMW returns early from TradeItemModel::update) not modelled.
- npc.cpp getServices body came from a fetch summary (truncated file); spec lines 1118-1127 cited.
- fBarterGoldResetDelay rounded up to whole hours (PurseRule is u32 hours).
- Line numbers for dialogue.cpp/tradeitemmodel.cpp are the spec's; fetches had no line numbers.
- Test `sells_hides_...` equips via MwEquipRules::equip on Loadout::new() (same pattern as record_clothing_value).

Framework gaps found (SV1, for the sf_rpg owner):
1. TradeSession::accept copies a restocking line's attrs into the player's AddItem, so the bought copy carries
   `mw.restock` (it won't stack with normal items; harmless for buys, since restocking is only asked of stock).
   Fix: a `ServiceRules` hook for the copied attrs, or strip an adapter-named key.
2. accept calls `haggle` only when balance != merchant_offer; OpenMW applies iBarterSuccessDisposition on every
   accepted deal (onOfferButtonClicked), so a plain-offer deal misses +1 disposition. Fix: always call haggle
   (A1's barter_haggle accepts at once when player <= merchant).
3. The host's service_refusal must reject every `Choice` condition when choice is None (OpenMW Filter gate);
   mw_rpg filter.rs currently returns -1 for Choice, so a "Choice != x" INFO would pass. Plus invertDisposition.

Next slices: A2b Mercantile UseSkill (HaggleOut::mercantile_scale -> ServiceEffect::UseSkill); A2c mw_rpg_bevy wiring
(MwTradeHost impl from sheets + derived disposition + dialogue filter "Service Refusal" with inverted disposition;
item port marks negative inventory counts via record_entry_stack; owned containers/items as holders; GMST labels);
A3-A6 list services; A7 stolen goods.

## For the orchestrator
- CHANGELOG: "2026-11-08 services A2a: `mw_rpg::barter` `MwServiceRules` (service flags to offers in OpenMW order,
  buys/sells filters, bound items, restocking stacks, gold pool, Service Refusal topic, A1 prices and haggle);
  written without compiling, UNTESTED."
- mw_rpg/Cargo.toml: add `smallvec.workspace = true` (blocker), then hakari.
- THIRD_PARTY_NOTICES (OpenMW port, GPL-3.0): "bevyrrowind/crates/mw_rpg/src/barter.rs ports Morrowind merchant rules
  from OpenMW (https://github.com/OpenMW/openmw, GPL-3.0-or-later) at commit ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f:
  apps/openmw/mwclass/{npc,creature}.cpp (getServices, getBaseGold), mwclass/*.cpp canSell and misc.cpp isGold,
  components/esm3/loadnpc.hpp (service bits, AllItems), mwgui/dialogue.cpp (updateTopicsPane, onSelectListItem,
  restock), mwbase/dialoguemanager.hpp (ServiceType), mwdialogue/dialoguemanagerimp.cpp (checkServiceRefused),
  mwgui/tradeitemmodel.cpp (update), mwgui/tradewindow.cpp (onOfferButtonClicked), mwmechanics/mechanicsmanagerimp.cpp
  (isBoundItem), mwworld/containerstore.cpp and mwgui/containeritemmodel.cpp (restocking counts)."
- services-adapter.md is 26.2 KB: move "As built (A1)" to docs/specs/archive/ in a docs slice.
- test-pass-2026-11-10.md: the three cargo commands above.
