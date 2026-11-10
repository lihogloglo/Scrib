# Crime adapter: Morrowind crime, law, stealth, locks and traps on `sf_crime` (`mw_crime`)

Status: spec written 2026-11-10; nothing built. Framework: [crime.md](../../../superfargoth/docs/specs/crime.md)
(`Ownership`, `StolenLedger`, `OffenceDef`, `assess`, `LockRules`, `DetectionRules`, arrest and jail flow). Also
[stats-adapter.md](stats-adapter.md) ("Factions, reputation, bounty, disposition": bounty is the `bounty` counter,
`MwGmst`, fatigue term, `UseSkill`), [services-adapter.md](services-adapter.md) (A7 stolen-goods veto waits for
the ledger here), [effects-adapter.md](effects-adapter.md) (trap spells, Open spell), the AI lane
([ai.md](../../../superfargoth/docs/specs/ai.md) perception) and
[research](../research/openmw-mechanics-2026-11-05.md). ROADMAP 2.8; owner decision 4 (2026-11-05: content
mechanics are ported from OpenMW).

Ported from OpenMW `ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f`; paths under `apps/openmw/` unless noted, line
numbers at that commit, read from the raw sources 2026-11-10. Abbreviations: **MM** = `mwmechanics/mechanicsmanagerimp.cpp`,
**ACT** = `mwmechanics/actors.cpp`. Base URL:
`https://github.com/OpenMW/openmw/blob/ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f/`.

<!-- agent-summary -->
> **Agent summary.** New Bevy-free crate `mw_crime` implements the framework rule traits with OpenMW's numbers:
> `isOwned` (owner / faction + rank / global), the stolen-items map, the offence table (trespass, pickpocket, assault,
> murder, theft, sleeping in an owned bed), `commitCrime` witness logic, `reportCrime` per-witness disposition and
> fight terms, guard pursuit at `iCrimeThreshold`, pay / jail / confiscate, `pickLock`, `probeTrap`, pickpocket odds
> and `awarenessCheck`. **Quirks to keep in the classic column**: a bounty is added whenever any non-reporting-exempt
> actor in `fAlarmRadius` has Alarm >= 100, seen or not; stolen entries never shrink on sale; pickpocket and
> awareness use integer rolls `0..=99`; jail skill loss lives in a Lua handler. Bounty is already a stat and
> `GetPCCrimeLevel`/`Set`/`Mod` already run (S7); `PayFine`, `PayFineThief`, `GoToJail` are `later` and land here.
> Read "Ownership", "Offences", "Guards and jail", "Locks", "GMSTs", "Options", "Tests", "Build plan".
<!-- /agent-summary -->

## Goal

Same crimes, same bounties, same reactions as OpenMW for the same records and GMSTs, through the modern host
(events, fixed tick, per-player state), with modern-first switches in Options. Non-goals: werewolf reveal and
`iWereWolfBounty` (the rule is listed so the werewolf slice calls `OffenceAttempt`; MM 1936-1976), the guard
dialogue content (it is in Morrowind.esm; this spec supplies the globals and script functions it reads),
perception (AI lane), combat hit rules.

Each rule's `In` holds every number it reads (GMSTs, roll, distances); rolls come from the host's seeded roll.

## What exists in the repo (2026-11-10)

- Bounty counter: `mw_stats` key `bounty`, `mw_rpg::script_stats` S7: `GetPCCrimeLevel`, `SetPCCrimeLevel`,
  `ModPCCrimeLevel` (floor 0) are live; they do **not** call `recordCrimeId` when the total reaches 0
  (`statsextensions.cpp` 463-493 do), listed in the module docs: closed by slice A2 below.
- `functions.txt` "factions, crime": `PayFine`, `PayFineThief`, `GoToJail` = `later`; `GetPCInJail` = `stub`.
  `Lock`, `Unlock`, `GetLocked` listed `live` in the lock family (semantics in "Locks", check them against it);
  `GetPCSneaking` live; `GetDetected` stub (AI).
- `mw_rpg_bevy::dialogue::DerivedDisposition::input`: `crime_modifier` is a named TODO (here: NPC crime modifier,
  A5). `derived_disposition` already subtracts `fDispCrimeMod x bounty` (MM 544).
