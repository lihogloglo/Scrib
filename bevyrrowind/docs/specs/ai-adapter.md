# AI adapter: Morrowind's packages, AI settings, crime and schedules on `sf_ai` / `sf_nav`

Status: **draft (2026-11-08); MA1 (NV4b) and MA3 built (2026-11-18, awaiting owner check AI-2)**. Framework design: `../../../superfargoth/docs/specs/ai.md`
(directives, drives, HTN, tiers) and `../../../superfargoth/docs/specs/nav.md` (navmesh, links, worldspace graph).
Research: `../../../superfargoth/docs/research/npc-ai-2026-11-08.md` §4, §7.2-7.3. Proposed ROADMAP 3.9 / 3.10.
OpenMW source for every port: `gitlab.com/OpenMW/openmw` at **`b8285cc95d19e080fe0532f9f5154b0c3f727735`** (GPL-3.0);
ported code cites file + commit in its module docs, gets a `superfargoth/THIRD_PARTY_NOTICES.md` entry and lives
**only in `mw_*`**.

## Crates

- **`mw_ai`** (exists, Bevy-free, hakari `final-excludes`): `pathgrid`, `wander`, `methods` (`WanderVocabulary`,
  `WanderDriver`, `wander_library`; `PathWalkScaffold` removed in MA3). Gains: `packages` (AI_W/T/E/F/A → directives), `policy` (the OpenMW `StackPolicy`), `settings`
  (Hello/Fight/Flee/Alarm considerations), `awareness` (the `DetectionModel`), `perception` (profile, AI0g),
  `fast_forward`, `schedules` (loader, anchors, quest-safety validator), `nav` (agent classes, area table, pathgrid links, door approach points), and the
  Morrowind HTN method set as data (`bevyrrowind/data/ai/methods/*.ron`).
- **`mw_rpg::crime`** (Bevy-free; crime is a content mechanic): offences, witnesses, bounty, guards, arrest. Not a new
  `mw_crime` crate (decided 2026-11-08, `superfargoth/docs/research/ai-nav-decisions-2026-11-08.md` (h)): OpenMW keeps
  crime beside disposition and persuasion (`mechanicsmanagerimp.cpp`), which `mw_rpg` already holds, with the bounty
  functions. No `sf_ai` dependency: witnesses in, effects out (bounty, Fight, reporters, pursuers, crime fact); the Bevy
  glue pushes *Pursue* and appends the fact. Split out only if it ever needs `sf_ai`/`sf_nav` types.
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
  at race scale 1.0; small creatures use it too) and `large` (creature collision bounds above a size threshold). **Two
  classes, no `small`** (decisions (a)): no vanilla passage admits only small creatures, and a small class would cost
  ~4× the humanoid bake. MA1 prints the creature-radius histogram per class (a revisit trigger). OpenMW bakes one mesh
  per agent bounds (`components/detournavigator/navigator.hpp`).
- **Areas** (`sf.nav.area`), from OpenMW's `areatype.hpp` (ground, water, door, pathgrid) plus ours: `ground` 1,
  `water` (swimmers only; others high cost), `door` 2 (OpenMW's door cost; door leaves excluded from the bake geometry,
  the doorway footprint marked), `door_closed` (set at runtime on those polygons), `path` (pathgrid corridors, slightly
  cheaper so NPCs keep to streets), `hazard` (lava: avoid filter). No vanilla record sets `carves_nav`.
- **Pathgrid** (`mw_ai::pathgrid`, PGRD → `petgraph`, exists): edges become **off-mesh links** only where the navmesh
  has no connection between the endpoints (OpenMW `offmeshconnectionsmanager.hpp`); nodes stay as **wander hints** and
  **door approach points** (nearest node in front of each door, seamless research §4.11). Fallback when a worldspace
  has no navmesh: the pathgrid route, as OpenMW `pathfinding.cpp` 150-210 does.
- **Geometry**: exactly the cooked collision the mover uses (`collision.cook`), terrain from LAND, water level per
  worldspace; NIF collision quirks are the bake's (actors-adapter "Static collision").
