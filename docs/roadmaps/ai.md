# Lane roadmap: NPC AI (hub lane)

Specs: `superfargoth/docs/specs/{ai,ai-as-built,world-reactions}.md`, `bevyrrowind/docs/specs/ai-adapter.md`. Research:
`docs/research/gameplay-2026-11-25/ai.md` + AI sections of `combat`, `crime`, `interaction`, `nav`, `travel`, `ecswind`.
Ledgers `docs/ledger/{ai,nav}.md`; wiki `docs/wiki/engine/nav-and-ai.md`.

## 1. Player-facing goal

Modern preset, lane done (stage in brackets): NPCs **see and hear** you by distance, cone, light, stance and noise,
grow suspicious before acting, search your last known position and call others [1]; keep **schedules**: shops open at
dawn, lamps lit at dusk, people home at night, streets empty in an ash storm [2]; **notice and react**: a witness walks
to a guard and can be stopped, guards warn, search and cool off, victims suspect whoever was near, NPCs bark [3];
**pick up, carry, run errands**, tidy what you knocked over [4]; **mantle, vault, drop, climb** after you, and **fliers
and swimmers path in 3D** (cliff racers around towers, slaughterfish in the water) [5]; fight with **tactics** (a few
attack, the rest circle, the beaten flee) [6]; **companions** stay out of your way, obey a command wheel, sneak with you
and are knocked down, not killed [7]; **gossip** carries your deeds NPC to NPC and sets greetings, prices and guards
[8]; caravaners and shipmasters are **operators** at their stops, **travellers** walk the roads [9].

**Classic keeps** Morrowind packages, OpenMW `awarenessCheck` as gain function, Hello/Fight/Flee/Alarm ratings, no
schedules (`ai.schedules = classic`), vanilla fame instead of gossip, later the combat action ratings. **Reports walk in
both presets** (owner 2026-11-25): Classic keeps Morrowind's bounty sums, not the instant report.

## 2. Where we are (2026-11-25)

- **Owner-verified:** MA3 wander/idle HTN on the navmesh (AI-2, Balmora, 2026-11-18); NV4c ring + terrain origin
  (NV-1); NV7a fixed tick (NV-2).
- **Built, headless:** AI1 trace, AI2 HTN, AI3 overlay (AI-1 awaiting owner), AI4 directives, AI5 `sf/ai` save, AI6
  executors (`MoveTo`, `Wait`, `Face`, `PlanRunner`), AI7 IAUS drives; NV6 query service, NV8 corridor.
- **In game:** only `AI_W` wander on `sf_ai`; fliers/swimmers on PGRD polylines; one nav anchor, humanoid only
  (ROADMAP 3.9, 3.10, living-world `[p]`).
- **Leftovers (ledger):** `Face` can't finish in pathgrid fallback (yaw 0.0); `mw_devtools_bevy` tunables list lacks
  `sf_ai_bevy`, `mw_walkers_bevy`.
- **Not started:** MA2, MA4, AI8 and everything after (AI9-AI25, MA5-MA10).

## 3. Decisions

1. **Keep the stack** (directives, IAUS, HTN, operators, smart objects, 3 tiers; Decima/KCD/UE shape, research §0-1).
   No AI crate as core: of the 0.20 crates (`bevior_tree`, `moonshine-behavior`, `bevy_gearbox`) none is a Bevy-free typed
   serde core. GOAP stays a non-goal.
2. **Graded perception first** (TLOU2, Hitman, AC Shadows; research §2): `Awareness.level` 0..1 with bands unaware /
   curious / suspicious / searching / alerted; gain from distance, central vs peripheral cone, visibility, alertness;
   decay; `LastSeen { pos, tick, velocity }` fact; alerts spread as `Stimulus::Notify` at finite speed through doors,
   confidence dropping (no telepathy); companions obey stealth. `DetectionRules::concealment` (crime.md) stays the
   formula seam; Meter default, Classic roll for Morrowind.
3. **Light from game state, never the GPU:** host `LightExposure` from clock, weather, interior flag and `Lit` sources,
   one sample per player per tick, shared by AI and the light gem. Rendered light would be presentation feeding
   gameplay (net rule 3).
4. **Living world before tactics** (research §3): thin day loop (AI9, AI10, AI12, AI14, AI11) and reactions before AI15;
   ai.md's "AI15 then AI14" is reversed. Schedules still wait for tier 0 and doors (ai.md decision (e)).