- Services A7 (stolen-goods veto): waits for `StolenLedger` (A1).
- Not in the repo: reference ownership, lock, key and trap fields; NPC `Alarm`/`Fight` and `Guard` class as rule inputs.

## Data on references (A0)

ESM reference subrecords (`components/esm3/cellref.cpp`): `ANAM` owner id (88-90), `BNAM` owner global variable
(91-96), `CNAM` owning faction (97-99), `INDX` faction rank (100-102), `FLTV` lock level (124-126), `KNAM` key id
(127-129), `TNAM` trap spell id (130-132); `NAM0` skipped. A level `0` is stored as `INT_MAX` (`ZeroLock`) so
"locked at level 0" survives (13, 162-163, 236-238). **`isLocked` is derived on load**: from a content file
`locked = (key != empty || level > 0)`, from a save `level > 0` (159-163). **Unlock negates the level**
(`mwworld/cellref.cpp` 319-329: `lock(level)` sets level and locked; `unlock()` stores `-level` and unlocked), so
an unlocked object that was ever locked has a nonzero level. Our `LockState { level, locked, key }` stores the
level as positive and `locked` separately; `was_locked = level != 0` reproduces this exactly.

Records: NPC AI settings Fight/Flee/Alarm/Hello (`AiSetting::Alarm` base read; the `$AISTATS` in `functions.txt`),
class id `guard` (`isClass(.., "guard")`, case-insensitive; MM 1397, ACT 1172), primary faction, faction ranks
(stats-adapter). Adapter `MwOwnership` maps `ANAM/BNAM/CNAM/INDX` to the framework's `Ownership`.

## Ownership (A1)

`isOwned(player, target) -> (owned, victim)` (MM 78-106):

- `owner = ANAM`; owned iff `owner != empty && owner != "Player"` (case-insensitive id compare).
- Faction-owned iff `CNAM != empty` and the actor is an NPC and (not a member of that faction **or** member rank
  `< INDX`).
- If `BNAM` names a global with a nonzero integer value: both flags cleared (the global lifts ownership).
- `victim` = the owner actor looked up in the world when `ANAM` is set; owned or faction-owned counts.

(The Lua `isAllowedToOpen` in `files/data/scripts/omw/crimes.lua` 5-33 differs: rank 0 means "not in faction"
there; we port the C++ above.) `isAllowedToUse(player, target) -> (allowed, victim)` (MM 886-925), in order:
empty target or self: allowed; an **unlocked, untrapped door**: allowed; targets without a tooltip: allowed;
activators that are not beds (script name starts "Bed"): allowed; for an NPC target: dead or in combat: allowed,
knocked down **or the player sneaking**: not allowed (this is the pickpocket gate), else allowed; owned: not
allowed; the evidence chest `stolen_goods` is never takeable. Maps to framework `use_allowed` with `never_take =
["stolen_goods"]`.

### Stolen items (A1)

`itemTaken(player, item, container, count, alarm)` (MM 1079-1135; callers `actiontake.cpp` 37,
`mwgui/containeritemmodel.cpp` 267, `mwgui/inventorywindow.cpp` 855, `mwgui/pickpocketitemmodel.cpp` 113 with
`alarm=false`): player only; allowed (above) returns. The owner key is: for an actor container its `refId`; else
the container's (or the item's) `ANAM`, and if empty its `CNAM` as a faction (`second = true`). Gold is never
recorded. A non-gold item is recorded (`stolen[item id][owner] += count`) iff there is no victim actor or the
victim actor is alive and `count > 0`. If `alarm`, `commitCrime(theft, faction = CNAM, value)`; `value = count`
for gold else `count x item value`. (`actiontake.cpp` passes `count x gold value` for gold.) Taking from a dead
actor's corpse is always allowed (`containeritemmodel.cpp` 256-258).
`isItemStolenFrom(item, merchant)` (MM 982-1002): the (merchant id, false) key, or the merchant's primary
faction as (faction, true). `confiscateStolenItemToOwner` (MM 1004-1046; callers `tradewindow.cpp` 438,
`mwgui/enchantingdialog.cpp` 364): victim's faction replaces the owner key when the item's `CNAM` equals it;
decrement `min(count, stolen)`, erase at 0, move the items to the victim, then `commitCrime(theft, item faction,
value x moved)`. `confiscateStolenItems(player, chest)` (MM 1048-1077): per stolen item, walk owners in map
order consuming `min(remaining, owned)`; move `count - leftover` to the chest; then **lock the chest at level 50**
(`// TODO: unhardcode the locklevel`). Map ordering is `(RefId, bool)` pair order; ours is `BTreeMap`.

