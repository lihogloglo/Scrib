# Navmesh and AI

## What it does
The framework has a navmesh core with a Recast backend, a bake, a query service, a corridor driver and a debug overlay,
plus an AI core (HTN planner, directive stacks, drives, primitive move executors, decision traces). In the game, baked
navmesh tiles are resident around the player and ground `AI_W` wanderers plan through the `sf_ai` wander/idle HTN
method set (`mw_ai::methods`) and walk over `sf_nav` via `NavCorridor` (`PathWalkScaffold` is gone).

## How it works
- **Navmesh core** (`sf_nav`, Bevy-free): agent classes, `AreaTable`, `QueryFilter`, sliced `PathJob`, `PathCorridor`
  (`sf_nav/src/corridor.rs:88`: surface move, local push repair, lookahead trim, topology + visibility optimization,
  string-pulled corners), and the `NavBackend` trait (`sf_nav/src/backend.rs:344`: bake tile, residency, `nearest`,
  sliced `find_path`, `raycast`, `move_along_surface`, `corridor_corners`, `repair_corridor`).
- **Backend** (`sf_nav_recast`): `RecastBackend` over the pure-Rust `navmesh-rs` (Recast + Detour). One Detour mesh per
  (worldspace, agent class, region); tiles are DNAV v7; region-local f32 meters; stale refs fail by salt; corridor
  queries in `sf_nav_recast/src/corridor.rs`.
- **Bake** (`sf_nav_bake`): `nav.tiles` (per tile, reads recorded as dynamic inputs so a moved wall re-bakes only its
  tiles; door volumes get own polygons) and `nav.links` (off-mesh links). Backend-agnostic.
- **Morrowind adapter** (`mw_bake/src/nav.rs`): `MwNavSource` feeds the cooked collision of the mover's solid set
  (`is_solid`, `:140`, over `mw_world::source::MOVER_SOLID`: statics, items, doors, containers, activators; the
  physics hook uses the same list) and baked terrain heights;
  `bake_nav` (`:960`) runs inside `bevyrrowind bake` (`mw_boot_bevy/src/bake.rs:73`). Water is a flat swim-level quad,
  as in OpenMW.
- **Test levels** (`mw_exterior_bevy/src/level/nav.rs`, AI47b): `level:` scenes bake their own tiles at load from the
  level's static colliders (props and `collide: false` left out) through the same `nav.tiles` step: one region at the
  origin, humanoid only, no links, cached in `<store>/level-nav` keyed by the geometry digest; log `nav: level tiles N
  in X ms` (perception lab 60 tiles, ~80 ms cold in debug). The level's `placed` Morrowind records are not in it.
- **Nav runtime shell** (`sf_nav_bevy`): `NavMeshes` resource, `nav.stats`, `nav.draw tiles|paths` debug layers, and
  `NavRingPlugin` (`residency.rs:158`): `sf_nav::ring::NavRing` (`ring.rs:209`) keeps the tiles within
  `ai.nav.ring.load_m` (256 m) of `NavAnchor` resident, drops them past `unload_m` (320 m), reads at most
  `tiles_per_frame` (4) store blobs per frame, nearest first. Scaffold `NavRingScaffold` (one anchor, synchronous
  reads) until NV5. Query service `NavQueryPlugin` (`sf_nav_bevy/src/queries.rs`, core `sf_nav/src/service.rs`): requests
  by `RequestId`, node-budgeted slices on `FixedUpdate`, replies at the next tick. Corridor driver `NavCorridorPlugin`
  (`sf_nav_bevy/src/corridor.rs:189`): `NavCorridor` steps `PathCorridor` in `NavSystems::Corridor` (inside
  `ControllerSystems::Drive`, between `Publish` and `Queries`) and feeds corners into `PathFollower`
  (`sf_controller/src/path.rs:158`) -> `MoveIntent`.
