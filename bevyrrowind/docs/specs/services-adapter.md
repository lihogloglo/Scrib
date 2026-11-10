# Services adapter: Morrowind barter, training, travel, spells and repair on `sf_rpg::service`

Status: A1 (formulas) written 2026-11-08, compiled + unit tests + pedantic clippy 2026-11-09; UNTESTED in game (owner test pass 2026-11-10); A2a (barter rules) the same day, same status; A2b-A7 not
built. Framework SV1 (`sf_rpg::service`) written 2026-11-08,
not yet compiled (services.md "As built (SV1)": `QuoteLine` has no `base_value`, the adapter derives it in `quote`;
`FixedPrice` takes `sell_permille`). Owner 2026-11-08: every open question below takes its default. Framework:
[services.md](../../../superfargoth/docs/specs/services.md) (`ServiceRules`, `TradeSession`, `Purse`, effects,
screens). Research: [openmw-mechanics-2026-11-05.md](../research/openmw-mechanics-2026-11-05.md) "Cross-cutting design:
services" and the per-mechanic sections. Stats: [stats-adapter.md](stats-adapter.md) (fatigue term, `MwGmst`, "Use
sources (S9)"). Disposition: the derived-disposition port in `mw_rpg` (a pure `DispositionInput -> i32`, written in
parallel; this spec only reads its output). Persuasion and bribes are not here.

Ported from OpenMW `ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f`; paths under `apps/openmw/`, line numbers at that
commit. Base URL for every citation: `https://github.com/OpenMW/openmw/blob/ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f/`.

<!-- agent-summary -->
> **Agent summary.** `mw_rpg::services` implements `ServiceRules` for Morrowind: AIDT service bits (class bits for
> autocalc NPCs) become offers; per-type "buys" bits filter items; six pure rules (`mw.barter.price`,
> `mw.barter.haggle`, `mw.services.training`, `mw.services.travel`, `mw.services.spell_price`,
> `mw.services.repair_price`) give OpenMW's exact numbers; gold pools reset after `fBarterGoldResetDelay` hours on
> dialogue open. Read "Formulas" for exact maths, "Options" for classic vs modern switches, "Build plan" for slices.

## Goal