5. **No needs system** (KCD2, RDR2): 2-3 considerations (tired at night, hungry at meals) bias smart-object picks.
   Weather/time are `WorldState` keys (`raining`, `storm`, `night`, `dark`) plus an `Ambient` concern kind.
6. **One interrupt contract** in ai.md: which drive pre-empts which, how plans resume (`Library::holds`, `Pause`).
7. **Barks: no own crate.** The dialogue lane's rule DB (DQ-23) picks lines, DQ-25 plays them; AI emits
   `BarkCue { speaker, concept, context }` gated by listener hearing/LOS (replaces research §5 `sf_barks`).
8. **Gossip = bounded propagation** (owner; ECSwind "typed facts, no rumour mutation"): facts carry `heard_from`, `hops`
   (≤ 3), confidence per hop; swapped in shared scenes and talks, at schedule edges on tier 0. Drops ai.md's non-goal
   "rumour kits".
9. **Followers are a directive** (nav research §5): `Follow { leader, slot, rules }`, leader-frame slots, personal space,
   out-of-view catch-up (Bethesda), escort speed clamp; commands are host commands, the wheel is presentation;
   knocked down not killed by default (owner; data flag, combat C2c knockdown).
10. **NPC traversal reuses the player's moves** (UE smart links): a link operator drives the same `sf_action` mantle/vault
    states (AL-10, AL-12) by `Command`; a failed link raises its cost for repath. No NPC-only climb code.
11. **3D fliers and swimmers** (owner; Horizon Zero Dawn GDC): clearance height field and water-body volume (nav NV13),
    3D executors here; levitation = class swap. Replaces "fliers steer with probes" (ai.md, nav.md non-goal).
12. **Combat split:** C6 (`sf_combat`) owns tokens, band picker, skill reactions, telegraph floors; AI15 owns drives,
    target choice, HTN engage/reposition/search, ring-slot nav, alert sharing.
13. **Tiers unchanged in numbers** (Floor 48/160 at ≤ 0.55 ms, High 128/400 at ≤ 1.0 ms, provisional until AIM; fixed
    counts as UE MassLOD, LOD-unaware scripts as KCD2). Added (research §4): staggered first update after cell load;
    tier 0 runs no planner (state machine over `(route, progress, activity, until)`); promotion resolve queued
    nearest-first with a per-tick cap; per-tier ms in `ai.stats`.
14. **Perception cost** (UE AIPerception time-slicing): grid prefilter, then cone, then 1-2 rays; ray cap per tick
    (`ai.perception.max_rays_per_tick`, Guess); cache N ticks; stagger by `hash(ActorKey)`; tier 1 distance + same space,
    tier 0 none; event-time witness queries stay tier-independent. Floor share 0.15 ms per tick.
15. **Operators are ordinary actors** (travel research §1): an `Operate` directive, killable, can be absent; road
    travellers are tier 0 on NV12 and materialize on the road out of view.
16. **Net-ready:** host-only directives, perception, facts, claims, gossip; replicated activity, look-at, barks; a remote
    player is another `PlayerKey`; `FixedUpdate`, hashed RNG.

## 4. Stages

Stages 1-3 in order; 4-9 start as their other-lane needs land (cross-lane: `order.md`). Owner checks `AI-n` go in
`bevyrrowind/docs/visual-checks/ai.md` (AI-1, AI-2 exist). New IDs: AI26+, MA11+; spec slices AI0b-AI0f.

### Stage 0: specs (docs only)

| ID | Deliverable | Crates | Depends | Verify |
|---|---|---|---|---|
| AI0b | ai.md "Perception" rewrite (decisions 2, 3, 14) | docs | - | review; crime LW4 links |
| AI0c | ai.md interrupt table, build order, tier additions, 3D movers, industry fixes (KCD = BTs, not GOAP) | docs | - | doccheck |
| AI0d | Followers and companions section (directive, commands, stealth, downed; adapter `AI_F`/`AI_E`) | docs | - | review |
| AI0e | ai.md "Memory": gossip, reputation per faction and settlement | docs | - | review |
| AI0f | ai.md errands, `Operate`, link-operator contract | docs | AI0c | review |

### Stage 1: NPCs see and hear

Ends with: a Balmora guard spots a sneaking player in light, not in shadow, checks a thrown cup, calls his partner.

