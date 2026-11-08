# Effects adapter: Morrowind's MGEF, SPEL, ALCH, ENCH, INGR on `sf_effects`

Status (2026-11-08): **agreed** (owner 2026-11-08, defaults taken). A1 written 2026-11-08 without compiling: UNTESTED (owner test pass 2026-11-10). Framework: `../../../superfargoth/docs/specs/effects.md`.
Ported from OpenMW `ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f` (paths under `apps/openmw/` unless noted; line numbers
approximate). Magic effect semantics are content mechanics: one implementation, ported 1:1 (ARCHITECTURE §1b; where
OpenMW deliberately differs from vanilla, follow vanilla). Cast chance, resist roll, reflect/absorb rolls, ingredient
eating and magnitude rolls are rules (classic = OpenMW, modern = framework defaults). There is no
`tickableeffects.cpp` at this commit: per-tick logic is in `mwmechanics/spelleffects.cpp` `applyMagicEffect`.

## Crates

- **`mw_effects`** (new, Bevy-free; deps `sf_effects`, `mw_stats`, `mw_records`/`tes3`; hakari `final-excludes`):
  MGEF table, source mapping (SPEL/ALCH/ENCH/INGR → `SourceDef`), magnitude stats, `OpenMwEffectRules` (classic),
  `MwModernEffectRules` (modern defaults with Morrowind numbers), request handlers' pure parts, `MwGmst` additions.
- Glue (no new Bevy crate): `mw_rpg_bevy` (`effects`: table setup, `Effects` on the player, NPCs, creatures at spawn,
  inventory potion use and equipment, save hooks), `mw_scripts_bevy` (MWScript), `mw_player_bevy` (cast input, HUD icons).

## MGEF → `EffectDef`

| MGEF field | `EffectDef` |
|---|---|
| index 0-142 | `key` = OpenMW's `ESM::MagicEffect` RefId name, case-folded (`fortifyattribute`, `drainhealth`); the index is kept for ESM effect lists |
| `school` (skill index) | `school` = the skill's `StatKey` (`destruction` ...) |
| `base_cost` | `base_cost` |
| flags | record flags OR `HardcodedFlags[index]` (`components/esm3/loadmgef.cpp` 13-207): `TargetSkill`/`TargetAttribute` → `arg: Stat(skill/attribute)`; `NoDuration`, `NoMagnitude`, `Harmful`, `AppliedOnce`, `NonRecastable`, `Unreflectable`, `CasterLinked`, `CastSelf/Touch/Target` → `ranges`; `AllowSpellmaking/Enchanting` → `crafting` |
| resistance | `resisted_by` / `weakened_by` from `MagicEffect::getResistanceEffect` / `getWeaknessEffect` (`loadmgef.cpp` 350-500), as magnitude stats; Fire/Shock/Frost Damage also resisted by Fire/Lightning/Frost Shield (`mwmechanics/spellresistance.cpp` `getEffectResistanceAttribute`) |
| particle, cast/bolt/hit/area statics and sounds, icon, colour, speed | `presentation` |

Stacking from the source class (below); `non_recastable` (bound items, summons) refuses a recast while active
(`spellcasting.cpp` `inflict` ~170, `sMagicCannotRecast`).

## Vanilla effects by archetype

