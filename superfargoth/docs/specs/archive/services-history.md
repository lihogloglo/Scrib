# Services: history

Moved out of [services.md](../services.md) on 2026-11-08 (doc budget), word for word: the "As built (SV1)" notes.
services.md and the `sf_rpg::service` code win where they differ. Headings are unchanged.

## As built (SV1)

2026-11-08, services-sv1. `sf_rpg::service` (`src/service/{mod,trade,purse,price,host}.rs`), tests in
`src/tests_service.rs`. **Not compiled or run** (this machine cannot build): the next session with cargo runs
`cargo test -p sf_rpg` and clippy first. Simplest readings taken where the design was open:

- `QuoteLine` has no `base_value`: it carries `side, source, item, attrs, count`; the adapter derives the
  per-instance value inside `quote` (no extra hook). `FixedPrice { sell_permille }` (integer thousandths, not an
  `f32` ratio: identical on every peer); input `FixedPriceIn { side, unit_value, count }`, sell floors per piece.
- `TradeEnv { ctx, rules, holdings, config }` bundles what a session reads; `Holdings::loadout(Holder)` is the
  inventory view (impl for `BTreeMap<Holder, CharacterRpg>`). `stage` returns the pieces staged; refusals while
  staging (`WrongSource`, `NoStack`, `Currency`, `NotTraded`, `NothingLeft`, `ZeroCount`) share `TradeRefusal`.
  Sell sources must be the player; Buy sources must be in `stock_sources`. Sold lines go to `Holder::Actor(provider)`.
- `accept(&mut self, env, MerchantMoney, roll)`: the only session change on refusal is `attempts += 1` when a
  haggle was rolled. `MerchantMoney { Pool(gold), Unlimited, Inventory }` comes from `MerchantMoney::of(rule,
  services, provider)`; `Inventory` also moves currency on the provider, and `purse_delta` is non-zero only for a
  pool. Currency commands follow the line commands. `set_balance` clamps to the deal's sign (offer 0 → 0).
- Rolls: `service_seed(provider, session, attempt, tick)` is a chain of splitmix64 finalizers (each a bijection, so
  any changed input changes the seed; no `sf_rng` dependency); `haggle_roll` = seed % 100.