Classic: no `reconcile` (stale entries stay, MM keeps counts after sale). Modern default: `reconcile` on every
inventory change (framework Q3).

## Offences (A2)

Types: `OT_Theft`, `OT_Assault`, `OT_Murder`, `OT_Trespassing`, `OT_SleepingInOwnedBed`, `OT_Pickpocket`
(`mwbase/mechanicsmanager.hpp`). They become `OffenceDef` records `mw.crime.<name>`; `Trespassing` and
`SleepingInOwnedBed` share a row.

| Offence | Bounty (MM 1233-1271) | Disposition, witness / victim | Fight, witness / victim (MM 1287-1309) |
|---|---|---|---|
| Trespass, SleepInBed | `iCrimeTresspass` | `iDispTresspass` / same | `iFightTrespass` / same |
| Pickpocket | `iCrimePickPocket` | `fDispPickPocketMod` / same | `iFightPickpocket` / `iFightPickpocket x 4` |
| Assault | `iCrimeAttack` | `iDispAttackMod` / `fDispAttacking` | `iFightAttacking` / `iFightAttack` |
| Murder | `iCrimeKilling` | `iDispKilling` / same | `iFightKilling` / same |
| Theft (`arg` = value) | `max(1, int(arg x fCrimeStealing))` | `fDispStealing x arg` / same | `int(fFightStealing) x arg` / same |
| Other (script) | `arg` | 0 | 0 |

(`fFightStealing` is a float GMST read through `getInteger()`: truncation kept, MM 1307.) Murder first
calls `notifyMurder` on the victim (MM 1233-1234).

## `commitCrime` (A2; MM 1137-1202)

1. Player only (our `OffenceAttempt.offender` is the player; NPC crimes are not tracked). Assault forces
   `victimAware = true`.
2. Candidates: actors within `fAlarmRadius` of the player plus the victim even beyond it. Followers and allies of the
   player (`getActorsSidingWith`, recursive) are excluded as reporters.
3. `canReportCrime(actor)` (MM 1204-1225): not the player, is an NPC, alive, not already in combat **with the victim**,
   not knocked down, not a follower of the player.
4. A candidate **saw** it iff: `(actor == victim && victimAware)`, or `(type == Murder && actor != victim)` (hearing
   suffices), or `(LOS(player, actor) && awarenessCheck(player, actor))` (the AI seam: `WitnessQuery`). A theft or
   pickpocket witness says `thief` (dialogue bark, `say`).
5. If anyone saw: `reportCrime`. Else if `Assault` with a victim: a `guard` victim not already pursuing reports with
   an empty faction; otherwise `startCombat(victim, player)` (the victim fights back without a bounty).
Returns `crimeSeen`.

## `reportCrime` (A2; MM 1227-1507)

Per witness the framework's `ReactionRules` is this port. Let `alarm = Alarm base`, `alarmTerm = alarm / 100`,
`isVictim`, `isGuard`, `cur = derived disposition`, `disp` the offence's disposition (victim value for the victim).

- **Reported** (bounty added) iff **any** reportable candidate in radius, seen or not, has `alarm >= 100`
  (MM 1316-1325; trespassers get the `intruder` bark). Quirk: not tied to who saw it.
- **Disposition modifier** (integer cast, MM 1355-1390): Theft: `int(disp x alarmTerm)`. Pickpocket: guard with
  `alarm >= 100`: `int(disp)`; victim guard: permanent `int(disp x alarmTerm)`; victim other: permanent `int(disp)`.
  Assault: victim non-guard: permanent `int(disp)`; `alarm >= 100`: `int(disp)`; victim guard: permanent
  `int(disp x alarmTerm)`; else `int(disp x alarmTerm)` **only if hostile**. Murder, trespass: 0. A change is clamped
  to `[-cur, 100 - cur]`; permanent edits the base disposition, others add to the NPC's `crimeDispositionModifier`
  (reset by pardon, MM 478, ACT 1217).
