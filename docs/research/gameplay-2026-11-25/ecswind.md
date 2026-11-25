# ECSwind: ideas worth a wishlist entry (research note, 2026-11-25)

Source: `D:\Gamedev\ECSwind` (C++ / flecs / raylib, turn-based action queue). Read: CLAUDE.md, the framework design
doc, masterplan (phases E-G, anti-pitfalls, risks), M27-M29 trackers, primitive-simulation-doctrine, ADR index,
`data/`, and `src/` by grep. Prior project = anecdote: ideas only, no tiers, constants or code carried over.
Paths below are relative to `D:\Gamedev\ECSwind`.

## Summary

ECSwind aimed at a Caves-of-Qud-like "every NPC has a life arc" sim on a Morrowind seed: persistent world that ticks
without the player, faction politics, gossip, history, tavern songs. What actually got built (M0-M29, ~30k lines C++):
ESM/NIF/BSA import and a viewer, a single flecs world, a turn-based action queue (three time granularities), a
spatial/tile substrate with cell reification and delta saves, TOML blueprints, a 19-verb affordance system, inventory,
a goal stack + ThinkTree (selector/conditional/leaf) + Job/Toil composer, A* over PGRD path grids, hunger + time of day
+ hour-band schedules, personality facets and drive weights, and a physical-interaction kernel (damage, harvest,
disassemble, repair, build, reservations, persisted). A monthly demographic sim (births, marriage, divorce,
inheritance, death, event log with reconstructable biographies) ran for tracked people, including ESM NPCs.
Everything social (mood, relations, factions, secrets, narrative events, conversation, history, worldgen) in
M30-M44 is **designed only**. The last pivot (primitive simulation doctrine) restarted gameplay on a boxes-and-tiles
truth layer with rooms, doors, ownership and reservations, still at smoke-test level. It never reached playable
systemic content: the only authored schedule is `guard`, the only jobs are `patrol` and `rest`.

## Idea table

| Idea | In ECSwind | Lane | Worth it? |
|---|---|---|---|
| Typed-secret knowledge: an NPC holds typed refs (Location, Event, Recipe); trade = set difference + topic tags; no text mutation | Designed only (masterplan M33, research s1.9/s9.10); no code | dialogue-quests, ai | Yes. Cheap, deterministic, replicable on a host; fits "gossip spreads reputation" and gives dialogue something to gate on |
| Gossip as reputation spread, not rumour mutation (explicit anti-pitfall: do not model mutation) | Designed only (anti-pitfalls) | crime, dialogue-quests | Yes. Matches the owner's walking-witness rule; carry the "no mutation" restraint |
| Faction rep as scalar bands, faction tree with inherit-down, per-NPC overlay | Designed only (M32) | crime, ai | Maybe. Morrowind already has faction rank/standing; the overlay (personal vs faction opinion) is the new part |
| Event log as one corpus: births, deaths, crimes, trades logged with subject/object; a biography is reconstructable from it | Implemented for demographics (`src/sim/sim.h` event kinds ~l.700, biography print ~l.804) | world, dialogue-quests | Yes. A host-side append-only log feeds rumours, tavern talk, journal and saves; cost is memory only |
| Score-function event matching (Wildermyth roles): events declare roles + expressions, engine fills roles from the live graph and fires the best match | Designed only (M34) | dialogue-quests, world | Maybe. Good for emergent small stories (grudges, thefts) outside the main quest; needs a tiny expression DSL and a data lane |
| World-state predicates as first-class data: one boolean read by dialogue gating, spawning, AI hostility and zone overrides (Kenshi) | Designed only (M36/M37) | dialogue-quests, world, crime | Yes. Single source of truth fits data-driven Morrowind globals; one flipped flag re-skins a zone |
| Drives as stable weights + momentary urgency, seeded from race/class, deterministic per NPC id across despawn/reify | Implemented (`src/sim/sim.h` Mind/DriveState ~l.412; `data/defs/drives.toml`, `personality_facets.toml`; M28 tracker) | ai | Maybe. Scrib already has IAUS drives; the useful bit is deterministic seeding from stable id + class defaults so an NPC stays the same person across streaming |
| Needs + hour-band schedules: a schedule entry maps an hour range to a Job; a need breaks the schedule, then it resumes | Implemented, one schedule (`data/schedules/guard.toml`, `src/brain/brain.cpp` ~l.814-870) | ai, world | Yes as pattern. A data-driven day plan (sleep, work, eat, pub) is the biggest "lived-in" gain; interruptible by needs |
| Job = list of Toils (walk_to, wait, reserve_target, invoke_verb, release); Toil handlers registered by id | Implemented (`src/brain/brain.cpp`, ADR-007, `data/jobs/*.toml`) | ai | Maybe. Equivalent to HTN methods + primitive tasks in `sf_ai`; borrow only "authored job in TOML, mod-spliceable" |
| Reservations: an actor claims a target (bed, chair, work spot, item) so two NPCs do not use it; released on finish | Implemented (`ReservedBy` `src/sim/sim.h` ~l.247; M29 toils) | ai, interaction | Yes. Needed for beds, seats, workbenches, loot; host-authoritative by nature; must expire if the holder dies or despawns |
| Affordance table: verbs require/forbid components on the target; player and NPC use the same verb set (parity) | Implemented (`data/affordances.toml`, ADR-004, `src/affordance/`) | interaction, ai | Yes. One interaction vocabulary lets NPCs use what the player uses and lets AI query "what can I do with this" |
| Ownership + claim + theft/crime hooks on the same affordance verbs | Designed (doctrine "Core systems", verbs `claim`, `reserve`); only `ReservedBy` found, no owner/crime code | crime, interaction | Yes as idea. An owner check inside the verb handler (take/open/damage) is the clean crime trigger; matches the owner's vandalism-when-owned rule |
| Physical integrity + materials: break/disassemble/build/repair; destroyed baseline refs and spawned results persist as versioned cell deltas | Implemented (M29; `data/materials.toml`; delta v2) | world, interaction | Yes for the persistence model (deltas vs baseline, versioned). Maybe for build/disassemble depth (not in owner decisions) |
| Rooms derived from walls/doors/roofs by flood fill; breaching a wall recomputes indoor/outdoor and paths | Implemented in the primitive prototype (`src/primitive/primitive_world.cpp`, `recompute_rooms`, smoke test) | world, ai | Maybe. Useful for "indoors" tests (rain, sleep, trespass) in generated settlements; Morrowind interiors are separate cells, low value there |
| Gameplay lattice: sim-only tile grid beside the continuous world for occupancy, blockers, layers | Implemented (M27 tracker) | ai, world | No for core: navmesh and continuous space are our base; at most a coarse occupancy grid |
| Two tiers: tracked individuals vs abstract population bins; promote when the player notices, demote when irrelevant | Tracked tier implemented (M26); cross-tier promotion designed (M41); bins later parked | world, ai | Yes as concept (simulation LOD): full sim for named NPCs, coarse for crowds, plausible state on return. Fits the 120 fps budget |
| Monthly life-event tick for tracked people: ageing, marriage, divorce, children, inheritance, death | Implemented (`src/sim/sim.cpp`, `tick_month`, TickStats) | world | Maybe. Cheap (off the frame path), gives flavour; conflicts with Morrowind quest-essential NPCs unless they are exempt |
| Mood from decaying thought memories; mental breaks (RimWorld pattern) | Designed only (M30) | ai | Maybe. Decaying memory entries also serve "NPC remembers what you did"; skip mood bars and breaks |
| Typed relation edges with levelled points (friend/rival/lover) | Designed only (M31) | ai, dialogue-quests | Maybe. Per-pair edges only for named NPCs near the player, else memory cost grows quadratically |
| Tracery-like grammar for songs, rumours, books, names, with a debug console | Designed only (M35); a name generator exists (`src/sim/names.cpp`, `data/names/`) | dialogue-quests, world | Maybe. Barks, rumours and tavern talk from templates over the event log; handwritten corpus, no LLM at runtime (their rule too) |
| Quest annotations: deferred messages that wait for a zone to load (CoQ) | Designed only (research s1.7/s9.12) | dialogue-quests | Maybe. Pattern for quest objects that exist only once their cell streams in |
| Goal-stack over ThinkTree: the active goal/job wins until finished; the tree decides only when the stack is empty | Implemented (ADR-005/006; `src/brain/`) | ai | Maybe. Same as HTN commitment; confirms "interrupt only on need or threat" |
| Mod splice into AI trees (`[[splice]]`, no fork of the file) | Designed in ADR-005, not implemented | ai | Yes in spirit: mods extend AI data additively; `sf_mods` probably covers it |
| Influence as currency (Bannerlord), Sultans/mythic history, reverse-causality history, monotonic generator DAG | Designed only (M39-M44) | world | No for now. Large worldgen scope; Scrib's world lane is Magna Urbis. The rule "nothing exists that only relates to itself" is a review principle at most |

