# Crime adapter: Morrowind crime, law, stealth, locks and traps on `sf_crime` (`mw_crime`)

Status: spec written 2026-11-10, revised 2026-11-26 (LW0e: reports at delivery, decision 14 split, options, open
questions; plan in the [crime roadmap](../../../docs/roadmaps/crime.md), "decision N" = its §3); nothing built.
Framework: [crime.md](../../../superfargoth/docs/specs/crime.md),
[crime-locks.md](../../../superfargoth/docs/specs/crime-locks.md). Also [stats-adapter.md](stats-adapter.md) (`bounty`
counter, `MwGmst`, fatigue, `UseSkill`), [services-adapter.md](services-adapter.md) (stolen veto),
[effects-adapter.md](effects-adapter.md) (trap and Open spells), [ai-adapter.md](ai-adapter.md) (MA5, MA6). ROADMAP 2.8.

Ported from OpenMW `ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f`; paths under `apps/openmw/` unless noted, line
numbers at that commit, read from the raw sources 2026-11-10. **MM** = `mwmechanics/mechanicsmanagerimp.cpp`,
**ACT** = `mwmechanics/actors.cpp`.

<!-- agent-summary -->
> **Agent summary.** Bevy-free `mw_crime` implements the framework rule traits with OpenMW's numbers: ownership,
> stolen map, offences, `commitCrime`, `reportCrime` split **at sight** (reporter, disposition, fight) and **at
> delivery** (bounty, expulsion, guard alert) in both presets, the `iCrimeThreshold` ladder on one island-wide
> jurisdiction, pay / jail, locks, traps, pickpocket. `awarenessCheck` is MA5's (`mw_ai`). **Classic quirks kept**:
> any alarmed reporter in `fAlarmRadius` reports, seen or not; stolen entries never shrink; integer rolls `0..=99`.
<!-- /agent-summary -->

## Goal

Same crimes, bounties and reactions as OpenMW for the same records and GMSTs, through the modern host (events, fixed
tick, per-player state), with modern-first switches in Options. Non-goals: werewolf bounty (MM 1936-1976; the
werewolf slice calls `OffenceAttempt`), guard dialogue content (in Morrowind.esm), perception, combat hit rules.

**Who owns what** (decision 14): `mw_crime` (no `sf_ai` dependency) owns A0-A5: reference data, ownership, offences
and reports (A2), law and jail (A3), pickpocket, locks and traps. MA5 (`mw_ai`) owns the `awarenessCheck` port behind
`WitnessQuery`. MA6 only turns a guard's `Stance` into AI directives (Pursue, combat).

Each rule's `In` holds every number it reads (GMSTs, roll, distances); rolls come from the host's seeded roll.

## What exists in the repo (2026-11-10)

- `mw_stats` key `bounty`; S7 `GetPCCrimeLevel`/`SetPCCrimeLevel`/`ModPCCrimeLevel` (floor 0) are live but skip
  `recordCrimeId` at 0 (`statsextensions.cpp` 463-493 call it): A2 closes this.
- `functions.txt`: `PayFine`, `PayFineThief`, `GoToJail` `later`; `GetPCInJail` `stub`; `Lock`, `Unlock`,
  `GetLocked` `live` (check them against "Locks").
- `DerivedDisposition::input`: `crime_modifier` is a named TODO (A5); `fDispCrimeMod x bounty` is applied (MM 544).

## Data on references (A0)

ESM reference subrecords (`components/esm3/cellref.cpp`): `ANAM` owner id (88-90), `BNAM` owner global variable
(91-96), `CNAM` owning faction (97-99), `INDX` faction rank (100-102), `FLTV` lock level (124-126), `KNAM` key id
(127-129), `TNAM` trap spell id (130-132); `NAM0` skipped. A level `0` is stored as `INT_MAX` (`ZeroLock`) so
"locked at level 0" survives (13, 162-163, 236-238). **`isLocked` is derived on load**: from a content file
`locked = (key != empty || level > 0)`, from a save `level > 0` (159-163). **Unlock negates the level**
(`mwworld/cellref.cpp` 319-329), so an unlocked object that was ever locked has a nonzero level. Our `LockState {
level, locked, key }` stores the level positive and `locked` apart; `was_locked = level != 0` reproduces this.