- **Guard with `alarm >= 100`**: set Alarmed, push `AiPursue(player)` if not pursuing (MM 1397-1405). Other witnesses:
  pickpocket and not victim: `alarmTerm = 0`; pickpocket victim with `alarm = 0`: `alarmTerm = 1`;
  `fight = (isVictim ? fightVictim : fight) + (50 - disp) x fFightDispMult + (iFightDistanceBase - fFightDistanceMultiplier x d)`
  (`mwmechanics/combat.cpp` 567-596), times `alarmTerm`; cap so `Fight + fight <= 100`, floor 0; if `Fight + fight >= 100`
  start combat, set the base Fight to `Fight + int(fight)`, set Alarmed.
- Any witness with a change gets `crimeId = newCrimeId()` (the pardon key, framework `CrimeLedger`).
- If reported: `bounty = max(0, bounty + amount)`; expulsion: a victim NPC with a primary faction the player belongs
  to expels the player (`expell(faction, true)`), else the offence's named faction; an assault victim not yet fighting
  starts combat with the player and gets the crime id (MM 1469-1505).

`actorAttacked` (MM 1509-1558): `canCommitCrimeAgainst(target, attacker)` (NPC, not aggressive toward the attacker,
not engaged, not pursuing, not a werewolf, no Vampirism, MM 1560-1568) then `Assault`; then fight back and force
friendly followers into combat. `actorKilled` (MM 1570-1601): player or their followers only; murder iff the victim
could be committed against, **or** an assault crime id exists on the victim (the player attacked first); creatures
are not murder ("animal rights" TODO).

## Guards, pay and jail (A3)

**Sight rule** (`ACT::updateCrimePursuit` 1153-1225, every actor every frame): a `guard` not pursuing, not in
combat, not under Calm Humanoid, with `bounty >= iCrimeThreshold` and LOS and `awarenessCheck(player, guard)`:
if `bounty >= iCrimeThreshold x iCrimeThresholdMultiplier` start combat (and keep the player as hit target), else
push `AiPursue`; set Alarmed and a new crime id (1177-1198). Werewolf players are skipped. Witnesses whose crime id
`<= paidCrimeId` calm down: guards stop pursuit, everyone stops combat, Alarmed/attacked reset, base Fight restored,
crime disposition modifier 0, id -1 (1203-1223; also on dialogue open, `mwgui/dialogue.cpp` 797-801). Maps to
`LawRules::stance` (thresholds) plus the AI behaviour; ours is event-driven, not a per-frame poll.

**Arrest** (`aipursue.cpp` 30-79): the pursuing guard stops when the player's bounty is 0, the target is dead or
hidden (magical hiding needs an awareness check); it paths to the player (tolerance 100 units), and when reached with
LOS pushes the dialogue GUI mode (the arrest greeting). The arrest choices are **content dialogue in Morrowind.esm**
driven by globals and result scripts. `World::updateDialogueGlobals` (`mwworld/worldimp.cpp` 3490-3512) writes:
`PCHasCrimeGold = bounty <= gold`, `CrimeGoldDiscount = discount`, `PCHasGoldDiscount = discount <= gold`,
`CrimeGoldTurnIn = turnIn`, `PCHasTurnIn = turnIn <= gold`, with `discount = int(bounty x fCrimeGoldDiscountMult)`,
`turnIn = int(bounty x fCrimeGoldTurnInMult)`, each floored to 1 when `bounty > 0` (global names spelled as in
`Globals`, **unverified**, check against the esm).

| Script | Effect (`mwscript/miscextensions.cpp`) |
|---|---|
| `PayFine` | 1351-1373: bounty 0, `confiscateStolenItems(player)` into the nearest prison's `stolen_goods` chest, `recordCrimeId`, holster |
| `PayFineThief` | 1375-1382: bounty 0, `recordCrimeId` (no confiscation) |
| `GoToJail` | `World::goToJail` (`worldimp.cpp` 3540-3573): first call: `days = max(1, bounty / iDaysinPrisonMod)` (integer), bounty 0, `recordCrimeId`, confiscate; the **second** call (next unpaused frame, so the player can read the text) closes dialogue, holsters, opens the jail screen |
| `GetPCInJail` | `isPlayerInJail` |
| `GetPCCrimeLevel`, `Set`, `Mod` | exist (S7); `Set`/`Mod` reaching 0 call `recordCrimeId` (`statsextensions.cpp` 463-493) |