| ID | Deliverable | Crates | Depends | Verify |
|---|---|---|---|---|
| MA2 | AI_* records + MWScript AI functions to directives, OpenMW `StackPolicy` | `mw_ai` | AI4 | exact vs OpenMW cases |
| AI8a | Awareness core: meter, bands, `LastSeen`, bounded per-player entries, `Stimulus::Noise` | `sf_ai` | AI0b | exact curves, eviction, serde |
| AI8b | Sight: grid prefilter, cone, batched LOS, ray cap, stagger, cache | `sf_ai_bevy` | AI8a | headless exact; ms as measurement test |
| AI8c | Hearing by `portal_query::path_distance`; emitters from action `noise`, gait, surface | `sf_ai_bevy` | AI8a, IN18 | exact attenuation through a door |
| AI8d | `VisibilityFactors` + host `LightExposure` | `sf_ai`, `sf_ai_bevy` | AI8a | exact: equal at 30/144 fps |
| AI8e | Shared knowledge: `Notify` from shouts and alarms | `sf_ai` | AI8a | exact arrival ticks, 3-room graph |
| AI8f | HTN `search`: LKP, look, widen, give up on ttl | `sf_ai` | AI8a, AI6 | headless: reaches LKP, quits at ttl |
| MA5 | `awarenessCheck` port as Classic gain | `mw_ai` | AI8a | exact vs formula |
| MA4 | Hello/Fight/Flee/Alarm considerations; greeting via AI8 | `mw_ai` | AI7, AI8b | exact vs OpenMW |
| AI26 | Perception gizmos (cone, meter, LKP), `ai.perception` | `sf_ai_bevy` | AI8b | owner **AI-3** (Balmora guard, light vs shadow) |

### Stage 2: a day in town

Ends with: Balmora runs dawn to night and through a 3 h wait; lamps lit at dusk; commoners take shelter in a storm.

| ID | Deliverable | Crates | Depends | Verify |
|---|---|---|---|---|
| AI9 | Allocator: tiers 2/1, `sim.cpu_tier` caps, pins, staggered first update | `sf_ai`, `sf_ai_bevy` | AI7 | exact + hysteresis; cost line |
| AI10 | Tier 0 state machine, queued promotion resolve, `fast_forward` | `sf_ai` | AI9, NV11, NV12 | tier invariance; no in-view promotion |
| AI12 | Doors: `Traverse`, fade-only unseen rule | `sf_ai`, `mw_ai` | NV11, AI6 | follower through both door modes |
| AI14 | `UseSlot` + fragments (= interaction IN8), claim priority | `sf_ai`, `sf_interaction` | IN0, IN2, IN3 | deterministic claims; IN-8 |
| AI11 | Schedules, `ScheduleRules`, quest-safety validator | `sf_ai` | AI10, AI12 | exact pushes at window edges |
| AI27 | Free-time picker (time, role, 2-3 considerations), ambient keys, shelter and dusk-lamp methods | `sf_ai` | AI14, IN19 | exact picks on a synthetic day |
| MA7 | `fastForward` rules | `mw_ai` | AI10 | exact placement |
| MA8 | Schedule pack (3 patterns, guards on post), anchors, `ai.schedules` | `mw_ai`, data | AI11, AI27 | validator report; owner **AI-4** (Balmora day, wait, storm) |
| AIM | Cap sweeps Floor/High, AI + mover + animation ms | measurement | AI9, MA8 | one run; provisional rows filled |

### Stage 3: NPCs notice and react

Ends with: steal in a Balmora shop; the clerk walks to a guard; stop her, or the guard warns, pursues, searches, cools
off; a snuffed lamp is relit; the clerk barks.

| ID | Deliverable | Crates | Depends | Verify |
|---|---|---|---|---|
| AI19 | `ConcernBoard` | `sf_ai` | AI8a, IN14 | exact merge, eviction |
| AI20 | `Notice` drive, `ConcernRules`, Glance/Bark rungs | `sf_ai` | AI7, AI19 | exact scores, rate limit |
| AI28 | Kinds `WeaponDrawn`, `Trespass`, `Sneaking`, `Loitering`, `Darkened`, `Nuisance`, `Fire` + emitters | `sf_ai` | AI20, IN18, IN19 | one concern per trigger |
| AI21 | Discovery by tier | `sf_ai` | AI10, AI11, AI19 | tier invariance |
| AI22 | `Investigate`/`Confront`/`Report` walking reporter on crime `ReportState`; stoppable | `sf_ai` | AI20, LW2b, LW9 | stopped reporter files nothing |
| AI29 | Guard heat: Warn, Pursue to LKP, Search, cool-down | `sf_ai`, `mw_ai` | AI8f, AI22, LW5 | pursuit ends, bounty kept |
| AI23 | Suspicion, `Suspects` fact | `sf_ai` | AI13, AI19 | exact; no bounty |
| AI30 | `BarkCue` from rungs, greetings, combat, flee | `sf_ai` | AI20, DQ-23 | exact cue order |
| MWR1 | Morrowind reaction rows, vandalism offence, Classic | `mw_ai` | AI19-AI23 | exact rows; owner **AI-5** (shop theft, relight) |
| AIM2 | Concern-check ms per tier | measurement | AI21 | measurement test |