Records: Alarm base, class id `guard` (case-insensitive; MM 1397, ACT 1172), factions and ranks (stats-adapter).
`MwOwnership` maps `ANAM/BNAM/CNAM/INDX` to the framework `Ownership`.

## Ownership (A1)

`isOwned(player, target) -> (owned, victim)` (MM 78-106):

- `owner = ANAM`; owned iff `owner != empty && owner != "Player"` (case-insensitive id compare).
- Faction-owned iff `CNAM != empty` and the actor is an NPC and (not a member **or** member rank `< INDX`).
- If `BNAM` names a global with a nonzero integer value: both flags cleared.
- `victim` = the owner actor looked up in the world when `ANAM` is set; owned or faction-owned counts.

(The Lua `isAllowedToOpen`, `files/data/scripts/omw/crimes.lua` 5-33, treats rank 0 as "not in faction"; we port the
C++.) `isAllowedToUse(player, target) -> (allowed, victim)` (MM 886-925), in order: empty target or self: allowed; an
**unlocked, untrapped door**: allowed; no tooltip: allowed; activators that are not beds (script name starts "Bed"):
allowed; an NPC target: dead or in combat allowed, knocked down **or the player sneaking** not allowed (the pickpocket
gate), else allowed; owned: not allowed. Framework `use_allowed` with `never_take = ["stolen_goods"]`.

### Stolen items (A1)