**Jail screen** (`mwgui/jailscreen.cpp` 35-90): fade out 0.5 s, `teleportToClosestMarker(player, "prisonmarker")` (the
marker's `destCell` names the prison; `confiscateStolenItems` needs `stolen_goods` in that cell, else a warning,
`worldimp.cpp` 3515-3537), progress bar over 100 ticks, then fade in, `rest(days x 24, sleep)`,
`advanceTime(days x 24)`, skip corprus worsenings, and the Lua event `jailTimeServed(player, days)`. **Skill change
in jail**: the engine does it in Lua through `SkillProgression` with source `jail`: -1 on every skill except Sneak and
Security (documented, `files/data/scripts/omw/skillhandlers.lua` 131, 207); the handler that fires per served day
is not in this commit's builtin scripts (grep finds `_onJailTimeServed` only as an engine hook, `mwlua/localscripts.hpp`
111), **unverified**: Morrowind's own rule is a random skill -1 per day and +1 Sneak/Security. `JailRules::served`
implements the documented one (Options).

## Sleeping in a bed (A3)

`sleepInBed` (MM 927-952): werewolf and enemies-nearby refuse; an owned bed is `commitCrime(SleepingInOwnedBed)`: seen means `sNotifyMessage64` and the sleep is refused, unseen proceeds.

## Pickpocket (A4)

Gate: activating an NPC while sneaking opens the pickpocket model (`isAllowedToUse` false for sneaking, above). Rule
(`mwmechanics/pickpocket.cpp` 18-80): `mod(actor, add) = (add + 0.2 Agility + 0.1 Luck + Sneak) x fatigueTerm`,
`x = mod(thief, 0)`, `y = mod(victim, valueTerm)`, `t = 2x - y`, `m = pcSneak / iPickMinChance`. Caught iff:
if `t < m`: `roll > int(m)`; else `roll > int(min(iPickMaxChance, t))` (integer roll `0..=99`). Taking an item:
`valueTerm = 10 x fPickPocketMod x (item value x count)`; closing the window without taking: `valueTerm = 0`
(`finish`). A catch is `commitCrime(Pickpocket, victimAware = true)` and closes the window; a success trains Sneak
(`Sneak_PickPocket`) and records the item as taken with no alarm (`pickpocketitemmodel.cpp` 85-130). A knocked-down
victim skips the check (118-120). "Reverse pickpocket" is refused (`onPlaceItem`, 80).

## Locks and traps (A5)

**Activation order**, same for container (`mwclass/container.cpp` 140-208) and door (`mwclass/door.cpp` 143-237): if
locked **and** the player carries the `KNAM` key: message `<key name> sKeyUsed`, unlock, and if trapped, **disarm**
the trap (sound `Disarm Trap`); then if unlocked or keyed: a trapped object fires its trap (`ActionTrap`, sound
`Disarm Trap Fail`) before opening, a door teleports only when untrapped; if still locked: `LockedChest`/`LockedDoor`
sound and no open. Tooltips show `sLockLevel: <n>` when locked, `sUnlocked` when level != 0 and unlocked, `sTrapped`
when trapped (`container.cpp` 250-259, `door.cpp` 282-291). A key use is **not** an unlock attempt (no trespass).

`Security::pickLock` (`security.cpp` 24-70): `level <= 0` or no tooltip: nothing; the lockpick has `uses` (item health,
0: nothing). `x = (0.2 Agility + 0.1 Luck + Security) x quality x fatigueTerm + fPickLockMult x level`; `unlockAttempted`
is emitted first; `x <= 0`: `sLockImpossible`; else success iff `roll <= x` (`roll0to99`): unlock, `sLockSuccess`, sound
`Open Lock`, skill use `Security_PickLock`; else `sLockFail`. Either way `uses -= 1`, removed at 0.
`probeTrap` (`security.cpp` 72-128): `x = ((0.2 Agility + 0.1 Luck + Security) + fTrapCostMult x trapSpellCost) x quality x
fatigueTerm`, `x <= 0`: `sTrapImpossible`, success iff `roll <= x`: trap cleared, `Disarm Trap`, `Security_DisarmTrap`,
else `sTrapFail`; uses -1. `trapSpellCost` is `calcSpellCost` of the trap SPEL (`mw_effects::spell_cost`).

