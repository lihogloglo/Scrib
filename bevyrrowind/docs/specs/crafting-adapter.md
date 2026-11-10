# Crafting adapter: Morrowind alchemy, enchanting, soul gems, recharge and spellmaking on `sf_crafting`

Status: spec written 2026-11-10; nothing built. Framework: [crafting.md](../../../superfargoth/docs/specs/crafting.md)
(`CraftSession`, `CraftRules`, created records). Also [effects-adapter.md](effects-adapter.md) (`SourceDef`,
`mw_effects::{effect_cost, spell_cost}`, `MwEffectGmst`), [services-adapter.md](services-adapter.md) (offers
`mw.spellmaking` 0x08000 and `mw.enchanting` 0x10000 are `Custom` there and are filled here; `BarterTerms`,
`barter_offer`), [stats-adapter.md](stats-adapter.md) (`MwGmst`, fatigue term, Use sources),
[research](../research/openmw-mechanics-2026-11-05.md). Owner asked 2026-11-05 (ROADMAP 2.7).

Ported from OpenMW `ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f`; paths under `apps/openmw/`, line numbers at that
commit, read from the raw sources 2026-11-10. Base URL:
`https://github.com/OpenMW/openmw/blob/ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f/`.

<!-- agent-summary -->
> **Agent summary.** New Bevy-free crate `mw_crafting` implements `CraftRules` for `mw.alchemy` (free combination,
> apparatus tiers), `mw.spellmaking` (NPC fee, player-spell cost method), `mw.enchanting` (self with a roll, or NPC for
> a fee; soul gem capacity, cast styles) and `mw.recharge`. Each formula is a pure rule with OpenMW lines cited; GMSTs are
> inputs. **Corrections to the research note** (verified in source): alchemy has no fatigue term; enchant chance is
> `(skill - points x mult + 0.2 Int + 0.1 Luck) x fatigue`; alchemy knowledge is positional, not per-player state;
> enchant points are cumulative across effects. Read "Formulas" sections, "GMSTs", "Options", "Build plan".

## Goal

Brew, enchant, recharge and spellmake with OpenMW's numbers for the same stats and GMSTs, through the modern session
and screens, with modern-first switches in Options. Non-goals: disenchanting (not in Morrowind), NPC recharge (OpenMW
offers none, `dialogue.cpp`), repair (services A6), magic-item time recharge, Soul Trap spell semantics (effects A7).

Content mechanics are ported 1:1 (ARCHITECTURE 1b): each rule's `In` holds every number it reads (GMSTs, roll). OpenMW
quirks (cumulative enchant costs, the luck cut-off) stay in the classic column, named.

## Design

### Crate and records

`mw_crafting` (Bevy-free, hakari-excluded; deps `sf_crafting`, `mw_effects`, `mw_stats`, `mw_rpg` for
`BarterTerms`/`barter_offer`). Records (field names to confirm in `tes3`): INGR (4 x effect, skill, attribute; weight,
value), APPA (type, quality), ALCH, ENCH (type, cost, charge, flags, effects), SPEL, CREA `soul`, MISC value, item
`enchant` (capacity, raw) and enchant id, MGEF (flags, base cost, school).

- **Soul gem** = id prefix `misc_soulgem` (`mwclass/misc.cpp` 260-262), tagged `mw.soulgem` at import. Soul on a gem:
  attr `mw.soul` = `Id(creature)`; its charge = that creature's `soul`, 0 if none (`enchanting.cpp` 306-318).
- Item charge: attr `mw.charge` (`Float`, absent = full, the `-1` sentinel, `spellcasting.cpp` 254-256). Item enchant
  capacity = raw record `enchant` (`mwclass/weapon.cpp` 325-330; armor 416, clothing 247, book 158).