- **As built (NV4b, 2026-11-11, `mw_bake::nav`)**: `MwNavSource` reads, per tile, the `mw.nav.cell` products (64 m
  squares listing every static/item/door whose world bounds touch them, plus the level-0 `terrain.tile`s under them;
  bytes = plan param, so a moved wall re-bakes only its cells' tiles), then `collision.cook` per model and the baked
  terrain heights (what the physics heightfield uses, not raw LAND). Actors, lights, markers, editor-only and
  start-disabled references are not geometry. Water: one quad at OpenMW's swim level (`getSwimLevel`: water −
  (`fSwimHeightScale` + 1) × half height, GMST read from the records), area `water`, only over tiles with terrain or
  objects; seabed kept. Doors: leaf excluded, `DoorVolume` = leaf footprint widened to 0.75 m each side, id = the
  reference's `PlacementId` folded to 64 bits; approach = nearest pathgrid node ≤ 6 m (`mw.nav.links`). Areas
  ground 1 / door 2 / water 3 (water cost 2); humanoid swims, `large` doesn't. Pathgrid off-mesh links: NV9.

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
(MA5); `Get/Set/ModHello|Fight|Flee|Alarm` on the sheet (MA4); crime's `PayFine`, `PayFineThief`, `GoToJail` (crime-adapter A3; MA6 only routes guards).

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
- **Awareness** (`DetectionModel`): `awarenessCheck` (`mechanicsmanagerimp.cpp` 1603) with its sneak, attribute,
  fatigue, chameleon/invisibility and facing terms; light enters only through `VisibilityFactors` (AI8d, crime LW0b), never inside MA5.
