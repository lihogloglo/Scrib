# NPC services and trade (`sf_rpg::service`, `sf_rpg_ui`, `sf_ui_screens`)

Status: SV1 built 2026-11-08 (services-sv1); SV2 (`sf_rpg_ui` view-models) and SV3a (`sf_ui_screens` screens) written 2026-11-08, compiled + unit tests + pedantic clippy 2026-11-09; UNTESTED in game (owner test pass 2026-11-10). See "As built". SV3b+ (host wiring) not started. Builds on [rpg.md](rpg.md) (items,
`Holder`, `RpgCommand`, dialogue session), [stats.md](stats.md) "Consumers" (`StatOp::UseSkill`/`RaiseSkill`),
[rule-hooks.md](rule-hooks.md) "Declaring a rule" (formulas are pure `fn(&In) -> Out`), [ui.md](ui.md) "Stock
screens" (container screen) and "Inventory logic (U5)", [net.md](net.md) "Net-ready rules" and "Time, pause and
rest". First user: Bevyrrowind ROADMAP 2.7 ([services-adapter.md](../../../bevyrrowind/docs/specs/services-adapter.md)).

<!-- agent-summary -->
> **Agent summary.** Generic NPC services: an actor exposes `ServiceOffer`s (opaque `ServiceKind` + a screen
> shape); a host-owned `TradeSession` stages two item lists, prices them through the adapter's pure price rules,
> and commits on accept as plain `RpgCommand`s; list services (training, travel, spells, repair) are priced
> `ServiceEntry` rows whose purchase yields generic `ServiceEffect`s. Merchant money is a `Purse` restocked on the
> world clock. UI: a two-pane trade screen on the container screen's parts, and one service-list screen. Formulas
> and flags live in the adapter. Read next: "Design", "Co-op", "Build plan".

## Goal

Every "pay an NPC for something" interaction runs through one model so a game supplies data and formulas, not
screens or transactions:

- **Trade** (barter): buy and sell items in one deal, priced per line, settled in one accept.
- **List services**: pick one priced row (train a skill, travel somewhere, learn a spell, repair an item); pay;
  something happens.
- **Custom services** (spellmaking, enchanting): the offer opens an adapter-owned screen; only the offer and the
  payment step are shared.

Out of scope: the formulas (adapter), persuasion and bribes (dialogue/disposition), player-side crafting.
Non-goals: haggling UI as the default flow (it is an option, below), shops without an actor (a vending chest is
an actor-less provider later if a game needs one).

## Canonical technique

Bethesda's engines and OpenMW share one shape, and we keep it:

1. **Service flags on the actor record** decide which services appear in the dialogue window (Morrowind/OpenMW
   `AIDT` service bits; Skyrim/Fallout vendor factions with "buys" form lists).
2. **A staged deal.** OpenMW's trade window "borrows" items between two item models and only moves them on
   accept (`tradeitemmodel.cpp` `transferItems`); nothing in the world changes until the deal commits, so cancel
   is free. Skyrim moves items at once (each click is its own deal); we stage, because co-op needs one validated
   request per deal and staging makes "cancel" and the price preview exact.
3. **Price = rule(base value, both parties' numbers).** Every engine computes per-item prices from a base value
   and a buy or sell multiplier; only the multiplier differs (OpenMW: disposition + Mercantile + attributes;
   Skyrim: Speech between `fBarterMin` and `fBarterMax`; Elden Ring: fixed).
4. **A merchant purse** apart from the merchant's inventory, reset on a game-time interval (Morrowind 24 h,
   Skyrim 48 h, New Vegas caps restock every few days).

## Design

### Offers and kinds (`sf_rpg::service`, Bevy-free)

```rust
/// Opaque service id from content ("trade", "train", "travel"...). The framework never matches on it.
pub struct ServiceKind(pub RecordId);

pub enum ServiceShape {
    Trade,              // two-pane trade screen + TradeSession
    List,               // service-list screen + ServiceEntry rows
    Custom(RecordId),   // adapter screen id; the framework only opens it and settles payment
}

pub struct ServiceOffer { pub provider: ActorKey, pub kind: ServiceKind, pub shape: ServiceShape,
                          pub label: LabelRef }   // localised button text key
```

The adapter implements one hook trait (rpg.md "Adapter hooks" style):

