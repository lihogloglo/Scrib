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

Rewritten for the nav lane roadmap (`../../../docs/roadmaps/nav.md`, NV0d 2026-11-26); the earlier text and the NV4b
as-built paragraph are in [archive/ai-adapter-history.md](archive/ai-adapter-history.md).

- **Agent classes** (`sf.nav.agent` records from `mw_ai::nav`): `humanoid` (radius and height from the human capsule
  at race scale 1.0; small creatures use it too) and `large` (creature collision bounds above a size threshold). **Two
  classes, no `small`** (decisions (a)): no vanilla passage admits only small creatures, and a small class would cost
  ~4× the humanoid bake. MA1 prints the creature-radius histogram per class (a revisit trigger). OpenMW bakes one mesh
  per agent bounds (`components/detournavigator/navigator.hpp`).
- **Areas** (`sf.nav.area`), from OpenMW's `areatype.hpp` (ground, water, door, pathgrid) plus ours: `ground` 1,
  `water` (cost per actor profile, below), `door` 2 (OpenMW's door cost; door leaves excluded from the bake geometry,
  the doorway footprint marked), `door_closed` (set at runtime on those polygons), `path` (pathgrid corridors, slightly
  cheaper so NPCs keep to streets), `hazard` (lava: avoid filter). No vanilla record sets `carves_nav`.
- **Geometry**: exactly the cooked collision the mover uses (`collision.cook`), terrain from LAND, water level per
  worldspace; NIF collision quirks are the bake's (actors-adapter "Static collision"). Source as built: NV4b
  `mw_bake::nav` `MwNavSource` (history file above).
- **Every worldspace has a navmesh; pathgrids are bake input only** (nav roadmap decision 6). Tiles are baked, or a
  missing one is built off the frame and cached (NV10c; owner 2026-11-26: never a stall while playing), so there is no
  runtime pathgrid route. PGRD (`mw_ai::pathgrid`) feeds: off-mesh links only where the navmesh has no connection
  between the endpoints (OpenMW `offmeshconnectionsmanager.hpp`; NV9d), wander hints, door approach points (MA21,
  "Doors and followers"), road chains (route source below).
- **Scaffold `PgrdFallback`** (named, to delete): `mw_walkers_bevy/src/walkers.rs` `drive`, the off-navmesh branch
  calling `mw_ai::methods::WanderDriver::update_fallback` (fliers, swimmers and walkers without a resident tile walk
  PGRD polylines), and `setup_of`'s swimmer `MoverMode::Fly { hover: None }` (fallfix). Removed by **MA12**
  (`docs/roadmaps/ai.md`), after NV13b, NV13c, AI34 and AI35.
- **Movement classes** (CREA flags, in `mw_anim::rig::CreatureMover::from_crea_flags` order, `Flies` first). An
  actor = a class plus an area-cost profile (a per-actor Detour query filter: nothing re-bakes; costs in NV9d):
  - NPCs (every race): ground class by size, `water` ×5: they swim after you, slower (decision 8, owner Q2).
  - `Walks` without `Swims`, and creatures with neither (biped dremora carry only `Biped`): ground class by size,
    `water` excluded: they wait at the shore (owner Q2).
  - `Walks | Swims` (mudcrab): **amphibious = ground class with `water` ×1**, no class swap at the shore (decision 8:
    swimming costs, not links; the swim quad already joins the shore polys).
  - `Swims` without `Walks` (slaughterfish, dreugh): **swimmer** class in the water-body volume (decision 2, NV13c),
    never the navmesh.
  - `Flies` (cliff racer, netch): **flier** class on the clearance field (decision 2, NV13a-b).
  - Levitate active (NPC or creature): class swap to the field; at its end, back to the ground class at the nearest
    poly (decision 2).