| Archetype | Effects |
|---|---|
| `Modify` (arg) | Fortify Attribute, Fortify Skill (Permanent layer for abilities: `Flag_AffectsBaseValues`, `spelleffects.cpp` ~830) |
| `Drain` (arg) | Drain Attribute, Drain Skill (damage on start, restore same on end: stats-adapter "Effect mapping") |
| `Damage` / `Restore` (arg) | Damage Attribute/Skill (capped at modified, `damageAttribute` 87), Restore Attribute/Skill |
| `Pool` | Restore/Damage/Drain/Fortify Health, Magicka, Fatigue; Fire/Frost/Shock Damage, Poison, Sun Damage (Hurt on health; sun scale is a request input) |
| `Absorb` | Absorb Health/Magicka/Fatigue/Attribute/Skill |
| `MagnitudeStat` | Resist and Weakness to Fire/Frost/Shock/Magicka/Poison/Paralysis/Common/Blight/Corprus Disease/Normal Weapons; Reflect, Spell Absorption, Fire/Frost/Lightning Shield, Sanctuary, Fortify Attack, Chameleon, Invisibility, Silence, Sound, Blind, Paralyze, Burden, Feather, Jump, Levitate, Slowfall, Swift Swim, Water Breathing, Water Walking, Night Eye, Light, Detect Animal/Enchantment/Key, Telekinesis, Fortify Maximum Magicka, Stunted Magicka, Charm (disposition) |
| `Cure` | Cure Common/Blight/Corprus Disease, Cure Poison, Cure Paralyzation, Remove Curse, Dispel (`spelleffects.cpp` ~440-475) |
| `Request` | Open, Lock, Soultrap, Mark, Recall, Divine/Almsivi Intervention, Summon * (25), Bound * (11), Calm/Frenzy/Demoralize/Rally Humanoid/Creature, Turn Undead, Command Humanoid/Creature, Disintegrate Weapon/Armor, Corprus, Vampirism, Extra Spell (unused) |

Magnitude stats (`Value`, base 0, hidden group `effect`) are read by their consumers: resistances by the resist
rule; Chameleon/Invisibility by AI detection; Paralyze, Levitate, Jump, Slowfall, Swift Swim, Water Walking,
Feather, Burden by the character controller and carry capacity; Silence and Sound by the cast rule; Sanctuary and
Attack Bonus by combat; Light, Night Eye, Detect by presentation.

### First slice (A1)

Fortify/Drain/Damage/Restore Attribute and Skill; Restore/Damage/Drain/Fortify Health, Magicka, Fatigue; Fortify
Maximum Magicka (into `MwDerived` magicka max); Fire/Frost/Shock Damage; all Resist*/Weakness* as magnitude stats.
Enough for potions, abilities, birthsigns and racial spells, diseases' attribute effects, and stats S8.

## Records → `SourceDef`

| Record | `SourceClass` | Notes |
|---|---|---|
| SPEL type 0 Spell | `Cast` | `cost` from `calcSpellCost` (`spellutil.cpp` 86: autocalc flag → rounded `getTotalCost`, else record cost); flags `F_Always` → `always_succeeds`, `F_PCStart` → `starts_known` |
| SPEL 5 Power | `Power { GameDays(1) }` | `Spells::canUsePower`/`usePower`: 24 game hours since last use (`spells.cpp` 200-216) |
| SPEL 1 Ability | `Innate` | Permanent layer; `ignore_resist` (`activespells.cpp` 115-126, 302) |
| SPEL 3 Disease, 2 Blight, 4 Curse | `Affliction { disease \| blight \| curse }` | cured by tag; Blight cure skips Corprus spells (`spells.cpp` 168-186) |
| ALCH | `Consumable` | all ranges applied to self, no area (`spellcasting.cpp` `cast(Potion)` 315); `Stack` |
| INGR (eaten) | `Consumable` | one effect from rule `mw.effects.ingredient_eat` (`spellutil.cpp` `rollIngredientEffect` 176) |
| ENCH 0 CastOnce, 2 WhenUsed, 1 WhenStrikes | `ItemCast { Once, OnUse, OnStrike }` | charge from `getEnchantmentCharge` (`spellutil.cpp` 118: autocalc × `iMagicItemCharge{Once,Strike,Use}`), cost per cast `getEffectiveEnchantmentCastCost` (96: `cost - cost/100 × (Enchant - 10)`, min 1); thrown/ammo do not spend charge; CastOnce untracked item (never stacks) |
| ENCH 3 ConstantEffect | `ItemConstant` | instance per equipped item `RefNum`; equip breaks Invisibility (`activespells.cpp` 315-345) |

