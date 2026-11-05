# Stats adapter: Morrowind's attributes, skills, pools, factions on `sf_stats`

Status (2026-11-05): spec, nothing built. Framework: `../../../superfargoth/docs/specs/stats.md` (build plan S0-S9).
Ported from OpenMW `ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f` (paths under `apps/openmw/` unless noted; line numbers
approximate). Content mechanics port 1:1; growth and recovery formulas are the first implementations of the rules seams
(owner 2026-11-05: `SkillUseModel` starts with OpenMW's values).

## Crates

- **`mw_stats`** (new, Bevy-free; deps `sf_stats`, `sf_records`, `mw_records`/`tes3`; hakari `final-excludes`): table,
  `MwGmst` (typed GMST snapshot), init, `MwDerived`, `OpenMwSkillUse`, `OpenMwLeveling`, rank requirements.
- Glue (no new Bevy crate): `mw_rpg_bevy` (NPC/creature sheets, dialogue `MwView`), `mw_scripts_bevy` (MWScript),
  `mw_player_bevy` (player sheet, combat vitals, HUD).

## Stat table

| Group | Keys (OpenMW `RefId` names) | Kind |
|---|---|---|
| Attributes (8, ESM indices 0-7) | `strength intelligence willpower agility speed endurance personality luck` | `Value { min 0, cap none }` |
| Skills (27, SKIL records: governing attribute, specialization, `use_value[4]`) | `block armorer mediumarmor heavyarmor bluntweapon longblade axe spear athletics enchant destruction alteration illusion conjuration mysticism restoration alchemy unarmored security sneak acrobatics lightarmor shortblade marksman mercantile speechcraft handtohand` | `Value` |
| Pools | `health` (`depleted_below` 1, `KeepCurrent`, stored base), `magicka` (`KeepRatio`, derived), `fatigue` (`KeepRatio`, derived) | `Pool` |
| Counters | `level`, `reputation` (0-255), `bounty` (≥ 0), `level.progress`, `progress.<skill>` (27), `levelup.attr.<attribute>` (8), `levelup.spec.<combat/magic/stealth>`, `werewolf.kills` | `Counter`, all but level/reputation/bounty hidden |
| Derived | `capacity`, `encumbrance` (carried weight from `Inputs`), `fatigue.term` | `Derived` |
| Magnitude (if the effect spec adopts them) | `fortifymaximummagicka`, resistances, ... | `Value`, base 0 |

Roles: `Health` = health, `Mana` = magicka, `Stamina` = fatigue, `Level`, `CarryCapacity` = capacity, `Encumbrance`,
`Bounty`. Groups: FACT record ids; NPC membership from the NPC record's faction and rank.

## Rules or content

| Content (one implementation, shared by modern and classic) | Rules (seam, OpenMW first, swappable) |
|---|---|
| Stat set, governing attributes, specializations (SKIL) | `MwDerived`: magicka and fatigue maxima, capacity, fatigue term, regen, rest |
| NPC record values and autocalc; creature records | `OpenMwSkillUse`: progress per use, skill raise tallies |
| Chargen bonuses (race, class, birthsign) | `OpenMwLeveling`: readiness, attribute multipliers, health per level |
| Faction rank requirements, reputation, expulsion | `DispositionModel` (exists, `mw_rpg_bevy/src/dialogue.rs:65`): stats only supply its inputs |
| Effect semantics on stats (effect spec), MWScript semantics, crime bounty amounts | |

## Initialisation

- **NPC, not autocalc**: level, attributes, skills, health, magicka, fatigue from NPDT; base disposition and reputation
  (`mwclass/npc.cpp` 305-330).
- **NPC autocalc** (`npc.cpp` `autoCalculateAttributes` 114-174, `autoCalculateSkills` 190-245): dynamics start at 10;
  attributes = race base (by sex) + 10 per class favoured attribute + round((level - 1) x Σ over its governed skills
  of 1.0 major / 0.5 minor / 0.2 misc), max 100; health = floor(0.5 x (Str + End)) + m x (level - 1), m = 3 + 2
  combat / + 1 stealth specialization + 1 if Endurance is favoured; skills = 5 + race bonus + 25 major / 10 minor + 5
  same specialization + round((level - 1) x (1.0 major-or-minor else 0.1, + 0.5 same specialization)), max 100.
  Magicka and fatigue then come from `MwDerived` (verify the NPC magicka path: `recalculateMagicka` on spawn).
- **Creature**: CREA attributes, health, magicka, fatigue, level; its three skill values (combat, magic, stealth) fill
  every skill of that specialization (`mwclass/creature.cpp` `getSkill`, verify).
- **Player, chargen** (`mwmechanics/mechanicsmanagerimp.cpp` `buildPlayer` 111-221): attributes = race base; skills = 5
  + race bonus; class: +10 favoured attributes, +10 minor, +25 major, +5 same specialization; race and birthsign powers
  (spells: effect spec); `updateHealth` = floor(0.5 x (Str + End)) (`npcstats.cpp` ~251); all currents = max.
- **Player, dev start** (chargen skipped, scaffold `SkipChargen`): the `player` record's NPDT values as today's HUD,
  else `buildPlayer` with the record's race and class.

## Formulas (`MwDerived`, `OpenMwSkillUse`, `OpenMwLeveling`)

- **Magicka max** = (`fPCbaseMagickaMult` player / `fNPCbaseMagickaMult` NPC + 0.1 x Fortify Maximum Magicka
  magnitude) x Int, current keeps its ratio (`creaturestats.cpp` `recalculateMagicka` 397-418).
- **Fatigue max** = max(0, Str + Wil + Agi + End), ratio kept (`creaturestats.cpp` `setAttribute` ~123-153).
- **Health** stored: chargen value, + End x `fLevelUpHealthEndMult` per level (non-retroactive, as vanilla).
- **Capacity** = Str x `fEncumbranceStrMult` (`npc.cpp` `getCapacity` 1002-1011); werewolves carry 0 encumbrance.
- **Fatigue term** = `fFatigueBase` - `fFatigueMult` x (1 - cur/max), 1 when max floors to 0 (`creaturestats.cpp`
  `getFatigueTerm` 39-53). Read by classic combat and the mechanics (barter, persuasion, alchemy).
- **Regen** (per second, when fatigue < base): `fFatigueReturnBase` + `fFatigueReturnMult` x End (`actors.cpp`
  `calculateRestoration` 939-962). Skipped for `Combatant`s (combat's regen owns stamina: stats.md "Consumers").
- **Rest** per hour (`actors.cpp` 106-120, `restoreDynamicStats` 869-937): sleeping: health 0.1 x End, magicka
  `fRestMagicMult` x Int (not while Stunted Magicka); always: fatigue 3600 x (`fFatigueReturnBase` + `fFatigueReturnMult`
  x (1 - min(1, normalised encumbrance))) x `fEndFatigueMult` x End.
- **Skill use** (`files/data-mw/scripts/omw/playerskillhandlers.lua` 21-38, 160-172; C++ twin `npcstats.cpp`
  `getSkillProgressRequirement` 187-212): gain = SKIL `use_value[use_kind]` x scale; progress += gain / ((base + 1) x
  type x spec), type = `fMajorSkillBonus` / `fMinorSkillBonus` / `fMiscSkillBonus`, spec = `fSpecialSkillBonus` when the
  skill's specialization is the class's, else 1; raise at progress ≥ 1. No gain at base 100 or as a werewolf.
- **Skill raise** (same file 40-110): base +1 (cap 100); `level.progress` += `iLevelUpMajorMult` / `iLevelUpMinorMult`
  (misc 0); `levelup.attr.<governing>` += `iLevelUpMajorMultAttribute` / `iLevelUpMinorMultAttribute` /
  `iLevelupMiscMultAttriubte` (sic); `levelup.spec.<spec>` += `iLevelupSpecialization`; usage resets progress to 0
  (trainer and book keep it); jail: -1 except Security and Sneak, no tallies.
- **Level up** (`mwgui/levelupdialog.cpp` 240-262, `npcstats.cpp` `levelUp` 224-249, `getLevelupAttributeMultiplier`
  258-272): ready at `level.progress` ≥ `iLevelUpTotal`; 3 distinct attribute picks, each + `iLevelUpNNMult` with NN =
  clamp(tally, 0, 10) (1 when the tally is 0 or the GMST is missing), cap 100; then progress -= `iLevelUpTotal` (≥ 0),
  health base and current += End x `fLevelUpHealthEndMult` (current ≥ 1), level + 1, attribute tallies cleared (verify
  whether specialization tallies clear too).

GMST defaults expected in Morrowind.esm (the S2 test prints the install's): fFatigueBase 1.25, fFatigueMult 0.5,
fPCbaseMagickaMult 1, fNPCbaseMagickaMult 2, fLevelUpHealthEndMult 0.1, iLevelUpTotal 10, fMajorSkillBonus 0.75,
fMinorSkillBonus 1, fMiscSkillBonus 1.25, fSpecialSkillBonus 0.8, fEncumbranceStrMult 5, fFatigueReturnBase 2.5,
fFatigueReturnMult 0.02, fEndFatigueMult 0.04, fRestMagicMult 0.15, iLevelUp01-04Mult 2, 05-07 3, 08-09 4, 10 5 (verify).

## Effect mapping (input to the effect spec)

From `mwmechanics/spelleffects.cpp` (helpers 66-140, apply ~700-920, remove ~1110-1200): Fortify Attribute/Skill =
`AddModifier` (Permanent layer for abilities, else Temporary); Drain Attribute/Skill = `Damage` on start + `Restore` of
the same magnitude on end (so Restore spells cancel drains, as OpenMW); Damage Attribute/Skill = `Damage` capped at
modified; Restore = `Restore`; Restore/Damage Health-Magicka-Fatigue = `PoolAdjust` (damage never below zero); Drain
Health-Magicka-Fatigue = `PoolAdjust { below_zero: true }`, inverted on end; Fortify Health-Magicka-Fatigue =
`PoolAdjust { above_max: true }`, or `PoolModMax` for abilities; Absorb = target and caster calls in one step.

## Factions, reputation, bounty, disposition

- Membership per FACT id: rank (0-9, -1 = not a member), faction reputation, expelled (`npcstats.cpp` 82-146).
  Promotion check `hasSkillsForRank` (341-380): the faction's skills sorted, best ≥ rank's primary skill value, the next
  two ≥ its favoured skill value; attribute requirements (FACT rank data) checked by the `RankRequirement` filter
  (`mwdialogue/filter.cpp`, verify).
- `reputation` clamps to 0..=255 (`npcstats.cpp` ~322-333). `bounty` ≥ 0; writers (crime spec) use `iCrimeTresspass`,
  `iCrimePickPocket`, `iCrimeAttack`, `iCrimeKilling`, `fCrimeStealing` x value, `iWereWolfBounty`
  (`mechanicsmanagerimp.cpp` 1227-1262, 1468, 1973).
- Disposition inputs for `DispositionModel` (`getDerivedDisposition` 475-560): same race `fDispRaceMod`, personality
  `fDispPersonalityMult` x (Per - `fDispPersonalityBase`), faction (`fDispFactionRankMult` x rank +
  `fDispFactionRankBase`) x `fDispFactionMod` x reaction, - `fDispCrimeMod` x bounty, disease, weapon drawn. Whether
  the OpenMW formula becomes the model is the persuasion lane's call (open question 2).

## Consumers

- **Combat** (`mw_player_bevy/src/combat.rs`, `mw_combat::fighter`): `Combatant` health = `health`, stamina = `fatigue`
  x `STAMINA_PER_FATIGUE` (0.5, combat-adapter.md "Health, stamina, fatigue"); differences back x 2. `PlayerCombat`'s
  record health and fatigue go. Death: `Depleted(health)` (< 1) drives the existing death hook.
- **HUD**: bars from roles; `PlayerVitals` (`hud.rs`) removed, magicka finally live.
- **Dialogue**: `MwView::function` for `PcLevel`, `PcHealth`/`PcMagicka`/`PcFatigue`(`Percent`), the 8 attribute and 27
  skill `Pc*` functions, `PcReputation`, `PcCrimeLevel` (= bounty), `PcExpelled`, speaker `Level`/`Reputation`/
  `HealthPercent`, `RankRequirement`, `FactionRankDifference`; `player_rank` from groups. `PlayerStatsPlaceholder` goes.
- **MWScript** (`mwscript/statsextensions.cpp` 81-490, 610-1100): `Get<Attr>`/`Get<Skill>` = modified; `Set<..>` = base;
  `Mod<..>` = base + delta; `Get<Dyn>` = current (magicka ≥ 0); `Set<Dyn>` = base, current = max (both flags);
  `Mod<Dyn>` = base + delta (≥ 0 except fatigue) and current + delta (both flags); `ModCurrent<Dyn>` = current + delta,
  fatigue may go below 0 and knocks down at ≤ 0; `Get<Dyn>GetRatio`; `GetLevel`/`SetLevel`; `Get/Set/ModReputation`;
  `GetPCRank`, `PCRaiseRank`, `PCLowerRank`, `PCJoinFaction`, `PCExpell(ed)`, `PCClearExpelled`, `Get/Set/ModPCFacRep`,
  `RaiseRank`/`LowerRank` (NPC); `Get/Set/ModPCCrimeLevel` = bounty. `dagoth_ur_1` `ModHealth` workaround kept (OpenMW
  252-300). `functions.txt` statuses move to `live`.

## Use sources (S9)

`UseSkill` where the action happens: weapon hit, armor hit taken (by slot), block, athletics and acrobatics (movement),
security, sneak, successful casts; alchemy, enchant, mercantile, speechcraft, armorer with their mechanics. Port the
call sites and `use_kind`s from OpenMW (grep `skillUsageSucceeded` in `mwmechanics/`, verify). Modern combat: landed hits.

## Tests (exact, synthetic records unless marked)

- Autocalc: a level-10 combat-class NPC with given race/class: attributes, skills, health against the formulas above.
- Chargen: Altmer (Fortify Maximum Magicka 15 ability) with Int 50 → magicka 125; Breton (5) → 75; fatigue = the sum;
  health = floor(0.5 x (Str + End)).
- Fortify Strength 10 on 50/200 fatigue → max +10, current keeps the ratio.
- Skill use: Long Blade major, combat class, base 30 → requirement 31 x 0.75 x 0.8 = 18.6; uses to raise with
  `use_value` 1 = 19; raise adds 1 level progress and 1 Strength tally (vanilla GMSTs).
- Level up after 10 raises with tallies Str 5, End 2, Luck 0 → +3, +2, +1; health + 0.1 x new End.
- Rest 8 h sleeping: health + 8 x 0.1 x End; magicka + 8 x 0.15 x Int.
- `hasSkillsForRank` edge cases; MWScript op semantics per bullet above.
- Real install (`#[ignore = "real install"]`): every NPC and creature initialises with attributes and skills in 0..=100,
  health > 0; prints the GMST snapshot.

## Open questions (owner)

1. Morrowind's level-up multipliers (min-maxing by training misc skills) and non-retroactive health in the modern
   profile, or a modern `LevelingModel` from the start? Default: OpenMW first (stats.md question 1).
2. Derived disposition: port OpenMW's `getDerivedDisposition` as the `DispositionModel` (the research suggests it; the
   model's comment says feel system, owner 2026-10-25)? Default: port it in the persuasion slice, keep the seam.