```rust
pub trait ServiceRules {
    /// Offers for this provider, in button order. Derived from content flags; cheap (called on dialogue open).
    fn offers(&self, ctx: &ServiceCtx) -> SmallVec<[ServiceOffer; 8]>;
    /// A refusal line instead of opening (the dialogue's Service topic); None = open.
    fn refusal(&self, ctx: &ServiceCtx, kind: &ServiceKind) -> Option<(RecordId, RecordId)>;
    /// Trade: where the provider's stock comes from, and which player stacks the provider buys.
    fn stock_sources(&self, ctx: &ServiceCtx) -> SmallVec<[Holder; 4]>;
    fn buys(&self, ctx: &ServiceCtx, stack: &ItemStack) -> bool;
    fn sells(&self, ctx: &ServiceCtx, source: Holder, stack: &ItemStack) -> bool;  // hides equipped, bound...
    /// A restocking stack is never used up by a sale (content's infinite stock).
    fn restocking(&self, ctx: &ServiceCtx, source: Holder, stack: &ItemStack) -> bool;
    /// Prices (each is the adapter's pure rule, In built from live state, see "Price seam").
    fn quote(&self, ctx: &ServiceCtx, line: &QuoteLine) -> i64;
    fn haggle(&self, ctx: &ServiceCtx, deal: &DealTotals, roll: u8) -> HaggleOutcome;
    /// Deal check beyond money and counts (stolen goods); Err aborts the deal with effects.
    fn vet_deal(&self, ctx: &ServiceCtx, session: &TradeSession) -> Result<(), DealVeto>;
    /// List services: rows (priced, enabled or not) and what buying one does.
    fn entries(&self, ctx: &ServiceCtx, kind: &ServiceKind) -> Vec<ServiceEntry>;
    fn buy_entry(&self, ctx: &ServiceCtx, kind: &ServiceKind, entry: &EntryId)
        -> Result<Vec<ServiceEffect>, EntryRefusal>;
    fn purse_base(&self, ctx: &ServiceCtx) -> PurseRule;
}
```

`ServiceCtx { player: PlayerId, provider: ActorKey, provider_base: RecordId, now: GameTime, tick: u64 }`, plus
read access the adapter holds (stats, inventories, records), as `DialogueRules` does. The hook reads; only the host
applies what it returns.

### Trade session (host-owned)

```rust
pub struct SessionId(pub u32);              // monotonic per host run; not saved
pub enum Side { Buy, Sell }                 // Buy: provider -> player. Sell: player -> provider

pub struct TradeLine { pub side: Side, pub source: Holder, pub stack: StackId, pub item: RecordId,
                       pub attrs: Attrs, pub count: u32, pub price: i64 }   // price of the whole line, >= 0

pub struct TradeSession {
    pub id: SessionId, pub player: PlayerId, pub provider: ActorKey, pub kind: ServiceKind,
    pub lines: Vec<TradeLine>,
    pub merchant_offer: i64,   // sum: +Sell lines, -Buy lines (positive = provider pays the player)
    pub balance: i64,          // the player's proposal; equals merchant_offer unless haggled
    pub haggled: bool,
    pub attempts: u16,         // feeds the roll seed
}
```