**Stacking** (`activespells.cpp` `addToSpells` 484-501): key = (source record, caster, item). Not stackable:
innate/constant → `Ignore`; temporary → if the new effect indices are disjoint from the old ones they merge into the
old instance, else the old instance ends (`Replace`). Potions and ingredients are `Flag_Stackable` → `Stack`
(`spellcasting.cpp` 320, 395).

## Semantics to port (application order)

From `spelleffects.cpp` `applyMagicEffect` (1223-1400) and `applyProtections` (323-392):

1. Reflect and Spell Absorption only for temporary sources from another caster: each **active** Reflect / Spell
   Absorption effect is rolled in order, `roll0to99 < magnitude`; reflect re-inflicts on the caster with
   `ignoreReflect` (`activespells.cpp` 386-430); absorption drops the spell and gives the target magicka (`absorbSpell`
   291). Rule `mw.effects.protect` (one roll per active protection effect as input fields).
2. `onHit` for harmful effects from another actor (AI aggression; combat glue).
3. Resistance (`spellresistance.cpp` `getEffectResistance` 25-70): `x = (Willpower + 0.1 Luck) × fatigueTerm`,
   `x *= 50 / castChance` (caster's uncapped chance for spells), `roll = U[0,1] × 100` (minus resistance for
   `NoMagnitude`), `x <= roll` → 0 else (`NoMagnitude` → 100 or `roll / min(x, 100)`), then `min(x + resistance,
   100)`; multiplier `1 - r/100`; 0 → fully resisted (`sMagicPCResisted`). Rule `mw.effects.resist` (classic).
   Modern: `1 - clamp(resist - weakness, -100, 100)/100`, no roll.
4. Magnitude: `min + rollDice(max - min + 1)` (`roll` 47). OpenMW re-rolls rate effects every frame and multiplies by
   `dt` (clipped `min(timeLeft, dt)`); `AppliedOnce` effects roll once. **We roll once per effect** at creation
   (framework rule 3; owner question 3).
5. Rate effects (not `AppliedOnce`, not `NoMagnitude`): amount per second; minimum duration 1 s (`calcEffectCost`).
6. Removal at duration end reverts `AppliedOnce` effects (`onMagicEffectRemoved` 1403, `removeMagicEffect` ~1110-1200).
7. God mode skips Drain/Damage on the player (debug only; `Debug` cause).
8. Rest / wait / travel: `Skip { hours × 3600 / timescale }` (`actors.cpp` `rest` 1903-1940).

Deferred effects: Recall, Interventions, Invisibility wait for a nonzero tick (`spelleffects.cpp` ~1320). Corprus
worsens on the world clock (`activespells.cpp` `worsen` 186-197, `CorprusStats::sWorseningPeriod`): an `Affliction`
handler that re-applies its harmful effects each period. Levitate is removed where levitation is disabled
(`shouldRemoveEffect` 956).

## Casting rules

| Rule | Classic (`OpenMwEffectRules`) | Modern (`MwModernEffectRules`) |
|---|---|---|
| `mw.effects.cast_chance` | `calcSpellBaseSuccessChance` (`spellutil.cpp` 225-270: per effect `x = max(1,dur) × 0.1 baseCost × avg magnitude + area × 0.05 baseCost`, ×1.5 on target, × `fEffectCostMult`; lowest `2 × school skill - x` picks the school; `chance = skill - cost + 0.2 Wil + 0.1 Luck`), then `getSpellSuccessChance` (272-308): powers 100/0 by recharge, Silence → 0, non-spell 100, magicka check, `F_Always` 100, minus Sound magnitude, × fatigue term, clamp 0-100 | 1 when magicka ≥ cost and not silenced; Sound raises cost |
| `mw.effects.cost` (content) | `calcEffectCost` (42-84) / `calcSpellCost` (86) | same (spellmaking, spell purchase, NPC AI use it) |
| roll | `roll0to99 >= chance` → fail, `sMagicSkillFail`, cost spent | no roll |
| skill use | `Spellcast_Success` on success, player only, if `spellIncreasesSkill` (332) | same through `UseSkill` |
| `mw.effects.item_charge` | `getEffectiveEnchantmentCastCost` (96) | same |

