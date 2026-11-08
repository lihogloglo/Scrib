# AI and navigation: decisions on the open questions (research, 2026-11-08)

The owner delegated the open technical questions of [nav.md](../specs/nav.md), [ai.md](../specs/ai.md),
[ai-adapter](../../../bevyrrowind/docs/specs/ai-adapter.md) and the seamless research
([seamless-interiors-2026-11-08.md](seamless-interiors-2026-11-08.md) §7): "decide based on the industry-standard way,
and really think this through." Each section below gives the question, the options, what shipped engines do, how the
options fit our hard constraints, the decision with a confidence level, and what would make us revisit it. The specs
carry only the outcome; this file keeps the reasoning. Docs only, written on a machine that cannot compile.

Hard constraints weighed throughout: `CONTRIBUTING.md` (reuse order, industry standard, fixed tick and net-ready,
nothing bakes during play except accepted runtime nav patches, framework/game split, Bevy-free cores); GPL-3.0;
multiplayer with no player ceiling (server authority, dedicated builds, interest management); Steam Deck floor;
seamless optional per door / per pack; vanilla NPCs get schedules; Nemesis-style memory; combat AI modern first; an
LLM plugin later through a seam.

Code facts checked for this note (2026-11-08): `sf_world::DoorLink { a, b, two_way }` has no stable id (no `LinkId`
anywhere in `superfargoth/crates`); `mw_walkers_bevy::walkers` runs `drive` (path follower, ORCA, mover) in `Update`
while the player's tick view is in `FixedUpdate`; `sf_world::RingConfig::scaled` scales R0 together with R1-R3 but has
no caller yet; `mw_rpg` (Bevy-free, deps `sf_rpg`, `sf_stats`, `mw_stats`, `mw_script`) already holds the bounty
result-script functions and, on disk, `disposition.rs`/`persuasion.rs` (the OpenMW mechanics-manager family);
`mw_ai` does not depend on `mw_rpg`; `sf_ids::PlayerId` is a `u8`.

"(verify)" marks claims from memory or community sources not re-read today.

## Summary of decisions

| # | Question | Decision | Confidence |
|---|---|---|---|
| a | 2 or 3 agent classes | **2** (`humanoid`, `large`); small creatures use `humanoid`; class tiles resident only where that class is simulated | high |
| b | Big props carve from day one? | **No.** Doors use baked door polygons + runtime area/flag changes (no rebuild); prop carving is NV10, opt-in per record, at rest only | medium-high |
| c | Client-side compass routes? | **Yes**, as presentation: `WorldGraph` + coarse graph from the identical bake, door state as replicated; never feeds gameplay | high |
| d | High tier-2 cap | **128, provisional** (a fixed count, not blank, not adaptive); AIM sweep decides, mover and animation included | medium |
| e | Schedules pack timing and patterns | **After AI10 + AI12** (tier 0 and doors), never before; first patterns: shopkeepers/service, publicans, commoners with a resolvable home; **guards stay on post** | medium-high |
| f | Merchants refuse service after hours? | **No.** Availability follows presence; no refusal rule, no schedule door locks in the first pack; travel-service NPCs never scheduled | medium-high |
| g | Vendettas across worldspaces? | **Yes**, through tier-0 travel to an encounter point near the target player, bounded (hops, TTL), quest-safe actors only | medium |
| h | `mw_rpg::crime` or `mw_crime` | **`mw_rpg::crime`**, no `sf_ai` dependency (witnesses in, effects out) | high |
| i | Seamless default for portal-ready doors | A **world rule** (host, saved), not a graphics option; default **off until the M9a gate passes, then on for new worlds**; clients degrade only the portal view | medium |
| j | Door swing with auto-close | **Yes** for load (portal) doors: ~1 s swing, auto-close when the doorway is clear and no player is near; NPCs close doors behind them; in-cell doors keep vanilla | high |
| k | Full AI and physics behind an open door | **Yes**: the far R0 joins the simulated set; actors take tiers from the one global allocator; no stand-in mode | high |
| l | Split M9 | **Yes**: M9a player seamless, M9b world seamless; SI3 becomes a shared prerequisite of `sf_nav`/`sf_ai`; NV11, AI12/MA9, AI10 replace SI13-SI15; SI12 splits | high |
| s5 | Portal views per tier, chained spaces | Floor 1 reduced view, Medium 1, High 2; recursion depth 1; a deeper open doorway draws as a fog card, not a shut door | medium |
| s6 | Fade-only links | Keep the fade; a trapdoor drop animation is later polish | high |
| s7 | Followers | Walk through; vanilla teleport only when blocked N s and unseen | high |

Overturned or narrowed leanings: (a) the spec kept `small` open "if bake time allows": the cell-size rule makes `small`
the most expensive class, so it is out, not deferred; (b) the spec made doors TileCache obstacles: Recast's own door
flags are cheaper and standard; (e) guard shifts dropped from the first pack; (d) "leave blank" rejected, but 128 is
provisional and the measurement includes mover and animation; (i) the seamless default could not be a client setting
at all once gameplay (LOS, witnesses, NPC crossings) depends on it; (l) SI3 is not an M9 slice but an M7 prerequisite.

