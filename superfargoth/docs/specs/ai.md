# NPC AI: directives, drives, HTN behaviours, tiers (`sf_ai`)

Status: **AI1-AI5 built (2026-11-08/09)**: `sf_ai` ids, trace, HTN core, directives, `sf/ai` save; `sf_ai_bevy` overlay; AI6 on not built. M7.
Former open questions decided 2026-11-08 (owner-delegated): `../research/ai-nav-decisions-2026-11-08.md` (d, g, k, l).
Crates (planned): `sf_ai` (Bevy-free: directives, drives, HTN, perception model, facts, tiers, trace), `sf_ai_bevy`
(thin: fixed-tick systems, executors' glue to `sf_action`/`sf_controller`, debugger, console). Uses [nav.md](nav.md)
(`sf_nav`) and the planned `sf_interaction`. Game side: [ai-adapter](../../../bevyrrowind/docs/specs/ai-adapter.md).
Sources: `../research/npc-ai-2026-11-08.md` (approved), seamless research §4.2/§4.8, vision §4/§5/§7, ARCHITECTURE §1b,
[action.md](action.md), [net.md](net.md), [combat.md](combat.md) C6.

<!-- agent-summary -->
> **Agent summary (2026-11-08).** AI1-AI5 (trace, HTN, overlay, directives, saves) built; the rest is design. Layers: directives (packages, schedules, quest orders) →
> utility drives → one shared data-driven HTN method library → executors writing `Command` + `LocomotionIntent`.
> Host-only, fixed tick. Tiers 2/1/0 under a budget that is a host CPU setting (never the graphics preset); tier 0
> simulates logically so outcomes are tier-invariant. Perception through doors, a persistent memory/relationship fact
> store (Nemesis-style, owner yes), an out-of-process LLM advisor seam (later). Debugger in slice AI1.
> Read next: "Data model", "Tiers and budget", "Build plan"; shared-doc edits in the decisions note.
<!-- /agent-summary -->

## Goal

NPCs and creatures that follow content directives predictably, react, fight, travel across worldspaces (seamless or
fade doors), keep routines off-screen and remember players. Morrowind's packages and settings are adapter data.
Non-goals: an LLM in the core, GOAP, flow fields, classic executors and combat (later seams), economy/rumour kits.

## Owner decisions this spec implements (2026-11-08)

1. Layering and the `sf_nav` + `sf_ai` (+ `sf_interaction`) split ("Architecture"). 2-3. Nav patches and the
`navmesh-rs` spike (nav.md). 4. Vanilla NPCs get schedules, behind a seam allowing "no schedules" ("Directives").
5. Caps are a **host CPU-tier setting**, never graphics; tier 0 simulates logically; clients vary only presentation
("Tiers and budget"). 6. LLM later, out of process, seam now ("LLM advisor seam"). 7. Nemesis-style memory: yes, the
patent is not a concern (owner decision) ("Memory"). 8. Combat modern first ("Combat AI"). 9. One label, M7 (M10 is
combat; "Doc changes"). Also: GPL-3.0; no MP ceiling (server authority, headless builds, interest management, tier by the
nearest player under a global budget); Steam Deck floor; seamless doors optional. Delegated (2026-11-08): fixed,
provisional caps ("Tiers"); bounded cross-worldspace vendettas ("Memory"); far sides of open doors fully simulated
("Tiers"); schedules after tier 0 ("Directives").

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
   → drives (Directive, Combat, Flee, Pursue, React, Converse) scored by considerations, with hysteresis
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
    Escort { who: ActorRef, to: Location }, Use(PlacementId), Fight(ActorRef), Custom(RecordId) }
