# AI adapter: Morrowind's packages, AI settings, crime and schedules on `sf_ai` / `sf_nav`

Status: **draft, docs only (2026-11-08)**, nothing built. Framework design: `../../../superfargoth/docs/specs/ai.md`
(directives, drives, HTN, tiers) and `../../../superfargoth/docs/specs/nav.md` (navmesh, links, worldspace graph).
Research: `../../../superfargoth/docs/research/npc-ai-2026-11-08.md` §4, §7.2-7.3. Proposed ROADMAP 3.9 / 3.10.
OpenMW source for every port: `gitlab.com/OpenMW/openmw` at **`b8285cc95d19e080fe0532f9f5154b0c3f727735`** (GPL-3.0);
ported code cites file + commit in its module docs, gets a `superfargoth/THIRD_PARTY_NOTICES.md` entry and lives
**only in `mw_*`**.

## Crates

- **`mw_ai`** (exists, Bevy-free, hakari `final-excludes`): today `pathgrid`, `wander`, `scaffold` (`PathWalkScaffold`,
  removed in MA3). Gains: `packages` (AI_W/T/E/F/A → directives), `policy` (the OpenMW `StackPolicy`), `settings`
  (Hello/Fight/Flee/Alarm considerations), `awareness` (the `AwarenessModel`), `fast_forward`, `schedules` (loader,
  anchors, quest-safety validator), `nav` (agent classes, area table, pathgrid links, door approach points), and the
  Morrowind HTN method set as data (`bevyrrowind/data/ai/methods/*.ron`).
- **`mw_rpg::crime`** (Bevy-free; crime is a content mechanic): offences, witnesses, bounty, guards, arrest.
- **`mw_bake`**: the `NavGeometrySource` over cooked collision (`collision.cook` products), LAND heights and water.
- **Glue**: `mw_walkers_bevy` (owns NPC walkers; installs `sf_ai_bevy` for them), `mw_script` (AI functions),
  `mw_doors_bevy` (door activation rules for the `Traverse` operator).
- **`mw_rules`** (later, never early): classic executors (`AiWander` etc.) and classic combat AI behind the seams.

## Rules or content (ARCHITECTURE §1b)

| Mechanism | Kind | Here |
|---|---|---|
| Package stacking, completion, repeat, `GetAIPackageDone` | content (scripts depend on it) | **port** `aisequence.cpp`, `aipackage.hpp` |
| AI settings Hello/Fight/Flee/Alarm, greeting, aggression, flee rating | content data + rules | **port** `actors.cpp`, `aicombataction.cpp`, `mechanicsmanagerimp.cpp` |
| Crime, witnesses, bounty, guards | content (listed as a content mechanic) | **port** `mechanicsmanagerimp.cpp` |
| Awareness (sneak detection) | content formula | **port** `awarenessCheck` |
| `fastForward` on rest/wait | content | **port** |
| How packages are walked (paths, wander movement, follow spacing) | feel | **modern** on `sf_nav`/`sf_ai` |
| Combat AI | feel | **modern** (combat.md C6); classic `aicombataction.cpp` ratings later in `mw_rules` |
| Schedules | modern addition (owner 2026-11-08) | data pack, `ScheduleRules` seam |

## Navigation (`sf_nav` data)

- **Agent classes** (`sf.nav.agent` records from `mw_ai::nav`): `humanoid` (radius and height from the human capsule
  at race scale 1.0; small creatures use it too) and `large` (creature collision bounds above a size threshold).
  A third `small` class only if NV4's bake time allows (nav.md open question 1). OpenMW bakes one mesh per agent
  bounds (`components/detournavigator/navigator.hpp`).
- **Areas** (`sf.nav.area`), from OpenMW's `areatype.hpp` (ground, water, door, pathgrid) plus ours: `ground` 1,
  `water` (swimmers only; others high cost), `door` 2 (OpenMW's door cost), `door_closed` (runtime), `path` (pathgrid
  corridors, slightly cheaper so NPCs keep to streets), `hazard` (lava: avoid filter).
- **Pathgrid** (`mw_ai::pathgrid`, PGRD → `petgraph`, exists): edges become **off-mesh links** only where the navmesh
  has no connection between the endpoints (OpenMW `offmeshconnectionsmanager.hpp`); nodes stay as **wander hints** and
  **door approach points** (nearest node in front of each door, seamless research §4.11). Fallback when a worldspace
  has no navmesh: the pathgrid route, as OpenMW `pathfinding.cpp` 150-210 does.
- **Geometry**: exactly the cooked collision the mover uses (`collision.cook`), terrain from LAND, water level per
  worldspace; NIF collision quirks are the bake's (actors-adapter "Static collision").

## Packages → directives

Records' AI_W/AI_T/AI_E/AI_F/AI_A (with CNDT cells) and the MWScript functions push the same directives.