- **Water bodies (Morrowind inputs to nav.md "Volumes", NV0b; built in NV13c, costs NV9d).** One flat water plane per
  worldspace: the exterior sea at 0 m (`mw_world::WorldRecords`, `sea_level`), and each interior whose CELL has
  `HAS_WATER` at its WHGT height (`CellEnvironment::water_level`, 0 m when WHGT is absent). Per body the adapter hands:
  the worldspace id, the surface height (m), the extent (the worldspace's nav extent: exterior = the bake's tile extent,
  interior = placement bounds + `INTERIOR_PAD`) and the bed from the same `NavGeometrySource` geometry the tiles bake
  (baked terrain heights + cooked collision), so no second source; a column with neither (open sea past the last
  cell) has no bed (depth rule: NV0b). Vanilla and Morrowind-format mods have no lakes or rivers at their own heights
  (the Odai is the sea), so one body per space; generated worlds' lakes and rivers come from world10x water bodies
  (NV9e, `sf_worldgen`). The navmesh swim quad (OpenMW `getSwimLevel`) stays for ground classes; the volume serves
  swimmers. Dependencies: CELL flags and WHGT, the tiles' terrain and collision products.
- **Route source (NV12c, `mw_bake`, input of `nav.routes`; nav.md "Route graph").** `RouteSource::route_input` gives
  one `RouteInput` for the exterior worldspace (`ids::exterior()`); interiors, "behave like exterior" ones included,
  give none (a straight clique of their exits in `journey`, NV12e).
  1. **Nodes and edges**: every node of every exterior PGRD (`PathGrids`, `CellKey::Exterior`) is `add_node(pos,
     Junction)` (the builder sets Junction/End from degree); every stored connection is one `two_way` edge, kind
     `Path`, `cost_m: None` (polyline length; NV12d measures). One-way connections become two-way (a road walks both
     ways; OpenMW keeps them directed per cell, `pathgrid.cpp` 174-190), counted. Vanilla has no road flag: every
     edge is `Path` in v1 (road LTEX under an edge is a possible later signal).
  2. **Cross-cell joins** (ours: OpenMW builds one `PathgridGraph` per `ESM::Pathgrid`, `apps/openmw/mwmechanics/
     pathgrid.cpp` 174, cached per record in `aipackage.cpp` 320-328, and crosses cells on the navmesh only; read at
     `4e718b500ffee6657db90297d7b8a08a57d439bc`). Each pair of 8-neighbour exterior cells is visited once (offsets
     (1,0), (0,1), (1,1), (1,−1) from the lower `(x, y)`); nodes a in A and b in B get a two-way `Path` edge when each
     is the other's nearest node in the other cell and their 3D distance ≤ `mw.nav.route.join_m`. Distances squared
     in f64; ties: lower `(cell x, cell y, point index)`. No height test: NV12d's Detour walk clears the class bit of
     a join nobody can walk.
  3. **Chains**: the adapter folds nothing; `RouteGraphBuilder::build` turns degree-2 nodes into polyline points and
     keeps junctions, ends, anchors and doors as nodes.
  4. **Settlement anchors**: exterior CELL records with a non-empty name (region-only cells have none), grouped by
     ASCII-lowercased name; tag = that name (`balmora`, `caldera`). One anchor per group: among the group's PGRD nodes
     in the largest component (union-find over the input after joins, by node count; tie: the component with the
     lowest `(x, y, index)` node) touching the group, the node nearest (XZ) the mean of the group's cell centres; ties
     lower `(x, y, index)`. That node goes in as `Anchor(tag)` instead of `Junction`. A group with no node: no anchor,
     counted.
  5. **Door ends**: each end of a `DoorLink` (`WorldRecords::door_links`) in the exterior is `add_door(link.id(), side,
     approach)` with MA21's approach (PGRD node of the arrival's square + 8 neighbours, not behind the door plane, ≤
     `mw.nav.door_approach_m`, else the arrival). NV12c moves `approach_point` (with the yaw of
     `mw_doors_bevy::arrival_rotation`) from `mw_exterior_bevy/src/nav_links.rs` into Bevy-free `mw_ai::nav` (new module, no new dependency: `glam`, `sf_world`, `mw_world` are there); the bake
     and MA21 call that one function, so door nodes equal `WorldGraph` approaches bit for bit. Node id = `RouteNodeId`
     over (`LinkId`, side) (nav.md), the key NV12e joins on; the builder connects doors (`door_connect_m`). The bake's
     own approach (`mw.nav.links`, ≤ 6 m from the door volume's floor centre, NV4b) feeds only `nav.coarse` and goes
     with it in NV12d.
  6. **No island filter**: small components are real places (camps, shrines, islets) and carry door nodes; queries
     answer `Unreachable` or `NoJoin`. The measurement prints a component-size histogram (revisit trigger).
  7. **Dynamic dependencies**: every exterior PGRD, every CELL record (names, door references, destinations), plan
     params `mw.nav.route.join_m` and `mw.nav.door_approach_m`; a mod editing any of them re-bakes the exterior. A
     live change of `door_approach_m` moves `WorldGraph` approaches only until the next bake (ids stay).
  8. **Determinism**: the builder is order-free; adapter choices break ties by `(cell x, cell y, point index)` and
     `LinkId`, never by record iteration order.
  - **Tunable**: `mw.nav.route.join_m` 20 m (0-117, one cell; Guess; bake). Reused: `mw.nav.door_approach_m` (MA21).
  - **Tests (NV12c, exact, synthetic records, `simplify_m` 0)**: two cells (0,0)/(1,0), one 3-node line each ending
    4 m from the shared border: one join, one edge, exact node/point counts; gap of `join_m` + 0.01 m: two components
    (`Unreachable`); mutual nearest (one node in A, two in B at 3 and 5 m: only 3 m joins; equal: lower index);
    diagonal (1,1) join; Y-shaped PGRD folds to 1 Junction + 3 End, 3 edges; a one-way connection is two-way and
    counted; anchor on the node nearest the group's centre, a nearer node in a smaller component skipped, tie by
    index, unnamed cell none, named cell without PGRD counted, `anchor("testtown")` found; door node position ==
    `mw_ai::nav::approach_point` (behind-plane node ignored, none within 4 m → arrival), id == `RouteNodeId` of
    (`LinkId`, side), interior end adds nothing, MA21's `nav_links` tests unchanged; interior and behave-like-exterior
    PGRDs ignored; shuffled record order → identical postcard bytes.
  - **Measurement** (`#[ignore = "measurement: ..."]`, real install): nodes, edges, points in and out vs nav.md
    "Budgets" (Vvardenfell ≤ 5k nodes, 6k edges, 60k points, ≤ 1.5 MiB), joins, one-way count, anchors (groups
    without one), door nodes (pathgrid vs arrival; joined vs isolated), component histogram, bake ms; asserts the
    Balmora → Caldera anchor route is `RouteKind::Graph`, not `Direct`.
  - **As built (NV12c2, 2026-11-26):** `mw_bake::nav::routes` (`MwRouteSource`, `exterior_route_input`, `bake_routes`
    called from `bake_nav`, `NavReport::routes`). The input is built once from the records; its postcard bytes plus
    both tunables are plan params (rule 7, as `mw.nav.cell`), `route_input` reads nothing. `mw_ai::nav` holds
    `approach_point` + `DOOR_APPROACH_M` (+ `sf_tunables` dep on mw_ai and mw_bake); door ends dedup by (`LinkId`, side).
- **Reuse check**: Bevy has no navigation (nav roadmap decision 1); no crate builds road graphs from waypoint grids;
  `mw_ai::pathgrid` (PGRD → `petgraph`, exists) reads, `sf_nav::route_graph` (NV12a) merges, folds and simplifies.
  OpenMW is the behaviour reference above (per-cell graphs, pathgrid off-mesh links), not a design to copy.

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

AI_E/AI_F framework side (slots, catch-up, commands, Classic `FollowRules` with `commandable: false`): [followers.md](../../../superfargoth/docs/specs/followers.md).

As built (MA2a, 2026-11-26): `mw_ai::packages::directives(&[AiPackage], &PackageCtx) -> PackageDirectives`
(`RecordDirective { index, spec, extra }` in record order + `skipped` with reasons). `PackageCtx` = origin record id,
spawn `home`, a `PackageResolver` (actor, placement). AI_W -> `StayNear` (idle table in `PackageExtra`, from
`Wander::from_package`), AI_T -> `GoTo`, AI_E -> `Escort`, AI_F -> `Follow` (+ `FollowUntil` end point), AI_A -> `Use`;
coordinates via `mw_world::convert::world_pos`, a CNDT cell -> `mw_world::ids::interior`. OpenMW @ 5f3aafb: record
packages are priority 0 (`aipackage.hpp:39`, `typedaipackage.hpp:16-19`), repeat from the flag (`aisequence.cpp`
`fill` 492-534); a positive duration (game hours, int16) -> `Limit::For`, zero or negative = none (`aiwander.cpp` 160,
378; `aiescort.cpp` 82-90; `aifollow.cpp` 159-171). An escort with FLT_MAX coordinates is skipped (`NoDestination`;
OpenMW ends it once in range); a no-destination follow runs until its duration ends.

**Stack policy** (`mw_ai::policy`): `AiSequence::stack` (`aisequence.cpp` 386) inserts by `getPriority()`, honours
`shouldCancelPreviousAi`/`canCancel`, sends repeating packages back to the end, keeps the combat/pursue counters;
`execute` (249) and the `Options` in `aipackage.hpp` (priority, repeat, canCancel, sideWithTarget,
followTargetThroughDoors, alwaysActive) map to `Directive` fields and `ParamBag`. `mDone` → `Directive::done`.

As built (MA2b, 2026-11-26): `mw_ai::MorrowindPolicy` (`policy.rs`), `options(&DirectiveKind) -> Options` (OpenMW
`aipackage.hpp` 37-46 + each package's `makeDefaultOptions` @ 5f3aafb). Priority comes from the kind as in OpenMW
(combat 1, packages 0), so `DirectiveSpec::priority` is ignored. `stack` (386-477): an actual package (wander, travel,
escort, follow, activate) first cancels every `Fight` (`stopCombat`), then `shouldCancelPreviousAi` cancels what
`canCancel`; insert before the first entry of lower or equal priority (newest on top among equals, no source rank).
Record pushes (`fill` 492-534) append in order and cancel nothing. `execute` 346-366: a finished repeating actual
package goes to the bottom (a `Limit::For` running out counts as finished; cancelled never returns). Not expressible
yet (no kinds or one actor's stack only): Pursue/Cast/Face/AvoidDoor/Breathe/InternalTravel, the return-travel push
under combat (408-430: our wander resumes below the fight instead), Cast replacing Cast, follow/escort ending allies'
fights, follow resetting packages below (MA2d executor), combat target rotation (Combat drive), combat counters.

As built (MA2d1, 2026-11-26): `mw_walkers_bevy/src/directives.rs` `seed_directives` (Update, after `attach`, on
`Added<AiActor>`): base record (same levelled pick as `setup_of`), home = `ActorSpace` + spawn feet, packages pushed in
record order through `MorrowindPolicy`; a stack already in `AiDirectives` (save, earlier spawn) is never replaced;
every actor gets one (empty = seeded, no packages). Resolver: `actor(id)` = lowest spawned placement of that base;
the player and AI_A objects resolve to `None` (skipped, `info!` count per batch; no retry) -> MA2d3. Extras
(`AiPackageExtras`, wander idles, follow end) re-mapped by kind order for kept stacks. Clock: `sf_sky_bevy::GameClock`.

**MWScript** (`mw_script/src/functions.txt`: "AI, combat" block, `Get/Set/Mod{$AISTATS}` 188-190, `GetLineOfSight`
90; "later" or "stub" today): `AIWander`, `AITravel`, `AIEscort(Cell)`, `AIFollow(Cell)`, `AIActivate` push
directives; `GetAIPackageDone`, `GetCurrentAIPackage` read the stack (MA2); `StartCombat`/`StopCombat`, `GetTarget`,
`GetAttacked`, `Face` (MA2/MA10); `GetDetected` (awareness) and `GetLineOfSight` (perception via `portal_query`)
(MA5); `Get/Set/ModHello|Fight|Flee|Alarm` on the sheet (MA4; as built MA4b 2026-11-26: Get = modified, Set/Mod write
this actor's base only, not the record as OpenMW's `setBaseAISetting` does; a negative base reads 0); crime's `PayFine`, `PayFineThief`, `GoToJail` (crime-adapter A3; MA6 only routes guards).

As built (MA2d2, 2026-11-26): `mw_walkers_bevy/src/directive_drive.rs` `drive_directives` (FixedUpdate, after
`ReactionSystems::React`, before `walkers::drive` and `NavSystems::Corridor`): `advance(now, &MorrowindPolicy)` on
`GameClock`, then the serving directive. `walkers::drive` skips while `reaction.owns() || DirectiveDrive::owns()`;
a reaction keeps priority (the stack still advances; a GoTo restarts from where the reaction left the actor).
StayNear copies home/radius/idles into the walker planner only when they differ (record-seeded wander untouched).
GoTo walks an `OpRunner` `MoveTo` over the corridor and finishes on arrival or on OpenMW's end rule (planar distance
minus half height within max(64 units, speed x dt) for 2 s; `aitravel.cpp` 101-123 @ 5f3aafb); beyond 7168 units it
stands (`isWithinMaxRange` 23-29). Tunables `mw.ai.travel.{end_tolerance_m, finish_s, max_range_m}` (OpenMW) +
`retry_s` (Guess 2 s). Follow/Escort/Use/Fight/Custom, GoTo into another worldspace or off the navmesh: routine as
today (debug log). Gaps -> MA2d4.

As built (MA2c1, 2026-11-26): `mw_script::aifn` (`AiFn`, `AiCall` Wander/Travel/Escort/Follow/PackageDone/
CurrentPackage, OpenMW type ids), `Host::ai(target, &AiCall) -> Option<f64>` (`None` = unhandled), rows live.
`mw_rpg::script_ai`: `GameOp::Ai { target, push: AiPush }`, `AiPush::package()` rebuilds the exact `tes3` record
struct so the host maps it with `mw_ai::packages::directive` (+ `DirectiveSource::Script`): mw_rpg doesn't depend on
mw_ai (the spec needs world data: escorted key, worldspace, home). Reads through `MwAi` (`NoAi`: done 0, current -1);
reads ignore the run's own pushes (OpenMW `getLastRunTypeId`). OpenMW `aiextensions.cpp` @ 5f3aafb: any optional arg
makes travel/escort/follow repeat; escort hours truncated, follow hours float (kept in `AiPush::Follow.hours`);
AIWander drops the first optional (Idle chance), Idle2-9 clamped 0-255, next = reset; pushes on the player or an
item do nothing; AIEscortCell with an empty cell does nothing. The game host applies pushes in MA2c2.

As built (MA2c2, 2026-11-26): every script kind goes through `Rpg::apply` (`mw_rpg_bevy/src/dialogue.rs`): target ->
actor key, queued `ScriptAiPush { actor, package }`, sent as a message by `send_ai_pushes` (Update). The walker side
(`mw_walkers_bevy/src/script_ai.rs`) `apply_script_pushes` (FixedUpdate, before `drive_directives`) maps with
`mw_ai::packages::directive` (home = current position/worldspace, spawn resolver `SpawnedActors::of`), Script source,
pushes through `MorrowindPolicy`. Reads: `drive_directives` records each stack's served package before advancing;
`record_runs` keeps `AiRuns` (OpenMW `aisequence.cpp` 258-372: empty stack -> last none, done kept; a served actual
package becomes last; done true when it finishes/expires that tick; cancel never sets done; Fight/Custom leave last
alone); `sync_ai_runs` copies it into the RPG host; `impl MwAi for View`. Fixed: `mw_script` `Scoped` host
(`instances.rs`) didn't forward `ai()` (global/local scripts pushed nothing). `mw_rpg_bevy` now depends on
`mw_walkers_bevy` directly (already reachable through `mw_scripts_bevy`). Gaps -> MA2c3, MA2d4.

## AI settings (stats + considerations)

Hello, Fight, Flee, Alarm (`aisetting.hpp`) are magnitude stats on the sheet (`sf_stats`, stats-adapter naming),
initialised from the NPC/creature AIDT. As built (MA4a, 2026-11-26): `mw_stats` defs `hello`/`fight`/`flee`/`alarm`
(indices 140-143, `MwIdx::ai_settings`, appended after the magnitudes), AIDT read unsigned, floor 0 and no cap (OpenMW
`stat.cpp` 22-28: Hello is a distance and may pass 100). As built (MA4e, 2026-11-26): persuasion reads the speaker's
*base* Flee/Fight from its sheet (`mw_rpg::persuasion_input`; OpenMW @ 5f3aafb `mechanicsmanagerimp.cpp` 664-669/705-710
`getBase`), so script `Set/ModFight` reach it, and writes the clamped 0..=100 nudge back as `SetBase` (`persuasion_ai_ops`,
a `StatCommand` on the speaker); the unsaved `ai_settings` overlay and `Actor.ai` are gone. Considerations in `mw_ai::settings`:
- **Greeting** (React drive): every 0.25 s, target within `Hello × iGreetDistanceMultiplier`, LOS and awareness
  (`actors.cpp` `updateGreetingState` 481-560); turn, look-at, Greeting bark from dialogue.
  As built (MA4c1, 2026-11-26): `mw_ai::settings::greeting_step(&mut GreetState, &GreetInput, &GreetGmst) ->
  GreetAction { say_hello, play_idle2, hold_still, turn_to }` (OpenMW @ 5f3aafb `actors.cpp` `updateGreetingState`
  500-596, `turnActorToFacePlayer` 598-619, the shared 0.25 s `HelloTimer` 241/1556/1674/1706; `steering.cpp` 16-43
  5-degree check only). Combat, swimming or a package other than Wander/Travel/none clears the state every frame; the
  count rises only on full updates (LOS, awareness (caller rolls), live player, not hidden, not paralysed); at 2 it says
  hello (or Done with no line), faces the player `iGreetDuration / 0.25` updates, then Done; re-arms at
  `fGreetDistanceReset`. Kept quirk: the count never decays out of range. GMSTs `iGreetDistanceMultiplier` 6,
  `iGreetDuration` 4, `fGreetDistanceReset` 512 (confirmed). `greet_geometry` converts to game units + heading.
- **Aggression** (Combat drive): `engageCombat` and `isAggressive` (Fight + disposition + `getFightDistanceBias`;
  `actors.cpp` 618-760, `mechanicsmanagerimp.cpp` 1429); allies side with allies.
  As built (MA4d, 2026-11-26): `mw_ai::settings` `fight_rating`/`is_aggressive` (OpenMW @ 5f3aafb `combat.cpp`:
  `getFightTerm` 556-565 = Fight + trunc(distance bias + disposition bias); disposition `(50 - disp) x fFightDispMult`
  (creature observers 50) 567-575; distance `iFightDistanceBase - fFightDistanceMultiplier x d`, `getAggroDistance`
  ignores height for swimmers/flyers 577-603; `isAggressive` 623-647: in combat = true, immobile/calmed = false,
  `iWerewolfFightMod` vs a known werewolf, aggressive at >= 100). GMSTs via `FightGmst::from_db` (vanilla fallbacks 20,
  0.005, 0.2; `iWerewolfFightMod` is a Bloodmoon GMST, fallback 100 unverified). `engages_on_sight` + `EngageGates` =
  the aggression branch of `actors.cpp` `engageCombat` (dead, hidden/unreachable, player followers, LOS + awareness);
  ally-siding, guards and processing range are the caller's.
- **Flee** (Flee drive): `vanillaRateFlee` (`aicombataction.cpp` 457-488 @ 5f3aafb), `updateFleeing` (`aicombat.cpp` 378-482),
  intimidation raising Flee (`mechanicsmanagerimp.cpp` 664-708). The rating is ported; flee movement is modern (nav
  `Goal::Away`, toward allies or a door).
  As built (MA4f, 2026-11-26): `mw_ai::settings` `flee_rating` (`vanillaRateFlee`: Flee >= 100 returns Flee; else
  `(1 - health ratio) x fAIFleeHealthMult + Flee x fAIFleeFleeMult`, werewolf `iWereWolfFleeMod` below
  `iWereWolfLevelToAttack`, + the Fight distance bias when non-zero), `wants_to_flee` (`makeFleeDecision` 490-503:
  under 100 counts 0), `flee_step` (flee half of `AiCombat::execute`/`attack`, 139-353: 3 s `ActionFlee` cooldown,
  start/keep/stop/end combat, immobile/paralysed gates), `flee_run_step` (`updateFleeing` modes, 378-482). GMSTs
  confirmed in `defaultgmsts.cpp`: 7.0, 0.3, 20, 100, 3000. Vanilla never flees below Flee 100 (best case 56.7): this
  is the Classic rule; the modern response (flee by temperament and danger) is AI49. Intimidation already lives in
  `mw_rpg::persuasion`.
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

## Crime, witnesses, guards (`mw_crime`; this section is the AI side only, the law is crime-adapter's)

- `commitCrime` → `reportCrime` (`mechanicsmanagerimp.cpp` 1137-1450): offence kinds (trespass, pickpocket, theft,
  assault, murder, werewolf); witnesses = the framework's event-time witness query over every simulated actor within
  `fAlarmRadius` with LOS + awareness, through open seamless doors via `portal_query` (no per-package hacks).
- Bounty is added when the witness's report is delivered to a guard (crime-adapter `reportCrime` "At delivery"), never
  at sight; GMSTs in stats-adapter ("Factions, reputation, bounty"); Fight raised by
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

**As built (MA21, 2026-11-26):** `mw_exterior_bevy/src/nav_links.rs` adds `NavWorldGraphPlugin` over `DoorLinks`.
Approach: the PGRD node of the end's cell (exterior: arrival square + 8 neighbours) nearest the arrival, not behind the
door plane along the arrival yaw, within `mw.nav.door_approach_m` (4 m, Guess); none: the arrival. Flags stay
`classic_flags` until `world.doors` exists. `NavDoorStates`: `Locked` for a link with a door reference whose lock level
is > 0 at load, nothing else (static; runtime locks come with the interaction lane). Vanilla: 1828 links, 0 skipped,
2803/3656 ends on PGRD, 121 locked; pathgrids + graph 18.8 ms, locks 4.8 ms at load; log `nav: door approach ...`.

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
| Long routes | per-cell pathgrids, navmesh across cells; off-range actors freeze | Travel packages advance unloaded along the path (Skyrim, Starfield; CK "Persistence") | PGRD chains → route graph, tier 0 moves along it (**match** RDR2 road network, UE ZoneGraph lanes; nav roadmap decisions 3-4) |
| Fliers, swimmers | navmesh per agent bounds, swim flag | navmesh + water flag | clearance field + water-body volume (**match** Horizon Zero Dawn, decision 2) |

## Build plan (adapter slices)

| Slice | Deliverable | Test | After |
|---|---|---|---|
| **MA1** nav data | two agent classes, area table, `NavGeometrySource` (door leaves out, doorway areas), pathgrid links, door approach points | real install: tile and link counts, bake time per class, creature-radius histogram printed | NV4 |
| **MA2** packages | AI_* records and MWScript functions → directives, OpenMW `StackPolicy`, `GetAIPackageDone` | exact stack orders vs OpenMW cases; script tests per function | AI4 |
| **MA3** wander methods (**built 2026-11-18**) | Morrowind wander/idle method set on `sf_ai`; `PathWalkScaffold` removed | headless: wanderers stay within radius; idle chances exact (`roll_idle_with` vs OpenMW `getRandomIdle` at b8285cc; `drive_tests.rs` runs the real Bevy `drive`) | AI6 |
| **MA4** settings | Hello/Fight/Flee stats + considerations | exact ratings vs OpenMW formulas | AI7 |
| **MA5** awareness | `awarenessCheck` port as `DetectionModel` Classic; Meter profile split (AI0g) | exact vs formula cases; Classic ignores profile sight fields | AI8 |
| **MA6** crime | glue only: a guard's crime `Stance` (crime.md ladder) -> directives (`Warn` approach, `Arrest` talk, `Pursue`, `Attack`); witnesses, reports and bounty are `mw_crime`'s (crime lane decision 14) | scripted stance changes give the expected directive | MA5 |
| **MA7** fast-forward | `fastForward` rules | exact placement for wanderers and travellers | AI10 |
| **MA8** schedules | data pack (three patterns, guards unscheduled), anchors, validator, `ai.schedules` setting | validator report per pattern on a real install; classic = vanilla stacks; unresolved anchors keep vanilla | AI10, AI11, AI12 |
| **MA9** doors | escort/follow/pursue through doors, follower fallback | headless: follower through the Census door, guard pursues out | AI12 |
| **MA10** combat sheets | NPC and creature combat through C6 | scripted fight, no NaN | AI15 |
| **Nav adapter** (nav lane IDs) | NV12c route source, NV13c water bodies, NV9d pathgrid links + area-cost profiles, MA12 `PgrdFallback` removal ("Navigation") | rows in `docs/roadmaps/nav.md`, `docs/roadmaps/ai.md`; NV12c list in "Navigation" | there |

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

As built (MA18, 2026-11-25): `mw_walkers_bevy/src/visibility.rs` writes the player body's `AiVisibilityFactors` each
fixed tick (after `ActionSystems::Move`, before `AiPerceptionSystems::Sight`) from the action state's mode: `Mode::Sneak`
= `Stance::Crouched` (+30 concealment under `StandardDetection`), `Normal`/`FreeRun` = `Upright`; written only on
change. Modifiers from the host sheet (`StatIdx` as `StatId`): `sneak` always, `chameleon` when the table has it (not
yet: MA19); `StandardDetection` ignores both, MA5 reads them. Measured (light 0.40, guard 10 m facing): awareness at
90 ticks 1.00 standing vs 0.24 sneaking; sneaking reaches 0.77 at 240.

As built (AI8i, 2026-11-25): `mw_walkers_bevy/src/reaction.rs`: perception reactions are a layer over the routine.
`react` (before `NavSystems::Corridor`) runs for every ground actor with a `NavCorridor` + `Reaction` (wanderers, AI_W
distance-0 NPCs, scripted level actors): Searching pre-empts the routine and records an anchor (pose + yaw at
pre-emption), runs the search, then walks back to the anchor and turns to its yaw, then hands back; a new Searching
episode can pre-empt the return. Routines read `Reaction::owns` (wander and level scripts pause);
`apply_reaction_intent` keeps a standing reaction's facing; level path drives run after the corridor. Off the navmesh:
no reaction. `ReactionStats` preempted / returns / returned / return_failed. Not yet: avoidance for non-wanderers (AI8j).

As built (AI8h2, 2026-11-25): `Phase::React` in the same layer. `Reaction` keeps a `sf_ai::ReactState` (+ the revision its
`OpRunner` started with); `ReactState::update` runs every tick for every actor with awareness and a perception setup, even
during a search or off the navmesh (double take and no-re-react need every band change). Order per tick: Search (`Run`),
a search that ended starts the Return, React (any phase but Search; pre-empts Routine or Return, records the anchor,
restarts the operators on a new revision, Walk gait), then the Return steps. An idle goal (band dropped, hold over)
returns to the anchor. Curious = face the stimulus; Suspicious walks one `react_step_m` toward the LKP and stands facing
it (no creep). Tunables `ai.perception.react_{step_m,retarget_m,hold_s,face_delay_s}` bound into
`AiPerceptionSettings.react` (`sf_ai_bevy/src/perception.rs`). A Curious wanderer now stops to look.

## Open questions

None open. Decided 2026-11-08 (owner-delegated; `superfargoth/docs/research/ai-nav-decisions-2026-11-08.md` (e), (f),
(h)): three first patterns with guards unscheduled; merchants never refuse service; crime in `mw_rpg::crime`. Shop
hours and pattern choice are taste calls the owner may overturn cheaply (data and one setting).