### Stage 4: carrying and errands

Ends with: a trader carries a crate from the dock to his shop at dawn; your knocked-over chair is set back.

| ID | Deliverable | Crates | Depends | Verify |
|---|---|---|---|---|
| AI31 | `Pick`/`Place`/`Give` over `HeldProp` (one thing, slower) | `sf_ai` | IN9, AI14 | headless plan; prop in hands |
| AI32 | `Errand` directive (fetch, carry, deliver) from schedules, quests, scripts | `sf_ai` | AI31, AI11 | tier-0 outcome = tier-2 |
| AI24 | `Tidy` / replace over `Relocate` | `sf_ai` | IN9, AI21 | tier invariance |
| MA11 | Restock and porter patterns as data | `mw_ai`, data | AI32 | owner **AI-6** (Balmora market dawn) |

### Stage 5: traversal and 3D movers

Ends with: a guard follows you over a wall and off a ledge; racers loop Balmora's towers; fish chase you in the Odai.

| ID | Deliverable | Crates | Depends | Verify |
|---|---|---|---|---|
| AI33 | Link operator (mantle, vault, drop, jump, ladder) via `sf_action`; failure raises link cost | `sf_ai`, `sf_ai_bevy` | NV9, AL-10, AL-12 | 1.2 m box crossed; failed link avoided |
| AI34 | `FlyTo` on the clearance field | `sf_ai`, `sf_ai_bevy` | NV13 | never under clearance |
| AI35 | `SwimTo` inside `[bed + r, surface - r]` | `sf_ai`, `sf_ai_bevy` | NV13 | stays in band |
| MA12 | Racers, slaughterfish, levitation; PGRD fallback gone | `mw_ai`, `mw_walkers_bevy` | AI34, AI35 | owner **AI-7** |

### Stage 6: combat tactics

Ends with: three Balmora guards, two attacking, one circling; a wounded bandit flees to his friends.

| ID | Deliverable | Crates | Depends | Verify |
|---|---|---|---|---|
| AI15 | Combat drive, target choice, HTN engage/reposition/search over C6 | `sf_ai` | C6, AI8f | duel, tokens respected |
| AI36 | Ring slots (nav-checked), roles, alert sharing | `sf_ai` | AI15, AI8e | non-holders circle on reachable slots |
| AI37 | Morale: flee, yield, surrender | `sf_ai` | AI15 | exact thresholds |
| MA10 | Morrowind sheets via C6, Fight/Flee mapping | `mw_ai` | AI15, MA4 | no NaN; owner **AI-8** |

### Stage 7: followers and companions

Ends with: a companion walks beside you through a door, waits on command, holds fire while you sneak, is knocked down.

| ID | Deliverable | Crates | Depends | Verify |
|---|---|---|---|---|
| AI38 | `Follow`/`Escort`: slots, personal space, unseen catch-up | `sf_ai` | AI0d, AI12 | never blocks the leader; catch-up unseen |
| AI39 | Companion commands as host commands | `sf_ai` | AI38 | exact mapping |
| AI40 | Command wheel | `sf_rpg_ui` | AI39 | with AI-9 |
| AI41 | Stealth and combat rules, downed flag | `sf_ai` | AI38, AI8b, C2c | downed not killed |
| AI42 | Companion remarks, paired-slot chatter cues | `sf_ai` | AI30, AI14 | exact cooldown |
| MA9 | Follow/escort/pursue through doors, fallback | `mw_ai` | AI38, AI12 | Census door; owner **AI-9** |

### Stage 8: memory, gossip, reputation

Ends with: rob a Balmora house; two days later a Caldera guard greets you coldly because a traveller told him.

| ID | Deliverable | Crates | Depends | Verify |
|---|---|---|---|---|
| AI13 | Fact store, relations, decay, vendettas, `sf/aimemory` | `sf_ai` | AI5 | exact decay, round trip |
| AI43 | Gossip swaps (hops, confidence, tier-0 edges) | `sf_ai` | AI13, AI27 | exact spread; tier invariance |
| AI44 | Reputation per faction and settlement | `sf_ai` | AI43 | exact thresholds |
| AI25 | `ConsequenceRules` | `sf_ai` | AI13, AI20 | exact; save bounded |
| MA13 | Fact kinds, reputation to disposition, Classic = fame | `mw_ai`, `mw_rpg` | AI44 | owner **AI-10** |