`unlockAttempted` (MM 954-966): if owned (`isOwned`, either flavour) **and** (`level != 0` or `locked` or trapped): `commitCrime(Trespassing,
faction)`. Callers: pick, probe (`security.cpp` 55, 105) and the **Open** spell effect on a lockable target
(`mwmechanics/spelleffects.cpp` 1287, with the player as actor). `ActionTrap` (`mwworld/actiontrap.cpp` 10-35): the player
beyond `getMaxActivationDistance()` of the object (telekinesis) detonates the trap spell at the object; else it is cast
**on the activator** at the activator's position; then the trap is cleared. Scripts: `Lock [level]`: level defaults to the
current one, or 100 if never set (`miscextensions.cpp` 332-357); `Unlock`: only if locked (363-372); `GetLocked`: `locked`.
`mw_rpg` implements these on `LockState`.

## Stealth and awareness (A6)

`isSneaking` (MM 372-382): the Sneak stance and (the character controller reports sneaking, or the actor is airborne
and not swimming or flying). Ours: the `Sneak` mode flag (action.md) is the stance; the airborne clause is classic.
`awarenessCheck(ptr, observer)` (MM 1603-1674), integer rolls, **dead or disabled observers never notice**:

```
sneakTerm = 0; if sneaking: sneakTerm = fSneakSkillMult*Sneak + 0.2*Agility + 0.1*Luck + bootWeight*fSneakBootMult
            (bootWeight = boots slot weight, NPCs on the ground only)
distTerm  = fSneakDistanceBase + fSneakDistanceMultiplier * distance
x         = sneakTerm * distTerm * fatigueTerm + Chameleon;   if Invisibility > 0: x += 100
obsTerm   = obsSneak + 0.2*obsAgility + 0.1*obsLuck - Blind
y         = obsTerm * obsFatigueTerm * (angle(observer facing, to target) > 90 deg ? fSneakNoViewMult : fSneakViewMult)
noticed   = roll0to99 >= x - y          (cached roll variant for repeat checks)
```

(So `concealment = x - y`; a character not sneaking has `x = Chameleon` only.) Maps one to one to
`DetectionRules::concealment`; the classic model compares the roll, the modern Meter derives a fill rate from it.
The `useCache` variant (`creaturestats` `awarenessRoll`, one roll per actor per interval) is the AI lane's to keep.

## GMSTs read (expected Morrowind.esm values; U = from memory, **unverified**, check in the `#[ignore = "needs Morrowind.esm"]` test)

| GMST | Use | Expected |
|---|---|---|
| `iCrimeThreshold`, `iCrimeThresholdMultiplier` | guard pursuit / attack | 1000, 10 (U) |
| `fCrimeGoldDiscountMult`, `fCrimeGoldTurnInMult` | dialogue globals | 0.9, 0.5 (U) |
| `iCrimeKilling`, `iCrimeAttack`, `iCrimePickPocket`, `iCrimeTresspass`, `fCrimeStealing` | bounties | 1000, 40, 25, 5, 1 (U) |
| `iDaysinPrisonMod` | jail days | 100 (U) |
| `fAlarmRadius` | witness radius (units) | 4000-5000 (U) |
| `iDispTresspass`, `fDispPickPocketMod`, `iDispAttackMod`, `fDispAttacking`, `iDispKilling`, `fDispStealing` | disposition hits | negative (U) |
| `iFightTrespass`, `iFightPickpocket`, `iFightAttacking`, `iFightAttack`, `iFightKilling`, `fFightStealing` | fight terms | U |
| `fFightDispMult`, `iFightDistanceBase`, `fFightDistanceMultiplier` | fight bias | U (~0.2, ~50, ~0.005) |
| `fDispCrimeMod` | disposition per bounty (read already) | U (~0.02) |
| `iPickMinChance`, `iPickMaxChance`, `fPickPocketMod` | pickpocket | 5, 75, 0.3 (U) |
| `fPickLockMult`, `fTrapCostMult` | lock and trap | -1, -0.15 (U) |
| `fSneakSkillMult`, `fSneakBootMult`, `fSneakDistanceBase`, `fSneakDistanceMultiplier`, `fSneakViewMult`, `fSneakNoViewMult` | awareness | U (~0.75, ~0.5, ~0.5, ~0.002, ~1.5-3, ~0.5-1) |
| `fFatigueBase`, `fFatigueMult` | fatigue term | 1.25, 0.5 |
| `iWereWolfBounty` | werewolf (later) | 1000 (U) |
| `sNotifyMessage2/49/64`, `sKeyUsed`, `sLock*`, `sTrap*`, `sUnlocked`, `sLockLevel`, `sTrapped`, `sWerewolfRefusal` | strings | from content |