Morrowind's merchants, trainers, guild guides, silt striders, spell vendors and smiths work as in OpenMW, behind the
framework's generic services: the same prices to the septim, the same refusals, the same gold limits, the same skill
gain, with the modern trade and list screens. Non-goals: spellmaking and enchanting screens (`ServiceShape::Custom`,
their own slices with the crafting work), persuasion, player-side repair and recharge (research "Repair and
recharge"), companion share.

## Canonical technique

Port OpenMW's content mechanics 1:1 (ARCHITECTURE §1b): each formula reads its numbers, GMSTs included, from one
input struct, and returns a number; rolls are inputs. Barter, training and travel prices are listed among the "feel"
formulas in §1b, but the owner's 2026-11-08 brief is "Morrowind rules ported from OpenMW, modern UI and flow first",
so the OpenMW rules ship first and stay swappable behind `ServiceRules`. Where OpenMW deliberately departs from
vanilla (the 75% barter cap, follower travel pricing), "Options" names the switch and the default.

## Design

### Records and flags

From the `tes3` NPC_/CREA records (field names to confirm at port time): `AIDT` services bitset, `NPDT` gold,
`DODT`/`DNAM` travel destinations (position + rotation, cell name; empty name = exterior), the actor's spell list,
the class's services bitset (CLAS `CLDT`). Services come from the **class** for autocalc NPCs, else from the NPC's
`AIDT` (`mwclass/npc.cpp` 1118-1127); creatures always use their own `AIDT` (`mwclass/creature.cpp` 577-580).

Service bits (`components/esm3/loadnpc.hpp` 37-62):

| Bit | Name | Meaning |
|---|---|---|
| 0x00001 | Weapon | buys weapons |
| 0x00002 | Armor | buys armor |
| 0x00004 | Clothing | buys clothing |
| 0x00008 | Books | buys books |
| 0x00010 | Ingredients | buys ingredients |
| 0x00020 | Picks | buys lockpicks |
| 0x00040 | Probes | buys probes |
| 0x00080 | Lights | buys lights |
| 0x00100 | Apparatus | buys alchemy apparatus |
| 0x00200 | RepairItem | buys repair tools |
| 0x00400 | Misc | buys misc items |
| 0x02000 | Potions | buys potions |
| 0x00800 | Spells | sells spells |
| 0x01000 | MagicItems | buys any enchanted weapon, armor, clothing or book |
| 0x04000 | Training | trains skills |
| 0x08000 | Spellmaking | spellmaking (Custom, later) |
| 0x10000 | Enchanting | enchanting (Custom, later) |
| 0x20000 | Repair | repairs items |

### Offers (`ServiceRules::offers`)

In OpenMW's button order (`mwgui/dialogue.cpp` 586-622); persuasion stays a dialogue button, not a service:

| Condition | `ServiceKind` | Shape | Label GMST |
|---|---|---|---|
| any "buys" bit (`AllItems` mask) | `mw.barter` | Trade | `sBarter` |
| `Spells` | `mw.spells` | List | `sSpells` |
| destinations non-empty | `mw.travel` | List | `sTravel` |
| `Spellmaking` | `mw.spellmaking` | Custom | `sSpellmakingMenuTitle` |
| `Enchanting` | `mw.enchanting` | Custom | `sEnchanting` |
| `Training` | `mw.training` | List | `sServiceTrainingTitle` |
| `Repair` | `mw.repair` | List | `sRepair` |

**Refusal** (`ServiceRules::refusal`, `dialogue.cpp` 443-465): first the general service-refusal check, then the
per-service one; both evaluate the "Service Refusal" dialogue topic with the service as the filter's context
(`DialogueManager::checkServiceRefused`, the dialogue filter port owns the evaluation). An accepted refusal response
is shown as a dialogue line and the service does not open.

### Stock and what a merchant buys

- **Sells** (`mwgui/tradewindow.cpp` 199-216, `tradeitemmodel.cpp` 168-215): the merchant's inventory first, then
  containers owned by the merchant in loaded cells, then world items owned by the merchant (`getContainersOwnedBy`,
  `getItemsOwnedBy`): `stock_sources` returns those holders. Hidden: gold, items not shown in inventory, bound
  items, items the merchant has equipped.
- **Buys** (`buys`, per item type, `mwclass/*.cpp` `canSell`): weapon/armor/clothing/book if its type bit is set
  **or** (`MagicItems` and the item has an enchantment); potion, ingredient, apparatus, lockpick, probe, repair
  tool, light by their own bit only; misc by `Misc`, never gold, never a key (misc `Key` flag). Equipped player
  items are tradeable after OpenMW's unequip (the trade model lists them; the transfer unequips).
- **Restocking items**: inventory-list entries with a negative count are restocking stacks (`containerstore.cpp`
  1030-1044 keeps the sign through adds and subtracts; `cellstore.cpp` 178-193 fixes old saves). While trading,
  OpenMW never removes a restocking quantity from the merchant (`mwgui/containeritemmodel.cpp` ~134: "If this is a
  restocking quantity, just don't remove it"), so they are infinite stock: `restocking` returns true and the
  settlement copies instead of moving. The item port must keep the negative-count marker as an `Attrs` flag
  (`mw.restock`), not a negative count (`sf_rpg` counts are `u32`).
- **Merchant equipping**: OpenMW lets merchants equip what they buy unless `prevent merchant equipping` is set
  (`tradeitemmodel.cpp` `transferItems`). Option below.

### Shared input: `BarterTerms`

Every price below passes through the barter offer, so each rule's `In` embeds the same block, filled by the host
from the sheets (stats-adapter "Fatigue term") and the disposition port:

```rust
pub struct PartyTerms { mercantile: f32, luck: f32, personality: f32, fatigue_term: f32 } // modified values
pub struct BarterTerms {
    merchant_is_creature: bool,
    disposition: i32,                 // derived, clamped 0..=100 (getDerivedDisposition, temporary change included)
    pc: PartyTerms, npc: PartyTerms,
}
```

### Formulas

All `f32` as in OpenMW; `int(x)` is truncation toward zero (C++ `static_cast<int>`).

**Barter offer** `barter_offer(t: &BarterTerms, base: i32, buying: bool) -> i32`
(`mwmechanics/mechanicsmanagerimp.cpp` 563-590):

```text
if base == 0 or t.merchant_is_creature: return base
a = min(pc.mercantile, 100);  b = min(0.1 * pc.luck, 10);  c = min(0.2 * pc.personality, 10)
d = min(npc.mercantile, 100); e = min(0.1 * npc.luck, 10); f = min(0.2 * npc.personality, 10)
pcTerm  = (disposition - 50 + a + b + c) * pc.fatigue_term
npcTerm = (d + e + f) * npc.fatigue_term
buyTerm  = 0.01 * (100 - 0.5 * (pcTerm - npcTerm))
sellTerm = 0.01 * (50  - 0.5 * (npcTerm - pcTerm))
return max(1, int(base * (buying ? buyTerm : sellTerm)))
```

**Effective value of a stack** (`tradewindow.cpp` 38-46): `int(value * (health/max_health if the item has health)
* count)`. Health applies to weapons, armor, lockpicks, probes, repair tools (classes with item health).

**Rule `mw.barter.price`** (one trade line; `tradewindow.cpp` `updateOffer` 608-645):

```text
In { terms: BarterTerms, base: i32 /* effective value of the line */, side: Buy|Sell, cap_75: bool }
cap = max(1, int(0.75 * base))
Buy  (player buys):  price = cap_75 ? max(cap, barter_offer(base, true)) : barter_offer(base, true)
Sell (player sells): offer = barter_offer(base, false)
                     price = (cap_75 and not creature) ? min(cap, offer) : offer
```

`merchant_offer` = Σ Sell − Σ Buy (framework sign convention matches OpenMW `mCurrentMerchantOffer`).

**Rule `mw.barter.haggle`** (`tradewindow.cpp` 48-114; called only when the player changed the balance):

```text
In { terms, player_offer: i32 /* balance */, merchant_offer: i32, roll: i32 /* 1..=100 */,
     fDispositionMod, fBargainOfferMulti, fBargainOfferBase: f32,
     iBarterSuccessDisposition, iBarterFailDisposition: i32 }
Out { accepted: bool, mercantile_scale: f32, disposition_delta: i32 }

if player_offer <= merchant_offer: accepted                       (checked first, creatures included)
else if merchant is a creature: rejected
else:
  buying = merchant_offer < 0;  a = |merchant_offer|;  b = |player_offer|
  d = buying ? (100 * (a - b)) / a : (100 * (b - a)) / b          (integer division)
  a1 = pc.mercantile; b1 = 0.1 * pc.luck; c1 = 0.2 * pc.personality   (NOT clamped, unlike barter_offer)
  d1, e1, f1 likewise for the merchant
  pcTerm  = (fDispositionMod * (disposition - 50) + a1 + b1 + c1) * pc.fatigue_term
  npcTerm = (d1 + e1 + f1) * npc.fatigue_term
  x = fBargainOfferMulti * d + fBargainOfferBase + int(pcTerm - npcTerm)
  rejected if roll > x, or if buying and player_offer > 0 (asking to be paid while buying)
mercantile_scale on acceptance (accepted with a better price than offered):
  selling and |player_offer| > |merchant_offer|: floor(100 * (|p| - |m|) / |p|)
  buying  and |player_offer| < |merchant_offer|: floor(100 * (|m| - |p|) / |m|)
  else 0
disposition_delta = accepted ? iBarterSuccessDisposition : iBarterFailDisposition   (NPC merchants only)
```

The disposition change is applied on **every** accept attempt that reaches the haggle step, including the
no-haggle path (OpenMW `onOfferButtonClicked` 447-457 applies it after `haggle` returns, and `haggle` accepts at once
when the player asks no more than offered). Our `TradeSession` skips `haggle` when `balance == merchant_offer`, so
the adapter's settlement adds `Disposition { delta: iBarterSuccessDisposition }` for NPC merchants on every accepted
deal, and a failed haggle emits `iBarterFailDisposition`. Temporary vs permanent: OpenMW's dialogue manager applies
it as the conversation's temporary change unless `barter disposition change is permanent` ([settings
doc](https://openmw.readthedocs.io/en/latest/reference/modding/settings/game.html)); the disposition port owns
where it lands.

**Mercantile skill use** (stats-adapter S9): an accepted haggle with `mercantile_scale > 0` emits
`StatOp::UseSkill { skill: Mercantile, use_kind: 0 /* Mercantile_Success */, scale: mercantile_scale }`
(`tradewindow.cpp` 112-113; OpenMW's Lua skill handler multiplies the SKIL use value by the scale). A plain
deal at the offered price trains nothing. Use kind 1 (bribe) belongs to persuasion. No service uses Speechcraft.

**Stolen goods** (`vet_deal`, `tradewindow.cpp` 428-445; `mechanicsmanagerimp.cpp` 982-1000): for each Sell line,
if the item id is recorded as stolen from this merchant (owner = the merchant's reference id, or the merchant's
primary faction as a faction owner), show `sNotifyMessage49` with the item name, confiscate that line's items to
the merchant, cancel the deal and close the barter screen. Needs the crime port's stolen-items map (not in the repo
today, grep "stolen" finds nothing): until it lands the check is a no-op, logged as a gap.

**Gold checks and transfer** (`tradewindow.cpp` 411-426, 467-475): refusals `sBarterDialog11` (nothing traded),
`sBarterDialog1` (player can't afford), `sBarterDialog2` (merchant can't afford), `sNotifyMessage9` (haggle
failed); settlement: player gold `+= balance`, pool `-= balance`.

### Merchant gold

`PurseRule::Pool { base: NPDT gold, reset_hours: fBarterGoldResetDelay }` (`mwclass/npc.cpp` 1260-1264, creatures
`creature.cpp` 796). Restock on dialogue open: `if now >= last_restock + delay { gold = base; last_restock = now }`
(`mwgui/dialogue.cpp` 525-540). Training, travel, spell and repair payments add their price to the pool
(`trainingwindow.cpp` 201, `travelwindow.cpp` 197, `spellbuyingwindow.cpp` 193, `merchantrepair.cpp` 167).
A first purse starts at `base` with `last_restock = now` (OpenMW starts at base with timestamp 0; same values).

### Training (`mw.training`, `mwgui/trainingwindow.cpp` 75-250)

- **Offered skills**: the trainer's three highest skills, walking skills in record order and replacing the lowest
  only when strictly greater (ties keep the earlier skill). Value = trainer's **modified** skill, or base when
  `trainers training skills based on base skill` is set (`getSkillForTraining` 236-241).
- **Rule `mw.services.training`**: `In { terms, pc_skill_base: i32, trainer_skill: f32, governing_attr_modified:
  i32, iTrainingMod: i32 }` → `Out { price: i32, refusal: None | TrainerNotBetter | AttributeCap }`:
  `price = barter_offer(max(1, pc_skill_base * iTrainingMod), buying)`; `TrainerNotBetter` if
  `trainer_skill <= pc_skill_base` (`sServiceTrainingWords`); `AttributeCap` if `pc_skill_base >=
  governing_attr_modified` (`sNotifyMessage17`). OpenMW's purchase path omits the `max(1, ...)` the list uses; we use
  the list's value in both so the shown price is the paid price (a 0-skill edge case only).
- **Effects**: `Pay(price)`, `RaiseStat(skill)` → `StatOp::RaiseSkill { source: Training }` (counts toward level
  progress, as OpenMW's `skillLevelUp(..., "trainer")`), `PassTime { hours: 2, sleep: false }` (OpenMW `rest(2,
  false)` then `advanceTime(2)`, with a 2-step progress bar and a fade).
- Rows: refused skills stay listed with the reason (modern: OpenMW only says why after the click). No per-level
  training limit: that is Skyrim's rule, not Morrowind's.

### Travel (`mw.travel`, `mwgui/travelwindow.cpp` 50-235)

- **Destinations**: `DODT` position + `DNAM` cell; empty `DNAM` = exterior cell at that position, named by the
  exterior cell's name; an unknown interior cell is skipped with an error (153-167).
- **Rule `mw.services.travel`**: `In { terms, provider_interior: bool, dist_3d: f32, dist_2d: f32, followers: u32,
  fTravelMult: f32, fTravelTimeMult: f32, fMagesGuildTravel: i32, follower_rule }` → `Out { price, hours }`:

  ```text
  base = provider_interior ? fMagesGuildTravel
                           : (fTravelMult != 0 ? int(dist_3d / fTravelMult) : int(dist_3d))
  base *= follower_multiplier          OpenMW: 1 + followers ("unlike vanilla the first follower doesn't
                                       travel for free", 76-80); vanilla: max(1, followers)
  price = barter_offer(max(1, base), buying)
  hours = provider_interior ? 0 : int(dist_2d / fTravelTimeMult)
  ```

  `dist_3d` is from the **player's** position to the destination; `dist_2d` the same in the horizontal plane
  (price uses 3D, time 2D: 63-67 vs 207-209).
- **Effects**: `Pay(price)`, `PassTime { hours, sleep: true }` (exterior providers only; OpenMW `rest(hours, true)`:
  travel counts as sleeping), `Teleport { to, with_followers: true }` behind a fade (the door teleport path),
  `Adapter("mw.pc_traveling")` setting the flag `GetPCTraveling` reads for one frame; guild guides
  (interior provider) also play "mysticism cast". Confirmation before travel is the framework's modern step.

### Spell purchase (`mw.spells`, `mwgui/spellbuyingwindow.cpp` 50-200)

- **Listed**: the vendor's spells of type Spell (no diseases, curses, powers, abilities), minus its race's powers,
  minus spells the player knows; sorted by name, case-insensitive (stable).
- **Rule `mw.services.spell_price`**: `In { terms, spell_cost: i32, fSpellValueMult: f32 }` → `price =
  barter_offer(max(1, int(spell_cost * fSpellValueMult)), buying)`. `spell_cost` = `calcSpellCost`
  (`mwmechanics/spellutil.cpp` 86-94): the record's cost unless autocalc, else `round(total effect cost)` from the
  magic port's effect-cost function.
- **Effects**: `Pay(price)`, `Grant(spell)` (adds to the player's spell list); the list is rebuilt (the spell drops
  out).

### Merchant repair (`mw.repair`, `mwgui/merchantrepair.cpp` 40-170)

- **Listed**: the player's weapons and armor with item health where `health < max` and `max != 0`, sorted by name.
- **Rule `mw.services.repair_price`**: `In { terms, value: i32, health: i32, max_health: i32, fRepairMult: f32 }`:

  ```text
  p = max(1, value) as f32;  r = max(1, int(max_health / p)) as f32
  x = int((max_health - health) / r);  x = int(fRepairMult * x);  x = max(1, x)
  price = barter_offer(x, buying)
  ```

- **Effects**: `Pay(price)`, `SetAttr { stack, key: condition, value: full }` (the item port's convention for
  "full": cleared attribute, so the stack re-merges as OpenMW's `restack` does).

### GMSTs read (from the loaded content via `MwGmst`; expected Morrowind.esm values, verify in A1's test)

| GMST | Use | Expected |
|---|---|---|
| `fBargainOfferBase`, `fBargainOfferMulti` | haggle | 50, -4 |
| `fDispositionMod` | haggle | 1 |
| `iBarterSuccessDisposition`, `iBarterFailDisposition` | haggle | 1, -1 |
| `fBarterGoldResetDelay` | purse | 24 |
| `iTrainingMod` | training | 10 |
| `fTravelMult`, `fTravelTimeMult`, `fMagesGuildTravel` | travel | 4000, 16000, 10 |
| `fSpellValueMult` | spells | 10 |
| `fRepairMult` | repair | 1 (verify) |
| `fFatigueBase`, `fFatigueMult` | fatigue term (stats-adapter) | 1.25, 0.5 |

### Options (classic vs modern)

`MwServiceOptions`, in the game's settings, read into rule inputs (never inside rules):

| Option | Modern default | Classic | Source |
|---|---|---|---|
| Haggle stepper shown | off (offered price only) | on | framework `TradeConfig::haggle` |
| 75% barter cap | on (OpenMW's exploit fix) | on (vanilla has none: off is "pure vanilla") | `tradewindow.cpp` 612-640 |
| Travel follower pricing | OpenMW `1 + followers` | vanilla `max(1, followers)` | `travelwindow.cpp` 76-80 |
| Trainers use base skill | on (no fortify exploit) | off | OpenMW setting |
| Prevent merchant equipping | on | off | OpenMW setting |
| Barter disposition permanent | off | off | OpenMW setting |
| Refusal reasons on rows, travel confirmation | on | off | framework screens |

## Co-op (net-ready rules)

All rules are pure and host-run; their inputs come from the buying player's sheet (rule 6), the haggle roll is an
input seeded from (provider, session, attempt, tick) (rule 4). Services, sessions and pools are host-owned (rule
5), the dialogue claim makes the provider exclusive, the UI sends `ServiceRequest`s (rule 2). `Teleport` targets
are `WorldPos` (rule 7); events carry tick, player and provider (rule 8). Training's 2 hours and travel's hours go
through the framework's co-op time rule (services.md "Co-op"): party rest vote, else skipped.

## Saves

Pools are the framework's `sf/services` section. Restocking flags, condition and spells are inventory and sheet
state. The stolen-items map belongs to the crime port's section. Nothing else is new.

## Performance

Each rule is under 30 float operations; gathering `BarterTerms` once per session (sheets + disposition) and
reusing it for every line keeps a 1,000-stack quote under the framework's 0.2 ms. Training scans ~27 skills,
travel ≤ 4 destinations, spells ≤ ~100: negligible.

## Industry comparison

| Source | Technique | Us |
|---|---|---|
| OpenMW (files cited above; [settings](https://openmw.readthedocs.io/en/latest/reference/modding/settings/game.html)) | the formulas, flags, pool, refusals | **match**, exact port |
| Morrowind vanilla ([UESP Mercantile](https://en.uesp.net/wiki/Morrowind:Mercantile)) | no 75% cap (buy-sell exploit at high skill), first follower travels free | **option** (classic column) |
| Skyrim ([UESP Speech](https://en.uesp.net/wiki/Skyrim:Speech)) | Speech factor between `fBarterMax` 3.3 and `fBarterMin` 2.0; trainers capped 5 per level; carriages priced per hold | **differ**: Morrowind content keeps its rules; a Skyrim factor rule could replace `mw.barter.price` behind the seam |
| Fallout: New Vegas ([fallout.wiki Barter](https://fallout.wiki/wiki/Barter_(Fallout:_New_Vegas))) | linear in Barter (buy 155%→110%, sell 45%→90%), reputation modifier, condition scales value | condition scaling **match**; reputation not a Morrowind input |
| Elden Ring ([Fextralife Merchants](https://eldenring.wiki.fextralife.com/Merchants)) | fixed prices, infinite or finite stock | **match** for restocking items (infinite); prices not fixed |

UESP refused automated fetches on 2026-11-08; the UESP-sourced values are as remembered, verify before quoting.

## Tests (exact; synthetic records and GMSTs unless marked)

- `barter_offer`: base 0 → 0; creature → base; a hand-computed case per branch (buy, sell, low term → 1).
- `mw.barter.price`: cap on buy (offer below 75% → cap), cap on sell (offer above 75% → cap), creature sell uncapped,
  cap off matches `barter_offer`.
- `mw.barter.haggle`: `player <= merchant` accepts with scale 0 (creature too); creature rejects a better ask;
  integer `d` (e.g. a=101, b=90 → d=10); unclamped a1 vs clamped a; roll = x accepts, x+1 rejects; buying with a
  positive ask rejects; scale floors; disposition deltas from GMSTs.
- Flags: autocalc NPC takes class services; creature uses AIDT; offer order; misc key and gold never bought;
  `MagicItems` buys an enchanted ring but not a plain one.
- Restocking stack: buying 5 of a `mw.restock` stack leaves the merchant's count unchanged.
- Pool: reset at exactly `last + 24 h`; a training payment raises the pool above base.
- Training: top three with a tie keeps record order; both refusals; price with `iTrainingMod` 10 and a Mercantile
  difference; effects in order, `PassTime` 2 h not sleeping.
- Travel: interior provider flat price, 0 hours; exterior price from 3D, hours from 2D (a destination straight up
  costs gold, takes no time); follower rules both ways; unknown cell skipped.
- Spells: known spells, powers and race powers excluded; name order; autocalc cost path.
- Repair: undamaged and zero-max items excluded; hand-computed price; condition cleared re-merges.
- Stolen goods: a line stolen from the merchant's faction vetoes with confiscation (once the crime port exists).
- Marked `#[ignore = "needs Morrowind.esm"]`: GMST defaults as in the table; Arrille's gold and services; the Seyda
  Neen silt strider's destinations and prices against OpenMW at the same stats (owner-run).

## As built

A1 (price formulas, `mw_rpg::services`) and A2a (barter rules, `mw_rpg::barter`), both written 2026-11-08 without
compiling: UNTESTED (owner test pass 2026-11-10). TR1 (2026-11-10, headless tests only): barter in the game host
(`mw_rpg_bevy` `dialogue/services.rs`: buttons, open/refusal, session requests, settlement, merchant stock seeded
from the record). Detail: [archive/services-adapter-history.md](archive/services-adapter-history.md).

## Build plan (one deliverable per slice; framework SV1-SV3 first)

| Slice | Deliverable |
|---|---|
| **A1** | `mw_rpg::services` rules: `BarterTerms`, `barter_offer`, the six rules with exact tests, GMST loading |
| **A2** | Barter: flags → offers, buys/sells filters, stock sources, restocking attr, pool, refusal topic, `ServiceRules` trade path; Mercantile `UseSkill` and disposition effects; wired in `mw_rpg_bevy` dialogue |
| **A3** | Training (entries, effects, 2 h time pass) |
| **A4** | Travel (destinations, teleport with followers, time pass, `GetPCTraveling`) |
| **A5** | Spell purchase (needs the player spell list) |
| **A6** | Merchant repair (needs item condition attrs) |
| A7 | Stolen-goods veto, once the crime port's stolen-items map exists |

## Open questions (owner)

1. **75% cap**: keep OpenMW's anti-exploit cap in the modern rules (default yes), and offer "off" as classic? **Owner
   2026-11-08: default taken** (on; "off" is classic).
2. **Haggling**: port the rule but hide the stepper by default (Skyrim-style fixed offer), or show it? **Owner
   2026-11-08: default taken** (rule ported, stepper hidden behind a setting).
3. **Travel followers**: OpenMW's `1 + followers` or vanilla's free first follower? **Owner 2026-11-08: default
   taken** (OpenMW `1 + followers`; vanilla in the classic column).
4. **Fortify exploits**: trainers by base skill (default) vs vanilla's modified skill? **Owner 2026-11-08: default
   taken** (base skill).
5. **Merchant chests**: OpenMW sells from containers the merchant owns (vanilla behaviour); keep, or inventory only
   (cleaner co-op, fewer surprises)? Default: keep. **Owner 2026-11-08: default taken.**