pub struct Directive { pub id: DirectiveId, pub kind: DirectiveKind, pub priority: u8, pub repeat: bool,
    pub until: Option<GameTime>, pub source: DirectiveSource /* Record | Schedule | Script | Quest | Memory */,
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

## Directives (the §1b seam)

- Sources, lowest to highest by default: record defaults (an NPC's packages), **schedules**, memory-driven goals
  (vendetta), scripts, quests. The adapter's `StackPolicy` decides insertion, cancelling and repeat (Morrowind: the
  OpenMW `aisequence.cpp` rules). Completion (`done`) is the signal scripts poll (`GetAIPackageDone`).
- **Schedules** (`sf.ai.schedule`): `{ days, from, to, directive template, place }`, `place` a `Location` or an
  adapter-resolved anchor ("home", "work", a slot tag); the host clock pushes and retires `Schedule` directives at
  window edges. Seam `ScheduleRules` (modern = records; classic = none). Quest safety (Radiant's lesson): scripts and
  quests outrank schedules; a validator lists actors that quests address by position, so packs can exclude them.
- **Timing**: schedules move actors between worldspaces, so they need tier 0 and door traversal: `ScheduleRules::None`
  is the default until AI10, AI11 and AI12 exist (decisions (e)); an anchor that doesn't resolve keeps the actor on
  its record directives.
- Directives are fully saved; the stack is the truth across tiers.
- **As built (AI4):** `sf_ai::directive` — limits, `Serving`, instance ids: [ai-as-built.md](ai-as-built.md) "AI4".

## Drives (utility arbitration)

Each consideration maps an input (threat distance, health ratio, a stat, disposition, stimulus) through a response
curve; a drive scores the compensated product (IAUS) times its weight. The running drive gets momentum and a minimum
commit time; *Directive* has a floor so content wins unless a reaction clearly beats it. Morrowind's Hello, Fight,
Flee and Alarm rules are adapter considerations.

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
`Wait`, `Activate`, `Flee`.

Fliers skip the navmesh (3D steer with probes); swimmers use water areas. The **classic executor slot**: an operator
set registered under the same task ids (e.g. an `AiWander` port in `mw_rules`), never built early.

## Perception and stimuli

- **Sight**: cone, range, LOS (`portal_query::line_of_sight`, batched rays, ≤ 2 per check); adapter
  `AwarenessModel` modifiers (sneak, light, invisibility).
- **Hearing**: `Stimulus { tick, kind: Sound { loudness } | Touch | Damage | Notify(record), source, at }`, reach by
  `portal_query::path_distance` (doors attenuate); action.md's `noise: Loud` emits one.
- **Detection is per player** (net.md): `Awareness { target, level, last_seen: Location, tick }`, bounded (8 entries,
  lowest dropped). It feeds world-state keys.
- **Event-time queries are tier-independent**: crime witnesses, alarms and greetings run a full check over every
  simulated actor in range when the event happens, whatever their tier.
- Generic crime hook: `OffenceObserved { offender, kind: RecordId, witnesses, at, tick }`; bounty, guards and arrest
  are adapter rules.
- **World reactions** (noticing changed objects, suspicion, investigate/report/tidy, NPC culprits, reset):
  [world-reactions.md](world-reactions.md), slices AI19-AI25 and AIM2 there.

## Memory and relationships (Nemesis-style; owner decision 7)

A host-side **fact store**, the social layer every NPC queries and systems append to.
- `Fact { id, kind: FactKindId, subject: ActorRef, object: FactObject /* PlayerKey | ActorRef | Faction | Placement */,
  value: i32, salience: f32, at: GameTime, decay: Decay }`, kinds as records. Per-NPC facts are bounded (default 32;
  lowest salience evicted), salience decays per kind; world facts (crime records, faction relations) are compacted.
- **Relationships**: `Relation { a, b: ActorRef | PlayerKey, affinity, fear, rivalry, history: SmallVec<FactId> }`,
  updated by fact-kind rules. Players are keyed by `PlayerKey` (the character), so memory survives sessions and works
  for any number of players.
- **Nemesis-style** `MemoryRules` (content): encounters (defeated by, fled from, spared), rivalry escalating to a
  vendetta (a `Memory` directive: seek, ambush), survivor returns, promotions via `sf_stats` groups. The framework
  ships store, decay, queries and events. Saved in `sf/aimemory` (one segment: see "Saves"); dialogue filters and scripts read facts.
- **Vendettas may cross worldspaces, bounded** (decisions (g); Nemesis ambushes, Skyrim's hired thugs): the host picks
  an encounter point near the target player (a link end or route node out of every player's view) within `max_hops`
  and `ttl`; the actor travels on tier 0 and is promoted out of view, then seeks/ambushes. Only actors the quest-safety
  validator allows; a target `PlayerKey` offline pauses the directive (never retargets); expiry returns the actor via
  its stack. Other memory goals stay local.

## Smart objects (`sf_interaction`, planned, own spec)

`sf_ai` needs `find_slots(ws, near, radius, tags)`, `claim`/`release` (host state, net.md) and per activity a method
fragment (approach, enter, loop, exit) with animation hooks. Until then the adapter's pathgrid wander stands in.

## Tiers and budget (owner decision 5)

| Tier | Who | Decides | Moves | Perceives |
|---|---|---|---|---|
| 2 full | allocator winners in the simulated set | 5-10 Hz staggered + events | nav corridor + ORCA + mover + probes | full, per player |
| 1 reduced | next band in the simulated set | ~1 Hz staggered | corridor, mover without foot probes | distance + same space; event-time checks full |
| 0 logical | everyone else, every worldspace | events (window edge, directive done, fact) | `(route, progress, until)` on `WorldGraph` + coarse graph, travel time | none |

- **Allocator** (the `AnimBudget` pattern): significance from distance to the **nearest player of all**, visibility to
  any player (through an open seamless doorway too), combat, dialogue and quest pins; hysteresis on both bands; one
  **global** cap across regions, so four players in four towns share the budget. Caps are **fixed counts** per CPU
  tier (UE MassLOD style), never adapted to measured time, so traces and the tier-invariance test reproduce;
  `ai.stats` warns when the tick budget is exceeded (decisions (d)).
- **Behind an open door** (decisions (k)): the far R0 around the mapped anchor joins the simulated set; its actors
  take tiers from the same allocator, physics steps, scripts run; no stand-in mode.
- **Pins outrank caps**: combat, dialogue and scripted scenes stay tier 2; if pins exceed the cap it is exceeded and
  `ai.stats` logs it, never a fighter demoted.
- **Tier invariance** (the multiplayer rule): a tier changes how often and how precisely an actor is stepped, never
  which rules apply. Directive completion, schedule activities at window edges, arrivals (from route length and
  speed), crimes and facts follow one logical clock, so world outcomes match on every CPU tier; only fine-grained
  motion differs.
- **Promotion resolve**: compute where directive/schedule state puts the actor at tick `t` (route progress or
  activity slot); snap to the nearest navmesh point on the route; claim the slot or an alternative; if a player could
  see the spot within the near radius, delay or enter from an occluded approach (door, corner); never promote combat
  in view. **Demotion** only out of view, past the outer band, never in combat or dialogue. Scripts see one actor in
  every tier (KCD's rule).
- **Fast-forward** (rest, wait, long travel): `fast_forward(actor, hours)` advances tier-0 state; the adapter keeps
  OpenMW's `fastForward` rules.
- **CPU tier** = host setting `sim.cpu_tier` (auto from cores, overridable on servers), apart from the graphics
  preset and `sf_scalability`'s player groups. Clients vary only presentation (animation LOD, interpolation).

| CPU tier | Reference | Tier 2 / tier 1 caps | AI main-thread budget per tick |
|---|---|---|---|
| Floor | Steam Deck (4c Zen 2) | **48 / 160** (research §8) | ≤ 0.55 ms |
| Medium | 6-core desktop, Steam Machine class | 80 / 260 (provisional) | ≤ 0.75 ms (provisional) |
| High | Ryzen 7 7435HS laptop (reference) | 128 / 400 (provisional) | ≤ 1.0 ms (provisional) |
| Ultra | 12+ cores | 192 / 600 (provisional) | ≤ 1.3 ms (provisional) |
| Dedicated | headless server, no render thread | 256 / 1000 at 8 cores, then measured per core (provisional) | ≤ 2.0 ms (provisional) |

Floor split per tick (research §8): steering 0.20, perception 0.15, decisions 0.10, tier 0 0.05, facts 0.05 ms; paths
0.30 ms on a worker. Two ticks run per 30 fps frame; animation has its own `AnimBudget`. Provisional rows are set by
**AIM**: tier-2 counts 32-256 in Balmora and a Vivec canton, 1 and 4 players, AI + mover/probes + animation ms
recorded together (tier 2's real cost is usually animation and probes: 128 × A0 ≈ 3.2 ms); the cap is the largest
count whose sum fits the tier's character budget. In single-player Morrowind the cap rarely binds.

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

Sections (owner-keyed, versioned, save.md): `sf/ai` per actor (directive stack, schedule cursor, tier-0 state
`(route, progress, until)`, current activity and claim, drive and running directive id) and `sf/aimemory` (facts,
relations). Not saved: plans, traces, awareness, path requests, `DirectiveDone` events (rebuilt on load; promotion
resolve places actors). Belongs to the one state registry when it exists (vision §4.2.3).
- **Section names are single-segment.** `sf_save` derives a section's family as the name up to the first `/`
  (`sf_save::SectionKey::family`, resolved by family alone), so a second section named `sf/ai/...` would clash with
  `sf/ai`'s spec, version and migration chain — registration either refuses it or it silently inherits the wrong
  steps. Hence `sf/aimemory` for AI13. Multi-segment names are for *instances* of one family (`sf/character/<key>`,
  declared with `SaveSection::family`), never for a second family (audit 2026-11-09).
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
| Top layer | KCD II routines, RDR2 and Cyberpunk schedules, Skyrim procedure trees, OpenMW `AiSequence` | directive stack + `StackPolicy` | **match** |
| Reactions | UE AI Perception sources; Hitman, KCD II, RDR2 witnesses and suspicion | world-reactions.md concerns | **match** (planned) |
| Selection | utility: Sims, GW2 IAUS, OpenMW combat ratings | drives + momentum | **match** |
| Behaviours | HTN: Horizon, Transformers FoC, Killzone; GOAP: F.E.A.R., KCD II; BT/StateTree: UE, CryEngine | shared HTN library, MTR | **match** (HTN: designer control, cost) |
| Schedules | Ultima VII, Gothic, Bethesda packages, KCD routines | records → directives | **match** |
| Smart objects | Sims, UE Smart Objects, Skyrim furniture, A-Life smart terrains | `sf_interaction` | **match** (planned) |
| Perception | UE AI Perception; OpenMW `awarenessCheck` | per player, through doors | **match** |
| Off-screen LOD | UE MassLOD caps, Bethesda process levels, A-Life, KCD | 3 tiers, global cap, resolve | **match** |
| Memory | Shadow of Mordor Nemesis; Bethesda crime per faction | fact store + `MemoryRules` | **match** |
| LLM NPCs | inZOI/ACE on-device, Mantella out of process, Where Winds Meet chatbots | advisor over validated options | **differ** (choices only) |
| Debugging | UE Gameplay Debugger + Visual Logger, Guerrilla in-game HTN view | trace from slice 1 | **match**; recording later (P2) |

Sources: research "Sources" (UESP, Warhorse GDC 2025/2026, Guerrilla, Humphreys, Orkin, IAUS, UE docs, OpenMW
`b8285cc9`).

## Tests

Logic exact; key test **tier invariance**: one scripted day at cap 48 and at cap 400 gives the same directive and
activity log at window edges. Feel: sanity only; budgets as `#[ignore = "measurement: ..."]`.

## Build plan

One deliverable plus its test per slice. Visual checks use prefix `AI-`.

| Slice | Deliverable | Test |
|---|---|---|
| **AI0** docs | this spec, nav.md, ai-adapter.md; doc changes after the owner's go | doccheck |
| **AI1** trace | `sf_ai::trace` ring buffer, JSON dump, `ai.trace` (debugger first) | synthetic trace dump exact |
| **AI2** HTN core | records, typed state, decomposition, partial plans, rejections traced | exact decompositions |
| **AI3** overlay | `sf_ai_bevy` gizmos, `ai.select`/`ai.tiers`, BRP | headless smoke; log line |
| **AI4** directives | stack, `StackPolicy`, completion events | exact orders, policy double |
| **AI5** saves | `sf/ai` section | round trip; old saves load |
| **AI6** move executors | `MoveTo`, `Wait`, `Face` over `sf_nav` (then MA3 drops `PathWalkScaffold`) | headless goals reached, no stuck |
| **AI7** drives | arbiter, considerations, curves, momentum | exact scores and switch ticks |
| **AI8** perception | sight, hearing, stimuli, per-player awareness, batched LOS | exact on synthetic scenes |
| **AI9** allocator | tiers 2/1, fixed `sim.cpu_tier` caps, pins; per-actor tier-2 cost in `ai.stats` | exact assignment + hysteresis; cost line logged |
| **AI10** tier 0 | logical state, promotion resolve, `fast_forward` | tier invariance; no in-view promotion |
| **AI11** schedules | records, `ScheduleRules`, quest-safety validator | exact pushes at window edges |
| **AI12** doors | `Traverse` (open, cross, close behind), fade-only "unseen" rule (supersedes seamless SI14; seamless branch on the `sf_portal` core, SI2) | follower through both door modes (seamless on the core double) |
| **AI13** memory | fact store, relations, decay, `MemoryRules`, bounded vendetta encounters, `sf/aimemory` | exact decay, eviction, round trip; vendetta pauses when its target is offline |
| **AI14** smart objects | `UseSlot` + fragments (after the `sf_interaction` spec) | claim conflicts deterministic |
| **AI15** combat | C6 operators, attack tokens, reactions | scripted duel, tokens respected |
| **AI16** net | replicated AI state via `sf_net` (after N2) | loopback sees activity, barks |
| **AI17** Luau | `@sf/ai` (when Lua is unparked) | push/done round trip |
| **AI18** advisor seam | `Advisor`, protocol schema, null advisor, deadline | late answer ignored; saved |
| AI19-AI25, AIM2 | world reactions: concerns, notice drive, discovery by tier, investigate/report, suspicion, tidy, consequences ([world-reactions.md](world-reactions.md)) | there |
| **AIM** measurement | Floor and High cap sweeps ("Tiers and budget"), AI + mover + animation ms together | owner runs; provisional rows filled |

Order: AI1 → AI2 → AI3 → AI4 → AI5 → AI6 (needs NV7a, NV8, seamless SI3) → AI7 → AI8 → AI9 → AI10 (needs NV12) →
AI11 → AI12 (needs NV11, SI2) → AI13 → AI15 → AI14 → AI16-AI18 later. NV0 runs in parallel with AI1-AI3. The
adapter's schedules pack (MA8) waits for AI10-AI12. AI10, AI12 and NV11 are seamless M9b (they replace SI13-SI15).

## Open questions

None open. Decided 2026-11-08 (`../research/ai-nav-decisions-2026-11-08.md`): (d) High 128 / 400 provisional fixed
counts, set by AIM; (e) schedules after AI10-AI12 (patterns in the adapter); (g) vendettas cross worldspaces, bounded.

## Doc changes for shared files

The ready-to-paste DECISIONS bullets and the exact edits for net.md, CONTRIBUTING, streaming.md, action.md,
ARCHITECTURE, ROADMAP (3.9, 3.10) and SYSTEMS.md are in `../research/ai-nav-decisions-2026-11-08.md` "Ready-to-paste
edits for shared docs" (they supersede the list this section held).