| Morrowind | Directive | Modern execution | Completion (ported) |
|---|---|---|---|
| `AI_W` / `AIWander d, h, t, idles` | `StayNear { home, radius: d }` + idle table | slot activities when present, else pathgrid-node wander + Idle2-9 chances × `fIdleChanceMultiplier` (today's `mw_ai::wander` rules); `t` (time of day) stays inert as in OpenMW | `h` game hours; repeat |
| `AI_T` / `AITravel x y z` | `GoTo(Location)` | `WorldGraph` + nav, `Traverse` through doors | within tolerance (`aitravel.cpp`) |
| `AI_E` / `AIEscort(Cell)` | `Escort { who, to }` | lead; wait when the escorted falls behind; resume | arrival or duration (`aiescort.cpp`) |
| `AI_F` / `AIFollow(Cell)` | `Follow(who)` (`PlayerId` resolved to `PlayerKey`) | spacing at distance, through doors (`followTargetThroughDoors`) | duration or arrival (`aifollow.cpp`) |
| `AI_A` / `AIActivate obj` | `Use(PlacementId)` | approach (slot if any), fire activation | activated (`aiactivate.cpp`) |
| `StartCombat` / Fight | *Combat* drive target | C6 operators | `StopCombat`, death, target lost |
| Pursue (guards), AvoidDoor, Face, Cast, Breathe | operators or drives (`aipursue.cpp`, `aiavoiddoor.cpp`, `aiface.cpp`, `aicast.cpp`, `aibreathe.cpp`) | modern | as OpenMW |

**Stack policy** (`mw_ai::policy`): `AiSequence::stack` (`aisequence.cpp` 386) inserts by `getPriority()`, honours
`shouldCancelPreviousAi`/`canCancel`, sends repeating packages back to the end, keeps the combat/pursue counters;
`execute` (249) and the `Options` in `aipackage.hpp` (priority, repeat, canCancel, sideWithTarget,
followTargetThroughDoors, alwaysActive) map to `Directive` fields and `ParamBag`. `mDone` → `Directive::done`.

**MWScript** (`mw_script/src/functions.txt`: "AI, combat" block, `Get/Set/Mod{$AISTATS}` 188-190, `GetLineOfSight`
90; "later" or "stub" today): `AIWander`, `AITravel`, `AIEscort(Cell)`, `AIFollow(Cell)`, `AIActivate` push
directives; `GetAIPackageDone`, `GetCurrentAIPackage` read the stack (MA2); `StartCombat`/`StopCombat`, `GetTarget`,
`GetAttacked`, `Face` (MA2/MA10); `GetDetected` (awareness) and `GetLineOfSight` (perception via `portal_query`)
(MA5); `Get/Set/ModHello|Fight|Flee|Alarm` on the sheet (MA4); crime's `PayFine`, `PayFineThief`, `GoToJail` (MA6).

## AI settings (stats + considerations)

Hello, Fight, Flee, Alarm (`aisetting.hpp`) are magnitude stats on the sheet (`sf_stats`, stats-adapter naming),
initialised from the NPC/creature AIDT. Considerations in `mw_ai::settings`:
- **Greeting** (React drive): every 0.25 s, target within `Hello × iGreetDistanceMultiplier`, LOS and awareness
  (`actors.cpp` `updateGreetingState` 481-560); turn, look-at, Greeting bark from dialogue.
- **Aggression** (Combat drive): `engageCombat` and `isAggressive` (Fight + disposition + `getFightDistanceBias`;
  `actors.cpp` 618-760, `mechanicsmanagerimp.cpp` 1429); allies side with allies.
- **Flee** (Flee drive): `vanillaRateFlee` (`aicombataction.cpp` 457-492), `updateFleeing` (`aicombat.cpp` 149-250),
  intimidation raising Flee (`mechanicsmanagerimp.cpp` 664-708). The rating is ported; flee movement is modern (nav
  `Goal::Away`, toward allies or a door).
- **Awareness** (`AwarenessModel`): `awarenessCheck` (`mechanicsmanagerimp.cpp` 1603) with its sneak, attribute,
  fatigue, chameleon/invisibility and facing terms; a modern light-level term is a later option behind the same model.

## Crime, witnesses, guards (`mw_rpg::crime`)

- `commitCrime` → `reportCrime` (`mechanicsmanagerimp.cpp` 1137-1450): offence kinds (trespass, pickpocket, theft,
  assault, murder, werewolf); witnesses = the framework's event-time witness query over every simulated actor within
  `fAlarmRadius` with LOS + awareness, through open seamless doors via `portal_query` (no per-package hacks).
- Bounty from the GMSTs listed in stats-adapter ("Factions, reputation, bounty"); Fight raised by
  `iFightTrespass/Pickpocket/Attack/Killing`; Alarm decides who reports; **guards** (`Alarm ≥ 100` and class Guard)
  get a *Pursue* directive (OpenMW `AiPursue`), across doors; arrest dialogue on contact (pay, jail, resist);
  `iCrimeThreshold`, faction expulsion.
- Crime records are world facts in the `sf_ai` fact store (per faction, with witnesses), so they save once.

## Fast-forward (rest, wait)

`fastForward` (`aipackage.hpp` 89, `actors.cpp` `fastForwardAi` 2413): wanderers to a random pathgrid point within
their radius, travellers to their destination. Implemented as the adapter half of `sf_ai::fast_forward`, also used by
tier-0 actors when a player rests.

## Schedules for vanilla NPCs (owner 2026-11-08, modern addition)

- A data pack `bevyrrowind/data/ai/schedules/` of `sf.ai.schedule` records plus assignment records (by class,
  faction, cell group or NPC id), moddable like any records. Vanilla has none (its `game_hour` field is inert).
- **Anchors** resolved by `mw_ai::schedules`: `home` (an owned bed or the NPC's own placement cell), `work` (the
  placement cell for merchants and service NPCs), `tavern`, `temple`, `patrol(route)` (a pathgrid loop).
- Starting patterns (tune by play): merchants and service NPCs at work in the day, home at night; guards in day and
  night shifts with relief; publicans late; commoners in their cell by day, indoors at night.
- **Seam**: game setting `ai.schedules = modern | classic` (host, saved with the world); `classic` installs
  `ScheduleRules::None`: NPCs keep their record packages (wander in place), exactly vanilla.
- **Quest safety** (validator, report on a real install): excluded by default are NPCs with an attached script,
  NPCs addressed by position in any script (`PositionCell`, `Position`, `AITravel`, `GetDistance` on them, `MoveWorld`),
  essential NPCs, and those whose dialogue filters on their own cell. Scripts and quests outrank schedules anyway.

## Doors and followers

Approach points from the nearest PGRD node in front of each door; `Traverse` opens doors through `OnActivate`
routing (`mw_doors_bevy`: locked, trapped and scripted doors that never `Activate` stay shut). Fade-only links (and
every link while seamless is off) are crossed only when no player sees either end. Followers walk through; vanilla's
follower teleport is the fallback when blocked for N s and unseen (seamless research Q7).

## Memory (Nemesis-style) for Morrowind

Fact kinds as records: attacked me, stole from me, spared me, defeated me, helped me (quest), crime witnessed.
Facts feed the `DispositionModel` input (stats-adapter "Factions, reputation, bounty, disposition") and dialogue
filters through a new function family; named enemies remembering and returning are content for later packs.

## Industry comparison

| Item | OpenMW | Bethesda (later games) | Ours |
|---|---|---|---|
| Packages | `AiSequence` stack, ported semantics | conditioned packages, procedure trees | ported semantics over directives (**match** OpenMW) |
| Schedules | none | conditioned packages (Oblivion on) | data pack, optional (**match** Bethesda; **differ** from vanilla by owner choice) |
| Doors | followers teleport; no cross-cell nav | door markers, out-of-view teleport | `WorldGraph` + traverse (**ahead** of OpenMW) |
| Crime | `commitCrime`/`reportCrime` | crime factions + witnesses | ported, witnesses through doors |
| Off-screen | processing range only; `fastForward` | process levels | tiers + `fastForward` port |

## Build plan (adapter slices)

| Slice | Deliverable | Test | After |
|---|---|---|---|
| **MA1** nav data | agent classes, area table, `NavGeometrySource`, pathgrid links and door approach points | real install: tile and link counts, bake time printed | NV4 |
| **MA2** packages | AI_* records and MWScript functions → directives, OpenMW `StackPolicy`, `GetAIPackageDone` | exact stack orders vs OpenMW cases; script tests per function | AI4 |
| **MA3** wander methods | Morrowind wander/idle method set on `sf_ai`; `PathWalkScaffold` removed | headless: wanderers stay within radius; idle chances exact | AI6 |
| **MA4** settings | Hello/Fight/Flee stats + considerations | exact ratings vs OpenMW formulas | AI7 |
| **MA5** awareness | `awarenessCheck` port as `AwarenessModel` | exact vs formula cases | AI8 |
| **MA6** crime | `mw_rpg::crime` port, guards' *Pursue* | scripted theft: witnesses, bounty, guard pursues | MA5 |
| **MA7** fast-forward | `fastForward` rules | exact placement for wanderers and travellers | AI10 |
| **MA8** schedules | data pack, anchors, validator, `ai.schedules` setting | validator report on a real install; classic = vanilla stacks | AI11 |
| **MA9** doors | escort/follow/pursue through doors, follower fallback | headless: follower through the Census door, guard pursues out | AI12 |
| **MA10** combat sheets | NPC and creature combat through C6 | scripted fight, no NaN | AI15 |

## Open questions (owner)

1. Schedule starting patterns: the four above, or fewer (merchants and guards only) for the first playable?
2. Should merchants refuse service outside work hours (modern), or stay available when found (vanilla-friendly)?
3. Crime in `mw_rpg::crime` (no new crate) or a separate `mw_crime`?