## Potion use

Inventory "use" on an ALCH: glue removes one from the stack and sends `Inflict { source: alch id, caster: self,
entries: all ranges as self, area 0 }`; `Stack`. Ingredient eat: `mw.effects.ingredient_eat` (`(Alchemy + 0.2 Int +
0.1 Luck) × fatigueTerm` vs `roll0to99`; duration and magnitude from `y = roll / min(x,100) × 0.25 x`), then
`Inflict` of the first effect only; failure → `sNotifyMessage50`. Both are `CastSpell::cast` (`spellcasting.cpp`
315-420). Item-use skill gain: `Enchant_UseMagicItem` for WhenUsed (`spellcasting.cpp` ~280).

## MWScript semantics

Bound in `mw_scripts_bevy` on the host, synchronous (scripting-adapter.md "MWScript semantics"). The slice pins line
numbers in OpenMW `mwscript/statsextensions.cpp` and `mwscript/miscextensions.cpp`.

| Function | Semantics → op |
|---|---|
| `AddSpell id` | `Learn` (book add; Ability/Disease/Curse start their instance at once, resistances ignored) |
| `RemoveSpell id` | `Forget` (book remove; its innate/affliction instance ends; unselect if selected) |
| `RemoveSpellEffects id` | `EndBySource { id }` (active instances from that source on the reference) |
| `RemoveEffects effect` | `Cure { Archetype(effect) }` without a roll |
| `GetSpell id` | book contains `id` |
| `GetSpellEffects id` | active set has a running source with record `id` (`ActiveSpells::isSpellActive`) |
| `GetEffect effect` | `ActiveSet::magnitude(effect, any) > 0` |
| `Cast id target` | scripted cast from the reference: no chance, no cost, no book check (`mScriptedSpell`); self entries on the caster, touch on `target`, target entries `Launch` toward `target` |
| `ExplodeSpell id` | scripted cast landing at the reference itself: self and touch entries on it, area entries exploded at its position |
| `GetMagicka`/`ModCurrentMagicka`... | stats (stats-adapter "Consumers") |
| `GetResist*` (`GetResistFire` ...) | magnitude stat value |
| `ModResist*` / `SetResist*` | `ModBase`/`SetBase` on the magnitude stat (base is script-owned) |

## Saves

Player: framework `sf.character` v4 `effects`; the init hook for `None` builds the book from race, birthsign and
class/record spells (`Spells::setSpells`, `addAutoCalc`, `spells.cpp` 307-333) and selects nothing. NPCs: overlay
blob `effects` v1 holds the book delta from the record plus running temporary sources. Vanilla ESS import is out of
scope. OpenMW stores used powers as timestamps (`spells.cpp` 243, 303): ours as `GameTime`.

## Classic vs modern

| Content (one implementation) | Rules (modern first, classic = OpenMW) |
|---|---|
| Effect semantics per archetype, flags, resist mapping, stacking key, cure filters, power 24 h, Corprus period, spell/enchant cost and charge, ability Permanent layer | cast chance and fizzle, resist roll, magnitude roll, reflect/absorb rolls, ingredient eating |

## Tests (exact, synthetic records unless marked)

MGEF flags OR hardcoded table; resist mapping per effect; each A1 archetype's ops; drain then restore then expiry
returns to base; potion stacks, same spell from same caster replaces, from two casters stacks; ability on
`AddSpell` and gone on `RemoveSpell`; power refused within 24 game hours; classic cast chance, resist and charge
against hand-computed OpenMW values with vanilla GMSTs; real install: every SPEL/ALCH/ENCH maps without unknown
effects.

## Build plan

