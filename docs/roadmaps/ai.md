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
5. **No needs system** (KCD2, RDR2): 2-3 considerations (tired at night, hungry at meals) bias smart-object picks;
   they never feed offence or combat options (ai.md "Drives": no escalation ladder, owner 2026-11-26).
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
| AI0b | ai.md "Perception" rewrite (decisions 2, 3, 14) | docs | - | **done 2026-11-25** (ai.md "Perception and stimuli" 145-323: bands, light, cost; checked 2026-11-26) |
| AI0g | `PerceptionProfile` per individual: multipliers + timed states; Morrowind mapping in ai-adapter (DECISIONS 2026-11-25) | docs | AI0b | **done 2026-11-25** (ai.md 195-235, ai-adapter 105-134) |
| AI0c | ai.md interrupt table, build order, tier additions, 3D movers, industry fixes (KCD = BTs, not GOAP) | docs | - | **done 2026-11-26** (ai.md "Interrupt contract", 3D movers, tiers, industry) |
| AI0d | Followers and companions section (directive, commands, stealth, downed; adapter `AI_F`/`AI_E`) | docs | - | **done 2026-11-26** (new `followers.md`) |
| AI0e | ai.md "Memory": gossip, reputation per faction and settlement | docs | - | **done 2026-11-26** (new `ai-memory.md`) |
| AI0f | ai.md errands, `Operate`, link-operator contract | docs | AI0c | **done 2026-11-26** (ai.md "Errands, operators and links") |

### Stage 1: NPCs see and hear

Ends with: a Balmora guard spots a sneaking player in light, not in shadow, checks a thrown cup, calls his partner.