- `stage(side, source, stack, count)`: adds or grows a line (same `(side, source, stack)` merges); `count` is
  capped by what the holder has now minus what other lines already stage from it. `unstage(line, count)`.
  Each change re-quotes the touched line through `ServiceRules::quote` and recomputes `merchant_offer`; when not
  `haggled`, `balance = merchant_offer`, else `balance` moves by the same delta (OpenMW `updateOffer` keeps the
  player's haggle margin as lines change).
- `set_balance(v)`: haggling. Sign is fixed by the deal direction (the player cannot turn "I pay" into "you pay";
  OpenMW `onBalanceValueChanged`). Only when the adapter enables haggling (`TradeConfig::haggle`).
- `accept(world, rules, roll) -> Result<Settlement, TradeRefusal>`, pure over its inputs, in this order
  (OpenMW `onOfferButtonClicked` order):
  1. `NothingStaged` if no lines.
  2. **Re-validate** every line against current counts (`StockChanged` if a stack shrank: another script, a
     co-op guest's container move). No partial deals.
  3. `PlayerCantAfford` if `balance < 0` and the player's currency count < `-balance`;
     `MerchantCantAfford` if `balance > 0` and the purse < `balance` (purse mode `Unlimited` skips this).
  4. `vet_deal` (adapter: stolen goods). A veto carries its own effects (confiscation) and closes the session.
  5. If `balance != merchant_offer`, `haggle(...)` with the caller's roll; `HaggleFailed` keeps the session open.
  6. `Settlement { commands: Vec<RpgCommand>, purse_delta: i64, effects: Vec<ServiceEffect> }`: one
     `MoveStack` per line (restocking Buy lines become `AddItem` copies to the player instead), the currency
     `AddItem`/`RemoveItem` on the player, `purse_delta = -balance`, and the adapter's extra effects (skill use,
     disposition change) from `HaggleOutcome`.
- `cancel()` drops the session; nothing was moved.

`Currency { item: RecordId }` comes from the adapter (one currency per game for now; New Vegas-style multiple
currencies are an open question). Player money = `count(currency)` in the player's loadout. The trade screen
hides currency stacks from both panes; money moves only via `balance`.

**New `RpgCommand`s** (rpg.md table; also used by the container screen, which today has the app move stacks):

| Command | Effect | Events |
|---|---|---|
| `MoveStack { from: Holder, to: Holder, stack, count }` | take `count` from a stack, add to `to` (merging) | `ItemsChanged` x2 |
| `SetStackAttr { holder, stack, key, value: Option<AttrValue> }` | set or clear one attribute, then re-merge | `StackChanged { holder, stack }` |

### List services

```rust
pub struct EntryId(pub RecordId);           // skill id, destination id, spell id, "holder:stack" for repair
pub struct ServiceEntry { pub id: EntryId, pub label: LabelRef, pub detail: EntryDetail,
                          pub price: i64, pub state: EntryState }
pub enum EntryState { Available, TooExpensive, Refused(LabelRef) }   // refused rows still show, with why
pub enum EntryDetail {                      // display data only, for the row's second line and tooltip
    Stat { stat: RecordId, current: i32, next: i32 },
    Destination { place: LabelRef, hours: u16 },
    Grant { what: RecordId },               // a spell, a recipe...
    Item { holder: Holder, stack: StackId, condition: i32, max: i32 },
    Text(LabelRef),
}

pub enum ServiceEffect {                    // what the host executes after payment, in order
    Pay { amount: i64 },                    // player -> provider purse (always first, always present)
    RaiseStat { stat: RecordId },           // host maps to StatOp::RaiseSkill { source: Training }
    UseSkill { stat: RecordId, use_kind: u8, scale: f32 },
    Teleport { to: TravelTarget, with_followers: bool },
    PassTime { hours: u16, sleep: bool },   // see "Co-op" for the shared clock
    Grant { what: RecordId },               // adapter applies (learn a spell)
    SetAttr { holder: Holder, stack: StackId, key: RecordId, value: Option<AttrValue> },
    Disposition { delta: i32 },             // adapter's disposition state
    Adapter { id: RecordId, payload: AttrValue },   // escape hatch, applied by the adapter
}
pub struct TravelTarget { pub cell: Option<RecordId>, pub pos: WorldPos, pub yaw: f32 }
```

`buy_entry` re-checks the price and refusals at purchase time (the list may be stale by a tick), so a client can
never buy at a quoted price that no longer holds. Rows are recomputed after each purchase (OpenMW re-runs
`setPtr` after a spell purchase: the bought spell disappears).

### Price seam (pure functions)

Every price and roll goes through the adapter's rules, each a pure `fn(&In) -> Out` whose `In` holds every number
it reads, GMSTs included (rule-hooks.md "Declaring a rule"); `ServiceRules::quote`/`haggle`/`entries` only gather
inputs and call them. Rolls are **input fields**: the host draws them from `sf_rng`-style seeding on
`(provider ActorKey, session id, attempts, tick)` (net rule 4), never inside a rule. The framework ships:

- `FixedPrice { sell_ratio }`: buy = base value, sell = `floor(base * sell_ratio)`, no haggle (Elden Ring). The
  test model and the default for games that want no barter skill.
- `QuoteLine { side, item, attrs, count, base_value }`: `base_value` is the adapter's per-instance value
  (condition-scaled) so quotes are per line, matching OpenMW's per-stack price.

A modern Skyrim-like model (`LinearPrice { buy_at_0, buy_at_100 }`) is a few lines when wanted; not built until a
game asks (ARCHITECTURE §1b: Morrowind's rules are the port, the UI flow is modern).

### Merchant purses on the world clock

```rust
pub struct Purse { pub gold: i64, pub last_restock: GameTime }
pub enum PurseRule { Pool { base: i64, reset_hours: u32 }, Unlimited, Inventory }
```

- `Pool` (Morrowind, Skyrim, New Vegas): the purse is world state apart from the inventory. **Lazy restock**:
  when a service opens, if `now >= last_restock + reset_hours` then `gold = base`, `last_restock = now` (OpenMW
  `DialogueWindow::restock`, run on dialogue open). No per-tick work; a merchant nobody visits costs nothing.
- Trades apply `purse_delta`; list services add their price (`Pay`). The purse may go above `base` (paid
  services) until the next restock resets it.
- `Unlimited` (Elden Ring): no purse. `Inventory`: the provider's own currency stacks are its money (a later
  "realistic" option; no game uses it yet).
- Purses live in `WorldServices { purses: BTreeMap<ActorKey, Purse> }`, created on first open. Sessions sit
  beside it in a host resource (`ServiceSessions`), not in saved state.

### Dialogue integration

`DialogueSession` gets no new states. The dialogue view-model lists `offers()` as buttons above the topics
(OpenMW order: services first, then topics). Pressing one sends `ServiceRequest::Open`; the host asks `refusal`
first (evaluated through the dialogue's `Service` topic kind, rpg.md "Database"): a refusal is a normal dialogue
`Line`, the service does not open. Otherwise the host restocks the purse, creates the session and emits
`ServiceOpened`; the UI pushes the trade, list or custom screen over the dialogue. Closing returns to dialogue.

### Commands and events (net rules 2, 5, 8)

Client to host, per player (`ServiceRequest`, carried in the player's command stream):
`Open { provider, kind }`, `Stage { session, side, source, stack, count }`, `Unstage { session, line, count }`,
`SetBalance { session, balance }`, `Accept { session }`, `Cancel { session }`, `BuyEntry { provider, kind, entry }`.

Host to clients (`ServiceEvent`, each with `tick`, `player`, `provider`):
`ServiceOpened { kind, shape, session: Option<SessionId> }`, `ServiceRefusedLine { topic, response }`,
`TradeQuoted { session, lines, merchant_offer, balance }` (the whole priced deal, to its player only),
`TradeDone { session, balance }`, `TradeRefused { session, reason }`, `EntriesListed { kind, entries }`,
`EntryBought { kind, entry, price }`, `EntryRefused { kind, entry, reason }`, `PurseRestocked { gold }`.
Item, stat and clock changes produce their own events (`ItemsChanged`, `SkillRaised`...) as today.

### Screens

**Trade screen** (`sf_ui_screens::TradeScreen`, logic `sf_rpg_ui::TradeView`): the container screen's layout
and parts (two `InventoryView` grids, tooltip, quantity dialog, drag ghost, LB/RB pane switch), plus:

- Each pane filters by `sells`/`buys`; stacks the provider will not buy show greyed with the reason in the tooltip
  (Skyrim hides them; greying teaches what a merchant takes).
- A `price` column/badge on every row, from the host's quotes (`ItemRow` gains `price: Option<i64>`; the inventory
  view's "barter prices" gap, ui.md U5).
- A **deal bar** between the panes: staged lines per side (moving a stack "stages" it: the grid shows it in the
  bar, the source count drops), totals, player and merchant gold, the resulting balance, Accept, Cancel. A or click
  stages a whole stack, X asks how many, drag works as in the container screen.
- Haggle (only when `TradeConfig::haggle`): an offer stepper on the balance (d-pad left/right by 1, LB/RB-held by
  10%), reset button. Off: the balance is fixed, Accept always settles if money allows.
- Refusals show as a toast on the bar ("not enough gold", "merchant can't afford", the haggle refusal line).

**Service list screen** (`sf_ui_screens::ServiceListScreen`, logic `sf_rpg_ui::ServiceListView`): a title,
player gold, a `sf_ui::VirtualList` of entries (label, detail line, price; disabled rows keep their tooltip and
reason, as OpenMW keeps the tooltip on greyed buttons), A buys. Entries with lasting or costly effects
(`Teleport`, `PassTime` over 1 hour) ask for confirmation first (modern flow; Morrowind buys on click). A purchase
with `PassTime` shows the clock-advance progress (training's 2-hour bar) the host drives.

Both screens send intents only; the host applies and re-quotes. Themes style them through `sf-trade-*` and
`sf-service-*` parts next to the existing `sf-inventory-*` ones.

## Co-op (net-ready rules)

1. **Fixed tick**: requests apply on the host tick; restock reads the game clock, never a frame clock.
2. **Commands only**: the UI sends `ServiceRequest`s; nothing in a screen touches inventories or purses.
3. **State apart**: `Purse`, `TradeSession`, `ServiceEntry` are serde data; screens read `ServiceEvent`s.
4. **Pure steps**: price, haggle, training, travel and repair rules are pure; the haggle roll is an input seeded
   from (provider, session, attempt, tick).
5. **One owner**: services and sessions are **host-owned**. A session belongs to the player whose dialogue claim
   opened it (rpg.md "Co-op": the provider is `Busy` for anyone else), so two players never trade with one
   merchant at once; the host still re-validates counts at accept (scripts and containers can change them).
6. **No singleton player**: `ServiceCtx.player` everywhere; quotes use that player's stats; money is that
   player's currency.
7. **Stable ids**: `ActorKey`, `Holder`, `StackId`, `SessionId`, `RecordId`; `TravelTarget` is a `WorldPos`.
8. **Events are data** with tick, player and provider.

**Time-passing services** (training, travel) touch the shared clock (net.md "Time, pause and rest": rest is a
party vote). Rule: single-player passes time as the adapter asks. In co-op, `PassTime` becomes a rest request
for the whole party; if the vote fails or is pending, the service still completes and the time is skipped (the
traveller arrives "instantly"; Valheim and Elden Ring have no travel time). `Teleport` moves the buyer and their
followers only. Owner question 1.

## Saves

- New section **`sf/services`** (owner `sf_rpg`, version 1): `WorldServices { purses }`. Purses keyed by
  `ActorKey`; a purse for an actor that no longer exists is dropped on load with a warning.
- Sessions are not saved. Saving with a service screen open closes it first (the save menu is a separate screen
  in single-player; in co-op the host cancels open sessions in the save snapshot only, since staged items never
  moved, nothing is lost).
- Restocking stacks and "already bought" spells are ordinary inventory and character state.
- As built 2026-11-10: `sf_save_bevy::sections::services`, wired in `mw_saves_bevy`; the load-side drop of missing
  actors' purses is not built ([archive/services-history.md](archive/services-history.md) "Saves (as built)").

## Performance

- Quoting is arithmetic: one rule call per stack (~100 ns with input gathering). Opening a trade with a
  1,000-stack player inventory and a 200-stack merchant: ≤ 0.2 ms host, once on open; each stage/unstage
  re-quotes only the touched line. Budget: **≤ 1 ms host per request**, measured in the SV1 bench.
- No per-tick cost: purses restock lazily; sessions only react to requests.
- UI: the trade screen is two inventory grids (U5 budget: 1,000 stacks re-sort in 0.30 ms) plus a deal bar of at
  most a few dozen rows; list screens are virtualized lists.
- `TradeQuoted` carries the full deal (dozens of lines, < 4 KB); pane prices for all stacks go once per open.

## Industry comparison

| Engine / game | Technique | Us |
|---|---|---|
| OpenMW / Morrowind ([tradewindow.cpp](https://github.com/OpenMW/openmw/blob/ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f/apps/openmw/mwgui/tradewindow.cpp), [dialogue.cpp](https://github.com/OpenMW/openmw/blob/ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f/apps/openmw/mwgui/dialogue.cpp)) | staged "borrowed" items, one accept, haggle slider with a roll, gold pool reset 24 h on dialogue open, service flags gate buttons, refusal via service dialogue | **match** (staging, purse, flags, refusal); haggle is an option, not the default UI |
| Skyrim ([UESP Speech](https://en.uesp.net/wiki/Skyrim:Speech), [CK fBarterMin](https://ck.uesp.net/wiki/Game_Settings)) | price factor from Speech between `fBarterMax` 3.3 and `fBarterMin` 2.0 (buy x factor, sell / factor), perks multiply; merchant gold resets 48 h; each transfer is immediate | **differ**: we stage a deal (co-op, exact preview). Factor model possible via a `LinearPrice` rule; not built (gap, low) |
| Fallout: New Vegas ([fallout.wiki Barter](https://fallout.wiki/wiki/Barter_(Fallout:_New_Vegas))) | linear in Barter: buy 155%→110%, sell 45%→90% of base; faction reputation modifier; caps restock periodically; item condition scales value | **match** in shape (rule input includes condition via `base_value`); reputation goes in the adapter's rule input |
| Elden Ring ([Fextralife Merchants](https://eldenring.wiki.fextralife.com/Merchants)) | fixed prices, no haggle, no merchant money limit, finite or infinite stock per item | **match** via `FixedPrice` + `PurseRule::Unlimited` + `restocking` stacks |
| Oblivion / Morrowind haggling | haggle mini-game or slider | **option** (`TradeConfig::haggle`), owner question 2 |

UESP refused automated fetches on 2026-11-08: the Skyrim values are UESP's as remembered; verify before quoting.
Gaps: multiple currencies (New Vegas casino chips, owner question 3); buy-back of just-sold items at the sell
price (Skyrim does not; not planned).

## Tests

Bevy-free, exact assertions, synthetic records (`sf_rpg` tests):

- **Offers**: a fake `ServiceRules` with three offers yields them in order; refusal returns a line and opens nothing.
- **Session math**: stage 3 + 2 lines both sides; `merchant_offer` is the signed sum; unstage restores; staging
  more than held is capped; same-stack staging merges; haggled `balance` keeps its margin when a line is added.
- **Accept order**: each refusal in order (`NothingStaged`, `StockChanged` after a hidden `RemoveItem`,
  `PlayerCantAfford`, `MerchantCantAfford`, veto with confiscation, `HaggleFailed`), and no state change on any.
- **Settlement**: exact `RpgCommand` list for a mixed deal; restocking Buy lines copy instead of move; currency
  never staged; `purse_delta = -balance`.
- **Purses**: restock at exactly `last + reset_hours` (boundary inclusive), not one minute before; paid services
  raise the purse above base; `Unlimited` never refuses.
- **Rolls**: same (provider, session, attempt, tick) → same roll; a second attempt differs.
- **List services**: `buy_entry` re-prices at purchase (price rose → `TooExpensive`); effects start with `Pay`.
- **Commands**: `MoveStack` merges into an identical stack at the target; `SetStackAttr` clearing `condition`
  re-merges with a plain stack.
- **Saves**: `WorldServices` serde round trip and shape golden; a purse for a missing actor drops on load.
- **Co-op** (headless, two `PlayerId`s): the second player's `Open` on a claimed provider fails `Busy`; quotes for
  player 1 and 2 differ when their stats differ.
- UI view-models (`sf_rpg_ui`): staging via A/X/drag yields the right `Stage` requests; disabled rows keep tooltips.

## Build plan (one deliverable per slice)

| Slice | Deliverable |
|---|---|
| **SV1** | `sf_rpg::service`: kinds, offers, `ServiceRules` trait, `TradeSession` (stage, quote, accept, settlement), `Purse` + lazy restock, `FixedPrice`, `MoveStack`/`SetStackAttr`, `sf/services` save section; tests above; bench for 1,000-stack quoting |
| **SV2** | `sf_rpg_ui`: `TradeView` (two `InventoryView`s, deal bar model, price column) and `ServiceListView`; dialogue view-model service buttons |
| **SV3** | `sf_ui_screens`: `TradeScreen` on the container parts, `ServiceListScreen`; host wiring of `ServiceRequest`/`ServiceEvent` in the game crate; visual check (gamepad + mouse) |
| SV4+ | adapter slices (services-adapter.md "Build plan") |

## As built (SV1, SV2, SV3a)

SV1 (`sf_rpg::service`, `tests_service.rs`) and SV2 (`sf_rpg_ui`: `TradeView` in `trade.rs`, `ServiceListView` in
`service_list.rs`; `tests_trade.rs`, `tests_service_list.rs`), SV3a (`sf_ui_screens`: `trade.rs`, `service_list.rs`, `tests_services.rs`) written 2026-11-08 without compiling: UNTESTED (owner
test pass 2026-11-10). Build decisions (SV1: no `base_value`, integer `sell_permille`, `MerchantMoney`, splitmix64
seed...; SV2: price column as a per-pane quote map, not `ItemRow::price`; the SV3 host contract):
[archive/services-history.md](archive/services-history.md).

## Open questions (owner)

1. **Co-op time**: training and travel pass hours on the shared clock. Default above: a party rest vote, else the
   time is skipped. Alternatives: always skip time in co-op; or only the buyer "sleeps" (no shared effect).
   **Owner 2026-11-08: default taken.**
2. **Haggling**: keep an offer stepper (Morrowind's rule, optional UI) or fixed prices only (Skyrim/modern)?
   Default: the rule is ported, the stepper ships hidden behind a setting. **Owner 2026-11-08: default taken.**
3. **Currencies**: one currency per game, or a list (New Vegas)? Default: one, `Currency` becomes a list later.
   **Owner 2026-11-08: default taken.**
4. **Staging vs immediate**: staged deal (OpenMW, co-op friendly) is the default; Skyrim's per-click transfer
   could be a UI mode over the same session (stage + accept per click). Wanted? **Owner 2026-11-08: default taken**
   (staged deal only).