`itemTaken(player, item, container, count, alarm)` (MM 1079-1135; pickpocket passes `alarm=false`): player only;
allowed returns. Owner key: an actor container's `refId`; else the container's (or
item's) `ANAM`, if empty its `CNAM` as a faction (`second = true`). Gold is never recorded. A non-gold item is
recorded (`stolen[item id][owner] += count`) iff there is no victim actor or it is alive and `count > 0`. If `alarm`,
`commitCrime(theft, faction = CNAM, value)`, `value = count` for gold else `count x item value`. Corpses are free
(`containeritemmodel.cpp` 256-258).
`isItemStolenFrom(item, merchant)` (MM 982-1002): the (merchant id, false) key, or the merchant's primary faction as
(faction, true). `confiscateStolenItemToOwner` (MM 1004-1046; trade and enchanting): the victim's faction replaces the owner key when the item's `CNAM` equals it; decrement `min(count, stolen)`,
erase at 0, move the items to the victim, `commitCrime(theft, item faction, value x moved)`.
`confiscateStolenItems(player, chest)` (MM 1048-1077): per item, walk owners in map order consuming `min(remaining,
owned)`; move `count - leftover` to the chest; **lock the chest at level 50** (OpenMW TODO). Map order is `(RefId,
bool)` pair order; ours is `BTreeMap`.

Classic: stolen is permanent. No `reconcile` (counts stay after sale), no cool-down; only the victim merchant (id or
primary faction) refuses and confiscates, forever (crime.md "Fences"). Modern (decision 11): `reconcile` on every
inventory change, lots cool off (`crime.stolen.cool_*`), other merchants refuse hot lots, no offence. No fence in
Morrowind content; a mod may flag one.

## Offences (A2)

Types: `OT_Theft`, `OT_Assault`, `OT_Murder`, `OT_Trespassing`, `OT_SleepingInOwnedBed`, `OT_Pickpocket`
(`mwbase/mechanicsmanager.hpp`), as `OffenceDef` records `mw.crime.<name>`; trespass and bed share a row.

| Offence | Bounty, at delivery (MM 1233-1271) | Disposition, witness / victim, at sight | Fight, witness / victim, at sight (MM 1287-1309) |
|---|---|---|---|
| Trespass, SleepInBed | `iCrimeTresspass` | `iDispTresspass` / same | `iFightTrespass` / same |
| Pickpocket | `iCrimePickPocket` | `fDispPickPocketMod` / same | `iFightPickpocket` / `iFightPickpocket x 4` |
| Assault | `iCrimeAttack` | `iDispAttackMod` / `fDispAttacking` | `iFightAttacking` / `iFightAttack` |
| Murder | `iCrimeKilling` | `iDispKilling` / same | `iFightKilling` / same |
| Theft (`arg` = value) | `max(1, int(arg x fCrimeStealing))` | `fDispStealing x arg` / same | `int(fFightStealing) x arg` / same |
| Other (script) | `arg` | 0 | 0 |
| Vandalism (added) | as Theft, `arg` = value destroyed | as Theft | as Theft |

`fFightStealing` is a float read through `getInteger()`: truncation kept (MM 1307). Murder first calls
`notifyMurder` (MM 1233-1234). Vandalism (owner 2026-11-11) is not OpenMW: breakables and IN37 `Forced`
(interaction-adapter.md), only where breaking or bashing is on.

## `commitCrime` (A2; MM 1137-1202)

Runs at sight, on the offence tick.

1. Player only (NPC culprits are Modern, crime.md LW10). Assault forces `victimAware = true`.
2. Candidates: actors within `fAlarmRadius` plus the victim beyond it; the player's followers and allies
   (`getActorsSidingWith`, recursive) never report.
3. `canReportCrime(actor)` (MM 1204-1225): not the player, an NPC, alive, not in combat **with the victim**, not
   knocked down, not a follower.
4. A candidate **saw** it iff `(actor == victim && victimAware)`, or `(type == Murder && actor != victim)` (hearing
   suffices), or `LOS && awarenessCheck` (`WitnessQuery`, answered by MA5). A theft or pickpocket witness says `thief`.
5. If anyone saw: `reportCrime`. Else for `Assault` with a victim: a `guard` victim not pursuing reports with an
   empty faction; otherwise `startCombat(victim, player)` (no bounty). Returns `crimeSeen`.

## `reportCrime` (A2; MM 1227-1507)

OpenMW runs all of it at once. We split it (decision 2; owner 2026-11-25: witnesses walk in Classic too): the **at
sight** half runs on the offence tick from `assess` (framework `ReactionRules`, `ReportPlan`); the **at delivery**
half on `ReportDelivered`, once per `CrimeId`. Let `alarm = Alarm base`, `alarmTerm = alarm / 100`, `cur` = derived
disposition, `disp` the offence's disposition (victim value for the victim).

**At sight:**

- **Reporter** iff a reportable candidate in radius has `alarm >= 100` (MM 1316-1325; trespass: `intruder` bark).
  Classic quirk: seen or not. Modern: only a witness who saw or heard. `ReportRules::target` sends it to the nearest
  `guard` by nav distance, through doors; a guard reporter is its own target, delivered next tick. Recognition:
  Classic `Identified`, Modern per crime.md.
- **Crime id**: the batch gets its `CrimeId` (OpenMW `newCrimeId`); each witness with a change keeps it (pardon key).
- **Disposition** (integer cast, MM 1355-1390): Theft: `int(disp x alarmTerm)`. Pickpocket: guard with `alarm >=
  100`: `int(disp)`; victim guard: permanent `int(disp x alarmTerm)`; other victim: permanent `int(disp)`. Assault:
  non-guard victim: permanent `int(disp)`; `alarm >= 100`: `int(disp)`; victim guard: permanent `int(disp x
  alarmTerm)`; else `int(disp x alarmTerm)` **only if hostile**. Murder, trespass: 0. Clamped to `[-cur, 100 - cur]`;
  permanent edits the base, others add to `crimeDispositionModifier` (reset by pardon, MM 478, ACT 1217).
- **Guard with `alarm >= 100`**: Alarmed, stance `Arrest` for this guard whatever the bounty; MA6 pushes Pursue
  (OpenMW `AiPursue`, MM 1397-1405); it arrests once its own report lands.
- **Other witnesses**: pickpocket non-victim: `alarmTerm = 0`; pickpocket victim with `alarm = 0`: `alarmTerm = 1`;
  `fight = (isVictim ? fightVictim : fight) + (50 - disp) x fFightDispMult + (iFightDistanceBase -
  fFightDistanceMultiplier x d)` (`mwmechanics/combat.cpp` 567-596), times `alarmTerm`, capped so `Fight + fight <=
  100`, floor 0; at `Fight + fight >= 100`: combat, base Fight `+= int(fight)`, Alarmed.
- An assault victim not yet fighting starts combat and keeps the crime id (MM 1469-1505).

**At delivery** (`ReportDelivered { report, guard }`):

- **Bounty**: `bounty = max(0, bounty + amount)` on the guard's jurisdiction (the island-wide `bounty` stat) with
  its `BountyCause`, via `CrimeCommand::AddBounty`, the S7 writer: `recordCrimeId` runs where OpenMW calls it (pay,
  jail, a write reaching 0). A second report of the same crime adds only a cause.
- **Expulsion** (MM 1469-1505): a victim NPC whose primary faction the player is in expels the player
  (`expell(faction, true)`), else the offence's named faction.
- **Guard alert**: `LawRules::stance` re-reads the bounty; MA6 turns it into directives. A cancelled report files
  none of this; the at-sight half stays done.

`actorAttacked` (MM 1509-1558): `canCommitCrimeAgainst` (NPC, not aggressive toward the attacker, not engaged, not
pursuing, not a werewolf, no Vampirism, MM 1560-1568), then `Assault`, fight back, friendly followers join.
`actorKilled` (MM 1570-1601): player or followers only; murder iff the victim could be committed against **or**
carries an assault crime id (the player attacked first); creatures are not murder.

## Guards, pay and jail (A3)

**Sight rule** (`ACT::updateCrimePursuit` 1153-1225, per frame): a `guard` not pursuing, not in combat, not Calmed,
with `bounty >= iCrimeThreshold`, LOS and `awarenessCheck`: at `bounty >= iCrimeThreshold x
iCrimeThresholdMultiplier` combat, else `AiPursue`; Alarmed and a new crime id (1177-1198); werewolves skipped.
Witnesses with crime id `<= paidCrimeId` calm down (pursuit, combat, Alarmed, Fight, disposition modifier reset;
1203-1223, `mwgui/dialogue.cpp` 797-801): framework `Pardon`. Ours is event-driven: AI29's notice asks
`LawRules::stance`, MA6 acts on it.

**Jurisdiction**: `JurisdictionRules::of` returns one island-wide jurisdiction on the default `bounty` stat, which
Morrowind's dialogue and scripts read (owner Q3 pending, recommended). Classic ladder: `Ignore` below
`iCrimeThreshold`, `Arrest` from it, `Attack` from `iCrimeThreshold x iCrimeThresholdMultiplier` with no murder test
(ACT 1177-1198), no decay. Modern: the framework ladder and `crime.law.*` defaults, decay with the murder floor, heat.

**Arrest** (`aipursue.cpp` 30-79): pursuit stops at bounty 0 or a dead or hidden target; the guard paths to the
player (tolerance 100 units) and with LOS opens the arrest greeting. Ours opens the framework `ArrestSession`; its
screen is that **content dialogue** (DQ-15 hooks): `PayFine` = `ArrestOption` Pay, `PayFineThief` = Pay without
confiscation, `GoToJail` = Jail, the refusal = Resist. Before each dialogue opens, `World::updateDialogueGlobals`
(`mwworld/worldimp.cpp` 3490-3512) writes `PCHasCrimeGold = bounty <= gold`, `CrimeGoldDiscount`, `PCHasGoldDiscount = discount <= gold`,
`CrimeGoldTurnIn`, `PCHasTurnIn = turnIn <= gold`, with `discount = int(bounty x fCrimeGoldDiscountMult)`, `turnIn =
int(bounty x fCrimeGoldTurnInMult)`, each floored to 1 when `bounty > 0` (names **unverified**, check the esm).

| Script | Effect (`mwscript/miscextensions.cpp`) |
|---|---|
| `PayFine` | 1351-1373: bounty 0, `confiscateStolenItems` into the nearest prison's `stolen_goods`, `recordCrimeId`, holster |
| `PayFineThief` | 1375-1382: bounty 0, `recordCrimeId`, no confiscation |
| `GoToJail` | `World::goToJail` (`worldimp.cpp` 3540-3573): first call `days = max(1, bounty / iDaysinPrisonMod)`, bounty 0, `recordCrimeId`, confiscate; the **second** call (next unpaused frame) closes dialogue, holsters, opens the jail screen |
| `GetPCInJail` | `isPlayerInJail` |
| `Set`/`ModPCCrimeLevel` | reaching 0 calls `recordCrimeId` (`statsextensions.cpp` 463-493) |

**Jail screen** (`mwgui/jailscreen.cpp` 35-90): fade out 0.5 s, `teleportToClosestMarker(player, "prisonmarker")`
(its `destCell` is the prison; `stolen_goods` must be there, else a warning, `worldimp.cpp` 3515-3537), progress bar
over 100 ticks, fade in, `rest(days x 24)`, `advanceTime`, corprus worsenings skipped, Lua `jailTimeServed`.

**Mapping** (crime.md "Reactions, arrest, jail"): `GoToJail` opens a `Sentence { days, place, clock }`:
`LawRules::jail_days` is the formula above, `jail_place` = `release_place` = the closest `prisonmarker`, evidence =
its cell's `stolen_goods`. Clock: `JailClock::PassTime` in solo in both presets (owner 2026-11-25: days pass as a
rest does), `JailClock::Credit` in co-op. The jail screen is the local scene. `JailRules::served`: Classic = the skill
loss hook; Modern = reputation hit, no skill loss (owner Q4 pending, recommended).