| ID | Deliverable | Crates | Depends | Verify |
|---|---|---|---|---|
| MA2 | AI_* records + MWScript AI functions to directives, OpenMW `StackPolicy` (split a-d 2026-11-26) | `mw_ai` | AI4 | exact vs OpenMW cases |
| MA2a | `mw_ai::packages`: AI_W/T/E/F/A records -> `DirectiveSpec` (priority, repeat, limit as OpenMW reads ESM packages) | `mw_ai` | AI4 | **done 2026-11-26** (9 exact tests; record packages priority 0) |
| MA2b | `mw_ai::policy`: OpenMW `AiSequence::stack` as a `StackPolicy` (priority insert, cancel rules, repeat to the end) | `mw_ai` | MA2a | **done 2026-11-26** (10 exact tests; mw_ai 34) |
| MA2c | MWScript `AIWander/AITravel/AIEscort(Cell)/AIFollow(Cell)/AIActivate` push, `GetAIPackageDone`/`GetCurrentAIPackage` read the stack (split 1/2 2026-11-26) | `mw_rpg`, `mw_rpg_bevy` | MA2b, DQ-4 | script test per function |
| MA2c1 | Bevy-free: the AI functions become script effects / host API calls (`mw_script` rows live, `mw_rpg` runner), reads through a host trait | `mw_script`, `mw_rpg` | MA2b | **done 2026-11-26** (`AiCall` / `Host::ai`, `GameOp::Ai { target, push }`, read trait `MwAi`; mw_rpg 133) |
| MA2c2 | Game host applies them to `AiDirectives` (MorrowindPolicy) and answers `GetAIPackageDone`/`GetCurrentAIPackage` | `mw_rpg_bevy`, `mw_walkers_bevy` | MA2c1, MA2d2 | **done 2026-11-26** (3 walker + 1 host test: script AITravel cancels the wander, walks 8 m, reads not-done/Travel -> done/Travel -> done/none) |
| MA2c3 | Script AI follow-ups: save `AiRuns` (OpenMW saves `mDone`/`mLastAiPackage`), a dead actor reads -1, pushes to unspawned actors land on their stored stack | `mw_walkers_bevy`, `mw_saves_bevy` | MA2c2 | exact per case; save round trip |
| MA2d | Game: spawned NPCs get their record packages on their `DirectiveStack`; wander reads `StayNear` instead of the record (split 1/2 2026-11-26) | `mw_walkers_bevy` | MA2a | headless: Travel package NPC walks to its point |
| MA2d1 | At walker spawn, build the actor's stack in `AiDirectives` from `mw_ai::packages` + `MorrowindPolicy` (a saved stack wins); no behaviour change | `mw_walkers_bevy` | MA2b | **done 2026-11-26** (3 headless tests; every actor gets a stack, empty if no packages) |
| MA2d3 | Package target resolver: a player `ActorKey` convention (AI_F/AI_E player), AI_A placements through a placement index, retry targets that weren't spawned at seed time | `mw_walkers_bevy`, `sf_ai` | MA2d1 | **done 2026-11-26** (1 exact + 2 headless tests: `ActorKey::player` in sf_ids; follow player seeds; late leader inserted at its record position; AI_A via the R0 `PlacedObjects` index) |
| MA2d2 | The stack's top drives the routine: `StayNear` -> wander, `GoTo` -> walk there, finish + advance on arrival / `Limit` (`DirectiveStack::serve/finish/advance`) | `mw_walkers_bevy` | MA2d1 | **done 2026-11-26** (2 headless tests: GoTo 8 m finishes, stack 2 -> 1, wander resumes; record wander unchanged) |
| MA2d4 | Directive drive follow-ups: StayNear home = where the package starts (OpenMW AiWander), `AiPackageExtras` survive a repeat's new id, one duration count (drop `WanderDriver`'s own AI_W timer), a StayNear actor without `Walker` gets one, an empty stack after a GoTo stands at the destination (OpenMW) instead of resuming the record wander at spawn | `mw_walkers_bevy` | MA2d2 | **done 2026-11-26** (5 headless + 1 exact test: wander home = arrival point, repeat keeps extras + home, AI_W ends once at its Limit then stands, walker given, empty stack stands after GoTo) |
| AI8a | Awareness core: meter, bands, `LastSeen`, bounded per-player entries, `Stimulus::Noise` | `sf_ai` | AI0b | exact curves, eviction, serde |
| AI8g | Profile core: `PerceptionProfile`, timed `PerceptionStates`, `EffectivePerception` resolve | `sf_ai` | AI0g | **done 2026-11-25** (13 exact tests) |
| AI8b1 | Sight maths core: gain terms, cone and range from `EffectivePerception`, stagger, ray queue with cap, cache rule | `sf_ai` | AI8g | **done 2026-11-25** (14 exact tests) |
| AI8b2 | Sight system: grid prefilter, batched physics rays, profile/record registration, `ai.perception.*` binding | `sf_ai_bevy` | AI8b1 | **done 2026-11-25** (0.037 ms/tick, 48×4, fake LOS) |
| MA14 | Sight wiring in the game: `AiSightPlugin::<Physics>`, `AiObserver` on NPCs, `Perceivable` on players, record load (forced checks on damage/notify = MA15) | `mw_walkers_bevy`, `mw_mods` | AI8b2 | **done 2026-11-25** (2 headless tests on real physics) |
| MA15 | Forced sight checks (`SightTimer::force`) on damage taken and on `Notify` arrival | `mw_ai`, `mw_walkers_bevy` | MA14, C17 (NPC/creature fighters: health from records, death via walkers ragdoll; NPCs take no damage before it), AI8c (Notify arrives by hearing) | headless: hit from behind turns the NPC's check due this tick |
| AI8c1 | Hearing system: `NoiseEvent` message, listener prefilter by reach, `NoisePropagation` seam (default straight line + door penalty from the host, UE AIPerception), delivery as `HeardNoise` + `Notify` | `sf_ai_bevy` | AI8a, AI8e | **done 2026-11-25** (9 headless tests) |
| AI8c2 | Player emitters in the game: gameplay stride footsteps by gait, landing (surface/gear seam ×1; NPC steps, action `noise: Loud` later) | `mw_walkers_bevy` | AI8c1 | **done 2026-11-25** (7 tests; run 12 m, sneak 2 m reach) |
| AI8c3 | Propagation by `portal_query::path_distance` (doors counted): `NoisePropagation` over `NavWorldGraph` (NV11a-d1 done 2026-11-26) | `sf_ai_bevy`, `sf_nav` | AI8c1, NV11 | exact through a door; **done 2026-11-26**: `PortalPaths` default, cross-space listeners, `noise_max_hops` 3, 7 tests |
| MA21 | Game adds `NavWorldGraphPlugin` over `DoorLinks`, approach points from pathgrid nodes in front of each door, seamless/fade flags from `world.doors`, door states from the door objects | `mw_walkers_bevy` or `mw_exterior_bevy` | NV11d1 | **done 2026-11-26** (`nav_links.rs`, 5 tests; vanilla 1828 links, 77% of ends on PGRD, 121 locked, 24 ms at load; census office 1 hop; flags classic until `world.doors`) |
| MA21b | Walkers read the shared pathgrids resource instead of their own copy (`mw_walkers_bevy` walkers.rs:351), drop the duplicate build | `mw_walkers_bevy`, `mw_exterior_bevy` | MA21 | build once, counter |
| AI8d1 | Light core: `VisibilityFactors`, pure `LightExposure` (sky term + strongest sources, LOS callback, roof ray), `darkness` term | `sf_ai` | AI8a | **done 2026-11-25** (14 exact tests) |
| AI8d2 | Light system: one sample per player per tick, host source list + sky resource (until IN19 `Lit`), roof ray, feeds sight in place of `VISIBILITY_UNMODELLED` | `sf_ai_bevy` | AI8d1, MA14 | **done 2026-11-25** (9 headless tests; inert until a host inserts `AiLightInputs`) |
| MA16 | Light inputs in the game: sky level from clock/weather/interior, Morrowind lamps (LIGH placements, carried torches) as sources | `mw_walkers_bevy` | AI8d2 | **done 2026-11-25** (2 tests; lit lamp at 1 m 0.7244, noon 1, midnight 0) |
| MA17 | Twilight term (sky reads 0 within ~0.5 h of sunset: no fill below the horizon), scripts switching lights, `level:` interiors | `mw_walkers_bevy`, `sf_sky` | MA16 | exact dusk curve |
| MA18 | Player sneak in sight (owner 2026-11-25): `VisibilityFactors` from the game (stance Crouched in `Mode::Sneak`, speed, sneak skill and chameleon as modifiers), so Ctrl hides you as well as quietens steps | `mw_walkers_bevy` | LW4b1 | **done 2026-11-25** (4 tests; owner **AI-12**) |
| MA19 | Rest of the target's visibility: Chameleon as a magnitude stat (`mw_stats::MAGNITUDE_STATS`, `mw_effects`), Invisibility as a sight gate, size from race height | `mw_walkers_bevy`, `mw_stats`, `mw_effects` | MA18 | exact per case |
| AI8e | Shared knowledge: `Notify` from shouts and alarms | `sf_ai` | AI8a | **done 2026-11-25** (9 exact tests) |
| AI8f1 | Search core: seeded search points around LKP (`error_m`, `widen_mps`), look/widen/give-up state on `ttl_s` from `EffectivePerception.search`, HTN operators/method registered | `sf_ai` | AI8a, AI6, AI8g | **done 2026-11-25** (12 exact tests) |
| AI8f2 | Search in the Bevy layer and game: Searching band starts the method, NPC walks to LKP, looks, widens, quits | `sf_ai_bevy`, `mw_walkers_bevy` | AI8f1 | **done 2026-11-25** (3 headless tests; LKP at 5.4 s, give-up at ttl tick) |
| AI8f3 | Hold the Searching band's decay while a search runs (TLOU2/Splinter Cell: the search, not the meter, ends Searching; today decay drops the band at 5.5 s, ttl 30 s) | `sf_ai`, `mw_walkers_bevy` | AI8f2 | **done 2026-11-25** (band leaves Searching 2.5 s after the ttl give-up) |
| AI8h1 | Band reactions core (ai.md 163: Curious = turn to face the stimulus, Suspicious = turn + step toward LKP, then hold and look): `sf_ai::react` goal from band + `LastSeen`, tunables (step distance, face delay, hold) | `sf_ai` | AI8a, AI8f1 | **done 2026-11-25** (11 exact tests; sf_ai 149 pass) |
| AI8h2 | Band reactions in the game: AI8i's `Reaction` layer gets Curious/Suspicious phases (seam: `react` beside the `SearchMode::Run` branch, `preempt` keeps the anchor, `OpRunner` runs `ReactGoal::operators`, then `start_return`); bind the 4 `ai.perception.react_*` tunables + CATALOG (owner 2026-11-25: "no guard is ever moving" in the lab) | `mw_walkers_bevy`, `sf_ai_bevy` | AI8h1, AI8i | **done 2026-11-25** (headless: Curious 0.000 rad off, 0 drift; Suspicious 6 -> 3 m from LKP; back at post 0.00 m); owner AI-11/AI-12 open |
| AI47c | Diagnosis: the lab's search guard never searched in the owner run (60 level tiles built, walkers 0 moving); headless repro through the real level + walkers stack, cause in a ledger line | `mw_walkers_bevy`, `mw_exterior_bevy` (tests) | AI47b | **done 2026-11-25** (cause: level `script` removes `Walker` (actors.rs:259); search only runs inside the walker drive; ignored failing-first test) |
| AI8i | Perception reactions independent of wander: search (and AI8h2 reactions) drive any `AiActor` with a `NavCorridor`, pre-empting its routine (wander, stand, level path) and returning it to its post after; stationary (AI_W 0) NPCs and level actors get it; level path actors stop fighting the corridor over `MoveIntent` | `mw_walkers_bevy`, `mw_exterior_bevy` | AI47c | **done 2026-11-25** (AI47c test passes; stationary NPC back within 0.01 m / 0 rad; path actor intent fight fixed) |
| AI8j | Reaction walks for stationary and level actors use ORCA avoidance (no `Avoider` today); measure the per-tick cost of idle corridors + `react` on distance-0 NPCs (headless counter, Balmora-sized crowd) | `mw_walkers_bevy` | AI8i | headless: two reacting guards don't overlap; µs/tick printed |
| MA5 | `awarenessCheck` port as Classic gain | `mw_ai` | AI8a | exact vs formula |
| MA4 | Hello/Fight/Flee/Alarm considerations; greeting via AI8 (split a-d 2026-11-26) | `mw_ai` | AI7, AI8b | exact vs OpenMW |
| MA4a | Hello/Fight/Flee/Alarm stats on the sheet from AIDT (appended defs) | `mw_stats` | - | **done 2026-11-26** (indices 140-143, `MwIdx::ai_settings`; floor 0, no cap as OpenMW) |
| MA4b | `Get/Set/Mod{Hello,Fight,Flee,Alarm}` script functions on the sheet | `mw_script`, `mw_rpg` | MA4a | **done 2026-11-26** (2 script tests; mw_rpg 129) |
| MA4e | Persuasion reads Fight/Flee from the sheet, not the record (`mw_rpg_bevy/src/dialogue.rs:662` reads `ai_data`), so script Set/Mod reach it | `mw_rpg_bevy` | MA4b | **done 2026-11-26** (base Flee/Fight from the sheet, nudge written back as `SetBase`; overlay map gone, saves untouched; mw_rpg 135, mw_rpg_bevy 80) |
| MA4c | Greeting consideration (`updateGreetingState`: Hello x `iGreetDistanceMultiplier`, LOS, awareness) + game turn/look-at (split 1/2 2026-11-26) | `mw_ai`, `mw_walkers_bevy` | MA4a, AI8b | exact vs OpenMW; headless greet |
| MA4c1 | Pure greeting state machine `mw_ai::settings::greeting_step` (OpenMW `updateGreetingState`) | `mw_ai` | MA4a | **done 2026-11-26** (14 exact tests; GMSTs 6 / 4 / 512 confirmed) |
| MA4c2 | Game: NPCs turn/look at and greet the player (Greeting bark via dialogue) from MA4c1 | `mw_walkers_bevy`, `mw_audio_bevy` | MA4c1, AI8b | **done 2026-11-26** (`greeting.rs`: stop, turn, Hello bark from the voices; old voice-director greeting removed; owner check AI-13) |
| MA4c3 | Greetings in every build: NPC voices (and so the MA4c2 greeting, which needs a Hello answer) are behind `mw_audio_bevy`'s `dev` feature (`Cargo.toml:10-12`; `VoicePlugin` added under `#[cfg(feature = "dev")]`, `lib.rs:~118-122`) until the owner hears them (AU-3 partial 2026-10-30, AU-5 rejected; AI-13 awaiting owner). Non-dev builds: no NPC greets, stops or turns. Ungate voices once AU-3/AI-13 pass (logged by meta 2026-11-26) | `mw_audio_bevy`, app features | MA4c2, owner AU-3 + AI-13 | release build: `greeting: N greeted` > 0 |
| MA4d | Fight rating: `isAggressive` (Fight + disposition + `getFightDistanceBias`), pure | `mw_ai` | MA4a | **done 2026-11-26** (11 exact tests; + `engages_on_sight` gates) |
| MA4f | Flee rating: `vanillaRateFlee` + `updateFleeing` thresholds, pure | `mw_ai` | MA4a | **done 2026-11-26** (15 exact tests; vanilla never flees below Flee 100 or without a werewolf) |
| MA4g | Peaceful NPCs run the stealth bands against a law-abiding player in plain view: Curious turn, a 3 m Suspicious step, a Search, then Alerted (latched) within ~1 s (measured headless, `mw_walkers_bevy` `greeting_sight_tests.rs`; found with AI-13 2026-11-26). Decide what a non-hostile observer does on plain sight (shipped games fill the detection meter only for a hostile, trespassing or wanted target), spec first; head tracking's "engaged" reads the same band | `sf_ai`, `mw_ai`, `mw_walkers_bevy` | MA5 | spec section + headless: a villager seeing a lawful player keeps its routine |
| AI26 | Perception gizmos (cone, meter, LKP), `ai.perception` | `sf_ai_bevy` | AI8b | **done 2026-11-25** (7 tests; `ai.perception`; dev enables `sf_ai_bevy/debug`); owner **AI-3** |
| AI47 | Perception test scene (owner 2026-11-25, after AI8c/d/f, AI26, IN18): reuse the arena, AI26 gizmos plus NPC state labels and noise/impact rings, scripted light, cover and throw cases | `bevyrrowind` (`level:`) | AI26, AI8c, AI8d, AI8f, IN18 | **done 2026-11-25** (`levels/perception-lab.ron`, 6 scenes; impacts wait for IN18); owner **AI-11** |
| AI47b | Level navmesh from the level's own geometry, so level actors wander/search (search can't walk in `level:` scenes today) | `mw_exterior_bevy`, `sf_nav_bevy` | AI47 | **done 2026-11-25** (path test; 60 tiles ~80 ms cold debug; guard search in game: owner AI-11) |
| AI48 | List the new perception tunables (`ai.perception.*`, `mw.ai.light.*`, `mw.ai.noise.*`) in `mw_devtools_bevy::perf::tunables::CATALOG` (the game warns once per key at start; `bevyrrowind tunables --md` misses them) | `mw_devtools_bevy` | AI47 | **done 2026-11-25** (74 defs catalogued incl. ai.exec + mw.ai.wander; test; tunables-index regen pending) |

