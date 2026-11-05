# OpenMW content mechanics inventory (ROADMAP 2.7), 2026-11-05

Research only. OpenMW read at master `ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f` (2026-11-04), paths under `apps/openmw/`.
Line counts are OpenMW's `.cpp` (a Rust port is usually 0.5-1x that without UI).

## Finding zero: the foundation does not exist

Every mechanic below needs one thing we lack: **character stats and a magic-effect runtime.**

- No attributes, skills, fatigue, magicka, level, reputation, faction ranks, bounty: `PlayerStatsPlaceholder` in
  `mw_rpg_bevy/src/dialogue.rs:86` is a hash map of fake filter values; `Disposition` is `base + modifier` only
  (`mw_rpg/src/state.rs`; no personality, race, faction, bounty, crime terms in `DialogueView::disposition`).
- No spells at all: grep finds no `AddSpell`, `Spell`, effect application in `mw_script`, `mw_combat`, `sf_character`,
  `sf_combat`. MWScript functions for stats/spells are not bound to anything.
- Records are available for free: `tes3` (pinned, `mw_records`) parses ALCH, INGR, ENCH, SPEL, MGEF, APPA, REPA,
  LOCK, PROB, CLAS, SKIL, GMST, and NPC/CREA service and travel-destination fields. `sf_records` stores them.
- What exists: gold as `Gold_001` stacks with OpenMW's pile rules (`mw_rpg_bevy/src/inventory.rs`, cites
  `mwclass/misc.cpp`), item transfer between holders, equipment, dialogue with Persuasion topic kinds
  (`DialogueKind::Service`, "Info Refusal" fallback) and a result-script runner (ModDisposition etc.). The
  container screen (`sf_ui_screens/container.rs`, `sf_ui_inventory`) is the nearest thing to a trade screen.
- Mudcrab (`superfargoth/docs/research/mudcrab-2026-10-25.md`): Skyrim actor import and viewer only; no mechanics.
  Nothing to port. OpenMW is the only source.

So slice 0 is **stats + effect runtime**; without it no mechanic can be tested with real numbers. This is also
the "M7 stats/skills/magic" the rpg spec marks out of scope ("rpg.md" line 25); it needs its own spec first
(CONTRIBUTING: no spec, write it first).

## Summary table

| Mechanic | OpenMW files (lines) | Port size | Deps | Our state | Recommended level |
|---|---|---|---|---|---|
| Stats/skills/derived (prereq) | mwmechanics/npcstats (~400), creaturestats, actorutil | 800 | MGEF/CLAS/SKIL/GMST | placeholder only | `sf_stats` generic (attributes/skills/dynamic stats as data-defined ids) + `mw_*` formulas |
| Magic effect runtime (prereq) | spellcasting (630), spells, magiceffects, spellresistance, spellutil (342), summoning | 2000+ | stats, combat hit events | none | generic effect/modifier engine in `sf_*`; Morrowind effect table in `mw_*` |
| Barter / mercantile | mechanicsmanagerimp `getBarterOffer` (563-590), mwmechanics/trading, mwgui/tradewindow (778) | 250 rules + UI | disposition, stats, gold, container UI | gold + transfer only | pure price function in `mw_rpg`; generic `Trade` session + price seam in `sf_rpg`; modern trade UI |
| Persuasion / disposition | `getDerivedDisposition` (475-560), `getPersuasionDispositionChange` (597-700) | 200 | stats, factions, crime, dialogue Service topics | topics exist, rules do not | 1:1 port into `mw_rpg`; UI is a dialogue sub-menu |
| Alchemy | mwmechanics/alchemy (634), mwgui/alchemywindow (~600) | 450 + UI | apparatus, INGR/ALCH, effect runtime for use | none | generic `sf_crafting` recipe-ish seam, Morrowind formula in `mw_*`, modern UI |
| Enchanting + soul gems | mwmechanics/enchanting (416), mwgui/enchantingdialog (~400) | 350 + UI | soul on gems (creature soul value), effects | none | same crafting seam; rules in `mw_*` |
| Spellmaking | mwgui/spellcreationdialog (1000+), spellutil `calcSpellCost` | 300 + UI | effect cost formulas, spell store | none | cost formula in `mw_*`, player-spell storage in generic spell book |
| Repair + recharge | mwmechanics/repair (115), recharge (110), mwgui/merchantrepair (222), recharge | 250 | item condition/charge fields, apparatus | no condition/charge on items | rules in `mw_*`; item condition is a generic `sf_rpg` item field |
| Training | mwgui/trainingwindow (280), `getTrainingSkills` | 150 | skills, gold, time pass | none | rules in `mw_*`; generic "service offer" |
| Travel services | mwgui/travelwindow (305), NPC/door travel destinations | 200 | cell load, time pass, fade | doors with load exist (`mw_doors_bevy`) | generic travel-destination service; fare formula in `mw_*` |
| Spell purchase | mwgui/spellbuyingwindow (294), `calcSpellCost` | 150 | spell book, stats | none | generic service; price in `mw_*` |
| Skill progression from use | npcstats `useSkill`/`increaseSkill`, skill gain GMSTs | 250 | stats | none | modern-first seam (see below) |