**Skill loss** (Classic hook): OpenMW does it in Lua via `SkillProgression`, source `jail`: -1 on every skill except
Sneak and Security (documented, `files/data/scripts/omw/skillhandlers.lua` 131, 207); the per-day handler is not in
this commit (`_onJailTimeServed` is only an engine hook, `mwlua/localscripts.hpp` 111). We port the documented rule.

## Sleeping in a bed (A3)

`sleepInBed` (MM 927-952): werewolf and enemies nearby refuse; an owned bed is `commitCrime(SleepingInOwnedBed)`:
seen means `sNotifyMessage64` and no sleep, unseen proceeds.

## Pickpocket (A4)

Gate: activating an NPC while sneaking opens the pickpocket model. Rule (`mwmechanics/pickpocket.cpp` 18-80):
`mod(actor, add) = (add + 0.2 Agility + 0.1 Luck + Sneak) x fatigueTerm`, `x = mod(thief, 0)`, `y = mod(victim,
valueTerm)`, `t = 2x - y`, `m = pcSneak / iPickMinChance`. Caught iff: `t < m`: `roll > int(m)`; else `roll >
int(min(iPickMaxChance, t))` (roll `0..=99`). Taking: `valueTerm = 10 x fPickPocketMod x (value x count)`; closing
without taking: `valueTerm = 0` (`finish`). A catch is `commitCrime(Pickpocket, victimAware = true)` and closes the
window; a success trains Sneak (`Sneak_PickPocket`) and records the take with no alarm (`pickpocketitemmodel.cpp`
85-130). A knocked-down victim skips the check (118-120). Reverse pickpocket is refused (`onPlaceItem`, 80).