- **Perception profile** (`mw_ai::perception`, AI0g; ai.md "Perception profile"): presets and state records in
  `bevyrrowind/data/ai/perception.ron`, curves as `mw.perception.*` tunables (source `Guess` unless OpenMW gives the
  number; OpenMW lines below checked on a local checkout at `4e718b50`, `awarenessCheck` at 1603 as at `b8285cc9`).
  Split for **Meter**: `awarenessCheck`'s observer half becomes the profile, its target half (sneak term, chameleon,
  invisibility) stays in `DetectionRules::concealment`, its facing test is `f_cone`.
  - *Stats:* `obs = Sneak + 0.2 Agility + 0.1 Luck - Blind` (OpenMW `obsTerm`, `mechanicsmanagerimp.cpp` 1651;
    creatures use the record's Stealth skill, `creature.cpp` `getSkill` 725-732); `x = obs / obs_ref`; `sight_gain` and
    `hearing` = `clamp(1 + slope × (x - 1), mul_min, mul_max)` with `sight_slope` / `hearing_slope`. Willpower →
    `search_ttl` (`willpower_slope`), Intelligence → `search_error` (`1 - int_slope × (Int / attr_ref - 1)`), same clamp.
    Refilled when a term moves by ≥ `refill_step` (Blind on and off included).
  - *Traits* (preset rows by race, creature type, or id): Khajiit `darkness` 0.5 (their Eye of Night power; vanilla
    Night Eye only brightens the player's view, so the NPC trait is a modern reading); undead `darkness` 0.3, daedra 0.5;
    beasts (CREA type Creature) `hearing` 1.3, `sight_cone` 1.2; Argonians none (water breathing is not perception). All
    `Guess`; a mod row can make a blind creature (`sight_gain` 0).
  - *States:* **tired** (condition): `sight_gain` × OpenMW's fatigue term `fFatigueBase - fFatigueMult × (1 - fatigue /
    max)` (`creaturestats.cpp` 39-53, applied to `obsTerm` as OpenMW does), re-pushed per `refill_step` of the fraction.
    **drunk** (timed): while an active effect sourced from an item in the `alcohol` list (sujamma, flin, greef, mazte,
    shein, skooma, moon sugar) lasts, or `drunk_s` from a tavern drink activity (MA8): `sight_gain` 0.7, `alertness` 0.7,
    `decay` 1.3, `search_error` 1.5. **injured** (condition, health fraction < `injured_below`): `sight_gain` 0.85,
    `search_ttl` 0.6. **fleeing** (condition, while MA4's Flee holds): `sight_cone` 0.7, `hearing` 1.2, `search_ttl` 0.
    **on duty** (condition, guard class as OpenMW's crime code `isClass(victim, "guard")`, 1193, while awake and not
    off-shift in MA8): `alertness` 1.2, `decay` 0.7, `search_ttl` 1.5. State numbers live in the records (data, `Guess`).
  - **Classic** (MA5): the roll already holds `obsTerm`, Blind and the observer fatigue term, so the stat part, `tired`
    and every sight field (`sight_*`, `darkness`, `alertness` on sight) of profile and states are not applied: MA5
    stays exact vs the formula, and drunk, injured, fleeing and on duty have no vanilla number to bend its odds with.
    Their hearing, decay and search fields still apply, since Classic keeps the framework meter there (vanilla has no
    hearing or search rule).
  - Tunables (`mw.perception.*`, net `Gameplay`): `obs_ref` 30 (≈ a commoner: Sneak 15, Agility 40, Luck 40), `attr_ref`
    50, `sight_slope` 0.5, `hearing_slope` 0.3, `willpower_slope` 0.5, `int_slope` 0.4, `mul_min` / `mul_max` 0.5 / 2,
    `refill_step` 0.1 (fraction of the term), `injured_below` 0.5, `drunk_s` 300.

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
- **When**: MA8 ships after AI10 (tier 0), AI11 and AI12 (doors), not before: a schedule that sends a shopkeeper home
  crosses worldspaces, and without tier 0 the actor would freeze dormant mid-route (decisions (e)). Until then
  `ScheduleRules::None`.
- **Anchors** resolved by `mw_ai::schedules`: `home` (an owned bed in another place, else none), `work` (the placement
  cell for merchants and service NPCs), `tavern`, `temple`, `patrol(route)` (a pathgrid loop, later). A pattern
  applies only where its anchors resolve; otherwise the NPC keeps its record packages (vanilla).
- **First patterns** (data, tune by play): (1) shopkeepers and service NPCs at work 08-20, home at night if `home`
  resolves, else in the shop cell; (2) publicans in the tavern 10-02; (3) commoners outdoors by day, indoors at night
  where `home` resolves, homeless exterior commoners stay vanilla; (4) **guards stay on post 24 h**: vanilla has one
  guard per post, so shifts would halve guard presence and change crime-witness balance (a ported mechanic); shifts
  wait for a pack that authors relief guards. Until AI14 (beds), "home at night" is `StayNear` indoors.
- **Merchants never refuse service** (decisions (f)): availability follows presence; a merchant found off-shift trades
  as vanilla (an "after hours" greeting is optional content). No schedule-driven door locks (a later opt-in
  `ai.shop_hours = off | lock_doors`). Travel-service NPCs (caravaners, boatmen, guild guides) are never scheduled.
- **Seam**: game setting `ai.schedules = modern | classic` (host, saved with the world); `classic` installs
  `ScheduleRules::None`: NPCs keep their record packages (wander in place), exactly vanilla.
- **Quest safety** (validator, report on a real install): excluded by default are NPCs with an attached script,
  NPCs addressed by position in any script (`PositionCell`, `Position`, `AITravel`, `GetDistance` on them, `MoveWorld`),
  essential NPCs, and those whose dialogue filters on their own cell. Scripts and quests outrank schedules anyway. The
  same exclusion list bars actors from cross-worldspace vendettas (ai.md "Memory"). The report prints, per pattern,
  NPCs resolved, excluded and left vanilla.

## Doors and followers

Approach points from the nearest PGRD node in front of each door; `Traverse` opens doors through `OnActivate`
routing (`mw_doors_bevy`: locked, trapped and scripted doors that never `Activate` stay shut) and closes a door it
found shut. Load (portal) doors swing ~1 s and auto-close when the passage is clear and no player is near; in-cell
doors keep vanilla behaviour (stay as left). Seamless vs classic is the host world rule `world.doors` (pack default
classic until seamless M9a passes; seamless research §7). Fade-only links (and every link while seamless is off) are
crossed only when no player sees either end. Followers walk through; vanilla's follower teleport is the fallback when
blocked for N s and unseen (decided, seamless research §7 Q7).

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
| **MA1** nav data | two agent classes, area table, `NavGeometrySource` (door leaves out, doorway areas), pathgrid links, door approach points | real install: tile and link counts, bake time per class, creature-radius histogram printed | NV4 |
| **MA2** packages | AI_* records and MWScript functions → directives, OpenMW `StackPolicy`, `GetAIPackageDone` | exact stack orders vs OpenMW cases; script tests per function | AI4 |
| **MA3** wander methods (**built 2026-11-18**) | Morrowind wander/idle method set on `sf_ai`; `PathWalkScaffold` removed | headless: wanderers stay within radius; idle chances exact (`roll_idle_with` vs OpenMW `getRandomIdle` at b8285cc; `drive_tests.rs` runs the real Bevy `drive`) | AI6 |
| **MA4** settings | Hello/Fight/Flee stats + considerations | exact ratings vs OpenMW formulas | AI7 |
| **MA5** awareness | `awarenessCheck` port as `DetectionModel` Classic; Meter profile split (AI0g) | exact vs formula cases; Classic ignores profile sight fields | AI8 |
| **MA6** crime | glue to `mw_crime` (crime-adapter; no `sf_ai` dep there): guards' *Pursue* | scripted theft: witnesses, bounty, guard pursues | MA5 |
| **MA7** fast-forward | `fastForward` rules | exact placement for wanderers and travellers | AI10 |
| **MA8** schedules | data pack (three patterns, guards unscheduled), anchors, validator, `ai.schedules` setting | validator report per pattern on a real install; classic = vanilla stacks; unresolved anchors keep vanilla | AI10, AI11, AI12 |
| **MA9** doors | escort/follow/pursue through doors, follower fallback | headless: follower through the Census door, guard pursues out | AI12 |
| **MA10** combat sheets | NPC and creature combat through C6 | scripted fight, no NaN | AI15 |

As built (MA14, 2026-11-25): `mw_walkers_bevy/src/sight.rs` adds `AiSightPlugin` on the controller's ground
(`Physics`, `MwGround` with `--off physics`); walkers get `AiObserver` with `AiActor` (`walkers.rs` `attach`); the
player body gets `Perceivable { Player(HOST_KEY) }` + `PerceivablePose` (fed each tick, survives F11 re-spawn).
Eye 0.936 x, chest 0.720 x standing height (Drillis-Contini 1966; NPC height = 1.8 m capsule x race `body_scale.y`,
creatures their fitted capsule, player `ActionBody.params.height`). Perception record kinds registered in `mw_mods`
(`sf_ai::register_perception_kinds`, moved from `sf_ai_bevy`). Not yet: profile/state adapter, NPCs as targets,
player race height, record reload, explicit worldspace filter (dormant observers skip checks).

As built (MA16, 2026-11-25): `mw_walkers_bevy/src/light_inputs.rs` writes `AiLightInputs` each fixed tick (after
`ActionSystems::Move`, before `AiPerceptionSystems::Sight`). Interior = interior CELL without "behave like exterior".
Sky outdoors `saturate(ln(E/3 lx) / ln(10000 lx / 3 lx))`, E = sun + moon ground illuminance × sin elevation + fill
from `CurrentSky` (weather dims it; the renderer's night ambient floor left out); indoors AMBI Rec. 709 luminance /
0.25 (no AMBI: 0.3 grey). Sources: R0 LIGH placements via a `LightHook` (LHDT radius in metres, `OFF_BY_DEFAULT` =
off, lifted 0.3 m off the base), carried lights at the carrier's chest. Tunables `mw.ai.light.*` (5). Known: with
light-only visibility (until crime LW4) pitch dark = unseen by sight; no light spells; dusk drops fast (MA17).

As built (AI8f2, 2026-11-25): `mw_walkers_bevy/src/search.rs` `Searcher` (required by `Walker`): `arbitrate` picks
search when the strongest entry is Searching (start at its `LastSeen`; restart only on fresher evidence outside the
radius, another target, or after give-up), `step` runs `SearchGoal`s through the wander vocabulary's MoveTo/Face
executors on the navmesh branch of `drive`; ends at the ttl tick or when the band leaves Searching, then resumes the
wander (`driver.face`). Reachable = `NavMeshes::nearest` within `DEFAULT_EXTENT`. Alerted does not search yet; off-mesh
aborts; not saved. AI8f3: the running search holds the band (`hold_searching`), released at give-up.

As built (AI8c2, 2026-11-25): `mw_walkers_bevy/src/noise_emitters.rs`: the player's `StrideClock` advances by ground
distance / stride (gait from the action intent: sneak 0.5, walk 0.75, run 1.5, sprint 2.0 m), one revealing footstep
per stride (reach sneak 2, walk 6, run 12, sprint 18 m, Guess); landing reach = min(30, 8 + 4 × (drop - 0.5 m)) from
the impact speed; teleports and non-ground modes emit nothing; `surface_gear_mul` = 1 seam. Written after
`ActionSystems::Move`, before `AiPerceptionSystems::Hearing`. Tunables `mw.ai.noise.*` (12). Not yet: NPC steps,
surface/gear, action `noise: Loud`, IN18 object noises.

## Open questions

None open. Decided 2026-11-08 (owner-delegated; `superfargoth/docs/research/ai-nav-decisions-2026-11-08.md` (e), (f),
(h)): three first patterns with guards unscheduled; merchants never refuse service; crime in `mw_rpg::crime`. Shop
hours and pattern choice are taste calls the owner may overturn cheaply (data and one setting).