- **Game side** (`mw_walkers_bevy/src/nav.rs:41`, added at `exterior.rs:244`): Recast backend, humanoid class only,
  `NavPlugin`, `NavQueryPlugin`, `NavCorridorPlugin` and `NavRingPlugin`, anchor = the first `Gameplay` streaming
  anchor, draw frame from the floating origin. A bake without `nav.tiles` logs one line and adds nothing. Debug draw is
  compiled into every build for now (the crate enables `debug`).
- **AI core** (`sf_ai`, Bevy-free): ids, decision `trace`, typed `WorldState`, HTN `Library` and planner (`htn`,
  MTR compare, bounded backtracking), per-actor `DirectiveStack` with `ModernPolicy` (`directive`), `AiSave` shape for
  the `sf/ai` section (`save`), drives (`drive`: `sf.ai.drive`/`sf.ai.consideration` records, IAUS curves, pure
  `arbitrate` at `drive.rs:614` with momentum, minimum commit and the directive floor), and primitive move executors
  (`exec`: `MoveTo`, `Wait`, `Face`, `OperatorExec`, `PlanRunner` at `exec.rs:321` with dirty-key MTR replan,
  `Library::holds` per-step monitoring and `Pause` resume).
- **AI shell** (`sf_ai_bevy`): `AiPlugin`, trace store, `AiLibrary` loaded from records, `ai.select`, optional gizmos,
  and `AiExecPlugin` (`exec.rs:151`: `AiExecutor`, `AiSystems::Execute` in `FixedUpdate` inside
  `ControllerSystems::Drive` before `NavSystems::Corridor`, tunables `ai.exec.*`).
  Added by `mw_walkers_bevy::walkers::add` (`walkers.rs:768`); the `sf/ai` save section
  (`sf_save_bevy/src/sections/mod.rs:285`) is registered by `mw_saves_bevy::register` (MA22a): directive stacks
  and tier-0 logical states (`AiLogical`, with their due instants; section v2, MA22b) save and load.
- **What NPCs actually do** (`mw_ai`, Bevy-free; `mw_walkers_bevy/src/walkers.rs`, added at `exterior.rs:229`):
  `WanderVocabulary` + `WanderDriver` (`mw_ai/src/methods.rs:55,156`: HTN tasks `wander` / `do_idle` / `do_walk` over
  `sf_ai::exec::PlanRunner`, `AI_W` idle rolls and PGRD wander destination hints from `wander.rs`) drive ground
  wanderers over the resident navmesh via `NavCorridor` (`walkers.rs:703`), recording decisions into `AiTraces`
  (`AiActor` from `CharacterPlacement`), and falling back to PGRD polyline `PathFollower` walks only for fliers/swimmers
  or unbaked cells (`PathWalkScaffold` removed in MA3). No combat AI, schedules or following yet.
- **Perception** (AI8, 2026-11-25; `sf_ai` perception/profile/sight/light/search, `sf_ai_bevy` sight.rs, light.rs,
  hearing.rs): walkers are `AiObserver`s, the player is `Perceivable` (`mw_walkers_bevy/src/sight.rs`); per fixed tick,
  before `ControllerSystems::Drive`: profiles resolve, light is sampled (`AiLightInputs` from `light_inputs.rs`: sky,
  interior AMBI, LIGH lamps), sight rays batch under `max_rays_per_tick`, `NoiseEvent`s are heard (AI8c3: tier-2
  listeners by `WorldGraph::path_distance` through doors and into other worldspaces, closed doors + `closed_door_m`,
  `hearing.rs` `PortalPaths`; tier 1 straight line in its space; no `NavWorldGraph`: straight line, same space only). A walker whose strongest entry is Searching stops wandering and searches (`search.rs`: LKP, look,
  widening rings) while the band is held, until the ttl gives up, then decays and wanders again.