## Options (classic vs modern; `MwCrimeOptions` feed rule inputs)

| Option | Modern default | Classic (OpenMW) |
|---|---|---|
| Stolen ledger | reconciled on inventory change (no stale entries) | never shrinks except on pay (1079-1135) |
| Who reports | witnesses who saw or heard, with `alarm >= 100` | any reporter in radius (1316-1325) |
| Detection model | classic roll for NPCs; Meter + sneak eye for guards once AI8 exists | roll `0..=99` |
| Prompt | owned/stolen marker on the crosshair and item rows | none (item names only) |
| Jail time | skill loss as documented (-1 each, except Sneak/Security), shown in the release message | same; handler unverified |
| Co-op jail | game time skipped for the jailed player only | n/a |

## MWScript mapping

TODO (unfinished 2026-11-10): full mapping. Short form: `PayFine`/`PayFineThief`/`GoToJail`/`GetPCInJail` route to crime commands (A3); `Lock`/`Unlock`/`GetLocked` to `LockState` (A5); `Set/ModPCCrimeLevel` record the crime id at 0 (A2); dialogue globals are written before each dialogue open.

## Tests (exact; synthetic records and GMSTs unless noted)

GMSTs for tests: `fCrimeStealing` 1, `iCrimeThreshold` 1000, `iCrimeThresholdMultiplier` 10, `iDaysinPrisonMod` 100,
`fPickLockMult` -1, `fTrapCostMult` -0.15, `iPickMinChance` 5, `iPickMaxChance` 75, `fPickPocketMod` 0.3,
`fSneakSkillMult` 0.75, `fSneakBootMult` 0.5, `fSneakDistanceBase` 0.5, `fSneakDistanceMultiplier` 0.002,
`fSneakViewMult` 1.5, `fSneakNoViewMult` 0.5, `fFightDispMult` 0.2, `fFatigueTerm` 1.

- **Ownership**: owner "fargoth" owned; owner "player" free; faction "mages" rank 3: member at 2 owned, at 3 free,
  non-member owned; `BNAM` global 1 frees both; unlocked untrapped door is allowed, locked or trapped owned door not;
  sneaking at an NPC denies, in combat or dead allows; `stolen_goods` never takeable.
- **Stolen**: take 3 swords from a Fargoth chest: ledger `(sword, fargoth) = 3`; gold never; dead owner: not
  recorded; faction chest keys the faction; held 2 of `{fargoth 2, guild 1}`: confiscate 2, remainder `{guild 1}`,
  chest locked at 50; merchant match by id or primary faction; reconcile (modern) to held 1 leaves `{guild? ...}` oldest
  removed first (assert the exact map).
- **Offence table**: theft arg 150 -> 150; arg 0 -> 1; `fCrimeStealing` 0.5 and arg 1 -> 1; pickpocket 25 etc.
- **Witness**: murder reported without LOS; assault forces victim aware; follower and dead and knocked-down excluded;
  no witness plus guard victim -> report, else victim combat.
- **Reported**: alarm 100 beyond sight still adds bounty (classic); modern requires a witness; bounty never below 0.
- **Disposition**: theft arg 100, `fDispStealing` -0.5, alarm 50: `int(-50 x 0.5) = -25`; modifier -80 with cur 30
  clamps to -30; permanent victim edits base.