- **Openers**: apparatus use opens alchemy (`mwworld/actionalchemy.cpp` 16-28, refused in combat); a soul gem with a
  soul opens self-enchanting (`actionsoulgem.cpp` 21-40, empty gem: `sNotifyMessage32`); dialogue offers open the NPC
  kinds; recharge window is `mwgui/recharge.cpp` 61-141 (its opener not located: verify).

### Shared pieces

**Effect cost methods** (`spellutil.cpp` 42-84). Extend `mw_effects::effect_cost` (only `GameSpell` today,
`source.rs` 214) with `EffectCostMethod { GameSpell, PlayerSpell, GameEnchantment, GamePotion }`:

```text
mag = hasMag ? (min,max) : (1,1);  clamp both to >= 1 for GameSpell and PlayerSpell only
dur = hasDur ? e.dur : 1;  if !AppliedOnce: dur = max(1, dur)
offset = 0, minArea = 0, mult = fEffectCostMult;  PlayerSpell: offset 1, minArea 1;  GamePotion: minArea 1, mult iAlchemyMod
cost = (0.5 (min+max) * 0.1 base * (offset + dur) + 0.05 * max(minArea, area) * base) * mult
```

`getTotalCost` (23-39): sum of `max(0, cost)`, x1.5 per Target effect. `calcSpellCost` (86-94): record cost unless Autocalc,
then `round(total)`. `getPotionValue` (152-160) likewise with `GamePotion`. `getEnchantmentCharge` (118-150): record
charge unless Autocalc, then `round(total, GameEnchantment) x iMagicItemCharge{Once,Strike,Use,Const}`.
`getEffectiveEnchantmentCastCost` (96-116): `r = cost - (cost/100)(Enchant - 10)`, `int(max(1, r))`
(rule `mw.enchant.cast_cost`).

**Skill uses** (`loadskil.hpp` 59-64): Alchemy `CreatePotion` 0, `UseIngredient` 1; Enchant `Recharge` 0,
`UseMagicItem` 1, `CreateMagicItem` 2, `CastOnStrike` 3; gains from the SKIL record, scale 1.0.

**Fees** use `barter_offer(terms, base, buying = true)` (Mercantile and disposition apply, `enchanting.cpp` 300,
`spellcreationdialog.cpp` 727); the NPC purse gets the same amount (`enchanting.cpp` 412-414; dialog 659-661).

### Alchemy (`mw.alchemy`)