Genuine owner calls left: none of the above is strategic, but three are taste calls the owner may overturn cheaply
because each is a data/setting default: (f) shop hours, (e) which patterns feel right in play, (i) flipping seamless on
by default at the gate (it needs the owner's visual sign-off anyway, CONTRIBUTING "No automated visual checks").

## (a) Navmesh agent classes: 2 or 3

**Question.** How many agent classes (one navmesh per worldspace and class) does the Morrowind pack bake: humanoid +
large, or also small?

**Options.** 1 class for everyone (Bethesda style); 2 (humanoid, large); 3 (+ small); one mesh per distinct collision
size (OpenMW); one mesh for the smallest agent with per-polygon clearance.

**Industry.**
- Recast bakes the agent radius into the mesh (it erodes walkable area by the radius), so one mesh per size class is
  the standard; its docs say different agent types often need several navmeshes for one environment
  ([Recast intro](https://recastnav.com/md_Docs__1_Introducation.html)). Epic's guidance for UE is one navmesh per
  supported agent, because size is pre-baked ([UE forum, Epic answer](https://forums.unrealengine.com/t/how-do-i-set-up-navmeshes-for-characters-of-different-sizes/280867));
  a [GameDev.net thread](https://www.gamedev.net/forums/topic/623727-how-do-navmesh-work-with-agents-in-different-sizes/)
  reports a few classed meshes cheaper in memory and CPU than one mesh supporting several radii.
- Unity AI Navigation: one `NavMeshSurface` per agent type; few agent types in practice (verify the guidance wording).
- Bethesda: one hand-authored navmesh per cell for every actor; giants and mammoths path on it and clip (verify).
- OpenMW: one mesh per distinct agent bounds (`components/detournavigator/navigator.hpp`), with a 256 MiB cache: a
  memory ceiling we explicitly do not want on the Deck.

**Constraints.** Bake time ("few minutes", all classes) and Deck tile memory (≤ 64 MiB) multiply by class. The key
fact the spec missed: Recast's rule `cs ≈ radius / 2` means voxel count per area scales with `1 / cs²`. With the
humanoid at r ≈ 0.3 m (cs 0.15), a large class at r ≈ 1 m (cs ≈ 0.5) costs roughly (0.15/0.5)² ≈ 9 % of the humanoid
bake, while a small class at r ≈ 0.15 m (cs 0.075) costs about 4×. Three classes ≈ 5× the humanoid bake; two ≈ 1.1×.
Gameplay cost of no `small` class: rats, scribs and mudcrabs can't enter gaps narrower than a human's: invisible in
Morrowind content (no rat holes, no crawlspaces). Gameplay cost of no `large` class: guars, kagouti, atronachs and
Daedroth path into doorways and alleys they can't fit and stick; very visible.

**Decision.** Two classes: `humanoid` (also small creatures) and `large`. Classes stay data (`sf.nav.agent`), so a
pack or mod can add one. **Interaction fix:** tiles of a class load only around anchors whose simulated set contains
an actor of that class (humanoid always), so `large` costs memory only near large creatures. **Confidence: high.**

**Revisit if** content needs small-only passages; MA1's class histogram shows many creatures between the two radii
stuck on `humanoid` (they'd need a middle class); NV4 shows the humanoid bake alone near the time gate (then coarsen
exterior humanoid tiles, not drop `large`). Measurement: NV4 prints cold bake time and MiB per class.

## (b) Do big props carve the navmesh from day one?

**Question.** Are large movable props (crates, carts, big corpses) TileCache obstacles from the first nav build, or
only doors until a need shows?

**Options.** Carve all large props; carve only at rest; never carve (avoidance only); doors only.

**Industry.**
- Unity: carving is optional per obstacle; the default "Carve Only Stationary" removes the hole while the obstacle
  moves and is "generally the best choice in terms of performance", while moving obstacles are avoided locally, not
  planned around ([Unity AI Navigation 2.0, About NavMesh Obstacles](https://docs.unity3d.com/Packages/com.unity.ai.navigation@2.0/manual/AboutObstacles.html)).
- UE: runtime generation "Static", "Dynamic Modifiers Only" or "Dynamic"; movable actors affect the mesh only when
  flagged, each change dirties tiles (verify the setting names).
- Bethesda: static navmesh; clutter and Havok objects don't touch it, actors bump or push; door triangles are marked
  at Finalize ([CK World Hookup](https://ck.uesp.net/wiki/Bethesda_Tutorial_World_Hookup)). Fallout 4 workshop objects
  cut the navmesh at runtime (verify).
- Recast/Detour: the demo marks door polygons with an area and a `DOOR` poly flag and toggles flags at runtime with
  `dtNavMesh::setPolyFlags` / `setPolyArea`, no rebuild (`RecastDemo` `SAMPLE_POLYFLAGS_DOOR`, verify the identifier);
  `dtTileCache` obstacles carve and need compressed layers per tile kept in memory.
- OpenMW: doors are excluded from the geometry and their footprint marked as door area (cost 2); objects that move
  dirty tiles regenerated asynchronously, ≥ 250 ms apart.

**Constraints.** Morrowind content has no large movable props (crates and carts are statics); our physics props are
small, corpses lie flat and are stepped over. TileCache needs extra per-tile layer memory on the Deck and a rebuild
budget. Doors are the one runtime state that matters, and the spec's plan (doors as TileCache obstacles) pays a tile
rebuild per door open/close: in a city with schedules and NPCs crossing doors that is a steady rebuild stream.

**Decision.** Day one: **doors only, and not by carving**: the bake excludes door leaves from the geometry and marks
the doorway footprint with a convex area volume, giving each doorway its own polygons; at runtime the host changes
their area (`door_closed` cost) and flags (`LOCKED`) with Detour's poly calls, no tile rebuild. Locked doors are
excluded by a filter flag except for queries whose per-agent predicate admits them (keys, scripted rights). Prop
carving stays NV10 but **opt-in** per record (`carves_nav`, adapter, default off in vanilla), only for a body at rest
above a size threshold ("carve only stationary"), with TileCache layers baked only for tiles that can host one.
**Confidence: medium-high** (navmesh-rs must expose `set_poly_flags`/`set_poly_area` or equivalents: NV0 checks).

**Revisit if** content with large movable objects arrives (building, carts, big physics props in mods), or `ai.stats`
stuck counts near props exceed the background rate in NVM playtests.

## (c) May clients compute compass and quest-marker routes locally?

**Question.** Is a client allowed to run route queries (identical bake, replicated door state) for UI, or must the
host compute and send them?

**Options.** Host computes and replicates routes; clients compute; hybrid (host sends waypoints, client smooths).

**Industry.** Authority covers gameplay state; UI derived from replicated state is computed on the client in every
server-authoritative engine (UE's cosmetic and UI code runs client-side; GPS-style minimap routes in open-world online
games are client-side, verify). Bethesda's compass points at the load door that leads toward the target: a door-graph
route, not a full path. Valve's rule "never trust the client" concerns inputs and results that change the world, not
presentation (Source networking docs, verify).

**Constraints.** No player ceiling: host-side UI routes would scale server CPU with player count for no gameplay
value. Anti-cheat: the route reveals nothing beyond the marker target the host already sends and the navmesh the
client already ships. A client's view of distant door state is stale; acceptable for UI. Deck: a non-hosting client
should not need nav tiles at all.

**Decision.** Yes, as presentation. Clients compute compass and marker routes from the `WorldGraph` and coarse graphs
(always resident, KB-MB) using door state replicated in their interest set and the baked default elsewhere; an
optional near-field breadcrumb uses Detour tiles inside the client's own ring. The host sends marker targets
(`Location`), never routes; a quest without a marker sends nothing. A dedicated server computes no UI routes. Results
never feed gameplay (net rule 3). **Confidence: high.**

**Revisit if** hidden-target content needs the route itself kept secret, or a client/host fingerprint mismatch path
ever lets clients join with a different bake (then routes come from the host).

## (d) High-tier tier-2 NPC cap: 128, or leave until measured?

**Question.** Should ai.md publish 128 full-tier actors as the High default or leave the cell blank until AIM?

**Options.** 128 provisional; blank; derive from the Floor ratio; an adaptive time-budget governor without a count.

**Industry.** UE MassLOD uses per-level distances plus a **max count** per LOD (verify), deterministic. UE's
Animation Budget Allocator adapts counts to a ms budget, but animation is presentation (verify). Bethesda's process
levels are by loaded grid, not count; Skyrim's loaded grid is famously not something to change mid-save
(see the conflicts section). KCD II runs thousands of NPCs with AI LOD tuned per scene (Warhorse GDC 2026).

**Constraints.** Code needs a value; "blank" means the allocator can't run and the owner can't test. An adaptive
governor reading wall-clock time would make tier assignment depend on timing, which breaks reproducible traces and the
tier-invariance test even though outcomes stay tier-invariant. Tier 2 is not only AI: it includes corridor + ORCA +
mover foot probes, and its animation (A0 ≈ 25 µs, so 128 × A0 ≈ 3.2 ms) is usually the binding cost, under
`AnimBudget`. In single-player Morrowind the cap rarely binds (Balmora's exterior holds a few dozen NPCs within R0); it
binds in multiplayer, where one global cap is shared across players.

**Decision.** Keep **128 / 400 on High as provisional fixed counts**, and every non-Floor row marked provisional. The
allocator uses fixed counts per `sim.cpu_tier`; `ai.stats` logs when the per-tick AI budget is exceeded (warning, not
adaptation). Measurement belongs in **AIM**: sweep tier-2 counts 32-256 on the High reference in Balmora and a Vivec
canton with 1 and 4 players, recording AI main-thread ms, mover/probe ms and animation ms together; the chosen cap is
the largest count where the sum stays inside the CPU tier's character budget. AI9 prints per-actor tier-2 cost so the
sweep is a log, not a profile. **Confidence: medium.**

**Revisit when** AIM lands, or if animation or probes dominate (then the cap follows `AnimBudget`, and tier 2 may run
mover probes at half rate).

## (e) When does the schedules pack ship, and with which starting patterns?

**Question.** Ship schedules with the first Morrowind playable, or only after AI10 proves tier 0? Four patterns
(merchants/service, guards with shifts, publicans, commoners) or fewer?

**Options.** Early (on tier 1-2 only); after tier 0; after tier 0 and smart objects. Patterns: all four; merchants and
guards; a resolvable subset.

**Industry.** Every routine system pairs schedules with off-screen simulation: Bethesda conditioned packages move
low-process actors between cells (Oblivion's schedules shipped with process levels); KCD routines run off-screen with
AI LOD; Gothic places NPCs by routine on arrival. Radiant's lesson: quest-relevant actors must stay where quests expect
them. A routine needs a valid place: Gothic waypoints/freepoints, Skyrim furniture and markers.

**Constraints.** A schedule that sends a shopkeeper home moves them into another worldspace; without tier 0 and
`WorldGraph` travel the actor goes dormant mid-route (walkers2 processing range) and vanishes or freezes: broken, not
just plain. So schedules need AI10 (tier 0, promotion resolve) and AI12 (door traversal). Guards: vanilla places one
guard per post; day/night shifts with relief halve visible guards at any hour, change crime-witness coverage and
bounty balance (a content mechanic we port), and need relief pairs vanilla doesn't have. Skyrim-era guards are on duty
at all hours in practice (verify). Homes: many vanilla NPCs own no bed elsewhere; an anchor that doesn't resolve must
mean "keep vanilla".

**Decision.** `ScheduleRules::None` (vanilla) is the default until MA8, which ships after **AI10, AI11 and AI12**,
independent of "first playable". First patterns, each applied only where its anchors resolve (else vanilla packages):
1. shopkeepers and service NPCs (not travel services): at work 08-20, home at night if an owned bed in another place
   resolves, else stay in the shop cell;
2. publicans: in the tavern 10-02, home or a tavern bed after;
3. commoners: outdoors by day, indoors at night where a home resolves; homeless exterior commoners stay vanilla;
4. **guards: no shifts** in the first pack (on post 24 h, as vanilla).
Night "home" means `StayNear` indoors until AI14 brings beds (`UseSlot`); sleeping is a later pattern change, not a
new system. The MA8 validator report prints, per pattern, how many NPCs resolve, are excluded by quest safety, or stay
vanilla. **Confidence: medium-high.**

**Revisit if** MA8's report shows a pattern resolving for few NPCs (then streets look unchanged and the pattern isn't
worth its risk), if playtests show empty streets feel wrong, or when a pack authors relief guards (shifts then become a
data change).

## (f) Do merchants refuse service after hours?

**Question.** Should a merchant met outside work hours refuse to trade (modern) or trade when found (vanilla-friendly)?

**Options.** Refuse by rule; trade anywhere; trade with a penalty; lock shops by schedule.

**Industry.** Oblivion, Skyrim and RDR2 enforce hours mostly through **location and locked shop doors**, not a refusal
rule; a merchant you find can usually still trade (verify per game). KCD merchants sleep at night and can't be traded
with while asleep (verify). Vanilla Morrowind: every merchant available 24 h; quests and player habits depend on it.

**Constraints.** Classic/modern seam: presence is the modern feel, refusal is an extra rule. Locking doors on a
schedule changes world state that scripts read (`GetLocked`) and crime (lockpicking a closed shop becomes a crime); it
also interacts with quest-safety. Travel services (silt striders, guild guides) gate player movement.

**Decision.** No refusal rule. Availability follows presence: a merchant at work trades; one found elsewhere trades as
vanilla (an "after hours" greeting bark is optional content). No schedule-driven door locks in the first pack; a later
opt-in `ai.shop_hours = off | lock_doors` setting can add them. Travel-service NPCs and guild guides are excluded from
schedules. Sleeping NPCs (after AI14) wake on activation, as in Skyrim. **Confidence: medium-high** (taste call; one
setting to change).

**Revisit if** the owner wants opening hours to matter (then `lock_doors`, with crime rules for break-ins), or players
report merchants are hard to find at night (then the shop pattern's night anchor stays in the shop).

## (g) May vendetta directives cross worldspaces?

**Question.** May a memory-driven `Memory` directive move a named NPC across worldspaces to find a player (KCD-like),
or stay local?

**Options.** Local only; cross via real tracking; cross via encounter placement; cross with bounds.

**Industry.** Shadow of Mordor's Nemesis captains return through **ambush encounters** near the player rather than by
tracking them across the map (Monolith's design; patent text, verify). Skyrim's hired thugs and Dark Brotherhood
assassins are world-encounter spawns triggered on location change (UESP, verify). KCD NPCs travel off-screen as part
of routines. All shared rule: off-screen, logical, and resolved near the player out of view.

**Constraints.** Tier 0 travel on `WorldGraph` plus promotion resolve already places an actor anywhere out of view
(ai.md "Tiers"); quest safety must still hold (named quest NPCs must not leave); multiplayer: the target is a
`PlayerKey`, possibly offline or in another town; host CPU must not grow with long pursuits.

**Decision.** Yes, bounded. A vendetta pushes a `Memory` directive of kind encounter: the host picks an approach point
near the target player (a link end or a route node out of every player's view) within `max_hops` and `ttl`, the actor
travels there on tier 0 and is promoted out of view (the standard promotion resolve), then seeks/ambushes. Only actors
the quest-safety validator allows; a directive whose target player is offline pauses (never retargets another player);
on expiry the actor returns by its directive stack. Ordinary (non-vendetta) memory goals stay local. **Confidence:
medium.**

**Revisit if** playtests show ambushes feel unfair or repetitive (tune the TTL/cooldown data), or promotion pops in
view (then encounters only enter through doors).

## (h) Crime in `mw_rpg::crime` or a new `mw_crime` crate?

**Question.** Where does the OpenMW crime port (`commitCrime`, `reportCrime`, witnesses, bounty, guards) live?

**Options.** `mw_rpg::crime`; `mw_crime`; `mw_ai::crime`.

**Industry/OpenMW.** OpenMW keeps crime in `mwmechanics/mechanicsmanagerimp.cpp` next to disposition, persuasion
and barter: one "mechanics manager" family. Bethesda tracks crime per faction as world data that NPCs query.

**Constraints.** Build-perf rule: a new crate pays off when its dependency set or its consumers differ. Crime needs
stats (bounty, Fight/Alarm), factions, GMSTs and dialogue functions (`PayFine`, `GoToJail`, arrest topics), all in or
under `mw_rpg`; `mw_rpg` already holds the bounty result-script functions and the disposition/persuasion family. The
one risk is a dependency on `sf_ai` (witness and fact types) that would couple dialogue builds to AI builds.

**Decision.** `mw_rpg::crime`, Bevy-free, with **no `sf_ai` dependency**: it takes the witness list (from the
framework's event-time witness query) as plain input and returns effects (bounty delta, Fight/disposition changes,
who reports, which guards pursue, a crime fact to record); the Bevy glue pushes the *Pursue* directive and appends the
fact. **Confidence: high.**

**Revisit if** crime needs `sf_ai`/`sf_nav` types directly, or `mw_rpg`'s cold build passes the build-perf budget
(then split `mw_crime` out with the same API).

## (i) Default on or off for portal-ready doors

**Question.** Once SI6-SI8 pass, is seamless the default for portal-ready links, given seamless must stay optional?

**Options.** Always off by default; on by default; per graphics preset; a world setting with a pack default.

**Industry.** Engines whose interiors fit the shell are seamless by design (RAGE, Cry, RED, UE); Bethesda keeps load
doors. Optional seamless exists only in the OpenMW fork (all settings default off). No shipped game makes door
semantics depend on a graphics preset.

**Constraints, and the interaction that decides it.** Seamless changes gameplay, not just pixels: open seamless doors
pass sight (`portal_query::line_of_sight`), so witnesses, alarms and combat see through them; NPCs walk through instead
of crossing unseen; the far side joins the simulated set. If seamless were a client graphics option, a Deck client
and a High client would disagree about who saw a theft, violating "gameplay never depends on graphics settings" and
host authority. Cost on the Deck: the portal view is a second frame (+1.5 ms looking in at 1440p High; looking out
unmeasured).

**Decision.** Seamless is a **world rule** `world.doors = seamless | classic` (host, saved with the world), with a pack
default and per-door data overrides (fade-only links stay fade). Clients cannot switch semantics; they may only
degrade the portal **view** (`portal.view = full | reduced`, reduced = lower resolution, no shadows, no cloud
raymarch, the OpenMW fork's look), and a client may fade its own screen during a crossing as presentation. Default:
**classic until the M9a gate** (SI6-SI8 built, SIM within the Floor budget with the reduced view, owner visual
sign-off), then **seamless for new worlds**; existing worlds keep their setting. **Confidence: medium.**

**Revisit if** SIM shows even the reduced view breaks the Floor gate (then the default stays classic on the Deck's
own hosted worlds, still a world setting), or the owner prefers classic as the shipped look.

## (j) Visible door swing with auto-close

**Question.** Visible swing with auto-close (OpenMW fork), or doorways that stay open once used? Swing time?

**Options.** Swing + auto-close; swing, stays open; instant open; stays open + NPCs close.

**Industry.** Swinging doors are universal (Bethesda, KCD, RDR2); NPCs in Skyrim and KCD close doors behind them
(verify); the OpenMW fork swings in 1.1 s and closes when the player is 1000 units away unless pursuit continues.
Vanilla Morrowind in-cell doors swing and stay as left.

**Constraints.** The swing is the loading mask (passable at > 50 % and `arrival_ready`). An open load door costs a
portal view and a live far side (k); with no player ceiling and schedules moving NPCs through doors, doors left open
would make every interior of a town simulated: auto-close is a scale control, not cosmetics.

**Decision.** Load (portal) doors swing visibly, ~1 s (adapter data), and auto-close after `door_close_delay` (~5 s)
once the passage box is empty and no player is within `door_close_radius` (~14 m), unless a pursuit or follow routes
through. The `Traverse` operator closes a door behind an NPC that found it shut. A body in the doorway stops and
reopens the leaf (kinematic, never crushes). In-cell doors keep vanilla behaviour. Host-authoritative state.
**Confidence: high.**

**Revisit if** the owner finds the auto-close fussy in play (tune delay/radius), or doors flap under NPC traffic.

## (k) Full AI and physics behind an open door

**Question.** Is the far side live (full AI + physics within R0 of the far door) or only visible stand-ins?

**Options.** Full simulation; stand-ins idling until the crossing; render only.

**Industry.** The OpenMW fork loads secondary cells with physics, actors and scripts; Bethesda simulates loaded cells
at high process; UE World Partition simulates everything streamed in, scaled by significance (MassLOD).

**Constraints.** Seeing a theft through a door, an NPC hearing combat, a guard pursuing out: all need the far side
simulated, and stand-ins would be a third mechanism beside the tiers. Tier invariance already makes cost a tier
matter, not a rules matter.

**Decision.** Full: the far R0 around the mapped anchor joins the simulated set; its physics world steps, scripts run,
actors take tiers from the **one global allocator**, with visibility through the opening counting as visibility. No
stand-in mode. **Confidence: high.**

**Revisit if** SIM/AIM on the Deck shows a canton behind an open door over budget: then shrink the portal anchor's
radius (a fixed per-game constant, never a preset).

## (l) Split M9; reconcile slices with `sf_ai`

**Question.** Split M9 into player/rendering and NPC/AI parts, and reconcile NV11/AI10/AI12 replacing SI13-SI15.

**Decision.** Yes.
- **M9a, seamless for the player:** SI0-SI2, SI4-SI11, SIM.
- **M9b, seamless for the world:** SI12b (portal driver anchor at the mapped pose, body and projectile crossing),
  NV11 (`LinkId`, `WorldGraph`, `portal_query`, replaces SI13), AI12 + MA9 (door traversal, replaces SI14), AI10 (tier
  0, replaces SI15), SI16 (co-op through portals, after net N5).
- **Pulled out as shared prerequisites** (they are M7 and net work, not M9): **SI3** (`InWorldspace`, per-viewer
  worldspace, derived `ActiveWorldspace`), needed before NV5 (nav ring per anchor's worldspace) and AI6; and **SI12a**
  (the simulated set as the union of anchors' worldspaces, physics stepping each), needed by AI9 and net N5; until it
  lands the set is the active worldspace.
- AI12 needs only SI2 (the Bevy-free `sf_portal` core) for its seamless branch; its fade-only branch works with
  seamless off, so AI doesn't wait for portal rendering.
**Confidence: high.**

**Revisit if** SI3 proves too large for one compiling slice (split by consumer, keeping the derived resource).

## Remaining seamless questions (technical)

- **(s5) Portal views and chained spaces.** Floor: 1 view, reduced; Medium: 1; High: 2. Recursion depth 1 (SPUD and
  most portal engines cap recursion; Valve draws the last level flat, verify). A second open doorway seen through the
  first draws as a card in the far-far space's fog/ambient colour, not as a shut leaf (a shut leaf would misstate door
  state). Confidence medium; revisit on owner look.
- **(s6) Fade-only links (216).** Keep the fade (Bethesda's model); animated trapdoor drops are later content polish.
  High.
- **(s7) Followers.** Walk through like anyone; vanilla teleport only when blocked N s and unseen by every player
  (Skyrim followers catch up out of view, verify). High.

## Conflicts the spec author found: resolutions

1. **net.md non-goals vs the owner** (no ceiling, dedicated servers): the owner's 2026-11-08 answer wins. Dedicated
   headless servers and > 4 players leave the non-goals; co-op 2-4 on a listen server is the first milestone. Anti-cheat
   stays a non-goal only as *client-side detection*: server authority and command validation are in scope (they are
   the baseline anti-cheat of every authoritative engine). `PlayerId` loses "0-3"; the wire index is ≥ `u16` when
   `sf_net` lands (`sf_ids::PlayerId(u8)` widens then; logged as a code backlog item, not done here).
2. **CONTRIBUTING "small co-op 2-4"**: replaced by "no ceiling, co-op first" plus the two rules the AI work exposed
   (no sim radius or cap tied to graphics; nothing sized to a player count).
3. **Streaming rings scale with the preset**: only R1-R3 (visual) scale; R0 (simulation, collision, processing range)
   and the nav ring are fixed per game, overridable per worldspace as data. Evidence: Skyrim players are warned not to
   change `uGridsToLoad` mid-game because the loaded-grid radius is stored in saves and destabilises them (community
   reports, [Nexus](https://forums.nexusmods.com/topic/657668-changing-ugridstoload-broke-the-game),
   [STEP](https://stepmodifications.org/forum/topic/1665-a-quick-question-about-ugridstoload/)); UE servers stream by
   player sources independent of client settings (verify). Code: `RingConfig::scaled` scales `active` too but has no
   caller; when the preset hook lands it must scale only R1-R3 (code backlog item).
4. **Walkers step ORCA per frame**: NPC drive (path follower, ORCA, mover) moves to `FixedUpdate` in the action
   layer's move set, with render interpolation like the player's tick view: slice **NV7a**, before AI6 (ORCA walls
   become NV7b). Cost: two ticks per 30 fps frame on the Deck; tier 1 avoidance runs every other tick (staggered by
   `ActorKey`) if the budget needs it. DECISIONS walkers is superseded by a new bullet.
5. **M7 vs M10**: AI and navigation are M7 (RPG core, renumbering 2026-10-13: old M10+ = M7); M10 is combat. AI15
   (combat AI operators) is labelled M7 and depends on combat C6 (M10). Stale "M10" in locomotion.md and in code
   docs (`mw_ai/src/lib.rs`, `mw_ai/src/scaffold.rs`, `sf_controller/src/path.rs`) → M7 (code edits are a later slice).
6. **action.md "NPCs path on the navmesh"**: no navmesh exists; pathgrid routes until nav.md NV8.
7. **ARCHITECTURE `bevy_landmass` row**: rejected, with the current version; add `navmesh-rs` and recastnavigation rows;
   §5 gains `sf_nav` and loses "navmesh generation per streamed tile" in `sf_ai`.

## Sources

- Recast Navigation docs, introduction: https://recastnav.com/md_Docs__1_Introducation.html
- Epic answer on navmeshes per agent size: https://forums.unrealengine.com/t/how-do-i-set-up-navmeshes-for-characters-of-different-sizes/280867
- GameDev.net, navmesh and agents of different sizes: https://www.gamedev.net/forums/topic/623727-how-do-navmesh-work-with-agents-in-different-sizes/
- Unity AI Navigation 2.0, About NavMesh Obstacles: https://docs.unity3d.com/Packages/com.unity.ai.navigation@2.0/manual/AboutObstacles.html
- Creation Kit, World Hookup (door navmesh): https://ck.uesp.net/wiki/Bethesda_Tutorial_World_Hookup
- `uGridsToLoad` and saves (community): https://forums.nexusmods.com/topic/657668-changing-ugridstoload-broke-the-game ,
  https://stepmodifications.org/forum/topic/1665-a-quick-question-about-ugridstoload/
- OpenMW `b8285cc9`: `components/detournavigator/` (navigator, area types, off-mesh connections, async updater),
  `apps/openmw/mwmechanics/mechanicsmanagerimp.cpp` (crime family); OpenMW seamless fork `48392fca` (door swing,
  auto-close, secondary cells).
- Warhorse GDC 2025/2026, Humphreys (Game AI Pro ch. 12), UE Mass/Smart Objects docs, Nemesis patent reports: as cited
  in [npc-ai-2026-11-08.md](npc-ai-2026-11-08.md) "Sources".

## Ready-to-paste edits for shared docs

Not applied here (other sessions may be editing these files). Each block is exact old → new text.

### DECISIONS.md (append)

- 2026-11-08 (ai-layers; ai.md, nav.md): NPC AI = directives → utility drives → one shared data-driven HTN method
  library → executors writing `Command`/`LocomotionIntent`; crates `sf_nav` + `sf_ai` (+ planned `sf_interaction`).
- 2026-11-08 (nav-bake; nav.md): navmesh baked per tile by `sf_bake`; runtime TileCache patches for doors and props are
  not "baking during play". Backend: spike `navmesh-rs` first, recastnavigation FFI is the accepted fallback.
- 2026-11-08 (ai-schedules; ai.md, ai-adapter.md): vanilla Morrowind NPCs get modern schedules as moddable data in
  `mw_*`, behind `ScheduleRules` so a classic "no schedules" rule set stays possible.
- 2026-11-08 (ai-tiers; ai.md "Tiers and budget"): AI caps are a host CPU-tier simulation setting, never the graphics
  preset; tier 0 simulates logically so outcomes are tier-invariant; Floor defaults 48 full / 160 reduced.
- 2026-11-08 (ai-llm; ai.md): LLM dialogue/goal choice later as an optional out-of-process plugin choosing among
  validated options; seam reserved, never required.
- 2026-11-08 (ai-memory; ai.md): Nemesis-style persistent NPC memory and relationships are in scope; the patent is not
  a concern (owner decision).
- 2026-11-08 (ai-combat): NPC combat AI modern first (C6); classic OpenMW combat is a later `CombatRules` seam.
- 2026-11-08 (ai-label): NPC AI and navigation are scope label M7 everywhere; "M10" for AI/nav in older docs is stale;
  M10 is combat.
- 2026-11-08 (nav-classes; nav.md, research ai-nav-decisions (a)): two agent classes, `humanoid` (small creatures
  too) and `large`; `small` rejected (cs ≈ r/2 makes it ~4× the humanoid bake); class tiles resident only where needed.
- 2026-11-08 (nav-doors; nav.md (b)): doorways are baked door polygons switched at runtime by area/flags, no tile
  rebuild; prop carving is opt-in per record, at rest only, in NV10; vanilla carves nothing.
- 2026-11-08 (nav-ui-routes; nav.md (c)): clients compute compass/marker routes locally from `WorldGraph` + coarse
  graphs as presentation; the host sends marker targets only.
- 2026-11-08 (ai-caps; ai.md (d)): tier caps are fixed counts per `sim.cpu_tier`, never adaptive; High 128/400 is
  provisional until the AIM sweep (AI, mover and animation measured together).
- 2026-11-08 (ai-schedules2; ai-adapter (e, f)): schedules ship after AI10-AI12; first patterns shopkeepers, publicans,
  commoners where anchors resolve; guards stay on post; merchants never refuse service; travel services unscheduled.
- 2026-11-08 (ai-vendetta; ai.md (g)): vendettas may cross worldspaces as bounded tier-0 travel to an out-of-view
  encounter point near the target player; quest-safe actors only; pause while the target is offline.
- 2026-11-08 (crime-crate; ai-adapter (h)): crime is `mw_rpg::crime`, Bevy-free, without an `sf_ai` dependency
  (witnesses in, effects out).
- 2026-11-08 (seamless-rule; seamless research (i)): seamless vs classic doors is a host world rule saved with the
  world, never a graphics option; clients may only degrade the portal view; seamless default for new worlds after M9a.
- 2026-11-08 (doors-swing; (j, k)): load doors swing ~1 s and auto-close when clear; NPCs close doors behind them; an
  open door's far R0 joins the simulated set with full AI and physics under the global tier allocator.
- 2026-11-08 (m9-split; (l)): M9a = seamless for the player, M9b = seamless for the world; SI3 and SI12a are shared
  M7/net prerequisites; NV11, AI12/MA9 and AI10 replace SI13-SI15.
- 2026-11-08 (net-ceiling; net.md, CONTRIBUTING): no player ceiling: co-op 2-4 on a listen server first, dedicated
  headless servers in scope; client-side anti-cheat stays a non-goal, server authority does not.
- 2026-11-08 (r0-fixed; streaming.md): only visual rings R1-R3 scale with the quality preset; R0 and the nav ring are
  fixed per game (per-worldspace data overrides allowed). Supersedes the 2026-10-11 "ring radii scale" for R0.
- 2026-11-08 (walkers-tick; nav.md NV7a): NPC drive (path follower, ORCA, mover) moves to the fixed tick with render
  interpolation; supersedes the per-frame stepping of DECISIONS 2026-10-12 walkers.

### net.md

- Agent summary (line 13): `Decisions: 2-4 players, host-authoritative with client prediction, no lockstep;` →
  `Decisions: co-op 2-4 first, no player ceiling (headless dedicated server, interest management; owner 2026-11-08), host-authoritative with client prediction, no lockstep;`
- "Goal" (lines 24-25): `Non-goals: competitive PvP, anti-cheat, dedicated servers, more than 4 players, lockstep or`
  `bit-exact determinism (owner decision; GGRS rejected in R12 §1), cross-version play.` →
  `Non-goals: competitive PvP, client-side anti-cheat (server authority and command validation are in scope), lockstep or`
  `bit-exact determinism (owner decision; GGRS rejected in R12 §1), cross-version play. Dedicated servers and more than`
  `4 players are no longer non-goals (owner 2026-11-08): co-op 2-4 is the first milestone, not the limit.`
- "Owner decisions" row (line 31): `| Small co-op, 2-4 players; one player hosts; host-authoritative with client prediction; no lockstep | "Topology", "Prediction" |` →
  `| Co-op first (2-4 players, one player hosts), no player ceiling: the same binary runs as a headless dedicated server (owner 2026-11-08); host-authoritative with client prediction; no lockstep | "Topology", "Prediction" |`
- "Topology" (line 86): `is never a separate game mode. Up to 4 players (setting, default 4).` →
  `is never a separate game mode. Player count is a host setting (default 4, no compiled limit); a dedicated server is a host with no local player.`
- "Players" (line 106): `- **`PlayerId`**: a small index (0-3) assigned by the host at join, used on the wire and in events,` →
  `- **`PlayerId`**: a compact index (at least `u16` on the wire; `sf_ids::PlayerId(u8)` widens in N2) assigned by the host at join, used on the wire and in events,`
- Lines 176, 228 and 320 (4-player bandwidth and budget targets): prefix each target with `First milestone:`.

### CONTRIBUTING.md

- "Design constraints" (lines 68-69): `- **Net-ready:** small co-op (2-4 players, host-authoritative) comes later, so new specs use a fixed tick,`
  `  input as commands, presentation that never feeds gameplay, and a remote player is just another character.` →
  `- **Net-ready:** multiplayer comes later with no player ceiling (owner 2026-11-08): co-op 2-4 on a listen server first,`
  `  then headless dedicated servers with interest management, always server/host-authoritative. So new specs use a fixed`
  `  tick, input as commands, presentation that never feeds gameplay, no simulation radius or cap tied to graphics`
  `  settings, nothing sized to a player count, and a remote player is just another character.`

### streaming.md

- Agent summary (line 11): `ring radii scale with quality preset; no colliders outside R0; actors simulate only where cells are Active ("Decided").` →
  `visual ring radii (R1-R3) scale with the quality preset, R0 is fixed per game; no colliders outside R0; actors simulate only in the simulated set, where cells are Active ("Decided").`
- "Decided" (lines 682-685): `- **Ring radii scale with the quality preset.** The radii above are the`
  `  "high" defaults. The fly-through calibrates them to the largest radii`
  `  that still meet the §0 gates, so view distance is bought with measured`
  `  frame time, never guessed.` →
  `- **Visual ring radii (R1-R3) scale with the quality preset; R0 does not (2026-11-08).** The radii above are the`
  `  "high" defaults. The fly-through calibrates R1-R3 to the largest radii that still meet the §0 gates, so view`
  `  distance is bought with measured frame time, never guessed. R0 (simulation, collision, processing range) and the`
  `  nav ring (nav.md "Streaming") are fixed per game, overridable per worldspace as data, identical on a headless`
  `  server: what simulates must not depend on a client's settings. `RingConfig::scaled` must leave `active` alone.`
- "Decided", processing range (lines 688-689): `movement, foot probes) only in the active worldspace and only while every cell within a small` →
  `movement, foot probes) only in a worldspace of the simulated set (every gameplay anchor's worldspace plus the far side of open portals, seamless research §4.2) and only while every cell within a small`

### action.md

- "NPCs" (lines 295-296): `Traversal AI (choosing to mantle or jump in a chase) waits for `sf_ai`; NPCs path on the navmesh`
  `until then. NPC probes stay gated (A0, blocked or asked).` →
  `Traversal AI (choosing to mantle or jump in a chase) waits for `sf_ai` (ai.md AI6, nav.md NV9 jump/drop/climb links);`
  `until then NPCs walk `mw_ai` pathgrid routes (no navmesh exists before nav.md NV8). NPC probes stay gated (A0, blocked or asked).`

### ARCHITECTURE.md

- §4 (line 164): `| `bevy_landmass` | navmesh candidate (M10) | no | 0.12.1 / 0.19 |` →
  `| `bevy_landmass` | rejected (glam 0.30 + `dodgy_2d` duplicates, no dynamic obstacles; nav.md "Reuse"); its islands idea is copied as nav regions | yes | 0.13.0 / 0.20 |`
  and add after it:
  `| `navmesh-rs` | navigation backend (M7), adopt after the NV0 spike (nav.md) | n/a (Bevy-free) | 0.2.2 (2026-11-06) |`
  `| recastnavigation (C++) | navigation fallback through FFI (`sf_nav_recast_sys`, nav.md NV0b) | n/a | upstream 2026-02; `-sys` 1.0.3 stale |`
- §5 (line 224): `| `sf_interaction` | Activation, grab/carry/throw (Oblivion-style), doors, containers. |` →
  `| `sf_interaction` | Activation, grab/carry/throw (Oblivion-style), doors, containers; smart-object slots and claims for AI activities (planned spec; ai.md "Smart objects"). |`
- §5 (line 226): `| `sf_ai` | Navmesh generation per streamed tile, behavior layer (schedules, goals, perception). Takes content goals (escort, follow, travel, wander, fight) through a seam so a classic Morrowind package executor can replace the modern behaviour (§1b). |` →
  `| `sf_nav` | Navigation (spec `nav.md`): Recast/Detour tiles baked by `sf_bake` per worldspace and agent class, runtime door/obstacle patches, worldspace graph across door links, `portal_query`; backend `sf_nav_recast`, thin `sf_nav_bevy`. |`
  `| `sf_ai` | NPC behaviour (spec `ai.md`): directives (packages, schedules, quest orders) → utility drives → HTN method library → executors; perception, memory facts, tiers 2/1/0. Takes content goals through a seam so a classic Morrowind package executor can replace the modern behaviour (§1b). |`
- §9 (line 364): `| M9 | Seamless interiors: portal rendering, crossing between worldspaces, environment-volume blending | Walk into the Census Office without a load or fade |` →
  `| M9a | Seamless interiors for the player: door swing, portal rendering, crossing, environment, weather and audio blending | Walk into the Census Office without a load or fade |`
  `| M9b | Seamless interiors for the world (after M7's AI slices): simulated far sides, bodies and NPCs through doors, worldspace graph | A guard pursues you out of the Census Office through the open door |`

### ROADMAP.md

- Phase 3, new lines after 3.8:
  `- [ ] 3.9 NPC navigation (M7; superfargoth/docs/specs/nav.md): NV0 navmesh-rs spike first (FFI fallback), then NV1-NV12; SI3 (InWorldspace) before NV5; checks NV-.`
  `- [ ] 3.10 NPC AI (M7; specs/ai.md + bevyrrowind specs/ai-adapter.md): AI1 trace/debugger first, then AI2-AI18, MA1-MA10; schedules (MA8) after AI10-AI12; checks AI-.`
- Phase 4 (line 145): `- [p] Seamless portals (M9), far-field performance (M8: HLOD, texture streaming, PBR generation).` →
  `- [p] Seamless portals: M9a player (SI0-SI11), M9b world (SI12b, NV11, AI12/MA9, AI10, SI16; after 3.10); far-field performance (M8: HLOD, texture streaming, PBR generation).`
- Phase 4 (line 151): `- [p] Living world AI (owner 2026-10-28, not started, no design yet): characters going about their lives.` →
  `- [p] Living world AI (owner 2026-10-28; design: ai.md, memory: ai.md "Memory", decisions: research/ai-nav-decisions-2026-11-08.md): characters going about their lives.`
  (rest of the line unchanged, except drop "Research before any spec (rule 8)." which is done).

### SYSTEMS.md

- AI row: `| AI | - no spec yet (see action: NPCs) | - | mw_ai | 3.5 | - | - | - |` →
  `| AI | [ai](../superfargoth/docs/specs/ai.md) Architecture, Tiers and budget, Build plan | [ai-adapter](../bevyrrowind/docs/specs/ai-adapter.md) | sf_ai(_bevy, planned) mw_ai | 3.10 | AI- | [npc-ai-2026-11-08](../superfargoth/docs/research/npc-ai-2026-11-08.md); [ai-nav-decisions-2026-11-08](../superfargoth/docs/research/ai-nav-decisions-2026-11-08.md) | - |`
- New rows after it:
  `| Navigation | [nav](../superfargoth/docs/specs/nav.md) Data model, Routing, Streaming, Build plan | [ai-adapter](../bevyrrowind/docs/specs/ai-adapter.md) Navigation | sf_nav(_bevy) sf_nav_recast (planned) mw_ai mw_bake | 3.9 | NV- | [npc-ai-2026-11-08](../superfargoth/docs/research/npc-ai-2026-11-08.md) | - |`
  `| Interaction, smart objects | - no spec yet (ai.md "Smart objects") | - | sf_interaction (planned) | - | - | - | - |`
- Portals row: `| Portals, interiors | [portals](../superfargoth/docs/specs/portals.md) Goal; [streaming](../superfargoth/docs/specs/streaming.md) Portal crossing | [world-adapter](../bevyrrowind/docs/specs/world-adapter.md) | sf_world_bevy | - | DR- | - | - |` →
  `| Portals, interiors | [portals](../superfargoth/docs/specs/portals.md) Goal; [streaming](../superfargoth/docs/specs/streaming.md) Portal crossing | [world-adapter](../bevyrrowind/docs/specs/world-adapter.md) | sf_world_bevy sf_portal(_bevy, planned) | Phase 4 seamless | DR- PT- | [seamless-interiors-2026-11-08](../superfargoth/docs/research/seamless-interiors-2026-11-08.md) | - |`

### Other drift (no exact text needed; one-word fixes)

- locomotion.md lines 26, 117 and the comparison row at 336: "M10" → "M7" for navigation/AI (keep M10 for combat).
- Code module docs (a later code slice): `mw_ai/src/lib.rs` line 1, `mw_ai/src/scaffold.rs` line 2,
  `sf_controller/src/path.rs` line 1: "(M10)" → "(M7)".