| Slice | Deliverable | Test |
|---|---|---|
| **A1** table + first slice | `mw_effects`: MGEF table, magnitude stats in the stat table, A1 archetypes, SPEL/ALCH mapping | mapping and ops tests; real-install sweep |
| **A2** potions | inventory use → `Inflict`, stack, consume; HUD icon list | drinking a Restore Health potion heals over its duration |
| **A3** abilities + book init | race/birthsign/class spells, `Learn`/`Forget`, Permanent layer; stats S8 round trip | fortified attribute equal after a save round trip |
| **A4** MWScript | the table above `live` | interp tests per function |
| **A5** casting | `OpenMwEffectRules` + modern rules, self/touch casts, powers | chance formulas; power recharge |
| **A6** enchantments | constant effect on equip, on use, on strike (combat), charge | charge spent; constant follows the loadout |
| **A7** afflictions + requests | diseases, curses, cures, dispel; first requests (Open/Lock, Levitate, Paralyze) | cure removes by tag; dispel chance |

Later: bolts and area (with projectiles), summons, bound items, Mark/Recall/Interventions, Soultrap (with soul
gems), Corprus, Vampirism, disease transmission on creature hits (`fDiseaseXferChance`; source location to find).

## As built (A1)

Written 2026-11-08 without compiling: UNTESTED (owner test pass 2026-11-10). Crate `mw_effects` (deps `sf_effects`,
`sf_stats`, `sf_records`, `mw_stats`, `tes3`; no `mw_records`): `mgef.rs` (143-row table of OpenMW `RefId` name,
`sEffect*` GMST label, `HardcodedFlags`; resistance / weakness / shield maps; `archetype(index)`; `MwEffects`),
`stats.rs`, `source.rs`, unit tests in `tests.rs`, `tests/real_install.rs` (`#[ignore]`). Choices:

- Flags as OpenMW's legacy loader: `(record & (spellmaking | enchanting | negative light)) | hardcoded`, not a
  plain OR. `EffectDef` has no `school`, ranges or presentation yet (E1 deferred them): they live in
  `MwEffectInfo` (`school`, `ranges()`, `Presentation`). Label = the GMST name. Stacking: class default.
- Only A1 indices get a def (47: the list above plus Poison, and the three elemental shields as magnitude stats
  because they resist fire / shock / frost damage). Other indices keep key and info; a landing is `Rejected`.
- Magnitude stats: key = effect key (`resistfire`), group `magnitude` (as `mw_stats`'s `fortifymaximummagicka`,
  which Fortify Maximum Magicka writes), `Value { min 0 }`. The 22 defs live in `mw_stats::MAGNITUDE_STATS`,
  appended last to `mw_stats::defs()` (`MwIdx::magnitudes`; older indices unchanged, saves are keyed so no version
  bump); `mw_effects::MAGNITUDE_STAT_EFFECTS` names their MGEF indices, a test checks the two agree (2026-11-08).
- Sources: `MwSource` (the adapter's `SourceDef` until E3) with `recharge` (`GameDays(1)` for powers) and
  `affliction` (disease / blight / curse) beside the flat `SourceClass`; `inflict()` builds the op (abilities
  `ignore_resist`). Entries of indices without an MGEF record are dropped and stray arguments cleared
  (`esmstore.cpp` `getSpellsToReplace`); a skill / attribute effect with no valid argument is kept with none and
  reported (`MapNote`). Areas: feet × 0.3048 m. ALCH: self, no area, no cost. SPEL cost: `calcSpellCost` with
  `fEffectCostMult` (`MwEffectGmst`, vanilla 0.5). ENCH / INGR mapping: A6 / A2.

## Open questions (owner)

1. Modern casting never fizzles (Sound raises cost instead), classic keeps the roll: agreed? Owner 2026-11-08: default taken.
2. Modern resist deterministic, classic OpenMW's willpower/luck roll: agreed? Owner 2026-11-08: default taken.
3. Roll magnitude once (vanilla behaviour as we read it) rather than OpenMW's per-frame re-roll of rate effects? Owner 2026-11-08: default taken.
4. NPC temporary effects resume after unload (proposed) or are purged (OpenMW `ActiveSpells::unloadActor`)? Owner 2026-11-08: default taken.
5. `ModResist*` writes a magnitude stat's base: fine, or a script-owned modifier source instead? Owner 2026-11-08: default taken.