Layout: 4 ingredient slots (a second ingredient of one record is rejected, `alchemy.cpp` 469-471), 4 tool slots by type
MortarPestle 0, Alembic 1, Calcinator 2, Retort 3 (`loadappa.hpp` 28-34; only Mortar required, `getReadyStatus` 528-543),
`batch` on ingredients, name required (suggested = first effect's string, 597-604). Tools auto-fill with the best
quality per type (`setAlchemist` 380-419) and are never consumed.

**Shared effects** (`listEffects` 58-86, rule `mw.alchemy.effects`): for slot pairs `i < j` in slot order, for each effect
of ingredient `i` in order, if its key (effect id plus skill/attribute argument, `toKey` 32-40) is not listed and
ingredient `j` has the key, append. Slot order is a player choice because effects apply in list order.

**Rule `mw.alchemy.potion`** (`updateEffects` 147-237):

```text
f = Alchemy + 0.1 Int + 0.1 Luck                 # 341-348; NO fatigue term
x = f * MortarQuality * fPotionStrengthMult;  value = int(x * iAlchemyMod)           # 159-170
per shared effect (base cost > 0):
  mag = NoMagnitude ? 1 : (x / fPotionT1MagMult) / base;  dur = NoDuration ? 1 : (x / fPotionT1DurMult) / base
  if has mag: applyTools(mag); if has dur: applyTools(dur);  mag = roundf(mag); dur = roundf(dur)
  keep only if mag > 0 and dur > 0: range Self, area 0, min = max = int(mag), dur = int(dur)
```

`applyTools` (88-145): `negative = Harmful`, `tool = negative ? Alembic : Retort`. `setup` 1 = tool and Calcinator, 2 =
tool only, 3 = Calcinator only, none = unchanged. `q_t`, `q_c` qualities (0 if absent):

| setup | negative | positive, mag and dur both exist | positive, one missing |
|---|---|---|---|
| 1 | `2 q_t + 3 q_c` | `2 q_t + q_c` | `2/3 (q_t + q_c) + 0.5` |
| 2 | `1 + q_t` | `q_t` | `q_t + 0.5` |
| 3 | `q_c` (+0.5 if one missing) | `q_c` | `q_c + 0.5` |

Then `v += q` if `setup == 3 or !negative`, else `v /= q` (error if 0): tools add to positive effects, divide harmful ones.
(Setup 3 negative uses the same `magnitude && duration ? q_c : q_c + 0.5`, 128-131.)

**Reveal** (`knownEffect` 517-526): shared-effect index `i` is visible if `(i<=1 && s>=W) || (i<=3 && s>=2W) || (i<=5 &&
s>=3W) || (i<=7 && s>=4W)`, `W = fWortChanceValue`; ingredient tooltips show effect `i` while `s >= W (i+1)`
(`effectsDescription` 606-634). Display only: the potion has all effects.

**Outcome** (`create`/`createSingle` 545-595), units `n = min(requested, smallest ingredient stack)` (361-378):
1. Refusals in order: no Mortar `sNotifyMessage45`, < 2 ingredients `6a`, no name `37`, no shared effects `8`. No
   effects **consumes** one of each ingredient (547-551).
2. Per unit: empty rounded list -> consume, fail (575-579); `f < roll0to99` -> consume, fail (581-586; `f` is uncapped, so
   `f >= 99` never fails); else add the potion, consume one of each, skill use `CreatePotion` (588-592).
3. Success if any unit succeeded (`sPotionSuccess`), else `sNotifyMessage8`.

**Created potion** (`addPotion` 297-334): weight = mean of ingredient weights, value = `int(x iAlchemyMod)` (tool bonuses
not included), flags 0, player's name, effects; dedup against dynamic potions with equal name, script, weight, value,
flags and effects (239-281; content records skipped, 256). OpenMW picks one of six `misc_potion_*_01` meshes by
`rollDice(6)` (316-325) outside the dedup key; the port derives the look from the dedup key so it is stable.
Eating an ingredient is effects-adapter "Potion use" (`spellutil.cpp` 176-223, skill `UseIngredient`).

### Spellmaking (`mw.spellmaking`)

NPC service (0x08000). `EditorLimits { max_entries 8 (dialog 885), distinct_effects, magnitude 1..100, duration 1..1440,
area 0..50 }` (slider ranges 100/100/1440/51 in `files/data/mygui/openmw_edit_effect.layout` 41, 53, 77, 100).

- **Known effects** (`startEditing` 773-844): effects of the player's known spells of type Spell whose MGEF has
  `AllowSpellmaking` 0x200 (`loadmgef.hpp` 59), de-duplicated, sorted by name. Rule `mw.spellmaking.known`.
- New entry (125-172): range Self if `CastSelf`, else Touch, else Target; 1-1, duration 1, area 0. Range cycles Self,
  Touch, Target skipping unflagged ones (278-310); Self forces area 0. One entry per effect without an argument
  (922-929); skill/attribute effects ask for the argument first and are not de-duplicated (904-919).
- **Rule `mw.spellmaking.cost`** (`notifyEffectsChanged` 694-735), `PlayerSpell` method:

```text
y = 0; for e: y += max(1, effect_cost(e, PlayerSpell)); if e.range == Target: y *= 1.5        # whole running total
spell.cost = int(y), type Spell, flags 0;  price = barter_offer(max(1, int(y * fSpellMakingValueMult)), buying)
shown chance = min(100, int(calcSpellBaseSuccessChance(spell, player)))     # spellutil 225-270, pre-fatigue, can be negative
```

The running x1.5 differs from `getTotalCost`'s per-effect x1.5 (kept: classic; the stored `int(y)` is what casting uses).
- **Buy** (625-672), refusals in order: no effects `sNotifyMessage30`, no name `10`, cost 0 `sEnchantmentMenu8`, gold
  `18`. Then pay, credit the purse, create the SPEL, `Learn`. No roll, no skill use, no stolen-goods check.

### Enchanting (`mw.enchanting`)

Modes (`enchantingdialog.cpp` 149-184): **NPC** (0x10000): fee, no roll; **self** (from a filled gem): no fee, one roll.
Slots: `Target` (unenchanted enchantable weapon/armor/clothing/book; exact filter `OnlyEnchantable` not read: verify in
`sortfilteritemmodel.cpp`), `Soul` (gem with soul; empty refused on pick, 239-253), editor (8 entries, name required,
`cast_style`). Known effects as spellmaking with `AllowEnchanting` 0x400. Constant mode allows every effect at range
Self, hides Touch/Target and the duration box (dialog 127-128, 245, 285-287). No per-item range restriction found.

**Cast styles** (`nextCastStyle` 118-171; `CastOnce` 0, `WhenStrikes` 1, `WhenUsed` 2, `ConstantEffect` 3,
`loadench.hpp` 28-34), re-run when the item, soul or type button changes; `powerful = gemCharge >=
iSoulAmountForConstantEffect`:

| Item | From | Next |
|---|---|---|
| armor, clothing | WhenUsed | Constant if powerful, else stays |
| armor, clothing | other | WhenUsed |
| weapon | WhenStrikes (Melee/Ranged) | WhenUsed |
| weapon | WhenUsed | Constant if powerful and not Ammo/Thrown; else WhenStrikes unless Ranged |
| weapon | other | WhenStrikes unless Ranged, then WhenUsed |
| book/scroll, none | any | CastOnce |

**Rule `mw.enchant.result`** (`getEffectCosts` 187-221, comment 173-186):

```text
cost = 0; for e in order:
  cost += (((max(1,min) + max(1,max)) * dur + max(1,area)) * base * fEffectCostMult * 0.05;   dur = Constant ? fEnchantmentConstantDurationMult : e.dur
  cost = max(1, cost); if Target: cost *= 1.5;  costs.push(cost)         # cumulative: effect k counts (n-k) times in the sum
points(precise) = sum(costs); points(floor) = sum(floor(c))             # 223-230
base_cast_cost = Constant ? 0 : int(points(floor))  (269-275);  capacity = int(item_points * fEnchantmentMult)  (320-329)
refuse if int(points(floor)) > capacity (sNotifyMessage29);  gem_charge = creature(soul).soul
```

**Chance** (352-370): `x = (Enchant - points(precise) * fEnchantmentChanceMult * typeMult * count + 0.2 Int + 0.1 Luck)
* fatigueTerm`; Constant: `x *= fEnchantmentConstantChanceMult`; `int(x)` (toward zero). Self mode only: fail if
`chance <= roll0to99` (80-88). Shown as `clamp(0,100)` only with OpenMW's `show enchant chance` (dialog 117).

**Create** (63-116), in order: (1) ENCH data (type, cost = base cast cost, flags 0); (2) remove one gem **before the
roll**, re-add Azura's Star `Misc_SoulGem_Azura` (73-78): a failed self-enchant loses the gem, keeps the item; (3) self:
roll, success gives `CreateMagicItem` (86-87); (4) `charge = Constant ? 0 : gem_charge / count`; (5) get-or-create ENCH
(dedup type, cost, charge, flags, effects; dynamic only, 232-267); (6) item variant (`applyEnchantment`): copy of the
base, new name, `enchant = gem_charge` (full, undivided), enchant id; weapons get `Magical` (`weapon.cpp` 256-269),
books become scrolls (`book.cpp` 131-143), armor 300-313; (7) NPC mode: pay the fee (404-415); (8) remove `count` old,
add `count` new. Refusal order before commit (dialog 304-345): no effects `sEnchantmentMenu11`, no name `sNotifyMessage10`,
no soul `52`, no item `11`, over capacity `29`, NPC and gold `18`; then NPC mode: item or gem stolen from this NPC ->
confiscate and close (`49`, 347-372) via `CraftRules::vet` (`DealVeto`). Success `sEnchantmentMenu12`, failure `34`.

**Fee** (NPC only, 284-304): `final = costs.last()`; `base = int(final * fEnchantmentValueMult)`;
`price = barter_offer(base, buying) * int(count * typeMult)`; `max(1, price)`.
**Ammo/thrown** (372-402): only when OpenMW's `projectiles enchant multiplier` `m > 0` and points > 0: `typeMult 0.125`,
`count = clamp(int(gem_charge * m / points), 1, owned)`, per-item charge `gem_charge / count`. `m = 0` (default): one item.
**Using the item** is effects A6 (`spellcasting.cpp` 248-297): WhenUsed and non-projectile WhenStrikes spend
`mw.enchant.cast_cost` of `mw.charge` (`sMagicInsufficientCharge`), skill uses 283-287, 293-297; CastOnce removes it.

### Soul capture

Port of `actors.cpp` `soulTrap` (166-230), on **creature** death (1831-1833): needs a running Soultrap and `creature.soul
> 0`. For the first active Soultrap effect whose caster is a live actor: among that caster's gems with no soul, take the
smallest with `gem.value * fSoulgemMult >= creature.soul` (197-198; first of equal capacity wins); set `mw.soul` on one gem
of the stack (209-214); `sSoultrapSuccess` toast, `VFX_Soul_Trap`, sound `conjuration hit`; **return after the first
success** (line 227: "remove to get vanilla behaviour"). Rule `mw.soultrap.pick`; the Soul Trap effect needs effects A7.

### Recharge (`mw.recharge`)

Self only; pick an enchanted item with a soul gem staged (`mwgui/recharge.cpp` 107-141). Rule `mw.recharge.result`
(`mwmechanics/recharge.cpp` 39-108):

```text
luck_t = 0.1 Luck, set to 1 if < 1 or > 10 (49-51: Luck > 100 counts as 1);  int_t = clamp(0.2 Int, 1, 20)
x = (Enchant + int_t + luck_t) * fatigueTerm;  roll = roll0to99
if roll < x: charge = min(charge + soul * (roll / x), maxCharge)    # a LOW roll restores LESS; max = getEnchantmentCharge(ENCH)
skill use Enchant.Recharge always (87); gem always consumed (88); Azura re-added (101-104); invisibility broken (47)
```

`sNotifyMessage51` when the last gem of a stack is used. Constant items have no charge. `rechargeItem(item, max, dt)` (23-37,
`fMagicItemRechargePerSecond`) is a time recharge whose caller was not found in `actors.cpp`: not vanilla (Options).

## GMSTs read (expected Morrowind.esm values; U = from memory, **unverified**, check in the `#[ignore = "needs Morrowind.esm"]` test)

| GMST | Use | Expected |
|---|---|---|
| `iAlchemyMod`, `fPotionStrengthMult` | potion value / `GamePotion` mult, strength | 2, 0.5 (U) |
| `fPotionT1MagMult`, `fPotionT1DurMult` | magnitude, duration divisors | 1.5, 0.5 (U) |
| `fWortChanceValue` | reveal step | 15 (U) |
| `fEffectCostMult` | effect cost (in `MwEffectGmst`) | 0.5 |
| `fSpellMakingValueMult` | spellmaking price | 3 (U) |
| `fEnchantmentMult`, `fEnchantmentChanceMult`, `fEnchantmentConstantChanceMult`, `fEnchantmentConstantDurationMult` | capacity, chance, constant chance, constant duration | U (0.1 or 10? / ~3 / ~0.5 / ~100) |
| `fEnchantmentValueMult` | NPC fee | U |
| `iSoulAmountForConstantEffect`, `fSoulgemMult` | Constant threshold, gem capacity | U (~400, ~0.4) |
| `iMagicItemChargeOnce/Strike/Use/Const` | autocalc ENCH charge | U (~10/5/20/100) |
| `fMagicItemRechargePerSecond` | time recharge option | U |
| `fFatigueBase`, `fFatigueMult` | fatigue term | 1.25, 0.5 |
| `sNotifyMessage*`, `sPotionSuccess`, `sEnchantmentMenu*`, `sMagicInsufficientCharge`, `sOnetypeEffectMessage`, `sSoultrapSuccess`, `sInventoryMessage3/5` | strings | from content |

## Options (classic vs modern; `MwCraftOptions` feed rule inputs)

| Option | Modern default | Classic (OpenMW) |
|---|---|---|
| Alchemy reveal | also shows effects of ingredients the player has brewed or eaten (discovery set, framework Q1) | skill-gated by position (517-526) |
| Alchemy with no usable result | refused in the preview, nothing consumed | consumes one of each (547-551, 575-579) |
| Alchemy failure roll | kept, preview shows `clamp(f, 0, 100)%` | silent |
| Enchant chance shown | always | only with `show enchant chance` |
| Self-enchant failure | gem lost, item kept (owner Q2) | same (73, 83) |
| Ammo/thrown bulk | on, multiplier 1.0 (owner Q3) | off, one at a time |
| Soul Trap | first trapper only (OpenMW) | vanilla: every trapper |
| Time recharge | off | off |
| Created potion look | stable per dedup key | random of six |

## Tests (exact; synthetic records and GMSTs: `iAlchemyMod` 2, `fPotionStrengthMult` 0.5, `fPotionT1MagMult` 1.5, `fPotionT1DurMult` 0.5, `fEffectCostMult` 0.5, `fEnchantmentChanceMult` 0.5, `fEnchantmentConstantChanceMult` 0.5, `fEnchantmentConstantDurationMult` 100)

- **Shared effects**: three slots with overlapping keys give the loop order; an effect shared by three is listed once; same
  effect with different skill argument is a different key; a repeated ingredient record is rejected.
- **Potion**: Alchemy 50, Int 40, Luck 50, Mortar 1.0, effect base 10: `f = 59`, `x = 29.5`, `value = 59`; no tools:
  magnitude `1.9667 -> 2`, duration `5.9 -> 6`. Retort 1.0 alone positive: `3`, `7`. Calcinator 0.5 alone positive: `2`, `6`.
  Alembic 1.0 alone harmful: divide by 2: `1`, `3`. Each table cell hand-computed; a `NoDuration` effect uses the
  "one missing" column; an effect rounding to 0 is dropped.
- **Outcome**: `f = 59`: roll 59 succeeds, 60 fails; `f = 120` never fails; no shared effects consumes and returns
  `NoEffects`; batch of 3 with rolls [10, 90, 10] makes 2. Reveal at skill 29/30/59/60 (`W = 15`) shows 1/2/3/4 pairs.
- **Created potion**: weight mean; value excludes tools; identical brews share an id; a content potion is never reused.
- **`effect_cost`**: base 10, mag 5-5, dur 20, area 0: `GameSpell` 52.5, `PlayerSpell` 52.75, `GameEnchantment` with and
  without clamp, `GamePotion` uses `iAlchemyMod`, `AppliedOnce` keeps duration 0.
- **Spellmaking**: one Self effect `52.75 -> cost 52`; at Target `79.125 -> 79`; two effects `[52.75 Self, 52.75 Target]`
  -> `(105.5) x 1.5 = 158.25 -> 158` (running total, not `132.1`); refusal order; known effects exclude powers,
  abilities and effects without `AllowSpellmaking`; ninth effect refused; duplicate no-argument effect refused, two
  skill-argument entries allowed.
- **Enchant points**: one Self effect (5-5, dur 20, area 0, base 10): `(10x20+1) x 10 x 0.5 x 0.05 = 50.25`; two such:
  `[50.25, 100.5]`, precise `150.75`, floor `150`; second at Target `[50.25, 150.75]`; Constant uses duration 100 and cast
  cost 0; capacity refusal at `points(floor) = capacity + 1` only.
- **Chance**: Enchant 60, Int 50, Luck 40, fatigue 1, 50.25 points, mult 0.5: `60 - 25.125 + 10 + 4 = 48.875 -> 48`; roll
  47 succeeds, 48 fails; fatigue 0.5 halves before truncation; Constant x0.5; negative truncates toward zero.
- **Create**: failed self-enchant loses the gem, keeps the item, no skill use; Azura re-added; success stores `cost` and
  `charge = gem / count` on the ENCH and `enchant = gem` on the item; NPC mode skips the roll, pays the fee, credits the
  purse; stolen item vetoes; weapon gets `Magical`, book becomes a scroll.
- **Cast styles**: the table for armor, melee, ranged, thrown, ammo, book, with and without a powerful soul. **Fee**: last
  cumulative cost, `barter_offer`, ammo `count 20, 0.125` multiplies by `int(2.5) = 2`; bulk option on/off counts.
- **Recharge**: Enchant 40, Int 50, Luck 50, fatigue 1: `x = 55`; roll 10 restores `soul x 10/55`; roll 55 fails; Luck 120
  gives `luck_t = 1`; Int 200 caps at 20, Int 2 floors at 1; capped at the ENCH charge; gem consumed and skill used on failure.
- **Soul pick**: capacities [30, 60, 120], soul 45 picks 60; gem with a soul skipped; ties first; soul 0 or an NPC picks none;
  one gem of a stack of 3 filled; first trapper only vs all (option). **Charge use**: cast cost at Enchant 60 of base 50 is
  25, at skill 10 is 50, floors at 1.
- `#[ignore = "needs Morrowind.esm"]`: the GMST table; every INGR/ALCH/ENCH/SPEL maps (owner-run).

## Build plan (one deliverable per slice; framework CR1-CR2 first)

| Slice | Deliverable |
|---|---|
| **A0** | `mw_effects`: `EffectCostMethod`, `total_cost`, `potion_value`, `enchantment_charge`, `effective_cast_cost` with the tests |
| **A1** | `mw_crafting::alchemy`: shared effects, `mw.alchemy.potion`, tools, outcome, reveal; exact tests; no UI |
| **A2** | Alchemy kind on the session: layout, apparatus opener, created potion to `SourceDef`, wiring; `visual-checks/crafting.md` entry (awaiting owner) |
| **A3** | `mw.spellmaking`: known effects, cost rule, fee, `Learn`; the `Custom` offer |
| **A4** | `mw_crafting::enchant`: points, chance, cast styles, fee, capacity; gem tag, `mw.soul`/`mw.charge`; exact tests |
| **A5** | Enchanting kind (self and NPC), created ENCH and item variants (`Derived` drafts); wiring |
| **A6** | `mw.recharge` rule, kind, opener |
| **A7** | Soul capture on creature death, once Soul Trap runs (effects A7) |
| A8 | Options UI, ammo bulk, discovery-set reveal |

## Open questions (owner)

1. **Alchemy reveal**: modern column reveals what the player has used, classic keeps the skill gate (default). Pending.
2. **Self-enchant failure**: keep OpenMW (gem lost, chance shown) or spare the gem? Default keep. Pending.
3. **Ammo/thrown bulk enchant**: multiplier 1.0 on (default) or off? Pending.
4. **Spellmaking running x1.5**: keep the shop's number (default) or the per-effect rule? Pending.
5. GMSTs marked U need no decision: the first owner run with Morrowind.esm confirms them.