## Lessons

- Scope: about 30 milestones of substrate (import, fusion, action queue, tiles, blueprints, affordances, jobs)
  shipped before any social system; M30-M44 stayed on paper. The masterplan itself lists content (not engine) as the
  open risk (CoQ has 15 years of corpus), and the doctrine pivot says the real risk is whether it is fun as a physical
  simulation.
- Two ECS worlds (viewer and sim) had to be fused (ADR-001), and a half-merge was a named risk. Rendering and sim
  truth drifting apart (meshes deciding collision, room shape) led to the "assets are presentation, ECS is truth" pivot.
- The persistence format was flagged early as easy to half-do; the delta-against-baseline, versioned design (v1 still
  readable in v2) held up.
- Determinism was enforced by smoke tests: new systems (Mind, tiles, physical verbs) had to leave the seeded
  demographic run bit-identical, so new seeding avoids the shared RNG stream and keys on object id.
- The turn-based action queue admitted its cost: real-time combat and dodging need a separate non-turn mode, and idle
  frames must not mutate state. Action costs were tunables that "need tuning sessions".
- Their own anti-pitfalls, worth keeping: do not over-model (operate at what the player sees plus one layer), do not
  model rumour mutation, no LLMs at runtime, do not make GOAP or behaviour trees the whole AI.
- Multiplayer was deferred and judged cheapest if designed early; Scrib already does this (fixed tick, commands).

## Not to carry over

- Turn-based action queue, segment/turn costs and the month tick as the main clock: Scrib is real-time fixed tick.
- Their numbers: facet/drive weights, schedule hours, reputation band thresholds, the 20/60/120/200/300 relation
  levels, hunger categories. Scrib tunables come from its own measurements (`tunables.md`).
- Sprite/billboard top-down presentation and the tile "truth layer" as authority for collision and pathing: conflicts
  with Scrib's continuous 3D world and navmesh.
- Whole-population demographic simulation (invisible bins, generic births and deaths, 100-year fast-forward):
  conflicts with Morrowind's authored NPCs and quest dependence; at most optional for generated towns.
- "Break the main quest honestly, successor NPC takes over" and "play as the NPC's child": out of scope, conflicts with
  the modern-first Morrowind content port.
- Primitive-only content (boxes and debug overlays) and re-schemaing ESM records into their own format: Scrib reads
  Morrowind records through `sf_records` and adapters.
- One trade = one schedule: Morrowind AI packages (wander, travel, escort, follow) stay the content source;
  schedules are an extra layer, not a replacement.