- **Fight**: assault witness, Fight 30, alarm 100, `iFightAttacking` 50, disp -30 (bias 16), distance bias 40: fight
  106 capped to 70, combat starts, base Fight 100; alarm 50: 53, 83 < 100, no combat.
- **Guards**: bounty 999 ignore; 1000 pursue (LOS and awareness); 10000 attack; pursuit ends at bounty 0; witnesses
  with id `<=` paid calm and lose the disposition modifier.
- **Pay/jail**: bounty 1550 -> 15 days; 50 -> 1; globals for bounty 100, discount 0.5, turn-in 0.25, gold 99/100:
  `PCHasCrimeGold` 0/1, discount 50, turn-in 25; bounty 1 -> both 1; `PayFine` confiscates, `PayFineThief` does not.
- **Pickpocket**: thief Agi 50, Luck 50, Sneak 30; victim Agi 40, Luck 40, Sneak 20; value 100 x 1: `x = 45`,
  `y = 332`, `t = -242 < 6`: roll 6 safe, 7 caught; `finish`: `y = 32`, `t = 58`: roll 58 safe, 59 caught; `t` 90 caps at 75.
- **Locks**: Agi 50, Luck 50, Security 40, quality 1, level 30: `x = 25`, roll 25 opens, 26 fails; level 100: impossible;
  quality 1.5 level 30: 52.5; probe cost 20: `x = 52`; uses 1 -> tool removed; key unlocks and disarms with no
  trespass; owned unlocked-but-once-locked object is trespass; Open spell emits `UnlockAttempted`; `Lock` with no
  argument on a never-locked object uses 100; `Unlock` keeps the level.
- **Awareness**: sneaking, Sneak 40 Agi 50 Luck 50, no boots, distance 500: `x = 45 x 1.5 = 67.5`; observer
  Sneak 30 Agi 40 Luck 40 in front: `y = 63`, target 4.5, roll 4 safe, 5 seen; behind: `y = 21`, target 46.5, roll 46 safe,
  47 seen; not sneaking always seen; Invisibility: 104.5 never seen; dead observer never.
- **Needs Morrowind.esm** (`#[ignore]`): GMST values above; the arrest greeting globals exist with those names.

TODO (unfinished 2026-11-10): this spec is ~28 KB, over the ~25 KB budget; move the GMST table and the Tests list to `archive/crime-adapter-tests.md` when slice A0 starts.

## Build plan (one deliverable per slice; framework LW1-LW5 first)

| Slice | Deliverable |
|---|---|
| **A0** | Reference data: `ANAM/BNAM/CNAM/INDX/FLTV/KNAM/TNAM` into the world model; `LockState` load rule; `MwCrimeGmst`; tests |
| **A1** | `mw_crime::ownership`: `isOwned`, `isAllowedToUse`, stolen map, `StolenVeto` for services A7; tests |
| **A2** | `mw_crime::offence`: table, `commitCrime` witness logic, `reportCrime` per-witness rules, `recordCrimeId` on S7 bounty writes; tests |
| **A3** | Guards, pay, jail: `LawRules`, `PayFine`/`PayFineThief`/`GoToJail`, dialogue globals, `JailRules`, bed rule; tests |
| **A4** | Pickpocket rule and pickpocket window host path |
| **A5** | `mw_crime::locks`: `pickLock`, `probeTrap`, trap rule, key flow, Open spell, `Lock`/`Unlock` scripts; NPC `crime_modifier` into `DerivedDisposition`; tests |
| **A6** | `mw_crime::stealth`: `awarenessCheck` as `DetectionRules`; scaffold witnesses wired |
| **A7** | Host wiring (`sf_crime_bevy`), saves, prompt marker, visual-check entry (awaiting owner) |
| A8 | Options UI, werewolf bounty, Meter model with AI8 |

## Open questions (owner)

1. **Stolen ledger**: reconcile (default) or classic stale entries. Pending.
2. **Detection**: classic roll (default) now, Meter once AI8. Pending.
3. **Jail skill loss**: documented rule (default); confirm against the game when the handler is found. Pending.
4. **Fences**: no fence in Morrowind content; modern fence offer off (default). Pending.
5. GMSTs marked U and the global names need no decision: the first owner run with Morrowind.esm confirms them.
