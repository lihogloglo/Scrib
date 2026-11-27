# NPC AI: directives, drives, HTN behaviours, tiers (`sf_ai`)

Status: slice status lives in [docs/roadmaps/ai.md](../../../docs/roadmaps/ai.md) §4; as-built notes in
[ai-as-built.md](ai-as-built.md). M7. Delegated decisions 2026-11-08: `../research/ai-nav-decisions-2026-11-08.md`.
Crates: `sf_ai` (Bevy-free: directives, drives, HTN, perception model, facts, tiers, trace), `sf_ai_bevy` (thin:
fixed-tick systems, executor glue to `sf_action`/`sf_controller`, debugger, console). Uses [nav.md](nav.md) and
`sf_interaction`. Game side: [ai-adapter](../../../bevyrrowind/docs/specs/ai-adapter.md). Sources:
`../research/npc-ai-2026-11-08.md`, `../research/gameplay-2026-11-25/ai.md`, vision §4/§5/§7, ARCHITECTURE §1b, [action.md](action.md),
[net.md](net.md), [combat.md](combat.md) C6.

<!-- agent-summary -->
> **Agent summary (2026-11-26).** Directives (packages, schedules, quest orders) → utility drives → one shared HTN
> method library → executors writing `Command` + `LocomotionIntent`; host-only, fixed tick. "Interrupt contract" says
> who pre-empts whom and how plans resume. Tiers 2/1/0 under fixed host-CPU caps; tier 0 is a planner-free state
> machine, so outcomes are tier-invariant. Graded perception, a Nemesis-style fact store with gossip ([ai-memory.md](ai-memory.md)), an LLM seam (later).
<!-- /agent-summary -->

## Goal

NPCs and creatures that follow content directives predictably, react, fight, travel across worldspaces (seamless or
fade doors), keep routines off-screen and remember players. Morrowind's packages and settings are adapter data.
Non-goals: an LLM in the core, GOAP, flow fields, classic executors and combat (later seams), economy kits. Gossip is
in scope, bounded: [ai-memory.md](ai-memory.md).

## Owner decisions this spec implements (2026-11-08)

1. `sf_nav` + `sf_ai` (+ `sf_interaction`) split. 2-3. Nav patches (nav.md). 4. Vanilla NPCs get schedules, behind a
"no schedules" seam. 5. Caps are a **host CPU-tier setting**, never graphics; tier 0 simulates logically. 6. LLM later,
out of process. 7. Nemesis-style memory: yes. 8. Combat modern first. 9. One label, M7. Also: GPL-3.0; no MP ceiling (tier by
the nearest player, one global budget); Steam Deck floor. Delegated: fixed provisional caps, bounded cross-worldspace
vendettas, far sides of open doors fully simulated, schedules after tier 0. Lane decisions since 2026-11-25:
[docs/roadmaps/ai.md](../../../docs/roadmaps/ai.md) §3 (6: interrupts, 11: 3D movers, 13: tier additions here).

## Canonical technique

**Authored directives with reactive overrides** (Bethesda packages, OpenMW `AiSequence`), **utility arbitration**
(IAUS), **HTN total-order forward decomposition** with plan monitoring (Humphreys, Game AI Pro ch. 12; Decima; Fluid
HTN), **smart objects**, **significance-based simulation LOD** with logical off-screen state (MassLOD, process levels).

## Reuse

Bevy 0.20 has no AI layer; no crate is Bevy-free, current, serde and typed (research §5.1: `big-brain` archived,
`bevy_bae` 0.18 with string state, BT crates Bevy-coupled, `cranium` MPL). So a small core of our own, **design from
Fluid HTN** (MIT; notice if code is ported) and Humphreys. OpenMW (GPL) is ported only in `mw_*`.

## Architecture

Principle: **content says what, AI decides how, the action layer does it, only the host thinks.**

```
directives (stack per actor: record defaults, schedules, scripts, quests)  ← adapter StackPolicy
   → drives (Directive, Combat, Flee, Pursue, Notice, Converse) scored by considerations, with hysteresis
      → HTN: drive's root task decomposed through the method library (+ smart-object fragments)
         → operators (MoveTo, Traverse, UseSlot, Attack, Guard, Speak, Wait, Face...) → Command + LocomotionIntent
perception (sight, hearing, stimuli; per player) and facts (memory, relationships, crime) feed the world state
tiers (2 full, 1 reduced, 0 logical) decide how often and how precisely each actor runs the above
```

`sf_ai` depends on `sf_ids`, `sf_world` (`Location`), `sf_nav` (core), `sf_records`, serde; not on Bevy, Rapier or
`sf_action` (executors emit plain intents the shell turns into `Command`).

## Data model

All saved/replicated types are serde; ids are stable (`ActorKey`, `PlayerKey`, `PlacementId`, `LinkId`, record ids).

```rust
// sf_ai (Bevy-free). Illustrative, not compiled.
pub enum DirectiveKind { StayNear { home: Location, radius: f32 }, GoTo(Location), Follow(ActorRef),
    Escort { who: ActorRef, to: Location }, Use(PlacementId), Fight(ActorRef),
    Errand(Errand), Operate(Operate) /* AI0f */, Custom(RecordId) }
pub struct Directive { pub id: DirectiveId, pub kind: DirectiveKind, pub priority: u8, pub repeat: bool,
    pub until: Option<GameTime>, pub source: DirectiveSource /* Record | Schedule | Reaction | Memory | Script | Quest */,
    pub params: ParamBag, pub done: bool }
pub trait StackPolicy: Send + Sync {       // adapter: OpenMW's insertion, cancel and repeat rules
    fn push(&self, stack: &mut Vec<Directive>, d: Directive);
    fn on_done(&self, stack: &mut Vec<Directive>, id: DirectiveId);
}
pub struct WorldState { bits: BitSet, ints: SmallVec<[i32; 16]>, refs: SmallVec<[Option<Target>; 8]> } // keys from a load-time registry
pub struct Method { pub task: TaskId, pub priority: u16, pub pre: Vec<Predicate>, pub subtasks: Vec<TaskRef> } // effects: primitives
pub fn decide(ws: &WorldState, stack: &[Directive], lib: &Library, rng: u64, trace: Option<&mut TraceEntry>) -> Decision; // pure
```