### Stage 9: operators and road travellers

| ID | Deliverable | Crates | Depends | Verify |
|---|---|---|---|---|
| AI45 | `Operate`: wait at stop or aboard, ride, alight, defend or flee | `sf_ai` | AI14, TV1, TV4 | absent operator closes the stop |
| AI46 | Long `GoTo` on NV12: tier-0 time, materialize on the road unseen | `sf_ai` | AI10, NV12 | invariance; owner **AI-11** (Balmora-Caldera road) |

**Later:** AI16 net (after N2), AI17 Luau, AI18 advisor + cosmetic chatter + intent-to-action (research §6),
optional `sf_crowd` (Q2), Nemesis content.

## 5. Spec changes (before the build slices)

- **ai.md:** AI0b, AI0c, AI0e, AI0f; drop non-goal "rumour kits" and "fliers steer with probes"; add AI26-AI46.
- **Followers** (AI0d): in ai.md, or `followers.md` if over ~6 KB.
- **world-reactions.md:** AI22 consumes crime `ReportState`; AI28 kinds; `BarkCue`; AI14 = IN8.
- **ai-adapter.md:** bounty on report delivery, not at `reportCrime` time; MA11-MA13; PGRD fallback dropped.
- **Other lanes:** crime LW2b (`ReportState`, `Recognition`, heat), IN0 claim priority + slot kinds, NV9 link contract,
  NV13 volumes (nav.md non-goal removed).

## 6. Dependencies on other lanes

| Lane | AI needs | AI provides |
|---|---|---|
| Crime | LW2b report/recognition/heat, LW4 `DetectionRules` (+noise, disguise), LW5, LW10 | AI8a-d Meter for LW4; `WitnessQuery` impl retiring `ScaffoldWitnesses` (LW9); AI22 reporter; AI29 heat; AI23; AI43 `Wanted` spread; MA5, MA6 |
| Interaction | IN0 (priority, door/ladder/workstation slots), IN2, IN3, IN9, IN14, IN16, IN17 fire/wet, IN18 `NoiseEvent`, IN19 `Lit` + snuff/relight (IN18, IN19 proposed) | AI14 = IN8; AI31; AI24; AI27 dusk lamps; AI28 reactions |
| Combat | C6, C2c knockdown, action `noise` | AI15, AI36, AI37, AI8e, MA10 |
| Travel | TV1, TV4 | AI4 + AI15 (TV9 ambush), AI6 (TV17), AI10 `fast_forward` (skip, TV21), AI45, AI46 |
| Dialogue-quests | DQ-23 rule DB, DQ-25 presenter, scripts running AI functions | MA2 + `GetAIPackageDone`, MA4 Hello, MA5 `GetDetected`, AI30/AI42 cues, AI43/AI44 facts for DQ-23, AI11 validator |
| Nav | NV5 multi-anchor, NV7b, NV9 (+auto drop/jump), NV11, NV12 road graph, NV13, corridor stuck monitor | tests and first users: AI10/AI46, AI33, AI34/AI35, AI38 |
| Anim | carry clips (MG7b), sit/sleep/work loops, NPC mantle/vault (AL-10, AL-12), creature fly/swim modes, look-at | activity and look-at targets; traversal via `Command` |

## 7. Owner questions

1. **Per-NPC awareness indicator** (white/yellow/red over the NPC, direction wedge off-screen) in Modern? Recommended:
   yes; Classic shows only the sneak icon (shared with crime LW8).
2. **Ambient crowd extras** for big generated cities? Recommended: parked; framework option later, off for Morrowind.
3. **Sleeping NPCs** in beds at night, hearing only loud noise (night burglary as stealth play)? Recommended: yes in
   Modern (KCD2, Skyrim); Classic keeps vanilla placement.
4. **Storms empty the streets** (commoners shelter; guards, travellers stay)? Recommended: yes in Modern.
5. **Companion opinions:** remarks on your deeds, and may they leave? Recommended: remarks yes; leaving only where a
   pack's content says so.

Owner answers 2026-11-25: Q1 a Skyrim-style sneak eye (how noticed you are overall), no per-NPC markers. Q2 later (parked). Q3 yes, except NPCs whose
schedule works at night (guards, night trades). Q4 yes, and people find cover where they can. Q5 much later (parked).