## Locks and traps (A5)

**Activation order**, container (`mwclass/container.cpp` 140-208) and door (`mwclass/door.cpp` 143-237): locked
**and** the player has the `KNAM` key: `<key name> sKeyUsed`, unlock, and **disarm** a trap (`Disarm Trap`); then if
unlocked or keyed, a trap fires (`ActionTrap`, `Disarm Trap Fail`) before opening, a door teleports only when
untrapped; still locked: `LockedChest`/`LockedDoor` sound, no open. Tooltips: `sLockLevel: <n>` when locked,
`sUnlocked` when level != 0 and unlocked, `sTrapped` (`container.cpp` 250-259, `door.cpp` 282-291). A key is not an
unlock attempt.

`Security::pickLock` (`security.cpp` 24-70): `level <= 0` or no tooltip: nothing; `uses` 0: nothing. `x = (0.2 Agility
+ 0.1 Luck + Security) x quality x fatigueTerm + fPickLockMult x level`; `unlockAttempted` first; `x <= 0`:
`sLockImpossible`; else success iff `roll <= x`: unlock, `sLockSuccess`, `Open Lock`, `Security_PickLock`; else
`sLockFail`. Either way `uses -= 1`, removed at 0. `probeTrap` (72-128): `x = ((0.2 Agility + 0.1 Luck + Security) +
fTrapCostMult x trapSpellCost) x quality x fatigueTerm`; `x <= 0`: `sTrapImpossible`; success iff `roll <= x`: trap
cleared, `Disarm Trap`, `Security_DisarmTrap`, else `sTrapFail`; uses -1. `trapSpellCost` = `mw_effects::spell_cost`.