- **Tiers** (AI9; `sf_ai_bevy/src/alloc.rs`): the allocator gives each awake `AiActor` an `AiTier` (Full / Reduced /
  Logical, caps per CPU tier) and a staggered `first_decide_tick`. Sight checks at its tier's period (AI9e). The wander
  HTN plans on its decide ticks only (AI9g, `mw_walkers_bevy/src/walkers.rs:840` `AiDecideGate::due` ->
  `ExecContext::decide`, gate in `sf_ai/src/exec.rs:659` `ensure_plan`): Full every `ai.alloc.full_decide_s` (0.15 s),
  Reduced every `ai.alloc.reduced_decide_s` (1 s); a walker whose step ran out stands until then. Operators and events
  (dirty read key, failed precondition, failed walk) run every tick; untiered actors plan every tick; a tier-0 body
  stands (below); the pathgrid fallback is not gated. `ai.tiers` prints the plans of the last second.
  **Tier 0 in the game** (AI10c2, `mw_walkers_bevy/src/logical.rs:318` `add`, FixedUpdate, only with a `GameClock`):
  Enter (`:377`) stores every `AiActor` body with a stack whose `AiTier` is `Logical` or that is `Dormant` (past the AI
  radius, another worldspace, ground not loaded), not `Held`, at its body; Step (`:415`) runs the due states on
  `WorldGraphEnv` (`NavWorldGraph` routes at the walker's walk speed, door states, straight legs until NV12f) with
  `Renewing`, ends feed `AiRuns` and repeats move their extras/home (`:442`); Promote (`:485`) resolves the shell's
  queue on `ViewsEnv` (the `AiViewer` frusta in the active worldspace, `NavMeshes` nearest) and moves each placed
  body (position, zero velocity, `Mover::snap`, corridor goal cleared, wander re-chosen, `GoTo` restarted); a
  placement in another worldspace puts the state back (AI10c3 moves bodies). A marked body stays rendered where it
  stood: `drive` stands it first (`walkers.rs:809`), `drive_directives` skips it (`directive_drive.rs:318`); script
  pushes and late packages `wake` it; a script push to a bodiless tier-0 actor lands on its stored stack (MA2c3a,
  `script_ai.rs` `apply_script_pushes`), and a dead actor reads `GetCurrentAIPackage` -1 (`clear_dead_runs`); the reads' `AiRuns` ride saves as `mw/ai_runs`
  (MA2c3b, `mw_saves_bevy` `AiRunsSave`).
  Proof line `tier0: N logical, Q queued, P placed (X other-space)` (≤ 1/s).
  **Bodies keep their pose across a cell unload** (AI10c3a, `mw_walkers_bevy/src/poses.rs:86` `ActorPoseHook`,
  installed on the actor class at `directives.rs:273`): when an actor placement leaves R0 (not a discard), a body
  that moved more than 1e-3 m on the ground plane or 1e-3 rad of yaw is `PlacementOverlay::move_to` its
  `ActorSpace`, feet and yaw (spawn turn taken off), so it reloads with the cell it stands in and `attach` respawns it
  there; tier 0 re-marks it (Enter). `persist_actor_poses` (`:127`) is the save path (MA22, not wired). Proof line
  `actors: N poses kept at unload` (≤ 1/s).
  **A body that walks into another loaded cell becomes that cell's** (AI10c3b2, `mw_walkers_bevy/src/crossing.rs`):
  `attach` gives each body a `BodyCell` (`:54`) from its ancestor `StreamCell`; `rehome_crossers` (`:178`, `Update`,
  after the streaming drive) compares it with the cell under the body's feet and asks `Streaming::rehome`
  (streaming.md "Re-homing a placement"). On success it writes the pose to the overlay (`poses.rs:178`
  `write_crossing`), then one entity command sets `ChildOf`, the local translation, `Mover.frame_origin` and
  `BodyCell` together, so the drawn position does not move; a refusal changes nothing and is asked again next
  frame. `Held` bodies wait for their release. Gap: a body whose placement no Active list and no overlay entry
  holds is not re-homed. Proof line `actors: N re-homed` (`, M without a held placement` when some wait; ≤ 1/s).
  **A bodiless tier-0 actor's placement follows it** (AI10c3c2a, `mw_walkers_bevy/src/bodiless.rs`): deactivation
  keeps the placement as the overlay holds it (`ResidentActors` away map); `follow_bodiless`, last of the Step chain,
  moves it in the overlay (kept rotation) on the tick a routed, non-following stored actor's logical position enters
  another cell or worldspace, so that cell's next fetch spawns the body there; no body and no state = pruned. Gaps:
  an actor never loaded since start has nothing kept. Into an Active cell (AI10c3c2b1) it gets its body at once:
  the promotion rules pick an unseen spot on the navmesh (else it waits `ai.tier.promote_retry_s`),
  `spawn_bodiless` calls `Streaming::spawn_into` there, and the body is re-marked and promoted like any stored one.
  Proof (AI10c3c2b2): `ai.tier0: N bodies spawned into loaded cells, M kept placements moved` (counts since start in
  `sf_ai_bevy::AiBodilessStats`, also on `ai.stats`).
  Tier 0 core (AI10a, `sf_ai/src/logical.rs`): `LogicalState::step` jumps event to event
  (route arrival, `DirectiveStack::next_deadline`, `LogicalEnv::next_window_edge` hook), each served kind taken up
  through `LogicalTransitions::transition` (StayNear waits in place or walks home, GoTo/Escort walk and end
  `Completed` at the arrival minute, Follow stands at the leader's logical position, Use walks then holds, Fight holds,
  Custom registered; `validate` fails at load for an unregistered custom kind). Routes advance at walk speed in
  simulated seconds (game ms / `time_scale`), arrival fixed when the route is taken up. `fast_forward` = step + the
  adapter's `FastForwardRules`: Morrowind's is `mw_ai::MorrowindFastForward` (MA7a, `mw_ai/src/fast_forward.rs`:
  a `StayNear` top with radius > 0 is placed on one of OpenMW's `allowed_positions` of its cell's pathgrid, one roll
  seeded by actor key and span end, nudged 5/15/30 % toward a neighbour when the occupancy seam says taken). Time skip
  (MA7b): `advance_sky` records the span in `AiTimeSkips`; next tick the game demotes every awake actor with a stack
  (not dead, knocked down or paralysed) at the span start, `AiTierZero::fast_forward` crosses the whole store with
  `MorrowindFastForward` (live occupancy), and `promote_dark` places every queued body with no view test, retries
  cleared and no per-tick cap (`mw_walkers_bevy/src/logical.rs` module docs, 6). Promotion core (AI10b, `sf_ai/src/promote.rs`):
  `PromoteQueue` holds every actor granted tier 1/2 still tier 0 (pins, then nearest player, then key), tries at most
  `per_tick` a tick, a waiter retried after `retry_s` (skips free, so seen waiters never starve unseen ones);
  `resolve_promotion` places at `position_at(t)` snapped, else back along its route (`backtrack_step_m` up to
  `backtrack_m`), else `PromoteEnv::approaches`, only where `PlayerViews::unseen`, else waits; a refused `Use` slot
  claim stands it there. `demote(body, now)` = a fresh `LogicalState`; the allocator alone grants demotion.
  Shell (AI10c1, `sf_ai_bevy/src/logical.rs`; the game's systems above): `AiLogical` holds the states by key, and
  an actor is tier 0 in fact iff stored; its body carries `AiLogicalBody` (`demote_body`). FixedUpdate sets
  `AiLogicalSystems::{Hold, Step, Promote, Cleanup}`, after `AiAllocSystems::Allocate`, before sight and `Drive`: Hold
  pins a held body's `AiTier` to `Logical` and queues the allocator's tier-1/2 grants (its distance and pin); the
  game's Step and Promote systems call `AiTierZero::step` (due index: new states and `until <= now`, leaders before
  followers, followers read leaders through the store) and `AiTierZero::promote` (a body already on its state's spot
  is placed with no view test, else `resolve_promotion`); Cleanup gives a body that left the store its allocated tier
  (staggered). Tunables `ai.tier.promote_*`; `ai.stats` prints a `logical:` line.
- **Greeting** (MA4c2, 2026-11-26; `mw_walkers_bevy/src/greeting.rs`): every awake AI actor steps `mw_ai`'s OpenMW
  greeting machine (`greeting_step`, MA4c1) on the fixed tick in `GreetSystems` (`add` `greeting.rs:271`): Decide
  (`:311`, after `ReactionSystems::React`; asks for the Hello when a greeting starts), the voices' answer
  (`mw_audio_bevy` `say_hellos`, dev only), Apply (`:400`, before `drive_directives`), Hold (`:438`, after
  `ReactionSystems::Apply`: stand + facing). While it holds, the wander and a `GoTo` pause (`walkers.rs` drive,
  `directive_drive.rs`). No voice answer (non-dev builds): the greeting goes Done, no turn. "In combat" is a serving
  `Fight` directive only (`:358`, OpenMW `isInCombat`), not the Alerted band (AI-13: sight latches a seen player at
  Alerted within a second, which reset every greeting); gap: no drive pushes `Fight` yet (MA10 / AI15), so an NPC
  OpenMW would fight greets. A perception reaction still pauses it: in plain view at 1.5 m the NPC reacts, searches,
  walks back to its post, then greets about 2 s after first sight. Proof line on full updates, on change, at most once
  a second (`:393`): `greeting: N in reach (C in combat, P other package, U not seen, H player hidden or dead), G greeting`.
- **Worldspace graph** (MA21, 2026-11-26; `mw_exterior_bevy/src/nav_links.rs`, added next to `mw_doors_bevy::add`):
  `NavWorldGraphPlugin` over `DoorLinks`; each link end's approach point is the PGRD node nearest its arrival, in front
  of the door plane and within `mw.nav.door_approach_m` (4 m), else the arrival (vanilla: 77% of 3656 ends from PGRD);
  classic fade-only flags; links whose door reference has a lock level > 0 read `Locked` (static, from the load).

## Bevy features used
Resources and console commands only; gizmos behind feature `debug`. No Bevy navmesh crate (surveyed in the spec).

## Switches and counters
`nav.stats` (tiles per class, polys, MiB, queue/tick stats, and a `ring` part: pending, empty, unbaked, failed, loaded, dropped, last ms),
`nav.draw tiles|paths|off` (outlines along the detail mesh, `TileGeometry::for_each_edge`), tunables `ai.nav.ring.*`,
`ai.nav.query.*`, `ai.nav.corridor.*`, `ai.exec.*` (live). Log: `nav ring: N tiles resident in <worldspace> (...)` once
per worldspace. `ai.select`, `ai.trace`, `ai.tiers`, `ai.stats` and `ai.gizmos` work in game on `AiActor` entities. Walkers' physics follows
`--off physics` (`exterior.rs:241`). No nav or AI `--off` part yet.

## Known limits
One anchor (the player), humanoid class only, synchronous store reads: portals and the large class wait for
NV5 ([ledger](../../ledger/nav.md)). Only `AI_W` wander and search run on `sf_ai` so far; packages (`MA2`), drives
(`MA4`), Classic awareness (`MA5`) are next. No game noise emitters yet (AI8c2); pitch dark hides fully from sight
(visibility from crime's `DetectionRules` via `AiDetectionRules`, default `StandardDetection`: light, stance, speed, facing; `sf_ai_bevy/src/detection.rs`, LW4b1; the player's sneak mode sets `Stance::Crouched` there, `mw_walkers_bevy/src/visibility.rs`, MA18); Reactions (AI8i, `mw_walkers_bevy/src/reaction.rs`) are a layer over the routine: Searching pre-empts wander, post or level script for any ground actor with a corridor, then the actor walks back to its anchor and facing; Curious turns to face the stimulus and Suspicious walks `ai.perception.react_step_m` (3 m) toward the last-known position, then stands facing it until the band drops (AI8h2). Alerted NPCs do nothing yet. See `docs/ROADMAP.md` 3.9/3.10.

## Specs
`superfargoth/docs/specs/nav.md`, `ai.md`, `ai-as-built.md`, `orca.md`; `bevyrrowind/docs/specs/ai-adapter.md`,
`actors-adapter.md`.