### Stage 2: a day in town

Ends with: Balmora runs dawn to night and through a 3 h wait; lamps lit at dusk; commoners take shelter in a storm.

| ID | Deliverable | Crates | Depends | Verify |
|---|---|---|---|---|
| AI9 | Allocator: tiers 2/1, `sim.cpu_tier` caps, pins, staggered first update (split a/b 2026-11-26) | `sf_ai`, `sf_ai_bevy` | AI7 | exact + hysteresis; cost line |
| AI9a | Bevy-free allocator core `sf_ai::alloc` | `sf_ai` | AI7 | **done 2026-11-26** (9 exact tests; sf_ai 158) |
| AI9a2 | Hard caps Full/Reduced: over cap, the lowest-ranked visible actors drop to Reduced; in-view hold only protects Reduced -> Logical (decision ai-as-built AI9a) | `sf_ai` | AI9a | **done 2026-11-26** (11 alloc tests; sf_ai 160) |
| AI9b | `sf_ai_bevy` system: inputs from players/visibility/pins, tunables `sim.cpu_tier` + caps + margins, tier component, cost line | `sf_ai_bevy` | AI9a | **done 2026-11-26** (6 headless tests; 2000 actors 555 µs/allocate release: too slow -> AI9c) |
| AI9c | Allocator cost: Vec-based core (one partial select per band, no BTreeMaps, reuse buffers) and an allocate period `ai.alloc.period_s` (tier changes are rare; MassLOD/significance-manager style) | `sf_ai`, `sf_ai_bevy` | AI9b | **done 2026-11-26** (2000 actors: 557 -> 64 µs mean, 902 -> 155 max; system 679 -> 161 µs/tick; no period needed) |
| AI9d | `sim.cpu_tier = auto` (default): pick the tier from physical core count + headless (Dedicated), the spec's reference rows | `sf_ai_bevy` | AI9b | **done 2026-11-26** (3 tests; threads <=8/12/16/>16 -> Floor/Medium/High/Ultra, headless host flag -> Dedicated) |
| AI9h | `ai.stats` / `ai.tiers` console lines (counts per tier, over-cap, held in view, allocate µs) | `sf_ai_bevy` | AI9b | **done 2026-11-26** (console test; `ai.tiers` from `AiAllocStats`, `ai.stats` alloc line) |
| AI9e | Tiers take effect on sight: observer sight period from `AiTier` (Full `sight_period_s`, Reduced `tier1_period_s`, Logical no sight and no hearing), first check at `first_decide_tick`; exec trace tier from `AiTier` | `sf_ai_bevy` | AI9c | **done 2026-11-26** (5 headless tests: checks over 600 ticks none/Full/Reduced/Logical = 100/100/10/0) |
| MA20a | Game writes `AiInView` from each player's `AiViewer` frustum (gameplay viewer, not render visibility) | `mw_walkers_bevy` | AI9b | **done 2026-11-26** (`in_view.rs`; 7 exact + 2 headless tests: front/behind swap on a 180° turn, seen one takes the only Full slot; `mw.ai.view.{margin_m,max_m}` 2/60 m) |
| MA20b | Dialogue partner `AiPin(Pin::Dialogue)` from `mw_rpg_bevy`'s talk session; combat/scripted pins when those run in game | `mw_rpg_bevy` | MA20a | **done 2026-11-26** (`dialogue/pin.rs`; 3 headless tests: partner Full at a Full cap of 0, pin moves to a new partner, gone on close, Combat pin untouched) |
| AI9f | Tier 1 hearing by straight-line distance only (no `NoisePropagation` query) | `sf_ai_bevy` | AI9e | **done 2026-11-26** (1 headless test; same-space rule via `AiExecutor.worldspace`) |
| AI9g | Decide period by tier for the drive/HTN decide step once drives run in the game (Full `full_decide_s` from `first_decide_tick`, Reduced `reduced_decide_s`) | `sf_ai_bevy` | AI9e, drives in game | **done 2026-11-26**: wander HTN plans on decide ticks (`ExecContext::decide`), events at once; a tier-0 body (`AiLogicalBody`) stands while its state runs at tier 0 (AI10c2) |
| AI10a | Tier 0 core: `LogicalState` stepped at events (arrival, limit, window-edge hook), per-kind transitions + `LogicalTransitions::validate` (custom kinds fail at load), `LogicalEnv` seam, `fast_forward` + `FastForwardRules` hook | `sf_ai` | AI9 | **done 2026-11-26** (`logical.rs`; 7 exact tests: arrival ends `GoTo` at its minute, tier 0 vs per-tick executor same ends within one tick, StayNear 2 h one event, `fast_forward` 3 h = event stepping, Follow/Escort, Fight holds, registry) |
| AI10b | Promotion resolve queue (nearest-first, `ai.tier.promote_per_tick`, pins jump) + placement at the state's position snapped on the route (seen spots wait, MA20a `seen_by_any`) + demotion | `sf_ai` | AI10a, MA20a | **done 2026-11-26** (`sf_ai::promote`: `PromoteQueue` + `resolve_promotion` + `demote`, `PlayerViews`/`PromoteEnv` seams; 12 exact tests; not in game until AI10c) |
| AI10c1 | Tier-0 shell: `AiLogical` store by `ActorKey` (an actor is tier 0 in fact while it is in it), step helper on the fixed tick (leaders before followers), promote-queue shell on AI10b (queued actors keep `Tier::Logical`; a placement where the body already stands needs no view test), `ai.tier.promote_*` tunables, `ai.stats` line | `sf_ai_bevy` | AI10a, AI10b | **done 2026-11-26** (`sf_ai_bevy::logical`: `AiLogical` store + due index, `AiLogicalBody`, `AiTierZero::{step, promote}`, Hold/Step/Promote/Cleanup sets, body-on-spot placement, `ai.tier.promote_*`, `ai.stats` line; 10 headless tests on a fake env; not in game until AI10c2) |
| AI10c2 | Game wiring: `LogicalEnv` over `NavWorldGraph` (straight legs until NV12f loads the route graph), `PromoteEnv` over `AiViewers` + navmesh nearest; a tier-0 body stays rendered and is re-placed only unseen (position + `Mover::snap` + corridor reset, same worldspace); `drive` / `drive_directives` stand down; removes AI9g's "Logical walkers stand" | `mw_walkers_bevy` | AI10c1, NV11 | **done 2026-11-26** (a: `WorldGraphEnv`, `ViewsEnv`; b: `logical::add` enter / step / promote on the fixed tick with a `GameClock`, body placement same worldspace only, `drive` / `drive_directives` stand down on the marker, wakes after pushes, `tier0:` proof line; 7 headless game tests incl. tier invariance); owner scene AI-14 awaiting |
| AI10c3a | An actor keeps its position and facing across its cell's unload and reload: `ActorPoseHook` on the actor class writes a moved body (> 1e-3 m planar or 1e-3 rad) to the overlay at deactivation (not on discard); `persist_actor_poses` for saves (MA22) | `mw_walkers_bevy` | AI10c2 | **done 2026-11-26** (`poses.rs`; 7 headless tests: write, no write unmoved/discard, overlay loads it in the new cell, turn round trip, `attach` respawn pose, tier-0 re-mark keeps the state, save copy); gap: a body that crossed into a loaded neighbour despawns with its home cell until the neighbour reloads (AI10c3b); owner scene AI-15 awaiting |
| AI10c3b1 | `Streaming::rehome(ws, id, from, to) -> Option<Rehomed>`: one live placement moves from loaded cell A's Active list to loaded cell B's (bookkeeping only; returns B's near group + origin, the caller swaps `ChildOf` + local transform); refused while a stage of either cell runs, B has no near group, or a scene covers the id | `sf_world_bevy` | AI10c3a | **done 2026-11-26** (4 headless tests: hand-over to B with B's deactivation once and B's despawn, Near-only B, refusals, stage in flight); note (not a ROADMAP item): physics items have the same gap (`sf_physics_bevy/src/stream.rs` `ItemPivot`), a later slice on the same API |
| AI10c3b2 | Walkers re-home a body at a cell crossing: per-frame cell test, `Streaming::rehome`, `ChildOf` + local transform + `Mover.frame_origin`, overlay pose write at the crossing so no cell spawns a second body | `mw_walkers_bevy` | AI10c3a, AI10c3b1 | **done 2026-11-27** (`crossing.rs`: `BodyCell` set by `attach`, `rehome_crossers` after the streaming drive, one entity command for parent + translation + frame origin; `Held` bodies wait for their release; 8 headless tests on a hand-built cell tree with a stand-in `rehome`); gaps: a body whose placement no Active list and no overlay entry holds stays in its home cell (counted in the proof line); the test on a real `Streaming` is AI10c3b3; owner scene AI-16 awaiting |
| AI10c3b3 | Real-wiring test of AI10c3b2: the real `rehome_crossers` against a real `Streaming` with two adjacent loaded cells: the body crosses, its home cell unloads, the body lives under the neighbour; the home cell reloads: exactly one entity carries that placement | `mw_walkers_bevy` (tests) | AI10c3b2 | **done 2026-11-27** (`crossing_wiring_tests.rs`, one test, 0.2 s, no manifest change: swap lands, the body outlives its home cell's near unload, the home cell's reload spawns no twin, after both unload the neighbour respawns it at the crossed position); not covered: the game's own registration order (`.after(processing_range)`), the real `CharacterSpawner` / `attach`, `interpolate` |
| AI10c3c1 | `Streaming::spawn_into(world, spawners, ws, cell, placement) -> Option<SpawnedInto>`: one placement spawned into an already loaded cell (batch under its near group; in an Active cell also `activate` + the Active list); refused while a stage of the cell runs, when the id is listed or scene-drawn there, without a spawner or a ready model | `sf_world_bevy` | AI10c3b1 | **done 2026-11-27** (3 headless tests: Active cell with its deactivation once and its despawn, Near-only cell, refusals); gaps: starts no model load (actors need none), a Near-only cell cannot show a twin from its own load |
| AI10c3c2a | A bodiless tier-0 actor's overlay entry follows its logical position: its placement kept at deactivation, moved to the cell its state stands in when that changes, so that cell's next fetch spawns the body there | `mw_walkers_bevy` | AI10c3a | **done 2026-11-27** (4 headless tests: one write on the tick it enters the next cell, bodies/unkept/unrouted not written, kept map at (de)activation and discard, prune); gaps: actors never loaded since start, followers, loaded cells (AI10c3c2b) |
| AI10c3c2b | A bodiless tier-0 actor whose spot stands in a loaded cell gets its body at once (`Streaming::spawn_into`, at the unseen spot the promotion rules pick), then is re-marked and promoted like any stored body | `mw_walkers_bevy` (`sf_ai_bevy` only if the queue needs it) | AI10c3c1, AI10c3c2a | **b1 done 2026-11-27** (3 headless tests on a real `Streaming`: one body on the tick it enters an Active cell, at its snapped unseen spot, no twin after a reload; a seen spot waits and retries; an unloaded cell spawns nothing); **b2 done 2026-11-27** (proof line `ai.tier0: N bodies spawned into loaded cells, M kept placements moved` + `ai.stats`; owner scene AI-17 awaiting owner) |
| MA22 | `sf/ai` save section registered in the game (`sf_save_bevy::sections::ai` exists; `mw_saves_bevy/src/lib.rs:201-256` registers none, so directive stacks are not saved), with tier-0 states once AI10c1 lands | `mw_saves_bevy`, `sf_ai` | AI10c1 | save + load round trip keeps stacks and states; **MA22a done 2026-11-27** (stacks saved, `mw_saves_bevy` round-trip test); **MA22b done 2026-11-27** (tier-0 states, section v2) |
| AI12 | Doors: `Traverse`, fade-only unseen rule | `sf_ai`, `mw_ai` | NV11, AI6 | follower through both door modes |
| AI14 | `UseSlot` + fragments (= interaction IN8), claim priority | `sf_ai`, `sf_interaction` | IN0, IN2, IN3 | deterministic claims; IN-8 |
| AI11 | Schedules, `ScheduleRules`, quest-safety validator | `sf_ai` | AI10, AI12 | exact pushes at window edges |
| AI27 | Free-time picker (time, role, 2-3 considerations), ambient keys, shelter and dusk-lamp methods | `sf_ai` | AI14, IN19 | exact picks on a synthetic day |
| MA7a | [x] `fastForward` rules: `MorrowindFastForward` + `allowed_positions` (2026-11-26). Gap: `WanderPlanner`'s allowed set differs from OpenMW's (no connectivity test, no closest removal, no single-node extension) | `mw_ai` | AI10 | exact placement |
| MA7b | The game fast-forwards on a time skip: after `GameClock::skip` every simulated actor that is not unconscious or paralysed is demoted, `fast_forward` with `MorrowindFastForward`, placed while the screen is dark (promotion ignores views for that tick); occupancy from the store + bodies | `sf_ai_bevy`, `mw_walkers_bevy`, `mw_rpg_bevy` | AI10c2, MA7a | headless: skip 8 h, wanderers stand on allowed positions |
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
| AI49 | `Temperament` per individual (flee, freeze, confront, fight, test) as data + role defaults, feeding AI20 response choice and AI37 (owner 2026-11-25, gta6.md GX2) | `sf_ai`, `mw_ai` | AI20, AI0g | exact picks per temperament and role; Classic = Fight/Flee |
| AI50 | Personal-space and tailing concerns: crowd or follow an NPC and it turns and speaks, then leaves or escalates by temperament (gta6.md GX3) | `sf_ai` | AI20, AI49 | exact ladder per temperament |
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
| Interaction | IN0 (priority, door/ladder/workstation slots), IN2, IN3, IN9, IN14, IN16, IN17 fire/wet, IN18 `NoiseEvent`, IN19 `Lit` + snuff/relight (IN17, IN18, IN19 proposed; owned by interaction.md) | AI14 = IN8; AI31; AI24; AI27 dusk lamps; AI28 reactions |
| Combat | C6, C2c knockdown, action `noise` | AI15, AI36, AI37, AI8e, MA10 |
| Travel | TV1, TV4 | AI4 + AI15 (TV9 ambush), AI6 (TV17), AI10 `fast_forward` (skip, TV21), AI45, AI46 |
| Dialogue-quests | DQ-23 rule DB, DQ-25 presenter, scripts running AI functions | MA2 + `GetAIPackageDone`, MA4 Hello, MA5 `GetDetected`, AI30/AI42 cues, AI43/AI44 facts for DQ-23, AI11 validator |
| Nav | NV5 multi-anchor, NV7b, NV9 (+auto drop/jump), NV11, NV12 road graph, NV13, corridor stuck monitor | tests and first users: AI10/AI46, AI33, AI34/AI35, AI38 |
| Anim | carry clips (MG7b), sit/sleep/work loops, NPC mantle/vault (AL-10, AL-12), creature fly/swim modes, look-at | activity and look-at targets; traversal via `Command` |

## 7. Owner questions

1. **Per-NPC awareness indicator** in Modern? **Answered (owner 2026-11-25): a Skyrim-style sneak eye** (reads max
   awareness), not per-NPC markers; see the answers below.
2. **Ambient crowd extras** for big generated cities? Recommended: parked; framework option later, off for Morrowind.
3. **Sleeping NPCs** in beds at night, hearing only loud noise (night burglary as stealth play)? Recommended: yes in
   Modern (KCD2, Skyrim); Classic keeps vanilla placement.
4. **Storms empty the streets** (commoners shelter; guards, travellers stay)? Recommended: yes in Modern.
5. **Companion opinions:** remarks on your deeds, and may they leave? Recommended: remarks yes; leaving only where a
   pack's content says so.
6. **Can the player silence gossip?** Recommended: yes in Modern: bribe or intimidate makes one holder drop one fact;
   people already told keep it; no planting lies (ai-memory.md). **Answered (owner 2026-11-26): as recommended.**
7. **Does a companion told to wait, wait forever?** Recommended: yes; a pack can add a timeout (Skyrim: home after
   3 days). **8. Companions per player in Modern?** Recommended: 4, no cap in Classic (followers.md). **Answered (owner 2026-11-26): Q7 and Q8 as recommended.**

Owner answers 2026-11-25: Q1 a Skyrim-style sneak eye (how noticed you are overall), no per-NPC markers. Q2 later (parked). Q3 yes, except NPCs whose
schedule works at night (guards, night trades). Q4 yes, and people find cover where they can. Q5 much later (parked).