`unlockAttempted` (MM 954-966): owned (either flavour) **and** (`level != 0` or locked or trapped):
`commitCrime(Trespassing, faction)`. Callers: pick, probe (`security.cpp` 55, 105), the **Open** spell on a lockable
target (`mwmechanics/spelleffects.cpp` 1287). `ActionTrap` (`mwworld/actiontrap.cpp` 10-35): beyond
`getMaxActivationDistance()` (telekinesis) the trap detonates at the object, else it is cast **on the activator**;
then cleared. Scripts: `Lock [level]` defaults to the current level, or 100 if never set (`miscextensions.cpp`
332-357); `Unlock` only if locked (363-372); `GetLocked` = `locked`. DQ-7 runs them on crime's `LockState`.

**On the pick seam** (crime-locks.md "The pick seam", decision 9): A5 provides `pick_term` (`pickLock`'s `x`),
`pick_chance`, `window` and `max_hp`.

- **Classic** (`LockInteraction::Roll`): `x <= 0` is `Impossible` (`sLockImpossible`, refused), else `Percent(x)`
  against the seeded roll. The tooltip shows the raw `sLockLevel: <n>`. No forcing.
- **Modern** (`Minigame`; "auto" rolls with the `lock.auto_min_chance` floor): `x` sets `window`; `x <= 0` gives the
  narrowest window, never a refusal. The prompt shows one of the five tier labels from `tier(level)`. Only the data
  `impossible` flag refuses; Morrowind content sets none (quest doors refuse through their scripts).
- **`max_hp(level)`** for IN37: `level x mw.crime.lock_hp_per_level` (hp per level, 0.5-10, default 2, Guess, set
  against IN37's blow scale), unless content sets `hp`.
- Probe stays a roll in both presets; `x <= 0` is `sTrapImpossible` in both.

## Stealth and awareness (the port is MA5 in `mw_ai`)

Slice A6 is dropped (decision 14). `awarenessCheck` (MM 1603-1674) is ported once, by MA5: ai-adapter.md "AI
settings" (**Awareness**, **Perception profile**) holds the formula, its Classic roll and its Meter split. The
player's sneak visibility (stance, speed, Sneak, Chameleon) is MA18 (`mw_walkers_bevy`). `mw_crime` reads no
awareness: A2 and A4 get witnesses from `WitnessQuery`. The OpenMW reading this section held, with its GMSTs and test
cases, is in [archive/crime-adapter-history.md](archive/crime-adapter-history.md).

## GMSTs read (U = from memory, **unverified**; checked by the `#[ignore = "needs Morrowind.esm"]` test)

| GMST | Use | Expected |
|---|---|---|
| `iCrimeThreshold`, `iCrimeThresholdMultiplier` | ladder | 1000, 10 (U) |
| `fCrimeGoldDiscountMult`, `fCrimeGoldTurnInMult` | dialogue globals | 0.9, 0.5 (U) |
| `iCrimeKilling`, `iCrimeAttack`, `iCrimePickPocket`, `iCrimeTresspass`, `fCrimeStealing` | bounties | 1000, 40, 25, 5, 1 (U) |
| `iDaysinPrisonMod`, `fAlarmRadius` | jail days, witness radius | 100, 4000-5000 units (U) |
| `iDisp*`, `fDisp*` of the offence table | disposition | negative (U) |
| `iFight*`, `fFightStealing`; `fFightDispMult`, `iFightDistanceBase`, `fFightDistanceMultiplier` | fight | U (bias ~0.2, ~50, ~0.005) |
| `fDispCrimeMod` | disposition per bounty | U (~0.02) |
| `iPickMinChance`, `iPickMaxChance`, `fPickPocketMod` | pickpocket | 5, 75, 0.3 (U) |
| `fPickLockMult`, `fTrapCostMult` | lock, trap | -1, -0.15 (U) |
| `fFatigueBase`, `fFatigueMult` | fatigue term | 1.25, 0.5 |
| `iWereWolfBounty` | werewolf (later) | 1000 (U) |

Strings (`sNotifyMessage*`, `sKeyUsed`, `sLock*`, `sTrap*`, `sUnlocked`, `sTrapped`) come from content.

## Options (`MwCrimeOptions`, one row per field; A8 shows them, owner LW-6)

| Field | Modern default | Classic (OpenMW) |
|---|---|---|
| `detection` | Meter + one sneak eye (MA5, decision 6) | one seeded roll per sight check (MA5) |
| `reporters` | witnesses who saw or heard, `alarm >= 100` | any reportable actor in radius, `alarm >= 100` |
| `report_cancel` | dead, downed, bribed, threatened, timeout | dead, downed (decided, open questions 1, 2) |
| `recognition` | `Unknown`/`Description`/`Identified` with `Disguise` (owner Q2 2026-11-26) | always `Identified` |
| `jurisdictions` | per jurisdiction (owner Q3 changed 2026-11-26; design in LW0f) | per jurisdiction (LW0f) |
| `law` | framework ladder, decay, murder floor, heat | `iCrimeThreshold` ladder, no decay |
| `locks` | `Minigame` ("auto" rolls), tier label, forcing (IN37) | `Roll` with `Impossible`, raw level, no forcing |
| `pickpocket` | KCD2 timer window, "auto" rolls (owner Q1 2026-11-26) | roll (A4) |
| `stolen` | reconcile, cool-down, other merchants refuse hot lots | permanent, only the victim refuses |
| `prompt` | owned/stolen marker on crosshair and item rows | none (item names only) |
| `jail` | confiscation, reputation hit, no skill loss (owner Q4 pending) | confiscation, skill loss hook shown at release |

Both presets: reports walk to a guard (owner 2026-11-25); jail is `PassTime` solo, `Credit` co-op; no fences.

## Tests (exact; synthetic records and GMSTs unless noted)

GMSTs: `fCrimeStealing` 1, `iCrimeThreshold` 1000, `iCrimeThresholdMultiplier` 10, `iDaysinPrisonMod` 100,
`fPickLockMult` -1, `fTrapCostMult` -0.15, `iPickMinChance` 5, `iPickMaxChance` 75, `fPickPocketMod` 0.3,
`fFightDispMult` 0.2, `fFatigueTerm` 1.

- **Ownership**: owner "fargoth" owned; "player" free; faction "mages" rank 3: member at 2 owned, at 3 free,
  non-member owned; `BNAM` global 1 frees both; unlocked untrapped door allowed, locked or trapped owned door not;
  sneaking at an NPC denies, in combat or dead allows; `stolen_goods` never takeable.
- **Stolen**: 3 swords from a Fargoth chest: `(sword, fargoth) = 3`; gold never; dead owner not recorded; faction
  chest keys the faction; held 2 of `{fargoth 2, guild 1}`: confiscate 2, remainder `{guild 1}`, chest locked at 50;
  merchant match by id or primary faction; Modern reconcile to held 1 removes the oldest first (exact map).
- **Offence table**: theft arg 150 -> 150; arg 0 -> 1; `fCrimeStealing` 0.5 and arg 1 -> 1; pickpocket 25 etc.
- **Witness**: murder reported without LOS; assault forces victim aware; follower, dead, knocked down excluded; no
  witness plus guard victim -> report, else victim combat.
- **Reported**: Classic alarm 100 beyond sight reports, Modern needs a witness; disposition and fight at sight;
  bounty and expulsion only on delivery (cancelled: none), never below 0; a guard reporter delivers next tick; a
  second report of one crime adds a cause, no bounty.
- **Disposition**: theft arg 100, `fDispStealing` -0.5, alarm 50: -25; modifier -80 with cur 30 clamps to -30;
  permanent victim edits base.
- **Fight**: assault witness, Fight 30, alarm 100, `iFightAttacking` 50, disp -30 (bias 16), distance bias 40: 106
  capped to 70, combat, base Fight 100; alarm 50: 53, 83 < 100, no combat.
- **Law**: one jurisdiction for every guard; 999 ignore; 1000 pursue (LOS and awareness); 10000 attack with no
  murder on record; pursuit ends at 0; witnesses with id `<=` paid calm down and lose the modifier.
- **Pay/jail**: 1550 -> 15 days; 50 -> 1; bounty 100, discount 0.5, turn-in 0.25, gold 99/100: `PCHasCrimeGold` 0/1,
  50, 25; bounty 1 -> both 1; `PayFine` confiscates, `PayFineThief` does not; solo `PassTime`.
- **Pickpocket**: thief Agi 50, Luck 50, Sneak 30; victim Agi 40, Luck 40, Sneak 20; value 100 x 1: `x = 45`,
  `y = 332`, `t = -242 < 6`: roll 6 safe, 7 caught; `finish`: `y = 32`, `t = 58`: 58 safe, 59 caught; `t` 90 caps at 75.
- **Locks**: Agi 50, Luck 50, Security 40, quality 1, level 30: `x = 25`, roll 25 opens, 26 fails; level 100: Classic
  `Impossible`, Modern narrowest window; quality 1.5: 52.5; probe cost 20: `x = 52`; uses 1 -> removed; a key
  unlocks and disarms, no trespass; owned once-locked object is trespass; Open spell emits `UnlockAttempted`; `Lock`
  on a never-locked object uses 100; level 30 tier = Apprentice (bounds 20/40/60/80); `max_hp(50) = 100`.
- **Needs Morrowind.esm** (`#[ignore]`): GMST values above; the arrest greeting globals exist with those names.

## Industry comparison

| Engine | Law, report, jail | Ours |
|---|---|---|
| OpenMW | alarmed NPCs report at once, seen or not; one bounty; jail passes days, costs skills | Classic keeps it; **departs**: reports walk (owner) |
| Skyrim | bounty per hold on sight; guard dialogue: pay, jail, resist; jail costs skill progress | **match**: arrest options; **departs**: one bounty (Q3), walked reports |
| KCD2 | witnesses run to a guard, can be stopped; face and clothes; fines, jail, reputation | **match**: Modern reports, recognition, jail without skill loss |

## Build plan

One deliverable per slice; rows, order and checks: [crime roadmap](../../../docs/roadmaps/crime.md) §4.

- **A0** reference data, `MwCrimeGmst`: Stage 1. **A1** ownership, Classic stolen map: Stage 1. **A5** locks, traps,
  keys, `pick_term`, `max_hp`: Stage 1.
- **A2** offences, `commitCrime`, `reportCrime` at sight and at delivery: Stage 3. **A3** ladder, pay, jail, skill
  loss hook, bed: Stage 3.
- **A4** pickpocket roll and window: Stage 4. **A8** options UI: Stage 5.
- A6 dropped (decision 14, MA5); no A7 (host, saves, prompt are framework LW slices).

## Open questions

Framework and preset questions (minigame feel, headgear, one bounty, jail skill loss in Modern, stolen cool-down) are
answered (owner 2026-11-26, crime roadmap §7): all as recommended except one bounty, changed to per jurisdiction
(rewrite LW0f; the island-wide lines above are superseded until then).

1. **Can a Classic witness be bribed or threatened out of reporting?** **Decided (meta, 2026-11-26): no.** Bribe and Threat are Modern;
   Morrowind has no such dialogue, so Classic keeps its outcome.
2. **A crime far from any guard (tombs, camps, the wilds)?** **Decided (meta, 2026-11-26), as written:** the witness walks to the nearest guard by
   nav distance. Past `crime.report.timeout_s`, Classic delivers anyway (Morrowind's bounty lands, later); Modern
   cancels (RDR2, KCD2: an unreported crime in the wilds stays unreported).

No decision needed:

3. **Jail skill loss**: the documented OpenMW rule is the default; confirm it against the game once the handler is found.
4. GMSTs marked U and the global names: the first run with Morrowind.esm confirms them.