- **Records** (mod-patchable): `sf.ai.method`, `sf.ai.consideration` (input, curve, weight), `sf.ai.drive`,
  `sf.ai.archetype` (parameters + method overrides, like package templates), `sf.ai.schedule`, `sf.ai.fact_kind`.
  Predicates, inputs and operators come from a registered vocabulary; unknown names fail at load with a report.
- **World state** is typed slots, not strings: names resolve to indices once at load. Plans record which keys they
  read, so a change marks them dirty.
- **Followers** ([followers.md](followers.md), AI0d): `Follow` becomes `Follow { leader, slot, rules }` (AI38); it is a
  row-5 directive at its source's rank, player commands are its params, its leash shapes row 1.

## Directives (the §1b seam)

- Sources, lowest to highest by default: record defaults (an NPC's packages), **schedules**, memory-driven goals
  (vendetta), scripts, quests. The adapter's `StackPolicy` decides insertion, cancelling and repeat (Morrowind: the
  OpenMW `aisequence.cpp` rules). Completion (`done`) is the signal scripts poll (`GetAIPackageDone`).
- **Schedules** (`sf.ai.schedule`): `{ days, from, to, directive template, place }`, `place` a `Location` or an
  adapter-resolved anchor ("home", "work", a slot tag); the host clock pushes and retires `Schedule` directives at
  window edges. Seam `ScheduleRules` (modern = records; classic = none). Quest safety (Radiant's lesson): a validator
  lists actors that quests address by position, so packs can exclude them.
- **Timing** (decision (e)): `ScheduleRules::None` until AI10-AI12 (tier 0, doors); an unresolved anchor keeps the
  record directives.
- Directives are fully saved; the stack is the truth across tiers.
- **As built (AI4):** `sf_ai::directive` — limits, `Serving`, instance ids: [ai-as-built.md](ai-as-built.md) "AI4".

## Drives (utility arbitration)

Each consideration maps an input (threat distance, health ratio, a stat, disposition, stimulus) through a response
curve; a drive scores the compensated product (IAUS) times its weight. The running drive gets momentum and a minimum
commit time; *Directive* has a floor so content wins unless a reaction clearly beats it. Morrowind's Hello, Fight,
Flee and Alarm rules are adapter considerations.
- **No escalation ladder** (owner 2026-11-26; Radiant AI's failure: a need with a ladder of means, pockets → steal →
  kill). An offence option (take owned, loot, vandalise, attack) is scored only for actors whose role or temperament
  allows it, on its own considerations. A lawful option failing or missing never raises an offence's score, and
  comfort considerations (hunger, fatigue, weather) never feed offence or combat options: no option, nothing happens.
- **As built (AI7):** `sf_ai::drive` — records, curves, `arbitrate`, `serving_for`, `ArbiterState`: [ai-as-built.md](ai-as-built.md) "AI7".

### Interrupt contract (AI lane decision 6)

Highest first: rows 1-4 are drives, row 5 the stack, rows 6-7 run inside the served directive; overlays pre-empt nothing.

| # | Layer: raised by | Mechanism (shipped default) |
|---|---|---|
| 1 | **Combat, Flee**: Alerted on a hostile, damage, health, morale | drives, weight 5; the two compete on score (low health: Flee) |
| 2 | **Pursue**: guard heat, chase to LKP (AI29) | drive, weight 4 |
| 3 | **Converse**: a talk opens (pins tier 2) | drive, weight 3 |
| 4 | **Notice**: a concern, a Curious or Suspicious band | drive, weight 2; Glance/Bark as overlays, heavy rungs push a `Reaction` directive |
| 5 | **Stack**, modern rank Quest > Script > Memory ([vendettas](ai-memory.md)) > Reaction (Search, Investigate, Confront, Report, Tidy) > Schedule > Record; `Errand`, `Operate` take their source's rank (stop operator `Record`, road traveller `Schedule`) | `Directive` drive, weight 1, floor 1 |
| 6 | **Activity**: errand stage, operator post, schedule `UseSlot`, free-time pick (AI27) | HTN methods |
| 7 | **Idle**: nothing scores above 0 (`DriveReason::Idle`), or the record default | |
| - | **Overlays**: look-at, Glance, `BarkCue` (AI30; line picked and gated by [dialogue-rules.md](dialogue-rules.md) "Barks"), greeting | own channels |

- **Scores decide; ranks are defaults.** Weights are IAUS levels in the shipped `sf.ai.drive` records (score, 0-10,
  Guess after Dave Mark's 1/2/3/5): a faint Notice (product < floor / weight) never beats content. Momentum and
  `commit_ticks` stop flip-flop.
- **Pre-emption.** By a drive: `serving_for` sets `Serving::Suspended`, `Limit::For` stops counting (`Until` does not),
  the runner drops its plan (`PlanRunner::clear`). In the stack: the lower directive waits, `For` paused.
  `Reaction` (AI22) waits under quest and script orders (quest safety); rows 1-3 don't.
- **Resume = replan from the root, never restore** (Fluid HTN, Decima): the world moved, so the served directive's root
  is decomposed fresh. `resume` continues a `Pause` split only within one unbroken run.
- **Kept:** the stack (`For` left, params), progress executors write into params (errand stage, route progress, patrol
  index, report target; methods read it, so a resumed errand skips done stages), awareness, LKP, facts, a held prop
  ("Held prop under pre-emption"). **Lost:** plan, cursor, MTR, executor state, look-at, the slot claim once the exit
  ran (Converse and overlays stay seated if the slot allows; rows 1-2 take the fast exit).
- **Monitoring is not pre-emption:** `Library::holds` false on the active step, or a dirty read key, replans the same
  root. Ambient keys (`raining`, `night`) work this way: a free-time pick re-picks shelter, a work shift does not.
- **Tier 0** runs no drives, only events (push, finish, window edge, fact, AI21 concerns); a promoted actor starts on
  row 5. Morrowind's `StackPolicy` (OpenMW priorities) replaces row 5's ranks. Host only; the trace logs each pre-emption with its `DriveReason`.

## Behaviours (HTN method library)

- **One shared library**; archetypes are parameters plus method overrides, no per-archetype domains.
- Total-order forward decomposition, priority-ordered methods, preconditions on `WorldState`, effects on a copy;
  **partial plans** for long tasks (stop at a pause marker, continue later).
- **Monitoring**: the running operator checks its preconditions every step; a dirty key re-validates the plan; drives
  re-arbitrate at the tier's rate; a replan wins only with a higher-priority method traversal record (MTR).
- **Smart objects** add method fragments through their slots, so ambient life doesn't grow the library.

**As built (AI2, AI2b).** `sf_ai::{state, htn}`: records, `WorldState`, `Library`, `decompose`/`resume`/`replan`, full backtracking, MTR-prefix replans: [ai-as-built.md](ai-as-built.md) "AI2".

## Executors (operators)

Small hand-written state machines in `sf_ai`, glued in `sf_ai_bevy`; one per primitive task. They write the actor's
`LocomotionIntent` and an `NpcIntent` the shell turns into the action layer's `Command` (action.md: one per tick, "an
NPC's from AI or scripts").

`MoveTo(goal, gait)` (path request, corridor, arrival; fails on `NoPath` or 3 s without progress), `Traverse(link)`
(approach, open by activation rules, wait `passable`, cross the `Gate`; fade links only unseen), `UseSlot(slot)`
(claim, approach, enter, loop, exit), `Attack/Guard/Dodge/Reposition` (moveset, C6), `Speak`, `Face`, `LookAt`,
`Wait`, `Activate`, `Flee`, `FlyTo`, `SwimTo`; `Pick`, `Place`, `Give`, `TraverseLink` (AI0f below).

**3D movers** (lane decision 11; Horizon Zero Dawn) never use the navmesh: `FlyTo` (AI34) and `SwimTo` (AI35) follow
nav's flight and swim queries (nav.md "Volumes", NV13), as `MoveTo` does its path: 3D corridor, yaw and pitch within the class's climb and dive limits, fail on
`NoPath` or stall, hold at a `Partial` end (a fish at the band edge); `SwimTo` stays in `[bed + r, surface - r]`.
**Levitation** is a class swap to the adapter's flier class, back to ground at a landing candidate. Amphibious ground
classes swim on navmesh `water` polys. **Classic executor slot**: operators under the same task ids (an `AiWander`
port in `mw_rules`), never built early.
- **As built (AI6):** `sf_ai::exec` (`MoveTo`, `Wait`, `Face`, `PlanRunner`, `OperatorBindings`) + `sf_ai_bevy::exec` (`AiExecutor`, `AiExecPlugin`, tunables `ai.exec.*`): [ai-as-built.md](ai-as-built.md) "AI6".

## Errands, operators and links (AI0f)

**Errand (AI32)** `{ item: Prop(PlacementId) | Tagged(TagFilter), from: Anchor, to: Place { at, spot } | Give(ActorRef) }`
from schedules (porter, MA11), quests, scripts: `MoveTo(from)` → `Pick` → `MoveTo(to)` (× `CarryClass.speed_mul`) →
`Place` or `Give` (AI31, one `HeldProp`); stage `Fetch | Carry | Deliver` in params. Both tiers call a pure
`pick_spot(anchor, filter, occupancy, ActorKey)`, and tier 0 steps the same stages as events (carry: an NV12 route), so
placement and result (`Delivered | Missing | Refused`) equal tier 2's (exact test). Prop gone: `Missing`. No spot within
`ai.errand.spot_radius_m`: nearest valid floor. Recipient away past `ai.errand.give_wait_s`: `Refused`, placed at `to`.
**Held prop under pre-emption:** rows 1-2 drop a two-hand or drag prop at the feet (no spot search), a one-hand one
only if the moveset needs that hand; rows 3-4 keep it. The stage returns to `Fetch` on it (owner kept). Never stowed.

**Operate (AI45, lane decision 15)** `{ slot: OperatorSlotRef, mode: Post | Crew | Road(RouteRef) }` on an ordinary
actor (travel.md "Operators and replacement"). `Post`: `UseSlot(post)` at the board point; `Crew` also rides each trip
(TV4 deck slot), then alights. Ambush (TV9): row 1 defends or flees by archetype. AI emits `OperatorPresence { slot,
present, reason }` on change (present = serving within `ai.operate.post_radius_m` of the post, or aboard); absent →
`Absent`, departures refused; dead → `Vacant`, TV26 replaces. **Road travellers, caravans** (TV21): `Road` on NV12 at
tier 0; promotion places them on the road only out of every player's view (`ViewSample`) or beyond
`ai.operate.road_pop_m`, else they stay tier 0, moving. Source: stop operator `Record` (bound at load, TV5), road
traveller `Schedule` (timetable windows).

**`TraverseLink` (AI33, lane decision 10)**, the NPC side of nav.md "Links": `LinkReached` suspends `MoveTo` (no
replan); face `to` within `ai.link.align_deg`; send the player's `Command` for the kind (`Vault`, `Mantle`, `Climb`:
AL-10; `Jump`: AL-12; `Drop`: step off; `Ladder`: `UseSlot`); `sf_action` runs the player's probes and states, no
NPC-only climb code. Grounded at `to`: `LinkDone`; else `LinkFailed` (refused `Envelope`, no progress `Blocked`, nav's
timeout `Timeout`, pre-empted `Interrupted`, `LinkDisabled` before the move `Disabled`); nav raises the cost (first
three) and repaths. Tier 1 lacks foot probes, so a link pins tier 2 until the answer; tier 0 meets none (NV12 edges).

Tunables (`Gameplay`, `Dev`, Guess): `ai.errand.spot_radius_m` m 1-20 (5), `ai.errand.give_wait_s` game s 5-300 (60),
`ai.operate.post_radius_m` m 1-15 (4), `ai.operate.road_pop_m` m 150-2000 (500), `ai.link.align_deg` deg 5-45 (15).
All host only, at tick boundaries.

## Perception and stimuli

As designed (AI0b, roadmap decisions 2, 3, 14); built by AI8a-AI8f. Host-only, fixed tick; pure maths in `sf_ai`,
queries (grid, rays, path distance) in `sf_ai_bevy`.

### Stimuli

- **Targets**: `PerceptTarget = Player(PlayerKey) | Actor(ActorRef)`; every player, local or remote, is one target
  with the same emitters; actors only when flagged (hostiles; companions, who obey stealth). Sight is polled (below).
- **Noise** (input, UE `ReportNoiseEvent` shape): `NoiseEvent { tick, at, space, kind: NoiseKindId, loudness_m,
  source: Option<PerceptTarget>, reveals_source }`; `loudness_m` = open-air reach from `NoiseKind` records × surface and
  gear. Emitters: action states (`noise: Loud`, action.md), gait, landing, combat, and interactions (IN18: thrown,
  dropped, smashed objects, picks, bashed locks, doors). Footsteps reveal their source; a thrown cup does not (`at` =
  where it landed).
- **Darkness**: IN19 `LightChanged { light, on, at, culprit }` (a `Lit` snuffed or relit) reaches actors in the room or
  with LOS to it: `Darkened` concern (AI28), relit by the dusk-lamp method (AI27).
- `Touch`, `Damage` (always perceived, always wake) and `Notify`. All are events (net rule 8).

### Awareness

`Awareness { target, level: f32 /* 0..1 */, band, last_seen: LastSeen { pos, tick, velocity, confidence }, last_gain }`
per actor, bounded (`max_entries`, lowest evicted), plus one `Distraction { at, tick, loudness_m }` slot (loudest
recent wins). Bands: **Unaware**, **Curious** (look toward), **Suspicious** (turn, step toward LKP), **Searching** (HTN
`search` at LKP: look, widen, give up on ttl; AI8f; a running search holds the band, so the search, not the decay,
ends Searching: AI8f3), **Alerted** (level 1, latched: drives pick combat, report or flee).
A band is entered at its threshold and left below it minus `band_hysteresis`; Alerted holds while the target is seen,
then drops to the Searching floor after `alert_hold_s`. Decay starts `decay_delay_s` after the last gain, per band
rate. Bands and `LastSeen` feed world-state keys and drives.

- **Sight gain** per check: `rate × dt × f_dist × f_cone × visibility × alertness`; `f_dist` linear from 1 at `near_m`
  to 0 at range (`lose_sight_range_m` once seen, UE-style); `f_cone` 1 central, `peripheral_mul` peripheral, 0 behind;
  `visibility = clamp(1 - concealment / 100, 0, 1)` from `DetectionRules::concealment` (crime.md), fed `VisibilityFactors
  { light, stance, speed_mps, size, noise_mul, disguise, modifiers: [(StatId, f32)] }`; `alertness` higher from
  Suspicious up, lower while groggy.
- **Noise gain**: heard iff `path_distance(at → listener)` plus `closed_door_m` per closed door ≤ `loudness_m ×
  hearing_mul × weather_mask`. A revealing noise raises the source's entry by `noise_gain × (1 - d / reach)`, capped at
  `noise_cap` (hearing alone never alerts), LKP = `at` with `noise_confidence`; a non-revealing one fills `Distraction`
  (investigate the spot; owner: thrown or smashed objects draw attention).
- **Notify** (shared knowledge, no telepathy): a shout or alarm is a `NoiseEvent` carrying `Notify { about, lkp,
  confidence, hops }`; listeners who hear it get it at `tick + (path_distance / notify_speed_mps + notify_react_s)`,
  confidence × `notify_confidence_mul`, `hops ≤ notify_max_hops`; it raises at most to Searching.
- **As built (AI8c3, 2026-11-26):** [ai-as-built.md](ai-as-built.md#ai8c3-noise-over-portal-paths-2026-11-26)
- **Asleep** (bed activity; night workers keep their schedule, owner 2026-11-25): no sight; only noise received at ≥
  `asleep_wake_m` wakes, as do Touch and Damage, then `groggy_s` of low alertness. Rain and storms scale sight range
  (`weather_sight_mul`) and mask noise (`weather_mask`); storms also send people to cover (AI27).
- **Models** (`DetectionModel`, crime.md): **Meter** (above; framework default) or **Classic**: per sight check one
  seeded roll `hash(actor, target, tick)` against concealment sets level 1 on success, else nothing.

### Light exposure (from game state, never the GPU)

`LightExposure` (0..1), pure: a sky term from clock, weather and interior flag, plus `Lit` sources (IN19: radius,
intensity, on) within `light_search_m`, the strongest `light_rays` each checked by one LOS ray (occluded adds
nothing); outdoors one upward ray for roof cover. One sample per player per tick (other targets on demand, cached);
it feeds `DetectIn.light`, the HUD and the `dark` world-state key (< `dark_threshold` at an actor: NPCs react to
darkness). Rendered light never enters (net rule 3); equal at 30 and 144 fps (AI8d).

### Perception profile (per individual)

Owner 2026-11-25 (DECISIONS): every actor perceives with its own multipliers on the global tunables; the game fills
them (AI0g). The framework knows only fields and record ids, never a stat or a race.

- **`PerceptionProfile`** (`sf_ai`, serde): a record type in `sf_records` (presets mods author: a race, creature
  type, role) and the actor's component (the adapter's product of presets and its own stat curves). Fields are ratios,
  default **1.0** each, so an actor without one behaves exactly as today; range checked at load:
  `sight_range` (× `sight_range_m`, `lose_sight_range_m`; `near_m` unscaled) 0-4; `sight_cone` (× both half-angles,
  ≤ 180°) 0-2; `sight_gain` (× `rate`) 0-4; `darkness` (how much darkness lowers sight: AI8b feeds `light' = 1 - (1 -
  light) × darkness` to `VisibilityFactors.light`; 0 sees in the dark, 1 normal, 2 night-blind) 0-2; `hearing` (it *is*
  the per-actor `hearing_mul`: × noise reach) 0-4; `alertness` (× sight gain and `noise_gain`) 0-4; `decay` (× the three
  band decays; < 1 holds suspicion longer) 0-4; `search_ttl` (× `search_ttl_s`: persistence) 0-4; `search_error`
  (× `search_error_m` and `search_widen_mps`: < 1 guesses closer to the true position and widens slower) 0-4.
  Not scaled: band thresholds, `band_hysteresis`, `alert_hold_s`, `noise_cap`, `max_entries` (a band means the same for
  everyone, so the HUD eye reads one scale; hearing alone still never alerts).
- **Temporary states** (`PerceptionStates`, inline array of 4, a capacity const, no heap): `PerceptionState` records
  (game-named: drunk, tired, injured, fleeing, on duty) carry the same fields (missing = 1). An entry is `{ state:
  RecordId, until: Option<u64 /* tick */>, source: Option<RecordId> }`: `Some` is timed (`duration_s` from the record
  or the pusher, expires on the host tick), `None` a condition the adapter holds and removes. The same state again
  refreshes `until` (later wins), never stacks; different states multiply; a full array replaces the entry that expires
  first. Effective field = profile × Π states, clamped to the field's range.
- **Groggy option and the cached `EffectivePerception` hook (AI8b, AI8g):** [ai-as-built.md](ai-as-built.md#perception-profile-groggy-option-and-effectiveperception-hook-ai8b-ai8g)
- **Ownership:** the adapter writes `PerceptionProfile` (on spawn, load, and when its inputs change past its own
  steps) and pushes or clears states; `sf_ai_bevy` resolves. Floor share unchanged (rebuilds are events, not ticks).

### Cost and budget

Tier 2 sight, staggered by `hash(ActorKey)` over `sight_period_s` (damage, noise or notify force a check next tick):
**grid prefilter** (same space or an open portal) → **cone and range** → skip below `skip_gain` → **rays**: chest,
then head only if blocked (≤ 2). Sight, light and concern-discovery rays (world-reactions step 4) share one queue
capped at `max_rays_per_tick` (UE `MaxTracesPerTick`), oldest first, overflow next tick; batched on workers, applied
in `ActorKey` order; cached `sight_cache_s` unless a side moves > `cache_move_m`. Hearing: grid by straight-line
reach, then `path_distance`.
**Tier 1**: sight every `tier1_period_s` and hearing by distance + same space, no rays or paths. **Tier 0**: none.
**Event-time queries** (witnesses, alarms, greetings) stay tier-independent: a full check over every simulated actor
within `witness_radius_m`, outside the cap, nearest first up to `witness_max`. Floor share 0.15 ms per tick (sight,
hearing, light); `ai.stats` reports perception ms per tier and rays issued/deferred. 48 tier-2 actors at 10 Hz ≈ 5
checks per tick (≤ 10 rays) plus ≤ 3 light rays per player: fits Floor's cap.

### Tunables (`ai.perception.*`, `ai.awareness.*`; net `Gameplay`, scope `Dev`)

Unit = name suffix (`_m`, `_s`, `_deg`, `_mps`; `rate` and decays in 1/s; others ratio or count). Source `Guess`
unless noted; `hearing_mul` and the other `PerceptionProfile` / `PerceptionState` fields are per-actor data
(records), not tunables.

| Names | Defaults | Ranges |
|---|---|---|
| `sight_range_m` / `lose_sight_range_m` / `near_m` (Industry: UE SightRadius, LoseSightRadius) | 30 / 36 / 3 | 1-200 |
| `central_half_deg` / `peripheral_half_deg` / `peripheral_mul` | 30 / 80 / 0.35 | 0-180, 0-1 |
| `rate` (full visibility at `near_m`) / `skip_gain` | 2.0 / 0.01 | 0.05-20, 0-1 |
| band thresholds Curious / Suspicious / Searching; `band_hysteresis` | 0.1 / 0.35 / 0.65; 0.05 | 0-1, 0-0.3 |
| `alert_hold_s` / `decay_delay_s`; decay Curious / Suspicious / Searching | 6 / 3; 0.10 / 0.05 / 0.02 | 0-60, 0-1 |
| `noise_gain` / `noise_cap` / `noise_confidence` / `closed_door_m` | 0.5 / 0.65 / 0.5 / 8 | 0-1, 0-50 |
| storm `weather_mask` / `weather_sight_mul` | 0.6 / 0.6 | 0-1 |
| `notify_speed_mps` / `notify_react_s` / `notify_confidence_mul` / `notify_max_hops` (hop rule reused by gossip, [ai-memory.md](ai-memory.md)) | 6 / 0.8 / 0.7 / 3 | 0.5-50, 0-5, 0-1, 0-8 |
| `asleep_wake_m` / `groggy_s` | 12 / 4 | 0-100, 0-30 |
| `distraction_ttl_s` / `alert_alertness` (gain mul from Suspicious up) / `groggy_alertness` (AI8a) | 10 / 1.5 / 0.5 | 0-60, 0-4, 0-1 |
| `light_search_m` / `light_rays` / `dark_threshold` | 15 / 2 / 0.15 | 1-60, 0-4, 0-1 |
| `react_step_m` / `react_retarget_m` / `react_hold_s` / `react_face_delay_s` (AI8h1, Guess: Curious turn, Suspicious step toward LKP) | 3 / 1.5 / 2 / 0.3 | 0-10, 0-10, 0-10, 0-2 |
| `roof_sky_mul` (sky under a roof) / `light_falloff_exp` (k in `saturate(1 - (d/r)^k)²`; UE4/Frostbite window use 4) (AI8d1, Guess) | 0.25 / 2 | 0-1, 1-8 |
| `sight_period_s` / `tier1_period_s` / `sight_cache_s` / `cache_move_m` | 0.1 / 1.0 / 0.25 / 0.5 | 0.02-10, 0-5 |
| `max_rays_per_tick` Floor / Medium / High / Ultra / Dedicated (group `sim.cpu_tier`) | 24 / 32 / 48 / 64 / 96 | 4-512 |
| `witness_radius_m` / `witness_max` / `max_entries` (8: earlier spec) | 25 / 32 / 8 | 1-100, 1-256, 1-64 |
| `search_ttl_s` / `search_error_m` / `search_widen_mps` (AI8f globals the profile scales) | 30 / 3 / 0.5 | 0-300, 0-30, 0-5 |
| `search_ring_points` / `search_look_half_deg` / `search_uncertain_mul` / `search_inner_frac` (AI8f1, Guess) | 6 / 60 / 2 / 0.5 | 1-16, 0-180, 1-4, 0-1 |

### Net, saves, HUD

- **Net**: all of it is host-only (rule 5); a remote player is another target with its own `LightExposure` and
  emitters. Clients get only "Co-op"'s replicated set and their own `PlayerDetection`.
- **Saves**: awareness, LKP, distraction, caches and light are not saved ("Saves"); a running `Search`, `Report` or
  combat directive is (LKP in its params), and on load its actor starts at that directive's band floor. The base
  `PerceptionProfile` and condition states are derived (the adapter refills them on load); only timed states are saved
  (`{ state, remaining_s, source }`); `EffectivePerception` is rebuilt. Profile and states are host-only gameplay state.
- **HUD** (owner 2026-11-25: Skyrim-style sneak eye, no per-NPC markers): `PlayerDetection { level, band }` per player
  = max level toward that player over tier 2 and 1 actors that count (not allies or followers); crime.md's
  `StealthView.exposure` derives from its band. Presentation reads it (smooths client-side), never feeds back.

### Crime, reactions, adapter

- Crime hook: `OffenceObserved { offender, kind: RecordId, witnesses, at, tick }`, each witness with its band and
  visibility (crime's recognition input). Reporting is a walking, interruptible `Report` directive in both presets
  (owner 2026-11-25: never instant); bounty, guards and arrest are adapter rules.
- **World reactions** (concerns, suspicion from `Awareness` entries): [world-reactions.md](world-reactions.md),
  AI19-AI25, AIM2.
- **Adapter seam**: the game implements `DetectionRules` and picks `DetectionModel`; the framework knows no sneak skill
  or magic effect (`VisibilityFactors.modifiers` carries stat ids the adapter reads). Bevyrrowind's is the OpenMW
  `awarenessCheck` port in `mw_ai` (MA5, `mechanicsmanagerimp.cpp` 1603 at `b8285cc9`, ai-adapter.md: sneak, attributes,
  fatigue, chameleon, invisibility, facing), light term optional.

### Industry comparison

| Reference | Technique | Ours |
|---|---|---|
| TLOU2 | graded awareness, searching state; companions obey stealth | **match** |
| Hitman | suspicion meter, shared knowledge, distraction throws, disguise | **match** (Notify, `Distraction`, `disguise`) |
| AC Shadows | stance, distance, light; snuffable lights; weather; per-enemy meter | **match**; per-enemy meter **differs** (owner: one eye) |
| UE AIPerception | sight/lose-sight radius, peripheral angle, `ReportNoiseEvent`, max traces per tick, max age | **match** (fixed caps) |
| KCD2 | gear-driven visibility and noise, dark nights, sleepers wake to noise | **match** |
| Skyrim | one sneak eye, light level, armour noise, sleepers less perceptive | **match** (`PlayerDetection`) |
| Thief (Dark Engine) / The Dark Mod | per-AI vision and hearing abilities and search diligence; TDM per-AI `acuity_vis` / `acuity_aud` | **match** (`PerceptionProfile`, `search_ttl`) |
| Cyberpunk 2077 | per-NPC sense preset record (`sensePreset`, e.g. `Senses.Relaxed`) | **match** (profile presets are records) |
| MGSV | per-soldier gear and time of day change perception (night-vision goggles, helmets) | **match** (`darkness` from a state or preset) |
## Memory, gossip and reputation (owner decision 7, lane decision 8)

Fact store, Nemesis-style relations and vendettas, bounded gossip, reputation per settlement and faction, and
`ConsequenceRules`: [ai-memory.md](ai-memory.md) (AI13, AI43, AI44, AI25). Saved in `sf/aimemory` (see "Saves").

## Smart objects (`sf_interaction`)

`find_slots`, `claim`/`release` (host) and per-activity method fragments: [interaction.md](interaction.md); carry and
`Relocate`: [interaction-carry.md](interaction-carry.md). Until IN9 the adapter's pathgrid wander stands in.

## Tiers and budget (owner decision 5)

| Tier | Who | Decides | Moves | Perceives |
|---|---|---|---|---|
| 2 full | allocator winners in the simulated set | 5-10 Hz staggered + events | nav corridor + ORCA + mover + probes | full, per player |
| 1 reduced | next band in the simulated set | ~1 Hz staggered | corridor, mover without foot probes | distance + same space; event-time checks full |
| 0 logical | everyone else, every worldspace | events (window edge, directive done, fact); no planner | `(route, progress, activity, until)` on `WorldGraph` + NV12, travel time | none |

- **Allocator** (the `AnimBudget` pattern): significance from distance to the **nearest player of all**, visibility to
  any player (open doorways too), combat, dialogue and quest pins; hysteresis on both bands; one **global** cap, so
  four players in four towns share it. Caps are **fixed counts** per CPU tier (UE MassLOD), never adapted to measured
  time, so traces reproduce. `ai.stats` warns over the tick budget and prints ms per tier (2, 1, 0) and the resolve
  queue's length and µs, for AIM.
- **Behind an open door** (decisions (k)): the far R0 around the mapped anchor joins the simulated set; its actors
  take tiers from the same allocator, physics steps, scripts run; no stand-in mode.
- **Pins outrank caps**: combat, dialogue and scripted scenes stay tier 2; over the cap is logged, never demoted.
- **Tier invariance** (the multiplayer rule): a tier changes how often and how precisely an actor is stepped, never
  which rules apply: completions, window edges, arrivals, crimes and facts follow one logical clock.
- **Tier 0 is a state machine, no planner** (research §4; KCD2's LOD-unaware scripts): `LogicalState { route:
  Option<RouteRef>, progress_m: f32, activity: Option<ActivityRef>, until: GameTime }`, stepped at events and at
  `until`: the stack top's kind gives the next state (a route on `WorldGraph` + NV12, an activity slot, a wait). No
  drives, HTN or perception run. Every `DirectiveKind` registers its tier-0 transition; one without it fails at load.
- **Staggered first update** (UE Mass): an actor joining the simulated set (cell load, open door, promotion) gets
  `first_decide_tick = tick + hash(ActorKey) % period` of its tier, for decisions and sight (AI9e).
- **Periodic vs event planning** (AI9g, as built): on decide ticks only: a fresh decomposition, the resume past a
  `Pause`, an explicit `request_replan` (callers hand over a chosen step); on any tick: a dirty read key, a failed
  precondition, the replan after an operator failure. Between decide ticks an actor whose plan ran out stands.
- **Promotion resolve**, queued nearest-first (distance to the nearest player), at most `ai.tier.promote_per_tick` per
  tick (count, 1-64, Guess 4; a count, not a time budget, so traces reproduce); pins jump the queue; a queued actor
  stays tier 0 and keeps advancing. Resolve: place the actor where its state puts it at tick `t`, snapped to the
  navmesh on its route; claim the slot or an alternative; a spot a player could see waits or is entered from an
  occluded approach (door, corner); never combat in view. **Demotion** only out of view, past the outer band, never in
  combat or dialogue. Scripts see one actor in every tier (KCD).
- **Fast-forward** (rest, wait, long travel): `fast_forward(actor, hours)` advances tier-0 state; the adapter keeps
  OpenMW's `fastForward` rules.
- **As built (AI10a, 2026-11-26):** `sf_ai::logical`. Differs from the shape above: `until` is an
  `Option<GameInstant>` (game ms: arrivals fall between minutes; ends still read the minute rounded down, like every
  tier), and the state also holds `pos`, `at`, `follow`, the arrival action and instant, and the served directive id.
  Routes come through the `LogicalEnv` seam (`RouteRef { from, to, length_m, speed_mps }`, `point_on`), not
  `sf_nav` types: the shell composes `WorldGraph` + NV12 (AI10c). Speed is per simulated second (game ms /
  `time_scale`), so tier 0 matches a tier-2 walker. Fight holds (pinned). `fast_forward(state, stack, hours, ..)`.
- **As built (AI10b, 2026-11-26):** `sf_ai::promote`. A try is one resolve (snap + view tests); a waiting entry is not
  tried again for `ai.tier.promote_retry_s` (0.1-5 s, Guess 0.5; skips are free), so seen waiters near the player
  never starve unseen ones. Candidates in order: the state's spot at `t`, then points back along its own route every
  `ai.tier.promote_backtrack_step_m` (1-20 m, Guess 4) up to `ai.tier.promote_backtrack_m` (0-200 m, Guess 40) or the
  route start (it walks into view on its path, GTA/RDR style), then the shell's occluded approaches; a refused `Use`
  claim places it standing. Seams: `PlayerViews::seen_by_any(at, radius)` (gameplay viewers, also for doors and
  follower catch-up) and `PromoteEnv` (snap, claim, approaches, body radius), both the shell's (AI10c).
  `demote(body, now)` is a fresh `LogicalState`; the allocator alone grants it.
- **As built (AI10c1, 2026-11-26):** `sf_ai_bevy::logical`. An actor is tier 0 in fact iff its key is in the
  `AiLogical` store (with or without a body); the allocator's `AiTier` is overridden to `Logical` on a stored body until
  it is placed, so a queued actor perceives and plans as tier 0. A stored body already standing on its state's spot
  (same worldspace, within its body radius) is placed there with no view test: nothing moves, so nothing pops in.
  Followers read their leader's state through the store, stepped after the actors they follow.
- **As built (AI10c3c2a, 2026-11-27):** `mw_walkers_bevy::bodiless`. At an actor cell's deactivation `ActorPoseHook`
  keeps its placement as the overlay holds it (`ResidentActors` away map; activation drops it, a discard keeps nothing).
  Last in the Step chain, each kept placement of a bodiless stored actor on a route moves (`PlacementOverlay::move_to`,
  kept rotation) when its logical position's `(worldspace, partition cell)` changes: one write per crossing; entries
  with neither body nor state are pruned. Gaps: actors never loaded since start, followers; loaded cells wait for
  `AI10c3c2b`.
- **As built (AI10c3c2b1, 2026-11-27):** same module. Right after the follow, a kept bodiless stored actor (not a
  follower) whose cell is Active is resolved by `sf_ai::resolve_promotion` (the promotion's `ViewsEnv`: spot, then back
  along its route, navmesh snap, unseen), at most `ai.tier.promote_per_tick` tries a tick in id order; a wait retries
  after `ai.tier.promote_retry_s` (game ms over the clock's scale). The pick goes to `Streaming::spawn_into` at that spot
  (exclusive system, last of the Step chain), the overlay takes it there, activation drops the kept entry, and
  `enter_tier0` re-marks the body: the store keeps its state until it is placed.
- **As built (AI10c3c2b2, 2026-11-27):** the follow's writes and the spawns count into `sf_ai_bevy::AiBodilessStats`
  (since start; the game inserts it, `ai.stats` prints `bodiless: N bodies spawned into loaded cells, M kept placements
  moved` only when it exists); `report_bodiless`, last of the Step chain, logs `ai.tier0: N bodies spawned into loaded
  cells, M kept placements moved` when they changed, at most once a second. Owner scene AI-17.
- **CPU tier** = host setting `sim.cpu_tier` (auto from cores, AI9d), apart from the graphics preset. Clients vary
  only presentation.

| CPU tier | Reference | Tier 2 / tier 1 caps | AI main-thread budget per tick |
|---|---|---|---|
| Floor | Steam Deck (4c Zen 2) | **48 / 160** (research §8) | ≤ 0.55 ms |
| Medium | 6-core desktop, Steam Machine class | 80 / 260 (provisional) | ≤ 0.75 ms (provisional) |
| High | Ryzen 7 7435HS laptop (reference) | 128 / 400 (provisional) | ≤ 1.0 ms (provisional) |
| Ultra | 12+ cores | 192 / 600 (provisional) | ≤ 1.3 ms (provisional) |
| Dedicated | headless server, no render thread | 256 / 1000 at 8 cores, then measured per core (provisional) | ≤ 2.0 ms (provisional) |

Floor split per tick (research §8): steering 0.20, perception 0.15, decisions 0.10, tier 0 0.05, facts 0.05 ms; paths
0.30 ms on a worker; animation has its own `AnimBudget`. Lane decision 13 keeps these numbers. **AIM** sets the
provisional rows: tier-2 counts 32-256 in Balmora and a Vivec canton, 1 and 4 players, AI + mover/probes + animation
ms together (128 × A0 ≈ 3.2 ms); the cap is the largest count that fits the tier's character budget.

## Scheduling (time slicing)

Budget per tick, not per frame. Decisions are staggered by `hash(ActorKey) % period`; LOS and path requests are
queued and sliced; `decide` is pure, so batches can run on workers and apply at the next tick boundary in `ActorKey`
order. RNG per actor = hash(stable id, tick, salt). Events (damage, stimulus, directive change) jump the queue.

## Combat AI (modern first; owner decision 8)

Moveset-driven (combat.md C6): keep a spacing band from baked reach, pick attacks by reach, recovery and the target's
state, react to telegraphed attacks with guard/dodge by skill, **attack tokens** (a few attackers per target at once,
others circle), flee through the *Flee* drive. Seam: `CombatRules` (modern operators; classic = the adapter's later
port of OpenMW `aicombataction.cpp` action ratings in `mw_rules`).

## Co-op and multiplayer (net-ready rules)

1: `FixedUpdate`, async results at tick boundaries. 2: AI writes the same `Command` players do. 3: serde state,
presentation never feeds AI. 4: pure `decide`, hashed RNG. 5: **host-only**: directives, drives, plans, perception,
facts, tiers, paths, claims. 6: per-player awareness and memory. 7: stable ids. 8: stimuli and offences are events.
- **Replicated** (interest-managed): `WorldPos`, facing, move id + start tick, locomotion values, health, equipment,
  current activity (slot + phase), look-at target, barks and dialogue events, a hostility flag for UI. Never plans or
  memory; a host-admin debug channel may stream traces.
- **Interest ≠ tier**: a client receives actors in its interest set whatever their tier, at a lower rate for tier 1.
- Headless/dedicated servers run the same `sf_ai` with no renderer; caps from `sim.cpu_tier = Dedicated`.

## Saves

Sections (owner-keyed, versioned, save.md): `sf/ai` per actor (directive stack, schedule cursor, tier-0
`LogicalState`, current activity and claim, drive and running directive id) and `sf/aimemory` (facts, relations: [ai-memory.md](ai-memory.md)). Not
saved: plans, traces, awareness, path requests, `DirectiveDone` events (rebuilt on load; promotion resolve places
actors). Belongs to the one state registry when it exists (vision §4.2.3).
- **Section names are single-segment** (a family is the name before the first `/`, save.md "Registry"): hence
  `sf/aimemory`, not `sf/ai/memory` (audit 2026-11-09).
- **As built (AI5):** one world section keyed by `ActorKey` (not overlay blobs), v1 = each actor's `SavedStack`; clock and key caveats: [ai-as-built.md](ai-as-built.md) "AI5".

## Scripting (Luau `@sf/ai`)

API per system (vision §7): push/pop/query directives, `done(id)`, read awareness and facts, append facts, events
(`directive_done`, `stimulus`, `offence`, `tier_changed` for debugging). No per-tick decision callbacks (scripting.md).
Built when Lua is unparked; MWScript AI functions go through the adapter.

## LLM advisor seam (owner decision 6; design only)

- `Advisor` trait: the host offers a **menu of options it already validated** (dialogue responses, or candidate
  directives/drives), a context (persona record, salient facts, awareness) and a deadline in ticks; the answer is an
  index plus optional display-only text. No answer in time: the built-in choice stands. Picking only among valid
  options means no rewards the rules didn't allow (Where Winds Meet's failure).
- **Out of process**: versioned JSON protocol over a local socket or stdio; plugin shipped separately
  (`superfargoth/plugins/`), off by default, disclosed in the UI, host-run in MP. Choices and shown text are saved.

## Debugging (slice AI1; industry: build the trace first)

Per-actor **trace ring buffer** (64 decisions: tick, tier, drive and consideration scores, chosen methods, each
**rejected method with its failing precondition**, plan, operator and path state, budget), serde. Console `ai.trace`,
`ai.select` (reference picking, ROADMAP 1.13), `ai.tiers`, `ai.stats`, `ai.facts`; JSON/BRP for agents; gizmos
(paths, cones, tier colours, claims, plan text), feature `debug`.

**As built (AI1-AI5):** [ai-as-built.md](ai-as-built.md).

## Industry comparison

| Technique | Industry | Ours | Verdict |
|---|---|---|---|
| Top layer | KCD II routines, RDR2 and Cyberpunk schedules (press only), Skyrim procedure trees, OpenMW `AiSequence` | directive stack + `StackPolicy` | **match** |
| Reactions | UE AI Perception sources; Hitman, KCD II, RDR2 witnesses and suspicion | world-reactions.md concerns | **match** (planned) |
| Selection | utility: Sims, GW2 IAUS, OpenMW combat ratings | drives + momentum | **match** |
| Behaviours | HTN: Horizon, Killzone 2, Transformers: Fall of Cybertron; GOAP: F.E.A.R., Transformers: War for Cybertron; BT: KCD 1 and 2 (CryEngine Modular Behavior Tree); StateTree: UE5 | shared HTN library, MTR | **match** (HTN: designer control, cost) |
| Interrupts | Bethesda package priority, OpenMW combat on top of `AiSequence`, IAUS weight levels, HTN replan on resume (Fluid HTN) | drive bands + stack ranks, replan from root | **match** |
| Schedules | Ultima VII, Gothic, Bethesda packages, KCD routines | records → directives | **match** |
| Smart objects | Sims, UE Smart Objects, Skyrim furniture, A-Life smart terrains | `sf_interaction` | **match** (planned) |
| Perception | UE AI Perception; OpenMW `awarenessCheck` | per player, through doors | **match** |
| Errands, operators | Skyrim packages (Acquire, UseItemAt, Travel), RDR2 ambient chores (press only), Mount & Blade caravans on the map | `Errand`, `Operate`, tier 0 on NV12 | **match** |
| NPC traversal | UE Smart Links (handoff event), Unity `NavMeshLink` manual traversal | `TraverseLink` drives the player's moves | **match** |
| Off-screen LOD | UE MassLOD caps, Bethesda process levels, A-Life, KCD | 3 tiers, global cap, resolve | **match** |
| Memory | Nemesis, KCD2 town reputation, RDR2 witnesses: [ai-memory.md](ai-memory.md) | fact store, gossip, reputation | **match** |
| LLM NPCs | inZOI/ACE on-device, Mantella out of process, Where Winds Meet chatbots | advisor over validated options | **differ** (choices only) |
| Debugging | UE Gameplay Debugger + Visual Logger, Guerrilla in-game HTN view | trace from slice 1 | **match**; recording later (P2) |

Sources: research "Sources" (UESP, Warhorse GDC 2025/2026, Guerrilla, Humphreys, Orkin, IAUS, UE docs, OpenMW
`b8285cc9`); Humphreys, Game AI Pro ch. 12 (Fall of Cybertron's HTN replaced War for Cybertron's GOAP); CryEngine
"Modular Behavior Tree" docs; `../research/gameplay-2026-11-25/ai.md` §1.

## Tests

Logic exact; key test **tier invariance**: one scripted day at cap 48 and at cap 400 gives the same directive and
activity log at window edges. Feel: sanity only; budgets as `#[ignore = "measurement: ..."]`.

## Build plan

The build order is [docs/roadmaps/ai.md](../../../docs/roadmaps/ai.md) §4 (stages, slices, dependencies, status); this
spec holds no order of its own. One deliverable plus its test per slice; visual checks use prefix `AI-`. The original
AI0-AI18 table is history: [ai-as-built.md](ai-as-built.md#original-build-plan-ai0-ai18-2026-11-08), as are the
[shared-doc changes](ai-as-built.md#doc-changes-for-shared-files-2026-11-08).

## Open questions

None open. Decided 2026-11-08 (`../research/ai-nav-decisions-2026-11-08.md`): (d) High 128 / 400 provisional fixed
counts, set by AIM; (e) schedules after AI10-AI12 (patterns in the adapter); (g) vendettas cross worldspaces, bounded.