- `open(rules, ctx, kind, claims, services, sessions)` does Busy (dialogue claim or another player's session) →
  `NotOffered` → refusal line → lazy restock (also for list services) → session for `Trade` only. `purchase(...,
  player_gold)` requires a leading `Pay` and re-checks gold; `payment_command` builds the player's `RemoveItem`.
  `ServiceRequest`/`ServiceEvent` are defined as data; the host loop that applies them is SV3.
- `MoveStack` spans two characters: `CharacterRpg::apply` returns `TwoHolders`; hosts call
  `CharacterRpg::apply_move(source, target, cmd)` (atomic: `Loadout::can_add` checks the target first).
  `RpgCommand::holder()` is the source, `target()` the destination. `SetStackAttr` (`Loadout::set_attr`) re-merges
  unless the stack is equipped or split-equipped; a re-merge emits `StacksMerged` then `StackChanged`.
- `TravelTarget.pos` is `TravelPos { cell: [i32; 3], local: [f32; 3] }` (`WorldPos`'s canonical form; `sf_coords`
  is not a dependency and `WorldPos` has no serde). `LabelRef(RecordId)` is a local text key.
- Save: `SERVICES_SECTION = "sf/services"`, `SERVICES_VERSION = 1`, `WorldServices::drop_missing(exists)` on load;
  the host registers the section (sf_rpg does not depend on `sf_save`).
- No `benches/` in sf_rpg: the 1,000 + 200-stack quote is `measure_pane_quoting` (`#[ignore = "measurement"]`,
  via `pane_prices`); no number recorded yet.


## As built (SV2)

2026-11-08, sv2-views. `sf_rpg_ui::{TradeView, ServiceListView}` (`src/trade.rs`, `src/service_list.rs`), tests in
`src/tests_trade.rs` and `src/tests_service_list.rs`; `sf_rpg_ui` now depends on `sf_ui_inventory`. **Not compiled or
run**: written 2026-11-08 without compiling, UNTESTED (owner test pass 2026-11-10). Readings taken:

- **Price column**: `TradeView::set_prices(pane, ...)` takes `pane_prices` output (`(StackId, Option<i64>)`) into a
  per-pane map instead of adding `price` to `sf_ui_inventory::ItemRow` (crate untouched). A stack with no quote is
  greyed (`RowBlock::NotTraded`) and never staged, so the host must re-send pane prices after `TradeDone` and after
  any `ItemsChanged` that adds a stack (new stacks are otherwise unsellable until re-priced).
- **Currency hidden** by filtering `InventoryView::visible()` (`TradeView::shown`); pane focus moves over the shown
  list, so it never lands on currency. A native "hide items" filter on `InventoryView` would be cleaner (follow-up).
- **One stock source per pane**: the provider pane is one `InventoryView` (one `Holder`). `stock_sources` returning
  several holders (owned chests) needs either one pane per source or a merged view: open for SV3/adapter.
- **Optimism**: only the haggle balance is shown before the host answers (`set_offer`); staging waits for
  `TradeQuoted`. `accept` locks until `TradeDone`/`TradeRefused`; list `BuyEntry` locks until `EntryBought`,
  `EntryRefused` or `EntriesListed`.
- **After `TradeDone`** the view clears its lines and keeps the same `SessionId` (the trade window stays open, as in
  OpenMW); the SV3 host must keep or reset the session to match, and apply `purse_delta` (the view does `Pool -=
  balance` itself for display).
- **Confirmation**: `ConfirmRule = fn(&ServiceEntry) -> bool`, stock `confirm_lasting` = travel rows
  (`EntryDetail::Destination`). Training that passes time needs an adapter rule (the entry carries no effects).
- **Events**: views take `&ServiceEventKind`; the caller routes by player and provider (`PurseRestocked` is not
  session-scoped). Item events go through `TradeView::on_rpg_event`, which names the pane to re-sync.
- Not built (SV3): the Bevy screens, the PassTime progress bar, dialogue service buttons (dialogue.rs, separate slice).

## As built (SV3a)

Written 2026-11-08 without compiling: UNTESTED (owner test pass 2026-11-10). `sf_ui_screens`: `trade.rs`
(`TradeScreen`, `TradeParts`, plus the shared `ServiceScreenEvent` / `ServiceIntent` message and `ServiceLabels`),
`service_list.rs` (`ServiceListScreen`, `ServiceListParts`, `ServicePart`), `tests_services.rs` (4 headless tests).

- **Panes are lists, not icon grids.** The spec says "the container screen's parts (two grids)"; the inventory
  `GridPane` binds `InventoryView::visible()` and shows `stack.count`, so it would show currency and the full count,
  with no price column. The trade panes are keyed lists (name, pieces not staged, price) in a `ScrollArea`;
  tooltip, count dialog (`SplitDialog`) and drag ghost are the inventory's shared parts. Blocked rows: class
  `blocked`, the reason in the price/count column ("not traded", "on the bar"), not in the tooltip (the shared
  tooltip has no reason line). Follow-up: a hide filter and a price badge in `GridPane` would let a theme pick.
- **Layout**: provider pane, deal bar (320 px), player pane; root 1100 x 600, capped at 100% (Steam Deck 1280 x 800).
- **Pad map**: d-pad up/down = `Move(±1)`; sideways from a pane towards the middle = `Bar`, from the bar = that
  pane; on the offer stepper left/right = `Step(±1)`, LB/RB = `StepBig(±1)`; LB/RB elsewhere = pick a pane; X
  (Context1) = `Quantity`, Y (Context2) = `Accept` (Start stays the menu key), LS (Context3) = `ResetOffer`, RS
  (Context4) = `Bar`, B = `Back`. Count dialog: left/right by 1, up/down by 10. A arrives as `Activate`.
- **Close**: the trade screen closes itself once `TradeView::is_closed` (cancel or a host veto); the list sends
  `ServiceIntent::Close` and closes itself. One message type for both screens (`ServiceScreenEvent`).
- **Labels**: `ServiceLabels` resource (fixed strings + a `LabelRef` key table, key shown when missing); toasts stay
  until the deal's lines change. The list spawns every row (short lists); the view's `VirtualList` only drives focus
  and paging. `progress: Option<f32>` on `ServiceListScreen` is the host-driven PassTime bar.
- Tests open the screens under the demo registry's `container` / `inventory` ids; a host registers its own ids.

## Fix (2026-11-09): two gaps found by services A2a

UNTESTED. `ServiceRules` gained two default methods: `restocked_copy_attrs` (a restocking sale gives the player a copy
whose attrs content may clean, so `mw.restock` no longer leaks onto bought items) and `plain_deal_effects` (effects of
a deal accepted at the merchant's own offer; `OpenMW` applies `iBarterSuccessDisposition` on every accepted deal).
`TradeSession::accept` calls the second when the balance equals the offer, the haggle otherwise.