## Cross-cutting design: services

All of barter, repair, training, travel, spell buying, spellmaking, recharge are NPC "services" gated by NPC
service flags (`tes3` NPC/CREA `ai_data` flags + travel destinations), opened from dialogue buttons. Recommendation:

- `sf_rpg` gains a generic **`ServiceOffer`** model: a provider (actor id) exposes a set of service kinds (opaque
  `ServiceKind(id)` plus a payload enum for list-style offers: priced entries). The dialogue session emits
  `RpgEvent::OpenService { provider, kind }`; the adapter and UI handle it. No Morrowind names.
- The adapter (`mw_rpg`) maps NPC service flags to offers and supplies the price functions (a `PriceModel` trait
  with `offer(provider, base_price, direction) -> i32`; `mw_rpg` implements OpenMW's formula). This is the seam
  for a modern (Skyrim-like, linear in Speech) or classic model later.
- UI: one `sf_ui_screens` "service list" screen (rows of label + price + action) covers training, travel,
  spell buying, merchant repair. Barter is the container/inventory two-pane screen with a price column.

## Per-mechanic sections

### Barter / mercantile (`mechanicsmanagerimp.cpp` 563-590, `tradewindow.cpp`)
Formula (pure, exact): `a=min(PC Mercantile,100)`, `b=min(0.1*Luck,10)`, `c=min(0.2*Personality,10)`, d/e/f the same for
the merchant; `pcTerm=(disp-50+a+b+c)*PCfatigueTerm`, `npcTerm=(d+e+f)*NPCfatigueTerm`;
`buyTerm=0.01*(100-0.5*(pcTerm-npcTerm))`, `sellTerm=0.01*(50-0.5*(npcTerm-pcTerm))`; price = `max(1, int(base*term))`,
base 0 stays 0, creature merchants unchanged. Haggling (the offer slider, `fBargain*` GMSTs and the Mercantile
skill use on a successful haggle) is in `tradewindow.cpp` (`onOfferButtonClicked`: a haggle succeeds if the
merchant's accepted-offer test `d <= ...` passes; check the file at port time). Also: merchants' gold pool
(`NPC barter gold`, regenerates daily), merchant restocking, per-class buy flags (`Buys/sells` service flags bitset),
"sell stolen item" check (`isItemStolenFrom`). Condition affects value (`value * condition/max`).
Split: formula, gold pool, flags, haggle test are content rules (`mw_rpg`, ~250 lines + tests, exact assertions).
Generic: `TradeSession` (two item lists, balance, accept) in `sf_rpg` with the price seam. UI: modern.
Needs item `value` plus condition fields on item records (not in `sf_rpg::item` today: verify).

### Persuasion / disposition (`mechanicsmanagerimp.cpp` 475-700)
`getDerivedDisposition`: base disposition + crime modifier + `fDispRaceMod` (same race) +
`fDispPersonalityMult*(Personality-fDispPersonalityBase)` + faction term `(fDispFactionRankMult*rank+fDispFactionRankBase)
*fDispFactionMod*reaction` - `fDispCrimeMod*bounty` + `fDispDiseaseMod` (diseased) + `fDispWeaponDrawn`, clamped 0-100.
`getPersuasionDispositionChange`: ratings from persuasion ratings (Speechcraft, Mercantile, Personality, Luck, level, fatigue),
`d = 1-0.02*|disp-50|`, targets for Admire/Intimidate/Taunt/Bribe10/100/1000 (bribe adds `fBribe*Mod`), roll 0-99, outcome
changes `x` (temporary) and `y` (permanent), `iPerMinChance`, `iPerMinChange`, `fPerDieRollMult`, `fPerTempMult`;
intimidate also nudges the NPC's Flee/Fight AI settings; the Persuasion topic reply (`Admire Success` etc.) is picked
by the result. Bribe takes gold. OpenMW deviates from vanilla on marginal intimidation wins (documented in source; keep).
Our side: `ResultEffect::ModDisposition/SetDisposition` and per-NPC modifier exist; the persuasion rules and the
Persuasion topics' selection exist in the DB (`DialogueKind::Service`). Missing: derived disposition inputs and the
persuasion menu. Pure content rule: port 1:1 (`mw_rpg::persuasion`, ~200 lines). The wheel/menu is dialogue UI (modern).
Seam: the `DispositionModel` trait already shaped like `mw_rpg_bevy/dialogue.rs:68` (`fn disposition`) is the right place.

### Alchemy (`mwmechanics/alchemy.cpp`)
Inputs: up to 4 ingredients, 4 apparatus slots (Mortar and Pestle required; Alembic, Calcinator, Retort optional);
effects = effects shared by at least two ingredients (ingredient effect visibility depends on Alchemy skill: the
first effect revealed at 15, then 30/45/60 skill via `fWortChanceValue`; known-effect tracking is per-player state,
`getKnownEffects`). Strength: `x = getAlchemyFactor() * MortarQuality * fPotionStrengthMult` where factor =
`(Alchemy + 0.1*Intelligence + 0.1*Luck) * fatigueTerm`; `value = int(x*iAlchemyMod)`; per effect
`magnitude=(x/fPotionT1MagMult)/baseCost`, `duration=(x/fPotionT1DurMult)/baseCost` (NoMagnitude/NoDuration flags
give 1), then `applyTools` multiplies by Alembic/Calcinator/Retort quality (lines 100-135, separate branches for
negative effects), rounded; success roll `... (Alchemy factor) vs random` with `fPotionStrengthMult` and the
brewing chance (potion created unless the roll fails, which still consumes ingredients); potion name prompt;
skill use on success (`SkillUse Alchemy`). GMSTs: iAlchemyMod, fPotionStrengthMult, fPotionT1MagMult, fPotionT1DurMult,
fWortChanceValue, fatigue terms. Pure rule (~450 lines Rust): `mw_*`. Generic part: a crafting session abstraction
(`sf_crafting`: slots of typed inputs, a deterministic `preview()` and a `craft(rng)`, produces item records created at
runtime) and **runtime-created records** (custom potions, enchantments, spells) through `sf_records` runtime overlay
and saved with `sf_save` - that is the real framework work and also serves enchanting and spellmaking.
UI: modern drag-into-slot screen; classic list later. Potions are used through the inventory (consume = apply effects:
needs the effect runtime).

### Enchanting and soul gems (`enchanting.cpp`, `enchantingdialog.cpp`)
Soul gems hold a creature's soul value (record field `soul` on the soul-trapped gem instance; Soul Trap effect fills
it when a victim dies, see below). Rules: enchant points = sum of effect costs (`getEnchantPoints`: per effect
`(min+max)/2 * duration * baseCost / 40` style, `fEnchantmentMult`, constant-effect duration `fEnchantmentConstantDurationMult`,
self vs touch vs target type multiplier); capacity = gem charge `soul * fSoulGemMult`; chance
`getEnchantChance`: `(Enchant + 0.25*Int + 0.125*Luck) * fatigue - points*fEnchantmentChanceMult*typeMult*count` (line 363);
roll against it (line 83); on success the new ENCH record (runtime) is attached with `applyEnchantment`; always consumes
the gem (powerful-soul check at line 123 for skill gain). Also charge/"recharge by use" for enchanted items
(`mwmechanics/spellcasting` item use consuming charge, `ItemCharge` state). Split like alchemy. Needs: runtime
records, item charge field, soul on gems. ~350 lines + UI.

### Spellmaking (`spellcreationdialog.cpp`, `spellutil.cpp`)
Vendor-gated (a "spellmaking" service NPC). Effect picker by known effects (school filter), magnitude/duration/range/area
sliders; cost = `calcSpellCost` (spellutil.cpp, effect baseCost * (min+max)/2 * duration + area terms, `fEffectCostMult`,
range multiplier); price `cost * fSpellMakingValueMult`-style; limit by `iSpellMakingMax`-ish and the player's own
skills for chance? (vanilla: no chance roll, but price). Port the cost function in `mw_*` (also used by NPC spell
selection in combat AI, spell purchase and the spell list display); the dialog is a UI over the generic effect-list
editor. 300 lines + UI. Depends on the effect table (MGEF) loaded into `mw_*`.

### Repair and recharge (`repair.cpp`, `recharge.cpp`, `merchantrepair.cpp`, `mwgui/recharge.cpp`)
Player repair with a hammer/prong (REPA record, quality, uses): `y = int(fRepairAmountMult * toolQuality * roll)` where chance
uses `(Armorer + 0.1*Agility + 0.1*Luck) * fatigue` vs `fRepairAmountMult` terms (repair.cpp 40-80); success raises
condition by y capped at max, tool uses decrease, break when uses 0; Armorer skill use. Merchant repair: cost
`int(fRepairMult * baseCost*...)` scaled by barter offer (merchantrepair.cpp 70-80), gold taken, condition set to max.
Recharge: soul gem consumed to restore charge, chance `(Enchant + 0.25*Int + 0.125*Luck) * fatigue` and
`fRechargeMult`-style gain (recharge.cpp). Needs item condition and charge as generic item attributes
(`sf_rpg::item`), which also feed combat/armor ratings later. 250 lines. Rule split like above.

### Training (`trainingwindow.cpp`, `getTrainingSkills`)
Trainer offers its three best skills (from NPC skill values, excluding skills where the trainer is lower than the PC;
cap: PC skill < trainer skill, PC skill < 100 is vanilla; ban if skill is Misc and beyond `iTrainingLimit`-style rule? verify).
Cost `pcSkillBase * iTrainingMod` then barter offer; sessions per level limited to 5 trains (`mTimesTrainedThisLevel`?
verify, NpcStats); training passes 2 hours (`pass time`). Port: ~150 lines in `mw_*` (rules), generic service list UI.

### Travel (`travelwindow.cpp`, NPC travel destinations)
Destination list from the NPC record (cell + position + rotation), price `d / fTravelMult` (d = distance to the destination
in world units) - and for the player's own mount services (guild guide, silt strider, boat) the same record type; time
passed `d / fTravelTimeMult` hours (lines 68-70, 207-212); followers go along; blocked while in jail or with a bounty? (checked
in `travelwindow`: `enable` only if no hostile NPC nearby). Needs: cell teleport with fade (exists via doors), time
passing (weather/clock), price seam. Dialogue "Travel" topic handled by the DB service kind. ~200 lines. Generic
`TravelOffer { destination, price, hours }` in `sf_rpg` (no Morrowind names), formula in `mw_*`. Silt strider/boat models
are ordinary NPC + creature placements; no new content work.

### Spell purchase (`spellbuyingwindow.cpp`)
Lists the vendor's spell list (NPC `Spells` subrecord, plus class-based `SpellList` for NPCs autocalculating); price
`max(1, int(calcSpellCost(spell) * iBaseSpellPrice-ish GMST)) ` scaled by barter offer; excludes already known spells;
adds to the PC's spell book. 150 lines. Needs spell book.

## Abstraction options

1. **Per-mechanic ports only (all in `mw_*`).** Fastest to the classic behaviour, but crafting and services get
   re-invented per mechanic; violates the "framework systems stay format-agnostic" rule for runtime records, services
   and price seams.
2. **Generic crafting/service framework (`sf_*`) with Morrowind rules plugged in.** Costs more up front, serves
   other games and a later modern variant. Risk: over-abstracting. Recommended only for the three seams that are
   *actually* shared: (a) **services** (offers, sessions, price model), (b) **runtime-created records** (custom
   potions, enchantments, spells, saved), (c) **item condition/charge**. Everything else (formulas, GMSTs, flags)
   stays in `mw_*`.
3. **Skyrim-style redesign** (Skyrim: Alchemy with any two-plus ingredient combos but known-effect discovery by eating;
   enchanting by learning from disenchanting; spellmaking removed; Speech as a flat price modifier; training capped per level
   via paid trainers; fast travel via carriages; repair at grindstone/workbench by perk). Useful as *UI/flow* inspiration
   (single crafting station screen, preview of the result, no haggling slider) but the rules stay Morrowind: the owner
   wants content ported; feel is modern behind the seam.

Recommendation: option 2 narrowly, with every formula as a trait impl or plain function so a "modern" model can
replace it: `PriceModel` (barter), `DispositionModel` (exists), `CraftModel` (alchemy/enchant chance + output), `SkillUseModel`
(progression). Classic = the OpenMW formulas, shipped first because they are the cheapest to verify against the source.

## Skill progression from use
OpenMW: `NpcStats::useSkill` gives skill gain per use action (GMST `fSkillUse*` per skill, `fMiscSkillBonus`,
`fSpecialSkillBonus`), skill rises on `skillUseValue >= 1`, level up when 10 skill increases; trained skills do not
count for major/minor/misc double-counting. It is a feel system: keep a `SkillUseModel` seam; first
implementation = OpenMW values so alchemy/repair/barter feed Mercantile/Alchemy/Armorer, but expose a hook (event
`SkillUsed { skill, action }`) so the modern progression can listen without the mechanic knowing.

## Soul trap and potion effects (closely tied)
Soul Trap is an MGEF applied through the effect runtime: on the victim dying while the effect is active, the caster's
filled-capacity soul gem is found and its soul set to the victim's soul value (creature `soul` field). Needs: effect
runtime, death event in `mw_combat`, soul gem state (`soul: Option<RecordId>` on item instances: item state in `sf_rpg`).
Potion use = consume item then `apply_effects(effect list, target = self)`; the same function powers spells and
enchantments. So the **effect runtime is the keystone**: build it once (timed modifiers on stats, instant effects, area
and range delivery) and these all reduce to "create effect list + call apply".

## Suggested order of slices (one deliverable each)
0a. Spec: stats, magic and services (`sf_*` + `mw_*` split, runtime records), industry section; no code.
0b. `sf_stats`/`mw_stats`: attributes, skills, dynamic stats, derived (fatigue term), NPC/PC init from CLAS/RACE/NPC; GMST table loaded; tests.
0c. Effect runtime (spell list, modifiers, instant effects, restore/drain/fortify subset first); potion use and `AddSpell`/`GetSkill` family MWScript bindings.
1. Disposition derivation (`getDerivedDisposition`) with real inputs; plug into `DialogueView::disposition`. (Unblocks dialogue gating that today uses placeholders.)
2. Persuasion rules + dialogue persuasion menu.
3. Services core (`ServiceOffer`, `PriceModel`, service flags from NPC records) + generic service list screen.
4. Barter price function + trade session + trade screen (haggle after).
5. Training, then travel, then spell purchase (each one a short slice on the service list screen).
6. Item condition and charge fields, repair (player tool), merchant repair, then recharge.
7. Runtime records (custom potion/enchant/spell, saved), then alchemy.
8. Soul gems + Soul Trap, then enchanting.
9. Spellmaking.
10. Skill-use progression model (feel; owner discussion first).

## Open points for the owner
- Haggling slider and barter gold pools: keep? (default: port formula and gold pool, skip the slider at first.)
- Alchemy ingredient-effect discovery: Morrowind-skill-gated (port) or Skyrim-style eat-to-learn?
- Training/travel are cheap once services exist; spellmaking is the only fully bespoke UI.
- This document did not verify every GMST default or the exact haggle/training constants; read the cited line ranges when porting.
